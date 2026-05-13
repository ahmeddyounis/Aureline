//! Alpha state-semantics registry and visual treatment projection.
//!
//! This module loads the checked-in alpha registry that binds component states,
//! badge families, notice families, and consuming surfaces to semantic tokens.

use std::collections::HashSet;
use std::fmt;
use std::sync::OnceLock;

use serde::Deserialize;

use super::{ColorRgba, TokenRegistry, TokenRegistryError};

const ALPHA_REGISTRY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/design/state_badge_families_alpha.yaml"
));

const ALPHA_FIXTURE_MANIFEST_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/design/m2_state_semantics/manifest.yaml"
));

const REQUIRED_STATE_CLASSES: &[&str] = &[
    "empty",
    "loading",
    "pending",
    "degraded",
    "blocked",
    "error",
    "completed",
    "focus_visible",
    "selection",
    "active_target",
    "trust_restricted",
    "policy_locked",
    "readiness_ready",
    "readiness_partial",
];

const REQUIRED_BADGE_FAMILIES: &[&str] = &[
    "lifecycle",
    "route",
    "support_class",
    "readiness",
    "policy",
    "trust",
    "docs_help",
    "package_marketplace",
    "support_export",
    "theme_package",
];

const REQUIRED_NOTICE_FAMILIES: &[&str] = &[
    "info",
    "warning",
    "degraded",
    "blocked",
    "restricted",
    "success",
];

const REQUIRED_SURFACES: &[&str] = &[
    "shell_chrome",
    "editor_canvas",
    "command_palette_and_search",
    "docs_help_canvas",
    "package_marketplace_canvas",
    "support_export",
    "trust_prompt_canvas",
    "extension_embedded_canvas",
];

static ALPHA_STATE_SEMANTICS: OnceLock<Result<StateSemanticsRegistry, StateSemanticsError>> =
    OnceLock::new();

/// Errors emitted while loading or projecting the alpha state-semantics registry.
#[derive(Debug, Clone)]
pub enum StateSemanticsError {
    /// The alpha registry YAML did not parse.
    RegistryParseFailed(String),
    /// The protected fixture manifest YAML did not parse.
    FixtureManifestParseFailed(String),
    /// The registry is structurally invalid.
    InvalidRegistry(String),
    /// A component-state class was not present in the registry.
    MissingStateClass(String),
    /// A badge family was not present in the registry.
    MissingBadgeFamily(String),
    /// A badge token was not present in its family.
    MissingBadgeToken {
        /// Badge-family class that was queried.
        family_class: String,
        /// Badge token that was queried.
        token: String,
    },
    /// A notice family was not present in the registry.
    MissingNoticeFamily(String),
    /// A semantic token could not be resolved through the token registry.
    TokenRegistry(TokenRegistryError),
}

impl fmt::Display for StateSemanticsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RegistryParseFailed(detail) => {
                write!(
                    f,
                    "failed to parse alpha state semantics registry: {detail}"
                )
            }
            Self::FixtureManifestParseFailed(detail) => {
                write!(
                    f,
                    "failed to parse alpha state semantics fixture manifest: {detail}"
                )
            }
            Self::InvalidRegistry(detail) => write!(f, "invalid alpha state registry: {detail}"),
            Self::MissingStateClass(state_class) => {
                write!(f, "missing component state class: {state_class}")
            }
            Self::MissingBadgeFamily(family_class) => {
                write!(f, "missing badge family: {family_class}")
            }
            Self::MissingBadgeToken {
                family_class,
                token,
            } => write!(f, "missing badge token {token} in family {family_class}"),
            Self::MissingNoticeFamily(family_class) => {
                write!(f, "missing notice family: {family_class}")
            }
            Self::TokenRegistry(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for StateSemanticsError {}

impl From<TokenRegistryError> for StateSemanticsError {
    fn from(value: TokenRegistryError) -> Self {
        Self::TokenRegistry(value)
    }
}

/// Token-backed visual treatment for a state, badge token, or notice family.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemanticVisualTreatment {
    /// Human-visible label that surfaces render near the cue.
    pub label: String,
    /// Assistive-technology label for the same semantic state.
    pub screen_reader_label: String,
    /// Stable icon metaphor name selected by the registry.
    pub icon: String,
    /// Stable shape or structural cue selected by the registry.
    pub shape: String,
    /// Foreground semantic color resolved for the active theme.
    pub foreground: ColorRgba,
    /// Border semantic color resolved for the active theme.
    pub border: ColorRgba,
    /// Fill semantic color resolved for the active theme.
    pub fill: ColorRgba,
    /// Non-color cues required to preserve meaning in high-contrast and forced-colors modes.
    pub required_non_color_cues: Vec<String>,
    /// Whether the state must remain visible without hover or tooltip-only disclosure.
    pub persistent_disclosure_required: bool,
}

