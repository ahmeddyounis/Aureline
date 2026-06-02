use super::*;

const PACKET_ID: &str = "ai-harden-repo-ai-instructions:stable:0001";

fn instruction_rows() -> Vec<RepoInstructionRow> {
    vec![
        RepoInstructionRow {
            instruction_ref: "instruction:designated-policy:0001".to_owned(),
            source_path_label: "Signed admin policy bundle".to_owned(),
            source_class: RepoInstructionSourceClass::DesignatedPolicyFile,
            trust_posture_class: InstructionTrustPostureClass::SignedAdminAuthority,
            origin_locus_class: TaintedContextOriginLocusClass::LocalInProcess,
            precedence_rank: RepoInstructionSourceClass::DesignatedPolicyFile
                .canonical_precedence_rank(),
            claims_widening_authority: true,
            may_narrow_authority: true,
            policy_vetted: true,
            fenced_to_data_only: false,
            signing_evidence_ref: Some("signing-evidence:designated-policy:0001".to_owned()),
            raw_body_forbidden: true,
            user_visible_label: "Signed admin policy file (full authority)".to_owned(),
        },
        RepoInstructionRow {
            instruction_ref: "instruction:repo-bundle:0002".to_owned(),
            source_path_label: "Repo-authored instruction bundle".to_owned(),
            source_class: RepoInstructionSourceClass::RepoInstructionBundle,
            trust_posture_class: InstructionTrustPostureClass::TrustedFirstPartyAuthored,
            origin_locus_class: TaintedContextOriginLocusClass::LocalInProcess,
            precedence_rank: RepoInstructionSourceClass::RepoInstructionBundle
                .canonical_precedence_rank(),
            claims_widening_authority: false,
            may_narrow_authority: true,
            policy_vetted: true,
            fenced_to_data_only: false,
            signing_evidence_ref: None,
            raw_body_forbidden: true,
            user_visible_label: "Repo instruction bundle (may narrow only)".to_owned(),
        },
        RepoInstructionRow {
            instruction_ref: "instruction:workspace-pinned:0003".to_owned(),
            source_path_label: "Workspace-pinned policy".to_owned(),
            source_class: RepoInstructionSourceClass::TrustedWorkspacePinnedPolicy,
            trust_posture_class: InstructionTrustPostureClass::WorkspacePinned,
            origin_locus_class: TaintedContextOriginLocusClass::LocalInProcess,
            precedence_rank: RepoInstructionSourceClass::TrustedWorkspacePinnedPolicy
                .canonical_precedence_rank(),
            claims_widening_authority: false,
            may_narrow_authority: true,
            policy_vetted: true,
            fenced_to_data_only: false,
            signing_evidence_ref: None,
            raw_body_forbidden: true,
            user_visible_label: "Workspace-pinned policy (may narrow only)".to_owned(),
        },
        RepoInstructionRow {
            instruction_ref: "instruction:unknown:0004".to_owned(),
            source_path_label: "Unclassified instruction source".to_owned(),
            source_class: RepoInstructionSourceClass::UnknownRepoInstructionFailClosed,
            trust_posture_class: InstructionTrustPostureClass::UntrustedMustFence,
            origin_locus_class: TaintedContextOriginLocusClass::UnknownLocusMustBeDisclosed,
            precedence_rank: RepoInstructionSourceClass::UnknownRepoInstructionFailClosed
                .canonical_precedence_rank(),
            claims_widening_authority: false,
            may_narrow_authority: true,
            policy_vetted: false,
            fenced_to_data_only: true,
            signing_evidence_ref: None,
            raw_body_forbidden: true,
            user_visible_label: "Unknown source fenced to data only".to_owned(),
        },
    ]
}

