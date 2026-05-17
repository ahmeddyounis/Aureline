//! Governed beta design-system contracts.
//!
//! This crate publishes the beta contract that launch-critical shell surfaces
//! consume for canonical component states, badge and notice cue families,
//! screenshot-diff evidence rows, and token-conformance gates. It builds on
//! [`aureline_ui`] token, density, motion, theme, and component-state types so
//! this crate remains a governance and evidence layer rather than a parallel UI
//! implementation.

use std::collections::{BTreeMap, BTreeSet};

use aureline_ui::components::ComponentStateClass;
use aureline_ui::density::DensityClass;
use aureline_ui::themes::AccessibilityPostureClass;
use aureline_ui::tokens::{seeded_token_registry, ThemeClass};
use serde::{Deserialize, Serialize};

/// Schema version emitted by all beta design-system contract records.
pub const DESIGN_SYSTEM_BETA_SCHEMA_VERSION: u32 = 1;

/// Shared beta contract reference carried by related records.
pub const DESIGN_SYSTEM_BETA_SHARED_CONTRACT_REF: &str = "design_system:component_state_token:v1";

/// Record kind for [`ComponentStateRegistryRecord`].
pub const COMPONENT_STATE_REGISTRY_RECORD_KIND: &str = "component_state_registry_beta_record";

/// Record kind for [`ComponentStateFamilyRow`].
pub const COMPONENT_STATE_FAMILY_RECORD_KIND: &str = "component_state_family_beta_record";

/// Record kind for [`CueFamilyRow`].
pub const CUE_FAMILY_RECORD_KIND: &str = "component_cue_family_beta_record";

/// Record kind for [`LaunchSurfaceConsumerRow`].
pub const LAUNCH_SURFACE_CONSUMER_RECORD_KIND: &str = "launch_surface_consumer_beta_record";

/// Record kind for [`ScreenshotDiffPacket`].
pub const SCREENSHOT_DIFF_PACKET_RECORD_KIND: &str = "component_state_screenshot_diff_packet";

/// Record kind for [`TokenConformancePacket`].
pub const TOKEN_CONFORMANCE_PACKET_RECORD_KIND: &str = "token_conformance_packet_record";

/// Record kind for [`DesignSystemFinding`].
pub const DESIGN_SYSTEM_FINDING_RECORD_KIND: &str = "design_system_contract_finding";

/// Canonical component-state classes enforced for beta truth surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalStateClass {
    /// The component is empty but ready to accept a useful route.
    Empty,
    /// Content is not ready yet and the product is preparing it.
    Loading,
    /// A user action has been submitted, staged, or queued.
    Pending,
    /// Reduced capability remains and the surface names what still works.
    Degraded,
    /// Action is prevented by policy, trust, permission, ownership, source, or capability.
    Blocked,
    /// A failed or invalid state requires retry, repair, or diagnostics.
    Error,
    /// Durable success has completed and remains reviewable.
    Completed,
}

impl CanonicalStateClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Loading => "loading",
            Self::Pending => "pending",
            Self::Degraded => "degraded",
            Self::Blocked => "blocked",
            Self::Error => "error",
            Self::Completed => "completed",
        }
    }

    /// Returns the closed required state set in stable order.
    pub const fn required() -> &'static [Self] {
        &[
            Self::Empty,
            Self::Loading,
            Self::Pending,
            Self::Degraded,
            Self::Blocked,
            Self::Error,
            Self::Completed,
        ]
    }
}

/// Badge or notice family class used for compact lifecycle and policy cues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CueFamilyClass {
    /// Lifecycle state such as experimental, preview, ready, deprecated, or retired.
    Lifecycle,
    /// Route or origin state such as local, remote, mirrored, or browser handoff.
    Route,
    /// Readiness state such as ready, partial, unavailable, or blocked.
    Readiness,
    /// Policy cue such as allowed, limited, locked, or exception required.
    Policy,
}

impl CueFamilyClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lifecycle => "lifecycle",
            Self::Route => "route",
            Self::Readiness => "readiness",
            Self::Policy => "policy",
        }
    }

    /// Returns the closed required cue-family set in stable order.
    pub const fn required() -> &'static [Self] {
        &[Self::Lifecycle, Self::Route, Self::Readiness, Self::Policy]
    }
}

/// Visual carrier for a cue family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CueKind {
    /// Compact badge or pill treatment.
    Badge,
    /// Inline or banner notice treatment.
    Notice,
}

impl CueKind {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Badge => "badge",
            Self::Notice => "notice",
        }
    }
}

/// Required non-color cue classes for state meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonColorCueClass {
    /// Visible label text names the state.
    LabelText,
    /// Icon or glyph carries a stable metaphor.
    Icon,
    /// Border participates in the state treatment.
    Border,
    /// Shape differentiates the state beyond hue.
    Shape,
    /// Progress indicator is present alongside labels.
    ProgressIndicator,
    /// Lock or shield glyph identifies trust, policy, or permission constraints.
    LockOrShieldGlyph,
    /// Check or selection marker differentiates completed and selected states.
    CheckOrSelectionMarker,
}

impl NonColorCueClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LabelText => "label_text",
            Self::Icon => "icon",
            Self::Border => "border",
            Self::Shape => "shape",
            Self::ProgressIndicator => "progress_indicator",
            Self::LockOrShieldGlyph => "lock_or_shield_glyph",
            Self::CheckOrSelectionMarker => "check_or_selection_marker",
        }
    }
}

/// Launch-critical surface classes covered by the beta contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchSurfaceClass {
    /// Main app frame, rails, status strip, and persistent shell chrome.
    ShellChrome,
    /// First-use and return-to-work entry surface.
    StartCenter,
    /// Keyboard-first command palette surface.
    CommandPalette,
    /// Search row and result-list surface.
    SearchSurface,
    /// Dialog or capability sheet row.
    DialogSheet,
    /// Trust or permission prompt sheet.
    TrustPrompt,
    /// Notification envelope, toast, or durable notification row.
    NotificationEnvelope,
    /// Help and About rows used by support and release review.
    HelpAboutRow,
    /// Settings root and policy-managed setting rows.
    SettingsRoot,
    /// Durable attention or activity-center row.
    ActivityCenterRow,
}

impl LaunchSurfaceClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellChrome => "shell_chrome",
            Self::StartCenter => "start_center",
            Self::CommandPalette => "command_palette",
            Self::SearchSurface => "search_surface",
            Self::DialogSheet => "dialog_sheet",
            Self::TrustPrompt => "trust_prompt",
            Self::NotificationEnvelope => "notification_envelope",
            Self::HelpAboutRow => "help_about_row",
            Self::SettingsRoot => "settings_root",
            Self::ActivityCenterRow => "activity_center_row",
        }
    }

    /// Returns the required launch-critical surface set in stable order.
    pub const fn required() -> &'static [Self] {
        &[
            Self::ShellChrome,
            Self::StartCenter,
            Self::CommandPalette,
            Self::SearchSurface,
            Self::DialogSheet,
            Self::TrustPrompt,
            Self::NotificationEnvelope,
            Self::HelpAboutRow,
            Self::SettingsRoot,
            Self::ActivityCenterRow,
        ]
    }
}

