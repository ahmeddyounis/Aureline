//! Typed register certifying open-durability across every claimed M5 ecosystem/release row.
//!
//! The sibling registers each make one durability axis inspectable: the versioned boundary-manifest
//! register ([`m5_versioned_boundary_manifests`](crate::m5_versioned_boundary_manifests)) publishes
//! the open-versus-paid boundary per family; the repository-compliance register
//! ([`m5_compliance_and_notice_binding`](crate::m5_compliance_and_notice_binding)) binds
//! REUSE/SPDX/notice/SBOM hygiene; the import-provenance register
//! ([`m5_import_provenance_and_fork_review`](crate::m5_import_provenance_and_fork_review))
//! attributes third-party and generated imports; the release-authority continuity register
//! ([`m5_release_authority_continuity`](crate::m5_release_authority_continuity)) names signer quorum
//! and backup coverage; the emergency-response evidence register
//! ([`m5_emergency_response_evidence`](crate::m5_emergency_response_evidence)) records
//! advisory/revocation/disable drills; and the critical-upstream health register
//! ([`m5_critical_upstream_health`](crate::m5_critical_upstream_health)) rates the protected-path
//! dependencies. None of them *certifies a single claimed M5 ecosystem/release row across all six
//! axes at once* — so a row could carry a green boundary card while its critical import is ownerless,
//! or a healthy upstream while its emergency authority is one irreplaceable human.
//!
//! This module is that certification layer. For every claimed M5 ecosystem/release row it records one
//! [`CertificationRecord`] binding the six durability axes:
//!
//! - the **boundary manifest** ([`BoundaryBinding`]): the versioned open-boundary manifest is
//!   published and release-linked, with no hidden proprietary baseline;
//! - the **repository compliance** ([`ComplianceBinding`]): REUSE/SPDX licensing is current and the
//!   notice inventory and SBOM are bound;
//! - the **import durability** ([`ImportBinding`]): third-party/generated import provenance is
//!   attributed and every critical import is owned;
//! - the **signer authority** ([`AuthorityBinding`]): the signer quorum is met and the emergency
//!   authority is not one irreplaceable human;
//! - the **emergency response** ([`EmergencyBinding`]): the advisory/revocation/disable drill
//!   evidence is current;
//! - the **critical upstream** ([`UpstreamBinding`]): the protected-path dependencies are healthy and
//!   owned.
//!
//! Each record also carries a [`scan_posture`](CertificationRecord::scan_posture) (what the
//! certification scan found) and a [`surface_posture`](CertificationRecord::surface_posture) (what
//! the service-health/release-center/support surface shows). The two **must agree**: a record may
//! never show a clean surface over a scan that found gaps, so a green certification card can never
//! mask a hidden proprietary baseline, an ownerless critical import, a single-person emergency
//! authority, a stale notice/SBOM, an uncovered drill, or an unhealthy upstream.
//!
//! A record is [`CertificationState::Certified`] only when every axis holds, the certification proof
//! is fresh, and the owner signed. Otherwise it narrows on the *specific* axis that thinned out — a
//! boundary, compliance, import, authority, emergency, upstream, or stale-proof gap — never
//! collapsing to one global flag. A narrowed record drops its
//! [`CertificationRecord::effective_label`] below the launch cutline and may never publish an
//! effective label wider than the one it declares.
//!
//! The [`CertificationRule`] set names the closed conditions that gate promotion. An *inherited*
//! narrowing — a row whose declared label already sits below the cutline, or a gap held by an
//! unexpired waiver — is gated upstream and does not itself hold promotion; a *certification* failure
//! on a row whose declared label is still at or above the cutline holds promotion through a shiproom
//! stop rule, recorded in [`OpenDurabilityCertificationRegister::publication`] — a row that depends
//! on a hidden proprietary baseline, an ownerless critical import, or a single-person emergency
//! authority cannot widen a stable claim without a plan. The cross-cutting [`ScanSurfaceParity`]
//! block summarizes scan/surface agreement over every row.
//!
//! The register is checked in at `artifacts/governance/m5-open-durability-certification.json` and
//! embedded here, so this typed consumer and the CI gate agree on every record without a cargo build
//! in CI. The model is metadata-only: every field is a typed state, a boolean flag, a small count, a
//! label, or an opaque ref. It carries no credential bodies, raw provider payloads, actor identities
//! beyond opaque role refs, or proprietary source. Date arithmetic (recomputing proof and waiver
//! freshness against an `as_of` date) lives in the CI gate and the integration test; this model
//! enforces the invariants that hold regardless of the clock: scan/surface parity, the no-widening
//! ceiling, control/fact consistency, reason/state coherence, summary agreement, and the verdict.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_boundary_and_upstream_durability::{
    FreshnessSloState, LifecycleLabel, OwnerSignoff, ProofPacket, SupportClass, Waiver,
};
use crate::m5_versioned_boundary_manifests::M5Family;

/// Supported register schema version.
pub const M5_OPEN_DURABILITY_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_OPEN_DURABILITY_CERTIFICATION_RECORD_KIND: &str =
    "m5_open_durability_certification_register";

/// Repo-relative path to the checked-in register.
pub const M5_OPEN_DURABILITY_CERTIFICATION_PATH: &str =
    "artifacts/governance/m5-open-durability-certification.json";

/// Embedded checked-in register JSON.
pub const M5_OPEN_DURABILITY_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/governance/m5-open-durability-certification.json"
));

/// The kind of claimed M5 row a certification record governs.
///
/// The same certification truth is published for ecosystem rows (extension/provider, registry) and
/// release rows (artifact-graph, channels) — so an ownerless ecosystem import cannot hide behind a
/// healthy release row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RowKind {
    /// An ecosystem row (extension/provider, registry, marketplace).
    Ecosystem,
    /// A release row (artifact-graph, channel/profile).
    Release,
}

impl RowKind {
    /// Every row kind, in declaration order. Each must be exercised by at least one record.
    pub const ALL: [Self; 2] = [Self::Ecosystem, Self::Release];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ecosystem => "ecosystem",
            Self::Release => "release",
        }
    }
}

/// A certification control dimension a record must declare.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlDimension {
    /// Boundary manifest: published, release-linked, no hidden proprietary baseline.
    BoundaryManifest,
    /// Repository compliance: REUSE/SPDX licensing current, notice inventory and SBOM bound.
    RepositoryCompliance,
    /// Import durability: third-party/generated import provenance attributed, critical imports owned.
    ImportDurability,
    /// Signer authority: signer quorum met, no single-person emergency authority.
    SignerAuthority,
    /// Emergency response: advisory/revocation/disable drill evidence current.
    EmergencyResponse,
    /// Critical upstream: protected-path dependencies healthy and owned.
    CriticalUpstream,
    /// Scan/surface parity: the certification scan and the governance surface agree.
    ScanSurfaceParity,
}

impl ControlDimension {
    /// Every control dimension, in declaration order. Every record declares each once.
    pub const ALL: [Self; 7] = [
        Self::BoundaryManifest,
        Self::RepositoryCompliance,
        Self::ImportDurability,
        Self::SignerAuthority,
        Self::EmergencyResponse,
        Self::CriticalUpstream,
        Self::ScanSurfaceParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundaryManifest => "boundary_manifest",
            Self::RepositoryCompliance => "repository_compliance",
            Self::ImportDurability => "import_durability",
            Self::SignerAuthority => "signer_authority",
            Self::EmergencyResponse => "emergency_response",
            Self::CriticalUpstream => "critical_upstream",
            Self::ScanSurfaceParity => "scan_surface_parity",
        }
    }
}

/// State of a row's open-boundary manifest binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryEvidenceState {
    /// The versioned boundary manifest is published and release-linked.
    Published,
    /// The boundary manifest is not published or not release-linked.
    Unpublished,
    /// The open-boundary claim depends on a hidden proprietary baseline.
    HiddenProprietaryBaseline,
}

impl BoundaryEvidenceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Published,
        Self::Unpublished,
        Self::HiddenProprietaryBaseline,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::Unpublished => "unpublished",
            Self::HiddenProprietaryBaseline => "hidden_proprietary_baseline",
        }
    }
}

/// State of a row's repository-compliance binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComplianceEvidenceState {
    /// REUSE/SPDX licensing is current and the notice inventory and SBOM are bound.
    Current,
    /// REUSE/SPDX licensing coverage has aged out of its review window.
    Stale,
    /// The notice inventory or SBOM is not bound to the row's artifacts.
    NoticeBindingMissing,
}

impl ComplianceEvidenceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Current, Self::Stale, Self::NoticeBindingMissing];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::NoticeBindingMissing => "notice_binding_missing",
        }
    }
}

/// State of a row's import-durability binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportEvidenceState {
    /// Import provenance is attributed and every critical import is owned.
    Attributed,
    /// A third-party/generated import carries no provenance attribution.
    ProvenanceMissing,
    /// A critical import has no update owner.
    OwnerlessCriticalImport,
}

