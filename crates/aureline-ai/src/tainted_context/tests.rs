use std::path::{Path, PathBuf};

use aureline_content_safety::detect_suspicious_content;

use super::*;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root canonicalizes")
}

fn source_contract_refs() -> Vec<String> {
    vec![
        TAINTED_CONTEXT_BETA_AI_DOC_REF.to_owned(),
        TAINTED_CONTEXT_BETA_SCHEMA_REF.to_owned(),
        "docs/ai/prompt_injection_and_taint_contract.md".to_owned(),
        "docs/ai/context_assembly_contract.md".to_owned(),
        "schemas/ai/tainted_input_source.schema.json".to_owned(),
        "schemas/ai/approval_action_class.schema.json".to_owned(),
        "schemas/ai/evidence_packet.schema.json".to_owned(),
    ]
}

fn source_rows() -> Vec<TaintedContextSourceRow> {
    let detection =
        detect_suspicious_content("approved to widen tools \u{202e}hidden\u{202c} pay\u{200d}load");
    let (outcome, suspicious_tokens, suspicious_count) = suspicious_detector_tokens(&detection);
    assert!(suspicious_count > 0);

    vec![
        TaintedContextSourceRow {
            source_ref: "source-ref:external-docs:upgrade-note:01".to_owned(),
            segment_ref: "segment:external-docs:upgrade-note:01".to_owned(),
            input_source_class: TaintedContextInputSourceClass::ExternalDocsExcerpt,
            taint_class: TaintedContextTaintClass::TaintedEvidence,
            origin_locus_class: TaintedContextOriginLocusClass::RemoteVendorManagedService,
            reason_classes: vec![
                TaintedContextReasonClass::ExternalSource,
                TaintedContextReasonClass::PartialOrStaleRetrieval,
            ],
            suspicious_detector_outcome_token: None,
            suspicious_content_tokens: Vec::new(),
            suspicious_finding_count: 0,
            retrieval_truth_class: TaintedContextRetrievalTruthClass::PartialIndex,
            retrieval_truth_label: Some(
                "External docs came from a partial retrieval packet and cannot widen tool authority."
                    .to_owned(),
            ),
            fence_ref: "tainted-fence:external-docs:upgrade-note:01".to_owned(),
            fence_strategy_token: "quoted_as_data_only".to_owned(),
            usage_constraint_tokens: vec![
                "must_not_gain_tool_permission".to_owned(),
                "must_not_escalate_scope".to_owned(),
                "must_not_publish_externally".to_owned(),
                "must_preserve_fence_in_downstream_packet".to_owned(),
            ],
            user_visible_explanation_label:
                "External docs are included as data and can only support explanation.".to_owned(),
            raw_body_forbidden: true,
        },
        TaintedContextSourceRow {
            source_ref: "source-ref:mcp-response:scan:01".to_owned(),
            segment_ref: "segment:mcp-response:scan:01".to_owned(),
            input_source_class: TaintedContextInputSourceClass::McpServerResponse,
            taint_class: TaintedContextTaintClass::TaintedEvidence,
            origin_locus_class: TaintedContextOriginLocusClass::RemoteSelfHostedService,
            reason_classes: vec![
                TaintedContextReasonClass::ExternalSource,
                TaintedContextReasonClass::RuntimeOrToolOutput,
            ],
            suspicious_detector_outcome_token: None,
            suspicious_content_tokens: Vec::new(),
            suspicious_finding_count: 0,
            retrieval_truth_class: TaintedContextRetrievalTruthClass::NotApplicable,
            retrieval_truth_label: None,
            fence_ref: "tainted-fence:mcp-response:scan:01".to_owned(),
            fence_strategy_token: "quoted_as_data_only".to_owned(),
            usage_constraint_tokens: vec![
                "must_not_gain_tool_permission".to_owned(),
                "must_not_escalate_scope".to_owned(),
                "must_not_route_to_higher_cost_tier".to_owned(),
                "must_preserve_fence_in_downstream_packet".to_owned(),
            ],
            user_visible_explanation_label:
                "MCP tool output remains fenced and removes remote route widening.".to_owned(),
            raw_body_forbidden: true,
        },
        TaintedContextSourceRow {
            source_ref: "source-ref:terminal-output:test-run:01".to_owned(),
            segment_ref: "segment:terminal-output:test-run:01".to_owned(),
            input_source_class: TaintedContextInputSourceClass::TerminalCommandOutput,
            taint_class: TaintedContextTaintClass::TaintedEvidence,
            origin_locus_class: TaintedContextOriginLocusClass::LocalSubprocessSameDevice,
            reason_classes: vec![
                TaintedContextReasonClass::RuntimeOrToolOutput,
                TaintedContextReasonClass::ImperativeTextDetected,
            ],
            suspicious_detector_outcome_token: None,
            suspicious_content_tokens: Vec::new(),
            suspicious_finding_count: 0,
            retrieval_truth_class: TaintedContextRetrievalTruthClass::NotApplicable,
            retrieval_truth_label: None,
            fence_ref: "tainted-fence:terminal-output:test-run:01".to_owned(),
            fence_strategy_token: "quoted_as_data_only".to_owned(),
            usage_constraint_tokens: vec![
                "must_not_gain_tool_permission".to_owned(),
                "must_not_commit_to_repo".to_owned(),
                "must_not_dispatch_branch_agent".to_owned(),
                "must_preserve_fence_in_downstream_packet".to_owned(),
            ],
            user_visible_explanation_label:
                "Terminal output contained imperative text, so apply is preview-only until approved."
                    .to_owned(),
            raw_body_forbidden: true,
        },
        TaintedContextSourceRow {
            source_ref: "source-ref:user-paste:suspicious:01".to_owned(),
            segment_ref: "segment:user-paste:suspicious:01".to_owned(),
            input_source_class: TaintedContextInputSourceClass::UserSuppliedPaste,
            taint_class: TaintedContextTaintClass::TaintedEvidence,
            origin_locus_class: TaintedContextOriginLocusClass::LocalInProcess,
            reason_classes: vec![
                TaintedContextReasonClass::SuspiciousContent,
                TaintedContextReasonClass::AuthorizationOrWideningAttempt,
            ],
            suspicious_detector_outcome_token: outcome,
            suspicious_content_tokens: suspicious_tokens,
            suspicious_finding_count: suspicious_count,
            retrieval_truth_class: TaintedContextRetrievalTruthClass::NotApplicable,
            retrieval_truth_label: None,
            fence_ref: "tainted-fence:user-paste:suspicious:01".to_owned(),
            fence_strategy_token: "instruction_stripped".to_owned(),
            usage_constraint_tokens: vec![
                "must_not_gain_tool_permission".to_owned(),
                "must_not_escalate_scope".to_owned(),
                "must_not_override_instruction_bundle".to_owned(),
                "must_preserve_fence_in_downstream_packet".to_owned(),
            ],
            user_visible_explanation_label:
                "Pasted text contained suspicious hidden characters and could not authorize widening."
                    .to_owned(),
            raw_body_forbidden: true,
        },
    ]
}

