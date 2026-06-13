//! Certification packet for structured configuration, signed policy bundles,
//! offline entitlements, and admin-audit explainability across claimed
//! deployment profiles.
//!
//! This module closes the loop over the existing structured-config control
//! matrix, mode/layer packet, and parameter-source/save-review packet. It does
//! not introduce a new resolver or bundle validator; it publishes one
//! metadata-only certification packet that downstream release, Help/About,
//! support-export, docs/help, and shiproom surfaces can ingest verbatim when
//! they need to answer these questions:
//!
//! 1. Which config-bearing artifact families currently hold a certified claim,
//!    and which are narrowed to limited or retest-pending?
//! 2. Which deployment profiles currently hold a certified configuration truth
//!    claim, and which degrade to offline-only or retest-pending under stale
//!    policy, reauth, signer-rotation, or mirror/offline drift?
//! 3. Whether authored/source, effective/resolved, and live/observed truth
//!    remain labeled instead of collapsed for every claimed family.
//! 4. Whether signed policy bundles, offline entitlement snapshots,
//!    emergency-disable bundles, and signer-rotation packets remain portable
//!    across managed, mirror, manual-import, and offline paths with a visible
//!    local-safe floor.
//! 5. Whether release-center, Help/About, docs/help, support-export, and
//!    shiproom consumers all quote one current packet instead of cloning status
//!    prose.
//!
//! The packet is metadata-only. It carries opaque refs, review-safe summaries,
//! typed state, evidence timestamps, and closed vocabularies only. Raw bundle
//! payloads, raw secrets, raw provider payloads, and raw tenant identifiers do
//! not cross this boundary.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::structured_config_artifact_modes_and_layers::{
    seeded_structured_config_artifact_modes_and_layers, ArtifactSurfaceRow,
    StructuredConfigArtifactModesAndLayersPacket,
    STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF,
};
use crate::structured_config_parameter_source_and_round_trip_review::{
    seeded_structured_config_parameter_source_and_round_trip_review, ArtifactReviewRow,
    StructuredConfigParameterSourceRoundTripReviewPacket,
    STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SHARED_CONTRACT_REF,
};
use crate::structured_config_policy_bundle_and_entitlement_matrix::{
    seeded_structured_config_policy_bundle_and_entitlement_matrix, ArtifactFamilyKind,
    BundleClass, DeploymentProfileKind, DistributionPath, DowngradeLabelClass, KnownLimitClass,
    LocalSafeLabelClass, ManagedAuthDependencyClass, QualificationLabel,
    StructuredConfigControlMatrix, STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SHARED_CONTRACT_REF,
};

#[cfg(test)]
mod tests;

/// Stable record-kind tag for the certification packet.
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_RECORD_KIND: &str =
    "structured_config_policy_entitlement_certification";

/// Schema version for [`StructuredConfigPolicyEntitlementCertificationPacket`].
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref quoted by release, help, support, and shiproom surfaces.
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "config:structured_config_policy_entitlement_certification:v1";

/// Repo-relative path to the checked-in canonical packet.
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_PATH: &str =
    "artifacts/config/structured_config_policy_entitlement_certification.json";

/// Reviewer-facing notice repeated by downstream publication surfaces.
pub const STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_NOTICE: &str =
    "Structured-config certification keeps one release-safe truth source for configuration, signed \
     policy, offline entitlement, and admin explainability: source, effective, and live planes stay \
     labeled; signed bundles and offline snapshots stay portable across managed, mirror, \
     manual-import, and offline paths; stale or last-known-good authority narrows managed actions \
     without hiding the local-safe floor; and Help/About, docs/help, release-center, shiproom, and \
     support exports ingest one packet instead of drifting after promotion.";

const PACKET_AS_OF: &str = "2026-06-12T18:00:00Z";
const MAX_CERTIFIED_EVIDENCE_AGE_DAYS: u32 = 7;

/// Publication state surfaced for an artifact family or deployment profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationState {
    /// Fully claimable on the named row.
    Certified,
    /// Claimable only with explicit narrower or feature-limited language.
    Limited,
    /// Claimable only as an offline or mirrored local-safe posture.
    OfflineOnly,
    /// Not currently claimable without rerunning the named drills.
    RetestPending,
}

impl CertificationState {
    /// States that remain claimable after narrowing.
    pub const CLAIMABLE: [Self; 3] = [Self::Certified, Self::Limited, Self::OfflineOnly];

    /// Returns true when the state keeps some published claim.
    pub const fn is_claimable(self) -> bool {
        !matches!(self, Self::RetestPending)
    }
}

/// Why a row narrows below its ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The upstream row remains explicitly preview-scoped.
    PreviewDependency,
    /// The upstream row is beta depth and does not hold a fully certified claim.
    DepthIncomplete,
    /// Policy freshness is stale enough that the managed row must narrow.
    StalePolicyEvidence,
    /// Managed authorization must refresh before privileged actions resume.
    ReauthRequired,
    /// Signer continuity requires review before trust can widen.
    SignerRotationPending,
    /// The mirror or offline snapshot is in use and must stay labeled.
    MirrorOfflineFallback,
    /// A required reference-workspace or profile drill has not been re-run.
    ReferenceWorkspaceDrillMissing,
    /// The admin-safe audit/export path is incomplete.
    AdminAuditabilityIncomplete,
}

/// Drill required on every claimed deployment profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProfileDrillKind {
    /// Reference-workspace parity drill.
    ReferenceWorkspace,
    /// Mirror or offline distribution drill.
    MirrorOffline,
    /// Managed or self-hosted continuity drill.
    ManagedSelfHosted,
    /// Stale-policy continuity drill.
    StalePolicy,
    /// Reauth-required continuity drill.
    ReauthRequired,
    /// Signer-rotation continuity drill.
    SignerRotation,
}

impl ProfileDrillKind {
    /// All required profile drills.
    pub const ALL: [Self; 6] = [
        Self::ReferenceWorkspace,
        Self::MirrorOffline,
        Self::ManagedSelfHosted,
        Self::StalePolicy,
        Self::ReauthRequired,
        Self::SignerRotation,
    ];
}

/// Current outcome of a named profile drill.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillStatus {
    /// Drill passed and the row may widen through it.
    PassedCurrent,
    /// Drill passed in a narrowed continuity posture.
    PassedNarrowed,
    /// Drill is not relevant to the profile.
    NotApplicable,
    /// Drill must be rerun before the row may widen again.
    RetestPending,
}

