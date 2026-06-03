//! Typed register tracking freshness SLOs for release packet types, shiproom
//! dashboard stale-claim/stale-report alarms, and proof-index export format
//! with validity windows, stale reasons, downgrade-propagation status, and
//! consuming-surface sets for the M4 stable line.
//!
//! The [`stable_claim_manifest`](crate::stable_claim_manifest) decides the single
//! canonical lifecycle label each *subject* publishes. This module answers the
//! question: **for each freshness-governed object — a claim publication manifest,
//! reference workspace report, compatibility report, evaluation evidence pack,
//! pilot evidence pack, Help/About consumer, support export consumer, build packet,
//! benchmark packet, shiproom dashboard panel, or proof-index export object — is
//! that object actually backed by a fresh proof packet, an owner sign-off, and a
//! complete validity window, and is it narrowed below the cutline the moment its
//! backing thins out?** This module is the **freshness-SLO and proof-index export
//! register**. For every such object it records one row that binds the object to
//! the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose lifecycle
//! label it backs, the proof packet that grounds it, the waiver (if any) holding it
//! provisionally, and the owner sign-off.
//!
//! Each [`FreshnessObjectRow`] is one `(object, public claim)` binding. It:
//!
//! - names the object kind it governs ([`FreshnessObjectRow::object_kind`],
//!   [`FreshnessObjectRow::surface_ref`], [`FreshnessObjectRow::surface_summary`])
//!   and whether that object is part of the release-blocking set
//!   ([`FreshnessObjectRow::release_blocking`]);
//! - pins the upstream source register and row ([`FreshnessObjectRow::source_register_ref`],
//!   [`FreshnessObjectRow::source_row_ref`]);
//! - pins the proof packet ([`ProofPacket`]) with its packet-freshness SLO;
//! - records the validity window ([`ValidityWindow`]) that bounds the object's
//!   freshness claim;
//! - names the [`stable_claim_manifest`](crate::stable_claim_manifest) entry whose
//!   public claim it backs ([`FreshnessObjectRow::claim_ref`]) and the canonical
//!   lifecycle label that entry publishes ([`FreshnessObjectRow::claim_label`]).
//!   That label is a hard **ceiling**: an object may carry the claim's label or
//!   narrow below it, but it may never assert a public claim wider than the public
//!   claim it backs;
//! - reuses the [`StableClaimLevel`] vocabulary rather than minting per-object labels,
//!   so docs, Help/About, the release center, shiproom dashboards, procurement portals,
//!   and support exports ingest one label per object instead of cloning their own;
//! - records the object state earned ([`FreshnessObjectState`]), the active gap reasons
//!   ([`FreshnessObjectGapReason`]), and the label it *effectively* publishes after
//!   narrowing ([`FreshnessObjectRow::effective_label`]);
//! - tracks the downgrade-propagation status ([`DowngradePropagationStatus`]) so
//!   shiproom dashboards know whether a narrowed label has been pushed to all
//!   consuming surfaces;
//! - names the consuming surfaces ([`ConsumingSurface`]) that ingest this object's
//!   proof so a breach can be routed to the right renderer.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between an object whose backing supports a Stable public claim and one narrowed
//! below it. An object that is not backed — because its proof packet aged out or is
//! missing, because its evidence is incomplete, because its waiver expired, because
//! the public claim it backs is itself below the cutline, or because downgrade
//! propagation is pending — is structurally required to drop below the cutline rather
//! than inherit an adjacent backed object. The [`FreshnessObjectRule`] set names the
//! closed conditions that gate publication, and
//! [`FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport::publication`]
//! records the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support.json`
//! and embedded here, so this typed consumer and the CI gate agree on every row
//! without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, or credential material. Two
//! classes of check live outside this model because they need more than the register
//! sees: date arithmetic (recomputing the packet-freshness state and waiver expiry
//! against an `as_of` date) and the cross-artifact ceiling check (whether each row's
//! `claim_label` still equals the label the stable claim manifest publishes for the
//! entry named by `claim_ref`). Those live in the CI gate. This model enforces the
//! structural and logical invariants that hold regardless of the clock and the
//! neighbouring artifact — the ceiling/no-widening rule, narrowing consistency,
//! packet/state coherence, owner sign-off on backed rows, kind and release-line
//! coverage, publication-rule wiring, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, QualificationWaiver, StableClaimLevel,
};

/// Supported schema version.
pub const FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_RECORD_KIND: &str = "finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support";

/// Repo-relative path to the checked-in register.
pub const FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_PATH: &str =
    "artifacts/release/finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support.json";

/// Embedded checked-in register JSON.
pub const FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support.json"
));

/// The kind of freshness-governed object a row tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessObjectKind {
    /// The claim publication manifest.
    ClaimPublicationManifest,
    /// A reference workspace report.
    ReferenceWorkspaceReport,
    /// A compatibility report.
    CompatibilityReport,
    /// An evaluation evidence pack.
    EvaluationEvidencePack,
    /// A pilot evidence pack.
    PilotEvidencePack,
    /// A Help/About consumer.
    HelpAboutConsumer,
    /// A support export consumer.
    SupportExportConsumer,
    /// A build packet.
    BuildPacket,
    /// A benchmark packet.
    BenchmarkPacket,
    /// A shiproom dashboard panel.
    ShiproomDashboardPanel,
    /// A proof-index export object.
    ProofIndexExportObject,
}

