use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use palette::Srgb;
use std::{collections::HashMap, path::Path};

const MAX_COLOR_DISTANCE: u32 = 10_000;

#[derive(Clone, Copy, Debug)]
enum PureColor {
    Red,
    Yellow,
    Orange,
    Green,
    Cyan,
    Blue,
    Purple,
    Brown,
    Magenta,     // Inverse of Green
    Azure,       // Inverse of Orange
    SpringGreen, // Inverse of Purple
    LightCyan,   // Inverse of Brown
}

impl PureColor {
    fn get_rgb(&self) -> Srgb<u8> {
        match self {
            PureColor::Red => Srgb::new(255, 0, 0),
            PureColor::Yellow => Srgb::new(255, 255, 0),
            PureColor::Orange => Srgb::new(255, 165, 0),
            PureColor::Green => Srgb::new(0, 255, 0),
            PureColor::Cyan => Srgb::new(0, 255, 255),
            PureColor::Blue => Srgb::new(0, 0, 255),
            PureColor::Purple => Srgb::new(128, 0, 128),
            PureColor::Magenta => Srgb::new(255, 0, 255),
            PureColor::Brown => Srgb::new(165, 42, 42),
            PureColor::Azure => Srgb::new(0, 90, 255),
            PureColor::SpringGreen => Srgb::new(127, 255, 127),
            PureColor::LightCyan => Srgb::new(90, 213, 213),
        }
    }

    fn as_str(&self) -> &str {
        match self {
            PureColor::Red => "red",
            PureColor::Yellow => "yellow",
            PureColor::Orange => "orange",
            PureColor::Green => "green",
            PureColor::Cyan => "cyan",
            PureColor::Blue => "blue",
            PureColor::Purple => "purple",
            PureColor::Magenta => "magenta",
            PureColor::Brown => "brown",
            PureColor::Azure => "azure",
            PureColor::SpringGreen => "spring_green",
            PureColor::LightCyan => "light_cyan",
        }
    }

