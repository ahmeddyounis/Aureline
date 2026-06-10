use super::*;

const PACKET_ID: &str = "m5-richer-prompt-composer:stable:0001";

fn intent_row() -> RicherIntentModeRow {
    RicherIntentModeRow {
        mode_class: IntentModeClass::DraftPatch,
        current_scope_label: "workspace/src".to_owned(),
        execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
        action_identity_ref: None,
        behavior_constraints: vec![
            IntentModeBehaviorConstraint::ReviewBeforeApply,
            IntentModeBehaviorConstraint::DraftOnlyNoInPlaceApply,
            IntentModeBehaviorConstraint::RequiresScopedApplyHardening,
            IntentModeBehaviorConstraint::RequiresEvidencePacket,
        ],
        required_tool_pack_refs: vec!["tool-pack:scoped-apply:v1".to_owned()],
        approval_posture_class: "review_required".to_owned(),
    }
}

fn attachment_rows() -> Vec<RicherAttachmentRow> {
    vec![
        RicherAttachmentRow {
            attachment_id: "att-001".to_owned(),
            stable_object_ref: "workspace:file:src/main.rs".to_owned(),
            origin_label: "src/main.rs".to_owned(),
            source_class: StableAttachmentSourceClass::WorkspaceFile,
            trust_posture: TrustPosture::TrustedFirstParty,
            freshness_class: ContextFreshnessClass::AuthoritativeLive,
            semantic_role: AttachmentSemanticRoleClass::PrimaryContext,
            provenance_class: AttachmentProvenanceClass::DirectWorkspace,
            mode_relevance: vec![
                IntentModeClass::Ask,
                IntentModeClass::Explain,
                IntentModeClass::DraftPatch,
                IntentModeClass::ReviewDiff,
            ],
            context_state: ContextItemStateClass::Included,
            display_label: "src/main.rs".to_owned(),
            estimated_byte_size: 4096,
            preview_action_ref: "action:preview:att-001".to_owned(),
            open_action_ref: "action:open:att-001".to_owned(),
            remove_action_ref: "action:remove:att-001".to_owned(),
            keyboard_reachable: true,
            screen_reader_label: "Attachment src main rs, workspace file, primary context".to_owned(),
        },
        RicherAttachmentRow {
            attachment_id: "att-002".to_owned(),
            stable_object_ref: "workspace:symbol:MyStruct".to_owned(),
            origin_label: "MyStruct".to_owned(),
            source_class: StableAttachmentSourceClass::Symbol,
            trust_posture: TrustPosture::TrustedFirstParty,
            freshness_class: ContextFreshnessClass::WarmCached,
            semantic_role: AttachmentSemanticRoleClass::ReferenceMaterial,
            provenance_class: AttachmentProvenanceClass::RetrievedQuery,
            mode_relevance: vec![
                IntentModeClass::Explain,
                IntentModeClass::DraftPatch,
                IntentModeClass::ReviewDiff,
            ],
            context_state: ContextItemStateClass::Included,
            display_label: "MyStruct".to_owned(),
            estimated_byte_size: 1024,
            preview_action_ref: "action:preview:att-002".to_owned(),
            open_action_ref: "action:open:att-002".to_owned(),
            remove_action_ref: "action:remove:att-002".to_owned(),
            keyboard_reachable: true,
            screen_reader_label: "Attachment MyStruct, symbol, reference material".to_owned(),
        },
        RicherAttachmentRow {
            attachment_id: "att-003".to_owned(),
            stable_object_ref: "docs:pack:api-guide".to_owned(),
            origin_label: "API Guide".to_owned(),
            source_class: StableAttachmentSourceClass::DocsReference,
            trust_posture: TrustPosture::TrustedFirstParty,
            freshness_class: ContextFreshnessClass::WarmCached,
            semantic_role: AttachmentSemanticRoleClass::InstructionSource,
            provenance_class: AttachmentProvenanceClass::DocsKnowledgePack,
            mode_relevance: vec![
                IntentModeClass::Ask,
                IntentModeClass::Explain,
                IntentModeClass::Plan,
            ],
            context_state: ContextItemStateClass::Included,
            display_label: "API Guide".to_owned(),
            estimated_byte_size: 2048,
            preview_action_ref: "action:preview:att-003".to_owned(),
            open_action_ref: "action:open:att-003".to_owned(),
            remove_action_ref: "action:remove:att-003".to_owned(),
            keyboard_reachable: true,
            screen_reader_label: "Attachment API Guide, docs reference, instruction source".to_owned(),
        },
        RicherAttachmentRow {
            attachment_id: "att-004".to_owned(),
            stable_object_ref: "workspace:diagnostic:unused-import".to_owned(),
            origin_label: "Unused import warning".to_owned(),
            source_class: StableAttachmentSourceClass::Diagnostic,
            trust_posture: TrustPosture::TrustedFirstParty,
            freshness_class: ContextFreshnessClass::AuthoritativeLive,
            semantic_role: AttachmentSemanticRoleClass::DiagnosticOutput,
            provenance_class: AttachmentProvenanceClass::DirectWorkspace,
            mode_relevance: vec![IntentModeClass::ReviewDiff, IntentModeClass::DraftPatch],
            context_state: ContextItemStateClass::Included,
            display_label: "Unused import warning".to_owned(),
            estimated_byte_size: 512,
            preview_action_ref: "action:preview:att-004".to_owned(),
            open_action_ref: "action:open:att-004".to_owned(),
            remove_action_ref: "action:remove:att-004".to_owned(),
            keyboard_reachable: true,
            screen_reader_label: "Attachment unused import warning, diagnostic, diagnostic output"
                .to_owned(),
        },
        RicherAttachmentRow {
            attachment_id: "att-005".to_owned(),
            stable_object_ref: "workspace:test:result:pass".to_owned(),
            origin_label: "Test result".to_owned(),
            source_class: StableAttachmentSourceClass::TestResult,
            trust_posture: TrustPosture::TrustedFirstParty,
            freshness_class: ContextFreshnessClass::AuthoritativeLive,
            semantic_role: AttachmentSemanticRoleClass::TestFixture,
            provenance_class: AttachmentProvenanceClass::RuntimeCapture,
            mode_relevance: vec![IntentModeClass::GenerateTests, IntentModeClass::ReviewDiff],
            context_state: ContextItemStateClass::Included,
            display_label: "Test result".to_owned(),
            estimated_byte_size: 1024,
            preview_action_ref: "action:preview:att-005".to_owned(),
            open_action_ref: "action:open:att-005".to_owned(),
            remove_action_ref: "action:remove:att-005".to_owned(),
            keyboard_reachable: true,
            screen_reader_label: "Attachment test result, test result, test fixture".to_owned(),
        },
        RicherAttachmentRow {
            attachment_id: "att-006".to_owned(),
            stable_object_ref: "terminal:capture:build-log".to_owned(),
            origin_label: "Build log".to_owned(),
            source_class: StableAttachmentSourceClass::TerminalToolOutput,
            trust_posture: TrustPosture::TrustedFirstParty,
            freshness_class: ContextFreshnessClass::AuthoritativeLive,
            semantic_role: AttachmentSemanticRoleClass::DiagnosticOutput,
            provenance_class: AttachmentProvenanceClass::RuntimeCapture,
            mode_relevance: vec![
                IntentModeClass::Ask,
                IntentModeClass::Explain,
                IntentModeClass::DraftPatch,
            ],
            context_state: ContextItemStateClass::Included,
            display_label: "Build log".to_owned(),
            estimated_byte_size: 3072,
            preview_action_ref: "action:preview:att-006".to_owned(),
            open_action_ref: "action:open:att-006".to_owned(),
            remove_action_ref: "action:remove:att-006".to_owned(),
            keyboard_reachable: true,
            screen_reader_label: "Attachment build log, terminal tool output, diagnostic output"
                .to_owned(),
        },
        RicherAttachmentRow {
            attachment_id: "att-007".to_owned(),
            stable_object_ref: "external:text:pasted-snippet".to_owned(),
            origin_label: "Pasted snippet".to_owned(),
            source_class: StableAttachmentSourceClass::ExternalText,
            trust_posture: TrustPosture::UntrustedExternal,
            freshness_class: ContextFreshnessClass::Unverified,
            semantic_role: AttachmentSemanticRoleClass::ExternalReference,
            provenance_class: AttachmentProvenanceClass::ExternalImport,
            mode_relevance: vec![IntentModeClass::Ask],
            context_state: ContextItemStateClass::Tainted,
            display_label: "Pasted snippet".to_owned(),
            estimated_byte_size: 256,
            preview_action_ref: "action:preview:att-007".to_owned(),
            open_action_ref: "action:open:att-007".to_owned(),
            remove_action_ref: "action:remove:att-007".to_owned(),
            keyboard_reachable: true,
            screen_reader_label: "Attachment pasted snippet, external text, external reference, tainted"
                .to_owned(),
        },
    ]
}

