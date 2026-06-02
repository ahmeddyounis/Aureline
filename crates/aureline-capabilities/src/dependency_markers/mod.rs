//! Capability records and artifact dependency markers.
//!
//! The two record kinds shared by every capability-sensitive artifact
//! the product persists:
//!
//! 1. [`CapabilityRecord`] describes one capability the artifact may
//!    depend on. It carries the stable [`capability_id`], the
//!    [`CapabilityLifecycleState`], the [`SupportPromise`], the
//!    [`DependencyClass`] (Labs / Preview / Beta-only / policy-gated /
//!    host-specific), and the typed [`EffectOnImport`] the artifact
//!    publishes when the dependency is missing on a target.
//! 2. [`ArtifactDependencyMarker`] is the per-artifact persisted
//!    marker that travels inside the artifact bytes when behavior
//!    narrows or portability changes. The marker reuses the same
//!    [`DependencyClass`], [`SupportPromise`], and
//!    [`EffectOnImport`] vocabulary as the [`CapabilityRecord`] so
//!    settings inspectors, import-review sheets, bundle detail pages,
//!    downgrade flows, headless / CLI inspect output, and docs /
//!    help pages read one warning vocabulary.
//!
//! See [`project_marker_for_host_surface`] for the projection that
//! consumers call to render the marker on a specific surface, and
//! [`validate_artifact_markers`] for the cross-record audit a
//! producer runs before it ships an artifact.

pub mod downgrade;
pub mod lifecycle;
pub mod transport_lanes;

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

pub use downgrade::{
    assert_downgrade_review_sheets, evaluate_downgrade, scenario_target_state, support_rank,
    CompareApplyReviewSheet, DowngradeAudit, DowngradeReviewDefect, DowngradeScenario,
    TargetCapabilityState,
};
pub use lifecycle::CapabilityLifecycleState;
pub use transport_lanes::{
    assert_marker_survives_all_lanes, replay_marker_through_all_lanes, replay_marker_through_lane,
    LaneReplayAudit, LaneReplayDefect, LaneReplayOutcome, LaneReplaySheet, TransportLane,
};

/// Schema version pinned into every persisted [`CapabilityRecord`].
pub const CAPABILITY_RECORD_SCHEMA_VERSION: u32 = 1;

/// Schema version pinned into every persisted
/// [`ArtifactDependencyMarker`].
pub const ARTIFACT_DEPENDENCY_MARKER_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every [`CapabilityRecord`].
pub const CAPABILITY_RECORD_SHARED_CONTRACT_REF: &str = "capabilities:capability_record:v1";

/// Shared contract ref consumed by every [`ArtifactDependencyMarker`].
pub const ARTIFACT_DEPENDENCY_MARKER_SHARED_CONTRACT_REF: &str =
    "capabilities:artifact_dependency_marker:v1";

/// Stable record kind for [`CapabilityRecord`] payloads.
pub const CAPABILITY_RECORD_KIND: &str = "capabilities_capability_record";

/// Stable record kind for [`ArtifactDependencyMarker`] payloads.
pub const ARTIFACT_DEPENDENCY_MARKER_KIND: &str = "capabilities_artifact_dependency_marker";

/// Why the artifact carries this dependency.
///
/// The closed vocabulary is the entire point of the marker: a surface
/// reading the artifact can branch on this enum without inspecting the
/// underlying capability registry. Adding a class is an additive bump
/// of [`ARTIFACT_DEPENDENCY_MARKER_SCHEMA_VERSION`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyClass {
    /// Opt-in Labs surface; the dependency is exploratory and may
    /// churn or vanish without notice.
    Labs,
    /// Named preview cohort; the dependency is opt-in but tracked for
    /// rollout.
    Preview,
    /// Beta-only capability; the dependency is broadly enabled but
    /// not yet stable.
    BetaOnly,
    /// Disabled or restricted by an admin policy, kill switch, or
    /// release-channel decision.
    PolicyGated,
    /// Available only on specific hosts (managed deployments,
    /// desktop-only surfaces, companion-only surfaces, CLI-only
    /// surfaces, sovereign deployments, and so on).
    HostSpecific,
}

impl DependencyClass {
    /// Stable snake_case token persisted in artifacts.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Labs => "labs",
            Self::Preview => "preview",
            Self::BetaOnly => "beta_only",
            Self::PolicyGated => "policy_gated",
            Self::HostSpecific => "host_specific",
        }
    }
}

/// Support promise the artifact recorded when the marker was minted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportPromise {
    /// No support promise; the dependency is best-effort.
    BestEffort,
    /// Community-supported only.
    CommunitySupported,
    /// Standard support; behavior is expected to remain stable
    /// within the channel.
    StandardSupport,
    /// Extended support; longer support window than the standard
    /// channel.
    ExtendedSupport,
    /// Operator / admin only; end-user UI cannot rely on it without
    /// admin grant.
    OperatorOnly,
    /// No support; explicitly excluded from any support pledge.
    NoSupport,
}

impl SupportPromise {
    /// Stable snake_case token persisted in artifacts.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BestEffort => "best_effort",
            Self::CommunitySupported => "community_supported",
            Self::StandardSupport => "standard_support",
            Self::ExtendedSupport => "extended_support",
            Self::OperatorOnly => "operator_only",
            Self::NoSupport => "no_support",
        }
    }
}

