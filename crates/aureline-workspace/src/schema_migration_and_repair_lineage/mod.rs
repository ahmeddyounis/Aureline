//! Schema-migration and repair lineage for workspace, profile, and
//! persistent-state artifacts: the governed, export-safe projection
//! that proves how schema migrations and repair flows preserve source
//! fidelity, restore provenance, encoding/newline class, trust state,
//! lineage, and no-rerun semantics across crashes, schema upgrades,
//! corrupt artifacts, and policy-bound workflows.
//!
//! Where the portable-state lineage proves how restored state
//! preserves provenance and the local-history export/replay lineage
//! proves how packets and replays preserve fidelity, this projection
//! proves the *schema-migration and repair layer* underneath both:
//! which artifact classes Aureline migrates and repairs, which
//! migration outcomes and repair flows are governed, which
//! repair-transaction ids and finding codes survive into support
//! evidence, and which inspection / repair hooks must fire before any
//! destructive migration or repair commits.
//!
//! The projection ingests a live [`SchemaMigrationAndRepairInputs`]
//! envelope verbatim (one [`MigrationObservation`] per artifact, one
//! [`RepairFlowObservation`] per governed repair flow, plus the
//! controlled inspection-hook table) and produces a lineage record
//! that proves the contract claims the stable line is anchored on:
//!
//! - **Artifact-class coverage truth.** Every governed artifact class
//!   (`workspace_state_artifact`, `profile_artifact`,
//!   `recent_work_registry`, `local_history_corpus`,
//!   `restore_checkpoint`, `persistent_state_envelope`) ships a row
//!   bound to one closed [`ArtifactClassKind`]; the optional
//!   `prebuild_cache_artifact` and `mutation_journal_artifact` rides
//!   ride on top without changing the required set.
//! - **Schema-version pinning truth.** Every migration row pins a
//!   non-zero `from_schema_version` and `to_schema_version`, and the
//!   to-version is at least the from-version so a downgrade or an
//!   unpinned version cannot ride on Stable.
//! - **Repair-flow coverage truth.** Every required repair flow
//!   (`inspect_repair`, `rebuild_derived_store`, `rehydrate_from_packet`,
//!   `quarantine_corrupt_artifact`, `restore_from_checkpoint`,
//!   `manual_repair_handoff`) ships a row bound to one closed
//!   [`RepairFlowKind`].
//! - **Migration-outcome honesty.** Every migration row declares one
//!   closed [`MigrationOutcome`] and a `lossy` migration or a refused
//!   migration carries a non-empty disclosure id. Silent unsafe
//!   migration narrows below Stable.
//! - **Repair-outcome honesty.** Every repair flow row declares one
//!   closed [`RepairOutcome`]; a `lossy` repair or refused repair
//!   carries a non-empty disclosure id. Silent unsafe repair narrows
//!   below Stable.
//! - **Restore-provenance / encoding / trust-state preservation.**
//!   Every migration and repair row preserves the restore provenance,
//!   encoding fidelity, and trust state; any deviation narrows the
//!   record below Stable.
//! - **No-silent-rerun honesty.** Every migration and repair row
//!   declares `explicit_user_action_required` or
//!   `terminal_no_further_run` with both a commit action id and a
//!   commit disclosure id whenever the row mutates persistent state;
//!   a `silent_rerun_permitted` posture is forbidden on Stable rows.
//! - **Repair-transaction-id / finding-code pinning.** Every
//!   migration and repair row pins a non-empty repair_transaction_id
//!   and a non-empty finding_code so support, docs, and shiproom
//!   packets all reference the same truth.
//! - **Pre-action inspection-hook honesty.** A controlled set of
//!   pre-action inspection / repair hooks (`inspect_artifact`,
//!   `compare_before_migration`, `preview_migration`,
//!   `preview_repair`, `rollback_migration`, `rollback_repair`,
//!   `export_before_migration`, `export_before_repair`) is reachable
//!   before any destructive migration / repair commits.
//! - **Support-export honesty.** Each row's support-export projection
//!   preserves the artifact class, migration outcome / repair flow
//!   class, schema versions, repair_transaction_id, finding_code, and
//!   redaction class while excluding raw secrets, raw artifact bytes,
//!   approval tickets, delegated credentials, and live authority
//!   handles.
//! - **Producer attribution.** Each record carries the producer ref,
//!   the schema version, the capture timestamp, and an integrity
//!   hash derived from the input identities so replay and support
//!   pipelines can pin the source before applying.
//! - **Lineage and export honesty.** The record sets
//!   `raw_payload_excluded = true` and carries only opaque refs to
//!   the source workspace, corpus, and producer.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Schema version for [`SchemaMigrationAndRepairLineageRecord`].
pub const SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the schema-migration and repair lineage record.
pub const SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_SCHEMA_REF: &str =
    "schemas/workspace/schema_migration_and_repair_lineage.schema.json";

/// Stable record-kind tag for the schema-migration and repair lineage
/// record.
pub const SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_RECORD_KIND: &str =
    "schema_migration_and_repair_lineage_record";

// ---------------------------------------------------------------------------
// Closed vocabularies.
// ---------------------------------------------------------------------------

/// Closed vocabulary for the artifact classes Aureline migrates and
/// repairs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClassKind {
    /// Durable workspace state — layout, panes, sessions.
    WorkspaceStateArtifact,
    /// Portable user profile (keymaps, settings, presets).
    ProfileArtifact,
    /// Persistent recent-work / entry-restore registry.
    RecentWorkRegistry,
    /// Local-history entry / group corpus.
    LocalHistoryCorpus,
    /// Named restore checkpoint snapshot.
    RestoreCheckpoint,
    /// Top-level persistent-state envelope binding the layers above.
    PersistentStateEnvelope,
    /// Optional prebuild / build-artifact cache row.
    PrebuildCacheArtifact,
    /// Optional mutation-journal artifact row.
    MutationJournalArtifact,
}

impl ArtifactClassKind {
    /// Returns the stable snake_case token for this artifact class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceStateArtifact => "workspace_state_artifact",
            Self::ProfileArtifact => "profile_artifact",
            Self::RecentWorkRegistry => "recent_work_registry",
            Self::LocalHistoryCorpus => "local_history_corpus",
            Self::RestoreCheckpoint => "restore_checkpoint",
            Self::PersistentStateEnvelope => "persistent_state_envelope",
            Self::PrebuildCacheArtifact => "prebuild_cache_artifact",
            Self::MutationJournalArtifact => "mutation_journal_artifact",
        }
    }

    /// True when this artifact class is part of the required set every
    /// Stable corpus must seed.
    pub const fn is_required(self) -> bool {
        matches!(
            self,
            Self::WorkspaceStateArtifact
                | Self::ProfileArtifact
                | Self::RecentWorkRegistry
                | Self::LocalHistoryCorpus
                | Self::RestoreCheckpoint
                | Self::PersistentStateEnvelope
        )
    }
}

/// Closed list of artifact classes every lineage record must seed.
pub const REQUIRED_ARTIFACT_CLASSES: [ArtifactClassKind; 6] = [
    ArtifactClassKind::WorkspaceStateArtifact,
    ArtifactClassKind::ProfileArtifact,
    ArtifactClassKind::RecentWorkRegistry,
    ArtifactClassKind::LocalHistoryCorpus,
    ArtifactClassKind::RestoreCheckpoint,
    ArtifactClassKind::PersistentStateEnvelope,
];

/// Closed vocabulary for the schema-compatibility class an artifact
/// reports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaCompatibilityClass {
    /// Already on the current schema version; no migration needed.
    CurrentSchema,
    /// On an older schema with a supported forward migration path.
    OlderSupportedSchema,
    /// On an older schema with no supported forward migration; repair
    /// is required.
    OlderUnsupportedRequiresRepair,
    /// On a newer schema version this runtime does not understand;
    /// refused.
    NewerUnknownSchemaRefused,
    /// The artifact is corrupt and cannot be classified safely.
    CorruptArtifact,
}

impl SchemaCompatibilityClass {
    /// Returns the stable snake_case token for this compatibility class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentSchema => "current_schema",
            Self::OlderSupportedSchema => "older_supported_schema",
            Self::OlderUnsupportedRequiresRepair => "older_unsupported_requires_repair",
            Self::NewerUnknownSchemaRefused => "newer_unknown_schema_refused",
            Self::CorruptArtifact => "corrupt_artifact",
        }
    }
}

