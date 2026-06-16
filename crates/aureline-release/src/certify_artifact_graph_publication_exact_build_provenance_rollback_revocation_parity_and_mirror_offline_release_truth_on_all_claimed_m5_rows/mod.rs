//! Typed M5 publication-certification register binding every claimed M5 artifact
//! family to one inspectable certification of its full publication artifact
//! graph: release-center object parity, clean-room rebuild, exact-build
//! symbolication, publish-target review, rollback, revocation, and mirror/offline
//! publication parity.
//!
//! Where the release-candidate/publish-target matrix freezes *what each family
//! publishes*, the promotion-ledger records *how it was promoted*, the
//! rollback/revocation records target *the smallest affected node set*, and the
//! clean-room rebuild proof records *exact-build supportability*, this register is
//! the closing certification layer: it asserts that every claimed M5 artifact
//! family is rebuildable, identifiable, symbolicated, support-explainable, and
//! revocable **as one system**. A family is certified only when its entire
//! publication artifact graph holds together; release-truth evidence that is
//! stale, partial, or missing narrows the row below the launch cutline rather than
//! being hand-waved in release notes.
//!
//! Each [`M5PublicationCertRow`] binds one [`M5ArtifactFamilyKind`] to:
//!
//! - the stable claim it backs ([`M5PublicationCertRow::claim_ref`],
//!   [`M5PublicationCertRow::claim_label`]);
//! - a certification scorecard ([`M5PublicationCertRow::scorecard`]) of one
//!   [`DimensionCell`] per [`PublicationDimension`], so release-center parity,
//!   clean-room rebuild, exact-build symbolication, publish-target review,
//!   rollback, revocation, and mirror/offline parity are each an explicit,
//!   inspectable grade;
//! - the publish-target posture ([`M5PublicationCertRow::publish_target`]) that
//!   makes the track invariant — *publish targets never inherit ambient
//!   credentials* — a first-class, machine-checkable field;
//! - the mirror/offline parity evidence ([`M5PublicationCertRow::mirror_offline`])
//!   that makes the guardrail — *no family claims mirror/offline parity without
//!   current drill evidence* — a first-class, machine-checkable field;
//! - the disclosed support posture ([`M5PublicationCertRow::disclosure`]), an
//!   owner-manifest sign-off ([`M5PublicationCertRow::owner_signoff`]), and an
//!   explicit downgrade automation ([`M5PublicationCertRow::downgrade_automation`])
//!   bound to a verified frozen-fallback rollback plan;
//! - the overall certification state earned ([`CertState`]), the active narrowing
//!   reasons ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`M5PublicationCertRow::published_label`]);
//! - a [`ProofPacket`] (reused from the stable claim manifest) and its freshness
//!   SLO, plus an optional waiver.
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the boundary
//! between a family that may publish a Stable claim and one that must narrow below
//! it. The [`M5PublicationCertStopRule`] set names the closed conditions that gate
//! promotion — one per [`NarrowingReason`] — and
//! [`M5PublicationCertRegister::promotion`] records the proceed/hold verdict. The
//! register binds the canonical M5 evidence index
//! ([`M5PublicationCertRegister::evidence_index_ref`]) so this publication truth is
//! shiproom-visible rather than buried in CI.
//!
//! The register is checked in at
//! `artifacts/release/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.json`
//! and embedded here, so this typed consumer and the CI gate agree on every M5
//! family without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no artifact bodies, signatures, symbol blobs, or credential material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::M5ArtifactFamilyKind;
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const M5_PUBLICATION_CERT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_PUBLICATION_CERT_RECORD_KIND: &str =
    "certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows";

/// Repo-relative path to the checked-in register.
pub const M5_PUBLICATION_CERT_PATH: &str =
    "artifacts/release/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.json";

/// Embedded checked-in register JSON.
pub const M5_PUBLICATION_CERT_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows.json"
));

/// One dimension of the publication-certification scorecard. Each maps one
/// publication-truth lane the family must hold to ship as one artifact graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicationDimension {
    /// The release-center object and headless flow render identical artifact-graph
    /// truth for the family.
    ReleaseCenterParity,
    /// A fresh clean-room rebuild reproduces the published artifact bit-for-bit.
    CleanRoomRebuild,
    /// Exact-build symbol/source-map linkage supports symbolication of the
    /// published build.
    ExactBuildSymbolication,
    /// The publish target was reviewed: scoped credentials, auth source disclosed,
    /// no ambient-credential inheritance.
    PublishTargetReview,
    /// A scoped rollback record targets the smallest affected node set.
    RollbackRecord,
    /// A revocation/emergency-disable record reaches every channel at parity.
    RevocationRecord,
    /// Mirror and offline channels publish the family at parity with current drill
    /// evidence.
    MirrorOfflineParity,
}

impl PublicationDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReleaseCenterParity,
        Self::CleanRoomRebuild,
        Self::ExactBuildSymbolication,
        Self::PublishTargetReview,
        Self::RollbackRecord,
        Self::RevocationRecord,
        Self::MirrorOfflineParity,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenterParity => "release_center_parity",
            Self::CleanRoomRebuild => "clean_room_rebuild",
            Self::ExactBuildSymbolication => "exact_build_symbolication",
            Self::PublishTargetReview => "publish_target_review",
            Self::RollbackRecord => "rollback_record",
            Self::RevocationRecord => "revocation_record",
            Self::MirrorOfflineParity => "mirror_offline_parity",
        }
    }

    /// The narrowing reason a non-passing, non-waived cell must name, given the
    /// cell's [`DimensionGrade`].
    pub const fn reason_for_grade(self, grade: DimensionGrade) -> Option<NarrowingReason> {
        match grade {
            DimensionGrade::Missing => Some(NarrowingReason::PublicationDimensionMissing),
            DimensionGrade::Fail | DimensionGrade::Partial => {
                Some(NarrowingReason::PublicationDimensionFailed)
            }
            DimensionGrade::Pass | DimensionGrade::Waived => None,
        }
    }
}

