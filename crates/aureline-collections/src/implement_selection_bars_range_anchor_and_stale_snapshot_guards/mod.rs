//! Selection bars, range-anchor identity, stale-query-snapshot guards, and
//! hidden-selected-count continuity for filtered, sorted, and streaming M5 dense
//! collections.
//!
//! Where
//! [`crate::ship_result_scope_counters_and_hidden_narrowing_chips`] made the
//! *result truth* (visible / loaded / matching / total, plus hidden narrowing)
//! of a dense collection explicit, and
//! [`crate::stabilize_selection_scope_and_batch_result_truth`] froze the portable
//! selection-scope and batch-review contract, this module makes the *live
//! selection state* of a dense surface a canonical product object.
//!
//! Each [`SelectionBar`] pins one [`DenseCollectionSurface`] rendered as a
//! [`CollectionViewKind`] under a [`CollectionDataMode`] (filtered/sorted,
//! streaming, virtualized, paginated, or static) to:
//!
//! - a [`SelectionMembership`] that tracks members by **stable item identity**
//!   rather than row position, so selection survives sort, filter, pagination,
//!   and virtualization;
//! - an optional [`RangeAnchor`] whose anchor and focus are stable item ids — not
//!   indices — so a shift-range selection keeps meaning after the list re-sorts or
//!   re-filters;
//! - [`SelectionBarCounts`] that keep visible-selected, hidden-selected (selected
//!   but outside the current filter), and prior-snapshot-derived selected counts
//!   distinct so a user can tell what a broad action will touch before it runs;
//!   and
//! - a [`StaleQuerySnapshotGuard`] that compares the dataset identity the
//!   selection was built against with the current one, so a materially changed
//!   query forces re-review or downgrade instead of hidden best-effort bulk
//!   behavior.
//!
//! The lane carries the guardrails the track demands: a selection bar never lets
//! a row highlight stand in for durable selection, never hides the
//! hidden-selected count, never lets a stale snapshot proceed silently, and never
//! lets a broad action bypass preview because the list is virtualized or
//! provider-backed. [`SelectionBar::reconstruction`] projects the same truth into
//! a redaction-aware [`SelectionBarReconstruction`] that diagnostics and support
//! packets reuse instead of re-deriving membership from raw rows.
//!
//! The boundary schema is
//! [`schemas/collections/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.schema.json`](../../../../schemas/collections/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.schema.json).
//! The contract doc is
//! [`docs/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.md`](../../../../docs/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.md).
//! The protected fixture directory is
//! [`fixtures/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele/`](../../../../fixtures/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::DenseCollectionSurface;
use crate::ship_result_scope_counters_and_hidden_narrowing_chips::CollectionViewKind;

/// Stable record-kind tag carried by [`SelectionBarContinuityPacket`].
pub const SELECTION_BAR_CONTINUITY_RECORD_KIND: &str = "m5_selection_bar_continuity_packet";

/// Integer schema version for the selection-bar continuity packet.
pub const SELECTION_BAR_CONTINUITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SELECTION_BAR_CONTINUITY_SCHEMA_REF: &str =
    "schemas/collections/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.schema.json";

/// Repo-relative path of the contract doc.
pub const SELECTION_BAR_CONTINUITY_DOC_REF: &str =
    "docs/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.md";

/// Repo-relative path of the protected fixture directory.
pub const SELECTION_BAR_CONTINUITY_FIXTURE_DIR: &str =
    "fixtures/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele";

/// Repo-relative path of the checked support-export artifact.
pub const SELECTION_BAR_CONTINUITY_ARTIFACT_REF: &str =
    "artifacts/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SELECTION_BAR_CONTINUITY_SUMMARY_REF: &str =
    "artifacts/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele.md";

/// The first real M5 dense surfaces this lane wires onto canonical selection
/// bars.
const REQUIRED_BAR_SURFACES: [DenseCollectionSurface; 6] = [
    DenseCollectionSurface::PipelineRunList,
    DenseCollectionSurface::ReviewQueue,
    DenseCollectionSurface::IncidentList,
    DenseCollectionSurface::GraphList,
    DenseCollectionSurface::MarketplaceResults,
    DenseCollectionSurface::ProviderAdminTable,
];

