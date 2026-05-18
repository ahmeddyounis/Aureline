use aureline_commands::registry::seeded_registry;
use aureline_commands::CommandEnablementContext;

use crate::composer::beta::ComposerContextEvidenceBetaPacket;
use crate::composer::{AttachmentKind, AttachmentStatusClass, MentionKind, SelectionReasonClass};
use crate::context_inspector::{
    BudgetPressureClass, ComposerAttachmentPill, ComposerBudgetStrip, ComposerContextAlphaSnapshot,
    ComposerContextItem, ComposerContextReviewLock, ComposerContextReviewState,
    ComposerMentionPreview, ContextFreshnessClass, ContextGroupClass, ContextItemStateClass,
    ContextLocalityClass, ContextOmissionReasonClass, ContextTrustClass, ExecutionBoundaryClass,
    IntentModeClass, MentionPreviewStateClass, ReviewLockClass,
};
use crate::{SourceClass, TrustPosture, COMPOSER_CONTEXT_ALPHA_RECORD_KIND};

use super::*;

fn command_context() -> CommandEnablementContext {
    CommandEnablementContext {
        client_scope: "desktop".to_owned(),
        workspace_trust_state: "trusted".to_owned(),
        execution_context_available: false,
        provider_linked: Some(true),
        credential_available: Some(true),
        policy_disabled: false,
        policy_blocked_in_context: false,
        labs_enabled: false,
        argument_provenance_map: Vec::new(),
    }
}

