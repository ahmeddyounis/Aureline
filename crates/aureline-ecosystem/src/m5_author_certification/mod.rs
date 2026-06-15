//! Canonical M5 author-side certification — the aggregator that decides whether a
//! marketed M5 ecosystem row's *author lane* may keep its end-user install claim.
//!
//! Where the [`m5_ecosystem_certification`](crate::m5_ecosystem_certification) module
//! is the install-side qualification layer — it rolls the end-user install drills into
//! one decision per marketed family — this module is its author-side counterpart. An
//! [`AuthorCertificationEntry`] aggregates the per-lane evidence the *author* drills
//! produce — local-dev workspace, sideload review, sandbox/runtime inspection, publish
//! preview, hot-reload/last-loaded-build continuity, and anti-abuse transparency —
//! alongside the family's signing state, workspace origin, registry binding, evidence
//! freshness, and owner, and decides whether that row's author-side trust and publish
//! truth still back the end-user install claim it advertises.
//!
//! The decision is honest by construction, on two axes:
//!
//! - The **effective trust posture** an entry publishes is recomputed as the weakest of
//!   the declared posture, the signing-state ceiling, the workspace-origin ceiling, and
//!   the registry-binding ceiling, so a locally-built, side-loaded, or
//!   pending-rebind artifact never inherits a verified-publisher or enterprise-approved
//!   badge just because it was built on a trusted machine.
//! - The **effective author support class** is recomputed as the weakest of the
//!   end-user install claim, the author-side ceiling (source class, evidence freshness,
//!   effective trust posture, and author publish readiness), and the disposition
//!   ceiling. When the author-side ceiling lands below the end-user install claim the
//!   row **narrows automatically** — the marketed claim drops to what the author lane
//!   can actually back, and the [`AuthorCertificationSignal::AuthorClaimBelowInstallClaim`]
//!   signal records the gap.
//!
//! The published [`AuthorCertificationSignal`] set, the
//! [`AuthorCertificationDisposition`], the effective trust posture, the effective
//! support class, and the [`AuthorDowngradePath`] are all recomputed from the entry's
//! facts; the stored values must equal that recomputation or [`M5AuthorCertification::validate`]
//! fails. A lane carrying disclosed warnings narrows the row to
//! [`AuthorCertificationDisposition::ConditionallyCertified`]; a fresh-review-required
//! lane (a hot reload that would widen the runtime class, permissions, or an external
//! executable; a pending rebind), a stale lane, stale evidence, or an author claim below
//! the install claim narrow it to [`AuthorCertificationDisposition::Downgraded`]; and a
//! missing or failed lane, a missing owner, a blocked publish gate, or an anti-abuse
//! quarantine hold each force [`AuthorCertificationDisposition::Uncertified`], whose
//! effective support class collapses to [`SupportClass::Unsupported`].
//!
//! Every narrowed entry carries an explicit [`AuthorDowngradePath`] — the support class
//! the marketed claim drops to and the opaque requalification ref an author follows to
//! restore it — and the packet exports a certification index and a flat downgrade report
//! through [`M5AuthorCertification::export_projection`], so local authoring surfaces,
//! marketplace badges, diagnostics, support, and release evidence all narrow from the
//! same packet rather than parallel spreadsheets.
//!
//! The packet is checked in at `artifacts/ecosystem/m5/m5-author-certification.json` and
//! embedded here, so this typed consumer and any CI gate agree on every record without a
//! cargo build in CI. The model is metadata-only: every field is a typed state or an
//! opaque ref. It carries no credential bodies, raw provider payloads, signing secrets,
//! or sideload source.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    ArtifactFamily, EvidenceFreshness, SupportClass,
};
use crate::m5_anti_abuse_and_repro::BindPublishedIdentity;
use crate::m5_author_and_publish_preview::{
    HostAbiClass, PublishReadiness, RuntimeClass, SignatureState, TrustPosture,
};
use crate::m5_ecosystem_certification::SourceClass;
use crate::m5_workspace_strip::WorkspaceOrigin;

/// Supported M5 author-certification schema version.
pub const M5_AUTHOR_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_AUTHOR_CERTIFICATION_RECORD_KIND: &str = "m5_author_certification";

/// Repo-relative path to the checked-in packet.
pub const M5_AUTHOR_CERTIFICATION_PATH: &str =
    "artifacts/ecosystem/m5/m5-author-certification.json";

/// Embedded checked-in packet JSON.
pub const M5_AUTHOR_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-author-certification.json"
));

/// One author-side drill lane whose evidence is aggregated into a certification entry.
///
/// The lanes are exactly the author-side drills a marketed M5 ecosystem-authoring row
/// must clear before its end-user install claim can stand; every entry carries one
/// [`AuthorLaneEvidence`] for each lane, so a row cannot be certified by running a subset
/// of the author drills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorCertificationLane {
    /// Local extension workspace (unsigned/local-only truth, build freshness) drill.
    LocalDevWorkspace,
    /// Sideload review (source identity, permissions, registry-binding) drill.
    SideloadReview,
    /// Sandbox/runtime inspection (host, capabilities, failures) drill.
    SandboxInspection,
    /// Publish-preview (blocker/warning suite, registry-policy consequences) drill.
    PublishPreview,
    /// Hot-reload/relaunch and last-loaded-build continuity drill.
    ReloadContinuity,
    /// Post-publication anti-abuse transparency drill.
    AntiAbuseTransparency,
}

impl AuthorCertificationLane {
    /// Every author-certification lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalDevWorkspace,
        Self::SideloadReview,
        Self::SandboxInspection,
        Self::PublishPreview,
        Self::ReloadContinuity,
        Self::AntiAbuseTransparency,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDevWorkspace => "local_dev_workspace",
            Self::SideloadReview => "sideload_review",
            Self::SandboxInspection => "sandbox_inspection",
            Self::PublishPreview => "publish_preview",
            Self::ReloadContinuity => "reload_continuity",
            Self::AntiAbuseTransparency => "anti_abuse_transparency",
        }
    }
}

/// The evidence state of one author drill lane for a certification entry.
///
/// Ordered weakest-narrowing to strongest by [`AuthorLaneState::rank`]: a
/// [`AuthorLaneState::Clean`] lane adds no narrowing, while a
/// [`AuthorLaneState::Failed`] lane forces an uncertified row. Each non-clean state maps
/// to one [`AuthorCertificationSignal`] through [`AuthorLaneState::narrowing_signal`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorLaneState {
    /// Clean, passing drill; no narrowing.
    Clean,
    /// Passing with disclosed warnings; narrows to conditionally certified.
    WarningsDisclosed,
    /// A widening hot reload, pending rebind, or other change requires a fresh review.
    FreshReviewRequired,
    /// The lane's drill evidence has lapsed; narrows to downgraded.
    Stale,
    /// No evidence for this lane; forces an uncertified row.
    Missing,
    /// The lane drill failed; forces an uncertified row.
    Failed,
}

