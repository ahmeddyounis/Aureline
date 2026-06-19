//! Mirror-only and air-gapped continuity packets with no-public-fallback rules.
//!
//! This module makes mirror-only, air-gapped, and self-hosted-restricted
//! continuity a first-class product truth instead of assuming a public-network
//! rescue path is always available. Every claimed offline-leaning surface that
//! carries mirror, sovereign, or air-gapped continuity language must point to one
//! typed [`MirrorAirgapPacketEntry`] that answers the same questions everywhere:
//!
//! 1. What trust-root continuity backs the boundary — which trust-root posture
//!    anchors it, and can that trust survive and renew offline without a public
//!    reissue?
//! 2. How fresh is the mirror or offline bundle the boundary depends on, when was
//!    it last synced, and when does that freshness age out?
//! 3. What offline import and export paths move content across the boundary
//!    without touching the public network?
//! 4. Where do security advisories and revocation data come from — a signed
//!    offline bundle, the approved mirror, or (forbidden on an isolated row) a
//!    live public fetch?
//! 5. Is public fallback **prohibited**, **unavailable**, or **policy-gated** —
//!    rather than silently attempted?
//!
//! The descriptor is projected identically onto every claimed surface
//! (release-center, shiproom, support-center, partner qualification, and public
//! claim-manifest generation) through a
//! [`MirrorAirgapSurfaceProjection`], so the exact trust-root, offline-exchange,
//! advisory, and public-fallback vocabulary stays byte-identical everywhere
//! instead of drifting per surface. The trust-root vocabulary is the same
//! [`TrustRootPostureClass`] the key-mode and storage-posture surface uses, not a
//! separate mirror-only dialect.
//!
//! Three guardrails are load-bearing and fail closed:
//!
//! - A mirror-only or air-gapped row may not **silently** fall back to public
//!   endpoints: a packet whose public-fallback policy is `silent_public_fallback`
//!   has its claim withdrawn.
//! - Advisory or revocation language may not imply a **live public fetch** on a
//!   mirror-only or air-gapped row: such a packet is withdrawn.
//! - On an isolated (mirror-only or air-gapped) row, a trust root that cannot be
//!   renewed without a public reissue **breaks offline continuity** and is
//!   withdrawn.
//!
//! The [`OfflineContinuityRegistry`] is the typed consumer the release-center,
//! shiproom, support-center, partner-qualification, and public claim-manifest
//! surfaces read. It indexes packets by claim row and reports, per claimed
//! offline row, whether a current packet backs the claim — so any affected
//! sovereign or restricted claim row narrows automatically when trust-root
//! continuity, mirror freshness, or advisory/offline-fallback evidence is
//! missing, stale, or profile-mismatched.
//!
//! The packet is metadata-only. It carries closed-vocabulary tokens, export-safe
//! plain-language labels, UTC timestamps, and opaque refs. Raw mirror bytes, raw
//! provider payloads, raw endpoint hostnames, raw trust-root key material, and
//! secret bodies never cross this boundary.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::m5_key_mode_and_storage_posture::TrustRootPostureClass;
use crate::m5_locality_tenant_keymode_and_drill_matrix::{
    ContinuityClaimQualificationClass, ContinuityProfileClass,
};

#[cfg(test)]
mod tests;

/// Schema version carried on every record in this module.
pub const MIRROR_AIRGAP_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const MIRROR_AIRGAP_SHARED_CONTRACT_REF: &str =
    "continuity:m5_mirror_airgap_continuity_packets:v1";

/// Record-kind tag for [`MirrorAirgapPage`] payloads.
pub const MIRROR_AIRGAP_PAGE_RECORD_KIND: &str = "mirror_airgap_continuity_page_record";

/// Record-kind tag for [`MirrorAirgapSummary`] payloads.
pub const MIRROR_AIRGAP_SUMMARY_RECORD_KIND: &str = "mirror_airgap_continuity_summary_record";

/// Record-kind tag for [`MirrorAirgapDescriptor`] payloads.
pub const MIRROR_AIRGAP_DESCRIPTOR_RECORD_KIND: &str = "mirror_airgap_continuity_descriptor_record";

/// Record-kind tag for [`MirrorAirgapSurfaceProjection`] payloads.
pub const MIRROR_AIRGAP_SURFACE_PROJECTION_RECORD_KIND: &str =
    "mirror_airgap_continuity_surface_projection_record";

/// Record-kind tag for [`MirrorAirgapOutcome`] payloads.
pub const MIRROR_AIRGAP_OUTCOME_RECORD_KIND: &str = "mirror_airgap_continuity_outcome_record";

/// Record-kind tag for [`MirrorAirgapDefect`] payloads.
pub const MIRROR_AIRGAP_DEFECT_RECORD_KIND: &str = "mirror_airgap_continuity_defect_record";

/// Record-kind tag for [`OfflineContinuityRegistry`] payloads.
pub const OFFLINE_CONTINUITY_REGISTRY_RECORD_KIND: &str = "offline_continuity_registry_record";

/// Record-kind tag for [`OfflineCoverageRow`] payloads.
pub const OFFLINE_COVERAGE_ROW_RECORD_KIND: &str = "offline_coverage_row_record";

/// Record-kind tag for [`MirrorAirgapSupportExport`] payloads.
pub const MIRROR_AIRGAP_SUPPORT_EXPORT_RECORD_KIND: &str =
    "mirror_airgap_continuity_support_export_record";

/// Repo-relative path of the canonical reviewer doc for this lane.
pub const MIRROR_AIRGAP_DOC_REF: &str = "docs/m5/continuity/mirror-airgap-continuity.md";

/// Repo-relative path of the checked-in artifact for this lane.
pub const MIRROR_AIRGAP_ARTIFACT_REF: &str =
    "artifacts/m5/continuity/mirror_airgap_continuity_packets.md";

/// Repo-relative path of the canonical JSON schema for this lane.
pub const MIRROR_AIRGAP_SCHEMA_REF: &str = "schemas/continuity/mirror_airgap_packet.schema.json";

/// Connectivity posture of a claimed continuity row.
///
/// The point of an explicit posture is to separate truly isolated boundaries
/// (mirror-only, air-gapped) — which may never imply a public fetch — from a
/// self-hosted boundary with controlled egress and from a pure local desktop
/// surface that needs no offline-continuity evidence at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectivityPostureClass {
    /// Content is served from an approved mirror with no public-network path.
    MirrorOnly,
    /// A fully isolated, air-gapped boundary reached only by offline exchange.
    AirGapped,
    /// A self-hosted boundary with restricted, controlled egress.
    SelfHostedRestricted,
    /// A pure local desktop surface with no claimed managed or mirror lane.
    LocalOnly,
}

impl ConnectivityPostureClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorOnly => "mirror_only",
            Self::AirGapped => "air_gapped",
            Self::SelfHostedRestricted => "self_hosted_restricted",
            Self::LocalOnly => "local_only",
        }
    }

    /// Plain-language label naming the posture.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::MirrorOnly => "mirror-only",
            Self::AirGapped => "air-gapped",
            Self::SelfHostedRestricted => "self-hosted (restricted egress)",
            Self::LocalOnly => "local-only",
        }
    }

    /// True when this posture must carry typed offline-continuity evidence.
    ///
    /// A pure local-only surface is exempt; every other posture claims an
    /// offline-leaning boundary and must explain its trust-root, mirror,
    /// offline-exchange, advisory, and public-fallback posture.
    pub const fn requires_offline_continuity_evidence(self) -> bool {
        !matches!(self, Self::LocalOnly)
    }

    /// True when this posture forbids any live public-network fetch.
    ///
    /// Mirror-only and air-gapped boundaries are isolated: a public fetch (for
    /// advisories, revocation, or fallback) would invalidate the claimed profile.
    pub const fn forbids_public_fetch(self) -> bool {
        matches!(self, Self::MirrorOnly | Self::AirGapped)
    }

    /// True when this posture depends on a live, syncing mirror.
    ///
    /// Air-gapped boundaries depend on signed offline bundles rather than a live
    /// mirror, so a not-applicable mirror is acceptable there but not on a
    /// mirror-only or self-hosted-restricted row.
    pub const fn requires_live_mirror(self) -> bool {
        matches!(self, Self::MirrorOnly | Self::SelfHostedRestricted)
    }
}

/// How a trust root is renewed or rotated across the boundary.
///
/// This binds trust-root *continuity* — not just posture — to the same
/// trust-store vocabulary, answering whether trust can be re-established without
/// reaching the public network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustRootRenewalClass {
    /// Renewed by importing a signed offline rotation bundle.
    OfflineSignedRotation,
    /// Renewed through the approved mirror.
    MirrorReplicatedRotation,
    /// Renewed by the customer in their own KMS or HSM.
    CustomerOperatedRotation,
    /// Renewal requires a live public reissue; it cannot complete offline.
    PublicReissueRequired,
    /// Renewal posture is not disclosed; the claim must narrow.
    Undisclosed,
    /// Trust-root renewal does not apply to this row.
    NotApplicable,
}

