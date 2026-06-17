//! Conformance dump for per-verb stash/recovery review sheets.
//!
//! Prints the canonical export-safe [`StashRecoveryPacket`] as deterministic JSON.
//! The packet carries one review sheet per verb — stash apply, pop, drop, and
//! create-branch, plus reflog-restore and checkpoint-restore — each with exact
//! repo/worktree target truth, the pre-execution gate states, the reflog/checkpoint
//! restore surface (expiry, retention, compare / open-diff), a recovery path, and a
//! derived allow/block/downgrade decision.
//!
//! The optional first argument selects a narrowed fixture variant:
//!
//! * (no argument) — the canonical packet (one allowed sheet per verb)
//! * `pop-conflict` — a stash-pop blocked by an unresolved conflict, proving a
//!   blocked verb still keeps local inspection/restore truth
//! * `reflog-only` — a reflog-restore whose anchor is expiring and whose only
//!   recovery is reflog-only, downgraded while preserving its caveats
//! * `provider-outage` — a stash-apply whose provider overlay is unavailable,
//!   downgraded to local-only truth (never blocked)
//!
//! The canonical document is the source of the checked-in artifact, and the variants
//! are the source of the protected narrowing fixtures.

use aureline_git::{
    RecoveryAnchor, StashRecoveryPacket, StashRecoverySheet, StashRecoverySheetInput,
    StashRecoverySupportExport, StashRecoveryVerb, STASH_RECOVERY_PACKET_RECORD_KIND,
    STASH_RECOVERY_REQUIRED_RECONSTRUCTION_FIELDS, STASH_RECOVERY_SCHEMA_VERSION,
    STASH_RECOVERY_SUPPORT_EXPORT_RECORD_KIND,
};

const STAMP: &str = "2026-06-17T00:00:00Z";
const REPO_REF: &str = "repo-ref:main";
const WORKTREE_REF: &str = "worktree-ref:main";

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

/// Base input with clear gates, full checkpoint recovery, and the common actions.
///
/// Stash and recovery verbs differ in their verb-specific fields; each builder fills
/// those in. The base leaves stash/anchor fields cleared as `not_applicable`.
fn base(
    sheet_id: &str,
    verb: StashRecoveryVerb,
    primary_target_ref: &str,
) -> StashRecoverySheetInput {
    StashRecoverySheetInput {
        sheet_id: sheet_id.to_owned(),
        verb,
        repo_ref: REPO_REF.to_owned(),
        worktree_ref: WORKTREE_REF.to_owned(),
        target_kind: "repository_root".to_owned(),
        primary_target_ref: primary_target_ref.to_owned(),
        stash_entry_ref: None,
        stash_index: None,
        new_branch_ref: None,
        recovery_anchor: None,
        stash_availability_state: "not_applicable".to_owned(),
        anchor_expiry_state: "not_applicable".to_owned(),
        dirty_worktree_state: "clean".to_owned(),
        conflict_source_state: "no_conflicts".to_owned(),
        provider_overlay_state: "overlay_fresh".to_owned(),
        checkpoint_lineage_refs: refs(&["checkpoint-ref:before-op"]),
        reflog_only_fallback: false,
        restore_caveats: Vec::new(),
        local_actions: refs(&["preview", "continue", "abort", "restore_checkpoint"]),
        created_at: STAMP.to_owned(),
        updated_at: STAMP.to_owned(),
        summary_label: format!("summary:{sheet_id}"),
    }
}

/// Fills the stash-verb fields and the inspect action onto a base input.
fn as_stash_entry(input: &mut StashRecoverySheetInput, entry_ref: &str, index: u32) {
    input.stash_entry_ref = Some(entry_ref.to_owned());
    input.stash_index = Some(index);
    input.stash_availability_state = "stash_present".to_owned();
    input.local_actions.push("inspect_stash".to_owned());
}

/// Fills the recovery-anchor fields and the compare/open-diff actions onto a base.
fn as_recovery_anchor(
    input: &mut StashRecoverySheetInput,
    anchor_kind: &str,
    anchor_ref: &str,
    expires_at: Option<&str>,
    retention_class: &str,
) {
    input.recovery_anchor = Some(RecoveryAnchor {
        anchor_kind: anchor_kind.to_owned(),
        anchor_ref: anchor_ref.to_owned(),
        expires_at: expires_at.map(|value| value.to_owned()),
        retention_class: retention_class.to_owned(),
        compare_action_ref: format!("compare-ref:{anchor_ref}"),
        open_diff_action_ref: format!("open-diff-ref:{anchor_ref}"),
    });
    input.anchor_expiry_state = "fresh_retained".to_owned();
    input.local_actions.push("compare".to_owned());
    input.local_actions.push("open_diff".to_owned());
}

fn stash_apply_sheet() -> StashRecoverySheet {
    let mut input = base(
        "stash-apply-0001",
        StashRecoveryVerb::StashApply,
        "ref:worktree/main",
    );
    as_stash_entry(&mut input, "stash-ref:stash@{0}", 0);
    StashRecoverySheet::new(input)
}

fn stash_pop_sheet() -> StashRecoverySheet {
    let mut input = base(
        "stash-pop-0001",
        StashRecoveryVerb::StashPop,
        "ref:worktree/main",
    );
    as_stash_entry(&mut input, "stash-ref:stash@{0}", 0);
    StashRecoverySheet::new(input)
}

fn stash_drop_sheet() -> StashRecoverySheet {
    let mut input = base(
        "stash-drop-0001",
        StashRecoveryVerb::StashDrop,
        "ref:worktree/main",
    );
    as_stash_entry(&mut input, "stash-ref:stash@{1}", 1);
    StashRecoverySheet::new(input)
}

