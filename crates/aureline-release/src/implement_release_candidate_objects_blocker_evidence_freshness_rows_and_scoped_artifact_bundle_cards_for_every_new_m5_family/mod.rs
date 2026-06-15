//! Per-family release-candidate records, blocker/evidence-freshness rows, and
//! scoped artifact-bundle cards for every new M5 artifact family.
//!
//! Where the exact-build publication matrix
//! ([`crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix`])
//! freezes the publish-target and exact-build identity each M5 family publishes
//! under, this module is the *artifact-graph* layer beside it: it materializes
//! one durable [`M5FamilyReleaseCandidate`] per new M5 artifact family
//! ([`M5ArtifactFamilyKind`]) and joins, into a single inspectable graph, the
//! release-control truth an operator needs to read candidate scope and bundle
//! membership without unpacking raw archives.
//!
//! Each candidate binds one family to:
//!
//! - the release candidate it ships under
//!   ([`M5FamilyReleaseCandidate::release_candidate_ref`]) and its per-family
//!   scope — families are never flattened into one monolithic release blob,
//! - a scoped [`ScopedArtifactBundleCard`] that joins the eight bundle member
//!   classes ([`BundleMemberKind`]) — binaries, sidecars, symbols, docs packs,
//!   schemas, SDK artifacts, support packets, and compatibility rows — each by
//!   its immutable digest and the family's exact-build identity, and each
//!   carrying an explicit [`MemberPresence`] so a missing member is shown as
//!   `not_provided` or `partial` rather than disappearing from the view,
//! - first-class [`BlockerRow`] entries and [`EvidenceFreshnessRow`] rows, so a
//!   recorded blocker or a stale/missing piece of required evidence is surfaced
//!   as a blocker instead of dropping out of the bundle,
//! - the known issues published with the candidate, the rollback target, the
//!   proof packet and its freshness SLO ([`ProofPacket`]), and owner sign-off
//!   ([`OwnerSignoff`]),
//! - the public claim it backs, the active gap reasons ([`FamilyGapReason`])
//!   narrowing it, and the effective label it carries after narrowing
//!   ([`M5FamilyReleaseCandidate::published_label`]).
//!
//! The [`LaunchCutline`] (reused from the stable claim matrix) fixes the
//! boundary between a family that may publish at or above Stable and one that
//! must narrow below it. The [`FamilyStopRule`] set names the closed conditions
//! that gate publication when a bundle member is missing or partial, required
//! evidence is stale or missing, a blocker is open, the rollback target or
//! exact-build identity is absent, the proof packet aged out, a waiver expired,
//! or owner sign-off is missing. [`M5FamilyReleaseGraph::publication`] records
//! the resulting proceed/hold verdict, computed only from candidates whose
//! public claim is still at or above the cutline — a family whose claim is
//! already narrowed inherits that ceiling rather than blocking the whole train.
//!
//! Build success is never treated as publication readiness: a candidate only
//! holds its claimed label when its bundle is intact, its required evidence is
//! within SLO, it has no open blocker, its rollback target and exact-build
//! identity are recorded, its proof packet is within its freshness SLO, and it
//! is owner-signed. Any candidate that fails one of those narrows below the
//! cutline before promotion and must name every reason that forced it there.
//!
//! The graph is checked in at
//! `artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! candidate without a cargo build in CI. [`build_m5_family_release_graph`]
//! constructs the same graph in code; a test proves the two never drift.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no raw artifacts, raw logs, signatures, digests material, or
//! credential bodies — only digest refs and evidence refs.

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

/// Supported graph schema version.
pub const IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the graph.
pub const IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_RECORD_KIND: &str =
    "implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family";

/// Repo-relative path to the checked-in graph.
pub const IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_PATH: &str =
    "artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json";

/// Embedded checked-in graph JSON.
pub const IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/implement_release_candidate_objects_blocker_evidence_freshness_rows_and_scoped_artifact_bundle_cards_for_every_new_m5_family.json"
));

/// One of the eight artifact-bundle member classes a scoped bundle joins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BundleMemberKind {
    /// The primary binary, package, or pack archive.
    Binary,
    /// A sidecar payload that travels with the binary.
    Sidecar,
    /// Debug symbols or source maps for symbolication.
    Symbols,
    /// The documentation pack.
    DocsPack,
    /// The schema or contract export.
    Schema,
    /// The SDK, ABI, or extension-host artifact.
    SdkArtifact,
    /// The support packet for the family.
    SupportPacket,
    /// The compatibility report or certified matrix row.
    CompatibilityRow,
}

impl BundleMemberKind {
    /// Every member kind, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Binary,
        Self::Sidecar,
        Self::Symbols,
        Self::DocsPack,
        Self::Schema,
        Self::SdkArtifact,
        Self::SupportPacket,
        Self::CompatibilityRow,
    ];

    /// Stable token recorded in the graph.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Binary => "binary",
            Self::Sidecar => "sidecar",
            Self::Symbols => "symbols",
            Self::DocsPack => "docs_pack",
            Self::Schema => "schema",
            Self::SdkArtifact => "sdk_artifact",
            Self::SupportPacket => "support_packet",
            Self::CompatibilityRow => "compatibility_row",
        }
    }
}

/// Presence state of one bundle member.
///
/// A bundle never silently omits a member: every member kind is listed with one
/// of these states, so `not_provided` and `partial` stay visible to operators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemberPresence {
    /// The member is present, digested, and joined to the bundle.
    Provided,
    /// The member is present but incomplete (some content or digest is missing).
    Partial,
    /// The member is expected for this family but is not provided.
    NotProvided,
    /// The member does not apply to this family.
    NotApplicable,
}

impl MemberPresence {
    /// Every presence state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Provided,
        Self::Partial,
        Self::NotProvided,
        Self::NotApplicable,
    ];

    /// Stable token recorded in the graph.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Provided => "provided",
            Self::Partial => "partial",
            Self::NotProvided => "not_provided",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether the member holds its membership in an intact bundle.
    pub const fn holds_membership(self) -> bool {
        matches!(self, Self::Provided | Self::NotApplicable)
    }
}

/// Closed class of a first-class blocker recorded against a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockerClass {
    /// An open defect against the family's surface.
    OpenDefect,
    /// A required bundle member is missing or partial.
    MissingBundleMember,
    /// Required evidence is stale or missing.
    StaleOrMissingEvidence,
    /// A published known issue is unresolved and blocking.
    KnownIssueUnresolved,
    /// The rollback or revocation path is unprepared.
    RollbackOrRevocationGap,
    /// The exact-build identity linkage is absent.
    ExactBuildGap,
}

