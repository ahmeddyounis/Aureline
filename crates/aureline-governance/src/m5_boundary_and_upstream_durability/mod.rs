//! Typed open/local-boundary, repository-compliance, third-party-import, and
//! maintainer/signer-durability matrix.
//!
//! Earlier governance registers each answer one slice of the question this
//! matrix answers as a whole. The
//! [`schema_registry`](crate::schema_registry) governs payload families; the
//! [`interface_freeze`](crate::interface_freeze) register freezes contract
//! surfaces; the release-side open-versus-paid boundary audit attests one
//! launch line. None of them freeze, in one inspectable place, the standing
//! durability facts every claimed ecosystem and release lane rests on: **where
//! the open/local core ends and the paid/managed tier begins, which
//! repository-compliance and third-party-import controls each lane must satisfy,
//! who holds the emergency signing/registry/security authority for it (and who
//! backs them up), and whether its critical upstreams are owned — and does the
//! lane narrow the moment any of that thins out?**
//!
//! This module is that matrix. For every claimed asset lane it records one
//! [`BoundaryDurabilityRow`] that binds the lane to:
//!
//! - its open/local [`BoundaryPosture`] and [`SupportClass`], with a
//!   `must_remain_open` flag marking the lanes whose ordinary local usefulness
//!   may never be blurred by commercial or managed value;
//! - the repository-compliance [`ControlBinding`] set it must satisfy
//!   (contribution provenance, file-level licensing, third-party imports,
//!   generated-code attribution, SBOM/notices, signer coverage, registry
//!   emergency action, security response, critical-upstream ownership);
//! - the [`EmergencyAuthority`] holding it — primary and backup owners, the
//!   signer quorum, and the registry-emergency and security-response owners —
//!   so no release/signing/registry/security lane depends on one irreplaceable
//!   human;
//! - the [`ContinuityCoverage`] for it — backup coverage, single-point-of-failure
//!   posture, and the owned critical upstreams;
//! - the proof packet ([`ProofPacket`]) that grounds the row, the optional
//!   [`Waiver`] holding a gap provisionally, and the [`OwnerSignoff`].
//!
//! A row's [`DurabilityState`] is **durable** only when every control is
//! satisfied, the authority and continuity are covered, the proof is fresh, the
//! owner signed, and (for a `must_remain_open` lane) the posture is an open
//! baseline posture. Otherwise the row is narrowed — and it narrows on the
//! *specific* axis that thinned out (boundary drift, a compliance gap, an
//! authority gap, a continuity gap, or stale proof), never collapsing to one
//! global flag. A narrowed lane drops its [`BoundaryDurabilityRow::effective_label`]
//! below the launch cutline and may never publish an effective label wider than
//! the one it declares.
//!
//! The [`GovernanceRule`] set names the closed conditions that gate publication.
//! An *inherited* narrowing — a lane whose declared label already sits below the
//! cutline, or a gap held by an unexpired waiver — is gated upstream and does
//! not itself hold promotion; a *durability-layer* failure on a lane whose
//! declared label is still at or above the cutline holds promotion through a
//! stop rule, recorded in [`BoundaryDurabilityMatrix::publication`].
//!
//! The matrix is checked in at
//! `artifacts/governance/m5-boundary-and-upstream-durability.json` and embedded
//! here, so this typed consumer and the CI gate agree on every row without a
//! cargo build in CI. The model is metadata-only: every field is a typed state,
//! a boolean flag, a small count, or an opaque ref. It carries no credential
//! bodies, raw provider payloads, signatures, or attestation material. Two
//! checks live outside this model because they need more than the matrix sees —
//! date arithmetic (recomputing proof freshness and waiver expiry against an
//! `as_of` date) and cross-artifact joins against the source registers — and
//! live in the CI gate and the integration test. This model enforces the
//! invariants that hold regardless of the clock: the open-baseline guardrail,
//! the no-widening ceiling, control/authority/continuity completeness, narrowing
//! consistency, reason/state coherence, summary agreement, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported matrix schema version.
pub const M5_BOUNDARY_AND_UPSTREAM_DURABILITY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the matrix.
pub const M5_BOUNDARY_AND_UPSTREAM_DURABILITY_RECORD_KIND: &str =
    "m5_boundary_and_upstream_durability_matrix";

/// Repo-relative path to the checked-in matrix.
pub const M5_BOUNDARY_AND_UPSTREAM_DURABILITY_PATH: &str =
    "artifacts/governance/m5-boundary-and-upstream-durability.json";

/// Embedded checked-in matrix JSON.
pub const M5_BOUNDARY_AND_UPSTREAM_DURABILITY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/m5-boundary-and-upstream-durability.json"
));

/// Lifecycle/support label a lane backs.
///
/// This reuses the train-wide lifecycle vocabulary rather than minting a
/// boundary-local synonym set: every consuming surface ingests one label per
/// row. `lts` and `stable` sit at or above the launch cutline; `beta`,
/// `preview`, and `withdrawn` sit below it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleLabel {
    /// Long-term-support line.
    Lts,
    /// Stable line.
    Stable,
    /// Beta line.
    Beta,
    /// Preview line.
    Preview,
    /// Withdrawn line.
    Withdrawn,
}

impl LifecycleLabel {
    /// Every label, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Lts,
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lts => "lts",
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Support rank: higher means more strongly supported.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Lts => 4,
            Self::Stable => 3,
            Self::Beta => 2,
            Self::Preview => 1,
            Self::Withdrawn => 0,
        }
    }

    /// The label for a support `rank`.
    const fn from_rank(rank: u8) -> Self {
        match rank {
            4 => Self::Lts,
            3 => Self::Stable,
            2 => Self::Beta,
            1 => Self::Preview,
            _ => Self::Withdrawn,
        }
    }

    /// True when the label is at or above the launch cutline (`lts`/`stable`).
    pub const fn is_at_or_above_cutline(self) -> bool {
        self.rank() >= Self::Stable.rank()
    }
}

/// The asset lane a row governs (the asset-lane matrix axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssetLane {
    /// The core desktop/client/platform shell that must run locally.
    CoreDesktopClientPlatform,
    /// SDKs, schemas, and exported contracts.
    SdkSchemaContract,
    /// Documentation and migration packs.
    DocsMigrationPack,
    /// Marketplace and extension-registry protocols.
    MarketplaceProtocol,
    /// Managed/hosted services.
    ManagedService,
    /// Restricted brand and trademark assets.
    RestrictedBrandAsset,
}