impl AuthorLaneState {
    /// Every author-lane state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Clean,
        Self::WarningsDisclosed,
        Self::FreshReviewRequired,
        Self::Stale,
        Self::Missing,
        Self::Failed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Clean => "clean",
            Self::WarningsDisclosed => "warnings_disclosed",
            Self::FreshReviewRequired => "fresh_review_required",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::Failed => "failed",
        }
    }

    /// Monotonic rank; higher means a stronger narrowing.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Clean => 0,
            Self::WarningsDisclosed => 1,
            Self::FreshReviewRequired => 2,
            Self::Stale => 3,
            Self::Missing => 4,
            Self::Failed => 5,
        }
    }

    /// The certification signal this lane state contributes, if any.
    pub const fn narrowing_signal(self) -> Option<AuthorCertificationSignal> {
        match self {
            Self::Clean => None,
            Self::WarningsDisclosed => Some(AuthorCertificationSignal::LaneWarnings),
            Self::FreshReviewRequired => Some(AuthorCertificationSignal::LaneFreshReviewRequired),
            Self::Stale => Some(AuthorCertificationSignal::LaneStale),
            Self::Missing => Some(AuthorCertificationSignal::LaneMissing),
            Self::Failed => Some(AuthorCertificationSignal::LaneFailed),
        }
    }
}

/// One author drill lane's aggregated evidence for a certification entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorLaneEvidence {
    /// Author drill lane this evidence covers.
    pub lane: AuthorCertificationLane,
    /// Aggregated evidence state for the lane.
    pub state: AuthorLaneState,
    /// Opaque ref to the drill packet or report this evidence rolls up.
    pub evidence_ref: String,
    /// Reviewer-facing summary of the lane's evidence.
    pub summary: String,
}

/// A certification signal an entry surfaces.
///
/// Each signal is recomputed from the entry's facts; the entry's stored
/// [`AuthorCertificationEntry::certification_signals`] must equal the recomputed set.
/// Each signal carries a fixed [`AuthorCertificationSignal::min_disposition`], so the
/// published [`AuthorCertificationDisposition`] is a pure function of which signals fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorCertificationSignal {
    /// The signing state, workspace origin, or registry binding capped the effective
    /// trust posture below the declared posture.
    TrustCappedBelowPublishGate,
    /// At least one lane is passing with disclosed warnings.
    LaneWarnings,
    /// The author publish gate publishes only with warnings.
    PublishGateNotClear,
    /// At least one lane needs a fresh review (a widening hot reload or pending rebind).
    LaneFreshReviewRequired,
    /// At least one lane's drill evidence has lapsed.
    LaneStale,
    /// The qualifying evidence freshness is not current.
    EvidenceNotCurrent,
    /// The author-side ceiling lands below the end-user install claim; the row narrows.
    AuthorClaimBelowInstallClaim,
    /// No owner is named for the entry.
    OwnerMissing,
    /// At least one lane has no evidence.
    LaneMissing,
    /// At least one lane drill failed.
    LaneFailed,
    /// The author publish gate is blocked from publish.
    PublishGateBlocked,
    /// The family is withheld under an anti-abuse quarantine hold.
    QuarantineHold,
}

impl AuthorCertificationSignal {
    /// Every certification signal, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::TrustCappedBelowPublishGate,
        Self::LaneWarnings,
        Self::PublishGateNotClear,
        Self::LaneFreshReviewRequired,
        Self::LaneStale,
        Self::EvidenceNotCurrent,
        Self::AuthorClaimBelowInstallClaim,
        Self::OwnerMissing,
        Self::LaneMissing,
        Self::LaneFailed,
        Self::PublishGateBlocked,
        Self::QuarantineHold,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustCappedBelowPublishGate => "trust_capped_below_publish_gate",
            Self::LaneWarnings => "lane_warnings",
            Self::PublishGateNotClear => "publish_gate_not_clear",
            Self::LaneFreshReviewRequired => "lane_fresh_review_required",
            Self::LaneStale => "lane_stale",
            Self::EvidenceNotCurrent => "evidence_not_current",
            Self::AuthorClaimBelowInstallClaim => "author_claim_below_install_claim",
            Self::OwnerMissing => "owner_missing",
            Self::LaneMissing => "lane_missing",
            Self::LaneFailed => "lane_failed",
            Self::PublishGateBlocked => "publish_gate_blocked",
            Self::QuarantineHold => "quarantine_hold",
        }
    }

    /// The minimum certification disposition this signal forces.
    ///
    /// [`Self::TrustCappedBelowPublishGate`] is transparency only — it marks that the
    /// signing state, origin, or binding narrowed the published trust posture without
    /// lowering the disposition on its own. [`Self::LaneWarnings`] and
    /// [`Self::PublishGateNotClear`] force
    /// [`AuthorCertificationDisposition::ConditionallyCertified`]. The narrowing signals —
    /// a fresh-review-required or stale lane, stale evidence, or an author claim below the
    /// install claim — force [`AuthorCertificationDisposition::Downgraded`]. The guardrail
    /// signals — a missing or failed lane, a missing owner, a blocked publish gate, or a
    /// quarantine hold — force [`AuthorCertificationDisposition::Uncertified`].
    pub const fn min_disposition(self) -> AuthorCertificationDisposition {
        match self {
            Self::TrustCappedBelowPublishGate => AuthorCertificationDisposition::Certified,
            Self::LaneWarnings | Self::PublishGateNotClear => {
                AuthorCertificationDisposition::ConditionallyCertified
            }
            Self::LaneFreshReviewRequired
            | Self::LaneStale
            | Self::EvidenceNotCurrent
            | Self::AuthorClaimBelowInstallClaim => AuthorCertificationDisposition::Downgraded,
            Self::OwnerMissing
            | Self::LaneMissing
            | Self::LaneFailed
            | Self::PublishGateBlocked
            | Self::QuarantineHold => AuthorCertificationDisposition::Uncertified,
        }
    }
}

