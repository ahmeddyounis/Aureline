//! Interaction-integrity alpha packet coverage.

use std::fs;
use std::path::Path;

use aureline_editor::clipboard::{CopyPayload, CopyVariantId, RepresentationClass};
use aureline_review::{DiffClosedSessionRecord, DiffCompareTarget, DiffScrollAnchor};
use aureline_shell::start_center::admission_review::drag_drop_admission_packet_for_path;
use aureline_shell::transfer::{
    action_counts_by_kind, ClipboardRouteClass, EditorPasteActionInput,
    InteractionIntegrityAlphaPacket, TerminalPasteBoundaryInput, TransferActionRecord,
    TransferGateResult,
};
use aureline_terminal::{
    restore_session_as_transcript, HostClass, OpenSessionRequest, PtyHost,
    ScrollbackRedactionClass, TerminalLastCommandClass, TerminalPastePolicyResult,
    TerminalPasteReviewInput, TerminalPasteSubmitBehavior, TerminalScrollback, TerminalTrustState,
};

const MINTED_AT: &str = "2026-05-14T20:10:00Z";
const POLICY_EPOCH_REF: &str = "policy:epoch:interaction-integrity-alpha";

#[test]
fn first_consumers_build_minimum_slice_packet() {
    let default_copy = TransferActionRecord::editor_copy_from_payload(
        "transfer:copy:editor-default",
        &CopyPayload {
            copy_variant_id: CopyVariantId::Line,
            representation_class: RepresentationClass::Raw,
            text: "let answer = 42;\n".to_string(),
        },
        "editor:buffer:alpha-main",
        "clipboard:local",
        POLICY_EPOCH_REF,
        MINTED_AT,
        Vec::new(),
    );
    let sensitive_copy = TransferActionRecord::editor_explicit_copy_action(
        "transfer:copy:support-link-sensitive",
        RepresentationClass::Raw,
        "support:bundle-link:alpha",
        "clipboard:local",
        POLICY_EPOCH_REF,
        MINTED_AT,
        vec![
            "support_bundle_link".to_string(),
            "private_path".to_string(),
        ],
    );
    let editor_paste = TransferActionRecord::editor_paste_action(EditorPasteActionInput {
        action_id: "transfer:paste:editor-buffer".to_string(),
        source_ref: "clipboard:local".to_string(),
        target_buffer_ref: "editor:buffer:alpha-main".to_string(),
        target_buffer_label: "alpha-main buffer".to_string(),
        undo_group_ref: "undo:paste:editor-buffer:alpha".to_string(),
        undo_group_name: "Paste into alpha-main".to_string(),
        mutation_journal_ref: "mutation:paste:editor-buffer:alpha".to_string(),
        policy_epoch_ref: POLICY_EPOCH_REF.to_string(),
        minted_at: MINTED_AT.to_string(),
    });
    let terminal_paste =
        TransferActionRecord::terminal_paste_boundary_action(TerminalPasteBoundaryInput {
            action_id: "transfer:paste:terminal-prod".to_string(),
            source_ref: "clipboard:local".to_string(),
            review_input: TerminalPasteReviewInput {
                session_id: "pty:ws-alpha|remote_agent_primary|1".to_string(),
                host_class: HostClass::RemoteAgentPrimary,
                target_label: "prod-shell".to_string(),
                boundary_label_visible: true,
                line_count: 14,
                bracketed_paste_available: true,
                remote_clipboard_bridge: false,
                production_labeled_target: true,
                policy_result: TerminalPastePolicyResult::ReviewRequired,
                submit_behavior: TerminalPasteSubmitBehavior::NoAutoSubmit,
                review_surface_present: true,
            },
            clipboard_route: ClipboardRouteClass::LocalSystemClipboard,
            trust_result: TransferGateResult::ReviewRequired,
            policy_epoch_ref: POLICY_EPOCH_REF.to_string(),
            minted_at: MINTED_AT.to_string(),
        });
    let admission = drag_drop_admission_packet_for_path(
        "/tmp/aureline-profile-export.zip",
        Some(32 * 1024 * 1024),
        Some("workspace:alpha".to_string()),
    );
    let drop_import = TransferActionRecord::drag_drop_admission_action(
        "transfer:drop:project-entry-import",
        &admission,
        POLICY_EPOCH_REF,
        MINTED_AT,
    )
    .expect("drag/drop action");
    let diff_reopen = TransferActionRecord::diff_reopen_action(
        "transfer:reopen:diff",
        &closed_diff_record(),
        POLICY_EPOCH_REF,
        MINTED_AT,
    );
    let terminal_recover = TransferActionRecord::terminal_recover_action(
        "transfer:recover:terminal",
        &restored_terminal_record(),
        POLICY_EPOCH_REF,
        MINTED_AT,
    );

    let packet = InteractionIntegrityAlphaPacket::new(
        "packet:interaction-integrity-alpha:test",
        MINTED_AT,
        vec![
            default_copy,
            sensitive_copy,
            editor_paste,
            terminal_paste,
            drop_import,
            diff_reopen,
            terminal_recover,
        ],
    );
    let report = packet.validate();

    assert!(report.passed(), "{:#?}", report.violations);
    assert_eq!(report.coverage.action_count, 7);
    assert!(report.coverage.high_risk_terminal_paste_covered);
    assert!(report.coverage.drag_drop_verb_truth_covered);
    assert!(report.coverage.large_transfer_progress_cancel_covered);
    assert_eq!(
        packet.support_export.included_action_ids.len(),
        packet.actions.len()
    );
    assert!(!packet.support_export.raw_payload_bodies_included);

    let counts = action_counts_by_kind(&packet.actions);
    assert_eq!(counts.get("copy"), Some(&2));
    assert_eq!(counts.get("paste"), Some(&2));
    assert_eq!(counts.get("drop"), Some(&1));
    assert_eq!(counts.get("reopen"), Some(&1));
    assert_eq!(counts.get("recover"), Some(&1));

    let recover = packet
        .actions
        .iter()
        .find(|action| action.action_id == "transfer:recover:terminal")
        .expect("terminal recover action");
    let reopen = recover
        .reopen_recovery
        .as_ref()
        .expect("terminal recover carries reopen/recovery details");
    assert_eq!(
        reopen.restored_working_directory.as_deref(),
        Some("/srv/app")
    );
    assert_eq!(
        reopen.restored_shell_identity.as_deref(),
        Some("prod-shell")
    );
    assert_eq!(
        reopen.restored_environment_scope_token.as_deref(),
        Some("remote_session")
    );
    assert_eq!(
        reopen.restored_last_command_class_token.as_deref(),
        Some("build")
    );
}

