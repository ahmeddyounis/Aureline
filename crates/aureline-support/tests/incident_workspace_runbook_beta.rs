//! Protected drills for beta incident-workspace runbook packets.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use aureline_support::incident_workspace::{
    current_incident_workspace_runbook_corpus, load_incident_workspace_runbook_packet,
    required_incident_workspace_drills, validate_incident_workspace_runbook_packet, ApprovalState,
    ConsoleHandoffReasonClass, DocsFreshnessState, IncidentActionOutcome,
    IncidentWorkspaceDrillClass, RunbookSourceClass, RunbookStepClass, RunbookStepState,
    INCIDENT_ACTION_LEDGER_SCHEMA_REF, INCIDENT_WORKSPACE_RUNBOOK_DOC_REF,
    INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_DIR, INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_MANIFEST_REF,
    INCIDENT_WORKSPACE_RUNBOOK_RECORD_KIND, INCIDENT_WORKSPACE_RUNBOOK_SCHEMA_VERSION,
    SUPPORT_RUNBOOK_PACKET_SCHEMA_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn corpus_validates_and_covers_required_drills() {
    let corpus = current_incident_workspace_runbook_corpus().expect("corpus parses");
    assert_eq!(corpus.entries.len(), 6);

    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");

    let mut covered = BTreeSet::new();
    for packet in corpus.packets() {
        assert_eq!(
            packet.schema_version,
            INCIDENT_WORKSPACE_RUNBOOK_SCHEMA_VERSION
        );
        assert_eq!(packet.record_kind, INCIDENT_WORKSPACE_RUNBOOK_RECORD_KIND);
        assert!(packet.export_reconstructs_action_lineage());
        covered.extend(packet.drill_classes.iter().copied());
    }
    for required in required_incident_workspace_drills() {
        assert!(
            covered.contains(required),
            "missing required drill {required:?}"
        );
    }
}

#[test]
fn fixtures_round_trip_from_disk() {
    let corpus = current_incident_workspace_runbook_corpus().expect("corpus parses");
    let root = repo_root();
    for entry in &corpus.entries {
        let yaml = std::fs::read_to_string(root.join(&entry.fixture_ref))
            .unwrap_or_else(|err| panic!("read {}: {err}", entry.fixture_ref));
        let packet = load_incident_workspace_runbook_packet(&yaml)
            .unwrap_or_else(|err| panic!("parse {}: {err}", entry.fixture_ref));
        assert_eq!(packet, entry.packet);
    }
}

#[test]
fn declared_docs_schema_and_fixture_paths_exist() {
    let root = repo_root();
    for path in [
        INCIDENT_ACTION_LEDGER_SCHEMA_REF,
        SUPPORT_RUNBOOK_PACKET_SCHEMA_REF,
        INCIDENT_WORKSPACE_RUNBOOK_DOC_REF,
        INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_MANIFEST_REF,
    ] {
        assert!(root.join(path).is_file(), "{path} missing");
    }
    assert!(
        root.join(INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_DIR).is_dir(),
        "{INCIDENT_WORKSPACE_RUNBOOK_FIXTURE_DIR} missing"
    );
}

#[test]
fn approved_mutating_step_requires_current_approval_target_and_evidence() {
    let corpus = current_incident_workspace_runbook_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .find(|packet| packet.covers_drill(IncidentWorkspaceDrillClass::ApprovedCurrentRunbook))
        .expect("approved runbook fixture")
        .clone();

    let step = packet
        .runbook_packet
        .steps
        .iter_mut()
        .find(|step| step.step_class == RunbookStepClass::Mitigate)
        .expect("mitigating step");
    step.approval_requirement.approval_ticket_ref = None;
    step.target_identity_ref = None;
    step.expected_evidence_outputs.clear();

    let violations = validate_incident_workspace_runbook_packet(&packet);
    assert!(violations
        .iter()
        .any(|v| { v.check_id == "packet.runbook.step.current_approval_required_for_mutation" }));
    assert!(violations
        .iter()
        .any(|v| { v.check_id == "packet.runbook.step.target_identity_required_for_mutation" }));
    assert!(violations
        .iter()
        .any(|v| { v.check_id == "packet.runbook.step.expected_evidence_outputs.empty" }));
}

