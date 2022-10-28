use std::process::Command;

use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::gtk::gio;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub(crate) struct ThemeModel;

#[derive(Debug)]
pub(crate) enum ThemeInput {
    EnableDarkTheme,
    EnableLightTheme,
}

#[derive(Debug)]
pub(crate) enum ThemeOutput {
    /// Move to the next page.
    NextPage,
    /// Move to the error page.
    ErrorOccured,
}

#[relm4::component(pub)]
impl SimpleComponent for ThemeModel {
    type Init = ();
    type Input = ThemeInput;
    type Output = ThemeOutput;
    type Widgets = ThemeWidgets;

    view! {
        #[root]
        gtk::Box {
            set_orientation: gtk::Orientation::Vertical,
            set_halign: gtk::Align::Fill,
            set_valign: gtk::Align::Center,
            set_hexpand: true,

            gtk::Box {
                set_valign: gtk::Align::Center,
                set_spacing: 10,
                set_halign: gtk::Align::Center,

                #[name = "light_button"]
                gtk::CheckButton {
                    set_tooltip_text: Some("Default"),
                    set_halign: gtk::Align::Center,
                    set_active: true,

                    set_css_classes: &["theme-selector", "light", "card"],

                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() && btn.is_focus() {
                            sender.input(Self::Input::EnableLightTheme);
                        }
                    }
                },

                gtk::CheckButton {
                    set_tooltip_text: Some("Dark"),
                    set_halign: gtk::Align::Center,

                    // Add `light_button` to the group, turning both of them mutually exclusive.
                    set_group: Some(&light_button),

                    set_css_classes: &["theme-selector", "dark", "card"],

                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() && btn.is_focus() {
                            sender.input(Self::Input::EnableDarkTheme);
                        }
                    }
                }
            },

            adw::StatusPage {
                set_title: &gettext("Color Scheme"),
                set_description: Some(&gettext("Choose a color scheme for your system.")),
                set_halign: gtk::Align::Fill,
                set_valign: gtk::Align::Fill,
                set_hexpand: true,

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

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = ThemeModel {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            Self::Input::EnableLightTheme => {
                log::debug!("Enabling on Light theme");
                if let Err(error) = Command::new("xfconf-query")
                    .args(&[
                        "--channel",
                        "xsettings",
                        "--property",
                        "/Net/ThemeName",
                        "--set",
                        "Yaru-purple",
                    ])
                    .status()
                {
                    log::error!("Error enabling light theme: {}", error);
                    sender.output(Self::Output::ErrorOccured);
                }

                if let Err(error) = gio::Settings::new("org.gnome.desktop.interface")
                    .set_string("color-scheme", "default")
                {
                    log::error!("Unable to change gsettings: {}", error);
                    sender.output(Self::Output::ErrorOccured);
                }
            },
            Self::Input::EnableDarkTheme => {
                log::debug!("Enabling Dark theme");
                if let Err(error) = Command::new("xfconf-query")
                    .args(&[
                        "--channel",
                        "xsettings",
                        "--property",
                        "/Net/ThemeName",
                        "--set",
                        "Yaru-purple-dark",
                    ])
                    .status()
                {
                    log::error!("Error enabling dark theme: {}", error);
                    sender.output(Self::Output::ErrorOccured);
                }

                if let Err(error) = gio::Settings::new("org.gnome.desktop.interface")
                    .set_string("color-scheme", "prefer-dark")
                {
                    log::error!("Unable to change gsettings: {}", error);
                    sender.output(Self::Output::ErrorOccured);
                }
            },
        }
    }
}
