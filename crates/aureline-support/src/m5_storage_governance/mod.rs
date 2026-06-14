//! Frozen storage-class, pin-source, clear-data, and low-disk ordering matrix
//! for the heavy artifact families the M5 depth lanes add.
//!
//! M5 lands notebook outputs, profiler traces, replay bundles, docs/model/
//! template packs, generated previews, extension downloads, prebuild layers,
//! support artifacts, and review/incident evidence — plus it touches the
//! user-owned recovery state earlier milestones own. Those lanes are not
//! complete until the shell can explain, per family, what is disposable, what
//! is rebuildable, what is durable evidence, what is user-owned recovery
//! state, which pins protect it, what a clear-data action may do to it, and
//! where it sits in the low-disk eviction order.
//!
//! This module folds the checked-in matrix at
//! [`M5_ARTIFACT_FAMILY_MATRIX_REF`] into typed records and validates every
//! row against the canonical runtime storage-class contract at
//! [`RUNTIME_STORAGE_CLASSES_REF`]. The matrix mints no new storage primitive:
//! `storage_class_id`, `authority_class`, `rebuild_cost_class`,
//! `gc_policy_class`, `pin_source_class`, `clear_cache_protection_class`, and
//! `low_disk_ladder_step` re-export verbatim from the runtime artifact. Only
//! the [`DefaultRetentionClass`] and [`ClearDataActionClass`] vocabularies are
//! introduced here, and they are bounded explanatory labels.
//!
//! ## What this owns
//!
//! - The [`M5ArtifactFamilyStorageMatrix`] record — one row per M5 heavy
//!   artifact family, mapping it to a frozen storage class plus its authority,
//!   rebuild cost, default retention, pin sources, allowed clear-data actions,
//!   and low-disk ladder step. Mirrors the boundary schema at
//!   [`M5_ARTIFACT_FAMILY_MATRIX_SCHEMA_REF`].
//! - The [`RuntimeStorageClassProfiles`] projection — the admissibility table
//!   parsed from the runtime artifact, against which each row is checked so a
//!   new family can never invent private cleanup semantics outside the shared
//!   registry.
//! - The cross-surface projections every consumer reuses instead of minting a
//!   parallel cleanup vocabulary: [`M5ArtifactFamilyStorageMatrix::low_disk_eviction_order`]
//!   (low-disk banner ordering), [`M5ArtifactFamilyStorageMatrix::clear_data_plan_for`]
//!   (clear-data review), [`M5ArtifactFamilyStorageMatrix::offboarding_reset_plan`]
//!   (offboarding/reset), and [`M5ArtifactFamilyStorageMatrix::support_export`]
//!   (metadata-safe support/export packets).
//!
//! ## What this does NOT own
//!
//! - Live byte-level garbage collection, eviction scheduling, or quota
//!   enforcement. Those belong to the runtime crates; this module is the
//!   shared truth table they and the inspectable surfaces project.
//! - The runtime storage-class vocabulary itself, which stays frozen in
//!   `artifacts/runtime/storage_classes.yaml`.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::storage_inspector::StorageClassId;

#[cfg(test)]
mod tests;

/// Frozen schema version shared by every record in this module.
pub const M5_STORAGE_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for one artifact-family matrix row.
pub const M5_ARTIFACT_FAMILY_ROW_RECORD_KIND: &str = "m5_artifact_family_storage_row";

/// Stable record-kind tag for the matrix container.
pub const M5_ARTIFACT_FAMILY_MATRIX_RECORD_KIND: &str = "m5_artifact_family_storage_matrix";

/// Stable record-kind tag for the support-export envelope.
pub const M5_STORAGE_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "m5_storage_governance_support_export";

/// Stable record-kind tag for one support-export row.
pub const M5_STORAGE_GOVERNANCE_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "m5_storage_governance_support_export_row";

/// Repository-relative path of the checked-in matrix artifact.
pub const M5_ARTIFACT_FAMILY_MATRIX_REF: &str =
    "artifacts/storage/m5_artifact_family_storage_matrix.yaml";

/// Repository-relative path of the boundary schema for the matrix.
pub const M5_ARTIFACT_FAMILY_MATRIX_SCHEMA_REF: &str =
    "schemas/storage/m5_artifact_family_storage_matrix.schema.json";

