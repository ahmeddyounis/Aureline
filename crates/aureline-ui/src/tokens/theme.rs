/// Supported first-party theme classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThemeClass {
    DarkReference,
    LightParity,
    HighContrastDark,
    HighContrastLight,
}

impl ThemeClass {
    /// Returns the canonical snake-case theme class identifier.
    pub const fn token(self) -> &'static str {
        match self {
            Self::DarkReference => "dark_reference",
            Self::LightParity => "light_parity",
            Self::HighContrastDark => "high_contrast_dark",
            Self::HighContrastLight => "high_contrast_light",
        }
    }
}
