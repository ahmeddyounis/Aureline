//! Governed profile sync, snapshot, backup, restore, and offboarding model.
//!
//! This module is the settings-lane contract for profile portability. It does
//! not implement a transport or backup engine; it defines the canonical record
//! that those surfaces must emit before they mutate local state or claim stable
//! profile roaming.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for profile portability certification records.
pub const PROFILE_SYNC_RESTORE_RECORD_KIND: &str = "profile_sync_restore_certification_record";

/// Schema version for [`ProfileSyncRestoreCertification`] records.
pub const PROFILE_SYNC_RESTORE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by UI, CLI, support, export, and docs surfaces.
pub const PROFILE_SYNC_RESTORE_SHARED_CONTRACT_REF: &str =
    "settings:profile_sync_snapshot_backup_restore:v1";

const MAX_REF_CHARS: usize = 240;
const CANONICAL_OBJECT_SCHEME: &str = "aureline://";

/// Returns true when `reference` is a non-empty canonical object ref.
pub fn is_canonical_object_ref(reference: &str) -> bool {
    let reference = reference.trim();
    if reference.is_empty() || reference.len() > MAX_REF_CHARS {
        return false;
    }
    let Some(rest) = reference.strip_prefix(CANONICAL_OBJECT_SCHEME) else {
        return false;
    };
    let Some((class, ident)) = rest.split_once('/') else {
        return false;
    };
    !class.is_empty() && !ident.is_empty()
}

/// Public claim class derived from certification evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StableClaimClass {
    /// The lane satisfies every stable profile-portability gate.
    Stable,
    /// The lane is usable but narrowed below the stable promise.
    Beta,
    /// The lane is preview-only.
    Preview,
    /// No public claim is made.
    NotClaimed,
}

/// Snapshot class defined by the profile portability contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SnapshotClass {
    /// Same-machine rollback point created before a mutating operation.
    LocalRollbackCheckpoint,
    /// User- or admin-initiated profile export for portability.
    PortableProfileExport,
    /// Optional managed sync snapshot for roaming non-sensitive profile state.
    ManagedSyncSnapshot,
    /// Redacted manifest for support or recovery explanation.
    SupportRecoveryManifest,
}

impl SnapshotClass {
    /// Returns the canonical token for this snapshot class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalRollbackCheckpoint => "local_rollback_checkpoint",
            Self::PortableProfileExport => "portable_profile_export",
            Self::ManagedSyncSnapshot => "managed_sync_snapshot",
            Self::SupportRecoveryManifest => "support_recovery_manifest",
        }
    }

    /// Returns true when the snapshot leaves same-machine rollback storage.
    pub const fn crosses_ordinary_roaming_lane(self) -> bool {
        matches!(
            self,
            Self::PortableProfileExport | Self::ManagedSyncSnapshot | Self::SupportRecoveryManifest
        )
    }

    /// All snapshot classes required by the stable contract.
    pub const REQUIRED: [Self; 4] = [
        Self::LocalRollbackCheckpoint,
        Self::PortableProfileExport,
        Self::ManagedSyncSnapshot,
        Self::SupportRecoveryManifest,
    ];
}

/// State class included in or excluded from snapshots and packages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StateClass {
    /// Scalar settings and UI preferences.
    ScalarSettings,
    /// Keybinding definitions.
    Keybindings,
    /// Snippets and templates.
    Snippets,
    /// Theme packages and UI presets.
    ThemesAndUiPresets,
    /// Task definitions.
    Tasks,
    /// Launch configurations.
    LaunchConfigs,
    /// Workset definitions.
    WorksetDefinitions,
    /// Reference-only extension inventory and lock state.
    ExtensionInventoryRefs,
    /// Reference-only secret metadata such as a vault alias.
    ReferenceOnlySecretMetadata,
    /// Dirty-buffer journals.
    DirtyBufferJournals,
    /// Session restore state.
    SessionRestoreState,
    /// Caches.
    Caches,
    /// Indexes.
    Indexes,
    /// Raw tokens, passkeys, private keys, certificates, or similar material.
    SecretMaterial,
}

