//! Project-entry review records that join entry sheets with admission checkpoints.
//!
//! The entry module is the workspace-level projection used by shell, CLI,
//! drag/drop, deep-link, and support surfaces when a user opens, clones,
//! imports, adds a root, or restores work. It builds on the lower-level
//! admission packet and checkpoint route types so every entry verb remains
//! distinct before writes, trust review, setup, or restore rehydration can run.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::admission::checkpoint::{
    build_admission_checkpoint_route, AdmissionCheckpointBuildRequest,
    AdmissionCheckpointRouteRecord, ArchetypeTruth, BlockedReasonClass, ContinueWithoutClass,
    DetectionConfidenceClass, DetectionOutcome, DetectionSignal, DetectionSignalSourceClass,
    DetectorState, ExecutionBoundary, FirstUsefulEntrySource, OptionalReasonClass, ReadinessBucket,
    ReadinessBuckets, ReadinessTask, ReadinessTaskClass, ReadinessTaskState, SideEffectClass,
    SignalMaterialEffect, SupportClaimClass, TrustReviewClass,
};
use crate::{
    review_entry_admission, AdmissionAction, AdmissionReviewPacket, AdmissionReviewRequest,
    AdmissionSourceSurface, CertificatePosture, CleanupPosture, CloneAuthMode, EntryVerb,
    ImportAction, ImportArtifactClass, LfsPosture, RefChoice, ResultingMode, SubmodulePosture,
    TargetIdentityClass, TargetKind, TrustState, ADMISSION_REVIEW_SCHEMA_VERSION,
};

macro_rules! impl_as_str {
    ($ty:ty { $($variant:ident => $value:literal),+ $(,)? }) => {
        impl $ty {
            /// Returns the stable snake_case token for this vocabulary value.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $value,)+
                }
            }
        }
    };
}

/// Schema version for [`ProjectEntryReviewRecord`].
pub const ENTRY_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Identifies a `project_entry_review_record`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectEntryReviewRecordKind {
    /// `project_entry_review_record`.
    ProjectEntryReviewRecord,
}

/// Request for building a complete entry review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectEntryReviewRequest {
    /// Surface that initiated the entry activation.
    pub source_surface: AdmissionSourceSurface,
    /// Entry verb selected by the user or handoff.
    pub entry_verb: EntryVerb,
    /// Target kind selected or resolved by the entry surface.
    pub target_kind: TargetKind,
    /// Resulting mode previewed before commit.
    pub resulting_mode: ResultingMode,
    /// Redaction-aware source input or target label.
    pub target_specifier: String,
    /// Destination path, workspace, or restore target selected for the flow.
    #[serde(default)]
    pub destination: Option<String>,
    /// Active workspace label when the flow mutates the current workspace.
    #[serde(default)]
    pub active_workspace_label: Option<String>,
    /// Proxy, mirror, offline, or air-gapped route label.
    #[serde(default)]
    pub network_route_label: Option<String>,
    /// Clone-specific choices supplied by a clone review sheet.
    #[serde(default)]
    pub clone_options: CloneReviewOptions,
    /// Import-specific choices supplied by an import review sheet.
    #[serde(default)]
    pub import_options: ImportReviewOptions,
    /// Known destination facts used by collision review.
    #[serde(default)]
    pub destination_facts: EntryDestinationFacts,
    /// Last failed diagnostic text, already safe to redact again.
    #[serde(default)]
    pub failed_diagnostic_text: Option<String>,
}

impl ProjectEntryReviewRequest {
    /// Builds a request with the required entry fields.
    pub fn new(
        source_surface: AdmissionSourceSurface,
        entry_verb: EntryVerb,
        target_kind: TargetKind,
        resulting_mode: ResultingMode,
        target_specifier: impl Into<String>,
    ) -> Self {
        Self {
            source_surface,
            entry_verb,
            target_kind,
            resulting_mode,
            target_specifier: target_specifier.into(),
            destination: None,
            active_workspace_label: None,
            network_route_label: None,
            clone_options: CloneReviewOptions::default(),
            import_options: ImportReviewOptions::default(),
            destination_facts: EntryDestinationFacts::default(),
            failed_diagnostic_text: None,
        }
    }

    /// Sets the reviewed destination label.
    pub fn with_destination(mut self, destination: impl Into<String>) -> Self {
        self.destination = Some(destination.into());
        self
    }

    /// Sets the active workspace label.
    pub fn with_active_workspace(mut self, active_workspace_label: impl Into<String>) -> Self {
        self.active_workspace_label = Some(active_workspace_label.into());
        self
    }

    /// Sets the route label for mirrors, proxies, or offline media.
    pub fn with_network_route(mut self, network_route_label: impl Into<String>) -> Self {
        self.network_route_label = Some(network_route_label.into());
        self
    }

    /// Sets clone-review options.
    pub fn with_clone_options(mut self, clone_options: CloneReviewOptions) -> Self {
        self.clone_options = clone_options;
        self
    }

    /// Sets import-review options.
    pub fn with_import_options(mut self, import_options: ImportReviewOptions) -> Self {
        self.import_options = import_options;
        self
    }

    /// Sets destination facts for collision review.
    pub fn with_destination_facts(mut self, destination_facts: EntryDestinationFacts) -> Self {
        self.destination_facts = destination_facts;
        self
    }

    /// Sets failed diagnostic text for repair-state projection.
    pub fn with_failed_diagnostic_text(
        mut self,
        failed_diagnostic_text: impl Into<String>,
    ) -> Self {
        self.failed_diagnostic_text = Some(failed_diagnostic_text.into());
        self
    }
}

/// Clone-specific choices supplied before network activity begins.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneReviewOptions {
    /// Branch or ref label shown in the review sheet.
    pub branch_or_ref_label: String,
    /// Ref-choice posture.
    pub ref_choice: RefChoice,
    /// Clone depth and filtering posture.
    pub clone_depth_class: CloneDepthClass,
    /// Optional partial-clone filter label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_filter_label: Option<String>,
    /// Submodule posture for the initial clone.
    pub submodule_posture: SubmodulePosture,
    /// LFS posture for the initial clone.
    pub lfs_posture: LfsPosture,
    /// Optional post-clone action override.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_clone_action: Option<AdmissionAction>,
}

impl Default for CloneReviewOptions {
    fn default() -> Self {
        Self {
            branch_or_ref_label: "HEAD (unresolved until remote query)".to_string(),
            ref_choice: RefChoice::UnresolvedUntilRemoteQuery,
            clone_depth_class: CloneDepthClass::FullHistory,
            partial_filter_label: None,
            submodule_posture: SubmodulePosture::DetectOnly,
            lfs_posture: LfsPosture::DetectOnly,
            post_clone_action: None,
        }
    }
}

/// Clone depth or filter posture previewed before clone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CloneDepthClass {
    /// Fetch full history.
    FullHistory,
    /// Fetch a bounded shallow history.
    ShallowDepth,
    /// Use a partial-clone filter such as `blob:none`.
    PartialCloneFiltered,
    /// Acquire through a mirror-first route.
    MirrorFirst,
    /// Acquire from offline or air-gapped media.
    OfflineBundle,
}

impl_as_str!(CloneDepthClass {
    FullHistory => "full_history",
    ShallowDepth => "shallow_depth",
    PartialCloneFiltered => "partial_clone_filtered",
    MirrorFirst => "mirror_first",
    OfflineBundle => "offline_bundle",
});

/// Import-specific choices supplied before extraction or restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportReviewOptions {
    /// Schema, producer, or version label shown in the review sheet.
    pub schema_or_version_label: String,
    /// Lossy mapping posture.
    pub lossy_mapping_class: ImportLossyMappingClass,
    /// Machine-local exclusions shown before commit.
    pub machine_local_exclusions: Vec<String>,
    /// Cleanup posture for staging or extracted content.
    pub cleanup_posture: CleanupPosture,
}

impl Default for ImportReviewOptions {
    fn default() -> Self {
        Self {
            schema_or_version_label: "schema or producer requires review".to_string(),
            lossy_mapping_class: ImportLossyMappingClass::UnknownUntilInspection,
            machine_local_exclusions: vec![
                "live auth tokens".to_string(),
                "machine-local trust anchors".to_string(),
                "runtime handles".to_string(),
            ],
            cleanup_posture: CleanupPosture::RetainLabelledStaging,
        }
    }
}

/// Lossy mapping posture for imports and portable-state packages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportLossyMappingClass {
    /// No lossy mapping is expected.
    NoLossyMapping,
    /// Lossy mapping exists and requires review.
    LossyWithReview,
    /// Schema migration is compatible but disclosed.
    SchemaMigratedCompatible,
    /// Competitor configuration maps only partially.
    CompetitorMappingPartial,
    /// Manual review is required before applying.
    ManualReviewRequired,
    /// Inspection has not yet resolved mapping fidelity.
    UnknownUntilInspection,
}

