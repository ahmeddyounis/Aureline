//! Backup, restore, and failover alpha fixture ingestion.
//!
//! This module consumes the checked-in recovery and operations fixtures that
//! define backup/checkpoint classes, failover continuity banners, and protected
//! backup-restore-failover rehearsal cases. It keeps fixture parsing typed so
//! recovery and support surfaces can share the same outage-class and plane
//! vocabulary instead of comparing opaque strings.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::recovery_ladder::{OutageClass, OutagePlaneClass};
use serde::{Deserialize, Serialize};

/// Artifact id for the checked-in backup/checkpoint class vocabulary.
pub const BACKUP_CHECKPOINT_CLASSES_ARTIFACT_ID: &str =
    "aureline.recovery.backup_checkpoint_classes";

/// Stable record-kind tag for the protected rehearsal manifest.
pub const BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_RECORD_KIND: &str =
    "backup_restore_failover_rehearsal_manifest";

/// Stable record-kind tag for one protected rehearsal case.
pub const BACKUP_RESTORE_FAILOVER_REHEARSAL_CASE_RECORD_KIND: &str =
    "backup_restore_failover_rehearsal_case";

/// Stable record-kind tag for local-safe baseline records.
pub const LOCAL_SAFE_BASELINE_RECORD_KIND: &str = "local_safe_baseline_record";

/// Stable record-kind tag for failover banner records.
pub const FAILOVER_BANNER_RECORD_KIND: &str = "failover_banner_record";

/// Repository-relative path for the backup/checkpoint class artifact.
pub const BACKUP_CHECKPOINT_CLASSES_PATH: &str =
    "artifacts/recovery/backup_checkpoint_classes.yaml";

/// Repository-relative path for the protected rehearsal manifest.
pub const BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_PATH: &str =
    "fixtures/ops/backup_restore_failover_rehearsal_cases/manifest.yaml";

/// Repository-relative path for the failover continuity fixture directory.
pub const FAILOVER_CONTINUITY_CASES_DIR: &str = "fixtures/ops/failover_continuity_cases";

/// Loads backup/checkpoint class definitions from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text does not match
/// [`BackupCheckpointClasses`].
pub fn load_backup_checkpoint_classes(
    yaml: &str,
) -> Result<BackupCheckpointClasses, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads a protected rehearsal manifest from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text does not match
/// [`BackupRestoreFailoverRehearsalManifest`].
pub fn load_rehearsal_manifest(
    yaml: &str,
) -> Result<BackupRestoreFailoverRehearsalManifest, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads one protected rehearsal case from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text does not match
/// [`BackupRestoreFailoverRehearsalCase`].
pub fn load_rehearsal_case(
    yaml: &str,
) -> Result<BackupRestoreFailoverRehearsalCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads one failover continuity case from YAML text.
///
/// # Errors
///
/// Returns a YAML parse error when the text does not match
/// [`FailoverContinuityCase`].
pub fn load_failover_continuity_case(
    yaml: &str,
) -> Result<FailoverContinuityCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the checked-in failover alpha corpus from a repository root.
///
/// # Errors
///
/// Returns [`FailoverAlphaLoadError`] when a required file cannot be read,
/// a fixture cannot be parsed, or the continuity fixture directory cannot be
/// listed.
pub fn load_current_failover_alpha_corpus(
    repo_root: impl AsRef<Path>,
) -> Result<FailoverAlphaCorpus, FailoverAlphaLoadError> {
    let repo_root = repo_root.as_ref();
    let backup_checkpoint_classes =
        read_yaml_path::<BackupCheckpointClasses>(&repo_root.join(BACKUP_CHECKPOINT_CLASSES_PATH))?;
    let manifest_path = repo_root.join(BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_PATH);
    let rehearsal_manifest =
        read_yaml_path::<BackupRestoreFailoverRehearsalManifest>(&manifest_path)?;
    let manifest_dir = manifest_path
        .parent()
        .expect("manifest path always has a parent directory");

    let rehearsal_cases = rehearsal_manifest
        .case_files
        .iter()
        .map(|case_ref| {
            let case_path = manifest_dir.join(&case_ref.file);
            let case = read_yaml_path::<BackupRestoreFailoverRehearsalCase>(&case_path)?;
            Ok(RehearsalCaseEntry {
                fixture_ref: repo_relative_path(repo_root, &case_path),
                manifest_ref: case_ref.clone(),
                case,
            })
        })
        .collect::<Result<Vec<_>, FailoverAlphaLoadError>>()?;

    let continuity_cases =
        load_failover_continuity_cases_from_dir(repo_root.join(FAILOVER_CONTINUITY_CASES_DIR))?
            .into_iter()
            .map(|(path, case)| ContinuityCaseEntry {
                fixture_ref: repo_relative_path(repo_root, &path),
                case,
            })
            .collect();

    Ok(FailoverAlphaCorpus {
        backup_checkpoint_classes,
        rehearsal_manifest,
        rehearsal_cases,
        continuity_cases,
    })
}

/// Loads every `.yaml` failover continuity case from a directory.
///
/// # Errors
///
/// Returns [`FailoverAlphaLoadError`] when the directory cannot be listed or a
/// fixture cannot be read or parsed.
pub fn load_failover_continuity_cases_from_dir(
    dir: impl AsRef<Path>,
) -> Result<Vec<(PathBuf, FailoverContinuityCase)>, FailoverAlphaLoadError> {
    let dir = dir.as_ref();
    let mut paths = Vec::new();
    for entry in fs::read_dir(dir).map_err(|source| FailoverAlphaLoadError::ReadDir {
        path: dir.to_path_buf(),
        source,
    })? {
        let entry = entry.map_err(|source| FailoverAlphaLoadError::ReadDir {
            path: dir.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) == Some("yaml") {
            paths.push(path);
        }
    }
    paths.sort();

    paths
        .into_iter()
        .map(|path| {
            let case = read_yaml_path::<FailoverContinuityCase>(&path)?;
            Ok((path, case))
        })
        .collect()
}

/// Loading failure for checked-in failover alpha artifacts.
#[derive(Debug)]
pub enum FailoverAlphaLoadError {
    /// A YAML fixture or artifact could not be read.
    Read {
        /// Filesystem path that failed.
        path: PathBuf,
        /// Underlying IO error.
        source: std::io::Error,
    },
    /// A directory containing fixtures could not be listed.
    ReadDir {
        /// Directory path that failed.
        path: PathBuf,
        /// Underlying IO error.
        source: std::io::Error,
    },
    /// A YAML fixture or artifact could not be parsed into its typed record.
    Parse {
        /// Filesystem path that failed.
        path: PathBuf,
        /// Underlying YAML parse error.
        source: serde_yaml::Error,
    },
}

impl fmt::Display for FailoverAlphaLoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => write!(f, "read {}: {source}", path.display()),
            Self::ReadDir { path, source } => {
                write!(f, "read directory {}: {source}", path.display())
            }
            Self::Parse { path, source } => write!(f, "parse {}: {source}", path.display()),
        }
    }
}

