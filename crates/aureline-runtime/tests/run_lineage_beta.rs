//! End-to-end coverage for run lineage, rerun review, and interruption truth.

use std::path::{Path, PathBuf};

use aureline_runtime::{
    seeded_run_history_support_export, DurableJobRow, RerunReviewDriftField,
    RunArtifactViewerClass, RunCurrentRelationshipClass, RunFreshnessClass,
    RunHistorySupportExport, RunInterruptionKind, RunLineageSeededScenario,
    RUN_HISTORY_SUPPORT_EXPORT_RECORD_KIND, RUN_LINEAGE_SCHEMA_VERSION,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("runtime")
        .join("m3")
        .join("run_lineage_and_interruptions")
}

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    #[allow(dead_code)]
    case_id: String,
    scenario: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    freshness: RunFreshnessClass,
    current_relationship: RunCurrentRelationshipClass,
    interruption: Option<RunInterruptionKind>,
    requires_review_before_dispatch: bool,
    required_drift_fields: Vec<RerunReviewDriftField>,
    required_continuity_markers: Vec<String>,
}

fn read_fixture(name: &str) -> CaseFixture {
    let path = fixture_root().join(name);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

fn scenario_files() -> [&'static str; 4] {
    [
        "current_local_passed_sleep_resume.json",
        "remote_disconnect_current_context_review.json",
        "auth_expiry_stale_evidence.json",
        "stale_import_manual_replay.json",
    ]
}

fn seeded_export() -> RunHistorySupportExport {
    seeded_run_history_support_export("run-history-support:integration", "2026-05-18T16:45:00Z")
}

#[test]
fn fixtures_replay_through_seeded_run_history_builder() {
    let export = seeded_export();
    assert_eq!(export.record_kind, RUN_HISTORY_SUPPORT_EXPORT_RECORD_KIND);
    assert_eq!(export.schema_version, RUN_LINEAGE_SCHEMA_VERSION);

    for fixture_name in scenario_files() {
        let fixture = read_fixture(fixture_name);
        assert_eq!(fixture.record_kind, "rerun_review_case");
        assert_eq!(fixture.schema_version, RUN_LINEAGE_SCHEMA_VERSION);

        let run_id = format!("run:lineage:{}", fixture.scenario);
        let summary = export
            .run_summaries
            .iter()
            .find(|summary| summary.run_id == run_id)
            .unwrap_or_else(|| panic!("missing summary for {}", fixture.scenario));
        assert_eq!(summary.freshness, fixture.expect.freshness);
        assert_eq!(
            summary.current_relationship,
            fixture.expect.current_relationship
        );
        assert_eq!(summary.interruption_kind, fixture.expect.interruption);
        assert_eq!(
            summary.interruption_token.as_deref(),
            fixture.expect.interruption.map(RunInterruptionKind::as_str)
        );
        assert_eq!(summary.context.workspace_id, "ws-run-lineage-beta");
        assert!(!summary.context.execution_context_ref.is_empty());
        assert!(!summary.context.canonical_target_id.is_empty());
        assert!(!summary.context.toolchain_id.is_empty());
        assert!(!summary.build_identity.commit_ref.is_empty());

        let job = export
            .durable_job_rows
            .iter()
            .find(|row| row.run_id == summary.run_id)
            .unwrap_or_else(|| panic!("missing durable job for {}", summary.run_id));
        assert_eq!(job.artifact_count, summary.artifact_count);
        assert!(job.durable_after_surface_close);
        assert!(job.raw_private_material_excluded);
        assert_eq!(job.rerun_exact_action.action_token, "rerun_exactly");
        assert_eq!(
            job.rerun_current_context_action.action_token,
            "rerun_with_current_context"
        );
        for marker in &fixture.expect.required_continuity_markers {
            assert!(
                job.continuity_marker_tokens.contains(marker),
                "{} missing continuity marker {}",
                fixture.scenario,
                marker
            );
        }

        let maybe_review = export
            .rerun_reviews
            .iter()
            .find(|review| review.source_run_id == summary.run_id);
        if fixture.expect.requires_review_before_dispatch {
            let review = maybe_review
                .unwrap_or_else(|| panic!("missing rerun review for {}", fixture.scenario));
            assert!(review.distinguishes_exact_and_current_modes());
            assert!(review.requires_review_before_dispatch);
            assert!(review.old_evidence_preserved);
            for field in &fixture.expect.required_drift_fields {
                assert!(
                    review.drift_rows.iter().any(|row| row.field == *field),
                    "{} missing drift field {:?}; got {:?}",
                    fixture.scenario,
                    field,
                    review.drift_field_tokens
                );
            }
        } else {
            assert!(
                maybe_review.is_none(),
                "{} should not require rerun review",
                fixture.scenario
            );
        }
    }
}

#[test]
fn support_export_names_the_full_interruption_taxonomy() {
    let export = seeded_export();
    for kind in RunInterruptionKind::ALL {
        assert!(
            export
                .interruption_taxonomy_tokens
                .contains(&kind.as_str().to_owned()),
            "taxonomy missing {}",
            kind.as_str()
        );
    }

    let plaintext = export.render_plaintext();
    for token in [
        "remote_disconnect",
        "auth_expiry",
        "manual_replay_requirement",
        "thermal_pause",
        "policy_block",
        "process_crash",
        "lost_source_map",
        "truncated_log",
        "stale_import",
        "user_cancel",
    ] {
        assert!(
            plaintext.contains(token),
            "plaintext must include interruption token {token}"
        );
    }
}

#[test]
fn artifacts_keep_producing_run_lineage_and_raw_fallback() {
    let export = seeded_export();
    assert_eq!(
        export.artifact_sheets.len(),
        RunLineageSeededScenario::ALL.len()
    );
    for artifact in &export.artifact_sheets {
        let summary = export
            .run_summaries
            .iter()
            .find(|summary| summary.run_id == artifact.producing_run_id)
            .expect("artifact producing run exists");
        assert_eq!(artifact.producing_attempt_id, summary.latest_attempt_id);
        assert_eq!(
            artifact.producing_execution_context_ref,
            summary.context.execution_context_ref
        );
        assert_eq!(
            artifact.raw_fallback_viewer,
            RunArtifactViewerClass::RawFallback
        );
        assert!(artifact.raw_fallback_available);
        assert_eq!(artifact.redaction_class_token, "metadata_safe_default");
        assert!(artifact
            .actions
            .iter()
            .any(|action| action.action_token == "export_redacted"));
        assert!(artifact
            .actions
            .iter()
            .any(|action| action.action_token == "review_redaction"));
    }
}

#[test]
fn stale_and_imported_rows_preserve_old_evidence_without_secret_leakage() {
    let export = seeded_export();
    assert!(export.all_jobs_are_durable());
    assert!(export.raw_private_material_excluded);
    assert!(export
        .durable_job_rows
        .iter()
        .any(DurableJobRow::preserves_old_evidence));
    assert!(export
        .run_summaries
        .iter()
        .any(|summary| summary.old_evidence_preserved));

    let json = serde_json::to_string(&export).expect("serialize support export");
    for forbidden in [
        "AWS_SECRET_ACCESS_KEY",
        "SSH_PRIVATE_KEY",
        "BEARER",
        "refresh_token",
    ] {
        assert!(
            !json.contains(forbidden),
            "secret marker leaked: {forbidden}"
        );
    }
}
