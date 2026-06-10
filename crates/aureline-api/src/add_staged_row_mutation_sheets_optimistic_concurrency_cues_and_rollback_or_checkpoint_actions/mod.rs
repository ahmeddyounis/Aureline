//! Staged row-mutation sheets, optimistic-concurrency cues, and rollback or
//! checkpoint action qualification records.
//!
//! This module owns the typed records that keep staged row mutations,
//! optimistic-concurrency control cues, rollback actions, and checkpoint
//! actions inspectable and attributable without depending on hidden shell
//! shortcuts or ad hoc scripts. The boundary schema is
//! [`/schemas/data/add-staged-row-mutation-sheets-optimistic-concurrency-cues-and-rollback-or-checkpoint-actions.schema.json`](../../../schemas/data/add-staged-row-mutation-sheets-optimistic-concurrency-cues-and-rollback-or-checkpoint-actions.schema.json)
//! and the checked-in qualification packet is
//! [`/artifacts/data/m5/add-staged-row-mutation-sheets-optimistic-concurrency-cues-and-rollback-or-checkpoint-actions.json`](../../../artifacts/data/m5/add-staged-row-mutation-sheets-optimistic-concurrency-cues-and-rollback-or-checkpoint-actions.json).
//!
//! Raw row values, raw primary keys, raw secrets, raw transaction IDs, and raw
//! database connection strings do not belong in these records. They carry stable
//! IDs, closed posture vocabularies, and reviewable summaries that UI, CLI,
//! export, support, and public-proof surfaces can ingest safely.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported schema version for staged row-mutation qualification packets.
pub const STAGED_ROW_MUTATION_QUALIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`StagedRowMutationQualificationPacket`].
pub const STAGED_ROW_MUTATION_QUALIFICATION_RECORD_KIND: &str =
    "staged_row_mutation_sheets_optimistic_concurrency_cues_and_rollback_or_checkpoint_actions";

/// Repo-relative path to the checked-in staged row-mutation qualification packet.
pub const STAGED_ROW_MUTATION_QUALIFICATION_PACKET_PATH: &str =
    "artifacts/data/m5/add-staged-row-mutation-sheets-optimistic-concurrency-cues-and-rollback-or-checkpoint-actions.json";

/// Embedded checked-in packet JSON.
pub const STAGED_ROW_MUTATION_QUALIFICATION_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/data/m5/add-staged-row-mutation-sheets-optimistic-concurrency-cues-and-rollback-or-checkpoint-actions.json"
));

/// Qualification label shown on promoted staged row-mutation surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StagedRowMutationQualificationLabel {
    /// Surface has current proof and may be called stable for its declared scope.
    Stable,
    /// Surface is visible but below stable.
    Preview,
    /// Surface is an experiment or internal lab.
    Labs,
    /// Surface may inspect metadata but must not execute or export live data.
    InspectOnly,
    /// Surface may import or view captured files only.
    ImportOnly,
}

impl StagedRowMutationQualificationLabel {
    /// Returns true when the label is a stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Staged row-mutation surface family governed by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StagedRowMutationSurfaceKind {
    /// Staged row-mutation sheet showing pending inserts, updates, and deletes.
    StagedRowMutationSheet,
    /// Optimistic-concurrency control cue (version stamp, conflict warning).
    OptimisticConcurrencyCue,
    /// Rollback action for undoing staged or committed mutations.
    RollbackAction,
    /// Checkpoint action for saving a restore point within a mutation session.
    CheckpointAction,
    /// Mutation session viewer showing sheet state, auth, and target context.
    MutationSession,
}

/// Kind of row mutation staged in a sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationKind {
    /// Insert one or more rows.
    Insert,
    /// Update existing rows.
    Update,
    /// Delete existing rows.
    Delete,
    /// Upsert (insert or update) rows.
    Upsert,
    /// Batch mutation mixing multiple kinds.
    Batch,
}

/// Concurrency conflict class shown on optimistic-concurrency cues.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConcurrencyConflictClass {
    /// No conflict detected.
    NoConflict,
    /// Read data has changed since it was loaded.
    StaleRead,
    /// Row version does not match the expected version.
    VersionMismatch,
    /// Another writer modified the row concurrently.
    ConcurrentWrite,
    /// Policy blocks the mutation regardless of version state.
    PolicyBlocked,
}

