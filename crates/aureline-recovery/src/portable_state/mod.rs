//! Portable-state package projection for recovery, profile, layout, and transfer metadata.
//!
//! The projection is the recovery-owned aggregate that joins shell layout
//! snapshots, portable-profile export summaries, session-restore proposals,
//! and transfer-integrity records without depending on the shell crate. Shell
//! producers can map their richer local records into these export-safe rows,
//! while recovery and support consumers get one deterministic package boundary.

use std::collections::BTreeSet;
use std::fmt;

use aureline_history::{checkpoints::SnapshotClass, RestoreCheckpointAlpha};
use aureline_settings::inspector::SettingsSupportExportProjection;
use aureline_workspace::{
    ArtifactPortabilityLabel, PortableProfileExport, PortableStateAlphaPackage,
    SerializedStateClass,
};
use serde::{Deserialize, Serialize};

use crate::session_restore::{
    records::{ProducerBuildStamp, RestoreClass},
    RestoreProposal,
};

/// Record-kind discriminator for portable-state package projections.
pub const PORTABLE_STATE_PACKAGE_RECORD_KIND: &str = "portable_state_package_record";

/// Schema version for the recovery-owned portable-state projection.
pub const PORTABLE_STATE_PACKAGE_SCHEMA_VERSION: u32 = 1;

/// Schema path for the portable-state package boundary record.
pub const PORTABLE_STATE_PACKAGE_SCHEMA_REF: &str =
    "schemas/state/portable_state_package.schema.json";

/// Schema path for the portable-state manifest wrapped by the package.
pub const PORTABLE_STATE_MANIFEST_SCHEMA_REF: &str =
    "schemas/state/portable_state_manifest.schema.json";

/// Existing local-history restore-checkpoint command used by recovery surfaces.
pub const RESTORE_CHECKPOINT_COMMAND_ID: &str = "cmd:local_history.restore_checkpoint";

/// Existing local-history snapshot class for restore rollback checkpoints.
pub const RESTORE_CHECKPOINT_SNAPSHOT_CLASS: &str =
    SnapshotClass::RestoreRollbackCheckpoint.as_str();

/// Source component class represented by one aggregate artifact ref.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableStateComponentClass {
    /// Shell layout serialization snapshot.
    LayoutSerializationSnapshot,
    /// Portable profile or settings-inspector export summary.
    PortableProfile,
    /// Recovery pre-hydration session-restore proposal.
    SessionRestoreProposal,
    /// Transfer-integrity metadata exported without raw payloads.
    TransferIntegrityRecords,
    /// Local-history restore checkpoint projection.
    RestoreCheckpoint,
    /// Restore-provenance lineage record.
    RestoreProvenanceLineage,
}

impl PortableStateComponentClass {
    /// Returns the stable token for this component class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutSerializationSnapshot => "layout_serialization_snapshot",
            Self::PortableProfile => "portable_profile",
            Self::SessionRestoreProposal => "session_restore_proposal",
            Self::TransferIntegrityRecords => "transfer_integrity_records",
            Self::RestoreCheckpoint => "restore_checkpoint",
            Self::RestoreProvenanceLineage => "restore_provenance_lineage",
        }
    }
}

/// Package purpose vocabulary copied from the portable-state schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackagePurpose {
    /// User-selected portable export.
    UserPortableExport,
    /// Workspace layout export.
    WorkspaceLayoutExport,
    /// Export built for restore comparison.
    RestoreCompareExport,
    /// Support-review export.
    SupportReviewExport,
    /// Migration handoff export.
    MigrationHandoffExport,
}

/// Destination class vocabulary copied from the portable-state schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationClass {
    /// Local file export.
    LocalFile,
    /// Workspace artifact export.
    WorkspaceArtifact,
    /// Support bundle attachment.
    SupportBundleAttachment,
    /// Managed sync service export.
    ManagedSyncService,
    /// Customer-managed storage export.
    CustomerManagedStorage,
    /// Air-gapped transfer artifact.
    AirGappedTransfer,
}

/// Redaction rules enforced before a package crosses a machine boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableStateRedactionRule {
    /// Raw secret material is excluded.
    RawSecretMaterialExcluded,
    /// Approval tickets are excluded.
    ApprovalTicketExcluded,
    /// Delegated credentials are excluded.
    DelegatedCredentialExcluded,
    /// Live authority handles are excluded.
    LiveAuthorityHandleExcluded,
    /// Machine-unique handles are excluded.
    MachineUniqueHandleExcluded,
    /// Concrete state roots are excluded.
    StateRootExcluded,
    /// Raw clipboard bodies are excluded.
    RawClipboardBodyExcluded,
    /// Raw terminal payloads are excluded.
    RawTerminalPayloadExcluded,
    /// Raw command lines are excluded.
    RawCommandLineExcluded,
    /// Raw environment bodies are excluded.
    RawEnvironmentBodyExcluded,
    /// Raw logs are excluded.
    RawLogExcluded,
    /// Raw source content is excluded.
    RawSourceContentExcluded,
    /// Provider payload bodies are excluded.
    ProviderPayloadExcluded,
}

/// Redaction manifest attached to the aggregate package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateRedactionManifest {
    /// Stable manifest ref.
    pub manifest_id: String,
    /// Applied redaction rules.
    pub rules: Vec<PortableStateRedactionRule>,
    /// True when the default package redaction floor was applied.
    pub redaction_defaults_applied: bool,
    /// True only through a separate high-friction raw-body path.
    pub raw_payload_bodies_included: bool,
    /// Redaction-aware notes.
    pub notes: String,
}

impl PortableStateRedactionManifest {
    /// Builds the default package redaction manifest.
    pub fn default_for(package_id: &str) -> Self {
        Self {
            manifest_id: format!("redaction-manifest:{package_id}"),
            rules: default_redaction_rules(),
            redaction_defaults_applied: true,
            raw_payload_bodies_included: false,
            notes: "Raw secrets, approvals, delegated credentials, live handles, state roots, clipboard bodies, terminal payloads, commands, logs, source content, and provider payloads are excluded."
                .to_string(),
        }
    }

