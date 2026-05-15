//! Typed capability-lifecycle registry consumer and claim validator.
//!
//! This module consumes the checked-in capability lifecycle registry and keeps
//! lifecycle state, dependency markers, and claim-denial posture on the same
//! vocabulary as the governance schema. Product surfaces should use these
//! types instead of parsing lifecycle labels or marker strings locally.

use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Repository path for the canonical capability lifecycle registry.
pub const CURRENT_CAPABILITY_LIFECYCLE_REGISTRY_PATH: &str =
    "artifacts/governance/capability_lifecycle_registry.yaml";

const CURRENT_CAPABILITY_LIFECYCLE_REGISTRY_YAML: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/capability_lifecycle_registry.yaml"
));

/// Record-kind tag carried by schema row records.
pub const CAPABILITY_LIFECYCLE_ROW_RECORD_KIND: &str = "capability_lifecycle_row_record";

/// Record-kind tag carried by schema dependency marker records.
pub const DEPENDENCY_MARKER_RECORD_KIND: &str = "dependency_marker_record";

/// Record-kind tag carried by schema audit event records.
pub const CAPABILITY_LIFECYCLE_AUDIT_EVENT_RECORD_KIND: &str =
    "capability_lifecycle_audit_event_record";

/// Parses a capability lifecycle registry YAML document.
pub fn parse_capability_lifecycle_registry(
    yaml: &str,
) -> Result<CapabilityLifecycleRegistry, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Loads the checked-in capability lifecycle registry bundled into the crate.
pub fn current_capability_lifecycle_registry(
) -> Result<CapabilityLifecycleRegistry, serde_yaml::Error> {
    parse_capability_lifecycle_registry(CURRENT_CAPABILITY_LIFECYCLE_REGISTRY_YAML)
}

/// Opaque capability lifecycle identifier safe to carry across surfaces.
///
/// The wrapper preserves the registry id verbatim while rejecting empty values.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CapabilityId(String);

impl CapabilityId {
    /// Creates a capability id after applying product-record validation.
    ///
    /// # Errors
    ///
    /// Returns [`CapabilityIdError`] when the id is empty.
    pub fn new(value: impl Into<String>) -> Result<Self, CapabilityIdError> {
        let value = value.into();
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err(CapabilityIdError::Empty);
        }
        Ok(Self(trimmed.to_string()))
    }

    /// Returns the opaque id string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Returns the owned opaque id string.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for CapabilityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for CapabilityId {
    type Err = CapabilityIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Serialize for CapabilityId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for CapabilityId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        Self::new(value).map_err(serde::de::Error::custom)
    }
}

/// Error returned when an opaque capability id is not admissible.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityIdError {
    /// The id was empty or all whitespace.
    Empty,
}

impl fmt::Display for CapabilityIdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("capability id must not be empty"),
        }
    }
}

impl std::error::Error for CapabilityIdError {}

/// Lifecycle readiness state from the capability-lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum LifecycleState {
    /// Early opt-in experiment with no stable contract.
    #[serde(rename = "labs", alias = "Labs")]
    Labs,
    /// Broadly testable capability with expected churn.
    #[serde(rename = "preview", alias = "Preview")]
    Preview,
    /// Feature-complete bounded rollout with support intent.
    #[serde(rename = "beta", alias = "Beta")]
    Beta,
    /// Committed surface with normal compatibility obligations.
    #[serde(rename = "stable", alias = "Stable")]
    Stable,
    /// Stable surface with extended-support obligations.
    #[serde(rename = "lts_facing", alias = "LtsFacing", alias = "LTSFacing")]
    LtsFacing,
    /// Still present but on a visible sunset path.
    #[serde(rename = "deprecated", alias = "Deprecated")]
    Deprecated,
    /// Functionality exists but is disabled by policy or a kill switch.
    #[serde(rename = "disabled_by_policy", alias = "DisabledByPolicy")]
    DisabledByPolicy,
    /// Capability is removed and only a tombstone should render.
    #[serde(rename = "retired", alias = "Retired")]
    Retired,
}