/// The grade earned on one certification dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionGrade {
    /// The dimension fully passes.
    Pass,
    /// The dimension partially passes; remediation is required.
    Partial,
    /// The dimension fails.
    Fail,
    /// Held provisionally under an active, unexpired waiver.
    Waived,
    /// The dimension has no certification evidence at all.
    Missing,
}

impl DimensionGrade {
    /// Every grade, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Pass,
        Self::Partial,
        Self::Fail,
        Self::Waived,
        Self::Missing,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Partial => "partial",
            Self::Fail => "fail",
            Self::Waived => "waived",
            Self::Missing => "missing",
        }
    }

    /// Whether a cell in this grade lets the family hold its claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Pass | Self::Waived)
    }
}

/// Trust tier of the owner behind an M5 artifact family's publication.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustTier {
    /// First-party Aureline-maintained family.
    FirstParty,
    /// A verified partner/vendor-maintained family.
    VerifiedPartner,
    /// A community-maintained family.
    Community,
    /// A scaffolded/generated family whose support posture must be disclosed.
    Generated,
}

impl TrustTier {
    /// Every tier, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FirstParty,
        Self::VerifiedPartner,
        Self::Community,
        Self::Generated,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::VerifiedPartner => "verified_partner",
            Self::Community => "community",
            Self::Generated => "generated",
        }
    }
}

/// Overall certification state a family earned for its publication artifact graph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertState {
    /// Every dimension passes, the publish target carries no ambient credentials,
    /// mirror/offline parity is proven with current drill evidence, the support
    /// posture is disclosed, the owner manifest is signed, and downgrade automation
    /// is defined and verified.
    Certified,
    /// One or more certification dimensions failed, are partial, or are missing.
    DimensionRegressed,
    /// The proof packet has gone stale.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// Downgrade automation is undefined or its frozen-fallback rollback plan is
    /// unverified.
    AutomationUndefined,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl CertState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::DimensionRegressed,
        Self::Stale,
        Self::OnWaiver,
        Self::AutomationUndefined,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::DimensionRegressed => "dimension_regressed",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::AutomationUndefined => "automation_undefined",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// Whether the state lets a family carry the claim at its label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Certified | Self::OnWaiver)
    }

    /// Whether the state forces the family below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a family's certification narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// A certification dimension failed or is only partial.
    PublicationDimensionFailed,
    /// A certification dimension is missing.
    PublicationDimensionMissing,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// The owner manifest is unsigned.
    OwnerManifestUnsigned,
    /// The frozen-fallback rollback plan is unverified.
    RollbackPlanUnverified,
    /// The downgrade automation is undefined.
    DowngradeAutomationUndefined,
    /// A waiver the family relied on has expired.
    WaiverExpired,
    /// The publish target inherits ambient credentials instead of a scoped,
    /// disclosed auth source.
    AmbientCredentialInherited,
    /// Mirror/offline parity is claimed without current drill evidence (or a
    /// channel is out of parity).
    MirrorOfflineDrillStale,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::PublicationDimensionFailed,
        Self::PublicationDimensionMissing,
        Self::ProofPacketMissing,
        Self::ProofPacketStale,
        Self::OwnerManifestUnsigned,
        Self::RollbackPlanUnverified,
        Self::DowngradeAutomationUndefined,
        Self::WaiverExpired,
        Self::AmbientCredentialInherited,
        Self::MirrorOfflineDrillStale,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicationDimensionFailed => "publication_dimension_failed",
            Self::PublicationDimensionMissing => "publication_dimension_missing",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::OwnerManifestUnsigned => "owner_manifest_unsigned",
            Self::RollbackPlanUnverified => "rollback_plan_unverified",
            Self::DowngradeAutomationUndefined => "downgrade_automation_undefined",
            Self::WaiverExpired => "waiver_expired",
            Self::AmbientCredentialInherited => "ambient_credential_inherited",
            Self::MirrorOfflineDrillStale => "mirror_offline_drill_stale",
        }
    }

    /// Whether this reason marks a downgrade-automation gap.
    pub const fn is_automation_gap(self) -> bool {
        matches!(
            self,
            Self::RollbackPlanUnverified | Self::DowngradeAutomationUndefined
        )
    }

    /// Whether this reason marks a certification-dimension gap.
    pub const fn is_dimension_gap(self) -> bool {
        matches!(
            self,
            Self::PublicationDimensionFailed | Self::PublicationDimensionMissing
        )
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold promotion until the condition clears.
    HoldPromotion,
    /// Narrow the claim below the cutline.
    NarrowLabel,
    /// Remediate the failing or missing certification dimension.
    RemediateDimension,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner-manifest sign-off.
    RequestOwnerSignoff,
    /// Verify the frozen-fallback rollback plan.
    VerifyRollbackPlan,
    /// Define the downgrade automation.
    DefineDowngradeAutomation,
    /// Renew the expired waiver.
    RenewWaiver,
    /// Rotate the publish target to a scoped, disclosed credential.
    RotateScopedCredential,
    /// Refresh the mirror/offline parity drill.
    RefreshMirrorDrill,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::HoldPromotion,
        Self::NarrowLabel,
        Self::RemediateDimension,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
        Self::VerifyRollbackPlan,
        Self::DefineDowngradeAutomation,
        Self::RenewWaiver,
        Self::RotateScopedCredential,
        Self::RefreshMirrorDrill,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPromotion => "hold_promotion",
            Self::NarrowLabel => "narrow_label",
            Self::RemediateDimension => "remediate_dimension",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::VerifyRollbackPlan => "verify_rollback_plan",
            Self::DefineDowngradeAutomation => "define_downgrade_automation",
            Self::RenewWaiver => "renew_waiver",
            Self::RotateScopedCredential => "rotate_scoped_credential",
            Self::RefreshMirrorDrill => "refresh_mirror_drill",
        }
    }
}

