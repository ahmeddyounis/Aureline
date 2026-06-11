//! Runbook execution rows, deviation notes, export bundles, and browser or
//! vendor-console handoff, projected as a downgrade-aware truth packet.
//!
//! This module owns the export-safe truth packet for the *execution* phase of an
//! incident runbook, building on the static runbook packets defined in
//! [`crate::add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets`].
//! It projects four sections: the **execution rows** that record each runbook step
//! as it runs, the **deviation notes** that record—as first-class facts—every
//! point where execution departed from the runbook, the **export bundles** that
//! package an incident's execution transcript and evidence for sharing, and the
//! **browser or vendor-console handoff** that resumes an item in an external
//! surface. It binds those four sections to the frozen M5 companion-matrix
//! [`M5CompanionMatrixLane::IncidentWorkspace`] lane that qualifies them, and gives
//! every item an exact [`CompanionDesktopHandoff`] (reused from
//! [`crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`])
//! so opening an item always resumes the precise host context locally.
//!
//! Three invariants make this surface safe to ship. First, **attributable and
//! read-mostly**: every section is read-only, an execution row never applies an
//! automated step without explicit host approval, and a row or note that loses
//! attribution narrows to [`IncidentAttributionState::Unattributed`] rather than
//! claiming a provenance it can no longer prove. A deviation from the runbook is
//! recorded as a first-class note, never silently dropped. Second, **stale-state
//! honesty**: every item carries a [`CompanionFreshnessState`], stale or unknown
//! freshness is always labeled, and a degraded item is never shown as live. Third,
//! **local-first external handoff**: a browser or vendor-console handoff is always
//! explicit that it requires provider continuity, and it never strands the user —
//! every external-handoff item also carries an exact desktop handoff as the
//! local-first path, so the local core never depends on the external surface.
//!
//! The packet reuses the matrix vocabulary from
//! [`crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes`]
//! ([`M5CompanionQualificationClass`], [`M5CompanionRolloutStage`],
//! [`M5CompanionDowngradeTrigger`], [`M5CompanionRollbackPosture`],
//! [`M5CompanionLocalityDisclosure`], [`M5CompanionConsumerSurface`]), the incident
//! severity, attribution, freshness, scope, and handoff vocabulary from
//! [`crate::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty`]
//! and the companion triage surface, and the runbook automation and step-state
//! vocabulary ([`RunbookAutomationClass`], [`RunbookStepState`]) from the incident
//! workspace surface, instead of inventing parallel terms. Each section row records
//! the matrix lane it inherits qualification from.
//!
//! [`RunbookExecutionSurfacePacket::apply_runbook_execution_degradation`] narrows
//! sections and downgrades freshness, attribution, and handoff resolution from a
//! per-observation signal — when the relay is unavailable, proof is stale, the host
//! session is inactive, trust narrowed, incident attribution was lost, an export
//! bundle is incomplete, the external browser/vendor console is unreachable, or an
//! upstream matrix lane narrowed — so CI or release tooling degrades the surface
//! honestly rather than show fresh state, a proven attribution, or a handoff that no
//! longer resolves. Degraded state is labeled, never hidden.
//!
//! [`canonical_runbook_execution_surface`] builds the surface and
//! [`current_stable_runbook_execution_surface_export`] reads and validates the
//! checked-in support export, so the incident workspace, the desktop companion
//! panel, the browser companion, diagnostics, support exports, and Help/About ingest
//! the packet rather than cloning status text. Credential bodies, raw provider
//! payloads, and raw execution, note, bundle, or vendor-console bodies stay outside
//! this boundary.
//!
//! The boundary schema is
//! [`schemas/companion/implement-runbook-execution-rows-deviation-notes-export-bundles-and-browser-or-vendor-console-handoff-truth.schema.json`](../../../../schemas/companion/implement-runbook-execution-rows-deviation-notes-export-bundles-and-browser-or-vendor-console-handoff-truth.schema.json).
//! The contract doc is
//! [`docs/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth.md`](../../../../docs/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth/`](../../../../fixtures/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets::{
    RunbookAutomationClass, RunbookStepState,
};
use crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff::{
    CompanionDesktopHandoff, CompanionHandoffResolution, CompanionHandoffTarget,
};
use crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes::{
    M5CompanionConsumerSurface, M5CompanionDowngradeTrigger, M5CompanionLocalityDisclosure,
    M5CompanionMatrixLane, M5CompanionQualificationClass, M5CompanionRollbackPosture,
    M5CompanionRolloutStage, M5_COMPANION_BOUNDARY_MANIFEST_REF, M5_COMPANION_MATRIX_SCHEMA_REF,
    M5_COMPANION_QUALIFICATION_REF, M5_COMPANION_SURFACE_CONTRACT_REF,
    M5_INCIDENT_WORKSPACE_CONTRACT_REF,
};
use crate::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty::{
    CompanionFreshnessState, CompanionReadWriteScope, IncidentAttributionState,
};

/// Stable record-kind tag carried by [`RunbookExecutionSurfacePacket`].
pub const RUNBOOK_EXECUTION_SURFACE_RECORD_KIND: &str =
    "implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth";

/// Schema version for runbook execution surface records.
pub const RUNBOOK_EXECUTION_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RUNBOOK_EXECUTION_SURFACE_SCHEMA_REF: &str =
    "schemas/companion/implement-runbook-execution-rows-deviation-notes-export-bundles-and-browser-or-vendor-console-handoff-truth.schema.json";

/// Repo-relative path of the runbook execution surface contract doc.
pub const RUNBOOK_EXECUTION_SURFACE_DOC_REF: &str =
    "docs/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth.md";

/// Repo-relative path of the protected fixture directory.
pub const RUNBOOK_EXECUTION_SURFACE_FIXTURE_DIR: &str =
    "fixtures/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth";

/// Repo-relative path of the checked support-export artifact.
pub const RUNBOOK_EXECUTION_SURFACE_ARTIFACT_REF: &str =
    "artifacts/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RUNBOOK_EXECUTION_SURFACE_SUMMARY_REF: &str =
    "artifacts/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth.md";

/// One of the four runbook execution sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookExecutionSection {
    /// The per-step runbook execution rows.
    ExecutionRow,
    /// The first-class deviation notes.
    DeviationNote,
    /// The export bundles that package the incident for sharing.
    ExportBundle,
    /// The browser or vendor-console handoff to an external surface.
    ExternalHandoff,
}

impl RunbookExecutionSection {
    /// Every section, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ExecutionRow,
        Self::DeviationNote,
        Self::ExportBundle,
        Self::ExternalHandoff,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutionRow => "execution_row",
            Self::DeviationNote => "deviation_note",
            Self::ExportBundle => "export_bundle",
            Self::ExternalHandoff => "external_handoff",
        }
    }

    /// Frozen M5 companion-matrix lane this section inherits qualification from.
    ///
    /// Every runbook execution section inherits from the single
    /// [`M5CompanionMatrixLane::IncidentWorkspace`] lane.
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        M5CompanionMatrixLane::IncidentWorkspace
    }

    /// Read/write scope this section is bounded to.
    ///
    /// Every section is read-only: the surface observes and packages but never
    /// mutates host state directly. An execution row's automated action is relayed
    /// for explicit host approval rather than applied from the surface.
    pub const fn bounded_scope(self) -> CompanionReadWriteScope {
        CompanionReadWriteScope::ReadOnly
    }
}

