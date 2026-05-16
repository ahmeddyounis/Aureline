//! Experiments and capability-dependency inventory projection.
//!
//! This module is the settings-owned runtime reader for the checked-in
//! experiments inventory. CLI inspection, diagnostics, and support exports
//! consume these records so Labs, Preview, policy-disabled, deprecated, and
//! retired capability truth stays tied to one artifact.

pub mod labs_governance_beta;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

/// Inventory schema version supported by this crate.
pub const EXPERIMENTS_INVENTORY_SCHEMA_VERSION: u32 = 1;

/// Source artifact consumed by [`load_default_inventory`].
pub const DEFAULT_EXPERIMENTS_INVENTORY_SOURCE_REF: &str =
    "artifacts/governance/experiments_inventory_alpha.yaml";

const DEFAULT_EXPERIMENTS_INVENTORY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/experiments_inventory_alpha.yaml"
));

const SHARED_CONTRACT_REF: &str = "settings:experiments_inventory_alpha:v1";

/// Lifecycle states rendered by experiments inventory consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CapabilityLifecycleState {
    /// Explicit opt-in or local-only exploratory surface.
    Labs,
    /// Named cohort or opt-in preview surface with expected churn.
    Preview,
    /// Broader rollout with support intent.
    Beta,
    /// Generally enabled alpha-supported contract.
    Stable,
    /// Still visible with replacement guidance.
    Deprecated,
    /// Present but unavailable because policy or a kill switch blocks it.
    DisabledByPolicy,
    /// No longer runnable; retained only as tombstone or migration truth.
    Retired,
}

impl CapabilityLifecycleState {
    /// Returns the controlled display token used by product surfaces.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Labs => "Labs",
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::Stable => "Stable",
            Self::Deprecated => "Deprecated",
            Self::DisabledByPolicy => "DisabledByPolicy",
            Self::Retired => "Retired",
        }
    }

    fn maturity_rank(self) -> Option<u8> {
        match self {
            Self::Labs => Some(0),
            Self::Preview => Some(1),
            Self::Beta => Some(2),
            Self::Stable => Some(3),
            Self::Deprecated | Self::DisabledByPolicy | Self::Retired => None,
        }
    }
}

/// Disable and kill-switch source classes in precedence order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum KillSwitchSourceClass {
    /// Emergency security or safety response.
    #[serde(rename = "emergency_security_response")]
    EmergencySecurityResponse,
    /// Signed admin policy ceiling.
    #[serde(rename = "admin_policy_ceiling")]
    AdminPolicyCeiling,
    /// Release-channel or rollout-plan override.
    #[serde(rename = "release_channel_or_rollout_override")]
    ReleaseChannelOrRolloutOverride,
    /// Cohort or ring assignment.
    #[serde(rename = "cohort_or_ring_assignment")]
    CohortOrRingAssignment,
    /// User opt-in or local preview setting.
    #[serde(rename = "user_opt_in_or_local_preview_toggle")]
    UserOptInOrLocalPreviewToggle,
}

impl KillSwitchSourceClass {
    /// Returns the stable snake-case token used by exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmergencySecurityResponse => "emergency_security_response",
            Self::AdminPolicyCeiling => "admin_policy_ceiling",
            Self::ReleaseChannelOrRolloutOverride => "release_channel_or_rollout_override",
            Self::CohortOrRingAssignment => "cohort_or_ring_assignment",
            Self::UserOptInOrLocalPreviewToggle => "user_opt_in_or_local_preview_toggle",
        }
    }
}

/// Effect a dependency marker has on the parent capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DependencyEffectOnParent {
    /// Marker declares saved-artifact dependency without changing state.
    #[serde(rename = "declares_artifact_dependency")]
    DeclaresArtifactDependency,
    /// Marker narrows the effective lifecycle to the dependency state.
    #[serde(rename = "narrows_lifecycle")]
    NarrowsLifecycle,
    /// Marker blocks the parent capability entirely.
    #[serde(rename = "gates_capability")]
    GatesCapability,
}

impl DependencyEffectOnParent {
    /// Returns the stable snake-case token used by exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DeclaresArtifactDependency => "declares_artifact_dependency",
            Self::NarrowsLifecycle => "narrows_lifecycle",
            Self::GatesCapability => "gates_capability",
        }
    }
}

/// One named consumer declared by the inventory artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InventoryConsumer {
    /// Stable consumer id.
    pub consumer_id: String,
    /// Surface family for the consumer.
    pub surface_class: String,
    /// Source file or artifact that consumes the inventory.
    pub consumer_ref: String,
    /// Commands that exercise the consumer, when applicable.
    #[serde(default)]
    pub commands: Vec<String>,
    /// Fields this consumer reads from the inventory.
    #[serde(default)]
    pub consumed_fields: Vec<String>,
}

