use super::*;

const PACKET_ID: &str = "script-risk-grouped-update-rollback:stable:0001";

fn notice(
    notice_id: &str,
    package_label: &str,
    execution_source: ScriptExecutionSource,
    policy_blocks: bool,
    source_trusted: bool,
) -> ScriptRiskNotice {
    let disclosure = resolve_script_risk(execution_source, policy_blocks, source_trusted);
    ScriptRiskNotice {
        component: M5PackageComponent::ScriptRiskNotice,
        notice_id: notice_id.to_owned(),
        package_label: package_label.to_owned(),
        execution_source,
        policy_blocks,
        source_trusted,
        risk_class: disclosure.risk_class,
        execution_source_note: if disclosure.requires_execution_disclosure {
            "A post-install lifecycle step runs a build on the host toolchain".to_owned()
        } else {
            String::new()
        },
        review_action_label: if disclosure.needs_review_action {
            "Review scripts before applying".to_owned()
        } else {
            String::new()
        },
        policy_block_action_label: if disclosure.needs_policy_block_action {
            "Blocked by policy; request an exception".to_owned()
        } else {
            String::new()
        },
        support_note: "Support: scripts are disclosed and gated before any apply".to_owned(),
        client_note: "This package may run build steps on your machine".to_owned(),
        degradation_state: M5PackageComponentDegradationState::ResolvedExact,
        degradation_note: String::new(),
        rollback_posture: M5PackageComponentRollbackPosture::ReadOnlyNoMutation,
        fields_shown: vec![
            "package_label".to_owned(),
            "execution_source".to_owned(),
            "risk_class".to_owned(),
        ],
        source_contract_refs: vec![M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF.to_owned()],
    }
}

fn notices() -> Vec<ScriptRiskNotice> {
    vec![
        // No execution: nothing runs.
        notice(
            "notice:no-execution",
            "left-pad",
            ScriptExecutionSource::NoScriptsDeclared,
            false,
            true,
        ),
        // Review recommended: trusted install lifecycle script.
        notice(
            "notice:review",
            "esbuild",
            ScriptExecutionSource::InstallLifecycleScript,
            false,
            true,
        ),
        // Policy blocked: native build blocked by policy.
        notice(
            "notice:policy-blocked",
            "node-sass",
            ScriptExecutionSource::NativeBuildStep,
            true,
            false,
        ),
        // Unknown / untrusted: post-install binary fetch from an untrusted source.
        notice(
            "notice:untrusted",
            "unknown-pkg",
            ScriptExecutionSource::PostinstallBinaryFetch,
            false,
            false,
        ),
    ]
}

fn planner(
    planner_id: &str,
    plan_label: &str,
    update_reason: UpdateReason,
    grouped_packages: Vec<&str>,
    transitive_churn_count: u32,
) -> GroupedUpdatePlanner {
    let disclosure = resolve_update_plan_class(
        update_reason,
        grouped_packages.len() as u32,
        transitive_churn_count,
    );
    GroupedUpdatePlanner {
        component: M5PackageComponent::GroupedUpdatePlanner,
        planner_id: planner_id.to_owned(),
        plan_label: plan_label.to_owned(),
        update_reason,
        reason_note: "Reason recorded for this plan".to_owned(),
        grouped_packages: grouped_packages.iter().map(|p| (*p).to_owned()).collect(),
        transitive_churn_count,
        plan_class: disclosure.plan_class,
        transitive_churn_note: if disclosure.needs_transitive_churn_note {
            "This plan churns transitive dependencies; see the counts".to_owned()
        } else {
            String::new()
        },
        convergence_note: if disclosure.needs_convergence_note {
            "Broad convergence: many packages reconcile onto compatible versions".to_owned()
        } else {
            String::new()
        },
        security_note: if disclosure.needs_security_note {
            "Motivated by a security advisory; prioritize this update".to_owned()
        } else {
            String::new()
        },
        degradation_state: M5PackageComponentDegradationState::ResolvedExact,
        degradation_note: String::new(),
        rollback_posture: M5PackageComponentRollbackPosture::StagedReviewNoWrite,
        fields_shown: vec![
            "plan_label".to_owned(),
            "update_reason".to_owned(),
            "plan_class".to_owned(),
        ],
        source_contract_refs: vec![
            M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF.to_owned()
        ],
    }
}