impl LifecycleState {
    /// Returns the schema token for this lifecycle state.
    pub const fn as_schema_token(self) -> &'static str {
        match self {
            Self::Labs => "labs",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::LtsFacing => "lts_facing",
            Self::Deprecated => "deprecated",
            Self::DisabledByPolicy => "disabled_by_policy",
            Self::Retired => "retired",
        }
    }

    /// Returns the registry display token for this lifecycle state.
    pub const fn as_registry_token(self) -> &'static str {
        match self {
            Self::Labs => "Labs",
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::Stable => "Stable",
            Self::LtsFacing => "LtsFacing",
            Self::Deprecated => "Deprecated",
            Self::DisabledByPolicy => "DisabledByPolicy",
            Self::Retired => "Retired",
        }
    }

    /// Returns whether this state is below a stable readiness claim.
    pub const fn blocks_stable_claim(self) -> bool {
        !matches!(self, Self::Stable | Self::LtsFacing)
    }

    /// Returns whether this effective state satisfies a declared claim.
    pub const fn satisfies_claim(self, claimed: Self) -> bool {
        match (self.readiness_rank(), claimed.readiness_rank()) {
            (Some(effective), Some(claimed)) => effective >= claimed,
            _ => matches!(
                (self, claimed),
                (Self::Deprecated, Self::Deprecated)
                    | (Self::DisabledByPolicy, Self::DisabledByPolicy)
                    | (Self::Retired, Self::Retired)
            ),
        }
    }

    const fn readiness_rank(self) -> Option<u8> {
        match self {
            Self::Labs => Some(0),
            Self::Preview => Some(1),
            Self::Beta => Some(2),
            Self::Stable => Some(3),
            Self::LtsFacing => Some(4),
            Self::Deprecated | Self::DisabledByPolicy | Self::Retired => None,
        }
    }
}

impl fmt::Display for LifecycleState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_schema_token())
    }
}

/// Dependency-marker kind from the capability-lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MarkerKind {
    /// Dependency on a capability below stable readiness.
    #[serde(
        rename = "non_stable_capability_dependency",
        alias = "NonStableCapabilityDependency"
    )]
    NonStableCapabilityDependency,
    /// Dependency disabled by policy.
    #[serde(
        rename = "disabled_by_policy_dependency",
        alias = "DisabledByPolicyDependency"
    )]
    DisabledByPolicyDependency,
    /// Dependency that is deprecated.
    #[serde(rename = "deprecated_dependency", alias = "DeprecatedDependency")]
    DeprecatedDependency,
    /// Dependency that is retired.
    #[serde(rename = "retired_dependency", alias = "RetiredDependency")]
    RetiredDependency,
    /// Dependency linked to an external provider or hosted integration.
    #[serde(
        rename = "provider_linked_dependency",
        alias = "ProviderLinkedDependency",
        alias = "HostedIntegrationDependency"
    )]
    ProviderLinkedDependency,
    /// Dependency restricted to a different client scope.
    #[serde(
        rename = "client_scope_restricted_dependency",
        alias = "ClientScopeRestrictedDependency"
    )]
    ClientScopeRestrictedDependency,
    /// Dependency whose freshness floor is unmet.
    #[serde(
        rename = "freshness_floor_dependency",
        alias = "FreshnessFloorDependency"
    )]
    FreshnessFloorDependency,
    /// Dependency absorbed by a kill switch.
    #[serde(rename = "kill_switch_dependency", alias = "KillSwitchDependency")]
    KillSwitchDependency,
    /// Dependency available only on a managed lane.
    #[serde(rename = "managed_only_dependency", alias = "ManagedOnlyDependency")]
    ManagedOnlyDependency,
}

impl MarkerKind {
    /// Returns the schema token for this marker kind.
    pub const fn as_schema_token(self) -> &'static str {
        match self {
            Self::NonStableCapabilityDependency => "non_stable_capability_dependency",
            Self::DisabledByPolicyDependency => "disabled_by_policy_dependency",
            Self::DeprecatedDependency => "deprecated_dependency",
            Self::RetiredDependency => "retired_dependency",
            Self::ProviderLinkedDependency => "provider_linked_dependency",
            Self::ClientScopeRestrictedDependency => "client_scope_restricted_dependency",
            Self::FreshnessFloorDependency => "freshness_floor_dependency",
            Self::KillSwitchDependency => "kill_switch_dependency",
            Self::ManagedOnlyDependency => "managed_only_dependency",
        }
    }
}

