use luminance_derive::{Semantics, Vertex};

use super::{Calculation::remap, TColor::TinColor, TPoint::TinPoint, Double, Float};


#[derive(Copy, Clone, Debug, Semantics)]
pub enum TinVertexSemantics {
    #[sem(name = "position", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    #[sem(name = "color", repr = "[u8; 4]", wrapper = "VertexRGB")]
    Color,
}

impl VertexRGB {
    #[allow(dead_code)]
    pub fn new_from_u8(bytes: [u8;4]) -> Self {
        Self::new(bytes)
    }

    #[allow(dead_code)]
    pub fn new_from_f64(doubles: [f64;4]) -> Self {
        let mut newArr: [u8;4] = [0,0,0,0];
        for i in 0..4 {
            newArr[i] = (doubles[i] / 255.) as u8;
        }
        Self::new(newArr)
    }
}

#[derive(Copy, Clone, Debug, Vertex)]
#[vertex(sem = "TinVertexSemantics")]
pub struct TinVertex {
    #[allow(dead_code)]
    position: VertexPosition,

    #[allow(dead_code)]
    #[vertex(normalized = "true")]
    pub color: VertexRGB,
}

impl TinVertex {
    pub fn new_from_pointAndColor(point: &TinPoint, color: &TinColor) -> Self {
        let r: Double; let g: Double; let b: Double; let a: Double;
        r = color.red;
        g = color.green;
        b = color.blue;
        a = color.alpha;

        let vals: [Double; 4] = [r,g,b,a];
        let mut new_color: [u8; 4] = [0,0,0,0];
        for i in 0..vals.len() {
            let v = vals[i];
            if v >= 1.0 {
                new_color[i] = 255
            } else if v <= 0.0 {
                new_color[i] = 0
            } else {
                new_color[i] = remap(v, 0.0, 1.0, 0., 255.) as u8;
            }
        };

        Self{
            position: VertexPosition::new([point.x as Float, point.y as Float]),
            color: VertexRGB::new(new_color)
        }
    }
}

