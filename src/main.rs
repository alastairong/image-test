use image::{GenericImageView, ImageBuffer, Rgba};
use rand::Rng;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::fs;
use std::path::Path;

pub struct PixelStruct {
    x: u32,
    y: u32,
    pixel: Rgba<u8>
}

const TEMPLATE_OFFSET: (u32, u32) = (0, 0);
const PLACEMENT_COUNT_OFFSET: (u32, u32) = (80, 135);
const AUTHOR_OFFSET: (u32, u32) = (45, 148);
const FONT_DATA: &[u8; 168260] = include_bytes!("./Roboto-Regular.ttf");
const TEMPLATE_DATA: &[u8] = include_bytes!("../template.png");

fn main() {
    let img = image::load_from_memory(TEMPLATE_DATA).unwrap();
    let (w, h) = img.dimensions();
    let mut output = ImageBuffer::new(w, h); // create a new buffer for our output
    // copy the template image into the output buffer
    for (x, y, pixel) in img.pixels() {
        output.put_pixel(x, y, pixel); 
    }
    // apply the snapshot image to the template output buffer
    let snapshot = random_snapshot_image_data();
    let pixel_array = convert_snapshot_to_pixel_array(snapshot);
    
    for pixel in pixel_array.iter() {
        output.put_pixel(pixel.x + TEMPLATE_OFFSET.0, pixel.y + TEMPLATE_OFFSET.1, pixel.pixel);
    }
    let font = Font::try_from_bytes(FONT_DATA as &[u8]).unwrap();
    output = write_placement_count(11, &font, output);
    output = write_author_name("Alastair".to_string(), &font, output);

    output.save("test.png").unwrap();

    let image = output.into_raw();
    let path: &Path = Path::new("./file");

    let img_base64 = base64::encode(&image);
    println!("{}", img_base64);
    
    fs::write(path, image).unwrap();

    
}


fn random_snapshot_image_data() -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let arr: Vec<u8> = (0..(128 * 64)).map(|_| rng.gen()).collect();
    arr
}

fn convert_snapshot_to_pixel_array(snapshot: Vec<u8>) -> Vec<PixelStruct> {
    
    // first unpack the doublepixels into normal uint4s representing the 16 colors
    let mut unpacked_array: Vec<u8> = Vec::new();
    for doublepixel in snapshot.iter() {
        unpacked_array.push(doublepixel % 16);
        unpacked_array.push(doublepixel / 16);
    }

    // then convert the uint4s into PixelStructs 
    let mut pixel_array: Vec<PixelStruct> = Vec::new();
    for i in 0..unpacked_array.len() {
        let color = COLOR_PALETTE[unpacked_array[i] as usize];
        pixel_array.push(PixelStruct {
            x: i as u32 % 128,
            y: i as u32 / 128,
            pixel: hex_to_rgba(color)
        })
    }
    pixel_array
}

fn hex_to_rgba(hex: &str) -> Rgba<u8> {
    let r = u8::from_str_radix(&hex[1..3], 16).unwrap();
    let g = u8::from_str_radix(&hex[3..5], 16).unwrap();
    let b = u8::from_str_radix(&hex[5..7], 16).unwrap();
    let a = if hex.len() > 7 {
        u8::from_str_radix(&hex[7..9], 16).unwrap()
    } else {
        255
    };
    Rgba([r, g, b, a])
}

fn write_placement_count(count: u32, font: &Font, mut image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Set the scale (font size)
    let scale = Scale { x: 11.0, y: 11.0 };

    // Set the position where the text will be drawn
    let x = PLACEMENT_COUNT_OFFSET.0 as i32;
    let y = PLACEMENT_COUNT_OFFSET.1 as i32;

    // Set the text color
    let color = Rgba([0u8, 0u8, 0u8, 255]);

    // Draw the text onto the ImageBuffer
    let count_text = count.to_string();
    draw_text_mut(&mut image, color, x, y, scale, font, &count_text);
    image
}

fn write_author_name(name: String, font: &Font, mut image: ImageBuffer<Rgba<u8>, Vec<u8>>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    // Set the scale (font size)
    let scale = Scale { x: 12.0, y: 12.0 };

    // Set the position where the text will be drawn
    let x = AUTHOR_OFFSET.0 as i32;
    let y = AUTHOR_OFFSET.1 as i32;

    // Set the text color
    let color = Rgba([0u8, 0u8, 0u8, 255]);

    // Draw the text onto the ImageBuffer
    draw_text_mut(&mut image, color, x, y, scale, font, &name);
    image
}

const COLOR_PALETTE: [&str; 16] = [
    "#FFFFFF",
    "#E4E4E4",
    "#888888",
    "#222222",
    "#FDA1D3",
    "#F82200",
    "#F09200",
    "#A86839",
    "#E6DA00",
    "#7BE400",
    "#0FC300",
    "#34D7E0",
    "#2B84CD",
    "#3200F4",
    "#DE64EA",
    "#8E0A85",
];