/// Enrollment and rollout context for one capability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentEnrollment {
    /// Enrollment scope such as user, workspace, org, or ring.
    pub scope: String,
    /// Enabled state token.
    pub enabled_state: String,
    /// Source that controls enrollment.
    pub source: String,
    /// Cohort or ring label rendered by inventory consumers.
    pub cohort_or_ring: String,
    /// User-facing label for the enrolled capability.
    pub public_label: String,
}

/// One possible kill-switch or policy-disable source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisableSource {
    /// Source class used to apply kill-switch precedence.
    pub source_class: KillSwitchSourceClass,
    /// Stable source ref for diagnostics and support exports.
    pub source_ref: String,
    /// True when this source currently fires.
    pub active: bool,
    /// Copy-safe disable or narrowing reason.
    pub reason: String,
    /// Last refresh timestamp supplied by the source.
    pub last_refreshed_at: String,
    /// True when durable user-authored data remains preserved.
    pub preserve_user_data: bool,
    /// Scope of data preserved while the capability is disabled.
    pub preserved_data_scope: String,
    /// Bounded fallback or recovery path.
    pub fallback_path: String,
}

/// One capability row in the experiments inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentCapabilityRow {
    /// Stable capability id.
    pub capability_id: String,
    /// Human-readable capability title.
    pub title: String,
    /// Target workflow or persona supported by the capability.
    pub target_workflow: String,
    /// Owning person or team ref.
    pub owner: String,
    /// Declared lifecycle state before live markers and disables apply.
    pub declared_lifecycle_state: CapabilityLifecycleState,
    /// Effective lifecycle state after markers and disables apply.
    pub effective_lifecycle_state: CapabilityLifecycleState,
    /// Default posture token.
    pub default_posture: String,
    /// Support promise token.
    pub support_promise: String,
    /// Next review date or expiry date in `YYYY-MM-DD` form.
    pub review_or_expiry_date: String,
    /// Enrollment and cohort/ring context.
    pub enrollment: ExperimentEnrollment,
    /// Possible policy or kill-switch sources.
    #[serde(default)]
    pub disable_sources: Vec<DisableSource>,
    /// Dependency marker ids that must render with this row.
    #[serde(default)]
    pub dependency_marker_ids: Vec<String>,
    /// Source refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// True when this row is part of visible alpha inventory.
    pub claimed_alpha_visible: bool,
}

/// Dependency marker carried by saved artifacts and exports.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityDependencyMarker {
    /// Stable marker id.
    pub marker_id: String,
    /// Parent capability id.
    pub parent_capability_id: String,
    /// Saved or exported artifact class.
    pub artifact_class: String,
    /// Copy-safe artifact ref.
    pub artifact_ref: String,
    /// Required capability id.
    pub required_capability_id: String,
    /// Required lifecycle state for the dependent artifact.
    pub required_lifecycle_state: CapabilityLifecycleState,
    /// Effect on the parent row.
    pub effect_on_parent: DependencyEffectOnParent,
    /// Stable reason code.
    pub reason_code: String,
    /// Warning shown before downgrade/import/headless apply narrows behavior.
    pub warning_on_missing: String,
    /// Fallback or recovery path when the dependency is missing.
    pub fallback_path: String,
    /// Export visibility token.
    pub export_visibility: String,
    /// Copy-safe disclosure summary.
    pub disclosure_summary: String,
}

/// Checked-in experiments inventory artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsInventory {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable inventory id.
    pub inventory_id: String,
    /// Source artifact ref.
    pub source_inventory_ref: String,
    /// Date the inventory was generated or reviewed.
    pub as_of: String,
    /// Inventory owner.
    pub owner: String,
    /// Source contracts this inventory composes.
    #[serde(default)]
    pub source_contract_refs: Vec<String>,
    /// Controlled lifecycle vocabulary.
    pub lifecycle_vocabulary: Vec<CapabilityLifecycleState>,
    /// Lifecycle states the inventory must exercise.
    pub required_lifecycle_state_coverage: Vec<CapabilityLifecycleState>,
    /// Disable-source precedence from highest to lowest.
    pub kill_switch_precedence: Vec<KillSwitchSourceClass>,
    /// Declared consumers.
    #[serde(default)]
    pub named_consumers: Vec<InventoryConsumer>,
    /// Capability rows.
    pub rows: Vec<ExperimentCapabilityRow>,
    /// Dependency markers carried by saved artifacts and exports.
    #[serde(default)]
    pub dependency_markers: Vec<CapabilityDependencyMarker>,
}

