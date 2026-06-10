use super::*;

const PACKET_ID: &str = "ai-patch-review:evidence-rich:m5:0001";
const APPLY_ID: &str = "ai-apply:evidence-rich:m5:0001";

fn diff_packet() -> DiffPacketBlock {
    let files = vec![
        DiffFileRow {
            file_ref: "file:retry-module".to_owned(),
            change_kind: crate::harden_ai_scoped_apply::PatchChangeKind::ModifyFile,
            hunk_count: 3,
            within_declared_scope: true,
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
            rename_from_ref: None,
        },
        DiffFileRow {
            file_ref: "file:retry-fixture".to_owned(),
            change_kind: crate::harden_ai_scoped_apply::PatchChangeKind::CreateFile,
            hunk_count: 1,
            within_declared_scope: true,
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
            rename_from_ref: None,
        },
        DiffFileRow {
            file_ref: "file:legacy-retry-shim".to_owned(),
            change_kind: crate::harden_ai_scoped_apply::PatchChangeKind::DeleteFile,
            hunk_count: 0,
            within_declared_scope: true,
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
            rename_from_ref: None,
        },
        DiffFileRow {
            file_ref: "file:retry-tests-renamed".to_owned(),
            change_kind: crate::harden_ai_scoped_apply::PatchChangeKind::RenameFile,
            hunk_count: 1,
            within_declared_scope: true,
            disclosed_in_preview: true,
            approved_for_apply: false,
            reached_live_tree: false,
            rename_from_ref: Some("file:retry-tests".to_owned()),
        },
    ];
    let hunks = vec![
        DiffHunkRow {
            hunk_id: "hunk:retry-module:1".to_owned(),
            file_ref: "file:retry-module".to_owned(),
            hunk_digest: "sha256:0000000000000000000000000000000000000000000000000000000000000001"
                .to_owned(),
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
        },
        DiffHunkRow {
            hunk_id: "hunk:retry-module:2".to_owned(),
            file_ref: "file:retry-module".to_owned(),
            hunk_digest: "sha256:0000000000000000000000000000000000000000000000000000000000000002"
                .to_owned(),
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
        },
        DiffHunkRow {
            hunk_id: "hunk:retry-module:3".to_owned(),
            file_ref: "file:retry-module".to_owned(),
            hunk_digest: "sha256:0000000000000000000000000000000000000000000000000000000000000003"
                .to_owned(),
            disclosed_in_preview: true,
            approved_for_apply: false,
            reached_live_tree: false,
        },
        DiffHunkRow {
            hunk_id: "hunk:retry-fixture:1".to_owned(),
            file_ref: "file:retry-fixture".to_owned(),
            hunk_digest: "sha256:0000000000000000000000000000000000000000000000000000000000000004"
                .to_owned(),
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
        },
        DiffHunkRow {
            hunk_id: "hunk:retry-tests-renamed:1".to_owned(),
            file_ref: "file:retry-tests-renamed".to_owned(),
            hunk_digest: "sha256:0000000000000000000000000000000000000000000000000000000000000005"
                .to_owned(),
            disclosed_in_preview: true,
            approved_for_apply: false,
            reached_live_tree: false,
        },
    ];
    DiffPacketBlock {
        patch_digest_ref: "sha256:000000000000000000000000000000000000000000000000000000000000000a"
            .to_owned(),
        declared_file_count: files.len() as u32,
        disclosed_file_count: files
            .iter()
            .filter(|file| file.disclosed_in_preview)
            .count() as u32,
        files,
        hunks,
        patch_artifact_ref: "artifact:patch:evidence-rich:m5:0001".to_owned(),
    }
}

fn validation_receipt() -> ValidationReceiptBlock {
    ValidationReceiptBlock {
        receipt_id: "validation:evidence-rich:m5:0001".to_owned(),
        validations: vec![
            ValidationReceiptRow {
                kind: ValidationKindClass::Lint,
                outcome: ValidationOutcomeClass::Passed,
                validator_ref: "validator:lint:m5:0001".to_owned(),
                run_at: "2026-06-10T09:00:00Z".to_owned(),
                bound_patch_digest_ref:
                    "sha256:000000000000000000000000000000000000000000000000000000000000000a"
                        .to_owned(),
                disclosed_before_apply: true,
                blocked_apply: false,
            },
            ValidationReceiptRow {
                kind: ValidationKindClass::TypeCheck,
                outcome: ValidationOutcomeClass::Passed,
                validator_ref: "validator:type-check:m5:0001".to_owned(),
                run_at: "2026-06-10T09:01:00Z".to_owned(),
                bound_patch_digest_ref:
                    "sha256:000000000000000000000000000000000000000000000000000000000000000a"
                        .to_owned(),
                disclosed_before_apply: true,
                blocked_apply: false,
            },
            ValidationReceiptRow {
                kind: ValidationKindClass::Test,
                outcome: ValidationOutcomeClass::Passed,
                validator_ref: "validator:test:m5:0001".to_owned(),
                run_at: "2026-06-10T09:02:00Z".to_owned(),
                bound_patch_digest_ref:
                    "sha256:000000000000000000000000000000000000000000000000000000000000000a"
                        .to_owned(),
                disclosed_before_apply: true,
                blocked_apply: false,
            },
        ],
        overall_outcome: ValidationOutcomeClass::Passed,
        validation_required_before_apply: true,
        all_required_passed: true,
    }
}

