use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::{adw, gtk, Component, ComponentParts, ComponentSender};

use crate::COMMANDS;

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct PackageManagerModel {
    install_flatpak: bool,
    install_flatpak_beta: bool,
    install_flatpak_flatseal: bool,
    install_nix: bool,
    install_snap: bool,
    install_appimage: bool,
    install_appimage_am: bool,
}

#[derive(Debug)]
pub(crate) enum PackageManagerInput {
    /// Represents the Flatpak switch states
    Flatpak(bool),
    FlatpakBeta(bool),
    FlatpakFlatSeal(bool),
    /// Represents the Nix switch state
    Nix(bool),
    /// Represents the Snap switch state
    Snap(bool),
    #[allow(clippy::doc_markdown)]
    /// Represents the AppImage switch states
    AppImage(bool),
    AppImageAM(bool),
}

#[derive(Debug)]
pub(crate) enum PackageManagerOutput {
    /// Move to the next page
    NextPage,
}

#[relm4::component(pub)]
impl Component for PackageManagerModel {
    type CommandOutput = ();
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
                            #[name="flatpak"]
                            adw::ExpanderRow {
                                set_title: "Flatpak",
                                set_subtitle: &gettext("Will also configure the Flathub repository."),
                                set_tooltip_text: Some(&gettext("System for application virtualization.")),
                                add_action = &gtk::Switch {
                                    set_valign: gtk::Align::Center,
                                    set_active: false,
                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Flatpak(switch.is_active()));
                                    }
                                },
                                #[name="flatpak_beta"]
                                add_row = &adw::ActionRow {
                                    set_title: "Flatpak Beta Channel",
                                    set_subtitle: &gettext("Allows software to be installed from the Flatpak Beta Channel"),
                                    set_tooltip_text: Some(&gettext("Enable Flatpak Beta Channel.")),
                                    set_sensitive: false,
                                    #[name="flatpak_beta_switch"]
                                    add_suffix = &gtk::Switch {
                                        set_valign: gtk::Align::Center,
                                        set_active: false,
                                        connect_active_notify[sender] => move |switch| {
                                            sender.input(PackageManagerInput::FlatpakBeta(switch.is_active()));
                                        }
                                    }
                                },
                                #[name="flatpak_flatseal"]
                                add_row = &adw::ActionRow {
                                    set_title: "Flatseal",
                                    set_subtitle: &gettext("Manage Flatpak permissions"),
                                    set_tooltip_text: Some(&gettext("Enable Flatseal permission manager.")),
                                    set_sensitive: false,
                                    #[name="flatpak_flatseal_switch"]
                                    add_suffix = &gtk::Switch {
                                        set_valign: gtk::Align::Center,
                                        set_active: false,
                                        connect_active_notify[sender] => move |switch| {
                                            sender.input(PackageManagerInput::FlatpakFlatSeal(switch.is_active()));
                                        }
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "Nix",
                                set_subtitle: &gettext("Will also configure the nixpkgs channel."),
                                set_tooltip_text: Some(&gettext("Purely functional package manager.")),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,
                                    set_active: false,
                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Nix(switch.is_active()));
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "Snap",
                                set_subtitle: &gettext("Uses the Snapcraft repository. Default in Ubuntu."),
                                set_tooltip_text: Some(&gettext("Software deployment and package management system developed by Canonical.")),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,
                                    set_active: false,
                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Snap(switch.is_active()));
                                    }
                                }
                            },
                            #[name="appimage"]
                            adw::ExpanderRow {
                                set_title: "AppImage",
                                set_subtitle: &gettext("Will install the necessary dependencies to run AppImages."),
                                set_tooltip_text: Some(&gettext("Self-contained and compressed executable format for the Linux platform.")),

                                add_action = &gtk::Switch {
                                    set_valign: gtk::Align::Center,
                                    set_active: false,
                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::AppImage(switch.is_active()));
                                    }
                                },
                                #[name="appimage_am"]
                                add_row = &adw::ActionRow {
                                    set_title: "AM",
                                    set_subtitle: &gettext("A command line interface to install AppImages."),
                                    set_tooltip_text: Some(&gettext("Enable the AM package manager.")),
                                    set_sensitive: false,
                                    #[name="appimage_am_switch"]
                                    add_suffix = &gtk::Switch {
                                        set_valign: gtk::Align::Center,
                                        set_active: false,
                                        connect_active_notify[sender] => move |switch| {
                                            sender.input(PackageManagerInput::AppImageAM(switch.is_active()));
                                        }
                                    }
                                }
                            }
                        }
                    },

                    gtk::Button::with_label(&gettext("Next")) {
                        set_halign: gtk::Align::Center,
                        set_css_classes: &["pill", "suggested-action"],

                        connect_clicked[sender] => move |_| {
                            sender.output(Self::Output::NextPage).unwrap();
                        }
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = PackageManagerModel {
            install_flatpak: false,
            install_flatpak_beta: false,
            install_flatpak_flatseal: false,
            install_nix: false,
            install_snap: false,
            install_appimage: false,
            install_appimage_am: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    #[allow(clippy::too_many_lines)]
    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
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

                widgets.flatpak.set_expanded(self.install_flatpak);
                widgets.flatpak_beta.set_sensitive(self.install_flatpak);
                widgets.flatpak_flatseal.set_sensitive(self.install_flatpak);
                if !self.install_flatpak {
                    widgets.flatpak_beta_switch.set_active(false);
                    widgets.flatpak_flatseal_switch.set_active(false);
                }
            },
            Self::Input::FlatpakBeta(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Flatpak Beta installation"
                    } else {
                        "Disabling Flatpak Beta installation"
                    }
                );

                self.install_flatpak_beta = switched_on;
            },
            Self::Input::FlatpakFlatSeal(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Flatseal installation"
                    } else {
                        "Disabling Flatseal installation"
                    }
                );

                self.install_flatpak_flatseal = switched_on;
            },
            Self::Input::Nix(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Nix installation"
                    } else {
                        "Disabling Nix installation"
                    }
                );

                self.install_nix = switched_on;
            },
            Self::Input::Snap(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Snap installation"
                    } else {
                        "Disabling Snap installation"
                    }
                );

                self.install_snap = switched_on;
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

                widgets.appimage.set_expanded(self.install_appimage);
                widgets.appimage_am.set_sensitive(self.install_appimage);
                if !self.install_appimage {
                    widgets.appimage_am_switch.set_active(false);
                }
            },
            Self::Input::AppImageAM(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling AM installation"
                    } else {
                        "Disabling AM installation"
                    }
                );
                self.install_appimage_am = switched_on;
            },
        }

        let mut commands: Vec<&str> = Vec::new();

        if self.install_flatpak {
            commands.push("sudo apt-get install -y flatpak");
            commands.push("flatpak remote-add --system --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo");
            commands.push(
                "flatpak install --system flathub org.gtk.Gtk3theme.adw-gtk3 \
                 org.gtk.Gtk3theme.adw-gtk3-dark -y",
            );
            commands.push(
                "sudo flatpak override --filesystem='xdg-config/gtk-3.0:ro' \
                 --filesystem='xdg-config/gtk-4.0:ro' --filesystem='xdg-data/icons:ro' \
                 --filesystem='xdg-data/themes:ro'",
            );
            if self.install_flatpak_beta {
                commands.push("flatpak remote-add --system --if-not-exists flathub-beta https://flathub.org/beta-repo/flathub-beta.flatpakrepo");
            }
            if self.install_flatpak_flatseal {
                commands.push("flatpak install --system flathub com.github.tchx84.Flatseal -y");
            }
        }

        if self.install_nix {
            commands.push("sudo apt-get install -y nix-bin nix-setup-systemd");
            commands.push("{ rhino-hotfix nix-bin || :; }");
            commands.push("sudo groupadd -f nix-users");
            commands.push("sudo usermod -a -G nix-users $USER");
            commands.push("{ sudo systemctl enable nix-daemon.service || :; }");
            commands.push("sudo su $USER -c 'nix-channel --add https://nixos.org/channels/nixpkgs-unstable nixpkgs'");
            commands.push("sudo su $USER -c 'nix-channel --update'");
        }

        if self.install_snap {
            commands.push("sudo apt-get install -y snapd");
        }

        if self.install_appimage {
            commands.push("sudo apt-get install -y libfuse2");
            if self.install_appimage_am {
                commands.push(
                    "{ wget https://github.com/ivan-hc/AM/raw/main/INSTALL && sudo sh ./INSTALL \
                     && rm ./INSTALL; }",
                );
            }
        }

        COMMANDS.write_inner().insert("package_manager", commands);
    }
}
