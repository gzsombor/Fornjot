use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry,
        topology::{edges::Edges, faces::Faces},
        Shape,
    },
    math::{Point, Vector},
};

impl Shape for fj::Transform {
    fn bounding_volume(&self) -> AABB {
        self.shape.bounding_volume().transform_by(&isometry(self))
    }

    fn faces(
        &self,
        tolerance: f64,
        cache: &mut geometry::Cache,
        debug_info: &mut DebugInfo,
    ) -> Faces {
        let mut faces = self.shape.faces(tolerance, cache, debug_info);
        let isometry = isometry(self);

        for face in &mut faces.0 {
            face.transform(&isometry);
        }

        faces
    }

    fn edges(&self, _: &mut geometry::Cache) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vec<Point<3>> {
        todo!()
    }
}

fn isometry(transform: &fj::Transform) -> Isometry<f64> {
    let axis = Vector::from(transform.axis).normalize();
    Isometry::new(Vector::from(transform.offset), axis * transform.angle)
}
