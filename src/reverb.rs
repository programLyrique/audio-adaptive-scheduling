//! Physical reverb using raycasting
//! The goal is to have here an anytime algorithm

#![allow(unused_imports)]
use na::{Identity, Point3, Vector3};
use ncollide::shape::Cuboid;
use ncollide::query::{Ray, RayCast};


pub struct World {
    room: Cuboid<f32>,
}


#[cfg(test)]
mod tests {
    use super::*;
    extern crate nalgebra;

    use self::nalgebra::{Identity, Point3, Vector3};
    use ncollide::shape::Cuboid;
    use ncollide::query::{Ray, RayCast};

    #[test]
    fn test_ncollide() {
        let cuboid     = Cuboid::new(Vector3::new(1.0, 2.0, 1.0));
        let ray_inside = Ray::new(Point3::new(0., 0., 0.), Vector3::y());
        let ray_miss   = Ray::new(Point3::new(2.0, 2.0, 2.0), Vector3::new(1.0, 1.0, 1.0));

        // Solid cast.
        assert!(cuboid.toi_with_ray(&Identity::new(), &ray_inside, true).unwrap()  == 0.0);

        // Non-solid cast.
        assert!(cuboid.toi_with_ray(&Identity::new(), &ray_inside, false).unwrap() == 2.0);

        // The other ray does not intersect this shape so the `solid` flag has no influence.
        assert!(cuboid.toi_with_ray(&Identity::new(), &ray_miss, false).is_none());
        assert!(cuboid.toi_with_ray(&Identity::new(), &ray_miss, true).is_none());
    }
}
