//! Fixture-driven coverage for the secret-repair beta projection.
//!
//! These tests parse the protected fixtures under
//! `/fixtures/security/m3/secret_repair`, validate the seeded page, and prove
//! that every drill fixture surfaces the expected typed defect kind. They
//! also confirm that the support-export wrapper preserves consumer lineage
//! and the no-plaintext-fallback invariant.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    audit_secret_repair_beta_page, validate_secret_repair_beta_page, RepairOutcomeClass,
    SecretBrokerBetaProfileClass, SecretRepairBetaDefectKind, SecretRepairBetaPage,
    SecretRepairBetaSupportExport,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/security/m3/secret_repair")
}

fn load_page(file_name: &str) -> SecretRepairBetaPage {
    let path = fixture_dir().join(file_name);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a page: {err}"))
}

#[test]
fn seeded_page_fixture_validates_with_zero_defects() {
    let page = load_page("page.json");
    validate_secret_repair_beta_page(&page).expect("seeded page validates");
    assert!(page.defects.is_empty());
    for profile in SecretBrokerBetaProfileClass::ALL {
        assert!(page
            .summary
            .profiles_present
            .iter()
            .any(|token| token == profile.as_str()));
    }
}

#[test]
fn seeded_page_links_denied_projections_to_lock_state_rows() {
    let page = load_page("page.json");
    let store_lock_reasons = [
        "backing_store_locked",
        "backing_store_unavailable",
        "backing_store_signature_missing",
    ];
    for row in &page.denied_projection_rows {
        if store_lock_reasons.contains(&row.denial_reason_token.as_str()) {
            assert!(
                row.linked_lock_state_row_ref.is_some(),
                "store-driven denial {} must link a lock-state row",
                row.denied_projection_row_id,
            );
        }
        assert!(
            !row.blocked_consumer.consumer_id.is_empty(),
            "denied row {} must identify the blocked consumer",
            row.denied_projection_row_id,
        );
        assert!(!row.remediation_path_label.is_empty());
        assert!(!row.remediation_path_ref.is_empty());
        assert!(!row.plaintext_fallback_offered);
        assert!(!row.public_endpoint_fallback_offered);
        assert!(row.local_editing_preserved);
    }
}

#[test]
fn lock_state_rows_preserve_repair_action_lineage() {
    let page = load_page("page.json");
    for row in &page.lock_state_rows {
        assert!(!row.plaintext_fallback_attempted);
        assert!(!row.plaintext_fallback_offered);
        assert!(!row.raw_secret_material_present);
        assert!(row.local_editing_preserved);
        if row.lock_state_token != "unlocked" {
            assert_ne!(row.repair_action_token, "none_required");
            assert!(!row.repair_action_label.is_empty());
            assert!(!row.remediation_path_ref.is_empty());
        }
    }
}

#[test]
fn repair_events_align_resolved_at_with_outcome() {
    let page = load_page("page.json");
    for event in &page.repair_events {
        assert!(!event.raw_secret_material_present);
        assert!(!event.plaintext_fallback_taken);
        if event.outcome.is_terminal() {
            assert!(
                event.resolved_at.is_some(),
                "terminal event {} must declare resolved_at",
                event.repair_event_id,
            );
        }
        if event.outcome.is_open() {
            assert!(
                event.resolved_at.is_none(),
                "open event {} must not declare resolved_at",
                event.repair_event_id,
            );
        }
    }
    assert!(page
        .repair_events
        .iter()
        .any(|event| event.outcome == RepairOutcomeClass::Resolved));
    assert!(page
        .repair_events
        .iter()
        .any(|event| event.outcome == RepairOutcomeClass::AwaitingUser));
}

#[test]
fn drill_plaintext_fallback_attempted_surfaces_typed_defect() {
    let page = load_page("drill_plaintext_fallback_attempted.json");
    assert!(
        page.defects
            .iter()
            .any(|defect| defect.defect_kind
                == SecretRepairBetaDefectKind::PlaintextFallbackAttempted)
    );
}

#[test]
fn drill_repair_action_missing_surfaces_typed_defect() {
    let page = load_page("drill_repair_action_missing.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind == SecretRepairBetaDefectKind::RepairActionMissing));
}

#[test]
fn drill_store_lock_denial_unlinked_surfaces_typed_defect() {
    let page = load_page("drill_store_lock_denial_unlinked.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind == SecretRepairBetaDefectKind::LinkedLockStateMissing));
}

#[test]
fn drill_terminal_outcome_missing_resolved_at_surfaces_typed_defect() {
    let page = load_page("drill_terminal_outcome_missing_resolved_at.json");
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == SecretRepairBetaDefectKind::TerminalRepairOutcomeMissingResolvedAt));
}

#[test]
fn support_export_round_trip_preserves_lineage_and_no_plaintext_invariant() {
    let page = load_page("page.json");
    let export = SecretRepairBetaSupportExport::from_page(
        "secret-repair-beta:support-export:fixture-001",
        "2026-05-16T05:00:00Z",
        page,
    );
    assert!(export.raw_secret_values_excluded);
    assert!(export.consumer_lineage_preserved);
    assert!(export.repair_lineage_preserved);
    assert!(export.no_plaintext_fallback_invariant);
    assert!(export.defect_kinds_present.is_empty());
}

#[test]
fn fixture_audit_matches_validator_recompute() {
    let page = load_page("page.json");
    let recomputed = audit_secret_repair_beta_page(
        &page.lock_state_rows,
        &page.denied_projection_rows,
        &page.repair_events,
    );
    assert!(recomputed.is_empty(), "fixture must hold zero defects");
}
