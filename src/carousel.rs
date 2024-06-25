use relm4::adw::prelude::*;
use relm4::{adw, Component, ComponentController, ComponentParts, Controller, SimpleComponent};

use crate::containers::{ContainersModel, ContainersOutput};
use crate::done::DoneModel;
use crate::extra_settings::{ExtraSettingsModel, ExtraSettingsOutput};
use crate::package_manager::{PackageManagerModel, PackageManagerOutput};
use crate::progress::{ProgressInput, ProgressModel, ProgressOutput};
use crate::theme::{ThemeModel, ThemeOutput};
use crate::welcome::{WelcomeModel, WelcomeOutput};

pub(crate) struct CarouselPagesModel {
    current: u32,

    welcome: Controller<WelcomeModel>,
    theme: Controller<ThemeModel>,
    package_manager: Controller<PackageManagerModel>,
    containers: Controller<ContainersModel>,
    extra_settings: Controller<ExtraSettingsModel>,
    progress: Controller<ProgressModel>,
    done: Controller<DoneModel>,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum CarouselInput {
    /// Move to next page.
    NextPage,
    /// Move to the previous page.
    PreviousPage,
    /// An error has occurred in one of the pages.
    /// Move to the [`crate::done`] page, with the error state.
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
impl SimpleComponent for CarouselPagesModel {
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

            append: model.welcome.widget(),
            append: model.theme.widget(),
            append: model.package_manager.widget(),
            append: model.containers.widget(),
            append: model.extra_settings.widget(),
            append: model.progress.widget(),
            append: model.done.widget(),
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = CarouselPagesModel {
            current: 0,
            welcome: WelcomeModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    WelcomeOutput::NextPage => CarouselInput::NextPage,
                }),
            theme: ThemeModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    ThemeOutput::NextPage => CarouselInput::NextPage,
                    ThemeOutput::ErrorOccured => CarouselInput::SkipToErrorPage,
                },
            ),
            package_manager: PackageManagerModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    PackageManagerOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            containers: ContainersModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    ContainersOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            extra_settings: ExtraSettingsModel::builder().launch(()).forward(
                sender.input_sender(),
                |msg| match msg {
                    ExtraSettingsOutput::NextPage => CarouselInput::NextPage,
                },
            ),
            progress: ProgressModel::builder()
                .launch(())
                .forward(sender.input_sender(), |msg| match msg {
                    ProgressOutput::InstallationComplete => CarouselInput::NextPage,
                    ProgressOutput::InstallationError => CarouselInput::SkipToErrorPage,
                }),
            done: DoneModel::builder().launch(()).detach(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: relm4::ComponentSender<Self>) {
        match message {
            CarouselInput::NextPage => {
                self.current += 1;

                // If the user hasn't reached the progress page yet.
                if self.current < 5 {
                    sender.output(CarouselOutput::ShowBackButton).unwrap();
                }

                // When the user is at the progress page.
                if self.current == 5 {
                    // Hide the back button, as the user is not supposed to return to previous pages
                    // after this point.
                    sender.output(CarouselOutput::HideBackButton).unwrap();

                    self.progress
                        .sender()
                        .send(ProgressInput::StartInstallation)
                        .unwrap();
                }
            },
            CarouselInput::PreviousPage => {
                self.current -= 1;

                // When on the first page (pages starts from 0), disable the back button.
                if self.current == 0 {
                    sender.output(CarouselOutput::HideBackButton).unwrap();
                }
            },
            CarouselInput::SkipToErrorPage => {
                self.current = 6;
                self.done
                    .sender()
                    .send(crate::done::DoneInput::SwitchToErrorState)
                    .unwrap();
                sender.output(CarouselOutput::HideBackButton).unwrap();
            },
        }
    }

    fn post_view() { carousel.scroll_to(&carousel.nth_page(model.current), true); }
}