impl BlockerClass {
    /// Every blocker class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenDefect,
        Self::MissingBundleMember,
        Self::StaleOrMissingEvidence,
        Self::KnownIssueUnresolved,
        Self::RollbackOrRevocationGap,
        Self::ExactBuildGap,
    ];

    /// Stable token recorded in the graph.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDefect => "open_defect",
            Self::MissingBundleMember => "missing_bundle_member",
            Self::StaleOrMissingEvidence => "stale_or_missing_evidence",
            Self::KnownIssueUnresolved => "known_issue_unresolved",
            Self::RollbackOrRevocationGap => "rollback_or_revocation_gap",
            Self::ExactBuildGap => "exact_build_gap",
        }
    }
}

/// Closed reason a family candidate narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyGapReason {
    /// A required bundle member is `not_provided`.
    BundleMemberMissing,
    /// A required bundle member is `partial`.
    BundleMemberPartial,
    /// A required evidence row breached its freshness SLO.
    EvidenceStale,
    /// A required evidence row has no capture.
    EvidenceMissing,
    /// An open blocker blocks promotion.
    BlockerOpen,
    /// No rollback target (last-known-good) is recorded.
    RollbackTargetMissing,
    /// No exact-build identity ref is recorded for the family.
    ExactBuildIdentityMissing,
    /// The proof packet breached its freshness SLO.
    ProofPacketStale,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// A waiver the candidate relied on has expired.
    WaiverExpired,
    /// Required owner sign-off is missing.
    OwnerSignoffMissing,
}

impl FamilyGapReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::BundleMemberMissing,
        Self::BundleMemberPartial,
        Self::EvidenceStale,
        Self::EvidenceMissing,
        Self::BlockerOpen,
        Self::RollbackTargetMissing,
        Self::ExactBuildIdentityMissing,
        Self::ProofPacketStale,
        Self::ProofPacketMissing,
        Self::WaiverExpired,
        Self::OwnerSignoffMissing,
    ];

    /// Stable token recorded in the graph.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BundleMemberMissing => "bundle_member_missing",
            Self::BundleMemberPartial => "bundle_member_partial",
            Self::EvidenceStale => "evidence_stale",
            Self::EvidenceMissing => "evidence_missing",
            Self::BlockerOpen => "blocker_open",
            Self::RollbackTargetMissing => "rollback_target_missing",
            Self::ExactBuildIdentityMissing => "exact_build_identity_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::WaiverExpired => "waiver_expired",
            Self::OwnerSignoffMissing => "owner_signoff_missing",
        }
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FamilyRemediationAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the family's public claim below the cutline.
    NarrowScope,
    /// Provide the missing or partial bundle member.
    ProvideBundleMember,
    /// Refresh the stale evidence row.
    RefreshEvidence,
    /// Recapture the missing evidence row.
    RecaptureEvidence,
    /// Resolve the open blocker.
    ResolveBlocker,
    /// Record the rollback target (last-known-good).
    RecordRollbackTarget,
    /// Link the exact-build identity ref.
    LinkExactBuildIdentity,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Renew the expired waiver.
    RenewWaiver,
    /// Obtain the required owner sign-off.
    RequestOwnerSignoff,
}

impl FamilyRemediationAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::HoldPublication,
        Self::NarrowScope,
        Self::ProvideBundleMember,
        Self::RefreshEvidence,
        Self::RecaptureEvidence,
        Self::ResolveBlocker,
        Self::RecordRollbackTarget,
        Self::LinkExactBuildIdentity,
        Self::RefreshProofPacket,
        Self::RenewWaiver,
        Self::RequestOwnerSignoff,
    ];

    /// Stable token recorded in the graph.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowScope => "narrow_scope",
            Self::ProvideBundleMember => "provide_bundle_member",
            Self::RefreshEvidence => "refresh_evidence",
            Self::RecaptureEvidence => "recapture_evidence",
            Self::ResolveBlocker => "resolve_blocker",
            Self::RecordRollbackTarget => "record_rollback_target",
            Self::LinkExactBuildIdentity => "link_exact_build_identity",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RenewWaiver => "renew_waiver",
            Self::RequestOwnerSignoff => "request_owner_signoff",
        }
    }
}

/// One artifact-bundle member card: one member class joined by digest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BundleMemberCard {
    /// The member class this card describes.
    pub member_kind: BundleMemberKind,
    /// Presence state for this member.
    pub presence: MemberPresence,
    /// Artifact ref for the member. Empty only when `not_provided`/`not_applicable`.
    pub artifact_ref: String,
    /// Digest algorithm, such as `sha256`. Empty only when no digest is provided.
    pub digest_algorithm: String,
    /// Immutable digest ref binding the member to the artifact graph. Empty only
    /// when no digest is provided.
    pub digest_ref: String,
    /// Reviewable one-line statement of the member's state.
    pub summary: String,
}

impl BundleMemberCard {
    /// True when an immutable digest is recorded for the member.
    pub fn digest_present(&self) -> bool {
        !self.digest_algorithm.trim().is_empty() && !self.digest_ref.trim().is_empty()
    }
}

/// A scoped artifact-bundle card joining the eight member classes for one family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScopedArtifactBundleCard {
    /// Stable bundle id.
    pub bundle_id: String,
    /// Artifact graph ref this bundle belongs to.
    pub artifact_graph_ref: String,
    /// Exact-build identity ref the bundle is joined under.
    pub exact_build_identity_ref: String,
    /// One member card per [`BundleMemberKind`]; never silently omitted.
    pub members: Vec<BundleMemberCard>,
}

impl ScopedArtifactBundleCard {
    /// Returns the member card for `kind`, if present.
    pub fn member(&self, kind: BundleMemberKind) -> Option<&BundleMemberCard> {
        self.members.iter().find(|m| m.member_kind == kind)
    }

    /// True when a member is `not_provided`.
    pub fn has_missing_member(&self) -> bool {
        self.members
            .iter()
            .any(|m| m.presence == MemberPresence::NotProvided)
    }

    /// True when a member is `partial`.
    pub fn has_partial_member(&self) -> bool {
        self.members
            .iter()
            .any(|m| m.presence == MemberPresence::Partial)
    }

    /// Member kinds that are `not_provided`, sorted.
    pub fn missing_member_kinds(&self) -> Vec<BundleMemberKind> {
        let mut kinds: Vec<BundleMemberKind> = self
            .members
            .iter()
            .filter(|m| m.presence == MemberPresence::NotProvided)
            .map(|m| m.member_kind)
            .collect();
        kinds.sort();
        kinds
    }

    /// Member kinds that are `partial`, sorted.
    pub fn partial_member_kinds(&self) -> Vec<BundleMemberKind> {
        let mut kinds: Vec<BundleMemberKind> = self
            .members
            .iter()
            .filter(|m| m.presence == MemberPresence::Partial)
            .map(|m| m.member_kind)
            .collect();
        kinds.sort();
        kinds
    }

