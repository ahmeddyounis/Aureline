//! Delivery-grade display clustering for M5 findings that keeps every clustered
//! row source-preserving: distinct origins are grouped for ergonomics, never
//! flattened into one synthetic finding.
//!
//! The normalization step in
//! [`crate::normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins`]
//! binds each M5 finding to one canonical
//! [`DiagnosticRecord`](crate::diagnostics::DiagnosticRecord). M5 surfaces also
//! need an *ergonomic* view: when several findings describe the same underlying
//! issue — the same line flagged by a language service, an imported scanner, and a
//! build task — users want one compact row, not three. The thin
//! [`crate::diagnostics::UnifiedDiagnosticCluster`] already preserves the source /
//! freshness / remap *label sets* of a cluster; this module takes the next step
//! and makes the cluster *recoverable and exportable* across Problems, review,
//! CLI/headless, AI evidence, and support export.
//!
//! A [`DiagnosticDisplayCluster`] carries the display facts a compact row needs —
//! a stable [`DiagnosticDisplayCluster::cluster_id`], a primary anchor, the
//! contributing diagnostic refs, a typed dedupe reason
//! ([`DiagnosticClusterMeaningClass`]), [`DiagnosticClusterAggregateCounts`], and a
//! [`DiagnosticClusterDominantDisplayState`] — *plus* one
//! [`DiagnosticClusterMemberDetailSheet`] per constituent. The detail sheet
//! preserves each member's provenance, target / environment ref, policy state
//! (support posture, suppression / baseline joins, redaction class), and
//! imported-versus-live class, so the convenience of clustering never erases the
//! distinct facts a user needs to debug and trust the finding.
//!
//! The three guarantees this delivery owns:
//!
//! 1. **Cluster without flattening.** Different sources reporting similar text can
//!    share one display row, but [`DiagnosticDisplayCluster::synthetic_finding`]
//!    stays `false` and every contributing id keeps its own detail sheet — the
//!    cluster is a view over real records, not a new synthetic finding.
//! 2. **Recover every constituent.** A cluster detail sheet lets a user recover
//!    every contributing record, source descriptor, and environment reference; the
//!    cluster never drops a member's source kind, origin class, freshness, or remap
//!    state from its preserved label sets.
//! 3. **Export the meaning and the members.** Problems, review, support export, and
//!    AI evidence each receive a [`DiagnosticClusterSurfaceProjection`] that
//!    exposes the dedupe reason and the full membership, and the
//!    [`DiagnosticClusterSupportExport`] preserves both the cluster meaning and the
//!    constituent diagnostic ids rather than serializing a lossy display-only row.
//!
//! [`DiagnosticClusterSetPacket::validate`] refuses a packet that flattens unlike
//! sources into a synthetic finding, drops a member's provenance, cannot recover a
//! constituent from its detail sheet, lets a cluster's aggregate counts or dominant
//! display state disagree with its members, hides the dedupe reason or membership
//! from a required surface, or serializes a lossy support export.
//!
//! Raw source bytes, raw provider payloads, raw scanner reports, credentials, and
//! raw artifact bodies never cross this boundary; the packet carries only typed
//! class tokens, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/quality/diagnostic-cluster.schema.json`](../../../../schemas/quality/diagnostic-cluster.schema.json).
//! The reviewer-facing doc is
//! [`docs/m5/diagnostic-clusters-and-dedupe.md`](../../../../docs/m5/diagnostic-clusters-and-dedupe.md).
//! The protected fixture directory is
//! [`fixtures/quality/m5/cluster-and-dedupe/`](../../../../fixtures/quality/m5/cluster-and-dedupe/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::diagnostics::{
    DiagnosticAnchorRemapStateClass, DiagnosticEvidencePlaneClass, DiagnosticFreshnessClass,
    DiagnosticOriginClass, DiagnosticRecord, DiagnosticRedactionClass, DiagnosticSeverityClass,
    DiagnosticSourceConfidenceClass, DiagnosticSourceKind, DiagnosticSupportClass,
    DiagnosticSurfaceClass,
};
pub use crate::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::DiagnosticClusterMeaningClass;
use crate::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::M5DiagnosticSurface;

/// Stable record-kind tag carried by [`DiagnosticClusterSetPacket`].
pub const M5_DIAGNOSTIC_CLUSTER_SET_RECORD_KIND: &str = "m5_diagnostic_cluster_set";

/// Stable record-kind tag for a [`DiagnosticDisplayCluster`].
pub const M5_DIAGNOSTIC_DISPLAY_CLUSTER_RECORD_KIND: &str = "m5_diagnostic_display_cluster";

/// Stable record-kind tag for a [`DiagnosticClusterMemberDetailSheet`].
pub const M5_DIAGNOSTIC_CLUSTER_MEMBER_DETAIL_SHEET_RECORD_KIND: &str =
    "m5_diagnostic_cluster_member_detail_sheet";

/// Stable record-kind tag for a [`DiagnosticClusterSurfaceProjection`].
pub const M5_DIAGNOSTIC_CLUSTER_SURFACE_PROJECTION_RECORD_KIND: &str =
    "m5_diagnostic_cluster_surface_projection";

/// Stable record-kind tag for a [`DiagnosticClusterSupportExport`].
pub const M5_DIAGNOSTIC_CLUSTER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_diagnostic_cluster_support_export";

/// Schema version for the M5 diagnostic-cluster set.
pub const M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-cluster.schema.json";

/// Repo-relative path of the reviewer-facing doc.
pub const M5_DIAGNOSTIC_CLUSTER_SET_DOC_REF: &str = "docs/m5/diagnostic-clusters-and-dedupe.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DIAGNOSTIC_CLUSTER_SET_ARTIFACT_REF: &str =
    "artifacts/m5/diagnostics/cluster-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_DIAGNOSTIC_CLUSTER_SET_SUMMARY_REF: &str =
    "artifacts/m5/diagnostics/cluster-proof/support_export.md";

/// Repo-relative path of the canonical normalized diagnostic-record set schema
/// this clustering view sits above rather than replaces.
pub const CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-record.schema.json";

/// Consumer surfaces that must expose the dedupe reason and cluster membership so
/// users can audit why several findings were shown as one ergonomic summary.
///
/// The editor and CLI/headless surfaces also consume clusters, but they do so
/// through the per-member detail sheets and the cluster body directly; these four
/// are the surfaces whose *projection* must carry the dedupe reason and full
/// membership.
pub const CLUSTER_EXPOSURE_SURFACES: [DiagnosticSurfaceClass; 4] = [
    DiagnosticSurfaceClass::Problems,
    DiagnosticSurfaceClass::Review,
    DiagnosticSurfaceClass::SupportExport,
    DiagnosticSurfaceClass::AiEvidence,
];

