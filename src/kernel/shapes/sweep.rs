use std::f64::consts::PI;

use nalgebra::vector;
use parry3d_f64::{bounding_volume::AABB, math::Isometry};

use crate::{
    debug::DebugInfo,
    kernel::{
        geometry::Surface,
        topology::{
            edges::{self, Edges},
            faces::{Face, Faces},
            vertices::Vertex,
        },
        Shape,
    },
    math::Point,
};

impl Shape for fj::Sweep {
    fn bounding_volume(&self) -> AABB {
        let mut aabb = self.shape.bounding_volume();
        aabb.maxs.z = self.length;
        aabb
    }

    fn faces(&self, tolerance: f64, debug_info: &mut DebugInfo) -> Faces {
        let original_faces = self.shape.faces(tolerance, debug_info);

        let mut bottom_faces = original_faces.clone();
        bottom_faces.transform(&Isometry::rotation(vector![PI, 0., 0.]));

        let mut top_faces = original_faces.clone();
        top_faces.transform(&Isometry::translation(0.0, 0.0, self.length));

        let mut side_faces = Vec::new();
        for cycle in self.shape.edges().cycles {
            for edge in cycle.edges {
                let length = vector![self.length];

                let mut top_edge = edge.clone();
                top_edge.transform(&Isometry::translation(
                    0.0,
                    0.0,
                    self.length,
                ));

                // TASK: If we can sweep lines into planes, then `Plane` becomes
                //       redundant.
                let surface = Surface::sweep_from(edge.curve.clone(), length);

                let edges = match edge.vertices {
                    Some([a, b]) => {
                        let a = edge.curve.point_curve_to_model(a);
                        let b = edge.curve.point_curve_to_model(b);

                        // TASK: These point in the same direction, thus not
                        //       really forming a cycle in the most perfect
                        //       sense. Shouldn't matter, but I don't know if
                        //       triangulation can handle it.
                        let a = Vertex::from(a).sweep(length);
                        let b = Vertex::from(b).sweep(length);

                        Edges::single_cycle([edge, a, top_edge, b])
                    }
                    None => Edges {
                        // TASK: This construct can't be triangulated with the
                        //       current algorithm.
                        cycles: vec![
                            edges::Cycle { edges: vec![edge] },
                            edges::Cycle {
                                edges: vec![top_edge],
                            },
                        ],
                    },
                };

                side_faces.push(Face::Face { edges, surface });
            }
        }

        // This will only work correctly, if the original shape consists of one
        // edge. If there are more, this will create some kind of weird face
        // chimera, a single face to represent all the side faces.
        //
        // It'll be even worse, if the original shape consists of multiple
        // faces.
        let mut segments = Vec::new();
        self.shape.edges().approx_segments(tolerance, &mut segments);

        let mut quads = Vec::new();
        for segment in segments {
            let [v0, v1] = [segment.a, segment.b];
            let [v3, v2] = {
                let segment = segment.transformed(&Isometry::translation(
                    0.0,
                    0.0,
                    self.length,
                ));
                [segment.a, segment.b]
            };

            quads.push([v0, v1, v2, v3]);
        }

        let mut side_face = Vec::new();
        for [v0, v1, v2, v3] in quads {
            side_face.push([v0, v1, v2].into());
            side_face.push([v0, v2, v3].into());
        }

        let mut faces = Vec::new();
        faces.extend(bottom_faces.0);
        faces.extend(top_faces.0);
        faces.push(Face::Triangles(side_face));

        Faces(faces)
    }

    fn edges(&self) -> Edges {
        todo!()
    }

    fn vertices(&self) -> Vec<Point<3>> {
        todo!()
    }
}
