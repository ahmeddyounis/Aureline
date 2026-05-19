//! Storage inspector, class-selective clear-data review, and cleanup-receipt
//! support projection.
//!
//! This module is the live shell's first trustworthy storage-truth path. It
//! folds the checked-in storage-class registry and the seeded clear-data
//! review / cleanup-receipt fixtures into typed records the chrome storage
//! inspector, low-disk banners, cleanup history, and support exports can
//! consume verbatim.
//!
//! ## What this seed owns
//!
//! - The [`StorageClassRegistry`] — the canonical truth model for the six
//!   storage classes the shell tracks (interactive hot cache, knowledge
//!   cache, artifact cache, prebuild/environment cache, evidence/support
//!   cache, and user-owned recovery state). It mirrors the boundary schema
//!   at [`/schemas/support/storage_class.schema.json`].
//! - The [`ClearDataReview`] record — the class-selective review sheet that
//!   names selected classes, affected workspaces, what will be rebuilt
//!   versus lost, pin/protection reasons, and export-before-delete options.
//!   Mirrors [`/schemas/support/clear_data_review.schema.json`].
//! - The [`StorageCleanupReceipt`] record — the metadata-safe receipt
//!   minted after cleanup runs, including actor, timestamp, classes
//!   affected, blocked pins, bytes reclaimed, resulting stale/rebuild
//!   state, and the reopen-inspector routing. Mirrors
//!   [`/schemas/support/storage_cleanup_receipt.schema.json`].
//! - The [`StorageCleanupCorpus`] container — folds the registry and every
//!   seeded scenario into a single validated bundle and projects them into
//!   a metadata-safe [`StorageCleanupSupportExport`] the support-bundle
//!   pipeline can quote without leaking raw payloads, raw paths, or raw
//!   credential bodies.
//!
//! ## What this seed does NOT own
//!
//! - Live byte-level garbage collection, eviction scheduling, or quota
//!   enforcement. Those belong to the runtime crates.
//! - Hosted intake or ticket routing. The receipt rows stay metadata-safe.
//! - Repair-transaction journaling. The receipt only quotes references to
//!   the repair lane; the repair preview itself remains owned by the
//!   support repair contract.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for one storage-class registry entry.
pub const STORAGE_CLASS_REGISTRY_ENTRY_RECORD_KIND: &str = "storage_class_registry_entry";

/// Stable record-kind tag for the registry container.
pub const STORAGE_CLASS_REGISTRY_RECORD_KIND: &str = "storage_class_registry";

/// Stable record-kind tag for the clear-data review sheet.
pub const CLEAR_DATA_REVIEW_RECORD_KIND: &str = "clear_data_review_record";

/// Stable record-kind tag for the storage cleanup receipt.
pub const STORAGE_CLEANUP_RECEIPT_RECORD_KIND: &str = "storage_cleanup_receipt_record";

/// Stable record-kind tag for a corpus scenario pairing review + receipt.
pub const STORAGE_CLEANUP_SCENARIO_RECORD_KIND: &str = "storage_cleanup_scenario_record";

/// Stable record-kind tag for the corpus manifest.
pub const STORAGE_CLEANUP_CORPUS_MANIFEST_RECORD_KIND: &str = "storage_cleanup_corpus_manifest";

/// Stable record-kind tag for the support-export row.
pub const STORAGE_CLEANUP_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "storage_cleanup_support_export_row";

/// Stable record-kind tag for the support-export envelope.
pub const STORAGE_CLEANUP_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "storage_cleanup_support_export_envelope";

/// Frozen schema version shared by every record in this module.
pub const STORAGE_INSPECTOR_SCHEMA_VERSION: u32 = 1;

/// Repository-relative path of the boundary schema for the registry.
pub const STORAGE_CLASS_SCHEMA_REF: &str = "schemas/support/storage_class.schema.json";

/// Repository-relative path of the boundary schema for the review sheet.
pub const CLEAR_DATA_REVIEW_SCHEMA_REF: &str = "schemas/support/clear_data_review.schema.json";

/// Repository-relative path of the boundary schema for the cleanup receipt.
pub const STORAGE_CLEANUP_RECEIPT_SCHEMA_REF: &str =
    "schemas/support/storage_cleanup_receipt.schema.json";

/// Reviewer doc ref every emitted record quotes.
pub const STORAGE_CLEANUP_DOC_REF: &str = "docs/support/m3/storage_cleanup_beta.md";

/// Repository-relative path of the protected corpus directory.
pub const STORAGE_CLEANUP_CORPUS_DIR: &str = "fixtures/support/m3/storage_cleanup";

/// Repository-relative path of the corpus manifest.
pub const STORAGE_CLEANUP_CORPUS_MANIFEST_REF: &str =
    "fixtures/support/m3/storage_cleanup/manifest.yaml";

/// Repository-relative path of the registry fixture.
pub const STORAGE_CLEANUP_REGISTRY_REF: &str = "fixtures/support/m3/storage_cleanup/registry.yaml";

const REGISTRY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/support/m3/storage_cleanup/registry.yaml"
));

const SCENARIO_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/support/m3/storage_cleanup/scenario.user_requested_hot_and_knowledge_cleanup.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/storage_cleanup/scenario.user_requested_hot_and_knowledge_cleanup.yaml"
        )),
    ),
    (
        "fixtures/support/m3/storage_cleanup/scenario.low_disk_ordered_eviction.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/storage_cleanup/scenario.low_disk_ordered_eviction.yaml"
        )),
    ),
    (
        "fixtures/support/m3/storage_cleanup/scenario.evidence_export_before_delete.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/storage_cleanup/scenario.evidence_export_before_delete.yaml"
        )),
    ),
    (
        "fixtures/support/m3/storage_cleanup/scenario.pinned_artifact_blocks_cleanup.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/storage_cleanup/scenario.pinned_artifact_blocks_cleanup.yaml"
        )),
    ),
    (
        "fixtures/support/m3/storage_cleanup/scenario.corruption_repair_prebuild_cache.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/storage_cleanup/scenario.corruption_repair_prebuild_cache.yaml"
        )),
    ),
    (
        "fixtures/support/m3/storage_cleanup/scenario.cancelled_recovery_state_override.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/support/m3/storage_cleanup/scenario.cancelled_recovery_state_override.yaml"
        )),
    ),
];