impl StateClass {
    /// Returns the canonical token for this state class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScalarSettings => "scalar_settings",
            Self::Keybindings => "keybindings",
            Self::Snippets => "snippets",
            Self::ThemesAndUiPresets => "themes_and_ui_presets",
            Self::Tasks => "tasks",
            Self::LaunchConfigs => "launch_configs",
            Self::WorksetDefinitions => "workset_definitions",
            Self::ExtensionInventoryRefs => "extension_inventory_refs",
            Self::ReferenceOnlySecretMetadata => "reference_only_secret_metadata",
            Self::DirtyBufferJournals => "dirty_buffer_journals",
            Self::SessionRestoreState => "session_restore_state",
            Self::Caches => "caches",
            Self::Indexes => "indexes",
            Self::SecretMaterial => "secret_material",
        }
    }

    /// Returns true when the class must be excluded from ordinary roaming lanes.
    pub const fn forbidden_in_ordinary_roaming(self) -> bool {
        matches!(
            self,
            Self::DirtyBufferJournals
                | Self::SessionRestoreState
                | Self::Caches
                | Self::Indexes
                | Self::SecretMaterial
        )
    }

    /// State classes that ordinary roaming/export lanes must explicitly exclude.
    pub const ORDINARY_ROAMING_EXCLUSIONS: [Self; 5] = [
        Self::DirtyBufferJournals,
        Self::SessionRestoreState,
        Self::Caches,
        Self::Indexes,
        Self::SecretMaterial,
    ];
}

/// Setting or asset category used to enforce merge behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeSubjectClass {
    /// Scalar setting fields.
    ScalarSetting,
    /// Additive assets such as snippets or templates.
    AdditiveAsset,
    /// Keybinding, task, launch, or workset definitions.
    StructuredDefinition,
}

/// Merge rule required before a value can be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeRuleClass {
    /// Fieldwise merge for scalar settings.
    FieldwiseMerge,
    /// Additive merge where assets can be safely appended.
    AdditiveMerge,
    /// Explicit conflict review for structured definitions.
    ExplicitConflictReview,
    /// Local explicit edits win over stale remote copies.
    LocalPrecedence,
}

impl MergeRuleClass {
    /// Returns the canonical token for this merge rule.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FieldwiseMerge => "fieldwise_merge",
            Self::AdditiveMerge => "additive_merge",
            Self::ExplicitConflictReview => "explicit_conflict_review",
            Self::LocalPrecedence => "local_precedence",
        }
    }
}

impl MergeSubjectClass {
    /// Returns the rule required for this subject class.
    pub const fn required_rule(self) -> MergeRuleClass {
        match self {
            Self::ScalarSetting => MergeRuleClass::FieldwiseMerge,
            Self::AdditiveAsset => MergeRuleClass::AdditiveMerge,
            Self::StructuredDefinition => MergeRuleClass::ExplicitConflictReview,
        }
    }
}

/// Conflict class rendered before a sync or restore mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConflictClass {
    /// Local and incoming values match.
    ExactMatch,
    /// A scalar field can merge field by field.
    FieldwiseMerge,
    /// Additive assets can merge without replacing local items.
    AdditiveMerge,
    /// Structured definitions require explicit review.
    ExplicitConflictReview,
    /// Incoming remote state is stale and local state remains authoritative.
    StaleRemoteLocalPrecedence,
}

impl ConflictClass {
    /// Conflict classes required in the deterministic corpus.
    pub const REQUIRED: [Self; 5] = [
        Self::ExactMatch,
        Self::FieldwiseMerge,
        Self::AdditiveMerge,
        Self::ExplicitConflictReview,
        Self::StaleRemoteLocalPrecedence,
    ];
}

/// Source surface that must show the same mutation preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceClass {
    /// Desktop profile sync and restore UI.
    DesktopSettings,
    /// CLI or headless inspect/export command.
    CliInspect,
    /// Support bundle or support center export.
    SupportExport,
    /// Help or docs surface.
    HelpDocs,
    /// Admin offboarding or delete flow.
    AdminOffboarding,
}

impl SurfaceClass {
    /// Required surface set for parity.
    pub const REQUIRED: [Self; 5] = [
        Self::DesktopSettings,
        Self::CliInspect,
        Self::SupportExport,
        Self::HelpDocs,
        Self::AdminOffboarding,
    ];
}