fn policy_interaction_rows() -> Vec<PolicyInteractionRow> {
    vec![
        PolicyInteractionRow {
            interaction_ref: "interaction:overrides:0001".to_owned(),
            instruction_ref: "instruction:repo-bundle:0002".to_owned(),
            repo_claim_label: "Repo bundle requested a retention window".to_owned(),
            governing_authority_class: RepoInstructionSourceClass::DesignatedPolicyFile,
            outcome_class: PolicyInteractionOutcomeClass::PolicyOverridesRepo,
            prohibited_case_class: None,
            effective_mode_class: TaintedContextRunModeClass::FullRun,
            auditable: true,
            user_visible_label: "Designated policy overrode the repo retention claim".to_owned(),
        },
        PolicyInteractionRow {
            interaction_ref: "interaction:narrowing:0002".to_owned(),
            instruction_ref: "instruction:repo-bundle:0002".to_owned(),
            repo_claim_label: "Repo bundle narrowed tool access to read-only".to_owned(),
            governing_authority_class: RepoInstructionSourceClass::DesignatedPolicyFile,
            outcome_class: PolicyInteractionOutcomeClass::RepoNarrowingAdmitted,
            prohibited_case_class: None,
            effective_mode_class: TaintedContextRunModeClass::PreviewOnly,
            auditable: true,
            user_visible_label: "Repo narrowing to read-only admitted".to_owned(),
        },
        PolicyInteractionRow {
            interaction_ref: "interaction:widening-denied:0003".to_owned(),
            instruction_ref: "instruction:repo-bundle:0002".to_owned(),
            repo_claim_label: "Repo bundle text tried to widen egress".to_owned(),
            governing_authority_class: RepoInstructionSourceClass::DesignatedPolicyFile,
            outcome_class: PolicyInteractionOutcomeClass::RepoWideningDenied,
            prohibited_case_class: Some(RepoProhibitedCaseClass::RepoTextWideningAttempted),
            effective_mode_class: TaintedContextRunModeClass::Blocked,
            auditable: true,
            user_visible_label: "Repo egress-widening attempt denied".to_owned(),
        },
    ]
}

fn kill_switch() -> KillSwitchPosture {
    KillSwitchPosture {
        kill_switch_ref: "kill-switch:provider-neutral:0001".to_owned(),
        state_class: KillSwitchStateClass::DisengagedNormalRouting,
        scope_class: KillSwitchScopeClass::AllProvidersAndTools,
        provider_neutral: true,
        fails_closed: true,
        disables_hosted_routing: true,
        disables_local_routing: true,
        disables_external_tools: true,
        re_arm_requires_approval: true,
        effective_mode_when_engaged: TaintedContextRunModeClass::Blocked,
        user_visible_label: "Provider-neutral AI kill switch (armed, disengaged)".to_owned(),
    }
}

fn backout() -> BackoutPosture {
    BackoutPosture {
        backout_ref: "backout:repo-instructions:0001".to_owned(),
        completeness_class: BackoutCompletenessClass::FullBackoutNoPartialWrites,
        rollback_checkpoint_ref: "checkpoint:repo-instructions:0001".to_owned(),
        reversible: true,
        evidence_linked: true,
        triggered_by_kill_switch: true,
        user_visible_label: "Full backout to checkpoint, no partial writes".to_owned(),
    }
}

fn surface_parity_rows() -> Vec<CommandSurfaceParityRow> {
    CommandSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface_class| CommandSurfaceParityRow {
            surface_class,
            descriptor_ref: "descriptor:harden-repo-ai-instructions:0001".to_owned(),
            shares_command_descriptor: true,
            shares_preview_model: true,
            shares_approval_model: true,
            shares_result_model: true,
            shares_rollback_model: true,
            honors_instruction_precedence: true,
            obeys_kill_switch: true,
            honors_backout_posture: true,
            route_disclosed: true,
            policy_checked: true,
            reachable: true,
            qualification: SurfaceQualificationClass::Stable,
            claimed_stable: true,
        })
        .collect()
}