fn planners() -> Vec<GroupedUpdatePlanner> {
    vec![
        // Direct bump: single package, little churn.
        planner(
            "planner:direct",
            "Bump lodash",
            UpdateReason::DirectRequest,
            vec!["lodash"],
            2,
        ),
        // Security patch.
        planner(
            "planner:security",
            "Patch minimist advisory",
            UpdateReason::SecurityAdvisory,
            vec!["minimist"],
            3,
        ),
        // Grouped refresh across several packages.
        planner(
            "planner:grouped",
            "Refresh eslint toolchain",
            UpdateReason::RoutineRefresh,
            vec!["eslint", "eslint-plugin-import", "eslint-config-airbnb"],
            10,
        ),
        // Broad convergence.
        planner(
            "planner:convergence",
            "Converge the dependency tree",
            UpdateReason::DependencyConvergence,
            vec!["react", "react-dom", "next", "webpack", "babel"],
            40,
        ),
    ]
}

fn strip(
    strip_id: &str,
    checkpoint_label: &str,
    remove_blocked_state: RemoveBlockedState,
    regenerated: bool,
) -> RollbackCheckpointStrip {
    let disclosure = resolve_recovery_posture(remove_blocked_state, regenerated);
    RollbackCheckpointStrip {
        component: M5PackageComponent::RollbackCheckpointStrip,
        strip_id: strip_id.to_owned(),
        checkpoint_label: checkpoint_label.to_owned(),
        checkpoint_id: "chk-0001".to_owned(),
        mutation_summary: "Applied grouped update to 3 packages".to_owned(),
        remove_blocked_state,
        remove_blocked_note: if disclosure.needs_remove_blocked_note {
            "This package cannot be cleanly removed; another dependency now requires it".to_owned()
        } else {
            String::new()
        },
        regenerated,
        regeneration_note: if disclosure.needs_regeneration_note {
            "Revert regenerates the lockfile rather than restoring manual edits".to_owned()
        } else {
            String::new()
        },
        recovery_posture_class: disclosure.recovery_posture,
        rollback_posture: disclosure.recovery_posture.expected_rollback_posture(),
        recovery_visible_now: true,
        revert_action_label: "Revert to checkpoint".to_owned(),
        open_diff_action_label: "Open diff".to_owned(),
        export_patch_action_label: "Export patch".to_owned(),
        degradation_state: M5PackageComponentDegradationState::ResolvedExact,
        degradation_note: String::new(),
        fields_shown: vec![
            "checkpoint_label".to_owned(),
            "recovery_posture_class".to_owned(),
            "remove_blocked_state".to_owned(),
        ],
        source_contract_refs: vec![
            M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF.to_owned()
        ],
    }
}

fn strips() -> Vec<RollbackCheckpointStrip> {
    vec![
        // Fully revertible.
        strip(
            "strip:full",
            "Checkpoint before install",
            RemoveBlockedState::Removable,
            false,
        ),
        // Revert with regeneration.
        strip(
            "strip:regenerate",
            "Checkpoint before grouped update",
            RemoveBlockedState::NotARemove,
            true,
        ),
        // Compensating only: a remove-blocked revert.
        strip(
            "strip:compensating",
            "Checkpoint before remove",
            RemoveBlockedState::RemoveBlockedRequiredBy,
            false,
        ),
    ]
}