/// Effect on import when the dependency is missing on the target.
///
/// `EffectOnImport` is the typed contract surfaces use to render a
/// downgrade flow rather than silently dropping behavior. Generic
/// `"unavailable"` chips are forbidden: the producing artifact picks
/// one of these classes, and consumers must render it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectOnImport {
    /// Importing is blocked outright; user-authored data is
    /// preserved untouched and the artifact is held in a review
    /// sheet until the dependency is satisfied.
    BlockApplyPreserveData,
    /// Importing proceeds but behavior narrows on the target; the
    /// marker carries the fallback path to recover the original
    /// behavior.
    NarrowBehaviorPreserveData,
    /// Importing proceeds; the producing capability is silently
    /// emulated with a downgrade path on the target. Marker still
    /// renders so the user knows the downgrade happened.
    EmulatedDowngradePreserveData,
    /// Importing proceeds but the dependent payload is parked in a
    /// hold for the user to re-apply once the dependency becomes
    /// available. Original payload is preserved.
    HoldForLaterPreserveData,
    /// Importing proceeds; the dependent payload is rendered as a
    /// tombstone (read-only) for migration disclosure. User-authored
    /// data is preserved as the source of truth for the tombstone.
    RenderTombstonePreserveData,
}

impl EffectOnImport {
    /// Stable snake_case token persisted in artifacts.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockApplyPreserveData => "block_apply_preserve_data",
            Self::NarrowBehaviorPreserveData => "narrow_behavior_preserve_data",
            Self::EmulatedDowngradePreserveData => "emulated_downgrade_preserve_data",
            Self::HoldForLaterPreserveData => "hold_for_later_preserve_data",
            Self::RenderTombstonePreserveData => "render_tombstone_preserve_data",
        }
    }

    /// True for every variant in this enum. The constant exists so a
    /// validator can prove every effect preserves user-authored data:
    /// the closed vocabulary forbids any silent-drop variant.
    pub const fn preserves_user_data(self) -> bool {
        match self {
            Self::BlockApplyPreserveData
            | Self::NarrowBehaviorPreserveData
            | Self::EmulatedDowngradePreserveData
            | Self::HoldForLaterPreserveData
            | Self::RenderTombstonePreserveData => true,
        }
    }
}

/// What happens to dependent behavior when the capability is missing.
///
/// `BehaviorOnMissing` is the side of the marker that names the user
/// visible consequence, in non-jargon copy, paired with the typed
/// [`EffectOnImport`]. Surfaces use this for the "what changed"
/// paragraph; the typed effect drives the flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BehaviorOnMissing {
    /// Short reviewer-facing summary. Must be non-empty.
    pub summary: String,
    /// Bounded fallback / recover / dismiss path. Must be non-empty;
    /// surfaces use this as the inline recover affordance.
    pub fallback_path: String,
    /// Optional documentation ref the user can open for context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_ref: Option<String>,
}

/// One capability description.
///
/// A `CapabilityRecord` is the producer-side description an artifact
/// snapshots when it mints a marker. The artifact then carries the
/// marker; the record itself does not need to travel with every
/// artifact (the marker references the capability by id and pins the
/// vocabulary tokens).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRecord {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version pinned into the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable capability id.
    pub capability_id: String,
    /// Reviewer-facing capability title.
    pub title: String,
    /// Lifecycle state of the capability when the record was minted.
    pub lifecycle_state: CapabilityLifecycleState,
    /// Support promise of the capability.
    pub support_promise: SupportPromise,
    /// Dependency class users see when a marker for this capability
    /// is rendered on an artifact.
    pub dependency_class: DependencyClass,
    /// Default import-behavior class when the capability is missing
    /// on a target. Artifacts may override this on a per-marker
    /// basis when their specific behavior diverges.
    pub default_effect_on_import: EffectOnImport,
    /// Default fallback / recover guidance when the capability is
    /// missing. Per-marker fallback_path on
    /// [`ArtifactDependencyMarker`] overrides this when present.
    pub default_fallback_path: String,
    /// Optional docs ref the marker can carry forward.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_ref: Option<String>,
    /// When non-empty, the capability is restricted to the named
    /// hosts (e.g. `managed_admin_surface`, `desktop_product`).
    /// Renders alongside [`DependencyClass::HostSpecific`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_scope: Vec<String>,
}

impl CapabilityRecord {
    /// Constructs a [`CapabilityRecord`] with stable scaffolding.
    pub fn new(
        capability_id: impl Into<String>,
        title: impl Into<String>,
        lifecycle_state: CapabilityLifecycleState,
        support_promise: SupportPromise,
        dependency_class: DependencyClass,
        default_effect_on_import: EffectOnImport,
        default_fallback_path: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: CAPABILITY_RECORD_KIND.to_owned(),
            schema_version: CAPABILITY_RECORD_SCHEMA_VERSION,
            shared_contract_ref: CAPABILITY_RECORD_SHARED_CONTRACT_REF.to_owned(),
            capability_id: capability_id.into(),
            title: title.into(),
            lifecycle_state,
            support_promise,
            dependency_class,
            default_effect_on_import,
            default_fallback_path: default_fallback_path.into(),
            docs_ref: None,
            host_scope: Vec::new(),
        }
    }
}