impl Error for FailoverAlphaLoadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Read { source, .. } | Self::ReadDir { source, .. } => Some(source),
            Self::Parse { source, .. } => Some(source),
        }
    }
}

/// Parsed backup/checkpoint class vocabulary artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupCheckpointClasses {
    /// Artifact schema version.
    pub schema_version: u32,
    /// Stable artifact identifier.
    pub artifact_id: String,
    /// Reviewer-facing artifact title.
    pub title: String,
    /// Contract document that owns this vocabulary.
    pub contract_source: String,
    /// Related contract documents.
    #[serde(default)]
    pub related_docs: Vec<String>,
    /// Companion artifact references.
    pub companion_refs: BackupCheckpointCompanionRefs,
    /// Closed recovery-promise class vocabulary.
    pub recovery_promise_class_vocabulary: Vec<RecoveryPromiseClass>,
    /// Closed restore-target class vocabulary.
    pub restore_target_class_vocabulary: Vec<RestoreTargetClass>,
    /// Closed restorability state vocabulary.
    pub restorability_state_vocabulary: Vec<RestorabilityState>,
    /// Closed verification state vocabulary.
    pub verification_state_vocabulary: Vec<VerificationState>,
    /// Closed local-history health vocabulary.
    pub local_history_health_vocabulary: Vec<LocalHistoryHealth>,
    /// Closed sync/replication state vocabulary.
    pub sync_replication_state_vocabulary: Vec<SyncReplicationState>,
    /// Closed export availability vocabulary.
    pub export_availability_vocabulary: Vec<ExportAvailability>,
    /// Closed deployment/profile scope vocabulary.
    pub deployment_profile_scope_vocabulary: Vec<DeploymentProfileScope>,
    /// Closed local-safe guidance vocabulary.
    pub local_safe_guidance_vocabulary: Vec<LocalSafeGuidance>,
    /// Recovery-promise class rows.
    pub recovery_promise_classes: Vec<RecoveryPromiseClassRow>,
    /// Restore-target class rows.
    pub restore_target_classes: Vec<RestoreTargetClassRow>,
    /// Cross-class honesty invariants.
    #[serde(default)]
    pub cross_class_invariants: Vec<CrossClassInvariant>,
}

impl BackupCheckpointClasses {
    /// Validates schema identity and vocabulary-to-row coverage.
    pub fn validate(&self) -> Vec<FailoverAlphaViolation> {
        let mut violations = Vec::new();
        if self.schema_version != 1 {
            push_violation(
                &mut violations,
                "backup_checkpoint_classes.schema_version",
                &self.artifact_id,
                "schema_version must be 1",
            );
        }
        if self.artifact_id != BACKUP_CHECKPOINT_CLASSES_ARTIFACT_ID {
            push_violation(
                &mut violations,
                "backup_checkpoint_classes.artifact_id",
                &self.artifact_id,
                "artifact_id does not match the canonical backup/checkpoint artifact",
            );
        }

        let promise_vocab = set_from_copied(self.recovery_promise_class_vocabulary.iter());
        let promise_rows = set_from_copied(self.recovery_promise_classes.iter().map(|row| &row.id));
        if promise_vocab != promise_rows {
            push_violation(
                &mut violations,
                "backup_checkpoint_classes.recovery_promise_coverage",
                &self.artifact_id,
                "recovery promise rows must exactly cover the vocabulary",
            );
        }

        let target_vocab = set_from_copied(self.restore_target_class_vocabulary.iter());
        let target_rows = set_from_copied(self.restore_target_classes.iter().map(|row| &row.id));
        if target_vocab != target_rows {
            push_violation(
                &mut violations,
                "backup_checkpoint_classes.restore_target_coverage",
                &self.artifact_id,
                "restore target rows must exactly cover the vocabulary",
            );
        }

        for row in &self.recovery_promise_classes {
            if matches!(
                row.id,
                RecoveryPromiseClass::MirrorCache | RecoveryPromiseClass::ConvenienceExport
            ) && !row.can_replace_target_classes.is_empty()
            {
                push_violation(
                    &mut violations,
                    "backup_checkpoint_classes.non_authoritative_restore_source",
                    row.id.as_str(),
                    "mirror caches and convenience exports must not replace restore targets",
                );
            }
            if row.required_verification_states.is_empty() {
                push_violation(
                    &mut violations,
                    "backup_checkpoint_classes.verification_states_missing",
                    row.id.as_str(),
                    "each recovery promise class must name verification states",
                );
            }
        }

        for row in &self.restore_target_classes {
            if row.minimum_local_safe_guidance.is_empty() {
                push_violation(
                    &mut violations,
                    "backup_checkpoint_classes.local_safe_guidance_missing",
                    row.id.as_str(),
                    "each restore target class must name local-safe guidance",
                );
            }
            if row.authoritative_promise_classes.is_empty() {
                push_violation(
                    &mut violations,
                    "backup_checkpoint_classes.authoritative_promise_missing",
                    row.id.as_str(),
                    "each restore target class must name an authoritative promise",
                );
            }
        }

        if self.cross_class_invariants.is_empty() {
            push_violation(
                &mut violations,
                "backup_checkpoint_classes.cross_class_invariants_missing",
                &self.artifact_id,
                "backup/checkpoint classes must carry cross-class invariants",
            );
        }

        violations
    }
}

/// Companion references for backup/checkpoint class consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupCheckpointCompanionRefs {
    /// Continuity status card schema reference.
    pub continuity_status_card_schema_ref: String,
    /// Continuity status fixture directory reference.
    pub continuity_status_cases_ref: String,
    /// Runtime storage class artifact reference.
    pub storage_classes_ref: String,
    /// State object inventory artifact reference.
    pub state_objects_ref: String,
    /// Deployment drill catalog artifact reference.
    pub drill_catalog_ref: String,
}

/// Recovery promise attached to a backup, checkpoint, replica, mirror, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryPromiseClass {
    /// Verified backup of user-authored durable state.
    AuthoritativeBackup,
    /// Local history, autosave journal, or dirty-buffer checkpoint.
    LocalCheckpoint,
    /// Optional profile sync replica.
    SyncReplica,
    /// Signed mirror or offline-bundle cache of upstream artifacts.
    MirrorCache,
    /// User-initiated export packet.
    ConvenienceExport,
}

impl RecoveryPromiseClass {
    /// Returns the stable fixture token for this promise class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeBackup => "authoritative_backup",
            Self::LocalCheckpoint => "local_checkpoint",
            Self::SyncReplica => "sync_replica",
            Self::MirrorCache => "mirror_cache",
            Self::ConvenienceExport => "convenience_export",
        }
    }
}

