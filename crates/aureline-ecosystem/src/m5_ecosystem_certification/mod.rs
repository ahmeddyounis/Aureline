//! Canonical M5 ecosystem certification, qualification automation, and downgrade
//! paths — the aggregator that decides which marketed M5 ecosystem row may publish.
//!
//! Where the
//! [`install-governance matrix`](crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix)
//! freezes one governance row per marketed M5 artifact family and the
//! [`m5_conformance_and_validators`](crate::m5_conformance_and_validators) module proves
//! each family's *support claim* is backed by a current, owned, evidence-linked
//! conformance scorecard, this module is the qualification layer that rolls every
//! ecosystem drill into one decision. A [`CertificationEntry`] aggregates the per-lane
//! evidence each ecosystem drill produces — marketplace information, install review,
//! lifecycle state, compatibility label, permission manifest, activation budget,
//! mirror/private-registry parity, and rollback/quarantine — alongside the family's
//! linked conformance scorecard, owner, evidence freshness, and supported deployment and
//! runtime profiles, and decides whether that row may publish a marketed claim.
//!
//! The decision is honest by construction. The [`QualificationDisposition`] an entry
//! publishes is **not** stored by hand: it is recomputed from the entry's facts as the
//! widest [`QualificationSignal::min_disposition`] over every detected
//! [`QualificationSignal`], and the stored signals, disposition, effective support class,
//! and [`DowngradePath`] must equal that recomputation or validation fails. The lane
//! guardrail rides that recomputation: a conditional lane narrows the row to
//! [`QualificationDisposition::ConditionallyQualified`]; a narrowed or stale lane, stale
//! evidence, or missing supported profiles narrow it to
//! [`QualificationDisposition::Downgraded`]; and a missing or failed lane, a missing
//! owner, or an uncertified conformance scorecard each force
//! [`QualificationDisposition::Disqualified`], whose effective support class collapses to
//! [`SupportClass::Unsupported`]. A [`SourceClass`] that structurally caps the claim — a
//! mirrored-registry, private-registry, bridge-backed, or side-loaded row — narrows the
//! effective support class even when the row is otherwise qualified, so a non-public,
//! non-first-party row can never inherit a broader first-party or public-registry claim.
//!
//! Every entry also carries an explicit [`DowngradePath`] — the exact support class the
//! claim drops to and the opaque requalification ref an owner follows to restore it — so
//! a narrowed row names how to recover instead of silently going green again. The packet
//! exports a certification index and a flat downgrade report through
//! [`M5EcosystemCertification::export_projection`], so release evidence, marketplace
//! badges, docs/help, and support exports all narrow from the same packet rather than
//! parallel spreadsheets.
//!
//! The packet is checked in at `artifacts/ecosystem/m5/m5-ecosystem-certification.json`
//! and embedded here, so this typed consumer and any CI gate agree on every record
//! without a cargo build in CI. The model is metadata-only: every field is a typed state
//! or an opaque ref. It carries no credential bodies, raw provider payloads, signing
//! secrets, or evidence bodies.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix::{
    ArtifactFamily, EvidenceFreshness, RuntimeOrigin, SupportClass,
};

/// Supported M5 ecosystem-certification schema version.
pub const M5_ECOSYSTEM_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const M5_ECOSYSTEM_CERTIFICATION_RECORD_KIND: &str = "m5_ecosystem_certification";

/// Repo-relative path to the checked-in packet.
pub const M5_ECOSYSTEM_CERTIFICATION_PATH: &str =
    "artifacts/ecosystem/m5/m5-ecosystem-certification.json";

/// Embedded checked-in packet JSON.
pub const M5_ECOSYSTEM_CERTIFICATION_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/ecosystem/m5/m5-ecosystem-certification.json"
));

/// The distribution source class of a marketed ecosystem row.
///
/// The source class is a structural cap, not an evidence-driven downgrade: a
/// mirrored-registry, private-registry, bridge-backed, or side-loaded row can never
/// inherit the support ceiling of a first-party, public-registry row, even when its
/// qualification evidence is current. The cap rides through
/// [`SourceClass::support_ceiling`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    /// First-party, public-registry distribution.
    FirstParty,
    /// Partner-published, public-registry distribution.
    Partner,
    /// Community-published, public-registry distribution.
    Community,
    /// Bridge-backed distribution through a compatibility runtime.
    BridgeBacked,
    /// Side-loaded, out-of-registry distribution.
    SideLoaded,
    /// Mirrored public registry (enterprise mirror).
    MirroredRegistry,
    /// Private registry distribution.
    PrivateRegistry,
}