/// Artifact classes that MUST persist [`ArtifactDependencyMarker`]
/// records when behavior narrows or portability changes on targets
/// lacking the dependency.
///
/// The list is closed by design and tracks the spec exactly:
/// settings exports, profiles, workflow bundles, portable-state
/// packages, recipes, saved views, migration packets, support
/// exports, and sync artifacts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactClass {
    /// Settings export package (effective values, overlays, lock
    /// state).
    SettingsExport,
    /// User or workspace profile.
    Profile,
    /// Workflow bundle (recorded macros, command compositions).
    WorkflowBundle,
    /// Portable state package (workspace-state snapshot, restore
    /// payload, handoff bundle).
    PortableStatePackage,
    /// Saved recipe (parameterized command + inputs).
    Recipe,
    /// Saved view (graph view, search view, navigation pin).
    SavedView,
    /// Migration packet (settings, profile, or capability migration
    /// payload).
    MigrationPacket,
    /// Support-export packet attached to a support bundle.
    SupportExport,
    /// Sync artifact (cross-device sync payload).
    SyncArtifact,
}

impl ArtifactClass {
    /// Stable snake_case token persisted in artifacts.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsExport => "settings_export",
            Self::Profile => "profile",
            Self::WorkflowBundle => "workflow_bundle",
            Self::PortableStatePackage => "portable_state_package",
            Self::Recipe => "recipe",
            Self::SavedView => "saved_view",
            Self::MigrationPacket => "migration_packet",
            Self::SupportExport => "support_export",
            Self::SyncArtifact => "sync_artifact",
        }
    }
}

/// Host surfaces that read the marker.
///
/// The closed list mirrors the spec's surface vocabulary:
/// settings inspectors, import-review sheets, bundle detail pages,
/// downgrade flows, headless / CLI inspect output, and docs / help
/// pages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HostSurface {
    /// Settings inspector pane.
    SettingsInspector,
    /// Import review sheet (settings, profile, bundle, recipe import
    /// review).
    ImportReviewSheet,
    /// Bundle detail page (workflow bundle, portable bundle).
    BundleDetailPage,
    /// Downgrade or fallback flow surface.
    DowngradeFlow,
    /// Headless / CLI inspect output.
    HeadlessCliInspect,
    /// Docs / help page that quotes the marker.
    DocsHelpPage,
}

impl HostSurface {
    /// Stable snake_case token persisted in projections.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SettingsInspector => "settings_inspector",
            Self::ImportReviewSheet => "import_review_sheet",
            Self::BundleDetailPage => "bundle_detail_page",
            Self::DowngradeFlow => "downgrade_flow",
            Self::HeadlessCliInspect => "headless_cli_inspect",
            Self::DocsHelpPage => "docs_help_page",
        }
    }
}

/// One per-artifact dependency marker.
///
/// The marker is the persisted payload that travels inside any
/// [`ArtifactClass`] when behavior narrows or portability changes on
/// targets that lack the capability. Producers mint it at save /
/// export / sync time; consumers read it before apply, before
/// downgrade, and during inspect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactDependencyMarker {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version pinned into the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by every surface.
    pub shared_contract_ref: String,
    /// Stable marker id. Unique within the artifact.
    pub marker_id: String,
    /// Artifact class that owns the marker.
    pub artifact_class: ArtifactClass,
    /// Opaque artifact ref the marker is attached to.
    pub artifact_ref: String,
    /// Capability id this marker depends on.
    pub required_capability_id: String,
    /// Capability lifecycle state the producer observed when the
    /// marker was minted.
    pub required_lifecycle_state: CapabilityLifecycleState,
    /// Dependency class. Mirrors the
    /// [`CapabilityRecord::dependency_class`] of the required
    /// capability so consumers branch on one closed vocabulary.
    pub dependency_class: DependencyClass,
    /// Support promise recorded for the dependency.
    pub support_promise: SupportPromise,
    /// Effect on import the producer published for the case where the
    /// dependency is missing on the target.
    pub effect_on_import: EffectOnImport,
    /// Behavior summary + fallback path the surfaces render.
    pub behavior_on_missing: BehaviorOnMissing,
    /// True when an active kill switch / policy disable narrowed the
    /// dependency at mint time. Producers set this so consumers know
    /// the marker reflects an active disable, not just a lifecycle
    /// floor.
    #[serde(default)]
    pub kill_switch_active: bool,
    /// True when the dependency is restricted to specific hosts
    /// (managed, desktop-only, companion-only, sovereign, etc.).
    /// When set, [`host_scope`] is the closed list of hosts the
    /// dependency is admitted on.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub host_scope: Vec<String>,
    /// Optional docs ref the surface can open for context.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_ref: Option<String>,
}

