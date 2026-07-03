//! Canonical advisory-claim downgrade certification across every claimed M5 deployment profile.
//!
//! The [frozen advisory-component matrix][matrix] already freezes Aureline's user-facing security
//! surfaces — the advisory card/row, the emergency notice, the affected-install panel, the
//! disclosure/history block, the advisory activity row, and the native-notification handoff — and
//! the five sibling implementation lanes narrow each family into a working resolver. This lane is
//! the **downgrade certification** capstone that decides, for every claimed deployment profile —
//! managed, self-hosted, and offline — whether Aureline may keep its release/help/procurement/
//! evaluation/support advisory claims, and *auto-narrows* those claims the moment advisory
//! freshness, mirror propagation, distribution signature, or local-continuity proof falls behind.
//!
//! Each profile row certifies four downgrade dimensions together:
//!
//! - **advisory freshness** — the notice state is current, not a stale notice sitting silently
//!   green;
//! - **mirror propagation** — the advisory has propagated to the profile's mirror rather than
//!   lagging behind upstream;
//! - **distribution signature** — the distribution the profile trusts is fully signed and
//!   verified, not unsigned or only partially verified;
//! - **local-continuity proof** — the profile can prove local work stays safe (no forced-disable
//!   scope hidden behind a generic banner).
//!
//! Three records carry the truth:
//!
//! - the per-profile **certification row** ([`AdvisoryClaimRow`]): one row per
//!   [`M5AdvisoryClaimProfile`] naming the claimed advisory-component families it evaluated
//!   (pulled from the matrix), the surfaces its downgrade state projects into, its
//!   advisory-freshness / mirror-propagation / distribution-signature / local-continuity posture,
//!   the distinct claim states it preserves, any active waiver, and a derived green/yellow/red
//!   [`AdvisoryClaimStatus`].
//! - the release **certification packet** ([`AdvisoryClaimPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   claim causes ([`AdvisoryClaimCause`]) — each naming the frozen downgrade trigger that fired
//!   and the [`M5AdvisoryRestoreAction`] that would restore the claim — and the blocking findings
//!   the lane refuses to ship with.
//! - the **certification dashboard** ([`AdvisoryClaimDashboard`]): a light projection the
//!   release / help / procurement / evaluation / support automation reads to auto-narrow a claimed
//!   advisory claim and paint the controlled badge when a profile's certification falls out of
//!   policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment
//! the profile discloses a stale notice, a lagging mirror, a partially verified distribution, or a
//! reduced local-continuity proof; it drops to `red` if any of those goes silent and overclaims,
//! local continuity is lost, or the profile fails to evaluate every claimed advisory family or
//! project into every claimed claim surface. That derivation is the auto-narrowing the acceptance
//! criteria require. Because managed, self-hosted, and offline profiles each carry their own
//! posture, the causes preserve the **distinct** downgrade reason (mirror-lagged vs
//! unsigned/unverified vs stale-notice vs continuity) rather than collapsing into one generic
//! "degraded" state, and each cause names the exact evidence or action that would restore the
//! claim.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The advisory-component family and downgrade-trigger
//! vocabulary is re-exported by reference from the already frozen [matrix]; the evaluated families
//! are pulled straight from that matrix's seeded packet, so this lane mints no parallel advisory
//! vocabulary and cannot certify a family the matrix does not freeze. Only the downgrade-claim
//! vocabulary ([`M5AdvisoryClaimProfile`], [`M5AdvisoryClaimDimension`], [`M5AdvisoryClaimState`],
//! [`M5AdvisoryClaimChannel`], [`M5AdvisoryRestoreAction`], [`AdvisoryClaimStatus`],
//! [`AdvisoryFreshnessState`], [`MirrorPropagationState`], [`DistributionSignatureState`],
//! [`LocalContinuityProofState`], [`AdvisoryClaimWaiver`], [`AdvisoryClaimCause`],
//! [`AdvisoryClaimFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix as matrix;

pub use matrix::{M5AdvisoryComponentFamily, M5AdvisoryDowngradeTrigger, M5AdvisoryQualificationClass};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_advisory_claim_downgrade_certification_packet,
    seeded_m5_advisory_claim_downgrade_certification_packet_managed_continuity_lost_blocked,
    seeded_m5_advisory_claim_downgrade_certification_packet_offline_stale_notice_blocked,
    seeded_m5_advisory_claim_downgrade_certification_packet_self_hosted_mirror_lag_blocked,
    seeded_m5_advisory_claim_downgrade_certification_packet_self_hosted_unsigned_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_SHARED_CONTRACT_REF: &str =
    "security:m5_advisory_claim_downgrade_certification:v1";

/// Stable record kind for [`AdvisoryClaimPacket`] payloads.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_PACKET_RECORD_KIND: &str =
    "security_m5_advisory_claim_downgrade_certification_packet_record";

/// Stable record kind for [`AdvisoryClaimDashboard`] payloads.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_DASHBOARD_RECORD_KIND: &str =
    "security_m5_advisory_claim_downgrade_certification_dashboard_record";

/// Stable record kind for [`AdvisoryClaimSupportExport`] payloads.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "security_m5_advisory_claim_downgrade_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_PACKET_ID: &str =
    "m5-advisory-claim-downgrade-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_DASHBOARD_ID: &str =
    "m5-advisory-claim-downgrade-certification-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-advisory-claim-downgrade-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_SOURCE_SCHEMA_REF: &str =
    "schemas/security/m5-advisory-claim-downgrade-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_REPORT_REF: &str =
    "artifacts/security/m5-advisory-claim-downgrade-certification.md";

/// Published certification-packet artifact ref.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-advisory-claim-downgrade-certification-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-advisory-claim-downgrade-certification-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-advisory-claim-downgrade-certification-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-advisory-claim-downgrade-certification-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_DOC_REF: &str =
    "docs/security/m5_advisory_claim_downgrade_certification_contract.md";

/// Repo-relative ref to the frozen advisory-component matrix schema.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_MATRIX_SCHEMA_REF: &str = matrix::M5_ADVISORY_COMPONENTS_SCHEMA_REF;

/// Advisory-card contract this proof mirrors for advisory-freshness truth.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_ADVISORY_CARD_CONTRACT_REF: &str =
    matrix::M5_ADVISORY_COMPONENTS_ADVISORY_CARD_CONTRACT_REF;

/// Affected-install contract this proof mirrors for local-continuity truth.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_AFFECTED_INSTALL_CONTRACT_REF: &str =
    matrix::M5_ADVISORY_COMPONENTS_AFFECTED_INSTALL_CONTRACT_REF;

/// Severity-matrix contract this proof mirrors for controlled severity vocabulary.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_SEVERITY_MATRIX_REF: &str =
    matrix::M5_ADVISORY_COMPONENTS_SEVERITY_MATRIX_REF;

/// Mirror/offline drill evidence this proof mirrors for mirror-propagation and offline continuity.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_MIRROR_OFFLINE_DRILL_REF: &str =
    "docs/release/finalize_security_response_advisory_cve_ghsa_publication_emergency_disable_and_mirror_offline_drills.md";

/// Continuity ship-room gate this proof feeds for release/procurement claim governance.
pub const M5_ADVISORY_CLAIM_DOWNGRADE_CONTINUITY_GATE_REF: &str =
    "docs/release/m5-continuity-shiproom-gates.md";