    /// Validates that the package redaction floor is present.
    pub fn validate(&self) -> Result<(), PortableStatePackageError> {
        if self.manifest_id.trim().is_empty() {
            return Err(PortableStatePackageError::MissingField(
                "redaction_manifest.manifest_id",
            ));
        }
        if !self.redaction_defaults_applied {
            return Err(PortableStatePackageError::RedactionDefaultsMissing);
        }
        if self.raw_payload_bodies_included {
            return Err(PortableStatePackageError::RawPayloadIncluded(
                "redaction_manifest.raw_payload_bodies_included",
            ));
        }
        let rules = self.rules.iter().copied().collect::<BTreeSet<_>>();
        for required in REQUIRED_REDACTION_RULES {
            if !rules.contains(&required) {
                return Err(PortableStatePackageError::MissingRedactionRule(required));
            }
        }
        Ok(())
    }
}

/// Snapshot of a shell layout serialization package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayoutSerializationSnapshot {
    /// Stable projection ref for the layout snapshot summary.
    pub snapshot_ref: String,
    /// Source shell package id.
    pub package_id: String,
    /// Source portable-state manifest id.
    pub manifest_id: String,
    /// Workspace ref described by the layout package.
    pub workspace_ref: String,
    /// Source package schema version.
    pub schema_version: u32,
    /// State-class row ids in the source package.
    pub state_class_refs: Vec<String>,
    /// Window-topology body refs carried by the source package.
    pub window_topology_snapshot_refs: Vec<String>,
    /// Stable pane ids present in the layout.
    pub stable_pane_ids: Vec<String>,
    /// Restore-provenance refs carried by the layout package.
    pub restore_provenance_refs: Vec<String>,
    /// Source redaction manifest ref.
    pub redaction_manifest_ref: String,
    /// Machine-local exclusion refs named by the source package.
    pub machine_local_exclusion_refs: Vec<String>,
}

impl LayoutSerializationSnapshot {
    /// Projects a validated workspace portable-state package into recovery metadata.
    pub fn from_workspace_package(
        snapshot_ref: impl Into<String>,
        package: &PortableStateAlphaPackage,
    ) -> Result<Self, PortableStatePackageError> {
        package
            .validate()
            .map_err(|err| PortableStatePackageError::InvalidLayoutPackage(err.to_string()))?;

        let mut state_class_refs = package
            .state_classes
            .iter()
            .map(|row| row.class_id.clone())
            .collect::<Vec<_>>();
        sort_unique(&mut state_class_refs);

        let mut stable_pane_ids = package
            .state_classes
            .iter()
            .flat_map(|row| row.pane_restore_postures.iter())
            .map(|pane| pane.stable_pane_id.clone())
            .collect::<Vec<_>>();
        sort_unique(&mut stable_pane_ids);

        let mut window_topology_snapshot_refs = package
            .state_classes
            .iter()
            .filter(|row| row.class_kind == SerializedStateClass::WindowTopology)
            .flat_map(|row| row.schema_binding.artifact_refs.iter().cloned())
            .collect::<Vec<_>>();
        sort_unique(&mut window_topology_snapshot_refs);

        let mut restore_provenance_refs =
            package.restore_provenance.restore_provenance_refs.clone();
        sort_unique(&mut restore_provenance_refs);

        let mut machine_local_exclusion_refs = package
            .machine_local_exclusions
            .iter()
            .map(|row| row.exclusion_id.clone())
            .collect::<Vec<_>>();
        sort_unique(&mut machine_local_exclusion_refs);

        Ok(Self {
            snapshot_ref: snapshot_ref.into(),
            package_id: package.package_id.clone(),
            manifest_id: package.manifest_id.clone(),
            workspace_ref: package.workspace_ref.clone(),
            schema_version: package.schema_version,
            state_class_refs,
            window_topology_snapshot_refs,
            stable_pane_ids,
            restore_provenance_refs,
            redaction_manifest_ref: package.redaction_manifest.manifest_id.clone(),
            machine_local_exclusion_refs,
        })
    }

    /// Validates required layout snapshot attribution.
    pub fn validate(&self) -> Result<(), PortableStatePackageError> {
        require_non_empty("layout_snapshot.snapshot_ref", &self.snapshot_ref)?;
        require_non_empty("layout_snapshot.package_id", &self.package_id)?;
        require_non_empty("layout_snapshot.manifest_id", &self.manifest_id)?;
        require_non_empty("layout_snapshot.workspace_ref", &self.workspace_ref)?;
        require_non_empty(
            "layout_snapshot.redaction_manifest_ref",
            &self.redaction_manifest_ref,
        )?;
        if self.state_class_refs.is_empty() {
            return Err(PortableStatePackageError::MissingComponent(
                PortableStateComponentClass::LayoutSerializationSnapshot,
            ));
        }
        if self.window_topology_snapshot_refs.is_empty() {
            return Err(PortableStatePackageError::MissingField(
                "layout_snapshot.window_topology_snapshot_refs",
            ));
        }
        Ok(())
    }
}

/// Portable profile summary carried by a package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableProfileSnapshot {
    /// Stable projection ref for the portable profile summary.
    pub profile_export_ref: String,
    /// Portable-profile schema version.
    pub schema_version: u32,
    /// Profile id.
    pub profile_id: String,
    /// Explicit profile scope.
    pub profile_scope: String,
    /// Profile revision captured by this export.
    pub profile_revision_ref: String,
    /// Source device ref.
    pub source_device_ref: String,
    /// Portable artifact refs included by the profile export.
    pub portable_artifact_refs: Vec<String>,
    /// Non-portable exclusion reason tokens named by the export.
    pub excluded_reason_tokens: Vec<String>,
    /// Optional settings support-export ref used as profile inspector evidence.
    pub settings_inspector_export_ref: Option<String>,
    /// Settings inspector source record refs.
    pub settings_record_refs: Vec<String>,
    /// Number of literal settings values redacted from the settings export.
    pub redacted_settings_count: usize,
    /// Number of settings locked or constrained by policy in the settings export.
    pub policy_locked_settings_count: usize,
}

