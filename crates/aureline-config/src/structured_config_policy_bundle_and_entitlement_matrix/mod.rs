//! Canonical matrix for structured config, signed policy bundles, offline
//! entitlements, and authored/effective/live truth.
//!
//! The packet in this module freezes the cross-surface control vocabulary for
//! M5-era config-bearing artifacts without introducing a new runtime resolver.
//! Shell settings, request-workspace setup, notebook runtime selection,
//! preview/runtime lanes, workflow bundles, admin policy review, support
//! exports, and release evidence all consume the same metadata-only object:
//!
//! - [`StructuredConfigControlMatrix`] is the canonical checked-in packet.
//! - [`ArtifactFamilyRow`] names each config-bearing family and whether it has
//!   authored source, effective projection, and live or observed state.
//! - [`BundleTaxonomyRow`] freezes the signed-bundle taxonomy and required
//!   envelope fields for policy, entitlement, emergency-disable, and trust-root
//!   or signer-update objects.
//! - [`ProfileQualificationRow`] declares what local-only, managed,
//!   self-hosted, mirrored, and fully air-gapped deployments may honestly
//!   claim.
//!
//! The packet is metadata-only. It carries opaque refs, bounded labels, and
//! closed vocabularies only; it does not embed raw bundle payloads, secrets,
//! trust-root bytes, network endpoints, or provider-specific identifiers.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Stable record-kind tag for the checked-in matrix packet.
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_RECORD_KIND: &str =
    "config_structured_config_policy_bundle_and_entitlement_matrix";

/// Schema version for [`StructuredConfigControlMatrix`].
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref quoted by every consumer.
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SHARED_CONTRACT_REF: &str =
    "config:structured_config_policy_bundle_and_entitlement_matrix:v1";

/// Repo-relative path to the checked-in canonical packet.
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_PATH: &str =
    "artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json";

/// Reviewer-facing notice repeated on support and release surfaces.
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_NOTICE: &str =
    "Structured config, policy bundle, and offline entitlement truth remains reviewable: \
     authored source, effective projection, and live or observed state are labeled rather than \
     collapsed; secrets stay handle-first; signed policy, entitlement, emergency-disable, and \
     trust-root update bundles stay portable across live, mirror, manual-import, and offline \
     paths; stale or last-known-good authority narrows managed actions without hiding the local-safe \
     floor; and preview-dependent configuration rows never present as stable-looking defaults.";

/// Artifact family covered by the control matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamilyKind {
    /// Request-workspace environment descriptors and scoped launch inputs.
    RequestWorkspaceEnvironment,
    /// Database connection and statement-execution profiles.
    DatabaseProfile,
    /// API and service-request profiles.
    ApiProfile,
    /// Notebook runtime manifests and kernel bindings.
    NotebookRuntimeManifest,
    /// Preview/runtime descriptors and preview-bound environment overlays.
    PreviewRuntimeConfig,
    /// Workflow bundle manifests and imported execution bundles.
    WorkflowBundleManifest,
    /// CI environment descriptors.
    CiEnvironmentDescriptor,
    /// Infrastructure/runtime environment descriptors.
    InfraEnvironmentDescriptor,
    /// Managed policy overlays projected into local review surfaces.
    ManagedPolicyOverlay,
    /// Signed admin policy bundle review object.
    AdminPolicyBundleArtifact,
    /// Offline entitlement snapshot review object.
    OfflineEntitlementSnapshotArtifact,
    /// Emergency disable bundle review object.
    EmergencyDisableBundleArtifact,
    /// Trust-root and signer-update review object.
    TrustRootSignerUpdateArtifact,
}

impl ArtifactFamilyKind {
    /// All required artifact families.
    pub const ALL: [Self; 13] = [
        Self::RequestWorkspaceEnvironment,
        Self::DatabaseProfile,
        Self::ApiProfile,
        Self::NotebookRuntimeManifest,
        Self::PreviewRuntimeConfig,
        Self::WorkflowBundleManifest,
        Self::CiEnvironmentDescriptor,
        Self::InfraEnvironmentDescriptor,
        Self::ManagedPolicyOverlay,
        Self::AdminPolicyBundleArtifact,
        Self::OfflineEntitlementSnapshotArtifact,
        Self::EmergencyDisableBundleArtifact,
        Self::TrustRootSignerUpdateArtifact,
    ];
}

/// Public qualification label for one row in the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationLabel {
    /// Claim is fully frozen for downstream reuse.
    Stable,
    /// Contract is frozen, but the family remains explicitly narrower than the stable line.
    Beta,
    /// Contract is frozen as preview-only and may not masquerade as stable.
    Preview,
}

impl QualificationLabel {
    fn is_preview(self) -> bool {
        matches!(self, Self::Preview)
    }

    fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// How much live or observed state the family can present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveStatePosture {
    /// No live or observed state is part of the family.
    NotSupported,
    /// Runtime convergence is deferred and only planned previews are available.
    DeferredByRuntime,
    /// Observed state is inspect-only from a live runtime or target.
    InspectOnlyObserved,
    /// Observed state is inspect-only from a mirror, cache, or snapshot.
    InspectOnlyMirrored,
}

/// Source layer that may appear in the row's explainability chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLayerClass {
    /// Embedded product or artifact defaults.
    EmbeddedDefault,
    /// Imported profile or authored team baseline.
    ProfileImport,
    /// Workspace-authored source file.
    WorkspaceSource,
    /// User or local-machine override.
    UserOverride,
    /// Signed admin policy bundle.
    PolicyBundle,
    /// Managed service overlay bounded by signed policy.
    ManagedOverride,
    /// Secret handle or reference projection.
    SecretReference,
    /// Runtime discovery or target probe.
    RuntimeDiscovery,
    /// Observed state from the target runtime.
    LiveObservation,
}

/// Secret or reference handling posture for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecretHandlingClass {
    /// Family is not expected to carry secret-bearing fields.
    NoSecretsExpected,
    /// Secret values stay as handles or references by default.
    ReferenceOrHandleOnly,
    /// Surfaces render redacted placeholders only.
    RedactedPlaceholderOnly,
    /// Surfaces expose a key path or locator only.
    KeyPathLocatorOnly,
}