/// Every governed advisory-component family a profile row must evaluate, in canonical order.
/// These are exactly the families the frozen advisory-component matrix freezes; a profile that
/// evaluates fewer regresses into a partial view and blocks.
pub const REQUIRED_FAMILIES: [M5AdvisoryComponentFamily; 6] = M5AdvisoryComponentFamily::ALL;

/// Every claimed M5 deployment profile the certification must cover, in canonical order.
pub const REQUIRED_PROFILES: [M5AdvisoryClaimProfile; 3] = M5AdvisoryClaimProfile::ALL;

/// Every downgrade dimension each profile row certifies, in canonical order.
pub const REQUIRED_DIMENSIONS: [M5AdvisoryClaimDimension; 4] = M5AdvisoryClaimDimension::ALL;

/// Every claim surface each profile row must project its downgrade state into, in canonical order.
pub const REQUIRED_CHANNELS: [M5AdvisoryClaimChannel; 5] = M5AdvisoryClaimChannel::ALL;

/// A claimed M5 deployment profile the certification covers.
///
/// The three profiles are the deployment topologies Aureline already claims advisory truth for:
/// centrally managed installs, self-hosted installs mirroring the advisory feed themselves, and
/// offline installs consuming a signed bundle. Each keeps its own distinct downgrade reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryClaimProfile {
    /// Managed: centrally governed fleet where policy and the advisory feed are administered.
    Managed,
    /// Self-hosted: an install mirroring the advisory feed and distribution itself.
    SelfHosted,
    /// Offline: an air-gapped install consuming a signed advisory/distribution bundle.
    Offline,
}

impl M5AdvisoryClaimProfile {
    /// Every deployment profile, in declaration order.
    pub const ALL: [Self; 3] = [Self::Managed, Self::SelfHosted, Self::Offline];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Managed => "managed",
            Self::SelfHosted => "self_hosted",
            Self::Offline => "offline",
        }
    }

    /// Short reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Managed => "Managed (centrally governed fleet)",
            Self::SelfHosted => "Self-hosted (self-mirrored advisory feed)",
            Self::Offline => "Offline (signed advisory/distribution bundle)",
        }
    }
}

/// A downgrade dimension each profile row certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryClaimDimension {
    /// Advisory freshness: the notice state is current, not stale-but-silently-green.
    AdvisoryFreshness,
    /// Mirror propagation: the advisory has propagated to the profile's mirror.
    MirrorPropagation,
    /// Distribution signature: the trusted distribution is fully signed and verified.
    DistributionSignature,
    /// Local-continuity proof: local work stays safe and forced-disable scope is never hidden.
    LocalContinuity,
}

impl M5AdvisoryClaimDimension {
    /// Every downgrade dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AdvisoryFreshness,
        Self::MirrorPropagation,
        Self::DistributionSignature,
        Self::LocalContinuity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdvisoryFreshness => "advisory_freshness",
            Self::MirrorPropagation => "mirror_propagation",
            Self::DistributionSignature => "distribution_signature",
            Self::LocalContinuity => "local_continuity",
        }
    }
}

/// A distinct claim-narrowing state the certification preserves rather than collapsing into one
/// generic "degraded" wording.
///
/// The five states are the distinct downgrade paths the spec requires be kept apart: a warning
/// that does not disable, a forced-disable, an awaiting-user-action prompt, a mirror-lagged claim,
/// and an unsigned/unverified-distribution claim. `forced_disable` is a blocked (red) state; the
/// other four can be disclosed narrowings (yellow).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryClaimState {
    /// A warning is shown but the claim keeps working; nothing is disabled.
    WarningOnly,
    /// The claim is forcibly disabled and may not be published until repaired.
    ForcedDisable,
    /// The claim is narrowed pending an explicit user action.
    AwaitingUserAction,
    /// The claim is narrowed because the profile's mirror lags upstream.
    MirrorLagged,
    /// The claim is narrowed because the trusted distribution is unsigned or unverified.
    UnsignedUnverified,
}

impl M5AdvisoryClaimState {
    /// Every distinct claim state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::WarningOnly,
        Self::ForcedDisable,
        Self::AwaitingUserAction,
        Self::MirrorLagged,
        Self::UnsignedUnverified,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WarningOnly => "warning_only",
            Self::ForcedDisable => "forced_disable",
            Self::AwaitingUserAction => "awaiting_user_action",
            Self::MirrorLagged => "mirror_lagged",
            Self::UnsignedUnverified => "unsigned_unverified",
        }
    }
}

/// A claim surface the downgrade state is wired into.
///
/// Every profile row must project its downgrade state into all five so a narrowed claim never
/// stays green on one surface while it is narrowed on another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryClaimChannel {
    /// The release-evidence packet and controlled release badge.
    Release,
    /// The in-product help / about advisory surface.
    Help,
    /// The procurement / security-questionnaire claim surface.
    Procurement,
    /// The evaluation / trial maturity surface.
    Evaluation,
    /// The support / diagnostic export.
    Support,
}

impl M5AdvisoryClaimChannel {
    /// Every claim surface, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Release,
        Self::Help,
        Self::Procurement,
        Self::Evaluation,
        Self::Support,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::Help => "help",
            Self::Procurement => "procurement",
            Self::Evaluation => "evaluation",
            Self::Support => "support",
        }
    }
}

/// The evidence or action that would restore a narrowed advisory claim.
///
/// Every claim cause names one so a release/help/support reader learns not only *why* the claim
/// narrowed but *what would restore it*.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AdvisoryRestoreAction {
    /// Refresh the profile's mirror so the advisory propagates.
    RefreshMirror,
    /// Re-sign or re-verify the distribution the profile trusts.
    ReSignOrReverify,
    /// Acknowledge or take the required user action.
    AcknowledgeOrAct,
    /// Await the next advisory-notice refresh so the state is current again.
    AwaitNoticeRefresh,
    /// Restore the local-continuity proof so local work is provably safe.
    RestoreContinuityProof,
    /// No action required; the dimension holds.
    NoneRequired,
}

impl M5AdvisoryRestoreAction {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshMirror => "refresh_mirror",
            Self::ReSignOrReverify => "re_sign_or_reverify",
            Self::AcknowledgeOrAct => "acknowledge_or_act",
            Self::AwaitNoticeRefresh => "await_notice_refresh",
            Self::RestoreContinuityProof => "restore_continuity_proof",
            Self::NoneRequired => "none_required",
        }
    }
}

