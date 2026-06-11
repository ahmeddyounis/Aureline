//! Incident workspace headers, evidence timelines, resource slices, and runbook
//! packets, projected as a downgrade-aware truth packet.
//!
//! This module owns the export-safe truth packet for the incident workspace
//! content: the **header** card that identifies an incident, the ordered
//! **evidence timeline** (including first-class missing spans), the read-only
//! **resource slices** attributed to the incident, and the **runbook packets**
//! that guide mitigation. It binds those four sections to the frozen M5
//! companion-matrix `incident_workspace` lane that qualifies them, and gives
//! every item an exact [`CompanionDesktopHandoff`] (reused from
//! [`crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`])
//! so opening an item resumes the precise host context.
//!
//! Two invariants make this surface safe to ship. First, **attributable and
//! read-mostly**: every section is read-only, the runbook never executes an
//! automated step without explicit host approval, and an incident header or
//! evidence span that loses attribution narrows to
//! [`IncidentAttributionState::Unattributed`] rather than claiming a provenance it
//! can no longer prove. A missing evidence span is recorded as a first-class fact,
//! never silently dropped. Second, **stale-state honesty**: every item carries a
//! [`CompanionFreshnessState`], stale or unknown freshness is always labeled, and a
//! degraded item is never shown as live.
//!
//! The packet reuses the matrix vocabulary from
//! [`crate::freeze_the_m5_companion_incident_sync_and_offboarding_matrix_with_staged_rollout_lanes`]
//! ([`M5CompanionQualificationClass`], [`M5CompanionRolloutStage`],
//! [`M5CompanionDowngradeTrigger`], [`M5CompanionRollbackPosture`],
//! [`M5CompanionLocalityDisclosure`], [`M5CompanionConsumerSurface`]) and the
//! incident severity, attribution, freshness, scope, and handoff vocabulary from
//! [`crate::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty`]
//! and the companion triage surface, instead of inventing parallel terms. Each
//! section row records the matrix lane it inherits qualification from.
//!
//! [`IncidentWorkspaceSurfacePacket::apply_incident_workspace_degradation`] narrows
//! sections and downgrades freshness, attribution, and handoff resolution from a
//! per-observation signal — when the relay is unavailable, proof is stale, the host
//! session is inactive, trust narrowed, incident attribution was lost, evidence is
//! incomplete, or an upstream matrix lane narrowed — so CI or release tooling
//! degrades the surface honestly rather than show fresh state, a proven attribution,
//! or an exact handoff that no longer resolves. Degraded state is labeled, never
//! hidden.
//!
//! [`canonical_incident_workspace_surface`] builds the surface and
//! [`current_stable_incident_workspace_surface_export`] reads and validates the
//! checked-in support export, so the incident workspace, the desktop companion
//! panel, diagnostics, support exports, and Help/About ingest the packet rather
//! than cloning status text. Credential bodies, raw provider payloads, and raw
//! incident, evidence, resource, or runbook bodies stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/companion/add-incident-workspace-headers-evidence-timelines-resource-slices-and-runbook-packets.schema.json`](../../../../schemas/companion/add-incident-workspace-headers-evidence-timelines-resource-slices-and-runbook-packets.schema.json).
//! The contract doc is
//! [`docs/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets.md`](../../../../docs/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets/`](../../../../fixtures/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

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
    CompanionFreshnessState, CompanionReadWriteScope, IncidentAttributionState, IncidentSeverity,
};

/// Stable record-kind tag carried by [`IncidentWorkspaceSurfacePacket`].
pub const INCIDENT_WORKSPACE_SURFACE_RECORD_KIND: &str =
    "add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets";

/// Schema version for incident workspace surface records.
pub const INCIDENT_WORKSPACE_SURFACE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const INCIDENT_WORKSPACE_SURFACE_SCHEMA_REF: &str =
    "schemas/companion/add-incident-workspace-headers-evidence-timelines-resource-slices-and-runbook-packets.schema.json";

/// Repo-relative path of the incident workspace surface contract doc.
pub const INCIDENT_WORKSPACE_SURFACE_DOC_REF: &str =
    "docs/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets.md";

/// Repo-relative path of the protected fixture directory.
pub const INCIDENT_WORKSPACE_SURFACE_FIXTURE_DIR: &str =
    "fixtures/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets";

/// Repo-relative path of the checked support-export artifact.
pub const INCIDENT_WORKSPACE_SURFACE_ARTIFACT_REF: &str =
    "artifacts/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const INCIDENT_WORKSPACE_SURFACE_SUMMARY_REF: &str =
    "artifacts/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets.md";

/// One of the four incident workspace sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentWorkspaceSection {
    /// The incident header card.
    Header,
    /// The ordered evidence timeline.
    EvidenceTimeline,
    /// The read-only resource slices.
    ResourceSlice,
    /// The runbook packets that guide mitigation.
    RunbookPacket,
}

impl IncidentWorkspaceSection {
    /// Every section, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Header,
        Self::EvidenceTimeline,
        Self::ResourceSlice,
        Self::RunbookPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Header => "header",
            Self::EvidenceTimeline => "evidence_timeline",
            Self::ResourceSlice => "resource_slice",
            Self::RunbookPacket => "runbook_packet",
        }
    }

    /// Frozen M5 companion-matrix lane this section inherits qualification from.
    ///
    /// Every incident workspace section inherits from the single
    /// [`M5CompanionMatrixLane::IncidentWorkspace`] lane.
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        M5CompanionMatrixLane::IncidentWorkspace
    }

    /// Read/write scope this section is bounded to.
    ///
    /// Every section is read-only: the incident workspace observes and guides but
    /// never mutates host state directly. A runbook automated action is relayed
    /// for explicit host approval rather than applied from the workspace.
    pub const fn bounded_scope(self) -> CompanionReadWriteScope {
        CompanionReadWriteScope::ReadOnly
    }
}

