//! Workspace serialization, portable-state review, and restore-provenance beta records.
//!
//! This module owns the beta boundary for remembered workspace state. It
//! keeps workspace authority, window topology, profile defaults, and
//! machine-local hints in separate rows so export, import, restore, support,
//! and diagnostics surfaces can quote the same truth without serializing live
//! authority.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::state_packages::{
    ExportMode as AlphaExportMode, MachineLocalExclusion as AlphaMachineLocalExclusion,
    MachineLocalExclusionReason as AlphaMachineLocalExclusionReason,
    PaneRestorePosture as AlphaPaneRestorePosture, PersistenceClassification,
    PlaceholderAction as AlphaPlaceholderAction, PlaceholderReason as AlphaPlaceholderReason,
    PortableStateAlphaPackage, RedactionRuleClass, RestoreCandidateClass, SerializedStateClass,
    SurfaceRestorePosture,
};

/// Schema version for the workspace portable-state beta package.
pub const WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION: u32 = 1;

/// Schema path for the workspace portable-state beta package.
pub const WORKSPACE_PORTABLE_STATE_PACKAGE_SCHEMA_REF: &str =
    "schemas/workspace/portable_state_package.schema.json";

/// Schema path for the workspace restore-provenance beta card.
pub const WORKSPACE_RESTORE_PROVENANCE_SCHEMA_REF: &str =
    "schemas/workspace/restore_provenance.schema.json";

/// Schema path for the pane-tree topology body.
pub const WORKSPACE_PANE_TREE_SCHEMA_REF: &str = "schemas/workspace/pane_tree.schema.json";

/// Record discriminator for workspace serialization beta records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceSerializationRecordKind {
    /// Versioned workspace portable-state package.
    WorkspacePortableStatePackageRecord,
    /// Remembered-state inspection projection.
    RememberedStateInspectionRecord,
    /// Export or import review sheet projection.
    PortableStateReviewSheetRecord,
}

/// Serialized state layer kept separate in a workspace package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceStateLayer {
    /// Shared workspace authority and durable workspace truth refs.
    WorkspaceAuthority,
    /// Window-local pane tree, focus, and placeholder posture.
    WindowTopology,
    /// Portable profile defaults linked by explicit artifact refs.
    ProfileDefaults,
    /// Machine-local display, state-root, trust-anchor, or install hints.
    MachineLocalHints,
    /// Local session context such as terminal transcript or notebook evidence.
    LocalSessionContext,
}

impl WorkspaceStateLayer {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceAuthority => "workspace_authority",
            Self::WindowTopology => "window_topology",
            Self::ProfileDefaults => "profile_defaults",
            Self::MachineLocalHints => "machine_local_hints",
            Self::LocalSessionContext => "local_session_context",
        }
    }
}

/// Persistence class shown by remembered-state and review surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspacePersistenceClass {
    /// State stays local to the current machine or user profile.
    LocalOnly,
    /// State may travel in a reviewed package.
    Portable,
    /// State is workspace-shared and must be applied through workspace review.
    Shared,
    /// State is machine-local metadata, not portable authority.
    MachineLocal,
}

impl WorkspacePersistenceClass {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::Portable => "portable",
            Self::Shared => "shared",
            Self::MachineLocal => "machine_local",
        }
    }
}

/// How a layer is represented in a portable-state package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceExportMode {
    /// The package carries the schema-backed body.
    CarriedBody,
    /// The package carries opaque refs to the body.
    ReferencedBody,
    /// The package links a separate portable artifact.
    LinkedArtifactRef,
    /// The package carries metadata only.
    MetadataOnly,
    /// The package intentionally excludes this layer.
    Excluded,
}

/// Restore-fidelity label reused by restore, diagnostics, and support surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceRestoreFidelity {
    /// Exact restore with no translation, placeholder, or review downgrade.
    ExactRestore,
    /// Compatible restore through a declared translation or adjustment.
    CompatibleRestore,
    /// Layout and context restored without live authority.
    LayoutOnly,
    /// Dirty drafts were recovered for compare or review.
    RecoveredDrafts,
    /// Only evidence, transcripts, snapshots, and provenance survived.
    EvidenceOnly,
}

impl WorkspaceRestoreFidelity {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::RecoveredDrafts => "recovered_drafts",
            Self::EvidenceOnly => "evidence_only",
        }
    }

    /// Returns the controlled display label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ExactRestore => "Exact restore",
            Self::CompatibleRestore => "Compatible restore",
            Self::LayoutOnly => "Layout only",
            Self::RecoveredDrafts => "Recovered drafts",
            Self::EvidenceOnly => "Evidence only",
        }
    }
}

/// Schema outcome shown when serialization or import changes meaning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceSchemaOutcome {
    /// Source and target meaning matched.
    Exact,
    /// Meaning was preserved through a declared compatibility path.
    Compatible,
    /// Only layout/context meaning survived.
    LayoutOnly,
    /// Human review is required before apply or restore.
    ManualReview,
}

impl WorkspaceSchemaOutcome {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::LayoutOnly => "layout_only",
            Self::ManualReview => "manual_review",
        }
    }
}

/// Dependency or policy condition that forces a placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MissingSurfaceDependency {
    /// Required extension or feature pack is missing.
    MissingExtension,
    /// Required remote target is unavailable.
    MissingRemote,
    /// Required provider connection is unavailable.
    MissingProvider,
    /// Permission or authority was revoked or expired.
    RevokedPermission,
    /// Workspace authority checkpoint is missing.
    MissingWorkspaceAuthority,
    /// Live surface cannot safely resume without explicit action.
    NonReentrantLiveSurface,
    /// Display topology changed materially.
    DisplayTopologyMismatch,
    /// Schema equivalence map is missing or refused.
    SchemaEquivalenceMissing,
}

impl MissingSurfaceDependency {
    /// Returns true when live authority must not be implied.
    pub const fn requires_manual_rebind(self) -> bool {
        matches!(
            self,
            Self::MissingRemote
                | Self::MissingProvider
                | Self::RevokedPermission
                | Self::NonReentrantLiveSurface
        )
    }
}

/// Reviewed action offered from an inspector row, sheet, or placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceReviewAction {
    /// Inspect without mutation.
    Inspect,
    /// Open export review.
    Export,
    /// Compare against a package, checkpoint, or prior artifact.
    Compare,
    /// Clear remembered-state metadata only.
    Clear,
    /// Confirm reviewed export.
    ConfirmExport,
    /// Cancel export without mutation.
    CancelExport,
    /// Confirm reviewed import.
    ConfirmImport,
    /// Cancel import without mutation.
    CancelImport,
    /// Open provenance or package details.
    OpenDetails,
    /// Install or enable a missing extension.
    InstallExtension,
    /// Reconnect a missing remote target.
    Reconnect,
    /// Reauthenticate a missing authority.
    Reauthenticate,
    /// Open without the missing dependency.
    OpenWithout,
    /// Export retained evidence.
    ExportEvidence,
    /// Remove a placeholder pane deliberately.
    RemovePane,
    /// Rerun only after explicit user action.
    RerunExplicitly,
    /// Escalate to manual review.
    ManualReview,
}

impl WorkspaceReviewAction {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::Export => "export",
            Self::Compare => "compare",
            Self::Clear => "clear",
            Self::ConfirmExport => "confirm_export",
            Self::CancelExport => "cancel_export",
            Self::ConfirmImport => "confirm_import",
            Self::CancelImport => "cancel_import",
            Self::OpenDetails => "open_details",
            Self::InstallExtension => "install_extension",
            Self::Reconnect => "reconnect",
            Self::Reauthenticate => "reauthenticate",
            Self::OpenWithout => "open_without",
            Self::ExportEvidence => "export_evidence",
            Self::RemovePane => "remove_pane",
            Self::RerunExplicitly => "rerun_explicitly",
            Self::ManualReview => "manual_review",
        }
    }
}

/// Effect boundary for a reviewed action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionEffectScope {
    /// Read-only inspection or compare.
    ReadOnly,
    /// Builds a portable export after review.
    BuildsExport,
    /// Applies an imported package after review.
    AppliesImport,
    /// Clears only selected remembered-state metadata.
    ClearsRememberedStateOnly,
    /// Cancels or closes without mutation.
    NoMutation,
    /// Rebinds a missing dependency after explicit user action.
    RebindsDependency,
}

/// One reviewed action row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceActionRecord {
    /// Action identifier.
    pub action: WorkspaceReviewAction,
    /// Whether the action is currently enabled.
    pub enabled: bool,
    /// Target ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
    /// Typed or redaction-aware disabled reason.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    /// Effect boundary for the action.
    pub effect_scope: ActionEffectScope,
}

/// Schema binding and artifact refs for one state layer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSchemaBinding {
    /// Schema path or URI for this layer.
    pub schema_ref: String,
    /// Integer schema version for this layer.
    pub schema_version: u32,
    /// Opaque artifact refs carried or referenced by this layer.
    pub artifact_refs: Vec<String>,
}