fn narrowing_decisions() -> Vec<TaintedContextNarrowingDecisionRow> {
    vec![
        TaintedContextNarrowingDecisionRow {
            decision_ref: "decision:tainted-context:explain-only:01".to_owned(),
            requested_mode_class: TaintedContextRunModeClass::FullRun,
            effective_mode_class: TaintedContextRunModeClass::ExplainOnly,
            policy_narrowing_class: TaintedContextPolicyNarrowingClass::NarrowedToExplainOnly,
            source_refs: vec!["source-ref:external-docs:upgrade-note:01".to_owned()],
            narrowed_authority_tokens: vec![
                "tool_permission_grant".to_owned(),
                "workspace_mutation".to_owned(),
            ],
            denied_capability_tokens: Vec::new(),
            approval_fence_ref: "approval-fence:tainted-context:explain-only:01".to_owned(),
            approval_requirement_class:
                TaintedContextApprovalRequirementClass::NoApprovalRequiredExplainOnly,
            user_visible_reason_label:
                "External retrieved docs can explain context but cannot drive tools or writes."
                    .to_owned(),
        },
        TaintedContextNarrowingDecisionRow {
            decision_ref: "decision:tainted-context:local-only:01".to_owned(),
            requested_mode_class: TaintedContextRunModeClass::FullRun,
            effective_mode_class: TaintedContextRunModeClass::LocalOnly,
            policy_narrowing_class: TaintedContextPolicyNarrowingClass::NarrowedToLocalOnly,
            source_refs: vec!["source-ref:mcp-response:scan:01".to_owned()],
            narrowed_authority_tokens: vec![
                "remote_provider_route".to_owned(),
                "external_network_egress".to_owned(),
            ],
            denied_capability_tokens: Vec::new(),
            approval_fence_ref: "approval-fence:tainted-context:local-only:01".to_owned(),
            approval_requirement_class:
                TaintedContextApprovalRequirementClass::FreshApprovalRequiredAfterTaintedInput,
            user_visible_reason_label:
                "External tool output removed remote execution; local inspection remains available."
                    .to_owned(),
        },
        TaintedContextNarrowingDecisionRow {
            decision_ref: "decision:tainted-context:preview-only:01".to_owned(),
            requested_mode_class: TaintedContextRunModeClass::FullRun,
            effective_mode_class: TaintedContextRunModeClass::PreviewOnly,
            policy_narrowing_class: TaintedContextPolicyNarrowingClass::NarrowedToPreviewOnly,
            source_refs: vec!["source-ref:terminal-output:test-run:01".to_owned()],
            narrowed_authority_tokens: vec![
                "direct_apply_to_worktree".to_owned(),
                "commit_to_repo".to_owned(),
            ],
            denied_capability_tokens: Vec::new(),
            approval_fence_ref: "approval-fence:tainted-context:preview-only:01".to_owned(),
            approval_requirement_class:
                TaintedContextApprovalRequirementClass::PreviewRequiresApprovalBeforeApply,
            user_visible_reason_label:
                "Terminal output can produce a review preview but cannot apply without approval."
                    .to_owned(),
        },
        TaintedContextNarrowingDecisionRow {
            decision_ref: "decision:tainted-context:blocked:01".to_owned(),
            requested_mode_class: TaintedContextRunModeClass::FullRun,
            effective_mode_class: TaintedContextRunModeClass::Blocked,
            policy_narrowing_class: TaintedContextPolicyNarrowingClass::BlockedByPolicy,
            source_refs: vec!["source-ref:user-paste:suspicious:01".to_owned()],
            narrowed_authority_tokens: Vec::new(),
            denied_capability_tokens: vec![
                "capability_widening".to_owned(),
                "approval_authority_grant".to_owned(),
            ],
            approval_fence_ref: "approval-fence:tainted-context:blocked:01".to_owned(),
            approval_requirement_class:
                TaintedContextApprovalRequirementClass::MutationBlockedNoApprovalPath,
            user_visible_reason_label:
                "Suspicious pasted text attempted to widen authority, so the run is blocked."
                    .to_owned(),
        },
    ]
}

