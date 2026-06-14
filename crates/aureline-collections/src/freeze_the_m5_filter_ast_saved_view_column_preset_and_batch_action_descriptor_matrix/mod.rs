//! Freeze of the filter-AST, saved-view, column-preset, selection-scope,
//! result-counter, and batch-action-descriptor matrix for every claimed M5 dense
//! collection surface.
//!
//! M5 operational surfaces are increasingly dense collections: pipeline run
//! lists, review queues, incident lists, graph lists, marketplace results,
//! activity rows, provider/admin tables, and query-backed result sets. Those
//! lanes only stay trustworthy if their filter grammar, saved views, selection
//! scope, result counters, and batch-action scope are canonical product objects
//! rather than surface-local heuristics.
//!
//! Where [`crate::stabilize_filter_ast_saved_view_scope_pack_column_preset`]
//! froze the cross-surface collection-truth *vocabulary* and
//! [`crate::stabilize_selection_scope_and_batch_result_truth`] froze the
//! *selection and batch-result* objects, this module binds those into a single
//! bounded **qualification matrix**. The matrix is the one canonical answer to
//! "for this claimed M5 dense surface, what is its filter-AST class, selection
//! scope class, result-counter class, and batch-action scope class — and is the
//! public qualification it claims actually backed by an identified filter, scope,
//! count, and batch semantics?"
//!
//! Each [`CollectionQualificationRow`] reuses the frozen
//! [`SelectionScopeClass`](crate::stabilize_selection_scope_and_batch_result_truth::SelectionScopeClass)
//! and
//! [`ScopeCounterVocabularyTerm`](crate::stabilize_filter_ast_saved_view_scope_pack_column_preset::ScopeCounterVocabularyTerm)
//! vocabularies rather than minting synonyms, and adds the matrix-level
//! dimensions this freeze owns: [`FilterAstClass`], [`ResultCounterClass`],
//! [`BatchActionScopeClass`], and [`BatchActionDescriptor`]. The matrix
//! *auto-downgrades*: a claimed row that cannot identify its filter-AST,
//! selection-scope, result-counter, or batch-action class must carry an
//! `effective_qualification` strictly below its claim, a recorded downgrade
//! trigger, and a precise degraded label — so a collection claim never outruns
//! the evidence that backs it.
//!
//! [`CollectionQualificationMatrixPacket::validate`] also refuses a row that lets
//! a transient row highlight stand in for durable selection, hides provider or
//! policy narrowing inside a generic filter chip, treats the visible rows as all
//! matching rows without an explicit step, or lets a broad batch action bypass
//! preview because the list is virtualized or provider-backed.
//!
//! Raw query text, raw filter literal bytes, provider cursors, credentials, and
//! raw row bodies never cross this boundary; the packet carries only typed class
//! tokens, booleans, opaque ids, and redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/collections/freeze-the-m5-filter-ast-saved-view-column-preset-and-batch-action-descriptor-matrix.schema.json`](../../../../schemas/collections/freeze-the-m5-filter-ast-saved-view-column-preset-and-batch-action-descriptor-matrix.schema.json).
//! The contract doc is
//! [`docs/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix.md`](../../../../docs/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix.md).
//! The protected fixture directory is
//! [`fixtures/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/`](../../../../fixtures/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_filter_ast_saved_view_scope_pack_column_preset::ScopeCounterVocabularyTerm;
use crate::stabilize_selection_scope_and_batch_result_truth::SelectionScopeClass;

/// Stable record-kind tag carried by [`CollectionQualificationMatrixPacket`].
pub const M5_COLLECTION_QUALIFICATION_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix";

/// Schema version for the dense-collection qualification matrix.
pub const M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_REF: &str =
    "schemas/collections/freeze-the-m5-filter-ast-saved-view-column-preset-and-batch-action-descriptor-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COLLECTION_QUALIFICATION_MATRIX_DOC_REF: &str =
    "docs/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COLLECTION_QUALIFICATION_MATRIX_FIXTURE_DIR: &str =
    "fixtures/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COLLECTION_QUALIFICATION_MATRIX_ARTIFACT_REF: &str =
    "artifacts/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_COLLECTION_QUALIFICATION_MATRIX_SUMMARY_REF: &str =
    "artifacts/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix.md";

/// Canonical scope-counter vocabulary every result-counter-identified row must
/// render so the visible / loaded / matching / selected distinctions never blur.
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