/// The disposition a certification entry publishes.
///
/// Ordered low-to-high by [`AuthorCertificationDisposition::rank`]: a
/// [`AuthorCertificationDisposition::Certified`] entry backs a full author-lane claim,
/// and an [`AuthorCertificationDisposition::Uncertified`] entry backs no claim at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorCertificationDisposition {
    /// No narrowing or guardrail signal applies; the author lane is current and clean.
    Certified,
    /// A disclosed condition applies; certified with conditions.
    ConditionallyCertified,
    /// A narrowing signal applies; the claim is narrowed to a lower support tier.
    Downgraded,
    /// A guardrail signal applies; the author lane backs no claim.
    Uncertified,
}

impl AuthorCertificationDisposition {
    /// Every certification disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Certified,
        Self::ConditionallyCertified,
        Self::Downgraded,
        Self::Uncertified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::ConditionallyCertified => "conditionally_certified",
            Self::Downgraded => "downgraded",
            Self::Uncertified => "uncertified",
        }
    }

    /// Monotonic rank; higher means a weaker certification.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Certified => 0,
            Self::ConditionallyCertified => 1,
            Self::Downgraded => 2,
            Self::Uncertified => 3,
        }
    }

    /// The weaker (higher-rank) of two dispositions.
    pub const fn widen(self, other: Self) -> Self {
        if other.rank() > self.rank() {
            other
        } else {
            self
        }
    }

    /// Highest support class this disposition permits a row to back.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::Certified | Self::ConditionallyCertified => SupportClass::FullySupported,
            Self::Downgraded => SupportClass::CommunitySupported,
            Self::Uncertified => SupportClass::Unsupported,
        }
    }
}

/// Highest support class an effective trust posture lets an author lane back.
///
/// A local-only artifact backs no end-user support claim through the author lane; a
/// registry-bound artifact backs best-effort depth; a verified-publisher or
/// enterprise-approved artifact backs full support.
pub const fn trust_posture_support_ceiling(posture: TrustPosture) -> SupportClass {
    match posture {
        TrustPosture::UnsignedLocalOnly => SupportClass::Unsupported,
        TrustPosture::RegistryBound => SupportClass::BestEffortSupported,
        TrustPosture::VerifiedPublisher | TrustPosture::EnterpriseApproved => {
            SupportClass::FullySupported
        }
    }
}

/// Highest support class an author publish-gate verdict lets a row back.
///
/// A clean publish backs full support; a publish-with-warnings backs best-effort depth;
/// a blocked or withheld publish backs no claim at all.
pub const fn publish_readiness_support_ceiling(readiness: PublishReadiness) -> SupportClass {
    match readiness {
        PublishReadiness::ReadyToPublish => SupportClass::FullySupported,
        PublishReadiness::PublishableWithWarnings => SupportClass::BestEffortSupported,
        PublishReadiness::BlockedFromPublish | PublishReadiness::WithheldQuarantined => {
            SupportClass::Unsupported
        }
    }
}

/// The exact downgrade path published with a narrowed certification entry.
///
/// Every entry carries a downgrade path so a narrowed claim names where it landed and
/// how to recover. The path is recomputed from the entry's facts; the stored value must
/// equal the recomputation. [`Self::applied`] is true whenever the effective support
/// class is below the end-user install claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorDowngradePath {
    /// Whether the effective support class is narrowed below the install claim.
    pub applied: bool,
    /// The end-user install support claim the row started from.
    pub from_support_class: SupportClass,
    /// The effective support class the author lane narrowed to.
    pub to_support_class: SupportClass,
    /// The trust posture the author lane published after the gate caps it.
    pub effective_trust_posture: TrustPosture,
    /// The certification signals that explain the narrowing.
    pub signals: Vec<AuthorCertificationSignal>,
    /// Opaque ref to the requalification steps an author follows to restore the claim.
    pub requalification_ref: String,
}