fn stash_create_branch_sheet() -> StashRecoverySheet {
    let mut input = base(
        "stash-create-branch-0001",
        StashRecoveryVerb::StashCreateBranch,
        "ref:branch/from-stash",
    );
    as_stash_entry(&mut input, "stash-ref:stash@{0}", 0);
    input.new_branch_ref = Some("ref:branch/from-stash".to_owned());
    StashRecoverySheet::new(input)
}

fn reflog_restore_sheet() -> StashRecoverySheet {
    let mut input = base(
        "reflog-restore-0001",
        StashRecoveryVerb::ReflogRestore,
        "ref:branch/feature",
    );
    as_recovery_anchor(
        &mut input,
        "reflog",
        "reflog-ref:HEAD@{3}",
        Some("2026-09-15T00:00:00Z"),
        "retained_default_window",
    );
    StashRecoverySheet::new(input)
}

fn checkpoint_restore_sheet() -> StashRecoverySheet {
    let mut input = base(
        "checkpoint-restore-0001",
        StashRecoveryVerb::CheckpointRestore,
        "ref:branch/feature",
    );
    as_recovery_anchor(
        &mut input,
        "checkpoint",
        "checkpoint-ref:pre-rebase",
        None,
        "pinned_no_expiry",
    );
    StashRecoverySheet::new(input)
}

fn support_export(sheets: &[StashRecoverySheet], export_id: &str) -> StashRecoverySupportExport {
    StashRecoverySupportExport {
        record_kind: STASH_RECOVERY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: export_id.to_owned(),
        sheet_refs: sheets.iter().map(|sheet| sheet.sheet_id.clone()).collect(),
        reconstruction_fields: STASH_RECOVERY_REQUIRED_RECONSTRUCTION_FIELDS
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
    sheets: Vec<StashRecoverySheet>,
) -> StashRecoveryPacket {
    let support_export = support_export(&sheets, export_id);
    StashRecoveryPacket {
        record_kind: STASH_RECOVERY_PACKET_RECORD_KIND.to_owned(),
        schema_version: STASH_RECOVERY_SCHEMA_VERSION,
        packet_id: packet_id.to_owned(),
        generated_at: STAMP.to_owned(),
        repo_ref: REPO_REF.to_owned(),
        sheets,
        support_export,
    }
}

fn canonical_packet() -> StashRecoveryPacket {
    packet_from(
        "git-stash-recovery:0001",
        "git-stash-recovery-export:0001",
        vec![
            stash_apply_sheet(),
            stash_pop_sheet(),
            stash_drop_sheet(),
            stash_create_branch_sheet(),
            reflog_restore_sheet(),
            checkpoint_restore_sheet(),
        ],
    )
}

fn pop_conflict_variant() -> StashRecoveryPacket {
    let mut input = base(
        "stash-pop-conflict-0001",
        StashRecoveryVerb::StashPop,
        "ref:worktree/main",
    );
    as_stash_entry(&mut input, "stash-ref:stash@{0}", 0);
    // An unresolved conflict blocks the pop, yet a checkpoint keeps local truth.
    input.conflict_source_state = "conflicts_present_blocks_continue".to_owned();
    input.summary_label = "summary:stash-pop-conflict-0001".to_owned();
    let sheet = StashRecoverySheet::new(input);
    packet_from(
        "git-stash-recovery:pop-conflict:0001",
        "git-stash-recovery-export:pop-conflict:0001",
        vec![sheet],
    )
}

fn reflog_only_variant() -> StashRecoveryPacket {
    let mut input = base(
        "reflog-restore-only-0001",
        StashRecoveryVerb::ReflogRestore,
        "ref:branch/feature",
    );
    as_recovery_anchor(
        &mut input,
        "reflog",
        "reflog-ref:HEAD@{7}",
        Some("2026-06-19T00:00:00Z"),
        "session_only",
    );
    // The anchor is expiring and there is no full checkpoint: both gates downgrade,
    // and the caveats are preserved so the restore never pretends to be durable.
    input.anchor_expiry_state = "expiring_soon".to_owned();
    input.checkpoint_lineage_refs = Vec::new();
    input.reflog_only_fallback = true;
    input.restore_caveats = refs(&[
        "anchor_expiring_soon",
        "reflog_only_no_full_checkpoint",
        "reflog_entry_may_expire",
    ]);
    input.summary_label = "summary:reflog-restore-only-0001".to_owned();
    let sheet = StashRecoverySheet::new(input);
    packet_from(
        "git-stash-recovery:reflog-only:0001",
        "git-stash-recovery-export:reflog-only:0001",
        vec![sheet],
    )
}

fn provider_outage_variant() -> StashRecoveryPacket {
    let mut input = base(
        "stash-apply-outage-0001",
        StashRecoveryVerb::StashApply,
        "ref:worktree/main",
    );
    as_stash_entry(&mut input, "stash-ref:stash@{0}", 0);
    // The provider overlay is unavailable; the apply downgrades to local-only and the
    // local inspect/restore truth stays available offline — never blocked.
    input.provider_overlay_state = "overlay_unavailable_local_only".to_owned();
    input.summary_label = "summary:stash-apply-outage-0001".to_owned();
    let sheet = StashRecoverySheet::new(input);
    packet_from(
        "git-stash-recovery:provider-outage:0001",
        "git-stash-recovery-export:provider-outage:0001",
        vec![sheet],
    )
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let packet = match variant.as_str() {
        "pop-conflict" => pop_conflict_variant(),
        "reflog-only" => reflog_only_variant(),
        "provider-outage" => provider_outage_variant(),
        _ => canonical_packet(),
    };
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "stash recovery packet invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", packet.render_markdown_summary());
    } else {
        println!("{}", packet.export_safe_json());
    }
}