impl FreshnessObjectKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ClaimPublicationManifest,
        Self::ReferenceWorkspaceReport,
        Self::CompatibilityReport,
        Self::EvaluationEvidencePack,
        Self::PilotEvidencePack,
        Self::HelpAboutConsumer,
        Self::SupportExportConsumer,
        Self::BuildPacket,
        Self::BenchmarkPacket,
        Self::ShiproomDashboardPanel,
        Self::ProofIndexExportObject,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimPublicationManifest => "claim_publication_manifest",
            Self::ReferenceWorkspaceReport => "reference_workspace_report",
            Self::CompatibilityReport => "compatibility_report",
            Self::EvaluationEvidencePack => "evaluation_evidence_pack",
            Self::PilotEvidencePack => "pilot_evidence_pack",
            Self::HelpAboutConsumer => "help_about_consumer",
            Self::SupportExportConsumer => "support_export_consumer",
            Self::BuildPacket => "build_packet",
            Self::BenchmarkPacket => "benchmark_packet",
            Self::ShiproomDashboardPanel => "shiproom_dashboard_panel",
            Self::ProofIndexExportObject => "proof_index_export_object",
        }
    }
}

/// State a row earned for the public claim it backs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessObjectState {
    /// The object is backed: a captured, within-SLO proof packet backs the public
    /// claim at its full canonical lifecycle label, owner-signed.
    Current,
    /// The object carries the claim's full label only because an active, unexpired
    /// waiver covers a recorded gap.
    CurrentOnWaiver,
    /// The proof packet or row evidence is incomplete, or owner sign-off is absent;
    /// the object is not backed and the label must narrow.
    NarrowedUnbacked,
    /// The public claim this object backs is itself below the cutline, so the object
    /// inherits that ceiling and narrows.
    NarrowedClaimNarrowed,
    /// The proof packet breached its freshness SLO (or is missing); the object is
    /// not backed and the label must narrow.
    NarrowedStale,
    /// The object relied on a waiver that has expired; the label must narrow.
    NarrowedWaiverExpired,
    /// Downgrade propagation is pending; the label is narrowed until propagation
    /// completes.
    NarrowedDowngradePending,
}

impl FreshnessObjectState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Current,
        Self::CurrentOnWaiver,
        Self::NarrowedUnbacked,
        Self::NarrowedClaimNarrowed,
        Self::NarrowedStale,
        Self::NarrowedWaiverExpired,
        Self::NarrowedDowngradePending,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::CurrentOnWaiver => "current_on_waiver",
            Self::NarrowedUnbacked => "narrowed_unbacked",
            Self::NarrowedClaimNarrowed => "narrowed_claim_narrowed",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedWaiverExpired => "narrowed_waiver_expired",
            Self::NarrowedDowngradePending => "narrowed_downgrade_pending",
        }
    }

    /// Whether the state lets an object carry the public claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Current | Self::CurrentOnWaiver)
    }

    /// Whether the state forces the object below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason an object narrows or a rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessObjectGapReason {
    /// The public claim this object backs is itself below the cutline.
    ClaimLabelNarrowed,
    /// The object names a capability the build does not yet implement.
    ObjectCapabilityAbsent,
    /// The proof packet's row-level evidence is incomplete.
    EvidenceIncomplete,
    /// The proof packet breached its freshness SLO.
    ProofPacketFreshnessBreached,
    /// No proof packet has been captured for the object.
    ProofPacketMissing,
    /// A waiver the object relied on has expired.
    WaiverExpired,
    /// The required owner sign-off is missing.
    OwnerSignoffMissing,
    /// Downgrade propagation is pending.
    DowngradePropagationPending,
    /// A shiproom dashboard stale-claim alarm is active.
    StaleClaimAlarm,
    /// A shiproom dashboard stale-report alarm is active.
    StaleReportAlarm,
}

impl FreshnessObjectGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ClaimLabelNarrowed,
        Self::ObjectCapabilityAbsent,
        Self::EvidenceIncomplete,
        Self::ProofPacketFreshnessBreached,
        Self::ProofPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
        Self::DowngradePropagationPending,
        Self::StaleClaimAlarm,
        Self::StaleReportAlarm,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimLabelNarrowed => "claim_label_narrowed",
            Self::ObjectCapabilityAbsent => "object_capability_absent",
            Self::EvidenceIncomplete => "evidence_incomplete",
            Self::ProofPacketFreshnessBreached => "proof_packet_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::DowngradePropagationPending => "downgrade_propagation_pending",
            Self::StaleClaimAlarm => "stale_claim_alarm",
            Self::StaleReportAlarm => "stale_report_alarm",
        }
    }
}

/// Default action a rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessObjectAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the object's published lifecycle label below the cutline.
    NarrowObjectLabel,
    /// Refresh the proof packet so it re-enters its freshness SLO.
    RefreshProofPacket,
    /// Recapture the row-level evidence the proof packet depends on.
    RecaptureEvidence,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
    /// Propagate the downgrade to all consuming surfaces.
    PropagateDowngrade,
}