    /// True when every member holds its membership and every provided member is
    /// digested.
    pub fn is_intact(&self) -> bool {
        self.members.iter().all(|m| {
            m.presence.holds_membership()
                && (m.presence != MemberPresence::Provided || m.digest_present())
        })
    }
}

/// One first-class blocker recorded against a family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlockerRow {
    /// Stable blocker id.
    pub blocker_id: String,
    /// Blocker class.
    pub class: BlockerClass,
    /// Whether the blocker blocks promotion.
    pub blocks_promotion: bool,
    /// Ref to the blocker source (defect, advisory, or evidence row).
    pub source_ref: String,
    /// Reviewable one-line statement of the blocker.
    pub summary: String,
}

/// One first-class evidence-freshness row attached to a family candidate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceFreshnessRow {
    /// Stable evidence row id.
    pub evidence_id: String,
    /// Evidence kind token, such as `clean_room_rebuild` or `compatibility_report`.
    pub evidence_kind: String,
    /// Freshness-SLO state for this evidence.
    pub slo_state: FreshnessSloState,
    /// Whether missing or stale evidence here must block promotion.
    pub required_for_promotion: bool,
    /// Ref to the evidence.
    pub evidence_ref: String,
    /// UTC date the evidence was captured, or null when none exists yet.
    #[serde(default)]
    pub captured_at: Option<String>,
    /// Reviewable one-line statement of the evidence.
    pub summary: String,
}

impl EvidenceFreshnessRow {
    /// True when this required evidence is outside its freshness SLO and blocks.
    pub fn is_blocking(&self) -> bool {
        self.required_for_promotion && !self.slo_state.is_within_slo()
    }
}

/// One family stop rule: a closed condition that gates publication.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FamilyStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The gap reason whose presence on a watched candidate fires this rule.
    pub trigger_reason: FamilyGapReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: FamilyRemediationAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One per-family release candidate: one family's release-control truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5FamilyReleaseCandidate {
    /// Stable candidate id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The artifact family this candidate governs.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact family subject ref this candidate speaks about.
    pub artifact_ref: String,
    /// Reviewable one-line statement of the artifact family.
    pub artifact_summary: String,
    /// Whether the family is part of the release-blocking set.
    pub release_blocking: bool,
    /// The release candidate this family ships under (per-family scope).
    pub release_candidate_ref: String,
    /// The candidate version label.
    pub candidate_version: String,
    /// The channel family the candidate ships through.
    pub channel_family: String,
    /// The stable-claim-manifest entry id whose public claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the public claim publishes.
    pub claim_label: StableClaimLevel,
    /// The exact-build identity ref for this family. Empty only when narrowed.
    pub exact_build_identity_ref: String,
    /// The rollback target (last-known-good) ref. Empty only when narrowed.
    pub rollback_target_ref: String,
    /// The scoped artifact-bundle card joined for this family.
    pub bundle: ScopedArtifactBundleCard,
    /// First-class blocker rows.
    #[serde(default)]
    pub blockers: Vec<BlockerRow>,
    /// First-class evidence-freshness rows.
    pub evidence_rows: Vec<EvidenceFreshnessRow>,
    /// Known-issue refs published with the candidate.
    #[serde(default)]
    pub known_issue_refs: Vec<String>,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active gap reasons narrowing the candidate.
    #[serde(default)]
    pub active_gap_reasons: Vec<FamilyGapReason>,
    /// The lifecycle label the family effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Reviewable reason the candidate carries this posture.
    pub rationale: String,
}

impl M5FamilyReleaseCandidate {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the public claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when an open blocker blocks promotion.
    pub fn has_blocking_blocker(&self) -> bool {
        self.blockers.iter().any(|b| b.blocks_promotion)
    }

    /// True when a required evidence row breached its freshness SLO.
    pub fn has_stale_evidence(&self) -> bool {
        self.evidence_rows
            .iter()
            .any(|e| e.required_for_promotion && e.slo_state == FreshnessSloState::Breached)
    }

    /// True when a required evidence row is missing a capture.
    pub fn has_missing_evidence(&self) -> bool {
        self.evidence_rows
            .iter()
            .any(|e| e.required_for_promotion && e.slo_state == FreshnessSloState::Missing)
    }

    /// The gap reasons the candidate's structural state requires it to name.
    ///
    /// Returns one reason per structural condition that fails, so the graph can
    /// prove a narrowed candidate names every reason that forced it below the
    /// cutline.
    pub fn required_gap_reasons(&self) -> Vec<FamilyGapReason> {
        let mut reasons = Vec::new();
        if self.bundle.has_missing_member() {
            reasons.push(FamilyGapReason::BundleMemberMissing);
        }
        if self.bundle.has_partial_member() {
            reasons.push(FamilyGapReason::BundleMemberPartial);
        }
        if self.has_stale_evidence() {
            reasons.push(FamilyGapReason::EvidenceStale);
        }
        if self.has_missing_evidence() {
            reasons.push(FamilyGapReason::EvidenceMissing);
        }
        if self.has_blocking_blocker() {
            reasons.push(FamilyGapReason::BlockerOpen);
        }
        if self.rollback_target_ref.trim().is_empty() {
            reasons.push(FamilyGapReason::RollbackTargetMissing);
        }
        if self.exact_build_identity_ref.trim().is_empty() {
            reasons.push(FamilyGapReason::ExactBuildIdentityMissing);
        }
        if self.proof_packet.slo_state == FreshnessSloState::Breached {
            reasons.push(FamilyGapReason::ProofPacketStale);
        }
        if self.proof_packet.slo_state == FreshnessSloState::Missing {
            reasons.push(FamilyGapReason::ProofPacketMissing);
        }
        reasons
    }

    /// True when the candidate's structural state lets it hold its claimed label.
    pub fn holds_label(&self) -> bool {
        self.required_gap_reasons().is_empty()
    }

    /// True when a gap reason is active on the candidate.
    pub fn has_active_reason(&self, reason: FamilyGapReason) -> bool {
        self.active_gap_reasons.contains(&reason)
    }

    /// True when the candidate holds its claimed label via an active waiver.
    pub fn on_active_waiver(&self) -> bool {
        self.waiver
            .as_ref()
            .map(|w| !w.waiver_ref.trim().is_empty())
            .unwrap_or(false)
    }
}