impl TrustRootRenewalClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfflineSignedRotation => "offline_signed_rotation",
            Self::MirrorReplicatedRotation => "mirror_replicated_rotation",
            Self::CustomerOperatedRotation => "customer_operated_rotation",
            Self::PublicReissueRequired => "public_reissue_required",
            Self::Undisclosed => "undisclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the renewal path.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::OfflineSignedRotation => "renewed by signed offline rotation",
            Self::MirrorReplicatedRotation => "renewed through the approved mirror",
            Self::CustomerOperatedRotation => "renewed by the customer in their own KMS or HSM",
            Self::PublicReissueRequired => "requires a live public reissue",
            Self::Undisclosed => "not disclosed",
            Self::NotApplicable => "not applicable",
        }
    }

    /// True when the renewal posture has been disclosed.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::Undisclosed | Self::NotApplicable)
    }

    /// True when renewal cannot complete without a public reissue.
    pub const fn requires_public_reissue(self) -> bool {
        matches!(self, Self::PublicReissueRequired)
    }
}

/// Freshness of the mirror content or offline bundle a boundary depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorFreshnessStateClass {
    /// The mirror or bundle is fresh within its sync window.
    FreshWithinWindow,
    /// Stale, but within an explicit grace window.
    StaleWithinGrace,
    /// Stale enough that a fresh sync or bundle import is required.
    StaleNeedsSync,
    /// The mirror has never been synced.
    NeverSynced,
    /// No live mirror applies (air-gapped or local-only).
    NotApplicable,
}

impl MirrorFreshnessStateClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshWithinWindow => "fresh_within_window",
            Self::StaleWithinGrace => "stale_within_grace",
            Self::StaleNeedsSync => "stale_needs_sync",
            Self::NeverSynced => "never_synced",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the freshness state.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::FreshWithinWindow => "fresh within the sync window",
            Self::StaleWithinGrace => "stale within grace",
            Self::StaleNeedsSync => "stale, needs a fresh sync",
            Self::NeverSynced => "never synced",
            Self::NotApplicable => "not applicable",
        }
    }

    /// True when the freshness is acceptable and need not narrow.
    pub const fn is_acceptable(self) -> bool {
        matches!(self, Self::FreshWithinWindow | Self::StaleWithinGrace)
    }

    /// True when a fresh sync or bundle import is required before claiming.
    pub const fn needs_sync(self) -> bool {
        matches!(self, Self::StaleNeedsSync | Self::NeverSynced)
    }

    /// True when this state must record a last-synced and expiry timestamp.
    pub const fn requires_sync_window(self) -> bool {
        matches!(self, Self::FreshWithinWindow | Self::StaleWithinGrace)
    }
}

/// How content crosses the boundary without the public network.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineExchangeClass {
    /// Exchanged as a signed offline bundle.
    SignedOfflineBundle,
    /// Exchanged by physical media transfer (sneakernet).
    PhysicalMediaTransfer,
    /// Exchanged by pulling from or pushing to the approved mirror.
    MirrorPullPush,
    /// No offline path exists for this direction (an honest, disclosed answer).
    NoOfflinePath,
    /// The path is not disclosed; the claim must narrow.
    Undisclosed,
    /// Offline exchange does not apply to this row.
    NotApplicable,
}

impl OfflineExchangeClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedOfflineBundle => "signed_offline_bundle",
            Self::PhysicalMediaTransfer => "physical_media_transfer",
            Self::MirrorPullPush => "mirror_pull_push",
            Self::NoOfflinePath => "no_offline_path",
            Self::Undisclosed => "undisclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the exchange path.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::SignedOfflineBundle => "signed offline bundle",
            Self::PhysicalMediaTransfer => "physical media transfer",
            Self::MirrorPullPush => "mirror pull/push",
            Self::NoOfflinePath => "no offline path",
            Self::Undisclosed => "not disclosed",
            Self::NotApplicable => "not applicable",
        }
    }

    /// True when the path has been disclosed (`no_offline_path` is a disclosure).
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::Undisclosed | Self::NotApplicable)
    }
}

/// Where security advisory and revocation data come from for the boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryRevocationSourceClass {
    /// Delivered as a signed offline advisory and revocation bundle.
    OfflineBundle,
    /// Replicated through the approved mirror.
    MirrorReplicated,
    /// Served only from the local cache; staleness must be disclosed.
    LocalCacheOnly,
    /// Fetched live from public endpoints (forbidden on an isolated row).
    LivePublicFetch,
    /// The source is not disclosed; the claim must narrow.
    Undisclosed,
    /// Advisory and revocation handling does not apply to this row.
    NotApplicable,
}

impl AdvisoryRevocationSourceClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfflineBundle => "offline_bundle",
            Self::MirrorReplicated => "mirror_replicated",
            Self::LocalCacheOnly => "local_cache_only",
            Self::LivePublicFetch => "live_public_fetch",
            Self::Undisclosed => "undisclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the advisory and revocation source.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::OfflineBundle => "signed offline advisory and revocation bundle",
            Self::MirrorReplicated => "replicated through the approved mirror",
            Self::LocalCacheOnly => "local cache only",
            Self::LivePublicFetch => "live public fetch",
            Self::Undisclosed => "not disclosed",
            Self::NotApplicable => "not applicable",
        }
    }

    /// True when the source has been disclosed.
    pub const fn is_disclosed(self) -> bool {
        !matches!(self, Self::Undisclosed | Self::NotApplicable)
    }

    /// True when the source implies a live public-network fetch.
    pub const fn implies_public_fetch(self) -> bool {
        matches!(self, Self::LivePublicFetch)
    }
}

/// Whether and how public fallback is permitted for the boundary.
///
/// This is the central disclosure for a mirror-only or air-gapped row: the
/// product must state public fallback as **prohibited**, **unavailable**, or
/// **policy-gated** rather than silently attempting it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicFallbackPolicyClass {
    /// Public fallback is prohibited by policy and never attempted.
    Prohibited,
    /// No public endpoint is reachable; fallback is physically unavailable.
    Unavailable,
    /// A public fallback exists but requires an explicit, logged policy change.
    PolicyGated,
    /// The boundary silently falls back to public endpoints (fails closed).
    SilentPublicFallback,
    /// The public-fallback policy is not disclosed; the claim must narrow.
    Undisclosed,
    /// Public fallback does not apply to this row.
    NotApplicable,
}

impl PublicFallbackPolicyClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prohibited => "prohibited",
            Self::Unavailable => "unavailable",
            Self::PolicyGated => "policy_gated",
            Self::SilentPublicFallback => "silent_public_fallback",
            Self::Undisclosed => "undisclosed",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Plain-language summary of the public-fallback policy.
    pub const fn plain(self) -> &'static str {
        match self {
            Self::Prohibited => "prohibited",
            Self::Unavailable => "unavailable",
            Self::PolicyGated => "requires an explicit policy change",
            Self::SilentPublicFallback => "silently attempted (not allowed)",
            Self::Undisclosed => "not disclosed",
            Self::NotApplicable => "not applicable",
        }
    }

    /// True when the policy is one of the three explicitly governed states.
    pub const fn is_explicitly_governed(self) -> bool {
        matches!(
            self,
            Self::Prohibited | Self::Unavailable | Self::PolicyGated
        )
    }

    /// True when the boundary silently falls back to the public network.
    pub const fn is_silent_public_fallback(self) -> bool {
        matches!(self, Self::SilentPublicFallback)
    }
}

/// Surface a mirror/air-gap descriptor is projected onto.
///
/// These are exactly the surfaces that reuse the packet family: the release
/// center, shiproom readiness dashboard, support center, partner qualification,
/// and public claim-manifest generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineSurfaceClass {
    /// The release-center readiness surface.
    ReleaseCenter,
    /// The shiproom readiness dashboard.
    Shiproom,
    /// The support-center export surface.
    SupportCenter,
    /// Partner qualification packets.
    PartnerQualification,
    /// Public claim-manifest generation.
    PublicClaimManifest,
}

impl OfflineSurfaceClass {
    /// Every surface in canonical projection order.
    pub const ALL: [OfflineSurfaceClass; 5] = [
        Self::ReleaseCenter,
        Self::Shiproom,
        Self::SupportCenter,
        Self::PartnerQualification,
        Self::PublicClaimManifest,
    ];

    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseCenter => "release_center",
            Self::Shiproom => "shiproom",
            Self::SupportCenter => "support_center",
            Self::PartnerQualification => "partner_qualification",
            Self::PublicClaimManifest => "public_claim_manifest",
        }
    }
}

/// Typed reason a mirror/air-gap continuity claim narrowed below stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorAirgapNarrowReasonClass {
    /// No narrowing is active.
    NotNarrowed,
    /// The boundary silently falls back to public endpoints.
    SilentPublicFallback,
    /// Advisory or revocation data implies a live public fetch on an isolated row.
    AdvisoryImpliesLivePublicFetch,
    /// An isolated row's trust root cannot be renewed without a public reissue.
    TrustRootBreaksOffline,
    /// The mirror has never been synced.
    MirrorNeverSynced,
    /// The mirror or bundle is stale (or its freshness window is undeclared).
    MirrorFreshnessStale,
    /// Trust-root continuity is not declared on an offline row.
    TrustRootContinuityUndeclared,
    /// An offline import or export path is not disclosed.
    OfflineExchangeUndisclosed,
    /// The advisory and revocation source is not disclosed.
    AdvisoryRevocationUndisclosed,
    /// The public-fallback policy is not disclosed.
    PublicFallbackUndisclosed,
    /// The claimed profile is inconsistent with its connectivity posture.
    ProfilePostureMismatch,
    /// A surface renders different trust-root, advisory, or fallback vocabulary than the descriptor.
    PacketVocabularyDrift,
    /// A packet is not projected onto every required surface.
    SurfaceReuseIncomplete,
    /// A claimed offline row has no mirror/air-gap continuity packet at all.
    PacketEvidenceMissing,
}

impl MirrorAirgapNarrowReasonClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::SilentPublicFallback => "silent_public_fallback",
            Self::AdvisoryImpliesLivePublicFetch => "advisory_implies_live_public_fetch",
            Self::TrustRootBreaksOffline => "trust_root_breaks_offline",
            Self::MirrorNeverSynced => "mirror_never_synced",
            Self::MirrorFreshnessStale => "mirror_freshness_stale",
            Self::TrustRootContinuityUndeclared => "trust_root_continuity_undeclared",
            Self::OfflineExchangeUndisclosed => "offline_exchange_undisclosed",
            Self::AdvisoryRevocationUndisclosed => "advisory_revocation_undisclosed",
            Self::PublicFallbackUndisclosed => "public_fallback_undisclosed",
            Self::ProfilePostureMismatch => "profile_posture_mismatch",
            Self::PacketVocabularyDrift => "packet_vocabulary_drift",
            Self::SurfaceReuseIncomplete => "surface_reuse_incomplete",
            Self::PacketEvidenceMissing => "packet_evidence_missing",
        }
    }

    /// True when this reason withdraws the claim immediately (fails closed).
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(
            self,
            Self::SilentPublicFallback
                | Self::AdvisoryImpliesLivePublicFetch
                | Self::TrustRootBreaksOffline
        )
    }

    /// True when this reason holds the claim at preview.
    pub const fn is_preview_reason(self) -> bool {
        matches!(
            self,
            Self::MirrorNeverSynced
                | Self::TrustRootContinuityUndeclared
                | Self::OfflineExchangeUndisclosed
                | Self::AdvisoryRevocationUndisclosed
                | Self::PublicFallbackUndisclosed
                | Self::ProfilePostureMismatch
                | Self::PacketVocabularyDrift
                | Self::PacketEvidenceMissing
        )
    }
}

/// Coverage state of a claimed offline row, derived from its packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OfflineCoverageClass {
    /// A current packet backs the claim.
    CurrentPacket,
    /// A packet backs the claim but its evidence is stale and must be refreshed.
    StalePacketNeedsRefresh,
    /// A packet backs the claim but its claim is withheld (fails closed).
    PacketWithheld,
    /// No mirror/air-gap continuity packet backs the claim at all.
    NoPacket,
}

impl OfflineCoverageClass {
    /// Stable token recorded on serialized records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentPacket => "current_packet",
            Self::StalePacketNeedsRefresh => "stale_packet_needs_refresh",
            Self::PacketWithheld => "packet_withheld",
            Self::NoPacket => "no_packet",
        }
    }

    /// True when the claim is backed by a current packet.
    pub const fn is_covered(self) -> bool {
        matches!(self, Self::CurrentPacket)
    }
}

/// Derives a qualification from the mirror/air-gap narrow reasons present.
fn qualification_from_reasons<'a>(
    reasons: impl IntoIterator<Item = &'a MirrorAirgapNarrowReasonClass>,
) -> ContinuityClaimQualificationClass {
    let mut saw_any = false;
    let mut saw_preview = false;
    for reason in reasons {
        saw_any = true;
        if reason.is_withdrawal_reason() {
            return ContinuityClaimQualificationClass::Withdrawn;
        }
        if reason.is_preview_reason() {
            saw_preview = true;
        }
    }
    if saw_preview {
        ContinuityClaimQualificationClass::Preview
    } else if saw_any {
        ContinuityClaimQualificationClass::Beta
    } else {
        ContinuityClaimQualificationClass::Stable
    }
}

/// Trust-root continuity for a boundary, bound to the trust-store vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustRootContinuity {
    /// Trust-root posture, reusing the key-mode and storage-posture vocabulary.
    pub posture: TrustRootPostureClass,
    /// Stable token for [`Self::posture`].
    pub posture_token: String,
    /// How the trust root is renewed across the boundary.
    pub renewal: TrustRootRenewalClass,
    /// Stable token for [`Self::renewal`].
    pub renewal_token: String,
    /// Export-safe note describing how trust survives the boundary.
    pub continuity_note: String,
}

impl TrustRootContinuity {
    /// Builds a trust-root continuity record, computing its tokens.
    pub fn new(
        posture: TrustRootPostureClass,
        renewal: TrustRootRenewalClass,
        continuity_note: impl Into<String>,
    ) -> Self {
        Self {
            posture,
            posture_token: posture.as_str().to_owned(),
            renewal,
            renewal_token: renewal.as_str().to_owned(),
            continuity_note: continuity_note.into(),
        }
    }

    /// True when both the posture and the renewal path are declared.
    pub fn is_declared(&self) -> bool {
        self.posture.is_declared() && self.renewal.is_disclosed()
    }

    /// True when the trust root survives and renews without a public reissue.
    pub fn survives_offline(&self) -> bool {
        self.posture.is_declared() && !self.renewal.requires_public_reissue()
    }
}

/// Mirror or offline-bundle freshness for a boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorFreshness {
    /// Freshness state of the mirror or offline bundle.
    pub state: MirrorFreshnessStateClass,
    /// Stable token for [`Self::state`].
    pub state_token: String,
    /// UTC timestamp of the last successful sync, empty when never synced.
    pub last_synced_at: String,
    /// UTC timestamp when the freshness ages out, empty when not applicable.
    pub freshness_expires_at: String,
    /// Opaque ref to the mirror or bundle; never a raw mirror body or hostname.
    pub mirror_ref: String,
}

impl MirrorFreshness {
    /// Builds a mirror-freshness record, computing its token.
    pub fn new(
        state: MirrorFreshnessStateClass,
        last_synced_at: impl Into<String>,
        freshness_expires_at: impl Into<String>,
        mirror_ref: impl Into<String>,
    ) -> Self {
        Self {
            state,
            state_token: state.as_str().to_owned(),
            last_synced_at: last_synced_at.into(),
            freshness_expires_at: freshness_expires_at.into(),
            mirror_ref: mirror_ref.into(),
        }
    }

    /// True when fresh/graced freshness is missing its required timestamps.
    pub fn missing_sync_window(&self) -> bool {
        self.state.requires_sync_window()
            && (self.last_synced_at.trim().is_empty()
                || self.freshness_expires_at.trim().is_empty())
    }
}

/// One claimed mirror/air-gap continuity packet decorated with its facts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapPacketEntry {
    /// Opaque packet identifier.
    pub packet_id: String,
    /// Opaque id of the continuity-claim row this packet backs.
    pub claim_row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Claimed deployment profile.
    pub profile_class: ContinuityProfileClass,
    /// Stable token for [`Self::profile_class`].
    pub profile_class_token: String,
    /// Connectivity posture of the boundary.
    pub connectivity_posture: ConnectivityPostureClass,
    /// Stable token for [`Self::connectivity_posture`].
    pub connectivity_posture_token: String,
    /// True when the claim row this packet backs carries offline-continuity language.
    pub backs_offline_claim: bool,
    /// Trust-root continuity backing the boundary.
    pub trust_root: TrustRootContinuity,
    /// Mirror or offline-bundle freshness.
    pub mirror_freshness: MirrorFreshness,
    /// How content is imported across the boundary offline.
    pub offline_import: OfflineExchangeClass,
    /// Stable token for [`Self::offline_import`].
    pub offline_import_token: String,
    /// How content is exported across the boundary offline.
    pub offline_export: OfflineExchangeClass,
    /// Stable token for [`Self::offline_export`].
    pub offline_export_token: String,
    /// Export-safe note describing the offline import and export paths.
    pub offline_exchange_note: String,
    /// Where advisory and revocation data come from.
    pub advisory_revocation_source: AdvisoryRevocationSourceClass,
    /// Stable token for [`Self::advisory_revocation_source`].
    pub advisory_revocation_source_token: String,
    /// Export-safe note describing advisory and revocation handling.
    pub advisory_revocation_note: String,
    /// Whether and how public fallback is permitted.
    pub public_fallback_policy: PublicFallbackPolicyClass,
    /// Stable token for [`Self::public_fallback_policy`].
    pub public_fallback_policy_token: String,
    /// Export-safe note describing the public-fallback boundary.
    pub public_fallback_note: String,
    /// Surfaces this packet is projected onto.
    pub projected_surfaces: Vec<OfflineSurfaceClass>,
}