/// Launch-priority class for a governed surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaunchPriorityClass {
    /// The surface carries beta truth and blocks release on drift.
    LaunchCritical,
    /// The surface participates in beta but does not alone carry launch truth.
    Supporting,
    /// The surface is explicitly outside the beta truth contract.
    OutOfScope,
}

/// Severity class for contract findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingSeverity {
    /// Informational finding only.
    Info,
    /// Non-blocking warning.
    Warning,
    /// Blocks the contract until fixed or waived.
    Error,
}

/// Gate state emitted by a checked packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateStateClass {
    /// The packet passes without findings.
    Pass,
    /// The packet passes with bounded, disclosed waivers.
    PassWithDisclosedGap,
    /// The packet should be reviewed but does not block by itself.
    Warn,
    /// The packet blocks release.
    Block,
}

impl GateStateClass {
    /// Returns the stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::PassWithDisclosedGap => "pass_with_disclosed_gap",
            Self::Warn => "warn",
            Self::Block => "block",
        }
    }
}

/// Token-backed treatment references for a canonical state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateTokenRefs {
    /// Foreground semantic token reference.
    pub foreground: String,
    /// Border semantic token reference.
    pub border: String,
    /// Fill semantic token reference.
    pub fill: String,
}

impl StateTokenRefs {
    fn new(foreground: &str, border: &str, fill: &str) -> Self {
        Self {
            foreground: foreground.to_owned(),
            border: border.to_owned(),
            fill: fill.to_owned(),
        }
    }

    fn all(&self) -> [&str; 3] {
        [
            self.foreground.as_str(),
            self.border.as_str(),
            self.fill.as_str(),
        ]
    }
}

/// Canonical state-family row consumed by surface contracts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentStateFamilyRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Canonical state class.
    pub state_class: CanonicalStateClass,
    /// Human-visible state label.
    pub display_label: String,
    /// Component-state classes the canonical state resolves to.
    pub taxonomy_state_refs: Vec<ComponentStateClass>,
    /// Token references that render this state.
    pub token_refs: StateTokenRefs,
    /// Non-color cues required for this state.
    pub required_non_color_cues: Vec<NonColorCueClass>,
    /// True when hue alone may carry meaning.
    pub hue_only_allowed: bool,
    /// True when a spinner alone may carry this state.
    pub spinner_only_allowed: bool,
    /// True when the state can hide critical action or reason behind hover only.
    pub hover_only_critical_action_allowed: bool,
    /// True when the state must remain visible without a tooltip.
    pub persistent_disclosure_required: bool,
    /// Screen-reader label for the state.
    pub screen_reader_label: String,
    /// Copy guidance for rows that enter this state.
    pub copy_rule: String,
}

/// Vocabulary token inside a badge or notice family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CueVocabularyToken {
    /// Stable token inside the cue family.
    pub token: String,
    /// Human-visible label.
    pub label: String,
    /// Canonical state class the token resolves to.
    pub state_class_ref: CanonicalStateClass,
    /// Token references for the cue token.
    pub token_refs: StateTokenRefs,
    /// Required non-color cues for this token.
    pub required_non_color_cues: Vec<NonColorCueClass>,
}

/// Badge or notice cue-family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CueFamilyRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Cue family.
    pub family_class: CueFamilyClass,
    /// Badge or notice carrier.
    pub cue_kind: CueKind,
    /// Token to use when the exact cue is unavailable.
    pub honesty_fallback_token: String,
    /// True when text and shape fallbacks are mandatory.
    pub requires_text_shape_fallback: bool,
    /// Required non-color cues for the family.
    pub required_non_color_cues: Vec<NonColorCueClass>,
    /// Vocabulary tokens published by the family.
    pub vocabulary_tokens: Vec<CueVocabularyToken>,
}

/// Surface consumer row binding launch-critical surfaces to the registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LaunchSurfaceConsumerRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Surface class.
    pub surface_class: LaunchSurfaceClass,
    /// Launch priority for this surface.
    pub launch_priority: LaunchPriorityClass,
    /// Repo-relative source or contract references that consume this registry.
    pub consumer_refs: Vec<String>,
    /// Component states consumed by this surface.
    pub consumes_component_states: Vec<CanonicalStateClass>,
    /// Cue families consumed by this surface.
    pub consumes_cue_families: Vec<CueFamilyClass>,
    /// Required theme classes for beta parity.
    pub required_theme_classes: Vec<ThemeClass>,
    /// Required density classes for beta parity.
    pub required_density_classes: Vec<DensityClass>,
    /// Required motion postures for beta parity.
    pub required_motion_postures: Vec<AccessibilityPostureClass>,
    /// True when critical actions are reachable without hover.
    pub critical_actions_keyboard_reachable: bool,
    /// True when focus visibility is explicitly preserved.
    pub focus_visibility_preserved: bool,
    /// True when state meaning has text, shape, icon, or border fallback.
    pub no_color_only_state_meaning: bool,
    /// Optional waiver reference for out-of-scope or bounded gaps.
    pub waiver_ref: Option<String>,
}

/// Canonical component-state registry and launch-surface bindings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentStateRegistryRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable registry identifier.
    pub registry_id: String,
    /// Source-of-truth document and artifact references.
    pub source_refs: Vec<String>,
    /// Runtime consumer references.
    pub runtime_consumer_refs: Vec<String>,
    /// Canonical component-state rows.
    pub component_state_families: Vec<ComponentStateFamilyRow>,
    /// Badge and notice family rows.
    pub cue_families: Vec<CueFamilyRow>,
    /// Launch-critical surface consumers.
    pub launch_surface_consumers: Vec<LaunchSurfaceConsumerRow>,
}

/// Contract finding emitted by registry, screenshot, or token validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DesignSystemFinding {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable finding identifier.
    pub finding_id: String,
    /// Finding severity.
    pub severity: FindingSeverity,
    /// Stable check identifier.
    pub check_id: String,
    /// Optional surface associated with the finding.
    pub surface_class: Option<LaunchSurfaceClass>,
    /// Field or row associated with the finding.
    pub field: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl DesignSystemFinding {
    fn error(
        check_id: &str,
        surface_class: Option<LaunchSurfaceClass>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let field = field.into();
        Self {
            record_kind: DESIGN_SYSTEM_FINDING_RECORD_KIND.to_owned(),
            schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
            finding_id: format!("design-system:finding:{check_id}:{}", stable_field(&field)),
            severity: FindingSeverity::Error,
            check_id: check_id.to_owned(),
            surface_class,
            field,
            note: note.into(),
        }
    }
}

fn stable_field(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'a'..='z' | '0'..='9' => ch,
            'A'..='Z' => ch.to_ascii_lowercase(),
            _ => '_',
        })
        .collect()
}

