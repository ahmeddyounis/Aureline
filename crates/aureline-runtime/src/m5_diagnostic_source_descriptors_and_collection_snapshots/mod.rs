//! Diagnostic source descriptors and collection snapshots for every claimed M5
//! diagnostic-producing surface.
//!
//! Where [`crate::diagnostics`] froze the per-record canonical diagnostic /
//! source / anchor-remap objects,
//! [`crate::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix`]
//! froze the surface-to-class matrix, and
//! [`crate::normalize_m5_diagnostic_records_with_stable_ids_and_suppression_baseline_joins`]
//! froze per-finding identity, this module answers a different honesty question:
//! **where did a finding set come from, what scope was actually analyzed, and
//! what was omitted or still streaming when the user inspected it?**
//!
//! It ships two delivery-grade objects that reuse — rather than re-mint — the
//! shared diagnostic vocabulary:
//!
//! 1. A **source descriptor** is the existing canonical
//!    [`DiagnosticSource`] reused verbatim: it names a producer identity, tool
//!    and tool version, target / environment fingerprint, confidence,
//!    raw-payload ref, and imported-versus-live origin class across the
//!    `editor_structural`, `language_service`, `build_or_task`,
//!    `runtime_or_test`, `scanner_import`, `policy`, and `heuristic` families.
//!    The packet proves each descriptor survives normalization with its
//!    provenance intact instead of flattening to a generic provider name.
//! 2. A **collection snapshot** ([`DiagnosticCollectionSnapshot`]) is new to this
//!    module. It names a snapshot id, the workspace / workset / target scope it
//!    analyzed, a [`DiagnosticCollectionCompletenessClass`] completeness label, a
//!    [`DiagnosticFreshnessClass`] freshness, a [`DiagnosticCollectionStreamingState`],
//!    a created-at clock, an active profile ref, the materialized diagnostic refs
//!    *or* a [`DiagnosticStreamingCursor`], and the [`DiagnosticOmittedScope`]
//!    list naming what was withheld and why. A partial, filtered, streaming, or
//!    aborted snapshot can no longer masquerade as a complete whole-workspace
//!    enumeration.
//!
//! Each [`DiagnosticCollectionSnapshotEntry`] *auto-downgrades*: a claimed
//! snapshot that cannot prove its freshness, establish its completeness, disclose
//! a partial / filtered / streaming scope, resolve its streaming cursor, or cite a
//! contributing source descriptor carries an `effective_qualification` strictly
//! below its claim, a recorded downgrade trigger, and a precise degraded label —
//! so a collection claim never outruns the evidence behind it.
//!
//! [`DiagnosticSourceAndCollectionPacket::validate`] also refuses a packet that
//! flattens unlike sources into a synthetic finding, drops a source descriptor's
//! producer / tool-version / target / origin provenance, renders imported or
//! replayed evidence as live local truth, hides a partial / filtered / streaming
//! collection behind a complete-looking label, or lets an omitted scope go
//! unnamed.
//!
//! Raw source bytes, raw provider payloads, raw scanner reports, provider
//! cursors, credentials, and raw artifact bodies never cross this boundary; the
//! packet carries only typed class tokens, booleans, opaque ids, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/quality/diagnostic-source-and-collection.schema.json`](../../../../schemas/quality/diagnostic-source-and-collection.schema.json),
//! composed from
//! [`schemas/quality/diagnostic-source-descriptor.schema.json`](../../../../schemas/quality/diagnostic-source-descriptor.schema.json)
//! and
//! [`schemas/quality/diagnostic-collection-snapshot.schema.json`](../../../../schemas/quality/diagnostic-collection-snapshot.schema.json).
//! The contract doc is
//! [`docs/m5/diagnostic-source-and-collection.md`](../../../../docs/m5/diagnostic-source-and-collection.md).
//! The protected fixture directory is
//! [`fixtures/quality/m5/collection-scope-and-streaming/`](../../../../fixtures/quality/m5/collection-scope-and-streaming/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::diagnostics::{
    DiagnosticFreshnessClass, DiagnosticOriginClass, DiagnosticSource, DiagnosticSourceKind,
};
use crate::freeze_the_m5_diagnostic_record_source_collection_snapshot_and_anchor_remap_matrix::{
    DiagnosticCollectionCompletenessClass, M5DiagnosticSurface,
};
use crate::quality::QualityTargetScopeClass;

/// Stable record-kind tag carried by [`DiagnosticSourceAndCollectionPacket`].
pub const M5_SOURCE_AND_COLLECTION_RECORD_KIND: &str = "m5_diagnostic_source_and_collection";

/// Stable record-kind tag for one [`DiagnosticCollectionSnapshot`].
pub const DIAGNOSTIC_COLLECTION_SNAPSHOT_RECORD_KIND: &str = "diagnostic_collection_snapshot";

/// Schema version for the source-and-collection packet and its records.
pub const M5_SOURCE_AND_COLLECTION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the composed packet schema.
pub const M5_SOURCE_AND_COLLECTION_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-source-and-collection.schema.json";

/// Repo-relative path of the source-descriptor component schema.
pub const M5_SOURCE_DESCRIPTOR_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-source-descriptor.schema.json";