/// Data modes the selection bar must survive. A bar must normalize identically
/// whether the collection is filtered/sorted, streaming, or virtualized.
const REQUIRED_DATA_MODES: [CollectionDataMode; 3] = [
    CollectionDataMode::FilteredSorted,
    CollectionDataMode::Streaming,
    CollectionDataMode::Virtualized,
];

/// How a dense collection is materialized while a selection is held. The
/// selection-bar contract is identical across modes so a filtered/sorted list, a
/// streaming queue, and a virtualized table never fork selection semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionDataMode {
    /// A static, fully materialized client dataset.
    StaticComplete,
    /// Rows reordered and narrowed by client-side sort and filter.
    FilteredSorted,
    /// Rows arrive live during active selection.
    Streaming,
    /// Only a viewport window of rows is materialized at a time.
    Virtualized,
    /// Rows are paged behind a provider.
    Paginated,
}

impl CollectionDataMode {
    /// Every data mode the lane recognizes.
    pub const ALL: [Self; 5] = [
        Self::StaticComplete,
        Self::FilteredSorted,
        Self::Streaming,
        Self::Virtualized,
        Self::Paginated,
    ];

    /// Stable token recorded in packets, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaticComplete => "static_complete",
            Self::FilteredSorted => "filtered_sorted",
            Self::Streaming => "streaming",
            Self::Virtualized => "virtualized",
            Self::Paginated => "paginated",
        }
    }
}

/// How the membership of a selection is defined.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelectionMembershipBasis {
    /// An explicit set of stable item ids.
    StableIdentitySet,
    /// A shift-range expansion anchored on a stable item id.
    RangeAnchorExpansion,
    /// All rows matching a reviewed query snapshot.
    QuerySnapshot,
}

impl SelectionMembershipBasis {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableIdentitySet => "stable_identity_set",
            Self::RangeAnchorExpansion => "range_anchor_expansion",
            Self::QuerySnapshot => "query_snapshot",
        }
    }

    /// True when the basis is backed by a reviewed query snapshot.
    pub const fn is_query_backed(self) -> bool {
        matches!(self, Self::QuerySnapshot)
    }
}

/// How the dataset identity changed between the moment a selection was built and
/// the moment a broad action wants to run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DatasetIdentityChange {
    /// The dataset identity is byte-identical; nothing changed.
    Unchanged,
    /// Only the order of rows changed; membership by identity is unaffected.
    ReorderedOnly,
    /// Rows were added to or removed from the matching set.
    RowsAddedOrRemoved,
    /// The underlying query or filter definition changed.
    QueryRedefined,
    /// The provider advanced to a new epoch / snapshot generation.
    ProviderEpochChanged,
}

impl DatasetIdentityChange {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchanged => "unchanged",
            Self::ReorderedOnly => "reordered_only",
            Self::RowsAddedOrRemoved => "rows_added_or_removed",
            Self::QueryRedefined => "query_redefined",
            Self::ProviderEpochChanged => "provider_epoch_changed",
        }
    }

    /// True when the change materially alters the matching set so a broad action
    /// cannot run against the prior snapshot without review.
    pub const fn is_material(self) -> bool {
        matches!(
            self,
            Self::RowsAddedOrRemoved | Self::QueryRedefined | Self::ProviderEpochChanged
        )
    }
}

/// What the stale-query-snapshot guard decided a broad action may do.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StaleGuardOutcome {
    /// The dataset is fresh for this selection; the action may proceed.
    ProceedFresh,
    /// The dataset changed materially; the selection must be reopened and
    /// re-reviewed before the action runs.
    RequireReopenReview,
    /// The action is narrowed to the still-visible, still-matching members only.
    DowngradeToVisibleOnly,
    /// The action is blocked until the client re-syncs to the current dataset.
    BlockUntilResynced,
}

impl StaleGuardOutcome {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProceedFresh => "proceed_fresh",
            Self::RequireReopenReview => "require_reopen_review",
            Self::DowngradeToVisibleOnly => "downgrade_to_visible_only",
            Self::BlockUntilResynced => "block_until_resynced",
        }
    }

    /// True when the outcome lets a broad action run without re-review.
    pub const fn is_proceed(self) -> bool {
        matches!(self, Self::ProceedFresh)
    }
}