fn pinned_rows() -> Vec<RicherPinnedContextRow> {
    vec![
        RicherPinnedContextRow {
            pin_id: "pin-001".to_owned(),
            stable_object_ref: "workspace:file:src/lib.rs".to_owned(),
            display_label: "src/lib.rs".to_owned(),
            freshness_state: PinnedFreshnessStateClass::PinnedFresh,
            drift_source: None,
            pin_policy: PinPolicyClass::AutoRefreshOnChange,
            auto_refresh: PinAutoRefreshClass::Immediate,
            stale_after_duration_seconds: None,
            refresh_action_ref: "action:refresh:pin-001".to_owned(),
            remove_action_ref: "action:remove:pin-001".to_owned(),
            blocks_send_until_resolved: false,
            keyboard_reachable: true,
        },
        RicherPinnedContextRow {
            pin_id: "pin-002".to_owned(),
            stable_object_ref: "workspace:file:tests/integration.rs".to_owned(),
            display_label: "tests/integration.rs".to_owned(),
            freshness_state: PinnedFreshnessStateClass::PinnedButStale,
            drift_source: Some(crate::stabilize_prompt_composer::DriftSourceClass::OnDisk),
            pin_policy: PinPolicyClass::StaleAfterDuration,
            auto_refresh: PinAutoRefreshClass::None,
            stale_after_duration_seconds: Some(3600),
            refresh_action_ref: "action:refresh:pin-002".to_owned(),
            remove_action_ref: "action:remove:pin-002".to_owned(),
            blocks_send_until_resolved: true,
            keyboard_reachable: true,
        },
    ]
}