/// Repo-relative path of the collection-snapshot component schema.
pub const M5_COLLECTION_SNAPSHOT_SCHEMA_REF: &str =
    "schemas/quality/diagnostic-collection-snapshot.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SOURCE_AND_COLLECTION_DOC_REF: &str = "docs/m5/diagnostic-source-and-collection.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SOURCE_AND_COLLECTION_ARTIFACT_REF: &str =
    "artifacts/m5/diagnostics/source-collection-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_SOURCE_AND_COLLECTION_SUMMARY_REF: &str =
    "artifacts/m5/diagnostics/source-collection-proof/support_export.md";

/// Streaming posture of a [`DiagnosticCollectionSnapshot`].
///
/// Names whether the finding set the user is inspecting is fully materialized or
/// still arriving, so a snapshot that is still streaming can never read as a
/// settled, complete enumeration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCollectionStreamingState {
    /// The full set for the admitted scope is materialized; nothing is streaming.
    Settled,
    /// Results are still arriving; the set is incomplete and will grow.
    Streaming,
    /// Streaming paused before completion and can resume from the cursor.
    PausedPartial,
    /// Collection aborted before completion and will not resume without a rerun.
    Aborted,
}

impl DiagnosticCollectionStreamingState {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Settled => "settled",
            Self::Streaming => "streaming",
            Self::PausedPartial => "paused_partial",
            Self::Aborted => "aborted",
        }
    }

    /// Whether this state requires a visible streaming / partial cue rather than
    /// reading as a settled enumeration.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::Settled)
    }

    /// Whether this state must carry a resumable [`DiagnosticStreamingCursor`].
    pub const fn expects_cursor(self) -> bool {
        matches!(self, Self::Streaming | Self::PausedPartial)
    }

    /// Whether this state is durable enough to back a public claim.
    ///
    /// Only [`Self::Aborted`] forces auto-downgrade; an open stream or a paused
    /// partial set backs a claim as long as its disclosure invariants hold.
    pub const fn backs_claim(self) -> bool {
        !matches!(self, Self::Aborted)
    }
}

/// Reason a scope is omitted from a [`DiagnosticCollectionSnapshot`].
///
/// Every omitted scope must name why it was withheld so an empty or tiny result
/// set cannot quietly imply whole-workspace coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticOmittedScopeReasonClass {
    /// The scope falls outside the active quality profile's selection.
    OutsideActiveProfile,
    /// Findings in the scope are filtered or suppression-applied.
    FilteredBySuppression,
    /// The scope was excluded by the user's workset or target selection.
    ExcludedFromSelection,
    /// The scope has not been scanned yet because the collection is streaming.
    NotYetScanned,
    /// The producing analyzer is unavailable for the scope.
    AnalyzerUnavailable,
    /// Policy or permission withheld coverage of the scope.
    PolicyOrPermissionWithheld,
    /// The target or environment for the scope was unreachable.
    TargetUnreachable,
    /// A budget or timeout cut analysis of the scope short.
    BudgetOrTimeoutCut,
}

impl DiagnosticOmittedScopeReasonClass {
    /// Stable token recorded in schemas, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OutsideActiveProfile => "outside_active_profile",
            Self::FilteredBySuppression => "filtered_by_suppression",
            Self::ExcludedFromSelection => "excluded_from_selection",
            Self::NotYetScanned => "not_yet_scanned",
            Self::AnalyzerUnavailable => "analyzer_unavailable",
            Self::PolicyOrPermissionWithheld => "policy_or_permission_withheld",
            Self::TargetUnreachable => "target_unreachable",
            Self::BudgetOrTimeoutCut => "budget_or_timeout_cut",
        }
    }
}

/// One scope named as omitted from a collection snapshot, with its reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticOmittedScope {
    /// Opaque ref to the omitted scope (path family, target, or selection).
    pub scope_ref: String,
    /// Why the scope was withheld.
    pub reason_class: DiagnosticOmittedScopeReasonClass,
    /// Export-safe reason summary.
    pub summary: String,
}

impl DiagnosticOmittedScope {
    /// Whether this omitted-scope row is structurally well-formed.
    pub fn is_well_formed(&self) -> bool {
        !self.scope_ref.trim().is_empty()
            && !self.summary.trim().is_empty()
            && !label_is_generic(&self.summary)
    }
}

/// Resumable cursor for a still-streaming collection snapshot.
///
/// Carries only an opaque resume token and counts; no provider cursor bytes or
/// raw paginator state cross this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticStreamingCursor {
    /// Opaque resume token for the next page of results.
    pub cursor_token: String,
    /// Count of diagnostics materialized so far.
    pub emitted_count: u32,
    /// Whether more results remain beyond the emitted set.
    pub has_more: bool,
    /// Optional opaque ref to a resume action or session.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resume_hint_ref: Option<String>,
    /// Export-safe cursor summary.
    pub summary: String,
}

impl DiagnosticStreamingCursor {
    /// Whether this cursor is structurally well-formed.
    pub fn is_well_formed(&self) -> bool {
        !self.cursor_token.trim().is_empty() && !self.summary.trim().is_empty()
    }
}

/// Workspace / workset / target scope a collection snapshot analyzed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticCollectionScope {
    /// Granularity the snapshot claims to cover.
    pub scope_class: QualityTargetScopeClass,
    /// Opaque workspace root ref.
    pub workspace_ref: String,
    /// Opaque workset ref, when the snapshot is narrowed to a selected workset.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workset_ref: Option<String>,
    /// Target / environment fingerprint ref, when the snapshot is target-scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_or_environment_ref: Option<String>,
    /// Active quality-profile ref governing what the snapshot collected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_profile_ref: Option<String>,
}

