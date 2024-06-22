use relm4::adw::prelude::*;
use relm4::{adw, Component, ComponentController, ComponentParts, Controller, SimpleComponent};

use crate::containers::{ContainersModel, ContainersOutput};
use crate::done::DoneModel;
use crate::extra_settings::{ExtraSettingsModel, ExtraSettingsOutput};
use crate::package_manager::{PackageManagerModel, PackageManagerOutput};
use crate::progress::{ProgressInput, ProgressModel, ProgressOutput};
use crate::theme::{ThemeModel, ThemeOutput};
use crate::welcome::{WelcomeModel, WelcomeOutput};

pub(crate) struct CarouselModel {
    current_page: u32,

    welcome_page: Controller<WelcomeModel>,
    theme_page: Controller<ThemeModel>,
    package_manager_page: Controller<PackageManagerModel>,
    containers_page: Controller<ContainersModel>,
    extra_settings_page: Controller<ExtraSettingsModel>,
    progress_page: Controller<ProgressModel>,
    done_page: Controller<DoneModel>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum CarouselInput {
    /// Move to next page.
    NextPage,
    /// Move to the previous page.
    PreviousPage,
    /// An error has occurred in one of the pages.
    /// Move to the [crate::done] page, with the error state.
    SkipToErrorPage,
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
            append: model.containers_page.widget(),
            append: model.extra_settings_page.widget(),
            append: model.progress_page.widget(),
            append: model.done_page.widget(),
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
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
                    ThemeOutput::ErrorOccured => CarouselInput::SkipToErrorPage,
                }),
            package_manager_page: PackageManagerModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    PackageManagerOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            containers_page: ContainersModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    ContainersOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            extra_settings_page: ExtraSettingsModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    ExtraSettingsOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            progress_page: ProgressModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    ProgressOutput::InstallationComplete => CarouselInput::NextPage,
                    ProgressOutput::InstallationError => CarouselInput::SkipToErrorPage,
                },
            ),
            done_page: DoneModel::builder().launch(()).detach(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            CarouselInput::NextPage => {
                self.current_page += 1;

                // If the user hasn't reached the progress page yet.
                if self.current_page < 5 {
                    sender.output(CarouselOutput::ShowBackButton).unwrap();
                }

                // When the user is at the progress page.
                if self.current_page == 5 {
                    // Hide the back button, as the user is not supposed to return to previous pages
                    // after this point.
                    sender.output(CarouselOutput::HideBackButton).unwrap();

                    self.progress_page
                        .sender()
                        .send(ProgressInput::StartInstallation)
                        .unwrap();
                }
            },
            CarouselInput::PreviousPage => {
                self.current_page -= 1;

                // When on the first page (pages starts from 0), disable the back button.
                if self.current_page == 0 {
                    sender.output(CarouselOutput::HideBackButton).unwrap();
                }
            },
            CarouselInput::SkipToErrorPage => {
                self.current_page = 6;
                self.done_page
                    .sender()
                    .send(crate::done::DoneInput::SwitchToErrorState)
                    .unwrap();
                sender.output(CarouselOutput::HideBackButton).unwrap();
            },
        }
    }

    fn post_view() { carousel.scroll_to(&carousel.nth_page(model.current_page), true); }
}