/// Restore target covered by recovery surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreTargetClass {
    /// Workspace bytes and in-progress work.
    Workspace,
    /// Profile and durable settings state.
    Profile,
    /// Evidence and support packets.
    Evidence,
    /// Window topology and layout state.
    Layout,
}

impl RestoreTargetClass {
    /// Returns the stable fixture token for this restore target class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workspace => "workspace",
            Self::Profile => "profile",
            Self::Evidence => "evidence",
            Self::Layout => "layout",
        }
    }
}

/// Restorability state for a restore-target row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorabilityState {
    /// Restore source is ready for the target.
    Ready,
    /// Restore source exists but is stale.
    Stale,
    /// Restore source exists but has not been verified.
    Unverified,
    /// Restore source is missing.
    Missing,
    /// Restore can recover only part of the target.
    PartiallyRestorable,
}

/// Verification state for a recovery source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationState {
    /// Source was verified recently.
    VerifiedRecent,
    /// Source was verified inside the policy window.
    VerifiedWithinPolicyWindow,
    /// Verification is stale.
    StaleVerification,
    /// Source was never verified.
    NeverVerified,
    /// Verification is currently unavailable.
    VerificationUnavailable,
}

/// Local-history health state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryHealth {
    /// Local history is healthy.
    Healthy,
    /// Retention is under pressure.
    RetentionPressure,
    /// Local-history quota is exhausted.
    QuotaExhausted,
    /// Local-history storage is unavailable.
    StorageUnavailable,
    /// Local history has never been initialized.
    NeverInitialized,
    /// Local history was cleared by the user.
    ClearedByUser,
}

/// Sync or replication state for a continuity card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SyncReplicationState {
    /// Sync is not configured.
    NotConfigured,
    /// Replica is in sync.
    InSync,
    /// Replica is lagging within the policy window.
    LaggingWithinWindow,
    /// Replica is stale outside the policy window.
    StaleOutsideWindow,
    /// Sync was paused by the user.
    PausedByUser,
    /// Sync was paused by policy.
    PausedByPolicy,
    /// Replica diverged and requires review.
    DivergentRequiresReview,
    /// Sync is unavailable.
    SyncUnavailable,
}

/// Export availability state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportAvailability {
    /// A recent export exists on disk.
    ExportOnDiskRecent,
    /// A stale export exists on disk.
    ExportOnDiskStale,
    /// An export was offered but not taken.
    ExportOfferedNotTaken,
    /// An export is in progress.
    ExportInProgress,
    /// Export is blocked by policy.
    ExportBlockedByPolicy,
    /// Export is not offered for metadata-only state.
    ExportNotOfferedMetadataOnly,
    /// Export is unavailable.
    ExportUnavailable,
}

/// Deployment and profile scope used by continuity status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileScope {
    /// Individual local profile.
    IndividualLocal,
    /// Self-hosted profile.
    SelfHosted,
    /// Air-gapped profile.
    AirGapped,
    /// Managed tenant profile.
    ManagedTenant,
    /// Cross-plane failover is pending.
    CrossPlaneFailoverPending,
}

/// Local-safe guidance label for a restore or continuity row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalSafeGuidance {
    /// Continue work using local-safe capability.
    ContinueLocalWork,
    /// Export before making a broader change.
    ExportNowBeforeChange,
    /// Verify the backup locally.
    VerifyBackupLocally,
    /// Capture a fresh local checkpoint.
    ReCaptureLocalCheckpoint,
    /// Import an offline bundle.
    ImportOfflineBundle,
    /// Reauthorize sync after review.
    ReauthorizeSyncAfterReview,
    /// Rebuild from an authoritative source.
    RebuildFromAuthoritativeSource,
    /// Use partial recovery because exact restore is unsupported.
    RestoreUnsupportedUsePartialRecovery,
    /// Escalate to support with redacted evidence.
    EscalateToSupportWithEvidence,
}

/// One recovery-promise class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryPromiseClassRow {
    /// Recovery promise class identifier.
    pub id: RecoveryPromiseClass,
    /// Reviewer-facing label.
    pub display_label: String,
    /// Restore-target classes this promise can replace.
    #[serde(default)]
    pub can_replace_target_classes: Vec<RestoreTargetClass>,
    /// Restore-target classes this promise cannot replace.
    #[serde(default)]
    pub cannot_replace_target_classes: Vec<RestoreTargetClass>,
    /// Verification states required for this promise.
    #[serde(default)]
    pub required_verification_states: Vec<VerificationState>,
    /// Typical storage classes used by this promise.
    #[serde(default)]
    pub typical_storage_classes: Vec<String>,
    /// Typical state categories covered by this promise.
    #[serde(default)]
    pub typical_state_categories: Vec<String>,
    /// Rationale for the row.
    pub rationale: String,
}

/// One restore-target class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreTargetClassRow {
    /// Restore target class identifier.
    pub id: RestoreTargetClass,
    /// Reviewer-facing label.
    pub display_label: String,
    /// State categories covered by this target class.
    #[serde(default)]
    pub covered_state_categories: Vec<String>,
    /// Promise classes authoritative for this target.
    #[serde(default)]
    pub authoritative_promise_classes: Vec<RecoveryPromiseClass>,
    /// Promise classes that can only advise for this target.
    #[serde(default)]
    pub advisory_promise_classes: Vec<RecoveryPromiseClass>,
    /// Minimum local-safe guidance labels for this target.
    #[serde(default)]
    pub minimum_local_safe_guidance: Vec<LocalSafeGuidance>,
    /// Rationale for the row.
    pub rationale: String,
}

/// Cross-class invariant row for recovery-source honesty.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossClassInvariant {
    /// Stable invariant identifier.
    pub id: String,
    /// Human-readable invariant statement.
    pub statement: String,
}

/// Parsed protected rehearsal manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverRehearsalManifest {
    /// Manifest schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Protection status for the fixture lane.
    pub status: String,
    /// Date the manifest represents.
    pub as_of: String,
    /// Owning role for the manifest.
    pub owner_role: String,
    /// Outage taxonomy artifact reference.
    pub taxonomy_ref: String,
    /// Rehearsal plan document reference.
    pub rehearsal_plan_ref: String,
    /// Control/data-plane examples reference.
    pub examples_ref: String,
    /// Proof packet reference.
    pub proof_packet_ref: String,
    /// Validator reference.
    pub validator_ref: String,
    /// Required outage classes.
    pub required_outage_class_ids: Vec<OutageClass>,
    /// Case file rows in this manifest.
    pub case_files: Vec<RehearsalCaseFileRef>,
    /// Acceptance state tokens required by the manifest.
    #[serde(default)]
    pub acceptance_states: Vec<String>,
    /// Export-safety declaration for the fixture lane.
    pub export_safety: ExportSafety,
}

