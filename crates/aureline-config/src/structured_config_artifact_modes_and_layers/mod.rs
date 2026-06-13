//! Canonical artifact-header, mode-switch, and environment-layer truth for
//! config-bearing surfaces.
//!
//! This packet deepens the higher-level family matrix with the concrete fields
//! that shell, CLI inspect, docs/help, and support export must share:
//!
//! - artifact headers disclose identity, class, canonical-source note, target
//!   context, validator state, and active mode;
//! - source/effective/live switches disclose resolution time, target boundary,
//!   write eligibility, and stale/unresolved/deferred state; and
//! - environment-bearing families expose in-IDE layer stacks with order, source
//!   class, tracked/ignored state, policy lock, secret-bearing note, and
//!   reset/open-source actions.
//!
//! The packet is metadata-only and reuses the family/source vocabulary already
//! frozen by [`crate::structured_config_policy_bundle_and_entitlement_matrix`].

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::structured_config_policy_bundle_and_entitlement_matrix::{
    ArtifactFamilyKind, QualificationLabel, SourceLayerClass,
    STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SHARED_CONTRACT_REF,
};

#[cfg(test)]
mod tests;

/// Stable record-kind tag for the artifact-mode packet.
pub const STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_RECORD_KIND: &str =
    "structured_config_artifact_modes_and_layers";

/// Schema version for [`StructuredConfigArtifactModesAndLayersPacket`].
pub const STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref every consuming surface must quote.
pub const STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF: &str =
    "config:structured_config_artifact_modes_and_layers:v1";

/// Repo-relative path to the checked-in canonical packet.
pub const STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_PATH: &str =
    "artifacts/config/structured_config_artifact_modes_and_layers.json";

/// Reviewer-facing notice repeated on UI, CLI, docs/help, and support surfaces.
pub const STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_NOTICE: &str =
    "Config-bearing artifact surfaces keep headers, mode switches, and layer stacks honest: \
     every header names the source object, artifact class, canonical writable source note, target \
     context, validator state, and active mode; every source/effective/live switch states when it \
     was resolved, which boundary it targets, whether it is writable or inspect-only, and whether \
     it is current, stale, unresolved, deferred, or unsupported; environment-bearing artifacts \
     keep layer order, source class, tracked or ignored state, policy locks, secret-bearing notes, \
     and reset/open-source actions visible without leaving the IDE; and shell, CLI inspect, docs/help, \
     and support export reuse the same mode and layer vocabulary instead of inventing surface-local copy.";

/// Header field that every consumer surface must expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeaderFieldClass {
    /// File or artifact identity.
    FileIdentity,
    /// Artifact class label.
    ArtifactClass,
    /// Canonical writable source note.
    CanonicalSourceNote,
    /// Visible target context label.
    TargetContext,
    /// Schema or validator state.
    ValidatorState,
    /// Active source/effective/live mode.
    ActiveMode,
}

impl HeaderFieldClass {
    /// Required header fields shared by every surface.
    pub const ALL: [Self; 6] = [
        Self::FileIdentity,
        Self::ArtifactClass,
        Self::CanonicalSourceNote,
        Self::TargetContext,
        Self::ValidatorState,
        Self::ActiveMode,
    ];
}

/// Shared mode-switch label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModeClass {
    /// Canonical authored source.
    Source,
    /// Effective or resolved projection.
    Effective,
    /// Live or observed state.
    Live,
}

impl ModeClass {
    /// Required mode labels reused across surfaces.
    pub const ALL: [Self; 3] = [Self::Source, Self::Effective, Self::Live];
}

/// Header validator state shown next to the artifact identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValidatorStateClass {
    /// Schema and validator state are current.
    Current,
    /// Validation passed but a warning remains visible.
    Warning,
    /// Validation is stale and must be refreshed before trust widens.
    Stale,
    /// Validation could not resolve the current target.
    Unresolved,
    /// Surface is intentionally narrower because it depends on preview depth.
    PreviewNarrowed,
    /// Policy narrowed or pinned the visible result.
    PolicyLocked,
}

/// Current state of one source/effective/live switch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ModeStateClass {
    /// Current and trusted for its class.
    Current,
    /// Present but stale.
    Stale,
    /// Present but unresolved.
    Unresolved,
    /// Deferred until a later runtime bind or apply.
    Deferred,
    /// Not available for this family.
    Unsupported,
}

/// Boundary the mode row targets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetBoundaryClass {
    /// Local workspace-owned source.
    LocalWorkspace,
    /// Request-workspace runtime boundary.
    RequestRuntime,
    /// Database execution boundary.
    DatabaseRuntime,
    /// API/service execution boundary.
    ApiRuntime,
    /// Notebook kernel or interpreter session boundary.
    KernelSession,
    /// Preview runtime or browser runtime boundary.
    PreviewRuntime,
    /// Workflow bundle execution boundary.
    WorkflowRuntime,
    /// CI mirror or import boundary.
    CiMirror,
    /// Infrastructure mirror or managed observation boundary.
    InfraMirror,
    /// Signed policy cache or review boundary.
    PolicyCache,
    /// Offline entitlement cache or grace snapshot boundary.
    OfflineEntitlementCache,
    /// Trust-root or signer continuity cache boundary.
    TrustRootCache,
}

/// Whether a mode row is writable or inspect-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WriteEligibilityClass {
    /// The row is the canonical writable source.
    WritableCanonicalSource,
    /// Inspectable effective projection only.
    InspectOnlyProjection,
    /// Inspectable live or observed state only.
    InspectOnlyLiveObservation,
    /// Writable truth is deferred until a runtime bind completes.
    DeferredUntilRuntime,
    /// Current target is unresolved, so mutation stays blocked.
    UnresolvedTarget,
    /// Signed bundle may be reviewed locally but not edited as ordinary text.
    SignedBundleReviewOnly,
}

/// Layer-stack vocabulary shared across surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerVocabularyField {
    /// Explicit layer ordering.
    LayerOrder,
    /// Source class for the layer.
    SourceClass,
    /// Tracked or ignored state.
    TrackedState,
    /// Policy lock state.
    PolicyLock,
    /// Secret-bearing note.
    SecretBearingNote,
    /// Layer-specific reset action.
    ResetAction,
    /// Open-source action.
    OpenSourceAction,
}

impl LayerVocabularyField {
    /// Required layer terms reused across surfaces.
    pub const ALL: [Self; 7] = [
        Self::LayerOrder,
        Self::SourceClass,
        Self::TrackedState,
        Self::PolicyLock,
        Self::SecretBearingNote,
        Self::ResetAction,
        Self::OpenSourceAction,
    ];
}

/// Lifecycle-sensitive dependency marker shown on a config-bearing row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleMarkerClass {
    /// The row depends on a Labs-only capability.
    LabsDependency,
    /// The row depends on a Preview-only capability.
    PreviewDependency,
    /// Signed policy or entitlement gates the visible behavior.
    PolicyGatedDependency,
    /// The target does not support the claimed capability depth.
    UnsupportedDependency,
    /// The row is still narrowed by stale experiment ownership or rollout state.
    StaleExperiment,
    /// An emergency disable or kill-switch window is expired and still active.
    ExpiredKillSwitch,
}

/// Hidden-flag spill verdict for a stable-facing config row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HiddenFlagSpillVerdict {
    /// Stable-facing truth is clean; no hidden lifecycle state remains.
    ClearStableSurface,
    /// Narrowing is present and explicitly disclosed on the row.
    DisclosedNarrowing,
    /// Stable-facing treatment is blocked until the hidden dependency is repaired.
    BlockedStableFacingRow,
}

/// Typed mutation flow that must stay scope-explicit and checkpointed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationFlowClass {
    /// Reset only the selected layer or override.
    ResetLayer,
    /// Repair a drifted or stale resolved value.
    RepairConfig,
    /// Import or re-apply a bundle/profile fragment.
    ImportBundle,
    /// Migrate or rotate the authored artifact.
    MigrationApply,
}

/// Shared consuming surface for the vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurfaceClass {
    /// Desktop shell/editor surface.
    DesktopShell,
    /// CLI or headless inspect output.
    CliInspect,
    /// Support export or support packet.
    SupportExport,
    /// Docs/help surface.
    DocsHelp,
}

impl ConsumerSurfaceClass {
    /// Required consumers for the shared vocabulary.
    pub const ALL: [Self; 4] = [
        Self::DesktopShell,
        Self::CliInspect,
        Self::SupportExport,
        Self::DocsHelp,
    ];
}

/// How a layer participates in the visible stack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerTrackedState {
    /// The layer is tracked and editable or resettable.
    Tracked,
    /// The layer exists but is intentionally ignored.
    Ignored,
    /// The layer is derived and read-only.
    DerivedReadOnly,
}

/// Policy posture visible on one layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayerPolicyLockState {
    /// No policy lock is active on this layer.
    None,
    /// Policy narrows the layer.
    Narrowed,
    /// Policy fully locks the layer.
    Locked,
}

/// One shared header-field definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HeaderFieldDefinition {
    /// Stable field token.
    pub field: HeaderFieldClass,
    /// Short human label.
    pub label: String,
    /// Reviewer-facing explanation.
    pub description: String,
}

/// One shared mode vocabulary definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeVocabularyRow {
    /// Stable mode token.
    pub mode: ModeClass,
    /// Short human label.
    pub label: String,
    /// Reviewer-facing explanation.
    pub description: String,
    /// Whether this mode is ever treated as canonical writable source.
    pub canonical_writable_truth: bool,
}

/// One shared layer-vocabulary definition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerVocabularyRow {
    /// Stable layer-vocabulary token.
    pub field: LayerVocabularyField,
    /// Short human label.
    pub label: String,
    /// Reviewer-facing explanation.
    pub description: String,
}

/// Surface-level binding proving vocabulary reuse.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceVocabularyBinding {
    /// Consumer surface.
    pub surface: ConsumerSurfaceClass,
    /// Contract ref the surface quotes.
    pub shared_contract_ref: String,
    /// Header fields exposed by the surface.
    pub header_fields: Vec<HeaderFieldClass>,
    /// Mode labels exposed by the surface.
    pub mode_labels: Vec<ModeClass>,
    /// Layer-stack fields exposed by the surface.
    pub layer_fields: Vec<LayerVocabularyField>,
}

/// Artifact header shown above the source/effective/live switch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactHeader {
    /// Visible file or artifact identity.
    pub identity_label: String,
    /// Opaque safe ref for the identity.
    pub identity_ref: String,
    /// Human-readable artifact class label.
    pub artifact_class_label: String,
    /// Explicit note that names the canonical writable source.
    pub canonical_source_note: String,
    /// Visible target context chip or summary.
    pub target_context_label: String,
    /// Schema or validator state token.
    pub validator_state: ValidatorStateClass,
    /// Exact validator-state summary shown to the user.
    pub validator_summary: String,
    /// Mode currently selected in the surface.
    pub active_mode: ModeClass,
}

/// One source/effective/live switch row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModeSwitchRow {
    /// Stable mode token.
    pub mode: ModeClass,
    /// Whether this mode is currently active.
    pub active: bool,
    /// Resolution time or freshness label shown in the switch body.
    pub resolution_time_label: String,
    /// Boundary the mode resolves against.
    pub target_boundary: TargetBoundaryClass,
    /// Whether the mode is writable or inspect-only.
    pub write_eligibility: WriteEligibilityClass,
    /// State of the current view.
    pub state: ModeStateClass,
    /// Reviewer-facing one-line summary.
    pub summary: String,
}

/// One layer-stack action visible in the IDE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LayerActionRow {
    /// Whether the action is currently available.
    pub available: bool,
    /// Action label shown to the user.
    pub action_label: String,
    /// Stable action ref or command id.
    pub action_ref: String,
}

