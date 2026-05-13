//! Theme-package appearance manifest projection.
//!
//! The manifest binds a theme package identity to its supported modes, density
//! defaults, motion-posture coverage, and minimum contrast targets.

use std::collections::HashSet;
use std::fmt;
use std::sync::OnceLock;

use serde::Deserialize;

use super::session::AccessibilityPostureClass;
use crate::density::DensityClass;
use crate::tokens::ThemeClass;

const FIRST_PARTY_MANIFEST_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/design/theme_package_cases/first_party_default_theme_manifest.yaml"
));

static FIRST_PARTY_MANIFEST: OnceLock<
    Result<ThemePackageAppearanceManifest, ThemePackageManifestError>,
> = OnceLock::new();

/// Error emitted while loading a theme-package appearance manifest.
#[derive(Debug, Clone)]
pub enum ThemePackageManifestError {
    /// The embedded manifest YAML did not parse.
    ParseFailed(String),
    /// The manifest is structurally invalid.
    InvalidManifest(String),
}

impl fmt::Display for ThemePackageManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseFailed(detail) => {
                write!(f, "failed to parse theme package manifest: {detail}")
            }
            Self::InvalidManifest(detail) => write!(f, "invalid theme package manifest: {detail}"),
        }
    }
}

impl std::error::Error for ThemePackageManifestError {}

/// Loaded theme-package manifest used by appearance sessions and audit packets.
#[derive(Debug, Clone)]
pub struct ThemePackageAppearanceManifest {
    doc: ThemePackageManifestDoc,
}

impl ThemePackageAppearanceManifest {
    /// Loads and validates the first-party default theme package manifest.
    pub fn load_first_party_default() -> Result<Self, ThemePackageManifestError> {
        let doc: ThemePackageManifestDoc = serde_yaml::from_str(FIRST_PARTY_MANIFEST_YAML)
            .map_err(|err| ThemePackageManifestError::ParseFailed(err.to_string()))?;
        let manifest = Self { doc };
        manifest.validate_first_party_complete()?;
        Ok(manifest)
    }

    /// Returns the design-side appearance manifest id.
    pub fn appearance_manifest_id(&self) -> &str {
        &self.doc.appearance_manifest_id
    }

    /// Returns the claim-facing theme-package manifest ref.
    pub fn theme_package_manifest_ref(&self) -> &str {
        &self.doc.theme_package_manifest_ref
    }

    /// Returns the claim-facing theme-package revision ref.
    pub fn theme_package_revision_ref(&self) -> &str {
        &self.doc.theme_package_revision_ref
    }

    /// Returns the canonical package id.
    pub fn package_id(&self) -> &str {
        &self.doc.package_id
    }

    /// Returns the package version label.
    pub fn package_version_label(&self) -> &str {
        &self.doc.package_version_label
    }

    /// Returns the default density class declared by the package.
    pub const fn default_density_class(&self) -> DensityClass {
        self.doc.density_defaults.default_density_class
    }

    /// Returns the default motion posture declared by the package.
    pub const fn default_motion_posture(&self) -> AccessibilityPostureClass {
        self.doc.motion_flags.default_accessibility_posture_class
    }

    /// Returns true when this package declares support for a theme class.
    pub fn supports_theme_class(&self, theme_class: ThemeClass) -> bool {
        self.doc
            .supported_modes
            .iter()
            .any(|mode| mode.theme_class == theme_class)
    }

    /// Returns true when this package declares support for a density class.
    pub fn supports_density_class(&self, density_class: DensityClass) -> bool {
        self.doc
            .density_defaults
            .supported_density_classes
            .contains(&density_class)
    }

    /// Returns true when this package declares support for a motion posture.
    pub fn supports_motion_posture(&self, posture: AccessibilityPostureClass) -> bool {
        self.doc
            .motion_flags
            .supported_accessibility_posture_classes
            .contains(&posture)
    }

    /// Returns the minimum text contrast target for the requested theme class.
    pub fn minimum_text_contrast_target(&self, theme_class: ThemeClass) -> Option<f32> {
        self.mode(theme_class)
            .map(|mode| mode.contrast_targets.minimum_text_contrast_target)
    }

    /// Returns the minimum non-text UI contrast target for the requested theme class.
    pub fn minimum_ui_contrast_target(&self, theme_class: ThemeClass) -> Option<f32> {
        self.mode(theme_class)
            .map(|mode| mode.contrast_targets.minimum_ui_contrast_target)
    }

    /// Returns the focus-ring stroke width for the requested theme class.
    pub fn focus_ring_stroke_px(&self, theme_class: ThemeClass) -> Option<u32> {
        self.mode(theme_class)
            .map(|mode| mode.contrast_targets.focus_ring_stroke_px)
    }

    fn mode(&self, theme_class: ThemeClass) -> Option<&SupportedModeDeclarationDoc> {
        self.doc
            .supported_modes
            .iter()
            .find(|mode| mode.theme_class == theme_class)
    }