/// What triggers a family's automated downgrade to a frozen floor label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationTrigger {
    /// Fires when the proof packet goes stale.
    ProofStale,
    /// Fires when a certification dimension regresses.
    DimensionRegressed,
    /// Fires when owner sign-off is revoked.
    OwnerRevoked,
    /// Operator-driven manual downgrade.
    Manual,
}

impl AutomationTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProofStale,
        Self::DimensionRegressed,
        Self::OwnerRevoked,
        Self::Manual,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::DimensionRegressed => "dimension_regressed",
            Self::OwnerRevoked => "owner_revoked",
            Self::Manual => "manual",
        }
    }
}

/// The defined/verified state of a family's downgrade automation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutomationState {
    /// The automation is defined and its frozen-fallback rollback plan is verified.
    Defined,
    /// The automation is defined but its frozen-fallback rollback plan is
    /// unverified.
    Unverified,
    /// The automation is undefined.
    Undefined,
}

impl AutomationState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Defined, Self::Unverified, Self::Undefined];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Defined => "defined",
            Self::Unverified => "unverified",
            Self::Undefined => "undefined",
        }
    }

    /// Whether the automation is defined and verified, letting a family hold a
    /// Stable claim.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Defined)
    }
}

/// One cell of the certification scorecard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionCell {
    /// The certification dimension this cell speaks for.
    pub dimension: PublicationDimension,
    /// The grade earned for the dimension.
    pub grade: DimensionGrade,
    /// Ref to the dimension's evidence. Empty only on a missing cell.
    pub evidence_ref: String,
}

/// The publish-target posture for a family, making the *no ambient credentials*
/// invariant a first-class field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishTargetPosture {
    /// Stable ref to the publish-target review sheet / descriptor.
    pub auth_source_ref: String,
    /// Whether the publish target's auth source is explicitly disclosed.
    pub auth_source_disclosed: bool,
    /// Whether the publish target inherits ambient credentials instead of a
    /// scoped, reviewed auth source. Always `false` on a certified family.
    pub inherits_ambient_credentials: bool,
}

impl PublishTargetPosture {
    /// True when the publish target is scoped: auth source disclosed and no
    /// ambient-credential inheritance.
    pub fn is_scoped(&self) -> bool {
        self.auth_source_disclosed && !self.inherits_ambient_credentials
    }
}

/// The mirror/offline publication parity evidence for a family, making the
/// *no parity claim without current drill evidence* guardrail a first-class field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MirrorOfflineParity {
    /// Stable ref to the mirror/offline publication drill record.
    pub drill_ref: String,
    /// Whether the hosted channel publishes the family at parity.
    pub hosted_parity: bool,
    /// Whether the mirrored channel publishes the family at parity.
    pub mirrored_parity: bool,
    /// Whether the offline/air-gapped channel publishes the family at parity.
    pub offline_parity: bool,
    /// Freshness state of the parity drill evidence.
    pub drill_state: FreshnessSloState,
}

impl MirrorOfflineParity {
    /// True when every channel is at parity and the drill evidence is within its
    /// freshness SLO.
    pub fn fully_proven(&self) -> bool {
        self.hosted_parity
            && self.mirrored_parity
            && self.offline_parity
            && self.drill_state.is_within_slo()
    }
}

/// The disclosed support posture of a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SupportDisclosure {
    /// Stable ref to the support window the family commits to.
    pub support_window_ref: String,
    /// Stable ref to the publication/advisory policy the family rides.
    pub policy_ref: String,
    /// Trust tier of the owner.
    pub trust_tier: TrustTier,
    /// Refs to the release-line scopes the family covers.
    #[serde(default)]
    pub scope_refs: Vec<String>,
    /// Whether the redaction/provenance posture of the family is disclosed to the
    /// operator.
    pub redaction_disclosed: bool,
}

/// A family's downgrade automation, falling back to a frozen floor label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DowngradeAutomation {
    /// Stable ref to the downgrade-automation definition.
    pub automation_ref: String,
    /// Ref to the frozen-fallback rollback plan the automation drives.
    pub rollback_plan_ref: String,
    /// What triggers the automated downgrade.
    pub trigger: AutomationTrigger,
    /// The lifecycle label the downgrade narrows the family to.
    pub target_floor: StableClaimLevel,
    /// The defined/verified state of the automation.
    pub state: AutomationState,
    /// Whether the frozen-fallback rollback plan has been verified end-to-end.
    pub rollback_verified: bool,
}

/// One certification stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicationCertStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched family fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StopAction,
    /// Whether firing this rule blocks promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One certified M5 artifact family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicationCertRow {
    /// Stable family id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The artifact family this row certifies.
    pub family_kind: M5ArtifactFamilyKind,
    /// The family ref this entry speaks about.
    pub family_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall certification state earned for the family.
    pub cert_state: CertState,
    /// The certification scorecard: one cell per [`PublicationDimension`].
    pub scorecard: Vec<DimensionCell>,
    /// The publish-target posture (scoped credentials, no ambient inheritance).
    pub publish_target: PublishTargetPosture,
    /// The mirror/offline parity evidence and drill freshness.
    pub mirror_offline: MirrorOfflineParity,
    /// The disclosed support posture of the family.
    pub disclosure: SupportDisclosure,
    /// The downgrade automation backing the family.
    pub downgrade_automation: DowngradeAutomation,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner manifest sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the family below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the family effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this family's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the family carries this posture.
    pub rationale: String,
}