impl_as_str!(ImportLossyMappingClass {
    NoLossyMapping => "no_lossy_mapping",
    LossyWithReview => "lossy_with_review",
    SchemaMigratedCompatible => "schema_migrated_compatible",
    CompetitorMappingPartial => "competitor_mapping_partial",
    ManualReviewRequired => "manual_review_required",
    UnknownUntilInspection => "unknown_until_inspection",
});

/// Known destination facts that cannot be inferred from the path alone.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryDestinationFacts {
    /// Previously cloned target reference when the destination matches history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previously_cloned_target_ref: Option<String>,
    /// Whether policy blocks writing to the chosen destination.
    #[serde(default)]
    pub policy_blocked: bool,
}

/// Complete project-entry review record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProjectEntryReviewRecord {
    /// Stable record kind.
    pub record_kind: ProjectEntryReviewRecordKind,
    /// Schema version for this record.
    pub entry_review_schema_version: u32,
    /// Opaque review id.
    pub entry_review_id: String,
    /// Surface that initiated the entry activation.
    pub source_surface: AdmissionSourceSurface,
    /// Entry verb being reviewed.
    pub entry_verb: EntryVerb,
    /// Target kind being reviewed.
    pub target_kind: TargetKind,
    /// Resulting mode reviewed before commit.
    pub resulting_mode: ResultingMode,
    /// Redaction-aware vocabulary summary for source, trust, destination, write scope, and next step.
    pub vocabulary_review: EntryVocabularyReview,
    /// Verb-specific review sheet.
    pub review_sheet: EntryReviewSheet,
    /// Collision sheet when the destination requires a deliberate choice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_collision_review: Option<EntryDestinationCollisionReview>,
    /// Post-entry handoff card.
    pub post_entry_handoff_card: EntryPostEntryHandoffCard,
    /// Failed-attempt repair state.
    pub failure_repair_state: EntryFailureRepairState,
    /// Cross-surface parity rows that preserve the verb and review requirement.
    pub surface_parity: Vec<EntrySurfaceParity>,
    /// Existing admission packet consumed by the checkpoint.
    pub admission_review_packet: AdmissionReviewPacket,
    /// Post-entry admission checkpoint and route record.
    pub admission_checkpoint_route: AdmissionCheckpointRouteRecord,
    /// Compact summary for support and CLI surfaces.
    pub summary: String,
}

impl ProjectEntryReviewRecord {
    /// Returns contract findings; an empty list means the record obeys entry invariants.
    pub fn contract_findings(&self) -> Vec<String> {
        let mut findings = Vec::new();
        if self.entry_review_schema_version != ENTRY_REVIEW_SCHEMA_VERSION {
            findings.push("entry review schema version mismatch".to_string());
        }
        if self.admission_review_packet.admission_review_schema_version
            != ADMISSION_REVIEW_SCHEMA_VERSION
        {
            findings.push("admission review schema version mismatch".to_string());
        }
        if self.review_sheet.review_sheet_kind != sheet_kind_for(self.entry_verb, self.target_kind)
        {
            findings.push("review sheet kind does not match entry verb".to_string());
        }
        if self.entry_verb == EntryVerb::Clone {
            let Some(clone) = self.review_sheet.clone_review.as_ref() else {
                findings.push("clone entry must carry clone review".to_string());
                return findings;
            };
            if clone.normalized_remote_url_label.contains('@') {
                findings.push("clone remote label must not expose credentials".to_string());
            }
            if !clone.clone_never_grants_trust
                || !clone.dependency_restore_deferred
                || !clone.task_execution_deferred
            {
                findings.push("clone must defer trust, dependencies, and tasks".to_string());
            }
        }
        if self.entry_verb == EntryVerb::Import {
            let Some(import) = self.review_sheet.import_review.as_ref() else {
                findings.push("import entry must carry import review".to_string());
                return findings;
            };
            if import.inspect_only
                && import.write_behavior_class != ImportWriteBehaviorClass::InspectOnlyNoWrite
            {
                findings.push("inspect-only import must advertise no write".to_string());
            }
            if !import.no_durable_write_before_review || !import.no_state_rehydration_before_review
            {
                findings.push("import must defer durable write and state rehydration".to_string());
            }
        }
        if let Some(collision) = self.destination_collision_review.as_ref() {
            if collision.collision_class != EntryDestinationCollisionClass::NoCollision
                && !collision.requires_explicit_choice
            {
                findings.push("destination collision requires explicit choice".to_string());
            }
        }
        if !self.failure_repair_state.typed_source_input_preserved
            || !self.failure_repair_state.chosen_destination_preserved
            || !self.failure_repair_state.redacted_diagnostics_preserved
        {
            findings.push(
                "failed entry repair state must preserve inputs and redacted diagnostics"
                    .to_string(),
            );
        }
        for parity in &self.surface_parity {
            if parity.entry_verb != self.entry_verb
                || parity.target_kind != self.target_kind
                || parity.resulting_mode != self.resulting_mode
            {
                findings.push(format!(
                    "surface parity drift on {}",
                    parity.source_surface.as_str()
                ));
            }
        }
        findings.extend(self.admission_checkpoint_route.contract_findings());
        findings
    }

    /// Returns true when the record obeys entry invariants.
    pub fn is_contract_valid(&self) -> bool {
        self.contract_findings().is_empty()
    }

    /// Returns compact support rows for the entry review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "entry_review: {} verb={} target={} mode={} sheet={}",
                self.entry_review_id,
                self.entry_verb.as_str(),
                self.target_kind.as_str(),
                self.resulting_mode.as_str(),
                self.review_sheet.review_sheet_kind.as_str()
            ),
            format!(
                "vocabulary: source={} trust={} destination={} write_scope={} next_step={}",
                self.vocabulary_review.source_access_class.as_str(),
                self.vocabulary_review.trust_label,
                self.vocabulary_review.destination_label,
                self.vocabulary_review.write_scope_label,
                self.vocabulary_review.next_step_label
            ),
        ];
        if let Some(collision) = self.destination_collision_review.as_ref() {
            lines.push(format!(
                "collision: {} explicit_choice={}",
                collision.collision_class.as_str(),
                collision.requires_explicit_choice
            ));
        }
        lines.push(format!(
            "handoff: primary={} not_yet_done={}",
            self.post_entry_handoff_card.primary_next_action.as_str(),
            self.post_entry_handoff_card.not_yet_done.len()
        ));
        lines
    }
}

/// Review of the shared source/trust/destination/write-scope/next-step vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryVocabularyReview {
    /// Source access posture.
    pub source_access_class: EntrySourceAccessClass,
    /// Source label safe for review surfaces.
    pub source_label: String,
    /// Trust posture label.
    pub trust_label: String,
    /// Destination label safe for review surfaces.
    pub destination_label: String,
    /// Write-scope label.
    pub write_scope_label: String,
    /// Next-step label.
    pub next_step_label: String,
}

/// Source access posture shared by online, mirror, offline, and air-gapped entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntrySourceAccessClass {
    /// Online direct source.
    DirectOnline,
    /// Mirror or proxy is the acquisition source.
    MirrorFirst,
    /// Offline snapshot source.
    OfflineSnapshot,
    /// Air-gapped media source.
    AirGappedMedia,
    /// Local filesystem source.
    LocalFilesystem,
}

impl_as_str!(EntrySourceAccessClass {
    DirectOnline => "direct_online",
    MirrorFirst => "mirror_first",
    OfflineSnapshot => "offline_snapshot",
    AirGappedMedia => "air_gapped_media",
    LocalFilesystem => "local_filesystem",
});

/// Verb-specific entry review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryReviewSheet {
    /// Review sheet kind.
    pub review_sheet_kind: EntryReviewSheetKind,
    /// Open-target review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_review: Option<OpenEntryReviewSheet>,
    /// Open-workspace review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_workspace_review: Option<OpenWorkspaceReviewSheet>,
    /// Clone review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_review: Option<CloneEntryReviewSheet>,
    /// Add-root review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_root_review: Option<AddRootEntryReviewSheet>,
    /// Import review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_review: Option<ImportEntryReviewSheet>,
    /// Restore review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_review: Option<RestoreEntryReviewSheet>,
}

/// Entry review sheet kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryReviewSheetKind {
    /// Open a local file, folder, or repository root.
    OpenLocalTarget,
    /// Open a workspace or workset manifest.
    OpenWorkspaceManifest,
    /// Clone a remote repository.
    CloneRepository,
    /// Add a root to the active workspace.
    AddRoot,
    /// Import an archive, handoff, or portable-state package.
    ImportArtifact,
    /// Restore a prior session or checkpoint.
    RestoreState,
}

impl_as_str!(EntryReviewSheetKind {
    OpenLocalTarget => "open_local_target",
    OpenWorkspaceManifest => "open_workspace_manifest",
    CloneRepository => "clone_repository",
    AddRoot => "add_root",
    ImportArtifact => "import_artifact",
    RestoreState => "restore_state",
});

