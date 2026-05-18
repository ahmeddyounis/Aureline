use std::path::Path;

use aureline_editor::{
    AssistSessionStore, AssistSourceDescriptor, AssistSourceFamily, AssistSourceLabelClass,
    AssistSurfaceSnapshot, AssistSurfaceSnapshotRequest, AssistSurfaceStateClass,
    CompletionAcceptanceContract, CompletionItemInit, CompletionItemKindClass,
    CompletionItemRecord, CompletionSideEffectClass, SignatureHelpInit, SignatureHelpRecord,
    SignaturePlacementClass, SnippetCursorPostureClass, SnippetImePostureClass,
    SnippetKeyIntentClass, SnippetKeyOutcomeClass, SnippetSessionController, SnippetSessionInit,
    SnippetSessionRecord, SnippetSessionStateClass, SnippetTabBehaviorClass, ASSIST_SCHEMA_VERSION,
};
use aureline_language::{
    LanguageServerHostIdentity, LanguageServerHostStatus, LspRouter, RouterCapabilityClass,
    RouterCompletenessClass, RouterFallbackClass, RouterFaultDomainId, RouterFreshnessClass,
    RouterHealthState, RouterLocalityClass, RouterRequest, RouterScopeClaimClass,
    RouterSurfaceClass, WorkspaceLocalRouterRequest,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Fixture {
    record_kind: String,
    schema_version: u32,
    workspace_id: String,
    document_ref: String,
    language_id: String,
    root_ref: String,
    execution_context_id: String,
    completion_session_id: String,
    assist_session_id: String,
    signature_help_id: String,
    snippet_session_id: String,
    snapshot_id: String,
    request_anchor_ref: String,
    captured_at: String,
    ready_router: RouterCase,
    fallback_router: RouterCase,
    completion_items: Vec<CompletionItemCase>,
    signature_help: SignatureHelpCase,
    snippet_session: SnippetSessionCase,
    key_sequence: Vec<KeySequenceCase>,
    expected: Expected,
}

#[derive(Debug, Deserialize)]
struct RouterCase {
    language_id: String,
    server_label: String,
    provider_id: String,
    host_health: RouterHealthState,
}

#[derive(Debug, Deserialize)]
struct CompletionItemCase {
    case_id: String,
    completion_item_id: String,
    label: String,
    kind_class: CompletionItemKindClass,
    source_route: String,
    insert_text_ref: String,
    rank: u32,
    sort_group: u32,
    side_effect_class: CompletionSideEffectClass,
    preview_required: bool,
    additional_edit_summary: Option<String>,
    expected_source_family: AssistSourceFamily,
    expected_source_label_class: AssistSourceLabelClass,
    expected_source_label: String,
    expected_disclosure: bool,
}

#[derive(Debug, Deserialize)]
struct SignatureHelpCase {
    active_signature_index: u32,
    signature_count: u32,
    active_parameter_index: u32,
    parameter_count: u32,
    placement_class: SignaturePlacementClass,
    non_blocking: bool,
    ime_composition_safe: bool,
    expected_source_family: AssistSourceFamily,
    expected_typing_loop_safe: bool,
}

#[derive(Debug, Deserialize)]
struct SnippetSessionCase {
    state_class: SnippetSessionStateClass,
    active_placeholder_index: Option<u32>,
    placeholder_count: u32,
    selection_count: u32,
    multi_cursor_compatible: bool,
    tab_behavior_class: SnippetTabBehaviorClass,
    ime_posture_class: SnippetImePostureClass,
    cursor_posture_class: SnippetCursorPostureClass,
    primary_caret_ref: Option<String>,
    visible_strip_required: bool,
    expected_source_family: AssistSourceFamily,
    expected_visible: bool,
    expected_can_escape: bool,
    expected_captures_tab: bool,
    expected_keyboard_ime_safe: bool,
    expected_composition_disclosure: bool,
}

#[derive(Debug, Deserialize)]
struct KeySequenceCase {
    intent: SnippetKeyIntentClass,
    expected_outcome: SnippetKeyOutcomeClass,
    expected_state: SnippetSessionStateClass,
    expected_placeholder_index: Option<u32>,
    expected_command_id_ref: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Expected {
    completion_total_count: usize,
    deterministic_language_completion_count: usize,
    lsp_completion_count: usize,
    fallback_completion_count: usize,
    snippet_completion_count: usize,
    ai_assist_completion_count: usize,
    preview_required_count: usize,
    active_snippet_session_count: usize,
    signature_help_count: usize,
    source_label_count: usize,
    source_families: Vec<AssistSourceFamily>,
    source_label_classes: Vec<AssistSourceLabelClass>,
    completion_disclosure_required: bool,
    surface_disclosure_required: bool,
    surface_state_class: AssistSurfaceStateClass,
}

#[test]
fn assist_sessions_preserve_sources_and_snippet_key_contracts() {
    let fixture = load_fixture();
    assert_eq!(fixture.record_kind, "assist_sessions_alpha_cases");
    assert_eq!(fixture.schema_version, ASSIST_SCHEMA_VERSION);

    let ready_completion_decision = route(
        &fixture,
        &fixture.ready_router,
        RouterSurfaceClass::Completion,
        RouterCapabilityClass::Completion,
    );
    let fallback_completion_decision = route(
        &fixture,
        &fixture.fallback_router,
        RouterSurfaceClass::Completion,
        RouterCapabilityClass::Completion,
    );
    let signature_decision = route(
        &fixture,
        &fixture.ready_router,
        RouterSurfaceClass::SignatureHelp,
        RouterCapabilityClass::SignatureHelp,
    );

    let ready_completion_source =
        AssistSourceDescriptor::from_router_decision(&ready_completion_decision)
            .expect("ready completion source resolves");
    let fallback_completion_source =
        AssistSourceDescriptor::from_router_decision(&fallback_completion_decision)
            .expect("fallback completion source resolves");
    let signature_source = AssistSourceDescriptor::from_router_decision(&signature_decision)
        .expect("signature source resolves");
    let snippet_source = snippet_source();

    let mut store = AssistSessionStore::new();
    for case in &fixture.completion_items {
        let source = match case.source_route.as_str() {
            "ready_router" => ready_completion_source.clone(),
            "fallback_router" => fallback_completion_source.clone(),
            "snippet" => snippet_source.clone(),
            "ai_inline" => ai_source(),
            other => panic!("unknown completion source route {other}"),
        };
        let item = completion_item(&fixture, case, source);

        assert_eq!(
            item.source.source_family, case.expected_source_family,
            "source family mismatch for {}",
            case.case_id
        );
        assert_eq!(
            item.source.source_label_class, case.expected_source_label_class,
            "source label class mismatch for {}",
            case.case_id
        );
        assert_eq!(
            item.source.source_label, case.expected_source_label,
            "source label mismatch for {}",
            case.case_id
        );
        assert_eq!(
            item.requires_degraded_disclosure(),
            case.expected_disclosure,
            "disclosure mismatch for {}",
            case.case_id
        );
        store.publish_completion_item(item);
    }

    let signature_help = SignatureHelpRecord::new(SignatureHelpInit {
        signature_help_id: fixture.signature_help_id.clone(),
        assist_session_id: fixture.assist_session_id.clone(),
        document_ref: fixture.document_ref.clone(),
        language_id: fixture.language_id.clone(),
        invocation_anchor_ref: "anchor:webapp:checkout-form:call:create-order".into(),
        source: signature_source,
        active_signature_index: fixture.signature_help.active_signature_index,
        signature_count: fixture.signature_help.signature_count,
        active_parameter_index: fixture.signature_help.active_parameter_index,
        parameter_count: fixture.signature_help.parameter_count,
        placement_class: fixture.signature_help.placement_class,
        non_blocking: fixture.signature_help.non_blocking,
        ime_composition_safe: fixture.signature_help.ime_composition_safe,
        dismiss_command_id_ref: "cmd:editor.signatureHelp.dismiss".into(),
        captured_at: fixture.captured_at.clone(),
    });
    assert_eq!(
        signature_help.source.source_family,
        fixture.signature_help.expected_source_family
    );
    assert_eq!(
        signature_help.is_typing_loop_safe(),
        fixture.signature_help.expected_typing_loop_safe
    );
    store.set_signature_help(signature_help);

    let snippet_session = snippet_session(&fixture, snippet_source);
    assert_eq!(
        snippet_session.source.source_family,
        fixture.snippet_session.expected_source_family
    );
    assert_eq!(
        snippet_session.is_visible(),
        fixture.snippet_session.expected_visible
    );
    assert_eq!(
        snippet_session.can_escape(),
        fixture.snippet_session.expected_can_escape
    );
    assert_eq!(
        snippet_session.captures_tab(),
        fixture.snippet_session.expected_captures_tab
    );
    assert_eq!(
        snippet_session.is_keyboard_and_ime_safe(),
        fixture.snippet_session.expected_keyboard_ime_safe
    );
    assert_eq!(
        snippet_session.requires_composition_disclosure(),
        fixture.snippet_session.expected_composition_disclosure
    );
    store.set_snippet_session(snippet_session.clone());

    let snapshot = store.surface_snapshot(AssistSurfaceSnapshotRequest {
        snapshot_id: fixture.snapshot_id.clone(),
        workspace_id: fixture.workspace_id.clone(),
        document_ref: fixture.document_ref.clone(),
        language_id: fixture.language_id.clone(),
        completion_session_id: fixture.completion_session_id.clone(),
        request_anchor_ref: fixture.request_anchor_ref.clone(),
        captured_at: fixture.captured_at.clone(),
    });

    assert_eq!(snapshot.record_kind, AssistSurfaceSnapshot::RECORD_KIND);
    assert_eq!(
        snapshot
            .completion_list
            .source_counts
            .completion_total_count,
        fixture.expected.completion_total_count
    );
    assert_eq!(
        snapshot
            .completion_list
            .source_counts
            .deterministic_language_completion_count,
        fixture.expected.deterministic_language_completion_count
    );
    assert_eq!(
        snapshot.source_counts.lsp_completion_count,
        fixture.expected.lsp_completion_count
    );
    assert_eq!(
        snapshot.source_counts.fallback_completion_count,
        fixture.expected.fallback_completion_count
    );
    assert_eq!(
        snapshot.source_counts.snippet_completion_count,
        fixture.expected.snippet_completion_count
    );
    assert_eq!(
        snapshot.source_counts.ai_assist_completion_count,
        fixture.expected.ai_assist_completion_count
    );
    assert_eq!(
        snapshot.source_counts.preview_required_count,
        fixture.expected.preview_required_count
    );
    assert_eq!(
        snapshot.source_counts.active_snippet_session_count,
        fixture.expected.active_snippet_session_count
    );
    assert_eq!(
        snapshot.source_counts.signature_help_count,
        fixture.expected.signature_help_count
    );
    assert_eq!(
        snapshot.source_counts.source_label_count,
        fixture.expected.source_label_count
    );
    assert_eq!(
        snapshot.source_counts.source_families,
        fixture.expected.source_families
    );
    assert_eq!(
        snapshot.source_counts.source_label_classes,
        fixture.expected.source_label_classes
    );
    assert_eq!(
        snapshot.completion_list.disclosure_required,
        fixture.expected.completion_disclosure_required
    );
    assert_eq!(
        snapshot.disclosure_required,
        fixture.expected.surface_disclosure_required
    );
    assert_eq!(snapshot.state_class, fixture.expected.surface_state_class);
    assert!(!snapshot.completion_list.router_decision_refs.is_empty());
    let ai_projection = ai_source().label_projection(&fixture.captured_at);
    assert_eq!(
        ai_projection.source_label_class,
        AssistSourceLabelClass::AiInlineAssist
    );
    assert!(ai_projection.requires_visual_distinction);

    let serialized = serde_json::to_string(&snapshot).expect("snapshot serializes");
    let round_trip: AssistSurfaceSnapshot =
        serde_json::from_str(&serialized).expect("snapshot deserializes");
    assert_eq!(round_trip, snapshot);

    let mut ime_composing_session = snippet_session.clone();
    ime_composing_session.ime_posture_class = SnippetImePostureClass::CompositionActivePassThrough;
    let mut ime_controller = SnippetSessionController::new(ime_composing_session);
    let ime_tab = ime_controller.handle_key(SnippetKeyIntentClass::Tab);
    assert_eq!(ime_tab.outcome_class, SnippetKeyOutcomeClass::PassThrough);
    assert_eq!(
        ime_tab.resulting_state_class,
        SnippetSessionStateClass::Active
    );
    assert_eq!(ime_tab.resulting_placeholder_index, Some(1));

    let mut controller = SnippetSessionController::new(snippet_session);
    for step in &fixture.key_sequence {
        let outcome = controller.handle_key(step.intent);
        assert_eq!(outcome.outcome_class, step.expected_outcome);
        assert_eq!(outcome.resulting_state_class, step.expected_state);
        assert_eq!(
            outcome.resulting_placeholder_index,
            step.expected_placeholder_index
        );
        assert_eq!(outcome.command_id_ref, step.expected_command_id_ref);
    }

    let inactive_outcome = controller.handle_key(SnippetKeyIntentClass::CommandShortcut);
    assert_eq!(
        inactive_outcome.outcome_class,
        SnippetKeyOutcomeClass::PassThrough
    );
    assert_eq!(
        inactive_outcome.resulting_state_class,
        SnippetSessionStateClass::Cancelled
    );

    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/assist/completion_item.schema.json");
    let schema_payload = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|err| panic!("read {schema_path:?}: {err}"));
    let schema: serde_json::Value = serde_json::from_str(&schema_payload)
        .unwrap_or_else(|err| panic!("parse {schema_path:?}: {err}"));
    assert_eq!(
        schema["$id"],
        "https://aureline.dev/schemas/assist/completion_item.schema.json"
    );
}

fn route(
    fixture: &Fixture,
    router_case: &RouterCase,
    surface_class: RouterSurfaceClass,
    capability_class: RouterCapabilityClass,
) -> aureline_language::RouterDecisionRecord {
    let router = LspRouter::new();
    let request = RouterRequest::workspace_local(WorkspaceLocalRouterRequest {
        language_id: router_case.language_id.clone(),
        surface_class,
        capability_class,
        requested_subject_ref: fixture.document_ref.clone(),
        workspace_id: fixture.workspace_id.clone(),
        root_ref: fixture.root_ref.clone(),
        execution_context_id: fixture.execution_context_id.clone(),
        captured_at: fixture.captured_at.clone(),
    });
    router.route(
        request,
        &[host_status(fixture, router_case, capability_class)],
    )
}

fn host_status(
    fixture: &Fixture,
    router_case: &RouterCase,
    capability_class: RouterCapabilityClass,
) -> LanguageServerHostStatus {
    LanguageServerHostStatus {
        identity: LanguageServerHostIdentity {
            host_instance_id: format!("host:lsp:{}", router_case.language_id),
            provider_id: router_case.provider_id.clone(),
            workspace_id: fixture.workspace_id.clone(),
            root_ref: fixture.root_ref.clone(),
            language_id: router_case.language_id.clone(),
            server_label: router_case.server_label.clone(),
            execution_context_id: fixture.execution_context_id.clone(),
            locality_class: RouterLocalityClass::LocalSidecar,
            fault_domain_id: RouterFaultDomainId::SessionScopedExecutionHosts,
            restart_budget_ref: "restart_budget:session_scoped_execution_hosts:language:assist"
                .into(),
        },
        health_state: router_case.host_health,
        freshness_class: freshness_for(router_case.host_health),
        scope_claim_class: RouterScopeClaimClass::ActiveWorkset,
        completeness_class: completeness_for(router_case.host_health),
        scope_limit_classes: Vec::new(),
        supported_capability_classes: vec![capability_class],
        restart_strike_count: 0,
        quarantine_ref: None,
        fallback_class: RouterFallbackClass::ProtocolToText,
        health_summary: format!(
            "{} is {:?} for assist routing.",
            router_case.server_label, router_case.host_health
        ),
    }
}

fn completion_item(
    fixture: &Fixture,
    case: &CompletionItemCase,
    source: AssistSourceDescriptor,
) -> CompletionItemRecord {
    CompletionItemRecord::new(CompletionItemInit {
        completion_item_id: case.completion_item_id.clone(),
        completion_session_id: fixture.completion_session_id.clone(),
        label: case.label.clone(),
        kind_class: case.kind_class,
        source,
        insert_text_ref: case.insert_text_ref.clone(),
        rank: case.rank,
        sort_group: case.sort_group,
        acceptance: CompletionAcceptanceContract {
            accept_command_id_ref: "cmd:editor.completion.accept".into(),
            detail_command_id_ref: "cmd:editor.completion.openDetail".into(),
            side_effect_class: case.side_effect_class,
            preview_required: case.preview_required,
            undo_group_label: "Accept completion".into(),
            additional_edit_summary: case.additional_edit_summary.clone(),
        },
        captured_at: fixture.captured_at.clone(),
    })
}

fn snippet_source() -> AssistSourceDescriptor {
    AssistSourceDescriptor::snippet(
        "assist-source:snippet:webapp:workspace",
        "Workspace snippets",
        "snippet-pack:webapp:workspace",
        RouterScopeClaimClass::ActiveWorkset,
        RouterLocalityClass::LocalInProcess,
        "Workspace snippets are loaded from the active profile and workspace snippet scopes.",
    )
}

fn ai_source() -> AssistSourceDescriptor {
    AssistSourceDescriptor::ai_inline(
        "assist-source:ai:inline:webapp",
        "AI inline assist",
        "ai-inline-assist:webapp:local-policy",
        RouterLocalityClass::LocalSidecar,
        "AI inline assist is advisory and must remain visually distinct from deterministic completion.",
    )
}

fn snippet_session(fixture: &Fixture, source: AssistSourceDescriptor) -> SnippetSessionRecord {
    SnippetSessionRecord::new(SnippetSessionInit {
        snippet_session_id: fixture.snippet_session_id.clone(),
        document_ref: fixture.document_ref.clone(),
        language_id: fixture.language_id.clone(),
        source,
        state_class: fixture.snippet_session.state_class,
        active_placeholder_index: fixture.snippet_session.active_placeholder_index,
        placeholder_count: fixture.snippet_session.placeholder_count,
        selection_count: fixture.snippet_session.selection_count,
        multi_cursor_compatible: fixture.snippet_session.multi_cursor_compatible,
        tab_behavior_class: fixture.snippet_session.tab_behavior_class,
        ime_posture_class: fixture.snippet_session.ime_posture_class,
        cursor_posture_class: fixture.snippet_session.cursor_posture_class,
        primary_caret_ref: fixture.snippet_session.primary_caret_ref.clone(),
        next_placeholder_command_id_ref: "cmd:editor.snippet.nextPlaceholder".into(),
        previous_placeholder_command_id_ref: "cmd:editor.snippet.previousPlaceholder".into(),
        exit_command_id_ref: "cmd:editor.snippet.exit".into(),
        escape_command_id_ref: "cmd:editor.snippet.cancel".into(),
        visible_strip_required: fixture.snippet_session.visible_strip_required,
        captured_at: fixture.captured_at.clone(),
    })
}

fn freshness_for(health: RouterHealthState) -> RouterFreshnessClass {
    match health {
        RouterHealthState::Ready => RouterFreshnessClass::AuthoritativeLive,
        RouterHealthState::Warming
        | RouterHealthState::Degraded
        | RouterHealthState::CachedOnly => RouterFreshnessClass::DegradedCached,
        RouterHealthState::PolicyBlocked
        | RouterHealthState::CapabilityMissing
        | RouterHealthState::CrashLoopQuarantined
        | RouterHealthState::Unavailable => RouterFreshnessClass::Unverified,
    }
}

fn completeness_for(health: RouterHealthState) -> RouterCompletenessClass {
    match health {
        RouterHealthState::Ready => RouterCompletenessClass::CompleteForClaimedScope,
        RouterHealthState::Warming
        | RouterHealthState::Degraded
        | RouterHealthState::CachedOnly => RouterCompletenessClass::PartialForClaimedScope,
        RouterHealthState::PolicyBlocked
        | RouterHealthState::CapabilityMissing
        | RouterHealthState::CrashLoopQuarantined
        | RouterHealthState::Unavailable => RouterCompletenessClass::UnavailableForClaimedScope,
    }
}

fn load_fixture() -> Fixture {
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/editor/assist_sessions_alpha/session_cases.json");
    let payload =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {path:?}: {err}"));
    serde_json::from_str(&payload).unwrap_or_else(|err| panic!("parse {path:?}: {err}"))
}