fn evidence_export() -> RepoInstructionEvidenceExport {
    RepoInstructionEvidenceExport {
        evidence_id: "ai-evidence:harden-repo-ai-instructions:stable:0001".to_owned(),
        json_export_ref: HARDEN_REPO_AI_INSTRUCTIONS_ARTIFACT_REF.to_owned(),
        markdown_summary_ref: HARDEN_REPO_AI_INSTRUCTIONS_SUMMARY_REF.to_owned(),
        admin_inspector_ref: "admin-inspector:harden-repo-ai-instructions:0001".to_owned(),
        support_export_ref: "support-export:harden-repo-ai-instructions:0001".to_owned(),
        rollback_lineage_refs: vec![
            "checkpoint:repo-instructions:0001".to_owned(),
            "checkpoint:kill-switch:0001".to_owned(),
        ],
        export_lineage_refs: vec!["export:operator:harden-repo-ai-instructions:0001".to_owned()],
    }
}

fn source_contract_refs() -> Vec<String> {
    vec![
        HARDEN_REPO_AI_INSTRUCTIONS_AI_DOC_REF.to_owned(),
        HARDEN_REPO_AI_INSTRUCTIONS_SCHEMA_REF.to_owned(),
        HARDEN_REPO_AI_INSTRUCTIONS_TAINT_CONTRACT_REF.to_owned(),
        HARDEN_REPO_AI_INSTRUCTIONS_KILL_SWITCH_CONTRACT_REF.to_owned(),
    ]
}

fn input() -> RepoAiInstructionHardeningPacketInput {
    RepoAiInstructionHardeningPacketInput {
        packet_id: PACKET_ID.to_owned(),
        workflow_or_surface_id: "workflow:ai-run-governed-by-repo-instructions".to_owned(),
        display_label: "Harden repo-defined AI instructions".to_owned(),
        context_snapshot_ref: "context-snapshot:harden-repo-ai-instructions:0001".to_owned(),
        evidence_packet_ref: "evidence-packet:harden-repo-ai-instructions:0001".to_owned(),
        claimed_stable: true,
        trust_state_token: "trusted".to_owned(),
        policy_epoch_ref: "policy-epoch:stable:0004".to_owned(),
        instruction_rows: instruction_rows(),
        policy_interaction_rows: policy_interaction_rows(),
        kill_switch: kill_switch(),
        backout: backout(),
        surface_parity_rows: surface_parity_rows(),
        evidence_export: evidence_export(),
        source_contract_refs: source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-05-31T23:45:00Z".to_owned(),
    }
}

fn packet() -> RepoAiInstructionHardeningPacket {
    RepoAiInstructionHardeningPacket::new(input())
}

#[test]
fn hardened_repo_instruction_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn instruction_rows_are_required() {
    let mut packet = packet();
    packet.instruction_rows.clear();

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::MissingInstructionRows));
}

#[test]
fn instruction_coverage_is_required() {
    let mut packet = packet();
    packet
        .instruction_rows
        .retain(|row| row.source_class != RepoInstructionSourceClass::DesignatedPolicyFile);

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::MissingInstructionCoverage));
}

#[test]
fn repo_bundle_may_not_claim_widening_authority() {
    let mut packet = packet();
    if let Some(row) = packet
        .instruction_rows
        .iter_mut()
        .find(|row| row.source_class == RepoInstructionSourceClass::RepoInstructionBundle)
    {
        row.claims_widening_authority = true;
    }

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::InstructionClaimsWideningAuthority));
}

#[test]
fn instruction_precedence_must_match_canonical_rank() {
    let mut packet = packet();
    if let Some(row) = packet
        .instruction_rows
        .iter_mut()
        .find(|row| row.source_class == RepoInstructionSourceClass::RepoInstructionBundle)
    {
        // Claiming a rank above the designated policy file inverts precedence.
        row.precedence_rank = 1;
    }

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::InstructionPrecedenceInconsistent));
}