/// Placeholder card that preserves a pane slot when hydration fails.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MissingSurfacePlaceholder {
    /// Stable placeholder card id.
    pub placeholder_id: String,
    /// Stable pane id preserved from the topology.
    pub preserved_pane_id: String,
    /// Dependency or policy condition that forced the placeholder.
    pub dependency: MissingSurfaceDependency,
    /// Original pane role such as editor, terminal, preview, or notebook.
    pub original_role: String,
    /// Original surface class such as text_editor or terminal_view.
    pub original_surface_class: String,
    /// Last known redaction-safe provenance label.
    pub last_known_provenance_label: String,
    /// Safe actions offered by the placeholder.
    pub safe_actions: Vec<WorkspaceReviewAction>,
    /// Opaque evidence ref retained behind the placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
    /// Redaction-aware notes.
    pub notes: String,
}

/// Pane-bound serialized restore state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerializedPaneState {
    /// Stable pane id from the pane-tree schema.
    pub stable_pane_id: String,
    /// Surface role label such as editor, terminal, preview, or notebook.
    pub surface_role: String,
    /// Surface class label such as text_editor or terminal_view.
    pub surface_class: String,
    /// Pane-level restore fidelity.
    pub restore_fidelity: WorkspaceRestoreFidelity,
    /// Whether retained evidence exists for the pane.
    pub evidence_retained: bool,
    /// Whether live rerun or authority reacquire requires explicit user action.
    pub no_rerun_required: bool,
    /// Placeholder card when the pane cannot hydrate live.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<MissingSurfacePlaceholder>,
    /// Last write time for the pane state.
    pub last_written_at: String,
}

impl SerializedPaneState {
    /// Validates placeholder honesty for this pane.
    pub fn validate(&self) -> Result<(), WorkspaceSerializationBetaError> {
        require_non_empty("pane.stable_pane_id", &self.stable_pane_id)?;
        require_non_empty("pane.surface_role", &self.surface_role)?;
        require_non_empty("pane.surface_class", &self.surface_class)?;
        require_non_empty("pane.last_written_at", &self.last_written_at)?;

        if let Some(placeholder) = &self.placeholder {
            placeholder.validate_for_pane(self)?;
            if self.restore_fidelity == WorkspaceRestoreFidelity::ExactRestore {
                return Err(WorkspaceSerializationBetaError::ExactPaneHasPlaceholder {
                    pane_id: self.stable_pane_id.clone(),
                });
            }
        }

        if matches!(
            self.restore_fidelity,
            WorkspaceRestoreFidelity::LayoutOnly | WorkspaceRestoreFidelity::EvidenceOnly
        ) && !self.no_rerun_required
        {
            return Err(WorkspaceSerializationBetaError::PaneMissingNoRerunTruth {
                pane_id: self.stable_pane_id.clone(),
            });
        }
        Ok(())
    }
}

impl MissingSurfacePlaceholder {
    /// Validates the placeholder against the pane slot it preserves.
    pub fn validate_for_pane(
        &self,
        pane: &SerializedPaneState,
    ) -> Result<(), WorkspaceSerializationBetaError> {
        require_non_empty("placeholder.placeholder_id", &self.placeholder_id)?;
        require_non_empty("placeholder.preserved_pane_id", &self.preserved_pane_id)?;
        require_non_empty("placeholder.original_role", &self.original_role)?;
        require_non_empty(
            "placeholder.original_surface_class",
            &self.original_surface_class,
        )?;
        require_non_empty(
            "placeholder.last_known_provenance_label",
            &self.last_known_provenance_label,
        )?;
        require_non_empty("placeholder.notes", &self.notes)?;

        if self.preserved_pane_id != pane.stable_pane_id {
            return Err(WorkspaceSerializationBetaError::PlaceholderPaneIdMismatch {
                placeholder_id: self.placeholder_id.clone(),
                pane_id: pane.stable_pane_id.clone(),
            });
        }
        if self.original_role != pane.surface_role
            || self.original_surface_class != pane.surface_class
        {
            return Err(
                WorkspaceSerializationBetaError::PlaceholderLostSurfaceAttribution {
                    placeholder_id: self.placeholder_id.clone(),
                },
            );
        }
        if self.safe_actions.is_empty() {
            return Err(WorkspaceSerializationBetaError::PlaceholderMissingAction {
                placeholder_id: self.placeholder_id.clone(),
            });
        }
        if self.dependency.requires_manual_rebind() && !pane.no_rerun_required {
            return Err(WorkspaceSerializationBetaError::PaneMissingNoRerunTruth {
                pane_id: pane.stable_pane_id.clone(),
            });
        }
        Ok(())
    }
}

/// One separated state layer inside a package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceStateLayerRecord {
    /// Stable layer row id.
    pub layer_id: String,
    /// Layer kind.
    pub layer: WorkspaceStateLayer,
    /// Persistence class.
    pub persistence: WorkspacePersistenceClass,
    /// Export mode for this layer.
    pub export_mode: WorkspaceExportMode,
    /// Schema binding and artifact refs.
    pub schema_binding: WorkspaceSchemaBinding,
    /// Last write time for this layer.
    pub last_written_at: String,
    /// Schema compatibility outcome for this layer.
    pub schema_outcome: WorkspaceSchemaOutcome,
    /// Restore fidelity for this layer.
    pub restore_fidelity: WorkspaceRestoreFidelity,
    /// Whether export can include this layer.
    pub export_allowed: bool,
    /// Whether clearing remembered state for this layer is allowed.
    pub clear_allowed: bool,
    /// Pane-level state for window-topology or local-session layers.
    #[serde(default)]
    pub pane_states: Vec<SerializedPaneState>,
    /// Local-only or machine-local explanation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_reason: Option<String>,
    /// Redaction-aware notes.
    pub notes: String,
}

impl WorkspaceStateLayerRecord {
    /// Validates one state layer row.
    pub fn validate(&self) -> Result<(), WorkspaceSerializationBetaError> {
        require_non_empty("layer.layer_id", &self.layer_id)?;
        require_non_empty("layer.schema_ref", &self.schema_binding.schema_ref)?;
        require_non_empty("layer.last_written_at", &self.last_written_at)?;
        require_non_empty("layer.notes", &self.notes)?;

        if self.schema_binding.schema_version == 0 {
            return Err(WorkspaceSerializationBetaError::ZeroSchemaVersion {
                layer_id: self.layer_id.clone(),
            });
        }

        if self.layer == WorkspaceStateLayer::MachineLocalHints {
            if self.persistence != WorkspacePersistenceClass::MachineLocal {
                return Err(
                    WorkspaceSerializationBetaError::MachineLocalHintsMisclassified {
                        layer_id: self.layer_id.clone(),
                    },
                );
            }
            if self.export_allowed || self.export_mode == WorkspaceExportMode::CarriedBody {
                return Err(WorkspaceSerializationBetaError::MachineLocalLayerExported {
                    layer_id: self.layer_id.clone(),
                });
            }
        }

        if self.layer == WorkspaceStateLayer::WindowTopology {
            if self.schema_binding.schema_ref != WORKSPACE_PANE_TREE_SCHEMA_REF {
                return Err(
                    WorkspaceSerializationBetaError::WindowTopologySchemaMismatch {
                        layer_id: self.layer_id.clone(),
                    },
                );
            }
            if self.pane_states.is_empty() {
                return Err(
                    WorkspaceSerializationBetaError::WindowTopologyMissingPanes {
                        layer_id: self.layer_id.clone(),
                    },
                );
            }
        }

        if matches!(
            self.persistence,
            WorkspacePersistenceClass::LocalOnly | WorkspacePersistenceClass::MachineLocal
        ) && self
            .local_only_reason
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        {
            return Err(WorkspaceSerializationBetaError::LocalOnlyReasonMissing {
                layer_id: self.layer_id.clone(),
            });
        }

        let mut pane_ids = BTreeSet::new();
        for pane in &self.pane_states {
            if !pane_ids.insert(pane.stable_pane_id.as_str()) {
                return Err(WorkspaceSerializationBetaError::DuplicatePaneId {
                    pane_id: pane.stable_pane_id.clone(),
                });
            }
            pane.validate()?;
        }

        Ok(())
    }
}

/// Reason a portable-state package intentionally excludes a state object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableStateExclusionReason {
    /// Raw secret material is excluded.
    SecretMaterial,
    /// Delegated approvals are excluded.
    DelegatedApproval,
    /// Approval tickets are excluded.
    ApprovalTicket,
    /// Delegated credentials are excluded.
    DelegatedCredential,
    /// Live authority handles are excluded.
    LiveAuthorityHandle,
    /// Machine-unique trust anchors are excluded.
    MachineUniqueTrustAnchor,
    /// Credential-store-only material is excluded.
    CredentialStoreOnly,
    /// Concrete state roots are excluded.
    StateRootOnly,
    /// Raw local absolute paths are excluded.
    LocalAbsolutePath,
    /// Raw hostnames are excluded.
    RawHostname,
    /// Raw command lines are excluded.
    RawCommandLine,
    /// Raw logs are excluded.
    RawLog,
    /// Raw source content is excluded.
    RawSourceContent,
    /// Raw provider payload bodies are excluded.
    ProviderPayload,
    /// Best-effort display hints are reduced or excluded.
    DisplayHintBestEffortOnly,
    /// Policy excludes this object from export.
    PolicyExcludesExport,
}

