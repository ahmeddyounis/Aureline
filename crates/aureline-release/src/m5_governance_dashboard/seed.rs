//! Canonical seed builders for the M5 governance dashboard.
//!
//! These builders are the single producer of the checked-in governance-dashboard packet, the
//! published inventory, the rendered overview document, the machine-readable fitness-tile matrix CSV,
//! the release-grade parity proof (and its Markdown report), the exported evaluation packet, and the
//! per-state drill fixtures. The headless emitter and the inline tests both call them so the in-code
//! packet, the artifacts, and the fixtures never drift. The canonical packet measures every fitness
//! function passing at current evidence, so every service is clean, every decision is exercisable,
//! and every deployment profile is honored; the drills perturb one function — a warning, stale
//! evidence, an in-date waiver, an expired waiver, or missing evidence — and let the derivation
//! recompute each tile, the waiver queue, the cards, and the overviews that read it.

use super::*;

/// Stable packet id for the canonical (all-passing) governance-dashboard packet.
pub const M5_GOVERNANCE_DASHBOARD_PACKET_ID: &str = "m5-governance-dashboard:stable:0001";

/// The reference corpus the canonical dashboard is measured against.
const SEED_CORPUS_ID: &str = "m5-reference-corpus:0001";

/// Human-readable corpus label.
const SEED_CORPUS_LABEL: &str = "M5 reference corpus";

/// Evaluation / mint timestamp for the canonical packet.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The function the warning drill perturbs. It is required under the managed profile, so the drill
/// narrows every profile's overview to `warning`.
const WARNING_DRILL_FUNCTION: FitnessFunction = FitnessFunction::SchemaExampleParity;

/// The function the stale drill perturbs. It is required under the regulated profile, so the drill
/// narrows the regulated profile to `evidence_stale`.
const STALE_DRILL_FUNCTION: FitnessFunction = FitnessFunction::ClaimNoOverclaim;

/// The function the in-date-waiver drill holds. It is required under the regulated profile, so the
/// drill narrows that profile to `waived`.
const WAIVER_DRILL_FUNCTION: FitnessFunction = FitnessFunction::EvidenceFreshnessSlo;

/// The function the expired-waiver drill holds. It is required under the sovereign profile, so the
/// drill blocks that profile to `waiver_expired`.
const EXPIRED_WAIVER_DRILL_FUNCTION: FitnessFunction = FitnessFunction::ProvenanceCompleteness;

/// The function the missing drill perturbs. It is required under the sovereign profile, so the drill
/// blocks that profile to `blocked`.
const MISSING_DRILL_FUNCTION: FitnessFunction = FitnessFunction::RouteExplainability;

/// The canonical function states: every function passing at current evidence, no waiver.
fn canonical_function_states() -> Vec<FitnessFunctionState> {
    FitnessFunction::ALL
        .iter()
        .map(|function| FitnessFunctionState {
            function: *function,
            measure: FitnessMeasure::Pass,
            freshness: FreshnessState::Current,
            last_run_at: SEED_EVALUATED_AT.to_owned(),
            consecutive_passing_runs: 42,
            waiver: None,
        })
        .collect()
}

/// Overrides one function's measured result, freshness, and waiver.
fn with_function(
    mut states: Vec<FitnessFunctionState>,
    function: FitnessFunction,
    measure: FitnessMeasure,
    freshness: FreshnessState,
    waiver: Option<WaiverSeed>,
) -> Vec<FitnessFunctionState> {
    for entry in &mut states {
        if entry.function == function {
            entry.measure = measure;
            entry.freshness = freshness;
            entry.waiver = waiver.clone();
            entry.consecutive_passing_runs = 0;
        }
    }
    states
}