/// Errors returned while parsing or validating the experiments inventory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExperimentsInventoryError {
    /// The YAML artifact could not be parsed.
    ParseFailed { detail: String },
    /// The schema version is not supported.
    UnsupportedSchemaVersion { actual: u32 },
    /// The lifecycle vocabulary does not match the controlled set.
    LifecycleVocabularyMismatch,
    /// The kill-switch precedence does not match the controlled order.
    KillSwitchPrecedenceMismatch,
    /// A date field is not `YYYY-MM-DD`.
    InvalidDate { field: String, value: String },
    /// A capability id appears more than once.
    DuplicateCapabilityId { capability_id: String },
    /// A dependency marker id appears more than once.
    DuplicateMarkerId { marker_id: String },
    /// A row references a marker that does not exist.
    UnknownDependencyMarker {
        capability_id: String,
        marker_id: String,
    },
    /// A marker points to a missing capability.
    UnknownMarkerParent {
        marker_id: String,
        parent_capability_id: String,
    },
    /// A marker is not listed by its parent row.
    MarkerNotRenderedByParent {
        marker_id: String,
        parent_capability_id: String,
    },
    /// A required lifecycle state has no visible row.
    MissingLifecycleCoverage { state: CapabilityLifecycleState },
    /// The checked-in effective state does not match derived runtime truth.
    EffectiveLifecycleMismatch {
        capability_id: String,
        expected: CapabilityLifecycleState,
        actual: CapabilityLifecycleState,
    },
    /// A disabled row does not expose an active disable source.
    DisabledRowMissingActiveSource { capability_id: String },
    /// An active disable source lacks preserved-data or fallback guidance.
    DisableSourceMissingRecovery { capability_id: String },
}

impl std::fmt::Display for ExperimentsInventoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseFailed { detail } => write!(f, "experiments inventory parse failed: {detail}"),
            Self::UnsupportedSchemaVersion { actual } => write!(
                f,
                "experiments inventory schema_version {actual} is not supported"
            ),
            Self::LifecycleVocabularyMismatch => {
                write!(f, "experiments inventory lifecycle vocabulary mismatch")
            }
            Self::KillSwitchPrecedenceMismatch => {
                write!(f, "experiments inventory kill-switch precedence mismatch")
            }
            Self::InvalidDate { field, value } => {
                write!(f, "experiments inventory {field} must be YYYY-MM-DD, got {value:?}")
            }
            Self::DuplicateCapabilityId { capability_id } => {
                write!(f, "duplicate experiments inventory capability id {capability_id:?}")
            }
            Self::DuplicateMarkerId { marker_id } => {
                write!(f, "duplicate experiments inventory marker id {marker_id:?}")
            }
            Self::UnknownDependencyMarker {
                capability_id,
                marker_id,
            } => write!(
                f,
                "capability {capability_id:?} references unknown dependency marker {marker_id:?}"
            ),
            Self::UnknownMarkerParent {
                marker_id,
                parent_capability_id,
            } => write!(
                f,
                "dependency marker {marker_id:?} references unknown parent capability {parent_capability_id:?}"
            ),
            Self::MarkerNotRenderedByParent {
                marker_id,
                parent_capability_id,
            } => write!(
                f,
                "dependency marker {marker_id:?} is not rendered by parent {parent_capability_id:?}"
            ),
            Self::MissingLifecycleCoverage { state } => write!(
                f,
                "experiments inventory does not cover lifecycle state {}",
                state.as_str()
            ),
            Self::EffectiveLifecycleMismatch {
                capability_id,
                expected,
                actual,
            } => write!(
                f,
                "capability {capability_id:?} effective lifecycle should be {}, got {}",
                expected.as_str(),
                actual.as_str()
            ),
            Self::DisabledRowMissingActiveSource { capability_id } => write!(
                f,
                "capability {capability_id:?} is disabled but has no active disable source"
            ),
            Self::DisableSourceMissingRecovery { capability_id } => write!(
                f,
                "capability {capability_id:?} has an active disable source without preserved-data and fallback guidance"
            ),
        }
    }
}

impl std::error::Error for ExperimentsInventoryError {}

/// Winning disable source projected into inspection and export records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DisableSourceInspection {
    /// Source class token.
    pub source_class: String,
    /// Stable source ref.
    pub source_ref: String,
    /// Disable reason.
    pub reason: String,
    /// Last refresh timestamp.
    pub last_refreshed_at: String,
    /// True when durable user-authored data is preserved.
    pub preserve_user_data: bool,
    /// Scope of preserved data.
    pub preserved_data_scope: String,
    /// Fallback or recovery path.
    pub fallback_path: String,
}

impl DisableSourceInspection {
    fn from_source(source: &DisableSource) -> Self {
        Self {
            source_class: source.source_class.as_str().to_owned(),
            source_ref: source.source_ref.clone(),
            reason: source.reason.clone(),
            last_refreshed_at: source.last_refreshed_at.clone(),
            preserve_user_data: source.preserve_user_data,
            preserved_data_scope: source.preserved_data_scope.clone(),
            fallback_path: source.fallback_path.clone(),
        }
    }
}

