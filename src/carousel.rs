use std::collections::HashMap;

use relm4::adw::prelude::*;
use relm4::{
    adw, Component, ComponentController, ComponentParts, ComponentSender, Controller, SharedState,
    SimpleComponent,
};

use crate::extra_settings::{ExtraSettingsModel, ExtraSettingsOutput};
use crate::package_manager::{PackageManagerModel, PackageManagerOutput};
use crate::progress::{ProgressInput, ProgressModel};
use crate::theme::{ThemeModel, ThemeOutput};
use crate::welcome::{WelcomeModel, WelcomeOutput};

/// Gathers all the commands to be executed from different pages.
pub(crate) static COMMANDS: SharedState<HashMap<&'static str, Vec<&'static str>>> =
    SharedState::new();

pub(crate) struct CarouselModel {
    current_page: u32,

    welcome_page: Controller<WelcomeModel>,
    theme_page: Controller<ThemeModel>,
    package_manager_page: Controller<PackageManagerModel>,
    extra_settings_page: Controller<ExtraSettingsModel>,
    progress_page: Controller<ProgressModel>,
}

#[derive(Debug)]
pub(crate) enum CarouselInput {
    /// Move to next page.
    NextPage,
    /// Move to the previous page.
    PreviousPage,
}

#[derive(Debug)]
pub(crate) enum CarouselOutput {
    /// Show the back button.
    ShowBackButton,
    /// Hide the back button.
    HideBackButton,
}

#[relm4::component(pub)]
impl SimpleComponent for CarouselModel {
    type Init = ();
    type Input = CarouselInput;
    type Output = CarouselOutput;
    type Widgets = CarouselWidgets;

    view! {
        #[name = "carousel"]
        adw::Carousel {
            set_vexpand: true,
            set_hexpand: true,
            set_allow_scroll_wheel: false,
            set_allow_mouse_drag: false,
            set_allow_long_swipes: false,

            append: model.welcome_page.widget(),
            append: model.theme_page.widget(),
            append: model.package_manager_page.widget(),
            append: model.extra_settings_page.widget(),
            append: model.progress_page.widget(),
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = CarouselModel {
            current_page: 0,
            welcome_page: WelcomeModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    WelcomeOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            theme_page: ThemeModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    ThemeOutput::NextPage => CarouselInput::NextPage,
                }),
            package_manager_page: PackageManagerModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    PackageManagerOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            extra_settings_page: ExtraSettingsModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    ExtraSettingsOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            progress_page: ProgressModel::builder().launch(()).detach(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            CarouselInput::NextPage => {
                sender.output(CarouselOutput::ShowBackButton);
                self.current_page += 1;

                // When the user is at the progress page.
                if self.current_page == 4 {
                    // Hide the back button, as the user is not supposed to return to previous pages
                    // after this point.
                    sender.output(CarouselOutput::HideBackButton);

                    self.progress_page
                        .sender()
                        .send(ProgressInput::StartInstallation);
                }
            },
            CarouselInput::PreviousPage => {
                // When on the second page (pages starts from 0), disable the back button while
                // going back.
                if self.current_page == 1 {
                    sender.output(CarouselOutput::HideBackButton);
                }
                self.current_page -= 1;
            },
        }
    }

    fn post_view() { carousel.scroll_to(&carousel.nth_page(model.current_page), true); }
}
