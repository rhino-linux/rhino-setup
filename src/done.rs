use std::process::Command;

use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::{adw, gtk, main_application, ComponentParts, ComponentSender, SimpleComponent};

#[derive(Debug)]
pub(crate) struct DoneModel {
    icon: &'static str,
    title: String,
    description: String,
    error_state: bool,
}

#[derive(Debug)]
pub(crate) enum DoneInput {
    /// Restarts the OS.
    Reboot,
    /// Sent by the [crate::carousel] whenever an error occurs.
    /// This signals the done page to switch to a "error" page state.
    SwitchToErrorState,
    /// Sent when an error has occurred, and the user has clicked the "close"
    /// button.
    CloseApp,
}

#[relm4::component(pub)]
impl SimpleComponent for DoneModel {
    type Init = ();
    type Input = DoneInput;
    type Output = ();
    type Widgets = DoneWidgets;

    view! {
        #[root]
        adw::Bin {
            set_halign: gtk::Align::Fill,
            set_valign: gtk::Align::Fill,
            set_hexpand: true,

            adw::StatusPage {
                #[watch]
                set_icon_name: Some(model.icon),
                #[watch]
                set_title: &model.title,
                #[watch]
                set_description: Some(&model.description),

                set_halign: gtk::Align::Fill,
                set_valign: gtk::Align::Fill,
                set_hexpand: true,

                gtk::Box {
                    set_halign: gtk::Align::Center,

                    gtk::Button::with_label(&gettext("Reboot Now")) {
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["pill", "suggested-action"],

                        #[watch]
                        set_visible: !model.error_state,

                        connect_clicked[sender] => move |_| {
                            sender.input(Self::Input::Reboot);
                        }
                    },
                    gtk::Button::with_label(&gettext("Close")) {
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["pill", "destructive-action"],

                        #[watch]
                        set_visible: model.error_state,

                        connect_clicked[sender] => move |_| {
                            sender.input(Self::Input::CloseApp);
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = DoneModel {
            icon: "emblem-default-symbolic",
            title: gettext("All done!"),
            description: gettext("Restart your device to enjoy your Rhino Linux experience."),
            error_state: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::Reboot => {
                Command::new("sudo").arg("reboot").status().unwrap();
            },
            Self::Input::SwitchToErrorState => {
                self.error_state = true;
                self.icon = "dialog-error-symbolic";
                self.title = gettext("Something went wrong");
                self.description = gettext("Please contact the distribution developers.");
            },
            Self::Input::CloseApp => {
                log::info!("Closing the application");
                main_application().quit();
            },
        }
    }
}