/// One environment-layer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentLayerRow {
    /// Stable order in the visible stack.
    pub layer_order: u8,
    /// Human-readable layer label.
    pub layer_label: String,
    /// Stable source class for the layer.
    pub source_class: SourceLayerClass,
    /// Whether the layer is tracked, ignored, or derived read-only.
    pub tracked_state: LayerTrackedState,
    /// Visible policy posture on the layer.
    pub policy_lock: LayerPolicyLockState,
    /// Secret-bearing disclosure shown in the row.
    pub secret_bearing_note: String,
    /// Whether this layer currently wins for the visible key set.
    pub wins_effective_value: bool,
    /// Layer-specific reset action.
    pub reset_action: LayerActionRow,
    /// Open-source action for the layer.
    pub open_source_action: LayerActionRow,
}

/// Stack of environment-bearing layers visible without leaving the IDE.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentLayerStack {
    /// Whether the stack is visible in the current surface without navigation away.
    pub visible_without_leaving_ide: bool,
    /// Summary line shown above the stack.
    pub summary: String,
    /// Ordered layers.
    pub layers: Vec<EnvironmentLayerRow>,
}

/// One visible lifecycle-sensitive dependency on an artifact row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LifecycleDependencyMarker {
    /// Marker classification.
    pub marker_class: LifecycleMarkerClass,
    /// Human-readable capability or rollout label.
    pub dependency_label: String,
    /// Stable ref for the dependency or rollout entry.
    pub dependency_ref: String,
    /// Lifecycle state the dependency still requires.
    pub required_lifecycle_label: String,
    /// Exact effect on the current row.
    pub effect_summary: String,
    /// Fallback or repair route the user can take.
    pub fallback_path: String,
    /// Whether the marker is rendered in-product.
    pub visible: bool,
}

/// Guard proving stable-facing rows do not silently depend on hidden flags.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HiddenFlagSpillGuard {
    /// Guard verdict for the current row.
    pub verdict: HiddenFlagSpillVerdict,
    /// Stable-facing surface or row label being protected.
    pub stable_facing_surface_label: String,
    /// Whether hidden lifecycle state was detected on the row.
    pub hidden_dependency_detected: bool,
    /// Stale experiment or rollout ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_experiment_ref: Option<String>,
    /// Expired kill-switch ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expired_kill_switch_ref: Option<String>,
    /// Reviewer-facing explanation for the verdict.
    pub review_summary: String,
}

/// Scope-explicit mutation preview and checkpoint row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeExplicitMutationFlow {
    /// Typed reset/repair/import/migration class.
    pub flow_class: MutationFlowClass,
    /// Exact scope the flow would touch.
    pub scope_label: String,
    /// Opaque ref to the preview sheet for this flow.
    pub preview_ref: String,
    /// Named layers or source fragments that would change.
    pub affected_layer_labels: Vec<String>,
    /// Named bundles or signed authorities that would change.
    pub affected_bundle_refs: Vec<String>,
    /// Rollback checkpoint created before apply, when the flow may mutate.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rollback_checkpoint_ref: Option<String>,
    /// Denial reason when policy blocks the flow before apply.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_denied_reason: Option<String>,
}

/// One artifact family row in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactSurfaceRow {
    /// Stable family token shared with the matrix contract.
    pub family: ArtifactFamilyKind,
    /// Existing qualification label inherited from the higher-level family matrix.
    pub qualification_label: QualificationLabel,
    /// Opaque ref safe for support exports.
    pub family_ref: String,
    /// Checked-in packet or primary evidence this row points at.
    pub source_packet_ref: String,
    /// Additional docs or schema evidence refs.
    pub evidence_refs: Vec<String>,
    /// Visible artifact header.
    pub header: ArtifactHeader,
    /// Mode-switch rows for source/effective/live.
    pub mode_switches: Vec<ModeSwitchRow>,
    /// Whether this family must carry a layer stack.
    pub environment_stack_required: bool,
    /// Layer stack when the family is environment-bearing.
    pub environment_layer_stack: Option<EnvironmentLayerStack>,
    /// Visible lifecycle-sensitive dependency markers for the row.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lifecycle_dependency_markers: Vec<LifecycleDependencyMarker>,
    /// Guard proving hidden feature-flag spill is disclosed or blocked.
    pub hidden_flag_spill_guard: HiddenFlagSpillGuard,
    /// Scope-explicit reset/repair/import/migration flows for the row.
    pub mutation_scope_flows: Vec<ScopeExplicitMutationFlow>,
}

/// Derived packet summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketSummary {
    /// Number of artifact-family rows.
    pub artifact_surface_count: usize,
    /// Number of environment-bearing families.
    pub environment_stack_count: usize,
    /// Number of shared consumer surfaces.
    pub shared_surface_count: usize,
    /// Count of rows currently showing source mode.
    pub active_source_count: usize,
    /// Count of rows currently showing effective mode.
    pub active_effective_count: usize,
    /// Count of rows currently showing live mode.
    pub active_live_count: usize,
    /// Whether every header discloses the required fields.
    pub all_headers_complete: bool,
    /// Whether effective and live rows stay non-writable.
    pub effective_and_live_non_writable: bool,
    /// Whether every environment-bearing family exposes layer actions in-IDE.
    pub in_ide_layer_actions_available: bool,
    /// Whether all consuming surfaces reuse the shared vocabulary.
    pub shared_surface_vocabulary_consistent: bool,
    /// Count of visible lifecycle dependency markers.
    pub lifecycle_dependency_marker_count: usize,
    /// Count of guarded rows with disclosed or blocked hidden-flag state.
    pub hidden_flag_guarded_family_count: usize,
    /// Count of scope-explicit mutation flows.
    pub mutation_scope_flow_count: usize,
    /// Count of mutation flows blocked by policy before apply.
    pub policy_denied_mutation_flow_count: usize,
}

/// Canonical packet shared by shell, CLI inspect, docs/help, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredConfigArtifactModesAndLayersPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Upstream family-matrix contract ref.
    pub upstream_family_matrix_ref: String,
    /// Shared header definitions.
    pub header_vocabulary: Vec<HeaderFieldDefinition>,
    /// Shared mode definitions.
    pub mode_vocabulary: Vec<ModeVocabularyRow>,
    /// Shared layer definitions.
    pub layer_vocabulary: Vec<LayerVocabularyRow>,
    /// Surface bindings proving vocabulary reuse.
    pub surface_vocabulary: Vec<SurfaceVocabularyBinding>,
    /// Artifact rows.
    pub artifact_surfaces: Vec<ArtifactSurfaceRow>,
    /// Derived packet summary.
    pub summary: PacketSummary,
    /// Companion docs ref.
    pub docs_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
}

/// Validation failures for the packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketValidationError {
    /// A required family row is missing.
    MissingFamily(ArtifactFamilyKind),
    /// A family row appears more than once.
    DuplicateFamily(ArtifactFamilyKind),
    /// One or more required header fields are missing in a row.
    MissingHeaderField {
        /// Family that failed validation.
        family: ArtifactFamilyKind,
        /// Missing field.
        field: HeaderFieldClass,
    },
    /// The active mode in the header does not match the active switch row.
    HeaderActiveModeMismatch(ArtifactFamilyKind),
    /// A required mode row is missing.
    MissingMode {
        /// Family that failed validation.
        family: ArtifactFamilyKind,
        /// Missing mode.
        mode: ModeClass,
    },
    /// A mode row appears more than once.
    DuplicateMode {
        /// Family that failed validation.
        family: ArtifactFamilyKind,
        /// Duplicated mode.
        mode: ModeClass,
    },
    /// A non-source mode is incorrectly marked as canonical writable source.
    NonSourceModeMarkedWritable {
        /// Family that failed validation.
        family: ArtifactFamilyKind,
        /// Mode that was incorrectly marked writable.
        mode: ModeClass,
    },
    /// An environment-bearing family omitted its stack.
    MissingEnvironmentLayerStack(ArtifactFamilyKind),
    /// A non-environment-bearing family exposed an unnecessary stack.
    UnexpectedEnvironmentLayerStack(ArtifactFamilyKind),
    /// A layer stack does not keep at least one effective winner visible.
    LayerStackMissingWinner(ArtifactFamilyKind),
    /// A layer stack has duplicate order values.
    DuplicateLayerOrder(ArtifactFamilyKind),
    /// A layer omits reset or open-source metadata.
    LayerActionsMissing {
        /// Family that failed validation.
        family: ArtifactFamilyKind,
        /// Offending layer order.
        layer_order: u8,
    },
    /// A row is narrower than stable but hides its lifecycle dependency.
    MissingLifecycleDependencyMarker(ArtifactFamilyKind),
    /// A lifecycle marker is malformed or hidden.
    InvalidLifecycleDependencyMarker {
        /// Family that failed validation.
        family: ArtifactFamilyKind,
        /// Marker label that failed validation.
        dependency_label: String,
    },
    /// A hidden-flag spill guard is inconsistent with the row's evidence.
    InvalidHiddenFlagSpillGuard(ArtifactFamilyKind),
    /// A row omits a required scope-explicit mutation preview.
    MissingMutationScopeFlow(ArtifactFamilyKind),
    /// A scope-explicit mutation flow is malformed.
    InvalidMutationScopeFlow {
        /// Family that failed validation.
        family: ArtifactFamilyKind,
        /// Flow class that failed validation.
        flow_class: MutationFlowClass,
    },
    /// A required surface binding is missing.
    MissingSurfaceVocabulary(ConsumerSurfaceClass),
    /// A surface does not reuse the full shared vocabulary.
    IncompleteSurfaceVocabulary(ConsumerSurfaceClass),
    /// A summary count drifted from the rows.
    SummaryCountMismatch {
        /// Field that drifted.
        field: &'static str,
        /// Expected value.
        expected: usize,
        /// Actual value.
        actual: usize,
    },
    /// A summary boolean drifted from the rows.
    SummaryFlagMismatch {
        /// Field that drifted.
        field: &'static str,
        /// Expected value.
        expected: bool,
        /// Actual value.
        actual: bool,
    },
}

impl fmt::Display for PacketValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFamily(family) => write!(f, "missing family row: {family:?}"),
            Self::DuplicateFamily(family) => write!(f, "duplicate family row: {family:?}"),
            Self::MissingHeaderField { family, field } => {
                write!(f, "family {family:?} is missing header field {field:?}")
            }
            Self::HeaderActiveModeMismatch(family) => {
                write!(
                    f,
                    "family {family:?} header active mode does not match the switch rows"
                )
            }
            Self::MissingMode { family, mode } => {
                write!(f, "family {family:?} is missing mode row {mode:?}")
            }
            Self::DuplicateMode { family, mode } => {
                write!(f, "family {family:?} has duplicate mode row {mode:?}")
            }
            Self::NonSourceModeMarkedWritable { family, mode } => write!(
                f,
                "family {family:?} marks non-source mode {mode:?} as canonical writable source"
            ),
            Self::MissingEnvironmentLayerStack(family) => {
                write!(f, "family {family:?} requires an environment layer stack")
            }
            Self::UnexpectedEnvironmentLayerStack(family) => write!(
                f,
                "family {family:?} is not environment-bearing but exposes a layer stack"
            ),
            Self::LayerStackMissingWinner(family) => {
                write!(
                    f,
                    "family {family:?} layer stack hides the effective winner"
                )
            }
            Self::DuplicateLayerOrder(family) => {
                write!(f, "family {family:?} layer stack reuses a layer order")
            }
            Self::LayerActionsMissing {
                family,
                layer_order,
            } => write!(
                f,
                "family {family:?} layer {layer_order} omits reset/open-source metadata"
            ),
            Self::MissingLifecycleDependencyMarker(family) => {
                write!(
                    f,
                    "family {family:?} hides a required lifecycle dependency marker"
                )
            }
            Self::InvalidLifecycleDependencyMarker {
                family,
                dependency_label,
            } => write!(
                f,
                "family {family:?} has an invalid lifecycle marker for {dependency_label}"
            ),
            Self::InvalidHiddenFlagSpillGuard(family) => {
                write!(
                    f,
                    "family {family:?} has an invalid hidden-flag spill guard"
                )
            }
            Self::MissingMutationScopeFlow(family) => {
                write!(f, "family {family:?} omits scope-explicit mutation flows")
            }
            Self::InvalidMutationScopeFlow { family, flow_class } => write!(
                f,
                "family {family:?} has an invalid scope-explicit mutation flow {flow_class:?}"
            ),
            Self::MissingSurfaceVocabulary(surface) => {
                write!(f, "missing shared surface binding: {surface:?}")
            }
            Self::IncompleteSurfaceVocabulary(surface) => {
                write!(
                    f,
                    "surface {surface:?} does not reuse the full shared vocabulary"
                )
            }
            Self::SummaryCountMismatch {
                field,
                expected,
                actual,
            } => write!(
                f,
                "summary field `{field}` drifted: expected {expected}, found {actual}"
            ),
            Self::SummaryFlagMismatch {
                field,
                expected,
                actual,
            } => write!(
                f,
                "summary flag `{field}` drifted: expected {expected}, found {actual}"
            ),
        }
    }
}