impl ImportEvidenceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Attributed,
        Self::ProvenanceMissing,
        Self::OwnerlessCriticalImport,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attributed => "attributed",
            Self::ProvenanceMissing => "provenance_missing",
            Self::OwnerlessCriticalImport => "ownerless_critical_import",
        }
    }
}

/// State of a row's signer-authority binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityEvidenceState {
    /// The signer quorum is met and the emergency authority is not one irreplaceable human.
    QuorumMet,
    /// The available signers are below the required quorum (but more than one).
    QuorumUnmet,
    /// The emergency authority resolves to a single irreplaceable human with no backup.
    SinglePersonAuthority,
}

impl AuthorityEvidenceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::QuorumMet,
        Self::QuorumUnmet,
        Self::SinglePersonAuthority,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::QuorumMet => "quorum_met",
            Self::QuorumUnmet => "quorum_unmet",
            Self::SinglePersonAuthority => "single_person_authority",
        }
    }
}

/// State of a row's emergency-response binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmergencyEvidenceState {
    /// The advisory/revocation/disable drill evidence is current.
    Current,
    /// The drill evidence has aged out of its window.
    Stale,
}

impl EmergencyEvidenceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Current, Self::Stale];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
        }
    }
}

/// State of a row's critical-upstream binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpstreamEvidenceState {
    /// The protected-path dependencies are healthy and owned.
    Healthy,
    /// A protected-path dependency is red-risk or unowned with no approved plan.
    Unhealthy,
}

impl UpstreamEvidenceState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 2] = [Self::Healthy, Self::Unhealthy];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Unhealthy => "unhealthy",
        }
    }
}

/// The posture a scan or a surface reports for a record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Posture {
    /// No certification gap found.
    Clear,
    /// One or more certification gaps found.
    GapsFound,
}

impl Posture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 2] = [Self::Clear, Self::GapsFound];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clear => "clear",
            Self::GapsFound => "gaps_found",
        }
    }
}

/// Satisfaction state of one control binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControlState {
    /// The control holds for this record.
    Satisfied,
    /// The control applies but is not satisfied.
    Unsatisfied,
    /// The control does not apply to this record.
    NotApplicable,
}

impl ControlState {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Unsatisfied => "unsatisfied",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// The state a record earns after narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationState {
    /// Boundary, compliance, import, authority, emergency, upstream, and proof all hold.
    Certified,
    /// The boundary manifest is unpublished or rests on a hidden proprietary baseline.
    NarrowedBoundary,
    /// REUSE/SPDX licensing is stale or the notice/SBOM binding is missing.
    NarrowedCompliance,
    /// An import lacks provenance or a critical import is ownerless.
    NarrowedImport,
    /// The signer quorum is unmet or the emergency authority is one irreplaceable human.
    NarrowedAuthority,
    /// The advisory/revocation/disable drill evidence is stale.
    NarrowedEmergency,
    /// A protected-path dependency is red-risk or unowned.
    NarrowedUpstream,
    /// The proof packet, sign-off, or waiver thinned out.
    NarrowedStale,
    /// The record is withdrawn.
    Withdrawn,
}

impl CertificationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Certified,
        Self::NarrowedBoundary,
        Self::NarrowedCompliance,
        Self::NarrowedImport,
        Self::NarrowedAuthority,
        Self::NarrowedEmergency,
        Self::NarrowedUpstream,
        Self::NarrowedStale,
        Self::Withdrawn,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedBoundary => "narrowed_boundary",
            Self::NarrowedCompliance => "narrowed_compliance",
            Self::NarrowedImport => "narrowed_import",
            Self::NarrowedAuthority => "narrowed_authority",
            Self::NarrowedEmergency => "narrowed_emergency",
            Self::NarrowedUpstream => "narrowed_upstream",
            Self::NarrowedStale => "narrowed_stale",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when the state is a narrowed state (not certified, not withdrawn).
    pub fn is_narrowed(self) -> bool {
        !matches!(self, Self::Certified | Self::Withdrawn)
    }
}

/// A reason a record narrowed. Closed vocabulary; every reason is watched by a [`CertificationRule`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationReason {
    /// The versioned boundary manifest is not published or release-linked.
    BoundaryManifestMissing,
    /// The open-boundary claim depends on a hidden proprietary baseline.
    HiddenProprietaryBaseline,
    /// REUSE/SPDX licensing coverage has aged out of its review window.
    RepositoryComplianceStale,
    /// The notice inventory or SBOM is not bound to the row's artifacts.
    NoticeBindingMissing,
    /// A third-party/generated import carries no provenance attribution.
    ImportProvenanceMissing,
    /// A critical import has no update owner.
    OwnerlessCriticalImport,
    /// The available signers are below the required quorum.
    SignerQuorumUnmet,
    /// The emergency authority resolves to a single irreplaceable human.
    SinglePersonEmergencyAuthority,
    /// The advisory/revocation/disable drill evidence is stale.
    EmergencyResponseStale,
    /// A protected-path dependency is red-risk or unowned.
    CriticalUpstreamUnhealthy,
    /// The certification proof packet aged past its freshness SLO.
    CertificationProofStale,
    /// No certification proof packet is captured.
    CertificationProofMissing,
    /// The owner sign-off is missing.
    OwnerSignoffMissing,
    /// The waiver relied on has expired.
    WaiverExpired,
}

impl CertificationReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::BoundaryManifestMissing,
        Self::HiddenProprietaryBaseline,
        Self::RepositoryComplianceStale,
        Self::NoticeBindingMissing,
        Self::ImportProvenanceMissing,
        Self::OwnerlessCriticalImport,
        Self::SignerQuorumUnmet,
        Self::SinglePersonEmergencyAuthority,
        Self::EmergencyResponseStale,
        Self::CriticalUpstreamUnhealthy,
        Self::CertificationProofStale,
        Self::CertificationProofMissing,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundaryManifestMissing => "boundary_manifest_missing",
            Self::HiddenProprietaryBaseline => "hidden_proprietary_baseline",
            Self::RepositoryComplianceStale => "repository_compliance_stale",
            Self::NoticeBindingMissing => "notice_binding_missing",
            Self::ImportProvenanceMissing => "import_provenance_missing",
            Self::OwnerlessCriticalImport => "ownerless_critical_import",
            Self::SignerQuorumUnmet => "signer_quorum_unmet",
            Self::SinglePersonEmergencyAuthority => "single_person_emergency_authority",
            Self::EmergencyResponseStale => "emergency_response_stale",
            Self::CriticalUpstreamUnhealthy => "critical_upstream_unhealthy",
            Self::CertificationProofStale => "certification_proof_stale",
            Self::CertificationProofMissing => "certification_proof_missing",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Precedence: lower is worse and wins when several reasons are active. The three "do not
    /// certify" guardrails lead: a single-person emergency authority, then an ownerless critical
    /// import, then a hidden proprietary baseline; then upstream, emergency, compliance, and finally
    /// the proof-staleness axis.
    const fn precedence(self) -> u8 {
        match self.state_group() {
            CertificationState::NarrowedAuthority => 0,
            CertificationState::NarrowedImport => 1,
            CertificationState::NarrowedBoundary => 2,
            CertificationState::NarrowedUpstream => 3,
            CertificationState::NarrowedEmergency => 4,
            CertificationState::NarrowedCompliance => 5,
            _ => 6,
        }
    }

    /// The narrowing state this reason maps to.
    pub const fn state_group(self) -> CertificationState {
        match self {
            Self::BoundaryManifestMissing | Self::HiddenProprietaryBaseline => {
                CertificationState::NarrowedBoundary
            }
            Self::RepositoryComplianceStale | Self::NoticeBindingMissing => {
                CertificationState::NarrowedCompliance
            }
            Self::ImportProvenanceMissing | Self::OwnerlessCriticalImport => {
                CertificationState::NarrowedImport
            }
            Self::SignerQuorumUnmet | Self::SinglePersonEmergencyAuthority => {
                CertificationState::NarrowedAuthority
            }
            Self::EmergencyResponseStale => CertificationState::NarrowedEmergency,
            Self::CriticalUpstreamUnhealthy => CertificationState::NarrowedUpstream,
            Self::CertificationProofStale
            | Self::CertificationProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => CertificationState::NarrowedStale,
        }
    }

    /// The control dimension this reason belongs to.
    pub const fn dimension(self) -> ControlDimension {
        match self {
            Self::BoundaryManifestMissing | Self::HiddenProprietaryBaseline => {
                ControlDimension::BoundaryManifest
            }
            Self::RepositoryComplianceStale | Self::NoticeBindingMissing => {
                ControlDimension::RepositoryCompliance
            }
            Self::ImportProvenanceMissing | Self::OwnerlessCriticalImport => {
                ControlDimension::ImportDurability
            }
            Self::SignerQuorumUnmet | Self::SinglePersonEmergencyAuthority => {
                ControlDimension::SignerAuthority
            }
            Self::EmergencyResponseStale => ControlDimension::EmergencyResponse,
            Self::CriticalUpstreamUnhealthy => ControlDimension::CriticalUpstream,
            Self::CertificationProofStale
            | Self::CertificationProofMissing
            | Self::OwnerSignoffMissing
            | Self::WaiverExpired => ControlDimension::ScanSurfaceParity,
        }
    }
}

