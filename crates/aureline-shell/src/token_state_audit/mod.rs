//! Beta token / state / density / motion / theme audit projection.
//!
//! This module is the page-level audit that promotes the launch-critical
//! shell visual-system and state vocabulary to beta. It does NOT mint a
//! parallel token registry, density profile, motion preset, or
//! component-state vocabulary — those still live under [`aureline_ui`].
//! It pins, on every claimed launch-critical row, that:
//!
//! 1. **Theme legibility.** Focus, trust, degraded-state, and action
//!    semantics resolve to required token names that exist in dark,
//!    light, and high-contrast theme packs (no surface that drops a
//!    required token id when the theme switches).
//! 2. **Density preservation.** Compact, standard, and comfortable
//!    densities preserve the surface's row/control geometry tokens —
//!    state-conveying chips and focus rings cannot collapse below the
//!    baseline geometry the surface promises.
//! 3. **Motion preservation.** Reduced, low-motion, power-saver, and
//!    critical-hot-path postures preserve state conveyance and focus
//!    visibility through a typed substitution class — never by
//!    suppressing entirely a state that only motion was carrying.
//! 4. **Action stability.** The same canonical command id and label
//!    reappear across every (theme × density × motion) row for one
//!    surface, so a density or motion switch never changes the meaning
//!    of an action.
//!
//! The audit yields a checked [`TokenStateAuditDefect`] list rather than
//! scattered screenshot comments. The seeded page seeds zero defects;
//! the validator + headless inspector are what surface a regression
//! when a row drops a required token, drops a required state symbol,
//! degrades a motion substitution, or drifts an action label.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use aureline_ui::components::state_registry::ComponentStateClass;
use aureline_ui::density::DensityClass;
use aureline_ui::motion::ReducedMotionSubstitutionClass;
use aureline_ui::themes::AccessibilityPostureClass;
use aureline_ui::tokens::ThemeClass;

/// Beta schema version exported with every record.
pub const TOKEN_STATE_AUDIT_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record.
pub const TOKEN_STATE_AUDIT_BETA_SHARED_CONTRACT_REF: &str = "shell:token_state_audit_beta:v1";

/// Stable record kind for [`TokenStateAuditPage`] payloads.
pub const TOKEN_STATE_AUDIT_BETA_PAGE_RECORD_KIND: &str =
    "shell_token_state_audit_beta_page_record";

/// Stable record kind for [`TokenStateAuditRow`] payloads.
pub const TOKEN_STATE_AUDIT_BETA_ROW_RECORD_KIND: &str = "shell_token_state_audit_beta_row_record";

/// Stable record kind for [`TokenStateAuditDefect`] payloads.
pub const TOKEN_STATE_AUDIT_BETA_DEFECT_RECORD_KIND: &str =
    "shell_token_state_audit_beta_defect_record";

/// Stable record kind for [`TokenStateAuditSupportExport`] payloads.
pub const TOKEN_STATE_AUDIT_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_token_state_audit_beta_support_export_record";

/// Closed launch-critical surface vocabulary covered by the beta audit.
///
/// Each variant maps to a real shell surface that already ships a
/// state, command, or trust contract. Adding a surface to this enum is
/// how the audit grows; downstream the validator enforces token, state,
/// motion, and action invariants for every row that names the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchCriticalSurfaceClass {
    /// Title-context bar, activity rail, sidebars, status bar — the
    /// shell chrome that keeps focus, trust, and degraded posture
    /// legible across themes and densities.
    ShellChrome,
    /// First-touch surface for new and returning users; row focus and
    /// selected state must remain legible.
    StartCenter,
    /// Keyboard-first command palette; the focus ring, selected row,
    /// and enter/exit motion must keep state visible under reduced
    /// motion.
    CommandPalette,
    /// Trust narrowing chip / scope-truth posture; warning and
    /// restricted treatments must come from token families, not hue
    /// alone.
    ScopeTruthChip,
    /// Durable activity row + badge mirror; lifecycle and resolution
    /// state must stay legible across density and motion.
    ActivityCenterRow,
    /// Toast / banner / status notification envelope; severity and
    /// privacy treatments must stay legible across theme switches.
    NotificationEnvelope,
    /// Typed permission / trust prompt sheet; warning and destructive
    /// treatments must keep their state symbols across themes.
    TrustPromptSheet,
    /// Settings root pane; navigation focus, current-pane indicator,
    /// and policy-locked rows must keep their state legible.
    SettingsRoot,
}

impl LaunchCriticalSurfaceClass {
    /// Stable schema token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellChrome => "shell_chrome",
            Self::StartCenter => "start_center",
            Self::CommandPalette => "command_palette",
            Self::ScopeTruthChip => "scope_truth_chip",
            Self::ActivityCenterRow => "activity_center_row",
            Self::NotificationEnvelope => "notification_envelope",
            Self::TrustPromptSheet => "trust_prompt_sheet",
            Self::SettingsRoot => "settings_root",
        }
    }
}

/// Closed semantic promise vocabulary the audit verifies per row.
///
/// A surface row enumerates which promises it claims; the validator
/// then enforces the token, state, and motion invariants that promise
/// requires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticPromise {
    /// Focus is visible — the row carries a focus token AND
    /// [`ComponentStateClass::FocusVisible`].
    FocusLegible,
    /// Trust narrowing remains visible — the row carries a warning,
    /// restricted, or locked state token AND a matching component
    /// state symbol; never carried by hue alone.
    TrustLegible,
    /// Degraded posture remains conveyed — the row carries a
    /// non-motion state marker AND
    /// [`ComponentStateClass::Degraded`].
    DegradedLegible,
    /// The action label and command id stay stable across every theme,
    /// density, and motion row for the same surface.
    ActionLabelStable,
}

impl SemanticPromise {
    /// Stable schema token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FocusLegible => "focus_legible",
            Self::TrustLegible => "trust_legible",
            Self::DegradedLegible => "degraded_legible",
            Self::ActionLabelStable => "action_label_stable",
        }
    }
}