/// Policy lock posture rendered for the family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PolicyLockPosture {
    /// Family is not policy-governed.
    NotPolicyOwned,
    /// Family may be narrowed or locked by signed policy.
    NarrowableByPolicy,
    /// Family is authored as a signed bundle and is review-only locally.
    SignedBundleOwned,
    /// Family may be ratcheted narrower by an emergency disable bundle.
    EmergencyDisableRatchet,
}

/// Portable or offline-capable distribution path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DistributionPath {
    /// Local reviewable file under user or workspace control.
    LocalFile,
    /// Live signed origin.
    SignedOrigin,
    /// Signed mirror or curated registry.
    SignedMirror,
    /// Admin-reviewed manual import.
    ManualImport,
    /// Offline bundle or transferred package.
    OfflineBundle,
    /// Preseeded last-known-good or shipped cache.
    PreseededCache,
}

/// Explicit downgrade or disclosure label required by the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeLabelClass {
    /// Local-safe mode remains available and visible.
    LocalSafeOnly,
    /// Last-known-good state is active and visible.
    LastKnownGood,
    /// Effective projection is stale and visibly labeled.
    StaleEffectiveProjection,
    /// Observed state is stale and visibly labeled.
    StaleLiveObservation,
    /// Managed actions pause while local work continues.
    ManagedActionsPaused,
    /// Mirror snapshot is active and visible.
    MirrorSnapshotInUse,
    /// Offline entitlement or bundle snapshot is required.
    OfflineSnapshotRequired,
    /// Preview-only dependency is disclosed rather than hidden.
    PreviewDependencyDisclosed,
    /// Policy lock or signed ownership is visibly labeled.
    PolicyLocked,
}

/// One config-bearing artifact family row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactFamilyRow {
    /// Stable family kind.
    pub family: ArtifactFamilyKind,
    /// Opaque ref safe for support exports and release evidence.
    pub family_ref: String,
    /// Reviewable one-line summary.
    pub summary: String,
    /// Claimed stability label for the family.
    pub qualification_label: QualificationLabel,
    /// Whether the family has a reviewable authored source object.
    pub authored_source: bool,
    /// Whether the family has a reviewable effective or resolved projection.
    pub effective_projection: bool,
    /// Whether the family has inspectable live or observed state.
    pub live_state_posture: LiveStatePosture,
    /// Explainability chain the family surfaces.
    pub source_layers: Vec<SourceLayerClass>,
    /// Secret or reference handling posture.
    pub secret_handling: SecretHandlingClass,
    /// Policy-lock posture.
    pub policy_lock_posture: PolicyLockPosture,
    /// Distribution paths the family must preserve.
    pub distribution_paths: Vec<DistributionPath>,
    /// Required downgrade or disclosure labels.
    pub downgrade_labels: Vec<DowngradeLabelClass>,
    /// Whether the family remains export-safe for support packets.
    pub support_export_safe: bool,
}

/// Signed bundle class frozen by the taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleClass {
    /// Signed admin policy bundle.
    AdminPolicyBundle,
    /// Offline entitlement snapshot.
    OfflineEntitlementSnapshot,
    /// Signed emergency disable bundle.
    EmergencyDisableBundle,
    /// Trust-root or signer-update bundle.
    TrustRootSignerUpdate,
}

impl BundleClass {
    /// All required bundle classes.
    pub const ALL: [Self; 4] = [
        Self::AdminPolicyBundle,
        Self::OfflineEntitlementSnapshot,
        Self::EmergencyDisableBundle,
        Self::TrustRootSignerUpdate,
    ];
}

/// Required field class in a signed bundle envelope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeFieldClass {
    /// Stable bundle id.
    BundleId,
    /// Bundle class discriminator.
    BundleClass,
    /// Schema version or envelope version.
    SchemaVersion,
    /// Bundle issuance time.
    IssuedAt,
    /// Earliest effective time.
    ValidFrom,
    /// Expiry or latest validity time.
    ValidUntil,
    /// Signer identity reference.
    SignerRef,
    /// Trust-root reference.
    TrustRootRef,
    /// Scope or tenant reference.
    ScopeRef,
    /// Content digest reference.
    DigestRef,
    /// Opaque payload reference.
    PayloadRef,
    /// Precedence layer declaration.
    PrecedenceLayer,
    /// Distribution-path declaration.
    DistributionPath,
    /// Grace-window declaration.
    GraceWindow,
    /// Superseded bundle references.
    SupersedesRefs,
    /// Revoked bundle or signer references.
    RevokesRefs,
    /// Emergency reason or action reference.
    EmergencyReason,
    /// Rotation-overlap or signer-roll window declaration.
    RotationWindow,
}

/// Precedence layer applied by a signed bundle class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundlePrecedenceClass {
    /// Admin ceiling and signed local authority.
    SignedLocalAdminBundle,
    /// Managed entitlement gating over hosted features.
    EntitlementGate,
    /// Highest-priority emergency ratchet.
    EmergencyDisable,
    /// Trust-root and signer continuity gate.
    TrustRootUpdateGate,
}

/// Expiry guidance frozen for a bundle class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpiryGuidanceClass {
    /// Bundle must carry expiry and a grace window.
    GraceWindowRequired,
    /// Snapshot may continue as last-known-good within a declared grace window.
    LastKnownGoodGraceAllowed,
    /// Bundle remains effective until superseded or explicitly expired.
    SupersedesOrExpiryRequired,
    /// Rotation overlap is mandatory during trust-root or signer change.
    RotationOverlapRequired,
}

/// Local-safe continuation posture promised by a profile or bundle class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalSafeLabelClass {
    /// Continue locally without managed dependencies.
    ContinueLocalOnly,
    /// Continue with last-known-good signed state.
    ContinueWithLastKnownGood,
    /// Continue with a mirrored snapshot.
    ContinueWithMirrorSnapshot,
    /// Continue with an offline-transferred snapshot.
    ContinueWithOfflineSnapshot,
    /// Continue local work while managed actions are paused.
    ContinueLocalWhileManagedActionsPause,
}