/// Screenshot-diff row for one surface and state across appearance axes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenshotDiffRow {
    /// Stable row identifier.
    pub row_id: String,
    /// Launch-critical surface.
    pub surface_class: LaunchSurfaceClass,
    /// Canonical state shown by the row.
    pub state_class: CanonicalStateClass,
    /// Theme class captured by this row.
    pub theme_class: ThemeClass,
    /// Density class captured by this row.
    pub density_class: DensityClass,
    /// Motion posture captured by this row.
    pub motion_posture: AccessibilityPostureClass,
    /// Baseline capture reference.
    pub baseline_capture_ref: String,
    /// Comparison capture reference.
    pub comparison_capture_ref: String,
    /// Diff artifact reference.
    pub diff_artifact_ref: String,
    /// Keyboard journey reference for the row.
    pub keyboard_journey_ref: String,
    /// Assistive-technology evidence reference.
    pub assistive_technology_ref: String,
    /// Token-conformance evidence reference.
    pub token_conformance_ref: String,
    /// Required non-color cues visible in this row.
    pub required_non_color_cues: Vec<NonColorCueClass>,
    /// True when no critical action depends on hover only.
    pub hover_only_critical_actions_absent: bool,
    /// True when focus visibility is present where the row can receive focus.
    pub focus_visibility_present: bool,
    /// True when blocked states are not represented by a spinner alone.
    pub spinner_only_blocked_state_absent: bool,
    /// True when state meaning is not encoded only through color.
    pub color_only_state_meaning_absent: bool,
    /// True when the diff preserves the state semantics.
    pub semantic_stability_passed: bool,
}

/// Summary for screenshot-diff or token-conformance packets.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct MatrixSummary {
    /// Number of rows in the packet.
    pub row_count: usize,
    /// Distinct surface count.
    pub surface_count: usize,
    /// Distinct state count.
    pub state_count: usize,
    /// Distinct theme count.
    pub theme_count: usize,
    /// Distinct density count.
    pub density_count: usize,
    /// Distinct motion-posture count.
    pub motion_posture_count: usize,
    /// Number of findings emitted.
    pub finding_count: usize,
}

/// Screenshot-diff packet for launch-critical component-state rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScreenshotDiffPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Registry reference used by this packet.
    pub registry_ref: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Screenshot-diff rows.
    pub rows: Vec<ScreenshotDiffRow>,
    /// Findings emitted by the packet validator.
    pub findings: Vec<DesignSystemFinding>,
    /// Aggregate summary.
    pub summary: MatrixSummary,
    /// Gate state resolved from findings and waivers.
    pub gate_state: GateStateClass,
}

/// Token-conformance row for one launch-critical surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenConformanceRow {
    /// Stable row identifier.
    pub row_id: String,
    /// Launch-critical surface.
    pub surface_class: LaunchSurfaceClass,
    /// Launch priority.
    pub launch_priority: LaunchPriorityClass,
    /// Source references scanned or asserted by the lane.
    pub source_refs: Vec<String>,
    /// Required color token family prefixes.
    pub required_color_token_families: Vec<String>,
    /// Required geometry token family prefixes.
    pub required_geometry_token_families: Vec<String>,
    /// Required motion token family prefixes.
    pub required_motion_token_families: Vec<String>,
    /// Required canonical component states.
    pub required_component_states: Vec<CanonicalStateClass>,
    /// Required cue families.
    pub required_cue_families: Vec<CueFamilyClass>,
    /// True when raw local color literals are forbidden.
    pub raw_color_literals_forbidden: bool,
    /// True when local spacing, density, or motion forks are forbidden.
    pub local_token_forks_forbidden: bool,
    /// True when the row has no bounded waiver.
    pub no_waiver_required: bool,
    /// Optional waiver reference.
    pub waiver_ref: Option<String>,
    /// Findings attached to this row.
    pub findings: Vec<DesignSystemFinding>,
}

/// Token-conformance packet for release review and CI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenConformancePacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract reference.
    pub shared_contract_ref: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Registry fixture reference.
    pub registry_ref: String,
    /// Screenshot-diff packet reference.
    pub screenshot_diff_packet_ref: String,
    /// Existing shell token-state audit reference.
    pub shell_token_state_audit_ref: String,
    /// Conformance rows.
    pub rows: Vec<TokenConformanceRow>,
    /// Aggregate findings.
    pub findings: Vec<DesignSystemFinding>,
    /// Aggregate summary.
    pub summary: MatrixSummary,
    /// Gate state resolved from findings and waivers.
    pub gate_state: GateStateClass,
    /// True when the packet excludes raw screenshots and private content.
    pub raw_private_material_excluded: bool,
}

/// Returns the seeded beta component-state registry.
pub fn seeded_component_state_registry() -> ComponentStateRegistryRecord {
    ComponentStateRegistryRecord {
        record_kind: COMPONENT_STATE_REGISTRY_RECORD_KIND.to_owned(),
        schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
        shared_contract_ref: DESIGN_SYSTEM_BETA_SHARED_CONTRACT_REF.to_owned(),
        registry_id: "component-state-registry:beta".to_owned(),
        source_refs: vec![
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md#15.4".to_owned(),
            ".t2/docs/Aureline_UX_Design_System_Style_Guide.md#appendix-n".to_owned(),
            ".t2/docs/Aureline_UI_UX_Spec_Document.md#6.4".to_owned(),
            "docs/design/component_state_diff_packet_template.md".to_owned(),
            "docs/ux/m3/design_token_beta_audit.md".to_owned(),
        ],
        runtime_consumer_refs: vec![
            "crates/aureline-ui/src/components/state_registry.rs".to_owned(),
            "crates/aureline-ui/src/tokens/state_semantics.rs".to_owned(),
            "crates/aureline-shell/src/token_state_audit/mod.rs".to_owned(),
            "crates/aureline-design-system/src/lib.rs".to_owned(),
        ],
        component_state_families: seeded_state_families(),
        cue_families: seeded_cue_families(),
        launch_surface_consumers: seeded_launch_surface_consumers(),
    }
}