impl DrillStatus {
    /// Returns true when the drill still blocks widening.
    pub const fn blocks_widening(self) -> bool {
        matches!(self, Self::RetestPending)
    }
}

/// Readiness of the admin-audit explainability path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdminAuditabilityState {
    /// Denials and narrowing export one admin-safe explanation packet.
    ExplainableExportReady,
    /// Export remains available, but only as a narrower local-safe packet.
    ExplainableLocalSafeOnly,
    /// Admin explainability is incomplete and the row may not widen.
    Incomplete,
}

/// Publication surface that must ingest this packet without restating it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationSurface {
    /// Release-center row or card.
    ReleaseCenter,
    /// Help/About truth surface.
    HelpAbout,
    /// Support export or support-center packet.
    SupportExport,
    /// Docs/help consumer surface.
    DocsHelp,
    /// Shiproom dashboard or shiproom review sheet.
    Shiproom,
}

impl PublicationSurface {
    /// Every publication surface that must ingest the packet.
    pub const ALL: [Self; 5] = [
        Self::ReleaseCenter,
        Self::HelpAbout,
        Self::SupportExport,
        Self::DocsHelp,
        Self::Shiproom,
    ];
}

/// Packet surface the family row primarily represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactLaneClass {
    /// User-authored or workspace-authored structured configuration.
    StructuredConfig,
    /// Signed policy, entitlement, emergency-disable, or signer-rotation review object.
    SignedBundleReview,
}

/// One current certification row for a config-bearing artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactCertificationRow {
    /// Stable family token inherited from the control matrix.
    pub family: ArtifactFamilyKind,
    /// High-level lane class for the family.
    pub lane_class: ArtifactLaneClass,
    /// Opaque ref safe for release/help/support surfaces.
    pub family_ref: String,
    /// Human-readable summary of what the row proves.
    pub summary: String,
    /// Upstream qualification ceiling inherited from the control matrix.
    pub claim_ceiling: QualificationLabel,
    /// Current published certification state.
    pub published_state: CertificationState,
    /// True when source/effective/live truth remains reviewable instead of collapsed.
    pub truth_planes_reviewable: bool,
    /// True when the family has an inspectable in-product mode/layer row.
    pub mode_layer_reviewable: bool,
    /// True when parameter-source/save-review depth is present where required.
    pub parameter_provenance_reviewable: bool,
    /// True when secret/reference class remains explicit.
    pub secret_reference_class_visible: bool,
    /// True when policy locks or signed ownership remain explicit.
    pub policy_lock_visible: bool,
    /// True when the family's signed-bundle or portable distribution path is reviewable.
    pub signed_path_reviewable: bool,
    /// Current admin-audit explainability state.
    pub admin_auditability: AdminAuditabilityState,
    /// Profiles on which the family remains claimable.
    pub supported_profiles: Vec<DeploymentProfileKind>,
    /// Exact evidence timestamp currently backing the row.
    pub evidence_as_of: String,
    /// Age in whole days for the current evidence.
    pub evidence_age_days: u32,
    /// Upstream packet refs this row quotes.
    pub upstream_packet_refs: Vec<String>,
    /// Explicit labels the row still renders when narrowed.
    pub visible_labels: Vec<DowngradeLabelClass>,
    /// Current reasons the row sits below its ceiling.
    #[serde(default)]
    pub narrowing_reasons: Vec<NarrowingReason>,
}

/// One current profile drill result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileDrillRow {
    /// Drill being reported.
    pub drill_kind: ProfileDrillKind,
    /// Current drill outcome.
    pub status: DrillStatus,
    /// Exact evidence timestamp for the drill.
    pub executed_at: String,
    /// Evidence ref safe for shiproom/help/support surfaces.
    pub evidence_ref: String,
    /// Human-readable drill summary.
    pub summary: String,
}

/// One current certification row for a deployment profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProfileCertificationRow {
    /// Deployment profile being certified.
    pub profile: DeploymentProfileKind,
    /// Upstream qualification ceiling inherited from the control matrix.
    pub claim_ceiling: QualificationLabel,
    /// Current published certification state.
    pub published_state: CertificationState,
    /// Bundle classes the profile keeps reviewable.
    pub required_bundle_classes: Vec<BundleClass>,
    /// Distribution paths still allowed on this profile.
    pub distribution_paths: Vec<DistributionPath>,
    /// Managed-auth dependency posture.
    pub managed_auth_dependency: ManagedAuthDependencyClass,
    /// Local-safe floor promised on the profile.
    pub local_safe_label: LocalSafeLabelClass,
    /// Known limits the profile must keep visible.
    pub known_limits: Vec<KnownLimitClass>,
    /// Profile drills proving the published state.
    pub drills: Vec<ProfileDrillRow>,
    /// Exact evidence timestamp currently backing the row.
    pub evidence_as_of: String,
    /// Age in whole days for the current evidence.
    pub evidence_age_days: u32,
    /// Current reasons the row sits below its ceiling.
    #[serde(default)]
    pub narrowing_reasons: Vec<NarrowingReason>,
}

/// One publication-surface binding proving this packet is the single source of truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceIngestionRow {
    /// User-/operator-facing surface.
    pub surface: PublicationSurface,
    /// Canonical packet ref the surface quotes.
    pub packet_ref: String,
    /// Whether the surface shows the published certification state.
    pub shows_published_state: bool,
    /// Whether the surface shows exact evidence age.
    pub shows_evidence_age: bool,
    /// Whether the surface shows the local-safe floor.
    pub shows_local_safe_floor: bool,
    /// Whether the surface shows supported profiles or rows.
    pub shows_supported_profiles: bool,
    /// Whether the surface shows downgrade or narrowing causes.
    pub shows_narrowing_reasons: bool,
}

/// One deterministic downgrade rule consumers reuse instead of re-deriving.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeRule {
    /// Trigger that narrows the row.
    pub trigger: NarrowingReason,
    /// Target state once the trigger fires.
    pub narrow_to: CertificationState,
    /// Reviewable explanation for the downgrade.
    pub rationale: String,
}

/// Packet refs quoted by the certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourcePacketRef {
    /// Upstream checked-in packet path.
    pub packet_ref: String,
    /// Shared contract ref frozen by the upstream packet.
    pub shared_contract_ref: String,
}