/// An action a [`CertificationRule`] recommends.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationAction {
    /// Hold promotion until the gap clears.
    HoldPromotion,
    /// Publish and release-link the boundary manifest.
    PublishBoundaryManifest,
    /// Disclose the proprietary baseline on the truth surfaces.
    DiscloseProprietaryBaseline,
    /// Refresh the REUSE/SPDX licensing coverage.
    RefreshRepositoryCompliance,
    /// Bind the notice inventory and SBOM.
    BindNoticesAndSbom,
    /// Attribute the import provenance.
    AttributeImportProvenance,
    /// Assign an owner to the critical import.
    AssignImportOwner,
    /// Meet the signer quorum.
    MeetSignerQuorum,
    /// Add a backup emergency authority.
    AddBackupAuthority,
    /// Refresh the emergency-response drill evidence.
    RefreshEmergencyResponse,
    /// Remediate the red-risk or unowned critical upstream.
    RemediateCriticalUpstream,
    /// Refresh the certification proof packet.
    RefreshCertificationProof,
    /// Request the owner sign-off.
    RequestOwnerSignoff,
}

impl CertificationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::HoldPromotion,
        Self::PublishBoundaryManifest,
        Self::DiscloseProprietaryBaseline,
        Self::RefreshRepositoryCompliance,
        Self::BindNoticesAndSbom,
        Self::AttributeImportProvenance,
        Self::AssignImportOwner,
        Self::MeetSignerQuorum,
        Self::AddBackupAuthority,
        Self::RefreshEmergencyResponse,
        Self::RemediateCriticalUpstream,
        Self::RefreshCertificationProof,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::PublishBoundaryManifest => "publish_boundary_manifest",
            Self::DiscloseProprietaryBaseline => "disclose_proprietary_baseline",
            Self::RefreshRepositoryCompliance => "refresh_repository_compliance",
            Self::BindNoticesAndSbom => "bind_notices_and_sbom",
            Self::AttributeImportProvenance => "attribute_import_provenance",
            Self::AssignImportOwner => "assign_import_owner",
            Self::MeetSignerQuorum => "meet_signer_quorum",
            Self::AddBackupAuthority => "add_backup_authority",
            Self::RefreshEmergencyResponse => "refresh_emergency_response",
            Self::RemediateCriticalUpstream => "remediate_critical_upstream",
            Self::RefreshCertificationProof => "refresh_certification_proof",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// Publication decision recorded by the register.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDecision {
    /// No certification stop rule fires; promotion may proceed.
    Proceed,
    /// A certification stop rule fires; hold promotion.
    Hold,
}

impl PublicationDecision {
    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proceed => "proceed",
            Self::Hold => "hold",
        }
    }
}

/// The open-boundary manifest binding for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoundaryBinding {
    /// Boundary evidence state.
    pub state: BoundaryEvidenceState,
    /// True when the versioned boundary manifest is published and release-linked.
    pub manifest_published: bool,
    /// True when the open-boundary claim rests on a hidden proprietary baseline.
    pub proprietary_baseline_hidden: bool,
    /// Reference to the boundary-manifest register entry.
    pub manifest_ref: String,
    /// Reference to the boundary-durability evidence.
    pub evidence_ref: String,
}

impl BoundaryBinding {
    /// The boundary evidence state implied by the binding's facts (the guardrail wins).
    fn derived_state(&self) -> BoundaryEvidenceState {
        if self.proprietary_baseline_hidden {
            BoundaryEvidenceState::HiddenProprietaryBaseline
        } else if !self.manifest_published {
            BoundaryEvidenceState::Unpublished
        } else {
            BoundaryEvidenceState::Published
        }
    }

    /// The narrowing reason this axis implies, if any.
    fn reason(&self) -> Option<CertificationReason> {
        match self.state {
            BoundaryEvidenceState::Published => None,
            BoundaryEvidenceState::Unpublished => {
                Some(CertificationReason::BoundaryManifestMissing)
            }
            BoundaryEvidenceState::HiddenProprietaryBaseline => {
                Some(CertificationReason::HiddenProprietaryBaseline)
            }
        }
    }
}

/// The repository-compliance binding for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComplianceBinding {
    /// Compliance evidence state.
    pub state: ComplianceEvidenceState,
    /// True when REUSE/SPDX licensing coverage is current.
    pub licensing_current: bool,
    /// True when the notice inventory and SBOM are bound to the row's artifacts.
    pub notice_sbom_bound: bool,
    /// Reference to the repository-compliance register entry.
    pub compliance_register_ref: String,
    /// Reference to the compliance evidence.
    pub evidence_ref: String,
}

impl ComplianceBinding {
    /// The compliance evidence state implied by the binding's facts.
    fn derived_state(&self) -> ComplianceEvidenceState {
        if !self.notice_sbom_bound {
            ComplianceEvidenceState::NoticeBindingMissing
        } else if !self.licensing_current {
            ComplianceEvidenceState::Stale
        } else {
            ComplianceEvidenceState::Current
        }
    }

    /// The narrowing reason this axis implies, if any.
    fn reason(&self) -> Option<CertificationReason> {
        match self.state {
            ComplianceEvidenceState::Current => None,
            ComplianceEvidenceState::Stale => Some(CertificationReason::RepositoryComplianceStale),
            ComplianceEvidenceState::NoticeBindingMissing => {
                Some(CertificationReason::NoticeBindingMissing)
            }
        }
    }
}

/// The import-durability binding for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ImportBinding {
    /// Import evidence state.
    pub state: ImportEvidenceState,
    /// True when third-party/generated import provenance is attributed.
    pub provenance_attributed: bool,
    /// True when every critical import is owned.
    pub critical_import_owned: bool,
    /// Reference to the import-provenance register entry.
    pub import_register_ref: String,
    /// Reference to the import evidence.
    pub evidence_ref: String,
}

impl ImportBinding {
    /// The import evidence state implied by the binding's facts (the guardrail wins).
    fn derived_state(&self) -> ImportEvidenceState {
        if !self.critical_import_owned {
            ImportEvidenceState::OwnerlessCriticalImport
        } else if !self.provenance_attributed {
            ImportEvidenceState::ProvenanceMissing
        } else {
            ImportEvidenceState::Attributed
        }
    }

    /// The narrowing reason this axis implies, if any.
    fn reason(&self) -> Option<CertificationReason> {
        match self.state {
            ImportEvidenceState::Attributed => None,
            ImportEvidenceState::ProvenanceMissing => {
                Some(CertificationReason::ImportProvenanceMissing)
            }
            ImportEvidenceState::OwnerlessCriticalImport => {
                Some(CertificationReason::OwnerlessCriticalImport)
            }
        }
    }
}

/// The signer-authority binding for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityBinding {
    /// Authority evidence state.
    pub state: AuthorityEvidenceState,
    /// Distinct humans required to authorize a protected action.
    pub required_distinct_humans: u32,
    /// Distinct humans actually available.
    pub available_distinct_humans: u32,
    /// True when a backup emergency authority is in place.
    pub backup_present: bool,
    /// Reference to the release-authority continuity register entry.
    pub continuity_register_ref: String,
    /// Reference to the authority evidence.
    pub evidence_ref: String,
}

impl AuthorityBinding {
    /// The authority evidence state implied by the binding's facts (the guardrail wins).
    fn derived_state(&self) -> AuthorityEvidenceState {
        if self.available_distinct_humans <= 1 || !self.backup_present {
            AuthorityEvidenceState::SinglePersonAuthority
        } else if self.available_distinct_humans < self.required_distinct_humans {
            AuthorityEvidenceState::QuorumUnmet
        } else {
            AuthorityEvidenceState::QuorumMet
        }
    }

    /// The narrowing reason this axis implies, if any.
    fn reason(&self) -> Option<CertificationReason> {
        match self.state {
            AuthorityEvidenceState::QuorumMet => None,
            AuthorityEvidenceState::QuorumUnmet => Some(CertificationReason::SignerQuorumUnmet),
            AuthorityEvidenceState::SinglePersonAuthority => {
                Some(CertificationReason::SinglePersonEmergencyAuthority)
            }
        }
    }
}

/// The emergency-response binding for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EmergencyBinding {
    /// Emergency evidence state.
    pub state: EmergencyEvidenceState,
    /// True when the advisory/revocation/disable drill evidence is current.
    pub drill_current: bool,
    /// Reference to the emergency-response evidence register entry.
    pub response_register_ref: String,
    /// Reference to the drill evidence.
    pub evidence_ref: String,
}