impl SourceClass {
    /// Every source class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FirstParty,
        Self::Partner,
        Self::Community,
        Self::BridgeBacked,
        Self::SideLoaded,
        Self::MirroredRegistry,
        Self::PrivateRegistry,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::Partner => "partner",
            Self::Community => "community",
            Self::BridgeBacked => "bridge_backed",
            Self::SideLoaded => "side_loaded",
            Self::MirroredRegistry => "mirrored_registry",
            Self::PrivateRegistry => "private_registry",
        }
    }

    /// Highest support class this source class permits a row to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::FirstParty | Self::Partner => SupportClass::FullySupported,
            Self::BridgeBacked | Self::MirroredRegistry | Self::PrivateRegistry => {
                SupportClass::BestEffortSupported
            }
            Self::Community => SupportClass::CommunitySupported,
            Self::SideLoaded => SupportClass::Unsupported,
        }
    }
}

/// One ecosystem drill lane whose evidence is aggregated into a certification entry.
///
/// The lanes are exactly the M5 ecosystem drills a marketed row must clear; every entry
/// carries one [`LaneEvidence`] for each lane, so a row cannot be certified by running a
/// subset of the drills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificationLane {
    /// Source-aware marketplace information drill.
    MarketplaceInformation,
    /// Unified install/update review drill.
    InstallReview,
    /// Post-install lifecycle-state drill.
    LifecycleState,
    /// Compatibility-label drill against the install target.
    CompatibilityLabel,
    /// Permission-manifest delta drill.
    PermissionManifest,
    /// Session activation-budget drill.
    ActivationBudget,
    /// Mirror / private-registry parity drill.
    MirrorPrivateRegistry,
    /// Rollback / quarantine recovery drill.
    RollbackQuarantine,
}

impl CertificationLane {
    /// Every certification lane, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::MarketplaceInformation,
        Self::InstallReview,
        Self::LifecycleState,
        Self::CompatibilityLabel,
        Self::PermissionManifest,
        Self::ActivationBudget,
        Self::MirrorPrivateRegistry,
        Self::RollbackQuarantine,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarketplaceInformation => "marketplace_information",
            Self::InstallReview => "install_review",
            Self::LifecycleState => "lifecycle_state",
            Self::CompatibilityLabel => "compatibility_label",
            Self::PermissionManifest => "permission_manifest",
            Self::ActivationBudget => "activation_budget",
            Self::MirrorPrivateRegistry => "mirror_private_registry",
            Self::RollbackQuarantine => "rollback_quarantine",
        }
    }
}

/// The evidence state of one drill lane for a certification entry.
///
/// Ordered weakest-narrowing to strongest by [`LaneEvidenceState::rank`]: a
/// [`LaneEvidenceState::Current`] lane adds no narrowing, while a
/// [`LaneEvidenceState::Failed`] lane disqualifies the row. Each non-current state maps
/// to one [`QualificationSignal`] through [`LaneEvidenceState::narrowing_signal`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneEvidenceState {
    /// Current, passing evidence; no narrowing.
    Current,
    /// Passing with a disclosed condition; narrows to conditionally qualified.
    Conditional,
    /// Evidence shows a narrowed claim; narrows to downgraded.
    Narrowed,
    /// Evidence has lapsed; narrows to downgraded.
    Stale,
    /// No evidence for this lane; disqualifies the row.
    Missing,
    /// The lane drill failed; disqualifies the row.
    Failed,
}

impl LaneEvidenceState {
    /// Every lane evidence state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Current,
        Self::Conditional,
        Self::Narrowed,
        Self::Stale,
        Self::Missing,
        Self::Failed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Conditional => "conditional",
            Self::Narrowed => "narrowed",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::Failed => "failed",
        }
    }

    /// Monotonic rank; higher means a stronger narrowing.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Current => 0,
            Self::Conditional => 1,
            Self::Narrowed => 2,
            Self::Stale => 3,
            Self::Missing => 4,
            Self::Failed => 5,
        }
    }

    /// The qualification signal this lane state contributes, if any.
    pub const fn narrowing_signal(self) -> Option<QualificationSignal> {
        match self {
            Self::Current => None,
            Self::Conditional => Some(QualificationSignal::LaneConditional),
            Self::Narrowed => Some(QualificationSignal::LaneNarrowed),
            Self::Stale => Some(QualificationSignal::LaneStale),
            Self::Missing => Some(QualificationSignal::LaneMissing),
            Self::Failed => Some(QualificationSignal::LaneFailed),
        }
    }
}

/// One drill lane's aggregated evidence for a certification entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LaneEvidence {
    /// Drill lane this evidence covers.
    pub lane: CertificationLane,
    /// Aggregated evidence state for the lane.
    pub state: LaneEvidenceState,
    /// Opaque ref to the drill packet or report this evidence rolls up.
    pub evidence_ref: String,
    /// Reviewer-facing summary of the lane's evidence.
    pub summary: String,
}

