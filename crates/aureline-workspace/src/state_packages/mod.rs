//! Workspace portable-state package primitives.
//!
//! This module owns the first runtime consumer for the workspace-specific
//! portable-state alpha package. The package separates workspace authority,
//! window topology, profile defaults, local session context, and machine-local
//! hints so restore, export, and support surfaces can inspect the same state
//! classes without serializing live authority or machine-unique handles.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::profiles::{ArtifactPortabilityLabel, PortableArtifactClass};

/// Schema version for workspace portable-state alpha packages.
pub const PORTABLE_STATE_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Schema path for the workspace portable-state alpha package.
pub const PORTABLE_STATE_ALPHA_SCHEMA_REF: &str =
    "schemas/workspace/portable_state_alpha.schema.json";

/// Schema path for the versioned pane-tree topology body.
pub const PANE_TREE_SCHEMA_REF: &str = "schemas/workspace/pane_tree.schema.json";

/// Schema path for linked portable-profile artifacts.
pub const PORTABLE_PROFILE_SCHEMA_REF: &str = "schemas/profile/portable_profile.schema.json";

/// Record kind for the workspace portable-state package body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PortableStateAlphaRecordKind {
    /// Package body containing core workspace, topology, profile, and machine-local rows.
    PortableStateAlphaPackage,
}

/// State class separated inside a workspace portable-state package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SerializedStateClass {
    /// Shared workspace authority and durable workspace state refs.
    WorkspaceAuthority,
    /// Window-local pane tree, focus, and placeholder posture.
    WindowTopology,
    /// Profile-carried defaults linked by explicit portable-profile artifact refs.
    ProfileDefaults,
    /// Machine-local display, geometry, state-root, or install hints.
    MachineLocalHints,
    /// Local context such as terminal evidence or session metadata.
    LocalSessionContext,
}

impl SerializedStateClass {
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

/// Persistence classification shown by the remembered-state inspector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersistenceClassification {
    /// State is owned by the current local machine or user profile.
    LocalOnly,
    /// State can travel in the package body.
    Portable,
    /// State is workspace-shared and must be applied through workspace review.
    Shared,
    /// State is machine-local metadata and not portable authority.
    MachineLocal,
}

impl PersistenceClassification {
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

/// How a state class appears in the package body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportMode {
    /// The package carries the schema-backed body.
    CarriedBody,
    /// The package carries opaque refs to a schema-backed body.
    ReferencedBody,
    /// The package links a different portable artifact explicitly.
    LinkedArtifactRef,
    /// The package carries metadata only.
    MetadataOnly,
    /// The package excludes the class and names why.
    Excluded,
}

/// Restore fidelity candidate for a state class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreCandidateClass {
    /// Candidate can restore exactly.
    ExactRestore,
    /// Candidate can restore compatibly with an explained translation or fallback.
    CompatibleRestore,
    /// Candidate can restore layout or context only.
    LayoutOnly,
    /// Candidate is excluded from restore.
    Excluded,
}

impl RestoreCandidateClass {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::Excluded => "excluded",
        }
    }
}

/// Schema binding and artifact refs for one state class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateSchemaBinding {
    /// Schema path or schema URI for the state class.
    pub schema_ref: String,
    /// Schema version pinned by this package.
    pub schema_version: u32,
    /// Opaque artifact refs carried or pointed to by this class.
    pub artifact_refs: Vec<String>,
}

/// Restore posture for one stable pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceRestorePosture {
    /// Pane can hydrate as a live surface.
    Live,
    /// Pane can reopen as context or evidence only.
    ContextOnly,
    /// Pane must reopen as a placeholder card.
    PlaceholderOnly,
    /// Pane is intentionally excluded.
    Excluded,
}

impl SurfaceRestorePosture {
    /// Returns `true` when live capabilities must not be implied.
    pub const fn requires_manual_action(self) -> bool {
        matches!(self, Self::ContextOnly | Self::PlaceholderOnly)
    }

    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::ContextOnly => "context_only",
            Self::PlaceholderOnly => "placeholder_only",
            Self::Excluded => "excluded",
        }
    }
}

/// Why a pane reopens as context or a placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderReason {
    /// Required extension or feature pack is unavailable.
    MissingExtension,
    /// Required remote target is unavailable.
    MissingRemoteTarget,
    /// Policy blocks the pane from hydrating.
    PolicyBlockedPane,
    /// User or policy disabled the surface.
    DisabledSurface,
    /// Live surface cannot safely resume without explicit user action.
    NonReentrantLiveSurface,
    /// Display topology changed and degraded the pane.
    DisplayTopologyAdjusted,
}