fn trust_review() -> ScriptRiskGroupedUpdateRollbackTrustReview {
    ScriptRiskGroupedUpdateRollbackTrustReview {
        script_execution_source_always_explicit: true,
        script_risk_class_derived_not_asserted: true,
        policy_block_or_review_action_always_offered: true,
        update_reason_always_explicit: true,
        grouped_packages_always_listed: true,
        transitive_churn_never_understated: true,
        plan_class_distinguishes_bump_patch_grouped_broad: true,
        remove_blocked_states_explicit: true,
        recovery_posture_visible_after_mutation: true,
        revert_open_diff_export_patch_always_offered: true,
        no_generic_one_click_update_language: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn consumer_projection() -> ScriptRiskGroupedUpdateRollbackConsumerProjection {
    ScriptRiskGroupedUpdateRollbackConsumerProjection {
        script_risk_notice_shows_source_and_risk: true,
        grouped_update_planner_shows_reason_and_class: true,
        transitive_churn_shown_inline: true,
        rollback_strip_shows_recovery_posture: true,
        remove_blocked_states_shown_inline: true,
        cli_headless_shows_control_truth: true,
        support_export_shows_control_truth: true,
        help_about_shows_control_truth: true,
    }
}

fn proof_freshness() -> ScriptRiskGroupedUpdateRollbackProofFreshness {
    ScriptRiskGroupedUpdateRollbackProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-07-08T00:00:00Z".to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn downgrade_triggers() -> Vec<M5PackageComponentDowngradeTrigger> {
    vec![
        M5PackageComponentDowngradeTrigger::ProofStale,
        M5PackageComponentDowngradeTrigger::ScriptOrNativeBuildRisk,
        M5PackageComponentDowngradeTrigger::BroadLockfileRegeneration,
        M5PackageComponentDowngradeTrigger::RollbackUnavailable,
        M5PackageComponentDowngradeTrigger::PolicyBlocked,
        M5PackageComponentDowngradeTrigger::UpstreamDependencyNarrowed,
    ]
}

fn consumer_surfaces() -> Vec<M5PackageComponentConsumerSurface> {
    vec![
        M5PackageComponentConsumerSurface::InstallUpdateReview,
        M5PackageComponentConsumerSurface::RollbackRecovery,
        M5PackageComponentConsumerSurface::CliHeadless,
        M5PackageComponentConsumerSurface::SupportExport,
        M5PackageComponentConsumerSurface::HelpAbout,
    ]
}

fn source_contract_refs() -> Vec<String> {
    vec![
        SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_SCHEMA_REF.to_owned(),
        SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_SCHEMA_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_DOC_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_SCRIPT_RISK_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_GROUPED_UPDATE_CONTRACT_REF.to_owned(),
        M5_PACKAGE_COMPONENT_MATRIX_ROLLBACK_STRIP_CONTRACT_REF.to_owned(),
    ]
}

fn packet() -> ScriptRiskGroupedUpdateRollbackControlsPacket {
    ScriptRiskGroupedUpdateRollbackControlsPacket::new(
        ScriptRiskGroupedUpdateRollbackControlsPacketInput {
            packet_id: PACKET_ID.to_owned(),
            surface_label: "Script-risk, grouped-update, and rollback controls".to_owned(),
            script_risk_notices: notices(),
            grouped_update_planners: planners(),
            rollback_checkpoint_strips: strips(),
            downgrade_triggers: downgrade_triggers(),
            consumer_surfaces: consumer_surfaces(),
            trust_review: trust_review(),
            consumer_projection: consumer_projection(),
            proof_freshness: proof_freshness(),
            source_contract_refs: source_contract_refs(),
            redaction_class_token: "metadata_safe_default".to_owned(),
            minted_at: "2026-07-08T00:00:00Z".to_owned(),
        },
    )
}

#[test]
fn controls_packet_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn script_risk_resolver_derives_class() {
    assert_eq!(
        resolve_script_risk(ScriptExecutionSource::NoScriptsDeclared, false, true).risk_class,
        ScriptRiskClass::NoExecution
    );
    let review = resolve_script_risk(ScriptExecutionSource::InstallLifecycleScript, false, true);
    assert_eq!(review.risk_class, ScriptRiskClass::ReviewRecommended);
    assert!(review.needs_review_action);
    assert!(review.requires_execution_disclosure);
    let blocked = resolve_script_risk(ScriptExecutionSource::NativeBuildStep, true, false);
    assert_eq!(blocked.risk_class, ScriptRiskClass::PolicyBlocked);
    assert!(blocked.needs_policy_block_action);
    let untrusted =
        resolve_script_risk(ScriptExecutionSource::PostinstallBinaryFetch, false, false);
    assert_eq!(untrusted.risk_class, ScriptRiskClass::UnknownUntrusted);
    assert!(untrusted.needs_review_action);
}

#[test]
fn update_plan_resolver_distinguishes_classes() {
    assert_eq!(
        resolve_update_plan_class(UpdateReason::DirectRequest, 1, 2).plan_class,
        GroupedUpdatePlanClass::DirectBump
    );
    assert_eq!(
        resolve_update_plan_class(UpdateReason::SecurityAdvisory, 1, 3).plan_class,
        GroupedUpdatePlanClass::SecurityPatch
    );
    assert_eq!(
        resolve_update_plan_class(UpdateReason::RoutineRefresh, 3, 10).plan_class,
        GroupedUpdatePlanClass::GroupedRefresh
    );
    let broad = resolve_update_plan_class(UpdateReason::DependencyConvergence, 5, 40);
    assert_eq!(broad.plan_class, GroupedUpdatePlanClass::BroadConvergence);
    assert!(broad.needs_convergence_note);
    // A large grouped set alone forces broad convergence.
    assert_eq!(
        resolve_update_plan_class(UpdateReason::RoutineRefresh, 8, 1).plan_class,
        GroupedUpdatePlanClass::BroadConvergence
    );
}

#[test]
fn recovery_resolver_derives_posture() {
    assert_eq!(
        resolve_recovery_posture(RemoveBlockedState::Removable, false).recovery_posture,
        RecoveryPostureClass::FullyRevertible
    );
    assert_eq!(
        resolve_recovery_posture(RemoveBlockedState::NotARemove, true).recovery_posture,
        RecoveryPostureClass::RevertWithRegeneration
    );
    let blocked = resolve_recovery_posture(RemoveBlockedState::RemoveBlockedRequiredBy, false);
    assert_eq!(
        blocked.recovery_posture,
        RecoveryPostureClass::CompensatingOnly
    );
    assert!(blocked.needs_remove_blocked_note);
}

#[test]
fn recovery_class_implies_rollback_posture() {
    assert_eq!(
        RecoveryPostureClass::FullyRevertible.expected_rollback_posture(),
        M5PackageComponentRollbackPosture::WriteBackCheckpointed
    );
    assert_eq!(
        RecoveryPostureClass::RevertWithRegeneration.expected_rollback_posture(),
        M5PackageComponentRollbackPosture::RegenerateOnlyNoManualEdit
    );
    assert_eq!(
        RecoveryPostureClass::CompensatingOnly.expected_rollback_posture(),
        M5PackageComponentRollbackPosture::CompensatingOnlyNoCleanRevert
    );
}

#[test]
fn notice_misrepresenting_risk_fails() {
    let mut packet = packet();
    let idx = packet
        .script_risk_notices
        .iter()
        .position(|n| n.risk_class == ScriptRiskClass::UnknownUntrusted)
        .expect("untrusted notice present");
    packet.script_risk_notices[idx].risk_class = ScriptRiskClass::NoExecution;
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskClassMisrepresented));
}