/// Stable identity of one selected item. Membership is tracked by
/// [`StableSelectionItem::stable_item_id`], never by row position, so selection
/// survives sort, filter, pagination, and virtualization.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StableSelectionItem {
    /// Stable item identity used across sort, filter, pagination, virtualization.
    pub stable_item_id: String,
    /// Redaction-aware label that never carries raw row bodies or query text.
    pub review_label: String,
    /// True when the item is still inside the current filtered/sorted view.
    pub in_current_filter: bool,
}

/// Membership of a selection, defined by stable item identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionMembership {
    /// How the membership is defined.
    pub basis: SelectionMembershipBasis,
    /// True when membership is tracked by stable item identity, not row position
    /// (required).
    pub by_stable_identity: bool,
    /// Explicitly enumerated stable members. May be a representative sample of a
    /// larger query-backed selection.
    pub members: Vec<StableSelectionItem>,
    /// Query snapshot ref, required when the basis is a query snapshot.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_snapshot_id_ref: Option<String>,
}

impl SelectionMembership {
    /// Whether the membership is well formed: tracked by stable identity, every
    /// member carries a stable id and a precise label, and a query-backed basis
    /// carries its snapshot ref.
    pub fn is_valid(&self) -> bool {
        if !self.by_stable_identity {
            return false;
        }
        if self.basis.is_query_backed() && self.query_snapshot_id_ref.is_none() {
            return false;
        }
        let ids: BTreeSet<&str> = self
            .members
            .iter()
            .map(|member| member.stable_item_id.as_str())
            .collect();
        if ids.len() != self.members.len() {
            return false;
        }
        self.members.iter().all(|member| {
            !member.stable_item_id.trim().is_empty() && !label_is_generic(&member.review_label)
        })
    }
}

/// A shift-range anchor pinned to a stable item id rather than a row index, so a
/// range selection keeps meaning after the list re-sorts or re-filters.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RangeAnchor {
    /// Stable item id of the range anchor (where the range started).
    pub anchor_item_id: String,
    /// Stable item id of the current range focus (where the range extends to).
    pub focus_item_id: String,
    /// True when the anchor and focus are stable item ids, not row positions
    /// (required).
    pub anchored_by_stable_identity: bool,
    /// True when the anchor item is still present in the current filtered/sorted
    /// view.
    pub anchor_still_present: bool,
    /// True when the range walks visible traversal order (collapsed descendants
    /// excluded by default) (required).
    pub visible_traversal_order: bool,
    /// Precise label describing how the range re-resolves when the anchor leaves
    /// the current view, required whenever the anchor is no longer present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reresolution_label: Option<String>,
}

impl RangeAnchor {
    /// Whether the anchor is well formed: anchored by stable identity, walking
    /// visible traversal order, with non-empty ids, and carrying a precise
    /// re-resolution label whenever the anchor has left the current view.
    pub fn is_valid(&self) -> bool {
        if self.anchor_item_id.trim().is_empty() || self.focus_item_id.trim().is_empty() {
            return false;
        }
        if !self.anchored_by_stable_identity || !self.visible_traversal_order {
            return false;
        }
        if !self.anchor_still_present {
            return self
                .reresolution_label
                .as_ref()
                .is_some_and(|label| !label_is_generic(label));
        }
        true
    }
}

/// Selection counts that keep visible, hidden, and prior-snapshot selected
/// populations distinct so a broad action's scope is legible before it runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionBarCounts {
    /// Total selected by stable identity for the stated scope.
    pub selected_total: u64,
    /// Selected items currently inside the filtered/sorted view.
    pub selected_visible: u64,
    /// Hidden-selected: selected items outside the current filter.
    pub selected_outside_filter: u64,
    /// Selected items carried from a prior query snapshot rather than the live
    /// matching set.
    pub selected_from_prior_snapshot: u64,
    /// Selected items currently blocked from the action.
    pub selected_blocked: u64,
}

impl SelectionBarCounts {
    /// Whether the counts reconcile: total splits into visible plus
    /// outside-filter, and the snapshot-derived and blocked sub-counts do not
    /// exceed the total.
    pub fn reconciles(&self) -> bool {
        self.selected_visible + self.selected_outside_filter == self.selected_total
            && self.selected_from_prior_snapshot <= self.selected_total
            && self.selected_blocked <= self.selected_total
    }

