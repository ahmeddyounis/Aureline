//! Scope receipts for export / copy / rerun / suppress / install / update /
//! delete flows, plus saved-query deep-link snapshot truth, so a bulk action on a
//! dense M5 surface names whether it touched the *selected items* versus *all
//! matching items*.
//!
//! Where
//! [`crate::bind_batch_review_sheets_and_action_descriptors_with_undo_class_and_policy_review`]
//! binds a reviewed selection scope to the broad action about to run and previews
//! its included / excluded / blocked / skipped members, this module records what a
//! broad action *actually touched* once it commits, and makes the scope class a
//! durable, export-safe receipt rather than a phrase in a toast. The companion
//! concern is reopen honesty: a shared link or saved-query snapshot must preserve
//! the difference between the scope it *captured* and the scope that is *currently
//! live*, and never imply frozen certainty about results that may have changed.
//!
//! Each [`ScopeReceipt`] pins one [`DenseCollectionSurface`] rendered as a
//! [`CollectionViewKind`] to:
//!
//! - a [`BatchActionKind`] (export, copy, rerun, suppress, install, update,
//!   delete) and an [`ExecutionOriginClass`];
//! - a [`ScopeReceiptClass`] — the dimension this lane makes canonical — naming
//!   whether the action touched [`ScopeReceiptClass::SelectedItems`],
//!   [`ScopeReceiptClass::VisibleRows`], [`ScopeReceiptClass::LoadedRows`],
//!   [`ScopeReceiptClass::AllMatchingQuery`], or a
//!   [`ScopeReceiptClass::ProviderSideSelection`]; and
//! - a [`ScopeReceiptCounts`] that records the selected, visible, loaded, and
//!   all-matching populations side by side so the receipt names *selected items
//!   versus all matching items* rather than letting visible rows pass as all
//!   matching rows.
//!
//! Each [`SavedQueryDeepLinkSnapshot`] records a captured scope plus its reopen
//! posture ([`DeepLinkReopenPosture`]) and any [`SnapshotOmission`] rows so a
//! reopened batch context preserves current-versus-captured scope and omission
//! reasons, and never implies frozen certainty about live results.
//!
//! The lane carries the guardrails the track demands: a row highlight never stands
//! in for a durable selection; provider or policy narrowing is never hidden inside
//! a generic filter chip; visible rows are never treated as all matching rows
//! without an explicit expansion step; a broad action never bypasses preview
//! because the list is virtualized or provider-backed; and a deep link never
//! implies frozen certainty about live results.
//! [`ScopeReceipt::reconstruction`] projects the same truth into a
//! redaction-aware [`ScopeReceiptReconstruction`] that support and audit packets
//! reuse to reconstruct the exact batch scope class a consequential operation used.
//!
//! The boundary schema is
//! [`schemas/collections/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.schema.json`](../../../../schemas/collections/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.schema.json).
//! The contract doc is
//! [`docs/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.md`](../../../../docs/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.md).
//! The protected fixture directory is
//! [`fixtures/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/`](../../../../fixtures/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::{
    BatchActionKind, DenseCollectionSurface,
};
use crate::ship_result_scope_counters_and_hidden_narrowing_chips::CollectionViewKind;
use crate::stabilize_selection_scope_and_batch_result_truth::ExecutionOriginClass;

/// Stable record-kind tag carried by [`ScopeReceiptPacket`].
pub const SCOPE_RECEIPT_RECORD_KIND: &str = "m5_scope_receipt_packet";

/// Integer schema version for the scope-receipt packet.
pub const SCOPE_RECEIPT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SCOPE_RECEIPT_SCHEMA_REF: &str =
    "schemas/collections/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.schema.json";

/// Repo-relative path of the contract doc.
pub const SCOPE_RECEIPT_DOC_REF: &str =
    "docs/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.md";

/// Repo-relative path of the protected fixture directory.
pub const SCOPE_RECEIPT_FIXTURE_DIR: &str =
    "fixtures/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l";

/// Repo-relative path of the checked support-export artifact.
pub const SCOPE_RECEIPT_ARTIFACT_REF: &str =
    "artifacts/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SCOPE_RECEIPT_SUMMARY_REF: &str =
    "artifacts/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l.md";

/// The first real M5 dense surfaces this lane must record a scope receipt for.
const REQUIRED_RECEIPT_SURFACES: [DenseCollectionSurface; 5] = [
    DenseCollectionSurface::PipelineRunList,
    DenseCollectionSurface::ReviewQueue,
    DenseCollectionSurface::IncidentList,
    DenseCollectionSurface::MarketplaceResults,
    DenseCollectionSurface::ProviderAdminTable,
];