impl FreshnessObjectAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HoldPublication,
        Self::NarrowObjectLabel,
        Self::RefreshProofPacket,
        Self::RecaptureEvidence,
        Self::RequestOwnerSignoff,
        Self::PropagateDowngrade,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowObjectLabel => "narrow_object_label",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::PropagateDowngrade => "propagate_downgrade",
        }
    }
}

/// Downgrade propagation status for a narrowed object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradePropagationStatus {
    /// No downgrade is required for this object.
    NotRequired,
    /// The downgrade has been propagated to all consuming surfaces.
    Propagated,
    /// The downgrade is pending propagation.
    Pending,
    /// Propagation is blocked on a consuming surface.
    Blocked,
}

impl DowngradePropagationStatus {
    /// Every status, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NotRequired,
        Self::Propagated,
        Self::Pending,
        Self::Blocked,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Propagated => "propagated",
            Self::Pending => "pending",
            Self::Blocked => "blocked",
        }
    }
}

/// Surfaces that consume the proof object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumingSurface {
    /// The docs site.
    DocsSite,
    /// Help/About.
    HelpAbout,
    /// Service health.
    ServiceHealth,
    /// Support export.
    SupportExport,
    /// The release packet.
    ReleasePacket,
    /// The shiproom dashboard.
    ShiproomDashboard,
    /// The procurement portal.
    ProcurementPortal,
}

impl ConsumingSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DocsSite,
        Self::HelpAbout,
        Self::ServiceHealth,
        Self::SupportExport,
        Self::ReleasePacket,
        Self::ShiproomDashboard,
        Self::ProcurementPortal,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSite => "docs_site",
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
            Self::SupportExport => "support_export",
            Self::ReleasePacket => "release_packet",
            Self::ShiproomDashboard => "shiproom_dashboard",
            Self::ProcurementPortal => "procurement_portal",
        }
    }
}

/// Validity window bounding a freshness object's claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidityWindow {
    /// UTC date the packet was captured.
    pub captured_at: String,
    /// UTC date the window expires.
    pub expires_at: String,
    /// UTC date after which the object is considered stale.
    pub stale_after: String,
}

/// One rule: a closed condition that narrows an object label and may gate publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessObjectRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched row fires this rule.
    pub trigger_reason: FreshnessObjectGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: FreshnessObjectAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One register row: an `(object, public claim)` binding bound to its proof packet,
/// canonical ceiling label, packet-freshness SLO, validity window, and consuming surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessObjectRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The object kind this row governs.
    pub object_kind: FreshnessObjectKind,
    /// Ref to the upstream source register this row ingests from.
    pub source_register_ref: String,
    /// Ref to the upstream source row this row ingests from.
    pub source_row_ref: String,
    /// The surface id this object speaks about.
    pub surface_ref: String,
    /// Reviewable one-line statement of the object.
    pub surface_summary: String,
    /// Whether the object is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this object backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes. The ceiling: an
    /// object may never carry a label wider than this.
    pub claim_label: StableClaimLevel,
    /// State earned for the object.
    pub object_state: FreshnessObjectState,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// The validity window bounding this object's freshness claim.
    pub validity_window: ValidityWindow,
    /// Reason the object is stale, when applicable.
    #[serde(default)]
    pub stale_reason: Option<String>,
    /// Downgrade propagation status for narrowed objects.
    pub downgrade_propagation_status: DowngradePropagationStatus,
    /// Surfaces that consume this proof object.
    #[serde(default)]
    pub consuming_surfaces: Vec<ConsumingSurface>,
    /// Waiver authorizing a provisional object, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<FreshnessObjectGapReason>,
    /// The lifecycle label the object effectively carries after narrowing.
    pub effective_label: StableClaimLevel,
    /// Publication destinations that render this row's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the row carries this posture.
    pub rationale: String,
}

impl FreshnessObjectRow {
    /// True when the effective label is at or above the cutline.
    pub fn holds_stable(&self) -> bool {
        self.effective_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the row's state lets the object carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.object_state.holds_label()
    }