/// Scope of a rollback action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackScope {
    /// Roll back a single row change.
    SingleRow,
    /// Roll back all changes in the current sheet.
    SheetLevel,
    /// Roll back all changes in the current mutation session.
    SessionLevel,
    /// Roll back to a named checkpoint.
    ToCheckpoint,
}

/// Scope of a checkpoint action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckpointScope {
    /// Checkpoint captured before any mutation in the session.
    PreMutation,
    /// Checkpoint captured immediately before commit.
    PreCommit,
    /// Checkpoint captured after successful commit.
    PostCommit,
    /// Automatically captured checkpoint at sheet open or mutation start.
    Automatic,
}

/// Proof packet metadata attached to a stable surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedRowMutationQualificationProof {
    /// Stable proof packet id.
    pub packet_id: String,
    /// Repo-relative proof artifact reference.
    pub packet_ref: String,
    /// Proof-index reference.
    pub proof_index_ref: String,
    /// UTC capture date.
    pub captured_at: String,
    /// Evidence artifact references.
    pub evidence_refs: Vec<String>,
}

/// Boolean guard set that keeps stable surfaces from inheriting generic table truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedRowMutationSurfaceGuardSet {
    /// Staged row-mutation sheet (pending inserts, updates, deletes) is visible.
    pub staged_sheet_visible: bool,
    /// Optimistic-concurrency cue (version stamp, conflict warning) is visible.
    pub concurrency_cue_visible: bool,
    /// Rollback action and scope disclosure are visible.
    pub rollback_action_visible: bool,
    /// Checkpoint action and restore-point disclosure are visible.
    pub checkpoint_action_visible: bool,
    /// Mutation session viewer (sheet state, auth, target context) is visible.
    pub mutation_session_visible: bool,
    /// Auth scope is visible without raw secrets.
    pub auth_scope_visible: bool,
    /// Target class and table identity are visible.
    pub target_class_visible: bool,
}

impl StagedRowMutationSurfaceGuardSet {
    /// Returns true when every required visible guard is present.
    pub const fn all_visible(&self) -> bool {
        self.staged_sheet_visible
            && self.concurrency_cue_visible
            && self.rollback_action_visible
            && self.checkpoint_action_visible
            && self.mutation_session_visible
            && self.auth_scope_visible
            && self.target_class_visible
    }
}

/// One governed surface row in the qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedRowMutationSurfaceQualificationRow {
    /// Stable surface identifier.
    pub surface_id: String,
    /// Reviewer-facing title.
    pub title: String,
    /// Surface family.
    pub surface_kind: StagedRowMutationSurfaceKind,
    /// Whether this surface is included in the promoted build.
    pub promoted_build_surface: bool,
    /// Claimed label from upstream release planning.
    pub claim_label: StagedRowMutationQualificationLabel,
    /// Actual displayed label after qualification.
    pub displayed_label: StagedRowMutationQualificationLabel,
    /// Proof packet when the surface is stable.
    pub qualification_packet: Option<StagedRowMutationQualificationProof>,
    /// Visible guard set.
    pub guards: StagedRowMutationSurfaceGuardSet,
    /// True when missing proof narrows below stable instead of inheriting a label.
    pub downgrade_if_missing: bool,
    /// Plain-language reason for the displayed label.
    pub rationale: String,
}

/// One staged row-mutation sheet row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedRowMutationSheetRow {
    /// Stable sheet id.
    pub sheet_id: String,
    /// Mutation kind.
    pub mutation_kind: MutationKind,
    /// Target table identity ref.
    pub target_table_ref: String,
    /// Number of rows affected (bounded, never unbounded).
    pub row_count_affected: u32,
    /// Whether the sheet is previewable before apply.
    pub previewable_before_apply: bool,
    /// Whether explicit confirmation is required before apply.
    pub confirmation_required: bool,
    /// Whether rollback path is visible from this sheet.
    pub rollback_path_visible: bool,
    /// Checkpoint ref, if any.
    pub checkpoint_ref: Option<String>,
    /// Auth scope ref.
    pub auth_scope_ref: String,
}