fn seeded_state_families() -> Vec<ComponentStateFamilyRow> {
    use CanonicalStateClass as State;
    use ComponentStateClass as Taxonomy;
    use NonColorCueClass as Cue;

    vec![
        state_row(
            State::Empty,
            "Empty",
            &[Taxonomy::Idle],
            StateTokenRefs::new(
                "al.color.text.secondary",
                "al.color.border.default",
                "al.color.bg.surface",
            ),
            &[Cue::LabelText, Cue::Icon, Cue::Shape],
            "Empty state",
            "Name the missing content and the first useful route.",
        ),
        state_row(
            State::Loading,
            "Loading",
            &[Taxonomy::Loading],
            StateTokenRefs::new("status.info", "status.info.border", "status.info.fill"),
            &[Cue::LabelText, Cue::ProgressIndicator],
            "Loading",
            "Use only for initial preparation before content is ready.",
        ),
        state_row(
            State::Pending,
            "Pending",
            &[Taxonomy::Pending],
            StateTokenRefs::new(
                "status.insight",
                "status.insight.border",
                "status.insight.fill",
            ),
            &[Cue::LabelText, Cue::Icon],
            "Pending action",
            "Use after a user action is submitted, staged, or queued.",
        ),
        state_row(
            State::Degraded,
            "Degraded",
            &[Taxonomy::Degraded],
            StateTokenRefs::new(
                "status.warning",
                "status.warning.border",
                "status.warning.fill",
            ),
            &[Cue::LabelText, Cue::Icon, Cue::Border],
            "Degraded mode",
            "Name what still works, what is reduced, and the recovery path.",
        ),
        state_row(
            State::Blocked,
            "Blocked",
            &[Taxonomy::Locked, Taxonomy::PolicyBlocked],
            StateTokenRefs::new(
                "status.danger",
                "status.danger.border",
                "status.danger.fill",
            ),
            &[
                Cue::LabelText,
                Cue::Icon,
                Cue::Shape,
                Cue::LockOrShieldGlyph,
            ],
            "Blocked",
            "Name the block source and the least-surprising recovery or inspect route.",
        ),
        state_row(
            State::Error,
            "Error",
            &[Taxonomy::Warning],
            StateTokenRefs::new(
                "status.danger",
                "status.danger.border",
                "status.danger.fill",
            ),
            &[Cue::LabelText, Cue::Icon, Cue::Border],
            "Error",
            "Describe the failure and the available retry, repair, or diagnostic path.",
        ),
        state_row(
            State::Completed,
            "Completed",
            &[Taxonomy::Completed],
            StateTokenRefs::new(
                "status.success",
                "status.success.border",
                "status.success.fill",
            ),
            &[Cue::LabelText, Cue::Icon, Cue::CheckOrSelectionMarker],
            "Completed",
            "Use for durable success that remains reviewable.",
        ),
    ]
}

fn state_row(
    state_class: CanonicalStateClass,
    display_label: &str,
    taxonomy_state_refs: &[ComponentStateClass],
    token_refs: StateTokenRefs,
    required_non_color_cues: &[NonColorCueClass],
    screen_reader_label: &str,
    copy_rule: &str,
) -> ComponentStateFamilyRow {
    ComponentStateFamilyRow {
        record_kind: COMPONENT_STATE_FAMILY_RECORD_KIND.to_owned(),
        schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
        state_class,
        display_label: display_label.to_owned(),
        taxonomy_state_refs: taxonomy_state_refs.to_vec(),
        token_refs,
        required_non_color_cues: required_non_color_cues.to_vec(),
        hue_only_allowed: false,
        spinner_only_allowed: false,
        hover_only_critical_action_allowed: false,
        persistent_disclosure_required: true,
        screen_reader_label: screen_reader_label.to_owned(),
        copy_rule: copy_rule.to_owned(),
    }
}

fn seeded_cue_families() -> Vec<CueFamilyRow> {
    use CanonicalStateClass as State;
    use CueFamilyClass as Family;
    use CueKind as Kind;
    use NonColorCueClass as Cue;

    let mut families = Vec::new();
    for family in CueFamilyClass::required() {
        for cue_kind in [Kind::Badge, Kind::Notice] {
            let tokens = match *family {
                Family::Lifecycle => vec![
                    cue_token("ready", "Ready", State::Completed, status_success_refs()),
                    cue_token("preview", "Preview", State::Pending, status_info_refs()),
                    cue_token(
                        "deprecated",
                        "Deprecated",
                        State::Degraded,
                        status_warning_refs(),
                    ),
                ],
                Family::Route => vec![
                    cue_token("local", "Local", State::Completed, status_success_refs()),
                    cue_token("remote", "Remote", State::Pending, status_info_refs()),
                    cue_token(
                        "unavailable",
                        "Unavailable",
                        State::Blocked,
                        status_danger_refs(),
                    ),
                ],
                Family::Readiness => vec![
                    cue_token("ready", "Ready", State::Completed, status_success_refs()),
                    cue_token("partial", "Partial", State::Degraded, status_warning_refs()),
                    cue_token("blocked", "Blocked", State::Blocked, status_danger_refs()),
                ],
                Family::Policy => vec![
                    cue_token(
                        "allowed",
                        "Allowed",
                        State::Completed,
                        status_success_refs(),
                    ),
                    cue_token("limited", "Limited", State::Degraded, status_warning_refs()),
                    cue_token("locked", "Locked", State::Blocked, status_danger_refs()),
                ],
            };
            families.push(CueFamilyRow {
                record_kind: CUE_FAMILY_RECORD_KIND.to_owned(),
                schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
                family_class: *family,
                cue_kind,
                honesty_fallback_token: match *family {
                    Family::Lifecycle => "deprecated",
                    Family::Readiness => "partial",
                    Family::Route => "unavailable",
                    Family::Policy => "limited",
                }
                .to_owned(),
                requires_text_shape_fallback: true,
                required_non_color_cues: vec![Cue::LabelText, Cue::Shape],
                vocabulary_tokens: tokens,
            });
        }
    }
    families
}

fn cue_token(
    token: &str,
    label: &str,
    state_class_ref: CanonicalStateClass,
    token_refs: StateTokenRefs,
) -> CueVocabularyToken {
    CueVocabularyToken {
        token: token.to_owned(),
        label: label.to_owned(),
        state_class_ref,
        token_refs,
        required_non_color_cues: vec![NonColorCueClass::LabelText, NonColorCueClass::Shape],
    }
}

fn status_info_refs() -> StateTokenRefs {
    StateTokenRefs::new("status.info", "status.info.border", "status.info.fill")
}

fn status_success_refs() -> StateTokenRefs {
    StateTokenRefs::new(
        "status.success",
        "status.success.border",
        "status.success.fill",
    )
}

fn status_warning_refs() -> StateTokenRefs {
    StateTokenRefs::new(
        "status.warning",
        "status.warning.border",
        "status.warning.fill",
    )
}

fn status_danger_refs() -> StateTokenRefs {
    StateTokenRefs::new(
        "status.danger",
        "status.danger.border",
        "status.danger.fill",
    )
}

fn seeded_launch_surface_consumers() -> Vec<LaunchSurfaceConsumerRow> {
    LaunchSurfaceClass::required()
        .iter()
        .map(|surface| surface_consumer_row(*surface))
        .collect()
}