/// Lifecycle status of an incident, shown on the header card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentWorkspaceStatus {
    /// Newly raised and being triaged.
    Triaging,
    /// Under active investigation.
    Investigating,
    /// Mitigation is in progress.
    Mitigating,
    /// Mitigated and being monitored.
    Monitoring,
    /// Resolved.
    Resolved,
    /// Closed out.
    Closed,
}

impl IncidentWorkspaceStatus {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Triaging => "triaging",
            Self::Investigating => "investigating",
            Self::Mitigating => "mitigating",
            Self::Monitoring => "monitoring",
            Self::Resolved => "resolved",
            Self::Closed => "closed",
        }
    }
}

/// Kind of evidence captured by an evidence-timeline span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSpanKind {
    /// A crash trail span.
    CrashTrail,
    /// A log window span.
    LogWindow,
    /// A metric series span.
    MetricSeries,
    /// A user-report span.
    UserReport,
    /// A diagnostic-bundle span.
    DiagnosticBundle,
    /// A build-artifact span.
    BuildArtifact,
}

impl EvidenceSpanKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CrashTrail => "crash_trail",
            Self::LogWindow => "log_window",
            Self::MetricSeries => "metric_series",
            Self::UserReport => "user_report",
            Self::DiagnosticBundle => "diagnostic_bundle",
            Self::BuildArtifact => "build_artifact",
        }
    }
}

/// Completeness state of an evidence-timeline span.
///
/// A [`Self::Missing`] span is recorded as a first-class fact so the timeline
/// never silently hides a gap. [`Self::is_gap`] marks the states that represent an
/// incomplete span.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceSpanState {
    /// The span is present and complete.
    Present,
    /// The span is partially captured.
    Partial,
    /// The span is missing; recorded as a first-class gap.
    Missing,
}

impl EvidenceSpanState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Partial => "partial",
            Self::Missing => "missing",
        }
    }

    /// True when the span represents an incomplete or missing gap.
    pub const fn is_gap(self) -> bool {
        matches!(self, Self::Partial | Self::Missing)
    }

    /// Narrows a present span to partial; honest gaps are kept.
    pub const fn narrowed(self) -> Self {
        match self {
            Self::Present => Self::Partial,
            Self::Partial | Self::Missing => self,
        }
    }
}

/// Kind of read-only resource slice attributed to an incident.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceSliceKind {
    /// A CPU profile slice.
    CpuProfile,
    /// A memory snapshot slice.
    MemorySnapshot,
    /// A log slice.
    LogSlice,
    /// A thread-dump slice.
    ThreadDump,
    /// A network-trace slice.
    NetworkTrace,
    /// A disk-I/O slice.
    DiskIo,
}

impl ResourceSliceKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CpuProfile => "cpu_profile",
            Self::MemorySnapshot => "memory_snapshot",
            Self::LogSlice => "log_slice",
            Self::ThreadDump => "thread_dump",
            Self::NetworkTrace => "network_trace",
            Self::DiskIo => "disk_io",
        }
    }
}

/// Automation class of a runbook packet.
///
/// A runbook never executes an automated step from the workspace; an automated
/// action is relayed for explicit host approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookAutomationClass {
    /// Manual guidance only.
    Manual,
    /// An assisted suggestion the operator runs.
    AssistedSuggestion,
    /// An automated action that still requires explicit host approval.
    AutomatedWithApproval,
}

impl RunbookAutomationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::AssistedSuggestion => "assisted_suggestion",
            Self::AutomatedWithApproval => "automated_with_approval",
        }
    }

    /// True when this class can carry an automated action.
    pub const fn carries_automation(self) -> bool {
        matches!(self, Self::AutomatedWithApproval)
    }
}

/// State of the next runbook step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunbookStepState {
    /// The step is pending.
    Pending,
    /// The step is in progress.
    InProgress,
    /// The step is completed.
    Completed,
    /// The step was skipped.
    Skipped,
    /// The step is blocked.
    Blocked,
}

impl RunbookStepState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Completed => "completed",
            Self::Skipped => "skipped",
            Self::Blocked => "blocked",
        }
    }
}

/// Reason an incident workspace section has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentWorkspaceDegradedReason {
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
    /// Evidence is incomplete; one or more spans narrowed or are missing.
    EvidenceIncomplete,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
    /// One or more item freshness states were downgraded to stale.
    FreshnessDowngradedToStale,
}

impl IncidentWorkspaceDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelayUnavailable => "relay_unavailable",
            Self::ProofStale => "proof_stale",
            Self::HostSessionInactive => "host_session_inactive",
            Self::TrustNarrowed => "trust_narrowed",
            Self::IncidentAttributionLost => "incident_attribution_lost",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
        }
    }
}

/// An incident workspace header item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceHeaderItem {
    /// Stable item id.
    pub item_id: String,
    /// Stable incident id this header identifies.
    pub incident_id: String,
    /// Incident severity.
    pub severity: IncidentSeverity,
    /// Lifecycle status.
    pub status: IncidentWorkspaceStatus,
    /// Attribution to evidence and build identity.
    pub attribution: IncidentAttributionState,
    /// Freshness of the header state.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the exact build identity. Carries no payload body.
    pub build_identity_ref: String,
    /// Ref to the originating incident evidence. Carries no payload body.
    pub evidence_ref: String,
    /// Exact desktop handoff into the incident workspace header.
    pub handoff: CompanionDesktopHandoff,
}

