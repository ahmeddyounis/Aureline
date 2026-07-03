//! Canonical seed builders for the M5 advisory-claim downgrade certification proof.
//!
//! These builders are the single producer of the checked-in packet, dashboard, support-export, and
//! CSV artifacts plus the blocked fixtures. The headless emitter and the inline tests both call
//! them so the in-code certification proof, the artifacts, and the fixtures never drift. The
//! claimed advisory-component families each profile evaluates are pulled straight from the frozen
//! advisory-component matrix's seeded packet, so the certification cannot audit a family the matrix
//! does not freeze, and the evaluated-family set is derived from the matrix rather than restated by
//! hand.

use super::*;
use crate::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix::{
    seeded_m5_advisory_component_matrix, M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID,
};

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-06-30T00:00:00Z";

/// Owner role accountable for every certified deployment profile.
const PROFILE_OWNER_ROLE: &str = "M5 security-advisory / release-claim owner";

/// Frozen, representative exact-build identity ref used by the seed.
///
/// A live runtime stamps the exact build identity here; the seed uses a fixed value so the
/// checked-in fixtures stay reproducible.
pub const SEED_BUILD_IDENTITY_REF: &str =
    "build-id:aureline:stable:1.0.0:aarch64-apple-darwin:release:9f3c1a2";

/// Frozen, representative release-channel class used by the seed.
pub const SEED_RELEASE_CHANNEL_CLASS: &str = "stable";

/// The advisory-claim downgrade posture seeded for one profile.
struct ProfileSpec {
    /// When set, the evaluated-family set used instead of the canonical full set (blocked fixtures
    /// use this to prove a partial evaluation blocks).
    evaluated_families_override: Option<Vec<M5AdvisoryComponentFamily>>,
    /// When set, the projected-channel set used instead of the canonical full set.
    projected_channels_override: Option<Vec<M5AdvisoryClaimChannel>>,
    advisory_freshness: AdvisoryFreshnessState,
    mirror_propagation: MirrorPropagationState,
    distribution_signature: DistributionSignatureState,
    local_continuity: LocalContinuityProofState,
    waiver: Option<AdvisoryClaimWaiver>,
    narrowing_reason: Option<&'static str>,
}

impl ProfileSpec {
    /// A full-standing posture: advisory freshness, mirror propagation, distribution signature, and
    /// local continuity all hold.
    fn stable() -> Self {
        Self {
            evaluated_families_override: None,
            projected_channels_override: None,
            advisory_freshness: AdvisoryFreshnessState::FreshAdvisoryStateCertified,
            mirror_propagation: MirrorPropagationState::MirrorCurrentAndPropagated,
            distribution_signature: DistributionSignatureState::FullySignedAndVerified,
            local_continuity: LocalContinuityProofState::LocalContinuityProvenAndSafe,
            waiver: None,
            narrowing_reason: None,
        }
    }
}

/// The claimed advisory-component families evaluated by every profile row, pulled from the frozen
/// matrix.
fn evaluated_families() -> Vec<M5AdvisoryComponentFamily> {
    seeded_m5_advisory_component_matrix()
        .component_rows
        .iter()
        .map(|matrix_row| matrix_row.component_family)
        .collect()
}

/// The five claim surfaces every profile row projects its downgrade state into.
fn projected_channels() -> Vec<M5AdvisoryClaimChannel> {
    M5AdvisoryClaimChannel::ALL.to_vec()
}

/// The profile-relevant downgrade triggers recorded on every row.
fn profile_downgrade_triggers() -> Vec<M5AdvisoryDowngradeTrigger> {
    vec![
        M5AdvisoryDowngradeTrigger::StaleNoticeStateSilent,
        M5AdvisoryDowngradeTrigger::MirrorLagUndisclosed,
        M5AdvisoryDowngradeTrigger::UnsignedDistributionUndisclosed,
        M5AdvisoryDowngradeTrigger::LocalContinuityHidden,
        M5AdvisoryDowngradeTrigger::ForcedDisableScopeHidden,
        M5AdvisoryDowngradeTrigger::ProofStale,
    ]
}

/// Short scenario summary for one profile.
fn scenario_summary(profile: M5AdvisoryClaimProfile) -> &'static str {
    match profile {
        M5AdvisoryClaimProfile::Managed => {
            "A centrally governed fleet where policy administers the advisory feed and the trusted \
             distribution; the reference profile every advisory claim is measured against."
        }
        M5AdvisoryClaimProfile::SelfHosted => {
            "A self-hosted install mirroring the advisory feed and distribution itself, where mirror \
             lag and partial re-verification are the standing exposure risks."
        }
        M5AdvisoryClaimProfile::Offline => {
            "An air-gapped install consuming a signed advisory/distribution bundle, where notice \
             staleness and a reduced local-continuity proof are the standing exposure risks."
        }
    }
}

