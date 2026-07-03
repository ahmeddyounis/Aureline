use super::*;

fn packet() -> AutomationGovernance {
    current_automation_governance().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, AUTOMATION_GOVERNANCE_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, AUTOMATION_GOVERNANCE_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_proposals() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn path_is_stable() {
    assert_eq!(
        AUTOMATION_GOVERNANCE_PATH,
        "artifacts/deps/m5/automation-governance.json"
    );
}

#[test]
fn corpus_covers_every_required_state() {
    let packet = packet();
    assert_eq!(packet.corpus_coverage_gaps(), Vec::new());
}

#[test]
fn every_automation_surface_is_represented() {
    let packet = packet();
    let surfaces: BTreeSet<ProposalSource> = packet
        .proposals
        .iter()
        .map(|p| p.automation_surface)
        .collect();
    for surface in ProposalSource::ALL {
        assert!(
            surfaces.contains(&surface),
            "no proposal exercises surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn every_proposal_binds_to_the_frozen_matrix() {
    let packet = packet();
    let matrix = current_m5_package_state_matrix().expect("matrix loads");
    assert_eq!(packet.references_matrix_id, matrix.packet_id);
    assert!(packet.all_bind_matrix());
    for proposal in &packet.proposals {
        for label in &proposal.applicable_labels {
            assert!(
                matrix.state(*label).is_some(),
                "proposal {} surfaces unbound label {}",
                proposal.proposal_id,
                label.as_str()
            );
        }
    }
}

#[test]
fn reuses_the_reviewed_mutation_flows_contract() {
    let packet = packet();
    let flows = crate::reviewed_mutation_flows::current_reviewed_mutation_flows()
        .expect("reviewed flows load");
    assert_eq!(packet.references_reviewed_flows_id, flows.packet_id);
    // Every governed proposal reuses a real review sheet by id where the sheet
    // exists in the canonical corpus; sheets the AI/recipe corpus reaches that
    // the reviewed-flows corpus also ships must agree on flow class.
    for proposal in &packet.proposals {
        if let Some(sheet) = flows.sheet(&proposal.reviewed_sheet.sheet_ref) {
            assert_eq!(
                sheet.flow_class, proposal.reviewed_sheet.flow_class,
                "proposal {} disagrees with sheet {} on flow class",
                proposal.proposal_id, proposal.reviewed_sheet.sheet_ref
            );
        }
    }
}

#[test]
fn every_proposal_discloses_required() {
    let packet = packet();
    assert!(packet.all_disclose_required());
    for proposal in &packet.proposals {
        assert!(
            proposal.discloses_all_required(),
            "proposal {} omits a required disclosure",
            proposal.proposal_id
        );
    }
}

#[test]
fn capability_gaps_always_narrow_away_from_execution() {
    let packet = packet();
    assert!(packet.all_gaps_narrowed());
    for proposal in &packet.proposals {
        if proposal.intrinsic_unsafe() {
            assert!(
                proposal.execution_decision.is_safe_narrowing(),
                "intrinsically unsafe proposal {} proceeds to execution",
                proposal.proposal_id
            );
        }
    }
}

#[test]
fn unsupported_ecosystem_narrows_to_inspect_only() {
    let packet = packet();
    let proposal = packet
        .proposal("ag:recipe:other:legacy-narrow")
        .expect("legacy narrow");
    assert!(!proposal.capability.all_promised_met());
    assert_eq!(
        proposal.execution_decision,
        ExecutionDecision::NarrowToInspectOnly
    );
    assert_eq!(
        proposal.result_class,
        GovernedResultClass::NarrowedInspectOnly
    );
    assert!(proposal.commit_gate_blocked());
}

#[test]
fn degraded_auth_narrows_to_export_or_blocks() {
    let packet = packet();
    let export = packet
        .proposal("ag:recipe:cargo:export-only")
        .expect("export only");
    assert!(export.reviewed_sheet.auth_blocks());
    assert_eq!(
        export.execution_decision,
        ExecutionDecision::NarrowToExportOnly
    );
    assert_eq!(export.result_class, GovernedResultClass::HandedOff);

    let blocked = packet
        .proposal("ag:ai:pip:auth-blocked")
        .expect("auth blocked");
    assert!(blocked.reviewed_sheet.auth_blocks());
    assert_eq!(
        blocked.execution_decision,
        ExecutionDecision::BlockedNoSafePath
    );
    assert_eq!(blocked.result_class, GovernedResultClass::BlockedUnsafe);
}

#[test]
fn native_build_the_ai_surface_cannot_run_hands_off_to_cli() {
    let packet = packet();
    let proposal = packet
        .proposal("ag:ai:node:native-cli-handoff")
        .expect("cli handoff");
    assert_eq!(proposal.automation_surface, ProposalSource::AiProposal);
    assert!(!proposal.capability.provides_validation_execution);
    assert_eq!(proposal.execution_decision, ExecutionDecision::HandoffToCli);
    assert_eq!(proposal.result_class, GovernedResultClass::HandedOff);
}

#[test]
fn ai_proposals_pass_through_the_same_review_and_validation() {
    let packet = packet();
    let proposal = packet
        .proposal("ag:ai:cargo:serde-add")
        .expect("ai serde add");
    assert_eq!(proposal.automation_surface, ProposalSource::AiProposal);
    // An AI proposal cannot commit without selecting its required validation.
    assert!(proposal.validation.all_required_selected());
    assert!(proposal.validation.has_selected_task());
    assert!(proposal.parity.fully_preserves_contract());
    assert!(!proposal.commit_gate_blocked());
    assert_eq!(
        proposal.result_class,
        GovernedResultClass::CommittedReviewed
    );
}

#[test]
fn whole_workspace_proposal_requires_confirmation() {
    let packet = packet();
    let proposal = packet
        .proposal("ag:desktop:cargo:workspace-converge")
        .expect("workspace converge");
    assert!(proposal.reviewed_sheet.requires_scope_confirmation());
    assert!(proposal.reviewed_sheet.scope_confirmed);
    assert!(proposal.reviewed_sheet.scope_confirmation_satisfied());
    assert!(!proposal.commit_gate_blocked());
}

#[test]
fn rolled_back_proposal_keeps_a_durable_recovery_handle() {
    let packet = packet();
    let proposal = packet
        .proposal("ag:ai:node:react-rollback")
        .expect("react rollback");
    assert_eq!(proposal.result_class, GovernedResultClass::RolledBack);
    assert!(proposal.rollback_handle.is_durable_recovery());
    assert!(proposal.rollback_handle.offers_all_recovery_actions());
    // A rolled-back (committed) result must carry no live block reason.
    assert!(!proposal.commit_gate_blocked());
}

#[test]
fn the_same_proposal_feeds_every_surface_identically() {
    let packet = packet();
    let id = "ag:ai:cargo:serde-add";
    let desktop = packet
        .surface_projection(id, PackageSurface::DesktopPackageWorkspace)
        .expect("desktop");
    let cli = packet
        .surface_projection(id, PackageSurface::CliHeadless)
        .expect("cli");
    let ai = packet
        .surface_projection(id, PackageSurface::AiContext)
        .expect("ai");
    let support = packet
        .surface_projection(id, PackageSurface::SupportExport)
        .expect("support");

    // The result, decision, and rollback handle are identical across surfaces.
    for projection in [&cli, &ai, &support] {
        assert_eq!(projection.result_class, desktop.result_class);
        assert_eq!(projection.execution_decision, desktop.execution_decision);
        assert_eq!(projection.rollback_handle_ref, desktop.rollback_handle_ref);
    }

    // Only write authority differs: desktop and CLI may execute; AI and support
    // are inspect-only / redacted-export.
    assert!(desktop.can_execute_here);
    assert!(cli.can_execute_here);
    assert!(!ai.can_execute_here);
    assert!(!support.can_execute_here);
    assert!(support.redacted);
    assert!(!ai.redacted);
}

#[test]
fn automation_manifest_diff_card_preserves_validation_selection() {
    let packet = packet();
    let proposal = packet
        .proposal("ag:ai:cargo:serde-add")
        .expect("ai serde add");
    let card = proposal.manifest_diff_card();
    assert_eq!(
        card.validation_selection_ref.as_deref(),
        Some("ag:val:ai:serde-add")
    );
    assert!(card.selected_validation_tasks.contains(&"build".to_owned()));
    assert!(card.selected_validation_tasks.contains(&"test".to_owned()));
    assert!(card
        .selected_validation_tasks
        .contains(&"lockfile_verify".to_owned()));
    assert!(card.discloses_apply_boundary());
    assert!(card.fallback_honest());
    assert_eq!(card.proposal_source, ProposalSource::AiProposal);
    assert_eq!(card.apply_action, ManifestDiffApplyAction::Apply);
}

#[test]
fn automation_manifest_diff_card_is_honest_when_preview_or_checkpoint_is_missing() {
    let packet = packet();
    let no_preview = packet
        .proposal("ag:ai:other:unsupported-handoff")
        .expect("unsupported handoff")
        .manifest_diff_card();
    assert_eq!(
        no_preview.preview_state,
        ManifestDiffPreviewState::NoPreview
    );
    assert_eq!(
        no_preview.checkpoint_state,
        ManifestDiffCheckpointState::NarrowedNoCheckpoint
    );
    assert_eq!(
        no_preview.apply_action,
        ManifestDiffApplyAction::InspectOnly
    );
    assert!(no_preview.fallback_honest());

    let blocked = packet
        .proposal("ag:ai:pip:auth-blocked")
        .expect("auth blocked")
        .manifest_diff_card();
    assert_eq!(
        blocked.checkpoint_state,
        ManifestDiffCheckpointState::NarrowedNoCheckpoint
    );
    assert_ne!(blocked.apply_action, ManifestDiffApplyAction::Apply);
    assert!(blocked.discloses_apply_boundary());
    assert!(blocked.fallback_honest());
}

#[test]
fn blocked_proposal_never_executes_from_any_surface() {
    let packet = packet();
    let desktop = packet
        .surface_projection(
            "ag:ai:pip:auth-blocked",
            PackageSurface::DesktopPackageWorkspace,
        )
        .expect("desktop");
    assert!(!desktop.can_execute_here);
    let cli = packet
        .surface_projection("ag:ai:pip:auth-blocked", PackageSurface::CliHeadless)
        .expect("cli");
    assert!(!cli.can_execute_here);
}

#[test]
fn export_projection_is_redaction_safe() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.proposals.len());
    assert!(projection.blocks_any_commit);
    assert!(projection.all_gaps_narrowed);
    assert!(projection.all_disclose_required);
    assert!(projection.all_bind_matrix);
    for row in &projection.rows {
        assert!(!row.summary.contains("://"));
        assert!(!row.label.contains("://"));
    }
}

#[test]
fn no_field_leaks_a_raw_url() {
    let packet = packet();
    for proposal in &packet.proposals {
        assert!(!proposal
            .reviewed_sheet
            .redacted_manifest_path
            .contains("://"));
        assert!(!proposal.rollback_handle.checkpoint_ref.contains("://"));
    }
}

#[test]
fn validate_flags_unsafe_fallback_execution() {
    let mut packet = packet();
    let proposal = packet
        .proposals
        .iter_mut()
        .find(|p| p.proposal_id == "ag:recipe:other:legacy-narrow")
        .expect("legacy narrow");
    // Force an intrinsically unsafe proposal to proceed.
    proposal.execution_decision = ExecutionDecision::ProceedAfterReview;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::UnsafeFallbackExecution { .. }
    )));
}