/// Dependency marker projected into row inspections.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyMarkerInspection {
    /// Stable marker id.
    pub marker_id: String,
    /// Artifact class that carries the dependency marker.
    pub artifact_class: String,
    /// Copy-safe artifact ref.
    pub artifact_ref: String,
    /// Required capability id.
    pub required_capability_id: String,
    /// Required lifecycle state token.
    pub required_lifecycle_state: String,
    /// Effect token.
    pub effect_on_parent: String,
    /// Stable reason code.
    pub reason_code: String,
    /// Copy-safe disclosure summary.
    pub disclosure_summary: String,
    /// Fallback or recovery path.
    pub fallback_path: String,
}

impl DependencyMarkerInspection {
    fn from_marker(marker: &CapabilityDependencyMarker) -> Self {
        Self {
            marker_id: marker.marker_id.clone(),
            artifact_class: marker.artifact_class.clone(),
            artifact_ref: marker.artifact_ref.clone(),
            required_capability_id: marker.required_capability_id.clone(),
            required_lifecycle_state: marker.required_lifecycle_state.as_str().to_owned(),
            effect_on_parent: marker.effect_on_parent.as_str().to_owned(),
            reason_code: marker.reason_code.clone(),
            disclosure_summary: marker.disclosure_summary.clone(),
            fallback_path: marker.fallback_path.clone(),
        }
    }
}

/// Row-level inspection record consumed by CLI, diagnostics, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsInventoryRowInspection {
    /// Stable capability id.
    pub capability_id: String,
    /// Human-readable title.
    pub title: String,
    /// Target workflow.
    pub target_workflow: String,
    /// Owner ref.
    pub owner: String,
    /// Declared lifecycle state token.
    pub declared_lifecycle_state: String,
    /// Effective lifecycle state token.
    pub effective_lifecycle_state: String,
    /// Default posture token.
    pub default_posture: String,
    /// Support promise token.
    pub support_promise: String,
    /// Review or expiry date.
    pub review_or_expiry_date: String,
    /// Enrollment scope.
    pub enrollment_scope: String,
    /// Enrollment source.
    pub enrollment_source: String,
    /// Cohort or ring.
    pub cohort_or_ring: String,
    /// Public label.
    pub public_label: String,
    /// True when any saved or exported artifact depends on this capability.
    pub saved_artifact_dependency_present: bool,
    /// Number of dependency markers attached to the row.
    pub artifact_dependency_count: usize,
    /// Artifact classes attached to this row.
    pub artifact_classes: Vec<String>,
    /// Winning kill-switch or policy-disable source.
    pub winning_disable_source: Option<DisableSourceInspection>,
    /// Dependency markers attached to this row.
    pub dependency_markers: Vec<DependencyMarkerInspection>,
}

/// Full inspection record for the experiments inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsInventoryInspectionRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref consumed by settings, diagnostics, and support export.
    pub shared_contract_ref: String,
    /// Source inventory artifact ref.
    pub source_inventory_ref: String,
    /// Inventory id.
    pub inventory_id: String,
    /// Inventory as-of date.
    pub as_of: String,
    /// Lifecycle counts keyed by controlled token.
    pub lifecycle_counts: BTreeMap<String, usize>,
    /// Number of rows with active policy or kill-switch disable.
    pub disabled_or_retired_count: usize,
    /// Number of dependency markers carried by saved artifacts or exports.
    pub artifact_dependency_marker_count: usize,
    /// Row inspections.
    pub rows: Vec<ExperimentsInventoryRowInspection>,
    /// Copy-safe warnings for downgrade/import/headless flows.
    pub artifact_dependency_warnings: Vec<ArtifactDependencyWarning>,
}

/// Copy-safe warning for an artifact that depends on a capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDependencyWarning {
    /// Stable warning record discriminator.
    pub record_kind: String,
    /// Source marker id.
    pub marker_id: String,
    /// Parent capability id.
    pub parent_capability_id: String,
    /// Artifact class.
    pub artifact_class: String,
    /// Copy-safe artifact ref.
    pub artifact_ref: String,
    /// Required capability id.
    pub required_capability_id: String,
    /// Required lifecycle state token.
    pub required_lifecycle_state: String,
    /// Warning shown before behavior narrows or disappears.
    pub warning: String,
    /// Bounded fallback or recovery path.
    pub fallback_path: String,
    /// Export visibility token.
    pub export_visibility: String,
}

impl ArtifactDependencyWarning {
    fn from_marker(marker: &CapabilityDependencyMarker) -> Self {
        Self {
            record_kind: "artifact_dependency_warning".to_owned(),
            marker_id: marker.marker_id.clone(),
            parent_capability_id: marker.parent_capability_id.clone(),
            artifact_class: marker.artifact_class.clone(),
            artifact_ref: marker.artifact_ref.clone(),
            required_capability_id: marker.required_capability_id.clone(),
            required_lifecycle_state: marker.required_lifecycle_state.as_str().to_owned(),
            warning: marker.warning_on_missing.clone(),
            fallback_path: marker.fallback_path.clone(),
            export_visibility: marker.export_visibility.clone(),
        }
    }
}

