//! Typed publication review-sheet register binding each M5 publication lane to a
//! shared version-bump proposal, a shared publish-target descriptor, auth-source
//! disclosure, a dry-run preview, rollout-ring truth, and downgrade automation.
//!
//! Where the per-family release graph speaks for the *release candidate* every M5
//! artifact family ships and the release-center visibility register speaks for the
//! *control surface* the release center exposes, this register speaks for the
//! *publication review sheet* every M5 publication lane exposes — the single
//! inspectable record a human reviewer reads before approving a publication and the
//! headless emitter consumes before executing one. Each [`PublicationReviewSheet`]
//! binds one lane to:
//!
//! - the stable claim it backs ([`PublicationReviewSheet::claim_ref`],
//!   [`PublicationReviewSheet::claim_label`]),
//! - a [`VersionBumpReview`] carrying the canonical
//!   [`VersionBumpProposal`](crate::release_center_model::VersionBumpProposal) —
//!   prior/target version, affected artifacts, compatibility notes — plus the
//!   migration flags, the [`PublicSurfaceImpact`] classification, and the
//!   public-surface impact summary, so a version bump can never hide migration or
//!   compatibility impact behind a version number,
//! - a [`PublishTargetReview`] carrying the canonical
//!   [`PublishTargetDescriptor`](crate::release_center_model::PublishTargetDescriptor)
//!   — target class, visibility, mutability, auth-source class, dry-run disclosure,
//!   rollout ring, mirror destination, rollback target — that human review and
//!   headless publication share verbatim, plus the [`AuthDisclosure`] proving the
//!   auth source and target scope are disclosed before any mutation and never
//!   inherited from ambient credentials,
//! - a [`ReviewParity`] record proving the human review and the headless plan
//!   share the same publish-target descriptor digest and the same diff-payload
//!   digest, so the sheet the reviewer approves is exactly the plan the emitter
//!   executes,
//! - an owner manifest ([`PublicationReviewSheet::owner_signoff`]), a
//!   [`ProofPacket`] and its freshness SLO, and an optional waiver,
//! - the overall sheet state earned ([`ReviewSheetState`]), the active narrowing
//!   reasons ([`NarrowingReason`]), and the effective label after narrowing
//!   ([`PublicationReviewSheet::published_label`]).
//!
//! The [`LaunchCutline`] fixes the boundary between a sheet that may publish a
//! Stable claim and one that must narrow below it. The
//! [`PublicationReviewStopRule`] set names the closed conditions that gate
//! publication — one per [`NarrowingReason`] — and
//! [`PublicationReviewRegister::publication`] records the proceed/hold verdict.
//!
//! The register is checked in at
//! `artifacts/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! publication lane without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref. It
//! carries no credential bodies, raw diff payloads, signatures, or provider
//! material.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_release_candidate_publish_target_artifact_bundle_and_exact_build_publication_matrix::M5ArtifactFamilyKind;
use crate::release_center_model::{PublishTargetDescriptor, VersionBumpProposal};
use crate::stable_claim_manifest::{FreshnessSloState, ProofPacket};
use crate::stable_claim_matrix::{
    LaunchCutline, OwnerSignoff, PromotionDecision, PromotionDecisionRecord, QualificationWaiver,
    StableClaimLevel,
};

mod builder;
pub use builder::build_publication_review_register;

/// Supported register schema version.
pub const PUBLICATION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the register.
pub const PUBLICATION_REVIEW_RECORD_KIND: &str =
    "ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes";

/// Repo-relative path to the checked-in register.
pub const PUBLICATION_REVIEW_PATH: &str =
    "artifacts/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.json";

/// Embedded checked-in register JSON.
pub const PUBLICATION_REVIEW_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/m5/ship_version_bump_proposals_publish_target_review_sheets_auth_source_disclosure_dry_run_previews_and_rollout_ring_truth_across_m5_publication_lanes.json"
));

/// The disclosed public-surface impact of a version bump.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicSurfaceImpact {
    /// No public-surface change ships with the bump.
    NoPublicChange,
    /// The change is backward-compatible for public consumers.
    BackwardCompatible,
    /// The change requires a migration; migration flags must be disclosed.
    MigrationRequired,
    /// The change is breaking for at least one public surface.
    Breaking,
}

impl PublicSurfaceImpact {
    /// Every impact, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoPublicChange,
        Self::BackwardCompatible,
        Self::MigrationRequired,
        Self::Breaking,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPublicChange => "no_public_change",
            Self::BackwardCompatible => "backward_compatible",
            Self::MigrationRequired => "migration_required",
            Self::Breaking => "breaking",
        }
    }

    /// Whether the impact requires migration flags to be disclosed.
    pub const fn requires_migration_flags(self) -> bool {
        matches!(self, Self::MigrationRequired | Self::Breaking)
    }
}

/// How a publication action's auth source and target scope are disclosed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthDisclosureState {
    /// The auth source and target scope are disclosed explicitly before mutation.
    ExplicitDisclosed,
    /// The auth source or target scope is not disclosed before mutation.
    Undisclosed,
    /// The publish flow would inherit ambient credentials invisibly.
    AmbientInherited,
}

impl AuthDisclosureState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::ExplicitDisclosed,
        Self::Undisclosed,
        Self::AmbientInherited,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitDisclosed => "explicit_disclosed",
            Self::Undisclosed => "undisclosed",
            Self::AmbientInherited => "ambient_inherited",
        }
    }

    /// Whether the state lets a lane publish: only explicit, non-ambient
    /// disclosure clears the auth gate.
    pub const fn holds(self) -> bool {
        matches!(self, Self::ExplicitDisclosed)
    }
}

/// Parity of the publish-target descriptor and diff payload across the human
/// review and the headless plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParityState {
    /// Human review and headless plan share the same descriptor and diff payload.
    Matched,
    /// Human review and headless plan diverge on the descriptor or diff payload.
    Divergent,
    /// One side has no recorded descriptor or diff payload to compare.
    Missing,
}

impl ParityState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 3] = [Self::Matched, Self::Divergent, Self::Missing];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Matched => "matched",
            Self::Divergent => "divergent",
            Self::Missing => "missing",
        }
    }

    /// Whether the state lets a lane publish: only matched parity clears the gate.
    pub const fn holds(self) -> bool {
        matches!(self, Self::Matched)
    }
}

/// Overall state a publication review sheet earned for its claimed label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSheetState {
    /// The version bump, publish target, auth disclosure, dry run, parity, proof
    /// packet, and owner sign-off all clear; the lane publishes its claim.
    Cleared,
    /// A review gap (impact, auth, dry-run, parity, rollout-ring, or rollback
    /// gap) narrows the lane below the cutline.
    ReviewGap,
    /// The proof packet has gone stale or is missing.
    Stale,
    /// Holds the claimed label only because an active, unexpired waiver covers a
    /// recorded gap.
    OnWaiver,
    /// The owner manifest is unsigned.
    OwnerUnsigned,
}

impl ReviewSheetState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Cleared,
        Self::ReviewGap,
        Self::Stale,
        Self::OnWaiver,
        Self::OwnerUnsigned,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Cleared => "cleared",
            Self::ReviewGap => "review_gap",
            Self::Stale => "stale",
            Self::OnWaiver => "on_waiver",
            Self::OwnerUnsigned => "owner_unsigned",
        }
    }

    /// Whether the state lets a lane carry its claimed label.
    pub const fn holds_label(self) -> bool {
        matches!(self, Self::Cleared | Self::OnWaiver)
    }

    /// Whether the state forces the lane below the claim's label.
    pub const fn forces_narrowing(self) -> bool {
        !self.holds_label()
    }
}