impl MirrorAirgapPacketEntry {
    /// Builds a mirror/air-gap packet entry.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        packet_id: impl Into<String>,
        claim_row_id: impl Into<String>,
        surface_label: impl Into<String>,
        profile_class: ContinuityProfileClass,
        connectivity_posture: ConnectivityPostureClass,
        backs_offline_claim: bool,
        trust_root: TrustRootContinuity,
        mirror_freshness: MirrorFreshness,
        offline_import: OfflineExchangeClass,
        offline_export: OfflineExchangeClass,
        offline_exchange_note: impl Into<String>,
        advisory_revocation_source: AdvisoryRevocationSourceClass,
        advisory_revocation_note: impl Into<String>,
        public_fallback_policy: PublicFallbackPolicyClass,
        public_fallback_note: impl Into<String>,
        projected_surfaces: Vec<OfflineSurfaceClass>,
    ) -> Self {
        Self {
            packet_id: packet_id.into(),
            claim_row_id: claim_row_id.into(),
            surface_label: surface_label.into(),
            profile_class,
            profile_class_token: profile_class.as_str().to_owned(),
            connectivity_posture,
            connectivity_posture_token: connectivity_posture.as_str().to_owned(),
            backs_offline_claim,
            trust_root,
            mirror_freshness,
            offline_import,
            offline_import_token: offline_import.as_str().to_owned(),
            offline_export,
            offline_export_token: offline_export.as_str().to_owned(),
            offline_exchange_note: offline_exchange_note.into(),
            advisory_revocation_source,
            advisory_revocation_source_token: advisory_revocation_source.as_str().to_owned(),
            advisory_revocation_note: advisory_revocation_note.into(),
            public_fallback_policy,
            public_fallback_policy_token: public_fallback_policy.as_str().to_owned(),
            public_fallback_note: public_fallback_note.into(),
            projected_surfaces,
        }
    }

    /// Surfaces this packet is required to reach (every surface).
    pub fn required_surfaces(&self) -> &'static [OfflineSurfaceClass] {
        &OfflineSurfaceClass::ALL
    }

    /// True when this packet must carry typed offline-continuity evidence.
    pub fn requires_offline_continuity_evidence(&self) -> bool {
        self.connectivity_posture
            .requires_offline_continuity_evidence()
    }

    /// True when this packet's posture forbids any live public-network fetch.
    pub fn forbids_public_fetch(&self) -> bool {
        self.connectivity_posture.forbids_public_fetch()
    }

    /// Returns a profile-vs-posture mismatch note when one applies.
    pub fn profile_posture_mismatch(&self) -> Option<&'static str> {
        let profile_local = self.profile_class == ContinuityProfileClass::LocalOnly;
        let posture_local = self.connectivity_posture == ConnectivityPostureClass::LocalOnly;
        if profile_local != posture_local {
            Some("a local-only profile and a local-only posture must agree")
        } else if self.connectivity_posture == ConnectivityPostureClass::AirGapped
            && self.profile_class == ContinuityProfileClass::Managed
        {
            Some("a managed profile cannot claim an air-gapped posture")
        } else {
            None
        }
    }
}

/// Plain-language descriptor for one mirror/air-gap continuity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapDescriptor {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque descriptor identifier.
    pub descriptor_id: String,
    /// Packet this descriptor describes.
    pub packet_id: String,
    /// Claim row this packet backs.
    pub claim_row_id: String,
    /// Reviewable label naming the claimed surface.
    pub surface_label: String,
    /// Stable token for the claimed profile.
    pub profile_class_token: String,
    /// Plain-language claimed profile.
    pub profile_class_plain: String,
    /// Stable token for the connectivity posture.
    pub connectivity_posture_token: String,
    /// Plain-language connectivity posture.
    pub connectivity_posture_plain: String,
    /// Stable token for the trust-root posture.
    pub trust_root_posture_token: String,
    /// Stable token for the trust-root renewal path.
    pub trust_root_renewal_token: String,
    /// True when the trust root survives and renews offline.
    pub trust_root_survives_offline: bool,
    /// Stable token for the mirror freshness state.
    pub mirror_freshness_token: String,
    /// True when the mirror or bundle freshness is acceptable.
    pub mirror_fresh: bool,
    /// Stable token for the offline import path.
    pub offline_import_token: String,
    /// Stable token for the offline export path.
    pub offline_export_token: String,
    /// Stable token for the advisory and revocation source.
    pub advisory_revocation_source_token: String,
    /// Stable token for the public-fallback policy.
    pub public_fallback_policy_token: String,
    /// True when public fallback is explicitly governed (prohibited, unavailable, or policy-gated).
    pub public_fallback_governed: bool,
    /// Canonical one-line trust-root summary reused by every surface projection.
    pub trust_root_line: String,
    /// Canonical one-line mirror-freshness summary reused by every surface projection.
    pub mirror_freshness_line: String,
    /// Canonical one-line offline-exchange summary reused by every surface projection.
    pub offline_exchange_line: String,
    /// Canonical one-line advisory/revocation summary reused by every surface projection.
    pub advisory_line: String,
    /// Canonical one-line public-fallback summary reused by every surface projection.
    pub public_fallback_line: String,
}

impl MirrorAirgapDescriptor {
    /// Builds a descriptor from a decorated packet entry.
    pub fn from_entry(entry: &MirrorAirgapPacketEntry) -> Self {
        Self {
            record_kind: MIRROR_AIRGAP_DESCRIPTOR_RECORD_KIND.to_owned(),
            schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
            descriptor_id: format!("continuity:mirror-airgap-descriptor:{}", entry.packet_id),
            packet_id: entry.packet_id.clone(),
            claim_row_id: entry.claim_row_id.clone(),
            surface_label: entry.surface_label.clone(),
            profile_class_token: entry.profile_class_token.clone(),
            profile_class_plain: profile_plain(entry.profile_class).to_owned(),
            connectivity_posture_token: entry.connectivity_posture_token.clone(),
            connectivity_posture_plain: entry.connectivity_posture.plain().to_owned(),
            trust_root_posture_token: entry.trust_root.posture_token.clone(),
            trust_root_renewal_token: entry.trust_root.renewal_token.clone(),
            trust_root_survives_offline: entry.trust_root.survives_offline(),
            mirror_freshness_token: entry.mirror_freshness.state_token.clone(),
            mirror_fresh: entry.mirror_freshness.state.is_acceptable(),
            offline_import_token: entry.offline_import_token.clone(),
            offline_export_token: entry.offline_export_token.clone(),
            advisory_revocation_source_token: entry.advisory_revocation_source_token.clone(),
            public_fallback_policy_token: entry.public_fallback_policy_token.clone(),
            public_fallback_governed: entry.public_fallback_policy.is_explicitly_governed(),
            trust_root_line: trust_root_line(entry),
            mirror_freshness_line: mirror_freshness_line(entry),
            offline_exchange_line: offline_exchange_line(entry),
            advisory_line: advisory_line(entry),
            public_fallback_line: public_fallback_line(entry),
        }
    }
}

/// One surface rendering of a mirror/air-gap descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapSurfaceProjection {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Surface this projection renders on.
    pub surface: OfflineSurfaceClass,
    /// Stable token for [`Self::surface`].
    pub surface_token: String,
    /// Packet this projection describes.
    pub packet_id: String,
    /// Descriptor id rendered on this surface.
    pub descriptor_id: String,
    /// Trust-root summary line rendered on this surface.
    pub trust_root_line: String,
    /// Mirror-freshness summary line rendered on this surface.
    pub mirror_freshness_line: String,
    /// Offline-exchange summary line rendered on this surface.
    pub offline_exchange_line: String,
    /// Advisory/revocation summary line rendered on this surface.
    pub advisory_line: String,
    /// Public-fallback summary line rendered on this surface.
    pub public_fallback_line: String,
}

/// Per-packet verdict joining a packet to its computed qualification and reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapOutcome {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Packet this outcome describes.
    pub packet_id: String,
    /// Claim row this packet backs.
    pub claim_row_id: String,
    /// Stable token for the connectivity posture.
    pub connectivity_posture_token: String,
    /// Computed qualification token for the packet.
    pub qualification_token: String,
    /// True when the packet narrowed below stable.
    pub narrowed: bool,
    /// True when the packet's claim is withheld entirely.
    pub claim_withheld: bool,
    /// Stable token for the trust-root posture.
    pub trust_root_posture_token: String,
    /// True when the trust root survives and renews offline.
    pub trust_root_survives_offline: bool,
    /// Stable token for the mirror freshness state.
    pub mirror_freshness_token: String,
    /// True when the mirror or bundle freshness is acceptable.
    pub mirror_fresh: bool,
    /// Stable token for the advisory and revocation source.
    pub advisory_revocation_source_token: String,
    /// Stable token for the public-fallback policy.
    pub public_fallback_policy_token: String,
    /// True when public fallback is explicitly governed.
    pub public_fallback_governed: bool,
    /// Stable narrow-reason tokens that applied to the packet.
    pub narrow_reason_tokens: Vec<String>,
}

/// One claimed offline row's coverage verdict, derived from its packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineCoverageRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Claim row this coverage row describes.
    pub claim_row_id: String,
    /// Coverage class derived from the backing packet.
    pub coverage_class: OfflineCoverageClass,
    /// Stable token for [`Self::coverage_class`].
    pub coverage_class_token: String,
    /// Packet id backing the claim, empty when none.
    pub packet_id: String,
    /// Computed qualification token for the coverage.
    pub qualification_token: String,
    /// True when a current packet backs the claim.
    pub covered: bool,
    /// True when the coverage narrowed below stable.
    pub narrowed: bool,
}