/// Review sheet for local open flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenEntryReviewSheet {
    /// Redaction-aware target label.
    pub target_label: String,
    /// Target identity class.
    pub target_identity_class: TargetIdentityClass,
    /// Whether a nested repository was detected.
    pub nested_repo_review_required: bool,
    /// Whether a workspace manifest was detected near the target.
    pub workspace_manifest_choice_available: bool,
    /// Whether opening writes project bytes before admission.
    pub writes_files_before_admission: bool,
    /// Whether setup stays deferred.
    pub setup_deferred: bool,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Review sheet for workspace or workset manifest open flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenWorkspaceReviewSheet {
    /// Redaction-aware manifest label.
    pub manifest_label: String,
    /// Schema or version label.
    pub schema_or_version_label: String,
    /// Missing-root posture label.
    pub missing_root_posture_label: String,
    /// Whether silent schema upgrade is forbidden.
    pub silent_upgrade_forbidden: bool,
    /// Whether dropped meaning must remain visible.
    pub dropped_meaning_disclosed: bool,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Review sheet for clone flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CloneEntryReviewSheet {
    /// Redaction-aware normalized remote URL.
    pub normalized_remote_url_label: String,
    /// Remote host label.
    pub host_label: String,
    /// Host key or certificate posture.
    pub certificate_posture: CertificatePosture,
    /// Authentication posture.
    pub auth_mode: CloneAuthMode,
    /// Branch or ref label.
    pub branch_or_ref_label: String,
    /// Branch or ref posture.
    pub ref_choice: RefChoice,
    /// Clone depth posture.
    pub clone_depth_class: CloneDepthClass,
    /// Optional partial-clone filter label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_filter_label: Option<String>,
    /// Submodule posture.
    pub submodule_posture: SubmodulePosture,
    /// LFS posture.
    pub lfs_posture: LfsPosture,
    /// Destination path label.
    pub destination_path_label: String,
    /// Route posture.
    pub source_access_class: EntrySourceAccessClass,
    /// Route note.
    pub route_note: String,
    /// Post-clone action.
    pub post_clone_action: AdmissionAction,
    /// Explicit actions offered.
    pub explicit_actions: Vec<AdmissionAction>,
    /// Whether clone never grants trust.
    pub clone_never_grants_trust: bool,
    /// Whether dependency restore stays deferred.
    pub dependency_restore_deferred: bool,
    /// Whether tasks and hooks stay deferred.
    pub task_execution_deferred: bool,
}

/// Review sheet for add-root flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AddRootEntryReviewSheet {
    /// Active workspace label.
    pub active_workspace_label: String,
    /// New root label.
    pub new_root_label: String,
    /// Whether per-root trust review is required.
    pub per_root_trust_review_required: bool,
    /// Whether workset, search, and watcher scope changes are reviewed.
    pub scope_change_review_required: bool,
    /// Whether sibling root trust inheritance is forbidden.
    pub sibling_trust_inheritance_forbidden: bool,
    /// Whether a checkpoint or undo group is required.
    pub checkpoint_required: bool,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Review sheet for import flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportEntryReviewSheet {
    /// Artifact class.
    pub artifact_class: ImportArtifactClass,
    /// Import action.
    pub import_action: ImportAction,
    /// Inspect or write behavior.
    pub write_behavior_class: ImportWriteBehaviorClass,
    /// Schema, producer, or version label.
    pub schema_or_version_label: String,
    /// Extraction or restore target label.
    pub extraction_or_restore_target_label: String,
    /// Lossy mapping posture.
    pub lossy_mapping_class: ImportLossyMappingClass,
    /// Machine-local exclusions.
    pub machine_local_exclusions: Vec<String>,
    /// Cleanup posture.
    pub cleanup_posture: CleanupPosture,
    /// Whether the import is inspect-only.
    pub inspect_only: bool,
    /// Whether no durable write occurs before review.
    pub no_durable_write_before_review: bool,
    /// Whether no state rehydration occurs before review.
    pub no_state_rehydration_before_review: bool,
}

/// Import write behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportWriteBehaviorClass {
    /// Inspect-only, no write.
    InspectOnlyNoWrite,
    /// Write to labelled staging.
    WriteToLabelledStaging,
    /// Write to a reviewed destination.
    WriteToReviewedDestination,
    /// Apply to the active workspace.
    ApplyToActiveWorkspace,
    /// Restore after compare.
    RestoreAfterCompare,
}

impl_as_str!(ImportWriteBehaviorClass {
    InspectOnlyNoWrite => "inspect_only_no_write",
    WriteToLabelledStaging => "write_to_labelled_staging",
    WriteToReviewedDestination => "write_to_reviewed_destination",
    ApplyToActiveWorkspace => "apply_to_active_workspace",
    RestoreAfterCompare => "restore_after_compare",
});

/// Review sheet for restore flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreEntryReviewSheet {
    /// Restore source label.
    pub restore_source_label: String,
    /// Schema, checkpoint, or version label.
    pub schema_or_checkpoint_label: String,
    /// Restore target label.
    pub restore_target_label: String,
    /// Retained state classes.
    pub retained_state_classes: Vec<String>,
    /// Replaced state classes.
    pub replaced_state_classes: Vec<String>,
    /// Whether a checkpoint is required before overwrite.
    pub checkpoint_required_before_overwrite: bool,
    /// Whether side-effect rerun is forbidden.
    pub auto_rerun_forbidden: bool,
    /// Whether missing dependency placeholders are required.
    pub missing_dependency_placeholders_required: bool,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Destination collision review for entry flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryDestinationCollisionReview {
    /// Destination collision class.
    pub collision_class: EntryDestinationCollisionClass,
    /// Existing target label.
    pub existing_target_label: String,
    /// Whether an explicit user choice is required.
    pub requires_explicit_choice: bool,
    /// Whether overwrite requires an explicit destructive review.
    pub explicit_overwrite_required: bool,
    /// Safe actions offered.
    pub safe_actions: Vec<EntryCollisionSafeAction>,
    /// Previously cloned target reference when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previously_cloned_target_ref: Option<String>,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Destination collision class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryDestinationCollisionClass {
    /// No collision.
    NoCollision,
    /// Existing non-empty path.
    ExistingPathNonEmpty,
    /// Existing repository root at the destination.
    ExistingRepoRoot,
    /// Existing nested repository below the destination.
    NestedRepository,
    /// Existing Git worktree at the destination.
    ExistingWorktree,
    /// Existing workspace manifest at the destination.
    ExistingWorkspaceManifest,
    /// Destination matches a prior clone of the same target.
    DuplicateCloneTarget,
    /// Policy blocks the destination.
    DestinationBlockedByPolicy,
}

impl_as_str!(EntryDestinationCollisionClass {
    NoCollision => "no_collision",
    ExistingPathNonEmpty => "existing_path_non_empty",
    ExistingRepoRoot => "existing_repo_root",
    NestedRepository => "nested_repository",
    ExistingWorktree => "existing_worktree",
    ExistingWorkspaceManifest => "existing_workspace_manifest",
    DuplicateCloneTarget => "duplicate_clone_target",
    DestinationBlockedByPolicy => "destination_blocked_by_policy",
});

/// Safe action offered by a destination collision review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryCollisionSafeAction {
    /// Reuse an existing target.
    ReuseExisting,
    /// Add existing target as a root.
    AddExistingAsRoot,
    /// Choose another destination.
    CloneElsewhere,
    /// Reveal the destination in the system shell.
    Reveal,
    /// Inspect only.
    InspectOnly,
    /// Cancel without durable change.
    Cancel,
}

impl_as_str!(EntryCollisionSafeAction {
    ReuseExisting => "reuse_existing",
    AddExistingAsRoot => "add_existing_as_root",
    CloneElsewhere => "clone_elsewhere",
    Reveal => "reveal",
    InspectOnly => "inspect_only",
    Cancel => "cancel",
});

/// Post-entry handoff card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryPostEntryHandoffCard {
    /// Opaque card id.
    pub handoff_card_id: String,
    /// Opened, materialized, imported, or restored target label.
    pub target_label: String,
    /// Work intentionally not run yet.
    pub not_yet_done: Vec<EntryDeferredWorkClass>,
    /// Blocking readiness task classes.
    pub blocked_tasks: Vec<ReadinessTaskClass>,
    /// Recommended readiness task classes.
    pub recommended_tasks: Vec<ReadinessTaskClass>,
    /// Optional readiness task classes.
    pub optional_tasks: Vec<ReadinessTaskClass>,
    /// Primary next action.
    pub primary_next_action: AdmissionAction,
    /// Safe alternate actions.
    pub safe_alternate_actions: Vec<AdmissionAction>,
    /// Whether support/export can retrieve this state later.
    pub export_or_share_state_available: bool,
    /// Summary for review surfaces.
    pub summary: String,
}

/// Work intentionally deferred by entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryDeferredWorkClass {
    /// Trust grant was not performed.
    TrustGrant,
    /// Dependency restore was not performed.
    DependencyRestore,
    /// Task execution was not performed.
    TaskExecution,
    /// Hook execution was not performed.
    HookExecution,
    /// Extension or bundle install was not performed.
    ExtensionOrBundleInstall,
    /// State rehydration was not performed.
    StateRehydration,
    /// Durable promotion was not performed.
    DurablePromotion,
    /// Runtime attach was not performed.
    RuntimeAttach,
    /// Credential admission was not performed.
    CredentialAdmission,
}