impl EmergencyBinding {
    /// The emergency evidence state implied by the binding's facts.
    fn derived_state(&self) -> EmergencyEvidenceState {
        if self.drill_current {
            EmergencyEvidenceState::Current
        } else {
            EmergencyEvidenceState::Stale
        }
    }

    /// The narrowing reason this axis implies, if any.
    fn reason(&self) -> Option<CertificationReason> {
        match self.state {
            EmergencyEvidenceState::Current => None,
            EmergencyEvidenceState::Stale => Some(CertificationReason::EmergencyResponseStale),
        }
    }
}

/// The critical-upstream binding for a row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpstreamBinding {
    /// Upstream evidence state.
    pub state: UpstreamEvidenceState,
    /// True when the protected-path dependencies are healthy and owned.
    pub upstream_healthy: bool,
    /// Reference to the critical-upstream health register entry.
    pub upstream_health_register_ref: String,
    /// Reference to the upstream evidence.
    pub evidence_ref: String,
}

impl UpstreamBinding {
    /// The upstream evidence state implied by the binding's facts.
    fn derived_state(&self) -> UpstreamEvidenceState {
        if self.upstream_healthy {
            UpstreamEvidenceState::Healthy
        } else {
            UpstreamEvidenceState::Unhealthy
        }
    }

    /// The narrowing reason this axis implies, if any.
    fn reason(&self) -> Option<CertificationReason> {
        match self.state {
            UpstreamEvidenceState::Healthy => None,
            UpstreamEvidenceState::Unhealthy => {
                Some(CertificationReason::CriticalUpstreamUnhealthy)
            }
        }
    }
}

/// One certification control binding on a record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationControl {
    /// The control dimension.
    pub dimension: ControlDimension,
    /// Reference to the source register/scan that governs the control.
    pub control_ref: String,
    /// Owning team or role.
    pub owner_ref: String,
    /// Satisfaction state.
    pub state: ControlState,
}

/// One open-durability certification record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationRecord {
    /// Stable record id.
    pub record_id: String,
    /// The M5 family this row serves.
    pub family: M5Family,
    /// The kind of claimed row (ecosystem or release).
    pub row_kind: RowKind,
    /// Human-readable title.
    pub title: String,
    /// Reference to the governed subject.
    pub subject_ref: String,
    /// One-line subject summary.
    pub subject_summary: String,
    /// True when this row is part of the release-blocking set.
    pub release_blocking: bool,
    /// The lifecycle/support label this record declares.
    pub declared_label: LifecycleLabel,
    /// Support class published for the subject.
    pub support_class: SupportClass,
    /// Boundary-manifest binding.
    pub boundary: BoundaryBinding,
    /// Repository-compliance binding.
    pub compliance: ComplianceBinding,
    /// Import-durability binding.
    pub import_durability: ImportBinding,
    /// Signer-authority binding.
    pub authority: AuthorityBinding,
    /// Emergency-response binding.
    pub emergency: EmergencyBinding,
    /// Critical-upstream binding.
    pub upstream: UpstreamBinding,
    /// Per-dimension control bindings.
    pub controls: Vec<CertificationControl>,
    /// What the certification scan found.
    pub scan_posture: Posture,
    /// What the service-health/release-center/support surface shows.
    pub surface_posture: Posture,
    /// Reference to the certification scan.
    pub scan_ref: String,
    /// Reference to the governance surface.
    pub surface_ref: String,
    /// Proof packet grounding the record.
    pub proof_packet: ProofPacket,
    /// Optional waiver holding a gap provisionally.
    pub waiver: Option<Waiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// State earned after narrowing.
    pub certification_state: CertificationState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<CertificationReason>,
    /// The label the record effectively publishes after narrowing.
    pub effective_label: LifecycleLabel,
    /// Surfaces that reuse this record (Help/About, service-health, release-center, support).
    pub surfaces: Vec<String>,
    /// Reviewable reason the record carries its state.
    pub rationale: String,
}

impl CertificationRecord {
    /// True when the record is held by an unexpired waiver.
    pub fn is_waived(&self) -> bool {
        self.waiver.is_some() && !self.has_active_reason(CertificationReason::WaiverExpired)
    }

    /// True when the record carries the given active reason.
    pub fn has_active_reason(&self, reason: CertificationReason) -> bool {
        self.active_reasons.contains(&reason)
    }

    /// True when the record holds a certified state.
    pub fn is_certified(&self) -> bool {
        self.certification_state == CertificationState::Certified
    }

    /// True when the subject declares a label at or above the cutline.
    pub fn declares_at_or_above_cutline(&self) -> bool {
        self.declared_label.is_at_or_above_cutline()
    }

    /// The narrowing reason the boundary axis implies, if any.
    pub fn boundary_reason(&self) -> Option<CertificationReason> {
        self.boundary.reason()
    }

    /// The narrowing reason the compliance axis implies, if any.
    pub fn compliance_reason(&self) -> Option<CertificationReason> {
        self.compliance.reason()
    }

    /// The narrowing reason the import axis implies, if any.
    pub fn import_reason(&self) -> Option<CertificationReason> {
        self.import_durability.reason()
    }

    /// The narrowing reason the authority axis implies, if any.
    pub fn authority_reason(&self) -> Option<CertificationReason> {
        self.authority.reason()
    }

    /// The narrowing reason the emergency axis implies, if any.
    pub fn emergency_reason(&self) -> Option<CertificationReason> {
        self.emergency.reason()
    }

    /// The narrowing reason the upstream axis implies, if any.
    pub fn upstream_reason(&self) -> Option<CertificationReason> {
        self.upstream.reason()
    }

