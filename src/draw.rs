// MARK: - Global drawing methods

use crate::{
    color::*,
    context::get_tin,
    point::{TPoint, TinPoint},
    shapes::*,
    Double,
};

/*
    TODO: Draw calls should add to a queue of commands to help manage draw state, and prevent potential locks.
    Each call should add to a threadsafe Queue that is then synchronously parsed by the backend.
    It also means that the TinContext doesn't have to maintain as much state.
*/

// MARK: - Color state

/// TODO: Document this function.
pub fn stroke_color_from_rgba(red: Double, green: Double, blue: Double, alpha: Double) {
    stroke_color_from_color(TinColor::from_rgba(red, green, blue, alpha))
}

/// TODO: Document this function.
pub fn stroke_color_from_gray_and_alpha(gray: Double, alpha: Double) {
    stroke_color_from_rgba(gray, gray, gray, alpha);
}

/// TODO: Document this function.
pub fn stroke_color_from_gray(gray: Double) {
    stroke_color_from_gray_and_alpha(gray, 1.0);
}

/// TODO: Document this function.
pub fn stroke_color_from_color(color: impl TColor) {
    add_draw_call(DrawCall::Stroke(TinColor::from(color)))
}

/// TODO: Document this function.
pub fn fill_color_from_rgba(red: Double, green: Double, blue: Double, alpha: Double) {
    fill_color_from_color(TinColor::from_rgba(red, green, blue, alpha))
}

/// TODO: Document this function.
pub fn fill_color_from_gray_and_alpha(gray: Double, alpha: Double) {
    fill_color_from_rgba(gray, gray, gray, alpha)
}

/// TODO: Document this function.
pub fn fill_color_from_gray(gray: Double) {
    fill_color_from_color(TinColor::from_rgba(gray, gray, gray, 1.0))
}

/// TODO: Document this function.
pub fn fill_color_from_color(color: impl TColor) {
    add_draw_call(DrawCall::Fill(TinColor::from(color)))
}

/// TODO: Document this function.
pub fn get_stroke_color() -> impl TColor {
    return get_tin().get_stroke_color();
}

/// TODO: Document this function.
pub fn get_fill_color() -> impl TColor {
    return get_tin().get_fill_color();
}

/// TODO: Document this function.
pub fn set_alpha(alpha: Double) {
    add_draw_call(DrawCall::SetAlpha(alpha))
}

/// Clear (erase) the background
pub fn background(red: Double, green: Double, blue: Double) {
    background_with_tin_color(TinColor::from_rgba(red, green, blue, 1.0))
}

/// TODO: Document this function.
pub fn background_gray(gray: Double) {
    background(gray, gray, gray)
}

/// TODO: Document this function.
pub fn background_with_tin_color(color: impl TColor) {
    add_draw_call(DrawCall::Background(TinColor::from(color.clone())));
}

/// TODO: Document this function.
pub fn fill_enable() {
    add_draw_call(DrawCall::FillEnable)
}
/// TODO: Document this function.
pub fn fill_disable() {
    add_draw_call(DrawCall::FillDisable)
}

/// TODO: Document this function.
pub fn stroke_enable() {
    add_draw_call(DrawCall::StrokeEnable)
}
/// TODO: Document this function.
pub fn stroke_disable() {
    add_draw_call(DrawCall::StrokeDisable)
}

/// TODO: Document this function.
pub fn arc(x: Double, y: Double, radius: Double, start_angle: Double, end_angle: Double) {
    let center = TinPoint::from_coords(x, y);
    add_draw_call(DrawCall::Arc(TinArc::new(
        center,
        radius,
        start_angle,
        end_angle,
    )))
}

// Ellipse method

/// Draw an ellipse. Input is centerX, centerY coordinate and width, height size.
pub fn ellipse(center_x: Double, center_y: Double, width: Double, height: Double) {
    let x = center_x - width / 2.0;
    let y = center_y - height / 2.0;
    let r = TinRect::from_dimensions(x, y, width, height);
    add_draw_call(DrawCall::Ellipse(r))
}

// Line methods

/// TODO: Document this function.
pub fn line(x1: Double, y1: Double, x2: Double, y2: Double) {
    let point1 = TinPoint::from_coords(x1, y1);
    let point2 = TinPoint::from_coords(x2, y2);
    add_draw_call(DrawCall::Line(point1, point2))
}

/// TODO: Document this function.
pub fn line_width(width: Double) {
    add_draw_call(DrawCall::LineWidth(width))
}

// Rectangle method

/// Draw a rectangle. Input is left, bottom coordinate and width, height size.
pub fn rect(x: Double, y: Double, width: Double, height: Double) {
    let r = TinRect::from_dimensions(x, y, width, height);
    add_draw_call(DrawCall::Rect(r))
}

/// Draw a rectangle with rounded corners, specified by radius_x, radius_y
pub fn rounded_rect(rect: &TinRect, radius_x: Double, radius_y: Double) {
    let rounded_rect = TinRoundedRect::new(rect.clone(), radius_x, radius_y);
    add_draw_call(DrawCall::RoundedRect(rounded_rect))
}