#[test]
fn executing_notice_without_source_note_fails() {
    let mut packet = packet();
    let idx = packet
        .script_risk_notices
        .iter()
        .position(|n| n.execution_source.executes_code())
        .expect("executing notice present");
    packet.script_risk_notices[idx].execution_source_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ScriptExecutionSourceNoteMissing));
}

#[test]
fn policy_blocked_notice_without_action_fails() {
    let mut packet = packet();
    let idx = packet
        .script_risk_notices
        .iter()
        .position(|n| n.risk_class == ScriptRiskClass::PolicyBlocked)
        .expect("policy-blocked notice present");
    packet.script_risk_notices[idx].policy_block_action_label = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ScriptPolicyBlockActionMissing));
}

#[test]
fn review_notice_without_action_fails() {
    let mut packet = packet();
    let idx = packet
        .script_risk_notices
        .iter()
        .position(|n| n.risk_class == ScriptRiskClass::ReviewRecommended)
        .expect("review notice present");
    packet.script_risk_notices[idx].review_action_label = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ScriptReviewActionMissing));
}

#[test]
fn notice_hiding_support_client_note_fails() {
    let mut packet = packet();
    packet.script_risk_notices[0].client_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ScriptSupportClientNoteMissing));
}

#[test]
fn notice_wrong_component_class_fails() {
    let mut packet = packet();
    packet.script_risk_notices[0].component = M5PackageComponent::GroupedUpdatePlanner;
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskNoticeWrongComponentClass));
}