/// Loaded alpha state-semantics registry plus its protected fixture manifest.
#[derive(Debug, Clone)]
pub struct StateSemanticsRegistry {
    doc: StateBadgeRegistryDoc,
    fixtures: FixtureManifestDoc,
}

impl StateSemanticsRegistry {
    /// Loads and validates the embedded alpha state-semantics registry.
    pub fn load_alpha() -> Result<Self, StateSemanticsError> {
        let doc: StateBadgeRegistryDoc = serde_yaml::from_str(ALPHA_REGISTRY_YAML)
            .map_err(|err| StateSemanticsError::RegistryParseFailed(err.to_string()))?;
        let fixtures: FixtureManifestDoc = serde_yaml::from_str(ALPHA_FIXTURE_MANIFEST_YAML)
            .map_err(|err| StateSemanticsError::FixtureManifestParseFailed(err.to_string()))?;
        let registry = Self { doc, fixtures };
        registry.validate()?;
        Ok(registry)
    }

    /// Returns the stable registry identifier.
    pub fn registry_id(&self) -> &str {
        &self.doc.registry_id
    }

    /// Returns the number of component-state families in the registry.
    pub fn component_state_family_count(&self) -> usize {
        self.doc.component_state_families.len()
    }

    /// Returns the number of badge families in the registry.
    pub fn badge_family_count(&self) -> usize {
        self.doc.badge_families.len()
    }

    /// Returns the number of protected fixture cases bound to this registry.
    pub fn fixture_case_count(&self) -> usize {
        self.fixtures.fixture_cases.len()
    }

    /// Returns `true` when the registry contains a component-state class.
    pub fn contains_state_class(&self, state_class: &str) -> bool {
        self.state_family(state_class).is_some()
    }

    /// Returns `true` when the registry contains a badge-family class.
    pub fn contains_badge_family(&self, family_class: &str) -> bool {
        self.badge_family(family_class).is_some()
    }

    /// Returns `true` when the registry contains a notice-family class.
    pub fn contains_notice_family(&self, family_class: &str) -> bool {
        self.notice_family(family_class).is_some()
    }

    /// Returns whether a surface declares a partial inheritance gap posture.
    pub fn declares_inheritance_gap(&self, surface_class: &str) -> bool {
        self.surface(surface_class)
            .map(|surface| {
                surface.inheritance_posture == "declares_inheritance_gap_when_not_aligned"
            })
            .unwrap_or(false)
    }

    /// Returns whether a surface consumes the given component-state class.
    pub fn surface_consumes_state(&self, surface_class: &str, state_class: &str) -> bool {
        self.surface(surface_class)
            .map(|surface| {
                surface
                    .consumes_component_state_classes
                    .iter()
                    .any(|item| item == state_class)
            })
            .unwrap_or(false)
    }

    /// Resolves a component-state treatment against a semantic token registry.
    pub fn state_treatment(
        &self,
        tokens: &TokenRegistry,
        state_class: &str,
    ) -> Result<SemanticVisualTreatment, StateSemanticsError> {
        let row = self
            .state_family(state_class)
            .ok_or_else(|| StateSemanticsError::MissingStateClass(state_class.to_owned()))?;
        treatment_from_parts(
            tokens,
            &row.display_label,
            &row.screen_reader_label,
            &row.icon,
            &row.shape,
            &row.token_refs,
            &row.required_non_color_cues,
            row.persistent_disclosure_required,
        )
    }

