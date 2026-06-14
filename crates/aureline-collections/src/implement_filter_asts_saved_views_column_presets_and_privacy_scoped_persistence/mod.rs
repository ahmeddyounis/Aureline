//! Privacy-scoped persistence binding for filter ASTs, saved views, and column
//! presets across the first real M5 dense collection surfaces.
//!
//! Where
//! [`crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix`]
//! froze the *qualification matrix* — the per-surface classification of filter,
//! scope, count, and batch semantics — this module binds the frozen taxonomy and
//! the shared `aureline-search` collection substrate into **durable, reopenable,
//! privacy-scoped persistence records** for the first real M5 consumers: the
//! pipeline run list, the review queue, the incident list, the graph list, the
//! marketplace results grid, and the provider/admin table.
//!
//! Each [`PersistedCollectionState`] binds one [`DenseCollectionSurface`] to a
//! real [`SavedCollectionView`] from `aureline-search` — the same shared object
//! search, review, and admin grids already speak — rather than minting a
//! surface-local serialization blob. The binding adds the persistence dimensions
//! this lane owns: a persisted schema version, a [`PersistenceCompatibility`]
//! state, an [`IncompatibilityResolution`] that fails *visibly* with migration or
//! reset when the persisted state can no longer replay exactly, and explicit
//! flags that the persisted state never carries transient selection, provider
//! cursors, or secret-bearing material.
//!
//! The lane is a **first consumer**, not just a contract: [`PersistedCollectionState::reopen`]
//! reconstructs the filter/view/column state a surface had active and reports the
//! [`ReopenOutcome`] (restored exactly, restored after migration, reset to
//! default, or refused pending rebind), and
//! [`PersistedCollectionState::support_reconstruction`] projects the same state
//! into a redaction-aware [`CollectionStateReconstruction`] that diagnostics and
//! support packets use to reconstruct which filter/view/column state the user had
//! active on a claimed M5 collection.
//!
//! Privacy scope has teeth here. A saved view whose owner scope is shared or
//! provider-owned may not persist a `local_only_private` privacy class, and a
//! non-local view must carry a portable filter AST — so the persisted state
//! cannot smuggle local-only literals into a shared or synced view. Export-safe
//! and sync-safe projections drop transient selection and provider cursors, and
//! the packet validator refuses any export that carries raw boundary material.
//!
//! The boundary schema is
//! [`schemas/collections/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.schema.json`](../../../../schemas/collections/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.schema.json).
//! The contract doc is
//! [`docs/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.md`](../../../../docs/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.md).
//! The protected fixture directory is
//! [`fixtures/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip/`](../../../../fixtures/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_search::{SavedCollectionView, SavedViewOwnerScope, SavedViewPrivacyClass};

use crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::DenseCollectionSurface;
use crate::stabilize_filter_ast_saved_view_scope_pack_column_preset::ScopeCounterVocabularyTerm;

/// Stable record-kind tag carried by [`M5CollectionPersistencePacket`].
pub const M5_COLLECTION_PERSISTENCE_RECORD_KIND: &str =
    "m5_collection_privacy_scoped_persistence_packet";

/// Integer schema version for the persistence packet.
pub const M5_COLLECTION_PERSISTENCE_SCHEMA_VERSION: u32 = 1;

/// Current persisted-state schema version. A [`PersistedCollectionState`] whose
/// `persisted_schema_version` differs from this is not byte-compatible and must
/// carry a visible [`IncompatibilityResolution`].
pub const M5_PERSISTED_STATE_SCHEMA_VERSION: u32 = 2;

/// Repo-relative path of the boundary schema.
pub const M5_COLLECTION_PERSISTENCE_SCHEMA_REF: &str =
    "schemas/collections/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COLLECTION_PERSISTENCE_DOC_REF: &str =
    "docs/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COLLECTION_PERSISTENCE_FIXTURE_DIR: &str =
    "fixtures/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COLLECTION_PERSISTENCE_ARTIFACT_REF: &str =
    "artifacts/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_COLLECTION_PERSISTENCE_SUMMARY_REF: &str =
    "artifacts/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip.md";

/// Canonical scope-counter vocabulary every persisted binding must keep so the
/// visible / loaded / matching / selected distinctions never blur on reopen.
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