    fn get_inverse(&self) -> PureColor {
        match self {
            PureColor::Red => PureColor::Cyan,
            PureColor::Yellow => PureColor::Blue,
            PureColor::Orange => PureColor::Azure,
            PureColor::Green => PureColor::Magenta,
            PureColor::Cyan => PureColor::Red,
            PureColor::Blue => PureColor::Yellow,
            PureColor::Purple => PureColor::SpringGreen,
            PureColor::Magenta => PureColor::Green,
            PureColor::Brown => PureColor::LightCyan,
            PureColor::Azure => PureColor::Orange,
            PureColor::SpringGreen => PureColor::Purple,
            PureColor::LightCyan => PureColor::Brown,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Color {
    associated_pure_color: PureColor,
    value: Srgb<u8>,
    distance: u32,
}

impl Color {
    fn new(pure_color: PureColor, value: Srgb<u8>) -> Self {
        let distance = Color::get_distance(&Color::from(pure_color).value, &value);

        Color {
            associated_pure_color: pure_color,
            value,
            distance,
        }
    }

    fn from(pure_color: PureColor) -> Self {
        Color {
            associated_pure_color: pure_color,
            value: pure_color.get_rgb(),
            distance: 0,
        }
    }

    fn get_inverse(&self) -> Self {
        let rgb_color_inverse = Srgb::new(
            255 - self.value.red,
            255 - self.value.green,
            255 - self.value.blue,
        );
        let pure_color_inverse = self.associated_pure_color.get_inverse();

        Color::new(pure_color_inverse, rgb_color_inverse)
    }

    // Order of c1 and c2 doesn't matter
    fn get_distance(c1: &Srgb<u8>, c2: &Srgb<u8>) -> u32 {
        let dr = c1.red as i32 - c2.red as i32;
        let dg = c1.green as i32 - c2.green as i32;
        let db = c1.blue as i32 - c2.blue as i32;

        (dr * dr + dg * dg + db * db) as u32
    }

    fn to_hex(self) -> String {
        let (r, g, b) = self.value.into_components();

        format!("#{:02X}{:02X}{:02X}", r, g, b)
    }
}

fn find_closest_palette(image: &DynamicImage) -> Vec<Color> {
    let target_colors: Vec<Color> = vec![
        Color::from(PureColor::Red),
        Color::from(PureColor::Yellow),
        Color::from(PureColor::Orange),
        Color::from(PureColor::Green),
        Color::from(PureColor::Cyan),
        Color::from(PureColor::Blue),
        Color::from(PureColor::Purple),
        Color::from(PureColor::Brown),
        Color::from(PureColor::Magenta),
        Color::from(PureColor::Azure),
        Color::from(PureColor::SpringGreen),
        Color::from(PureColor::LightCyan),
    ];

    let mut closest_colors_with_distance = target_colors.clone();
    let mut closest_distances = [u32::MAX; 13];

    for (_, _, pixel) in image.pixels() {
        let color = Srgb::new(pixel[0], pixel[1], pixel[2]);

        for (i, &target_color) in target_colors.iter().enumerate() {
            let distance = Color::get_distance(&color, &target_color.value);
            if distance < closest_distances[i] {
                closest_distances[i] = distance;
                closest_colors_with_distance[i] = Color {
                    associated_pure_color: target_color.associated_pure_color,
                    value: color,
                    distance,
                };
            }
        }
    }

    closest_colors_with_distance.to_vec()
}

fn load_image(path: &Path) -> DynamicImage {
    image::open(path).expect("Unable to load image")
}

fn create_palette_with_inverse_colors(palette: &[Color], inverse_palette: &[Color]) -> Vec<Color> {
    let mut curated_palette: Vec<Color> = Vec::new();

    for color in palette {
        let color_inverse_opt = inverse_palette
            .iter()
            .find(|c| c.associated_pure_color.as_str() == color.associated_pure_color.as_str());

        if let Some(color_inverse) = color_inverse_opt {
            if color.distance > MAX_COLOR_DISTANCE && color.distance < color_inverse.distance {
                curated_palette.push(*color);
            } else {
                curated_palette.push(*color_inverse);
            }
        } else {
            curated_palette.push(*color);
        }
    }

    curated_palette
}

fn create_palette_with_color_thief_colors(
    palette: &[Color],
    image: DynamicImage,
) -> Result<Vec<Color>> {
    let img_pixels = image.to_rgba8().into_raw();
    let color_thief_palette: Vec<Option<Color>> =
        color_thief::get_palette(img_pixels.as_slice(), color_thief::ColorFormat::Rgba, 1, 15)?
            .iter()
            .map(|c| {
                let mut matching_colors: Vec<Color> = Vec::new();

                for color in palette {
                    let rgb = Srgb::new(c.r, c.g, c.b);
                    let attempted_color = Color::new(color.associated_pure_color, rgb);

                    if attempted_color.distance < MAX_COLOR_DISTANCE {
                        matching_colors.push(attempted_color);
                    }
                }

                matching_colors.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());

                if matching_colors.is_empty() {
                    None
                } else {
                    matching_colors.first().copied()
                }
            })
            .collect();
    let mut color_by_pure_color: HashMap<String, Color> = HashMap::new();

    for color in &color_thief_palette
        .into_iter()
        .flatten()
        .collect::<Vec<Color>>()
    {
        color_by_pure_color
            .entry(color.associated_pure_color.as_str().to_string())
            .and_modify(|e| {
                if color.distance < e.distance {
                    *e = *color
                }
            })
            .or_insert(*color);
    }

    let mut palette_with_color_thief_colors: Vec<Color> =
        color_by_pure_color.into_values().collect();

    for color in palette {
        if !palette_with_color_thief_colors
            .iter()
            .any(|c| c.associated_pure_color.as_str() == color.associated_pure_color.as_str())
        {
            palette_with_color_thief_colors.push(*color);
        }
    }

    Ok(palette_with_color_thief_colors.clone())
}

fn main() -> Result<()> {
    let image_path = Path::new("./src/assets/keyboard.png");
    let image = load_image(image_path);
    let scheme_palette: Vec<Color> = find_closest_palette(&image);
    let inverse_palette: Vec<Color> = find_closest_palette(&image)
        .iter()
        .map(|color| color.get_inverse())
        .collect();
    let curated_palette = create_palette_with_inverse_colors(&scheme_palette, &inverse_palette);
    let combined_palette = create_palette_with_color_thief_colors(&curated_palette, image)?;

    println!(
        "Colors: {:?}",
        &combined_palette
            .iter()
            .map(|p| (p.to_hex(), p.associated_pure_color.as_str(), p.distance))
            .collect::<Vec<(String, &str, u32)>>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::RgbaImage;

    #[test]
    fn test_get_distance() {
        let color1 = Srgb::new(255, 0, 0);
        let color2 = Srgb::new(0, 255, 0);

        let distance = Color::get_distance(&color1, &color2);

        assert_eq!(distance, (255 * 255 * 2) as u32);
    }

    #[test]
    fn test_color_new() {
        let pure_color = PureColor::Red;
        let value = Srgb::new(255, 0, 0);

        let color = Color::new(pure_color, value);

        assert_eq!(color.associated_pure_color.get_rgb(), pure_color.get_rgb());
        assert_eq!(color.value, value);
        assert_eq!(color.distance, 0);
    }

    #[test]
    fn test_color_get_inverse() {
        let color = Color::from(PureColor::Red);

        let inverse_color = color.get_inverse();

        assert_eq!(
            inverse_color.associated_pure_color.get_rgb(),
            PureColor::Cyan.get_rgb()
        );
        assert_eq!(inverse_color.value, Srgb::new(0, 255, 255));
    }

    #[test]
    fn test_find_closest_palette() {
        let img = RgbaImage::from_fn(100, 100, |x, _y| {
            if x < 50 {
                image::Rgba([255, 0, 0, 255]) // Red
            } else {
                image::Rgba([0, 255, 0, 255]) // Green
            }
        });
        let dynamic_img = DynamicImage::ImageRgba8(img);

        let closest_palette = find_closest_palette(&dynamic_img);

        assert_eq!(closest_palette.len(), 12);
    }
}
