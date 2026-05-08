use std::collections::HashMap;
use std::fmt;
use std::sync::OnceLock;

use super::color::ColorRgba;
use super::loaders::{
    load_color_tokens, load_geometry_tokens, load_motion_tokens, GeometryTokenLedger,
    MotionTokenLedger,
};
use super::theme::ThemeClass;

/// Errors emitted while building or querying the token registry.
#[derive(Debug, Clone)]
pub enum TokenRegistryError {
    LoadFailed(&'static str, String),
    MissingToken { token_name: String },
}

impl fmt::Display for TokenRegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LoadFailed(ledger, detail) => write!(f, "failed to load {ledger}: {detail}"),
            Self::MissingToken { token_name } => write!(f, "missing token: {token_name}"),
        }
    }
}

impl std::error::Error for TokenRegistryError {}

/// Registry for semantic color tokens plus the baseline geometry and motion ledgers.
#[derive(Debug, Clone)]
pub struct TokenRegistry {
    theme: ThemeClass,
    colors: HashMap<String, ColorRgba>,
    geometry: GeometryTokenLedger,
    motion: MotionTokenLedger,
}

impl TokenRegistry {
    /// Loads the token registry for the provided theme class.
    pub fn load(theme: ThemeClass) -> Result<Self, TokenRegistryError> {
        let colors = load_color_tokens(theme)
            .map_err(|err| TokenRegistryError::LoadFailed("color ledgers", err))?;
        let geometry = load_geometry_tokens()
            .map_err(|err| TokenRegistryError::LoadFailed("geometry token ledger", err))?;
        let motion = load_motion_tokens()
            .map_err(|err| TokenRegistryError::LoadFailed("motion token ledger", err))?;

        Ok(Self {
            theme,
            colors,
            geometry,
            motion,
        })
    }

    /// Returns the active theme class.
    pub const fn theme(&self) -> ThemeClass {
        self.theme
    }

    /// Returns the color bound to a semantic token name.
    pub fn color(&self, token_name: &str) -> Option<ColorRgba> {
        self.colors.get(token_name).copied()
    }

    /// Returns the color bound to a semantic token name or errors if missing.
    pub fn require_color(&self, token_name: &str) -> Result<ColorRgba, TokenRegistryError> {
        self.color(token_name)
            .ok_or_else(|| TokenRegistryError::MissingToken {
                token_name: token_name.to_owned(),
            })
    }

    /// Returns the space token (px) for the provided token name.
    pub fn space_px(&self, token_name: &str) -> Option<u32> {
        self.geometry.spaces_px.get(token_name).copied()
    }

    /// Returns the space token (px) for the provided token name or errors if missing.
    pub fn require_space_px(&self, token_name: &str) -> Result<u32, TokenRegistryError> {
        self.space_px(token_name)
            .ok_or_else(|| TokenRegistryError::MissingToken {
                token_name: token_name.to_owned(),
            })
    }

    /// Returns the size token (px) for the provided token name.
    pub fn size_px(&self, token_name: &str) -> Option<u32> {
        self.geometry.sizes_px.get(token_name).copied()
    }

    /// Returns the size token (px) for the provided token name or errors if missing.
    pub fn require_size_px(&self, token_name: &str) -> Result<u32, TokenRegistryError> {
        self.size_px(token_name)
            .ok_or_else(|| TokenRegistryError::MissingToken {
                token_name: token_name.to_owned(),
            })
    }

    /// Returns the radius token (px) for the provided token name.
    pub fn radius_px(&self, token_name: &str) -> Option<u32> {
        self.geometry.radii_px.get(token_name).copied()
    }

    /// Returns the radius token (px) for the provided token name or errors if missing.
    pub fn require_radius_px(&self, token_name: &str) -> Result<u32, TokenRegistryError> {
        self.radius_px(token_name)
            .ok_or_else(|| TokenRegistryError::MissingToken {
                token_name: token_name.to_owned(),
            })
    }

    /// Returns the border stroke token (px) for the provided token name.
    pub fn stroke_px(&self, token_name: &str) -> Option<u32> {
        self.geometry.strokes_px.get(token_name).copied()
    }

    /// Returns the border stroke token (px) for the provided token name or errors if missing.
    pub fn require_stroke_px(&self, token_name: &str) -> Result<u32, TokenRegistryError> {
        self.stroke_px(token_name)
            .ok_or_else(|| TokenRegistryError::MissingToken {
                token_name: token_name.to_owned(),
            })
    }

    /// Returns the motion duration token in milliseconds.
    pub fn motion_ms(&self, token_name: &str) -> Option<u32> {
        self.motion.durations_ms.get(token_name).copied()
    }

    /// Returns the motion duration token in milliseconds or errors if missing.
    pub fn require_motion_ms(&self, token_name: &str) -> Result<u32, TokenRegistryError> {
        self.motion_ms(token_name)
            .ok_or_else(|| TokenRegistryError::MissingToken {
                token_name: token_name.to_owned(),
            })
    }
}

static SEEDED_DARK_REFERENCE: OnceLock<Result<TokenRegistry, TokenRegistryError>> = OnceLock::new();
static SEEDED_LIGHT_PARITY: OnceLock<Result<TokenRegistry, TokenRegistryError>> = OnceLock::new();
static SEEDED_HIGH_CONTRAST_DARK: OnceLock<Result<TokenRegistry, TokenRegistryError>> =
    OnceLock::new();
static SEEDED_HIGH_CONTRAST_LIGHT: OnceLock<Result<TokenRegistry, TokenRegistryError>> =
    OnceLock::new();

/// Returns the canonical token registry for the requested theme class.
///
/// The registry is backed by the design ledgers under `artifacts/design/`.
pub fn seeded_token_registry(
    theme: ThemeClass,
) -> Result<&'static TokenRegistry, TokenRegistryError> {
    let seeded = match theme {
        ThemeClass::DarkReference => {
            SEEDED_DARK_REFERENCE.get_or_init(|| TokenRegistry::load(theme))
        }
        ThemeClass::LightParity => SEEDED_LIGHT_PARITY.get_or_init(|| TokenRegistry::load(theme)),
        ThemeClass::HighContrastDark => {
            SEEDED_HIGH_CONTRAST_DARK.get_or_init(|| TokenRegistry::load(theme))
        }
        ThemeClass::HighContrastLight => {
            SEEDED_HIGH_CONTRAST_LIGHT.get_or_init(|| TokenRegistry::load(theme))
        }
    };

    match seeded {
        Ok(registry) => Ok(registry),
        Err(err) => Err(err.clone()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_theme_and_status_tokens() {
        let registry =
            TokenRegistry::load(ThemeClass::DarkReference).expect("load dark reference registry");
        let overlay = registry
            .require_color("al.color.bg.overlay")
            .expect("overlay token");
        assert!(overlay.a < 255, "overlay scrim should carry alpha");

        let status_border = registry
            .require_color("status.warning.border")
            .expect("status warning border token");
        assert!(
            status_border.a < 255,
            "status border token should carry alpha"
        );

        registry
            .require_space_px("space.4")
            .expect("geometry spacing token");
        registry
            .require_motion_ms("motion.ui")
            .expect("motion duration token");
    }

    #[test]
    fn loads_high_contrast_theme_with_fallback() {
        let registry =
            TokenRegistry::load(ThemeClass::HighContrastDark).expect("load high contrast registry");
        registry
            .require_color("al.color.bg.canvas")
            .expect("fallback theme token");
    }
}