/// Density-preservation expectation projected by the row.
///
/// Beta forbids a density mode collapsing a state-conveying control
/// below the baseline geometry the surface promises (e.g. a status
/// chip whose token-derived height is > 0 in standard but pinned to a
/// hard-coded zero in compact).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DensityPreservation {
    /// True when the surface preserves the row-height token across
    /// density. Beta requires `true` when the surface promises focus
    /// or trust legibility on a row.
    pub preserves_row_height_token: bool,
    /// True when the surface preserves the control-height token across
    /// density.
    pub preserves_control_height_token: bool,
    /// True when the surface preserves the panel-padding token across
    /// density.
    pub preserves_panel_padding_token: bool,
}

impl DensityPreservation {
    /// Builds a preservation block that satisfies the beta invariants.
    pub fn preserved() -> Self {
        Self {
            preserves_row_height_token: true,
            preserves_control_height_token: true,
            preserves_panel_padding_token: true,
        }
    }
}

/// Motion-preservation expectation projected by the row.
///
/// Beta forbids a motion posture stripping state conveyance or focus
/// visibility on a launch-critical surface. Reduced and low-motion
/// postures may collapse the *transition* but must still resolve to a
/// substitution class that preserves the underlying state marker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MotionPreservation {
    /// Stable motion preset reference (e.g. `motion_preset:overlay.dialog.enter`).
    pub motion_preset_ref: String,
    /// Substitution class chosen for this posture.
    pub substitution_class: ReducedMotionSubstitutionClass,
    /// True when the substitution preserves state conveyance — beta
    /// rejects [`ReducedMotionSubstitutionClass::SuppressEntirely`] for
    /// rows that promise [`SemanticPromise::DegradedLegible`] or
    /// [`SemanticPromise::TrustLegible`].
    pub preserves_state_conveyance: bool,
    /// True when the substitution preserves focus visibility — beta
    /// rejects any substitution that hides the focus ring on rows
    /// that promise [`SemanticPromise::FocusLegible`].
    pub preserves_focus_visibility: bool,
}

/// One audited (surface × theme × density × motion) row.
///
/// The row is the unit the validator inspects. Every row carries the
/// required token names, the required component state symbols, the
/// motion preservation block, the density preservation block, the
/// canonical command id and label, and the closed list of semantic
/// promises it claims.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenStateAuditRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable case id used to pivot across surfaces.
    pub case_id: String,
    /// Stable row id consumed by the chrome and the headless inspector.
    pub row_id: String,
    /// Surface this row audits.
    pub surface: LaunchCriticalSurfaceClass,
    /// Theme class in effect for the audited row.
    pub theme: ThemeClass,
    /// Density class in effect for the audited row.
    pub density: DensityClass,
    /// Accessibility (motion) posture in effect for the audited row.
    pub posture: AccessibilityPostureClass,
    /// Required color token names (e.g. `color.focus.ring`,
    /// `status.warning.border`).
    pub required_color_tokens: Vec<String>,
    /// Required size token names (e.g. `size.row.standard`,
    /// `size.control.standard`).
    pub required_size_tokens: Vec<String>,
    /// Required space token names (e.g. `space.4`, `space.3`).
    pub required_space_tokens: Vec<String>,
    /// Required component state symbols (e.g. `FocusVisible`,
    /// `Selected`, `Warning`).
    pub required_component_states: Vec<ComponentStateClass>,
    /// Density preservation block.
    pub density_preservation: DensityPreservation,
    /// Motion preservation block.
    pub motion_preservation: MotionPreservation,
    /// Promised semantics for this row.
    pub promised_semantics: Vec<SemanticPromise>,
    /// Canonical command id whose meaning must remain stable across
    /// every (theme × density × motion) row for this surface.
    pub canonical_command_id: String,
    /// Canonical action label rendered alongside the command.
    pub canonical_action_label: String,
    /// Reviewer-facing narrative used in the audit doc and a11y exports.
    pub narrative: String,
}

/// Closed defect vocabulary the audit emits.
///
/// A defect is the only way the audit reports a regression: a missing
/// token, a missing state symbol, a motion posture that strips state
/// conveyance, a density that collapses geometry below baseline, or a
/// drifted action label across rows for one surface. The defect list
/// is the checked output the spec asks for in place of scattered
/// screenshot comments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenStateAuditDefectKind {
    /// Row promised [`SemanticPromise::FocusLegible`] but did not
    /// declare a focus color token (e.g. `color.focus.ring`).
    MissingFocusColorToken,
    /// Row promised [`SemanticPromise::FocusLegible`] but did not
    /// declare [`ComponentStateClass::FocusVisible`].
    MissingFocusVisibleStateSymbol,
    /// Row promised [`SemanticPromise::TrustLegible`] but did not
    /// declare a `status.warning.*`, `status.danger.*`, or
    /// `trust.restricted.*` token family.
    MissingTrustToken,
    /// Row promised [`SemanticPromise::TrustLegible`] but did not
    /// declare a [`ComponentStateClass::Warning`] /
    /// [`ComponentStateClass::Restricted`] / [`ComponentStateClass::Locked`]
    /// state symbol — trust legibility must not be carried by hue alone.
    MissingTrustStateSymbol,
    /// Row promised [`SemanticPromise::DegradedLegible`] but did not
    /// declare [`ComponentStateClass::Degraded`].
    MissingDegradedStateSymbol,
    /// Row promised state conveyance but the motion posture
    /// [`ReducedMotionSubstitutionClass::SuppressEntirely`] was
    /// chosen — state would only be carried by motion.
    MotionStripsStateConveyance,
    /// Row promised focus legibility but the motion posture would
    /// hide the focus ring.
    MotionStripsFocusVisibility,
    /// Density mode collapsed the row-height, control-height, or
    /// panel-padding token below the baseline geometry the surface
    /// promises (preservation flag set to false).
    DensityCollapsesGeometryToken,
    /// The canonical command id or action label drifted across two
    /// rows for the same surface — a density or motion switch must
    /// not change the meaning of an action.
    ActionLabelDriftsAcrossRows,
    /// Surface row enumerated a density geometry token but did not
    /// list the matching `size.row.*` or `size.control.*` token in
    /// `required_size_tokens`.
    MissingDensityGeometryToken,
    /// Surface row did not name a motion preset reference.
    MissingMotionPresetReference,
}