/// Guardrail preventing hidden rerun or hidden authority reacquire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoRerunGuardrail {
    /// Terminal commands must not rerun.
    NoCommandRerun,
    /// Tasks must not rerun.
    NoTaskRerun,
    /// Debuggers must not reattach silently.
    NoDebuggerReattach,
    /// Notebook kernels must not restart silently.
    NoNotebookKernelRestart,
    /// Preview servers must not restart silently.
    NoPreviewServerRestart,
    /// Remote sessions must not resume silently.
    NoRemoteSessionResume,
    /// User action is required before live behavior resumes.
    ExplicitUserActionRequired,
    /// Placeholder card keeps the pane slot visible.
    PlaceholderPreserved,
}

/// Safe action offered from a placeholder card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlaceholderAction {
    /// Try hydration again.
    RetryHydrate,
    /// Install or enable the missing extension.
    InstallExtension,
    /// Reauthenticate the missing authority.
    Reauthenticate,
    /// Reconnect the missing remote.
    ReconnectRemote,
    /// Open retained metadata without live capability.
    OpenContextOnly,
    /// Export retained evidence.
    ExportEvidence,
    /// Remove the pane slot deliberately.
    RemovePane,
    /// Rerun only after explicit user action.
    RerunExplicitly,
}

/// Placeholder card preserved in a pane slot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderCard {
    /// Reason the pane could not hydrate live.
    pub reason: PlaceholderReason,
    /// Safe actions that do not silently widen authority.
    pub safe_actions: Vec<PlaceholderAction>,
    /// Whether redacted evidence or context remains available.
    pub evidence_retained: bool,
    /// Last known redaction-aware label, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_label: Option<String>,
}

/// Restore posture for one stable pane id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneRestorePosture {
    /// Stable pane id from the pane-tree schema.
    pub stable_pane_id: String,
    /// Surface role label such as editor, terminal, preview, or notebook.
    pub surface_role: String,
    /// Surface class label such as text_editor or terminal_view.
    pub surface_class: String,
    /// Resulting restore posture.
    pub restore_posture: SurfaceRestorePosture,
    /// Placeholder card, required for placeholder-only panes and useful for context-only panes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_card: Option<PlaceholderCard>,
    /// No-rerun guardrails attached to this pane.
    #[serde(default)]
    pub no_rerun_guardrails: Vec<NoRerunGuardrail>,
    /// Last time this pane posture was written.
    pub last_written_at: String,
}

/// One separated state class inside a workspace portable-state package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateClassRecord {
    /// Stable class row id.
    pub class_id: String,
    /// Class kind.
    pub class_kind: SerializedStateClass,
    /// Persistence classification shown to users and support.
    pub classification: PersistenceClassification,
    /// Package export mode for this class.
    pub export_mode: ExportMode,
    /// Schema binding and artifact refs for this class.
    pub schema_binding: StateSchemaBinding,
    /// Last write time for this class.
    pub last_written_at: String,
    /// Restore fidelity candidate for this class.
    pub restore_candidate: RestoreCandidateClass,
    /// Whether export can include this class or its refs.
    pub export_allowed: bool,
    /// Whether clearing this remembered-state class is allowed.
    pub clear_allowed: bool,
    /// Pane-level restore rows for topology or local context classes.
    #[serde(default)]
    pub pane_restore_postures: Vec<PaneRestorePosture>,
    /// Explicit linked profile artifacts referenced by this class.
    #[serde(default)]
    pub linked_profile_artifact_refs: Vec<String>,
    /// Explanation for local-only or machine-local rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_reason: Option<String>,
    /// Redaction-aware notes.
    pub notes: String,
}

/// Explicit link to a portable-profile artifact consumed by this package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LinkedProfileArtifactRef {
    /// Opaque profile artifact ref.
    pub artifact_ref: String,
    /// Profile artifact class from the portable-profile contract.
    pub artifact_class: PortableArtifactClass,
    /// Portability label from the portable-profile contract.
    pub portability_label: ArtifactPortabilityLabel,
    /// Schema path for the linked profile artifact.
    pub schema_ref: String,
    /// Opaque source profile or package ref.
    pub source_ref: String,
    /// Redaction-aware notes.
    pub notes: String,
}

/// Redaction rules enforced before the package crosses a machine boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionRuleClass {
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
    /// Raw filesystem paths are excluded.
    RawPathExcluded,
    /// Raw hostnames are excluded.
    RawHostExcluded,
    /// Raw command lines are excluded.
    RawCommandLineExcluded,
    /// Raw logs are excluded.
    RawLogExcluded,
    /// Raw source content is excluded.
    RawSourceContentExcluded,
    /// Provider payload bodies are excluded.
    ProviderPayloadExcluded,
}

/// Redaction manifest attached to a package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionManifest {
    /// Stable redaction manifest id.
    pub manifest_id: String,
    /// Exclusion rules applied by the exporter.
    pub rules: Vec<RedactionRuleClass>,
    /// Whether machine-local exclusions were reviewed before export.
    pub machine_local_exclusions_reviewed: bool,
    /// Redaction-aware notes.
    pub notes: String,
}