/// The broad action kinds this lane must emit a scope receipt for.
const REQUIRED_RECEIPT_ACTION_KINDS: [BatchActionKind; 7] = [
    BatchActionKind::Export,
    BatchActionKind::Copy,
    BatchActionKind::Rerun,
    BatchActionKind::Suppress,
    BatchActionKind::Install,
    BatchActionKind::Update,
    BatchActionKind::Delete,
];

/// Which population a bulk action actually touched. This is the dimension this
/// lane makes canonical: a receipt always names whether the action touched the
/// selected items, the visible rows, the loaded rows, all matching query results,
/// or a provider-side selection set — never letting one stand in for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReceiptClass {
    /// The explicit, stable-identity selection the operator built.
    SelectedItems,
    /// Only the rows currently visible / rendered in the viewport.
    VisibleRows,
    /// Every materialized client row, including rows scrolled out of view.
    LoadedRows,
    /// Every row matching the active query, beyond what the client loaded.
    AllMatchingQuery,
    /// A provider-owned selection set the client did not enumerate row by row.
    ProviderSideSelection,
}

impl ScopeReceiptClass {
    /// Every scope-receipt class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SelectedItems,
        Self::VisibleRows,
        Self::LoadedRows,
        Self::AllMatchingQuery,
        Self::ProviderSideSelection,
    ];

    /// Stable token recorded in packets, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedItems => "selected_items",
            Self::VisibleRows => "visible_rows",
            Self::LoadedRows => "loaded_rows",
            Self::AllMatchingQuery => "all_matching_query",
            Self::ProviderSideSelection => "provider_side_selection",
        }
    }

    /// True when the class is a provider-owned selection the client did not
    /// enumerate.
    pub const fn is_provider_side(self) -> bool {
        matches!(self, Self::ProviderSideSelection)
    }

    /// True when the class reaches beyond the loaded client rows and so must be
    /// reached through a deliberate, explicit expansion step backed by a query
    /// snapshot — visible rows are never treated as all matching rows implicitly.
    pub const fn requires_explicit_expansion(self) -> bool {
        matches!(self, Self::AllMatchingQuery | Self::ProviderSideSelection)
    }
}

/// Selected / visible / loaded / all-matching populations recorded side by side
/// for one scope receipt, plus the count the action acted on and the count it
/// omitted. Recording the populations together is what lets the receipt name
/// *selected items versus all matching items*.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReceiptCounts {
    /// Size of the explicit, stable-identity selection.
    pub selected_count: u64,
    /// Rows visible in the viewport at action time.
    pub visible_count: u64,
    /// Materialized client rows at action time.
    pub loaded_count: u64,
    /// All rows matching the active query, when known. `None` when the count is
    /// provider-limited or still resolving.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_count: Option<u64>,
    /// Provider-side selection size, when the provider reports it. `None` when the
    /// provider does not enumerate the set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_side_count: Option<u64>,
    /// True when the all-matching (or provider-side) count is approximate.
    pub matching_is_approximate: bool,
    /// Members the action actually acted on.
    pub acted_on_count: u64,
    /// Members named in the receipt's scope but omitted (blocked, policy-narrowed,
    /// provider-removed) and accounted for explicitly.
    pub omitted_count: u64,
}

impl ScopeReceiptCounts {
    /// The recorded population for one scope class, when it is known.
    pub fn population_for(&self, class: ScopeReceiptClass) -> Option<u64> {
        match class {
            ScopeReceiptClass::SelectedItems => Some(self.selected_count),
            ScopeReceiptClass::VisibleRows => Some(self.visible_count),
            ScopeReceiptClass::LoadedRows => Some(self.loaded_count),
            ScopeReceiptClass::AllMatchingQuery => self.matching_count,
            ScopeReceiptClass::ProviderSideSelection => self.provider_side_count,
        }
    }

    /// Whether the acted-on and omitted counts reconcile against the receipt's
    /// scope class. When the population is known, the acted-on and omitted members
    /// partition it exactly; when it is unknown (an unbounded provider or matching
    /// set), the count must be flagged approximate and the acted-on members must
    /// not exceed any loaded basis.
    pub fn reconciles(&self, class: ScopeReceiptClass) -> bool {
        match self.population_for(class) {
            Some(population) => {
                self.acted_on_count.checked_add(self.omitted_count) == Some(population)
            }
            None => self.matching_is_approximate,
        }
    }

    /// Whether the counts let the operator tell the selected scope from the
    /// all-matching scope: the selection size is recorded and the matching scope is
    /// either known or explicitly flagged approximate.
    pub fn distinguishes_selected_from_matching(&self) -> bool {
        self.matching_count.is_some()
            || self.provider_side_count.is_some()
            || self.matching_is_approximate
    }
}