fn surface_consumer_row(surface_class: LaunchSurfaceClass) -> LaunchSurfaceConsumerRow {
    let consumer_refs = match surface_class {
        LaunchSurfaceClass::ShellChrome => vec![
            "crates/aureline-shell/src/bootstrap/native_shell.rs",
            "crates/aureline-shell/src/token_state_audit/mod.rs",
        ],
        LaunchSurfaceClass::StartCenter => vec![
            "docs/ux/m3/start_center_and_workspace_switcher_beta.md",
            "crates/aureline-shell/src/token_state_audit/mod.rs",
        ],
        LaunchSurfaceClass::CommandPalette => vec![
            "docs/ux/m3/command_palette_diagnostics_beta.md",
            "crates/aureline-shell/src/token_state_audit/mod.rs",
        ],
        LaunchSurfaceClass::SearchSurface => vec![
            "docs/ux/quick_open_contract.md",
            "crates/aureline-search/src/lib.rs",
        ],
        LaunchSurfaceClass::DialogSheet => vec![
            "docs/ux/dialog_sheet_contract.md",
            "crates/aureline-shell/src/token_state_audit/mod.rs",
        ],
        LaunchSurfaceClass::TrustPrompt => vec![
            "docs/ux/trust_prompt_contract.md",
            "crates/aureline-shell/src/token_state_audit/mod.rs",
        ],
        LaunchSurfaceClass::NotificationEnvelope => vec![
            "docs/ux/notification_envelope_contract.md",
            "crates/aureline-shell/src/token_state_audit/mod.rs",
        ],
        LaunchSurfaceClass::HelpAboutRow => vec![
            "docs/ux/m3/support_center_beta.md",
            "crates/aureline-docs/src/lib.rs",
        ],
        LaunchSurfaceClass::SettingsRoot => vec![
            "docs/ux/m3/settings_ui_beta.md",
            "crates/aureline-settings/src/lib.rs",
        ],
        LaunchSurfaceClass::ActivityCenterRow => vec![
            "docs/ux/m3/activity_center_beta.md",
            "crates/aureline-shell/src/token_state_audit/mod.rs",
        ],
    };

    LaunchSurfaceConsumerRow {
        record_kind: LAUNCH_SURFACE_CONSUMER_RECORD_KIND.to_owned(),
        schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
        surface_class,
        launch_priority: LaunchPriorityClass::LaunchCritical,
        consumer_refs: consumer_refs.into_iter().map(str::to_owned).collect(),
        consumes_component_states: CanonicalStateClass::required().to_vec(),
        consumes_cue_families: CueFamilyClass::required().to_vec(),
        required_theme_classes: required_theme_classes(),
        required_density_classes: required_density_classes(),
        required_motion_postures: required_motion_postures(),
        critical_actions_keyboard_reachable: true,
        focus_visibility_preserved: true,
        no_color_only_state_meaning: true,
        waiver_ref: None,
    }
}

fn required_theme_classes() -> Vec<ThemeClass> {
    vec![
        ThemeClass::DarkReference,
        ThemeClass::LightParity,
        ThemeClass::HighContrastDark,
        ThemeClass::HighContrastLight,
    ]
}

fn required_density_classes() -> Vec<DensityClass> {
    vec![
        DensityClass::Compact,
        DensityClass::Standard,
        DensityClass::Comfortable,
    ]
}

fn required_motion_postures() -> Vec<AccessibilityPostureClass> {
    vec![
        AccessibilityPostureClass::MotionStandard,
        AccessibilityPostureClass::MotionReduced,
        AccessibilityPostureClass::MotionLowMotion,
        AccessibilityPostureClass::MotionPowerSaver,
        AccessibilityPostureClass::MotionCriticalHotPath,
    ]
}