impl AssetLane {
    /// Every asset lane, in declaration order. Every lane must be covered by at
    /// least one row.
    pub const ALL: [Self; 6] = [
        Self::CoreDesktopClientPlatform,
        Self::SdkSchemaContract,
        Self::DocsMigrationPack,
        Self::MarketplaceProtocol,
        Self::ManagedService,
        Self::RestrictedBrandAsset,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreDesktopClientPlatform => "core_desktop_client_platform",
            Self::SdkSchemaContract => "sdk_schema_contract",
            Self::DocsMigrationPack => "docs_migration_pack",
            Self::MarketplaceProtocol => "marketplace_protocol",
            Self::ManagedService => "managed_service",
            Self::RestrictedBrandAsset => "restricted_brand_asset",
        }
    }
}

/// Where the open/local core ends and the paid/managed tier begins for a lane.
///
/// Ordered most-open first. The first two postures are the *open baseline*: a
/// `must_remain_open` lane may only carry one of them.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryPosture {
    /// Fully open and locally useful with no managed dependency.
    OpenLocalCore,
    /// Open and locally useful core; managed value is an optional add-on.
    OpenLocalWithManagedOptional,
    /// Source-available with restricted redistribution.
    SourceAvailableRestricted,
    /// A paid/managed service, not part of the local core.
    ManagedService,
    /// Restricted brand/trademark assets, not open.
    RestrictedBrand,
}

impl BoundaryPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenLocalCore,
        Self::OpenLocalWithManagedOptional,
        Self::SourceAvailableRestricted,
        Self::ManagedService,
        Self::RestrictedBrand,
    ];

    /// The open-baseline postures a `must_remain_open` lane may carry.
    pub const OPEN_BASELINE: [Self; 2] = [Self::OpenLocalCore, Self::OpenLocalWithManagedOptional];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocalCore => "open_local_core",
            Self::OpenLocalWithManagedOptional => "open_local_with_managed_optional",
            Self::SourceAvailableRestricted => "source_available_restricted",
            Self::ManagedService => "managed_service",
            Self::RestrictedBrand => "restricted_brand",
        }
    }

    /// True when the posture is an open-baseline posture.
    pub fn is_open_baseline(self) -> bool {
        Self::OPEN_BASELINE.contains(&self)
    }

    /// The support class consistent with this posture.
    const fn expected_support_class(self) -> SupportClass {
        match self {
            Self::OpenLocalCore => SupportClass::OpenLocal,
            Self::OpenLocalWithManagedOptional => SupportClass::MixedOpenManaged,
            Self::SourceAvailableRestricted => SupportClass::Restricted,
            Self::ManagedService => SupportClass::Managed,
            Self::RestrictedBrand => SupportClass::Restricted,
        }
    }
}

/// The support class a lane publishes (open/local, mixed, managed, restricted).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    /// Open and locally supported.
    OpenLocal,
    /// Open/local core with managed value as an optional add-on.
    MixedOpenManaged,
    /// Managed/hosted support only.
    Managed,
    /// Restricted (brand/source-available) support.
    Restricted,
}

impl SupportClass {
    /// Every support class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::OpenLocal,
        Self::MixedOpenManaged,
        Self::Managed,
        Self::Restricted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocal => "open_local",
            Self::MixedOpenManaged => "mixed_open_managed",
            Self::Managed => "managed",
            Self::Restricted => "restricted",
        }
    }
}

/// A repository-compliance / review control dimension (the review/control matrix
/// axis).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlDimension {
    /// Contribution provenance: DCO/CLA sign-off and contributor terms.
    ContributionProvenance,
    /// File-level licensing: REUSE/SPDX headers and per-file license clarity.
    FileLevelLicensing,
    /// Third-party imports: inventory, attribution, and modification posture.
    ThirdPartyImport,
    /// Generated-code attribution and provenance.
    GeneratedCodeAttribution,
    /// SBOM and third-party notices.
    SbomAndNotices,
    /// Signer coverage: signing quorum and no single-signer release path.
    SignerCoverage,
    /// Registry emergency action: moderation and emergency unpublish.
    RegistryEmergencyAction,
    /// Security response: advisory, CVE/GHSA, and revocation operations.
    SecurityResponse,
    /// Critical-upstream ownership: owner and fork/replace plan.
    CriticalUpstreamOwnership,
}

impl ControlDimension {
    /// Every control dimension, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ContributionProvenance,
        Self::FileLevelLicensing,
        Self::ThirdPartyImport,
        Self::GeneratedCodeAttribution,
        Self::SbomAndNotices,
        Self::SignerCoverage,
        Self::RegistryEmergencyAction,
        Self::SecurityResponse,
        Self::CriticalUpstreamOwnership,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContributionProvenance => "contribution_provenance",
            Self::FileLevelLicensing => "file_level_licensing",
            Self::ThirdPartyImport => "third_party_import",
            Self::GeneratedCodeAttribution => "generated_code_attribution",
            Self::SbomAndNotices => "sbom_and_notices",
            Self::SignerCoverage => "signer_coverage",
            Self::RegistryEmergencyAction => "registry_emergency_action",
            Self::SecurityResponse => "security_response",
            Self::CriticalUpstreamOwnership => "critical_upstream_ownership",
        }
    }
}

/// Satisfaction state of one control binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlState {
    /// The control is satisfied for this lane.
    Satisfied,
    /// The control applies but is not satisfied.
    Unsatisfied,
    /// The control does not apply to this lane.
    NotApplicable,
}

impl ControlState {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unsatisfied => "unsatisfied",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Backup-coverage posture for a lane's authority and maintenance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupCoverage {
    /// A backup maintainer/owner is in place.
    Covered,
    /// Backup is missing but held by a recorded, time-boxed waiver.
    Waived,
    /// Backup is missing and unwaived.
    Uncovered,
}

impl BackupCoverage {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Covered => "covered",
            Self::Waived => "waived",
            Self::Uncovered => "uncovered",
        }
    }
}

/// Risk class for an owned critical upstream.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskClass {
    /// Low risk.
    Low,
    /// Medium risk.
    Medium,
    /// High risk.
    High,
    /// Blocked: failure would block the lane.
    Blocked,
}

impl RiskClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Blocked => "blocked",
        }
    }
}

/// Freshness state of a proof packet against its SLO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessSloState {
    /// Captured within the SLO window.
    Current,
    /// Within the warn band: due for refresh.
    DueForRefresh,
    /// Past the SLO window.
    Breached,
    /// No packet captured.
    Missing,
}

impl FreshnessSloState {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::DueForRefresh => "due_for_refresh",
            Self::Breached => "breached",
            Self::Missing => "missing",
        }
    }
}

