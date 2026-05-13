//! Shell consumers for workspace layout serialization and portable state.
//!
//! The shell owns the first runtime projection from an in-memory split tree to
//! the workspace portable-state package model. The projection keeps stable pane
//! ids, placeholder metadata, profile defaults, and machine-local display hints
//! in separate rows so inspector and export surfaces can explain what will be
//! restored, exported, or cleared.

use std::collections::BTreeMap;
use std::fmt;

use aureline_workspace::{
    ArtifactPortabilityLabel, DisplayAdjustmentClass, ExclusionSubstituteClass, ExportMode,
    LinkedProfileArtifactRef, MachineLocalExclusion, MachineLocalExclusionReason, NoRerunGuardrail,
    PaneRestorePosture, PersistenceClassification, PlaceholderCard, PortableArtifactClass,
    PortableStateAlphaPackage, PortableStateAlphaRecordKind, PortableStateAlphaValidationError,
    PortableStateClassRecord, PortableStateRestoreProvenance, RedactionManifest,
    RedactionRuleClass, RememberedStateAction, RememberedStateActionKind, RememberedStateInspector,
    RestoreCandidateClass, SerializedStateClass, StateSchemaBinding, SurfaceRestorePosture,
    TopologyAdjustment, PANE_TREE_SCHEMA_REF, PORTABLE_PROFILE_SCHEMA_REF,
    PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
};

use crate::layout::split_tree::{PaneId, SplitTree};

/// Canonical command id for opening the remembered-state inspector.
pub const REMEMBERED_STATE_INSPECTOR_COMMAND_ID: &str =
    "cmd:workspace.open_remembered_state_inspector";

/// Pane surface data supplied by the live shell before serialization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaneSurfaceSerialization {
    /// Stable pane id minted by the split tree.
    pub pane_id: PaneId,
    /// Surface role label such as editor, terminal, preview, or notebook.
    pub surface_role: String,
    /// Surface class label such as text_editor or terminal_view.
    pub surface_class: String,
    /// Restore posture for this pane.
    pub restore_posture: SurfaceRestorePosture,
    /// Placeholder card when the pane is unavailable or context-only.
    pub placeholder_card: Option<PlaceholderCard>,
    /// No-rerun guardrails for live-capability surfaces.
    pub no_rerun_guardrails: Vec<NoRerunGuardrail>,
}

impl PaneSurfaceSerialization {
    /// Creates a pane surface serialization row.
    pub fn new(
        pane_id: PaneId,
        surface_role: impl Into<String>,
        surface_class: impl Into<String>,
        restore_posture: SurfaceRestorePosture,
    ) -> Self {
        Self {
            pane_id,
            surface_role: surface_role.into(),
            surface_class: surface_class.into(),
            restore_posture,
            placeholder_card: None,
            no_rerun_guardrails: Vec::new(),
        }
    }

    /// Attaches placeholder metadata.
    pub fn with_placeholder(mut self, card: PlaceholderCard) -> Self {
        self.placeholder_card = Some(card);
        self
    }

    /// Attaches no-rerun guardrails.
    pub fn with_guardrails(mut self, guardrails: Vec<NoRerunGuardrail>) -> Self {
        self.no_rerun_guardrails = guardrails;
        self
    }
}

/// Input required to build a workspace portable-state package from the shell.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutPortableStateRequest {
    /// Stable package id.
    pub package_id: String,
    /// Portable-state manifest ref.
    pub manifest_id: String,
    /// Workspace ref being exported.
    pub workspace_ref: String,
    /// Package creation timestamp.
    pub created_at: String,
    /// Producer ref.
    pub producer_ref: String,
    /// Last-write timestamp used for generated class rows.
    pub last_written_at: String,
    /// Window-topology snapshot ref.
    pub window_topology_snapshot_ref: String,
    /// Machine-display hint ref.
    pub machine_display_hint_ref: String,
    /// Pane surface rows for every leaf in the split tree.
    pub pane_surfaces: Vec<PaneSurfaceSerialization>,
    /// Linked profile artifact refs consumed as profile defaults.
    pub linked_profile_artifacts: Vec<LinkedProfileArtifactRef>,
    /// Display-topology adjustments known at export time.
    pub topology_adjustments: Vec<TopologyAdjustment>,
    /// Restore provenance refs associated with the package.
    pub restore_provenance_refs: Vec<String>,
}

