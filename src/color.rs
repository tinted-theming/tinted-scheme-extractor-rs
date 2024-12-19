use palette::{rgb::Rgb, FromColor, Hsl, IntoColor, Srgb};

#[derive(Clone, Copy, Debug)]
pub(crate) struct Color {
    pub(crate) associated_pure_color: PureColor,
    pub(crate) value: Srgb<u8>,
    pub(crate) distance: f64,
}

impl Color {
    /// Create a new color
    /// The distance is calculated using the Euclidean distance formula
    ///
    /// # Arguments
    /// * `pure_color` - A PureColor enum
    /// * `value` - A Srgb<u8> color
    pub(crate) fn new(pure_color: PureColor, value: Srgb<u8>) -> Self {
        let distance = Color::get_distance(&Color::from(pure_color).value, &value);

        Color {
            associated_pure_color: pure_color,
            value,
            distance,
        }
    }

    /// Create a new color from a pure color
    pub(crate) fn from(pure_color: PureColor) -> Self {
        Color {
            associated_pure_color: pure_color,
            value: pure_color.get_rgb(),
            distance: 0.0,
        }
    }

    /// Get the inverse of the color
    pub(crate) fn get_inverse(&self) -> Self {
        let rgb_color_inverse = Srgb::new(
            255 - self.value.red,
            255 - self.value.green,
            255 - self.value.blue,
        );
        let pure_color_inverse = self.associated_pure_color.get_inverse();

        Color::new(pure_color_inverse, rgb_color_inverse)
    }

    /// Get the distance between two colors
    /// The distance is calculated using the Euclidean distance formula
    ///
    /// # Arguments
    /// * `c1` - A reference to a Srgb<u8> color
    /// * `c2` - A reference to a Srgb<u8> color
    pub(crate) fn get_distance(c1: &Srgb<u8>, c2: &Srgb<u8>) -> f64 {
        // Order of c1 and c2 doesn't matter
        let dr = c1.red as i32 - c2.red as i32;
        let dg = c1.green as i32 - c2.green as i32;
        let db = c1.blue as i32 - c2.blue as i32;

        ((dr * dr + dg * dg + db * db) as f64).sqrt()
    }

    /// Convert the color to a hex string
    pub(crate) fn to_hex(self) -> String {
        let (r, g, b) = self.value.into_components();

        format!("{:02X}{:02X}{:02X}", r, g, b)
    }

    /// Saturate the color
    /// The percentage is squared to make the saturation effect more noticeable
    ///
    /// # Arguments
    /// * `percentage` - A f32 value between 0.0 and 1.0
    pub(crate) fn to_saturated(mut self, percentage: f32) -> Self {
        let percentage = percentage.clamp(0.0, 1.0);
        let hsl: Hsl = Hsl::from_color(self.value.into_format::<f32>());
        let updated_saturation: Hsl = Hsl::new(
            hsl.hue,
            hsl.saturation * percentage * percentage,
            hsl.lightness,
        );
        let updated_rgb: Rgb = updated_saturation.into_color();

        self.value = Srgb::new(
            (updated_rgb.red * 255.0) as u8,
            (updated_rgb.green * 255.0) as u8,
            (updated_rgb.blue * 255.0) as u8,
        );

        self
    }

    /// Add lightness to the color
    ///
    /// # Arguments
    ///
    /// * `value` - A f32 value between 0.0 and 1.0
    ///
    pub(crate) fn add_lightness(mut self, value: f32) -> Self {
        let hsl: Hsl = Hsl::from_color(self.value.into_format::<f32>());
        let updated_lightness = (hsl.lightness + value.clamp(0.0, 1.0)).clamp(0.0, 1.0);
        let hsl: Hsl = Hsl::new(hsl.hue, hsl.saturation, updated_lightness);
        let updated_rgb: Rgb = hsl.into_color();

        self.value = Srgb::new(
            (updated_rgb.red * 255.0) as u8,
            (updated_rgb.green * 255.0) as u8,
            (updated_rgb.blue * 255.0) as u8,
        );

        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum PureColor {
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
    pub(crate) fn get_rgb(&self) -> Srgb<u8> {
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

    pub(crate) fn as_str(&self) -> &str {
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

    pub(crate) fn get_inverse(&self) -> PureColor {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_lightness() {
        let color = Color::new(PureColor::Red, Srgb::new(255, 0, 0));
        let color = color.add_lightness(0.1);

        assert_eq!(color.value, Srgb::new(255, 51, 51));
    }

    #[test]
    fn test_get_distance() {
        let color1 = Srgb::new(255, 0, 0);
        let color2 = Srgb::new(0, 255, 0);

        assert_eq!(Color::get_distance(&color1, &color2), 360.62445840513925);
    }

    #[test]
    fn test_get_inverse() {
        let color = Color::new(PureColor::Red, Srgb::new(255, 0, 0));
        let color = color.get_inverse();

        assert_eq!(color.associated_pure_color, PureColor::Cyan);
        assert_eq!(color.value, Srgb::new(0, 255, 255));
    }

    #[test]
    fn test_to_hex() {
        let color = Color::new(PureColor::Red, Srgb::new(255, 0, 0));

        assert_eq!(color.to_hex(), "FF0000");
    }
}