/// Whether a cluster member's evidence copy is live or imported.
///
/// Derived from [`DiagnosticOriginClass`] and kept explicit on every detail sheet
/// so convenience clustering can never let an imported snapshot read as live local
/// truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticImportedLiveClass {
    /// Produced against the current local workspace session.
    LiveLocal,
    /// Produced against the current remote workspace or target session.
    LiveRemote,
    /// Produced live by a managed or service-backed provider.
    ManagedLive,
    /// Imported from scanner, release, review, support, or provider evidence.
    Imported,
    /// Replayed from preserved support evidence rather than rerun live.
    Replayed,
    /// Restored from a local cache without fresh producer confirmation.
    Cached,
}

impl DiagnosticImportedLiveClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveLocal => "live_local",
            Self::LiveRemote => "live_remote",
            Self::ManagedLive => "managed_live",
            Self::Imported => "imported",
            Self::Replayed => "replayed",
            Self::Cached => "cached",
        }
    }

    /// Derives the imported-versus-live class from a diagnostic origin.
    pub const fn from_origin(origin: DiagnosticOriginClass) -> Self {
        match origin {
            DiagnosticOriginClass::LiveLocalSession => Self::LiveLocal,
            DiagnosticOriginClass::LiveRemoteSession => Self::LiveRemote,
            DiagnosticOriginClass::ManagedProviderLive => Self::ManagedLive,
            DiagnosticOriginClass::ImportedSnapshot => Self::Imported,
            DiagnosticOriginClass::ReplayedSupportBundle => Self::Replayed,
            DiagnosticOriginClass::LocalCache => Self::Cached,
        }
    }

    /// Returns true when this class is live local, remote, or managed truth.
    pub const fn is_live(self) -> bool {
        matches!(self, Self::LiveLocal | Self::LiveRemote | Self::ManagedLive)
    }

    /// Returns true when this class is imported or replayed evidence.
    pub const fn is_imported_or_replayed(self) -> bool {
        matches!(self, Self::Imported | Self::Replayed)
    }
}

/// Source-preserving detail sheet for one constituent of a display cluster.
///
/// Even when the default display row is clustered, this sheet keeps every fact a
/// user needs to recover and trust the member: its canonical diagnostic id,
/// provenance, target / environment ref, policy state, and imported-versus-live
/// class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterMemberDetailSheet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable detail-sheet id.
    pub detail_sheet_id: String,
    /// Canonical diagnostic id of the constituent, recoverable from the cluster.
    pub member_diagnostic_id: String,
    /// M5 surface that produced or preserved the constituent.
    pub surface: M5DiagnosticSurface,
    /// Normalized severity of the constituent.
    pub severity_class: DiagnosticSeverityClass,
    /// Source kind that produced or preserved the constituent.
    pub source_kind: DiagnosticSourceKind,
    /// Plane of evidence behind the constituent.
    pub evidence_plane_class: DiagnosticEvidencePlaneClass,
    /// Origin of the evidence copy held for the constituent.
    pub origin_class: DiagnosticOriginClass,
    /// Imported-versus-live class derived from the origin.
    pub imported_live_class: DiagnosticImportedLiveClass,
    /// Confidence class for the constituent's source.
    pub confidence_class: DiagnosticSourceConfidenceClass,
    /// Authority / policy posture for the constituent.
    pub support_class: DiagnosticSupportClass,
    /// Freshness state of the constituent.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Anchor remap state of the constituent.
    pub remap_state_class: DiagnosticAnchorRemapStateClass,
    /// Anchor family id of the constituent.
    pub anchor_family_id: String,
    /// Current anchor ref of the constituent, when one can be shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_anchor_ref: Option<String>,
    /// Source descriptor id, so the source can be recovered.
    pub source_descriptor_ref: String,
    /// Producer reference.
    pub producer_ref: String,
    /// Tool identity reference.
    pub tool_ref: String,
    /// Tool version reference, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_version_ref: Option<String>,
    /// Adapter reference, when an adapter emitted the constituent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_ref: Option<String>,
    /// Target or environment fingerprint reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_or_environment_ref: Option<String>,
    /// Strongest origin reference (session, import, run, or task).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_ref: Option<String>,
    /// Suppression refs that are part of the constituent's policy state.
    pub suppression_refs: Vec<String>,
    /// Baseline refs that are part of the constituent's policy state.
    pub baseline_refs: Vec<String>,
    /// Redaction posture for the constituent.
    pub redaction_class: DiagnosticRedactionClass,
    /// Stable surface-local ref a user follows to reopen the constituent.
    pub reopen_surface_ref: String,
    /// True when the constituent can be recovered from this sheet.
    pub recoverable: bool,
    /// True when this constituent requires visible source / freshness disclosure.
    pub disclosure_required: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl DiagnosticClusterMemberDetailSheet {
    /// Builds a source-preserving detail sheet from one canonical record.
    pub fn from_record(
        detail_sheet_id: impl Into<String>,
        surface: M5DiagnosticSurface,
        record: &DiagnosticRecord,
        reopen_surface_ref: impl Into<String>,
    ) -> Self {
        let source = &record.source;
        Self {
            record_kind: M5_DIAGNOSTIC_CLUSTER_MEMBER_DETAIL_SHEET_RECORD_KIND.to_owned(),
            schema_version: M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_VERSION,
            detail_sheet_id: detail_sheet_id.into(),
            member_diagnostic_id: record.diagnostic_id.clone(),
            surface,
            severity_class: record.severity_class,
            source_kind: source.source_kind,
            evidence_plane_class: source.evidence_plane_class,
            origin_class: source.origin_class,
            imported_live_class: DiagnosticImportedLiveClass::from_origin(source.origin_class),
            confidence_class: source.confidence_class,
            support_class: record.support_class,
            freshness_class: record.freshness_class,
            remap_state_class: record.anchor_remap.remap_state_class,
            anchor_family_id: record.anchor_remap.anchor_family_id.clone(),
            current_anchor_ref: record.anchor_remap.current_anchor_ref.clone(),
            source_descriptor_ref: source.source_id.clone(),
            producer_ref: source.producer_ref.clone(),
            tool_ref: source.tool_ref.clone(),
            tool_version_ref: source.tool_version_ref.clone(),
            adapter_ref: source.adapter_ref.clone(),
            target_or_environment_ref: source.target_or_environment_ref.clone(),
            origin_ref: source.origin_ref().map(str::to_owned),
            suppression_refs: record.suppression_refs.clone(),
            baseline_refs: record.baseline_refs.clone(),
            redaction_class: record.redaction_class,
            reopen_surface_ref: reopen_surface_ref.into(),
            recoverable: true,
            disclosure_required: record.requires_disclosure(),
            export_safe_summary: format!(
                "Detail sheet preserves {} provenance, target ref, and policy state for {}.",
                source.source_kind.as_str(),
                record.diagnostic_id
            ),
        }
    }

    /// Whether the constituent can be recovered from this detail sheet without
    /// translation loss.
    pub fn recovers_member(&self) -> bool {
        self.recoverable
            && !self.member_diagnostic_id.trim().is_empty()
            && !self.reopen_surface_ref.trim().is_empty()
            && !self.source_descriptor_ref.trim().is_empty()
    }

    /// Whether the imported-versus-live class agrees with the origin class.
    pub fn imported_live_consistent(&self) -> bool {
        self.imported_live_class == DiagnosticImportedLiveClass::from_origin(self.origin_class)
    }
}