const REQUIRED_TRIGGER_CLASSES: &[TriggerClass] = &[
    TriggerClass::UserRequestedCleanup,
    TriggerClass::LowDiskPressure,
    TriggerClass::CorruptionRepairRequest,
];

const REQUIRED_PROTECTED_CLASSES: &[StorageClassId] = &[
    StorageClassId::EvidenceSupportCache,
    StorageClassId::UserOwnedRecoveryState,
];

const REQUIRED_REGISTRY_CLASSES: &[StorageClassId] = &[
    StorageClassId::InteractiveHotCache,
    StorageClassId::KnowledgeCache,
    StorageClassId::ArtifactCache,
    StorageClassId::PrebuildEnvironmentCache,
    StorageClassId::EvidenceSupportCache,
    StorageClassId::UserOwnedRecoveryState,
];

/// Closed storage-class id vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StorageClassId {
    /// Disposable session-local cache that rebuilds within one interaction.
    InteractiveHotCache,
    /// Workspace-derived semantic index of code, docs, and embeddings.
    KnowledgeCache,
    /// Reproducible build artifacts and provider-derived blobs.
    ArtifactCache,
    /// Toolchain, language-runtime, and prebuild environment artifacts.
    PrebuildEnvironmentCache,
    /// Crash reports, support-bundle drafts, and incident evidence.
    EvidenceSupportCache,
    /// Local history, rollback checkpoints, dirty-buffer journals, pinned
    /// review artifacts, offline entitlement bundles, and last-known-good
    /// policy bundles.
    UserOwnedRecoveryState,
}

impl StorageClassId {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InteractiveHotCache => "interactive_hot_cache",
            Self::KnowledgeCache => "knowledge_cache",
            Self::ArtifactCache => "artifact_cache",
            Self::PrebuildEnvironmentCache => "prebuild_environment_cache",
            Self::EvidenceSupportCache => "evidence_support_cache",
            Self::UserOwnedRecoveryState => "user_owned_recovery_state",
        }
    }
}

/// Closed scope-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeClass {
    ProcessLocal,
    WorkspaceLocal,
    ProfileLocal,
    MachineLocal,
    UserAccountLocal,
}

/// Closed authority-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityClass {
    RuntimeDisposable,
    ProviderDerived,
    WorkspaceDerived,
    PolicyOwned,
    UserAuthoredRecovery,
    EvidenceGrade,
}

/// Closed rebuild-cost vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildCostClass {
    None,
    CheapLocal,
    ExpensiveLocal,
    NetworkRequired,
    Irrecoverable,
}

/// Closed sensitivity-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitivityClass {
    NonSensitive,
    MetadataOnly,
    RedactedContent,
    PrivateUserContent,
    EvidenceGrade,
}

/// Closed garbage-collection policy vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GcPolicyClass {
    /// Disposable; ordinary cleanup may evict without a per-item review.
    EvictionEligible,
    /// Eviction requires a confirmation step before the bytes are removed.
    EvictionWithReview,
    /// Pinned by default; only an explicit per-item override may evict.
    PinDefault,
    /// Never evicted silently; protected from ordinary cleanup paths.
    NeverEvictSilently,
}

/// Closed actor-lineage vocabulary for review and receipt records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorLineageClass {
    UserInvokedInShell,
    AdminInvokedInConsole,
    AutomationInvoked,
    LowDiskEvictionPrompt,
}

/// Closed trigger-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TriggerClass {
    UserRequestedCleanup,
    LowDiskPressure,
    QuotaPressure,
    CorruptionRepairRequest,
}

/// Closed consequence-class vocabulary used on review rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsequenceClass {
    RebuiltCheaplyOnDemand,
    RebuiltWithExpensiveLocalWork,
    RebuiltRequiresNetwork,
    BecomesStaleUntilRebuilt,
    LostWithNoRebuildPath,
}

/// Closed export-before-delete vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportBeforeDeleteClass {
    NotApplicable,
    Available,
    RequiredBeforeDelete,
    UnavailableOfflineOnly,
}

/// Closed pin-source vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinSourceClass {
    UserPin,
    WorkspacePin,
    PolicyPin,
    EvidenceHold,
    RollbackCheckpointPin,
    LastKnownGoodPolicyPin,
    OfflineEntitlementPin,
}

/// Closed protected-reason vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProtectedReasonClass {
    UserOwnedRecoveryState,
    EvidenceClass,
    PolicyPin,
    ActiveHold,
    RollbackCheckpoint,
    DirtyBufferJournal,
    OfflineEntitlementBundle,
    LastKnownGoodPolicyBundle,
}

/// Closed consent-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentState {
    Unconfirmed,
    Confirmed,
    Cancelled,
    BlockedByProtectedClass,
}

/// Closed cleanup result-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupResultClass {
    Completed,
    Partial,
    BlockedByPin,
    BlockedByProtectedClass,
    Cancelled,
    NoOpNothingToReclaim,
}