#[test]
fn checked_in_interaction_integrity_packet_validates() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/ux/m2_interaction_integrity_packet.json");
    let raw = fs::read_to_string(&path).expect("packet fixture readable");
    let packet: InteractionIntegrityAlphaPacket =
        serde_json::from_str(&raw).expect("packet fixture parses");
    let report = packet.validate();
    assert!(report.passed(), "{:#?}", report.violations);
}

#[test]
fn checked_in_terminal_paste_boundary_fixture_validates() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/terminal/paste_boundary_alpha/high_risk_remote_multiline_review.json",
    );
    let raw = fs::read_to_string(&path).expect("paste fixture readable");
    let action: TransferActionRecord = serde_json::from_str(&raw).expect("paste fixture parses");
    let packet = InteractionIntegrityAlphaPacket::new(
        "packet:paste-boundary-fixture",
        MINTED_AT,
        vec![
            minimum_default_copy(),
            minimum_editor_paste_group(),
            action,
            minimum_drop_import_group(),
            TransferActionRecord::diff_reopen_action(
                "transfer:reopen:diff:fixture",
                &closed_diff_record(),
                POLICY_EPOCH_REF,
                MINTED_AT,
            ),
            TransferActionRecord::terminal_recover_action(
                "transfer:recover:terminal:fixture",
                &restored_terminal_record(),
                POLICY_EPOCH_REF,
                MINTED_AT,
            ),
        ],
    );
    let report = packet.validate();
    assert!(report.passed(), "{:#?}", report.violations);
}

fn minimum_default_copy() -> TransferActionRecord {
    TransferActionRecord::editor_copy_from_payload(
        "transfer:copy:minimum-default",
        &CopyPayload {
            copy_variant_id: CopyVariantId::Line,
            representation_class: RepresentationClass::Raw,
            text: "plain\n".to_string(),
        },
        "editor:buffer:minimum",
        "clipboard:local",
        POLICY_EPOCH_REF,
        MINTED_AT,
        Vec::new(),
    )
}