fn snapshot() -> ComposerContextAlphaSnapshot {
    ComposerContextAlphaSnapshot {
        record_kind: COMPOSER_CONTEXT_ALPHA_RECORD_KIND.to_owned(),
        schema_version: crate::COMPOSER_CONTEXT_ALPHA_SCHEMA_VERSION,
        composer_draft_id: "turn-draft:prompt-composer:beta:0001".to_owned(),
        composer_session_id: "composer-session:prompt-composer:beta:0001".to_owned(),
        request_workspace_id: "request-workspace:prompt-composer:beta:0001".to_owned(),
        intent_text: "Review the retry change and keep the stale run visible.".to_owned(),
        intent_mode: IntentModeClass::ReviewDiff,
        scope_label: "Current diff".to_owned(),
        execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
        action_identity_ref: Some("cmd:task.rerun_last".to_owned()),
        mention_previews: vec![
            ComposerMentionPreview {
                mention_id: "mention:file:retry".to_owned(),
                kind: MentionKind::FileMention,
                preview_state: MentionPreviewStateClass::ResolvedExact,
                target_stable_id: Some("file:payments-retry".to_owned()),
                candidate_target_refs: Vec::new(),
                display_label: "@file retry".to_owned(),
                docs_identity: None,
            },
            ComposerMentionPreview {
                mention_id: "mention:symbol:route".to_owned(),
                kind: MentionKind::SymbolMention,
                preview_state: MentionPreviewStateClass::Ambiguous,
                target_stable_id: None,
                candidate_target_refs: vec![
                    "symbol:RetryRoute".to_owned(),
                    "symbol:RetryRouter".to_owned(),
                ],
                display_label: "@symbol RetryRoute".to_owned(),
                docs_identity: None,
            },
            ComposerMentionPreview {
                mention_id: "mention:root:workspace".to_owned(),
                kind: MentionKind::RootMention,
                preview_state: MentionPreviewStateClass::Blocked,
                target_stable_id: None,
                candidate_target_refs: Vec::new(),
                display_label: "@root payments".to_owned(),
                docs_identity: None,
            },
            ComposerMentionPreview {
                mention_id: "mention:run:last-test".to_owned(),
                kind: MentionKind::RunMention,
                preview_state: MentionPreviewStateClass::Stale,
                target_stable_id: None,
                candidate_target_refs: Vec::new(),
                display_label: "@run last test".to_owned(),
                docs_identity: None,
            },
        ],
        attachment_pills: vec![
            attachment_pill(
                "att:file:retry",
                AttachmentKind::WorkspaceSliceBundle,
                SourceClass::WorkspaceFileSlice,
                TrustPosture::TrustedFirstParty,
                AttachmentStatusClass::Live,
                ContextItemStateClass::Included,
                "retry.rs selected lines",
                2_048,
            ),
            attachment_pill(
                "att:run:last-test",
                AttachmentKind::TerminalLogCapture,
                SourceClass::TerminalTranscriptExcerpt,
                TrustPosture::UntrustedExternal,
                AttachmentStatusClass::Stale,
                ContextItemStateClass::Stale,
                "last test output",
                1_024,
            ),
        ],
        context_items: vec![
            context_item(
                "ctx:file:retry",
                ContextGroupClass::OpenFiles,
                ContextItemStateClass::Included,
                SourceClass::WorkspaceFileSlice,
                "file:payments-retry",
                None,
                Some("att:file:retry"),
                Some("mention:file:retry"),
                2_048,
            ),
            context_item(
                "ctx:run:last-test",
                ContextGroupClass::RuntimeArtifacts,
                ContextItemStateClass::Stale,
                SourceClass::TerminalTranscriptExcerpt,
                "run:last-test",
                Some(ContextOmissionReasonClass::Stale),
                Some("att:run:last-test"),
                Some("mention:run:last-test"),
                1_024,
            ),
            context_item(
                "ctx:history:large",
                ContextGroupClass::DiffsHistory,
                ContextItemStateClass::Omitted,
                SourceClass::WorkspaceSearchResult,
                "history:large-diff",
                Some(ContextOmissionReasonClass::Budget),
                None,
                None,
                6_144,
            ),
            context_item(
                "ctx:docs:summary",
                ContextGroupClass::DocsKnowledgeSources,
                ContextItemStateClass::Summarized,
                SourceClass::DocsPackExcerpt,
                "docs:retry-policy",
                Some(ContextOmissionReasonClass::Budget),
                None,
                None,
                4_096,
            ),
            context_item(
                "ctx:tool:trimmed",
                ContextGroupClass::ExternalToolResults,
                ContextItemStateClass::Trimmed,
                SourceClass::TerminalTranscriptExcerpt,
                "tool-result:retry-lint",
                Some(ContextOmissionReasonClass::Budget),
                None,
                None,
                2_048,
            ),
        ],
        graph_cue_packets: Vec::new(),
        budget_strip: ComposerBudgetStrip {
            aggregate_byte_estimate: 15_360,
            budget_byte_ceiling: 8_192,
            pressure_class: BudgetPressureClass::Overflow,
            included_context_group_tokens: vec!["open_files".to_owned()],
            omitted_or_trimmed_group_tokens: vec![
                "runtime_artifacts".to_owned(),
                "diffs_history".to_owned(),
                "docs_knowledge_sources".to_owned(),
                "external_tool_results".to_owned(),
            ],
            selected_provider_label: "Aureline managed hosted AI".to_owned(),
            selected_model_label: "Hosted context review preview".to_owned(),
            quota_state_token: "within_limit".to_owned(),
            cost_envelope_token: "vendor_hosted_entitlement_band".to_owned(),
        },
        review_lock: ComposerContextReviewLock {
            lock_class: ReviewLockClass::FrozenForReview,
            context_snapshot_ref: "context-snapshot:prompt-composer:beta:0001".to_owned(),
            route_snapshot_ref: "routing-packet:prompt-composer:beta:0001".to_owned(),
            review_started_at: Some("2026-05-18T20:00:00Z".to_owned()),
        },
        review_state: ComposerContextReviewState::BudgetReviewRequired,
    }
}

fn attachment_pill(
    id: &str,
    kind: AttachmentKind,
    source_class: SourceClass,
    trust_posture: TrustPosture,
    status: AttachmentStatusClass,
    context_state: ContextItemStateClass,
    label: &str,
    estimated_byte_size: u64,
) -> ComposerAttachmentPill {
    ComposerAttachmentPill {
        attachment_id: id.to_owned(),
        kind,
        source_class,
        trust_posture,
        selection_reason: SelectionReasonClass::UserPinned,
        status,
        context_state,
        display_label: label.to_owned(),
        estimated_byte_size,
        removable: true,
        docs_identity: None,
    }
}