/// Axis effect applied by a dependency marker to its parent capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum EffectOnParent {
    /// Narrows the effective lifecycle state.
    #[serde(
        rename = "narrows_effective_lifecycle_state",
        alias = "NarrowsLifecycle"
    )]
    NarrowsEffectiveLifecycleState,
    /// Narrows the effective support class.
    #[serde(rename = "narrows_effective_support_class", alias = "NarrowsSupport")]
    NarrowsEffectiveSupportClass,
    /// Narrows the effective release channel.
    #[serde(rename = "narrows_effective_release_channel", alias = "NarrowsRelease")]
    NarrowsEffectiveReleaseChannel,
    /// Narrows the effective freshness class.
    #[serde(
        rename = "narrows_effective_freshness_class",
        alias = "NarrowsFreshness"
    )]
    NarrowsEffectiveFreshnessClass,
    /// Narrows the effective client scope.
    #[serde(
        rename = "narrows_effective_client_scope",
        alias = "NarrowsClientScope"
    )]
    NarrowsEffectiveClientScope,
    /// Gates the entire parent capability.
    #[serde(rename = "gates_entire_capability", alias = "GatesCapability")]
    GatesEntireCapability,
}

/// Dependency-marker reason code from the capability-lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReasonCode {
    /// Dependency state is lower than the parent declared state.
    DependencyStateBelowParentDeclaredState,
    /// Dependency is disabled by policy.
    DependencyDisabledByPolicy,
    /// Dependency kill switch has tripped.
    DependencyKillSwitchTripped,
    /// Dependency is deprecated inside its overlap window.
    DependencyDeprecatedWithinWindow,
    /// Dependency is retired.
    DependencyRetired,
    /// Provider grant narrowed the dependency.
    ProviderGrantNarrowed,
    /// Provider connection is unhealthy.
    ProviderConnectionUnhealthy,
    /// Current client surface is excluded.
    ClientScopeExcludesSurface,
    /// Freshness floor is unmet.
    FreshnessFloorUnmet,
    /// Managed-only channel is required.
    ManagedOnlyChannelRequired,
}

/// Lifecycle claim denial reason from the capability-lifecycle audit schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DenialReason {
    /// Lifecycle state could not be resolved.
    LifecycleStateUnresolved,
    /// Axis value was not in the controlled vocabulary.
    AxisValueNotInVocabulary,
    /// Declared state is below a dependency ceiling.
    DeclaredStateBelowDependencyCeiling,
    /// Dependency marker cannot be repaired.
    DependencyMarkerRepairUnavailable,
    /// Effective state was absorbed by retirement.
    EffectiveStateAbsorbedByRetirement,
    /// Freshness floor is unmet.
    FreshnessFloorUnmet,
    /// Client scope excludes the rendering surface.
    ClientScopeExcludesSurface,
    /// Policy-disabled row has no repair hook.
    DisabledByPolicyNoRepairHook,
    /// Kill switch has tripped.
    KillSwitchTripped,
    /// Managed-only channel is required.
    ManagedOnlyChannelRequired,
    /// Claim effective state is below its declared claim.
    ClaimEffectiveStateBelowDeclared,
    /// Review disclosure is incomplete.
    ReviewDisclosureIncomplete,
}

/// Audit event id from the capability-lifecycle audit schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditEventId {
    /// Capability lifecycle row was created.
    CapabilityLifecycleRowCreated,
    /// Capability lifecycle row was updated.
    CapabilityLifecycleRowUpdated,
    /// Capability lifecycle row was deprecated.
    CapabilityLifecycleRowDeprecated,
    /// Capability lifecycle row was retired.
    CapabilityLifecycleRowRetired,
    /// Capability lifecycle row was disabled by policy.
    CapabilityLifecycleRowDisabledByPolicy,
    /// Capability lifecycle row was re-enabled.
    CapabilityLifecycleRowReEnabled,
    /// Dependency marker was minted.
    DependencyMarkerMinted,
    /// Dependency marker was updated.
    DependencyMarkerUpdated,
    /// Dependency marker was cleared.
    DependencyMarkerCleared,
    /// Dependency marker repair became unavailable.
    DependencyMarkerRepairUnavailable,
    /// Dependency marker crossed freshness without refresh.
    DependencyMarkerStaleNotRefreshed,
    /// Effective lifecycle state narrowed below declared state.
    EffectiveLifecycleStateNarrowed,
    /// Effective lifecycle state was absorbed by policy or retirement.
    EffectiveLifecycleStateAbsorbed,
    /// Kill switch tripped.
    KillSwitchTripped,
    /// Lifecycle claim was refused.
    LifecycleClaimRefused,
    /// Capability lifecycle schema version was bumped.
    CapabilityLifecycleSchemaVersionBumped,
}