/// The first real M5 dense surfaces this lane binds to durable persistence.
const REQUIRED_PERSISTED_SURFACES: [DenseCollectionSurface; 6] = [
    DenseCollectionSurface::PipelineRunList,
    DenseCollectionSurface::ReviewQueue,
    DenseCollectionSurface::IncidentList,
    DenseCollectionSurface::GraphList,
    DenseCollectionSurface::MarketplaceResults,
    DenseCollectionSurface::ProviderAdminTable,
];

/// Compatibility of a persisted collection state with the current schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistenceCompatibility {
    /// The persisted schema version matches the current one; the state replays
    /// exactly on reopen.
    Current,
    /// The persisted state predates the current schema but can be migrated
    /// forward without losing filter/view/column meaning.
    MigratableForward,
    /// The persisted state cannot be migrated and must be reset or rebound.
    IncompatibleNeedsReset,
}

impl PersistenceCompatibility {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::MigratableForward => "migratable_forward",
            Self::IncompatibleNeedsReset => "incompatible_needs_reset",
        }
    }

    /// True when the state replays exactly without migration or reset.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// How a persisted state that is not [`PersistenceCompatibility::Current`]
/// resolves on reopen. The resolution is shown to the operator verbatim; an
/// incompatible state never silently drops the user's filter/view/column choices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncompatibilityResolution {
    /// Migrate the persisted state forward to the current schema.
    MigrateForward,
    /// Reset the persisted state to the surface default and disclose the loss.
    ResetToDefault,
    /// Refuse to load until the user or provider rebinds the drifted state.
    RefuseUntilRebound,
}

impl IncompatibilityResolution {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigrateForward => "migrate_forward",
            Self::ResetToDefault => "reset_to_default",
            Self::RefuseUntilRebound => "refuse_until_rebound",
        }
    }
}

/// Outcome of reopening a persisted collection state. Returned by
/// [`PersistedCollectionState::reopen`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReopenOutcome {
    /// The persisted filter/view/column state was restored exactly.
    RestoredExact,
    /// The persisted state was migrated forward, then restored.
    RestoredAfterMigration,
    /// The persisted state was reset to the surface default with a visible label.
    ResetToDefault,
    /// The persisted state was refused pending rebind, with a visible label.
    RefusedNeedsRebind,
}

impl ReopenOutcome {
    /// Stable token recorded in reconstructions.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoredExact => "restored_exact",
            Self::RestoredAfterMigration => "restored_after_migration",
            Self::ResetToDefault => "reset_to_default",
            Self::RefusedNeedsRebind => "refused_needs_rebind",
        }
    }

    /// True when the user's exact filter/view/column state survived reopen
    /// (either directly or after a lossless migration).
    pub const fn restored_state(self) -> bool {
        matches!(self, Self::RestoredExact | Self::RestoredAfterMigration)
    }
}

/// Persisted column-preset state bound to a surface. The required identity
/// columns may never be dropped from the visible set when the preset is restored.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedColumnPreset {
    /// Stable column-preset id.
    pub column_preset_id: String,
    /// Ordered visible columns restored on reopen.
    pub visible_column_ids: Vec<String>,
    /// Pinned columns that cannot be silently hidden.
    pub pinned_column_ids: Vec<String>,
    /// Identity or provenance columns that may not be dropped from the visible set.
    pub required_identity_column_ids: Vec<String>,
    /// Density-mode token.
    pub density_mode_token: String,
}

impl PersistedColumnPreset {
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

/// Redaction-aware reconstruction of the filter/view/column state a surface had
/// active, projected for diagnostics and support packets. Carries only ids,
/// tokens, labels, and counts — never raw filter literals, provider cursors, or
/// selection identities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionStateReconstruction {
    /// Binding id this reconstruction projects.
    pub binding_id: String,
    /// Surface token.
    pub surface_token: String,
    /// Saved-view id that backs the state.
    pub saved_view_id: String,
    /// Saved-view scope label (no raw literal bytes).
    pub scope_label: String,
    /// Owner-scope token.
    pub owner_scope_token: String,
    /// Privacy-class token.
    pub privacy_class_token: String,
    /// Filter AST id replayed by the state.
    pub filter_ast_id: String,
    /// Number of filter clauses captured.
    pub filter_clause_count: usize,
    /// Visible column ids restored on reopen.
    pub visible_column_ids: Vec<String>,
    /// Pinned column ids restored on reopen.
    pub pinned_column_ids: Vec<String>,
    /// Compatibility token.
    pub compatibility_token: String,
    /// Resolution token, present only when the state is not current.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolution_token: Option<String>,
    /// Reopen outcome token.
    pub reopen_outcome_token: String,
    /// Whether the reconstruction restored the exact user state.
    pub restored_exact_state: bool,
}