impl LayoutPortableStateRequest {
    /// Creates a request with required identity fields and no panes.
    pub fn new(
        package_id: impl Into<String>,
        manifest_id: impl Into<String>,
        workspace_ref: impl Into<String>,
        created_at: impl Into<String>,
        producer_ref: impl Into<String>,
        last_written_at: impl Into<String>,
        window_topology_snapshot_ref: impl Into<String>,
        machine_display_hint_ref: impl Into<String>,
    ) -> Self {
        Self {
            package_id: package_id.into(),
            manifest_id: manifest_id.into(),
            workspace_ref: workspace_ref.into(),
            created_at: created_at.into(),
            producer_ref: producer_ref.into(),
            last_written_at: last_written_at.into(),
            window_topology_snapshot_ref: window_topology_snapshot_ref.into(),
            machine_display_hint_ref: machine_display_hint_ref.into(),
            pane_surfaces: Vec::new(),
            linked_profile_artifacts: Vec::new(),
            topology_adjustments: Vec::new(),
            restore_provenance_refs: Vec::new(),
        }
    }
}

/// Builds a portable-state package from a shell split tree and pane metadata.
pub fn build_portable_state_package_from_split_tree(
    split_tree: &SplitTree,
    request: LayoutPortableStateRequest,
) -> Result<PortableStateAlphaPackage, LayoutSerializationError> {
    let panes = pane_postures_from_split_tree(split_tree, &request)?;
    let placeholder_summary = panes
        .iter()
        .filter(|pane| pane.restore_posture != SurfaceRestorePosture::Live)
        .cloned()
        .collect::<Vec<_>>();
    let profile_artifact_refs = request
        .linked_profile_artifacts
        .iter()
        .map(|artifact| artifact.artifact_ref.clone())
        .collect::<Vec<_>>();

    let mut state_classes = vec![
        PortableStateClassRecord {
            class_id: format!("state-class-workspace-authority:{}", request.workspace_ref),
            class_kind: SerializedStateClass::WorkspaceAuthority,
            classification: PersistenceClassification::Shared,
            export_mode: ExportMode::ReferencedBody,
            schema_binding: StateSchemaBinding {
                schema_ref: "schemas/config/workspace_manifest.schema.json".to_string(),
                schema_version: 1,
                artifact_refs: vec![format!("workspace-authority:{}", request.workspace_ref)],
            },
            last_written_at: request.last_written_at.clone(),
            restore_candidate: RestoreCandidateClass::CompatibleRestore,
            export_allowed: true,
            clear_allowed: false,
            pane_restore_postures: Vec::new(),
            linked_profile_artifact_refs: Vec::new(),
            local_only_reason: None,
            notes: "Workspace authority is referenced by opaque ids; live authority remains excluded."
                .to_string(),
        },
        PortableStateClassRecord {
            class_id: format!("state-class-window-topology:{}", request.workspace_ref),
            class_kind: SerializedStateClass::WindowTopology,
            classification: PersistenceClassification::Portable,
            export_mode: ExportMode::CarriedBody,
            schema_binding: StateSchemaBinding {
                schema_ref: PANE_TREE_SCHEMA_REF.to_string(),
                schema_version: 1,
                artifact_refs: vec![request.window_topology_snapshot_ref.clone()],
            },
            last_written_at: request.last_written_at.clone(),
            restore_candidate: RestoreCandidateClass::LayoutOnly,
            export_allowed: true,
            clear_allowed: true,
            pane_restore_postures: panes.clone(),
            linked_profile_artifact_refs: Vec::new(),
            local_only_reason: None,
            notes: "Window topology carries stable pane ids and placeholder metadata.".to_string(),
        },
        PortableStateClassRecord {
            class_id: format!("state-class-profile-defaults:{}", request.workspace_ref),
            class_kind: SerializedStateClass::ProfileDefaults,
            classification: PersistenceClassification::Portable,
            export_mode: ExportMode::LinkedArtifactRef,
            schema_binding: StateSchemaBinding {
                schema_ref: PORTABLE_PROFILE_SCHEMA_REF.to_string(),
                schema_version: 1,
                artifact_refs: profile_artifact_refs.clone(),
            },
            last_written_at: request.last_written_at.clone(),
            restore_candidate: RestoreCandidateClass::CompatibleRestore,
            export_allowed: true,
            clear_allowed: false,
            pane_restore_postures: Vec::new(),
            linked_profile_artifact_refs: profile_artifact_refs,
            local_only_reason: None,
            notes: "Profile defaults remain linked portable-profile artifact refs.".to_string(),
        },
        PortableStateClassRecord {
            class_id: format!("state-class-machine-local-hints:{}", request.workspace_ref),
            class_kind: SerializedStateClass::MachineLocalHints,
            classification: PersistenceClassification::MachineLocal,
            export_mode: ExportMode::MetadataOnly,
            schema_binding: StateSchemaBinding {
                schema_ref: PANE_TREE_SCHEMA_REF.to_string(),
                schema_version: 1,
                artifact_refs: vec![request.machine_display_hint_ref.clone()],
            },
            last_written_at: request.last_written_at.clone(),
            restore_candidate: RestoreCandidateClass::Excluded,
            export_allowed: false,
            clear_allowed: true,
            pane_restore_postures: Vec::new(),
            linked_profile_artifact_refs: Vec::new(),
            local_only_reason: Some(
                "Display geometry, DPI, monitor affinity, fullscreen state, and state roots are machine-local hints."
                    .to_string(),
            ),
            notes: "Machine-local hints are listed for honesty and excluded from portable authority."
                .to_string(),
        },
    ];

    if !placeholder_summary.is_empty() {
        state_classes.push(PortableStateClassRecord {
            class_id: format!("state-class-local-session-context:{}", request.workspace_ref),
            class_kind: SerializedStateClass::LocalSessionContext,
            classification: PersistenceClassification::LocalOnly,
            export_mode: ExportMode::MetadataOnly,
            schema_binding: StateSchemaBinding {
                schema_ref: "schemas/state/portable_state_package.schema.json".to_string(),
                schema_version: 1,
                artifact_refs: vec![format!("local-session-context:{}", request.workspace_ref)],
            },
            last_written_at: request.last_written_at.clone(),
            restore_candidate: RestoreCandidateClass::LayoutOnly,
            export_allowed: false,
            clear_allowed: true,
            pane_restore_postures: placeholder_summary.clone(),
            linked_profile_artifact_refs: Vec::new(),
            local_only_reason: Some(
                "Live session context is local-only metadata and cannot export handles or rerun work."
                    .to_string(),
            ),
            notes: "Context-only panes can be inspected or cleared without deleting workspace content."
                .to_string(),
        });
    }

    let package = PortableStateAlphaPackage {
        record_kind: PortableStateAlphaRecordKind::PortableStateAlphaPackage,
        schema_version: PORTABLE_STATE_ALPHA_SCHEMA_VERSION,
        package_id: request.package_id,
        manifest_id: request.manifest_id,
        workspace_ref: request.workspace_ref.clone(),
        created_at: request.created_at,
        producer_ref: request.producer_ref,
        state_classes,
        linked_profile_artifacts: request.linked_profile_artifacts,
        redaction_manifest: default_redaction_manifest(&request.workspace_ref),
        machine_local_exclusions: default_machine_local_exclusions(
            &request.workspace_ref,
            &request.machine_display_hint_ref,
        ),
        restore_provenance: PortableStateRestoreProvenance {
            source_snapshot_refs: vec![request.window_topology_snapshot_ref],
            restore_provenance_refs: request.restore_provenance_refs,
            topology_adjustments: request.topology_adjustments,
            placeholder_summary,
            notes: "Restore provenance separates live, context-only, placeholder-only, and excluded classes."
                .to_string(),
        },
        actions: default_package_actions(&request.workspace_ref),
        notes: "Workspace portable-state package generated from shell split-tree serialization."
            .to_string(),
    };
    package
        .validate()
        .map_err(LayoutSerializationError::InvalidPackage)?;
    Ok(package)
}

