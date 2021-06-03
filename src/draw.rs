// MARK: - Global drawing methods

use crate::{
    backends::{ArcRenderer}, color::TColor, font::TinFont,
    Double,
    image::TinImage,
    point::TinPoint,
    shapes::TinRect,
    backends::*,
    context::{get_tin, get_tin_mut}
};


/// Clear (erase) the background
pub fn background(red: &Double, green: &Double, blue: &Double) {
    get_tin_mut().set_background_color(&red, &green, &blue, &1.0);
}


/// TODO: Document this function.
pub fn background_gray(gray: Double) {
    background(&gray, &gray, &gray)
}


/// TODO: Document this function.
pub fn background_with_tin_color(color: &impl TColor) {
    get_tin_mut().set_background_color_with_color(color);
}


/// TODO: Document this function.
pub fn arc(x: &Double, y: &Double, radius: &Double, start_angle: &Double, end_angle: &Double) {
    let mut tin = get_tin_mut();
    let brush = tin.get_brush();
    let state = tin.get_state();
    tin.render.arc( x, y, radius, start_angle, end_angle, brush, state);
}


// Ellipse method

/// Draw an ellipse. Input is centerX, centerY coordinate and width, height size.
pub fn ellipse(center_x: &Double, center_y: &Double, width: &Double, height: &Double) {
    let x = center_x - width / 2.0;
    let y = center_y - height / 2.0;
    let r = TinRect::new_from_dimensions( x, y, *width, *height);
    let mut tin = get_tin_mut();
    let brush = tin.get_brush();
    let state = tin.get_state();
    tin.render.ellipse_in_tinrect( &r, brush, state);
}


// Line methods

/// TODO: Document this function.
pub fn line(x1: &Double, y1: &Double, x2: &Double, y2: &Double) {
    let mut tin = get_tin_mut();
    let width = tin.line_width;
    let brush = tin.get_brush();
    
    let state = tin.get_state();
    tin.render.line( x1, y1, x2, y2, &width, brush, state);
}


/// TODO: Document this function.
pub fn line_width(width: &Double) {
    let mut tin = get_tin_mut();
    tin.line_width = width.clone();
}


// Rectangle method

/// Draw a rectangle. Input is left, bottom coordinate and width, height size.
pub fn rect(x: &Double, y: &Double, width: &Double, height: &Double) {
    eprintln!("draw::rect()");
    let r = TinRect::new_from_dimensions( *x, *y, *width, *height);
    let mut tin = get_tin_mut();
    let brush = tin.get_brush();
    let state = tin.get_state();
    tin.render.rect_with_tinrect(&r, brush, state);
}


/// Draw a rectangle with rounded corners, specified by radius_x, radius_y
pub fn rounded_rect(rect: &TinRect, radius_x: &Double, radius_y: &Double) {
    let mut tin = get_tin_mut();
    let brush = tin.get_brush();
    let state = tin.get_state();
    tin.render.rounded_rect(rect, radius_x, radius_y, brush, state);
}



/// TODO: Document this function.
pub fn triangle(x1: Double, y1: Double, x2: Double, y2: Double, x3: Double, y3: Double) {
    eprintln!("draw::triangle()");
    let mut tin = get_tin_mut();
    let brush = tin.get_brush();
    let state = tin.get_state();
    tin.render.triangle(&x1, &y1, &x2, &y2, &x3, &y3, brush, state);
}


// Path methods


/// Create a new path.
pub fn path_begin() {
    let mut tin = get_tin_mut();
    tin.render.path_begin();
    tin.path_vertex_count = 0;
}


/// Add a new point to the current path. (input 2 CGFloats)
pub fn path_vertex(x: &Double, y: &Double) {
    let point = TinPoint {x: *x, y: *y};
    let mut tin = get_tin_mut();
    tin.render.path_vertex( &point);
    tin.path_vertex_count += 1;
}


/// Add a bezier curve to the current path
pub fn path_add_curve(to: &TinPoint, control1: &TinPoint, control2: &TinPoint) {
    let mut tin = get_tin_mut();
    tin.render.path_add_curve( to, control1, control2);
    tin.path_vertex_count += 4;
}


/// Stroke/Fill the current path.
pub fn path_end() {
    let mut tin = get_tin_mut();
    tin.render.path_end();
    tin.path_vertex_count = 0;
}