/// Repository-relative path of the reviewer doc every row quotes.
pub const M5_ARTIFACT_FAMILY_MATRIX_DOC_REF: &str =
    "docs/m5/freeze-the-m5-storage-class-pin-source-clear-data-and-low-disk-ordering-matrix-for-new-artifact-families.md";

/// Repository-relative path of the canonical runtime storage-class contract.
pub const RUNTIME_STORAGE_CLASSES_REF: &str = "artifacts/runtime/storage_classes.yaml";

const MATRIX_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/storage/m5_artifact_family_storage_matrix.yaml"
));

const RUNTIME_STORAGE_CLASSES_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/runtime/storage_classes.yaml"
));

/// The closed set of M5 artifact families the matrix must cover exactly once.
const REQUIRED_FAMILIES: &[ArtifactFamilyId] = &[
    ArtifactFamilyId::GeneratedPreview,
    ArtifactFamilyId::NotebookOutput,
    ArtifactFamilyId::DocsPack,
    ArtifactFamilyId::ModelPack,
    ArtifactFamilyId::TemplatePack,
    ArtifactFamilyId::ExtensionDownload,
    ArtifactFamilyId::PrebuildLayer,
    ArtifactFamilyId::ProfilerTrace,
    ArtifactFamilyId::ReplayBundle,
    ArtifactFamilyId::SupportArtifact,
    ArtifactFamilyId::ReviewIncidentEvidence,
    ArtifactFamilyId::UserOwnedRecoveryState,
];

// --------------------------------------------------------------------------
// Closed vocabularies.
//
// storage_class_id is reused from `storage_inspector`. authority_class,
// rebuild_cost_class, gc_policy_class, clear_cache_protection_class,
// low_disk_ladder_step, and pin_source_class re-export the runtime tokens by
// value. default_retention_class and clear_data_action_class are introduced
// here as bounded explanatory labels.
// --------------------------------------------------------------------------

/// Closed M5 heavy-artifact-family vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamilyId {
    /// Rendered previews regenerated on demand from authoritative source.
    GeneratedPreview,
    /// Cached notebook cell outputs, rebuildable by re-execution.
    NotebookOutput,
    /// Downloaded documentation packs retained by digest.
    DocsPack,
    /// Downloaded model packs and weights retained by digest.
    ModelPack,
    /// Downloaded scaffolding template / archetype packs retained by digest.
    TemplatePack,
    /// Downloaded extension packages retained by digest.
    ExtensionDownload,
    /// Container layers, toolchain packs, and environment capsules.
    PrebuildLayer,
    /// Captured profiler traces of a specific run.
    ProfilerTrace,
    /// Recorded trace/replay bundles backing regression and incident analysis.
    ReplayBundle,
    /// Support-bundle drafts and in-flight support-export assemblies.
    SupportArtifact,
    /// Review packets, validation artifacts, and incident-workspace evidence.
    ReviewIncidentEvidence,
    /// Local history, checkpoints, and session-restore state M5 lanes touch.
    UserOwnedRecoveryState,
}

impl ArtifactFamilyId {
    /// Stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratedPreview => "generated_preview",
            Self::NotebookOutput => "notebook_output",
            Self::DocsPack => "docs_pack",
            Self::ModelPack => "model_pack",
            Self::TemplatePack => "template_pack",
            Self::ExtensionDownload => "extension_download",
            Self::PrebuildLayer => "prebuild_layer",
            Self::ProfilerTrace => "profiler_trace",
            Self::ReplayBundle => "replay_bundle",
            Self::SupportArtifact => "support_artifact",
            Self::ReviewIncidentEvidence => "review_incident_evidence",
            Self::UserOwnedRecoveryState => "user_owned_recovery_state",
        }
    }
}

/// Authority posture re-exported from the runtime storage-class contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityClass {
    UserAuthoredDurableTruth,
    UserOwnedRecoveryState,
    AdminOrControlArtifact,
    DisposableDerivedCache,
}

/// Rebuild-cost posture re-exported from the runtime storage-class contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RebuildCostClass {
    AuthoritativeNoRebuild,
    HighRebuildCost,
    MediumRebuildCost,
    LowRebuildCost,
}

/// Garbage-collection policy re-exported from the runtime contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GcPolicyClass {
    NeverGcAuthoritative,
    GcOnVersionReplace,
    GcOnPressureIfUnpinned,
    GcOnCaseClose,
    GcOnExplicitResetOnly,
}