    /// The structural axis reasons (the six durability axes), in axis order.
    pub fn axis_reasons(&self) -> Vec<CertificationReason> {
        [
            self.boundary_reason(),
            self.compliance_reason(),
            self.import_reason(),
            self.authority_reason(),
            self.emergency_reason(),
            self.upstream_reason(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    /// True when any structural axis carries a gap.
    pub fn has_structural_gap(&self) -> bool {
        self.boundary_reason().is_some()
            || self.compliance_reason().is_some()
            || self.import_reason().is_some()
            || self.authority_reason().is_some()
            || self.emergency_reason().is_some()
            || self.upstream_reason().is_some()
    }

    /// The expected control state for a dimension, derived from the subject's facts.
    pub fn expected_control_state(&self, dimension: ControlDimension) -> ControlState {
        let unsatisfied = match dimension {
            ControlDimension::BoundaryManifest => self.boundary_reason().is_some(),
            ControlDimension::RepositoryCompliance => self.compliance_reason().is_some(),
            ControlDimension::ImportDurability => self.import_reason().is_some(),
            ControlDimension::SignerAuthority => self.authority_reason().is_some(),
            ControlDimension::EmergencyResponse => self.emergency_reason().is_some(),
            ControlDimension::CriticalUpstream => self.upstream_reason().is_some(),
            ControlDimension::ScanSurfaceParity => self.scan_posture != self.surface_posture,
        };
        if unsatisfied {
            ControlState::Unsatisfied
        } else {
            ControlState::Satisfied
        }
    }

    /// The state implied by the active reasons and the declared label.
    pub fn computed_state(&self) -> CertificationState {
        if self.declared_label == LifecycleLabel::Withdrawn {
            return CertificationState::Withdrawn;
        }
        match self
            .active_reasons
            .iter()
            .min_by_key(|reason| reason.precedence())
        {
            None => CertificationState::Certified,
            Some(reason) => reason.state_group(),
        }
    }

    /// The effective label implied by the state and the declared label.
    pub fn computed_effective_label(&self) -> LifecycleLabel {
        match self.computed_state() {
            CertificationState::Certified => self.declared_label,
            CertificationState::Withdrawn => LifecycleLabel::Withdrawn,
            _ => {
                // Narrowing drops the subject below the cutline: take the less-supported of the
                // declared label and beta.
                if self.declared_label.rank() <= LifecycleLabel::Beta.rank() {
                    self.declared_label
                } else {
                    LifecycleLabel::Beta
                }
            }
        }
    }

    /// The posture implied by the record's state: gaps found iff narrowed.
    pub fn computed_posture(&self) -> Posture {
        if self.certification_state.is_narrowed() {
            Posture::GapsFound
        } else {
            Posture::Clear
        }
    }

    /// True when the record may hold promotion: a release-blocking subject, narrowed by a
    /// certification gap, declaring a label at or above the cutline, and not held by an unexpired
    /// waiver.
    fn holds_promotion(&self) -> bool {
        self.release_blocking
            && self.certification_state.is_narrowed()
            && self.declares_at_or_above_cutline()
            && !self.is_waived()
    }

    /// True when the scan and the surface agree.
    pub fn scan_surface_agree(&self) -> bool {
        self.scan_posture == self.surface_posture
    }
}

/// A closed stop-rule that gates promotion on a narrowing reason.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The reason that triggers the rule.
    pub trigger_reason: CertificationReason,
    /// Declared labels the rule applies to.
    pub applies_to_labels: Vec<LifecycleLabel>,
    /// Default recommended action.
    pub default_action: CertificationAction,
    /// True when the rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// The launch cutline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationCutline {
    /// The cutline level (`stable`).
    pub cutline_level: LifecycleLabel,
    /// Labels at or above the cutline.
    pub above_cutline_levels: Vec<LifecycleLabel>,
    /// Labels below the cutline.
    pub below_cutline_levels: Vec<LifecycleLabel>,
    /// Description.
    pub description: String,
}

/// Canonical source registers this certification binds together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceContractRefs {
    /// Versioned boundary-manifest register.
    pub boundary_manifest_register_ref: String,
    /// Open/local-boundary durability matrix.
    pub boundary_durability_matrix_ref: String,
    /// Repository-compliance and notice-binding register.
    pub compliance_register_ref: String,
    /// Import-provenance and fork-review register.
    pub import_register_ref: String,
    /// Release-authority continuity register.
    pub authority_continuity_register_ref: String,
    /// Emergency-response evidence register.
    pub emergency_response_register_ref: String,
    /// Critical-upstream health register.
    pub upstream_health_register_ref: String,
    /// Release artifact-graph.
    pub release_graph_ref: String,
    /// Support-export index.
    pub support_export_ref: String,
    /// Shiproom gate register.
    pub shiproom_register_ref: String,
    /// Stable-promotion packet.
    pub stable_promotion_packet_ref: String,
    /// Canonical M5 evidence index.
    pub m5_evidence_index_ref: String,
}

/// Promotion verdict.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    /// Stable promotion-gate id.
    pub publication_gate: String,
    /// Proceed/hold decision.
    pub decision: PublicationDecision,
    /// Firing rule ids.
    pub blocking_rule_ids: Vec<String>,
    /// Offending record ids.
    pub blocking_record_ids: Vec<String>,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Cross-cutting scan/surface parity summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScanSurfaceParity {
    /// Stable parity-gate id.
    pub parity_gate: String,
    /// Total subjects.
    pub subjects_total: usize,
    /// Subjects whose scan and surface agree.
    pub subjects_in_agreement: usize,
    /// Subjects whose scan and surface disagree.
    pub subjects_in_disagreement: usize,
    /// Subjects whose surface reports gaps found.
    pub subjects_with_gaps: usize,
    /// True when every subject's scan and surface agree.
    pub all_subjects_agree: bool,
    /// Reviewable rationale.
    pub rationale: String,
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationSummary {
    /// Total records.
    pub total_records: usize,
    /// Certified records.
    pub records_certified: usize,
    /// Narrowed records.
    pub records_narrowed: usize,
    /// Records in the `certified` state.
    pub state_certified: usize,
    /// Records in the `narrowed_boundary` state.
    pub state_narrowed_boundary: usize,
    /// Records in the `narrowed_compliance` state.
    pub state_narrowed_compliance: usize,
    /// Records in the `narrowed_import` state.
    pub state_narrowed_import: usize,
    /// Records in the `narrowed_authority` state.
    pub state_narrowed_authority: usize,
    /// Records in the `narrowed_emergency` state.
    pub state_narrowed_emergency: usize,
    /// Records in the `narrowed_upstream` state.
    pub state_narrowed_upstream: usize,
    /// Records in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Records in the `withdrawn` state.
    pub state_withdrawn: usize,
    /// Release-blocking records.
    pub release_blocking_total: usize,
    /// Release-blocking records that are narrowed.
    pub release_blocking_narrowed: usize,
    /// Records held by an active waiver.
    pub records_on_active_waiver: usize,
    /// Records carrying a boundary gap.
    pub boundary_gaps: usize,
    /// Records carrying a compliance gap.
    pub compliance_gaps: usize,
    /// Records carrying an import gap.
    pub import_gaps: usize,
    /// Records carrying an authority gap.
    pub authority_gaps: usize,
    /// Records carrying an emergency gap.
    pub emergency_gaps: usize,
    /// Records carrying an upstream gap.
    pub upstream_gaps: usize,
    /// Records depending on a hidden proprietary baseline (guardrail).
    pub hidden_proprietary_baseline_gaps: usize,
    /// Records depending on an ownerless critical import (guardrail).
    pub ownerless_critical_import_gaps: usize,
    /// Records depending on a single-person emergency authority (guardrail).
    pub single_person_authority_gaps: usize,
    /// Total active narrowing reasons.
    pub total_active_reasons: usize,
    /// Distinct rules firing.
    pub rules_firing: usize,
}

/// The typed register of open-durability certification records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenDurabilityCertificationRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register id.
    pub register_id: String,
    /// Lifecycle status of this artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// Date the register was last reconciled.
    pub as_of: String,
    /// Canonical source registers.
    pub source_contract_refs: SourceContractRefs,
    /// Launch cutline.
    pub certification_cutline: CertificationCutline,
    /// Closed family vocabulary.
    pub families: Vec<M5Family>,
    /// Closed row-kind vocabulary.
    pub row_kinds: Vec<RowKind>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed control-dimension vocabulary.
    pub control_dimensions: Vec<ControlDimension>,
    /// Closed boundary-state vocabulary.
    pub boundary_states: Vec<BoundaryEvidenceState>,
    /// Closed compliance-state vocabulary.
    pub compliance_states: Vec<ComplianceEvidenceState>,
    /// Closed import-state vocabulary.
    pub import_states: Vec<ImportEvidenceState>,
    /// Closed authority-state vocabulary.
    pub authority_states: Vec<AuthorityEvidenceState>,
    /// Closed emergency-state vocabulary.
    pub emergency_states: Vec<EmergencyEvidenceState>,
    /// Closed upstream-state vocabulary.
    pub upstream_states: Vec<UpstreamEvidenceState>,
    /// Closed posture vocabulary.
    pub postures: Vec<Posture>,
    /// Closed certification-state vocabulary.
    pub certification_states: Vec<CertificationState>,
    /// Closed certification-reason vocabulary.
    pub certification_reasons: Vec<CertificationReason>,
    /// Closed certification-action vocabulary.
    pub certification_actions: Vec<CertificationAction>,
    /// Stop rules.
    pub rules: Vec<CertificationRule>,
    /// Per-row records.
    pub records: Vec<CertificationRecord>,
    /// Cross-cutting scan/surface parity summary.
    pub scan_surface_parity: ScanSurfaceParity,
    /// Promotion verdict.
    pub publication: Publication,
    /// Summary counts.
    pub summary: CertificationSummary,
}

impl OpenDurabilityCertificationRegister {
    /// Returns the record with the given id.
    pub fn record(&self, record_id: &str) -> Option<&CertificationRecord> {
        self.records.iter().find(|r| r.record_id == record_id)
    }

    /// Returns the certified records.
    pub fn records_certified(&self) -> Vec<&CertificationRecord> {
        self.records.iter().filter(|r| r.is_certified()).collect()
    }

    /// Returns the narrowed records.
    pub fn records_narrowed(&self) -> Vec<&CertificationRecord> {
        self.records
            .iter()
            .filter(|r| r.certification_state.is_narrowed())
            .collect()
    }

    /// Returns the records of a given row kind.
    pub fn records_of_kind(&self, kind: RowKind) -> Vec<&CertificationRecord> {
        self.records.iter().filter(|r| r.row_kind == kind).collect()
    }

    /// Returns the rule with the given trigger reason, if any.
    fn rule_for(&self, reason: CertificationReason) -> Option<&CertificationRule> {
        self.rules.iter().find(|rule| rule.trigger_reason == reason)
    }