    /// Resolves a badge-token treatment against a semantic token registry.
    pub fn badge_treatment(
        &self,
        tokens: &TokenRegistry,
        family_class: &str,
        token: &str,
    ) -> Result<SemanticVisualTreatment, StateSemanticsError> {
        let family = self
            .badge_family(family_class)
            .ok_or_else(|| StateSemanticsError::MissingBadgeFamily(family_class.to_owned()))?;
        let badge_token = family
            .vocabulary_tokens
            .iter()
            .find(|candidate| candidate.token == token)
            .ok_or_else(|| StateSemanticsError::MissingBadgeToken {
                family_class: family_class.to_owned(),
                token: token.to_owned(),
            })?;
        treatment_from_parts(
            tokens,
            &badge_token.label,
            &badge_token.screen_reader_label,
            &badge_token.icon,
            &badge_token.shape,
            &badge_token.token_refs,
            &family.required_non_color_cues,
            true,
        )
    }

    /// Resolves a notice-family treatment against a semantic token registry.
    pub fn notice_treatment(
        &self,
        tokens: &TokenRegistry,
        family_class: &str,
    ) -> Result<SemanticVisualTreatment, StateSemanticsError> {
        let family = self
            .notice_family(family_class)
            .ok_or_else(|| StateSemanticsError::MissingNoticeFamily(family_class.to_owned()))?;
        treatment_from_parts(
            tokens,
            &family.title,
            &family.title,
            &family.icon,
            &family.shape,
            &family.token_refs,
            &family.required_non_color_cues,
            family.persistent_disclosure_required,
        )
    }