fn context_item(
    id: &str,
    group_class: ContextGroupClass,
    state_class: ContextItemStateClass,
    source_class: SourceClass,
    stable_identity_ref: &str,
    omission_reason_class: Option<ContextOmissionReasonClass>,
    source_attachment_ref: Option<&str>,
    source_mention_ref: Option<&str>,
    estimated_byte_size: u64,
) -> ComposerContextItem {
    ComposerContextItem {
        context_item_id: id.to_owned(),
        group_class,
        state_class,
        source_class,
        stable_identity_ref: stable_identity_ref.to_owned(),
        display_label: id.to_owned(),
        freshness_class: if state_class == ContextItemStateClass::Stale {
            ContextFreshnessClass::Stale
        } else {
            ContextFreshnessClass::AuthoritativeLive
        },
        trust_class: if source_class == SourceClass::TerminalTranscriptExcerpt {
            ContextTrustClass::UntrustedExternal
        } else {
            ContextTrustClass::TrustedFirstParty
        },
        locality_class: if source_class == SourceClass::TerminalTranscriptExcerpt {
            ContextLocalityClass::RemoteRuntime
        } else {
            ContextLocalityClass::LocalWorkspace
        },
        estimated_byte_size,
        omission_reason_class,
        source_attachment_ref: source_attachment_ref.map(str::to_owned),
        source_mention_ref: source_mention_ref.map(str::to_owned),
        docs_identity: None,
    }
}

fn beta_evidence_stub() -> ComposerContextEvidenceBetaPacket {
    ComposerContextEvidenceBetaPacket {
        record_kind: crate::COMPOSER_CONTEXT_EVIDENCE_BETA_PACKET_RECORD_KIND.to_owned(),
        schema_version: crate::COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_VERSION,
        packet_id: "composer-context-evidence:prompt-composer:beta:0001".to_owned(),
        workflow_or_surface_id: "surface:prompt-composer:beta".to_owned(),
        display_label: "Prompt composer conformance beta".to_owned(),
        composer_context_snapshot_ref: "context-snapshot:prompt-composer:beta:0001".to_owned(),
        composer_session_ref: "composer-session:prompt-composer:beta:0001".to_owned(),
        turn_draft_ref: "turn-draft:prompt-composer:beta:0001".to_owned(),
        request_workspace_ref: "request-workspace:prompt-composer:beta:0001".to_owned(),
        context_review_state_token: "budget_review_required".to_owned(),
        required_context_state_tokens: vec!["included".to_owned()],
        observed_context_state_tokens: vec!["included".to_owned()],
        context_rows: Vec::new(),
        retrieval_packet_ref: "retrieval-packet:prompt-composer:beta:0001".to_owned(),
        retrieval_promotion_state_token: "promotable".to_owned(),
        retrieval_validation_finding_tokens: Vec::new(),
        evidence_packet_ref: "evidence-packet:prompt-composer:beta:0001".to_owned(),
        evidence_packet_state_token: "applied".to_owned(),
        routing_packet_ref: "routing-packet:prompt-composer:beta:0001".to_owned(),
        route_receipt_ref: "route-receipt:prompt-composer:beta:0001".to_owned(),
        spend_receipt_ref: "spend-receipt:prompt-composer:beta:0001".to_owned(),
        selected_provider_entry_ref: "provider-entry:managed:prompt-composer".to_owned(),
        selected_model_entry_ref: "model-entry:managed:prompt-composer".to_owned(),
        selected_provider_label: "Aureline managed hosted AI".to_owned(),
        selected_model_label: "Hosted context review preview".to_owned(),
        route_origin_token: "vendor_hosted_managed".to_owned(),
        cost_envelope_token: "vendor_hosted_entitlement_band".to_owned(),
        cost_visibility_token: "bundled_no_incremental_cost".to_owned(),
        tool_call_lineage_refs: vec!["tool-call-lineage:prompt-composer:beta:0001".to_owned()],
        approval_ticket_refs: vec!["approval-ticket:prompt-composer:beta:0001".to_owned()],
        apply_outcome_token: "applied_success".to_owned(),
        spend_receipt_run_state_token: "post_run_completed".to_owned(),
        spend_receipt_cost_envelope_token: "vendor_hosted_entitlement_band".to_owned(),
        spend_receipt_cost_visibility_token: "bundled_no_incremental_cost".to_owned(),
        spend_receipt_charge_locus_token: "not_charged_bundled".to_owned(),
        surface_rows: Vec::new(),
        source_contract_refs: Vec::new(),
        json_export_ref: PROMPT_COMPOSER_CONFORMANCE_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: PROMPT_COMPOSER_CONFORMANCE_SUMMARY_REF.to_owned(),
        minted_at: "2026-05-18T20:00:00Z".to_owned(),
    }
}