/// An evidence-timeline item, including first-class missing spans.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceTimelineItem {
    /// Stable item id.
    pub item_id: String,
    /// Ordering position within the timeline.
    pub sequence: u32,
    /// Kind of evidence span.
    pub span_kind: EvidenceSpanKind,
    /// Completeness state of the span.
    pub span_state: EvidenceSpanState,
    /// Attribution to evidence and build identity.
    pub attribution: IncidentAttributionState,
    /// Freshness of the span.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// True when a missing/partial gap label is shown to the user.
    pub gap_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the evidence span. Carries no payload body.
    pub evidence_ref: String,
    /// Exact desktop handoff to the evidence span.
    pub handoff: CompanionDesktopHandoff,
}

/// A read-only resource slice attributed to an incident.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceSliceItem {
    /// Stable item id.
    pub item_id: String,
    /// Kind of resource slice.
    pub slice_kind: ResourceSliceKind,
    /// Human-readable summary of the bounded window the slice captured.
    pub bounded_window_summary: String,
    /// Freshness of the slice.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the resource slice. Carries no payload body.
    pub resource_ref: String,
    /// Exact desktop handoff to the resource slice.
    pub handoff: CompanionDesktopHandoff,
}

/// A runbook packet that guides mitigation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunbookPacketItem {
    /// Stable item id.
    pub item_id: String,
    /// Number of steps in the runbook.
    pub step_count: u32,
    /// Automation class of the runbook.
    pub automation_class: RunbookAutomationClass,
    /// State of the next runbook step.
    pub next_step_state: RunbookStepState,
    /// Always true: any automated action requires explicit host approval.
    pub requires_host_approval: bool,
    /// Freshness of the runbook state.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the runbook packet. Carries no payload body.
    pub runbook_ref: String,
    /// Exact desktop handoff to the runbook.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-section qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceSectionQualification {
    /// Section the row applies to.
    pub section: IncidentWorkspaceSection,
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

/// Read/write scope and authority contract for the whole workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceScopeContract {
    /// The header section is read-only.
    pub header_read_only: bool,
    /// The evidence timeline is read-only.
    pub evidence_timeline_read_only: bool,
    /// The resource slices are read-only.
    pub resource_slice_read_only: bool,
    /// A runbook never executes an automated step without explicit host approval.
    pub runbook_read_only_unless_host_approved: bool,
    /// The workspace never holds an unbounded write authority.
    pub no_unbounded_workspace_write: bool,
    /// The desktop host stays authoritative.
    pub host_authoritative: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Attribution contract: incident packets stay attributable or narrow honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceAttributionContract {
    /// Headers are attributed or narrowed to unattributed, never falsely claimed.
    pub headers_attributed_or_narrowed: bool,
    /// Evidence spans are attributed or narrowed, never falsely claimed.
    pub evidence_spans_attributed_or_narrowed: bool,
    /// Missing or partial evidence spans are recorded as first-class facts.
    pub missing_spans_recorded_as_first_class: bool,
    /// No provenance is claimed without backing evidence.
    pub no_provenance_claimed_without_evidence: bool,
}