/// Closed vocabulary for the outcome of a schema-migration attempt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationOutcome {
    /// Artifact already on the current schema; no migration applied.
    NoMigrationNeeded,
    /// Forward migration applied with no observable user-state loss.
    ForwardMigratedLossless,
    /// Forward migration applied but observable user-state changes
    /// shipped behind an explicit override disclosure.
    ForwardMigratedLossyWithDisclosure,
    /// Migration refused because it would not be safe; held for
    /// repair.
    MigrationRefusedUnsafe,
    /// Migration paused awaiting explicit user review.
    MigrationAwaitingUserReview,
    /// Compatibility-only report (no migration applied), e.g. for a
    /// newer-than-runtime schema.
    CompatReportOnly,
}

impl MigrationOutcome {
    /// Returns the stable snake_case token for this migration outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoMigrationNeeded => "no_migration_needed",
            Self::ForwardMigratedLossless => "forward_migrated_lossless",
            Self::ForwardMigratedLossyWithDisclosure => "forward_migrated_lossy_with_disclosure",
            Self::MigrationRefusedUnsafe => "migration_refused_unsafe",
            Self::MigrationAwaitingUserReview => "migration_awaiting_user_review",
            Self::CompatReportOnly => "compat_report_only",
        }
    }

    /// True when this outcome requires a non-empty disclosure ref.
    pub const fn requires_disclosure(self) -> bool {
        matches!(
            self,
            Self::ForwardMigratedLossyWithDisclosure
                | Self::MigrationRefusedUnsafe
                | Self::MigrationAwaitingUserReview
                | Self::CompatReportOnly
        )
    }

    /// True when this outcome mutates persistent state (and therefore
    /// must declare an explicit no-silent-rerun posture with both a
    /// commit action id and a commit disclosure id).
    pub const fn mutates_state(self) -> bool {
        matches!(
            self,
            Self::ForwardMigratedLossless | Self::ForwardMigratedLossyWithDisclosure
        )
    }
}

/// Closed vocabulary for the governed repair flow kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairFlowKind {
    /// Open the typed repair-inspector for an artifact (read-only).
    InspectRepair,
    /// Rebuild a derived store (search/symbol index, derived layout
    /// cache) from authoritative sources.
    RebuildDerivedStore,
    /// Rehydrate the artifact from a checked-in packet (local history,
    /// support-bundle replay).
    RehydrateFromPacket,
    /// Quarantine a corrupt artifact so it cannot ship into the next
    /// session.
    QuarantineCorruptArtifact,
    /// Restore the artifact from a named restore checkpoint.
    RestoreFromCheckpoint,
    /// Hand off to a typed manual repair flow (out-of-band repair
    /// console / support ticket).
    ManualRepairHandoff,
}

impl RepairFlowKind {
    /// Returns the stable snake_case token for this repair flow kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectRepair => "inspect_repair",
            Self::RebuildDerivedStore => "rebuild_derived_store",
            Self::RehydrateFromPacket => "rehydrate_from_packet",
            Self::QuarantineCorruptArtifact => "quarantine_corrupt_artifact",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
            Self::ManualRepairHandoff => "manual_repair_handoff",
        }
    }

    /// True when this repair flow mutates persistent state (and
    /// therefore must declare an explicit no-silent-rerun posture with
    /// both a commit action id and a commit disclosure id).
    pub const fn mutates_state(self) -> bool {
        matches!(
            self,
            Self::RebuildDerivedStore
                | Self::RehydrateFromPacket
                | Self::QuarantineCorruptArtifact
                | Self::RestoreFromCheckpoint
        )
    }
}

/// Closed list of repair-flow kinds every lineage record must seed.
pub const REQUIRED_REPAIR_FLOW_KINDS: [RepairFlowKind; 6] = [
    RepairFlowKind::InspectRepair,
    RepairFlowKind::RebuildDerivedStore,
    RepairFlowKind::RehydrateFromPacket,
    RepairFlowKind::QuarantineCorruptArtifact,
    RepairFlowKind::RestoreFromCheckpoint,
    RepairFlowKind::ManualRepairHandoff,
];

/// Closed vocabulary for the outcome of a repair flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairOutcome {
    /// Repair succeeded with no observable user-state loss.
    RepairSucceededLossless,
    /// Repair succeeded but observable user-state changes shipped
    /// behind an explicit override disclosure.
    RepairSucceededLossyWithDisclosure,
    /// Repair refused because it would not be safe; held.
    RepairRefusedUnsafe,
    /// Repair paused awaiting explicit user action.
    RepairAwaitingUserAction,
    /// Repair quarantined the artifact rather than mutating it.
    RepairQuarantined,
}

impl RepairOutcome {
    /// Returns the stable snake_case token for this repair outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RepairSucceededLossless => "repair_succeeded_lossless",
            Self::RepairSucceededLossyWithDisclosure => "repair_succeeded_lossy_with_disclosure",
            Self::RepairRefusedUnsafe => "repair_refused_unsafe",
            Self::RepairAwaitingUserAction => "repair_awaiting_user_action",
            Self::RepairQuarantined => "repair_quarantined",
        }
    }

    /// True when this outcome requires a non-empty disclosure ref.
    pub const fn requires_disclosure(self) -> bool {
        matches!(
            self,
            Self::RepairSucceededLossyWithDisclosure
                | Self::RepairRefusedUnsafe
                | Self::RepairAwaitingUserAction
                | Self::RepairQuarantined
        )
    }
}

/// Closed vocabulary for the no-rerun posture a migration or repair
/// declares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RerunPosture {
    /// Requires an explicit user commit action before re-running.
    ExplicitUserActionRequired,
    /// Terminal: does not re-fire after the captured run.
    TerminalNoFurtherRun,
    /// May re-fire silently — forbidden on Stable.
    SilentRerunPermitted,
}

impl RerunPosture {
    /// Returns the stable snake_case token for this rerun posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitUserActionRequired => "explicit_user_action_required",
            Self::TerminalNoFurtherRun => "terminal_no_further_run",
            Self::SilentRerunPermitted => "silent_rerun_permitted",
        }
    }

    /// True when this posture is safe to ship on a Stable row.
    pub const fn safe_for_stable(self) -> bool {
        matches!(
            self,
            Self::ExplicitUserActionRequired | Self::TerminalNoFurtherRun
        )
    }
}

/// Closed redaction class for migration / repair rows shipped into
/// support evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe row; no body bytes, no payload refs.
    MetadataOnly,
    /// Body shipped behind an explicit override-disclosure ref.
    RedactedWithDisclosure,
    /// Body excluded by policy (trust / license / export control).
    ExcludedByPolicy,
}

impl RedactionClass {
    /// Returns the stable snake_case token for this redaction class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::RedactedWithDisclosure => "redacted_with_disclosure",
            Self::ExcludedByPolicy => "excluded_by_policy",
        }
    }

    /// True when this class requires an explicit override-disclosure
    /// ref before being shipped.
    pub const fn requires_override_disclosure(self) -> bool {
        matches!(self, Self::RedactedWithDisclosure)
    }
}

/// Closed vocabulary for pre-action inspection / repair hooks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaMigrationInspectionHookClass {
    /// Open the artifact inspector with the artifact's class, schema
    /// version, repair_transaction_id, and finding_code.
    InspectArtifact,
    /// Compare the in-place artifact to the would-be migrated artifact
    /// before any destructive migration commits.
    CompareBeforeMigration,
    /// Preview the migration's effects (schema delta, user-state
    /// changes, restore-provenance changes) before the apply.
    PreviewMigration,
    /// Preview the repair flow's effects before the apply.
    PreviewRepair,
    /// Roll a destructive migration back to the pre-migration artifact
    /// identity.
    RollbackMigration,
    /// Roll a destructive repair back to the pre-repair artifact
    /// identity.
    RollbackRepair,
    /// Export the artifact (support-safe) before any destructive
    /// migration commits.
    ExportBeforeMigration,
    /// Export the artifact (support-safe) before any destructive
    /// repair commits.
    ExportBeforeRepair,
}

