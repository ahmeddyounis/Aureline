//! Typed M5 private evaluation/pilot evidence-pack register and no-overclaim guard.
//!
//! Where the claim-publication manifest is the single *public* source of truth
//! every claim-bearing surface reads, this register is the *private* layer that
//! packages enterprise and ecosystem evaluation/pilot materials on top of that
//! public baseline. For each enterprise/ecosystem lane it binds one [`EvalPack`]:
//!
//! - a named [`EvalPack::bundle_id`] and its [`EvalPackMirrorRef`]s (primary,
//!   offline, partner, air-gapped) — where the private bundle is mirrored,
//! - the [`EvalPackSupportContact`]s, the [`EvalPackKnownIssue`] deltas beyond the
//!   public known-limits, and the deployment caveats that travel with a pilot,
//! - and the public claim-publication manifest entry it reuses
//!   ([`EvalPack::claim_manifest_entry_ref`]) — its exact wording
//!   ([`EvalPack::public_claim_text`]), its support class
//!   ([`EvalPack::public_support_class`]), and its published label
//!   ([`EvalPack::public_claim_label`]), all of which are hard ceilings.
//!
//! The no-overclaim guard is the spine of the register: a pack may never publish a
//! greener [`EvalPack::pack_published_label`] than the public claim, never
//! advertise a broader [`EvalPack::pack_support_class`], and a published pack must
//! reuse the public wording verbatim. "Pilot-only" wording can never bypass a
//! support-class limit or stale evidence. Because every partner-facing destination
//! ([`EvalPackDestination`]) renders from the one pack, a narrowed pack downgrades
//! the evaluation pack, the pilot packet, the admin export, and the support export
//! at once.
//!
//! A pack that merely inherits a narrowed public claim
//! ([`EvalPackNarrowingReason::PublicClaimNarrowed`]) downgrades its partner
//! surfaces but does not itself hold promotion — the claim manifest already gates
//! the public claim. A *pack-layer* failure (a stale, missing, dropped, or unsigned
//! bundle mirror; stale or missing proof evidence; an expired validity window; an
//! over-claiming label or support class; a missing owner sign-off; or an expired
//! waiver) on a pack whose public claim is still at or above the cutline holds
//! promotion through an [`EvalPackStopRule`].
//!
//! This register reuses the canonical [`FamilyKind`] and [`SupportClass`]
//! vocabularies from the qualification/skew matrix, the evidence-state vocabulary
//! ([`M5ClaimReportState`]) from the claim-publication manifest, the [`ProofPacket`]
//! and [`FreshnessSloState`] freshness vocabulary from the stable claim manifest,
//! and the [`LaunchCutline`], [`StableClaimLevel`], [`OwnerSignoff`],
//! [`QualificationWaiver`], [`PromotionDecision`], and [`PromotionDecisionRecord`]
//! types from the stable claim matrix rather than minting local synonyms.
//!
//! The register is checked in at [`M5_EVALUATION_PILOT_PACKS_PATH`] and embedded
//! here, so this typed consumer and the CI gate agree on every pack without a cargo
//! build in CI. The model is metadata-only: every field is a typed state or an
//! opaque ref. It carries no raw artifacts, raw logs, signatures, or credential
//! material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::add_claim_publication_manifests_and_automatic_claim_narrowing_so_docs_release_notes_badges_cli_inspect_and_evaluation_packs_reuse_one_source_of_truth::M5ClaimReportState;
use crate::freeze_the_m5_qualification_row_support_window_skew_window_and_deprecation_packet_matrix::{
    FamilyKind, SupportClass,
};
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

/// Supported register schema version.
pub const M5_EVALUATION_PILOT_PACKS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const M5_EVALUATION_PILOT_PACKS_RECORD_KIND: &str =
    "ship_m5_private_evaluation_pilot_evidence_packs";

/// Repo-relative path to the checked-in register.
pub const M5_EVALUATION_PILOT_PACKS_PATH: &str =
    "artifacts/release/m5/ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes.json";

/// Embedded checked-in register JSON.
pub const M5_EVALUATION_PILOT_PACKS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/ship_private_evaluation_pilot_evidence_packs_with_bundle_ids_mirror_refs_known_issues_deltas_and_no_overclaim_guards_for_m5_enterprise_ecosystem_lanes.json"
));

/// The breadth rank of a support class; a broader class ranks higher. A pack may
/// never advertise a support class broader than the public claim it reuses.
const fn support_breadth(class: SupportClass) -> u8 {
    match class {
        SupportClass::FullSupport => 4,
        SupportClass::MaintenanceOnly => 3,
        SupportClass::SecurityOnly => 2,
        SupportClass::Limited => 1,
        SupportClass::EndOfLife => 0,
    }
}

/// The private distribution lane an evaluation/pilot pack rides.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalPackLaneKind {
    /// An enterprise pre-deployment technical evaluation pack.
    EnterpriseEvaluation,
    /// An enterprise pilot-deployment pack.
    EnterprisePilot,
    /// An ecosystem/partner integration pack.
    EcosystemPartner,
    /// A managed-service pilot pack.
    ManagedPilot,
}

impl EvalPackLaneKind {
    /// Every lane kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::EnterpriseEvaluation,
        Self::EnterprisePilot,
        Self::EcosystemPartner,
        Self::ManagedPilot,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnterpriseEvaluation => "enterprise_evaluation",
            Self::EnterprisePilot => "enterprise_pilot",
            Self::EcosystemPartner => "ecosystem_partner",
            Self::ManagedPilot => "managed_pilot",
        }
    }
}

/// Severity of a known-issues delta carried by a private pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalPackIssueSeverity {
    /// A blocking issue for the pilot scope.
    Blocker,
    /// A major issue with a documented workaround.
    Major,
    /// A minor issue.
    Minor,
}

impl EvalPackIssueSeverity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 3] = [Self::Blocker, Self::Major, Self::Minor];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocker => "blocker",
            Self::Major => "major",
            Self::Minor => "minor",
        }
    }
}

/// The kind of bundle mirror a private pack distributes through.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalPackMirrorKind {
    /// The primary distribution mirror.
    Primary,
    /// An offline bundle mirror.
    OfflineBundle,
    /// A partner-hosted mirror.
    PartnerMirror,
    /// An air-gapped transfer mirror.
    AirGapped,
}

impl EvalPackMirrorKind {
    /// Every mirror kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Primary,
        Self::OfflineBundle,
        Self::PartnerMirror,
        Self::AirGapped,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Primary => "primary",
            Self::OfflineBundle => "offline_bundle",
            Self::PartnerMirror => "partner_mirror",
            Self::AirGapped => "air_gapped",
        }
    }
}

/// A partner-facing surface that consumes a private evaluation/pilot pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalPackDestination {
    /// The partner-facing evaluation pack document.
    EvaluationPack,
    /// The partner pilot packet.
    PilotPacket,
    /// The admin/support export.
    AdminExport,
    /// The support-bundle export.
    SupportExport,
    /// The service-health surface.
    ServiceHealth,
    /// The release-center card.
    ReleaseCenter,
}

impl EvalPackDestination {
    /// Every destination, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EvaluationPack,
        Self::PilotPacket,
        Self::AdminExport,
        Self::SupportExport,
        Self::ServiceHealth,
        Self::ReleaseCenter,
    ];

    /// The destinations every pack must drive, so the evaluation pack, the pilot
    /// packet, the admin export, and the support export reconstruct from one source.
    pub const REQUIRED: [Self; 4] = [
        Self::EvaluationPack,
        Self::PilotPacket,
        Self::AdminExport,
        Self::SupportExport,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvaluationPack => "evaluation_pack",
            Self::PilotPacket => "pilot_packet",
            Self::AdminExport => "admin_export",
            Self::SupportExport => "support_export",
            Self::ServiceHealth => "service_health",
            Self::ReleaseCenter => "release_center",
        }
    }
}