#[test]
fn designated_policy_requires_signing_evidence() {
    let mut packet = packet();
    if let Some(row) = packet
        .instruction_rows
        .iter_mut()
        .find(|row| row.source_class == RepoInstructionSourceClass::DesignatedPolicyFile)
    {
        row.signing_evidence_ref = None;
    }

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::DesignatedPolicySigningEvidenceMissing));
}

#[test]
fn unknown_instruction_must_fail_closed() {
    let mut packet = packet();
    if let Some(row) = packet.instruction_rows.iter_mut().find(|row| {
        row.source_class == RepoInstructionSourceClass::UnknownRepoInstructionFailClosed
    }) {
        row.fenced_to_data_only = false;
        // Keep it from tripping the vetted-or-fenced rule first.
        row.policy_vetted = true;
    }

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::UnknownInstructionNotFailClosed));
}

#[test]
fn instruction_must_be_vetted_or_fenced() {
    let mut packet = packet();
    if let Some(row) = packet
        .instruction_rows
        .iter_mut()
        .find(|row| row.source_class == RepoInstructionSourceClass::RepoInstructionBundle)
    {
        row.policy_vetted = false;
        row.fenced_to_data_only = false;
    }

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::InstructionNeitherVettedNorFenced));
}

#[test]
fn policy_interaction_coverage_is_required() {
    let mut packet = packet();
    packet
        .policy_interaction_rows
        .retain(|row| row.outcome_class != PolicyInteractionOutcomeClass::RepoWideningDenied);

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::MissingPolicyInteractionCoverage));
}

#[test]
fn widening_denial_requires_prohibited_case() {
    let mut packet = packet();
    if let Some(row) = packet
        .policy_interaction_rows
        .iter_mut()
        .find(|row| row.outcome_class == PolicyInteractionOutcomeClass::RepoWideningDenied)
    {
        row.prohibited_case_class = None;
    }

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::WideningDenialMissingProhibitedCase));
}

#[test]
fn widening_denial_must_not_leave_full_run() {
    let mut packet = packet();
    if let Some(row) = packet
        .policy_interaction_rows
        .iter_mut()
        .find(|row| row.outcome_class == PolicyInteractionOutcomeClass::RepoWideningDenied)
    {
        row.effective_mode_class = TaintedContextRunModeClass::FullRun;
    }

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::WideningDenialNotBlocked));
}

#[test]
fn kill_switch_must_be_provider_neutral() {
    let mut packet = packet();
    packet.kill_switch.scope_class = KillSwitchScopeClass::SingleProvider;
    packet.kill_switch.provider_neutral = false;

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::KillSwitchNotProviderNeutral));
}

#[test]
fn kill_switch_scope_flag_must_agree() {
    let mut packet = packet();
    // Provider-neutral flag lies about a single-provider scope.
    packet.kill_switch.scope_class = KillSwitchScopeClass::SingleProvider;
    packet.kill_switch.provider_neutral = true;

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::KillSwitchScopeMismatch));
}

#[test]
fn kill_switch_must_fail_closed() {
    let mut packet = packet();
    packet.kill_switch.disables_local_routing = false;

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::KillSwitchDoesNotFailClosed));
}

#[test]
fn kill_switch_re_arm_must_be_gated() {
    let mut packet = packet();
    packet.kill_switch.re_arm_requires_approval = false;

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::KillSwitchReArmNotGated));
}

#[test]
fn engaged_kill_switch_must_block() {
    let mut packet = packet();
    packet.kill_switch.state_class = KillSwitchStateClass::EngagedFailClosed;
    packet.kill_switch.effective_mode_when_engaged = TaintedContextRunModeClass::LocalOnly;

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::KillSwitchEngagedNotBlocking));
}