// MARK: - Color state

/// TODO: Document this function.
pub fn stroke_color_from_rgba(red: &Double, green: &Double, blue: &Double, alpha: &Double) {
    let mut tin = get_tin_mut();
    tin.stroke = true;
    tin.set_stroke_color(red, green, blue, alpha);
}


/// TODO: Document this function.
pub fn stroke_color_from_gray_and_alpha(gray: &Double, alpha: &Double) {
    stroke_color_from_rgba(gray, gray, gray, alpha);
}


/// TODO: Document this function.
pub fn stroke_color_from_gray(gray: &Double) {
    stroke_color_from_rgba(gray, gray, gray, &1.0);
}


/// TODO: Document this function.
pub fn stroke_color_from_color(color: & impl TColor) {
    stroke_color_from_rgba( &color.get_red(), &color.get_green(), &color.get_blue(), &color.get_alpha());
}


/// TODO: Document this function.
pub fn fill_color_from_rgba(red: &Double, green: &Double, blue: &Double, alpha: &Double) {
    let mut tin = get_tin_mut();
    tin.fill = true;
    tin.set_fill_color( red, green, blue, alpha);
}


/// TODO: Document this function.
pub fn fill_color_from_gray_and_alpha(gray: &Double, alpha: &Double) {
    fill_color_from_rgba(gray, gray, gray, alpha)
}


/// TODO: Document this function.
pub fn fill_color_from_gray(gray: &Double) {
    let mut tin = get_tin_mut();
    tin.fill = true;
    tin.set_fill_color(gray, gray, gray, &1.0);
}


/// TODO: Document this function.
pub fn fill_color_from_color(color: &impl TColor) {
    let mut tin = get_tin_mut();
    tin.fill = true;
    tin.set_fill_color( &color.get_red(), &color.get_green(), &color.get_blue(), &color.get_alpha());
}


/// TODO: Document this function.
pub fn get_stroke_color() -> impl TColor {
    let tin = get_tin_mut();
    return tin.get_stroke_color();
}


/// TODO: Document this function.
pub fn get_fill_color() -> impl TColor {
    let tin = get_tin();
    return tin.get_fill_color();
}


/// TODO: Document this function.
pub fn set_alpha(alpha: &Double) {
    let mut tin = get_tin_mut();
    tin.set_alpha(alpha);
}


/// TODO: Document this function.
pub fn stroke_disable() {
    get_tin_mut().stroke = false;
}


/// TODO: Document this function.
pub fn fill_disable() {
    get_tin_mut().fill = false;
}


/// TODO: Document this function.
pub fn stroke_enable() {
    get_tin_mut().stroke = true;
}


/// TODO: Document this function.
pub fn fill_enable() {
    get_tin_mut().fill = true;
}


// MARK: - Context state and Transformations


/// TODO: Document this function.
pub fn push_state() {
    get_tin_mut().render.push_state();
}


/// TODO: Document this function.
pub fn pop_state() {
    get_tin_mut().render.pop_state();
}


/// TODO: Document this function.
pub fn translate(dx: &Double, dy: &Double) {
    let mut tin = get_tin_mut();
    tin.translation.0 += dx;
    tin.translation.1 += dy;
}


/// TODO: Document this function.
pub fn rotate(by_angle: &Double) {
    get_tin_mut().rotation += by_angle;
}


/// TODO: Document this function.
pub fn scale(amount: &Double) {
    get_tin_mut().scale += amount;
}


// MARK: - Image


/// TODO: Document this function.
pub fn image(image: &TinImage, x: &Double, y: &Double) {
    get_tin_mut().render.image(image, x, y);
}


/// TODO: Document this function.
pub fn image_with_size(image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double) {
    get_tin_mut().render.image_with_size(image, x, y, width, height);
}


/// TODO: Document this function.
pub fn image_with_size_and_resize(image: &TinImage, x: &Double, y: &Double, width: &Double, height: &Double, resize: bool) {
    get_tin_mut().render.image_with_size_and_resize(image, x, y, width, height, resize);
    // Draw image at point
}


// MARK: - Text

pub fn text(message: &String, font: &TinFont, x: &Double, y: &Double) {
    get_tin_mut().render.text(message, font, x, y);
}

