use super::TVertex::{
    TinVertex, 
    VertexPosition, 
    VertexRGB
};
pub struct TinShape {
    vertices: Vec<TinVertex>,
    shouldDraw: bool
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
            shouldDraw: true
        }
    }

    pub fn shouldDraw(&self) -> bool {
        self.shouldDraw
    }

    pub fn getVertices(&self) -> &Vec<TinVertex> {
        return &self.vertices;
    }
}

impl Default for TinShape {
    fn default() -> Self {
        Self::new(Vec::from(VERTICES))
    }
}


#[derive(Debug, Clone)]
pub struct TinRect {
    pub x: f64,
    pub y: f64,
    width: f64,
    height: f64
}

#[allow(dead_code)]
impl TinRect {
    pub fn new_from_dimensions(x: f64, y: f64, w: f64, h: f64) -> Self {
        Self {
            x: x, y: y, width: w.abs(), height: h.abs()
        }
    }

    pub fn get_width(&self) -> f64 {
        self.width
    }
    pub fn set_width(&mut self, width: f64){
        self.width = width.abs()
    }

    pub fn get_height(&self) -> f64 {
        self.height
    }
    pub fn set_height(&mut self, height: f64) {
        self.height = height.abs()
    }

}