impl PortableProfileSnapshot {
    /// Projects a validated portable profile export into recovery metadata.
    pub fn from_profile_export(
        profile_export_ref: impl Into<String>,
        export: &PortableProfileExport,
    ) -> Result<Self, PortableStatePackageError> {
        export
            .validate()
            .map_err(|err| PortableStatePackageError::InvalidPortableProfile(err.to_string()))?;

        let mut portable_artifact_refs = export
            .artifacts
            .iter()
            .filter(|artifact| artifact.portability_label == ArtifactPortabilityLabel::Portable)
            .map(|artifact| artifact.artifact_id.clone())
            .collect::<Vec<_>>();
        sort_unique(&mut portable_artifact_refs);

        let mut excluded_reason_tokens = export
            .non_portable_exclusions
            .iter()
            .map(|reason| {
                serde_json::to_value(reason)
                    .expect("enum serializes")
                    .to_string()
            })
            .map(|value| value.trim_matches('"').to_string())
            .collect::<Vec<_>>();
        sort_unique(&mut excluded_reason_tokens);

        Ok(Self {
            profile_export_ref: profile_export_ref.into(),
            schema_version: export.schema_version,
            profile_id: export.profile_id.clone(),
            profile_scope: export.profile_scope.clone(),
            profile_revision_ref: export.profile_revision_ref.clone(),
            source_device_ref: export.source_device_ref.clone(),
            portable_artifact_refs,
            excluded_reason_tokens,
            settings_inspector_export_ref: None,
            settings_record_refs: Vec::new(),
            redacted_settings_count: 0,
            policy_locked_settings_count: 0,
        })
    }

    /// Adds settings-inspector support export evidence to the profile summary.
    pub fn with_settings_support_export(
        mut self,
        export: &SettingsSupportExportProjection,
    ) -> Self {
        self.settings_inspector_export_ref = Some(export.export_id.clone());
        self.settings_record_refs = export
            .effective_settings
            .iter()
            .map(|record| record.source_record_ref.clone())
            .collect();
        sort_unique(&mut self.settings_record_refs);
        self.redacted_settings_count = export.redacted_value_count;
        self.policy_locked_settings_count = export.policy_locked_count;
        self
    }

    /// Validates required portable profile attribution and exclusion floor.
    pub fn validate(&self) -> Result<(), PortableStatePackageError> {
        require_non_empty(
            "portable_profile.profile_export_ref",
            &self.profile_export_ref,
        )?;
        require_non_empty("portable_profile.profile_id", &self.profile_id)?;
        require_non_empty("portable_profile.profile_scope", &self.profile_scope)?;
        require_non_empty(
            "portable_profile.profile_revision_ref",
            &self.profile_revision_ref,
        )?;
        require_non_empty(
            "portable_profile.source_device_ref",
            &self.source_device_ref,
        )?;
        if self.portable_artifact_refs.is_empty() {
            return Err(PortableStatePackageError::MissingComponent(
                PortableStateComponentClass::PortableProfile,
            ));
        }
        for required in ["secret_material", "delegated_credential"] {
            if !self
                .excluded_reason_tokens
                .iter()
                .any(|reason| reason == required)
            {
                return Err(PortableStatePackageError::MissingProfileExclusion(required));
            }
        }
        Ok(())
    }
}

/// Metadata-only transfer action summary included in portable-state packages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferIntegrityRecord {
    /// Stable transfer action id.
    pub action_id: String,
    /// Stable action-kind token.
    pub action_kind_token: String,
    /// Stable source-surface token.
    pub source_surface_token: String,
    /// Stable representation-class token.
    pub representation_class_token: String,
    /// Stable recovery-class token.
    pub recovery_class_token: String,
    /// Redaction-safe target ref.
    pub target_ref: String,
    /// Stable boundary-class token.
    pub boundary_class_token: String,
    /// Checkpoint ref when the transfer is recovered through a checkpoint.
    pub checkpoint_ref: Option<String>,
    /// True when the transfer record carries metadata only.
    pub metadata_only: bool,
    /// True if a raw clipboard body is present.
    pub raw_clipboard_body_present: bool,
    /// True if a raw terminal payload is present.
    pub raw_terminal_payload_present: bool,
    /// True if a raw terminal command body is present.
    pub raw_terminal_command_body_present: bool,
    /// True if raw source or file body content is present.
    pub raw_source_body_present: bool,
    /// True when replay or rerun is forbidden for the action.
    pub auto_rerun_forbidden: bool,
    /// Fixture or monotonic timestamp.
    pub minted_at: String,
}

impl TransferIntegrityRecord {
    /// Builds a metadata-only transfer integrity row.
    #[allow(clippy::too_many_arguments)]
    pub fn metadata_only(
        action_id: impl Into<String>,
        action_kind_token: impl Into<String>,
        source_surface_token: impl Into<String>,
        representation_class_token: impl Into<String>,
        recovery_class_token: impl Into<String>,
        target_ref: impl Into<String>,
        boundary_class_token: impl Into<String>,
        checkpoint_ref: Option<String>,
        auto_rerun_forbidden: bool,
        minted_at: impl Into<String>,
    ) -> Self {
        Self {
            action_id: action_id.into(),
            action_kind_token: action_kind_token.into(),
            source_surface_token: source_surface_token.into(),
            representation_class_token: representation_class_token.into(),
            recovery_class_token: recovery_class_token.into(),
            target_ref: target_ref.into(),
            boundary_class_token: boundary_class_token.into(),
            checkpoint_ref,
            metadata_only: true,
            raw_clipboard_body_present: false,
            raw_terminal_payload_present: false,
            raw_terminal_command_body_present: false,
            raw_source_body_present: false,
            auto_rerun_forbidden,
            minted_at: minted_at.into(),
        }
    }