/// A qualification signal an entry surfaces.
///
/// Each signal is recomputed from the entry's facts; the entry's stored
/// [`CertificationEntry::qualification_signals`] must equal the recomputed set. Each
/// signal carries a fixed [`QualificationSignal::min_disposition`], so the published
/// [`QualificationDisposition`] is a pure function of which signals fire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationSignal {
    /// The source class structurally caps the claim below the claimed support class.
    SourceClassCapped,
    /// At least one lane is conditionally passing.
    LaneConditional,
    /// At least one lane shows a narrowed claim.
    LaneNarrowed,
    /// At least one lane's evidence has lapsed.
    LaneStale,
    /// The qualifying evidence freshness is not current.
    EvidenceNotCurrent,
    /// No supported deployment or runtime profile is linked.
    SupportedProfilesMissing,
    /// No owner is named for the entry.
    OwnerMissing,
    /// At least one lane has no evidence.
    LaneMissing,
    /// At least one lane drill failed.
    LaneFailed,
    /// The linked conformance scorecard is not certified.
    ConformanceNotCertified,
}

impl QualificationSignal {
    /// Every qualification signal, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::SourceClassCapped,
        Self::LaneConditional,
        Self::LaneNarrowed,
        Self::LaneStale,
        Self::EvidenceNotCurrent,
        Self::SupportedProfilesMissing,
        Self::OwnerMissing,
        Self::LaneMissing,
        Self::LaneFailed,
        Self::ConformanceNotCertified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceClassCapped => "source_class_capped",
            Self::LaneConditional => "lane_conditional",
            Self::LaneNarrowed => "lane_narrowed",
            Self::LaneStale => "lane_stale",
            Self::EvidenceNotCurrent => "evidence_not_current",
            Self::SupportedProfilesMissing => "supported_profiles_missing",
            Self::OwnerMissing => "owner_missing",
            Self::LaneMissing => "lane_missing",
            Self::LaneFailed => "lane_failed",
            Self::ConformanceNotCertified => "conformance_not_certified",
        }
    }

    /// The minimum qualification disposition this signal forces.
    ///
    /// [`Self::SourceClassCapped`] is transparency only — it marks that the source class
    /// narrowed the effective support class without lowering the qualification
    /// disposition. [`Self::LaneConditional`] forces
    /// [`QualificationDisposition::ConditionallyQualified`]. The narrowing signals — a
    /// narrowed or stale lane, stale evidence, or missing supported profiles — force
    /// [`QualificationDisposition::Downgraded`]. The guardrail signals — a missing or
    /// failed lane, a missing owner, or an uncertified conformance scorecard — force
    /// [`QualificationDisposition::Disqualified`].
    pub const fn min_disposition(self) -> QualificationDisposition {
        match self {
            Self::SourceClassCapped => QualificationDisposition::Qualified,
            Self::LaneConditional => QualificationDisposition::ConditionallyQualified,
            Self::LaneNarrowed
            | Self::LaneStale
            | Self::EvidenceNotCurrent
            | Self::SupportedProfilesMissing => QualificationDisposition::Downgraded,
            Self::OwnerMissing
            | Self::LaneMissing
            | Self::LaneFailed
            | Self::ConformanceNotCertified => QualificationDisposition::Disqualified,
        }
    }
}

/// The disposition a certification entry publishes.
///
/// Ordered low-to-high by [`QualificationDisposition::rank`]: a
/// [`QualificationDisposition::Qualified`] entry backs a full claim, and a
/// [`QualificationDisposition::Disqualified`] entry backs no claim at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QualificationDisposition {
    /// No narrowing or guardrail signal applies; the row is current and fully backed.
    Qualified,
    /// A disclosed condition applies; qualified with conditions.
    ConditionallyQualified,
    /// A narrowing signal applies; the claim is narrowed to a lower support tier.
    Downgraded,
    /// A guardrail signal applies; the row backs no claim.
    Disqualified,
}

impl QualificationDisposition {
    /// Every qualification disposition, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Qualified,
        Self::ConditionallyQualified,
        Self::Downgraded,
        Self::Disqualified,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ConditionallyQualified => "conditionally_qualified",
            Self::Downgraded => "downgraded",
            Self::Disqualified => "disqualified",
        }
    }

    /// Monotonic rank; higher means a weaker qualification.
    pub const fn rank(self) -> u8 {
        match self {
            Self::Qualified => 0,
            Self::ConditionallyQualified => 1,
            Self::Downgraded => 2,
            Self::Disqualified => 3,
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

    /// Highest support class this disposition permits a row to publish.
    pub const fn support_ceiling(self) -> SupportClass {
        match self {
            Self::Qualified | Self::ConditionallyQualified => SupportClass::FullySupported,
            Self::Downgraded => SupportClass::CommunitySupported,
            Self::Disqualified => SupportClass::Unsupported,
        }
    }
}

/// The exact downgrade path published with a narrowed certification entry.
///
/// Every entry carries a downgrade path so a narrowed claim names where it landed and
/// how to recover. The path is recomputed from the entry's facts; the stored value must
/// equal the recomputation. [`Self::applied`] is true whenever the effective support
/// class is below the claimed support class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradePath {
    /// Whether the effective support class is narrowed below the claimed class.
    pub applied: bool,
    /// The claimed support class the row started from.
    pub from_support_class: SupportClass,
    /// The effective support class the row narrowed to.
    pub to_support_class: SupportClass,
    /// The qualification signals that explain the narrowing.
    pub signals: Vec<QualificationSignal>,
    /// Opaque ref to the requalification steps an owner follows to restore the claim.
    pub requalification_ref: String,
}

