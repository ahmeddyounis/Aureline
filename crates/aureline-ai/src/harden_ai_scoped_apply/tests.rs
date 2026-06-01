use super::*;

const APPLY_ID: &str = "ai-apply:scoped:stable:0001";

fn lifecycle() -> ApplyLifecycleBlock {
    ApplyLifecycleBlock {
        lifecycle_state: ApplyLifecycleStateClass::AppliedKept,
        preview_record_ref: "preview:apply:scoped:stable:0001".to_owned(),
        preview_shown: true,
        approval_required: true,
        approval_record_ref: Some("approval:apply:scoped:stable:0001".to_owned()),
        approval_granted: true,
        checkpoint_captured_before_apply: true,
        rollback_checkpoint_ref: Some("checkpoint:apply:scoped:stable:0001".to_owned()),
        mutation_journal_ref: Some("journal:apply:scoped:stable:0001".to_owned()),
        apply_audit_ref: Some("apply-audit:scoped:stable:0001".to_owned()),
        revert_available: true,
        revert_handle_ref: Some("rollback-handle:apply:scoped:stable:0001".to_owned()),
        direct_trusted_apply_attempted: false,
    }
}

fn scope_contract() -> ScopeContractBlock {
    ScopeContractBlock {
        declared_scope_label: "Edit the retry module and its tests; create one fixture.".to_owned(),
        scope_class: ApplyWriteScopeClass::MultiFileBounded,
        requested_scope_ref: "requested-scope:apply:scoped:stable:0001".to_owned(),
        declared_path_class_count: 3,
        apply_bounded_to_declared_scope: true,
    }
}

fn patch_honesty() -> PatchHonestyBlock {
    let files = vec![
        PatchFileRow {
            file_ref: "file:retry-module".to_owned(),
            change_kind: PatchChangeKind::ModifyFile,
            hunk_count: 3,
            within_declared_scope: true,
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
            rename_from_ref: None,
        },
        PatchFileRow {
            file_ref: "file:retry-fixture".to_owned(),
            change_kind: PatchChangeKind::CreateFile,
            hunk_count: 1,
            within_declared_scope: true,
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
            rename_from_ref: None,
        },
        PatchFileRow {
            file_ref: "file:legacy-retry-shim".to_owned(),
            change_kind: PatchChangeKind::DeleteFile,
            hunk_count: 0,
            within_declared_scope: true,
            disclosed_in_preview: true,
            approved_for_apply: true,
            reached_live_tree: true,
            rename_from_ref: None,
        },
        PatchFileRow {
            file_ref: "file:retry-tests-renamed".to_owned(),
            change_kind: PatchChangeKind::RenameFile,
            hunk_count: 1,
            within_declared_scope: true,
            disclosed_in_preview: true,
            // Disclosed and approved but the operator deselected this hunk, so it
            // never reached the live tree.
            approved_for_apply: false,
            reached_live_tree: false,
            rename_from_ref: Some("file:retry-tests".to_owned()),
        },
    ];
    PatchHonestyBlock {
        patch_digest_ref: "sha256:0000000000000000000000000000000000000000000000000000000000000001"
            .to_owned(),
        declared_file_count: files.len() as u32,
        disclosed_file_count: files
            .iter()
            .filter(|file| file.disclosed_in_preview)
            .count() as u32,
        files,
    }
}

