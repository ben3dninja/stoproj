use draw::*;

use crate::geometry::{Face, Vec3};
use std::io::Result;

pub fn render_projection(points: Vec<Vec3>, faces: Vec<Face>, desc: ImageDescriptor) -> Result<()> {
    let ImageDescriptor {
        width,
        height,
        path,
    } = desc;
    render::save(
        &build_canvas(points, faces, width, height),
        path,
        SvgRenderer::new(),
    )
}

fn build_canvas(points: Vec<Vec3>, faces: Vec<Face>, width: u32, height: u32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let max_distance = points
        .iter()
        .map(|v| if v.x > v.y { v.x } else { v.y })
        .max_by(|a, b| a.partial_cmp(b).unwrap());
    if let Some(max_distance) = max_distance {
        let factor = 0.4 / max_distance;
        for face in faces {
            let origin = points[face.indices[0]];
            let mut builder = LineBuilder::new(
                (origin.x * factor + 0.5) * width as f32,
                (origin.y * factor + 0.5) * height as f32,
            );
            for i in face.indices[1..].iter() {
                builder = builder.line_to(
                    (points[*i].x * factor + 0.5) * width as f32,
                    (points[*i].y * factor + 0.5) * height as f32,
                );
            }
            builder = builder.line_to(
                (origin.x * factor + 0.5) * width as f32,
                (origin.y * factor + 0.5) * height as f32,
            );
            canvas.display_list.add(
                Drawing::new()
                    .with_shape(builder.build())
                    .with_style(Style::stroked(5, Color::black())),
            );
        }
    }
    canvas
}

pub struct ImageDescriptor<'a> {
    pub width: u32,
    pub height: u32,
    pub path: &'a str,
}