/// Overall state a private pack earned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalPackState {
    /// The pack publishes the public claim's label; all private evidence is current.
    Published,
    /// The pack inherited a narrowed public claim.
    NarrowedPublicClaim,
    /// A bundle mirror or the proof packet is stale; the pack narrows.
    NarrowedStale,
    /// A bundle mirror or the proof packet is missing; the pack narrows.
    NarrowedMissing,
    /// The pack is withheld entirely (over-claim, dropped/unsigned mirror, expired
    /// window or waiver, or missing sign-off).
    Withheld,
}

impl EvalPackState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Published,
        Self::NarrowedPublicClaim,
        Self::NarrowedStale,
        Self::NarrowedMissing,
        Self::Withheld,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Published => "published",
            Self::NarrowedPublicClaim => "narrowed_public_claim",
            Self::NarrowedStale => "narrowed_stale",
            Self::NarrowedMissing => "narrowed_missing",
            Self::Withheld => "withheld",
        }
    }

    /// Whether the state lets the pack publish the public claim's label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Published)
    }
}

/// Closed reason a pack narrows below the public claim it reuses or a stop rule
/// fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalPackNarrowingReason {
    /// The reused public claim narrowed below the cutline.
    PublicClaimNarrowed,
    /// The pack proof packet breached its freshness SLO.
    EvidenceStale,
    /// No pack proof packet has been captured.
    EvidenceMissing,
    /// A bundle mirror is stale.
    MirrorStale,
    /// A bundle mirror is missing.
    MirrorMissing,
    /// A bundle mirror was dropped or revoked.
    MirrorDropped,
    /// A bundle mirror is unsigned.
    MirrorUnsigned,
    /// The pack's validity window has expired.
    ValidityWindowExpired,
    /// The pack would advertise a label or support class wider than the public claim.
    OverClaimBeyondPublicClaim,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
    /// A waiver the pack relied on has expired.
    WaiverExpired,
}

impl EvalPackNarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::PublicClaimNarrowed,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::MirrorStale,
        Self::MirrorMissing,
        Self::MirrorDropped,
        Self::MirrorUnsigned,
        Self::ValidityWindowExpired,
        Self::OverClaimBeyondPublicClaim,
        Self::OwnerSignoffMissing,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicClaimNarrowed => "public_claim_narrowed",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::MirrorStale => "mirror_stale",
            Self::MirrorMissing => "mirror_missing",
            Self::MirrorDropped => "mirror_dropped",
            Self::MirrorUnsigned => "mirror_unsigned",
            Self::ValidityWindowExpired => "validity_window_expired",
            Self::OverClaimBeyondPublicClaim => "over_claim_beyond_public_claim",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether a pack whose public claim is at or above the cutline carrying this
    /// reason holds promotion. A reason that merely inherits an upstream public-claim
    /// narrowing is gated by the claim manifest, not this register.
    pub const fn blocks_promotion(self) -> bool {
        !matches!(self, Self::PublicClaimNarrowed)
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalPackStopAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the pack to inherit the public claim.
    NarrowPack,
    /// Withhold the pack entirely.
    WithholdPack,
    /// Refresh the bundle mirror.
    RefreshMirror,
    /// Refresh the pack evidence packet.
    RefreshEvidence,
    /// Align the pack wording to the public claim.
    AlignCopyToPublicClaim,
    /// Renew the pack validity window.
    RenewValidityWindow,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl EvalPackStopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::HoldPublication,
        Self::NarrowPack,
        Self::WithholdPack,
        Self::RefreshMirror,
        Self::RefreshEvidence,
        Self::AlignCopyToPublicClaim,
        Self::RenewValidityWindow,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowPack => "narrow_pack",
            Self::WithholdPack => "withhold_pack",
            Self::RefreshMirror => "refresh_mirror",
            Self::RefreshEvidence => "refresh_evidence",
            Self::AlignCopyToPublicClaim => "align_copy_to_public_claim",
            Self::RenewValidityWindow => "renew_validity_window",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// The validity window the private pack is asserted within.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPackValidityWindow {
    /// UTC date the pack becomes valid.
    pub starts_at: String,
    /// UTC date the pack expires and must be renewed.
    pub expires_at: String,
    /// Whether the window has expired as of the register's `as_of` date.
    pub expired: bool,
}

/// One bundle mirror a private pack distributes through.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPackMirrorRef {
    /// Stable mirror id.
    pub mirror_id: String,
    /// The kind of mirror this ref names.
    pub mirror_kind: EvalPackMirrorKind,
    /// Opaque ref to where the bundle is mirrored. Empty only when the state is
    /// `missing`.
    pub location_ref: String,
    /// Opaque ref to the bundle digest this mirror carries.
    pub bundle_digest_ref: String,
    /// The mirror's freshness/integrity state.
    pub state: M5ClaimReportState,
}

/// A support contact a private pack names for an evaluation or pilot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPackSupportContact {
    /// The contact's role (e.g. evaluation lead, support escalation).
    pub role: String,
    /// Opaque ref to the contact.
    pub contact_ref: String,
}

/// One known-issues delta carried beyond the public known-limits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPackKnownIssue {
    /// Stable issue id.
    pub issue_id: String,
    /// Reviewable one-line summary of the issue.
    pub summary: String,
    /// The issue's severity for the pilot scope.
    pub severity: EvalPackIssueSeverity,
    /// Opaque ref to the documented workaround.
    pub workaround_ref: String,
    /// The public known-limit this delta extends, or null when it is net-new.
    #[serde(default)]
    pub public_known_limit_ref: Option<String>,
    /// Whether the issue is disclosed in the pack copy. Always required to be true.
    pub disclosed: bool,
}

/// One partner-facing destination's rendering of a private pack.
///
/// Each rendering reads the pack id, the published label, the support class, and
/// the exact wording from the one pack, so a narrowed pack downgrades every partner
/// surface at once and no surface can keep a greener private claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPackDestinationRendering {
    /// The destination this rendering targets.
    pub destination: EvalPackDestination,
    /// The pack id this destination renders from. Equals the register id.
    pub source_pack_id: String,
    /// The label rendered. Equals the pack's published label.
    pub rendered_label: StableClaimLevel,
    /// The support class rendered. Equals the pack's support class.
    pub rendered_support_class: SupportClass,
    /// The exact wording rendered. Equals the pack's claim text.
    pub rendered_claim_text: String,
    /// Whether the destination discloses the pack freshness. Always required.
    pub discloses_freshness: bool,
    /// Whether the destination discloses the known-issues delta. Required when any.
    pub discloses_known_issues: bool,
    /// Whether the destination discloses the deployment caveats. Required when any.
    pub discloses_caveats: bool,
}

/// One evaluation/pilot-pack stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPackStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched pack fires this rule.
    pub trigger_reason: EvalPackNarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: EvalPackStopAction,
    /// Whether firing this rule holds promotion.
    pub blocks_promotion: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One private evaluation/pilot evidence pack: the partner-facing layer over one
/// public claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPack {
    /// Stable pack id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The private distribution lane this pack rides.
    pub lane_kind: EvalPackLaneKind,
    /// The family this pack governs.
    pub family_kind: FamilyKind,
    /// The family ref this pack speaks about.
    pub family_ref: String,
    /// Reviewable one-line statement of the family.
    pub family_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// Ref to the claim-publication register this pack reuses.
    pub claim_manifest_ref: String,
    /// The claim-publication manifest entry id whose public claim this pack reuses.
    pub claim_manifest_entry_ref: String,
    /// The public claim's published label (the hard ceiling for this pack).
    pub public_claim_label: StableClaimLevel,
    /// The public claim's support class (the support ceiling for this pack).
    pub public_support_class: SupportClass,
    /// The public claim's exact wording, mirrored verbatim from the public claim.
    pub public_claim_text: String,
    /// The named private bundle id.
    pub bundle_id: String,
    /// The bundle mirrors this pack distributes through. Always at least one.
    pub mirror_refs: Vec<EvalPackMirrorRef>,
    /// The support contacts the pack names.
    pub support_contacts: Vec<EvalPackSupportContact>,
    /// The known-issues deltas beyond the public known-limits.
    #[serde(default)]
    pub known_issues_delta: Vec<EvalPackKnownIssue>,
    /// The deployment caveats that travel with the pilot.
    #[serde(default)]
    pub deployment_caveats: Vec<String>,
    /// The validity window the pack is asserted within.
    pub validity_window: EvalPackValidityWindow,
    /// Overall pack state earned.
    pub pack_state: EvalPackState,
    /// The support class the pack advertises. Never broader than the public class.
    pub pack_support_class: SupportClass,
    /// The lifecycle label the pack effectively publishes. Never greener than the
    /// public claim label.
    pub pack_published_label: StableClaimLevel,
    /// The partner-facing wording. A published pack reuses the public claim verbatim.
    pub pack_claim_text: String,
    /// The partner-facing destinations the pack drives. Always covers the required
    /// set (evaluation pack, pilot packet, admin export, support export).
    pub destinations: Vec<EvalPackDestinationRendering>,
    /// The pack proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional pack, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the pack below the public claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<EvalPackNarrowingReason>,
    /// Reviewable reason the pack carries this posture.
    pub rationale: String,
}

