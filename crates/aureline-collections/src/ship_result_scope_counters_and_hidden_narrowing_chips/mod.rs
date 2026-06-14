//! Result-scope counters and hidden-narrowing chips for the first real M5 dense
//! collection surfaces.
//!
//! Where
//! [`crate::implement_filter_asts_saved_views_column_presets_and_privacy_scoped_persistence`]
//! made the *saved filter/view/column* state durable, this module makes the
//! *result truth* of a dense collection explicit: how many rows are visible,
//! loaded, matching, and total; whether each of those numbers is exact or
//! approximate; whether it is fresh, stale, or partial; and how many rows are
//! hidden by policy, workset, provider, client, or partial-data limits.
//!
//! Each [`ResultScopeCounterBinding`] pins one [`DenseCollectionSurface`] rendered
//! as a [`CollectionViewKind`] (list, tree, table, or queue) to a normalized set
//! of [`ResultScopeCount`]s and [`HiddenNarrowingChip`]s. The same counter and
//! chip vocabulary is used for every surface kind so a virtualized list, a
//! provider-backed table, and a streaming queue never invent surface-local
//! wording for "visible" versus "loaded" versus "all matching".
//!
//! The lane carries the guardrails the track demands. Hidden narrowing is always
//! representable next to the active filters via a [`HiddenNarrowingChip`] with a
//! precise [`NarrowingCause`] — provider and policy narrowing get their own chip
//! rather than disappearing into a generic filter pill. Exact and approximate
//! counts are distinguished by [`CountExactness`], stale and partial data by
//! [`CountFreshness`], and the [`CounterPlacement`] keeps the truth near the
//! filters instead of in footer-only or provider-private text.
//! [`ResultScopeCounterBinding::reconstruction`] projects the same truth into a
//! redaction-aware [`ResultScopeReconstruction`] that diagnostics and support
//! packets reuse instead of re-deriving counts from raw rows.
//!
//! The boundary schema is
//! [`schemas/collections/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.schema.json`](../../../../schemas/collections/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.schema.json).
//! The contract doc is
//! [`docs/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.md`](../../../../docs/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.md).
//! The protected fixture directory is
//! [`fixtures/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver/`](../../../../fixtures/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::DenseCollectionSurface;
use crate::stabilize_filter_ast_saved_view_scope_pack_column_preset::{
    ScopeCounterVocabularyTerm, SelectAllMeaning,
};

/// Stable record-kind tag carried by [`ResultScopeCounterPacket`].
pub const RESULT_SCOPE_COUNTER_RECORD_KIND: &str = "m5_result_scope_counter_packet";

/// Integer schema version for the result-scope counter packet.
pub const RESULT_SCOPE_COUNTER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RESULT_SCOPE_COUNTER_SCHEMA_REF: &str =
    "schemas/collections/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.schema.json";

/// Repo-relative path of the contract doc.
pub const RESULT_SCOPE_COUNTER_DOC_REF: &str =
    "docs/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.md";

/// Repo-relative path of the protected fixture directory.
pub const RESULT_SCOPE_COUNTER_FIXTURE_DIR: &str =
    "fixtures/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver";

/// Repo-relative path of the checked support-export artifact.
pub const RESULT_SCOPE_COUNTER_ARTIFACT_REF: &str =
    "artifacts/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const RESULT_SCOPE_COUNTER_SUMMARY_REF: &str =
    "artifacts/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver.md";

/// Canonical scope-counter vocabulary every binding must keep so the visible /
/// loaded / matching / selected distinctions never blur across surface kinds.
const REQUIRED_SCOPE_VOCABULARY_TERMS: [ScopeCounterVocabularyTerm; 8] = [
    ScopeCounterVocabularyTerm::Visible,
    ScopeCounterVocabularyTerm::Loaded,
    ScopeCounterVocabularyTerm::Matching,
    ScopeCounterVocabularyTerm::Selected,
    ScopeCounterVocabularyTerm::Approx,
    ScopeCounterVocabularyTerm::Exact,
    ScopeCounterVocabularyTerm::HiddenByPolicy,
    ScopeCounterVocabularyTerm::OutsideCurrentFilter,
];