/// Clear-cache protection posture re-exported from the runtime contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearCacheProtectionClass {
    GenericClearAlwaysAllowed,
    GenericClearWithPinExclusions,
    ProtectedRequiresClassSpecificReview,
    ProtectedNeverGenericClear,
}

/// Low-disk ladder step re-exported from the runtime contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LowDiskLadderStep {
    StopSpeculativeFetchAndPrefetch,
    PauseManagedReplicationAndPackRefresh,
    TrimInteractiveHotCache,
    TrimKnowledgeCacheRebuildable,
    TrimArtifactCacheUnpinned,
    TrimPrebuildEnvironmentUnpinned,
    ExpireUnpinnedEvidencePastRetention,
    UserOwnedRecoveryStateOnlyUnderExplicitReview,
}

impl LowDiskLadderStep {
    /// 1-based position in the canonical low-disk ladder (early → late).
    pub const fn ladder_order(self) -> u32 {
        match self {
            Self::StopSpeculativeFetchAndPrefetch => 1,
            Self::PauseManagedReplicationAndPackRefresh => 2,
            Self::TrimInteractiveHotCache => 3,
            Self::TrimKnowledgeCacheRebuildable => 4,
            Self::TrimArtifactCacheUnpinned => 5,
            Self::TrimPrebuildEnvironmentUnpinned => 6,
            Self::ExpireUnpinnedEvidencePastRetention => 7,
            Self::UserOwnedRecoveryStateOnlyUnderExplicitReview => 8,
        }
    }
}

/// Pin-source vocabulary re-exported from the runtime contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PinSourceClass {
    ExplicitUserPin,
    ExplicitAdminPolicyPin,
    ReleaseArtifactGraphRef,
    CaseReferenceRef,
    ReviewPackRef,
    OfflineBundleRef,
    CertifiedArchetypeOrTemplateRef,
    PolicyBundleLastKnownGoodRef,
    SupportExportAssemblyRef,
    RetentionWindowRef,
}

/// Default-retention posture introduced by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultRetentionClass {
    /// Dropped at the end of the session; pure ephemeral output.
    EvictOnSessionEnd,
    /// Kept until disk pressure trims it, unless a pin protects it.
    EvictUnderPressureIfUnpinned,
    /// Kept until a newer version replaces it.
    RetainUntilVersionReplace,
    /// Kept for a policy/case retention window.
    RetainForPolicyWindow,
    /// Kept until the user explicitly resets it.
    RetainUntilExplicitUserReset,
}

/// One allowed clear-data action introduced by this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClearDataActionClass {
    /// Bulk clear of an always-disposable class; no entry survives.
    GenericClearInBulk,
    /// Bulk clear that skips every pinned entry.
    GenericClearExcludingPins,
    /// Class-scoped clear chosen from the class-selective review sheet.
    ClassSelectiveClear,
    /// Removal only through a class-specific review flow.
    ClassSpecificReviewRequired,
    /// Removal only through an explicit per-item review.
    ExplicitPerItemReviewRequired,
    /// Export-before-delete is offered before any removal.
    ExportBeforeDeleteOffered,
}

// --------------------------------------------------------------------------
// Matrix records.
// --------------------------------------------------------------------------

/// One artifact-family row in the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactFamilyRow {
    pub schema_version: u32,
    pub record_kind: String,
    pub family_id: ArtifactFamilyId,
    pub label: String,
    pub summary: String,
    pub storage_class_id: StorageClassId,
    pub authority_class: AuthorityClass,
    pub rebuild_cost_class: RebuildCostClass,
    pub gc_policy_class: GcPolicyClass,
    pub default_retention_class: DefaultRetentionClass,
    pub clear_cache_protection_class: ClearCacheProtectionClass,
    pub low_disk_ladder_step: LowDiskLadderStep,
    #[serde(default)]
    pub pin_source_classes: Vec<PinSourceClass>,
    pub allowed_clear_data_actions: Vec<ClearDataActionClass>,
    pub export_before_delete_required: bool,
    pub protected_continuity: bool,
    pub schema_ref: String,
    pub doc_ref: String,
}