/// Stale-state honesty contract for the whole workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceStaleStateHonesty {
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
pub struct IncidentWorkspaceSecurityReview {
    /// The header section is read-only.
    pub header_read_only: bool,
    /// The evidence timeline is read-only.
    pub evidence_timeline_read_only: bool,
    /// The resource slices are read-only.
    pub resource_slice_read_only: bool,
    /// A runbook never executes an automated step without explicit host approval.
    pub runbook_read_only_unless_host_approved: bool,
    /// No unbounded workspace write authority is exposed.
    pub no_unbounded_workspace_write: bool,
    /// The desktop host stays authoritative.
    pub host_stays_authoritative: bool,
    /// Incident attribution is preserved or honestly narrowed.
    pub incident_attribution_preserved: bool,
    /// Missing or partial evidence is recorded rather than hidden.
    pub missing_evidence_recorded_not_hidden: bool,
    /// Stale state is labeled rather than hidden.
    pub stale_state_labeled_never_hidden: bool,
    /// Exact desktop handoff is preserved or honestly degraded.
    pub exact_desktop_handoff_preserved: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the section.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Every section discloses local, staged, and provider/admin continuity.
    pub locality_disclosed: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceConsumerProjection {
    /// Incident workspace projects the header section.
    pub incident_workspace_shows_header: bool,
    /// Incident workspace projects the evidence timeline.
    pub incident_workspace_shows_evidence_timeline: bool,
    /// Incident workspace projects the resource slices.
    pub incident_workspace_shows_resource_slices: bool,
    /// Incident workspace projects the runbook packets.
    pub incident_workspace_shows_runbook_packets: bool,
    /// Desktop panel shows the handoff targets.
    pub desktop_panel_shows_handoff_target: bool,
    /// Support export shows attribution and freshness state.
    pub support_export_shows_attribution_and_freshness: bool,
    /// Diagnostics shows missing and stale labels.
    pub diagnostics_shows_missing_and_stale_labels: bool,
    /// Preview / Labs sections are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_sections: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the section.
    pub auto_narrow_on_stale: bool,
}

/// Per-observation signal fed to
/// [`IncidentWorkspaceSurfacePacket::apply_incident_workspace_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncidentWorkspaceObservation {
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
    /// True when the evidence timeline is complete.
    pub evidence_complete: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Constructor input for [`IncidentWorkspaceSurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncidentWorkspaceSurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<IncidentWorkspaceSectionQualification>,
    /// Header items.
    pub headers: Vec<IncidentWorkspaceHeaderItem>,
    /// Evidence-timeline items.
    pub evidence_timeline: Vec<EvidenceTimelineItem>,
    /// Resource-slice items.
    pub resource_slices: Vec<ResourceSliceItem>,
    /// Runbook-packet items.
    pub runbook_packets: Vec<RunbookPacketItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: IncidentWorkspaceScopeContract,
    /// Attribution contract.
    pub attribution_contract: IncidentWorkspaceAttributionContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: IncidentWorkspaceStaleStateHonesty,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: IncidentWorkspaceSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: IncidentWorkspaceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: IncidentWorkspaceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe incident workspace surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IncidentWorkspaceSurfacePacket {
    /// Record kind; must equal [`INCIDENT_WORKSPACE_SURFACE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`INCIDENT_WORKSPACE_SURFACE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<IncidentWorkspaceSectionQualification>,
    /// Header items.
    pub headers: Vec<IncidentWorkspaceHeaderItem>,
    /// Evidence-timeline items.
    pub evidence_timeline: Vec<EvidenceTimelineItem>,
    /// Resource-slice items.
    pub resource_slices: Vec<ResourceSliceItem>,
    /// Runbook-packet items.
    pub runbook_packets: Vec<RunbookPacketItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: IncidentWorkspaceScopeContract,
    /// Attribution contract.
    pub attribution_contract: IncidentWorkspaceAttributionContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: IncidentWorkspaceStaleStateHonesty,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: IncidentWorkspaceSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: IncidentWorkspaceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: IncidentWorkspaceProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<IncidentWorkspaceDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl IncidentWorkspaceSurfacePacket {
    /// Builds an incident workspace surface packet from stable-lane input.
    pub fn new(input: IncidentWorkspaceSurfacePacketInput) -> Self {
        Self {
            record_kind: INCIDENT_WORKSPACE_SURFACE_RECORD_KIND.to_owned(),
            schema_version: INCIDENT_WORKSPACE_SURFACE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            section_qualifications: input.section_qualifications,
            headers: input.headers,
            evidence_timeline: input.evidence_timeline,
            resource_slices: input.resource_slices,
            runbook_packets: input.runbook_packets,
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
    /// Lost incident attribution marks every header and evidence span unattributed
    /// and narrows the header and evidence-timeline sections. Incomplete evidence
    /// narrows every present evidence span to partial, labels the gap, and narrows
    /// the evidence-timeline section. Narrowed trust narrows the runbook section, the
    /// only one that can carry an approved automated action. An inactive host session
    /// downgrades the resolution of every handoff that requires an active host and
    /// narrows the runbook section, since an approved action can no longer be relayed.
    /// Degraded state is labeled, never hidden.
    pub fn apply_incident_workspace_degradation(
        &mut self,
        observation: &IncidentWorkspaceObservation,
    ) {
        let mut labels: BTreeSet<IncidentWorkspaceDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let section_adverse = !observation.relay_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.relay_available {
            labels.insert(IncidentWorkspaceDegradedReason::RelayUnavailable);
            if self.force_all_freshness_stale() {
                labels.insert(IncidentWorkspaceDegradedReason::FreshnessDowngradedToStale);
            }
        }
        if !observation.proof_fresh {
            labels.insert(IncidentWorkspaceDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(IncidentWorkspaceDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.trust_intact {
            labels.insert(IncidentWorkspaceDegradedReason::TrustNarrowed);
        }
        if !observation.incident_attribution_intact {
            labels.insert(IncidentWorkspaceDegradedReason::IncidentAttributionLost);
            for item in &mut self.headers {
                item.attribution = IncidentAttributionState::Unattributed;
            }
            for item in &mut self.evidence_timeline {
                item.attribution = IncidentAttributionState::Unattributed;
            }
        }
        if !observation.evidence_complete {
            labels.insert(IncidentWorkspaceDegradedReason::EvidenceIncomplete);
            for item in &mut self.evidence_timeline {
                if item.span_state != item.span_state.narrowed() {
                    item.span_state = item.span_state.narrowed();
                }
                if item.span_state.is_gap() {
                    item.gap_label_shown = true;
                }
            }
        }

        for row in &mut self.section_qualifications {
            let adverse = section_adverse
                || (!observation.trust_intact
                    && row.section == IncidentWorkspaceSection::RunbookPacket)
                || (!observation.host_session_active
                    && row.section == IncidentWorkspaceSection::RunbookPacket)
                || (!observation.incident_attribution_intact
                    && matches!(
                        row.section,
                        IncidentWorkspaceSection::Header
                            | IncidentWorkspaceSection::EvidenceTimeline
                    ))
                || (!observation.evidence_complete
                    && row.section == IncidentWorkspaceSection::EvidenceTimeline);
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(IncidentWorkspaceDegradedReason::HostSessionInactive);
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
                labels.insert(IncidentWorkspaceDegradedReason::HandoffTargetUnresolved);
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
        self.headers
            .iter_mut()
            .map(|item| (&mut item.freshness, &mut item.stale_label_shown))
            .chain(
                self.evidence_timeline
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.resource_slices
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.runbook_packets
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
    }

    /// Validates the incident workspace surface invariants.
    pub fn validate(&self) -> Vec<IncidentWorkspaceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != INCIDENT_WORKSPACE_SURFACE_RECORD_KIND {
            violations.push(IncidentWorkspaceViolation::WrongRecordKind);
        }
        if self.schema_version != INCIDENT_WORKSPACE_SURFACE_SCHEMA_VERSION {
            violations.push(IncidentWorkspaceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(IncidentWorkspaceViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(IncidentWorkspaceViolation::ProjectedSurfacesMissing);
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
            &serde_json::to_value(self).expect("incident workspace packet serializes"),
        ) {
            violations.push(IncidentWorkspaceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("incident workspace packet serializes")
    }

    /// Sections currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_sections(
        &self,
    ) -> impl Iterator<Item = &IncidentWorkspaceSectionQualification> {
        self.section_qualifications.iter().filter(|row| {
            matches!(
                row.qualification,
                M5CompanionQualificationClass::Stable
                    | M5CompanionQualificationClass::Beta
                    | M5CompanionQualificationClass::Preview
            ) && row.rollout_stage != M5CompanionRolloutStage::Withheld
        })
    }

    /// True when every item's handoff resolves to the exact desktop location.
    pub fn all_handoffs_exact(&self) -> bool {
        self.handoffs()
            .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact)
    }

    /// True when every stale or unknown-freshness item carries a visible label.
    pub fn stale_state_honestly_labeled(&self) -> bool {
        self.headers
            .iter()
            .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .evidence_timeline
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .resource_slices
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .runbook_packets
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
    }

    /// True when every missing or partial evidence span carries a visible gap label.
    pub fn evidence_gaps_honestly_labeled(&self) -> bool {
        self.evidence_timeline
            .iter()
            .all(|item| !item.span_state.is_gap() || item.gap_label_shown)
    }

    /// Iterates every handoff across all four sections, in section order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.headers
            .iter()
            .map(|item| &item.handoff)
            .chain(self.evidence_timeline.iter().map(|item| &item.handoff))
            .chain(self.resource_slices.iter().map(|item| &item.handoff))
            .chain(self.runbook_packets.iter().map(|item| &item.handoff))
    }

    fn handoffs_mut(&mut self) -> impl Iterator<Item = &mut CompanionDesktopHandoff> {
        self.headers
            .iter_mut()
            .map(|item| &mut item.handoff)
            .chain(
                self.evidence_timeline
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(
                self.resource_slices
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(
                self.runbook_packets
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Incident Workspace Headers, Evidence Timelines, Resource Slices, and Runbook Packets\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Sections: {} | Headers: {} | Evidence spans: {} | Resource slices: {} | Runbook packets: {}\n",
            self.section_qualifications.len(),
            self.headers.len(),
            self.evidence_timeline.len(),
            self.resource_slices.len(),
            self.runbook_packets.len(),
        ));
        out.push_str(&format!(
            "- Exact handoff for every item: {}\n",
            if self.all_handoffs_exact() {
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
            "- Evidence gaps honestly labeled: {}\n",
            if self.evidence_gaps_honestly_labeled() {
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

        out.push_str("\n## Headers\n\n");
        for item in &self.headers {
            out.push_str(&format!(
                "- `{}` [{}/{}] {} — {} ({}) → `{}` ({})\n",
                item.item_id,
                item.severity.as_str(),
                item.attribution.as_str(),
                item.status.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Evidence timeline\n\n");
        for item in &self.evidence_timeline {
            out.push_str(&format!(
                "- `{}` #{} [{}/{}] {} — {} ({}) → `{}` ({})\n",
                item.item_id,
                item.sequence,
                item.span_kind.as_str(),
                item.span_state.as_str(),
                item.attribution.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Resource slices\n\n");
        for item in &self.resource_slices {
            out.push_str(&format!(
                "- `{}` [{}] {} ({}) → `{}` ({})\n",
                item.item_id,
                item.slice_kind.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Runbook packets\n\n");
        for item in &self.runbook_packets {
            out.push_str(&format!(
                "- `{}` [{}] {} steps, next `{}` — {} ({}) → `{}` ({})\n",
                item.item_id,
                item.automation_class.as_str(),
                item.step_count,
                item.next_step_state.as_str(),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out
    }
}

/// Errors emitted when reading the checked-in incident workspace export.
#[derive(Debug)]
pub enum IncidentWorkspaceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<IncidentWorkspaceViolation>),
}

impl fmt::Display for IncidentWorkspaceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "incident workspace export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "incident workspace export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for IncidentWorkspaceArtifactError {}

/// Validation failures emitted by [`IncidentWorkspaceSurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncidentWorkspaceViolation {
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
    /// A runbook item that can carry automation does not require host approval.
    RunbookAutomationNotApproved,
    /// An item is missing identity or a redacted body, or has a payload-like body.
    ItemIncomplete,
    /// A stale or unknown-freshness item is not labeled.
    StaleStateNotLabeled,
    /// A missing or partial evidence span is not labeled as a gap.
    EvidenceGapNotLabeled,
    /// An item's handoff is missing its deep-link ref.
    HandoffRefMissing,
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

impl IncidentWorkspaceViolation {
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
            Self::RunbookAutomationNotApproved => "runbook_automation_not_approved",
            Self::ItemIncomplete => "item_incomplete",
            Self::StaleStateNotLabeled => "stale_state_not_labeled",
            Self::EvidenceGapNotLabeled => "evidence_gap_not_labeled",
            Self::HandoffRefMissing => "handoff_ref_missing",
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

/// Reads and validates the checked-in stable incident workspace surface export.
///
/// This is the canonical reader: the incident workspace, the desktop companion
/// panel, diagnostics, support-export, or Help/About surface calls it to ingest
/// the packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`IncidentWorkspaceArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_stable_incident_workspace_surface_export(
) -> Result<IncidentWorkspaceSurfacePacket, IncidentWorkspaceArtifactError> {
    let packet: IncidentWorkspaceSurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/add_incident_workspace_headers_evidence_timelines_resource_slices_and_runbook_packets/support_export.json"
    )))
    .map_err(IncidentWorkspaceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(IncidentWorkspaceArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every workspace export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        INCIDENT_WORKSPACE_SURFACE_SCHEMA_REF.to_owned(),
        INCIDENT_WORKSPACE_SURFACE_DOC_REF.to_owned(),
        M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_QUALIFICATION_REF.to_owned(),
        M5_COMPANION_BOUNDARY_MANIFEST_REF.to_owned(),
        M5_INCIDENT_WORKSPACE_CONTRACT_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical read/write scope and authority contract with every guarantee met.
pub fn canonical_scope_contract() -> IncidentWorkspaceScopeContract {
    IncidentWorkspaceScopeContract {
        header_read_only: true,
        evidence_timeline_read_only: true,
        resource_slice_read_only: true,
        runbook_read_only_unless_host_approved: true,
        no_unbounded_workspace_write: true,
        host_authoritative: true,
        no_payload_bodies: true,
    }
}

/// Canonical attribution contract with every guarantee satisfied.
pub fn canonical_attribution_contract() -> IncidentWorkspaceAttributionContract {
    IncidentWorkspaceAttributionContract {
        headers_attributed_or_narrowed: true,
        evidence_spans_attributed_or_narrowed: true,
        missing_spans_recorded_as_first_class: true,
        no_provenance_claimed_without_evidence: true,
    }
}

/// Canonical stale-state honesty contract with every guarantee satisfied.
pub fn canonical_stale_state_honesty() -> IncidentWorkspaceStaleStateHonesty {
    IncidentWorkspaceStaleStateHonesty {
        stale_items_labeled: true,
        unknown_freshness_labeled: true,
        never_show_stale_as_live: true,
        freshness_floor_enforced: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> IncidentWorkspaceSecurityReview {
    IncidentWorkspaceSecurityReview {
        header_read_only: true,
        evidence_timeline_read_only: true,
        resource_slice_read_only: true,
        runbook_read_only_unless_host_approved: true,
        no_unbounded_workspace_write: true,
        host_stays_authoritative: true,
        incident_attribution_preserved: true,
        missing_evidence_recorded_not_hidden: true,
        stale_state_labeled_never_hidden: true,
        exact_desktop_handoff_preserved: true,
        no_payload_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every section projecting truth.
pub fn canonical_consumer_projection() -> IncidentWorkspaceConsumerProjection {
    IncidentWorkspaceConsumerProjection {
        incident_workspace_shows_header: true,
        incident_workspace_shows_evidence_timeline: true,
        incident_workspace_shows_resource_slices: true,
        incident_workspace_shows_runbook_packets: true,
        desktop_panel_shows_handoff_target: true,
        support_export_shows_attribution_and_freshness: true,
        diagnostics_shows_missing_and_stale_labels: true,
        preview_labs_label_for_unqualified_sections: true,
    }
}

/// Canonical per-section qualification rows, inherited from the frozen matrix.
pub fn canonical_section_qualifications() -> Vec<IncidentWorkspaceSectionQualification> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;

    let lane_ref = IncidentWorkspaceSection::Header.matrix_lane().as_str();
    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        IncidentWorkspaceSectionQualification {
            section: IncidentWorkspaceSection::Header,
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            read_write_scope: scope,
            matrix_lane_ref: lane_ref.to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::IncidentAttributionMissing,
                Trigger::PolicyBlocked,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
        },
        IncidentWorkspaceSectionQualification {
            section: IncidentWorkspaceSection::EvidenceTimeline,
            qualification: Qual::Stable,
            rollout_stage: Stage::GeneralAvailability,
            read_write_scope: scope,
            matrix_lane_ref: lane_ref.to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::IncidentAttributionMissing,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
        },
        IncidentWorkspaceSectionQualification {
            section: IncidentWorkspaceSection::ResourceSlice,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: lane_ref.to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::TrustNarrowing,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
        },
        IncidentWorkspaceSectionQualification {
            section: IncidentWorkspaceSection::RunbookPacket,
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

/// Canonical locality disclosure for the incident workspace surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "Incident headers, evidence spans, resource slices, and runbook packets are owned by the local core and stay inspectable offline."
                .to_owned(),
        staged:
            "Resource-slice capture and runbook automation roll out per cohort and capability gate."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Exact handoff into a live host, and relaying a host-approved runbook action, require the companion relay and an active host session; the local core never depends on them to function."
                .to_owned(),
    }
}

fn handoff(
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

/// Canonical incident workspace header items.
pub fn canonical_headers() -> Vec<IncidentWorkspaceHeaderItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use IncidentAttributionState as Attribution;
    use IncidentSeverity as Severity;
    use IncidentWorkspaceStatus as Status;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        IncidentWorkspaceHeaderItem {
            item_id: "header:incident:0001".to_owned(),
            incident_id: "incident:0001".to_owned(),
            severity: Severity::Critical,
            status: Status::Investigating,
            attribution: Attribution::Attributed,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Critical incident raised from a crash trail, under investigation".to_owned(),
            build_identity_ref: "build:identity:0001".to_owned(),
            evidence_ref: "evidence:incident:0001".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:incident-workspace:header-0001",
                false,
            ),
        },
        IncidentWorkspaceHeaderItem {
            item_id: "header:incident:0002".to_owned(),
            incident_id: "incident:0002".to_owned(),
            severity: Severity::High,
            status: Status::Mitigating,
            attribution: Attribution::Attributed,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "High-severity incident under mitigation on the host".to_owned(),
            build_identity_ref: "build:identity:0002".to_owned(),
            evidence_ref: "evidence:incident:0002".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:incident-workspace:header-0002",
                false,
            ),
        },
    ]
}

/// Canonical evidence-timeline items, including a first-class missing span.
pub fn canonical_evidence_timeline() -> Vec<EvidenceTimelineItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use EvidenceSpanKind as Kind;
    use EvidenceSpanState as SpanState;
    use IncidentAttributionState as Attribution;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        EvidenceTimelineItem {
            item_id: "evidence:0001".to_owned(),
            sequence: 1,
            span_kind: Kind::CrashTrail,
            span_state: SpanState::Present,
            attribution: Attribution::Attributed,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            gap_label_shown: false,
            summary: "Crash trail captured at the moment of failure".to_owned(),
            evidence_ref: "evidence:span:crash-0001".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:evidence:crash-0001",
                false,
            ),
        },
        EvidenceTimelineItem {
            item_id: "evidence:0002".to_owned(),
            sequence: 2,
            span_kind: Kind::LogWindow,
            span_state: SpanState::Present,
            attribution: Attribution::Attributed,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            gap_label_shown: false,
            summary: "Log window around the failure".to_owned(),
            evidence_ref: "evidence:span:log-0002".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:evidence:log-0002",
                false,
            ),
        },
        EvidenceTimelineItem {
            item_id: "evidence:0003".to_owned(),
            sequence: 3,
            span_kind: Kind::MetricSeries,
            span_state: SpanState::Missing,
            attribution: Attribution::PartiallyAttributed,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            gap_label_shown: true,
            summary: "Metric series for the window is missing and recorded as a gap".to_owned(),
            evidence_ref: "evidence:span:metric-0003".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:evidence:metric-0003",
                false,
            ),
        },
    ]
}