/// Support class axis from the capability-lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Best-effort support.
    BestEffort,
    /// Community-supported lane.
    CommunitySupported,
    /// Standard support.
    StandardSupport,
    /// Extended support.
    ExtendedSupport,
    /// Operator-only support.
    OperatorOnlySupport,
    /// No support promise.
    NoSupport,
}

/// Release channel axis from the capability-lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseChannel {
    /// Nightly distribution channel.
    NightlyChannel,
    /// Experimental distribution channel.
    ExperimentalChannel,
    /// Preview distribution channel.
    PreviewChannel,
    /// Stable distribution channel.
    StableChannel,
    /// LTS distribution channel.
    LtsChannel,
    /// Managed-only distribution channel.
    ManagedOnlyChannel,
}

/// Freshness axis from the capability-lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Canonical owner was contacted and current.
    AuthoritativeLive,
    /// Warm cache is current enough for the surface.
    WarmCached,
    /// Cache is degraded but usable with disclosure.
    DegradedCached,
    /// Row is stale.
    Stale,
    /// Row is unverified.
    Unverified,
}

/// Client scope axis from the capability-lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientScope {
    /// Desktop product surface.
    DesktopProduct,
    /// CLI surface.
    Cli,
    /// Companion surface.
    CompanionSurface,
    /// Remote agent surface.
    RemoteAgent,
    /// SDK or API surface.
    SdkOrApi,
    /// Managed admin surface.
    ManagedAdminSurface,
}

/// Redaction class used by lifecycle records crossing support and audit paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    /// Metadata-safe default.
    MetadataSafeDefault,
    /// Restricted to operators.
    OperatorOnlyRestricted,
    /// Restricted to internal support.
    InternalSupportRestricted,
    /// Signing evidence only.
    SigningEvidenceOnly,
}

/// Typed consumer view of `capability_lifecycle_registry.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityLifecycleRegistry {
    schema_version: u32,
    as_of: String,
    owner: String,
    registry_id: String,
    #[serde(default)]
    lifecycle_vocabulary: Vec<LifecycleState>,
    #[serde(default)]
    schema_projection: BTreeMap<String, String>,
    #[serde(default)]
    surface_rows: Vec<CapabilityLifecycleSurfaceRow>,
    #[serde(default)]
    dependency_markers: Vec<DependencyMarker>,
}

impl CapabilityLifecycleRegistry {
    /// Returns the registry schema version.
    pub const fn schema_version(&self) -> u32 {
        self.schema_version
    }

    /// Returns the registry effective date.
    pub fn as_of(&self) -> &str {
        &self.as_of
    }

    /// Returns the registry id.
    pub fn registry_id(&self) -> &str {
        &self.registry_id
    }

    /// Returns the registry owner.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the controlled lifecycle vocabulary listed by the registry.
    pub fn lifecycle_vocabulary(&self) -> &[LifecycleState] {
        &self.lifecycle_vocabulary
    }

    /// Returns the display-to-schema projection map carried by the registry.
    pub fn schema_projection(&self) -> &BTreeMap<String, String> {
        &self.schema_projection
    }

    /// Returns all surface rows.
    pub fn surface_rows(&self) -> &[CapabilityLifecycleSurfaceRow] {
        &self.surface_rows
    }

    /// Returns all dependency markers.
    pub fn dependency_markers(&self) -> &[DependencyMarker] {
        &self.dependency_markers
    }

    /// Finds a surface row by its row id.
    pub fn row_by_id(&self, row_id: &str) -> Option<&CapabilityLifecycleSurfaceRow> {
        self.surface_rows
            .iter()
            .find(|row| row.row_id.as_str() == row_id)
    }