/// Substitute retained when a state object is excluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExclusionSubstitute {
    /// Opaque ref is retained.
    OpaqueRef,
    /// Redacted summary is retained.
    RedactedSummary,
    /// Safe placeholder is retained.
    SafePlaceholder,
    /// Metadata only is retained.
    MetadataOnly,
    /// Nothing is retained.
    Omitted,
}

/// One explicit exclusion row named by export and restore summaries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateExclusion {
    /// Stable exclusion id.
    pub exclusion_id: String,
    /// Affected state layer.
    pub layer: WorkspaceStateLayer,
    /// Opaque artifact ref for the excluded object.
    pub artifact_ref: String,
    /// Exclusion reason.
    pub reason: PortableStateExclusionReason,
    /// Substitute retained in the package.
    pub substitute: ExclusionSubstitute,
    /// Whether the export sheet names this exclusion.
    pub named_in_export_summary: bool,
    /// Whether restore/provenance summaries name this exclusion.
    pub named_in_restore_summary: bool,
    /// Redaction-aware notes.
    pub notes: String,
}

impl PortableStateExclusion {
    /// Validates that this exclusion is visible on both package boundaries.
    pub fn validate(&self) -> Result<(), WorkspaceSerializationBetaError> {
        require_non_empty("exclusion.exclusion_id", &self.exclusion_id)?;
        require_non_empty("exclusion.artifact_ref", &self.artifact_ref)?;
        require_non_empty("exclusion.notes", &self.notes)?;
        if !self.named_in_export_summary || !self.named_in_restore_summary {
            return Err(WorkspaceSerializationBetaError::ExclusionNotNamed {
                exclusion_id: self.exclusion_id.clone(),
            });
        }
        Ok(())
    }
}

/// Redaction manifest for a workspace portable-state package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRedactionManifest {
    /// Stable redaction manifest id.
    pub manifest_id: String,
    /// Whether path redaction is available before export.
    pub path_redaction_available: bool,
    /// Whether host redaction is available before export.
    pub host_redaction_available: bool,
    /// Whether machine-local and high-risk exclusions were reviewed.
    pub exclusions_reviewed: bool,
    /// Applied exclusion reason classes.
    pub applied_rules: Vec<PortableStateExclusionReason>,
    /// Redaction-aware notes.
    pub notes: String,
}

impl WorkspaceRedactionManifest {
    /// Validates redaction availability and review truth.
    pub fn validate(&self) -> Result<(), WorkspaceSerializationBetaError> {
        require_non_empty("redaction.manifest_id", &self.manifest_id)?;
        require_non_empty("redaction.notes", &self.notes)?;
        if !self.exclusions_reviewed {
            return Err(WorkspaceSerializationBetaError::ExclusionsNotReviewed);
        }
        let rules = self.applied_rules.iter().copied().collect::<BTreeSet<_>>();
        for required in REQUIRED_EXCLUSION_REASONS {
            if !rules.contains(&required) {
                return Err(WorkspaceSerializationBetaError::MissingExclusionReason {
                    reason: required,
                });
            }
        }
        Ok(())
    }
}

/// Source event that produced a restore provenance card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreSourceEvent {
    /// Restore came from an automatic checkpoint.
    AutoCheckpoint,
    /// Restore came from a manual export.
    ManualExport,
    /// Restore came from a backup artifact.
    Backup,
    /// Restore came from sync.
    Sync,
    /// Restore came from import.
    Import,
}

/// Restore provenance card shown after import or restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceRestoreProvenanceCard {
    /// Stable card id.
    pub card_id: String,
    /// Source event family.
    pub source_event: RestoreSourceEvent,
    /// Opaque source artifact ref.
    pub source_artifact_ref: String,
    /// Producer ref or version label.
    pub producer_ref: String,
    /// Schema outcome for the restore/import event.
    pub schema_outcome: WorkspaceSchemaOutcome,
    /// Resulting restore fidelity.
    pub resulting_fidelity: WorkspaceRestoreFidelity,
    /// Opaque diagnostics surface ref where this card remains visible.
    pub diagnostics_ref: String,
    /// Opaque support-export ref where this card remains visible.
    pub support_export_ref: String,
    /// Opaque crash-recovery ref where this card remains visible.
    pub crash_recovery_ref: String,
    /// Top-level compare ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_ref: Option<String>,
    /// Top-level export ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub export_ref: Option<String>,
    /// Placeholder cards created or expected by this restore.
    #[serde(default)]
    pub missing_surface_placeholders: Vec<MissingSurfacePlaceholder>,
    /// Exclusion ids intentionally omitted by the package.
    #[serde(default)]
    pub intentional_exclusion_refs: Vec<String>,
    /// Redaction-aware notes.
    pub notes: String,
}

impl WorkspaceRestoreProvenanceCard {
    /// Builds a restore-provenance card that names every alpha package exclusion.
    pub fn for_alpha_package(
        alpha: &PortableStateAlphaPackage,
        card_id: impl Into<String>,
        source_event: RestoreSourceEvent,
        resulting_fidelity: WorkspaceRestoreFidelity,
        diagnostics_ref: impl Into<String>,
        support_export_ref: impl Into<String>,
        crash_recovery_ref: impl Into<String>,
    ) -> Self {
        let mut exclusion_rows = alpha
            .machine_local_exclusions
            .iter()
            .map(PortableStateExclusion::from_alpha_machine_exclusion)
            .collect::<Vec<_>>();
        exclusion_rows.extend(redaction_exclusions_from_alpha(alpha));
        sort_exclusions(&mut exclusion_rows);
        let mut exclusions = exclusion_rows
            .into_iter()
            .map(|row| row.exclusion_id)
            .collect::<Vec<_>>();
        sort_strings(&mut exclusions);

        let mut placeholders = alpha
            .state_classes
            .iter()
            .flat_map(|layer| layer.pane_restore_postures.iter())
            .filter_map(|pane| {
                pane.placeholder_card
                    .as_ref()
                    .map(|card| MissingSurfacePlaceholder::from_alpha_card(pane, card))
            })
            .collect::<Vec<_>>();
        placeholders.sort_by(|left, right| left.placeholder_id.cmp(&right.placeholder_id));
        placeholders.dedup_by(|left, right| left.placeholder_id == right.placeholder_id);

        Self {
            card_id: card_id.into(),
            source_event,
            source_artifact_ref: alpha.package_id.clone(),
            producer_ref: alpha.producer_ref.clone(),
            schema_outcome: WorkspaceSchemaOutcome::Compatible,
            resulting_fidelity,
            diagnostics_ref: diagnostics_ref.into(),
            support_export_ref: support_export_ref.into(),
            crash_recovery_ref: crash_recovery_ref.into(),
            compare_ref: Some(format!("compare:{}", alpha.package_id)),
            export_ref: Some(format!("export:{}", alpha.package_id)),
            missing_surface_placeholders: placeholders,
            intentional_exclusion_refs: exclusions,
            notes: "Restore provenance remains visible in diagnostics, support export, and crash recovery."
                .to_string(),
        }
    }

    /// Validates restore-provenance visibility and downgrade honesty.
    pub fn validate(&self) -> Result<(), WorkspaceSerializationBetaError> {
        require_non_empty("restore_card.card_id", &self.card_id)?;
        require_non_empty(
            "restore_card.source_artifact_ref",
            &self.source_artifact_ref,
        )?;
        require_non_empty("restore_card.producer_ref", &self.producer_ref)?;
        require_non_empty("restore_card.diagnostics_ref", &self.diagnostics_ref)?;
        require_non_empty("restore_card.support_export_ref", &self.support_export_ref)?;
        require_non_empty("restore_card.crash_recovery_ref", &self.crash_recovery_ref)?;
        require_non_empty("restore_card.notes", &self.notes)?;

        if self.resulting_fidelity == WorkspaceRestoreFidelity::ExactRestore
            && !self.missing_surface_placeholders.is_empty()
        {
            return Err(
                WorkspaceSerializationBetaError::ExactRestoreHasPlaceholder {
                    card_id: self.card_id.clone(),
                },
            );
        }

        if self.schema_outcome == WorkspaceSchemaOutcome::ManualReview {
            require_option("restore_card.compare_ref", &self.compare_ref)?;
            require_option("restore_card.export_ref", &self.export_ref)?;
        }

        let mut placeholders = BTreeSet::new();
        for placeholder in &self.missing_surface_placeholders {
            if !placeholders.insert(placeholder.placeholder_id.as_str()) {
                return Err(WorkspaceSerializationBetaError::DuplicatePlaceholder {
                    placeholder_id: placeholder.placeholder_id.clone(),
                });
            }
            require_non_empty("restore_card.placeholder_id", &placeholder.placeholder_id)?;
            if placeholder.safe_actions.is_empty() {
                return Err(WorkspaceSerializationBetaError::PlaceholderMissingAction {
                    placeholder_id: placeholder.placeholder_id.clone(),
                });
            }
        }
        Ok(())
    }
}