/// The derived advisory-claim-downgrade light a profile carries; doubles as the controlled badge.
///
/// `green` means the notice is fresh, the mirror is current, the distribution is fully signed and
/// verified, and local continuity is proven. `yellow` is a disclosed narrowing (a stale notice, a
/// lagging mirror, a partially verified distribution, or a reduced local-continuity proof, all
/// disclosed). `red` is blocked: a stale/lagging/unsigned state went silent and overclaimed,
/// local continuity was lost, or the profile did not evaluate every claimed advisory family or
/// project into every claimed claim surface — and it may not keep an advisory claim until
/// repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryClaimStatus {
    /// Full standing: every downgrade dimension holds.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl AdvisoryClaimStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// Stable controlled-badge token painted on claim surfaces.
    pub const fn controlled_badge_token(self) -> &'static str {
        match self {
            Self::Green => "advisory_claim_current",
            Self::Yellow => "advisory_claim_narrowed",
            Self::Red => "advisory_claim_blocked",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the profile keeps the advisory notice state fresh.
///
/// `fresh_advisory_state_certified` means the notice state is current. `disclosed_stale_notice_narrowing`
/// means the notice is stale and the claim is narrowed and disclosed (a warning-only yellow).
/// `advisory_state_stale_and_overclaimed` means a stale notice stayed silently green — a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryFreshnessState {
    /// The advisory notice state is current.
    FreshAdvisoryStateCertified,
    /// The notice is stale; the claim is narrowed and disclosed.
    DisclosedStaleNoticeNarrowing,
    /// A stale notice stayed silently green — a blocker.
    AdvisoryStateStaleAndOverclaimed,
}

impl AdvisoryFreshnessState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshAdvisoryStateCertified => "fresh_advisory_state_certified",
            Self::DisclosedStaleNoticeNarrowing => "disclosed_stale_notice_narrowing",
            Self::AdvisoryStateStaleAndOverclaimed => "advisory_state_stale_and_overclaimed",
        }
    }

    /// `true` when the notice is current at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::FreshAdvisoryStateCertified)
    }

    /// `true` when the profile took a disclosed stale-notice narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedStaleNoticeNarrowing)
    }
}

/// How the profile keeps its advisory mirror propagated.
///
/// `mirror_current_and_propagated` means the advisory reached the profile's mirror.
/// `disclosed_mirror_lag_narrowing` means the mirror lags and the claim is narrowed and disclosed
/// (a mirror-lagged yellow). `mirror_lagged_claim_overclaimed` means mirror lag stayed silently
/// green — a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorPropagationState {
    /// The advisory reached the profile's mirror.
    MirrorCurrentAndPropagated,
    /// The mirror lags; the claim is narrowed and disclosed.
    DisclosedMirrorLagNarrowing,
    /// Mirror lag stayed silently green — a blocker.
    MirrorLaggedClaimOverclaimed,
}

impl MirrorPropagationState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorCurrentAndPropagated => "mirror_current_and_propagated",
            Self::DisclosedMirrorLagNarrowing => "disclosed_mirror_lag_narrowing",
            Self::MirrorLaggedClaimOverclaimed => "mirror_lagged_claim_overclaimed",
        }
    }

    /// `true` when the mirror is current at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::MirrorCurrentAndPropagated)
    }

    /// `true` when the profile took a disclosed mirror-lag narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedMirrorLagNarrowing)
    }
}

/// How the profile keeps the distribution it trusts signed and verified.
///
/// `fully_signed_and_verified` means the distribution is fully signed and verified.
/// `disclosed_partial_verification_narrowing` means only part of the distribution is verified and
/// the claim is narrowed and disclosed (an unsigned/unverified yellow). `unsigned_or_unverified_distribution`
/// means an unsigned or unverified distribution stayed silently green — a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DistributionSignatureState {
    /// The distribution is fully signed and verified.
    FullySignedAndVerified,
    /// Only part of the distribution is verified; the claim is narrowed and disclosed.
    DisclosedPartialVerificationNarrowing,
    /// An unsigned or unverified distribution stayed silently green — a blocker.
    UnsignedOrUnverifiedDistribution,
}

impl DistributionSignatureState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullySignedAndVerified => "fully_signed_and_verified",
            Self::DisclosedPartialVerificationNarrowing => "disclosed_partial_verification_narrowing",
            Self::UnsignedOrUnverifiedDistribution => "unsigned_or_unverified_distribution",
        }
    }

    /// `true` when the distribution is fully verified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::FullySignedAndVerified)
    }

    /// `true` when the profile took a disclosed partial-verification narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialVerificationNarrowing)
    }
}

/// How the profile keeps proving local work stays safe.
///
/// `local_continuity_proven_and_safe` means local continuity is proven and forced-disable scope is
/// never hidden. `disclosed_reduced_continuity_proof` means the continuity proof is reduced pending
/// a user action and the claim is narrowed and disclosed (an awaiting-user-action yellow), which
/// requires a waiver. `continuity_proof_missing_or_unsafe` means local continuity was lost or
/// forced-disable scope was hidden — a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalContinuityProofState {
    /// Local continuity is proven and forced-disable scope is never hidden.
    LocalContinuityProvenAndSafe,
    /// The continuity proof is reduced pending a user action; the claim is narrowed and disclosed.
    DisclosedReducedContinuityProof,
    /// Local continuity was lost or forced-disable scope was hidden — a blocker.
    ContinuityProofMissingOrUnsafe,
}

impl LocalContinuityProofState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalContinuityProvenAndSafe => "local_continuity_proven_and_safe",
            Self::DisclosedReducedContinuityProof => "disclosed_reduced_continuity_proof",
            Self::ContinuityProofMissingOrUnsafe => "continuity_proof_missing_or_unsafe",
        }
    }

    /// `true` when local continuity is proven at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::LocalContinuityProvenAndSafe)
    }

    /// `true` when the profile took a disclosed reduced-continuity-proof narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedContinuityProof)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow)
/// rather than blocked — never lets a stale notice, a lagging mirror, an unsigned distribution, or
/// a lost local continuity hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryClaimWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The deployment profile the waiver applies to.
    pub profile: M5AdvisoryClaimProfile,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl AdvisoryClaimWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a profile's advisory claim.
///
/// The trigger token mirrors the frozen [`M5AdvisoryDowngradeTrigger`] vocabulary so a cause never
/// mints a parallel reason synonym, and the restore action names what would restore the claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryClaimCause {
    /// The deployment profile the cause applies to.
    pub profile: M5AdvisoryClaimProfile,
    /// The downgrade dimension the cause narrowed.
    pub dimension: M5AdvisoryClaimDimension,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5AdvisoryDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is
    /// a blocker.
    pub disclosed: bool,
    /// The evidence or action that would restore the claim.
    pub restore_action: M5AdvisoryRestoreAction,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl AdvisoryClaimCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One claimed deployment profile, certified across its advisory-freshness, mirror-propagation,