/// Canonical read-only resource-slice items.
pub fn canonical_resource_slices() -> Vec<ResourceSliceItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use ResourceSliceKind as Kind;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        ResourceSliceItem {
            item_id: "slice:0001".to_owned(),
            slice_kind: Kind::CpuProfile,
            bounded_window_summary: "30-second CPU profile around the incident window".to_owned(),
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "CPU profile slice attributed to the incident".to_owned(),
            resource_ref: "resource:cpu:0001".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:resource:cpu-0001",
                false,
            ),
        },
        ResourceSliceItem {
            item_id: "slice:0002".to_owned(),
            slice_kind: Kind::MemorySnapshot,
            bounded_window_summary: "Single memory snapshot at the failure point".to_owned(),
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Memory snapshot slice attributed to the incident".to_owned(),
            resource_ref: "resource:memory:0002".to_owned(),
            handoff: handoff(
                Target::IncidentWorkspace,
                "handoff:resource:memory-0002",
                false,
            ),
        },
    ]
}

/// Canonical runbook-packet items.
pub fn canonical_runbook_packets() -> Vec<RunbookPacketItem> {
    use CompanionFreshnessState as Fresh;
    use CompanionHandoffTarget as Target;
    use RunbookAutomationClass as Automation;
    use RunbookStepState as StepState;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        RunbookPacketItem {
            item_id: "runbook:0001".to_owned(),
            step_count: 5,
            automation_class: Automation::Manual,
            next_step_state: StepState::InProgress,
            requires_host_approval: true,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Manual mitigation runbook for the crash incident".to_owned(),
            runbook_ref: "runbook:packet:0001".to_owned(),
            handoff: handoff(Target::IncidentWorkspace, "handoff:runbook:0001", false),
        },
        RunbookPacketItem {
            item_id: "runbook:0002".to_owned(),
            step_count: 3,
            automation_class: Automation::AutomatedWithApproval,
            next_step_state: StepState::Pending,
            requires_host_approval: true,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Automated rollback runbook; each action requires host approval".to_owned(),
            runbook_ref: "runbook:packet:0002".to_owned(),
            handoff: handoff(Target::IncidentWorkspace, "handoff:runbook:0002", true),
        },
    ]
}