/// A certification entry for one marketed M5 ecosystem-authoring row.
///
/// The entry aggregates the per-lane author drill evidence and the family's signing,
/// origin, and binding facts into one qualification decision. The published signals,
/// disposition, effective trust posture, effective support class, and downgrade path are
/// recomputed from the entry's facts and must equal the recomputation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorCertificationEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable row label.
    pub display_label: String,
    /// Ref to the author-and-publish-preview matrix row this entry resolves through.
    pub author_matrix_ref: String,
    /// Ref to the install-side certification entry whose claim this entry guards.
    pub install_certification_ref: String,
    /// Ref to the governance-matrix family this entry resolves through.
    pub governance_family_ref: String,
    /// Marketed M5 artifact family.
    pub package_kind: ArtifactFamily,
    /// Distribution source class.
    pub source_class: SourceClass,
    /// Runtime class of the authored artifact.
    pub runtime_class: RuntimeClass,
    /// Host/ABI execution locus.
    pub host_abi: HostAbiClass,
    /// Signing/provenance state of the authored artifact.
    pub signature_state: SignatureState,
    /// Workspace origin of the authored artifact.
    pub workspace_origin: WorkspaceOrigin,
    /// Registry-binding decision for the authored artifact.
    pub registry_binding: BindPublishedIdentity,
    /// Trust posture the author requests, before the gate caps it.
    pub declared_trust_posture: TrustPosture,
    /// Recomputed effective trust posture; must equal the recomputed value.
    pub effective_trust_posture: TrustPosture,
    /// The end-user install support claim this row advertises (from the install lane).
    pub end_user_install_support_class: SupportClass,
    /// The author-side support class the row wants to back.
    pub claimed_author_support_class: SupportClass,
    /// Recomputed effective author support class; must equal the recomputed value.
    pub effective_author_support_class: SupportClass,
    /// Author publish-gate verdict aggregated from the publish-preview lane.
    pub author_publish_readiness: PublishReadiness,
    /// Evidence freshness of the qualifying author drill results.
    pub evidence_freshness: EvidenceFreshness,
    /// Opaque ref to the owner accountable for the row (empty when unowned).
    #[serde(default)]
    pub owner_ref: String,
    /// Per-lane author drill evidence; one entry for every [`AuthorCertificationLane`].
    pub lane_evidence: Vec<AuthorLaneEvidence>,
    /// Reviewer-facing caveats disclosed with the row.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Recomputed certification signals; must equal the recomputed set.
    #[serde(default)]
    pub certification_signals: Vec<AuthorCertificationSignal>,
    /// Recomputed certification disposition; must equal the recomputed value.
    pub certification_disposition: AuthorCertificationDisposition,
    /// Recomputed downgrade path; must equal the recomputed value.
    pub downgrade_path: AuthorDowngradePath,
    /// Opaque ref to the requalification steps to restore a narrowed claim.
    #[serde(default)]
    pub requalification_ref: String,
    /// Ref binding this entry into release evidence.
    pub release_evidence_ref: String,
    /// Ref binding this entry into support and authoring surfaces.
    pub support_export_ref: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl AuthorCertificationEntry {
    /// The trust posture the gate lets this family publish.
    ///
    /// Lowers the author's declared posture to the weakest ceiling implied by the
    /// signing state, the workspace origin, and the registry binding, so a
    /// locally-built, side-loaded, or pending-rebind artifact can never inherit a
    /// verified-publisher or enterprise-approved badge.
    pub fn computed_effective_trust_posture(&self) -> TrustPosture {
        self.declared_trust_posture
            .min(self.signature_state.trust_ceiling())
            .min(self.workspace_origin.trust_ceiling())
            .min(self.registry_binding.trust_ceiling())
    }

    /// The author-side support ceiling, before the install claim and disposition caps.
    ///
    /// This is the weakest of the claimed author class, the source-class ceiling, the
    /// evidence-freshness ceiling, the effective-trust-posture ceiling, and the author
    /// publish-readiness ceiling. It is computed without the disposition so the
    /// [`AuthorCertificationSignal::AuthorClaimBelowInstallClaim`] signal stays a pure
    /// function of the entry's facts.
    pub fn author_side_support_ceiling(&self) -> SupportClass {
        self.claimed_author_support_class
            .min(self.source_class.support_ceiling())
            .min(self.evidence_freshness.support_ceiling())
            .min(trust_posture_support_ceiling(
                self.computed_effective_trust_posture(),
            ))
            .min(publish_readiness_support_ceiling(
                self.author_publish_readiness,
            ))
    }

    /// Whether the source/origin/binding facts structurally cap the trust posture below
    /// the declared posture.
    pub fn trust_capped_below_declared(&self) -> bool {
        self.computed_effective_trust_posture().rank() < self.declared_trust_posture.rank()
    }

    /// Whether the author-side ceiling lands below the end-user install claim.
    pub fn author_claim_below_install(&self) -> bool {
        self.author_side_support_ceiling().rank() < self.end_user_install_support_class.rank()
    }

    /// Whether any lane carries the given evidence state.
    pub fn has_lane_state(&self, state: AuthorLaneState) -> bool {
        self.lane_evidence.iter().any(|l| l.state == state)
    }

    /// The certification signals recomputed from this entry's facts, in canonical order.
    pub fn computed_certification_signals(&self) -> Vec<AuthorCertificationSignal> {
        AuthorCertificationSignal::ALL
            .into_iter()
            .filter(|signal| self.signal_detected(*signal))
            .collect()
    }

    fn signal_detected(&self, signal: AuthorCertificationSignal) -> bool {
        match signal {
            AuthorCertificationSignal::TrustCappedBelowPublishGate => {
                self.trust_capped_below_declared()
            }
            AuthorCertificationSignal::LaneWarnings => {
                self.has_lane_state(AuthorLaneState::WarningsDisclosed)
            }
            AuthorCertificationSignal::PublishGateNotClear => {
                self.author_publish_readiness == PublishReadiness::PublishableWithWarnings
            }
            AuthorCertificationSignal::LaneFreshReviewRequired => {
                self.has_lane_state(AuthorLaneState::FreshReviewRequired)
            }
            AuthorCertificationSignal::LaneStale => self.has_lane_state(AuthorLaneState::Stale),
            AuthorCertificationSignal::EvidenceNotCurrent => !self.evidence_freshness.is_current(),
            AuthorCertificationSignal::AuthorClaimBelowInstallClaim => {
                self.author_claim_below_install()
            }
            AuthorCertificationSignal::OwnerMissing => self.owner_ref.trim().is_empty(),
            AuthorCertificationSignal::LaneMissing => self.has_lane_state(AuthorLaneState::Missing),
            AuthorCertificationSignal::LaneFailed => self.has_lane_state(AuthorLaneState::Failed),
            AuthorCertificationSignal::PublishGateBlocked => {
                self.author_publish_readiness == PublishReadiness::BlockedFromPublish
            }
            AuthorCertificationSignal::QuarantineHold => {
                self.author_publish_readiness == PublishReadiness::WithheldQuarantined
            }
        }
    }

    /// The certification disposition recomputed from this entry's facts.
    pub fn computed_certification_disposition(&self) -> AuthorCertificationDisposition {
        self.computed_certification_signals().into_iter().fold(
            AuthorCertificationDisposition::Certified,
            |disposition, signal| disposition.widen(signal.min_disposition()),
        )
    }

    /// The effective author support class recomputed from this entry's facts.
    ///
    /// The effective class is forced to [`SupportClass::Unsupported`] when the entry is
    /// [`AuthorCertificationDisposition::Uncertified`]; otherwise it is the weakest of the
    /// end-user install claim, the author-side ceiling, and the disposition ceiling, so a
    /// weaker author lane always narrows the marketed claim rather than inheriting it.
    pub fn computed_effective_author_support_class(&self) -> SupportClass {
        if self.computed_certification_disposition() == AuthorCertificationDisposition::Uncertified
        {
            return SupportClass::Unsupported;
        }
        self.end_user_install_support_class
            .min(self.author_side_support_ceiling())
            .min(self.computed_certification_disposition().support_ceiling())
    }

    /// The downgrade path recomputed from this entry's facts.
    pub fn computed_downgrade_path(&self) -> AuthorDowngradePath {
        let to = self.computed_effective_author_support_class();
        AuthorDowngradePath {
            applied: to.rank() < self.end_user_install_support_class.rank(),
            from_support_class: self.end_user_install_support_class,
            to_support_class: to,
            effective_trust_posture: self.computed_effective_trust_posture(),
            signals: self.computed_certification_signals(),
            requalification_ref: self.requalification_ref.clone(),
        }
    }

    /// Whether the entry carries every author-lane ref a positive claim requires.
    ///
    /// A claim must name an owner, link the author matrix and install certification rows,
    /// and not be uncertified, so first-party or trusted-machine status never implies an
    /// author-lane claim alone.
    pub fn is_evidence_backed(&self) -> bool {
        !self.owner_ref.trim().is_empty()
            && !self.author_matrix_ref.trim().is_empty()
            && !self.install_certification_ref.trim().is_empty()
            && !self.has_lane_state(AuthorLaneState::Missing)
            && !self.has_lane_state(AuthorLaneState::Failed)
            && self.computed_certification_disposition()
                != AuthorCertificationDisposition::Uncertified
    }

    /// Whether this family publishes its authored artifact as local-only.
    pub fn is_local_only(&self) -> bool {
        self.computed_effective_trust_posture() == TrustPosture::UnsignedLocalOnly
    }

    /// Whether the stored signals, disposition, effective trust, effective support, and
    /// downgrade path agree with the recomputed values.
    pub fn is_consistent(&self) -> bool {
        self.certification_signals == self.computed_certification_signals()
            && self.certification_disposition == self.computed_certification_disposition()
            && self.effective_trust_posture == self.computed_effective_trust_posture()
            && self.effective_author_support_class == self.computed_effective_author_support_class()
            && self.downgrade_path == self.computed_downgrade_path()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AuthorCertificationSummary {
    /// Total entries.
    pub total_entries: usize,
    /// Entries that are certified.
    pub certified_entries: usize,
    /// Entries that are conditionally certified.
    pub conditionally_certified_entries: usize,
    /// Entries that are downgraded.
    pub downgraded_entries: usize,
    /// Entries that are uncertified.
    pub uncertified_entries: usize,
    /// Entries whose effective support class is narrowed below the install claim.
    pub entries_with_downgrade_applied: usize,
    /// Entries that publish their authored artifact as local-only.
    pub local_only_trust_entries: usize,
    /// Entries whose effective support class is fully supported.
    pub fully_supported_entries: usize,
    /// Entries whose effective support class is unsupported.
    pub unsupported_entries: usize,
    /// Distinct package kinds across entries.
    pub distinct_package_kinds: usize,
    /// Distinct source classes across entries.
    pub distinct_source_classes: usize,
    /// Total disclosed caveats across all entries.
    pub total_caveats: usize,
    /// Total lane evidence records across all entries.
    pub total_lane_evidence: usize,
}

/// A machine-readable certification-index row projected from an entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorCertificationIndexRow {
    /// Entry id.
    pub entry_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Source-class token.
    pub source_class: String,
    /// Signing-state token.
    pub signature_state: String,
    /// Workspace-origin token.
    pub workspace_origin: String,
    /// Registry-binding token.
    pub registry_binding: String,
    /// Declared trust-posture token.
    pub declared_trust_posture: String,
    /// Effective trust-posture token.
    pub effective_trust_posture: String,
    /// End-user install support-class token.
    pub end_user_install_support_class: String,
    /// Effective author support-class token.
    pub effective_author_support_class: String,
    /// Author publish-readiness token.
    pub author_publish_readiness: String,
    /// Certification-disposition token.
    pub certification_disposition: String,
    /// Certification-signal tokens.
    pub certification_signals: Vec<String>,
    /// Whether the effective support class is narrowed below the install claim.
    pub downgrade_applied: bool,
    /// Whether the family publishes its authored artifact as local-only.
    pub local_only: bool,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Owner ref.
    pub owner_ref: String,
    /// Author-matrix ref.
    pub author_matrix_ref: String,
    /// Install-certification ref.
    pub install_certification_ref: String,
    /// Release-evidence ref.
    pub release_evidence_ref: String,
    /// Support-export ref.
    pub support_export_ref: String,
    /// Whether the entry carries every linkage a claim requires.
    pub evidence_backed: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A flat downgrade-report row for issue reports and release evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorDowngradeReportRow {
    /// Entry id the downgrade belongs to.
    pub entry_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Source-class token.
    pub source_class: String,
    /// Support class the row started from.
    pub from_support_class: String,
    /// Support class the author lane narrowed to.
    pub to_support_class: String,
    /// Effective trust-posture token.
    pub effective_trust_posture: String,
    /// Signal tokens that explain the narrowing.
    pub signals: Vec<String>,
    /// Opaque requalification ref.
    pub requalification_ref: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AuthorCertificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Machine-readable certification index.
    pub index_rows: Vec<M5AuthorCertificationIndexRow>,
    /// Flat downgrade report across every narrowed entry.
    pub downgrade_report: Vec<M5AuthorDowngradeReportRow>,
    /// Whether every entry is recomputation-consistent.
    pub all_entries_consistent: bool,
    /// Entries that are certified.
    pub certified_count: usize,
    /// Entries that are downgraded.
    pub downgraded_count: usize,
    /// Entries that are uncertified.
    pub uncertified_count: usize,
    /// Entries whose effective support class is narrowed below the install claim.
    pub downgrade_applied_count: usize,
    /// Entries that publish their authored artifact as local-only.
    pub local_only_count: usize,
}

/// The typed M5 author-certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5AuthorCertification {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Closed package-kind vocabulary (reused from the governance matrix).
    pub package_kinds: Vec<ArtifactFamily>,
    /// Closed source-class vocabulary (reused from the install certification).
    pub source_classes: Vec<SourceClass>,
    /// Closed runtime-class vocabulary (reused from the author matrix).
    pub runtime_classes: Vec<RuntimeClass>,
    /// Closed host/ABI vocabulary (reused from the author matrix).
    pub host_abi_classes: Vec<HostAbiClass>,
    /// Closed signing-state vocabulary (reused from the author matrix).
    pub signature_states: Vec<SignatureState>,
    /// Closed workspace-origin vocabulary (reused from the workspace strip).
    pub workspace_origins: Vec<WorkspaceOrigin>,
    /// Closed registry-binding vocabulary (reused from the anti-abuse board).
    pub registry_bindings: Vec<BindPublishedIdentity>,
    /// Closed trust-posture vocabulary (reused from the author matrix).
    pub trust_postures: Vec<TrustPosture>,
    /// Closed support-class vocabulary (reused from the governance matrix).
    pub support_classes: Vec<SupportClass>,
    /// Closed evidence-freshness vocabulary (reused from the governance matrix).
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed publish-readiness vocabulary (reused from the author matrix).
    pub publish_readiness_states: Vec<PublishReadiness>,
    /// Closed author-certification-lane vocabulary.
    pub author_certification_lanes: Vec<AuthorCertificationLane>,
    /// Closed author-lane-state vocabulary.
    pub author_lane_states: Vec<AuthorLaneState>,
    /// Closed author-certification-signal vocabulary.
    pub author_certification_signals: Vec<AuthorCertificationSignal>,
    /// Closed author-certification-disposition vocabulary.
    pub author_certification_dispositions: Vec<AuthorCertificationDisposition>,
    /// Certification entries, one per marketed ecosystem-authoring row.
    #[serde(default)]
    pub entries: Vec<AuthorCertificationEntry>,
    /// Summary counts.
    pub summary: M5AuthorCertificationSummary,
}

impl M5AuthorCertification {
    /// Returns the entry with the given id.
    pub fn entry(&self, entry_id: &str) -> Option<&AuthorCertificationEntry> {
        self.entries.iter().find(|e| e.entry_id == entry_id)
    }

    /// Recomputes the summary block from the entries.
    pub fn computed_summary(&self) -> M5AuthorCertificationSummary {
        let count_disposition = |d: AuthorCertificationDisposition| {
            self.entries
                .iter()
                .filter(|e| e.certification_disposition == d)
                .count()
        };
        let count_support = |s: SupportClass| {
            self.entries
                .iter()
                .filter(|e| e.effective_author_support_class == s)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.entries.iter().map(|e| e.package_kind).collect();
        let source_classes: BTreeSet<SourceClass> =
            self.entries.iter().map(|e| e.source_class).collect();
        M5AuthorCertificationSummary {
            total_entries: self.entries.len(),
            certified_entries: count_disposition(AuthorCertificationDisposition::Certified),
            conditionally_certified_entries: count_disposition(
                AuthorCertificationDisposition::ConditionallyCertified,
            ),
            downgraded_entries: count_disposition(AuthorCertificationDisposition::Downgraded),
            uncertified_entries: count_disposition(AuthorCertificationDisposition::Uncertified),
            entries_with_downgrade_applied: self
                .entries
                .iter()
                .filter(|e| e.downgrade_path.applied)
                .count(),
            local_only_trust_entries: self.entries.iter().filter(|e| e.is_local_only()).count(),
            fully_supported_entries: count_support(SupportClass::FullySupported),
            unsupported_entries: count_support(SupportClass::Unsupported),
            distinct_package_kinds: package_kinds.len(),
            distinct_source_classes: source_classes.len(),
            total_caveats: self.entries.iter().map(|e| e.caveats.len()).sum(),
            total_lane_evidence: self.entries.iter().map(|e| e.lane_evidence.len()).sum(),
        }
    }

    /// Whether every entry agrees with its recomputation.
    pub fn all_records_consistent(&self) -> bool {
        self.entries
            .iter()
            .all(AuthorCertificationEntry::is_consistent)
    }

    /// Produces an export projection that downstream surfaces — local authoring surfaces,
    /// marketplace badges, diagnostics, support, and release evidence — render instead of
    /// restating certification, trust, support, and downgrade status by hand.
    pub fn export_projection(&self) -> M5AuthorCertificationExportProjection {
        let index_rows = self
            .entries
            .iter()
            .map(|e| M5AuthorCertificationIndexRow {
                entry_id: e.entry_id.clone(),
                package_kind: e.package_kind.as_str().to_owned(),
                source_class: e.source_class.as_str().to_owned(),
                signature_state: e.signature_state.as_str().to_owned(),
                workspace_origin: e.workspace_origin.as_str().to_owned(),
                registry_binding: e.registry_binding.as_str().to_owned(),
                declared_trust_posture: e.declared_trust_posture.as_str().to_owned(),
                effective_trust_posture: e.effective_trust_posture.as_str().to_owned(),
                end_user_install_support_class: e.end_user_install_support_class.as_str().to_owned(),
                effective_author_support_class: e
                    .effective_author_support_class
                    .as_str()
                    .to_owned(),
                author_publish_readiness: e.author_publish_readiness.as_str().to_owned(),
                certification_disposition: e.certification_disposition.as_str().to_owned(),
                certification_signals: e
                    .certification_signals
                    .iter()
                    .map(|s| s.as_str().to_owned())
                    .collect(),
                downgrade_applied: e.downgrade_path.applied,
                local_only: e.is_local_only(),
                evidence_freshness: e.evidence_freshness.as_str().to_owned(),
                owner_ref: e.owner_ref.clone(),
                author_matrix_ref: e.author_matrix_ref.clone(),
                install_certification_ref: e.install_certification_ref.clone(),
                release_evidence_ref: e.release_evidence_ref.clone(),
                support_export_ref: e.support_export_ref.clone(),
                evidence_backed: e.is_evidence_backed(),
                summary: format!(
                    "{}: source {}, signing {}, origin {}, declared {}, published {}, install claim {}, author claim {}, disposition {}",
                    e.package_kind.as_str(),
                    e.source_class.as_str(),
                    e.signature_state.as_str(),
                    e.workspace_origin.as_str(),
                    e.declared_trust_posture.as_str(),
                    e.effective_trust_posture.as_str(),
                    e.end_user_install_support_class.as_str(),
                    e.effective_author_support_class.as_str(),
                    e.certification_disposition.as_str(),
                ),
            })
            .collect();
        let downgrade_report = self
            .entries
            .iter()
            .filter(|e| e.downgrade_path.applied)
            .map(|e| M5AuthorDowngradeReportRow {
                entry_id: e.entry_id.clone(),
                package_kind: e.package_kind.as_str().to_owned(),
                source_class: e.source_class.as_str().to_owned(),
                from_support_class: e.downgrade_path.from_support_class.as_str().to_owned(),
                to_support_class: e.downgrade_path.to_support_class.as_str().to_owned(),
                effective_trust_posture: e
                    .downgrade_path
                    .effective_trust_posture
                    .as_str()
                    .to_owned(),
                signals: e
                    .downgrade_path
                    .signals
                    .iter()
                    .map(|s| s.as_str().to_owned())
                    .collect(),
                requalification_ref: e.downgrade_path.requalification_ref.clone(),
            })
            .collect();
        M5AuthorCertificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            index_rows,
            downgrade_report,
            all_entries_consistent: self.all_records_consistent(),
            certified_count: self
                .entries
                .iter()
                .filter(|e| {
                    e.certification_disposition == AuthorCertificationDisposition::Certified
                })
                .count(),
            downgraded_count: self
                .entries
                .iter()
                .filter(|e| {
                    e.certification_disposition == AuthorCertificationDisposition::Downgraded
                })
                .count(),
            uncertified_count: self
                .entries
                .iter()
                .filter(|e| {
                    e.certification_disposition == AuthorCertificationDisposition::Uncertified
                })
                .count(),
            downgrade_applied_count: self
                .entries
                .iter()
                .filter(|e| e.downgrade_path.applied)
                .count(),
            local_only_count: self.entries.iter().filter(|e| e.is_local_only()).count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5AuthorCertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen = BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.entry_id.clone()) {
                violations.push(M5AuthorCertificationViolation::DuplicateEntryId {
                    entry_id: entry.entry_id.clone(),
                });
            }
            self.validate_entry(entry, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5AuthorCertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5AuthorCertificationViolation>) {
        if self.schema_version != M5_AUTHOR_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5AuthorCertificationViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_AUTHOR_CERTIFICATION_RECORD_KIND {
            violations.push(M5AuthorCertificationViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(M5AuthorCertificationViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "package_kinds",
                self.package_kinds == ArtifactFamily::ALL.to_vec(),
            ),
            (
                "source_classes",
                self.source_classes == SourceClass::ALL.to_vec(),
            ),
            (
                "runtime_classes",
                self.runtime_classes == RuntimeClass::ALL.to_vec(),
            ),
            (
                "host_abi_classes",
                self.host_abi_classes == HostAbiClass::ALL.to_vec(),
            ),
            (
                "signature_states",
                self.signature_states == SignatureState::ALL.to_vec(),
            ),
            (
                "workspace_origins",
                self.workspace_origins == WorkspaceOrigin::ALL.to_vec(),
            ),
            (
                "registry_bindings",
                self.registry_bindings == BindPublishedIdentity::ALL.to_vec(),
            ),
            (
                "trust_postures",
                self.trust_postures == TrustPosture::ALL.to_vec(),
            ),
            (
                "support_classes",
                self.support_classes == SupportClass::ALL.to_vec(),
            ),
            (
                "evidence_freshness_classes",
                self.evidence_freshness_classes == EvidenceFreshness::ALL.to_vec(),
            ),
            (
                "publish_readiness_states",
                self.publish_readiness_states == PublishReadiness::ALL.to_vec(),
            ),
            (
                "author_certification_lanes",
                self.author_certification_lanes == AuthorCertificationLane::ALL.to_vec(),
            ),
            (
                "author_lane_states",
                self.author_lane_states == AuthorLaneState::ALL.to_vec(),
            ),
            (
                "author_certification_signals",
                self.author_certification_signals == AuthorCertificationSignal::ALL.to_vec(),
            ),
            (
                "author_certification_dispositions",
                self.author_certification_dispositions
                    == AuthorCertificationDisposition::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations.push(M5AuthorCertificationViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_entry(
        &self,
        entry: &AuthorCertificationEntry,
        violations: &mut Vec<M5AuthorCertificationViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &entry.entry_id),
            ("display_label", &entry.display_label),
            ("author_matrix_ref", &entry.author_matrix_ref),
            (
                "install_certification_ref",
                &entry.install_certification_ref,
            ),
            ("governance_family_ref", &entry.governance_family_ref),
            ("release_evidence_ref", &entry.release_evidence_ref),
            ("support_export_ref", &entry.support_export_ref),
            ("summary", &entry.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5AuthorCertificationViolation::EmptyField {
                    id: entry.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every author drill lane must appear exactly once, so a row cannot be certified
        // by running a subset of the author drills.
        let mut seen_lanes = BTreeSet::new();
        for evidence in &entry.lane_evidence {
            if evidence.evidence_ref.trim().is_empty() || evidence.summary.trim().is_empty() {
                violations.push(M5AuthorCertificationViolation::EmptyLaneField {
                    id: entry.entry_id.clone(),
                    lane: evidence.lane.as_str(),
                });
            }
            if !seen_lanes.insert(evidence.lane) {
                violations.push(M5AuthorCertificationViolation::DuplicateLane {
                    id: entry.entry_id.clone(),
                    lane: evidence.lane.as_str(),
                });
            }
        }
        for lane in AuthorCertificationLane::ALL {
            if !seen_lanes.contains(&lane) {
                violations.push(M5AuthorCertificationViolation::MissingLane {
                    id: entry.entry_id.clone(),
                    lane: lane.as_str(),
                });
            }
        }

        let mut seen_signals = BTreeSet::new();
        for signal in &entry.certification_signals {
            if !seen_signals.insert(*signal) {
                violations.push(
                    M5AuthorCertificationViolation::DuplicateCertificationSignal {
                        id: entry.entry_id.clone(),
                        signal: signal.as_str(),
                    },
                );
            }
        }

        // The published signals must equal the recomputed set, so a narrowing can never
        // be asserted or hidden by hand.
        if entry.certification_signals != entry.computed_certification_signals() {
            violations.push(
                M5AuthorCertificationViolation::CertificationSignalsMismatch {
                    id: entry.entry_id.clone(),
                },
            );
        }

        // The published disposition must equal the recomputed disposition.
        let disposition = entry.computed_certification_disposition();
        if entry.certification_disposition != disposition {
            violations.push(
                M5AuthorCertificationViolation::CertificationDispositionMismatch {
                    id: entry.entry_id.clone(),
                    stored: entry.certification_disposition.as_str(),
                    computed: disposition.as_str(),
                },
            );
        }

        // The published trust posture must equal the gate's recomputed posture, so a
        // family can never publish a stronger badge than its signing/origin/binding
        // facts support.
        let trust = entry.computed_effective_trust_posture();
        if entry.effective_trust_posture != trust {
            violations.push(M5AuthorCertificationViolation::OverstatedTrustPosture {
                id: entry.entry_id.clone(),
                published: entry.effective_trust_posture.as_str(),
                computed: trust.as_str(),
            });
        }

        // Non-inheritance: a local-dev, side-loaded, unsigned, revoked, or pending-rebind
        // artifact must publish as local-only and may never inherit a trusted badge.
        let structurally_local = entry.signature_state.is_local_or_untrusted()
            || entry.workspace_origin.is_local_authored()
            || matches!(
                entry.registry_binding,
                BindPublishedIdentity::StayLocal | BindPublishedIdentity::BindReviewRequired
            );
        if structurally_local && entry.effective_trust_posture != TrustPosture::UnsignedLocalOnly {
            violations.push(
                M5AuthorCertificationViolation::LocalArtifactInheritedTrust {
                    id: entry.entry_id.clone(),
                    published: entry.effective_trust_posture.as_str(),
                },
            );
        }

        // The published effective support class must equal the recomputed value.
        let effective = entry.computed_effective_author_support_class();
        if entry.effective_author_support_class != effective {
            violations.push(M5AuthorCertificationViolation::EffectiveSupportMismatch {
                id: entry.entry_id.clone(),
                stored: entry.effective_author_support_class.as_str(),
                computed: effective.as_str(),
            });
        }

        // The published downgrade path must equal the recomputed path.
        if entry.downgrade_path != entry.computed_downgrade_path() {
            violations.push(M5AuthorCertificationViolation::DowngradePathMismatch {
                id: entry.entry_id.clone(),
            });
        }

        // A narrowed claim must name how to recover.
        if entry.downgrade_path.applied && entry.requalification_ref.trim().is_empty() {
            violations.push(
                M5AuthorCertificationViolation::DowngradeWithoutRequalification {
                    id: entry.entry_id.clone(),
                },
            );
        }

        // The author claim must never exceed the end-user install claim it guards: the
        // certification narrows the marketed row, it never widens it.
        if entry.effective_author_support_class.rank() > entry.end_user_install_support_class.rank()
        {
            violations.push(
                M5AuthorCertificationViolation::AuthorClaimExceedsInstallClaim {
                    id: entry.entry_id.clone(),
                    author: entry.effective_author_support_class.as_str(),
                    install: entry.end_user_install_support_class.as_str(),
                },
            );
        }

        // The support guardrail: any positive effective support claim must be
        // evidence-backed and not uncertified, so first-party or trusted-machine status
        // never implies an author-lane claim alone.
        if entry.effective_author_support_class != SupportClass::Unsupported
            && (!entry.is_evidence_backed()
                || entry.certification_disposition == AuthorCertificationDisposition::Uncertified)
        {
            violations.push(
                M5AuthorCertificationViolation::SupportClaimedWithoutEvidence {
                    id: entry.entry_id.clone(),
                },
            );
        }
    }
}

/// A validation violation for the M5 author-certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5AuthorCertificationViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Entry or packet-envelope id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A required lane-evidence field is empty.
    EmptyLaneField {
        /// Entry id.
        id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// An entry id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// An entry lists an author drill lane more than once.
    DuplicateLane {
        /// Entry id.
        id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// An entry is missing a required author drill lane.
    MissingLane {
        /// Entry id.
        id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// An entry lists a certification signal more than once.
    DuplicateCertificationSignal {
        /// Entry id.
        id: String,
        /// Signal token.
        signal: &'static str,
    },
    /// An entry's certification signals disagree with the recomputed set.
    CertificationSignalsMismatch {
        /// Entry id.
        id: String,
    },
    /// An entry's stored disposition disagrees with the recomputed value.
    CertificationDispositionMismatch {
        /// Entry id.
        id: String,
        /// Stored disposition token.
        stored: &'static str,
        /// Recomputed disposition token.
        computed: &'static str,
    },
    /// A family publishes a trust posture beyond what its facts support.
    OverstatedTrustPosture {
        /// Entry id.
        id: String,
        /// Published trust-posture token.
        published: &'static str,
        /// Computed effective trust-posture token.
        computed: &'static str,
    },
    /// A local/side-loaded/pending-rebind artifact inherited a trusted publisher badge.
    LocalArtifactInheritedTrust {
        /// Entry id.
        id: String,
        /// Published trust-posture token.
        published: &'static str,
    },
    /// An entry's stored effective support disagrees with the recomputed value.
    EffectiveSupportMismatch {
        /// Entry id.
        id: String,
        /// Stored support token.
        stored: &'static str,
        /// Recomputed support token.
        computed: &'static str,
    },
    /// An entry's stored downgrade path disagrees with the recomputed value.
    DowngradePathMismatch {
        /// Entry id.
        id: String,
    },
    /// A narrowed entry does not name a requalification path.
    DowngradeWithoutRequalification {
        /// Entry id.
        id: String,
    },
    /// An entry's author claim exceeds the end-user install claim it guards.
    AuthorClaimExceedsInstallClaim {
        /// Entry id.
        id: String,
        /// Author effective support token.
        author: &'static str,
        /// Install support-claim token.
        install: &'static str,
    },
    /// An entry publishes a support claim without evidence or certification.
    SupportClaimedWithoutEvidence {
        /// Entry id.
        id: String,
    },
    /// The summary counts disagree with the entries.
    SummaryMismatch,
}

impl fmt::Display for M5AuthorCertificationViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::EmptyLaneField { id, lane } => {
                write!(f, "entry {id} has a {lane} lane with an empty field")
            }
            Self::DuplicateEntryId { entry_id } => {
                write!(f, "duplicate entry id {entry_id}")
            }
            Self::DuplicateLane { id, lane } => {
                write!(f, "entry {id} repeats author drill lane {lane}")
            }
            Self::MissingLane { id, lane } => {
                write!(f, "entry {id} is missing author drill lane {lane}")
            }
            Self::DuplicateCertificationSignal { id, signal } => {
                write!(f, "entry {id} repeats certification signal {signal}")
            }
            Self::CertificationSignalsMismatch { id } => {
                write!(
                    f,
                    "entry {id} certification signals disagree with the recomputed set"
                )
            }
            Self::CertificationDispositionMismatch {
                id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "entry {id} publishes disposition {stored} but the recomputed disposition is {computed}"
                )
            }
            Self::OverstatedTrustPosture {
                id,
                published,
                computed,
            } => {
                write!(
                    f,
                    "entry {id} publishes trust posture {published} but the gate computes {computed}"
                )
            }
            Self::LocalArtifactInheritedTrust { id, published } => {
                write!(
                    f,
                    "entry {id} is locally authored but publishes {published}; local artifacts must publish unsigned_local_only"
                )
            }
            Self::EffectiveSupportMismatch {
                id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "entry {id} publishes effective support {stored} but the recomputed value is {computed}"
                )
            }
            Self::DowngradePathMismatch { id } => {
                write!(
                    f,
                    "entry {id} downgrade path disagrees with the recomputed path"
                )
            }
            Self::DowngradeWithoutRequalification { id } => {
                write!(
                    f,
                    "entry {id} is narrowed but names no requalification path"
                )
            }
            Self::AuthorClaimExceedsInstallClaim {
                id,
                author,
                install,
            } => {
                write!(
                    f,
                    "entry {id} author claim {author} exceeds the install claim {install} it guards"
                )
            }
            Self::SupportClaimedWithoutEvidence { id } => {
                write!(
                    f,
                    "entry {id} publishes an author-lane support claim without an owned, linked, certified row"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the entries")
            }
        }
    }
}

impl Error for M5AuthorCertificationViolation {}

/// Loads the embedded M5 author-certification packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5AuthorCertification`].
pub fn current_m5_author_certification() -> Result<M5AuthorCertification, serde_json::Error> {
    serde_json::from_str(M5_AUTHOR_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