/// Aggregate counts for one display cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterAggregateCounts {
    /// Total constituent count.
    pub member_count: usize,
    /// Count of distinct source kinds among constituents.
    pub distinct_source_kind_count: usize,
    /// Count of distinct origin classes among constituents.
    pub distinct_origin_class_count: usize,
    /// Count of error-severity constituents.
    pub error_count: usize,
    /// Count of warning-severity constituents.
    pub warning_count: usize,
    /// Count of notice-severity constituents.
    pub notice_count: usize,
    /// Count of hint-severity constituents.
    pub hint_count: usize,
    /// Count of imported or replayed constituents.
    pub imported_or_replayed_member_count: usize,
    /// Count of live (local, remote, or managed) constituents.
    pub live_member_count: usize,
    /// Count of constituents carrying a suppression join.
    pub suppressed_member_count: usize,
    /// Count of constituents carrying a baseline join.
    pub baselined_member_count: usize,
}

impl DiagnosticClusterAggregateCounts {
    /// Recomputes aggregate counts from a cluster's detail sheets.
    pub fn from_detail_sheets(sheets: &[DiagnosticClusterMemberDetailSheet]) -> Self {
        let distinct_source_kind_count = sheets
            .iter()
            .map(|sheet| sheet.source_kind)
            .collect::<BTreeSet<_>>()
            .len();
        let distinct_origin_class_count = sheets
            .iter()
            .map(|sheet| sheet.origin_class)
            .collect::<BTreeSet<_>>()
            .len();
        Self {
            member_count: sheets.len(),
            distinct_source_kind_count,
            distinct_origin_class_count,
            error_count: count_severity(sheets, DiagnosticSeverityClass::Error),
            warning_count: count_severity(sheets, DiagnosticSeverityClass::Warning),
            notice_count: count_severity(sheets, DiagnosticSeverityClass::Notice),
            hint_count: count_severity(sheets, DiagnosticSeverityClass::Hint),
            imported_or_replayed_member_count: sheets
                .iter()
                .filter(|sheet| sheet.imported_live_class.is_imported_or_replayed())
                .count(),
            live_member_count: sheets
                .iter()
                .filter(|sheet| sheet.imported_live_class.is_live())
                .count(),
            suppressed_member_count: sheets
                .iter()
                .filter(|sheet| !sheet.suppression_refs.is_empty())
                .count(),
            baselined_member_count: sheets
                .iter()
                .filter(|sheet| !sheet.baseline_refs.is_empty())
                .count(),
        }
    }
}

fn count_severity(
    sheets: &[DiagnosticClusterMemberDetailSheet],
    severity: DiagnosticSeverityClass,
) -> usize {
    sheets
        .iter()
        .filter(|sheet| sheet.severity_class == severity)
        .count()
}

/// The dominant display posture a compact cluster row shows.
///
/// The dominant severity is the most severe member; the dominant freshness and
/// remap states are the most cautionary members, so a compact row never reads as
/// fresher or better-anchored than its least-trustworthy constituent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterDominantDisplayState {
    /// Most severe constituent severity.
    pub dominant_severity_class: DiagnosticSeverityClass,
    /// Most cautionary constituent freshness.
    pub dominant_freshness_class: DiagnosticFreshnessClass,
    /// Most cautionary constituent remap state.
    pub dominant_remap_state_class: DiagnosticAnchorRemapStateClass,
    /// True when any constituent is imported or replayed.
    pub contains_imported_or_replayed_member: bool,
    /// True when any constituent is live local, remote, or managed.
    pub contains_live_member: bool,
    /// True when any constituent requires visible disclosure.
    pub disclosure_required: bool,
}

impl DiagnosticClusterDominantDisplayState {
    /// Recomputes the dominant display state from a cluster's detail sheets.
    ///
    /// Returns `None` for an empty member set; a display cluster always has at
    /// least one member.
    pub fn from_detail_sheets(sheets: &[DiagnosticClusterMemberDetailSheet]) -> Option<Self> {
        let dominant_severity_class = sheets.iter().map(|sheet| sheet.severity_class).min()?;
        let dominant_freshness_class = sheets
            .iter()
            .map(|sheet| sheet.freshness_class)
            .max()
            .expect("non-empty member set has a freshness");
        let dominant_remap_state_class = sheets
            .iter()
            .map(|sheet| sheet.remap_state_class)
            .max()
            .expect("non-empty member set has a remap state");
        Some(Self {
            dominant_severity_class,
            dominant_freshness_class,
            dominant_remap_state_class,
            contains_imported_or_replayed_member: sheets
                .iter()
                .any(|sheet| sheet.imported_live_class.is_imported_or_replayed()),
            contains_live_member: sheets
                .iter()
                .any(|sheet| sheet.imported_live_class.is_live()),
            disclosure_required: sheets.iter().any(|sheet| sheet.disclosure_required),
        })
    }
}

/// One constituent input to a display cluster: a canonical record plus the surface
/// that produced it and the ref a user follows to reopen it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticClusterMemberInput {
    /// M5 surface that produced or preserved the constituent.
    pub surface: M5DiagnosticSurface,
    /// Canonical diagnostic record for the constituent.
    pub record: DiagnosticRecord,
    /// Stable surface-local ref a user follows to reopen the constituent.
    pub reopen_surface_ref: String,
}

