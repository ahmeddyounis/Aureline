use aureline_ai::{
    AttachmentKind, AttachmentStatusClass, ComposerAttachment, ComposerDraft, ComposerMention,
    ComposerSlashCommandInvocation, MentionKind, MentionResolutionState, SelectionReasonClass,
    SourceClass, TrustPosture,
};
use aureline_commands::registry::seeded_registry;
use aureline_search::{
    HiddenScopeDisclosure, ScopeCandidateTruthRecord, ScopeTruthSurface, SearchNoResultsState,
    SearchScopeCountsInputs, SearchScopeCountsRecord,
};

use super::*;

fn baseline_draft() -> ComposerDraft {
    let mut draft = ComposerDraft::new(
        "draft.test",
        "session.test",
        "request_workspace.test",
        "Explain editor.find",
    );
    draft.add_mention(ComposerMention {
        mention_id: "mention.editor_find".to_owned(),
        kind: MentionKind::SymbolMention,
        target_stable_id: Some("cmd:editor.find".to_owned()),
        display_label: "@editor.find".to_owned(),
        resolution_state: MentionResolutionState::Resolved,
    });
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.live.slice".to_owned(),
        kind: AttachmentKind::WorkspaceSliceBundle,
        source_class: SourceClass::WorkspaceFileSlice,
        trust_posture: TrustPosture::TrustedFirstParty,
        selection_reason: SelectionReasonClass::UserPinned,
        status: AttachmentStatusClass::Live,
        estimated_byte_size: 1024,
        display_label: "src/lib.rs slice".to_owned(),
        scope_truth: None,
        placed_under_fenced_role: false,
    });
    draft
}

fn ai_scope_truth() -> ScopeCandidateTruthRecord {
    let counts = SearchScopeCountsRecord::derive(SearchScopeCountsInputs {
        visible_rows: 0,
        loaded_rows: Some(0),
        all_matching_rows: Some(1),
        hidden_by_current_scope_rows: 1,
        hidden_by_policy_rows: 0,
        hidden_by_remote_cache_rows: 0,
        readiness_is_ready: true,
    });
    let hidden =
        HiddenScopeDisclosure::derive("Selected workset · Editor core", &counts, None, false);
    ScopeCandidateTruthRecord::new(
        ScopeTruthSurface::AiContextCandidate,
        "Selected workset · Editor core",
        "selected_workset",
        Some("scope:editor_core".to_owned()),
        Some("sparse".to_owned()),
        Some("repo:payments-api".to_owned()),
        "authoritative_live",
        true,
        false,
        counts,
        SearchNoResultsState::NoResultsInThisWorkset,
        hidden,
        Vec::new(),
    )
}

#[test]
fn snapshot_shape_is_stable_for_a_clean_draft() {
    let draft = baseline_draft();
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    assert_eq!(snapshot.record_kind, AI_CONTEXT_INSPECTOR_RECORD_KIND);
    assert_eq!(snapshot.schema_version, AI_CONTEXT_INSPECTOR_SCHEMA_VERSION);
    assert_eq!(snapshot.composer_draft_id, "draft.test");

    let section_ids: Vec<_> = snapshot
        .sections
        .iter()
        .map(|section| section.section_id)
        .collect();
    assert_eq!(
        section_ids,
        vec![
            InspectorSectionId::PrototypeLabel,
            InspectorSectionId::Intent,
            InspectorSectionId::Mentions,
            InspectorSectionId::Attachments,
            InspectorSectionId::SlashCommands,
            InspectorSectionId::RoutePlaceholder,
            InspectorSectionId::BlockReasons,
            InspectorSectionId::DraftState,
        ]
    );

    let actions: Vec<_> = snapshot.actions().map(|row| row.action).collect();
    assert_eq!(
        actions,
        vec![
            InspectorAction::CopyDraft,
            InspectorAction::InspectAttachment,
            InspectorAction::RemoveAttachment,
            InspectorAction::ResolveAddressable,
            InspectorAction::ReturnToComposer,
        ]
    );

    assert!(!snapshot.has_actionable_blocks);
    assert!(!snapshot.has_tainted_attachments);
}