fn packet() -> PromptComposerConformancePacket {
    let snapshot = snapshot();
    let evidence = beta_evidence_stub();
    let slash_command = PromptSlashCommandBinding::from_registry_preflight(
        "slash:task-rerun-last",
        "cmd:task.rerun_last",
        "/run test",
        seeded_registry(),
        &command_context(),
    );
    PromptComposerConformancePacket::from_context_snapshot(
        &snapshot,
        &evidence,
        PromptComposerConformanceInput {
            packet_id: "prompt-composer-conformance:beta:0001".to_owned(),
            workflow_or_surface_id: "surface:prompt-composer:beta".to_owned(),
            display_label: "Prompt composer beta conformance".to_owned(),
            draft_persistence: PromptDraftPersistence {
                retention_scope_class: DraftRetentionScopeClass::ManagedPolicyReplicated,
                local_first: true,
                policy_epoch_ref: "policy-epoch:prompt-composer:beta".to_owned(),
                collaboration_session_ref: None,
                visibility_note: "Draft is local first; managed policy mirrors metadata only."
                    .to_owned(),
                clear_action_ref: "action:prompt-composer.clear-draft".to_owned(),
            },
            input_semantics: PromptInputSemantics {
                multiline_entry_preserved: true,
                ime_composition_blocks_send: true,
                send_requires_explicit_action: true,
                draft_survives_degraded_routes: true,
            },
            slash_command_rows: vec![slash_command],
            budget_decisions: Vec::new(),
            edge_case_rows: edge_case_rows(),
            evidence_lineage: evidence_lineage(),
            preview_branch_rows: vec![PreviewBranchComposerRow {
                preview_row_ref: "preview-row:branch-agent:prompt-composer:0001".to_owned(),
                branch_or_worktree_ref: "worktree:prompt-composer-review:0001".to_owned(),
                preview_only: true,
                autonomous_apply_enabled: false,
                cumulative_budget_posture_ref: "budget-posture:branch-agent:0001".to_owned(),
                route_receipt_refs: vec!["route-receipt:prompt-composer:beta:0001".to_owned()],
            }],
            source_contract_refs: source_contract_refs(),
            json_export_ref: PROMPT_COMPOSER_CONFORMANCE_ARTIFACT_REF.to_owned(),
            markdown_summary_ref: PROMPT_COMPOSER_CONFORMANCE_SUMMARY_REF.to_owned(),
            minted_at: "2026-05-18T20:00:00Z".to_owned(),
        },
    )
}

fn edge_case_rows() -> Vec<PromptComposerEdgeCaseRow> {
    vec![
        edge_case(
            PromptComposerEdgeCaseClass::StaleAttachment,
            "att:run:last-test",
            PromptComposerSafeFallbackClass::ManualEditAndSearch,
        ),
        edge_case(
            PromptComposerEdgeCaseClass::UnresolvedMention,
            "mention:symbol:route",
            PromptComposerSafeFallbackClass::ManualEditAndSearch,
        ),
        edge_case(
            PromptComposerEdgeCaseClass::OverBudgetComposition,
            "context-snapshot:prompt-composer:beta:0001",
            PromptComposerSafeFallbackClass::ManualEditAndSearch,
        ),
        edge_case(
            PromptComposerEdgeCaseClass::PolicyBlockedRoute,
            "route-receipt:prompt-composer:beta:0001",
            PromptComposerSafeFallbackClass::CliHeadlessPath,
        ),
        edge_case(
            PromptComposerEdgeCaseClass::OfflineLocalOnlyDegradation,
            "route:offline-local-only",
            PromptComposerSafeFallbackClass::LocalOnlyReview,
        ),
    ]
}

fn edge_case(
    edge_case_class: PromptComposerEdgeCaseClass,
    source_ref: &str,
    safe_fallback_class: PromptComposerSafeFallbackClass,
) -> PromptComposerEdgeCaseRow {
    PromptComposerEdgeCaseRow {
        edge_case_class,
        source_ref: source_ref.to_owned(),
        preserves_current_draft: true,
        safe_fallback_class,
        explanation_label: format!(
            "{} keeps the current draft intact.",
            edge_case_class.as_str()
        ),
    }
}