/// Builds a default linked layout-preset artifact ref for tests and seeds.
pub fn default_layout_preset_artifact(workspace_ref: &str) -> LinkedProfileArtifactRef {
    LinkedProfileArtifactRef {
        artifact_ref: format!("profile-layout-preset:{workspace_ref}"),
        artifact_class: PortableArtifactClass::LayoutPreset,
        portability_label: ArtifactPortabilityLabel::Portable,
        schema_ref: PORTABLE_PROFILE_SCHEMA_REF.to_string(),
        source_ref: format!("portable-profile:{workspace_ref}"),
        notes: "Layout defaults travel through the portable-profile artifact contract.".to_string(),
    }
}

/// Remembered-state inspector projection rendered by shell surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RememberedStateInspectorPanel {
    inspector: RememberedStateInspector,
}

impl RememberedStateInspectorPanel {
    /// Builds the inspector panel from a validated portable-state package.
    pub fn from_package(
        package: &PortableStateAlphaPackage,
    ) -> Result<Self, PortableStateAlphaValidationError> {
        Ok(Self {
            inspector: package.inspector()?,
        })
    }

    /// Returns the inspector model.
    pub const fn inspector(&self) -> &RememberedStateInspector {
        &self.inspector
    }

    /// Renders support-safe lines for native or CLI surfaces.
    pub fn render_lines(&self) -> Vec<String> {
        self.inspector
            .render_plaintext()
            .lines()
            .map(str::to_owned)
            .collect()
    }
}