/// Summary counts carried by the graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5FamilyReleaseGraphSummary {
    /// Total number of family candidates.
    pub total_candidates: usize,
    /// Distinct release candidates covered.
    pub total_release_candidates: usize,
    /// Candidates publishing a label at or above the cutline.
    pub candidates_backed: usize,
    /// Candidates narrowed below the cutline.
    pub candidates_narrowed: usize,
    /// Candidates holding their label via an active waiver.
    pub candidates_on_active_waiver: usize,
    /// Total release-blocking candidates.
    pub release_blocking_total: usize,
    /// Release-blocking candidates publishing at or above the cutline.
    pub release_blocking_backed: usize,
    /// Release-blocking candidates narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook-pack candidates.
    pub notebook_pack_candidates: usize,
    /// Request/data-asset candidates.
    pub request_data_asset_candidates: usize,
    /// Profiler/replay candidates.
    pub profiler_replay_candidates: usize,
    /// Framework/template candidates.
    pub framework_template_candidates: usize,
    /// Docs-pack candidates.
    pub docs_pack_candidates: usize,
    /// Model-pack candidates.
    pub model_pack_candidates: usize,
    /// Companion/offboarding candidates.
    pub companion_offboarding_candidates: usize,
    /// Managed-output candidates.
    pub managed_output_candidates: usize,
    /// Bundles whose members are all intact.
    pub bundles_intact: usize,
    /// Bundles with at least one `not_provided` member.
    pub bundles_with_missing_member: usize,
    /// Bundles with at least one `partial` member.
    pub bundles_with_partial_member: usize,
    /// Total blocker rows across all candidates.
    pub total_blockers: usize,
    /// Blocker rows that block promotion.
    pub blocking_blockers: usize,
    /// Total evidence-freshness rows across all candidates.
    pub total_evidence_rows: usize,
    /// Evidence rows that block promotion.
    pub blocking_evidence_rows: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active gap reasons across all candidates.
    pub total_active_gap_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FamilyReleaseExportRow {
    /// Stable candidate id.
    pub entry_id: String,
    /// The artifact family this candidate governs.
    pub family_kind: M5ArtifactFamilyKind,
    /// The artifact family subject ref.
    pub artifact_ref: String,
    /// Whether the family is release-blocking.
    pub release_blocking: bool,
    /// The release candidate this family ships under.
    pub release_candidate_ref: String,
    /// The stable-claim-manifest entry id whose public claim this family backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the candidate publishes at or above the cutline.
    pub publishes_stable: bool,
    /// The scoped bundle id.
    pub bundle_id: String,
    /// Whether the bundle is intact.
    pub bundle_intact: bool,
    /// Member presence for every member kind (never omitted).
    pub member_presence: Vec<(BundleMemberKind, MemberPresence)>,
    /// Member kinds that are `not_provided`.
    pub missing_member_kinds: Vec<BundleMemberKind>,
    /// Member kinds that are `partial`.
    pub partial_member_kinds: Vec<BundleMemberKind>,
    /// Total blocker count.
    pub total_blocker_count: usize,
    /// Blocking blocker count.
    pub blocking_blocker_count: usize,
    /// Evidence freshness per evidence id.
    pub evidence_freshness: Vec<(String, FreshnessSloState)>,
    /// Blocking evidence-row count.
    pub blocking_evidence_count: usize,
    /// Whether a rollback target is recorded.
    pub rollback_target_present: bool,
    /// Whether an exact-build identity is recorded.
    pub exact_build_identity_present: bool,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active gap reasons.
    pub active_gap_reasons: Vec<FamilyGapReason>,
}

/// Export projection for Help/About, release-center, support, and docs surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FamilyReleaseExportProjection {
    /// Graph identifier.
    pub graph_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<M5FamilyReleaseExportRow>,
}

/// The typed per-family release-candidate, blocker/evidence, and artifact-bundle
/// graph.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct M5FamilyReleaseGraph {
    /// Graph schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable graph identifier.
    pub graph_id: String,
    /// Lifecycle status of this graph artifact.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Ref to the stable claim manifest this graph ingests.
    pub claim_manifest_ref: String,
    /// Ref to the release artifact graph this graph publishes from.
    pub artifact_graph_ref: String,
    /// Ref to the M5 exact-build publication matrix this graph extends.
    pub publication_matrix_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed artifact-family-kind vocabulary.
    pub family_kinds: Vec<M5ArtifactFamilyKind>,
    /// Closed bundle-member-kind vocabulary.
    pub bundle_member_kinds: Vec<BundleMemberKind>,
    /// Closed member-presence vocabulary.
    pub member_presence_states: Vec<MemberPresence>,
    /// Closed blocker-class vocabulary.
    pub blocker_classes: Vec<BlockerClass>,
    /// Closed freshness-SLO-state vocabulary.
    pub freshness_states: Vec<FreshnessSloState>,
    /// Closed gap-reason vocabulary.
    pub gap_reasons: Vec<FamilyGapReason>,
    /// Closed remediation-action vocabulary.
    pub remediation_actions: Vec<FamilyRemediationAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking artifact refs this graph must cover.
    pub release_blocking_artifact_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<FamilyStopRule>,
    /// Family candidates.
    pub candidates: Vec<M5FamilyReleaseCandidate>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: M5FamilyReleaseGraphSummary,
}

impl M5FamilyReleaseGraph {
    /// Returns the candidate registered for `entry_id`.
    pub fn candidate(&self, entry_id: &str) -> Option<&M5FamilyReleaseCandidate> {
        self.candidates.iter().find(|c| c.entry_id == entry_id)
    }

