//! Canonical seed builders for the M5 runbook executable step library.
//!
//! These builders are the single producer of the checked-in step library, the
//! published inventory, the Markdown proof, the companion-scoped follow view, and
//! the per-step fixtures. The headless emitter and the inline tests both call them
//! so the in-code library, the artifacts, and the fixtures never drift. Every
//! library derives each step's preview/approval/audit projection and the
//! conformance review from the same declared step objects, so a step behaves
//! identically wherever it is previewed, executed, followed, or exported, and no
//! step can mint a hidden privileged mutate channel.

use super::*;

use crate::m5_runbook_governance::{
    ControlPlaneBoundaryClass, RunbookApprovalScope, RunbookStepClass,
};

/// Stable library id for the canonical step library.
pub const M5_RUNBOOK_STEP_LIBRARY_ID: &str = "m5-runbook-step-library:stable:0001";

/// Evaluation / mint timestamp for the canonical library.
const SEED_EVALUATED_AT: &str = "2026-07-06T00:00:00Z";

const REDACTION_CLASS: &str = "metadata_safe_default";

fn target(
    selector_ref: &str,
    breadth: TargetSelectorBreadth,
    crosses_environment: bool,
) -> TargetSelectorScope {
    TargetSelectorScope {
        selector_ref: selector_ref.to_owned(),
        breadth,
        crosses_environment,
    }
}

fn view_only_binding() -> CommandEnvelopeBinding {
    CommandEnvelopeBinding {
        action_envelope_ref: String::new(),
        approval_authority_ref: String::new(),
        binds_shared_envelope: false,
        uses_runbook_local_bypass: false,
    }
}

fn binding(action_envelope_ref: &str, approval_authority_ref: &str) -> CommandEnvelopeBinding {
    CommandEnvelopeBinding {
        action_envelope_ref: action_envelope_ref.to_owned(),
        approval_authority_ref: approval_authority_ref.to_owned(),
        binds_shared_envelope: true,
        uses_runbook_local_bypass: false,
    }
}

/// Builds one governed executable step.
#[allow(clippy::too_many_arguments)]
fn step(
    step_id: &str,
    label: &str,
    step_class: RunbookStepClass,
    target_selector: TargetSelectorScope,
    approval_scope: RunbookApprovalScope,
    execution_mode: StepExecutionMode,
    control_plane_boundary: ControlPlaneBoundaryClass,
    command_binding: CommandEnvelopeBinding,
    expected_evidence_outputs: &[&str],
    companion_permitted: bool,
) -> RunbookExecutableStep {
    RunbookExecutableStep {
        record_kind: M5_RUNBOOK_EXECUTABLE_STEP_RECORD_KIND.to_owned(),
        schema_version: M5_RUNBOOK_STEP_SCHEMA_VERSION,
        step_id: step_id.to_owned(),
        step_label: label.to_owned(),
        step_class,
        target_selector,
        approval_scope,
        execution_mode,
        control_plane_boundary,
        command_binding,
        mutating: step_class.is_mutating(),
        expected_evidence_outputs: expected_evidence_outputs
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        companion_permitted,
        detail_message_id: format!("{}step.{}", M5_RUNBOOK_STEP_MESSAGE_ID_PREFIX, step_id),
    }
}

/// Read-only inspection: view-only, no approval, companion may run it.
fn inspect_pipeline_state() -> RunbookExecutableStep {
    step(
        "step:inspect-pipeline-state",
        "Inspect pipeline worker state",
        RunbookStepClass::Inspect,
        target(
            "workspace:pipeline/worker-3",
            TargetSelectorBreadth::SingleTarget,
            false,
        ),
        RunbookApprovalScope::NoApprovalReadOnly,
        StepExecutionMode::ViewOnly,
        ControlPlaneBoundaryClass::InAppGoverned,
        view_only_binding(),
        &["pipeline_state_snapshot"],
        true,
    )
}

/// Read-only diagnosis across a scoped set: view-only, companion may run it.
fn diagnose_stalled_worker() -> RunbookExecutableStep {
    step(
        "step:diagnose-stalled-worker",
        "Diagnose the stalled worker set",
        RunbookStepClass::Diagnose,
        target(
            "workspace:pipeline/workers",
            TargetSelectorBreadth::ScopedSet,
            false,
        ),
        RunbookApprovalScope::NoApprovalReadOnly,
        StepExecutionMode::ViewOnly,
        ControlPlaneBoundaryClass::InAppGoverned,
        view_only_binding(),
        &["diagnosis_note"],
        true,
    )
}

/// Scoped mutating mitigation a companion may self-approve within bounds.
fn mitigate_restart_worker() -> RunbookExecutableStep {
    step(
        "step:mitigate-restart-worker",
        "Restart the stalled pipeline worker",
        RunbookStepClass::Mitigate,
        target(
            "workspace:pipeline/worker-3",
            TargetSelectorBreadth::SingleTarget,
            false,
        ),
        RunbookApprovalScope::ScopedSelfApprove,
        StepExecutionMode::InProductExecutable,
        ControlPlaneBoundaryClass::InAppGoverned,
        binding(
            "action_envelope:ops.worker_restart",
            "approval_authority:scoped_self_approve",
        ),
        &["restart_action_record", "post_restart_state"],
        true,
    )
}