    /// Recomputes the firing rule ids: a blocking rule fires when a promotion-holding record carries
    /// its trigger reason at an applicable label.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for rule in &self.rules {
            if !rule.blocks_promotion {
                continue;
            }
            let fires = self.records.iter().any(|r| {
                r.holds_promotion()
                    && r.has_active_reason(rule.trigger_reason)
                    && rule.applies_to_labels.contains(&r.declared_label)
            });
            if fires {
                ids.insert(rule.rule_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the offending record ids: promotion-holding records carrying a reason watched by a
    /// firing blocking rule.
    pub fn computed_blocking_record_ids(&self) -> Vec<String> {
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for r in &self.records {
            if !r.holds_promotion() {
                continue;
            }
            let blocked = r.active_reasons.iter().any(|reason| {
                self.rule_for(*reason).is_some_and(|rule| {
                    rule.blocks_promotion && rule.applies_to_labels.contains(&r.declared_label)
                })
            });
            if blocked {
                ids.insert(r.record_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the promotion decision.
    pub fn computed_decision(&self) -> PublicationDecision {
        if self.computed_blocking_record_ids().is_empty() {
            PublicationDecision::Proceed
        } else {
            PublicationDecision::Hold
        }
    }

    /// Recomputes the cross-cutting scan/surface parity summary.
    pub fn computed_scan_surface_parity(&self) -> ScanSurfaceParity {
        ScanSurfaceParity {
            parity_gate: self.scan_surface_parity.parity_gate.clone(),
            subjects_total: self.records.len(),
            subjects_in_agreement: self
                .records
                .iter()
                .filter(|r| r.scan_surface_agree())
                .count(),
            subjects_in_disagreement: self
                .records
                .iter()
                .filter(|r| !r.scan_surface_agree())
                .count(),
            subjects_with_gaps: self
                .records
                .iter()
                .filter(|r| r.surface_posture == Posture::GapsFound)
                .count(),
            all_subjects_agree: self.records.iter().all(|r| r.scan_surface_agree()),
            rationale: self.scan_surface_parity.rationale.clone(),
        }
    }

    /// Recomputes the summary block from the records.
    pub fn computed_summary(&self) -> CertificationSummary {
        let count_state = |state: CertificationState| {
            self.records
                .iter()
                .filter(|r| r.certification_state == state)
                .count()
        };
        let count_reason = |reason: CertificationReason| {
            self.records
                .iter()
                .filter(|r| r.has_active_reason(reason))
                .count()
        };
        CertificationSummary {
            total_records: self.records.len(),
            records_certified: self.records_certified().len(),
            records_narrowed: self.records_narrowed().len(),
            state_certified: count_state(CertificationState::Certified),
            state_narrowed_boundary: count_state(CertificationState::NarrowedBoundary),
            state_narrowed_compliance: count_state(CertificationState::NarrowedCompliance),
            state_narrowed_import: count_state(CertificationState::NarrowedImport),
            state_narrowed_authority: count_state(CertificationState::NarrowedAuthority),
            state_narrowed_emergency: count_state(CertificationState::NarrowedEmergency),
            state_narrowed_upstream: count_state(CertificationState::NarrowedUpstream),
            state_narrowed_stale: count_state(CertificationState::NarrowedStale),
            state_withdrawn: count_state(CertificationState::Withdrawn),
            release_blocking_total: self.records.iter().filter(|r| r.release_blocking).count(),
            release_blocking_narrowed: self
                .records
                .iter()
                .filter(|r| r.release_blocking && r.certification_state.is_narrowed())
                .count(),
            records_on_active_waiver: self.records.iter().filter(|r| r.is_waived()).count(),
            boundary_gaps: self
                .records
                .iter()
                .filter(|r| r.boundary_reason().is_some())
                .count(),
            compliance_gaps: self
                .records
                .iter()
                .filter(|r| r.compliance_reason().is_some())
                .count(),
            import_gaps: self
                .records
                .iter()
                .filter(|r| r.import_reason().is_some())
                .count(),
            authority_gaps: self
                .records
                .iter()
                .filter(|r| r.authority_reason().is_some())
                .count(),
            emergency_gaps: self
                .records
                .iter()
                .filter(|r| r.emergency_reason().is_some())
                .count(),
            upstream_gaps: self
                .records
                .iter()
                .filter(|r| r.upstream_reason().is_some())
                .count(),
            hidden_proprietary_baseline_gaps: count_reason(
                CertificationReason::HiddenProprietaryBaseline,
            ),
            ownerless_critical_import_gaps: count_reason(
                CertificationReason::OwnerlessCriticalImport,
            ),
            single_person_authority_gaps: count_reason(
                CertificationReason::SinglePersonEmergencyAuthority,
            ),
            total_active_reasons: self.records.iter().map(|r| r.active_reasons.len()).sum(),
            rules_firing: self.computed_blocking_rule_ids().len(),
        }
    }

    /// A copy-safe projection for reuse by Help/About, service-health, release-center publication,
    /// support exports, and shiproom panels. It carries only the family, row kind, declared and
    /// effective labels, state, the scan/surface-agreement flag, the per-axis evidence states,
    /// active reasons, and surfaces — never the detailed binding refs and proof internals.
    pub fn reuse_projection(&self) -> Vec<CertificationReuseRow> {
        self.records
            .iter()
            .map(|r| CertificationReuseRow {
                record_id: r.record_id.clone(),
                family: r.family,
                row_kind: r.row_kind,
                declared_label: r.declared_label,
                effective_label: r.effective_label,
                support_class: r.support_class,
                certification_state: r.certification_state,
                release_blocking: r.release_blocking,
                scan_surface_agree: r.scan_surface_agree(),
                boundary_state: r.boundary.state,
                compliance_state: r.compliance.state,
                import_state: r.import_durability.state,
                authority_state: r.authority.state,
                emergency_state: r.emergency.state,
                upstream_state: r.upstream.state,
                active_reasons: r.active_reasons.clone(),
                surfaces: r.surfaces.clone(),
            })
            .collect()
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<RegisterViolation> {
        let mut v = Vec::new();

        if self.schema_version != M5_OPEN_DURABILITY_CERTIFICATION_SCHEMA_VERSION {
            v.push(RegisterViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_OPEN_DURABILITY_CERTIFICATION_RECORD_KIND {
            v.push(RegisterViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }

        self.validate_vocabularies(&mut v);

        if self.records.is_empty() {
            v.push(RegisterViolation::EmptyRegister);
        }

        // Every row kind must be exercised by at least one record.
        for kind in RowKind::ALL {
            if !self.records.iter().any(|r| r.row_kind == kind) {
                v.push(RegisterViolation::RowKindUncovered { kind });
            }
        }

        // Every reason must have a stop rule.
        for reason in CertificationReason::ALL {
            if self.rule_for(reason).is_none() {
                v.push(RegisterViolation::ReasonUncoveredByRule { reason });
            }
        }

        let mut seen = BTreeSet::new();
        for r in &self.records {
            self.validate_record(r, &mut seen, &mut v);
        }

        // Verdict, parity, and summary coherence.
        if self.publication.decision != self.computed_decision() {
            v.push(RegisterViolation::PublicationDecisionInconsistent);
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            v.push(RegisterViolation::PublicationBlockingRulesMismatch);
        }
        if self.publication.blocking_record_ids != self.computed_blocking_record_ids() {
            v.push(RegisterViolation::PublicationBlockingRecordsMismatch);
        }
        if self.scan_surface_parity != self.computed_scan_surface_parity() {
            v.push(RegisterViolation::ScanSurfaceParityMismatch);
        }
        if self.summary != self.computed_summary() {
            v.push(RegisterViolation::SummaryMismatch);
        }

        v
    }

    fn validate_vocabularies(&self, v: &mut Vec<RegisterViolation>) {
        if self.families != M5Family::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "families" });
        }
        if self.row_kinds != RowKind::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "row_kinds" });
        }
        if self.support_classes != SupportClass::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "support_classes",
            });
        }
        if self.control_dimensions != ControlDimension::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "control_dimensions",
            });
        }
        if self.boundary_states != BoundaryEvidenceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "boundary_states",
            });
        }
        if self.compliance_states != ComplianceEvidenceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "compliance_states",
            });
        }
        if self.import_states != ImportEvidenceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "import_states",
            });
        }
        if self.authority_states != AuthorityEvidenceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "authority_states",
            });
        }
        if self.emergency_states != EmergencyEvidenceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "emergency_states",
            });
        }
        if self.upstream_states != UpstreamEvidenceState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "upstream_states",
            });
        }
        if self.postures != Posture::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch { field: "postures" });
        }
        if self.certification_states != CertificationState::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "certification_states",
            });
        }
        if self.certification_reasons != CertificationReason::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "certification_reasons",
            });
        }
        if self.certification_actions != CertificationAction::ALL.to_vec() {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "certification_actions",
            });
        }
        if self.certification_cutline.cutline_level != LifecycleLabel::Stable {
            v.push(RegisterViolation::ClosedVocabularyMismatch {
                field: "certification_cutline",
            });
        }
    }

    fn validate_record(
        &self,
        r: &CertificationRecord,
        seen: &mut BTreeSet<String>,
        v: &mut Vec<RegisterViolation>,
    ) {
        for (field, value) in [
            ("record_id", &r.record_id),
            ("title", &r.title),
            ("subject_ref", &r.subject_ref),
            ("subject_summary", &r.subject_summary),
            ("rationale", &r.rationale),
        ] {
            if value.trim().is_empty() {
                v.push(RegisterViolation::EmptyField {
                    record_id: r.record_id.clone(),
                    field_name: field,
                });
            }
        }
        if !seen.insert(r.record_id.clone()) {
            v.push(RegisterViolation::DuplicateRecordId {
                record_id: r.record_id.clone(),
            });
        }
        if r.surfaces.is_empty() {
            v.push(RegisterViolation::RecordMissingSurfaces {
                record_id: r.record_id.clone(),
            });
        }

        self.validate_fact_consistency(r, v);
        self.validate_controls(r, v);
        self.validate_reason_evidence(r, v);
        self.validate_scan_surface(r, v);
        self.validate_state_and_label(r, v);
    }

    /// Each axis binding must be internally consistent — so a state token can never sit over a
    /// contradicting fact (a "published" manifest with `manifest_published=false`, an "attributed"
    /// import with an ownerless critical dependency, a "quorum_met" authority with one available
    /// human, and so on) — and every binding ref must be present.
    fn validate_fact_consistency(&self, r: &CertificationRecord, v: &mut Vec<RegisterViolation>) {
        if r.boundary.state != r.boundary.derived_state() {
            v.push(RegisterViolation::AxisFactInconsistent {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::BoundaryManifest,
            });
        }
        if r.boundary.manifest_ref.trim().is_empty() || r.boundary.evidence_ref.trim().is_empty() {
            v.push(RegisterViolation::AxisRefMissing {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::BoundaryManifest,
            });
        }

        if r.compliance.state != r.compliance.derived_state() {
            v.push(RegisterViolation::AxisFactInconsistent {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::RepositoryCompliance,
            });
        }
        if r.compliance.compliance_register_ref.trim().is_empty()
            || r.compliance.evidence_ref.trim().is_empty()
        {
            v.push(RegisterViolation::AxisRefMissing {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::RepositoryCompliance,
            });
        }

        if r.import_durability.state != r.import_durability.derived_state() {
            v.push(RegisterViolation::AxisFactInconsistent {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::ImportDurability,
            });
        }
        if r.import_durability.import_register_ref.trim().is_empty()
            || r.import_durability.evidence_ref.trim().is_empty()
        {
            v.push(RegisterViolation::AxisRefMissing {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::ImportDurability,
            });
        }

        if r.authority.state != r.authority.derived_state() {
            v.push(RegisterViolation::AxisFactInconsistent {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::SignerAuthority,
            });
        }
        if r.authority.continuity_register_ref.trim().is_empty()
            || r.authority.evidence_ref.trim().is_empty()
        {
            v.push(RegisterViolation::AxisRefMissing {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::SignerAuthority,
            });
        }

        if r.emergency.state != r.emergency.derived_state() {
            v.push(RegisterViolation::AxisFactInconsistent {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::EmergencyResponse,
            });
        }
        if r.emergency.response_register_ref.trim().is_empty()
            || r.emergency.evidence_ref.trim().is_empty()
        {
            v.push(RegisterViolation::AxisRefMissing {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::EmergencyResponse,
            });
        }

        if r.upstream.state != r.upstream.derived_state() {
            v.push(RegisterViolation::AxisFactInconsistent {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::CriticalUpstream,
            });
        }
        if r.upstream.upstream_health_register_ref.trim().is_empty()
            || r.upstream.evidence_ref.trim().is_empty()
        {
            v.push(RegisterViolation::AxisRefMissing {
                record_id: r.record_id.clone(),
                dimension: ControlDimension::CriticalUpstream,
            });
        }
    }

    fn validate_controls(&self, r: &CertificationRecord, v: &mut Vec<RegisterViolation>) {
        // Every control dimension must be declared exactly once, and its declared state must equal
        // the state its facts imply — so a control can never assert "satisfied" over a gap.
        for dimension in ControlDimension::ALL {
            let matches: Vec<&CertificationControl> = r
                .controls
                .iter()
                .filter(|c| c.dimension == dimension)
                .collect();
            if matches.len() != 1 {
                v.push(RegisterViolation::ControlDimensionNotDeclaredOnce {
                    record_id: r.record_id.clone(),
                    dimension,
                });
                continue;
            }
            let expected = r.expected_control_state(dimension);
            if matches[0].state != expected {
                v.push(RegisterViolation::ControlStateInconsistent {
                    record_id: r.record_id.clone(),
                    dimension,
                });
            }
        }
    }

    /// Every active reason must be justified by the record's own facts, and every structural gap
    /// must surface its reason.
    fn validate_reason_evidence(&self, r: &CertificationRecord, v: &mut Vec<RegisterViolation>) {
        let proof_stale = r.proof_packet.slo_state == FreshnessSloState::Breached;
        let proof_missing = r.proof_packet.slo_state == FreshnessSloState::Missing;
        let signoff_missing = !r.owner_signoff.signed_off;

        // reason present ⇒ justified
        for reason in &r.active_reasons {
            let justified = match reason {
                CertificationReason::BoundaryManifestMissing
                | CertificationReason::HiddenProprietaryBaseline => {
                    r.boundary_reason() == Some(*reason)
                }
                CertificationReason::RepositoryComplianceStale
                | CertificationReason::NoticeBindingMissing => {
                    r.compliance_reason() == Some(*reason)
                }
                CertificationReason::ImportProvenanceMissing
                | CertificationReason::OwnerlessCriticalImport => {
                    r.import_reason() == Some(*reason)
                }
                CertificationReason::SignerQuorumUnmet
                | CertificationReason::SinglePersonEmergencyAuthority => {
                    r.authority_reason() == Some(*reason)
                }
                CertificationReason::EmergencyResponseStale => r.emergency_reason().is_some(),
                CertificationReason::CriticalUpstreamUnhealthy => r.upstream_reason().is_some(),
                CertificationReason::CertificationProofStale => proof_stale,
                CertificationReason::CertificationProofMissing => proof_missing,
                CertificationReason::OwnerSignoffMissing => signoff_missing,
                CertificationReason::WaiverExpired => r.waiver.is_some(),
            };
            if !justified {
                v.push(RegisterViolation::ReasonNotJustified {
                    record_id: r.record_id.clone(),
                    reason: *reason,
                });
            }
        }

        // structural gap ⇒ reason present (so a gap can never hide).
        let require = |present: Option<CertificationReason>, v: &mut Vec<RegisterViolation>| {
            if let Some(reason) = present {
                if !r.has_active_reason(reason) {
                    v.push(RegisterViolation::GapWithoutReason {
                        record_id: r.record_id.clone(),
                        reason,
                    });
                }
            }
        };
        require(r.boundary_reason(), v);
        require(r.compliance_reason(), v);
        require(r.import_reason(), v);
        require(r.authority_reason(), v);
        require(r.emergency_reason(), v);
        require(r.upstream_reason(), v);
        if proof_stale && !r.has_active_reason(CertificationReason::CertificationProofStale) {
            v.push(RegisterViolation::GapWithoutReason {
                record_id: r.record_id.clone(),
                reason: CertificationReason::CertificationProofStale,
            });
        }
        if proof_missing && !r.has_active_reason(CertificationReason::CertificationProofMissing) {
            v.push(RegisterViolation::GapWithoutReason {
                record_id: r.record_id.clone(),
                reason: CertificationReason::CertificationProofMissing,
            });
        }
        if signoff_missing && !r.has_active_reason(CertificationReason::OwnerSignoffMissing) {
            v.push(RegisterViolation::GapWithoutReason {
                record_id: r.record_id.clone(),
                reason: CertificationReason::OwnerSignoffMissing,
            });
        }
    }

    /// The scan and the surface must agree, and the posture must reflect the gaps — a green surface
    /// may never sit over a scan that found a hidden proprietary baseline, an ownerless critical
    /// import, or a single-person emergency authority.
    fn validate_scan_surface(&self, r: &CertificationRecord, v: &mut Vec<RegisterViolation>) {
        if r.scan_posture != r.surface_posture {
            v.push(RegisterViolation::ScanSurfaceDisagreement {
                record_id: r.record_id.clone(),
            });
        }
        let computed = r.computed_posture();
        if r.surface_posture != computed || r.scan_posture != computed {
            v.push(RegisterViolation::PostureMismatch {
                record_id: r.record_id.clone(),
            });
        }
    }

    fn validate_state_and_label(&self, r: &CertificationRecord, v: &mut Vec<RegisterViolation>) {
        // certified ⇒ no reasons; narrowed ⇒ at least one reason.
        if r.is_certified() && !r.active_reasons.is_empty() {
            v.push(RegisterViolation::CertifiedWithActiveReason {
                record_id: r.record_id.clone(),
            });
        }
        if r.certification_state.is_narrowed() && r.active_reasons.is_empty() {
            v.push(RegisterViolation::NarrowedWithoutReason {
                record_id: r.record_id.clone(),
            });
        }
        // state must equal the state implied by the reasons.
        if r.certification_state != r.computed_state() {
            v.push(RegisterViolation::StateReasonMismatch {
                record_id: r.record_id.clone(),
                declared: r.certification_state,
                computed: r.computed_state(),
            });
        }
        // never widen: effective may not rank above declared.
        if r.effective_label.rank() > r.declared_label.rank() {
            v.push(RegisterViolation::EffectiveLabelExceedsDeclared {
                record_id: r.record_id.clone(),
            });
        }
        // effective must equal the computed effective label.
        if r.effective_label != r.computed_effective_label() {
            v.push(RegisterViolation::EffectiveLabelMismatch {
                record_id: r.record_id.clone(),
            });
        }
        // a narrowed record must drop below the cutline.
        if r.certification_state.is_narrowed() && r.effective_label.is_at_or_above_cutline() {
            v.push(RegisterViolation::NarrowedAboveCutline {
                record_id: r.record_id.clone(),
            });
        }
    }
}

