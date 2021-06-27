

use crate::{
    calculation::remap, color::TColor, point::TPoint, shapes::TinShape, vector2::TinVector2,
    Double, Float,
};

#[cfg(luminance_derive)]
use luminance_derive::{Semantics, Vertex};

#[cfg(luminance_derive)]
#[derive(Copy, Clone, Debug, Semantics)]
pub enum TinVertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 4]", wrapper = "VertexRGB")]
    Color,
}


#[cfg(not(luminance_derive))]
struct VertexPosition([f32; 2]);

#[cfg(not(luminance_derive))]
impl VertexPosition {
    pub fn new(floats: [f32; 2]) -> Self {
        Self(floats)
    }
}

#[cfg(not(luminance_derive))]
pub(crate) struct VertexRGB([u8; 4]);

impl VertexRGB {
    #[cfg(not(luminance_derive))]
    pub fn new(bytes: [u8; 4]) -> Self {
        Self(bytes)
    }

    pub fn from_u8(bytes: [u8; 4]) -> Self {
        Self::new(bytes)
    }

    pub fn from_doubles(doubles: [Double; 4]) -> Self {
        let mut new_color_array: [u8; 4] = [0, 0, 0, 0];
        for i in 0..4 {
            new_color_array[i] = (doubles[i] / 255.) as u8;
        }
        Self::new(new_color_array)
    }
}

#[cfg(luminance_derive)]
#[derive(Copy, Clone, Debug, Vertex)]
#[vertex(sem = "TinVertexSemantics")]
pub struct TinVertex {
    position: VertexPosition,

    #[vertex(normalized = "true")]
    pub color: VertexRGB,
}

#[cfg(not(luminance_derive))]
pub(crate) struct TinVertex {
    position: VertexPosition,
    pub color: VertexRGB,
}

impl TinVertex {
    pub fn from_point_and_color(point: &impl TPoint, color: &impl TColor) -> Self {
        let r: Double = color.get_red();
        let g: Double = color.get_green();
        let b: Double = color.get_blue();
        let a: Double = color.get_alpha();

        let vals: [Double; 4] = [r, g, b, a];
        let mut new_color: [u8; 4] = [0, 0, 0, 0];
        for i in 0..vals.len() {
            let v = vals[i];
            if v >= 1.0 {
                new_color[i] = 255
            } else if v <= 0.0 {
                new_color[i] = 0
            } else {
                new_color[i] = remap(v, 0.0, 1.0, 0., 255.) as u8;
            }
        }

        Self {
            position: VertexPosition::new([point.get_x() as Float, point.get_y() as Float]),
            color: VertexRGB::new(new_color),
        }
    }

    pub fn from_vector_and_color(point: &TinVector2, color: &impl TColor) -> Self {
        let r: Double;
        let g: Double;
        let b: Double;
        let a: Double;
        r = color.get_red();
        g = color.get_green();
        b = color.get_blue();
        a = color.get_alpha();

        let vals: [Double; 4] = [r, g, b, a];
        let mut new_color: [u8; 4] = [0, 0, 0, 0];
        for i in 0..vals.len() {
            let v = vals[i];
            if v >= 1.0 {
                new_color[i] = 255
            } else if v <= 0.0 {
                new_color[i] = 0
            } else {
                new_color[i] = remap(v, 0.0, 1.0, 0., 255.) as u8;
            }
        }

        Self {
            position: VertexPosition::new([point.x as Float, point.y as Float]),
            color: VertexRGB::new(new_color),
        }
    }
}

pub(crate) fn make_shape_from_vertex_vec(vertices: Vec<TinVertex>) -> TinShape {
    TinShape::new(vertices)
}

pub(crate) fn make_shape_from_vector_vec(points: Vec<TinVector2>, color: &impl TColor) -> TinShape {
    let mut vertices = Vec::new();
    for point in points {
        let vertex = TinVertex::from_vector_and_color(&point, color);
        vertices.push(vertex);
    }
    return make_shape_from_vertex_vec(vertices);
}
