use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry,
        topology::{edges::Edges, faces::Faces},
        Shape,
    },
    math::Point,
};

impl Shape for fj::Difference {
    fn bounding_volume(&self) -> AABB {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }

    fn faces(
        &self,
        _tolerance: f64,
        _: &mut geometry::Cache,
        _: &mut DebugInfo,
    ) -> Faces {
        todo!()
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vec<Point<3>> {
        todo!()
    }
}