/// Typed consumer that indexes mirror/air-gap packets by claim row.
///
/// The release-center, shiproom, support-center, partner-qualification, and
/// public claim-manifest surfaces read this registry instead of re-deriving
/// offline-continuity coverage by hand. It reports, per claimed offline row,
/// whether a current packet backs the claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineContinuityRegistry {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable registry identifier.
    pub registry_id: String,
    /// Per-claim-row coverage rows.
    pub coverage: Vec<OfflineCoverageRow>,
    /// Claim row ids that point to a current packet.
    pub covered_claim_row_ids: Vec<String>,
    /// Claim row ids that narrowed because their packet is stale, withheld, or missing.
    pub uncovered_claim_row_ids: Vec<String>,
}

impl OfflineContinuityRegistry {
    /// Builds a registry from a finished page's outcomes and expected rows.
    pub fn from_page(page: &MirrorAirgapPage) -> Self {
        build_registry(&page.input, &page.outcomes)
    }

    /// Returns the coverage row for a claim row id, if present.
    pub fn coverage_for_claim_row(&self, claim_row_id: &str) -> Option<&OfflineCoverageRow> {
        self.coverage
            .iter()
            .find(|row| row.claim_row_id == claim_row_id)
    }

    /// True when a current packet backs the claim row.
    pub fn is_claim_row_covered(&self, claim_row_id: &str) -> bool {
        self.coverage_for_claim_row(claim_row_id)
            .is_some_and(|row| row.covered)
    }

    /// Number of claim rows backed by a current packet.
    pub fn covered_claim_count(&self) -> usize {
        self.covered_claim_row_ids.len()
    }

    /// True when every tracked claim row points to a current packet.
    pub fn all_claims_covered(&self) -> bool {
        self.uncovered_claim_row_ids.is_empty()
    }
}

/// Typed defect emitted by the mirror/air-gap audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque defect identifier.
    pub defect_id: String,
    /// Typed narrow reason.
    pub narrow_reason: MirrorAirgapNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Opaque source packet id or claim row that triggered the defect.
    pub source: String,
    /// Export-safe explanation of the defect.
    pub note: String,
}

impl MirrorAirgapDefect {
    fn new(
        narrow_reason: MirrorAirgapNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source = source.into();
        Self {
            record_kind: MIRROR_AIRGAP_DEFECT_RECORD_KIND.to_owned(),
            schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "continuity:defect:mirror-airgap:{}:{}",
                narrow_reason.as_str(),
                source
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source,
            note: note.into(),
        }
    }
}

/// Aggregate summary for a mirror/air-gap continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapSummary {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Overall qualification for the page.
    pub overall_qualification_token: String,
    /// Number of packets.
    pub packet_count: usize,
    /// Number of distinct connectivity postures covered.
    pub posture_count: usize,
    /// Number of packets that carry offline-continuity evidence.
    pub offline_evidence_packet_count: usize,
    /// Number of mirror-only packets.
    pub mirror_only_count: usize,
    /// Number of air-gapped packets.
    pub air_gapped_count: usize,
    /// Number of self-hosted-restricted packets.
    pub self_hosted_restricted_count: usize,
    /// Number of offline packets that declare trust-root continuity.
    pub trust_root_declared_count: usize,
    /// Number of packets whose mirror or bundle freshness is acceptable.
    pub mirror_fresh_count: usize,
    /// Number of packets that need a fresh sync.
    pub needs_sync_count: usize,
    /// Number of packets whose public fallback is explicitly governed.
    pub public_fallback_governed_count: usize,
    /// Number of packets that narrowed below stable.
    pub narrowed_count: usize,
    /// Number of packets whose claim is withheld.
    pub withdrawn_count: usize,
    /// Number of tracked claim rows.
    pub claim_coverage_count: usize,
    /// Number of claim rows backed by a current packet.
    pub covered_claim_count: usize,
    /// Number of claim rows that narrowed for lack of a current packet.
    pub uncovered_claim_count: usize,
    /// Number of surface projections emitted.
    pub surface_projection_count: usize,
    /// True when every surface renders the same trust-root/advisory/fallback vocabulary.
    pub vocabulary_consistent: bool,
    /// True when every offline row declares trust-root continuity.
    pub all_offline_rows_declare_trust_root_continuity: bool,
    /// True when every offline row states an explicitly governed public-fallback policy.
    pub all_offline_rows_state_public_fallback_policy: bool,
    /// True when no offline row silently falls back to public endpoints.
    pub no_silent_public_fallback: bool,
    /// True when no isolated row's advisory or revocation implies a live public fetch.
    pub no_advisory_live_public_fetch_on_isolated: bool,
    /// True when every tracked claim row points to a current packet.
    pub all_expected_claims_covered: bool,
    /// True when at least one mirror-only and one air-gapped row are exercised.
    pub exercises_mirror_only_and_air_gapped: bool,
    /// True when trust-root and fallback fields are export-safe by default.
    pub fallback_and_trust_root_export_safe: bool,
    /// True when no raw provider payload is carried anywhere in the packet.
    pub raw_payloads_excluded: bool,
    /// Number of defects recorded for the page.
    pub defect_count: usize,
}

/// Full auditable input for a mirror/air-gap continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapInput {
    /// Reviewable label for the page.
    pub input_label: String,
    /// Claimed mirror/air-gap continuity packets.
    pub packets: Vec<MirrorAirgapPacketEntry>,
    /// Claim rows that carry offline-continuity language and must point to a current packet.
    pub expected_claim_row_ids: Vec<String>,
}

/// Canonical proof packet for the mirror/air-gap continuity lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page identifier.
    pub page_id: String,
    /// Reviewable page label.
    pub page_label: String,
    /// UTC timestamp when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from the embedded input and defects.
    pub summary: MirrorAirgapSummary,
    /// Typed defects for the packet.
    pub defects: Vec<MirrorAirgapDefect>,
    /// Plain-language descriptors, one per packet.
    pub descriptors: Vec<MirrorAirgapDescriptor>,
    /// Per-surface projections proving identical vocabulary across surfaces.
    pub surface_projections: Vec<MirrorAirgapSurfaceProjection>,
    /// Per-packet verdicts joining each packet to its computed qualification.
    pub outcomes: Vec<MirrorAirgapOutcome>,
    /// The typed consumer registry of claim-row coverage.
    pub registry: OfflineContinuityRegistry,
    /// The audited input embedded as evidence.
    pub input: MirrorAirgapInput,
}

impl MirrorAirgapPage {
    /// Builds a mirror/air-gap continuity page from the supplied input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: MirrorAirgapInput,
    ) -> Self {
        let descriptors: Vec<MirrorAirgapDescriptor> = input
            .packets
            .iter()
            .map(MirrorAirgapDescriptor::from_entry)
            .collect();
        let surface_projections = build_surface_projections(&input.packets);
        let defects = audit(&input, &surface_projections);
        let outcomes = build_outcomes(&input, &defects);
        let registry = build_registry(&input, &outcomes);
        let summary = build_summary(&input, &surface_projections, &outcomes, &registry, &defects);
        Self {
            record_kind: MIRROR_AIRGAP_PAGE_RECORD_KIND.to_owned(),
            schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            defects,
            descriptors,
            surface_projections,
            outcomes,
            registry,
            input,
        }
    }

    /// True when the page qualifies stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == ContinuityClaimQualificationClass::Stable.as_str()
    }

    /// True when every surface renders identical trust-root/advisory/fallback vocabulary.
    pub fn surfaces_share_vocabulary(&self) -> bool {
        self.summary.vocabulary_consistent
    }

    /// True when every tracked claim row points to a current packet.
    pub fn every_claim_covered(&self) -> bool {
        self.summary.all_expected_claims_covered
    }

    /// Returns the descriptor for a packet id, if present.
    pub fn descriptor(&self, packet_id: &str) -> Option<&MirrorAirgapDescriptor> {
        self.descriptors.iter().find(|d| d.packet_id == packet_id)
    }

    /// Returns the computed outcome for a packet id, if present.
    pub fn outcome(&self, packet_id: &str) -> Option<&MirrorAirgapOutcome> {
        self.outcomes.iter().find(|o| o.packet_id == packet_id)
    }
}

/// Support-export wrapper for the mirror/air-gap continuity page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MirrorAirgapSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export identifier.
    pub export_id: String,
    /// UTC timestamp when the export was produced.
    pub generated_at: String,
    /// The mirror/air-gap continuity page embedded as evidence.
    pub page: MirrorAirgapPage,
    /// Typed narrow reasons present in the embedded packet.
    pub narrow_reasons_present: Vec<MirrorAirgapNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when trust-root and fallback fields are export-safe by default.
    pub fallback_and_trust_root_export_safe: bool,
    /// True when raw provider payloads are excluded from this export.
    pub raw_payloads_excluded: bool,
}

impl MirrorAirgapSupportExport {
    /// Wraps a mirror/air-gap continuity page inside a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: MirrorAirgapPage,
    ) -> Self {
        let mut reasons: Vec<MirrorAirgapNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: MIRROR_AIRGAP_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            fallback_and_trust_root_export_safe: true,
            raw_payloads_excluded: true,
        }
    }
}

/// Re-runs the mirror/air-gap audit over a page, including its projections.
///
/// Unlike [`MirrorAirgapPage::new`], this validates the page's stored surface
/// projections against freshly derived canonical lines, so a tampered projection
/// (one that renders different vocabulary than its descriptor) is caught on
/// re-validation.
pub fn audit_mirror_airgap_page(page: &MirrorAirgapPage) -> Vec<MirrorAirgapDefect> {
    audit(&page.input, &page.surface_projections)
}

