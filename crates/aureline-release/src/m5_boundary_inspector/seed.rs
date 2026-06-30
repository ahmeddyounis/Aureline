//! Canonical seed builders for the M5 boundary inspector.
//!
//! These builders are the single producer of the checked-in boundary-inspector packet, the published
//! inventory, the rendered overview document, the machine-readable action / facet matrix CSV, the
//! release-grade parity proof (and its Markdown report), the exported evaluation packet, and the
//! per-state drill fixtures. The headless emitter and the inline tests both call them so the in-code
//! packet, the artifacts, and the fixtures never drift. The canonical packet stands every action
//! within boundary, on a fully attributed route, under a live approval, so every inspector is
//! governed; the drills perturb one facet of one action — a boundary edge, a route drift, an
//! unattributed hop, or an expired approval — and let the derivation recompute that action's gate.

use super::*;

/// Stable packet id for the canonical (all-governed) boundary-inspector packet.
pub const M5_BOUNDARY_INSPECTOR_PACKET_ID: &str = "m5-boundary-inspector:stable:0001";

/// Evaluation / mint timestamp for the canonical packet — a date at which every facet's proof is
/// current and every approval is in date.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

/// A far-future approval expiry used by the canonical seeds.
const ACTIVE_EXPIRY: &str = "2027-06-30T00:00:00Z";

/// A past approval expiry used by the expired-approval drill.
const PAST_EXPIRY: &str = "2026-01-15T00:00:00Z";

/// The action whose boundary the boundary-edge drill narrows.
const BOUNDARY_DRILL_ACTION: HighRiskAction = HighRiskAction::WorkspaceDataExport;

/// The action whose route the mirror-substitution drill narrows.
const ROUTE_DRIFT_ACTION: HighRiskAction = HighRiskAction::RemoteModelInference;

/// The action whose route the unattributed-hop drill blocks.
const ROUTE_BLOCK_ACTION: HighRiskAction = HighRiskAction::SupportBundleHandoff;

/// The action whose approval the expired-approval drill blocks.
const APPROVAL_DRILL_ACTION: HighRiskAction = HighRiskAction::ProviderCredentialRotation;

/// The canonical route-hop state for an action: local work stays `local_only`; every crossing action
/// is fully attributed.
fn canonical_route_state(action: HighRiskAction) -> RouteHopState {
    match action {
        HighRiskAction::LocalModelExecution => RouteHopState::LocalOnly,
        _ => RouteHopState::AttributedRemote,
    }
}

/// A non-drifting hop seed.
fn hop(
    locality: HopLocality,
    role: HopRole,
    certificate_context: CertificateContext,
) -> RouteHopSeed {
    RouteHopSeed {
        locality,
        role,
        certificate_context,
        drift_marker: HopDriftMarker::None,
    }
}

/// The canonical ordered hops for an action.
fn canonical_hops(action: HighRiskAction) -> Vec<RouteHopSeed> {
    use CertificateContext::*;
    use HopLocality::*;
    use HopRole::*;
    match action {
        HighRiskAction::LocalModelExecution => vec![hop(LocalMachine, Origin, NoTlsLocal)],
        HighRiskAction::RemoteModelInference | HighRiskAction::WorkspaceDataExport => vec![
            hop(LocalMachine, Origin, LocalTrust),
            hop(LocalNetwork, Proxy, PinnedCertificate),
            hop(RemoteRegion, Target, PinnedCertificate),
        ],
        HighRiskAction::ProviderCredentialRotation
        | HighRiskAction::ControlPlaneSync
        | HighRiskAction::AdminPolicyPush => vec![
            hop(LocalMachine, Origin, LocalTrust),
            hop(ControlPlane, Target, ControlPlaneCertificate),
        ],
        HighRiskAction::OfflineModelAcquisition => vec![
            hop(LocalMachine, Origin, LocalTrust),
            hop(MirrorEdge, Mirror, MirrorCertificate),
            hop(LocalMachine, Target, NoTlsLocal),
        ],
        HighRiskAction::SupportBundleHandoff => vec![
            hop(LocalMachine, Origin, LocalTrust),
            hop(VendorEdge, Target, PinnedCertificate),
        ],
    }
}

/// The canonical approval state for an action: standing-policy actions are pre-authorized; the rest
/// carry a named approval.
fn canonical_approval_state(action: HighRiskAction) -> ApprovalState {
    match action {
        HighRiskAction::LocalModelExecution
        | HighRiskAction::ControlPlaneSync
        | HighRiskAction::OfflineModelAcquisition => ApprovalState::PreAuthorized,
        _ => ApprovalState::Approved,
    }
}

