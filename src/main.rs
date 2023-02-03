mod geometry;
mod projector;

use std::fs::File;
use std::io::prelude::*;

use geometry::Vec3;
use projector::Projector;

fn main() -> std::io::Result<()> {
    let mut content = String::new();
    File::open("points.obj")?.read_to_string(&mut content)?;
    let points: Vec<Vec3> = content
        .split('\n')
        .filter(|s| s.starts_with("v "))
        .map(|s| s.into())
        .collect();
    Ok(())
}