impl_as_str!(EntryDeferredWorkClass {
    TrustGrant => "trust_grant",
    DependencyRestore => "dependency_restore",
    TaskExecution => "task_execution",
    HookExecution => "hook_execution",
    ExtensionOrBundleInstall => "extension_or_bundle_install",
    StateRehydration => "state_rehydration",
    DurablePromotion => "durable_promotion",
    RuntimeAttach => "runtime_attach",
    CredentialAdmission => "credential_admission",
});

/// Failed-attempt repair state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntryFailureRepairState {
    /// Whether typed source input is preserved.
    pub typed_source_input_preserved: bool,
    /// Whether the chosen destination is preserved.
    pub chosen_destination_preserved: bool,
    /// Whether diagnostics are preserved after redaction.
    pub redacted_diagnostics_preserved: bool,
    /// Redacted source input label.
    pub source_input_label: String,
    /// Redacted destination label.
    pub destination_label: String,
    /// Redacted diagnostic summary.
    pub redacted_diagnostic_summary: String,
    /// Repair actions offered after failure.
    pub repair_actions: Vec<AdmissionAction>,
}

/// Cross-surface parity row for one entry activation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntrySurfaceParity {
    /// Source surface covered by this row.
    pub source_surface: AdmissionSourceSurface,
    /// Entry verb preserved on the surface.
    pub entry_verb: EntryVerb,
    /// Target kind preserved on the surface.
    pub target_kind: TargetKind,
    /// Resulting mode preserved on the surface.
    pub resulting_mode: ResultingMode,
    /// Review requirement class.
    pub review_requirement: EntryReviewRequirementClass,
    /// Whether the surface uses the same review model.
    pub same_review_model: bool,
    /// Summary for support surfaces.
    pub summary: String,
}

/// Review requirement class for entry parity rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryReviewRequirementClass {
    /// Target review is required.
    TargetReviewRequired,
    /// Trust review is required.
    TrustReviewRequired,
    /// Destination collision review is required.
    DestinationCollisionReviewRequired,
    /// Import compare is required.
    ImportCompareRequired,
    /// Restore review is required.
    RestoreReviewRequired,
    /// Deep-link intent review is required.
    DeepLinkIntentReviewRequired,
    /// No review is required.
    NoReviewRequired,
}

impl_as_str!(EntryReviewRequirementClass {
    TargetReviewRequired => "target_review_required",
    TrustReviewRequired => "trust_review_required",
    DestinationCollisionReviewRequired => "destination_collision_review_required",
    ImportCompareRequired => "import_compare_required",
    RestoreReviewRequired => "restore_review_required",
    DeepLinkIntentReviewRequired => "deep_link_intent_review_required",
    NoReviewRequired => "no_review_required",
});

/// Builds a complete project-entry review from a surface request.
pub fn build_project_entry_review(request: ProjectEntryReviewRequest) -> ProjectEntryReviewRecord {
    let admission_packet = admission_packet_for(&request);
    let destination_collision_review = destination_collision_review_for(&request);
    let review_sheet = review_sheet_for(&request, &admission_packet);
    let readiness = readiness_for_entry(&request);
    let entry_source = first_useful_entry_source_for(request.entry_verb, request.target_kind);
    let continue_without = continue_without_for(request.entry_verb, request.resulting_mode);
    let archetype = archetype_truth_for(&request);
    let admission_checkpoint_route = build_admission_checkpoint_route(
        AdmissionCheckpointBuildRequest::new(
            admission_packet.clone(),
            format!("entry.review.{}", admission_packet.admission_review_id),
            entry_source,
            archetype,
        )
        .with_readiness(readiness.clone())
        .with_continue_without(continue_without)
        .with_trust(
            TrustState::PendingEvaluation,
            trust_review_for(request.entry_verb),
        ),
    );
    let vocabulary_review = vocabulary_review_for(&request, &admission_packet);
    let post_entry_handoff_card = post_entry_handoff_card_for(
        &request,
        &admission_packet,
        &admission_checkpoint_route,
        &readiness,
    );
    let failure_repair_state = failure_repair_state_for(&request, &admission_packet);
    let surface_parity = surface_parity_for(
        &request,
        destination_collision_review.is_some(),
        request.source_surface,
    );
    let entry_review_id = format!(
        "entry-review-{:016x}",
        stable_hash(&format!(
            "{}\n{}\n{}",
            admission_packet.admission_review_id,
            request.source_surface.as_str(),
            vocabulary_review.destination_label
        ))
    );

    ProjectEntryReviewRecord {
        record_kind: ProjectEntryReviewRecordKind::ProjectEntryReviewRecord,
        entry_review_schema_version: ENTRY_REVIEW_SCHEMA_VERSION,
        entry_review_id,
        source_surface: request.source_surface,
        entry_verb: request.entry_verb,
        target_kind: request.target_kind,
        resulting_mode: request.resulting_mode,
        vocabulary_review,
        review_sheet,
        destination_collision_review,
        post_entry_handoff_card,
        failure_repair_state,
        surface_parity,
        admission_review_packet: admission_packet,
        admission_checkpoint_route,
        summary: format!(
            "{} entry review preserves verb, destination, trust, setup, and recovery checkpoints.",
            request.entry_verb.as_str()
        ),
    }
}

fn admission_packet_for(request: &ProjectEntryReviewRequest) -> AdmissionReviewPacket {
    let mut admission_request = AdmissionReviewRequest::new(
        request.source_surface,
        request.entry_verb,
        request.target_kind,
        request.resulting_mode,
        request.target_specifier.clone(),
    );
    if let Some(destination) = request.destination.clone() {
        admission_request = admission_request.with_destination(destination);
    }
    if let Some(active_workspace) = request.active_workspace_label.clone() {
        admission_request = admission_request.with_active_workspace(active_workspace);
    }
    if let Some(network_route) = request.network_route_label.clone() {
        admission_request = admission_request.with_network_route(network_route);
    }
    review_entry_admission(admission_request)
}

fn review_sheet_for(
    request: &ProjectEntryReviewRequest,
    admission_packet: &AdmissionReviewPacket,
) -> EntryReviewSheet {
    match sheet_kind_for(request.entry_verb, request.target_kind) {
        EntryReviewSheetKind::OpenLocalTarget => EntryReviewSheet {
            review_sheet_kind: EntryReviewSheetKind::OpenLocalTarget,
            open_review: Some(open_review_for(request, admission_packet)),
            open_workspace_review: None,
            clone_review: None,
            add_root_review: None,
            import_review: None,
            restore_review: None,
        },
        EntryReviewSheetKind::OpenWorkspaceManifest => EntryReviewSheet {
            review_sheet_kind: EntryReviewSheetKind::OpenWorkspaceManifest,
            open_review: None,
            open_workspace_review: Some(open_workspace_review_for(request)),
            clone_review: None,
            add_root_review: None,
            import_review: None,
            restore_review: None,
        },
        EntryReviewSheetKind::CloneRepository => EntryReviewSheet {
            review_sheet_kind: EntryReviewSheetKind::CloneRepository,
            open_review: None,
            open_workspace_review: None,
            clone_review: Some(clone_review_for(request, admission_packet)),
            add_root_review: None,
            import_review: None,
            restore_review: None,
        },
        EntryReviewSheetKind::AddRoot => EntryReviewSheet {
            review_sheet_kind: EntryReviewSheetKind::AddRoot,
            open_review: None,
            open_workspace_review: None,
            clone_review: None,
            add_root_review: Some(add_root_review_for(request)),
            import_review: None,
            restore_review: None,
        },
        EntryReviewSheetKind::ImportArtifact => EntryReviewSheet {
            review_sheet_kind: EntryReviewSheetKind::ImportArtifact,
            open_review: None,
            open_workspace_review: None,
            clone_review: None,
            add_root_review: None,
            import_review: Some(import_review_for(request, admission_packet)),
            restore_review: None,
        },
        EntryReviewSheetKind::RestoreState => EntryReviewSheet {
            review_sheet_kind: EntryReviewSheetKind::RestoreState,
            open_review: None,
            open_workspace_review: None,
            clone_review: None,
            add_root_review: None,
            import_review: None,
            restore_review: Some(restore_review_for(request, admission_packet)),
        },
    }
}

fn sheet_kind_for(entry_verb: EntryVerb, target_kind: TargetKind) -> EntryReviewSheetKind {
    match entry_verb {
        EntryVerb::Open
            if matches!(
                target_kind,
                TargetKind::WorkspaceManifest | TargetKind::WorksetManifest
            ) =>
        {
            EntryReviewSheetKind::OpenWorkspaceManifest
        }
        EntryVerb::Open => EntryReviewSheetKind::OpenLocalTarget,
        EntryVerb::Clone => EntryReviewSheetKind::CloneRepository,
        EntryVerb::AddRoot => EntryReviewSheetKind::AddRoot,
        EntryVerb::Import | EntryVerb::StartFromSnapshot => EntryReviewSheetKind::ImportArtifact,
        EntryVerb::Restore | EntryVerb::Resume => EntryReviewSheetKind::RestoreState,
    }
}