impl DiagnosticCollectionScope {
    /// Whether the scope names a workspace root.
    pub fn is_well_formed(&self) -> bool {
        !self.workspace_ref.trim().is_empty()
    }
}

/// Constructor input for one [`DiagnosticCollectionSnapshot`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticCollectionSnapshotInput {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Human-readable snapshot label.
    pub snapshot_label: String,
    /// Diagnostic-producing surface the snapshot belongs to.
    pub surface: M5DiagnosticSurface,
    /// Workspace / workset / target scope analyzed.
    pub scope: DiagnosticCollectionScope,
    /// Completeness label for the collected set.
    pub completeness_class: DiagnosticCollectionCompletenessClass,
    /// Freshness of the collected evidence.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Streaming posture of the collected set.
    pub streaming_state: DiagnosticCollectionStreamingState,
    /// Imported-versus-live origin class for the collected evidence.
    pub origin_class: DiagnosticOriginClass,
    /// Snapshot creation clock reference.
    pub created_at: String,
    /// Materialized diagnostic refs collected so far.
    pub diagnostic_refs: Vec<String>,
    /// Resume cursor, present when the collection is still streaming.
    pub streaming_cursor: Option<DiagnosticStreamingCursor>,
    /// Scopes named as omitted, each with its reason.
    pub omitted_scopes: Vec<DiagnosticOmittedScope>,
    /// Source-descriptor ids that contributed to the snapshot.
    pub contributing_source_ids: Vec<String>,
    /// True when a non-complete / non-current / streaming set carries a cue.
    pub completeness_disclosed: bool,
    /// True when imported / replayed evidence is never shown as live local truth.
    pub imported_not_shown_as_live: bool,
    /// Export-safe snapshot summary.
    pub export_safe_summary: String,
}

/// Collection snapshot for one claimed M5 diagnostic-producing surface.
///
/// The headline new object this module owns: it makes the scope, completeness,
/// freshness, streaming state, omitted scopes, and contributing sources of a
/// finding set explicit and exportable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticCollectionSnapshot {
    /// Record kind; equals [`DIAGNOSTIC_COLLECTION_SNAPSHOT_RECORD_KIND`].
    pub record_kind: String,
    /// Integer schema version.
    pub diagnostic_collection_snapshot_schema_version: u32,
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Human-readable snapshot label.
    pub snapshot_label: String,
    /// Diagnostic-producing surface the snapshot belongs to.
    pub surface: M5DiagnosticSurface,
    /// Workspace / workset / target scope analyzed.
    pub scope: DiagnosticCollectionScope,
    /// Completeness label for the collected set.
    pub completeness_class: DiagnosticCollectionCompletenessClass,
    /// Freshness of the collected evidence.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Streaming posture of the collected set.
    pub streaming_state: DiagnosticCollectionStreamingState,
    /// Imported-versus-live origin class for the collected evidence.
    pub origin_class: DiagnosticOriginClass,
    /// Snapshot creation clock reference.
    pub created_at: String,
    /// Materialized diagnostic refs collected so far.
    pub diagnostic_refs: Vec<String>,
    /// Resume cursor, present when the collection is still streaming.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub streaming_cursor: Option<DiagnosticStreamingCursor>,
    /// Scopes named as omitted, each with its reason.
    pub omitted_scopes: Vec<DiagnosticOmittedScope>,
    /// Source-descriptor ids that contributed to the snapshot.
    pub contributing_source_ids: Vec<String>,
    /// True when a non-complete / non-current / streaming set carries a cue.
    pub completeness_disclosed: bool,
    /// True when imported / replayed evidence is never shown as live local truth.
    pub imported_not_shown_as_live: bool,
    /// Export-safe snapshot summary.
    pub export_safe_summary: String,
}

impl DiagnosticCollectionSnapshot {
    /// Builds a collection snapshot with required record-kind and schema fields.
    pub fn new(input: DiagnosticCollectionSnapshotInput) -> Self {
        Self {
            record_kind: DIAGNOSTIC_COLLECTION_SNAPSHOT_RECORD_KIND.to_owned(),
            diagnostic_collection_snapshot_schema_version: M5_SOURCE_AND_COLLECTION_SCHEMA_VERSION,
            snapshot_id: input.snapshot_id,
            snapshot_label: input.snapshot_label,
            surface: input.surface,
            scope: input.scope,
            completeness_class: input.completeness_class,
            freshness_class: input.freshness_class,
            streaming_state: input.streaming_state,
            origin_class: input.origin_class,
            created_at: input.created_at,
            diagnostic_refs: input.diagnostic_refs,
            streaming_cursor: input.streaming_cursor,
            omitted_scopes: input.omitted_scopes,
            contributing_source_ids: input.contributing_source_ids,
            completeness_disclosed: input.completeness_disclosed,
            imported_not_shown_as_live: input.imported_not_shown_as_live,
            export_safe_summary: input.export_safe_summary,
        }
    }