/// One snapshot manifest row with complete provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SnapshotManifestRow {
    /// Snapshot class.
    pub snapshot_class: SnapshotClass,
    /// Canonical snapshot ref.
    pub snapshot_ref: String,
    /// Snapshot schema version.
    pub snapshot_schema_version: String,
    /// Producing Aureline version.
    pub aureline_version: String,
    /// Coarse platform traits preserved with the snapshot.
    pub platform_traits: Vec<String>,
    /// Included state classes.
    pub included_state_classes: Vec<StateClass>,
    /// Excluded state classes.
    pub excluded_state_classes: Vec<StateClass>,
    /// Integrity hash over the package body or manifest.
    pub integrity_hash: String,
    /// Source provenance such as device/package/profile revision.
    pub source_provenance: String,
    /// Whether the snapshot is only same-machine rollback storage.
    pub local_only: bool,
    /// Optional waiver for narrowed rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub waiver_ref: Option<String>,
}

/// One merge or conflict-review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeRuleRow {
    /// Setting id or asset id.
    pub subject_id: String,
    /// Subject class used to select the required merge rule.
    pub subject_class: MergeSubjectClass,
    /// Conflict class rendered to the user or admin.
    pub conflict_class: ConflictClass,
    /// Applied merge rule.
    pub merge_rule: MergeRuleClass,
    /// Incoming source device or package.
    pub source_provenance: String,
    /// Whether the incoming state is stale compared with local lineage.
    pub stale_remote: bool,
    /// Whether local explicit edits remain authoritative.
    pub local_explicit_edit_wins: bool,
    /// Structured diff or change-set ref shown before mutation.
    pub change_set_ref: String,
    /// Rollback checkpoint created before any local overwrite.
    pub rollback_checkpoint_ref: String,
    /// Whether applying the row would overwrite current local state.
    pub overwrites_local_state: bool,
    /// Whether explicit human or policy-approved review is required.
    pub explicit_review_required: bool,
    /// Whether the row can be inspected before apply without mutation.
    pub previewable_before_apply: bool,
}

/// One restore preview row rendered before applying a package or snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorePreviewRow {
    /// Restore preview id.
    pub preview_id: String,
    /// Source device id or package ref.
    pub source_ref: String,
    /// Snapshot class being restored.
    pub snapshot_class: SnapshotClass,
    /// Structured diff or change-set ref.
    pub structured_change_set_ref: String,
    /// Rollback checkpoint created before overwrite.
    pub rollback_checkpoint_ref: String,
    /// Sidecar ref containing cross-platform unmappable values.
    pub cross_platform_unmappable_sidecar_ref: String,
    /// Whether unmappable values are preserved rather than dropped.
    pub preserves_unmappable_sidecar: bool,
    /// Whether the preview names the current local state to be retained or overwritten.
    pub retained_vs_overwritten_explicit: bool,
    /// Whether the preview is non-mutating.
    pub previewable_before_apply: bool,
    /// Whether the row would overwrite current local state.
    pub overwrites_local_state: bool,
}

/// One secret-boundary audit row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretBoundaryAuditRow {
    /// State class under audit.
    pub state_class: StateClass,
    /// Lane under audit.
    pub lane: SnapshotClass,
    /// Whether raw material is excluded.
    pub raw_material_excluded: bool,
    /// Whether reference-only metadata is the strongest allowed representation.
    pub reference_only_metadata_allowed: bool,
    /// Audit evidence ref.
    pub evidence_ref: String,
}

/// One surface-parity row for preview and offboarding truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthRow {
    /// Surface class.
    pub surface_class: SurfaceClass,
    /// Whether the surface consumes this shared record.
    pub consumes_shared_record: bool,
    /// Whether the surface shows source device or source package.
    pub shows_source: bool,
    /// Whether the surface shows snapshot class.
    pub shows_snapshot_class: bool,
    /// Whether the surface shows included and excluded state classes.
    pub shows_state_classes: bool,
    /// Whether the surface shows conflict class.
    pub shows_conflict_class: bool,
    /// Whether the surface shows rollback checkpoint.
    pub shows_rollback_checkpoint: bool,
    /// Whether the surface shows local-authoritative fallback posture.
    pub shows_local_authoritative_fallback: bool,
}