fn open_review_for(
    request: &ProjectEntryReviewRequest,
    admission_packet: &AdmissionReviewPacket,
) -> OpenEntryReviewSheet {
    let path = Path::new(request.target_specifier.trim());
    OpenEntryReviewSheet {
        target_label: admission_packet
            .normalized_target_identity
            .normalized_label
            .clone(),
        target_identity_class: admission_packet.normalized_target_identity.identity_class,
        nested_repo_review_required: path.is_dir() && contains_nested_repo(path),
        workspace_manifest_choice_available: path.is_dir()
            && has_workspace_manifest_near_target(path),
        writes_files_before_admission: false,
        setup_deferred: true,
        summary: "Open inspects existing content and keeps trust and setup separate.".to_string(),
    }
}

fn open_workspace_review_for(request: &ProjectEntryReviewRequest) -> OpenWorkspaceReviewSheet {
    OpenWorkspaceReviewSheet {
        manifest_label: normalize_review_label(&request.target_specifier),
        schema_or_version_label: "workspace manifest schema reviewed before upgrade".to_string(),
        missing_root_posture_label: "missing roots become repairable placeholders".to_string(),
        silent_upgrade_forbidden: true,
        dropped_meaning_disclosed: true,
        summary: "Open workspace reviews manifest version, roots, and restore posture.".to_string(),
    }
}

fn clone_review_for(
    request: &ProjectEntryReviewRequest,
    admission_packet: &AdmissionReviewPacket,
) -> CloneEntryReviewSheet {
    let clone = admission_packet
        .clone_review
        .as_ref()
        .expect("clone admission packet must carry clone review");
    let options = &request.clone_options;
    let source_access_class = source_access_class_for(request);
    CloneEntryReviewSheet {
        normalized_remote_url_label: clone.normalized_remote_label.clone(),
        host_label: clone.host_label.clone(),
        certificate_posture: clone.certificate_posture,
        auth_mode: clone.auth_mode,
        branch_or_ref_label: options.branch_or_ref_label.clone(),
        ref_choice: options.ref_choice,
        clone_depth_class: depth_for_source_access(options.clone_depth_class, source_access_class),
        partial_filter_label: options.partial_filter_label.clone(),
        submodule_posture: options.submodule_posture,
        lfs_posture: options.lfs_posture,
        destination_path_label: admission_packet
            .destination_review
            .destination_label
            .clone(),
        source_access_class,
        route_note: clone.route_note.clone(),
        post_clone_action: options
            .post_clone_action
            .unwrap_or_else(|| post_clone_action_for(request.resulting_mode)),
        explicit_actions: clone.explicit_actions.clone(),
        clone_never_grants_trust: true,
        dependency_restore_deferred: true,
        task_execution_deferred: true,
    }
}

fn add_root_review_for(request: &ProjectEntryReviewRequest) -> AddRootEntryReviewSheet {
    AddRootEntryReviewSheet {
        active_workspace_label: request
            .active_workspace_label
            .clone()
            .unwrap_or_else(|| "current workspace".to_string()),
        new_root_label: normalize_review_label(&request.target_specifier),
        per_root_trust_review_required: true,
        scope_change_review_required: true,
        sibling_trust_inheritance_forbidden: true,
        checkpoint_required: true,
        summary: "Add root reviews the active workspace mutation and per-root trust.".to_string(),
    }
}

fn import_review_for(
    request: &ProjectEntryReviewRequest,
    admission_packet: &AdmissionReviewPacket,
) -> ImportEntryReviewSheet {
    let import = admission_packet
        .import_review
        .as_ref()
        .expect("import admission packet must carry import review");
    let inspect_only = matches!(
        import.import_action,
        ImportAction::InspectOnly | ImportAction::CompareBeforeRestore
    ) || request.resulting_mode == ResultingMode::InspectOnly;
    ImportEntryReviewSheet {
        artifact_class: import.artifact_class,
        import_action: import.import_action,
        write_behavior_class: import_write_behavior_for(import.import_action, inspect_only),
        schema_or_version_label: if request.import_options.schema_or_version_label.is_empty() {
            import.schema_or_producer_label.clone()
        } else {
            request.import_options.schema_or_version_label.clone()
        },
        extraction_or_restore_target_label: import.extraction_or_restore_target_label.clone(),
        lossy_mapping_class: request.import_options.lossy_mapping_class,
        machine_local_exclusions: request.import_options.machine_local_exclusions.clone(),
        cleanup_posture: request.import_options.cleanup_posture,
        inspect_only,
        no_durable_write_before_review: true,
        no_state_rehydration_before_review: true,
    }
}

fn restore_review_for(
    request: &ProjectEntryReviewRequest,
    admission_packet: &AdmissionReviewPacket,
) -> RestoreEntryReviewSheet {
    RestoreEntryReviewSheet {
        restore_source_label: normalize_review_label(&request.target_specifier),
        schema_or_checkpoint_label: "checkpoint or session schema reviewed".to_string(),
        restore_target_label: admission_packet
            .destination_review
            .destination_label
            .clone(),
        retained_state_classes: vec!["local history".to_string(), "recovery journal".to_string()],
        replaced_state_classes: vec!["window layout".to_string(), "editor tabs".to_string()],
        checkpoint_required_before_overwrite: true,
        auto_rerun_forbidden: true,
        missing_dependency_placeholders_required: true,
        summary: "Restore reviews retained, replaced, and placeholder state before overwrite."
            .to_string(),
    }
}

fn destination_collision_review_for(
    request: &ProjectEntryReviewRequest,
) -> Option<EntryDestinationCollisionReview> {
    let destination = request
        .destination
        .as_deref()
        .unwrap_or(request.target_specifier.as_str());
    let collision_class = destination_collision_class(destination, &request.destination_facts);
    if collision_class == EntryDestinationCollisionClass::NoCollision {
        return None;
    }
    let safe_actions = match collision_class {
        EntryDestinationCollisionClass::DuplicateCloneTarget => vec![
            EntryCollisionSafeAction::ReuseExisting,
            EntryCollisionSafeAction::AddExistingAsRoot,
            EntryCollisionSafeAction::Reveal,
            EntryCollisionSafeAction::InspectOnly,
            EntryCollisionSafeAction::Cancel,
        ],
        EntryDestinationCollisionClass::DestinationBlockedByPolicy => vec![
            EntryCollisionSafeAction::Reveal,
            EntryCollisionSafeAction::Cancel,
        ],
        _ => vec![
            EntryCollisionSafeAction::ReuseExisting,
            EntryCollisionSafeAction::AddExistingAsRoot,
            EntryCollisionSafeAction::CloneElsewhere,
            EntryCollisionSafeAction::Reveal,
            EntryCollisionSafeAction::Cancel,
        ],
    };
    Some(EntryDestinationCollisionReview {
        collision_class,
        existing_target_label: normalize_review_label(destination),
        requires_explicit_choice: true,
        explicit_overwrite_required: matches!(
            collision_class,
            EntryDestinationCollisionClass::ExistingPathNonEmpty
                | EntryDestinationCollisionClass::NestedRepository
        ),
        safe_actions,
        previously_cloned_target_ref: request
            .destination_facts
            .previously_cloned_target_ref
            .clone(),
        summary: collision_summary(collision_class),
    })
}

fn destination_collision_class(
    destination: &str,
    facts: &EntryDestinationFacts,
) -> EntryDestinationCollisionClass {
    if facts.policy_blocked {
        return EntryDestinationCollisionClass::DestinationBlockedByPolicy;
    }
    if facts.previously_cloned_target_ref.is_some() {
        return EntryDestinationCollisionClass::DuplicateCloneTarget;
    }
    if destination.trim().is_empty() || destination.starts_with("~/") {
        return EntryDestinationCollisionClass::NoCollision;
    }
    let path = Path::new(destination);
    if !path.exists() {
        return EntryDestinationCollisionClass::NoCollision;
    }
    if is_workspace_manifest_path(path) {
        return EntryDestinationCollisionClass::ExistingWorkspaceManifest;
    }
    if path.is_dir() && path.join(".git").is_file() {
        return EntryDestinationCollisionClass::ExistingWorktree;
    }
    if path.is_dir() && path.join(".git").is_dir() {
        return EntryDestinationCollisionClass::ExistingRepoRoot;
    }
    if path.is_dir() && contains_nested_repo(path) {
        return EntryDestinationCollisionClass::NestedRepository;
    }
    if path.is_dir() {
        match std::fs::read_dir(path) {
            Ok(mut entries) => {
                if entries.next().is_none() {
                    EntryDestinationCollisionClass::NoCollision
                } else {
                    EntryDestinationCollisionClass::ExistingPathNonEmpty
                }
            }
            Err(_) => EntryDestinationCollisionClass::DestinationBlockedByPolicy,
        }
    } else {
        EntryDestinationCollisionClass::ExistingPathNonEmpty
    }
}