/// The frozen M5 artifact-family storage matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ArtifactFamilyStorageMatrix {
    pub schema_version: u32,
    pub record_kind: String,
    pub matrix_id: String,
    pub title: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub runtime_storage_classes_ref: String,
    pub emitted_at: String,
    pub rows: Vec<M5ArtifactFamilyRow>,
}

impl M5ArtifactFamilyStorageMatrix {
    /// Returns the row for the given family, if present.
    pub fn family(&self, family_id: ArtifactFamilyId) -> Option<&M5ArtifactFamilyRow> {
        self.rows.iter().find(|row| row.family_id == family_id)
    }

    /// Returns the closed list of families the matrix must cover.
    pub fn required_families() -> &'static [ArtifactFamilyId] {
        REQUIRED_FAMILIES
    }

    /// Projects the low-disk eviction order a low-disk banner renders: every
    /// family sorted early → late by its ladder step, with protected classes
    /// trailing. Ties break by family id for a stable, inspectable order.
    pub fn low_disk_eviction_order(&self) -> Vec<&M5ArtifactFamilyRow> {
        let mut rows: Vec<&M5ArtifactFamilyRow> = self.rows.iter().collect();
        rows.sort_by(|a, b| {
            a.low_disk_ladder_step
                .ladder_order()
                .cmp(&b.low_disk_ladder_step.ladder_order())
                .then_with(|| a.family_id.cmp(&b.family_id))
        });
        rows
    }

    /// Projects the clear-data plan a class-selective review sheet renders for
    /// one family: its protection posture, the actions a review may take, the
    /// pins it preserves, and whether export-before-delete is required.
    pub fn clear_data_plan_for(&self, family_id: ArtifactFamilyId) -> Option<ClearDataPlan> {
        self.family(family_id).map(|row| ClearDataPlan {
            family_id,
            storage_class_id: row.storage_class_id,
            clear_cache_protection_class: row.clear_cache_protection_class,
            allowed_clear_data_actions: row.allowed_clear_data_actions.clone(),
            preserved_pin_source_classes: row.pin_source_classes.clone(),
            export_before_delete_required: row.export_before_delete_required,
            protected_continuity: row.protected_continuity,
        })
    }

    /// Projects the offboarding/reset plan: which families a reset disposes of
    /// freely versus which require an export-before-delete review first. No
    /// protected family is silently disposed.
    pub fn offboarding_reset_plan(&self) -> OffboardingResetPlan {
        let mut disposed_without_review = Vec::new();
        let mut export_before_delete = Vec::new();
        for row in &self.rows {
            if row.export_before_delete_required || row.protected_continuity {
                export_before_delete.push(row.family_id);
            } else {
                disposed_without_review.push(row.family_id);
            }
        }
        disposed_without_review.sort();
        export_before_delete.sort();
        OffboardingResetPlan {
            disposed_without_review,
            export_before_delete,
        }
    }

    /// Validates every row against the canonical runtime storage-class
    /// profiles plus the matrix-level coverage and consistency contract.
    pub fn validate(
        &self,
        profiles: &RuntimeStorageClassProfiles,
    ) -> Vec<M5StorageGovernanceViolation> {
        let mut violations = Vec::new();

        if self.schema_version != M5_STORAGE_GOVERNANCE_SCHEMA_VERSION {
            push(
                &mut violations,
                "matrix.schema_version",
                &self.matrix_id,
                "schema_version must be 1",
            );
        }
        if self.record_kind != M5_ARTIFACT_FAMILY_MATRIX_RECORD_KIND {
            push(
                &mut violations,
                "matrix.record_kind",
                &self.matrix_id,
                "record_kind must be m5_artifact_family_storage_matrix",
            );
        }
        if self.schema_ref != M5_ARTIFACT_FAMILY_MATRIX_SCHEMA_REF {
            push(
                &mut violations,
                "matrix.schema_ref",
                &self.matrix_id,
                "schema_ref must pin the matrix boundary schema",
            );
        }
        if self.doc_ref != M5_ARTIFACT_FAMILY_MATRIX_DOC_REF {
            push(
                &mut violations,
                "matrix.doc_ref",
                &self.matrix_id,
                "doc_ref must pin the matrix reviewer doc",
            );
        }
        if self.runtime_storage_classes_ref != RUNTIME_STORAGE_CLASSES_REF {
            push(
                &mut violations,
                "matrix.runtime_ref",
                &self.matrix_id,
                "runtime_storage_classes_ref must pin artifacts/runtime/storage_classes.yaml",
            );
        }

        let mut seen: BTreeSet<ArtifactFamilyId> = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.family_id) {
                push(
                    &mut violations,
                    "matrix.duplicate_family",
                    row.family_id.as_str(),
                    "each family must appear exactly once",
                );
            }
            validate_row(&mut violations, row, profiles);
        }
        for required in REQUIRED_FAMILIES {
            if !seen.contains(required) {
                push(
                    &mut violations,
                    "matrix.required_family_missing",
                    required.as_str(),
                    format!("matrix must declare family {}", required.as_str()),
                );
            }
        }

        violations
    }

    /// Projects the matrix into a metadata-safe support/export envelope the
    /// support-bundle pipeline can quote without leaking raw payloads.
    pub fn support_export(
        &self,
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
    ) -> M5StorageGovernanceSupportExport {
        let mut rows: Vec<M5StorageGovernanceSupportExportRow> = self
            .rows
            .iter()
            .map(M5StorageGovernanceSupportExportRow::from_row)
            .collect();
        rows.sort_by(|a, b| a.family_id.cmp(&b.family_id));
        let protected_family_count = self
            .rows
            .iter()
            .filter(|row| row.protected_continuity)
            .count() as u32;
        M5StorageGovernanceSupportExport {
            record_kind: M5_STORAGE_GOVERNANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_STORAGE_GOVERNANCE_SCHEMA_VERSION,
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            matrix_id: self.matrix_id.clone(),
            matrix_ref: M5_ARTIFACT_FAMILY_MATRIX_REF.to_owned(),
            schema_ref: M5_ARTIFACT_FAMILY_MATRIX_SCHEMA_REF.to_owned(),
            doc_ref: M5_ARTIFACT_FAMILY_MATRIX_DOC_REF.to_owned(),
            runtime_storage_classes_ref: RUNTIME_STORAGE_CLASSES_REF.to_owned(),
            family_count: self.rows.len() as u32,
            protected_family_count,
            raw_content_exported: false,
            redaction_class: "metadata_safe_default".to_owned(),
            rows,
        }
    }
}