impl ArtifactDependencyMarker {
    /// Constructs a marker from a [`CapabilityRecord`] using the
    /// capability's defaults for [`effect_on_import`] and
    /// [`behavior_on_missing.fallback_path`]. Producers override the
    /// fields directly when their case diverges from the capability
    /// defaults.
    pub fn from_capability(
        marker_id: impl Into<String>,
        artifact_class: ArtifactClass,
        artifact_ref: impl Into<String>,
        capability: &CapabilityRecord,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: ARTIFACT_DEPENDENCY_MARKER_KIND.to_owned(),
            schema_version: ARTIFACT_DEPENDENCY_MARKER_SCHEMA_VERSION,
            shared_contract_ref: ARTIFACT_DEPENDENCY_MARKER_SHARED_CONTRACT_REF.to_owned(),
            marker_id: marker_id.into(),
            artifact_class,
            artifact_ref: artifact_ref.into(),
            required_capability_id: capability.capability_id.clone(),
            required_lifecycle_state: capability.lifecycle_state,
            dependency_class: capability.dependency_class,
            support_promise: capability.support_promise,
            effect_on_import: capability.default_effect_on_import,
            behavior_on_missing: BehaviorOnMissing {
                summary: summary.into(),
                fallback_path: capability.default_fallback_path.clone(),
                docs_ref: capability.docs_ref.clone(),
            },
            kill_switch_active: false,
            host_scope: capability.host_scope.clone(),
            docs_ref: capability.docs_ref.clone(),
        }
    }
}

/// Validation errors emitted by [`validate_capability_record`],
/// [`validate_marker`], and [`validate_artifact_markers`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MarkerValidationError {
    /// Required string field was empty.
    EmptyField {
        /// Owning record kind / marker id for context.
        owner: String,
        /// Empty field name.
        field: String,
    },
    /// Two markers in the same artifact shared a marker id.
    DuplicateMarkerId {
        /// Artifact ref.
        artifact_ref: String,
        /// Reused marker id.
        marker_id: String,
    },
    /// Marker's dependency class disagreed with the capability
    /// record it claims to mirror.
    DependencyClassDrift {
        /// Marker id.
        marker_id: String,
        /// Marker's recorded class.
        marker_class: DependencyClass,
        /// Capability record's class.
        capability_class: DependencyClass,
    },
    /// Marker's support promise disagreed with the capability record
    /// it claims to mirror.
    SupportPromiseDrift {
        /// Marker id.
        marker_id: String,
        /// Marker's recorded support promise.
        marker_promise: SupportPromise,
        /// Capability record's support promise.
        capability_promise: SupportPromise,
    },
    /// Marker's lifecycle state disagreed with the capability record
    /// it claims to mirror.
    LifecycleStateDrift {
        /// Marker id.
        marker_id: String,
        /// Marker's recorded lifecycle state.
        marker_state: CapabilityLifecycleState,
        /// Capability record's lifecycle state.
        capability_state: CapabilityLifecycleState,
    },
    /// Marker advertised [`DependencyClass::HostSpecific`] but
    /// carried no host scope.
    HostSpecificMissingScope {
        /// Marker id.
        marker_id: String,
    },
    /// Marker shipped behavior_on_missing with an empty fallback
    /// path. Surfaces depend on this field to render a recover
    /// affordance.
    FallbackPathMissing {
        /// Marker id.
        marker_id: String,
    },
    /// Marker shipped behavior_on_missing with an empty summary.
    SummaryMissing {
        /// Marker id.
        marker_id: String,
    },
    /// Marker advertised an unsupported schema version.
    SchemaVersionMismatch {
        /// Marker id.
        marker_id: String,
        /// Found version.
        found: u32,
        /// Supported version.
        expected: u32,
    },
    /// Marker claimed an artifact class outside the persisted set.
    /// Reserved for future deserializer drift (the in-memory enum
    /// can't construct an unknown variant).
    UnknownArtifactClass {
        /// Marker id.
        marker_id: String,
        /// Unknown token.
        token: String,
    },
    /// Marker carried `kill_switch_active=true` but the artifact's
    /// behavior_on_missing did not carry a recover path. A
    /// kill-switched marker MUST carry a recover path so the user
    /// can dismiss / wait / open the resolution flow.
    KillSwitchActiveWithoutRecoverPath {
        /// Marker id.
        marker_id: String,
    },
    /// Artifact carried two markers for the same required capability
    /// id. The producer must collapse them into one marker.
    DuplicateCapabilityWithinArtifact {
        /// Artifact ref.
        artifact_ref: String,
        /// Reused capability id.
        capability_id: String,
    },
}

