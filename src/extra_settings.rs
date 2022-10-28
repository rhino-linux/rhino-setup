use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::COMMANDS;

#[derive(Debug)]
pub(crate) struct ExtraSettingsModel {
    install_nala: bool,
    enable_apport: bool,
}

#[derive(Debug)]
pub(crate) enum ExtraSettingsInput {
    /// Represents the Nala switch state
    Nala(bool),
    /// Represents the Apport switch state
    Apport(bool),
}

#[derive(Debug)]
pub(crate) enum ExtraSettingsOutput {
    /// Move to the next page
    NextPage,
}

#[relm4::component(pub)]
impl SimpleComponent for ExtraSettingsModel {
    type Init = ();
    type Input = ExtraSettingsInput;
    type Output = ExtraSettingsOutput;
    type Widgets = ExtraSettingsWidgets;

    view! {
        #[root]
        adw::Bin {
            set_halign: gtk::Align::Fill,
            set_valign: gtk::Align::Fill,
            set_hexpand: true,

            adw::StatusPage {
                set_halign: gtk::Align::Fill,
                set_valign: gtk::Align::Fill,
                set_hexpand: true,

                set_icon_name: Some("rhinosetup-puzzle-piece-symbolic"),
                set_title: &gettext("Extra Settings"),
                set_description: Some(&gettext("The following are optional settings, leave them as they are if you don't know what they do.")),

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,
                    set_hexpand: true,
                    set_valign: gtk::Align::Center,

                    adw::PreferencesPage {
                        add = &adw::PreferencesGroup {
                            adw::ActionRow {
                                set_title: "Nala",
                                set_subtitle: &gettext("Nala is an alternative front-end to APT, featuring a beautiful UI/UX."),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Nala(switch.is_active()));
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "Apport",
                                set_subtitle: &gettext("Apport is a crash reporting system that helps us improve the stability of the system."),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Apport(switch.is_active()));
                                    }
                                }
                            },
                        }
                    },
                    gtk::Button::with_label(&gettext("Next")) {
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["pill", "suggested-action"],

                        connect_clicked[sender] => move |_| {
                            sender.output(Self::Output::NextPage);
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
        let model = ExtraSettingsModel {
            install_nala: false,
            enable_apport: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::Nala(switched_on) => {
                log::info!(
                    "{}",
                    if switched_on {
                        "Enabling Nala installation"
                    } else {
                        "Disabling Nala installation"
                    }
                );

                self.install_nala = switched_on;
            },
            Self::Input::Apport(switched_on) => {
                log::info!(
                    "{}",
                    if switched_on {
                        "Enabling Apport"
                    } else {
                        "Disabling Apport"
                    }
                );

                self.enable_apport = switched_on;
            },
        }

        let mut commands: Vec<&str> = Vec::new();

        if self.install_nala {
            // HACK: Required to run pacstall from a root context
            commands.push(
                "cd /home/$(logname) && HOME=/home/$(logname) runuser -m -u $(logname) -- sh -c \
                 'pacstall -PI nala-deb'",
            );
        }

        if self.enable_apport {
            commands.push("sudo apt-get install -y apport");
            commands.push("systemctl enable apport.service || true");
        }

        COMMANDS.write_inner().insert("extra_settings", commands);
    }
}