/// The first real M5 dense surfaces this lane wires onto canonical counters.
const REQUIRED_COUNTER_SURFACES: [DenseCollectionSurface; 6] = [
    DenseCollectionSurface::PipelineRunList,
    DenseCollectionSurface::ReviewQueue,
    DenseCollectionSurface::IncidentList,
    DenseCollectionSurface::GraphList,
    DenseCollectionSurface::MarketplaceResults,
    DenseCollectionSurface::ProviderAdminTable,
];

/// Count kinds that must appear on every binding so the visible / loaded /
/// matching / total distinctions are always available, never implied.
const REQUIRED_COUNT_KINDS: [ResultCountKind; 4] = [
    ResultCountKind::Visible,
    ResultCountKind::Loaded,
    ResultCountKind::Matching,
    ResultCountKind::Total,
];

/// The way a dense collection is rendered. The counter and chip vocabulary is
/// identical across kinds so virtualized lists, trees, tables, and queues never
/// fork result-truth wording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionViewKind {
    /// Flat virtualized list / feed.
    List,
    /// Hierarchical virtualized tree / outline.
    Tree,
    /// Column-oriented virtualized table / data grid.
    Table,
    /// Work queue with claim / triage semantics.
    Queue,
}

impl CollectionViewKind {
    /// Every view kind the lane must normalize onto the shared vocabulary.
    pub const ALL: [Self; 4] = [Self::List, Self::Tree, Self::Table, Self::Queue];

    /// Stable token recorded in packets, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::List => "list",
            Self::Tree => "tree",
            Self::Table => "table",
            Self::Queue => "queue",
        }
    }
}

/// Which population a [`ResultScopeCount`] measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultCountKind {
    /// Rows currently rendered in the viewport / page window.
    Visible,
    /// Rows materialized into the client.
    Loaded,
    /// Rows matching the active filter or authoritative query.
    Matching,
    /// Rows in the full collection before narrowing.
    Total,
    /// Rows selected by stable identity or an explicit query snapshot.
    Selected,
    /// Rows hidden from the matching set by policy, workset, provider, client, or
    /// partial-data narrowing.
    HiddenByScope,
}

impl ResultCountKind {
    /// Stable token recorded in packets, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Visible => "visible",
            Self::Loaded => "loaded",
            Self::Matching => "matching",
            Self::Total => "total",
            Self::Selected => "selected",
            Self::HiddenByScope => "hidden_by_scope",
        }
    }
}

/// Whether a count is exact for its stated scope or an approximation that must be
/// labeled as such.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountExactness {
    /// The value is exact for the stated count kind.
    Exact,
    /// The value is approximate and must be rendered with an approximate marker.
    Approximate,
}

impl CountExactness {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Approximate => "approximate",
        }
    }

    /// True when the value is exact.
    pub const fn is_exact(self) -> bool {
        matches!(self, Self::Exact)
    }
}

/// Whether a count reflects current data, stale data, or a partially loaded set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CountFreshness {
    /// The value reflects the current authoritative data.
    Fresh,
    /// The value is from a prior fetch and may be out of date.
    Stale,
    /// The value reflects a partially loaded or still-streaming set.
    Partial,
}

impl CountFreshness {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
            Self::Partial => "partial",
        }
    }

    /// True when the value reflects the current authoritative data.
    pub const fn is_fresh(self) -> bool {
        matches!(self, Self::Fresh)
    }
}

/// The dataset posture a binding renders under. Drives which counts may be
/// approximate or non-fresh and whether hidden narrowing must be disclosed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultScopePosture {
    /// The client holds a complete, exact, current dataset.
    ClientComplete,
    /// A narrow client renders a windowed slice of a larger known set.
    NarrowClientWindowed,
    /// Results are paged or sampled behind a provider; the total is not local.
    ProviderPaginated,
    /// Some rows are not yet loaded; counts are partial pending more data.
    PartialDataPending,
    /// Rows arrive live during review; matching and total can move.
    StreamingLive,
}

impl ResultScopePosture {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClientComplete => "client_complete",
            Self::NarrowClientWindowed => "narrow_client_windowed",
            Self::ProviderPaginated => "provider_paginated",
            Self::PartialDataPending => "partial_data_pending",
            Self::StreamingLive => "streaming_live",
        }
    }

    /// True when the client holds a complete, exact, current dataset.
    pub const fn is_client_complete(self) -> bool {
        matches!(self, Self::ClientComplete)
    }
}

