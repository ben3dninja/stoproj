use crate::geometry::{Face, Vec3};

#[derive(Debug, PartialEq)]
pub struct Obj {
    pub points: Vec<Vec3>,
    pub faces: Vec<Face>,
}

impl Obj {
    pub fn from_string(s: &str) -> Self {
        Self {
            points: s
                .split('\n')
                .filter(|s| s.starts_with("v "))
                .map(|s| s.trim().into())
                .collect(),
            faces: s
                .split('\n')
                .filter(|s| s.starts_with("f "))
                .map(|s| s.trim().into())
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_string() {
        assert_eq!(
            Obj::from_string(
                "v 1.0 0.0 0.0\n\
            v 0.0 1.0 0.0\n\
            v 0.0 0.0 1.0\n\
            f 0 1 2"
            ),
            Obj {
                points: vec![Vec3::X, Vec3::Y, Vec3::Z],
                faces: vec![Face {
                    indices: vec![0, 1, 2]
                }],
            }
        );
    }
}
