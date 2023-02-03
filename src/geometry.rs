use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul for Vec3 {
    type Output = f32;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self)
    }
}

impl From<Vec<f32>> for Vec3 {
    fn from(v: Vec<f32>) -> Self {
        Vec3::new(v[0], v[1], v[2])
    }
}

impl From<&str> for Vec3 {
    fn from(s: &str) -> Self {
        let s = if s.starts_with('v') {
            s.strip_prefix("v ").unwrap().to_string()
        } else {
            s.to_string()
        };
        s.split(' ')
            .map(|x| x.parse::<f32>().unwrap())
            .collect::<Vec<f32>>()
            .into()
    }
}

pub struct Ray {
    d: Vec3,
    o: Vec3,
}

impl Ray {
    pub fn new(d: Vec3, o: Vec3) -> Self {
        Ray { d, o }
    }

    pub fn from_points(o: Vec3, p: Vec3) -> Self {
        Ray { d: p - o, o }
    }
}

pub struct Plane {
    n: Vec3,
    o: Vec3,
}

impl Plane {
    pub fn intersect_ray(&self, r: Ray) -> Vec3 {
        (self.n * (self.o - r.o) / (r.d * self.n)) * r.d + r.o
    }
}