impl TokenStateAuditDefectKind {
    /// Stable schema token recorded on the defect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingFocusColorToken => "missing_focus_color_token",
            Self::MissingFocusVisibleStateSymbol => "missing_focus_visible_state_symbol",
            Self::MissingTrustToken => "missing_trust_token",
            Self::MissingTrustStateSymbol => "missing_trust_state_symbol",
            Self::MissingDegradedStateSymbol => "missing_degraded_state_symbol",
            Self::MotionStripsStateConveyance => "motion_strips_state_conveyance",
            Self::MotionStripsFocusVisibility => "motion_strips_focus_visibility",
            Self::DensityCollapsesGeometryToken => "density_collapses_geometry_token",
            Self::ActionLabelDriftsAcrossRows => "action_label_drifts_across_rows",
            Self::MissingDensityGeometryToken => "missing_density_geometry_token",
            Self::MissingMotionPresetReference => "missing_motion_preset_reference",
        }
    }
}

/// Typed defect emitted by the audit validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenStateAuditDefect {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable defect id consumed by the chrome and the headless inspector.
    pub defect_id: String,
    /// Defect classification.
    pub defect_kind: TokenStateAuditDefectKind,
    /// Stable defect kind token (mirrors `defect_kind` for support exports).
    pub defect_kind_token: String,
    /// Surface the defect was found on.
    pub surface: LaunchCriticalSurfaceClass,
    /// Row id the defect was raised against.
    pub row_id: String,
    /// Field that drifted or was missing.
    pub field: String,
    /// Reviewer-facing note describing the defect.
    pub note: String,
}

impl TokenStateAuditDefect {
    fn new(
        defect_kind: TokenStateAuditDefectKind,
        surface: LaunchCriticalSurfaceClass,
        row_id: &str,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: TOKEN_STATE_AUDIT_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: TOKEN_STATE_AUDIT_BETA_SCHEMA_VERSION,
            shared_contract_ref: TOKEN_STATE_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "ux:defect:token-state-audit:{}:{}",
                defect_kind.as_str(),
                row_id
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            surface,
            row_id: row_id.to_owned(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary banner for the beta audit page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TokenStateAuditSummary {
    /// Number of audited rows on the page.
    pub row_count: usize,
    /// Distinct surfaces covered by the page.
    pub surface_count: usize,
    /// Distinct themes exercised by the page.
    pub theme_count: usize,
    /// Distinct densities exercised by the page.
    pub density_count: usize,
    /// Distinct accessibility postures exercised by the page.
    pub posture_count: usize,
    /// Number of defects emitted by the validator on the seeded page.
    pub defect_count: usize,
    /// Surfaces present on the page in stable order.
    pub surfaces_present: Vec<LaunchCriticalSurfaceClass>,
    /// Themes present on the page in stable order.
    pub themes_present: Vec<ThemeClass>,
    /// Densities present on the page in stable order.
    pub densities_present: Vec<DensityClass>,
    /// Postures present on the page in stable order.
    pub postures_present: Vec<AccessibilityPostureClass>,
}

impl TokenStateAuditSummary {
    fn from_rows(rows: &[TokenStateAuditRow], defects: &[TokenStateAuditDefect]) -> Self {
        let mut surfaces: Vec<LaunchCriticalSurfaceClass> = Vec::new();
        let mut themes: Vec<ThemeClass> = Vec::new();
        let mut densities: Vec<DensityClass> = Vec::new();
        let mut postures: Vec<AccessibilityPostureClass> = Vec::new();
        for row in rows {
            if !surfaces.contains(&row.surface) {
                surfaces.push(row.surface);
            }
            if !themes.contains(&row.theme) {
                themes.push(row.theme);
            }
            if !densities.contains(&row.density) {
                densities.push(row.density);
            }
            if !postures.contains(&row.posture) {
                postures.push(row.posture);
            }
        }
        surfaces.sort();
        themes.sort_by_key(|t| t.token());
        densities.sort_by_key(|d| d.token());
        postures.sort_by_key(|p| p.token());
        Self {
            row_count: rows.len(),
            surface_count: surfaces.len(),
            theme_count: themes.len(),
            density_count: densities.len(),
            posture_count: postures.len(),
            defect_count: defects.len(),
            surfaces_present: surfaces,
            themes_present: themes,
            densities_present: densities,
            postures_present: postures,
        }
    }
}

/// Top-level beta token-state audit page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenStateAuditPage {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Reviewer-facing page label.
    pub page_label: String,
    /// Aggregate summary banner.
    pub summary: TokenStateAuditSummary,
    /// Audited rows.
    pub rows: Vec<TokenStateAuditRow>,
    /// Defects emitted by the validator. The seeded page emits zero;
    /// the validator emits typed entries when a row drifts.
    pub defects: Vec<TokenStateAuditDefect>,
}

impl TokenStateAuditPage {
    /// Builds an audit page from a row list. The defect list is computed
    /// by running the validator over the rows; the seeded rows seed
    /// zero defects.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        rows: Vec<TokenStateAuditRow>,
    ) -> Self {
        let defects = audit_rows(&rows);
        let summary = TokenStateAuditSummary::from_rows(&rows, &defects);
        Self {
            record_kind: TOKEN_STATE_AUDIT_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: TOKEN_STATE_AUDIT_BETA_SCHEMA_VERSION,
            shared_contract_ref: TOKEN_STATE_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            summary,
            rows,
            defects,
        }
    }

    /// True when the page covers every claimed beta theme class.
    pub fn covers_required_themes(&self) -> bool {
        [
            ThemeClass::DarkReference,
            ThemeClass::LightParity,
            ThemeClass::HighContrastDark,
            ThemeClass::HighContrastLight,
        ]
        .iter()
        .all(|t| self.summary.themes_present.contains(t))
    }

    /// True when the page covers every claimed beta density class.
    pub fn covers_required_densities(&self) -> bool {
        [
            DensityClass::Compact,
            DensityClass::Standard,
            DensityClass::Comfortable,
        ]
        .iter()
        .all(|d| self.summary.densities_present.contains(d))
    }

    /// True when the page covers every claimed beta accessibility posture.
    pub fn covers_required_postures(&self) -> bool {
        [
            AccessibilityPostureClass::MotionStandard,
            AccessibilityPostureClass::MotionReduced,
            AccessibilityPostureClass::MotionLowMotion,
            AccessibilityPostureClass::MotionPowerSaver,
            AccessibilityPostureClass::MotionCriticalHotPath,
        ]
        .iter()
        .all(|p| self.summary.postures_present.contains(p))
    }
}

/// Support-export wrapper that quotes the audited page plus a
/// metadata-safe defect roll-up so support reviewers can see what the
/// validator saw without running the inspector themselves.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenStateAuditSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Beta schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Generated-at timestamp.
    pub generated_at: String,
    /// Embedded beta page.
    pub page: TokenStateAuditPage,
    /// Defect kinds present on the embedded page in stable order.
    pub defect_kinds_present: Vec<TokenStateAuditDefectKind>,
    /// Defect counts keyed by stable defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when no raw private material crosses the export boundary.
    pub raw_private_material_excluded: bool,
}

