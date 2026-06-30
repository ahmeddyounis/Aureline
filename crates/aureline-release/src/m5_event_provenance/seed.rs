//! Canonical seed builders for the M5 event-provenance inspector.
//!
//! These builders are the single producer of the checked-in event-provenance packet, the published
//! inventory, the rendered overview document, the machine-readable event / facet matrix CSV, the
//! release-grade parity proof (and its Markdown report), the exported redaction-safe preview, and the
//! per-state drill fixtures. The headless emitter and the inline tests both call them so the in-code
//! packet, the artifacts, and the fixtures never drift. The canonical packet stands every deferred
//! action on a fully-traced event, a drift-free route against its last success, and a within-boundary
//! live approval, so every event is governed and may replay as-is; the drills perturb one facet of one
//! action — a stale provenance ledger, a region drift, a tenant drift, a narrowed boundary, or a
//! denied approval — and let the derivation recompute that event's gate and reapproval decision.

use super::*;

/// Stable packet id for the canonical (all-governed) event-provenance packet.
pub const M5_EVENT_PROVENANCE_PACKET_ID: &str = "m5-event-provenance:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every facet's proof is
/// current and every approval is in date.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// The retrieval epoch the canonical rows are read as-of.
const SEED_RETRIEVAL_EPOCH: &str = "epoch:2026-07-06T00:00:00Z";

/// The action whose provenance the stale-ledger drill narrows.
const PROVENANCE_DRILL_ACTION: DeferredAction = DeferredAction::QueuedPromptReplay;

/// The action whose route the region-drift drill narrows.
const DRIFT_REGION_ACTION: DeferredAction = DeferredAction::PublishLaterDataExport;

/// The action whose route the tenant-drift drill blocks.
const DRIFT_TENANT_ACTION: DeferredAction = DeferredAction::QueuedControlPlaneSync;

/// The action whose boundary the narrowed-boundary drill re-approves.
const REAPPROVAL_REQUIRED_ACTION: DeferredAction = DeferredAction::ScheduledCredentialRotation;

/// The action whose approval the denied-approval drill blocks.
const REAPPROVAL_BLOCKED_ACTION: DeferredAction = DeferredAction::RetriedPolicyPush;

/// The canonical route-hop state for an action: local work stays `local_only`; every crossing action
/// is fully attributed.
fn canonical_route_state(action: DeferredAction) -> RouteHopState {
    match action.host_lane() {
        HostLane::LocalMachine => RouteHopState::LocalOnly,
        _ => RouteHopState::AttributedRemote,
    }
}

/// The canonical provenance state for an action: derived outputs trace to their source; the rest are
/// fully traced. Both are governed.
fn canonical_provenance_state(action: DeferredAction) -> ProvenanceState {
    match action {
        DeferredAction::DeferredModelDownload
        | DeferredAction::PublishLaterDataExport
        | DeferredAction::ReplayedAuditExport => ProvenanceState::DerivedTraced,
        _ => ProvenanceState::FullyTraced,
    }
}

/// The canonical approval state for an action: standing-policy actions are pre-authorized; the rest
/// carry a named approval. Both are governed.
fn canonical_approval_state(action: DeferredAction) -> ApprovalState {
    match action {
        DeferredAction::DeferredModelDownload
        | DeferredAction::QueuedControlPlaneSync
        | DeferredAction::ReplayedAuditExport => ApprovalState::PreAuthorized,
        _ => ApprovalState::Approved,
    }
}

impl DeferredEventSeed {
    /// The canonical (fully governed) seed for an action: fully-traced event, drift-free route against
    /// its last success, within-boundary live approval, every proof current.
    fn canonical(action: DeferredAction) -> Self {
        Self {
            action,
            provenance_state: canonical_provenance_state(action),
            provenance_freshness: FreshnessState::Current,
            retrieval_epoch: SEED_RETRIEVAL_EPOCH.to_owned(),
            route_state: canonical_route_state(action),
            route_freshness: FreshnessState::Current,
            baseline: DriftBaseline::LastSuccess,
            drifted_facets: Vec::new(),
            boundary_state: CapabilityBoundaryState::WithinBoundary,
            approval_state: canonical_approval_state(action),
            approval_freshness: FreshnessState::Current,
        }
    }
}

/// The canonical seeds: every event fully governed.
fn canonical_seeds() -> Vec<DeferredEventSeed> {
    DeferredAction::ALL
        .iter()
        .map(|a| DeferredEventSeed::canonical(*a))
        .collect()
}