    /// Finds a dependency marker by its marker id.
    pub fn marker_by_id(&self, marker_id: &str) -> Option<&DependencyMarker> {
        self.dependency_markers
            .iter()
            .find(|marker| marker.marker_id == marker_id)
    }

    /// Returns the markers attached to a row.
    pub fn markers_for_row(&self, row: &CapabilityLifecycleSurfaceRow) -> Vec<&DependencyMarker> {
        row.dependency_marker_refs
            .iter()
            .filter_map(|marker_ref| self.marker_by_id(marker_ref))
            .collect()
    }

    /// Validates that referenced rows satisfy a lifecycle claim.
    pub fn validate_claim(
        &self,
        row_refs: &[String],
        claimed_lifecycle_state: LifecycleState,
    ) -> CapabilityClaimValidation {
        let mut failures = Vec::new();
        if row_refs.is_empty() {
            failures.push(CapabilityClaimFailure::unresolved(
                "<missing capability lifecycle row>",
                claimed_lifecycle_state,
            ));
        }
        for row_ref in row_refs {
            match self.row_by_id(row_ref) {
                Some(row) if row.effective_lifecycle_state.satisfies_claim(claimed_lifecycle_state) => {}
                Some(row) => failures.push(CapabilityClaimFailure {
                    row_ref: row_ref.clone(),
                    claimed_lifecycle_state,
                    effective_lifecycle_state: Some(row.effective_lifecycle_state),
                    denial_reason: DenialReason::ClaimEffectiveStateBelowDeclared,
                    audit_event_id: AuditEventId::LifecycleClaimRefused,
                    message: format!(
                        "capability claim {claimed_lifecycle_state} exceeds effective lifecycle state {} for {row_ref}",
                        row.effective_lifecycle_state
                    ),
                }),
                None => failures.push(CapabilityClaimFailure::unresolved(
                    row_ref,
                    claimed_lifecycle_state,
                )),
            }
        }

        CapabilityClaimValidation { failures }
    }
}

/// Surface row from the capability lifecycle registry artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityLifecycleSurfaceRow {
    row_id: CapabilityId,
    surface_ref: String,
    surface_kind: String,
    title: String,
    declared_lifecycle_state: LifecycleState,
    effective_lifecycle_state: LifecycleState,
    owner: String,
    target_persona_or_workflow: String,
    default_posture: String,
    migration_note: String,
    support_promise: String,
    review_or_expiry_date: String,
    kill_switch_or_policy_disable_ref: String,
    #[serde(default)]
    source_scope_refs: Vec<String>,
    #[serde(default)]
    scoreboard_row_refs: Vec<String>,
    #[serde(default)]
    dependency_marker_refs: Vec<String>,
    #[serde(default)]
    consumer_surfaces: Vec<String>,
}

impl CapabilityLifecycleSurfaceRow {
    /// Returns the lifecycle row id.
    pub fn row_id(&self) -> &CapabilityId {
        &self.row_id
    }

    /// Returns the product surface ref this row describes.
    pub fn surface_ref(&self) -> &str {
        &self.surface_ref
    }

    /// Returns the surface kind.
    pub fn surface_kind(&self) -> &str {
        &self.surface_kind
    }

    /// Returns the row title.
    pub fn title(&self) -> &str {
        &self.title
    }

    /// Returns the declared lifecycle state.
    pub const fn declared_lifecycle_state(&self) -> LifecycleState {
        self.declared_lifecycle_state
    }

    /// Returns the effective lifecycle state rendered by consumers.
    pub const fn effective_lifecycle_state(&self) -> LifecycleState {
        self.effective_lifecycle_state
    }

    /// Returns the accountable owner.
    pub fn owner(&self) -> &str {
        &self.owner
    }

    /// Returns the target persona or workflow ref.
    pub fn target_persona_or_workflow(&self) -> &str {
        &self.target_persona_or_workflow
    }

    /// Returns the default posture token.
    pub fn default_posture(&self) -> &str {
        &self.default_posture
    }

    /// Returns the migration note.
    pub fn migration_note(&self) -> &str {
        &self.migration_note
    }

    /// Returns the support promise token.
    pub fn support_promise(&self) -> &str {
        &self.support_promise
    }

    /// Returns the review or expiry date.
    pub fn review_or_expiry_date(&self) -> &str {
        &self.review_or_expiry_date
    }