fn evidence_lineage() -> PromptEvidenceLineage {
    PromptEvidenceLineage {
        evidence_id: "evidence-packet:prompt-composer:beta:0001".to_owned(),
        composer_session_ref: "composer-session:prompt-composer:beta:0001".to_owned(),
        turn_draft_ref: "turn-draft:prompt-composer:beta:0001".to_owned(),
        composer_context_snapshot_ref: "context-snapshot:prompt-composer:beta:0001".to_owned(),
        packet_classes: vec![
            PromptEvidencePacketClass::InlineStub,
            PromptEvidencePacketClass::OperatorPacket,
            PromptEvidencePacketClass::SupportPacket,
            PromptEvidencePacketClass::ComplianceAuditPacket,
        ],
        route_receipt_ref: "route-receipt:prompt-composer:beta:0001".to_owned(),
        spend_receipt_ref: "spend-receipt:prompt-composer:beta:0001".to_owned(),
        redaction_manifest_ref: "redaction-manifest:prompt-composer:beta:0001".to_owned(),
        replay_lineage_ref: "replay-lineage:prompt-composer:beta:0001".to_owned(),
        operator_packet_ref: "operator-packet:prompt-composer:beta:0001".to_owned(),
        support_packet_ref: "support-packet:prompt-composer:beta:0001".to_owned(),
        compliance_packet_ref: "compliance-packet:prompt-composer:beta:0001".to_owned(),
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        PROMPT_COMPOSER_AI_DOC_REF.to_owned(),
        PROMPT_COMPOSER_BETA_UX_DOC_REF.to_owned(),
        PROMPT_COMPOSER_DRAFT_SCHEMA_REF.to_owned(),
        PROMPT_CONTEXT_ATTACHMENT_SCHEMA_REF.to_owned(),
        "docs/ai/context_assembly_contract.md".to_owned(),
        "docs/ai/spend_and_route_receipt_contract.md".to_owned(),
        "docs/ai/evidence_replayability_contract.md".to_owned(),
    ]
}

#[test]
fn conformance_packet_validates_required_composer_semantics() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert!(packet
        .mention_rows
        .iter()
        .any(|row| row.mention_kind == PromptMentionKind::File));
    assert!(packet
        .mention_rows
        .iter()
        .any(|row| row.mention_kind == PromptMentionKind::Symbol));
    assert!(packet
        .mention_rows
        .iter()
        .any(|row| row.mention_kind == PromptMentionKind::Root));
    assert!(packet
        .mention_rows
        .iter()
        .any(|row| row.mention_kind == PromptMentionKind::Run));
    assert!(packet.budget_strip.decision_rows.iter().any(|row| {
        matches!(
            row.action_class,
            PromptBudgetActionClass::Omit
                | PromptBudgetActionClass::Summarize
                | PromptBudgetActionClass::Trim
                | PromptBudgetActionClass::RouteSwitch
        )
    }));
}

#[test]
fn ambiguous_mentions_must_expose_candidates_before_send() {
    let mut packet = packet();
    let ambiguous = packet
        .mention_rows
        .iter_mut()
        .find(|row| row.resolution_class == PromptMentionResolutionClass::Ambiguous)
        .expect("ambiguous row");
    ambiguous.candidate_target_refs.truncate(1);

    assert!(packet
        .validate()
        .contains(&PromptComposerConformanceViolation::MentionResolutionUnsafe));
}

#[test]
fn attachments_require_individual_keyboard_reachable_actions() {
    let mut packet = packet();
    packet.attachment_rows[0].remove_action_ref.clear();

    assert!(packet
        .validate()
        .contains(&PromptComposerConformanceViolation::AttachmentActionMissing));
}

#[test]
fn disabled_slash_commands_keep_command_graph_reason_semantics() {
    let mut packet = packet();
    assert!(packet.slash_command_rows[0].disabled_reason_token.is_some());
    packet.slash_command_rows[0].disabled_reason_token = None;

    assert!(packet
        .validate()
        .contains(&PromptComposerConformanceViolation::DisabledReasonMissing));
}

#[test]
fn preview_branch_rows_cannot_widen_into_autonomous_apply() {
    let mut packet = packet();
    packet.preview_branch_rows[0].autonomous_apply_enabled = true;

    assert!(packet
        .validate()
        .contains(&PromptComposerConformanceViolation::PreviewBranchWidened));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_beta_prompt_composer_conformance_export()
        .expect("checked prompt-composer conformance export validates");
    assert!(packet.validate().is_empty());
}
