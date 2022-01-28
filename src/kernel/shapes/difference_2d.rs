use parry3d_f64::bounding_volume::AABB;

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry,
        topology::{
            edges::Edges,
            faces::{Face, Faces},
        },
        Shape,
    },
    math::Point,
};

impl Shape for fj::Difference2d {
    fn bounding_volume(&self) -> AABB {
        // This is a conservative estimate of the bounding box: It's never going
        // to be bigger than the bounding box of the original shape that another
        // is being subtracted from.
        self.a.bounding_volume()
    }

    fn faces(
        &self,
        tolerance: f64,
        cache: &mut geometry::Cache,
        debug_info: &mut DebugInfo,
    ) -> Faces {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut a = self.a.faces(tolerance, cache, debug_info);
        let mut b = self.b.faces(tolerance, cache, debug_info);

        let (a, b) = if a.0.len() == 1 && b.0.len() == 1 {
            // Can't panic. We just checked that length of `a` and `b` is 1.
            (a.0.pop().unwrap(), b.0.pop().unwrap())
        } else {
            // See issue:
            // https://github.com/hannobraun/Fornjot/issues/95
            todo!(
                "The 2-dimensional difference operation only supports one face \
                in each operand."
            );
        };

        let (a, b, surface_a, surface_b) = match (a, b) {
            (
                Face::Face {
                    edges: a,
                    surface: surface_a,
                },
                Face::Face {
                    edges: b,
                    surface: surface_b,
                },
            ) => (a, b, surface_a, surface_b),
            _ => {
                // None of the 2D types still use the triangles representation.
                unreachable!()
            }
        };

        if surface_a != surface_b {
            // Panicking is not great, but as long as we don't have a real error
            // handling mechanism, it will do.
            panic!("Trying to subtract sketches with different surfaces.")
        }
        let surface = surface_a;

        let mut edges = a;
        edges.cycles.extend(b.cycles);

        Faces(vec![Face::Face { edges, surface }])
    }

    fn edges(&self, cache: &mut geometry::Cache) -> Edges {
        // This method assumes that `b` is fully contained within `a`:
        // https://github.com/hannobraun/Fornjot/issues/92

        let mut a = self.a.edges(cache);
        let mut b = self.b.edges(cache);

        let (a, mut b) = if a.cycles.len() == 1 && b.cycles.len() == 1 {
            (a.cycles.pop().unwrap(), b.cycles.pop().unwrap())
        } else {
            // See issue:
            // https://github.com/hannobraun/Fornjot/issues/95
            todo!(
                "The 2-dimensional difference operation only supports one \
                cycle in each operand."
            );
        };

        for edge in &mut b.edges {
            edge.reverse();
        }

        Edges { cycles: vec![a, b] }
    }

    fn vertices(&self) -> Vec<Point<3>> {
        todo!()
    }
}