/// One claimed M5 dense collection surface a matrix row covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenseCollectionSurface {
    /// Pipeline run list / queue.
    PipelineRunList,
    /// Review or approval queue.
    ReviewQueue,
    /// Incident or alert list.
    IncidentList,
    /// Graph / dependency / reference list.
    GraphList,
    /// Marketplace / extension search results.
    MarketplaceResults,
    /// Activity / audit / history rows.
    ActivityRows,
    /// Provider-backed admin or settings table.
    ProviderAdminTable,
    /// Query-backed result set (search, data grid, log table).
    QueryBackedResultSet,
    /// Support / export projection of the matrix.
    SupportExportProjection,
}

impl DenseCollectionSurface {
    /// Every claimed surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PipelineRunList,
        Self::ReviewQueue,
        Self::IncidentList,
        Self::GraphList,
        Self::MarketplaceResults,
        Self::ActivityRows,
        Self::ProviderAdminTable,
        Self::QueryBackedResultSet,
        Self::SupportExportProjection,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PipelineRunList => "pipeline_run_list",
            Self::ReviewQueue => "review_queue",
            Self::IncidentList => "incident_list",
            Self::GraphList => "graph_list",
            Self::MarketplaceResults => "marketplace_results",
            Self::ActivityRows => "activity_rows",
            Self::ProviderAdminTable => "provider_admin_table",
            Self::QueryBackedResultSet => "query_backed_result_set",
            Self::SupportExportProjection => "support_export_projection",
        }
    }
}

/// Closed filter-AST class vocabulary. Names the canonical filter grammar a row
/// declares; distinct from any per-clause object, this is the matrix-level
/// classification a claimed surface must identify.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterAstClass {
    /// A typed clause AST with operators, source classes, and round-trip state.
    TypedClauseAst,
    /// A saved or pinned query snapshot replayed as the filter.
    SavedQuerySnapshot,
    /// The provider owns query semantics; the client delegates the filter.
    ProviderDelegatedQuery,
    /// A scoped free-text query with no structured clauses.
    FreeTextScoped,
}

impl FilterAstClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypedClauseAst => "typed_clause_ast",
            Self::SavedQuerySnapshot => "saved_query_snapshot",
            Self::ProviderDelegatedQuery => "provider_delegated_query",
            Self::FreeTextScoped => "free_text_scoped",
        }
    }

    /// True when the provider, not the client, owns the query semantics.
    pub const fn is_provider_owned(self) -> bool {
        matches!(self, Self::ProviderDelegatedQuery)
    }
}

/// Closed result-counter class vocabulary. Names how a surface establishes its
/// counts so an approximate or provider-limited count never reads as exact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResultCounterClass {
    /// Counts are exact for the stated scope.
    ExactCount,
    /// Counts are approximate and labeled as such.
    ApproximateCount,
    /// The provider limits or samples the count.
    ProviderLimitedCount,
    /// Counts are partial because rows are still streaming.
    PartialStreamingCount,
}

impl ResultCounterClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactCount => "exact_count",
            Self::ApproximateCount => "approximate_count",
            Self::ProviderLimitedCount => "provider_limited_count",
            Self::PartialStreamingCount => "partial_streaming_count",
        }
    }
}

/// Closed batch-action scope class vocabulary. Names how a surface establishes
/// the scope of a broad action so a provider-owned or destructive batch is never
/// mistaken for a local reversible one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchActionScopeClass {
    /// Local, reversible batch over a client-owned set.
    LocalReversibleBatch,
    /// Client stages membership; the provider completes execution.
    MixedClientProviderBatch,
    /// The provider owns membership and execution authority.
    ProviderAuthoritativeBatch,
    /// Destructive or irreversible batch gated behind explicit review.
    DestructiveGatedBatch,
    /// Inspect-only surface that offers no batch action.
    InspectOnlyNoBatch,
}

impl BatchActionScopeClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalReversibleBatch => "local_reversible_batch",
            Self::MixedClientProviderBatch => "mixed_client_provider_batch",
            Self::ProviderAuthoritativeBatch => "provider_authoritative_batch",
            Self::DestructiveGatedBatch => "destructive_gated_batch",
            Self::InspectOnlyNoBatch => "inspect_only_no_batch",
        }
    }

    /// True when the batch touches provider state or is destructive and so must
    /// preview included / excluded / blocked / skipped / hidden members first.
    pub const fn requires_preview(self) -> bool {
        matches!(
            self,
            Self::MixedClientProviderBatch
                | Self::ProviderAuthoritativeBatch
                | Self::DestructiveGatedBatch
        )
    }
}

