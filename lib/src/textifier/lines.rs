use image::{Luma, io::Reader};

const LINES: &'static str = "linePieces.png";

pub fn get_line_pieces() -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let reader_result = Reader::open(LINES);
    if reader_result.is_err() {
        panic!("Line pieces image is missing");
    }
    let img_result = reader_result.unwrap().decode();

    if let Ok(image) = img_result {
        let luma = image.to_luma8();
        return luma;
    }

    panic!();
}