    /// Validates that no raw transfer payload crossed the package boundary.
    pub fn validate(&self) -> Result<(), PortableStatePackageError> {
        require_non_empty("transfer_integrity.action_id", &self.action_id)?;
        require_non_empty(
            "transfer_integrity.action_kind_token",
            &self.action_kind_token,
        )?;
        require_non_empty(
            "transfer_integrity.source_surface_token",
            &self.source_surface_token,
        )?;
        require_non_empty(
            "transfer_integrity.representation_class_token",
            &self.representation_class_token,
        )?;
        require_non_empty(
            "transfer_integrity.recovery_class_token",
            &self.recovery_class_token,
        )?;
        require_non_empty("transfer_integrity.target_ref", &self.target_ref)?;
        require_non_empty(
            "transfer_integrity.boundary_class_token",
            &self.boundary_class_token,
        )?;
        require_non_empty("transfer_integrity.minted_at", &self.minted_at)?;
        if !self.metadata_only {
            return Err(PortableStatePackageError::RawPayloadIncluded(
                "transfer_integrity.metadata_only=false",
            ));
        }
        for (field, present) in [
            (
                "transfer_integrity.raw_clipboard_body_present",
                self.raw_clipboard_body_present,
            ),
            (
                "transfer_integrity.raw_terminal_payload_present",
                self.raw_terminal_payload_present,
            ),
            (
                "transfer_integrity.raw_terminal_command_body_present",
                self.raw_terminal_command_body_present,
            ),
            (
                "transfer_integrity.raw_source_body_present",
                self.raw_source_body_present,
            ),
        ] {
            if present {
                return Err(PortableStatePackageError::RawPayloadIncluded(field));
            }
        }
        Ok(())
    }
}

/// Transfer-integrity packet summary included in a portable-state package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TransferIntegrityRecords {
    /// Stable packet ref.
    pub packet_ref: String,
    /// Support export ref for this transfer packet.
    pub support_export_ref: String,
    /// Transfer action metadata rows.
    pub actions: Vec<TransferIntegrityRecord>,
    /// Payload classes intentionally omitted from the package.
    pub omitted_payload_classes: Vec<String>,
    /// True only through a separate high-friction raw-body path.
    pub raw_payload_bodies_included: bool,
}

impl TransferIntegrityRecords {
    /// Builds a metadata-only transfer-integrity packet.
    pub fn metadata_only(
        packet_ref: impl Into<String>,
        support_export_ref: impl Into<String>,
        mut actions: Vec<TransferIntegrityRecord>,
    ) -> Self {
        actions.sort_by(|left, right| left.action_id.cmp(&right.action_id));
        Self {
            packet_ref: packet_ref.into(),
            support_export_ref: support_export_ref.into(),
            actions,
            omitted_payload_classes: vec![
                "raw_clipboard_body".to_string(),
                "raw_terminal_payload".to_string(),
                "raw_terminal_command_body".to_string(),
                "raw_source_body".to_string(),
            ],
            raw_payload_bodies_included: false,
        }
    }

    /// Validates transfer metadata-only export rules.
    pub fn validate(&self) -> Result<(), PortableStatePackageError> {
        require_non_empty("transfer_integrity.packet_ref", &self.packet_ref)?;
        require_non_empty(
            "transfer_integrity.support_export_ref",
            &self.support_export_ref,
        )?;
        if self.actions.is_empty() {
            return Err(PortableStatePackageError::MissingComponent(
                PortableStateComponentClass::TransferIntegrityRecords,
            ));
        }
        if self.raw_payload_bodies_included {
            return Err(PortableStatePackageError::RawPayloadIncluded(
                "transfer_integrity.raw_payload_bodies_included",
            ));
        }
        for required in ["raw_clipboard_body", "raw_terminal_payload"] {
            if !self
                .omitted_payload_classes
                .iter()
                .any(|omitted| omitted == required)
            {
                return Err(PortableStatePackageError::MissingOmittedPayloadClass(
                    required,
                ));
            }
        }
        for action in &self.actions {
            action.validate()?;
        }
        Ok(())
    }

    /// Returns every checkpoint ref cited by transfer records.
    pub fn checkpoint_refs(&self) -> Vec<String> {
        let mut refs = self
            .actions
            .iter()
            .filter_map(|action| action.checkpoint_ref.clone())
            .collect::<Vec<_>>();
        sort_unique(&mut refs);
        refs
    }
}

/// Local-history restore checkpoint metadata linked by the package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreCheckpointProjection {
    /// Stable checkpoint entry or group ref.
    pub checkpoint_ref: String,
    /// Human-readable checkpoint name.
    pub checkpoint_name: String,
    /// Timestamp copied from the checkpoint record.
    pub created_at: String,
    /// Existing command id that opens the checkpoint restore path.
    pub restore_command_id: String,
    /// Existing local-history snapshot class token.
    pub resulting_snapshot_class: String,
    /// True only if raw body refs were intentionally exported.
    pub raw_body_refs_exported: bool,
}

