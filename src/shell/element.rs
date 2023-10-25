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

use crate::AnvilState;

#[derive(Debug, Clone, PartialEq)]
pub enum WindowElement {
    Wayland(Window),
}

impl WindowElement {
    pub fn surface_under(
        &self,
        location: Point<f64, Logical>,
        window_type: WindowSurfaceType,
    ) -> Option<(WlSurface, Point<i32, Logical>)> {
        match self {
            WindowElement::Wayland(w) => w.surface_under(location, window_type),
        }
    }

    pub fn with_surfaces<F>(&self, processor: F)
    where
        F: FnMut(&WlSurface, &WlSurfaceData) + Copy,
    {
        match self {
            WindowElement::Wayland(w) => w.with_surfaces(processor),
        }
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
        match self {
            WindowElement::Wayland(w) => {
                w.send_frame(output, time, throttle, primary_scan_out_output)
            }
        }
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
        match self {
            WindowElement::Wayland(w) => {
                w.send_dmabuf_feedback(output, primary_scan_out_output, select_dmabuf_feedback)
            }
        }
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
        match self {
            WindowElement::Wayland(w) => w.take_presentation_feedback(
                output_feedback,
                primary_scan_out_output,
                presentation_feedback_flags,
            ),
        }
    }

    pub fn is_wayland(&self) -> bool {
        matches!(self, WindowElement::Wayland(_))
    }

    pub fn wl_surface(&self) -> Option<WlSurface> {
        match self {
            WindowElement::Wayland(w) => w.wl_surface(),
        }
    }

    pub fn user_data(&self) -> &UserDataMap {
        match self {
            WindowElement::Wayland(w) => w.user_data(),
        }
    }
}

impl IsAlive for WindowElement {
    fn alive(&self) -> bool {
        match self {
            WindowElement::Wayland(w) => w.alive(),
        }
    }
}

impl<Backend: crate::state::Backend> PointerTarget<AnvilState<Backend>> for WindowElement {
    fn enter(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &MotionEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::enter(w, seat, data, event),
        };
    }
    fn motion(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &MotionEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::motion(w, seat, data, event),
        };
    }
    fn relative_motion(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &RelativeMotionEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::relative_motion(w, seat, data, event),
        }
    }
    fn button(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &ButtonEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::button(w, seat, data, event),
        };
    }
    fn axis(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        frame: AxisFrame,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::axis(w, seat, data, frame),
        }
    }
    fn frame(&self, seat: &Seat<AnvilState<Backend>>, data: &mut AnvilState<Backend>) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::frame(w, seat, data),
        }
    }
    fn leave(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        serial: Serial,
        time: u32,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::leave(w, seat, data, serial, time),
        };
    }
    fn gesture_swipe_begin(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &GestureSwipeBeginEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::gesture_swipe_begin(w, seat, data, event),
        }
    }
    fn gesture_swipe_update(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &GestureSwipeUpdateEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::gesture_swipe_update(w, seat, data, event),
        }
    }
    fn gesture_swipe_end(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &GestureSwipeEndEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::gesture_swipe_end(w, seat, data, event),
        }
    }
    fn gesture_pinch_begin(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &GesturePinchBeginEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::gesture_pinch_begin(w, seat, data, event),
        }
    }
    fn gesture_pinch_update(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &GesturePinchUpdateEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::gesture_pinch_update(w, seat, data, event),
        }
    }
    fn gesture_pinch_end(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &GesturePinchEndEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::gesture_pinch_end(w, seat, data, event),
        }
    }
    fn gesture_hold_begin(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &GestureHoldBeginEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::gesture_hold_begin(w, seat, data, event),
        }
    }
    fn gesture_hold_end(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        event: &GestureHoldEndEvent,
    ) {
        match self {
            WindowElement::Wayland(w) => PointerTarget::gesture_hold_end(w, seat, data, event),
        }
    }
}

impl<Backend: crate::state::Backend> KeyboardTarget<AnvilState<Backend>> for WindowElement {
    fn enter(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        keys: Vec<KeysymHandle<'_>>,
        serial: Serial,
    ) {
        match self {
            WindowElement::Wayland(w) => KeyboardTarget::enter(w, seat, data, keys, serial),
        }
    }
    fn leave(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        serial: Serial,
    ) {
        match self {
            WindowElement::Wayland(w) => KeyboardTarget::leave(w, seat, data, serial),
        }
    }
    fn key(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        key: KeysymHandle<'_>,
        state: KeyState,
        serial: Serial,
        time: u32,
    ) {
        match self {
            WindowElement::Wayland(w) => {
                KeyboardTarget::key(w, seat, data, key, state, serial, time)
            }
        }
    }
    fn modifiers(
        &self,
        seat: &Seat<AnvilState<Backend>>,
        data: &mut AnvilState<Backend>,
        modifiers: ModifiersState,
        serial: Serial,
    ) {
        match self {
            WindowElement::Wayland(w) => {
                KeyboardTarget::modifiers(w, seat, data, modifiers, serial)
            }
        }
    }
}

impl SpaceElement for WindowElement {
    fn geometry(&self) -> Rectangle<i32, Logical> {
        match self {
            WindowElement::Wayland(w) => SpaceElement::geometry(w),
        }
    }
    fn bbox(&self) -> Rectangle<i32, Logical> {
        match self {
            WindowElement::Wayland(w) => SpaceElement::bbox(w),
        }
    }
    fn is_in_input_region(&self, point: &Point<f64, Logical>) -> bool {
        match self {
            WindowElement::Wayland(w) => SpaceElement::is_in_input_region(w, point),
        }
    }
    fn z_index(&self) -> u8 {
        match self {
            WindowElement::Wayland(w) => SpaceElement::z_index(w),
        }
    }

    fn set_activate(&self, activated: bool) {
        match self {
            WindowElement::Wayland(w) => SpaceElement::set_activate(w, activated),
        }
    }
    fn output_enter(&self, output: &Output, overlap: Rectangle<i32, Logical>) {
        match self {
            WindowElement::Wayland(w) => SpaceElement::output_enter(w, output, overlap),
        }
    }
    fn output_leave(&self, output: &Output) {
        match self {
            WindowElement::Wayland(w) => SpaceElement::output_leave(w, output),
        }
    }
    #[profiling::function]
    fn refresh(&self) {
        match self {
            WindowElement::Wayland(w) => SpaceElement::refresh(w),
        }
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
        match self {
            WindowElement::Wayland(xdg) => AsRenderElements::<R>::render_elements::<
                WindowRenderElement<R>,
            >(xdg, renderer, location, scale, alpha),
        }
        .into_iter()
        .map(C::from)
        .collect()
    }
}
