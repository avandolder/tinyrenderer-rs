use image;

mod geometry;
mod model;

use geometry::Vec2i;
use model::Model;

type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
type Color = image::Rgba<u8>;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);
const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const RED: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);

fn draw_line(
    (mut x0, mut y0): (i32, i32),
    (mut x1, mut y1): (i32, i32),
    img: &mut Image,
    color: Color,
) {
    let mut steep = false;
    if (x0 - x1).abs() < (y0 - y1).abs() {
        steep = true;
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let derror2 = dy.abs() * 2;
    let step = (y1 - y0).signum();
    let mut error2 = 0;
    let mut y = y0;
    for x in x0..=x1 {
        if steep {
            img.put_pixel(y as u32, x as u32, color);
        } else {
            img.put_pixel(x as u32, y as u32, color);
        }

        error2 += derror2;
        if error2 > dx {
            y += step;
            error2 -= dx * 2;
        }
    }
}

fn triangle<T>(t0: T, t1: T, t2: T, img: &mut Image, color: Color)
where
    T: Copy + Into<(i32, i32)>,
{
    draw_line(t0.into(), t1.into(), img, RED);
    draw_line(t1.into(), t2.into(), img, RED);
    draw_line(t0.into(), t2.into(), img, RED);

    let mut vertices = vec![t0.into(), t1.into(), t2.into()];
    vertices.sort_by_key(|t| t.1);

    let y0 = vertices[0].1;
    let y1 = vertices[1].1;
    let y2 = vertices[2].1;
    let x0 = vertices[0].0;
    let x1 = vertices[1].0;
    let x2 = vertices[2].0;

    let m1 = (x1 - x0) as f32 / (y1 - y0) as f32;
    let mut currx1 = x0 as f32;
    let m2 = (x2 - x0) as f32 / (y2 - y0) as f32;
    let mut currx2 = x0 as f32;

    for y in y0..y1 {
        currx1 += m1;
        currx2 += m2;
        draw_line((currx1.ceil() as i32, y), (currx2.ceil() as i32, y), img, color);
    }

    let m1 = (x2 - x1) as f32 / (y2 - y1) as f32;
    for y in y1..y2 {
        currx1 += m1;
        currx2 += m2;
        draw_line((currx1.ceil() as i32, y), (currx2.ceil() as i32, y), img, color);
    }
}

fn main() {
    let mut img = image::ImageBuffer::from_pixel(WIDTH, HEIGHT, BLACK);
    let (width, height) = (WIDTH as f32, HEIGHT as f32);

    let ts = [
        [(10, 70), (50, 160), (70, 80)],
        [(180, 50), (150, 1), (70, 180)],
        [(180, 150), (120, 160), (130, 180)],
    ];
    for t in &ts {
        triangle(t[0], t[1], t[2], &mut img, WHITE);
    }

    /*let model = Model::new("obj/african_head.obj").unwrap();
    for face in &model.faces {
        for i in 0..3 {
            let v0 = model.verts[face[i]];
            let v1 = model.verts[face[(i + 1) % 3]];
            let x0 = ((v0[0] + 1.) * width / 2.).min(width - 1.) as i32;
            let y0 = ((v0[1] + 1.) * height / 2.).min(height - 1.) as i32;
            let x1 = ((v1[0] + 1.) * width / 2.).min(width - 1.) as i32;
            let y1 = ((v1[1] + 1.) * height / 2.).min(height - 1.) as i32;
            draw_line((x0, y0), (x1, y1), &mut img, WHITE);
        }
    }*/

    let dyn_img = image::DynamicImage::ImageRgba8(img);
    dyn_img.flipv().save("output.png").unwrap();
}