/// Versioned workspace portable-state package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspacePortableStatePackage {
    /// Record discriminator.
    pub record_kind: WorkspaceSerializationRecordKind,
    /// Schema version.
    pub schema_version: u32,
    /// Stable package id.
    pub package_id: String,
    /// Workspace ref this package describes.
    pub workspace_ref: String,
    /// Package creation time.
    pub created_at: String,
    /// Producer build or instance ref.
    pub producer_ref: String,
    /// Separated state layer rows.
    pub state_layers: Vec<WorkspaceStateLayerRecord>,
    /// Redaction manifest.
    pub redaction_manifest: WorkspaceRedactionManifest,
    /// Explicit exclusions named by export and restore summaries.
    pub exclusions: Vec<PortableStateExclusion>,
    /// Restore provenance card for this package.
    pub restore_provenance_card: WorkspaceRestoreProvenanceCard,
    /// Package-level actions.
    pub actions: Vec<WorkspaceActionRecord>,
    /// Redaction-aware notes.
    pub notes: String,
}

impl WorkspacePortableStatePackage {
    /// Converts an existing alpha package into the beta workspace boundary.
    pub fn from_alpha_package(
        alpha: &PortableStateAlphaPackage,
        restore_provenance_card: WorkspaceRestoreProvenanceCard,
    ) -> Result<Self, WorkspaceSerializationBetaError> {
        alpha
            .validate()
            .map_err(|err| WorkspaceSerializationBetaError::InvalidAlphaPackage(err.to_string()))?;

        let state_layers = alpha
            .state_classes
            .iter()
            .map(WorkspaceStateLayerRecord::from_alpha_layer)
            .collect::<Vec<_>>();
        let mut exclusions = alpha
            .machine_local_exclusions
            .iter()
            .map(PortableStateExclusion::from_alpha_machine_exclusion)
            .collect::<Vec<_>>();
        exclusions.extend(redaction_exclusions_from_alpha(alpha));
        sort_exclusions(&mut exclusions);

        let redaction_manifest = WorkspaceRedactionManifest {
            manifest_id: format!("workspace-redaction-manifest:{}", alpha.package_id),
            path_redaction_available: alpha
                .redaction_manifest
                .rules
                .contains(&RedactionRuleClass::RawPathExcluded),
            host_redaction_available: alpha
                .redaction_manifest
                .rules
                .contains(&RedactionRuleClass::RawHostExcluded),
            exclusions_reviewed: alpha.redaction_manifest.machine_local_exclusions_reviewed,
            applied_rules: unique_exclusion_reasons(&exclusions),
            notes: alpha.redaction_manifest.notes.clone(),
        };

        let package = Self {
            record_kind: WorkspaceSerializationRecordKind::WorkspacePortableStatePackageRecord,
            schema_version: WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION,
            package_id: alpha.package_id.clone(),
            workspace_ref: alpha.workspace_ref.clone(),
            created_at: alpha.created_at.clone(),
            producer_ref: alpha.producer_ref.clone(),
            state_layers,
            redaction_manifest,
            exclusions,
            restore_provenance_card,
            actions: default_workspace_actions(&alpha.workspace_ref),
            notes: "Workspace portable-state beta package derived from separated alpha rows."
                .to_string(),
        };
        package.validate()?;
        Ok(package)
    }

    /// Validates package separation, redaction, placeholder, and provenance invariants.
    pub fn validate(&self) -> Result<(), WorkspaceSerializationBetaError> {
        if self.record_kind != WorkspaceSerializationRecordKind::WorkspacePortableStatePackageRecord
        {
            return Err(WorkspaceSerializationBetaError::WrongRecordKind);
        }
        if self.schema_version != WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION {
            return Err(WorkspaceSerializationBetaError::WrongSchemaVersion {
                expected: WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        require_non_empty("package.package_id", &self.package_id)?;
        require_non_empty("package.workspace_ref", &self.workspace_ref)?;
        require_non_empty("package.created_at", &self.created_at)?;
        require_non_empty("package.producer_ref", &self.producer_ref)?;
        require_non_empty("package.notes", &self.notes)?;

        let mut layers = BTreeSet::new();
        for layer in &self.state_layers {
            if !layers.insert(layer.layer) {
                return Err(WorkspaceSerializationBetaError::DuplicateStateLayer {
                    layer: layer.layer,
                });
            }
            layer.validate()?;
        }
        for required in REQUIRED_LAYERS {
            if !layers.contains(&required) {
                return Err(WorkspaceSerializationBetaError::MissingStateLayer { layer: required });
            }
        }

        self.redaction_manifest.validate()?;
        self.validate_exclusions()?;
        self.validate_restore_card_links()?;
        self.validate_actions()?;
        Ok(())
    }

    /// Builds a remembered-state inspection model from this package.
    pub fn inspection(&self) -> Result<RememberedStateInspection, WorkspaceSerializationBetaError> {
        self.validate()?;
        Ok(RememberedStateInspection::from_package(self))
    }

    /// Builds an export review sheet from this package.
    pub fn export_review_sheet(
        &self,
        review_id: impl Into<String>,
    ) -> Result<PortableStateReviewSheet, WorkspaceSerializationBetaError> {
        self.validate()?;
        PortableStateReviewSheet::from_package(review_id, ReviewSheetPurpose::Export, self)
    }

    /// Builds an import review sheet from this package.
    pub fn import_review_sheet(
        &self,
        review_id: impl Into<String>,
    ) -> Result<PortableStateReviewSheet, WorkspaceSerializationBetaError> {
        self.validate()?;
        PortableStateReviewSheet::from_package(review_id, ReviewSheetPurpose::Import, self)
    }

    fn validate_exclusions(&self) -> Result<(), WorkspaceSerializationBetaError> {
        let mut exclusion_ids = BTreeSet::new();
        let mut reasons = BTreeSet::new();
        for exclusion in &self.exclusions {
            if !exclusion_ids.insert(exclusion.exclusion_id.as_str()) {
                return Err(WorkspaceSerializationBetaError::DuplicateExclusion {
                    exclusion_id: exclusion.exclusion_id.clone(),
                });
            }
            reasons.insert(exclusion.reason);
            exclusion.validate()?;
        }
        for required in REQUIRED_EXCLUSION_REASONS {
            if !reasons.contains(&required) {
                return Err(WorkspaceSerializationBetaError::MissingExclusionReason {
                    reason: required,
                });
            }
        }
        Ok(())
    }

    fn validate_restore_card_links(&self) -> Result<(), WorkspaceSerializationBetaError> {
        self.restore_provenance_card.validate()?;
        let exclusions = self
            .exclusions
            .iter()
            .map(|row| row.exclusion_id.as_str())
            .collect::<BTreeSet<_>>();
        for exclusion in &self.exclusions {
            if !self
                .restore_provenance_card
                .intentional_exclusion_refs
                .iter()
                .any(|row| row == &exclusion.exclusion_id)
            {
                return Err(
                    WorkspaceSerializationBetaError::RestoreCardMissingExclusion {
                        exclusion_id: exclusion.exclusion_id.clone(),
                    },
                );
            }
        }
        for exclusion_ref in &self.restore_provenance_card.intentional_exclusion_refs {
            if !exclusions.contains(exclusion_ref.as_str()) {
                return Err(
                    WorkspaceSerializationBetaError::RestoreCardUnknownExclusion {
                        exclusion_id: exclusion_ref.clone(),
                    },
                );
            }
        }
        Ok(())
    }

    fn validate_actions(&self) -> Result<(), WorkspaceSerializationBetaError> {
        let actions = self
            .actions
            .iter()
            .map(|action| action.action)
            .collect::<BTreeSet<_>>();
        for required in [
            WorkspaceReviewAction::Inspect,
            WorkspaceReviewAction::Export,
            WorkspaceReviewAction::Compare,
            WorkspaceReviewAction::Clear,
        ] {
            if !actions.contains(&required) {
                return Err(WorkspaceSerializationBetaError::MissingAction { action: required });
            }
        }
        for action in &self.actions {
            if action.action == WorkspaceReviewAction::Clear
                && action.effect_scope != ActionEffectScope::ClearsRememberedStateOnly
            {
                return Err(WorkspaceSerializationBetaError::ClearActionTooBroad);
            }
        }
        Ok(())
    }
}

/// Remembered-state inspection projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedStateInspection {
    /// Record discriminator.
    pub record_kind: WorkspaceSerializationRecordKind,
    /// Schema version.
    pub schema_version: u32,
    /// Package id being inspected.
    pub package_id: String,
    /// Inspection rows.
    pub rows: Vec<RememberedStateInspectionRow>,
    /// Top-level inspector actions.
    pub actions: Vec<WorkspaceActionRecord>,
}

impl RememberedStateInspection {
    fn from_package(package: &WorkspacePortableStatePackage) -> Self {
        let mut rows = Vec::new();
        for layer in &package.state_layers {
            rows.push(RememberedStateInspectionRow::from_layer(layer));
            for pane in &layer.pane_states {
                rows.push(RememberedStateInspectionRow::from_pane(layer, pane));
            }
        }
        Self {
            record_kind: WorkspaceSerializationRecordKind::RememberedStateInspectionRecord,
            schema_version: WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION,
            package_id: package.package_id.clone(),
            rows,
            actions: package.actions.clone(),
        }
    }

    /// Renders a support-safe plaintext view of remembered state.
    pub fn render_plaintext(&self) -> String {
        let mut lines = vec![format!(
            "Remembered State Inspection package={} schema_version={}",
            self.package_id, self.schema_version
        )];
        for row in &self.rows {
            lines.push(format!(
                "- {} {} persistence={} schema={} schema_outcome={} fidelity={} actions={}",
                row.layer.as_str(),
                row.artifact_ref,
                row.persistence.as_str(),
                row.schema_version,
                row.schema_outcome.as_str(),
                row.restore_fidelity.display_label(),
                row.actions
                    .iter()
                    .map(|action| action.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
            if let Some(pane_id) = &row.stable_pane_id {
                lines.push(format!("  pane={pane_id}"));
            }
            if let Some(reason) = &row.local_only_reason {
                lines.push(format!("  reason={reason}"));
            }
        }
        lines.join("\n")
    }
}

/// One row in the remembered-state inspection projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedStateInspectionRow {
    /// Artifact, layer, or pane ref.
    pub artifact_ref: String,
    /// State layer.
    pub layer: WorkspaceStateLayer,
    /// Persistence class.
    pub persistence: WorkspacePersistenceClass,
    /// Schema ref.
    pub schema_ref: String,
    /// Schema version.
    pub schema_version: u32,
    /// Last write time.
    pub last_written_at: String,
    /// Schema compatibility outcome.
    pub schema_outcome: WorkspaceSchemaOutcome,
    /// Restore fidelity.
    pub restore_fidelity: WorkspaceRestoreFidelity,
    /// Stable pane id when pane-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_pane_id: Option<String>,
    /// Local-only explanation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_reason: Option<String>,
    /// Available actions.
    pub actions: Vec<WorkspaceReviewAction>,
}

impl RememberedStateInspectionRow {
    fn from_layer(layer: &WorkspaceStateLayerRecord) -> Self {
        Self {
            artifact_ref: layer.layer_id.clone(),
            layer: layer.layer,
            persistence: layer.persistence,
            schema_ref: layer.schema_binding.schema_ref.clone(),
            schema_version: layer.schema_binding.schema_version,
            last_written_at: layer.last_written_at.clone(),
            schema_outcome: layer.schema_outcome,
            restore_fidelity: layer.restore_fidelity,
            stable_pane_id: None,
            local_only_reason: layer.local_only_reason.clone(),
            actions: actions_for_layer(layer),
        }
    }

    fn from_pane(layer: &WorkspaceStateLayerRecord, pane: &SerializedPaneState) -> Self {
        let mut actions = vec![
            WorkspaceReviewAction::Inspect,
            WorkspaceReviewAction::Compare,
        ];
        if layer.export_allowed {
            actions.push(WorkspaceReviewAction::Export);
        }
        if layer.clear_allowed {
            actions.push(WorkspaceReviewAction::Clear);
        }
        Self {
            artifact_ref: pane.stable_pane_id.clone(),
            layer: layer.layer,
            persistence: layer.persistence,
            schema_ref: layer.schema_binding.schema_ref.clone(),
            schema_version: layer.schema_binding.schema_version,
            last_written_at: pane.last_written_at.clone(),
            schema_outcome: layer.schema_outcome,
            restore_fidelity: pane.restore_fidelity,
            stable_pane_id: Some(pane.stable_pane_id.clone()),
            local_only_reason: layer.local_only_reason.clone(),
            actions,
        }
    }
}

/// Purpose of a portable-state review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSheetPurpose {
    /// Export review before bytes leave the boundary.
    Export,
    /// Import review before state is applied or inspected.
    Import,
}

/// Integrity checksum state for a review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecksumState {
    /// Checksum is available.
    ChecksumAvailable,
    /// Checksum is pending.
    ChecksumPending,
    /// Checksum is unavailable during preflight.
    UnavailablePreflight,
}

/// Signature state for a review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignatureState {
    /// Signature is verified.
    SignedVerified,
    /// Signature is not verified.
    SignedUnverified,
    /// Package is unsigned.
    Unsigned,
    /// Signature is pending.
    Pending,
}

/// Export or import review sheet for a workspace portable-state package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateReviewSheet {
    /// Record discriminator.
    pub record_kind: WorkspaceSerializationRecordKind,
    /// Schema version.
    pub schema_version: u32,
    /// Stable review id.
    pub review_id: String,
    /// Reviewed package id.
    pub package_id: String,
    /// Review purpose.
    pub purpose: ReviewSheetPurpose,
    /// Selected state layer ids.
    pub selected_layer_ids: Vec<String>,
    /// Machine-local and high-risk exclusions visible on the sheet.
    pub exclusions: Vec<PortableStateExclusion>,
    /// Restore-provenance card ref.
    pub restore_provenance_card_ref: String,
    /// Whether path redaction is enabled for this review.
    pub path_redaction_enabled: bool,
    /// Whether host redaction is enabled for this review.
    pub host_redaction_enabled: bool,
    /// Checksum state.
    pub checksum_state: ChecksumState,
    /// Signature state.
    pub signature_state: SignatureState,
    /// Review actions.
    pub actions: Vec<WorkspaceActionRecord>,
    /// Redaction-aware notes.
    pub notes: String,
}

