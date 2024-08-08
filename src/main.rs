#![allow(clippy::used_underscore_binding)]

use std::collections::HashMap;
use std::env;
use std::path::Path;

use carousel::{CarouselInput, CarouselOutput, CarouselPagesModel};
use config::{APP_ID, GETTEXT_PACKAGE, LOCALEDIR, PROFILE, RESOURCES_FILE};
use gettextrs::{gettext, LocaleCategory};
use relm4::adw::prelude::*;
use relm4::gtk::{gdk, gio, glib};
use relm4::{
    adw, gtk, main_application, Component, ComponentController, ComponentParts, ComponentSender,
    Controller, RelmApp, SharedState, SimpleComponent,
};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::fmt::writer::MakeWriterExt;

mod carousel;
mod config;
mod containers;
mod done;
mod extra_settings;
mod package_manager;
mod progress;
mod theme;
mod welcome;

/// Gathers all the commands to be executed from different pages.
pub(crate) static COMMANDS: SharedState<HashMap<&'static str, Vec<&'static str>>> =
    SharedState::new();

struct AppModel {
    carousel: Controller<CarouselPagesModel>,
    back_button_visible: bool,
}

#[derive(Debug)]
enum AppInput {
    ShowBackButton,
    HideBackButton,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Init = ();
    type Input = AppInput;
    type Output = ();
    // AppWidgets is generated by the macro
    type Widgets = AppWidgets;

    view! {
        adw::ApplicationWindow {
            set_default_width: 800,
            set_default_height: 900,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,


                adw::HeaderBar {
                    set_css_classes: &["flat"],

                    #[wrap(Some)]
                    set_title_widget = &adw::CarouselIndicatorDots::builder().orientation(gtk::Orientation::Horizontal).build() {
                        set_carousel: Some(carousel),
                    },


                    pack_start = &gtk::Button::with_label(&gettext("Back")) {
                        set_halign: gtk::Align::Center,
                        #[watch]
                        set_visible: model.back_button_visible,

                        connect_clicked[carousel_sender] => move |_| {
                            carousel_sender.send(CarouselInput::PreviousPage).unwrap();
                        }
                    },
                },


                adw::ToastOverlay {
                    #[wrap(Some)]
                    #[local_ref]
                     set_child = carousel -> adw::Carousel {
                     }
                },

            },

        }
    }

    // Initialize the UI.
    fn init(
        _counter: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {
            carousel: CarouselPagesModel::builder().launch(()).forward(
                sender.input_sender(),
                |message| match message {
                    CarouselOutput::ShowBackButton => AppInput::ShowBackButton,
                    CarouselOutput::HideBackButton => AppInput::HideBackButton,
                },
            ),
            back_button_visible: false,
        };

        let carousel = model.carousel.widget();
        let carousel_sender = model.carousel.sender().clone();

        // Insert the macro code generation here
        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppInput::ShowBackButton => self.back_button_visible = true,
            AppInput::HideBackButton => self.back_button_visible = false,
        }
    }
}

fn main() {
    let logfile = RollingFileAppender::new(
        Rotation::NEVER,
        Path::new(&env::var("HOME").unwrap()).join(".local/share/"),
        "rhino-setup.log",
    )
    .with_max_level(tracing::Level::DEBUG);

    let stdout = std::io::stdout.with_max_level(tracing::Level::INFO);

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_writer(logfile.and(stdout))
        .init();

    adw::init().unwrap();

    // Prepare i18n
    gettextrs::setlocale(LocaleCategory::LcAll, "");
    gettextrs::bindtextdomain(GETTEXT_PACKAGE, LOCALEDIR).expect("Unable to bind the text domain");
    gettextrs::textdomain(GETTEXT_PACKAGE).expect("Unable to switch to the text domain");

    glib::set_application_name(&gettext("Rhino Setup"));

    let res = gio::Resource::load(RESOURCES_FILE).expect("Could not load gresource file");
    gio::resources_register(&res);

    let provider = gtk::CssProvider::new();
    provider.load_from_resource("/org/rhinolinux/RhinoSetup/style.css");

    if let Some(display) = gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }

    gtk::Window::set_default_icon_name(APP_ID);

    let app = main_application();
    app.set_application_id(Some(APP_ID));
    app.set_resource_base_path(Some("/org/rhinolinux/RhinoSetup/"));

    main_application()
        .downcast_ref::<adw::Application>()
        .unwrap()
        .style_manager()
        .set_color_scheme(adw::ColorScheme::PreferDark);

    let app = RelmApp::from_app(app);
    app.run::<AppModel>(());
}
