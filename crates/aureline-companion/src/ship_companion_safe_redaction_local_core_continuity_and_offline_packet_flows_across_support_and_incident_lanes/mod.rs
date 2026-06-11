//! Companion-safe redaction, local-core continuity, and offline packet flows across the
//! support and incident lanes, projected as a downgrade-aware truth packet.
//!
//! This module owns the export-safe truth packet that ties the companion, incident, and
//! support lanes together around three guarantees: every record that crosses a companion,
//! support, or incident boundary is **redaction-safe**, the **local core stays
//! authoritative and continues** when a provider degrades, and the support/incident
//! packets that flow out **assemble and replay offline** from the local core. It projects
//! four sections: the **redaction policy** rows that record, per boundary and content
//! class, the redaction class applied before content crosses to a companion, support, or
//! incident surface, and whether that redaction is proven or honestly labeled; the
//! **local-core continuity** rows that record, per capability, whether the capability
//! continues from the authoritative local core when a provider degrades and what (if
//! anything) requires provider or admin continuity; the **offline incident packet** rows
//! that record the incident packets that assemble and replay offline, each attributable
//! and redacted; and the **offline support packet** rows that record the support-export
//! packets that assemble and replay offline, each redacted and local-first. The redaction
//! section binds to the frozen M5 companion-matrix
//! [`M5CompanionMatrixLane::CompanionNotification`] lane, the incident section to
//! [`M5CompanionMatrixLane::IncidentWorkspace`], and the continuity and support sections to
//! [`M5CompanionMatrixLane::OffboardingContinuity`]. Every item carries an exact
//! [`CompanionDesktopHandoff`] (reused from
//! [`crate::companion_notification_triage_review_queues_and_ci_status_cards_with_desktop_handoff`])
//! so opening an item always resumes the precise host context locally.
//!
//! Three invariants make this surface safe to ship. First, **companion-safe redaction**:
//! every redaction-policy row, incident packet, and support packet asserts that no payload
//! body crosses the boundary, and a redaction claim is shown as proven only when it is
//! backed by evidence — an unverifiable claim narrows the redaction to a more conservative
//! class ([`RedactionClass::narrowed_when_proof_lost`]) and is labeled rather than shown as
//! proven. Second, **local-core continuity**: every local-core-authoritative capability
//! stays available offline, `local_work_preserved` never goes false, and a degraded
//! provider is labeled rather than allowed to look like a hard dependency. Third, **offline
//! packet flows**: every incident and support lane always offers at least one local-first
//! packet path so a degraded provider never strands the support or incident workflow, an
//! incident packet stays attributable or is honestly labeled, and a completeness claim is
//! proven where claimed or labeled where not.
//!
//! [`RedactionContinuitySurfacePacket::apply_redaction_degradation`] narrows sections,
//! narrows redaction to its conservative class, narrows packet availability to its local
//! path, downgrades completeness claims, and downgrades freshness from a per-observation
//! signal — when redaction proof is unavailable, the offline packet assembler is
//! unavailable, proof is stale, completeness is unverified, incident attribution is
//! unavailable, the managed service is degraded, the host session is inactive, or an
//! upstream matrix lane narrowed — so CI or release tooling degrades the surface honestly
//! rather than show a redaction it can no longer prove, a packet it can no longer assemble,
//! or a completeness claim it can no longer verify. Degraded state is labeled, never hidden,
//! the local path always remains, and local work is always preserved.
//!
//! [`canonical_redaction_continuity_surface`] builds the surface and
//! [`current_stable_redaction_continuity_surface_export`] reads and validates the
//! checked-in support export, so the desktop companion panel, the CLI/headless surface,
//! diagnostics, support exports, and Help/About ingest the packet rather than cloning
//! status text. Credential bodies, raw provider payloads, raw incident evidence bodies, and
//! raw support-bundle contents stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/companion/ship-companion-safe-redaction-local-core-continuity-and-offline-packet-flows-across-support-and-incident-lanes.schema.json`](../../../../schemas/companion/ship-companion-safe-redaction-local-core-continuity-and-offline-packet-flows-across-support-and-incident-lanes.schema.json).
//! The contract doc is
//! [`docs/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes.md`](../../../../docs/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes.md).
//! The protected fixture directory is
//! [`fixtures/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes/`](../../../../fixtures/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes/).

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
    M5CompanionRolloutStage, M5_COMPANION_MATRIX_SCHEMA_REF, M5_COMPANION_SURFACE_CONTRACT_REF,
    M5_INCIDENT_WORKSPACE_CONTRACT_REF, M5_OFFBOARDING_CONTRACT_REF,
};
use crate::ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty::{
    CompanionFreshnessState, CompanionReadWriteScope,
};

/// Stable record-kind tag carried by [`RedactionContinuitySurfacePacket`].
pub const REDACTION_CONTINUITY_RECORD_KIND: &str =
    "ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes";

/// Schema version for redaction/continuity/offline-packet surface records.
pub const REDACTION_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const REDACTION_CONTINUITY_SCHEMA_REF: &str =
    "schemas/companion/ship-companion-safe-redaction-local-core-continuity-and-offline-packet-flows-across-support-and-incident-lanes.schema.json";

/// Repo-relative path of the surface contract doc.
pub const REDACTION_CONTINUITY_DOC_REF: &str =
    "docs/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes.md";

/// Repo-relative path of the protected fixture directory.
pub const REDACTION_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes";

/// Repo-relative path of the checked support-export artifact.
pub const REDACTION_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const REDACTION_CONTINUITY_SUMMARY_REF: &str =
    "artifacts/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes.md";

/// One of the four redaction/continuity/offline-packet sections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionContinuitySection {
    /// The companion-safe redaction policy rows.
    RedactionPolicy,
    /// The local-core continuity rows.
    LocalCoreContinuity,
    /// The offline incident packet rows.
    OfflineIncidentPacket,
    /// The offline support packet rows.
    OfflineSupportPacket,
}

impl RedactionContinuitySection {
    /// Every section, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RedactionPolicy,
        Self::LocalCoreContinuity,
        Self::OfflineIncidentPacket,
        Self::OfflineSupportPacket,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactionPolicy => "redaction_policy",
            Self::LocalCoreContinuity => "local_core_continuity",
            Self::OfflineIncidentPacket => "offline_incident_packet",
            Self::OfflineSupportPacket => "offline_support_packet",
        }
    }

    /// Frozen M5 companion-matrix lane this section inherits qualification from.
    ///
    /// The redaction-policy section binds to the
    /// [`M5CompanionMatrixLane::CompanionNotification`] lane (keeping companions narrow and
    /// redaction-safe), the incident-packet section to
    /// [`M5CompanionMatrixLane::IncidentWorkspace`] (keeping incident packets attributable),
    /// and the continuity and support-packet sections to
    /// [`M5CompanionMatrixLane::OffboardingContinuity`] (keeping the local core authoritative
    /// and never stranding local work).
    pub const fn matrix_lane(self) -> M5CompanionMatrixLane {
        match self {
            Self::RedactionPolicy => M5CompanionMatrixLane::CompanionNotification,
            Self::OfflineIncidentPacket => M5CompanionMatrixLane::IncidentWorkspace,
            Self::LocalCoreContinuity | Self::OfflineSupportPacket => {
                M5CompanionMatrixLane::OffboardingContinuity
            }
        }
    }

    /// Read/write scope this section is bounded to.
    ///
    /// Every section is read-only: the surface projects redaction posture, continuity, and
    /// offline-packet availability but never applies them. Redaction is enforced and a
    /// packet is assembled by the local core, never authored from this surface.
    pub const fn bounded_scope(self) -> CompanionReadWriteScope {
        CompanionReadWriteScope::ReadOnly
    }
}

/// Boundary a redaction-policy row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionBoundary {
    /// The browser/mobile companion boundary.
    Companion,
    /// The support-export boundary.
    Support,
    /// The incident-workspace boundary.
    Incident,
}

impl RedactionBoundary {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Companion => "companion",
            Self::Support => "support",
            Self::Incident => "incident",
        }
    }
}

/// Class of content a redaction-policy row governs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionContentClass {
    /// Notification bodies surfaced to a companion.
    NotificationBody,
    /// Review-queue content surfaced to a companion.
    ReviewContent,
    /// Incident evidence crossing to support or a companion.
    IncidentEvidence,
    /// Support diagnostics bundles.
    SupportDiagnostics,
    /// Session transcripts followed by a companion.
    SessionTranscript,
    /// Usage and activity metrics.
    UsageMetrics,
}

impl RedactionContentClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationBody => "notification_body",
            Self::ReviewContent => "review_content",
            Self::IncidentEvidence => "incident_evidence",
            Self::SupportDiagnostics => "support_diagnostics",
            Self::SessionTranscript => "session_transcript",
            Self::UsageMetrics => "usage_metrics",
        }
    }
}

/// Redaction class applied before content crosses a companion, support, or incident
/// boundary.
///
/// Every class is body-free by construction: no raw payload body ever crosses the boundary.
/// The classes differ only in how much redacted metadata accompanies the record. From least
/// to most conservative: [`Self::RedactedSummary`] (a redacted summary string crosses),
/// [`Self::MetadataOnly`] (only structural metadata crosses), [`Self::ReferenceOnly`] (only
/// an opaque record ref crosses), and [`Self::Withheld`] (nothing crosses). When a redaction
/// claim can no longer be proven, the class narrows toward the conservative end.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// A redacted summary string crosses; the raw body never does.
    RedactedSummary,
    /// Only structural metadata crosses.
    MetadataOnly,
    /// Only an opaque, resolvable record ref crosses.
    ReferenceOnly,
    /// Nothing crosses the boundary.
    Withheld,
}