/// Clear-data plan projected for one family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClearDataPlan {
    pub family_id: ArtifactFamilyId,
    pub storage_class_id: StorageClassId,
    pub clear_cache_protection_class: ClearCacheProtectionClass,
    pub allowed_clear_data_actions: Vec<ClearDataActionClass>,
    pub preserved_pin_source_classes: Vec<PinSourceClass>,
    pub export_before_delete_required: bool,
    pub protected_continuity: bool,
}

/// Offboarding/reset plan projected from the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingResetPlan {
    /// Families a reset disposes of without a class-specific review.
    pub disposed_without_review: Vec<ArtifactFamilyId>,
    /// Families a reset must offer export-before-delete for before removal.
    pub export_before_delete: Vec<ArtifactFamilyId>,
}

// --------------------------------------------------------------------------
// Support-export projection.
// --------------------------------------------------------------------------

/// One row in the support-export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StorageGovernanceSupportExportRow {
    pub record_kind: String,
    pub family_id: ArtifactFamilyId,
    pub label: String,
    pub storage_class_id: StorageClassId,
    pub authority_class: AuthorityClass,
    pub rebuild_cost_class: RebuildCostClass,
    pub gc_policy_class: GcPolicyClass,
    pub default_retention_class: DefaultRetentionClass,
    pub clear_cache_protection_class: ClearCacheProtectionClass,
    pub low_disk_ladder_step: LowDiskLadderStep,
    pub low_disk_ladder_order: u32,
    pub pin_source_class_count: u32,
    pub allowed_clear_data_actions: Vec<ClearDataActionClass>,
    pub export_before_delete_required: bool,
    pub protected_continuity: bool,
}