/// Redaction-aware projection of one scope receipt for support and audit packets.
/// Carries only ids, tokens, and counts — never raw row bodies or provider
/// payloads — so support can reconstruct the exact batch scope class used.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReceiptReconstruction {
    /// Receipt id this reconstruction projects.
    pub receipt_id: String,
    /// Surface token.
    pub surface_token: String,
    /// View-kind token.
    pub view_kind_token: String,
    /// Action-kind token.
    pub action_kind_token: String,
    /// Scope-class token — the exact batch scope class the operation used.
    pub scope_class_token: String,
    /// Execution-origin token.
    pub execution_origin_token: String,
    /// Members the action acted on.
    pub acted_on_count: u64,
    /// Members omitted from the scope and accounted for.
    pub omitted_count: u64,
    /// Selected-items population.
    pub selected_count: u64,
    /// All-matching population, when known.
    pub matching_count: Option<u64>,
    /// True when the all-matching count is approximate.
    pub matching_is_approximate: bool,
    /// True when the scope is a provider-side selection.
    pub provider_side: bool,
    /// True when expansion beyond loaded rows required an explicit step.
    pub expansion_was_explicit: bool,
    /// True when a query snapshot backs the scope.
    pub has_query_snapshot: bool,
}

/// One scope receipt recording what a committed broad action actually touched.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReceipt {
    /// Stable receipt id.
    pub receipt_id: String,
    /// Bound dense collection surface.
    pub surface: DenseCollectionSurface,
    /// How the surface is rendered.
    pub view_kind: CollectionViewKind,
    /// Broad action kind this receipt records.
    pub action_kind: BatchActionKind,
    /// Which population the action touched.
    pub scope_class: ScopeReceiptClass,
    /// Where the action executed.
    pub execution_origin: ExecutionOriginClass,
    /// Reviewed selection-scope object this receipt acted on.
    pub selection_id_ref: String,
    /// Query snapshot backing an all-matching or provider-side scope. Required
    /// whenever the scope reaches beyond loaded rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_snapshot_id_ref: Option<String>,
    /// Selected / visible / loaded / all-matching populations and acted-on count.
    pub counts: ScopeReceiptCounts,
    /// Redaction-aware, precise label naming the scope the action touched — never a
    /// generic non-answer.
    pub scope_label: String,
    /// True when expansion from visible/loaded rows to an all-matching or
    /// provider-side scope required a deliberate, explicit step.
    pub expansion_was_explicit: bool,
    /// True when the action mutated persistent or provider state.
    pub mutates_state: bool,
    /// True when a provider owns the execution or authoritative membership.
    pub provider_backed: bool,
    /// Packet redaction class token for this receipt.
    pub redaction_class_token: String,
    /// Evidence packet refs backing this receipt.
    pub evidence_refs: Vec<String>,
}

impl ScopeReceipt {
    /// Whether the receipt must carry a query snapshot: an all-matching or
    /// provider-side scope is only honest when it is pinned to a snapshot.
    pub fn requires_query_snapshot(&self) -> bool {
        self.scope_class.requires_explicit_expansion()
    }

    /// Whether the receipt distinguishes the selected scope from the all-matching
    /// scope so the operator can tell which one the action touched.
    pub fn names_selected_versus_matching(&self) -> bool {
        self.counts.distinguishes_selected_from_matching()
    }

    /// Whether the receipt satisfies its invariants: it carries identity and a
    /// precise scope label, its counts reconcile against the scope class, it pins a
    /// query snapshot and an explicit expansion when the scope reaches beyond
    /// loaded rows, a provider-side scope is genuinely provider-backed, and a
    /// provider-backed action does not claim a purely local origin.
    pub fn is_valid(&self) -> bool {
        if self.receipt_id.trim().is_empty()
            || self.selection_id_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || label_is_generic(&self.scope_label)
        {
            return false;
        }
        if !self.counts.reconciles(self.scope_class) {
            return false;
        }
        if !self.names_selected_versus_matching() {
            return false;
        }
        if self.requires_query_snapshot() {
            // Beyond-loaded scope must be pinned to a snapshot and reached by an
            // explicit expansion step rather than treating visible rows as all
            // matching rows.
            if !self.expansion_was_explicit {
                return false;
            }
            match &self.query_snapshot_id_ref {
                Some(snapshot) if !snapshot.trim().is_empty() => {}
                _ => return false,
            }
        }
        // A provider-side scope is only meaningful for a provider-backed action.
        if self.scope_class.is_provider_side() && !self.provider_backed {
            return false;
        }
        // A provider-backed action cannot claim a purely local execution origin.
        if self.provider_backed && self.execution_origin == ExecutionOriginClass::LocalClient {
            return false;
        }
        if self.evidence_refs.is_empty() || self.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            return false;
        }
        true
    }

    /// Projects the receipt into a redaction-aware reconstruction for support and
    /// audit packets.
    pub fn reconstruction(&self) -> ScopeReceiptReconstruction {
        ScopeReceiptReconstruction {
            receipt_id: self.receipt_id.clone(),
            surface_token: self.surface.as_str().to_owned(),
            view_kind_token: self.view_kind.as_str().to_owned(),
            action_kind_token: self.action_kind.as_str().to_owned(),
            scope_class_token: self.scope_class.as_str().to_owned(),
            execution_origin_token: self.execution_origin.as_str().to_owned(),
            acted_on_count: self.counts.acted_on_count,
            omitted_count: self.counts.omitted_count,
            selected_count: self.counts.selected_count,
            matching_count: self.counts.matching_count,
            matching_is_approximate: self.counts.matching_is_approximate,
            provider_side: self.scope_class.is_provider_side(),
            expansion_was_explicit: self.expansion_was_explicit,
            has_query_snapshot: self.query_snapshot_id_ref.is_some(),
        }
    }
}

