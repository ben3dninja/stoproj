mod vec3;
pub use vec3::Vec3;

pub struct Ray {
    o: Vec3,
    d: Vec3,
}

impl Ray {
    pub fn from_points(o: Vec3, p: Vec3) -> Self {
        Ray { d: p - o, o }
    }
}

pub struct Plane {
    o: Vec3,
    n: Vec3,
}

impl Plane {
    pub fn new(o: Vec3, n: Vec3) -> Self {
        Plane { o, n }
    }
    pub fn intersect_ray(&self, r: Ray) -> Vec3 {
        (self.n * (self.o - r.o) / (r.d * self.n)) * r.d + r.o
    }
}

#[derive(Debug, PartialEq)]
pub struct Face {
    pub indices: Vec<u32>,
}

impl From<&str> for Face {
    fn from(s: &str) -> Self {
        let s = if s.starts_with('f') {
            s.strip_prefix("f ").unwrap().to_string()
        } else {
            s.to_string()
        };
        Face::from_indices(
            s.split(' ')
                .map(|x| x.split('/'))
                .map(|mut x| x.next().unwrap().parse::<u32>().unwrap())
                .collect::<Vec<u32>>(),
        )
    }
}

impl Face {
    fn from_indices(indices: Vec<u32>) -> Self {
        Face { indices }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection() {
        let plane = Plane {
            n: Vec3::Z,
            o: Vec3::ZERO,
        };
        let ray = Ray::from_points(Vec3::Z, Vec3::new(0.5, 0., 0.5));
        assert!((plane.intersect_ray(ray) - Vec3::X).norm() <= 0.01);
    }

    #[test]
    fn face_from_string() {
        assert_eq!(Face::from("f 1 4//2 3//1 2").indices, vec![1, 4, 3, 2]);
    }
}
