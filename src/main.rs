mod geometry;
mod obj;
mod projector;

use std::fs::File;
use std::io::prelude::*;

use geometry::Vec3;
use obj::Obj;
use projector::Projector;

fn main() -> std::io::Result<()> {
    let mut content = String::new();
    File::open("points.obj")?.read_to_string(&mut content)?;
    let object = Obj::from_string(&content);
    Ok(())
}