fn minimum_editor_paste_group() -> TransferActionRecord {
    TransferActionRecord::editor_paste_action(EditorPasteActionInput {
        action_id: "transfer:paste:minimum-editor".to_string(),
        source_ref: "clipboard:local".to_string(),
        target_buffer_ref: "editor:buffer:minimum".to_string(),
        target_buffer_label: "minimum buffer".to_string(),
        undo_group_ref: "undo:paste:minimum".to_string(),
        undo_group_name: "Paste into minimum buffer".to_string(),
        mutation_journal_ref: "mutation:paste:minimum".to_string(),
        policy_epoch_ref: POLICY_EPOCH_REF.to_string(),
        minted_at: MINTED_AT.to_string(),
    })
}

fn minimum_drop_import_group() -> TransferActionRecord {
    let admission = drag_drop_admission_packet_for_path(
        "/tmp/aureline-profile-export.zip",
        Some(32 * 1024 * 1024),
        Some("workspace:alpha".to_string()),
    );
    TransferActionRecord::drag_drop_admission_action(
        "transfer:drop:minimum-import",
        &admission,
        POLICY_EPOCH_REF,
        MINTED_AT,
    )
    .expect("drop import")
}

fn closed_diff_record() -> DiffClosedSessionRecord {
    DiffClosedSessionRecord {
        record_kind: "diff_closed_session_record".to_string(),
        schema_version: 1,
        closed_session_ref: "git.diff.closed.alpha".to_string(),
        closed_at: MINTED_AT.to_string(),
        reopen_command_id: "cmd:git.diff.reopen_closed".to_string(),
        diff_surface_ref: "surface:diff:alpha".to_string(),
        workspace_ref: "workspace:alpha".to_string(),
        path_truth_ref: "path-truth:src/lib.rs".to_string(),
        compare_target_ref: "git.diff.target.alpha.working_tree.src_lib_rs".to_string(),
        compare_target: DiffCompareTarget {
            compare_target_ref: "git.diff.target.alpha.working_tree.src_lib_rs".to_string(),
            target_kind_token: "working_tree".to_string(),
            base_label: "Base".to_string(),
            head_label: "Working tree".to_string(),
            base_revision_ref: Some("rev:base".to_string()),
            head_revision_ref: Some("rev:worktree".to_string()),
            exact_target_label: "Working tree diff for src/lib.rs".to_string(),
            local_diff_authority: "authoritative_local_git".to_string(),
            truth_source_ref: "git:status:snapshot:alpha".to_string(),
        },
        scroll_anchor: DiffScrollAnchor {
            first_visible_row_ref: "git.diff.row.alpha.42".to_string(),
            scroll_offset: 3,
        },
        selected_hunk_ref: Some("git.diff.hunk.alpha.1".to_string()),
        selected_row_ref: Some("git.diff.row.alpha.42".to_string()),
        launch_source_ref: "git.change.row.alpha".to_string(),
    }
}

fn restored_terminal_record() -> aureline_terminal::RestoredTerminalRecord {
    let mut host = PtyHost::new();
    let id = host.open_session(OpenSessionRequest {
        workspace_id: "ws-alpha",
        host_class: HostClass::RemoteAgentPrimary,
        display_title: "prod-shell",
        cwd_hint: Some("/srv/app"),
        execution_context_ref: "exec:terminal:prod",
        trust_state: TerminalTrustState::Trusted,
        observed_at: "mono:1",
    });
    host.update_last_command_class(&id, TerminalLastCommandClass::Build, "mono:1.5")
        .expect("last command class updates");
    host.mark_lost_transport(&id, "mono:2", Some("network_drop"))
        .expect("transport drop");
    let mut scrollback = TerminalScrollback::new(id.clone());
    scrollback.record_line(
        "$ deploy --dry-run",
        ScrollbackRedactionClass::SupportBundleScoped,
        "mono:1",
    );
    let prior = host.session(&id).expect("prior session");
    restore_session_as_transcript(prior, Some(&scrollback), MINTED_AT)
}