/// Closed reason a publication review sheet narrows or a stop rule fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowingReason {
    /// The version bump hides migration or compatibility impact behind a version
    /// number: the public-surface impact is undisclosed or its summary is empty.
    VersionImpactUndisclosed,
    /// The auth source or target scope is not disclosed before mutation.
    AuthSourceUndisclosed,
    /// The publish flow would inherit ambient credentials invisibly.
    AmbientCredentialInheritance,
    /// The dry-run preview is unavailable, stale, or failed.
    DryRunUnavailable,
    /// The publish-target descriptor diverges between human review and headless
    /// plan.
    DescriptorParityBroken,
    /// The diff payload diverges between human review and headless plan.
    DiffPayloadParityBroken,
    /// The rollout ring is not disclosed.
    RolloutRingUndisclosed,
    /// No rollback target is recorded before publication.
    RollbackTargetMissing,
    /// The proof packet is stale.
    ProofPacketStale,
    /// The proof packet is missing.
    ProofPacketMissing,
    /// The owner manifest is unsigned.
    OwnerManifestUnsigned,
    /// A waiver the lane relied on has expired.
    WaiverExpired,
}

impl NarrowingReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::VersionImpactUndisclosed,
        Self::AuthSourceUndisclosed,
        Self::AmbientCredentialInheritance,
        Self::DryRunUnavailable,
        Self::DescriptorParityBroken,
        Self::DiffPayloadParityBroken,
        Self::RolloutRingUndisclosed,
        Self::RollbackTargetMissing,
        Self::ProofPacketStale,
        Self::ProofPacketMissing,
        Self::OwnerManifestUnsigned,
        Self::WaiverExpired,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VersionImpactUndisclosed => "version_impact_undisclosed",
            Self::AuthSourceUndisclosed => "auth_source_undisclosed",
            Self::AmbientCredentialInheritance => "ambient_credential_inheritance",
            Self::DryRunUnavailable => "dry_run_unavailable",
            Self::DescriptorParityBroken => "descriptor_parity_broken",
            Self::DiffPayloadParityBroken => "diff_payload_parity_broken",
            Self::RolloutRingUndisclosed => "rollout_ring_undisclosed",
            Self::RollbackTargetMissing => "rollback_target_missing",
            Self::ProofPacketStale => "proof_packet_stale",
            Self::ProofPacketMissing => "proof_packet_missing",
            Self::OwnerManifestUnsigned => "owner_manifest_unsigned",
            Self::WaiverExpired => "waiver_expired",
        }
    }

    /// Whether this reason is a review-sheet gap (rather than a proof-packet or
    /// owner-manifest gap). The [`ReviewSheetState::ReviewGap`] state must name at
    /// least one of these.
    pub const fn is_review_gap(self) -> bool {
        matches!(
            self,
            Self::VersionImpactUndisclosed
                | Self::AuthSourceUndisclosed
                | Self::AmbientCredentialInheritance
                | Self::DryRunUnavailable
                | Self::DescriptorParityBroken
                | Self::DiffPayloadParityBroken
                | Self::RolloutRingUndisclosed
                | Self::RollbackTargetMissing
                | Self::WaiverExpired
        )
    }
}

/// Default action a stop rule prescribes when it fires.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StopAction {
    /// Hold publication until the condition clears.
    HoldPublication,
    /// Narrow the claim below the cutline.
    NarrowLabel,
    /// Disclose the version bump's migration and compatibility impact.
    DiscloseVersionImpact,
    /// Disclose the auth source and target scope before mutation.
    DiscloseAuthSource,
    /// Rebind the publish flow to an explicit, non-ambient auth source.
    RebindNonAmbientAuth,
    /// Refresh the dry-run preview.
    RefreshDryRun,
    /// Reconcile the publish-target descriptor across review and plan.
    ReconcileDescriptorParity,
    /// Reconcile the diff payload across review and plan.
    ReconcileDiffPayload,
    /// Disclose the rollout ring.
    DiscloseRolloutRing,
    /// Record a rollback target before publication.
    RecordRollbackTarget,
    /// Refresh the proof packet.
    RefreshProofPacket,
    /// Obtain the required owner-manifest sign-off.
    RequestOwnerSignoff,
    /// Renew the expired waiver.
    RenewWaiver,
}

impl StopAction {
    /// Every action, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::HoldPublication,
        Self::NarrowLabel,
        Self::DiscloseVersionImpact,
        Self::DiscloseAuthSource,
        Self::RebindNonAmbientAuth,
        Self::RefreshDryRun,
        Self::ReconcileDescriptorParity,
        Self::ReconcileDiffPayload,
        Self::DiscloseRolloutRing,
        Self::RecordRollbackTarget,
        Self::RefreshProofPacket,
        Self::RequestOwnerSignoff,
        Self::RenewWaiver,
    ];

    /// Stable token recorded in the register.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HoldPublication => "hold_publication",
            Self::NarrowLabel => "narrow_label",
            Self::DiscloseVersionImpact => "disclose_version_impact",
            Self::DiscloseAuthSource => "disclose_auth_source",
            Self::RebindNonAmbientAuth => "rebind_non_ambient_auth",
            Self::RefreshDryRun => "refresh_dry_run",
            Self::ReconcileDescriptorParity => "reconcile_descriptor_parity",
            Self::ReconcileDiffPayload => "reconcile_diff_payload",
            Self::DiscloseRolloutRing => "disclose_rollout_ring",
            Self::RecordRollbackTarget => "record_rollback_target",
            Self::RefreshProofPacket => "refresh_proof_packet",
            Self::RequestOwnerSignoff => "request_owner_signoff",
            Self::RenewWaiver => "renew_waiver",
        }
    }
}

/// One migration flag disclosed by a version bump.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MigrationFlag {
    /// Stable flag id.
    pub flag_id: String,
    /// Reviewable one-line summary of the migration this flag covers.
    pub summary: String,
    /// Whether this migration must complete before consumers upgrade.
    pub blocking: bool,
    /// Ref to the migration guide or compatibility note backing the flag.
    pub migration_ref: String,
}

/// The version-bump section of a publication review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VersionBumpReview {
    /// The canonical version-bump proposal shared with the release-center model.
    pub proposal: VersionBumpProposal,
    /// The disclosed public-surface impact class.
    pub public_surface_impact: PublicSurfaceImpact,
    /// Whether the migration and compatibility impact is disclosed (never hidden
    /// behind a version number).
    pub impact_disclosed: bool,
    /// Reviewable public-surface impact summary. Non-empty on a disclosed bump.
    pub impact_summary: String,
    /// Migration flags disclosed by the bump.
    #[serde(default)]
    pub migration_flags: Vec<MigrationFlag>,
}

/// The auth-source disclosure of a publish-target review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthDisclosure {
    /// How the auth source and target scope are disclosed.
    pub state: AuthDisclosureState,
    /// Ref to the disclosed auth source (vault token, OIDC identity, receipt).
    pub auth_source_ref: String,
    /// Whether the auth source is disclosed before any channel/mirror/docs/market
    /// mutation runs.
    pub disclosed_before_mutation: bool,
    /// Whether the target scope is disclosed before any mutation runs.
    pub target_scope_disclosed: bool,
    /// Reviewable disclosure summary.
    pub summary: String,
}

/// The publish-target section of a publication review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublishTargetReview {
    /// The canonical publish-target descriptor shared with the release-center
    /// model. Human review and headless publication consume this verbatim.
    pub descriptor: PublishTargetDescriptor,
    /// The auth-source and target-scope disclosure.
    pub auth_disclosure: AuthDisclosure,
    /// Whether the rollout ring is disclosed to the operator.
    pub rollout_ring_disclosed: bool,
    /// Ref to the mirror or registry destination the publication reaches.
    pub mirror_destination_ref: String,
}

/// Proof that the human review and the headless plan share the same publish-target
/// descriptor and the same diff payload.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewParity {
    /// Ref to the human review record.
    pub human_review_ref: String,
    /// Ref to the headless publication plan record.
    pub headless_plan_ref: String,
    /// Descriptor digest the human review approved.
    pub human_descriptor_digest: String,
    /// Descriptor digest the headless plan would execute.
    pub headless_descriptor_digest: String,
    /// Ref to the shared diff payload.
    pub diff_payload_ref: String,
    /// Diff-payload digest the human review approved.
    pub human_diff_payload_digest: String,
    /// Diff-payload digest the headless plan would execute.
    pub headless_diff_payload_digest: String,
    /// The parity state earned.
    pub parity_state: ParityState,
}