/// CLI projection over the experiments inventory inspection record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsInventoryCliProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source inventory artifact ref.
    pub source_inventory_ref: String,
    /// Deterministic summary fields.
    pub fields: BTreeMap<String, String>,
    /// Row summaries for human and machine-readable CLI output.
    pub rows: Vec<ExperimentsInventoryCliRow>,
}

/// One CLI row in the experiments inventory projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsInventoryCliRow {
    /// Stable capability id.
    pub capability_id: String,
    /// Effective lifecycle state token.
    pub effective_lifecycle_state: String,
    /// Owner ref.
    pub owner: String,
    /// Cohort or ring.
    pub cohort_or_ring: String,
    /// Review or expiry date.
    pub review_or_expiry_date: String,
    /// Winning disable source class, if any.
    pub winning_disable_source: Option<String>,
    /// True when saved/exported artifacts carry dependency markers.
    pub saved_artifact_dependency_present: bool,
    /// Number of dependency markers attached to the row.
    pub artifact_dependency_count: usize,
    /// Fallback path for disabled or dependency-narrowed rows.
    pub fallback_path: Option<String>,
}

/// Support-export projection over the experiments inventory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExperimentsInventorySupportExportProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export id supplied by the caller.
    pub export_id: String,
    /// Shared contract ref consumed by support tooling.
    pub shared_contract_ref: String,
    /// Source inventory artifact ref.
    pub source_inventory_ref: String,
    /// Inventory as-of date.
    pub as_of: String,
    /// Copy-safe row inspections.
    pub rows: Vec<ExperimentsInventoryRowInspection>,
    /// Copy-safe dependency warnings carried by saved artifacts and exports.
    pub artifact_dependency_warnings: Vec<ArtifactDependencyWarning>,
    /// Number of disabled or retired rows.
    pub disabled_or_retired_count: usize,
    /// Number of dependency markers.
    pub artifact_dependency_marker_count: usize,
}

/// Loads the checked-in experiments inventory artifact.
pub fn load_default_inventory() -> Result<ExperimentsInventory, ExperimentsInventoryError> {
    serde_yaml::from_str(DEFAULT_EXPERIMENTS_INVENTORY_YAML).map_err(|err| {
        ExperimentsInventoryError::ParseFailed {
            detail: err.to_string(),
        }
    })
}

/// Validates and materializes the checked-in experiments inventory.
pub fn inspect_default_inventory(
) -> Result<ExperimentsInventoryInspectionRecord, ExperimentsInventoryError> {
    let inventory = load_default_inventory()?;
    inspect_inventory(&inventory)
}

/// Validates an inventory without materializing projections.
pub fn validate_inventory(
    inventory: &ExperimentsInventory,
) -> Result<(), ExperimentsInventoryError> {
    if inventory.schema_version != EXPERIMENTS_INVENTORY_SCHEMA_VERSION {
        return Err(ExperimentsInventoryError::UnsupportedSchemaVersion {
            actual: inventory.schema_version,
        });
    }
    if inventory.lifecycle_vocabulary != expected_lifecycle_vocabulary() {
        return Err(ExperimentsInventoryError::LifecycleVocabularyMismatch);
    }
    if inventory.required_lifecycle_state_coverage != expected_lifecycle_vocabulary() {
        return Err(ExperimentsInventoryError::LifecycleVocabularyMismatch);
    }
    if inventory.kill_switch_precedence != expected_kill_switch_precedence() {
        return Err(ExperimentsInventoryError::KillSwitchPrecedenceMismatch);
    }
    validate_date("as_of", &inventory.as_of)?;

    let mut row_ids = BTreeSet::new();
    let mut state_coverage = BTreeSet::new();
    for row in &inventory.rows {
        if !row_ids.insert(row.capability_id.clone()) {
            return Err(ExperimentsInventoryError::DuplicateCapabilityId {
                capability_id: row.capability_id.clone(),
            });
        }
        validate_date(
            &format!("rows[{}].review_or_expiry_date", row.capability_id),
            &row.review_or_expiry_date,
        )?;
        state_coverage.insert(row.effective_lifecycle_state);
        for source in &row.disable_sources {
            if source.active
                && (!source.preserve_user_data
                    || source.preserved_data_scope.trim().is_empty()
                    || source.fallback_path.trim().is_empty())
            {
                return Err(ExperimentsInventoryError::DisableSourceMissingRecovery {
                    capability_id: row.capability_id.clone(),
                });
            }
        }
    }

    let mut markers = BTreeMap::new();
    for marker in &inventory.dependency_markers {
        if markers.insert(marker.marker_id.clone(), marker).is_some() {
            return Err(ExperimentsInventoryError::DuplicateMarkerId {
                marker_id: marker.marker_id.clone(),
            });
        }
        if !row_ids.contains(&marker.parent_capability_id) {
            return Err(ExperimentsInventoryError::UnknownMarkerParent {
                marker_id: marker.marker_id.clone(),
                parent_capability_id: marker.parent_capability_id.clone(),
            });
        }
    }

    for row in &inventory.rows {
        let row_markers = row_markers(row, &markers)?;
        let derived = derive_effective_lifecycle_state(row, &row_markers, inventory);
        if derived != row.effective_lifecycle_state {
            return Err(ExperimentsInventoryError::EffectiveLifecycleMismatch {
                capability_id: row.capability_id.clone(),
                expected: derived,
                actual: row.effective_lifecycle_state,
            });
        }
        if matches!(
            row.effective_lifecycle_state,
            CapabilityLifecycleState::DisabledByPolicy
        ) && winning_disable_source(row, &inventory.kill_switch_precedence).is_none()
        {
            return Err(ExperimentsInventoryError::DisabledRowMissingActiveSource {
                capability_id: row.capability_id.clone(),
            });
        }
    }

    for state in &inventory.required_lifecycle_state_coverage {
        if !state_coverage.contains(state) {
            return Err(ExperimentsInventoryError::MissingLifecycleCoverage { state: *state });
        }
    }

    Ok(())
}