/// Outcome of a single runbook execution row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookExecutionOutcome {
    /// The step is still running or pending; no terminal outcome yet.
    InFlight,
    /// The step completed successfully.
    Succeeded,
    /// The step failed.
    Failed,
    /// The step was deviated from (skipped, reordered, or overridden).
    Deviated,
    /// The step was rolled back.
    RolledBack,
}

impl RunbookExecutionOutcome {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InFlight => "in_flight",
            Self::Succeeded => "succeeded",
            Self::Failed => "failed",
            Self::Deviated => "deviated",
            Self::RolledBack => "rolled_back",
        }
    }

    /// True when the outcome records a departure from the planned runbook.
    pub const fn is_deviation(self) -> bool {
        matches!(self, Self::Deviated | Self::RolledBack)
    }
}

/// Kind of deviation recorded by a deviation note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviationNoteKind {
    /// A planned step was skipped.
    StepSkipped,
    /// Steps were executed out of the planned order.
    StepReordered,
    /// An operator manually overrode the runbook guidance.
    ManualOverride,
    /// A step's parameters were changed from the runbook default.
    ParameterChanged,
    /// The runbook was aborted before completion.
    RunbookAborted,
}

impl DeviationNoteKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StepSkipped => "step_skipped",
            Self::StepReordered => "step_reordered",
            Self::ManualOverride => "manual_override",
            Self::ParameterChanged => "parameter_changed",
            Self::RunbookAborted => "runbook_aborted",
        }
    }
}

/// Significance of a deviation note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviationSignificance {
    /// A minor, low-impact departure.
    Minor,
    /// A notable departure worth reviewing.
    Notable,
    /// A major departure that materially changed the mitigation.
    Major,
}

impl DeviationSignificance {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Minor => "minor",
            Self::Notable => "notable",
            Self::Major => "major",
        }
    }
}

/// Kind of export bundle packaged for sharing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportBundleKind {
    /// A bundle of the incident's evidence spans.
    IncidentEvidenceBundle,
    /// A transcript of the runbook execution rows.
    RunbookExecutionTranscript,
    /// A log of the recorded deviation notes.
    DeviationLog,
    /// A full incident archive (evidence, transcript, and deviations).
    FullIncidentArchive,
}

impl ExportBundleKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentEvidenceBundle => "incident_evidence_bundle",
            Self::RunbookExecutionTranscript => "runbook_execution_transcript",
            Self::DeviationLog => "deviation_log",
            Self::FullIncidentArchive => "full_incident_archive",
        }
    }
}

/// Readiness state of an export bundle.
///
/// A bundle that is not [`Self::Ready`] is recorded as a first-class incomplete
/// fact so the surface never claims an export is complete when it is not.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportBundleState {
    /// The bundle is fully built and ready to share.
    Ready,
    /// The bundle is still building.
    Building,
    /// The bundle is partial; some inputs were missing.
    Partial,
    /// The bundle failed to build.
    Failed,
}

impl ExportBundleState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Building => "building",
            Self::Partial => "partial",
            Self::Failed => "failed",
        }
    }

    /// True when the bundle is not fully ready and must carry an incomplete label.
    pub const fn is_incomplete(self) -> bool {
        !matches!(self, Self::Ready)
    }

    /// Narrows a ready bundle to partial; already-incomplete states are kept.
    pub const fn narrowed(self) -> Self {
        match self {
            Self::Ready => Self::Partial,
            Self::Building | Self::Partial | Self::Failed => self,
        }
    }
}

/// External surface a browser or vendor-console handoff resumes in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExternalHandoffTarget {
    /// A browser companion tab.
    BrowserCompanionTab,
    /// A vendor's incident-management console.
    VendorIncidentConsole,
    /// A vendor's public status page.
    VendorStatusPage,
}

impl ExternalHandoffTarget {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowserCompanionTab => "browser_companion_tab",
            Self::VendorIncidentConsole => "vendor_incident_console",
            Self::VendorStatusPage => "vendor_status_page",
        }
    }
}

/// A browser or vendor-console handoff to an external surface.
///
/// An external handoff always requires provider continuity, and it is never the
/// only path: the owning item also carries an exact desktop handoff as the
/// local-first fallback so the local core never depends on the external surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHandoff {
    /// External surface the handoff resumes in.
    pub target: ExternalHandoffTarget,
    /// Human-readable provider label. Carries no payload body.
    pub provider_label: String,
    /// Opaque, resolvable deep-link ref into the external surface. Carries no body.
    pub deep_link_ref: String,
    /// How precisely the external handoff resolves.
    pub resolution: CompanionHandoffResolution,
    /// Always true: a browser/vendor-console handoff requires provider continuity.
    pub requires_provider_continuity: bool,
    /// Always true: an exact local desktop handoff remains as the local-first path.
    pub local_fallback_available: bool,
}

/// A runbook execution row that records a step as it runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionRowItem {
    /// Stable item id.
    pub item_id: String,
    /// Ref to the runbook packet this row executes. Carries no payload body.
    pub runbook_ref: String,
    /// Stable incident id this execution belongs to.
    pub incident_id: String,
    /// Ordering position of the step within the runbook.
    pub step_index: u32,
    /// State of the step.
    pub step_state: RunbookStepState,
    /// Outcome of the step.
    pub outcome: RunbookExecutionOutcome,
    /// Automation class of the step.
    pub automation_class: RunbookAutomationClass,
    /// Always true: any automated action requires explicit host approval.
    pub requires_host_approval: bool,
    /// Attribution to evidence and build identity.
    pub attribution: IncidentAttributionState,
    /// Freshness of the row state.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the execution record. Carries no payload body.
    pub execution_ref: String,
    /// Exact desktop handoff into the execution row.
    pub handoff: CompanionDesktopHandoff,
}

/// A first-class deviation note recording a departure from the runbook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviationNoteItem {
    /// Stable item id.
    pub item_id: String,
    /// Ref to the execution row this note annotates. Carries no payload body.
    pub execution_row_ref: String,
    /// Kind of deviation.
    pub kind: DeviationNoteKind,
    /// Significance of the deviation.
    pub significance: DeviationSignificance,
    /// Attribution to the operator and evidence.
    pub attribution: IncidentAttributionState,
    /// Freshness of the note.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the deviation note record. Carries no payload body.
    pub note_ref: String,
    /// Exact desktop handoff to the deviation note.
    pub handoff: CompanionDesktopHandoff,
}

/// An export bundle that packages the incident for sharing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportBundleItem {
    /// Stable item id.
    pub item_id: String,
    /// Kind of export bundle.
    pub bundle_kind: ExportBundleKind,
    /// Readiness state of the bundle.
    pub bundle_state: ExportBundleState,
    /// Always true: the bundle carries no credential bodies or raw provider payloads.
    pub redaction_checked: bool,
    /// True when a not-ready bundle carries a visible incomplete label.
    pub incomplete_label_shown: bool,
    /// Freshness of the bundle.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the export bundle record. Carries no payload body.
    pub bundle_ref: String,
    /// Exact desktop handoff to the export bundle.
    pub handoff: CompanionDesktopHandoff,
}