impl RestoreCheckpointProjection {
    /// Builds a restore-checkpoint projection with existing local-history constants.
    pub fn new(
        checkpoint_ref: impl Into<String>,
        checkpoint_name: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Self {
        Self {
            checkpoint_ref: checkpoint_ref.into(),
            checkpoint_name: checkpoint_name.into(),
            created_at: created_at.into(),
            restore_command_id: RESTORE_CHECKPOINT_COMMAND_ID.to_string(),
            resulting_snapshot_class: RESTORE_CHECKPOINT_SNAPSHOT_CLASS.to_string(),
            raw_body_refs_exported: false,
        }
    }

    /// Projects a local-history restore checkpoint into the package shape.
    pub fn from_history_checkpoint(checkpoint: &RestoreCheckpointAlpha) -> Self {
        Self {
            checkpoint_ref: checkpoint.checkpoint_ref.clone(),
            checkpoint_name: checkpoint.checkpoint_name.clone(),
            created_at: checkpoint.created_at.clone(),
            restore_command_id: checkpoint.restore_command_id.clone(),
            resulting_snapshot_class: checkpoint.resulting_snapshot_class.clone(),
            raw_body_refs_exported: checkpoint.raw_body_refs_exported,
        }
    }

    /// Validates the restore-checkpoint projection.
    pub fn validate(&self) -> Result<(), PortableStatePackageError> {
        require_non_empty("restore_checkpoint.checkpoint_ref", &self.checkpoint_ref)?;
        require_non_empty("restore_checkpoint.checkpoint_name", &self.checkpoint_name)?;
        require_non_empty("restore_checkpoint.created_at", &self.created_at)?;
        if self.restore_command_id != RESTORE_CHECKPOINT_COMMAND_ID {
            return Err(
                PortableStatePackageError::RestoreCheckpointConstantMismatch("restore_command_id"),
            );
        }
        if self.resulting_snapshot_class != RESTORE_CHECKPOINT_SNAPSHOT_CLASS {
            return Err(
                PortableStatePackageError::RestoreCheckpointConstantMismatch(
                    "resulting_snapshot_class",
                ),
            );
        }
        if self.raw_body_refs_exported {
            return Err(PortableStatePackageError::RawPayloadIncluded(
                "restore_checkpoint.raw_body_refs_exported",
            ));
        }
        Ok(())
    }
}

/// Restore-provenance lineage preserved across the aggregate package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProvenanceLineage {
    /// Stable lineage ref for this package projection.
    pub lineage_ref: String,
    /// Source topology snapshot refs.
    pub source_snapshot_refs: Vec<String>,
    /// Restore-provenance record refs.
    pub restore_provenance_refs: Vec<String>,
    /// Restore checkpoint refs preserved by the package.
    pub checkpoint_refs: Vec<String>,
    /// Preserved prior artifact refs carried forward by the restore proposal.
    pub preserved_prior_artifact_refs: Vec<String>,
    /// Session-restore checkpoint ref from the proposal.
    pub session_restore_checkpoint_ref: Option<String>,
    /// Session-restore snapshot ref from the proposal.
    pub session_restore_snapshot_ref: Option<String>,
    /// Existing command id used for checkpoint restore.
    pub restore_checkpoint_command_id: String,
}

impl RestoreProvenanceLineage {
    /// Builds lineage from the package components.
    pub fn from_components(
        lineage_ref: impl Into<String>,
        layout: &LayoutSerializationSnapshot,
        restore_proposal: &RestoreProposal,
        transfer_integrity: &TransferIntegrityRecords,
        restore_checkpoint: &RestoreCheckpointProjection,
    ) -> Self {
        let mut source_snapshot_refs = layout.window_topology_snapshot_refs.clone();
        if let Some(snapshot_id) = &restore_proposal.artifact_refs.snapshot_id {
            source_snapshot_refs.push(snapshot_id.clone());
        }
        sort_unique(&mut source_snapshot_refs);

        let mut restore_provenance_refs = layout.restore_provenance_refs.clone();
        sort_unique(&mut restore_provenance_refs);

        let mut checkpoint_refs = transfer_integrity.checkpoint_refs();
        checkpoint_refs.push(restore_checkpoint.checkpoint_ref.clone());
        if let Some(checkpoint_id) = &restore_proposal.artifact_refs.checkpoint_id {
            checkpoint_refs.push(checkpoint_id.clone());
        }
        sort_unique(&mut checkpoint_refs);

        Self {
            lineage_ref: lineage_ref.into(),
            source_snapshot_refs,
            restore_provenance_refs,
            checkpoint_refs,
            preserved_prior_artifact_refs: Vec::new(),
            session_restore_checkpoint_ref: restore_proposal.artifact_refs.checkpoint_id.clone(),
            session_restore_snapshot_ref: restore_proposal.artifact_refs.snapshot_id.clone(),
            restore_checkpoint_command_id: RESTORE_CHECKPOINT_COMMAND_ID.to_string(),
        }
    }

    /// Validates that component lineage is preserved.
    pub fn validate(
        &self,
        layout: &LayoutSerializationSnapshot,
        restore_proposal: &RestoreProposal,
        restore_checkpoint: &RestoreCheckpointProjection,
    ) -> Result<(), PortableStatePackageError> {
        require_non_empty("restore_provenance_lineage.lineage_ref", &self.lineage_ref)?;
        if self.restore_checkpoint_command_id != RESTORE_CHECKPOINT_COMMAND_ID {
            return Err(
                PortableStatePackageError::RestoreCheckpointConstantMismatch(
                    "restore_provenance_lineage.restore_checkpoint_command_id",
                ),
            );
        }
        for snapshot_ref in &layout.window_topology_snapshot_refs {
            if !self.source_snapshot_refs.contains(snapshot_ref) {
                return Err(PortableStatePackageError::LineageMissingRef {
                    field: "source_snapshot_refs",
                    missing_ref: snapshot_ref.clone(),
                });
            }
        }
        for provenance_ref in &layout.restore_provenance_refs {
            if !self.restore_provenance_refs.contains(provenance_ref) {
                return Err(PortableStatePackageError::LineageMissingRef {
                    field: "restore_provenance_refs",
                    missing_ref: provenance_ref.clone(),
                });
            }
        }
        if !self
            .checkpoint_refs
            .contains(&restore_checkpoint.checkpoint_ref)
        {
            return Err(PortableStatePackageError::LineageMissingRef {
                field: "checkpoint_refs",
                missing_ref: restore_checkpoint.checkpoint_ref.clone(),
            });
        }
        if let Some(checkpoint_id) = &restore_proposal.artifact_refs.checkpoint_id {
            if self.session_restore_checkpoint_ref.as_ref() != Some(checkpoint_id) {
                return Err(PortableStatePackageError::LineageMissingRef {
                    field: "session_restore_checkpoint_ref",
                    missing_ref: checkpoint_id.clone(),
                });
            }
        }
        if let Some(snapshot_id) = &restore_proposal.artifact_refs.snapshot_id {
            if self.session_restore_snapshot_ref.as_ref() != Some(snapshot_id) {
                return Err(PortableStatePackageError::LineageMissingRef {
                    field: "session_restore_snapshot_ref",
                    missing_ref: snapshot_id.clone(),
                });
            }
        }
        Ok(())
    }
}