impl SchemaMigrationInspectionHookClass {
    /// Returns the stable snake_case token for this hook class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectArtifact => "inspect_artifact",
            Self::CompareBeforeMigration => "compare_before_migration",
            Self::PreviewMigration => "preview_migration",
            Self::PreviewRepair => "preview_repair",
            Self::RollbackMigration => "rollback_migration",
            Self::RollbackRepair => "rollback_repair",
            Self::ExportBeforeMigration => "export_before_migration",
            Self::ExportBeforeRepair => "export_before_repair",
        }
    }
}

/// One pre-action inspection / repair hook offered before a
/// destructive migration / repair commits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigrationInspectionHook {
    /// Hook class.
    pub hook_class: SchemaMigrationInspectionHookClass,
    /// Stable action id.
    pub action_id: String,
    /// UI label.
    pub label: String,
    /// Whether the hook is reachable on this posture.
    pub available: bool,
    /// Disclosure shown when the hook is offered.
    pub disclosure: String,
}

/// Returns the default pre-action inspection / repair hook table.
pub fn default_schema_migration_inspection_hooks() -> Vec<SchemaMigrationInspectionHook> {
    vec![
        SchemaMigrationInspectionHook {
            hook_class: SchemaMigrationInspectionHookClass::InspectArtifact,
            action_id: "schema_migration_and_repair.inspect_artifact".to_owned(),
            label: "Inspect artifact".to_owned(),
            available: true,
            disclosure:
                "Opens the artifact inspector with the artifact's class, schema version, repair_transaction_id, finding_code, and restore provenance before any migration or repair commits."
                    .to_owned(),
        },
        SchemaMigrationInspectionHook {
            hook_class: SchemaMigrationInspectionHookClass::CompareBeforeMigration,
            action_id: "schema_migration_and_repair.compare_before_migration".to_owned(),
            label: "Compare before migration".to_owned(),
            available: true,
            disclosure:
                "Renders the typed compare view between the in-place artifact and the would-be migrated artifact so the user can review schema deltas before any migration applies."
                    .to_owned(),
        },
        SchemaMigrationInspectionHook {
            hook_class: SchemaMigrationInspectionHookClass::PreviewMigration,
            action_id: "schema_migration_and_repair.preview_migration".to_owned(),
            label: "Preview migration".to_owned(),
            available: true,
            disclosure:
                "Previews the schema delta, user-state changes, and restore-provenance changes the migration will land before any apply commits."
                    .to_owned(),
        },
        SchemaMigrationInspectionHook {
            hook_class: SchemaMigrationInspectionHookClass::PreviewRepair,
            action_id: "schema_migration_and_repair.preview_repair".to_owned(),
            label: "Preview repair".to_owned(),
            available: true,
            disclosure:
                "Previews the repair flow's effects (bytes regenerated, state quarantined, restore-provenance changes) before any apply commits."
                    .to_owned(),
        },
        SchemaMigrationInspectionHook {
            hook_class: SchemaMigrationInspectionHookClass::RollbackMigration,
            action_id: "schema_migration_and_repair.rollback_migration".to_owned(),
            label: "Roll back migration".to_owned(),
            available: true,
            disclosure:
                "Reverts the most recent destructive migration back to the pre-migration artifact identity, preserving restore provenance, encoding, and trust state."
                    .to_owned(),
        },
        SchemaMigrationInspectionHook {
            hook_class: SchemaMigrationInspectionHookClass::RollbackRepair,
            action_id: "schema_migration_and_repair.rollback_repair".to_owned(),
            label: "Roll back repair".to_owned(),
            available: true,
            disclosure:
                "Reverts the most recent destructive repair back to the pre-repair artifact identity, preserving restore provenance, encoding, and trust state."
                    .to_owned(),
        },
        SchemaMigrationInspectionHook {
            hook_class: SchemaMigrationInspectionHookClass::ExportBeforeMigration,
            action_id: "schema_migration_and_repair.export_before_migration".to_owned(),
            label: "Export before migration".to_owned(),
            available: true,
            disclosure:
                "Exports the in-place artifact (support-safe) before any destructive migration commits so the user can replay or audit elsewhere."
                    .to_owned(),
        },
        SchemaMigrationInspectionHook {
            hook_class: SchemaMigrationInspectionHookClass::ExportBeforeRepair,
            action_id: "schema_migration_and_repair.export_before_repair".to_owned(),
            label: "Export before repair".to_owned(),
            available: true,
            disclosure:
                "Exports the in-place artifact (support-safe) before any destructive repair commits so the user can replay or audit elsewhere."
                    .to_owned(),
        },
    ]
}

// ---------------------------------------------------------------------------
// Input envelope.
// ---------------------------------------------------------------------------

/// Metadata-safe support-export projection input for a migration or
/// repair row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigrationSupportExportInputs {
    /// Whether the row ships a metadata-safe export or holds for
    /// manual review.
    pub posture: SchemaMigrationSupportExportPosture,
    pub includes_artifact_class: bool,
    pub includes_migration_outcome_or_repair_flow: bool,
    pub includes_schema_versions: bool,
    pub includes_repair_transaction_id: bool,
    pub includes_finding_code: bool,
    pub includes_redaction_class: bool,
    pub raw_secrets_excluded: bool,
    pub raw_artifact_bytes_excluded: bool,
    pub approval_tickets_excluded: bool,
    pub delegated_credentials_excluded: bool,
    pub live_authority_handles_excluded: bool,
}

impl SchemaMigrationSupportExportInputs {
    /// Returns the metadata-safe baseline for a given posture.
    pub const fn metadata_safe_baseline(posture: SchemaMigrationSupportExportPosture) -> Self {
        Self {
            posture,
            includes_artifact_class: true,
            includes_migration_outcome_or_repair_flow: true,
            includes_schema_versions: true,
            includes_repair_transaction_id: true,
            includes_finding_code: true,
            includes_redaction_class: true,
            raw_secrets_excluded: true,
            raw_artifact_bytes_excluded: true,
            approval_tickets_excluded: true,
            delegated_credentials_excluded: true,
            live_authority_handles_excluded: true,
        }
    }
}

/// Closed support-export posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaMigrationSupportExportPosture {
    /// Row ships a metadata-safe projection in the support packet.
    MetadataSafeExport,
    /// Row withholds its state until manual review.
    HeldRecord,
}

impl SchemaMigrationSupportExportPosture {
    /// Returns the stable snake_case token for this posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeExport => "metadata_safe_export",
            Self::HeldRecord => "held_record",
        }
    }
}