impl M5StorageGovernanceSupportExportRow {
    fn from_row(row: &M5ArtifactFamilyRow) -> Self {
        Self {
            record_kind: M5_STORAGE_GOVERNANCE_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            family_id: row.family_id,
            label: row.label.clone(),
            storage_class_id: row.storage_class_id,
            authority_class: row.authority_class,
            rebuild_cost_class: row.rebuild_cost_class,
            gc_policy_class: row.gc_policy_class,
            default_retention_class: row.default_retention_class,
            clear_cache_protection_class: row.clear_cache_protection_class,
            low_disk_ladder_step: row.low_disk_ladder_step,
            low_disk_ladder_order: row.low_disk_ladder_step.ladder_order(),
            pin_source_class_count: row.pin_source_classes.len() as u32,
            allowed_clear_data_actions: row.allowed_clear_data_actions.clone(),
            export_before_delete_required: row.export_before_delete_required,
            protected_continuity: row.protected_continuity,
        }
    }
}

/// Support-export envelope folded from the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StorageGovernanceSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub envelope_id: String,
    pub captured_at: String,
    pub matrix_id: String,
    pub matrix_ref: String,
    pub schema_ref: String,
    pub doc_ref: String,
    pub runtime_storage_classes_ref: String,
    pub family_count: u32,
    pub protected_family_count: u32,
    pub raw_content_exported: bool,
    pub redaction_class: String,
    pub rows: Vec<M5StorageGovernanceSupportExportRow>,
}

impl M5StorageGovernanceSupportExport {
    /// Returns true when the envelope is metadata-safe and family-complete.
    pub fn is_export_safe(&self) -> bool {
        !self.raw_content_exported
            && self.redaction_class == "metadata_safe_default"
            && self.family_count >= REQUIRED_FAMILIES.len() as u32
            && self.rows.len() as u32 == self.family_count
    }
}

// --------------------------------------------------------------------------
// Runtime storage-class admissibility profiles.
// --------------------------------------------------------------------------

/// Admissibility profile for one runtime storage class, parsed from the
/// canonical runtime artifact.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RuntimeStorageClassProfile {
    pub id: StorageClassId,
    #[serde(default)]
    pub admissible_authority_classes: Vec<AuthorityClass>,
    #[serde(default)]
    pub admissible_rebuild_cost_classes: Vec<RebuildCostClass>,
    #[serde(default)]
    pub admissible_gc_policy_classes: Vec<GcPolicyClass>,
    #[serde(default)]
    pub admissible_pin_source_classes: Vec<PinSourceClass>,
    pub clear_cache_protection_class: ClearCacheProtectionClass,
    pub low_disk_ladder_step: LowDiskLadderStep,
    #[serde(default)]
    pub export_before_delete_required: bool,
}

/// Lookup over the runtime storage-class admissibility profiles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeStorageClassProfiles {
    by_class: BTreeMap<StorageClassId, RuntimeStorageClassProfile>,
}

impl RuntimeStorageClassProfiles {
    /// Returns the profile for the given class, if present.
    pub fn get(&self, class_id: StorageClassId) -> Option<&RuntimeStorageClassProfile> {
        self.by_class.get(&class_id)
    }
}

#[derive(Debug, Deserialize)]
struct RuntimeStorageClassesDoc {
    storage_classes: Vec<RuntimeStorageClassProfile>,
}

// --------------------------------------------------------------------------
// Loaders.
// --------------------------------------------------------------------------

/// Strongly typed error returned by the loaders.
#[derive(Debug)]
pub enum M5StorageGovernanceLoadError {
    Yaml(serde_yaml::Error),
}

impl fmt::Display for M5StorageGovernanceLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Yaml(err) => write!(f, "m5 storage-governance yaml parse error: {err}"),
        }
    }
}

impl Error for M5StorageGovernanceLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Yaml(err) => Some(err),
        }
    }
}

impl From<serde_yaml::Error> for M5StorageGovernanceLoadError {
    fn from(value: serde_yaml::Error) -> Self {
        Self::Yaml(value)
    }
}

/// Loads the checked-in M5 artifact-family storage matrix.
pub fn current_m5_artifact_family_storage_matrix(
) -> Result<M5ArtifactFamilyStorageMatrix, M5StorageGovernanceLoadError> {
    serde_yaml::from_str::<M5ArtifactFamilyStorageMatrix>(MATRIX_YAML)
        .map_err(M5StorageGovernanceLoadError::from)
}