/// A browser or vendor-console handoff item.
///
/// Carries both the external handoff and an exact desktop handoff: the desktop
/// handoff is the local-first path, the external handoff is the provider-continuity
/// path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExternalHandoffItem {
    /// Stable item id.
    pub item_id: String,
    /// The external (browser/vendor-console) handoff.
    pub external: ExternalHandoff,
    /// Freshness of the item.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the external target. Carries no payload body.
    pub target_ref: String,
    /// Exact desktop handoff: the local-first fallback for this item.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-section qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionSectionQualification {
    /// Section the row applies to.
    pub section: RunbookExecutionSection,
    /// Qualification class earned by this section.
    pub qualification: M5CompanionQualificationClass,
    /// Staged rollout stage.
    pub rollout_stage: M5CompanionRolloutStage,
    /// Read/write scope this section is bounded to.
    pub read_write_scope: CompanionReadWriteScope,
    /// Token of the frozen matrix lane this section inherits qualification from.
    pub matrix_lane_ref: String,
    /// Downgrade triggers that apply to this section.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5CompanionRollbackPosture,
}

/// Read/write scope and authority contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionScopeContract {
    /// An execution row never applies an automated step without host approval.
    pub execution_row_read_only_unless_host_approved: bool,
    /// The deviation notes are read-only.
    pub deviation_note_read_only: bool,
    /// The export bundles are read-only.
    pub export_bundle_read_only: bool,
    /// The external handoff is read-only.
    pub external_handoff_read_only: bool,
    /// Every external handoff requires provider continuity.
    pub external_handoff_requires_provider_continuity: bool,
    /// Every external-handoff item keeps an exact local desktop fallback.
    pub local_fallback_always_available: bool,
    /// The surface never holds an unbounded write authority.
    pub no_unbounded_workspace_write: bool,
    /// The desktop host stays authoritative.
    pub host_authoritative: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Attribution contract: execution stays attributable and deviations are recorded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionAttributionContract {
    /// Execution rows are attributed or narrowed, never falsely claimed.
    pub execution_rows_attributed_or_narrowed: bool,
    /// Deviation notes are attributed or narrowed, never falsely claimed.
    pub deviation_notes_attributed_or_narrowed: bool,
    /// Every deviation from the runbook is recorded as a first-class note.
    pub deviations_recorded_as_first_class: bool,
    /// Export bundles track the provenance of what they package.
    pub export_bundles_provenance_tracked: bool,
    /// No provenance is claimed without backing evidence.
    pub no_provenance_claimed_without_evidence: bool,
}

/// Stale-state honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionStaleStateHonesty {
    /// Every stale item is labeled.
    pub stale_items_labeled: bool,
    /// Every unknown-freshness item is labeled.
    pub unknown_freshness_labeled: bool,
    /// A stale item is never shown as live.
    pub never_show_stale_as_live: bool,
    /// A freshness floor is enforced before an item is shown.
    pub freshness_floor_enforced: bool,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionSecurityReview {
    /// An execution row never applies an automated step without host approval.
    pub execution_row_read_only_unless_host_approved: bool,
    /// The deviation notes are read-only.
    pub deviation_note_read_only: bool,
    /// The export bundles are read-only.
    pub export_bundle_read_only: bool,
    /// The external handoff is read-only.
    pub external_handoff_read_only: bool,
    /// Every external handoff requires provider continuity.
    pub external_handoff_requires_provider_continuity: bool,
    /// Every external-handoff item keeps an exact local desktop fallback.
    pub local_fallback_always_available: bool,
    /// No unbounded workspace write authority is exposed.
    pub no_unbounded_workspace_write: bool,
    /// The desktop host stays authoritative.
    pub host_stays_authoritative: bool,
    /// Incident attribution is preserved or honestly narrowed.
    pub incident_attribution_preserved: bool,
    /// Deviations are recorded rather than hidden.
    pub deviations_recorded_not_hidden: bool,
    /// An incomplete export bundle is recorded rather than claimed complete.
    pub incomplete_bundle_recorded_not_hidden: bool,
    /// Stale state is labeled rather than hidden.
    pub stale_state_labeled_never_hidden: bool,
    /// Exact desktop handoff is preserved or honestly degraded.
    pub exact_desktop_handoff_preserved: bool,
    /// Export bundles carry no credential bodies or raw provider payloads.
    pub export_bundle_carries_no_payload_bodies: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the section.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Every section discloses local, staged, and provider/admin continuity.
    pub locality_disclosed: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionConsumerProjection {
    /// Incident workspace projects the execution rows.
    pub incident_workspace_shows_execution_rows: bool,
    /// Incident workspace projects the deviation notes.
    pub incident_workspace_shows_deviation_notes: bool,
    /// Incident workspace projects the export bundles.
    pub incident_workspace_shows_export_bundles: bool,
    /// Incident workspace projects the external handoffs.
    pub incident_workspace_shows_external_handoffs: bool,
    /// Desktop panel shows the desktop handoff targets.
    pub desktop_panel_shows_handoff_target: bool,
    /// Browser companion shows the external handoff with its provider-continuity label.
    pub browser_companion_shows_external_handoff: bool,
    /// Support export shows attribution and freshness state.
    pub support_export_shows_attribution_and_freshness: bool,
    /// Diagnostics shows deviation and stale labels.
    pub diagnostics_shows_deviation_and_stale_labels: bool,
    /// Preview / Labs sections are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_sections: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the section.
    pub auto_narrow_on_stale: bool,
}

/// Per-observation signal fed to
/// [`RunbookExecutionSurfacePacket::apply_runbook_execution_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunbookExecutionObservation {
    /// True when the companion relay is available.
    pub relay_available: bool,
    /// True when proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an active desktop host session exists.
    pub host_session_active: bool,
    /// True when workspace and device trust are intact.
    pub trust_intact: bool,
    /// True when incident attribution to evidence and build identity is intact.
    pub incident_attribution_intact: bool,
    /// True when every export bundle is fully ready.
    pub export_complete: bool,
    /// True when the external browser/vendor console is reachable.
    pub external_reachable: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Reason a runbook execution section has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookExecutionDegradedReason {
    /// The companion relay is unavailable.
    RelayUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// No active desktop host session.
    HostSessionInactive,
    /// Workspace or device trust narrowed.
    TrustNarrowed,
    /// Incident attribution to evidence or build identity was lost.
    IncidentAttributionLost,
    /// One or more export bundles are incomplete.
    ExportBundleIncomplete,
    /// The external browser/vendor console is unreachable.
    ExternalHandoffUnavailable,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more desktop handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
    /// One or more item freshness states were downgraded to stale.
    FreshnessDowngradedToStale,
}

impl RunbookExecutionDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelayUnavailable => "relay_unavailable",
            Self::ProofStale => "proof_stale",
            Self::HostSessionInactive => "host_session_inactive",
            Self::TrustNarrowed => "trust_narrowed",
            Self::IncidentAttributionLost => "incident_attribution_lost",
            Self::ExportBundleIncomplete => "export_bundle_incomplete",
            Self::ExternalHandoffUnavailable => "external_handoff_unavailable",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
        }
    }
}