impl std::error::Error for PacketValidationError {}

impl StructuredConfigArtifactModesAndLayersPacket {
    /// Returns compact support-export lines.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("packet_id: {}", self.packet_id),
            format!(
                "artifact_surface_count: {}",
                self.summary.artifact_surface_count
            ),
            format!(
                "environment_stack_count: {}",
                self.summary.environment_stack_count
            ),
            format!(
                "active_mode_counts: source={} effective={} live={}",
                self.summary.active_source_count,
                self.summary.active_effective_count,
                self.summary.active_live_count
            ),
            format!(
                "all_headers_complete: {}",
                self.summary.all_headers_complete
            ),
            format!(
                "effective_and_live_non_writable: {}",
                self.summary.effective_and_live_non_writable
            ),
            format!(
                "shared_surface_vocabulary_consistent: {}",
                self.summary.shared_surface_vocabulary_consistent
            ),
            format!(
                "lifecycle_dependency_marker_count: {}",
                self.summary.lifecycle_dependency_marker_count
            ),
            format!(
                "hidden_flag_guarded_family_count: {}",
                self.summary.hidden_flag_guarded_family_count
            ),
            format!(
                "mutation_scope_flow_count: {}",
                self.summary.mutation_scope_flow_count
            ),
            format!(
                "policy_denied_mutation_flow_count: {}",
                self.summary.policy_denied_mutation_flow_count
            ),
        ]
    }
}

/// Returns the deterministic canonical packet.
pub fn seeded_structured_config_artifact_modes_and_layers(
) -> StructuredConfigArtifactModesAndLayersPacket {
    let header_vocabulary = seeded_header_vocabulary();
    let mode_vocabulary = seeded_mode_vocabulary();
    let layer_vocabulary = seeded_layer_vocabulary();
    let surface_vocabulary = seeded_surface_vocabulary();
    let artifact_surfaces = seeded_artifact_surfaces();
    let summary = derive_summary(&surface_vocabulary, &artifact_surfaces);

    StructuredConfigArtifactModesAndLayersPacket {
        record_kind: STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_RECORD_KIND.to_owned(),
        schema_version: STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SCHEMA_VERSION,
        packet_id: "config:structured-config-artifact-modes-and-layers".to_owned(),
        shared_contract_ref: STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF
            .to_owned(),
        notice: STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_NOTICE.to_owned(),
        upstream_family_matrix_ref: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SHARED_CONTRACT_REF
            .to_owned(),
        header_vocabulary,
        mode_vocabulary,
        layer_vocabulary,
        surface_vocabulary,
        artifact_surfaces,
        summary,
        docs_ref: "docs/config/structured_config_artifact_modes_and_layers.md".to_owned(),
        schema_ref: "schemas/config/structured_config_artifact_modes_and_layers.schema.json"
            .to_owned(),
    }
}

/// Parses a packet from JSON text.
pub fn parse_structured_config_artifact_modes_and_layers(
    json: &str,
) -> Result<StructuredConfigArtifactModesAndLayersPacket, serde_json::Error> {
    serde_json::from_str(json)
}

/// Audits the packet and returns every defect found.
pub fn audit_structured_config_artifact_modes_and_layers(
    packet: &StructuredConfigArtifactModesAndLayersPacket,
) -> Vec<PacketValidationError> {
    let mut defects = Vec::new();

    append_presence_defects(
        &mut defects,
        &packet.artifact_surfaces,
        ArtifactFamilyKind::ALL.as_slice(),
        |row| row.family,
        PacketValidationError::MissingFamily,
        PacketValidationError::DuplicateFamily,
    );
    append_surface_presence_defects(&mut defects, &packet.surface_vocabulary);

    for row in &packet.artifact_surfaces {
        if row.header.identity_label.trim().is_empty() {
            defects.push(PacketValidationError::MissingHeaderField {
                family: row.family,
                field: HeaderFieldClass::FileIdentity,
            });
        }
        if row.header.artifact_class_label.trim().is_empty() {
            defects.push(PacketValidationError::MissingHeaderField {
                family: row.family,
                field: HeaderFieldClass::ArtifactClass,
            });
        }
        if row.header.canonical_source_note.trim().is_empty() {
            defects.push(PacketValidationError::MissingHeaderField {
                family: row.family,
                field: HeaderFieldClass::CanonicalSourceNote,
            });
        }
        if row.header.target_context_label.trim().is_empty() {
            defects.push(PacketValidationError::MissingHeaderField {
                family: row.family,
                field: HeaderFieldClass::TargetContext,
            });
        }
        if row.header.validator_summary.trim().is_empty() {
            defects.push(PacketValidationError::MissingHeaderField {
                family: row.family,
                field: HeaderFieldClass::ValidatorState,
            });
        }

        let mut seen_modes = BTreeSet::new();
        let mut active_modes = Vec::new();
        for mode_row in &row.mode_switches {
            if !seen_modes.insert(mode_row.mode) {
                defects.push(PacketValidationError::DuplicateMode {
                    family: row.family,
                    mode: mode_row.mode,
                });
            }
            if mode_row.active {
                active_modes.push(mode_row.mode);
            }
            if mode_row.resolution_time_label.trim().is_empty()
                || mode_row.summary.trim().is_empty()
            {
                defects.push(PacketValidationError::MissingHeaderField {
                    family: row.family,
                    field: HeaderFieldClass::ActiveMode,
                });
            }
            if mode_row.mode != ModeClass::Source
                && mode_row.write_eligibility == WriteEligibilityClass::WritableCanonicalSource
            {
                defects.push(PacketValidationError::NonSourceModeMarkedWritable {
                    family: row.family,
                    mode: mode_row.mode,
                });
            }
        }
        for mode in ModeClass::ALL {
            if !seen_modes.contains(&mode) {
                defects.push(PacketValidationError::MissingMode {
                    family: row.family,
                    mode,
                });
            }
        }
        if active_modes.len() != 1 || active_modes[0] != row.header.active_mode {
            defects.push(PacketValidationError::HeaderActiveModeMismatch(row.family));
        }

        let requires_stack = family_requires_environment_stack(row.family);
        match (requires_stack, row.environment_layer_stack.as_ref()) {
            (true, None) => defects.push(PacketValidationError::MissingEnvironmentLayerStack(
                row.family,
            )),
            (false, Some(_)) => defects.push(
                PacketValidationError::UnexpectedEnvironmentLayerStack(row.family),
            ),
            (true, Some(stack)) => {
                let mut orders = BTreeSet::new();
                let mut any_reset_available = false;
                let mut any_open_available = false;
                let mut any_winner = false;
                if !stack.visible_without_leaving_ide || stack.summary.trim().is_empty() {
                    defects.push(PacketValidationError::MissingEnvironmentLayerStack(
                        row.family,
                    ));
                }
                for layer in &stack.layers {
                    if !orders.insert(layer.layer_order) {
                        defects.push(PacketValidationError::DuplicateLayerOrder(row.family));
                    }
                    if layer.layer_label.trim().is_empty()
                        || layer.secret_bearing_note.trim().is_empty()
                        || layer.reset_action.action_label.trim().is_empty()
                        || layer.reset_action.action_ref.trim().is_empty()
                        || layer.open_source_action.action_label.trim().is_empty()
                        || layer.open_source_action.action_ref.trim().is_empty()
                    {
                        defects.push(PacketValidationError::LayerActionsMissing {
                            family: row.family,
                            layer_order: layer.layer_order,
                        });
                    }
                    any_reset_available |= layer.reset_action.available;
                    any_open_available |= layer.open_source_action.available;
                    any_winner |= layer.wins_effective_value;
                }
                if !any_winner {
                    defects.push(PacketValidationError::LayerStackMissingWinner(row.family));
                }
                if !any_reset_available || !any_open_available {
                    defects.push(PacketValidationError::MissingEnvironmentLayerStack(
                        row.family,
                    ));
                }
            }
            (false, None) => {}
        }

        let has_lifecycle_markers = !row.lifecycle_dependency_markers.is_empty();
        let requires_lifecycle_marker = row.qualification_label != QualificationLabel::Stable
            || row.hidden_flag_spill_guard.verdict != HiddenFlagSpillVerdict::ClearStableSurface;
        if requires_lifecycle_marker && !has_lifecycle_markers {
            defects.push(PacketValidationError::MissingLifecycleDependencyMarker(
                row.family,
            ));
        }
        for marker in &row.lifecycle_dependency_markers {
            if marker.dependency_label.trim().is_empty()
                || marker.dependency_ref.trim().is_empty()
                || marker.required_lifecycle_label.trim().is_empty()
                || marker.effect_summary.trim().is_empty()
                || marker.fallback_path.trim().is_empty()
                || !marker.visible
            {
                defects.push(PacketValidationError::InvalidLifecycleDependencyMarker {
                    family: row.family,
                    dependency_label: marker.dependency_label.clone(),
                });
            }
        }

        if row
            .hidden_flag_spill_guard
            .stable_facing_surface_label
            .trim()
            .is_empty()
            || row.hidden_flag_spill_guard.review_summary.trim().is_empty()
        {
            defects.push(PacketValidationError::InvalidHiddenFlagSpillGuard(
                row.family,
            ));
        }
        if row.hidden_flag_spill_guard.verdict == HiddenFlagSpillVerdict::ClearStableSurface
            && row.hidden_flag_spill_guard.hidden_dependency_detected
        {
            defects.push(PacketValidationError::InvalidHiddenFlagSpillGuard(
                row.family,
            ));
        }
        if row.hidden_flag_spill_guard.verdict != HiddenFlagSpillVerdict::ClearStableSurface
            && !has_lifecycle_markers
        {
            defects.push(PacketValidationError::InvalidHiddenFlagSpillGuard(
                row.family,
            ));
        }
        if row.qualification_label == QualificationLabel::Stable
            && row.lifecycle_dependency_markers.iter().any(|marker| {
                matches!(
                    marker.marker_class,
                    LifecycleMarkerClass::LabsDependency
                        | LifecycleMarkerClass::PreviewDependency
                        | LifecycleMarkerClass::StaleExperiment
                        | LifecycleMarkerClass::ExpiredKillSwitch
                )
            })
            && row.hidden_flag_spill_guard.verdict == HiddenFlagSpillVerdict::ClearStableSurface
        {
            defects.push(PacketValidationError::InvalidHiddenFlagSpillGuard(
                row.family,
            ));
        }

        if row.mutation_scope_flows.is_empty() {
            defects.push(PacketValidationError::MissingMutationScopeFlow(row.family));
        }
        for flow in &row.mutation_scope_flows {
            if flow.scope_label.trim().is_empty()
                || flow.preview_ref.trim().is_empty()
                || (flow.affected_layer_labels.is_empty() && flow.affected_bundle_refs.is_empty())
                || (flow.policy_denied_reason.is_none()
                    && flow
                        .rollback_checkpoint_ref
                        .as_deref()
                        .is_none_or(|value| value.trim().is_empty()))
                || flow
                    .policy_denied_reason
                    .as_deref()
                    .is_some_and(|value| value.trim().is_empty())
            {
                defects.push(PacketValidationError::InvalidMutationScopeFlow {
                    family: row.family,
                    flow_class: flow.flow_class,
                });
            }
        }
    }

    for binding in &packet.surface_vocabulary {
        if binding.shared_contract_ref != packet.shared_contract_ref
            || !covers_all(binding.header_fields.iter().copied(), HeaderFieldClass::ALL)
            || !covers_all(binding.mode_labels.iter().copied(), ModeClass::ALL)
            || !covers_all(
                binding.layer_fields.iter().copied(),
                LayerVocabularyField::ALL,
            )
        {
            defects.push(PacketValidationError::IncompleteSurfaceVocabulary(
                binding.surface,
            ));
        }
    }

    let expected_summary = derive_summary(&packet.surface_vocabulary, &packet.artifact_surfaces);
    compare_summary_count(
        &mut defects,
        "artifact_surface_count",
        expected_summary.artifact_surface_count,
        packet.summary.artifact_surface_count,
    );
    compare_summary_count(
        &mut defects,
        "environment_stack_count",
        expected_summary.environment_stack_count,
        packet.summary.environment_stack_count,
    );
    compare_summary_count(
        &mut defects,
        "shared_surface_count",
        expected_summary.shared_surface_count,
        packet.summary.shared_surface_count,
    );
    compare_summary_count(
        &mut defects,
        "active_source_count",
        expected_summary.active_source_count,
        packet.summary.active_source_count,
    );
    compare_summary_count(
        &mut defects,
        "active_effective_count",
        expected_summary.active_effective_count,
        packet.summary.active_effective_count,
    );
    compare_summary_count(
        &mut defects,
        "active_live_count",
        expected_summary.active_live_count,
        packet.summary.active_live_count,
    );
    compare_summary_flag(
        &mut defects,
        "all_headers_complete",
        expected_summary.all_headers_complete,
        packet.summary.all_headers_complete,
    );
    compare_summary_flag(
        &mut defects,
        "effective_and_live_non_writable",
        expected_summary.effective_and_live_non_writable,
        packet.summary.effective_and_live_non_writable,
    );
    compare_summary_flag(
        &mut defects,
        "in_ide_layer_actions_available",
        expected_summary.in_ide_layer_actions_available,
        packet.summary.in_ide_layer_actions_available,
    );
    compare_summary_flag(
        &mut defects,
        "shared_surface_vocabulary_consistent",
        expected_summary.shared_surface_vocabulary_consistent,
        packet.summary.shared_surface_vocabulary_consistent,
    );
    compare_summary_count(
        &mut defects,
        "lifecycle_dependency_marker_count",
        expected_summary.lifecycle_dependency_marker_count,
        packet.summary.lifecycle_dependency_marker_count,
    );
    compare_summary_count(
        &mut defects,
        "hidden_flag_guarded_family_count",
        expected_summary.hidden_flag_guarded_family_count,
        packet.summary.hidden_flag_guarded_family_count,
    );
    compare_summary_count(
        &mut defects,
        "mutation_scope_flow_count",
        expected_summary.mutation_scope_flow_count,
        packet.summary.mutation_scope_flow_count,
    );
    compare_summary_count(
        &mut defects,
        "policy_denied_mutation_flow_count",
        expected_summary.policy_denied_mutation_flow_count,
        packet.summary.policy_denied_mutation_flow_count,
    );

    defects
}