/// Why rows are narrowed out of the matching set. Each cause is a first-class
/// chip so provider and policy narrowing never hides inside a generic filter
/// pill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingCause {
    /// Rows hidden or blocked by policy.
    Policy,
    /// Rows outside the active workset / scope.
    Workset,
    /// Rows withheld or capped by the backing provider.
    Provider,
    /// Rows not materialized because of a client / device limit.
    Client,
    /// Rows missing because the dataset is only partially loaded.
    PartialData,
}

impl NarrowingCause {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Policy => "policy",
            Self::Workset => "workset",
            Self::Provider => "provider",
            Self::Client => "client",
            Self::PartialData => "partial_data",
        }
    }
}

/// Where the result-scope counters are surfaced. Only [`CounterPlacement::NearActiveFilters`]
/// keeps the truth visible; footer-only or provider-private placement buries it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CounterPlacement {
    /// Counters and chips render next to the active filters.
    NearActiveFilters,
    /// Counters render only in a footer / status bar.
    FooterOnly,
    /// Counters live in provider-private text the user cannot see.
    ProviderPrivate,
}

impl CounterPlacement {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NearActiveFilters => "near_active_filters",
            Self::FooterOnly => "footer_only",
            Self::ProviderPrivate => "provider_private",
        }
    }

    /// True when the placement keeps result truth visible to the user.
    pub const fn is_visible(self) -> bool {
        matches!(self, Self::NearActiveFilters)
    }
}

/// One result-scope count: a kind, a value, and the exactness and freshness that
/// keep the number honest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultScopeCount {
    /// Which population this count measures.
    pub kind: ResultCountKind,
    /// Non-negative count value.
    pub value: u64,
    /// Whether the value is exact or approximate.
    pub exactness: CountExactness,
    /// Whether the value is fresh, stale, or partial.
    pub freshness: CountFreshness,
    /// Precise basis label, required whenever the value is approximate or not
    /// fresh so the user sees *why* the number is qualified.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub basis_label: Option<String>,
}

impl ResultScopeCount {
    /// Builds an exact, fresh count.
    pub fn exact(kind: ResultCountKind, value: u64) -> Self {
        Self {
            kind,
            value,
            exactness: CountExactness::Exact,
            freshness: CountFreshness::Fresh,
            basis_label: None,
        }
    }

    /// Whether the count requires a basis label (approximate or non-fresh).
    pub fn requires_basis_label(&self) -> bool {
        !self.exactness.is_exact() || !self.freshness.is_fresh()
    }

    /// Whether the count is internally consistent: a qualified value carries a
    /// precise, non-generic basis label.
    pub fn is_valid(&self) -> bool {
        if self.requires_basis_label() {
            self.basis_label
                .as_ref()
                .is_some_and(|label| !label_is_generic(label))
        } else {
            true
        }
    }
}

/// One hidden-narrowing chip rendered near the active filters. Discloses how many
/// rows a single cause removed from the matching set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenNarrowingChip {
    /// Why these rows are narrowed out.
    pub cause: NarrowingCause,
    /// Number of rows this cause hid from the matching set.
    pub hidden_count: u64,
    /// Precise, non-generic chip label shown to the operator.
    pub chip_label: String,
    /// True when the chip renders next to the active filters (required).
    pub near_active_filters: bool,
}

impl HiddenNarrowingChip {
    /// Whether the chip is well formed: a real hidden count, a precise label, and
    /// placement next to the active filters.
    pub fn is_valid(&self) -> bool {
        self.hidden_count > 0 && !label_is_generic(&self.chip_label) && self.near_active_filters
    }
}

