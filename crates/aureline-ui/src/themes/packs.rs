//! First-party theme-pack fixtures.
//!
//! The current shell embeds the first-party theme packs from
//! `fixtures/design/themes/` at compile time. The packs are used to seed the
//! semantic token registry for each [`ThemeClass`].

use std::collections::HashMap;
use std::fmt;

use serde::Deserialize;

use crate::tokens::{ColorRgba, ThemeClass};

/// Errors returned when parsing the first-party theme-pack fixtures.
#[derive(Debug, Clone)]
pub enum ThemePackError {
    /// The fixture payload failed to parse.
    ParseFailed { theme: ThemeClass, detail: String },
    /// The fixture declared a different theme class than the loader requested.
    ThemeClassMismatch {
        requested: ThemeClass,
        declared: ThemeClass,
    },
    /// A token value literal could not be parsed as a color.
    InvalidTokenValue {
        theme: ThemeClass,
        token_name: String,
        value_literal: String,
    },
}

impl fmt::Display for ThemePackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseFailed { theme, detail } => {
                write!(f, "failed to parse theme pack {}: {detail}", theme.token())
            }
            Self::ThemeClassMismatch {
                requested,
                declared,
            } => write!(
                f,
                "theme pack mismatch: requested {} but fixture declared {}",
                requested.token(),
                declared.token()
            ),
            Self::InvalidTokenValue {
                theme,
                token_name,
                value_literal,
            } => write!(
                f,
                "theme pack {}: invalid token value for {token_name}: {value_literal}",
                theme.token()
            ),
        }
    }
}

impl std::error::Error for ThemePackError {}

/// First-party theme pack carrying semantic token literals for one [`ThemeClass`].
#[derive(Debug, Clone)]
pub struct ThemePack {
    theme_class: ThemeClass,
    mode_class: String,
    semantic_tokens: HashMap<String, ColorRgba>,
}

impl ThemePack {
    /// Returns the theme class this pack supports.
    pub const fn theme_class(&self) -> ThemeClass {
        self.theme_class
    }

    /// Returns the compact mode class (`dark`, `light`, `hc-dark`, `hc-light`).
    pub fn mode_class(&self) -> &str {
        &self.mode_class
    }

    /// Returns the semantic token table for this pack.
    pub fn semantic_tokens(&self) -> &HashMap<String, ColorRgba> {
        &self.semantic_tokens
    }
}

#[derive(Debug, Deserialize)]
struct ThemePackDoc {
    theme_class: ThemeClass,
    mode_class: String,
    #[serde(default)]
    semantic_tokens: HashMap<String, String>,
}

/// Loads the embedded first-party theme pack for the provided [`ThemeClass`].
pub fn load_first_party_theme_pack(theme: ThemeClass) -> Result<ThemePack, ThemePackError> {
    let payload = match theme {
        ThemeClass::DarkReference => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/design/themes/dark.json"
        )),
        ThemeClass::LightParity => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/design/themes/light.json"
        )),
        ThemeClass::HighContrastDark => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/design/themes/hc-dark.json"
        )),
        ThemeClass::HighContrastLight => include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/design/themes/hc-light.json"
        )),
    };

    let doc: ThemePackDoc =
        serde_json::from_str(payload).map_err(|err| ThemePackError::ParseFailed {
            theme,
            detail: err.to_string(),
        })?;

    if doc.theme_class != theme {
        return Err(ThemePackError::ThemeClassMismatch {
            requested: theme,
            declared: doc.theme_class,
        });
    }

    let mut semantic_tokens = HashMap::new();
    for (token_name, literal) in doc.semantic_tokens {
        let Some(color) = ColorRgba::parse(&literal) else {
            return Err(ThemePackError::InvalidTokenValue {
                theme,
                token_name,
                value_literal: literal,
            });
        };
        semantic_tokens.insert(token_name, color);
    }

    Ok(ThemePack {
        theme_class: doc.theme_class,
        mode_class: doc.mode_class,
        semantic_tokens,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tokens::TokenRegistry;

    #[test]
    fn loads_theme_packs_for_all_supported_classes() {
        for theme in [
            ThemeClass::DarkReference,
            ThemeClass::LightParity,
            ThemeClass::HighContrastDark,
            ThemeClass::HighContrastLight,
        ] {
            let pack = load_first_party_theme_pack(theme).expect("theme pack should load");
            assert_eq!(pack.theme_class(), theme);
            assert!(!pack.mode_class().is_empty());
            assert!(
                pack.semantic_tokens().contains_key("al.color.bg.canvas"),
                "theme pack must include canvas token"
            );
        }
    }

    #[test]
    fn high_contrast_packs_override_canvas_values() {
        let dark = load_first_party_theme_pack(ThemeClass::DarkReference).expect("dark pack");
        let hc_dark =
            load_first_party_theme_pack(ThemeClass::HighContrastDark).expect("hc-dark pack");
        assert_ne!(
            dark.semantic_tokens().get("al.color.bg.canvas"),
            hc_dark.semantic_tokens().get("al.color.bg.canvas"),
            "high-contrast dark must not be identical to the dark reference canvas"
        );

        let light = load_first_party_theme_pack(ThemeClass::LightParity).expect("light pack");
        let hc_light =
            load_first_party_theme_pack(ThemeClass::HighContrastLight).expect("hc-light pack");
        assert_ne!(
            light.semantic_tokens().get("al.color.bg.canvas"),
            hc_light.semantic_tokens().get("al.color.bg.canvas"),
            "high-contrast light must not be identical to the light parity canvas"
        );
    }

    #[test]
    fn token_registry_matches_theme_pack_semantic_literals() {
        for theme in [
            ThemeClass::DarkReference,
            ThemeClass::LightParity,
            ThemeClass::HighContrastDark,
            ThemeClass::HighContrastLight,
        ] {
            let pack = load_first_party_theme_pack(theme).expect("theme pack should load");
            let registry = TokenRegistry::load(theme).expect("token registry should load");
            for (token_name, expected) in pack.semantic_tokens() {
                let actual = registry.require_color(token_name).unwrap_or_else(|_| {
                    panic!("token registry missing {token_name} for {theme:?}")
                });
                assert_eq!(
                    actual,
                    *expected,
                    "token registry drift for {token_name} on {}",
                    theme.token()
                );
            }
        }
    }
}
