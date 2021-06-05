use crate::{key::TinKey, point::TinPoint, scene::TScene};

#[allow(type_alias_bounds)]
pub(crate) type TEventHandler<S: TScene> = fn(TinEvent, &mut S, &mut crate::view::TinView);

pub enum TinEvent {
    KeyDown(TinKey),
    KeyUp(TinKey),

    MouseMoved(TinPoint),

    MouseDown(TinPoint),
    MouseUp(TinPoint),

    RightMouseDown(TinPoint),
    RightMouseUp(TinPoint),

    OtherMouseDown(TinPoint),
    OtherMouseUp(TinPoint),
}