/// A display cluster that groups several findings into one ergonomic row while
/// preserving each constituent's distinct provenance and recoverability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticDisplayCluster {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable cluster id.
    pub cluster_id: String,
    /// Human-readable display label for the compact row.
    pub display_label_summary: String,
    /// Primary diagnostic id used for compact rendering; must be a member.
    pub primary_diagnostic_id: String,
    /// Primary anchor ref for the compact row, when one can be shown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_anchor_ref: Option<String>,
    /// Primary anchor family id for the compact row.
    pub primary_anchor_family_id: String,
    /// Diagnostic ids contributing to this cluster.
    pub contributing_diagnostic_ids: Vec<String>,
    /// Typed dedupe / clustering reason.
    pub dedupe_reason_class: DiagnosticClusterMeaningClass,
    /// Export-safe explanation of why the members were shown as one row.
    pub dedupe_reason_detail: String,
    /// Aggregate counts over constituents.
    pub aggregate_counts: DiagnosticClusterAggregateCounts,
    /// Dominant display posture over constituents.
    pub dominant_display_state: DiagnosticClusterDominantDisplayState,
    /// One source-preserving detail sheet per constituent.
    pub member_detail_sheets: Vec<DiagnosticClusterMemberDetailSheet>,
    /// Distinct source kinds preserved after clustering.
    pub preserved_source_kinds: Vec<DiagnosticSourceKind>,
    /// Distinct origin classes preserved after clustering.
    pub preserved_origin_classes: Vec<DiagnosticOriginClass>,
    /// Distinct freshness classes preserved after clustering.
    pub preserved_freshness_classes: Vec<DiagnosticFreshnessClass>,
    /// Distinct remap states preserved after clustering.
    pub preserved_remap_states: Vec<DiagnosticAnchorRemapStateClass>,
    /// Must stay `false`: a cluster is a view over real records, never a synthetic
    /// finding minted from flattened members.
    pub synthetic_finding: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl DiagnosticDisplayCluster {
    /// Builds a display cluster from constituent records while preserving each
    /// member's distinct provenance.
    pub fn from_members(
        cluster_id: impl Into<String>,
        display_label_summary: impl Into<String>,
        primary_diagnostic_id: impl Into<String>,
        dedupe_reason_class: DiagnosticClusterMeaningClass,
        dedupe_reason_detail: impl Into<String>,
        members: &[DiagnosticClusterMemberInput],
        export_safe_summary: impl Into<String>,
    ) -> Self {
        let cluster_id = cluster_id.into();
        let primary_diagnostic_id = primary_diagnostic_id.into();
        let member_detail_sheets = members
            .iter()
            .map(|member| {
                DiagnosticClusterMemberDetailSheet::from_record(
                    format!(
                        "detail_sheet:{}:{}",
                        sanitize_id(&cluster_id),
                        sanitize_id(&member.record.diagnostic_id)
                    ),
                    member.surface,
                    &member.record,
                    member.reopen_surface_ref.clone(),
                )
            })
            .collect::<Vec<_>>();

        let primary_member = members
            .iter()
            .find(|member| member.record.diagnostic_id == primary_diagnostic_id)
            .or_else(|| members.first());
        let primary_anchor_ref =
            primary_member.and_then(|member| member.record.anchor_remap.current_anchor_ref.clone());
        let primary_anchor_family_id = primary_member
            .map(|member| member.record.anchor_remap.anchor_family_id.clone())
            .unwrap_or_default();

        let aggregate_counts =
            DiagnosticClusterAggregateCounts::from_detail_sheets(&member_detail_sheets);
        let dominant_display_state =
            DiagnosticClusterDominantDisplayState::from_detail_sheets(&member_detail_sheets)
                .unwrap_or(DiagnosticClusterDominantDisplayState {
                    dominant_severity_class: DiagnosticSeverityClass::Hint,
                    dominant_freshness_class: DiagnosticFreshnessClass::Unverified,
                    dominant_remap_state_class: DiagnosticAnchorRemapStateClass::Unmapped,
                    contains_imported_or_replayed_member: false,
                    contains_live_member: false,
                    disclosure_required: false,
                });

        Self {
            record_kind: M5_DIAGNOSTIC_DISPLAY_CLUSTER_RECORD_KIND.to_owned(),
            schema_version: M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_VERSION,
            cluster_id,
            display_label_summary: display_label_summary.into(),
            primary_diagnostic_id,
            primary_anchor_ref,
            primary_anchor_family_id,
            contributing_diagnostic_ids: members
                .iter()
                .map(|member| member.record.diagnostic_id.clone())
                .collect(),
            dedupe_reason_class,
            dedupe_reason_detail: dedupe_reason_detail.into(),
            aggregate_counts,
            dominant_display_state,
            preserved_source_kinds: distinct_sorted(
                member_detail_sheets.iter().map(|sheet| sheet.source_kind),
            ),
            preserved_origin_classes: distinct_sorted(
                member_detail_sheets.iter().map(|sheet| sheet.origin_class),
            ),
            preserved_freshness_classes: distinct_sorted(
                member_detail_sheets
                    .iter()
                    .map(|sheet| sheet.freshness_class),
            ),
            preserved_remap_states: distinct_sorted(
                member_detail_sheets
                    .iter()
                    .map(|sheet| sheet.remap_state_class),
            ),
            member_detail_sheets,
            synthetic_finding: false,
            export_safe_summary: export_safe_summary.into(),
        }
    }

    /// Whether this cluster groups more than one underlying finding.
    pub fn groups_multiple(&self) -> bool {
        self.contributing_diagnostic_ids.len() > 1
    }

    /// Whether this cluster groups findings from more than one distinct source.
    pub fn is_cross_source(&self) -> bool {
        self.aggregate_counts.distinct_source_kind_count > 1
    }

    /// Whether every contributing id has a detail sheet that recovers it.
    pub fn recovers_every_member(&self) -> bool {
        self.contributing_diagnostic_ids.iter().all(|id| {
            self.member_detail_sheets
                .iter()
                .find(|sheet| &sheet.member_diagnostic_id == id)
                .is_some_and(DiagnosticClusterMemberDetailSheet::recovers_member)
        })
    }

    /// Whether the cluster preserved every member's source, origin, freshness, and
    /// remap label rather than dropping it during clustering.
    pub fn preserves_provenance(&self) -> bool {
        let counts_match = self.preserved_source_kinds
            == distinct_sorted(self.member_detail_sheets.iter().map(|s| s.source_kind))
            && self.preserved_origin_classes
                == distinct_sorted(self.member_detail_sheets.iter().map(|s| s.origin_class))
            && self.preserved_freshness_classes
                == distinct_sorted(self.member_detail_sheets.iter().map(|s| s.freshness_class))
            && self.preserved_remap_states
                == distinct_sorted(
                    self.member_detail_sheets
                        .iter()
                        .map(|s| s.remap_state_class),
                );
        let per_member_ok = self.member_detail_sheets.iter().all(|sheet| {
            self.preserved_source_kinds.contains(&sheet.source_kind)
                && self.preserved_origin_classes.contains(&sheet.origin_class)
                && self
                    .preserved_freshness_classes
                    .contains(&sheet.freshness_class)
                && self
                    .preserved_remap_states
                    .contains(&sheet.remap_state_class)
                && sheet.imported_live_consistent()
        });
        counts_match && per_member_ok
    }

    /// Whether the cluster is a view over real records rather than a flattened
    /// synthetic finding: it is not marked synthetic, has one detail sheet per
    /// contributing id, and names a primary that is itself a member.
    pub fn not_flattened(&self) -> bool {
        !self.synthetic_finding
            && self.member_detail_sheets.len() == self.contributing_diagnostic_ids.len()
            && self
                .contributing_diagnostic_ids
                .contains(&self.primary_diagnostic_id)
    }

    /// Whether the stored aggregate counts agree with the detail sheets.
    pub fn aggregate_counts_consistent(&self) -> bool {
        self.aggregate_counts
            == DiagnosticClusterAggregateCounts::from_detail_sheets(&self.member_detail_sheets)
    }

    /// Whether the stored dominant display state agrees with the detail sheets.
    pub fn dominant_state_consistent(&self) -> bool {
        DiagnosticClusterDominantDisplayState::from_detail_sheets(&self.member_detail_sheets)
            .is_some_and(|recomputed| recomputed == self.dominant_display_state)
    }

    /// Whether the dedupe-reason class is meaningful for the membership: a reason
    /// that groups multiple findings must back a cluster of more than one member.
    pub fn dedupe_reason_consistent(&self) -> bool {
        if self.dedupe_reason_class.groups_multiple() {
            self.groups_multiple()
        } else {
            self.contributing_diagnostic_ids.len() == 1
        }
    }

    /// Whether this cluster holds every structural invariant.
    pub fn is_structurally_complete(&self) -> bool {
        !self.cluster_id.trim().is_empty()
            && !self.display_label_summary.trim().is_empty()
            && !self.primary_diagnostic_id.trim().is_empty()
            && !self.dedupe_reason_detail.trim().is_empty()
            && !self.contributing_diagnostic_ids.is_empty()
            && self
                .contributing_diagnostic_ids
                .iter()
                .all(|id| !id.trim().is_empty())
            && self.not_flattened()
            && self.recovers_every_member()
            && self.preserves_provenance()
            && self.aggregate_counts_consistent()
            && self.dominant_state_consistent()
            && self.dedupe_reason_consistent()
    }

    /// Builds the cross-surface projection of this cluster for one surface.
    pub fn surface_projection(
        &self,
        surface_class: DiagnosticSurfaceClass,
    ) -> DiagnosticClusterSurfaceProjection {
        DiagnosticClusterSurfaceProjection {
            record_kind: M5_DIAGNOSTIC_CLUSTER_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
            schema_version: M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_VERSION,
            projection_id: format!(
                "cluster_projection:{}:{}",
                surface_class.as_str(),
                sanitize_id(&self.cluster_id)
            ),
            cluster_id: self.cluster_id.clone(),
            surface_class,
            primary_diagnostic_id: self.primary_diagnostic_id.clone(),
            dedupe_reason_class: self.dedupe_reason_class,
            dedupe_reason_detail: self.dedupe_reason_detail.clone(),
            member_diagnostic_ids: self.contributing_diagnostic_ids.clone(),
            member_count: self.contributing_diagnostic_ids.len(),
            dominant_severity_class: self.dominant_display_state.dominant_severity_class,
            dominant_freshness_class: self.dominant_display_state.dominant_freshness_class,
            disclosure_required: self.dominant_display_state.disclosure_required,
            exposes_dedupe_reason: true,
            exposes_membership: true,
            recovers_constituents: true,
            raw_source_content_included: false,
            raw_payload_included: false,
            export_safe_summary: format!(
                "{} projection exposes the {} dedupe reason and all {} constituents of cluster {}.",
                surface_class.as_str(),
                self.dedupe_reason_class.as_str(),
                self.contributing_diagnostic_ids.len(),
                self.cluster_id
            ),
        }
    }
}