impl PortableStateReviewSheet {
    fn from_package(
        review_id: impl Into<String>,
        purpose: ReviewSheetPurpose,
        package: &WorkspacePortableStatePackage,
    ) -> Result<Self, WorkspaceSerializationBetaError> {
        let mut actions = vec![
            WorkspaceActionRecord {
                action: WorkspaceReviewAction::OpenDetails,
                enabled: true,
                target_ref: Some(package.restore_provenance_card.card_id.clone()),
                disabled_reason: None,
                effect_scope: ActionEffectScope::ReadOnly,
            },
            WorkspaceActionRecord {
                action: WorkspaceReviewAction::Compare,
                enabled: true,
                target_ref: package.restore_provenance_card.compare_ref.clone(),
                disabled_reason: None,
                effect_scope: ActionEffectScope::ReadOnly,
            },
        ];
        match purpose {
            ReviewSheetPurpose::Export => {
                actions.push(WorkspaceActionRecord {
                    action: WorkspaceReviewAction::ConfirmExport,
                    enabled: true,
                    target_ref: Some(format!("export:{}", package.package_id)),
                    disabled_reason: None,
                    effect_scope: ActionEffectScope::BuildsExport,
                });
                actions.push(WorkspaceActionRecord {
                    action: WorkspaceReviewAction::CancelExport,
                    enabled: true,
                    target_ref: None,
                    disabled_reason: None,
                    effect_scope: ActionEffectScope::NoMutation,
                });
            }
            ReviewSheetPurpose::Import => {
                actions.push(WorkspaceActionRecord {
                    action: WorkspaceReviewAction::ConfirmImport,
                    enabled: true,
                    target_ref: Some(format!("import:{}", package.package_id)),
                    disabled_reason: None,
                    effect_scope: ActionEffectScope::AppliesImport,
                });
                actions.push(WorkspaceActionRecord {
                    action: WorkspaceReviewAction::CancelImport,
                    enabled: true,
                    target_ref: None,
                    disabled_reason: None,
                    effect_scope: ActionEffectScope::NoMutation,
                });
            }
        }
        let sheet = Self {
            record_kind: WorkspaceSerializationRecordKind::PortableStateReviewSheetRecord,
            schema_version: WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION,
            review_id: review_id.into(),
            package_id: package.package_id.clone(),
            purpose,
            selected_layer_ids: package
                .state_layers
                .iter()
                .filter(|layer| layer.export_allowed)
                .map(|layer| layer.layer_id.clone())
                .collect(),
            exclusions: package.exclusions.clone(),
            restore_provenance_card_ref: package.restore_provenance_card.card_id.clone(),
            path_redaction_enabled: package.redaction_manifest.path_redaction_available,
            host_redaction_enabled: package.redaction_manifest.host_redaction_available,
            checksum_state: ChecksumState::ChecksumAvailable,
            signature_state: SignatureState::Unsigned,
            actions,
            notes: "Review sheet names every selected layer and every excluded live or machine-local class."
                .to_string(),
        };
        sheet.validate()?;
        Ok(sheet)
    }