fn post_entry_handoff_card_for(
    request: &ProjectEntryReviewRequest,
    admission_packet: &AdmissionReviewPacket,
    route: &AdmissionCheckpointRouteRecord,
    readiness: &ReadinessBuckets,
) -> EntryPostEntryHandoffCard {
    let not_yet_done = deferred_work_for(request.entry_verb);
    EntryPostEntryHandoffCard {
        handoff_card_id: format!("handoff.{}", route.checkpoint.admission_checkpoint_id),
        target_label: admission_packet
            .normalized_target_identity
            .normalized_label
            .clone(),
        not_yet_done,
        blocked_tasks: readiness
            .blocking_now
            .iter()
            .map(|task| task.task_class)
            .collect(),
        recommended_tasks: readiness
            .recommended_soon
            .iter()
            .map(|task| task.task_class)
            .collect(),
        optional_tasks: readiness
            .optional_later
            .iter()
            .map(|task| task.task_class)
            .collect(),
        primary_next_action: primary_next_action_for(request.entry_verb, request.resulting_mode),
        safe_alternate_actions: vec![
            AdmissionAction::SetUpLater,
            AdmissionAction::OpenMinimal,
            AdmissionAction::Cancel,
        ],
        export_or_share_state_available: true,
        summary: format!(
            "{} entry produced a handoff card before trust, setup, or side-effect execution.",
            request.entry_verb.as_str()
        ),
    }
}

fn failure_repair_state_for(
    request: &ProjectEntryReviewRequest,
    admission_packet: &AdmissionReviewPacket,
) -> EntryFailureRepairState {
    EntryFailureRepairState {
        typed_source_input_preserved: true,
        chosen_destination_preserved: true,
        redacted_diagnostics_preserved: true,
        source_input_label: redact_sensitive_text(&request.target_specifier),
        destination_label: admission_packet
            .destination_review
            .destination_label
            .clone(),
        redacted_diagnostic_summary: request
            .failed_diagnostic_text
            .as_deref()
            .map(redact_sensitive_text)
            .unwrap_or_else(|| {
                "No failure captured; repair state will preserve redacted diagnostics when needed."
                    .to_string()
            }),
        repair_actions: repair_actions_for(request.entry_verb),
    }
}

fn vocabulary_review_for(
    request: &ProjectEntryReviewRequest,
    admission_packet: &AdmissionReviewPacket,
) -> EntryVocabularyReview {
    EntryVocabularyReview {
        source_access_class: source_access_class_for(request),
        source_label: admission_packet
            .normalized_target_identity
            .normalized_label
            .clone(),
        trust_label: "pending_evaluation; no silent trust grant".to_string(),
        destination_label: admission_packet
            .destination_review
            .destination_label
            .clone(),
        write_scope_label: admission_packet
            .write_scope
            .write_scope_class
            .as_str()
            .to_string(),
        next_step_label: primary_next_action_for(request.entry_verb, request.resulting_mode)
            .as_str()
            .to_string(),
    }
}

fn surface_parity_for(
    request: &ProjectEntryReviewRequest,
    has_collision: bool,
    primary_surface: AdmissionSourceSurface,
) -> Vec<EntrySurfaceParity> {
    let mut surfaces = vec![
        AdmissionSourceSurface::StartCenter,
        AdmissionSourceSurface::CommandPalette,
        AdmissionSourceSurface::DragAndDrop,
        AdmissionSourceSurface::SystemFileAssociation,
        AdmissionSourceSurface::DeepLink,
        AdmissionSourceSurface::CliHeadless,
        AdmissionSourceSurface::WorkspaceSwitcher,
    ];
    if !surfaces.contains(&primary_surface) {
        surfaces.push(primary_surface);
    }
    surfaces
        .into_iter()
        .map(|source_surface| EntrySurfaceParity {
            source_surface,
            entry_verb: request.entry_verb,
            target_kind: request.target_kind,
            resulting_mode: request.resulting_mode,
            review_requirement: review_requirement_for(
                request.entry_verb,
                source_surface,
                has_collision,
            ),
            same_review_model: true,
            summary: format!(
                "{} preserves {} as {} with the same review model.",
                source_surface.as_str(),
                request.entry_verb.as_str(),
                request.resulting_mode.as_str()
            ),
        })
        .collect()
}

fn readiness_for_entry(request: &ProjectEntryReviewRequest) -> ReadinessBuckets {
    let mut readiness = ReadinessBuckets::new().with_task(
        ReadinessTask::new(
            "task.entry.trust_review",
            ReadinessTaskClass::TrustReview,
            ReadinessBucket::BlockingNow,
            ReadinessTaskState::BlockedByTrust,
            ExecutionBoundary::NoExecution,
            vec![SideEffectClass::WidensTrust],
            "Review trust before setup, task execution, or authority widening.",
        )
        .with_blocked_reason(BlockedReasonClass::BlockedByTrust),
    );
    match request.entry_verb {
        EntryVerb::Clone => {
            readiness = readiness
                .with_task(ReadinessTask::new(
                    "task.entry.clone.dependency_restore",
                    ReadinessTaskClass::DependencyRestore,
                    ReadinessBucket::RecommendedSoon,
                    ReadinessTaskState::Pending,
                    ExecutionBoundary::LocalMachine,
                    vec![
                        SideEffectClass::DownloadsDependencies,
                        SideEffectClass::StartsProcess,
                    ],
                    "Dependency restore remains a reviewed post-clone step.",
                ))
                .with_task(
                    ReadinessTask::new(
                        "task.entry.clone.extension_recommendation",
                        ReadinessTaskClass::ExtensionRecommendation,
                        ReadinessBucket::OptionalLater,
                        ReadinessTaskState::Optional,
                        ExecutionBoundary::NoExecution,
                        vec![SideEffectClass::NoSideEffect],
                        "Extension or bundle recommendations remain optional.",
                    )
                    .with_optional_reason(OptionalReasonClass::OptionalRecommendedOnly),
                );
        }
        EntryVerb::Import | EntryVerb::StartFromSnapshot => {
            readiness = readiness.with_task(
                ReadinessTask::new(
                    "task.entry.import.compare",
                    ReadinessTaskClass::ImportedStateCompare,
                    ReadinessBucket::BlockingNow,
                    ReadinessTaskState::Pending,
                    ExecutionBoundary::NoExecution,
                    vec![SideEffectClass::NoSideEffect],
                    "Compare imported state before extraction, apply, or restore.",
                )
                .with_blocked_reason(BlockedReasonClass::BlockedByTrust),
            );
        }
        EntryVerb::Restore | EntryVerb::Resume => {
            readiness = readiness.with_task(
                ReadinessTask::new(
                    "task.entry.restore.review",
                    ReadinessTaskClass::RestoreReview,
                    ReadinessBucket::BlockingNow,
                    ReadinessTaskState::Pending,
                    ExecutionBoundary::NoExecution,
                    vec![SideEffectClass::ChangesLayout],
                    "Review retained and replaced state before restore applies.",
                )
                .with_blocked_reason(BlockedReasonClass::BlockedByTrust),
            );
        }
        EntryVerb::AddRoot => {
            readiness = readiness.with_task(
                ReadinessTask::new(
                    "task.entry.add_root.boundary",
                    ReadinessTaskClass::UserBoundaryChoice,
                    ReadinessBucket::BlockingNow,
                    ReadinessTaskState::BlockedByTrust,
                    ExecutionBoundary::NoExecution,
                    vec![SideEffectClass::NoSideEffect],
                    "Choose the root boundary before widening workspace scope.",
                )
                .with_blocked_reason(BlockedReasonClass::BlockedByTrust),
            );
        }
        EntryVerb::Open => {}
    }
    readiness
}

fn archetype_truth_for(request: &ProjectEntryReviewRequest) -> ArchetypeTruth {
    let signal_source = match request.entry_verb {
        EntryVerb::Import | EntryVerb::StartFromSnapshot => {
            DetectionSignalSourceClass::ImportPacket
        }
        EntryVerb::Open
            if matches!(
                request.target_kind,
                TargetKind::WorkspaceManifest | TargetKind::WorksetManifest
            ) =>
        {
            DetectionSignalSourceClass::WorkspaceFile
        }
        EntryVerb::AddRoot => DetectionSignalSourceClass::FilesystemLayout,
        EntryVerb::Clone => DetectionSignalSourceClass::VcsMetadata,
        EntryVerb::Restore | EntryVerb::Resume => DetectionSignalSourceClass::PreviousUserChoice,
        EntryVerb::Open => DetectionSignalSourceClass::FilesystemLayout,
    };
    ArchetypeTruth::new(
        DetectionOutcome::UnknownOrGenericWorkspace,
        DetectionConfidenceClass::GenericUnknown,
        SupportClaimClass::GenericNoClaim,
        DetectorState::ReadyEnough,
        vec![DetectionSignal::new(
            format!("signal.entry.{}", request.entry_verb.as_str()),
            signal_source,
            vec![
                SignalMaterialEffect::Trust,
                SignalMaterialEffect::RouteSelection,
                SignalMaterialEffect::Readiness,
            ],
            "Entry review signal preserves target, trust, destination, and next-step vocabulary.",
        )],
    )
    .with_detected_fact_refs(vec![format!("fact.entry.{}", request.target_kind.as_str())])
}

