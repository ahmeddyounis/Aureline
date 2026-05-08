use core::fmt;

/// An sRGBA color in 8-bit channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ColorRgba {
    /// Returns a fully opaque color.
    pub const fn opaque(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Parses a design-token color literal.
    ///
    /// Supported forms:
    /// - `#RRGGBB`
    /// - `#RRGGBBAA`
    /// - CSS `rgba` functional notation where alpha is in `[0,1]`
    pub fn parse(value: &str) -> Option<Self> {
        let trimmed = value.trim();
        if let Some(hex) = trimmed.strip_prefix('#') {
            return parse_hex(hex);
        }
        if trimmed.starts_with("rgba") {
            return parse_rgba_function(trimmed);
        }
        None
    }

    /// Converts the color into the `softbuffer` pixel format (`0RGB`).
    pub const fn to_u32_rgb(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Blends this color over an opaque destination pixel in `0RGB` format.
    pub fn blend_over_u32(self, dst_rgb: u32) -> u32 {
        if self.a == 255 {
            return self.to_u32_rgb();
        }
        if self.a == 0 {
            return dst_rgb;
        }
        let dst_r = ((dst_rgb >> 16) & 255) as u8;
        let dst_g = ((dst_rgb >> 8) & 255) as u8;
        let dst_b = (dst_rgb & 255) as u8;
        let a = self.a as u16;
        let inv_a = 255u16.saturating_sub(a);

        let out_r = ((u16::from(self.r) * a + u16::from(dst_r) * inv_a) / 255) as u8;
        let out_g = ((u16::from(self.g) * a + u16::from(dst_g) * inv_a) / 255) as u8;
        let out_b = ((u16::from(self.b) * a + u16::from(dst_b) * inv_a) / 255) as u8;
        Self::opaque(out_r, out_g, out_b).to_u32_rgb()
    }
}

impl fmt::Display for ColorRgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgba[{}, {}, {}, {}]", self.r, self.g, self.b, self.a)
    }
}

fn parse_hex(value: &str) -> Option<ColorRgba> {
    fn hex_byte(two: &str) -> Option<u8> {
        u8::from_str_radix(two, 16).ok()
    }

    match value.len() {
        6 => {
            let r = hex_byte(value.get(0..2)?)?;
            let g = hex_byte(value.get(2..4)?)?;
            let b = hex_byte(value.get(4..6)?)?;
            Some(ColorRgba::opaque(r, g, b))
        }
        8 => {
            let r = hex_byte(value.get(0..2)?)?;
            let g = hex_byte(value.get(2..4)?)?;
            let b = hex_byte(value.get(4..6)?)?;
            let a = hex_byte(value.get(6..8)?)?;
            Some(ColorRgba { r, g, b, a })
        }
        _ => None,
    }
}

fn parse_rgba_function(value: &str) -> Option<ColorRgba> {
    let open = value.find('(')?;
    let close = value.rfind(')')?;
    let args = value.get(open + 1..close)?;
    let mut parts = args.split(',').map(|p| p.trim());
    let r: u8 = parts.next()?.parse().ok()?;
    let g: u8 = parts.next()?.parse().ok()?;
    let b: u8 = parts.next()?.parse().ok()?;
    let a_raw = parts.next()?;
    if parts.next().is_some() {
        return None;
    }

    let alpha: f32 = a_raw.parse().ok()?;
    if !(0.0..=1.0).contains(&alpha) {
        return None;
    }
    let a = (alpha * 255.0).round().clamp(0.0, 255.0) as u8;
    Some(ColorRgba { r, g, b, a })
}