/// Retention and offboarding truth for a delete or exit flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingRetentionSummary {
    /// Bounded local checkpoint retention in days.
    pub local_checkpoint_retention_days: u32,
    /// Whether users can inspect retained local checkpoints.
    pub retention_inspectable: bool,
    /// Final export package produced before delete/offboarding.
    pub final_export_package_ref: String,
    /// Latest successful sync manifest included in the package.
    pub latest_successful_sync_manifest_ref: String,
    /// Profile export pointers included in the package.
    pub profile_export_pointers: Vec<String>,
    /// Extension inventory pointer included in the package.
    pub extension_inventory_ref: String,
    /// Remaining-retention timeline ref included in the package.
    pub remaining_retention_timeline_ref: String,
    /// Whether the package explains retained/exported/excluded classes without internal logs.
    pub explainable_without_internal_logs: bool,
    /// Whether local launch and editing survive missing managed sync.
    pub local_launch_edit_authority_retained: bool,
    /// Whether managed sync is required for local launch or editing.
    pub managed_sync_required_for_local_work: bool,
}

/// Derived stable-pillar verdicts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSyncRestorePillars {
    /// All four snapshot classes are present and fully described.
    pub snapshot_classes_complete: bool,
    /// Merge rules satisfy local-precedence and conflict-review requirements.
    pub merge_rules_enforced: bool,
    /// Ordinary roaming/export lanes exclude volatile and secret classes.
    pub secret_boundary_held: bool,
    /// Restores are previewable and checkpointed before overwrite.
    pub restore_preview_checkpointed: bool,
    /// Cross-platform unmappable values are preserved in a sidecar.
    pub cross_platform_sidecar_preserved: bool,
    /// Retention and offboarding package truth is complete.
    pub retention_offboarding_truth: bool,
    /// Mutating surfaces expose the required source, class, conflict, and fallback fields.
    pub surface_truth_complete: bool,
    /// Local launch and editing do not depend on managed sync.
    pub local_authority_retained: bool,
}

/// Reason the record is narrowed below the stable claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// Snapshot classes or metadata are incomplete.
    SnapshotClassesIncomplete,
    /// Merge rules are missing or incorrect.
    MergeRulesUnenforced,
    /// Forbidden state classes crossed an ordinary roaming lane.
    SecretBoundaryFailed,
    /// Restore preview or rollback checkpoint is missing.
    RestorePreviewMissing,
    /// Cross-platform unmappables are not preserved.
    CrossPlatformSidecarMissing,
    /// Retention or offboarding truth is incomplete.
    RetentionOffboardingIncomplete,
    /// One or more surfaces omit required mutation truth.
    SurfaceTruthIncomplete,
    /// Managed sync became a prerequisite for local launch or editing.
    LocalAuthorityLost,
}

/// Derived stable-claim verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSyncRestoreQualification {
    /// Derived claim class.
    pub claim_class: StableClaimClass,
    /// Whether the record qualifies at the stable cutline.
    pub qualifies_stable: bool,
    /// Named narrowing reasons.
    pub narrowing_reasons: Vec<NarrowingReason>,
}

/// Input used to build a [`ProfileSyncRestoreCertification`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileSyncRestoreInput {
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Snapshot manifest rows.
    pub snapshots: Vec<SnapshotManifestRow>,
    /// Merge rule rows.
    pub merge_rules: Vec<MergeRuleRow>,
    /// Restore preview rows.
    pub restore_previews: Vec<RestorePreviewRow>,
    /// Secret boundary audit rows.
    pub secret_boundary_audit: Vec<SecretBoundaryAuditRow>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
    /// Retention and offboarding summary.
    pub offboarding_retention: OffboardingRetentionSummary,
}

/// Canonical profile sync, snapshot, backup, restore, and offboarding record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileSyncRestoreCertification {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable record id.
    pub record_id: String,
    /// UTC timestamp for the record.
    pub as_of: String,
    /// Human-readable summary.
    pub summary: String,
    /// Snapshot manifest rows.
    pub snapshots: Vec<SnapshotManifestRow>,
    /// Merge rule rows.
    pub merge_rules: Vec<MergeRuleRow>,
    /// Restore preview rows.
    pub restore_previews: Vec<RestorePreviewRow>,
    /// Secret boundary audit rows.
    pub secret_boundary_audit: Vec<SecretBoundaryAuditRow>,
    /// Surface truth rows.
    pub surface_truth: Vec<SurfaceTruthRow>,
    /// Retention and offboarding summary.
    pub offboarding_retention: OffboardingRetentionSummary,
    /// Conflict classes covered by the corpus.
    pub conflict_coverage: Vec<ConflictClass>,
    /// Merge rules covered by the corpus.
    pub merge_rule_coverage: Vec<MergeRuleClass>,
    /// Derived pillar verdicts.
    pub pillars: ProfileSyncRestorePillars,
    /// Derived stable qualification.
    pub stable_qualification: ProfileSyncRestoreQualification,
}