fn pane_postures_from_split_tree(
    split_tree: &SplitTree,
    request: &LayoutPortableStateRequest,
) -> Result<Vec<PaneRestorePosture>, LayoutSerializationError> {
    let mut by_id = BTreeMap::new();
    for surface in &request.pane_surfaces {
        by_id.insert(surface.pane_id.value(), surface);
    }

    let mut panes = Vec::new();
    for pane_id in split_tree.leaf_ids_in_order() {
        let surface =
            by_id
                .get(&pane_id.value())
                .ok_or(LayoutSerializationError::MissingPaneSurface {
                    pane_id: pane_id.value(),
                })?;
        panes.push(PaneRestorePosture {
            stable_pane_id: stable_pane_ref(pane_id),
            surface_role: surface.surface_role.clone(),
            surface_class: surface.surface_class.clone(),
            restore_posture: surface.restore_posture,
            placeholder_card: surface.placeholder_card.clone(),
            no_rerun_guardrails: surface.no_rerun_guardrails.clone(),
            last_written_at: request.last_written_at.clone(),
        });
    }
    Ok(panes)
}

fn stable_pane_ref(pane_id: PaneId) -> String {
    format!("pane-{:04}", pane_id.value())
}

fn default_redaction_manifest(workspace_ref: &str) -> RedactionManifest {
    RedactionManifest {
        manifest_id: format!("redaction-manifest:{workspace_ref}"),
        rules: vec![
            RedactionRuleClass::RawSecretMaterialExcluded,
            RedactionRuleClass::ApprovalTicketExcluded,
            RedactionRuleClass::DelegatedCredentialExcluded,
            RedactionRuleClass::LiveAuthorityHandleExcluded,
            RedactionRuleClass::MachineUniqueHandleExcluded,
            RedactionRuleClass::StateRootExcluded,
            RedactionRuleClass::RawPathExcluded,
            RedactionRuleClass::RawHostExcluded,
            RedactionRuleClass::RawCommandLineExcluded,
            RedactionRuleClass::RawLogExcluded,
            RedactionRuleClass::RawSourceContentExcluded,
            RedactionRuleClass::ProviderPayloadExcluded,
        ],
        machine_local_exclusions_reviewed: true,
        notes: "Secrets, approval tickets, delegated credentials, live handles, state roots, raw paths, hosts, commands, logs, source content, and provider payloads are excluded."
            .to_string(),
    }
}