impl BackupRestoreFailoverRehearsalManifest {
    /// Returns the outage classes covered by manifest case rows.
    pub fn covered_outage_classes(&self) -> BTreeSet<OutageClass> {
        self.case_files
            .iter()
            .map(|case_ref| case_ref.outage_class_id)
            .collect()
    }
}

/// One case file row from the protected manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RehearsalCaseFileRef {
    /// Case file name relative to the manifest directory.
    pub file: String,
    /// Outage class exercised by the case.
    pub outage_class_id: OutageClass,
    /// Expected primary plane for the case.
    pub expected_primary_plane_class: OutagePlaneClass,
}

/// Parsed protected backup, restore, and failover rehearsal case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverRehearsalCase {
    /// Case schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable case id.
    pub case_id: String,
    /// Outage class exercised by the case.
    pub outage_class_id: OutageClass,
    /// Primary plane exercised by the case.
    pub primary_plane_class: OutagePlaneClass,
    /// Reviewer-facing scenario summary.
    pub scenario: String,
    /// Source artifacts cited by the case.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// Observed plane states.
    pub plane_observation: PlaneObservation,
    /// Expected product posture.
    pub expected_product_posture: ExpectedProductPosture,
    /// Recovery actions this case expects.
    #[serde(default)]
    pub expected_recovery_actions: Vec<FailoverRecoveryActionId>,
    /// Proof artifacts cited by the case.
    #[serde(default)]
    pub proof_artifacts: Vec<String>,
    /// Acceptance assertions embedded in the case.
    #[serde(default)]
    pub acceptance_assertions: BTreeMap<String, bool>,
    /// Export-safety declaration for the case.
    pub export_safety: ExportSafety,
}

impl BackupRestoreFailoverRehearsalCase {
    /// Returns true when class, plane, posture, and recovery actions align.
    pub fn aligns_with_taxonomy(&self) -> bool {
        self.primary_plane_class == self.outage_class_id.primary_plane_class()
            && self.expected_product_posture.posture_class
                == expected_posture_for_outage(self.outage_class_id)
            && self
                .expected_recovery_actions
                .iter()
                .all(|action| expected_actions_for_outage(self.outage_class_id).contains(action))
    }
}

/// Observed state of each plane for one rehearsal case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaneObservation {
    /// Local-core observed state token.
    pub local_core_state: String,
    /// Control-plane observed state token.
    pub control_plane_state: String,
    /// Data-plane observed state token.
    pub data_plane_state: String,
    /// Target-authority observed state token.
    pub target_authority_state: String,
    /// Whether the target is reachable.
    pub target_reachable: bool,
}

/// Expected product posture for one rehearsal case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpectedProductPosture {
    /// Expected posture class.
    pub posture_class: FailoverProductPostureClass,
    /// Whether local-core workflows remain available.
    pub local_core_available: bool,
    /// Whether the local-safe baseline must be visible.
    pub must_surface_local_safe_baseline: bool,
    /// Whether cached or optional state must be labeled.
    pub must_label_cached_or_optional_state: bool,
    /// Whether a boundary review is required.
    pub boundary_review_required: bool,
    /// Restore claim class allowed for the case.
    pub restore_claim_class: RestoreClaimClass,
}

/// Product posture class from the failover taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailoverProductPostureClass {
    /// Continue local work with a visible capability limit.
    ContinueLocalWithLimits,
    /// Managed actions are blocked or reviewed.
    ManagedActionsBlockedOrReviewed,
    /// Live runtime is blocked while cached reads are labeled.
    LiveRuntimeBlockedCachedReadsLabeled,
    /// Restore or target-location flow is required.
    RestoreOrLocateTargetRequired,
}

/// Restore claim class from protected rehearsal cases.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreClaimClass {
    /// No restore is implied by the outage.
    NoRestoreImplied,
    /// Last-known-good state is stale evidence only.
    LastKnownGoodIsStaleEvidenceOnly,
    /// Compare-before-restore is required for cached or mirrored data.
    CompareBeforeRestoreRequiredForCachedOrMirrorData,
    /// Exact restore requires matching target evidence.
    ExactRestoreRequiresMatchingTargetEvidence,
}

/// Recovery action identifier declared by the failover taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailoverRecoveryActionId {
    /// Continue local work.
    ContinueLocalWork,
    /// Export the local continuity packet.
    ExportLocalContinuityPacket,
    /// Preserve a checkpoint before repair.
    PreservePreRepairCheckpoint,
    /// Continue locally and label stale authority.
    ContinueLocalAndLabelStaleAuthority,
    /// Open boundary details.
    OpenBoundaryDetails,
    /// Reconnect or reauthenticate after recovery.
    ReconnectOrReauthAfterRecovery,
    /// Export control-plane impairment metadata.
    ExportControlPlaneImpairmentPacket,
    /// Freeze live runtime and preserve local state.
    FreezeLiveRuntimeAndPreserveLocalState,
    /// Run the Project Doctor transport probe.
    RunProjectDoctorTransportProbe,
    /// Compare before restoring data-plane sources.
    CompareBeforeDataRestore,
    /// Export data-plane impairment metadata.
    ExportDataPlaneImpairmentPacket,
    /// Stop live target actions.
    StopLiveTargetActions,
    /// Locate or replace the target identity.
    LocateOrReplaceTargetIdentity,
    /// Restore from a reviewed source.
    RestoreFromReviewedSource,
    /// Escalate when target identity cannot be proved.
    EscalateIfTargetIdentityCannotBeProved,
}

/// Export-safety declaration shared by manifests and cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportSafety {
    /// Whether this export path is metadata-only.
    #[serde(default)]
    pub metadata_only: bool,
    /// Whether raw payload bytes are exported.
    #[serde(default)]
    pub raw_payload_exported: bool,
    /// Whether raw tenant names are exported.
    #[serde(default)]
    pub raw_tenant_names_exported: bool,
    /// Whether raw URLs or hostnames are exported.
    #[serde(default)]
    pub raw_urls_or_hostnames_exported: bool,
    /// Whether raw secret material is exported.
    #[serde(default)]
    pub raw_secret_material_exported: bool,
    /// Whether exact build identity is required.
    #[serde(default)]
    pub exact_build_identity_required: Option<bool>,
    /// Default redaction class when present on taxonomy-derived artifacts.
    #[serde(default)]
    pub redaction_default_class: Option<String>,
}

impl ExportSafety {
    /// Returns true when no raw private material is declared for export.
    pub fn excludes_raw_material(&self) -> bool {
        !self.raw_payload_exported
            && !self.raw_tenant_names_exported
            && !self.raw_urls_or_hostnames_exported
            && !self.raw_secret_material_exported
    }

