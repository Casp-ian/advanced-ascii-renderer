use image::{Luma, io::Reader};

const LINES: &'static str = "linePieces.png";

pub fn get_line_pieces() -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let reader_result = Reader::open(LINES);
    if reader_result.is_err() {
        panic!();
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        let luma = image.to_luma8();
        return luma;
    }

    panic!("linepieces image is gone");
}