/// One observation of an artifact migration captured at a moment in
/// time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationObservation {
    /// Stable artifact identity every consumer preserves.
    pub artifact_id: String,
    /// Closed artifact class.
    pub artifact_class: ArtifactClassKind,
    /// Opaque artifact ref (e.g. `art:workspace_state:abc`).
    pub artifact_ref: String,
    /// Schema-compatibility class reported for the artifact.
    pub schema_compatibility_class: SchemaCompatibilityClass,
    /// Pinned source schema version (must be non-zero on Stable).
    pub from_schema_version: u32,
    /// Pinned target schema version (must be >= from_schema_version on
    /// Stable).
    pub to_schema_version: u32,
    /// Closed migration outcome.
    pub migration_outcome: MigrationOutcome,
    /// Optional override-disclosure ref (required for outcomes that
    /// declare lossy or refused state).
    pub migration_disclosure_ref: Option<String>,
    /// Stable repair-transaction id pinned for this migration.
    pub repair_transaction_id: String,
    /// Stable finding code (e.g. `WS-MIG-0001`).
    pub finding_code: String,
    /// True when the migration preserves the restore provenance
    /// (restore_of_ref + mutation_journal_ref + actor_class) on the
    /// migrated artifact.
    pub preserves_restore_provenance: bool,
    /// True when the migration preserves the source encoding/newline
    /// class.
    pub preserves_encoding_fidelity: bool,
    /// True when the migration preserves the workspace trust state.
    pub preserves_trust_state: bool,
    /// True when the migration preserves the no-rerun semantics of
    /// any captured remembered actions.
    pub preserves_no_rerun_semantics: bool,
    /// Closed no-rerun posture for this migration.
    pub rerun_posture: RerunPosture,
    /// Stable id of the commit action that gates this migration
    /// (required when the outcome mutates persistent state).
    pub commit_action_id: String,
    /// Stable id of the disclosure paired with the commit action
    /// (required when the outcome mutates persistent state).
    pub commit_disclosure_id: String,
    /// Closed redaction class for support evidence.
    pub redaction_class: RedactionClass,
    /// Optional redaction-disclosure ref (required when the redaction
    /// class requires one).
    pub redaction_disclosure_ref: Option<String>,
    /// Support-export projection for the migration row.
    pub support_export: SchemaMigrationSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// One observation of a repair flow captured at a moment in time.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairFlowObservation {
    /// Stable repair-flow id.
    pub repair_flow_id: String,
    /// Human-readable label.
    pub label: String,
    /// Closed repair-flow kind.
    pub repair_flow_kind: RepairFlowKind,
    /// The stable artifact id the repair flow targets.
    pub artifact_id: String,
    /// Closed repair outcome.
    pub repair_outcome: RepairOutcome,
    /// Optional override-disclosure ref (required for outcomes that
    /// declare lossy or refused state).
    pub repair_disclosure_ref: Option<String>,
    /// Stable repair-transaction id pinned for this repair.
    pub repair_transaction_id: String,
    /// Stable finding code (e.g. `WS-REP-0001`).
    pub finding_code: String,
    /// True when the repair preserves the restore provenance.
    pub preserves_restore_provenance: bool,
    /// True when the repair preserves the source encoding/newline
    /// class.
    pub preserves_encoding_fidelity: bool,
    /// True when the repair preserves the workspace trust state.
    pub preserves_trust_state: bool,
    /// True when the repair preserves the no-rerun semantics of any
    /// captured remembered actions.
    pub preserves_no_rerun_semantics: bool,
    /// Closed no-rerun posture for this repair.
    pub rerun_posture: RerunPosture,
    /// Stable id of the commit action that gates this repair
    /// (required when the flow mutates persistent state).
    pub commit_action_id: String,
    /// Stable id of the disclosure paired with the commit action
    /// (required when the flow mutates persistent state).
    pub commit_disclosure_id: String,
    /// Closed redaction class for support evidence.
    pub redaction_class: RedactionClass,
    /// Optional redaction-disclosure ref (required when the redaction
    /// class requires one).
    pub redaction_disclosure_ref: Option<String>,
    /// Support-export projection for the repair row.
    pub support_export: SchemaMigrationSupportExportInputs,
    /// Capture timestamp.
    pub captured_at: String,
}

/// Input envelope ingested by the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigrationAndRepairInputs {
    /// Opaque workspace ref the corpus describes.
    pub workspace_ref: String,
    /// Opaque producer ref.
    pub producer_ref: String,
    /// Opaque corpus ref.
    pub corpus_ref: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Captured artifact-migration observations.
    pub migrations: Vec<MigrationObservation>,
    /// Captured repair-flow observations.
    pub repair_flows: Vec<RepairFlowObservation>,
}

// ---------------------------------------------------------------------------
// Narrow reasons + qualification.
// ---------------------------------------------------------------------------

/// Named reason a schema-migration and repair lineage record narrows
/// below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaMigrationAndRepairLineageNarrowReason {
    /// The captured input had no migrations or no repair flows.
    CorpusEmpty,
    /// A required artifact class is missing from the corpus.
    RequiredArtifactClassMissing,
    /// A required repair flow kind is missing from the corpus.
    RequiredRepairFlowKindMissing,
    /// A repair flow references an artifact id not present in the
    /// migration corpus.
    RepairReferencesUnknownArtifact,
    /// A migration row pins a zero `from_schema_version` or
    /// `to_schema_version`, or a `to_schema_version` lower than the
    /// `from_schema_version`.
    SchemaVersionUnpinned,
    /// A migration declares a lossy or refused outcome without a
    /// migration disclosure ref.
    MigrationDisclosureMissing,
    /// A repair declares a lossy or refused outcome without a repair
    /// disclosure ref.
    RepairDisclosureMissing,
    /// A migration or repair row ships a redaction class that requires
    /// an override-disclosure ref but no disclosure ref is present.
    RedactionDisclosureMissing,
    /// A migration or repair row declares `silent_rerun_permitted`
    /// (forbidden on Stable rows).
    RerunSilentForbidden,
    /// A state-mutating migration or repair is missing its commit
    /// action id or commit disclosure id.
    CommitActionMetadataMissing,
    /// A migration or repair row does not preserve restore provenance.
    RestoreProvenanceNotPreserved,
    /// A migration or repair row does not preserve encoding fidelity.
    EncodingFidelityNotPreserved,
    /// A migration or repair row does not preserve trust state.
    TrustStateNotPreserved,
    /// A migration or repair row does not preserve no-rerun semantics.
    NoRerunSemanticsNotPreserved,
    /// A migration or repair row ships without a repair-transaction id.
    RepairTransactionIdNotPinned,
    /// A migration or repair row ships without a finding code.
    FindingCodeMissing,
    /// A required pre-action inspection / repair hook is unavailable.
    InspectionHookUnavailable,
    /// A support-export projection drops a required field.
    SupportExportFieldsDropped,
    /// Raw secrets, raw artifact bytes, approval tickets, delegated
    /// credentials, or live authority handles slipped into a
    /// support-export projection.
    SupportExportRedactionUnsafe,
    /// Producer attribution is incomplete (producer ref or
    /// captured-at empty).
    ProducerAttributionIncomplete,
    /// Workspace ref or corpus ref is empty (would break support
    /// export).
    LineageExportUnsafe,
}

impl SchemaMigrationAndRepairLineageNarrowReason {
    /// Returns the stable snake_case token for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CorpusEmpty => "corpus_empty",
            Self::RequiredArtifactClassMissing => "required_artifact_class_missing",
            Self::RequiredRepairFlowKindMissing => "required_repair_flow_kind_missing",
            Self::RepairReferencesUnknownArtifact => "repair_references_unknown_artifact",
            Self::SchemaVersionUnpinned => "schema_version_unpinned",
            Self::MigrationDisclosureMissing => "migration_disclosure_missing",
            Self::RepairDisclosureMissing => "repair_disclosure_missing",
            Self::RedactionDisclosureMissing => "redaction_disclosure_missing",
            Self::RerunSilentForbidden => "rerun_silent_forbidden",
            Self::CommitActionMetadataMissing => "commit_action_metadata_missing",
            Self::RestoreProvenanceNotPreserved => "restore_provenance_not_preserved",
            Self::EncodingFidelityNotPreserved => "encoding_fidelity_not_preserved",
            Self::TrustStateNotPreserved => "trust_state_not_preserved",
            Self::NoRerunSemanticsNotPreserved => "no_rerun_semantics_not_preserved",
            Self::RepairTransactionIdNotPinned => "repair_transaction_id_not_pinned",
            Self::FindingCodeMissing => "finding_code_missing",
            Self::InspectionHookUnavailable => "inspection_hook_unavailable",
            Self::SupportExportFieldsDropped => "support_export_fields_dropped",
            Self::SupportExportRedactionUnsafe => "support_export_redaction_unsafe",
            Self::ProducerAttributionIncomplete => "producer_attribution_incomplete",
            Self::LineageExportUnsafe => "lineage_export_unsafe",
        }
    }
}

/// Stable-qualification posture for a schema-migration and repair
/// lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigrationAndRepairLineageQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not
    /// qualified.
    pub narrow_reasons: Vec<SchemaMigrationAndRepairLineageNarrowReason>,
}

// ---------------------------------------------------------------------------
// Pillar projections.
// ---------------------------------------------------------------------------

/// One migration row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MigrationRow {
    pub artifact_id: String,
    pub artifact_class: ArtifactClassKind,
    pub artifact_ref: String,
    pub schema_compatibility_class: SchemaCompatibilityClass,
    pub from_schema_version: u32,
    pub to_schema_version: u32,
    pub migration_outcome: MigrationOutcome,
    pub migration_disclosure_ref: Option<String>,
    pub repair_transaction_id: String,
    pub finding_code: String,
    pub preserves_restore_provenance: bool,
    pub preserves_encoding_fidelity: bool,
    pub preserves_trust_state: bool,
    pub preserves_no_rerun_semantics: bool,
    pub rerun_posture: RerunPosture,
    pub commit_action_id: String,
    pub commit_disclosure_id: String,
    pub redaction_class: RedactionClass,
    pub redaction_disclosure_ref: Option<String>,
    pub support_export_posture: SchemaMigrationSupportExportPosture,
    pub mutates_state: bool,
    pub is_required: bool,
}