fn rollback_handle() -> RollbackHandleBlock {
    RollbackHandleBlock {
        handle_id: "rollback:evidence-rich:m5:0001".to_owned(),
        scope: RollbackScopeClass::MultiFileBounded,
        state: RollbackStateClass::Available,
        checkpoint_ref: "checkpoint:evidence-rich:m5:0001".to_owned(),
        mutation_journal_ref: Some("journal:evidence-rich:m5:0001".to_owned()),
        revert_available: true,
        revert_handle_ref: Some("revert:evidence-rich:m5:0001".to_owned()),
        expires_at: Some("2026-06-17T09:00:00Z".to_owned()),
    }
}

fn apply_flow_bindings() -> Vec<ApplyFlowBindingRow> {
    vec![
        ApplyFlowBindingRow {
            flow: ApplyFlowClass::InlineAssist,
            diff_state: DiffPacketStateClass::Applied,
            has_diff_packet: true,
            has_validation_receipt: true,
            has_rollback_handle: true,
            preview_shown: true,
            approval_required: true,
            approval_granted: true,
            apply_bounded_to_scope: true,
        },
        ApplyFlowBindingRow {
            flow: ApplyFlowClass::PatchReview,
            diff_state: DiffPacketStateClass::Approved,
            has_diff_packet: true,
            has_validation_receipt: true,
            has_rollback_handle: true,
            preview_shown: true,
            approval_required: true,
            approval_granted: true,
            apply_bounded_to_scope: true,
        },
        ApplyFlowBindingRow {
            flow: ApplyFlowClass::BranchOrWorktreeAgent,
            diff_state: DiffPacketStateClass::Proposed,
            has_diff_packet: true,
            has_validation_receipt: true,
            has_rollback_handle: true,
            preview_shown: true,
            approval_required: true,
            approval_granted: false,
            apply_bounded_to_scope: true,
        },
    ]
}

fn consumer_surface_parity() -> Vec<ConsumerSurfaceParityRow> {
    ConsumerSurfaceClass::ALL
        .into_iter()
        .map(|surface| ConsumerSurfaceParityRow {
            surface,
            shows_diff_packet: true,
            shows_validation_receipt: true,
            shows_rollback_handle: true,
            shows_apply_flow_binding: true,
            reachable: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn source_contract_refs() -> Vec<String> {
    vec![
        EVIDENCE_RICH_PATCH_REVIEW_DOC_REF.to_owned(),
        EVIDENCE_RICH_PATCH_REVIEW_SCHEMA_REF.to_owned(),
        EVIDENCE_RICH_PATCH_REVIEW_PATCH_SEQUENCE_REF.to_owned(),
        EVIDENCE_RICH_PATCH_REVIEW_M5_MATRIX_CONTRACT_REF.to_owned(),
        EVIDENCE_RICH_PATCH_REVIEW_PREVIEW_APPLY_REVERT_CONTRACT_REF.to_owned(),
    ]
}

fn packet_input() -> EvidenceRichPatchReviewPacketInput {
    EvidenceRichPatchReviewPacketInput {
        packet_id: PACKET_ID.to_owned(),
        apply_id: APPLY_ID.to_owned(),
        display_label: "M5 evidence-rich patch review for retry module refactor".to_owned(),
        trust_state_token: "restricted".to_owned(),
        policy_epoch_ref: "policy-epoch:m5:2026-06-01".to_owned(),
        diff_packet: diff_packet(),
        validation_receipt: validation_receipt(),
        rollback_handle: rollback_handle(),
        apply_flow_bindings: apply_flow_bindings(),
        consumer_surface_parity: consumer_surface_parity(),
        downgrade_triggers: vec![
            DowngradeTrigger::ProofStale,
            DowngradeTrigger::PolicyBlocked,
            DowngradeTrigger::ProviderUnavailable,
            DowngradeTrigger::TrustNarrowing,
            DowngradeTrigger::ScopeExpansionUnqualified,
            DowngradeTrigger::UpstreamDependencyNarrowed,
            DowngradeTrigger::ValidationFailedOrMissing,
            DowngradeTrigger::RollbackUnavailable,
        ],
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T09:44:16Z".to_owned(),
    }
}

#[test]
fn packet_constructs_and_serializes() {
    let packet = EvidenceRichPatchReviewPacket::new(packet_input());
    let json = packet.export_safe_json();
    assert!(json.contains("evidence_rich_patch_review"));
    assert!(json.contains(PACKET_ID));
}

#[test]
fn valid_packet_passes_validation() {
    let packet = EvidenceRichPatchReviewPacket::new(packet_input());
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "expected no violations, got: {:?}",
        violations
    );
}

#[test]
fn wrong_record_kind_fails() {
    let mut input = packet_input();
    input.packet_id = PACKET_ID.to_owned();
    let mut packet = EvidenceRichPatchReviewPacket::new(input);
    packet.record_kind = "wrong_kind".to_owned();
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::WrongRecordKind));
}