/// Durability state a row earns after narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurabilityState {
    /// Boundary, compliance, authority, continuity, and proof all hold.
    Durable,
    /// A `must_remain_open` lane drifted off the open baseline.
    NarrowedBoundaryDrift,
    /// A required compliance control is unsatisfied.
    NarrowedComplianceGap,
    /// Emergency authority is incomplete (owner missing or quorum unmet).
    NarrowedAuthorityGap,
    /// Continuity is incomplete (single point of failure, backup, or upstream).
    NarrowedContinuityGap,
    /// The proof packet, sign-off, or waiver thinned out.
    NarrowedStale,
    /// The lane is withdrawn.
    Withdrawn,
}

impl DurabilityState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Durable,
        Self::NarrowedBoundaryDrift,
        Self::NarrowedComplianceGap,
        Self::NarrowedAuthorityGap,
        Self::NarrowedContinuityGap,
        Self::NarrowedStale,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Durable => "durable",
            Self::NarrowedBoundaryDrift => "narrowed_boundary_drift",
            Self::NarrowedComplianceGap => "narrowed_compliance_gap",
            Self::NarrowedAuthorityGap => "narrowed_authority_gap",
            Self::NarrowedContinuityGap => "narrowed_continuity_gap",
            Self::NarrowedStale => "narrowed_stale",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the state is one of the narrowed states (not durable, not
    /// withdrawn).
    pub fn is_narrowed(self) -> bool {
        !matches!(self, Self::Durable | Self::Withdrawn)
    }
}

/// A reason a row narrowed. Closed vocabulary; every reason is watched by a
/// [`GovernanceRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurabilityReason {
    /// A `must_remain_open` lane carries a non-open-baseline posture.
    BoundaryBaselineViolated,
    /// A required compliance control is unsatisfied.
    ComplianceControlUnsatisfied,
    /// The signer quorum is not met (fewer available signers than required).
    SignerQuorumUnmet,
    /// An emergency authority owner reference is missing.
    EmergencyAuthorityOwnerMissing,
    /// The lane has an unmitigated single point of failure.
    SinglePointOfFailure,
    /// Backup maintainer/owner coverage is missing and unwaived.
    BackupCoverageMissing,
    /// A critical upstream has no recorded owner.
    CriticalUpstreamUnowned,
    /// The proof packet aged past its freshness SLO.
    ProofFreshnessBreached,
    /// No proof packet is captured.
    ProofPacketMissing,
    /// The owner sign-off is missing.
    OwnerSignoffMissing,
    /// The waiver relied on has expired.
    WaiverExpired,
}

impl DurabilityReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::BoundaryBaselineViolated,
        Self::ComplianceControlUnsatisfied,
        Self::SignerQuorumUnmet,
        Self::EmergencyAuthorityOwnerMissing,
        Self::SinglePointOfFailure,
        Self::BackupCoverageMissing,
        Self::CriticalUpstreamUnowned,
        Self::ProofFreshnessBreached,
        Self::ProofPacketMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundaryBaselineViolated => "boundary_baseline_violated",
            Self::ComplianceControlUnsatisfied => "compliance_control_unsatisfied",
            Self::SignerQuorumUnmet => "signer_quorum_unmet",
            Self::EmergencyAuthorityOwnerMissing => "emergency_authority_owner_missing",
            Self::SinglePointOfFailure => "single_point_of_failure",
            Self::BackupCoverageMissing => "backup_coverage_missing",
            Self::CriticalUpstreamUnowned => "critical_upstream_unowned",
            Self::ProofFreshnessBreached => "proof_freshness_breached",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Precedence: lower is worse and wins when several reasons are active.
    const fn precedence(self) -> u8 {
        match self.state_group() {
            DurabilityState::NarrowedBoundaryDrift => 0,
            DurabilityState::NarrowedAuthorityGap => 1,
            DurabilityState::NarrowedContinuityGap => 2,
            DurabilityState::NarrowedComplianceGap => 3,
            _ => 4,
        }
    }

    /// The narrowing state this reason maps to.
    pub const fn state_group(self) -> DurabilityState {
        match self {
            Self::BoundaryBaselineViolated => DurabilityState::NarrowedBoundaryDrift,
            Self::ComplianceControlUnsatisfied => DurabilityState::NarrowedComplianceGap,
            Self::SignerQuorumUnmet | Self::EmergencyAuthorityOwnerMissing => {
                DurabilityState::NarrowedAuthorityGap
            }
            Self::SinglePointOfFailure
            | Self::BackupCoverageMissing
            | Self::CriticalUpstreamUnowned => DurabilityState::NarrowedContinuityGap,
            Self::ProofFreshnessBreached
            | Self::ProofPacketMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => DurabilityState::NarrowedStale,
        }
    }
}

/// An action a [`GovernanceRule`] recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatrixAction {
    /// Hold publication until the gap clears.
    HoldPublication,
    /// Restore the open/local baseline posture.
    RestoreOpenBaseline,
    /// Narrow the lane's boundary label.
    NarrowBoundaryLabel,
    /// Satisfy the unsatisfied compliance control.
    SatisfyComplianceControl,
    /// Assign or restore the emergency authority owner/quorum.
    AssignEmergencyAuthority,
    /// Add backup coverage / remove the single point of failure.
    AddBackupCoverage,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Request the owner sign-off.
    RequestOwnerSignoff,
}

impl MatrixAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldPublication,
        Self::RestoreOpenBaseline,
        Self::NarrowBoundaryLabel,
        Self::SatisfyComplianceControl,
        Self::AssignEmergencyAuthority,
        Self::AddBackupCoverage,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::RestoreOpenBaseline => "restore_open_baseline",
            Self::NarrowBoundaryLabel => "narrow_boundary_label",
            Self::SatisfyComplianceControl => "satisfy_compliance_control",
            Self::AssignEmergencyAuthority => "assign_emergency_authority",
            Self::AddBackupCoverage => "add_backup_coverage",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication decision recorded by the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// No durability-layer stop rule fires; publication may proceed.
    Proceed,
    /// A durability-layer stop rule fires; hold publication.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// Freshness SLO for a proof packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FreshnessSlo {
    /// Maximum packet age in days before the SLO is breached.
    pub target_max_age_days: u32,
    /// Warn band: days before breach the packet is due for refresh.
    pub warn_within_days: u32,
    /// Reference to the SLO register that defines this window.
    pub slo_register_ref: String,
}

/// A captured proof packet grounding a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProofPacket {
    /// Stable packet id.
    pub packet_id: String,
    /// Reference to the packet artifact.
    pub packet_ref: String,
    /// Capture date (`null` when no packet is captured).
    pub captured_at: Option<String>,
    /// Freshness SLO for this packet.
    pub freshness_slo: FreshnessSlo,
    /// Freshness state against the SLO.
    pub slo_state: FreshnessSloState,
    /// Evidence references backing the packet.
    pub evidence_refs: Vec<String>,
}

/// One repository-compliance / review control binding on a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlBinding {
    /// The control dimension.
    pub dimension: ControlDimension,
    /// Reference to the source register/scan that governs the control.
    pub control_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Satisfaction state.
    pub state: ControlState,
}