/// Derived packet summary for release review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationSummary {
    /// Number of artifact rows.
    pub artifact_family_count: usize,
    /// Number of certified artifact rows.
    pub certified_artifact_family_count: usize,
    /// Number of narrowed artifact rows.
    pub narrowed_artifact_family_count: usize,
    /// Number of profile rows.
    pub profile_count: usize,
    /// Number of certified profile rows.
    pub certified_profile_count: usize,
    /// Number of publication surfaces ingesting the packet.
    pub publication_surface_count: usize,
    /// Whether every claimed profile preserves a visible local-safe floor.
    pub local_safe_floor_visible_everywhere: bool,
    /// Whether release/help/support/docs surfaces all ingest this packet.
    pub publication_surfaces_aligned: bool,
}

/// Canonical certification packet for structured config and portable policy truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredConfigPolicyEntitlementCertificationPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Exact evaluation timestamp for the packet.
    pub as_of: String,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Upstream packet refs the certification quotes.
    pub upstream_packets: Vec<SourcePacketRef>,
    /// Artifact-family certification rows.
    pub artifact_rows: Vec<ArtifactCertificationRow>,
    /// Deployment-profile certification rows.
    pub profile_rows: Vec<ProfileCertificationRow>,
    /// Downstream publication-surface bindings.
    pub surface_rows: Vec<SurfaceIngestionRow>,
    /// Deterministic downgrade rules.
    pub downgrade_rules: Vec<DowngradeRule>,
    /// Derived summary.
    pub summary: CertificationSummary,
    /// Narrative config doc ref.
    pub docs_ref: String,
    /// Narrative help doc ref.
    pub help_doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
}

/// Validation defect for the certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificationValidationError {
    /// A required artifact row is missing.
    MissingArtifactFamily(ArtifactFamilyKind),
    /// An artifact row appears more than once.
    DuplicateArtifactFamily(ArtifactFamilyKind),
    /// A required deployment profile row is missing.
    MissingProfile(DeploymentProfileKind),
    /// A profile row appears more than once.
    DuplicateProfile(DeploymentProfileKind),
    /// A required publication surface row is missing.
    MissingSurface(PublicationSurface),
    /// A publication surface row appears more than once.
    DuplicateSurface(PublicationSurface),
    /// A profile row omitted a required drill.
    MissingProfileDrill {
        /// Profile that failed validation.
        profile: DeploymentProfileKind,
        /// Drill kind that is missing.
        drill_kind: ProfileDrillKind,
    },
    /// A certified artifact row uses stale evidence.
    CertifiedArtifactEvidenceStale(ArtifactFamilyKind),
    /// A certified profile row uses stale evidence.
    CertifiedProfileEvidenceStale(DeploymentProfileKind),
    /// A certified row still records narrowing reasons.
    CertifiedRowHasNarrowingReasons(&'static str, String),
    /// A narrowed row forgot to record narrowing reasons.
    NarrowedRowMissingReason(&'static str, String),
    /// A preview-scoped family published too wide a state.
    PreviewFamilyPublishedTooWide(ArtifactFamilyKind),
    /// A structured-config family lost parameter provenance review.
    StructuredFamilyMissingParameterProvenance(ArtifactFamilyKind),
    /// A signed-bundle family lost signed-path reviewability.
    SignedBundleFamilyMissingPathReview(ArtifactFamilyKind),
    /// Admin-audit explainability is incomplete on a certified row.
    CertifiedRowMissingAdminAuditability(ArtifactFamilyKind),
    /// A surface row does not fully ingest the packet.
    SurfaceDoesNotFullyIngest(PublicationSurface),
    /// A summary field drifted from the rows.
    SummaryCountMismatch {
        /// Summary field name.
        field: &'static str,
        /// Expected derived value.
        expected: usize,
        /// Stored summary value.
        actual: usize,
    },
}

impl fmt::Display for CertificationValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingArtifactFamily(family) => {
                write!(f, "missing artifact-family row: {family:?}")
            }
            Self::DuplicateArtifactFamily(family) => {
                write!(f, "duplicate artifact-family row: {family:?}")
            }
            Self::MissingProfile(profile) => write!(f, "missing profile row: {profile:?}"),
            Self::DuplicateProfile(profile) => write!(f, "duplicate profile row: {profile:?}"),
            Self::MissingSurface(surface) => write!(f, "missing surface row: {surface:?}"),
            Self::DuplicateSurface(surface) => write!(f, "duplicate surface row: {surface:?}"),
            Self::MissingProfileDrill { profile, drill_kind } => write!(
                f,
                "profile {profile:?} is missing required drill {drill_kind:?}"
            ),
            Self::CertifiedArtifactEvidenceStale(family) => {
                write!(f, "certified artifact-family evidence is stale: {family:?}")
            }
            Self::CertifiedProfileEvidenceStale(profile) => {
                write!(f, "certified profile evidence is stale: {profile:?}")
            }
            Self::CertifiedRowHasNarrowingReasons(kind, id) => {
                write!(f, "{kind} row {id} is certified but still records narrowing")
            }
            Self::NarrowedRowMissingReason(kind, id) => {
                write!(f, "{kind} row {id} is narrowed without a reason")
            }
            Self::PreviewFamilyPublishedTooWide(family) => {
                write!(f, "preview family published too wide a state: {family:?}")
            }
            Self::StructuredFamilyMissingParameterProvenance(family) => write!(
                f,
                "structured-config family lost parameter-source/save-review depth: {family:?}"
            ),
            Self::SignedBundleFamilyMissingPathReview(family) => write!(
                f,
                "signed-bundle family lost signed-path reviewability: {family:?}"
            ),
            Self::CertifiedRowMissingAdminAuditability(family) => write!(
                f,
                "certified artifact-family row is missing admin-audit explainability: {family:?}"
            ),
            Self::SurfaceDoesNotFullyIngest(surface) => {
                write!(f, "surface does not fully ingest the packet: {surface:?}")
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

impl std::error::Error for CertificationValidationError {}

/// Scenario used when materializing a certification packet or a degraded fixture.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificationScenario {
    /// Canonical checked-in certification state.
    Canonical,
    /// Managed and mirror rows narrow because policy evidence is stale.
    StalePolicy,
    /// Managed rows narrow because a fresh managed action requires reauth.
    ReauthRequired,
    /// Signed-bundle trust rows narrow because signer continuity needs review.
    SignerRotation,
}

impl StructuredConfigPolicyEntitlementCertificationPacket {
    /// Returns compact support-export summary lines.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("packet_id: {}", self.packet_id),
            format!("as_of: {}", self.as_of),
            format!(
                "certified_artifact_family_count: {}",
                self.summary.certified_artifact_family_count
            ),
            format!(
                "narrowed_artifact_family_count: {}",
                self.summary.narrowed_artifact_family_count
            ),
            format!("certified_profile_count: {}", self.summary.certified_profile_count),
            format!(
                "publication_surface_count: {}",
                self.summary.publication_surface_count
            ),
            format!(
                "local_safe_floor_visible_everywhere: {}",
                self.summary.local_safe_floor_visible_everywhere
            ),
            format!(
                "publication_surfaces_aligned: {}",
                self.summary.publication_surfaces_aligned
            ),
        ]
    }
}