/// Closed batch-action kind vocabulary the freeze owns. Each broad action a dense
/// surface offers declares its kind so its scope receipt is canonical.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchActionKind {
    /// Export a scope to a file or packet.
    Export,
    /// Copy a scope to the clipboard or another surface.
    Copy,
    /// Suppress / mute / dismiss a scope.
    Suppress,
    /// Install a scope (marketplace, extensions).
    Install,
    /// Update a scope (versions, settings).
    Update,
    /// Delete a scope.
    Delete,
    /// Re-run a scope (pipeline, tests).
    Rerun,
    /// Share a scope (links, handoff).
    Share,
    /// Approve a scope (review, provider queue).
    Approve,
}

impl BatchActionKind {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Export => "export",
            Self::Copy => "copy",
            Self::Suppress => "suppress",
            Self::Install => "install",
            Self::Update => "update",
            Self::Delete => "delete",
            Self::Rerun => "rerun",
            Self::Share => "share",
            Self::Approve => "approve",
        }
    }

    /// True when this action kind always produces a scope receipt the operator
    /// must be able to review (exported, copied, shared, or mutated members).
    pub const fn always_emits_scope_receipt(self) -> bool {
        true
    }
}

/// One declared batch action a dense surface offers, with its scope-receipt and
/// preview requirements.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BatchActionDescriptor {
    /// Stable action id.
    pub action_id: String,
    /// Action kind.
    pub action_kind: BatchActionKind,
    /// True when the action mutates persistent or provider state.
    pub mutates_state: bool,
    /// True when a provider owns the execution or authoritative membership.
    pub provider_backed: bool,
    /// True when the action is reversible without data loss.
    pub reversible: bool,
    /// True when the action previews included / excluded / blocked / skipped /
    /// hidden members before commit.
    pub requires_preview: bool,
    /// True when the action emits a reviewable scope receipt.
    pub scope_receipt_required: bool,
}

impl BatchActionDescriptor {
    /// Whether this action must preview its scope before commit. Any mutating,
    /// provider-backed, or irreversible action — and every export / copy / share —
    /// must preview, regardless of how the list is rendered.
    pub fn must_preview(&self) -> bool {
        self.mutates_state
            || self.provider_backed
            || !self.reversible
            || matches!(
                self.action_kind,
                BatchActionKind::Export | BatchActionKind::Copy | BatchActionKind::Share
            )
    }

    /// Whether this descriptor satisfies its preview and scope-receipt invariants.
    pub fn is_safe(&self) -> bool {
        !self.action_id.trim().is_empty()
            && (!self.must_preview() || self.requires_preview)
            && (!self.action_kind.always_emits_scope_receipt() || self.scope_receipt_required)
    }
}

/// Declared saved-view contract for a dense surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedViewDeclaration {
    /// Stable saved-view id.
    pub saved_view_id: String,
    /// Owner-scope token (e.g. `workspace`, `user`, `provider`).
    pub owner_scope_token: String,
    /// Privacy-class token (e.g. `shared_redacted`).
    pub privacy_class_token: String,
    /// Fallback-behavior token shown when the view cannot replay exactly.
    pub fallback_behavior_token: String,
    /// True when the view captures a transient selection (forbidden).
    pub captures_selection: bool,
    /// True when the view captures a provider cursor (forbidden).
    pub captures_provider_cursor: bool,
    /// True when reopen can rebind to current roots, providers, and policy.
    pub reopen_rebind_supported: bool,
}

impl SavedViewDeclaration {
    /// Whether the saved-view declaration is valid: it carries identity, never
    /// captures transient selection or a provider cursor, and supports rebind.
    pub fn is_valid(&self) -> bool {
        !self.saved_view_id.trim().is_empty()
            && !self.owner_scope_token.trim().is_empty()
            && !self.privacy_class_token.trim().is_empty()
            && !self.fallback_behavior_token.trim().is_empty()
            && !self.captures_selection
            && !self.captures_provider_cursor
            && self.reopen_rebind_supported
    }
}

