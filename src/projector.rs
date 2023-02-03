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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection() {
        let proj = Projector {
            points: vec![Vec3::new(1., 0., 1.), Vec3::new(-1., 0., 1.)],
            origin: 2. * Vec3::Z,
            plane: Plane::new(Vec3::ZERO, Vec3::Z),
        };
        assert_eq!(proj.project(), vec![2. * Vec3::X, -2. * Vec3::X])
    }
}