/// One signed bundle taxonomy row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BundleTaxonomyRow {
    /// Stable bundle class.
    pub bundle_class: BundleClass,
    /// Opaque ref for the envelope family.
    pub bundle_ref: String,
    /// Reviewable envelope summary.
    pub summary: String,
    /// Required field classes for the signed envelope.
    pub required_envelope_fields: Vec<EnvelopeFieldClass>,
    /// Precedence layer applied by this bundle class.
    pub precedence_layer: BundlePrecedenceClass,
    /// Frozen expiry guidance.
    pub expiry_guidance: ExpiryGuidanceClass,
    /// Whether a successor relation is required and reviewable.
    pub supports_supersedes: bool,
    /// Whether revocation or revokes relations are required and reviewable.
    pub supports_revokes: bool,
    /// Distribution paths the class must preserve.
    pub distribution_paths: Vec<DistributionPath>,
    /// User-visible stale-state label.
    pub stale_label: DowngradeLabelClass,
    /// Local-safe continuation when the class is stale or unavailable.
    pub local_safe_label: LocalSafeLabelClass,
}

/// Deployment profile frozen by the qualification section.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentProfileKind {
    /// Local-only desktop and OSS-first posture.
    LocalOnly,
    /// Managed online control plane.
    Managed,
    /// Customer-operated or sovereign control plane.
    SelfHosted,
    /// Signed-mirror-first posture.
    Mirrored,
    /// Fully air-gapped posture.
    FullyAirGapped,
}

impl DeploymentProfileKind {
    /// All required profile rows.
    pub const ALL: [Self; 5] = [
        Self::LocalOnly,
        Self::Managed,
        Self::SelfHosted,
        Self::Mirrored,
        Self::FullyAirGapped,
    ];
}

/// How much a profile depends on live managed authorization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedAuthDependencyClass {
    /// No managed authorization is required for the baseline profile.
    NoneLocalOnly,
    /// Live managed auth is preferred, but last-known-good narrows safely.
    ManagedLiveOrLastKnownGood,
    /// Customer-operated auth is preferred, but last-known-good narrows safely.
    SelfHostedLiveOrLastKnownGood,
    /// Mirror or snapshot authority is expected alongside local admin cache.
    MirrorSnapshotWithLocalAdminCache,
    /// Offline-transferred snapshots and local admin cache are required.
    OfflineSnapshotAndLocalAdminCache,
}

/// Known limit that a profile row must disclose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownLimitClass {
    /// No managed plane exists for the baseline profile.
    NoManagedPlane,
    /// No authoritative live observation exists; mirrored or deferred truth only.
    NoAuthoritativeLiveObservation,
    /// Trust-root and signer updates may require manual review or import.
    ManualRotationRequired,
    /// Mirror freshness may legitimately become stale and must stay labeled.
    MirrorFreshnessMayStale,
    /// Hosted capabilities may pause while local-safe work continues.
    HostedCapabilitiesMayPause,
    /// Preview rows remain visibly narrower than the stable line.
    PreviewRowsRemainNarrowed,
}

/// Qualification row for one deployment profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileQualificationRow {
    /// Frozen deployment profile.
    pub profile: DeploymentProfileKind,
    /// Claimed label for the profile contract.
    pub qualification_label: QualificationLabel,
    /// Bundle classes that must remain reviewable on the profile.
    pub required_bundle_classes: Vec<BundleClass>,
    /// Distribution paths allowed or required on the profile.
    pub distribution_paths: Vec<DistributionPath>,
    /// Managed-auth dependency posture.
    pub managed_auth_dependency: ManagedAuthDependencyClass,
    /// Local-safe continuation promise.
    pub local_safe_label: LocalSafeLabelClass,
    /// Required downgrade labels the profile must render.
    pub downgrade_labels: Vec<DowngradeLabelClass>,
    /// Known limits the profile must disclose.
    pub known_limits: Vec<KnownLimitClass>,
    /// Whether authoritative live observation is in scope on this profile.
    pub supports_authoritative_live_observation: bool,
    /// Whether support/export-safe truth remains available on this profile.
    pub support_export_safe: bool,
}

/// Derived summary for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatrixSummary {
    /// Number of artifact-family rows.
    pub artifact_family_count: usize,
    /// Number of bundle taxonomy rows.
    pub bundle_class_count: usize,
    /// Number of deployment-profile rows.
    pub profile_count: usize,
    /// Number of rows explicitly held below stable.
    pub non_stable_artifact_family_count: usize,
    /// Whether authored, effective, and live truth remain distinct.
    pub truth_planes_never_collapsed: bool,
    /// Whether every profile preserves a local-safe or last-known-good posture.
    pub local_safe_continuation_preserved: bool,
    /// Whether preview or beta rows are explicitly disclosed rather than hidden.
    pub narrower_rows_explicit: bool,
}

/// Canonical metadata-only control matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredConfigControlMatrix {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub matrix_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Config-bearing artifact families.
    pub artifact_families: Vec<ArtifactFamilyRow>,
    /// Signed bundle taxonomy.
    pub bundle_taxonomy: Vec<BundleTaxonomyRow>,
    /// Deployment profile qualification rows.
    pub profile_qualifications: Vec<ProfileQualificationRow>,
    /// Derived packet summary.
    pub summary: MatrixSummary,
    /// Companion documentation ref.
    pub docs_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
}

/// Validation failures for the control matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatrixValidationError {
    /// A required artifact family is missing.
    MissingArtifactFamily(ArtifactFamilyKind),
    /// An artifact family appears more than once.
    DuplicateArtifactFamily(ArtifactFamilyKind),
    /// A required bundle class is missing.
    MissingBundleClass(BundleClass),
    /// A bundle class appears more than once.
    DuplicateBundleClass(BundleClass),
    /// A required profile row is missing.
    MissingProfile(DeploymentProfileKind),
    /// A profile row appears more than once.
    DuplicateProfile(DeploymentProfileKind),
    /// A row fails to preserve distinct authored/effective/live truth.
    ArtifactTruthCollapsed(ArtifactFamilyKind),
    /// A non-stable row is not explicitly disclosed.
    NarrowerRowHidden(ArtifactFamilyKind),
    /// A bundle omits core signed-envelope fields.
    BundleMissingCoreEnvelopeFields(BundleClass),
    /// A profile does not preserve local-safe continuity.
    ProfileMissingLocalSafeContinuation(DeploymentProfileKind),
    /// The local-only profile incorrectly depends on managed auth.
    LocalOnlyDependsOnManagedAuth,
    /// The air-gapped profile allows a live signed origin.
    AirGappedAllowsLiveOrigin,
    /// A count in the derived summary drifted from the rows.
    SummaryCountMismatch {
        /// Field name that drifted.
        field: &'static str,
        /// Expected count.
        expected: usize,
        /// Actual count from the summary.
        actual: usize,
    },
}

