use std::time::Duration;

use smithay::{
    backend::{
        input::KeyState,
        renderer::{
            element::{
                solid::SolidColorRenderElement, surface::WaylandSurfaceRenderElement,
                AsRenderElements,
            },
            ImportAll, ImportMem, Renderer, Texture,
        },
    },
    desktop::{space::SpaceElement, utils::OutputPresentationFeedback, Window, WindowSurfaceType},
    input::{
        keyboard::{KeyboardTarget, KeysymHandle, ModifiersState},
        pointer::{
            AxisFrame, ButtonEvent, GestureHoldBeginEvent, GestureHoldEndEvent,
            GesturePinchBeginEvent, GesturePinchEndEvent, GesturePinchUpdateEvent,
            GestureSwipeBeginEvent, GestureSwipeEndEvent, GestureSwipeUpdateEvent, MotionEvent,
            PointerTarget, RelativeMotionEvent,
        },
        Seat,
    },
    output::Output,
    reexports::{
        wayland_protocols::wp::presentation_time::server::wp_presentation_feedback,
        wayland_server::protocol::wl_surface::WlSurface,
    },
    render_elements,
    utils::{user_data::UserDataMap, IsAlive, Logical, Physical, Point, Rectangle, Scale, Serial},
    wayland::{
        compositor::SurfaceData as WlSurfaceData, dmabuf::DmabufFeedback, seat::WaylandFocus,
    },
};

use crate::BuedchenState;

#[derive(Debug, Clone, PartialEq)]
pub struct WindowElement(pub Window);

impl WindowElement {
    pub fn surface_under(
        &self,
        location: Point<f64, Logical>,
        window_type: WindowSurfaceType,
    ) -> Option<(WlSurface, Point<i32, Logical>)> {
        self.0.surface_under(location, window_type)
    }

    pub fn with_surfaces<F>(&self, processor: F)
    where
        F: FnMut(&WlSurface, &WlSurfaceData) + Copy,
    {
        self.0.with_surfaces(processor)
    }

    pub fn send_frame<T, F>(
        &self,
        output: &Output,
        time: T,
        throttle: Option<Duration>,
        primary_scan_out_output: F,
    ) where
        T: Into<Duration>,
        F: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
    {
        self.0
            .send_frame(output, time, throttle, primary_scan_out_output)
    }

    pub fn send_dmabuf_feedback<'a, P, F>(
        &self,
        output: &Output,
        primary_scan_out_output: P,
        select_dmabuf_feedback: F,
    ) where
        P: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
        F: Fn(&WlSurface, &WlSurfaceData) -> &'a DmabufFeedback + Copy,
    {
        self.0
            .send_dmabuf_feedback(output, primary_scan_out_output, select_dmabuf_feedback)
    }

    pub fn take_presentation_feedback<F1, F2>(
        &self,
        output_feedback: &mut OutputPresentationFeedback,
        primary_scan_out_output: F1,
        presentation_feedback_flags: F2,
    ) where
        F1: FnMut(&WlSurface, &WlSurfaceData) -> Option<Output> + Copy,
        F2: FnMut(&WlSurface, &WlSurfaceData) -> wp_presentation_feedback::Kind + Copy,
    {
        self.0.take_presentation_feedback(
            output_feedback,
            primary_scan_out_output,
            presentation_feedback_flags,
        )
    }

    pub fn wl_surface(&self) -> Option<WlSurface> {
        self.0.wl_surface()
    }

    pub fn user_data(&self) -> &UserDataMap {
        self.0.user_data()
    }
}

impl IsAlive for WindowElement {
    fn alive(&self) -> bool {
        self.0.alive()
    }
}

