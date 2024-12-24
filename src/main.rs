use image::ImageReader;
use image_webp::WebPDecoder;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::env::args().nth(1).unwrap();
    let reader = ImageReader::open(input)?.into_inner();
    let mut decoder = WebPDecoder::new(reader)?;
    let image_len = decoder.output_buffer_size().unwrap();
    let mut buffer = vec![0u8; image_len];
    decoder.read_image(&mut buffer)?;
    std::hint::black_box(buffer);
    Ok(())
}