/// One repair-flow row carried in the lineage projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairFlowRow {
    pub repair_flow_id: String,
    pub label: String,
    pub repair_flow_kind: RepairFlowKind,
    pub artifact_id: String,
    pub repair_outcome: RepairOutcome,
    pub repair_disclosure_ref: Option<String>,
    pub repair_transaction_id: String,
    pub finding_code: String,
    pub preserves_restore_provenance: bool,
    pub preserves_encoding_fidelity: bool,
    pub preserves_trust_state: bool,
    pub preserves_no_rerun_semantics: bool,
    pub rerun_posture: RerunPosture,
    pub commit_action_id: String,
    pub commit_disclosure_id: String,
    pub redaction_class: RedactionClass,
    pub redaction_disclosure_ref: Option<String>,
    pub support_export_posture: SchemaMigrationSupportExportPosture,
    pub mutates_state: bool,
    pub is_required: bool,
}

/// Artifact-class coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactClassCoverageSummary {
    pub migration_rows: Vec<MigrationRow>,
    pub all_required_artifact_classes_present: bool,
}

/// Repair-flow coverage posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairFlowCoverageSummary {
    pub repair_flow_rows: Vec<RepairFlowRow>,
    pub all_required_repair_flow_kinds_present: bool,
}

/// Schema-version pinning posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaVersionPinningSummary {
    pub all_migrations_pin_from_schema: bool,
    pub all_migrations_pin_to_schema: bool,
    pub all_migrations_no_downgrade: bool,
}

/// Migration-outcome / repair-outcome honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutcomeHonestySummary {
    pub all_migration_disclosures_present: bool,
    pub all_repair_disclosures_present: bool,
    pub all_redaction_disclosures_present: bool,
}

/// Preservation posture for restore provenance, encoding, trust state,
/// and no-rerun semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservationSummary {
    pub all_rows_preserve_restore_provenance: bool,
    pub all_rows_preserve_encoding_fidelity: bool,
    pub all_rows_preserve_trust_state: bool,
    pub all_rows_preserve_no_rerun_semantics: bool,
}

/// No-silent-rerun posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoSilentRerunSummary {
    pub all_rows_safe_rerun_posture: bool,
    pub all_mutating_rows_have_commit_metadata: bool,
}

/// Repair-transaction-id / finding-code pinning posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairTransactionPinningSummary {
    pub all_rows_pin_repair_transaction_id: bool,
    pub all_rows_pin_finding_code: bool,
}

/// Support-export honesty posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigrationSupportExportHonestySummary {
    pub all_rows_preserve_fields: bool,
    pub all_rows_exclude_raw_secrets: bool,
    pub all_rows_exclude_raw_artifact_bytes: bool,
    pub all_rows_exclude_approval_tickets: bool,
    pub all_rows_exclude_delegated_credentials: bool,
    pub all_rows_exclude_live_authority_handles: bool,
}

/// Producer-attribution posture for replay safety.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigrationProducerAttributionSummary {
    pub producer_ref: String,
    pub schema_version: u32,
    pub integrity_hash: String,
    pub captured_at: String,
    pub producer_attribution_complete: bool,
}

// ---------------------------------------------------------------------------
// Top-level record.
// ---------------------------------------------------------------------------

/// Governed, export-safe schema-migration and repair lineage record
/// per posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SchemaMigrationAndRepairLineageRecord {
    pub record_kind: String,
    pub schema_migration_and_repair_lineage_schema_version: u32,
    pub schema_ref: String,
    pub posture_id: String,
    pub workspace_ref: String,
    pub corpus_ref: String,
    pub producer_attribution: SchemaMigrationProducerAttributionSummary,
    pub artifact_class_coverage: ArtifactClassCoverageSummary,
    pub repair_flow_coverage: RepairFlowCoverageSummary,
    pub schema_version_pinning: SchemaVersionPinningSummary,
    pub outcome_honesty: OutcomeHonestySummary,
    pub preservation: PreservationSummary,
    pub no_silent_rerun: NoSilentRerunSummary,
    pub repair_transaction_pinning: RepairTransactionPinningSummary,
    pub support_export_honesty: SchemaMigrationSupportExportHonestySummary,
    pub inspection_hooks: Vec<SchemaMigrationInspectionHook>,
    pub stable_qualification: SchemaMigrationAndRepairLineageQualification,
    pub raw_payload_excluded: bool,
    pub summary: String,
}

impl SchemaMigrationAndRepairLineageRecord {
    /// Returns true when the record is metadata-safe for support
    /// export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_SCHEMA_REF
            && self.record_kind == SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_RECORD_KIND
            && !self.workspace_ref.trim().is_empty()
            && !self.corpus_ref.trim().is_empty()
    }

    /// Returns true when the record proves the contract on the
    /// claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }

    /// Returns the inspection hook of the given class, when present.
    pub fn inspection_hook(
        &self,
        class: SchemaMigrationInspectionHookClass,
    ) -> Option<&SchemaMigrationInspectionHook> {
        self.inspection_hooks
            .iter()
            .find(|hook| hook.hook_class == class)
    }
}

// ---------------------------------------------------------------------------
// Projection.
// ---------------------------------------------------------------------------

/// Projects a governed schema-migration and repair lineage record from
/// a live [`SchemaMigrationAndRepairInputs`] envelope using the
/// default inspection-hook set.
pub fn project_schema_migration_and_repair_lineage(
    posture_id: impl Into<String>,
    inputs: &SchemaMigrationAndRepairInputs,
) -> SchemaMigrationAndRepairLineageRecord {
    project_schema_migration_and_repair_lineage_with_hooks(
        posture_id,
        inputs,
        default_schema_migration_inspection_hooks(),
    )
}

