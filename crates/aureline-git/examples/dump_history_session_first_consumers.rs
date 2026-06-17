//! Conformance dump for durable history-surgery sessions and their first
//! consumers.
//!
//! Prints the canonical export-safe [`HistorySessionConsumerMap`] as
//! deterministic JSON. The optional first argument selects a narrowed fixture
//! variant:
//!
//! * (no argument) — the canonical first-consumers map (one session per kind)
//! * `stash` — a single stash entry proving apply/pop/drop/create-branch stay
//!   distinct verbs across every surface
//! * `publish-blocked` — a publish proposal whose invalidated checks keep the
//!   network mutation gated on every surface
//! * `conflict` — a conflict session proving repo/worktree identity and raw +
//!   structured source text survive reopen, support, and provider surfaces
//!
//! The canonical document is the source of the checked-in artifact, and the
//! variants are the source of the protected narrowing fixtures.

use aureline_git::{
    HistorySession, HistorySessionConsumerMap, HistorySessionSupportExport, HistorySurgerySession,
    SessionConsumerBinding, SessionConsumerSurface, HISTORY_SESSION_DESCRIPTOR_RECORD_KIND,
    HISTORY_SESSION_MAP_RECORD_KIND, HISTORY_SESSION_REQUIRED_RECONSTRUCTION_FIELDS,
    HISTORY_SESSION_SCHEMA_VERSION, HISTORY_SESSION_SUPPORT_EXPORT_RECORD_KIND,
};

const STAMP: &str = "2026-06-17T00:00:00Z";
const REPO_REF: &str = "repo-ref:main";
const WORKTREE_REF: &str = "worktree-ref:main";

/// Base descriptor with empty optional facets; callers fill in what their kind
/// needs.
fn base(
    session_kind: HistorySurgerySession,
    session_id: &str,
    lifecycle_state: &str,
    available_actions: &[&str],
) -> HistorySession {
    HistorySession {
        record_kind: HISTORY_SESSION_DESCRIPTOR_RECORD_KIND.to_owned(),
        session_kind,
        canonical_record_kind: session_kind.canonical_record_kind().to_owned(),
        session_id: session_id.to_owned(),
        repo_ref: REPO_REF.to_owned(),
        worktree_ref: WORKTREE_REF.to_owned(),
        lifecycle_state: lifecycle_state.to_owned(),
        target_refs: Vec::new(),
        path_scope_tokens: Vec::new(),
        unresolved_count: 0,
        checkpoint_lineage_refs: Vec::new(),
        raw_source_text_ref: None,
        structured_cards_ref: None,
        available_actions: available_actions.iter().map(|a| (*a).to_owned()).collect(),
        resolution_mode: None,
        divergence_class: None,
        approval_state: None,
        check_invalidation_state: None,
        publish_mode: None,
        affected_approval_refs: Vec::new(),
        affected_check_refs: Vec::new(),
        trigger_kind: None,
        restore_option_classes: Vec::new(),
        reflog_only_fallback: false,
        created_at: STAMP.to_owned(),
        updated_at: STAMP.to_owned(),
        summary_label: format!("summary:{session_id}"),
    }
}