/// Constructor input for [`RunbookExecutionSurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunbookExecutionSurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<RunbookExecutionSectionQualification>,
    /// Execution-row items.
    pub execution_rows: Vec<RunbookExecutionRowItem>,
    /// Deviation-note items.
    pub deviation_notes: Vec<DeviationNoteItem>,
    /// Export-bundle items.
    pub export_bundles: Vec<ExportBundleItem>,
    /// External-handoff items.
    pub external_handoffs: Vec<ExternalHandoffItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: RunbookExecutionScopeContract,
    /// Attribution contract.
    pub attribution_contract: RunbookExecutionAttributionContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: RunbookExecutionStaleStateHonesty,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: RunbookExecutionSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: RunbookExecutionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RunbookExecutionProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe runbook execution surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookExecutionSurfacePacket {
    /// Record kind; must equal [`RUNBOOK_EXECUTION_SURFACE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RUNBOOK_EXECUTION_SURFACE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<RunbookExecutionSectionQualification>,
    /// Execution-row items.
    pub execution_rows: Vec<RunbookExecutionRowItem>,
    /// Deviation-note items.
    pub deviation_notes: Vec<DeviationNoteItem>,
    /// Export-bundle items.
    pub export_bundles: Vec<ExportBundleItem>,
    /// External-handoff items.
    pub external_handoffs: Vec<ExternalHandoffItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: RunbookExecutionScopeContract,
    /// Attribution contract.
    pub attribution_contract: RunbookExecutionAttributionContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: RunbookExecutionStaleStateHonesty,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: RunbookExecutionSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: RunbookExecutionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RunbookExecutionProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<RunbookExecutionDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RunbookExecutionSurfacePacket {
    /// Builds a runbook execution surface packet from stable-lane input.
    pub fn new(input: RunbookExecutionSurfacePacketInput) -> Self {
        Self {
            record_kind: RUNBOOK_EXECUTION_SURFACE_RECORD_KIND.to_owned(),
            schema_version: RUNBOOK_EXECUTION_SURFACE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            section_qualifications: input.section_qualifications,
            execution_rows: input.execution_rows,
            deviation_notes: input.deviation_notes,
            export_bundles: input.export_bundles,
            external_handoffs: input.external_handoffs,
            scope_contract: input.scope_contract,
            attribution_contract: input.attribution_contract,
            stale_state_honesty: input.stale_state_honesty,
            locality_disclosure: input.locality_disclosure,
            security_review: input.security_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            degraded_labels: Vec::new(),
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows sections and downgrades freshness, attribution, and handoff
    /// resolution from a per-observation signal, recording the reasons in
    /// [`Self::degraded_labels`].
    ///
    /// An unavailable relay, stale proof, or narrowed upstream matrix lane narrows
    /// every section's qualification and rollout stage one step, and an unavailable
    /// relay additionally forces every live or cached item to stale and labels it.
    /// Lost incident attribution marks every execution row and deviation note
    /// unattributed and narrows the execution-row and deviation-note sections.
    /// An incomplete export narrows every ready bundle to partial, labels it, and
    /// narrows the export-bundle section. An unreachable external console marks every
    /// exact external handoff unresolved and narrows the external-handoff section.
    /// Narrowed trust narrows the execution-row section (which can carry an approved
    /// automated action) and the external-handoff section (the most provider-dependent
    /// one). An inactive host session downgrades the resolution of every desktop
    /// handoff that requires an active host and narrows the execution-row section,
    /// since an approved action can no longer be relayed. Degraded state is labeled,
    /// never hidden.
    pub fn apply_runbook_execution_degradation(
        &mut self,
        observation: &RunbookExecutionObservation,
    ) {
        let mut labels: BTreeSet<RunbookExecutionDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let section_adverse = !observation.relay_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.relay_available {
            labels.insert(RunbookExecutionDegradedReason::RelayUnavailable);
            if self.force_all_freshness_stale() {
                labels.insert(RunbookExecutionDegradedReason::FreshnessDowngradedToStale);
            }
        }
        if !observation.proof_fresh {
            labels.insert(RunbookExecutionDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(RunbookExecutionDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.trust_intact {
            labels.insert(RunbookExecutionDegradedReason::TrustNarrowed);
        }
        if !observation.incident_attribution_intact {
            labels.insert(RunbookExecutionDegradedReason::IncidentAttributionLost);
            for item in &mut self.execution_rows {
                item.attribution = IncidentAttributionState::Unattributed;
            }
            for item in &mut self.deviation_notes {
                item.attribution = IncidentAttributionState::Unattributed;
            }
        }
        if !observation.export_complete {
            labels.insert(RunbookExecutionDegradedReason::ExportBundleIncomplete);
            for item in &mut self.export_bundles {
                if item.bundle_state != item.bundle_state.narrowed() {
                    item.bundle_state = item.bundle_state.narrowed();
                }
                if item.bundle_state.is_incomplete() {
                    item.incomplete_label_shown = true;
                }
            }
        }
        if !observation.external_reachable {
            labels.insert(RunbookExecutionDegradedReason::ExternalHandoffUnavailable);
            for item in &mut self.external_handoffs {
                if item.external.resolution == CompanionHandoffResolution::Exact {
                    item.external.resolution = CompanionHandoffResolution::Unresolved;
                }
            }
        }

        for row in &mut self.section_qualifications {
            let adverse = section_adverse
                || (!observation.trust_intact
                    && matches!(
                        row.section,
                        RunbookExecutionSection::ExecutionRow
                            | RunbookExecutionSection::ExternalHandoff
                    ))
                || (!observation.host_session_active
                    && row.section == RunbookExecutionSection::ExecutionRow)
                || (!observation.incident_attribution_intact
                    && matches!(
                        row.section,
                        RunbookExecutionSection::ExecutionRow
                            | RunbookExecutionSection::DeviationNote
                    ))
                || (!observation.export_complete
                    && row.section == RunbookExecutionSection::ExportBundle)
                || (!observation.external_reachable
                    && row.section == RunbookExecutionSection::ExternalHandoff);
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(RunbookExecutionDegradedReason::HostSessionInactive);
            let mut any_unresolved = false;
            for handoff in self.handoffs_mut() {
                if handoff.requires_active_host
                    && handoff.resolution == CompanionHandoffResolution::Exact
                {
                    handoff.resolution = CompanionHandoffResolution::Unresolved;
                    any_unresolved = true;
                }
            }
            if any_unresolved {
                labels.insert(RunbookExecutionDegradedReason::HandoffTargetUnresolved);
            }
        }

        self.degraded_labels = labels.into_iter().collect();
    }

    /// Forces every live/cached item freshness to stale and labels it. Returns
    /// true when at least one item was downgraded.
    fn force_all_freshness_stale(&mut self) -> bool {
        let mut downgraded = false;
        for freshness in self.freshness_states_mut() {
            let (state, label) = freshness;
            if *state != state.forced_stale() {
                *state = state.forced_stale();
                *label = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Mutable access to every item's freshness state and stale-label flag.
    fn freshness_states_mut(
        &mut self,
    ) -> impl Iterator<Item = (&mut CompanionFreshnessState, &mut bool)> {
        self.execution_rows
            .iter_mut()
            .map(|item| (&mut item.freshness, &mut item.stale_label_shown))
            .chain(
                self.deviation_notes
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.export_bundles
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.external_handoffs
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
    }

    /// Validates the runbook execution surface invariants.
    pub fn validate(&self) -> Vec<RunbookExecutionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RUNBOOK_EXECUTION_SURFACE_RECORD_KIND {
            violations.push(RunbookExecutionViolation::WrongRecordKind);
        }
        if self.schema_version != RUNBOOK_EXECUTION_SURFACE_SCHEMA_VERSION {
            violations.push(RunbookExecutionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RunbookExecutionViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(RunbookExecutionViolation::ProjectedSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_section_qualifications(self, &mut violations);
        validate_items(self, &mut violations);
        validate_scope_contract(self, &mut violations);
        validate_attribution_contract(self, &mut violations);
        validate_stale_state_honesty(self, &mut violations);
        validate_locality(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("runbook execution packet serializes"),
        ) {
            violations.push(RunbookExecutionViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("runbook execution packet serializes")
    }

    /// Sections currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_sections(
        &self,
    ) -> impl Iterator<Item = &RunbookExecutionSectionQualification> {
        self.section_qualifications.iter().filter(|row| {
            matches!(
                row.qualification,
                M5CompanionQualificationClass::Stable
                    | M5CompanionQualificationClass::Beta
                    | M5CompanionQualificationClass::Preview
            ) && row.rollout_stage != M5CompanionRolloutStage::Withheld
        })
    }

    /// True when every item's desktop handoff resolves to the exact location.
    pub fn all_handoffs_exact(&self) -> bool {
        self.handoffs()
            .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact)
    }

    /// True when every external handoff carries a local desktop fallback and
    /// discloses that it requires provider continuity.
    pub fn external_handoffs_have_local_fallback(&self) -> bool {
        self.external_handoffs.iter().all(|item| {
            item.external.local_fallback_available
                && item.external.requires_provider_continuity
                && !item.handoff.deep_link_ref.trim().is_empty()
        })
    }

    /// True when every stale or unknown-freshness item carries a visible label.
    pub fn stale_state_honestly_labeled(&self) -> bool {
        self.execution_rows
            .iter()
            .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .deviation_notes
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .export_bundles
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .external_handoffs
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
    }

    /// True when every incomplete export bundle carries a visible incomplete label.
    pub fn export_bundles_honestly_labeled(&self) -> bool {
        self.export_bundles
            .iter()
            .all(|item| !item.bundle_state.is_incomplete() || item.incomplete_label_shown)
    }

    /// Iterates every desktop handoff across all four sections, in section order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.execution_rows
            .iter()
            .map(|item| &item.handoff)
            .chain(self.deviation_notes.iter().map(|item| &item.handoff))
            .chain(self.export_bundles.iter().map(|item| &item.handoff))
            .chain(self.external_handoffs.iter().map(|item| &item.handoff))
    }

    fn handoffs_mut(&mut self) -> impl Iterator<Item = &mut CompanionDesktopHandoff> {
        self.execution_rows
            .iter_mut()
            .map(|item| &mut item.handoff)
            .chain(
                self.deviation_notes
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(self.export_bundles.iter_mut().map(|item| &mut item.handoff))
            .chain(
                self.external_handoffs
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Runbook Execution Rows, Deviation Notes, Export Bundles, and Browser or Vendor-Console Handoff\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Sections: {} | Execution rows: {} | Deviation notes: {} | Export bundles: {} | External handoffs: {}\n",
            self.section_qualifications.len(),
            self.execution_rows.len(),
            self.deviation_notes.len(),
            self.export_bundles.len(),
            self.external_handoffs.len(),
        ));
        out.push_str(&format!(
            "- Exact desktop handoff for every item: {}\n",
            if self.all_handoffs_exact() {
                "yes"
            } else {
                "no"
            }
        ));
        out.push_str(&format!(
            "- External handoffs keep a local fallback: {}\n",
            if self.external_handoffs_have_local_fallback() {
                "yes"
            } else {
                "no"
            }
        ));
        out.push_str(&format!(
            "- Stale state honestly labeled: {}\n",
            if self.stale_state_honestly_labeled() {
                "yes"
            } else {
                "no"
            }
        ));
        out.push_str(&format!(
            "- Export bundles honestly labeled: {}\n",
            if self.export_bundles_honestly_labeled() {
                "yes"
            } else {
                "no"
            }
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        if self.degraded_labels.is_empty() {
            out.push_str("- Degraded: none\n");
        } else {
            let labels = self
                .degraded_labels
                .iter()
                .map(|reason| reason.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!("- Degraded: {labels}\n"));
        }

        out.push_str("\n## Sections\n\n");
        for row in &self.section_qualifications {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}` [{}] (matrix lane `{}`)\n",
                row.section.as_str(),
                row.qualification.as_str(),
                row.rollout_stage.as_str(),
                row.read_write_scope.as_str(),
                row.matrix_lane_ref,
            ));
        }

        out.push_str("\n## Execution rows\n\n");
        for item in &self.execution_rows {
            out.push_str(&format!(
                "- `{}` #{} [{}/{}/{}] {} — {} ({}) → `{}` ({})\n",
                item.item_id,
                item.step_index,
                item.step_state.as_str(),
                item.outcome.as_str(),
                item.attribution.as_str(),
                item.automation_class.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Deviation notes\n\n");
        for item in &self.deviation_notes {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] {} ({}) → `{}` ({})\n",
                item.item_id,
                item.kind.as_str(),
                item.significance.as_str(),
                item.attribution.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Export bundles\n\n");
        for item in &self.export_bundles {
            out.push_str(&format!(
                "- `{}` [{}/{}] {} ({}) → `{}` ({})\n",
                item.item_id,
                item.bundle_kind.as_str(),
                item.bundle_state.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## External handoffs\n\n");
        for item in &self.external_handoffs {
            out.push_str(&format!(
                "- `{}` [{}] {} — {} ({}) → external `{}` ({}), local `{}` ({})\n",
                item.item_id,
                item.external.target.as_str(),
                item.external.provider_label,
                item.summary,
                item.freshness.as_str(),
                item.external.deep_link_ref,
                item.external.resolution.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out
    }
}

/// Errors emitted when reading the checked-in runbook execution export.
#[derive(Debug)]
pub enum RunbookExecutionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RunbookExecutionViolation>),
}

impl fmt::Display for RunbookExecutionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "runbook execution export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "runbook execution export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RunbookExecutionArtifactError {}

/// Validation failures emitted by [`RunbookExecutionSurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RunbookExecutionViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Projected surfaces list is empty.
    ProjectedSurfacesMissing,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required section qualification row is missing.
    RequiredSectionMissing,
    /// A section row's matrix lane ref does not match its section.
    SectionLaneMismatch,
    /// A section row's read/write scope does not match its bounded scope.
    SectionScopeMismatch,
    /// A section row is incomplete.
    SectionRowIncomplete,
    /// A section has no content items.
    SectionContentMissing,
    /// A read-only section item is not marked read-only.
    ReadOnlyScopeViolated,
    /// An execution row that can carry automation does not require host approval.
    ExecutionAutomationNotApproved,
    /// An export bundle is not redaction-checked.
    ExportBundleNotRedactionChecked,
    /// An incomplete export bundle is not labeled.
    ExportBundleIncompleteNotLabeled,
    /// An external handoff does not require provider continuity.
    ExternalHandoffContinuityNotDisclosed,
    /// An external handoff has no local desktop fallback.
    ExternalHandoffMissingLocalFallback,
    /// An item is missing identity or a redacted body, or has a payload-like body.
    ItemIncomplete,
    /// A stale or unknown-freshness item is not labeled.
    StaleStateNotLabeled,
    /// An item's desktop handoff is missing its deep-link ref.
    HandoffRefMissing,
    /// An external handoff is missing its deep-link ref or provider label.
    ExternalHandoffRefMissing,
    /// The read/write scope contract is not fully satisfied.
    ScopeContractIncomplete,
    /// The attribution contract is not fully satisfied.
    AttributionContractIncomplete,
    /// The stale-state honesty contract is not fully satisfied.
    StaleStateHonestyIncomplete,
    /// The locality disclosure is incomplete.
    LocalityDisclosureIncomplete,
    /// Security review does not satisfy required invariants.
    SecurityReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl RunbookExecutionViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::ProjectedSurfacesMissing => "projected_surfaces_missing",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSectionMissing => "required_section_missing",
            Self::SectionLaneMismatch => "section_lane_mismatch",
            Self::SectionScopeMismatch => "section_scope_mismatch",
            Self::SectionRowIncomplete => "section_row_incomplete",
            Self::SectionContentMissing => "section_content_missing",
            Self::ReadOnlyScopeViolated => "read_only_scope_violated",
            Self::ExecutionAutomationNotApproved => "execution_automation_not_approved",
            Self::ExportBundleNotRedactionChecked => "export_bundle_not_redaction_checked",
            Self::ExportBundleIncompleteNotLabeled => "export_bundle_incomplete_not_labeled",
            Self::ExternalHandoffContinuityNotDisclosed => {
                "external_handoff_continuity_not_disclosed"
            }
            Self::ExternalHandoffMissingLocalFallback => "external_handoff_missing_local_fallback",
            Self::ItemIncomplete => "item_incomplete",
            Self::StaleStateNotLabeled => "stale_state_not_labeled",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::ExternalHandoffRefMissing => "external_handoff_ref_missing",
            Self::ScopeContractIncomplete => "scope_contract_incomplete",
            Self::AttributionContractIncomplete => "attribution_contract_incomplete",
            Self::StaleStateHonestyIncomplete => "stale_state_honesty_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable runbook execution surface export.
///
/// This is the canonical reader: the incident workspace, the desktop companion
/// panel, the browser companion, diagnostics, support-export, or Help/About surface
/// calls it to ingest the packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`RunbookExecutionArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_stable_runbook_execution_surface_export(
) -> Result<RunbookExecutionSurfacePacket, RunbookExecutionArtifactError> {
    let packet: RunbookExecutionSurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/implement_runbook_execution_rows_deviation_notes_export_bundles_and_browser_or_vendor_console_handoff_truth/support_export.json"
    )))
    .map_err(RunbookExecutionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RunbookExecutionArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every runbook execution export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        RUNBOOK_EXECUTION_SURFACE_SCHEMA_REF.to_owned(),
        RUNBOOK_EXECUTION_SURFACE_DOC_REF.to_owned(),
        M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_QUALIFICATION_REF.to_owned(),
        M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
        M5_INCIDENT_WORKSPACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical read/write scope and authority contract with every guarantee met.
pub fn canonical_scope_contract() -> RunbookExecutionScopeContract {
    RunbookExecutionScopeContract {
        execution_row_read_only_unless_host_approved: true,
        deviation_note_read_only: true,
        export_bundle_read_only: true,
        external_handoff_read_only: true,
        external_handoff_requires_provider_continuity: true,
        local_fallback_always_available: true,
        no_unbounded_workspace_write: true,
        host_authoritative: true,
        no_payload_bodies: true,
    }
}

/// Canonical attribution contract with every guarantee satisfied.
pub fn canonical_attribution_contract() -> RunbookExecutionAttributionContract {
    RunbookExecutionAttributionContract {
        execution_rows_attributed_or_narrowed: true,
        deviation_notes_attributed_or_narrowed: true,
        deviations_recorded_as_first_class: true,
        export_bundles_provenance_tracked: true,
        no_provenance_claimed_without_evidence: true,
    }
}

/// Canonical stale-state honesty contract with every guarantee satisfied.
pub fn canonical_stale_state_honesty() -> RunbookExecutionStaleStateHonesty {
    RunbookExecutionStaleStateHonesty {
        stale_items_labeled: true,
        unknown_freshness_labeled: true,
        never_show_stale_as_live: true,
        freshness_floor_enforced: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> RunbookExecutionSecurityReview {
    RunbookExecutionSecurityReview {
        execution_row_read_only_unless_host_approved: true,
        deviation_note_read_only: true,
        export_bundle_read_only: true,
        external_handoff_read_only: true,
        external_handoff_requires_provider_continuity: true,
        local_fallback_always_available: true,
        no_unbounded_workspace_write: true,
        host_stays_authoritative: true,
        incident_attribution_preserved: true,
        deviations_recorded_not_hidden: true,
        incomplete_bundle_recorded_not_hidden: true,
        stale_state_labeled_never_hidden: true,
        exact_desktop_handoff_preserved: true,
        export_bundle_carries_no_payload_bodies: true,
        no_payload_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every section projecting truth.
pub fn canonical_consumer_projection() -> RunbookExecutionConsumerProjection {
    RunbookExecutionConsumerProjection {
        incident_workspace_shows_execution_rows: true,
        incident_workspace_shows_deviation_notes: true,
        incident_workspace_shows_export_bundles: true,
        incident_workspace_shows_external_handoffs: true,
        desktop_panel_shows_handoff_target: true,
        browser_companion_shows_external_handoff: true,
        support_export_shows_attribution_and_freshness: true,
        diagnostics_shows_deviation_and_stale_labels: true,
        preview_labs_label_for_unqualified_sections: true,
    }
}

/// Canonical per-section qualification rows, inherited from the frozen matrix.
pub fn canonical_section_qualifications() -> Vec<RunbookExecutionSectionQualification> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;

    let lane_ref = RunbookExecutionSection::ExecutionRow.matrix_lane().as_str();
    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        RunbookExecutionSectionQualification {
            section: RunbookExecutionSection::ExecutionRow,
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            read_write_scope: scope,
            matrix_lane_ref: lane_ref.to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::IncidentAttributionMissing,
                Trigger::TrustNarrowing,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
        },
        RunbookExecutionSectionQualification {
            section: RunbookExecutionSection::DeviationNote,
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            read_write_scope: scope,
            matrix_lane_ref: lane_ref.to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::IncidentAttributionMissing,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::EvidencePreservedNoRevert,
        },
        RunbookExecutionSectionQualification {
            section: RunbookExecutionSection::ExportBundle,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: lane_ref.to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::ProviderUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
        },
        RunbookExecutionSectionQualification {
            section: RunbookExecutionSection::ExternalHandoff,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: scope,
            matrix_lane_ref: lane_ref.to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::TrustNarrowing,
                Trigger::PolicyBlocked,
                Trigger::CompanionScopeExpansionUnqualified,
            ],
            rollback_posture: Rollback::StagedReversibleViaRollout,
        },
    ]
}

/// Canonical locality disclosure for the runbook execution surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "Runbook execution rows, deviation notes, and the exact desktop handoff for every item are owned by the local core and stay inspectable offline."
                .to_owned(),
        staged:
            "Export-bundle building and browser/vendor-console handoff roll out per cohort and capability gate."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Resuming an item in a browser or vendor console, and relaying a host-approved execution action, require provider continuity and an active host session; the local core and its desktop handoff never depend on them to function."
                .to_owned(),
    }
}

fn desktop_handoff(
    target: CompanionHandoffTarget,
    deep_link_ref: &str,
    requires_active_host: bool,
) -> CompanionDesktopHandoff {
    CompanionDesktopHandoff {
        target,
        deep_link_ref: deep_link_ref.to_owned(),
        resolution: CompanionHandoffResolution::Exact,
        requires_active_host,
    }
}

/// Canonical runbook execution-row items.
pub fn canonical_execution_rows() -> Vec<RunbookExecutionRowItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use IncidentAttributionState as Attribution;
    use RunbookAutomationClass as Automation;
    use RunbookExecutionOutcome as Outcome;
    use RunbookStepState as StepState;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        RunbookExecutionRowItem {
            item_id: "exec:0001".to_owned(),
            runbook_ref: "runbook:packet:0001".to_owned(),
            incident_id: "incident:0001".to_owned(),
            step_index: 1,
            step_state: StepState::Completed,
            outcome: Outcome::Succeeded,
            automation_class: Automation::Manual,
            requires_host_approval: true,
            attribution: Attribution::Attributed,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "First mitigation step completed successfully".to_owned(),
            execution_ref: "execution:row:0001".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:exec:0001", false),
        },
        RunbookExecutionRowItem {
            item_id: "exec:0002".to_owned(),
            runbook_ref: "runbook:packet:0002".to_owned(),
            incident_id: "incident:0001".to_owned(),
            step_index: 2,
            step_state: StepState::InProgress,
            outcome: Outcome::InFlight,
            automation_class: Automation::AutomatedWithApproval,
            requires_host_approval: true,
            attribution: Attribution::Attributed,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Automated rollback step in progress; awaiting host approval to apply"
                .to_owned(),
            execution_ref: "execution:row:0002".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:exec:0002", true),
        },
        RunbookExecutionRowItem {
            item_id: "exec:0003".to_owned(),
            runbook_ref: "runbook:packet:0002".to_owned(),
            incident_id: "incident:0001".to_owned(),
            step_index: 3,
            step_state: StepState::Skipped,
            outcome: Outcome::Deviated,
            automation_class: Automation::AssistedSuggestion,
            requires_host_approval: true,
            attribution: Attribution::Attributed,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Third step skipped by the operator; recorded as a deviation".to_owned(),
            execution_ref: "execution:row:0003".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:exec:0003", false),
        },
    ]
}

/// Canonical deviation-note items.
pub fn canonical_deviation_notes() -> Vec<DeviationNoteItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use DeviationNoteKind as Kind;
    use DeviationSignificance as Significance;
    use IncidentAttributionState as Attribution;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        DeviationNoteItem {
            item_id: "deviation:0001".to_owned(),
            execution_row_ref: "execution:row:0003".to_owned(),
            kind: Kind::StepSkipped,
            significance: Significance::Notable,
            attribution: Attribution::Attributed,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Operator skipped the cache-flush step; redundant for this incident"
                .to_owned(),
            note_ref: "deviation:note:0001".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:deviation:0001", false),
        },
        DeviationNoteItem {
            item_id: "deviation:0002".to_owned(),
            execution_row_ref: "execution:row:0002".to_owned(),
            kind: Kind::ParameterChanged,
            significance: Significance::Major,
            attribution: Attribution::PartiallyAttributed,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary: "Rollback target changed from the runbook default; attribution partial"
                .to_owned(),
            note_ref: "deviation:note:0002".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:deviation:0002", false),
        },
    ]
}

/// Canonical export-bundle items.
pub fn canonical_export_bundles() -> Vec<ExportBundleItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use ExportBundleKind as Kind;
    use ExportBundleState as BundleState;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        ExportBundleItem {
            item_id: "bundle:0001".to_owned(),
            bundle_kind: Kind::RunbookExecutionTranscript,
            bundle_state: BundleState::Ready,
            redaction_checked: true,
            incomplete_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Redacted transcript of the runbook execution rows".to_owned(),
            bundle_ref: "export:bundle:0001".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:bundle:0001", false),
        },
        ExportBundleItem {
            item_id: "bundle:0002".to_owned(),
            bundle_kind: Kind::FullIncidentArchive,
            bundle_state: BundleState::Partial,
            redaction_checked: true,
            incomplete_label_shown: true,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Full incident archive; one evidence input missing, labeled partial"
                .to_owned(),
            bundle_ref: "export:bundle:0002".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:bundle:0002", false),
        },
    ]
}

/// Canonical external-handoff items (browser or vendor-console).
pub fn canonical_external_handoffs() -> Vec<ExternalHandoffItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffResolution as Resolution;
    use CompanionHandoffTarget as Target;
    use ExternalHandoffTarget as External;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        ExternalHandoffItem {
            item_id: "external:0001".to_owned(),
            external: ExternalHandoff {
                target: External::BrowserCompanionTab,
                provider_label: "Browser companion".to_owned(),
                deep_link_ref: "external:browser:incident-0001".to_owned(),
                resolution: Resolution::Exact,
                requires_provider_continuity: true,
                local_fallback_available: true,
            },
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Resume the incident in a browser companion tab; local desktop fallback kept"
                .to_owned(),
            target_ref: "external:target:browser-0001".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:external:0001", false),
        },
        ExternalHandoffItem {
            item_id: "external:0002".to_owned(),
            external: ExternalHandoff {
                target: External::VendorIncidentConsole,
                provider_label: "Vendor incident console".to_owned(),
                deep_link_ref: "external:vendor-console:incident-0001".to_owned(),
                resolution: Resolution::Exact,
                requires_provider_continuity: true,
                local_fallback_available: true,
            },
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Open the incident in the vendor incident console; requires provider continuity"
                    .to_owned(),
            target_ref: "external:target:vendor-console-0001".to_owned(),
            handoff: desktop_handoff(Target::IncidentWorkspace, "handoff:external:0002", false),
        },
    ]
}

/// Builds the canonical runbook execution surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export
/// and Markdown summary are generated from, so the artifact never drifts from the
/// typed section, item, scope, attribution, and freshness definitions.
pub fn canonical_runbook_execution_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: RunbookExecutionProofFreshness,
) -> RunbookExecutionSurfacePacket {
    RunbookExecutionSurfacePacket::new(RunbookExecutionSurfacePacketInput {
        packet_id,
        surface_label,
        projected_surfaces: vec![
            M5CompanionConsumerSurface::IncidentWorkspace,
            M5CompanionConsumerSurface::DesktopCompanionPanel,
            M5CompanionConsumerSurface::BrowserCompanion,
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::Diagnostics,
            M5CompanionConsumerSurface::HelpAbout,
        ],
        section_qualifications: canonical_section_qualifications(),
        execution_rows: canonical_execution_rows(),
        deviation_notes: canonical_deviation_notes(),
        export_bundles: canonical_export_bundles(),
        external_handoffs: canonical_external_handoffs(),
        scope_contract: canonical_scope_contract(),
        attribution_contract: canonical_attribution_contract(),
        stale_state_honesty: canonical_stale_state_honesty(),
        locality_disclosure: canonical_locality_disclosure(),
        security_review: canonical_security_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

fn validate_source_contracts(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RUNBOOK_EXECUTION_SURFACE_SCHEMA_REF,
        RUNBOOK_EXECUTION_SURFACE_DOC_REF,
        M5_COMPANION_SURFACE_CONTRACT_REF,
        M5_INCIDENT_WORKSPACE_CONTRACT_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RunbookExecutionViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_section_qualifications(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    let present: BTreeSet<RunbookExecutionSection> = packet
        .section_qualifications
        .iter()
        .map(|row| row.section)
        .collect();
    for required in RunbookExecutionSection::ALL {
        if !present.contains(&required) {
            violations.push(RunbookExecutionViolation::RequiredSectionMissing);
            return;
        }
    }

    for row in &packet.section_qualifications {
        if row.matrix_lane_ref != row.section.matrix_lane().as_str() {
            violations.push(RunbookExecutionViolation::SectionLaneMismatch);
        }
        if row.read_write_scope != row.section.bounded_scope() {
            violations.push(RunbookExecutionViolation::SectionScopeMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(RunbookExecutionViolation::SectionRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    if packet.execution_rows.is_empty()
        || packet.deviation_notes.is_empty()
        || packet.export_bundles.is_empty()
        || packet.external_handoffs.is_empty()
    {
        violations.push(RunbookExecutionViolation::SectionContentMissing);
    }

    for item in &packet.execution_rows {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(RunbookExecutionViolation::ReadOnlyScopeViolated);
        }
        if !item.requires_host_approval {
            violations.push(RunbookExecutionViolation::ExecutionAutomationNotApproved);
        }
        if item.item_id.trim().is_empty()
            || item.runbook_ref.trim().is_empty()
            || item.incident_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.execution_ref.trim().is_empty()
        {
            violations.push(RunbookExecutionViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.deviation_notes {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(RunbookExecutionViolation::ReadOnlyScopeViolated);
        }
        if item.item_id.trim().is_empty()
            || item.execution_row_ref.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.note_ref.trim().is_empty()
        {
            violations.push(RunbookExecutionViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.export_bundles {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(RunbookExecutionViolation::ReadOnlyScopeViolated);
        }
        if !item.redaction_checked {
            violations.push(RunbookExecutionViolation::ExportBundleNotRedactionChecked);
        }
        if item.bundle_state.is_incomplete() && !item.incomplete_label_shown {
            violations.push(RunbookExecutionViolation::ExportBundleIncompleteNotLabeled);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.bundle_ref.trim().is_empty()
        {
            violations.push(RunbookExecutionViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.external_handoffs {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(RunbookExecutionViolation::ReadOnlyScopeViolated);
        }
        if !item.external.requires_provider_continuity {
            violations.push(RunbookExecutionViolation::ExternalHandoffContinuityNotDisclosed);
        }
        if !item.external.local_fallback_available || item.handoff.deep_link_ref.trim().is_empty() {
            violations.push(RunbookExecutionViolation::ExternalHandoffMissingLocalFallback);
        }
        if item.external.deep_link_ref.trim().is_empty()
            || item.external.provider_label.trim().is_empty()
        {
            violations.push(RunbookExecutionViolation::ExternalHandoffRefMissing);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.target_ref.trim().is_empty()
        {
            violations.push(RunbookExecutionViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }
}

fn validate_freshness_label(
    freshness: CompanionFreshnessState,
    stale_label_shown: bool,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    if freshness.requires_label() && !stale_label_shown {
        violations.push(RunbookExecutionViolation::StaleStateNotLabeled);
    }
}

fn validate_handoff(
    handoff: &CompanionDesktopHandoff,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(RunbookExecutionViolation::HandoffRefMissing);
    }
}

fn validate_scope_contract(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    let contract = &packet.scope_contract;
    for ok in [
        contract.execution_row_read_only_unless_host_approved,
        contract.deviation_note_read_only,
        contract.export_bundle_read_only,
        contract.external_handoff_read_only,
        contract.external_handoff_requires_provider_continuity,
        contract.local_fallback_always_available,
        contract.no_unbounded_workspace_write,
        contract.host_authoritative,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(RunbookExecutionViolation::ScopeContractIncomplete);
            return;
        }
    }
}

fn validate_attribution_contract(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    let contract = &packet.attribution_contract;
    for ok in [
        contract.execution_rows_attributed_or_narrowed,
        contract.deviation_notes_attributed_or_narrowed,
        contract.deviations_recorded_as_first_class,
        contract.export_bundles_provenance_tracked,
        contract.no_provenance_claimed_without_evidence,
    ] {
        if !ok {
            violations.push(RunbookExecutionViolation::AttributionContractIncomplete);
            return;
        }
    }
}

fn validate_stale_state_honesty(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    let honesty = &packet.stale_state_honesty;
    for ok in [
        honesty.stale_items_labeled,
        honesty.unknown_freshness_labeled,
        honesty.never_show_stale_as_live,
        honesty.freshness_floor_enforced,
    ] {
        if !ok {
            violations.push(RunbookExecutionViolation::StaleStateHonestyIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(RunbookExecutionViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.execution_row_read_only_unless_host_approved,
        review.deviation_note_read_only,
        review.export_bundle_read_only,
        review.external_handoff_read_only,
        review.external_handoff_requires_provider_continuity,
        review.local_fallback_always_available,
        review.no_unbounded_workspace_write,
        review.host_stays_authoritative,
        review.incident_attribution_preserved,
        review.deviations_recorded_not_hidden,
        review.incomplete_bundle_recorded_not_hidden,
        review.stale_state_labeled_never_hidden,
        review.exact_desktop_handoff_preserved,
        review.export_bundle_carries_no_payload_bodies,
        review.no_payload_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(RunbookExecutionViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.incident_workspace_shows_execution_rows,
        projection.incident_workspace_shows_deviation_notes,
        projection.incident_workspace_shows_export_bundles,
        projection.incident_workspace_shows_external_handoffs,
        projection.desktop_panel_shows_handoff_target,
        projection.browser_companion_shows_external_handoff,
        projection.support_export_shows_attribution_and_freshness,
        projection.diagnostics_shows_deviation_and_stale_labels,
        projection.preview_labs_label_for_unqualified_sections,
    ] {
        if !ok {
            violations.push(RunbookExecutionViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &RunbookExecutionSurfacePacket,
    violations: &mut Vec<RunbookExecutionViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(RunbookExecutionViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
