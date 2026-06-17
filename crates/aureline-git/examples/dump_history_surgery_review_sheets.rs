//! Conformance dump for per-verb history-surgery review sheets.
//!
//! Prints the canonical export-safe [`HistorySurgeryReviewPacket`] as
//! deterministic JSON. The packet carries one review sheet per risky verb
//! (rebase, cherry-pick, revert, reset, patch-apply, force-push), each with exact
//! repo/worktree target truth, the pre-execution gate states, raw todo/patch
//! inspection refs, a reflog/checkpoint recovery path, and a derived
//! allow/block/downgrade decision.
//!
//! The optional first argument selects a narrowed fixture variant:
//!
//! * (no argument) — the canonical packet (one allowed sheet per verb)
//! * `force-push-protected` — a force-push blocked by a protected branch, proving
//!   a blocked decision still keeps local preview/abort/restore truth
//! * `rebase-raw-fallback` — a rebase whose structured parsing failed, downgraded
//!   to raw-todo-only inspection rather than blocked
//! * `provider-outage` — a reset whose provider overlay is unavailable, downgraded
//!   to local-only truth (never blocked)
//!
//! The canonical document is the source of the checked-in artifact, and the
//! variants are the source of the protected narrowing fixtures.

use aureline_git::{
    HistorySurgeryReviewPacket, HistorySurgeryReviewSheet, HistorySurgeryReviewSheetInput,
    HistorySurgeryReviewSupportExport, HistorySurgeryVerb,
    HISTORY_SURGERY_REVIEW_PACKET_RECORD_KIND,
    HISTORY_SURGERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS, HISTORY_SURGERY_REVIEW_SCHEMA_VERSION,
    HISTORY_SURGERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND,
};

const STAMP: &str = "2026-06-17T00:00:00Z";
const REPO_REF: &str = "repo-ref:main";
const WORKTREE_REF: &str = "worktree-ref:main";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

/// Base input with clear gates, full checkpoint recovery, and full local actions.
fn base(
    sheet_id: &str,
    verb: HistorySurgeryVerb,
    primary_target_ref: &str,
) -> HistorySurgeryReviewSheetInput {
    HistorySurgeryReviewSheetInput {
        sheet_id: sheet_id.to_owned(),
        verb,
        repo_ref: REPO_REF.to_owned(),
        worktree_ref: WORKTREE_REF.to_owned(),
        target_kind: "repository_root".to_owned(),
        primary_target_ref: primary_target_ref.to_owned(),
        secondary_refs: Vec::new(),
        reset_mode: None,
        force_lease_ref: None,
        divergence_class: None,
        protected_branch_posture: "no_protected_refs".to_owned(),
        stale_review_state: "approval_not_required".to_owned(),
        merge_queue_state: "not_enqueued".to_owned(),
        dirty_worktree_state: "clean".to_owned(),
        conflict_source_state: "no_conflicts".to_owned(),
        provider_overlay_state: "overlay_fresh".to_owned(),
        raw_source_text_ref: None,
        structured_cards_ref: None,
        checkpoint_lineage_refs: refs(&["checkpoint-ref:before-op"]),
        reflog_only_fallback: false,
        local_actions: refs(&[
            "preview",
            "continue",
            "skip",
            "abort",
            "restore_checkpoint",
            "inspect_raw_source",
        ]),
        created_at: STAMP.to_owned(),
        updated_at: STAMP.to_owned(),
        summary_label: format!("summary:{sheet_id}"),
    }
}

fn rebase_sheet() -> HistorySurgeryReviewSheet {
    let mut input = base(
        "rebase-0001",
        HistorySurgeryVerb::Rebase,
        "ref:branch/feature",
    );
    input.secondary_refs = refs(&["onto-ref:main", "base-ref:merge-base"]);
    input.raw_source_text_ref = Some("raw-ref:rebase-0001/todo".to_owned());
    input.structured_cards_ref = Some("cards-ref:rebase-0001".to_owned());
    input.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-rebase"]);
    HistorySurgeryReviewSheet::new(input)
}

fn cherry_pick_sheet() -> HistorySurgeryReviewSheet {
    let mut input = base(
        "cherry-pick-0001",
        HistorySurgeryVerb::CherryPick,
        "ref:branch/release",
    );
    input.secondary_refs = refs(&["commit-ref:fix-a", "commit-ref:fix-b"]);
    input.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-cherry-pick"]);
    HistorySurgeryReviewSheet::new(input)
}

fn revert_sheet() -> HistorySurgeryReviewSheet {
    let mut input = base("revert-0001", HistorySurgeryVerb::Revert, "ref:branch/main");
    input.secondary_refs = refs(&["commit-ref:bad-change"]);
    input.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-revert"]);
    HistorySurgeryReviewSheet::new(input)
}

fn reset_sheet() -> HistorySurgeryReviewSheet {
    let mut input = base(
        "reset-0001",
        HistorySurgeryVerb::Reset,
        "ref:branch/feature",
    );
    input.reset_mode = Some("hard".to_owned());
    input.secondary_refs = refs(&["target-ref:origin/feature"]);
    input.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-reset"]);
    HistorySurgeryReviewSheet::new(input)
}