fn first_useful_entry_source_for(
    entry_verb: EntryVerb,
    target_kind: TargetKind,
) -> FirstUsefulEntrySource {
    match entry_verb {
        EntryVerb::Clone => FirstUsefulEntrySource::RepositoryClone,
        EntryVerb::Import | EntryVerb::StartFromSnapshot => {
            FirstUsefulEntrySource::ImportedStateOrHandoffPacket
        }
        EntryVerb::Restore | EntryVerb::Resume => FirstUsefulEntrySource::RestoreLastSession,
        EntryVerb::Open if target_kind == TargetKind::LocalFile => {
            FirstUsefulEntrySource::SingleFileOpen
        }
        EntryVerb::Open => FirstUsefulEntrySource::FolderOrRepoOpen,
        EntryVerb::AddRoot => FirstUsefulEntrySource::FolderOrRepoOpen,
    }
}

fn source_access_class_for(request: &ProjectEntryReviewRequest) -> EntrySourceAccessClass {
    if matches!(
        request.target_kind,
        TargetKind::LocalFile
            | TargetKind::LocalFolder
            | TargetKind::LocalRepoRoot
            | TargetKind::WorkspaceManifest
            | TargetKind::WorksetManifest
    ) {
        return EntrySourceAccessClass::LocalFilesystem;
    }
    let combined = format!(
        "{} {}",
        request.target_specifier,
        request.network_route_label.as_deref().unwrap_or("")
    )
    .to_ascii_lowercase();
    if combined.contains("airgap") || combined.contains("air-gapped") {
        EntrySourceAccessClass::AirGappedMedia
    } else if combined.contains("offline") {
        EntrySourceAccessClass::OfflineSnapshot
    } else if combined.contains("mirror") {
        EntrySourceAccessClass::MirrorFirst
    } else {
        EntrySourceAccessClass::DirectOnline
    }
}

fn depth_for_source_access(
    clone_depth: CloneDepthClass,
    source_access: EntrySourceAccessClass,
) -> CloneDepthClass {
    match source_access {
        EntrySourceAccessClass::MirrorFirst if clone_depth == CloneDepthClass::FullHistory => {
            CloneDepthClass::MirrorFirst
        }
        EntrySourceAccessClass::AirGappedMedia | EntrySourceAccessClass::OfflineSnapshot => {
            CloneDepthClass::OfflineBundle
        }
        _ => clone_depth,
    }
}

fn post_clone_action_for(resulting_mode: ResultingMode) -> AdmissionAction {
    match resulting_mode {
        ResultingMode::CloneOnly => AdmissionAction::CloneOnly,
        ResultingMode::CloneThenOpen => AdmissionAction::CloneAndOpen,
        ResultingMode::CloneThenAdd => AdmissionAction::CloneAndAdd,
        _ => AdmissionAction::CloneAndReview,
    }
}

fn primary_next_action_for(
    entry_verb: EntryVerb,
    resulting_mode: ResultingMode,
) -> AdmissionAction {
    match entry_verb {
        EntryVerb::Open | EntryVerb::Restore | EntryVerb::Resume => AdmissionAction::Open,
        EntryVerb::Clone => post_clone_action_for(resulting_mode),
        EntryVerb::Import | EntryVerb::StartFromSnapshot => AdmissionAction::Import,
        EntryVerb::AddRoot => AdmissionAction::AddRoot,
    }
}

fn continue_without_for(
    entry_verb: EntryVerb,
    resulting_mode: ResultingMode,
) -> ContinueWithoutClass {
    match (entry_verb, resulting_mode) {
        (EntryVerb::Import, ResultingMode::InspectOnly) => ContinueWithoutClass::InspectOnly,
        (EntryVerb::Restore, ResultingMode::RestoreFromCheckpoint)
        | (EntryVerb::Import, ResultingMode::CompareBeforeRestore) => {
            ContinueWithoutClass::CompareBeforeRestore
        }
        _ => ContinueWithoutClass::SetUpLater,
    }
}

fn trust_review_for(entry_verb: EntryVerb) -> TrustReviewClass {
    match entry_verb {
        EntryVerb::Restore | EntryVerb::Resume => TrustReviewClass::TrustRevalidationRequired,
        _ => TrustReviewClass::TrustReviewPending,
    }
}

fn review_requirement_for(
    entry_verb: EntryVerb,
    source_surface: AdmissionSourceSurface,
    has_collision: bool,
) -> EntryReviewRequirementClass {
    if source_surface == AdmissionSourceSurface::DeepLink {
        return EntryReviewRequirementClass::DeepLinkIntentReviewRequired;
    }
    if has_collision {
        return EntryReviewRequirementClass::DestinationCollisionReviewRequired;
    }
    match entry_verb {
        EntryVerb::Import | EntryVerb::StartFromSnapshot => {
            EntryReviewRequirementClass::ImportCompareRequired
        }
        EntryVerb::Restore | EntryVerb::Resume => {
            EntryReviewRequirementClass::RestoreReviewRequired
        }
        EntryVerb::Clone | EntryVerb::AddRoot | EntryVerb::Open => {
            EntryReviewRequirementClass::TargetReviewRequired
        }
    }
}

fn import_write_behavior_for(
    import_action: ImportAction,
    inspect_only: bool,
) -> ImportWriteBehaviorClass {
    if inspect_only {
        return ImportWriteBehaviorClass::InspectOnlyNoWrite;
    }
    match import_action {
        ImportAction::AddToCurrentWorkspace => ImportWriteBehaviorClass::ApplyToActiveWorkspace,
        ImportAction::RestoreFromPacket | ImportAction::CompareBeforeRestore => {
            ImportWriteBehaviorClass::RestoreAfterCompare
        }
        ImportAction::ExtractAndOpen => ImportWriteBehaviorClass::WriteToReviewedDestination,
        ImportAction::InspectOnly | ImportAction::ExtractAndReview => {
            ImportWriteBehaviorClass::WriteToLabelledStaging
        }
    }
}

fn deferred_work_for(entry_verb: EntryVerb) -> Vec<EntryDeferredWorkClass> {
    let mut deferred = vec![
        EntryDeferredWorkClass::TrustGrant,
        EntryDeferredWorkClass::DependencyRestore,
        EntryDeferredWorkClass::TaskExecution,
        EntryDeferredWorkClass::HookExecution,
        EntryDeferredWorkClass::RuntimeAttach,
    ];
    match entry_verb {
        EntryVerb::Clone => {
            deferred.push(EntryDeferredWorkClass::ExtensionOrBundleInstall);
            deferred.push(EntryDeferredWorkClass::DurablePromotion);
            deferred.push(EntryDeferredWorkClass::CredentialAdmission);
        }
        EntryVerb::Import | EntryVerb::StartFromSnapshot => {
            deferred.push(EntryDeferredWorkClass::StateRehydration);
            deferred.push(EntryDeferredWorkClass::DurablePromotion);
        }
        EntryVerb::Restore | EntryVerb::Resume => {
            deferred.push(EntryDeferredWorkClass::StateRehydration);
        }
        EntryVerb::Open | EntryVerb::AddRoot => {}
    }
    deferred
}

fn repair_actions_for(entry_verb: EntryVerb) -> Vec<AdmissionAction> {
    match entry_verb {
        EntryVerb::Clone => vec![
            AdmissionAction::CloneElsewhere,
            AdmissionAction::RevealTarget,
            AdmissionAction::Cancel,
        ],
        EntryVerb::Import | EntryVerb::StartFromSnapshot => vec![
            AdmissionAction::InspectOnly,
            AdmissionAction::SetUpLater,
            AdmissionAction::Cancel,
        ],
        EntryVerb::AddRoot => vec![
            AdmissionAction::AddRoot,
            AdmissionAction::OpenMinimal,
            AdmissionAction::Cancel,
        ],
        EntryVerb::Open | EntryVerb::Restore | EntryVerb::Resume => vec![
            AdmissionAction::OpenMinimal,
            AdmissionAction::SetUpLater,
            AdmissionAction::Cancel,
        ],
    }
}

fn normalize_review_label(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        "unresolved target".to_string()
    } else {
        redact_sensitive_text(trimmed)
    }
}

fn redact_sensitive_text(value: &str) -> String {
    let mut redacted = value.to_string();
    if let Some(scheme_index) = redacted.find("://") {
        let credentials_start = scheme_index + 3;
        if let Some(at_offset) = redacted[credentials_start..].find('@') {
            redacted.replace_range(
                credentials_start..credentials_start + at_offset,
                "<redacted>",
            );
        }
    }
    if let Some((user_host, path)) = redacted.split_once(':') {
        if user_host.contains('@') && !user_host.contains("://") {
            let host = user_host.rsplit('@').next().unwrap_or(user_host);
            return format!("{host}:{path}");
        }
    }
    redacted
}