impl RedactionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactedSummary => "redacted_summary",
            Self::MetadataOnly => "metadata_only",
            Self::ReferenceOnly => "reference_only",
            Self::Withheld => "withheld",
        }
    }

    /// Narrows a redacted-summary class to [`Self::ReferenceOnly`]; more conservative
    /// classes are kept.
    ///
    /// Used when a redaction claim can no longer be proven: the only class that carries a
    /// redacted summary string narrows to crossing an opaque ref only, so an unprovable
    /// redaction never leaves a summary that was not verified, while metadata-only,
    /// reference-only, and withheld classes are already conservative enough.
    pub const fn narrowed_when_proof_lost(self) -> Self {
        match self {
            Self::RedactedSummary => Self::ReferenceOnly,
            Self::MetadataOnly | Self::ReferenceOnly | Self::Withheld => self,
        }
    }
}

/// Availability of an offline support or incident packet.
///
/// A packet is either ready locally now, staging locally, available only via provider
/// assembly, or unavailable. A local path ([`Self::LocalReady`], [`Self::LocalStaging`]) is
/// always offered so a degraded provider never strands the support or incident workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflinePacketAvailability {
    /// The packet is ready locally right now.
    LocalReady,
    /// The packet is assembling locally from the local core.
    LocalStaging,
    /// The packet requires provider assembly.
    RequiresProviderAssembly,
    /// The packet is unavailable in this tier or while the provider is down.
    Unavailable,
}

impl OfflinePacketAvailability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReady => "local_ready",
            Self::LocalStaging => "local_staging",
            Self::RequiresProviderAssembly => "requires_provider_assembly",
            Self::Unavailable => "unavailable",
        }
    }

    /// True when this packet can be produced entirely from the local core.
    pub const fn is_local_path(self) -> bool {
        matches!(self, Self::LocalReady | Self::LocalStaging)
    }

    /// Narrows a provider-assembled packet to [`Self::Unavailable`]; a local-path or
    /// already-unavailable packet is kept.
    pub const fn narrowed_when_provider_lost(self) -> Self {
        match self {
            Self::RequiresProviderAssembly => Self::Unavailable,
            Self::LocalReady | Self::LocalStaging | Self::Unavailable => self,
        }
    }
}

/// Completeness of an offline support or incident packet.
///
/// A packet claims completeness only when it is backed by evidence; an unverifiable claim
/// narrows to [`Self::CompleteUnverified`] and is labeled rather than shown as proven, and a
/// known-partial packet states so honestly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PacketCompleteness {
    /// Complete, and the completeness is verified by evidence.
    CompleteVerified,
    /// Claimed complete but the completeness could not be verified.
    CompleteUnverified,
    /// Known to be partial; stated honestly.
    Partial,
}

impl PacketCompleteness {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompleteVerified => "complete_verified",
            Self::CompleteUnverified => "complete_unverified",
            Self::Partial => "partial",
        }
    }

    /// True when this is a verified completeness claim.
    pub const fn is_complete_claim(self) -> bool {
        matches!(self, Self::CompleteVerified)
    }

    /// Downgrades a verified completeness claim to [`Self::CompleteUnverified`]; other
    /// states are kept.
    pub const fn downgraded_to_unverified(self) -> Self {
        match self {
            Self::CompleteVerified => Self::CompleteUnverified,
            Self::CompleteUnverified | Self::Partial => self,
        }
    }
}

/// A local-core capability whose continuity posture the surface records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityCapability {
    /// Editing the local workspace.
    LocalEditing,
    /// Searching the local workspace and history.
    LocalSearch,
    /// Reviewing an incident from local evidence.
    IncidentReview,
    /// Assembling a support-export bundle.
    SupportExportAssembly,
    /// Enforcing companion-safe redaction.
    RedactionEnforcement,
    /// Replaying an offline packet.
    OfflinePacketReplay,
}

impl ContinuityCapability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalEditing => "local_editing",
            Self::LocalSearch => "local_search",
            Self::IncidentReview => "incident_review",
            Self::SupportExportAssembly => "support_export_assembly",
            Self::RedactionEnforcement => "redaction_enforcement",
            Self::OfflinePacketReplay => "offline_packet_replay",
        }
    }
}

/// Continuity posture of a local-core capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityPosture {
    /// The local core is authoritative and the capability works fully offline.
    LocalCoreAuthoritative,
    /// The local core continues the capability in a degraded but usable mode offline.
    LocalCoreContinuesDegraded,
    /// The capability requires provider continuity (and is labeled when the provider degrades).
    RequiresProviderContinuity,
    /// The capability requires admin continuity (and is labeled when admin continuity is lost).
    RequiresAdminContinuity,
}

impl ContinuityPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCoreAuthoritative => "local_core_authoritative",
            Self::LocalCoreContinuesDegraded => "local_core_continues_degraded",
            Self::RequiresProviderContinuity => "requires_provider_continuity",
            Self::RequiresAdminContinuity => "requires_admin_continuity",
        }
    }

    /// True when this capability continues from the local core offline.
    pub const fn continues_offline(self) -> bool {
        matches!(
            self,
            Self::LocalCoreAuthoritative | Self::LocalCoreContinuesDegraded
        )
    }

    /// True when this capability requires provider continuity.
    pub const fn requires_provider(self) -> bool {
        matches!(self, Self::RequiresProviderContinuity)
    }

    /// True when this capability requires admin continuity.
    pub const fn requires_admin(self) -> bool {
        matches!(self, Self::RequiresAdminContinuity)
    }
}

/// Class of an offline incident packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentPacketClass {
    /// An ordered incident evidence timeline.
    EvidenceTimeline,
    /// A runbook execution record.
    RunbookExecution,
    /// A read-only resource slice.
    ResourceSlice,
    /// A full incident export bundle.
    IncidentExportBundle,
}

impl IncidentPacketClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceTimeline => "evidence_timeline",
            Self::RunbookExecution => "runbook_execution",
            Self::ResourceSlice => "resource_slice",
            Self::IncidentExportBundle => "incident_export_bundle",
        }
    }
}

/// Class of an offline support packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportPacketClass {
    /// A diagnostics bundle.
    DiagnosticsBundle,
    /// A configuration snapshot.
    ConfigSnapshot,
    /// A proof-packet export.
    ProofPacketExport,
    /// A session diagnostics export.
    SessionDiagnostics,
}

impl SupportPacketClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticsBundle => "diagnostics_bundle",
            Self::ConfigSnapshot => "config_snapshot",
            Self::ProofPacketExport => "proof_packet_export",
            Self::SessionDiagnostics => "session_diagnostics",
        }
    }
}

/// A companion-safe redaction-policy row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionPolicyItem {
    /// Stable item id.
    pub item_id: String,
    /// Boundary this row governs.
    pub boundary: RedactionBoundary,
    /// Content class this row governs.
    pub content_class: RedactionContentClass,
    /// Redaction class applied before content crosses the boundary.
    pub redaction_class: RedactionClass,
    /// True when the redaction is proven by evidence.
    pub redaction_verified: bool,
    /// True when an unverified redaction carries a visible "claimed, not verified" label.
    pub redaction_label_shown: bool,
    /// Always true: no raw payload body crosses the boundary.
    pub no_payload_body: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the redaction-policy record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the redaction-policy row.
    pub handoff: CompanionDesktopHandoff,
}

/// A local-core continuity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalCoreContinuityItem {
    /// Stable item id.
    pub item_id: String,
    /// Capability this row describes.
    pub capability: ContinuityCapability,
    /// Continuity posture of the capability.
    pub continuity_posture: ContinuityPosture,
    /// True when the capability continues from the local core offline.
    pub available_offline: bool,
    /// True when the capability requires provider continuity; mirrors [`ContinuityPosture::requires_provider`].
    pub requires_provider_continuity: bool,
    /// True when the capability requires admin continuity; mirrors [`ContinuityPosture::requires_admin`].
    pub requires_admin_continuity: bool,
    /// Always true: continuity never strands user-owned local work.
    pub local_work_preserved: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the continuity record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the continuity row.
    pub handoff: CompanionDesktopHandoff,
}

/// An offline incident-packet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineIncidentPacketItem {
    /// Stable item id.
    pub item_id: String,
    /// Class of the incident packet.
    pub packet_class: IncidentPacketClass,
    /// Availability of the packet.
    pub availability: OfflinePacketAvailability,
    /// Completeness of the packet, proved where claimed.
    pub completeness: PacketCompleteness,
    /// True when the completeness claim is verified by evidence.
    pub claim_verified: bool,
    /// True when an unverified completeness claim carries a visible label.
    pub proof_label_shown: bool,
    /// Redaction class applied before the packet crosses the boundary.
    pub redaction_class: RedactionClass,
    /// True when the redaction is proven by evidence.
    pub redaction_verified: bool,
    /// True when an unverified redaction carries a visible label.
    pub redaction_label_shown: bool,
    /// True when the incident packet preserves its attribution.
    pub attribution_present: bool,
    /// True when a packet whose attribution was narrowed carries a visible label.
    pub attribution_label_shown: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the incident-packet record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the incident packet.
    pub handoff: CompanionDesktopHandoff,
}

/// An offline support-packet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineSupportPacketItem {
    /// Stable item id.
    pub item_id: String,
    /// Class of the support packet.
    pub packet_class: SupportPacketClass,
    /// Availability of the packet.
    pub availability: OfflinePacketAvailability,
    /// Completeness of the packet, proved where claimed.
    pub completeness: PacketCompleteness,
    /// True when the completeness claim is verified by evidence.
    pub claim_verified: bool,
    /// True when an unverified completeness claim carries a visible label.
    pub proof_label_shown: bool,
    /// Redaction class applied before the packet crosses the boundary.
    pub redaction_class: RedactionClass,
    /// True when the redaction is proven by evidence.
    pub redaction_verified: bool,
    /// True when an unverified redaction carries a visible label.
    pub redaction_label_shown: bool,
    /// Freshness of the row.
    pub freshness: CompanionFreshnessState,
    /// Read/write scope. Always [`CompanionReadWriteScope::ReadOnly`].
    pub read_write_scope: CompanionReadWriteScope,
    /// True when a stale/unknown freshness label is shown to the user.
    pub stale_label_shown: bool,
    /// Redacted summary. Carries no payload body.
    pub summary: String,
    /// Ref to the support-packet record. Carries no payload body.
    pub record_ref: String,
    /// Exact desktop handoff to the support packet.
    pub handoff: CompanionDesktopHandoff,
}