/// The signer quorum for a lane's protected actions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignerQuorum {
    /// Distinct humans required to authorize a protected action.
    pub required_distinct_humans: u32,
    /// Distinct humans actually available.
    pub available_distinct_humans: u32,
    /// Reference to the quorum profile.
    pub quorum_profile_ref: String,
}

impl SignerQuorum {
    /// True when the available signers meet the requirement.
    pub fn is_met(&self) -> bool {
        self.available_distinct_humans >= self.required_distinct_humans
    }
}

/// The emergency signing/registry/security authority holding a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmergencyAuthority {
    /// Primary owning team or role.
    pub primary_owner_ref: String,
    /// Backup owners who can act if the primary is unavailable.
    pub backup_owner_refs: Vec<String>,
    /// The signer quorum.
    pub signer_quorum: SignerQuorum,
    /// Owner of registry emergency action (moderation/unpublish).
    pub registry_emergency_owner_ref: String,
    /// Owner of security response (advisory/revocation).
    pub security_response_owner_ref: String,
}

impl EmergencyAuthority {
    /// True when every owner reference is present.
    pub fn owners_present(&self) -> bool {
        !self.primary_owner_ref.trim().is_empty()
            && !self.registry_emergency_owner_ref.trim().is_empty()
            && !self.security_response_owner_ref.trim().is_empty()
            && self
                .backup_owner_refs
                .iter()
                .all(|owner| !owner.trim().is_empty())
    }
}

/// An owned critical upstream for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CriticalUpstream {
    /// Reference to the upstream dependency/import.
    pub upstream_ref: String,
    /// Owning team or role (empty means unowned).
    pub owner_ref: String,
    /// Risk class.
    pub risk_class: RiskClass,
    /// Reference to the fork/replace contingency plan.
    pub fork_replace_plan_ref: String,
}

/// Continuity coverage for a lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityCoverage {
    /// Backup-coverage posture.
    pub backup_coverage: BackupCoverage,
    /// True when the lane has an unmitigated single point of failure.
    pub single_point_of_failure: bool,
    /// Owned critical upstreams.
    pub critical_upstreams: Vec<CriticalUpstream>,
}

/// A waiver holding a gap provisionally.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Waiver {
    /// Reference to the waiver record.
    pub waiver_ref: String,
    /// Expiry date.
    pub expires_at: String,
    /// Reviewable reason.
    pub reason: String,
}

/// Owner sign-off on a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerSignoff {
    /// Owning team or role.
    pub owner_ref: String,
    /// True when the owner has signed off.
    pub signed_off: bool,
    /// Sign-off date (`null` when unsigned).
    pub signed_at: Option<String>,
}

/// One asset-lane durability row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryDurabilityRow {
    /// Stable row id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The asset lane this row governs.
    pub asset_lane: AssetLane,
    /// Reference to the governed subject.
    pub subject_ref: String,
    /// One-line subject summary.
    pub subject_summary: String,
    /// True when this lane is part of the release-blocking set.
    pub release_blocking: bool,
    /// True when this lane's ordinary local usefulness must remain open.
    pub must_remain_open: bool,
    /// Open/local boundary posture.
    pub boundary_posture: BoundaryPosture,
    /// Support class published.
    pub support_class: SupportClass,
    /// The lifecycle/support label this lane declares.
    pub declared_label: LifecycleLabel,
    /// Repository-compliance / review control bindings.
    pub compliance_controls: Vec<ControlBinding>,
    /// Emergency signing/registry/security authority.
    pub emergency_authority: EmergencyAuthority,
    /// Continuity coverage.
    pub continuity: ContinuityCoverage,
    /// Proof packet grounding the row.
    pub proof_packet: ProofPacket,
    /// Optional waiver holding a gap provisionally.
    pub waiver: Option<Waiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Durability state earned after narrowing.
    pub durability_state: DurabilityState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<DurabilityReason>,
    /// The label the lane effectively publishes after narrowing.
    pub effective_label: LifecycleLabel,
    /// Surfaces that reuse this row (release packets, boundary manifests,
    /// repo-compliance scans, shiproom gates).
    pub reuse_destinations: Vec<String>,
    /// Reviewable reason the row carries its state.
    pub rationale: String,
}

impl BoundaryDurabilityRow {
    /// True when the row is held by an unexpired waiver (one that does not carry
    /// a `waiver_expired` reason).
    pub fn is_waived(&self) -> bool {
        self.waiver.is_some() && !self.has_active_reason(DurabilityReason::WaiverExpired)
    }

    /// True when the row carries the given active reason.
    pub fn has_active_reason(&self, reason: DurabilityReason) -> bool {
        self.active_reasons.contains(&reason)
    }

    /// True when the row holds a durable state.
    pub fn is_durable(&self) -> bool {
        self.durability_state == DurabilityState::Durable
    }

    /// True when the lane declares a label at or above the cutline.
    pub fn declares_at_or_above_cutline(&self) -> bool {
        self.declared_label.is_at_or_above_cutline()
    }

    /// The state implied by the active reasons and the declared label.
    pub fn computed_state(&self) -> DurabilityState {
        if self.declared_label == LifecycleLabel::Withdrawn {
            return DurabilityState::Withdrawn;
        }
        match self
            .active_reasons
            .iter()
            .min_by_key(|reason| reason.precedence())
        {
            None => DurabilityState::Durable,
            Some(reason) => reason.state_group(),
        }
    }

    /// The effective label implied by the state and the declared label.
    pub fn computed_effective_label(&self) -> LifecycleLabel {
        match self.computed_state() {
            DurabilityState::Durable => self.declared_label,
            DurabilityState::Withdrawn => LifecycleLabel::Withdrawn,
            _ => {
                // Narrowing drops the lane below the cutline: take the
                // less-supported of the declared label and beta.
                let rank = self.declared_label.rank().min(LifecycleLabel::Beta.rank());
                LifecycleLabel::from_rank(rank)
            }
        }
    }

    /// True when the row may hold promotion: it is a release-blocking lane,
    /// narrowed by a durability-layer gap, declaring a label at or above the
    /// cutline, and not held by an unexpired waiver.
    fn holds_promotion(&self) -> bool {
        self.release_blocking
            && self.durability_state.is_narrowed()
            && self.declares_at_or_above_cutline()
            && !self.is_waived()
    }
}