/// Reason a machine-local object is excluded or reduced to metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MachineLocalExclusionReason {
    /// Excluded class can contain raw secret material.
    ContainsSecretMaterial,
    /// Excluded class can contain live authority handles.
    ContainsLiveHandle,
    /// Excluded class belongs in the credential store.
    CredentialStoreOnly,
    /// Excluded class is unique to this machine.
    MachineUniqueHandle,
    /// Excluded class is a concrete state root.
    StateRootOnly,
    /// Excluded class is a best-effort display hint.
    DisplayHintBestEffortOnly,
    /// Excluded class can contain local absolute paths.
    LocalAbsolutePath,
    /// Policy excludes this class from export.
    PolicyExcludesExport,
}

/// Substitute used when a machine-local object is excluded.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExclusionSubstituteClass {
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

/// One machine-local exclusion row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MachineLocalExclusion {
    /// Stable exclusion id.
    pub exclusion_id: String,
    /// State class affected by the exclusion.
    pub class_kind: SerializedStateClass,
    /// Opaque excluded artifact ref.
    pub artifact_ref: String,
    /// Exclusion reason.
    pub reason: MachineLocalExclusionReason,
    /// Substitute retained in the package.
    pub substitute_class: ExclusionSubstituteClass,
    /// Redaction-aware notes.
    pub notes: String,
}

/// Display-topology adjustment recorded during restore or import review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DisplayAdjustmentClass {
    /// Window was snapped into safe visible bounds.
    SnappedToSafeBounds,
    /// Window was moved to the primary display.
    MovedToPrimaryDisplay,
    /// Window was normalized across scale buckets.
    ScaleNormalized,
    /// Fullscreen state was cleared.
    FullscreenCleared,
    /// Fidelity downgraded to layout-only.
    LayoutOnlyFallback,
}

/// Topology-adjustment provenance for display changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TopologyAdjustment {
    /// Stable adjustment id.
    pub adjustment_id: String,
    /// Adjustment class.
    pub adjustment_class: DisplayAdjustmentClass,
    /// Whether the display topology changed.
    pub display_topology_changed: bool,
    /// Pane ids whose layout intent was preserved.
    pub affected_pane_ids: Vec<String>,
    /// Whether the destination verified visible bounds after adjustment.
    pub visible_bounds_verified: bool,
    /// Restore fidelity after the adjustment.
    pub restore_fidelity_after_adjustment: RestoreCandidateClass,
    /// Redaction-aware notes.
    pub notes: String,
}

/// Restore provenance summary carried by the package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateRestoreProvenance {
    /// Source topology snapshot refs.
    pub source_snapshot_refs: Vec<String>,
    /// Restore provenance record refs.
    pub restore_provenance_refs: Vec<String>,
    /// Display or topology adjustments applied or expected.
    #[serde(default)]
    pub topology_adjustments: Vec<TopologyAdjustment>,
    /// Pane placeholder and context summary.
    #[serde(default)]
    pub placeholder_summary: Vec<PaneRestorePosture>,
    /// Redaction-aware notes.
    pub notes: String,
}

/// Package or inspector action kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RememberedStateActionKind {
    /// Inspect without mutation.
    Inspect,
    /// Export or open the export review.
    Export,
    /// Clear selected remembered state only.
    Clear,
    /// Compare with local or prior state.
    Compare,
}

impl RememberedStateActionKind {
    /// Returns the stable serialized token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Inspect => "inspect",
            Self::Export => "export",
            Self::Clear => "clear",
            Self::Compare => "compare",
        }
    }
}

/// One package action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedStateAction {
    /// Action kind.
    pub action: RememberedStateActionKind,
    /// Whether the action is available.
    pub enabled: bool,
    /// Target ref when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_ref: Option<String>,
    /// Disabled reason when unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
}

/// Workspace portable-state package body for core classes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PortableStateAlphaPackage {
    /// Record discriminator.
    pub record_kind: PortableStateAlphaRecordKind,
    /// Schema version.
    pub schema_version: u32,
    /// Stable package id.
    pub package_id: String,
    /// Portable-state manifest ref that wraps this body.
    pub manifest_id: String,
    /// Workspace ref the package describes.
    pub workspace_ref: String,
    /// Package creation time.
    pub created_at: String,
    /// Producer build or instance ref.
    pub producer_ref: String,
    /// Separated state class rows.
    pub state_classes: Vec<PortableStateClassRecord>,
    /// Linked profile artifacts that remain outside workspace authority.
    #[serde(default)]
    pub linked_profile_artifacts: Vec<LinkedProfileArtifactRef>,
    /// Redaction manifest for the package.
    pub redaction_manifest: RedactionManifest,
    /// Machine-local exclusions named by the package.
    #[serde(default)]
    pub machine_local_exclusions: Vec<MachineLocalExclusion>,
    /// Restore provenance and topology-adjustment summary.
    pub restore_provenance: PortableStateRestoreProvenance,
    /// Package-level actions.
    #[serde(default)]
    pub actions: Vec<RememberedStateAction>,
    /// Redaction-aware notes.
    pub notes: String,
}

