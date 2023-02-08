mod geometry;
mod obj;
mod projector;
mod render;

use std::env::args;
use std::fs::File;
use std::io::prelude::*;

use geometry::{Plane, Vec3};
use obj::Obj;
use projector::Projector;
use render::{render_projection, ImageDescriptor};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = args().skip(1).collect();
    if args.len() >= 4 {
        let mut content = String::new();
        File::open(&args[0])?.read_to_string(&mut content)?;
        let object = Obj::from_string(&content);
        let projector = Projector {
            points: object.points,
            origin: Vec3::Z,
            plane: Plane::new(Vec3::ZERO, Vec3::Z),
        };
        let desc = ImageDescriptor {
            width: args[2].parse().unwrap_or(100),
            height: args[3].parse().unwrap_or(100),
            path: &args[1],
        };
        render_projection(projector.project(), object.faces, desc)
    } else {
        Ok(())
    }
}