impl ReviewParity {
    /// Whether the descriptor digest matches across review and plan.
    pub fn descriptor_digests_match(&self) -> bool {
        !self.human_descriptor_digest.trim().is_empty()
            && self.human_descriptor_digest == self.headless_descriptor_digest
    }

    /// Whether the diff-payload digest matches across review and plan.
    pub fn diff_payload_digests_match(&self) -> bool {
        !self.human_diff_payload_digest.trim().is_empty()
            && self.human_diff_payload_digest == self.headless_diff_payload_digest
    }
}

/// One publication review-sheet stop rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationReviewStopRule {
    /// Stable rule id.
    pub rule_id: String,
    /// Human-readable title.
    pub title: String,
    /// The narrowing reason whose presence on a watched lane fires this rule.
    pub trigger_reason: NarrowingReason,
    /// Public-claim labels this rule watches.
    pub applies_to_labels: Vec<StableClaimLevel>,
    /// Default action prescribed when the rule fires.
    pub default_action: StopAction,
    /// Whether firing this rule blocks publication.
    pub blocks_publication: bool,
    /// Reviewable reason this rule exists.
    pub rationale: String,
}

/// One publication review sheet for one M5 publication lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationReviewSheet {
    /// Stable sheet id.
    pub entry_id: String,
    /// Human-readable title.
    pub title: String,
    /// The M5 artifact family this publication lane publishes.
    pub lane_kind: M5ArtifactFamilyKind,
    /// The publication-lane ref this sheet speaks about.
    pub lane_ref: String,
    /// Reviewable one-line statement of the lane.
    pub lane_summary: String,
    /// Whether the lane is part of the release-blocking set.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this lane backs.
    pub claim_ref: String,
    /// The canonical lifecycle label the claim publishes.
    pub claim_label: StableClaimLevel,
    /// Overall sheet state earned for the lane.
    pub sheet_state: ReviewSheetState,
    /// The version-bump review section.
    pub version_bump: VersionBumpReview,
    /// The publish-target review section.
    pub publish_target: PublishTargetReview,
    /// The review/plan parity record.
    pub review_parity: ReviewParity,
    /// The proof packet and its freshness SLO.
    pub proof_packet: ProofPacket,
    /// Waiver authorizing a provisional claim, when present.
    #[serde(default)]
    pub waiver: Option<QualificationWaiver>,
    /// Owner manifest sign-off.
    pub owner_signoff: OwnerSignoff,
    /// Active narrowing reasons dropping the lane below its claim label.
    #[serde(default)]
    pub active_narrowing_reasons: Vec<NarrowingReason>,
    /// The lifecycle label the lane effectively carries after narrowing.
    pub published_label: StableClaimLevel,
    /// Publication destinations that render this lane's label.
    #[serde(default)]
    pub publication_destinations: Vec<String>,
    /// Reviewable reason the lane carries this posture.
    pub rationale: String,
}

impl PublicationReviewSheet {
    /// True when the published label is at or above the cutline.
    pub fn publishes_stable(&self) -> bool {
        self.published_label.is_at_or_above_cutline()
    }

    /// True when the claim's canonical label is at or above the cutline.
    pub fn claim_holds_stable(&self) -> bool {
        self.claim_label.is_at_or_above_cutline()
    }

    /// True when the sheet's state lets it carry its claimed label.
    pub fn holds_label(&self) -> bool {
        self.sheet_state.holds_label()
    }

    /// True when a narrowing reason is active on the lane.
    pub fn has_active_reason(&self, reason: NarrowingReason) -> bool {
        self.active_narrowing_reasons.contains(&reason)
    }

    /// True when the dry-run preview is current enough to publish.
    pub fn dry_run_current(&self) -> bool {
        self.publish_target
            .descriptor
            .dry_run
            .is_current_enough_for_publication()
    }
}

/// Summary counts carried by the register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationReviewSummary {
    /// Total number of publication review sheets.
    pub total_entries: usize,
    /// Distinct claims covered.
    pub total_claims: usize,
    /// Sheets publishing a label at or above the cutline.
    pub entries_cleared: usize,
    /// Sheets narrowed below the cutline.
    pub entries_narrowed: usize,
    /// Sheets holding their label via an active waiver.
    pub entries_on_active_waiver: usize,
    /// Sheets carrying an undisclosed-version-impact reason.
    pub entries_with_impact_gap: usize,
    /// Sheets carrying an auth-disclosure or ambient-credential reason.
    pub entries_with_auth_gap: usize,
    /// Sheets carrying a dry-run-unavailable reason.
    pub entries_with_dry_run_gap: usize,
    /// Sheets carrying a descriptor- or diff-payload-parity reason.
    pub entries_with_parity_gap: usize,
    /// Sheets carrying a rollback-target-missing reason.
    pub entries_with_rollback_gap: usize,
    /// Total release-blocking lanes.
    pub release_blocking_total: usize,
    /// Release-blocking lanes publishing a label at or above the cutline.
    pub release_blocking_cleared: usize,
    /// Release-blocking lanes narrowed below the cutline.
    pub release_blocking_narrowed: usize,
    /// Notebook-pack lanes.
    pub notebook_pack_entries: usize,
    /// Request/data-asset lanes.
    pub request_data_asset_entries: usize,
    /// Profiler/replay-artifact lanes.
    pub profiler_replay_entries: usize,
    /// Framework/template-pack lanes.
    pub framework_template_entries: usize,
    /// Docs-pack lanes.
    pub docs_pack_entries: usize,
    /// Model-pack lanes.
    pub model_pack_entries: usize,
    /// Companion/offboarding-packet lanes.
    pub companion_offboarding_entries: usize,
    /// Managed-output lanes.
    pub managed_output_entries: usize,
    /// Sheets whose parity state is `matched`.
    pub parity_matched: usize,
    /// Sheets whose parity state is `divergent`.
    pub parity_divergent: usize,
    /// Sheets whose parity state is `missing`.
    pub parity_missing: usize,
    /// Sheets whose auth disclosure is `explicit_disclosed`.
    pub auth_explicit_disclosed: usize,
    /// Sheets whose auth disclosure is `undisclosed`.
    pub auth_undisclosed: usize,
    /// Sheets whose auth disclosure is `ambient_inherited`.
    pub auth_ambient_inherited: usize,
    /// Proof packets whose SLO state is `current`.
    pub packets_current: usize,
    /// Proof packets whose SLO state is `due_for_refresh`.
    pub packets_due_for_refresh: usize,
    /// Proof packets whose SLO state is `breached`.
    pub packets_breached: usize,
    /// Proof packets whose SLO state is `missing`.
    pub packets_missing: usize,
    /// Total active narrowing reasons across all sheets.
    pub total_active_narrowing_reasons: usize,
    /// Number of stop rules currently firing.
    pub rules_firing: usize,
}

/// One export row for downstream Help/About, support, and diagnostics surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationReviewExportRow {
    /// Stable sheet id.
    pub entry_id: String,
    /// The M5 artifact family this lane publishes.
    pub lane_kind: M5ArtifactFamilyKind,
    /// The publication-lane ref.
    pub lane_ref: String,
    /// Whether the lane is release-blocking.
    pub release_blocking: bool,
    /// The stable-claim-manifest entry id whose claim this lane backs.
    pub claim_ref: String,
    /// The canonical lifecycle label.
    pub claim_label: StableClaimLevel,
    /// The effective label after narrowing.
    pub published_label: StableClaimLevel,
    /// Whether the lane publishes at or above the cutline.
    pub publishes_stable: bool,
    /// Overall sheet state earned.
    pub sheet_state: ReviewSheetState,
    /// Prior version proposed for the bump.
    pub prior_version: String,
    /// Target version proposed for the bump.
    pub target_version: String,
    /// The disclosed public-surface impact class.
    pub public_surface_impact: PublicSurfaceImpact,
    /// Whether the version bump's impact is disclosed.
    pub impact_disclosed: bool,
    /// The disclosed publish-target class.
    pub target_class: crate::release_center_model::PublishTargetClass,
    /// The disclosed visibility class.
    pub visibility_class: crate::release_center_model::TargetVisibilityClass,
    /// The disclosed mutability class.
    pub mutability_class: crate::release_center_model::TargetMutabilityClass,
    /// The disclosed auth-source class.
    pub auth_source_class: crate::release_center_model::AuthSourceClass,
    /// How the auth source and target scope are disclosed.
    pub auth_disclosure_state: AuthDisclosureState,
    /// The disclosed rollout ring.
    pub rollout_ring: crate::release_center_model::RolloutRing,
    /// Whether the rollout ring is disclosed.
    pub rollout_ring_disclosed: bool,
    /// The disclosed dry-run availability.
    pub dry_run_availability: crate::release_center_model::DryRunAvailabilityClass,
    /// The mirror or registry destination ref.
    pub mirror_destination_ref: String,
    /// The rollback target ref disclosed before publication.
    pub rollback_target_ref: String,
    /// The review/plan parity state.
    pub parity_state: ParityState,
    /// Proof packet SLO state.
    pub slo_state: FreshnessSloState,
    /// Active narrowing reasons.
    pub active_narrowing_reasons: Vec<NarrowingReason>,
}

