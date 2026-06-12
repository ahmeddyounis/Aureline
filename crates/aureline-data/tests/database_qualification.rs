//! Fixture replay for database qualification truth.

use std::path::{Path, PathBuf};

use aureline_data::{
    current_database_qualification, DatabaseConnectionClass, DatabaseQualificationLabel,
    DatabaseQualificationPacket, DatabaseQualificationViolation, DatabaseRedactionMode,
    DatabaseResultScope, DatabaseStatementSafetyClass, DatabaseWritePosture,
    DATABASE_QUALIFICATION_RECORD_KIND, DATABASE_QUALIFICATION_SCHEMA_VERSION,
};
use serde::Deserialize;

fn packet() -> DatabaseQualificationPacket {
    current_database_qualification().expect("checked-in packet parses")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[derive(Debug, Deserialize)]
struct FixtureManifest {
    schema_version: u32,
    cases: Vec<FixtureCase>,
}

#[derive(Debug, Deserialize)]
struct FixtureCase {
    case_id: String,
    case_kind: String,
    #[serde(default)]
    expected_connection_class: Option<DatabaseConnectionClass>,
    #[serde(default)]
    expected_write_posture: Option<DatabaseWritePosture>,
    #[serde(default)]
    expected_projected_everywhere: Option<bool>,
    #[serde(default)]
    expected_statement_class: Option<DatabaseStatementSafetyClass>,
    #[serde(default)]
    expected_blocked: Option<bool>,
    #[serde(default)]
    expected_review_or_step_up_required: Option<bool>,
    #[serde(default)]
    expected_result_scope: Option<DatabaseResultScope>,
    #[serde(default)]
    expected_redaction_mode: Option<DatabaseRedactionMode>,
    #[serde(default)]
    expected_virtualization: Option<bool>,
    #[serde(default)]
    expected_export_truth: Option<bool>,
    #[serde(default)]
    expected_safe_history: Option<bool>,
    #[serde(default)]
    expected_plan_mode: Option<aureline_data::DatabaseExplainPlanMode>,
    #[serde(default)]
    expected_implies_execution: Option<bool>,
    #[serde(default)]
    expected_stale_import_labeled: Option<bool>,
    #[serde(default)]
    expected_destination_class: Option<String>,
    #[serde(default)]
    expected_truth_preserved: Option<bool>,
}

#[test]
fn checked_in_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, DATABASE_QUALIFICATION_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, DATABASE_QUALIFICATION_RECORD_KIND);
    assert_eq!(packet.summary, packet.computed_summary());
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "checked-in database qualification packet must validate cleanly: {violations:#?}"
    );
}

#[test]
fn packet_projects_m5_secret_boundary_states() {
    let states = packet().secret_boundary_states();
    assert_eq!(states.len(), 2);
    assert_eq!(
        states[0].matrix_row_id,
        "m5.secret.database.connection_picker"
    );
    assert_eq!(
        states[1].matrix_row_id,
        "m5.secret.database.query_history_portability"
    );
    assert_eq!(
        states[0]
            .consumer_identity_receipt
            .consumer_identity
            .as_str(),
        "database_connector"
    );
    assert!(!states[0].repairable_states.is_empty());
    assert!(!states[0]
        .projection_mode_audit
        .available_controls
        .is_empty());
    assert!(!states[0].export_safety_banner.raw_secret_values_included);
}

#[test]
fn stable_surfaces_have_complete_proof_and_guards() {
    let packet = packet();
    let stable_surfaces: Vec<_> = packet
        .surfaces
        .iter()
        .filter(|surface| surface.displayed_label == DatabaseQualificationLabel::Stable)
        .collect();
    assert_eq!(stable_surfaces.len(), 5);

    for surface in stable_surfaces {
        assert!(
            surface.qualification_packet.is_some(),
            "{} must cite proof",
            surface.surface_id
        );
        assert!(
            surface.guards.all_visible(),
            "{} must expose every connection, safety, transaction, result, and export guard",
            surface.surface_id
        );
        assert!(
            surface.downgrade_if_missing,
            "{} must narrow rather than inherit stable language",
            surface.surface_id
        );
    }

    assert!(packet.surfaces.iter().any(|surface| {
        surface.surface_id == "surface:data.live_sql_execution_workspace"
            && surface.displayed_label == DatabaseQualificationLabel::Preview
            && surface.qualification_packet.is_none()
    }));
    assert!(packet.surfaces.iter().any(|surface| {
        surface.surface_id == "surface:data.direct_row_mutation"
            && surface.displayed_label == DatabaseQualificationLabel::InspectOnly
            && surface.qualification_packet.is_none()
    }));
}