impl std::fmt::Display for MarkerValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyField { owner, field } => write!(
                f,
                "capabilities record {owner:?} has empty required field {field:?}"
            ),
            Self::DuplicateMarkerId {
                artifact_ref,
                marker_id,
            } => write!(
                f,
                "artifact {artifact_ref:?} carries duplicate marker_id {marker_id:?}"
            ),
            Self::DependencyClassDrift {
                marker_id,
                marker_class,
                capability_class,
            } => write!(
                f,
                "marker {marker_id:?} dependency_class {} drifted from capability record class {}",
                marker_class.as_str(),
                capability_class.as_str()
            ),
            Self::SupportPromiseDrift {
                marker_id,
                marker_promise,
                capability_promise,
            } => write!(
                f,
                "marker {marker_id:?} support_promise {} drifted from capability record promise {}",
                marker_promise.as_str(),
                capability_promise.as_str()
            ),
            Self::LifecycleStateDrift {
                marker_id,
                marker_state,
                capability_state,
            } => write!(
                f,
                "marker {marker_id:?} required_lifecycle_state {} drifted from capability record state {}",
                marker_state.as_str(),
                capability_state.as_str()
            ),
            Self::HostSpecificMissingScope { marker_id } => write!(
                f,
                "marker {marker_id:?} is host_specific but ships no host_scope entries"
            ),
            Self::FallbackPathMissing { marker_id } => write!(
                f,
                "marker {marker_id:?} ships an empty behavior_on_missing.fallback_path"
            ),
            Self::SummaryMissing { marker_id } => write!(
                f,
                "marker {marker_id:?} ships an empty behavior_on_missing.summary"
            ),
            Self::SchemaVersionMismatch {
                marker_id,
                found,
                expected,
            } => write!(
                f,
                "marker {marker_id:?} schema_version {found} does not match expected {expected}"
            ),
            Self::UnknownArtifactClass { marker_id, token } => write!(
                f,
                "marker {marker_id:?} carries unknown artifact_class token {token:?}"
            ),
            Self::KillSwitchActiveWithoutRecoverPath { marker_id } => write!(
                f,
                "marker {marker_id:?} has kill_switch_active=true but carries no fallback_path"
            ),
            Self::DuplicateCapabilityWithinArtifact {
                artifact_ref,
                capability_id,
            } => write!(
                f,
                "artifact {artifact_ref:?} carries two markers for capability {capability_id:?}"
            ),
        }
    }
}

impl std::error::Error for MarkerValidationError {}