/// Returns the deterministic canonical certification packet.
pub fn seeded_structured_config_policy_entitlement_certification(
) -> StructuredConfigPolicyEntitlementCertificationPacket {
    seeded_structured_config_policy_entitlement_certification_scenario(
        CertificationScenario::Canonical,
    )
}

/// Returns a deterministic certification packet for the requested scenario.
pub fn seeded_structured_config_policy_entitlement_certification_scenario(
    scenario: CertificationScenario,
) -> StructuredConfigPolicyEntitlementCertificationPacket {
    let matrix = seeded_structured_config_policy_bundle_and_entitlement_matrix();
    let modes = seeded_structured_config_artifact_modes_and_layers();
    let review = seeded_structured_config_parameter_source_and_round_trip_review();

    let mut artifact_rows = build_artifact_rows(&matrix, &modes, &review);
    let mut profile_rows = build_profile_rows(&matrix);
    apply_scenario(scenario, &mut artifact_rows, &mut profile_rows);
    let surface_rows = seeded_surface_rows();
    let downgrade_rules = seeded_downgrade_rules();
    let summary = derive_summary(&artifact_rows, &profile_rows, &surface_rows);

    StructuredConfigPolicyEntitlementCertificationPacket {
        record_kind: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_RECORD_KIND.to_owned(),
        schema_version: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_SCHEMA_VERSION,
        packet_id: "config:structured-policy-entitlement-certification".to_owned(),
        shared_contract_ref:
            STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        as_of: PACKET_AS_OF.to_owned(),
        notice: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_NOTICE.to_owned(),
        upstream_packets: vec![
            SourcePacketRef {
                packet_ref:
                    "artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json"
                        .to_owned(),
                shared_contract_ref:
                    STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SHARED_CONTRACT_REF.to_owned(),
            },
            SourcePacketRef {
                packet_ref: "artifacts/config/structured_config_artifact_modes_and_layers.json"
                    .to_owned(),
                shared_contract_ref:
                    STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF.to_owned(),
            },
            SourcePacketRef {
                packet_ref:
                    "artifacts/config/structured_config_parameter_source_and_round_trip_review.json"
                        .to_owned(),
                shared_contract_ref:
                    STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SHARED_CONTRACT_REF
                        .to_owned(),
            },
        ],
        artifact_rows,
        profile_rows,
        surface_rows,
        downgrade_rules,
        summary,
        docs_ref: "docs/config/structured_config_policy_entitlement_certification.md".to_owned(),
        help_doc_ref: "docs/help/structured_config_policy_entitlement_certification.md".to_owned(),
        schema_ref:
            "schemas/config/structured_config_policy_entitlement_certification.schema.json"
                .to_owned(),
    }
}

/// Parses a certification packet from JSON text.
pub fn parse_structured_config_policy_entitlement_certification(
    json: &str,
) -> Result<StructuredConfigPolicyEntitlementCertificationPacket, serde_json::Error> {
    serde_json::from_str(json)
}

/// Audits the certification packet and returns every defect found.
pub fn audit_structured_config_policy_entitlement_certification(
    packet: &StructuredConfigPolicyEntitlementCertificationPacket,
) -> Vec<CertificationValidationError> {
    let mut defects = Vec::new();

    append_presence_defects(
        &mut defects,
        &packet.artifact_rows,
        ArtifactFamilyKind::ALL.as_slice(),
        |row| row.family,
        CertificationValidationError::MissingArtifactFamily,
        CertificationValidationError::DuplicateArtifactFamily,
    );
    append_presence_defects(
        &mut defects,
        &packet.profile_rows,
        DeploymentProfileKind::ALL.as_slice(),
        |row| row.profile,
        CertificationValidationError::MissingProfile,
        CertificationValidationError::DuplicateProfile,
    );
    append_presence_defects(
        &mut defects,
        &packet.surface_rows,
        PublicationSurface::ALL.as_slice(),
        |row| row.surface,
        CertificationValidationError::MissingSurface,
        CertificationValidationError::DuplicateSurface,
    );

    for row in &packet.artifact_rows {
        let row_id = format!("{:?}", row.family);
        if row.published_state == CertificationState::Certified {
            if row.evidence_age_days > MAX_CERTIFIED_EVIDENCE_AGE_DAYS {
                defects.push(CertificationValidationError::CertifiedArtifactEvidenceStale(
                    row.family,
                ));
            }
            if !row.narrowing_reasons.is_empty() {
                defects.push(CertificationValidationError::CertifiedRowHasNarrowingReasons(
                    "artifact",
                    row_id.clone(),
                ));
            }
            if row.admin_auditability != AdminAuditabilityState::ExplainableExportReady {
                defects.push(
                    CertificationValidationError::CertifiedRowMissingAdminAuditability(row.family),
                );
            }
        } else if row.narrowing_reasons.is_empty() {
            defects.push(CertificationValidationError::NarrowedRowMissingReason(
                "artifact",
                row_id.clone(),
            ));
        }

        if row.claim_ceiling == QualificationLabel::Preview
            && row.published_state == CertificationState::Certified
        {
            defects.push(CertificationValidationError::PreviewFamilyPublishedTooWide(
                row.family,
            ));
        }

        if row.lane_class == ArtifactLaneClass::StructuredConfig && !row.parameter_provenance_reviewable {
            defects.push(
                CertificationValidationError::StructuredFamilyMissingParameterProvenance(
                    row.family,
                ),
            );
        }

        if row.lane_class == ArtifactLaneClass::SignedBundleReview && !row.signed_path_reviewable {
            defects.push(CertificationValidationError::SignedBundleFamilyMissingPathReview(
                row.family,
            ));
        }
    }

    for row in &packet.profile_rows {
        let row_id = format!("{:?}", row.profile);
        if row.published_state == CertificationState::Certified {
            if row.evidence_age_days > MAX_CERTIFIED_EVIDENCE_AGE_DAYS {
                defects.push(CertificationValidationError::CertifiedProfileEvidenceStale(
                    row.profile,
                ));
            }
            if !row.narrowing_reasons.is_empty() {
                defects.push(CertificationValidationError::CertifiedRowHasNarrowingReasons(
                    "profile",
                    row_id.clone(),
                ));
            }
        } else if row.narrowing_reasons.is_empty() {
            defects.push(CertificationValidationError::NarrowedRowMissingReason(
                "profile",
                row_id.clone(),
            ));
        }

        let present: BTreeSet<_> = row.drills.iter().map(|drill| drill.drill_kind).collect();
        for drill_kind in ProfileDrillKind::ALL {
            if !present.contains(&drill_kind) {
                defects.push(CertificationValidationError::MissingProfileDrill {
                    profile: row.profile,
                    drill_kind,
                });
            }
        }
    }

    for row in &packet.surface_rows {
        if !(row.shows_published_state
            && row.shows_evidence_age
            && row.shows_local_safe_floor
            && row.shows_supported_profiles
            && row.shows_narrowing_reasons
            && row.packet_ref == STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_PATH)
        {
            defects.push(CertificationValidationError::SurfaceDoesNotFullyIngest(
                row.surface,
            ));
        }
    }

    let derived_summary = derive_summary(&packet.artifact_rows, &packet.profile_rows, &packet.surface_rows);
    compare_summary(
        &mut defects,
        "artifact_family_count",
        derived_summary.artifact_family_count,
        packet.summary.artifact_family_count,
    );
    compare_summary(
        &mut defects,
        "certified_artifact_family_count",
        derived_summary.certified_artifact_family_count,
        packet.summary.certified_artifact_family_count,
    );
    compare_summary(
        &mut defects,
        "narrowed_artifact_family_count",
        derived_summary.narrowed_artifact_family_count,
        packet.summary.narrowed_artifact_family_count,
    );
    compare_summary(
        &mut defects,
        "profile_count",
        derived_summary.profile_count,
        packet.summary.profile_count,
    );
    compare_summary(
        &mut defects,
        "certified_profile_count",
        derived_summary.certified_profile_count,
        packet.summary.certified_profile_count,
    );
    compare_summary(
        &mut defects,
        "publication_surface_count",
        derived_summary.publication_surface_count,
        packet.summary.publication_surface_count,
    );

    defects
}

