use super::vertex::{
    TinVertex, 
    VertexPosition, 
    VertexRGB
};
pub struct TinShape {
    vertices: Vec<TinVertex>,
    should_draw: bool
}


const VERTICES: [TinVertex; 3] = [
    TinVertex::new(
        VertexPosition::new([-0.5, -0.5]),
        VertexRGB::new([255, 0, 0, 255]),
    ),
    TinVertex::new(
        VertexPosition::new([0.5, -0.5]),
        VertexRGB::new([0, 255, 0, 255]),
    ),
    TinVertex::new(
        VertexPosition::new([0., 0.5]),
        VertexRGB::new([0, 0, 255, 255])
    ),
];

impl TinShape {
    pub fn new(vertices:Vec<TinVertex>) -> Self {
        Self {
            vertices: vertices,
            should_draw: true
        }
    }

    pub fn should_draw(&self) -> bool {
        self.should_draw
    }

    pub fn get_vertices(&self) -> &Vec<TinVertex> {
        return &self.vertices;
    }
}

impl Default for TinShape {
    fn default() -> Self {
        Self::new(Vec::from(VERTICES))
    }
}


use crate::Double;
use crate::color::TColor;
use crate::vector2::TVector2;

pub(crate) fn make_shape_from_vertex_vec(vertices: Vec<TinVertex>) -> TinShape {
    TinShape::new(vertices)
}

pub(crate) fn make_shape_from_vector_vec(points: Vec<TVector2>, color: &impl TColor) -> TinShape {
    let mut vertices = Vec::new();
    for point in points {
        let vertex = TinVertex::new_from_vector_and_color(&point, color);
        vertices.push(vertex);
    }
    return make_shape_from_vertex_vec(vertices);
}

#[derive(Debug, Clone)]
pub struct TinRect {
    pub x: Double,
    pub y: Double,
    width: Double,
    height: Double
}


impl TinRect {
    pub fn new_from_dimensions(x: Double, y: Double, w: Double, h: Double) -> Self {
        Self {
            x, y, width: w.abs(), height: h.abs()
        }
    }

    pub fn get_width(&self) -> Double {
        self.width
    }
    pub fn set_width(&mut self, width: Double){
        self.width = width.abs()
    }

    pub fn get_height(&self) -> Double {
        self.height
    }
    pub fn set_height(&mut self, height: Double) {
        self.height = height.abs()
    }

}