/// Cross-surface projection of one cluster that exposes its dedupe reason and full
/// membership so a user can audit the ergonomic summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterSurfaceProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable projection id.
    pub projection_id: String,
    /// Cluster id projected.
    pub cluster_id: String,
    /// Surface consuming the projection.
    pub surface_class: DiagnosticSurfaceClass,
    /// Primary diagnostic id for the compact row.
    pub primary_diagnostic_id: String,
    /// Dedupe reason copied from the cluster.
    pub dedupe_reason_class: DiagnosticClusterMeaningClass,
    /// Dedupe reason detail copied from the cluster.
    pub dedupe_reason_detail: String,
    /// Full constituent membership exposed to the surface.
    pub member_diagnostic_ids: Vec<String>,
    /// Member count.
    pub member_count: usize,
    /// Dominant severity copied from the cluster.
    pub dominant_severity_class: DiagnosticSeverityClass,
    /// Dominant freshness copied from the cluster.
    pub dominant_freshness_class: DiagnosticFreshnessClass,
    /// Whether the cluster requires visible disclosure.
    pub disclosure_required: bool,
    /// Whether this projection exposes the dedupe reason.
    pub exposes_dedupe_reason: bool,
    /// Whether this projection exposes the full membership.
    pub exposes_membership: bool,
    /// Whether the constituents stay recoverable from this projection.
    pub recovers_constituents: bool,
    /// Whether raw source content is included in this projection.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included in this projection.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl DiagnosticClusterSurfaceProjection {
    /// Whether this projection exposes the dedupe reason and full membership and
    /// keeps constituents recoverable without raw content.
    pub fn is_honest(&self, cluster: &DiagnosticDisplayCluster) -> bool {
        self.exposes_dedupe_reason
            && self.exposes_membership
            && self.recovers_constituents
            && !self.raw_source_content_included
            && !self.raw_payload_included
            && self.dedupe_reason_class == cluster.dedupe_reason_class
            && self.member_diagnostic_ids == cluster.contributing_diagnostic_ids
    }
}

/// One row of a cluster's preserved constituent membership in a support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterConstituentExportRow {
    /// Cluster id this row preserves.
    pub cluster_id: String,
    /// Constituent diagnostic ids preserved for the cluster.
    pub member_diagnostic_ids: Vec<String>,
}