fn omitted_rows() -> Vec<RicherOmittedContextRow> {
    vec![
        RicherOmittedContextRow {
            source_ref: "workspace:file:vendor/lib.c".to_owned(),
            source_class: SourceClass::WorkspaceFileSlice,
            omission_reason_class: ContextOmissionReasonClass::Budget,
            restoration_class: OmittedContextRestorationClass::OneClickRestore,
            exclusion_freshness: ExclusionFreshnessClass::ReasonStillValid,
            inspectable_after_send: true,
            inspect_action_ref: "action:inspect:omitted-001".to_owned(),
            restoration_action_ref: "action:restore:omitted-001".to_owned(),
            replay_explains_exclusion: true,
            keyboard_reachable: true,
        },
        RicherOmittedContextRow {
            source_ref: "workspace:file:secrets.env".to_owned(),
            source_class: SourceClass::WorkspaceFileSlice,
            omission_reason_class: ContextOmissionReasonClass::Policy,
            restoration_class: OmittedContextRestorationClass::PermanentlyExcluded,
            exclusion_freshness: ExclusionFreshnessClass::ReasonStillValid,
            inspectable_after_send: true,
            inspect_action_ref: "action:inspect:omitted-002".to_owned(),
            restoration_action_ref: "action:restore:omitted-002".to_owned(),
            replay_explains_exclusion: true,
            keyboard_reachable: true,
        },
    ]
}