/// TODO: Document this function.
pub fn triangle(x1: Double, y1: Double, x2: Double, y2: Double, x3: Double, y3: Double) {
    let triangle = TinTriangle::new(
        TinPoint::from_coords(x1, y1),
        TinPoint::from_coords(x2, y2),
        TinPoint::from_coords(x3, y3),
    );
    add_draw_call(DrawCall::Triangle(triangle))
}

// Path methods

/// Create a new path.
pub fn path_begin() {
    add_draw_call(DrawCall::PathBegin)
}

/// Add a new point to the current path. (input 2 CGFloats)
pub fn path_vertex(x: Double, y: Double) {
    add_draw_call(DrawCall::PathVertex(TinPoint::from_coords(x, y)))
}

/// Add a bezier curve to the current path
pub fn path_add_curve<P>(to: &P, control1: &P, control2: &P)
where
    P: TPoint,
{
    add_draw_call(DrawCall::PathAddCurve(PathAddCurveCall {
        to: (to.get_x(), to.get_y()),
        control1: (control1.get_x(), control1.get_y()),
        control2: (control2.get_x(), control2.get_y()),
    }))
}

/// Stroke/Fill the current path.
pub fn path_end() {
    add_draw_call(DrawCall::PathEnd)
}

// MARK: - Context state and Transformations

/// TODO: Document this function.
pub fn push_state() {
    add_draw_call(DrawCall::PushState)
}

/// TODO: Document this function.
pub fn pop_state() {
    add_draw_call(DrawCall::PopState)
}

/// TODO: Document this function.
pub fn translate(dx: Double, dy: Double) {
    add_draw_call(DrawCall::Translate(dx, dy))
}

/// TODO: Document this function.
pub fn rotate(by_angle: Double) {
    add_draw_call(DrawCall::Rotate(by_angle))
}

/// TODO: Document this function.
pub fn scale(amount: Double) {
    add_draw_call(DrawCall::Scale(amount))
}

// MARK: - Image
#[cfg(feature = "image")]
use crate::image::TinImage;
/// TODO: Document this function.
#[cfg(feature = "image")]
pub fn image(image: &'static TinImage, x: Double, y: Double) {
    image_with_size(
        image,
        x,
        y,
        image.get_width() as f64,
        image.get_height() as f64,
    );
}

/// TODO: Document this function.
#[cfg(feature = "image")]
pub fn image_with_size(
    image: &'static TinImage,
    x: Double,
    y: Double,
    width: Double,
    height: Double,
) {
    image_with_size_and_resize(image, x, y, width, height, false);
}

/// TODO: Document this function.
#[cfg(feature = "image")]
pub fn image_with_size_and_resize(
    image: &'static TinImage,
    x: Double,
    y: Double,
    width: Double,
    height: Double,
    resize: bool,
) {
    let center = TinPoint::from_coords(x, y);
    add_draw_call(DrawCall::Image(ImageCall {
        image,
        center,
        width,
        height,
        resize,
    }))
}

#[cfg(feature = "text")]
pub fn text(message: &String, font: &crate::text::TinFont, x: Double, y: Double) {
    use crate::text::*;
    let center = TinPoint::from_coords(x, y);
    let f = font.clone();
    add_draw_call(DrawCall::Text(TextCall {
        message: message.clone(),
        font: f,
        center,
    }))
}

pub fn get_frame_count() -> crate::ULong {
    get_tin().get_frame_count()
}

fn add_draw_call(call: DrawCall) {
    crate::context::get_tin_mut().calls.push_back(call)
}

#[derive(Debug)]
pub(crate) struct PathAddCurveCall {
    pub to: (Double, Double),
    pub control1: (Double, Double),
    pub control2: (Double, Double),
}

#[cfg(feature = "image")]
#[derive(Debug)]
pub(crate) struct ImageCall {
    pub image: &'static crate::image::TinImage,
    pub center: TinPoint,
    pub width: Double,
    pub height: Double,
    pub resize: bool,
}

#[cfg(feature = "text")]
#[derive(Debug)]
pub(crate) struct TextCall {
    message: String,
    font: crate::text::TinFont,
    pub center: TinPoint,
}

#[derive(Debug)]
pub(crate) enum DrawCall {
    Background(TinColor),
    Fill(TinColor),
    Stroke(TinColor),

    SetAlpha(Double),

    LineWidth(Double),

    Arc(TinArc),
    Ellipse(TinRect),
    Line(TinPoint, TinPoint),
    Rect(TinRect),
    RoundedRect(TinRoundedRect),
    Triangle(TinTriangle),

    PathBegin,
    PathVertex(TinPoint),
    PathAddCurve(PathAddCurveCall),
    PathEnd,

    FillEnable,
    FillDisable,
    StrokeEnable,
    StrokeDisable,

    PushState,
    PopState,

    Translate(Double, Double),
    Rotate(Double),
    Scale(Double),

    #[cfg(feature = "image")]
    Image(ImageCall),

    #[cfg(feature = "text")]
    Text(TextCall),
}