/// Validates the packet and returns `Ok(())` when it is clean.
pub fn validate_structured_config_policy_entitlement_certification(
    packet: &StructuredConfigPolicyEntitlementCertificationPacket,
) -> Result<(), Vec<CertificationValidationError>> {
    let defects = audit_structured_config_policy_entitlement_certification(packet);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

fn build_artifact_rows(
    matrix: &StructuredConfigControlMatrix,
    modes: &StructuredConfigArtifactModesAndLayersPacket,
    review: &StructuredConfigParameterSourceRoundTripReviewPacket,
) -> Vec<ArtifactCertificationRow> {
    matrix
        .artifact_families
        .iter()
        .map(|row| build_artifact_row(row, modes, review))
        .collect()
}

fn build_artifact_row(
    row: &crate::structured_config_policy_bundle_and_entitlement_matrix::ArtifactFamilyRow,
    modes: &StructuredConfigArtifactModesAndLayersPacket,
    review: &StructuredConfigParameterSourceRoundTripReviewPacket,
) -> ArtifactCertificationRow {
    let mode_row = modes
        .artifact_surfaces
        .iter()
        .find(|surface| surface.family == row.family);
    let review_row = review
        .artifact_reviews
        .iter()
        .find(|artifact_review| artifact_review.family == row.family);

    let lane_class = if matches!(
        row.family,
        ArtifactFamilyKind::AdminPolicyBundleArtifact
            | ArtifactFamilyKind::OfflineEntitlementSnapshotArtifact
            | ArtifactFamilyKind::EmergencyDisableBundleArtifact
            | ArtifactFamilyKind::TrustRootSignerUpdateArtifact
    ) {
        ArtifactLaneClass::SignedBundleReview
    } else {
        ArtifactLaneClass::StructuredConfig
    };

    let published_state = match row.qualification_label {
        QualificationLabel::Stable => CertificationState::Certified,
        QualificationLabel::Beta => CertificationState::Limited,
        QualificationLabel::Preview => CertificationState::RetestPending,
    };

    let mut narrowing_reasons = Vec::new();
    if row.qualification_label == QualificationLabel::Beta {
        narrowing_reasons.push(NarrowingReason::DepthIncomplete);
    }
    if row.qualification_label == QualificationLabel::Preview {
        narrowing_reasons.push(NarrowingReason::PreviewDependency);
    }

    ArtifactCertificationRow {
        family: row.family,
        lane_class,
        family_ref: row.family_ref.clone(),
        summary: row.summary.clone(),
        claim_ceiling: row.qualification_label,
        published_state,
        truth_planes_reviewable: row.authored_source && row.effective_projection,
        mode_layer_reviewable: mode_row.is_some(),
        parameter_provenance_reviewable: review_row.is_some() || lane_class == ArtifactLaneClass::SignedBundleReview,
        secret_reference_class_visible: !matches!(
            row.secret_handling,
            crate::structured_config_policy_bundle_and_entitlement_matrix::SecretHandlingClass::NoSecretsExpected
        ) || lane_class == ArtifactLaneClass::SignedBundleReview,
        policy_lock_visible: !matches!(
            row.policy_lock_posture,
            crate::structured_config_policy_bundle_and_entitlement_matrix::PolicyLockPosture::NotPolicyOwned
        ),
        signed_path_reviewable: !row.distribution_paths.is_empty(),
        admin_auditability: match published_state {
            CertificationState::Certified => AdminAuditabilityState::ExplainableExportReady,
            CertificationState::Limited => AdminAuditabilityState::ExplainableLocalSafeOnly,
            CertificationState::OfflineOnly => AdminAuditabilityState::ExplainableLocalSafeOnly,
            CertificationState::RetestPending => AdminAuditabilityState::Incomplete,
        },
        supported_profiles: supported_profiles_for_family(row.family),
        evidence_as_of: evidence_as_of_for_family(row.family).to_owned(),
        evidence_age_days: evidence_age_for_family(row.family),
        upstream_packet_refs: upstream_refs_for_family(row.family, mode_row, review_row),
        visible_labels: row.downgrade_labels.clone(),
        narrowing_reasons,
    }
}

fn build_profile_rows(matrix: &StructuredConfigControlMatrix) -> Vec<ProfileCertificationRow> {
    matrix
        .profile_qualifications
        .iter()
        .map(|row| ProfileCertificationRow {
            profile: row.profile,
            claim_ceiling: row.qualification_label,
            published_state: CertificationState::Certified,
            required_bundle_classes: row.required_bundle_classes.clone(),
            distribution_paths: row.distribution_paths.clone(),
            managed_auth_dependency: row.managed_auth_dependency,
            local_safe_label: row.local_safe_label,
            known_limits: row.known_limits.clone(),
            drills: seeded_profile_drills(row.profile),
            evidence_as_of: evidence_as_of_for_profile(row.profile).to_owned(),
            evidence_age_days: evidence_age_for_profile(row.profile),
            narrowing_reasons: Vec::new(),
        })
        .collect()
}

fn apply_scenario(
    scenario: CertificationScenario,
    artifact_rows: &mut [ArtifactCertificationRow],
    profile_rows: &mut [ProfileCertificationRow],
) {
    match scenario {
        CertificationScenario::Canonical => {}
        CertificationScenario::StalePolicy => {
            for row in artifact_rows.iter_mut().filter(|row| {
                matches!(
                    row.family,
                    ArtifactFamilyKind::ManagedPolicyOverlay
                        | ArtifactFamilyKind::AdminPolicyBundleArtifact
                        | ArtifactFamilyKind::OfflineEntitlementSnapshotArtifact
                )
            }) {
                row.published_state = CertificationState::Limited;
                row.evidence_age_days = 10;
                row.narrowing_reasons = vec![NarrowingReason::StalePolicyEvidence];
                row.admin_auditability = AdminAuditabilityState::ExplainableLocalSafeOnly;
                if !row.visible_labels.contains(&DowngradeLabelClass::LastKnownGood) {
                    row.visible_labels.push(DowngradeLabelClass::LastKnownGood);
                }
            }
            for row in profile_rows.iter_mut().filter(|row| {
                matches!(
                    row.profile,
                    DeploymentProfileKind::Managed | DeploymentProfileKind::SelfHosted
                )
            }) {
                row.published_state = CertificationState::OfflineOnly;
                row.evidence_age_days = 10;
                row.narrowing_reasons = vec![
                    NarrowingReason::StalePolicyEvidence,
                    NarrowingReason::MirrorOfflineFallback,
                ];
                if let Some(drill) = row
                    .drills
                    .iter_mut()
                    .find(|drill| drill.drill_kind == ProfileDrillKind::StalePolicy)
                {
                    drill.status = DrillStatus::PassedNarrowed;
                }
            }
        }
        CertificationScenario::ReauthRequired => {
            for row in artifact_rows.iter_mut().filter(|row| {
                matches!(
                    row.family,
                    ArtifactFamilyKind::DatabaseProfile
                        | ArtifactFamilyKind::ApiProfile
                        | ArtifactFamilyKind::ManagedPolicyOverlay
                )
            }) {
                row.published_state = CertificationState::Limited;
                row.narrowing_reasons = vec![NarrowingReason::ReauthRequired];
                row.admin_auditability = AdminAuditabilityState::ExplainableLocalSafeOnly;
            }
            for row in profile_rows.iter_mut().filter(|row| row.profile == DeploymentProfileKind::Managed) {
                row.published_state = CertificationState::OfflineOnly;
                row.narrowing_reasons = vec![NarrowingReason::ReauthRequired];
                if let Some(drill) = row
                    .drills
                    .iter_mut()
                    .find(|drill| drill.drill_kind == ProfileDrillKind::ReauthRequired)
                {
                    drill.status = DrillStatus::PassedNarrowed;
                }
            }
        }
        CertificationScenario::SignerRotation => {
            for row in artifact_rows.iter_mut().filter(|row| {
                matches!(
                    row.family,
                    ArtifactFamilyKind::TrustRootSignerUpdateArtifact
                        | ArtifactFamilyKind::AdminPolicyBundleArtifact
                        | ArtifactFamilyKind::EmergencyDisableBundleArtifact
                )
            }) {
                row.published_state = CertificationState::RetestPending;
                row.narrowing_reasons = vec![NarrowingReason::SignerRotationPending];
                row.admin_auditability = AdminAuditabilityState::Incomplete;
            }
            for row in profile_rows.iter_mut().filter(|row| {
                matches!(
                    row.profile,
                    DeploymentProfileKind::Managed
                        | DeploymentProfileKind::Mirrored
                        | DeploymentProfileKind::FullyAirGapped
                )
            }) {
                row.published_state = CertificationState::RetestPending;
                row.narrowing_reasons = vec![NarrowingReason::SignerRotationPending];
                if let Some(drill) = row
                    .drills
                    .iter_mut()
                    .find(|drill| drill.drill_kind == ProfileDrillKind::SignerRotation)
                {
                    drill.status = DrillStatus::RetestPending;
                }
            }
        }
    }
}

fn seeded_surface_rows() -> Vec<SurfaceIngestionRow> {
    PublicationSurface::ALL
        .iter()
        .copied()
        .map(|surface| SurfaceIngestionRow {
            surface,
            packet_ref: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_PATH.to_owned(),
            shows_published_state: true,
            shows_evidence_age: true,
            shows_local_safe_floor: true,
            shows_supported_profiles: true,
            shows_narrowing_reasons: true,
        })
        .collect()
}

fn seeded_downgrade_rules() -> Vec<DowngradeRule> {
    vec![
        DowngradeRule {
            trigger: NarrowingReason::PreviewDependency,
            narrow_to: CertificationState::RetestPending,
            rationale:
                "Preview-scoped configuration rows must never render as stable-looking defaults."
                    .to_owned(),
        },
        DowngradeRule {
            trigger: NarrowingReason::DepthIncomplete,
            narrow_to: CertificationState::Limited,
            rationale: "Beta-depth rows stay claimable only with explicit narrower language."
                .to_owned(),
        },
        DowngradeRule {
            trigger: NarrowingReason::StalePolicyEvidence,
            narrow_to: CertificationState::OfflineOnly,
            rationale:
                "Stale policy or entitlement evidence keeps the local-safe floor visible while managed widening pauses."
                    .to_owned(),
        },
        DowngradeRule {
            trigger: NarrowingReason::ReauthRequired,
            narrow_to: CertificationState::OfflineOnly,
            rationale:
                "Managed reauth pauses privileged actions without stranding local work."
                    .to_owned(),
        },
        DowngradeRule {
            trigger: NarrowingReason::SignerRotationPending,
            narrow_to: CertificationState::RetestPending,
            rationale:
                "Signer continuity must be reviewed before trust-bearing rows widen again."
                    .to_owned(),
        },
        DowngradeRule {
            trigger: NarrowingReason::MirrorOfflineFallback,
            narrow_to: CertificationState::OfflineOnly,
            rationale:
                "Mirror/offline fallback remains claimable only with explicit snapshot or last-known-good labeling."
                    .to_owned(),
        },
        DowngradeRule {
            trigger: NarrowingReason::ReferenceWorkspaceDrillMissing,
            narrow_to: CertificationState::RetestPending,
            rationale:
                "Reference-workspace certification must be rerun before a profile can widen again."
                    .to_owned(),
        },
        DowngradeRule {
            trigger: NarrowingReason::AdminAuditabilityIncomplete,
            narrow_to: CertificationState::Limited,
            rationale:
                "Rows that cannot export a safe admin explanation may not publish a fully certified claim."
                    .to_owned(),
        },
    ]
}

fn derive_summary(
    artifact_rows: &[ArtifactCertificationRow],
    profile_rows: &[ProfileCertificationRow],
    surface_rows: &[SurfaceIngestionRow],
) -> CertificationSummary {
    CertificationSummary {
        artifact_family_count: artifact_rows.len(),
        certified_artifact_family_count: artifact_rows
            .iter()
            .filter(|row| row.published_state == CertificationState::Certified)
            .count(),
        narrowed_artifact_family_count: artifact_rows
            .iter()
            .filter(|row| row.published_state != CertificationState::Certified)
            .count(),
        profile_count: profile_rows.len(),
        certified_profile_count: profile_rows
            .iter()
            .filter(|row| row.published_state == CertificationState::Certified)
            .count(),
        publication_surface_count: surface_rows.len(),
        local_safe_floor_visible_everywhere: profile_rows
            .iter()
            .all(|row| !matches!(row.local_safe_label, LocalSafeLabelClass::ContinueLocalOnly) || row.profile == DeploymentProfileKind::LocalOnly || row.published_state.is_claimable()),
        publication_surfaces_aligned: surface_rows.iter().all(|row| {
            row.packet_ref == STRUCTURED_CONFIG_POLICY_ENTITLEMENT_CERTIFICATION_PATH
                && row.shows_published_state
                && row.shows_evidence_age
                && row.shows_local_safe_floor
                && row.shows_supported_profiles
                && row.shows_narrowing_reasons
        }),
    }
}

fn supported_profiles_for_family(family: ArtifactFamilyKind) -> Vec<DeploymentProfileKind> {
    match family {
        ArtifactFamilyKind::ManagedPolicyOverlay
        | ArtifactFamilyKind::AdminPolicyBundleArtifact
        | ArtifactFamilyKind::OfflineEntitlementSnapshotArtifact
        | ArtifactFamilyKind::EmergencyDisableBundleArtifact
        | ArtifactFamilyKind::TrustRootSignerUpdateArtifact => vec![
            DeploymentProfileKind::Managed,
            DeploymentProfileKind::SelfHosted,
            DeploymentProfileKind::Mirrored,
            DeploymentProfileKind::FullyAirGapped,
        ],
        ArtifactFamilyKind::PreviewRuntimeConfig => vec![
            DeploymentProfileKind::LocalOnly,
            DeploymentProfileKind::Managed,
            DeploymentProfileKind::SelfHosted,
            DeploymentProfileKind::Mirrored,
        ],
        _ => DeploymentProfileKind::ALL.to_vec(),
    }
}

fn evidence_as_of_for_family(family: ArtifactFamilyKind) -> &'static str {
    match family {
        ArtifactFamilyKind::RequestWorkspaceEnvironment => "2026-06-11T16:10:00Z",
        ArtifactFamilyKind::DatabaseProfile => "2026-06-12T09:40:00Z",
        ArtifactFamilyKind::ApiProfile => "2026-06-12T09:42:00Z",
        ArtifactFamilyKind::NotebookRuntimeManifest => "2026-06-10T20:35:00Z",
        ArtifactFamilyKind::PreviewRuntimeConfig => "2026-06-09T18:05:00Z",
        ArtifactFamilyKind::WorkflowBundleManifest => "2026-06-10T14:55:00Z",
        ArtifactFamilyKind::CiEnvironmentDescriptor => "2026-06-10T08:15:00Z",
        ArtifactFamilyKind::InfraEnvironmentDescriptor => "2026-06-10T08:20:00Z",
        ArtifactFamilyKind::ManagedPolicyOverlay => "2026-06-12T11:25:00Z",
        ArtifactFamilyKind::AdminPolicyBundleArtifact => "2026-06-12T11:27:00Z",
        ArtifactFamilyKind::OfflineEntitlementSnapshotArtifact => "2026-06-12T11:29:00Z",
        ArtifactFamilyKind::EmergencyDisableBundleArtifact => "2026-06-12T11:31:00Z",
        ArtifactFamilyKind::TrustRootSignerUpdateArtifact => "2026-06-12T11:33:00Z",
    }
}