    fn validate(&self) -> Result<(), StateSemanticsError> {
        if self.doc.record_kind != "state_badge_family_alpha_registry" {
            return Err(StateSemanticsError::InvalidRegistry(format!(
                "unexpected record_kind {}",
                self.doc.record_kind
            )));
        }
        if self.doc.state_badge_family_schema_version != 1 {
            return Err(StateSemanticsError::InvalidRegistry(format!(
                "unsupported schema version {}",
                self.doc.state_badge_family_schema_version
            )));
        }
        if self.fixtures.record_kind != "state_semantics_fixture_manifest" {
            return Err(StateSemanticsError::InvalidRegistry(format!(
                "unexpected fixture manifest kind {}",
                self.fixtures.record_kind
            )));
        }

        let state_classes = collect_unique(
            self.doc
                .component_state_families
                .iter()
                .map(|row| row.state_class.as_str()),
            "component state classes",
        )?;
        require_all(
            &state_classes,
            REQUIRED_STATE_CLASSES,
            "component state classes",
        )?;

        for row in &self.doc.component_state_families {
            require_non_color_cues(&row.required_non_color_cues, &row.state_class)?;
            if row.hue_only_allowed {
                return Err(StateSemanticsError::InvalidRegistry(format!(
                    "{} cannot permit hue-only state signaling",
                    row.state_class
                )));
            }
            validate_token_refs(&row.token_refs, &row.state_class)?;
        }

        let badge_families = collect_unique(
            self.doc
                .badge_families
                .iter()
                .map(|row| row.badge_family_class.as_str()),
            "badge families",
        )?;
        require_all(&badge_families, REQUIRED_BADGE_FAMILIES, "badge families")?;

        for family in &self.doc.badge_families {
            if family.vocabulary_tokens.is_empty() {
                return Err(StateSemanticsError::InvalidRegistry(format!(
                    "{} has no vocabulary tokens",
                    family.badge_family_class
                )));
            }
            require_non_color_cues(&family.required_non_color_cues, &family.badge_family_class)?;
            if family.requires_text_shape_fallback
                && !(family
                    .required_non_color_cues
                    .iter()
                    .any(|cue| cue == "label_text")
                    && family
                        .required_non_color_cues
                        .iter()
                        .any(|cue| cue == "shape"))
            {
                return Err(StateSemanticsError::InvalidRegistry(format!(
                    "{} must include label_text and shape fallbacks",
                    family.badge_family_class
                )));
            }
            if !family
                .vocabulary_tokens
                .iter()
                .any(|token| token.token == family.honesty_fallback_token)
            {
                return Err(StateSemanticsError::InvalidRegistry(format!(
                    "{} fallback token {} is not in the vocabulary",
                    family.badge_family_class, family.honesty_fallback_token
                )));
            }
            for token in &family.vocabulary_tokens {
                validate_token_refs(
                    &token.token_refs,
                    &format!("{}.{}", family.badge_family_class, token.token),
                )?;
                if !state_classes.contains(token.state_class_ref.as_str()) {
                    return Err(StateSemanticsError::InvalidRegistry(format!(
                        "{}.{} references unknown state class {}",
                        family.badge_family_class, token.token, token.state_class_ref
                    )));
                }
            }
        }

        let notice_families = collect_unique(
            self.doc
                .notice_families
                .iter()
                .map(|row| row.notice_family_class.as_str()),
            "notice families",
        )?;
        require_all(
            &notice_families,
            REQUIRED_NOTICE_FAMILIES,
            "notice families",
        )?;
        for family in &self.doc.notice_families {
            require_non_color_cues(&family.required_non_color_cues, &family.notice_family_class)?;
            validate_token_refs(&family.token_refs, &family.notice_family_class)?;
            if !state_classes.contains(family.state_class_ref.as_str()) {
                return Err(StateSemanticsError::InvalidRegistry(format!(
                    "{} references unknown state class {}",
                    family.notice_family_class, family.state_class_ref
                )));
            }
        }

        let surfaces = collect_unique(
            self.doc
                .surface_consumers
                .iter()
                .map(|row| row.surface_class.as_str()),
            "surface consumers",
        )?;
        require_all(&surfaces, REQUIRED_SURFACES, "surface consumers")?;
        for surface in &self.doc.surface_consumers {
            for state_class in &surface.consumes_component_state_classes {
                if !state_classes.contains(state_class.as_str()) {
                    return Err(StateSemanticsError::InvalidRegistry(format!(
                        "{} consumes unknown state class {}",
                        surface.surface_class, state_class
                    )));
                }
            }
            for family_class in &surface.consumes_badge_family_classes {
                if !badge_families.contains(family_class.as_str()) {
                    return Err(StateSemanticsError::InvalidRegistry(format!(
                        "{} consumes unknown badge family {}",
                        surface.surface_class, family_class
                    )));
                }
            }
            for family_class in &surface.consumes_notice_family_classes {
                if !notice_families.contains(family_class.as_str()) {
                    return Err(StateSemanticsError::InvalidRegistry(format!(
                        "{} consumes unknown notice family {}",
                        surface.surface_class, family_class
                    )));
                }
            }
        }

        let fixture_cases = collect_unique(
            self.fixtures
                .fixture_cases
                .iter()
                .map(|row| row.case_id.as_str()),
            "fixture cases",
        )?;
        for required_case in &self.fixtures.required_case_ids {
            if !fixture_cases.contains(required_case.as_str()) {
                return Err(StateSemanticsError::InvalidRegistry(format!(
                    "fixture manifest is missing required case {required_case}"
                )));
            }
        }

        Ok(())
    }

    fn state_family(&self, state_class: &str) -> Option<&ComponentStateFamilyRow> {
        self.doc
            .component_state_families
            .iter()
            .find(|row| row.state_class == state_class)
    }

    fn badge_family(&self, family_class: &str) -> Option<&BadgeFamilyRow> {
        self.doc
            .badge_families
            .iter()
            .find(|row| row.badge_family_class == family_class)
    }

    fn notice_family(&self, family_class: &str) -> Option<&NoticeFamilyRow> {
        self.doc
            .notice_families
            .iter()
            .find(|row| row.notice_family_class == family_class)
    }

    fn surface(&self, surface_class: &str) -> Option<&SurfaceConsumerRow> {
        self.doc
            .surface_consumers
            .iter()
            .find(|row| row.surface_class == surface_class)
    }
}

