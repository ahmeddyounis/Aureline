//! Protected tests for alpha runtime fault-domain and supervisor health events.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::repair::current_alpha_repair_seed_cases;
use aureline_support::runtime_health_alpha::{
    current_alpha_fault_domain_drill_corpus, current_alpha_fault_domain_taxonomy,
    current_alpha_supervisor_health_events, RuntimeFaultDomainTaxonomy,
    RuntimeFaultDomainViolation, SupervisorHealthEventCatalog, CURRENT_ALPHA_DRILL_MANIFEST_PATH,
    CURRENT_ALPHA_HEALTH_EVENTS_PATH, CURRENT_ALPHA_TAXONOMY_PATH,
    RUNTIME_FAULT_DOMAIN_SUPPORT_PACKET_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|path| path.parent())
        .expect("derive repo root")
        .to_path_buf()
}

fn load_taxonomy_and_events() -> (RuntimeFaultDomainTaxonomy, SupervisorHealthEventCatalog) {
    let taxonomy =
        current_alpha_fault_domain_taxonomy().expect("runtime fault-domain taxonomy parses");
    let events = current_alpha_supervisor_health_events().expect("supervisor health events parse");
    (taxonomy, events)
}

fn assert_no_violations(violations: Vec<RuntimeFaultDomainViolation>) {
    assert_eq!(violations, Vec::new());
}

#[test]
fn runtime_fault_domain_taxonomy_validates_lane_budget_and_projection_coverage() {
    let (taxonomy, events) = load_taxonomy_and_events();
    assert_no_violations(taxonomy.validate(&events));

    assert_eq!(taxonomy.alpha_runtime_lanes.len(), 12);
    assert!(taxonomy
        .alpha_runtime_lanes
        .iter()
        .all(|lane| !lane.fault_domain_id.is_empty()));
    assert!(taxonomy
        .alpha_runtime_lanes
        .iter()
        .all(|lane| lane.strike_budget.window_seconds > 0));
    assert!(taxonomy.alpha_runtime_lanes.iter().all(|lane| lane
        .projection_contract
        .required_projection_fields
        .iter()
        .any(|field| field == "forensic_packet_ref")));

    let fail_closed = taxonomy
        .alpha_runtime_lanes
        .iter()
        .map(|lane| lane.visible_state_contract.fail_closed_state_class.as_str())
        .collect::<BTreeSet<_>>();
    assert!(fail_closed.contains("quarantined"));
    assert!(fail_closed.contains("disabled"));
    assert!(fail_closed.contains("offline"));
    assert!(fail_closed.contains("lost"));
    assert!(fail_closed.contains("degraded"));
}

#[test]
fn supervisor_health_events_cover_required_transitions_and_consumers() {
    let (taxonomy, events) = load_taxonomy_and_events();
    assert_no_violations(events.validate_against_taxonomy(&taxonomy));

    let transitions = events
        .event_classes
        .iter()
        .map(|event| event.transition_class.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        transitions,
        ["degrade", "quarantine", "recover", "restart", "start"]
            .into_iter()
            .collect::<BTreeSet<_>>()
    );

    for event in &events.event_classes {
        assert!(event.support_bundle_projection.consumable);
        assert!(event.incident_packet_projection.consumable);
        assert!(event.ui_state_copy_projection.consumable);
        assert_eq!(
            event.support_bundle_projection.item_id,
            "support.item.runtime_fault_domains_alpha"
        );
    }
}

#[test]
fn fault_domain_drills_exercise_fail_closed_and_recovery_states() {
    let (taxonomy, events) = load_taxonomy_and_events();
    let corpus =
        current_alpha_fault_domain_drill_corpus().expect("runtime fault-domain drills parse");
    assert_no_violations(corpus.validate(&taxonomy, &events));

    assert_eq!(corpus.entries.len(), 3);
    assert!(corpus.entries.iter().any(|entry| {
        entry.case.budget.budget_exhausted
            && entry
                .case
                .health_events
                .iter()
                .any(|event| event.transition_class == "quarantine")
            && !entry.case.budget.automatic_restart_admitted_after_budget
    }));
    assert!(corpus.entries.iter().any(|entry| entry
        .case
        .health_events
        .iter()
        .any(|event| event.transition_class == "recover" && event.state_after == "ready")));
    assert!(corpus
        .entries
        .iter()
        .all(|entry| entry.case.repair_handoff.preview_required_before_mutation));
}

#[test]
fn support_packet_projects_runtime_taxonomy_without_raw_payloads() {
    let (taxonomy, events) = load_taxonomy_and_events();
    let corpus =
        current_alpha_fault_domain_drill_corpus().expect("runtime fault-domain drills parse");
    assert_no_violations(taxonomy.validate(&events));
    assert_no_violations(corpus.validate(&taxonomy, &events));

    let packet = taxonomy.support_packet(
        "support.packet.runtime_fault_domains.alpha",
        "2026-05-14T08:45:00Z",
        &corpus,
    );

    assert_eq!(
        packet.record_kind,
        RUNTIME_FAULT_DOMAIN_SUPPORT_PACKET_RECORD_KIND
    );
    assert!(packet.is_export_safe());
    assert_eq!(packet.rows.len(), taxonomy.alpha_runtime_lanes.len());
    assert!(packet
        .rows
        .iter()
        .any(|row| row.lane_id == "workspace_knowledge_lane"
            && row.repair_transaction_ref.as_deref()
                == Some("repair_transaction:disposable_state_rebuild.cache_index_repair")
            && !row.drill_case_refs.is_empty()));
    assert!(packet
        .rows
        .iter()
        .any(|row| row.lane_id == "remote_connector_lane"
            && row.fail_closed_state_class == "offline"
            && !row.drill_case_refs.is_empty()));
}

#[test]
fn runtime_repair_handoffs_reference_checked_in_repair_transactions() {
    let (taxonomy, _) = load_taxonomy_and_events();
    let corpus =
        current_alpha_fault_domain_drill_corpus().expect("runtime fault-domain drills parse");
    let repair_ids = current_alpha_repair_seed_cases()
        .expect("repair seed cases parse")
        .into_iter()
        .map(|seed| seed.repair_transaction_id)
        .collect::<BTreeSet<_>>();

    for repair_ref in taxonomy
        .alpha_runtime_lanes
        .iter()
        .filter_map(|lane| lane.repair_handoff.repair_transaction_ref.as_ref())
    {
        assert!(
            repair_ids.contains(repair_ref),
            "taxonomy repair ref {repair_ref} must exist in repair seeds"
        );
    }
    for repair_ref in corpus
        .entries
        .iter()
        .map(|entry| entry.case.repair_handoff.repair_transaction_ref.as_str())
    {
        assert!(
            repair_ids.contains(repair_ref),
            "drill repair ref {repair_ref} must exist in repair seeds"
        );
    }
}

#[test]
fn checked_in_artifacts_and_docs_exist_for_reviewers() {
    let root = repo_root();
    for rel in [
        CURRENT_ALPHA_TAXONOMY_PATH,
        CURRENT_ALPHA_HEALTH_EVENTS_PATH,
        CURRENT_ALPHA_DRILL_MANIFEST_PATH,
        "docs/runtime/restart_budget_alpha.md",
        "docs/support/repair_preview_alpha.md",
        "schemas/support/repair_transaction.schema.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}