/// A closed stop-rule that gates publication on a narrowing reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GovernanceRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: DurabilityReason,
    /// Declared labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// Default recommended action.
    pub default_action: MatrixAction,
    /// True when the rule holds publication.
    pub blocks_publication: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// The launch cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryCutline {
    /// The cutline level (`stable`).
    pub cutline_level: LifecycleLabel,
    /// Labels at or above the cutline.
    pub above_cutline_levels: Vec<LifecycleLabel>,
    /// Labels below the cutline.
    pub below_cutline_levels: Vec<LifecycleLabel>,
    /// Open-baseline postures a `must_remain_open` lane may carry.
    pub open_baseline_postures: Vec<BoundaryPosture>,
    /// Description.
    pub description: String,
}

/// Canonical source registers this matrix binds together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceContractRefs {
    /// Open-versus-paid boundary audit (release lane).
    pub open_paid_boundary_audit_ref: String,
    /// Signing/approval quorum policy.
    pub signing_quorum_ref: String,
    /// Third-party-import / repository-compliance manifest.
    pub third_party_import_manifest_ref: String,
    /// Critical-upstream health scorecard.
    pub critical_upstream_health_ref: String,
    /// Maintainer-coverage policy.
    pub maintainer_coverage_policy_ref: String,
    /// Security severity matrix.
    pub security_severity_matrix_ref: String,
}

/// Publication verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    /// Stable publication-gate id.
    pub publication_gate: String,
    /// Proceed/hold decision.
    pub decision: PublicationDecision,
    /// Firing rule ids.
    pub blocking_rule_ids: Vec<String>,
    /// Offending row ids.
    pub blocking_row_ids: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Summary counts carried by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatrixSummary {
    /// Total rows.
    pub total_rows: usize,
    /// Durable rows.
    pub rows_durable: usize,
    /// Narrowed rows.
    pub rows_narrowed: usize,
    /// Rows in the `durable` state.
    pub state_durable: usize,
    /// Rows in the `narrowed_boundary_drift` state.
    pub state_narrowed_boundary_drift: usize,
    /// Rows in the `narrowed_compliance_gap` state.
    pub state_narrowed_compliance_gap: usize,
    /// Rows in the `narrowed_authority_gap` state.
    pub state_narrowed_authority_gap: usize,
    /// Rows in the `narrowed_continuity_gap` state.
    pub state_narrowed_continuity_gap: usize,
    /// Rows in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Rows in the `withdrawn` state.
    pub state_withdrawn: usize,
    /// Rows flagged `must_remain_open`.
    pub must_remain_open_rows: usize,
    /// Rows carrying an open-baseline posture.
    pub open_baseline_rows: usize,
    /// Release-blocking rows.
    pub release_blocking_total: usize,
    /// Release-blocking rows that are durable.
    pub release_blocking_durable: usize,
    /// Release-blocking rows that are narrowed.
    pub release_blocking_narrowed: usize,
    /// Rows held by an active waiver.
    pub rows_on_active_waiver: usize,
    /// Total control bindings.
    pub total_controls: usize,
    /// Satisfied control bindings.
    pub controls_satisfied: usize,
    /// Unsatisfied control bindings.
    pub controls_unsatisfied: usize,
    /// Not-applicable control bindings.
    pub controls_not_applicable: usize,
    /// Proof packets current.
    pub packets_current: usize,
    /// Proof packets due for refresh.
    pub packets_due_for_refresh: usize,
    /// Proof packets breached.
    pub packets_breached: usize,
    /// Proof packets missing.
    pub packets_missing: usize,
    /// Total active narrowing reasons.
    pub total_active_reasons: usize,
    /// Distinct rules firing.
    pub rules_firing: usize,
}

/// The typed open/local-boundary and upstream-durability matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryDurabilityMatrix {
    /// Matrix schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable matrix id.
    pub matrix_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Date the matrix was last reconciled.
    pub as_of: String,
    /// Canonical source registers.
    pub source_contract_refs: SourceContractRefs,
    /// Launch cutline.
    pub boundary_cutline: BoundaryCutline,
    /// Closed asset-lane vocabulary.
    pub asset_lanes: Vec<AssetLane>,
    /// Closed boundary-posture vocabulary.
    pub boundary_postures: Vec<BoundaryPosture>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed control-dimension vocabulary.
    pub control_dimensions: Vec<ControlDimension>,
    /// Closed durability-state vocabulary.
    pub durability_states: Vec<DurabilityState>,
    /// Closed durability-reason vocabulary.
    pub durability_reasons: Vec<DurabilityReason>,
    /// Closed matrix-action vocabulary.
    pub matrix_actions: Vec<MatrixAction>,
    /// Stop rules.
    pub rules: Vec<GovernanceRule>,
    /// Durability rows.
    pub rows: Vec<BoundaryDurabilityRow>,
    /// Publication verdict.
    pub publication: Publication,
    /// Summary counts.
    pub summary: MatrixSummary,
}

impl BoundaryDurabilityMatrix {
    /// Returns the row with the given id.
    pub fn row(&self, entry_id: &str) -> Option<&BoundaryDurabilityRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the rows for an asset lane.
    pub fn rows_for_lane(&self, lane: AssetLane) -> Vec<&BoundaryDurabilityRow> {
        self.rows
            .iter()
            .filter(|row| row.asset_lane == lane)
            .collect()
    }

    /// Returns the durable rows.
    pub fn rows_durable(&self) -> Vec<&BoundaryDurabilityRow> {
        self.rows.iter().filter(|row| row.is_durable()).collect()
    }

    /// Returns the narrowed rows.
    pub fn rows_narrowed(&self) -> Vec<&BoundaryDurabilityRow> {
        self.rows
            .iter()
            .filter(|row| row.durability_state.is_narrowed())
            .collect()
    }

    /// Returns the rule with the given trigger reason, if any.
    fn rule_for(&self, reason: DurabilityReason) -> Option<&GovernanceRule> {
        self.rules.iter().find(|rule| rule.trigger_reason == reason)
    }