/// Assembles a packet from the given function states.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    function_states: Vec<FitnessFunctionState>,
) -> M5GovernanceDashboard {
    M5GovernanceDashboard::new(M5GovernanceDashboardInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        corpus_id: SEED_CORPUS_ID.to_owned(),
        corpus_label: SEED_CORPUS_LABEL.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        function_states,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-passing governance-dashboard packet: every fitness function passing at current
/// evidence, so every service is clean, every decision is exercisable, and every profile is honored.
pub fn seeded_m5_governance_dashboard() -> M5GovernanceDashboard {
    assemble_packet(
        M5_GOVERNANCE_DASHBOARD_PACKET_ID,
        "M5 governance dashboard",
        canonical_function_states(),
    )
}

/// Drill: one fitness function breaches a warning threshold, so its tile reads `warning` and the
/// profiles that require it narrow below their claimed posture.
pub fn seeded_m5_governance_dashboard_warning() -> M5GovernanceDashboard {
    let states = with_function(
        canonical_function_states(),
        WARNING_DRILL_FUNCTION,
        FitnessMeasure::Warn,
        FreshnessState::Current,
        None,
    );
    assemble_packet(
        "m5-governance-dashboard:drill-warning:0001",
        "M5 governance dashboard — warning drill",
        states,
    )
}

/// Drill: one fitness function's evidence is stale, so its tile reads `evidence_stale` and the
/// profile that requires it auto-narrows below its claimed posture.
pub fn seeded_m5_governance_dashboard_evidence_stale_narrowed() -> M5GovernanceDashboard {
    let states = with_function(
        canonical_function_states(),
        STALE_DRILL_FUNCTION,
        FitnessMeasure::Pass,
        FreshnessState::Stale,
        None,
    );
    assemble_packet(
        "m5-governance-dashboard:drill-stale:0001",
        "M5 governance dashboard — stale-evidence drill",
        states,
    )
}

/// Drill: a failing fitness function is held under an accepted, in-date waiver, so its tile reads
/// `waived`, the waiver queue carries one open row disclosing its expiry and clearing action, and
/// Stable promotion is not blocked.
pub fn seeded_m5_governance_dashboard_waiver_active_narrowed() -> M5GovernanceDashboard {
    let waiver = WaiverSeed {
        standing: WaiverStanding::Active,
        expiry: "2026-09-30T00:00:00Z".to_owned(),
        rationale: "Freshness SLO breach accepted while the nightly corpus refresh lands."
            .to_owned(),
        responsible_party: WaiverParty::ServiceOwner,
        action: WaiverClearingAction::RemediateAndReverify,
        ticket_ref: "gov-waiver-0001".to_owned(),
    };
    let states = with_function(
        canonical_function_states(),
        WAIVER_DRILL_FUNCTION,
        FitnessMeasure::Fail,
        FreshnessState::Current,
        Some(waiver),
    );
    assemble_packet(
        "m5-governance-dashboard:drill-waiver-active:0001",
        "M5 governance dashboard — active-waiver drill",
        states,
    )
}

/// Drill: a failing fitness function's waiver has expired, so its tile reads `waiver_expired`, the
/// waiver queue heads with one expired row, the profile that requires it is blocked, and Stable
/// promotion is held.
pub fn seeded_m5_governance_dashboard_waiver_expired_blocked() -> M5GovernanceDashboard {
    let waiver = WaiverSeed {
        standing: WaiverStanding::Expired,
        expiry: "2026-05-31T00:00:00Z".to_owned(),
        rationale: "Provenance backfill waiver lapsed; the gap is no longer covered.".to_owned(),
        responsible_party: WaiverParty::GovernanceOwner,
        action: WaiverClearingAction::RenewWaiver,
        ticket_ref: "gov-waiver-0002".to_owned(),
    };
    let states = with_function(
        canonical_function_states(),
        EXPIRED_WAIVER_DRILL_FUNCTION,
        FitnessMeasure::Fail,
        FreshnessState::Current,
        Some(waiver),
    );
    assemble_packet(
        "m5-governance-dashboard:drill-waiver-expired:0001",
        "M5 governance dashboard — expired-waiver drill",
        states,
    )
}

/// Drill: one fitness function's evidence is missing, so its tile reads `blocked`, the profile that
/// requires it is blocked, and Stable promotion is held.
pub fn seeded_m5_governance_dashboard_missing_evidence_blocked() -> M5GovernanceDashboard {
    let states = with_function(
        canonical_function_states(),
        MISSING_DRILL_FUNCTION,
        FitnessMeasure::Pass,
        FreshnessState::Missing,
        None,
    );
    assemble_packet(
        "m5-governance-dashboard:drill-missing:0001",
        "M5 governance dashboard — missing-evidence drill",
        states,
    )
}