/// Materializes the export-safe inventory inspection record.
pub fn inspect_inventory(
    inventory: &ExperimentsInventory,
) -> Result<ExperimentsInventoryInspectionRecord, ExperimentsInventoryError> {
    validate_inventory(inventory)?;

    let markers = inventory
        .dependency_markers
        .iter()
        .map(|marker| (marker.marker_id.clone(), marker))
        .collect::<BTreeMap<_, _>>();
    let mut lifecycle_counts = BTreeMap::new();
    let mut rows = Vec::new();

    for row in &inventory.rows {
        let row_markers = row_markers(row, &markers)?;
        let dependency_markers = row_markers
            .iter()
            .map(|marker| DependencyMarkerInspection::from_marker(marker))
            .collect::<Vec<_>>();
        let artifact_classes = row_markers
            .iter()
            .map(|marker| marker.artifact_class.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        *lifecycle_counts
            .entry(row.effective_lifecycle_state.as_str().to_owned())
            .or_insert(0) += 1;
        rows.push(ExperimentsInventoryRowInspection {
            capability_id: row.capability_id.clone(),
            title: row.title.clone(),
            target_workflow: row.target_workflow.clone(),
            owner: row.owner.clone(),
            declared_lifecycle_state: row.declared_lifecycle_state.as_str().to_owned(),
            effective_lifecycle_state: row.effective_lifecycle_state.as_str().to_owned(),
            default_posture: row.default_posture.clone(),
            support_promise: row.support_promise.clone(),
            review_or_expiry_date: row.review_or_expiry_date.clone(),
            enrollment_scope: row.enrollment.scope.clone(),
            enrollment_source: row.enrollment.source.clone(),
            cohort_or_ring: row.enrollment.cohort_or_ring.clone(),
            public_label: row.enrollment.public_label.clone(),
            saved_artifact_dependency_present: !row_markers.is_empty(),
            artifact_dependency_count: row_markers.len(),
            artifact_classes,
            winning_disable_source: winning_disable_source(row, &inventory.kill_switch_precedence)
                .map(DisableSourceInspection::from_source),
            dependency_markers,
        });
    }

    let artifact_dependency_warnings = inventory
        .dependency_markers
        .iter()
        .map(ArtifactDependencyWarning::from_marker)
        .collect::<Vec<_>>();
    let disabled_or_retired_count = rows
        .iter()
        .filter(|row| {
            matches!(
                row.effective_lifecycle_state.as_str(),
                "DisabledByPolicy" | "Retired"
            )
        })
        .count();

    Ok(ExperimentsInventoryInspectionRecord {
        record_kind: "experiments_inventory_inspection_record".to_owned(),
        schema_version: EXPERIMENTS_INVENTORY_SCHEMA_VERSION,
        shared_contract_ref: SHARED_CONTRACT_REF.to_owned(),
        source_inventory_ref: inventory.source_inventory_ref.clone(),
        inventory_id: inventory.inventory_id.clone(),
        as_of: inventory.as_of.clone(),
        lifecycle_counts,
        disabled_or_retired_count,
        artifact_dependency_marker_count: inventory.dependency_markers.len(),
        rows,
        artifact_dependency_warnings,
    })
}

/// Builds the CLI projection for one inventory inspection record.
pub fn project_cli_inventory(
    record: &ExperimentsInventoryInspectionRecord,
) -> ExperimentsInventoryCliProjection {
    let mut fields = BTreeMap::new();
    fields.insert("inventory_id".to_owned(), record.inventory_id.clone());
    fields.insert("as_of".to_owned(), record.as_of.clone());
    fields.insert("row_count".to_owned(), record.rows.len().to_string());
    fields.insert(
        "disabled_or_retired_count".to_owned(),
        record.disabled_or_retired_count.to_string(),
    );
    fields.insert(
        "artifact_dependency_marker_count".to_owned(),
        record.artifact_dependency_marker_count.to_string(),
    );

    let rows = record
        .rows
        .iter()
        .map(|row| ExperimentsInventoryCliRow {
            capability_id: row.capability_id.clone(),
            effective_lifecycle_state: row.effective_lifecycle_state.clone(),
            owner: row.owner.clone(),
            cohort_or_ring: row.cohort_or_ring.clone(),
            review_or_expiry_date: row.review_or_expiry_date.clone(),
            winning_disable_source: row
                .winning_disable_source
                .as_ref()
                .map(|source| source.source_class.clone()),
            saved_artifact_dependency_present: row.saved_artifact_dependency_present,
            artifact_dependency_count: row.artifact_dependency_count,
            fallback_path: row
                .winning_disable_source
                .as_ref()
                .map(|source| source.fallback_path.clone())
                .or_else(|| {
                    row.dependency_markers
                        .first()
                        .map(|marker| marker.fallback_path.clone())
                }),
        })
        .collect();

    ExperimentsInventoryCliProjection {
        record_kind: "experiments_inventory_cli_projection".to_owned(),
        schema_version: EXPERIMENTS_INVENTORY_SCHEMA_VERSION,
        source_inventory_ref: record.source_inventory_ref.clone(),
        fields,
        rows,
    }
}

/// Builds a support-export projection from the inventory inspection record.
pub fn project_support_export(
    export_id: impl Into<String>,
    record: &ExperimentsInventoryInspectionRecord,
) -> ExperimentsInventorySupportExportProjection {
    ExperimentsInventorySupportExportProjection {
        record_kind: "experiments_inventory_support_export_projection".to_owned(),
        schema_version: EXPERIMENTS_INVENTORY_SCHEMA_VERSION,
        export_id: export_id.into(),
        shared_contract_ref: record.shared_contract_ref.clone(),
        source_inventory_ref: record.source_inventory_ref.clone(),
        as_of: record.as_of.clone(),
        rows: record.rows.clone(),
        artifact_dependency_warnings: record.artifact_dependency_warnings.clone(),
        disabled_or_retired_count: record.disabled_or_retired_count,
        artifact_dependency_marker_count: record.artifact_dependency_marker_count,
    }
}

fn expected_lifecycle_vocabulary() -> Vec<CapabilityLifecycleState> {
    vec![
        CapabilityLifecycleState::Labs,
        CapabilityLifecycleState::Preview,
        CapabilityLifecycleState::Beta,
        CapabilityLifecycleState::Stable,
        CapabilityLifecycleState::Deprecated,
        CapabilityLifecycleState::DisabledByPolicy,
        CapabilityLifecycleState::Retired,
    ]
}

fn expected_kill_switch_precedence() -> Vec<KillSwitchSourceClass> {
    vec![
        KillSwitchSourceClass::EmergencySecurityResponse,
        KillSwitchSourceClass::AdminPolicyCeiling,
        KillSwitchSourceClass::ReleaseChannelOrRolloutOverride,
        KillSwitchSourceClass::CohortOrRingAssignment,
        KillSwitchSourceClass::UserOptInOrLocalPreviewToggle,
    ]
}

fn validate_date(field: &str, value: &str) -> Result<(), ExperimentsInventoryError> {
    let bytes = value.as_bytes();
    let valid = bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(idx, byte)| idx == 4 || idx == 7 || byte.is_ascii_digit());
    if valid {
        Ok(())
    } else {
        Err(ExperimentsInventoryError::InvalidDate {
            field: field.to_owned(),
            value: value.to_owned(),
        })
    }
}