/// How a captured deep-link or saved-query scope relates to the currently live
/// results when it is reopened. The posture is descriptive truth, never a frozen
/// guarantee — a snapshot always reports whether the captured scope still holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeepLinkReopenPosture {
    /// The captured scope still matches the live results, reported as an
    /// observation rather than a guarantee.
    CapturedMatchesCurrent,
    /// The live results have diverged from the captured scope; both are shown.
    CurrentDivergedFromCaptured,
    /// The captured snapshot is stale and must be re-resolved against the current
    /// query before any batch action.
    CapturedSnapshotStale,
    /// The scope is provider-backed and the provider cannot guarantee the live set
    /// matches the captured one.
    ProviderResultsMayDiffer,
}

impl DeepLinkReopenPosture {
    /// Every reopen posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CapturedMatchesCurrent,
        Self::CurrentDivergedFromCaptured,
        Self::CapturedSnapshotStale,
        Self::ProviderResultsMayDiffer,
    ];

    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedMatchesCurrent => "captured_matches_current",
            Self::CurrentDivergedFromCaptured => "current_diverged_from_captured",
            Self::CapturedSnapshotStale => "captured_snapshot_stale",
            Self::ProviderResultsMayDiffer => "provider_results_may_differ",
        }
    }

    /// True when the posture asserts the captured scope still matches live results.
    /// Even this posture is an observation, not a frozen guarantee.
    pub const fn claims_current_match(self) -> bool {
        matches!(self, Self::CapturedMatchesCurrent)
    }

    /// True when the posture explicitly acknowledges divergence from the captured
    /// scope.
    pub const fn acknowledges_divergence(self) -> bool {
        matches!(
            self,
            Self::CurrentDivergedFromCaptured
                | Self::CapturedSnapshotStale
                | Self::ProviderResultsMayDiffer
        )
    }
}

/// Why a member captured in a deep-link or saved-query snapshot is no longer in
/// the current scope when the snapshot is reopened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotOmissionCause {
    /// The member no longer matches the live query.
    NoLongerMatchesQuery,
    /// A policy / permission rule now narrows the member out.
    PolicyNarrowed,
    /// The provider removed or revoked the member.
    ProviderRemoved,
    /// The member was deleted since capture.
    Deleted,
    /// The member is outside the current workset scope.
    OutsideCurrentWorkset,
    /// Partial data means the member's current status is unknown.
    PartialDataUnknown,
}

impl SnapshotOmissionCause {
    /// Every omission cause, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoLongerMatchesQuery,
        Self::PolicyNarrowed,
        Self::ProviderRemoved,
        Self::Deleted,
        Self::OutsideCurrentWorkset,
        Self::PartialDataUnknown,
    ];

    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLongerMatchesQuery => "no_longer_matches_query",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::ProviderRemoved => "provider_removed",
            Self::Deleted => "deleted",
            Self::OutsideCurrentWorkset => "outside_current_workset",
            Self::PartialDataUnknown => "partial_data_unknown",
        }
    }

    /// True when the omission comes from a provider or policy decision, which the
    /// snapshot must surface explicitly rather than folding into a filter chip.
    pub const fn is_provider_or_policy(self) -> bool {
        matches!(self, Self::PolicyNarrowed | Self::ProviderRemoved)
    }
}

/// One reason some captured members are no longer in the current scope. Always
/// visible to the operator and carrying a precise reason, so omissions are never
/// silent.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotOmission {
    /// Why the members are omitted.
    pub cause: SnapshotOmissionCause,
    /// Number of captured members omitted by this cause.
    pub member_count: u64,
    /// Redaction-aware, precise reason label — never a generic non-answer.
    pub reason_label: String,
    /// True when the omission is surfaced to the operator rather than hidden in a
    /// generic filter chip (required).
    pub visible_to_operator: bool,
}

