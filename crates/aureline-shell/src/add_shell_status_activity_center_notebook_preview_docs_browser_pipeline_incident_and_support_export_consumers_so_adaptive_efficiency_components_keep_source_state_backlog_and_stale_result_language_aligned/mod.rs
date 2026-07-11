//! Shared shell-status / activity-center, notebook / preview / pipeline / graph
//! work-content, docs-browser / companion-adjacent, incident / diagnostics, and
//! support / export consumers for the frozen M5 adaptive-efficiency components.
//!
//! This module is the M05-1066 consumer-adoption lane over the frozen M5
//! adaptive-efficiency component matrix
//! ([`crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix`]).
//! Where the freeze matrix defines the eight reusable power-state indicator,
//! throttled-subsystem row, background-work row / banner, per-workspace override
//! sheet, override-policy note row, resume-summary card, and stale-result
//! continuity note primitives — and the four B126 implement lanes wire their
//! resolvers and controls contracts — this lane proves those families are
//! reusable *primitives* rather than per-surface low-power prose. It adopts them
//! across five claimed M5 adaptive-efficiency consumer classes:
//!
//! 1. a shell-status / activity-center / background-work surface,
//! 2. a notebook / preview / pipeline / graph work-content surface (the work
//!    that actually slows or pauses under pressure),
//! 3. a docs-browser / companion-adjacent handoff surface,
//! 4. an incident / diagnostics surface, and
//! 5. a support / export + Help/About lane (AC2).
//!
//! Each [`EfficiencyConsumerRow`] points back to exactly one canonical component
//! family (its per-family matrix schema) and the one canonical controls contract
//! (schema + doc + release-proof artifact) its family group belongs to, instead
//! of cloning surface-local efficiency vocabulary. Every consumer — even a
//! read-only, inspect-only, export-only, or docs reference — keeps the identical
//! source-of-change / active-efficiency-state / slowed-versus-paused /
//! what-still-works / override-availability / policy-owner / resumed-work-backlog
//! / stale-result-continuity / next-safe-action labels and the identical frozen
//! work-disposition vocabulary. A narrower consumer discloses the reduction with
//! a reduced-capability banner (and, when it punts to another surface, a
//! desktop / companion / browser / support-packet note) rather than renaming or
//! dropping governed state, so notebook, preview, docs, incident, and support
//! lanes never fork efficiency vocabulary by surface. This is what makes the same
//! constrained *or recovered* state render with one vocabulary and one component
//! family across every claimed consumer (AC1), and lets Help/support/export
//! consumers drop bespoke per-lane prose (AC2).
//!
//! The four spec guardrails are enforced per row and must all stay false: no
//! consumer collapses battery saver, thermal pressure, user-selected low-power
//! mode, and policy cap into one generic warning; no consumer hides paused work
//! behind toast-only messaging; no consumer presents an override as available
//! when policy blocks it; no consumer clears stale-result context merely because
//! background work resumed.
//!
//! The packet is metadata-only: raw battery / thermal telemetry, workspace
//! secrets, and scheduler cursors never cross this boundary; the packet carries
//! only typed class tokens, opaque constrained-state refs, booleans, and redacted
//! labels.
//!
//! The boundary schema is
//! [`schemas/ui/m5-efficiency-component-consumer.schema.json`](../../../../schemas/ui/m5-efficiency-component-consumer.schema.json).
//! The contract doc is
//! [`docs/help/m5_efficiency_component_consumer_contract.md`](../../../../docs/help/m5_efficiency_component_consumer_contract.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_power_state_indicator_throttled_subsystem_row_background_work_row_background_work_banner_per_workspace_override_sheet_override_policy_note_row_resume_summary_card_and_stale_result_continuity_note_component_matrix as matrix;
use crate::implement_the_m5_background_work_row_and_background_work_banner_affected_work_class_state_what_still_works_resume_condition_and_override_primitive as background_work_controls;
use crate::implement_the_m5_per_workspace_override_sheet_and_override_policy_note_row_current_mode_ceilings_expected_effect_reset_path_and_blocked_by_policy_primitive as override_controls;
use crate::implement_the_m5_power_state_indicator_and_throttled_subsystem_row_source_active_state_affected_subsystem_and_inspect_path_primitive as power_throttle_controls;
use crate::implement_the_m5_resume_summary_card_and_stale_result_continuity_note_resumed_work_backlog_state_stale_results_visible_and_next_safe_action_primitive as resume_controls;

pub use matrix::{M5EfficiencyComponentFamily, M5EfficiencyWorkDisposition};