/// Validates the seeded component-state registry contract.
pub fn validate_component_state_registry(
    registry: &ComponentStateRegistryRecord,
) -> Result<(), Vec<DesignSystemFinding>> {
    let findings = audit_component_state_registry(registry);
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

/// Audits the component-state registry and returns all findings.
pub fn audit_component_state_registry(
    registry: &ComponentStateRegistryRecord,
) -> Vec<DesignSystemFinding> {
    let mut findings = Vec::new();
    if registry.record_kind != COMPONENT_STATE_REGISTRY_RECORD_KIND {
        findings.push(DesignSystemFinding::error(
            "registry.record_kind.invalid",
            None,
            "record_kind",
            "component-state registry record kind is invalid",
        ));
    }
    if registry.schema_version != DESIGN_SYSTEM_BETA_SCHEMA_VERSION {
        findings.push(DesignSystemFinding::error(
            "registry.schema_version.invalid",
            None,
            "schema_version",
            "component-state registry schema version is unsupported",
        ));
    }

    let states: BTreeSet<_> = registry
        .component_state_families
        .iter()
        .map(|row| row.state_class)
        .collect();
    for state in CanonicalStateClass::required() {
        if !states.contains(state) {
            findings.push(DesignSystemFinding::error(
                "registry.state.required_missing",
                None,
                state.as_str(),
                "required canonical component state is missing",
            ));
        }
    }

    for row in &registry.component_state_families {
        if row.required_non_color_cues.is_empty() {
            findings.push(DesignSystemFinding::error(
                "registry.state.non_color_cues_missing",
                None,
                row.state_class.as_str(),
                "component state has no required non-color cues",
            ));
        }
        if row.hue_only_allowed {
            findings.push(DesignSystemFinding::error(
                "registry.state.hue_only_allowed",
                None,
                row.state_class.as_str(),
                "component state permits hue-only meaning",
            ));
        }
        if row.state_class == CanonicalStateClass::Blocked && row.spinner_only_allowed {
            findings.push(DesignSystemFinding::error(
                "registry.state.spinner_only_blocked",
                None,
                row.state_class.as_str(),
                "blocked state may not be represented by a spinner alone",
            ));
        }
        if row.hover_only_critical_action_allowed {
            findings.push(DesignSystemFinding::error(
                "registry.state.hover_only_critical_action",
                None,
                row.state_class.as_str(),
                "critical action or reason may not depend on hover only",
            ));
        }
        for token in row.token_refs.all() {
            append_missing_token_findings(&mut findings, token, row.state_class.as_str());
        }
    }

    let mut cue_pairs = BTreeSet::new();
    for family in &registry.cue_families {
        cue_pairs.insert((family.family_class, family.cue_kind));
        if !family.requires_text_shape_fallback {
            findings.push(DesignSystemFinding::error(
                "registry.cue_family.text_shape_fallback_missing",
                None,
                format!(
                    "{}.{}",
                    family.family_class.as_str(),
                    family.cue_kind.as_str()
                ),
                "cue family does not require text and shape fallback",
            ));
        }
        if !family
            .vocabulary_tokens
            .iter()
            .any(|token| token.token == family.honesty_fallback_token)
        {
            findings.push(DesignSystemFinding::error(
                "registry.cue_family.fallback_missing",
                None,
                family.family_class.as_str(),
                "cue family honesty fallback token is not in its vocabulary",
            ));
        }
    }
    for family in CueFamilyClass::required() {
        for cue_kind in [CueKind::Badge, CueKind::Notice] {
            if !cue_pairs.contains(&(*family, cue_kind)) {
                findings.push(DesignSystemFinding::error(
                    "registry.cue_family.required_missing",
                    None,
                    format!("{}.{}", family.as_str(), cue_kind.as_str()),
                    "required badge or notice family is missing",
                ));
            }
        }
    }

    let surfaces: BTreeSet<_> = registry
        .launch_surface_consumers
        .iter()
        .map(|row| row.surface_class)
        .collect();
    for surface in LaunchSurfaceClass::required() {
        if !surfaces.contains(surface) {
            findings.push(DesignSystemFinding::error(
                "registry.surface.required_missing",
                Some(*surface),
                surface.as_str(),
                "required launch-critical surface is missing",
            ));
        }
    }
    for row in &registry.launch_surface_consumers {
        if row.launch_priority == LaunchPriorityClass::OutOfScope && row.waiver_ref.is_none() {
            findings.push(DesignSystemFinding::error(
                "registry.surface.out_of_scope_without_waiver",
                Some(row.surface_class),
                row.surface_class.as_str(),
                "out-of-scope beta truth rows require a bounded waiver reference",
            ));
        }
        if row.launch_priority != LaunchPriorityClass::OutOfScope {
            if row.consumes_component_states.is_empty() || row.consumes_cue_families.is_empty() {
                findings.push(DesignSystemFinding::error(
                    "registry.surface.canonical_consumption_missing",
                    Some(row.surface_class),
                    row.surface_class.as_str(),
                    "launch-critical surface does not consume canonical state and cue families",
                ));
            }
            if !row.critical_actions_keyboard_reachable {
                findings.push(DesignSystemFinding::error(
                    "registry.surface.hover_only_critical_action",
                    Some(row.surface_class),
                    row.surface_class.as_str(),
                    "critical action is not reachable from keyboard/focus",
                ));
            }
            if !row.focus_visibility_preserved {
                findings.push(DesignSystemFinding::error(
                    "registry.surface.focus_visibility_missing",
                    Some(row.surface_class),
                    row.surface_class.as_str(),
                    "surface does not preserve visible focus",
                ));
            }
            if !row.no_color_only_state_meaning {
                findings.push(DesignSystemFinding::error(
                    "registry.surface.color_only_state_meaning",
                    Some(row.surface_class),
                    row.surface_class.as_str(),
                    "surface allows state meaning to be carried by color alone",
                ));
            }
        }
    }

    findings
}

fn append_missing_token_findings(
    findings: &mut Vec<DesignSystemFinding>,
    token: &str,
    field: &str,
) {
    for theme in required_theme_classes() {
        match seeded_token_registry(theme) {
            Ok(registry) if registry.color(token).is_some() => {}
            Ok(_) => findings.push(DesignSystemFinding::error(
                "registry.token_ref.missing",
                None,
                format!("{field}.{token}.{}", theme.token()),
                "state or cue token reference is not present in a first-party theme registry",
            )),
            Err(err) => findings.push(DesignSystemFinding::error(
                "registry.token_ref.registry_unavailable",
                None,
                format!("{field}.{token}.{}", theme.token()),
                format!("theme token registry could not load: {err}"),
            )),
        }
    }
}

/// Returns the seeded screenshot-diff packet.
pub fn seeded_screenshot_diff_packet() -> ScreenshotDiffPacket {
    let rows = seeded_screenshot_diff_rows();
    let findings = audit_screenshot_diff_rows(&rows);
    let summary = matrix_summary(&rows, findings.len());
    ScreenshotDiffPacket {
        record_kind: SCREENSHOT_DIFF_PACKET_RECORD_KIND.to_owned(),
        schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
        shared_contract_ref: DESIGN_SYSTEM_BETA_SHARED_CONTRACT_REF.to_owned(),
        packet_id: "component-state-screenshot-diff:beta".to_owned(),
        registry_ref: "fixtures/ux/m3/state_semantics/component_state_registry.json".to_owned(),
        packet_label: "Component-state screenshot diff for launch-critical beta rows".to_owned(),
        rows,
        gate_state: gate_for_findings(&findings),
        findings,
        summary,
    }
}

fn seeded_screenshot_diff_rows() -> Vec<ScreenshotDiffRow> {
    use AccessibilityPostureClass as Motion;
    use CanonicalStateClass as State;
    use DensityClass as Density;
    use LaunchSurfaceClass as Surface;
    use NonColorCueClass as Cue;
    use ThemeClass as Theme;

    let row_specs = [
        (
            Surface::ShellChrome,
            State::Loading,
            Theme::DarkReference,
            Density::Standard,
            Motion::MotionStandard,
        ),
        (
            Surface::StartCenter,
            State::Empty,
            Theme::LightParity,
            Density::Comfortable,
            Motion::MotionReduced,
        ),
        (
            Surface::CommandPalette,
            State::Pending,
            Theme::HighContrastDark,
            Density::Compact,
            Motion::MotionLowMotion,
        ),
        (
            Surface::SearchSurface,
            State::Completed,
            Theme::HighContrastLight,
            Density::Standard,
            Motion::MotionPowerSaver,
        ),
        (
            Surface::DialogSheet,
            State::Blocked,
            Theme::DarkReference,
            Density::Comfortable,
            Motion::MotionCriticalHotPath,
        ),
        (
            Surface::TrustPrompt,
            State::Error,
            Theme::LightParity,
            Density::Standard,
            Motion::MotionStandard,
        ),
        (
            Surface::NotificationEnvelope,
            State::Degraded,
            Theme::HighContrastDark,
            Density::Comfortable,
            Motion::MotionReduced,
        ),
        (
            Surface::HelpAboutRow,
            State::Completed,
            Theme::HighContrastLight,
            Density::Compact,
            Motion::MotionLowMotion,
        ),
        (
            Surface::SettingsRoot,
            State::Blocked,
            Theme::DarkReference,
            Density::Compact,
            Motion::MotionPowerSaver,
        ),
        (
            Surface::ActivityCenterRow,
            State::Pending,
            Theme::LightParity,
            Density::Standard,
            Motion::MotionCriticalHotPath,
        ),
    ];

    row_specs
        .into_iter()
        .map(|(surface, state, theme, density, posture)| {
            let slug = format!(
                "{}.{}.{}.{}.{}",
                surface.as_str(),
                state.as_str(),
                theme.token(),
                density.token(),
                posture.token()
            );
            ScreenshotDiffRow {
                row_id: format!("component-state-diff:{slug}"),
                surface_class: surface,
                state_class: state,
                theme_class: theme,
                density_class: density,
                motion_posture: posture,
                baseline_capture_ref: format!(
                    "artifacts/ux/m3/component_state_screenshot_diff/packet.json#baseline/{slug}"
                ),
                comparison_capture_ref: format!(
                    "artifacts/ux/m3/component_state_screenshot_diff/packet.json#comparison/{slug}"
                ),
                diff_artifact_ref: format!(
                    "artifacts/ux/m3/component_state_screenshot_diff/packet.json#diff/{slug}"
                ),
                keyboard_journey_ref: format!(
                    "fixtures/ux/m3/state_semantics/screenshot_diff_matrix.json#keyboard/{slug}"
                ),
                assistive_technology_ref: format!(
                    "fixtures/ux/m3/state_semantics/screenshot_diff_matrix.json#at/{slug}"
                ),
                token_conformance_ref:
                    "fixtures/ux/m3/state_semantics/token_conformance_report.json".to_owned(),
                required_non_color_cues: match state {
                    State::Blocked => vec![Cue::LabelText, Cue::Icon, Cue::LockOrShieldGlyph],
                    State::Loading => vec![Cue::LabelText, Cue::ProgressIndicator],
                    State::Completed => vec![Cue::LabelText, Cue::CheckOrSelectionMarker],
                    State::Degraded | State::Error => vec![Cue::LabelText, Cue::Icon, Cue::Border],
                    State::Empty | State::Pending => vec![Cue::LabelText, Cue::Icon],
                },
                hover_only_critical_actions_absent: true,
                focus_visibility_present: true,
                spinner_only_blocked_state_absent: true,
                color_only_state_meaning_absent: true,
                semantic_stability_passed: true,
            }
        })
        .collect()
}

/// Validates a screenshot-diff packet.
pub fn validate_screenshot_diff_packet(
    packet: &ScreenshotDiffPacket,
) -> Result<(), Vec<DesignSystemFinding>> {
    let findings = audit_screenshot_diff_rows(&packet.rows);
    if findings.is_empty()
        && packet.findings.is_empty()
        && packet.gate_state == GateStateClass::Pass
    {
        Ok(())
    } else {
        let mut all = packet.findings.clone();
        all.extend(findings);
        Err(all)
    }
}

fn audit_screenshot_diff_rows(rows: &[ScreenshotDiffRow]) -> Vec<DesignSystemFinding> {
    let mut findings = Vec::new();
    let surfaces: BTreeSet<_> = rows.iter().map(|row| row.surface_class).collect();
    let states: BTreeSet<_> = rows.iter().map(|row| row.state_class).collect();
    let themes = unique_themes(rows);
    let densities = unique_densities(rows);
    let postures = unique_postures(rows);

    for surface in LaunchSurfaceClass::required() {
        if !surfaces.contains(surface) {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.surface.required_missing",
                Some(*surface),
                surface.as_str(),
                "screenshot diff matrix is missing a launch-critical surface",
            ));
        }
    }
    for state in CanonicalStateClass::required() {
        if !states.contains(state) {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.state.required_missing",
                None,
                state.as_str(),
                "screenshot diff matrix is missing a canonical state",
            ));
        }
    }
    for theme in required_theme_classes() {
        if !themes.contains(&theme) {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.theme.required_missing",
                None,
                theme.token(),
                "screenshot diff matrix is missing a first-party theme class",
            ));
        }
    }
    for density in required_density_classes() {
        if !densities.contains(&density) {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.density.required_missing",
                None,
                density.token(),
                "screenshot diff matrix is missing a density class",
            ));
        }
    }
    for posture in required_motion_postures() {
        if !postures.contains(&posture) {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.motion_posture.required_missing",
                None,
                posture.token(),
                "screenshot diff matrix is missing a motion posture",
            ));
        }
    }

    for row in rows {
        if row.required_non_color_cues.is_empty() || !row.color_only_state_meaning_absent {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.color_only_state_meaning",
                Some(row.surface_class),
                &row.row_id,
                "state meaning is missing non-color evidence",
            ));
        }
        if !row.hover_only_critical_actions_absent {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.hover_only_critical_action",
                Some(row.surface_class),
                &row.row_id,
                "critical action depends on hover-only discovery",
            ));
        }
        if !row.focus_visibility_present {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.focus_visibility_missing",
                Some(row.surface_class),
                &row.row_id,
                "focused row has no visible focus evidence",
            ));
        }
        if row.state_class == CanonicalStateClass::Blocked && !row.spinner_only_blocked_state_absent
        {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.spinner_only_blocked_state",
                Some(row.surface_class),
                &row.row_id,
                "blocked row is represented by a spinner alone",
            ));
        }
        if !row.semantic_stability_passed {
            findings.push(DesignSystemFinding::error(
                "screenshot_diff.semantic_stability_failed",
                Some(row.surface_class),
                &row.row_id,
                "screenshot diff changed the state meaning",
            ));
        }
    }
    findings
}