#[test]
fn notice_writing_posture_fails() {
    let mut packet = packet();
    packet.script_risk_notices[0].rollback_posture =
        M5PackageComponentRollbackPosture::WriteBackCheckpointed;
    assert!(packet.validate().contains(
        &ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskNoticeRollbackPostureInconsistent
    ));
}

#[test]
fn missing_risk_coverage_fails() {
    let mut packet = packet();
    packet
        .script_risk_notices
        .retain(|n| n.risk_class != ScriptRiskClass::PolicyBlocked);
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskCoverageMissing));
}

#[test]
fn empty_notices_fails() {
    let mut packet = packet();
    packet.script_risk_notices.clear();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ScriptRiskNoticesMissing));
}

#[test]
fn planner_misrepresenting_plan_class_fails() {
    let mut packet = packet();
    let idx = packet
        .grouped_update_planners
        .iter()
        .position(|p| p.plan_class == GroupedUpdatePlanClass::BroadConvergence)
        .expect("broad planner present");
    packet.grouped_update_planners[idx].plan_class = GroupedUpdatePlanClass::DirectBump;
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::PlanClassMisrepresented));
}

#[test]
fn planner_hiding_grouped_packages_fails() {
    let mut packet = packet();
    packet.grouped_update_planners[0].grouped_packages.clear();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::GroupedPackagesMissing));
}

#[test]
fn planner_hiding_reason_note_fails() {
    let mut packet = packet();
    packet.grouped_update_planners[0].reason_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::UpdateReasonNoteMissing));
}

#[test]
fn broad_planner_without_convergence_note_fails() {
    let mut packet = packet();
    let idx = packet
        .grouped_update_planners
        .iter()
        .position(|p| p.plan_class == GroupedUpdatePlanClass::BroadConvergence)
        .expect("broad planner present");
    packet.grouped_update_planners[idx].convergence_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ConvergenceNoteMissing));
}

#[test]
fn security_planner_without_security_note_fails() {
    let mut packet = packet();
    let idx = packet
        .grouped_update_planners
        .iter()
        .position(|p| p.plan_class == GroupedUpdatePlanClass::SecurityPatch)
        .expect("security planner present");
    packet.grouped_update_planners[idx].security_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::SecurityNoteMissing));
}

#[test]
fn planner_writing_posture_fails() {
    let mut packet = packet();
    packet.grouped_update_planners[0].rollback_posture =
        M5PackageComponentRollbackPosture::WriteBackCheckpointed;
    assert!(packet.validate().contains(
        &ScriptRiskGroupedUpdateRollbackViolation::GroupedUpdatePlannerRollbackPostureInconsistent
    ));
}