#[test]
fn validate_flags_required_validation_unselected() {
    let mut packet = packet();
    let proposal = packet
        .proposals
        .iter_mut()
        .find(|p| p.proposal_id == "ag:ai:cargo:serde-add")
        .expect("serde add");
    // Deselect a required validation task on a proceeding proposal.
    proposal
        .validation
        .tasks
        .iter_mut()
        .find(|t| t.kind == ValidationTaskKind::Build)
        .expect("build task")
        .selected = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::RequiredValidationUnselected { .. }
    )));
}

#[test]
fn validate_flags_parity_contract_broken() {
    let mut packet = packet();
    let proposal = packet
        .proposals
        .iter_mut()
        .find(|p| p.proposal_id == "ag:ai:cargo:serde-add")
        .expect("serde add");
    proposal.parity.not_a_bypass_lane = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::ParityContractBroken { .. }
    )));
}

#[test]
fn validate_flags_hidden_scripting_allowed() {
    let mut packet = packet();
    packet.proposals[0].parity.no_hidden_scripting = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::HiddenScriptingAllowed { .. }
    )));
}

#[test]
fn validate_flags_silent_scope_broadening() {
    let mut packet = packet();
    let proposal = packet
        .proposals
        .iter_mut()
        .find(|p| p.proposal_id == "ag:desktop:cargo:workspace-converge")
        .expect("workspace converge");
    // A proceeding whole-workspace mutation with confirmation withdrawn.
    proposal.reviewed_sheet.scope_confirmed = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::SilentScopeBroadening { .. }
    )));
    // It is also an unsafe-fallback execution and a commit-gate violation.
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::UnsafeFallbackExecution { .. }
    )));
}