fn matrix_summary(rows: &[ScreenshotDiffRow], finding_count: usize) -> MatrixSummary {
    MatrixSummary {
        row_count: rows.len(),
        surface_count: rows
            .iter()
            .map(|row| row.surface_class)
            .collect::<BTreeSet<_>>()
            .len(),
        state_count: rows
            .iter()
            .map(|row| row.state_class)
            .collect::<BTreeSet<_>>()
            .len(),
        theme_count: unique_themes(rows).len(),
        density_count: unique_densities(rows).len(),
        motion_posture_count: unique_postures(rows).len(),
        finding_count,
    }
}

fn unique_themes(rows: &[ScreenshotDiffRow]) -> Vec<ThemeClass> {
    let mut values = Vec::new();
    for row in rows {
        if !values.contains(&row.theme_class) {
            values.push(row.theme_class);
        }
    }
    values
}

fn unique_densities(rows: &[ScreenshotDiffRow]) -> Vec<DensityClass> {
    let mut values = Vec::new();
    for row in rows {
        if !values.contains(&row.density_class) {
            values.push(row.density_class);
        }
    }
    values
}

fn unique_postures(rows: &[ScreenshotDiffRow]) -> Vec<AccessibilityPostureClass> {
    let mut values = Vec::new();
    for row in rows {
        if !values.contains(&row.motion_posture) {
            values.push(row.motion_posture);
        }
    }
    values
}

fn gate_for_findings(findings: &[DesignSystemFinding]) -> GateStateClass {
    if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Error)
    {
        GateStateClass::Block
    } else if findings
        .iter()
        .any(|finding| finding.severity == FindingSeverity::Warning)
    {
        GateStateClass::Warn
    } else {
        GateStateClass::Pass
    }
}

/// Returns the seeded token-conformance packet.
pub fn seeded_token_conformance_packet() -> TokenConformancePacket {
    let registry = seeded_component_state_registry();
    let mut rows = Vec::new();
    for surface in &registry.launch_surface_consumers {
        rows.push(TokenConformanceRow {
            row_id: format!("token-conformance:{}", surface.surface_class.as_str()),
            surface_class: surface.surface_class,
            launch_priority: surface.launch_priority,
            source_refs: surface.consumer_refs.clone(),
            required_color_token_families: vec![
                "al.color.".to_owned(),
                "status.".to_owned(),
                "trust.".to_owned(),
            ],
            required_geometry_token_families: vec!["size.".to_owned(), "space.".to_owned()],
            required_motion_token_families: vec!["motion.".to_owned()],
            required_component_states: surface.consumes_component_states.clone(),
            required_cue_families: surface.consumes_cue_families.clone(),
            raw_color_literals_forbidden: true,
            local_token_forks_forbidden: true,
            no_waiver_required: surface.waiver_ref.is_none(),
            waiver_ref: surface.waiver_ref.clone(),
            findings: Vec::new(),
        });
    }
    let findings = audit_token_conformance_rows(&rows);
    TokenConformancePacket {
        record_kind: TOKEN_CONFORMANCE_PACKET_RECORD_KIND.to_owned(),
        schema_version: DESIGN_SYSTEM_BETA_SCHEMA_VERSION,
        shared_contract_ref: DESIGN_SYSTEM_BETA_SHARED_CONTRACT_REF.to_owned(),
        packet_id: "token-conformance:launch-critical-beta".to_owned(),
        registry_ref: "fixtures/ux/m3/state_semantics/component_state_registry.json".to_owned(),
        screenshot_diff_packet_ref: "artifacts/ux/m3/component_state_screenshot_diff/packet.json"
            .to_owned(),
        shell_token_state_audit_ref: "artifacts/ux/m3/token_state_audit.md".to_owned(),
        rows,
        summary: MatrixSummary {
            row_count: LaunchSurfaceClass::required().len(),
            surface_count: LaunchSurfaceClass::required().len(),
            state_count: CanonicalStateClass::required().len(),
            theme_count: required_theme_classes().len(),
            density_count: required_density_classes().len(),
            motion_posture_count: required_motion_postures().len(),
            finding_count: findings.len(),
        },
        gate_state: gate_for_findings(&findings),
        findings,
        raw_private_material_excluded: true,
    }
}

