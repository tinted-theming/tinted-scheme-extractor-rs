use std::{collections::HashMap, path::Path};

use crate::{
    color::{Color, PureColor},
    Error, Variant,
};
use image::{DynamicImage, GenericImageView};
use palette::{rgb::Rgb, Hsl, IntoColor, Srgb, Yxy};

const MAX_COLOR_DISTANCE: u32 = 10_000;

pub(crate) fn find_closest_palette(image: &DynamicImage) -> Vec<Color> {
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

pub(crate) fn load_image(path: &Path) -> DynamicImage {
    image::open(path).expect("Unable to load image")
}

pub(crate) fn interpolate_color(start: Srgb<u8>, end: Srgb<u8>, t: f32) -> Srgb<u8> {
    Srgb::new(
        (start.red as f32 + t * (end.red as f32 - start.red as f32)) as u8,
        (start.green as f32 + t * (end.green as f32 - start.green as f32)) as u8,
        (start.blue as f32 + t * (end.blue as f32 - start.blue as f32)) as u8,
    )
}

pub(crate) fn generate_gradient(
    darkest: Srgb<u8>,
    lightest: Srgb<u8>,
    steps: usize,
) -> Vec<String> {
    (0..steps)
        .map(|i| {
            let t = i as f32 / (steps - 1) as f32;
            let rgb = interpolate_color(darkest, lightest, t);

            format!("#{:02X}{:02X}{:02X}", rgb.red, rgb.green, rgb.blue)
        })
        .collect()
}

pub(crate) fn create_palette_with_inverse_colors(
    palette: &[Color],
    inverse_palette: &[Color],
) -> Vec<Color> {
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

pub(crate) fn create_palette_with_color_thief_colors(
    palette: &[Color],
    color_thief_palette: &[Srgb<u8>],
) -> Result<Vec<Color>, Error> {
    let color_thief_palette: Vec<Option<Color>> = color_thief_palette
        .iter()
        .map(|c| {
            let mut matching_colors: Vec<Color> = Vec::new();
            let rgb = Srgb::new(c.red, c.green, c.blue);

            for color in palette {
                let attempted_color = Color::new(color.associated_pure_color, rgb);

                if attempted_color.distance < MAX_COLOR_DISTANCE {
                    matching_colors.push(attempted_color);
                }
            }

            matching_colors.sort_by(|a, b| {
                a.distance
                    .partial_cmp(&b.distance)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

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

fn get_sat_luma(color: Rgb) -> (f32, f32) {
    let yxy: Yxy = color.into_color();
    let (_, _, luma) = yxy.into_components();
    let hsl: Hsl = color.into_color();
    let (_, saturation, _) = hsl.into_components();
    (saturation, luma)
}

pub(crate) fn fix_colors(dark: Rgb, light: Rgb, mode: &Variant) -> (Rgb, Rgb) {
    match mode {
        Variant::Light => {
            let mut fg = dark;
            let mut bg = light;
            // Foreground should be pretty dark and have:
            // luma <= 0.015 && saturation <= 0.65
            let (saturation, luma) = get_sat_luma(fg);
            if luma > 0.015 {
                let yxy: Yxy = fg.into_color();
                let (x, y, _) = yxy.into_components();
                let yxy: Yxy = Yxy::from_components((x, y, 0.015));
                fg = yxy.into_color();
            }
            if saturation > 0.65 {
                let hsl: Hsl = fg.into_color();
                let (h, _, l) = hsl.into_components();
                let hsl: Hsl = Hsl::from_components((h, 0.65, l));
                fg = hsl.into_color();
            }

            // Background should be light have:
            // luma >= 0.7 && saturation <= 0.12
            let (saturation, luma) = get_sat_luma(light);
            if luma < 0.75 {
                let yxy: Yxy = bg.into_color();
                let (x, y, _) = yxy.into_components();
                let yxy: Yxy = Yxy::from_components((x, y, 0.75));
                bg = yxy.into_color();
            }
            if saturation > 0.12 {
                let hsl: Hsl = bg.into_color();
                let (h, _, l) = hsl.into_components();
                let hsl: Hsl = Hsl::from_components((h, 0.15, l));
                bg = hsl.into_color();
            }
            (bg, fg)
        }
        Variant::Dark => {
            let mut fg = light;
            let mut bg = dark;
            // Foreground should be light and have:
            // luma >= 0.6 && saturation <= 0.15
            let (saturation, luma) = get_sat_luma(light);
            if luma < 0.6 {
                let yxy: Yxy = fg.into_color();
                let (x, y, _) = yxy.into_components();
                let yxy: Yxy = Yxy::from_components((x, y, 0.6));
                fg = yxy.into_color();
            }
            if saturation > 0.15 {
                let hsl: Hsl = fg.into_color();
                let (h, _, l) = hsl.into_components();
                let hsl: Hsl = Hsl::from_components((h, 0.15, l));
                fg = hsl.into_color();
            }
            // Background should be dark and have:
            // luma <= 0.02 && saturation <= 0.6
            let (saturation, luma) = get_sat_luma(dark);
            if luma > 0.02 {
                let yxy: Yxy = bg.into_color();
                let (x, y, _) = yxy.into_components();
                let yxy: Yxy = Yxy::from_components((x, y, 0.02));
                bg = yxy.into_color();
            }
            if saturation > 0.6 {
                let hsl: Hsl = bg.into_color();
                let (h, _, l) = hsl.into_components();
                let hsl: Hsl = Hsl::from_components((h, 0.6, l));
                bg = hsl.into_color();
            }
            (bg, fg)
        }
    }
}

fn color_pass(
    colors: &[Rgb],
    min_luma: Option<f32>,
    max_luma: Option<f32>,
    min_saturation: Option<f32>,
    max_saturation: Option<f32>,
) -> Option<Rgb> {
    let predicate = |rgb: &Rgb| {
        let (saturation, luma) = get_sat_luma(*rgb);

        let luma_check = match (min_luma, max_luma) {
            (Some(min), Some(max)) => luma >= min && luma <= max,
            (Some(min), None) => luma >= min,
            (None, Some(max)) => luma <= max,
            (None, None) => true,
        };

        let saturation_check = match (min_saturation, max_saturation) {
            (Some(min), Some(max)) => saturation >= min && saturation <= max,
            (Some(min), None) => saturation >= min,
            (None, Some(max)) => saturation <= max,
            (None, None) => true,
        };

        luma_check && saturation_check
    };

    colors.iter().copied().find(predicate)
}

pub(crate) fn light_color(colors: &[Srgb<f32>], verbose: bool) -> Result<Srgb<f32>, Error> {
    let mut passes = 1;
    // Try to find a nice light color with low saturation
    let mut light = color_pass(colors, Some(0.6), None, None, Some(0.4));

    // Try again, but now we will accept saturated colors, as long as they're very bright
    if light.is_none() {
        passes += 1;
        light = color_pass(colors, Some(0.7), None, None, Some(0.85));
    }

    // Try again, same as first, but a little more permissive
    if light.is_none() {
        passes += 1;
        light = color_pass(colors, Some(0.5), None, None, Some(0.5));
    }

    // Try again, but accept more saturated colors
    if light.is_none() {
        passes += 1;
        light = color_pass(colors, Some(0.6), None, None, Some(0.85));
    }

    // Try again, but now we will accept darker colors, as long as they're not saturated
    if light.is_none() {
        passes += 1;
        light = color_pass(colors, Some(0.32), None, None, Some(0.4));
    }

    // Try again, but now we will accept even more saturated colors
    if light.is_none() {
        passes += 1;
        light = color_pass(colors, Some(0.4), None, None, None);
    }

    // Try again, with darker colors
    if light.is_none() {
        passes += 1;
        light = color_pass(colors, Some(0.3), None, None, None);
    }

    // Ok, we didn't find anything usable. So let's just grab the most dominant color (we'll lighten it later)
    if light.is_none() {
        passes += 1;
        light = colors.first().copied();
    }

    if verbose {
        println!("Passes: {}", passes);
    }

    light.ok_or_else(|| Error::NoColors("Failed to find colors on image".to_string()))
}

pub(crate) fn dark_color(colors: &[Srgb<f32>], verbose: bool) -> Result<Srgb<f32>, Error> {
    let mut passes = 1;
    // Try to find a nice darkish color with at least a bit of color
    let mut dark = color_pass(colors, Some(0.012), Some(0.1), Some(0.18), Some(0.9));

    // Try again, but now we will accept colors with any saturations, as long long as they're dark but not very dark
    if dark.is_none() {
        passes += 1;
        dark = color_pass(colors, Some(0.012), Some(0.1), None, None);
    }

    // Try again, but now we will accept darker colors too
    if dark.is_none() {
        passes += 1;
        dark = color_pass(colors, None, Some(0.1), None, None);
    }

    // Ok, we didn't find anything usable. So let's just grab the most dominant color (we'll darken it later)
    if dark.is_none() {
        passes += 1;
        dark = colors.first().copied()
    }

    if verbose {
        println!("Passes: {}", passes);
    }

    dark.ok_or_else(|| Error::NoColors("Failed to find colors on image".to_string()))
}