    /// Returns true when the fixture declares a metadata-only export path.
    pub fn is_metadata_only_safe(&self) -> bool {
        self.metadata_only && self.excludes_raw_material()
    }
}

/// Parsed failover continuity case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailoverContinuityCase {
    /// Optional fixture metadata.
    #[serde(rename = "__fixture__")]
    pub fixture_metadata: ContinuityFixtureMetadata,
    /// Schema references used by the case.
    pub schema_refs: ContinuitySchemaRefs,
    /// Local-safe baseline record.
    pub local_safe_baseline: LocalSafeBaselineRecord,
    /// Failover banner record.
    pub failover_banner: FailoverBannerRecord,
    /// Surface contract assertions embedded in the case.
    #[serde(default)]
    pub expected_surface_contract: BTreeMap<String, bool>,
}

impl FailoverContinuityCase {
    /// Returns true when the banner preserves local-safe and export-safe posture.
    pub fn preserves_local_safe_posture(&self) -> bool {
        self.local_safe_baseline.record_kind == LOCAL_SAFE_BASELINE_RECORD_KIND
            && self.failover_banner.record_kind == FAILOVER_BANNER_RECORD_KIND
            && !self.local_safe_baseline.workflow_rows.is_empty()
            && !self
                .failover_banner
                .retained_local_safe_capabilities
                .is_empty()
            && !self.failover_banner.display_copy.local_safe_invisible
            && !self.failover_banner.display_copy.all_work_broken_implied
            && !self
                .failover_banner
                .display_copy
                .generic_unavailable_banner_used
            && !self
                .failover_banner
                .display_copy
                .queued_and_blocked_collapsed
    }
}

/// Metadata attached to a continuity fixture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityFixtureMetadata {
    /// Fixture name.
    pub name: String,
    /// Fixture scenario summary.
    pub scenario: String,
    /// Contract sections exercised by the fixture.
    #[serde(default)]
    pub contract_sections: Vec<String>,
}

/// Schema refs used by a continuity case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuitySchemaRefs {
    /// Failover banner schema reference.
    pub failover_banner: String,
    /// Local-safe baseline schema reference.
    pub local_safe_baseline: String,
}

/// Local-safe baseline record embedded in a continuity case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSafeBaselineRecord {
    /// Baseline schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable baseline id.
    pub baseline_id: String,
    /// Baseline kind class.
    pub baseline_kind_class: String,
    /// Deployment profile tokens covered by the baseline.
    #[serde(default)]
    pub deployment_profiles: Vec<String>,
    /// Workflow availability rows.
    #[serde(default)]
    pub workflow_rows: Vec<LocalSafeWorkflowRow>,
    /// Related continuity and deployment records.
    pub composes_with: LocalSafeComposesWith,
    /// Reviewer-facing baseline summary.
    pub summary: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Narrative references.
    #[serde(default)]
    pub narrative_refs: Vec<String>,
}

/// One local-safe workflow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSafeWorkflowRow {
    /// Workflow class.
    pub workflow_class: LocalSafeWorkflowClass,
    /// Availability class.
    pub availability_class: LocalSafeAvailabilityClass,
    /// Freshness posture for the row.
    pub freshness_posture_class: FreshnessPostureClass,
    /// User-facing guidance summary.
    pub guidance_summary: String,
}

/// Local-safe workflow class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalSafeWorkflowClass {
    /// Local editing workflow.
    LocalEditing,
    /// Local save workflow.
    LocalSave,
    /// Local undo/redo workflow.
    LocalUndoRedo,
    /// Local search and index query workflow.
    LocalSearchIndexQuery,
    /// Local Git commit and branch workflow.
    LocalGitCommitBranch,
    /// Local build, test, and debug workflow.
    LocalBuildTestDebug,
    /// Local export workflow.
    LocalExportBundle,
    /// Local diagnostics workflow.
    LocalDiagnostics,
    /// Local recent-workspace open workflow.
    LocalOpenRecentWorkspace,
    /// Local docs-pack inspection workflow.
    LocalInspectDocsPack,
    /// Cached provider snapshot inspection workflow.
    LocalInspectCachedProviderSnapshot,
    /// Cached policy snapshot inspection workflow.
    LocalInspectPolicySnapshot,
}

/// Local-safe availability class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalSafeAvailabilityClass {
    /// Workflow is available under local-safe posture.
    AvailableLocalSafe,
    /// Workflow is available with an explicit freshness label.
    AvailableWithFreshnessLabel,
}

/// Freshness posture for local-safe and continuity rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessPostureClass {
    /// Fresh state.
    Fresh,
    /// Bounded stale state.
    BoundedStale,
}

/// Related records composed with a local-safe baseline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalSafeComposesWith {
    /// Local-core continuity packet references.
    #[serde(default)]
    pub local_core_continuity_packet_refs: Vec<String>,
    /// Deployment summary card references.
    #[serde(default)]
    pub deployment_summary_card_refs: Vec<String>,
    /// Connectivity snapshot references.
    #[serde(default)]
    pub connectivity_snapshot_refs: Vec<String>,
}

/// Failover banner record embedded in a continuity case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailoverBannerRecord {
    /// Banner schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable banner id.
    pub banner_id: String,
    /// Banner title.
    pub title: String,
    /// Banner summary.
    pub summary: String,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
    /// Trigger kind class.
    pub trigger_kind_class: FailoverTriggerKindClass,
    /// Impairment source details.
    pub impairment_source: ImpairmentSource,
    /// Impacted features.
    pub impacted_features: ImpactedFeatures,
    /// Local-safe baseline reference.
    pub local_safe_baseline_ref: String,
    /// Retained local-safe capabilities.
    #[serde(default)]
    pub retained_local_safe_capabilities: Vec<String>,
    /// Continuity action rows.
    #[serde(default)]
    pub continuity_action_rows: Vec<ContinuityActionRow>,
    /// Evidence export action.
    pub evidence_export_action: EvidenceExportAction,
    /// Boundary change note.
    pub boundary_change_note: BoundaryChangeNote,
    /// Lifecycle details.
    pub lifecycle: FailoverBannerLifecycle,
    /// Display copy and invariant flags.
    pub display_copy: FailoverDisplayCopy,
    /// Narrative references.
    #[serde(default)]
    pub narrative_refs: Vec<String>,
}

/// Trigger kind for a failover banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailoverTriggerKindClass {
    /// No managed plane is reachable but local-safe work continues.
    LocalSafeOnlyMode,
    /// A planned maintenance window drives the banner.
    PlannedMaintenanceWindow,
    /// Regional failover drives the banner.
    RegionalFailover,
    /// Service-family outage drives the banner.
    ServiceFamilyOutage,
}

