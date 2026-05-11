use std::path::Path;

use serde::Deserialize;

use aureline_commands::registry::seeded_registry;

use super::*;

fn live_attachment(id: &str) -> ComposerAttachment {
    ComposerAttachment {
        attachment_id: id.to_owned(),
        kind: AttachmentKind::WorkspaceSliceBundle,
        source_class: SourceClass::WorkspaceFileSlice,
        trust_posture: TrustPosture::TrustedFirstParty,
        selection_reason: SelectionReasonClass::UserPinned,
        status: AttachmentStatusClass::Live,
        estimated_byte_size: 1024,
        display_label: format!("Live attachment {id}"),
        placed_under_fenced_role: false,
    }
}

fn resolved_mention(id: &str) -> ComposerMention {
    ComposerMention {
        mention_id: id.to_owned(),
        kind: MentionKind::SymbolMention,
        target_stable_id: Some(format!("symbol::{id}")),
        display_label: format!("@{id}"),
        resolution_state: MentionResolutionState::Resolved,
    }
}

fn seed_draft() -> ComposerDraft {
    let mut draft = ComposerDraft::new(
        "draft.m1.seed",
        "session.m1.seed",
        "request_workspace.m1.seed",
        "Explain how the M1 composer seed validates context.",
    );
    draft.add_mention(resolved_mention("editor.find"));
    draft.add_attachment(live_attachment("att.live.workspace_slice"));
    draft
}

#[test]
fn empty_draft_lands_in_dispatch_disabled_state_with_route_marker_only() {
    let draft = ComposerDraft::new(
        "draft.empty",
        "session.empty",
        "request_workspace.empty",
        "",
    );
    let outcome = draft.validate();

    assert_eq!(outcome.state, ComposerDraftState::DispatchDisabledInM1Seed);
    assert_eq!(outcome.block_reasons.len(), 1);
    assert!(matches!(
        outcome.block_reasons[0],
        BlockReason::PolicyBlockedRoute
    ));
    assert!(!draft.has_actionable_block_reasons());
    assert_eq!(
        draft.prototype_label,
        PrototypeLabel::M1PrototypeReadOnlyNoMutation
    );
    assert_eq!(
        draft.route_placeholder.provider_class,
        ProviderClass::DisabledNoProviderInM1Seed
    );
    assert_eq!(
        draft.route_placeholder.dispatch_target_class,
        DispatchTargetClass::DisabledNoDispatchInM1Seed
    );
}

#[test]
fn happy_path_draft_keeps_route_marker_but_no_actionable_blocks() {
    let draft = seed_draft();
    let outcome = draft.validate();

    assert_eq!(outcome.state, ComposerDraftState::DispatchDisabledInM1Seed);
    assert_eq!(outcome.block_reasons.len(), 1);
    assert!(matches!(
        outcome.block_reasons[0],
        BlockReason::PolicyBlockedRoute
    ));
    assert!(!draft.has_actionable_block_reasons());
    assert_eq!(outcome.aggregate_byte_estimate, 1024);
}

#[test]
fn unresolved_mention_blocks_the_draft() {
    let mut draft = seed_draft();
    draft.add_mention(ComposerMention {
        mention_id: "mention.unresolved".to_owned(),
        kind: MentionKind::FileMention,
        target_stable_id: None,
        display_label: "@missing.rs".to_owned(),
        resolution_state: MentionResolutionState::UnresolvedNotFound,
    });

    let outcome = draft.validate();
    assert_eq!(outcome.state, ComposerDraftState::BlockedPendingResolution);
    assert!(outcome.block_reasons.iter().any(|reason| matches!(
        reason,
        BlockReason::UnresolvedMention {
            mention_id,
            resolution_state: MentionResolutionState::UnresolvedNotFound
        } if mention_id == "mention.unresolved"
    )));
    assert!(draft.has_actionable_block_reasons());
}

#[test]
fn tainted_attachment_outside_fence_is_blocked_even_when_status_lies() {
    // Failure drill: a caller adds an attachment with an untrusted posture
    // but forgets to mark its status as `TaintedOutsideFencedSection`. The
    // composer surfaces the block reason from the trust posture invariant
    // so the inspector cannot silently include the bytes.
    let mut draft = seed_draft();
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.tainted_user_paste".to_owned(),
        kind: AttachmentKind::UserSuppliedText,
        source_class: SourceClass::UserSuppliedText,
        trust_posture: TrustPosture::UntrustedUserSupplied,
        selection_reason: SelectionReasonClass::UserPastedFreeformText,
        status: AttachmentStatusClass::Live,
        estimated_byte_size: 512,
        display_label: "Pasted free-form text".to_owned(),
        placed_under_fenced_role: false,
    });

    let outcome = draft.validate();
    assert_eq!(outcome.state, ComposerDraftState::BlockedPendingResolution);
    assert!(outcome.block_reasons.iter().any(|reason| matches!(
        reason,
        BlockReason::TaintedAttachmentOutsideFencedSection { attachment_id, trust_posture }
            if attachment_id == "att.tainted_user_paste"
                && *trust_posture == TrustPosture::UntrustedUserSupplied
    )));
}