/// A copy-safe reuse projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CertificationReuseRow {
    /// Record id.
    pub record_id: String,
    /// Family.
    pub family: M5Family,
    /// Row kind.
    pub row_kind: RowKind,
    /// Declared label.
    pub declared_label: LifecycleLabel,
    /// Effective label after narrowing.
    pub effective_label: LifecycleLabel,
    /// Support class.
    pub support_class: SupportClass,
    /// Certification state.
    pub certification_state: CertificationState,
    /// Release-blocking flag.
    pub release_blocking: bool,
    /// True when the scan and the surface agree.
    pub scan_surface_agree: bool,
    /// Boundary axis state.
    pub boundary_state: BoundaryEvidenceState,
    /// Compliance axis state.
    pub compliance_state: ComplianceEvidenceState,
    /// Import axis state.
    pub import_state: ImportEvidenceState,
    /// Authority axis state.
    pub authority_state: AuthorityEvidenceState,
    /// Emergency axis state.
    pub emergency_state: EmergencyEvidenceState,
    /// Upstream axis state.
    pub upstream_state: UpstreamEvidenceState,
    /// Active narrowing reasons.
    pub active_reasons: Vec<CertificationReason>,
    /// Reuse surfaces.
    pub surfaces: Vec<String>,
}

/// A validation violation for the open-durability certification register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterViolation {
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
    /// The register has no records.
    EmptyRegister,
    /// A row kind has no record.
    RowKindUncovered {
        /// Uncovered kind.
        kind: RowKind,
    },
    /// A narrowing reason has no stop rule.
    ReasonUncoveredByRule {
        /// Uncovered reason.
        reason: CertificationReason,
    },
    /// A record id appears more than once.
    DuplicateRecordId {
        /// Duplicate id.
        record_id: String,
    },
    /// A required field is empty.
    EmptyField {
        /// Record id.
        record_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A record lists no reuse surfaces.
    RecordMissingSurfaces {
        /// Record id.
        record_id: String,
    },
    /// An axis binding's state disagrees with the facts it governs.
    AxisFactInconsistent {
        /// Record id.
        record_id: String,
        /// Offending axis.
        dimension: ControlDimension,
    },
    /// An axis binding is missing a required reference.
    AxisRefMissing {
        /// Record id.
        record_id: String,
        /// Offending axis.
        dimension: ControlDimension,
    },
    /// A control dimension is not declared exactly once.
    ControlDimensionNotDeclaredOnce {
        /// Record id.
        record_id: String,
        /// Offending dimension.
        dimension: ControlDimension,
    },
    /// A control's declared state disagrees with the facts it governs.
    ControlStateInconsistent {
        /// Record id.
        record_id: String,
        /// Offending dimension.
        dimension: ControlDimension,
    },
    /// An active reason is not justified by the record's fields.
    ReasonNotJustified {
        /// Record id.
        record_id: String,
        /// Offending reason.
        reason: CertificationReason,
    },
    /// A structural gap is present but its reason is not active.
    GapWithoutReason {
        /// Record id.
        record_id: String,
        /// Missing reason.
        reason: CertificationReason,
    },
    /// A record's scan and surface postures disagree.
    ScanSurfaceDisagreement {
        /// Record id.
        record_id: String,
    },
    /// A record's posture disagrees with the gaps its state implies.
    PostureMismatch {
        /// Record id.
        record_id: String,
    },
    /// A certified record carries an active reason.
    CertifiedWithActiveReason {
        /// Record id.
        record_id: String,
    },
    /// A narrowed record carries no reason.
    NarrowedWithoutReason {
        /// Record id.
        record_id: String,
    },
    /// The record state disagrees with the active reasons.
    StateReasonMismatch {
        /// Record id.
        record_id: String,
        /// Declared state.
        declared: CertificationState,
        /// Computed state.
        computed: CertificationState,
    },
    /// The effective label ranks above the declared label.
    EffectiveLabelExceedsDeclared {
        /// Record id.
        record_id: String,
    },
    /// The effective label disagrees with the computed effective label.
    EffectiveLabelMismatch {
        /// Record id.
        record_id: String,
    },
    /// A narrowed record did not drop below the cutline.
    NarrowedAboveCutline {
        /// Record id.
        record_id: String,
    },
    /// The promotion decision disagrees with the firing rules.
    PublicationDecisionInconsistent,
    /// The recorded blocking rule ids disagree with the computed set.
    PublicationBlockingRulesMismatch,
    /// The recorded blocking record ids disagree with the computed set.
    PublicationBlockingRecordsMismatch,
    /// The recorded scan/surface parity disagrees with the computed summary.
    ScanSurfaceParityMismatch,
    /// The summary counts disagree with the records.
    SummaryMismatch,
}