/// A certification entry for one marketed M5 ecosystem row.
///
/// The entry aggregates the per-lane drill evidence and the linked conformance scorecard
/// into one qualification decision. The published signals, disposition, effective support
/// class, and downgrade path are recomputed from the entry's facts and must equal the
/// recomputation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CertificationEntry {
    /// Stable entry id.
    pub entry_id: String,
    /// Human-readable row label.
    pub display_label: String,
    /// Ref to the governance-matrix family this entry resolves through.
    pub governance_family_ref: String,
    /// Ref to the conformance scorecard this entry aggregates.
    pub conformance_scorecard_ref: String,
    /// Package kind / marketed artifact family.
    pub package_kind: ArtifactFamily,
    /// Distribution source class.
    pub source_class: SourceClass,
    /// Runtime origin.
    pub runtime_origin: RuntimeOrigin,
    /// Support class the row wants to publish.
    pub claimed_support_class: SupportClass,
    /// Recomputed effective support class; must equal the recomputed value.
    pub effective_support_class: SupportClass,
    /// Evidence freshness of the qualifying result.
    pub evidence_freshness: EvidenceFreshness,
    /// Whether the linked conformance scorecard is certified or conditionally certified.
    pub conformance_certified: bool,
    /// Opaque ref to the owner accountable for the row (empty when unowned).
    #[serde(default)]
    pub owner_ref: String,
    /// Supported deployment profile refs (empty when missing).
    #[serde(default)]
    pub supported_deployment_profile_refs: Vec<String>,
    /// Supported runtime profile refs (empty when missing).
    #[serde(default)]
    pub supported_runtime_profile_refs: Vec<String>,
    /// Per-lane drill evidence; one entry for every [`CertificationLane`].
    pub lane_evidence: Vec<LaneEvidence>,
    /// Reviewer-facing caveats disclosed with the row.
    #[serde(default)]
    pub caveats: Vec<String>,
    /// Recomputed qualification signals; must equal the recomputed set.
    #[serde(default)]
    pub qualification_signals: Vec<QualificationSignal>,
    /// Recomputed qualification disposition; must equal the recomputed value.
    pub qualification_disposition: QualificationDisposition,
    /// Recomputed downgrade path; must equal the recomputed value.
    pub downgrade_path: DowngradePath,
    /// Opaque ref to the requalification steps to restore a narrowed claim.
    #[serde(default)]
    pub requalification_ref: String,
    /// Ref binding this entry into release evidence.
    pub release_evidence_ref: String,
    /// Ref binding this entry into support and marketplace surfaces.
    pub support_export_ref: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl CertificationEntry {
    /// Whether the entry carries every linkage a positive support claim requires.
    ///
    /// A claim must name an owner, link at least one supported deployment and runtime
    /// profile, link a conformance scorecard, and ride a certified conformance result; a
    /// row that drops any of these can back no positive claim.
    pub fn is_evidence_backed(&self) -> bool {
        !self.owner_ref.trim().is_empty()
            && !self.supported_deployment_profile_refs.is_empty()
            && self
                .supported_deployment_profile_refs
                .iter()
                .all(|r| !r.trim().is_empty())
            && !self.supported_runtime_profile_refs.is_empty()
            && self
                .supported_runtime_profile_refs
                .iter()
                .all(|r| !r.trim().is_empty())
            && !self.conformance_scorecard_ref.trim().is_empty()
            && self.conformance_certified
    }

    /// Whether any lane carries the given evidence state.
    pub fn has_lane_state(&self, state: LaneEvidenceState) -> bool {
        self.lane_evidence.iter().any(|l| l.state == state)
    }

    /// Whether the source class structurally caps the claim below the claimed class.
    pub fn source_class_caps_claim(&self) -> bool {
        self.source_class.support_ceiling().rank() < self.claimed_support_class.rank()
    }

    /// The qualification signals recomputed from this entry's facts, in canonical order.
    pub fn computed_qualification_signals(&self) -> Vec<QualificationSignal> {
        QualificationSignal::ALL
            .into_iter()
            .filter(|signal| self.signal_detected(*signal))
            .collect()
    }

    fn signal_detected(&self, signal: QualificationSignal) -> bool {
        match signal {
            QualificationSignal::SourceClassCapped => self.source_class_caps_claim(),
            QualificationSignal::LaneConditional => {
                self.has_lane_state(LaneEvidenceState::Conditional)
            }
            QualificationSignal::LaneNarrowed => self.has_lane_state(LaneEvidenceState::Narrowed),
            QualificationSignal::LaneStale => self.has_lane_state(LaneEvidenceState::Stale),
            QualificationSignal::EvidenceNotCurrent => !self.evidence_freshness.is_current(),
            QualificationSignal::SupportedProfilesMissing => {
                self.supported_deployment_profile_refs.is_empty()
                    || self.supported_runtime_profile_refs.is_empty()
            }
            QualificationSignal::OwnerMissing => self.owner_ref.trim().is_empty(),
            QualificationSignal::LaneMissing => self.has_lane_state(LaneEvidenceState::Missing),
            QualificationSignal::LaneFailed => self.has_lane_state(LaneEvidenceState::Failed),
            QualificationSignal::ConformanceNotCertified => !self.conformance_certified,
        }
    }

    /// The qualification disposition recomputed from this entry's facts.
    pub fn computed_qualification_disposition(&self) -> QualificationDisposition {
        self.computed_qualification_signals().into_iter().fold(
            QualificationDisposition::Qualified,
            |disposition, signal| disposition.widen(signal.min_disposition()),
        )
    }

    /// The effective support class recomputed from this entry's facts.
    ///
    /// The effective class is forced to [`SupportClass::Unsupported`] when the entry is
    /// [`QualificationDisposition::Disqualified`]; otherwise it is the weakest of the
    /// claimed class, the disposition ceiling, the source-class ceiling, and the evidence
    /// freshness ceiling, so a narrowed disposition or a capped source class always wins.
    pub fn computed_effective_support_class(&self) -> SupportClass {
        if self.computed_qualification_disposition() == QualificationDisposition::Disqualified {
            return SupportClass::Unsupported;
        }
        self.claimed_support_class
            .min(self.computed_qualification_disposition().support_ceiling())
            .min(self.source_class.support_ceiling())
            .min(self.evidence_freshness.support_ceiling())
    }

    /// The downgrade path recomputed from this entry's facts.
    pub fn computed_downgrade_path(&self) -> DowngradePath {
        let to = self.computed_effective_support_class();
        DowngradePath {
            applied: to.rank() < self.claimed_support_class.rank(),
            from_support_class: self.claimed_support_class,
            to_support_class: to,
            signals: self.computed_qualification_signals(),
            requalification_ref: self.requalification_ref.clone(),
        }
    }

    /// Whether the stored signals, disposition, effective support, and downgrade path
    /// agree with the recomputed values.
    pub fn is_consistent(&self) -> bool {
        self.qualification_signals == self.computed_qualification_signals()
            && self.qualification_disposition == self.computed_qualification_disposition()
            && self.effective_support_class == self.computed_effective_support_class()
            && self.downgrade_path == self.computed_downgrade_path()
    }
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5EcosystemCertificationSummary {
    /// Total entries.
    pub total_entries: usize,
    /// Entries that are qualified.
    pub qualified_entries: usize,
    /// Entries that are conditionally qualified.
    pub conditionally_qualified_entries: usize,
    /// Entries that are downgraded.
    pub downgraded_entries: usize,
    /// Entries that are disqualified.
    pub disqualified_entries: usize,
    /// Entries whose effective support class is narrowed below the claimed class.
    pub entries_with_downgrade_applied: usize,
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
pub struct M5CertificationIndexRow {
    /// Entry id.
    pub entry_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Source-class token.
    pub source_class: String,
    /// Runtime-origin token.
    pub runtime_origin: String,
    /// Claimed-support-class token.
    pub claimed_support_class: String,
    /// Effective-support-class token.
    pub effective_support_class: String,
    /// Qualification-disposition token.
    pub qualification_disposition: String,
    /// Qualification-signal tokens.
    pub qualification_signals: Vec<String>,
    /// Whether the effective support class is narrowed below the claimed class.
    pub downgrade_applied: bool,
    /// Evidence-freshness token.
    pub evidence_freshness: String,
    /// Whether the linked conformance scorecard is certified.
    pub conformance_certified: bool,
    /// Owner ref.
    pub owner_ref: String,
    /// Governance-matrix family ref.
    pub governance_family_ref: String,
    /// Conformance-scorecard ref.
    pub conformance_scorecard_ref: String,
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
pub struct M5DowngradeReportRow {
    /// Entry id the downgrade belongs to.
    pub entry_id: String,
    /// Package-kind token.
    pub package_kind: String,
    /// Source-class token.
    pub source_class: String,
    /// Support class the row started from.
    pub from_support_class: String,
    /// Support class the row narrowed to.
    pub to_support_class: String,
    /// Signal tokens that explain the narrowing.
    pub signals: Vec<String>,
    /// Opaque requalification ref.
    pub requalification_ref: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5EcosystemCertificationExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Machine-readable certification index.
    pub index_rows: Vec<M5CertificationIndexRow>,
    /// Flat downgrade report across every narrowed entry.
    pub downgrade_report: Vec<M5DowngradeReportRow>,
    /// Whether every entry is recomputation-consistent.
    pub all_entries_consistent: bool,
    /// Entries that are qualified.
    pub qualified_count: usize,
    /// Entries that are downgraded.
    pub downgraded_count: usize,
    /// Entries that are disqualified.
    pub disqualified_count: usize,
    /// Entries whose effective support class is narrowed below the claimed class.
    pub downgrade_applied_count: usize,
}

/// The typed M5 ecosystem-certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5EcosystemCertification {
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
    /// Closed runtime-origin vocabulary (reused from the governance matrix).
    pub runtime_origins: Vec<RuntimeOrigin>,
    /// Closed support-class vocabulary (reused from the governance matrix).
    pub support_classes: Vec<SupportClass>,
    /// Closed evidence-freshness vocabulary (reused from the governance matrix).
    pub evidence_freshness_classes: Vec<EvidenceFreshness>,
    /// Closed source-class vocabulary.
    pub source_classes: Vec<SourceClass>,
    /// Closed certification-lane vocabulary.
    pub certification_lanes: Vec<CertificationLane>,
    /// Closed lane-evidence-state vocabulary.
    pub lane_evidence_states: Vec<LaneEvidenceState>,
    /// Closed qualification-signal vocabulary.
    pub qualification_signals: Vec<QualificationSignal>,
    /// Closed qualification-disposition vocabulary.
    pub qualification_dispositions: Vec<QualificationDisposition>,
    /// Certification entries, one per marketed ecosystem row.
    #[serde(default)]
    pub entries: Vec<CertificationEntry>,
    /// Summary counts.
    pub summary: M5EcosystemCertificationSummary,
}

impl M5EcosystemCertification {
    /// Returns the entry with the given id.
    pub fn entry(&self, entry_id: &str) -> Option<&CertificationEntry> {
        self.entries.iter().find(|e| e.entry_id == entry_id)
    }

    /// Recomputes the summary block from the entries.
    pub fn computed_summary(&self) -> M5EcosystemCertificationSummary {
        let count_disposition = |d: QualificationDisposition| {
            self.entries
                .iter()
                .filter(|e| e.qualification_disposition == d)
                .count()
        };
        let count_support = |s: SupportClass| {
            self.entries
                .iter()
                .filter(|e| e.effective_support_class == s)
                .count()
        };
        let package_kinds: BTreeSet<ArtifactFamily> =
            self.entries.iter().map(|e| e.package_kind).collect();
        let source_classes: BTreeSet<SourceClass> =
            self.entries.iter().map(|e| e.source_class).collect();
        M5EcosystemCertificationSummary {
            total_entries: self.entries.len(),
            qualified_entries: count_disposition(QualificationDisposition::Qualified),
            conditionally_qualified_entries: count_disposition(
                QualificationDisposition::ConditionallyQualified,
            ),
            downgraded_entries: count_disposition(QualificationDisposition::Downgraded),
            disqualified_entries: count_disposition(QualificationDisposition::Disqualified),
            entries_with_downgrade_applied: self
                .entries
                .iter()
                .filter(|e| e.downgrade_path.applied)
                .count(),
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
        self.entries.iter().all(CertificationEntry::is_consistent)
    }

    /// Produces an export projection that downstream surfaces — release evidence,
    /// marketplace badges, docs/help, and support exports — render instead of restating
    /// qualification, support, and downgrade status by hand.
    pub fn export_projection(&self) -> M5EcosystemCertificationExportProjection {
        let index_rows = self
            .entries
            .iter()
            .map(|e| M5CertificationIndexRow {
                entry_id: e.entry_id.clone(),
                package_kind: e.package_kind.as_str().to_owned(),
                source_class: e.source_class.as_str().to_owned(),
                runtime_origin: e.runtime_origin.as_str().to_owned(),
                claimed_support_class: e.claimed_support_class.as_str().to_owned(),
                effective_support_class: e.effective_support_class.as_str().to_owned(),
                qualification_disposition: e.qualification_disposition.as_str().to_owned(),
                qualification_signals: e
                    .qualification_signals
                    .iter()
                    .map(|s| s.as_str().to_owned())
                    .collect(),
                downgrade_applied: e.downgrade_path.applied,
                evidence_freshness: e.evidence_freshness.as_str().to_owned(),
                conformance_certified: e.conformance_certified,
                owner_ref: e.owner_ref.clone(),
                governance_family_ref: e.governance_family_ref.clone(),
                conformance_scorecard_ref: e.conformance_scorecard_ref.clone(),
                release_evidence_ref: e.release_evidence_ref.clone(),
                support_export_ref: e.support_export_ref.clone(),
                evidence_backed: e.is_evidence_backed(),
                summary: format!(
                    "{}: source {}, claimed {}, effective {}, disposition {}",
                    e.package_kind.as_str(),
                    e.source_class.as_str(),
                    e.claimed_support_class.as_str(),
                    e.effective_support_class.as_str(),
                    e.qualification_disposition.as_str(),
                ),
            })
            .collect();
        let downgrade_report = self
            .entries
            .iter()
            .filter(|e| e.downgrade_path.applied)
            .map(|e| M5DowngradeReportRow {
                entry_id: e.entry_id.clone(),
                package_kind: e.package_kind.as_str().to_owned(),
                source_class: e.source_class.as_str().to_owned(),
                from_support_class: e.downgrade_path.from_support_class.as_str().to_owned(),
                to_support_class: e.downgrade_path.to_support_class.as_str().to_owned(),
                signals: e
                    .downgrade_path
                    .signals
                    .iter()
                    .map(|s| s.as_str().to_owned())
                    .collect(),
                requalification_ref: e.downgrade_path.requalification_ref.clone(),
            })
            .collect();
        M5EcosystemCertificationExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            index_rows,
            downgrade_report,
            all_entries_consistent: self.all_records_consistent(),
            qualified_count: self
                .entries
                .iter()
                .filter(|e| e.qualification_disposition == QualificationDisposition::Qualified)
                .count(),
            downgraded_count: self
                .entries
                .iter()
                .filter(|e| e.qualification_disposition == QualificationDisposition::Downgraded)
                .count(),
            disqualified_count: self
                .entries
                .iter()
                .filter(|e| e.qualification_disposition == QualificationDisposition::Disqualified)
                .count(),
            downgrade_applied_count: self
                .entries
                .iter()
                .filter(|e| e.downgrade_path.applied)
                .count(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<M5EcosystemCertificationViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen = BTreeSet::new();
        for entry in &self.entries {
            if !seen.insert(entry.entry_id.clone()) {
                violations.push(M5EcosystemCertificationViolation::DuplicateEntryId {
                    entry_id: entry.entry_id.clone(),
                });
            }
            self.validate_entry(entry, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(M5EcosystemCertificationViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5EcosystemCertificationViolation>) {
        if self.schema_version != M5_ECOSYSTEM_CERTIFICATION_SCHEMA_VERSION {
            violations.push(
                M5EcosystemCertificationViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != M5_ECOSYSTEM_CERTIFICATION_RECORD_KIND {
            violations.push(M5EcosystemCertificationViolation::UnsupportedRecordKind {
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
                violations.push(M5EcosystemCertificationViolation::EmptyField {
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
                "runtime_origins",
                self.runtime_origins == RuntimeOrigin::ALL.to_vec(),
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
                "source_classes",
                self.source_classes == SourceClass::ALL.to_vec(),
            ),
            (
                "certification_lanes",
                self.certification_lanes == CertificationLane::ALL.to_vec(),
            ),
            (
                "lane_evidence_states",
                self.lane_evidence_states == LaneEvidenceState::ALL.to_vec(),
            ),
            (
                "qualification_signals",
                self.qualification_signals == QualificationSignal::ALL.to_vec(),
            ),
            (
                "qualification_dispositions",
                self.qualification_dispositions == QualificationDisposition::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(M5EcosystemCertificationViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_entry(
        &self,
        entry: &CertificationEntry,
        violations: &mut Vec<M5EcosystemCertificationViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &entry.entry_id),
            ("display_label", &entry.display_label),
            ("governance_family_ref", &entry.governance_family_ref),
            (
                "conformance_scorecard_ref",
                &entry.conformance_scorecard_ref,
            ),
            ("release_evidence_ref", &entry.release_evidence_ref),
            ("support_export_ref", &entry.support_export_ref),
            ("summary", &entry.summary),
        ] {
            if value.trim().is_empty() {
                violations.push(M5EcosystemCertificationViolation::EmptyField {
                    id: entry.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every drill lane must appear exactly once, so a row cannot be certified by
        // running a subset of the ecosystem drills.
        let mut seen_lanes = BTreeSet::new();
        for evidence in &entry.lane_evidence {
            if evidence.evidence_ref.trim().is_empty() || evidence.summary.trim().is_empty() {
                violations.push(M5EcosystemCertificationViolation::EmptyLaneField {
                    id: entry.entry_id.clone(),
                    lane: evidence.lane.as_str(),
                });
            }
            if !seen_lanes.insert(evidence.lane) {
                violations.push(M5EcosystemCertificationViolation::DuplicateLane {
                    id: entry.entry_id.clone(),
                    lane: evidence.lane.as_str(),
                });
            }
        }
        for lane in CertificationLane::ALL {
            if !seen_lanes.contains(&lane) {
                violations.push(M5EcosystemCertificationViolation::MissingLane {
                    id: entry.entry_id.clone(),
                    lane: lane.as_str(),
                });
            }
        }

        let mut seen_signals = BTreeSet::new();
        for signal in &entry.qualification_signals {
            if !seen_signals.insert(*signal) {
                violations.push(
                    M5EcosystemCertificationViolation::DuplicateQualificationSignal {
                        id: entry.entry_id.clone(),
                        signal: signal.as_str(),
                    },
                );
            }
        }

        // The published signals must equal the recomputed set, so a narrowing can never
        // be asserted or hidden by hand.
        if entry.qualification_signals != entry.computed_qualification_signals() {
            violations.push(
                M5EcosystemCertificationViolation::QualificationSignalsMismatch {
                    id: entry.entry_id.clone(),
                },
            );
        }

        // The published disposition must equal the recomputed disposition.
        let disposition = entry.computed_qualification_disposition();
        if entry.qualification_disposition != disposition {
            violations.push(
                M5EcosystemCertificationViolation::QualificationDispositionMismatch {
                    id: entry.entry_id.clone(),
                    stored: entry.qualification_disposition.as_str(),
                    computed: disposition.as_str(),
                },
            );
        }

        // The published effective support class must equal the recomputed value.
        let effective = entry.computed_effective_support_class();
        if entry.effective_support_class != effective {
            violations.push(
                M5EcosystemCertificationViolation::EffectiveSupportMismatch {
                    id: entry.entry_id.clone(),
                    stored: entry.effective_support_class.as_str(),
                    computed: effective.as_str(),
                },
            );
        }

        // The published downgrade path must equal the recomputed path.
        if entry.downgrade_path != entry.computed_downgrade_path() {
            violations.push(M5EcosystemCertificationViolation::DowngradePathMismatch {
                id: entry.entry_id.clone(),
            });
        }

        // A narrowed claim must name how to recover.
        if entry.downgrade_path.applied && entry.requalification_ref.trim().is_empty() {
            violations.push(
                M5EcosystemCertificationViolation::DowngradeWithoutRequalification {
                    id: entry.entry_id.clone(),
                },
            );
        }

        // The lane guardrail: any positive support claim must be evidence-backed and not
        // disqualified, so first-party or public-registry status never implies support
        // alone.
        if entry.effective_support_class != SupportClass::Unsupported
            && (!entry.is_evidence_backed()
                || entry.qualification_disposition == QualificationDisposition::Disqualified)
        {
            violations.push(
                M5EcosystemCertificationViolation::SupportClaimedWithoutEvidence {
                    id: entry.entry_id.clone(),
                },
            );
        }
    }
}

/// A validation violation for the M5 ecosystem-certification packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5EcosystemCertificationViolation {
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
    /// An entry lists a drill lane more than once.
    DuplicateLane {
        /// Entry id.
        id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// An entry is missing a required drill lane.
    MissingLane {
        /// Entry id.
        id: String,
        /// Lane token.
        lane: &'static str,
    },
    /// An entry lists a qualification signal more than once.
    DuplicateQualificationSignal {
        /// Entry id.
        id: String,
        /// Signal token.
        signal: &'static str,
    },
    /// An entry's qualification signals disagree with the recomputed set.
    QualificationSignalsMismatch {
        /// Entry id.
        id: String,
    },
    /// An entry's stored disposition disagrees with the recomputed value.
    QualificationDispositionMismatch {
        /// Entry id.
        id: String,
        /// Stored disposition token.
        stored: &'static str,
        /// Recomputed disposition token.
        computed: &'static str,
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
    /// An entry publishes a support claim without evidence or certification.
    SupportClaimedWithoutEvidence {
        /// Entry id.
        id: String,
    },
    /// The summary counts disagree with the entries.
    SummaryMismatch,
}

impl fmt::Display for M5EcosystemCertificationViolation {
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
                write!(f, "entry {id} repeats drill lane {lane}")
            }
            Self::MissingLane { id, lane } => {
                write!(f, "entry {id} is missing drill lane {lane}")
            }
            Self::DuplicateQualificationSignal { id, signal } => {
                write!(f, "entry {id} repeats qualification signal {signal}")
            }
            Self::QualificationSignalsMismatch { id } => {
                write!(
                    f,
                    "entry {id} qualification signals disagree with the recomputed set"
                )
            }
            Self::QualificationDispositionMismatch {
                id,
                stored,
                computed,
            } => {
                write!(
                    f,
                    "entry {id} publishes disposition {stored} but the recomputed disposition is {computed}"
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
            Self::SupportClaimedWithoutEvidence { id } => {
                write!(
                    f,
                    "entry {id} publishes a support claim without a current, owned, evidence-linked, certified row"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the entries")
            }
        }
    }
}

impl Error for M5EcosystemCertificationViolation {}

/// Loads the embedded M5 ecosystem-certification packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`M5EcosystemCertification`].
pub fn current_m5_ecosystem_certification() -> Result<M5EcosystemCertification, serde_json::Error> {
    serde_json::from_str(M5_ECOSYSTEM_CERTIFICATION_JSON)
}

#[cfg(test)]
mod tests;