/// One durable, privacy-scoped persistence binding for a dense M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedCollectionState {
    /// Stable binding id.
    pub binding_id: String,
    /// Bound dense collection surface.
    pub surface: DenseCollectionSurface,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Shared saved-view object that carries the persisted filter AST, columns,
    /// owner scope, privacy class, and fallback behavior.
    pub saved_view: SavedCollectionView,
    /// Persisted column-preset state bound to the surface.
    pub column_preset: PersistedColumnPreset,
    /// Canonical scope-counter vocabulary terms the surface keeps on reopen.
    pub scope_vocabulary_terms: Vec<ScopeCounterVocabularyTerm>,
    /// Schema version the state was persisted under.
    pub persisted_schema_version: u32,
    /// Compatibility of the persisted state with the current schema.
    pub compatibility: PersistenceCompatibility,
    /// Resolution for an incompatible state, required when the state is not
    /// current.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incompatibility_resolution: Option<IncompatibilityResolution>,
    /// Precise, visible label for an incompatible state, required when the state
    /// is not current.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incompatibility_label: Option<String>,
    /// True when reopen can rebind to current roots, providers, and policy.
    pub reopen_rebind_supported: bool,
    /// True when the persisted state excludes transient selection (required).
    pub excludes_transient_selection: bool,
    /// True when the persisted state excludes provider cursors (required).
    pub excludes_provider_cursor: bool,
    /// True when the persisted state excludes secret-bearing material (required).
    pub excludes_secret_material: bool,
    /// Evidence packet refs backing this binding.
    pub evidence_refs: Vec<String>,
}

impl PersistedCollectionState {
    /// Whether the persisted schema version matches the current one.
    pub fn schema_version_current(&self) -> bool {
        self.persisted_schema_version == M5_PERSISTED_STATE_SCHEMA_VERSION
    }

    /// Whether the saved view may be exported or synced beyond the local profile.
    /// A `local_only_private` view stays local; anything else is portable.
    pub fn is_portable(&self) -> bool {
        !matches!(
            self.saved_view.privacy_class,
            SavedViewPrivacyClass::LocalOnlyPrivate
        )
    }

    /// Whether the owner scope demands a portable privacy class. A shared or
    /// provider-owned view must not be persisted as `local_only_private`.
    pub fn owner_scope_requires_portable(&self) -> bool {
        matches!(
            self.saved_view.owner_scope,
            SavedViewOwnerScope::Shared
                | SavedViewOwnerScope::ProviderOwned
                | SavedViewOwnerScope::PolicyPinned
        )
    }

    /// Whether the privacy scope is internally consistent: a shared, policy, or
    /// provider-owned view is portable, and a portable view carries a portable
    /// filter AST (no local-only literals) so it never leaks local material.
    pub fn privacy_scope_consistent(&self) -> bool {
        if self.owner_scope_requires_portable() && !self.is_portable() {
            return false;
        }
        if self.is_portable() && !self.saved_view.filter_ast.is_portable() {
            return false;
        }
        true
    }

    /// Whether the persisted state never carries transient selection, provider
    /// cursors, or secret material — checked both on this binding and on the
    /// shared saved-view object.
    pub fn excludes_transient_state(&self) -> bool {
        self.excludes_transient_selection
            && self.excludes_provider_cursor
            && self.excludes_secret_material
            && !self.saved_view.captures_selection
            && !self.saved_view.captures_provider_cursor
    }

    /// Whether the compatibility state and its resolution evidence are
    /// consistent.
    ///
    /// A current state matches the schema version, carries no resolution, and
    /// needs no incompatibility label. A non-current state must mismatch the
    /// schema version, carry a resolution and a precise label, and must not claim
    /// a forward migration when the state is genuinely incompatible.
    pub fn compatibility_consistent(&self) -> bool {
        match self.compatibility {
            PersistenceCompatibility::Current => {
                self.schema_version_current()
                    && self.incompatibility_resolution.is_none()
                    && self.incompatibility_label.is_none()
            }
            PersistenceCompatibility::MigratableForward => {
                !self.schema_version_current()
                    && self.incompatibility_resolution.is_some()
                    && self
                        .incompatibility_label
                        .as_ref()
                        .is_some_and(|label| !label_is_generic(label))
            }
            PersistenceCompatibility::IncompatibleNeedsReset => {
                !self.schema_version_current()
                    && !matches!(
                        self.incompatibility_resolution,
                        Some(IncompatibilityResolution::MigrateForward) | None
                    )
                    && self
                        .incompatibility_label
                        .as_ref()
                        .is_some_and(|label| !label_is_generic(label))
            }
        }
    }

