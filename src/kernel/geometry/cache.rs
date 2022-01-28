use std::{collections::HashMap, marker::PhantomData};

use crate::math::Point;

/// The geometry cache
///
/// Due to floating point accuracy issues, it is error-prone to refer to
/// geometry.
///
/// Since any object can be referenced by multiple other objects (for example, a
/// vertex can be shared by multiple edges), storing such a reference as
/// geometry (for example, storing a vertex as a point in space) risks computing
/// those same objects in different ways, leading to different results.
///
/// This can result in the same objects being mistaken for different ones, due
/// to slight differences.
///
/// This cache presents a principled approach to preventing this: Each geometric
/// object is computed only once, and is only ever referred to by the id
/// returned from this cache.
///
/// The alternative would be to be really careful, everywhere, and plug holes as
/// they are found.
pub struct Cache {
    // TASK: Un-suppress warning.
    #[allow(unused)]
    points: HashMap<Id<Point<3>>, Point<3>>,
}

impl Cache {
    /// Construct a new instance of the geometry cache
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
        }
    }

    // TASK: Document.
    pub fn insert<T>(&mut self, _value: T) -> Id<T> {
        // TASK: Implement.
        todo!()
    }

    // TASK: Document.
    // TASK: This design doesn't work. The purpose of this method is to be able
    //       to add different representation of the same object. Like, you
    //       already have a vertex in 3-dimensional coordinates, now you're
    //       adding the computed value for the same vertex, but in 2-dimensional
    //       surface coordinates.
    //
    //       But where would the caller get a differently-typed `Id<T>`? What
    //       the caller has, is the previous id, and what they can pass in
    //       addition is some additional argument that identifies what type of
    //       variant this is.
    // TASK: Un-suppress warning.
    #[allow(unused)]
    pub fn update<T>(&mut self, _id: Id<T>, _value: T) {
        // TASK: Implement.
        todo!()
    }
}

/// An id that refers to an object
///
/// Instances of this struct are constructed when an object is added to
/// [`Cache`]. It can afterwards be used to retrieved the geometrical
/// representation of that object from the cache.
///
/// This struct must be the only way that objects are referenced. See the
/// documentation of [`Cache`] for more information.
#[derive(Clone, Copy, Debug)]
pub struct Id<T>(PhantomData<T>);
