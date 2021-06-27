use crate::{key::TinKey, point::TinPoint};

pub enum TinEvent {
    KeyDown(TinKey),
    KeyUp(TinKey),

    MouseMoved(TinPoint),

    MouseDown,
    MouseUp,

    RightMouseDown,
    RightMouseUp,

    OtherMouseDown,
    OtherMouseUp,

    Unknown,
}
