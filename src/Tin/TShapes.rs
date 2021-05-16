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



pub struct TinRect {
    pub x: f32,
    pub y: f32,
    width: f32,
    height: f32
}

#[allow(dead_code)]
impl TinRect {
    pub fn new_from_dimensions(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            x: x, y: y, width: w, height: h
        }
    }

    pub fn get_width(&self) -> f32 {
        self.width
    }
    pub fn set_width(&mut self, width: f32){
        self.width = width.abs()
    }

    pub fn get_height(&self) -> f32 {
        self.height
    }
    pub fn set_height(&mut self, height: f32) {
        self.height = height.abs()
    }

}

