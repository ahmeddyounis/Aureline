//! Alpha appearance visual-diff and accessibility audit projection.
//!
//! The audit manifest ties dark, light, high-contrast, density, reduced-motion,
//! OS-signal, and import-review evidence to one appearance-session object.

use std::collections::HashSet;
use std::fmt;
use std::sync::OnceLock;

use serde::Deserialize;

use super::session::AccessibilityPostureClass;
use crate::density::DensityClass;
use crate::tokens::ThemeClass;

const ALPHA_AUDIT_MANIFEST_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/design/m2_appearance_visual_diff_alpha/manifest.yaml"
));

static ALPHA_AUDIT_MANIFEST: OnceLock<
    Result<AppearanceVisualDiffAuditManifest, AppearanceAuditError>,
> = OnceLock::new();

/// Error emitted while loading the alpha appearance audit manifest.
#[derive(Debug, Clone)]
pub enum AppearanceAuditError {
    /// The embedded manifest YAML did not parse.
    ParseFailed(String),
    /// The manifest is structurally invalid.
    InvalidManifest(String),
}

impl fmt::Display for AppearanceAuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseFailed(detail) => {
                write!(f, "failed to parse appearance audit manifest: {detail}")
            }
            Self::InvalidManifest(detail) => {
                write!(f, "invalid appearance audit manifest: {detail}")
            }
        }
    }
}

impl std::error::Error for AppearanceAuditError {}

/// Loaded alpha visual-diff and accessibility audit manifest.
#[derive(Debug, Clone)]
pub struct AppearanceVisualDiffAuditManifest {
    doc: AppearanceAuditManifestDoc,
}

impl AppearanceVisualDiffAuditManifest {
    /// Loads and validates the embedded alpha audit manifest.
    pub fn load_alpha() -> Result<Self, AppearanceAuditError> {
        let doc: AppearanceAuditManifestDoc = serde_yaml::from_str(ALPHA_AUDIT_MANIFEST_YAML)
            .map_err(|err| AppearanceAuditError::ParseFailed(err.to_string()))?;
        let manifest = Self { doc };
        manifest.validate()?;
        Ok(manifest)
    }

    /// Returns the stable manifest id.
    pub fn manifest_id(&self) -> &str {
        &self.doc.manifest_id
    }

    /// Returns the appearance session ref shared by visual-diff rows.
    pub fn appearance_session_ref(&self) -> &str {
        &self.doc.appearance_session_ref
    }

    /// Returns the number of visual-diff cases.
    pub fn visual_diff_case_count(&self) -> usize {
        self.doc.visual_diff_cases.len()
    }

    /// Returns true when the manifest contains a visual-diff row for a theme class.
    pub fn has_visual_diff_for_theme(&self, theme_class: ThemeClass) -> bool {
        self.doc
            .visual_diff_cases
            .iter()
            .any(|case| case.theme_class == theme_class)
    }

    /// Returns true when all visual-diff rows cite the manifest appearance session.
    pub fn visual_diff_rows_share_session(&self) -> bool {
        self.doc
            .visual_diff_cases
            .iter()
            .all(|case| case.appearance_session_ref == self.doc.appearance_session_ref)
    }

    /// Returns true when the safety gate requires visual diff and accessibility audit.
    pub fn gate_requires_review(&self, surface_class: &str) -> bool {
        self.doc.safety_critical_change_gates.iter().any(|gate| {
            gate.surface_class == surface_class
                && gate.requires_visual_diff
                && gate.requires_accessibility_audit
        })
    }

    /// Returns true when a motion-reduction row exists for the surface class.
    pub fn has_motion_reduction_for_surface(&self, surface_class: &str) -> bool {
        self.doc
            .motion_reduction_rows
            .iter()
            .any(|row| row.surface_class == surface_class)
    }