/// Closed rebuild-state vocabulary for receipts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildStateClass {
    FreshNoRebuildNeeded,
    RebuildOnDemandCheaply,
    RebuildOnDemandExpensive,
    RebuildRequiresNetwork,
    StaleUntilRebuilt,
    IrrecoverableWithoutExternalSource,
}

/// Closed low-disk state-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LowDiskStateClass {
    NotTriggered,
    Warning,
    Critical,
    QuotaPressure,
}

/// One entry in the storage-class registry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageClassRegistryEntry {
    pub schema_version: u32,
    pub record_kind: String,
    pub class_id: StorageClassId,
    pub label: String,
    pub summary: String,
    pub scope_class: ScopeClass,
    pub authority_class: AuthorityClass,
    pub rebuild_cost_class: RebuildCostClass,
    pub sensitivity_class: SensitivityClass,
    pub gc_policy_class: GcPolicyClass,
    pub low_disk_eviction_priority: u32,
    pub protected_default: bool,
    #[serde(default)]
    pub protected_class_examples: Vec<String>,
    pub schema_ref: String,
    pub doc_ref: String,
}

/// Storage-class registry container.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageClassRegistry {
    pub schema_version: u32,
    pub record_kind: String,
    pub registry_id: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub emitted_at: String,
    pub entries: Vec<StorageClassRegistryEntry>,
}

impl StorageClassRegistry {
    /// Returns the entry for the given class id, if present.
    pub fn entry(&self, class_id: StorageClassId) -> Option<&StorageClassRegistryEntry> {
        self.entries.iter().find(|entry| entry.class_id == class_id)
    }

    /// Returns true when the class is protected from ordinary cleanup by default.
    pub fn is_protected_default(&self, class_id: StorageClassId) -> bool {
        self.entry(class_id)
            .map(|entry| entry.protected_default)
            .unwrap_or(false)
    }

    /// Returns the closed list of class ids the registry must cover.
    pub fn required_class_ids() -> &'static [StorageClassId] {
        REQUIRED_REGISTRY_CLASSES
    }
}

/// One row naming a workspace's bytes-in-scope for a storage class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceScopeRow {
    pub workspace_ref: String,
    pub class_id: StorageClassId,
    pub bytes_in_scope: u64,
    pub summary: String,
}

/// One selected-class row on a clear-data review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectedClassRow {
    pub class_id: StorageClassId,
    pub bytes_in_scope: u64,
    pub consequence_class: ConsequenceClass,
    pub consequence_summary: String,
    pub export_before_delete_class: ExportBeforeDeleteClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_target_ref: Option<String>,
}

/// One protected-class row on a clear-data review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProtectedClassRow {
    pub class_id: StorageClassId,
    pub protected_reason_class: ProtectedReasonClass,
    pub reason_summary: String,
}

/// One pin-block row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PinBlockRow {
    pub pin_ref: String,
    pub class_id: StorageClassId,
    pub pin_source_class: PinSourceClass,
    pub pin_summary: String,
}

/// One override row for a protected class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OverrideProtectedClassRow {
    pub class_id: StorageClassId,
    pub override_justification: String,
    pub consequence_class: ConsequenceClass,
    pub consequence_summary: String,
}

/// Class-selective clear-data review sheet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataReview {
    pub schema_version: u32,
    pub record_kind: String,
    pub review_id: String,
    pub presented_at: String,
    pub actor_lineage_class: ActorLineageClass,
    pub trigger_class: TriggerClass,
    pub selected_class_refs: Vec<SelectedClassRow>,
    #[serde(default)]
    pub workspace_scope_rows: Vec<WorkspaceScopeRow>,
    pub protected_class_rows: Vec<ProtectedClassRow>,
    #[serde(default)]
    pub pin_block_rows: Vec<PinBlockRow>,
    #[serde(default)]
    pub override_protected_class_refs: Vec<OverrideProtectedClassRow>,
    pub consent_state: ConsentState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmed_by_ref: Option<String>,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub storage_class_registry_ref: String,
    pub reviewer_summary: String,
}

/// One outcome row on a cleanup receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClassOutcomeRow {
    pub class_id: StorageClassId,
    pub bytes_reclaimed: u64,
    pub rebuild_state_class: RebuildStateClass,
    pub rebuild_summary: String,
}

/// One blocked-pin row on a cleanup receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockedPinRow {
    pub pin_ref: String,
    pub class_id: StorageClassId,
    pub pin_source_class: PinSourceClass,
    pub pin_summary: String,
}

/// One skipped protected-class row on a cleanup receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SkippedProtectedClassRow {
    pub class_id: StorageClassId,
    pub protected_reason_class: ProtectedReasonClass,
    pub reason_summary: String,
}

/// One ordered eviction step recorded in the low-disk context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LowDiskEvictionStep {
    pub order: u32,
    pub class_id: StorageClassId,
    pub bytes_evicted: u64,
    pub summary: String,
}

/// One paused-work row recorded in the low-disk context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PausedWorkRow {
    pub work_ref: String,
    pub summary: String,
}

/// Low-disk context attached to a cleanup receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LowDiskContext {
    pub state_class: LowDiskStateClass,
    pub ordered_eviction_steps: Vec<LowDiskEvictionStep>,
    #[serde(default)]
    pub paused_work_rows: Vec<PausedWorkRow>,
    pub reviewer_summary: String,
}

/// Storage cleanup receipt record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCleanupReceipt {
    pub schema_version: u32,
    pub record_kind: String,
    pub receipt_id: String,
    pub review_ref: String,
    pub executed_at: String,
    pub actor_lineage_class: ActorLineageClass,
    pub trigger_class: TriggerClass,
    pub result_class: CleanupResultClass,
    pub class_outcomes: Vec<ClassOutcomeRow>,
    #[serde(default)]
    pub blocked_pin_rows: Vec<BlockedPinRow>,
    pub skipped_protected_class_rows: Vec<SkippedProtectedClassRow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_disk_context: Option<LowDiskContext>,
    pub reopen_inspector_action_ref: String,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub registry_ref: String,
    pub review_schema_ref: String,
    pub reviewer_summary: String,
}

