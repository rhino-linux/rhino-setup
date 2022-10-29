use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

use crate::COMMANDS;

#[derive(Debug)]
pub(crate) struct PackageManagerModel {
    install_flatpak: bool,
    remove_snap: bool,
    install_appimage: bool,
}

#[derive(Debug)]
pub(crate) enum PackageManagerInput {
    /// Represents the Flatpak switch state
    Flatpak(bool),
    /// Represents the Snap switch state
    Snap(bool),
    /// Represents the AppImage switch state
    AppImage(bool),
}

#[derive(Debug)]
pub(crate) enum PackageManagerOutput {
    /// Move to the next page
    NextPage,
}

#[relm4::component(pub)]
impl SimpleComponent for PackageManagerModel {
    type Init = ();
    type Input = PackageManagerInput;
    type Output = PackageManagerOutput;
    type Widgets = PackageManagerWidgets;

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

                set_icon_name: Some("rhinosetup-package-symbolic"),
                set_title: &gettext("Package Manager"),
                set_description: Some(&gettext("Choose one or more package managers to install")),

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,
                    set_hexpand: true,
                    set_valign: gtk::Align::Center,

                    adw::PreferencesPage {
                        add = &adw::PreferencesGroup {
                            adw::ActionRow {
                                set_title: "Flatpak",
                                set_subtitle: &gettext("Will also configure the Flathub repository."),
                                set_tooltip_text: Some(&gettext("System for application virtualization.")),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Flatpak(switch.is_active()));
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "Snap",
                                set_subtitle: &gettext("Uses the Snapcraft repository. Default in Ubuntu."),
                                set_tooltip_text: Some(&gettext("Software deployment and package management system developed by Canonical.")),

                                add_suffix = &gtk::Switch {
                                    set_active: true,
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Snap(switch.is_active()));
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "AppImage",
                                set_subtitle: &gettext("Will install the necessary dependencies to run AppImages."),
                                set_tooltip_text: Some(&gettext("Self-contained and compressed executable format for the Linux platform.")),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::AppImage(switch.is_active()));
                                    }
                                }
                            }
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
        let model = PackageManagerModel {
            install_flatpak: false,
            remove_snap: false,
            install_appimage: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            Self::Input::Flatpak(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Flatpak installation"
                    } else {
                        "Disabling Flatpak installation"
                    }
                );

                self.install_flatpak = switched_on;
            },
            Self::Input::Snap(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Disabling Snap removal"
                    } else {
                        "Enabling Snap removal"
                    }
                );

                self.remove_snap = !switched_on;
            },
            Self::Input::AppImage(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling AppImage installation"
                    } else {
                        "Disabling AppImage installation"
                    }
                );
                self.install_appimage = switched_on;
            },
        }

        let mut commands: Vec<&str> = Vec::new();

        if self.install_flatpak {
            commands.push("sudo apt-get install -y flatpak gnome-software-plugin-flatpak");
            commands.push("flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo");
        }

        if self.remove_snap {
            commands.push("sudo rm -rf /var/cache/snapd/");
            commands.push("sudo apt-get autopurge -y snapd gnome-software-plugin-snap");
        }

        if self.install_appimage {
            commands.push("sudo apt-get install -y libfuse2");
        }

        COMMANDS.write_inner().insert("package_manager", commands);
    }
}