/// Redaction-aware projection of one binding's result truth for diagnostics and
/// support packets. Carries only kinds, tokens, labels, and counts — never raw
/// row bodies or provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultScopeReconstruction {
    /// Binding id this reconstruction projects.
    pub binding_id: String,
    /// Surface token.
    pub surface_token: String,
    /// View-kind token.
    pub view_kind_token: String,
    /// Dataset-posture token.
    pub posture_token: String,
    /// Whether the surface is virtualized.
    pub virtualized: bool,
    /// Whether the surface is provider-backed.
    pub provider_backed: bool,
    /// Visible count value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub visible_value: Option<u64>,
    /// Loaded count value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loaded_value: Option<u64>,
    /// Matching count value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matching_value: Option<u64>,
    /// Total count value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub total_value: Option<u64>,
    /// Hidden-by-scope count value, if present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hidden_by_scope_value: Option<u64>,
    /// True when any count is approximate.
    pub has_approximate_count: bool,
    /// True when any count is stale or partial.
    pub has_qualified_freshness: bool,
    /// Narrowing-cause tokens disclosed by chips.
    pub narrowing_cause_tokens: Vec<String>,
    /// Whether selecting all matching rows requires an explicit expansion step.
    pub all_matching_requires_explicit_step: bool,
}

/// One result-scope counter binding for a dense M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultScopeCounterBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Bound dense collection surface.
    pub surface: DenseCollectionSurface,
    /// How the surface is rendered.
    pub view_kind: CollectionViewKind,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Result-scope counts. Must cover visible, loaded, matching, and total.
    pub counts: Vec<ResultScopeCount>,
    /// Hidden-narrowing chips disclosed near the active filters.
    pub narrowing_chips: Vec<HiddenNarrowingChip>,
    /// Dataset posture this binding renders under.
    pub posture: ResultScopePosture,
    /// True when the surface is virtualized.
    pub virtualized: bool,
    /// True when the surface is provider-backed.
    pub provider_backed: bool,
    /// Canonical scope-counter vocabulary terms the surface keeps.
    pub scope_vocabulary_terms: Vec<ScopeCounterVocabularyTerm>,
    /// Where the counters and chips are surfaced.
    pub counter_placement: CounterPlacement,
    /// What a select-all control means on this surface.
    pub select_all_meaning: SelectAllMeaning,
    /// True when expanding selection to all matching rows requires an explicit
    /// step rather than treating visible rows as all matching (required).
    pub all_matching_requires_explicit_step: bool,
    /// True when the counters and chips survive virtualization, reopen, and
    /// export (required).
    pub survives_reopen: bool,
    /// Evidence packet refs backing this binding.
    pub evidence_refs: Vec<String>,
}

impl ResultScopeCounterBinding {
    /// The count for a given kind, if present.
    pub fn count_for(&self, kind: ResultCountKind) -> Option<&ResultScopeCount> {
        self.counts.iter().find(|count| count.kind == kind)
    }

    /// Whether every required count kind (visible, loaded, matching, total) is
    /// present.
    pub fn has_required_counts(&self) -> bool {
        REQUIRED_COUNT_KINDS
            .iter()
            .all(|kind| self.count_for(*kind).is_some())
    }

    /// Whether each count kind appears at most once.
    pub fn counts_unique(&self) -> bool {
        let kinds: BTreeSet<_> = self.counts.iter().map(|count| count.kind).collect();
        kinds.len() == self.counts.len()
    }

    /// Whether the nested counts respect visible ≤ loaded ≤ matching ≤ total for
    /// every adjacent pair whose values are both exact. Approximate values are
    /// exempt from the ordering check (they are explicitly imprecise) but must
    /// still be labeled approximate.
    pub fn counts_monotonic(&self) -> bool {
        let chain = [
            ResultCountKind::Visible,
            ResultCountKind::Loaded,
            ResultCountKind::Matching,
            ResultCountKind::Total,
        ];
        for window in chain.windows(2) {
            let (Some(lower), Some(upper)) = (self.count_for(window[0]), self.count_for(window[1]))
            else {
                continue;
            };
            if lower.exactness.is_exact() && upper.exactness.is_exact() && lower.value > upper.value
            {
                return false;
            }
        }
        true
    }