impl fmt::Display for MatrixValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArtifactFamily(family) => {
                write!(f, "missing artifact family row: {family:?}")
            }
            Self::DuplicateArtifactFamily(family) => {
                write!(f, "duplicate artifact family row: {family:?}")
            }
            Self::MissingBundleClass(class) => write!(f, "missing bundle class row: {class:?}"),
            Self::DuplicateBundleClass(class) => {
                write!(f, "duplicate bundle class row: {class:?}")
            }
            Self::MissingProfile(profile) => write!(f, "missing profile row: {profile:?}"),
            Self::DuplicateProfile(profile) => write!(f, "duplicate profile row: {profile:?}"),
            Self::ArtifactTruthCollapsed(family) => {
                write!(
                    f,
                    "artifact family collapses authored/effective/live truth: {family:?}"
                )
            }
            Self::NarrowerRowHidden(family) => {
                write!(f, "artifact family is narrower than stable without an explicit disclosure: {family:?}")
            }
            Self::BundleMissingCoreEnvelopeFields(class) => {
                write!(f, "bundle class is missing core envelope fields: {class:?}")
            }
            Self::ProfileMissingLocalSafeContinuation(profile) => {
                write!(
                    f,
                    "profile row does not preserve local-safe continuity: {profile:?}"
                )
            }
            Self::LocalOnlyDependsOnManagedAuth => {
                write!(
                    f,
                    "local-only profile must not depend on managed authorization"
                )
            }
            Self::AirGappedAllowsLiveOrigin => {
                write!(
                    f,
                    "fully air-gapped profile must not allow a live signed origin"
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
        }
    }
}

impl std::error::Error for MatrixValidationError {}

impl StructuredConfigControlMatrix {
    /// Returns a compact support-export projection.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("matrix_id: {}", self.matrix_id),
            format!(
                "artifact_family_count: {}",
                self.summary.artifact_family_count
            ),
            format!("bundle_class_count: {}", self.summary.bundle_class_count),
            format!("profile_count: {}", self.summary.profile_count),
            format!(
                "local_safe_continuation_preserved: {}",
                self.summary.local_safe_continuation_preserved
            ),
            format!(
                "truth_planes_never_collapsed: {}",
                self.summary.truth_planes_never_collapsed
            ),
            format!(
                "narrower_rows_explicit: {}",
                self.summary.narrower_rows_explicit
            ),
        ]
    }
}

/// Returns the deterministic canonical matrix.
pub fn seeded_structured_config_policy_bundle_and_entitlement_matrix(
) -> StructuredConfigControlMatrix {
    let artifact_families = seeded_artifact_families();
    let bundle_taxonomy = seeded_bundle_taxonomy();
    let profile_qualifications = seeded_profile_qualifications();
    let summary = derive_summary(
        &artifact_families,
        &bundle_taxonomy,
        &profile_qualifications,
    );

    StructuredConfigControlMatrix {
        record_kind: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_RECORD_KIND.to_owned(),
        schema_version: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SCHEMA_VERSION,
        matrix_id: "config:structured-policy-entitlement-matrix".to_owned(),
        shared_contract_ref: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SHARED_CONTRACT_REF
            .to_owned(),
        notice: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_NOTICE.to_owned(),
        artifact_families,
        bundle_taxonomy,
        profile_qualifications,
        summary,
        docs_ref: "docs/config/structured_config_policy_bundle_and_entitlement_matrix.md"
            .to_owned(),
        schema_ref:
            "schemas/config/structured_config_policy_bundle_and_entitlement_matrix.schema.json"
                .to_owned(),
    }
}

/// Parses a matrix packet from JSON text.
pub fn parse_structured_config_policy_bundle_and_entitlement_matrix(
    json: &str,
) -> Result<StructuredConfigControlMatrix, serde_json::Error> {
    serde_json::from_str(json)
}

/// Audits the control matrix and returns every violation found.
pub fn audit_structured_config_policy_bundle_and_entitlement_matrix(
    matrix: &StructuredConfigControlMatrix,
) -> Vec<MatrixValidationError> {
    let mut defects = Vec::new();

    append_presence_defects(
        &mut defects,
        &matrix.artifact_families,
        ArtifactFamilyKind::ALL.as_slice(),
        |row| row.family,
        MatrixValidationError::MissingArtifactFamily,
        MatrixValidationError::DuplicateArtifactFamily,
    );
    append_presence_defects(
        &mut defects,
        &matrix.bundle_taxonomy,
        BundleClass::ALL.as_slice(),
        |row| row.bundle_class,
        MatrixValidationError::MissingBundleClass,
        MatrixValidationError::DuplicateBundleClass,
    );
    append_presence_defects(
        &mut defects,
        &matrix.profile_qualifications,
        DeploymentProfileKind::ALL.as_slice(),
        |row| row.profile,
        MatrixValidationError::MissingProfile,
        MatrixValidationError::DuplicateProfile,
    );

    for row in &matrix.artifact_families {
        let truth_planes_distinct = row.authored_source
            || row.live_state_posture != LiveStatePosture::NotSupported
            || (row.effective_projection && !row.source_layers.is_empty());
        if !truth_planes_distinct || !row.effective_projection {
            defects.push(MatrixValidationError::ArtifactTruthCollapsed(row.family));
        }
        if !row.qualification_label.is_stable()
            && (row.downgrade_labels.is_empty()
                || (row.qualification_label.is_preview()
                    && !row
                        .downgrade_labels
                        .contains(&DowngradeLabelClass::PreviewDependencyDisclosed)))
        {
            defects.push(MatrixValidationError::NarrowerRowHidden(row.family));
        }
    }

    let core_fields = [
        EnvelopeFieldClass::BundleId,
        EnvelopeFieldClass::BundleClass,
        EnvelopeFieldClass::SchemaVersion,
        EnvelopeFieldClass::IssuedAt,
        EnvelopeFieldClass::SignerRef,
        EnvelopeFieldClass::ScopeRef,
        EnvelopeFieldClass::DigestRef,
        EnvelopeFieldClass::PayloadRef,
        EnvelopeFieldClass::DistributionPath,
    ];
    for row in &matrix.bundle_taxonomy {
        let fields: BTreeSet<EnvelopeFieldClass> =
            row.required_envelope_fields.iter().copied().collect();
        if core_fields.iter().any(|field| !fields.contains(field)) {
            defects.push(MatrixValidationError::BundleMissingCoreEnvelopeFields(
                row.bundle_class,
            ));
        }
    }

    for row in &matrix.profile_qualifications {
        if !matches!(
            row.local_safe_label,
            LocalSafeLabelClass::ContinueLocalOnly
                | LocalSafeLabelClass::ContinueWithLastKnownGood
                | LocalSafeLabelClass::ContinueWithMirrorSnapshot
                | LocalSafeLabelClass::ContinueWithOfflineSnapshot
                | LocalSafeLabelClass::ContinueLocalWhileManagedActionsPause
        ) {
            defects.push(MatrixValidationError::ProfileMissingLocalSafeContinuation(
                row.profile,
            ));
        }
    }

    if matrix
        .profile_qualifications
        .iter()
        .find(|row| row.profile == DeploymentProfileKind::LocalOnly)
        .is_some_and(|row| row.managed_auth_dependency != ManagedAuthDependencyClass::NoneLocalOnly)
    {
        defects.push(MatrixValidationError::LocalOnlyDependsOnManagedAuth);
    }

    if matrix
        .profile_qualifications
        .iter()
        .find(|row| row.profile == DeploymentProfileKind::FullyAirGapped)
        .is_some_and(|row| {
            row.distribution_paths
                .contains(&DistributionPath::SignedOrigin)
        })
    {
        defects.push(MatrixValidationError::AirGappedAllowsLiveOrigin);
    }

    let expected_summary = derive_summary(
        &matrix.artifact_families,
        &matrix.bundle_taxonomy,
        &matrix.profile_qualifications,
    );
    compare_summary_count(
        &mut defects,
        "artifact_family_count",
        expected_summary.artifact_family_count,
        matrix.summary.artifact_family_count,
    );
    compare_summary_count(
        &mut defects,
        "bundle_class_count",
        expected_summary.bundle_class_count,
        matrix.summary.bundle_class_count,
    );
    compare_summary_count(
        &mut defects,
        "profile_count",
        expected_summary.profile_count,
        matrix.summary.profile_count,
    );
    compare_summary_count(
        &mut defects,
        "non_stable_artifact_family_count",
        expected_summary.non_stable_artifact_family_count,
        matrix.summary.non_stable_artifact_family_count,
    );

    defects
}