impl M5PublicationCertRow {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the family's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.cert_state.holds_label()
    }

    /// True when a narrowing reason is active on the family.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// Returns the cell registered for `dimension`, if any.
    pub fn cell(&self, dimension: PublicationDimension) -> Option<&DimensionCell> {
        self.scorecard
            .iter()
            .find(|cell| cell.dimension == dimension)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicationCertSummary {
    /// Total number of certified families.
    pub total_entries: usize,
    /// Distinct artifact families covered.
    pub total_families: usize,
    /// Families publishing a label at or above the cutline.
    pub entries_certified: usize,
    /// Families narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Families holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Families carrying a certification-dimension gap (failed or missing).
    pub entries_with_dimension_gap: usize,
    /// Families carrying an owner-manifest-unsigned reason.
    pub entries_with_owner_gap: usize,
    /// Families carrying a downgrade-automation gap.
    pub entries_with_automation_gap: usize,
    /// Families whose publish target inherits ambient credentials.
    pub entries_with_ambient_credential_gap: usize,
    /// Families whose mirror/offline parity drill is stale or out of parity.
    pub entries_with_mirror_drill_gap: usize,
    /// Families whose redaction/provenance posture is not disclosed.
    pub entries_redaction_undisclosed: usize,
    /// Total release-blocking families.
    pub release_blocking_total: usize,
    /// Release-blocking families publishing a label at or above the cutline.
    pub release_blocking_certified: usize,
    /// Release-blocking families narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// First-party-trust families.
    pub first_party_entries: usize,
    /// Verified-partner-trust families.
    pub verified_partner_entries: usize,
    /// Community-trust families.
    pub community_entries: usize,
    /// Generated families.
    pub generated_entries: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Mirror/offline drills whose state is `current`.
    pub mirror_drills_current: usize,
    /// Mirror/offline drills whose state is `due_for_refresh`.
    pub mirror_drills_due_for_refresh: usize,
    /// Mirror/offline drills whose state is `breached`.
    pub mirror_drills_breached: usize,
    /// Mirror/offline drills whose state is `missing`.
    pub mirror_drills_missing: usize,
    /// Total active narrowing reasons across all families.
    pub total_active_narrowing_reasons: usize,
    /// Total certification cells across all families.
    pub total_dimension_cells: usize,
    /// Cells graded `pass`.
    pub cells_pass: usize,
    /// Cells graded `partial`.
    pub cells_partial: usize,
    /// Cells graded `fail`.
    pub cells_fail: usize,
    /// Cells graded `waived`.
    pub cells_waived: usize,
    /// Cells graded `missing`.
    pub cells_missing: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationCertExportRow {
    /// Stable family id.
    pub entry_id: String,
    /// The artifact family this row certifies.
    pub family_kind: M5ArtifactFamilyKind,
    /// The family ref this entry speaks about.
    pub family_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the family publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall certification state earned.
    pub cert_state: CertState,
    /// Trust tier of the owner.
    pub trust_tier: TrustTier,
    /// Whether the redaction/provenance posture is disclosed to the operator.
    pub redaction_disclosed: bool,
    /// Whether the publish target inherits ambient credentials.
    pub inherits_ambient_credentials: bool,
    /// Freshness state of the mirror/offline parity drill.
    pub mirror_drill_state: FreshnessSloState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Downgrade-automation state.
    pub automation_state: AutomationState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, release-center, service-health, support, and
/// docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PublicationCertExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5PublicationCertExportRow>,
}

/// The typed M5 publication-certification register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5PublicationCertRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub manifest_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this register ingests.
    pub claim_manifest_ref: String,
    /// Ref to the canonical M5 evidence index this register reports into.
    pub evidence_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed artifact-family vocabulary.
    pub family_kinds: Vec<M5ArtifactFamilyKind>,
    /// Closed publication-dimension vocabulary.
    pub publication_dimensions: Vec<PublicationDimension>,
    /// Closed dimension-grade vocabulary.
    pub dimension_grades: Vec<DimensionGrade>,
    /// Closed certification-state vocabulary.
    pub cert_states: Vec<CertState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// Closed downgrade-automation-trigger vocabulary.
    pub automation_triggers: Vec<AutomationTrigger>,
    /// Closed downgrade-automation-state vocabulary.
    pub automation_states: Vec<AutomationState>,
    /// Closed trust-tier vocabulary.
    pub trust_tiers: Vec<TrustTier>,
    /// Closed freshness-state vocabulary (proof packets and mirror drills).
    pub freshness_states: Vec<FreshnessSloState>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family refs this register must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<M5PublicationCertStopRule>,
    /// Certified families.
    pub rows: Vec<M5PublicationCertRow>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5PublicationCertSummary,
}

impl M5PublicationCertRegister {
    /// Returns the family registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&M5PublicationCertRow> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the families publishing a label at or above the cutline.
    pub fn rows_published_stable(&self) -> Vec<&M5PublicationCertRow> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the families narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&M5PublicationCertRow> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking families.
    pub fn release_blocking_rows(&self) -> Vec<&M5PublicationCertRow> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the families for one artifact-family kind.
    pub fn rows_for_kind(&self, kind: M5ArtifactFamilyKind) -> Vec<&M5PublicationCertRow> {
        self.rows
            .iter()
            .filter(|row| row.family_kind == kind)
            .collect()
    }

    /// Distinct claims (by claim ref) the register covers.
    pub fn claims(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            set.insert(row.claim_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched family carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &M5PublicationCertStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the families and stop rules.
    pub fn computed_promotion_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block promotion and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Family ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only families whose claim is at or above the cutline count: a family whose
    /// claim is already canonically narrowed is not a *promotion* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for row in &self.rows {
            if row.claim_holds_stable()
                && row
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(row.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the families and stop rules.
    pub fn computed_summary(&self) -> M5PublicationCertSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let drills = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.mirror_offline.drill_state == state)
                .count()
        };
        let trust = |tier: TrustTier| {
            self.rows
                .iter()
                .filter(|row| row.disclosure.trust_tier == tier)
                .count()
        };
        let with_predicate = |pred: fn(NarrowingReason) -> bool| {
            self.rows
                .iter()
                .filter(|row| row.active_narrowing_reasons.iter().any(|r| pred(*r)))
                .count()
        };
        let with_reason = |reason: NarrowingReason| {
            self.rows
                .iter()
                .filter(|row| row.has_active_reason(reason))
                .count()
        };
        let cell_grade = |grade: DimensionGrade| {
            self.rows
                .iter()
                .flat_map(|row| row.scorecard.iter())
                .filter(|cell| cell.grade == grade)
                .count()
        };
        let release_blocking: Vec<&M5PublicationCertRow> = self.release_blocking_rows();
        M5PublicationCertSummary {
            total_entries: self.rows.len(),
            total_families: self
                .rows
                .iter()
                .map(|row| row.family_kind)
                .collect::<BTreeSet<_>>()
                .len(),
            entries_certified: self
                .rows
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            entries_narrowed: self
                .rows
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            entries_on_active_waiver: self
                .rows
                .iter()
                .filter(|row| row.cert_state == CertState::OnWaiver)
                .count(),
            entries_with_dimension_gap: with_predicate(NarrowingReason::is_dimension_gap),
            entries_with_owner_gap: with_reason(NarrowingReason::OwnerManifestUnsigned),
            entries_with_automation_gap: with_predicate(NarrowingReason::is_automation_gap),
            entries_with_ambient_credential_gap: with_reason(
                NarrowingReason::AmbientCredentialInherited,
            ),
            entries_with_mirror_drill_gap: with_reason(NarrowingReason::MirrorOfflineDrillStale),
            entries_redaction_undisclosed: self
                .rows
                .iter()
                .filter(|row| !row.disclosure.redaction_disclosed)
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_certified: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            first_party_entries: trust(TrustTier::FirstParty),
            verified_partner_entries: trust(TrustTier::VerifiedPartner),
            community_entries: trust(TrustTier::Community),
            generated_entries: trust(TrustTier::Generated),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            mirror_drills_current: drills(FreshnessSloState::Current),
            mirror_drills_due_for_refresh: drills(FreshnessSloState::DueForRefresh),
            mirror_drills_breached: drills(FreshnessSloState::Breached),
            mirror_drills_missing: drills(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            total_dimension_cells: self.rows.iter().map(|row| row.scorecard.len()).sum(),
            cells_pass: cell_grade(DimensionGrade::Pass),
            cells_partial: cell_grade(DimensionGrade::Partial),
            cells_fail: cell_grade(DimensionGrade::Fail),
            cells_waived: cell_grade(DimensionGrade::Waived),
            cells_missing: cell_grade(DimensionGrade::Missing),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5PublicationCertExportProjection {
        M5PublicationCertExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .rows
                .iter()
                .map(|row| M5PublicationCertExportRow {
                    entry_id: row.entry_id.clone(),
                    family_kind: row.family_kind,
                    family_ref: row.family_ref.clone(),
                    release_blocking: row.release_blocking,
                    claim_ref: row.claim_ref.clone(),
                    claim_label: row.claim_label,
                    published_label: row.published_label,
                    publishes_stable: row.publishes_stable(),
                    cert_state: row.cert_state,
                    trust_tier: row.disclosure.trust_tier,
                    redaction_disclosed: row.disclosure.redaction_disclosed,
                    inherits_ambient_credentials: row.publish_target.inherits_ambient_credentials,
                    mirror_drill_state: row.mirror_offline.drill_state,
                    slo_state: row.proof_packet.slo_state,
                    automation_state: row.downgrade_automation.state,
                    active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<M5PublicationCertViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(M5PublicationCertViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(M5PublicationCertViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5PublicationCertViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5PublicationCertViolation>) {
        if self.schema_version != M5_PUBLICATION_CERT_SCHEMA_VERSION {
            violations.push(M5PublicationCertViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_PUBLICATION_CERT_RECORD_KIND {
            violations.push(M5PublicationCertViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("evidence_index_ref", &self.evidence_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PublicationCertViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.family_kinds != M5ArtifactFamilyKind::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "family_kinds",
            });
        }
        if self.publication_dimensions != PublicationDimension::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "publication_dimensions",
            });
        }
        if self.dimension_grades != DimensionGrade::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "dimension_grades",
            });
        }
        if self.cert_states != CertState::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "cert_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }
        if self.automation_triggers != AutomationTrigger::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "automation_triggers",
            });
        }
        if self.automation_states != AutomationState::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "automation_states",
            });
        }
        if self.trust_tiers != TrustTier::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "trust_tiers",
            });
        }
        if self.freshness_states != FreshnessSloState::ALL.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "freshness_states",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5PublicationCertViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5PublicationCertViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5PublicationCertViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5PublicationCertViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5PublicationCertViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5PublicationCertViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5PublicationCertViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(M5PublicationCertViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &M5PublicationCertRow,
        violations: &mut Vec<M5PublicationCertViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("family_ref", &row.family_ref),
            ("family_summary", &row.family_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            ("proof_packet.packet_id", &row.proof_packet.packet_id),
            ("proof_packet.packet_ref", &row.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &row.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &row.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &row.owner_signoff.owner_ref),
            (
                "disclosure.support_window_ref",
                &row.disclosure.support_window_ref,
            ),
            ("disclosure.policy_ref", &row.disclosure.policy_ref),
            (
                "publish_target.auth_source_ref",
                &row.publish_target.auth_source_ref,
            ),
            ("mirror_offline.drill_ref", &row.mirror_offline.drill_ref),
            (
                "downgrade_automation.automation_ref",
                &row.downgrade_automation.automation_ref,
            ),
            (
                "downgrade_automation.rollback_plan_ref",
                &row.downgrade_automation.rollback_plan_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(M5PublicationCertViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_scorecard(row, violations);
        self.validate_downgrade_automation(row, violations);
        self.validate_publish_target(row, violations);
        self.validate_mirror_offline(row, violations);

        // The ceiling: no family may carry a label wider than the claim's canonical
        // label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(M5PublicationCertViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5PublicationCertViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5PublicationCertViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the family to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(M5PublicationCertViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(M5PublicationCertViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.cert_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A certified family publishes exactly the claim's canonical label,
            // carries no active reason, rides a captured within-SLO packet,
            // discloses the redaction posture, is owner-signed, rides defined-and-
            // verified downgrade automation, publishes through a scoped publish
            // target, and proves mirror/offline parity with current drill evidence.
            if row.published_label != row.claim_label {
                violations.push(M5PublicationCertViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(M5PublicationCertViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.disclosure.redaction_disclosed {
                violations.push(M5PublicationCertViolation::HeldWithoutRedactionDisclosure {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(M5PublicationCertViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5PublicationCertViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(M5PublicationCertViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.downgrade_automation.state.holds()
                || !row.downgrade_automation.rollback_verified
            {
                violations.push(M5PublicationCertViolation::HeldWithoutDowngradeAutomation {
                    entry_id: row.entry_id.clone(),
                    state: row.downgrade_automation.state,
                });
            }
            if !row.publish_target.is_scoped() {
                violations.push(M5PublicationCertViolation::HeldWithoutScopedPublishTarget {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.mirror_offline.fully_proven() {
                violations.push(M5PublicationCertViolation::HeldWithoutMirrorParity {
                    entry_id: row.entry_id.clone(),
                });
            }
        } else {
            // A narrowing state must drop the published label below the cutline and
            // name at least one active reason.
            if row.publishes_stable() {
                violations.push(M5PublicationCertViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.cert_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(M5PublicationCertViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.cert_state,
                });
            }
            // A narrowing family whose packet is breached or missing must name the
            // matching freshness reason.
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(M5PublicationCertViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(M5PublicationCertViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    fn validate_scorecard(
        &self,
        row: &M5PublicationCertRow,
        violations: &mut Vec<M5PublicationCertViolation>,
    ) {
        let mut seen: BTreeSet<PublicationDimension> = BTreeSet::new();
        for cell in &row.scorecard {
            if !seen.insert(cell.dimension) {
                violations.push(M5PublicationCertViolation::DuplicateDimension {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A missing cell carries no evidence ref; every other grade must.
            if cell.grade != DimensionGrade::Missing && cell.evidence_ref.trim().is_empty() {
                violations.push(M5PublicationCertViolation::CellEvidenceMissing {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A waived cell only holds under an unexpired waiver.
            if cell.grade == DimensionGrade::Waived && row.waiver.is_none() {
                violations.push(M5PublicationCertViolation::WaivedCellWithoutWaiver {
                    entry_id: row.entry_id.clone(),
                    dimension: cell.dimension,
                });
            }
            // A non-passing, non-waived cell must name its narrowing reason.
            if !cell.grade.holds() {
                if let Some(reason) = cell.dimension.reason_for_grade(cell.grade) {
                    if !row.has_active_reason(reason) {
                        violations.push(M5PublicationCertViolation::CellReasonNotActive {
                            entry_id: row.entry_id.clone(),
                            dimension: cell.dimension,
                            reason,
                        });
                    }
                }
            }
        }
        // The scorecard must carry exactly one cell per dimension.
        for dimension in PublicationDimension::ALL {
            if !seen.contains(&dimension) {
                violations.push(M5PublicationCertViolation::DimensionIncompleteCoverage {
                    entry_id: row.entry_id.clone(),
                    dimension,
                });
            }
        }
    }

    fn validate_downgrade_automation(
        &self,
        row: &M5PublicationCertRow,
        violations: &mut Vec<M5PublicationCertViolation>,
    ) {
        let automation = &row.downgrade_automation;
        // A downgrade narrows the claim, so its floor must be below the cutline.
        if automation.target_floor.is_at_or_above_cutline() {
            violations.push(M5PublicationCertViolation::AutomationFloorNotBelowCutline {
                entry_id: row.entry_id.clone(),
                floor: automation.target_floor,
            });
        }
        // An undefined automation must name the undefined reason.
        if automation.state == AutomationState::Undefined
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(M5PublicationCertViolation::AutomationStateWithoutReason {
                entry_id: row.entry_id.clone(),
                state: automation.state,
            });
        }
        // An unverified frozen-fallback rollback plan must name a rollback reason.
        if !automation.rollback_verified
            && !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
            && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
        {
            violations.push(
                M5PublicationCertViolation::RollbackUnverifiedWithoutReason {
                    entry_id: row.entry_id.clone(),
                },
            );
        }
    }

    fn validate_publish_target(
        &self,
        row: &M5PublicationCertRow,
        violations: &mut Vec<M5PublicationCertViolation>,
    ) {
        // The track invariant: a publish target that inherits ambient credentials
        // must name the matching reason and may not hold its publish-target-review
        // dimension.
        if row.publish_target.inherits_ambient_credentials {
            if !row.has_active_reason(NarrowingReason::AmbientCredentialInherited) {
                violations.push(M5PublicationCertViolation::AmbientCredentialWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row
                .cell(PublicationDimension::PublishTargetReview)
                .map(|cell| cell.grade.holds())
                .unwrap_or(false)
            {
                violations.push(
                    M5PublicationCertViolation::AmbientCredentialDimensionHolds {
                        entry_id: row.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_mirror_offline(
        &self,
        row: &M5PublicationCertRow,
        violations: &mut Vec<M5PublicationCertViolation>,
    ) {
        // The guardrail: a family that has not proven mirror/offline parity with
        // current drill evidence must name the matching reason and may not hold its
        // mirror/offline-parity dimension.
        if !row.mirror_offline.fully_proven() {
            if !row.has_active_reason(NarrowingReason::MirrorOfflineDrillStale) {
                violations.push(M5PublicationCertViolation::MirrorParityWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if row
                .cell(PublicationDimension::MirrorOfflineParity)
                .map(|cell| cell.grade.holds())
                .unwrap_or(false)
            {
                violations.push(M5PublicationCertViolation::MirrorParityDimensionHolds {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &M5PublicationCertRow,
        violations: &mut Vec<M5PublicationCertViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<M5PublicationCertViolation>,
                               expected: NarrowingReason| {
            violations.push(M5PublicationCertViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.cert_state,
                expected_reason: expected,
            });
        };

        match row.cert_state {
            CertState::DimensionRegressed => {
                if !row.has_active_reason(NarrowingReason::PublicationDimensionFailed)
                    && !row.has_active_reason(NarrowingReason::PublicationDimensionMissing)
                {
                    push_incoherent(violations, NarrowingReason::PublicationDimensionFailed);
                }
            }
            CertState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale) {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            CertState::AutomationUndefined => {
                if !row.has_active_reason(NarrowingReason::RollbackPlanUnverified)
                    && !row.has_active_reason(NarrowingReason::DowngradeAutomationUndefined)
                {
                    push_incoherent(violations, NarrowingReason::DowngradeAutomationUndefined);
                }
            }
            CertState::OwnerUnsigned => {
                if !row.has_active_reason(NarrowingReason::OwnerManifestUnsigned) {
                    push_incoherent(violations, NarrowingReason::OwnerManifestUnsigned);
                }
            }
            CertState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(M5PublicationCertViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.cert_state,
                    });
                }
            }
            CertState::Certified => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5PublicationCertViolation>) {
        // Every artifact family kind must be certified by at least one row.
        for kind in M5ArtifactFamilyKind::ALL {
            if self.rows_for_kind(kind).is_empty() {
                violations
                    .push(M5PublicationCertViolation::FamilyKindUncovered { family_kind: kind });
            }
        }

        let covered: BTreeSet<String> =
            self.rows.iter().map(|row| row.family_ref.clone()).collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(M5PublicationCertViolation::ReleaseBlockingFamilyUncovered {
                    family_ref: declared.clone(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_family_refs.contains(&row.family_ref)
            {
                violations.push(M5PublicationCertViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<M5PublicationCertViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(M5PublicationCertViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(M5PublicationCertViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(M5PublicationCertViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(M5PublicationCertViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(M5PublicationCertViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the publication-certification register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PublicationCertViolation {
    /// The register carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the register.
        actual: u32,
    },
    /// The register carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the register.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The register has no families.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Family or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A family id appears more than once.
    DuplicateEntryId {
        /// Duplicate entry id.
        entry_id: String,
    },
    /// A stop-rule id appears more than once.
    DuplicateStopRuleId {
        /// Duplicate rule id.
        rule_id: String,
    },
    /// A stop rule names no labels to watch.
    StopRuleWithoutLabels {
        /// Rule id.
        rule_id: String,
    },
    /// A narrowing reason has no stop rule watching for it.
    NarrowingReasonWithoutStopRule {
        /// Uncovered reason.
        reason: NarrowingReason,
    },
    /// A family has two cells for one dimension.
    DuplicateDimension {
        /// Family id.
        entry_id: String,
        /// Duplicated dimension.
        dimension: PublicationDimension,
    },
    /// A family is missing a dimension cell.
    DimensionIncompleteCoverage {
        /// Family id.
        entry_id: String,
        /// Uncovered dimension.
        dimension: PublicationDimension,
    },
    /// A non-missing cell has no evidence ref.
    CellEvidenceMissing {
        /// Family id.
        entry_id: String,
        /// Dimension.
        dimension: PublicationDimension,
    },
    /// A waived cell is carried without a waiver.
    WaivedCellWithoutWaiver {
        /// Family id.
        entry_id: String,
        /// Dimension.
        dimension: PublicationDimension,
    },
    /// A non-passing cell does not name its narrowing reason.
    CellReasonNotActive {
        /// Family id.
        entry_id: String,
        /// Dimension.
        dimension: PublicationDimension,
        /// The reason the cell requires.
        reason: NarrowingReason,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Family id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A family holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Family id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Family id.
        entry_id: String,
        /// Certification state.
        state: CertState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Family id.
        entry_id: String,
        /// Certification state.
        state: CertState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held family carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Family id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held family has active narrowing reasons.
    HeldWithActiveGap {
        /// Family id.
        entry_id: String,
    },
    /// A held family does not disclose its redaction/provenance posture.
    HeldWithoutRedactionDisclosure {
        /// Family id.
        entry_id: String,
    },
    /// A held family has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Family id.
        entry_id: String,
    },
    /// A held family rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Family id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held family lacks owner-manifest sign-off.
    HeldWithoutSignoff {
        /// Family id.
        entry_id: String,
    },
    /// A held family lacks defined-and-verified downgrade automation.
    HeldWithoutDowngradeAutomation {
        /// Family id.
        entry_id: String,
        /// Downgrade-automation state.
        state: AutomationState,
    },
    /// A held family publishes through an unscoped publish target (ambient
    /// credentials, or auth source undisclosed).
    HeldWithoutScopedPublishTarget {
        /// Family id.
        entry_id: String,
    },
    /// A held family claims mirror/offline parity without full, current drill
    /// evidence.
    HeldWithoutMirrorParity {
        /// Family id.
        entry_id: String,
    },
    /// The downgrade floor is not below the cutline.
    AutomationFloorNotBelowCutline {
        /// Family id.
        entry_id: String,
        /// Declared floor.
        floor: StableClaimLevel,
    },
    /// An undefined automation does not name the undefined reason.
    AutomationStateWithoutReason {
        /// Family id.
        entry_id: String,
        /// Downgrade-automation state.
        state: AutomationState,
    },
    /// An unverified frozen-fallback rollback plan does not name a rollback reason.
    RollbackUnverifiedWithoutReason {
        /// Family id.
        entry_id: String,
    },
    /// A publish target inheriting ambient credentials does not name the matching
    /// reason.
    AmbientCredentialWithoutReason {
        /// Family id.
        entry_id: String,
    },
    /// A publish target inheriting ambient credentials still holds its
    /// publish-target-review dimension.
    AmbientCredentialDimensionHolds {
        /// Family id.
        entry_id: String,
    },
    /// A family without full, current mirror/offline parity does not name the
    /// matching reason.
    MirrorParityWithoutReason {
        /// Family id.
        entry_id: String,
    },
    /// A family without full, current mirror/offline parity still holds its
    /// mirror/offline-parity dimension.
    MirrorParityDimensionHolds {
        /// Family id.
        entry_id: String,
    },
    /// A narrowing family with a breached proof packet does not name the stale
    /// reason.
    BreachedPacketWithoutReason {
        /// Family id.
        entry_id: String,
    },
    /// A narrowing family with a missing proof packet does not name the missing
    /// reason.
    MissingPacketWithoutReason {
        /// Family id.
        entry_id: String,
    },
    /// A family state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Family id.
        entry_id: String,
        /// Certification state.
        state: CertState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Family id.
        entry_id: String,
        /// Certification state.
        state: CertState,
    },
    /// An artifact family kind has no covering row.
    FamilyKindUncovered {
        /// Uncovered family kind.
        family_kind: M5ArtifactFamilyKind,
    },
    /// A release-blocking family ref has no covering row.
    ReleaseBlockingFamilyUncovered {
        /// Family ref.
        family_ref: String,
    },
    /// A release-blocking family is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Family id.
        entry_id: String,
    },
    /// The declared promotion decision disagrees with the computed one.
    PromotionDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared promotion blocking set disagrees with the computed one.
    PromotionBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the families.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Family id.
        entry_id: String,
    },
}

impl fmt::Display for M5PublicationCertViolation {
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
            Self::EmptyRegister => write!(f, "register has no families"),
            Self::NoStopRules => write!(f, "register has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate entry id {entry_id}"),
            Self::DuplicateStopRuleId { rule_id } => write!(f, "duplicate stop rule id {rule_id}"),
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::NarrowingReasonWithoutStopRule { reason } => write!(
                f,
                "narrowing reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::DuplicateDimension {
                entry_id,
                dimension,
            } => write!(
                f,
                "family {entry_id} has duplicate dimension {}",
                dimension.as_str()
            ),
            Self::DimensionIncompleteCoverage {
                entry_id,
                dimension,
            } => write!(
                f,
                "family {entry_id} is missing dimension {}",
                dimension.as_str()
            ),
            Self::CellEvidenceMissing {
                entry_id,
                dimension,
            } => write!(
                f,
                "family {entry_id} dimension {} has no evidence ref",
                dimension.as_str()
            ),
            Self::WaivedCellWithoutWaiver {
                entry_id,
                dimension,
            } => write!(
                f,
                "family {entry_id} dimension {} is waived without a waiver",
                dimension.as_str()
            ),
            Self::CellReasonNotActive {
                entry_id,
                dimension,
                reason,
            } => write!(
                f,
                "family {entry_id} dimension {} requires active reason {}",
                dimension.as_str(),
                reason.as_str()
            ),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "family {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "family {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "family {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "family {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "family {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "family {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutRedactionDisclosure { entry_id } => write!(
                f,
                "family {entry_id} holds stable without disclosing its redaction posture"
            ),
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "family {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "family {entry_id} holds stable on stale packet {slo_state:?}"
            ),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "family {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithoutDowngradeAutomation { entry_id, state } => write!(
                f,
                "family {entry_id} holds stable without defined+verified downgrade automation ({state:?})"
            ),
            Self::HeldWithoutScopedPublishTarget { entry_id } => write!(
                f,
                "family {entry_id} holds stable through an unscoped publish target (ambient credentials or undisclosed auth source)"
            ),
            Self::HeldWithoutMirrorParity { entry_id } => write!(
                f,
                "family {entry_id} holds stable without full, current mirror/offline parity"
            ),
            Self::AutomationFloorNotBelowCutline { entry_id, floor } => write!(
                f,
                "family {entry_id} downgrade floor {floor:?} is not below the cutline"
            ),
            Self::AutomationStateWithoutReason { entry_id, state } => write!(
                f,
                "family {entry_id} downgrade-automation state {state:?} names no narrowing reason"
            ),
            Self::RollbackUnverifiedWithoutReason { entry_id } => write!(
                f,
                "family {entry_id} has an unverified frozen-fallback rollback plan without a reason"
            ),
            Self::AmbientCredentialWithoutReason { entry_id } => write!(
                f,
                "family {entry_id} inherits ambient credentials without naming the matching reason"
            ),
            Self::AmbientCredentialDimensionHolds { entry_id } => write!(
                f,
                "family {entry_id} inherits ambient credentials but still holds publish_target_review"
            ),
            Self::MirrorParityWithoutReason { entry_id } => write!(
                f,
                "family {entry_id} lacks full, current mirror/offline parity without naming the matching reason"
            ),
            Self::MirrorParityDimensionHolds { entry_id } => write!(
                f,
                "family {entry_id} lacks full, current mirror/offline parity but still holds mirror_offline_parity"
            ),
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "family {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "family {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "family {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "family {entry_id} state {state:?} names no waiver")
            }
            Self::FamilyKindUncovered { family_kind } => write!(
                f,
                "artifact family kind {} has no covering row",
                family_kind.as_str()
            ),
            Self::ReleaseBlockingFamilyUncovered { family_ref } => write!(
                f,
                "release-blocking family {family_ref} has no covering row"
            ),
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking family {entry_id} is not declared in release_blocking_family_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => {
                write!(f, "promotion {declared:?} disagrees with computed {computed:?}")
            }
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with families"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "family {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for M5PublicationCertViolation {}

/// Loads the embedded publication-certification register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`M5PublicationCertRegister`].
pub fn current_m5_publication_cert_register() -> Result<M5PublicationCertRegister, serde_json::Error>
{
    serde_json::from_str(M5_PUBLICATION_CERT_JSON)
}

#[cfg(test)]
mod tests;