/// Validates a mirror/air-gap continuity page and returns `Ok(())` when clean.
pub fn validate_mirror_airgap_page(page: &MirrorAirgapPage) -> Result<(), Vec<MirrorAirgapDefect>> {
    let defects = audit_mirror_airgap_page(page);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Returns the seeded stable mirror/air-gap continuity page.
pub fn seeded_mirror_airgap_page() -> MirrorAirgapPage {
    MirrorAirgapPage::new(
        "continuity:mirror-airgap:seeded",
        "Mirror-only and air-gapped continuity packets",
        "2026-06-01T00:00:00Z",
        seeded_mirror_airgap_input(),
    )
}

/// Returns the seeded input used by the canonical mirror/air-gap page.
///
/// The seeded page carries one mirror-only self-hosted row, one air-gapped
/// sovereign row, one self-hosted-restricted row, and one exempt local-only row.
/// Every offline row declares trust-root continuity that survives offline, a
/// disclosed offline import and export path, an offline-safe advisory and
/// revocation source, and an explicitly governed public-fallback policy
/// (prohibited, unavailable, or policy-gated). Every claimed offline row points
/// to a current packet, so the page qualifies stable.
pub fn seeded_mirror_airgap_input() -> MirrorAirgapInput {
    let all = OfflineSurfaceClass::ALL.to_vec();
    let packets = vec![
        MirrorAirgapPacketEntry::new(
            "continuity-mirror:mirror-only-self-hosted",
            "continuity:row:mirror-only-self-hosted-registry",
            "Mirror-only self-hosted package and policy registry",
            ContinuityProfileClass::SelfHosted,
            ConnectivityPostureClass::MirrorOnly,
            true,
            TrustRootContinuity::new(
                TrustRootPostureClass::CustomerManagedTrustRoot,
                TrustRootRenewalClass::CustomerOperatedRotation,
                "The customer-managed trust root is rotated in the customer KMS; verification never reaches the public network.",
            ),
            MirrorFreshness::new(
                MirrorFreshnessStateClass::FreshWithinWindow,
                "2026-05-30T00:00:00Z",
                "2026-06-30T00:00:00Z",
                "mirror-ref:self-hosted-registry:2026-05-30",
            ),
            OfflineExchangeClass::MirrorPullPush,
            OfflineExchangeClass::MirrorPullPush,
            "Packages, policy bundles, advisories, and revocation lists are pulled from and pushed to the approved mirror.",
            AdvisoryRevocationSourceClass::MirrorReplicated,
            "Security advisories and revocation lists are replicated through the mirror; no live public fetch is made.",
            PublicFallbackPolicyClass::Prohibited,
            "Public fallback is prohibited by policy; the registry never reaches public endpoints.",
            all.clone(),
        ),
        MirrorAirgapPacketEntry::new(
            "continuity-mirror:air-gapped-sovereign",
            "continuity:row:air-gapped-sovereign-boundary",
            "Air-gapped sovereign deployment boundary",
            ContinuityProfileClass::Sovereign,
            ConnectivityPostureClass::AirGapped,
            true,
            TrustRootContinuity::new(
                TrustRootPostureClass::OfflineTrustRoot,
                TrustRootRenewalClass::OfflineSignedRotation,
                "The offline signed trust root is rotated by importing a signed rotation bundle across the air gap.",
            ),
            MirrorFreshness::new(
                MirrorFreshnessStateClass::NotApplicable,
                "",
                "",
                "bundle-ref:sovereign-offline:2026-05-15",
            ),
            OfflineExchangeClass::SignedOfflineBundle,
            OfflineExchangeClass::PhysicalMediaTransfer,
            "Updates arrive as signed offline bundles; exports leave on physical media after review. No network path crosses the boundary.",
            AdvisoryRevocationSourceClass::OfflineBundle,
            "Advisories and revocation lists arrive in the signed offline bundle; the boundary never fetches them live.",
            PublicFallbackPolicyClass::Unavailable,
            "No public endpoint is reachable across the air gap; public fallback is unavailable by construction.",
            all.clone(),
        ),
        MirrorAirgapPacketEntry::new(
            "continuity-mirror:self-hosted-restricted",
            "continuity:row:self-hosted-restricted-egress",
            "Self-hosted deployment with restricted egress",
            ContinuityProfileClass::SelfHosted,
            ConnectivityPostureClass::SelfHostedRestricted,
            true,
            TrustRootContinuity::new(
                TrustRootPostureClass::CustomerManagedTrustRoot,
                TrustRootRenewalClass::MirrorReplicatedRotation,
                "The customer-managed trust root is renewed through the approved mirror over the controlled egress path.",
            ),
            MirrorFreshness::new(
                MirrorFreshnessStateClass::StaleWithinGrace,
                "2026-05-10T00:00:00Z",
                "2026-06-25T00:00:00Z",
                "mirror-ref:restricted-egress:2026-05-10",
            ),
            OfflineExchangeClass::MirrorPullPush,
            OfflineExchangeClass::SignedOfflineBundle,
            "Content is pulled from the mirror over controlled egress; exports are produced as signed offline bundles for review.",
            AdvisoryRevocationSourceClass::MirrorReplicated,
            "Advisories and revocation lists are replicated through the mirror; a live public fetch is never required.",
            PublicFallbackPolicyClass::PolicyGated,
            "Public fallback exists but is gated behind an explicit, logged policy change; it is never attempted silently.",
            all.clone(),
        ),
        MirrorAirgapPacketEntry::new(
            "continuity-mirror:local-only-core",
            "continuity:row:local-desktop-core",
            "Local desktop core continuity",
            ContinuityProfileClass::LocalOnly,
            ConnectivityPostureClass::LocalOnly,
            false,
            TrustRootContinuity::new(
                TrustRootPostureClass::OsStoreTrustRoot,
                TrustRootRenewalClass::CustomerOperatedRotation,
                "The local editor trusts the OS keystore; nothing depends on a managed or mirror trust root.",
            ),
            MirrorFreshness::new(
                MirrorFreshnessStateClass::NotApplicable,
                "",
                "",
                "local-core:no-mirror",
            ),
            OfflineExchangeClass::NotApplicable,
            OfflineExchangeClass::NotApplicable,
            "Local editing, save, search, and version control need no boundary exchange.",
            AdvisoryRevocationSourceClass::LocalCacheOnly,
            "The local core carries no managed advisory or revocation lane.",
            PublicFallbackPolicyClass::NotApplicable,
            "Public fallback does not apply to a pure local-only surface.",
            all,
        ),
    ];
    MirrorAirgapInput {
        input_label:
            "Mirror-only, air-gapped, self-hosted-restricted, and local-only continuity packets"
                .to_owned(),
        expected_claim_row_ids: vec![
            "continuity:row:mirror-only-self-hosted-registry".to_owned(),
            "continuity:row:air-gapped-sovereign-boundary".to_owned(),
            "continuity:row:self-hosted-restricted-egress".to_owned(),
        ],
        packets,
    }
}

fn audit(
    input: &MirrorAirgapInput,
    projections: &[MirrorAirgapSurfaceProjection],
) -> Vec<MirrorAirgapDefect> {
    let mut defects = Vec::new();
    for packet in &input.packets {
        audit_packet(packet, &mut defects);
    }
    audit_vocabulary(input, projections, &mut defects);
    audit_claim_coverage(input, &mut defects);
    defects
}

fn audit_packet(packet: &MirrorAirgapPacketEntry, defects: &mut Vec<MirrorAirgapDefect>) {
    // Profile-vs-posture mismatch applies to every packet.
    if let Some(note) = packet.profile_posture_mismatch() {
        defects.push(MirrorAirgapDefect::new(
            MirrorAirgapNarrowReasonClass::ProfilePostureMismatch,
            packet.packet_id.clone(),
            note,
        ));
    }

    // Surface projection completeness applies to every packet.
    let missing = packet
        .required_surfaces()
        .iter()
        .any(|surface| !packet.projected_surfaces.contains(surface));
    if missing {
        defects.push(MirrorAirgapDefect::new(
            MirrorAirgapNarrowReasonClass::SurfaceReuseIncomplete,
            packet.packet_id.clone(),
            "every packet must reach the release-center, shiproom, support-center, partner-qualification, and public claim-manifest surfaces",
        ));
    }

    // Local-only rows are exempt from offline-continuity evidence.
    if !packet.requires_offline_continuity_evidence() {
        return;
    }

    // Headline guardrail: a mirror-only or air-gapped row may not silently fall
    // back to public endpoints.
    if packet.public_fallback_policy.is_silent_public_fallback() {
        defects.push(MirrorAirgapDefect::new(
            MirrorAirgapNarrowReasonClass::SilentPublicFallback,
            packet.packet_id.clone(),
            "an offline row may not silently fall back to public endpoints; state public fallback as prohibited, unavailable, or policy-gated",
        ));
    } else if !packet.public_fallback_policy.is_explicitly_governed() {
        defects.push(MirrorAirgapDefect::new(
            MirrorAirgapNarrowReasonClass::PublicFallbackUndisclosed,
            packet.packet_id.clone(),
            "an offline row must state whether public fallback is prohibited, unavailable, or requires an explicit policy change",
        ));
    }

    // Guardrail: advisory or revocation language may not imply a live public
    // fetch on a mirror-only or air-gapped row.
    if packet.advisory_revocation_source.implies_public_fetch() {
        if packet.forbids_public_fetch() {
            defects.push(MirrorAirgapDefect::new(
                MirrorAirgapNarrowReasonClass::AdvisoryImpliesLivePublicFetch,
                packet.packet_id.clone(),
                "a mirror-only or air-gapped row may not source advisories or revocation from a live public fetch",
            ));
        }
    } else if !packet.advisory_revocation_source.is_disclosed() {
        defects.push(MirrorAirgapDefect::new(
            MirrorAirgapNarrowReasonClass::AdvisoryRevocationUndisclosed,
            packet.packet_id.clone(),
            "an offline row must disclose where advisory and revocation data come from",
        ));
    }

    // Trust-root continuity. An undeclared trust root narrows; an isolated row
    // whose trust root needs a public reissue breaks offline continuity.
    if !packet.trust_root.is_declared() {
        defects.push(MirrorAirgapDefect::new(
            MirrorAirgapNarrowReasonClass::TrustRootContinuityUndeclared,
            packet.packet_id.clone(),
            "an offline row must declare its trust-root posture and how that trust root is renewed",
        ));
    } else if packet.forbids_public_fetch() && !packet.trust_root.survives_offline() {
        defects.push(MirrorAirgapDefect::new(
            MirrorAirgapNarrowReasonClass::TrustRootBreaksOffline,
            packet.packet_id.clone(),
            "a mirror-only or air-gapped row's trust root may not require a live public reissue to renew",
        ));
    }

    // Mirror or offline-bundle freshness.
    if packet.connectivity_posture.requires_live_mirror() {
        match packet.mirror_freshness.state {
            MirrorFreshnessStateClass::NeverSynced => defects.push(MirrorAirgapDefect::new(
                MirrorAirgapNarrowReasonClass::MirrorNeverSynced,
                packet.packet_id.clone(),
                "a mirror-only or self-hosted-restricted row whose mirror has never synced cannot exceed preview",
            )),
            MirrorFreshnessStateClass::StaleNeedsSync
            | MirrorFreshnessStateClass::NotApplicable => defects.push(MirrorAirgapDefect::new(
                MirrorAirgapNarrowReasonClass::MirrorFreshnessStale,
                packet.packet_id.clone(),
                "a mirror-backed row must name a mirror and a current freshness window",
            )),
            _ if packet.mirror_freshness.missing_sync_window() => {
                defects.push(MirrorAirgapDefect::new(
                    MirrorAirgapNarrowReasonClass::MirrorFreshnessStale,
                    packet.packet_id.clone(),
                    "fresh or graced mirror evidence must record a last-synced timestamp and a freshness expiry",
                ));
            }
            _ => {}
        }
    } else {
        // Air-gapped boundaries use offline bundles rather than a live mirror, so
        // a not-applicable mirror is acceptable; a stale live mirror is not.
        match packet.mirror_freshness.state {
            MirrorFreshnessStateClass::NeverSynced => defects.push(MirrorAirgapDefect::new(
                MirrorAirgapNarrowReasonClass::MirrorNeverSynced,
                packet.packet_id.clone(),
                "a row that claims a mirror it never synced cannot exceed preview",
            )),
            MirrorFreshnessStateClass::StaleNeedsSync => defects.push(MirrorAirgapDefect::new(
                MirrorAirgapNarrowReasonClass::MirrorFreshnessStale,
                packet.packet_id.clone(),
                "a stale mirror or bundle must be refreshed before the claim can stand",
            )),
            _ => {}
        }
    }

    // Offline import and export paths must be disclosed.
    if !packet.offline_import.is_disclosed() || !packet.offline_export.is_disclosed() {
        defects.push(MirrorAirgapDefect::new(
            MirrorAirgapNarrowReasonClass::OfflineExchangeUndisclosed,
            packet.packet_id.clone(),
            "an offline row must disclose its offline import and export paths",
        ));
    }
}

fn audit_vocabulary(
    input: &MirrorAirgapInput,
    projections: &[MirrorAirgapSurfaceProjection],
    defects: &mut Vec<MirrorAirgapDefect>,
) {
    for packet in &input.packets {
        let canonical_trust = trust_root_line(packet);
        let canonical_mirror = mirror_freshness_line(packet);
        let canonical_exchange = offline_exchange_line(packet);
        let canonical_advisory = advisory_line(packet);
        let canonical_fallback = public_fallback_line(packet);
        let drifted = projections
            .iter()
            .filter(|projection| projection.packet_id == packet.packet_id)
            .any(|projection| {
                projection.trust_root_line != canonical_trust
                    || projection.mirror_freshness_line != canonical_mirror
                    || projection.offline_exchange_line != canonical_exchange
                    || projection.advisory_line != canonical_advisory
                    || projection.public_fallback_line != canonical_fallback
            });
        if drifted {
            defects.push(MirrorAirgapDefect::new(
                MirrorAirgapNarrowReasonClass::PacketVocabularyDrift,
                packet.packet_id.clone(),
                "a surface renders different trust-root, mirror, offline-exchange, advisory, or public-fallback vocabulary than the descriptor",
            ));
        }
    }
}

fn audit_claim_coverage(input: &MirrorAirgapInput, defects: &mut Vec<MirrorAirgapDefect>) {
    for claim_row_id in &input.expected_claim_row_ids {
        let has_packet = input
            .packets
            .iter()
            .any(|packet| &packet.claim_row_id == claim_row_id);
        if !has_packet {
            defects.push(MirrorAirgapDefect::new(
                MirrorAirgapNarrowReasonClass::PacketEvidenceMissing,
                claim_row_id.clone(),
                "a claimed offline row carries no mirror/air-gap continuity packet; the claim narrows",
            ));
        }
    }
}

fn build_surface_projections(
    packets: &[MirrorAirgapPacketEntry],
) -> Vec<MirrorAirgapSurfaceProjection> {
    let mut projections = Vec::new();
    for packet in packets {
        let trust_root_line = trust_root_line(packet);
        let mirror_freshness_line = mirror_freshness_line(packet);
        let offline_exchange_line = offline_exchange_line(packet);
        let advisory_line = advisory_line(packet);
        let public_fallback_line = public_fallback_line(packet);
        let descriptor_id = format!("continuity:mirror-airgap-descriptor:{}", packet.packet_id);
        for surface in OfflineSurfaceClass::ALL {
            if !packet.projected_surfaces.contains(&surface) {
                continue;
            }
            projections.push(MirrorAirgapSurfaceProjection {
                record_kind: MIRROR_AIRGAP_SURFACE_PROJECTION_RECORD_KIND.to_owned(),
                schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
                shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
                surface,
                surface_token: surface.as_str().to_owned(),
                packet_id: packet.packet_id.clone(),
                descriptor_id: descriptor_id.clone(),
                trust_root_line: trust_root_line.clone(),
                mirror_freshness_line: mirror_freshness_line.clone(),
                offline_exchange_line: offline_exchange_line.clone(),
                advisory_line: advisory_line.clone(),
                public_fallback_line: public_fallback_line.clone(),
            });
        }
    }
    projections
}

fn build_outcomes(
    input: &MirrorAirgapInput,
    defects: &[MirrorAirgapDefect],
) -> Vec<MirrorAirgapOutcome> {
    input
        .packets
        .iter()
        .map(|packet| {
            let reasons: Vec<MirrorAirgapNarrowReasonClass> = defects
                .iter()
                .filter(|defect| defect.source == packet.packet_id)
                .map(|defect| defect.narrow_reason)
                .collect();
            let qualification = qualification_from_reasons(reasons.iter());
            let mut reason_tokens: Vec<String> = reasons
                .iter()
                .map(|reason| reason.as_str().to_owned())
                .collect();
            reason_tokens.sort();
            reason_tokens.dedup();
            MirrorAirgapOutcome {
                record_kind: MIRROR_AIRGAP_OUTCOME_RECORD_KIND.to_owned(),
                schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
                shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
                packet_id: packet.packet_id.clone(),
                claim_row_id: packet.claim_row_id.clone(),
                connectivity_posture_token: packet.connectivity_posture_token.clone(),
                qualification_token: qualification.as_str().to_owned(),
                narrowed: qualification != ContinuityClaimQualificationClass::Stable,
                claim_withheld: qualification == ContinuityClaimQualificationClass::Withdrawn,
                trust_root_posture_token: packet.trust_root.posture_token.clone(),
                trust_root_survives_offline: packet.trust_root.survives_offline(),
                mirror_freshness_token: packet.mirror_freshness.state_token.clone(),
                mirror_fresh: packet.mirror_freshness.state.is_acceptable(),
                advisory_revocation_source_token: packet.advisory_revocation_source_token.clone(),
                public_fallback_policy_token: packet.public_fallback_policy_token.clone(),
                public_fallback_governed: packet.public_fallback_policy.is_explicitly_governed(),
                narrow_reason_tokens: reason_tokens,
            }
        })
        .collect()
}

fn build_registry(
    input: &MirrorAirgapInput,
    outcomes: &[MirrorAirgapOutcome],
) -> OfflineContinuityRegistry {
    // The tracked claim rows are the declared offline rows plus every row a
    // packet actually backs, in stable sorted order.
    let mut claim_row_ids: Vec<String> = input.expected_claim_row_ids.clone();
    for packet in &input.packets {
        claim_row_ids.push(packet.claim_row_id.clone());
    }
    claim_row_ids.sort();
    claim_row_ids.dedup();

    let mut coverage = Vec::new();
    let mut covered = Vec::new();
    let mut uncovered = Vec::new();
    for claim_row_id in claim_row_ids {
        let outcome = outcomes
            .iter()
            .find(|outcome| outcome.claim_row_id == claim_row_id);
        let (coverage_class, qualification_token, packet_id) = match outcome {
            None => (
                OfflineCoverageClass::NoPacket,
                ContinuityClaimQualificationClass::Preview
                    .as_str()
                    .to_owned(),
                String::new(),
            ),
            Some(outcome) if outcome.claim_withheld => (
                OfflineCoverageClass::PacketWithheld,
                outcome.qualification_token.clone(),
                outcome.packet_id.clone(),
            ),
            Some(outcome) if outcome.narrowed => (
                OfflineCoverageClass::StalePacketNeedsRefresh,
                outcome.qualification_token.clone(),
                outcome.packet_id.clone(),
            ),
            Some(outcome) => (
                OfflineCoverageClass::CurrentPacket,
                outcome.qualification_token.clone(),
                outcome.packet_id.clone(),
            ),
        };
        let covered_now = coverage_class.is_covered();
        if covered_now {
            covered.push(claim_row_id.clone());
        } else {
            uncovered.push(claim_row_id.clone());
        }
        coverage.push(OfflineCoverageRow {
            record_kind: OFFLINE_COVERAGE_ROW_RECORD_KIND.to_owned(),
            schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
            shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
            claim_row_id,
            coverage_class,
            coverage_class_token: coverage_class.as_str().to_owned(),
            packet_id,
            qualification_token,
            covered: covered_now,
            narrowed: !covered_now,
        });
    }

    OfflineContinuityRegistry {
        record_kind: OFFLINE_CONTINUITY_REGISTRY_RECORD_KIND.to_owned(),
        schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
        shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
        registry_id: "continuity:offline-continuity-registry".to_owned(),
        coverage,
        covered_claim_row_ids: covered,
        uncovered_claim_row_ids: uncovered,
    }
}

fn build_summary(
    input: &MirrorAirgapInput,
    projections: &[MirrorAirgapSurfaceProjection],
    outcomes: &[MirrorAirgapOutcome],
    registry: &OfflineContinuityRegistry,
    defects: &[MirrorAirgapDefect],
) -> MirrorAirgapSummary {
    let overall = if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_withdrawal_reason())
    {
        ContinuityClaimQualificationClass::Withdrawn
    } else if defects
        .iter()
        .any(|defect| defect.narrow_reason.is_preview_reason())
    {
        ContinuityClaimQualificationClass::Preview
    } else if defects.is_empty() {
        ContinuityClaimQualificationClass::Stable
    } else {
        ContinuityClaimQualificationClass::Beta
    };

    let vocabulary_consistent = !defects
        .iter()
        .any(|defect| defect.narrow_reason == MirrorAirgapNarrowReasonClass::PacketVocabularyDrift);

    let mut postures: Vec<ConnectivityPostureClass> = input
        .packets
        .iter()
        .map(|packet| packet.connectivity_posture)
        .collect();
    postures.sort();
    postures.dedup();

    let offline_packets: Vec<&MirrorAirgapPacketEntry> = input
        .packets
        .iter()
        .filter(|packet| packet.requires_offline_continuity_evidence())
        .collect();

    let mirror_only_count = posture_count(input, ConnectivityPostureClass::MirrorOnly);
    let air_gapped_count = posture_count(input, ConnectivityPostureClass::AirGapped);

    MirrorAirgapSummary {
        record_kind: MIRROR_AIRGAP_SUMMARY_RECORD_KIND.to_owned(),
        schema_version: MIRROR_AIRGAP_SCHEMA_VERSION,
        shared_contract_ref: MIRROR_AIRGAP_SHARED_CONTRACT_REF.to_owned(),
        overall_qualification_token: overall.as_str().to_owned(),
        packet_count: input.packets.len(),
        posture_count: postures.len(),
        offline_evidence_packet_count: offline_packets.len(),
        mirror_only_count,
        air_gapped_count,
        self_hosted_restricted_count: posture_count(
            input,
            ConnectivityPostureClass::SelfHostedRestricted,
        ),
        trust_root_declared_count: offline_packets
            .iter()
            .filter(|packet| packet.trust_root.is_declared())
            .count(),
        mirror_fresh_count: input
            .packets
            .iter()
            .filter(|packet| packet.mirror_freshness.state.is_acceptable())
            .count(),
        needs_sync_count: input
            .packets
            .iter()
            .filter(|packet| packet.mirror_freshness.state.needs_sync())
            .count(),
        public_fallback_governed_count: input
            .packets
            .iter()
            .filter(|packet| packet.public_fallback_policy.is_explicitly_governed())
            .count(),
        narrowed_count: outcomes.iter().filter(|outcome| outcome.narrowed).count(),
        withdrawn_count: outcomes
            .iter()
            .filter(|outcome| outcome.claim_withheld)
            .count(),
        claim_coverage_count: registry.coverage.len(),
        covered_claim_count: registry.covered_claim_row_ids.len(),
        uncovered_claim_count: registry.uncovered_claim_row_ids.len(),
        surface_projection_count: projections.len(),
        vocabulary_consistent,
        all_offline_rows_declare_trust_root_continuity: offline_packets
            .iter()
            .all(|packet| packet.trust_root.is_declared()),
        all_offline_rows_state_public_fallback_policy: offline_packets
            .iter()
            .all(|packet| packet.public_fallback_policy.is_explicitly_governed()),
        no_silent_public_fallback: !input
            .packets
            .iter()
            .any(|packet| packet.public_fallback_policy.is_silent_public_fallback()),
        no_advisory_live_public_fetch_on_isolated: !input.packets.iter().any(|packet| {
            packet.forbids_public_fetch()
                && packet.advisory_revocation_source.implies_public_fetch()
        }),
        all_expected_claims_covered: input
            .expected_claim_row_ids
            .iter()
            .all(|claim_row_id| registry.is_claim_row_covered(claim_row_id)),
        exercises_mirror_only_and_air_gapped: mirror_only_count > 0 && air_gapped_count > 0,
        fallback_and_trust_root_export_safe: true,
        raw_payloads_excluded: true,
        defect_count: defects.len(),
    }
}