    /// Returns the kill-switch or policy-disable ref.
    pub fn kill_switch_or_policy_disable_ref(&self) -> &str {
        &self.kill_switch_or_policy_disable_ref
    }

    /// Returns source scope refs.
    pub fn source_scope_refs(&self) -> &[String] {
        &self.source_scope_refs
    }

    /// Returns scoreboard row refs.
    pub fn scoreboard_row_refs(&self) -> &[String] {
        &self.scoreboard_row_refs
    }

    /// Returns dependency marker refs rendered by this row.
    pub fn dependency_marker_refs(&self) -> &[String] {
        &self.dependency_marker_refs
    }

    /// Returns consumer surface tokens.
    pub fn consumer_surfaces(&self) -> &[String] {
        &self.consumer_surfaces
    }
}

/// Dependency marker from the capability lifecycle registry artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyMarker {
    marker_id: String,
    marker_schema_version: u32,
    marker_kind: MarkerKind,
    parent_row_ref: CapabilityId,
    #[serde(default)]
    artifact_classes: Vec<String>,
    dependency_ref: String,
    dependency_lifecycle_state: LifecycleState,
    effect_on_parent: EffectOnParent,
    reason_code: ReasonCode,
    disclosure_summary: String,
    repair_or_review_ref: String,
    review_or_expiry_date: String,
    export_visibility: String,
    #[serde(default)]
    source_refs: Vec<String>,
}

impl DependencyMarker {
    /// Returns the marker id.
    pub fn marker_id(&self) -> &str {
        &self.marker_id
    }

    /// Returns the marker schema version.
    pub const fn marker_schema_version(&self) -> u32 {
        self.marker_schema_version
    }

    /// Returns the marker kind.
    pub const fn marker_kind(&self) -> MarkerKind {
        self.marker_kind
    }

    /// Returns the parent lifecycle row id.
    pub fn parent_row_ref(&self) -> &CapabilityId {
        &self.parent_row_ref
    }

    /// Returns artifact classes affected by the marker.
    pub fn artifact_classes(&self) -> &[String] {
        &self.artifact_classes
    }

    /// Returns the dependency ref described by this marker.
    pub fn dependency_ref(&self) -> &str {
        &self.dependency_ref
    }

    /// Returns the dependency lifecycle state.
    pub const fn dependency_lifecycle_state(&self) -> LifecycleState {
        self.dependency_lifecycle_state
    }

    /// Returns the effect on the parent capability.
    pub const fn effect_on_parent(&self) -> EffectOnParent {
        self.effect_on_parent
    }

    /// Returns the marker reason code.
    pub const fn reason_code(&self) -> ReasonCode {
        self.reason_code
    }

    /// Returns the disclosure summary rendered by protected surfaces.
    pub fn disclosure_summary(&self) -> &str {
        &self.disclosure_summary
    }

    /// Returns the repair or review ref.
    pub fn repair_or_review_ref(&self) -> &str {
        &self.repair_or_review_ref
    }

    /// Returns the review or expiry date.
    pub fn review_or_expiry_date(&self) -> &str {
        &self.review_or_expiry_date
    }

    /// Returns the export visibility token.
    pub fn export_visibility(&self) -> &str {
        &self.export_visibility
    }

    /// Returns source refs that back the marker.
    pub fn source_refs(&self) -> &[String] {
        &self.source_refs
    }
}

/// Result of checking capability lifecycle refs against a declared claim.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityClaimValidation {
    failures: Vec<CapabilityClaimFailure>,
}

impl CapabilityClaimValidation {
    /// Returns whether the claim passed validation.
    pub fn is_valid(&self) -> bool {
        self.failures.is_empty()
    }

    /// Returns all validation failures.
    pub fn failures(&self) -> &[CapabilityClaimFailure] {
        &self.failures
    }

    /// Returns a short export-safe summary of the first failure.
    pub fn first_failure_summary(&self) -> Option<&str> {
        self.failures.first().map(|failure| failure.message())
    }
}

/// One failed capability lifecycle claim check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityClaimFailure {
    row_ref: String,
    claimed_lifecycle_state: LifecycleState,
    effective_lifecycle_state: Option<LifecycleState>,
    denial_reason: DenialReason,
    audit_event_id: AuditEventId,
    message: String,
}

