use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use palette::Srgb;
use std::path::Path;

fn color_distance(c1: &Srgb<u8>, c2: &Srgb<u8>) -> u32 {
    let dr = c1.red as i32 - c2.red as i32;
    let dg = c1.green as i32 - c2.green as i32;
    let db = c1.blue as i32 - c2.blue as i32;
    (dr * dr + dg * dg + db * db) as u32
}

fn find_closest_palette(image: &DynamicImage) -> Vec<Srgb<u8>> {
    let target_colors = [
        Srgb::new(255, 0, 0),    // Red
        Srgb::new(255, 255, 0),  // Yellow
        Srgb::new(255, 165, 0),  // Orange
        Srgb::new(0, 255, 0),    // Green
        Srgb::new(0, 255, 255),  // Cyan
        Srgb::new(0, 0, 255),    // Blue
        Srgb::new(128, 0, 128),  // Purple
        Srgb::new(165, 42, 42),  // Brown
    ];

    let mut closest_colors = target_colors;
    let mut closest_distances = [u32::MAX; 8];

    for (_, _, pixel) in image.pixels() {
        let color = Srgb::new(pixel[0], pixel[1], pixel[2]);

        for (i, &target_color) in target_colors.iter().enumerate() {
            let distance = color_distance(&color, &target_color);
            if distance < closest_distances[i] {
                closest_distances[i] = distance;
                closest_colors[i] = color;
            }
        }
    }

    closest_colors.to_vec()
}

fn load_image(path: &Path) -> DynamicImage {
    image::open(path).expect("Unable to load image")
}

fn to_hex(color: Srgb<u8>) -> String {
    let (r, g, b) = color.into_components();
    format!("#{:02X}{:02X}{:02X}", r, g, b)
}

fn main() -> Result<()> {
    let image_path = Path::new("./src/assets/keyboard.png");
    let image = load_image(image_path);
    let scheme_palette: Vec<String> = find_closest_palette(&image).iter().map(|p| to_hex(*p)).collect();

    println!("green: {:?}", scheme_palette);

    Ok(())
}