fn budget_strip() -> RicherBudgetStrip {
    RicherBudgetStrip {
        aggregate_byte_estimate: 15360,
        budget_byte_ceiling: 16384,
        pressure_class: BudgetPressureClass::Warning,
        included_context_group_tokens: vec![
            "open_files".to_owned(),
            "symbols_graph_entities".to_owned(),
        ],
        omitted_or_trimmed_group_tokens: vec!["runtime_artifacts".to_owned()],
        decision_rows: vec![
            RicherBudgetDecisionRow {
                decision_id: "budget-decision:001".to_owned(),
                source_ref: "workspace:file:vendor/lib.c".to_owned(),
                source_class: SourceClass::WorkspaceFileSlice,
                context_state: ContextItemStateClass::Omitted,
                action_class: PromptBudgetActionClass::Omit,
                reason_token: Some("budget".to_owned()),
                estimated_byte_size: 8192,
                route_receipt_ref: None,
                driven_by_pin_policy: false,
            },
            RicherBudgetDecisionRow {
                decision_id: "budget-decision:002".to_owned(),
                source_ref: "workspace:file:tests/integration.rs".to_owned(),
                source_class: SourceClass::WorkspaceFileSlice,
                context_state: ContextItemStateClass::Pinned,
                action_class: PromptBudgetActionClass::PinKeep,
                reason_token: Some("pin_policy".to_owned()),
                estimated_byte_size: 2048,
                route_receipt_ref: None,
                driven_by_pin_policy: true,
            },
        ],
        safe_fallback_class: PromptComposerSafeFallbackClass::ManualEditAndSearch,
        explanation_label: "The turn is near budget; pinned context remains visible before send."
            .to_owned(),
    }
}

fn thread_header() -> RicherThreadHeader {
    RicherThreadHeader {
        thread_id: "thread-001".to_owned(),
        current_scope_label: "workspace/src".to_owned(),
        provider_label: "Aureline Managed".to_owned(),
        model_label: "claude-sonnet-4".to_owned(),
        execution_boundary_class: ExecutionBoundaryClass::ManagedHosted,
        retention_mode_class: crate::stabilize_prompt_composer::ThreadRetentionModeClass::LocalOnly,
        memory_class_token: "composer-thread".to_owned(),
        save_memory_action_ref: "action:memory:save".to_owned(),
        delete_action_ref: "action:memory:delete".to_owned(),
        export_action_ref: "action:memory:export".to_owned(),
        remember_preview: crate::stabilize_prompt_composer::RememberPreview {
            retained_summary_label: "Current composer thread with draft patch intent".to_owned(),
            retention_locus_class: crate::stabilize_prompt_composer::RetentionLocusClass::LocalDevice,
            reuse_audience_class: crate::stabilize_prompt_composer::ReuseAudienceClass::OnlyMe,
            memory_class_token: "composer-thread".to_owned(),
            preview_action_ref: "action:memory:preview".to_owned(),
        },
    }
}

fn surface_consistency_rows() -> Vec<RicherSurfaceConsistencyRow> {
    vec![
        RicherSurfaceConsistencyRow {
            surface_class: ComposerSurfaceClass::EditorAttached,
            attachment_pills_keyboard_reachable: true,
            mention_rows_screen_reader_describable: true,
            omitted_context_review_reachable: true,
            pinned_context_review_reachable: true,
            context_drift_banner_reachable: true,
            intent_mode_constraints_visible: true,
        },
        RicherSurfaceConsistencyRow {
            surface_class: ComposerSurfaceClass::Sidebar,
            attachment_pills_keyboard_reachable: true,
            mention_rows_screen_reader_describable: true,
            omitted_context_review_reachable: true,
            pinned_context_review_reachable: true,
            context_drift_banner_reachable: true,
            intent_mode_constraints_visible: true,
        },
        RicherSurfaceConsistencyRow {
            surface_class: ComposerSurfaceClass::Detached,
            attachment_pills_keyboard_reachable: true,
            mention_rows_screen_reader_describable: true,
            omitted_context_review_reachable: true,
            pinned_context_review_reachable: true,
            context_drift_banner_reachable: true,
            intent_mode_constraints_visible: true,
        },
    ]
}