/// Validates a packet, returning every defect when validation fails.
pub fn validate_structured_config_artifact_modes_and_layers(
    packet: &StructuredConfigArtifactModesAndLayersPacket,
) -> Result<(), Vec<PacketValidationError>> {
    let defects = audit_structured_config_artifact_modes_and_layers(packet);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

fn compare_summary_count(
    defects: &mut Vec<PacketValidationError>,
    field: &'static str,
    expected: usize,
    actual: usize,
) {
    if expected != actual {
        defects.push(PacketValidationError::SummaryCountMismatch {
            field,
            expected,
            actual,
        });
    }
}

fn compare_summary_flag(
    defects: &mut Vec<PacketValidationError>,
    field: &'static str,
    expected: bool,
    actual: bool,
) {
    if expected != actual {
        defects.push(PacketValidationError::SummaryFlagMismatch {
            field,
            expected,
            actual,
        });
    }
}

fn append_presence_defects<T, K>(
    defects: &mut Vec<PacketValidationError>,
    rows: &[T],
    required: &[K],
    key: impl Fn(&T) -> K,
    missing: impl Fn(K) -> PacketValidationError,
    duplicate: impl Fn(K) -> PacketValidationError,
) where
    K: Ord + Copy,
{
    let mut seen = BTreeSet::new();
    for row in rows {
        let current = key(row);
        if !seen.insert(current) {
            defects.push(duplicate(current));
        }
    }
    for required_key in required {
        if !seen.contains(required_key) {
            defects.push(missing(*required_key));
        }
    }
}

fn append_surface_presence_defects(
    defects: &mut Vec<PacketValidationError>,
    rows: &[SurfaceVocabularyBinding],
) {
    let present: BTreeSet<ConsumerSurfaceClass> = rows.iter().map(|row| row.surface).collect();
    for surface in ConsumerSurfaceClass::ALL {
        if !present.contains(&surface) {
            defects.push(PacketValidationError::MissingSurfaceVocabulary(surface));
        }
    }
}

fn derive_summary(
    surface_vocabulary: &[SurfaceVocabularyBinding],
    artifact_surfaces: &[ArtifactSurfaceRow],
) -> PacketSummary {
    PacketSummary {
        artifact_surface_count: artifact_surfaces.len(),
        environment_stack_count: artifact_surfaces
            .iter()
            .filter(|row| row.environment_stack_required)
            .count(),
        shared_surface_count: surface_vocabulary.len(),
        active_source_count: artifact_surfaces
            .iter()
            .filter(|row| row.header.active_mode == ModeClass::Source)
            .count(),
        active_effective_count: artifact_surfaces
            .iter()
            .filter(|row| row.header.active_mode == ModeClass::Effective)
            .count(),
        active_live_count: artifact_surfaces
            .iter()
            .filter(|row| row.header.active_mode == ModeClass::Live)
            .count(),
        all_headers_complete: artifact_surfaces.iter().all(|row| {
            !row.header.identity_label.trim().is_empty()
                && !row.header.identity_ref.trim().is_empty()
                && !row.header.artifact_class_label.trim().is_empty()
                && !row.header.canonical_source_note.trim().is_empty()
                && !row.header.target_context_label.trim().is_empty()
                && !row.header.validator_summary.trim().is_empty()
        }),
        effective_and_live_non_writable: artifact_surfaces.iter().all(|row| {
            row.mode_switches.iter().all(|mode_row| {
                mode_row.mode == ModeClass::Source
                    || mode_row.write_eligibility != WriteEligibilityClass::WritableCanonicalSource
            })
        }),
        in_ide_layer_actions_available: artifact_surfaces
            .iter()
            .filter_map(|row| row.environment_layer_stack.as_ref())
            .all(|stack| {
                stack.visible_without_leaving_ide
                    && stack
                        .layers
                        .iter()
                        .any(|layer| layer.reset_action.available)
                    && stack
                        .layers
                        .iter()
                        .any(|layer| layer.open_source_action.available)
            }),
        shared_surface_vocabulary_consistent: surface_vocabulary.iter().all(|row| {
            row.shared_contract_ref
                == STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF
                && covers_all(row.header_fields.iter().copied(), HeaderFieldClass::ALL)
                && covers_all(row.mode_labels.iter().copied(), ModeClass::ALL)
                && covers_all(row.layer_fields.iter().copied(), LayerVocabularyField::ALL)
        }),
        lifecycle_dependency_marker_count: artifact_surfaces
            .iter()
            .map(|row| row.lifecycle_dependency_markers.len())
            .sum(),
        hidden_flag_guarded_family_count: artifact_surfaces
            .iter()
            .filter(|row| {
                row.hidden_flag_spill_guard.verdict != HiddenFlagSpillVerdict::ClearStableSurface
            })
            .count(),
        mutation_scope_flow_count: artifact_surfaces
            .iter()
            .map(|row| row.mutation_scope_flows.len())
            .sum(),
        policy_denied_mutation_flow_count: artifact_surfaces
            .iter()
            .flat_map(|row| row.mutation_scope_flows.iter())
            .filter(|flow| flow.policy_denied_reason.is_some())
            .count(),
    }
}

fn family_requires_environment_stack(family: ArtifactFamilyKind) -> bool {
    matches!(
        family,
        ArtifactFamilyKind::RequestWorkspaceEnvironment
            | ArtifactFamilyKind::DatabaseProfile
            | ArtifactFamilyKind::ApiProfile
            | ArtifactFamilyKind::NotebookRuntimeManifest
            | ArtifactFamilyKind::PreviewRuntimeConfig
            | ArtifactFamilyKind::WorkflowBundleManifest
            | ArtifactFamilyKind::CiEnvironmentDescriptor
            | ArtifactFamilyKind::InfraEnvironmentDescriptor
    )
}

fn covers_all<T, const N: usize>(present: impl IntoIterator<Item = T>, required: [T; N]) -> bool
where
    T: Ord + Copy,
{
    let seen: BTreeSet<T> = present.into_iter().collect();
    required
        .iter()
        .all(|required_item| seen.contains(required_item))
}

fn seeded_header_vocabulary() -> Vec<HeaderFieldDefinition> {
    vec![
        header_field(
            HeaderFieldClass::FileIdentity,
            "File identity",
            "Stable file or artifact identity shown in the header before any edit or inspect action.",
        ),
        header_field(
            HeaderFieldClass::ArtifactClass,
            "Artifact class",
            "Human-readable class label so users can tell whether they are editing a request env, runtime manifest, policy bundle, or another governed artifact.",
        ),
        header_field(
            HeaderFieldClass::CanonicalSourceNote,
            "Canonical source",
            "Explicit note naming the only writable canonical source so effective or live views never masquerade as round-trip-safe text.",
        ),
        header_field(
            HeaderFieldClass::TargetContext,
            "Target context",
            "Visible boundary label naming the workspace, runtime, mirror, or policy scope the current mode resolves against.",
        ),
        header_field(
            HeaderFieldClass::ValidatorState,
            "Validator state",
            "Schema or validator state carried next to the identity so stale or unresolved views stay labeled.",
        ),
        header_field(
            HeaderFieldClass::ActiveMode,
            "Active mode",
            "The currently selected source/effective/live projection repeated in the header for cross-surface parity.",
        ),
    ]
}

fn seeded_mode_vocabulary() -> Vec<ModeVocabularyRow> {
    vec![
        ModeVocabularyRow {
            mode: ModeClass::Source,
            label: "Source".to_owned(),
            description:
                "Canonical authored source object. This is the only mode that may be writable as canonical text."
                    .to_owned(),
            canonical_writable_truth: true,
        },
        ModeVocabularyRow {
            mode: ModeClass::Effective,
            label: "Effective".to_owned(),
            description:
                "Resolved projection after imports, defaults, policy, secrets, and runtime discovery. It is inspect-only unless promoted back through the source object."
                    .to_owned(),
            canonical_writable_truth: false,
        },
        ModeVocabularyRow {
            mode: ModeClass::Live,
            label: "Live".to_owned(),
            description:
                "Observed runtime or mirrored target state. It is never treated as canonical writable text and stays labeled when stale, unresolved, or deferred."
                    .to_owned(),
            canonical_writable_truth: false,
        },
    ]
}

fn seeded_layer_vocabulary() -> Vec<LayerVocabularyRow> {
    vec![
        layer_term(
            LayerVocabularyField::LayerOrder,
            "Layer order",
            "Visible precedence order for the environment or runtime stack.",
        ),
        layer_term(
            LayerVocabularyField::SourceClass,
            "Source class",
            "Stable source class such as workspace source, policy bundle, user override, or runtime discovery.",
        ),
        layer_term(
            LayerVocabularyField::TrackedState,
            "Tracked state",
            "Whether the layer is tracked, ignored, or derived read-only.",
        ),
        layer_term(
            LayerVocabularyField::PolicyLock,
            "Policy lock",
            "Whether policy leaves the layer alone, narrows it, or locks it.",
        ),
        layer_term(
            LayerVocabularyField::SecretBearingNote,
            "Secret note",
            "Disclosure that a layer carries secret handles, redacted placeholders, or no secrets.",
        ),
        layer_term(
            LayerVocabularyField::ResetAction,
            "Reset action",
            "Layer-specific reset or remove-override action that does not require leaving the IDE.",
        ),
        layer_term(
            LayerVocabularyField::OpenSourceAction,
            "Open source action",
            "Direct jump back to the source object that contributed the layer.",
        ),
    ]
}

fn seeded_surface_vocabulary() -> Vec<SurfaceVocabularyBinding> {
    ConsumerSurfaceClass::ALL
        .into_iter()
        .map(|surface| SurfaceVocabularyBinding {
            surface,
            shared_contract_ref: STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF
                .to_owned(),
            header_fields: HeaderFieldClass::ALL.to_vec(),
            mode_labels: ModeClass::ALL.to_vec(),
            layer_fields: LayerVocabularyField::ALL.to_vec(),
        })
        .collect()
}

fn seeded_artifact_surfaces() -> Vec<ArtifactSurfaceRow> {
    use ArtifactFamilyKind as Family;
    use LayerPolicyLockState as LayerLock;
    use LayerTrackedState as Track;
    use ModeClass as Mode;
    use ModeStateClass as State;
    use QualificationLabel as Qual;
    use SourceLayerClass as Source;
    use TargetBoundaryClass as Boundary;
    use ValidatorStateClass as Validator;
    use WriteEligibilityClass as Write;

    vec![
        artifact(
            Family::RequestWorkspaceEnvironment,
            Qual::Beta,
            "family:request-workspace-environment",
            "artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json",
            vec![
                "docs/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.md",
                "schemas/data/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.schema.json",
            ],
            ArtifactHeader {
                identity_label: "requests/payments.http#env:production".to_owned(),
                identity_ref: "artifact:request-workspace-environment:payments-production".to_owned(),
                artifact_class_label: "Request workspace environment".to_owned(),
                canonical_source_note: "Edit the request document or environment set source; effective and live views are inspect-only.".to_owned(),
                target_context_label: "Workspace payments-api / request runtime production".to_owned(),
                validator_state: Validator::Current,
                validator_summary: "Schema snapshot current; environment and auth inspectors resolved.".to_owned(),
                active_mode: Mode::Effective,
            },
            vec![
                mode_row(
                    Mode::Source,
                    false,
                    "Opened from workspace source at 2026-06-11T09:24:00Z",
                    Boundary::LocalWorkspace,
                    Write::WritableCanonicalSource,
                    State::Current,
                    "Canonical request and environment source stays writable.",
                ),
                mode_row(
                    Mode::Effective,
                    true,
                    "Resolved at 2026-06-11T09:24:18Z",
                    Boundary::RequestRuntime,
                    Write::InspectOnlyProjection,
                    State::Current,
                    "Resolved request values include policy, secret handles, and ad hoc overrides.",
                ),
                mode_row(
                    Mode::Live,
                    false,
                    "Live bind deferred until next send",
                    Boundary::RequestRuntime,
                    Write::DeferredUntilRuntime,
                    State::Deferred,
                    "Observed runtime values are deferred until the request is dispatched.",
                ),
            ],
            Some(layer_stack(
                "Environment layers stay visible from the request editor and send inspector.",
                vec![
                    layer(10, "Workspace defaults", Source::WorkspaceSource, Track::Tracked, LayerLock::None, "No secret material on this layer.", false, true, true),
                    layer(20, "Imported profile fallback", Source::ProfileImport, Track::Tracked, LayerLock::None, "No secret material on this layer.", false, true, true),
                    layer(30, "Policy egress overlay", Source::PolicyBundle, Track::Tracked, LayerLock::Narrowed, "No secret material on this layer.", true, false, true),
                    layer(40, "Secret broker handles", Source::SecretReference, Track::Tracked, LayerLock::None, "Carries broker handles only; raw secrets never render.", false, true, true),
                ],
            )),
            vec![
                lifecycle_marker(
                    LifecycleMarkerClass::PolicyGatedDependency,
                    "Policy egress overlay",
                    "aureline://policy/request-egress-overlay",
                    "Stable",
                    "Effective request values remain narrowed by the signed egress overlay.",
                    "Review the policy bundle or stay on source/effective mode.",
                ),
                lifecycle_marker(
                    LifecycleMarkerClass::UnsupportedDependency,
                    "Deferred live bind",
                    "aureline://runtime/request-send-bind",
                    "Stable",
                    "Live truth is deferred until a request run captures the bound runtime values.",
                    "Dispatch the request or inspect the effective projection.",
                ),
            ],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "Request environment row",
                false,
                None,
                None,
                "Request-environment truth stays beta-scoped because policy and deferred runtime bind still narrow the row.",
            ),
            vec![
                mutation_flow(
                    MutationFlowClass::ResetLayer,
                    "workspace env set: production",
                    "aureline://preview/request-env-reset-production",
                    &["Workspace defaults", "Imported profile fallback"],
                    &[],
                    Some("aureline://checkpoint/request-env-reset-production"),
                    None,
                ),
                mutation_flow(
                    MutationFlowClass::ImportBundle,
                    "imported request env fragment: production",
                    "aureline://preview/request-env-import-production",
                    &["Imported profile fallback"],
                    &["aureline://bundle/request-env-import-production"],
                    Some("aureline://checkpoint/request-env-import-production"),
                    None,
                ),
            ],
        ),
        artifact(
            Family::DatabaseProfile,
            Qual::Stable,
            "family:database-profile",
            "artifacts/release/m4/notebook-and-data-rich-surface-qualification.json",
            vec![
                "docs/help/notebook-and-data-rich-surface-qualification.md",
                "schemas/data/certify-api-database-and-browser-runtime-workflows-with-mutation-redaction-and-scale-drills.schema.json",
            ],
            ArtifactHeader {
                identity_label: "db/profiles/warehouse.production.json".to_owned(),
                identity_ref: "artifact:database-profile:warehouse-production".to_owned(),
                artifact_class_label: "Database profile".to_owned(),
                canonical_source_note: "Edit the profile source; effective and live connectivity views are inspect-only.".to_owned(),
                target_context_label: "Workspace analytics / database runtime production".to_owned(),
                validator_state: Validator::Current,
                validator_summary: "Connection profile validates and the last probe is current.".to_owned(),
                active_mode: Mode::Live,
            },
            vec![
                mode_row(Mode::Source, false, "Opened from workspace source at 2026-06-11T09:31:00Z", Boundary::LocalWorkspace, Write::WritableCanonicalSource, State::Current, "Canonical connection source stays writable."),
                mode_row(Mode::Effective, false, "Resolved at 2026-06-11T09:31:07Z", Boundary::DatabaseRuntime, Write::InspectOnlyProjection, State::Current, "Effective connection profile includes policy and secret-handle narrowing."),
                mode_row(Mode::Live, true, "Observed at 2026-06-11T09:31:10Z", Boundary::DatabaseRuntime, Write::InspectOnlyLiveObservation, State::Current, "Observed connectivity is live runtime state, not canonical text."),
            ],
            Some(layer_stack(
                "Connection layers stay visible from the profile editor and inspect path.",
                vec![
                    layer(10, "Workspace profile file", Source::WorkspaceSource, Track::Tracked, LayerLock::None, "No secret material on this layer.", false, true, true),
                    layer(20, "User override", Source::UserOverride, Track::Tracked, LayerLock::None, "No secret material on this layer.", true, true, true),
                    layer(30, "Policy TLS floor", Source::PolicyBundle, Track::Tracked, LayerLock::Narrowed, "No secret material on this layer.", false, false, true),
                    layer(40, "Credential handles", Source::SecretReference, Track::Tracked, LayerLock::None, "Carries credential handles only; raw credentials are excluded.", false, true, true),
                ],
            )),
            vec![lifecycle_marker(
                LifecycleMarkerClass::PolicyGatedDependency,
                "Policy TLS floor",
                "aureline://policy/database-profile-tls-floor",
                "Stable",
                "The live connection posture is narrowed by the signed TLS floor before any local override can win.",
                "Inspect the policy source or repair the profile under the same scope.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "Database profile row",
                false,
                None,
                None,
                "The profile is stable but still discloses its policy-gated transport floor instead of hiding it behind a healthy-looking live row.",
            ),
            vec![mutation_flow(
                MutationFlowClass::RepairConfig,
                "workspace database profile: warehouse.production",
                "aureline://preview/database-profile-repair",
                &["Workspace profile file", "User override"],
                &["aureline://policy/database-profile-tls-floor"],
                Some("aureline://checkpoint/database-profile-repair"),
                None,
            )],
        ),
        artifact(
            Family::ApiProfile,
            Qual::Stable,
            "family:api-profile",
            "artifacts/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.json",
            vec![
                "docs/data/m5/materialize-versioned-request-workspace-documents-environment-sets-and-auth-source-inspectors.md",
                "artifacts/release/m4/notebook-and-data-rich-surface-qualification.json",
            ],
            ArtifactHeader {
                identity_label: "api/profiles/payments.production.json".to_owned(),
                identity_ref: "artifact:api-profile:payments-production".to_owned(),
                artifact_class_label: "API profile".to_owned(),
                canonical_source_note: "Edit the API profile source; effective and live rows remain inspect-only.".to_owned(),
                target_context_label: "Workspace payments-api / service runtime production".to_owned(),
                validator_state: Validator::PolicyLocked,
                validator_summary: "Profile resolves, but policy pins outbound routing and auth posture.".to_owned(),
                active_mode: Mode::Effective,
            },
            vec![
                mode_row(Mode::Source, false, "Opened from workspace source at 2026-06-11T09:42:00Z", Boundary::LocalWorkspace, Write::WritableCanonicalSource, State::Current, "Canonical API profile source stays writable."),
                mode_row(Mode::Effective, true, "Resolved at 2026-06-11T09:42:06Z", Boundary::ApiRuntime, Write::InspectOnlyProjection, State::Current, "Effective profile resolves request defaults, policy, and secret-handle layers."),
                mode_row(Mode::Live, false, "Observed at 2026-06-11T09:42:11Z", Boundary::ApiRuntime, Write::InspectOnlyLiveObservation, State::Current, "Live mode shows observed service posture and never implies writable source text."),
            ],
            Some(layer_stack(
                "Profile layers stay visible from the editor, CLI inspect, and send review.",
                vec![
                    layer(10, "Workspace profile file", Source::WorkspaceSource, Track::Tracked, LayerLock::None, "No secret material on this layer.", false, true, true),
                    layer(20, "User route override", Source::UserOverride, Track::Ignored, LayerLock::None, "No secret material on this layer; ignored because policy route wins.", false, true, true),
                    layer(30, "Policy route and auth floor", Source::PolicyBundle, Track::Tracked, LayerLock::Locked, "No secret material on this layer.", true, false, true),
                    layer(40, "Credential handles", Source::SecretReference, Track::Tracked, LayerLock::None, "Carries broker handles only.", false, true, true),
                ],
            )),
            vec![lifecycle_marker(
                LifecycleMarkerClass::PolicyGatedDependency,
                "Policy route and auth floor",
                "aureline://policy/api-profile-route-floor",
                "Stable",
                "Effective API routing stays locked to the signed policy floor instead of silently widening to the ignored local override.",
                "Open the policy source or request an admin-authored change.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "API profile row",
                false,
                None,
                None,
                "Stable-facing API rows stay explicit about locked policy routing and do not inherit a hidden local-override story.",
            ),
            vec![mutation_flow(
                MutationFlowClass::ResetLayer,
                "workspace API profile: payments.production",
                "aureline://preview/api-profile-reset",
                &["User route override", "Policy route and auth floor"],
                &["aureline://policy/api-profile-route-floor"],
                None,
                Some(
                    "Signed policy ownership blocks local reset of the winning route and auth floor.",
                ),
            )],
        ),
        artifact(
            Family::NotebookRuntimeManifest,
            Qual::Beta,
            "family:notebook-runtime-manifest",
            "schemas/notebook/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors.schema.json",
            vec![
                "fixtures/notebook/m5/implement_kernel_discovery_kernelspec_and_interpreter_resolution_and_environment_fingerprint_inspectors/manifest.yaml",
                "docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md",
            ],
            ArtifactHeader {
                identity_label: "notebooks/runtime/etl.kernel.json".to_owned(),
                identity_ref: "artifact:notebook-runtime-manifest:etl".to_owned(),
                artifact_class_label: "Notebook runtime manifest".to_owned(),
                canonical_source_note: "Edit the runtime manifest source; effective and live kernel rows remain inspect-only.".to_owned(),
                target_context_label: "Notebook etl / python kernel session".to_owned(),
                validator_state: Validator::Warning,
                validator_summary: "Resolver matched a runtime, but the active kernel session is reconnecting.".to_owned(),
                active_mode: Mode::Effective,
            },
            vec![
                mode_row(Mode::Source, false, "Opened from workspace source at 2026-06-11T10:03:00Z", Boundary::LocalWorkspace, Write::WritableCanonicalSource, State::Current, "Manifest source remains the canonical writable truth."),
                mode_row(Mode::Effective, true, "Resolved at 2026-06-11T10:03:08Z", Boundary::KernelSession, Write::InspectOnlyProjection, State::Current, "Effective runtime choice combines authored kernelspec intent and runtime discovery."),
                mode_row(Mode::Live, false, "Kernel handoff unresolved at 2026-06-11T10:03:11Z", Boundary::KernelSession, Write::UnresolvedTarget, State::Unresolved, "Live kernel state is unresolved while the session reattaches."),
            ],
            Some(layer_stack(
                "Kernel resolution layers stay visible from the manifest and inspector surfaces.",
                vec![
                    layer(10, "Workspace kernelspec", Source::WorkspaceSource, Track::Tracked, LayerLock::None, "No secret material on this layer.", false, true, true),
                    layer(20, "Policy runtime floor", Source::PolicyBundle, Track::Tracked, LayerLock::Narrowed, "No secret material on this layer.", false, false, true),
                    layer(30, "Interpreter discovery", Source::RuntimeDiscovery, Track::DerivedReadOnly, LayerLock::None, "No secret material on this layer; derived from runtime discovery.", true, false, true),
                    layer(40, "Observed kernel session", Source::LiveObservation, Track::Ignored, LayerLock::None, "No secret material on this layer; ignored until reattach completes.", false, false, true),
                ],
            )),
            vec![
                lifecycle_marker(
                    LifecycleMarkerClass::StaleExperiment,
                    "Notebook runtime rollout cohort",
                    "aureline://experiments/notebook-runtime-rollout",
                    "Beta",
                    "Kernel selection is still narrowed by a stale rollout cohort until the runtime rollout metadata is refreshed.",
                    "Refresh the experiments inventory or pin the authored kernelspec explicitly.",
                ),
                lifecycle_marker(
                    LifecycleMarkerClass::UnsupportedDependency,
                    "Live kernel reattach",
                    "aureline://runtime/notebook-kernel-reattach",
                    "Stable",
                    "Live kernel truth is unresolved while the current session reattaches.",
                    "Retry the kernel attach or continue from the effective manifest view.",
                ),
            ],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "Notebook runtime manifest row",
                true,
                Some("aureline://experiments/notebook-runtime-rollout"),
                None,
                "Notebook runtime rows remain explicitly beta-scoped because stale rollout state and unresolved live attach would otherwise spill into a stable-looking kernel choice.",
            ),
            vec![
                mutation_flow(
                    MutationFlowClass::RepairConfig,
                    "workspace notebook runtime manifest: etl",
                    "aureline://preview/notebook-runtime-repair",
                    &["Workspace kernelspec", "Interpreter discovery"],
                    &["aureline://experiments/notebook-runtime-rollout"],
                    Some("aureline://checkpoint/notebook-runtime-repair"),
                    None,
                ),
                mutation_flow(
                    MutationFlowClass::MigrationApply,
                    "runtime manifest migration: etl",
                    "aureline://preview/notebook-runtime-migration",
                    &["Workspace kernelspec"],
                    &[],
                    Some("aureline://checkpoint/notebook-runtime-migration"),
                    None,
                ),
            ],
        ),
        artifact(
            Family::PreviewRuntimeConfig,
            Qual::Preview,
            "family:preview-runtime-config",
            "artifacts/release/m4/preview-designer-publish-surface-qualification.json",
            vec![
                "artifacts/runtime/m4/stabilize_the_artifact_manager_preview_runtime_inspectors_and_truth_packet.json",
                "docs/help/preview-designer-publish-surface-qualification.md",
            ],
            ArtifactHeader {
                identity_label: "preview/runtime/frontpage.preview.json".to_owned(),
                identity_ref: "artifact:preview-runtime-config:frontpage".to_owned(),
                artifact_class_label: "Preview runtime config".to_owned(),
                canonical_source_note: "Edit the preview config source; effective and live preview rows stay inspect-only and visibly narrower than stable.".to_owned(),
                target_context_label: "Workspace webapp / preview runtime browser".to_owned(),
                validator_state: Validator::PreviewNarrowed,
                validator_summary: "Preview runtime validates, but this row stays explicitly preview-scoped.".to_owned(),
                active_mode: Mode::Live,
            },
            vec![
                mode_row(Mode::Source, false, "Opened from workspace source at 2026-06-11T10:18:00Z", Boundary::LocalWorkspace, Write::WritableCanonicalSource, State::Current, "Preview source remains the canonical editable object."),
                mode_row(Mode::Effective, false, "Resolved at 2026-06-11T10:18:09Z", Boundary::PreviewRuntime, Write::InspectOnlyProjection, State::Current, "Effective preview projection includes managed overrides and runtime discovery."),
                mode_row(Mode::Live, true, "Observed at 2026-06-11T10:18:13Z", Boundary::PreviewRuntime, Write::InspectOnlyLiveObservation, State::Stale, "Live preview capture is stale and remains labeled rather than masquerading as current source."),
            ],
            Some(layer_stack(
                "Preview runtime layers stay visible from the preview inspector and publish review.",
                vec![
                    layer(10, "Workspace preview config", Source::WorkspaceSource, Track::Tracked, LayerLock::None, "No secret material on this layer.", false, true, true),
                    layer(20, "Policy preview floor", Source::PolicyBundle, Track::Tracked, LayerLock::Narrowed, "No secret material on this layer.", false, false, true),
                    layer(30, "Managed preview override", Source::ManagedOverride, Track::Tracked, LayerLock::Locked, "Carries redacted placeholders only.", true, false, true),
                    layer(40, "Runtime discovery", Source::RuntimeDiscovery, Track::DerivedReadOnly, LayerLock::None, "No secret material on this layer; derived from preview runtime inspection.", false, false, true),
                ],
            )),
            vec![
                lifecycle_marker(
                    LifecycleMarkerClass::PreviewDependency,
                    "Preview runtime browser lane",
                    "aureline://feature/preview-runtime-browser",
                    "Preview",
                    "Authored preview config still depends on the preview-runtime lane and cannot inherit stable-facing defaults.",
                    "Stay on the preview-qualified flow or edit the canonical source directly.",
                ),
                lifecycle_marker(
                    LifecycleMarkerClass::PolicyGatedDependency,
                    "Managed preview override",
                    "aureline://policy/preview-runtime-override",
                    "Stable",
                    "Managed preview policy still narrows the effective runtime route.",
                    "Inspect the policy source or continue with the authored preview source only.",
                ),
            ],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::BlockedStableFacingRow,
                "Preview runtime row",
                true,
                Some("aureline://experiments/preview-runtime-rollout"),
                None,
                "Preview runtime rows are blocked from stable-facing treatment until the preview rollout dependency is removed or graduated.",
            ),
            vec![mutation_flow(
                MutationFlowClass::RepairConfig,
                "workspace preview runtime config: frontpage",
                "aureline://preview/preview-runtime-repair",
                &["Workspace preview config", "Managed preview override"],
                &["aureline://policy/preview-runtime-override"],
                Some("aureline://checkpoint/preview-runtime-repair"),
                None,
            )],
        ),
        artifact(
            Family::WorkflowBundleManifest,
            Qual::Beta,
            "family:workflow-bundle-manifest",
            "artifacts/workspace/m5/m5-workflow-bundle-manifests.json",
            vec![
                "artifacts/workspace/m5/m5-workflow-bundle-manifests.md",
                "artifacts/compat/m3/workflow_bundle_examples/workflow_bundle_review_examples.json",
            ],
            ArtifactHeader {
                identity_label: "workflow-bundles/data-api.bundle.json".to_owned(),
                identity_ref: "artifact:workflow-bundle-manifest:data-api".to_owned(),
                artifact_class_label: "Workflow bundle manifest".to_owned(),
                canonical_source_note: "Edit the workflow bundle manifest source; effective and runtime views remain inspect-only.".to_owned(),
                target_context_label: "Workspace payments-api / bundle runtime data-api".to_owned(),
                validator_state: Validator::Current,
                validator_summary: "Bundle manifest validates and dependency markers remain explicit.".to_owned(),
                active_mode: Mode::Source,
            },
            vec![
                mode_row(Mode::Source, true, "Opened from workspace source at 2026-06-11T10:37:00Z", Boundary::LocalWorkspace, Write::WritableCanonicalSource, State::Current, "Workflow bundle manifest source is the canonical writable truth."),
                mode_row(Mode::Effective, false, "Resolved at 2026-06-11T10:37:05Z", Boundary::WorkflowRuntime, Write::InspectOnlyProjection, State::Current, "Effective bundle view expands imported defaults and policy markers."),
                mode_row(Mode::Live, false, "Bundle execution binds on demand", Boundary::WorkflowRuntime, Write::DeferredUntilRuntime, State::Deferred, "Live execution state is deferred until the bundle runs."),
            ],
            Some(layer_stack(
                "Bundle environment layers stay visible from bundle review and runtime inspect surfaces.",
                vec![
                    layer(10, "Imported profile baseline", Source::ProfileImport, Track::Tracked, LayerLock::None, "No secret material on this layer.", false, true, true),
                    layer(20, "Bundle manifest source", Source::WorkspaceSource, Track::Tracked, LayerLock::None, "No secret material on this layer.", true, true, true),
                    layer(30, "Policy execution floor", Source::PolicyBundle, Track::Tracked, LayerLock::Narrowed, "No secret material on this layer.", false, false, true),
                    layer(40, "Runtime discovery", Source::RuntimeDiscovery, Track::DerivedReadOnly, LayerLock::None, "No secret material on this layer; derived from bundle runtime selection.", false, false, true),
                ],
            )),
            vec![lifecycle_marker(
                LifecycleMarkerClass::LabsDependency,
                "Imported execution bundle lane",
                "aureline://feature/workflow-bundle-import",
                "Labs",
                "Imported execution bundles still depend on the Labs-only bundle lane and remain visibly narrower than stable manifest rows.",
                "Open the canonical bundle source or keep the import in review-only mode.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "Workflow bundle manifest row",
                true,
                Some("aureline://experiments/workflow-bundle-import"),
                None,
                "Workflow bundle manifests keep Labs-only imported execution state explicit instead of leaking it into a stable-looking authored row.",
            ),
            vec![mutation_flow(
                MutationFlowClass::ImportBundle,
                "workflow bundle import: data-api",
                "aureline://preview/workflow-bundle-import",
                &["Imported profile baseline", "Bundle manifest source"],
                &["aureline://bundle/workflow-data-api"],
                Some("aureline://checkpoint/workflow-bundle-import"),
                None,
            )],
        ),
        artifact(
            Family::CiEnvironmentDescriptor,
            Qual::Beta,
            "family:ci-environment-descriptor",
            "artifacts/infra/infrastructure-surface-qualification/support_export.json",
            vec![
                "docs/infra/infrastructure-surface-qualification.md",
                "fixtures/infra/plan-and-validation-viewers/qualified_viewer_packet.json",
            ],
            ArtifactHeader {
                identity_label: "ci/environments/release.pipeline.yaml".to_owned(),
                identity_ref: "artifact:ci-environment-descriptor:release-pipeline".to_owned(),
                artifact_class_label: "CI environment descriptor".to_owned(),
                canonical_source_note: "Edit the CI descriptor source; effective and mirrored observation modes are inspect-only.".to_owned(),
                target_context_label: "Repo release pipeline / CI mirror lane".to_owned(),
                validator_state: Validator::Stale,
                validator_summary: "Mirror-backed CI inspection is stale and remains labeled as such.".to_owned(),
                active_mode: Mode::Live,
            },
            vec![
                mode_row(Mode::Source, false, "Opened from workspace source at 2026-06-11T10:52:00Z", Boundary::LocalWorkspace, Write::WritableCanonicalSource, State::Current, "CI descriptor source remains the canonical writable truth."),
                mode_row(Mode::Effective, false, "Resolved at 2026-06-11T10:52:05Z", Boundary::CiMirror, Write::InspectOnlyProjection, State::Current, "Effective CI projection includes policy and managed overrides."),
                mode_row(Mode::Live, true, "Mirrored observation captured at 2026-06-11T10:50:31Z", Boundary::CiMirror, Write::InspectOnlyLiveObservation, State::Stale, "Mirrored CI observation is inspect-only and visibly stale."),
            ],
            Some(layer_stack(
                "CI layers stay visible from plan, diff, and support surfaces without leaving the IDE.",
                vec![
                    layer(10, "Repo descriptor source", Source::WorkspaceSource, Track::Tracked, LayerLock::None, "No secret material on this layer.", false, true, true),
                    layer(20, "Policy gate", Source::PolicyBundle, Track::Tracked, LayerLock::Narrowed, "No secret material on this layer.", false, false, true),
                    layer(30, "Managed CI overlay", Source::ManagedOverride, Track::Tracked, LayerLock::Locked, "Carries redacted placeholders only.", true, false, true),
                    layer(40, "Mirror snapshot", Source::LiveObservation, Track::DerivedReadOnly, LayerLock::None, "No secret material on this layer; derived from mirrored CI observation.", false, false, true),
                ],
            )),
            vec![lifecycle_marker(
                LifecycleMarkerClass::UnsupportedDependency,
                "Mirror-backed CI observation",
                "aureline://runtime/ci-mirror-observation",
                "Stable",
                "Live CI truth is mirror-backed and stale, so the row stays beta-scoped instead of claiming authoritative live observation.",
                "Refresh the mirror or inspect the authored descriptor and effective projection only.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "CI environment descriptor row",
                false,
                None,
                None,
                "CI descriptor rows stay beta-scoped because mirrored live truth can stale independently of the authored source.",
            ),
            vec![mutation_flow(
                MutationFlowClass::MigrationApply,
                "ci descriptor migration: release.pipeline",
                "aureline://preview/ci-descriptor-migration",
                &["Repo descriptor source", "Managed CI overlay"],
                &["aureline://policy/ci-managed-overlay"],
                Some("aureline://checkpoint/ci-descriptor-migration"),
                None,
            )],
        ),
        artifact(
            Family::InfraEnvironmentDescriptor,
            Qual::Beta,
            "family:infra-environment-descriptor",
            "artifacts/infra/infrastructure-surface-qualification/support_export.json",
            vec![
                "docs/infra/cluster-context-and-live-resource.md",
                "fixtures/infra/cluster-context-and-live-resource/qualified_cluster_context_packet.json",
            ],
            ArtifactHeader {
                identity_label: "infra/environments/payments.prod.yaml".to_owned(),
                identity_ref: "artifact:infra-environment-descriptor:payments-prod".to_owned(),
                artifact_class_label: "Infrastructure environment descriptor".to_owned(),
                canonical_source_note: "Edit the infrastructure descriptor source; effective and mirrored live modes remain inspect-only.".to_owned(),
                target_context_label: "Cluster payments-us-1 / checkout namespace".to_owned(),
                validator_state: Validator::Current,
                validator_summary: "Target context is current; mirrored live overlay remains inspect-only.".to_owned(),
                active_mode: Mode::Live,
            },
            vec![
                mode_row(Mode::Source, false, "Opened from workspace source at 2026-06-11T11:04:00Z", Boundary::LocalWorkspace, Write::WritableCanonicalSource, State::Current, "Infrastructure descriptor source remains the canonical writable truth."),
                mode_row(Mode::Effective, false, "Resolved at 2026-06-11T11:04:04Z", Boundary::InfraMirror, Write::InspectOnlyProjection, State::Current, "Effective descriptor view keeps rendered and planned truth separate from source."),
                mode_row(Mode::Live, true, "Mirrored observation captured at 2026-06-11T11:04:09Z", Boundary::InfraMirror, Write::InspectOnlyLiveObservation, State::Current, "Live overlay is mirrored observation only and never canonical source text."),
            ],
            Some(layer_stack(
                "Infrastructure layers stay visible from target-context, plan, and support surfaces.",
                vec![
                    layer(10, "Repo descriptor source", Source::WorkspaceSource, Track::Tracked, LayerLock::None, "No secret material on this layer.", true, true, true),
                    layer(20, "Policy environment floor", Source::PolicyBundle, Track::Tracked, LayerLock::Narrowed, "No secret material on this layer.", false, false, true),
                    layer(30, "Managed environment overlay", Source::ManagedOverride, Track::Tracked, LayerLock::Locked, "Carries redacted placeholders only.", false, false, true),
                    layer(40, "Observed resource overlay", Source::LiveObservation, Track::DerivedReadOnly, LayerLock::None, "No secret material on this layer; derived from mirrored cluster observation.", false, false, true),
                ],
            )),
            vec![lifecycle_marker(
                LifecycleMarkerClass::UnsupportedDependency,
                "Mirrored cluster observation",
                "aureline://runtime/infra-mirror-observation",
                "Stable",
                "Observed cluster state is mirrored and cannot silently stand in for authoritative live infrastructure truth.",
                "Inspect the target context or continue from source/effective views.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "Infrastructure environment descriptor row",
                false,
                None,
                None,
                "Infrastructure rows stay beta-scoped because mirrored observations may diverge from authoritative live target state.",
            ),
            vec![mutation_flow(
                MutationFlowClass::RepairConfig,
                "infra descriptor repair: payments.prod",
                "aureline://preview/infra-descriptor-repair",
                &["Repo descriptor source", "Managed environment overlay"],
                &["aureline://policy/infra-environment-floor"],
                Some("aureline://checkpoint/infra-descriptor-repair"),
                None,
            )],
        ),
        artifact(
            Family::ManagedPolicyOverlay,
            Qual::Stable,
            "family:managed-policy-overlay",
            "artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json",
            vec![
                "docs/config/structured_config_policy_bundle_and_entitlement_matrix.md",
                "docs/policy/admin_policy_and_bundle_cache_contract.md",
            ],
            ArtifactHeader {
                identity_label: "policy/overlays/org-defaults.policy.json".to_owned(),
                identity_ref: "artifact:managed-policy-overlay:org-defaults".to_owned(),
                artifact_class_label: "Managed policy overlay".to_owned(),
                canonical_source_note: "The signed policy bundle is canonical; this local overlay surface is review-only.".to_owned(),
                target_context_label: "Org policy cache / managed policy lane".to_owned(),
                validator_state: Validator::PolicyLocked,
                validator_summary: "Signed policy narrowing is current and visibly locked.".to_owned(),
                active_mode: Mode::Effective,
            },
            vec![
                mode_row(Mode::Source, false, "Signed bundle opened at 2026-06-11T11:21:00Z", Boundary::PolicyCache, Write::SignedBundleReviewOnly, State::Current, "Signed policy source is reviewable locally but not editable as ordinary text."),
                mode_row(Mode::Effective, true, "Policy narrowed at 2026-06-11T11:21:06Z", Boundary::PolicyCache, Write::InspectOnlyProjection, State::Current, "Effective policy overlay shows the narrowed result without implying user ownership."),
                mode_row(Mode::Live, false, "No live mode for signed policy overlays", Boundary::PolicyCache, Write::SignedBundleReviewOnly, State::Unsupported, "Managed policy overlays do not claim a separate live text mode."),
            ],
            None,
            vec![lifecycle_marker(
                LifecycleMarkerClass::PolicyGatedDependency,
                "Signed policy ownership",
                "aureline://bundle/admin-policy-primary",
                "Stable",
                "The winning overlay remains policy-owned and cannot silently collapse into a writable local row.",
                "Open the signed bundle or continue with inspect-only effective truth.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "Managed policy overlay row",
                false,
                None,
                None,
                "Managed policy overlays stay explicit about signed ownership and do not leak a hidden writable reset path.",
            ),
            vec![mutation_flow(
                MutationFlowClass::ResetLayer,
                "managed policy overlay: org-defaults",
                "aureline://preview/managed-policy-overlay-reset",
                &["Signed policy overlay"],
                &["aureline://bundle/admin-policy-primary"],
                None,
                Some("Signed policy ownership denies local reset of the active overlay."),
            )],
        ),
        artifact(
            Family::AdminPolicyBundleArtifact,
            Qual::Stable,
            "family:admin-policy-bundle",
            "artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json",
            vec![
                "docs/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md",
                "schemas/policy/policy_bundle_cache_entry.schema.json",
            ],
            ArtifactHeader {
                identity_label: "bundles/policy/admin-policy.bundle.json".to_owned(),
                identity_ref: "artifact:admin-policy-bundle:primary".to_owned(),
                artifact_class_label: "Admin policy bundle".to_owned(),
                canonical_source_note: "The signed bundle envelope is canonical; local review surfaces are inspect-only.".to_owned(),
                target_context_label: "Admin policy import review / signed bundle cache".to_owned(),
                validator_state: Validator::Current,
                validator_summary: "Signer, scope, digest, and expiry state are inspectable before apply.".to_owned(),
                active_mode: Mode::Source,
            },
            vec![
                mode_row(Mode::Source, true, "Bundle opened at 2026-06-11T11:33:00Z", Boundary::PolicyCache, Write::SignedBundleReviewOnly, State::Current, "Signed bundle envelope is the authored source object under review."),
                mode_row(Mode::Effective, false, "Policy effect computed at 2026-06-11T11:33:05Z", Boundary::PolicyCache, Write::InspectOnlyProjection, State::Current, "Effective mode shows the exact policy narrowing the bundle would apply."),
                mode_row(Mode::Live, false, "No live text mode for signed policy bundles", Boundary::PolicyCache, Write::SignedBundleReviewOnly, State::Unsupported, "Signed policy bundles do not expose a separate live text projection."),
            ],
            None,
            vec![lifecycle_marker(
                LifecycleMarkerClass::PolicyGatedDependency,
                "Signed bundle apply boundary",
                "aureline://bundle/admin-policy-primary",
                "Stable",
                "Import and apply stay review-only until the signed policy boundary is explicitly accepted.",
                "Open the signed bundle diff or import through the reviewed apply sheet.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "Admin policy bundle row",
                false,
                None,
                None,
                "Admin policy bundle rows keep review-only import state explicit instead of presenting a stable-looking text editor.",
            ),
            vec![mutation_flow(
                MutationFlowClass::ImportBundle,
                "admin policy bundle import: primary",
                "aureline://preview/admin-policy-bundle-import",
                &[],
                &["aureline://bundle/admin-policy-primary"],
                Some("aureline://checkpoint/admin-policy-bundle-import"),
                None,
            )],
        ),
        artifact(
            Family::OfflineEntitlementSnapshotArtifact,
            Qual::Stable,
            "family:offline-entitlement-snapshot",
            "artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json",
            vec![
                "docs/identity/offline_entitlement_and_policy_seed.md",
                "docs/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md",
            ],
            ArtifactHeader {
                identity_label: "bundles/entitlement/offline-seat.snapshot.json".to_owned(),
                identity_ref: "artifact:offline-entitlement-snapshot:seat-cache".to_owned(),
                artifact_class_label: "Offline entitlement snapshot".to_owned(),
                canonical_source_note: "The signed entitlement snapshot is canonical; local review and grace handling are inspect-only.".to_owned(),
                target_context_label: "Offline entitlement cache / managed feature gate".to_owned(),
                validator_state: Validator::Warning,
                validator_summary: "Snapshot is within grace and remains visibly narrower than live managed authority.".to_owned(),
                active_mode: Mode::Effective,
            },
            vec![
                mode_row(Mode::Source, false, "Snapshot opened at 2026-06-11T11:48:00Z", Boundary::OfflineEntitlementCache, Write::SignedBundleReviewOnly, State::Current, "Signed entitlement snapshot stays reviewable as the source object."),
                mode_row(Mode::Effective, true, "Grace posture computed at 2026-06-11T11:48:08Z", Boundary::OfflineEntitlementCache, Write::InspectOnlyProjection, State::Current, "Effective mode shows grace-window posture and any narrowed managed capability state."),
                mode_row(Mode::Live, false, "Live managed entitlement unavailable while offline", Boundary::OfflineEntitlementCache, Write::UnresolvedTarget, State::Unsupported, "Offline entitlement snapshots do not claim a live managed text projection."),
            ],
            None,
            vec![lifecycle_marker(
                LifecycleMarkerClass::PolicyGatedDependency,
                "Offline entitlement grace",
                "aureline://bundle/offline-entitlement-seat-cache",
                "Stable",
                "Managed capability truth is narrowed to the signed grace snapshot while live authority is unavailable.",
                "Refresh live authority or continue under the labeled grace snapshot.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::DisclosedNarrowing,
                "Offline entitlement snapshot row",
                false,
                None,
                None,
                "Offline entitlement rows remain stable-qualified but keep grace-snapshot narrowing explicit instead of hiding it as generic auth freshness.",
            ),
            vec![mutation_flow(
                MutationFlowClass::MigrationApply,
                "offline entitlement snapshot refresh: seat-cache",
                "aureline://preview/offline-entitlement-refresh",
                &[],
                &["aureline://bundle/offline-entitlement-seat-cache"],
                Some("aureline://checkpoint/offline-entitlement-refresh"),
                None,
            )],
        ),
        artifact(
            Family::EmergencyDisableBundleArtifact,
            Qual::Stable,
            "family:emergency-disable-bundle",
            "artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json",
            vec![
                "docs/security/emergency_distribution_policy.md",
                "artifacts/extensions/m3/advisory_templates/emergency_disable_support_export.json",
            ],
            ArtifactHeader {
                identity_label: "bundles/policy/emergency-disable.bundle.json".to_owned(),
                identity_ref: "artifact:emergency-disable-bundle:primary".to_owned(),
                artifact_class_label: "Emergency disable bundle".to_owned(),
                canonical_source_note: "The signed emergency bundle is canonical; local surfaces review its effect but do not edit it as ordinary text.".to_owned(),
                target_context_label: "Emergency policy review / signed bundle cache".to_owned(),
                validator_state: Validator::PolicyLocked,
                validator_summary: "Emergency ratchet is active and remains explicitly reviewable.".to_owned(),
                active_mode: Mode::Effective,
            },
            vec![
                mode_row(Mode::Source, false, "Bundle opened at 2026-06-11T12:02:00Z", Boundary::PolicyCache, Write::SignedBundleReviewOnly, State::Current, "Signed emergency-disable envelope is the authored source object under review."),
                mode_row(Mode::Effective, true, "Emergency effect computed at 2026-06-11T12:02:05Z", Boundary::PolicyCache, Write::InspectOnlyProjection, State::Current, "Effective mode shows the narrowed or disabled state without implying user-owned settings."),
                mode_row(Mode::Live, false, "No separate live text mode for emergency bundles", Boundary::PolicyCache, Write::SignedBundleReviewOnly, State::Unsupported, "Emergency bundles do not advertise a live text projection beyond the effective ratchet."),
            ],
            None,
            vec![lifecycle_marker(
                LifecycleMarkerClass::ExpiredKillSwitch,
                "Expired emergency disable window",
                "aureline://bundle/emergency-disable-primary",
                "DisabledByPolicy",
                "The emergency ratchet is still affecting the row after its planned expiry window and must stay visible instead of silently riding a stable row.",
                "Import the successor bundle or revoke the expired kill switch explicitly.",
            )],
            hidden_flag_guard(
                HiddenFlagSpillVerdict::BlockedStableFacingRow,
                "Emergency disable bundle row",
                true,
                None,
                Some("aureline://bundle/emergency-disable-primary"),
                "Stable-facing publication is blocked until the expired emergency disable state is reconciled or replaced.",
            ),
            vec![mutation_flow(
                MutationFlowClass::ImportBundle,
                "emergency disable bundle import: primary",
                "aureline://preview/emergency-disable-bundle-import",
                &[],
                &["aureline://bundle/emergency-disable-primary"],
                Some("aureline://checkpoint/emergency-disable-bundle-import"),
                None,
            )],
        ),
        artifact(
            Family::TrustRootSignerUpdateArtifact,
            Qual::Stable,
            "family:trust-root-signer-update",
            "artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json",
            vec![
                "docs/enterprise/m4/finalize-signed-policy-bundle-offline-entitlement-and-mirror.md",
                "docs/security/emergency_distribution_policy.md",
            ],
            ArtifactHeader {
                identity_label: "bundles/trust/trust-root-rotation.bundle.json".to_owned(),
                identity_ref: "artifact:trust-root-signer-update:rotation".to_owned(),
                artifact_class_label: "Trust-root or signer update".to_owned(),
                canonical_source_note: "The signed trust-root or signer-update bundle is canonical; local review surfaces stay inspect-only.".to_owned(),
                target_context_label: "Trust-root continuity review / signer cache".to_owned(),
                validator_state: Validator::Current,
                validator_summary: "Rotation window and overlap state are inspectable before import.".to_owned(),
                active_mode: Mode::Source,
            },
            vec![
                mode_row(Mode::Source, true, "Bundle opened at 2026-06-11T12:17:00Z", Boundary::TrustRootCache, Write::SignedBundleReviewOnly, State::Current, "Signed trust-root update envelope is the authored source object under review."),
                mode_row(Mode::Effective, false, "Continuity effect computed at 2026-06-11T12:17:04Z", Boundary::TrustRootCache, Write::InspectOnlyProjection, State::Current, "Effective mode shows which trust roots and signers would remain trusted after import."),
                mode_row(Mode::Live, false, "No separate live text mode for trust-root updates", Boundary::TrustRootCache, Write::SignedBundleReviewOnly, State::Unsupported, "Trust-root updates do not claim a separate live text projection."),
            ],
            None,
            Vec::new(),
            hidden_flag_guard(
                HiddenFlagSpillVerdict::ClearStableSurface,
                "Trust-root signer update row",
                false,
                None,
                None,
                "Trust-root rotation review is stable and does not depend on hidden experiment or kill-switch state.",
            ),
            vec![mutation_flow(
                MutationFlowClass::MigrationApply,
                "trust-root rotation import: rotation",
                "aureline://preview/trust-root-rotation-import",
                &[],
                &["aureline://bundle/trust-root-rotation"],
                Some("aureline://checkpoint/trust-root-rotation-import"),
                None,
            )],
        ),
    ]
}

fn artifact(
    family: ArtifactFamilyKind,
    qualification_label: QualificationLabel,
    family_ref: &str,
    source_packet_ref: &str,
    evidence_refs: Vec<&str>,
    header: ArtifactHeader,
    mode_switches: Vec<ModeSwitchRow>,
    environment_layer_stack: Option<EnvironmentLayerStack>,
    lifecycle_dependency_markers: Vec<LifecycleDependencyMarker>,
    hidden_flag_spill_guard: HiddenFlagSpillGuard,
    mutation_scope_flows: Vec<ScopeExplicitMutationFlow>,
) -> ArtifactSurfaceRow {
    ArtifactSurfaceRow {
        family,
        qualification_label,
        family_ref: family_ref.to_owned(),
        source_packet_ref: source_packet_ref.to_owned(),
        evidence_refs: evidence_refs.into_iter().map(str::to_owned).collect(),
        header,
        mode_switches,
        environment_stack_required: family_requires_environment_stack(family),
        environment_layer_stack,
        lifecycle_dependency_markers,
        hidden_flag_spill_guard,
        mutation_scope_flows,
    }
}

fn header_field(field: HeaderFieldClass, label: &str, description: &str) -> HeaderFieldDefinition {
    HeaderFieldDefinition {
        field,
        label: label.to_owned(),
        description: description.to_owned(),
    }
}

fn layer_term(field: LayerVocabularyField, label: &str, description: &str) -> LayerVocabularyRow {
    LayerVocabularyRow {
        field,
        label: label.to_owned(),
        description: description.to_owned(),
    }
}

fn mode_row(
    mode: ModeClass,
    active: bool,
    resolution_time_label: &str,
    target_boundary: TargetBoundaryClass,
    write_eligibility: WriteEligibilityClass,
    state: ModeStateClass,
    summary: &str,
) -> ModeSwitchRow {
    ModeSwitchRow {
        mode,
        active,
        resolution_time_label: resolution_time_label.to_owned(),
        target_boundary,
        write_eligibility,
        state,
        summary: summary.to_owned(),
    }
}

fn layer_stack(summary: &str, layers: Vec<EnvironmentLayerRow>) -> EnvironmentLayerStack {
    EnvironmentLayerStack {
        visible_without_leaving_ide: true,
        summary: summary.to_owned(),
        layers,
    }
}

fn lifecycle_marker(
    marker_class: LifecycleMarkerClass,
    dependency_label: &str,
    dependency_ref: &str,
    required_lifecycle_label: &str,
    effect_summary: &str,
    fallback_path: &str,
) -> LifecycleDependencyMarker {
    LifecycleDependencyMarker {
        marker_class,
        dependency_label: dependency_label.to_owned(),
        dependency_ref: dependency_ref.to_owned(),
        required_lifecycle_label: required_lifecycle_label.to_owned(),
        effect_summary: effect_summary.to_owned(),
        fallback_path: fallback_path.to_owned(),
        visible: true,
    }
}

fn hidden_flag_guard(
    verdict: HiddenFlagSpillVerdict,
    stable_facing_surface_label: &str,
    hidden_dependency_detected: bool,
    stale_experiment_ref: Option<&str>,
    expired_kill_switch_ref: Option<&str>,
    review_summary: &str,
) -> HiddenFlagSpillGuard {
    HiddenFlagSpillGuard {
        verdict,
        stable_facing_surface_label: stable_facing_surface_label.to_owned(),
        hidden_dependency_detected,
        stale_experiment_ref: stale_experiment_ref.map(str::to_owned),
        expired_kill_switch_ref: expired_kill_switch_ref.map(str::to_owned),
        review_summary: review_summary.to_owned(),
    }
}

fn mutation_flow(
    flow_class: MutationFlowClass,
    scope_label: &str,
    preview_ref: &str,
    affected_layer_labels: &[&str],
    affected_bundle_refs: &[&str],
    rollback_checkpoint_ref: Option<&str>,
    policy_denied_reason: Option<&str>,
) -> ScopeExplicitMutationFlow {
    ScopeExplicitMutationFlow {
        flow_class,
        scope_label: scope_label.to_owned(),
        preview_ref: preview_ref.to_owned(),
        affected_layer_labels: affected_layer_labels
            .iter()
            .map(ToString::to_string)
            .collect(),
        affected_bundle_refs: affected_bundle_refs
            .iter()
            .map(ToString::to_string)
            .collect(),
        rollback_checkpoint_ref: rollback_checkpoint_ref.map(str::to_owned),
        policy_denied_reason: policy_denied_reason.map(str::to_owned),
    }
}

fn layer(
    layer_order: u8,
    layer_label: &str,
    source_class: SourceLayerClass,
    tracked_state: LayerTrackedState,
    policy_lock: LayerPolicyLockState,
    secret_bearing_note: &str,
    wins_effective_value: bool,
    reset_available: bool,
    open_available: bool,
) -> EnvironmentLayerRow {
    EnvironmentLayerRow {
        layer_order,
        layer_label: layer_label.to_owned(),
        source_class,
        tracked_state,
        policy_lock,
        secret_bearing_note: secret_bearing_note.to_owned(),
        wins_effective_value,
        reset_action: LayerActionRow {
            available: reset_available,
            action_label: if reset_available {
                format!("Reset {}", layer_label.to_lowercase())
            } else {
                format!("Inspect reset limits for {}", layer_label.to_lowercase())
            },
            action_ref: format!("action:reset-layer:{layer_order}"),
        },
        open_source_action: LayerActionRow {
            available: open_available,
            action_label: if open_available {
                format!("Open {}", layer_label.to_lowercase())
            } else {
                format!("Inspect source for {}", layer_label.to_lowercase())
            },
            action_ref: format!("action:open-layer-source:{layer_order}"),
        },
    }
}