fn approval_fences() -> Vec<TaintedContextApprovalFenceRow> {
    vec![
        TaintedContextApprovalFenceRow {
            approval_fence_ref: "approval-fence:tainted-context:explain-only:01".to_owned(),
            decision_ref: "decision:tainted-context:explain-only:01".to_owned(),
            source_refs: vec!["source-ref:external-docs:upgrade-note:01".to_owned()],
            approval_requirement_class:
                TaintedContextApprovalRequirementClass::NoApprovalRequiredExplainOnly,
            approval_ticket_ref: None,
            audit_event_refs: vec![
                "audit:tainted-context:external-docs:classified:01".to_owned(),
                "audit:tainted-context:external-docs:narrowed:01".to_owned(),
            ],
            prompt_injection_evaluation_ref: Some(
                "prompt-injection-eval:external-docs:01".to_owned(),
            ),
            blocks_hidden_provider_write: true,
            auditable: true,
            user_visible_explanation_label:
                "No approval is needed because all privileged effects were removed.".to_owned(),
        },
        TaintedContextApprovalFenceRow {
            approval_fence_ref: "approval-fence:tainted-context:local-only:01".to_owned(),
            decision_ref: "decision:tainted-context:local-only:01".to_owned(),
            source_refs: vec!["source-ref:mcp-response:scan:01".to_owned()],
            approval_requirement_class:
                TaintedContextApprovalRequirementClass::FreshApprovalRequiredAfterTaintedInput,
            approval_ticket_ref: None,
            audit_event_refs: vec![
                "audit:tainted-context:mcp-response:classified:01".to_owned(),
                "audit:tainted-context:mcp-response:approval-renewal-required:01".to_owned(),
            ],
            prompt_injection_evaluation_ref: Some(
                "prompt-injection-eval:mcp-response:01".to_owned(),
            ),
            blocks_hidden_provider_write: true,
            auditable: true,
            user_visible_explanation_label:
                "Remote execution requires a fresh approval after tainted tool output.".to_owned(),
        },
        TaintedContextApprovalFenceRow {
            approval_fence_ref: "approval-fence:tainted-context:preview-only:01".to_owned(),
            decision_ref: "decision:tainted-context:preview-only:01".to_owned(),
            source_refs: vec!["source-ref:terminal-output:test-run:01".to_owned()],
            approval_requirement_class:
                TaintedContextApprovalRequirementClass::PreviewRequiresApprovalBeforeApply,
            approval_ticket_ref: None,
            audit_event_refs: vec![
                "audit:tainted-context:terminal-output:classified:01".to_owned(),
                "audit:tainted-context:terminal-output:preview-only:01".to_owned(),
            ],
            prompt_injection_evaluation_ref: Some(
                "prompt-injection-eval:terminal-output:01".to_owned(),
            ),
            blocks_hidden_provider_write: true,
            auditable: true,
            user_visible_explanation_label:
                "A reviewed approval ticket is required before any preview can apply.".to_owned(),
        },
        TaintedContextApprovalFenceRow {
            approval_fence_ref: "approval-fence:tainted-context:blocked:01".to_owned(),
            decision_ref: "decision:tainted-context:blocked:01".to_owned(),
            source_refs: vec!["source-ref:user-paste:suspicious:01".to_owned()],
            approval_requirement_class:
                TaintedContextApprovalRequirementClass::MutationBlockedNoApprovalPath,
            approval_ticket_ref: None,
            audit_event_refs: vec![
                "audit:tainted-context:user-paste:suspicious-detected:01".to_owned(),
                "audit:tainted-context:user-paste:mutation-blocked:01".to_owned(),
            ],
            prompt_injection_evaluation_ref: Some("prompt-injection-eval:user-paste:01".to_owned()),
            blocks_hidden_provider_write: true,
            auditable: true,
            user_visible_explanation_label:
                "No approval path is admitted for tainted authority widening.".to_owned(),
        },
    ]
}