fn refs(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn conflict_session() -> HistorySession {
    let mut session = base(
        HistorySurgerySession::ConflictSession,
        "conflict-0001",
        "active_awaiting_resolution",
        &["continue", "abort", "skip"],
    );
    session.target_refs = refs(&["rev-ref:base", "rev-ref:ours", "rev-ref:theirs"]);
    session.path_scope_tokens = refs(&["path-token:src/app", "path-token:src/lib"]);
    session.unresolved_count = 2;
    session.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-merge"]);
    session.raw_source_text_ref = Some("raw-ref:conflict-0001/markers".to_owned());
    session.structured_cards_ref = Some("cards-ref:conflict-0001".to_owned());
    session.resolution_mode = Some("structured".to_owned());
    session
}

fn sequence_edit_session() -> HistorySession {
    let mut session = base(
        HistorySurgerySession::SequenceEditSession,
        "sequence-0001",
        "running",
        &["continue", "abort", "skip", "edit_sequence"],
    );
    session.target_refs = refs(&["rev-ref:onto"]);
    session.path_scope_tokens = refs(&["path-token:src"]);
    session.unresolved_count = 0;
    session.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-rebase"]);
    session.raw_source_text_ref = Some("raw-ref:sequence-0001/todo".to_owned());
    session.structured_cards_ref = Some("cards-ref:sequence-0001".to_owned());
    session
}

fn stash_entry() -> HistorySession {
    let mut session = base(
        HistorySurgerySession::StashShelfEntry,
        "stash-0001",
        "captured_unapplied",
        &["apply", "pop", "drop", "create_branch"],
    );
    session.target_refs = refs(&["rev-ref:stash@{0}"]);
    session.path_scope_tokens = refs(&["path-token:src/feature"]);
    session.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-stash-apply"]);
    session
}

fn publish_proposal() -> HistorySession {
    let mut session = base(
        HistorySurgerySession::PublishRefUpdateProposal,
        "publish-0001",
        "ready_to_publish",
        &["publish", "withdraw"],
    );
    session.target_refs = refs(&["ref-pos:local/main", "ref-pos:remote/main"]);
    session.checkpoint_lineage_refs = refs(&["checkpoint-ref:before-publish"]);
    session.divergence_class = Some("local_ahead".to_owned());
    session.approval_state = Some("approved_current".to_owned());
    session.check_invalidation_state = Some("checks_current".to_owned());
    session.publish_mode = Some("push_branch".to_owned());
    session.affected_approval_refs = refs(&["approval-ref:reviewer-a"]);
    session.affected_check_refs = refs(&["check-ref:ci-build"]);
    session
}

fn recovery_checkpoint() -> HistorySession {
    let mut session = base(
        HistorySurgerySession::RecoveryCheckpoint,
        "checkpoint-0001",
        "captured_ready_to_restore",
        &["restore", "prune"],
    );
    session.target_refs = refs(&["rev-ref:pre-mutation-head"]);
    session.trigger_kind = Some("before_rebase".to_owned());
    session.restore_option_classes = refs(&["restore_head_index_worktree", "export_patch_bundle"]);
    session
}

fn bindings(sessions: &[HistorySession]) -> Vec<SessionConsumerBinding> {
    let mut out = Vec::new();
    for session in sessions {
        for surface in SessionConsumerSurface::ALL {
            let binding_id = format!("binding-{}-{}", surface.as_str(), session.session_id);
            out.push(session.project(surface, binding_id));
        }
    }
    out
}

fn support_export(
    sessions: &[HistorySession],
    bindings: &[SessionConsumerBinding],
    export_id: &str,
) -> HistorySessionSupportExport {
    HistorySessionSupportExport {
        record_kind: HISTORY_SESSION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        export_id: export_id.to_owned(),
        session_refs: sessions
            .iter()
            .map(|session| session.session_id.clone())
            .collect(),
        binding_refs: bindings
            .iter()
            .map(|binding| binding.binding_id.clone())
            .collect(),
        reconstruction_fields: HISTORY_SESSION_REQUIRED_RECONSTRUCTION_FIELDS
            .iter()
            .map(|field| (*field).to_owned())
            .collect(),
        raw_paths_redacted: true,
        raw_patch_bodies_redacted: true,
        raw_provider_payloads_redacted: true,
    }
}

fn map_from(
    map_id: &str,
    export_id: &str,
    sessions: Vec<HistorySession>,
) -> HistorySessionConsumerMap {
    let consumer_bindings = bindings(&sessions);
    let support_export = support_export(&sessions, &consumer_bindings, export_id);
    HistorySessionConsumerMap {
        record_kind: HISTORY_SESSION_MAP_RECORD_KIND.to_owned(),
        schema_version: HISTORY_SESSION_SCHEMA_VERSION,
        map_id: map_id.to_owned(),
        generated_at: STAMP.to_owned(),
        repo_ref: REPO_REF.to_owned(),
        worktree_ref: WORKTREE_REF.to_owned(),
        sessions,
        consumer_bindings,
        support_export,
    }
}

fn canonical_map() -> HistorySessionConsumerMap {
    map_from(
        "git-history-session-first-consumers:0001",
        "git-history-session-first-consumers-export:0001",
        vec![
            conflict_session(),
            sequence_edit_session(),
            stash_entry(),
            publish_proposal(),
            recovery_checkpoint(),
        ],
    )
}

fn stash_variant() -> HistorySessionConsumerMap {
    map_from(
        "git-history-session-first-consumers:stash-distinct-verbs:0001",
        "git-history-session-first-consumers-export:stash:0001",
        vec![stash_entry()],
    )
}

fn publish_blocked_variant() -> HistorySessionConsumerMap {
    let mut proposal = publish_proposal();
    proposal.session_id = "publish-blocked-0001".to_owned();
    proposal.lifecycle_state = "blocked_invalidated_approval".to_owned();
    proposal.approval_state = Some("approval_invalidated_by_changes".to_owned());
    proposal.check_invalidation_state = Some("checks_invalidated_blocks_publish".to_owned());
    proposal.summary_label = "summary:publish-blocked-0001".to_owned();
    map_from(
        "git-history-session-first-consumers:publish-blocked:0001",
        "git-history-session-first-consumers-export:publish-blocked:0001",
        vec![proposal],
    )
}

fn conflict_variant() -> HistorySessionConsumerMap {
    map_from(
        "git-history-session-first-consumers:conflict-reopen:0001",
        "git-history-session-first-consumers-export:conflict:0001",
        vec![conflict_session()],
    )
}

fn main() {
    let variant = std::env::args().nth(1).unwrap_or_default();
    let map = match variant.as_str() {
        "stash" => stash_variant(),
        "publish-blocked" => publish_blocked_variant(),
        "conflict" => conflict_variant(),
        _ => canonical_map(),
    };
    let violations = map.validate();
    assert!(
        violations.is_empty(),
        "history session map invalid: {violations:?}"
    );
    if std::env::args().any(|arg| arg == "--markdown") {
        print!("{}", map.render_markdown_summary());
    } else {
        println!("{}", map.export_safe_json());
    }
}