/// One optimistic-concurrency cue row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OptimisticConcurrencyCueRow {
    /// Stable cue id.
    pub cue_id: String,
    /// Concurrency conflict class.
    pub conflict_class: ConcurrencyConflictClass,
    /// Version ref (opaque, non-secret).
    pub version_ref: String,
    /// Last-modified timestamp ref.
    pub last_modified_ref: String,
    /// Whether resolution actions (refresh, overwrite, merge) are visible.
    pub resolution_action_visible: bool,
    /// Whether stale-read warning is visible.
    pub stale_read_warning_visible: bool,
    /// Target table identity ref.
    pub target_table_ref: String,
}

/// One rollback action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RollbackActionRow {
    /// Stable action id.
    pub action_id: String,
    /// Rollback scope.
    pub rollback_scope: RollbackScope,
    /// Whether explicit confirmation is required.
    pub requires_confirmation: bool,
    /// Affected sheet ref.
    pub affected_sheet_ref: String,
    /// Checkpoint ref, if rolling back to a checkpoint.
    pub checkpoint_ref: Option<String>,
    /// Undo stack depth (bounded).
    pub undo_stack_depth: u32,
}

/// One checkpoint action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckpointActionRow {
    /// Stable action id.
    pub action_id: String,
    /// Checkpoint scope.
    pub checkpoint_scope: CheckpointScope,
    /// Whether the checkpoint is captured automatically.
    pub auto_checkpoint: bool,
    /// Session ref.
    pub session_ref: String,
    /// Human-readable checkpoint label.
    pub label: String,
    /// Whether the checkpoint is restorable.
    pub restorable: bool,
}

/// Summary counts for a staged row-mutation qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedRowMutationQualificationSummary {
    /// Number of promoted surfaces.
    pub promoted_surface_count: usize,
    /// Number of stable surfaces.
    pub stable_surface_count: usize,
    /// Number of narrowed promoted surfaces.
    pub narrowed_surface_count: usize,
    /// Number of staged row-mutation sheet rows.
    pub staged_row_mutation_sheet_count: usize,
    /// Number of optimistic-concurrency cue rows.
    pub optimistic_concurrency_cue_count: usize,
    /// Number of rollback action rows.
    pub rollback_action_count: usize,
    /// Number of checkpoint action rows.
    pub checkpoint_action_count: usize,
}

/// Canonical staged row-mutation qualification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedRowMutationQualificationPacket {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Release document reference.
    pub release_doc_ref: String,
    /// Help document reference.
    pub help_doc_ref: String,
    /// JSON Schema path.
    pub schema_ref: String,
    /// Surface rows.
    pub surfaces: Vec<StagedRowMutationSurfaceQualificationRow>,
    /// Staged row-mutation sheet rows.
    pub staged_row_mutation_sheets: Vec<StagedRowMutationSheetRow>,
    /// Optimistic-concurrency cue rows.
    pub optimistic_concurrency_cues: Vec<OptimisticConcurrencyCueRow>,
    /// Rollback action rows.
    pub rollback_actions: Vec<RollbackActionRow>,
    /// Checkpoint action rows.
    pub checkpoint_actions: Vec<CheckpointActionRow>,
    /// Summary counts.
    pub summary: StagedRowMutationQualificationSummary,
}