/// Reasons a profile sync/restore certification cannot be built.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildError {
    /// A required snapshot class is missing.
    MissingSnapshotClass { class: SnapshotClass },
    /// A snapshot class is duplicated.
    DuplicateSnapshotClass { class: SnapshotClass },
    /// A canonical ref field is invalid.
    NonCanonicalRef { field: &'static str, value: String },
    /// Required conflict coverage is missing.
    MissingConflictCoverage { class: ConflictClass },
    /// Required merge-rule coverage is missing.
    MissingMergeRuleCoverage { rule: MergeRuleClass },
    /// A required surface row is missing.
    MissingSurface { surface: SurfaceClass },
}

impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingSnapshotClass { class } => {
                write!(f, "missing snapshot class `{}`", class.as_str())
            }
            Self::DuplicateSnapshotClass { class } => {
                write!(f, "duplicated snapshot class `{}`", class.as_str())
            }
            Self::NonCanonicalRef { field, value } => {
                write!(f, "field `{field}` must be a canonical ref, got {value:?}")
            }
            Self::MissingConflictCoverage { class } => {
                write!(f, "missing conflict coverage `{class:?}`")
            }
            Self::MissingMergeRuleCoverage { rule } => {
                write!(f, "missing merge-rule coverage `{}`", rule.as_str())
            }
            Self::MissingSurface { surface } => write!(f, "missing surface `{surface:?}`"),
        }
    }
}

impl std::error::Error for BuildError {}

fn require_ref(field: &'static str, value: &str) -> Result<(), BuildError> {
    if is_canonical_object_ref(value) {
        Ok(())
    } else {
        Err(BuildError::NonCanonicalRef {
            field,
            value: value.to_owned(),
        })
    }
}