/// distribution-signature, and local-continuity posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryClaimRow {
    /// The deployment profile being certified.
    pub profile: M5AdvisoryClaimProfile,
    /// Short reviewer-facing profile label.
    pub profile_label: String,
    /// Owner role accountable for keeping this profile governed.
    pub owner_role: String,
    /// Short scenario summary describing the profile's advisory condition.
    pub scenario_summary: String,
    /// Claimed advisory-component families evaluated under this profile. Pulled from the matrix.
    pub evaluated_families: Vec<M5AdvisoryComponentFamily>,
    /// Claim surfaces this row's downgrade state projects into.
    pub projected_channels: Vec<M5AdvisoryClaimChannel>,
    /// Advisory-freshness posture.
    pub advisory_freshness: AdvisoryFreshnessState,
    /// Mirror-propagation posture.
    pub mirror_propagation: MirrorPropagationState,
    /// Distribution-signature posture.
    pub distribution_signature: DistributionSignatureState,
    /// Local-continuity-proof posture.
    pub local_continuity: LocalContinuityProofState,
    /// Downgrade triggers that apply to this profile.
    pub applicable_downgrade_triggers: Vec<M5AdvisoryDowngradeTrigger>,
    /// Active waiver, when a disclosed reduced-continuity proof is in force.
    pub active_waiver: Option<AdvisoryClaimWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: AdvisoryClaimStatus,
    /// Distinct claim states preserved for this row. Recomputed by the builder; never asserted.
    pub claim_states: Vec<M5AdvisoryClaimState>,
    /// The exact claim causes that narrowed or blocked this row.
    pub claim_causes: Vec<AdvisoryClaimCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl AdvisoryClaimRow {
    /// `true` when the row evaluated all six claimed advisory-component families — no claimed
    /// family is left uncertified under this profile and none is invented.
    pub fn families_complete(&self) -> bool {
        let mut declared: Vec<&str> = self.evaluated_families.iter().map(|f| f.as_str()).collect();
        declared.sort_unstable();
        let mut required: Vec<&str> = REQUIRED_FAMILIES.iter().map(|f| f.as_str()).collect();
        required.sort_unstable();
        declared == required && self.evaluated_families.len() == REQUIRED_FAMILIES.len()
    }

    /// `true` when the row projects its downgrade state into all five claimed claim surfaces.
    pub fn channels_complete(&self) -> bool {
        let mut declared: Vec<&str> = self.projected_channels.iter().map(|c| c.as_str()).collect();
        declared.sort_unstable();
        let mut required: Vec<&str> = REQUIRED_CHANNELS.iter().map(|c| c.as_str()).collect();
        required.sort_unstable();
        declared == required && self.projected_channels.len() == REQUIRED_CHANNELS.len()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.families_complete() {
            return true;
        }
        if !self.channels_complete() {
            return true;
        }
        if matches!(
            self.advisory_freshness,
            AdvisoryFreshnessState::AdvisoryStateStaleAndOverclaimed
        ) {
            return true;
        }
        if matches!(
            self.mirror_propagation,
            MirrorPropagationState::MirrorLaggedClaimOverclaimed
        ) {
            return true;
        }
        if matches!(
            self.distribution_signature,
            DistributionSignatureState::UnsignedOrUnverifiedDistribution
        ) {
            return true;
        }
        if matches!(
            self.local_continuity,
            LocalContinuityProofState::ContinuityProofMissingOrUnsafe
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.advisory_freshness.is_disclosed_narrowing()
            || self.mirror_propagation.is_disclosed_narrowing()
            || self.distribution_signature.is_disclosed_narrowing()
            || self.local_continuity.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the profile posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> AdvisoryClaimStatus {
        if self.has_hard_blocker() {
            AdvisoryClaimStatus::Red
        } else if self.has_narrowing() {
            AdvisoryClaimStatus::Yellow
        } else {
            AdvisoryClaimStatus::Green
        }
    }

    /// Recomputes the distinct claim states this row preserves, in canonical order.
    ///
    /// A narrowed profile keeps its own distinct downgrade path — warning-only, mirror-lagged,
    /// unsigned/unverified, awaiting-user-action, or forced-disable — instead of collapsing into a
    /// single generic "degraded" state.
    pub fn recompute_claim_states(&self) -> Vec<M5AdvisoryClaimState> {
        let mut states: BTreeSet<M5AdvisoryClaimState> = BTreeSet::new();
        if !matches!(self.advisory_freshness, AdvisoryFreshnessState::FreshAdvisoryStateCertified) {
            states.insert(M5AdvisoryClaimState::WarningOnly);
        }
        if !matches!(self.mirror_propagation, MirrorPropagationState::MirrorCurrentAndPropagated) {
            states.insert(M5AdvisoryClaimState::MirrorLagged);
        }
        if !matches!(
            self.distribution_signature,
            DistributionSignatureState::FullySignedAndVerified
        ) {
            states.insert(M5AdvisoryClaimState::UnsignedUnverified);
        }
        if self.local_continuity.is_disclosed_narrowing() {
            states.insert(M5AdvisoryClaimState::AwaitingUserAction);
        }
        if matches!(self.recompute_status(), AdvisoryClaimStatus::Red) {
            states.insert(M5AdvisoryClaimState::ForcedDisable);
        }
        // Canonical declaration order.
        M5AdvisoryClaimState::ALL
            .into_iter()
            .filter(|state| states.contains(state))
            .collect()
    }

    /// Recomputes the exact claim causes for the row, in deterministic dimension order.
    pub fn recompute_causes(&self) -> Vec<AdvisoryClaimCause> {
        let mut causes = Vec::new();
        match self.advisory_freshness {
            AdvisoryFreshnessState::FreshAdvisoryStateCertified => {}
            AdvisoryFreshnessState::DisclosedStaleNoticeNarrowing => {
                causes.push(AdvisoryClaimCause {
                    profile: self.profile,
                    dimension: M5AdvisoryClaimDimension::AdvisoryFreshness,
                    trigger: M5AdvisoryDowngradeTrigger::StaleNoticeStateSilent,
                    disclosed: true,
                    restore_action: M5AdvisoryRestoreAction::AwaitNoticeRefresh,
                    detail: "The advisory notice state is stale under this profile, so the claim is \
                             narrowed to a disclosed warning rather than left silently green until \
                             the next notice refresh lands."
                        .to_owned(),
                });
            }
            AdvisoryFreshnessState::AdvisoryStateStaleAndOverclaimed => {
                causes.push(AdvisoryClaimCause {
                    profile: self.profile,
                    dimension: M5AdvisoryClaimDimension::AdvisoryFreshness,
                    trigger: M5AdvisoryDowngradeTrigger::StaleNoticeStateSilent,
                    disclosed: false,
                    restore_action: M5AdvisoryRestoreAction::AwaitNoticeRefresh,
                    detail: "A stale advisory notice stayed silently green under this profile, \
                             overclaiming currency instead of narrowing the claim."
                        .to_owned(),
                });
            }
        }
        match self.mirror_propagation {
            MirrorPropagationState::MirrorCurrentAndPropagated => {}
            MirrorPropagationState::DisclosedMirrorLagNarrowing => {
                causes.push(AdvisoryClaimCause {
                    profile: self.profile,
                    dimension: M5AdvisoryClaimDimension::MirrorPropagation,
                    trigger: M5AdvisoryDowngradeTrigger::MirrorLagUndisclosed,
                    disclosed: true,
                    restore_action: M5AdvisoryRestoreAction::RefreshMirror,
                    detail: "The profile's advisory mirror lags upstream, so the claim is narrowed \
                             to a disclosed mirror-lagged state until the mirror is refreshed."
                        .to_owned(),
                });
            }
            MirrorPropagationState::MirrorLaggedClaimOverclaimed => {
                causes.push(AdvisoryClaimCause {
                    profile: self.profile,
                    dimension: M5AdvisoryClaimDimension::MirrorPropagation,
                    trigger: M5AdvisoryDowngradeTrigger::MirrorLagUndisclosed,
                    disclosed: false,
                    restore_action: M5AdvisoryRestoreAction::RefreshMirror,
                    detail: "The profile's advisory mirror lagged upstream but the claim stayed \
                             silently green, overclaiming propagation instead of narrowing."
                        .to_owned(),
                });
            }
        }
        match self.distribution_signature {
            DistributionSignatureState::FullySignedAndVerified => {}
            DistributionSignatureState::DisclosedPartialVerificationNarrowing => {
                causes.push(AdvisoryClaimCause {
                    profile: self.profile,
                    dimension: M5AdvisoryClaimDimension::DistributionSignature,
                    trigger: M5AdvisoryDowngradeTrigger::UnsignedDistributionUndisclosed,
                    disclosed: true,
                    restore_action: M5AdvisoryRestoreAction::ReSignOrReverify,
                    detail: "Only part of the distribution the profile trusts is verified, so the \
                             claim is narrowed to a disclosed unsigned/unverified state until the \
                             distribution is fully re-signed or re-verified."
                        .to_owned(),
                });
            }
            DistributionSignatureState::UnsignedOrUnverifiedDistribution => {
                causes.push(AdvisoryClaimCause {
                    profile: self.profile,
                    dimension: M5AdvisoryClaimDimension::DistributionSignature,
                    trigger: M5AdvisoryDowngradeTrigger::UnsignedDistributionUndisclosed,
                    disclosed: false,
                    restore_action: M5AdvisoryRestoreAction::ReSignOrReverify,
                    detail: "An unsigned or unverified distribution stayed silently green under \
                             this profile, overclaiming verified provenance instead of narrowing."
                        .to_owned(),
                });
            }
        }
        match self.local_continuity {
            LocalContinuityProofState::LocalContinuityProvenAndSafe => {}
            LocalContinuityProofState::DisclosedReducedContinuityProof => {
                causes.push(AdvisoryClaimCause {
                    profile: self.profile,
                    dimension: M5AdvisoryClaimDimension::LocalContinuity,
                    trigger: M5AdvisoryDowngradeTrigger::LocalContinuityHidden,
                    disclosed: true,
                    restore_action: M5AdvisoryRestoreAction::AcknowledgeOrAct,
                    detail: "The local-continuity proof is reduced pending a user action under this \
                             profile, so the claim is narrowed to a disclosed, waivered \
                             awaiting-user-action state while local work stays visibly safe."
                        .to_owned(),
                });
            }
            LocalContinuityProofState::ContinuityProofMissingOrUnsafe => {
                causes.push(AdvisoryClaimCause {
                    profile: self.profile,
                    dimension: M5AdvisoryClaimDimension::LocalContinuity,
                    trigger: M5AdvisoryDowngradeTrigger::LocalContinuityHidden,
                    disclosed: false,
                    restore_action: M5AdvisoryRestoreAction::RestoreContinuityProof,
                    detail: "Local continuity was lost or forced-disable scope was hidden under \
                             this profile, so the claim is forcibly disabled until the continuity \
                             proof is restored."
                        .to_owned(),
                });
            }
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced-continuity proof may only stay yellow (rather than red) when a waiver
    /// discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.local_continuity,
            LocalContinuityProofState::DisclosedReducedContinuityProof
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<AdvisoryClaimFinding> {
        let mut findings = Vec::new();
        let profile = self.profile.as_str().to_owned();

        if !self.families_complete() {
            findings.push(AdvisoryClaimFinding::EvaluatedFamiliesIncomplete {
                profile: profile.clone(),
            });
        }
        if !self.channels_complete() {
            findings.push(AdvisoryClaimFinding::ProjectedChannelsIncomplete {
                profile: profile.clone(),
            });
        }
        if matches!(
            self.advisory_freshness,
            AdvisoryFreshnessState::AdvisoryStateStaleAndOverclaimed
        ) {
            findings.push(AdvisoryClaimFinding::AdvisoryStateStaleAndOverclaimed {
                profile: profile.clone(),
            });
        }
        if matches!(
            self.mirror_propagation,
            MirrorPropagationState::MirrorLaggedClaimOverclaimed
        ) {
            findings.push(AdvisoryClaimFinding::MirrorLaggedClaimOverclaimed {
                profile: profile.clone(),
            });
        }
        if matches!(
            self.distribution_signature,
            DistributionSignatureState::UnsignedOrUnverifiedDistribution
        ) {
            findings.push(AdvisoryClaimFinding::UnsignedOrUnverifiedDistribution {
                profile: profile.clone(),
            });
        }
        if matches!(
            self.local_continuity,
            LocalContinuityProofState::ContinuityProofMissingOrUnsafe
        ) {
            findings.push(AdvisoryClaimFinding::ContinuityProofMissingOrUnsafe {
                profile: profile.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, AdvisoryClaimStatus::Green) && !self.has_reason() {
            findings.push(AdvisoryClaimFinding::NarrowedRowWithoutReason {
                profile: profile.clone(),
            });
        }
        // A narrowed/blocked row must keep at least one distinct claim state — never a generic
        // "degraded" collapse.
        if !matches!(derived, AdvisoryClaimStatus::Green) && self.recompute_claim_states().is_empty() {
            findings.push(AdvisoryClaimFinding::NarrowedRowWithoutDistinctState {
                profile: profile.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(AdvisoryClaimFinding::NarrowedRowWithoutWaiver {
                profile: profile.clone(),
            });
        }
        // An attached waiver must still be active and must point at this profile.
        if let Some(waiver) = &self.active_waiver {
            if waiver.profile != self.profile {
                findings.push(AdvisoryClaimFinding::WaiverProfileMismatch {
                    profile: profile.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(AdvisoryClaimFinding::WaiverExpired {
                    profile: profile.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(AdvisoryClaimFinding::RowStatusStale {
                profile: profile.clone(),
            });
        }
        if self.claim_states != self.recompute_claim_states() {
            findings.push(AdvisoryClaimFinding::RowClaimStatesStale {
                profile: profile.clone(),
            });
        }
        if self.claim_causes != self.recompute_causes() {
            findings.push(AdvisoryClaimFinding::RowCausesStale { profile });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} badge={} freshness={} mirror={} signature={} continuity={} states=[{}] families={} channels={} waiver={}",
            self.profile.as_str(),
            self.derived_status.as_str(),
            self.derived_status.controlled_badge_token(),
            self.advisory_freshness.as_str(),
            self.mirror_propagation.as_str(),
            self.distribution_signature.as_str(),
            self.local_continuity.as_str(),
            self.claim_states
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join("|"),
            self.evaluated_families.len(),
            self.projected_channels.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the advisory-claim-downgrade certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum AdvisoryClaimFinding {
    /// A claimed deployment profile has no certification row.
    ProfileMissing {
        /// The missing profile token.
        profile: String,
    },
    /// A row did not evaluate all six claimed advisory-component families.
    EvaluatedFamiliesIncomplete {
        /// The profile token.
        profile: String,
    },
    /// A row did not project its downgrade state into all five claimed claim surfaces.
    ProjectedChannelsIncomplete {
        /// The profile token.
        profile: String,
    },
    /// A stale advisory notice stayed silently green and overclaimed currency.
    AdvisoryStateStaleAndOverclaimed {
        /// The profile token.
        profile: String,
    },
    /// A lagging advisory mirror stayed silently green and overclaimed propagation.
    MirrorLaggedClaimOverclaimed {
        /// The profile token.
        profile: String,
    },
    /// An unsigned or unverified distribution stayed silently green.
    UnsignedOrUnverifiedDistribution {
        /// The profile token.
        profile: String,
    },
    /// Local continuity was lost or forced-disable scope was hidden.
    ContinuityProofMissingOrUnsafe {
        /// The profile token.
        profile: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The profile token.
        profile: String,
    },
    /// A narrowed or blocked row collapsed into a generic degraded state with no distinct claim
    /// state preserved.
    NarrowedRowWithoutDistinctState {
        /// The profile token.
        profile: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The profile token.
        profile: String,
    },
    /// An attached waiver does not point at the row's profile.
    WaiverProfileMismatch {
        /// The profile token.
        profile: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The profile token.
        profile: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The profile token.
        profile: String,
    },
    /// The declared claim states do not match the recomputed claim states.
    RowClaimStatesStale {
        /// The profile token.
        profile: String,
    },
    /// The declared claim causes do not match the recomputed causes.
    RowCausesStale {
        /// The profile token.
        profile: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered profiles do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl AdvisoryClaimFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ProfileMissing { .. } => "profile_missing",
            Self::EvaluatedFamiliesIncomplete { .. } => "evaluated_families_incomplete",
            Self::ProjectedChannelsIncomplete { .. } => "projected_channels_incomplete",
            Self::AdvisoryStateStaleAndOverclaimed { .. } => "advisory_state_stale_and_overclaimed",
            Self::MirrorLaggedClaimOverclaimed { .. } => "mirror_lagged_claim_overclaimed",
            Self::UnsignedOrUnverifiedDistribution { .. } => "unsigned_or_unverified_distribution",
            Self::ContinuityProofMissingOrUnsafe { .. } => "continuity_proof_missing_or_unsafe",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutDistinctState { .. } => "narrowed_row_without_distinct_state",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverProfileMismatch { .. } => "waiver_profile_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowClaimStatesStale { .. } => "row_claim_states_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::ProfileMissing { profile }
            | Self::EvaluatedFamiliesIncomplete { profile }
            | Self::ProjectedChannelsIncomplete { profile }
            | Self::AdvisoryStateStaleAndOverclaimed { profile }
            | Self::MirrorLaggedClaimOverclaimed { profile }
            | Self::UnsignedOrUnverifiedDistribution { profile }
            | Self::ContinuityProofMissingOrUnsafe { profile }
            | Self::NarrowedRowWithoutReason { profile }
            | Self::NarrowedRowWithoutDistinctState { profile }
            | Self::NarrowedRowWithoutWaiver { profile }
            | Self::WaiverProfileMismatch { profile, .. }
            | Self::WaiverExpired { profile, .. }
            | Self::RowStatusStale { profile }
            | Self::RowClaimStatesStale { profile }
            | Self::RowCausesStale { profile } => profile,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release advisory-claim-downgrade certification packet shared by the release / help /
/// procurement / evaluation / support automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryClaimPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen advisory-component matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen advisory-component matrix schema.
    pub matrix_schema_ref: String,
    /// Advisory-card contract this proof mirrors for advisory-freshness truth.
    pub advisory_card_contract_ref: String,
    /// Affected-install contract this proof mirrors for local-continuity truth.
    pub affected_install_contract_ref: String,
    /// Severity-matrix contract this proof mirrors for controlled severity vocabulary.
    pub severity_matrix_ref: String,
    /// Mirror/offline drill evidence this proof mirrors for mirror-propagation and offline
    /// continuity.
    pub mirror_offline_drill_ref: String,
    /// Continuity ship-room gate this proof feeds for release/procurement claim governance.
    pub continuity_gate_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four downgrade dimensions every profile row certifies.
    pub required_dimensions: Vec<String>,
    /// The three claimed deployment profiles the certification must cover.
    pub required_profiles: Vec<String>,
    /// The six claimed advisory-component families every profile row must evaluate.
    pub required_families: Vec<String>,
    /// The five claim surfaces every profile row must project into.
    pub required_channels: Vec<String>,
    /// The five distinct claim states the certification preserves.
    pub required_claim_states: Vec<String>,
    /// Per-profile certification rows, in canonical order.
    pub rows: Vec<AdvisoryClaimRow>,
    /// Deployment profiles certified, in canonical (sorted) order.
    pub covered_profiles: Vec<String>,
    /// Distinct claim states observed across the rows, in canonical order.
    pub covered_claim_states: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-standing) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<AdvisoryClaimWaiver>,
    /// Every exact claim cause, in row then cause order.
    pub claim_causes: Vec<AdvisoryClaimCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<AdvisoryClaimFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Release / help / procurement / evaluation automation refs that consume this packet to
    /// auto-narrow claimed advisory claims.
    pub claim_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl AdvisoryClaimPacket {
    /// Returns the certification row for `profile`, if present.
    pub fn row(&self, profile: M5AdvisoryClaimProfile) -> Option<&AdvisoryClaimRow> {
        self.rows.iter().find(|row| row.profile == profile)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={} states=[{}]",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
                self.covered_claim_states.join("|"),
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.profile.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.claim_causes {
            lines.push(format!(
                "  cause {} {} {} disclosed={} restore={}",
                cause.profile.as_str(),
                cause.dimension.as_str(),
                cause.cause_token(),
                cause.disclosed,
                cause.restore_action.as_str(),
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the release automation consumes.
    pub fn dashboard(&self) -> AdvisoryClaimDashboard {
        AdvisoryClaimDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 advisory-claim-downgrade certification packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per profile naming its status,
    /// controlled badge, the four dimension postures, the distinct claim states, the evaluated
    /// family and projected channel counts, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "profile,status,badge,advisory_freshness,mirror_propagation,distribution_signature,local_continuity,claim_states,evaluated_families,projected_channels,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.profile.as_str(),
                row.derived_status.as_str(),
                row.derived_status.controlled_badge_token(),
                row.advisory_freshness.as_str(),
                row.mirror_propagation.as_str(),
                row.distribution_signature.as_str(),
                row.local_continuity.as_str(),
                row.claim_states
                    .iter()
                    .map(|s| s.as_str())
                    .collect::<Vec<_>>()
                    .join("|"),
                row.evaluated_families.len(),
                row.projected_channels.len(),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 advisory-claim downgrade certification: stale-advisory, mirror-lag, unsigned-distribution, and continuity-downgrade rules across managed, self-hosted, and offline profiles\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_advisory_claim_downgrade_certification`](../../crates/aureline-shell/src/m5_advisory_claim_downgrade_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- markdown > \\\n  artifacts/security/m5-advisory-claim-downgrade-certification.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!("- Source schema ref: `{}`\n", self.source_schema_ref));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!("- Release channel: `{}`\n", self.release_channel_class));
        out.push_str(&format!(
            "- Required dimensions: {}\n",
            self.required_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Required profiles: {}\n",
            self.required_profiles
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Required claim surfaces: {}\n",
            self.required_channels
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Distinct claim states preserved: {}\n",
            self.required_claim_states
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!("- Green (full standing): {}\n", self.green_row_count));
        out.push_str(&format!("- Yellow (auto-narrowed): {}\n", self.yellow_row_count));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean { "clean" } else { "blocked" }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Profile | Status | Badge | Advisory freshness | Mirror propagation | Distribution signature | Local continuity | Waiver |\n\
             | ------- | ------ | ----- | ------------------ | ------------------ | ---------------------- | ---------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.profile_label,
                row.derived_status.as_str(),
                row.derived_status.controlled_badge_token(),
                row.advisory_freshness.as_str(),
                row.mirror_propagation.as_str(),
                row.distribution_signature.as_str(),
                row.local_continuity.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&AdvisoryClaimRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, AdvisoryClaimStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed deployment profile keeps a fresh advisory notice, a current mirror, a fully signed distribution, and proven local continuity.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`, states `{}`) — {}\n",
                    row.profile.as_str(),
                    row.derived_status.as_str(),
                    row.claim_states
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>()
                        .join("|"),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact claim causes\n\n");
        if self.claim_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.claim_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` / `{}` (disclosed: `{}`, restore: `{}`) — {}\n",
                    cause.profile.as_str(),
                    cause.dimension.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.restore_action.as_str(),
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.profile.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_claim_downgrade_certification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_advisory_claim_downgrade_certification_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryClaimDashboardRow {
    /// The deployment profile.
    pub profile: M5AdvisoryClaimProfile,
    /// Short profile label.
    pub profile_label: String,
    /// Derived green/yellow/red status.
    pub status: AdvisoryClaimStatus,
    /// Controlled badge token painted on claim surfaces.
    pub controlled_badge_token: String,
    /// Number of claimed advisory families evaluated under this profile.
    pub evaluated_family_count: usize,
    /// Number of claim surfaces the downgrade state projects into.
    pub projected_channel_count: usize,
    /// Advisory-freshness posture.
    pub advisory_freshness: AdvisoryFreshnessState,
    /// Mirror-propagation posture.
    pub mirror_propagation: MirrorPropagationState,
    /// Distribution-signature posture.
    pub distribution_signature: DistributionSignatureState,
    /// Local-continuity-proof posture.
    pub local_continuity: LocalContinuityProofState,
    /// Distinct claim states preserved.
    pub claim_states: Vec<M5AdvisoryClaimState>,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Restore actions that would restore the claim, in cause order.
    pub restore_actions: Vec<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the release / help / procurement / evaluation / support
/// automation reads to auto-narrow claimed advisory claims and paint the controlled badge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryClaimDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<AdvisoryClaimDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Distinct claim states observed across the rows.
    pub covered_claim_states: Vec<String>,
    /// Release / help / procurement / evaluation automation refs that consume the dashboard.
    pub claim_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl AdvisoryClaimDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &AdvisoryClaimPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| AdvisoryClaimDashboardRow {
                profile: row.profile,
                profile_label: row.profile_label.clone(),
                status: row.derived_status,
                controlled_badge_token: row.derived_status.controlled_badge_token().to_owned(),
                evaluated_family_count: row.evaluated_families.len(),
                projected_channel_count: row.projected_channels.len(),
                advisory_freshness: row.advisory_freshness,
                mirror_propagation: row.mirror_propagation,
                distribution_signature: row.distribution_signature,
                local_continuity: row.local_continuity,
                claim_states: row.claim_states.clone(),
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                restore_actions: row
                    .claim_causes
                    .iter()
                    .map(|cause| cause.restore_action.as_str().to_owned())
                    .collect(),
                cause_tokens: row
                    .claim_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_ADVISORY_CLAIM_DOWNGRADE_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_ADVISORY_CLAIM_DOWNGRADE_SCHEMA_VERSION,
            dashboard_id: M5_ADVISORY_CLAIM_DOWNGRADE_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            covered_claim_states: packet.covered_claim_states.clone(),
            claim_automation_refs: packet.claim_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 advisory-claim-downgrade certification dashboard serializes")
    }
}

/// Support-export wrapper for the advisory-claim-downgrade certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryClaimSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: AdvisoryClaimPacket,
    /// Dashboard quoted in full.
    pub dashboard: AdvisoryClaimDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl AdvisoryClaimSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each profile, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the claim automation — can name
    /// the same profile and waiver the runtime certified.
    pub fn from_packet(support_export_id: impl Into<String>, packet: AdvisoryClaimPacket) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.profile.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_ADVISORY_CLAIM_DOWNGRADE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ADVISORY_CLAIM_DOWNGRADE_SCHEMA_VERSION,
            shared_contract_ref: M5_ADVISORY_CLAIM_DOWNGRADE_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_advisory_claim_downgrade_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvisoryClaimInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen advisory-component matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-profile certification rows.
    pub rows: Vec<AdvisoryClaimRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The certification packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds an [`AdvisoryClaimPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-profile certification rows.
///
/// Each row's derived status, distinct claim states, and claim causes, the aggregate counts, the
/// active waivers, and the blocking findings are recomputed here so the packet is the single
/// source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_advisory_claim_downgrade_certification_packet(
    input: AdvisoryClaimInput,
) -> AdvisoryClaimPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status, claim states, and causes so the packet is
    // self-consistent and the auto-narrowing is the single source of truth.
    let rows: Vec<AdvisoryClaimRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.claim_states = row.recompute_claim_states();
            row.claim_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<AdvisoryClaimFinding> = Vec::new();

    // Every claimed profile must carry a certification row.
    let present: BTreeSet<M5AdvisoryClaimProfile> = rows.iter().map(|row| row.profile).collect();
    for profile in REQUIRED_PROFILES {
        if !present.contains(&profile) {
            blocking_findings.push(AdvisoryClaimFinding::ProfileMissing {
                profile: profile.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_profiles: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|profile| profile.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let covered_claim_states: Vec<String> = {
        let observed: BTreeSet<M5AdvisoryClaimState> =
            rows.iter().flat_map(|row| row.claim_states.clone()).collect();
        M5AdvisoryClaimState::ALL
            .into_iter()
            .filter(|state| observed.contains(state))
            .map(|state| state.as_str().to_owned())
            .collect()
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AdvisoryClaimStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AdvisoryClaimStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AdvisoryClaimStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(AdvisoryClaimFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<AdvisoryClaimWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let claim_causes: Vec<AdvisoryClaimCause> = rows
        .iter()
        .flat_map(|row| row.claim_causes.clone())
        .collect();

    let required_dimensions: Vec<String> = REQUIRED_DIMENSIONS
        .iter()
        .map(|dim| dim.as_str().to_owned())
        .collect();
    let required_profiles: Vec<String> = REQUIRED_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    let required_families: Vec<String> = REQUIRED_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    let required_channels: Vec<String> = REQUIRED_CHANNELS
        .iter()
        .map(|channel| channel.as_str().to_owned())
        .collect();
    let required_claim_states: Vec<String> = M5AdvisoryClaimState::ALL
        .iter()
        .map(|state| state.as_str().to_owned())
        .collect();

    let mut packet = AdvisoryClaimPacket {
        record_kind: M5_ADVISORY_CLAIM_DOWNGRADE_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_ADVISORY_CLAIM_DOWNGRADE_SCHEMA_VERSION,
        shared_contract_ref: M5_ADVISORY_CLAIM_DOWNGRADE_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_ADVISORY_CLAIM_DOWNGRADE_PACKET_ID.to_owned(),
        source_schema_ref: M5_ADVISORY_CLAIM_DOWNGRADE_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Stale-advisory, mirror-lag, unsigned-distribution, and continuity-downgrade \
                   rules across every claimed M5 deployment profile: managed, self-hosted, and \
                   offline each certified so a stale advisory notice, a lagging mirror, an unsigned \
                   or partially verified distribution, or a missing local-continuity proof \
                   auto-narrows the release/help/procurement/evaluation/support claim to a distinct \
                   downgrade reason with a named restore action, instead of silently preserving a \
                   stronger trust badge, with each profile's green/yellow/red claim auto-narrowed \
                   from its advisory-freshness, mirror-propagation, distribution-signature, and \
                   local-continuity posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_ADVISORY_CLAIM_DOWNGRADE_MATRIX_SCHEMA_REF.to_owned(),
        advisory_card_contract_ref: M5_ADVISORY_CLAIM_DOWNGRADE_ADVISORY_CARD_CONTRACT_REF.to_owned(),
        affected_install_contract_ref: M5_ADVISORY_CLAIM_DOWNGRADE_AFFECTED_INSTALL_CONTRACT_REF
            .to_owned(),
        severity_matrix_ref: M5_ADVISORY_CLAIM_DOWNGRADE_SEVERITY_MATRIX_REF.to_owned(),
        mirror_offline_drill_ref: M5_ADVISORY_CLAIM_DOWNGRADE_MIRROR_OFFLINE_DRILL_REF.to_owned(),
        continuity_gate_ref: M5_ADVISORY_CLAIM_DOWNGRADE_CONTINUITY_GATE_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_dimensions,
        required_profiles,
        required_families,
        required_channels,
        required_claim_states,
        rows,
        covered_profiles,
        covered_claim_states,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        claim_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        claim_automation_refs: vec![
            "release_automation.auto_narrow.advisory_claim_downgrade_dashboard".to_owned(),
            "help_center.advisory_claim_downgrade_badge_registry".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.advisory_claim_downgrade_certification".to_owned(),
            M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-advisory-claim-downgrade-certification".to_owned()],
        published_report_ref: M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_ADVISORY_CLAIM_DOWNGRADE_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(AdvisoryClaimFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_advisory_claim_downgrade_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum AdvisoryClaimValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The declared required dimensions do not match the lane constants.
    RequiredDimensionsStale,
    /// The declared required profiles do not match the lane constants.
    RequiredProfilesStale,
    /// The declared required families do not match the lane constants.
    RequiredFamiliesStale,
    /// The declared required channels do not match the lane constants.
    RequiredChannelsStale,
    /// The declared required claim states do not match the lane constants.
    RequiredClaimStatesStale,
    /// The rows do not cover all three claimed deployment profiles.
    CoverageIncomplete,
    /// The declared covered profiles do not match the rows.
    CoverageStale,
    /// The declared covered claim states do not match the rows.
    CoveredClaimStatesStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared claim causes do not match the recomputed causes.
    ClaimCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the advisory-claim-downgrade certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every claimed deployment profile
/// carries a current certification row; each row's status is the derived auto-narrowed value, never
/// asserted; a green row cannot keep a claim while a stale notice, a lagging mirror, an unsigned
/// distribution, or a lost local continuity goes silent, or the profile fails to evaluate every
/// claimed advisory family or project into every claimed claim surface; a narrowed profile keeps a
/// distinct claim state rather than a generic degraded collapse; and a disclosed narrowing is
/// backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_advisory_claim_downgrade_certification_packet(
    packet: &AdvisoryClaimPacket,
) -> Result<(), Vec<AdvisoryClaimValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(AdvisoryClaimValidationError::NoRows);
    }
    if packet.record_kind != M5_ADVISORY_CLAIM_DOWNGRADE_PACKET_RECORD_KIND {
        errors.push(AdvisoryClaimValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_ADVISORY_CLAIM_DOWNGRADE_SCHEMA_VERSION {
        errors.push(AdvisoryClaimValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(AdvisoryClaimValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(AdvisoryClaimValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_DIMENSIONS
        .iter()
        .map(|dim| dim.as_str().to_owned())
        .collect();
    if packet.required_dimensions != expected_dimensions {
        errors.push(AdvisoryClaimValidationError::RequiredDimensionsStale);
    }
    let expected_profiles: Vec<String> = REQUIRED_PROFILES
        .iter()
        .map(|profile| profile.as_str().to_owned())
        .collect();
    if packet.required_profiles != expected_profiles {
        errors.push(AdvisoryClaimValidationError::RequiredProfilesStale);
    }
    let expected_families: Vec<String> = REQUIRED_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_families != expected_families {
        errors.push(AdvisoryClaimValidationError::RequiredFamiliesStale);
    }
    let expected_channels: Vec<String> = REQUIRED_CHANNELS
        .iter()
        .map(|channel| channel.as_str().to_owned())
        .collect();
    if packet.required_channels != expected_channels {
        errors.push(AdvisoryClaimValidationError::RequiredChannelsStale);
    }
    let expected_claim_states: Vec<String> = M5AdvisoryClaimState::ALL
        .iter()
        .map(|state| state.as_str().to_owned())
        .collect();
    if packet.required_claim_states != expected_claim_states {
        errors.push(AdvisoryClaimValidationError::RequiredClaimStatesStale);
    }

    let present: BTreeSet<M5AdvisoryClaimProfile> = packet.rows.iter().map(|row| row.profile).collect();
    let coverage_complete = REQUIRED_PROFILES.iter().all(|profile| present.contains(profile));
    if !coverage_complete || packet.rows.len() != REQUIRED_PROFILES.len() {
        errors.push(AdvisoryClaimValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|profile| profile.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_profiles {
        errors.push(AdvisoryClaimValidationError::CoverageStale);
    }

    let covered_states: Vec<String> = {
        let observed: BTreeSet<M5AdvisoryClaimState> = packet
            .rows
            .iter()
            .flat_map(|row| row.recompute_claim_states())
            .collect();
        M5AdvisoryClaimState::ALL
            .into_iter()
            .filter(|state| observed.contains(state))
            .map(|state| state.as_str().to_owned())
            .collect()
    };
    if covered_states != packet.covered_claim_states {
        errors.push(AdvisoryClaimValidationError::CoveredClaimStatesStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AdvisoryClaimStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AdvisoryClaimStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AdvisoryClaimStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(AdvisoryClaimValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<AdvisoryClaimWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(AdvisoryClaimValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<AdvisoryClaimCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.claim_causes {
        errors.push(AdvisoryClaimValidationError::ClaimCausesStale);
    }

    let mut recomputed: Vec<AdvisoryClaimFinding> = Vec::new();
    for profile in REQUIRED_PROFILES {
        if !present.contains(&profile) {
            recomputed.push(AdvisoryClaimFinding::ProfileMissing {
                profile: profile.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(AdvisoryClaimFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(AdvisoryClaimFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(AdvisoryClaimValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(AdvisoryClaimValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(AdvisoryClaimValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(AdvisoryClaimValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(AdvisoryClaimValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(AdvisoryClaimValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