impl ActionInspectorSeed {
    /// The canonical (fully governed) seed for an action: within boundary, fully attributed route,
    /// live approval, every proof current.
    fn canonical(action: HighRiskAction) -> Self {
        Self {
            action,
            boundary_state: CapabilityBoundaryState::WithinBoundary,
            boundary_freshness: FreshnessState::Current,
            route_state: canonical_route_state(action),
            route_freshness: FreshnessState::Current,
            hops: canonical_hops(action),
            approval_state: canonical_approval_state(action),
            expiry_standing: ExpiryStanding::Active,
            expiry: ACTIVE_EXPIRY.to_owned(),
        }
    }
}

/// The canonical seeds: every action fully governed.
fn canonical_seeds() -> Vec<ActionInspectorSeed> {
    HighRiskAction::ALL
        .iter()
        .map(|a| ActionInspectorSeed::canonical(*a))
        .collect()
}

/// Replaces the seed for one action via the given mutator.
fn with_action(
    mut seeds: Vec<ActionInspectorSeed>,
    action: HighRiskAction,
    mutate: impl Fn(&mut ActionInspectorSeed),
) -> Vec<ActionInspectorSeed> {
    for seed in &mut seeds {
        if seed.action == action {
            mutate(seed);
        }
    }
    seeds
}

/// Assembles a packet from the given action seeds.
fn assemble_packet(
    packet_id: &str,
    report_label: &str,
    action_seeds: Vec<ActionInspectorSeed>,
) -> M5BoundaryInspector {
    M5BoundaryInspector::new(M5BoundaryInspectorInput {
        packet_id: packet_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        action_seeds,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical, all-governed boundary-inspector packet: every action within boundary, on a fully
/// attributed route, under a live approval.
pub fn seeded_m5_boundary_inspector() -> M5BoundaryInspector {
    assemble_packet(
        M5_BOUNDARY_INSPECTOR_PACKET_ID,
        "M5 boundary inspector",
        canonical_seeds(),
    )
}

/// Drill: one action's boundary sits at the boundary edge, so its inspector auto-narrows below fully
/// governed while every other action stays governed.
pub fn seeded_m5_boundary_inspector_boundary_narrowed() -> M5BoundaryInspector {
    let seeds = with_action(canonical_seeds(), BOUNDARY_DRILL_ACTION, |seed| {
        seed.boundary_state = CapabilityBoundaryState::AtBoundaryEdge;
    });
    assemble_packet(
        "m5-boundary-inspector:drill-boundary:0001",
        "M5 boundary inspector — boundary-edge drill",
        seeds,
    )
}

/// Drill: one action's route was served by a mirror that silently replaced the named target, so the
/// route timeline narrows that action's inspector.
pub fn seeded_m5_boundary_inspector_route_drift_narrowed() -> M5BoundaryInspector {
    let seeds = with_action(canonical_seeds(), ROUTE_DRIFT_ACTION, |seed| {
        seed.route_state = RouteHopState::MirroredRoute;
        if let Some(last) = seed.hops.last_mut() {
            last.drift_marker = HopDriftMarker::MirrorSubstitution;
        }
    });
    assemble_packet(
        "m5-boundary-inspector:drill-route-drift:0001",
        "M5 boundary inspector — route-drift drill",
        seeds,
    )
}

/// Drill: one action's route reached an unattributable hop, so the route timeline blocks that action's
/// inspector and holds Stable promotion.
pub fn seeded_m5_boundary_inspector_route_unattributed_blocked() -> M5BoundaryInspector {
    let seeds = with_action(canonical_seeds(), ROUTE_BLOCK_ACTION, |seed| {
        seed.route_state = RouteHopState::UnattributedRoute;
        if let Some(last) = seed.hops.last_mut() {
            last.drift_marker = HopDriftMarker::UnattributedHop;
        }
    });
    assemble_packet(
        "m5-boundary-inspector:drill-route-unattributed:0001",
        "M5 boundary inspector — unattributed-route drill",
        seeds,
    )
}

/// Drill: one action's approval has expired, so its approval ticket blocks that action's inspector and
/// holds Stable promotion.
pub fn seeded_m5_boundary_inspector_approval_expired_blocked() -> M5BoundaryInspector {
    let seeds = with_action(canonical_seeds(), APPROVAL_DRILL_ACTION, |seed| {
        seed.expiry_standing = ExpiryStanding::Expired;
        seed.expiry = PAST_EXPIRY.to_owned();
    });
    assemble_packet(
        "m5-boundary-inspector:drill-approval-expired:0001",
        "M5 boundary inspector — expired-approval drill",
        seeds,
    )
}