fn collision_summary(collision_class: EntryDestinationCollisionClass) -> String {
    match collision_class {
        EntryDestinationCollisionClass::NoCollision => "No destination collision.".to_string(),
        EntryDestinationCollisionClass::ExistingPathNonEmpty => {
            "Destination is non-empty; choose reuse, add existing, clone elsewhere, reveal, or cancel."
                .to_string()
        }
        EntryDestinationCollisionClass::ExistingRepoRoot => {
            "Destination is already a repository root; reuse or add existing after review."
                .to_string()
        }
        EntryDestinationCollisionClass::NestedRepository => {
            "Destination contains a nested repository; choose the intended root explicitly."
                .to_string()
        }
        EntryDestinationCollisionClass::ExistingWorktree => {
            "Destination is a Git worktree; reveal or reuse it without overwriting.".to_string()
        }
        EntryDestinationCollisionClass::ExistingWorkspaceManifest => {
            "Destination is a workspace manifest; open or add it through the workspace path."
                .to_string()
        }
        EntryDestinationCollisionClass::DuplicateCloneTarget => {
            "Destination matches a previous clone; reuse or add existing instead of recloning."
                .to_string()
        }
        EntryDestinationCollisionClass::DestinationBlockedByPolicy => {
            "Destination is blocked by policy; choose another path or cancel.".to_string()
        }
    }
}

fn contains_nested_repo(path: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(path) else {
        return false;
    };
    entries.filter_map(Result::ok).any(|entry| {
        let child = entry.path();
        child.is_dir() && child.join(".git").exists()
    })
}

fn has_workspace_manifest_near_target(path: &Path) -> bool {
    ["aureline.workspace.json", "workspace.json"]
        .iter()
        .any(|name| path.join(name).exists())
        || std::fs::read_dir(path)
            .ok()
            .into_iter()
            .flatten()
            .filter_map(Result::ok)
            .any(|entry| is_workspace_manifest_path(&entry.path()))
}

fn is_workspace_manifest_path(path: &Path) -> bool {
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    file_name == "aureline.workspace.json"
        || file_name == "workspace.json"
        || path
            .extension()
            .and_then(|ext| ext.to_str())
            .is_some_and(|ext| {
                ext.eq_ignore_ascii_case("code-workspace")
                    || ext.eq_ignore_ascii_case("aureline-workspace")
            })
}

fn stable_hash(value: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LandingSurface;

    #[derive(Debug, Deserialize)]
    struct EntryReviewCaseRecord {
        record_kind: String,
        entry_review_case_schema_version: u32,
        case_id: String,
        request: ProjectEntryReviewRequest,
        expect: EntryReviewExpectation,
    }

    #[derive(Debug, Deserialize)]
    struct EntryReviewExpectation {
        review_sheet_kind: EntryReviewSheetKind,
        #[serde(default)]
        collision_class: Option<EntryDestinationCollisionClass>,
        source_access_class: EntrySourceAccessClass,
        first_useful_entry_source: FirstUsefulEntrySource,
        landing_surface: LandingSurface,
        primary_next_action: AdmissionAction,
    }

    #[test]
    fn clone_review_keeps_materialization_separate_from_trust_and_setup() {
        let record = build_project_entry_review(
            ProjectEntryReviewRequest::new(
                AdmissionSourceSurface::StartCenter,
                EntryVerb::Clone,
                TargetKind::RemoteRepository,
                ResultingMode::CloneThenReview,
                "https://token:secret@github.com/acme/payments.git",
            )
            .with_destination("~/Code/payments")
            .with_clone_options(CloneReviewOptions {
                branch_or_ref_label: "main".to_string(),
                ref_choice: RefChoice::UserSelectedBranch,
                clone_depth_class: CloneDepthClass::PartialCloneFiltered,
                partial_filter_label: Some("blob:none".to_string()),
                ..CloneReviewOptions::default()
            })
            .with_failed_diagnostic_text(
                "fatal: Authentication failed for https://token:secret@github.com/acme/payments.git",
            ),
        );

        assert!(
            record.is_contract_valid(),
            "{:?}",
            record.contract_findings()
        );
        let clone = record
            .review_sheet
            .clone_review
            .as_ref()
            .expect("clone review");
        assert_eq!(
            clone.normalized_remote_url_label,
            "github.com/acme/payments"
        );
        assert_eq!(
            clone.clone_depth_class,
            CloneDepthClass::PartialCloneFiltered
        );
        assert!(clone.clone_never_grants_trust);
        assert!(clone.dependency_restore_deferred);
        assert!(record
            .post_entry_handoff_card
            .not_yet_done
            .contains(&EntryDeferredWorkClass::TrustGrant));
        assert!(!record
            .failure_repair_state
            .source_input_label
            .contains("secret"));
    }

    #[test]
    fn import_inspect_only_has_no_write_or_rehydrate_before_review() {
        let record = build_project_entry_review(
            ProjectEntryReviewRequest::new(
                AdmissionSourceSurface::CommandPalette,
                EntryVerb::Import,
                TargetKind::PortableStatePackage,
                ResultingMode::InspectOnly,
                "~/Downloads/workspace.aureline-state.zip",
            )
            .with_import_options(ImportReviewOptions {
                schema_or_version_label: "portable state schema v1".to_string(),
                lossy_mapping_class: ImportLossyMappingClass::NoLossyMapping,
                cleanup_posture: CleanupPosture::NoCleanupRequired,
                ..ImportReviewOptions::default()
            }),
        );

        assert!(
            record.is_contract_valid(),
            "{:?}",
            record.contract_findings()
        );
        let import = record
            .review_sheet
            .import_review
            .as_ref()
            .expect("import review");
        assert!(import.inspect_only);
        assert_eq!(
            import.write_behavior_class,
            ImportWriteBehaviorClass::InspectOnlyNoWrite
        );
        assert!(import.no_durable_write_before_review);
        assert!(import.no_state_rehydration_before_review);
    }

    #[test]
    fn destination_collision_detects_nested_repo_and_requires_choice() {
        let temp = tempfile_dir("entry-review-nested");
        let nested = temp.join("service");
        std::fs::create_dir_all(nested.join(".git")).expect("nested git");

        let record = build_project_entry_review(
            ProjectEntryReviewRequest::new(
                AdmissionSourceSurface::WorkspaceSwitcher,
                EntryVerb::AddRoot,
                TargetKind::LocalFolder,
                ResultingMode::WorkspaceWithRoots,
                temp.display().to_string(),
            )
            .with_destination(temp.display().to_string())
            .with_active_workspace("workspace:active"),
        );

        let collision = record
            .destination_collision_review
            .as_ref()
            .expect("collision review");
        assert_eq!(
            collision.collision_class,
            EntryDestinationCollisionClass::NestedRepository
        );
        assert!(collision.requires_explicit_choice);
        assert!(
            record.is_contract_valid(),
            "{:?}",
            record.contract_findings()
        );
    }

    #[test]
    fn entry_review_case_fixtures_match_builder_contract() {
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/workspace/m3/entry_and_clone_review");
        let mut seen = 0usize;
        for entry in std::fs::read_dir(root).expect("entry review fixture dir") {
            let entry = entry.expect("fixture entry");
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }
            seen += 1;
            let payload = std::fs::read_to_string(&path).expect("fixture reads");
            let fixture: EntryReviewCaseRecord =
                serde_json::from_str(&payload).expect("fixture parses");
            assert_eq!(fixture.record_kind, "entry_review_case_record");
            assert_eq!(fixture.entry_review_case_schema_version, 1);
            assert!(!fixture.case_id.trim().is_empty());

            let record = build_project_entry_review(fixture.request);
            assert!(
                record.is_contract_valid(),
                "{}: {:?}",
                fixture.case_id,
                record.contract_findings()
            );
            assert_eq!(
                record.review_sheet.review_sheet_kind,
                fixture.expect.review_sheet_kind
            );
            assert_eq!(
                record.vocabulary_review.source_access_class,
                fixture.expect.source_access_class
            );
            assert_eq!(
                record.admission_checkpoint_route.checkpoint.entry_source,
                fixture.expect.first_useful_entry_source
            );
            assert_eq!(
                record
                    .admission_checkpoint_route
                    .first_useful_route
                    .landing_surface,
                fixture.expect.landing_surface
            );
            assert_eq!(
                record.post_entry_handoff_card.primary_next_action,
                fixture.expect.primary_next_action
            );
            assert_eq!(
                record
                    .destination_collision_review
                    .as_ref()
                    .map(|collision| collision.collision_class),
                fixture.expect.collision_class
            );
        }
        assert!(
            seen >= 6,
            "expected entry review fixtures for all entry verbs"
        );
    }

    fn tempfile_dir(label: &str) -> std::path::PathBuf {
        let root = std::env::temp_dir().join(format!(
            "aureline-{label}-{:016x}",
            stable_hash(&format!("{:?}", std::time::SystemTime::now()))
        ));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).expect("create temp dir");
        root
    }
}