/// Validates a token-conformance packet.
pub fn validate_token_conformance_packet(
    packet: &TokenConformancePacket,
) -> Result<(), Vec<DesignSystemFinding>> {
    let findings = audit_token_conformance_rows(&packet.rows);
    if findings.is_empty()
        && packet.findings.is_empty()
        && packet.gate_state == GateStateClass::Pass
    {
        Ok(())
    } else {
        let mut all = packet.findings.clone();
        all.extend(findings);
        Err(all)
    }
}

fn audit_token_conformance_rows(rows: &[TokenConformanceRow]) -> Vec<DesignSystemFinding> {
    let mut findings = Vec::new();
    let surfaces: BTreeSet<_> = rows.iter().map(|row| row.surface_class).collect();
    for surface in LaunchSurfaceClass::required() {
        if !surfaces.contains(surface) {
            findings.push(DesignSystemFinding::error(
                "token_conformance.surface.required_missing",
                Some(*surface),
                surface.as_str(),
                "token conformance packet is missing a launch-critical surface",
            ));
        }
    }
    for row in rows {
        if row.launch_priority != LaunchPriorityClass::OutOfScope {
            if row.required_component_states.is_empty() || row.required_cue_families.is_empty() {
                findings.push(DesignSystemFinding::error(
                    "token_conformance.canonical_consumption_missing",
                    Some(row.surface_class),
                    &row.row_id,
                    "row does not require canonical state and cue consumption",
                ));
            }
            if !row.raw_color_literals_forbidden {
                findings.push(DesignSystemFinding::error(
                    "token_conformance.raw_color_literals_allowed",
                    Some(row.surface_class),
                    &row.row_id,
                    "launch-critical row allows raw color literals",
                ));
            }
            if !row.local_token_forks_forbidden {
                findings.push(DesignSystemFinding::error(
                    "token_conformance.local_token_forks_allowed",
                    Some(row.surface_class),
                    &row.row_id,
                    "launch-critical row allows local token forks",
                ));
            }
        } else if row.waiver_ref.is_none() {
            findings.push(DesignSystemFinding::error(
                "token_conformance.out_of_scope_without_waiver",
                Some(row.surface_class),
                &row.row_id,
                "out-of-scope token conformance rows require a waiver",
            ));
        }
        if !row.findings.is_empty() {
            findings.extend(row.findings.clone());
        }
    }
    findings
}

/// Runs all beta contract validations and returns the findings grouped by lane.
pub fn audit_seeded_beta_contract() -> BTreeMap<String, Vec<DesignSystemFinding>> {
    let mut findings = BTreeMap::new();
    let registry = seeded_component_state_registry();
    let screenshot = seeded_screenshot_diff_packet();
    let conformance = seeded_token_conformance_packet();

    findings.insert(
        "component_state_registry".to_owned(),
        audit_component_state_registry(&registry),
    );
    findings.insert(
        "component_state_screenshot_diff".to_owned(),
        audit_screenshot_diff_rows(&screenshot.rows),
    );
    findings.insert(
        "token_conformance".to_owned(),
        audit_token_conformance_rows(&conformance.rows),
    );

    findings
}

/// Validates all seeded beta contract records.
pub fn validate_seeded_beta_contract() -> Result<(), BTreeMap<String, Vec<DesignSystemFinding>>> {
    let findings = audit_seeded_beta_contract();
    if findings.values().all(Vec::is_empty) {
        Ok(())
    } else {
        Err(findings)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_registry_validates() {
        let registry = seeded_component_state_registry();
        validate_component_state_registry(&registry).expect("registry validates");
        assert_eq!(
            registry.component_state_families.len(),
            CanonicalStateClass::required().len()
        );
    }

    #[test]
    fn registry_flags_missing_state() {
        let mut registry = seeded_component_state_registry();
        registry
            .component_state_families
            .retain(|row| row.state_class != CanonicalStateClass::Blocked);
        let findings = audit_component_state_registry(&registry);
        assert!(findings
            .iter()
            .any(|finding| finding.check_id == "registry.state.required_missing"));
    }

    #[test]
    fn registry_flags_hue_only_state() {
        let mut registry = seeded_component_state_registry();
        registry.component_state_families[0].hue_only_allowed = true;
        let findings = audit_component_state_registry(&registry);
        assert!(findings
            .iter()
            .any(|finding| finding.check_id == "registry.state.hue_only_allowed"));
    }

    #[test]
    fn screenshot_packet_covers_axes() {
        let packet = seeded_screenshot_diff_packet();
        validate_screenshot_diff_packet(&packet).expect("screenshot packet validates");
        assert_eq!(packet.gate_state, GateStateClass::Pass);
        assert_eq!(
            packet.summary.surface_count,
            LaunchSurfaceClass::required().len()
        );
        assert_eq!(
            packet.summary.state_count,
            CanonicalStateClass::required().len()
        );
    }

    #[test]
    fn screenshot_packet_flags_hover_only_action() {
        let mut packet = seeded_screenshot_diff_packet();
        packet.rows[0].hover_only_critical_actions_absent = false;
        let findings = audit_screenshot_diff_rows(&packet.rows);
        assert!(findings
            .iter()
            .any(|finding| finding.check_id == "screenshot_diff.hover_only_critical_action"));
    }

    #[test]
    fn token_conformance_packet_validates() {
        let packet = seeded_token_conformance_packet();
        validate_token_conformance_packet(&packet).expect("token conformance validates");
        assert!(packet.raw_private_material_excluded);
    }

    #[test]
    fn token_conformance_flags_local_token_fork() {
        let mut packet = seeded_token_conformance_packet();
        packet.rows[0].local_token_forks_forbidden = false;
        let findings = audit_token_conformance_rows(&packet.rows);
        assert!(findings
            .iter()
            .any(|finding| finding.check_id == "token_conformance.local_token_forks_allowed"));
    }
}