/// Declared column-preset contract for a dense surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ColumnPresetDeclaration {
    /// Stable column-preset id.
    pub column_preset_id: String,
    /// Ordered visible columns.
    pub visible_column_ids: Vec<String>,
    /// Required identity or provenance columns that may not be silently dropped.
    pub required_identity_column_ids: Vec<String>,
    /// Pinned columns.
    pub pinned_column_ids: Vec<String>,
    /// Density-mode token.
    pub density_mode_token: String,
}

impl ColumnPresetDeclaration {
    /// Required identity columns missing from the visible set.
    pub fn missing_identity_columns(&self) -> Vec<&str> {
        self.required_identity_column_ids
            .iter()
            .filter(|id| !self.visible_column_ids.iter().any(|visible| visible == *id))
            .map(String::as_str)
            .collect()
    }

    /// Whether the preset keeps every required identity column visible.
    pub fn is_valid(&self) -> bool {
        !self.column_preset_id.trim().is_empty()
            && !self.density_mode_token.trim().is_empty()
            && self.missing_identity_columns().is_empty()
    }
}

/// Closed qualification vocabulary the matrix freezes for claimed rows. Higher
/// means a stronger public claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionMatrixQualificationClass {
    /// Promoted, durable, publicly claimed.
    Stable,
    /// Publicly claimed but still hardening.
    Beta,
    /// Narrow public preview.
    Preview,
    /// Internal / experimental; not a public claim.
    Experimental,
    /// Held below preview pending evidence.
    Held,
    /// Not available on this surface.
    Unavailable,
}

impl CollectionMatrixQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Held => "held",
            Self::Unavailable => "unavailable",
        }
    }

    /// Whether this class is a publicly claimed lane (Stable, Beta, or Preview).
    pub const fn is_claimed(self) -> bool {
        matches!(self, Self::Stable | Self::Beta | Self::Preview)
    }

    /// Ordinal rank used to compare claim severity; higher is a stronger claim, so
    /// a downgrade must move strictly lower.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Unavailable => 0,
            Self::Held => 1,
            Self::Experimental => 2,
            Self::Preview => 3,
            Self::Beta => 4,
            Self::Stable => 5,
        }
    }
}

/// Closed downgrade-trigger vocabulary. Names why a claimed row auto-downgraded
/// below its claim; the chrome quotes the trigger verbatim instead of a generic
/// error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollectionMatrixDowngradeTrigger {
    /// The filter-AST class could not be identified.
    UnidentifiedFilterAst,
    /// The selection-scope class could not be identified.
    UnidentifiedSelectionScope,
    /// The result-counter class could not be identified.
    UnidentifiedResultCounter,
    /// The batch-action scope class could not be identified.
    UnidentifiedBatchAction,
    /// A provider narrowed the surface below its claim.
    ProviderNarrowed,
    /// Policy narrowed the surface below its claim.
    PolicyNarrowed,
    /// Partial or streaming data limited the surface below its claim.
    PartialDataLimited,
    /// An upstream dependency narrowed and dragged this row down with it.
    UpstreamDependencyNarrowed,
}