    /// Whether this snapshot must carry a visible completeness / freshness /
    /// streaming cue rather than reading as a current, complete enumeration.
    pub fn requires_disclosure(&self) -> bool {
        self.completeness_class.requires_disclosure()
            || self.freshness_class.requires_disclosure()
            || self.streaming_state.requires_disclosure()
            || !self.omitted_scopes.is_empty()
    }

    /// Whether a required completeness / freshness / streaming cue is present.
    pub fn disclosure_ok(&self) -> bool {
        if self.requires_disclosure() {
            self.completeness_disclosed
        } else {
            true
        }
    }

    /// Whether the membership and streaming cursor are mutually consistent.
    ///
    /// A settled snapshot carries no cursor; a streaming snapshot carries a
    /// resumable cursor that still reports more results; a paused-partial
    /// snapshot carries a cursor; an aborted snapshot may carry one or not.
    pub fn streaming_consistent(&self) -> bool {
        match self.streaming_state {
            DiagnosticCollectionStreamingState::Settled => self.streaming_cursor.is_none(),
            DiagnosticCollectionStreamingState::Streaming => self
                .streaming_cursor
                .as_ref()
                .is_some_and(|cursor| cursor.is_well_formed() && cursor.has_more),
            DiagnosticCollectionStreamingState::PausedPartial => self
                .streaming_cursor
                .as_ref()
                .is_some_and(DiagnosticStreamingCursor::is_well_formed),
            DiagnosticCollectionStreamingState::Aborted => match self.streaming_cursor.as_ref() {
                Some(cursor) => cursor.is_well_formed(),
                None => true,
            },
        }
    }

    /// Whether a partial / filtered / streaming / aborted snapshot names at least
    /// one omitted scope, so an incomplete set cannot pose as whole coverage.
    pub fn omitted_scopes_sufficient(&self) -> bool {
        let must_name_omitted = matches!(
            self.completeness_class,
            DiagnosticCollectionCompletenessClass::PartialVisibleScan
                | DiagnosticCollectionCompletenessClass::FilteredView
        ) || self.streaming_state.requires_disclosure();
        if must_name_omitted {
            !self.omitted_scopes.is_empty()
        } else {
            true
        }
    }

    /// Whether every named omitted scope is well-formed.
    pub fn omitted_scopes_well_formed(&self) -> bool {
        self.omitted_scopes
            .iter()
            .all(DiagnosticOmittedScope::is_well_formed)
    }

    /// Whether imported / replayed evidence keeps explicit separation from live
    /// local truth.
    pub fn imported_separation_ok(&self) -> bool {
        if self.origin_class.is_imported_or_replayed() {
            self.imported_not_shown_as_live
        } else {
            true
        }
    }

    /// Whether this snapshot holds every structural invariant.
    pub fn is_complete(&self) -> bool {
        self.record_kind == DIAGNOSTIC_COLLECTION_SNAPSHOT_RECORD_KIND
            && self.diagnostic_collection_snapshot_schema_version
                == M5_SOURCE_AND_COLLECTION_SCHEMA_VERSION
            && !self.snapshot_id.trim().is_empty()
            && !self.snapshot_label.trim().is_empty()
            && !self.created_at.trim().is_empty()
            && !self.export_safe_summary.trim().is_empty()
            && self.scope.is_well_formed()
            && self.disclosure_ok()
            && self.streaming_consistent()
            && self.omitted_scopes_sufficient()
            && self.omitted_scopes_well_formed()
            && self.imported_separation_ok()
            && !self.contributing_source_ids.is_empty()
            && self
                .contributing_source_ids
                .iter()
                .all(|id| !id.trim().is_empty())
            && self.diagnostic_refs.iter().all(|r| !r.trim().is_empty())
    }
}

/// Headline qualification a collection-snapshot entry may claim or hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCollectionQualificationClass {
    /// Held below preview until the collection truth is established.
    Held,
    /// Claimed at preview maturity.
    Preview,
    /// Claimed at beta maturity.
    Beta,
    /// Claimed at stable maturity.
    Stable,
}

impl DiagnosticCollectionQualificationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Held => "held",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
        }
    }

    /// Whether this class carries a public claim above held.
    pub const fn is_claimed(self) -> bool {
        !matches!(self, Self::Held)
    }

    /// Monotonic rank used to compare claimed and effective qualifications.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Held => 0,
            Self::Preview => 1,
            Self::Beta => 2,
            Self::Stable => 3,
        }
    }
}

/// Trigger that fired an auto-downgrade on a collection-snapshot entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticCollectionDowngradeTrigger {
    /// Freshness could not be proven for the admitted scope.
    UnprovenFreshness,
    /// Completeness could not be established and requires review.
    UnknownCompleteness,
    /// A partial / filtered / streaming scope was not disclosed.
    UndisclosedPartialScope,
    /// The streaming cursor could not be resolved for the streaming state.
    UnresolvedStreamingCursor,
    /// An omitted scope was named without a reason or precise summary.
    UnnamedOmittedScope,
    /// No contributing source descriptor backs the snapshot.
    MissingContributingSource,
    /// The collection aborted before completion.
    AbortedCollection,
}

impl DiagnosticCollectionDowngradeTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnprovenFreshness => "unproven_freshness",
            Self::UnknownCompleteness => "unknown_completeness",
            Self::UndisclosedPartialScope => "undisclosed_partial_scope",
            Self::UnresolvedStreamingCursor => "unresolved_streaming_cursor",
            Self::UnnamedOmittedScope => "unnamed_omitted_scope",
            Self::MissingContributingSource => "missing_contributing_source",
            Self::AbortedCollection => "aborted_collection",
        }
    }
}

/// A collection snapshot wrapped with its governance qualification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticCollectionSnapshotEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// The collection snapshot this entry governs.
    pub snapshot: DiagnosticCollectionSnapshot,
    /// Headline qualification publicly claimed for this entry.
    pub claimed_qualification: DiagnosticCollectionQualificationClass,
    /// Effective qualification after auto-downgrade; equals the claim when the
    /// snapshot's completeness, freshness, streaming, omitted-scope, and source
    /// truth all hold, and ranks strictly below it otherwise.
    pub effective_qualification: DiagnosticCollectionQualificationClass,
    /// Trigger that fired the downgrade, required when the entry is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<DiagnosticCollectionDowngradeTrigger>,
    /// Precise degraded label, required when the entry is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this entry.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this entry.
    pub source_contract_refs: Vec<String>,
}

impl DiagnosticCollectionSnapshotEntry {
    /// Whether this entry carries a public claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_qualification.is_claimed()
    }

    /// Whether the snapshot's collection truth is durable enough to back a claim.
    pub fn collection_truth_durable(&self) -> bool {
        let snapshot = &self.snapshot;
        snapshot.completeness_class.backs_claim()
            && !matches!(
                snapshot.freshness_class,
                DiagnosticFreshnessClass::Unverified
            )
            && snapshot.streaming_state.backs_claim()
            && snapshot.disclosure_ok()
            && snapshot.streaming_consistent()
            && snapshot.omitted_scopes_sufficient()
            && snapshot.omitted_scopes_well_formed()
            && !snapshot.contributing_source_ids.is_empty()
    }

    /// Whether the entry must downgrade below its claim.
    pub fn needs_downgrade(&self) -> bool {
        !self.collection_truth_durable()
    }

    /// Whether the effective qualification and downgrade evidence are consistent.
    pub fn downgrade_consistent(&self) -> bool {
        if self.needs_downgrade() {
            self.effective_qualification.rank() < self.claimed_qualification.rank()
                && self.downgrade_trigger.is_some()
                && self
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label))
        } else {
            self.effective_qualification == self.claimed_qualification
        }
    }

    /// Whether this entry holds every structural invariant the packet requires.
    pub fn is_structurally_complete(&self) -> bool {
        self.snapshot.is_complete()
            && self.downgrade_consistent()
            && !self.entry_id.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
    }
}

/// Packet-level guardrail invariants that must all hold.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSourceAndCollectionGuardrails {
    /// Unlike sources are never flattened into one synthetic finding.
    pub unlike_sources_never_flattened: bool,
    /// Source descriptors survive normalization with full provenance.
    pub source_descriptors_survive_normalization: bool,
    /// Imported-versus-live class stays explicit on sources and snapshots.
    pub imported_live_class_explicit: bool,
    /// Target / environment refs are preserved on sources and snapshots.
    pub target_environment_refs_preserved: bool,
    /// Every snapshot carries a completeness label.
    pub completeness_label_always_present: bool,
    /// Omitted scopes are named with reasons, never silently dropped.
    pub omitted_scopes_named_with_reasons: bool,
    /// Diagnostic ids and completeness stay exportable and support-safe.
    pub ids_and_completeness_exportable: bool,
    /// Snapshots auto-downgrade when their collection truth is not durable.
    pub snapshots_auto_downgrade_on_weak_truth: bool,
}

impl DiagnosticSourceAndCollectionGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.unlike_sources_never_flattened
            && self.source_descriptors_survive_normalization
            && self.imported_live_class_explicit
            && self.target_environment_refs_preserved
            && self.completeness_label_always_present
            && self.omitted_scopes_named_with_reasons
            && self.ids_and_completeness_exportable
            && self.snapshots_auto_downgrade_on_weak_truth
    }
}

/// Declares which downstream consumers preserve source and completeness truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSourceAndCollectionConsumerProjection {
    /// Problems rows show source and completeness truth.
    pub problems_shows_source_and_completeness: bool,
    /// Review packets carry source and completeness truth.
    pub review_carries_source_and_completeness: bool,
    /// Saved views preserve source and completeness truth.
    pub saved_views_preserve_source_and_completeness: bool,
    /// CLI / headless output prints source and completeness truth.
    pub cli_headless_prints_source_and_completeness: bool,
    /// Support export carries source and completeness truth.
    pub support_export_carries_source_and_completeness: bool,
    /// Omitted scopes stay visible rather than flattened away on every surface.
    pub omitted_scopes_visible_on_every_surface: bool,
}

impl DiagnosticSourceAndCollectionConsumerProjection {
    /// Whether every consumer projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.problems_shows_source_and_completeness
            && self.review_carries_source_and_completeness
            && self.saved_views_preserve_source_and_completeness
            && self.cli_headless_prints_source_and_completeness
            && self.support_export_carries_source_and_completeness
            && self.omitted_scopes_visible_on_every_surface
    }
}