/// Loads the canonical runtime storage-class admissibility profiles.
pub fn current_runtime_storage_class_profiles(
) -> Result<RuntimeStorageClassProfiles, M5StorageGovernanceLoadError> {
    let doc = serde_yaml::from_str::<RuntimeStorageClassesDoc>(RUNTIME_STORAGE_CLASSES_YAML)
        .map_err(M5StorageGovernanceLoadError::from)?;
    let by_class = doc
        .storage_classes
        .into_iter()
        .map(|profile| (profile.id, profile))
        .collect();
    Ok(RuntimeStorageClassProfiles { by_class })
}

// --------------------------------------------------------------------------
// Validation.
// --------------------------------------------------------------------------

/// A validation violation surfaced by the matrix harness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StorageGovernanceViolation {
    pub check_id: String,
    pub target_ref: String,
    pub message: String,
}

impl fmt::Display for M5StorageGovernanceViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.check_id, self.target_ref, self.message
        )
    }
}

fn validate_row(
    violations: &mut Vec<M5StorageGovernanceViolation>,
    row: &M5ArtifactFamilyRow,
    profiles: &RuntimeStorageClassProfiles,
) {
    let target = row.family_id.as_str();

    if row.schema_version != M5_STORAGE_GOVERNANCE_SCHEMA_VERSION {
        push(
            violations,
            "row.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if row.record_kind != M5_ARTIFACT_FAMILY_ROW_RECORD_KIND {
        push(
            violations,
            "row.record_kind",
            target,
            "record_kind must be m5_artifact_family_storage_row",
        );
    }
    if row.label.trim().is_empty() {
        push(violations, "row.label", target, "label must be non-empty");
    }
    if row.summary.trim().is_empty() {
        push(
            violations,
            "row.summary",
            target,
            "summary must be non-empty",
        );
    }
    if row.schema_ref != M5_ARTIFACT_FAMILY_MATRIX_SCHEMA_REF {
        push(
            violations,
            "row.schema_ref",
            target,
            "schema_ref must pin the matrix boundary schema",
        );
    }
    if row.doc_ref != M5_ARTIFACT_FAMILY_MATRIX_DOC_REF {
        push(
            violations,
            "row.doc_ref",
            target,
            "doc_ref must pin the matrix reviewer doc",
        );
    }
    if row.allowed_clear_data_actions.is_empty() {
        push(
            violations,
            "row.allowed_clear_data_actions.empty",
            target,
            "allowed_clear_data_actions must declare at least one action",
        );
    }

    let Some(profile) = profiles.get(row.storage_class_id) else {
        push(
            violations,
            "row.unknown_storage_class",
            target,
            format!(
                "storage_class_id {} has no runtime profile",
                row.storage_class_id.as_str()
            ),
        );
        return;
    };

    // Every declared posture MUST be admissible under the canonical runtime
    // row for the family's storage class — no private cleanup semantics.
    if !profile
        .admissible_authority_classes
        .contains(&row.authority_class)
    {
        push(
            violations,
            "row.authority_not_admissible",
            target,
            "authority_class is not admissible under the runtime storage class",
        );
    }
    if !profile
        .admissible_rebuild_cost_classes
        .contains(&row.rebuild_cost_class)
    {
        push(
            violations,
            "row.rebuild_cost_not_admissible",
            target,
            "rebuild_cost_class is not admissible under the runtime storage class",
        );
    }
    if !profile
        .admissible_gc_policy_classes
        .contains(&row.gc_policy_class)
    {
        push(
            violations,
            "row.gc_policy_not_admissible",
            target,
            "gc_policy_class is not admissible under the runtime storage class",
        );
    }
    for pin in &row.pin_source_classes {
        if !profile.admissible_pin_source_classes.contains(pin) {
            push(
                violations,
                "row.pin_source_not_admissible",
                target,
                "a pin_source_class is not admissible under the runtime storage class",
            );
        }
    }
    if row.clear_cache_protection_class != profile.clear_cache_protection_class {
        push(
            violations,
            "row.clear_protection_mismatch",
            target,
            "clear_cache_protection_class must equal the runtime storage class",
        );
    }
    if row.low_disk_ladder_step != profile.low_disk_ladder_step {
        push(
            violations,
            "row.ladder_step_mismatch",
            target,
            "low_disk_ladder_step must equal the runtime storage class",
        );
    }
    if row.export_before_delete_required != profile.export_before_delete_required {
        push(
            violations,
            "row.export_before_delete_mismatch",
            target,
            "export_before_delete_required must equal the runtime storage class",
        );
    }

    // Protected-continuity tracks the protected storage classes exactly.
    let protected_class = matches!(
        row.storage_class_id,
        StorageClassId::EvidenceSupportCache | StorageClassId::UserOwnedRecoveryState
    );
    if row.protected_continuity != protected_class {
        push(violations, "row.protected_continuity_mismatch", target, "protected_continuity must be true exactly for evidence and user-owned recovery classes");
    }

    validate_clear_actions(violations, row);
}

/// Enforces the clear-data action contract against the protection posture so
/// no protected class can be erased by a generic bulk clear.
fn validate_clear_actions(
    violations: &mut Vec<M5StorageGovernanceViolation>,
    row: &M5ArtifactFamilyRow,
) {
    let target = row.family_id.as_str();
    let actions: BTreeSet<ClearDataActionClass> =
        row.allowed_clear_data_actions.iter().copied().collect();
    let has = |action: ClearDataActionClass| actions.contains(&action);
    let any_generic = has(ClearDataActionClass::GenericClearInBulk)
        || has(ClearDataActionClass::GenericClearExcludingPins)
        || has(ClearDataActionClass::ClassSelectiveClear);

    match row.clear_cache_protection_class {
        ClearCacheProtectionClass::GenericClearAlwaysAllowed => {
            if !has(ClearDataActionClass::GenericClearInBulk) {
                push(
                    violations,
                    "row.clear_actions.generic_required",
                    target,
                    "generic_clear_always_allowed must allow generic_clear_in_bulk",
                );
            }
            if has(ClearDataActionClass::ClassSpecificReviewRequired)
                || has(ClearDataActionClass::ExplicitPerItemReviewRequired)
            {
                push(
                    violations,
                    "row.clear_actions.unexpected_review",
                    target,
                    "generic_clear_always_allowed must not require a review action",
                );
            }
            if !row.pin_source_classes.is_empty() {
                push(
                    violations,
                    "row.clear_actions.pins_on_always_clear",
                    target,
                    "generic_clear_always_allowed admits no pin sources",
                );
            }
        }
        ClearCacheProtectionClass::GenericClearWithPinExclusions => {
            if !has(ClearDataActionClass::GenericClearExcludingPins) {
                push(
                    violations,
                    "row.clear_actions.pin_exclusion_required",
                    target,
                    "generic_clear_with_pin_exclusions must allow generic_clear_excluding_pins",
                );
            }
            if has(ClearDataActionClass::GenericClearInBulk) {
                push(
                    violations,
                    "row.clear_actions.bulk_ignores_pins",
                    target,
                    "generic_clear_with_pin_exclusions must not allow generic_clear_in_bulk",
                );
            }
        }
        ClearCacheProtectionClass::ProtectedRequiresClassSpecificReview => {
            if any_generic {
                push(violations, "row.clear_actions.protected_generic", target, "protected_requires_class_specific_review must not allow any generic clear action");
            }
            if !has(ClearDataActionClass::ClassSpecificReviewRequired) {
                push(violations, "row.clear_actions.review_required", target, "protected_requires_class_specific_review must require class_specific_review_required");
            }
            if !has(ClearDataActionClass::ExportBeforeDeleteOffered) {
                push(violations, "row.clear_actions.export_required", target, "protected_requires_class_specific_review must offer export_before_delete_offered");
            }
        }
        ClearCacheProtectionClass::ProtectedNeverGenericClear => {
            if any_generic {
                push(
                    violations,
                    "row.clear_actions.never_generic",
                    target,
                    "protected_never_generic_clear must not allow any generic clear action",
                );
            }
            if !has(ClearDataActionClass::ExplicitPerItemReviewRequired) {
                push(
                    violations,
                    "row.clear_actions.per_item_required",
                    target,
                    "protected_never_generic_clear must require explicit_per_item_review_required",
                );
            }
            if !has(ClearDataActionClass::ExportBeforeDeleteOffered) {
                push(
                    violations,
                    "row.clear_actions.export_required",
                    target,
                    "protected_never_generic_clear must offer export_before_delete_offered",
                );
            }
        }
    }
}

fn push(
    violations: &mut Vec<M5StorageGovernanceViolation>,
    check_id: impl Into<String>,
    target_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(M5StorageGovernanceViolation {
        check_id: check_id.into(),
        target_ref: target_ref.into(),
        message: message.into(),
    });
}
