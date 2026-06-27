//! Canonical seed builders for the M5 runbook certification packet.
//!
//! These builders are the single producer of the checked-in certification packet, the
//! published inventory, the Markdown proof, and the stale / missing-proof drill
//! fixtures. The headless emitter and the inline tests both call them so the in-code
//! packet, the artifacts, and the fixtures never drift. Every builder derives each
//! row's verdict from the same checked-in proof-lane contracts, so the qualification is
//! always generated from the lane proofs Aureline ships: the canonical packet is
//! all-certified; the drills perturb one lane's proof freshness and let the derivation
//! recompute each row's status, gate, effective claim, and named gaps.

use super::*;

/// Stable packet id for the canonical (all-certified) certification packet.
pub const M5_RUNBOOK_CERTIFICATION_PACKET_ID: &str = "m5-runbook-certification:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every lane's
/// proof is current.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The lane the stale drill perturbs. It is bound by the operator and support/release
/// rows but not the docs reference, so the drill narrows exactly the rows that depend
/// on browser/console boundary honesty.
const STALE_DRILL_LANE: RunbookProofLane = RunbookProofLane::Handoffs;

/// The lane the missing drill perturbs. It is bound by the companion, support, and
/// release rows, so the drill blocks exactly the rows that depend on companion-scoped
/// export proof.
const MISSING_DRILL_LANE: RunbookProofLane = RunbookProofLane::Companion;

/// The claimed incident/operator rows and the proof lanes each binds. Together the
/// bindings cover every proof lane.
const ROW_DEFS: [(&str, &str, RunbookConsumer, &str, &[RunbookProofLane]); 7] = [
    (
        "incident-runbook-execution-pane",
        "Incident workspace runbook execution pane",
        RunbookConsumer::IncidentWorkspace,
        "incident_operations_owner",
        &[
            RunbookProofLane::Governance,
            RunbookProofLane::Sources,
            RunbookProofLane::Steps,
            RunbookProofLane::Executions,
        ],
    ),
    (
        "operator-runbook-history",
        "Operator dashboard runbook execution history",
        RunbookConsumer::OperatorDashboard,
        "operator_console_owner",
        &[
            RunbookProofLane::Governance,
            RunbookProofLane::Steps,
            RunbookProofLane::Executions,
            RunbookProofLane::Handoffs,
        ],
    ),
    (
        "operator-console-boundary-pane",
        "Operator dashboard browser/console boundary pane",
        RunbookConsumer::OperatorDashboard,
        "control_plane_boundary_owner",
        &[RunbookProofLane::Governance, RunbookProofLane::Handoffs],
    ),
    (
        "companion-runbook-follow",
        "Companion runbook follow surface",
        RunbookConsumer::Companion,
        "companion_owner",
        &[
            RunbookProofLane::Governance,
            RunbookProofLane::Steps,
            RunbookProofLane::Companion,
        ],
    ),
    (
        "support-runbook-bundle",
        "Support bundle runbook export",
        RunbookConsumer::SupportBundle,
        "support_export_owner",
        &[
            RunbookProofLane::Executions,
            RunbookProofLane::Handoffs,
            RunbookProofLane::Companion,
        ],
    ),
    (
        "docs-runbook-reference",
        "Docs/Help runbook reference",
        RunbookConsumer::DocsHelp,
        "docs_help_owner",
        &[
            RunbookProofLane::Governance,
            RunbookProofLane::Sources,
            RunbookProofLane::Steps,
        ],
    ),
    (
        "release-runbook-certification-gate",
        "Release center runbook certification gate",
        RunbookConsumer::ReleaseCenter,
        "release_center_owner",
        &[
            RunbookProofLane::Governance,
            RunbookProofLane::Sources,
            RunbookProofLane::Steps,
            RunbookProofLane::Executions,
            RunbookProofLane::Handoffs,
            RunbookProofLane::Companion,
        ],
    ),
];

/// Builds the canonical proof-lane contracts with every proof current.
fn canonical_proof_lanes() -> Vec<RunbookProofLaneContract> {
    RunbookProofLane::ALL
        .iter()
        .map(|lane| RunbookProofLaneContract::for_lane(*lane, ProofFreshnessState::Current))
        .collect()
}

/// Marks one lane's proof at the given freshness state.
fn with_lane_state(
    mut lanes: Vec<RunbookProofLaneContract>,
    lane: RunbookProofLane,
    state: ProofFreshnessState,
) -> Vec<RunbookProofLaneContract> {
    for contract in &mut lanes {
        if contract.lane == lane {
            contract.proof_freshness = state;
        }
    }
    lanes
}

/// Builds a row from a definition; gaps and verdict are recomputed in the packet.
fn build_row(
    def: &(&str, &str, RunbookConsumer, &str, &[RunbookProofLane]),
) -> IncidentOperatorRow {
    let (row_id, label, consumer, owner, bound) = def;
    IncidentOperatorRow {
        row_id: (*row_id).to_owned(),
        row_label: (*label).to_owned(),
        consumer: *consumer,
        owner_role: (*owner).to_owned(),
        claimed_class: RunbookClaimClass::Stable,
        bound_lanes: bound.to_vec(),
        covered_facets: Vec::new(),
        effective_class: RunbookClaimClass::Stable,
        status: RunbookSurfaceStatus::Mapped,
        signal: RunbookSignal::Green,
        gate_decision: RunbookGate::Governed,
        gaps: Vec::new(),
        status_message_id: format!(
            "{}{}.status",
            M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX, row_id
        ),
        gate_message_id: format!(
            "{}{}.gate",
            M5_RUNBOOK_CERTIFICATION_MESSAGE_ID_PREFIX, row_id
        ),
    }
}

/// Assembles a packet from the given lane contracts.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    lanes: Vec<RunbookProofLaneContract>,
) -> M5RunbookCertificationPacket {
    let rows: Vec<IncidentOperatorRow> = ROW_DEFS.iter().map(build_row).collect();
    M5RunbookCertificationPacket::new(M5RunbookCertificationPacketInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        proof_lanes: lanes,
        rows,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-certified runbook certification packet.
pub fn seeded_m5_runbook_certification_packet() -> M5RunbookCertificationPacket {
    assemble_packet(
        M5_RUNBOOK_CERTIFICATION_PACKET_ID,
        "M5 runbook certification",
        canonical_proof_lanes(),
    )
}

/// Drill: one lane's proof is stale, so the rows that bind it auto-narrow below Stable.
pub fn seeded_m5_runbook_certification_packet_stale_proof_narrowed() -> M5RunbookCertificationPacket
{
    let lanes = with_lane_state(
        canonical_proof_lanes(),
        STALE_DRILL_LANE,
        ProofFreshnessState::Stale,
    );
    assemble_packet(
        "m5-runbook-certification:drill-stale:0001",
        "M5 runbook certification — stale-proof drill",
        lanes,
    )
}

/// Drill: one lane's proof is missing, so the rows that bind it are blocked from Stable.
pub fn seeded_m5_runbook_certification_packet_missing_proof_blocked() -> M5RunbookCertificationPacket
{
    let lanes = with_lane_state(
        canonical_proof_lanes(),
        MISSING_DRILL_LANE,
        ProofFreshnessState::Missing,
    );
    assemble_packet(
        "m5-runbook-certification:drill-missing:0001",
        "M5 runbook certification — missing-proof drill",
        lanes,
    )
}
