use crate::geometry::{Face, Vec3};

pub struct Obj {
    points: Vec<Vec3>,
    faces: Vec<Face>,
}

impl Obj {
    pub fn from_string(s: &str) -> Self {
        Self {
            points: s
                .split('\n')
                .filter(|s| s.starts_with("v "))
                .map(|s| s.into())
                .collect(),
            faces: s
                .split('\n')
                .filter(|s| s.starts_with("f "))
                .map(|s| s.into())
                .collect(),
        }
    }
}
