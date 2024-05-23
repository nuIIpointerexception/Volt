use ::anyhow::Result;

use crate::{
    graphics::triangles::Frame,
    ui::{Input, InternalState, primitives::Dimensions},
    Vec2,
};

pub use self::{
    align::{Align, HAlignment, VAlignment},
    button::Button,
    col::Col,
    composite::{ComposedElement, ComposedMessage, Composite, CompositeWidget},
    container::{Constraint, Container, WithContainer},
    element::Element,
    hsplit::HSplit,
    label::Label,
    row::Row,
    slider::Slider,
    window::Window,
};

mod align;
mod button;
mod col;
mod composite;
mod container;
mod element;
mod hsplit;
mod label;
mod row;
mod slider;
mod window;

pub mod prelude;

pub trait Widget<Message> {
    fn handle_event(
        &mut self,
        internal_state: &mut InternalState,
        input: &Input,
        event: &glfw::WindowEvent,
    ) -> Result<Option<Message>>;

    fn draw_frame(
        &self,
        internal_state: &mut InternalState,
        frame: &mut Frame,
    ) -> Result<()>;

    fn dimensions(
        &mut self,
        internal_state: &mut InternalState,
        max_size: &Dimensions,
    ) -> Dimensions;

    fn set_top_left_position(
        &mut self,
        internal_state: &mut InternalState,
        position: Vec2,
    );
}