impl CollectionMatrixDowngradeTrigger {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnidentifiedFilterAst => "unidentified_filter_ast",
            Self::UnidentifiedSelectionScope => "unidentified_selection_scope",
            Self::UnidentifiedResultCounter => "unidentified_result_counter",
            Self::UnidentifiedBatchAction => "unidentified_batch_action",
            Self::ProviderNarrowed => "provider_narrowed",
            Self::PolicyNarrowed => "policy_narrowed",
            Self::PartialDataLimited => "partial_data_limited",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// One claimed M5 dense collection row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionQualificationRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed dense collection surface.
    pub surface: DenseCollectionSurface,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Identified filter-AST class. `None` means it could not be identified and
    /// forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filter_ast_class: Option<FilterAstClass>,
    /// Identified selection-scope class, reusing the frozen selection vocabulary.
    /// `None` means it could not be identified and forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selection_scope_class: Option<SelectionScopeClass>,
    /// Identified result-counter class. `None` means it could not be identified
    /// and forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result_counter_class: Option<ResultCounterClass>,
    /// Identified batch-action scope class. `None` means it could not be
    /// identified and forces auto-downgrade.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub batch_action_class: Option<BatchActionScopeClass>,
    /// Canonical scope-counter vocabulary terms this row renders.
    pub scope_vocabulary_terms: Vec<ScopeCounterVocabularyTerm>,
    /// Declared saved-view contract.
    pub saved_view: SavedViewDeclaration,
    /// Declared column-preset contract.
    pub column_preset: ColumnPresetDeclaration,
    /// Declared batch actions the surface offers.
    pub batch_action_descriptors: Vec<BatchActionDescriptor>,
    /// True when selection survives sort / filter / virtualization by stable
    /// identity rather than transient row highlight.
    pub selection_survives_by_stable_identity: bool,
    /// True when provider or policy narrowing is disclosed instead of hidden in a
    /// generic filter chip.
    pub provider_policy_narrowing_disclosed: bool,
    /// True when the visible row count is kept distinct from the all-matching
    /// count, so expanding to all matching requires an explicit step.
    pub visible_distinct_from_all_matching: bool,
    /// Headline qualification publicly claimed for this row.
    pub claimed_qualification: CollectionMatrixQualificationClass,
    /// Effective qualification after auto-downgrade; equals the claim when every
    /// identity dimension is present, and ranks strictly below it otherwise.
    pub effective_qualification: CollectionMatrixQualificationClass,
    /// Trigger that fired the downgrade, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgrade_trigger: Option<CollectionMatrixDowngradeTrigger>,
    /// Precise degraded label, required when the row is downgraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl CollectionQualificationRow {
    /// Whether this row carries a public claim.
    pub fn is_claimed(&self) -> bool {
        self.claimed_qualification.is_claimed()
    }

    /// Whether every required identity dimension (filter-AST, selection-scope,
    /// result-counter, batch-action) is identified.
    pub fn identity_complete(&self) -> bool {
        self.filter_ast_class.is_some()
            && self.selection_scope_class.is_some()
            && self.result_counter_class.is_some()
            && self.batch_action_class.is_some()
    }

    /// Whether the row must downgrade below its claim because an identity
    /// dimension is missing.
    pub fn needs_downgrade(&self) -> bool {
        !self.identity_complete()
    }

    /// Whether the effective qualification and downgrade evidence are consistent.
    ///
    /// When every identity dimension is present the effective qualification equals
    /// the claim; otherwise it must rank strictly below the claim and carry both a
    /// recorded downgrade trigger and a precise degraded label.
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

    /// Whether the scope-counter vocabulary is complete when the result-counter
    /// class is identified, so visible / loaded / matching / selected never blur.
    pub fn scope_vocabulary_ok(&self) -> bool {
        if self.result_counter_class.is_none() {
            return true;
        }
        let present: BTreeSet<_> = self.scope_vocabulary_terms.iter().copied().collect();
        REQUIRED_SCOPE_VOCABULARY_TERMS
            .iter()
            .all(|term| present.contains(term))
    }

    /// Whether selection is durable by stable identity rather than row highlight.
    pub fn selection_identity_ok(&self) -> bool {
        self.selection_survives_by_stable_identity
    }

    /// Whether provider or policy narrowing is disclosed when the surface is
    /// provider-owned or any batch is provider-backed.
    pub fn narrowing_disclosure_ok(&self) -> bool {
        let provider_involved = self
            .filter_ast_class
            .is_some_and(FilterAstClass::is_provider_owned)
            || matches!(
                self.batch_action_class,
                Some(BatchActionScopeClass::ProviderAuthoritativeBatch)
                    | Some(BatchActionScopeClass::MixedClientProviderBatch)
            )
            || self
                .batch_action_descriptors
                .iter()
                .any(|descriptor| descriptor.provider_backed);
        if provider_involved {
            self.provider_policy_narrowing_disclosed
        } else {
            true
        }
    }

    /// Whether the visible count stays distinct from the all-matching count when
    /// the scope can expand to all matching rows.
    pub fn visible_versus_matching_ok(&self) -> bool {
        if matches!(
            self.selection_scope_class,
            Some(SelectionScopeClass::AllMatchingQuery)
        ) {
            self.visible_distinct_from_all_matching
        } else {
            true
        }
    }

    /// Whether every declared batch action previews its scope before commit and
    /// emits a scope receipt — never bypassed because the list is virtualized or
    /// provider-backed.
    pub fn batch_preview_ok(&self) -> bool {
        self.batch_action_descriptors
            .iter()
            .all(BatchActionDescriptor::is_safe)
    }