/// Export projection for Help/About, support, and diagnostics surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicationReviewExportProjection {
    /// Register identifier.
    pub manifest_id: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Publication decision.
    pub publication_decision: PromotionDecision,
    /// Export rows.
    pub rows: Vec<PublicationReviewExportRow>,
}

/// The typed publication review-sheet register.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationReviewRegister {
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
    /// Ref to the M5 publication matrix this register publishes against.
    pub publication_matrix_ref: String,
    /// Ref to the shared release-center object model.
    pub release_center_model_ref: String,
    /// Closed lifecycle-label vocabulary.
    pub lifecycle_labels: Vec<StableClaimLevel>,
    /// Closed publication-lane-kind vocabulary.
    pub lane_kinds: Vec<M5ArtifactFamilyKind>,
    /// Closed sheet-state vocabulary.
    pub sheet_states: Vec<ReviewSheetState>,
    /// Closed public-surface-impact vocabulary.
    pub public_surface_impacts: Vec<PublicSurfaceImpact>,
    /// Closed auth-disclosure-state vocabulary.
    pub auth_disclosure_states: Vec<AuthDisclosureState>,
    /// Closed parity-state vocabulary.
    pub parity_states: Vec<ParityState>,
    /// Closed narrowing-reason vocabulary.
    pub narrowing_reasons: Vec<NarrowingReason>,
    /// Closed stop-rule-action vocabulary.
    pub stop_rule_actions: Vec<StopAction>,
    /// The launch cutline.
    pub launch_cutline: LaunchCutline,
    /// The closed set of release-blocking lane refs this register must cover.
    pub release_blocking_lane_refs: Vec<String>,
    /// Stop rules.
    pub stop_rules: Vec<PublicationReviewStopRule>,
    /// Publication review sheets.
    pub rows: Vec<PublicationReviewSheet>,
    /// Recorded publication verdict.
    pub publication: PromotionDecisionRecord,
    /// Summary counts.
    pub summary: PublicationReviewSummary,
}

impl PublicationReviewRegister {
    /// Returns the sheet registered for `entry_id`.
    pub fn row(&self, entry_id: &str) -> Option<&PublicationReviewSheet> {
        self.rows.iter().find(|row| row.entry_id == entry_id)
    }

    /// Returns the sheets publishing a label at or above the cutline.
    pub fn rows_cleared(&self) -> Vec<&PublicationReviewSheet> {
        self.rows
            .iter()
            .filter(|row| row.publishes_stable())
            .collect()
    }

    /// Returns the sheets narrowed below the cutline.
    pub fn rows_narrowed(&self) -> Vec<&PublicationReviewSheet> {
        self.rows
            .iter()
            .filter(|row| !row.publishes_stable())
            .collect()
    }

    /// Returns the release-blocking sheets.
    pub fn release_blocking_rows(&self) -> Vec<&PublicationReviewSheet> {
        self.rows
            .iter()
            .filter(|row| row.release_blocking)
            .collect()
    }