/// One seeded corpus scenario pairing a review with a receipt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCleanupScenario {
    pub schema_version: u32,
    pub record_kind: String,
    pub scenario_id: String,
    pub title: String,
    pub trigger_class: TriggerClass,
    pub summary: String,
    pub review: ClearDataReview,
    pub receipt: StorageCleanupReceipt,
}

/// Corpus entry pairing a fixture path with a parsed scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCleanupCorpusEntry {
    pub fixture_ref: String,
    pub scenario: StorageCleanupScenario,
}

/// Validation violation emitted by the corpus harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCleanupViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

impl fmt::Display for StorageCleanupViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.check_id, self.target_ref, self.message)
    }
}

/// Storage-cleanup corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCleanupCorpus {
    pub registry: StorageClassRegistry,
    pub entries: Vec<StorageCleanupCorpusEntry>,
}

impl StorageCleanupCorpus {
    /// Returns an iterator over the parsed scenarios.
    pub fn scenarios(&self) -> impl Iterator<Item = &StorageCleanupScenario> {
        self.entries.iter().map(|entry| &entry.scenario)
    }

    /// Validates the corpus against the storage-cleanup safety contract.
    pub fn validate(&self) -> Vec<StorageCleanupViolation> {
        let mut violations = Vec::new();
        validate_registry(&mut violations, &self.registry);

        if self.entries.is_empty() {
            push(
                &mut violations,
                "corpus.empty",
                STORAGE_CLEANUP_CORPUS_DIR,
                "corpus must contain at least one scenario",
            );
            return violations;
        }

        let mut fixture_refs = BTreeSet::new();
        let mut scenario_ids = BTreeSet::new();
        let mut triggers_seen = BTreeSet::new();

        for entry in &self.entries {
            if !fixture_refs.insert(entry.fixture_ref.clone()) {
                push(
                    &mut violations,
                    "corpus.duplicate_fixture_ref",
                    &entry.fixture_ref,
                    "fixture_ref must be unique within the corpus",
                );
            }
            let scenario = &entry.scenario;
            if !scenario_ids.insert(scenario.scenario_id.clone()) {
                push(
                    &mut violations,
                    "corpus.duplicate_scenario_id",
                    &scenario.scenario_id,
                    "scenario_id must be unique within the corpus",
                );
            }
            triggers_seen.insert(scenario.trigger_class);
            validate_scenario(&mut violations, scenario, &self.registry);
        }

        for required in REQUIRED_TRIGGER_CLASSES {
            if !triggers_seen.contains(required) {
                push(
                    &mut violations,
                    "corpus.required_trigger_missing",
                    serde_token(*required),
                    format!(
                        "required trigger {} has no seeded scenario",
                        serde_token(*required)
                    ),
                );
            }
        }

        violations
    }

    /// Projects the corpus into a metadata-safe support-export envelope.
    pub fn support_export(
        &self,
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> StorageCleanupSupportExport {
        let mut rows: Vec<StorageCleanupSupportExportRow> = self
            .entries
            .iter()
            .map(StorageCleanupSupportExportRow::from_entry)
            .collect();
        rows.sort_by(|a, b| a.scenario_id.cmp(&b.scenario_id));
        let registry_class_count = self.registry.entries.len() as u32;
        let protected_class_count = self
            .registry
            .entries
            .iter()
            .filter(|entry| entry.protected_default)
            .count() as u32;
        StorageCleanupSupportExport {
            record_kind: STORAGE_CLEANUP_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            schema_version: STORAGE_INSPECTOR_SCHEMA_VERSION,
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: STORAGE_CLEANUP_DOC_REF.to_owned(),
            registry_ref: STORAGE_CLEANUP_REGISTRY_REF.to_owned(),
            registry_schema_ref: STORAGE_CLASS_SCHEMA_REF.to_owned(),
            review_schema_ref: CLEAR_DATA_REVIEW_SCHEMA_REF.to_owned(),
            receipt_schema_ref: STORAGE_CLEANUP_RECEIPT_SCHEMA_REF.to_owned(),
            manifest_ref: STORAGE_CLEANUP_CORPUS_MANIFEST_REF.to_owned(),
            registry_class_count,
            protected_class_count,
            raw_content_exported: false,
            redaction_class: "metadata_safe_default".to_owned(),
            rows,
        }
    }
}

/// One row in the support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCleanupSupportExportRow {
    pub record_kind: String,
    pub fixture_ref: String,
    pub scenario_id: String,
    pub title: String,
    pub trigger_class: TriggerClass,
    pub actor_lineage_class: ActorLineageClass,
    pub result_class: CleanupResultClass,
    pub bytes_reclaimed_total: u64,
    pub selected_class_ids: Vec<String>,
    pub skipped_protected_class_ids: Vec<String>,
    pub blocked_pin_ids: Vec<String>,
    pub low_disk_state: Option<LowDiskStateClass>,
    pub reopen_inspector_action_ref: String,
    pub raw_content_exported: bool,
}