impl EvalPack {
    /// True when the pack's published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.pack_published_label.is_at_or_above_cutline()
    }

    /// True when the reused public claim is itself at or above the cutline.
    pub fn public_claim_holds_stable(&self) -> bool {
        self.public_claim_label.is_at_or_above_cutline()
    }

    /// True when the pack state lets the pack carry the public claim's label.
    pub fn holds_label(&self) -> bool {
        self.pack_state.holds_label()
    }

    /// True when a narrowing reason is active on the pack.
    pub fn has_active_reason(&self, reason: EvalPackNarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// True when the pack advertises a label or support class wider than the public
    /// claim it reuses.
    pub fn over_claims_public(&self) -> bool {
        self.pack_published_label.rank() > self.public_claim_label.rank()
            || support_breadth(self.pack_support_class) > support_breadth(self.public_support_class)
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPackSummary {
    /// Total number of packs.
    pub total_packs: usize,
    /// Distinct families covered.
    pub total_families: usize,
    /// Packs publishing at or above the cutline.
    pub packs_published: usize,
    /// Packs narrowed below the cutline.
    pub packs_narrowed: usize,
    /// Total release-blocking packs.
    pub release_blocking_total: usize,
    /// Release-blocking packs publishing at or above the cutline.
    pub release_blocking_published: usize,
    /// Release-blocking packs narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Enterprise-evaluation packs.
    pub enterprise_evaluation_packs: usize,
    /// Enterprise-pilot packs.
    pub enterprise_pilot_packs: usize,
    /// Ecosystem-partner packs.
    pub ecosystem_partner_packs: usize,
    /// Managed-pilot packs.
    pub managed_pilot_packs: usize,
    /// Notebook packs.
    pub notebook_packs: usize,
    /// AI/provider packs.
    pub ai_provider_packs: usize,
    /// Remote/helper packs.
    pub remote_helper_packs: usize,
    /// Companion packs.
    pub companion_packs: usize,
    /// Ecosystem packs.
    pub ecosystem_packs: usize,
    /// Managed-service packs.
    pub managed_service_packs: usize,
    /// Toolchain/runtime packs.
    pub toolchain_runtime_packs: usize,
    /// Packs in the `published` state.
    pub state_published: usize,
    /// Packs in the `narrowed_public_claim` state.
    pub state_narrowed_public_claim: usize,
    /// Packs in the `narrowed_stale` state.
    pub state_narrowed_stale: usize,
    /// Packs in the `narrowed_missing` state.
    pub state_narrowed_missing: usize,
    /// Packs in the `withheld` state.
    pub state_withheld: usize,
    /// Packs carrying at least one known-issues delta.
    pub packs_with_known_issues: usize,
    /// Total known-issues deltas across all packs.
    pub total_known_issues: usize,
    /// Packs carrying at least one deployment caveat.
    pub packs_with_deployment_caveats: usize,
    /// Total bundle mirror refs across all packs.
    pub total_mirror_refs: usize,
    /// Bundle mirrors that are current.
    pub mirrors_current: usize,
    /// Bundle mirrors that are stale.
    pub mirrors_stale: usize,
    /// Bundle mirrors that are missing.
    pub mirrors_missing: usize,
    /// Bundle mirrors that are dropped.
    pub mirrors_dropped: usize,
    /// Bundle mirrors that are unsigned.
    pub mirrors_unsigned: usize,
    /// Total support contacts across all packs.
    pub total_support_contacts: usize,
    /// Total partner-facing destination renderings across all packs.
    pub total_destinations: usize,
    /// Destination renderings that disclose the pack freshness.
    pub destinations_freshness_disclosed: usize,
    /// Destination renderings that disclose the known-issues delta.
    pub destinations_known_issues_disclosed: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all packs.
    pub total_active_narrowing_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalPackExportRow {
    /// Stable pack id.
    pub entry_id: String,
    /// The private distribution lane.
    pub lane_kind: EvalPackLaneKind,
    /// The family this pack governs.
    pub family_kind: FamilyKind,
    /// The family ref this pack speaks about.
    pub family_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The named private bundle id.
    pub bundle_id: String,
    /// The claim-publication manifest entry this pack reuses.
    pub claim_manifest_entry_ref: String,
    /// The public claim's published label (the ceiling).
    pub public_claim_label: StableClaimLevel,
    /// The pack's effective published label.
    pub pack_published_label: StableClaimLevel,
    /// Whether the pack publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall pack state earned.
    pub pack_state: EvalPackState,
    /// The support class the pack advertises.
    pub pack_support_class: SupportClass,
    /// The partner-facing wording every destination renders.
    pub pack_claim_text: String,
    /// The disclosed freshness state.
    pub freshness_state: FreshnessSloState,
    /// The deployment caveats that travel with the pack.
    pub deployment_caveats: Vec<String>,
    /// The number of known-issues deltas.
    pub known_issue_count: usize,
    /// The number of bundle mirrors.
    pub mirror_count: usize,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<EvalPackNarrowingReason>,
}

/// Export projection for evaluation-pack, pilot-packet, admin-export, and
/// support-export surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalPackExportProjection {
    /// Register identifier.
    pub register_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Promotion decision.
    pub promotion_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<EvalPackExportRow>,
}

/// The typed M5 private evaluation/pilot evidence-pack register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalPackRegister {
    /// Register schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable register identifier.
    pub register_id: String,
    /// Lifecycle status of this register artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the claim-publication register whose public claims this register reuses.
    pub claim_manifest_ref: String,
    /// Ref to the qualification/skew matrix that grounds the public claims.
    pub qualification_matrix_ref: String,
    /// Ref to the public known-limits matrix the known-issues deltas extend.
    pub known_limits_ref: String,
    /// Ref to the canonical M5 evidence index this register is recorded under.
    pub evidence_index_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed lane-kind vocabulary.
    pub lane_kinds: Vec<EvalPackLaneKind>,
    /// Closed family-kind vocabulary.
    pub family_kinds: Vec<FamilyKind>,
    /// Closed support-class vocabulary.
    pub support_classes: Vec<SupportClass>,
    /// Closed evidence-state (mirror/report) vocabulary.
    pub evidence_states: Vec<M5ClaimReportState>,
    /// Closed mirror-kind vocabulary.
    pub mirror_kinds: Vec<EvalPackMirrorKind>,
    /// Closed issue-severity vocabulary.
    pub issue_severities: Vec<EvalPackIssueSeverity>,
    /// Closed destination-kind vocabulary.
    pub destination_kinds: Vec<EvalPackDestination>,
    /// The required partner-facing destinations every pack must drive.
    pub required_destinations: Vec<EvalPackDestination>,
    /// Closed pack-state vocabulary.
    pub pack_states: Vec<EvalPackState>,
    /// Closed freshness-state vocabulary.
    pub freshness_states: Vec<FreshnessSloState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<EvalPackNarrowingReason>,
    /// Closed stop-action vocabulary.
    pub stop_actions: Vec<EvalPackStopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking family refs this register must cover.
    pub release_blocking_family_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<EvalPackStopRule>,
    /// Evaluation/pilot packs.
    pub packs: Vec<EvalPack>,
    /// Recorded promotion verdict.
    pub promotion: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: EvalPackSummary,
}

impl EvalPackRegister {
    /// Returns the pack registered for `entry_id`.
    pub fn pack(&self, entry_id: &str) -> Option<&EvalPack> {
        self.packs.iter().find(|p| p.entry_id == entry_id)
    }

    /// Returns the packs publishing at or above the cutline.
    pub fn packs_published(&self) -> Vec<&EvalPack> {
        self.packs.iter().filter(|p| p.publishes_stable()).collect()
    }

    /// Returns the packs narrowed below the cutline.
    pub fn packs_narrowed(&self) -> Vec<&EvalPack> {
        self.packs
            .iter()
            .filter(|p| !p.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking packs.
    pub fn release_blocking_packs(&self) -> Vec<&EvalPack> {
        self.packs.iter().filter(|p| p.release_blocking).collect()
    }

    /// Returns the packs for one family kind.
    pub fn packs_for_kind(&self, kind: FamilyKind) -> Vec<&EvalPack> {
        self.packs
            .iter()
            .filter(|p| p.family_kind == kind)
            .collect()
    }

    /// Returns the packs for one lane kind.
    pub fn packs_for_lane(&self, lane: EvalPackLaneKind) -> Vec<&EvalPack> {
        self.packs.iter().filter(|p| p.lane_kind == lane).collect()
    }

    /// Distinct families (by family ref) the register covers.
    pub fn families(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for p in &self.packs {
            set.insert(p.family_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched pack carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &EvalPackStopRule) -> bool {
        self.packs.iter().any(|p| {
            rule.applies_to_labels.contains(&p.public_claim_label)
                && p.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the promotion verdict from the packs and stop rules.
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

    /// Pack ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only packs whose public claim is at or above the cutline count: a pack whose
    /// public claim is already narrowed merely inherits the ceiling, and the claim
    /// manifest already holds promotion for it.
    pub fn computed_blocking_claim_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<EvalPackNarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_promotion && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for p in &self.packs {
            if p.public_claim_holds_stable()
                && p.active_narrowing_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(p.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Counts the bundle mirrors across all packs in `state`.
    fn mirrors_in(&self, state: M5ClaimReportState) -> usize {
        self.packs
            .iter()
            .flat_map(|p| p.mirror_refs.iter())
            .filter(|m| m.state == state)
            .count()
    }

    /// Recomputes the summary block from the packs and stop rules.
    pub fn computed_summary(&self) -> EvalPackSummary {
        let lane = |lane: EvalPackLaneKind| self.packs_for_lane(lane).len();
        let kind = |kind: FamilyKind| self.packs_for_kind(kind).len();
        let state =
            |state: EvalPackState| self.packs.iter().filter(|p| p.pack_state == state).count();
        let packets = |s: FreshnessSloState| {
            self.packs
                .iter()
                .filter(|p| p.proof_packet.slo_state == s)
                .count()
        };
        let release_blocking: Vec<&EvalPack> = self.release_blocking_packs();
        EvalPackSummary {
            total_packs: self.packs.len(),
            total_families: self.families().len(),
            packs_published: self.packs_published().len(),
            packs_narrowed: self.packs_narrowed().len(),
            release_blocking_total: release_blocking.len(),
            release_blocking_published: release_blocking
                .iter()
                .filter(|p| p.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|p| !p.publishes_stable())
                .count(),
            enterprise_evaluation_packs: lane(EvalPackLaneKind::EnterpriseEvaluation),
            enterprise_pilot_packs: lane(EvalPackLaneKind::EnterprisePilot),
            ecosystem_partner_packs: lane(EvalPackLaneKind::EcosystemPartner),
            managed_pilot_packs: lane(EvalPackLaneKind::ManagedPilot),
            notebook_packs: kind(FamilyKind::Notebook),
            ai_provider_packs: kind(FamilyKind::AiProvider),
            remote_helper_packs: kind(FamilyKind::RemoteHelper),
            companion_packs: kind(FamilyKind::Companion),
            ecosystem_packs: kind(FamilyKind::Ecosystem),
            managed_service_packs: kind(FamilyKind::ManagedService),
            toolchain_runtime_packs: kind(FamilyKind::ToolchainRuntime),
            state_published: state(EvalPackState::Published),
            state_narrowed_public_claim: state(EvalPackState::NarrowedPublicClaim),
            state_narrowed_stale: state(EvalPackState::NarrowedStale),
            state_narrowed_missing: state(EvalPackState::NarrowedMissing),
            state_withheld: state(EvalPackState::Withheld),
            packs_with_known_issues: self
                .packs
                .iter()
                .filter(|p| !p.known_issues_delta.is_empty())
                .count(),
            total_known_issues: self.packs.iter().map(|p| p.known_issues_delta.len()).sum(),
            packs_with_deployment_caveats: self
                .packs
                .iter()
                .filter(|p| !p.deployment_caveats.is_empty())
                .count(),
            total_mirror_refs: self.packs.iter().map(|p| p.mirror_refs.len()).sum(),
            mirrors_current: self.mirrors_in(M5ClaimReportState::Current),
            mirrors_stale: self.mirrors_in(M5ClaimReportState::Stale),
            mirrors_missing: self.mirrors_in(M5ClaimReportState::Missing),
            mirrors_dropped: self.mirrors_in(M5ClaimReportState::Dropped),
            mirrors_unsigned: self.mirrors_in(M5ClaimReportState::Unsigned),
            total_support_contacts: self.packs.iter().map(|p| p.support_contacts.len()).sum(),
            total_destinations: self.packs.iter().map(|p| p.destinations.len()).sum(),
            destinations_freshness_disclosed: self
                .packs
                .iter()
                .flat_map(|p| p.destinations.iter())
                .filter(|d| d.discloses_freshness)
                .count(),
            destinations_known_issues_disclosed: self
                .packs
                .iter()
                .flat_map(|p| p.destinations.iter())
                .filter(|d| d.discloses_known_issues)
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .packs
                .iter()
                .map(|p| p.active_narrowing_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export-safe projection that downstream partner surfaces render
    /// instead of cloning status text. The exact wording, freshness state, known-
    /// issue count, and caveats travel with every row, so the evaluation pack, the
    /// pilot packet, the admin export, and the support export reconstruct from one
    /// source.
    pub fn support_export_projection(&self) -> EvalPackExportProjection {
        EvalPackExportProjection {
            register_id: self.register_id.clone(),
            as_of: self.as_of.clone(),
            promotion_decision: self.promotion.decision,
            rows: self
                .packs
                .iter()
                .map(|p| EvalPackExportRow {
                    entry_id: p.entry_id.clone(),
                    lane_kind: p.lane_kind,
                    family_kind: p.family_kind,
                    family_ref: p.family_ref.clone(),
                    release_blocking: p.release_blocking,
                    bundle_id: p.bundle_id.clone(),
                    claim_manifest_entry_ref: p.claim_manifest_entry_ref.clone(),
                    public_claim_label: p.public_claim_label,
                    pack_published_label: p.pack_published_label,
                    publishes_stable: p.publishes_stable(),
                    pack_state: p.pack_state,
                    pack_support_class: p.pack_support_class,
                    pack_claim_text: p.pack_claim_text.clone(),
                    freshness_state: p.proof_packet.slo_state,
                    deployment_caveats: p.deployment_caveats.clone(),
                    known_issue_count: p.known_issues_delta.len(),
                    mirror_count: p.mirror_refs.len(),
                    active_narrowing_reasons: p.active_narrowing_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<EvalPackViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for p in &self.packs {
            if !seen.insert(p.entry_id.clone()) {
                violations.push(EvalPackViolation::DuplicateEntryId {
                    entry_id: p.entry_id.clone(),
                });
            }
            self.validate_pack(p, &mut violations);
        }
        if self.packs.is_empty() {
            violations.push(EvalPackViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_promotion(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(EvalPackViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<EvalPackViolation>) {
        if self.schema_version != M5_EVALUATION_PILOT_PACKS_SCHEMA_VERSION {
            violations.push(EvalPackViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != M5_EVALUATION_PILOT_PACKS_RECORD_KIND {
            violations.push(EvalPackViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("register_id", &self.register_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("qualification_matrix_ref", &self.qualification_matrix_ref),
            ("known_limits_ref", &self.known_limits_ref),
            ("evidence_index_ref", &self.evidence_index_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(EvalPackViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        let vocab: [(bool, &'static str); 13] = [
            (
                self.lifecycle_labels == StableClaimLevel::ALL.to_vec(),
                "lifecycle_labels",
            ),
            (
                self.lane_kinds == EvalPackLaneKind::ALL.to_vec(),
                "lane_kinds",
            ),
            (
                self.family_kinds == FamilyKind::ALL.to_vec(),
                "family_kinds",
            ),
            (
                self.support_classes == SupportClass::ALL.to_vec(),
                "support_classes",
            ),
            (
                self.evidence_states == M5ClaimReportState::ALL.to_vec(),
                "evidence_states",
            ),
            (
                self.mirror_kinds == EvalPackMirrorKind::ALL.to_vec(),
                "mirror_kinds",
            ),
            (
                self.issue_severities == EvalPackIssueSeverity::ALL.to_vec(),
                "issue_severities",
            ),
            (
                self.destination_kinds == EvalPackDestination::ALL.to_vec(),
                "destination_kinds",
            ),
            (
                self.required_destinations == EvalPackDestination::REQUIRED.to_vec(),
                "required_destinations",
            ),
            (
                self.pack_states == EvalPackState::ALL.to_vec(),
                "pack_states",
            ),
            (
                self.freshness_states == FreshnessSloState::ALL.to_vec(),
                "freshness_states",
            ),
            (
                self.narrowing_reasons == EvalPackNarrowingReason::ALL.to_vec(),
                "narrowing_reasons",
            ),
            (
                self.stop_actions == EvalPackStopAction::ALL.to_vec(),
                "stop_actions",
            ),
        ];
        for (ok, field) in vocab {
            if !ok {
                violations.push(EvalPackViolation::ClosedVocabularyMismatch { field });
            }
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(EvalPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(EvalPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(EvalPackViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(EvalPackViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<EvalPackViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(EvalPackViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(EvalPackViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(EvalPackViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(EvalPackViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            if rule.blocks_promotion != rule.trigger_reason.blocks_promotion() {
                violations.push(EvalPackViolation::StopRuleBlockingMismatch {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in EvalPackNarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations.push(EvalPackViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_pack(&self, p: &EvalPack, violations: &mut Vec<EvalPackViolation>) {
        for (field, value) in [
            ("entry_id", &p.entry_id),
            ("title", &p.title),
            ("family_ref", &p.family_ref),
            ("family_summary", &p.family_summary),
            ("claim_manifest_ref", &p.claim_manifest_ref),
            ("claim_manifest_entry_ref", &p.claim_manifest_entry_ref),
            ("public_claim_text", &p.public_claim_text),
            ("bundle_id", &p.bundle_id),
            ("pack_claim_text", &p.pack_claim_text),
            ("validity_window.starts_at", &p.validity_window.starts_at),
            ("validity_window.expires_at", &p.validity_window.expires_at),
            ("proof_packet.packet_id", &p.proof_packet.packet_id),
            ("proof_packet.packet_ref", &p.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &p.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &p.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &p.owner_signoff.owner_ref),
            ("rationale", &p.rationale),
        ] {
            if value.trim().is_empty() {
                violations.push(EvalPackViolation::EmptyField {
                    entry_id: p.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_mirrors(p, violations);
        self.validate_support_contacts(p, violations);
        self.validate_known_issues(p, violations);
        self.validate_destinations(p, violations);

        // The no-overclaim guard: the pack may never publish a greener label or a
        // broader support class than the public claim it reuses.
        if p.pack_published_label.rank() > p.public_claim_label.rank() {
            violations.push(EvalPackViolation::PackLabelExceedsPublicClaim {
                entry_id: p.entry_id.clone(),
                public: p.public_claim_label,
                pack: p.pack_published_label,
            });
        }
        if support_breadth(p.pack_support_class) > support_breadth(p.public_support_class) {
            violations.push(EvalPackViolation::PackSupportClassExceedsPublicClaim {
                entry_id: p.entry_id.clone(),
                public: p.public_support_class,
                pack: p.pack_support_class,
            });
        }

        if p.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(EvalPackViolation::EmptyField {
                entry_id: p.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !p.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(EvalPackViolation::FreshnessSloInconsistent {
                entry_id: p.entry_id.clone(),
            });
        }

        // A public claim narrowed below the cutline must name the inherited reason.
        if !p.public_claim_label.is_at_or_above_cutline()
            && !p.has_active_reason(EvalPackNarrowingReason::PublicClaimNarrowed)
        {
            violations.push(EvalPackViolation::PublicClaimNarrowedWithoutReason {
                entry_id: p.entry_id.clone(),
            });
        }

        // A limited support class must record at least one deployment caveat.
        if p.pack_support_class == SupportClass::Limited
            && p.deployment_caveats.iter().all(|c| c.trim().is_empty())
        {
            violations.push(EvalPackViolation::LimitedWithoutCaveat {
                entry_id: p.entry_id.clone(),
            });
        }

        if p.holds_label() {
            self.validate_published_pack(p, violations);
        } else {
            self.validate_narrowed_pack(p, violations);
        }
    }

    fn validate_mirrors(&self, p: &EvalPack, violations: &mut Vec<EvalPackViolation>) {
        if p.mirror_refs.is_empty() {
            violations.push(EvalPackViolation::PackWithoutMirror {
                entry_id: p.entry_id.clone(),
            });
        }
        let mut seen: BTreeSet<String> = BTreeSet::new();
        for mirror in &p.mirror_refs {
            if !seen.insert(mirror.mirror_id.clone()) {
                violations.push(EvalPackViolation::DuplicateMirrorId {
                    entry_id: p.entry_id.clone(),
                    mirror_id: mirror.mirror_id.clone(),
                });
            }
            if mirror.mirror_id.trim().is_empty() || mirror.bundle_digest_ref.trim().is_empty() {
                violations.push(EvalPackViolation::MirrorRefIncomplete {
                    entry_id: p.entry_id.clone(),
                });
            }
            // A present mirror carries a location ref; only a missing one carries none.
            if mirror.state != M5ClaimReportState::Missing && mirror.location_ref.trim().is_empty()
            {
                violations.push(EvalPackViolation::MirrorRefIncomplete {
                    entry_id: p.entry_id.clone(),
                });
            }
        }
    }

    fn validate_support_contacts(&self, p: &EvalPack, violations: &mut Vec<EvalPackViolation>) {
        for contact in &p.support_contacts {
            if contact.role.trim().is_empty() || contact.contact_ref.trim().is_empty() {
                violations.push(EvalPackViolation::SupportContactIncomplete {
                    entry_id: p.entry_id.clone(),
                });
            }
        }
    }

    fn validate_known_issues(&self, p: &EvalPack, violations: &mut Vec<EvalPackViolation>) {
        for issue in &p.known_issues_delta {
            if issue.issue_id.trim().is_empty()
                || issue.summary.trim().is_empty()
                || issue.workaround_ref.trim().is_empty()
            {
                violations.push(EvalPackViolation::KnownIssueIncomplete {
                    entry_id: p.entry_id.clone(),
                });
            }
            // A private pack may never hide a known issue from its partner copy.
            if !issue.disclosed {
                violations.push(EvalPackViolation::KnownIssueNotDisclosed {
                    entry_id: p.entry_id.clone(),
                    issue_id: issue.issue_id.clone(),
                });
            }
        }
    }

    fn validate_destinations(&self, p: &EvalPack, violations: &mut Vec<EvalPackViolation>) {
        let mut seen: BTreeSet<EvalPackDestination> = BTreeSet::new();
        let has_issues = !p.known_issues_delta.is_empty();
        let has_caveats = !p.deployment_caveats.is_empty();
        for rendering in &p.destinations {
            if !seen.insert(rendering.destination) {
                violations.push(EvalPackViolation::DuplicateDestination {
                    entry_id: p.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            // Every destination must render from this one pack, with the exact pack
            // label, support class, and wording, so a narrowed pack downgrades every
            // partner surface at once.
            if rendering.source_pack_id != self.register_id {
                violations.push(EvalPackViolation::DestinationSourceMismatch {
                    entry_id: p.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            if rendering.rendered_label != p.pack_published_label {
                violations.push(EvalPackViolation::DestinationLabelDrift {
                    entry_id: p.entry_id.clone(),
                    destination: rendering.destination,
                    rendered: rendering.rendered_label,
                    published: p.pack_published_label,
                });
            }
            if rendering.rendered_support_class != p.pack_support_class {
                violations.push(EvalPackViolation::DestinationSupportClassDrift {
                    entry_id: p.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            if rendering.rendered_claim_text != p.pack_claim_text {
                violations.push(EvalPackViolation::DestinationCopyDrift {
                    entry_id: p.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            if !rendering.discloses_freshness {
                violations.push(EvalPackViolation::DestinationFreshnessNotDisclosed {
                    entry_id: p.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            if has_issues && !rendering.discloses_known_issues {
                violations.push(EvalPackViolation::DestinationKnownIssuesNotDisclosed {
                    entry_id: p.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
            if has_caveats && !rendering.discloses_caveats {
                violations.push(EvalPackViolation::DestinationCaveatsNotDisclosed {
                    entry_id: p.entry_id.clone(),
                    destination: rendering.destination,
                });
            }
        }
        for destination in EvalPackDestination::REQUIRED {
            if !seen.contains(&destination) {
                violations.push(EvalPackViolation::RequiredDestinationUncovered {
                    entry_id: p.entry_id.clone(),
                    destination,
                });
            }
        }
    }

    fn validate_published_pack(&self, p: &EvalPack, violations: &mut Vec<EvalPackViolation>) {
        // A published pack publishes exactly the public claim's label, that label is
        // at or above the cutline, it reuses the public wording verbatim, names no
        // active reason, rides a captured within-SLO packet, all bundle mirrors are
        // current inside an open validity window, it is owner-signed, and it names at
        // least one support contact.
        if p.pack_published_label != p.public_claim_label {
            violations.push(EvalPackViolation::PublishedLabelNotPublicClaim {
                entry_id: p.entry_id.clone(),
                public: p.public_claim_label,
                pack: p.pack_published_label,
            });
        }
        if !p.publishes_stable() {
            violations.push(EvalPackViolation::PublishedStateNotStable {
                entry_id: p.entry_id.clone(),
                published: p.pack_published_label,
            });
        }
        if p.pack_claim_text != p.public_claim_text {
            violations.push(EvalPackViolation::PublishedCopyNotPublicClaim {
                entry_id: p.entry_id.clone(),
            });
        }
        if !p.active_narrowing_reasons.is_empty() {
            violations.push(EvalPackViolation::PublishedWithActiveGap {
                entry_id: p.entry_id.clone(),
            });
        }
        if !p.proof_packet.has_capture() {
            violations.push(EvalPackViolation::PublishedWithoutFreshPacket {
                entry_id: p.entry_id.clone(),
            });
        }
        if !p.proof_packet.slo_state.is_within_slo() {
            violations.push(EvalPackViolation::PublishedOnStalePacket {
                entry_id: p.entry_id.clone(),
                slo_state: p.proof_packet.slo_state,
            });
        }
        for mirror in &p.mirror_refs {
            if !mirror.state.is_current() {
                violations.push(EvalPackViolation::PublishedWithStaleMirror {
                    entry_id: p.entry_id.clone(),
                    mirror_id: mirror.mirror_id.clone(),
                    state: mirror.state,
                });
            }
        }
        if p.validity_window.expired {
            violations.push(EvalPackViolation::PublishedWithExpiredWindow {
                entry_id: p.entry_id.clone(),
            });
        }
        if !(p.owner_signoff.signed_off && p.owner_signoff.signed_at.is_some()) {
            violations.push(EvalPackViolation::PublishedWithoutSignoff {
                entry_id: p.entry_id.clone(),
            });
        }
        if p.support_contacts.is_empty() {
            violations.push(EvalPackViolation::PublishedWithoutSupportContact {
                entry_id: p.entry_id.clone(),
            });
        }
    }

    fn validate_narrowed_pack(&self, p: &EvalPack, violations: &mut Vec<EvalPackViolation>) {
        // A narrowing pack must drop below the cutline and name at least one active
        // reason.
        if p.publishes_stable() {
            violations.push(EvalPackViolation::NarrowedButPublishedStable {
                entry_id: p.entry_id.clone(),
                state: p.pack_state,
                published: p.pack_published_label,
            });
        }
        if p.active_narrowing_reasons.is_empty() {
            violations.push(EvalPackViolation::NarrowingWithoutReason {
                entry_id: p.entry_id.clone(),
                state: p.pack_state,
            });
        }

        // The narrowing state must be coherent with its active reasons.
        let any =
            |reasons: &[EvalPackNarrowingReason]| reasons.iter().any(|r| p.has_active_reason(*r));
        let coherent = match p.pack_state {
            EvalPackState::NarrowedPublicClaim => {
                any(&[EvalPackNarrowingReason::PublicClaimNarrowed])
            }
            EvalPackState::NarrowedStale => any(&[
                EvalPackNarrowingReason::EvidenceStale,
                EvalPackNarrowingReason::MirrorStale,
            ]),
            EvalPackState::NarrowedMissing => any(&[
                EvalPackNarrowingReason::EvidenceMissing,
                EvalPackNarrowingReason::MirrorMissing,
            ]),
            EvalPackState::Withheld => any(&[
                EvalPackNarrowingReason::MirrorDropped,
                EvalPackNarrowingReason::MirrorUnsigned,
                EvalPackNarrowingReason::ValidityWindowExpired,
                EvalPackNarrowingReason::OverClaimBeyondPublicClaim,
                EvalPackNarrowingReason::OwnerSignoffMissing,
                EvalPackNarrowingReason::WaiverExpired,
            ]),
            EvalPackState::Published => true,
        };
        if !coherent {
            violations.push(EvalPackViolation::StateReasonIncoherent {
                entry_id: p.entry_id.clone(),
                state: p.pack_state,
            });
        }

        // A stale or missing proof packet must name its matching reason.
        if p.proof_packet.slo_state == FreshnessSloState::Breached
            && !p.has_active_reason(EvalPackNarrowingReason::EvidenceStale)
        {
            violations.push(EvalPackViolation::StateWithoutReason {
                entry_id: p.entry_id.clone(),
                reason: EvalPackNarrowingReason::EvidenceStale,
            });
        }
        if p.proof_packet.slo_state == FreshnessSloState::Missing
            && !p.has_active_reason(EvalPackNarrowingReason::EvidenceMissing)
        {
            violations.push(EvalPackViolation::StateWithoutReason {
                entry_id: p.entry_id.clone(),
                reason: EvalPackNarrowingReason::EvidenceMissing,
            });
        }
        // A stale/missing/dropped/unsigned bundle mirror must name its reason.
        for mirror in &p.mirror_refs {
            let reason = match mirror.state {
                M5ClaimReportState::Stale => Some(EvalPackNarrowingReason::MirrorStale),
                M5ClaimReportState::Missing => Some(EvalPackNarrowingReason::MirrorMissing),
                M5ClaimReportState::Dropped => Some(EvalPackNarrowingReason::MirrorDropped),
                M5ClaimReportState::Unsigned => Some(EvalPackNarrowingReason::MirrorUnsigned),
                M5ClaimReportState::Current => None,
            };
            if let Some(reason) = reason {
                if !p.has_active_reason(reason) {
                    violations.push(EvalPackViolation::StateWithoutReason {
                        entry_id: p.entry_id.clone(),
                        reason,
                    });
                }
            }
        }
        // An expired validity window must name its reason.
        if p.validity_window.expired
            && !p.has_active_reason(EvalPackNarrowingReason::ValidityWindowExpired)
        {
            violations.push(EvalPackViolation::StateWithoutReason {
                entry_id: p.entry_id.clone(),
                reason: EvalPackNarrowingReason::ValidityWindowExpired,
            });
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<EvalPackViolation>) {
        let covered: BTreeSet<String> = self.packs.iter().map(|p| p.family_ref.clone()).collect();
        for declared in &self.release_blocking_family_refs {
            if !covered.contains(declared) {
                violations.push(EvalPackViolation::ReleaseBlockingFamilyUncovered {
                    family_ref: declared.clone(),
                });
            }
        }
        for p in &self.packs {
            if p.release_blocking && !self.release_blocking_family_refs.contains(&p.family_ref) {
                violations.push(EvalPackViolation::ReleaseBlockingPackNotDeclared {
                    entry_id: p.entry_id.clone(),
                });
            }
        }
    }

    fn validate_promotion(&self, violations: &mut Vec<EvalPackViolation>) {
        if self.promotion.promotion_gate.trim().is_empty() {
            violations.push(EvalPackViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.promotion.rationale.trim().is_empty() {
            violations.push(EvalPackViolation::EmptyField {
                entry_id: "<promotion>".to_owned(),
                field_name: "promotion.rationale",
            });
        }
        let computed = self.computed_promotion_decision();
        if self.promotion.decision != computed {
            violations.push(EvalPackViolation::PromotionDecisionInconsistent {
                declared: self.promotion.decision,
                computed,
            });
        }
        if self.promotion.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(EvalPackViolation::PromotionBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.promotion.blocking_claim_ids != self.computed_blocking_claim_ids() {
            violations.push(EvalPackViolation::PromotionBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the M5 evaluation/pilot-pack register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvalPackViolation {
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
    /// The register has no packs.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Pack or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A pack id appears more than once.
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
    /// A stop rule's blocking flag disagrees with its reason's semantics.
    StopRuleBlockingMismatch {
        /// Rule id.
        rule_id: String,
    },
    /// A narrowing reason has no stop rule watching for it.
    NarrowingReasonWithoutStopRule {
        /// Uncovered reason.
        reason: EvalPackNarrowingReason,
    },
    /// A pack has no bundle mirror.
    PackWithoutMirror {
        /// Pack id.
        entry_id: String,
    },
    /// A pack drives the same bundle mirror id twice.
    DuplicateMirrorId {
        /// Pack id.
        entry_id: String,
        /// Duplicated mirror id.
        mirror_id: String,
    },
    /// A bundle mirror ref is incomplete.
    MirrorRefIncomplete {
        /// Pack id.
        entry_id: String,
    },
    /// A support contact is incomplete.
    SupportContactIncomplete {
        /// Pack id.
        entry_id: String,
    },
    /// A known-issues delta is incomplete.
    KnownIssueIncomplete {
        /// Pack id.
        entry_id: String,
    },
    /// A known-issues delta is not disclosed in the pack copy.
    KnownIssueNotDisclosed {
        /// Pack id.
        entry_id: String,
        /// The undisclosed issue id.
        issue_id: String,
    },
    /// A pack drives the same destination twice.
    DuplicateDestination {
        /// Pack id.
        entry_id: String,
        /// Duplicated destination.
        destination: EvalPackDestination,
    },
    /// A pack does not drive a required partner-facing destination.
    RequiredDestinationUncovered {
        /// Pack id.
        entry_id: String,
        /// Uncovered destination.
        destination: EvalPackDestination,
    },
    /// A destination renders from a different pack id.
    DestinationSourceMismatch {
        /// Pack id.
        entry_id: String,
        /// Offending destination.
        destination: EvalPackDestination,
    },
    /// A destination renders a label that differs from the pack's.
    DestinationLabelDrift {
        /// Pack id.
        entry_id: String,
        /// Offending destination.
        destination: EvalPackDestination,
        /// Label the destination rendered.
        rendered: StableClaimLevel,
        /// Label the pack publishes.
        published: StableClaimLevel,
    },
    /// A destination renders a support class that differs from the pack's.
    DestinationSupportClassDrift {
        /// Pack id.
        entry_id: String,
        /// Offending destination.
        destination: EvalPackDestination,
    },
    /// A destination renders wording that drifted from the pack's.
    DestinationCopyDrift {
        /// Pack id.
        entry_id: String,
        /// Offending destination.
        destination: EvalPackDestination,
    },
    /// A destination does not disclose the pack freshness.
    DestinationFreshnessNotDisclosed {
        /// Pack id.
        entry_id: String,
        /// Offending destination.
        destination: EvalPackDestination,
    },
    /// A destination carries a known-issues delta it does not disclose.
    DestinationKnownIssuesNotDisclosed {
        /// Pack id.
        entry_id: String,
        /// Offending destination.
        destination: EvalPackDestination,
    },
    /// A destination carries caveats it does not disclose.
    DestinationCaveatsNotDisclosed {
        /// Pack id.
        entry_id: String,
        /// Offending destination.
        destination: EvalPackDestination,
    },
    /// A limited support class records no deployment caveat.
    LimitedWithoutCaveat {
        /// Pack id.
        entry_id: String,
    },
    /// The pack's published label is wider than the public claim it reuses.
    PackLabelExceedsPublicClaim {
        /// Pack id.
        entry_id: String,
        /// Public claim label.
        public: StableClaimLevel,
        /// Pack published label.
        pack: StableClaimLevel,
    },
    /// The pack's support class is broader than the public claim it reuses.
    PackSupportClassExceedsPublicClaim {
        /// Pack id.
        entry_id: String,
        /// Public support class.
        public: SupportClass,
        /// Pack support class.
        pack: SupportClass,
    },
    /// A public claim narrowed below the cutline does not name the inherited reason.
    PublicClaimNarrowedWithoutReason {
        /// Pack id.
        entry_id: String,
    },
    /// A published pack does not publish the public claim's label.
    PublishedLabelNotPublicClaim {
        /// Pack id.
        entry_id: String,
        /// Public claim label.
        public: StableClaimLevel,
        /// Pack published label.
        pack: StableClaimLevel,
    },
    /// A published pack does not publish at or above the cutline.
    PublishedStateNotStable {
        /// Pack id.
        entry_id: String,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A published pack does not reuse the public claim wording verbatim.
    PublishedCopyNotPublicClaim {
        /// Pack id.
        entry_id: String,
    },
    /// A published pack carries active narrowing reasons.
    PublishedWithActiveGap {
        /// Pack id.
        entry_id: String,
    },
    /// A published pack has no captured proof packet.
    PublishedWithoutFreshPacket {
        /// Pack id.
        entry_id: String,
    },
    /// A published pack rides a packet outside its freshness SLO.
    PublishedOnStalePacket {
        /// Pack id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A published pack rides a non-current bundle mirror.
    PublishedWithStaleMirror {
        /// Pack id.
        entry_id: String,
        /// Mirror id.
        mirror_id: String,
        /// Mirror state.
        state: M5ClaimReportState,
    },
    /// A published pack rides an expired validity window.
    PublishedWithExpiredWindow {
        /// Pack id.
        entry_id: String,
    },
    /// A published pack lacks owner sign-off.
    PublishedWithoutSignoff {
        /// Pack id.
        entry_id: String,
    },
    /// A published pack names no support contact.
    PublishedWithoutSupportContact {
        /// Pack id.
        entry_id: String,
    },
    /// A narrowing pack did not drop below the cutline.
    NarrowedButPublishedStable {
        /// Pack id.
        entry_id: String,
        /// Pack state.
        state: EvalPackState,
        /// Published label.
        published: StableClaimLevel,
    },
    /// A narrowing pack names no active reason.
    NarrowingWithoutReason {
        /// Pack id.
        entry_id: String,
        /// Pack state.
        state: EvalPackState,
    },
    /// A pack state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Pack id.
        entry_id: String,
        /// Pack state.
        state: EvalPackState,
    },
    /// A stale/missing/dropped/unsigned/expired input does not name its reason.
    StateWithoutReason {
        /// Pack id.
        entry_id: String,
        /// Reason the input state requires.
        reason: EvalPackNarrowingReason,
    },
    /// A release-blocking family ref has no covering pack.
    ReleaseBlockingFamilyUncovered {
        /// Family ref.
        family_ref: String,
    },
    /// A release-blocking pack is not declared in the release-blocking list.
    ReleaseBlockingPackNotDeclared {
        /// Pack id.
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
    /// The summary counts disagree with the packs.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Pack id.
        entry_id: String,
    },
}

impl fmt::Display for EvalPackViolation {
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
            Self::EmptyRegister => write!(f, "register has no packs"),
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
            Self::StopRuleBlockingMismatch { rule_id } => write!(
                f,
                "stop rule {rule_id} blocking flag disagrees with its reason"
            ),
            Self::NarrowingReasonWithoutStopRule { reason } => write!(
                f,
                "narrowing reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::PackWithoutMirror { entry_id } => {
                write!(f, "pack {entry_id} has no bundle mirror")
            }
            Self::DuplicateMirrorId {
                entry_id,
                mirror_id,
            } => write!(f, "pack {entry_id} drives mirror id {mirror_id} twice"),
            Self::MirrorRefIncomplete { entry_id } => {
                write!(f, "pack {entry_id} has an incomplete bundle mirror ref")
            }
            Self::SupportContactIncomplete { entry_id } => {
                write!(f, "pack {entry_id} has an incomplete support contact")
            }
            Self::KnownIssueIncomplete { entry_id } => {
                write!(f, "pack {entry_id} has an incomplete known-issues delta")
            }
            Self::KnownIssueNotDisclosed { entry_id, issue_id } => write!(
                f,
                "pack {entry_id} known issue {issue_id} is not disclosed in the pack copy"
            ),
            Self::DuplicateDestination {
                entry_id,
                destination,
            } => write!(
                f,
                "pack {entry_id} drives destination {} twice",
                destination.as_str()
            ),
            Self::RequiredDestinationUncovered {
                entry_id,
                destination,
            } => write!(
                f,
                "pack {entry_id} does not drive required destination {}",
                destination.as_str()
            ),
            Self::DestinationSourceMismatch {
                entry_id,
                destination,
            } => write!(
                f,
                "pack {entry_id} destination {} renders from a different pack id",
                destination.as_str()
            ),
            Self::DestinationLabelDrift {
                entry_id,
                destination,
                rendered,
                published,
            } => write!(
                f,
                "pack {entry_id} destination {} rendered {rendered:?} but pack publishes {published:?}",
                destination.as_str()
            ),
            Self::DestinationSupportClassDrift {
                entry_id,
                destination,
            } => write!(
                f,
                "pack {entry_id} destination {} support class drifted from the pack",
                destination.as_str()
            ),
            Self::DestinationCopyDrift {
                entry_id,
                destination,
            } => write!(
                f,
                "pack {entry_id} destination {} wording drifted from the pack",
                destination.as_str()
            ),
            Self::DestinationFreshnessNotDisclosed {
                entry_id,
                destination,
            } => write!(
                f,
                "pack {entry_id} destination {} does not disclose freshness",
                destination.as_str()
            ),
            Self::DestinationKnownIssuesNotDisclosed {
                entry_id,
                destination,
            } => write!(
                f,
                "pack {entry_id} destination {} does not disclose its known-issues delta",
                destination.as_str()
            ),
            Self::DestinationCaveatsNotDisclosed {
                entry_id,
                destination,
            } => write!(
                f,
                "pack {entry_id} destination {} does not disclose its caveats",
                destination.as_str()
            ),
            Self::LimitedWithoutCaveat { entry_id } => {
                write!(f, "pack {entry_id} is limited without a deployment caveat")
            }
            Self::PackLabelExceedsPublicClaim {
                entry_id,
                public,
                pack,
            } => write!(
                f,
                "pack {entry_id} published {pack:?} is greener than the public claim {public:?}"
            ),
            Self::PackSupportClassExceedsPublicClaim {
                entry_id,
                public,
                pack,
            } => write!(
                f,
                "pack {entry_id} support class {} is broader than the public claim {}",
                pack.as_str(),
                public.as_str()
            ),
            Self::PublicClaimNarrowedWithoutReason { entry_id } => write!(
                f,
                "pack {entry_id} public claim narrowed without public_claim_narrowed reason"
            ),
            Self::PublishedLabelNotPublicClaim {
                entry_id,
                public,
                pack,
            } => write!(
                f,
                "pack {entry_id} published label {pack:?} does not equal public claim {public:?}"
            ),
            Self::PublishedStateNotStable {
                entry_id,
                published,
            } => write!(
                f,
                "pack {entry_id} is published but publishes {published:?} below the cutline"
            ),
            Self::PublishedCopyNotPublicClaim { entry_id } => write!(
                f,
                "pack {entry_id} publishes wording that does not reuse the public claim verbatim"
            ),
            Self::PublishedWithActiveGap { entry_id } => {
                write!(f, "pack {entry_id} publishes with an active gap")
            }
            Self::PublishedWithoutFreshPacket { entry_id } => {
                write!(f, "pack {entry_id} publishes without a fresh packet")
            }
            Self::PublishedOnStalePacket {
                entry_id,
                slo_state,
            } => write!(f, "pack {entry_id} publishes on stale packet {slo_state:?}"),
            Self::PublishedWithStaleMirror {
                entry_id,
                mirror_id,
                state,
            } => write!(
                f,
                "pack {entry_id} publishes on mirror {mirror_id} in state {}",
                state.as_str()
            ),
            Self::PublishedWithExpiredWindow { entry_id } => {
                write!(f, "pack {entry_id} publishes on an expired validity window")
            }
            Self::PublishedWithoutSignoff { entry_id } => {
                write!(f, "pack {entry_id} publishes without owner signoff")
            }
            Self::PublishedWithoutSupportContact { entry_id } => {
                write!(f, "pack {entry_id} publishes without a support contact")
            }
            Self::NarrowedButPublishedStable {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "pack {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "pack {entry_id} state {state:?} narrows without active reason"
            ),
            Self::StateReasonIncoherent { entry_id, state } => write!(
                f,
                "pack {entry_id} state {state:?} is incoherent with its active reasons"
            ),
            Self::StateWithoutReason { entry_id, reason } => write!(
                f,
                "pack {entry_id} stale/missing/dropped/unsigned/expired input without {} reason",
                reason.as_str()
            ),
            Self::ReleaseBlockingFamilyUncovered { family_ref } => {
                write!(f, "release-blocking family {family_ref} has no covering pack")
            }
            Self::ReleaseBlockingPackNotDeclared { entry_id } => write!(
                f,
                "release-blocking pack {entry_id} is not declared in release_blocking_family_refs"
            ),
            Self::PromotionDecisionInconsistent { declared, computed } => write!(
                f,
                "promotion {declared:?} disagrees with computed {computed:?}"
            ),
            Self::PromotionBlockingSetMismatch { field } => {
                write!(f, "promotion {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with packs"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "pack {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for EvalPackViolation {}

/// Loads the embedded M5 private evaluation/pilot evidence-pack register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`EvalPackRegister`].
pub fn current_m5_evaluation_pilot_packs() -> Result<EvalPackRegister, serde_json::Error> {
    serde_json::from_str(M5_EVALUATION_PILOT_PACKS_JSON)
}

#[cfg(test)]
mod tests;