    /// Whether every dimension required to record this row is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.downgrade_consistent()
            && self.scope_vocabulary_ok()
            && self.selection_identity_ok()
            && self.narrowing_disclosure_ok()
            && self.visible_versus_matching_ok()
            && self.batch_preview_ok()
            && self.saved_view.is_valid()
            && self.column_preset.is_valid()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.source_contract_refs.is_empty()
            && self
                .source_contract_refs
                .iter()
                .all(|r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixGuardrails {
    /// Durable selection survives by stable identity; a row highlight never
    /// stands in for it.
    pub selection_durable_by_stable_identity: bool,
    /// Provider or policy narrowing is always visible, never hidden in a generic
    /// filter chip.
    pub provider_policy_narrowing_always_visible: bool,
    /// Visible, loaded, and all-matching counts never blur.
    pub visible_loaded_matching_counts_never_blur: bool,
    /// Visible rows are never treated as all matching rows without an explicit step.
    pub visible_never_all_matching_without_explicit_step: bool,
    /// Broad batch actions never bypass preview because the list is virtualized or
    /// provider-backed.
    pub broad_batch_actions_never_bypass_preview: bool,
    /// Any row lacking an identified filter / scope / count / batch class
    /// auto-downgrades below its claim.
    pub rows_auto_downgrade_on_unidentified_semantics: bool,
}

impl MatrixGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.selection_durable_by_stable_identity
            && self.provider_policy_narrowing_always_visible
            && self.visible_loaded_matching_counts_never_blur
            && self.visible_never_all_matching_without_explicit_step
            && self.broad_batch_actions_never_bypass_preview
            && self.rows_auto_downgrade_on_unidentified_semantics
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixConsumerProjection {
    /// Product surfaces ingest this matrix instead of cloning collection semantics.
    pub product_ingests_matrix: bool,
    /// Docs/help ingests the same matrix.
    pub docs_help_ingests_matrix: bool,
    /// Diagnostics ingests the same matrix.
    pub diagnostics_ingests_matrix: bool,
    /// Accessibility guidance ingests the same matrix.
    pub accessibility_ingests_matrix: bool,
    /// Release-control surfaces ingest the same matrix.
    pub release_control_ingests_matrix: bool,
    /// Downgraded rows are visibly labeled below current in every surface.
    pub downgraded_rows_labeled_below_current: bool,
}

impl MatrixConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_ingests_matrix
            && self.docs_help_ingests_matrix
            && self.diagnostics_ingests_matrix
            && self.accessibility_ingests_matrix
            && self.release_control_ingests_matrix
            && self.downgraded_rows_labeled_below_current
    }
}

/// Evidence freshness block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixEvidenceFreshness {
    /// Evidence-freshness SLO in hours.
    pub evidence_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last evidence refresh.
    pub last_evidence_refresh: String,
    /// True when stale evidence automatically downgrades claimed rows.
    pub auto_downgrade_on_stale: bool,
}