#[test]
fn out_of_scope_attachment_records_block_reason_addressable_by_id() {
    let mut draft = seed_draft();
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.outside_scope".to_owned(),
        kind: AttachmentKind::WorkspaceSliceBundle,
        source_class: SourceClass::WorkspaceFileSlice,
        trust_posture: TrustPosture::TrustedFirstParty,
        selection_reason: SelectionReasonClass::UserPinned,
        status: AttachmentStatusClass::OutOfScope,
        estimated_byte_size: 256,
        display_label: "out-of-scope file".to_owned(),
        placed_under_fenced_role: false,
    });
    let outcome = draft.validate();
    assert!(outcome.block_reasons.iter().any(|reason| matches!(
        reason,
        BlockReason::OutOfScopeAttachment { attachment_id }
            if attachment_id == "att.outside_scope"
    )));
}

#[test]
fn over_budget_aggregate_attributes_to_the_last_attachment() {
    let mut draft = ComposerDraft::new(
        "draft.budget",
        "session.budget",
        "request_workspace.budget",
        "Summarise this.",
    );
    draft.budget_byte_ceiling = 1500;
    draft.add_attachment(live_attachment("att.first"));
    let mut second = live_attachment("att.second");
    second.estimated_byte_size = 1024;
    draft.add_attachment(second);

    let outcome = draft.validate();
    assert!(outcome.aggregate_byte_estimate > outcome.budget_byte_ceiling);
    assert!(outcome.block_reasons.iter().any(|reason| matches!(
        reason,
        BlockReason::OverBudgetContext { attachment_id }
            if attachment_id == "att.second"
    )));
}

#[test]
fn slash_command_resolves_against_seeded_registry_or_records_unresolved_state() {
    let registry = seeded_registry();
    let resolved = ComposerSlashCommandInvocation::resolve_in_registry(
        "invocation.open_folder",
        "cmd:workspace.open_folder",
        "/open-folder",
        registry,
    );
    assert_eq!(
        resolved.resolution_state,
        SlashCommandResolutionState::Resolved
    );
    assert_eq!(resolved.command_id, "cmd:workspace.open_folder");

    let unresolved = ComposerSlashCommandInvocation::resolve_in_registry(
        "invocation.does_not_exist",
        "cmd:does_not_exist",
        "/does-not-exist",
        registry,
    );
    assert_eq!(
        unresolved.resolution_state,
        SlashCommandResolutionState::UnresolvedNoMatch
    );
    assert!(unresolved.command_id.is_empty());

    let mut draft = seed_draft();
    draft.add_slash_command(unresolved);
    let outcome = draft.validate();
    assert!(outcome.block_reasons.iter().any(|reason| matches!(
        reason,
        BlockReason::UnresolvedSlashCommand {
            invocation_id,
            resolution_state: SlashCommandResolutionState::UnresolvedNoMatch
        } if invocation_id == "invocation.does_not_exist"
    )));
}

#[test]
fn remove_attachment_clears_block_and_drops_the_row() {
    let mut draft = seed_draft();
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.stale".to_owned(),
        kind: AttachmentKind::WorkspaceSliceBundle,
        source_class: SourceClass::WorkspaceFileSlice,
        trust_posture: TrustPosture::TrustedFirstParty,
        selection_reason: SelectionReasonClass::UserPinned,
        status: AttachmentStatusClass::Stale,
        estimated_byte_size: 256,
        display_label: "stale slice".to_owned(),
        placed_under_fenced_role: false,
    });
    assert!(draft.has_actionable_block_reasons());

    let removed = draft.remove_attachment("att.stale");
    assert!(removed);
    assert!(!draft.has_actionable_block_reasons());
    let removed_again = draft.remove_attachment("att.stale");
    assert!(!removed_again);
}

#[test]
fn fixture_failure_drill_replays_the_tainted_outside_fence_block_reason() {
    let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
        "../../fixtures/ai/m1_composer_and_context_inspector_seed_cases/tainted_attachment_outside_fence.json",
    );
    let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
    let fixture: TaintedAttachmentCase =
        serde_json::from_str(&payload).expect("fixture must parse");

    assert_eq!(fixture.record_kind, "ai_composer_seed_case");
    assert_eq!(fixture.schema_version, 1);

    let mut draft = ComposerDraft::new(
        fixture.input.composer_draft_id,
        fixture.input.composer_session_id,
        fixture.input.request_workspace_id,
        fixture.input.intent_text,
    );
    for attachment in fixture.input.attachments {
        draft.add_attachment(attachment);
    }
    for mention in fixture.input.mentions {
        draft.add_mention(mention);
    }

    let outcome = draft.validate();
    assert_eq!(outcome.state, fixture.expect.draft_state);
    assert!(outcome.block_reasons.iter().any(|reason| matches!(
        reason,
        BlockReason::TaintedAttachmentOutsideFencedSection { attachment_id, .. }
            if attachment_id == &fixture.expect.tainted_attachment_id
    )));
}

#[derive(Debug, Deserialize)]
struct TaintedAttachmentCase {
    record_kind: String,
    schema_version: u32,
    input: TaintedAttachmentInput,
    expect: TaintedAttachmentExpect,
}

#[derive(Debug, Deserialize)]
struct TaintedAttachmentInput {
    composer_draft_id: String,
    composer_session_id: String,
    request_workspace_id: String,
    intent_text: String,
    #[serde(default)]
    mentions: Vec<ComposerMention>,
    #[serde(default)]
    attachments: Vec<ComposerAttachment>,
}

#[derive(Debug, Deserialize)]
struct TaintedAttachmentExpect {
    draft_state: ComposerDraftState,
    tainted_attachment_id: String,
}
