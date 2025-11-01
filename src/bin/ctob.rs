use clap::Parser;
use bytes::Bytes;
use std::io::Cursor;
use image::{ImageReader, DynamicImage};

#[derive(Parser)]
#[command(version, about = "A decoder for UTF-8 messages encoded into images with hex colors", long_about = None)]
struct Args {
    file: String,

    #[arg(short = 'w', long = "web", help = "Whether the file should be downloaded from a URL")]
    is_web: bool,

    #[arg(short = 's', long = "size", help = "The size of each color block in pixels", default_value_t = 1)]
    block_size: u32
}

async fn request(url: String) -> Result<Bytes, reqwest::Error> {
    reqwest::get(url).await?.bytes().await
}

fn decode(buf: Bytes) -> Result<DynamicImage, image::error::ImageError> {
    ImageReader::new(Cursor::new(buf)).with_guessed_format()?.decode()
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let args = Args::parse();
    let buffer: Bytes = match args.is_web {
        true => {
            // Perform web request for image file
            match request(args.file).await {
                Ok(bytes) => bytes,
                _ => {
                    eprintln!("Error in web request.");
                    Bytes::from(vec![])
                }
            } 
        },
        false => {
            match std::fs::read(args.file) {
                Ok(bytes) => Bytes::from(bytes),
                _ => {
                    eprintln!("Error reading file.");
                    Bytes::from(vec![])
                }
            }
        },
    };
    if let Ok(img) = decode(buffer) {
        let rgba = img.into_rgba8();
        let mut bytes: Vec<u8> = vec![];
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        loop {
            let pix = rgba.get_pixel(x, y);
            if pix[0] != 0 { bytes.push(pix[0]); }
            if pix[1] != 0 { bytes.push(pix[1]); }
            if pix[2] != 0 { bytes.push(pix[2]); }
            x += args.block_size;
            if x >= rgba.width() {
                x = 0;
                y += args.block_size;
                if y >= rgba.height() {
                    break;
                }
            }
        }
        if let Ok(string) = String::from_utf8(bytes) {
            println!("{}", string);
        } else {
            eprintln!("Could not decode UTF-8 from image.");
        }
    } else {
        eprintln!("Error decoding image.");
    }
}