impl PortableStateAlphaPackage {
    /// Validates package separation, redaction, placeholder, and topology invariants.
    pub fn validate(&self) -> Result<(), PortableStateAlphaValidationError> {
        if self.record_kind != PortableStateAlphaRecordKind::PortableStateAlphaPackage {
            return Err(PortableStateAlphaValidationError::WrongRecordKind);
        }
        if self.schema_version != PORTABLE_STATE_ALPHA_SCHEMA_VERSION {
            return Err(PortableStateAlphaValidationError::WrongSchemaVersion {
                expected: PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.package_id.trim().is_empty()
            || self.manifest_id.trim().is_empty()
            || self.workspace_ref.trim().is_empty()
            || self.created_at.trim().is_empty()
            || self.producer_ref.trim().is_empty()
        {
            return Err(PortableStateAlphaValidationError::MissingPackageAttribution);
        }

        let mut observed_classes = BTreeSet::new();
        for state_class in &self.state_classes {
            if !observed_classes.insert(state_class.class_kind) {
                return Err(PortableStateAlphaValidationError::DuplicateStateClass {
                    class_kind: state_class.class_kind,
                });
            }
            state_class.validate()?;
        }

        for required in [
            SerializedStateClass::WorkspaceAuthority,
            SerializedStateClass::WindowTopology,
            SerializedStateClass::ProfileDefaults,
            SerializedStateClass::MachineLocalHints,
        ] {
            if !observed_classes.contains(&required) {
                return Err(PortableStateAlphaValidationError::MissingStateClass {
                    class_kind: required,
                });
            }
        }

        self.validate_redaction_manifest()?;
        self.validate_profile_links()?;
        self.validate_machine_local_exclusions()?;
        self.validate_restore_provenance()?;
        self.validate_actions()?;
        Ok(())
    }

    /// Builds remembered-state inspector rows from the package body.
    pub fn inspector(&self) -> Result<RememberedStateInspector, PortableStateAlphaValidationError> {
        self.validate()?;
        Ok(RememberedStateInspector::from_package(self))
    }

    fn validate_redaction_manifest(&self) -> Result<(), PortableStateAlphaValidationError> {
        let rules: BTreeSet<_> = self.redaction_manifest.rules.iter().copied().collect();
        for required in [
            RedactionRuleClass::RawSecretMaterialExcluded,
            RedactionRuleClass::ApprovalTicketExcluded,
            RedactionRuleClass::DelegatedCredentialExcluded,
            RedactionRuleClass::LiveAuthorityHandleExcluded,
            RedactionRuleClass::MachineUniqueHandleExcluded,
            RedactionRuleClass::StateRootExcluded,
        ] {
            if !rules.contains(&required) {
                return Err(PortableStateAlphaValidationError::MissingRedactionRule {
                    rule: required,
                });
            }
        }
        if !self.redaction_manifest.machine_local_exclusions_reviewed {
            return Err(PortableStateAlphaValidationError::MachineLocalExclusionsNotReviewed);
        }
        Ok(())
    }

    fn validate_profile_links(&self) -> Result<(), PortableStateAlphaValidationError> {
        let profile_refs: BTreeSet<_> = self
            .linked_profile_artifacts
            .iter()
            .map(|artifact| artifact.artifact_ref.as_str())
            .collect();
        for artifact in &self.linked_profile_artifacts {
            if artifact.artifact_ref.trim().is_empty()
                || artifact.schema_ref != PORTABLE_PROFILE_SCHEMA_REF
                || artifact.source_ref.trim().is_empty()
            {
                return Err(
                    PortableStateAlphaValidationError::InvalidLinkedProfileArtifact {
                        artifact_ref: artifact.artifact_ref.clone(),
                    },
                );
            }
            if artifact.portability_label == ArtifactPortabilityLabel::Excluded {
                return Err(
                    PortableStateAlphaValidationError::InvalidLinkedProfileArtifact {
                        artifact_ref: artifact.artifact_ref.clone(),
                    },
                );
            }
        }
        for state_class in self
            .state_classes
            .iter()
            .filter(|row| row.class_kind == SerializedStateClass::ProfileDefaults)
        {
            if state_class.export_mode == ExportMode::LinkedArtifactRef
                && state_class.linked_profile_artifact_refs.is_empty()
            {
                return Err(PortableStateAlphaValidationError::MissingLinkedProfileArtifact);
            }
            for artifact_ref in &state_class.linked_profile_artifact_refs {
                if !profile_refs.contains(artifact_ref.as_str()) {
                    return Err(
                        PortableStateAlphaValidationError::ProfileArtifactRefNotLinked {
                            artifact_ref: artifact_ref.clone(),
                        },
                    );
                }
            }
        }
        Ok(())
    }

    fn validate_machine_local_exclusions(&self) -> Result<(), PortableStateAlphaValidationError> {
        if self.machine_local_exclusions.is_empty() {
            return Err(PortableStateAlphaValidationError::MissingMachineLocalExclusion);
        }
        let reasons: BTreeSet<_> = self
            .machine_local_exclusions
            .iter()
            .map(|row| row.reason)
            .collect();
        for required in [
            MachineLocalExclusionReason::ContainsLiveHandle,
            MachineLocalExclusionReason::DisplayHintBestEffortOnly,
            MachineLocalExclusionReason::StateRootOnly,
            MachineLocalExclusionReason::CredentialStoreOnly,
        ] {
            if !reasons.contains(&required) {
                return Err(
                    PortableStateAlphaValidationError::MissingMachineLocalExclusionReason {
                        reason: required,
                    },
                );
            }
        }
        Ok(())
    }

    fn validate_restore_provenance(&self) -> Result<(), PortableStateAlphaValidationError> {
        for adjustment in &self.restore_provenance.topology_adjustments {
            if adjustment.display_topology_changed && !adjustment.visible_bounds_verified {
                return Err(
                    PortableStateAlphaValidationError::DisplayTopologyAdjustmentUnverified {
                        adjustment_id: adjustment.adjustment_id.clone(),
                    },
                );
            }
            if adjustment.display_topology_changed && adjustment.affected_pane_ids.is_empty() {
                return Err(
                    PortableStateAlphaValidationError::DisplayTopologyLostPaneIds {
                        adjustment_id: adjustment.adjustment_id.clone(),
                    },
                );
            }
        }

        let mut postures = BTreeSet::new();
        for state_class in &self.state_classes {
            if state_class.class_kind == SerializedStateClass::WindowTopology {
                for pane in &state_class.pane_restore_postures {
                    postures.insert(pane.restore_posture);
                }
            }
        }
        for required in [
            SurfaceRestorePosture::Live,
            SurfaceRestorePosture::ContextOnly,
            SurfaceRestorePosture::PlaceholderOnly,
        ] {
            if !postures.contains(&required) {
                return Err(
                    PortableStateAlphaValidationError::MissingPaneRestorePosture {
                        posture: required,
                    },
                );
            }
        }
        Ok(())
    }

    fn validate_actions(&self) -> Result<(), PortableStateAlphaValidationError> {
        let actions: BTreeSet<_> = self.actions.iter().map(|action| action.action).collect();
        for required in [
            RememberedStateActionKind::Inspect,
            RememberedStateActionKind::Export,
            RememberedStateActionKind::Compare,
            RememberedStateActionKind::Clear,
        ] {
            if !actions.contains(&required) {
                return Err(PortableStateAlphaValidationError::MissingPackageAction {
                    action: required,
                });
            }
        }
        Ok(())
    }
}

impl PortableStateClassRecord {
    /// Validates one state class row.
    pub fn validate(&self) -> Result<(), PortableStateAlphaValidationError> {
        if self.class_id.trim().is_empty()
            || self.schema_binding.schema_ref.trim().is_empty()
            || self.last_written_at.trim().is_empty()
        {
            return Err(
                PortableStateAlphaValidationError::StateClassMissingAttribution {
                    class_id: self.class_id.clone(),
                },
            );
        }

        if self.class_kind == SerializedStateClass::MachineLocalHints {
            if self.classification != PersistenceClassification::MachineLocal {
                return Err(
                    PortableStateAlphaValidationError::MachineLocalHintsMisclassified {
                        class_id: self.class_id.clone(),
                    },
                );
            }
            if self.export_allowed || self.export_mode == ExportMode::CarriedBody {
                return Err(
                    PortableStateAlphaValidationError::MachineLocalClassExported {
                        class_id: self.class_id.clone(),
                    },
                );
            }
        }

        if self.class_kind == SerializedStateClass::WindowTopology {
            if self.schema_binding.schema_ref != PANE_TREE_SCHEMA_REF
                || self.schema_binding.schema_version != 1
            {
                return Err(
                    PortableStateAlphaValidationError::WindowTopologySchemaMismatch {
                        class_id: self.class_id.clone(),
                    },
                );
            }
            if self.pane_restore_postures.is_empty() {
                return Err(
                    PortableStateAlphaValidationError::WindowTopologyMissingPanes {
                        class_id: self.class_id.clone(),
                    },
                );
            }
        }

        let mut pane_ids = BTreeSet::new();
        for pane in &self.pane_restore_postures {
            if !pane_ids.insert(pane.stable_pane_id.as_str()) {
                return Err(PortableStateAlphaValidationError::DuplicatePaneId {
                    pane_id: pane.stable_pane_id.clone(),
                });
            }
            pane.validate()?;
        }

        if matches!(
            self.classification,
            PersistenceClassification::LocalOnly | PersistenceClassification::MachineLocal
        ) && self
            .local_only_reason
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
        {
            return Err(PortableStateAlphaValidationError::LocalOnlyReasonMissing {
                class_id: self.class_id.clone(),
            });
        }

        Ok(())
    }
}

impl PaneRestorePosture {
    /// Validates placeholder and no-rerun honesty for one pane.
    pub fn validate(&self) -> Result<(), PortableStateAlphaValidationError> {
        if self.stable_pane_id.trim().is_empty()
            || self.surface_role.trim().is_empty()
            || self.surface_class.trim().is_empty()
            || self.last_written_at.trim().is_empty()
        {
            return Err(PortableStateAlphaValidationError::PaneMissingAttribution {
                pane_id: self.stable_pane_id.clone(),
            });
        }
        if self.restore_posture == SurfaceRestorePosture::PlaceholderOnly
            && self.placeholder_card.is_none()
        {
            return Err(PortableStateAlphaValidationError::PaneMissingPlaceholder {
                pane_id: self.stable_pane_id.clone(),
            });
        }
        if self.restore_posture.requires_manual_action() {
            let guardrails: BTreeSet<_> = self.no_rerun_guardrails.iter().copied().collect();
            if !guardrails.contains(&NoRerunGuardrail::ExplicitUserActionRequired)
                || !guardrails.contains(&NoRerunGuardrail::PlaceholderPreserved)
            {
                return Err(
                    PortableStateAlphaValidationError::PaneMissingNoRerunGuardrail {
                        pane_id: self.stable_pane_id.clone(),
                    },
                );
            }
        }
        if let Some(card) = &self.placeholder_card {
            if card.safe_actions.is_empty() {
                return Err(PortableStateAlphaValidationError::PaneMissingSafeAction {
                    pane_id: self.stable_pane_id.clone(),
                });
            }
        }
        Ok(())
    }
}

/// Support-safe inspector model over a portable-state package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedStateInspector {
    /// Package id being inspected.
    pub package_id: String,
    /// Package schema version.
    pub schema_version: u32,
    /// Inspector rows in display order.
    pub rows: Vec<RememberedStateInspectorRow>,
}

impl RememberedStateInspector {
    fn from_package(package: &PortableStateAlphaPackage) -> Self {
        let mut rows = Vec::new();
        for state_class in &package.state_classes {
            rows.push(RememberedStateInspectorRow::from_state_class(state_class));
            for pane in &state_class.pane_restore_postures {
                rows.push(RememberedStateInspectorRow::from_pane(state_class, pane));
            }
        }
        Self {
            package_id: package.package_id.clone(),
            schema_version: package.schema_version,
            rows,
        }
    }

    /// Renders a support-safe plaintext view of the inspector.
    pub fn render_plaintext(&self) -> String {
        let mut lines = Vec::new();
        lines.push(format!(
            "Remembered State Inspector package={} schema_version={}",
            self.package_id, self.schema_version
        ));
        for row in &self.rows {
            lines.push(format!(
                "- {} {} classification={} schema_version={} last_written_at={} restore_candidate={} actions={}",
                row.state_class.as_str(),
                row.artifact_ref,
                row.classification.as_str(),
                row.schema_version,
                row.last_written_at,
                row.restore_candidate.as_str(),
                row.actions
                    .iter()
                    .map(|action| action.as_str())
                    .collect::<Vec<_>>()
                    .join(",")
            ));
            if let Some(pane_id) = &row.stable_pane_id {
                lines.push(format!(
                    "  pane={} posture={}",
                    pane_id,
                    row.pane_restore_posture
                        .map(SurfaceRestorePosture::as_str)
                        .unwrap_or("unknown")
                ));
            }
            if let Some(reason) = &row.local_only_reason {
                lines.push(format!("  reason={reason}"));
            }
        }
        lines.join("\n")
    }
}

/// One support-safe row in the remembered-state inspector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RememberedStateInspectorRow {
    /// Artifact or class ref for the row.
    pub artifact_ref: String,
    /// State class represented by the row.
    pub state_class: SerializedStateClass,
    /// Persistence classification.
    pub classification: PersistenceClassification,
    /// Schema version.
    pub schema_version: u32,
    /// Last write time.
    pub last_written_at: String,
    /// Restore candidate class.
    pub restore_candidate: RestoreCandidateClass,
    /// Stable pane id when the row is pane-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stable_pane_id: Option<String>,
    /// Pane restore posture when the row is pane-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pane_restore_posture: Option<SurfaceRestorePosture>,
    /// Available row actions.
    pub actions: Vec<RememberedStateActionKind>,
    /// Local-only or machine-local explanation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_reason: Option<String>,
}

impl RememberedStateInspectorRow {
    fn from_state_class(state_class: &PortableStateClassRecord) -> Self {
        let mut actions = vec![RememberedStateActionKind::Inspect];
        if state_class.export_allowed {
            actions.push(RememberedStateActionKind::Export);
        }
        if state_class.clear_allowed {
            actions.push(RememberedStateActionKind::Clear);
        }
        if state_class.restore_candidate != RestoreCandidateClass::Excluded {
            actions.push(RememberedStateActionKind::Compare);
        }
        Self {
            artifact_ref: state_class.class_id.clone(),
            state_class: state_class.class_kind,
            classification: state_class.classification,
            schema_version: state_class.schema_binding.schema_version,
            last_written_at: state_class.last_written_at.clone(),
            restore_candidate: state_class.restore_candidate,
            stable_pane_id: None,
            pane_restore_posture: None,
            actions,
            local_only_reason: state_class.local_only_reason.clone(),
        }
    }