/// Replaces the seed for one action via the given mutator.
fn with_action(
    mut seeds: Vec<DeferredEventSeed>,
    action: DeferredAction,
    mutate: impl Fn(&mut DeferredEventSeed),
) -> Vec<DeferredEventSeed> {
    for seed in &mut seeds {
        if seed.action == action {
            mutate(seed);
        }
    }
    seeds
}

/// Assembles a packet from the given event seeds.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    event_seeds: Vec<DeferredEventSeed>,
) -> M5EventProvenance {
    M5EventProvenance::new(M5EventProvenanceInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        event_seeds,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-governed event-provenance packet: every deferred action on a fully-traced
/// event, a drift-free route, and a within-boundary live approval.
pub fn seeded_m5_event_provenance() -> M5EventProvenance {
    assemble_packet(
        M5_EVENT_PROVENANCE_PACKET_ID,
        "M5 event provenance",
        canonical_seeds(),
    )
}

/// Drill: one action's provenance ledger aged out, so its event-provenance row auto-narrows the event
/// below fully governed while every other event stays governed.
pub fn seeded_m5_event_provenance_provenance_stale_narrowed() -> M5EventProvenance {
    let seeds = with_action(canonical_seeds(), PROVENANCE_DRILL_ACTION, |seed| {
        seed.provenance_state = ProvenanceState::ProvenanceStale;
    });
    assemble_packet(
        "m5-event-provenance:drill-provenance-stale:0001",
        "M5 event provenance — stale-ledger drill",
        seeds,
    )
}

/// Drill: one action's route changed region since its last success, so the route-drift banner names
/// the region drift and narrows that event.
pub fn seeded_m5_event_provenance_drift_region_narrowed() -> M5EventProvenance {
    let seeds = with_action(canonical_seeds(), DRIFT_REGION_ACTION, |seed| {
        seed.baseline = DriftBaseline::LastSuccess;
        seed.drifted_facets = vec![DriftFactSeed {
            facet: DriftFacet::Region,
            planned_ref: "plan:region:home_region",
            current_ref: "observed:region:alternate_region",
        }];
    });
    assemble_packet(
        "m5-event-provenance:drill-drift-region:0001",
        "M5 event provenance — region-drift drill",
        seeds,
    )
}

/// Drill: one action's route changed tenant since it was planned, so the route-drift banner blocks
/// that event and holds Stable promotion — a tenant change crosses a hard isolation boundary.
pub fn seeded_m5_event_provenance_drift_tenant_blocked() -> M5EventProvenance {
    let seeds = with_action(canonical_seeds(), DRIFT_TENANT_ACTION, |seed| {
        seed.baseline = DriftBaseline::Plan;
        seed.drifted_facets = vec![DriftFactSeed {
            facet: DriftFacet::Tenant,
            planned_ref: "plan:tenant:home_tenant",
            current_ref: "observed:tenant:other_tenant",
        }];
    });
    assemble_packet(
        "m5-event-provenance:drill-drift-tenant:0001",
        "M5 event provenance — tenant-drift drill",
        seeds,
    )
}

/// Drill: one action's boundary narrowed since it was planned, so its replay / reapproval gate
/// requires re-approval before the deferred action may run again.
pub fn seeded_m5_event_provenance_reapproval_required_narrowed() -> M5EventProvenance {
    let seeds = with_action(canonical_seeds(), REAPPROVAL_REQUIRED_ACTION, |seed| {
        seed.boundary_state = CapabilityBoundaryState::AtBoundaryEdge;
    });
    assemble_packet(
        "m5-event-provenance:drill-reapproval-required:0001",
        "M5 event provenance — reapproval-required drill",
        seeds,
    )
}

/// Drill: one action's approval was denied, so its replay / reapproval gate holds the action and
/// blocks Stable promotion.
pub fn seeded_m5_event_provenance_reapproval_blocked() -> M5EventProvenance {
    let seeds = with_action(canonical_seeds(), REAPPROVAL_BLOCKED_ACTION, |seed| {
        seed.boundary_state = CapabilityBoundaryState::OutsideBoundary;
        seed.approval_state = ApprovalState::ApprovalDenied;
    });
    assemble_packet(
        "m5-event-provenance:drill-reapproval-blocked:0001",
        "M5 event provenance — reapproval-blocked drill",
        seeds,
    )
}