impl StorageCleanupSupportExportRow {
    fn from_entry(entry: &StorageCleanupCorpusEntry) -> Self {
        let scenario = &entry.scenario;
        let receipt = &scenario.receipt;
        let bytes_reclaimed_total = receipt
            .class_outcomes
            .iter()
            .map(|row| row.bytes_reclaimed)
            .sum();
        let mut selected_class_ids: Vec<String> = scenario
            .review
            .selected_class_refs
            .iter()
            .map(|row| row.class_id.as_str().to_owned())
            .collect();
        selected_class_ids.sort();
        selected_class_ids.dedup();
        let mut skipped_protected_class_ids: Vec<String> = receipt
            .skipped_protected_class_rows
            .iter()
            .map(|row| row.class_id.as_str().to_owned())
            .collect();
        skipped_protected_class_ids.sort();
        skipped_protected_class_ids.dedup();
        let mut blocked_pin_ids: Vec<String> = receipt
            .blocked_pin_rows
            .iter()
            .map(|row| row.pin_ref.clone())
            .collect();
        blocked_pin_ids.sort();
        blocked_pin_ids.dedup();
        let low_disk_state = receipt
            .low_disk_context
            .as_ref()
            .map(|ctx| ctx.state_class);
        Self {
            record_kind: STORAGE_CLEANUP_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            fixture_ref: entry.fixture_ref.clone(),
            scenario_id: scenario.scenario_id.clone(),
            title: scenario.title.clone(),
            trigger_class: scenario.trigger_class,
            actor_lineage_class: receipt.actor_lineage_class,
            result_class: receipt.result_class,
            bytes_reclaimed_total,
            selected_class_ids,
            skipped_protected_class_ids,
            blocked_pin_ids,
            low_disk_state,
            reopen_inspector_action_ref: receipt.reopen_inspector_action_ref.clone(),
            raw_content_exported: receipt.raw_content_exported,
        }
    }
}

/// Support-export envelope folded from the storage-cleanup corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StorageCleanupSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub envelope_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub registry_ref: String,
    pub registry_schema_ref: String,
    pub review_schema_ref: String,
    pub receipt_schema_ref: String,
    pub manifest_ref: String,
    pub registry_class_count: u32,
    pub protected_class_count: u32,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub rows: Vec<StorageCleanupSupportExportRow>,
}

impl StorageCleanupSupportExport {
    /// Returns true when the envelope is metadata-safe and vocabulary-aligned.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported
            && self.redaction_class == "metadata_safe_default"
            && self.registry_class_count >= REQUIRED_REGISTRY_CLASSES.len() as u32
            && self.protected_class_count >= REQUIRED_PROTECTED_CLASSES.len() as u32
            && !self.rows.is_empty()
            && self.rows.iter().all(|row| !row.raw_content_exported)
    }
}

/// Strongly typed error returned by the corpus loader.
#[derive(Debug)]
pub enum StorageCleanupLoadError {
    Yaml(serde_yaml::Error),
}

impl fmt::Display for StorageCleanupLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(err) => write!(f, "storage-cleanup yaml parse error: {err}"),
        }
    }
}

impl Error for StorageCleanupLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml(err) => Some(err),
        }
    }
}

impl From<serde_yaml::Error> for StorageCleanupLoadError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

/// Loads the checked-in storage-class registry.
pub fn current_storage_class_registry() -> Result<StorageClassRegistry, StorageCleanupLoadError> {
    serde_yaml::from_str::<StorageClassRegistry>(REGISTRY_YAML).map_err(StorageCleanupLoadError::from)
}

