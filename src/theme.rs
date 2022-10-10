use std::process::Command;

use relm4::adw::prelude::*;
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub(crate) struct ThemeModel;

#[derive(Debug)]
pub(crate) enum ThemeMsg {
    EnableDarkTheme,
    EnableLightTheme,
}

#[relm4::component(pub)]
impl SimpleComponent for ThemeModel {
    type Init = ();
    type Input = ThemeMsg;
    type Output = ();
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
                            sender.input(ThemeMsg::EnableLightTheme);
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
                            sender.input(ThemeMsg::EnableDarkTheme);
                        }
                    }
                }
            },

            adw::StatusPage {
                set_title: "Color Scheme",
                set_description: Some("Choose a color scheme for your system."),
                set_halign: gtk::Align::Fill,
                set_valign: gtk::Align::Fill,
                set_hexpand: true,

                gtk::Button::with_label("Next") {
                    set_halign: gtk::Align::Center,
                    set_css_classes: &["pill", "suggested-action"]
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

    fn update(&mut self, message: Self::Input, _sender: relm4::ComponentSender<Self>) {
        match message {
            ThemeMsg::EnableLightTheme => {
                log::debug!("Enabling on Light theme");
                if let Err(error) = Command::new("xfconf-query")
                    .args(&[
                        "--channel",
                        "xsettings",
                        "--property",
                        "/Net/ThemeName",
                        "--set",
                        "Yaru-Light",
                    ])
                    .status()
                {
                    log::error!("Error enabling light theme: {}", error);
                }
            },
            ThemeMsg::EnableDarkTheme => {
                log::debug!("Enabling Dark theme");
                if let Err(error) = Command::new("xfconf-query")
                    .args(&[
                        "--channel",
                        "xsettings",
                        "--property",
                        "/Net/ThemeName",
                        "--set",
                        "Yaru-Dark",
                    ])
                    .status()
                {
                    log::error!("Error enabling dark theme: {}", error);
                }
            },
        }
    }
}