#[test]
fn prototype_chip_carries_read_only_no_dispatch_label() {
    let draft = baseline_draft();
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    let chip = snapshot
        .section(InspectorSectionId::PrototypeLabel)
        .expect("prototype section");
    let row = &chip.rows[0];
    assert_eq!(
        row.value_token.as_deref(),
        Some("m1_prototype_read_only_no_mutation")
    );
    assert!(row.value.contains("read-only"));
    assert!(row.value.contains("no model dispatch"));
}

#[test]
fn search_result_attachment_reuses_shared_scope_truth() {
    let mut draft = baseline_draft();
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.search_result.outside_scope".to_owned(),
        kind: AttachmentKind::WorkspaceSliceBundle,
        source_class: SourceClass::WorkspaceSearchResult,
        trust_posture: TrustPosture::TrustedFirstParty,
        selection_reason: SelectionReasonClass::SearchResultPacket,
        status: AttachmentStatusClass::OutOfScope,
        estimated_byte_size: 256,
        display_label: "payments route result".to_owned(),
        scope_truth: Some(ai_scope_truth()),
        placed_under_fenced_role: false,
    });

    let snapshot = AiContextInspectorSnapshot::project(&draft);
    let attachments = snapshot
        .section(InspectorSectionId::Attachments)
        .expect("attachments section");
    let row = attachments
        .rows
        .iter()
        .find(|row| row.row_id == "attachment_att.search_result.outside_scope")
        .expect("scope-truth attachment row");

    assert_eq!(row.status, InspectorRowStatusClass::Blocked);
    assert_eq!(
        row.blocked_reason_token.as_deref(),
        Some("out_of_scope_attachment")
    );
    assert!(row.value.contains("Outside current scope"));
    assert!(row.value.contains("Selected workset"));
    assert!(row.value.contains("partial_truth"));
    assert!(row.value.contains("authoritative_live"));
}

#[test]
fn route_placeholder_renders_dispatch_disabled_marker_on_every_row() {
    let draft = baseline_draft();
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    let route = snapshot
        .section(InspectorSectionId::RoutePlaceholder)
        .expect("route section");
    for row in &route.rows {
        if row.row_id == "seed_note" {
            continue;
        }
        assert_eq!(row.status, InspectorRowStatusClass::DispatchDisabled);
    }
}

#[test]
fn tainted_attachment_failure_drill_lights_chip_and_addresses_the_attachment() {
    // Failure drill: a draft pastes free-form text with an untrusted
    // posture. The inspector MUST surface the typed block reason on the
    // attachment row and on the block-reason section, and the snapshot's
    // top-level chip MUST tell the user the wedge carries tainted context.
    let mut draft = baseline_draft();
    draft.add_attachment(ComposerAttachment {
        attachment_id: "att.tainted_user_paste".to_owned(),
        kind: AttachmentKind::UserSuppliedText,
        source_class: SourceClass::UserSuppliedText,
        trust_posture: TrustPosture::UntrustedUserSupplied,
        selection_reason: SelectionReasonClass::UserPastedFreeformText,
        status: AttachmentStatusClass::TaintedOutsideFencedSection,
        estimated_byte_size: 512,
        display_label: "Pasted instructions from external chat".to_owned(),
        scope_truth: None,
        placed_under_fenced_role: false,
    });
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    assert!(snapshot.has_actionable_blocks);
    assert!(snapshot.has_tainted_attachments);

    let attachments = snapshot
        .section(InspectorSectionId::Attachments)
        .expect("attachments section");
    let row = attachments
        .rows
        .iter()
        .find(|row| row.row_id == "attachment_att.tainted_user_paste")
        .expect("tainted row");
    assert_eq!(row.status, InspectorRowStatusClass::Blocked);
    assert_eq!(
        row.blocked_reason_token.as_deref(),
        Some("tainted_attachment_outside_fenced_section")
    );
    assert!(matches!(
        row.address,
        InspectorRowAddress::Attachment { ref attachment_id } if attachment_id == "att.tainted_user_paste"
    ));

    let blocks = snapshot
        .section(InspectorSectionId::BlockReasons)
        .expect("block reasons section");
    let tainted_block = blocks
        .rows
        .iter()
        .find(|row| {
            row.blocked_reason_token.as_deref() == Some("tainted_attachment_outside_fenced_section")
        })
        .expect("tainted block reason row");
    assert!(matches!(
        tainted_block.address,
        InspectorRowAddress::Attachment { ref attachment_id } if attachment_id == "att.tainted_user_paste"
    ));
    assert_eq!(tainted_block.status, InspectorRowStatusClass::Blocked);
}