    /// Returns the candidates publishing a label at or above the cutline.
    pub fn candidates_backed(&self) -> Vec<&M5FamilyReleaseCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.publishes_stable())
            .collect()
    }

    /// Returns the candidates narrowed below the cutline.
    pub fn candidates_narrowed(&self) -> Vec<&M5FamilyReleaseCandidate> {
        self.candidates
            .iter()
            .filter(|c| !c.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking candidates.
    pub fn release_blocking_candidates(&self) -> Vec<&M5FamilyReleaseCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.release_blocking)
            .collect()
    }

    /// Returns the candidates for one family kind.
    pub fn candidates_for_kind(
        &self,
        kind: M5ArtifactFamilyKind,
    ) -> Vec<&M5FamilyReleaseCandidate> {
        self.candidates
            .iter()
            .filter(|c| c.family_kind == kind)
            .collect()
    }

    /// Distinct release candidates (by ref) the graph covers.
    pub fn release_candidates(&self) -> Vec<String> {
        let mut set: BTreeSet<String> = BTreeSet::new();
        for c in &self.candidates {
            set.insert(c.release_candidate_ref.clone());
        }
        set.into_iter().collect()
    }

    /// True when `rule` fires: a watched candidate carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &FamilyStopRule) -> bool {
        self.candidates.iter().any(|c| {
            rule.applies_to_labels.contains(&c.claim_label)
                && c.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the candidates and stop rules.
    pub fn computed_publication_decision(&self) -> PromotionDecision {
        if self
            .stop_rules
            .iter()
            .any(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
        {
            PromotionDecision::Hold
        } else {
            PromotionDecision::Proceed
        }
    }

    /// Stop-rule ids that block publication and are currently firing, sorted.
    pub fn computed_blocking_rule_ids(&self) -> Vec<String> {
        let mut ids: Vec<String> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.rule_id.clone())
            .collect();
        ids.sort();
        ids
    }

    /// Candidate ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only candidates whose public claim is at or above the cutline count: a
    /// candidate whose claim is already canonically narrowed is not a
    /// *publication* blocker, it merely inherits the upstream ceiling.
    pub fn computed_blocking_candidate_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<FamilyGapReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
            .map(|rule| rule.trigger_reason)
            .collect();
        let mut ids: BTreeSet<String> = BTreeSet::new();
        for c in &self.candidates {
            if c.claim_holds_stable()
                && c.active_gap_reasons
                    .iter()
                    .any(|reason| blocking_triggers.contains(reason))
            {
                ids.insert(c.entry_id.clone());
            }
        }
        ids.into_iter().collect()
    }

    /// Recomputes the summary block from the candidates and stop rules.
    pub fn computed_summary(&self) -> M5FamilyReleaseGraphSummary {
        let packets = |state: FreshnessSloState| {
            self.candidates
                .iter()
                .filter(|c| c.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5ArtifactFamilyKind| self.candidates_for_kind(kind).len();
        let release_blocking: Vec<&M5FamilyReleaseCandidate> = self.release_blocking_candidates();
        M5FamilyReleaseGraphSummary {
            total_candidates: self.candidates.len(),
            total_release_candidates: self.release_candidates().len(),
            candidates_backed: self
                .candidates
                .iter()
                .filter(|c| c.publishes_stable())
                .count(),
            candidates_narrowed: self
                .candidates
                .iter()
                .filter(|c| !c.publishes_stable())
                .count(),
            candidates_on_active_waiver: self
                .candidates
                .iter()
                .filter(|c| c.on_active_waiver())
                .count(),
            release_blocking_total: release_blocking.len(),
            release_blocking_backed: release_blocking
                .iter()
                .filter(|c| c.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|c| !c.publishes_stable())
                .count(),
            notebook_pack_candidates: kind(M5ArtifactFamilyKind::NotebookPack),
            request_data_asset_candidates: kind(M5ArtifactFamilyKind::RequestDataAsset),
            profiler_replay_candidates: kind(M5ArtifactFamilyKind::ProfilerReplayArtifact),
            framework_template_candidates: kind(M5ArtifactFamilyKind::FrameworkTemplatePack),
            docs_pack_candidates: kind(M5ArtifactFamilyKind::DocsPack),
            model_pack_candidates: kind(M5ArtifactFamilyKind::ModelPack),
            companion_offboarding_candidates: kind(
                M5ArtifactFamilyKind::CompanionOffboardingPacket,
            ),
            managed_output_candidates: kind(M5ArtifactFamilyKind::ManagedOutput),
            bundles_intact: self
                .candidates
                .iter()
                .filter(|c| c.bundle.is_intact())
                .count(),
            bundles_with_missing_member: self
                .candidates
                .iter()
                .filter(|c| c.bundle.has_missing_member())
                .count(),
            bundles_with_partial_member: self
                .candidates
                .iter()
                .filter(|c| c.bundle.has_partial_member())
                .count(),
            total_blockers: self.candidates.iter().map(|c| c.blockers.len()).sum(),
            blocking_blockers: self
                .candidates
                .iter()
                .flat_map(|c| c.blockers.iter())
                .filter(|b| b.blocks_promotion)
                .count(),
            total_evidence_rows: self.candidates.iter().map(|c| c.evidence_rows.len()).sum(),
            blocking_evidence_rows: self
                .candidates
                .iter()
                .flat_map(|c| c.evidence_rows.iter())
                .filter(|e| e.is_blocking())
                .count(),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_gap_reasons: self
                .candidates
                .iter()
                .map(|c| c.active_gap_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About/release-center-safe projection that
    /// downstream surfaces render instead of cloning status text.
    pub fn support_export_projection(&self) -> M5FamilyReleaseExportProjection {
        M5FamilyReleaseExportProjection {
            graph_id: self.graph_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .candidates
                .iter()
                .map(|c| M5FamilyReleaseExportRow {
                    entry_id: c.entry_id.clone(),
                    family_kind: c.family_kind,
                    artifact_ref: c.artifact_ref.clone(),
                    release_blocking: c.release_blocking,
                    release_candidate_ref: c.release_candidate_ref.clone(),
                    claim_ref: c.claim_ref.clone(),
                    claim_label: c.claim_label,
                    published_label: c.published_label,
                    publishes_stable: c.publishes_stable(),
                    bundle_id: c.bundle.bundle_id.clone(),
                    bundle_intact: c.bundle.is_intact(),
                    member_presence: c
                        .bundle
                        .members
                        .iter()
                        .map(|m| (m.member_kind, m.presence))
                        .collect(),
                    missing_member_kinds: c.bundle.missing_member_kinds(),
                    partial_member_kinds: c.bundle.partial_member_kinds(),
                    total_blocker_count: c.blockers.len(),
                    blocking_blocker_count: c
                        .blockers
                        .iter()
                        .filter(|b| b.blocks_promotion)
                        .count(),
                    evidence_freshness: c
                        .evidence_rows
                        .iter()
                        .map(|e| (e.evidence_id.clone(), e.slo_state))
                        .collect(),
                    blocking_evidence_count: c
                        .evidence_rows
                        .iter()
                        .filter(|e| e.is_blocking())
                        .count(),
                    rollback_target_present: !c.rollback_target_ref.trim().is_empty(),
                    exact_build_identity_present: !c.exact_build_identity_ref.trim().is_empty(),
                    slo_state: c.proof_packet.slo_state,
                    active_gap_reasons: c.active_gap_reasons.clone(),
                })
                .collect(),
        }
    }

    /// Validates the graph, returning every violation found.
    pub fn validate(&self) -> Vec<M5FamilyReleaseGraphViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for c in &self.candidates {
            if !seen.insert(c.entry_id.clone()) {
                violations.push(M5FamilyReleaseGraphViolation::DuplicateEntryId {
                    entry_id: c.entry_id.clone(),
                });
            }
            self.validate_candidate(c, &mut violations);
        }
        if self.candidates.is_empty() {
            violations.push(M5FamilyReleaseGraphViolation::EmptyGraph);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(M5FamilyReleaseGraphViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<M5FamilyReleaseGraphViolation>) {
        if self.schema_version != IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_SCHEMA_VERSION {
            violations.push(M5FamilyReleaseGraphViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_RECORD_KIND {
            violations.push(M5FamilyReleaseGraphViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("graph_id", &self.graph_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("artifact_graph_ref", &self.artifact_graph_ref),
            ("publication_matrix_ref", &self.publication_matrix_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                    entry_id: "<graph>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.family_kinds != M5ArtifactFamilyKind::ALL.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "family_kinds",
            });
        }
        if self.bundle_member_kinds != BundleMemberKind::ALL.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "bundle_member_kinds",
            });
        }
        if self.member_presence_states != MemberPresence::ALL.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "member_presence_states",
            });
        }
        if self.blocker_classes != BlockerClass::ALL.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "blocker_classes",
            });
        }
        if self.freshness_states != FreshnessSloState::ALL.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "freshness_states",
            });
        }
        if self.gap_reasons != FamilyGapReason::ALL.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "gap_reasons",
            });
        }
        if self.remediation_actions != FamilyRemediationAction::ALL.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "remediation_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(M5FamilyReleaseGraphViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<M5FamilyReleaseGraphViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(M5FamilyReleaseGraphViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(M5FamilyReleaseGraphViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(M5FamilyReleaseGraphViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in FamilyGapReason::ALL {
            if !covered.contains(&reason) {
                violations.push(M5FamilyReleaseGraphViolation::GapReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_candidate(
        &self,
        c: &M5FamilyReleaseCandidate,
        violations: &mut Vec<M5FamilyReleaseGraphViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &c.entry_id),
            ("title", &c.title),
            ("artifact_ref", &c.artifact_ref),
            ("artifact_summary", &c.artifact_summary),
            ("release_candidate_ref", &c.release_candidate_ref),
            ("candidate_version", &c.candidate_version),
            ("channel_family", &c.channel_family),
            ("claim_ref", &c.claim_ref),
            ("rationale", &c.rationale),
            ("proof_packet.packet_id", &c.proof_packet.packet_id),
            ("proof_packet.packet_ref", &c.proof_packet.packet_ref),
            (
                "proof_packet.proof_index_ref",
                &c.proof_packet.proof_index_ref,
            ),
            (
                "proof_packet.freshness_slo.slo_register_ref",
                &c.proof_packet.freshness_slo.slo_register_ref,
            ),
            ("owner_signoff.owner_ref", &c.owner_signoff.owner_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                    entry_id: c.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_bundle(c, violations);
        self.validate_blockers(c, violations);
        self.validate_evidence_rows(c, violations);

        // The ceiling: no family may carry a label wider than the public claim's
        // canonical label.
        if c.published_label.rank() > c.claim_label.rank() {
            violations.push(M5FamilyReleaseGraphViolation::PublishedWiderThanClaim {
                entry_id: c.entry_id.clone(),
                claim: c.claim_label,
                published: c.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if c.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                entry_id: c.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !c.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(M5FamilyReleaseGraphViolation::FreshnessSloInconsistent {
                entry_id: c.entry_id.clone(),
            });
        }

        // A public claim whose canonical label is below the cutline forces the
        // family to inherit that ceiling and narrow.
        if !c.claim_holds_stable() {
            if c.holds_label() {
                violations.push(M5FamilyReleaseGraphViolation::HeldOnNarrowedClaim {
                    entry_id: c.entry_id.clone(),
                    claim: c.claim_label,
                });
            }
            if c.active_gap_reasons.is_empty() {
                violations.push(M5FamilyReleaseGraphViolation::NarrowingWithoutReason {
                    entry_id: c.entry_id.clone(),
                });
            }
        }

        let slo_state = c.proof_packet.slo_state;

        if c.holds_label() {
            // A backed family carries exactly the public claim's canonical label,
            // names no active gap reason, rides a captured within-SLO packet, and
            // is owner-signed.
            if c.published_label != c.claim_label {
                violations.push(M5FamilyReleaseGraphViolation::HeldLabelNotEqualClaim {
                    entry_id: c.entry_id.clone(),
                    claim: c.claim_label,
                    published: c.published_label,
                });
            }
            if !c.active_gap_reasons.is_empty() {
                violations.push(M5FamilyReleaseGraphViolation::HeldWithActiveGap {
                    entry_id: c.entry_id.clone(),
                });
            }
            if !c.proof_packet.has_capture() {
                violations.push(M5FamilyReleaseGraphViolation::HeldWithoutFreshPacket {
                    entry_id: c.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(M5FamilyReleaseGraphViolation::HeldOnStalePacket {
                    entry_id: c.entry_id.clone(),
                    slo_state,
                });
            }
            if !(c.owner_signoff.signed_off && c.owner_signoff.signed_at.is_some()) {
                violations.push(M5FamilyReleaseGraphViolation::HeldWithoutSignoff {
                    entry_id: c.entry_id.clone(),
                });
            }
        } else {
            // A narrowing family must drop the published label below the cutline
            // and name at least one active reason.
            if c.publishes_stable() {
                violations.push(M5FamilyReleaseGraphViolation::PublishedLabelNotNarrowed {
                    entry_id: c.entry_id.clone(),
                    published: c.published_label,
                });
            }
            if c.active_gap_reasons.is_empty() {
                violations.push(M5FamilyReleaseGraphViolation::NarrowingWithoutReason {
                    entry_id: c.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !c.has_active_reason(FamilyGapReason::ProofPacketStale)
            {
                violations.push(M5FamilyReleaseGraphViolation::BreachedPacketWithoutReason {
                    entry_id: c.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !c.has_active_reason(FamilyGapReason::ProofPacketMissing)
            {
                violations.push(M5FamilyReleaseGraphViolation::MissingPacketWithoutReason {
                    entry_id: c.entry_id.clone(),
                });
            }
        }

        // Every structural gap reason the candidate's state requires must be named
        // by an active gap reason so the narrowing is fully explained.
        for expected in c.required_gap_reasons() {
            if !c.has_active_reason(expected) {
                violations.push(M5FamilyReleaseGraphViolation::StructuralReasonIncoherent {
                    entry_id: c.entry_id.clone(),
                    expected_reason: expected,
                });
            }
        }
    }

    fn validate_bundle(
        &self,
        c: &M5FamilyReleaseCandidate,
        violations: &mut Vec<M5FamilyReleaseGraphViolation>,
    ) {
        let bundle = &c.bundle;
        for (field, value) in [
            ("bundle.bundle_id", &bundle.bundle_id),
            ("bundle.artifact_graph_ref", &bundle.artifact_graph_ref),
            (
                "bundle.exact_build_identity_ref",
                &bundle.exact_build_identity_ref,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                    entry_id: c.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        // Every member kind must be listed exactly once: never silently omitted.
        let mut seen: BTreeSet<BundleMemberKind> = BTreeSet::new();
        for member in &bundle.members {
            if !seen.insert(member.member_kind) {
                violations.push(M5FamilyReleaseGraphViolation::DuplicateBundleMember {
                    entry_id: c.entry_id.clone(),
                    member_kind: member.member_kind,
                });
            }
            if member.summary.trim().is_empty() {
                violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                    entry_id: c.entry_id.clone(),
                    field_name: "bundle.member.summary",
                });
            }
            self.validate_member_presence(c, member, violations);
        }
        for kind in BundleMemberKind::ALL {
            if !seen.contains(&kind) {
                violations.push(M5FamilyReleaseGraphViolation::BundleMemberOmitted {
                    entry_id: c.entry_id.clone(),
                    member_kind: kind,
                });
            }
        }
    }

    fn validate_member_presence(
        &self,
        c: &M5FamilyReleaseCandidate,
        member: &BundleMemberCard,
        violations: &mut Vec<M5FamilyReleaseGraphViolation>,
    ) {
        match member.presence {
            MemberPresence::Provided => {
                if member.artifact_ref.trim().is_empty() || !member.digest_present() {
                    violations.push(M5FamilyReleaseGraphViolation::ProvidedMemberWithoutDigest {
                        entry_id: c.entry_id.clone(),
                        member_kind: member.member_kind,
                    });
                }
            }
            MemberPresence::Partial => {
                if member.artifact_ref.trim().is_empty() {
                    violations.push(
                        M5FamilyReleaseGraphViolation::PartialMemberWithoutArtifact {
                            entry_id: c.entry_id.clone(),
                            member_kind: member.member_kind,
                        },
                    );
                }
            }
            MemberPresence::NotProvided | MemberPresence::NotApplicable => {
                if !member.artifact_ref.trim().is_empty() || member.digest_present() {
                    violations.push(M5FamilyReleaseGraphViolation::AbsentMemberWithArtifact {
                        entry_id: c.entry_id.clone(),
                        member_kind: member.member_kind,
                    });
                }
            }
        }
    }

    fn validate_blockers(
        &self,
        c: &M5FamilyReleaseCandidate,
        violations: &mut Vec<M5FamilyReleaseGraphViolation>,
    ) {
        let mut seen = BTreeSet::new();
        for blocker in &c.blockers {
            if !seen.insert(blocker.blocker_id.clone()) {
                violations.push(M5FamilyReleaseGraphViolation::DuplicateBlockerId {
                    entry_id: c.entry_id.clone(),
                    blocker_id: blocker.blocker_id.clone(),
                });
            }
            for (field, value) in [
                ("blocker.blocker_id", &blocker.blocker_id),
                ("blocker.source_ref", &blocker.source_ref),
                ("blocker.summary", &blocker.summary),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                        entry_id: c.entry_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_evidence_rows(
        &self,
        c: &M5FamilyReleaseCandidate,
        violations: &mut Vec<M5FamilyReleaseGraphViolation>,
    ) {
        if c.evidence_rows.is_empty() {
            violations.push(M5FamilyReleaseGraphViolation::NoEvidenceRows {
                entry_id: c.entry_id.clone(),
            });
        }
        let mut seen = BTreeSet::new();
        for row in &c.evidence_rows {
            if !seen.insert(row.evidence_id.clone()) {
                violations.push(M5FamilyReleaseGraphViolation::DuplicateEvidenceId {
                    entry_id: c.entry_id.clone(),
                    evidence_id: row.evidence_id.clone(),
                });
            }
            for (field, value) in [
                ("evidence.evidence_id", &row.evidence_id),
                ("evidence.evidence_kind", &row.evidence_kind),
                ("evidence.evidence_ref", &row.evidence_ref),
                ("evidence.summary", &row.summary),
            ] {
                if value.trim().is_empty() {
                    violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                        entry_id: c.entry_id.clone(),
                        field_name: field,
                    });
                }
            }
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<M5FamilyReleaseGraphViolation>) {
        let covered: BTreeSet<String> = self
            .candidates
            .iter()
            .map(|c| c.artifact_ref.clone())
            .collect();
        for declared in &self.release_blocking_artifact_refs {
            if !covered.contains(declared) {
                violations.push(
                    M5FamilyReleaseGraphViolation::ReleaseBlockingArtifactUncovered {
                        artifact_ref: declared.clone(),
                    },
                );
            }
        }
        for c in &self.candidates {
            if c.release_blocking
                && !self
                    .release_blocking_artifact_refs
                    .contains(&c.artifact_ref)
            {
                violations.push(
                    M5FamilyReleaseGraphViolation::ReleaseBlockingCandidateNotDeclared {
                        entry_id: c.entry_id.clone(),
                    },
                );
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<M5FamilyReleaseGraphViolation>) {
        if self.publication.promotion_gate.trim().is_empty() {
            violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(M5FamilyReleaseGraphViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                M5FamilyReleaseGraphViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(
                M5FamilyReleaseGraphViolation::PublicationBlockingSetMismatch {
                    field: "blocking_rule_ids",
                },
            );
        }
        if self.publication.blocking_claim_ids != self.computed_blocking_candidate_ids() {
            violations.push(
                M5FamilyReleaseGraphViolation::PublicationBlockingSetMismatch {
                    field: "blocking_claim_ids",
                },
            );
        }
    }
}

/// A validation violation for the M5 family release graph.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5FamilyReleaseGraphViolation {
    /// The graph carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the graph.
        actual: u32,
    },
    /// The graph carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the graph.
        actual: String,
    },
    /// A closed vocabulary or pinned cutline value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The graph has no candidates.
    EmptyGraph,
    /// The graph has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Candidate or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A candidate id appears more than once.
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
    /// A gap reason has no stop rule watching for it.
    GapReasonWithoutStopRule {
        /// Uncovered reason.
        reason: FamilyGapReason,
    },
    /// A bundle member kind is listed more than once.
    DuplicateBundleMember {
        /// Candidate id.
        entry_id: String,
        /// Duplicated member kind.
        member_kind: BundleMemberKind,
    },
    /// A bundle member kind is omitted (members must be shown, never dropped).
    BundleMemberOmitted {
        /// Candidate id.
        entry_id: String,
        /// Omitted member kind.
        member_kind: BundleMemberKind,
    },
    /// A `provided` member lacks an artifact ref or immutable digest.
    ProvidedMemberWithoutDigest {
        /// Candidate id.
        entry_id: String,
        /// Member kind.
        member_kind: BundleMemberKind,
    },
    /// A `partial` member lacks an artifact ref.
    PartialMemberWithoutArtifact {
        /// Candidate id.
        entry_id: String,
        /// Member kind.
        member_kind: BundleMemberKind,
    },
    /// A `not_provided`/`not_applicable` member carries an artifact ref or digest.
    AbsentMemberWithArtifact {
        /// Candidate id.
        entry_id: String,
        /// Member kind.
        member_kind: BundleMemberKind,
    },
    /// A blocker id appears more than once on a candidate.
    DuplicateBlockerId {
        /// Candidate id.
        entry_id: String,
        /// Duplicate blocker id.
        blocker_id: String,
    },
    /// A candidate has no evidence-freshness rows.
    NoEvidenceRows {
        /// Candidate id.
        entry_id: String,
    },
    /// An evidence id appears more than once on a candidate.
    DuplicateEvidenceId {
        /// Candidate id.
        entry_id: String,
        /// Duplicate evidence id.
        evidence_id: String,
    },
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Candidate id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A candidate holds a label while the public claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Candidate id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing candidate carries no active gap reason.
    NarrowingWithoutReason {
        /// Candidate id.
        entry_id: String,
    },
    /// A narrowing candidate did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Candidate id.
        entry_id: String,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A backed candidate carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Candidate id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A backed candidate has active gap reasons.
    HeldWithActiveGap {
        /// Candidate id.
        entry_id: String,
    },
    /// A backed candidate has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Candidate id.
        entry_id: String,
    },
    /// A backed candidate rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Candidate id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A backed candidate lacks owner sign-off.
    HeldWithoutSignoff {
        /// Candidate id.
        entry_id: String,
    },
    /// A narrowing candidate with a breached packet does not name the stale reason.
    BreachedPacketWithoutReason {
        /// Candidate id.
        entry_id: String,
    },
    /// A narrowing candidate with a missing packet does not name the missing reason.
    MissingPacketWithoutReason {
        /// Candidate id.
        entry_id: String,
    },
    /// A structural gap is not named by an active gap reason.
    StructuralReasonIncoherent {
        /// Candidate id.
        entry_id: String,
        /// Reason the structural gap requires.
        expected_reason: FamilyGapReason,
    },
    /// A release-blocking artifact ref has no covering candidate.
    ReleaseBlockingArtifactUncovered {
        /// Artifact ref.
        artifact_ref: String,
    },
    /// A release-blocking candidate is not declared in the release-blocking list.
    ReleaseBlockingCandidateNotDeclared {
        /// Candidate id.
        entry_id: String,
    },
    /// The declared publication decision disagrees with the computed one.
    PublicationDecisionInconsistent {
        /// Declared decision.
        declared: PromotionDecision,
        /// Computed decision.
        computed: PromotionDecision,
    },
    /// The declared publication blocking set disagrees with the computed one.
    PublicationBlockingSetMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// The summary counts disagree with the candidates.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Candidate id.
        entry_id: String,
    },
}

impl fmt::Display for M5FamilyReleaseGraphViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported graph schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported graph record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "graph {field} is not the canonical value")
            }
            Self::EmptyGraph => write!(f, "graph has no candidates"),
            Self::NoStopRules => write!(f, "graph has no stop rules"),
            Self::EmptyField {
                entry_id,
                field_name,
            } => write!(f, "{entry_id} has empty field {field_name}"),
            Self::DuplicateEntryId { entry_id } => write!(f, "duplicate entry id {entry_id}"),
            Self::DuplicateStopRuleId { rule_id } => write!(f, "duplicate stop rule id {rule_id}"),
            Self::StopRuleWithoutLabels { rule_id } => {
                write!(f, "stop rule {rule_id} watches no labels")
            }
            Self::GapReasonWithoutStopRule { reason } => write!(
                f,
                "gap reason {} has no stop rule watching for it",
                reason.as_str()
            ),
            Self::DuplicateBundleMember {
                entry_id,
                member_kind,
            } => write!(
                f,
                "candidate {entry_id} lists bundle member {} twice",
                member_kind.as_str()
            ),
            Self::BundleMemberOmitted {
                entry_id,
                member_kind,
            } => write!(
                f,
                "candidate {entry_id} omits bundle member {}",
                member_kind.as_str()
            ),
            Self::ProvidedMemberWithoutDigest {
                entry_id,
                member_kind,
            } => write!(
                f,
                "candidate {entry_id} provided member {} lacks an artifact ref or digest",
                member_kind.as_str()
            ),
            Self::PartialMemberWithoutArtifact {
                entry_id,
                member_kind,
            } => write!(
                f,
                "candidate {entry_id} partial member {} lacks an artifact ref",
                member_kind.as_str()
            ),
            Self::AbsentMemberWithArtifact {
                entry_id,
                member_kind,
            } => write!(
                f,
                "candidate {entry_id} absent member {} carries an artifact ref or digest",
                member_kind.as_str()
            ),
            Self::DuplicateBlockerId {
                entry_id,
                blocker_id,
            } => write!(f, "candidate {entry_id} lists blocker {blocker_id} twice"),
            Self::NoEvidenceRows { entry_id } => {
                write!(f, "candidate {entry_id} has no evidence-freshness rows")
            }
            Self::DuplicateEvidenceId {
                entry_id,
                evidence_id,
            } => write!(f, "candidate {entry_id} lists evidence {evidence_id} twice"),
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "candidate {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "candidate {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id } => {
                write!(f, "candidate {entry_id} narrows without an active reason")
            }
            Self::PublishedLabelNotNarrowed {
                entry_id,
                published,
            } => write!(
                f,
                "candidate {entry_id} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "candidate {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "candidate {entry_id} holds stable with an active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "candidate {entry_id} holds stable without a fresh packet")
            }
            Self::HeldOnStalePacket { entry_id, slo_state } => write!(
                f,
                "candidate {entry_id} holds stable on stale packet {}",
                slo_state.as_str()
            ),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "candidate {entry_id} holds stable without owner signoff")
            }
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "candidate {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "candidate {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::StructuralReasonIncoherent {
                entry_id,
                expected_reason,
            } => write!(
                f,
                "candidate {entry_id} structural gap requires reason {}",
                expected_reason.as_str()
            ),
            Self::ReleaseBlockingArtifactUncovered { artifact_ref } => {
                write!(f, "release-blocking artifact {artifact_ref} has no covering candidate")
            }
            Self::ReleaseBlockingCandidateNotDeclared { entry_id } => write!(
                f,
                "release-blocking candidate {entry_id} is not declared in release_blocking_artifact_refs"
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(f, "publication {declared:?} disagrees with computed {computed:?}")
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with candidates"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "candidate {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for M5FamilyReleaseGraphViolation {}

/// Loads the embedded M5 family release graph.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in graph no longer matches
/// [`M5FamilyReleaseGraph`].
pub fn current_m5_family_release_graph() -> Result<M5FamilyReleaseGraph, serde_json::Error> {
    serde_json::from_str(IMPLEMENT_M5_FAMILY_RELEASE_GRAPH_JSON)
}

mod builder;
pub use builder::build_m5_family_release_graph;

#[cfg(test)]
mod tests;
