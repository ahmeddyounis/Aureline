//! Fixture-driven coverage for stabilized work-item status-transition review
//! records.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_review::{
    audit_stable_work_item_packet, seeded_stable_work_item_status_transition_packet,
    validate_stable_work_item_packet, ReviewPolicyContext, ReviewWorkspaceBetaPacket,
    ReviewWorkspaceRecord, StableWorkItemStatusTransitionPacket,
    STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION,
};

fn fixture_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/review/m4/stabilize-work-item-status-transition-review/packet.json")
}

fn minimal_workspace_packet() -> ReviewWorkspaceBetaPacket {
    ReviewWorkspaceBetaPacket {
        record_kind: "review_workspace_beta_packet".to_string(),
        schema_version: 1,
        packet_id: "review-workspace-packet:test:001".to_string(),
        generated_at: "2026-06-03T09:55:00Z".to_string(),
        review_workspace: ReviewWorkspaceRecord {
            record_kind: "review_workspace_record".to_string(),
            review_workspace_schema_version: 1,
            review_workspace_id: "review-workspace:test:001".to_string(),
            review_workspace_source_class: "local_only".to_string(),
            provider_authority_class: "provider_authority_none".to_string(),
            review_workspace_lifecycle_state: "active".to_string(),
            local_locator: None,
            provider_overlay: None,
            imported_bundle_envelope: None,
            browser_handoff_envelope: None,
            policy_context: ReviewPolicyContext {
                policy_epoch: "policy.epoch.unit".to_string(),
                trust_state: "trusted".to_string(),
                execution_context_id: Some("exec.ctx.unit".to_string()),
                workspace_trust_state_class: "trusted".to_string(),
            },
            client_scopes: vec!["desktop_product".to_string()],
            redaction_class: "metadata_only".to_string(),
            freshness_class: "fresh".to_string(),
            summary_label: "Test review workspace".to_string(),
            created_at: "2026-06-03T09:55:00Z".to_string(),
            updated_at: "2026-06-03T09:55:00Z".to_string(),
            archived_at: None,
            hosted_review_inbox_record_id_ref: None,
            merge_policy_record_id_ref: None,
        },
        diff_entries: vec![],
        durable_comment_anchors: vec![],
        object_lineage: vec![],
        check_freshness: vec![],
        browser_handoff: None,
        support_export: aureline_review::ReviewWorkspaceSupportExportPacket {
            record_kind: "review_workspace_support_export_packet".to_string(),
            schema_version: 1,
            support_export_id: "support-export:test:001".to_string(),
            review_workspace_id_ref: "review-workspace:test:001".to_string(),
            reopen_context_ref: "review-workspace:test:001".to_string(),
            reopen_command_id_ref: "cmd:review.workspace.reopen".to_string(),
            durable_comment_anchor_refs: vec![],
            check_freshness_refs: vec![],
            object_lineage_refs: vec![],
            browser_handoff_ref: None,
            consumer_surfaces: vec!["support_export".to_string()],
            source_schema_refs: vec![],
            raw_comment_body_export_allowed: false,
            raw_url_export_allowed: false,
            raw_source_body_export_allowed: false,
            redaction_class: "metadata_only".to_string(),
            summary_label: "Test support export".to_string(),
        },
        inspection: aureline_review::ReviewWorkspaceBetaInspectionRecord {
            record_kind: "review_workspace_beta_inspection_record".to_string(),
            schema_version: 1,
            review_workspace_id_ref: "review-workspace:test:001".to_string(),
            durable_comment_anchor_count: 0,
            object_lineage_count: 0,
            check_freshness_count: 0,
            anchor_identity_preserved: true,
            object_lineage_preserved: true,
            check_freshness_browser_independent: true,
            typed_reversible_browser_handoff_present: false,
            support_export_reopenable: true,
            raw_escape_hatches_absent: true,
            operator_truth_current: true,
            stale_check_blocks_operator_truth: false,
            summary_label: "Test inspection".to_string(),
        },
    }
}

fn load_or_seed_packet() -> StableWorkItemStatusTransitionPacket {
    let path = fixture_path();
    if path.exists() {
        let text = fs::read_to_string(&path).expect("read stable work-item fixture");
        serde_json::from_str(&text).expect("parse stable work-item fixture")
    } else {
        let wp = minimal_workspace_packet();
        seeded_stable_work_item_status_transition_packet(&wp)
    }
}

#[test]
fn seeded_packet_passes_validation() {
    let wp = minimal_workspace_packet();
    let packet = seeded_stable_work_item_status_transition_packet(&wp);
    assert!(validate_stable_work_item_packet(&packet).is_ok());
    assert!(packet.raw_escape_hatches_absent());
    assert!(packet.restartable_from_support_export());
}

#[test]
fn packet_has_work_item_details() {
    let packet = load_or_seed_packet();
    assert!(!packet.work_item_details.is_empty());
}

#[test]
fn packet_has_offline_handoffs() {
    let packet = load_or_seed_packet();
    assert!(!packet.offline_handoffs.is_empty());
}

#[test]
fn packet_inspection_matches_counts() {
    let packet = load_or_seed_packet();
    assert_eq!(
        packet.inspection.work_item_detail_count,
        packet.work_item_details.len()
    );
    assert_eq!(
        packet.inspection.transition_sheet_count,
        packet.transition_sheets.len()
    );
    assert_eq!(
        packet.inspection.offline_handoff_count,
        packet.offline_handoffs.len()
    );
    assert_eq!(
        packet.inspection.publish_later_continuity_count,
        packet.publish_later_continuities.len()
    );
    assert_eq!(packet.inspection.command_count, packet.commands.len());
}

#[test]
fn schema_version_is_stable() {
    let packet = load_or_seed_packet();
    assert_eq!(
        packet.schema_version,
        STABLE_WORK_ITEM_STATUS_TRANSITION_REVIEW_SCHEMA_VERSION
    );
}

#[test]
fn audit_returns_empty_for_valid_packet() {
    let wp = minimal_workspace_packet();
    let packet = seeded_stable_work_item_status_transition_packet(&wp);
    let defects = audit_stable_work_item_packet(&packet);
    assert!(defects.is_empty());
}

#[test]
fn offline_handoff_survives_restart() {
    let packet = load_or_seed_packet();
    for handoff in &packet.offline_handoffs {
        assert!(
            handoff.survives_restart,
            "offline handoff {} must survive restart",
            handoff.offline_handoff_id
        );
    }
}