#[test]
fn engaged_and_blocking_kill_switch_validates() {
    let mut packet = packet();
    packet.kill_switch.state_class = KillSwitchStateClass::EngagedFailClosed;
    packet.kill_switch.effective_mode_when_engaged = TaintedContextRunModeClass::Blocked;

    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn backout_requires_checkpoint() {
    let mut packet = packet();
    packet.backout.rollback_checkpoint_ref = String::new();

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::BackoutPostureIncomplete));
}

#[test]
fn claimed_stable_requires_full_backout() {
    let mut packet = packet();
    packet.backout.completeness_class = BackoutCompletenessClass::PartialBackoutNeedsReview;

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::BackoutNotStableQualified));
}

#[test]
fn non_stable_run_admits_partial_backout() {
    let mut packet = packet();
    packet.claimed_stable = false;
    packet.backout.completeness_class = BackoutCompletenessClass::PartialBackoutNeedsReview;
    // A run that does not claim Stable must also narrow its surfaces.
    for row in &mut packet.surface_parity_rows {
        row.qualification = SurfaceQualificationClass::Beta;
        row.claimed_stable = false;
        row.reachable = false;
    }

    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn command_surface_coverage_is_required() {
    let mut packet = packet();
    packet
        .surface_parity_rows
        .retain(|row| row.surface_class != CommandSurfaceClass::DeepLink);

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::CommandSurfaceCoverageMissing));
}

#[test]
fn claimed_stable_surface_must_preserve_parity() {
    let mut packet = packet();
    packet.surface_parity_rows[0].obeys_kill_switch = false;

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::CommandParityBroken));
}

#[test]
fn unqualified_surface_must_narrow_below_stable() {
    let mut packet = packet();
    packet.surface_parity_rows[0].qualification = SurfaceQualificationClass::Beta;

    let violations = packet.validate();
    assert!(
        violations.contains(&RepoAiInstructionHardeningViolation::UnqualifiedSurfaceClaimsStable)
            || violations.contains(&RepoAiInstructionHardeningViolation::CommandParityBroken)
    );
}

#[test]
fn narrowed_surface_below_stable_validates() {
    let mut packet = packet();
    if let Some(row) = packet
        .surface_parity_rows
        .iter_mut()
        .find(|row| row.surface_class == CommandSurfaceClass::AutomationRecipe)
    {
        row.qualification = SurfaceQualificationClass::Beta;
        row.claimed_stable = false;
        row.reachable = false;
    }

    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn missing_evidence_id_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.evidence_id = String::new();

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::EvidenceExportRefsMissing));
}

#[test]
fn missing_rollback_lineage_is_rejected() {
    let mut packet = packet();
    packet.evidence_export.rollback_lineage_refs.clear();

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::RollbackLineageMissing));
}

#[test]
fn missing_source_contract_is_rejected() {
    let mut packet = packet();
    packet
        .source_contract_refs
        .retain(|reference| reference != HARDEN_REPO_AI_INSTRUCTIONS_KILL_SWITCH_CONTRACT_REF);

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::MissingSourceContracts));
}

#[test]
fn raw_material_is_rejected() {
    let mut packet = packet();
    packet.instruction_rows[0].user_visible_label =
        "signed via https://provider.example/v1".to_owned();

    assert!(packet
        .validate()
        .contains(&RepoAiInstructionHardeningViolation::RawMaterialInExport));
}

#[test]
fn checked_artifact_validates() {
    let packet = current_stable_harden_repo_ai_instructions_export()
        .expect("checked hardened repo instruction export validates");
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
#[ignore = "run manually to regenerate the checked artifact"]
fn emit_artifact() {
    let packet = packet();
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
    let dir = format!("{root}/artifacts/ai/m4/harden_repo_ai_instructions");
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
    let fixture_dir = format!("{root}/fixtures/ai/m4/harden_repo_ai_instructions");
    std::fs::create_dir_all(&fixture_dir).unwrap();
    std::fs::write(
        format!("{fixture_dir}/harden_repo_ai_instructions_packet.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .unwrap();
}