/// One component ref selected into the aggregate package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateSelectedComponent {
    /// Component class.
    pub component_class: PortableStateComponentClass,
    /// Stable component ref.
    pub component_ref: String,
    /// Schema ref for the component family.
    pub schema_ref: String,
    /// Redaction-aware notes.
    pub notes: String,
}

/// Input used to build a portable-state package record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortableStatePackageInput {
    /// Stable package id.
    pub package_id: String,
    /// Stable manifest id.
    pub manifest_id: String,
    /// Redaction-aware package label.
    pub package_label: String,
    /// Package purpose.
    pub package_purpose: PackagePurpose,
    /// Export destination class.
    pub destination_class: DestinationClass,
    /// Package creation timestamp.
    pub created_at: String,
    /// Producer build stamp.
    pub producer_build: ProducerBuildStamp,
    /// Shell layout serialization snapshot.
    pub layout_snapshot: LayoutSerializationSnapshot,
    /// Portable profile summary.
    pub portable_profile: PortableProfileSnapshot,
    /// Session-restore proposal.
    pub session_restore_proposal: RestoreProposal,
    /// Transfer-integrity records.
    pub transfer_integrity: TransferIntegrityRecords,
    /// Restore checkpoint projection.
    pub restore_checkpoint: RestoreCheckpointProjection,
    /// Optional package notes.
    pub notes: Option<String>,
}

/// Aggregate portable-state package record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStatePackageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Projection schema version.
    pub portable_state_schema_version: u32,
    /// Stable package id.
    pub package_id: String,
    /// Portable-state manifest id.
    pub manifest_id: String,
    /// Redaction-aware package label.
    pub package_label: String,
    /// Package purpose.
    pub package_purpose: PackagePurpose,
    /// Export destination class.
    pub destination_class: DestinationClass,
    /// Package creation timestamp.
    pub created_at: String,
    /// Producer build stamp.
    pub producer_build: ProducerBuildStamp,
    /// Shell layout serialization snapshot.
    pub layout_snapshot: LayoutSerializationSnapshot,
    /// Portable profile summary.
    pub portable_profile: PortableProfileSnapshot,
    /// Session-restore proposal captured before hydration.
    pub session_restore_proposal: RestoreProposal,
    /// Metadata-only transfer-integrity packet.
    pub transfer_integrity: TransferIntegrityRecords,
    /// Restore checkpoint projection.
    pub restore_checkpoint: RestoreCheckpointProjection,
    /// Package redaction manifest.
    pub redaction_manifest: PortableStateRedactionManifest,
    /// Restore-provenance lineage carried through the aggregate.
    pub restore_provenance_lineage: RestoreProvenanceLineage,
    /// Selected component refs in deterministic order.
    pub selected_components: Vec<PortableStateSelectedComponent>,
    /// Redaction-aware notes.
    pub notes: String,
}

impl PortableStatePackageRecord {
    /// Builds and validates a portable-state package record from component snapshots.
    pub fn build(input: PortableStatePackageInput) -> Result<Self, PortableStatePackageError> {
        let redaction_manifest = PortableStateRedactionManifest::default_for(&input.package_id);
        let restore_provenance_lineage = RestoreProvenanceLineage::from_components(
            format!("restore-lineage:{}", input.package_id),
            &input.layout_snapshot,
            &input.session_restore_proposal,
            &input.transfer_integrity,
            &input.restore_checkpoint,
        );
        let selected_components = selected_components(
            &input.layout_snapshot,
            &input.portable_profile,
            &input.session_restore_proposal,
            &input.transfer_integrity,
            &input.restore_checkpoint,
            &restore_provenance_lineage,
        );
        let record = Self {
            record_kind: PORTABLE_STATE_PACKAGE_RECORD_KIND.to_string(),
            portable_state_schema_version: PORTABLE_STATE_PACKAGE_SCHEMA_VERSION,
            package_id: input.package_id,
            manifest_id: input.manifest_id,
            package_label: input.package_label,
            package_purpose: input.package_purpose,
            destination_class: input.destination_class,
            created_at: input.created_at,
            producer_build: input.producer_build,
            layout_snapshot: input.layout_snapshot,
            portable_profile: input.portable_profile,
            session_restore_proposal: input.session_restore_proposal,
            transfer_integrity: input.transfer_integrity,
            restore_checkpoint: input.restore_checkpoint,
            redaction_manifest,
            restore_provenance_lineage,
            selected_components,
            notes: input.notes.unwrap_or_else(|| {
                "Portable-state package projection carries layout, profile, restore, transfer, and lineage metadata with raw payloads excluded."
                    .to_string()
            }),
        };
        record.validate()?;
        Ok(record)
    }

