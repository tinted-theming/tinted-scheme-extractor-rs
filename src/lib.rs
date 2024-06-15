mod color;
mod utils;

use palette::{rgb::Rgb, Srgb};
use std::{collections::HashMap, fmt, path::PathBuf};

use crate::{
    color::Color,
    utils::{
        create_palette_with_color_thief_colors, create_palette_with_inverse_colors, dark_color,
        find_closest_palette, fix_colors, generate_gradient, light_color, load_image,
    },
};

#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("no colors")]
    NoColors(String),
    #[error("generate colors")]
    GenerateColors(String),
    #[error("other")]
    Other(String),
}

#[derive(Debug)]
pub enum Variant {
    Dark,
    Light,
}

#[derive(Debug)]
pub enum System {
    Base16,
}

#[derive(Debug)]
pub struct Scheme {
    pub author: String,
    pub description: Option<String>,
    pub name: String,
    pub slug: String,
    pub system: System,
    pub variant: Variant,
    pub palette: HashMap<String, String>,
}

impl fmt::Display for Scheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "author: {}", self.author)?;
        if let Some(ref desc) = self.description {
            writeln!(f, "description: {}", desc)?;
        }
        writeln!(f, "name: {}", self.name)?;
        writeln!(f, "slug: {}", self.slug)?;
        writeln!(f, "system: {}", self.system)?;
        writeln!(f, "variant: {}", self.variant)?;
        writeln!(f, "palette:")?;

        let mut palette_vec: Vec<(String, String)> = self
            .palette
            .clone()
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
        palette_vec.sort_by_key(|k| k.0.clone());

        for (key, value) in palette_vec {
            writeln!(f, "  {}: {}", key, value)?;
        }
        Ok(())
    }
}

impl fmt::Display for System {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            System::Base16 => write!(f, "base16"),
        }
    }
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Variant::Light => write!(f, "light"),
            Variant::Dark => write!(f, "dark"),
        }
    }
}

#[derive(Debug)]
pub struct SchemeParams {
    pub image_path: PathBuf,
    pub author: String,
    pub description: Option<String>,
    pub name: String,
    pub slug: String,
    pub system: System,
    pub variant: Variant,
    pub verbose: bool,
}

pub fn create_scheme_from_image(params: SchemeParams) -> Result<Scheme, Error> {
    let SchemeParams {
        image_path,
        author,
        description,
        name,
        slug,
        system,
        variant,
        verbose,
    } = params;
    let image = load_image(&image_path);
    let initial_palette: Vec<Color> = find_closest_palette(&image);
    let inital_inverse_palette: Vec<Color> = find_closest_palette(&image)
        .iter()
        .map(|color| color.get_inverse())
        .collect();
    let curated_palette =
        create_palette_with_inverse_colors(&initial_palette, &inital_inverse_palette);
    let color_thief_palette: Vec<Srgb<u8>> = color_thief::get_palette(
        image.to_rgba8().into_raw().as_slice(),
        color_thief::ColorFormat::Rgba,
        1,
        15,
    )
    .map_err(|err| Error::GenerateColors(err.to_string()))?
    .iter()
    .map(|c| Srgb::new(c.r, c.g, c.b))
    .collect();
    let combined_palette =
        create_palette_with_color_thief_colors(&curated_palette, &color_thief_palette)?;
    let color_thief_pallette_as_rgb_vec: Vec<Rgb> = color_thief_palette
        .clone()
        .iter()
        .map(|c| {
            Rgb::new(
                c.red as f32 / 255.0,
                c.green as f32 / 255.0,
                c.blue as f32 / 255.0,
            )
        })
        .collect();
    let light = light_color(&color_thief_pallette_as_rgb_vec, verbose)?;
    let dark = dark_color(&color_thief_pallette_as_rgb_vec, verbose)?;
    let (background, foreground) = fix_colors(dark, light, &variant);
    let gradient = generate_gradient(Srgb::from(background), Srgb::from(foreground), 8);

    let mut scheme_palette: HashMap<String, String> = HashMap::new();

    for (index, color_text) in gradient.iter().enumerate() {
        scheme_palette
            .entry(format!("base0{}", index))
            .or_insert(color_text.to_string());
    }

    for color in &combined_palette {
        match color.associated_pure_color.as_str() {
            "red" => {
                scheme_palette
                    .entry("base08".to_string())
                    .or_insert(color.to_hex());
            }
            "orange" => {
                scheme_palette
                    .entry("base09".to_string())
                    .or_insert(color.to_hex());
            }
            "yellow" => {
                scheme_palette
                    .entry("base0A".to_string())
                    .or_insert(color.to_hex());
            }
            "green" => {
                scheme_palette
                    .entry("base0B".to_string())
                    .or_insert(color.to_hex());
            }
            "cyan" => {
                scheme_palette
                    .entry("base0C".to_string())
                    .or_insert(color.to_hex());
            }
            "blue" => {
                scheme_palette
                    .entry("base0D".to_string())
                    .or_insert(color.to_hex());
            }
            "purple" => {
                scheme_palette
                    .entry("base0E".to_string())
                    .or_insert(color.to_hex());
            }
            "brown" => {
                scheme_palette
                    .entry("base0F".to_string())
                    .or_insert(color.to_hex());
            }
            _ => {}
        }
    }

    let scheme = Scheme {
        author,
        description,
        name,
        slug,
        system,
        variant,
        palette: scheme_palette,
    };

    Ok(scheme)
}
