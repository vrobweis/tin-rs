use crate::{key::TinKey, point::TinPoint};


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