fn row_markers<'a>(
    row: &ExperimentCapabilityRow,
    markers: &'a BTreeMap<String, &'a CapabilityDependencyMarker>,
) -> Result<Vec<&'a CapabilityDependencyMarker>, ExperimentsInventoryError> {
    let mut out = Vec::new();
    for marker_id in &row.dependency_marker_ids {
        let marker = markers.get(marker_id).copied().ok_or_else(|| {
            ExperimentsInventoryError::UnknownDependencyMarker {
                capability_id: row.capability_id.clone(),
                marker_id: marker_id.clone(),
            }
        })?;
        if marker.parent_capability_id != row.capability_id {
            return Err(ExperimentsInventoryError::MarkerNotRenderedByParent {
                marker_id: marker.marker_id.clone(),
                parent_capability_id: marker.parent_capability_id.clone(),
            });
        }
        out.push(marker);
    }

    for marker in markers.values() {
        if marker.parent_capability_id == row.capability_id
            && !row.dependency_marker_ids.contains(&marker.marker_id)
        {
            return Err(ExperimentsInventoryError::MarkerNotRenderedByParent {
                marker_id: marker.marker_id.clone(),
                parent_capability_id: row.capability_id.clone(),
            });
        }
    }
    Ok(out)
}

fn derive_effective_lifecycle_state(
    row: &ExperimentCapabilityRow,
    markers: &[&CapabilityDependencyMarker],
    inventory: &ExperimentsInventory,
) -> CapabilityLifecycleState {
    if matches!(
        row.declared_lifecycle_state,
        CapabilityLifecycleState::Retired
    ) {
        return CapabilityLifecycleState::Retired;
    }
    if matches!(
        row.declared_lifecycle_state,
        CapabilityLifecycleState::DisabledByPolicy
    ) {
        return CapabilityLifecycleState::DisabledByPolicy;
    }
    if winning_disable_source(row, &inventory.kill_switch_precedence).is_some() {
        return CapabilityLifecycleState::DisabledByPolicy;
    }

    let mut state = row.declared_lifecycle_state;
    for marker in markers {
        state = match marker.effect_on_parent {
            DependencyEffectOnParent::DeclaresArtifactDependency => state,
            DependencyEffectOnParent::GatesCapability => match marker.required_lifecycle_state {
                CapabilityLifecycleState::Retired => CapabilityLifecycleState::Retired,
                _ => CapabilityLifecycleState::DisabledByPolicy,
            },
            DependencyEffectOnParent::NarrowsLifecycle => {
                narrower_lifecycle_state(state, marker.required_lifecycle_state)
            }
        };
    }
    state
}