    /// Whether hidden-by-scope truth reconciles. When a hidden-by-scope count is
    /// present and exact, it must equal `total - matching` (when both are exact)
    /// and must equal the sum of the disclosed chip counts.
    pub fn hidden_reconciles(&self) -> bool {
        let Some(hidden) = self.count_for(ResultCountKind::HiddenByScope) else {
            // No hidden count: chips must not claim hidden rows out of nowhere.
            return self.narrowing_chips.is_empty();
        };
        if !hidden.exactness.is_exact() {
            return true;
        }
        let chip_sum: u64 = self
            .narrowing_chips
            .iter()
            .map(|chip| chip.hidden_count)
            .sum();
        if chip_sum != hidden.value {
            return false;
        }
        if let (Some(total), Some(matching)) = (
            self.count_for(ResultCountKind::Total),
            self.count_for(ResultCountKind::Matching),
        ) {
            if total.exactness.is_exact() && matching.exactness.is_exact() {
                return total.value.saturating_sub(matching.value) == hidden.value;
            }
        }
        true
    }

    /// Whether narrowing disclosure is consistent: any hidden rows are explained
    /// by at least one valid chip, every chip is well formed, and each cause is
    /// disclosed by its own chip rather than folded into another.
    pub fn narrowing_consistent(&self) -> bool {
        if !self
            .narrowing_chips
            .iter()
            .all(HiddenNarrowingChip::is_valid)
        {
            return false;
        }
        let causes: BTreeSet<_> = self.narrowing_chips.iter().map(|chip| chip.cause).collect();
        if causes.len() != self.narrowing_chips.len() {
            return false;
        }
        let hidden_rows = self
            .count_for(ResultCountKind::HiddenByScope)
            .map(|count| count.value)
            .unwrap_or(0);
        if hidden_rows > 0 && self.narrowing_chips.is_empty() {
            return false;
        }
        self.hidden_reconciles()
    }

    /// Whether the posture is consistent with the declared counts. A complete
    /// client is exact and fresh everywhere; a provider-paginated surface cannot
    /// claim an exact total; partial / streaming surfaces carry a qualified count.
    pub fn posture_consistent(&self) -> bool {
        match self.posture {
            ResultScopePosture::ClientComplete => self
                .counts
                .iter()
                .all(|count| count.exactness.is_exact() && count.freshness.is_fresh()),
            ResultScopePosture::ProviderPaginated => self
                .count_for(ResultCountKind::Total)
                .is_some_and(|total| !total.exactness.is_exact()),
            ResultScopePosture::PartialDataPending | ResultScopePosture::StreamingLive => self
                .counts
                .iter()
                .any(|count| !count.exactness.is_exact() || !count.freshness.is_fresh()),
            ResultScopePosture::NarrowClientWindowed => true,
        }
    }

    /// Whether the scope-counter vocabulary is complete.
    pub fn scope_vocabulary_ok(&self) -> bool {
        let present: BTreeSet<_> = self.scope_vocabulary_terms.iter().copied().collect();
        REQUIRED_SCOPE_VOCABULARY_TERMS
            .iter()
            .all(|term| present.contains(term))
    }