fn default_machine_local_exclusions(
    workspace_ref: &str,
    machine_display_hint_ref: &str,
) -> Vec<MachineLocalExclusion> {
    vec![
        MachineLocalExclusion {
            exclusion_id: format!("exclusion-live-handles:{workspace_ref}"),
            class_kind: SerializedStateClass::WorkspaceAuthority,
            artifact_ref: format!("live-authority-handles:{workspace_ref}"),
            reason: MachineLocalExclusionReason::ContainsLiveHandle,
            substitute_class: ExclusionSubstituteClass::OpaqueRef,
            notes: "Live authority handles are excluded and represented only by opaque refs."
                .to_string(),
        },
        MachineLocalExclusion {
            exclusion_id: format!("exclusion-display-hints:{workspace_ref}"),
            class_kind: SerializedStateClass::MachineLocalHints,
            artifact_ref: machine_display_hint_ref.to_string(),
            reason: MachineLocalExclusionReason::DisplayHintBestEffortOnly,
            substitute_class: ExclusionSubstituteClass::MetadataOnly,
            notes: "Display hints remain best-effort metadata on the source machine.".to_string(),
        },
        MachineLocalExclusion {
            exclusion_id: format!("exclusion-state-roots:{workspace_ref}"),
            class_kind: SerializedStateClass::MachineLocalHints,
            artifact_ref: format!("state-roots:{workspace_ref}"),
            reason: MachineLocalExclusionReason::StateRootOnly,
            substitute_class: ExclusionSubstituteClass::Omitted,
            notes: "Concrete state roots do not cross the package boundary.".to_string(),
        },
        MachineLocalExclusion {
            exclusion_id: format!("exclusion-credentials:{workspace_ref}"),
            class_kind: SerializedStateClass::WorkspaceAuthority,
            artifact_ref: format!("credential-store:{workspace_ref}"),
            reason: MachineLocalExclusionReason::CredentialStoreOnly,
            substitute_class: ExclusionSubstituteClass::Omitted,
            notes: "Credential and approval material remains in protected stores.".to_string(),
        },
    ]
}

fn default_package_actions(workspace_ref: &str) -> Vec<RememberedStateAction> {
    vec![
        RememberedStateAction {
            action: RememberedStateActionKind::Inspect,
            enabled: true,
            target_ref: Some(format!("portable-state-inspect:{workspace_ref}")),
            disabled_reason: None,
        },
        RememberedStateAction {
            action: RememberedStateActionKind::Export,
            enabled: true,
            target_ref: Some(format!("portable-state-export:{workspace_ref}")),
            disabled_reason: None,
        },
        RememberedStateAction {
            action: RememberedStateActionKind::Compare,
            enabled: true,
            target_ref: Some(format!("portable-state-compare:{workspace_ref}")),
            disabled_reason: None,
        },
        RememberedStateAction {
            action: RememberedStateActionKind::Clear,
            enabled: true,
            target_ref: Some(format!("portable-state-clear:{workspace_ref}")),
            disabled_reason: None,
        },
    ]
}

/// Layout serialization failures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutSerializationError {
    /// A split-tree leaf has no surface metadata.
    MissingPaneSurface {
        /// Numeric pane id from the split tree.
        pane_id: u64,
    },
    /// The generated package violates the workspace package contract.
    InvalidPackage(PortableStateAlphaValidationError),
}

impl fmt::Display for LayoutSerializationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingPaneSurface { pane_id } => {
                write!(f, "missing surface metadata for pane {pane_id}")
            }
            Self::InvalidPackage(err) => write!(f, "invalid portable-state package: {err}"),
        }
    }
}

impl std::error::Error for LayoutSerializationError {}

/// Builds a display-topology adjustment row for tests and shell restore logs.
pub fn display_topology_adjustment(
    adjustment_id: impl Into<String>,
    adjustment_class: DisplayAdjustmentClass,
    affected_pane_ids: Vec<String>,
    restore_fidelity_after_adjustment: RestoreCandidateClass,
    notes: impl Into<String>,
) -> TopologyAdjustment {
    TopologyAdjustment {
        adjustment_id: adjustment_id.into(),
        adjustment_class,
        display_topology_changed: true,
        affected_pane_ids,
        visible_bounds_verified: true,
        restore_fidelity_after_adjustment,
        notes: notes.into(),
    }
}