/// Builds the canonical incident workspace surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export
/// and Markdown summary are generated from, so the artifact never drifts from the
/// typed section, item, scope, attribution, and freshness definitions.
pub fn canonical_incident_workspace_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: IncidentWorkspaceProofFreshness,
) -> IncidentWorkspaceSurfacePacket {
    IncidentWorkspaceSurfacePacket::new(IncidentWorkspaceSurfacePacketInput {
        packet_id,
        surface_label,
        projected_surfaces: vec![
            M5CompanionConsumerSurface::IncidentWorkspace,
            M5CompanionConsumerSurface::DesktopCompanionPanel,
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::Diagnostics,
            M5CompanionConsumerSurface::HelpAbout,
        ],
        section_qualifications: canonical_section_qualifications(),
        headers: canonical_headers(),
        evidence_timeline: canonical_evidence_timeline(),
        resource_slices: canonical_resource_slices(),
        runbook_packets: canonical_runbook_packets(),
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
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        INCIDENT_WORKSPACE_SURFACE_SCHEMA_REF,
        INCIDENT_WORKSPACE_SURFACE_DOC_REF,
        M5_COMPANION_SURFACE_CONTRACT_REF,
        M5_INCIDENT_WORKSPACE_CONTRACT_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(IncidentWorkspaceViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_section_qualifications(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    let present: BTreeSet<IncidentWorkspaceSection> = packet
        .section_qualifications
        .iter()
        .map(|row| row.section)
        .collect();
    for required in IncidentWorkspaceSection::ALL {
        if !present.contains(&required) {
            violations.push(IncidentWorkspaceViolation::RequiredSectionMissing);
            return;
        }
    }

    for row in &packet.section_qualifications {
        if row.matrix_lane_ref != row.section.matrix_lane().as_str() {
            violations.push(IncidentWorkspaceViolation::SectionLaneMismatch);
        }
        if row.read_write_scope != row.section.bounded_scope() {
            violations.push(IncidentWorkspaceViolation::SectionScopeMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(IncidentWorkspaceViolation::SectionRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    if packet.headers.is_empty()
        || packet.evidence_timeline.is_empty()
        || packet.resource_slices.is_empty()
        || packet.runbook_packets.is_empty()
    {
        violations.push(IncidentWorkspaceViolation::SectionContentMissing);
    }

    for item in &packet.headers {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(IncidentWorkspaceViolation::ReadOnlyScopeViolated);
        }
        if item.item_id.trim().is_empty()
            || item.incident_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.build_identity_ref.trim().is_empty()
            || item.evidence_ref.trim().is_empty()
        {
            violations.push(IncidentWorkspaceViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.evidence_timeline {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(IncidentWorkspaceViolation::ReadOnlyScopeViolated);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.evidence_ref.trim().is_empty()
        {
            violations.push(IncidentWorkspaceViolation::ItemIncomplete);
        }
        if item.span_state.is_gap() && !item.gap_label_shown {
            violations.push(IncidentWorkspaceViolation::EvidenceGapNotLabeled);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.resource_slices {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(IncidentWorkspaceViolation::ReadOnlyScopeViolated);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.bounded_window_summary.trim().is_empty()
            || item.resource_ref.trim().is_empty()
        {
            violations.push(IncidentWorkspaceViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.runbook_packets {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(IncidentWorkspaceViolation::ReadOnlyScopeViolated);
        }
        if !item.requires_host_approval {
            violations.push(IncidentWorkspaceViolation::RunbookAutomationNotApproved);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.runbook_ref.trim().is_empty()
        {
            violations.push(IncidentWorkspaceViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }
}

fn validate_freshness_label(
    freshness: CompanionFreshnessState,
    stale_label_shown: bool,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    if freshness.requires_label() && !stale_label_shown {
        violations.push(IncidentWorkspaceViolation::StaleStateNotLabeled);
    }
}

fn validate_handoff(
    handoff: &CompanionDesktopHandoff,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(IncidentWorkspaceViolation::HandoffRefMissing);
    }
}

fn validate_scope_contract(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    let contract = &packet.scope_contract;
    for ok in [
        contract.header_read_only,
        contract.evidence_timeline_read_only,
        contract.resource_slice_read_only,
        contract.runbook_read_only_unless_host_approved,
        contract.no_unbounded_workspace_write,
        contract.host_authoritative,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(IncidentWorkspaceViolation::ScopeContractIncomplete);
            return;
        }
    }
}

fn validate_attribution_contract(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    let contract = &packet.attribution_contract;
    for ok in [
        contract.headers_attributed_or_narrowed,
        contract.evidence_spans_attributed_or_narrowed,
        contract.missing_spans_recorded_as_first_class,
        contract.no_provenance_claimed_without_evidence,
    ] {
        if !ok {
            violations.push(IncidentWorkspaceViolation::AttributionContractIncomplete);
            return;
        }
    }
}

fn validate_stale_state_honesty(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    let honesty = &packet.stale_state_honesty;
    for ok in [
        honesty.stale_items_labeled,
        honesty.unknown_freshness_labeled,
        honesty.never_show_stale_as_live,
        honesty.freshness_floor_enforced,
    ] {
        if !ok {
            violations.push(IncidentWorkspaceViolation::StaleStateHonestyIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(IncidentWorkspaceViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.header_read_only,
        review.evidence_timeline_read_only,
        review.resource_slice_read_only,
        review.runbook_read_only_unless_host_approved,
        review.no_unbounded_workspace_write,
        review.host_stays_authoritative,
        review.incident_attribution_preserved,
        review.missing_evidence_recorded_not_hidden,
        review.stale_state_labeled_never_hidden,
        review.exact_desktop_handoff_preserved,
        review.no_payload_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(IncidentWorkspaceViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.incident_workspace_shows_header,
        projection.incident_workspace_shows_evidence_timeline,
        projection.incident_workspace_shows_resource_slices,
        projection.incident_workspace_shows_runbook_packets,
        projection.desktop_panel_shows_handoff_target,
        projection.support_export_shows_attribution_and_freshness,
        projection.diagnostics_shows_missing_and_stale_labels,
        projection.preview_labs_label_for_unqualified_sections,
    ] {
        if !ok {
            violations.push(IncidentWorkspaceViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &IncidentWorkspaceSurfacePacket,
    violations: &mut Vec<IncidentWorkspaceViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(IncidentWorkspaceViolation::ProofFreshnessIncomplete);
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