    /// Whether every dimension required to record this binding is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.binding_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.has_required_counts()
            && self.counts_unique()
            && self.counts.iter().all(ResultScopeCount::is_valid)
            && self.counts_monotonic()
            && self.narrowing_consistent()
            && self.posture_consistent()
            && self.scope_vocabulary_ok()
            && self.counter_placement.is_visible()
            && self.all_matching_requires_explicit_step
            && self.survives_reopen
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Projects the binding into a redaction-aware reconstruction for diagnostics
    /// and support packets.
    pub fn reconstruction(&self) -> ResultScopeReconstruction {
        ResultScopeReconstruction {
            binding_id: self.binding_id.clone(),
            surface_token: self.surface.as_str().to_owned(),
            view_kind_token: self.view_kind.as_str().to_owned(),
            posture_token: self.posture.as_str().to_owned(),
            virtualized: self.virtualized,
            provider_backed: self.provider_backed,
            visible_value: self
                .count_for(ResultCountKind::Visible)
                .map(|count| count.value),
            loaded_value: self
                .count_for(ResultCountKind::Loaded)
                .map(|count| count.value),
            matching_value: self
                .count_for(ResultCountKind::Matching)
                .map(|count| count.value),
            total_value: self
                .count_for(ResultCountKind::Total)
                .map(|count| count.value),
            hidden_by_scope_value: self
                .count_for(ResultCountKind::HiddenByScope)
                .map(|count| count.value),
            has_approximate_count: self.counts.iter().any(|count| !count.exactness.is_exact()),
            has_qualified_freshness: self.counts.iter().any(|count| !count.freshness.is_fresh()),
            narrowing_cause_tokens: self
                .narrowing_chips
                .iter()
                .map(|chip| chip.cause.as_str().to_owned())
                .collect(),
            all_matching_requires_explicit_step: self.all_matching_requires_explicit_step,
        }
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultScopeGuardrails {
    /// Hidden narrowing is always visible near the active filters.
    pub hidden_narrowing_always_visible: bool,
    /// Exact and approximate counts are explicitly distinguished.
    pub exact_versus_approximate_labeled: bool,
    /// Visible, loaded, and matching counts are never blurred together.
    pub visible_loaded_matching_never_blurred: bool,
    /// Counters survive virtualization, pagination, reopen, and export.
    pub counters_survive_virtualization_and_reopen: bool,
    /// Provider and policy narrowing is never folded into a generic filter chip.
    pub provider_policy_narrowing_not_generic: bool,
}

impl ResultScopeGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.hidden_narrowing_always_visible
            && self.exact_versus_approximate_labeled
            && self.visible_loaded_matching_never_blurred
            && self.counters_survive_virtualization_and_reopen
            && self.provider_policy_narrowing_not_generic
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultScopeConsumerProjection {
    /// Product renders counters and chips from these shared records.
    pub product_renders_counters_and_chips: bool,
    /// Diagnostics reconstruct scope truth from these records.
    pub diagnostics_reconstructs_scope_truth: bool,
    /// Support/export reuses the result-truth projection.
    pub support_export_reuses_records: bool,
    /// Docs and help reuse the counter and chip vocabulary.
    pub docs_help_reuses_vocabulary: bool,
}

impl ResultScopeConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_renders_counters_and_chips
            && self.diagnostics_reconstructs_scope_truth
            && self.support_export_reuses_records
            && self.docs_help_reuses_vocabulary
    }
}