fn evidence_age_for_family(family: ArtifactFamilyKind) -> u32 {
    match family {
        ArtifactFamilyKind::PreviewRuntimeConfig => 3,
        ArtifactFamilyKind::RequestWorkspaceEnvironment
        | ArtifactFamilyKind::NotebookRuntimeManifest
        | ArtifactFamilyKind::WorkflowBundleManifest
        | ArtifactFamilyKind::CiEnvironmentDescriptor
        | ArtifactFamilyKind::InfraEnvironmentDescriptor => 2,
        _ => 0,
    }
}

fn upstream_refs_for_family(
    family: ArtifactFamilyKind,
    mode_row: Option<&ArtifactSurfaceRow>,
    review_row: Option<&ArtifactReviewRow>,
) -> Vec<String> {
    let mut refs = vec![
        "artifacts/config/structured_config_policy_bundle_and_entitlement_matrix.json".to_owned(),
    ];
    if let Some(mode_row) = mode_row {
        refs.push(mode_row.source_packet_ref.clone());
    } else if matches!(
        family,
        ArtifactFamilyKind::AdminPolicyBundleArtifact
            | ArtifactFamilyKind::OfflineEntitlementSnapshotArtifact
            | ArtifactFamilyKind::EmergencyDisableBundleArtifact
            | ArtifactFamilyKind::TrustRootSignerUpdateArtifact
    ) {
        refs.push("artifacts/config/structured_config_artifact_modes_and_layers.json".to_owned());
    }
    if let Some(review_row) = review_row {
        refs.push(review_row.artifact_surface_ref.clone());
    }
    refs
}

