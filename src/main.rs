use image;
use rand::Rng;

mod geometry;
mod model;

use geometry::{Vec2f, Vec2i, Vec3f, Vec3i};
use model::Model;

type Image = image::ImageBuffer<image::Rgba<u8>, Vec<u8>>;
type Color = image::Rgba<u8>;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

const BLACK: Color = image::Rgba([0, 0, 0, 255]);
const WHITE: Color = image::Rgba([255, 255, 255, 255]);
const RED: Color = image::Rgba([255, 0, 0, 255]);

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

fn barycentric(pts: &[Vec2f], p: impl Into<Vec2f>) -> Vec3f {
    let p = p.into();
    let u = Vec3f::new(pts[2][0]-pts[0][0], pts[1][0]-pts[0][0], pts[0][0]-p[0])
        .cross(Vec3f::new(pts[2][1]-pts[0][1], pts[1][1]-pts[0][1], pts[0][1]-p[1]));
    if u[2].abs() < 1. {
        Vec3f::new(-1., 1., 1.)
    } else {
        Vec3f::new(1. - (u[0] + u[1]) / u[2], u[1] / u[2], u[0] / u[2])
    }
}

fn triangle(pts: &[Vec2f], img: &mut Image, color: Color) {
    let mut bboxmin = Vec2f::new(WIDTH as f32 - 1., HEIGHT as f32 - 1.);
    let mut bboxmax = Vec2f::new(0., 0.);
    let clamp = Vec2f::new(WIDTH as f32 - 1., HEIGHT as f32 - 1.);
    for i in 0..3 {
        for j in 0..2 {
            bboxmin[j] = 0f32.max(bboxmin[j].min(pts[i][j]));
            bboxmax[j] = clamp[j].min(bboxmax[j].max(pts[i][j]));
        }
    }

    for x in bboxmin[0] as u32..bboxmax[0] as u32 {
        for y in bboxmin[1] as u32..bboxmax[1] as u32 {
            let bc_screen = barycentric(&pts, (x as f32, y as f32));
            if bc_screen[0] >= 0. && bc_screen[1] >= 0. && bc_screen[2] >= 0. {
                img.put_pixel(x, y, color);
            }
        }
    }
}

fn main() {
    let mut img = image::ImageBuffer::from_pixel(WIDTH, HEIGHT, BLACK);
    let (width, height) = (WIDTH as f32, HEIGHT as f32);

    let mut rng = rand::thread_rng();

    let model = Model::new("obj/african_head.obj").unwrap();
    for face in &model.faces {
        let mut sc = vec![Vec2f::default(); 3];

        for i in 0..3 {
            let wc = model.verts[face[i]];
            sc[i] = Vec2f::new(
                ((wc[0] as f32 + 1.) * width / 2.).min(width - 1.),
                ((wc[1] as f32 + 1.) * height / 2.).min(height - 1.),
            );
        }

        let color = image::Rgba([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>(), 255]);
        triangle(&sc, &mut img, color);
    }

    let dyn_img = image::DynamicImage::ImageRgba8(img);
    dyn_img.flipv().save("output.png").unwrap();
}
