use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::{
    gtk, Component, ComponentController, ComponentParts, ComponentSender, Controller,
    SimpleComponent,
};

use crate::config::PROFILE;
use crate::COMMANDS;

#[derive(Debug)]
struct ProgressBarModel {
    /// The fraction of the progress bar completed.
    completed: f64,
    /// Total size of the progress bar.
    total: f64,
}

#[derive(Debug)]
enum ProgressBarInput {
    /// Sent when a command finishes, and it's time to update the progress bar.
    Progress,
}

#[relm4::component]
impl SimpleComponent for ProgressBarModel {
    /// Initialize with the total size of the progress bar.
    type Init = f64;
    type Input = ProgressBarInput;
    type Output = ();
    type Widgets = ProgressBarWidget;

    view! {
        gtk::ProgressBar {
            set_text: Some(&gettext("The changes are being applied. Please Wait...")),
            set_show_text: true,
            set_margin_top: 40,
            set_margin_start: 40,
            set_margin_bottom: 40,
            set_margin_end: 40,

            #[watch]
            set_fraction: model.completed
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ProgressBarModel {
            completed: 0.0,
            total: init,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::Progress => self.completed += 1.0 / self.total,
        }
    }
}

pub(crate) struct ProgressModel {
    /// This is lazily initialized when the [COMMANDS] are available, i.e, the
    /// user has progressed to this page.
    progress_bar: Option<Controller<ProgressBarModel>>,
}

#[derive(Debug)]
pub(crate) enum ProgressInput {
    /// Sent by [`crate::CarouselPagesModel`] when the user has finished
    /// browsing through all the other pages.
    StartInstallation,
}

#[derive(Debug)]
pub(crate) enum ProgressOutput {
    InstallationComplete,
    InstallationError,
}

#[relm4::component(pub)]
impl SimpleComponent for ProgressModel {
    type Init = ();
    type Input = ProgressInput;
    type Output = ProgressOutput;
    type Widgets = ProgressWidget;

    view! {
        #[name(root)]
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_valign: gtk::Align::Center,
        }
    }

    fn pre_view() {
        if let Some(progress_bar) = model.progress_bar.as_ref() {
            let widget = progress_bar.widget();
            if widget.parent().is_none() {
                root.append(widget);
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ProgressModel { progress_bar: None };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    #[allow(clippy::cast_precision_loss)]
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            Self::Input::StartInstallation => {
                tracing::info!("Starting installation");

                COMMANDS.write_inner().insert(
                    "pre_run",
                    vec![
                        "sudo apt-get update",
                        "sudo sed -i 's/ignore_stack=false/ignore_stack=true/g' /usr/bin/pacstall \
                        && TERM=linux pacstall -U pacstall master",
                    ],
                );

                let commands = COMMANDS.read_inner();
                let commands = commands.values().flatten();
                let mut commands_with_results = String::new();

                // Aggregate all the commands.
                for command in commands.clone() {
                    commands_with_results += &format!(
                        "{command} && {{ echo ---successful---; }} || {{ echo ---failed---; }}; "
                    );
                }

                tracing::debug!("{commands_with_results}");

                if PROFILE == "Devel" {
                    sender.output(Self::Output::InstallationComplete).unwrap();
                    tracing::info!("Installation skipped");
                    return;
                }

                // Spawn a process to execute the commands
                let mut processor = Command::new("sh")
                    .args([
                        "-c",
                        &format!(r#"pkexec sh -c "{commands_with_results}" || echo ---failed---"#,),
                    ])
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .unwrap();

                let stdout_reader = BufReader::new(processor.stdout.take().unwrap());

                // Initialize the progress_bar now, as the commands are available.
                self.progress_bar = Some(
                    ProgressBarModel::builder()
                        .launch(commands.count() as f64)
                        .detach(),
                );

                let progress_bar_sender = self.progress_bar.as_ref().unwrap().sender().clone();
                relm4::spawn_blocking(move || {
                    let mut error_occured = false;

                    for line in stdout_reader.lines().map(Result::unwrap) {
                        tracing::debug!("{line}");

                        if line.contains("---successful---") {
                            progress_bar_sender
                                .send(ProgressBarInput::Progress)
                                .unwrap();
                        } else if line.contains("---failed---") {
                            tracing::error!(
                                "Error executing commands: {}",
                                BufReader::new(processor.stderr.take().unwrap())
                                    .lines()
                                    .map(Result::unwrap)
                                    .collect::<String>()
                            );

                            error_occured = true;
                            sender.output(Self::Output::InstallationError).unwrap();

                            // Kill the processor to avoid any extra changes to the system.
                            processor.kill().unwrap();
                        }
                    }

                    if !error_occured {
                        sender.output(Self::Output::InstallationComplete).unwrap();
                        tracing::info!("Installation complete");
                        Command::new("pkexec")
                            .args(["sh", "-c", "sudo apt remove -yq rhino-setup"])
                            .status()
                            .unwrap();
                    }
                });
            },
        };
    }
}