    /// Returns the sheets for one publication-lane kind.
    pub fn rows_for_kind(&self, kind: M5ArtifactFamilyKind) -> Vec<&PublicationReviewSheet> {
        self.rows
            .iter()
            .filter(|row| row.lane_kind == kind)
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

    /// True when `rule` fires: a watched lane carries its trigger reason.
    pub fn stop_rule_fires(&self, rule: &PublicationReviewStopRule) -> bool {
        self.rows.iter().any(|row| {
            rule.applies_to_labels.contains(&row.claim_label)
                && row.has_active_reason(rule.trigger_reason)
        })
    }

    /// Recomputes the publication verdict from the sheets and stop rules.
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

    /// Sheet ids that trigger a blocking, firing rule, sorted and unique.
    ///
    /// Only sheets whose claim is at or above the cutline count: a sheet whose
    /// claim is already canonically narrowed is not a *publication* blocker, it
    /// merely inherits the upstream ceiling.
    pub fn computed_blocking_entry_ids(&self) -> Vec<String> {
        let blocking_triggers: BTreeSet<NarrowingReason> = self
            .stop_rules
            .iter()
            .filter(|rule| rule.blocks_publication && self.stop_rule_fires(rule))
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

    /// Recomputes the summary block from the sheets and stop rules.
    pub fn computed_summary(&self) -> PublicationReviewSummary {
        let packets = |state: FreshnessSloState| {
            self.rows
                .iter()
                .filter(|row| row.proof_packet.slo_state == state)
                .count()
        };
        let kind = |kind: M5ArtifactFamilyKind| self.rows_for_kind(kind).len();
        let parity = |state: ParityState| {
            self.rows
                .iter()
                .filter(|row| row.review_parity.parity_state == state)
                .count()
        };
        let auth = |state: AuthDisclosureState| {
            self.rows
                .iter()
                .filter(|row| row.publish_target.auth_disclosure.state == state)
                .count()
        };
        let with_reason = |reason: NarrowingReason| {
            self.rows
                .iter()
                .filter(|row| row.has_active_reason(reason))
                .count()
        };
        let with_any = |reasons: &[NarrowingReason]| {
            self.rows
                .iter()
                .filter(|row| reasons.iter().any(|r| row.has_active_reason(*r)))
                .count()
        };
        let release_blocking: Vec<&PublicationReviewSheet> = self.release_blocking_rows();
        PublicationReviewSummary {
            total_entries: self.rows.len(),
            total_claims: self.claims().len(),
            entries_cleared: self
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
                .filter(|row| row.sheet_state == ReviewSheetState::OnWaiver)
                .count(),
            entries_with_impact_gap: with_reason(NarrowingReason::VersionImpactUndisclosed),
            entries_with_auth_gap: with_any(&[
                NarrowingReason::AuthSourceUndisclosed,
                NarrowingReason::AmbientCredentialInheritance,
            ]),
            entries_with_dry_run_gap: with_reason(NarrowingReason::DryRunUnavailable),
            entries_with_parity_gap: with_any(&[
                NarrowingReason::DescriptorParityBroken,
                NarrowingReason::DiffPayloadParityBroken,
            ]),
            entries_with_rollback_gap: with_reason(NarrowingReason::RollbackTargetMissing),
            release_blocking_total: release_blocking.len(),
            release_blocking_cleared: release_blocking
                .iter()
                .filter(|row| row.publishes_stable())
                .count(),
            release_blocking_narrowed: release_blocking
                .iter()
                .filter(|row| !row.publishes_stable())
                .count(),
            notebook_pack_entries: kind(M5ArtifactFamilyKind::NotebookPack),
            request_data_asset_entries: kind(M5ArtifactFamilyKind::RequestDataAsset),
            profiler_replay_entries: kind(M5ArtifactFamilyKind::ProfilerReplayArtifact),
            framework_template_entries: kind(M5ArtifactFamilyKind::FrameworkTemplatePack),
            docs_pack_entries: kind(M5ArtifactFamilyKind::DocsPack),
            model_pack_entries: kind(M5ArtifactFamilyKind::ModelPack),
            companion_offboarding_entries: kind(M5ArtifactFamilyKind::CompanionOffboardingPacket),
            managed_output_entries: kind(M5ArtifactFamilyKind::ManagedOutput),
            parity_matched: parity(ParityState::Matched),
            parity_divergent: parity(ParityState::Divergent),
            parity_missing: parity(ParityState::Missing),
            auth_explicit_disclosed: auth(AuthDisclosureState::ExplicitDisclosed),
            auth_undisclosed: auth(AuthDisclosureState::Undisclosed),
            auth_ambient_inherited: auth(AuthDisclosureState::AmbientInherited),
            packets_current: packets(FreshnessSloState::Current),
            packets_due_for_refresh: packets(FreshnessSloState::DueForRefresh),
            packets_breached: packets(FreshnessSloState::Breached),
            packets_missing: packets(FreshnessSloState::Missing),
            total_active_narrowing_reasons: self
                .rows
                .iter()
                .map(|row| row.active_narrowing_reasons.len())
                .sum(),
            rules_firing: self
                .stop_rules
                .iter()
                .filter(|rule| self.stop_rule_fires(rule))
                .count(),
        }
    }

    /// Produces an export/Help-About-safe projection that downstream surfaces
    /// render instead of cloning status text.
    pub fn support_export_projection(&self) -> PublicationReviewExportProjection {
        PublicationReviewExportProjection {
            manifest_id: self.manifest_id.clone(),
            as_of: self.as_of.clone(),
            publication_decision: self.publication.decision,
            rows: self
                .rows
                .iter()
                .map(|row| {
                    let descriptor = &row.publish_target.descriptor;
                    PublicationReviewExportRow {
                        entry_id: row.entry_id.clone(),
                        lane_kind: row.lane_kind,
                        lane_ref: row.lane_ref.clone(),
                        release_blocking: row.release_blocking,
                        claim_ref: row.claim_ref.clone(),
                        claim_label: row.claim_label,
                        published_label: row.published_label,
                        publishes_stable: row.publishes_stable(),
                        sheet_state: row.sheet_state,
                        prior_version: row.version_bump.proposal.prior_version.clone(),
                        target_version: row.version_bump.proposal.target_version.clone(),
                        public_surface_impact: row.version_bump.public_surface_impact,
                        impact_disclosed: row.version_bump.impact_disclosed,
                        target_class: descriptor.target_class,
                        visibility_class: descriptor.visibility_class,
                        mutability_class: descriptor.mutability_class,
                        auth_source_class: descriptor.auth_source_class,
                        auth_disclosure_state: row.publish_target.auth_disclosure.state,
                        rollout_ring: descriptor.rollout_ring,
                        rollout_ring_disclosed: row.publish_target.rollout_ring_disclosed,
                        dry_run_availability: descriptor.dry_run.availability_class,
                        mirror_destination_ref: row.publish_target.mirror_destination_ref.clone(),
                        rollback_target_ref: descriptor.rollback_target_ref.clone(),
                        parity_state: row.review_parity.parity_state,
                        slo_state: row.proof_packet.slo_state,
                        active_narrowing_reasons: row.active_narrowing_reasons.clone(),
                    }
                })
                .collect(),
        }
    }

    /// Validates the register, returning every violation found.
    pub fn validate(&self) -> Vec<PublicationReviewViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_stop_rules(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.entry_id.clone()) {
                violations.push(PublicationReviewViolation::DuplicateEntryId {
                    entry_id: row.entry_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }
        if self.rows.is_empty() {
            violations.push(PublicationReviewViolation::EmptyRegister);
        }

        self.validate_coverage(&mut violations);
        self.validate_publication(&mut violations);

        if self.summary != self.computed_summary() {
            violations.push(PublicationReviewViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<PublicationReviewViolation>) {
        if self.schema_version != PUBLICATION_REVIEW_SCHEMA_VERSION {
            violations.push(PublicationReviewViolation::UnsupportedSchemaVersion {
                actual: self.schema_version,
            });
        }
        if self.record_kind != PUBLICATION_REVIEW_RECORD_KIND {
            violations.push(PublicationReviewViolation::UnsupportedRecordKind {
                actual: self.record_kind.clone(),
            });
        }
        for (field, value) in [
            ("manifest_id", &self.manifest_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
            ("claim_manifest_ref", &self.claim_manifest_ref),
            ("publication_matrix_ref", &self.publication_matrix_ref),
            ("release_center_model_ref", &self.release_center_model_ref),
        ] {
            if value.trim().is_empty() {
                violations.push(PublicationReviewViolation::EmptyField {
                    entry_id: "<register>".to_owned(),
                    field_name: field,
                });
            }
        }
        if self.lifecycle_labels != StableClaimLevel::ALL.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "lifecycle_labels",
            });
        }
        if self.lane_kinds != M5ArtifactFamilyKind::ALL.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "lane_kinds",
            });
        }
        if self.sheet_states != ReviewSheetState::ALL.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "sheet_states",
            });
        }
        if self.public_surface_impacts != PublicSurfaceImpact::ALL.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "public_surface_impacts",
            });
        }
        if self.auth_disclosure_states != AuthDisclosureState::ALL.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "auth_disclosure_states",
            });
        }
        if self.parity_states != ParityState::ALL.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "parity_states",
            });
        }
        if self.narrowing_reasons != NarrowingReason::ALL.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "narrowing_reasons",
            });
        }
        if self.stop_rule_actions != StopAction::ALL.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "stop_rule_actions",
            });
        }

        let cutline = &self.launch_cutline;
        if cutline.cutline_level != StableClaimLevel::Stable {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.cutline_level",
            });
        }
        if cutline.above_cutline_levels != StableClaimLevel::ABOVE_CUTLINE.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.above_cutline_levels",
            });
        }
        if cutline.below_cutline_levels != StableClaimLevel::BELOW_CUTLINE.to_vec() {
            violations.push(PublicationReviewViolation::ClosedVocabularyMismatch {
                field: "launch_cutline.below_cutline_levels",
            });
        }
        if cutline.description.trim().is_empty() {
            violations.push(PublicationReviewViolation::EmptyField {
                entry_id: "<launch_cutline>".to_owned(),
                field_name: "description",
            });
        }
    }

    fn validate_stop_rules(&self, violations: &mut Vec<PublicationReviewViolation>) {
        if self.stop_rules.is_empty() {
            violations.push(PublicationReviewViolation::NoStopRules);
        }
        let mut seen = BTreeSet::new();
        let mut covered = BTreeSet::new();
        for rule in &self.stop_rules {
            if !seen.insert(rule.rule_id.clone()) {
                violations.push(PublicationReviewViolation::DuplicateStopRuleId {
                    rule_id: rule.rule_id.clone(),
                });
            }
            for (field, value) in [
                ("rule_id", &rule.rule_id),
                ("title", &rule.title),
                ("rationale", &rule.rationale),
            ] {
                if value.trim().is_empty() {
                    violations.push(PublicationReviewViolation::EmptyField {
                        entry_id: rule.rule_id.clone(),
                        field_name: field,
                    });
                }
            }
            if rule.applies_to_labels.is_empty() {
                violations.push(PublicationReviewViolation::StopRuleWithoutLabels {
                    rule_id: rule.rule_id.clone(),
                });
            }
            covered.insert(rule.trigger_reason);
        }

        for reason in NarrowingReason::ALL {
            if !covered.contains(&reason) {
                violations
                    .push(PublicationReviewViolation::NarrowingReasonWithoutStopRule { reason });
            }
        }
    }

    fn validate_row(
        &self,
        row: &PublicationReviewSheet,
        violations: &mut Vec<PublicationReviewViolation>,
    ) {
        for (field, value) in [
            ("entry_id", &row.entry_id),
            ("title", &row.title),
            ("lane_ref", &row.lane_ref),
            ("lane_summary", &row.lane_summary),
            ("claim_ref", &row.claim_ref),
            ("rationale", &row.rationale),
            (
                "version_bump.proposal.proposal_id",
                &row.version_bump.proposal.proposal_id,
            ),
            (
                "version_bump.proposal.prior_version",
                &row.version_bump.proposal.prior_version,
            ),
            (
                "version_bump.proposal.target_version",
                &row.version_bump.proposal.target_version,
            ),
            (
                "publish_target.descriptor.publish_target_id",
                &row.publish_target.descriptor.publish_target_id,
            ),
            (
                "publish_target.descriptor.destination_class",
                &row.publish_target.descriptor.destination_class,
            ),
            (
                "publish_target.mirror_destination_ref",
                &row.publish_target.mirror_destination_ref,
            ),
            (
                "publish_target.auth_disclosure.auth_source_ref",
                &row.publish_target.auth_disclosure.auth_source_ref,
            ),
            (
                "review_parity.human_review_ref",
                &row.review_parity.human_review_ref,
            ),
            (
                "review_parity.headless_plan_ref",
                &row.review_parity.headless_plan_ref,
            ),
            (
                "review_parity.diff_payload_ref",
                &row.review_parity.diff_payload_ref,
            ),
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
        ] {
            if value.trim().is_empty() {
                violations.push(PublicationReviewViolation::EmptyField {
                    entry_id: row.entry_id.clone(),
                    field_name: field,
                });
            }
        }

        self.validate_disclosure(row, violations);

        // The ceiling: no lane may publish a label wider than the claim's
        // canonical label.
        if row.published_label.rank() > row.claim_label.rank() {
            violations.push(PublicationReviewViolation::PublishedWiderThanClaim {
                entry_id: row.entry_id.clone(),
                claim: row.claim_label,
                published: row.published_label,
            });
        }

        // The freshness SLO target must be positive and the warn window may not
        // exceed it.
        if row.proof_packet.freshness_slo.target_max_age_days == 0 {
            violations.push(PublicationReviewViolation::EmptyField {
                entry_id: row.entry_id.clone(),
                field_name: "proof_packet.freshness_slo.target_max_age_days",
            });
        }
        if !row.proof_packet.freshness_slo.window_is_consistent() {
            violations.push(PublicationReviewViolation::FreshnessSloInconsistent {
                entry_id: row.entry_id.clone(),
            });
        }

        // A claim whose canonical label is below the cutline forces the lane to
        // inherit that ceiling and narrow.
        if !row.claim_holds_stable() {
            if row.holds_label() {
                violations.push(PublicationReviewViolation::HeldOnNarrowedClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(PublicationReviewViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.sheet_state,
                });
            }
        }

        let slo_state = row.proof_packet.slo_state;

        if row.holds_label() {
            // A cleared/on-waiver lane publishes exactly the claim's canonical
            // label, carries no active reason, rides a captured within-SLO packet,
            // and is owner-signed.
            if row.published_label != row.claim_label {
                violations.push(PublicationReviewViolation::HeldLabelNotEqualClaim {
                    entry_id: row.entry_id.clone(),
                    claim: row.claim_label,
                    published: row.published_label,
                });
            }
            if !row.active_narrowing_reasons.is_empty() {
                violations.push(PublicationReviewViolation::HeldWithActiveGap {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.proof_packet.has_capture() {
                violations.push(PublicationReviewViolation::HeldWithoutFreshPacket {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !slo_state.is_within_slo() {
                violations.push(PublicationReviewViolation::HeldOnStalePacket {
                    entry_id: row.entry_id.clone(),
                    slo_state,
                });
            }
            if !(row.owner_signoff.signed_off && row.owner_signoff.signed_at.is_some()) {
                violations.push(PublicationReviewViolation::HeldWithoutSignoff {
                    entry_id: row.entry_id.clone(),
                });
            }
            // The cleared invariants on the review sheet itself.
            if !row.version_bump.impact_disclosed
                || row.version_bump.impact_summary.trim().is_empty()
            {
                violations.push(PublicationReviewViolation::HeldWithUndisclosedImpact {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.publish_target.auth_disclosure.state.holds()
                || !row.publish_target.auth_disclosure.disclosed_before_mutation
                || !row.publish_target.auth_disclosure.target_scope_disclosed
            {
                violations.push(PublicationReviewViolation::HeldWithoutAuthDisclosure {
                    entry_id: row.entry_id.clone(),
                    state: row.publish_target.auth_disclosure.state,
                });
            }
            if !row.dry_run_current() {
                violations.push(PublicationReviewViolation::HeldWithoutDryRun {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.review_parity.parity_state.holds()
                || !row.review_parity.descriptor_digests_match()
                || !row.review_parity.diff_payload_digests_match()
            {
                violations.push(PublicationReviewViolation::HeldWithoutParity {
                    entry_id: row.entry_id.clone(),
                    state: row.review_parity.parity_state,
                });
            }
            if row
                .publish_target
                .descriptor
                .rollback_target_ref
                .trim()
                .is_empty()
            {
                violations.push(PublicationReviewViolation::HeldWithoutRollbackTarget {
                    entry_id: row.entry_id.clone(),
                });
            }
            if !row.publish_target.rollout_ring_disclosed {
                violations.push(PublicationReviewViolation::HeldWithoutRolloutRing {
                    entry_id: row.entry_id.clone(),
                });
            }
            // A cleared lane carries no waiver; an on-waiver lane carries a valid
            // one.
            match row.sheet_state {
                ReviewSheetState::Cleared => {
                    if row.waiver.is_some() {
                        violations.push(PublicationReviewViolation::ClearedWithWaiver {
                            entry_id: row.entry_id.clone(),
                        });
                    }
                }
                ReviewSheetState::OnWaiver => {
                    if row
                        .waiver
                        .as_ref()
                        .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                        .unwrap_or(true)
                    {
                        violations.push(PublicationReviewViolation::WaiverStateWithoutWaiver {
                            entry_id: row.entry_id.clone(),
                            state: row.sheet_state,
                        });
                    }
                }
                _ => {}
            }
        } else {
            // A narrowing state must drop the published label below the cutline
            // and name at least one active reason.
            if row.publishes_stable() {
                violations.push(PublicationReviewViolation::PublishedLabelNotNarrowed {
                    entry_id: row.entry_id.clone(),
                    state: row.sheet_state,
                    published: row.published_label,
                });
            }
            if row.active_narrowing_reasons.is_empty() {
                violations.push(PublicationReviewViolation::NarrowingWithoutReason {
                    entry_id: row.entry_id.clone(),
                    state: row.sheet_state,
                });
            }
            if slo_state == FreshnessSloState::Breached
                && !row.has_active_reason(NarrowingReason::ProofPacketStale)
            {
                violations.push(PublicationReviewViolation::BreachedPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
            if slo_state == FreshnessSloState::Missing
                && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
            {
                violations.push(PublicationReviewViolation::MissingPacketWithoutReason {
                    entry_id: row.entry_id.clone(),
                });
            }
        }

        self.validate_state_reason_coherence(row, violations);
    }

    /// Every "if this aspect is bad, the matching reason must be active" rule.
    /// These apply to every lane regardless of held/narrowing state.
    fn validate_disclosure(
        &self,
        row: &PublicationReviewSheet,
        violations: &mut Vec<PublicationReviewViolation>,
    ) {
        let require = |violations: &mut Vec<PublicationReviewViolation>,
                       bad: bool,
                       reason: NarrowingReason| {
            if bad && !row.has_active_reason(reason) {
                violations.push(PublicationReviewViolation::DisclosureGapWithoutReason {
                    entry_id: row.entry_id.clone(),
                    reason,
                });
            }
        };

        // A migration-or-breaking impact must disclose its flags.
        if row
            .version_bump
            .public_surface_impact
            .requires_migration_flags()
            && row.version_bump.migration_flags.is_empty()
            && row.version_bump.impact_disclosed
        {
            violations.push(PublicationReviewViolation::MigrationFlagsMissing {
                entry_id: row.entry_id.clone(),
                impact: row.version_bump.public_surface_impact,
            });
        }

        require(
            violations,
            !row.version_bump.impact_disclosed || row.version_bump.impact_summary.trim().is_empty(),
            NarrowingReason::VersionImpactUndisclosed,
        );
        require(
            violations,
            row.publish_target.auth_disclosure.state == AuthDisclosureState::AmbientInherited,
            NarrowingReason::AmbientCredentialInheritance,
        );
        require(
            violations,
            row.publish_target.auth_disclosure.state == AuthDisclosureState::Undisclosed
                || !row.publish_target.auth_disclosure.disclosed_before_mutation
                || !row.publish_target.auth_disclosure.target_scope_disclosed,
            NarrowingReason::AuthSourceUndisclosed,
        );
        require(
            violations,
            !row.dry_run_current(),
            NarrowingReason::DryRunUnavailable,
        );
        require(
            violations,
            !row.publish_target.rollout_ring_disclosed,
            NarrowingReason::RolloutRingUndisclosed,
        );
        require(
            violations,
            row.publish_target
                .descriptor
                .rollback_target_ref
                .trim()
                .is_empty(),
            NarrowingReason::RollbackTargetMissing,
        );

        // Parity: a divergent or missing descriptor/diff payload must name the
        // matching reason, and an unequal digest must name it specifically.
        let descriptor_broken = !row.review_parity.descriptor_digests_match();
        let diff_broken = !row.review_parity.diff_payload_digests_match();
        require(
            violations,
            descriptor_broken,
            NarrowingReason::DescriptorParityBroken,
        );
        require(
            violations,
            diff_broken,
            NarrowingReason::DiffPayloadParityBroken,
        );
        if row.review_parity.parity_state != ParityState::Matched
            && !row.has_active_reason(NarrowingReason::DescriptorParityBroken)
            && !row.has_active_reason(NarrowingReason::DiffPayloadParityBroken)
        {
            violations.push(PublicationReviewViolation::ParityBrokenWithoutReason {
                entry_id: row.entry_id.clone(),
                state: row.review_parity.parity_state,
            });
        }
        // A matched parity state must actually carry equal digests.
        if row.review_parity.parity_state == ParityState::Matched
            && (descriptor_broken || diff_broken)
        {
            violations.push(PublicationReviewViolation::ParityMatchedButDigestsDiffer {
                entry_id: row.entry_id.clone(),
            });
        }
    }

    fn validate_state_reason_coherence(
        &self,
        row: &PublicationReviewSheet,
        violations: &mut Vec<PublicationReviewViolation>,
    ) {
        let push_incoherent = |violations: &mut Vec<PublicationReviewViolation>,
                               expected: NarrowingReason| {
            violations.push(PublicationReviewViolation::StateReasonIncoherent {
                entry_id: row.entry_id.clone(),
                state: row.sheet_state,
                expected_reason: expected,
            });
        };

        match row.sheet_state {
            ReviewSheetState::ReviewGap => {
                if !row
                    .active_narrowing_reasons
                    .iter()
                    .any(|reason| reason.is_review_gap())
                {
                    push_incoherent(violations, NarrowingReason::DescriptorParityBroken);
                }
            }
            ReviewSheetState::Stale => {
                if !row.has_active_reason(NarrowingReason::ProofPacketStale)
                    && !row.has_active_reason(NarrowingReason::ProofPacketMissing)
                {
                    push_incoherent(violations, NarrowingReason::ProofPacketStale);
                }
            }
            ReviewSheetState::OwnerUnsigned => {
                if !row.has_active_reason(NarrowingReason::OwnerManifestUnsigned) {
                    push_incoherent(violations, NarrowingReason::OwnerManifestUnsigned);
                }
            }
            ReviewSheetState::OnWaiver => {
                if row
                    .waiver
                    .as_ref()
                    .map(|w| w.waiver_ref.trim().is_empty() || w.expires_at.trim().is_empty())
                    .unwrap_or(true)
                {
                    violations.push(PublicationReviewViolation::WaiverStateWithoutWaiver {
                        entry_id: row.entry_id.clone(),
                        state: row.sheet_state,
                    });
                }
            }
            ReviewSheetState::Cleared => {}
        }
    }

    fn validate_coverage(&self, violations: &mut Vec<PublicationReviewViolation>) {
        let covered: BTreeSet<String> = self.rows.iter().map(|row| row.lane_ref.clone()).collect();
        for declared in &self.release_blocking_lane_refs {
            if !covered.contains(declared) {
                violations.push(PublicationReviewViolation::ReleaseBlockingLaneUncovered {
                    lane_ref: declared.clone(),
                });
            }
        }
        for row in &self.rows {
            if row.release_blocking && !self.release_blocking_lane_refs.contains(&row.lane_ref) {
                violations.push(PublicationReviewViolation::ReleaseBlockingRowNotDeclared {
                    entry_id: row.entry_id.clone(),
                });
            }
        }
    }

    fn validate_publication(&self, violations: &mut Vec<PublicationReviewViolation>) {
        if self.publication.promotion_gate.trim().is_empty() {
            violations.push(PublicationReviewViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "promotion_gate",
            });
        }
        if self.publication.rationale.trim().is_empty() {
            violations.push(PublicationReviewViolation::EmptyField {
                entry_id: "<publication>".to_owned(),
                field_name: "publication.rationale",
            });
        }
        let computed = self.computed_publication_decision();
        if self.publication.decision != computed {
            violations.push(
                PublicationReviewViolation::PublicationDecisionInconsistent {
                    declared: self.publication.decision,
                    computed,
                },
            );
        }
        if self.publication.blocking_rule_ids != self.computed_blocking_rule_ids() {
            violations.push(PublicationReviewViolation::PublicationBlockingSetMismatch {
                field: "blocking_rule_ids",
            });
        }
        if self.publication.blocking_claim_ids != self.computed_blocking_entry_ids() {
            violations.push(PublicationReviewViolation::PublicationBlockingSetMismatch {
                field: "blocking_claim_ids",
            });
        }
    }
}

/// A validation violation for the publication review-sheet register.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationReviewViolation {
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
    /// The register has no sheets.
    EmptyRegister,
    /// The register has no stop rules.
    NoStopRules,
    /// A required field is empty.
    EmptyField {
        /// Sheet or section id.
        entry_id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A sheet id appears more than once.
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
    /// The published label is wider than the backed claim's canonical label.
    PublishedWiderThanClaim {
        /// Sheet id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A sheet holds a label while the claim is below the cutline.
    HeldOnNarrowedClaim {
        /// Sheet id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
    },
    /// A narrowing state carries no active reason.
    NarrowingWithoutReason {
        /// Sheet id.
        entry_id: String,
        /// Sheet state.
        state: ReviewSheetState,
    },
    /// A narrowing state did not drop the published label below the cutline.
    PublishedLabelNotNarrowed {
        /// Sheet id.
        entry_id: String,
        /// Sheet state.
        state: ReviewSheetState,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held sheet carries a published label different from the claim.
    HeldLabelNotEqualClaim {
        /// Sheet id.
        entry_id: String,
        /// Claimed level.
        claim: StableClaimLevel,
        /// Published level.
        published: StableClaimLevel,
    },
    /// A held sheet has active narrowing reasons.
    HeldWithActiveGap {
        /// Sheet id.
        entry_id: String,
    },
    /// A held sheet has no captured proof packet.
    HeldWithoutFreshPacket {
        /// Sheet id.
        entry_id: String,
    },
    /// A held sheet rides a packet outside its freshness SLO.
    HeldOnStalePacket {
        /// Sheet id.
        entry_id: String,
        /// Packet SLO state.
        slo_state: FreshnessSloState,
    },
    /// A held sheet lacks owner-manifest sign-off.
    HeldWithoutSignoff {
        /// Sheet id.
        entry_id: String,
    },
    /// A held sheet hides its version bump's migration or compatibility impact.
    HeldWithUndisclosedImpact {
        /// Sheet id.
        entry_id: String,
    },
    /// A held sheet does not disclose its auth source and target scope before
    /// mutation, or would inherit ambient credentials.
    HeldWithoutAuthDisclosure {
        /// Sheet id.
        entry_id: String,
        /// Auth disclosure state.
        state: AuthDisclosureState,
    },
    /// A held sheet lacks a current dry-run preview.
    HeldWithoutDryRun {
        /// Sheet id.
        entry_id: String,
    },
    /// A held sheet does not share the descriptor and diff payload across review
    /// and plan.
    HeldWithoutParity {
        /// Sheet id.
        entry_id: String,
        /// Parity state.
        state: ParityState,
    },
    /// A held sheet records no rollback target.
    HeldWithoutRollbackTarget {
        /// Sheet id.
        entry_id: String,
    },
    /// A held sheet does not disclose its rollout ring.
    HeldWithoutRolloutRing {
        /// Sheet id.
        entry_id: String,
    },
    /// A cleared sheet carries a waiver.
    ClearedWithWaiver {
        /// Sheet id.
        entry_id: String,
    },
    /// A bad disclosure aspect did not name its narrowing reason.
    DisclosureGapWithoutReason {
        /// Sheet id.
        entry_id: String,
        /// The reason the aspect requires.
        reason: NarrowingReason,
    },
    /// A migration-or-breaking impact discloses no migration flags.
    MigrationFlagsMissing {
        /// Sheet id.
        entry_id: String,
        /// The impact class that requires flags.
        impact: PublicSurfaceImpact,
    },
    /// A non-matched parity state names no parity reason.
    ParityBrokenWithoutReason {
        /// Sheet id.
        entry_id: String,
        /// Parity state.
        state: ParityState,
    },
    /// A matched parity state carries unequal digests.
    ParityMatchedButDigestsDiffer {
        /// Sheet id.
        entry_id: String,
    },
    /// A sheet state is incoherent with its active reasons.
    StateReasonIncoherent {
        /// Sheet id.
        entry_id: String,
        /// Sheet state.
        state: ReviewSheetState,
        /// Reason the state requires.
        expected_reason: NarrowingReason,
    },
    /// A waiver-bearing state names no waiver.
    WaiverStateWithoutWaiver {
        /// Sheet id.
        entry_id: String,
        /// Sheet state.
        state: ReviewSheetState,
    },
    /// A narrowing sheet with a breached proof packet does not name the stale
    /// reason.
    BreachedPacketWithoutReason {
        /// Sheet id.
        entry_id: String,
    },
    /// A narrowing sheet with a missing proof packet does not name the missing
    /// reason.
    MissingPacketWithoutReason {
        /// Sheet id.
        entry_id: String,
    },
    /// A release-blocking lane ref has no covering sheet.
    ReleaseBlockingLaneUncovered {
        /// Lane ref.
        lane_ref: String,
    },
    /// A release-blocking sheet is not declared in the release-blocking list.
    ReleaseBlockingRowNotDeclared {
        /// Sheet id.
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
    /// The summary counts disagree with the sheets.
    SummaryMismatch,
    /// The freshness SLO window is inconsistent.
    FreshnessSloInconsistent {
        /// Sheet id.
        entry_id: String,
    },
}

impl fmt::Display for PublicationReviewViolation {
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
            Self::EmptyRegister => write!(f, "register has no sheets"),
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
            Self::PublishedWiderThanClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "sheet {entry_id} published level {published:?} is wider than claim {claim:?}"
            ),
            Self::HeldOnNarrowedClaim { entry_id, claim } => write!(
                f,
                "sheet {entry_id} holds label while claim {claim:?} is below cutline"
            ),
            Self::NarrowingWithoutReason { entry_id, state } => write!(
                f,
                "sheet {entry_id} state {state:?} narrows without active reason"
            ),
            Self::PublishedLabelNotNarrowed {
                entry_id,
                state,
                published,
            } => write!(
                f,
                "sheet {entry_id} state {state:?} must narrow but publishes {published:?}"
            ),
            Self::HeldLabelNotEqualClaim {
                entry_id,
                claim,
                published,
            } => write!(
                f,
                "sheet {entry_id} held label {published:?} does not equal claim {claim:?}"
            ),
            Self::HeldWithActiveGap { entry_id } => {
                write!(f, "sheet {entry_id} holds stable with active gap")
            }
            Self::HeldWithoutFreshPacket { entry_id } => {
                write!(f, "sheet {entry_id} holds stable without fresh packet")
            }
            Self::HeldOnStalePacket {
                entry_id,
                slo_state,
            } => write!(
                f,
                "sheet {entry_id} holds stable on stale packet {slo_state:?}"
            ),
            Self::HeldWithoutSignoff { entry_id } => {
                write!(f, "sheet {entry_id} holds stable without owner signoff")
            }
            Self::HeldWithUndisclosedImpact { entry_id } => write!(
                f,
                "sheet {entry_id} holds stable while hiding its version-bump impact"
            ),
            Self::HeldWithoutAuthDisclosure { entry_id, state } => write!(
                f,
                "sheet {entry_id} holds stable without explicit auth disclosure ({state:?})"
            ),
            Self::HeldWithoutDryRun { entry_id } => {
                write!(f, "sheet {entry_id} holds stable without a current dry run")
            }
            Self::HeldWithoutParity { entry_id, state } => write!(
                f,
                "sheet {entry_id} holds stable without review/plan parity ({state:?})"
            ),
            Self::HeldWithoutRollbackTarget { entry_id } => {
                write!(f, "sheet {entry_id} holds stable without a rollback target")
            }
            Self::HeldWithoutRolloutRing { entry_id } => {
                write!(
                    f,
                    "sheet {entry_id} holds stable without disclosing its rollout ring"
                )
            }
            Self::ClearedWithWaiver { entry_id } => {
                write!(f, "cleared sheet {entry_id} carries a waiver")
            }
            Self::DisclosureGapWithoutReason { entry_id, reason } => write!(
                f,
                "sheet {entry_id} disclosure gap requires active reason {}",
                reason.as_str()
            ),
            Self::MigrationFlagsMissing { entry_id, impact } => write!(
                f,
                "sheet {entry_id} impact {} discloses no migration flags",
                impact.as_str()
            ),
            Self::ParityBrokenWithoutReason { entry_id, state } => write!(
                f,
                "sheet {entry_id} parity {state:?} names no parity reason"
            ),
            Self::ParityMatchedButDigestsDiffer { entry_id } => write!(
                f,
                "sheet {entry_id} parity matched but descriptor/diff digests differ"
            ),
            Self::StateReasonIncoherent {
                entry_id,
                state,
                expected_reason,
            } => write!(
                f,
                "sheet {entry_id} state {state:?} requires reason {expected_reason:?}"
            ),
            Self::WaiverStateWithoutWaiver { entry_id, state } => {
                write!(f, "sheet {entry_id} state {state:?} names no waiver")
            }
            Self::BreachedPacketWithoutReason { entry_id } => write!(
                f,
                "sheet {entry_id} breached packet without proof_packet_stale reason"
            ),
            Self::MissingPacketWithoutReason { entry_id } => write!(
                f,
                "sheet {entry_id} missing packet without proof_packet_missing reason"
            ),
            Self::ReleaseBlockingLaneUncovered { lane_ref } => {
                write!(f, "release-blocking lane {lane_ref} has no covering sheet")
            }
            Self::ReleaseBlockingRowNotDeclared { entry_id } => write!(
                f,
                "release-blocking lane {entry_id} is not declared in release_blocking_lane_refs"
            ),
            Self::PublicationDecisionInconsistent { declared, computed } => {
                write!(
                    f,
                    "publication {declared:?} disagrees with computed {computed:?}"
                )
            }
            Self::PublicationBlockingSetMismatch { field } => {
                write!(f, "publication {field} disagrees with firing stop rules")
            }
            Self::SummaryMismatch => write!(f, "summary counts disagree with sheets"),
            Self::FreshnessSloInconsistent { entry_id } => {
                write!(f, "sheet {entry_id} freshness SLO window is inconsistent")
            }
        }
    }
}

impl Error for PublicationReviewViolation {}

/// Loads the embedded publication review-sheet register.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in register no longer matches
/// [`PublicationReviewRegister`].
pub fn current_publication_review_register() -> Result<PublicationReviewRegister, serde_json::Error>
{
    serde_json::from_str(PUBLICATION_REVIEW_JSON)
}

#[cfg(test)]
mod tests;