impl ProfileSyncRestoreCertification {
    /// Builds a derived certification record from raw evidence rows.
    pub fn build(mut input: ProfileSyncRestoreInput) -> Result<Self, BuildError> {
        let mut snapshot_classes = BTreeSet::new();
        for row in &input.snapshots {
            if !snapshot_classes.insert(row.snapshot_class) {
                return Err(BuildError::DuplicateSnapshotClass {
                    class: row.snapshot_class,
                });
            }
            require_ref("snapshots.snapshot_ref", &row.snapshot_ref)?;
        }
        for class in SnapshotClass::REQUIRED {
            if !snapshot_classes.contains(&class) {
                return Err(BuildError::MissingSnapshotClass { class });
            }
        }

        for row in &input.merge_rules {
            require_ref("merge_rules.change_set_ref", &row.change_set_ref)?;
            if !row.rollback_checkpoint_ref.is_empty() {
                require_ref(
                    "merge_rules.rollback_checkpoint_ref",
                    &row.rollback_checkpoint_ref,
                )?;
            }
        }
        for row in &input.restore_previews {
            require_ref("restore_previews.source_ref", &row.source_ref)?;
            require_ref(
                "restore_previews.structured_change_set_ref",
                &row.structured_change_set_ref,
            )?;
            if !row.rollback_checkpoint_ref.is_empty() {
                require_ref(
                    "restore_previews.rollback_checkpoint_ref",
                    &row.rollback_checkpoint_ref,
                )?;
            }
            require_ref(
                "restore_previews.cross_platform_unmappable_sidecar_ref",
                &row.cross_platform_unmappable_sidecar_ref,
            )?;
        }
        for row in &input.secret_boundary_audit {
            require_ref("secret_boundary_audit.evidence_ref", &row.evidence_ref)?;
        }
        require_ref(
            "offboarding_retention.final_export_package_ref",
            &input.offboarding_retention.final_export_package_ref,
        )?;
        require_ref(
            "offboarding_retention.latest_successful_sync_manifest_ref",
            &input
                .offboarding_retention
                .latest_successful_sync_manifest_ref,
        )?;
        require_ref(
            "offboarding_retention.extension_inventory_ref",
            &input.offboarding_retention.extension_inventory_ref,
        )?;
        require_ref(
            "offboarding_retention.remaining_retention_timeline_ref",
            &input.offboarding_retention.remaining_retention_timeline_ref,
        )?;
        for pointer in &input.offboarding_retention.profile_export_pointers {
            require_ref("offboarding_retention.profile_export_pointers", pointer)?;
        }

        input.snapshots.sort_by_key(|row| row.snapshot_class);
        input
            .merge_rules
            .sort_by(|a, b| a.subject_id.cmp(&b.subject_id));
        input
            .restore_previews
            .sort_by(|a, b| a.preview_id.cmp(&b.preview_id));
        input
            .secret_boundary_audit
            .sort_by_key(|row| (row.lane, row.state_class));
        input.surface_truth.sort_by_key(|row| row.surface_class);

        let conflict_coverage: Vec<ConflictClass> = input
            .merge_rules
            .iter()
            .map(|row| row.conflict_class)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        for class in ConflictClass::REQUIRED {
            if !conflict_coverage.contains(&class) {
                return Err(BuildError::MissingConflictCoverage { class });
            }
        }

        let merge_rule_coverage: Vec<MergeRuleClass> = input
            .merge_rules
            .iter()
            .map(|row| row.merge_rule)
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();
        for rule in [
            MergeRuleClass::FieldwiseMerge,
            MergeRuleClass::AdditiveMerge,
            MergeRuleClass::ExplicitConflictReview,
            MergeRuleClass::LocalPrecedence,
        ] {
            if !merge_rule_coverage.contains(&rule) {
                return Err(BuildError::MissingMergeRuleCoverage { rule });
            }
        }

        let present_surfaces: BTreeSet<SurfaceClass> = input
            .surface_truth
            .iter()
            .map(|row| row.surface_class)
            .collect();
        for surface in SurfaceClass::REQUIRED {
            if !present_surfaces.contains(&surface) {
                return Err(BuildError::MissingSurface { surface });
            }
        }

        let snapshot_classes_complete = input.snapshots.iter().all(|row| {
            !row.snapshot_schema_version.trim().is_empty()
                && !row.aureline_version.trim().is_empty()
                && !row.platform_traits.is_empty()
                && !row.included_state_classes.is_empty()
                && !row.excluded_state_classes.is_empty()
                && !row.integrity_hash.trim().is_empty()
                && !row.source_provenance.trim().is_empty()
                && !row
                    .included_state_classes
                    .contains(&StateClass::SecretMaterial)
                && (!row.snapshot_class.crosses_ordinary_roaming_lane() || !row.local_only)
        });

        let merge_rules_enforced = input.merge_rules.iter().all(|row| {
            let expected = if row.stale_remote {
                MergeRuleClass::LocalPrecedence
            } else {
                row.subject_class.required_rule()
            };
            row.merge_rule == expected
                && row.previewable_before_apply
                && (!row.stale_remote || row.local_explicit_edit_wins)
                && (!row.overwrites_local_state
                    || (!row.change_set_ref.is_empty() && !row.rollback_checkpoint_ref.is_empty()))
                && (!matches!(row.subject_class, MergeSubjectClass::StructuredDefinition)
                    || row.explicit_review_required)
        });

        let secret_boundary_held = input
            .snapshots
            .iter()
            .filter(|row| row.snapshot_class.crosses_ordinary_roaming_lane())
            .all(|row| {
                StateClass::ORDINARY_ROAMING_EXCLUSIONS
                    .iter()
                    .all(|class| row.excluded_state_classes.contains(class))
                    && !row
                        .included_state_classes
                        .iter()
                        .any(|class| class.forbidden_in_ordinary_roaming())
            })
            && input.secret_boundary_audit.iter().all(|row| {
                row.raw_material_excluded
                    && row.reference_only_metadata_allowed
                    && row.state_class.forbidden_in_ordinary_roaming()
            });

        let restore_preview_checkpointed = input.restore_previews.iter().all(|row| {
            row.previewable_before_apply
                && row.retained_vs_overwritten_explicit
                && (!row.overwrites_local_state
                    || (!row.structured_change_set_ref.is_empty()
                        && !row.rollback_checkpoint_ref.is_empty()))
        });

        let cross_platform_sidecar_preserved = input.restore_previews.iter().all(|row| {
            row.preserves_unmappable_sidecar
                && !row.cross_platform_unmappable_sidecar_ref.trim().is_empty()
        });

        let retention = &input.offboarding_retention;
        let retention_offboarding_truth = retention.local_checkpoint_retention_days > 0
            && retention.local_checkpoint_retention_days <= 90
            && retention.retention_inspectable
            && !retention.profile_export_pointers.is_empty()
            && retention.explainable_without_internal_logs;

        let surface_truth_complete = input.surface_truth.iter().all(|row| {
            row.consumes_shared_record
                && row.shows_source
                && row.shows_snapshot_class
                && row.shows_state_classes
                && row.shows_conflict_class
                && row.shows_rollback_checkpoint
                && row.shows_local_authoritative_fallback
        });

        let local_authority_retained = retention.local_launch_edit_authority_retained
            && !retention.managed_sync_required_for_local_work;

        let pillars = ProfileSyncRestorePillars {
            snapshot_classes_complete,
            merge_rules_enforced,
            secret_boundary_held,
            restore_preview_checkpointed,
            cross_platform_sidecar_preserved,
            retention_offboarding_truth,
            surface_truth_complete,
            local_authority_retained,
        };

        let mut narrowing_reasons = Vec::new();
        if !pillars.snapshot_classes_complete {
            narrowing_reasons.push(NarrowingReason::SnapshotClassesIncomplete);
        }
        if !pillars.merge_rules_enforced {
            narrowing_reasons.push(NarrowingReason::MergeRulesUnenforced);
        }
        if !pillars.secret_boundary_held {
            narrowing_reasons.push(NarrowingReason::SecretBoundaryFailed);
        }
        if !pillars.restore_preview_checkpointed {
            narrowing_reasons.push(NarrowingReason::RestorePreviewMissing);
        }
        if !pillars.cross_platform_sidecar_preserved {
            narrowing_reasons.push(NarrowingReason::CrossPlatformSidecarMissing);
        }
        if !pillars.retention_offboarding_truth {
            narrowing_reasons.push(NarrowingReason::RetentionOffboardingIncomplete);
        }
        if !pillars.surface_truth_complete {
            narrowing_reasons.push(NarrowingReason::SurfaceTruthIncomplete);
        }
        if !pillars.local_authority_retained {
            narrowing_reasons.push(NarrowingReason::LocalAuthorityLost);
        }

        let qualifies_stable = narrowing_reasons.is_empty();
        let stable_qualification = ProfileSyncRestoreQualification {
            claim_class: if qualifies_stable {
                StableClaimClass::Stable
            } else {
                StableClaimClass::Beta
            },
            qualifies_stable,
            narrowing_reasons,
        };

        Ok(Self {
            record_kind: PROFILE_SYNC_RESTORE_RECORD_KIND.to_owned(),
            schema_version: PROFILE_SYNC_RESTORE_SCHEMA_VERSION,
            shared_contract_ref: PROFILE_SYNC_RESTORE_SHARED_CONTRACT_REF.to_owned(),
            record_id: input.record_id,
            as_of: input.as_of,
            summary: input.summary,
            snapshots: input.snapshots,
            merge_rules: input.merge_rules,
            restore_previews: input.restore_previews,
            secret_boundary_audit: input.secret_boundary_audit,
            surface_truth: input.surface_truth,
            offboarding_retention: input.offboarding_retention,
            conflict_coverage,
            merge_rule_coverage,
            pillars,
            stable_qualification,
        })
    }

    /// Renders a compact support-export summary from the shared record.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("record_id: {}", self.record_id),
            format!("claim: {:?}", self.stable_qualification.claim_class),
            format!("snapshot_classes: {}", self.snapshots.len()),
            format!("restore_previews: {}", self.restore_previews.len()),
            format!(
                "final_export_package: {}",
                self.offboarding_retention.final_export_package_ref
            ),
            format!(
                "local_authority_retained: {}",
                self.pillars.local_authority_retained
            ),
        ]
    }
}
