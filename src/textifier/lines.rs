use image::{io::Reader, Rgba};

const LINES: &'static str = "angles.png";

pub fn get_line_pieces() -> image::ImageBuffer<Rgba<u8>, Vec<u8>> {
    let reader_result = Reader::open(LINES);
    if reader_result.is_err() {
        panic!();
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        return image.to_rgba8();
    }

    panic!();
}