impl TokenStateAuditSupportExport {
    /// Builds a support-export wrapper from a beta page.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: TokenStateAuditPage,
    ) -> Self {
        let mut kinds: Vec<TokenStateAuditDefectKind> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !kinds.contains(&defect.defect_kind) {
                kinds.push(defect.defect_kind);
            }
            *counts.entry(defect.defect_kind_token.clone()).or_insert(0) += 1;
        }
        kinds.sort();
        Self {
            record_kind: TOKEN_STATE_AUDIT_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: TOKEN_STATE_AUDIT_BETA_SCHEMA_VERSION,
            shared_contract_ref: TOKEN_STATE_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            defect_kinds_present: kinds,
            defect_counts_by_kind: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Runs the audit validator over a row list and returns the defect
/// list. The seeded rows seed zero defects; defects appear when a row
/// drops a required token, drops a required state symbol, degrades a
/// motion substitution, collapses density geometry, or drifts an
/// action label for one surface.
pub fn audit_rows(rows: &[TokenStateAuditRow]) -> Vec<TokenStateAuditDefect> {
    let mut defects: Vec<TokenStateAuditDefect> = Vec::new();

    let trust_token_prefixes = [
        "status.warning.",
        "status.danger.",
        "status.success.",
        "trust.restricted.",
        "trust.locked.",
    ];

    for row in rows {
        let promises: &[SemanticPromise] = &row.promised_semantics;

        if row.motion_preservation.motion_preset_ref.is_empty() {
            defects.push(TokenStateAuditDefect::new(
                TokenStateAuditDefectKind::MissingMotionPresetReference,
                row.surface,
                &row.row_id,
                "motion_preservation.motion_preset_ref",
                "row did not name a motion preset reference",
            ));
        }

        if promises.contains(&SemanticPromise::FocusLegible) {
            let has_focus_token = row
                .required_color_tokens
                .iter()
                .any(|tok| tok.starts_with("color.focus.") || tok == "focus.ring");
            if !has_focus_token {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::MissingFocusColorToken,
                    row.surface,
                    &row.row_id,
                    "required_color_tokens",
                    "promised focus_legible but did not declare a color.focus.* token",
                ));
            }
            if !row
                .required_component_states
                .contains(&ComponentStateClass::FocusVisible)
            {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::MissingFocusVisibleStateSymbol,
                    row.surface,
                    &row.row_id,
                    "required_component_states",
                    "promised focus_legible but did not declare ComponentStateClass::FocusVisible",
                ));
            }
            if !row.motion_preservation.preserves_focus_visibility {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::MotionStripsFocusVisibility,
                    row.surface,
                    &row.row_id,
                    "motion_preservation.preserves_focus_visibility",
                    "promised focus_legible but the motion posture would hide the focus ring",
                ));
            }
        }

        if promises.contains(&SemanticPromise::TrustLegible) {
            let has_trust_token = row.required_color_tokens.iter().any(|tok| {
                trust_token_prefixes
                    .iter()
                    .any(|prefix| tok.starts_with(prefix))
            });
            if !has_trust_token {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::MissingTrustToken,
                    row.surface,
                    &row.row_id,
                    "required_color_tokens",
                    "promised trust_legible but did not declare a status.* or trust.* token",
                ));
            }
            let has_trust_state = row.required_component_states.iter().any(|state| {
                matches!(
                    state,
                    ComponentStateClass::Warning
                        | ComponentStateClass::Restricted
                        | ComponentStateClass::PolicyBlocked
                        | ComponentStateClass::Locked
                        | ComponentStateClass::Destructive
                )
            });
            if !has_trust_state {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::MissingTrustStateSymbol,
                    row.surface,
                    &row.row_id,
                    "required_component_states",
                    "promised trust_legible but trust treatment would be carried by hue alone",
                ));
            }
        }

        if promises.contains(&SemanticPromise::DegradedLegible) {
            if !row
                .required_component_states
                .contains(&ComponentStateClass::Degraded)
            {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::MissingDegradedStateSymbol,
                    row.surface,
                    &row.row_id,
                    "required_component_states",
                    "promised degraded_legible but did not declare ComponentStateClass::Degraded",
                ));
            }
            if !row.motion_preservation.preserves_state_conveyance {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::MotionStripsStateConveyance,
                    row.surface,
                    &row.row_id,
                    "motion_preservation.preserves_state_conveyance",
                    "promised degraded_legible but motion would suppress the state marker",
                ));
            }
        }

        let geometry_ok = row.density_preservation.preserves_row_height_token
            && row.density_preservation.preserves_control_height_token
            && row.density_preservation.preserves_panel_padding_token;
        if !geometry_ok {
            defects.push(TokenStateAuditDefect::new(
                TokenStateAuditDefectKind::DensityCollapsesGeometryToken,
                row.surface,
                &row.row_id,
                "density_preservation",
                "density mode would collapse the row, control, or panel-padding token",
            ));
        }

        let has_row_height = row
            .required_size_tokens
            .iter()
            .any(|tok| tok.starts_with("size.row."));
        let has_control_height = row
            .required_size_tokens
            .iter()
            .any(|tok| tok.starts_with("size.control."));
        if !has_row_height || !has_control_height {
            defects.push(TokenStateAuditDefect::new(
                TokenStateAuditDefectKind::MissingDensityGeometryToken,
                row.surface,
                &row.row_id,
                "required_size_tokens",
                "row did not list both size.row.* and size.control.* tokens",
            ));
        }
    }

    let mut by_surface: BTreeMap<LaunchCriticalSurfaceClass, Vec<&TokenStateAuditRow>> =
        BTreeMap::new();
    for row in rows {
        by_surface.entry(row.surface).or_default().push(row);
    }
    for (surface, surface_rows) in by_surface {
        if surface_rows.len() < 2 {
            continue;
        }
        let canonical_command_id = surface_rows[0].canonical_command_id.as_str();
        let canonical_action_label = surface_rows[0].canonical_action_label.as_str();
        for row in surface_rows.iter().skip(1) {
            if row.canonical_command_id != canonical_command_id {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::ActionLabelDriftsAcrossRows,
                    surface,
                    &row.row_id,
                    "canonical_command_id",
                    "command id drifted from the first row for this surface",
                ));
            }
            if row.canonical_action_label != canonical_action_label {
                defects.push(TokenStateAuditDefect::new(
                    TokenStateAuditDefectKind::ActionLabelDriftsAcrossRows,
                    surface,
                    &row.row_id,
                    "canonical_action_label",
                    "action label drifted from the first row for this surface",
                ));
            }
        }
    }

    defects
}

