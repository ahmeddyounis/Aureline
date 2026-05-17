use std::path::{Path, PathBuf};

use aureline_runtime::{
    seeded_preview_commit_guard_scenario, PreviewCommitAdmissionDecision,
    PreviewCommitGuardScenario, PreviewCommitGuardSupportExport, PREVIEW_COMMIT_GUARD_RECORD_KIND,
    PREVIEW_COMMIT_GUARD_SCHEMA_VERSION,
};
use serde::Deserialize;

fn fixture_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/trust/m3/preview_drift")
}

#[derive(Debug, Deserialize)]
struct CaseFixture {
    record_kind: String,
    schema_version: u32,
    case_id: String,
    scenario: String,
    action_class: String,
    expect: CaseExpect,
}

#[derive(Debug, Deserialize)]
struct CaseExpect {
    admission_decision: String,
    blocks_apply: bool,
    requires_re_review: bool,
    may_auto_refresh: bool,
    reason_tokens: Vec<String>,
    cli_exit_code: u8,
}

fn scenario_for(token: &str) -> PreviewCommitGuardScenario {
    match token {
        "destructive_target_moved" => PreviewCommitGuardScenario::DestructiveTargetMoved,
        "remote_route_host_drift" => PreviewCommitGuardScenario::RemoteRouteHostDrift,
        "publish_approval_expired" => PreviewCommitGuardScenario::PublishApprovalExpired,
        "provider_representation_changed" => {
            PreviewCommitGuardScenario::ProviderRepresentationChanged
        }
        other => panic!("unknown preview-drift scenario: {other}"),
    }
}

#[test]
fn preview_drift_fixtures_replay_through_guard_evaluator() {
    for fixture_name in [
        "destructive_target_moved.json",
        "remote_route_host_drift.json",
        "publish_approval_expired.json",
        "provider_representation_changed.json",
    ] {
        let path = fixture_root().join(fixture_name);
        let payload = std::fs::read_to_string(&path)
            .unwrap_or_else(|err| panic!("read fixture {fixture_name}: {err}"));
        let fixture: CaseFixture = serde_json::from_str(&payload).unwrap_or_else(|err| {
            panic!(
                "parse fixture {} for {}: {err}",
                path.display(),
                fixture_name
            )
        });
        assert_eq!(fixture.record_kind, "preview_commit_guard_case");
        assert_eq!(fixture.schema_version, PREVIEW_COMMIT_GUARD_SCHEMA_VERSION);
        assert!(!fixture.case_id.is_empty());

        let (guard, current, evaluation) =
            seeded_preview_commit_guard_scenario(scenario_for(&fixture.scenario));
        assert_eq!(guard.record_kind, PREVIEW_COMMIT_GUARD_RECORD_KIND);
        assert_eq!(guard.schema_version, PREVIEW_COMMIT_GUARD_SCHEMA_VERSION);
        assert_eq!(guard.action_class.as_str(), fixture.action_class);
        assert_ne!(guard.basis_hash, current.basis_hash);
        assert_eq!(
            evaluation.admission_decision.as_str(),
            fixture.expect.admission_decision
        );
        assert_eq!(evaluation.blocks_apply, fixture.expect.blocks_apply);
        assert_eq!(
            evaluation.requires_re_review,
            fixture.expect.requires_re_review
        );
        assert_eq!(evaluation.may_auto_refresh, fixture.expect.may_auto_refresh);
        assert_eq!(
            evaluation.cli_output.exit_code,
            fixture.expect.cli_exit_code
        );
        assert_eq!(
            evaluation.admission_decision,
            PreviewCommitAdmissionDecision::BlockRequireReview
        );
        let actual_reason_tokens = evaluation.reason_tokens();
        for expected in &fixture.expect.reason_tokens {
            assert!(
                actual_reason_tokens.contains(expected),
                "{fixture_name}: missing reason {expected}; got {actual_reason_tokens:?}"
            );
            assert!(
                evaluation
                    .surface_projection
                    .reason_tokens
                    .contains(expected),
                "{fixture_name}: surface projection missing reason {expected}"
            );
            assert!(
                evaluation.cli_output.reason_tokens.contains(expected),
                "{fixture_name}: CLI projection missing reason {expected}"
            );
        }
        assert_eq!(evaluation.audit_events.len(), 1);
        assert!(evaluation.audit_events[0].export_safe);
        assert_eq!(
            evaluation.audit_events[0].event_token,
            "preview_commit_guard_invalidated"
        );
    }
}

#[test]
fn support_export_explains_invalidations_without_raw_payloads() {
    let evaluations = PreviewCommitGuardScenario::ALL
        .into_iter()
        .map(|scenario| seeded_preview_commit_guard_scenario(scenario).2)
        .collect::<Vec<_>>();
    let export = PreviewCommitGuardSupportExport::from_evaluations(
        "support:preview-commit-guard:all",
        "2026-05-17T22:30:00Z",
        &evaluations,
    );
    assert_eq!(export.blocked_evaluation_count, evaluations.len() as u32);
    assert_eq!(export.audit_events.len(), evaluations.len());
    let plaintext = export.render_plaintext();
    for token in [
        "target_set_drift",
        "host_boundary_drift",
        "route_drift",
        "approval_ticket_expired",
        "representation_class_drift",
    ] {
        assert!(plaintext.contains(token), "missing plaintext token {token}");
    }

    let json = serde_json::to_string(&export).expect("support export serializes");
    assert!(!json.contains("SECRET"));
    assert!(!json.contains("BEARER"));
    assert!(!json.contains("raw_payload"));
    assert!(!json.contains("private_key"));
}