    /// Whether the scope-counter vocabulary is complete so visible / loaded /
    /// matching / selected never blur on reopen.
    pub fn scope_vocabulary_ok(&self) -> bool {
        let present: BTreeSet<_> = self.scope_vocabulary_terms.iter().copied().collect();
        REQUIRED_SCOPE_VOCABULARY_TERMS
            .iter()
            .all(|term| present.contains(term))
    }

    /// Reconstructs the reopen outcome for the persisted state.
    ///
    /// A current state restores exactly. A migratable state restores after
    /// migration, resets, or refuses depending on its declared resolution. An
    /// incompatible state resets or refuses. The outcome never silently restores
    /// an incompatible state as if it were exact.
    pub fn reopen(&self) -> ReopenOutcome {
        match self.compatibility {
            PersistenceCompatibility::Current => ReopenOutcome::RestoredExact,
            PersistenceCompatibility::MigratableForward => match self.incompatibility_resolution {
                Some(IncompatibilityResolution::MigrateForward) => {
                    ReopenOutcome::RestoredAfterMigration
                }
                Some(IncompatibilityResolution::ResetToDefault) => ReopenOutcome::ResetToDefault,
                _ => ReopenOutcome::RefusedNeedsRebind,
            },
            PersistenceCompatibility::IncompatibleNeedsReset => {
                match self.incompatibility_resolution {
                    Some(IncompatibilityResolution::ResetToDefault) => {
                        ReopenOutcome::ResetToDefault
                    }
                    _ => ReopenOutcome::RefusedNeedsRebind,
                }
            }
        }
    }

    /// Projects the persisted state into a redaction-aware reconstruction for
    /// diagnostics and support packets.
    pub fn support_reconstruction(&self) -> CollectionStateReconstruction {
        let outcome = self.reopen();
        CollectionStateReconstruction {
            binding_id: self.binding_id.clone(),
            surface_token: self.surface.as_str().to_owned(),
            saved_view_id: self.saved_view.saved_view_id.clone(),
            scope_label: self.saved_view.scope_label.clone(),
            owner_scope_token: self.saved_view.owner_scope.as_str().to_owned(),
            privacy_class_token: self.saved_view.privacy_class.as_str().to_owned(),
            filter_ast_id: self.saved_view.filter_ast.filter_ast_id.clone(),
            filter_clause_count: self.saved_view.filter_ast.clauses().len(),
            visible_column_ids: self.column_preset.visible_column_ids.clone(),
            pinned_column_ids: self.column_preset.pinned_column_ids.clone(),
            compatibility_token: self.compatibility.as_str().to_owned(),
            resolution_token: self
                .incompatibility_resolution
                .map(|resolution| resolution.as_str().to_owned()),
            reopen_outcome_token: outcome.as_str().to_owned(),
            restored_exact_state: outcome.restored_state(),
        }
    }

    /// Whether every dimension required to record this binding is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.binding_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.saved_view_well_formed()
            && self.column_preset.is_valid()
            && self.scope_vocabulary_ok()
            && self.persisted_schema_version > 0
            && self.compatibility_consistent()
            && self.privacy_scope_consistent()
            && self.excludes_transient_state()
            && self.reopen_rebind_supported
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    fn saved_view_well_formed(&self) -> bool {
        !self.saved_view.saved_view_id.trim().is_empty()
            && !self.saved_view.scope_label.trim().is_empty()
            && self.saved_view.filter_ast.validate().is_empty()
    }
}

/// Persistence guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistenceGuardrails {
    /// State reopens through shared collection objects rather than surface-local
    /// serialization.
    pub reopen_through_shared_objects: bool,
    /// Saved views preserve owner and privacy scope.
    pub saved_views_preserve_owner_privacy_scope: bool,
    /// Transient selection and provider cursors are never persisted.
    pub transient_selection_and_cursors_never_persisted: bool,
    /// Incompatible state fails visibly with migration or reset.
    pub incompatible_state_fails_visibly: bool,
    /// Diagnostics and support can reconstruct active filter/view/column state.
    pub support_can_reconstruct_active_state: bool,
}