fn seeded_profile_drills(profile: DeploymentProfileKind) -> Vec<ProfileDrillRow> {
    ProfileDrillKind::ALL
        .iter()
        .copied()
        .map(|drill_kind| ProfileDrillRow {
            drill_kind,
            status: default_drill_status(profile, drill_kind),
            executed_at: executed_at_for_drill(profile, drill_kind).to_owned(),
            evidence_ref: evidence_ref_for_drill(profile, drill_kind).to_owned(),
            summary: summary_for_drill(profile, drill_kind).to_owned(),
        })
        .collect()
}

fn default_drill_status(profile: DeploymentProfileKind, drill_kind: ProfileDrillKind) -> DrillStatus {
    match (profile, drill_kind) {
        (DeploymentProfileKind::LocalOnly, ProfileDrillKind::ManagedSelfHosted)
        | (DeploymentProfileKind::LocalOnly, ProfileDrillKind::ReauthRequired)
        | (DeploymentProfileKind::LocalOnly, ProfileDrillKind::SignerRotation) => DrillStatus::NotApplicable,
        (DeploymentProfileKind::FullyAirGapped, ProfileDrillKind::ManagedSelfHosted)
        | (DeploymentProfileKind::FullyAirGapped, ProfileDrillKind::ReauthRequired) => DrillStatus::NotApplicable,
        _ => DrillStatus::PassedCurrent,
    }
}