impl SnapshotOmission {
    /// Whether the omission is well formed: it covers at least one member, carries
    /// a precise reason, and is surfaced to the operator.
    pub fn is_valid(&self) -> bool {
        self.member_count > 0 && self.visible_to_operator && !label_is_generic(&self.reason_label)
    }
}

/// One saved-query or deep-link snapshot binding a captured scope to its reopen
/// truth, so a shared batch context preserves current-versus-captured honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedQueryDeepLinkSnapshot {
    /// Stable snapshot id.
    pub snapshot_id: String,
    /// Bound dense collection surface.
    pub surface: DenseCollectionSurface,
    /// Scope class captured when the snapshot was minted.
    pub captured_scope_class: ScopeReceiptClass,
    /// Query snapshot backing the captured scope.
    pub query_snapshot_id_ref: String,
    /// When the scope was captured.
    pub captured_at: String,
    /// All-matching population at capture, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub captured_matching_count: Option<u64>,
    /// True when the captured count was approximate.
    pub captured_is_approximate: bool,
    /// When the snapshot was reopened, if it has been.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reopened_at: Option<String>,
    /// All-matching population observed on reopen, when re-resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_matching_count: Option<u64>,
    /// How the captured scope relates to the current live results.
    pub reopen_posture: DeepLinkReopenPosture,
    /// Captured members no longer in the current scope, with reasons.
    pub omissions: Vec<SnapshotOmission>,
    /// True when the snapshot preserves the captured-versus-current distinction
    /// (required).
    pub preserves_current_versus_captured: bool,
    /// True when the snapshot implies frozen certainty about live results
    /// (forbidden).
    pub implies_frozen_certainty: bool,
    /// True when reopening re-resolves the captured query against the live results
    /// rather than replaying a frozen set (required).
    pub reopen_rebinds_to_live_query: bool,
    /// Redaction-aware, precise snapshot label — never a generic non-answer.
    pub snapshot_label: String,
    /// Evidence packet refs backing this snapshot.
    pub evidence_refs: Vec<String>,
}

impl SavedQueryDeepLinkSnapshot {
    /// Whether the snapshot has been reopened.
    pub fn is_reopened(&self) -> bool {
        self.reopened_at.is_some()
    }

    /// Whether the reopen honesty invariants hold: the snapshot preserves the
    /// captured-versus-current distinction, never implies frozen certainty, and
    /// rebinds the query to live results on reopen.
    pub fn honesty_holds(&self) -> bool {
        self.preserves_current_versus_captured
            && !self.implies_frozen_certainty
            && self.reopen_rebinds_to_live_query
    }

    /// Whether divergence is reported consistently: when any captured member is
    /// omitted, the posture must acknowledge divergence rather than claim the
    /// captured scope still matches the live results.
    pub fn divergence_consistent(&self) -> bool {
        let has_omissions = self
            .omissions
            .iter()
            .any(|omission| omission.member_count > 0);
        if has_omissions {
            self.reopen_posture.acknowledges_divergence()
        } else {
            true
        }
    }

    /// Provider/policy omission cause tokens this snapshot surfaces.
    pub fn provider_policy_omission_tokens(&self) -> Vec<String> {
        self.omissions
            .iter()
            .filter(|omission| omission.cause.is_provider_or_policy())
            .map(|omission| omission.cause.as_str().to_owned())
            .collect()
    }

    /// Whether the snapshot satisfies every invariant required to record it.
    pub fn is_valid(&self) -> bool {
        if self.snapshot_id.trim().is_empty()
            || self.query_snapshot_id_ref.trim().is_empty()
            || self.captured_at.trim().is_empty()
            || label_is_generic(&self.snapshot_label)
        {
            return false;
        }
        if !self.omissions.iter().all(SnapshotOmission::is_valid) {
            return false;
        }
        if !self.honesty_holds() || !self.divergence_consistent() {
            return false;
        }
        if self.is_reopened()
            && self
                .reopened_at
                .as_ref()
                .map_or(true, |stamp| stamp.trim().is_empty())
        {
            return false;
        }
        if self.evidence_refs.is_empty() || self.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            return false;
        }
        true
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReceiptGuardrails {
    /// A row highlight never stands in for a durable selection.
    pub row_highlight_is_not_durable_selection: bool,
    /// Provider or policy narrowing is never hidden inside a generic filter chip.
    pub provider_policy_narrowing_never_hidden: bool,
    /// Visible rows are never treated as all matching rows without an explicit
    /// step.
    pub visible_rows_not_all_matching_without_explicit_step: bool,
    /// A broad action cannot bypass preview because the list is virtualized or
    /// provider-backed.
    pub broad_action_cannot_bypass_preview: bool,
    /// A deep link or saved-query snapshot never implies frozen certainty about
    /// live results.
    pub deep_link_never_implies_frozen_certainty: bool,
    /// Every scope receipt names the selected scope versus the all-matching scope.
    pub receipt_names_selected_versus_matching: bool,
}

impl ScopeReceiptGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.row_highlight_is_not_durable_selection
            && self.provider_policy_narrowing_never_hidden
            && self.visible_rows_not_all_matching_without_explicit_step
            && self.broad_action_cannot_bypass_preview
            && self.deep_link_never_implies_frozen_certainty
            && self.receipt_names_selected_versus_matching
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReceiptConsumerProjection {
    /// Product renders the scope receipt and its counts from these records.
    pub product_renders_scope_receipt: bool,
    /// Diagnostics reconstruct the exact scope class from these records.
    pub diagnostics_reconstructs_scope_class: bool,
    /// Support/export reuses the scope-receipt projection.
    pub support_export_reuses_records: bool,
    /// Docs and help reuse the scope-receipt vocabulary.
    pub docs_help_reuses_vocabulary: bool,
}

impl ScopeReceiptConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_renders_scope_receipt
            && self.diagnostics_reconstructs_scope_class
            && self.support_export_reuses_records
            && self.docs_help_reuses_vocabulary
    }
}

