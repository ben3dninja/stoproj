use crate::geometry::{Plane, Ray, Vec3};

pub struct Projector {
    points: Vec<Vec3>,
    origin: Vec3,
    plane: Plane,
}

impl Projector {
    fn project(&self) -> Vec<Vec3> {
        self.points
            .iter()
            .map(|p| {
                self.plane
                    .intersect_ray(Ray::from_points(self.origin, p.clone()))
            })
            .collect()
    }
}