fn surface_parity() -> Vec<SurfaceParityRow> {
    CommandSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| SurfaceParityRow {
            surface_class,
            descriptor_ref: format!("descriptor:cmd.ai.apply_patch:{}", surface_class.as_str()),
            shares_command_descriptor: true,
            shares_preview_model: true,
            shares_approval_model: true,
            shares_result_model: true,
            shares_rollback_model: true,
            route_disclosed: true,
            policy_checked: true,
            reachable: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn route_authority() -> RouteSpendAuthorityBlock {
    RouteSpendAuthorityBlock {
        provider_label: "Aureline managed hosted AI".to_owned(),
        model_label: "Hosted apply review".to_owned(),
        route_receipt_ref: "route-receipt:apply:scoped:stable:0001".to_owned(),
        spend_receipt_ref: "spend-receipt:apply:scoped:stable:0001".to_owned(),
        egress_disclosed: true,
        tainted_context_present: true,
        tainted_context_fence_ref: Some("taint-fence:apply:scoped:stable:0001".to_owned()),
        authority_widened_without_disclosure: false,
    }
}

fn evidence_export() -> EvidenceExportBlock {
    EvidenceExportBlock {
        evidence_packet_ref: "ai-evidence:apply:scoped:stable:0001".to_owned(),
        patch_review_summary_ref: "patch-review:apply:scoped:stable:0001".to_owned(),
        rollback_handle_ref: "rollback-handle:apply:scoped:stable:0001".to_owned(),
        json_export_ref: AI_SCOPED_APPLY_HARDENING_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: AI_SCOPED_APPLY_HARDENING_SUMMARY_REF.to_owned(),
        export_lineage_refs: vec!["export:operator:apply:scoped:stable:0001".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        AI_SCOPED_APPLY_HARDENING_AI_DOC_REF.to_owned(),
        AI_SCOPED_APPLY_HARDENING_PREVIEW_APPLY_REVERT_CONTRACT_REF.to_owned(),
        AI_SCOPED_APPLY_HARDENING_PARITY_CONTRACT_REF.to_owned(),
        AI_SCOPED_APPLY_HARDENING_SCHEMA_REF.to_owned(),
    ]
}

fn input() -> AiScopedApplyHardeningPacketInput {
    AiScopedApplyHardeningPacketInput {
        packet_id: "ai-scoped-apply-hardening:stable:0001".to_owned(),
        apply_id: APPLY_ID.to_owned(),
        display_label: "AI scoped-apply hardening".to_owned(),
        command_descriptor_ref: "cmd:ai.apply_patch".to_owned(),
        command_revision: "rev:cmd.ai.apply_patch:0007".to_owned(),
        trust_state_token: "trusted".to_owned(),
        policy_epoch_ref: "policy-epoch:stable:0004".to_owned(),
        approval_path_label: "One-time apply approval".to_owned(),
        lifecycle: lifecycle(),
        scope_contract: scope_contract(),
        patch_honesty: patch_honesty(),
        surface_parity: surface_parity(),
        route_authority: route_authority(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-05-31T22:00:00Z".to_owned(),
    }
}

fn packet() -> AiScopedApplyHardeningPacket {
    AiScopedApplyHardeningPacket::new(input())
}

#[test]
fn scoped_apply_hardening_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn applied_state_requires_preview() {
    let mut packet = packet();
    packet.lifecycle.preview_shown = false;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::AppliedWithoutPreviewOrApproval));
}

#[test]
fn applied_state_requires_granted_approval() {
    let mut packet = packet();
    packet.lifecycle.approval_granted = false;
    packet.lifecycle.approval_record_ref = None;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::AppliedWithoutPreviewOrApproval));
}

#[test]
fn direct_trusted_apply_is_rejected() {
    let mut packet = packet();
    packet.lifecycle.direct_trusted_apply_attempted = true;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::DirectApplyBypassedReview));
}

#[test]
fn applied_state_requires_checkpoint_and_audit() {
    let mut packet = packet();
    packet.lifecycle.checkpoint_captured_before_apply = false;
    packet.lifecycle.rollback_checkpoint_ref = None;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::ApplyLifecycleIncomplete));
}

#[test]
fn applied_state_requires_available_revert() {
    let mut packet = packet();
    packet.lifecycle.revert_available = false;
    packet.lifecycle.revert_handle_ref = None;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::RevertUnavailableAfterApply));
}

#[test]
fn rejected_state_must_not_mutate_tree() {
    let mut packet = packet();
    packet.lifecycle.lifecycle_state = ApplyLifecycleStateClass::RejectedNoApply;
    // The mutation journal and apply audit linger as if the patch had applied.

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::RejectedStateApplied));
}

#[test]
fn scope_must_stay_bounded() {
    let mut packet = packet();
    packet.scope_contract.apply_bounded_to_declared_scope = false;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::ScopeWidenedBeyondDeclared));
}