    /// True when a gap reason is active on the row.
    pub fn has_active_reason(&self, reason: FreshnessObjectGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// Alias for [`holds_stable`](FreshnessObjectRow::holds_stable).
    pub fn proves_stable(&self) -> bool {
        self.holds_stable()
    }
}

/// The recorded publication verdict for the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessObjectPublicationRecord {
    /// The gate this verdict governs.
    pub publication_gate: String,
    /// Proceed or hold.
    pub decision: PromotionDecision,
    /// Rule ids that block publication, sorted.
    #[serde(default)]
    pub blocking_rule_ids: Vec<String>,
    /// Row ids that triggered a blocking rule, sorted.
    #[serde(default)]
    pub blocking_entry_ids: Vec<String>,
    /// Reviewable summary of the verdict.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessObjectSummary {
    /// Total number of rows.
    pub total_entries: usize,
    /// Distinct public claims covered.
    pub total_claims: usize,
    /// Rows whose effective label is at or above the cutline.
    pub entries_current_stable: usize,
    /// Rows narrowed below the cutline.
    pub entries_narrowed_below_cutline: usize,
    /// Rows holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Total release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows whose effective label is at or above the cutline.
    pub release_blocking_current_stable: usize,
    /// Release-blocking rows narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Claim publication manifest entries.
    pub claim_publication_manifest_entries: usize,
    /// Reference workspace report entries.
    pub reference_workspace_report_entries: usize,
    /// Compatibility report entries.
    pub compatibility_report_entries: usize,
    /// Evaluation evidence pack entries.
    pub evaluation_evidence_pack_entries: usize,
    /// Pilot evidence pack entries.
    pub pilot_evidence_pack_entries: usize,
    /// Help/About consumer entries.
    pub help_about_consumer_entries: usize,
    /// Support export consumer entries.
    pub support_export_consumer_entries: usize,
    /// Build packet entries.
    pub build_packet_entries: usize,
    /// Benchmark packet entries.
    pub benchmark_packet_entries: usize,
    /// Shiproom dashboard panel entries.
    pub shiproom_dashboard_panel_entries: usize,
    /// Proof-index export object entries.
    pub proof_index_export_object_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Downgrades propagated to all consuming surfaces.
    pub downgrade_propagated: usize,
    /// Downgrades pending propagation.
    pub downgrade_pending: usize,
    /// Downgrades blocked on a consuming surface.
    pub downgrade_blocked: usize,
    /// Rows consumed by the docs site.
    pub docs_site_consuming: usize,
    /// Rows consumed by Help/About.
    pub help_about_consuming: usize,
    /// Rows consumed by service health.
    pub service_health_consuming: usize,
    /// Rows consumed by support export.
    pub support_export_consuming: usize,
    /// Rows consumed by the release packet.
    pub release_packet_consuming: usize,
    /// Rows consumed by the shiproom dashboard.
    pub shiproom_dashboard_consuming: usize,
    /// Rows consumed by the procurement portal.
    pub procurement_portal_consuming: usize,
    /// Total active gap reasons across all rows.
    pub total_active_gap_reasons: usize,
    /// Number of rules currently firing.
    pub rules_firing: usize,
}

/// A redaction-safe export row projected from the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessObjectExportRow {
    /// Stable row id.
    pub entry_id: String,
    /// The object kind this row governs.
    pub object_kind: FreshnessObjectKind,
    /// Ref to the upstream source register.
    pub source_register_ref: String,
    /// Ref to the upstream source row.
    pub source_row_ref: String,
    /// The surface id this object speaks about.
    pub surface_ref: String,
    /// Whether the object is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose public claim this object backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The lifecycle label the object effectively carries after narrowing.
    pub effective_label: StableClaimLevel,
    /// Whether the object holds a stable label.
    pub holds_stable: bool,
    /// State earned for the object.
    pub object_state: FreshnessObjectState,
    /// Proof packet freshness SLO state.
    pub slo_state: FreshnessSloState,
    /// The validity window bounding this object's freshness claim.
    pub validity_window: ValidityWindow,
    /// Reason the object is stale, when applicable.
    #[serde(default)]
    pub stale_reason: Option<String>,
    /// Downgrade propagation status.
    pub downgrade_propagation_status: DowngradePropagationStatus,
    /// Surfaces that consume this proof object.
    #[serde(default)]
    pub consuming_surfaces: Vec<ConsumingSurface>,
    /// Active gap reasons narrowing the row.
    #[serde(default)]
    pub active_gap_reasons: Vec<FreshnessObjectGapReason>,
}

/// A redaction-safe export projection of the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessObjectExportProjection {
    /// Register id this projection was produced from.
    pub register_id: String,
    /// Register as-of date.
    pub as_of: String,
    /// Publication verdict.
    pub publication_decision: PromotionDecision,
    /// Projected rows.
    pub rows: Vec<FreshnessObjectExportRow>,
}

/// The typed register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport
{
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this register ingests as its public-claim
    /// source and ceiling.
    pub claim_manifest_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed object-kind vocabulary.
    pub object_kinds: Vec<FreshnessObjectKind>,
    /// Closed object-state vocabulary.
    pub object_states: Vec<FreshnessObjectState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<FreshnessObjectGapReason>,
    /// Closed action vocabulary.
    pub object_actions: Vec<FreshnessObjectAction>,
    /// Closed downgrade-propagation-status vocabulary.
    pub downgrade_propagation_statuses: Vec<DowngradePropagationStatus>,
    /// Closed consuming-surface vocabulary.
    pub consuming_surfaces: Vec<ConsumingSurface>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking surface refs this register must cover.
    pub release_blocking_surface_refs: Vec<String>,
    /// Publication rules.
    pub rules: Vec<FreshnessObjectRule>,
    /// Register rows.
    pub rows: Vec<FreshnessObjectRow>,
    /// Recorded publication verdict.
    pub publication: FreshnessObjectPublicationRecord,
    /// Summary counts.
    pub summary: FreshnessObjectSummary,
}

/// Parses the embedded checked-in register JSON.
pub fn current_finalize_release_packet_freshness_slos_shiproom_dashboards_and_proof_index_export_for_procurement_and_support(
) -> Result<
    FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport,
    serde_json::Error,
> {
    serde_json::from_str(FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_JSON)
}