#[test]
fn validate_flags_rollback_handle_not_durable() {
    let mut packet = packet();
    let proposal = packet
        .proposals
        .iter_mut()
        .find(|p| p.proposal_id == "ag:ai:cargo:serde-add")
        .expect("serde add");
    proposal.rollback_handle.durable = false;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::RollbackHandleNotDurable { .. }
    )));
}

#[test]
fn validate_flags_result_decision_mismatch() {
    let mut packet = packet();
    let proposal = packet
        .proposals
        .iter_mut()
        .find(|p| p.proposal_id == "ag:recipe:other:legacy-narrow")
        .expect("legacy narrow");
    // A narrowing decision can never produce a committed result.
    proposal.result_class = GovernedResultClass::CommittedReviewed;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::ResultDecisionMismatch { .. }
    )));
}

#[test]
fn validate_flags_capability_ecosystem_mismatch() {
    let mut packet = packet();
    packet.proposals[0].capability.ecosystem = EcosystemKind::Other;
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::CapabilityEcosystemMismatch { .. }
    )));
}

#[test]
fn validate_flags_surface_parity_broken() {
    let mut packet = packet();
    packet.proposals[0].surface_parity.cli_headless = false;
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AutomationGovernanceViolation::SurfaceParityBroken { .. })));
}

