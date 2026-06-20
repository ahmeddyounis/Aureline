//! Inline unit tests for the typed M5 contract-health register.

use super::*;

fn register() -> M5ContractHealthRegister {
    current_m5_contract_health_register().expect("checked-in register parses")
}

#[test]
fn checked_in_register_parses_and_validates() {
    let r = register();
    assert_eq!(r.schema_version, M5_CONTRACT_HEALTH_SCHEMA_VERSION);
    assert_eq!(r.record_kind, M5_CONTRACT_HEALTH_RECORD_KIND);
    assert_eq!(r.register_id, M5_CONTRACT_HEALTH_REGISTER_ID);
    let violations = r.validate();
    assert!(
        violations.is_empty(),
        "register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_recomputes_from_rows() {
    let r = register();
    assert_eq!(r.summary, r.computed_summary());
    assert!(r.summary.total_families > 0);
    assert_eq!(
        r.summary.healthy_families + r.summary.narrowed_families + r.summary.blocked_families,
        r.rows.len(),
        "every family is healthy, narrowed, or blocked"
    );
}

#[test]
fn every_row_has_one_gate_per_kind() {
    let r = register();
    for row in &r.rows {
        let kinds: Vec<GateKind> = row.gates.iter().map(|g| g.gate_kind).collect();
        assert_eq!(
            kinds,
            GateKind::ALL.to_vec(),
            "{} must carry one gate per kind in order",
            row.family_id
        );
    }
}

#[test]
fn health_state_and_decision_follow_the_gates() {
    let r = register();
    for row in &r.rows {
        assert_eq!(
            row.health_state,
            row.computed_health(),
            "{} health state must follow its gates",
            row.family_id
        );
        assert_eq!(
            row.blocker.decision,
            row.computed_decision(),
            "{} blocker decision must follow its health",
            row.family_id
        );
    }
}

#[test]
fn release_blocking_family_with_a_missing_contract_is_held() {
    // The acceptance anchor: a release-blocking family whose required contract
    // gate fails holds promotion and is not mirror-publishable.
    let r = register();
    let te = r
        .row("task_event_envelope")
        .expect("task_event_envelope present");
    assert!(te.release_blocking);
    assert!(te.narrowed, "task_event_envelope narrows in the matrix");
    assert_eq!(te.lifecycle_label, LifecycleLabel::Beta);
    assert_eq!(te.health_state, HealthState::Blocked);
    assert_eq!(te.blocker.decision, BlockerDecision::Hold);
    assert!(te.gates.iter().any(Gate::is_required_failure));
    let failing = te
        .gates
        .iter()
        .find(|g| g.is_required_failure())
        .expect("a failing gate");
    assert_eq!(failing.gate_kind, GateKind::CompatibilityReport);
    assert_eq!(failing.freshness, FreshnessState::Missing);
    // Mirror/offline publishability follows the gate outputs.
    assert_eq!(
        te.graph_linkage.mirror_parity,
        MirrorParityState::Unpublished
    );
    assert!(!te.graph_linkage.offline_verifiable);
    // The top-level decision holds promotion on the same signal.
    assert!(r.holds_promotion());
    assert_eq!(r.blockers.blocking_family_ids, vec!["task_event_envelope"]);
    assert_eq!(
        r.blockers.blocking_gate_kinds,
        vec![GateKind::CompatibilityReport]
    );
}

#[test]
fn healthy_family_resolves_and_is_mirror_publishable() {
    let r = register();
    let (label, health, decision) = r
        .resolve_health("command_descriptors")
        .expect("command_descriptors resolves");
    assert_eq!(label, LifecycleLabel::Stable);
    assert_eq!(health, HealthState::Healthy);
    assert_eq!(decision, BlockerDecision::Clear);
    let cd = r.row("command_descriptors").unwrap();
    assert!(cd.gates.iter().all(|g| g.outcome == GateOutcome::Pass));
    assert!(cd.graph_linkage.offline_verifiable);
    assert!(r.resolve_health("not_a_family").is_none());
}

#[test]
fn graph_linkage_binds_build_identity_and_package_version() {
    let r = register();
    assert!(r
        .build_identity
        .build_identity_ref
        .ends_with("build_identity.json"));
    assert!(!r.build_identity.toolchain_channel.is_empty());
    for row in &r.rows {
        assert!(!row.graph_linkage.release_packet_ref.is_empty());
        assert!(row
            .graph_linkage
            .artifact_graph_node_ref
            .contains(&row.family_id));
        assert_eq!(
            row.graph_linkage.build_identity_ref,
            r.build_identity.build_identity_ref
        );
        assert!(row.package_identity.package_version >= 1);
        assert_eq!(
            row.package_identity.package_kind,
            row.package_identity.identity_kind
        );
    }
}

#[test]
fn support_export_projection_covers_every_family() {
    let r = register();
    let projection = r.support_export_projection();
    assert_eq!(projection.register_id, r.register_id);
    assert_eq!(projection.decision, r.blockers.decision);
    assert_eq!(projection.rows.len(), r.rows.len());
    for row in &projection.rows {
        let src = r.row(&row.family_id).expect("row family is in the model");
        assert_eq!(row.lifecycle_label, src.lifecycle_label);
        assert_eq!(row.health_state, src.health_state);
        assert_eq!(row.decision, src.blocker.decision);
    }
}

#[test]
fn duplicate_family_id_is_rejected() {
    let mut r = register();
    let dup = r.rows[0].clone();
    r.rows.push(dup);
    r.summary = r.computed_summary();
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.duplicate_family_id"));
}

#[test]
fn missing_gate_is_rejected() {
    let mut r = register();
    r.rows[0].gates.pop();
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.gate_coverage"));
}

#[test]
fn lying_blocker_decision_is_rejected() {
    let mut r = register();
    let idx = r
        .rows
        .iter()
        .position(|row| row.health_state == HealthState::Blocked)
        .expect("a blocked row");
    r.rows[idx].blocker.decision = BlockerDecision::Clear;
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "rows.blocker_decision"));
}

#[test]
fn summary_drift_is_rejected() {
    let mut r = register();
    r.summary.total_families += 1;
    assert!(r
        .validate()
        .iter()
        .any(|v| v.check_id == "summary.count_mismatch"));
}