/// Constructor input for [`ResultScopeCounterPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResultScopeCounterPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-surface counter bindings.
    pub bindings: Vec<ResultScopeCounterBinding>,
    /// Guardrail invariants block.
    pub guardrails: ResultScopeGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ResultScopeConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe result-scope counter packet for the first real M5 dense surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResultScopeCounterPacket {
    /// Record kind; must equal [`RESULT_SCOPE_COUNTER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RESULT_SCOPE_COUNTER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-surface counter bindings.
    pub bindings: Vec<ResultScopeCounterBinding>,
    /// Guardrail invariants block.
    pub guardrails: ResultScopeGuardrails,
    /// Consumer projection block.
    pub consumer_projection: ResultScopeConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ResultScopeCounterPacket {
    /// Builds a result-scope counter packet.
    pub fn new(input: ResultScopeCounterPacketInput) -> Self {
        Self {
            record_kind: RESULT_SCOPE_COUNTER_RECORD_KIND.to_owned(),
            schema_version: RESULT_SCOPE_COUNTER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            bindings: input.bindings,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some binding in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<DenseCollectionSurface> {
        self.bindings
            .iter()
            .map(|binding| binding.surface)
            .collect()
    }

    /// View kinds represented by some binding in this packet.
    pub fn represented_view_kinds(&self) -> BTreeSet<CollectionViewKind> {
        self.bindings
            .iter()
            .map(|binding| binding.view_kind)
            .collect()
    }

    /// Count of bindings that disclose at least one hidden-narrowing chip.
    pub fn narrowed_binding_count(&self) -> usize {
        self.bindings
            .iter()
            .filter(|binding| !binding.narrowing_chips.is_empty())
            .count()
    }

    /// Reconstructions for every binding, used by diagnostics and support packets.
    pub fn reconstructions(&self) -> Vec<ResultScopeReconstruction> {
        self.bindings
            .iter()
            .map(ResultScopeCounterBinding::reconstruction)
            .collect()
    }

    /// Validates the result-scope counter packet invariants.
    pub fn validate(&self) -> Vec<ResultScopeCounterViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RESULT_SCOPE_COUNTER_RECORD_KIND {
            violations.push(ResultScopeCounterViolation::WrongRecordKind);
        }
        if self.schema_version != RESULT_SCOPE_COUNTER_SCHEMA_VERSION {
            violations.push(ResultScopeCounterViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ResultScopeCounterViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(ResultScopeCounterViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ResultScopeCounterViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("result scope counter packet serializes"),
        ) {
            violations.push(ResultScopeCounterViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("result scope counter packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Result-Scope Counters And Hidden-Narrowing Chips\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Bindings: {} ({} narrowed)\n",
            self.bindings.len(),
            self.narrowed_binding_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            REQUIRED_COUNTER_SURFACES.len()
        ));
        out.push_str(&format!(
            "- View kinds: {} / {}\n",
            self.represented_view_kinds().len(),
            CollectionViewKind::ALL.len()
        ));
        out.push_str("\n## Bindings\n\n");
        for binding in &self.bindings {
            out.push_str(&format!(
                "- **{}** ({} / {}): {}\n",
                binding.binding_id,
                binding.surface.as_str(),
                binding.view_kind.as_str(),
                binding.label_summary,
            ));
            out.push_str(&format!("  - posture=`{}`", binding.posture.as_str()));
            for kind in [
                ResultCountKind::Visible,
                ResultCountKind::Loaded,
                ResultCountKind::Matching,
                ResultCountKind::Total,
            ] {
                if let Some(count) = binding.count_for(kind) {
                    out.push_str(&format!(
                        " {}={}{}",
                        kind.as_str(),
                        count.value,
                        if count.exactness.is_exact() { "" } else { "~" },
                    ));
                }
            }
            out.push('\n');
            for chip in &binding.narrowing_chips {
                out.push_str(&format!(
                    "  - hidden by {}: {} ({})\n",
                    chip.cause.as_str(),
                    chip.hidden_count,
                    chip.chip_label,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in result-scope counter export.
#[derive(Debug)]
pub enum ResultScopeCounterArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ResultScopeCounterViolation>),
}

impl fmt::Display for ResultScopeCounterArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "result scope counter export parse failed: {error}"
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
                    "result scope counter export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ResultScopeCounterArtifactError {}

/// Validation failures emitted by [`ResultScopeCounterPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResultScopeCounterViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required real M5 surface is bound by no counter record.
    RequiredSurfaceMissing,
    /// A required view kind (list, tree, table, queue) is represented by no
    /// binding.
    RequiredViewKindMissing,
    /// No binding demonstrates the hidden-narrowing chip path.
    NarrowingCaseMissing,
    /// No binding demonstrates an approximate result-scope counter.
    ApproximateCaseMissing,
    /// No binding demonstrates a stale or partial result-scope counter.
    StaleOrPartialCaseMissing,
    /// A binding is incomplete.
    BindingIncomplete,
    /// A binding is missing a required count kind.
    RequiredCountMissing,
    /// A binding repeats a count kind.
    DuplicateCountKind,
    /// A binding's nested counts are not monotonic.
    CountsNotMonotonic,
    /// A binding's qualified count lacks a precise basis label.
    QualifiedCountMissingLabel,
    /// A binding's narrowing disclosure is inconsistent.
    NarrowingInconsistent,
    /// A binding's posture is inconsistent with its counts.
    PostureInconsistent,
    /// A binding's scope-counter vocabulary is incomplete.
    ScopeVocabularyIncomplete,
    /// A binding surfaces counters somewhere other than near the active filters.
    CounterPlacementBuried,
    /// A binding lets visible rows stand in for all matching rows without an
    /// explicit step.
    AllMatchingWithoutExplicitStep,
    /// A binding does not preserve counters across reopen / virtualization.
    DoesNotSurviveReopen,
    /// A binding lacks evidence refs.
    BindingEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ResultScopeCounterViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::RequiredViewKindMissing => "required_view_kind_missing",
            Self::NarrowingCaseMissing => "narrowing_case_missing",
            Self::ApproximateCaseMissing => "approximate_case_missing",
            Self::StaleOrPartialCaseMissing => "stale_or_partial_case_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::RequiredCountMissing => "required_count_missing",
            Self::DuplicateCountKind => "duplicate_count_kind",
            Self::CountsNotMonotonic => "counts_not_monotonic",
            Self::QualifiedCountMissingLabel => "qualified_count_missing_label",
            Self::NarrowingInconsistent => "narrowing_inconsistent",
            Self::PostureInconsistent => "posture_inconsistent",
            Self::ScopeVocabularyIncomplete => "scope_vocabulary_incomplete",
            Self::CounterPlacementBuried => "counter_placement_buried",
            Self::AllMatchingWithoutExplicitStep => "all_matching_without_explicit_step",
            Self::DoesNotSurviveReopen => "does_not_survive_reopen",
            Self::BindingEvidenceMissing => "binding_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in result-scope counter export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_result_scope_counter_export(
) -> Result<ResultScopeCounterPacket, ResultScopeCounterArtifactError> {
    let packet: ResultScopeCounterPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m5/ship-hidden-narrowing-chips-exact-versus-approximate-result-scope-counters-and-visible-ver/support_export.json"
    )))
    .map_err(ResultScopeCounterArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ResultScopeCounterArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ResultScopeCounterPacket,
    violations: &mut Vec<ResultScopeCounterViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RESULT_SCOPE_COUNTER_SCHEMA_REF,
        RESULT_SCOPE_COUNTER_DOC_REF,
        RESULT_SCOPE_COUNTER_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ResultScopeCounterViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &ResultScopeCounterPacket,
    violations: &mut Vec<ResultScopeCounterViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in REQUIRED_COUNTER_SURFACES {
        if !surfaces.contains(&required) {
            violations.push(ResultScopeCounterViolation::RequiredSurfaceMissing);
            break;
        }
    }

    let view_kinds = packet.represented_view_kinds();
    for required in CollectionViewKind::ALL {
        if !view_kinds.contains(&required) {
            violations.push(ResultScopeCounterViolation::RequiredViewKindMissing);
            break;
        }
    }

    if !packet
        .bindings
        .iter()
        .any(|binding| !binding.narrowing_chips.is_empty() && binding.narrowing_consistent())
    {
        violations.push(ResultScopeCounterViolation::NarrowingCaseMissing);
    }

    if !packet.bindings.iter().any(|binding| {
        binding
            .counts
            .iter()
            .any(|count| !count.exactness.is_exact())
    }) {
        violations.push(ResultScopeCounterViolation::ApproximateCaseMissing);
    }

    if !packet.bindings.iter().any(|binding| {
        binding
            .counts
            .iter()
            .any(|count| !count.freshness.is_fresh())
    }) {
        violations.push(ResultScopeCounterViolation::StaleOrPartialCaseMissing);
    }
}

fn validate_bindings(
    packet: &ResultScopeCounterPacket,
    violations: &mut Vec<ResultScopeCounterViolation>,
) {
    for binding in &packet.bindings {
        if !binding.is_complete() {
            violations.push(ResultScopeCounterViolation::BindingIncomplete);
        }
        if !binding.has_required_counts() {
            violations.push(ResultScopeCounterViolation::RequiredCountMissing);
        }
        if !binding.counts_unique() {
            violations.push(ResultScopeCounterViolation::DuplicateCountKind);
        }
        if !binding.counts_monotonic() {
            violations.push(ResultScopeCounterViolation::CountsNotMonotonic);
        }
        if !binding.counts.iter().all(ResultScopeCount::is_valid) {
            violations.push(ResultScopeCounterViolation::QualifiedCountMissingLabel);
        }
        if !binding.narrowing_consistent() {
            violations.push(ResultScopeCounterViolation::NarrowingInconsistent);
        }
        if !binding.posture_consistent() {
            violations.push(ResultScopeCounterViolation::PostureInconsistent);
        }
        if !binding.scope_vocabulary_ok() {
            violations.push(ResultScopeCounterViolation::ScopeVocabularyIncomplete);
        }
        if !binding.counter_placement.is_visible() {
            violations.push(ResultScopeCounterViolation::CounterPlacementBuried);
        }
        if !binding.all_matching_requires_explicit_step {
            violations.push(ResultScopeCounterViolation::AllMatchingWithoutExplicitStep);
        }
        if !binding.survives_reopen {
            violations.push(ResultScopeCounterViolation::DoesNotSurviveReopen);
        }
        if binding.evidence_refs.is_empty()
            || binding.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(ResultScopeCounterViolation::BindingEvidenceMissing);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label. A generic
/// provider error must never stand in for a precise truth.
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
            | "narrowed"
            | "filtered"
            | "approximate"
            | "stale"
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