fn evidence_lineage() -> PromptEvidenceLineage {
    PromptEvidenceLineage {
        evidence_id: "evidence-001".to_owned(),
        composer_session_ref: "session-001".to_owned(),
        turn_draft_ref: "draft-001".to_owned(),
        composer_context_snapshot_ref: "snapshot-001".to_owned(),
        packet_classes: vec![
            PromptEvidencePacketClass::InlineStub,
            PromptEvidencePacketClass::OperatorPacket,
            PromptEvidencePacketClass::SupportPacket,
            PromptEvidencePacketClass::ComplianceAuditPacket,
        ],
        route_receipt_ref: "route-001".to_owned(),
        spend_receipt_ref: "spend-001".to_owned(),
        redaction_manifest_ref: "redaction-001".to_owned(),
        replay_lineage_ref: "replay-001".to_owned(),
        operator_packet_ref: "operator-001".to_owned(),
        support_packet_ref: "support-001".to_owned(),
        compliance_packet_ref: "compliance-001".to_owned(),
    }
}

fn forked_thread_lineage() -> ForkedThreadLineage {
    ForkedThreadLineage {
        thread_id: "thread-001".to_owned(),
        is_forked: false,
        parent_thread_ref: None,
        parent_run_ref: None,
        inherited_context_snapshot_ref: None,
        divergence_point_ref: None,
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        RICHER_PROMPT_COMPOSER_DOC_REF.to_owned(),
        RICHER_PROMPT_COMPOSER_BASE_CONTRACT_REF.to_owned(),
        RICHER_PROMPT_COMPOSER_SCHEMA_REF.to_owned(),
        RICHER_PROMPT_COMPOSER_BETA_ARTIFACT_REF.to_owned(),
        RICHER_PROMPT_COMPOSER_STABLE_ARTIFACT_REF.to_owned(),
    ]
}

fn packet() -> RicherPromptComposerPacket {
    RicherPromptComposerPacket::new(RicherPromptComposerInput {
        packet_id: PACKET_ID.to_owned(),
        workflow_or_surface_id: "desktop-composer".to_owned(),
        display_label: "M5 Richer Prompt Composer".to_owned(),
        composer_conformance_packet_ref: "conformance-001".to_owned(),
        composer_stabilization_packet_ref: "stabilization-001".to_owned(),
        composer_context_snapshot_ref: "snapshot-001".to_owned(),
        composer_session_ref: "session-001".to_owned(),
        composer_draft_ref: "draft-001".to_owned(),
        thread_header: thread_header(),
        intent_row: intent_row(),
        attachment_rows: attachment_rows(),
        pinned_context_rows: pinned_rows(),
        omitted_context_rows: omitted_rows(),
        budget_strip: budget_strip(),
        context_drift_banners: vec![],
        compare_answer_rows: vec![],
        forked_thread_lineage: forked_thread_lineage(),
        surface_consistency_rows: surface_consistency_rows(),
        evidence_lineage: evidence_lineage(),
        source_contract_refs: source_contract_refs(),
        json_export_ref: RICHER_PROMPT_COMPOSER_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: RICHER_PROMPT_COMPOSER_SUMMARY_REF.to_owned(),
        minted_at: "2026-06-10T09:22:12Z".to_owned(),
    })
}

#[test]
fn packet_serializes_and_deserializes() {
    let original = packet();
    let json = original.export_safe_json();
    let round_tripped: RicherPromptComposerPacket = serde_json::from_str(&json).unwrap();
    assert_eq!(original, round_tripped);
}

#[test]
fn validate_self_passes_for_valid_packet() {
    let packet = packet();
    let violations = packet.validate_self();
    assert!(violations.is_empty(), "expected no violations, got: {violations:?}");
}

#[test]
fn validate_self_fails_on_missing_identity() {
    let mut packet = packet();
    packet.packet_id = "".to_owned();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::MissingIdentity));
}

#[test]
fn validate_self_fails_on_missing_source_contracts() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::MissingSourceContracts));
}

#[test]
fn validate_self_fails_on_wrong_record_kind() {
    let mut packet = packet();
    packet.record_kind = "wrong_kind".to_owned();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::WrongRecordKind));
}