/// Per-section qualification inherited from the frozen M5 matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionSectionQualification {
    /// Section the row applies to.
    pub section: RedactionContinuitySection,
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
pub struct RedactionScopeContract {
    /// The redaction-policy section is read-only.
    pub redaction_policy_read_only: bool,
    /// The local-core continuity section is read-only.
    pub continuity_read_only: bool,
    /// The offline incident-packet section is read-only.
    pub incident_packet_read_only: bool,
    /// The offline support-packet section is read-only.
    pub support_packet_read_only: bool,
    /// The local core stays the authoritative source of truth.
    pub local_core_authoritative: bool,
    /// Redaction is enforced and a packet is assembled by the local core, never authored from this surface.
    pub action_applied_by_local_core_not_surface: bool,
    /// The surface never holds an unbounded write authority.
    pub no_unbounded_workspace_write: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies: bool,
}

/// Companion-safe redaction and offline-packet honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionHonestyContract {
    /// Companion-safe redaction is enforced on every boundary.
    pub companion_safe_redaction_enforced: bool,
    /// Redaction is provable or labeled as unverified.
    pub redaction_provable_or_labeled: bool,
    /// No raw payload body crosses the companion, support, or incident boundary.
    pub no_payload_body_crosses_boundary: bool,
    /// A local-first incident-packet path is always offered.
    pub incident_packet_local_path_always_available: bool,
    /// A local-first support-packet path is always offered.
    pub support_packet_local_path_always_available: bool,
    /// Packet completeness is provable or labeled as unverified.
    pub packet_completeness_provable_or_labeled: bool,
    /// Incident packets stay attributable or are honestly labeled.
    pub incident_packets_attributable_or_labeled: bool,
    /// No completeness, redaction, or attribution claim is made without backing evidence.
    pub no_claim_without_evidence: bool,
}

/// Stale-state honesty contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionStaleStateHonesty {
    /// Every stale item is labeled.
    pub stale_items_labeled: bool,
    /// Every unknown-freshness item is labeled.
    pub unknown_freshness_labeled: bool,
    /// A stale item is never shown as live.
    pub never_show_stale_as_live: bool,
    /// A freshness floor is enforced before an item is shown.
    pub freshness_floor_enforced: bool,
}

/// Local-core continuity contract for the whole surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionContinuityContract {
    /// The local core continues when the managed provider degrades.
    pub local_core_continues_when_provider_degrades: bool,
    /// Local-core-authoritative capabilities are available offline.
    pub capabilities_available_offline: bool,
    /// A degraded capability is labeled, not hidden.
    pub degraded_capability_labeled_not_hidden: bool,
    /// User-owned local work is never stranded.
    pub local_work_never_stranded: bool,
    /// Provider and admin continuity requirements are disclosed.
    pub provider_and_admin_continuity_disclosed: bool,
}

/// Security and privacy review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionSecurityReview {
    /// The redaction-policy section is read-only.
    pub redaction_policy_read_only: bool,
    /// The local-core continuity section is read-only.
    pub continuity_read_only: bool,
    /// The offline incident-packet section is read-only.
    pub incident_packet_read_only: bool,
    /// The offline support-packet section is read-only.
    pub support_packet_read_only: bool,
    /// The local core stays the authoritative source of truth.
    pub local_core_authoritative: bool,
    /// Redaction/assembly is applied by the local core, never from this surface.
    pub action_applied_by_local_core_not_surface: bool,
    /// Companion-safe redaction is enforced on every boundary.
    pub companion_safe_redaction_enforced: bool,
    /// Redaction is provable or labeled as unverified.
    pub redaction_provable_or_labeled: bool,
    /// No raw payload body crosses the boundary.
    pub no_payload_body_crosses_boundary: bool,
    /// A local-first incident-packet path is always offered.
    pub incident_packet_local_path_always_available: bool,
    /// A local-first support-packet path is always offered.
    pub support_packet_local_path_always_available: bool,
    /// Packet completeness is provable or labeled as unverified.
    pub packet_completeness_provable_or_labeled: bool,
    /// Incident packets stay attributable or are honestly labeled.
    pub incident_packets_attributable_or_labeled: bool,
    /// Local-core-authoritative capabilities are available offline.
    pub capabilities_available_offline: bool,
    /// User-owned local work is never stranded.
    pub local_work_never_stranded: bool,
    /// Stale state is labeled rather than hidden.
    pub stale_state_labeled_never_hidden: bool,
    /// Exact desktop handoff is preserved or honestly degraded.
    pub exact_desktop_handoff_preserved: bool,
    /// No credential or provider bodies cross the export boundary.
    pub no_credential_or_provider_bodies_in_export: bool,
    /// No payload bodies cross the export boundary.
    pub no_payload_bodies_in_export: bool,
    /// Downgrade narrows the claim rather than hiding the section.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Every section discloses local, staged, and provider/admin continuity.
    pub locality_disclosed: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionConsumerProjection {
    /// Desktop panel projects the redaction policy.
    pub desktop_panel_shows_redaction_policy: bool,
    /// Desktop panel projects the local-core continuity.
    pub desktop_panel_shows_continuity: bool,
    /// Incident workspace projects the offline incident packets.
    pub incident_workspace_shows_offline_incident_packets: bool,
    /// Diagnostics shows the offline support packets.
    pub diagnostics_shows_offline_support_packets: bool,
    /// CLI / headless shows the redaction and offline-packet state.
    pub cli_headless_shows_redaction_and_packet_state: bool,
    /// Support export shows the redaction and packet state.
    pub support_export_shows_redaction_and_packets: bool,
    /// Help / About shows the redaction and continuity honesty.
    pub help_about_shows_redaction_and_continuity_honesty: bool,
    /// Preview / Labs sections are visibly labeled when not qualified Stable.
    pub preview_labs_label_for_unqualified_sections: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the section.
    pub auto_narrow_on_stale: bool,
}

/// Per-observation signal fed to
/// [`RedactionContinuitySurfacePacket::apply_redaction_degradation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactionDegradationObservation {
    /// True when redaction proofs are available (redaction can be verified).
    pub redaction_proof_available: bool,
    /// True when the offline packet assembler is available.
    pub packet_assembler_available: bool,
    /// True when proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when packet completeness claims are verified.
    pub completeness_verified: bool,
    /// True when incident attribution is available.
    pub incident_attribution_available: bool,
    /// True when the managed service is available (not degraded).
    pub managed_service_available: bool,
    /// True when an active desktop host session exists.
    pub host_session_active: bool,
    /// True when an upstream frozen matrix lane narrowed.
    pub upstream_matrix_narrowed: bool,
}

/// Reason a section has degraded below its qualified state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionDegradedReason {
    /// Redaction proofs are unavailable.
    RedactionProofUnavailable,
    /// The offline packet assembler is unavailable.
    PacketAssemblerUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// A packet completeness claim could not be verified.
    CompletenessUnverified,
    /// Incident attribution is unavailable.
    IncidentAttributionUnavailable,
    /// The managed service is degraded.
    ManagedServiceDegraded,
    /// No active desktop host session.
    HostSessionInactive,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more desktop handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
    /// One or more item freshness states were downgraded to stale.
    FreshnessDowngradedToStale,
    /// One or more redaction claims were narrowed to a more conservative class.
    RedactionNarrowedToReference,
    /// One or more packets narrowed to their local path.
    PacketNarrowedToLocalPath,
    /// One or more completeness claims were downgraded to claimed-but-unverified.
    CompletenessClaimDowngraded,
    /// One or more incident packets had their attribution narrowed and labeled.
    IncidentAttributionNarrowed,
}

impl RedactionDegradedReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactionProofUnavailable => "redaction_proof_unavailable",
            Self::PacketAssemblerUnavailable => "packet_assembler_unavailable",
            Self::ProofStale => "proof_stale",
            Self::CompletenessUnverified => "completeness_unverified",
            Self::IncidentAttributionUnavailable => "incident_attribution_unavailable",
            Self::ManagedServiceDegraded => "managed_service_degraded",
            Self::HostSessionInactive => "host_session_inactive",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::FreshnessDowngradedToStale => "freshness_downgraded_to_stale",
            Self::RedactionNarrowedToReference => "redaction_narrowed_to_reference",
            Self::PacketNarrowedToLocalPath => "packet_narrowed_to_local_path",
            Self::CompletenessClaimDowngraded => "completeness_claim_downgraded",
            Self::IncidentAttributionNarrowed => "incident_attribution_narrowed",
        }
    }
}