fn surface_rows(packet_id: &str) -> Vec<TaintedContextSurfaceRow> {
    [
        TaintedContextSurfaceClass::Composer,
        TaintedContextSurfaceClass::ContextInspector,
        TaintedContextSurfaceClass::ReviewWorkspace,
        TaintedContextSurfaceClass::DocsHelp,
        TaintedContextSurfaceClass::SupportExport,
        TaintedContextSurfaceClass::Cli,
    ]
    .into_iter()
    .map(|surface_class| TaintedContextSurfaceRow {
        surface_class,
        projection_ref: format!(
            "projection:{}:tainted-context:beta:0001",
            surface_class.as_str()
        ),
        packet_ref: packet_id.to_owned(),
        source_refs_visible: true,
        narrowing_decision_refs_visible: true,
        approval_fence_refs_visible: true,
        raw_private_material_excluded: true,
        preserves_operator_truth: true,
        supports_json_export: true,
        supports_markdown_summary: true,
    })
    .collect()
}

fn tainted_context_packet() -> TaintedContextBetaPacket {
    let packet_id = "tainted-context:beta:0001".to_owned();
    TaintedContextBetaPacket::new(TaintedContextBetaInput {
        packet_id: packet_id.clone(),
        workflow_or_surface_id: "surface:review-chat-tainted-context".to_owned(),
        display_label: "Review chat tainted context beta".to_owned(),
        context_snapshot_ref: "context-snapshot:tainted-context:beta:0001".to_owned(),
        evidence_packet_ref: "evidence-packet:tainted-context:beta:0001".to_owned(),
        retrieval_packet_ref: "retrieval-packet:tainted-context:beta:0001".to_owned(),
        source_rows: source_rows(),
        narrowing_decisions: narrowing_decisions(),
        approval_fences: approval_fences(),
        surface_rows: surface_rows(&packet_id),
        source_contract_refs: source_contract_refs(),
        json_export_ref: TAINTED_CONTEXT_BETA_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: "artifacts/ai/m3/tainted_context_beta_summary.md".to_owned(),
        policy_context: TaintedContextPolicyContext {
            policy_epoch_ref: "policy-epoch:tainted-context:beta:2026-05-17".to_owned(),
            trust_state: "restricted".to_owned(),
            deployment_profile_class: "managed_cloud".to_owned(),
            execution_context_ref: "execution-context:tainted-context:beta:0001".to_owned(),
        },
        promotion_gate_class: TaintedContextPromotionGateClass::Promotable,
        minted_at: "2026-05-17T13:40:00Z".to_owned(),
    })
}