    /// True when some selected items are hidden (outside the current filter) or
    /// carried from a prior snapshot — the continuity a broad action must surface.
    pub fn has_hidden_or_snapshot_selection(&self) -> bool {
        self.selected_outside_filter > 0 || self.selected_from_prior_snapshot > 0
    }
}

/// The stale-query-snapshot guard. Compares the dataset identity the selection
/// was built against with the current identity and records what a broad action
/// may do.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleQuerySnapshotGuard {
    /// Dataset identity captured when the selection was reviewed.
    pub selection_dataset_identity: String,
    /// Dataset identity at the moment the action wants to run.
    pub current_dataset_identity: String,
    /// Query snapshot the selection was reviewed against, if query-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub query_snapshot_id_ref: Option<String>,
    /// How the dataset identity changed.
    pub dataset_identity_change: DatasetIdentityChange,
    /// What the guard decided a broad action may do.
    pub guard_outcome: StaleGuardOutcome,
    /// True when a broad/destructive action cannot bypass review or preview just
    /// because the list is virtualized or provider-backed (required).
    pub broad_action_cannot_bypass_preview: bool,
    /// Precise guidance shown to the operator, required whenever the outcome is
    /// not [`StaleGuardOutcome::ProceedFresh`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guidance_label: Option<String>,
}

impl StaleQuerySnapshotGuard {
    /// True when the guard reports a materially changed dataset.
    pub fn is_stale(&self) -> bool {
        self.dataset_identity_change.is_material()
            || self.selection_dataset_identity != self.current_dataset_identity
                && self.dataset_identity_change != DatasetIdentityChange::ReorderedOnly
    }

    /// Whether the guard is internally consistent. The core invariant: a material
    /// dataset change must never resolve to "proceed" — it must reopen review,
    /// downgrade scope, or block — and a non-proceed outcome must carry precise
    /// guidance. Broad actions can never bypass preview.
    pub fn is_consistent(&self) -> bool {
        if self.selection_dataset_identity.trim().is_empty()
            || self.current_dataset_identity.trim().is_empty()
        {
            return false;
        }
        if !self.broad_action_cannot_bypass_preview {
            return false;
        }
        // A material change can never silently proceed.
        if self.dataset_identity_change.is_material() && self.guard_outcome.is_proceed() {
            return false;
        }
        // Differing identities that are not a pure reorder cannot silently
        // proceed either.
        if self.selection_dataset_identity != self.current_dataset_identity
            && self.dataset_identity_change == DatasetIdentityChange::Unchanged
        {
            return false;
        }
        // Unchanged + identical identity must proceed (no spurious downgrade).
        if self.dataset_identity_change == DatasetIdentityChange::Unchanged
            && self.selection_dataset_identity == self.current_dataset_identity
            && !self.guard_outcome.is_proceed()
        {
            return false;
        }
        if !self.guard_outcome.is_proceed() {
            return self
                .guidance_label
                .as_ref()
                .is_some_and(|label| !label_is_generic(label));
        }
        true
    }
}

/// Redaction-aware projection of one selection bar for diagnostics and support
/// packets. Carries only ids, tokens, labels, and counts — never raw row bodies
/// or provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionBarReconstruction {
    /// Bar id this reconstruction projects.
    pub bar_id: String,
    /// Surface token.
    pub surface_token: String,
    /// View-kind token.
    pub view_kind_token: String,
    /// Data-mode token.
    pub data_mode_token: String,
    /// Membership-basis token.
    pub membership_basis_token: String,
    /// True when membership is tracked by stable identity.
    pub by_stable_identity: bool,
    /// Total selected.
    pub selected_total: u64,
    /// Visible-selected.
    pub selected_visible: u64,
    /// Hidden-selected (outside current filter).
    pub selected_outside_filter: u64,
    /// Prior-snapshot-derived selected.
    pub selected_from_prior_snapshot: u64,
    /// True when a range anchor is held.
    pub has_range_anchor: bool,
    /// True when a held range anchor is still present in the current view.
    pub range_anchor_present: bool,
    /// Dataset-identity-change token.
    pub dataset_identity_change_token: String,
    /// Stale-guard-outcome token.
    pub guard_outcome_token: String,
    /// True when the guard reports a materially changed dataset.
    pub is_stale: bool,
}

