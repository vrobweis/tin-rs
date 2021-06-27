use crate::color::TinColor;

pub enum TBrush {
    Fill(TinColor),
    Stroke(TinColor),
    FillAndStroke(TinColor, TinColor),
    Disabled,
}