    /// Validates review-sheet visibility and confirm/cancel controls.
    pub fn validate(&self) -> Result<(), WorkspaceSerializationBetaError> {
        if self.record_kind != WorkspaceSerializationRecordKind::PortableStateReviewSheetRecord {
            return Err(WorkspaceSerializationBetaError::WrongRecordKind);
        }
        if self.schema_version != WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION {
            return Err(WorkspaceSerializationBetaError::WrongSchemaVersion {
                expected: WORKSPACE_SERIALIZATION_BETA_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        require_non_empty("review.review_id", &self.review_id)?;
        require_non_empty("review.package_id", &self.package_id)?;
        require_non_empty(
            "review.restore_provenance_card_ref",
            &self.restore_provenance_card_ref,
        )?;
        require_non_empty("review.notes", &self.notes)?;
        if self.selected_layer_ids.is_empty() {
            return Err(WorkspaceSerializationBetaError::ReviewSheetMissingSelectedLayer);
        }
        if self.exclusions.is_empty() {
            return Err(WorkspaceSerializationBetaError::ReviewSheetMissingExclusions);
        }
        let actions = self
            .actions
            .iter()
            .map(|action| action.action)
            .collect::<BTreeSet<_>>();
        let required = match self.purpose {
            ReviewSheetPurpose::Export => [
                WorkspaceReviewAction::ConfirmExport,
                WorkspaceReviewAction::CancelExport,
            ],
            ReviewSheetPurpose::Import => [
                WorkspaceReviewAction::ConfirmImport,
                WorkspaceReviewAction::CancelImport,
            ],
        };
        for action in required {
            if !actions.contains(&action) {
                return Err(WorkspaceSerializationBetaError::MissingAction { action });
            }
        }
        Ok(())
    }
}

/// Validation errors for workspace serialization beta records.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorkspaceSerializationBetaError {
    /// Record kind does not match.
    WrongRecordKind,
    /// Schema version is unsupported.
    WrongSchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Required field is empty.
    MissingField {
        /// Field path.
        field: &'static str,
    },
    /// Existing alpha package failed validation.
    InvalidAlphaPackage(String),
    /// Required state layer is missing.
    MissingStateLayer {
        /// Missing layer.
        layer: WorkspaceStateLayer,
    },
    /// State layer appears more than once.
    DuplicateStateLayer {
        /// Duplicated layer.
        layer: WorkspaceStateLayer,
    },
    /// Schema version cannot be zero.
    ZeroSchemaVersion {
        /// Layer id.
        layer_id: String,
    },
    /// Machine-local hints are misclassified.
    MachineLocalHintsMisclassified {
        /// Layer id.
        layer_id: String,
    },
    /// Machine-local layer would export as authority.
    MachineLocalLayerExported {
        /// Layer id.
        layer_id: String,
    },
    /// Window-topology layer does not bind to pane-tree schema.
    WindowTopologySchemaMismatch {
        /// Layer id.
        layer_id: String,
    },
    /// Window-topology layer has no panes.
    WindowTopologyMissingPanes {
        /// Layer id.
        layer_id: String,
    },
    /// Local-only or machine-local layer lacks explanation.
    LocalOnlyReasonMissing {
        /// Layer id.
        layer_id: String,
    },
    /// Pane id appears more than once in a layer.
    DuplicatePaneId {
        /// Pane id.
        pane_id: String,
    },
    /// Exact pane carried a placeholder.
    ExactPaneHasPlaceholder {
        /// Pane id.
        pane_id: String,
    },
    /// Pane downgrade lacks no-rerun truth.
    PaneMissingNoRerunTruth {
        /// Pane id.
        pane_id: String,
    },
    /// Placeholder preserved a different pane id.
    PlaceholderPaneIdMismatch {
        /// Placeholder id.
        placeholder_id: String,
        /// Pane id.
        pane_id: String,
    },
    /// Placeholder lost role or surface-class attribution.
    PlaceholderLostSurfaceAttribution {
        /// Placeholder id.
        placeholder_id: String,
    },
    /// Placeholder has no action.
    PlaceholderMissingAction {
        /// Placeholder id.
        placeholder_id: String,
    },
    /// Redaction or exclusion review was not completed.
    ExclusionsNotReviewed,
    /// Required exclusion reason is missing.
    MissingExclusionReason {
        /// Missing reason.
        reason: PortableStateExclusionReason,
    },
    /// Exclusion is not named on export and restore boundaries.
    ExclusionNotNamed {
        /// Exclusion id.
        exclusion_id: String,
    },
    /// Exclusion id appears more than once.
    DuplicateExclusion {
        /// Exclusion id.
        exclusion_id: String,
    },
    /// Exact restore carried a placeholder.
    ExactRestoreHasPlaceholder {
        /// Restore card id.
        card_id: String,
    },
    /// Placeholder id appears more than once.
    DuplicatePlaceholder {
        /// Placeholder id.
        placeholder_id: String,
    },
    /// Restore card does not name an exclusion.
    RestoreCardMissingExclusion {
        /// Exclusion id.
        exclusion_id: String,
    },
    /// Restore card names an unknown exclusion.
    RestoreCardUnknownExclusion {
        /// Exclusion id.
        exclusion_id: String,
    },
    /// Required package or sheet action is missing.
    MissingAction {
        /// Missing action.
        action: WorkspaceReviewAction,
    },
    /// Clear action reaches beyond remembered-state metadata.
    ClearActionTooBroad,
    /// Review sheet has no selected layer.
    ReviewSheetMissingSelectedLayer,
    /// Review sheet has no visible exclusions.
    ReviewSheetMissingExclusions,
}

impl fmt::Display for WorkspaceSerializationBetaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind => write!(f, "wrong workspace serialization record kind"),
            Self::WrongSchemaVersion { expected, actual } => {
                write!(f, "expected schema version {expected}, got {actual}")
            }
            Self::MissingField { field } => write!(f, "missing required field {field}"),
            Self::InvalidAlphaPackage(err) => write!(f, "invalid alpha package: {err}"),
            Self::MissingStateLayer { layer } => {
                write!(f, "missing state layer {}", layer.as_str())
            }
            Self::DuplicateStateLayer { layer } => {
                write!(f, "duplicate state layer {}", layer.as_str())
            }
            Self::ZeroSchemaVersion { layer_id } => {
                write!(f, "state layer {layer_id} has schema version zero")
            }
            Self::MachineLocalHintsMisclassified { layer_id } => {
                write!(f, "machine-local hints {layer_id} are misclassified")
            }
            Self::MachineLocalLayerExported { layer_id } => {
                write!(
                    f,
                    "machine-local layer {layer_id} would export as authority"
                )
            }
            Self::WindowTopologySchemaMismatch { layer_id } => {
                write!(
                    f,
                    "window topology {layer_id} does not use pane-tree schema"
                )
            }
            Self::WindowTopologyMissingPanes { layer_id } => {
                write!(f, "window topology {layer_id} has no pane states")
            }
            Self::LocalOnlyReasonMissing { layer_id } => {
                write!(
                    f,
                    "local-only or machine-local layer {layer_id} lacks reason"
                )
            }
            Self::DuplicatePaneId { pane_id } => write!(f, "duplicate pane id {pane_id}"),
            Self::ExactPaneHasPlaceholder { pane_id } => {
                write!(f, "exact pane {pane_id} carried a placeholder")
            }
            Self::PaneMissingNoRerunTruth { pane_id } => {
                write!(f, "pane {pane_id} lacks no-rerun truth")
            }
            Self::PlaceholderPaneIdMismatch {
                placeholder_id,
                pane_id,
            } => write!(
                f,
                "placeholder {placeholder_id} does not preserve pane {pane_id}"
            ),
            Self::PlaceholderLostSurfaceAttribution { placeholder_id } => write!(
                f,
                "placeholder {placeholder_id} lost role or surface attribution"
            ),
            Self::PlaceholderMissingAction { placeholder_id } => {
                write!(f, "placeholder {placeholder_id} has no action")
            }
            Self::ExclusionsNotReviewed => write!(f, "exclusions were not reviewed"),
            Self::MissingExclusionReason { reason } => {
                write!(f, "missing exclusion reason {reason:?}")
            }
            Self::ExclusionNotNamed { exclusion_id } => {
                write!(f, "exclusion {exclusion_id} was not named in summaries")
            }
            Self::DuplicateExclusion { exclusion_id } => {
                write!(f, "duplicate exclusion {exclusion_id}")
            }
            Self::ExactRestoreHasPlaceholder { card_id } => {
                write!(f, "exact restore card {card_id} carried placeholders")
            }
            Self::DuplicatePlaceholder { placeholder_id } => {
                write!(f, "duplicate placeholder {placeholder_id}")
            }
            Self::RestoreCardMissingExclusion { exclusion_id } => {
                write!(f, "restore card did not name exclusion {exclusion_id}")
            }
            Self::RestoreCardUnknownExclusion { exclusion_id } => {
                write!(f, "restore card names unknown exclusion {exclusion_id}")
            }
            Self::MissingAction { action } => write!(f, "missing action {}", action.as_str()),
            Self::ClearActionTooBroad => write!(f, "clear action is broader than remembered state"),
            Self::ReviewSheetMissingSelectedLayer => {
                write!(f, "review sheet has no selected layer")
            }
            Self::ReviewSheetMissingExclusions => write!(f, "review sheet has no exclusions"),
        }
    }
}

impl std::error::Error for WorkspaceSerializationBetaError {}