/// Evidence freshness window for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSourceAndCollectionEvidenceFreshness {
    /// Freshness SLO in hours; zero is invalid.
    pub evidence_freshness_slo_hours: u32,
    /// Timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// Whether a snapshot auto-downgrades when its evidence is stale.
    pub auto_downgrade_on_stale: bool,
}

/// Constructor input for a [`DiagnosticSourceAndCollectionPacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSourceAndCollectionPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Source descriptors covering every claimed source family.
    pub source_descriptors: Vec<DiagnosticSource>,
    /// Collection-snapshot entries covering every claimed surface.
    pub snapshot_entries: Vec<DiagnosticCollectionSnapshotEntry>,
    /// Guardrail invariants block.
    pub guardrails: DiagnosticSourceAndCollectionGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DiagnosticSourceAndCollectionConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: DiagnosticSourceAndCollectionEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 diagnostic source-descriptor and collection-snapshot packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiagnosticSourceAndCollectionPacket {
    /// Record kind; must equal [`M5_SOURCE_AND_COLLECTION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SOURCE_AND_COLLECTION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Source descriptors covering every claimed source family.
    pub source_descriptors: Vec<DiagnosticSource>,
    /// Collection-snapshot entries covering every claimed surface.
    pub snapshot_entries: Vec<DiagnosticCollectionSnapshotEntry>,
    /// Guardrail invariants block.
    pub guardrails: DiagnosticSourceAndCollectionGuardrails,
    /// Consumer projection block.
    pub consumer_projection: DiagnosticSourceAndCollectionConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: DiagnosticSourceAndCollectionEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl DiagnosticSourceAndCollectionPacket {
    /// Builds a source-and-collection packet.
    pub fn new(input: DiagnosticSourceAndCollectionPacketInput) -> Self {
        Self {
            record_kind: M5_SOURCE_AND_COLLECTION_RECORD_KIND.to_owned(),
            schema_version: M5_SOURCE_AND_COLLECTION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            source_descriptors: input.source_descriptors,
            snapshot_entries: input.snapshot_entries,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Source families represented by some descriptor in this packet.
    pub fn represented_source_kinds(&self) -> BTreeSet<DiagnosticSourceKind> {
        self.source_descriptors
            .iter()
            .map(|source| source.source_kind)
            .collect()
    }

    /// Surfaces represented by some snapshot in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<M5DiagnosticSurface> {
        self.snapshot_entries
            .iter()
            .map(|entry| entry.snapshot.surface)
            .collect()
    }

    /// Count of snapshot entries whose effective qualification was downgraded.
    pub fn downgraded_entry_count(&self) -> usize {
        self.snapshot_entries
            .iter()
            .filter(|entry| entry.needs_downgrade())
            .count()
    }

    /// Count of snapshot entries holding a public claim.
    pub fn claimed_entry_count(&self) -> usize {
        self.snapshot_entries
            .iter()
            .filter(|entry| entry.is_claimed())
            .count()
    }

    /// Validates the source-descriptor and collection-snapshot invariants.
    pub fn validate(&self) -> Vec<DiagnosticSourceAndCollectionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SOURCE_AND_COLLECTION_RECORD_KIND {
            violations.push(DiagnosticSourceAndCollectionViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SOURCE_AND_COLLECTION_SCHEMA_VERSION {
            violations.push(DiagnosticSourceAndCollectionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(DiagnosticSourceAndCollectionViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_source_descriptors(self, &mut violations);
        validate_snapshot_coverage(self, &mut violations);
        validate_snapshot_entries(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_evidence_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("source-and-collection packet serializes"),
        ) {
            violations.push(DiagnosticSourceAndCollectionViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("source-and-collection packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Diagnostic Source Descriptors and Collection Snapshots\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!("- Minted: `{}`\n", self.minted_at));
        out.push_str(&format!(
            "- Source descriptors: {}\n",
            self.source_descriptors.len()
        ));
        out.push_str(&format!("- Snapshots: {}\n", self.snapshot_entries.len()));
        out.push_str(&format!(
            "- Claimed snapshots: {}\n",
            self.claimed_entry_count()
        ));
        out.push_str(&format!(
            "- Downgraded snapshots: {}\n\n",
            self.downgraded_entry_count()
        ));

        out.push_str("## Source descriptors\n\n");
        out.push_str("| Family | Origin | Confidence | Tool version |\n");
        out.push_str("| --- | --- | --- | --- |\n");
        for source in &self.source_descriptors {
            out.push_str(&format!(
                "| {} | {} | {:?} | {} |\n",
                source.source_kind.as_str(),
                origin_token(source.origin_class),
                source.confidence_class,
                source.tool_version_ref.as_deref().unwrap_or("unset"),
            ));
        }

        out.push_str("\n## Collection snapshots\n\n");
        out.push_str(
            "| Surface | Scope | Completeness | Freshness | Streaming | Omitted | Claimed | Effective |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
        for entry in &self.snapshot_entries {
            let snapshot = &entry.snapshot;
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
                snapshot.surface.as_str(),
                snapshot.scope.scope_class.as_str(),
                snapshot.completeness_class.as_str(),
                snapshot.freshness_class.as_str(),
                snapshot.streaming_state.as_str(),
                snapshot.omitted_scopes.len(),
                entry.claimed_qualification.as_str(),
                entry.effective_qualification.as_str(),
            ));
        }

        out.push('\n');
        for entry in &self.snapshot_entries {
            if let Some(label) = &entry.degraded_label {
                out.push_str(&format!(
                    "- Degraded: `{}` — {}\n",
                    entry.snapshot.surface.as_str(),
                    label
                ));
            }
        }

        out
    }
}

/// Error returned when the checked support-export artifact fails to load or
/// validate.
#[derive(Debug)]
pub enum DiagnosticSourceAndCollectionArtifactError {
    /// The support-export artifact could not be parsed.
    SupportExport(serde_json::Error),
    /// The parsed packet failed validation.
    Validation(Vec<DiagnosticSourceAndCollectionViolation>),
}

impl fmt::Display for DiagnosticSourceAndCollectionArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(err) => {
                write!(f, "source-and-collection support export parse error: {err}")
            }
            Self::Validation(violations) => write!(
                f,
                "source-and-collection support export failed validation: {violations:?}"
            ),
        }
    }
}

impl Error for DiagnosticSourceAndCollectionArtifactError {}

/// Invariant violations reported by [`DiagnosticSourceAndCollectionPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSourceAndCollectionViolation {
    /// Record kind is wrong.
    WrongRecordKind,
    /// Schema version is wrong.
    WrongSchemaVersion,
    /// Packet identity fields are missing.
    MissingIdentity,
    /// Required canonical source contracts are missing.
    MissingSourceContracts,
    /// A required source family is unrepresented.
    RequiredSourceFamilyMissing,
    /// A source descriptor lost producer / tool-version / target / origin truth.
    SourceDescriptorProvenanceMissing,
    /// A required diagnostic-producing surface is unrepresented.
    RequiredSurfaceMissing,
    /// No consistent downgraded entry demonstrates the auto-downgrade rule.
    DowngradedEntryCaseMissing,
    /// A snapshot entry failed its structural completeness invariants.
    EntryIncomplete,
    /// An entry with weak collection truth was not downgraded.
    EntryNotDowngradedOnWeakTruth,
    /// A downgraded entry is missing its precise label or trigger.
    DowngradedEntryMissingLabelOrTrigger,
    /// A partial / filtered / streaming collection was hidden.
    CollectionCompletenessHidden,
    /// A partial / filtered / streaming snapshot named no omitted scope.
    OmittedScopeMissing,
    /// An omitted scope was named without a reason or precise summary.
    OmittedScopeMalformed,
    /// The streaming state and cursor were inconsistent.
    StreamingStateInconsistent,
    /// Imported or replayed evidence was rendered as live local truth.
    ImportedShownAsLive,
    /// A snapshot is missing backing evidence refs.
    EntryEvidenceMissing,
    /// A snapshot cites no contributing source descriptor.
    ContributingSourceMissing,
    /// Guardrail block is incomplete.
    GuardrailsIncomplete,
    /// Consumer projection block is incomplete.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export-safe JSON carried forbidden boundary material.
    RawBoundaryMaterialInExport,
}

impl DiagnosticSourceAndCollectionViolation {
    /// Stable token for the violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSourceFamilyMissing => "required_source_family_missing",
            Self::SourceDescriptorProvenanceMissing => "source_descriptor_provenance_missing",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DowngradedEntryCaseMissing => "downgraded_entry_case_missing",
            Self::EntryIncomplete => "entry_incomplete",
            Self::EntryNotDowngradedOnWeakTruth => "entry_not_downgraded_on_weak_truth",
            Self::DowngradedEntryMissingLabelOrTrigger => {
                "downgraded_entry_missing_label_or_trigger"
            }
            Self::CollectionCompletenessHidden => "collection_completeness_hidden",
            Self::OmittedScopeMissing => "omitted_scope_missing",
            Self::OmittedScopeMalformed => "omitted_scope_malformed",
            Self::StreamingStateInconsistent => "streaming_state_inconsistent",
            Self::ImportedShownAsLive => "imported_shown_as_live",
            Self::EntryEvidenceMissing => "entry_evidence_missing",
            Self::ContributingSourceMissing => "contributing_source_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Loads and validates the checked support-export artifact.
///
/// This is the canonical entry point downstream Problems, review, saved-view,
/// CLI/headless, and support surfaces use to ingest the frozen source-descriptor
/// and collection-snapshot truth instead of cloning provider-local scan state.
///
/// # Errors
///
/// Returns [`DiagnosticSourceAndCollectionArtifactError`] when the artifact
/// cannot be parsed or fails validation.
pub fn current_m5_source_and_collection_export(
) -> Result<DiagnosticSourceAndCollectionPacket, DiagnosticSourceAndCollectionArtifactError> {
    let packet: DiagnosticSourceAndCollectionPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/m5/diagnostics/source-collection-proof/support_export.json"
    )))
    .map_err(DiagnosticSourceAndCollectionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DiagnosticSourceAndCollectionArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &DiagnosticSourceAndCollectionPacket,
    violations: &mut Vec<DiagnosticSourceAndCollectionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SOURCE_AND_COLLECTION_SCHEMA_REF,
        M5_SOURCE_DESCRIPTOR_SCHEMA_REF,
        M5_COLLECTION_SNAPSHOT_SCHEMA_REF,
        M5_SOURCE_AND_COLLECTION_DOC_REF,
        M5_SOURCE_AND_COLLECTION_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(DiagnosticSourceAndCollectionViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_source_descriptors(
    packet: &DiagnosticSourceAndCollectionPacket,
    violations: &mut Vec<DiagnosticSourceAndCollectionViolation>,
) {
    let kinds = packet.represented_source_kinds();
    for required in DiagnosticSourceKind::ALL_BETA_CLAIMED {
        if !kinds.contains(&required) {
            violations.push(DiagnosticSourceAndCollectionViolation::RequiredSourceFamilyMissing);
            break;
        }
    }

    for source in &packet.source_descriptors {
        // A descriptor must survive normalization with its producer, tool
        // version, origin reference, and target / environment fingerprint
        // intact rather than flattening to a generic provider name.
        let intact = source.has_required_provenance()
            && source
                .target_or_environment_ref
                .as_ref()
                .is_some_and(|value| !value.trim().is_empty());
        if !intact {
            violations
                .push(DiagnosticSourceAndCollectionViolation::SourceDescriptorProvenanceMissing);
        }
    }
}

fn validate_snapshot_coverage(
    packet: &DiagnosticSourceAndCollectionPacket,
    violations: &mut Vec<DiagnosticSourceAndCollectionViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in M5DiagnosticSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(DiagnosticSourceAndCollectionViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .snapshot_entries
        .iter()
        .any(|entry| entry.needs_downgrade() && entry.downgrade_consistent())
    {
        violations.push(DiagnosticSourceAndCollectionViolation::DowngradedEntryCaseMissing);
    }
}

fn validate_snapshot_entries(
    packet: &DiagnosticSourceAndCollectionPacket,
    violations: &mut Vec<DiagnosticSourceAndCollectionViolation>,
) {
    let known_source_ids: BTreeSet<&str> = packet
        .source_descriptors
        .iter()
        .map(|source| source.source_id.as_str())
        .collect();

    for entry in &packet.snapshot_entries {
        let snapshot = &entry.snapshot;
        if !entry.is_structurally_complete() {
            violations.push(DiagnosticSourceAndCollectionViolation::EntryIncomplete);
        }
        if entry.needs_downgrade()
            && entry.effective_qualification.rank() >= entry.claimed_qualification.rank()
        {
            violations.push(DiagnosticSourceAndCollectionViolation::EntryNotDowngradedOnWeakTruth);
        }
        if entry.needs_downgrade()
            && (entry.downgrade_trigger.is_none()
                || !entry
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations
                .push(DiagnosticSourceAndCollectionViolation::DowngradedEntryMissingLabelOrTrigger);
        }
        if !snapshot.disclosure_ok() {
            violations.push(DiagnosticSourceAndCollectionViolation::CollectionCompletenessHidden);
        }
        if !snapshot.omitted_scopes_sufficient() {
            violations.push(DiagnosticSourceAndCollectionViolation::OmittedScopeMissing);
        }
        if !snapshot.omitted_scopes_well_formed() {
            violations.push(DiagnosticSourceAndCollectionViolation::OmittedScopeMalformed);
        }
        if !snapshot.streaming_consistent() {
            violations.push(DiagnosticSourceAndCollectionViolation::StreamingStateInconsistent);
        }
        if !snapshot.imported_separation_ok() {
            violations.push(DiagnosticSourceAndCollectionViolation::ImportedShownAsLive);
        }
        if entry.evidence_refs.is_empty() || entry.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(DiagnosticSourceAndCollectionViolation::EntryEvidenceMissing);
        }
        if snapshot.contributing_source_ids.is_empty()
            || snapshot
                .contributing_source_ids
                .iter()
                .any(|id| !known_source_ids.contains(id.as_str()))
        {
            violations.push(DiagnosticSourceAndCollectionViolation::ContributingSourceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &DiagnosticSourceAndCollectionPacket,
    violations: &mut Vec<DiagnosticSourceAndCollectionViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(DiagnosticSourceAndCollectionViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &DiagnosticSourceAndCollectionPacket,
    violations: &mut Vec<DiagnosticSourceAndCollectionViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(DiagnosticSourceAndCollectionViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_evidence_freshness(
    packet: &DiagnosticSourceAndCollectionPacket,
    violations: &mut Vec<DiagnosticSourceAndCollectionViolation>,
) {
    if packet.evidence_freshness.evidence_freshness_slo_hours == 0
        || packet
            .evidence_freshness
            .last_evidence_refresh
            .trim()
            .is_empty()
    {
        violations.push(DiagnosticSourceAndCollectionViolation::EvidenceFreshnessIncomplete);
    }
}

/// Stable token for a [`DiagnosticOriginClass`] in Markdown summaries.
fn origin_token(origin: DiagnosticOriginClass) -> &'static str {
    match origin {
        DiagnosticOriginClass::LiveLocalSession => "live_local_session",
        DiagnosticOriginClass::LiveRemoteSession => "live_remote_session",
        DiagnosticOriginClass::ManagedProviderLive => "managed_provider_live",
        DiagnosticOriginClass::ImportedSnapshot => "imported_snapshot",
        DiagnosticOriginClass::ReplayedSupportBundle => "replayed_support_bundle",
        DiagnosticOriginClass::LocalCache => "local_cache",
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
///
/// A generic provider error must never stand in for a precise downgrade truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "narrowed"
            | "downgraded"
            | "omitted"
            | "partial"
    )
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