/// Like [`project_schema_migration_and_repair_lineage`] but with an
/// explicit inspection-hook set (for testing degraded-hook postures).
pub fn project_schema_migration_and_repair_lineage_with_hooks(
    posture_id: impl Into<String>,
    inputs: &SchemaMigrationAndRepairInputs,
    inspection_hooks: Vec<SchemaMigrationInspectionHook>,
) -> SchemaMigrationAndRepairLineageRecord {
    let posture_id: String = posture_id.into();

    let artifact_class_coverage = project_artifact_class_coverage(inputs);
    let repair_flow_coverage = project_repair_flow_coverage(inputs);
    let schema_version_pinning = project_schema_version_pinning(&artifact_class_coverage);
    let outcome_honesty = project_outcome_honesty(&artifact_class_coverage, &repair_flow_coverage);
    let preservation = project_preservation(&artifact_class_coverage, &repair_flow_coverage);
    let no_silent_rerun = project_no_silent_rerun(&artifact_class_coverage, &repair_flow_coverage);
    let repair_transaction_pinning =
        project_repair_transaction_pinning(&artifact_class_coverage, &repair_flow_coverage);
    let support_export_honesty = project_support_export_honesty(inputs);
    let producer_attribution = project_producer_attribution(inputs);

    let known_artifact_ids: BTreeSet<&str> = artifact_class_coverage
        .migration_rows
        .iter()
        .map(|row| row.artifact_id.as_str())
        .collect();

    let mut narrow_reasons = Vec::new();

    if inputs.migrations.is_empty() || inputs.repair_flows.is_empty() {
        narrow_reasons.push(SchemaMigrationAndRepairLineageNarrowReason::CorpusEmpty);
    }
    if !artifact_class_coverage.all_required_artifact_classes_present {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::RequiredArtifactClassMissing);
    }
    if !repair_flow_coverage.all_required_repair_flow_kinds_present {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::RequiredRepairFlowKindMissing);
    }
    if repair_flow_coverage
        .repair_flow_rows
        .iter()
        .any(|row| !known_artifact_ids.contains(row.artifact_id.as_str()))
    {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::RepairReferencesUnknownArtifact);
    }
    if !(schema_version_pinning.all_migrations_pin_from_schema
        && schema_version_pinning.all_migrations_pin_to_schema
        && schema_version_pinning.all_migrations_no_downgrade)
    {
        narrow_reasons.push(SchemaMigrationAndRepairLineageNarrowReason::SchemaVersionUnpinned);
    }
    if !outcome_honesty.all_migration_disclosures_present {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::MigrationDisclosureMissing);
    }
    if !outcome_honesty.all_repair_disclosures_present {
        narrow_reasons.push(SchemaMigrationAndRepairLineageNarrowReason::RepairDisclosureMissing);
    }
    if !outcome_honesty.all_redaction_disclosures_present {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::RedactionDisclosureMissing);
    }
    if !no_silent_rerun.all_rows_safe_rerun_posture {
        narrow_reasons.push(SchemaMigrationAndRepairLineageNarrowReason::RerunSilentForbidden);
    }
    if !no_silent_rerun.all_mutating_rows_have_commit_metadata {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::CommitActionMetadataMissing);
    }
    if !preservation.all_rows_preserve_restore_provenance {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::RestoreProvenanceNotPreserved);
    }
    if !preservation.all_rows_preserve_encoding_fidelity {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::EncodingFidelityNotPreserved);
    }
    if !preservation.all_rows_preserve_trust_state {
        narrow_reasons.push(SchemaMigrationAndRepairLineageNarrowReason::TrustStateNotPreserved);
    }
    if !preservation.all_rows_preserve_no_rerun_semantics {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::NoRerunSemanticsNotPreserved);
    }
    if !repair_transaction_pinning.all_rows_pin_repair_transaction_id {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::RepairTransactionIdNotPinned);
    }
    if !repair_transaction_pinning.all_rows_pin_finding_code {
        narrow_reasons.push(SchemaMigrationAndRepairLineageNarrowReason::FindingCodeMissing);
    }

    let required_hooks = [
        SchemaMigrationInspectionHookClass::InspectArtifact,
        SchemaMigrationInspectionHookClass::CompareBeforeMigration,
        SchemaMigrationInspectionHookClass::PreviewMigration,
        SchemaMigrationInspectionHookClass::PreviewRepair,
        SchemaMigrationInspectionHookClass::RollbackMigration,
        SchemaMigrationInspectionHookClass::RollbackRepair,
        SchemaMigrationInspectionHookClass::ExportBeforeMigration,
        SchemaMigrationInspectionHookClass::ExportBeforeRepair,
    ];
    if !required_hooks
        .iter()
        .all(|required| hook_available(&inspection_hooks, *required))
    {
        narrow_reasons.push(SchemaMigrationAndRepairLineageNarrowReason::InspectionHookUnavailable);
    }

    collect_support_export_narrows(&support_export_honesty, &mut narrow_reasons);

    if !producer_attribution.producer_attribution_complete {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::ProducerAttributionIncomplete);
    }

    if inputs.workspace_ref.trim().is_empty() || inputs.corpus_ref.trim().is_empty() {
        narrow_reasons.push(SchemaMigrationAndRepairLineageNarrowReason::LineageExportUnsafe);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = SchemaMigrationAndRepairLineageQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(
        &artifact_class_coverage,
        &repair_flow_coverage,
        &stable_qualification,
    );

    SchemaMigrationAndRepairLineageRecord {
        record_kind: SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_RECORD_KIND.to_owned(),
        schema_migration_and_repair_lineage_schema_version:
            SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_SCHEMA_VERSION,
        schema_ref: SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_SCHEMA_REF.to_owned(),
        posture_id,
        workspace_ref: inputs.workspace_ref.clone(),
        corpus_ref: inputs.corpus_ref.clone(),
        producer_attribution,
        artifact_class_coverage,
        repair_flow_coverage,
        schema_version_pinning,
        outcome_honesty,
        preservation,
        no_silent_rerun,
        repair_transaction_pinning,
        support_export_honesty,
        inspection_hooks,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

// ---------------------------------------------------------------------------
// Pillar builders.
// ---------------------------------------------------------------------------

fn project_artifact_class_coverage(
    inputs: &SchemaMigrationAndRepairInputs,
) -> ArtifactClassCoverageSummary {
    let migration_rows: Vec<MigrationRow> = inputs
        .migrations
        .iter()
        .map(project_migration_row)
        .collect();
    let observed: BTreeSet<_> = migration_rows
        .iter()
        .map(|row| row.artifact_class)
        .collect();
    let all_required_artifact_classes_present = REQUIRED_ARTIFACT_CLASSES
        .iter()
        .all(|required| observed.contains(required));
    ArtifactClassCoverageSummary {
        migration_rows,
        all_required_artifact_classes_present,
    }
}

fn project_migration_row(observation: &MigrationObservation) -> MigrationRow {
    MigrationRow {
        artifact_id: observation.artifact_id.clone(),
        artifact_class: observation.artifact_class,
        artifact_ref: observation.artifact_ref.clone(),
        schema_compatibility_class: observation.schema_compatibility_class,
        from_schema_version: observation.from_schema_version,
        to_schema_version: observation.to_schema_version,
        migration_outcome: observation.migration_outcome,
        migration_disclosure_ref: observation.migration_disclosure_ref.clone(),
        repair_transaction_id: observation.repair_transaction_id.clone(),
        finding_code: observation.finding_code.clone(),
        preserves_restore_provenance: observation.preserves_restore_provenance,
        preserves_encoding_fidelity: observation.preserves_encoding_fidelity,
        preserves_trust_state: observation.preserves_trust_state,
        preserves_no_rerun_semantics: observation.preserves_no_rerun_semantics,
        rerun_posture: observation.rerun_posture,
        commit_action_id: observation.commit_action_id.clone(),
        commit_disclosure_id: observation.commit_disclosure_id.clone(),
        redaction_class: observation.redaction_class,
        redaction_disclosure_ref: observation.redaction_disclosure_ref.clone(),
        support_export_posture: observation.support_export.posture,
        mutates_state: observation.migration_outcome.mutates_state(),
        is_required: observation.artifact_class.is_required(),
    }
}

fn project_repair_flow_coverage(
    inputs: &SchemaMigrationAndRepairInputs,
) -> RepairFlowCoverageSummary {
    let repair_flow_rows: Vec<RepairFlowRow> = inputs
        .repair_flows
        .iter()
        .map(project_repair_flow_row)
        .collect();
    let observed: BTreeSet<_> = repair_flow_rows
        .iter()
        .map(|row| row.repair_flow_kind)
        .collect();
    let all_required_repair_flow_kinds_present = REQUIRED_REPAIR_FLOW_KINDS
        .iter()
        .all(|required| observed.contains(required));
    RepairFlowCoverageSummary {
        repair_flow_rows,
        all_required_repair_flow_kinds_present,
    }
}

fn project_repair_flow_row(observation: &RepairFlowObservation) -> RepairFlowRow {
    RepairFlowRow {
        repair_flow_id: observation.repair_flow_id.clone(),
        label: observation.label.clone(),
        repair_flow_kind: observation.repair_flow_kind,
        artifact_id: observation.artifact_id.clone(),
        repair_outcome: observation.repair_outcome,
        repair_disclosure_ref: observation.repair_disclosure_ref.clone(),
        repair_transaction_id: observation.repair_transaction_id.clone(),
        finding_code: observation.finding_code.clone(),
        preserves_restore_provenance: observation.preserves_restore_provenance,
        preserves_encoding_fidelity: observation.preserves_encoding_fidelity,
        preserves_trust_state: observation.preserves_trust_state,
        preserves_no_rerun_semantics: observation.preserves_no_rerun_semantics,
        rerun_posture: observation.rerun_posture,
        commit_action_id: observation.commit_action_id.clone(),
        commit_disclosure_id: observation.commit_disclosure_id.clone(),
        redaction_class: observation.redaction_class,
        redaction_disclosure_ref: observation.redaction_disclosure_ref.clone(),
        support_export_posture: observation.support_export.posture,
        mutates_state: observation.repair_flow_kind.mutates_state(),
        is_required: true,
    }
}

fn project_schema_version_pinning(
    coverage: &ArtifactClassCoverageSummary,
) -> SchemaVersionPinningSummary {
    let mut from_ok = true;
    let mut to_ok = true;
    let mut no_downgrade = true;
    for row in &coverage.migration_rows {
        if row.from_schema_version == 0 {
            from_ok = false;
        }
        if row.to_schema_version == 0 {
            to_ok = false;
        }
        if row.to_schema_version < row.from_schema_version {
            no_downgrade = false;
        }
    }
    SchemaVersionPinningSummary {
        all_migrations_pin_from_schema: from_ok,
        all_migrations_pin_to_schema: to_ok,
        all_migrations_no_downgrade: no_downgrade,
    }
}

fn project_outcome_honesty(
    artifact_coverage: &ArtifactClassCoverageSummary,
    repair_coverage: &RepairFlowCoverageSummary,
) -> OutcomeHonestySummary {
    let mut mig_ok = true;
    let mut rep_ok = true;
    let mut red_ok = true;
    for row in &artifact_coverage.migration_rows {
        if row.migration_outcome.requires_disclosure()
            && row
                .migration_disclosure_ref
                .as_ref()
                .map_or(true, |value| value.trim().is_empty())
        {
            mig_ok = false;
        }
        if row.redaction_class.requires_override_disclosure()
            && row
                .redaction_disclosure_ref
                .as_ref()
                .map_or(true, |value| value.trim().is_empty())
        {
            red_ok = false;
        }
    }
    for row in &repair_coverage.repair_flow_rows {
        if row.repair_outcome.requires_disclosure()
            && row
                .repair_disclosure_ref
                .as_ref()
                .map_or(true, |value| value.trim().is_empty())
        {
            rep_ok = false;
        }
        if row.redaction_class.requires_override_disclosure()
            && row
                .redaction_disclosure_ref
                .as_ref()
                .map_or(true, |value| value.trim().is_empty())
        {
            red_ok = false;
        }
    }
    OutcomeHonestySummary {
        all_migration_disclosures_present: mig_ok,
        all_repair_disclosures_present: rep_ok,
        all_redaction_disclosures_present: red_ok,
    }
}

fn project_preservation(
    artifact_coverage: &ArtifactClassCoverageSummary,
    repair_coverage: &RepairFlowCoverageSummary,
) -> PreservationSummary {
    let mut prov_ok = true;
    let mut enc_ok = true;
    let mut trust_ok = true;
    let mut rerun_ok = true;
    for row in &artifact_coverage.migration_rows {
        if !row.preserves_restore_provenance {
            prov_ok = false;
        }
        if !row.preserves_encoding_fidelity {
            enc_ok = false;
        }
        if !row.preserves_trust_state {
            trust_ok = false;
        }
        if !row.preserves_no_rerun_semantics {
            rerun_ok = false;
        }
    }
    for row in &repair_coverage.repair_flow_rows {
        if !row.preserves_restore_provenance {
            prov_ok = false;
        }
        if !row.preserves_encoding_fidelity {
            enc_ok = false;
        }
        if !row.preserves_trust_state {
            trust_ok = false;
        }
        if !row.preserves_no_rerun_semantics {
            rerun_ok = false;
        }
    }
    PreservationSummary {
        all_rows_preserve_restore_provenance: prov_ok,
        all_rows_preserve_encoding_fidelity: enc_ok,
        all_rows_preserve_trust_state: trust_ok,
        all_rows_preserve_no_rerun_semantics: rerun_ok,
    }
}

fn project_no_silent_rerun(
    artifact_coverage: &ArtifactClassCoverageSummary,
    repair_coverage: &RepairFlowCoverageSummary,
) -> NoSilentRerunSummary {
    let mut posture_ok = true;
    let mut commit_ok = true;
    for row in &artifact_coverage.migration_rows {
        if !row.rerun_posture.safe_for_stable() {
            posture_ok = false;
        }
        if row.mutates_state
            && (row.commit_action_id.trim().is_empty()
                || row.commit_disclosure_id.trim().is_empty())
        {
            commit_ok = false;
        }
    }
    for row in &repair_coverage.repair_flow_rows {
        if !row.rerun_posture.safe_for_stable() {
            posture_ok = false;
        }
        if row.mutates_state
            && (row.commit_action_id.trim().is_empty()
                || row.commit_disclosure_id.trim().is_empty())
        {
            commit_ok = false;
        }
    }
    NoSilentRerunSummary {
        all_rows_safe_rerun_posture: posture_ok,
        all_mutating_rows_have_commit_metadata: commit_ok,
    }
}

fn project_repair_transaction_pinning(
    artifact_coverage: &ArtifactClassCoverageSummary,
    repair_coverage: &RepairFlowCoverageSummary,
) -> RepairTransactionPinningSummary {
    let mut tx_ok = true;
    let mut finding_ok = true;
    for row in &artifact_coverage.migration_rows {
        if row.repair_transaction_id.trim().is_empty() {
            tx_ok = false;
        }
        if row.finding_code.trim().is_empty() {
            finding_ok = false;
        }
    }
    for row in &repair_coverage.repair_flow_rows {
        if row.repair_transaction_id.trim().is_empty() {
            tx_ok = false;
        }
        if row.finding_code.trim().is_empty() {
            finding_ok = false;
        }
    }
    RepairTransactionPinningSummary {
        all_rows_pin_repair_transaction_id: tx_ok,
        all_rows_pin_finding_code: finding_ok,
    }
}

fn project_support_export_honesty(
    inputs: &SchemaMigrationAndRepairInputs,
) -> SchemaMigrationSupportExportHonestySummary {
    let mut preserve_fields = true;
    let mut redact_secrets = true;
    let mut exclude_bytes = true;
    let mut exclude_approvals = true;
    let mut exclude_credentials = true;
    let mut exclude_authority = true;

    let supports = inputs
        .migrations
        .iter()
        .map(|m| m.support_export)
        .chain(inputs.repair_flows.iter().map(|r| r.support_export));

    for support in supports {
        if !(support.includes_artifact_class
            && support.includes_migration_outcome_or_repair_flow
            && support.includes_schema_versions
            && support.includes_repair_transaction_id
            && support.includes_finding_code
            && support.includes_redaction_class)
        {
            preserve_fields = false;
        }
        if !support.raw_secrets_excluded {
            redact_secrets = false;
        }
        if !support.raw_artifact_bytes_excluded {
            exclude_bytes = false;
        }
        if !support.approval_tickets_excluded {
            exclude_approvals = false;
        }
        if !support.delegated_credentials_excluded {
            exclude_credentials = false;
        }
        if !support.live_authority_handles_excluded {
            exclude_authority = false;
        }
    }

    SchemaMigrationSupportExportHonestySummary {
        all_rows_preserve_fields: preserve_fields,
        all_rows_exclude_raw_secrets: redact_secrets,
        all_rows_exclude_raw_artifact_bytes: exclude_bytes,
        all_rows_exclude_approval_tickets: exclude_approvals,
        all_rows_exclude_delegated_credentials: exclude_credentials,
        all_rows_exclude_live_authority_handles: exclude_authority,
    }
}

fn project_producer_attribution(
    inputs: &SchemaMigrationAndRepairInputs,
) -> SchemaMigrationProducerAttributionSummary {
    let integrity_hash = compute_integrity_hash(inputs);
    let producer_attribution_complete =
        !inputs.producer_ref.trim().is_empty() && !inputs.captured_at.trim().is_empty();
    SchemaMigrationProducerAttributionSummary {
        producer_ref: inputs.producer_ref.clone(),
        schema_version: SCHEMA_MIGRATION_AND_REPAIR_LINEAGE_SCHEMA_VERSION,
        integrity_hash,
        captured_at: inputs.captured_at.clone(),
        producer_attribution_complete,
    }
}

fn collect_support_export_narrows(
    summary: &SchemaMigrationSupportExportHonestySummary,
    narrow_reasons: &mut Vec<SchemaMigrationAndRepairLineageNarrowReason>,
) {
    if !summary.all_rows_preserve_fields {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::SupportExportFieldsDropped);
    }
    if !(summary.all_rows_exclude_raw_secrets
        && summary.all_rows_exclude_raw_artifact_bytes
        && summary.all_rows_exclude_approval_tickets
        && summary.all_rows_exclude_delegated_credentials
        && summary.all_rows_exclude_live_authority_handles)
    {
        narrow_reasons
            .push(SchemaMigrationAndRepairLineageNarrowReason::SupportExportRedactionUnsafe);
    }
}

fn compute_integrity_hash(inputs: &SchemaMigrationAndRepairInputs) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    let header = [
        inputs.workspace_ref.as_str(),
        inputs.producer_ref.as_str(),
        inputs.corpus_ref.as_str(),
        inputs.captured_at.as_str(),
    ];
    for input in header {
        for byte in input.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for migration in &inputs.migrations {
        for byte in migration.artifact_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(migration.artifact_class.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(migration.migration_outcome.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    for repair in &inputs.repair_flows {
        for byte in repair.repair_flow_id.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(repair.repair_flow_kind.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
        hash ^= u64::from(repair.repair_outcome.as_str().len() as u8);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("smr:{hash:016x}")
}

fn hook_available(
    hooks: &[SchemaMigrationInspectionHook],
    class: SchemaMigrationInspectionHookClass,
) -> bool {
    hooks
        .iter()
        .find(|hook| hook.hook_class == class)
        .map(|hook| hook.available)
        .unwrap_or(false)
}

fn build_summary(
    artifact_coverage: &ArtifactClassCoverageSummary,
    repair_coverage: &RepairFlowCoverageSummary,
    qualification: &SchemaMigrationAndRepairLineageQualification,
) -> String {
    if qualification.qualified {
        format!(
            "Schema-migration and repair lineage proven Stable: artifacts={artifacts} repair_flows={repairs}.",
            artifacts = artifact_coverage.migration_rows.len(),
            repairs = repair_coverage.repair_flow_rows.len(),
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Schema-migration and repair lineage narrowed below Stable (artifacts={artifacts} repair_flows={repairs}): {reasons}.",
            artifacts = artifact_coverage.migration_rows.len(),
            repairs = repair_coverage.repair_flow_rows.len(),
            reasons = reasons.join(", "),
        )
    }
}

// ---------------------------------------------------------------------------
// Human-readable projection (for headless emitter / shell status surface).
// ---------------------------------------------------------------------------

/// Returns the human-readable projection of a schema-migration and
/// repair lineage record. The same projection is consumed by the
/// workspace schema-migration status surface, the headless CLI
/// emitter, Help/About, and support export.
pub fn schema_migration_and_repair_lineage_lines(
    record: &SchemaMigrationAndRepairLineageRecord,
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Schema-migration and repair lineage — {} ({})",
        record.posture_id, record.stable_qualification.level
    ));
    lines.push(format!(
        "workspace={} corpus={} producer={} integrity_hash={} captured_at={}",
        record.workspace_ref,
        record.corpus_ref,
        record.producer_attribution.producer_ref,
        record.producer_attribution.integrity_hash,
        record.producer_attribution.captured_at,
    ));
    lines.push(format!(
        "artifact_class_coverage: artifacts={} required_present={}",
        record.artifact_class_coverage.migration_rows.len(),
        record
            .artifact_class_coverage
            .all_required_artifact_classes_present,
    ));
    lines.push("Migrations:".to_owned());
    for row in &record.artifact_class_coverage.migration_rows {
        let mig_disclosure = row.migration_disclosure_ref.as_deref().unwrap_or("none");
        let red_disclosure = row.redaction_disclosure_ref.as_deref().unwrap_or("none");
        lines.push(format!(
            "  - {class} {id} ref={artifact_ref} compat={compat} from={from_v} to={to_v} outcome={outcome} mig_disclosure={mig_disclosure} repair_tx={tx} finding={finding} preserves_provenance={prov} preserves_encoding={enc} preserves_trust={trust} preserves_no_rerun={rerun} rerun_posture={rerun_posture} commit_action={commit_action} commit_disclosure={commit_disclosure} redaction={red} red_disclosure={red_disclosure} mutates={mutates} required={required} support_export={support}",
            class = row.artifact_class.as_str(),
            id = row.artifact_id,
            artifact_ref = row.artifact_ref,
            compat = row.schema_compatibility_class.as_str(),
            from_v = row.from_schema_version,
            to_v = row.to_schema_version,
            outcome = row.migration_outcome.as_str(),
            mig_disclosure = mig_disclosure,
            tx = row.repair_transaction_id,
            finding = row.finding_code,
            prov = row.preserves_restore_provenance,
            enc = row.preserves_encoding_fidelity,
            trust = row.preserves_trust_state,
            rerun = row.preserves_no_rerun_semantics,
            rerun_posture = row.rerun_posture.as_str(),
            commit_action = row.commit_action_id,
            commit_disclosure = row.commit_disclosure_id,
            red = row.redaction_class.as_str(),
            red_disclosure = red_disclosure,
            mutates = row.mutates_state,
            required = row.is_required,
            support = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "repair_flow_coverage: flows={} required_present={}",
        record.repair_flow_coverage.repair_flow_rows.len(),
        record
            .repair_flow_coverage
            .all_required_repair_flow_kinds_present,
    ));
    lines.push("Repair flows:".to_owned());
    for row in &record.repair_flow_coverage.repair_flow_rows {
        let rep_disclosure = row.repair_disclosure_ref.as_deref().unwrap_or("none");
        let red_disclosure = row.redaction_disclosure_ref.as_deref().unwrap_or("none");
        lines.push(format!(
            "  - {kind} {id} artifact={artifact} outcome={outcome} rep_disclosure={rep_disclosure} repair_tx={tx} finding={finding} preserves_provenance={prov} preserves_encoding={enc} preserves_trust={trust} preserves_no_rerun={rerun} rerun_posture={rerun_posture} commit_action={commit_action} commit_disclosure={commit_disclosure} redaction={red} red_disclosure={red_disclosure} mutates={mutates} required={required} support_export={support}",
            kind = row.repair_flow_kind.as_str(),
            id = row.repair_flow_id,
            artifact = row.artifact_id,
            outcome = row.repair_outcome.as_str(),
            rep_disclosure = rep_disclosure,
            tx = row.repair_transaction_id,
            finding = row.finding_code,
            prov = row.preserves_restore_provenance,
            enc = row.preserves_encoding_fidelity,
            trust = row.preserves_trust_state,
            rerun = row.preserves_no_rerun_semantics,
            rerun_posture = row.rerun_posture.as_str(),
            commit_action = row.commit_action_id,
            commit_disclosure = row.commit_disclosure_id,
            red = row.redaction_class.as_str(),
            red_disclosure = red_disclosure,
            mutates = row.mutates_state,
            required = row.is_required,
            support = row.support_export_posture.as_str(),
        ));
    }
    lines.push(format!(
        "Schema-version pinning: from={f} to={t} no_downgrade={nd}",
        f = record.schema_version_pinning.all_migrations_pin_from_schema,
        t = record.schema_version_pinning.all_migrations_pin_to_schema,
        nd = record.schema_version_pinning.all_migrations_no_downgrade,
    ));
    lines.push(format!(
        "Outcome honesty: migrations={m} repairs={r} redactions={red}",
        m = record.outcome_honesty.all_migration_disclosures_present,
        r = record.outcome_honesty.all_repair_disclosures_present,
        red = record.outcome_honesty.all_redaction_disclosures_present,
    ));
    lines.push(format!(
        "Preservation: provenance={p} encoding={e} trust={t} no_rerun={nr}",
        p = record.preservation.all_rows_preserve_restore_provenance,
        e = record.preservation.all_rows_preserve_encoding_fidelity,
        t = record.preservation.all_rows_preserve_trust_state,
        nr = record.preservation.all_rows_preserve_no_rerun_semantics,
    ));
    lines.push(format!(
        "No-silent-rerun: posture={p} commit_metadata={c}",
        p = record.no_silent_rerun.all_rows_safe_rerun_posture,
        c = record
            .no_silent_rerun
            .all_mutating_rows_have_commit_metadata,
    ));
    lines.push(format!(
        "Repair-transaction pinning: tx={t} finding={f}",
        t = record
            .repair_transaction_pinning
            .all_rows_pin_repair_transaction_id,
        f = record.repair_transaction_pinning.all_rows_pin_finding_code,
    ));
    lines.push(format!(
        "Support-export honesty: preserve_fields={fields} exclude_secrets={secrets} exclude_bytes={bytes} exclude_approvals={approvals} exclude_credentials={credentials} exclude_authority={authority}",
        fields = record.support_export_honesty.all_rows_preserve_fields,
        secrets = record.support_export_honesty.all_rows_exclude_raw_secrets,
        bytes = record
            .support_export_honesty
            .all_rows_exclude_raw_artifact_bytes,
        approvals = record
            .support_export_honesty
            .all_rows_exclude_approval_tickets,
        credentials = record
            .support_export_honesty
            .all_rows_exclude_delegated_credentials,
        authority = record
            .support_export_honesty
            .all_rows_exclude_live_authority_handles,
    ));
    lines.push("Inspection hooks:".to_owned());
    for hook in &record.inspection_hooks {
        lines.push(format!(
            "  {class} [{id}] available={available} — {label}",
            class = hook.hook_class.as_str(),
            id = hook.action_id,
            available = hook.available,
            label = hook.label,
        ));
    }
    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }
    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests;