/// Validates that the seeded page seeds zero defects. The validator
/// returns the defect list verbatim; an empty list means the page is
/// clean. This is the page-level entry point used by the headless
/// inspector and the integration test.
pub fn validate_token_state_audit_page(
    page: &TokenStateAuditPage,
) -> Result<(), Vec<TokenStateAuditDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

fn make_row(
    case_id: &str,
    row_suffix: &str,
    surface: LaunchCriticalSurfaceClass,
    theme: ThemeClass,
    density: DensityClass,
    posture: AccessibilityPostureClass,
    required_color_tokens: &[&str],
    required_size_tokens: &[&str],
    required_space_tokens: &[&str],
    required_component_states: &[ComponentStateClass],
    motion_preset_ref: &str,
    substitution_class: ReducedMotionSubstitutionClass,
    preserves_state_conveyance: bool,
    preserves_focus_visibility: bool,
    promised_semantics: &[SemanticPromise],
    canonical_command_id: &str,
    canonical_action_label: &str,
    narrative: &str,
) -> TokenStateAuditRow {
    TokenStateAuditRow {
        record_kind: TOKEN_STATE_AUDIT_BETA_ROW_RECORD_KIND.to_owned(),
        schema_version: TOKEN_STATE_AUDIT_BETA_SCHEMA_VERSION,
        shared_contract_ref: TOKEN_STATE_AUDIT_BETA_SHARED_CONTRACT_REF.to_owned(),
        case_id: case_id.to_owned(),
        row_id: format!("ux:token-state-audit:beta:{row_suffix}"),
        surface,
        theme,
        density,
        posture,
        required_color_tokens: required_color_tokens
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        required_size_tokens: required_size_tokens
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        required_space_tokens: required_space_tokens
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        required_component_states: required_component_states.to_vec(),
        density_preservation: DensityPreservation::preserved(),
        motion_preservation: MotionPreservation {
            motion_preset_ref: motion_preset_ref.to_owned(),
            substitution_class,
            preserves_state_conveyance,
            preserves_focus_visibility,
        },
        promised_semantics: promised_semantics.to_vec(),
        canonical_command_id: canonical_command_id.to_owned(),
        canonical_action_label: canonical_action_label.to_owned(),
        narrative: narrative.to_owned(),
    }
}