/// Constructor input for [`ScopeReceiptPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeReceiptPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-action scope receipts.
    pub receipts: Vec<ScopeReceipt>,
    /// Saved-query / deep-link snapshots.
    pub deep_link_snapshots: Vec<SavedQueryDeepLinkSnapshot>,
    /// Guardrail invariants block.
    pub guardrails: ScopeReceiptGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ScopeReceiptConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe scope-receipt and deep-link snapshot packet for the first real
/// broad M5 actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReceiptPacket {
    /// Record kind; must equal [`SCOPE_RECEIPT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCOPE_RECEIPT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-action scope receipts.
    pub receipts: Vec<ScopeReceipt>,
    /// Saved-query / deep-link snapshots.
    pub deep_link_snapshots: Vec<SavedQueryDeepLinkSnapshot>,
    /// Guardrail invariants block.
    pub guardrails: ScopeReceiptGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ScopeReceiptConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ScopeReceiptPacket {
    /// Builds a scope-receipt packet.
    pub fn new(input: ScopeReceiptPacketInput) -> Self {
        Self {
            record_kind: SCOPE_RECEIPT_RECORD_KIND.to_owned(),
            schema_version: SCOPE_RECEIPT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            receipts: input.receipts,
            deep_link_snapshots: input.deep_link_snapshots,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some receipt in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<DenseCollectionSurface> {
        self.receipts
            .iter()
            .map(|receipt| receipt.surface)
            .collect()
    }

    /// Action kinds represented by some receipt in this packet.
    pub fn represented_action_kinds(&self) -> BTreeSet<BatchActionKind> {
        self.receipts
            .iter()
            .map(|receipt| receipt.action_kind)
            .collect()
    }

    /// Scope classes represented by some receipt in this packet.
    pub fn represented_scope_classes(&self) -> BTreeSet<ScopeReceiptClass> {
        self.receipts
            .iter()
            .map(|receipt| receipt.scope_class)
            .collect()
    }

    /// Reopen postures represented by some snapshot in this packet.
    pub fn represented_reopen_postures(&self) -> BTreeSet<DeepLinkReopenPosture> {
        self.deep_link_snapshots
            .iter()
            .map(|snapshot| snapshot.reopen_posture)
            .collect()
    }

    /// Reconstructions for every receipt, used by support and audit packets.
    pub fn reconstructions(&self) -> Vec<ScopeReceiptReconstruction> {
        self.receipts
            .iter()
            .map(ScopeReceipt::reconstruction)
            .collect()
    }

    /// Validates the scope-receipt packet invariants.
    pub fn validate(&self) -> Vec<ScopeReceiptViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SCOPE_RECEIPT_RECORD_KIND {
            violations.push(ScopeReceiptViolation::WrongRecordKind);
        }
        if self.schema_version != SCOPE_RECEIPT_SCHEMA_VERSION {
            violations.push(ScopeReceiptViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ScopeReceiptViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_receipts(self, &mut violations);
        validate_snapshots(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(ScopeReceiptViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ScopeReceiptViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("scope receipt packet serializes"),
        ) {
            violations.push(ScopeReceiptViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("scope receipt packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Scope Receipts And Saved-Query Deep-Link Snapshots\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Receipts: {} across {} / {} surfaces\n",
            self.receipts.len(),
            self.represented_surfaces().len(),
            REQUIRED_RECEIPT_SURFACES.len()
        ));
        out.push_str(&format!(
            "- Scope classes: {} / {}\n",
            self.represented_scope_classes().len(),
            ScopeReceiptClass::ALL.len()
        ));
        out.push_str(&format!(
            "- Deep-link snapshots: {} across {} / {} postures\n",
            self.deep_link_snapshots.len(),
            self.represented_reopen_postures().len(),
            DeepLinkReopenPosture::ALL.len()
        ));
        out.push_str("\n## Scope receipts\n\n");
        for receipt in &self.receipts {
            out.push_str(&format!(
                "- **{}** ({} / {} / {}): {}\n",
                receipt.receipt_id,
                receipt.surface.as_str(),
                receipt.view_kind.as_str(),
                receipt.action_kind.as_str(),
                receipt.scope_label,
            ));
            out.push_str(&format!(
                "  - scope=`{}` origin=`{}` acted_on={} omitted={}\n",
                receipt.scope_class.as_str(),
                receipt.execution_origin.as_str(),
                receipt.counts.acted_on_count,
                receipt.counts.omitted_count,
            ));
            out.push_str(&format!(
                "  - selected={} visible={} loaded={} matching={} approx={}\n",
                receipt.counts.selected_count,
                receipt.counts.visible_count,
                receipt.counts.loaded_count,
                receipt
                    .counts
                    .matching_count
                    .map_or_else(|| "?".to_owned(), |value| value.to_string()),
                receipt.counts.matching_is_approximate,
            ));
        }
        out.push_str("\n## Deep-link snapshots\n\n");
        for snapshot in &self.deep_link_snapshots {
            out.push_str(&format!(
                "- **{}** ({} / captured `{}`): {}\n",
                snapshot.snapshot_id,
                snapshot.surface.as_str(),
                snapshot.captured_scope_class.as_str(),
                snapshot.snapshot_label,
            ));
            out.push_str(&format!(
                "  - posture=`{}` reopened={} frozen_certainty={}\n",
                snapshot.reopen_posture.as_str(),
                snapshot.is_reopened(),
                snapshot.implies_frozen_certainty,
            ));
            for omission in &snapshot.omissions {
                out.push_str(&format!(
                    "  - omitted `{}` x{}: {}\n",
                    omission.cause.as_str(),
                    omission.member_count,
                    omission.reason_label,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in scope-receipt export.
#[derive(Debug)]
pub enum ScopeReceiptArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ScopeReceiptViolation>),
}

impl fmt::Display for ScopeReceiptArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "scope receipt export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "scope receipt export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ScopeReceiptArtifactError {}

/// Validation failures emitted by [`ScopeReceiptPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeReceiptViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required real M5 surface has no scope receipt.
    RequiredSurfaceMissing,
    /// A required broad action kind has no scope receipt.
    RequiredActionKindMissing,
    /// A scope-receipt class is represented by no receipt.
    ScopeClassCoverageMissing,
    /// No receipt records a provider-side selection scope.
    ProviderSideCaseMissing,
    /// No receipt records an all-matching-query scope.
    AllMatchingCaseMissing,
    /// A scope receipt fails its invariants.
    ReceiptInvalid,
    /// A scope receipt's counts do not reconcile against its scope class.
    CountsDoNotReconcile,
    /// A beyond-loaded scope is not pinned to a query snapshot.
    ScopeSnapshotMissing,
    /// A beyond-loaded scope was reached without an explicit expansion step.
    ExpansionNotExplicit,
    /// A receipt does not name the selected scope versus the all-matching scope.
    SelectedVersusMatchingNotNamed,
    /// A receipt lacks evidence refs.
    ReceiptEvidenceMissing,
    /// A deep-link snapshot fails its invariants.
    DeepLinkSnapshotInvalid,
    /// A deep-link snapshot implies frozen certainty about live results.
    DeepLinkImpliesFrozenCertainty,
    /// A deep-link snapshot hides divergence from the captured scope.
    DeepLinkDivergenceHidden,
    /// No deep-link snapshot demonstrates a diverged reopen.
    DeepLinkDivergenceCaseMissing,
    /// No deep-link snapshot demonstrates a provider reopen posture.
    DeepLinkProviderCaseMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ScopeReceiptViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::RequiredActionKindMissing => "required_action_kind_missing",
            Self::ScopeClassCoverageMissing => "scope_class_coverage_missing",
            Self::ProviderSideCaseMissing => "provider_side_case_missing",
            Self::AllMatchingCaseMissing => "all_matching_case_missing",
            Self::ReceiptInvalid => "receipt_invalid",
            Self::CountsDoNotReconcile => "counts_do_not_reconcile",
            Self::ScopeSnapshotMissing => "scope_snapshot_missing",
            Self::ExpansionNotExplicit => "expansion_not_explicit",
            Self::SelectedVersusMatchingNotNamed => "selected_versus_matching_not_named",
            Self::ReceiptEvidenceMissing => "receipt_evidence_missing",
            Self::DeepLinkSnapshotInvalid => "deep_link_snapshot_invalid",
            Self::DeepLinkImpliesFrozenCertainty => "deep_link_implies_frozen_certainty",
            Self::DeepLinkDivergenceHidden => "deep_link_divergence_hidden",
            Self::DeepLinkDivergenceCaseMissing => "deep_link_divergence_case_missing",
            Self::DeepLinkProviderCaseMissing => "deep_link_provider_case_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in scope-receipt export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_scope_receipt_export() -> Result<ScopeReceiptPacket, ScopeReceiptArtifactError> {
    let packet: ScopeReceiptPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m5/add-export-copy-rerun-suppress-install-update-delete-scope-receipts-and-saved-query-deep-l/support_export.json"
    )))
    .map_err(ScopeReceiptArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ScopeReceiptArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ScopeReceiptPacket,
    violations: &mut Vec<ScopeReceiptViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SCOPE_RECEIPT_SCHEMA_REF,
        SCOPE_RECEIPT_DOC_REF,
        SCOPE_RECEIPT_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ScopeReceiptViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(packet: &ScopeReceiptPacket, violations: &mut Vec<ScopeReceiptViolation>) {
    let surfaces = packet.represented_surfaces();
    for required in REQUIRED_RECEIPT_SURFACES {
        if !surfaces.contains(&required) {
            violations.push(ScopeReceiptViolation::RequiredSurfaceMissing);
            break;
        }
    }

    let action_kinds = packet.represented_action_kinds();
    for required in REQUIRED_RECEIPT_ACTION_KINDS {
        if !action_kinds.contains(&required) {
            violations.push(ScopeReceiptViolation::RequiredActionKindMissing);
            break;
        }
    }

    let scope_classes = packet.represented_scope_classes();
    for required in ScopeReceiptClass::ALL {
        if !scope_classes.contains(&required) {
            violations.push(ScopeReceiptViolation::ScopeClassCoverageMissing);
            break;
        }
    }

    if !scope_classes.contains(&ScopeReceiptClass::ProviderSideSelection) {
        violations.push(ScopeReceiptViolation::ProviderSideCaseMissing);
    }
    if !scope_classes.contains(&ScopeReceiptClass::AllMatchingQuery) {
        violations.push(ScopeReceiptViolation::AllMatchingCaseMissing);
    }

    if !packet
        .deep_link_snapshots
        .iter()
        .any(|snapshot| snapshot.reopen_posture.acknowledges_divergence())
    {
        violations.push(ScopeReceiptViolation::DeepLinkDivergenceCaseMissing);
    }
    if !packet
        .deep_link_snapshots
        .iter()
        .any(|snapshot| snapshot.reopen_posture == DeepLinkReopenPosture::ProviderResultsMayDiffer)
    {
        violations.push(ScopeReceiptViolation::DeepLinkProviderCaseMissing);
    }
}

fn validate_receipts(packet: &ScopeReceiptPacket, violations: &mut Vec<ScopeReceiptViolation>) {
    for receipt in &packet.receipts {
        if !receipt.is_valid() {
            violations.push(ScopeReceiptViolation::ReceiptInvalid);
        }
        if !receipt.counts.reconciles(receipt.scope_class) {
            violations.push(ScopeReceiptViolation::CountsDoNotReconcile);
        }
        if receipt.requires_query_snapshot() {
            if receipt
                .query_snapshot_id_ref
                .as_ref()
                .map_or(true, |snapshot| snapshot.trim().is_empty())
            {
                violations.push(ScopeReceiptViolation::ScopeSnapshotMissing);
            }
            if !receipt.expansion_was_explicit {
                violations.push(ScopeReceiptViolation::ExpansionNotExplicit);
            }
        }
        if !receipt.names_selected_versus_matching() {
            violations.push(ScopeReceiptViolation::SelectedVersusMatchingNotNamed);
        }
        if receipt.evidence_refs.is_empty()
            || receipt.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(ScopeReceiptViolation::ReceiptEvidenceMissing);
        }
    }
}

fn validate_snapshots(packet: &ScopeReceiptPacket, violations: &mut Vec<ScopeReceiptViolation>) {
    for snapshot in &packet.deep_link_snapshots {
        if !snapshot.is_valid() {
            violations.push(ScopeReceiptViolation::DeepLinkSnapshotInvalid);
        }
        if snapshot.implies_frozen_certainty {
            violations.push(ScopeReceiptViolation::DeepLinkImpliesFrozenCertainty);
        }
        if !snapshot.divergence_consistent() {
            violations.push(ScopeReceiptViolation::DeepLinkDivergenceHidden);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label. A generic
/// provider error must never stand in for precise scope truth.
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
            | "blocked"
            | "hidden"
            | "skipped"
            | "excluded"
            | "done"
            | "ok"
            | "success"
            | "scope"
            | "selected"
            | "all"
            | "everything"
            | "continue"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret_value")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