/// Validates one [`CapabilityRecord`] against the field-presence
/// promises.
pub fn validate_capability_record(
    record: &CapabilityRecord,
) -> Result<(), Vec<MarkerValidationError>> {
    let mut errors = Vec::new();
    let owner = format!("capability:{}", record.capability_id);
    check_empty(&owner, "capability_id", &record.capability_id, &mut errors);
    check_empty(&owner, "title", &record.title, &mut errors);
    check_empty(
        &owner,
        "default_fallback_path",
        &record.default_fallback_path,
        &mut errors,
    );
    if record.dependency_class == DependencyClass::HostSpecific && record.host_scope.is_empty() {
        errors.push(MarkerValidationError::EmptyField {
            owner,
            field: "host_scope".to_owned(),
        });
    }
    if record.schema_version != CAPABILITY_RECORD_SCHEMA_VERSION {
        errors.push(MarkerValidationError::SchemaVersionMismatch {
            marker_id: record.capability_id.clone(),
            found: record.schema_version,
            expected: CAPABILITY_RECORD_SCHEMA_VERSION,
        });
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates one [`ArtifactDependencyMarker`] independently.
///
/// Cross-marker checks (duplicate ids, drift against
/// [`CapabilityRecord`]) live in [`validate_artifact_markers`].
pub fn validate_marker(
    marker: &ArtifactDependencyMarker,
) -> Result<(), Vec<MarkerValidationError>> {
    let mut errors = Vec::new();
    if marker.schema_version != ARTIFACT_DEPENDENCY_MARKER_SCHEMA_VERSION {
        errors.push(MarkerValidationError::SchemaVersionMismatch {
            marker_id: marker.marker_id.clone(),
            found: marker.schema_version,
            expected: ARTIFACT_DEPENDENCY_MARKER_SCHEMA_VERSION,
        });
    }
    check_empty(
        &marker.marker_id,
        "marker_id",
        &marker.marker_id,
        &mut errors,
    );
    check_empty(
        &marker.marker_id,
        "artifact_ref",
        &marker.artifact_ref,
        &mut errors,
    );
    check_empty(
        &marker.marker_id,
        "required_capability_id",
        &marker.required_capability_id,
        &mut errors,
    );
    if marker.behavior_on_missing.summary.trim().is_empty() {
        errors.push(MarkerValidationError::SummaryMissing {
            marker_id: marker.marker_id.clone(),
        });
    }
    if marker.behavior_on_missing.fallback_path.trim().is_empty() {
        errors.push(MarkerValidationError::FallbackPathMissing {
            marker_id: marker.marker_id.clone(),
        });
    }
    if marker.kill_switch_active && marker.behavior_on_missing.fallback_path.trim().is_empty() {
        errors.push(MarkerValidationError::KillSwitchActiveWithoutRecoverPath {
            marker_id: marker.marker_id.clone(),
        });
    }
    if marker.dependency_class == DependencyClass::HostSpecific && marker.host_scope.is_empty() {
        errors.push(MarkerValidationError::HostSpecificMissingScope {
            marker_id: marker.marker_id.clone(),
        });
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates a set of markers belonging to one artifact, cross-checked
/// against the capability records the markers claim to mirror.
///
/// The catalog is optional; passing an empty slice runs only the
/// per-marker and uniqueness checks.
pub fn validate_artifact_markers(
    artifact_ref: &str,
    markers: &[ArtifactDependencyMarker],
    catalog: &[CapabilityRecord],
) -> Result<(), Vec<MarkerValidationError>> {
    let mut errors = Vec::new();
    let mut seen_marker_ids = BTreeSet::new();
    let mut seen_capability_ids = BTreeSet::new();
    let catalog_index: BTreeMap<&str, &CapabilityRecord> = catalog
        .iter()
        .map(|record| (record.capability_id.as_str(), record))
        .collect();

    for marker in markers {
        if let Err(per_marker) = validate_marker(marker) {
            errors.extend(per_marker);
        }
        if !seen_marker_ids.insert(marker.marker_id.clone()) {
            errors.push(MarkerValidationError::DuplicateMarkerId {
                artifact_ref: artifact_ref.to_owned(),
                marker_id: marker.marker_id.clone(),
            });
        }
        if !seen_capability_ids.insert(marker.required_capability_id.clone()) {
            errors.push(MarkerValidationError::DuplicateCapabilityWithinArtifact {
                artifact_ref: artifact_ref.to_owned(),
                capability_id: marker.required_capability_id.clone(),
            });
        }
        if let Some(capability) = catalog_index.get(marker.required_capability_id.as_str()) {
            if marker.dependency_class != capability.dependency_class {
                errors.push(MarkerValidationError::DependencyClassDrift {
                    marker_id: marker.marker_id.clone(),
                    marker_class: marker.dependency_class,
                    capability_class: capability.dependency_class,
                });
            }
            if marker.support_promise != capability.support_promise {
                errors.push(MarkerValidationError::SupportPromiseDrift {
                    marker_id: marker.marker_id.clone(),
                    marker_promise: marker.support_promise,
                    capability_promise: capability.support_promise,
                });
            }
            if marker.required_lifecycle_state != capability.lifecycle_state {
                errors.push(MarkerValidationError::LifecycleStateDrift {
                    marker_id: marker.marker_id.clone(),
                    marker_state: marker.required_lifecycle_state,
                    capability_state: capability.lifecycle_state,
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// One per-surface projection of a marker. Surfaces render this
/// directly so settings inspectors, import-review sheets, bundle
/// detail pages, downgrade flows, CLI inspect, and docs / help pages
/// all consume the same fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MarkerHostProjection {
    /// Host surface this projection targets.
    pub host_surface: HostSurface,
    /// Source marker id.
    pub marker_id: String,
    /// Artifact ref the marker is attached to.
    pub artifact_ref: String,
    /// Artifact class token.
    pub artifact_class: String,
    /// Required capability id.
    pub required_capability_id: String,
    /// Dependency class token.
    pub dependency_class: String,
    /// Required lifecycle state token.
    pub required_lifecycle_state: String,
    /// Support promise token.
    pub support_promise: String,
    /// Effect on import token.
    pub effect_on_import: String,
    /// True when the marker represents an active kill switch / policy
    /// disable.
    pub kill_switch_active: bool,
    /// True when the projection's surface MUST render a recover
    /// affordance for the user (e.g. inspector, import-review sheet,
    /// downgrade flow). Headless / CLI inspect and docs / help pages
    /// quote the marker without requiring a recover button.
    pub requires_recover_affordance: bool,
    /// Bounded summary copy.
    pub summary: String,
    /// Bounded fallback / recover path copy.
    pub fallback_path: String,
    /// True for every projection; the closed [`EffectOnImport`]
    /// vocabulary forbids silent drop.
    pub user_authored_data_preserved: bool,
    /// True when the surface MUST disclose the marker before applying
    /// the artifact (inspector and import-review sheet). False for
    /// surfaces that quote the marker as context (docs / help, CLI
    /// inspect output).
    pub blocks_apply_until_disclosed: bool,
}

/// Projects a marker onto one host surface using the shared
/// vocabulary.
pub fn project_marker_for_host_surface(
    marker: &ArtifactDependencyMarker,
    host_surface: HostSurface,
) -> MarkerHostProjection {
    let requires_recover_affordance = match host_surface {
        HostSurface::SettingsInspector
        | HostSurface::ImportReviewSheet
        | HostSurface::DowngradeFlow
        | HostSurface::BundleDetailPage => true,
        HostSurface::HeadlessCliInspect | HostSurface::DocsHelpPage => false,
    };
    let blocks_apply_until_disclosed = match host_surface {
        HostSurface::ImportReviewSheet | HostSurface::DowngradeFlow => true,
        HostSurface::SettingsInspector
        | HostSurface::BundleDetailPage
        | HostSurface::HeadlessCliInspect
        | HostSurface::DocsHelpPage => false,
    };

    MarkerHostProjection {
        host_surface,
        marker_id: marker.marker_id.clone(),
        artifact_ref: marker.artifact_ref.clone(),
        artifact_class: marker.artifact_class.as_str().to_owned(),
        required_capability_id: marker.required_capability_id.clone(),
        dependency_class: marker.dependency_class.as_str().to_owned(),
        required_lifecycle_state: marker.required_lifecycle_state.as_str().to_owned(),
        support_promise: marker.support_promise.as_str().to_owned(),
        effect_on_import: marker.effect_on_import.as_str().to_owned(),
        kill_switch_active: marker.kill_switch_active,
        requires_recover_affordance,
        summary: marker.behavior_on_missing.summary.clone(),
        fallback_path: marker.behavior_on_missing.fallback_path.clone(),
        user_authored_data_preserved: marker.effect_on_import.preserves_user_data(),
        blocks_apply_until_disclosed,
    }
}

/// Seed catalog of capability records covering the dependency classes
/// the spec calls out (Labs, Preview, Beta-only, policy-gated,
/// host-specific). The catalog is intentionally small; downstream
/// crates register their own capabilities. Used by tests and by the
/// release-evidence packet builder.
pub fn catalog_default_capabilities() -> Vec<CapabilityRecord> {
    vec![
        {
            let mut record = CapabilityRecord::new(
                "ai.inline_completions.labs",
                "Inline completions (Labs)",
                CapabilityLifecycleState::Labs,
                SupportPromise::BestEffort,
                DependencyClass::Labs,
                EffectOnImport::NarrowBehaviorPreserveData,
                "Open Settings → Labs to opt into inline completions; the original prompt template is preserved either way.",
            );
            record.docs_ref = Some("docs/help/labs/inline_completions.md".to_owned());
            record
        },
        {
            let mut record = CapabilityRecord::new(
                "preview.structured_diagnostics",
                "Structured diagnostics (Preview)",
                CapabilityLifecycleState::Preview,
                SupportPromise::CommunitySupported,
                DependencyClass::Preview,
                EffectOnImport::EmulatedDowngradePreserveData,
                "Diagnostics render in the legacy path on this target; enable the structured-diagnostics preview to restore.",
            );
            record.docs_ref = Some("docs/help/preview/structured_diagnostics.md".to_owned());
            record
        },
        {
            let mut record = CapabilityRecord::new(
                "beta.workflow_bundle.recorded_macros",
                "Recorded macros (Beta)",
                CapabilityLifecycleState::Beta,
                SupportPromise::StandardSupport,
                DependencyClass::BetaOnly,
                EffectOnImport::HoldForLaterPreserveData,
                "Recorded macros are held in the import review sheet on this target until the beta capability is enabled.",
            );
            record.docs_ref = Some("docs/help/beta/workflow_bundle_recorded_macros.md".to_owned());
            record
        },
        {
            let mut record = CapabilityRecord::new(
                "policy.telemetry.send_redacted_bundle",
                "Telemetry redacted bundle (policy-gated)",
                CapabilityLifecycleState::DisabledByPolicy,
                SupportPromise::OperatorOnly,
                DependencyClass::PolicyGated,
                EffectOnImport::BlockApplyPreserveData,
                "Apply is blocked on this target by the active admin policy bundle; request an admin policy change to proceed.",
            );
            record.docs_ref = Some("docs/help/policy/telemetry_redacted_bundle.md".to_owned());
            record
        },
        {
            let mut record = CapabilityRecord::new(
                "host.managed_admin_surface.sso_bundle",
                "Managed SSO bundle (host-specific)",
                CapabilityLifecycleState::Stable,
                SupportPromise::ExtendedSupport,
                DependencyClass::HostSpecific,
                EffectOnImport::RenderTombstonePreserveData,
                "The SSO bundle renders as a read-only tombstone on self-hosted / account-free-local installs; request managed access to enable.",
            );
            record.docs_ref = Some("docs/help/host/managed_sso_bundle.md".to_owned());
            record.host_scope = vec![
                "managed_admin_surface".to_owned(),
                "desktop_product".to_owned(),
            ];
            record
        },
    ]
}

fn check_empty(owner: &str, field: &str, value: &str, errors: &mut Vec<MarkerValidationError>) {
    if value.trim().is_empty() {
        errors.push(MarkerValidationError::EmptyField {
            owner: owner.to_owned(),
            field: field.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_marker(capability: &CapabilityRecord) -> ArtifactDependencyMarker {
        ArtifactDependencyMarker::from_capability(
            format!("marker:{}:test", capability.capability_id),
            ArtifactClass::SettingsExport,
            "artifact:settings_export:test",
            capability,
            format!(
                "Behavior depends on {}; rendered with the {} dependency class.",
                capability.title,
                capability.dependency_class.as_str()
            ),
        )
    }

    #[test]
    fn seed_catalog_validates_and_covers_all_dependency_classes() {
        let catalog = catalog_default_capabilities();
        let mut seen = BTreeSet::new();
        for record in &catalog {
            validate_capability_record(record).unwrap_or_else(|e| {
                panic!(
                    "capability {} failed validation: {e:?}",
                    record.capability_id
                )
            });
            seen.insert(record.dependency_class);
        }
        for class in [
            DependencyClass::Labs,
            DependencyClass::Preview,
            DependencyClass::BetaOnly,
            DependencyClass::PolicyGated,
            DependencyClass::HostSpecific,
        ] {
            assert!(seen.contains(&class), "missing class {}", class.as_str());
        }
    }

    #[test]
    fn marker_from_capability_inherits_vocabulary() {
        let catalog = catalog_default_capabilities();
        for record in &catalog {
            let marker = sample_marker(record);
            assert_eq!(marker.required_capability_id, record.capability_id);
            assert_eq!(marker.required_lifecycle_state, record.lifecycle_state);
            assert_eq!(marker.dependency_class, record.dependency_class);
            assert_eq!(marker.support_promise, record.support_promise);
            assert_eq!(marker.effect_on_import, record.default_effect_on_import);
            assert_eq!(
                marker.behavior_on_missing.fallback_path,
                record.default_fallback_path
            );
            assert!(marker.effect_on_import.preserves_user_data());
            validate_marker(&marker).expect("marker validates");
        }
    }

    #[test]
    fn validate_artifact_markers_flags_drift() {
        let catalog = catalog_default_capabilities();
        let preview = &catalog[1];
        let mut marker = sample_marker(preview);
        marker.dependency_class = DependencyClass::Labs;
        let errors = validate_artifact_markers("artifact:test", &[marker], &catalog)
            .expect_err("drift must be flagged");
        assert!(errors
            .iter()
            .any(|e| matches!(e, MarkerValidationError::DependencyClassDrift { .. })));
    }

    #[test]
    fn validate_artifact_markers_flags_duplicate_capability() {
        let catalog = catalog_default_capabilities();
        let beta = &catalog[2];
        let m1 = sample_marker(beta);
        let mut m2 = sample_marker(beta);
        m2.marker_id = format!("{}:dup", m2.marker_id);
        let errors = validate_artifact_markers("artifact:test", &[m1, m2], &catalog)
            .expect_err("duplicate capability must be flagged");
        assert!(errors.iter().any(|e| matches!(
            e,
            MarkerValidationError::DuplicateCapabilityWithinArtifact { .. }
        )));
    }

    #[test]
    fn validate_marker_flags_host_specific_without_scope() {
        let catalog = catalog_default_capabilities();
        let host = &catalog[4];
        let mut marker = sample_marker(host);
        marker.host_scope.clear();
        let errors = validate_marker(&marker).expect_err("host-specific must carry host_scope");
        assert!(errors
            .iter()
            .any(|e| matches!(e, MarkerValidationError::HostSpecificMissingScope { .. })));
    }

    #[test]
    fn validate_marker_flags_empty_fallback_path_and_summary() {
        let catalog = catalog_default_capabilities();
        let labs = &catalog[0];
        let mut marker = sample_marker(labs);
        marker.behavior_on_missing.fallback_path.clear();
        marker.behavior_on_missing.summary.clear();
        let errors = validate_marker(&marker).expect_err("must flag empties");
        assert!(errors
            .iter()
            .any(|e| matches!(e, MarkerValidationError::FallbackPathMissing { .. })));
        assert!(errors
            .iter()
            .any(|e| matches!(e, MarkerValidationError::SummaryMissing { .. })));
    }

    #[test]
    fn projection_matches_marker_and_uses_shared_vocabulary() {
        let catalog = catalog_default_capabilities();
        let preview = &catalog[1];
        let marker = sample_marker(preview);
        for surface in [
            HostSurface::SettingsInspector,
            HostSurface::ImportReviewSheet,
            HostSurface::BundleDetailPage,
            HostSurface::DowngradeFlow,
            HostSurface::HeadlessCliInspect,
            HostSurface::DocsHelpPage,
        ] {
            let projection = project_marker_for_host_surface(&marker, surface);
            assert_eq!(projection.marker_id, marker.marker_id);
            assert_eq!(
                projection.required_capability_id,
                marker.required_capability_id
            );
            assert_eq!(
                projection.dependency_class,
                marker.dependency_class.as_str()
            );
            assert_eq!(
                projection.required_lifecycle_state,
                marker.required_lifecycle_state.as_str()
            );
            assert_eq!(projection.support_promise, marker.support_promise.as_str());
            assert_eq!(
                projection.effect_on_import,
                marker.effect_on_import.as_str()
            );
            assert!(projection.user_authored_data_preserved);
            match surface {
                HostSurface::SettingsInspector
                | HostSurface::ImportReviewSheet
                | HostSurface::BundleDetailPage
                | HostSurface::DowngradeFlow => {
                    assert!(projection.requires_recover_affordance);
                }
                HostSurface::HeadlessCliInspect | HostSurface::DocsHelpPage => {
                    assert!(!projection.requires_recover_affordance);
                }
            }
            match surface {
                HostSurface::ImportReviewSheet | HostSurface::DowngradeFlow => {
                    assert!(projection.blocks_apply_until_disclosed);
                }
                _ => assert!(!projection.blocks_apply_until_disclosed),
            }
        }
    }

    #[test]
    fn marker_round_trips_through_serde_json() {
        let catalog = catalog_default_capabilities();
        let marker = sample_marker(&catalog[3]);
        let bytes = serde_json::to_string(&marker).expect("serialize");
        let decoded: ArtifactDependencyMarker = serde_json::from_str(&bytes).expect("deserialize");
        assert_eq!(decoded, marker);
    }

    #[test]
    fn kill_switch_active_marker_preserves_recover_path() {
        let catalog = catalog_default_capabilities();
        let policy = &catalog[3];
        let mut marker = sample_marker(policy);
        marker.kill_switch_active = true;
        validate_marker(&marker).expect("kill-switched marker validates with recover path");
        let projection = project_marker_for_host_surface(&marker, HostSurface::DowngradeFlow);
        assert!(projection.kill_switch_active);
        assert!(projection.requires_recover_affordance);
        assert!(projection.blocks_apply_until_disclosed);
        assert!(!projection.fallback_path.trim().is_empty());
    }
}