impl PersistenceGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.reopen_through_shared_objects
            && self.saved_views_preserve_owner_privacy_scope
            && self.transient_selection_and_cursors_never_persisted
            && self.incompatible_state_fails_visibly
            && self.support_can_reconstruct_active_state
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistenceConsumerProjection {
    /// Product surfaces reopen state from these shared persistence records.
    pub product_reopens_from_records: bool,
    /// Diagnostics reconstruct active state from these records.
    pub diagnostics_reconstructs_from_records: bool,
    /// Support/export reuses the export-safe persistence projection.
    pub support_export_reuses_records: bool,
    /// Sync reuses the sync-safe persistence projection.
    pub sync_reuses_records: bool,
}

impl PersistenceConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_reopens_from_records
            && self.diagnostics_reconstructs_from_records
            && self.support_export_reuses_records
            && self.sync_reuses_records
    }
}

/// Constructor input for [`M5CollectionPersistencePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CollectionPersistencePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-surface persistence bindings.
    pub bindings: Vec<PersistedCollectionState>,
    /// Guardrail invariants block.
    pub guardrails: PersistenceGuardrails,
    /// Consumer projection block.
    pub consumer_projection: PersistenceConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe, privacy-scoped collection persistence packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CollectionPersistencePacket {
    /// Record kind; must equal [`M5_COLLECTION_PERSISTENCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COLLECTION_PERSISTENCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-surface persistence bindings.
    pub bindings: Vec<PersistedCollectionState>,
    /// Guardrail invariants block.
    pub guardrails: PersistenceGuardrails,
    /// Consumer projection block.
    pub consumer_projection: PersistenceConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CollectionPersistencePacket {
    /// Builds a privacy-scoped collection persistence packet.
    pub fn new(input: M5CollectionPersistencePacketInput) -> Self {
        Self {
            record_kind: M5_COLLECTION_PERSISTENCE_RECORD_KIND.to_owned(),
            schema_version: M5_COLLECTION_PERSISTENCE_SCHEMA_VERSION,
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

    /// Count of bindings whose persisted state is not current.
    pub fn incompatible_binding_count(&self) -> usize {
        self.bindings
            .iter()
            .filter(|binding| !binding.compatibility.is_current())
            .count()
    }

    /// Reconstructions for every binding, used by diagnostics and support packets.
    pub fn support_reconstructions(&self) -> Vec<CollectionStateReconstruction> {
        self.bindings
            .iter()
            .map(PersistedCollectionState::support_reconstruction)
            .collect()
    }

    /// Validates the persistence packet invariants.
    pub fn validate(&self) -> Vec<M5CollectionPersistenceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COLLECTION_PERSISTENCE_RECORD_KIND {
            violations.push(M5CollectionPersistenceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COLLECTION_PERSISTENCE_SCHEMA_VERSION {
            violations.push(M5CollectionPersistenceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CollectionPersistenceViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(M5CollectionPersistenceViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5CollectionPersistenceViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("collection persistence packet serializes"),
        ) {
            violations.push(M5CollectionPersistenceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("collection persistence packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Collection Privacy-Scoped Persistence\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Bindings: {} ({} incompatible)\n",
            self.bindings.len(),
            self.incompatible_binding_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            REQUIRED_PERSISTED_SURFACES.len()
        ));
        out.push_str("\n## Bindings\n\n");
        for binding in &self.bindings {
            let outcome = binding.reopen();
            out.push_str(&format!(
                "- **{}** ({}): view `{}` scope `{}/{}`\n",
                binding.binding_id,
                binding.surface.as_str(),
                binding.saved_view.saved_view_id,
                binding.saved_view.owner_scope.as_str(),
                binding.saved_view.privacy_class.as_str(),
            ));
            out.push_str(&format!("  - {}\n", binding.label_summary));
            out.push_str(&format!(
                "  - compatibility=`{}` reopen=`{}`\n",
                binding.compatibility.as_str(),
                outcome.as_str(),
            ));
            if let Some(label) = &binding.incompatibility_label {
                out.push_str(&format!("  - Incompatible: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in persistence export.
#[derive(Debug)]
pub enum M5CollectionPersistenceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CollectionPersistenceViolation>),
}

impl fmt::Display for M5CollectionPersistenceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "collection persistence export parse failed: {error}"
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
                    "collection persistence export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CollectionPersistenceArtifactError {}

/// Validation failures emitted by [`M5CollectionPersistencePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CollectionPersistenceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required real M5 surface is bound by no persistence record.
    RequiredSurfaceMissing,
    /// No binding demonstrates the incompatible-state migration/reset path.
    IncompatibleCaseMissing,
    /// A binding is incomplete.
    BindingIncomplete,
    /// A binding's compatibility state and resolution evidence are inconsistent.
    CompatibilityInconsistent,
    /// A binding's owner/privacy scope is inconsistent (shared view kept local,
    /// or a portable view carries local-only filter literals).
    PrivacyScopeInconsistent,
    /// A binding persists transient selection or a provider cursor.
    PersistsTransientState,
    /// A binding's column preset drops a required identity column.
    ColumnPresetDropsIdentityColumn,
    /// A binding's scope-counter vocabulary is incomplete.
    ScopeVocabularyIncomplete,
    /// A binding lacks evidence refs.
    BindingEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5CollectionPersistenceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::IncompatibleCaseMissing => "incompatible_case_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::CompatibilityInconsistent => "compatibility_inconsistent",
            Self::PrivacyScopeInconsistent => "privacy_scope_inconsistent",
            Self::PersistsTransientState => "persists_transient_state",
            Self::ColumnPresetDropsIdentityColumn => "column_preset_drops_identity_column",
            Self::ScopeVocabularyIncomplete => "scope_vocabulary_incomplete",
            Self::BindingEvidenceMissing => "binding_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable persistence export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_collection_persistence_export(
) -> Result<M5CollectionPersistencePacket, M5CollectionPersistenceArtifactError> {
    let packet: M5CollectionPersistencePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m5/implement-filter-asts-saved-views-column-presets-and-privacy-scoped-persistence-across-pip/support_export.json"
    )))
    .map_err(M5CollectionPersistenceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CollectionPersistenceArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5CollectionPersistencePacket,
    violations: &mut Vec<M5CollectionPersistenceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COLLECTION_PERSISTENCE_SCHEMA_REF,
        M5_COLLECTION_PERSISTENCE_DOC_REF,
        M5_COLLECTION_PERSISTENCE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CollectionPersistenceViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &M5CollectionPersistencePacket,
    violations: &mut Vec<M5CollectionPersistenceViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in REQUIRED_PERSISTED_SURFACES {
        if !surfaces.contains(&required) {
            violations.push(M5CollectionPersistenceViolation::RequiredSurfaceMissing);
            break;
        }
    }

    if !packet
        .bindings
        .iter()
        .any(|binding| !binding.compatibility.is_current() && binding.compatibility_consistent())
    {
        violations.push(M5CollectionPersistenceViolation::IncompatibleCaseMissing);
    }
}

fn validate_bindings(
    packet: &M5CollectionPersistencePacket,
    violations: &mut Vec<M5CollectionPersistenceViolation>,
) {
    for binding in &packet.bindings {
        if !binding.is_complete() {
            violations.push(M5CollectionPersistenceViolation::BindingIncomplete);
        }
        if !binding.compatibility_consistent() {
            violations.push(M5CollectionPersistenceViolation::CompatibilityInconsistent);
        }
        if !binding.privacy_scope_consistent() {
            violations.push(M5CollectionPersistenceViolation::PrivacyScopeInconsistent);
        }
        if !binding.excludes_transient_state() {
            violations.push(M5CollectionPersistenceViolation::PersistsTransientState);
        }
        if !binding.column_preset.is_valid() {
            violations.push(M5CollectionPersistenceViolation::ColumnPresetDropsIdentityColumn);
        }
        if !binding.scope_vocabulary_ok() {
            violations.push(M5CollectionPersistenceViolation::ScopeVocabularyIncomplete);
        }
        if binding.evidence_refs.is_empty()
            || binding.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(M5CollectionPersistenceViolation::BindingEvidenceMissing);
        }
    }
}

/// Whether an incompatibility label is a generic non-answer rather than a precise
/// label. A generic provider error must never stand in for a precise truth.
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
            | "incompatible"
            | "reset"
            | "migrate"
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