#[test]
fn missing_plan_class_coverage_fails() {
    let mut packet = packet();
    packet
        .grouped_update_planners
        .retain(|p| p.plan_class != GroupedUpdatePlanClass::GroupedRefresh);
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::PlanClassCoverageMissing));
}

#[test]
fn empty_planners_fails() {
    let mut packet = packet();
    packet.grouped_update_planners.clear();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::GroupedUpdatePlannersMissing));
}

#[test]
fn strip_misrepresenting_recovery_fails() {
    let mut packet = packet();
    let idx = packet
        .rollback_checkpoint_strips
        .iter()
        .position(|s| s.recovery_posture_class == RecoveryPostureClass::CompensatingOnly)
        .expect("compensating strip present");
    packet.rollback_checkpoint_strips[idx].recovery_posture_class =
        RecoveryPostureClass::FullyRevertible;
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::RecoveryPostureMisrepresented));
}

#[test]
fn remove_blocked_strip_without_note_fails() {
    let mut packet = packet();
    let idx = packet
        .rollback_checkpoint_strips
        .iter()
        .position(|s| s.remove_blocked_state.is_blocked())
        .expect("remove-blocked strip present");
    packet.rollback_checkpoint_strips[idx].remove_blocked_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::RemoveBlockedNoteMissing));
}

#[test]
fn regenerating_strip_without_note_fails() {
    let mut packet = packet();
    let idx = packet
        .rollback_checkpoint_strips
        .iter()
        .position(|s| s.regenerated)
        .expect("regenerating strip present");
    packet.rollback_checkpoint_strips[idx].regeneration_note = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::RegenerationNoteMissing));
}

#[test]
fn strip_hiding_checkpoint_identity_fails() {
    let mut packet = packet();
    packet.rollback_checkpoint_strips[0].checkpoint_id = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::CheckpointIdentityMissing));
}

#[test]
fn strip_hiding_actions_fails() {
    let mut packet = packet();
    packet.rollback_checkpoint_strips[0].export_patch_action_label = String::new();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::RollbackActionsMissing));
}

#[test]
fn strip_hiding_recovery_after_mutation_fails() {
    let mut packet = packet();
    packet.rollback_checkpoint_strips[0].recovery_visible_now = false;
    assert!(packet.validate().contains(
        &ScriptRiskGroupedUpdateRollbackViolation::RecoveryPostureNotVisibleAfterMutation
    ));
}

#[test]
fn strip_inconsistent_rollback_posture_fails() {
    let mut packet = packet();
    let idx = packet
        .rollback_checkpoint_strips
        .iter()
        .position(|s| s.recovery_posture_class == RecoveryPostureClass::CompensatingOnly)
        .expect("compensating strip present");
    packet.rollback_checkpoint_strips[idx].rollback_posture =
        M5PackageComponentRollbackPosture::WriteBackCheckpointed;
    assert!(packet.validate().contains(
        &ScriptRiskGroupedUpdateRollbackViolation::RollbackStripRollbackPostureInconsistent
    ));
}

#[test]
fn strip_wrong_component_class_fails() {
    let mut packet = packet();
    packet.rollback_checkpoint_strips[0].component = M5PackageComponent::ScriptRiskNotice;
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::RollbackStripWrongComponentClass));
}

#[test]
fn missing_recovery_posture_coverage_fails() {
    let mut packet = packet();
    packet
        .rollback_checkpoint_strips
        .retain(|s| s.recovery_posture_class != RecoveryPostureClass::RevertWithRegeneration);
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::RecoveryPostureCoverageMissing));
}

#[test]
fn missing_remove_blocked_coverage_fails() {
    let mut packet = packet();
    packet
        .rollback_checkpoint_strips
        .retain(|s| !s.remove_blocked_state.is_blocked());
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::RemoveBlockedCoverageMissing));
}