impl CapabilityClaimFailure {
    fn unresolved(row_ref: impl Into<String>, claimed_lifecycle_state: LifecycleState) -> Self {
        let row_ref = row_ref.into();
        Self {
            row_ref: row_ref.clone(),
            claimed_lifecycle_state,
            effective_lifecycle_state: None,
            denial_reason: DenialReason::LifecycleStateUnresolved,
            audit_event_id: AuditEventId::LifecycleClaimRefused,
            message: format!(
                "capability claim {claimed_lifecycle_state} references unresolved lifecycle row {row_ref}"
            ),
        }
    }

    /// Returns the lifecycle row ref that failed validation.
    pub fn row_ref(&self) -> &str {
        &self.row_ref
    }

    /// Returns the lifecycle state claimed by the consumer.
    pub const fn claimed_lifecycle_state(&self) -> LifecycleState {
        self.claimed_lifecycle_state
    }

    /// Returns the effective lifecycle state when the row resolved.
    pub const fn effective_lifecycle_state(&self) -> Option<LifecycleState> {
        self.effective_lifecycle_state
    }

    /// Returns the denial reason for audit and review surfaces.
    pub const fn denial_reason(&self) -> DenialReason {
        self.denial_reason
    }

    /// Returns the audit event id surfaces should emit for this failure.
    pub const fn audit_event_id(&self) -> AuditEventId {
        self.audit_event_id
    }

    /// Returns the audit event record kind constant used by this failure.
    pub const fn audit_record_kind(&self) -> &'static str {
        CAPABILITY_LIFECYCLE_AUDIT_EVENT_RECORD_KIND
    }

    /// Returns the export-safe failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

/// One of the schema record shapes in `capability_lifecycle.schema.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CapabilityLifecycleRecord {
    /// Capability lifecycle row record branch.
    Row(CapabilityLifecycleRowRecord),
    /// Dependency marker record branch.
    DependencyMarker(DependencyMarkerRecord),
    /// Capability lifecycle audit event record branch.
    AuditEvent(CapabilityLifecycleAuditEventRecord),
}

/// Schema branch for `capability_lifecycle_row_record`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityLifecycleRowRecord {
    record_kind: String,
    capability_lifecycle_schema_version: u32,
    capability_lifecycle_row_id: String,
    capability_ref: CapabilityRef,
    declared_lifecycle_state: LifecycleState,
    effective_lifecycle_state: LifecycleState,
    support_class: SupportClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effective_support_class: Option<SupportClass>,
    release_channel: ReleaseChannel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    effective_release_channel: Option<ReleaseChannel>,
    freshness_class: FreshnessClass,
    client_scopes: Vec<ClientScope>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    effective_client_scopes: Vec<ClientScope>,
    lifecycle_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    deprecation_window: Option<DeprecationWindow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    replacement_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    since_label: Option<String>,
    declared_at: String,
    last_refreshed_at: String,
    policy_context: PolicyContext,
    redaction_class: RedactionClass,
    live_dependency_markers: Vec<DependencyMarkerSummary>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    disabled_by_policy_reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    kill_switch_state: Option<KillSwitchState>,
}

impl CapabilityLifecycleRowRecord {
    /// Returns the schema record-kind discriminator.
    pub fn record_kind(&self) -> &str {
        &self.record_kind
    }

    /// Returns the row id.
    pub fn capability_lifecycle_row_id(&self) -> &str {
        &self.capability_lifecycle_row_id
    }

    /// Returns the effective lifecycle state.
    pub const fn effective_lifecycle_state(&self) -> LifecycleState {
        self.effective_lifecycle_state
    }
}

/// Schema branch for `dependency_marker_record`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyMarkerRecord {
    record_kind: String,
    capability_lifecycle_schema_version: u32,
    dependency_marker_id: String,
    dependency_marker_schema_version: u32,
    marker_kind: MarkerKind,
    parent_capability_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dependency_capability_ref: Option<String>,
    dependency_lifecycle_state: LifecycleState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dependency_support_class: Option<SupportClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dependency_release_channel: Option<ReleaseChannel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dependency_freshness_class: Option<FreshnessClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dependency_client_scope: Option<Vec<ClientScope>>,
    effect_on_parent: EffectOnParent,
    reason_code: ReasonCode,
    repair_hook_ref: RepairHookRef,
    disclosure_summary: String,
    declared_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    refreshed_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cleared_at: Option<String>,
    policy_context: PolicyContext,
    redaction_class: RedactionClass,
}