    fn validate_first_party_complete(&self) -> Result<(), ThemePackageManifestError> {
        if self.doc.record_kind != "theme_package_appearance_manifest_record" {
            return Err(ThemePackageManifestError::InvalidManifest(format!(
                "unexpected record_kind {}",
                self.doc.record_kind
            )));
        }
        if self.doc.theme_package_manifest_schema_version != 1 {
            return Err(ThemePackageManifestError::InvalidManifest(format!(
                "unsupported schema version {}",
                self.doc.theme_package_manifest_schema_version
            )));
        }

        let expected_modes = [
            ("dark", ThemeClass::DarkReference),
            ("light", ThemeClass::LightParity),
            ("hc-dark", ThemeClass::HighContrastDark),
            ("hc-light", ThemeClass::HighContrastLight),
        ];
        let mut seen = HashSet::new();
        for mode in &self.doc.supported_modes {
            if !seen.insert(mode.theme_class) {
                return Err(ThemePackageManifestError::InvalidManifest(format!(
                    "duplicate supported mode for {}",
                    mode.theme_class.token()
                )));
            }
            let aligned = expected_modes.iter().any(|(mode_class, theme_class)| {
                mode.mode_class == *mode_class && mode.theme_class == *theme_class
            });
            if !aligned {
                return Err(ThemePackageManifestError::InvalidManifest(format!(
                    "mode {} is not aligned with theme class {}",
                    mode.mode_class,
                    mode.theme_class.token()
                )));
            }
            if mode.evidence_path.trim().is_empty() {
                return Err(ThemePackageManifestError::InvalidManifest(format!(
                    "{} mode has empty evidence_path",
                    mode.theme_class.token()
                )));
            }
        }
        for (_, theme_class) in expected_modes {
            if !seen.contains(&theme_class) {
                return Err(ThemePackageManifestError::InvalidManifest(format!(
                    "missing first-party mode {}",
                    theme_class.token()
                )));
            }
        }

        for density_class in [
            DensityClass::Compact,
            DensityClass::Standard,
            DensityClass::Comfortable,
        ] {
            if !self.supports_density_class(density_class) {
                return Err(ThemePackageManifestError::InvalidManifest(format!(
                    "missing density {}",
                    density_class.token()
                )));
            }
        }
        for posture in [
            AccessibilityPostureClass::MotionStandard,
            AccessibilityPostureClass::MotionReduced,
            AccessibilityPostureClass::MotionLowMotion,
            AccessibilityPostureClass::MotionPowerSaver,
            AccessibilityPostureClass::MotionCriticalHotPath,
        ] {
            if !self.supports_motion_posture(posture) {
                return Err(ThemePackageManifestError::InvalidManifest(format!(
                    "missing motion posture {}",
                    posture.token()
                )));
            }
        }
        Ok(())
    }
}

/// Returns the canonical first-party theme-package manifest.
pub fn first_party_theme_package_manifest(
) -> Result<&'static ThemePackageAppearanceManifest, ThemePackageManifestError> {
    let manifest =
        FIRST_PARTY_MANIFEST.get_or_init(ThemePackageAppearanceManifest::load_first_party_default);
    match manifest {
        Ok(manifest) => Ok(manifest),
        Err(err) => Err(err.clone()),
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct ThemePackageManifestDoc {
    record_kind: String,
    theme_package_manifest_schema_version: u32,
    appearance_manifest_id: String,
    theme_package_manifest_ref: String,
    theme_package_revision_ref: String,
    package_id: String,
    package_revision_ref: String,
    package_version_label: String,
    supported_modes: Vec<SupportedModeDeclarationDoc>,
    density_defaults: DensityDefaultsDoc,
    motion_flags: MotionFlagSummaryDoc,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct SupportedModeDeclarationDoc {
    mode_class: String,
    theme_class: ThemeClass,
    contrast_targets: ContrastTargetMetadataDoc,
    evidence_path: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct ContrastTargetMetadataDoc {
    minimum_text_contrast_target: f32,
    minimum_ui_contrast_target: f32,
    focus_ring_contrast_target: f32,
    focus_ring_stroke_px: u32,
    forced_colors_friendly: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct DensityDefaultsDoc {
    default_density_class: DensityClass,
    supported_density_classes: Vec<DensityClass>,
}

#[derive(Debug, Clone, Deserialize)]
struct MotionFlagSummaryDoc {
    default_accessibility_posture_class: AccessibilityPostureClass,
    supported_accessibility_posture_classes: Vec<AccessibilityPostureClass>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_party_manifest_declares_complete_alpha_appearance_floor() {
        let manifest = first_party_theme_package_manifest().expect("first-party manifest");
        assert_eq!(
            manifest.theme_package_manifest_ref(),
            "theme_package_manifest:default.aureline:01"
        );
        for theme_class in [
            ThemeClass::DarkReference,
            ThemeClass::LightParity,
            ThemeClass::HighContrastDark,
            ThemeClass::HighContrastLight,
        ] {
            assert!(manifest.supports_theme_class(theme_class));
            assert!(manifest
                .minimum_text_contrast_target(theme_class)
                .is_some_and(|target| target >= 4.5));
        }
        assert!(manifest.supports_density_class(DensityClass::Compact));
        assert!(manifest.supports_density_class(DensityClass::Standard));
        assert!(manifest.supports_density_class(DensityClass::Comfortable));
        assert!(manifest.supports_motion_posture(AccessibilityPostureClass::MotionPowerSaver));
        assert!(manifest.supports_motion_posture(AccessibilityPostureClass::MotionCriticalHotPath));
    }
}