impl StagedRowMutationQualificationPacket {
    /// Recomputes summary counts from packet rows.
    pub fn computed_summary(&self) -> StagedRowMutationQualificationSummary {
        let promoted_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.promoted_build_surface)
            .count();
        let stable_surface_count = self
            .surfaces
            .iter()
            .filter(|surface| surface.displayed_label.is_stable())
            .count();
        StagedRowMutationQualificationSummary {
            promoted_surface_count,
            stable_surface_count,
            narrowed_surface_count: promoted_surface_count.saturating_sub(stable_surface_count),
            staged_row_mutation_sheet_count: self.staged_row_mutation_sheets.len(),
            optimistic_concurrency_cue_count: self.optimistic_concurrency_cues.len(),
            rollback_action_count: self.rollback_actions.len(),
            checkpoint_action_count: self.checkpoint_actions.len(),
        }
    }

    /// Validates packet invariants for UI, CLI, support, and release consumers.
    pub fn validate(&self) -> Vec<StagedRowMutationQualificationViolation> {
        let mut violations = Vec::new();
        if self.schema_version != STAGED_ROW_MUTATION_QUALIFICATION_SCHEMA_VERSION {
            violations.push(StagedRowMutationQualificationViolation::SchemaVersion {
                expected: STAGED_ROW_MUTATION_QUALIFICATION_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != STAGED_ROW_MUTATION_QUALIFICATION_RECORD_KIND {
            violations.push(StagedRowMutationQualificationViolation::RecordKind {
                expected: STAGED_ROW_MUTATION_QUALIFICATION_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }

        collect_ids(
            self.surfaces
                .iter()
                .map(|surface| surface.surface_id.as_str()),
            &mut violations,
            StagedRowMutationQualificationViolationKind::Surface,
        );
        collect_ids(
            self.staged_row_mutation_sheets
                .iter()
                .map(|row| row.sheet_id.as_str()),
            &mut violations,
            StagedRowMutationQualificationViolationKind::StagedRowMutationSheet,
        );
        collect_ids(
            self.optimistic_concurrency_cues
                .iter()
                .map(|row| row.cue_id.as_str()),
            &mut violations,
            StagedRowMutationQualificationViolationKind::OptimisticConcurrencyCue,
        );
        collect_ids(
            self.rollback_actions
                .iter()
                .map(|row| row.action_id.as_str()),
            &mut violations,
            StagedRowMutationQualificationViolationKind::RollbackAction,
        );
        collect_ids(
            self.checkpoint_actions
                .iter()
                .map(|row| row.action_id.as_str()),
            &mut violations,
            StagedRowMutationQualificationViolationKind::CheckpointAction,
        );

        for surface in &self.surfaces {
            if surface.displayed_label.is_stable() {
                if surface.qualification_packet.is_none() {
                    violations.push(
                        StagedRowMutationQualificationViolation::StableSurfaceMissingProof {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
                if !surface.guards.all_visible() {
                    violations.push(
                        StagedRowMutationQualificationViolation::StableSurfaceMissingGuard {
                            surface_id: surface.surface_id.clone(),
                        },
                    );
                }
            }
            if !surface.displayed_label.is_stable()
                && surface.claim_label.is_stable()
                && !surface.downgrade_if_missing
            {
                violations.push(
                    StagedRowMutationQualificationViolation::NarrowedSurfaceLacksDowngradeRule {
                        surface_id: surface.surface_id.clone(),
                    },
                );
            }
        }

        let mutation_kinds: BTreeSet<_> = self
            .staged_row_mutation_sheets
            .iter()
            .map(|row| row.mutation_kind)
            .collect();
        for required_kind in [
            MutationKind::Insert,
            MutationKind::Update,
            MutationKind::Delete,
            MutationKind::Upsert,
            MutationKind::Batch,
        ] {
            if !mutation_kinds.contains(&required_kind) {
                violations.push(
                    StagedRowMutationQualificationViolation::MissingMutationKind {
                        mutation_kind: required_kind,
                    },
                );
            }
        }

        for row in &self.staged_row_mutation_sheets {
            if row.target_table_ref.is_empty()
                || row.auth_scope_ref.is_empty()
                || !row.previewable_before_apply
            {
                violations.push(
                    StagedRowMutationQualificationViolation::IncompleteStagedRowMutationSheet {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
            if row.confirmation_required && !row.rollback_path_visible {
                violations.push(
                    StagedRowMutationQualificationViolation::MutationSheetMissingRollbackPath {
                        sheet_id: row.sheet_id.clone(),
                    },
                );
            }
        }

        let conflict_classes: BTreeSet<_> = self
            .optimistic_concurrency_cues
            .iter()
            .map(|row| row.conflict_class)
            .collect();
        for required_class in [
            ConcurrencyConflictClass::NoConflict,
            ConcurrencyConflictClass::StaleRead,
            ConcurrencyConflictClass::VersionMismatch,
            ConcurrencyConflictClass::ConcurrentWrite,
            ConcurrencyConflictClass::PolicyBlocked,
        ] {
            if !conflict_classes.contains(&required_class) {
                violations.push(
                    StagedRowMutationQualificationViolation::MissingConcurrencyConflictClass {
                        conflict_class: required_class,
                    },
                );
            }
        }

        for row in &self.optimistic_concurrency_cues {
            if row.version_ref.is_empty()
                || row.last_modified_ref.is_empty()
                || row.target_table_ref.is_empty()
            {
                violations.push(
                    StagedRowMutationQualificationViolation::IncompleteOptimisticConcurrencyCue {
                        cue_id: row.cue_id.clone(),
                    },
                );
            }
        }

        let rollback_scopes: BTreeSet<_> = self
            .rollback_actions
            .iter()
            .map(|row| row.rollback_scope)
            .collect();
        for required_scope in [
            RollbackScope::SingleRow,
            RollbackScope::SheetLevel,
            RollbackScope::SessionLevel,
            RollbackScope::ToCheckpoint,
        ] {
            if !rollback_scopes.contains(&required_scope) {
                violations.push(
                    StagedRowMutationQualificationViolation::MissingRollbackScope {
                        rollback_scope: required_scope,
                    },
                );
            }
        }

        for row in &self.rollback_actions {
            if row.affected_sheet_ref.is_empty() {
                violations.push(
                    StagedRowMutationQualificationViolation::IncompleteRollbackAction {
                        action_id: row.action_id.clone(),
                    },
                );
            }
            if row.rollback_scope == RollbackScope::ToCheckpoint && row.checkpoint_ref.is_none() {
                violations.push(
                    StagedRowMutationQualificationViolation::RollbackToCheckpointMissingRef {
                        action_id: row.action_id.clone(),
                    },
                );
            }
        }

        let checkpoint_scopes: BTreeSet<_> = self
            .checkpoint_actions
            .iter()
            .map(|row| row.checkpoint_scope)
            .collect();
        for required_scope in [
            CheckpointScope::PreMutation,
            CheckpointScope::PreCommit,
            CheckpointScope::PostCommit,
            CheckpointScope::Automatic,
        ] {
            if !checkpoint_scopes.contains(&required_scope) {
                violations.push(
                    StagedRowMutationQualificationViolation::MissingCheckpointScope {
                        checkpoint_scope: required_scope,
                    },
                );
            }
        }

        for row in &self.checkpoint_actions {
            if row.session_ref.is_empty() || row.label.is_empty() {
                violations.push(
                    StagedRowMutationQualificationViolation::IncompleteCheckpointAction {
                        action_id: row.action_id.clone(),
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(StagedRowMutationQualificationViolation::SummaryMismatch);
        }

        violations
    }
}

/// Loads the checked-in staged row-mutation qualification packet.
///
/// # Errors
///
/// Returns the underlying JSON parse error when the embedded artifact no longer
/// matches the typed model.
pub fn current_staged_row_mutation_qualification(
) -> Result<StagedRowMutationQualificationPacket, serde_json::Error> {
    serde_json::from_str(STAGED_ROW_MUTATION_QUALIFICATION_PACKET_JSON)
}

/// Identity family used when reporting duplicate ids.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StagedRowMutationQualificationViolationKind {
    /// Surface rows.
    Surface,
    /// Staged row-mutation sheet rows.
    StagedRowMutationSheet,
    /// Optimistic-concurrency cue rows.
    OptimisticConcurrencyCue,
    /// Rollback action rows.
    RollbackAction,
    /// Checkpoint action rows.
    CheckpointAction,
}

fn collect_ids<'a>(
    ids: impl Iterator<Item = &'a str>,
    violations: &mut Vec<StagedRowMutationQualificationViolation>,
    kind: StagedRowMutationQualificationViolationKind,
) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for id in ids {
        if !out.insert(id.to_owned()) {
            violations.push(StagedRowMutationQualificationViolation::DuplicateId {
                kind,
                id: id.to_owned(),
            });
        }
    }
    out
}

/// Validation failure for staged row-mutation qualification packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StagedRowMutationQualificationViolation {
    /// Schema version does not match the model.
    SchemaVersion { expected: u32, actual: u32 },
    /// Record kind does not match the model.
    RecordKind { expected: String, actual: String },
    /// IDs must be unique inside an object family.
    DuplicateId {
        kind: StagedRowMutationQualificationViolationKind,
        id: String,
    },
    /// Stable row has no proof packet.
    StableSurfaceMissingProof { surface_id: String },
    /// Stable row is missing one or more visible guards.
    StableSurfaceMissingGuard { surface_id: String },
    /// Narrowed stable claim lacks an explicit downgrade rule.
    NarrowedSurfaceLacksDowngradeRule { surface_id: String },
    /// Required mutation kind is missing.
    MissingMutationKind { mutation_kind: MutationKind },
    /// Staged row-mutation sheet is incomplete.
    IncompleteStagedRowMutationSheet { sheet_id: String },
    /// Staged row-mutation sheet requires confirmation but omits rollback path.
    MutationSheetMissingRollbackPath { sheet_id: String },
    /// Required concurrency conflict class is missing.
    MissingConcurrencyConflictClass {
        conflict_class: ConcurrencyConflictClass,
    },
    /// Optimistic-concurrency cue is incomplete.
    IncompleteOptimisticConcurrencyCue { cue_id: String },
    /// Required rollback scope is missing.
    MissingRollbackScope { rollback_scope: RollbackScope },
    /// Rollback action is incomplete.
    IncompleteRollbackAction { action_id: String },
    /// Rollback-to-checkpoint action lacks a checkpoint ref.
    RollbackToCheckpointMissingRef { action_id: String },
    /// Required checkpoint scope is missing.
    MissingCheckpointScope { checkpoint_scope: CheckpointScope },
    /// Checkpoint action is incomplete.
    IncompleteCheckpointAction { action_id: String },
    /// Stored summary no longer matches row state.
    SummaryMismatch,
}

impl fmt::Display for StagedRowMutationQualificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(f, "schema_version expected {expected}, got {actual}")
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record_kind expected {expected}, got {actual}")
            }
            Self::DuplicateId { kind, id } => write!(f, "{kind:?} id {id} is duplicated"),
            Self::StableSurfaceMissingProof { surface_id } => {
                write!(f, "{surface_id} is stable without a proof packet")
            }
            Self::StableSurfaceMissingGuard { surface_id } => {
                write!(f, "{surface_id} is stable without complete guard truth")
            }
            Self::NarrowedSurfaceLacksDowngradeRule { surface_id } => {
                write!(f, "{surface_id} is narrowed without a downgrade rule")
            }
            Self::MissingMutationKind { mutation_kind } => {
                write!(f, "mutation kind {mutation_kind:?} is not covered")
            }
            Self::IncompleteStagedRowMutationSheet { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} does not project staged row-mutation sheet truth everywhere"
                )
            }
            Self::MutationSheetMissingRollbackPath { sheet_id } => {
                write!(
                    f,
                    "{sheet_id} requires confirmation but omits rollback path visibility"
                )
            }
            Self::MissingConcurrencyConflictClass { conflict_class } => {
                write!(
                    f,
                    "concurrency conflict class {conflict_class:?} is not covered"
                )
            }
            Self::IncompleteOptimisticConcurrencyCue { cue_id } => {
                write!(
                    f,
                    "{cue_id} does not project optimistic-concurrency cue truth everywhere"
                )
            }
            Self::MissingRollbackScope { rollback_scope } => {
                write!(f, "rollback scope {rollback_scope:?} is not covered")
            }
            Self::IncompleteRollbackAction { action_id } => {
                write!(
                    f,
                    "{action_id} does not project rollback action truth everywhere"
                )
            }
            Self::RollbackToCheckpointMissingRef { action_id } => {
                write!(
                    f,
                    "{action_id} is rollback_to_checkpoint without a checkpoint ref"
                )
            }
            Self::MissingCheckpointScope { checkpoint_scope } => {
                write!(f, "checkpoint scope {checkpoint_scope:?} is not covered")
            }
            Self::IncompleteCheckpointAction { action_id } => {
                write!(
                    f,
                    "{action_id} does not project checkpoint action truth everywhere"
                )
            }
            Self::SummaryMismatch => write!(f, "summary does not match row state"),
        }
    }
}

impl Error for StagedRowMutationQualificationViolation {}