    /// Validates package attribution, redaction, raw-payload, and lineage invariants.
    pub fn validate(&self) -> Result<(), PortableStatePackageError> {
        if self.record_kind != PORTABLE_STATE_PACKAGE_RECORD_KIND {
            return Err(PortableStatePackageError::WrongRecordKind {
                expected: PORTABLE_STATE_PACKAGE_RECORD_KIND,
                actual: self.record_kind.clone(),
            });
        }
        if self.portable_state_schema_version != PORTABLE_STATE_PACKAGE_SCHEMA_VERSION {
            return Err(PortableStatePackageError::WrongSchemaVersion {
                expected: PORTABLE_STATE_PACKAGE_SCHEMA_VERSION,
                actual: self.portable_state_schema_version,
            });
        }
        require_non_empty("package_id", &self.package_id)?;
        require_non_empty("manifest_id", &self.manifest_id)?;
        require_non_empty("package_label", &self.package_label)?;
        require_non_empty("created_at", &self.created_at)?;
        require_non_empty(
            "producer_build.producer_name",
            &self.producer_build.producer_name,
        )?;
        require_non_empty(
            "producer_build.producer_version",
            &self.producer_build.producer_version,
        )?;
        self.layout_snapshot.validate()?;
        self.portable_profile.validate()?;
        validate_restore_proposal(&self.session_restore_proposal)?;
        self.transfer_integrity.validate()?;
        self.restore_checkpoint.validate()?;
        self.redaction_manifest.validate()?;
        self.restore_provenance_lineage.validate(
            &self.layout_snapshot,
            &self.session_restore_proposal,
            &self.restore_checkpoint,
        )?;
        validate_selected_components(&self.selected_components)?;
        Ok(())
    }

    /// Serializes the package to deterministic JSON bytes after validation.
    pub fn pack(&self) -> Result<Vec<u8>, PortableStatePackageError> {
        self.validate()?;
        serde_json::to_vec(self).map_err(PortableStatePackageError::Json)
    }

    /// Deserializes and validates a portable-state package from JSON bytes.
    pub fn unpack(bytes: &[u8]) -> Result<Self, PortableStatePackageError> {
        let package =
            serde_json::from_slice::<Self>(bytes).map_err(PortableStatePackageError::Json)?;
        package.validate()?;
        Ok(package)
    }

    /// Returns a stable BLAKE3 digest over the packed package bytes.
    pub fn stable_digest(&self) -> Result<String, PortableStatePackageError> {
        let bytes = self.pack()?;
        Ok(format!("blake3:{}", blake3::hash(&bytes).to_hex()))
    }
}