/// Returns the canonical alpha state-semantics registry.
pub fn alpha_state_semantics_registry(
) -> Result<&'static StateSemanticsRegistry, StateSemanticsError> {
    let registry = ALPHA_STATE_SEMANTICS.get_or_init(StateSemanticsRegistry::load_alpha);
    match registry {
        Ok(registry) => Ok(registry),
        Err(err) => Err(err.clone()),
    }
}

fn treatment_from_parts(
    tokens: &TokenRegistry,
    label: &str,
    screen_reader_label: &str,
    icon: &str,
    shape: &str,
    refs: &TokenRefs,
    required_non_color_cues: &[String],
    persistent_disclosure_required: bool,
) -> Result<SemanticVisualTreatment, StateSemanticsError> {
    Ok(SemanticVisualTreatment {
        label: label.to_owned(),
        screen_reader_label: screen_reader_label.to_owned(),
        icon: icon.to_owned(),
        shape: shape.to_owned(),
        foreground: tokens.require_color(&refs.foreground)?,
        border: tokens.require_color(&refs.border)?,
        fill: tokens.require_color(&refs.fill)?,
        required_non_color_cues: required_non_color_cues.to_vec(),
        persistent_disclosure_required,
    })
}

fn collect_unique<'a>(
    values: impl Iterator<Item = &'a str>,
    label: &str,
) -> Result<HashSet<&'a str>, StateSemanticsError> {
    let mut seen = HashSet::new();
    for value in values {
        if !seen.insert(value) {
            return Err(StateSemanticsError::InvalidRegistry(format!(
                "duplicate {label}: {value}"
            )));
        }
    }
    Ok(seen)
}

fn require_all(
    actual: &HashSet<&str>,
    required: &[&str],
    label: &str,
) -> Result<(), StateSemanticsError> {
    for value in required {
        if !actual.contains(value) {
            return Err(StateSemanticsError::InvalidRegistry(format!(
                "missing required {label}: {value}"
            )));
        }
    }
    Ok(())
}

fn require_non_color_cues(cues: &[String], row_id: &str) -> Result<(), StateSemanticsError> {
    if cues.is_empty() {
        return Err(StateSemanticsError::InvalidRegistry(format!(
            "{row_id} has no non-color cues"
        )));
    }
    Ok(())
}