fn narrower_lifecycle_state(
    current: CapabilityLifecycleState,
    dependency: CapabilityLifecycleState,
) -> CapabilityLifecycleState {
    match (current.maturity_rank(), dependency.maturity_rank()) {
        (Some(current_rank), Some(dependency_rank)) if dependency_rank < current_rank => dependency,
        _ => current,
    }
}

fn winning_disable_source<'a>(
    row: &'a ExperimentCapabilityRow,
    precedence: &[KillSwitchSourceClass],
) -> Option<&'a DisableSource> {
    row.disable_sources
        .iter()
        .filter(|source| source.active)
        .min_by_key(|source| {
            precedence
                .iter()
                .position(|class| *class == source.source_class)
                .unwrap_or(usize::MAX)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_inventory_validates_and_covers_lifecycle_states() {
        let inventory = load_default_inventory().expect("inventory parses");
        validate_inventory(&inventory).expect("inventory validates");
        let inspection = inspect_inventory(&inventory).expect("inspection builds");

        for state in expected_lifecycle_vocabulary() {
            assert!(
                inspection.lifecycle_counts.contains_key(state.as_str()),
                "missing lifecycle count for {}",
                state.as_str()
            );
        }
        assert_eq!(inspection.rows.len(), 7);
        assert_eq!(inspection.artifact_dependency_marker_count, 8);
    }

    #[test]
    fn kill_switch_precedence_prefers_admin_policy_over_user_toggle() {
        let inspection = inspect_default_inventory().expect("inspection builds");
        let row = inspection
            .rows
            .iter()
            .find(|row| row.capability_id == "alpha.managed_cloud_daily_driver")
            .expect("managed cloud row");
        let source = row
            .winning_disable_source
            .as_ref()
            .expect("winning disable source");

        assert_eq!(row.effective_lifecycle_state, "DisabledByPolicy");
        assert_eq!(source.source_class, "admin_policy_ceiling");
        assert!(source.preserve_user_data);
        assert!(source.fallback_path.contains("Continue locally"));
    }

    #[test]
    fn support_export_carries_dependency_warnings() {
        let inspection = inspect_default_inventory().expect("inspection builds");
        let export = project_support_export("support-export:experiments:alpha", &inspection);

        assert_eq!(export.shared_contract_ref, SHARED_CONTRACT_REF);
        assert_eq!(
            export.artifact_dependency_warnings.len(),
            inspection.artifact_dependency_marker_count
        );
        assert!(export.artifact_dependency_warnings.iter().any(|warning| {
            warning.artifact_class == "settings_profile"
                && warning.required_lifecycle_state == "Retired"
                && warning.warning.contains("refuse to reactivate")
        }));
    }

    #[test]
    fn cli_projection_names_rollout_visibility() {
        let inspection = inspect_default_inventory().expect("inspection builds");
        let cli = project_cli_inventory(&inspection);
        let row = cli
            .rows
            .iter()
            .find(|row| row.capability_id == "settings.sync_conflict_review")
            .expect("sync row");

        assert_eq!(cli.fields.get("row_count").map(String::as_str), Some("7"));
        assert_eq!(row.effective_lifecycle_state, "Beta");
        assert_eq!(row.cohort_or_ring, "design_partner_beta_seed");
        assert!(row.saved_artifact_dependency_present);
    }
}
