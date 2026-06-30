//! Canonical seed builders for the M5 assurance-claim reducer.
//!
//! These builders are the single producer of the checked-in reducer packet, the published inventory,
//! the rendered narrowing overview document, the machine-readable claim / precondition matrix CSV, the
//! release-grade narrowing proof (and its Markdown report), the exported redaction-safe preview, and
//! the per-state drill fixtures. The headless emitter and the inline tests both call them so the
//! in-code packet, the artifacts, and the fixtures never drift. The canonical packet holds every
//! precondition satisfied, so every regulated claim stays proven; each drill perturbs one global
//! precondition — stale evidence, hosted-dependency drift, a key / residency mismatch, or a policy-path
//! regression — and lets the reducer narrow or block exactly the claims that depend on it, recording
//! which precondition drifted.

use super::*;

/// Stable packet id for the canonical (all-satisfied) reducer packet.
pub const M5_ASSURANCE_CLAIM_REDUCER_PACKET_ID: &str = "m5-assurance-claim-reducer:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every precondition holds.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The canonical precondition states: every precondition satisfied.
fn canonical_precondition_states() -> Vec<(ClaimPrecondition, PreconditionStatus)> {
    ClaimPrecondition::ALL
        .iter()
        .map(|precondition| (*precondition, PreconditionStatus::Satisfied))
        .collect()
}

/// Overrides one precondition's status.
fn with_precondition(
    mut states: Vec<(ClaimPrecondition, PreconditionStatus)>,
    precondition: ClaimPrecondition,
    status: PreconditionStatus,
) -> Vec<(ClaimPrecondition, PreconditionStatus)> {
    for entry in &mut states {
        if entry.0 == precondition {
            *entry = (precondition, status);
        }
    }
    states
}

/// Assembles a packet from the given precondition states.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    precondition_states: Vec<(ClaimPrecondition, PreconditionStatus)>,
) -> M5AssuranceClaimReducer {
    M5AssuranceClaimReducer::new(M5AssuranceClaimReducerInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        precondition_states,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-satisfied reducer packet: every precondition holds, so every regulated claim
/// stays proven and every consumer reads the same proven state.
pub fn seeded_m5_assurance_claim_reducer() -> M5AssuranceClaimReducer {
    assemble_packet(
        M5_ASSURANCE_CLAIM_REDUCER_PACKET_ID,
        "M5 assurance-claim reducer",
        canonical_precondition_states(),
    )
}

/// Drill: the supporting evidence is stale, so every claim that depends on fresh evidence auto-narrows
/// to `under_review` below its claimed posture, attributed to the `stale_evidence` drift.
pub fn seeded_m5_assurance_claim_reducer_stale_evidence_narrowed() -> M5AssuranceClaimReducer {
    let states = with_precondition(
        canonical_precondition_states(),
        ClaimPrecondition::EvidenceFreshness,
        PreconditionStatus::Drifted,
    );
    assemble_packet(
        "m5-assurance-claim-reducer:drill-stale-evidence:0001",
        "M5 assurance-claim reducer — stale-evidence drill",
        states,
    )
}

/// Drill: a hosted dependency drifted toward the claim boundary, so the claims that depend on the
/// hosted-dependency boundary narrow to `under_review`, attributed to the `hosted_dependency_drift`
/// drift, while claims that do not depend on it stay proven.
pub fn seeded_m5_assurance_claim_reducer_hosted_dependency_drift_narrowed(
) -> M5AssuranceClaimReducer {
    let states = with_precondition(
        canonical_precondition_states(),
        ClaimPrecondition::HostedDependencyBoundary,
        PreconditionStatus::Drifted,
    );
    assemble_packet(
        "m5-assurance-claim-reducer:drill-hosted-dependency:0001",
        "M5 assurance-claim reducer — hosted-dependency-drift drill",
        states,
    )
}

/// Drill: key / data residency no longer matches the claim, so the claims that depend on key / data
/// residency are blocked to `unproven` and Stable promotion is held, attributed to the
/// `key_residency_mismatch` drift.
pub fn seeded_m5_assurance_claim_reducer_key_residency_mismatch_blocked() -> M5AssuranceClaimReducer
{
    let states = with_precondition(
        canonical_precondition_states(),
        ClaimPrecondition::KeyResidency,
        PreconditionStatus::Invalidated,
    );
    assemble_packet(
        "m5-assurance-claim-reducer:drill-key-residency:0001",
        "M5 assurance-claim reducer — key-residency-mismatch drill",
        states,
    )
}

/// Drill: the required policy / control path regressed, so the claims that depend on it are blocked to
/// `unproven` and Stable promotion is held, attributed to the `policy_path_regression` drift.
pub fn seeded_m5_assurance_claim_reducer_policy_path_regression_blocked() -> M5AssuranceClaimReducer
{
    let states = with_precondition(
        canonical_precondition_states(),
        ClaimPrecondition::PolicyControlPath,
        PreconditionStatus::Invalidated,
    );
    assemble_packet(
        "m5-assurance-claim-reducer:drill-policy-path:0001",
        "M5 assurance-claim reducer — policy-path-regression drill",
        states,
    )
}