/// One selection bar for a dense M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionBar {
    /// Stable bar id.
    pub bar_id: String,
    /// Bound dense collection surface.
    pub surface: DenseCollectionSurface,
    /// How the surface is rendered.
    pub view_kind: CollectionViewKind,
    /// How the collection is materialized while the selection is held.
    pub data_mode: CollectionDataMode,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Membership tracked by stable item identity.
    pub membership: SelectionMembership,
    /// Range anchor, when a shift-range selection is held.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub range_anchor: Option<RangeAnchor>,
    /// Visible / hidden / snapshot-derived selected counts.
    pub counts: SelectionBarCounts,
    /// Stale-query-snapshot guard.
    pub snapshot_guard: StaleQuerySnapshotGuard,
    /// True when the selection survives sort, filter, and virtualization by stable
    /// identity (required).
    pub survives_sort_filter_virtualization: bool,
    /// Screen-reader and keyboard-safe summary label.
    pub accessibility_summary: String,
    /// Evidence packet refs backing this bar.
    pub evidence_refs: Vec<String>,
}

impl SelectionBar {
    /// Whether the selected count is at least the number of enumerated members
    /// (a sample may be smaller than the true total, never larger).
    pub fn count_truth_holds(&self) -> bool {
        self.counts.selected_total >= self.membership.members.len() as u64
    }

    /// Whether the hidden-selected continuity is disclosed: when selected items
    /// are outside the current filter or carried from a prior snapshot, the
    /// accessibility summary names that fact so a broad action's scope is legible.
    pub fn hidden_selection_disclosed(&self) -> bool {
        if !self.counts.has_hidden_or_snapshot_selection() {
            return true;
        }
        let lower = self.accessibility_summary.to_lowercase();
        lower.contains("outside")
            || lower.contains("hidden")
            || lower.contains("prior snapshot")
            || lower.contains("snapshot")
    }

    /// Whether membership outside the current filter agrees with the counts: a
    /// non-zero hidden-selected count means at least one enumerated member is out
    /// of filter, when members are enumerated.
    pub fn membership_matches_counts(&self) -> bool {
        if self.membership.members.is_empty() {
            return true;
        }
        let any_outside = self
            .membership
            .members
            .iter()
            .any(|member| !member.in_current_filter);
        if self.counts.selected_outside_filter > 0 {
            any_outside
        } else {
            !any_outside
        }
    }

    /// Whether every dimension required to record this bar is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.bar_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.accessibility_summary.trim().is_empty()
            && self.membership.is_valid()
            && self
                .range_anchor
                .as_ref()
                .map_or(true, RangeAnchor::is_valid)
            && self.counts.reconciles()
            && self.count_truth_holds()
            && self.membership_matches_counts()
            && self.hidden_selection_disclosed()
            && self.snapshot_guard.is_consistent()
            && self.survives_sort_filter_virtualization
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Projects the bar into a redaction-aware reconstruction for diagnostics and
    /// support packets.
    pub fn reconstruction(&self) -> SelectionBarReconstruction {
        SelectionBarReconstruction {
            bar_id: self.bar_id.clone(),
            surface_token: self.surface.as_str().to_owned(),
            view_kind_token: self.view_kind.as_str().to_owned(),
            data_mode_token: self.data_mode.as_str().to_owned(),
            membership_basis_token: self.membership.basis.as_str().to_owned(),
            by_stable_identity: self.membership.by_stable_identity,
            selected_total: self.counts.selected_total,
            selected_visible: self.counts.selected_visible,
            selected_outside_filter: self.counts.selected_outside_filter,
            selected_from_prior_snapshot: self.counts.selected_from_prior_snapshot,
            has_range_anchor: self.range_anchor.is_some(),
            range_anchor_present: self
                .range_anchor
                .as_ref()
                .is_some_and(|anchor| anchor.anchor_still_present),
            dataset_identity_change_token: self
                .snapshot_guard
                .dataset_identity_change
                .as_str()
                .to_owned(),
            guard_outcome_token: self.snapshot_guard.guard_outcome.as_str().to_owned(),
            is_stale: self.snapshot_guard.is_stale(),
        }
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionBarGuardrails {
    /// Selection survives sort, filter, pagination, and virtualization by stable
    /// identity.
    pub selection_survives_sort_filter_virtualization: bool,
    /// The hidden-selected count is always visible before a broad action.
    pub hidden_selected_count_always_visible: bool,
    /// A stale snapshot triggers review or downgrade, never hidden best-effort.
    pub stale_snapshot_triggers_review_or_downgrade: bool,
    /// A broad action cannot bypass preview because the list is virtualized or
    /// provider-backed.
    pub broad_action_cannot_bypass_preview: bool,
    /// A range anchor is held by stable identity, not row position.
    pub range_anchor_by_stable_identity: bool,
}

impl SelectionBarGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.selection_survives_sort_filter_virtualization
            && self.hidden_selected_count_always_visible
            && self.stale_snapshot_triggers_review_or_downgrade
            && self.broad_action_cannot_bypass_preview
            && self.range_anchor_by_stable_identity
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionBarConsumerProjection {
    /// Product renders the selection bar and its counts from these records.
    pub product_renders_selection_bar: bool,
    /// Diagnostics reconstruct selection truth from these records.
    pub diagnostics_reconstructs_selection_truth: bool,
    /// Support/export reuses the selection-truth projection.
    pub support_export_reuses_records: bool,
    /// Docs and help reuse the selection-bar vocabulary.
    pub docs_help_reuses_vocabulary: bool,
}

impl SelectionBarConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_renders_selection_bar
            && self.diagnostics_reconstructs_selection_truth
            && self.support_export_reuses_records
            && self.docs_help_reuses_vocabulary
    }
}