#[test]
fn wrong_schema_version_fails() {
    let mut input = packet_input();
    input.packet_id = PACKET_ID.to_owned();
    let mut packet = EvidenceRichPatchReviewPacket::new(input);
    packet.schema_version = 999;
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::WrongSchemaVersion));
}

#[test]
fn missing_identity_fails() {
    let mut input = packet_input();
    input.packet_id = "   ".to_owned();
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::MissingIdentity));
}

#[test]
fn missing_source_contracts_fails() {
    let mut input = packet_input();
    input.source_contract_refs = vec![];
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::MissingSourceContracts));
}

#[test]
fn hidden_patch_file_fails() {
    let mut input = packet_input();
    input.diff_packet.files[0].disclosed_in_preview = false;
    input.diff_packet.files[0].reached_live_tree = true;
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::HiddenPatchFile));
}

#[test]
fn unapproved_file_applied_fails() {
    let mut input = packet_input();
    input.diff_packet.files[0].approved_for_apply = false;
    input.diff_packet.files[0].reached_live_tree = true;
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::UnapprovedFileApplied));
}

#[test]
fn diff_packet_count_mismatch_fails() {
    let mut input = packet_input();
    input.diff_packet.declared_file_count = 99;
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::DiffPacketCountMismatch));
}

#[test]
fn validation_receipt_missing_fails() {
    let mut input = packet_input();
    input.validation_receipt.validation_required_before_apply = true;
    input.validation_receipt.validations = vec![];
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::ValidationReceiptMissing));
}

#[test]
fn validation_failed_but_applied_fails() {
    let mut input = packet_input();
    input.validation_receipt.validation_required_before_apply = true;
    input.validation_receipt.all_required_passed = false;
    input.validation_receipt.overall_outcome = ValidationOutcomeClass::Failed;
    // Force an applied state in bindings.
    input.apply_flow_bindings[0].diff_state = DiffPacketStateClass::Applied;
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::ValidationFailedButApplied));
}

#[test]
fn rollback_unavailable_after_apply_fails() {
    let mut input = packet_input();
    input.rollback_handle.revert_available = false;
    input.apply_flow_bindings[0].diff_state = DiffPacketStateClass::Applied;
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::RollbackUnavailableAfterApply));
}

#[test]
fn apply_flow_binding_missing_fails() {
    let mut input = packet_input();
    input.apply_flow_bindings = vec![input.apply_flow_bindings[0].clone()];
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::ApplyFlowBindingMissing));
}

#[test]
fn applied_without_preview_or_approval_fails() {
    let mut input = packet_input();
    input.apply_flow_bindings[0].preview_shown = false;
    input.apply_flow_bindings[0].approval_granted = false;
    input.apply_flow_bindings[0].diff_state = DiffPacketStateClass::Applied;
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::AppliedWithoutPreviewOrApproval));
}

#[test]
fn consumer_surface_coverage_missing_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity = vec![input.consumer_surface_parity[0].clone()];
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.contains(&EvidenceRichPatchReviewViolation::ConsumerSurfaceCoverageMissing)
    );
}

#[test]
fn stable_claim_not_qualified_fails() {
    let mut input = packet_input();
    input.consumer_surface_parity[0].claimed_stable = true;
    input.consumer_surface_parity[0].reachable = false;
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(violations.contains(&EvidenceRichPatchReviewViolation::StableClaimNotQualified));
}

#[test]
fn raw_boundary_material_detected() {
    let mut input = packet_input();
    input.diff_packet.patch_artifact_ref =
        "https://internal.aureline.dev/artifacts/patch-secret".to_owned();
    let packet = EvidenceRichPatchReviewPacket::new(input);
    let violations = packet.validate();
    assert!(
        violations.contains(&EvidenceRichPatchReviewViolation::RawBoundaryMaterialInExport)
    );
}

#[test]
fn markdown_summary_renders() {
    let packet = EvidenceRichPatchReviewPacket::new(packet_input());
    let md = packet.render_markdown_summary();
    assert!(md.starts_with("# Evidence-Rich Patch Review"));
    assert!(md.contains(PACKET_ID));
    assert!(md.contains(APPLY_ID));
}

#[test]
fn checked_in_export_loads_and_validates() {
    let result = current_stable_evidence_rich_patch_review_export();
    assert!(
        result.is_ok(),
        "checked-in export should load and validate: {:?}",
        result
    );
}
