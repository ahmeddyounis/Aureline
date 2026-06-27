//! Inline tests for the M5 runbook executable step library.

use super::*;

fn canonical() -> M5RunbookStepLibrary {
    seeded_m5_runbook_step_library()
}

fn step_named(id: &str) -> RunbookExecutableStep {
    seeded_executable_steps()
        .into_iter()
        .find(|s| s.step_id == id)
        .unwrap()
}

#[test]
fn canonical_library_validates() {
    let library = canonical();
    assert!(library.validate().is_empty(), "{:?}", library.validate());
    assert_eq!(library.library_id, M5_RUNBOOK_STEP_LIBRARY_ID);
    assert_eq!(library.record_kind, M5_RUNBOOK_STEP_LIBRARY_RECORD_KIND);
}

#[test]
fn every_step_class_and_execution_mode_is_represented() {
    let library = canonical();
    let classes: std::collections::BTreeSet<RunbookStepClass> =
        library.steps.iter().map(|s| s.step_class).collect();
    for class in RunbookStepClass::ALL {
        assert!(classes.contains(&class), "class {} absent", class.as_str());
    }
    let modes: std::collections::BTreeSet<StepExecutionMode> =
        library.steps.iter().map(|s| s.execution_mode).collect();
    for mode in StepExecutionMode::ALL {
        assert!(modes.contains(&mode), "mode {} absent", mode.as_str());
    }
}

#[test]
fn preview_disposition_derives_from_mode_and_mutation() {
    let library = canonical();
    let disp = |id: &str| library.step(id).unwrap().preview_disposition();
    assert_eq!(
        disp("step:inspect-pipeline-state"),
        StepPreviewDisposition::ReadOnlyPreview
    );
    assert_eq!(
        disp("step:mitigate-restart-worker"),
        StepPreviewDisposition::DiffThenConfirm
    );
    assert_eq!(
        disp("step:console-handoff-vendor-scaling"),
        StepPreviewDisposition::HandoffPreview
    );
}

#[test]
fn approval_requirement_derives_from_scope() {
    let library = canonical();
    let read_only = library.step("step:inspect-pipeline-state").unwrap();
    assert!(!read_only.requires_approval());
    let self_approve = library.step("step:mitigate-restart-worker").unwrap();
    assert!(self_approve.requires_approval());
    assert!(!self_approve.requires_explicit_human_approval());
    let privileged = library.step("step:failover-region-privileged").unwrap();
    assert!(privileged.requires_explicit_human_approval());
}

#[test]
fn audit_evidence_is_declared_for_every_executable_step() {
    let library = canonical();
    for step in &library.steps {
        if step.mutating || step.execution_mode.is_in_product_executable() {
            assert!(
                !step.expected_evidence_outputs.is_empty(),
                "step {} declares no evidence",
                step.step_id
            );
            assert!(step.project().audit_expects_evidence);
        }
    }
}

#[test]
fn projection_recomputes_from_the_step_object() {
    let library = canonical();
    for (step, projection) in library.steps.iter().zip(&library.projections) {
        assert_eq!(&step.project(), projection);
        assert!(!projection.creates_hidden_mutate_channel);
    }
}

#[test]
fn projection_drift_is_caught() {
    let mut library = canonical();
    library.projections[0].requires_approval = !library.projections[0].requires_approval;
    assert!(library
        .validate()
        .contains(&M5RunbookStepViolation::ProjectionDrift));
}

#[test]
fn a_mutating_step_with_no_approval_is_a_hidden_mutate_channel() {
    let mut step = step_named("step:mitigate-restart-worker");
    step.approval_scope = RunbookApprovalScope::NoApprovalReadOnly;
    step.command_binding.approval_authority_ref = String::new();
    assert!(step.creates_hidden_mutate_channel());
    assert!(step
        .validate()
        .contains(&M5RunbookStepViolation::HiddenMutateChannel));
}

#[test]
fn a_companion_outside_scope_is_a_hidden_mutate_channel() {
    let mut step = step_named("step:rollback-bad-deploy");
    // Rollback requires human approval; a companion cannot be permitted on it.
    step.companion_permitted = true;
    assert!(step.creates_hidden_mutate_channel());
    assert!(step
        .validate()
        .contains(&M5RunbookStepViolation::HiddenMutateChannel));
}

#[test]
fn a_runbook_local_bypass_is_rejected() {
    let mut step = step_named("step:mitigate-restart-worker");
    step.command_binding.uses_runbook_local_bypass = true;
    let violations = step.validate();
    assert!(violations.contains(&M5RunbookStepViolation::RunbookLocalBypass));
    assert!(violations.contains(&M5RunbookStepViolation::HiddenMutateChannel));
}

