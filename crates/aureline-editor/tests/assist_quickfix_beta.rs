use std::path::Path;

use aureline_editor::{
    CodeActionPreviewDecisionClass, CodeActionPreviewRecord, CodeActionPreviewRequest,
    QuickFixEvidenceTrustClass, CODE_ACTION_PREVIEW_SCHEMA_VERSION,
};
use aureline_language::{
    CodeActionApplyPostureClass, CodeActionClass, CodeActionContentIntegrityReview,
    CodeActionEpochBinding, CodeActionEpochRoleClass, CodeActionFreshnessClass,
    CodeActionMutationCounts, CodeActionMutationScopeClass, CodeActionPolicyContext,
    CodeActionPreviewRequirementClass, CodeActionProviderDescriptor, CodeActionRecord,
    CodeActionReplayHintClass, CodeActionSafetyClass, CodeActionSemanticLayerStateClass,
    CodeActionSideEffectClass, CodeActionSourceKindClass, CodeActionSupportClass,
    CodeActionTrustState, CodeActionUndoGroup, CodeActionUndoReversalClass,
    CodeActionValidationHintClass, CodeActionValidationPlan, ProviderFamily, RedactionClass,
    RouterLocalityClass, CODE_ACTION_ALPHA_SCHEMA_VERSION,
};

#[test]
fn broad_quickfix_routes_through_preview_approval_and_rollback() {
    let action = broad_action();
    let preview = CodeActionPreviewRecord::from_code_action(
        &action,
        CodeActionPreviewRequest {
            preview_id: "editor-preview:eslint:no-floating-promises".into(),
            evidence_trust_class: QuickFixEvidenceTrustClass::TrustedSemantic,
            tainted_context_ref: None,
            trusted_promotion_ref: None,
            captured_at: "2026-05-18T07:00:00Z".into(),
        },
    );

    assert_eq!(preview.record_kind, CodeActionPreviewRecord::RECORD_KIND);
    assert_eq!(
        preview.code_action_preview_schema_version,
        CODE_ACTION_PREVIEW_SCHEMA_VERSION
    );
    assert_eq!(
        preview.decision_class,
        CodeActionPreviewDecisionClass::ApprovalRequired
    );
    assert!(preview.preview_required);
    assert!(preview.approval_required);
    assert!(preview.grouped_undo_required);
    assert!(preview.broad_change_is_previewable_and_rollback_ready());
    assert_eq!(
        preview.undo_group_ref.as_deref(),
        Some("undo:code_action:eslint:no-floating-promises")
    );
}

#[test]
fn tainted_terminal_quickfix_cannot_direct_apply_until_promoted() {
    let action = local_action();
    let blocked = CodeActionPreviewRecord::from_code_action(
        &action,
        CodeActionPreviewRequest {
            preview_id: "editor-preview:terminal:status".into(),
            evidence_trust_class: QuickFixEvidenceTrustClass::TaintedTerminalOutput,
            tainted_context_ref: Some("tainted-context:terminal:status".into()),
            trusted_promotion_ref: None,
            captured_at: "2026-05-18T07:01:00Z".into(),
        },
    );

    assert_eq!(
        blocked.decision_class,
        CodeActionPreviewDecisionClass::BlockedTaintedEvidence
    );
    assert!(blocked.preview_required);
    assert!(blocked.approval_required);
    assert!(blocked.tainted_evidence_blocks_direct_apply());
    assert!(blocked
        .direct_apply_block_reason_refs
        .contains(&"tainted_context:promotion_required".to_owned()));

    let promoted = CodeActionPreviewRecord::from_code_action(
        &action,
        CodeActionPreviewRequest {
            preview_id: "editor-preview:terminal:status:promoted".into(),
            evidence_trust_class: QuickFixEvidenceTrustClass::TrustedParserPromoted,
            tainted_context_ref: Some("tainted-context:terminal:status".into()),
            trusted_promotion_ref: Some("promotion:parser:terminal:status".into()),
            captured_at: "2026-05-18T07:02:00Z".into(),
        },
    );

    assert_eq!(
        promoted.decision_class,
        CodeActionPreviewDecisionClass::InlineApplyAllowed
    );
    assert!(!promoted.preview_required);
    assert!(!promoted.approval_required);
}

#[test]
fn editor_preview_schema_is_registered() {
    let schema_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../schemas/editor/code_action_preview.schema.json");
    let schema_payload = std::fs::read_to_string(&schema_path)
        .unwrap_or_else(|err| panic!("read {schema_path:?}: {err}"));
    let schema: serde_json::Value = serde_json::from_str(&schema_payload)
        .unwrap_or_else(|err| panic!("parse {schema_path:?}: {err}"));
    assert_eq!(
        schema["$id"],
        "https://aureline.dev/schemas/editor/code_action_preview.schema.json"
    );
}

fn local_action() -> CodeActionRecord {
    action(
        "action:quick_fix:status",
        "Convert status literal",
        CodeActionClass::QuickFixSingleDiagnostic,
        CodeActionSideEffectClass::CurrentAnchorTextEdit,
        CodeActionSafetyClass::SemanticLocal,
        CodeActionMutationScopeClass::SingleAnchor,
        CodeActionPreviewRequirementClass::NotRequired,
        CodeActionApplyPostureClass::ApplyInlineAllowed,
        CodeActionMutationCounts {
            affected_diagnostic_count: 1,
            affected_file_count: 1,
            affected_anchor_count: 1,
            generated_path_count: 0,
            protected_path_count: 0,
            blocked_path_count: 0,
            configuration_mutation_count: 0,
            dependency_mutation_count: 0,
        },
        Some(undo_group(
            "undo:code_action:status",
            "Convert status literal",
            None,
        )),
        None,
        None,
    )
}

