use crate::math::{Point, Vector};

use super::edges::Edge;

// TASK: Document.
pub struct Vertex {
    // TASK: Document.
    pub point: Point<3>,
}

impl Vertex {
    // TASK: This is inconsistent with the draft of the equivalent method for
    //       curve/surface.
    // TASK: Consider moving this to `Curve` and calling it `sweep_from`.
    // TASK: Document.
    pub fn sweep(self, length: Vector<1>) -> Edge {
        // TASK: Implement.
        todo!()
    }
}

impl From<Point<3>> for Vertex {
    fn from(point: Point<3>) -> Self {
        Self { point }
    }
}