fn patch_apply_sheet() -> HistorySurgeryReviewSheet {
    let mut input = base(
        "patch-apply-0001",
        HistorySurgeryVerb::PatchApply,
        "ref:worktree/feature",
    );
    input.target_kind = "linked_worktree".to_owned();
    input.worktree_ref = "worktree-ref:feature".to_owned();
    input.raw_source_text_ref = Some("raw-ref:patch-apply-0001/patch".to_owned());
    input.structured_cards_ref = Some("cards-ref:patch-apply-0001".to_owned());
    input.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-patch-apply"]);
    HistorySurgeryReviewSheet::new(input)
}

fn force_push_sheet() -> HistorySurgeryReviewSheet {
    let mut input = base(
        "force-push-0001",
        HistorySurgeryVerb::ForcePush,
        "ref:remote/feature",
    );
    input.secondary_refs = refs(&["remote-ref:origin", "local-ref:feature"]);
    input.force_lease_ref = Some("lease-ref:expected-old-value".to_owned());
    input.divergence_class = Some("local_ahead".to_owned());
    input.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-force-push"]);
    HistorySurgeryReviewSheet::new(input)
}

fn support_export(
    sheets: &[HistorySurgeryReviewSheet],
    export_id: &str,
) -> HistorySurgeryReviewSupportExport {
    HistorySurgeryReviewSupportExport {
        record_kind: HISTORY_SURGERY_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: export_id.to_owned(),
        sheet_refs: sheets.iter().map(|sheet| sheet.sheet_id.clone()).collect(),
        reconstruction_fields: HISTORY_SURGERY_REVIEW_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_patch_bodies_redacted: true,
        raw_provider_payloads_redacted: true,
    }
}

fn packet_from(
    packet_id: &str,
    export_id: &str,
    sheets: Vec<HistorySurgeryReviewSheet>,
) -> HistorySurgeryReviewPacket {
    let support_export = support_export(&sheets, export_id);
    HistorySurgeryReviewPacket {
        record_kind: HISTORY_SURGERY_REVIEW_PACKET_RECORD_KIND.to_owned(),
        schema_version: HISTORY_SURGERY_REVIEW_SCHEMA_VERSION,
        packet_id: packet_id.to_owned(),
        generated_at: STAMP.to_owned(),
        repo_ref: REPO_REF.to_owned(),
        sheets,
        support_export,
    }
}

fn canonical_packet() -> HistorySurgeryReviewPacket {
    packet_from(
        "git-history-surgery-review:0001",
        "git-history-surgery-review-export:0001",
        vec![
            rebase_sheet(),
            cherry_pick_sheet(),
            revert_sheet(),
            reset_sheet(),
            patch_apply_sheet(),
            force_push_sheet(),
        ],
    )
}

fn force_push_protected_variant() -> HistorySurgeryReviewPacket {
    let mut input = base(
        "force-push-blocked-0001",
        HistorySurgeryVerb::ForcePush,
        "ref:remote/main",
    );
    input.secondary_refs = refs(&["remote-ref:origin", "local-ref:main"]);
    input.force_lease_ref = Some("lease-ref:expected-old-value".to_owned());
    input.divergence_class = Some("diverged_requires_rebase".to_owned());
    input.protected_branch_posture = "protected_branch_blocked".to_owned();
    input.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-force-push"]);
    input.summary_label = "summary:force-push-blocked-0001".to_owned();
    let sheet = HistorySurgeryReviewSheet::new(input);
    packet_from(
        "git-history-surgery-review:force-push-protected:0001",
        "git-history-surgery-review-export:force-push-protected:0001",
        vec![sheet],
    )
}

fn rebase_raw_fallback_variant() -> HistorySurgeryReviewPacket {
    let mut input = base(
        "rebase-raw-fallback-0001",
        HistorySurgeryVerb::Rebase,
        "ref:branch/feature",
    );
    input.secondary_refs = refs(&["onto-ref:main"]);
    // Raw todo is preserved, but structured parsing failed: downgrade to raw-only.
    input.raw_source_text_ref = Some("raw-ref:rebase-raw-fallback-0001/todo".to_owned());
    input.structured_cards_ref = None;
    input.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-rebase"]);
    input.summary_label = "summary:rebase-raw-fallback-0001".to_owned();
    let sheet = HistorySurgeryReviewSheet::new(input);
    packet_from(
        "git-history-surgery-review:rebase-raw-fallback:0001",
        "git-history-surgery-review-export:rebase-raw-fallback:0001",
        vec![sheet],
    )
}

fn provider_outage_variant() -> HistorySurgeryReviewPacket {
    let mut input = base(
        "reset-provider-outage-0001",
        HistorySurgeryVerb::Reset,
        "ref:branch/feature",
    );
    input.reset_mode = Some("mixed".to_owned());
    input.secondary_refs = refs(&["target-ref:HEAD~2"]);
    // Provider overlay is unavailable; only a reflog fallback remains. Both
    // downgrade — neither blocks — so local preview/abort/restore stays available.
    input.provider_overlay_state = "overlay_unavailable_local_only".to_owned();
    input.checkpoint_lineage_refs = Vec::new();
    input.reflog_only_fallback = true;
    input.summary_label = "summary:reset-provider-outage-0001".to_owned();
    let sheet = HistorySurgeryReviewSheet::new(input);
    packet_from(
        "git-history-surgery-review:provider-outage:0001",
        "git-history-surgery-review-export:provider-outage:0001",
        vec![sheet],
    )
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let packet = match variant.as_str() {
        "force-push-protected" => force_push_protected_variant(),
        "rebase-raw-fallback" => rebase_raw_fallback_variant(),
        "provider-outage" => provider_outage_variant(),
        _ => canonical_packet(),
    };
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "history surgery review packet invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