#[test]
fn fixture_manifest_replays_expected_acceptance_cases() {
    let fixture_path =
        repo_root().join("fixtures/data/m4/database-statement-safety-and-result-grid/cases.json");
    let payload = std::fs::read_to_string(&fixture_path)
        .unwrap_or_else(|err| panic!("read {}: {err}", fixture_path.display()));
    let manifest: FixtureManifest =
        serde_json::from_str(&payload).expect("fixture manifest parses");
    assert_eq!(manifest.schema_version, 1);
    assert!(!manifest.cases.is_empty());

    let packet = packet();
    for case in manifest.cases {
        match case.case_kind.as_str() {
            "connection_projection" => {
                let row = packet
                    .connection_corpus
                    .iter()
                    .find(|row| row.case_id == case.case_id)
                    .unwrap_or_else(|| panic!("missing connection case {}", case.case_id));
                assert_eq!(
                    Some(row.connection_class),
                    case.expected_connection_class,
                    "{} connection class",
                    row.case_id
                );
                assert_eq!(
                    Some(row.write_posture),
                    case.expected_write_posture,
                    "{} write posture",
                    row.case_id
                );
                assert_eq!(
                    Some(row.projected_everywhere()),
                    case.expected_projected_everywhere,
                    "{} projection",
                    row.case_id
                );
            }
            "statement_safety" => {
                let row = packet
                    .statement_labs
                    .iter()
                    .find(|row| row.case_id == case.case_id)
                    .unwrap_or_else(|| panic!("missing statement case {}", case.case_id));
                assert_eq!(Some(row.statement_class), case.expected_statement_class);
                assert_eq!(Some(row.blocked), case.expected_blocked);
                assert_eq!(
                    Some(row.review_or_step_up_required),
                    case.expected_review_or_step_up_required
                );
            }
            "result_grid" => {
                let row = packet
                    .result_grid_labs
                    .iter()
                    .find(|row| row.case_id == case.case_id)
                    .unwrap_or_else(|| panic!("missing result-grid case {}", case.case_id));
                assert_eq!(Some(row.result_scope), case.expected_result_scope);
                assert_eq!(Some(row.redaction_mode), case.expected_redaction_mode);
                assert_eq!(Some(row.virtualization), case.expected_virtualization);
                assert_eq!(
                    Some(row.copy_export_preserves_scope_format_redaction),
                    case.expected_export_truth
                );
            }
            "query_history" => {
                let row = packet
                    .query_history_labs
                    .iter()
                    .find(|row| row.case_id == case.case_id)
                    .unwrap_or_else(|| panic!("missing query-history case {}", case.case_id));
                assert_eq!(
                    Some(
                        row.local_first
                            && row.bounded
                            && row.redactable
                            && row.metadata_first
                            && row.no_raw_secret_statement_or_payload
                    ),
                    case.expected_safe_history
                );
            }
            "explain_plan" => {
                let row = packet
                    .explain_plan_labs
                    .iter()
                    .find(|row| row.case_id == case.case_id)
                    .unwrap_or_else(|| panic!("missing explain-plan case {}", case.case_id));
                assert_eq!(Some(row.plan_mode), case.expected_plan_mode);
                if let Some(expected) = case.expected_implies_execution {
                    assert_eq!(row.implies_execution, expected);
                }
                if let Some(expected) = case.expected_stale_import_labeled {
                    assert_eq!(row.stale_import_labeled, expected);
                }
            }
            "handoff" => {
                let row = packet
                    .handoff_labs
                    .iter()
                    .find(|row| row.case_id == case.case_id)
                    .unwrap_or_else(|| panic!("missing handoff case {}", case.case_id));
                assert_eq!(
                    Some(row.destination_class.clone()),
                    case.expected_destination_class
                );
                assert_eq!(
                    Some(
                        row.source_refs_preserved
                            && row.row_column_scope_preserved
                            && row.type_fidelity_notes_preserved
                            && row.freshness_preserved
                            && row.share_local_restrictions_preserved
                    ),
                    case.expected_truth_preserved
                );
            }
            other => panic!("unknown fixture case kind {other}"),
        }
    }
}

#[test]
fn validator_catches_overclaims_and_unsafe_lanes() {
    let mut missing_guard = packet();
    let stable_surface = missing_guard
        .surfaces
        .iter_mut()
        .find(|surface| surface.surface_id == "surface:data.result_grid_export_contract")
        .expect("result-grid surface exists");
    stable_surface.guards.result_scope_visible = false;
    assert!(missing_guard.validate().iter().any(|violation| matches!(
        violation,
        DatabaseQualificationViolation::StableSurfaceMissingGuard { surface_id }
            if surface_id == "surface:data.result_grid_export_contract"
    )));

    let mut unsafe_statement = packet();
    let statement = unsafe_statement
        .statement_labs
        .iter_mut()
        .find(|row| row.case_id == "dml_update_requires_step_up")
        .expect("DML case exists");
    statement.review_or_step_up_required = false;
    assert!(unsafe_statement.validate().iter().any(|violation| matches!(
        violation,
        DatabaseQualificationViolation::UnsafeStatementAdmitted { case_id }
            if case_id == "dml_update_requires_step_up"
    )));

    let mut bad_plan = packet();
    let plan = bad_plan
        .explain_plan_labs
        .iter_mut()
        .find(|row| row.case_id == "estimated_plan_no_execution")
        .expect("estimated plan case exists");
    plan.implies_execution = true;
    assert!(bad_plan.validate().iter().any(|violation| matches!(
        violation,
        DatabaseQualificationViolation::EstimatedPlanImpliesExecution { case_id }
            if case_id == "estimated_plan_no_execution"
    )));
}