/// Constructor input for [`SelectionBarContinuityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelectionBarContinuityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-surface selection bars.
    pub bars: Vec<SelectionBar>,
    /// Guardrail invariants block.
    pub guardrails: SelectionBarGuardrails,
    /// Consumer projection block.
    pub consumer_projection: SelectionBarConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe selection-bar continuity packet for the first real M5 dense
/// surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionBarContinuityPacket {
    /// Record kind; must equal [`SELECTION_BAR_CONTINUITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SELECTION_BAR_CONTINUITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-surface selection bars.
    pub bars: Vec<SelectionBar>,
    /// Guardrail invariants block.
    pub guardrails: SelectionBarGuardrails,
    /// Consumer projection block.
    pub consumer_projection: SelectionBarConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl SelectionBarContinuityPacket {
    /// Builds a selection-bar continuity packet.
    pub fn new(input: SelectionBarContinuityPacketInput) -> Self {
        Self {
            record_kind: SELECTION_BAR_CONTINUITY_RECORD_KIND.to_owned(),
            schema_version: SELECTION_BAR_CONTINUITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            bars: input.bars,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some bar in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<DenseCollectionSurface> {
        self.bars.iter().map(|bar| bar.surface).collect()
    }

    /// View kinds represented by some bar in this packet.
    pub fn represented_view_kinds(&self) -> BTreeSet<CollectionViewKind> {
        self.bars.iter().map(|bar| bar.view_kind).collect()
    }

    /// Data modes represented by some bar in this packet.
    pub fn represented_data_modes(&self) -> BTreeSet<CollectionDataMode> {
        self.bars.iter().map(|bar| bar.data_mode).collect()
    }

    /// Count of bars whose guard reports a stale dataset.
    pub fn stale_bar_count(&self) -> usize {
        self.bars
            .iter()
            .filter(|bar| bar.snapshot_guard.is_stale())
            .count()
    }

    /// Reconstructions for every bar, used by diagnostics and support packets.
    pub fn reconstructions(&self) -> Vec<SelectionBarReconstruction> {
        self.bars.iter().map(SelectionBar::reconstruction).collect()
    }

    /// Validates the selection-bar continuity packet invariants.
    pub fn validate(&self) -> Vec<SelectionBarContinuityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SELECTION_BAR_CONTINUITY_RECORD_KIND {
            violations.push(SelectionBarContinuityViolation::WrongRecordKind);
        }
        if self.schema_version != SELECTION_BAR_CONTINUITY_SCHEMA_VERSION {
            violations.push(SelectionBarContinuityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(SelectionBarContinuityViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_bars(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(SelectionBarContinuityViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(SelectionBarContinuityViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("selection bar continuity packet serializes"),
        ) {
            violations.push(SelectionBarContinuityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("selection bar continuity packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Selection Bars And Stale-Query-Snapshot Guards\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Bars: {} ({} stale)\n",
            self.bars.len(),
            self.stale_bar_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            REQUIRED_BAR_SURFACES.len()
        ));
        out.push_str(&format!(
            "- View kinds: {} / {}\n",
            self.represented_view_kinds().len(),
            CollectionViewKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Data modes: {} / {}\n",
            self.represented_data_modes().len(),
            CollectionDataMode::ALL.len()
        ));
        out.push_str("\n## Bars\n\n");
        for bar in &self.bars {
            out.push_str(&format!(
                "- **{}** ({} / {} / {}): {}\n",
                bar.bar_id,
                bar.surface.as_str(),
                bar.view_kind.as_str(),
                bar.data_mode.as_str(),
                bar.label_summary,
            ));
            out.push_str(&format!(
                "  - selected={} visible={} outside_filter={} prior_snapshot={} blocked={}\n",
                bar.counts.selected_total,
                bar.counts.selected_visible,
                bar.counts.selected_outside_filter,
                bar.counts.selected_from_prior_snapshot,
                bar.counts.selected_blocked,
            ));
            if let Some(anchor) = &bar.range_anchor {
                out.push_str(&format!(
                    "  - range anchor `{}` -> `{}` (present={})\n",
                    anchor.anchor_item_id, anchor.focus_item_id, anchor.anchor_still_present,
                ));
            }
            out.push_str(&format!(
                "  - guard: change=`{}` outcome=`{}`\n",
                bar.snapshot_guard.dataset_identity_change.as_str(),
                bar.snapshot_guard.guard_outcome.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in selection-bar continuity export.
#[derive(Debug)]
pub enum SelectionBarContinuityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<SelectionBarContinuityViolation>),
}

impl fmt::Display for SelectionBarContinuityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "selection bar continuity export parse failed: {error}"
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
                    "selection bar continuity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for SelectionBarContinuityArtifactError {}

/// Validation failures emitted by [`SelectionBarContinuityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SelectionBarContinuityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required real M5 surface is bound by no selection bar.
    RequiredSurfaceMissing,
    /// A required view kind (list, tree, table, queue) is represented by no bar.
    RequiredViewKindMissing,
    /// A required data mode (filtered/sorted, streaming, virtualized) is
    /// represented by no bar.
    RequiredDataModeMissing,
    /// No bar demonstrates a hidden-selected (outside-filter) count.
    HiddenSelectedCaseMissing,
    /// No bar demonstrates a prior-snapshot-derived selection.
    PriorSnapshotCaseMissing,
    /// No bar demonstrates the stale-snapshot guard path.
    StaleGuardCaseMissing,
    /// No bar demonstrates a held range anchor.
    RangeAnchorCaseMissing,
    /// A bar is incomplete.
    BarIncomplete,
    /// A bar tracks selection by row position instead of stable identity.
    MembershipNotByStableIdentity,
    /// A bar's range anchor is not held by stable identity.
    RangeAnchorNotByStableIdentity,
    /// A bar's selected counts do not reconcile.
    CountsDoNotReconcile,
    /// A bar hides the hidden-selected or prior-snapshot continuity.
    HiddenSelectionUndisclosed,
    /// A bar lets a stale snapshot proceed without review or downgrade.
    StaleSnapshotProceedsSilently,
    /// A bar lets a broad action bypass preview.
    BroadActionBypassesPreview,
    /// A bar lacks evidence refs.
    BarEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl SelectionBarContinuityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::RequiredViewKindMissing => "required_view_kind_missing",
            Self::RequiredDataModeMissing => "required_data_mode_missing",
            Self::HiddenSelectedCaseMissing => "hidden_selected_case_missing",
            Self::PriorSnapshotCaseMissing => "prior_snapshot_case_missing",
            Self::StaleGuardCaseMissing => "stale_guard_case_missing",
            Self::RangeAnchorCaseMissing => "range_anchor_case_missing",
            Self::BarIncomplete => "bar_incomplete",
            Self::MembershipNotByStableIdentity => "membership_not_by_stable_identity",
            Self::RangeAnchorNotByStableIdentity => "range_anchor_not_by_stable_identity",
            Self::CountsDoNotReconcile => "counts_do_not_reconcile",
            Self::HiddenSelectionUndisclosed => "hidden_selection_undisclosed",
            Self::StaleSnapshotProceedsSilently => "stale_snapshot_proceeds_silently",
            Self::BroadActionBypassesPreview => "broad_action_bypasses_preview",
            Self::BarEvidenceMissing => "bar_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in selection-bar continuity export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_selection_bar_continuity_export(
) -> Result<SelectionBarContinuityPacket, SelectionBarContinuityArtifactError> {
    let packet: SelectionBarContinuityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m5/implement-selection-bars-range-anchor-identity-stale-query-snapshot-guards-and-hidden-sele/support_export.json"
    )))
    .map_err(SelectionBarContinuityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(SelectionBarContinuityArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &SelectionBarContinuityPacket,
    violations: &mut Vec<SelectionBarContinuityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SELECTION_BAR_CONTINUITY_SCHEMA_REF,
        SELECTION_BAR_CONTINUITY_DOC_REF,
        SELECTION_BAR_CONTINUITY_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(SelectionBarContinuityViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &SelectionBarContinuityPacket,
    violations: &mut Vec<SelectionBarContinuityViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in REQUIRED_BAR_SURFACES {
        if !surfaces.contains(&required) {
            violations.push(SelectionBarContinuityViolation::RequiredSurfaceMissing);
            break;
        }
    }

    let view_kinds = packet.represented_view_kinds();
    for required in CollectionViewKind::ALL {
        if !view_kinds.contains(&required) {
            violations.push(SelectionBarContinuityViolation::RequiredViewKindMissing);
            break;
        }
    }

    let data_modes = packet.represented_data_modes();
    for required in REQUIRED_DATA_MODES {
        if !data_modes.contains(&required) {
            violations.push(SelectionBarContinuityViolation::RequiredDataModeMissing);
            break;
        }
    }

    if !packet
        .bars
        .iter()
        .any(|bar| bar.counts.selected_outside_filter > 0)
    {
        violations.push(SelectionBarContinuityViolation::HiddenSelectedCaseMissing);
    }
    if !packet
        .bars
        .iter()
        .any(|bar| bar.counts.selected_from_prior_snapshot > 0)
    {
        violations.push(SelectionBarContinuityViolation::PriorSnapshotCaseMissing);
    }
    if !packet
        .bars
        .iter()
        .any(|bar| bar.snapshot_guard.is_stale() && !bar.snapshot_guard.guard_outcome.is_proceed())
    {
        violations.push(SelectionBarContinuityViolation::StaleGuardCaseMissing);
    }
    if !packet.bars.iter().any(|bar| bar.range_anchor.is_some()) {
        violations.push(SelectionBarContinuityViolation::RangeAnchorCaseMissing);
    }
}

fn validate_bars(
    packet: &SelectionBarContinuityPacket,
    violations: &mut Vec<SelectionBarContinuityViolation>,
) {
    for bar in &packet.bars {
        if !bar.is_complete() {
            violations.push(SelectionBarContinuityViolation::BarIncomplete);
        }
        if !bar.membership.by_stable_identity {
            violations.push(SelectionBarContinuityViolation::MembershipNotByStableIdentity);
        }
        if let Some(anchor) = &bar.range_anchor {
            if !anchor.anchored_by_stable_identity {
                violations.push(SelectionBarContinuityViolation::RangeAnchorNotByStableIdentity);
            }
        }
        if !bar.counts.reconciles() {
            violations.push(SelectionBarContinuityViolation::CountsDoNotReconcile);
        }
        if !bar.hidden_selection_disclosed() {
            violations.push(SelectionBarContinuityViolation::HiddenSelectionUndisclosed);
        }
        if bar.snapshot_guard.dataset_identity_change.is_material()
            && bar.snapshot_guard.guard_outcome.is_proceed()
        {
            violations.push(SelectionBarContinuityViolation::StaleSnapshotProceedsSilently);
        }
        if !bar.snapshot_guard.broad_action_cannot_bypass_preview {
            violations.push(SelectionBarContinuityViolation::BroadActionBypassesPreview);
        }
        if bar.evidence_refs.is_empty() || bar.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(SelectionBarContinuityViolation::BarEvidenceMissing);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label. A generic
/// provider error must never stand in for precise selection truth.
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
            | "hidden"
            | "stale"
            | "changed"
            | "review"
            | "selected"
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