/// Schema version stamped on the M05-1066 consumer packet.
pub const EFFICIENCY_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EfficiencyConsumerPacket`].
pub const EFFICIENCY_CONSUMER_RECORD_KIND: &str = "m5_efficiency_component_consumer_packet";

/// Stable record-kind tag carried by each [`EfficiencyConsumerRow`].
pub const EFFICIENCY_CONSUMER_ROW_RECORD_KIND: &str = "m5_efficiency_component_consumer_row";

/// Repo-relative path of the boundary schema.
pub const EFFICIENCY_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-efficiency-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const EFFICIENCY_CONSUMER_DOC_REF: &str =
    "docs/help/m5_efficiency_component_consumer_contract.md";

/// Repo-relative path of the frozen adaptive-efficiency component matrix
/// release proof these consumers adopt.
pub const EFFICIENCY_CONSUMER_MATRIX_REF: &str = matrix::M5_EFFICIENCY_COMPONENT_ARTIFACT_REF;

/// Repo-relative path of the shared frozen component-matrix schema.
pub const EFFICIENCY_CONSUMER_SHARED_SCHEMA_REF: &str = matrix::M5_EFFICIENCY_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EFFICIENCY_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-efficiency-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EFFICIENCY_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-efficiency-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EFFICIENCY_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-efficiency-component-consumer-proof/report.md";

/// Repo-relative path of the checked consumer-fixture directory.
pub const EFFICIENCY_CONSUMER_FIXTURE_DIR: &str = "fixtures/ui/m5-efficiency-component-consumers";

/// The controlled label families a consumer must preserve identically across
/// every surface. These are the track-invariant truth pillars of the
/// adaptive-efficiency components: the source of change, the active efficiency
/// state, whether work slowed versus paused, what still works, override
/// availability, the policy owner, the resumed-work backlog, stale-result
/// continuity, and the next safe action. The union of every row's
/// `preserved_label_families` must cover this set.
pub const REQUIRED_LABEL_FAMILIES: [&str; 9] = [
    "source_of_change",
    "active_efficiency_state",
    "slowed_versus_paused_work",
    "what_still_works",
    "override_availability",
    "policy_owner",
    "resumed_work_backlog",
    "stale_result_continuity",
    "next_safe_action",
];

/// The canonical work-disposition vocabulary every consumer keeps visible even
/// when narrowed or export-only — the frozen `M5EfficiencyWorkDisposition` set
/// (running-full / slowed / paused / policy-blocked / override-available /
/// override-blocked / resuming / stale-result-shown / not-evaluated). Every
/// consumer renders the same constrained or recovered state with these exact
/// tokens rather than surface-local phrasing (AC1).
pub fn canonical_work_disposition_vocab() -> Vec<String> {
    M5EfficiencyWorkDisposition::ALL
        .iter()
        .map(|d| d.as_str().to_owned())
        .collect()
}

/// Whether a token is one of the frozen work-disposition tokens.
pub fn is_canonical_work_disposition(token: &str) -> bool {
    M5EfficiencyWorkDisposition::ALL
        .iter()
        .any(|d| d.as_str() == token)
}

/// The canonical per-family matrix schema that defines a family's contract.
pub fn canonical_family_schema_ref_for(family: M5EfficiencyComponentFamily) -> &'static str {
    family.canonical_component_schema_ref()
}

/// The four B126 controls contracts the eight component families group into. A
/// consumer must point at the one canonical controls contract for its family's
/// lane rather than inventing a surface-local one — this is the heart of the
/// "efficiency surfaces no longer fork vocabulary" acceptance criterion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EfficiencyControlsLane {
    /// Power-state indicator + throttled-subsystem row controls (M05-1061).
    PowerThrottle,
    /// Background-work row + banner controls (M05-1062).
    BackgroundWork,
    /// Per-workspace override sheet + override-policy note row controls
    /// (M05-1063).
    OverridePolicy,
    /// Resume-summary card + stale-result continuity note controls (M05-1064).
    ResumeContinuity,
}

impl M5EfficiencyControlsLane {
    /// Every controls lane, in declaration order.
    pub const ALL: [M5EfficiencyControlsLane; 4] = [
        M5EfficiencyControlsLane::PowerThrottle,
        M5EfficiencyControlsLane::BackgroundWork,
        M5EfficiencyControlsLane::OverridePolicy,
        M5EfficiencyControlsLane::ResumeContinuity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PowerThrottle => "power_throttle",
            Self::BackgroundWork => "background_work",
            Self::OverridePolicy => "override_policy",
            Self::ResumeContinuity => "resume_continuity",
        }
    }

    /// The canonical controls schema every surface reuses for this lane.
    pub const fn canonical_schema_ref(self) -> &'static str {
        match self {
            Self::PowerThrottle => power_throttle_controls::M5_POWER_THROTTLE_CONTROLS_SCHEMA_REF,
            Self::BackgroundWork => {
                background_work_controls::M5_BACKGROUND_WORK_CONTROLS_SCHEMA_REF
            }
            Self::OverridePolicy => override_controls::M5_OVERRIDE_CONTROLS_SCHEMA_REF,
            Self::ResumeContinuity => resume_controls::M5_RESUME_CONTROLS_SCHEMA_REF,
        }
    }

    /// The canonical controls contract doc for this lane.
    pub const fn canonical_doc_ref(self) -> &'static str {
        match self {
            Self::PowerThrottle => power_throttle_controls::M5_POWER_THROTTLE_CONTROLS_DOC_REF,
            Self::BackgroundWork => background_work_controls::M5_BACKGROUND_WORK_CONTROLS_DOC_REF,
            Self::OverridePolicy => override_controls::M5_OVERRIDE_CONTROLS_DOC_REF,
            Self::ResumeContinuity => resume_controls::M5_RESUME_CONTROLS_DOC_REF,
        }
    }

    /// The canonical controls release-proof artifact every consumer points back
    /// to as the first-resolved truth for this lane.
    pub const fn canonical_artifact_ref(self) -> &'static str {
        match self {
            Self::PowerThrottle => power_throttle_controls::M5_POWER_THROTTLE_CONTROLS_ARTIFACT_REF,
            Self::BackgroundWork => {
                background_work_controls::M5_BACKGROUND_WORK_CONTROLS_ARTIFACT_REF
            }
            Self::OverridePolicy => override_controls::M5_OVERRIDE_CONTROLS_ARTIFACT_REF,
            Self::ResumeContinuity => resume_controls::M5_RESUME_CONTROLS_ARTIFACT_REF,
        }
    }
}

/// The one controls lane a component family belongs to. The eight frozen
/// families group into the four B126 controls contracts; a consumer must reuse
/// the lane's canonical contract rather than forking it per surface.
pub const fn controls_lane_for(family: M5EfficiencyComponentFamily) -> M5EfficiencyControlsLane {
    use M5EfficiencyComponentFamily::*;
    match family {
        PowerStateIndicator | ThrottledSubsystemRow => M5EfficiencyControlsLane::PowerThrottle,
        BackgroundWorkRow | BackgroundWorkBanner => M5EfficiencyControlsLane::BackgroundWork,
        PerWorkspaceOverrideSheet | OverridePolicyNoteRow => {
            M5EfficiencyControlsLane::OverridePolicy
        }
        ResumeSummaryCard | StaleResultContinuityNote => M5EfficiencyControlsLane::ResumeContinuity,
    }
}

/// The five claimed M5 adaptive-efficiency consumer classes that must each adopt
/// at least one canonical component family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerClass {
    /// A shell-status / activity-center / background-work surface.
    ShellStatusActivity,
    /// A notebook / preview / pipeline / graph work-content surface — the work
    /// that actually slows or pauses under pressure.
    WorkContentSurface,
    /// A docs-browser / companion-adjacent handoff surface.
    DocsBrowserCompanion,
    /// An incident / diagnostics surface.
    IncidentDiagnostics,
    /// A support / export + Help/About lane (AC2).
    SupportExportHelp,
}

impl ConsumerClass {
    /// Every consumer class that must be present for cross-surface reuse.
    pub const ALL: [ConsumerClass; 5] = [
        ConsumerClass::ShellStatusActivity,
        ConsumerClass::WorkContentSurface,
        ConsumerClass::DocsBrowserCompanion,
        ConsumerClass::IncidentDiagnostics,
        ConsumerClass::SupportExportHelp,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellStatusActivity => "shell_status_activity",
            Self::WorkContentSurface => "work_content_surface",
            Self::DocsBrowserCompanion => "docs_browser_companion",
            Self::IncidentDiagnostics => "incident_diagnostics",
            Self::SupportExportHelp => "support_export_help",
        }
    }

    /// True when this class is a work-content surface whose rows must preserve
    /// the slowed-versus-paused and what-still-works truth so a throttled work
    /// surface never drops which work slowed and what still runs.
    pub const fn is_constrained_origin(self) -> bool {
        matches!(self, Self::WorkContentSurface)
    }
}

/// The concrete M5 adaptive-efficiency surface a component is embedded in. Each
/// surface belongs to exactly one [`ConsumerClass`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EfficiencyConsumerSurface {
    /// The shell status bar / power chrome.
    ShellStatusBar,
    /// The activity center.
    ActivityCenter,
    /// The background-work tray.
    BackgroundWorkTray,
    /// The notebook canvas.
    NotebookCanvas,
    /// The preview pane.
    PreviewPane,
    /// The pipeline runner surface.
    PipelineRunner,
    /// The graph / enrichment explorer.
    GraphExplorer,
    /// A docs / browser handoff surface.
    DocsBrowserHandoff,
    /// A companion-adjacent surface.
    CompanionAdjacent,
    /// The incident console.
    IncidentConsole,
    /// The diagnostics panel.
    DiagnosticsPanel,
    /// The support / export replay surface.
    SupportExportReplay,
    /// The Help / About docs reference surface.
    HelpAboutReference,
}

impl EfficiencyConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [EfficiencyConsumerSurface; 13] = [
        EfficiencyConsumerSurface::ShellStatusBar,
        EfficiencyConsumerSurface::ActivityCenter,
        EfficiencyConsumerSurface::BackgroundWorkTray,
        EfficiencyConsumerSurface::NotebookCanvas,
        EfficiencyConsumerSurface::PreviewPane,
        EfficiencyConsumerSurface::PipelineRunner,
        EfficiencyConsumerSurface::GraphExplorer,
        EfficiencyConsumerSurface::DocsBrowserHandoff,
        EfficiencyConsumerSurface::CompanionAdjacent,
        EfficiencyConsumerSurface::IncidentConsole,
        EfficiencyConsumerSurface::DiagnosticsPanel,
        EfficiencyConsumerSurface::SupportExportReplay,
        EfficiencyConsumerSurface::HelpAboutReference,
    ];

    /// The consumer class this surface belongs to.
    pub const fn consumer_class(self) -> ConsumerClass {
        match self {
            Self::ShellStatusBar | Self::ActivityCenter | Self::BackgroundWorkTray => {
                ConsumerClass::ShellStatusActivity
            }
            Self::NotebookCanvas
            | Self::PreviewPane
            | Self::PipelineRunner
            | Self::GraphExplorer => ConsumerClass::WorkContentSurface,
            Self::DocsBrowserHandoff | Self::CompanionAdjacent => {
                ConsumerClass::DocsBrowserCompanion
            }
            Self::IncidentConsole | Self::DiagnosticsPanel => ConsumerClass::IncidentDiagnostics,
            Self::SupportExportReplay | Self::HelpAboutReference => {
                ConsumerClass::SupportExportHelp
            }
        }
    }

    /// True when this surface is a docs / help reference surface (AC2).
    pub const fn is_docs_help(self) -> bool {
        matches!(self, Self::HelpAboutReference)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellStatusBar => "shell_status_bar",
            Self::ActivityCenter => "activity_center",
            Self::BackgroundWorkTray => "background_work_tray",
            Self::NotebookCanvas => "notebook_canvas",
            Self::PreviewPane => "preview_pane",
            Self::PipelineRunner => "pipeline_runner",
            Self::GraphExplorer => "graph_explorer",
            Self::DocsBrowserHandoff => "docs_browser_handoff",
            Self::CompanionAdjacent => "companion_adjacent",
            Self::IncidentConsole => "incident_console",
            Self::DiagnosticsPanel => "diagnostics_panel",
            Self::SupportExportReplay => "support_export_replay",
            Self::HelpAboutReference => "help_about_reference",
        }
    }
}

/// The rendering authority a consumer exercises over a canonical component.
///
/// A consumer may narrow authority (read-only, inspect-only, override-gated,
/// export-only, policy-blocked) but never rename or drop the governed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityMode {
    /// Full-interactive control (act on the efficiency component directly).
    FullInteractive,
    /// Read-only projection of the component.
    ReadOnly,
    /// Inspect-only: read every governed label but take no action.
    InspectOnly,
    /// Override-gated: the override is visible but staged behind an explicit
    /// gate before it applies.
    OverrideGated,
    /// Export-only: reconstruct the component from an export packet.
    ExportOnly,
    /// Policy-blocked: the component is visible but action is gated by policy.
    PolicyBlocked,
}

impl AuthorityMode {
    /// Every authority mode, in declaration order.
    pub const ALL: [AuthorityMode; 6] = [
        AuthorityMode::FullInteractive,
        AuthorityMode::ReadOnly,
        AuthorityMode::InspectOnly,
        AuthorityMode::OverrideGated,
        AuthorityMode::ExportOnly,
        AuthorityMode::PolicyBlocked,
    ];

    /// Returns true when the consumer narrows below full-interactive authority
    /// and therefore must disclose the reduction with a banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullInteractive)
    }

    /// The banner `capability_state` label this authority maps to.
    pub const fn capability_state(self) -> &'static str {
        match self {
            Self::FullInteractive => "full",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
            Self::OverrideGated => "override_gated",
            Self::ExportOnly => "export_only",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The surface a narrower consumer hands off to when it cannot act on the
/// efficiency component locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffTarget {
    /// No handoff: the consumer renders and acts on the component in-place.
    None,
    /// Punt to the desktop shell to act on the efficiency state.
    DesktopShell,
    /// Punt to the companion app.
    CompanionApp,
    /// Punt to a read-only browser surface.
    BrowserReadonly,
    /// Punt to a portable support / export packet.
    SupportPacket,
    /// Punt to a headless CLI.
    CliHeadless,
}

impl HandoffTarget {
    /// Returns true when the consumer punts to another surface and therefore
    /// must carry a desktop / companion / browser / support note.
    pub const fn requires_note(self) -> bool {
        !matches!(self, Self::None)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::DesktopShell => "desktop_shell",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::SupportPacket => "support_packet",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Whether the consumer preserves the canonical component's controlled labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelParityState {
    /// Full label parity across the truth pillars.
    Preserved,
    /// Reduced interactivity, disclosed, but the labels are still preserved.
    DisclosedNarrowed,
    /// A label was renamed, flattened, or dropped (red; blocks review).
    RenamedOrDropped,
}

impl LabelParityState {
    /// Returns true when no controlled label is renamed or dropped.
    pub const fn keeps_labels(self) -> bool {
        !matches!(self, Self::RenamedOrDropped)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::RenamedOrDropped => "renamed_or_dropped",
        }
    }
}

/// The copy / export parity a consumer keeps for the adopted component: the
/// governed labels must be copyable as text / JSON / Markdown, and a
/// screenshot-only export is prohibited (it would lose the machine-readable
/// source / state / disposition identity support and automation need to
/// reconstruct the constrained state).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy formats the consumer offers (must include text / json /
    /// markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The export fields the consumer preserves.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// True when a screenshot-only export is prohibited.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the parity offers text / JSON / Markdown copy and prohibits a
    /// screenshot-only export.
    pub fn is_complete(&self) -> bool {
        let has = |f: &str| self.formats.iter().any(|v| v == f);
        has("text")
            && has("json")
            && has("markdown")
            && !self.export_fields.is_empty()
            && self.screenshot_only_prohibited
    }
}

/// The reduced-capability banner a narrower consumer shows to disclose the
/// control it drops relative to the full desktop efficiency surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReducedCapabilityBanner {
    /// Stable banner id.
    pub banner_id: String,
    /// The visible, non-generic banner label.
    pub visible_label: String,
    /// The capability state; must match the row's `authority_mode`.
    pub capability_state: String,
    /// The capabilities the narrowed surface is missing relative to full.
    #[serde(default)]
    pub missing_capabilities: Vec<String>,
}

/// One consumer adopting one canonical adaptive-efficiency component family on
/// one M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyConsumerRow {
    /// Record kind; must equal [`EFFICIENCY_CONSUMER_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EFFICIENCY_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The claimed consumer class.
    pub consumer_class: ConsumerClass,
    /// The concrete surface; must belong to `consumer_class`.
    pub consumer_surface: EfficiencyConsumerSurface,
    /// The single canonical component family this consumer reuses.
    pub component_family: M5EfficiencyComponentFamily,
    /// The controls lane the family belongs to; must equal
    /// `controls_lane_for(component_family)`.
    pub controls_lane: M5EfficiencyControlsLane,
    /// The canonical per-family matrix schema. Must equal
    /// `canonical_family_schema_ref_for(component_family)`.
    pub canonical_family_schema_ref: String,
    /// The canonical controls schema for the lane. Must equal
    /// `controls_lane.canonical_schema_ref()`.
    pub canonical_controls_schema_ref: String,
    /// The canonical controls release-proof artifact(s) this consumer points
    /// back to. Must contain `controls_lane.canonical_artifact_ref()`.
    #[serde(default)]
    pub canonical_controls_artifact_refs: Vec<String>,
    /// True when the consumer references the canonical family + controls lane
    /// rather than cloning surface-local efficiency prose.
    pub references_canonical_not_local_prose: bool,
    /// An opaque, redaction-safe ref to the constrained / recovered state the
    /// user saw, so support and automation can reconstruct it without leaking
    /// raw battery / thermal telemetry.
    pub constrained_state_ref: String,
    /// The rendering authority the consumer exercises.
    pub authority_mode: AuthorityMode,
    /// The controlled label families the consumer preserves verbatim (subset of
    /// [`REQUIRED_LABEL_FAMILIES`]).
    #[serde(default)]
    pub preserved_label_families: Vec<String>,
    /// The frozen work-disposition vocabulary the consumer keeps visible even
    /// when narrowed.
    #[serde(default)]
    pub work_disposition_vocab: Vec<String>,
    /// Whether the consumer keeps the controlled labels.
    pub label_parity: LabelParityState,
    /// The surface a narrower consumer hands off to, if any.
    pub handoff_target: HandoffTarget,
    /// The desktop / companion / browser / support note ref; required when
    /// `handoff_target` is not `None`.
    #[serde(default)]
    pub handoff_note_ref: String,
    /// The reduced-capability banner, present only when the consumer narrows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduced_capability_banner: Option<ReducedCapabilityBanner>,
    /// The copy / export parity of the adopted component.
    pub copy_export: CopyExportParity,
    /// Guardrail: the consumer collapses battery saver, thermal pressure,
    /// user-selected low-power mode, and policy cap into one generic warning.
    /// Must be false.
    pub collapses_pressure_sources_into_generic_warning: bool,
    /// Guardrail: the consumer hides paused work behind toast-only messaging.
    /// Must be false.
    pub hides_paused_work_behind_toast_only: bool,
    /// Guardrail: the consumer presents an override as available when policy
    /// blocks it. Must be false.
    pub presents_override_available_when_policy_blocks: bool,
    /// Guardrail: the consumer clears stale-result context merely because
    /// background work resumed. Must be false.
    pub clears_stale_context_on_resume: bool,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the adoption was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl EfficiencyConsumerRow {
    /// Returns true when the consumer narrows below full authority.
    pub fn is_narrowed(&self) -> bool {
        self.authority_mode.is_narrowed()
    }

    /// The surface's declared class matches the row's declared class.
    pub fn surface_class_consistent(&self) -> bool {
        self.consumer_surface.consumer_class() == self.consumer_class
    }

    /// AC (no fork): the consumer reuses the canonical controls contract for its
    /// family's lane rather than a surface-local one.
    pub fn controls_lane_is_canonical(&self) -> bool {
        self.controls_lane == controls_lane_for(self.component_family)
            && self.canonical_controls_schema_ref == self.controls_lane.canonical_schema_ref()
            && self
                .canonical_controls_artifact_refs
                .iter()
                .any(|r| r == self.controls_lane.canonical_artifact_ref())
    }

    /// AC1 (canonical): the consumer points back to exactly one canonical family
    /// — the declared matrix schema matches the family, a controls release-proof
    /// artifact is referenced, and no surface-local prose is cloned.
    pub fn points_to_canonical_family(&self) -> bool {
        self.canonical_family_schema_ref == canonical_family_schema_ref_for(self.component_family)
            && self.controls_lane_is_canonical()
            && self.references_canonical_not_local_prose
    }

    /// AC1 (parity): the consumer preserves the family's controlled label
    /// families and frozen work-disposition vocabulary rather than renaming or
    /// omitting them.
    pub fn preserves_labels(&self) -> bool {
        self.label_parity.keeps_labels()
            && !self.preserved_label_families.is_empty()
            && self
                .preserved_label_families
                .iter()
                .all(|f| REQUIRED_LABEL_FAMILIES.contains(&f.as_str()))
            && !self.work_disposition_vocab.is_empty()
            && self
                .work_disposition_vocab
                .iter()
                .all(|v| is_canonical_work_disposition(v))
    }

    /// AC (work-content): a constrained-origin consumer preserves the
    /// slowed-versus-paused and what-still-works truth so a throttled work
    /// surface never drops which work slowed and what still runs.
    pub fn preserves_constrained_truth(&self) -> bool {
        if !self.consumer_class.is_constrained_origin() {
            return true;
        }
        let has = |f: &str| self.preserved_label_families.iter().any(|v| v == f);
        has("slowed_versus_paused_work") && has("what_still_works")
    }

    /// AC2: the row carries the opaque constrained-state ref and canonical
    /// controls contract support and automation reconstruct the seen state from.
    pub fn supports_state_reconstruction(&self) -> bool {
        !self.constrained_state_ref.trim().is_empty()
            && self.controls_lane_is_canonical()
            && self.copy_export.is_complete()
    }

    /// The four spec guardrails are all clear (false).
    pub fn guardrails_clear(&self) -> bool {
        self.first_failed_guardrail().is_none()
    }

    /// The first guardrail that is (wrongly) set, if any.
    pub fn first_failed_guardrail(&self) -> Option<&'static str> {
        if self.collapses_pressure_sources_into_generic_warning {
            Some("collapses_pressure_sources_into_generic_warning")
        } else if self.hides_paused_work_behind_toast_only {
            Some("hides_paused_work_behind_toast_only")
        } else if self.presents_override_available_when_policy_blocks {
            Some("presents_override_available_when_policy_blocks")
        } else if self.clears_stale_context_on_resume {
            Some("clears_stale_context_on_resume")
        } else {
            None
        }
    }

    /// AC (disclosure): a narrower consumer discloses the reduction with a
    /// reduced-capability banner whose state matches the authority mode, and
    /// carries a note whenever it punts to another surface.
    pub fn discloses_narrowing(&self) -> bool {
        if self.is_narrowed() {
            match &self.reduced_capability_banner {
                None => return false,
                Some(banner) => {
                    if banner.banner_id.trim().is_empty()
                        || banner.visible_label.trim().is_empty()
                        || label_is_generic(&banner.visible_label)
                        || banner.capability_state != self.authority_mode.capability_state()
                        || banner.capability_state == "full"
                        || banner.missing_capabilities.is_empty()
                    {
                        return false;
                    }
                }
            }
            // A narrowed consumer that keeps every label is disclosed-narrowed,
            // never plain preserved.
            if self.label_parity == LabelParityState::Preserved {
                return false;
            }
        } else if self.reduced_capability_banner.is_some() {
            // A full-interactive consumer must not carry a spurious banner.
            return false;
        }
        if self.handoff_target.requires_note() && self.handoff_note_ref.trim().is_empty() {
            return false;
        }
        true
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EFFICIENCY_CONSUMER_ROW_RECORD_KIND
            && self.schema_version == EFFICIENCY_CONSUMER_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.constrained_state_ref.trim().is_empty()
            && !self.canonical_family_schema_ref.trim().is_empty()
            && !self.canonical_controls_schema_ref.trim().is_empty()
            && !self.canonical_controls_artifact_refs.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "surface={surface} class={class} family={family} lane={lane} \
authority={authority} label_parity={label_parity} handoff={handoff}",
            surface = self.consumer_surface.as_str(),
            class = self.consumer_class.as_str(),
            family = self.component_family.as_str(),
            lane = self.controls_lane.as_str(),
            authority = self.authority_mode.capability_state(),
            label_parity = self.label_parity.as_str(),
            handoff = self.handoff_target.as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1066 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyConsumerSummary {
    pub row_count: usize,
    pub consumer_class_count: usize,
    pub consumer_surface_count: usize,
    pub component_family_count: usize,
    pub controls_lane_count: usize,
    pub work_disposition_count: usize,
    pub all_rows_point_to_canonical_family: bool,
    pub all_rows_preserve_labels: bool,
    pub all_rows_use_canonical_controls_lane: bool,
    pub all_constrained_rows_preserve_truth: bool,
    pub all_rows_reconstructable: bool,
    pub all_narrowed_rows_disclose: bool,
    pub all_rows_have_copy_export: bool,
    pub all_rows_guardrails_clear: bool,
    pub controls_lanes_stable_across_surfaces: bool,
    pub shell_status_activity_consumer_present: bool,
    pub work_content_consumer_present: bool,
    pub docs_browser_companion_consumer_present: bool,
    pub incident_diagnostics_consumer_present: bool,
    pub support_export_help_consumer_present: bool,
    pub docs_help_reference_present: bool,
    pub label_family_coverage_complete: bool,
    pub work_disposition_coverage_complete: bool,
    pub families_reused_across_classes: usize,
}

/// Constructor input for [`EfficiencyConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EfficiencyConsumerPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<EfficiencyConsumerRow>,
}