#[test]
fn unresolved_mention_renders_with_blocked_status_and_routes_to_mention_id() {
    let mut draft = baseline_draft();
    draft.add_mention(ComposerMention {
        mention_id: "mention.missing_file".to_owned(),
        kind: MentionKind::FileMention,
        target_stable_id: None,
        display_label: "@missing.rs".to_owned(),
        resolution_state: MentionResolutionState::UnresolvedNotFound,
    });
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    let mentions = snapshot
        .section(InspectorSectionId::Mentions)
        .expect("mentions section");
    let row = mentions
        .rows
        .iter()
        .find(|row| row.row_id == "mention_mention.missing_file")
        .expect("unresolved mention row");
    assert_eq!(row.status, InspectorRowStatusClass::Blocked);
    assert_eq!(
        row.blocked_reason_token.as_deref(),
        Some("unresolved_not_found")
    );
    assert!(matches!(
        row.address,
        InspectorRowAddress::Mention { ref mention_id } if mention_id == "mention.missing_file"
    ));
}

#[test]
fn resolved_slash_command_quotes_canonical_command_id_from_seeded_registry() {
    let mut draft = baseline_draft();
    let registry = seeded_registry();
    draft.add_slash_command(ComposerSlashCommandInvocation::resolve_in_registry(
        "invocation.open_folder",
        "cmd:workspace.open_folder",
        "/open-folder",
        registry,
    ));
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    let slashes = snapshot
        .section(InspectorSectionId::SlashCommands)
        .expect("slash commands section");
    let row = slashes
        .rows
        .iter()
        .find(|row| row.row_id == "invocation_invocation.open_folder")
        .expect("slash command row");
    assert_eq!(row.status, InspectorRowStatusClass::Live);
    assert_eq!(row.value, "cmd:workspace.open_folder");
    assert_eq!(
        row.value_token.as_deref(),
        Some("cmd:workspace.open_folder")
    );
}

#[test]
fn unresolved_slash_command_renders_blocked_with_typed_reason() {
    let mut draft = baseline_draft();
    let registry = seeded_registry();
    draft.add_slash_command(ComposerSlashCommandInvocation::resolve_in_registry(
        "invocation.bogus",
        "cmd:does_not_exist",
        "/does-not-exist",
        registry,
    ));
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    let slashes = snapshot
        .section(InspectorSectionId::SlashCommands)
        .expect("slash commands section");
    let row = slashes
        .rows
        .iter()
        .find(|row| row.row_id == "invocation_invocation.bogus")
        .expect("slash command row");
    assert_eq!(row.status, InspectorRowStatusClass::Blocked);
    assert_eq!(
        row.blocked_reason_token.as_deref(),
        Some("unresolved_no_match")
    );
}

#[test]
fn render_plaintext_includes_dispatch_disabled_marker_and_section_headings() {
    let draft = baseline_draft();
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    let block = snapshot.render_plaintext();
    assert!(block.contains("AI composer / context inspector"));
    assert!(block.contains("[Prototype wedge]"));
    assert!(block.contains("[Intent]"));
    assert!(block.contains("[Route placeholder]"));
    assert!(block.contains("[dispatch_disabled]"));
    assert!(block.contains("policy_blocked_route"));
}

#[test]
fn draft_state_section_quotes_dispatch_disabled_label_for_m1_seed() {
    let draft = baseline_draft();
    let snapshot = AiContextInspectorSnapshot::project(&draft);
    let state = snapshot
        .section(InspectorSectionId::DraftState)
        .expect("draft state section");
    let row = &state.rows[0];
    assert_eq!(
        row.value_token.as_deref(),
        Some("dispatch_disabled_in_m1_seed")
    );
    assert_eq!(row.status, InspectorRowStatusClass::DispatchDisabled);
}