fn posture_count(input: &MirrorAirgapInput, posture: ConnectivityPostureClass) -> usize {
    input
        .packets
        .iter()
        .filter(|packet| packet.connectivity_posture == posture)
        .count()
}

fn trust_root_line(entry: &MirrorAirgapPacketEntry) -> String {
    let survives = if entry.trust_root.survives_offline() {
        "survives offline"
    } else {
        "does not survive offline"
    };
    let note = entry.trust_root.continuity_note.trim();
    let suffix = if note.is_empty() {
        String::new()
    } else {
        format!(" — {note}")
    };
    format!(
        "Trust-root continuity: {}; {}; {}.{}",
        entry.trust_root.posture.plain(),
        entry.trust_root.renewal.plain(),
        survives,
        suffix
    )
}

fn mirror_freshness_line(entry: &MirrorAirgapPacketEntry) -> String {
    let synced = if entry.mirror_freshness.last_synced_at.trim().is_empty() {
        String::new()
    } else {
        format!(
            " Last synced {}.",
            entry.mirror_freshness.last_synced_at.trim()
        )
    };
    format!(
        "Mirror freshness: {}.{}",
        entry.mirror_freshness.state.plain(),
        synced
    )
}

fn offline_exchange_line(entry: &MirrorAirgapPacketEntry) -> String {
    let note = entry.offline_exchange_note.trim();
    let suffix = if note.is_empty() {
        String::new()
    } else {
        format!(" {note}")
    };
    format!(
        "Offline import: {}; export: {}.{}",
        entry.offline_import.plain(),
        entry.offline_export.plain(),
        suffix
    )
}

fn advisory_line(entry: &MirrorAirgapPacketEntry) -> String {
    let note = entry.advisory_revocation_note.trim();
    if note.is_empty() {
        format!(
            "Advisory/revocation: {}.",
            entry.advisory_revocation_source.plain()
        )
    } else {
        format!(
            "Advisory/revocation: {} — {}",
            entry.advisory_revocation_source.plain(),
            note
        )
    }
}

fn public_fallback_line(entry: &MirrorAirgapPacketEntry) -> String {
    let note = entry.public_fallback_note.trim();
    if note.is_empty() {
        format!("Public fallback: {}.", entry.public_fallback_policy.plain())
    } else {
        format!(
            "Public fallback: {} — {}",
            entry.public_fallback_policy.plain(),
            note
        )
    }
}

fn profile_plain(class: ContinuityProfileClass) -> &'static str {
    match class {
        ContinuityProfileClass::Managed => "managed cloud",
        ContinuityProfileClass::SelfHosted => "self-hosted",
        ContinuityProfileClass::Sovereign => "sovereign",
        ContinuityProfileClass::LocalOnly => "local-only",
    }
}