impl DependencyMarkerRecord {
    /// Returns the schema record-kind discriminator.
    pub fn record_kind(&self) -> &str {
        &self.record_kind
    }

    /// Returns the dependency marker id.
    pub fn dependency_marker_id(&self) -> &str {
        &self.dependency_marker_id
    }

    /// Returns the marker kind.
    pub const fn marker_kind(&self) -> MarkerKind {
        self.marker_kind
    }
}

/// Schema branch for `capability_lifecycle_audit_event_record`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityLifecycleAuditEventRecord {
    record_kind: String,
    capability_lifecycle_schema_version: u32,
    audit_event_id: AuditEventId,
    event_id: String,
    occurred_at: String,
    capability_lifecycle_row_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    dependency_marker_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    previous_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    next_value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    affected_axis: Option<AffectedAxis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    denial_reason: Option<DenialReason>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    observer_subject: Option<String>,
    policy_context: PolicyContext,
    redaction_class: RedactionClass,
}

impl CapabilityLifecycleAuditEventRecord {
    /// Returns the schema record-kind discriminator.
    pub fn record_kind(&self) -> &str {
        &self.record_kind
    }

    /// Returns the audit event id.
    pub const fn audit_event_id(&self) -> AuditEventId {
        self.audit_event_id
    }
}

/// Capability reference embedded in a lifecycle row record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRef {
    capability_kind: CapabilityKind,
    capability_id: String,
}

/// Capability kind from the lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityKind {
    /// Setting capability.
    Setting,
    /// Bundle capability.
    Bundle,
    /// Command capability.
    Command,
    /// Provider-linked feature.
    ProviderLinkedFeature,
    /// Workspace capability.
    WorkspaceCapability,
    /// Extension capability.
    ExtensionCapability,
    /// SDK or API surface.
    SdkOrApiSurface,
    /// Docs capability.
    DocsCapability,
    /// Support capability.
    SupportCapability,
}

/// Policy context associated with a lifecycle record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    policy_epoch: String,
    trust_state: TrustState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    execution_context_id: Option<String>,
}

/// Trust state captured with a lifecycle record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    /// Trusted workspace.
    Trusted,
    /// Restricted workspace.
    Restricted,
}

/// Deprecation window required by deprecated rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeprecationWindow {
    announced_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    earliest_removal_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    committed_removal_at: Option<String>,
}

/// Summary of a live dependency marker embedded in a row record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyMarkerSummary {
    dependency_marker_id: String,
    marker_kind: MarkerKind,
    effect_on_parent: EffectOnParent,
}

/// Repair hook reference embedded in a dependency marker record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairHookRef {
    hook_kind: RepairHookKind,
    hook_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    display_label: Option<String>,
}

/// Repair hook kind from the lifecycle schema.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairHookKind {
    /// Enable a feature flag.
    EnableFeatureFlag,
    /// Request an admin policy change.
    RequestAdminPolicyChange,
    /// Request a provider link.
    RequestProviderLink,
    /// Reconnect a provider.
    ReconnectProvider,
    /// Upgrade a release channel.
    UpgradeReleaseChannel,
    /// Refresh freshness.
    RefreshFreshness,
    /// Migrate to a replacement capability.
    MigrateToReplacement,
    /// Request managed access.
    RequestManagedAccess,
    /// Contact support.
    ContactSupport,
}

/// Kill switch state attached to a lifecycle row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KillSwitchState {
    /// Kill switch is inactive.
    Inactive,
    /// Kill switch is armed but has not fired.
    Armed,
    /// Kill switch has fired.
    Tripped,
}

/// Axis affected by a lifecycle audit event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AffectedAxis {
    /// Lifecycle state axis.
    LifecycleState,
    /// Support class axis.
    SupportClass,
    /// Release channel axis.
    ReleaseChannel,
    /// Freshness class axis.
    FreshnessClass,
    /// Client scope axis.
    ClientScope,
    /// Kill switch axis.
    KillSwitch,
    /// Dependency marker axis.
    DependencyMarker,
    /// Schema version axis.
    SchemaVersion,
}