#[test]
fn empty_strips_fails() {
    let mut packet = packet();
    packet.rollback_checkpoint_strips.clear();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::RollbackStripsMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut packet = packet();
    packet.trust_review.no_generic_one_click_update_language = false;
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::TrustReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .rollback_strip_shows_recovery_posture = false;
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&ScriptRiskGroupedUpdateRollbackViolation::ProofFreshnessIncomplete));
}

#[test]
fn markdown_summary_lists_controls() {
    let summary = packet().render_markdown_summary();
    assert!(summary.contains("## Script-risk notices"));
    assert!(summary.contains("## Grouped-update planners"));
    assert!(summary.contains("## Rollback/checkpoint strips"));
    assert!(summary.contains("broad_convergence"));
    assert!(summary.contains("compensating_only"));
}

#[test]
fn checked_support_export_validates() {
    let packet = current_script_risk_grouped_update_rollback_export()
        .expect("checked script risk grouped update rollback export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-script-risk-grouped-update-rollback-controls/untrusted_script_broad_convergence.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-script-risk-grouped-update-rollback-controls/remove_blocked_recovery.json"
        )),
    ] {
        let packet: ScriptRiskGroupedUpdateRollbackControlsPacket =
            serde_json::from_str(raw).expect("fixture parses as controls packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

/// Regenerates the checked release artifacts and narrowed fixtures.
///
/// Guarded behind `GEN_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_ARTIFACTS` so ordinary
/// test runs never touch the working tree.
#[test]
fn generate_artifacts() {
    if std::env::var("GEN_SCRIPT_RISK_GROUPED_UPDATE_ROLLBACK_ARTIFACTS").is_err() {
        return;
    }

    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let repo_root = std::path::Path::new(manifest_dir).join("..").join("..");

    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());

    let proof_dir = repo_root
        .join("artifacts")
        .join("release")
        .join("m5-script-risk-grouped-update-rollback-proof");
    std::fs::create_dir_all(&proof_dir).expect("create proof dir");
    std::fs::write(
        proof_dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    std::fs::write(
        proof_dir.join("summary.md"),
        packet.render_markdown_summary(),
    )
    .expect("write summary");

    let fixture_dir = repo_root
        .join("fixtures")
        .join("ui")
        .join("m5-script-risk-grouped-update-rollback-controls");
    std::fs::create_dir_all(&fixture_dir).expect("create fixture dir");

    // Fixture 1: an untrusted post-install script paired with a broad convergence
    // plan — neither the risk nor the breadth can read as benign.
    let mut untrusted = packet.clone();
    untrusted.packet_id =
        "script-risk-grouped-update-rollback:fixture:untrusted-broad-convergence".to_owned();
    assert!(
        untrusted.validate().is_empty(),
        "{:?}",
        untrusted.validate()
    );
    std::fs::write(
        fixture_dir.join("untrusted_script_broad_convergence.json"),
        format!("{}\n", untrusted.export_safe_json()),
    )
    .expect("write untrusted fixture");

    // Fixture 2: a remove-blocked revert answered from an offline snapshot — the
    // recovery posture stays visible and cannot claim a clean rollback.
    let mut blocked = packet.clone();
    blocked.packet_id = "script-risk-grouped-update-rollback:fixture:remove-blocked".to_owned();
    for strip in blocked.rollback_checkpoint_strips.iter_mut() {
        if strip.remove_blocked_state.is_blocked() {
            strip.degradation_state = M5PackageComponentDegradationState::OfflineSnapshotOnly;
            strip.degradation_note =
                "Offline snapshot only; recovery reachability is estimated from the local cache"
                    .to_owned();
        }
    }
    assert!(blocked.validate().is_empty(), "{:?}", blocked.validate());
    std::fs::write(
        fixture_dir.join("remove_blocked_recovery.json"),
        format!("{}\n", blocked.export_safe_json()),
    )
    .expect("write remove-blocked fixture");
}