/// Checked-in M05-1066 consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EfficiencyConsumerPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<EfficiencyConsumerRow>,
    pub summary: EfficiencyConsumerSummary,
}

impl EfficiencyConsumerPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: EfficiencyConsumerPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EFFICIENCY_CONSUMER_SCHEMA_VERSION,
            record_kind: EFFICIENCY_CONSUMER_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: EfficiencyConsumerSummary {
                row_count: 0,
                consumer_class_count: 0,
                consumer_surface_count: 0,
                component_family_count: 0,
                controls_lane_count: 0,
                work_disposition_count: 0,
                all_rows_point_to_canonical_family: false,
                all_rows_preserve_labels: false,
                all_rows_use_canonical_controls_lane: false,
                all_constrained_rows_preserve_truth: false,
                all_rows_reconstructable: false,
                all_narrowed_rows_disclose: false,
                all_rows_have_copy_export: false,
                all_rows_guardrails_clear: false,
                controls_lanes_stable_across_surfaces: false,
                shell_status_activity_consumer_present: false,
                work_content_consumer_present: false,
                docs_browser_companion_consumer_present: false,
                incident_diagnostics_consumer_present: false,
                support_export_help_consumer_present: false,
                docs_help_reference_present: false,
                label_family_coverage_complete: false,
                work_disposition_coverage_complete: false,
                families_reused_across_classes: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5EfficiencyComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// The union of every row's preserved label families.
    pub fn covered_label_families(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.preserved_label_families.iter().cloned())
            .collect()
    }

    /// The union of every row's work-disposition vocabulary.
    pub fn covered_work_dispositions(&self) -> BTreeSet<String> {
        self.rows
            .iter()
            .flat_map(|r| r.work_disposition_vocab.iter().cloned())
            .collect()
    }

    /// The count of component families adopted by two or more distinct consumer
    /// classes — the strongest evidence that a family is a reusable primitive.
    pub fn families_reused_across_classes(&self) -> usize {
        M5EfficiencyComponentFamily::ALL
            .iter()
            .filter(|family| {
                let classes: BTreeSet<ConsumerClass> = self
                    .rows
                    .iter()
                    .filter(|r| r.component_family == **family)
                    .map(|r| r.consumer_class)
                    .collect();
                classes.len() >= 2
            })
            .count()
    }

    /// Whether every family maps to exactly one controls lane across every
    /// surface — no surface forks the lane by consumer.
    pub fn controls_lanes_stable_across_surfaces(&self) -> bool {
        let mut per_family: BTreeMap<
            M5EfficiencyComponentFamily,
            BTreeSet<M5EfficiencyControlsLane>,
        > = BTreeMap::new();
        for row in &self.rows {
            per_family
                .entry(row.component_family)
                .or_default()
                .insert(row.controls_lane);
        }
        per_family.values().all(|lanes| lanes.len() <= 1)
    }

    /// Whether some docs / help surface references the canonical families (AC2).
    pub fn has_docs_help_reference(&self) -> bool {
        self.rows
            .iter()
            .any(|r| r.consumer_surface.is_docs_help() && r.references_canonical_not_local_prose)
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EfficiencyConsumerSummary {
        let mut classes = BTreeSet::new();
        let mut surfaces = BTreeSet::new();
        let mut families = BTreeSet::new();
        let mut lanes = BTreeSet::new();
        for row in &self.rows {
            classes.insert(row.consumer_class);
            surfaces.insert(row.consumer_surface);
            families.insert(row.component_family);
            lanes.insert(row.controls_lane);
        }

        let has_class = |c: ConsumerClass| classes.contains(&c);
        let covered = self.covered_label_families();
        let covered_dispositions = self.covered_work_dispositions();

        EfficiencyConsumerSummary {
            row_count: self.rows.len(),
            consumer_class_count: classes.len(),
            consumer_surface_count: surfaces.len(),
            component_family_count: families.len(),
            controls_lane_count: lanes.len(),
            work_disposition_count: covered_dispositions.len(),
            all_rows_point_to_canonical_family: self
                .rows
                .iter()
                .all(EfficiencyConsumerRow::points_to_canonical_family),
            all_rows_preserve_labels: self
                .rows
                .iter()
                .all(EfficiencyConsumerRow::preserves_labels),
            all_rows_use_canonical_controls_lane: self
                .rows
                .iter()
                .all(EfficiencyConsumerRow::controls_lane_is_canonical),
            all_constrained_rows_preserve_truth: self
                .rows
                .iter()
                .all(EfficiencyConsumerRow::preserves_constrained_truth),
            all_rows_reconstructable: self
                .rows
                .iter()
                .all(EfficiencyConsumerRow::supports_state_reconstruction),
            all_narrowed_rows_disclose: self
                .rows
                .iter()
                .all(EfficiencyConsumerRow::discloses_narrowing),
            all_rows_have_copy_export: self.rows.iter().all(|r| r.copy_export.is_complete()),
            all_rows_guardrails_clear: self
                .rows
                .iter()
                .all(EfficiencyConsumerRow::guardrails_clear),
            controls_lanes_stable_across_surfaces: self.controls_lanes_stable_across_surfaces(),
            shell_status_activity_consumer_present: has_class(ConsumerClass::ShellStatusActivity),
            work_content_consumer_present: has_class(ConsumerClass::WorkContentSurface),
            docs_browser_companion_consumer_present: has_class(ConsumerClass::DocsBrowserCompanion),
            incident_diagnostics_consumer_present: has_class(ConsumerClass::IncidentDiagnostics),
            support_export_help_consumer_present: has_class(ConsumerClass::SupportExportHelp),
            docs_help_reference_present: self.has_docs_help_reference(),
            label_family_coverage_complete: REQUIRED_LABEL_FAMILIES
                .iter()
                .all(|f| covered.contains(*f)),
            work_disposition_coverage_complete: M5EfficiencyWorkDisposition::ALL
                .iter()
                .all(|d| covered_dispositions.contains(d.as_str())),
            families_reused_across_classes: self.families_reused_across_classes(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EfficiencyConsumerViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EFFICIENCY_CONSUMER_SCHEMA_VERSION {
            violations.push(EfficiencyConsumerViolation::SchemaVersion {
                expected: EFFICIENCY_CONSUMER_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EFFICIENCY_CONSUMER_RECORD_KIND {
            violations.push(EfficiencyConsumerViolation::RecordKind {
                expected: EFFICIENCY_CONSUMER_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EfficiencyConsumerViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_classes = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EfficiencyConsumerViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_classes.insert(row.consumer_class);

            if !row.is_complete() {
                violations.push(EfficiencyConsumerViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // The concrete surface must belong to the declared consumer class.
            if !row.surface_class_consistent() {
                violations.push(EfficiencyConsumerViolation::SurfaceClassMismatch {
                    id: row.row_id.clone(),
                });
            }

            // AC1: exactly one canonical family, no cloned surface-local prose.
            if !row.points_to_canonical_family() {
                violations.push(EfficiencyConsumerViolation::NotCanonicalFamily {
                    id: row.row_id.clone(),
                });
            }

            // AC (no fork): canonical controls lane per family.
            if !row.controls_lane_is_canonical() {
                violations.push(EfficiencyConsumerViolation::NonCanonicalControlsLane {
                    id: row.row_id.clone(),
                });
            }

            // AC1: controlled label families / work-disposition vocab preserved.
            if !row.preserves_labels() {
                violations.push(EfficiencyConsumerViolation::LabelParityBroken {
                    id: row.row_id.clone(),
                });
            }

            // AC (work-content): slowed-versus-paused + what-still-works kept.
            if !row.preserves_constrained_truth() {
                violations.push(EfficiencyConsumerViolation::ConstrainedDropsTruth {
                    id: row.row_id.clone(),
                });
            }

            // AC2: constrained state is reconstructable from the opaque ref +
            // canonical controls contract.
            if !row.supports_state_reconstruction() {
                violations.push(EfficiencyConsumerViolation::StateNotReconstructable {
                    id: row.row_id.clone(),
                });
            }

            // Disclosure: narrower consumers disclose reduction with banner + note.
            if !row.discloses_narrowing() {
                violations.push(EfficiencyConsumerViolation::NarrowedWithoutDisclosure {
                    id: row.row_id.clone(),
                });
            }

            // Copy / export parity: text / JSON / Markdown, screenshot prohibited.
            if !row.copy_export.is_complete() {
                violations.push(EfficiencyConsumerViolation::MissingCopyExportParity {
                    id: row.row_id.clone(),
                });
            }

            // Spec guardrails must all stay false.
            if let Some(guardrail) = row.first_failed_guardrail() {
                violations.push(EfficiencyConsumerViolation::GuardrailViolated {
                    id: row.row_id.clone(),
                    guardrail,
                });
            }
        }

        // Cross-surface reuse spans all five claimed consumer classes.
        for class in ConsumerClass::ALL {
            if !seen_classes.contains(&class) {
                violations.push(EfficiencyConsumerViolation::MissingConsumerClass { class });
            }
        }

        // Every frozen family is adopted by at least one consumer.
        let families = self.represented_families();
        for family in M5EfficiencyComponentFamily::ALL {
            if !families.contains(&family) {
                violations.push(EfficiencyConsumerViolation::MissingFamilyCoverage { family });
            }
        }

        // AC1: at least one family is reused across two or more consumer classes
        // so multiple M5 surfaces point back to one canonical family.
        if self.families_reused_across_classes() == 0 {
            violations.push(EfficiencyConsumerViolation::NoFamilyReusedAcrossClasses);
        }

        // AC (no fork): families resolve to one stable controls lane per family.
        if !self.controls_lanes_stable_across_surfaces() {
            violations.push(EfficiencyConsumerViolation::ControlsLaneForkedAcrossSurfaces);
        }

        // AC1: the controlled label families are collectively preserved.
        let covered = self.covered_label_families();
        for family in REQUIRED_LABEL_FAMILIES {
            if !covered.contains(family) {
                violations.push(EfficiencyConsumerViolation::MissingLabelFamily {
                    family: family.to_owned(),
                });
            }
        }

        // AC1: the frozen work-disposition vocabulary is collectively preserved.
        let covered_dispositions = self.covered_work_dispositions();
        for disposition in M5EfficiencyWorkDisposition::ALL {
            if !covered_dispositions.contains(disposition.as_str()) {
                violations.push(EfficiencyConsumerViolation::MissingWorkDisposition {
                    disposition: disposition.as_str().to_owned(),
                });
            }
        }

        // AC2: a docs / help consumer references the canonical components rather
        // than cloning local efficiency vocabulary.
        if !self.has_docs_help_reference() {
            violations.push(EfficiencyConsumerViolation::MissingDocsHelpReference);
        }

        if self.summary != self.computed_summary() {
            violations.push(EfficiencyConsumerViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("consumer packet serializes"),
        ) {
            violations.push(EfficiencyConsumerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("consumer packet serializes")
    }

    /// Deterministic CSV of the adoption rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,consumer_class,consumer_surface,component_family,controls_lane,authority,label_parity,handoff\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{class},{surface},{family},{lane},{authority},{label_parity},{handoff}\n",
                id = row.row_id,
                class = row.consumer_class.as_str(),
                surface = row.consumer_surface.as_str(),
                family = row.component_family.as_str(),
                lane = row.controls_lane.as_str(),
                authority = row.authority_mode.capability_state(),
                label_parity = row.label_parity.as_str(),
                handoff = row.handoff_target.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Adaptive-Efficiency Component Consumers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Rows: {} across {} consumer classes and {} / {} frozen families\n",
            self.summary.row_count,
            self.summary.consumer_class_count,
            self.represented_families().len(),
            M5EfficiencyComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Controls lanes adopted: {} / {}\n",
            self.summary.controls_lane_count,
            M5EfficiencyControlsLane::ALL.len(),
        ));
        out.push_str(&format!(
            "- Work dispositions preserved: {} / {}\n",
            self.summary.work_disposition_count,
            M5EfficiencyWorkDisposition::ALL.len(),
        ));
        out.push_str(&format!(
            "- Families reused across classes: {}\n",
            self.summary.families_reused_across_classes,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!("- **{}** — {}\n", row.row_id, row.chip_tokens()));
        }
        out
    }
}

/// Reads and validates the checked-in consumer export.
pub fn current_m5_efficiency_component_consumers_export(
) -> Result<EfficiencyConsumerPacket, EfficiencyConsumerArtifactError> {
    let packet: EfficiencyConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-efficiency-component-consumer-proof/support_export.json"
    )))
    .map_err(EfficiencyConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EfficiencyConsumerArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in consumer export.
#[derive(Debug)]
pub enum EfficiencyConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EfficiencyConsumerViolation>),
}

impl fmt::Display for EfficiencyConsumerArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "consumer export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "consumer export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for EfficiencyConsumerArtifactError {}

/// Validation failure for M05-1066 consumer packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EfficiencyConsumerViolation {
    SchemaVersion { expected: u32, actual: u32 },
    RecordKind { expected: String, actual: String },
    MissingIdentity,
    DuplicateId { id: String },
    IncompleteRow { id: String },
    SurfaceClassMismatch { id: String },
    NotCanonicalFamily { id: String },
    NonCanonicalControlsLane { id: String },
    LabelParityBroken { id: String },
    ConstrainedDropsTruth { id: String },
    StateNotReconstructable { id: String },
    NarrowedWithoutDisclosure { id: String },
    MissingCopyExportParity { id: String },
    GuardrailViolated { id: String, guardrail: &'static str },
    MissingConsumerClass { class: ConsumerClass },
    MissingFamilyCoverage { family: M5EfficiencyComponentFamily },
    NoFamilyReusedAcrossClasses,
    ControlsLaneForkedAcrossSurfaces,
    MissingLabelFamily { family: String },
    MissingWorkDisposition { disposition: String },
    MissingDocsHelpReference,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for EfficiencyConsumerViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete consumer row: {id}"),
            Self::SurfaceClassMismatch { id } => {
                write!(
                    f,
                    "row {id} declares a surface that does not belong to its consumer class"
                )
            }
            Self::NotCanonicalFamily { id } => {
                write!(
                    f,
                    "row {id} does not point back to exactly one canonical component family"
                )
            }
            Self::NonCanonicalControlsLane { id } => {
                write!(
                    f,
                    "row {id} forks the controls lane instead of reusing the canonical contract"
                )
            }
            Self::LabelParityBroken { id } => {
                write!(
                    f,
                    "row {id} renames or drops a canonical source-of-change, active-state, \
slowed-versus-paused, what-still-works, override, policy-owner, backlog, \
stale-continuity, or next-safe-action label"
                )
            }
            Self::ConstrainedDropsTruth { id } => {
                write!(
                    f,
                    "work-content row {id} drops slowed-versus-paused or what-still-works truth"
                )
            }
            Self::StateNotReconstructable { id } => {
                write!(
                    f,
                    "row {id} cannot be reconstructed from its constrained-state ref and controls contract"
                )
            }
            Self::NarrowedWithoutDisclosure { id } => {
                write!(
                    f,
                    "row {id} narrows authority without a reduced-capability banner or handoff note"
                )
            }
            Self::MissingCopyExportParity { id } => {
                write!(
                    f,
                    "row {id} is missing text / JSON / Markdown copy-export parity"
                )
            }
            Self::GuardrailViolated { id, guardrail } => {
                write!(f, "row {id} violates guardrail {guardrail}")
            }
            Self::MissingConsumerClass { class } => {
                write!(f, "consumer class {class:?} is not adopted in the packet")
            }
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "component family {family:?} is not adopted in the packet"
                )
            }
            Self::NoFamilyReusedAcrossClasses => write!(
                f,
                "no component family is adopted across two or more consumer classes"
            ),
            Self::ControlsLaneForkedAcrossSurfaces => write!(
                f,
                "a component family resolves to more than one controls lane across surfaces"
            ),
            Self::MissingLabelFamily { family } => {
                write!(
                    f,
                    "controlled label family {family} is not preserved anywhere"
                )
            }
            Self::MissingWorkDisposition { disposition } => {
                write!(
                    f,
                    "work-disposition token {disposition} is not preserved anywhere"
                )
            }
            Self::MissingDocsHelpReference => write!(
                f,
                "no docs / help consumer references the canonical component families"
            ),
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for EfficiencyConsumerViolation {}

/// Whether a banner label is a generic non-answer rather than a precise label.
/// Adds the adaptive-efficiency generic phrasings the spec forbids collapsing
/// into (low power, power saver, battery saver, throttled, slowed down) to the
/// shared generic-label blocklist. These are matched as *whole* labels rather
/// than substrings so a descriptive banner may still name "OS battery saver" or
/// "user low-power mode" as a source of change without being flagged; only a
/// banner whose entire label collapses to the generic phrase is rejected.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    if lower.contains("get started") {
        return true;
    }
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "read only"
            | "read-only"
            | "offline"
            | "throttled"
            | "blocked"
            | "low power"
            | "low-power"
            | "power saver"
            | "power-saving"
            | "battery saver"
            | "saving power"
            | "slowed down"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in consumer packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay
/// byte-aligned.
pub fn seeded_m5_efficiency_component_consumers_packet() -> EfficiencyConsumerPacket {
    EfficiencyConsumerPacket::new(EfficiencyConsumerPacketInput {
        packet_id: "m5-efficiency-component-consumers:stable:0001".to_owned(),
        as_of: "2026-07-10T00:00:00Z".to_owned(),
        matrix_ref: EFFICIENCY_CONSUMER_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:efficiency-consumer:{id}")]
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn labels(families: &[&str]) -> Vec<String> {
    families.iter().map(|f| (*f).to_owned()).collect()
}

fn banner(
    id: &str,
    label: &str,
    authority: AuthorityMode,
    missing: &[&str],
) -> ReducedCapabilityBanner {
    ReducedCapabilityBanner {
        banner_id: id.to_owned(),
        visible_label: label.to_owned(),
        capability_state: authority.capability_state().to_owned(),
        missing_capabilities: missing.iter().map(|m| (*m).to_owned()).collect(),
    }
}

#[allow(clippy::too_many_arguments)]
fn row(
    row_id: &str,
    consumer_surface: EfficiencyConsumerSurface,
    component_family: M5EfficiencyComponentFamily,
    authority_mode: AuthorityMode,
    label_families: &[&str],
    export_fields: &[&str],
    handoff_target: HandoffTarget,
    handoff_note_ref: &str,
    reduced_capability_banner: Option<ReducedCapabilityBanner>,
) -> EfficiencyConsumerRow {
    let label_parity = if authority_mode.is_narrowed() {
        LabelParityState::DisclosedNarrowed
    } else {
        LabelParityState::Preserved
    };
    let controls_lane = controls_lane_for(component_family);
    EfficiencyConsumerRow {
        record_kind: EFFICIENCY_CONSUMER_ROW_RECORD_KIND.to_owned(),
        schema_version: EFFICIENCY_CONSUMER_SCHEMA_VERSION,
        row_id: row_id.to_owned(),
        consumer_class: consumer_surface.consumer_class(),
        consumer_surface,
        component_family,
        controls_lane,
        canonical_family_schema_ref: canonical_family_schema_ref_for(component_family).to_owned(),
        canonical_controls_schema_ref: controls_lane.canonical_schema_ref().to_owned(),
        canonical_controls_artifact_refs: vec![controls_lane.canonical_artifact_ref().to_owned()],
        references_canonical_not_local_prose: true,
        constrained_state_ref: format!("efficiency-state:{row_id}"),
        authority_mode,
        preserved_label_families: labels(label_families),
        work_disposition_vocab: canonical_work_disposition_vocab(),
        label_parity,
        handoff_target,
        handoff_note_ref: handoff_note_ref.to_owned(),
        reduced_capability_banner,
        copy_export: copy_export(export_fields),
        collapses_pressure_sources_into_generic_warning: false,
        hides_paused_work_behind_toast_only: false,
        presents_override_available_when_policy_blocks: false,
        clears_stale_context_on_resume: false,
        source_refs: vec![
            EFFICIENCY_CONSUMER_MATRIX_REF.to_owned(),
            EFFICIENCY_CONSUMER_SHARED_SCHEMA_REF.to_owned(),
            controls_lane.canonical_doc_ref().to_owned(),
        ],
        observed_at: "2026-07-10T00:00:00Z".to_owned(),
        evidence_refs: ev(row_id),
    }
}

fn seeded_rows() -> Vec<EfficiencyConsumerRow> {
    use AuthorityMode::*;
    use EfficiencyConsumerSurface::*;
    use HandoffTarget as H;
    use M5EfficiencyComponentFamily::*;

    vec![
        // --- Shell status / activity center / background work --------------
        row(
            "consumer:shell-status:power-state",
            ShellStatusBar,
            PowerStateIndicator,
            FullInteractive,
            &["source_of_change", "active_efficiency_state"],
            &["source_of_change", "active_efficiency_state", "controls_lane"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:activity-center:background-row",
            ActivityCenter,
            BackgroundWorkRow,
            FullInteractive,
            &["slowed_versus_paused_work", "what_still_works"],
            &[
                "slowed_versus_paused_work",
                "what_still_works",
                "resume_condition",
                "controls_lane",
            ],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:activity-center:override-sheet",
            ActivityCenter,
            PerWorkspaceOverrideSheet,
            FullInteractive,
            &["override_availability", "policy_owner", "next_safe_action"],
            &[
                "override_availability",
                "policy_owner",
                "expected_effect",
                "controls_lane",
            ],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:activity-center:resume-card",
            ActivityCenter,
            ResumeSummaryCard,
            FullInteractive,
            &["resumed_work_backlog", "next_safe_action"],
            &["resumed_work_backlog", "next_safe_action", "controls_lane"],
            H::None,
            "",
            None,
        ),
        row(
            "consumer:background-tray:background-banner",
            BackgroundWorkTray,
            BackgroundWorkBanner,
            FullInteractive,
            &["slowed_versus_paused_work", "next_safe_action"],
            &["slowed_versus_paused_work", "next_safe_action", "controls_lane"],
            H::None,
            "",
            None,
        ),
        // --- Notebook / preview / pipeline / graph work content ------------
        row(
            "consumer:notebook:throttled-row",
            NotebookCanvas,
            ThrottledSubsystemRow,
            ReadOnly,
            &[
                "slowed_versus_paused_work",
                "what_still_works",
                "active_efficiency_state",
            ],
            &[
                "slowed_versus_paused_work",
                "what_still_works",
                "affected_subsystem",
                "controls_lane",
            ],
            H::DesktopShell,
            "handoff:notebook:throttled-row-desktop-shell",
            Some(banner(
                "banner:notebook:throttled-row",
                "Read-only notebook status: shows which subsystem slowed, what still runs, and the active efficiency state; changing it stays in the desktop shell",
                ReadOnly,
                &["change_efficiency_state", "resume_subsystem"],
            )),
        ),
        row(
            "consumer:preview:background-row",
            PreviewPane,
            BackgroundWorkRow,
            ReadOnly,
            &["slowed_versus_paused_work", "what_still_works"],
            &[
                "slowed_versus_paused_work",
                "what_still_works",
                "resume_condition",
                "controls_lane",
            ],
            H::DesktopShell,
            "handoff:preview:background-row-desktop-shell",
            Some(banner(
                "banner:preview:background-row",
                "Read-only preview status: shows the paused preview refresh, what still renders, and the resume condition; resuming stays in the desktop shell",
                ReadOnly,
                &["resume_preview_refresh", "override_pause"],
            )),
        ),
        row(
            "consumer:pipeline:throttled-row",
            PipelineRunner,
            ThrottledSubsystemRow,
            InspectOnly,
            &["slowed_versus_paused_work", "what_still_works"],
            &[
                "slowed_versus_paused_work",
                "what_still_works",
                "affected_subsystem",
                "controls_lane",
            ],
            H::DesktopShell,
            "handoff:pipeline:throttled-row-desktop-shell",
            Some(banner(
                "banner:pipeline:throttled-row",
                "Inspect-only pipeline status: read which pipeline stage slowed and what still runs without altering the run; acting stays in the desktop shell",
                InspectOnly,
                &["alter_pipeline_run", "resume_stage"],
            )),
        ),
        row(
            "consumer:graph:stale-note",
            GraphExplorer,
            StaleResultContinuityNote,
            ReadOnly,
            &[
                "stale_result_continuity",
                "slowed_versus_paused_work",
                "what_still_works",
                "next_safe_action",
            ],
            &[
                "stale_result_continuity",
                "what_still_works",
                "next_safe_action",
                "controls_lane",
            ],
            H::DesktopShell,
            "handoff:graph:stale-note-desktop-shell",
            Some(banner(
                "banner:graph:stale-note",
                "Read-only graph status: keeps the stale-result continuity note visible and states which enrichment slowed and the next safe action; resuming stays in the desktop shell",
                ReadOnly,
                &["resume_enrichment", "clear_stale_result"],
            )),
        ),
        // --- Docs-browser / companion-adjacent handoff ---------------------
        row(
            "consumer:docs-browser:power-state",
            DocsBrowserHandoff,
            PowerStateIndicator,
            ReadOnly,
            &["source_of_change", "active_efficiency_state"],
            &["source_of_change", "active_efficiency_state", "controls_lane"],
            H::BrowserReadonly,
            "handoff:docs-browser:power-state-desktop-shell",
            Some(banner(
                "banner:docs-browser:power-state",
                "Read-only browser view: reads the source of change and the active efficiency state; the desktop shell acts on it",
                ReadOnly,
                &["change_efficiency_state"],
            )),
        ),
        row(
            "consumer:docs-browser:override-note",
            DocsBrowserHandoff,
            OverridePolicyNoteRow,
            ReadOnly,
            &["override_availability", "policy_owner"],
            &["override_availability", "policy_owner", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:docs-browser:override-note",
                "Read-only docs reference: explains whether the override is available and which policy owner controls it, without offering the override here",
                ReadOnly,
                &["apply_override"],
            )),
        ),
        row(
            "consumer:companion:background-banner",
            CompanionAdjacent,
            BackgroundWorkBanner,
            InspectOnly,
            &["slowed_versus_paused_work", "next_safe_action"],
            &["slowed_versus_paused_work", "next_safe_action", "controls_lane"],
            H::DesktopShell,
            "handoff:companion:background-banner-desktop-shell",
            Some(banner(
                "banner:companion:background-banner",
                "Inspect-only companion banner: shows the paused background work and the next safe action; the desktop shell resumes it",
                InspectOnly,
                &["resume_background_work", "override_pause"],
            )),
        ),
        // --- Incident / diagnostics ----------------------------------------
        row(
            "consumer:incident:power-state",
            IncidentConsole,
            PowerStateIndicator,
            ReadOnly,
            &["source_of_change", "active_efficiency_state"],
            &["source_of_change", "active_efficiency_state", "controls_lane"],
            H::DesktopShell,
            "handoff:incident:power-state-desktop-shell",
            Some(banner(
                "banner:incident:power-state",
                "Read-only incident view: reads the pressure source and active efficiency state during triage; acting stays in the desktop shell",
                ReadOnly,
                &["change_efficiency_state"],
            )),
        ),
        row(
            "consumer:diagnostics:override-note",
            DiagnosticsPanel,
            OverridePolicyNoteRow,
            ReadOnly,
            &["override_availability", "policy_owner"],
            &["override_availability", "policy_owner", "controls_lane"],
            H::DesktopShell,
            "handoff:diagnostics:override-note-desktop-shell",
            Some(banner(
                "banner:diagnostics:override-note",
                "Read-only diagnostics view: reads whether the override is available and the policy owner that blocks or allows it; acting stays in the desktop shell",
                ReadOnly,
                &["apply_override"],
            )),
        ),
        row(
            "consumer:diagnostics:resume-card",
            DiagnosticsPanel,
            ResumeSummaryCard,
            ReadOnly,
            &["resumed_work_backlog", "next_safe_action"],
            &["resumed_work_backlog", "next_safe_action", "controls_lane"],
            H::DesktopShell,
            "handoff:diagnostics:resume-card-desktop-shell",
            Some(banner(
                "banner:diagnostics:resume-card",
                "Read-only diagnostics view: reads the resumed-work backlog and the next safe action after pressure ended; resuming stays in the desktop shell",
                ReadOnly,
                &["resume_backlog_item"],
            )),
        ),
        // --- Support / export + Help/About (AC2) ---------------------------
        row(
            "consumer:support-export:override-sheet",
            SupportExportReplay,
            PerWorkspaceOverrideSheet,
            ExportOnly,
            &["override_availability", "policy_owner", "next_safe_action"],
            &[
                "override_availability",
                "policy_owner",
                "expected_effect",
                "constrained_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:override-sheet-support-packet",
            Some(banner(
                "banner:support-export:override-sheet",
                "Export-only support replay: reconstruct whether the override was available, its policy owner, and the expected effect the user saw from the support packet",
                ExportOnly,
                &["apply_override", "reset_override"],
            )),
        ),
        row(
            "consumer:support-export:stale-note",
            SupportExportReplay,
            StaleResultContinuityNote,
            ExportOnly,
            &["stale_result_continuity", "next_safe_action"],
            &[
                "stale_result_continuity",
                "next_safe_action",
                "constrained_state_ref",
                "controls_lane",
            ],
            H::SupportPacket,
            "handoff:support-export:stale-note-support-packet",
            Some(banner(
                "banner:support-export:stale-note",
                "Export-only support replay: reconstruct which results stayed stale, that they were still shown, and the next safe action from the support packet",
                ExportOnly,
                &["refresh_result", "clear_stale_result"],
            )),
        ),
        row(
            "consumer:help-about:power-state",
            HelpAboutReference,
            PowerStateIndicator,
            ReadOnly,
            &["source_of_change", "active_efficiency_state"],
            &["source_of_change", "active_efficiency_state", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:help-about:power-state",
                "Read-only help reference: explains each source of change — AC power, battery, OS battery saver, user low-power mode, thermal pressure, and policy cap — and the matching efficiency state",
                ReadOnly,
                &["change_efficiency_state"],
            )),
        ),
        row(
            "consumer:help-about:resume-card",
            HelpAboutReference,
            ResumeSummaryCard,
            ReadOnly,
            &["resumed_work_backlog", "next_safe_action"],
            &["resumed_work_backlog", "next_safe_action", "controls_lane"],
            H::None,
            "",
            Some(banner(
                "banner:help-about:resume-card",
                "Read-only help reference: explains what the resume summary lists when pressure ends — resumed work, remaining backlog, and the next safe action",
                ReadOnly,
                &["resume_backlog_item"],
            )),
        ),
    ]
}