/// Impairment source details for a failover banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpairmentSource {
    /// Source kind class.
    pub source_kind_class: ImpairmentSourceKindClass,
    /// Outage notice reference when present.
    #[serde(default)]
    pub outage_notice_ref: Option<String>,
    /// Maintenance notice reference when present.
    #[serde(default)]
    pub maintenance_notice_ref: Option<String>,
    /// Migration event reference when present.
    #[serde(default)]
    pub migration_event_ref: Option<String>,
    /// Connectivity snapshot reference when present.
    #[serde(default)]
    pub connectivity_snapshot_ref: Option<String>,
    /// Outage state class when present.
    #[serde(default)]
    pub outage_state_class: Option<String>,
    /// Maintenance state class when present.
    #[serde(default)]
    pub maintenance_state_class: Option<String>,
    /// Migration state class when present.
    #[serde(default)]
    pub migration_state_class: Option<String>,
    /// Connectivity state class when present.
    #[serde(default)]
    pub connectivity_state_class: Option<String>,
    /// Rationale summary.
    pub rationale_summary: String,
}

/// Source kind for a failover impairment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImpairmentSourceKindClass {
    /// Connectivity state snapshot record.
    ConnectivityStateSnapshotRecord,
    /// Composite source from multiple records.
    CompositeMultipleRecords,
}

/// Impacted feature lists for a failover banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImpactedFeatures {
    /// Control-plane service classes.
    #[serde(default)]
    pub control_plane_service_classes: Vec<String>,
    /// Service-family classes.
    #[serde(default)]
    pub service_family_classes: Vec<String>,
    /// Reviewer-facing feature summary.
    pub feature_summary: String,
}

/// Continuity action row surfaced by a failover banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityActionRow {
    /// Continuity action class.
    pub action_class: ContinuityActionClass,
    /// Continuity action state class.
    pub continuity_action_state_class: ContinuityActionStateClass,
    /// Queue posture class.
    pub queue_posture_class: QueuePostureClass,
    /// Required user step class.
    pub required_user_step_class: RequiredUserStepClass,
    /// Queue or intent reference when present.
    #[serde(default)]
    pub queue_or_intent_ref: Option<String>,
    /// Whether an idempotency key is present.
    pub idempotency_key_present: bool,
    /// Reviewer-facing row summary.
    pub row_summary: String,
}

/// Action class for continuity rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityActionClass {
    /// Idempotent managed field set.
    IdempotentManagedFieldSet,
    /// Managed review-comment publish.
    ManagedReviewCommentPublish,
    /// Telemetry upload.
    TelemetryUpload,
    /// AI gateway prompt submit.
    AiGatewayPromptSubmit,
    /// Support bundle upload.
    SupportBundleUpload,
    /// Paid model dispatch invocation.
    PaidModelDispatchInvocation,
    /// Collaboration role grant or revoke.
    CollaborationRoleGrantOrRevoke,
    /// Provider publish irreversible release.
    ProviderPublishIrreversibleRelease,
    /// Merge queue enqueue.
    MergeQueueEnqueue,
    /// Managed workspace lifecycle write.
    ManagedWorkspaceLifecycleWrite,
    /// Policy admin write.
    PolicyAdminWrite,
    /// Remote execution dispatch.
    RemoteExecutionDispatch,
    /// Auth identity or policy refresh.
    AuthIdentityPolicyRefresh,
}

/// State class for continuity actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityActionStateClass {
    /// Queued for replay.
    QueuedForReplay,
    /// Blocked pending reconnect.
    BlockedPendingReconnect,
    /// Retryable when connected.
    RetryableWhenConnected,
    /// Refused because the action changes authority.
    RefusedAuthorityChanging,
    /// Requires manual rerun.
    RequiresManualRerun,
    /// Blocked pending boundary recheck.
    BlockedPendingBoundaryRecheck,
    /// Blocked because no safe retry exists.
    BlockedNoSafeRetry,
}

/// Queue posture for continuity actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QueuePostureClass {
    /// Idempotent outbox queue.
    IdempotentOutboxQueue,
    /// Publish-later queue.
    PublishLaterQueue,
    /// Upload replication queue.
    UploadReplicationQueue,
    /// No queue is admitted.
    NoQueueAdmitted,
    /// Queue drained with a conflict.
    DrainedWithConflict,
}

/// Required user step for a continuity action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredUserStepClass {
    /// Wait for reconnect.
    WaitForReconnect,
    /// No step is required.
    NoStepRequired,
    /// Wait for a maintenance window to end.
    WaitForWindowEnd,
    /// Reissue with fresh freshness state.
    ReissueWithFreshFreshness,
    /// Review a new boundary.
    ReviewNewBoundary,
    /// Escalate for admin review.
    EscalateForAdminReview,
    /// Reauthenticate.
    Reauthenticate,
}

/// Evidence export action for a failover banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceExportAction {
    /// Evidence action class.
    pub action_class: EvidenceExportActionClass,
    /// Evidence packet reference.
    pub evidence_packet_ref: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Evidence action class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceExportActionClass {
    /// Open a continuity packet.
    OpenContinuityPacket,
    /// Open history.
    OpenHistory,
    /// Open boundary details.
    OpenBoundaryDetails,
    /// Export diagnostics.
    ExportDiagnostics,
}

/// Boundary change note for a failover banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryChangeNote {
    /// Whether a boundary change requires review.
    pub boundary_change_required: bool,
    /// Boundary axes affected by the change.
    #[serde(default)]
    pub boundary_axes_summary: Vec<BoundaryAxisSummary>,
    /// Linked outage or migration event refs.
    #[serde(default)]
    pub linked_outage_or_event_refs: Vec<String>,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Boundary axis summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryAxisSummary {
    /// Boundary axis class.
    pub axis_class: BoundaryAxisClass,
    /// Boundary axis state class.
    pub axis_state_class: BoundaryAxisStateClass,
    /// Reviewer-facing summary.
    pub summary: String,
}

/// Boundary axis class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryAxisClass {
    /// Tenant boundary.
    Tenant,
    /// Region boundary.
    Region,
    /// Residency boundary.
    Residency,
    /// Key ownership boundary.
    KeyOwnership,
    /// Endpoint identity boundary.
    EndpointIdentity,
}

/// Boundary axis state class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryAxisStateClass {
    /// Boundary is unchanged.
    Unchanged,
    /// Boundary changed.
    Changed,
    /// Boundary is unknown and requires recheck.
    UnknownRecheckRequired,
}

/// Lifecycle details for a failover banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailoverBannerLifecycle {
    /// Freshness class for the banner.
    pub freshness_class: String,
    /// Superseded banner id when present.
    #[serde(default)]
    pub supersedes_banner_id: Option<String>,
    /// Replacing banner id when present.
    #[serde(default)]
    pub superseded_by_banner_id: Option<String>,
    /// Retention timestamp when present.
    #[serde(default)]
    pub retained_until_at: Option<String>,
    /// Historical banner refs.
    #[serde(default)]
    pub history_banner_refs: Vec<String>,
}