    fn from_pane(state_class: &PortableStateClassRecord, pane: &PaneRestorePosture) -> Self {
        let mut actions = vec![RememberedStateActionKind::Inspect];
        if state_class.export_allowed {
            actions.push(RememberedStateActionKind::Export);
        }
        if state_class.clear_allowed {
            actions.push(RememberedStateActionKind::Clear);
        }
        actions.push(RememberedStateActionKind::Compare);
        Self {
            artifact_ref: pane.stable_pane_id.clone(),
            state_class: state_class.class_kind,
            classification: state_class.classification,
            schema_version: state_class.schema_binding.schema_version,
            last_written_at: pane.last_written_at.clone(),
            restore_candidate: state_class.restore_candidate,
            stable_pane_id: Some(pane.stable_pane_id.clone()),
            pane_restore_posture: Some(pane.restore_posture),
            actions,
            local_only_reason: state_class.local_only_reason.clone(),
        }
    }
}

/// Validation errors for workspace portable-state packages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PortableStateAlphaValidationError {
    /// The record kind is not the portable-state alpha package kind.
    WrongRecordKind,
    /// The schema version is unsupported.
    WrongSchemaVersion {
        /// Expected schema version.
        expected: u32,
        /// Actual schema version.
        actual: u32,
    },
    /// Package-level identity or producer attribution is missing.
    MissingPackageAttribution,
    /// A required separated state class is absent.
    MissingStateClass {
        /// Missing state class.
        class_kind: SerializedStateClass,
    },
    /// A state class kind appears more than once.
    DuplicateStateClass {
        /// Duplicated state class.
        class_kind: SerializedStateClass,
    },
    /// A state class row lacks attribution.
    StateClassMissingAttribution {
        /// State class id.
        class_id: String,
    },
    /// Machine-local hints are not classified as machine-local.
    MachineLocalHintsMisclassified {
        /// State class id.
        class_id: String,
    },
    /// A machine-local class would be exported as authority.
    MachineLocalClassExported {
        /// State class id.
        class_id: String,
    },
    /// Window topology does not bind to the pane-tree schema.
    WindowTopologySchemaMismatch {
        /// State class id.
        class_id: String,
    },
    /// Window topology has no pane rows.
    WindowTopologyMissingPanes {
        /// State class id.
        class_id: String,
    },
    /// Duplicate stable pane id.
    DuplicatePaneId {
        /// Duplicated pane id.
        pane_id: String,
    },
    /// Pane identity or surface attribution is missing.
    PaneMissingAttribution {
        /// Pane id.
        pane_id: String,
    },
    /// Placeholder-only pane lacks placeholder metadata.
    PaneMissingPlaceholder {
        /// Pane id.
        pane_id: String,
    },
    /// Context or placeholder pane lacks no-rerun guardrails.
    PaneMissingNoRerunGuardrail {
        /// Pane id.
        pane_id: String,
    },
    /// Placeholder pane lacks safe actions.
    PaneMissingSafeAction {
        /// Pane id.
        pane_id: String,
    },
    /// Local-only or machine-local state lacks explanation.
    LocalOnlyReasonMissing {
        /// State class id.
        class_id: String,
    },
    /// A required redaction rule is missing.
    MissingRedactionRule {
        /// Missing rule.
        rule: RedactionRuleClass,
    },
    /// Machine-local exclusions were not reviewed.
    MachineLocalExclusionsNotReviewed,
    /// No linked profile artifact is present.
    MissingLinkedProfileArtifact,
    /// Linked profile artifact is malformed.
    InvalidLinkedProfileArtifact {
        /// Artifact ref.
        artifact_ref: String,
    },
    /// Profile default row points to an unlisted artifact.
    ProfileArtifactRefNotLinked {
        /// Artifact ref.
        artifact_ref: String,
    },
    /// No machine-local exclusion row is present.
    MissingMachineLocalExclusion,
    /// A required machine-local exclusion reason is missing.
    MissingMachineLocalExclusionReason {
        /// Missing reason.
        reason: MachineLocalExclusionReason,
    },
    /// Display-topology adjustment changed displays without visible-bounds proof.
    DisplayTopologyAdjustmentUnverified {
        /// Adjustment id.
        adjustment_id: String,
    },
    /// Display-topology adjustment lost pane-id provenance.
    DisplayTopologyLostPaneIds {
        /// Adjustment id.
        adjustment_id: String,
    },
    /// The window topology did not prove one required pane posture.
    MissingPaneRestorePosture {
        /// Missing posture.
        posture: SurfaceRestorePosture,
    },
    /// Package action is missing.
    MissingPackageAction {
        /// Missing action.
        action: RememberedStateActionKind,
    },
}