#[test]
fn out_of_scope_file_reaching_tree_is_rejected() {
    let mut packet = packet();
    packet.patch_honesty.files[0].within_declared_scope = false;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::ScopeWidenedBeyondDeclared));
}

#[test]
fn hidden_patch_file_is_rejected() {
    let mut packet = packet();
    // A file reaches the live tree but was never disclosed in the preview.
    packet.patch_honesty.files[0].disclosed_in_preview = false;
    // Keep the disclosed count honest with the rows so the dedicated honesty
    // count check does not mask the hidden-file check.
    packet.patch_honesty.disclosed_file_count = packet
        .patch_honesty
        .files
        .iter()
        .filter(|file| file.disclosed_in_preview)
        .count() as u32;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::HiddenPatchFile));
}

#[test]
fn unapproved_file_reaching_tree_is_rejected() {
    let mut packet = packet();
    packet.patch_honesty.files[0].approved_for_apply = false;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::UnapprovedFileApplied));
}

#[test]
fn declared_file_count_mismatch_is_rejected() {
    let mut packet = packet();
    packet.patch_honesty.declared_file_count += 1;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::PatchHonestyIncomplete));
}

#[test]
fn rename_without_source_is_rejected() {
    let mut packet = packet();
    let rename = packet
        .patch_honesty
        .files
        .iter_mut()
        .find(|file| file.change_kind == PatchChangeKind::RenameFile)
        .expect("rename row");
    rename.rename_from_ref = None;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::PatchHonestyIncomplete));
}

#[test]
fn surface_parity_must_cover_every_wedge() {
    let mut packet = packet();
    packet
        .surface_parity
        .retain(|row| row.surface_class != CommandSurfaceClass::DeepLink);

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::CommandSurfaceCoverageMissing));
}

#[test]
fn surface_parity_must_share_full_model() {
    let mut packet = packet();
    packet.surface_parity[0].shares_rollback_model = false;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::CommandParityBroken));
}

#[test]
fn unqualified_surface_must_not_claim_stable() {
    let mut packet = packet();
    let cli = packet
        .surface_parity
        .iter_mut()
        .find(|row| row.surface_class == CommandSurfaceClass::CliHeadless)
        .expect("cli row");
    // The surface is only beta-qualified but still claims the stable lane.
    cli.qualification = SurfaceQualificationClass::Beta;
    cli.claimed_stable = true;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::StableClaimNotQualified));
}

#[test]
fn route_authority_must_disclose_egress() {
    let mut packet = packet();
    packet.route_authority.egress_disclosed = false;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::RouteAuthorityIncomplete));
}

#[test]
fn tainted_context_must_be_fenced() {
    let mut packet = packet();
    packet.route_authority.tainted_context_fence_ref = None;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::TaintedContextUnfenced));
}

#[test]
fn authority_widening_without_disclosure_is_rejected() {
    let mut packet = packet();
    packet.route_authority.authority_widened_without_disclosure = true;

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::AuthorityWidenedWithoutDisclosure));
}

#[test]
fn missing_export_ref_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.rollback_handle_ref = String::new();

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::ExportRefsMissing));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != AI_SCOPED_APPLY_HARDENING_SCHEMA_REF);

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::MissingSourceContracts));
}

#[test]
fn raw_boundary_material_is_rejected() {
    let mut packet = packet();
    packet.scope_contract.declared_scope_label = "edit https://provider.example/v1".to_owned();

    assert!(packet
        .validate()
        .contains(&AiScopedApplyHardeningViolation::RawBoundaryMaterialInExport));
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/ai/m4/harden_ai_scoped_apply");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        format!("{dir}/support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
    std::fs::write(
        format!("{dir}/summary.md"),
        packet.render_markdown_summary(),
    )
    .unwrap();
    let fixture_dir = format!("{root}/fixtures/ai/m4/harden_ai_scoped_apply");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/scoped_apply_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_ai_scoped_apply_hardening_export()
        .expect("checked ai scoped-apply hardening export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}