#[test]
fn an_unbound_executable_step_is_rejected() {
    let mut step = step_named("step:mitigate-restart-worker");
    step.command_binding.binds_shared_envelope = false;
    let violations = step.validate();
    assert!(violations.contains(&M5RunbookStepViolation::EnvelopeBindingMissing));
    assert!(violations.contains(&M5RunbookStepViolation::HiddenMutateChannel));
}

#[test]
fn a_view_only_step_cannot_mutate_or_require_approval() {
    let mut step = step_named("step:inspect-pipeline-state");
    step.approval_scope = RunbookApprovalScope::RequiresHumanApproval;
    assert!(step
        .validate()
        .contains(&M5RunbookStepViolation::ViewOnlyStepIsActive));
}

#[test]
fn a_handoff_class_must_be_handoff_only_and_leave_the_plane() {
    let mut step = step_named("step:console-handoff-vendor-scaling");
    step.execution_mode = StepExecutionMode::InProductExecutable;
    let violations = step.validate();
    assert!(violations.contains(&M5RunbookStepViolation::HandoffModeMismatch));
    assert!(violations.contains(&M5RunbookStepViolation::ExecutionModeBoundaryMismatch));
}

#[test]
fn declaring_the_prohibited_scope_is_rejected() {
    let mut step = step_named("step:mitigate-restart-worker");
    step.approval_scope = RunbookApprovalScope::ProhibitedHiddenMutate;
    assert!(step
        .validate()
        .contains(&M5RunbookStepViolation::DeclaresProhibitedScope));
}

#[test]
fn an_approval_step_must_name_a_shared_authority() {
    let mut step = step_named("step:rollback-bad-deploy");
    step.command_binding.approval_authority_ref = String::new();
    assert!(step
        .validate()
        .contains(&M5RunbookStepViolation::ApprovalAuthorityMissing));
}

#[test]
fn a_read_only_step_must_not_name_an_approval_authority() {
    let mut step = step_named("step:inspect-pipeline-state");
    step.command_binding.approval_authority_ref = "approval_authority:spurious".to_owned();
    assert!(step
        .validate()
        .contains(&M5RunbookStepViolation::SpuriousApprovalAuthority));
}

#[test]
fn duplicate_step_ids_are_rejected() {
    let mut library = canonical();
    let dup = library.steps[0].clone();
    library.steps.push(dup);
    library.projections = library.steps.iter().map(|s| s.project()).collect();
    assert!(library
        .validate()
        .contains(&M5RunbookStepViolation::DuplicateStepId));
}

#[test]
fn the_same_projection_is_rendered_on_every_surface() {
    let library = canonical();
    assert!(library.surface_exposure.all_expose());
    let desktop = library.projections_for_surface(RunbookStepSurface::DesktopUi);
    let companion = library.projections_for_surface(RunbookStepSurface::CompanionFollow);
    let export = library.projections_for_surface(RunbookStepSurface::SupportExport);
    assert_eq!(desktop, companion);
    assert_eq!(desktop, export);
    assert_eq!(desktop, library.projections);
}

#[test]
fn conformance_review_holds_and_is_derived() {
    let library = canonical();
    assert!(library.conformance.all_hold());
    assert!(library.vocabulary.matches_canonical());
    let mut tampered = library.clone();
    tampered
        .conformance
        .no_step_mints_hidden_privileged_mutate_channel = false;
    assert!(tampered
        .validate()
        .contains(&M5RunbookStepViolation::ConformanceReviewFailed));
}

#[test]
fn companion_scoped_library_contains_only_companion_executable_steps() {
    let library = seeded_m5_runbook_step_library_companion_scoped();
    assert!(library.validate().is_empty(), "{:?}", library.validate());
    assert!(!library.steps.is_empty());
    for step in &library.steps {
        assert!(
            step.companion_may_execute(),
            "step {} is not companion-executable",
            step.step_id
        );
    }
    // The rollback and failover steps require human/privileged approval, so a
    // companion cannot drive them and they are absent from the follow view.
    assert!(library.step("step:rollback-bad-deploy").is_none());
    assert!(library.step("step:failover-region-privileged").is_none());
    assert!(library
        .step("step:console-handoff-vendor-scaling")
        .is_none());
}

#[test]
fn round_trips_through_json() {
    let library = canonical();
    let json = library.export_safe_json();
    let parsed: M5RunbookStepLibrary = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, library);
    assert!(parsed.validate().is_empty());
}

#[test]
fn markdown_summary_names_steps_and_modes() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("Governed executable steps"));
    assert!(summary.contains("step:mitigate-restart-worker"));
    assert!(summary.contains("handoff_only"));
    assert!(summary.contains("in_product_executable"));
}

#[test]
fn export_carries_no_forbidden_boundary_material() {
    let json = canonical().export_safe_json();
    for needle in ["credential", "secret", "password", "bearer_token"] {
        assert!(!json.contains(needle), "export leaked {needle}");
    }
}