/// Display copy and invariant flags for a failover banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailoverDisplayCopy {
    /// Primary status line.
    pub primary_status_line: String,
    /// Local-safe line.
    pub local_safe_line: String,
    /// Queued or retry line.
    pub queued_or_retry_line: String,
    /// Blocked or manual line.
    pub blocked_or_manual_line: String,
    /// Evidence export line.
    pub evidence_export_line: String,
    /// Boundary change line.
    pub boundary_change_line: String,
    /// Stale label when present.
    #[serde(default)]
    pub stale_label: Option<String>,
    /// Whether the banner implies all work is broken.
    pub all_work_broken_implied: bool,
    /// Whether a generic unavailable banner is used.
    pub generic_unavailable_banner_used: bool,
    /// Whether queued and blocked states are collapsed.
    pub queued_and_blocked_collapsed: bool,
    /// Whether local-safe posture is invisible.
    pub local_safe_invisible: bool,
    /// Whether incident language is used for planned work.
    pub incident_language_for_planned_used: bool,
}

/// Full checked-in corpus for failover alpha validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailoverAlphaCorpus {
    /// Backup/checkpoint class artifact.
    pub backup_checkpoint_classes: BackupCheckpointClasses,
    /// Protected rehearsal manifest.
    pub rehearsal_manifest: BackupRestoreFailoverRehearsalManifest,
    /// Protected rehearsal cases loaded from the manifest.
    pub rehearsal_cases: Vec<RehearsalCaseEntry>,
    /// Failover continuity cases loaded from the continuity directory.
    pub continuity_cases: Vec<ContinuityCaseEntry>,
}

impl FailoverAlphaCorpus {
    /// Validates fixture coverage and typed taxonomy alignment.
    pub fn validate(&self) -> Vec<FailoverAlphaViolation> {
        let mut violations = self.backup_checkpoint_classes.validate();
        validate_manifest(&self.rehearsal_manifest, &mut violations);
        validate_rehearsal_cases(
            &self.rehearsal_manifest,
            &self.rehearsal_cases,
            &mut violations,
        );
        validate_continuity_cases(&self.continuity_cases, &mut violations);
        violations
    }
}

/// One protected rehearsal case with its manifest row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RehearsalCaseEntry {
    /// Repository-relative fixture path.
    pub fixture_ref: String,
    /// Manifest row that loaded this case.
    pub manifest_ref: RehearsalCaseFileRef,
    /// Parsed case body.
    pub case: BackupRestoreFailoverRehearsalCase,
}

/// One failover continuity case with its fixture path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinuityCaseEntry {
    /// Repository-relative fixture path.
    pub fixture_ref: String,
    /// Parsed continuity case.
    pub case: FailoverContinuityCase,
}

/// Validation failure emitted by the failover alpha consumer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FailoverAlphaViolation {
    /// Stable check id.
    pub check_id: String,
    /// Subject reference that failed validation.
    pub subject_ref: String,
    /// Reviewer-facing message.
    pub message: String,
}

fn validate_manifest(
    manifest: &BackupRestoreFailoverRehearsalManifest,
    violations: &mut Vec<FailoverAlphaViolation>,
) {
    if manifest.schema_version != 1 {
        push_violation(
            violations,
            "rehearsal_manifest.schema_version",
            &manifest.manifest_id,
            "schema_version must be 1",
        );
    }
    if manifest.record_kind != BACKUP_RESTORE_FAILOVER_REHEARSAL_MANIFEST_RECORD_KIND {
        push_violation(
            violations,
            "rehearsal_manifest.record_kind",
            &manifest.manifest_id,
            "manifest record_kind does not match the protected rehearsal manifest",
        );
    }
    if manifest.status != "protected" {
        push_violation(
            violations,
            "rehearsal_manifest.status",
            &manifest.manifest_id,
            "rehearsal manifest must stay protected",
        );
    }

    let required = OutageClass::ALL.into_iter().collect::<BTreeSet<_>>();
    let declared = manifest
        .required_outage_class_ids
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    if declared != required {
        push_violation(
            violations,
            "rehearsal_manifest.required_outage_class_ids",
            &manifest.manifest_id,
            "manifest must require exactly the protected outage classes",
        );
    }
    if manifest.covered_outage_classes() != required {
        push_violation(
            violations,
            "rehearsal_manifest.case_class_coverage",
            &manifest.manifest_id,
            "case files must cover every protected outage class",
        );
    }
    if !manifest.export_safety.excludes_raw_material() {
        push_violation(
            violations,
            "rehearsal_manifest.export_safety",
            &manifest.manifest_id,
            "manifest export safety must exclude raw private material",
        );
    }
}

fn validate_rehearsal_cases(
    manifest: &BackupRestoreFailoverRehearsalManifest,
    entries: &[RehearsalCaseEntry],
    violations: &mut Vec<FailoverAlphaViolation>,
) {
    if entries.len() != manifest.case_files.len() {
        push_violation(
            violations,
            "rehearsal_cases.count",
            &manifest.manifest_id,
            "loaded rehearsal case count must match the manifest",
        );
    }

    let covered = entries
        .iter()
        .map(|entry| entry.case.outage_class_id)
        .collect::<BTreeSet<_>>();
    let required = OutageClass::ALL.into_iter().collect::<BTreeSet<_>>();
    if covered != required {
        push_violation(
            violations,
            "rehearsal_cases.coverage",
            &manifest.manifest_id,
            "loaded rehearsal cases must cover every protected outage class",
        );
    }

    for entry in entries {
        let case = &entry.case;
        if case.schema_version != 1 {
            push_violation(
                violations,
                "rehearsal_case.schema_version",
                &entry.fixture_ref,
                "case schema_version must be 1",
            );
        }
        if case.record_kind != BACKUP_RESTORE_FAILOVER_REHEARSAL_CASE_RECORD_KIND {
            push_violation(
                violations,
                "rehearsal_case.record_kind",
                &entry.fixture_ref,
                "case record_kind does not match the protected rehearsal case",
            );
        }
        if case.outage_class_id != entry.manifest_ref.outage_class_id {
            push_violation(
                violations,
                "rehearsal_case.outage_class_id",
                &entry.fixture_ref,
                "case outage class must match the manifest row",
            );
        }
        if case.primary_plane_class != entry.manifest_ref.expected_primary_plane_class {
            push_violation(
                violations,
                "rehearsal_case.primary_plane_class",
                &entry.fixture_ref,
                "case primary plane must match the manifest row",
            );
        }
        if !case.aligns_with_taxonomy() {
            push_violation(
                violations,
                "rehearsal_case.taxonomy_alignment",
                &entry.fixture_ref,
                "case class, plane, posture, and recovery actions must align with the typed taxonomy",
            );
        }
        if !case.expected_product_posture.local_core_available {
            push_violation(
                violations,
                "rehearsal_case.local_core_available",
                &entry.fixture_ref,
                "local core must remain available in every protected rehearsal case",
            );
        }
        if !case
            .expected_product_posture
            .must_surface_local_safe_baseline
        {
            push_violation(
                violations,
                "rehearsal_case.local_safe_baseline_hidden",
                &entry.fixture_ref,
                "protected rehearsal cases must surface the local-safe baseline",
            );
        }
        if case.expected_recovery_actions.is_empty() {
            push_violation(
                violations,
                "rehearsal_case.expected_recovery_actions",
                &entry.fixture_ref,
                "case must exercise at least one recovery action",
            );
        }
        if case
            .acceptance_assertions
            .get("exact_build_identity_required")
            != Some(&true)
        {
            push_violation(
                violations,
                "rehearsal_case.exact_build_identity_required",
                &entry.fixture_ref,
                "case must require exact build identity",
            );
        }
        if !case.export_safety.is_metadata_only_safe() {
            push_violation(
                violations,
                "rehearsal_case.export_safety",
                &entry.fixture_ref,
                "case export safety must be metadata-only and exclude raw private material",
            );
        }
        if case.outage_class_id == OutageClass::FullTargetLoss
            && case.plane_observation.target_reachable
        {
            push_violation(
                violations,
                "rehearsal_case.full_target_loss.target_reachable",
                &entry.fixture_ref,
                "full target loss must mark the target unreachable",
            );
        }
    }
}