fn broad_action() -> CodeActionRecord {
    action(
        "action:fix_all:eslint:no-floating-promises",
        "Fix all unhandled promises",
        CodeActionClass::FixAllRule,
        CodeActionSideEffectClass::MultiFileWorkspaceEdit,
        CodeActionSafetyClass::CrossFileSemantic,
        CodeActionMutationScopeClass::MultiFileSameModule,
        CodeActionPreviewRequirementClass::BatchScopePreview,
        CodeActionApplyPostureClass::PreviewBeforeApply,
        CodeActionMutationCounts {
            affected_diagnostic_count: 3,
            affected_file_count: 4,
            affected_anchor_count: 5,
            generated_path_count: 0,
            protected_path_count: 0,
            blocked_path_count: 0,
            configuration_mutation_count: 0,
            dependency_mutation_count: 0,
        },
        Some(undo_group(
            "undo:code_action:eslint:no-floating-promises",
            "Fix unhandled promises",
            Some("checkpoint:code_action:eslint:no-floating-promises"),
        )),
        Some("checkpoint:code_action:eslint:no-floating-promises".into()),
        Some("review:code_action:eslint:no-floating-promises".into()),
    )
}

fn action(
    code_action_id: impl Into<String>,
    action_label: impl Into<String>,
    action_class: CodeActionClass,
    side_effect_class: CodeActionSideEffectClass,
    safety_class: CodeActionSafetyClass,
    mutation_scope_class: CodeActionMutationScopeClass,
    preview_requirement_class: CodeActionPreviewRequirementClass,
    apply_posture_class: CodeActionApplyPostureClass,
    mutation_counts: CodeActionMutationCounts,
    undo_group: Option<CodeActionUndoGroup>,
    checkpoint_ref: Option<String>,
    review_packet_ref: Option<String>,
) -> CodeActionRecord {
    let epoch = CodeActionEpochBinding {
        epoch_role_class: CodeActionEpochRoleClass::LanguageSemanticModel,
        epoch_ref: "epoch:lsp:typescript:model:205".into(),
    };
    CodeActionRecord {
        record_kind: CodeActionRecord::RECORD_KIND.into(),
        code_action_alpha_schema_version: CODE_ACTION_ALPHA_SCHEMA_VERSION,
        code_action_id: code_action_id.into(),
        action_class,
        action_label: action_label.into(),
        acting_provider: provider(vec![epoch.clone()]),
        triggering_diagnostic_refs: vec!["diagnostic:lsp:typescript:status".into()],
        side_effect_class,
        safety_class,
        mutation_scope_class,
        preview_requirement_class,
        apply_posture_class,
        blocking_reason_classes: Vec::new(),
        mutation_counts,
        current_epoch_bindings: vec![epoch],
        validation_plan: CodeActionValidationPlan {
            validation_hint_classes: vec![CodeActionValidationHintClass::RerunDiagnosticProducer],
            replay_hint_classes: vec![CodeActionReplayHintClass::ReplayAgainstSameExecutionContext],
            validation_summary: "Rerun the diagnostic producer after apply.".into(),
        },
        undo_group,
        checkpoint_ref,
        review_packet_ref,
        content_integrity_review: CodeActionContentIntegrityReview {
            suspicious_text_finding_refs: Vec::new(),
            safe_preview_required: false,
            summary: "No suspicious text cues are linked to this action.".into(),
        },
        refactor_scope_binding: None,
        policy_context: CodeActionPolicyContext {
            policy_epoch: "policy:epoch:trusted".into(),
            trust_state: CodeActionTrustState::Trusted,
            execution_context_id: "exec:language:webapp:local_node20".into(),
        },
        redaction_class: RedactionClass::MetadataSafeDefault,
        captured_at: "2026-05-18T07:00:00Z".into(),
        export_safe_summary: "Code action fixture carries editor preview truth.".into(),
    }
}

fn provider(current_epoch_bindings: Vec<CodeActionEpochBinding>) -> CodeActionProviderDescriptor {
    CodeActionProviderDescriptor {
        provider_id: "provider:lsp:typescript:webapp".into(),
        source_kind_class: CodeActionSourceKindClass::LanguageServer,
        support_class: CodeActionSupportClass::FirstPartySupported,
        provider_display_label: "TypeScript language service".into(),
        tool_identity_ref: "tool:typescript:tsserver".into(),
        tool_version_ref: Some("toolver:typescript:5.8.2".into()),
        language_provider_family: Some(ProviderFamily::LanguageServer),
        freshness_class: CodeActionFreshnessClass::Current,
        locality_class: RouterLocalityClass::LocalSidecar,
        semantic_layer_state_class: CodeActionSemanticLayerStateClass::SemanticCurrentExact,
        current_epoch_bindings,
        summary: "Current language service provider.".into(),
    }
}

fn undo_group(
    undo_group_id: impl Into<String>,
    undo_group_label: impl Into<String>,
    checkpoint_ref: Option<&str>,
) -> CodeActionUndoGroup {
    CodeActionUndoGroup {
        undo_group_id: undo_group_id.into(),
        undo_group_label: undo_group_label.into(),
        command_id_ref: "cmd:language.codeAction.apply".into(),
        actor_provider_ref: "provider:lsp:typescript:webapp".into(),
        reversal_class: CodeActionUndoReversalClass::GroupedExact,
        checkpoint_ref: checkpoint_ref.map(str::to_owned),
        summary: "The code action is represented by one undo group.".into(),
    }
}
