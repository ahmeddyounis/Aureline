//! Canonical seed builders for the M5 assurance center.
//!
//! These builders are the single producer of the checked-in assurance-center packet, the published
//! inventory, the rendered overview document, the machine-readable claim / control matrix CSV, the
//! release-grade parity proof (and its Markdown report), the exported evaluation packet, and the
//! per-state drill fixtures. The headless emitter and the inline tests both call them so the in-code
//! packet, the artifacts, and the fixtures never drift. The canonical packet proves every control at
//! a current proof, so every claim stands proven and every deployment profile is honored; the drills
//! perturb one control — a waiver, a stale proof, or a missing proof — and let the derivation
//! recompute each claim's active state, fallback, and the overviews that read it.

use super::*;

/// Stable packet id for the canonical (all-proven) assurance-center packet.
pub const M5_ASSURANCE_CENTER_PACKET_ID: &str = "m5-assurance-center:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every control's proof is
/// current.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The control the waiver drill holds under an accepted waiver. It backs the customer key-ownership
/// claim, so the drill narrows exactly that claim to `exception_pending`.
const WAIVER_DRILL_CONTROL: ControlId = ControlId::LocalKeyEscrow;

/// The control the stale drill perturbs. It backs the regulated-operation claim, so the drill
/// narrows exactly that claim to `under_review`.
const STALE_DRILL_CONTROL: ControlId = ControlId::RegulatedAuditTrail;

/// The control the missing drill perturbs. It backs the sovereign-deployment claim, so the drill
/// blocks exactly that claim to `unproven`.
const MISSING_DRILL_CONTROL: ControlId = ControlId::SovereignControlPlane;

/// The canonical control states: every control proven at current evidence.
fn canonical_control_states() -> Vec<(ControlId, AssuranceClaimState, FreshnessState)> {
    ControlId::ALL
        .iter()
        .map(|control| {
            (
                *control,
                AssuranceClaimState::Proven,
                FreshnessState::Current,
            )
        })
        .collect()
}

/// Overrides one control's proof state and evidence freshness.
fn with_control(
    mut states: Vec<(ControlId, AssuranceClaimState, FreshnessState)>,
    control: ControlId,
    state: AssuranceClaimState,
    freshness: FreshnessState,
) -> Vec<(ControlId, AssuranceClaimState, FreshnessState)> {
    for entry in &mut states {
        if entry.0 == control {
            *entry = (control, state, freshness);
        }
    }
    states
}

/// Assembles a packet from the given control states and exceptions.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    control_states: Vec<(ControlId, AssuranceClaimState, FreshnessState)>,
    exceptions: Vec<ExceptionWaiverSeed>,
) -> M5AssuranceCenter {
    M5AssuranceCenter::new(M5AssuranceCenterInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        control_states,
        exceptions,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-proven assurance-center packet: every control proven at a current proof, so
/// every claim stands proven and every deployment profile is honored.
pub fn seeded_m5_assurance_center() -> M5AssuranceCenter {
    assemble_packet(
        M5_ASSURANCE_CENTER_PACKET_ID,
        "M5 assurance center",
        canonical_control_states(),
        Vec::new(),
    )
}

/// Drill: a customer-accepted waiver holds the local key escrow control, so the customer
/// key-ownership claim narrows to `exception_pending` with a disclosed mitigation, expiry,
/// compensating control, and clearing action.
pub fn seeded_m5_assurance_center_waiver_narrowed() -> M5AssuranceCenter {
    let control_states = with_control(
        canonical_control_states(),
        WAIVER_DRILL_CONTROL,
        AssuranceClaimState::ExceptionPending,
        FreshnessState::Current,
    );
    let exceptions = vec![ExceptionWaiverSeed {
        control: WAIVER_DRILL_CONTROL,
        mitigation: "Keys held in customer HSM; local escrow deferred to next maintenance window."
            .to_owned(),
        expiry: "2026-09-30T00:00:00Z".to_owned(),
        compensating_control: ControlId::CustomerManagedKeyCustody,
        responsible_party: ResponsibleParty::Customer,
        action: WaiverAction::EnableCompensatingControl,
    }];
    assemble_packet(
        "m5-assurance-center:drill-waiver:0001",
        "M5 assurance center — accepted-waiver drill",
        control_states,
        exceptions,
    )
}

/// Drill: one control's evidence is stale, so the claim that requires it auto-narrows to
/// `under_review` below its claimed posture.
pub fn seeded_m5_assurance_center_stale_evidence_narrowed() -> M5AssuranceCenter {
    let control_states = with_control(
        canonical_control_states(),
        STALE_DRILL_CONTROL,
        AssuranceClaimState::Proven,
        FreshnessState::Stale,
    );
    assemble_packet(
        "m5-assurance-center:drill-stale:0001",
        "M5 assurance center — stale-evidence drill",
        control_states,
        Vec::new(),
    )
}

/// Drill: one control's evidence is missing, so the claim that requires it is blocked to `unproven`
/// and Stable promotion is held.
pub fn seeded_m5_assurance_center_missing_evidence_blocked() -> M5AssuranceCenter {
    let control_states = with_control(
        canonical_control_states(),
        MISSING_DRILL_CONTROL,
        AssuranceClaimState::Proven,
        FreshnessState::Missing,
    );
    assemble_packet(
        "m5-assurance-center:drill-missing:0001",
        "M5 assurance center — missing-evidence drill",
        control_states,
        Vec::new(),
    )
}