/// Constructor input for [`RedactionContinuitySurfacePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RedactionContinuitySurfacePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<RedactionSectionQualification>,
    /// Redaction-policy items.
    pub redaction_policy: Vec<RedactionPolicyItem>,
    /// Local-core continuity items.
    pub continuity_rows: Vec<LocalCoreContinuityItem>,
    /// Offline incident-packet items.
    pub incident_packets: Vec<OfflineIncidentPacketItem>,
    /// Offline support-packet items.
    pub support_packets: Vec<OfflineSupportPacketItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: RedactionScopeContract,
    /// Redaction and offline-packet honesty contract.
    pub honesty_contract: RedactionHonestyContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: RedactionStaleStateHonesty,
    /// Local-core continuity contract.
    pub continuity_contract: RedactionContinuityContract,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: RedactionSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: RedactionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RedactionProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe redaction/continuity/offline-packet surface packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionContinuitySurfacePacket {
    /// Record kind; must equal [`REDACTION_CONTINUITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`REDACTION_CONTINUITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer surfaces that project this packet.
    pub projected_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Per-section qualification rows.
    pub section_qualifications: Vec<RedactionSectionQualification>,
    /// Redaction-policy items.
    pub redaction_policy: Vec<RedactionPolicyItem>,
    /// Local-core continuity items.
    pub continuity_rows: Vec<LocalCoreContinuityItem>,
    /// Offline incident-packet items.
    pub incident_packets: Vec<OfflineIncidentPacketItem>,
    /// Offline support-packet items.
    pub support_packets: Vec<OfflineSupportPacketItem>,
    /// Read/write scope and authority contract.
    pub scope_contract: RedactionScopeContract,
    /// Redaction and offline-packet honesty contract.
    pub honesty_contract: RedactionHonestyContract,
    /// Stale-state honesty contract.
    pub stale_state_honesty: RedactionStaleStateHonesty,
    /// Local-core continuity contract.
    pub continuity_contract: RedactionContinuityContract,
    /// Locality disclosure.
    pub locality_disclosure: M5CompanionLocalityDisclosure,
    /// Security review block.
    pub security_review: RedactionSecurityReview,
    /// Consumer projection block.
    pub consumer_projection: RedactionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: RedactionProofFreshness,
    /// Degraded-state labels currently applied (empty when fully qualified).
    pub degraded_labels: Vec<RedactionDegradedReason>,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RedactionContinuitySurfacePacket {
    /// Builds a redaction/continuity/offline-packet surface packet from stable-lane input.
    pub fn new(input: RedactionContinuitySurfacePacketInput) -> Self {
        Self {
            record_kind: REDACTION_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: REDACTION_CONTINUITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            projected_surfaces: input.projected_surfaces,
            section_qualifications: input.section_qualifications,
            redaction_policy: input.redaction_policy,
            continuity_rows: input.continuity_rows,
            incident_packets: input.incident_packets,
            support_packets: input.support_packets,
            scope_contract: input.scope_contract,
            honesty_contract: input.honesty_contract,
            stale_state_honesty: input.stale_state_honesty,
            continuity_contract: input.continuity_contract,
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

    /// Narrows sections, narrows redaction to its conservative class, narrows packet
    /// availability to its local path, downgrades completeness claims, and downgrades
    /// freshness from a per-observation signal, recording the reasons in
    /// [`Self::degraded_labels`].
    ///
    /// A degraded managed service, stale proof, or narrowed upstream matrix lane narrows
    /// every section one step, and a degraded managed service additionally forces every
    /// live or cached item to stale and labels it. When redaction proof is unavailable,
    /// every verified redaction (in every section) downgrades to claimed-but-unverified,
    /// is labeled, and a redacted-summary class narrows to reference-only. When the offline
    /// packet assembler is unavailable, every provider-assembled packet narrows to
    /// [`OfflinePacketAvailability::Unavailable`] while the local path always remains. When
    /// completeness is unverified, every verified completeness claim downgrades to
    /// [`PacketCompleteness::CompleteUnverified`] and is labeled. When incident attribution
    /// is unavailable, every attributed incident packet narrows its attribution and labels
    /// it. An inactive host session downgrades the resolution of every host-dependent
    /// desktop handoff. The local path always remains, and local work is always preserved.
    /// Degraded state is labeled, never hidden.
    pub fn apply_redaction_degradation(&mut self, observation: &RedactionDegradationObservation) {
        let mut labels: BTreeSet<RedactionDegradedReason> =
            self.degraded_labels.iter().copied().collect();

        let section_adverse = !observation.managed_service_available
            || !observation.proof_fresh
            || observation.upstream_matrix_narrowed;

        if !observation.managed_service_available {
            labels.insert(RedactionDegradedReason::ManagedServiceDegraded);
            if self.force_all_freshness_stale() {
                labels.insert(RedactionDegradedReason::FreshnessDowngradedToStale);
            }
        }
        if !observation.proof_fresh {
            labels.insert(RedactionDegradedReason::ProofStale);
        }
        if observation.upstream_matrix_narrowed {
            labels.insert(RedactionDegradedReason::UpstreamMatrixNarrowed);
        }
        if !observation.redaction_proof_available {
            labels.insert(RedactionDegradedReason::RedactionProofUnavailable);
            // A redaction downgrade always carries a visible label; the conservative-class
            // narrowing is recorded separately only when a class actually moved.
            let (_downgraded, narrowed) = self.force_all_redaction_unverified();
            if narrowed {
                labels.insert(RedactionDegradedReason::RedactionNarrowedToReference);
            }
        }
        if !observation.packet_assembler_available {
            labels.insert(RedactionDegradedReason::PacketAssemblerUnavailable);
            let mut narrowed = false;
            for item in &mut self.incident_packets {
                let next = item.availability.narrowed_when_provider_lost();
                if next != item.availability {
                    item.availability = next;
                    narrowed = true;
                }
            }
            for item in &mut self.support_packets {
                let next = item.availability.narrowed_when_provider_lost();
                if next != item.availability {
                    item.availability = next;
                    narrowed = true;
                }
            }
            if narrowed {
                labels.insert(RedactionDegradedReason::PacketNarrowedToLocalPath);
            }
        }
        if !observation.completeness_verified {
            labels.insert(RedactionDegradedReason::CompletenessUnverified);
            if self.force_all_completeness_unverified() {
                labels.insert(RedactionDegradedReason::CompletenessClaimDowngraded);
            }
        }
        if !observation.incident_attribution_available {
            labels.insert(RedactionDegradedReason::IncidentAttributionUnavailable);
            if self.force_all_incident_attribution_narrowed() {
                labels.insert(RedactionDegradedReason::IncidentAttributionNarrowed);
            }
        }

        for row in &mut self.section_qualifications {
            let adverse = section_adverse
                || (!observation.redaction_proof_available
                    && row.section == RedactionContinuitySection::RedactionPolicy)
                || (!observation.packet_assembler_available
                    && matches!(
                        row.section,
                        RedactionContinuitySection::OfflineIncidentPacket
                            | RedactionContinuitySection::OfflineSupportPacket
                    ))
                || (!observation.completeness_verified
                    && matches!(
                        row.section,
                        RedactionContinuitySection::OfflineIncidentPacket
                            | RedactionContinuitySection::OfflineSupportPacket
                    ))
                || (!observation.incident_attribution_available
                    && row.section == RedactionContinuitySection::OfflineIncidentPacket);
            if adverse {
                row.qualification = row.qualification.narrowed_one_step();
                row.rollout_stage = row.rollout_stage.narrowed_one_step();
            }
        }

        if !observation.host_session_active {
            labels.insert(RedactionDegradedReason::HostSessionInactive);
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
                labels.insert(RedactionDegradedReason::HandoffTargetUnresolved);
            }
        }

        self.degraded_labels = labels.into_iter().collect();
    }

    /// Forces every live/cached item freshness to stale and labels it. Returns true when
    /// at least one item was downgraded.
    fn force_all_freshness_stale(&mut self) -> bool {
        let mut downgraded = false;
        for (state, label) in self.freshness_states_mut() {
            if *state != state.forced_stale() {
                *state = state.forced_stale();
                *label = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Downgrades every verified redaction (across every section) to unverified, labels it,
    /// and narrows a redacted-summary class to reference-only. Returns `(downgraded,
    /// narrowed)`.
    fn force_all_redaction_unverified(&mut self) -> (bool, bool) {
        let mut downgraded = false;
        let mut narrowed = false;
        for item in &mut self.redaction_policy {
            let (d, n) = narrow_redaction(
                &mut item.redaction_class,
                &mut item.redaction_verified,
                &mut item.redaction_label_shown,
            );
            downgraded |= d;
            narrowed |= n;
        }
        for item in &mut self.incident_packets {
            let (d, n) = narrow_redaction(
                &mut item.redaction_class,
                &mut item.redaction_verified,
                &mut item.redaction_label_shown,
            );
            downgraded |= d;
            narrowed |= n;
        }
        for item in &mut self.support_packets {
            let (d, n) = narrow_redaction(
                &mut item.redaction_class,
                &mut item.redaction_verified,
                &mut item.redaction_label_shown,
            );
            downgraded |= d;
            narrowed |= n;
        }
        (downgraded, narrowed)
    }

    /// Downgrades every verified completeness claim (in incident and support packets) to
    /// unverified and labels it. Returns true when at least one claim was downgraded.
    fn force_all_completeness_unverified(&mut self) -> bool {
        let mut downgraded = false;
        for item in &mut self.incident_packets {
            if item.completeness.is_complete_claim() {
                item.completeness = item.completeness.downgraded_to_unverified();
                item.claim_verified = false;
                item.proof_label_shown = true;
                downgraded = true;
            }
        }
        for item in &mut self.support_packets {
            if item.completeness.is_complete_claim() {
                item.completeness = item.completeness.downgraded_to_unverified();
                item.claim_verified = false;
                item.proof_label_shown = true;
                downgraded = true;
            }
        }
        downgraded
    }

    /// Narrows the attribution of every attributed incident packet and labels it. Returns
    /// true when at least one packet was narrowed.
    fn force_all_incident_attribution_narrowed(&mut self) -> bool {
        let mut narrowed = false;
        for item in &mut self.incident_packets {
            if item.attribution_present {
                item.attribution_present = false;
                item.attribution_label_shown = true;
                narrowed = true;
            }
        }
        narrowed
    }

    /// Mutable access to every item's freshness state and stale-label flag.
    fn freshness_states_mut(
        &mut self,
    ) -> impl Iterator<Item = (&mut CompanionFreshnessState, &mut bool)> {
        self.redaction_policy
            .iter_mut()
            .map(|item| (&mut item.freshness, &mut item.stale_label_shown))
            .chain(
                self.continuity_rows
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.incident_packets
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
            .chain(
                self.support_packets
                    .iter_mut()
                    .map(|item| (&mut item.freshness, &mut item.stale_label_shown)),
            )
    }

    /// Validates the surface invariants.
    pub fn validate(&self) -> Vec<RedactionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != REDACTION_CONTINUITY_RECORD_KIND {
            violations.push(RedactionViolation::WrongRecordKind);
        }
        if self.schema_version != REDACTION_CONTINUITY_SCHEMA_VERSION {
            violations.push(RedactionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(RedactionViolation::MissingIdentity);
        }
        if self.projected_surfaces.is_empty() {
            violations.push(RedactionViolation::ProjectedSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_section_qualifications(self, &mut violations);
        validate_items(self, &mut violations);
        validate_scope_contract(self, &mut violations);
        validate_honesty_contract(self, &mut violations);
        validate_stale_state_honesty(self, &mut violations);
        validate_continuity_contract(self, &mut violations);
        validate_locality(self, &mut violations);
        validate_security_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("redaction/continuity packet serializes"),
        ) {
            violations.push(RedactionViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("redaction/continuity packet serializes")
    }

    /// Sections currently publishable (Stable, Beta, or Preview) and not withheld.
    pub fn publishable_sections(&self) -> impl Iterator<Item = &RedactionSectionQualification> {
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

    /// True when every redaction claim (across every section) is verified or labeled.
    pub fn redaction_provable_or_labeled(&self) -> bool {
        let row_ok = |verified: bool, labeled: bool| verified || labeled;
        self.redaction_policy
            .iter()
            .all(|item| row_ok(item.redaction_verified, item.redaction_label_shown))
            && self
                .incident_packets
                .iter()
                .all(|item| row_ok(item.redaction_verified, item.redaction_label_shown))
            && self
                .support_packets
                .iter()
                .all(|item| row_ok(item.redaction_verified, item.redaction_label_shown))
    }

    /// True when no redaction-policy row carries a payload body.
    pub fn no_payload_body_crosses_boundary(&self) -> bool {
        self.redaction_policy
            .iter()
            .all(|item| item.no_payload_body)
    }

    /// True when a local-first incident-packet path is offered as a fallback.
    pub fn incident_packet_local_path_available(&self) -> bool {
        self.incident_packets
            .iter()
            .any(|item| item.availability.is_local_path())
    }

    /// True when a local-first support-packet path is offered as a fallback.
    pub fn support_packet_local_path_available(&self) -> bool {
        self.support_packets
            .iter()
            .any(|item| item.availability.is_local_path())
    }

    /// True when every packet completeness claim is verified or labeled.
    pub fn packet_completeness_honestly_qualified(&self) -> bool {
        let row_ok = |complete: PacketCompleteness, verified: bool, labeled: bool| {
            if complete.is_complete_claim() {
                verified
            } else if complete == PacketCompleteness::CompleteUnverified {
                labeled
            } else {
                true
            }
        };
        self.incident_packets.iter().all(|item| {
            row_ok(
                item.completeness,
                item.claim_verified,
                item.proof_label_shown,
            )
        }) && self.support_packets.iter().all(|item| {
            row_ok(
                item.completeness,
                item.claim_verified,
                item.proof_label_shown,
            )
        })
    }

    /// True when every incident packet stays attributable or is honestly labeled.
    pub fn incident_packets_attributable(&self) -> bool {
        self.incident_packets
            .iter()
            .all(|item| item.attribution_present || item.attribution_label_shown)
    }

    /// True when local-core continuity never strands user-owned local work and every
    /// local-core-authoritative capability stays available offline.
    pub fn local_core_continuity_preserved(&self) -> bool {
        self.continuity_rows.iter().all(|item| {
            item.local_work_preserved
                && (!item.continuity_posture.continues_offline() || item.available_offline)
        })
    }

    /// True when every stale or unknown-freshness item carries a visible label.
    pub fn stale_state_honestly_labeled(&self) -> bool {
        self.redaction_policy
            .iter()
            .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .continuity_rows
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .incident_packets
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
            && self
                .support_packets
                .iter()
                .all(|item| !item.freshness.requires_label() || item.stale_label_shown)
    }

    /// Iterates every desktop handoff across all four sections, in section order.
    pub fn handoffs(&self) -> impl Iterator<Item = &CompanionDesktopHandoff> {
        self.redaction_policy
            .iter()
            .map(|item| &item.handoff)
            .chain(self.continuity_rows.iter().map(|item| &item.handoff))
            .chain(self.incident_packets.iter().map(|item| &item.handoff))
            .chain(self.support_packets.iter().map(|item| &item.handoff))
    }

    fn handoffs_mut(&mut self) -> impl Iterator<Item = &mut CompanionDesktopHandoff> {
        self.redaction_policy
            .iter_mut()
            .map(|item| &mut item.handoff)
            .chain(
                self.continuity_rows
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(
                self.incident_packets
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
            .chain(
                self.support_packets
                    .iter_mut()
                    .map(|item| &mut item.handoff),
            )
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Companion-Safe Redaction, Local-Core Continuity, and Offline Packet Flows Across Support and Incident Lanes\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Sections: {} | Redaction-policy rows: {} | Continuity rows: {} | Incident packets: {} | Support packets: {}\n",
            self.section_qualifications.len(),
            self.redaction_policy.len(),
            self.continuity_rows.len(),
            self.incident_packets.len(),
            self.support_packets.len(),
        ));
        out.push_str(&format!(
            "- Exact desktop handoff for every item: {}\n",
            yes_no(self.all_handoffs_exact())
        ));
        out.push_str(&format!(
            "- Redaction provable or labeled: {}\n",
            yes_no(self.redaction_provable_or_labeled())
        ));
        out.push_str(&format!(
            "- No payload body crosses boundary: {}\n",
            yes_no(self.no_payload_body_crosses_boundary())
        ));
        out.push_str(&format!(
            "- Local incident-packet path available: {}\n",
            yes_no(self.incident_packet_local_path_available())
        ));
        out.push_str(&format!(
            "- Local support-packet path available: {}\n",
            yes_no(self.support_packet_local_path_available())
        ));
        out.push_str(&format!(
            "- Packet completeness honestly qualified: {}\n",
            yes_no(self.packet_completeness_honestly_qualified())
        ));
        out.push_str(&format!(
            "- Incident packets attributable: {}\n",
            yes_no(self.incident_packets_attributable())
        ));
        out.push_str(&format!(
            "- Local-core continuity preserved: {}\n",
            yes_no(self.local_core_continuity_preserved())
        ));
        out.push_str(&format!(
            "- Stale state honestly labeled: {}\n",
            yes_no(self.stale_state_honestly_labeled())
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

        out.push_str("\n## Redaction policy\n\n");
        for item in &self.redaction_policy {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] (verified: {}) {} ({}) → `{}` ({})\n",
                item.item_id,
                item.boundary.as_str(),
                item.content_class.as_str(),
                item.redaction_class.as_str(),
                yes_no(item.redaction_verified),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Local-core continuity\n\n");
        for item in &self.continuity_rows {
            out.push_str(&format!(
                "- `{}` [{}/{}] offline `{}` local_work_preserved `{}` {} ({}) → `{}` ({})\n",
                item.item_id,
                item.capability.as_str(),
                item.continuity_posture.as_str(),
                yes_no(item.available_offline),
                yes_no(item.local_work_preserved),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Offline incident packets\n\n");
        for item in &self.incident_packets {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] redaction `{}` (verified: {}) attribution `{}` {} ({}) → `{}` ({})\n",
                item.item_id,
                item.packet_class.as_str(),
                item.availability.as_str(),
                item.completeness.as_str(),
                item.redaction_class.as_str(),
                yes_no(item.redaction_verified),
                yes_no(item.attribution_present),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out.push_str("\n## Offline support packets\n\n");
        for item in &self.support_packets {
            out.push_str(&format!(
                "- `{}` [{}/{}/{}] redaction `{}` (verified: {}) {} ({}) → `{}` ({})\n",
                item.item_id,
                item.packet_class.as_str(),
                item.availability.as_str(),
                item.completeness.as_str(),
                item.redaction_class.as_str(),
                yes_no(item.redaction_verified),
                item.summary,
                item.freshness.as_str(),
                item.handoff.target.as_str(),
                item.handoff.resolution.as_str(),
            ));
        }

        out
    }
}

/// Downgrades a verified redaction to claimed-but-unverified, labels it, and narrows a
/// redacted-summary class to reference-only. Returns `(downgraded, narrowed)`.
fn narrow_redaction(
    class: &mut RedactionClass,
    verified: &mut bool,
    label: &mut bool,
) -> (bool, bool) {
    if !*verified {
        return (false, false);
    }
    *verified = false;
    *label = true;
    let next = class.narrowed_when_proof_lost();
    let narrowed = next != *class;
    *class = next;
    (true, narrowed)
}

fn yes_no(value: bool) -> &'static str {
    if value {
        "yes"
    } else {
        "no"
    }
}

/// Errors emitted when reading the checked-in surface export.
#[derive(Debug)]
pub enum RedactionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<RedactionViolation>),
}

impl fmt::Display for RedactionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "redaction/continuity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "redaction/continuity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for RedactionArtifactError {}

/// Validation failures emitted by [`RedactionContinuitySurfacePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RedactionViolation {
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
    /// A redaction-policy row or packet carries a payload body.
    PayloadBodyPresent,
    /// An unverified redaction claim is not labeled.
    RedactionClaimNotLabeled,
    /// No local-first incident-packet path is offered.
    LocalIncidentPacketPathMissing,
    /// No local-first support-packet path is offered.
    LocalSupportPacketPathMissing,
    /// A completeness claim is verified-marked without backing verification.
    CompletenessClaimedButUnverified,
    /// An unverified completeness claim is not labeled.
    CompletenessClaimNotLabeled,
    /// An incident packet without attribution is not labeled.
    IncidentAttributionNotLabeled,
    /// A continuity row's offline-availability does not match its posture.
    ContinuityOfflineMismatch,
    /// A continuity row's provider/admin flags do not match its posture.
    ContinuityFlagMismatch,
    /// A continuity row does not preserve local work.
    LocalWorkStranded,
    /// An item is missing identity or a redacted body.
    ItemIncomplete,
    /// A stale or unknown-freshness item is not labeled.
    StaleStateNotLabeled,
    /// An item's desktop handoff is missing its deep-link ref.
    HandoffRefMissing,
    /// The read/write scope contract is not fully satisfied.
    ScopeContractIncomplete,
    /// The redaction/offline-packet honesty contract is not fully satisfied.
    HonestyContractIncomplete,
    /// The stale-state honesty contract is not fully satisfied.
    StaleStateHonestyIncomplete,
    /// The continuity contract is not fully satisfied.
    ContinuityContractIncomplete,
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

impl RedactionViolation {
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
            Self::PayloadBodyPresent => "payload_body_present",
            Self::RedactionClaimNotLabeled => "redaction_claim_not_labeled",
            Self::LocalIncidentPacketPathMissing => "local_incident_packet_path_missing",
            Self::LocalSupportPacketPathMissing => "local_support_packet_path_missing",
            Self::CompletenessClaimedButUnverified => "completeness_claimed_but_unverified",
            Self::CompletenessClaimNotLabeled => "completeness_claim_not_labeled",
            Self::IncidentAttributionNotLabeled => "incident_attribution_not_labeled",
            Self::ContinuityOfflineMismatch => "continuity_offline_mismatch",
            Self::ContinuityFlagMismatch => "continuity_flag_mismatch",
            Self::LocalWorkStranded => "local_work_stranded",
            Self::ItemIncomplete => "item_incomplete",
            Self::StaleStateNotLabeled => "stale_state_not_labeled",
            Self::HandoffRefMissing => "handoff_ref_missing",
            Self::ScopeContractIncomplete => "scope_contract_incomplete",
            Self::HonestyContractIncomplete => "honesty_contract_incomplete",
            Self::StaleStateHonestyIncomplete => "stale_state_honesty_incomplete",
            Self::ContinuityContractIncomplete => "continuity_contract_incomplete",
            Self::LocalityDisclosureIncomplete => "locality_disclosure_incomplete",
            Self::SecurityReviewIncomplete => "security_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable surface export.
///
/// This is the canonical reader: the desktop companion panel, the CLI/headless surface,
/// diagnostics, support-export, or Help/About surface calls it to ingest the packet rather
/// than cloning status text.
///
/// # Errors
///
/// Returns [`RedactionArtifactError`] when the checked-in support export fails to parse or
/// fails validation.
pub fn current_stable_redaction_continuity_surface_export(
) -> Result<RedactionContinuitySurfacePacket, RedactionArtifactError> {
    let packet: RedactionContinuitySurfacePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/companion/m5/ship_companion_safe_redaction_local_core_continuity_and_offline_packet_flows_across_support_and_incident_lanes/support_export.json"
    )))
    .map_err(RedactionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(RedactionArtifactError::Validation(violations))
    }
}

/// Canonical source contract refs that every export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        REDACTION_CONTINUITY_SCHEMA_REF.to_owned(),
        REDACTION_CONTINUITY_DOC_REF.to_owned(),
        M5_COMPANION_SURFACE_CONTRACT_REF.to_owned(),
        M5_INCIDENT_WORKSPACE_CONTRACT_REF.to_owned(),
        M5_OFFBOARDING_CONTRACT_REF.to_owned(),
        M5_COMPANION_MATRIX_SCHEMA_REF.to_owned(),
    ]
}

/// Canonical read/write scope and authority contract with every guarantee met.
pub fn canonical_scope_contract() -> RedactionScopeContract {
    RedactionScopeContract {
        redaction_policy_read_only: true,
        continuity_read_only: true,
        incident_packet_read_only: true,
        support_packet_read_only: true,
        local_core_authoritative: true,
        action_applied_by_local_core_not_surface: true,
        no_unbounded_workspace_write: true,
        no_payload_bodies: true,
    }
}

/// Canonical redaction/offline-packet honesty contract with every guarantee satisfied.
pub fn canonical_honesty_contract() -> RedactionHonestyContract {
    RedactionHonestyContract {
        companion_safe_redaction_enforced: true,
        redaction_provable_or_labeled: true,
        no_payload_body_crosses_boundary: true,
        incident_packet_local_path_always_available: true,
        support_packet_local_path_always_available: true,
        packet_completeness_provable_or_labeled: true,
        incident_packets_attributable_or_labeled: true,
        no_claim_without_evidence: true,
    }
}

/// Canonical stale-state honesty contract with every guarantee satisfied.
pub fn canonical_stale_state_honesty() -> RedactionStaleStateHonesty {
    RedactionStaleStateHonesty {
        stale_items_labeled: true,
        unknown_freshness_labeled: true,
        never_show_stale_as_live: true,
        freshness_floor_enforced: true,
    }
}

/// Canonical local-core continuity contract with every guarantee satisfied.
pub fn canonical_continuity_contract() -> RedactionContinuityContract {
    RedactionContinuityContract {
        local_core_continues_when_provider_degrades: true,
        capabilities_available_offline: true,
        degraded_capability_labeled_not_hidden: true,
        local_work_never_stranded: true,
        provider_and_admin_continuity_disclosed: true,
    }
}

/// Canonical security review block with every invariant satisfied.
pub fn canonical_security_review() -> RedactionSecurityReview {
    RedactionSecurityReview {
        redaction_policy_read_only: true,
        continuity_read_only: true,
        incident_packet_read_only: true,
        support_packet_read_only: true,
        local_core_authoritative: true,
        action_applied_by_local_core_not_surface: true,
        companion_safe_redaction_enforced: true,
        redaction_provable_or_labeled: true,
        no_payload_body_crosses_boundary: true,
        incident_packet_local_path_always_available: true,
        support_packet_local_path_always_available: true,
        packet_completeness_provable_or_labeled: true,
        incident_packets_attributable_or_labeled: true,
        capabilities_available_offline: true,
        local_work_never_stranded: true,
        stale_state_labeled_never_hidden: true,
        exact_desktop_handoff_preserved: true,
        no_credential_or_provider_bodies_in_export: true,
        no_payload_bodies_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        locality_disclosed: true,
    }
}

/// Canonical consumer projection block with every section projecting truth.
pub fn canonical_consumer_projection() -> RedactionConsumerProjection {
    RedactionConsumerProjection {
        desktop_panel_shows_redaction_policy: true,
        desktop_panel_shows_continuity: true,
        incident_workspace_shows_offline_incident_packets: true,
        diagnostics_shows_offline_support_packets: true,
        cli_headless_shows_redaction_and_packet_state: true,
        support_export_shows_redaction_and_packets: true,
        help_about_shows_redaction_and_continuity_honesty: true,
        preview_labs_label_for_unqualified_sections: true,
    }
}

/// Canonical per-section qualification rows, inherited from the frozen matrix.
///
/// The redaction-policy, continuity, and support-packet sections earn the Beta/staged
/// qualification because companion-safe redaction is enforced, the local core stays
/// authoritative, and a local-first path is always available; the offline incident-packet
/// section inherits the Preview/early-access qualification because its attribution and
/// provider-assembled paths are less mature.
pub fn canonical_section_qualifications() -> Vec<RedactionSectionQualification> {
    use M5CompanionDowngradeTrigger as Trigger;
    use M5CompanionQualificationClass as Qual;
    use M5CompanionRollbackPosture as Rollback;
    use M5CompanionRolloutStage as Stage;
    use RedactionContinuitySection as Section;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        RedactionSectionQualification {
            section: Section::RedactionPolicy,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::RedactionPolicy.matrix_lane().as_str().to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::TrustNarrowing,
                Trigger::CompanionScopeExpansionUnqualified,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::CompanionReadOnlyNarrowScope,
        },
        RedactionSectionQualification {
            section: Section::LocalCoreContinuity,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::LocalCoreContinuity
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::LocalCoreContinuesNoRemoteState,
        },
        RedactionSectionQualification {
            section: Section::OfflineIncidentPacket,
            qualification: Qual::Preview,
            rollout_stage: Stage::EarlyAccess,
            read_write_scope: scope,
            matrix_lane_ref: Section::OfflineIncidentPacket
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::IncidentAttributionMissing,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::EvidencePreservedNoRevert,
        },
        RedactionSectionQualification {
            section: Section::OfflineSupportPacket,
            qualification: Qual::Beta,
            rollout_stage: Stage::StagedRollout,
            read_write_scope: scope,
            matrix_lane_ref: Section::OfflineSupportPacket
                .matrix_lane()
                .as_str()
                .to_owned(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProviderUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::OffboardingExportPreservesLocalWork,
        },
    ]
}

/// Canonical locality disclosure for the surface.
pub fn canonical_locality_disclosure() -> M5CompanionLocalityDisclosure {
    M5CompanionLocalityDisclosure {
        stays_local:
            "Companion-safe redaction is enforced by the local core on every companion, support, and incident boundary so no raw payload body crosses; the local core stays the authoritative source of truth and its capabilities keep working offline; and a local-first incident-packet and support-packet path is always offered and always assembles and replays offline."
                .to_owned(),
        staged:
            "Provider-assembled incident and support packets and provider-continuity capabilities roll out per cohort and managed tenant and are visibly labeled until qualified."
                .to_owned(),
        requires_provider_or_admin_continuity:
            "Provider-assembled packets require the offline packet assembler upstream, provider-continuity capabilities require the managed service, and a verified redaction or incident attribution requires its proof; these claims are shown as proven only when verifiable. When a provider degrades the local path keeps the support and incident workflow working, redaction narrows to a more conservative class, and the degraded capability is labeled, never hidden."
                .to_owned(),
    }
}

fn desktop_handoff(deep_link_ref: &str, requires_active_host: bool) -> CompanionDesktopHandoff {
    CompanionDesktopHandoff {
        target: CompanionHandoffTarget::ReviewPanel,
        deep_link_ref: deep_link_ref.to_owned(),
        resolution: CompanionHandoffResolution::Exact,
        requires_active_host,
    }
}

/// Canonical redaction-policy items.
pub fn canonical_redaction_policy() -> Vec<RedactionPolicyItem> {
    use CompanionFreshnessState as Fresh;
    use RedactionBoundary as Boundary;
    use RedactionClass as Class;
    use RedactionContentClass as Content;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        RedactionPolicyItem {
            item_id: "redact:0001".to_owned(),
            boundary: Boundary::Companion,
            content_class: Content::NotificationBody,
            redaction_class: Class::RedactedSummary,
            redaction_verified: true,
            redaction_label_shown: false,
            no_payload_body: true,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Companion notification bodies cross as a redacted summary; redaction verified"
                    .to_owned(),
            record_ref: "redaction:companion:notification".to_owned(),
            handoff: desktop_handoff("handoff:redact:0001", false),
        },
        RedactionPolicyItem {
            item_id: "redact:0002".to_owned(),
            boundary: Boundary::Support,
            content_class: Content::SupportDiagnostics,
            redaction_class: Class::MetadataOnly,
            redaction_verified: true,
            redaction_label_shown: false,
            no_payload_body: true,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Support diagnostics cross as metadata only; redaction verified".to_owned(),
            record_ref: "redaction:support:diagnostics".to_owned(),
            handoff: desktop_handoff("handoff:redact:0002", false),
        },
        RedactionPolicyItem {
            item_id: "redact:0003".to_owned(),
            boundary: Boundary::Incident,
            content_class: Content::IncidentEvidence,
            redaction_class: Class::ReferenceOnly,
            redaction_verified: false,
            redaction_label_shown: true,
            no_payload_body: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary:
                "Incident evidence crosses as an opaque ref; redaction not yet verified; labeled"
                    .to_owned(),
            record_ref: "redaction:incident:evidence".to_owned(),
            handoff: desktop_handoff("handoff:redact:0003", false),
        },
    ]
}

/// Canonical local-core continuity items.
pub fn canonical_continuity_rows() -> Vec<LocalCoreContinuityItem> {
    use CompanionFreshnessState as Fresh;
    use ContinuityCapability as Cap;
    use ContinuityPosture as Posture;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        LocalCoreContinuityItem {
            item_id: "cont:0001".to_owned(),
            capability: Cap::LocalEditing,
            continuity_posture: Posture::LocalCoreAuthoritative,
            available_offline: true,
            requires_provider_continuity: false,
            requires_admin_continuity: false,
            local_work_preserved: true,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Local editing is authoritative and works fully offline".to_owned(),
            record_ref: "continuity:local-editing".to_owned(),
            handoff: desktop_handoff("handoff:cont:0001", false),
        },
        LocalCoreContinuityItem {
            item_id: "cont:0002".to_owned(),
            capability: Cap::RedactionEnforcement,
            continuity_posture: Posture::LocalCoreAuthoritative,
            available_offline: true,
            requires_provider_continuity: false,
            requires_admin_continuity: false,
            local_work_preserved: true,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary: "Redaction enforcement runs in the local core and works offline".to_owned(),
            record_ref: "continuity:redaction-enforcement".to_owned(),
            handoff: desktop_handoff("handoff:cont:0002", false),
        },
        LocalCoreContinuityItem {
            item_id: "cont:0003".to_owned(),
            capability: Cap::OfflinePacketReplay,
            continuity_posture: Posture::LocalCoreContinuesDegraded,
            available_offline: true,
            requires_provider_continuity: false,
            requires_admin_continuity: false,
            local_work_preserved: true,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Offline packet replay continues from the local core in a degraded but usable mode"
                    .to_owned(),
            record_ref: "continuity:offline-packet-replay".to_owned(),
            handoff: desktop_handoff("handoff:cont:0003", false),
        },
        LocalCoreContinuityItem {
            item_id: "cont:0004".to_owned(),
            capability: Cap::SupportExportAssembly,
            continuity_posture: Posture::RequiresProviderContinuity,
            available_offline: false,
            requires_provider_continuity: true,
            requires_admin_continuity: false,
            local_work_preserved: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary:
                "Provider-assembled support-export pieces require provider continuity; labeled; local work retained"
                    .to_owned(),
            record_ref: "continuity:support-export-assembly".to_owned(),
            handoff: desktop_handoff("handoff:cont:0004", false),
        },
    ]
}

/// Canonical offline incident-packet items.
pub fn canonical_incident_packets() -> Vec<OfflineIncidentPacketItem> {
    use CompanionFreshnessState as Fresh;
    use IncidentPacketClass as Class;
    use OfflinePacketAvailability as Avail;
    use PacketCompleteness as Complete;
    use RedactionClass as Redact;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        OfflineIncidentPacketItem {
            item_id: "inc:0001".to_owned(),
            packet_class: Class::EvidenceTimeline,
            availability: Avail::LocalReady,
            completeness: Complete::CompleteVerified,
            claim_verified: true,
            proof_label_shown: false,
            redaction_class: Redact::MetadataOnly,
            redaction_verified: true,
            redaction_label_shown: false,
            attribution_present: true,
            attribution_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Incident evidence timeline assembled locally now; complete, verified; attributable; redaction verified"
                    .to_owned(),
            record_ref: "incident-packet:evidence-timeline".to_owned(),
            handoff: desktop_handoff("handoff:inc:0001", false),
        },
        OfflineIncidentPacketItem {
            item_id: "inc:0002".to_owned(),
            packet_class: Class::RunbookExecution,
            availability: Avail::LocalStaging,
            completeness: Complete::CompleteVerified,
            claim_verified: true,
            proof_label_shown: false,
            redaction_class: Redact::ReferenceOnly,
            redaction_verified: true,
            redaction_label_shown: false,
            attribution_present: true,
            attribution_label_shown: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Runbook execution packet staging locally from the local core; complete, verified; attributable"
                    .to_owned(),
            record_ref: "incident-packet:runbook-execution".to_owned(),
            handoff: desktop_handoff("handoff:inc:0002", false),
        },
        OfflineIncidentPacketItem {
            item_id: "inc:0003".to_owned(),
            packet_class: Class::IncidentExportBundle,
            availability: Avail::RequiresProviderAssembly,
            completeness: Complete::CompleteUnverified,
            claim_verified: false,
            proof_label_shown: true,
            redaction_class: Redact::ReferenceOnly,
            redaction_verified: false,
            redaction_label_shown: true,
            attribution_present: false,
            attribution_label_shown: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary:
                "Provider-assembled incident export bundle; completeness, redaction, and attribution not yet verified; all labeled"
                    .to_owned(),
            record_ref: "incident-packet:export-bundle".to_owned(),
            handoff: desktop_handoff("handoff:inc:0003", false),
        },
    ]
}

/// Canonical offline support-packet items.
pub fn canonical_support_packets() -> Vec<OfflineSupportPacketItem> {
    use CompanionFreshnessState as Fresh;
    use OfflinePacketAvailability as Avail;
    use PacketCompleteness as Complete;
    use RedactionClass as Redact;
    use SupportPacketClass as Class;

    let scope = CompanionReadWriteScope::ReadOnly;
    vec![
        OfflineSupportPacketItem {
            item_id: "supp:0001".to_owned(),
            packet_class: Class::DiagnosticsBundle,
            availability: Avail::LocalReady,
            completeness: Complete::CompleteVerified,
            claim_verified: true,
            proof_label_shown: false,
            redaction_class: Redact::MetadataOnly,
            redaction_verified: true,
            redaction_label_shown: false,
            freshness: Fresh::Live,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Diagnostics bundle assembled locally now; complete, verified; redaction verified"
                    .to_owned(),
            record_ref: "support-packet:diagnostics-bundle".to_owned(),
            handoff: desktop_handoff("handoff:supp:0001", false),
        },
        OfflineSupportPacketItem {
            item_id: "supp:0002".to_owned(),
            packet_class: Class::ProofPacketExport,
            availability: Avail::LocalStaging,
            completeness: Complete::CompleteVerified,
            claim_verified: true,
            proof_label_shown: false,
            redaction_class: Redact::ReferenceOnly,
            redaction_verified: true,
            redaction_label_shown: false,
            freshness: Fresh::Cached,
            read_write_scope: scope,
            stale_label_shown: false,
            summary:
                "Proof-packet export staging locally from the local core; complete, verified; redaction verified"
                    .to_owned(),
            record_ref: "support-packet:proof-packet-export".to_owned(),
            handoff: desktop_handoff("handoff:supp:0002", false),
        },
        OfflineSupportPacketItem {
            item_id: "supp:0003".to_owned(),
            packet_class: Class::SessionDiagnostics,
            availability: Avail::RequiresProviderAssembly,
            completeness: Complete::CompleteUnverified,
            claim_verified: false,
            proof_label_shown: true,
            redaction_class: Redact::ReferenceOnly,
            redaction_verified: false,
            redaction_label_shown: true,
            freshness: Fresh::Unknown,
            read_write_scope: scope,
            stale_label_shown: true,
            summary:
                "Provider-assembled session diagnostics; completeness and redaction not yet verified; labeled"
                    .to_owned(),
            record_ref: "support-packet:session-diagnostics".to_owned(),
            handoff: desktop_handoff("handoff:supp:0003", false),
        },
    ]
}

/// Builds the canonical surface packet.
///
/// This is the first consumer: it mints the surface the checked-in support export and
/// Markdown summary are generated from, so the artifact never drifts from the typed section,
/// item, scope, honesty, continuity, and freshness definitions.
pub fn canonical_redaction_continuity_surface(
    packet_id: String,
    surface_label: String,
    minted_at: String,
    proof_freshness: RedactionProofFreshness,
) -> RedactionContinuitySurfacePacket {
    RedactionContinuitySurfacePacket::new(RedactionContinuitySurfacePacketInput {
        packet_id,
        surface_label,
        projected_surfaces: vec![
            M5CompanionConsumerSurface::DesktopCompanionPanel,
            M5CompanionConsumerSurface::IncidentWorkspace,
            M5CompanionConsumerSurface::CliHeadless,
            M5CompanionConsumerSurface::SupportExport,
            M5CompanionConsumerSurface::Diagnostics,
            M5CompanionConsumerSurface::HelpAbout,
        ],
        section_qualifications: canonical_section_qualifications(),
        redaction_policy: canonical_redaction_policy(),
        continuity_rows: canonical_continuity_rows(),
        incident_packets: canonical_incident_packets(),
        support_packets: canonical_support_packets(),
        scope_contract: canonical_scope_contract(),
        honesty_contract: canonical_honesty_contract(),
        stale_state_honesty: canonical_stale_state_honesty(),
        continuity_contract: canonical_continuity_contract(),
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
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        REDACTION_CONTINUITY_SCHEMA_REF,
        REDACTION_CONTINUITY_DOC_REF,
        M5_COMPANION_SURFACE_CONTRACT_REF,
        M5_INCIDENT_WORKSPACE_CONTRACT_REF,
        M5_OFFBOARDING_CONTRACT_REF,
        M5_COMPANION_MATRIX_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(RedactionViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_section_qualifications(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let present: BTreeSet<RedactionContinuitySection> = packet
        .section_qualifications
        .iter()
        .map(|row| row.section)
        .collect();
    for required in RedactionContinuitySection::ALL {
        if !present.contains(&required) {
            violations.push(RedactionViolation::RequiredSectionMissing);
            return;
        }
    }

    for row in &packet.section_qualifications {
        if row.matrix_lane_ref != row.section.matrix_lane().as_str() {
            violations.push(RedactionViolation::SectionLaneMismatch);
        }
        if row.read_write_scope != row.section.bounded_scope() {
            violations.push(RedactionViolation::SectionScopeMismatch);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(RedactionViolation::SectionRowIncomplete);
        }
    }
}

fn validate_items(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    if packet.redaction_policy.is_empty()
        || packet.continuity_rows.is_empty()
        || packet.incident_packets.is_empty()
        || packet.support_packets.is_empty()
    {
        violations.push(RedactionViolation::SectionContentMissing);
    }

    if !packet.incident_packet_local_path_available() {
        violations.push(RedactionViolation::LocalIncidentPacketPathMissing);
    }
    if !packet.support_packet_local_path_available() {
        violations.push(RedactionViolation::LocalSupportPacketPathMissing);
    }

    for item in &packet.redaction_policy {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(RedactionViolation::ReadOnlyScopeViolated);
        }
        if !item.no_payload_body {
            violations.push(RedactionViolation::PayloadBodyPresent);
        }
        if !item.redaction_verified && !item.redaction_label_shown {
            violations.push(RedactionViolation::RedactionClaimNotLabeled);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(RedactionViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.continuity_rows {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(RedactionViolation::ReadOnlyScopeViolated);
        }
        if item.continuity_posture.continues_offline() && !item.available_offline {
            violations.push(RedactionViolation::ContinuityOfflineMismatch);
        }
        if item.requires_provider_continuity != item.continuity_posture.requires_provider()
            || item.requires_admin_continuity != item.continuity_posture.requires_admin()
        {
            violations.push(RedactionViolation::ContinuityFlagMismatch);
        }
        if !item.local_work_preserved {
            violations.push(RedactionViolation::LocalWorkStranded);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(RedactionViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.incident_packets {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(RedactionViolation::ReadOnlyScopeViolated);
        }
        if item.completeness.is_complete_claim() && !item.claim_verified {
            violations.push(RedactionViolation::CompletenessClaimedButUnverified);
        }
        if item.completeness == PacketCompleteness::CompleteUnverified && !item.proof_label_shown {
            violations.push(RedactionViolation::CompletenessClaimNotLabeled);
        }
        if !item.redaction_verified && !item.redaction_label_shown {
            violations.push(RedactionViolation::RedactionClaimNotLabeled);
        }
        if !item.attribution_present && !item.attribution_label_shown {
            violations.push(RedactionViolation::IncidentAttributionNotLabeled);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(RedactionViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }

    for item in &packet.support_packets {
        if item.read_write_scope != CompanionReadWriteScope::ReadOnly {
            violations.push(RedactionViolation::ReadOnlyScopeViolated);
        }
        if item.completeness.is_complete_claim() && !item.claim_verified {
            violations.push(RedactionViolation::CompletenessClaimedButUnverified);
        }
        if item.completeness == PacketCompleteness::CompleteUnverified && !item.proof_label_shown {
            violations.push(RedactionViolation::CompletenessClaimNotLabeled);
        }
        if !item.redaction_verified && !item.redaction_label_shown {
            violations.push(RedactionViolation::RedactionClaimNotLabeled);
        }
        if item.item_id.trim().is_empty()
            || item.summary.trim().is_empty()
            || item.record_ref.trim().is_empty()
        {
            violations.push(RedactionViolation::ItemIncomplete);
        }
        validate_freshness_label(item.freshness, item.stale_label_shown, violations);
        validate_handoff(&item.handoff, violations);
    }
}

fn validate_freshness_label(
    freshness: CompanionFreshnessState,
    stale_label_shown: bool,
    violations: &mut Vec<RedactionViolation>,
) {
    if freshness.requires_label() && !stale_label_shown {
        violations.push(RedactionViolation::StaleStateNotLabeled);
    }
}

fn validate_handoff(handoff: &CompanionDesktopHandoff, violations: &mut Vec<RedactionViolation>) {
    if handoff.deep_link_ref.trim().is_empty() {
        violations.push(RedactionViolation::HandoffRefMissing);
    }
}

fn validate_scope_contract(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let contract = &packet.scope_contract;
    for ok in [
        contract.redaction_policy_read_only,
        contract.continuity_read_only,
        contract.incident_packet_read_only,
        contract.support_packet_read_only,
        contract.local_core_authoritative,
        contract.action_applied_by_local_core_not_surface,
        contract.no_unbounded_workspace_write,
        contract.no_payload_bodies,
    ] {
        if !ok {
            violations.push(RedactionViolation::ScopeContractIncomplete);
            return;
        }
    }
}

fn validate_honesty_contract(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let contract = &packet.honesty_contract;
    for ok in [
        contract.companion_safe_redaction_enforced,
        contract.redaction_provable_or_labeled,
        contract.no_payload_body_crosses_boundary,
        contract.incident_packet_local_path_always_available,
        contract.support_packet_local_path_always_available,
        contract.packet_completeness_provable_or_labeled,
        contract.incident_packets_attributable_or_labeled,
        contract.no_claim_without_evidence,
    ] {
        if !ok {
            violations.push(RedactionViolation::HonestyContractIncomplete);
            return;
        }
    }
}

fn validate_stale_state_honesty(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let honesty = &packet.stale_state_honesty;
    for ok in [
        honesty.stale_items_labeled,
        honesty.unknown_freshness_labeled,
        honesty.never_show_stale_as_live,
        honesty.freshness_floor_enforced,
    ] {
        if !ok {
            violations.push(RedactionViolation::StaleStateHonestyIncomplete);
            return;
        }
    }
}

fn validate_continuity_contract(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let contract = &packet.continuity_contract;
    for ok in [
        contract.local_core_continues_when_provider_degrades,
        contract.capabilities_available_offline,
        contract.degraded_capability_labeled_not_hidden,
        contract.local_work_never_stranded,
        contract.provider_and_admin_continuity_disclosed,
    ] {
        if !ok {
            violations.push(RedactionViolation::ContinuityContractIncomplete);
            return;
        }
    }
}

fn validate_locality(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let locality = &packet.locality_disclosure;
    if locality.stays_local.trim().is_empty()
        || locality.staged.trim().is_empty()
        || locality
            .requires_provider_or_admin_continuity
            .trim()
            .is_empty()
    {
        violations.push(RedactionViolation::LocalityDisclosureIncomplete);
    }
}

fn validate_security_review(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let review = &packet.security_review;
    for ok in [
        review.redaction_policy_read_only,
        review.continuity_read_only,
        review.incident_packet_read_only,
        review.support_packet_read_only,
        review.local_core_authoritative,
        review.action_applied_by_local_core_not_surface,
        review.companion_safe_redaction_enforced,
        review.redaction_provable_or_labeled,
        review.no_payload_body_crosses_boundary,
        review.incident_packet_local_path_always_available,
        review.support_packet_local_path_always_available,
        review.packet_completeness_provable_or_labeled,
        review.incident_packets_attributable_or_labeled,
        review.capabilities_available_offline,
        review.local_work_never_stranded,
        review.stale_state_labeled_never_hidden,
        review.exact_desktop_handoff_preserved,
        review.no_credential_or_provider_bodies_in_export,
        review.no_payload_bodies_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.locality_disclosed,
    ] {
        if !ok {
            violations.push(RedactionViolation::SecurityReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.desktop_panel_shows_redaction_policy,
        projection.desktop_panel_shows_continuity,
        projection.incident_workspace_shows_offline_incident_packets,
        projection.diagnostics_shows_offline_support_packets,
        projection.cli_headless_shows_redaction_and_packet_state,
        projection.support_export_shows_redaction_and_packets,
        projection.help_about_shows_redaction_and_continuity_honesty,
        projection.preview_labs_label_for_unqualified_sections,
    ] {
        if !ok {
            violations.push(RedactionViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &RedactionContinuitySurfacePacket,
    violations: &mut Vec<RedactionViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(RedactionViolation::ProofFreshnessIncomplete);
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