/// Loads the checked-in storage-cleanup corpus (registry + scenarios).
pub fn current_storage_cleanup_corpus() -> Result<StorageCleanupCorpus, StorageCleanupLoadError> {
    let registry = current_storage_class_registry()?;
    let entries = SCENARIO_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<StorageCleanupScenario>(yaml).map(|scenario| {
                StorageCleanupCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    scenario,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(StorageCleanupCorpus { registry, entries })
}

/// Parses one storage-cleanup scenario YAML record.
pub fn load_storage_cleanup_scenario(
    yaml: &str,
) -> Result<StorageCleanupScenario, StorageCleanupLoadError> {
    serde_yaml::from_str::<StorageCleanupScenario>(yaml).map_err(StorageCleanupLoadError::from)
}

fn validate_registry(violations: &mut Vec<StorageCleanupViolation>, registry: &StorageClassRegistry) {
    if registry.schema_version != STORAGE_INSPECTOR_SCHEMA_VERSION {
        push(
            violations,
            "registry.schema_version",
            &registry.registry_id,
            "registry schema_version must be 1",
        );
    }
    if registry.record_kind != STORAGE_CLASS_REGISTRY_RECORD_KIND {
        push(
            violations,
            "registry.record_kind",
            &registry.registry_id,
            "registry record_kind must be storage_class_registry",
        );
    }
    if registry.schema_ref != STORAGE_CLASS_SCHEMA_REF {
        push(
            violations,
            "registry.schema_ref",
            &registry.registry_id,
            "registry schema_ref must pin schemas/support/storage_class.schema.json",
        );
    }
    if registry.doc_ref != STORAGE_CLEANUP_DOC_REF {
        push(
            violations,
            "registry.doc_ref",
            &registry.registry_id,
            "registry doc_ref must pin docs/support/m3/storage_cleanup_beta.md",
        );
    }
    let mut seen_ids: BTreeMap<StorageClassId, ()> = BTreeMap::new();
    let mut seen_priorities: BTreeSet<u32> = BTreeSet::new();
    for entry in &registry.entries {
        let target = entry.class_id.as_str();
        if seen_ids.insert(entry.class_id, ()).is_some() {
            push(
                violations,
                "registry.duplicate_class_id",
                target,
                "registry must list each storage class once",
            );
        }
        if entry.schema_version != STORAGE_INSPECTOR_SCHEMA_VERSION {
            push(violations, "registry.entry.schema_version", target, "schema_version must be 1");
        }
        if entry.record_kind != STORAGE_CLASS_REGISTRY_ENTRY_RECORD_KIND {
            push(
                violations,
                "registry.entry.record_kind",
                target,
                "record_kind must be storage_class_registry_entry",
            );
        }
        if entry.label.trim().is_empty() {
            push(violations, "registry.entry.label", target, "label must be non-empty");
        }
        if entry.summary.trim().is_empty() {
            push(violations, "registry.entry.summary", target, "summary must be non-empty");
        }
        if !(1..=6).contains(&entry.low_disk_eviction_priority) {
            push(
                violations,
                "registry.entry.eviction_priority",
                target,
                "low_disk_eviction_priority must be between 1 and 6",
            );
        }
        if !seen_priorities.insert(entry.low_disk_eviction_priority) {
            push(
                violations,
                "registry.entry.eviction_priority_unique",
                target,
                "low_disk_eviction_priority must be unique within the registry",
            );
        }
        if entry.schema_ref != STORAGE_CLASS_SCHEMA_REF {
            push(
                violations,
                "registry.entry.schema_ref",
                target,
                "entry schema_ref must pin schemas/support/storage_class.schema.json",
            );
        }
        if entry.doc_ref != STORAGE_CLEANUP_DOC_REF {
            push(
                violations,
                "registry.entry.doc_ref",
                target,
                "entry doc_ref must pin docs/support/m3/storage_cleanup_beta.md",
            );
        }
        let pin_default_class = matches!(
            entry.gc_policy_class,
            GcPolicyClass::PinDefault | GcPolicyClass::NeverEvictSilently
        );
        let recovery_or_evidence_authority = matches!(
            entry.authority_class,
            AuthorityClass::UserAuthoredRecovery | AuthorityClass::EvidenceGrade
        );
        if (pin_default_class || recovery_or_evidence_authority) && !entry.protected_default {
            push(
                violations,
                "registry.entry.protected_default",
                target,
                "pin_default/never_evict_silently or user_authored_recovery/evidence_grade entries must be protected_default=true",
            );
        }
    }
    for required in REQUIRED_REGISTRY_CLASSES {
        if !seen_ids.contains_key(required) {
            push(
                violations,
                "registry.required_class_missing",
                required.as_str(),
                format!("registry must declare class {}", required.as_str()),
            );
        }
    }
    for required in REQUIRED_PROTECTED_CLASSES {
        let entry = registry.entry(*required);
        match entry {
            Some(entry) if entry.protected_default => {}
            _ => push(
                violations,
                "registry.required_protected_default",
                required.as_str(),
                format!("class {} must be protected_default=true", required.as_str()),
            ),
        }
    }
}

fn validate_scenario(
    violations: &mut Vec<StorageCleanupViolation>,
    scenario: &StorageCleanupScenario,
    registry: &StorageClassRegistry,
) {
    let target = scenario.scenario_id.as_str();
    if scenario.schema_version != STORAGE_INSPECTOR_SCHEMA_VERSION {
        push(violations, "scenario.schema_version", target, "schema_version must be 1");
    }
    if scenario.record_kind != STORAGE_CLEANUP_SCENARIO_RECORD_KIND {
        push(
            violations,
            "scenario.record_kind",
            target,
            "record_kind must be storage_cleanup_scenario_record",
        );
    }
    if scenario.scenario_id.trim().is_empty() {
        push(violations, "scenario.scenario_id", target, "scenario_id must be non-empty");
    }
    if scenario.title.trim().is_empty() {
        push(violations, "scenario.title", target, "title must be non-empty");
    }
    if scenario.summary.trim().is_empty() {
        push(violations, "scenario.summary", target, "summary must be non-empty");
    }

    validate_review(violations, target, &scenario.review, registry);
    validate_receipt(violations, target, &scenario.receipt, &scenario.review);

    if scenario.trigger_class != scenario.review.trigger_class {
        push(
            violations,
            "scenario.trigger_class.mismatch",
            target,
            "scenario trigger_class must match review.trigger_class",
        );
    }
    if scenario.trigger_class != scenario.receipt.trigger_class {
        push(
            violations,
            "scenario.trigger_class.receipt_mismatch",
            target,
            "scenario trigger_class must match receipt.trigger_class",
        );
    }
}

fn validate_review(
    violations: &mut Vec<StorageCleanupViolation>,
    target: &str,
    review: &ClearDataReview,
    registry: &StorageClassRegistry,
) {
    if review.schema_version != STORAGE_INSPECTOR_SCHEMA_VERSION {
        push(violations, "review.schema_version", target, "review.schema_version must be 1");
    }
    if review.record_kind != CLEAR_DATA_REVIEW_RECORD_KIND {
        push(
            violations,
            "review.record_kind",
            target,
            "review.record_kind must be clear_data_review_record",
        );
    }
    if review.review_id.trim().is_empty() {
        push(violations, "review.review_id", target, "review.review_id must be non-empty");
    }
    if review.raw_content_exported {
        push(
            violations,
            "review.raw_content_exported",
            target,
            "review.raw_content_exported must be false",
        );
    }
    if review.redaction_class != "metadata_safe_default" {
        push(
            violations,
            "review.redaction_class",
            target,
            "review.redaction_class must be metadata_safe_default",
        );
    }
    if review.schema_ref != CLEAR_DATA_REVIEW_SCHEMA_REF {
        push(
            violations,
            "review.schema_ref",
            target,
            "review.schema_ref must pin schemas/support/clear_data_review.schema.json",
        );
    }
    if review.doc_ref != STORAGE_CLEANUP_DOC_REF {
        push(
            violations,
            "review.doc_ref",
            target,
            "review.doc_ref must pin docs/support/m3/storage_cleanup_beta.md",
        );
    }
    if review.storage_class_registry_ref != STORAGE_CLASS_SCHEMA_REF {
        push(
            violations,
            "review.storage_class_registry_ref",
            target,
            "review.storage_class_registry_ref must pin schemas/support/storage_class.schema.json",
        );
    }
    if review.reviewer_summary.trim().is_empty() {
        push(
            violations,
            "review.reviewer_summary",
            target,
            "review.reviewer_summary must be non-empty",
        );
    }
    if review.selected_class_refs.is_empty() {
        push(
            violations,
            "review.selected_class_refs.empty",
            target,
            "review.selected_class_refs must declare at least one class",
        );
    }
    let mut selected_seen: BTreeSet<StorageClassId> = BTreeSet::new();
    for row in &review.selected_class_refs {
        if !selected_seen.insert(row.class_id) {
            push(
                violations,
                "review.selected_class_refs.duplicate",
                target,
                format!(
                    "review.selected_class_refs has duplicate class {}",
                    row.class_id.as_str()
                ),
            );
        }
        if row.consequence_summary.trim().is_empty() {
            push(
                violations,
                "review.selected_class_refs.consequence_summary",
                target,
                format!(
                    "review.selected_class_refs[{}] consequence_summary must be non-empty",
                    row.class_id.as_str()
                ),
            );
        }
        if matches!(
            row.export_before_delete_class,
            ExportBeforeDeleteClass::RequiredBeforeDelete
        ) && row.export_target_ref.is_none()
        {
            push(
                violations,
                "review.selected_class_refs.export_target_required",
                target,
                format!(
                    "review.selected_class_refs[{}] must declare export_target_ref when export_before_delete_class=required_before_delete",
                    row.class_id.as_str()
                ),
            );
        }
    }

    let override_classes: BTreeSet<StorageClassId> = review
        .override_protected_class_refs
        .iter()
        .map(|row| row.class_id)
        .collect();

    for required in REQUIRED_PROTECTED_CLASSES {
        let listed_protected = review
            .protected_class_rows
            .iter()
            .any(|row| row.class_id == *required);
        let listed_override = override_classes.contains(required);
        let listed_selected = review
            .selected_class_refs
            .iter()
            .any(|row| row.class_id == *required);
        // Required protected classes must appear in protected_class_rows
        // OR be the explicit subject of an override.
        if !listed_protected && !listed_override {
            push(
                violations,
                "review.required_protected_class_missing",
                target,
                format!(
                    "review must list protected class {} in protected_class_rows or override_protected_class_refs",
                    required.as_str()
                ),
            );
        }
        if listed_selected && !listed_override {
            push(
                violations,
                "review.protected_class_in_selected_without_override",
                target,
                format!(
                    "review.selected_class_refs contains protected class {} without an override_protected_class_refs entry",
                    required.as_str()
                ),
            );
        }
    }

    for row in &review.override_protected_class_refs {
        if !registry.is_protected_default(row.class_id) {
            push(
                violations,
                "review.override_non_protected",
                target,
                format!(
                    "override_protected_class_refs[{}] is not a protected_default class",
                    row.class_id.as_str()
                ),
            );
        }
        if row.override_justification.trim().is_empty() {
            push(
                violations,
                "review.override_justification_empty",
                target,
                format!(
                    "override_protected_class_refs[{}] override_justification must be non-empty",
                    row.class_id.as_str()
                ),
            );
        }
    }

    if matches!(review.consent_state, ConsentState::Confirmed) && review.confirmed_by_ref.is_none()
    {
        push(
            violations,
            "review.confirmed_by_ref_required",
            target,
            "review.consent_state=confirmed requires a confirmed_by_ref",
        );
    }
}

fn validate_receipt(
    violations: &mut Vec<StorageCleanupViolation>,
    target: &str,
    receipt: &StorageCleanupReceipt,
    review: &ClearDataReview,
) {
    if receipt.schema_version != STORAGE_INSPECTOR_SCHEMA_VERSION {
        push(violations, "receipt.schema_version", target, "receipt.schema_version must be 1");
    }
    if receipt.record_kind != STORAGE_CLEANUP_RECEIPT_RECORD_KIND {
        push(
            violations,
            "receipt.record_kind",
            target,
            "receipt.record_kind must be storage_cleanup_receipt_record",
        );
    }
    if receipt.review_ref != review.review_id {
        push(
            violations,
            "receipt.review_ref",
            target,
            "receipt.review_ref must match review.review_id",
        );
    }
    if receipt.raw_content_exported {
        push(
            violations,
            "receipt.raw_content_exported",
            target,
            "receipt.raw_content_exported must be false",
        );
    }
    if receipt.redaction_class != "metadata_safe_default" {
        push(
            violations,
            "receipt.redaction_class",
            target,
            "receipt.redaction_class must be metadata_safe_default",
        );
    }
    if receipt.schema_ref != STORAGE_CLEANUP_RECEIPT_SCHEMA_REF {
        push(
            violations,
            "receipt.schema_ref",
            target,
            "receipt.schema_ref must pin schemas/support/storage_cleanup_receipt.schema.json",
        );
    }
    if receipt.doc_ref != STORAGE_CLEANUP_DOC_REF {
        push(
            violations,
            "receipt.doc_ref",
            target,
            "receipt.doc_ref must pin docs/support/m3/storage_cleanup_beta.md",
        );
    }
    if receipt.registry_ref != STORAGE_CLASS_SCHEMA_REF {
        push(
            violations,
            "receipt.registry_ref",
            target,
            "receipt.registry_ref must pin schemas/support/storage_class.schema.json",
        );
    }
    if receipt.review_schema_ref != CLEAR_DATA_REVIEW_SCHEMA_REF {
        push(
            violations,
            "receipt.review_schema_ref",
            target,
            "receipt.review_schema_ref must pin schemas/support/clear_data_review.schema.json",
        );
    }
    if receipt.reopen_inspector_action_ref.trim().is_empty() {
        push(
            violations,
            "receipt.reopen_inspector_action_ref",
            target,
            "receipt.reopen_inspector_action_ref must be non-empty",
        );
    }
    if receipt.class_outcomes.is_empty() {
        push(
            violations,
            "receipt.class_outcomes.empty",
            target,
            "receipt.class_outcomes must declare at least one row",
        );
    }
    let mut outcome_classes: BTreeSet<StorageClassId> = BTreeSet::new();
    for row in &receipt.class_outcomes {
        if !outcome_classes.insert(row.class_id) {
            push(
                violations,
                "receipt.class_outcomes.duplicate",
                target,
                format!(
                    "receipt.class_outcomes has duplicate class {}",
                    row.class_id.as_str()
                ),
            );
        }
    }

    if receipt.skipped_protected_class_rows.is_empty() {
        push(
            violations,
            "receipt.skipped_protected_class_rows.empty",
            target,
            "receipt.skipped_protected_class_rows must name every protected class skipped",
        );
    }
    let skipped_classes: BTreeSet<StorageClassId> = receipt
        .skipped_protected_class_rows
        .iter()
        .map(|row| row.class_id)
        .collect();
    let override_classes: BTreeSet<StorageClassId> = review
        .override_protected_class_refs
        .iter()
        .map(|row| row.class_id)
        .collect();
    for required in REQUIRED_PROTECTED_CLASSES {
        if !skipped_classes.contains(required) && !override_classes.contains(required) {
            push(
                violations,
                "receipt.required_protected_class_missing",
                target,
                format!(
                    "receipt must name protected class {} in skipped_protected_class_rows or pair with a review override",
                    required.as_str()
                ),
            );
        }
    }

    if matches!(receipt.trigger_class, TriggerClass::LowDiskPressure)
        && receipt.low_disk_context.is_none()
    {
        push(
            violations,
            "receipt.low_disk_context_required",
            target,
            "receipt.trigger_class=low_disk_pressure requires a populated low_disk_context",
        );
    }
    if let Some(ctx) = &receipt.low_disk_context {
        if ctx.ordered_eviction_steps.is_empty() {
            push(
                violations,
                "receipt.low_disk_context.ordered_eviction_steps_empty",
                target,
                "low_disk_context.ordered_eviction_steps must declare at least one step",
            );
        }
        let mut prev_order = 0_u32;
        for step in &ctx.ordered_eviction_steps {
            if step.order <= prev_order {
                push(
                    violations,
                    "receipt.low_disk_context.ordered_eviction_steps_order",
                    target,
                    format!(
                        "ordered_eviction_steps must be strictly increasing by order; saw {} after {}",
                        step.order, prev_order
                    ),
                );
            }
            prev_order = step.order;
        }
    }

    if matches!(receipt.result_class, CleanupResultClass::Cancelled) {
        for row in &receipt.class_outcomes {
            if row.bytes_reclaimed != 0 {
                push(
                    violations,
                    "receipt.cancelled_must_be_zero_bytes",
                    target,
                    format!(
                        "cancelled receipt must not reclaim bytes; saw {} bytes for {}",
                        row.bytes_reclaimed,
                        row.class_id.as_str()
                    ),
                );
            }
        }
    }

    if matches!(receipt.result_class, CleanupResultClass::Partial)
        && receipt.blocked_pin_rows.is_empty()
        && receipt
            .class_outcomes
            .iter()
            .all(|row| row.bytes_reclaimed > 0)
    {
        // partial without blocked pins must indicate something is incomplete
        // — we permit it when the partial flag is intentional, but flag a
        // result_class mismatch only if every outcome looks complete and
        // nothing is blocked.
        push(
            violations,
            "receipt.partial_without_block_or_incomplete",
            target,
            "result_class=partial requires either blocked pins or an incomplete outcome",
        );
    }
}

fn push(
    violations: &mut Vec<StorageCleanupViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(StorageCleanupViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}

fn serde_token(trigger: TriggerClass) -> &'static str {
    match trigger {
        TriggerClass::UserRequestedCleanup => "user_requested_cleanup",
        TriggerClass::LowDiskPressure => "low_disk_pressure",
        TriggerClass::QuotaPressure => "quota_pressure",
        TriggerClass::CorruptionRepairRequest => "corruption_repair_request",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_parses_and_validates() {
        let registry = current_storage_class_registry().expect("registry parses");
        assert_eq!(
            registry.entries.len(),
            REQUIRED_REGISTRY_CLASSES.len(),
            "registry must list every required class"
        );
        for required in REQUIRED_REGISTRY_CLASSES {
            assert!(registry.entry(*required).is_some(), "missing {}", required.as_str());
        }
        for required in REQUIRED_PROTECTED_CLASSES {
            assert!(registry.is_protected_default(*required));
        }
    }

    #[test]
    fn corpus_validates_against_the_safety_contract() {
        let corpus = current_storage_cleanup_corpus().expect("corpus parses");
        let violations = corpus.validate();
        assert_eq!(violations, Vec::new(), "{violations:#?}");
    }

    #[test]
    fn support_export_is_metadata_safe() {
        let corpus = current_storage_cleanup_corpus().expect("corpus parses");
        let export = corpus.support_export(
            "support_export.storage_cleanup.v1",
            "2026-05-19T15:30:00Z",
        );
        assert!(export.is_export_safe());
        assert_eq!(export.rows.len(), SCENARIO_FIXTURES.len());
    }
}