/// Constructor input for [`CollectionQualificationMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionQualificationMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Per-row qualifications.
    pub rows: Vec<CollectionQualificationRow>,
    /// Guardrail invariants block.
    pub guardrails: MatrixGuardrails,
    /// Consumer projection block.
    pub consumer_projection: MatrixConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: MatrixEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe dense-collection qualification matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionQualificationMatrixPacket {
    /// Record kind; must equal [`M5_COLLECTION_QUALIFICATION_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Per-row qualifications.
    pub rows: Vec<CollectionQualificationRow>,
    /// Guardrail invariants block.
    pub guardrails: MatrixGuardrails,
    /// Consumer projection block.
    pub consumer_projection: MatrixConsumerProjection,
    /// Evidence freshness block.
    pub evidence_freshness: MatrixEvidenceFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl CollectionQualificationMatrixPacket {
    /// Builds a dense-collection qualification matrix packet.
    pub fn new(input: CollectionQualificationMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_COLLECTION_QUALIFICATION_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            evidence_freshness: input.evidence_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some row in this matrix.
    pub fn represented_surfaces(&self) -> BTreeSet<DenseCollectionSurface> {
        self.rows.iter().map(|row| row.surface).collect()
    }

    /// Count of rows whose effective qualification was downgraded below its claim.
    pub fn downgraded_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.needs_downgrade()).count()
    }

    /// Count of rows holding a public claim.
    pub fn claimed_row_count(&self) -> usize {
        self.rows.iter().filter(|row| row.is_claimed()).count()
    }

    /// Validates the dense-collection qualification matrix invariants.
    pub fn validate(&self) -> Vec<CollectionQualificationMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COLLECTION_QUALIFICATION_MATRIX_RECORD_KIND {
            violations.push(CollectionQualificationMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_VERSION {
            violations.push(CollectionQualificationMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(CollectionQualificationMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_evidence_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("collection qualification matrix packet serializes"),
        ) {
            violations.push(CollectionQualificationMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("collection qualification matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Dense Collection Qualification Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Rows: {} ({} claimed, {} downgraded)\n",
            self.rows.len(),
            self.claimed_row_count(),
            self.downgraded_row_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            DenseCollectionSurface::ALL.len()
        ));
        out.push_str(&format!(
            "- Evidence freshness SLO: {} hours (last refresh: {})\n",
            self.evidence_freshness.evidence_freshness_slo_hours,
            self.evidence_freshness.last_evidence_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}): claim `{}` -> effective `{}`\n",
                row.row_id,
                row.surface.as_str(),
                row.claimed_qualification.as_str(),
                row.effective_qualification.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - filter=`{}` scope=`{}` counter=`{}` batch=`{}`\n",
                row.filter_ast_class
                    .map_or("unidentified", FilterAstClass::as_str),
                row.selection_scope_class
                    .map_or("unidentified", SelectionScopeClass::as_str),
                row.result_counter_class
                    .map_or("unidentified", ResultCounterClass::as_str),
                row.batch_action_class
                    .map_or("unidentified", BatchActionScopeClass::as_str),
            ));
            out.push_str(&format!(
                "  - actions: {}\n",
                if row.batch_action_descriptors.is_empty() {
                    "none".to_owned()
                } else {
                    row.batch_action_descriptors
                        .iter()
                        .map(|descriptor| descriptor.action_kind.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            ));
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in matrix export.
#[derive(Debug)]
pub enum CollectionQualificationMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<CollectionQualificationMatrixViolation>),
}

impl fmt::Display for CollectionQualificationMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "collection qualification matrix export parse failed: {error}"
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
                    "collection qualification matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for CollectionQualificationMatrixArtifactError {}

/// Validation failures emitted by [`CollectionQualificationMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionQualificationMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required dense collection surface is represented by no row.
    RequiredSurfaceMissing,
    /// No row demonstrates auto-downgrade on an unidentified identity dimension.
    DowngradedRowCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A claimed row was not downgraded below its claim despite a missing
    /// dimension.
    RowNotDowngradedOnUnidentifiedSemantics,
    /// A downgraded row lacks a precise degraded label or downgrade trigger.
    DowngradedRowMissingLabelOrTrigger,
    /// Selection is not durable by stable identity.
    SelectionNotDurableByStableIdentity,
    /// Provider or policy narrowing is hidden in a generic chip.
    ProviderPolicyNarrowingHidden,
    /// Visible rows are treated as all matching rows without an explicit step.
    VisibleTreatedAsAllMatching,
    /// A broad batch action bypasses preview or omits its scope receipt.
    BroadBatchActionBypassesPreview,
    /// The scope-counter vocabulary is incomplete for an identified counter.
    ScopeVocabularyIncomplete,
    /// A saved view captures transient selection or a provider cursor.
    SavedViewCapturesTransientState,
    /// A column preset drops a required identity column.
    ColumnPresetDropsIdentityColumn,
    /// A row lacks evidence refs.
    RowEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Evidence freshness block is incomplete.
    EvidenceFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl CollectionQualificationMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::DowngradedRowCaseMissing => "downgraded_row_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::RowNotDowngradedOnUnidentifiedSemantics => {
                "row_not_downgraded_on_unidentified_semantics"
            }
            Self::DowngradedRowMissingLabelOrTrigger => "downgraded_row_missing_label_or_trigger",
            Self::SelectionNotDurableByStableIdentity => "selection_not_durable_by_stable_identity",
            Self::ProviderPolicyNarrowingHidden => "provider_policy_narrowing_hidden",
            Self::VisibleTreatedAsAllMatching => "visible_treated_as_all_matching",
            Self::BroadBatchActionBypassesPreview => "broad_batch_action_bypasses_preview",
            Self::ScopeVocabularyIncomplete => "scope_vocabulary_incomplete",
            Self::SavedViewCapturesTransientState => "saved_view_captures_transient_state",
            Self::ColumnPresetDropsIdentityColumn => "column_preset_drops_identity_column",
            Self::RowEvidenceMissing => "row_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::EvidenceFreshnessIncomplete => "evidence_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable matrix export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_collection_qualification_matrix_export(
) -> Result<CollectionQualificationMatrixPacket, CollectionQualificationMatrixArtifactError> {
    let packet: CollectionQualificationMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m5/freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix/support_export.json"
    )))
    .map_err(CollectionQualificationMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(CollectionQualificationMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &CollectionQualificationMatrixPacket,
    violations: &mut Vec<CollectionQualificationMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COLLECTION_QUALIFICATION_MATRIX_SCHEMA_REF,
        M5_COLLECTION_QUALIFICATION_MATRIX_DOC_REF,
        M5_COLLECTION_QUALIFICATION_MATRIX_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(CollectionQualificationMatrixViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &CollectionQualificationMatrixPacket,
    violations: &mut Vec<CollectionQualificationMatrixViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in DenseCollectionSurface::ALL {
        if !surfaces.contains(&required) {
            violations.push(CollectionQualificationMatrixViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .rows
        .iter()
        .any(|row| row.needs_downgrade() && row.downgrade_consistent())
    {
        violations.push(CollectionQualificationMatrixViolation::DowngradedRowCaseMissing);
    }
}

fn validate_rows(
    packet: &CollectionQualificationMatrixPacket,
    violations: &mut Vec<CollectionQualificationMatrixViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(CollectionQualificationMatrixViolation::RowIncomplete);
        }
        if row.needs_downgrade()
            && row.effective_qualification.rank() >= row.claimed_qualification.rank()
        {
            violations.push(
                CollectionQualificationMatrixViolation::RowNotDowngradedOnUnidentifiedSemantics,
            );
        }
        if row.needs_downgrade()
            && (row.downgrade_trigger.is_none()
                || !row
                    .degraded_label
                    .as_ref()
                    .is_some_and(|label| !label_is_generic(label)))
        {
            violations
                .push(CollectionQualificationMatrixViolation::DowngradedRowMissingLabelOrTrigger);
        }
        if !row.selection_identity_ok() {
            violations
                .push(CollectionQualificationMatrixViolation::SelectionNotDurableByStableIdentity);
        }
        if !row.narrowing_disclosure_ok() {
            violations.push(CollectionQualificationMatrixViolation::ProviderPolicyNarrowingHidden);
        }
        if !row.visible_versus_matching_ok() {
            violations.push(CollectionQualificationMatrixViolation::VisibleTreatedAsAllMatching);
        }
        if !row.batch_preview_ok() {
            violations
                .push(CollectionQualificationMatrixViolation::BroadBatchActionBypassesPreview);
        }
        if !row.scope_vocabulary_ok() {
            violations.push(CollectionQualificationMatrixViolation::ScopeVocabularyIncomplete);
        }
        if !row.saved_view.is_valid() {
            violations
                .push(CollectionQualificationMatrixViolation::SavedViewCapturesTransientState);
        }
        if !row.column_preset.is_valid() {
            violations
                .push(CollectionQualificationMatrixViolation::ColumnPresetDropsIdentityColumn);
        }
        if row.evidence_refs.is_empty() || row.evidence_refs.iter().any(|r| r.trim().is_empty()) {
            violations.push(CollectionQualificationMatrixViolation::RowEvidenceMissing);
        }
    }
}

fn validate_guardrails(
    packet: &CollectionQualificationMatrixPacket,
    violations: &mut Vec<CollectionQualificationMatrixViolation>,
) {
    if !packet.guardrails.all_hold() {
        violations.push(CollectionQualificationMatrixViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &CollectionQualificationMatrixPacket,
    violations: &mut Vec<CollectionQualificationMatrixViolation>,
) {
    if !packet.consumer_projection.all_hold() {
        violations.push(CollectionQualificationMatrixViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_evidence_freshness(
    packet: &CollectionQualificationMatrixPacket,
    violations: &mut Vec<CollectionQualificationMatrixViolation>,
) {
    if packet.evidence_freshness.evidence_freshness_slo_hours == 0
        || packet
            .evidence_freshness
            .last_evidence_refresh
            .trim()
            .is_empty()
    {
        violations.push(CollectionQualificationMatrixViolation::EvidenceFreshnessIncomplete);
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