#[test]
fn validate_flags_raw_url_leak() {
    let mut packet = packet();
    packet.proposals[0].reviewed_sheet.redacted_manifest_path =
        "https://secret.example.com/Cargo.toml".to_owned();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AutomationGovernanceViolation::RawUrlLeak { .. })));
}

#[test]
fn validate_flags_matrix_binding_mismatch() {
    let mut packet = packet();
    packet.references_matrix_id = "some-other-matrix:v9".to_owned();
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        AutomationGovernanceViolation::MatrixBindingMismatch { .. }
    )));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_proposals = packet.summary.total_proposals.wrapping_add(1);
    let violations = packet.validate();
    assert!(violations.contains(&AutomationGovernanceViolation::SummaryMismatch));
}

#[test]
fn validate_flags_duplicate_proposal_id() {
    let mut packet = packet();
    let clone = packet.proposals[0].clone();
    packet.proposals.push(clone);
    packet.summary = packet.computed_summary();
    let violations = packet.validate();
    assert!(violations
        .iter()
        .any(|v| matches!(v, AutomationGovernanceViolation::DuplicateRowId { .. })));
}

#[test]
fn tokens_are_stable() {
    assert_eq!(
        ExecutionDecision::ProceedAfterReview.as_str(),
        "proceed_after_review"
    );
    assert_eq!(
        ExecutionDecision::NarrowToInspectOnly.as_str(),
        "narrow_to_inspect_only"
    );
    assert_eq!(
        ExecutionDecision::HandoffToBrowser.as_str(),
        "handoff_to_browser"
    );
    assert_eq!(
        GovernedResultClass::NarrowedInspectOnly.as_str(),
        "narrowed_inspect_only"
    );
    assert_eq!(
        GovernedResultClass::CommittedReviewed.as_str(),
        "committed_reviewed"
    );
    assert_eq!(ValidationTaskKind::SecurityAudit.as_str(), "security_audit");
    assert_eq!(
        ValidationTaskKind::LockfileVerify.as_str(),
        "lockfile_verify"
    );
}