impl WorkspaceStateLayerRecord {
    fn from_alpha_layer(layer: &crate::PortableStateClassRecord) -> Self {
        Self {
            layer_id: layer.class_id.clone(),
            layer: map_alpha_layer(layer.class_kind),
            persistence: map_alpha_persistence(layer.classification),
            export_mode: map_alpha_export_mode(layer.export_mode),
            schema_binding: WorkspaceSchemaBinding {
                schema_ref: layer.schema_binding.schema_ref.clone(),
                schema_version: layer.schema_binding.schema_version,
                artifact_refs: layer.schema_binding.artifact_refs.clone(),
            },
            last_written_at: layer.last_written_at.clone(),
            schema_outcome: map_alpha_schema_outcome(layer.restore_candidate),
            restore_fidelity: map_alpha_restore_candidate(layer.restore_candidate),
            export_allowed: layer.export_allowed,
            clear_allowed: layer.clear_allowed,
            pane_states: layer
                .pane_restore_postures
                .iter()
                .map(SerializedPaneState::from_alpha_pane)
                .collect(),
            local_only_reason: layer.local_only_reason.clone(),
            notes: layer.notes.clone(),
        }
    }
}

impl SerializedPaneState {
    fn from_alpha_pane(pane: &AlphaPaneRestorePosture) -> Self {
        Self {
            stable_pane_id: pane.stable_pane_id.clone(),
            surface_role: pane.surface_role.clone(),
            surface_class: pane.surface_class.clone(),
            restore_fidelity: map_alpha_pane_posture(pane.restore_posture),
            evidence_retained: pane
                .placeholder_card
                .as_ref()
                .map(|card| card.evidence_retained)
                .unwrap_or(false),
            no_rerun_required: !pane.no_rerun_guardrails.is_empty()
                || pane.restore_posture != SurfaceRestorePosture::Live,
            placeholder: pane
                .placeholder_card
                .as_ref()
                .map(|card| MissingSurfacePlaceholder::from_alpha_card(pane, card)),
            last_written_at: pane.last_written_at.clone(),
        }
    }
}

impl MissingSurfacePlaceholder {
    fn from_alpha_card(pane: &AlphaPaneRestorePosture, card: &crate::PlaceholderCard) -> Self {
        Self {
            placeholder_id: format!("placeholder:{}", pane.stable_pane_id),
            preserved_pane_id: pane.stable_pane_id.clone(),
            dependency: map_alpha_placeholder_reason(card.reason),
            original_role: pane.surface_role.clone(),
            original_surface_class: pane.surface_class.clone(),
            last_known_provenance_label: card
                .last_known_label
                .clone()
                .unwrap_or_else(|| pane.surface_role.clone()),
            safe_actions: card
                .safe_actions
                .iter()
                .map(|action| map_alpha_placeholder_action(*action))
                .collect(),
            evidence_ref: card
                .evidence_retained
                .then(|| format!("evidence:{}", pane.stable_pane_id)),
            notes: "Pane slot preserved as a missing-surface placeholder.".to_string(),
        }
    }
}

impl PortableStateExclusion {
    fn from_alpha_machine_exclusion(exclusion: &AlphaMachineLocalExclusion) -> Self {
        Self {
            exclusion_id: exclusion.exclusion_id.clone(),
            layer: map_alpha_layer(exclusion.class_kind),
            artifact_ref: exclusion.artifact_ref.clone(),
            reason: map_alpha_machine_exclusion_reason(exclusion.reason),
            substitute: map_alpha_substitute(exclusion.substitute_class),
            named_in_export_summary: true,
            named_in_restore_summary: true,
            notes: exclusion.notes.clone(),
        }
    }
}

const REQUIRED_LAYERS: [WorkspaceStateLayer; 4] = [
    WorkspaceStateLayer::WorkspaceAuthority,
    WorkspaceStateLayer::WindowTopology,
    WorkspaceStateLayer::ProfileDefaults,
    WorkspaceStateLayer::MachineLocalHints,
];

const REQUIRED_EXCLUSION_REASONS: [PortableStateExclusionReason; 4] = [
    PortableStateExclusionReason::SecretMaterial,
    PortableStateExclusionReason::DelegatedApproval,
    PortableStateExclusionReason::LiveAuthorityHandle,
    PortableStateExclusionReason::MachineUniqueTrustAnchor,
];

fn actions_for_layer(layer: &WorkspaceStateLayerRecord) -> Vec<WorkspaceReviewAction> {
    let mut actions = vec![WorkspaceReviewAction::Inspect];
    if layer.export_allowed {
        actions.push(WorkspaceReviewAction::Export);
    }
    if layer.restore_fidelity != WorkspaceRestoreFidelity::EvidenceOnly {
        actions.push(WorkspaceReviewAction::Compare);
    }
    if layer.clear_allowed {
        actions.push(WorkspaceReviewAction::Clear);
    }
    actions
}

fn default_workspace_actions(workspace_ref: &str) -> Vec<WorkspaceActionRecord> {
    vec![
        WorkspaceActionRecord {
            action: WorkspaceReviewAction::Inspect,
            enabled: true,
            target_ref: Some(format!("remembered-state-inspect:{workspace_ref}")),
            disabled_reason: None,
            effect_scope: ActionEffectScope::ReadOnly,
        },
        WorkspaceActionRecord {
            action: WorkspaceReviewAction::Export,
            enabled: true,
            target_ref: Some(format!("portable-state-export:{workspace_ref}")),
            disabled_reason: None,
            effect_scope: ActionEffectScope::BuildsExport,
        },
        WorkspaceActionRecord {
            action: WorkspaceReviewAction::Compare,
            enabled: true,
            target_ref: Some(format!("portable-state-compare:{workspace_ref}")),
            disabled_reason: None,
            effect_scope: ActionEffectScope::ReadOnly,
        },
        WorkspaceActionRecord {
            action: WorkspaceReviewAction::Clear,
            enabled: true,
            target_ref: Some(format!("remembered-state-clear:{workspace_ref}")),
            disabled_reason: None,
            effect_scope: ActionEffectScope::ClearsRememberedStateOnly,
        },
    ]
}

fn redaction_exclusions_from_alpha(
    alpha: &PortableStateAlphaPackage,
) -> Vec<PortableStateExclusion> {
    alpha
        .redaction_manifest
        .rules
        .iter()
        .filter_map(|rule| {
            let reason = match rule {
                RedactionRuleClass::RawSecretMaterialExcluded => {
                    PortableStateExclusionReason::SecretMaterial
                }
                RedactionRuleClass::ApprovalTicketExcluded => {
                    PortableStateExclusionReason::ApprovalTicket
                }
                RedactionRuleClass::DelegatedCredentialExcluded => {
                    PortableStateExclusionReason::DelegatedCredential
                }
                RedactionRuleClass::LiveAuthorityHandleExcluded => {
                    PortableStateExclusionReason::LiveAuthorityHandle
                }
                RedactionRuleClass::MachineUniqueHandleExcluded => {
                    PortableStateExclusionReason::MachineUniqueTrustAnchor
                }
                RedactionRuleClass::RawPathExcluded => {
                    PortableStateExclusionReason::LocalAbsolutePath
                }
                RedactionRuleClass::RawHostExcluded => PortableStateExclusionReason::RawHostname,
                RedactionRuleClass::RawCommandLineExcluded => {
                    PortableStateExclusionReason::RawCommandLine
                }
                RedactionRuleClass::RawLogExcluded => PortableStateExclusionReason::RawLog,
                RedactionRuleClass::RawSourceContentExcluded => {
                    PortableStateExclusionReason::RawSourceContent
                }
                RedactionRuleClass::ProviderPayloadExcluded => {
                    PortableStateExclusionReason::ProviderPayload
                }
                RedactionRuleClass::StateRootExcluded => {
                    PortableStateExclusionReason::StateRootOnly
                }
            };
            let layer = match reason {
                PortableStateExclusionReason::LocalAbsolutePath
                | PortableStateExclusionReason::RawHostname
                | PortableStateExclusionReason::RawCommandLine
                | PortableStateExclusionReason::RawLog
                | PortableStateExclusionReason::RawSourceContent
                | PortableStateExclusionReason::ProviderPayload => {
                    WorkspaceStateLayer::LocalSessionContext
                }
                PortableStateExclusionReason::MachineUniqueTrustAnchor
                | PortableStateExclusionReason::StateRootOnly => {
                    WorkspaceStateLayer::MachineLocalHints
                }
                _ => WorkspaceStateLayer::WorkspaceAuthority,
            };
            Some(PortableStateExclusion {
                exclusion_id: format!("redaction-exclusion:{:?}:{}", rule, alpha.package_id),
                layer,
                artifact_ref: format!("redaction-rule:{:?}:{}", rule, alpha.package_id),
                reason,
                substitute: ExclusionSubstitute::Omitted,
                named_in_export_summary: true,
                named_in_restore_summary: true,
                notes: "Redaction rule excludes this class from portable-state packages."
                    .to_string(),
            })
        })
        .collect()
}

