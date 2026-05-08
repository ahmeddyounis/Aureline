use std::collections::HashMap;

use serde::Deserialize;
use serde_yaml::Value as YamlValue;

use super::color::ColorRgba;
use super::theme::ThemeClass;

pub(super) fn load_color_tokens(theme: ThemeClass) -> Result<HashMap<String, ColorRgba>, String> {
    let mut colors: HashMap<String, ColorRgba> = HashMap::new();
    match theme {
        ThemeClass::HighContrastDark => {
            load_theme_support_semantic_tokens(ThemeClass::DarkReference, &mut colors)?;
            load_theme_support_semantic_tokens(theme, &mut colors)?;
        }
        ThemeClass::HighContrastLight => {
            load_theme_support_semantic_tokens(ThemeClass::LightParity, &mut colors)?;
            load_theme_support_semantic_tokens(theme, &mut colors)?;
        }
        _ => load_theme_support_semantic_tokens(theme, &mut colors)?,
    }
    load_semantic_domain_tokens(theme, &mut colors)?;
    Ok(colors)
}

pub(super) fn load_geometry_tokens() -> Result<GeometryTokenLedger, String> {
    let doc: GeometryTokenDoc = serde_yaml::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/design/geometry_token_ledger.yaml"
    )))
    .map_err(|err| format!("failed to parse geometry token ledger: {err}"))?;

    Ok(GeometryTokenLedger {
        spaces_px: collect_px_tokens(doc.spacing_tokens),
        sizes_px: collect_px_tokens(doc.sizing_tokens),
        radii_px: collect_px_tokens(doc.radius_tokens),
        strokes_px: collect_px_tokens(doc.border_stroke_tokens),
    })
}

pub(super) fn load_motion_tokens() -> Result<MotionTokenLedger, String> {
    let doc: MotionTokenDoc = serde_yaml::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/design/motion_tokens.yaml"
    )))
    .map_err(|err| format!("failed to parse motion token ledger: {err}"))?;

    let mut durations_ms: HashMap<String, u32> = HashMap::new();
    for token in doc.duration_tokens {
        durations_ms.insert(token.token_name, token.ms);
    }
    Ok(MotionTokenLedger { durations_ms })
}

fn load_theme_support_semantic_tokens(
    theme: ThemeClass,
    colors: &mut HashMap<String, ColorRgba>,
) -> Result<(), String> {
    let doc: ThemeSupportDoc = serde_yaml::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/design/theme_support_rows.yaml"
    )))
    .map_err(|err| format!("failed to parse theme support rows: {err}"))?;

    for row in doc.theme_support_rows {
        if row.theme_class.as_deref() != Some(theme.token()) {
            continue;
        }
        let Some(tokens) = row.example_semantic_tokens else {
            continue;
        };
        for token in tokens {
            let Some(value) = token.value_literal.as_deref() else {
                continue;
            };
            let Some(color) = ColorRgba::parse(value) else {
                continue;
            };
            colors.insert(token.token_name, color);
        }
    }
    Ok(())
}

fn load_semantic_domain_tokens(
    theme: ThemeClass,
    colors: &mut HashMap<String, ColorRgba>,
) -> Result<(), String> {
    let doc: SemanticTokenDomainDoc = serde_yaml::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/design/semantic_token_domains.yaml"
    )))
    .map_err(|err| format!("failed to parse semantic token domains ledger: {err}"))?;

    let mut pending_status_refs: Vec<(String, String)> = Vec::new();
    for domain in doc.domains {
        let Some(tokens) = domain.tokens else {
            continue;
        };
        for token in tokens {
            let value = match theme {
                ThemeClass::DarkReference | ThemeClass::HighContrastDark => {
                    token.dark_reference.as_ref()
                }
                ThemeClass::LightParity | ThemeClass::HighContrastLight => {
                    token.light_parity.as_ref()
                }
            }
            .and_then(YamlValue::as_str);

            if let Some(value) = value {
                let Some(color) = ColorRgba::parse(value) else {
                    continue;
                };
                colors.insert(token.token_name.clone(), color);
                continue;
            }

            if let Some(status_ref) = token.semantic_status_token_ref.as_deref() {
                pending_status_refs.push((token.token_name, status_ref.to_owned()));
            }
        }
    }

    for (token_name, status_ref) in pending_status_refs {
        let Some(color) = colors.get(&status_ref).copied() else {
            continue;
        };
        colors.insert(token_name, color);
    }
    Ok(())
}

fn collect_px_tokens(entries: Vec<PxTokenRow>) -> HashMap<String, u32> {
    let mut out = HashMap::new();
    for row in entries {
        out.insert(row.token_name, row.px);
    }
    out
}

#[derive(Debug, Clone)]
pub(super) struct GeometryTokenLedger {
    pub(super) spaces_px: HashMap<String, u32>,
    pub(super) sizes_px: HashMap<String, u32>,
    pub(super) radii_px: HashMap<String, u32>,
    pub(super) strokes_px: HashMap<String, u32>,
}

#[derive(Debug, Clone)]
pub(super) struct MotionTokenLedger {
    pub(super) durations_ms: HashMap<String, u32>,
}

#[derive(Debug, Deserialize)]
struct ThemeSupportDoc {
    #[serde(default)]
    theme_support_rows: Vec<ThemeSupportRow>,
}

#[derive(Debug, Deserialize)]
struct ThemeSupportRow {
    theme_class: Option<String>,
    #[serde(default)]
    example_semantic_tokens: Option<Vec<ThemeSupportToken>>,
}

#[derive(Debug, Deserialize)]
struct ThemeSupportToken {
    token_name: String,
    value_literal: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SemanticTokenDomainDoc {
    #[serde(default)]
    domains: Vec<SemanticTokenDomain>,
}

#[derive(Debug, Deserialize)]
struct SemanticTokenDomain {
    #[serde(default)]
    tokens: Option<Vec<SemanticDomainToken>>,
}

#[derive(Debug, Deserialize)]
struct SemanticDomainToken {
    token_name: String,
    #[serde(default)]
    dark_reference: Option<YamlValue>,
    #[serde(default)]
    light_parity: Option<YamlValue>,
    #[serde(default)]
    semantic_status_token_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GeometryTokenDoc {
    #[serde(default)]
    spacing_tokens: Vec<PxTokenRow>,
    #[serde(default)]
    sizing_tokens: Vec<PxTokenRow>,
    #[serde(default)]
    radius_tokens: Vec<PxTokenRow>,
    #[serde(default)]
    border_stroke_tokens: Vec<PxTokenRow>,
}

#[derive(Debug, Deserialize)]
struct PxTokenRow {
    token_name: String,
    px: u32,
}

#[derive(Debug, Deserialize)]
struct MotionTokenDoc {
    #[serde(default)]
    duration_tokens: Vec<MotionDurationToken>,
}

#[derive(Debug, Deserialize)]
struct MotionDurationToken {
    token_name: String,
    ms: u32,
}