/// Errors returned by portable-state package projection.
#[derive(Debug)]
pub enum PortableStatePackageError {
    /// The record kind does not match the package projection.
    WrongRecordKind {
        /// Expected record-kind tag.
        expected: &'static str,
        /// Actual record-kind tag.
        actual: String,
    },
    /// The schema version does not match this crate's package projection.
    WrongSchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// A required field was empty.
    MissingField(&'static str),
    /// A required source component is absent.
    MissingComponent(PortableStateComponentClass),
    /// The layout package failed its source validation.
    InvalidLayoutPackage(String),
    /// The portable profile failed its source validation.
    InvalidPortableProfile(String),
    /// A required redaction rule is missing.
    MissingRedactionRule(PortableStateRedactionRule),
    /// Redaction defaults were not applied.
    RedactionDefaultsMissing,
    /// A raw clipboard, terminal, source, or body payload crossed the boundary.
    RawPayloadIncluded(&'static str),
    /// A required omitted payload class is missing.
    MissingOmittedPayloadClass(&'static str),
    /// A required portable-profile exclusion reason is missing.
    MissingProfileExclusion(&'static str),
    /// Restore-checkpoint values drifted from the shared local-history constants.
    RestoreCheckpointConstantMismatch(&'static str),
    /// Restore lineage dropped a required ref.
    LineageMissingRef {
        /// Lineage field that should contain the ref.
        field: &'static str,
        /// Missing ref.
        missing_ref: String,
    },
    /// The session-restore proposal is not export-safe.
    InvalidRestoreProposal(&'static str),
    /// A selected component row is duplicated or incomplete.
    InvalidSelectedComponent(String),
    /// JSON serialization or deserialization failed.
    Json(serde_json::Error),
}

impl fmt::Display for PortableStatePackageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind { expected, actual } => {
                write!(f, "expected record kind {expected}, got {actual}")
            }
            Self::WrongSchemaVersion { expected, actual } => {
                write!(f, "expected schema version {expected}, got {actual}")
            }
            Self::MissingField(field) => write!(f, "missing required field {field}"),
            Self::MissingComponent(component) => {
                write!(f, "missing component {}", component.as_str())
            }
            Self::InvalidLayoutPackage(detail) => write!(f, "invalid layout package: {detail}"),
            Self::InvalidPortableProfile(detail) => {
                write!(f, "invalid portable profile: {detail}")
            }
            Self::MissingRedactionRule(rule) => write!(f, "missing redaction rule {rule:?}"),
            Self::RedactionDefaultsMissing => write!(f, "redaction defaults were not applied"),
            Self::RawPayloadIncluded(field) => write!(f, "raw payload included at {field}"),
            Self::MissingOmittedPayloadClass(class) => {
                write!(f, "missing omitted payload class {class}")
            }
            Self::MissingProfileExclusion(reason) => {
                write!(f, "missing portable profile exclusion reason {reason}")
            }
            Self::RestoreCheckpointConstantMismatch(field) => {
                write!(f, "restore-checkpoint constant mismatch at {field}")
            }
            Self::LineageMissingRef { field, missing_ref } => {
                write!(f, "restore lineage {field} is missing {missing_ref}")
            }
            Self::InvalidRestoreProposal(field) => {
                write!(f, "invalid session restore proposal at {field}")
            }
            Self::InvalidSelectedComponent(component_ref) => {
                write!(f, "invalid selected component {component_ref}")
            }
            Self::Json(err) => write!(f, "portable-state package json error: {err}"),
        }
    }
}

impl std::error::Error for PortableStatePackageError {}

const REQUIRED_REDACTION_RULES: [PortableStateRedactionRule; 9] = [
    PortableStateRedactionRule::RawSecretMaterialExcluded,
    PortableStateRedactionRule::ApprovalTicketExcluded,
    PortableStateRedactionRule::DelegatedCredentialExcluded,
    PortableStateRedactionRule::LiveAuthorityHandleExcluded,
    PortableStateRedactionRule::MachineUniqueHandleExcluded,
    PortableStateRedactionRule::StateRootExcluded,
    PortableStateRedactionRule::RawClipboardBodyExcluded,
    PortableStateRedactionRule::RawTerminalPayloadExcluded,
    PortableStateRedactionRule::ProviderPayloadExcluded,
];

fn default_redaction_rules() -> Vec<PortableStateRedactionRule> {
    vec![
        PortableStateRedactionRule::RawSecretMaterialExcluded,
        PortableStateRedactionRule::ApprovalTicketExcluded,
        PortableStateRedactionRule::DelegatedCredentialExcluded,
        PortableStateRedactionRule::LiveAuthorityHandleExcluded,
        PortableStateRedactionRule::MachineUniqueHandleExcluded,
        PortableStateRedactionRule::StateRootExcluded,
        PortableStateRedactionRule::RawClipboardBodyExcluded,
        PortableStateRedactionRule::RawTerminalPayloadExcluded,
        PortableStateRedactionRule::RawCommandLineExcluded,
        PortableStateRedactionRule::RawEnvironmentBodyExcluded,
        PortableStateRedactionRule::RawLogExcluded,
        PortableStateRedactionRule::RawSourceContentExcluded,
        PortableStateRedactionRule::ProviderPayloadExcluded,
    ]
}

fn selected_components(
    layout: &LayoutSerializationSnapshot,
    profile: &PortableProfileSnapshot,
    proposal: &RestoreProposal,
    transfer: &TransferIntegrityRecords,
    checkpoint: &RestoreCheckpointProjection,
    lineage: &RestoreProvenanceLineage,
) -> Vec<PortableStateSelectedComponent> {
    let mut rows = vec![
        PortableStateSelectedComponent {
            component_class: PortableStateComponentClass::LayoutSerializationSnapshot,
            component_ref: layout.snapshot_ref.clone(),
            schema_ref: "schemas/workspace/portable_state_alpha.schema.json".to_string(),
            notes: "Shell layout package snapshot summary.".to_string(),
        },
        PortableStateSelectedComponent {
            component_class: PortableStateComponentClass::PortableProfile,
            component_ref: profile.profile_export_ref.clone(),
            schema_ref: "schemas/profile/portable_profile.schema.json".to_string(),
            notes: "Portable profile export summary with non-portable exclusions.".to_string(),
        },
        PortableStateSelectedComponent {
            component_class: PortableStateComponentClass::SessionRestoreProposal,
            component_ref: proposal
                .artifact_refs
                .snapshot_id
                .clone()
                .unwrap_or_else(|| "session-restore-proposal:metadata-only".to_string()),
            schema_ref: "schemas/recovery/restore_preview.schema.json".to_string(),
            notes: "Pre-hydration session restore proposal.".to_string(),
        },
        PortableStateSelectedComponent {
            component_class: PortableStateComponentClass::TransferIntegrityRecords,
            component_ref: transfer.packet_ref.clone(),
            schema_ref: "schemas/events/transfer_action.schema.json".to_string(),
            notes: "Metadata-only transfer integrity rows.".to_string(),
        },
        PortableStateSelectedComponent {
            component_class: PortableStateComponentClass::RestoreCheckpoint,
            component_ref: checkpoint.checkpoint_ref.clone(),
            schema_ref: "schemas/history/local_history_alpha.schema.json".to_string(),
            notes: "Local-history restore checkpoint projection.".to_string(),
        },
        PortableStateSelectedComponent {
            component_class: PortableStateComponentClass::RestoreProvenanceLineage,
            component_ref: lineage.lineage_ref.clone(),
            schema_ref: PORTABLE_STATE_PACKAGE_SCHEMA_REF.to_string(),
            notes: "Aggregate restore-provenance lineage.".to_string(),
        },
    ];
    rows.sort_by(|left, right| {
        left.component_class
            .cmp(&right.component_class)
            .then(left.component_ref.cmp(&right.component_ref))
    });
    rows
}

fn validate_selected_components(
    components: &[PortableStateSelectedComponent],
) -> Result<(), PortableStatePackageError> {
    let mut seen = BTreeSet::new();
    for component in components {
        require_non_empty(
            "selected_components.component_ref",
            &component.component_ref,
        )?;
        require_non_empty("selected_components.schema_ref", &component.schema_ref)?;
        if !seen.insert((component.component_class, component.component_ref.as_str())) {
            return Err(PortableStatePackageError::InvalidSelectedComponent(
                component.component_ref.clone(),
            ));
        }
    }
    for required in [
        PortableStateComponentClass::LayoutSerializationSnapshot,
        PortableStateComponentClass::PortableProfile,
        PortableStateComponentClass::SessionRestoreProposal,
        PortableStateComponentClass::TransferIntegrityRecords,
        PortableStateComponentClass::RestoreCheckpoint,
        PortableStateComponentClass::RestoreProvenanceLineage,
    ] {
        if !components
            .iter()
            .any(|component| component.component_class == required)
        {
            return Err(PortableStatePackageError::MissingComponent(required));
        }
    }
    Ok(())
}

fn validate_restore_proposal(proposal: &RestoreProposal) -> Result<(), PortableStatePackageError> {
    if proposal.record_kind != "restore_proposal_record" {
        return Err(PortableStatePackageError::InvalidRestoreProposal(
            "record_kind",
        ));
    }
    if !proposal.auto_rerun_forbidden {
        return Err(PortableStatePackageError::InvalidRestoreProposal(
            "auto_rerun_forbidden",
        ));
    }
    if matches!(proposal.restore_class, RestoreClass::NoRestore)
        && proposal.artifact_refs.checkpoint_id.is_none()
        && proposal.artifact_refs.snapshot_id.is_none()
    {
        return Err(PortableStatePackageError::InvalidRestoreProposal(
            "artifact_refs",
        ));
    }
    Ok(())
}

fn require_non_empty(field: &'static str, value: &str) -> Result<(), PortableStatePackageError> {
    if value.trim().is_empty() {
        Err(PortableStatePackageError::MissingField(field))
    } else {
        Ok(())
    }
}

fn sort_unique(values: &mut Vec<String>) {
    values.sort();
    values.dedup();
}