fn validate_token_refs(refs: &TokenRefs, row_id: &str) -> Result<(), StateSemanticsError> {
    for (slot, value) in [
        ("foreground", refs.foreground.as_str()),
        ("border", refs.border.as_str()),
        ("fill", refs.fill.as_str()),
    ] {
        if value.trim().is_empty() {
            return Err(StateSemanticsError::InvalidRegistry(format!(
                "{row_id} token_refs.{slot} is empty"
            )));
        }
    }
    Ok(())
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct StateBadgeRegistryDoc {
    record_kind: String,
    state_badge_family_schema_version: u32,
    registry_id: String,
    #[serde(default)]
    component_state_families: Vec<ComponentStateFamilyRow>,
    #[serde(default)]
    badge_families: Vec<BadgeFamilyRow>,
    #[serde(default)]
    notice_families: Vec<NoticeFamilyRow>,
    #[serde(default)]
    surface_consumers: Vec<SurfaceConsumerRow>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct ComponentStateFamilyRow {
    state_class: String,
    display_label: String,
    #[serde(default)]
    taxonomy_state_classes: Vec<String>,
    semantic_kind: String,
    token_refs: TokenRefs,
    icon: String,
    shape: String,
    #[serde(default)]
    required_non_color_cues: Vec<String>,
    #[serde(default)]
    hue_only_allowed: bool,
    #[serde(default)]
    persistent_disclosure_required: bool,
    screen_reader_label: String,
    #[serde(default)]
    distinct_from: Vec<String>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct BadgeFamilyRow {
    badge_family_class: String,
    #[serde(default)]
    consumes_surface_classes: Vec<String>,
    #[serde(default)]
    support_export_compatible: bool,
    honesty_fallback_token: String,
    #[serde(default)]
    required_non_color_cues: Vec<String>,
    #[serde(default)]
    requires_text_shape_fallback: bool,
    #[serde(default)]
    vocabulary_tokens: Vec<BadgeTokenRow>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct BadgeTokenRow {
    token: String,
    label: String,
    state_class_ref: String,
    token_refs: TokenRefs,
    icon: String,
    shape: String,
    screen_reader_label: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct NoticeFamilyRow {
    notice_family_class: String,
    title: String,
    state_class_ref: String,
    token_refs: TokenRefs,
    #[serde(default)]
    required_non_color_cues: Vec<String>,
    icon: String,
    shape: String,
    #[serde(default)]
    persistent_disclosure_required: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct SurfaceConsumerRow {
    surface_class: String,
    #[serde(default)]
    consumes_component_state_classes: Vec<String>,
    #[serde(default)]
    consumes_badge_family_classes: Vec<String>,
    #[serde(default)]
    consumes_notice_family_classes: Vec<String>,
    inheritance_posture: String,
}

#[derive(Debug, Clone, Deserialize)]
struct TokenRefs {
    foreground: String,
    border: String,
    fill: String,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct FixtureManifestDoc {
    record_kind: String,
    #[serde(default)]
    required_case_ids: Vec<String>,
    #[serde(default)]
    fixture_cases: Vec<FixtureCaseRow>,
}

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
struct FixtureCaseRow {
    case_id: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tokens::{seeded_token_registry, ThemeClass};

    #[test]
    fn loads_alpha_registry_with_required_surfaces() {
        let registry = StateSemanticsRegistry::load_alpha().expect("alpha semantics registry");
        assert_eq!(registry.registry_id(), "state_badge_families.alpha.01");
        assert!(registry.component_state_family_count() >= REQUIRED_STATE_CLASSES.len());
        assert!(registry.badge_family_count() >= REQUIRED_BADGE_FAMILIES.len());
        assert!(registry.fixture_case_count() >= 6);
        assert!(registry.surface_consumes_state("shell_chrome", "policy_locked"));
        assert!(registry.surface_consumes_state("command_palette_and_search", "pending"));
    }

    #[test]
    fn resolves_loading_pending_and_blocked_treatments() {
        let registry = alpha_state_semantics_registry().expect("alpha semantics registry");
        let tokens = seeded_token_registry(ThemeClass::DarkReference).expect("seeded tokens");

        let loading = registry
            .state_treatment(tokens, "loading")
            .expect("loading treatment");
        let pending = registry
            .state_treatment(tokens, "pending")
            .expect("pending treatment");
        let blocked = registry
            .state_treatment(tokens, "blocked")
            .expect("blocked treatment");

        assert_ne!(loading.fill, pending.fill);
        assert!(loading
            .required_non_color_cues
            .iter()
            .any(|cue| cue == "progress_indicator"));
        assert!(pending
            .required_non_color_cues
            .iter()
            .any(|cue| cue == "icon"));
        assert_eq!(blocked.shape, "blocked_badge_with_bar");
    }

    #[test]
    fn resolves_cross_surface_badge_and_notice_treatments() {
        let registry = alpha_state_semantics_registry().expect("alpha semantics registry");
        let tokens = seeded_token_registry(ThemeClass::LightParity).expect("seeded tokens");

        let docs_stale = registry
            .badge_treatment(tokens, "docs_help", "stale")
            .expect("docs stale badge");
        let package_blocked = registry
            .badge_treatment(tokens, "package_marketplace", "install_blocked")
            .expect("package blocked badge");
        let restricted_notice = registry
            .notice_treatment(tokens, "restricted")
            .expect("restricted notice");

        assert_eq!(docs_stale.label, "Stale");
        assert_eq!(package_blocked.screen_reader_label, "Install blocked");
        assert!(restricted_notice.persistent_disclosure_required);
    }

    #[test]
    fn extension_surface_declares_inheritance_gap() {
        let registry = alpha_state_semantics_registry().expect("alpha semantics registry");
        assert!(registry.declares_inheritance_gap("extension_embedded_canvas"));
        assert!(registry.contains_badge_family("theme_package"));
        assert!(registry.contains_notice_family("blocked"));
    }
}