#[test]
fn validate_self_fails_on_wrong_schema_version() {
    let mut packet = packet();
    packet.schema_version = 99;
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::WrongSchemaVersion));
}

#[test]
fn validate_self_fails_on_incomplete_intent_constraints() {
    let mut packet = packet();
    packet.intent_row.behavior_constraints.clear();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::IntentModeConstraintsMissing));
}

#[test]
fn validate_self_fails_on_attachment_without_semantic_role() {
    let mut packet = packet();
    packet.attachment_rows[0].semantic_role = AttachmentSemanticRoleClass::PrimaryContext;
    packet.attachment_rows[0].screen_reader_label.clear();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::AttachmentRichnessIncomplete));
}

#[test]
fn validate_self_fails_on_missing_attachment_source_coverage() {
    let mut packet = packet();
    packet.attachment_rows.retain(|row| {
        !matches!(
            row.source_class,
            StableAttachmentSourceClass::WorkspaceFile
                | StableAttachmentSourceClass::Symbol
                | StableAttachmentSourceClass::DocsReference
                | StableAttachmentSourceClass::Diagnostic
                | StableAttachmentSourceClass::TestResult
                | StableAttachmentSourceClass::TerminalToolOutput
                | StableAttachmentSourceClass::ExternalText
        )
    });
    let violations = packet.validate_self();
    assert!(
        violations.contains(&RicherPromptComposerViolation::AttachmentSourceClassCoverageMissing)
    );
}

#[test]
fn validate_self_fails_on_pin_without_policy() {
    let mut packet = packet();
    packet.pinned_context_rows[0].pin_policy = PinPolicyClass::StaleAfterDuration;
    packet.pinned_context_rows[0].stale_after_duration_seconds = None;
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::PinPolicyIncomplete));
}

#[test]
fn validate_self_fails_on_stale_pin_without_drift() {
    let mut packet = packet();
    packet.pinned_context_rows[1].drift_source = None;
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::PinnedStaleNotSurfaced));
}

#[test]
fn validate_self_fails_on_omitted_without_restoration() {
    let mut packet = packet();
    packet.omitted_context_rows[0].restoration_action_ref.clear();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::OmittedContextNotInspectable));
}

#[test]
fn validate_self_fails_on_budget_overflow_without_explanation() {
    let mut packet = packet();
    packet.budget_strip.pressure_class = BudgetPressureClass::Overflow;
    packet.budget_strip.explanation_label.clear();
    let violations = packet.validate_self();
    assert!(
        violations.contains(&RicherPromptComposerViolation::BudgetOverflowWithoutExplanation)
    );
}

#[test]
fn validate_self_fails_on_missing_surface_consistency() {
    let mut packet = packet();
    packet.surface_consistency_rows.clear();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::SurfaceConsistencyMissing));
}

#[test]
fn validate_self_fails_on_incomplete_evidence_lineage() {
    let mut packet = packet();
    packet.evidence_lineage.route_receipt_ref.clear();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::EvidenceLineageIncomplete));
}

#[test]
fn validate_self_fails_on_missing_evidence_packet_class() {
    let mut packet = packet();
    packet.evidence_lineage.packet_classes.clear();
    let violations = packet.validate_self();
    assert!(violations.contains(&RicherPromptComposerViolation::EvidencePacketClassMissing));
}

#[test]
fn markdown_summary_renders() {
    let packet = packet();
    let summary = packet.render_markdown_summary();
    assert!(summary.contains("Richer Prompt Composer (M5)"));
    assert!(summary.contains(PACKET_ID));
    assert!(summary.contains(&packet.composer_conformance_packet_ref));
    assert!(summary.contains(&packet.composer_stabilization_packet_ref));
}

#[test]
fn current_export_loads_and_validates() {
    let result = current_richer_prompt_composer_export();
    assert!(result.is_ok(), "export load failed: {result:?}");
    let packet = result.unwrap();
    assert_eq!(packet.record_kind, RICHER_PROMPT_COMPOSER_RECORD_KIND);
    assert_eq!(packet.schema_version, RICHER_PROMPT_COMPOSER_SCHEMA_VERSION);
}