impl<Backend: crate::state::Backend> PointerTarget<BuedchenState<Backend>> for WindowElement {
    fn enter(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &MotionEvent,
    ) {
        PointerTarget::enter(&self.0, seat, data, event)
    }
    fn motion(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &MotionEvent,
    ) {
        PointerTarget::motion(&self.0, seat, data, event)
    }
    fn relative_motion(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &RelativeMotionEvent,
    ) {
        PointerTarget::relative_motion(&self.0, seat, data, event)
    }
    fn button(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &ButtonEvent,
    ) {
        PointerTarget::button(&self.0, seat, data, event)
    }
    fn axis(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        frame: AxisFrame,
    ) {
        PointerTarget::axis(&self.0, seat, data, frame)
    }
    fn frame(&self, seat: &Seat<BuedchenState<Backend>>, data: &mut BuedchenState<Backend>) {
        PointerTarget::frame(&self.0, seat, data)
    }
    fn leave(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        serial: Serial,
        time: u32,
    ) {
        PointerTarget::leave(&self.0, seat, data, serial, time)
    }
    fn gesture_swipe_begin(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &GestureSwipeBeginEvent,
    ) {
        PointerTarget::gesture_swipe_begin(&self.0, seat, data, event)
    }
    fn gesture_swipe_update(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &GestureSwipeUpdateEvent,
    ) {
        PointerTarget::gesture_swipe_update(&self.0, seat, data, event)
    }
    fn gesture_swipe_end(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &GestureSwipeEndEvent,
    ) {
        PointerTarget::gesture_swipe_end(&self.0, seat, data, event)
    }
    fn gesture_pinch_begin(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &GesturePinchBeginEvent,
    ) {
        PointerTarget::gesture_pinch_begin(&self.0, seat, data, event)
    }
    fn gesture_pinch_update(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &GesturePinchUpdateEvent,
    ) {
        PointerTarget::gesture_pinch_update(&self.0, seat, data, event)
    }
    fn gesture_pinch_end(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &GesturePinchEndEvent,
    ) {
        PointerTarget::gesture_pinch_end(&self.0, seat, data, event)
    }
    fn gesture_hold_begin(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &GestureHoldBeginEvent,
    ) {
        PointerTarget::gesture_hold_begin(&self.0, seat, data, event)
    }
    fn gesture_hold_end(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        event: &GestureHoldEndEvent,
    ) {
        PointerTarget::gesture_hold_end(&self.0, seat, data, event)
    }
}

impl<Backend: crate::state::Backend> KeyboardTarget<BuedchenState<Backend>> for WindowElement {
    fn enter(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        keys: Vec<KeysymHandle<'_>>,
        serial: Serial,
    ) {
        KeyboardTarget::enter(&self.0, seat, data, keys, serial)
    }
    fn leave(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        serial: Serial,
    ) {
        KeyboardTarget::leave(&self.0, seat, data, serial)
    }
    fn key(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        key: KeysymHandle<'_>,
        state: KeyState,
        serial: Serial,
        time: u32,
    ) {
        KeyboardTarget::key(&self.0, seat, data, key, state, serial, time)
    }
    fn modifiers(
        &self,
        seat: &Seat<BuedchenState<Backend>>,
        data: &mut BuedchenState<Backend>,
        modifiers: ModifiersState,
        serial: Serial,
    ) {
        KeyboardTarget::modifiers(&self.0, seat, data, modifiers, serial)
    }
}

impl SpaceElement for WindowElement {
    fn geometry(&self) -> Rectangle<i32, Logical> {
        SpaceElement::geometry(&self.0)
    }
    fn bbox(&self) -> Rectangle<i32, Logical> {
        SpaceElement::bbox(&self.0)
    }
    fn is_in_input_region(&self, point: &Point<f64, Logical>) -> bool {
        SpaceElement::is_in_input_region(&self.0, point)
    }
    fn z_index(&self) -> u8 {
        SpaceElement::z_index(&self.0)
    }

    fn set_activate(&self, activated: bool) {
        SpaceElement::set_activate(&self.0, activated)
    }
    fn output_enter(&self, output: &Output, overlap: Rectangle<i32, Logical>) {
        SpaceElement::output_enter(&self.0, output, overlap)
    }
    fn output_leave(&self, output: &Output) {
        SpaceElement::output_leave(&self.0, output)
    }
    #[profiling::function]
    fn refresh(&self) {
        SpaceElement::refresh(&self.0)
    }
}

render_elements!(
    pub WindowRenderElement<R> where R: ImportAll + ImportMem;
    Window=WaylandSurfaceRenderElement<R>,
    Decoration=SolidColorRenderElement,
);

impl<R: Renderer> std::fmt::Debug for WindowRenderElement<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Window(arg0) => f.debug_tuple("Window").field(arg0).finish(),
            Self::Decoration(arg0) => f.debug_tuple("Decoration").field(arg0).finish(),
            Self::_GenericCatcher(arg0) => f.debug_tuple("_GenericCatcher").field(arg0).finish(),
        }
    }
}

impl<R> AsRenderElements<R> for WindowElement
where
    R: Renderer + ImportAll + ImportMem,
    <R as Renderer>::TextureId: Texture + 'static,
{
    type RenderElement = WindowRenderElement<R>;

    fn render_elements<C: From<Self::RenderElement>>(
        &self,
        renderer: &mut R,
        location: Point<i32, Physical>,
        scale: Scale<f64>,
        alpha: f32,
    ) -> Vec<C> {
        AsRenderElements::<R>::render_elements::<WindowRenderElement<R>>(
            &self.0, renderer, location, scale, alpha,
        )
        .into_iter()
        .map(C::from)
        .collect()
    }
}