fn sort_exclusions(exclusions: &mut Vec<PortableStateExclusion>) {
    exclusions.sort_by(|left, right| left.exclusion_id.cmp(&right.exclusion_id));
    exclusions.dedup_by(|left, right| left.exclusion_id == right.exclusion_id);

    let has_delegated_approval = exclusions
        .iter()
        .any(|row| row.reason == PortableStateExclusionReason::DelegatedApproval);
    if !has_delegated_approval {
        exclusions.push(PortableStateExclusion {
            exclusion_id: "redaction-exclusion:delegated-approval".to_string(),
            layer: WorkspaceStateLayer::WorkspaceAuthority,
            artifact_ref: "delegated-approval-material".to_string(),
            reason: PortableStateExclusionReason::DelegatedApproval,
            substitute: ExclusionSubstitute::Omitted,
            named_in_export_summary: true,
            named_in_restore_summary: true,
            notes: "Delegated approvals are intentionally excluded from portable packages."
                .to_string(),
        });
    }
}

fn sort_strings(values: &mut Vec<String>) {
    values.sort();
    values.dedup();
}

fn unique_exclusion_reasons(
    exclusions: &[PortableStateExclusion],
) -> Vec<PortableStateExclusionReason> {
    exclusions
        .iter()
        .map(|row| row.reason)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn map_alpha_layer(class: SerializedStateClass) -> WorkspaceStateLayer {
    match class {
        SerializedStateClass::WorkspaceAuthority => WorkspaceStateLayer::WorkspaceAuthority,
        SerializedStateClass::WindowTopology => WorkspaceStateLayer::WindowTopology,
        SerializedStateClass::ProfileDefaults => WorkspaceStateLayer::ProfileDefaults,
        SerializedStateClass::MachineLocalHints => WorkspaceStateLayer::MachineLocalHints,
        SerializedStateClass::LocalSessionContext => WorkspaceStateLayer::LocalSessionContext,
    }
}

fn map_alpha_persistence(classification: PersistenceClassification) -> WorkspacePersistenceClass {
    match classification {
        PersistenceClassification::LocalOnly => WorkspacePersistenceClass::LocalOnly,
        PersistenceClassification::Portable => WorkspacePersistenceClass::Portable,
        PersistenceClassification::Shared => WorkspacePersistenceClass::Shared,
        PersistenceClassification::MachineLocal => WorkspacePersistenceClass::MachineLocal,
    }
}

fn map_alpha_export_mode(mode: AlphaExportMode) -> WorkspaceExportMode {
    match mode {
        AlphaExportMode::CarriedBody => WorkspaceExportMode::CarriedBody,
        AlphaExportMode::ReferencedBody => WorkspaceExportMode::ReferencedBody,
        AlphaExportMode::LinkedArtifactRef => WorkspaceExportMode::LinkedArtifactRef,
        AlphaExportMode::MetadataOnly => WorkspaceExportMode::MetadataOnly,
        AlphaExportMode::Excluded => WorkspaceExportMode::Excluded,
    }
}

fn map_alpha_restore_candidate(candidate: RestoreCandidateClass) -> WorkspaceRestoreFidelity {
    match candidate {
        RestoreCandidateClass::ExactRestore => WorkspaceRestoreFidelity::ExactRestore,
        RestoreCandidateClass::CompatibleRestore => WorkspaceRestoreFidelity::CompatibleRestore,
        RestoreCandidateClass::LayoutOnly => WorkspaceRestoreFidelity::LayoutOnly,
        RestoreCandidateClass::Excluded => WorkspaceRestoreFidelity::EvidenceOnly,
    }
}

fn map_alpha_schema_outcome(candidate: RestoreCandidateClass) -> WorkspaceSchemaOutcome {
    match candidate {
        RestoreCandidateClass::ExactRestore => WorkspaceSchemaOutcome::Exact,
        RestoreCandidateClass::CompatibleRestore => WorkspaceSchemaOutcome::Compatible,
        RestoreCandidateClass::LayoutOnly | RestoreCandidateClass::Excluded => {
            WorkspaceSchemaOutcome::LayoutOnly
        }
    }
}

fn map_alpha_pane_posture(posture: SurfaceRestorePosture) -> WorkspaceRestoreFidelity {
    match posture {
        SurfaceRestorePosture::Live => WorkspaceRestoreFidelity::ExactRestore,
        SurfaceRestorePosture::ContextOnly => WorkspaceRestoreFidelity::EvidenceOnly,
        SurfaceRestorePosture::PlaceholderOnly => WorkspaceRestoreFidelity::LayoutOnly,
        SurfaceRestorePosture::Excluded => WorkspaceRestoreFidelity::EvidenceOnly,
    }
}

fn map_alpha_placeholder_reason(reason: AlphaPlaceholderReason) -> MissingSurfaceDependency {
    match reason {
        AlphaPlaceholderReason::MissingExtension => MissingSurfaceDependency::MissingExtension,
        AlphaPlaceholderReason::MissingRemoteTarget => MissingSurfaceDependency::MissingRemote,
        AlphaPlaceholderReason::PolicyBlockedPane | AlphaPlaceholderReason::DisabledSurface => {
            MissingSurfaceDependency::RevokedPermission
        }
        AlphaPlaceholderReason::NonReentrantLiveSurface => {
            MissingSurfaceDependency::NonReentrantLiveSurface
        }
        AlphaPlaceholderReason::DisplayTopologyAdjusted => {
            MissingSurfaceDependency::DisplayTopologyMismatch
        }
    }
}

fn map_alpha_placeholder_action(action: AlphaPlaceholderAction) -> WorkspaceReviewAction {
    match action {
        AlphaPlaceholderAction::RetryHydrate => WorkspaceReviewAction::OpenDetails,
        AlphaPlaceholderAction::InstallExtension => WorkspaceReviewAction::InstallExtension,
        AlphaPlaceholderAction::Reauthenticate => WorkspaceReviewAction::Reauthenticate,
        AlphaPlaceholderAction::ReconnectRemote => WorkspaceReviewAction::Reconnect,
        AlphaPlaceholderAction::OpenContextOnly => WorkspaceReviewAction::OpenWithout,
        AlphaPlaceholderAction::ExportEvidence => WorkspaceReviewAction::ExportEvidence,
        AlphaPlaceholderAction::RemovePane => WorkspaceReviewAction::RemovePane,
        AlphaPlaceholderAction::RerunExplicitly => WorkspaceReviewAction::RerunExplicitly,
    }
}

fn map_alpha_machine_exclusion_reason(
    reason: AlphaMachineLocalExclusionReason,
) -> PortableStateExclusionReason {
    match reason {
        AlphaMachineLocalExclusionReason::ContainsSecretMaterial => {
            PortableStateExclusionReason::SecretMaterial
        }
        AlphaMachineLocalExclusionReason::ContainsLiveHandle => {
            PortableStateExclusionReason::LiveAuthorityHandle
        }
        AlphaMachineLocalExclusionReason::CredentialStoreOnly => {
            PortableStateExclusionReason::CredentialStoreOnly
        }
        AlphaMachineLocalExclusionReason::MachineUniqueHandle => {
            PortableStateExclusionReason::MachineUniqueTrustAnchor
        }
        AlphaMachineLocalExclusionReason::StateRootOnly => {
            PortableStateExclusionReason::StateRootOnly
        }
        AlphaMachineLocalExclusionReason::DisplayHintBestEffortOnly => {
            PortableStateExclusionReason::DisplayHintBestEffortOnly
        }
        AlphaMachineLocalExclusionReason::LocalAbsolutePath => {
            PortableStateExclusionReason::LocalAbsolutePath
        }
        AlphaMachineLocalExclusionReason::PolicyExcludesExport => {
            PortableStateExclusionReason::PolicyExcludesExport
        }
    }
}

fn map_alpha_substitute(
    substitute: crate::state_packages::ExclusionSubstituteClass,
) -> ExclusionSubstitute {
    match substitute {
        crate::state_packages::ExclusionSubstituteClass::OpaqueRef => {
            ExclusionSubstitute::OpaqueRef
        }
        crate::state_packages::ExclusionSubstituteClass::RedactedSummary => {
            ExclusionSubstitute::RedactedSummary
        }
        crate::state_packages::ExclusionSubstituteClass::SafePlaceholder => {
            ExclusionSubstitute::SafePlaceholder
        }
        crate::state_packages::ExclusionSubstituteClass::MetadataOnly => {
            ExclusionSubstitute::MetadataOnly
        }
        crate::state_packages::ExclusionSubstituteClass::Omitted => ExclusionSubstitute::Omitted,
    }
}

fn require_non_empty(
    field: &'static str,
    value: &str,
) -> Result<(), WorkspaceSerializationBetaError> {
    if value.trim().is_empty() {
        Err(WorkspaceSerializationBetaError::MissingField { field })
    } else {
        Ok(())
    }
}

fn require_option(
    field: &'static str,
    value: &Option<String>,
) -> Result<(), WorkspaceSerializationBetaError> {
    if value.as_deref().unwrap_or("").trim().is_empty() {
        Err(WorkspaceSerializationBetaError::MissingField { field })
    } else {
        Ok(())
    }
}