#[test]
fn every_vocabulary_round_trips_through_serde() {
    fn round_trip<T>(all: &[T])
    where
        T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        for value in all {
            let json = serde_json::to_string(value).expect("serialize");
            let back: T = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&back, value);
        }
    }
    round_trip(&ValidationTaskKind::ALL);
    round_trip(&ExecutionDecision::ALL);
    round_trip(&GovernedResultClass::ALL);
}

/// Scenario fixtures, embedded so they validate without a runtime walk.
const FIXTURE_AI_PROCEED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/ai-recipe-cli-package-mutation/ai_install_proceed_committed.json"
));
const FIXTURE_RECIPE_NARROW: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/ai-recipe-cli-package-mutation/recipe_capability_gap_inspect_only.json"
));
const FIXTURE_CLI_PREVIEW: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/ai-recipe-cli-package-mutation/cli_dry_run_preview_pending.json"
));
const FIXTURE_AI_BLOCKED: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/dependencies/m5/ai-recipe-cli-package-mutation/ai_auth_blocked_no_safe_path.json"
));

#[test]
fn fixtures_parse_and_validate() {
    for (name, json) in [
        ("ai_install_proceed_committed", FIXTURE_AI_PROCEED),
        ("recipe_capability_gap_inspect_only", FIXTURE_RECIPE_NARROW),
        ("cli_dry_run_preview_pending", FIXTURE_CLI_PREVIEW),
        ("ai_auth_blocked_no_safe_path", FIXTURE_AI_BLOCKED),
    ] {
        let packet: AutomationGovernance =
            serde_json::from_str(json).unwrap_or_else(|e| panic!("{name} parses: {e}"));
        assert_eq!(packet.validate(), Vec::new(), "{name} validates");
        assert!(packet.all_bind_matrix(), "{name} binds the matrix");
        assert!(packet.all_gaps_narrowed(), "{name} narrows every gap");
    }
}

#[test]
fn fixtures_cover_the_governance_guards() {
    let proceed: AutomationGovernance =
        serde_json::from_str(FIXTURE_AI_PROCEED).expect("ai proceed fixture");
    assert_eq!(
        proceed.proposals[0].automation_surface,
        ProposalSource::AiProposal
    );
    assert_eq!(
        proceed.proposals[0].execution_decision,
        ExecutionDecision::ProceedAfterReview
    );
    assert!(!proceed.proposals[0].commit_gate_blocked());

    let narrow: AutomationGovernance =
        serde_json::from_str(FIXTURE_RECIPE_NARROW).expect("recipe narrow fixture");
    assert!(!narrow.proposals[0].capability.all_promised_met());
    assert!(narrow.proposals[0].execution_decision.is_safe_narrowing());

    let blocked: AutomationGovernance =
        serde_json::from_str(FIXTURE_AI_BLOCKED).expect("ai blocked fixture");
    assert_eq!(
        blocked.proposals[0].execution_decision,
        ExecutionDecision::BlockedNoSafePath
    );
    assert!(blocked.proposals[0].commit_gate_blocked());
}