/// One row of a cluster's dedupe reason in a support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClusterDedupeReasonExportRow {
    /// Cluster id this row describes.
    pub cluster_id: String,
    /// Typed dedupe reason for the cluster.
    pub dedupe_reason_class: DiagnosticClusterMeaningClass,
    /// Export-safe dedupe reason detail.
    pub dedupe_reason_detail: String,
}

/// Support export that preserves both cluster meaning and constituent findings
/// rather than serializing a lossy display-only row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterSupportExport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable support export id.
    pub export_id: String,
    /// Workspace id covered by the export.
    pub workspace_id: String,
    /// Cluster ids cited by the export.
    pub cluster_refs: Vec<String>,
    /// Per-cluster constituent membership preserved by the export.
    pub clustered_constituent_refs: Vec<ClusterConstituentExportRow>,
    /// Flattened distinct constituent diagnostic ids.
    pub all_constituent_diagnostic_refs: Vec<String>,
    /// Per-cluster dedupe reasons preserved by the export.
    pub dedupe_reasons: Vec<ClusterDedupeReasonExportRow>,
    /// True when the export preserves each cluster's dedupe meaning.
    pub preserves_cluster_meaning: bool,
    /// True when the export preserves each cluster's constituent findings.
    pub preserves_constituents: bool,
    /// Redaction posture for the export.
    pub redaction_class: DiagnosticRedactionClass,
    /// Whether raw source content is included by default.
    pub raw_source_content_included: bool,
    /// Whether raw payload content is included by default.
    pub raw_payload_included: bool,
    /// Export-safe summary.
    pub export_safe_summary: String,
}

impl DiagnosticClusterSupportExport {
    /// Builds a metadata-only support export from a set of clusters.
    pub fn from_clusters(
        export_id: impl Into<String>,
        workspace_id: impl Into<String>,
        clusters: &[DiagnosticDisplayCluster],
    ) -> Self {
        let cluster_refs = clusters
            .iter()
            .map(|cluster| cluster.cluster_id.clone())
            .collect::<Vec<_>>();
        let clustered_constituent_refs = clusters
            .iter()
            .map(|cluster| ClusterConstituentExportRow {
                cluster_id: cluster.cluster_id.clone(),
                member_diagnostic_ids: cluster.contributing_diagnostic_ids.clone(),
            })
            .collect::<Vec<_>>();
        let all_constituent_diagnostic_refs = clusters
            .iter()
            .flat_map(|cluster| cluster.contributing_diagnostic_ids.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        let dedupe_reasons = clusters
            .iter()
            .map(|cluster| ClusterDedupeReasonExportRow {
                cluster_id: cluster.cluster_id.clone(),
                dedupe_reason_class: cluster.dedupe_reason_class,
                dedupe_reason_detail: cluster.dedupe_reason_detail.clone(),
            })
            .collect::<Vec<_>>();
        let member_total = all_constituent_diagnostic_refs.len();

        Self {
            record_kind: M5_DIAGNOSTIC_CLUSTER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_VERSION,
            export_id: export_id.into(),
            workspace_id: workspace_id.into(),
            cluster_refs,
            clustered_constituent_refs,
            all_constituent_diagnostic_refs,
            dedupe_reasons,
            preserves_cluster_meaning: true,
            preserves_constituents: true,
            redaction_class: DiagnosticRedactionClass::MetadataSafeDefault,
            raw_source_content_included: false,
            raw_payload_included: false,
            export_safe_summary: format!(
                "Support export preserves {} clusters and {} constituent diagnostics by id with raw content omitted by default.",
                clusters.len(),
                member_total
            ),
        }
    }

    /// Whether the export preserves every cluster's id, membership, and dedupe
    /// reason without lowering it to a lossy display-only row.
    pub fn preserves(&self, clusters: &[DiagnosticDisplayCluster]) -> bool {
        if !self.preserves_cluster_meaning || !self.preserves_constituents {
            return false;
        }
        clusters.iter().all(|cluster| {
            self.cluster_refs.contains(&cluster.cluster_id)
                && self.clustered_constituent_refs.iter().any(|row| {
                    row.cluster_id == cluster.cluster_id
                        && row.member_diagnostic_ids == cluster.contributing_diagnostic_ids
                })
                && self.dedupe_reasons.iter().any(|row| {
                    row.cluster_id == cluster.cluster_id
                        && row.dedupe_reason_class == cluster.dedupe_reason_class
                })
                && cluster
                    .contributing_diagnostic_ids
                    .iter()
                    .all(|id| self.all_constituent_diagnostic_refs.contains(id))
        })
    }
}

/// Set-level guardrail invariants that must all hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterGuardrails {
    /// Unlike sources are clustered for display but never flattened into one
    /// synthetic finding.
    pub unlike_sources_clustered_not_flattened: bool,
    /// No cluster mints a synthetic finding.
    pub no_synthetic_findings: bool,
    /// Anchors are never silently repaired by clustering.
    pub anchors_never_silently_repaired: bool,
    /// Imported-versus-live class stays explicit on every detail sheet.
    pub imported_live_class_preserved_in_detail: bool,
    /// Target / environment refs stay on every detail sheet.
    pub target_environment_refs_preserved_in_detail: bool,
    /// Policy state (support posture, suppression / baseline joins) stays on every
    /// detail sheet.
    pub policy_state_preserved_in_detail: bool,
    /// Dedupe reason and membership are exposed on every required surface.
    pub dedupe_reason_exposed_on_required_surfaces: bool,
    /// Diagnostic ids and collection completeness stay exportable and support-safe.
    pub diagnostic_ids_and_completeness_exportable: bool,
    /// Every constituent is recoverable from its detail sheet.
    pub every_constituent_recoverable_from_detail_sheet: bool,
}

impl DiagnosticClusterGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.unlike_sources_clustered_not_flattened
            && self.no_synthetic_findings
            && self.anchors_never_silently_repaired
            && self.imported_live_class_preserved_in_detail
            && self.target_environment_refs_preserved_in_detail
            && self.policy_state_preserved_in_detail
            && self.dedupe_reason_exposed_on_required_surfaces
            && self.diagnostic_ids_and_completeness_exportable
            && self.every_constituent_recoverable_from_detail_sheet
    }
}

/// Declares which consumer surfaces expose cluster membership and recover members.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterConsumerProjection {
    /// Problems rows expose the cluster membership and dedupe reason.
    pub problems_exposes_cluster_membership: bool,
    /// Review annotations expose the cluster membership and dedupe reason.
    pub review_exposes_cluster_membership: bool,
    /// Support export preserves the constituents and cluster meaning.
    pub support_export_preserves_constituents: bool,
    /// AI evidence exposes the cluster membership and dedupe reason.
    pub ai_evidence_exposes_cluster_membership: bool,
    /// Editor detail sheets recover each constituent.
    pub editor_detail_sheet_recovers_each_member: bool,
    /// CLI / headless output lists the dedupe reason.
    pub cli_headless_lists_dedupe_reason: bool,
}

