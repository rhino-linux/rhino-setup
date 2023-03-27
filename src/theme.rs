use std::process::Command;

use gettextrs::gettext;
use relm4::adw::prelude::*;
use relm4::adw::StyleManager;
use relm4::gtk::{gdk, gio};
use relm4::{adw, gtk, ComponentParts, ComponentSender, SimpleComponent};

pub(crate) struct ThemeModel {
    style_manager: StyleManager,
}

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


                #[name = "dark_button"]
                gtk::CheckButton {
                    set_tooltip_text: Some("Dark"),
                    set_halign: gtk::Align::Center,
                    set_active: true,

                    set_css_classes: &["theme-selector", "dark", "card"],

                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() && btn.is_focus() {
                            sender.input(Self::Input::EnableDarkTheme);
                        }
                    }
                },

                gtk::CheckButton {
                    set_tooltip_text: Some("Default"),
                    set_halign: gtk::Align::Center,

                    // Add `dark_button` to the group, turning both of them mutually exclusive.
                    set_group: Some(&dark_button),

                    set_css_classes: &["theme-selector", "light", "card"],

                    connect_toggled[sender] => move |btn| {
                        if btn.is_active() && btn.is_focus() {
                            sender.input(Self::Input::EnableLightTheme);
                        }
                    }
                },

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
                        sender.output(Self::Output::NextPage).expect("Failed to send the signal to move to the next page");
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
        let model = ThemeModel {
            style_manager: relm4::main_application()
                .downcast_ref::<adw::Application>()
                .unwrap()
                .style_manager(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        let style_manager = &self.style_manager;
        match message {
            Self::Input::EnableLightTheme => {
                tracing::info!("Enabling on Light theme");
                if let Err(error) = Command::new("xfconf-query")
                    .args([
                        "--channel",
                        "xsettings",
                        "--property",
                        "/Net/ThemeName",
                        "--set",
                        "Yaru-purple",
                    ])
                    .status(){
                if let Err(error) = Command::new("xfconf-query")
                    .args([
                        "--channel",
                        "xsettings",
                        "--property",
                        "/general/theme",
                        "--set",
                        "Yaru",
                    ])
                {
                    tracing::error!("Error enabling light theme: {}", error);
                    sender.output(Self::Output::ErrorOccured).expect("");
                }

                style_manager.set_color_scheme(adw::ColorScheme::ForceLight);

                if let Err(error) = gio::Settings::new("org.gnome.desktop.interface")
                    .set_string("color-scheme", "default")
                {
                    tracing::error!("Unable to change gsettings: {}", error);
                    sender.output(Self::Output::ErrorOccured).expect(
                        "Failed to send the signal to move to the
                error page",
                    );
                }
            },
            Self::Input::EnableDarkTheme => {
                tracing::info!("Enabling Dark theme");
                if let Err(error) = Command::new("xfconf-query")
                    .args([
                        "--channel",
                        "xsettings",
                        "--property",
                        "/Net/ThemeName",
                        "--set",
                        "Yaru-purple-dark",
                    ])
                    .status(){
                if let Err(error) = Command::new("xfconf-query")
                    .args([
                        "--channel",
                        "xsettings",
                        "--property",
                        "/general/theme",
                        "--set",
                        "Yaru-dark",
                    ])
                {
                    tracing::error!("Error enabling dark theme: {}", error);
                    sender
                        .output(Self::Output::ErrorOccured)
                        .expect("Failed to send the signal to move to the error page");
                }

                style_manager.set_color_scheme(adw::ColorScheme::ForceDark);

                if let Err(error) = gio::Settings::new("org.gnome.desktop.interface")
                    .set_string("color-scheme", "prefer-dark")
                {
                    tracing::error!("Unable to change gsettings: {}", error);
                    sender.output(Self::Output::ErrorOccured).expect(
                        "Failed to send the signal to move to the
                error page",
                    );
                }
            },
        }
    }
}
