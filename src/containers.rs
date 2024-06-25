use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::{adw, gtk, Component, ComponentParts, ComponentSender};

use crate::COMMANDS;

#[derive(Debug)]
#[allow(clippy::struct_excessive_bools)]
pub(crate) struct ContainersModel {
    enable_docker: bool,
    enable_podman: bool,
    enable_distrobox: bool,
    enable_apptainer: bool,
    enable_qemu: bool,
    enable_virtualbox: bool,
}

#[derive(Debug)]
pub(crate) enum ContainersInput {
    /// Represents the container engines switch states
    Docker(bool),
    Podman(bool),
    Apptainer(bool),
    /// Represents the Distrobox switch state
    Distrobox(bool),
    #[allow(clippy::upper_case_acronyms)]
    /// Represents the virtual machine switch states
    QEMU(bool),
    Virtualbox(bool),
}

#[derive(Debug)]
pub(crate) enum ContainersOutput {
    /// Move to the next page
    NextPage,
}

#[relm4::component(pub)]
impl Component for ContainersModel {
    type CommandOutput = ();
    type Init = ();
    type Input = ContainersInput;
    type Output = ContainersOutput;
    type Widgets = ContainersWidgets;

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
                set_title: &gettext("Containerization"),
                set_description: Some(&gettext("Container Engines and Virtual Machines")),

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_vexpand: true,
                    set_hexpand: true,
                    set_valign: gtk::Align::Center,

                    adw::PreferencesPage {
                        add = &adw::PreferencesGroup {
                            adw::ActionRow {
                                set_title: "Docker",
                                set_subtitle: &gettext("The open-source application container engine."),
                                set_tooltip_text: Some(&gettext("Enable the Docker container engine.")),
                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,
                                    set_active: false,
                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Docker(switch.is_active()));
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "Podman",
                                set_subtitle: &gettext("Engine to run OCI-based containers in Pods."),
                                set_tooltip_text: Some(&gettext("Enable the Podman container engine.")),
                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,
                                    set_active: false,
                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Podman(switch.is_active()));
                                    }
                                }
                            },
                            #[name="distrobox"]
                            adw::ActionRow {
                                set_title: "Distrobox",
                                set_subtitle: &gettext("Use any linux distribution inside your terminal.\nRequires Docker or Podman. "),
                                set_sensitive: false,
                                set_visible: true,

                                #[name="distrobox_switch"]
                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Distrobox(switch.is_active()));
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "Apptainer",
                                set_subtitle: &gettext("Container platform focused on supporting \"Mobility of Compute\""),
                                set_tooltip_text: Some(&gettext("Enable the Apptainer container engine.")),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Apptainer(switch.is_active()));
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "QEMU",
                                set_subtitle: &gettext("QEMU full system emulation"),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::QEMU(switch.is_active()));
                                    }
                                }
                            },
                            adw::ActionRow {
                                set_title: "VirtualBox",
                                set_subtitle: &gettext("x86 virtualization solution"),

                                add_suffix = &gtk::Switch {
                                    set_valign: gtk::Align::Center,

                                    connect_active_notify[sender] => move |switch| {
                                        sender.input(Self::Input::Virtualbox(switch.is_active()));
                                    }
                                }
                            },
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
        let model = ContainersModel {
            enable_docker: false,
            enable_podman: false,
            enable_apptainer: false,
            enable_distrobox: false,
            enable_qemu: false,
            enable_virtualbox: false,
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            Self::Input::Docker(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Docker"
                    } else {
                        "Disabling Docker"
                    }
                );

                self.enable_docker = switched_on;

                widgets
                    .distrobox
                    .set_sensitive(self.enable_docker || self.enable_podman);
                if !(self.enable_docker || self.enable_podman) {
                    widgets.distrobox_switch.set_active(false);
                }
            },
            Self::Input::Podman(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Podman"
                    } else {
                        "Disabling Podman"
                    }
                );

                self.enable_podman = switched_on;

                widgets
                    .distrobox
                    .set_sensitive(self.enable_docker || self.enable_podman);
                if !(self.enable_docker || self.enable_podman) {
                    widgets.distrobox_switch.set_active(false);
                }
            },
            Self::Input::Distrobox(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Distrobox"
                    } else {
                        "Disabling Distrobox"
                    }
                );

                self.enable_distrobox = switched_on;
            },
            Self::Input::Apptainer(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Apptainer"
                    } else {
                        "Disabling Apptainer"
                    }
                );

                self.enable_distrobox = switched_on;
            },
            Self::Input::QEMU(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling QEMU"
                    } else {
                        "Disabling QEMU"
                    }
                );

                self.enable_qemu = switched_on;
            },
            Self::Input::Virtualbox(switched_on) => {
                tracing::info!(
                    "{}",
                    if switched_on {
                        "Enabling Virtualbox"
                    } else {
                        "Disabling Virtualbox"
                    }
                );

                self.enable_virtualbox = switched_on;
            },
        }

        let mut commands: Vec<&str> = Vec::new();

        if self.enable_podman {
            commands.push("sudo apt-get -y docker.io");
        }

        if self.enable_podman {
            commands.push("sudo apt-get -y podman");
        }

        if self.enable_distrobox && (self.enable_docker || self.enable_podman) {
            commands.push("sudo PACSTALL_DOWNLOADER=quiet-wget pacstall -PI distrobox");
        }

        if self.enable_apptainer {
            commands.push("sudo apt-get install apptainer");
        }

        if self.enable_qemu {
            commands.push("sudo apt-get install qemu-system qemu-user-static qemu-utils");
        }

        if self.enable_virtualbox {
            commands.push("sudo apt-get install virtualbox");
        }

        COMMANDS.write_inner().insert("containers", commands);
    }
}