    /// Recomputes the firing rule ids: a blocking rule fires when a
    /// promotion-holding row carries its trigger reason at an applicable label.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for rule in &self.rules {
            if !rule.blocks_publication {
                continue;
            }
            let fires = self.rows.iter().any(|row| {
                row.holds_promotion()
                    && row.has_active_reason(rule.trigger_reason)
                    && rule.applies_to_labels.contains(&row.declared_label)
            });
            if fires {
                ids.insert(rule.rule_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the offending row ids: promotion-holding rows carrying a
    /// reason watched by a firing blocking rule.
    pub fn computed_blocking_row_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if !row.holds_promotion() {
                continue;
            }
            let blocked = row.active_reasons.iter().any(|reason| {
                self.rule_for(*reason).is_some_and(|rule| {
                    rule.blocks_publication && rule.applies_to_labels.contains(&row.declared_label)
                })
            });
            if blocked {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the publication decision.
    pub fn computed_decision(&self) -> PublicationDecision {
        if self.computed_blocking_row_ids().is_empty() {
            PublicationDecision::Proceed
        } else {
            PublicationDecision::Hold
        }
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> MatrixSummary {
        let count_state = |state: DurabilityState| {
            self.rows
                .iter()
                .filter(|r| r.durability_state == state)
                .count()
        };
        let count_packet = |s: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|r| r.proof_packet.slo_state == s)
                .count()
        };
        let count_control = |s: ControlState| {
            self.rows
                .iter()
                .flat_map(|r| r.compliance_controls.iter())
                .filter(|c| c.state == s)
                .count()
        };
        MatrixSummary {
            total_rows: self.rows.len(),
            rows_durable: self.rows_durable().len(),
            rows_narrowed: self.rows_narrowed().len(),
            state_durable: count_state(DurabilityState::Durable),
            state_narrowed_boundary_drift: count_state(DurabilityState::NarrowedBoundaryDrift),
            state_narrowed_compliance_gap: count_state(DurabilityState::NarrowedComplianceGap),
            state_narrowed_authority_gap: count_state(DurabilityState::NarrowedAuthorityGap),
            state_narrowed_continuity_gap: count_state(DurabilityState::NarrowedContinuityGap),
            state_narrowed_stale: count_state(DurabilityState::NarrowedStale),
            state_withdrawn: count_state(DurabilityState::Withdrawn),
            must_remain_open_rows: self.rows.iter().filter(|r| r.must_remain_open).count(),
            open_baseline_rows: self
                .rows
                .iter()
                .filter(|r| r.boundary_posture.is_open_baseline())
                .count(),
            release_blocking_total: self.rows.iter().filter(|r| r.release_blocking).count(),
            release_blocking_durable: self
                .rows
                .iter()
                .filter(|r| r.release_blocking && r.is_durable())
                .count(),
            release_blocking_narrowed: self
                .rows
                .iter()
                .filter(|r| r.release_blocking && r.durability_state.is_narrowed())
                .count(),
            rows_on_active_waiver: self.rows.iter().filter(|r| r.is_waived()).count(),
            total_controls: self.rows.iter().map(|r| r.compliance_controls.len()).sum(),
            controls_satisfied: count_control(ControlState::Satisfied),
            controls_unsatisfied: count_control(ControlState::Unsatisfied),
            controls_not_applicable: count_control(ControlState::NotApplicable),
            packets_current: count_packet(FreshnessSloState::Current),
            packets_due_for_refresh: count_packet(FreshnessSloState::DueForRefresh),
            packets_breached: count_packet(FreshnessSloState::Breached),
            packets_missing: count_packet(FreshnessSloState::Missing),
            total_active_reasons: self.rows.iter().map(|r| r.active_reasons.len()).sum(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// A copy-safe projection for reuse by release packets, docs/boundary
    /// manifests, repository-compliance scans, and shiproom gates. It carries
    /// only the boundary posture, effective label, durability state, active
    /// reasons, and reuse destinations — never the credential-free-but-detailed
    /// authority and proof internals.
    pub fn reuse_projection(&self) -> Vec<BoundaryReuseRow> {
        self.rows
            .iter()
            .map(|row| BoundaryReuseRow {
                entry_id: row.entry_id.clone(),
                asset_lane: row.asset_lane,
                must_remain_open: row.must_remain_open,
                boundary_posture: row.boundary_posture,
                support_class: row.support_class,
                declared_label: row.declared_label,
                effective_label: row.effective_label,
                durability_state: row.durability_state,
                release_blocking: row.release_blocking,
                active_reasons: row.active_reasons.clone(),
                reuse_destinations: row.reuse_destinations.clone(),
            })
            .collect()
    }

    /// Validates the matrix, returning every violation found.
    pub fn validate(&self) -> Vec<MatrixViolation> {
        let mut v = Vec::new();

        if self.schema_version != M5_BOUNDARY_AND_UPSTREAM_DURABILITY_SCHEMA_VERSION {
            v.push(MatrixViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_BOUNDARY_AND_UPSTREAM_DURABILITY_RECORD_KIND {
            v.push(MatrixViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }

        self.validate_vocabularies(&mut v);

        if self.rows.is_empty() {
            v.push(MatrixViolation::EmptyMatrix);
        }

        // Every asset lane must be covered.
        for lane in AssetLane::ALL {
            if !self.rows.iter().any(|row| row.asset_lane == lane) {
                v.push(MatrixViolation::AssetLaneUncovered { lane });
            }
        }

        // Every blocking reason must have a stop rule.
        for reason in DurabilityReason::ALL {
            if self.rule_for(reason).is_none() {
                v.push(MatrixViolation::ReasonUncoveredByRule { reason });
            }
        }

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            self.validate_row(row, &mut seen, &mut v);
        }

        // Verdict and summary coherence.
        if self.publication.decision != self.computed_decision() {
            v.push(MatrixViolation::PublicationDecisionInconsistent);
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            v.push(MatrixViolation::PublicationBlockingRulesMismatch);
        }
        if self.publication.blocking_row_ids != self.computed_blocking_row_ids() {
            v.push(MatrixViolation::PublicationBlockingRowsMismatch);
        }
        if self.summary != self.computed_summary() {
            v.push(MatrixViolation::SummaryMismatch);
        }

        v
    }

    fn validate_vocabularies(&self, v: &mut Vec<MatrixViolation>) {
        if self.asset_lanes != AssetLane::ALL.to_vec() {
            v.push(MatrixViolation::ClosedVocabularyMismatch {
                field: "asset_lanes",
            });
        }
        if self.boundary_postures != BoundaryPosture::ALL.to_vec() {
            v.push(MatrixViolation::ClosedVocabularyMismatch {
                field: "boundary_postures",
            });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            v.push(MatrixViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.control_dimensions != ControlDimension::ALL.to_vec() {
            v.push(MatrixViolation::ClosedVocabularyMismatch {
                field: "control_dimensions",
            });
        }
        if self.durability_states != DurabilityState::ALL.to_vec() {
            v.push(MatrixViolation::ClosedVocabularyMismatch {
                field: "durability_states",
            });
        }
        if self.durability_reasons != DurabilityReason::ALL.to_vec() {
            v.push(MatrixViolation::ClosedVocabularyMismatch {
                field: "durability_reasons",
            });
        }
        if self.matrix_actions != MatrixAction::ALL.to_vec() {
            v.push(MatrixViolation::ClosedVocabularyMismatch {
                field: "matrix_actions",
            });
        }
        if self.boundary_cutline.cutline_level != LifecycleLabel::Stable
            || self.boundary_cutline.open_baseline_postures
                != BoundaryPosture::OPEN_BASELINE.to_vec()
        {
            v.push(MatrixViolation::ClosedVocabularyMismatch {
                field: "boundary_cutline",
            });
        }
    }

    fn validate_row(
        &self,
        row: &BoundaryDurabilityRow,
        seen: &mut BTreeSet<String>,
        v: &mut Vec<MatrixViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("subject_ref", &row.subject_ref),
            ("subject_summary", &row.subject_summary),
            ("rationale", &row.rationale),
        ] {
            if value.trim().is_empty() {
                v.push(MatrixViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }
        if !seen.insert(row.entry_id.clone()) {
            v.push(MatrixViolation::DuplicateEntryId {
                entry_id: row.entry_id.clone(),
            });
        }
        if row.compliance_controls.is_empty() {
            v.push(MatrixViolation::RowMissingControls {
                entry_id: row.entry_id.clone(),
            });
        }
        if row.reuse_destinations.is_empty() {
            v.push(MatrixViolation::RowMissingReuseDestinations {
                entry_id: row.entry_id.clone(),
            });
        }

        // Support class must match posture.
        if row.support_class != row.boundary_posture.expected_support_class() {
            v.push(MatrixViolation::SupportClassPostureMismatch {
                entry_id: row.entry_id.clone(),
            });
        }

        // Open-baseline guardrail: a must-remain-open lane off the open baseline
        // must narrow on boundary drift and name the reason.
        if row.must_remain_open && !row.boundary_posture.is_open_baseline() {
            if !row.has_active_reason(DurabilityReason::BoundaryBaselineViolated) {
                v.push(MatrixViolation::MustRemainOpenViolated {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else if row.has_active_reason(DurabilityReason::BoundaryBaselineViolated) {
            // A baseline-violation reason on a row that is open (or not flagged)
            // is incoherent.
            v.push(MatrixViolation::ReasonNotJustified {
                entry_id: row.entry_id.clone(),
                reason: DurabilityReason::BoundaryBaselineViolated,
            });
        }

        self.validate_reason_evidence(row, v);
        self.validate_state_and_label(row, v);
    }

    /// Every active reason must be justified by the row's own fields, and every
    /// structural gap must surface its reason.
    fn validate_reason_evidence(&self, row: &BoundaryDurabilityRow, v: &mut Vec<MatrixViolation>) {
        let unsatisfied_control = row
            .compliance_controls
            .iter()
            .any(|c| c.state == ControlState::Unsatisfied);
        let quorum_unmet = !row.emergency_authority.signer_quorum.is_met();
        let owner_missing = !row.emergency_authority.owners_present();
        let spof = row.continuity.single_point_of_failure;
        let backup_missing = row.continuity.backup_coverage == BackupCoverage::Uncovered;
        let upstream_unowned = row
            .continuity
            .critical_upstreams
            .iter()
            .any(|u| u.owner_ref.trim().is_empty());
        let proof_breached = row.proof_packet.slo_state == FreshnessSloState::Breached;
        let proof_missing = row.proof_packet.slo_state == FreshnessSloState::Missing;
        let signoff_missing = !row.owner_signoff.signed_off;

        // reason present ⇒ justified
        for reason in &row.active_reasons {
            let justified = match reason {
                DurabilityReason::BoundaryBaselineViolated => {
                    row.must_remain_open && !row.boundary_posture.is_open_baseline()
                }
                DurabilityReason::ComplianceControlUnsatisfied => unsatisfied_control,
                DurabilityReason::SignerQuorumUnmet => quorum_unmet,
                DurabilityReason::EmergencyAuthorityOwnerMissing => owner_missing,
                DurabilityReason::SinglePointOfFailure => spof,
                DurabilityReason::BackupCoverageMissing => backup_missing,
                DurabilityReason::CriticalUpstreamUnowned => upstream_unowned,
                DurabilityReason::ProofFreshnessBreached => proof_breached,
                DurabilityReason::ProofPacketMissing => proof_missing,
                DurabilityReason::OwnerSignoffMissing => signoff_missing,
                DurabilityReason::WaiverExpired => row.waiver.is_some(),
            };
            if !justified {
                v.push(MatrixViolation::ReasonNotJustified {
                    entry_id: row.entry_id.clone(),
                    reason: *reason,
                });
            }
        }

        // structural gap ⇒ reason present (so a gap can never hide).
        let require = |present: bool, reason: DurabilityReason, v: &mut Vec<MatrixViolation>| {
            if present && !row.has_active_reason(reason) {
                v.push(MatrixViolation::GapWithoutReason {
                    entry_id: row.entry_id.clone(),
                    reason,
                });
            }
        };
        require(
            unsatisfied_control,
            DurabilityReason::ComplianceControlUnsatisfied,
            v,
        );
        require(quorum_unmet, DurabilityReason::SignerQuorumUnmet, v);
        require(
            owner_missing,
            DurabilityReason::EmergencyAuthorityOwnerMissing,
            v,
        );
        require(spof, DurabilityReason::SinglePointOfFailure, v);
        require(backup_missing, DurabilityReason::BackupCoverageMissing, v);
        require(
            upstream_unowned,
            DurabilityReason::CriticalUpstreamUnowned,
            v,
        );
        require(proof_breached, DurabilityReason::ProofFreshnessBreached, v);
        require(proof_missing, DurabilityReason::ProofPacketMissing, v);
        require(signoff_missing, DurabilityReason::OwnerSignoffMissing, v);
    }

    fn validate_state_and_label(&self, row: &BoundaryDurabilityRow, v: &mut Vec<MatrixViolation>) {
        // durable ⇒ no reasons; narrowed ⇒ at least one reason.
        if row.is_durable() && !row.active_reasons.is_empty() {
            v.push(MatrixViolation::DurableWithActiveReason {
                entry_id: row.entry_id.clone(),
            });
        }
        if row.durability_state.is_narrowed() && row.active_reasons.is_empty() {
            v.push(MatrixViolation::NarrowedWithoutReason {
                entry_id: row.entry_id.clone(),
            });
        }
        // state must equal the state implied by the reasons.
        if row.durability_state != row.computed_state() {
            v.push(MatrixViolation::StateReasonMismatch {
                entry_id: row.entry_id.clone(),
                declared: row.durability_state,
                computed: row.computed_state(),
            });
        }
        // never widen: effective may not rank above declared.
        if row.effective_label.rank() > row.declared_label.rank() {
            v.push(MatrixViolation::EffectiveLabelExceedsDeclared {
                entry_id: row.entry_id.clone(),
            });
        }
        // effective must equal the computed effective label.
        if row.effective_label != row.computed_effective_label() {
            v.push(MatrixViolation::EffectiveLabelMismatch {
                entry_id: row.entry_id.clone(),
            });
        }
        // a narrowed row must drop below the cutline.
        if row.durability_state.is_narrowed() && row.effective_label.is_at_or_above_cutline() {
            v.push(MatrixViolation::NarrowedAboveCutline {
                entry_id: row.entry_id.clone(),
            });
        }
    }
}

/// A copy-safe reuse projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryReuseRow {
    /// Row id.
    pub entry_id: String,
    /// Asset lane.
    pub asset_lane: AssetLane,
    /// Must-remain-open flag.
    pub must_remain_open: bool,
    /// Boundary posture.
    pub boundary_posture: BoundaryPosture,
    /// Support class.
    pub support_class: SupportClass,
    /// Declared label.
    pub declared_label: LifecycleLabel,
    /// Effective label after narrowing.
    pub effective_label: LifecycleLabel,
    /// Durability state.
    pub durability_state: DurabilityState,
    /// Release-blocking flag.
    pub release_blocking: bool,
    /// Active narrowing reasons.
    pub active_reasons: Vec<DurabilityReason>,
    /// Reuse destinations.
    pub reuse_destinations: Vec<String>,
}

/// A validation violation for the boundary/durability matrix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatrixViolation {
    /// Unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found.
        actual: u32,
    },
    /// Unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The matrix has no rows.
    EmptyMatrix,
    /// An asset lane has no row.
    AssetLaneUncovered {
        /// Uncovered lane.
        lane: AssetLane,
    },
    /// A narrowing reason has no stop rule.
    ReasonUncoveredByRule {
        /// Uncovered reason.
        reason: DurabilityReason,
    },
    /// A row id appears more than once.
    DuplicateEntryId {
        /// Duplicate id.
        entry_id: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Row id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row binds no compliance controls.
    RowMissingControls {
        /// Row id.
        entry_id: String,
    },
    /// A row lists no reuse destinations.
    RowMissingReuseDestinations {
        /// Row id.
        entry_id: String,
    },
    /// The support class disagrees with the boundary posture.
    SupportClassPostureMismatch {
        /// Row id.
        entry_id: String,
    },
    /// A must-remain-open lane drifted off the open baseline without narrowing.
    MustRemainOpenViolated {
        /// Row id.
        entry_id: String,
    },
    /// An active reason is not justified by the row's fields.
    ReasonNotJustified {
        /// Row id.
        entry_id: String,
        /// Offending reason.
        reason: DurabilityReason,
    },
    /// A structural gap is present but its reason is not active.
    GapWithoutReason {
        /// Row id.
        entry_id: String,
        /// Missing reason.
        reason: DurabilityReason,
    },
    /// A durable row carries an active reason.
    DurableWithActiveReason {
        /// Row id.
        entry_id: String,
    },
    /// A narrowed row carries no reason.
    NarrowedWithoutReason {
        /// Row id.
        entry_id: String,
    },
    /// The durability state disagrees with the active reasons.
    StateReasonMismatch {
        /// Row id.
        entry_id: String,
        /// Declared state.
        declared: DurabilityState,
        /// Computed state.
        computed: DurabilityState,
    },
    /// The effective label ranks above the declared label.
    EffectiveLabelExceedsDeclared {
        /// Row id.
        entry_id: String,
    },
    /// The effective label disagrees with the computed effective label.
    EffectiveLabelMismatch {
        /// Row id.
        entry_id: String,
    },
    /// A narrowed row did not drop below the cutline.
    NarrowedAboveCutline {
        /// Row id.
        entry_id: String,
    },
    /// The publication decision disagrees with the firing rules.
    PublicationDecisionInconsistent,
    /// The recorded blocking rule ids disagree with the computed set.
    PublicationBlockingRulesMismatch,
    /// The recorded blocking row ids disagree with the computed set.
    PublicationBlockingRowsMismatch,
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for MatrixViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported matrix schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported matrix record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "matrix {field} is not the canonical value")
            }
            Self::EmptyMatrix => write!(f, "matrix has no rows"),
            Self::AssetLaneUncovered { lane } => {
                write!(f, "asset lane {} has no row", lane.as_str())
            }
            Self::ReasonUncoveredByRule { reason } => {
                write!(f, "reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate row id {entry_id}"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "row {entry_id} has empty field {field_name}"),
            Self::RowMissingControls { entry_id } => {
                write!(f, "row {entry_id} binds no compliance controls")
            }
            Self::RowMissingReuseDestinations { entry_id } => {
                write!(f, "row {entry_id} lists no reuse destinations")
            }
            Self::SupportClassPostureMismatch { entry_id } => {
                write!(f, "row {entry_id} support class disagrees with its boundary posture")
            }
            Self::MustRemainOpenViolated { entry_id } => write!(
                f,
                "row {entry_id} must remain open but carries a non-open-baseline posture without narrowing on boundary drift"
            ),
            Self::ReasonNotJustified { entry_id, reason } => write!(
                f,
                "row {entry_id} names reason {} which its fields do not justify",
                reason.as_str()
            ),
            Self::GapWithoutReason { entry_id, reason } => write!(
                f,
                "row {entry_id} has a structural gap but does not name reason {}",
                reason.as_str()
            ),
            Self::DurableWithActiveReason { entry_id } => {
                write!(f, "durable row {entry_id} carries an active narrowing reason")
            }
            Self::NarrowedWithoutReason { entry_id } => {
                write!(f, "narrowed row {entry_id} names no reason")
            }
            Self::StateReasonMismatch {
                entry_id,
                declared,
                computed,
            } => write!(
                f,
                "row {entry_id} records state {} but its reasons imply {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::EffectiveLabelExceedsDeclared { entry_id } => {
                write!(f, "row {entry_id} effective label is wider than its declared label")
            }
            Self::EffectiveLabelMismatch { entry_id } => {
                write!(f, "row {entry_id} effective label disagrees with its state")
            }
            Self::NarrowedAboveCutline { entry_id } => {
                write!(f, "narrowed row {entry_id} did not drop below the cutline")
            }
            Self::PublicationDecisionInconsistent => {
                write!(f, "publication decision disagrees with the firing rules")
            }
            Self::PublicationBlockingRulesMismatch => {
                write!(f, "publication blocking_rule_ids disagree with the computed set")
            }
            Self::PublicationBlockingRowsMismatch => {
                write!(f, "publication blocking_row_ids disagree with the computed set")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with the rows"),
        }
    }
}

impl Error for MatrixViolation {}

/// Loads the embedded boundary/durability matrix.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in matrix no longer matches
/// [`BoundaryDurabilityMatrix`] — including when a row carries a token outside
/// any closed vocabulary.
pub fn current_m5_boundary_and_upstream_durability(
) -> Result<BoundaryDurabilityMatrix, serde_json::Error> {
    serde_json::from_str(M5_BOUNDARY_AND_UPSTREAM_DURABILITY_JSON)
}

#[cfg(test)]
mod tests;