/// Builds one certification row from a profile and a certification posture.
fn row_from_profile(profile: M5AdvisoryClaimProfile, spec: ProfileSpec) -> AdvisoryClaimRow {
    let families = spec
        .evaluated_families_override
        .unwrap_or_else(evaluated_families);
    let channels = spec
        .projected_channels_override
        .unwrap_or_else(projected_channels);
    let mut row = AdvisoryClaimRow {
        profile,
        profile_label: profile.label().to_owned(),
        owner_role: PROFILE_OWNER_ROLE.to_owned(),
        scenario_summary: scenario_summary(profile).to_owned(),
        evaluated_families: families,
        projected_channels: channels,
        advisory_freshness: spec.advisory_freshness,
        mirror_propagation: spec.mirror_propagation,
        distribution_signature: spec.distribution_signature,
        local_continuity: spec.local_continuity,
        applicable_downgrade_triggers: profile_downgrade_triggers(),
        active_waiver: spec.waiver,
        // Recomputed by the builder; the seed value is the derived status.
        derived_status: AdvisoryClaimStatus::Green,
        claim_states: Vec::new(),
        claim_causes: Vec::new(),
        narrowing_reason: spec.narrowing_reason.map(str::to_owned),
    };
    row.derived_status = row.recompute_status();
    row.claim_states = row.recompute_claim_states();
    row.claim_causes = row.recompute_causes();
    row
}

/// Builds the offline reduced-continuity-proof waiver carried by the seed.
fn offline_reduced_continuity_waiver() -> AdvisoryClaimWaiver {
    AdvisoryClaimWaiver {
        waiver_id: "waiver:offline-reduced-continuity-proof:0001".to_owned(),
        profile: M5AdvisoryClaimProfile::Offline,
        reason: "On an air-gapped install the local-continuity proof is reduced to the last signed \
                 bundle's evidence pending an operator acknowledgement, so the advisory claim is \
                 narrowed to a disclosed, waivered awaiting-user-action state while local work \
                 stays visibly safe; the full continuity proof is restored on the next bundle \
                 import."
            .to_owned(),
        owner_role: "Offline continuity surface owner".to_owned(),
        expires_at: "2026-09-30T00:00:00Z".to_owned(),
    }
}

/// Returns the seeded advisory-claim downgrade posture for one profile.
fn profile_spec(profile: M5AdvisoryClaimProfile) -> ProfileSpec {
    match profile {
        // Managed keeps full standing: the fleet administers a fresh feed, a current mirror, a
        // fully signed distribution, and a proven local-continuity report.
        M5AdvisoryClaimProfile::Managed => ProfileSpec::stable(),
        M5AdvisoryClaimProfile::SelfHosted => ProfileSpec {
            // A self-mirrored feed lags upstream and only part of the distribution is re-verified,
            // both disclosed.
            mirror_propagation: MirrorPropagationState::DisclosedMirrorLagNarrowing,
            distribution_signature:
                DistributionSignatureState::DisclosedPartialVerificationNarrowing,
            narrowing_reason: Some(
                "The self-hosted profile's advisory mirror lags upstream and only part of the \
                 distribution it trusts is re-verified, so the release/help/procurement/evaluation/\
                 support claim auto-narrows to disclosed mirror-lagged and unsigned/unverified \
                 states with refresh-mirror and re-sign/re-verify as the named restore actions, \
                 instead of staying silently green.",
            ),
            ..ProfileSpec::stable()
        },
        M5AdvisoryClaimProfile::Offline => ProfileSpec {
            // An offline bundle's notice is stale and its local-continuity proof is reduced pending
            // an operator acknowledgement, both disclosed; the continuity narrowing is waivered.
            advisory_freshness: AdvisoryFreshnessState::DisclosedStaleNoticeNarrowing,
            local_continuity: LocalContinuityProofState::DisclosedReducedContinuityProof,
            waiver: Some(offline_reduced_continuity_waiver()),
            narrowing_reason: Some(
                "The offline profile's advisory notice is stale between bundle imports and its \
                 local-continuity proof is reduced to the last signed bundle pending an operator \
                 acknowledgement, so the claim auto-narrows to disclosed warning-only and \
                 waivered awaiting-user-action states with await-notice-refresh and acknowledge-or-\
                 act as the named restore actions, instead of staying silently green.",
            ),
            ..ProfileSpec::stable()
        },
    }
}

/// Builds the certification rows for the canonical seed, one per claimed deployment profile.
fn seeded_rows() -> Vec<AdvisoryClaimRow> {
    M5AdvisoryClaimProfile::ALL
        .iter()
        .map(|&profile| row_from_profile(profile, profile_spec(profile)))
        .collect()
}

