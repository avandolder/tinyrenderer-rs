use image;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

const BLACK: image::Rgba<u8> = image::Rgba([0, 0, 0, 255]);
const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const RED: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);

fn draw_line(
    (mut x0, mut y0): (i32, i32),
    (mut x1, mut y1): (i32, i32),
    img: &mut image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    color: image::Rgba<u8>,
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

fn main() {
    let mut img = image::ImageBuffer::from_pixel(WIDTH, HEIGHT, BLACK);
    draw_line((0, 0), (20, 99), &mut img, RED);

    let dyn_img = image::DynamicImage::ImageRgba8(img);
    dyn_img.flipv().save("output.png").unwrap();
}