impl fmt::Display for RegisterViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported register schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported register record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "register {field} is not the canonical value")
            }
            Self::EmptyRegister => write!(f, "register has no records"),
            Self::RowKindUncovered { kind } => {
                write!(f, "row kind {} has no record", kind.as_str())
            }
            Self::ReasonUncoveredByRule { reason } => {
                write!(f, "reason {} has no stop rule", reason.as_str())
            }
            Self::DuplicateRecordId { record_id } => {
                write!(f, "duplicate record id {record_id}")
            }
            Self::EmptyField {
                record_id,
                field_name,
            } => write!(f, "record {record_id} has empty field {field_name}"),
            Self::RecordMissingSurfaces { record_id } => {
                write!(f, "record {record_id} lists no reuse surfaces")
            }
            Self::AxisFactInconsistent {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} axis {} state disagrees with its facts",
                dimension.as_str()
            ),
            Self::AxisRefMissing {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} axis {} is missing a required reference",
                dimension.as_str()
            ),
            Self::ControlDimensionNotDeclaredOnce {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} does not declare control {} exactly once",
                dimension.as_str()
            ),
            Self::ControlStateInconsistent {
                record_id,
                dimension,
            } => write!(
                f,
                "record {record_id} control {} state disagrees with its facts",
                dimension.as_str()
            ),
            Self::ReasonNotJustified { record_id, reason } => write!(
                f,
                "record {record_id} names reason {} which its fields do not justify",
                reason.as_str()
            ),
            Self::GapWithoutReason { record_id, reason } => write!(
                f,
                "record {record_id} has a structural gap but does not name reason {}",
                reason.as_str()
            ),
            Self::ScanSurfaceDisagreement { record_id } => {
                write!(f, "record {record_id} scan and surface postures disagree")
            }
            Self::PostureMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} posture disagrees with the gaps its state implies"
                )
            }
            Self::CertifiedWithActiveReason { record_id } => {
                write!(
                    f,
                    "certified record {record_id} carries an active narrowing reason"
                )
            }
            Self::NarrowedWithoutReason { record_id } => {
                write!(f, "narrowed record {record_id} names no reason")
            }
            Self::StateReasonMismatch {
                record_id,
                declared,
                computed,
            } => write!(
                f,
                "record {record_id} records state {} but its reasons imply {}",
                declared.as_str(),
                computed.as_str()
            ),
            Self::EffectiveLabelExceedsDeclared { record_id } => {
                write!(
                    f,
                    "record {record_id} effective label is wider than its declared label"
                )
            }
            Self::EffectiveLabelMismatch { record_id } => {
                write!(
                    f,
                    "record {record_id} effective label disagrees with its state"
                )
            }
            Self::NarrowedAboveCutline { record_id } => {
                write!(
                    f,
                    "narrowed record {record_id} did not drop below the cutline"
                )
            }
            Self::PublicationDecisionInconsistent => {
                write!(f, "promotion decision disagrees with the firing rules")
            }
            Self::PublicationBlockingRulesMismatch => {
                write!(
                    f,
                    "publication blocking_rule_ids disagree with the computed set"
                )
            }
            Self::PublicationBlockingRecordsMismatch => {
                write!(
                    f,
                    "publication blocking_record_ids disagree with the computed set"
                )
            }
            Self::ScanSurfaceParityMismatch => {
                write!(f, "scan_surface_parity disagrees with the computed summary")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with the records"),
        }
    }
}

impl Error for RegisterViolation {}

/// Loads the embedded open-durability certification register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`OpenDurabilityCertificationRegister`] — including when a record carries a token outside any
/// closed vocabulary.
pub fn current_m5_open_durability_certification(
) -> Result<OpenDurabilityCertificationRegister, serde_json::Error> {
    serde_json::from_str(M5_OPEN_DURABILITY_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