    fn validate(&self) -> Result<(), AppearanceAuditError> {
        if self.doc.record_kind != "appearance_visual_diff_alpha_manifest" {
            return Err(AppearanceAuditError::InvalidManifest(format!(
                "unexpected record_kind {}",
                self.doc.record_kind
            )));
        }
        if self.doc.schema_version != 1 {
            return Err(AppearanceAuditError::InvalidManifest(format!(
                "unsupported schema version {}",
                self.doc.schema_version
            )));
        }
        if !self.visual_diff_rows_share_session() {
            return Err(AppearanceAuditError::InvalidManifest(
                "visual-diff rows must cite the manifest appearance session".to_string(),
            ));
        }

        for theme_class in [
            ThemeClass::DarkReference,
            ThemeClass::LightParity,
            ThemeClass::HighContrastDark,
            ThemeClass::HighContrastLight,
        ] {
            if !self.doc.claimed_theme_classes.contains(&theme_class) {
                return Err(AppearanceAuditError::InvalidManifest(format!(
                    "claimed_theme_classes missing {}",
                    theme_class.token()
                )));
            }
            if !self.has_visual_diff_for_theme(theme_class) {
                return Err(AppearanceAuditError::InvalidManifest(format!(
                    "missing visual-diff case for {}",
                    theme_class.token()
                )));
            }
        }
        for density_class in [
            DensityClass::Compact,
            DensityClass::Standard,
            DensityClass::Comfortable,
        ] {
            if !self.doc.claimed_density_classes.contains(&density_class) {
                return Err(AppearanceAuditError::InvalidManifest(format!(
                    "claimed_density_classes missing {}",
                    density_class.token()
                )));
            }
            if !self
                .doc
                .density_audit_rows
                .iter()
                .any(|row| row.density_class == density_class)
            {
                return Err(AppearanceAuditError::InvalidManifest(format!(
                    "missing density audit row for {}",
                    density_class.token()
                )));
            }
        }
        for posture in [
            AccessibilityPostureClass::MotionStandard,
            AccessibilityPostureClass::MotionReduced,
            AccessibilityPostureClass::MotionLowMotion,
            AccessibilityPostureClass::MotionPowerSaver,
        ] {
            if !self.doc.claimed_motion_postures.contains(&posture) {
                return Err(AppearanceAuditError::InvalidManifest(format!(
                    "claimed_motion_postures missing {}",
                    posture.token()
                )));
            }
        }

        let motion_surfaces: HashSet<&str> = self
            .doc
            .motion_reduction_rows
            .iter()
            .map(|row| row.surface_class.as_str())
            .collect();
        for surface in [
            "ai_assist_panel",
            "terminal_panel",
            "list_panel",
            "decorative_shell_chrome",
        ] {
            if !motion_surfaces.contains(surface) {
                return Err(AppearanceAuditError::InvalidManifest(format!(
                    "missing motion-reduction row for {surface}"
                )));
            }
        }
        for row in &self.doc.motion_reduction_rows {
            if !row.critical_attention_cue_preserved {
                return Err(AppearanceAuditError::InvalidManifest(format!(
                    "{} does not preserve critical attention cues",
                    row.row_id
                )));
            }
        }
        for surface in [
            "trust_prompt_canvas",
            "onboarding_start_center",
            "notification_surface",
        ] {
            if !self.gate_requires_review(surface) {
                return Err(AppearanceAuditError::InvalidManifest(format!(
                    "missing safety critical review gate for {surface}"
                )));
            }
        }
        Ok(())
    }
}

/// Returns the canonical alpha appearance audit manifest.
pub fn alpha_appearance_audit_manifest(
) -> Result<&'static AppearanceVisualDiffAuditManifest, AppearanceAuditError> {
    let manifest = ALPHA_AUDIT_MANIFEST.get_or_init(AppearanceVisualDiffAuditManifest::load_alpha);
    match manifest {
        Ok(manifest) => Ok(manifest),
        Err(err) => Err(err.clone()),
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct AppearanceAuditManifestDoc {
    record_kind: String,
    schema_version: u32,
    manifest_id: String,
    appearance_session_ref: String,
    theme_package_manifest_ref: String,
    theme_package_revision_ref: String,
    token_state_registry_ref: String,
    #[serde(default)]
    claimed_theme_classes: Vec<ThemeClass>,
    #[serde(default)]
    claimed_density_classes: Vec<DensityClass>,
    #[serde(default)]
    claimed_motion_postures: Vec<AccessibilityPostureClass>,
    #[serde(default)]
    visual_diff_cases: Vec<VisualDiffCaseDoc>,
    #[serde(default)]
    density_audit_rows: Vec<DensityAuditRowDoc>,
    #[serde(default)]
    motion_reduction_rows: Vec<MotionReductionRowDoc>,
    #[serde(default)]
    safety_critical_change_gates: Vec<SafetyCriticalGateDoc>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct VisualDiffCaseDoc {
    case_id: String,
    surface_class: String,
    theme_class: ThemeClass,
    density_class: DensityClass,
    motion_posture: AccessibilityPostureClass,
    appearance_session_ref: String,
    baseline_ref: String,
    comparison_ref: String,
    accessibility_audit_ref: String,
    semantic_stability_result: String,
    minimum_contrast_result: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct DensityAuditRowDoc {
    row_id: String,
    density_class: DensityClass,
    fixture_ref: String,
    affects_information_architecture: bool,
    affects_focus_visibility: bool,
    affects_state_conveyance: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct MotionReductionRowDoc {
    row_id: String,
    surface_class: String,
    standard_motion_family: String,
    reduced_motion_substitution: String,
    power_saver_substitution: String,
    critical_attention_cue_preserved: bool,
    state_cue_ref: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct SafetyCriticalGateDoc {
    gate_id: String,
    surface_class: String,
    change_class: String,
    requires_visual_diff: bool,
    requires_accessibility_audit: bool,
    evidence_ref: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_audit_manifest_covers_theme_density_motion_and_gates() {
        let manifest = alpha_appearance_audit_manifest().expect("alpha audit manifest");
        assert_eq!(manifest.visual_diff_case_count(), 5);
        assert!(manifest.visual_diff_rows_share_session());
        assert!(manifest.has_visual_diff_for_theme(ThemeClass::LightParity));
        assert!(manifest.has_visual_diff_for_theme(ThemeClass::HighContrastDark));
        assert!(manifest.has_motion_reduction_for_surface("terminal_panel"));
        assert!(manifest.gate_requires_review("trust_prompt_canvas"));
        assert!(manifest.gate_requires_review("notification_surface"));
    }
}