fn executed_at_for_drill(profile: DeploymentProfileKind, drill_kind: ProfileDrillKind) -> &'static str {
    match (profile, drill_kind) {
        (DeploymentProfileKind::LocalOnly, ProfileDrillKind::ReferenceWorkspace) => "2026-06-12T08:00:00Z",
        (DeploymentProfileKind::Managed, ProfileDrillKind::StalePolicy) => "2026-06-12T09:05:00Z",
        (DeploymentProfileKind::Managed, ProfileDrillKind::ReauthRequired) => "2026-06-12T09:15:00Z",
        (DeploymentProfileKind::Mirrored, ProfileDrillKind::MirrorOffline) => "2026-06-12T07:40:00Z",
        (DeploymentProfileKind::FullyAirGapped, ProfileDrillKind::MirrorOffline) => "2026-06-11T19:10:00Z",
        _ => "2026-06-12T10:00:00Z",
    }
}

fn evidence_ref_for_drill(profile: DeploymentProfileKind, drill_kind: ProfileDrillKind) -> &'static str {
    match (profile, drill_kind) {
        (DeploymentProfileKind::LocalOnly, ProfileDrillKind::ReferenceWorkspace) => {
            "evidence:config-profile:local-only:reference-workspace"
        }
        (DeploymentProfileKind::Managed, ProfileDrillKind::StalePolicy) => {
            "evidence:config-profile:managed:stale-policy"
        }
        (DeploymentProfileKind::Managed, ProfileDrillKind::ReauthRequired) => {
            "evidence:config-profile:managed:reauth-required"
        }
        (DeploymentProfileKind::Mirrored, ProfileDrillKind::MirrorOffline) => {
            "evidence:config-profile:mirrored:mirror-offline"
        }
        (DeploymentProfileKind::FullyAirGapped, ProfileDrillKind::MirrorOffline) => {
            "evidence:config-profile:air-gap:offline-import"
        }
        _ => "evidence:config-profile:shared:current",
    }
}

fn summary_for_drill(profile: DeploymentProfileKind, drill_kind: ProfileDrillKind) -> &'static str {
    match (profile, drill_kind) {
        (_, ProfileDrillKind::ReferenceWorkspace) => {
            "Reference-workspace packet still distinguishes source, effective, and live truth before save, preview, apply, and export."
        }
        (_, ProfileDrillKind::MirrorOffline) => {
            "Mirror/offline drill keeps signed path, snapshot age, and local-safe continuation visible without implying live freshness."
        }
        (_, ProfileDrillKind::ManagedSelfHosted) => {
            "Managed/self-hosted continuity drill keeps policy source, entitlement source, and support-safe explainability aligned."
        }
        (_, ProfileDrillKind::StalePolicy) => {
            "Stale-policy drill proves last-known-good continuity narrows managed actions without hiding the local-safe floor."
        }
        (_, ProfileDrillKind::ReauthRequired) => {
            "Reauth-required drill proves managed actions pause while local editing, diagnostics, and export stay available."
        }
        (_, ProfileDrillKind::SignerRotation) => {
            "Signer-rotation drill proves successor-root review is explicit and portable across mirror and offline paths."
        }
    }
}

fn evidence_as_of_for_profile(profile: DeploymentProfileKind) -> &'static str {
    match profile {
        DeploymentProfileKind::LocalOnly => "2026-06-12T08:00:00Z",
        DeploymentProfileKind::Managed => "2026-06-12T09:15:00Z",
        DeploymentProfileKind::SelfHosted => "2026-06-12T09:20:00Z",
        DeploymentProfileKind::Mirrored => "2026-06-12T07:40:00Z",
        DeploymentProfileKind::FullyAirGapped => "2026-06-11T19:10:00Z",
    }
}

fn evidence_age_for_profile(profile: DeploymentProfileKind) -> u32 {
    match profile {
        DeploymentProfileKind::FullyAirGapped => 1,
        _ => 0,
    }
}

fn compare_summary(
    defects: &mut Vec<CertificationValidationError>,
    field: &'static str,
    expected: usize,
    actual: usize,
) {
    if expected != actual {
        defects.push(CertificationValidationError::SummaryCountMismatch {
            field,
            expected,
            actual,
        });
    }
}

fn append_presence_defects<T, K: Copy + Ord>(
    defects: &mut Vec<CertificationValidationError>,
    rows: &[T],
    expected: &[K],
    mut key_of: impl FnMut(&T) -> K,
    missing: impl Fn(K) -> CertificationValidationError,
    duplicate: impl Fn(K) -> CertificationValidationError,
) {
    let mut seen = BTreeSet::new();
    for row in rows {
        let key = key_of(row);
        if !seen.insert(key) {
            defects.push(duplicate(key));
        }
    }
    for &key in expected {
        if !seen.contains(&key) {
            defects.push(missing(key));
        }
    }
}