impl
    FinalizeReleasePacketFreshnessSlosShiproomDashboardsAndProofIndexExportForProcurementAndSupport
{
    /// Returns the row registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&FreshnessObjectRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows whose effective label is at or above the cutline.
    pub fn rows_current_stable(&self) -> Vec<&FreshnessObjectRow> {
        self.rows.iter().filter(|row| row.holds_stable()).collect()
    }

    /// Returns the rows narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&FreshnessObjectRow> {
        self.rows.iter().filter(|row| !row.holds_stable()).collect()
    }

    /// Returns the release-blocking rows.
    pub fn release_blocking_rows(&self) -> Vec<&FreshnessObjectRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the rows for one object kind.
    pub fn rows_for_kind(&self, kind: FreshnessObjectKind) -> Vec<&FreshnessObjectRow> {
        self.rows
            .iter()
            .filter(|row| row.object_kind == kind)
            .collect()
    }

    /// Distinct public claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched row carries its trigger reason.
    pub fn rule_fires(&self, rule: &FreshnessObjectRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the rows and rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .rules
            .iter()
            .any(|rule| rule.blocks_publication && self.rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Row ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only rows whose public claim is at or above the cutline count: a row whose claim
    /// is already canonically narrowed is not a *register* blocker, it merely inherits
    /// the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<FreshnessObjectGapReason> = self
            .rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the rows and rules.
    pub fn computed_summary(&self) -> FreshnessObjectSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: FreshnessObjectKind| self.rows_for_kind(kind).len();
        let release_blocking: Vec<&FreshnessObjectRow> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect();
        let downgrade = |status: DowngradePropagationStatus| {
            self.rows
                .iter()
                .filter(|row| row.downgrade_propagation_status == status)
                .count()
        };
        let consumes = |surface: ConsumingSurface| {
            self.rows
                .iter()
                .filter(|row| row.consuming_surfaces.contains(&surface))
                .count()
        };
        FreshnessObjectSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_current_stable: self.rows.iter().filter(|row| row.holds_stable()).count(),
            entries_narrowed_below_cutline: self
                .rows
                .iter()
                .filter(|row| !row.holds_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.object_state == FreshnessObjectState::CurrentOnWaiver)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_current_stable: release_blocking
                .iter()
                .filter(|row| row.holds_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.holds_stable())
                .count(),
            claim_publication_manifest_entries: kind(FreshnessObjectKind::ClaimPublicationManifest),
            reference_workspace_report_entries: kind(FreshnessObjectKind::ReferenceWorkspaceReport),
            compatibility_report_entries: kind(FreshnessObjectKind::CompatibilityReport),
            evaluation_evidence_pack_entries: kind(FreshnessObjectKind::EvaluationEvidencePack),
            pilot_evidence_pack_entries: kind(FreshnessObjectKind::PilotEvidencePack),
            help_about_consumer_entries: kind(FreshnessObjectKind::HelpAboutConsumer),
            support_export_consumer_entries: kind(FreshnessObjectKind::SupportExportConsumer),
            build_packet_entries: kind(FreshnessObjectKind::BuildPacket),
            benchmark_packet_entries: kind(FreshnessObjectKind::BenchmarkPacket),
            shiproom_dashboard_panel_entries: kind(FreshnessObjectKind::ShiproomDashboardPanel),
            proof_index_export_object_entries: kind(FreshnessObjectKind::ProofIndexExportObject),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            downgrade_propagated: downgrade(DowngradePropagationStatus::Propagated),
            downgrade_pending: downgrade(DowngradePropagationStatus::Pending),
            downgrade_blocked: downgrade(DowngradePropagationStatus::Blocked),
            docs_site_consuming: consumes(ConsumingSurface::DocsSite),
            help_about_consuming: consumes(ConsumingSurface::HelpAbout),
            service_health_consuming: consumes(ConsumingSurface::ServiceHealth),
            support_export_consuming: consumes(ConsumingSurface::SupportExport),
            release_packet_consuming: consumes(ConsumingSurface::ReleasePacket),
            shiproom_dashboard_consuming: consumes(ConsumingSurface::ShiproomDashboard),
            procurement_portal_consuming: consumes(ConsumingSurface::ProcurementPortal),
            total_active_gap_reasons: self
                .rows
                .iter()
                .map(|row| row.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .rules
                .iter()
                .filter(|rule| self.rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection of the register that downstream
    /// surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> FreshnessObjectExportProjection {
        FreshnessObjectExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| FreshnessObjectExportRow {
                    entry_id: row.entry_id.clone(),
                    object_kind: row.object_kind,
                    source_register_ref: row.source_register_ref.clone(),
                    source_row_ref: row.source_row_ref.clone(),
                    surface_ref: row.surface_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    effective_label: row.effective_label,
                    holds_stable: row.holds_stable(),
                    object_state: row.object_state,
                    slo_state: row.proof_packet.slo_state,
                    validity_window: row.validity_window.clone(),
                    stale_reason: row.stale_reason.clone(),
                    downgrade_propagation_status: row.downgrade_propagation_status,
                    consuming_surfaces: row.consuming_surfaces.clone(),
                    active_gap_reasons: row.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<FreshnessObjectViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(FreshnessObjectViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(FreshnessObjectViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(FreshnessObjectViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<FreshnessObjectViolation>) {
        if self.schema_version != FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_SCHEMA_VERSION {
            violations.push(FreshnessObjectViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != FINALIZE_RELEASE_PACKET_FRESHNESS_SLOS_SHIPROOM_DASHBOARDS_AND_PROOF_INDEX_EXPORT_FOR_PROCUREMENT_AND_SUPPORT_RECORD_KIND {
            violations.push(FreshnessObjectViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(FreshnessObjectViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.object_kinds != FreshnessObjectKind::ALL.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "object_kinds",
            });
        }
        if self.object_states != FreshnessObjectState::ALL.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "object_states",
            });
        }
        if self.gap_reasons != FreshnessObjectGapReason::ALL.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.object_actions != FreshnessObjectAction::ALL.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "object_actions",
            });
        }
        if self.downgrade_propagation_statuses != DowngradePropagationStatus::ALL.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "downgrade_propagation_statuses",
            });
        }
        if self.consuming_surfaces != ConsumingSurface::ALL.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "consuming_surfaces",
            });
        }
        if self.release_blocking_surface_refs.is_empty() {
            violations.push(FreshnessObjectViolation::EmptyField {
                entry_id: "<register>".to_owned(),
                field_name: "release_blocking_surface_refs",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(FreshnessObjectViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(FreshnessObjectViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_rules(&self, violations: &mut Vec<FreshnessObjectViolation>) {
        if self.rules.is_empty() {
            violations.push(FreshnessObjectViolation::NoRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(FreshnessObjectViolation::DuplicateRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(FreshnessObjectViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(FreshnessObjectViolation::RuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in FreshnessObjectGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(FreshnessObjectViolation::GapReasonWithoutRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &FreshnessObjectRow,
        violations: &mut Vec<FreshnessObjectViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("source_register_ref", &row.source_register_ref),
            ("source_row_ref", &row.source_row_ref),
            ("surface_ref", &row.surface_ref),
            ("surface_summary", &row.surface_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            (
                "validity_window.captured_at",
                &row.validity_window.captured_at,
            ),
            (
                "validity_window.expires_at",
                &row.validity_window.expires_at,
            ),
            (
                "validity_window.stale_after",
                &row.validity_window.stale_after,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(FreshnessObjectViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        if row.effective_label.rank() > row.claim_label.rank() {
            violations.push(FreshnessObjectViolation::EffectiveWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                effective: row.effective_label,
            });
        }

        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(FreshnessObjectViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(FreshnessObjectViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(FreshnessObjectViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if !row.has_active_reason(FreshnessObjectGapReason::ClaimLabelNarrowed) {
                violations.push(FreshnessObjectViolation::ClaimNarrowedWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            if row.effective_label != row.claim_label {
                violations.push(FreshnessObjectViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    effective: row.effective_label,
                });
            }
            if !row.active_gap_reasons.is_empty() {
                violations.push(FreshnessObjectViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(FreshnessObjectViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(FreshnessObjectViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(FreshnessObjectViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            if row.holds_stable() {
                violations.push(FreshnessObjectViolation::EffectiveLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.object_state,
                    effective: row.effective_label,
                });
            }
            if row.active_gap_reasons.is_empty() {
                violations.push(FreshnessObjectViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.object_state,
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(FreshnessObjectGapReason::ProofPacketFreshnessBreached)
            {
                violations.push(FreshnessObjectViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(FreshnessObjectGapReason::ProofPacketMissing)
            {
                violations.push(FreshnessObjectViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        if row.has_active_reason(FreshnessObjectGapReason::StaleClaimAlarm)
            && !row.object_state.forces_narrowing()
        {
            violations.push(FreshnessObjectViolation::StaleClaimAlarmWithoutNarrowing {
                entry_id: row.entry_id.clone(),
            });
        }
        if row.has_active_reason(FreshnessObjectGapReason::StaleReportAlarm)
            && !row.object_state.forces_narrowing()
        {
            violations.push(FreshnessObjectViolation::StaleReportAlarmWithoutNarrowing {
                entry_id: row.entry_id.clone(),
            });
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_state_reason_coherence(
        &self,
        row: &FreshnessObjectRow,
        violations: &mut Vec<FreshnessObjectViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<FreshnessObjectViolation>,
                               expected: FreshnessObjectGapReason| {
            violations.push(FreshnessObjectViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.object_state,
                expected_reason: expected,
            });
        };

        match row.object_state {
            FreshnessObjectState::NarrowedUnbacked => {
                const ALLOWED: [FreshnessObjectGapReason; 3] = [
                    FreshnessObjectGapReason::ObjectCapabilityAbsent,
                    FreshnessObjectGapReason::EvidenceIncomplete,
                    FreshnessObjectGapReason::OwnerSignoffMissing,
                ];
                if !ALLOWED.iter().any(|r| row.has_active_reason(*r)) {
                    push_incoherent(violations, FreshnessObjectGapReason::EvidenceIncomplete);
                }
            }
            FreshnessObjectState::NarrowedClaimNarrowed => {
                if !row.has_active_reason(FreshnessObjectGapReason::ClaimLabelNarrowed) {
                    push_incoherent(violations, FreshnessObjectGapReason::ClaimLabelNarrowed);
                }
            }
            FreshnessObjectState::NarrowedStale => {
                if !(row.has_active_reason(FreshnessObjectGapReason::ProofPacketFreshnessBreached)
                    || row.has_active_reason(FreshnessObjectGapReason::ProofPacketMissing))
                {
                    push_incoherent(
                        violations,
                        FreshnessObjectGapReason::ProofPacketFreshnessBreached,
                    );
                }
            }
            FreshnessObjectState::NarrowedWaiverExpired => {
                if !row.has_active_reason(FreshnessObjectGapReason::WaiverExpired) {
                    push_incoherent(violations, FreshnessObjectGapReason::WaiverExpired);
                }
                if row.waiver.is_none() {
                    violations.push(FreshnessObjectViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.object_state,
                    });
                }
            }
            FreshnessObjectState::NarrowedDowngradePending => {
                if !row.has_active_reason(FreshnessObjectGapReason::DowngradePropagationPending) {
                    push_incoherent(
                        violations,
                        FreshnessObjectGapReason::DowngradePropagationPending,
                    );
                }
            }
            FreshnessObjectState::CurrentOnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(FreshnessObjectViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.object_state,
                    });
                }
            }
            FreshnessObjectState::Current => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<FreshnessObjectViolation>) {
        let declared: BTreeSet<&str> = self
            .release_blocking_surface_refs
            .iter()
            .map(String::as_str)
            .collect();
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .filter(|row| row.release_blocking)
            .map(|row| row.surface_ref.as_str())
            .collect();
        for declared_ref in &declared {
            if !covered.contains(declared_ref) {
                violations.push(FreshnessObjectViolation::LaunchBlockingRefWithoutRow {
                    requirement_ref: (*declared_ref).to_owned(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !declared.contains(row.surface_ref.as_str()) {
                violations.push(FreshnessObjectViolation::LaunchBlockingRowNotInSet {
                    entry_id: row.entry_id.clone(),
                    requirement_ref: row.surface_ref.clone(),
                });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<FreshnessObjectViolation>) {
        if self.publication.publication_gate.trim().is_empty() {
            violations.push(FreshnessObjectViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(FreshnessObjectViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(FreshnessObjectViolation::PublicationDecisionInconsistent {
                declared: self.publication.decision,
                computed,
            });
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(FreshnessObjectViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_entry_ids != self.computed_blocking_entry_ids() {
            violations.push(FreshnessObjectViolation::PublicationBlockingSetMismatch {
                field: "blocking_entry_ids",
            });
        }
    }
}

/// A validation violation for the register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FreshnessObjectViolation {
    /// The register carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// The register carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no rows.
    EmptyRegister,
    /// The register has no rules.
    NoRules,
    /// A required field is empty.
    EmptyField {
        /// Row or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate row id.
        entry_id: String,
    },
    /// A rule id appears more than once.
    DuplicateRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A rule names no labels to watch.
    RuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A gap reason has no rule watching for it.
    GapReasonWithoutRule {
        /// Uncovered reason.
        reason: FreshnessObjectGapReason,
    },
    /// An effective label is stronger than the claim's canonical label.
    EffectiveWiderThanClaim {
        /// Row id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A narrowing state did not drop the row below the cutline.
    EffectiveLabelNotNarrowed {
        /// Row id.
        entry_id: String,
        /// State.
        state: FreshnessObjectState,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A narrowing row carries no active gap reason.
    NarrowingWithoutReason {
        /// Row id.
        entry_id: String,
        /// State.
        state: FreshnessObjectState,
    },
    /// A held row carries an active gap reason.
    HeldWithActiveGap {
        /// Row id.
        entry_id: String,
    },
    /// A held row does not have a captured within-SLO proof packet.
    HeldWithoutFreshPacket {
        /// Row id.
        entry_id: String,
    },
    /// A held row rides a stale proof packet.
    HeldOnStalePacket {
        /// Row id.
        entry_id: String,
        /// SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held row's effective label does not match its claim label.
    HeldLabelNotEqualClaim {
        /// Row id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
        /// Effective label.
        effective: StableClaimLevel,
    },
    /// A held row lacks owner sign-off.
    HeldWithoutSignoff {
        /// Row id.
        entry_id: String,
    },
    /// A row holds a label on a narrowed claim.
    HeldOnNarrowedClaim {
        /// Row id.
        entry_id: String,
        /// Claim label.
        claim: StableClaimLevel,
    },
    /// A narrowed claim row does not name `claim_label_narrowed`.
    ClaimNarrowedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A breached packet does not name `proof_packet_freshness_breached`.
    BreachedPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A missing packet does not name `proof_packet_missing`.
    MissingPacketWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// A state and its active gap reasons are incoherent.
    StateReasonIncoherent {
        /// Row id.
        entry_id: String,
        /// State.
        state: FreshnessObjectState,
        /// Expected reason.
        expected_reason: FreshnessObjectGapReason,
    },
    /// A waiver state lacks a waiver.
    WaiverStateWithoutWaiver {
        /// Row id.
        entry_id: String,
        /// State.
        state: FreshnessObjectState,
    },
    /// A declared launch-blocking surface ref has no covering row.
    LaunchBlockingRefWithoutRow {
        /// Surface ref.
        requirement_ref: String,
    },
    /// A launch-blocking row is not in the declared set.
    LaunchBlockingRowNotInSet {
        /// Row id.
        entry_id: String,
        /// Surface ref.
        requirement_ref: String,
    },
    /// The publication decision does not match the computed decision.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// A blocking set does not match the computed set.
    PublicationBlockingSetMismatch {
        /// Field name.
        field: &'static str,
    },
    /// The summary does not match the computed summary.
    SummaryMismatch,
    /// A stale-claim alarm is active but the row is not narrowed.
    StaleClaimAlarmWithoutNarrowing {
        /// Row id.
        entry_id: String,
    },
    /// A stale-report alarm is active but the row is not narrowed.
    StaleReportAlarmWithoutNarrowing {
        /// Row id.
        entry_id: String,
    },
    /// The freshness SLO target/warn window is inconsistent.
    FreshnessSloInconsistent {
        /// Row id.
        entry_id: String,
    },
}

impl fmt::Display for FreshnessObjectViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported schema version: {}", actual)
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported record kind: {}", actual)
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "closed vocabulary mismatch on {}", field)
            }
            Self::EmptyRegister => write!(f, "register has no rows"),
            Self::NoRules => write!(f, "register has no rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => {
                write!(f, "empty field '{}' in {}", field_name, entry_id)
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id: {}", entry_id)
            }
            Self::DuplicateRuleId { rule_id } => {
                write!(f, "duplicate rule id: {}", rule_id)
            }
            Self::RuleWithoutLabels { rule_id } => {
                write!(f, "rule '{}' has no labels", rule_id)
            }
            Self::GapReasonWithoutRule { reason } => {
                write!(f, "gap reason '{}' has no rule", reason.as_str())
            }
            Self::EffectiveWiderThanClaim {
                entry_id,
                claim,
                effective,
            } => {
                write!(
                    f,
                    "entry '{}' effective '{}' is wider than claim '{}'",
                    entry_id,
                    effective.as_str(),
                    claim.as_str()
                )
            }
            Self::EffectiveLabelNotNarrowed {
                entry_id,
                state,
                effective,
            } => {
                write!(
                    f,
                    "entry '{}' in state '{}' should be narrowed but is '{}'",
                    entry_id,
                    state.as_str(),
                    effective.as_str()
                )
            }
            Self::NarrowingWithoutReason { entry_id, state } => {
                write!(
                    f,
                    "entry '{}' in state '{}' narrows without a reason",
                    entry_id,
                    state.as_str()
                )
            }
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "entry '{}' is held but has an active gap", entry_id)
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "entry '{}' is held without a fresh packet", entry_id)
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => {
                write!(
                    f,
                    "entry '{}' is held on a stale packet ({})",
                    entry_id,
                    slo_state.as_str()
                )
            }
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                effective,
            } => {
                write!(
                    f,
                    "entry '{}' is held but effective '{}' != claim '{}'",
                    entry_id,
                    effective.as_str(),
                    claim.as_str()
                )
            }
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "entry '{}' is held without owner sign-off", entry_id)
            }
            Self::HeldOnNarrowedClaim { entry_id, claim } => {
                write!(
                    f,
                    "entry '{}' is held on a narrowed claim ({})",
                    entry_id,
                    claim.as_str()
                )
            }
            Self::ClaimNarrowedWithoutReason { entry_id } => {
                write!(
                    f,
                    "entry '{}' inherits a narrowed claim without recording the reason",
                    entry_id
                )
            }
            Self::BreachedPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "entry '{}' has a breached packet without recording the reason",
                    entry_id
                )
            }
            Self::MissingPacketWithoutReason { entry_id } => {
                write!(
                    f,
                    "entry '{}' has a missing packet without recording the reason",
                    entry_id
                )
            }
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => {
                write!(
                    f,
                    "entry '{}' in state '{}' is incoherent (expected '{}')",
                    entry_id,
                    state.as_str(),
                    expected_reason.as_str()
                )
            }
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(
                    f,
                    "entry '{}' in state '{}' lacks a waiver",
                    entry_id,
                    state.as_str()
                )
            }
            Self::LaunchBlockingRefWithoutRow { requirement_ref } => {
                write!(
                    f,
                    "launch-blocking surface '{}' has no covering row",
                    requirement_ref
                )
            }
            Self::LaunchBlockingRowNotInSet {
                entry_id,
                requirement_ref,
            } => {
                write!(
                    f,
                    "launch-blocking row '{}' (surface '{}') is not in the declared set",
                    entry_id, requirement_ref
                )
            }
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication decision '{}' does not match computed '{}'",
                    declared.as_str(),
                    computed.as_str()
                )
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication blocking set mismatch on {}", field)
            }
            Self::SummaryMismatch => write!(f, "summary does not match computed summary"),
            Self::StaleClaimAlarmWithoutNarrowing { entry_id } => {
                write!(
                    f,
                    "entry '{}' has a stale-claim alarm but is not narrowed",
                    entry_id
                )
            }
            Self::StaleReportAlarmWithoutNarrowing { entry_id } => {
                write!(
                    f,
                    "entry '{}' has a stale-report alarm but is not narrowed",
                    entry_id
                )
            }
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "entry '{}' has an inconsistent freshness SLO", entry_id)
            }
        }
    }
}

impl Error for FreshnessObjectViolation {}