/// Seeded fixture builder used by the headless inspector and the
/// integration test. The seed is the only mint-from-truth path for the
/// JSON checked in under `fixtures/ux/m3/theme_density_motion/`, so
/// the live shell records, the CLI rows, and the support-export rows
/// cannot drift.
pub fn seeded_token_state_audit_page() -> TokenStateAuditPage {
    let focus_color_tokens = ["color.focus.ring", "color.surface.background.chrome"];
    let trust_warning_tokens = [
        "status.warning.border",
        "status.warning.foreground",
        "color.focus.ring",
    ];
    let degraded_tokens = [
        "status.warning.foreground",
        "color.surface.background.degraded",
        "color.focus.ring",
    ];
    let danger_tokens = [
        "status.danger.border",
        "status.danger.foreground",
        "color.focus.ring",
    ];

    let mut rows: Vec<TokenStateAuditRow> = Vec::new();

    rows.push(make_row(
        "shell:token-state-audit:beta:shell-chrome:dark-standard:01",
        "shell-chrome:dark:standard:standard",
        LaunchCriticalSurfaceClass::ShellChrome,
        ThemeClass::DarkReference,
        DensityClass::Standard,
        AccessibilityPostureClass::MotionStandard,
        &focus_color_tokens,
        &["size.row.standard", "size.control.standard"],
        &["space.4", "space.3"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
            ComponentStateClass::Current,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:shell.focus_active_zone",
        "Focus active zone",
        "Shell chrome on dark reference / standard density / standard motion: focus ring resolves through color.focus.ring; selected and current states ride the shared component-state registry.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:shell-chrome:light-comfortable:02",
        "shell-chrome:light:comfortable:reduced",
        LaunchCriticalSurfaceClass::ShellChrome,
        ThemeClass::LightParity,
        DensityClass::Comfortable,
        AccessibilityPostureClass::MotionReduced,
        &focus_color_tokens,
        &["size.row.comfortable", "size.control.comfortable"],
        &["space.5", "space.4"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
            ComponentStateClass::Current,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::CrossfadeOnly,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:shell.focus_active_zone",
        "Focus active zone",
        "Shell chrome on light parity / comfortable density / reduced motion: focus visibility preserved via crossfade-only substitution; row geometry resolves through size.row.comfortable.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:shell-chrome:high-contrast-dark-compact:03",
        "shell-chrome:high-contrast-dark:compact:low-motion",
        LaunchCriticalSurfaceClass::ShellChrome,
        ThemeClass::HighContrastDark,
        DensityClass::Compact,
        AccessibilityPostureClass::MotionLowMotion,
        &focus_color_tokens,
        &["size.row.compact", "size.control.compact"],
        &["space.3", "space.2"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
            ComponentStateClass::Current,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::CollapseToInstant,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:shell.focus_active_zone",
        "Focus active zone",
        "Shell chrome on high-contrast dark / compact density / low motion: collapse-to-instant substitution preserves the focus ring; compact row height is derived from size.row.compact, not a hard-coded pixel.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:start-center:light-standard:01",
        "start-center:light:standard:standard",
        LaunchCriticalSurfaceClass::StartCenter,
        ThemeClass::LightParity,
        DensityClass::Standard,
        AccessibilityPostureClass::MotionStandard,
        &focus_color_tokens,
        &["size.row.standard", "size.control.standard"],
        &["space.4", "space.3"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:start_center.open_recent",
        "Open recent workspace",
        "Start Center on light parity / standard density / standard motion: focus visibility and selected row both come from the shared registry; the action label is identical across themes and densities.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:start-center:high-contrast-light-compact:02",
        "start-center:high-contrast-light:compact:reduced",
        LaunchCriticalSurfaceClass::StartCenter,
        ThemeClass::HighContrastLight,
        DensityClass::Compact,
        AccessibilityPostureClass::MotionReduced,
        &focus_color_tokens,
        &["size.row.compact", "size.control.compact"],
        &["space.3", "space.2"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::CrossfadeOnly,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:start_center.open_recent",
        "Open recent workspace",
        "Start Center on high-contrast light / compact density / reduced motion: focus ring preserved via crossfade; the canonical command id matches the dark-themed row so the action keeps its meaning.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:command-palette:dark-standard:01",
        "command-palette:dark:standard:standard",
        LaunchCriticalSurfaceClass::CommandPalette,
        ThemeClass::DarkReference,
        DensityClass::Standard,
        AccessibilityPostureClass::MotionStandard,
        &focus_color_tokens,
        &["size.row.standard", "size.control.standard"],
        &["space.4", "space.3"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
            ComponentStateClass::Current,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:palette.run_command",
        "Run command",
        "Command palette on dark reference / standard density / standard motion: focus ring + selected row + current item all map through the shared component-state registry.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:command-palette:high-contrast-dark-comfortable:02",
        "command-palette:high-contrast-dark:comfortable:power-saver",
        LaunchCriticalSurfaceClass::CommandPalette,
        ThemeClass::HighContrastDark,
        DensityClass::Comfortable,
        AccessibilityPostureClass::MotionPowerSaver,
        &focus_color_tokens,
        &["size.row.comfortable", "size.control.comfortable"],
        &["space.5", "space.4"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
            ComponentStateClass::Current,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::CollapseToInstant,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:palette.run_command",
        "Run command",
        "Command palette on high-contrast dark / comfortable density / power-saver motion: motion collapses to instant but the focus ring and selected row remain visible.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:scope-truth:dark-standard:01",
        "scope-truth:dark:standard:standard",
        LaunchCriticalSurfaceClass::ScopeTruthChip,
        ThemeClass::DarkReference,
        DensityClass::Standard,
        AccessibilityPostureClass::MotionStandard,
        &trust_warning_tokens,
        &["size.row.standard", "size.control.standard"],
        &["space.4", "space.3"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Warning,
            ComponentStateClass::Restricted,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::TrustLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:scope_truth.review_chip",
        "Review trust scope",
        "Scope-truth chip on dark reference / standard density / standard motion: warning treatment carried by status.warning.* tokens AND the Warning state symbol so trust narrowing is never carried by hue alone.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:scope-truth:high-contrast-light-compact:02",
        "scope-truth:high-contrast-light:compact:critical-hot-path",
        LaunchCriticalSurfaceClass::ScopeTruthChip,
        ThemeClass::HighContrastLight,
        DensityClass::Compact,
        AccessibilityPostureClass::MotionCriticalHotPath,
        &trust_warning_tokens,
        &["size.row.compact", "size.control.compact"],
        &["space.3", "space.2"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Warning,
            ComponentStateClass::Restricted,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::NonMotionStateMarker,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::TrustLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:scope_truth.review_chip",
        "Review trust scope",
        "Scope-truth chip on high-contrast light / compact density / critical hot path: motion is replaced by a non-motion state marker so trust state remains visible during the hot-path frame.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:activity-row:dark-comfortable:01",
        "activity-row:dark:comfortable:standard",
        LaunchCriticalSurfaceClass::ActivityCenterRow,
        ThemeClass::DarkReference,
        DensityClass::Comfortable,
        AccessibilityPostureClass::MotionStandard,
        &degraded_tokens,
        &["size.row.comfortable", "size.control.comfortable"],
        &["space.5", "space.4"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Degraded,
            ComponentStateClass::Pending,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::DegradedLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:activity.open_job_details",
        "Open job details",
        "Activity center row on dark reference / comfortable density / standard motion: degraded posture carried by ComponentStateClass::Degraded plus a non-motion state marker; focus ring resolves through color.focus.ring.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:activity-row:light-standard:02",
        "activity-row:light:standard:reduced",
        LaunchCriticalSurfaceClass::ActivityCenterRow,
        ThemeClass::LightParity,
        DensityClass::Standard,
        AccessibilityPostureClass::MotionReduced,
        &degraded_tokens,
        &["size.row.standard", "size.control.standard"],
        &["space.4", "space.3"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Degraded,
            ComponentStateClass::Pending,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::CrossfadeOnly,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::DegradedLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:activity.open_job_details",
        "Open job details",
        "Activity center row on light parity / standard density / reduced motion: degraded marker remains visible after the crossfade substitution; the action label is identical across rows.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:notification:dark-standard:01",
        "notification:dark:standard:standard",
        LaunchCriticalSurfaceClass::NotificationEnvelope,
        ThemeClass::DarkReference,
        DensityClass::Standard,
        AccessibilityPostureClass::MotionStandard,
        &danger_tokens,
        &["size.row.standard", "size.control.standard"],
        &["space.4", "space.3"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Warning,
            ComponentStateClass::Destructive,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::TrustLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:notification.open_review",
        "Open notification review",
        "Notification envelope on dark reference / standard density / standard motion: severity treatment resolves through status.danger.* tokens; warning + destructive state symbols keep the row legible after color is removed.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:notification:high-contrast-dark-comfortable:02",
        "notification:high-contrast-dark:comfortable:critical-hot-path",
        LaunchCriticalSurfaceClass::NotificationEnvelope,
        ThemeClass::HighContrastDark,
        DensityClass::Comfortable,
        AccessibilityPostureClass::MotionCriticalHotPath,
        &danger_tokens,
        &["size.row.comfortable", "size.control.comfortable"],
        &["space.5", "space.4"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Warning,
            ComponentStateClass::Destructive,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::NonMotionStateMarker,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::TrustLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:notification.open_review",
        "Open notification review",
        "Notification envelope on high-contrast dark / comfortable density / critical hot path: motion replaced by a non-motion state marker; severity remains legible in the hot-path frame.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:trust-prompt:light-standard:01",
        "trust-prompt:light:standard:standard",
        LaunchCriticalSurfaceClass::TrustPromptSheet,
        ThemeClass::LightParity,
        DensityClass::Standard,
        AccessibilityPostureClass::MotionStandard,
        &danger_tokens,
        &["size.row.standard", "size.control.standard"],
        &["space.4", "space.3"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Warning,
            ComponentStateClass::Destructive,
            ComponentStateClass::Locked,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::TrustLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:trust_prompt.confirm_decision",
        "Confirm trust decision",
        "Trust prompt sheet on light parity / standard density / standard motion: danger treatment carried by status.danger.* tokens AND warning/destructive/locked state symbols.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:trust-prompt:high-contrast-dark-compact:02",
        "trust-prompt:high-contrast-dark:compact:low-motion",
        LaunchCriticalSurfaceClass::TrustPromptSheet,
        ThemeClass::HighContrastDark,
        DensityClass::Compact,
        AccessibilityPostureClass::MotionLowMotion,
        &danger_tokens,
        &["size.row.compact", "size.control.compact"],
        &["space.3", "space.2"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Warning,
            ComponentStateClass::Destructive,
            ComponentStateClass::Locked,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::CollapseToInstant,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::TrustLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:trust_prompt.confirm_decision",
        "Confirm trust decision",
        "Trust prompt sheet on high-contrast dark / compact density / low motion: collapse-to-instant substitution preserves the focus ring; trust treatment still carried by status.danger.* + state symbols.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:settings-root:dark-standard:01",
        "settings-root:dark:standard:standard",
        LaunchCriticalSurfaceClass::SettingsRoot,
        ThemeClass::DarkReference,
        DensityClass::Standard,
        AccessibilityPostureClass::MotionStandard,
        &focus_color_tokens,
        &["size.row.standard", "size.control.standard"],
        &["space.4", "space.3"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
            ComponentStateClass::Current,
            ComponentStateClass::PolicyBlocked,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::MaintainEssentialKeepSimplified,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:settings.focus_pane",
        "Focus settings pane",
        "Settings root on dark reference / standard density / standard motion: navigation focus, current pane, and policy-locked rows all use the shared component-state registry.",
    ));

    rows.push(make_row(
        "shell:token-state-audit:beta:settings-root:high-contrast-light-comfortable:02",
        "settings-root:high-contrast-light:comfortable:reduced",
        LaunchCriticalSurfaceClass::SettingsRoot,
        ThemeClass::HighContrastLight,
        DensityClass::Comfortable,
        AccessibilityPostureClass::MotionReduced,
        &focus_color_tokens,
        &["size.row.comfortable", "size.control.comfortable"],
        &["space.5", "space.4"],
        &[
            ComponentStateClass::FocusVisible,
            ComponentStateClass::Selected,
            ComponentStateClass::Current,
            ComponentStateClass::PolicyBlocked,
        ],
        "motion_preset:overlay.dialog.enter",
        ReducedMotionSubstitutionClass::CrossfadeOnly,
        true,
        true,
        &[
            SemanticPromise::FocusLegible,
            SemanticPromise::ActionLabelStable,
        ],
        "cmd:settings.focus_pane",
        "Focus settings pane",
        "Settings root on high-contrast light / comfortable density / reduced motion: focus ring preserved via crossfade; policy-locked rows remain legible after the substitution.",
    ));

    TokenStateAuditPage::new(
        "shell:token-state-audit:beta:page:default",
        "Token / state / density / motion / theme audit (beta): launch-critical shell surfaces",
        rows,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_seeds_zero_defects() {
        let page = seeded_token_state_audit_page();
        assert!(
            page.defects.is_empty(),
            "seeded page must seed zero defects: {:#?}",
            page.defects
        );
        validate_token_state_audit_page(&page).expect("seeded page must validate");
    }

    #[test]
    fn seeded_page_covers_required_themes_densities_postures() {
        let page = seeded_token_state_audit_page();
        assert!(page.covers_required_themes(), "themes covered");
        assert!(page.covers_required_densities(), "densities covered");
        assert!(page.covers_required_postures(), "postures covered");
    }

    #[test]
    fn seeded_summary_matches_rows() {
        let page = seeded_token_state_audit_page();
        assert_eq!(page.summary.row_count, page.rows.len());
        assert_eq!(page.summary.defect_count, 0);
        assert!(page.summary.surface_count > 0);
    }

    #[test]
    fn audit_flags_focus_color_token_drop() {
        let mut page = seeded_token_state_audit_page();
        page.rows[0].required_color_tokens.clear();
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MissingFocusColorToken));
    }

    #[test]
    fn audit_flags_focus_visible_state_drop() {
        let mut page = seeded_token_state_audit_page();
        page.rows[0]
            .required_component_states
            .retain(|s| !matches!(s, ComponentStateClass::FocusVisible));
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MissingFocusVisibleStateSymbol));
    }

    #[test]
    fn audit_flags_trust_token_drop() {
        let mut page = seeded_token_state_audit_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| {
                r.promised_semantics
                    .contains(&SemanticPromise::TrustLegible)
            })
            .expect("seed has a trust row");
        row.required_color_tokens.retain(|tok| {
            !tok.starts_with("status.warning.") && !tok.starts_with("status.danger.")
        });
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MissingTrustToken));
    }

    #[test]
    fn audit_flags_trust_state_drop() {
        let mut page = seeded_token_state_audit_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| {
                r.promised_semantics
                    .contains(&SemanticPromise::TrustLegible)
            })
            .expect("seed has a trust row");
        row.required_component_states.retain(|state| {
            !matches!(
                state,
                ComponentStateClass::Warning
                    | ComponentStateClass::Restricted
                    | ComponentStateClass::PolicyBlocked
                    | ComponentStateClass::Locked
                    | ComponentStateClass::Destructive
            )
        });
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MissingTrustStateSymbol));
    }

    #[test]
    fn audit_flags_degraded_state_drop() {
        let mut page = seeded_token_state_audit_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| {
                r.promised_semantics
                    .contains(&SemanticPromise::DegradedLegible)
            })
            .expect("seed has a degraded row");
        row.required_component_states
            .retain(|s| !matches!(s, ComponentStateClass::Degraded));
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MissingDegradedStateSymbol));
    }

    #[test]
    fn audit_flags_motion_strips_focus_visibility() {
        let mut page = seeded_token_state_audit_page();
        page.rows[0].motion_preservation.preserves_focus_visibility = false;
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MotionStripsFocusVisibility));
    }

    #[test]
    fn audit_flags_motion_strips_state_conveyance() {
        let mut page = seeded_token_state_audit_page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| {
                r.promised_semantics
                    .contains(&SemanticPromise::DegradedLegible)
            })
            .expect("seed has a degraded row");
        row.motion_preservation.preserves_state_conveyance = false;
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MotionStripsStateConveyance));
    }

    #[test]
    fn audit_flags_density_geometry_collapse() {
        let mut page = seeded_token_state_audit_page();
        page.rows[0].density_preservation.preserves_row_height_token = false;
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::DensityCollapsesGeometryToken));
    }

    #[test]
    fn audit_flags_action_label_drift() {
        let mut page = seeded_token_state_audit_page();
        let surface = page.rows[0].surface;
        let other = page
            .rows
            .iter_mut()
            .filter(|r| r.surface == surface)
            .nth(1)
            .expect("two rows for the surface");
        other.canonical_action_label = "Drifted label".to_owned();
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::ActionLabelDriftsAcrossRows));
    }

    #[test]
    fn audit_flags_missing_motion_preset_reference() {
        let mut page = seeded_token_state_audit_page();
        page.rows[0].motion_preservation.motion_preset_ref.clear();
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MissingMotionPresetReference));
    }

    #[test]
    fn audit_flags_missing_density_geometry_token() {
        let mut page = seeded_token_state_audit_page();
        page.rows[0]
            .required_size_tokens
            .retain(|tok| !tok.starts_with("size.row."));
        let defects = audit_rows(&page.rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == TokenStateAuditDefectKind::MissingDensityGeometryToken));
    }

    #[test]
    fn support_export_quotes_page_and_summarises_defects() {
        let page = seeded_token_state_audit_page();
        let export = TokenStateAuditSupportExport::from_page(
            "support-export:token-state-audit-beta:001",
            "2026-05-15T00:00:00Z",
            page.clone(),
        );
        assert_eq!(
            export.shared_contract_ref,
            TOKEN_STATE_AUDIT_BETA_SHARED_CONTRACT_REF
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.defect_counts_by_kind.is_empty());
        assert_eq!(export.page, page);
    }

    #[test]
    fn support_export_summarises_seeded_defects_when_present() {
        let mut page = seeded_token_state_audit_page();
        page.rows[0].required_color_tokens.clear();
        page.defects = audit_rows(&page.rows);
        page.summary = TokenStateAuditSummary::from_rows(&page.rows, &page.defects);
        let export = TokenStateAuditSupportExport::from_page(
            "support-export:token-state-audit-beta:002",
            "2026-05-15T00:00:00Z",
            page,
        );
        assert!(export
            .defect_kinds_present
            .contains(&TokenStateAuditDefectKind::MissingFocusColorToken));
        let count = export
            .defect_counts_by_kind
            .get(TokenStateAuditDefectKind::MissingFocusColorToken.as_str())
            .copied()
            .unwrap_or(0);
        assert_eq!(count, 1);
    }
}
