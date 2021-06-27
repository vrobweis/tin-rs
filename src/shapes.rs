mod arc;
pub(crate) use arc::*;

mod ellipse;
pub(crate) use ellipse::*;

mod line;
pub(crate) use line::*;

mod path;
pub(crate) use path::*;

mod rect;
pub use rect::*;

mod triangle;
pub(crate) use triangle::*;

use crate::point::TinPoint;

use super::vertex::TinVertex;
pub(crate) struct TinShape {
    vertices: Vec<TinVertex>,
}

pub enum TShape {
    Triangle([TinPoint; 3]),
}

impl TinShape {
    pub fn new(vertices: Vec<TinVertex>) -> Self {
        Self { vertices }
    }

    pub fn get_vertices(&self) -> &Vec<TinVertex> {
        return &self.vertices;
    }
}

impl From<TinShape> for Vec<TinVertex> {
    fn from(s: TinShape) -> Self {
        s.vertices
    }
}