/// Validates a control matrix, returning every defect when validation fails.
pub fn validate_structured_config_policy_bundle_and_entitlement_matrix(
    matrix: &StructuredConfigControlMatrix,
) -> Result<(), Vec<MatrixValidationError>> {
    let defects = audit_structured_config_policy_bundle_and_entitlement_matrix(matrix);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

fn compare_summary_count(
    defects: &mut Vec<MatrixValidationError>,
    field: &'static str,
    expected: usize,
    actual: usize,
) {
    if expected != actual {
        defects.push(MatrixValidationError::SummaryCountMismatch {
            field,
            expected,
            actual,
        });
    }
}

fn append_presence_defects<T, K>(
    defects: &mut Vec<MatrixValidationError>,
    rows: &[T],
    required: &[K],
    key: impl Fn(&T) -> K,
    missing: impl Fn(K) -> MatrixValidationError,
    duplicate: impl Fn(K) -> MatrixValidationError,
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

fn derive_summary(
    artifact_families: &[ArtifactFamilyRow],
    bundle_taxonomy: &[BundleTaxonomyRow],
    profile_qualifications: &[ProfileQualificationRow],
) -> MatrixSummary {
    MatrixSummary {
        artifact_family_count: artifact_families.len(),
        bundle_class_count: bundle_taxonomy.len(),
        profile_count: profile_qualifications.len(),
        non_stable_artifact_family_count: artifact_families
            .iter()
            .filter(|row| !row.qualification_label.is_stable())
            .count(),
        truth_planes_never_collapsed: artifact_families.iter().all(|row| {
            row.effective_projection
                && (row.authored_source
                    || row.live_state_posture != LiveStatePosture::NotSupported
                    || !row.source_layers.is_empty())
        }),
        local_safe_continuation_preserved: profile_qualifications.iter().all(|row| {
            row.support_export_safe
                && !row.downgrade_labels.is_empty()
                && !matches!(
                    row.local_safe_label,
                    LocalSafeLabelClass::ContinueLocalWhileManagedActionsPause
                )
                || row.profile == DeploymentProfileKind::Managed
        }),
        narrower_rows_explicit: artifact_families.iter().all(|row| {
            row.qualification_label.is_stable()
                || !row.downgrade_labels.is_empty()
                    && (!row.qualification_label.is_preview()
                        || row
                            .downgrade_labels
                            .contains(&DowngradeLabelClass::PreviewDependencyDisclosed))
        }),
    }
}

fn seeded_artifact_families() -> Vec<ArtifactFamilyRow> {
    use ArtifactFamilyKind as Family;
    use DistributionPath as Path;
    use DowngradeLabelClass as Label;
    use LiveStatePosture as Live;
    use PolicyLockPosture as Lock;
    use QualificationLabel as Qual;
    use SecretHandlingClass as Secret;
    use SourceLayerClass as Source;

    vec![
        ArtifactFamilyRow {
            family: Family::RequestWorkspaceEnvironment,
            family_ref: "family:request-workspace-environment".to_owned(),
            summary: "Request-workspace environments keep authored source, effective overlay, and deferred runtime apply distinct.".to_owned(),
            qualification_label: Qual::Beta,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::DeferredByRuntime,
            source_layers: vec![
                Source::WorkspaceSource,
                Source::ProfileImport,
                Source::PolicyBundle,
                Source::SecretReference,
                Source::RuntimeDiscovery,
            ],
            secret_handling: Secret::ReferenceOrHandleOnly,
            policy_lock_posture: Lock::NarrowableByPolicy,
            distribution_paths: vec![Path::LocalFile, Path::ManualImport, Path::OfflineBundle],
            downgrade_labels: vec![
                Label::PolicyLocked,
                Label::StaleEffectiveProjection,
                Label::PreviewDependencyDisclosed,
            ],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::DatabaseProfile,
            family_ref: "family:database-profile".to_owned(),
            summary: "Database profiles surface authored settings, effective connection policy, and inspect-only observed connectivity state.".to_owned(),
            qualification_label: Qual::Stable,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::InspectOnlyObserved,
            source_layers: vec![
                Source::WorkspaceSource,
                Source::UserOverride,
                Source::PolicyBundle,
                Source::SecretReference,
                Source::LiveObservation,
            ],
            secret_handling: Secret::ReferenceOrHandleOnly,
            policy_lock_posture: Lock::NarrowableByPolicy,
            distribution_paths: vec![Path::LocalFile, Path::ManualImport, Path::SignedMirror],
            downgrade_labels: vec![Label::PolicyLocked, Label::StaleLiveObservation],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::ApiProfile,
            family_ref: "family:api-profile".to_owned(),
            summary: "API profiles distinguish authored request defaults, effective policy-narrowed routes, and inspect-only observed runtime state.".to_owned(),
            qualification_label: Qual::Stable,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::InspectOnlyObserved,
            source_layers: vec![
                Source::WorkspaceSource,
                Source::UserOverride,
                Source::PolicyBundle,
                Source::SecretReference,
                Source::LiveObservation,
            ],
            secret_handling: Secret::ReferenceOrHandleOnly,
            policy_lock_posture: Lock::NarrowableByPolicy,
            distribution_paths: vec![Path::LocalFile, Path::ManualImport, Path::SignedMirror],
            downgrade_labels: vec![Label::PolicyLocked, Label::StaleLiveObservation],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::NotebookRuntimeManifest,
            family_ref: "family:notebook-runtime-manifest".to_owned(),
            summary: "Notebook runtime manifests preserve authored kernelspec intent, effective resolver choice, and inspect-only observed kernel state.".to_owned(),
            qualification_label: Qual::Beta,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::InspectOnlyObserved,
            source_layers: vec![
                Source::WorkspaceSource,
                Source::PolicyBundle,
                Source::RuntimeDiscovery,
                Source::LiveObservation,
            ],
            secret_handling: Secret::KeyPathLocatorOnly,
            policy_lock_posture: Lock::NarrowableByPolicy,
            distribution_paths: vec![Path::LocalFile, Path::ManualImport, Path::OfflineBundle],
            downgrade_labels: vec![
                Label::PolicyLocked,
                Label::ManagedActionsPaused,
                Label::PreviewDependencyDisclosed,
            ],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::PreviewRuntimeConfig,
            family_ref: "family:preview-runtime-config".to_owned(),
            summary: "Preview runtime config remains explicitly preview-scoped and never presents staged or captured preview state as a stable live row.".to_owned(),
            qualification_label: Qual::Preview,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::InspectOnlyObserved,
            source_layers: vec![
                Source::WorkspaceSource,
                Source::PolicyBundle,
                Source::ManagedOverride,
                Source::RuntimeDiscovery,
                Source::LiveObservation,
            ],
            secret_handling: Secret::RedactedPlaceholderOnly,
            policy_lock_posture: Lock::NarrowableByPolicy,
            distribution_paths: vec![
                Path::LocalFile,
                Path::ManualImport,
                Path::SignedMirror,
                Path::OfflineBundle,
            ],
            downgrade_labels: vec![
                Label::PreviewDependencyDisclosed,
                Label::StaleLiveObservation,
                Label::ManagedActionsPaused,
            ],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::WorkflowBundleManifest,
            family_ref: "family:workflow-bundle-manifest".to_owned(),
            summary: "Workflow bundle manifests keep authored bundle intent, effective imported policy, and deferred execution state separate.".to_owned(),
            qualification_label: Qual::Beta,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::DeferredByRuntime,
            source_layers: vec![
                Source::WorkspaceSource,
                Source::ProfileImport,
                Source::PolicyBundle,
                Source::RuntimeDiscovery,
            ],
            secret_handling: Secret::KeyPathLocatorOnly,
            policy_lock_posture: Lock::NarrowableByPolicy,
            distribution_paths: vec![
                Path::LocalFile,
                Path::ManualImport,
                Path::SignedMirror,
                Path::OfflineBundle,
            ],
            downgrade_labels: vec![
                Label::PolicyLocked,
                Label::StaleEffectiveProjection,
                Label::PreviewDependencyDisclosed,
            ],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::CiEnvironmentDescriptor,
            family_ref: "family:ci-environment-descriptor".to_owned(),
            summary: "CI environment descriptors expose authored source, effective policy overlay, and mirrored observation without implying write-back to the target.".to_owned(),
            qualification_label: Qual::Beta,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::InspectOnlyMirrored,
            source_layers: vec![
                Source::WorkspaceSource,
                Source::PolicyBundle,
                Source::ManagedOverride,
                Source::RuntimeDiscovery,
                Source::LiveObservation,
            ],
            secret_handling: Secret::RedactedPlaceholderOnly,
            policy_lock_posture: Lock::NarrowableByPolicy,
            distribution_paths: vec![
                Path::LocalFile,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
            ],
            downgrade_labels: vec![
                Label::MirrorSnapshotInUse,
                Label::ManagedActionsPaused,
                Label::PreviewDependencyDisclosed,
            ],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::InfraEnvironmentDescriptor,
            family_ref: "family:infra-environment-descriptor".to_owned(),
            summary: "Infrastructure descriptors preserve authored source and effective policy while mirrored observation remains inspect-only.".to_owned(),
            qualification_label: Qual::Beta,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::InspectOnlyMirrored,
            source_layers: vec![
                Source::WorkspaceSource,
                Source::PolicyBundle,
                Source::ManagedOverride,
                Source::RuntimeDiscovery,
                Source::LiveObservation,
            ],
            secret_handling: Secret::RedactedPlaceholderOnly,
            policy_lock_posture: Lock::NarrowableByPolicy,
            distribution_paths: vec![
                Path::LocalFile,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
            ],
            downgrade_labels: vec![
                Label::MirrorSnapshotInUse,
                Label::ManagedActionsPaused,
                Label::PreviewDependencyDisclosed,
            ],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::ManagedPolicyOverlay,
            family_ref: "family:managed-policy-overlay".to_owned(),
            summary: "Managed policy overlays remain reviewable as signed authored input plus effective narrowing; they never masquerade as user-owned defaults.".to_owned(),
            qualification_label: Qual::Stable,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::NotSupported,
            source_layers: vec![
                Source::PolicyBundle,
                Source::ManagedOverride,
                Source::EmbeddedDefault,
            ],
            secret_handling: Secret::NoSecretsExpected,
            policy_lock_posture: Lock::SignedBundleOwned,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            downgrade_labels: vec![Label::PolicyLocked, Label::LastKnownGood],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::AdminPolicyBundleArtifact,
            family_ref: "family:admin-policy-bundle".to_owned(),
            summary: "Admin policy bundles are signed authored objects with effective local review; they remain portable across live, mirror, manual, and offline paths.".to_owned(),
            qualification_label: Qual::Stable,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::NotSupported,
            source_layers: vec![Source::PolicyBundle, Source::EmbeddedDefault],
            secret_handling: Secret::NoSecretsExpected,
            policy_lock_posture: Lock::SignedBundleOwned,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            downgrade_labels: vec![Label::LastKnownGood, Label::PolicyLocked],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::OfflineEntitlementSnapshotArtifact,
            family_ref: "family:offline-entitlement-snapshot".to_owned(),
            summary: "Offline entitlement snapshots stay reviewable as signed authored inputs and visibly narrow managed availability when stale.".to_owned(),
            qualification_label: Qual::Stable,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::NotSupported,
            source_layers: vec![Source::PolicyBundle, Source::ManagedOverride],
            secret_handling: Secret::NoSecretsExpected,
            policy_lock_posture: Lock::SignedBundleOwned,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            downgrade_labels: vec![
                Label::LastKnownGood,
                Label::OfflineSnapshotRequired,
                Label::ManagedActionsPaused,
            ],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::EmergencyDisableBundleArtifact,
            family_ref: "family:emergency-disable-bundle".to_owned(),
            summary: "Emergency disable bundles remain explicit signed ratchets with visible local-safe consequences rather than silent feature disappearance.".to_owned(),
            qualification_label: Qual::Stable,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::NotSupported,
            source_layers: vec![Source::PolicyBundle, Source::ManagedOverride],
            secret_handling: Secret::NoSecretsExpected,
            policy_lock_posture: Lock::EmergencyDisableRatchet,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            downgrade_labels: vec![Label::ManagedActionsPaused, Label::LocalSafeOnly],
            support_export_safe: true,
        },
        ArtifactFamilyRow {
            family: Family::TrustRootSignerUpdateArtifact,
            family_ref: "family:trust-root-signer-update".to_owned(),
            summary: "Trust-root and signer updates remain signed, reviewable, and portable, with overlap and pause semantics explicit during rotation.".to_owned(),
            qualification_label: Qual::Stable,
            authored_source: true,
            effective_projection: true,
            live_state_posture: Live::NotSupported,
            source_layers: vec![Source::PolicyBundle, Source::EmbeddedDefault],
            secret_handling: Secret::NoSecretsExpected,
            policy_lock_posture: Lock::SignedBundleOwned,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            downgrade_labels: vec![Label::ManagedActionsPaused, Label::LastKnownGood],
            support_export_safe: true,
        },
    ]
}

fn seeded_bundle_taxonomy() -> Vec<BundleTaxonomyRow> {
    use BundleClass as Bundle;
    use BundlePrecedenceClass as Precedence;
    use DistributionPath as Path;
    use DowngradeLabelClass as Label;
    use EnvelopeFieldClass as Field;
    use ExpiryGuidanceClass as Expiry;
    use LocalSafeLabelClass as Safe;

    vec![
        BundleTaxonomyRow {
            bundle_class: Bundle::AdminPolicyBundle,
            bundle_ref: "bundle:admin-policy".to_owned(),
            summary: "Signed admin policy bundles declare scope, signer, trust root, grace, and successor links before any widening review.".to_owned(),
            required_envelope_fields: vec![
                Field::BundleId,
                Field::BundleClass,
                Field::SchemaVersion,
                Field::IssuedAt,
                Field::ValidFrom,
                Field::ValidUntil,
                Field::SignerRef,
                Field::TrustRootRef,
                Field::ScopeRef,
                Field::DigestRef,
                Field::PayloadRef,
                Field::PrecedenceLayer,
                Field::DistributionPath,
                Field::GraceWindow,
                Field::SupersedesRefs,
            ],
            precedence_layer: Precedence::SignedLocalAdminBundle,
            expiry_guidance: Expiry::GraceWindowRequired,
            supports_supersedes: true,
            supports_revokes: false,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            stale_label: Label::LastKnownGood,
            local_safe_label: Safe::ContinueWithLastKnownGood,
        },
        BundleTaxonomyRow {
            bundle_class: Bundle::OfflineEntitlementSnapshot,
            bundle_ref: "bundle:offline-entitlement-snapshot".to_owned(),
            summary: "Offline entitlement snapshots declare freshness, signer continuity, scope, and grace so stale managed authority never looks like generic auth failure.".to_owned(),
            required_envelope_fields: vec![
                Field::BundleId,
                Field::BundleClass,
                Field::SchemaVersion,
                Field::IssuedAt,
                Field::ValidFrom,
                Field::ValidUntil,
                Field::SignerRef,
                Field::TrustRootRef,
                Field::ScopeRef,
                Field::DigestRef,
                Field::PayloadRef,
                Field::DistributionPath,
                Field::GraceWindow,
                Field::SupersedesRefs,
                Field::RevokesRefs,
            ],
            precedence_layer: Precedence::EntitlementGate,
            expiry_guidance: Expiry::LastKnownGoodGraceAllowed,
            supports_supersedes: true,
            supports_revokes: true,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            stale_label: Label::OfflineSnapshotRequired,
            local_safe_label: Safe::ContinueWithOfflineSnapshot,
        },
        BundleTaxonomyRow {
            bundle_class: Bundle::EmergencyDisableBundle,
            bundle_ref: "bundle:emergency-disable".to_owned(),
            summary: "Emergency disable bundles are highest-priority signed ratchets with explicit target scope, reason, supersedes, and revoke relations.".to_owned(),
            required_envelope_fields: vec![
                Field::BundleId,
                Field::BundleClass,
                Field::SchemaVersion,
                Field::IssuedAt,
                Field::ValidFrom,
                Field::ValidUntil,
                Field::SignerRef,
                Field::TrustRootRef,
                Field::ScopeRef,
                Field::DigestRef,
                Field::PayloadRef,
                Field::PrecedenceLayer,
                Field::DistributionPath,
                Field::SupersedesRefs,
                Field::RevokesRefs,
                Field::EmergencyReason,
            ],
            precedence_layer: Precedence::EmergencyDisable,
            expiry_guidance: Expiry::SupersedesOrExpiryRequired,
            supports_supersedes: true,
            supports_revokes: true,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            stale_label: Label::ManagedActionsPaused,
            local_safe_label: Safe::ContinueLocalWhileManagedActionsPause,
        },
        BundleTaxonomyRow {
            bundle_class: Bundle::TrustRootSignerUpdate,
            bundle_ref: "bundle:trust-root-signer-update".to_owned(),
            summary: "Trust-root and signer-update bundles require overlap metadata, successor links, and revoke relations so rotation remains portable and reviewable.".to_owned(),
            required_envelope_fields: vec![
                Field::BundleId,
                Field::BundleClass,
                Field::SchemaVersion,
                Field::IssuedAt,
                Field::ValidFrom,
                Field::ValidUntil,
                Field::SignerRef,
                Field::TrustRootRef,
                Field::DigestRef,
                Field::PayloadRef,
                Field::DistributionPath,
                Field::SupersedesRefs,
                Field::RevokesRefs,
                Field::RotationWindow,
                Field::ScopeRef,
            ],
            precedence_layer: Precedence::TrustRootUpdateGate,
            expiry_guidance: Expiry::RotationOverlapRequired,
            supports_supersedes: true,
            supports_revokes: true,
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            stale_label: Label::ManagedActionsPaused,
            local_safe_label: Safe::ContinueWithLastKnownGood,
        },
    ]
}

fn seeded_profile_qualifications() -> Vec<ProfileQualificationRow> {
    use BundleClass as Bundle;
    use DeploymentProfileKind as Profile;
    use DistributionPath as Path;
    use DowngradeLabelClass as Label;
    use KnownLimitClass as Limit;
    use LocalSafeLabelClass as Safe;
    use ManagedAuthDependencyClass as Auth;
    use QualificationLabel as Qual;

    vec![
        ProfileQualificationRow {
            profile: Profile::LocalOnly,
            qualification_label: Qual::Stable,
            required_bundle_classes: vec![],
            distribution_paths: vec![Path::LocalFile, Path::ManualImport, Path::OfflineBundle],
            managed_auth_dependency: Auth::NoneLocalOnly,
            local_safe_label: Safe::ContinueLocalOnly,
            downgrade_labels: vec![Label::LocalSafeOnly, Label::PreviewDependencyDisclosed],
            known_limits: vec![
                Limit::NoManagedPlane,
                Limit::NoAuthoritativeLiveObservation,
                Limit::PreviewRowsRemainNarrowed,
            ],
            supports_authoritative_live_observation: false,
            support_export_safe: true,
        },
        ProfileQualificationRow {
            profile: Profile::Managed,
            qualification_label: Qual::Stable,
            required_bundle_classes: vec![
                Bundle::AdminPolicyBundle,
                Bundle::OfflineEntitlementSnapshot,
                Bundle::EmergencyDisableBundle,
                Bundle::TrustRootSignerUpdate,
            ],
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            managed_auth_dependency: Auth::ManagedLiveOrLastKnownGood,
            local_safe_label: Safe::ContinueWithLastKnownGood,
            downgrade_labels: vec![
                Label::LastKnownGood,
                Label::ManagedActionsPaused,
                Label::PreviewDependencyDisclosed,
            ],
            known_limits: vec![
                Limit::HostedCapabilitiesMayPause,
                Limit::PreviewRowsRemainNarrowed,
            ],
            supports_authoritative_live_observation: true,
            support_export_safe: true,
        },
        ProfileQualificationRow {
            profile: Profile::SelfHosted,
            qualification_label: Qual::Stable,
            required_bundle_classes: vec![
                Bundle::AdminPolicyBundle,
                Bundle::OfflineEntitlementSnapshot,
                Bundle::EmergencyDisableBundle,
                Bundle::TrustRootSignerUpdate,
            ],
            distribution_paths: vec![
                Path::SignedOrigin,
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            managed_auth_dependency: Auth::SelfHostedLiveOrLastKnownGood,
            local_safe_label: Safe::ContinueWithLastKnownGood,
            downgrade_labels: vec![
                Label::LastKnownGood,
                Label::ManagedActionsPaused,
                Label::PolicyLocked,
            ],
            known_limits: vec![
                Limit::ManualRotationRequired,
                Limit::HostedCapabilitiesMayPause,
                Limit::PreviewRowsRemainNarrowed,
            ],
            supports_authoritative_live_observation: true,
            support_export_safe: true,
        },
        ProfileQualificationRow {
            profile: Profile::Mirrored,
            qualification_label: Qual::Stable,
            required_bundle_classes: vec![
                Bundle::AdminPolicyBundle,
                Bundle::OfflineEntitlementSnapshot,
                Bundle::EmergencyDisableBundle,
                Bundle::TrustRootSignerUpdate,
            ],
            distribution_paths: vec![
                Path::SignedMirror,
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            managed_auth_dependency: Auth::MirrorSnapshotWithLocalAdminCache,
            local_safe_label: Safe::ContinueWithMirrorSnapshot,
            downgrade_labels: vec![
                Label::MirrorSnapshotInUse,
                Label::LastKnownGood,
                Label::ManagedActionsPaused,
            ],
            known_limits: vec![
                Limit::MirrorFreshnessMayStale,
                Limit::ManualRotationRequired,
                Limit::PreviewRowsRemainNarrowed,
            ],
            supports_authoritative_live_observation: false,
            support_export_safe: true,
        },
        ProfileQualificationRow {
            profile: Profile::FullyAirGapped,
            qualification_label: Qual::Stable,
            required_bundle_classes: vec![
                Bundle::AdminPolicyBundle,
                Bundle::OfflineEntitlementSnapshot,
                Bundle::EmergencyDisableBundle,
                Bundle::TrustRootSignerUpdate,
            ],
            distribution_paths: vec![
                Path::ManualImport,
                Path::OfflineBundle,
                Path::PreseededCache,
            ],
            managed_auth_dependency: Auth::OfflineSnapshotAndLocalAdminCache,
            local_safe_label: Safe::ContinueWithOfflineSnapshot,
            downgrade_labels: vec![
                Label::OfflineSnapshotRequired,
                Label::LocalSafeOnly,
                Label::ManagedActionsPaused,
            ],
            known_limits: vec![
                Limit::NoManagedPlane,
                Limit::NoAuthoritativeLiveObservation,
                Limit::ManualRotationRequired,
                Limit::PreviewRowsRemainNarrowed,
            ],
            supports_authoritative_live_observation: false,
            support_export_safe: true,
        },
    ]
}
