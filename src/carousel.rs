use relm4::adw::prelude::*;
use relm4::{adw, ComponentParts, ComponentSender, SimpleComponent};

pub(crate) struct CarouselModel;

#[derive(Debug)]
pub(crate) struct CarouselMsg;

#[relm4::component(pub)]
impl SimpleComponent for CarouselModel {
    type Init = ();
    type Input = CarouselMsg;
    type Output = ();
    type Widgets = CarouselWidgets;

    view! {
        adw::Carousel {
            set_vexpand: true,
            set_hexpand: true,
            set_allow_scroll_wheel: false,
            set_allow_mouse_drag: false,
            set_allow_long_swipes: false,
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = CarouselModel {};

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, _message: Self::Input, _sender: relm4::ComponentSender<Self>) {}
}
