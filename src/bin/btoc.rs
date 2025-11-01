use clap::Parser;
use image::{Rgb, RgbImage};

#[derive(Parser)]
#[command(version, about = "An encoder for UTF-8 messages into images with hex colors", long_about = None)]
struct Args {
    outfile: String,

    #[arg(short = 's', long = "size", help = "The size of each color block in pixels", default_value_t = 1)]
    block_size: u32,

    #[arg(short = 'w', long = "width", help = "The width of the image in blocks", default_value_t = 64)]
    width: u32,

    #[arg(short = 'm', long = "message", help = "A message to use instead of reading stdin")]
    message: Option<String>
}

fn paint_block(image: &mut RgbImage, block_x: u32, block_y: u32, size: u32, val: [u8; 3]) {
    for x in block_x*size..(block_x+1)*size {
        for y in block_y*size..(block_y+1)*size {
            image.put_pixel(x, y, Rgb(val));
        }
    }
}

fn main() {
    let args = Args::parse();
    let message: String = match args.message {
        Some(m) => m,
        _ => {
            let input = std::io::stdin().lines();
            let mut m = String::new();
            for line in input {
                if let Ok(string) = line {
                    m.push_str(&string);
                    m.push('\n');
                } else {
                    eprintln!("Failed to read line.");
                    return;
                }
            }
            m
        }
    };
    let blocks = message.len().div_ceil(3);
    let mut bytes = message.into_bytes();
    bytes.resize(blocks*3, 0);
    // Assume usize can be at least 32 bits.
    if let Ok(lines) = TryInto::<u32>::try_into(blocks.div_ceil(args.width.try_into().unwrap())) {
        let mut image = RgbImage::new(args.block_size * args.width, args.block_size * lines);
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        for i in 0..blocks {
            paint_block(&mut image, x, y, args.block_size, [bytes[3*i], bytes[3*i+1], bytes[3*i+2]]);
            x += 1;
            if x >= args.width {
                x = 0;
                y += 1;
            }
        }
        if let Err(_) = image.save(args.outfile) {
            eprintln!("Failed to write file.");
        }
    } else {
        eprintln!("Image would be too tall! Try shortening the message or increasing the width.");
    }
}
