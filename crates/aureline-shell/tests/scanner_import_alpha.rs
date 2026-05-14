//! Protected checks for scanner import alpha truth and export projections.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_language::{DiagnosticBus, DiagnosticBusSnapshotRequest, DiagnosticSurfaceClass};
use aureline_shell::diagnostics::imported::{
    materialize_sarif_import_session, ScannerDeltaCompatibilityClass, ScannerFindingDeltaState,
    ScannerFindingTruthClass, ScannerImportRequest, ScannerLocalConfirmationStateClass,
    IMPORTED_DIAGNOSTICS_SUPPORT_ITEM_ID,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("repo root")
        .join("fixtures")
        .join("quality")
        .join("sarif_alpha")
}

fn load_fixture(name: &str) -> String {
    let path = fixture_dir().join(name);
    fs::read_to_string(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()))
}

fn load_request() -> ScannerImportRequest {
    serde_json::from_str(&load_fixture("import_request.json")).expect("parse import request")
}

#[test]
fn scanner_import_preserves_imported_truth_delta_debt_and_support_export() {
    let request = load_request();
    let session =
        materialize_sarif_import_session(request, &load_fixture("scanner_output.sarif.json"))
            .expect("scanner import session materializes");

    assert_eq!(session.run_descriptors.len(), 1);
    assert_eq!(session.run_descriptors[0].result_count, 5);
    assert_eq!(
        session.raw_payload_refs,
        vec!["raw_payload:scanner_payload:security_policy_alpha"]
    );
    assert!(session.findings.iter().all(|finding| finding.read_only));
    assert_eq!(session.findings.len(), 5);
    assert_eq!(
        session
            .findings
            .iter()
            .filter(|finding| finding.truth_class == ScannerFindingTruthClass::LocallyConfirmed)
            .count(),
        1
    );
    assert!(session.findings.iter().any(|finding| {
        finding.local_confirmation_state_class == ScannerLocalConfirmationStateClass::Confirmed
            && finding.local_confirmation_ref.as_deref()
                == Some("local_confirmation:secret:persisting")
    }));

    let delta_states = session
        .delta_packet
        .finding_deltas
        .iter()
        .map(|delta| delta.delta_state_class)
        .collect::<BTreeSet<_>>();
    assert_eq!(
        delta_states,
        BTreeSet::from([
            ScannerFindingDeltaState::New,
            ScannerFindingDeltaState::Resolved,
            ScannerFindingDeltaState::Persisting,
            ScannerFindingDeltaState::Suppressed,
            ScannerFindingDeltaState::Waived,
            ScannerFindingDeltaState::Unmapped,
        ])
    );
    assert_eq!(session.delta_packet.delta_counts.new_count, 1);
    assert_eq!(session.delta_packet.delta_counts.resolved_count, 1);
    assert_eq!(session.delta_packet.delta_counts.persisting_count, 1);
    assert_eq!(session.delta_packet.delta_counts.suppressed_count, 1);
    assert_eq!(session.delta_packet.delta_counts.waived_count, 1);
    assert_eq!(session.delta_packet.delta_counts.unmapped_count, 1);
    assert_eq!(
        session.delta_packet.compatibility_class,
        ScannerDeltaCompatibilityClass::BlockedAnchorMappingUncertain
    );

    assert_eq!(
        session
            .suppression_baseline_register
            .release_visible_debt_count,
        4
    );
    assert_eq!(session.review_packet.imported_finding_count, 5);
    assert_eq!(session.review_packet.locally_confirmed_finding_count, 1);
    assert_eq!(session.review_packet.local_confirmation_actions.len(), 1);
    assert!(session.review_packet.local_confirmation_actions[0].required_before_mutation);
    assert!(session.review_packet.quality_action_refs.is_empty());

    let mut bus = DiagnosticBus::new();
    session.publish_to_diagnostic_bus(&mut bus);
    let snapshot = bus.snapshot(DiagnosticBusSnapshotRequest {
        snapshot_id: "diagnostic_bus_snapshot:scanner_import_alpha".into(),
        workspace_id: session.workspace_id.clone(),
        collection_id: session.collection_id.clone(),
        captured_at: "2026-05-14T17:46:00Z".into(),
    });
    assert_eq!(snapshot.aggregate_counts.total_count, 5);
    assert_eq!(snapshot.aggregate_counts.imported_count, 5);
    assert_eq!(snapshot.aggregate_counts.local_count, 0);
    assert_eq!(snapshot.aggregate_counts.partial_count, 1);
    assert!(snapshot.requires_degraded_disclosure());
    let inline_projection =
        snapshot.surface_projection(DiagnosticSurfaceClass::EditorInline, "2026-05-14T17:46:00Z");
    assert_eq!(inline_projection.visible_count, 4);

    let problems = session.problems_projection(Some(&snapshot));
    assert_eq!(problems.imported_count, 5);
    assert_eq!(problems.locally_confirmed_count, 1);
    assert_eq!(problems.read_only_count, 5);
    assert!(problems.rows.iter().any(|row| {
        row.delta_state_class == ScannerFindingDeltaState::Unmapped
            && row.local_confirmation_state_class
                == ScannerLocalConfirmationStateClass::BlockedByUnmappedAnchor
    }));

    let support_export = session.support_export(Some(&snapshot));
    assert_eq!(
        support_export.support_pack_item_id,
        IMPORTED_DIAGNOSTICS_SUPPORT_ITEM_ID
    );
    assert_eq!(support_export.imported_finding_count, 5);
    assert_eq!(support_export.locally_confirmed_count, 1);
    assert_eq!(support_export.read_only_count, 5);
    assert_eq!(support_export.release_visible_debt_count, 4);
    assert!(support_export.raw_private_material_excluded);
    assert_eq!(
        support_export.raw_payload_refs,
        vec!["raw_payload:scanner_payload:security_policy_alpha"]
    );

    let serialized = serde_json::to_string(&support_export).expect("support export serializes");
    assert!(!serialized.contains("src/payments"));
    assert!(!serialized.contains("package-lock"));
    assert!(!serialized.contains("payment_token_fixture"));
    assert!(!serialized.contains("Hardcoded token pattern"));
}