impl fmt::Display for PortableStateAlphaValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongRecordKind => write!(f, "wrong portable-state alpha record kind"),
            Self::WrongSchemaVersion { expected, actual } => {
                write!(f, "expected schema version {expected}, got {actual}")
            }
            Self::MissingPackageAttribution => {
                write!(f, "package identity or producer attribution is missing")
            }
            Self::MissingStateClass { class_kind } => {
                write!(f, "missing state class {}", class_kind.as_str())
            }
            Self::DuplicateStateClass { class_kind } => {
                write!(f, "duplicate state class {}", class_kind.as_str())
            }
            Self::StateClassMissingAttribution { class_id } => {
                write!(f, "state class {class_id} is missing attribution")
            }
            Self::MachineLocalHintsMisclassified { class_id } => {
                write!(f, "machine-local hints {class_id} are misclassified")
            }
            Self::MachineLocalClassExported { class_id } => {
                write!(
                    f,
                    "machine-local class {class_id} would be exported as authority"
                )
            }
            Self::WindowTopologySchemaMismatch { class_id } => {
                write!(
                    f,
                    "window topology {class_id} does not bind to pane-tree schema"
                )
            }
            Self::WindowTopologyMissingPanes { class_id } => {
                write!(f, "window topology {class_id} has no pane rows")
            }
            Self::DuplicatePaneId { pane_id } => write!(f, "duplicate pane id {pane_id}"),
            Self::PaneMissingAttribution { pane_id } => {
                write!(f, "pane {pane_id} is missing attribution")
            }
            Self::PaneMissingPlaceholder { pane_id } => {
                write!(f, "pane {pane_id} is missing placeholder metadata")
            }
            Self::PaneMissingNoRerunGuardrail { pane_id } => {
                write!(f, "pane {pane_id} is missing no-rerun guardrails")
            }
            Self::PaneMissingSafeAction { pane_id } => {
                write!(f, "pane {pane_id} placeholder has no safe action")
            }
            Self::LocalOnlyReasonMissing { class_id } => {
                write!(
                    f,
                    "local-only or machine-local class {class_id} lacks a reason"
                )
            }
            Self::MissingRedactionRule { rule } => {
                write!(f, "redaction manifest is missing {rule:?}")
            }
            Self::MachineLocalExclusionsNotReviewed => {
                write!(f, "machine-local exclusions were not reviewed")
            }
            Self::MissingLinkedProfileArtifact => write!(f, "missing linked profile artifact"),
            Self::InvalidLinkedProfileArtifact { artifact_ref } => {
                write!(f, "linked profile artifact {artifact_ref} is invalid")
            }
            Self::ProfileArtifactRefNotLinked { artifact_ref } => {
                write!(f, "profile artifact ref {artifact_ref} is not linked")
            }
            Self::MissingMachineLocalExclusion => write!(f, "missing machine-local exclusion"),
            Self::MissingMachineLocalExclusionReason { reason } => {
                write!(f, "missing machine-local exclusion reason {reason:?}")
            }
            Self::DisplayTopologyAdjustmentUnverified { adjustment_id } => {
                write!(
                    f,
                    "display-topology adjustment {adjustment_id} lacks visible-bounds proof"
                )
            }
            Self::DisplayTopologyLostPaneIds { adjustment_id } => {
                write!(
                    f,
                    "display-topology adjustment {adjustment_id} lost pane-id provenance"
                )
            }
            Self::MissingPaneRestorePosture { posture } => {
                write!(f, "missing pane restore posture {}", posture.as_str())
            }
            Self::MissingPackageAction { action } => {
                write!(f, "missing package action {}", action.as_str())
            }
        }
    }
}

impl std::error::Error for PortableStateAlphaValidationError {}
