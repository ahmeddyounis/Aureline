//! Emit the seeded stable work-item status-transition packet as JSON.

use std::io::{self, Write};

use aureline_review::{
    seeded_stable_work_item_status_transition_packet, ReviewPolicyContext,
    ReviewWorkspaceBetaInspectionRecord, ReviewWorkspaceBetaPacket, ReviewWorkspaceRecord,
    ReviewWorkspaceSupportExportPacket,
};

fn main() {
    let wp = ReviewWorkspaceBetaPacket {
        record_kind: "review_workspace_beta_packet".to_string(),
        schema_version: 1,
        packet_id: "review-workspace-packet:seed:001".to_string(),
        generated_at: "2026-06-03T09:55:00Z".to_string(),
        review_workspace: ReviewWorkspaceRecord {
            record_kind: "review_workspace_record".to_string(),
            review_workspace_schema_version: 1,
            review_workspace_id: "review-workspace:seed:001".to_string(),
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
            summary_label: "Seed review workspace".to_string(),
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
        support_export: ReviewWorkspaceSupportExportPacket {
            record_kind: "review_workspace_support_export_packet".to_string(),
            schema_version: 1,
            support_export_id: "support-export:seed:001".to_string(),
            review_workspace_id_ref: "review-workspace:seed:001".to_string(),
            reopen_context_ref: "review-workspace:seed:001".to_string(),
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
            summary_label: "Seed support export".to_string(),
        },
        inspection: ReviewWorkspaceBetaInspectionRecord {
            record_kind: "review_workspace_beta_inspection_record".to_string(),
            schema_version: 1,
            review_workspace_id_ref: "review-workspace:seed:001".to_string(),
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
            summary_label: "Seed inspection".to_string(),
        },
    };
    let packet = seeded_stable_work_item_status_transition_packet(&wp);
    let json = serde_json::to_string_pretty(&packet).expect("serialize packet");
    io::stdout()
        .write_all(json.as_bytes())
        .expect("write stdout");
    io::stdout().write_all(b"\n").expect("write newline");
}