#[test]
fn mutating_action_ledger_success_requires_current_approval() {
    let corpus = current_incident_workspace_runbook_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .find(|packet| packet.covers_drill(IncidentWorkspaceDrillClass::ApprovedCurrentRunbook))
        .expect("approved runbook fixture")
        .clone();

    let entry = packet
        .action_ledger
        .iter_mut()
        .find(|entry| entry.outcome == IncidentActionOutcome::Succeeded)
        .expect("successful mutation");
    entry.approval_state = ApprovalState::Expired;
    entry.approval_ticket_ref = None;
    entry.preview_hash_ref = None;

    let violations = validate_incident_workspace_runbook_packet(&packet);
    assert!(violations
        .iter()
        .any(|v| { v.check_id == "packet.action_ledger.current_approval_required_for_mutation" }));
    assert!(violations
        .iter()
        .any(|v| v.check_id == "packet.action_ledger.fail_closed_without_approval"));
}

#[test]
fn stale_runbook_and_blocked_approval_fixtures_fail_closed_without_violations() {
    let corpus = current_incident_workspace_runbook_corpus().expect("corpus parses");

    let stale = corpus
        .packets()
        .find(|packet| packet.covers_drill(IncidentWorkspaceDrillClass::StaleRunbookVersion))
        .expect("stale runbook fixture");
    assert_eq!(
        stale.runbook_packet.docs_freshness_state,
        DocsFreshnessState::StaleRequiresReview
    );
    assert!(stale
        .runbook_packet
        .steps
        .iter()
        .filter(|step| step.step_class.is_mutating())
        .all(|step| step.current_state == RunbookStepState::Blocked));

    let blocked_approval = corpus
        .packets()
        .find(|packet| packet.covers_drill(IncidentWorkspaceDrillClass::BlockedApproval))
        .expect("blocked approval fixture");
    assert!(blocked_approval
        .action_ledger
        .iter()
        .any(|entry| entry.outcome == IncidentActionOutcome::FailedClosed));

    let violations = corpus.validate();
    assert_eq!(violations, Vec::new(), "{violations:#?}");
}

#[test]
fn browser_only_vendor_docs_preserve_explicit_handoff_boundary() {
    let corpus = current_incident_workspace_runbook_corpus().expect("corpus parses");
    let packet = corpus
        .packets()
        .find(|packet| packet.covers_drill(IncidentWorkspaceDrillClass::BrowserOnlyVendorDocs))
        .expect("browser-only fixture");

    assert_eq!(
        packet.runbook_packet.source_class,
        RunbookSourceClass::BrowserOnlyVendorDocs
    );
    assert_eq!(
        packet.runbook_packet.docs_freshness_state,
        DocsFreshnessState::BrowserOnlyReference
    );
    assert!(packet.console_handoffs.iter().any(|handoff| {
        handoff.reason_class == ConsoleHandoffReasonClass::BrowserOnlyVendorDocs
            && handoff.external_control_plane_authoritative
            && !handoff.in_product_mutation_supported
            && handoff.exportable_as_metadata
    }));
    assert!(packet
        .runbook_packet
        .steps
        .iter()
        .filter(|step| step.step_class.is_mutating())
        .all(|step| step.current_state == RunbookStepState::HandoffRequired));
}

#[test]
fn handoff_validator_refuses_in_product_parity_overclaim() {
    let corpus = current_incident_workspace_runbook_corpus().expect("corpus parses");
    let mut packet = corpus
        .packets()
        .find(|packet| packet.covers_drill(IncidentWorkspaceDrillClass::BrowserOnlyVendorDocs))
        .expect("browser-only fixture")
        .clone();
    packet.console_handoffs[0].in_product_mutation_supported = true;

    let violations = validate_incident_workspace_runbook_packet(&packet);
    assert!(violations
        .iter()
        .any(|v| v.check_id == "packet.console_handoffs.overclaims_in_product_parity"));
}

#[test]
fn redacted_export_fixture_declares_omissions_and_excludes_private_material() {
    let corpus = current_incident_workspace_runbook_corpus().expect("corpus parses");
    let packet = corpus
        .packets()
        .find(|packet| packet.covers_drill(IncidentWorkspaceDrillClass::RedactedExportBundle))
        .expect("redacted export fixture");

    assert!(!packet.export_bundle.omitted_evidence_refs.is_empty());
    assert!(packet.export_bundle.raw_private_material_excluded);
    assert!(packet.export_bundle.secrets_excluded);
    assert!(packet.privacy_baseline.raw_console_sessions_excluded);
    assert!(packet
        .evidence_timeline
        .iter()
        .filter(|row| packet
            .export_bundle
            .omitted_evidence_refs
            .contains(&row.entry_id))
        .all(|row| row.omission_declared));
}
