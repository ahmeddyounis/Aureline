use std::fs;
use std::path::{Path, PathBuf};

use aureline_support::{
    seeded_recovery_review_packet, RecoveryReviewPacket, CRASH_LOOP_REVIEW_ROW_RECORD_KIND,
    QUARANTINE_REVIEW_ROW_RECORD_KIND, RECOVERY_CONTINUITY_ROW_RECORD_KIND,
    RECOVERY_REVIEW_ARTIFACT_REF, RECOVERY_REVIEW_DOC_REF, RECOVERY_REVIEW_FIXTURE_DIR,
    RECOVERY_REVIEW_PACKET_RECORD_KIND, RECOVERY_REVIEW_SCHEMA_REF, RECOVERY_REVIEW_SCHEMA_VERSION,
    SCOPED_RESET_REVIEW_ROW_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn fixture_dir() -> PathBuf {
    repo_root().join(RECOVERY_REVIEW_FIXTURE_DIR)
}

fn load_fixture(name: &str) -> RecoveryReviewPacket {
    let path = fixture_dir().join(name);
    serde_json::from_slice(
        &fs::read(&path).unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display())),
    )
    .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()))
}

#[test]
fn seeded_packet_joins_crash_loop_scoped_reset_quarantine_and_continuity_truth() {
    let packet = seeded_recovery_review_packet();

    assert_eq!(packet.record_kind, RECOVERY_REVIEW_PACKET_RECORD_KIND);
    assert_eq!(packet.schema_version, RECOVERY_REVIEW_SCHEMA_VERSION);
    assert_eq!(packet.doc_ref, RECOVERY_REVIEW_DOC_REF);
    assert_eq!(packet.schema_ref, RECOVERY_REVIEW_SCHEMA_REF);
    assert_eq!(packet.artifact_ref, RECOVERY_REVIEW_ARTIFACT_REF);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    for lane in [
        "lane:notebook-kernel",
        "lane:preview-dev-server",
        "lane:provider-run",
        "lane:data-api-connector",
        "lane:debug-task-adapter",
        "lane:remote-agent",
    ] {
        assert!(
            packet
                .continuity_rows
                .iter()
                .any(|row| row.host_lane_ref == lane
                    && row.record_kind == RECOVERY_CONTINUITY_ROW_RECORD_KIND),
            "missing continuity row for {lane}",
        );
    }

    assert!(packet
        .protected_fault_domain_tokens
        .contains(&"session_execution_host".to_owned()));
    assert!(packet
        .protected_fault_domain_tokens
        .contains(&"remote_connector".to_owned()));
}

#[test]
fn crash_loop_review_keeps_build_session_and_bounded_recovery_paths_visible() {
    let packet = seeded_recovery_review_packet();
    let review = packet
        .crash_loop_reviews
        .iter()
        .find(|row| row.host_lane_ref == "lane:notebook-kernel")
        .expect("notebook crash-loop review exists");

    assert_eq!(review.record_kind, CRASH_LOOP_REVIEW_ROW_RECORD_KIND);
    assert_eq!(review.fault_domain_token, "session_execution_host");
    assert_eq!(review.last_reopen_mode_token, "full_restore");
    assert!(!review.build_id.is_empty());
    assert!(!review.crash_id.is_empty());
    assert_eq!(review.session_ref, "notebook:analysis.ipynb");
    assert_eq!(
        review.safe_mode_command_id,
        "command.recovery.enter_safe_mode"
    );
    assert_eq!(
        review.open_without_restore_command_id,
        "command.recovery.open_without_restore"
    );
    assert_eq!(review.open_logs_command_id, "command.recovery.open_logs");
    assert_eq!(
        review.export_command_id,
        "command.recovery.export_crash_manifest"
    );
    assert!(review
        .disable_recent_change_refs
        .contains(&"change.extension.notebook-lsp.update".to_owned()));
    assert!(review
        .disable_recent_change_refs
        .contains(&"change.layout.notebook-runtime-panel".to_owned()));
    assert!(review.no_hidden_rerun);
}

#[test]
fn scoped_reset_review_preserves_identity_drift_and_no_hidden_rerun() {
    let packet = seeded_recovery_review_packet();
    let review = packet
        .scoped_reset_reviews
        .iter()
        .find(|row| row.current_host_lane_ref == "lane:remote-agent")
        .expect("remote scoped reset review exists");

    assert_eq!(review.record_kind, SCOPED_RESET_REVIEW_ROW_RECORD_KIND);
    assert_eq!(review.replay_risk_token, "privileged");
    assert_eq!(review.rerun_requirement_token, "reapproval_required");
    assert_eq!(review.decision_token, "reapproval_required");
    assert!(review.approval_or_policy_drift_present);
    assert!(review.auth_drift_present);
    assert!(review
        .surrounding_surface_tokens
        .contains(&"preview".to_owned()));
    assert!(review
        .preserved_state_refs
        .contains(&"checkpoint:remote-route-witness".to_owned()));
    assert!(review
        .lost_state_refs
        .contains(&"forwarded-port:3000".to_owned()));
    assert!(review.no_hidden_rerun);
}

#[test]
fn quarantine_reviews_cover_quarantined_and_budget_abuse_lanes() {
    let packet = seeded_recovery_review_packet();

    for (lane, state) in [
        ("lane:extension-sandbox", "quarantined"),
        ("lane:notebook-kernel", "quarantined"),
        ("lane:provider-run", "budget_exhausted"),
        ("lane:data-api-connector", "budget_warning"),
    ] {
        let review = packet
            .quarantine_reviews
            .iter()
            .find(|row| row.host_lane_ref == lane)
            .unwrap_or_else(|| panic!("missing quarantine review for {lane}"));
        assert_eq!(review.record_kind, QUARANTINE_REVIEW_ROW_RECORD_KIND);
        assert_eq!(review.current_state_token, state);
        assert!(!review.trigger_summary.is_empty());
        assert!(!review.scope_summary.is_empty());
        assert!(!review.evidence_refs.is_empty());
        assert!(!review.rollback_candidate_ref.is_empty());
        assert!(!review.support_export_ref.is_empty());
        assert!(!review.confirm_action_ref.is_empty());
    }
}

#[test]
fn plaintext_summary_stays_export_safe_and_mentions_review_classes() {
    let packet = seeded_recovery_review_packet();
    let plaintext = packet.render_plaintext();

    assert!(plaintext.contains("Recovery review packet"));
    assert!(plaintext.contains("Crash loop: lane:notebook-kernel"));
    assert!(plaintext.contains("Scoped reset: lane:remote-agent -> lane:remote-agent"));
    assert!(plaintext.contains("Quarantine review: lane:provider-run state=budget_exhausted"));
    assert!(!plaintext.contains("/Users/"));
}

#[test]
fn checked_in_docs_schema_artifact_and_fixtures_exist() {
    let root = repo_root();
    for rel in [
        RECOVERY_REVIEW_SCHEMA_REF,
        RECOVERY_REVIEW_DOC_REF,
        RECOVERY_REVIEW_ARTIFACT_REF,
        "fixtures/support/m5/recovery_review/README.md",
        "fixtures/support/m5/recovery_review/manifest.yaml",
        "fixtures/support/m5/recovery_review/packet.json",
    ] {
        assert!(root.join(rel).is_file(), "{rel} must exist");
    }
}

#[test]
fn checked_in_fixture_validates() {
    let packet = load_fixture("packet.json");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}