/// Builds a variant where one profile's spec is mutated after the canonical spec is resolved, used
/// by the blocked fixtures.
fn seeded_rows_with<F>(target: M5AdvisoryClaimProfile, mutate: F) -> Vec<AdvisoryClaimRow>
where
    F: Fn(&mut ProfileSpec),
{
    M5AdvisoryClaimProfile::ALL
        .iter()
        .map(|&profile| {
            let mut spec = profile_spec(profile);
            if profile == target {
                mutate(&mut spec);
            }
            row_from_profile(profile, spec)
        })
        .collect()
}

fn packet_from_rows(rows: Vec<AdvisoryClaimRow>) -> AdvisoryClaimPacket {
    build_m5_advisory_claim_downgrade_certification_packet(AdvisoryClaimInput {
        build_identity_ref: SEED_BUILD_IDENTITY_REF.to_owned(),
        release_channel_class: SEED_RELEASE_CHANNEL_CLASS.to_owned(),
        matrix_packet_ref: M5_ADVISORY_COMPONENTS_MATRIX_PACKET_ID.to_owned(),
        rows,
        generated_at: GENERATED_AT.to_owned(),
    })
}

/// Builds the canonical M5 advisory-claim downgrade certification packet.
///
/// This is the single producer of the checked-in packet, dashboard, support-export, and CSV
/// artifacts. The managed profile keeps full standing (green); the self-hosted profile auto-narrows
/// to yellow disclosing a mirror-lagged and unsigned/unverified claim; and the offline profile
/// auto-narrows to yellow disclosing a warning-only stale notice and a waivered awaiting-user-action
/// reduced-continuity proof — and no row is blocked, so the packet is clean and every row is
/// publishable while preserving four of the five distinct claim states.
pub fn seeded_m5_advisory_claim_downgrade_certification_packet() -> AdvisoryClaimPacket {
    packet_from_rows(seeded_rows())
}

/// Builds a variant where the self-hosted profile's mirror lag stays silently green, proving mirror
/// lag blocks the claim (red) rather than staying a disclosed yellow.
pub fn seeded_m5_advisory_claim_downgrade_certification_packet_self_hosted_mirror_lag_blocked(
) -> AdvisoryClaimPacket {
    let rows = seeded_rows_with(M5AdvisoryClaimProfile::SelfHosted, |spec| {
        spec.mirror_propagation = MirrorPropagationState::MirrorLaggedClaimOverclaimed;
        spec.distribution_signature = DistributionSignatureState::FullySignedAndVerified;
        spec.narrowing_reason = Some(
            "The self-hosted profile's advisory mirror lagged upstream but the claim stayed \
             silently green, overclaiming propagation, so the claim blocks before it can keep an \
             advisory claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the self-hosted profile trusts an unsigned distribution that stays
/// silently green, proving unsigned/unverified distribution blocks the claim (red).
pub fn seeded_m5_advisory_claim_downgrade_certification_packet_self_hosted_unsigned_blocked(
) -> AdvisoryClaimPacket {
    let rows = seeded_rows_with(M5AdvisoryClaimProfile::SelfHosted, |spec| {
        spec.mirror_propagation = MirrorPropagationState::MirrorCurrentAndPropagated;
        spec.distribution_signature = DistributionSignatureState::UnsignedOrUnverifiedDistribution;
        spec.narrowing_reason = Some(
            "The self-hosted profile trusted an unsigned, unverified distribution that stayed \
             silently green, overclaiming verified provenance, so the claim blocks before it can \
             keep an advisory claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the offline profile's advisory notice goes stale and stays silently
/// green, proving a stale notice blocks the claim (red) rather than staying a disclosed yellow.
pub fn seeded_m5_advisory_claim_downgrade_certification_packet_offline_stale_notice_blocked(
) -> AdvisoryClaimPacket {
    let rows = seeded_rows_with(M5AdvisoryClaimProfile::Offline, |spec| {
        spec.advisory_freshness = AdvisoryFreshnessState::AdvisoryStateStaleAndOverclaimed;
        spec.local_continuity = LocalContinuityProofState::LocalContinuityProvenAndSafe;
        spec.waiver = None;
        spec.narrowing_reason = Some(
            "The offline profile's advisory notice went stale between bundle imports but stayed \
             silently green, overclaiming currency, so the claim blocks before it can keep an \
             advisory claim.",
        );
    });
    packet_from_rows(rows)
}

/// Builds a variant where the managed profile loses its local-continuity proof, proving a missing
/// continuity proof forces the claim disabled (red) before it can keep its continuity claim.
pub fn seeded_m5_advisory_claim_downgrade_certification_packet_managed_continuity_lost_blocked(
) -> AdvisoryClaimPacket {
    let rows = seeded_rows_with(M5AdvisoryClaimProfile::Managed, |spec| {
        spec.local_continuity = LocalContinuityProofState::ContinuityProofMissingOrUnsafe;
        spec.narrowing_reason = Some(
            "The managed profile lost its local-continuity proof and forced-disable scope was \
             hidden behind a generic banner, so the claim is forcibly disabled before it can keep \
             its continuity claim.",
        );
    });
    packet_from_rows(rows)
}
