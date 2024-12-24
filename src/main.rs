use clap::Parser;
use image::ImageReader;
use std::error::Error;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Image file
    input: String,

    /// Use reference implementation
    #[arg(short, long, default_value_t = false)]
    use_reference: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    let reader = ImageReader::open(args.input)?.into_inner();
    if args.use_reference {
        let mut decoder = image_webp_reference::WebPDecoder::new(reader)?;
        let image_len = decoder.output_buffer_size().unwrap();
        let mut buffer = vec![0u8; image_len];
        decoder.read_image(&mut buffer)?;
        std::hint::black_box(buffer);
    } else {
        let mut decoder = image_webp_custom::WebPDecoder::new(reader)?;
        let image_len = decoder.output_buffer_size().unwrap();
        let mut buffer = vec![0u8; image_len];
        decoder.read_image(&mut buffer)?;
        std::hint::black_box(buffer);
    };
    Ok(())
}