fn validate_continuity_cases(
    entries: &[ContinuityCaseEntry],
    violations: &mut Vec<FailoverAlphaViolation>,
) {
    if entries.is_empty() {
        push_violation(
            violations,
            "continuity_cases.empty",
            FAILOVER_CONTINUITY_CASES_DIR,
            "at least one continuity case must be loaded",
        );
    }

    let mut names = BTreeSet::new();
    for entry in entries {
        let case = &entry.case;
        names.insert(case.fixture_metadata.name.as_str());
        if case.local_safe_baseline.schema_version != 1 {
            push_violation(
                violations,
                "continuity_case.local_safe_baseline.schema_version",
                &entry.fixture_ref,
                "local-safe baseline schema_version must be 1",
            );
        }
        if case.failover_banner.schema_version != 1 {
            push_violation(
                violations,
                "continuity_case.failover_banner.schema_version",
                &entry.fixture_ref,
                "failover banner schema_version must be 1",
            );
        }
        if !case.preserves_local_safe_posture() {
            push_violation(
                violations,
                "continuity_case.local_safe_posture",
                &entry.fixture_ref,
                "continuity case must preserve local-safe posture and distinct queued/blocked states",
            );
        }
        if case.failover_banner.local_safe_baseline_ref != case.local_safe_baseline.baseline_id {
            push_violation(
                violations,
                "continuity_case.baseline_ref_mismatch",
                &entry.fixture_ref,
                "failover banner must reference the embedded local-safe baseline",
            );
        }
        if case
            .failover_banner
            .boundary_change_note
            .boundary_change_required
            && case
                .failover_banner
                .boundary_change_note
                .boundary_axes_summary
                .is_empty()
        {
            push_violation(
                violations,
                "continuity_case.boundary_axes_missing",
                &entry.fixture_ref,
                "boundary-changing continuity cases must name affected axes",
            );
        }
    }

    for required_name in [
        "local_safe_only_mode",
        "partial_queue_retry_continuity",
        "regional_failover_changed_boundary",
        "service_family_outage",
    ] {
        if !names.contains(required_name) {
            push_violation(
                violations,
                "continuity_cases.required_fixture_missing",
                FAILOVER_CONTINUITY_CASES_DIR,
                format!("missing required continuity fixture {required_name}"),
            );
        }
    }
}

fn expected_posture_for_outage(outage_class: OutageClass) -> FailoverProductPostureClass {
    match outage_class {
        OutageClass::LocalCoreContinuity => FailoverProductPostureClass::ContinueLocalWithLimits,
        OutageClass::ControlPlaneImpairment => {
            FailoverProductPostureClass::ManagedActionsBlockedOrReviewed
        }
        OutageClass::DataPlaneImpairment => {
            FailoverProductPostureClass::LiveRuntimeBlockedCachedReadsLabeled
        }
        OutageClass::FullTargetLoss => FailoverProductPostureClass::RestoreOrLocateTargetRequired,
    }
}

fn expected_actions_for_outage(outage_class: OutageClass) -> &'static [FailoverRecoveryActionId] {
    match outage_class {
        OutageClass::LocalCoreContinuity => &[
            FailoverRecoveryActionId::ContinueLocalWork,
            FailoverRecoveryActionId::ExportLocalContinuityPacket,
            FailoverRecoveryActionId::PreservePreRepairCheckpoint,
        ],
        OutageClass::ControlPlaneImpairment => &[
            FailoverRecoveryActionId::ContinueLocalAndLabelStaleAuthority,
            FailoverRecoveryActionId::OpenBoundaryDetails,
            FailoverRecoveryActionId::ReconnectOrReauthAfterRecovery,
            FailoverRecoveryActionId::ExportControlPlaneImpairmentPacket,
        ],
        OutageClass::DataPlaneImpairment => &[
            FailoverRecoveryActionId::FreezeLiveRuntimeAndPreserveLocalState,
            FailoverRecoveryActionId::RunProjectDoctorTransportProbe,
            FailoverRecoveryActionId::CompareBeforeDataRestore,
            FailoverRecoveryActionId::ExportDataPlaneImpairmentPacket,
        ],
        OutageClass::FullTargetLoss => &[
            FailoverRecoveryActionId::StopLiveTargetActions,
            FailoverRecoveryActionId::LocateOrReplaceTargetIdentity,
            FailoverRecoveryActionId::RestoreFromReviewedSource,
            FailoverRecoveryActionId::EscalateIfTargetIdentityCannotBeProved,
        ],
    }
}

fn read_yaml_path<T>(path: &Path) -> Result<T, FailoverAlphaLoadError>
where
    T: for<'de> Deserialize<'de>,
{
    let yaml = fs::read_to_string(path).map_err(|source| FailoverAlphaLoadError::Read {
        path: path.to_path_buf(),
        source,
    })?;
    serde_yaml::from_str(&yaml).map_err(|source| FailoverAlphaLoadError::Parse {
        path: path.to_path_buf(),
        source,
    })
}

fn set_from_copied<'a, T>(iter: impl Iterator<Item = &'a T>) -> BTreeSet<T>
where
    T: 'a + Copy + Ord,
{
    iter.copied().collect()
}

fn repo_relative_path(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn push_violation(
    violations: &mut Vec<FailoverAlphaViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(FailoverAlphaViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}