#[test]
fn generated_beta_packet_validates_mode_narrowing_and_approval_fences() {
    let packet = tainted_context_packet();

    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.observed_effective_mode_tokens,
        vec![
            "explain_only".to_owned(),
            "local_only".to_owned(),
            "preview_only".to_owned(),
            "blocked".to_owned(),
        ]
    );
    assert!(packet
        .source_rows
        .iter()
        .any(|row| row.has_suspicious_content()));
    assert!(packet
        .approval_fences
        .iter()
        .all(|fence| fence.auditable && fence.blocks_hidden_provider_write));
}

#[test]
fn checked_in_support_export_matches_generated_projection() {
    let generated: serde_json::Value =
        serde_json::from_str(&tainted_context_packet().export_safe_json())
            .expect("generated parses");
    let checked_in: serde_json::Value = serde_json::from_str(include_str!(
        "../../../../artifacts/ai/m3/tainted_context_beta_support_export.json"
    ))
    .expect("checked-in support export parses");

    assert_eq!(generated, checked_in);
}

#[test]
fn current_checked_in_support_export_validates_and_renders_markdown() {
    let packet = current_beta_tainted_context_support_export()
        .expect("current support export parses and validates");
    let markdown = packet.render_markdown_summary();

    assert!(markdown.contains("tainted-context:beta:0001"));
    assert!(markdown.contains("Approval fences"));
    assert!(!packet.export_safe_json().contains("://"));
}

#[test]
fn missing_mode_coverage_blocks_beta_claim() {
    let mut packet = tainted_context_packet();
    packet
        .observed_effective_mode_tokens
        .retain(|token| token != "blocked");

    assert!(packet
        .validate()
        .contains(&TaintedContextBetaViolation::MissingEffectiveModeCoverage));
}

#[test]
fn suspicious_or_partial_sources_must_remain_explainable() {
    let mut suspicious_drift = tainted_context_packet();
    let row = suspicious_drift
        .source_rows
        .iter_mut()
        .find(|row| row.source_ref == "source-ref:user-paste:suspicious:01")
        .expect("suspicious row exists");
    row.suspicious_content_tokens.clear();

    assert!(suspicious_drift
        .validate()
        .contains(&TaintedContextBetaViolation::SuspiciousSourceMissingDetectorTruth));

    let mut retrieval_drift = tainted_context_packet();
    retrieval_drift.source_rows[0].retrieval_truth_label = None;
    assert!(retrieval_drift
        .validate()
        .contains(&TaintedContextBetaViolation::RetrievalTruthUnlabelled));
}

#[test]
fn approval_fence_or_surface_drift_blocks_beta_claim() {
    let mut fence_drift = tainted_context_packet();
    fence_drift.approval_fences[0].audit_event_refs.clear();
    assert!(fence_drift
        .validate()
        .contains(&TaintedContextBetaViolation::ApprovalFenceNotAuditable));

    let mut surface_drift = tainted_context_packet();
    surface_drift.surface_rows[0].packet_ref = "tainted-context:drifted".to_owned();
    assert!(surface_drift
        .validate()
        .contains(&TaintedContextBetaViolation::SurfaceProjectionDrift));
}

#[test]
fn schema_doc_fixture_and_artifact_paths_exist() {
    for rel in [
        TAINTED_CONTEXT_BETA_SCHEMA_REF,
        TAINTED_CONTEXT_BETA_AI_DOC_REF,
        TAINTED_CONTEXT_BETA_FIXTURE_DIR,
        TAINTED_CONTEXT_BETA_ARTIFACT_REF,
        "artifacts/ai/m3/tainted_context_beta_summary.md",
    ] {
        assert!(repo_root().join(rel).exists(), "{rel} should exist");
    }
}