impl DiagnosticClusterConsumerProjection {
    /// Whether every consumer projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.problems_exposes_cluster_membership
            && self.review_exposes_cluster_membership
            && self.support_export_preserves_constituents
            && self.ai_evidence_exposes_cluster_membership
            && self.editor_detail_sheet_recovers_each_member
            && self.cli_headless_lists_dedupe_reason
    }
}

/// Constructor input for a [`DiagnosticClusterSetPacket`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticClusterSetPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Workspace id covered by the set.
    pub workspace_id: String,
    /// Display clusters in the set.
    pub clusters: Vec<DiagnosticDisplayCluster>,
    /// Guardrail invariants block.
    pub guardrails: DiagnosticClusterGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DiagnosticClusterConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 diagnostic-cluster set packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticClusterSetPacket {
    /// Record kind; must equal [`M5_DIAGNOSTIC_CLUSTER_SET_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable set label.
    pub set_label: String,
    /// Workspace id covered by the set.
    pub workspace_id: String,
    /// Display clusters in the set.
    pub clusters: Vec<DiagnosticDisplayCluster>,
    /// Cross-surface cluster projections, one per cluster per exposure surface.
    pub surface_projections: Vec<DiagnosticClusterSurfaceProjection>,
    /// Default support export preserving cluster meaning and constituents.
    pub support_export: DiagnosticClusterSupportExport,
    /// Guardrail invariants block.
    pub guardrails: DiagnosticClusterGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DiagnosticClusterConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DiagnosticClusterSetPacket {
    /// Builds an M5 diagnostic-cluster set packet, deriving cross-surface
    /// projections and the default support export from the clusters.
    pub fn new(input: DiagnosticClusterSetPacketInput) -> Self {
        let surface_projections = input
            .clusters
            .iter()
            .flat_map(|cluster| {
                CLUSTER_EXPOSURE_SURFACES
                    .into_iter()
                    .map(|surface| cluster.surface_projection(surface))
            })
            .collect::<Vec<_>>();
        let support_export = DiagnosticClusterSupportExport::from_clusters(
            format!("cluster_support_export:{}", sanitize_id(&input.packet_id)),
            input.workspace_id.clone(),
            &input.clusters,
        );

        Self {
            record_kind: M5_DIAGNOSTIC_CLUSTER_SET_RECORD_KIND.to_owned(),
            schema_version: M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_VERSION,
            packet_id: input.packet_id,
            set_label: input.set_label,
            workspace_id: input.workspace_id,
            clusters: input.clusters,
            surface_projections,
            support_export,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// M5 surfaces represented by some cluster member in this set.
    pub fn represented_surfaces(&self) -> BTreeSet<M5DiagnosticSurface> {
        self.clusters
            .iter()
            .flat_map(|cluster| {
                cluster
                    .member_detail_sheets
                    .iter()
                    .map(|sheet| sheet.surface)
            })
            .collect()
    }

    /// Distinct constituent diagnostic ids represented in this set.
    pub fn represented_diagnostic_ids(&self) -> BTreeSet<String> {
        self.clusters
            .iter()
            .flat_map(|cluster| cluster.contributing_diagnostic_ids.iter().cloned())
            .collect()
    }

    /// Count of clusters that group more than one distinct source kind.
    pub fn cross_source_cluster_count(&self) -> usize {
        self.clusters
            .iter()
            .filter(|cluster| cluster.is_cross_source())
            .count()
    }

    /// The projection matching one cluster and surface, when present.
    pub fn projection_for(
        &self,
        cluster_id: &str,
        surface_class: DiagnosticSurfaceClass,
    ) -> Option<&DiagnosticClusterSurfaceProjection> {
        self.surface_projections.iter().find(|projection| {
            projection.cluster_id == cluster_id && projection.surface_class == surface_class
        })
    }

    /// Validates the M5 diagnostic-cluster set invariants.
    pub fn validate(&self) -> Vec<DiagnosticClusterViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DIAGNOSTIC_CLUSTER_SET_RECORD_KIND {
            violations.push(DiagnosticClusterViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_VERSION {
            violations.push(DiagnosticClusterViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.set_label.trim().is_empty()
            || self.workspace_id.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DiagnosticClusterViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_clusters(self, &mut violations);
        validate_support_export(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(DiagnosticClusterViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(DiagnosticClusterViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("diagnostic-cluster set serializes"),
        ) {
            violations.push(DiagnosticClusterViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("diagnostic-cluster set serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Diagnostic-Cluster Set\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.set_label));
        out.push_str(&format!("- Workspace: `{}`\n", self.workspace_id));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(&format!("- Clusters: {}\n", self.clusters.len()));
        out.push_str(&format!(
            "- Cross-source clusters: {}\n\n",
            self.cross_source_cluster_count()
        ));

        out.push_str(
            "| Cluster | Dedupe reason | Members | Sources | Dominant severity | Dominant freshness | Imported | Disclosure |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for cluster in &self.clusters {
            out.push_str(&format!(
                "| `{}` | {} | {} | {} | {} | {} | {} | {} |\n",
                cluster.cluster_id,
                cluster.dedupe_reason_class.as_str(),
                cluster.aggregate_counts.member_count,
                cluster.aggregate_counts.distinct_source_kind_count,
                cluster
                    .dominant_display_state
                    .dominant_severity_class
                    .as_str(),
                cluster
                    .dominant_display_state
                    .dominant_freshness_class
                    .as_str(),
                cluster
                    .dominant_display_state
                    .contains_imported_or_replayed_member,
                cluster.dominant_display_state.disclosure_required,
            ));
        }

        out.push('\n');
        for cluster in &self.clusters {
            out.push_str(&format!(
                "- `{}` — {} ({})\n",
                cluster.cluster_id, cluster.dedupe_reason_detail, cluster.display_label_summary
            ));
            for sheet in &cluster.member_detail_sheets {
                out.push_str(&format!(
                    "  - `{}` — {} / {} / {}\n",
                    sheet.member_diagnostic_id,
                    sheet.source_kind.as_str(),
                    sheet.imported_live_class.as_str(),
                    sheet.freshness_class.as_str(),
                ));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum DiagnosticClusterArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<DiagnosticClusterViolation>),
}

impl fmt::Display for DiagnosticClusterArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(
                    f,
                    "diagnostic-cluster set support export parse error: {err}"
                )
            }
            Self::Validation(violations) => write!(
                f,
                "diagnostic-cluster set support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for DiagnosticClusterArtifactError {}

/// Invariant violations reported by [`DiagnosticClusterSetPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticClusterViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Required canonical source contracts are missing.
    MissingSourceContracts,
    /// The set has no clusters.
    NoClusters,
    /// A cluster failed its structural completeness invariants.
    ClusterStructurallyIncomplete,
    /// A cluster flattened unlike sources into a synthetic finding.
    SyntheticFindingFlattening,
    /// A cluster dropped a member's source, origin, freshness, or remap label.
    ClusterDroppedProvenance,
    /// A constituent cannot be recovered from its detail sheet.
    MemberNotRecoverable,
    /// A cluster's primary diagnostic id is not one of its members.
    PrimaryNotAMember,
    /// A cluster's aggregate counts disagree with its members.
    AggregateCountsInconsistent,
    /// A cluster's dominant display state disagrees with its members.
    DominantStateInconsistent,
    /// No cluster proves different sources clustered without flattening.
    CrossSourceClusterMissing,
    /// A required exposure-surface projection is missing for a cluster.
    SurfaceProjectionMissing,
    /// A surface projection drops the dedupe reason or membership.
    SurfaceProjectionDropsDedupeOrMembership,
    /// The support export lost cluster meaning or constituent findings.
    SupportExportLossy,
    /// The support export includes raw source or payload content by default.
    SupportExportIncludesRawContent,
    /// Guardrail block is incomplete.
    GuardrailsIncomplete,
    /// Consumer projection block is incomplete.
    ConsumerProjectionIncomplete,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl DiagnosticClusterViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NoClusters => "no_clusters",
            Self::ClusterStructurallyIncomplete => "cluster_structurally_incomplete",
            Self::SyntheticFindingFlattening => "synthetic_finding_flattening",
            Self::ClusterDroppedProvenance => "cluster_dropped_provenance",
            Self::MemberNotRecoverable => "member_not_recoverable",
            Self::PrimaryNotAMember => "primary_not_a_member",
            Self::AggregateCountsInconsistent => "aggregate_counts_inconsistent",
            Self::DominantStateInconsistent => "dominant_state_inconsistent",
            Self::CrossSourceClusterMissing => "cross_source_cluster_missing",
            Self::SurfaceProjectionMissing => "surface_projection_missing",
            Self::SurfaceProjectionDropsDedupeOrMembership => {
                "surface_projection_drops_dedupe_or_membership"
            }
            Self::SupportExportLossy => "support_export_lossy",
            Self::SupportExportIncludesRawContent => "support_export_includes_raw_content",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked support-export artifact.
///
/// This is the canonical entry point downstream Problems, review, support, and AI
/// evidence surfaces use to ingest the cluster set instead of recomputing display
/// clustering per surface.
///
/// # Errors
///
/// Returns [`DiagnosticClusterArtifactError`] when the artifact cannot be parsed
/// or fails validation.
pub fn current_m5_diagnostic_cluster_set_export(
) -> Result<DiagnosticClusterSetPacket, DiagnosticClusterArtifactError> {
    let packet: DiagnosticClusterSetPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/diagnostics/cluster-proof/support_export.json"
    )))
    .map_err(DiagnosticClusterArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DiagnosticClusterArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &DiagnosticClusterSetPacket,
    violations: &mut Vec<DiagnosticClusterViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DIAGNOSTIC_CLUSTER_SET_SCHEMA_REF,
        M5_DIAGNOSTIC_CLUSTER_SET_DOC_REF,
        M5_DIAGNOSTIC_CLUSTER_SET_ARTIFACT_REF,
        CANONICAL_DIAGNOSTIC_RECORD_SET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DiagnosticClusterViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_clusters(
    packet: &DiagnosticClusterSetPacket,
    violations: &mut Vec<DiagnosticClusterViolation>,
) {
    if packet.clusters.is_empty() {
        violations.push(DiagnosticClusterViolation::NoClusters);
    }

    for cluster in &packet.clusters {
        if !cluster.is_structurally_complete() {
            violations.push(DiagnosticClusterViolation::ClusterStructurallyIncomplete);
        }
        if cluster.synthetic_finding
            || cluster.member_detail_sheets.len() != cluster.contributing_diagnostic_ids.len()
        {
            violations.push(DiagnosticClusterViolation::SyntheticFindingFlattening);
        }
        if !cluster.preserves_provenance() {
            violations.push(DiagnosticClusterViolation::ClusterDroppedProvenance);
        }
        if !cluster.recovers_every_member() {
            violations.push(DiagnosticClusterViolation::MemberNotRecoverable);
        }
        if !cluster
            .contributing_diagnostic_ids
            .contains(&cluster.primary_diagnostic_id)
        {
            violations.push(DiagnosticClusterViolation::PrimaryNotAMember);
        }
        if !cluster.aggregate_counts_consistent() {
            violations.push(DiagnosticClusterViolation::AggregateCountsInconsistent);
        }
        if !cluster.dominant_state_consistent() {
            violations.push(DiagnosticClusterViolation::DominantStateInconsistent);
        }

        for surface_class in CLUSTER_EXPOSURE_SURFACES {
            match packet.projection_for(&cluster.cluster_id, surface_class) {
                Some(projection) => {
                    if !projection.is_honest(cluster) {
                        violations.push(
                            DiagnosticClusterViolation::SurfaceProjectionDropsDedupeOrMembership,
                        );
                    }
                }
                None => violations.push(DiagnosticClusterViolation::SurfaceProjectionMissing),
            }
        }
    }

    if !packet.clusters.is_empty() && packet.cross_source_cluster_count() == 0 {
        violations.push(DiagnosticClusterViolation::CrossSourceClusterMissing);
    }
}

fn validate_support_export(
    packet: &DiagnosticClusterSetPacket,
    violations: &mut Vec<DiagnosticClusterViolation>,
) {
    if packet.support_export.raw_source_content_included
        || packet.support_export.raw_payload_included
    {
        violations.push(DiagnosticClusterViolation::SupportExportIncludesRawContent);
    }
    if !packet.support_export.preserves(&packet.clusters) {
        violations.push(DiagnosticClusterViolation::SupportExportLossy);
    }
}

fn distinct_sorted<T>(values: impl Iterator<Item = T>) -> Vec<T>
where
    T: Ord,
{
    values.collect::<BTreeSet<_>>().into_iter().collect()
}

fn sanitize_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '_' || ch == '-' {
                ch
            } else {
                '_'
            }
        })
        .collect()
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