/// Mutating rollback gated behind explicit human approval; companion may request.
fn rollback_bad_deploy() -> RunbookExecutableStep {
    step(
        "step:rollback-bad-deploy",
        "Roll back the bad deployment",
        RunbookStepClass::Rollback,
        target(
            "workspace:deploy/release-channel",
            TargetSelectorBreadth::ScopedSet,
            false,
        ),
        RunbookApprovalScope::RequiresHumanApproval,
        StepExecutionMode::InProductExecutable,
        ControlPlaneBoundaryClass::InAppGoverned,
        binding(
            "action_envelope:deploy.rollback",
            "approval_authority:human_change_gate",
        ),
        &["rollback_action_record", "rollback_verification_snapshot"],
        false,
    )
}

/// Environment-wide privileged mitigation gated behind privileged approval.
fn failover_region_privileged() -> RunbookExecutableStep {
    step(
        "step:failover-region-privileged",
        "Fail over the affected region",
        RunbookStepClass::Mitigate,
        target(
            "environment:region-west",
            TargetSelectorBreadth::EnvironmentWide,
            true,
        ),
        RunbookApprovalScope::RequiresPrivilegedApproval,
        StepExecutionMode::InProductExecutable,
        ControlPlaneBoundaryClass::InAppGoverned,
        binding(
            "action_envelope:dr.region_failover",
            "approval_authority:privileged_change_gate",
        ),
        &[
            "failover_action_record",
            "traffic_shift_snapshot",
            "failover_verification",
        ],
        false,
    )
}

/// A handoff-only pivot to an external vendor console; attributable, never in-product.
fn console_handoff_vendor_scaling() -> RunbookExecutableStep {
    step(
        "step:console-handoff-vendor-scaling",
        "Hand off to the vendor console to adjust scaling",
        RunbookStepClass::ConsoleHandoff,
        target(
            "vendor-console:scaling-group",
            TargetSelectorBreadth::ExternalTarget,
            true,
        ),
        RunbookApprovalScope::RequiresHumanApproval,
        StepExecutionMode::HandoffOnly,
        ControlPlaneBoundaryClass::VendorConsoleHandoff,
        binding(
            "action_envelope:handoff.vendor_console",
            "approval_authority:human_change_gate",
        ),
        &["console_handoff_attribution_record"],
        false,
    )
}

/// A communication annotation: view-only, untargeted, companion may run it.
fn annotate_comms_update() -> RunbookExecutableStep {
    step(
        "step:annotate-comms-update",
        "Post an incident communications update",
        RunbookStepClass::Annotate,
        target(
            "incident:comms-thread",
            TargetSelectorBreadth::NoTarget,
            false,
        ),
        RunbookApprovalScope::NoApprovalReadOnly,
        StepExecutionMode::ViewOnly,
        ControlPlaneBoundaryClass::InAppGoverned,
        view_only_binding(),
        &["incident_comms_note"],
        true,
    )
}

/// An explicit human approval gate routed through the shared approval system.
fn approval_change_gate() -> RunbookExecutableStep {
    step(
        "step:approval-change-gate",
        "Record the change-approval decision",
        RunbookStepClass::Approval,
        target(
            "incident:change-gate",
            TargetSelectorBreadth::NoTarget,
            false,
        ),
        RunbookApprovalScope::RequiresHumanApproval,
        StepExecutionMode::InProductExecutable,
        ControlPlaneBoundaryClass::InAppGoverned,
        binding(
            "action_envelope:approval.change_gate",
            "approval_authority:human_change_gate",
        ),
        &["approval_decision_record"],
        false,
    )
}

/// The checked-in governed executable steps demonstrating every step class and
/// every execution mode.
pub fn seeded_executable_steps() -> Vec<RunbookExecutableStep> {
    vec![
        inspect_pipeline_state(),
        diagnose_stalled_worker(),
        mitigate_restart_worker(),
        rollback_bad_deploy(),
        failover_region_privileged(),
        console_handoff_vendor_scaling(),
        annotate_comms_update(),
        approval_change_gate(),
    ]
}

fn assemble(
    library_id: &str,
    report_label: &str,
    steps: Vec<RunbookExecutableStep>,
) -> M5RunbookStepLibrary {
    M5RunbookStepLibrary::new(M5RunbookStepLibraryInput {
        library_id: library_id.to_owned(),
        report_label: report_label.to_owned(),
        evaluated_at: SEED_EVALUATED_AT.to_owned(),
        steps,
        redaction_class_token: REDACTION_CLASS.to_owned(),
        minted_at: SEED_EVALUATED_AT.to_owned(),
    })
}

/// The canonical runbook executable step library: every step class, every
/// execution mode, all bound to the shared command/action-envelope and approval
/// systems.
pub fn seeded_m5_runbook_step_library() -> M5RunbookStepLibrary {
    assemble(
        M5_RUNBOOK_STEP_LIBRARY_ID,
        "M5 runbook executable step library",
        seeded_executable_steps(),
    )
}

/// The companion follow view: the subset of steps a companion may *execute* itself
/// within declared scope (read-only inspection/diagnosis/annotation plus the
/// self-approve mitigation). The same typed step objects compose into a scoped
/// follow view without any per-step rewiring.
pub fn seeded_m5_runbook_step_library_companion_scoped() -> M5RunbookStepLibrary {
    let steps = seeded_executable_steps()
        .into_iter()
        .filter(|s| s.companion_may_execute())
        .collect();
    assemble(
        "m5-runbook-step-library:companion-scoped:0001",
        "M5 runbook executable step library — companion follow view",
        steps,
    )
}
