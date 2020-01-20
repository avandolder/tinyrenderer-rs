use image;

const WHITE: image::Rgba<u8> = image::Rgba([255, 255, 255, 255]);
const RED: image::Rgba<u8> = image::Rgba([255, 0, 0, 255]);

fn main() {
    let mut img = image::ImageBuffer::new(100, 100);
    img.put_pixel(51, 42, RED);

    let dyn_img = image::DynamicImage::ImageRgba8(img);
    dyn_img.flipv().save("output.png").unwrap();
}
