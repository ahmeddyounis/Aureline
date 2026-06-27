//! Inline tests for the M5 runbook companion register.

use super::*;

use crate::m5_runbook_steps::seeded_m5_runbook_step_library;

fn canonical() -> M5RunbookCompanionRegister {
    seeded_m5_runbook_companion_register()
}

#[test]
fn canonical_register_validates() {
    let register = canonical();
    assert!(register.validate().is_empty(), "{:?}", register.validate());
    assert_eq!(register.register_id, M5_RUNBOOK_COMPANION_REGISTER_ID);
    assert_eq!(
        register.record_kind,
        M5_RUNBOOK_COMPANION_REGISTER_RECORD_KIND
    );
    assert!(!register.steps.is_empty());
    assert_eq!(register.surfaces.len(), register.steps.len());
    assert!(register.conformance.all_hold());
    assert!(register.vocabulary.matches_canonical());
}

#[test]
fn every_surface_can_follow_and_acknowledge_in_scope() {
    let register = canonical();
    for s in &register.surfaces {
        assert!(s.validate().is_empty(), "{}: {:?}", s.step_id, s.validate());
        assert!(s.offers(CompanionActionClass::Follow));
        assert!(s.offers(CompanionActionClass::Acknowledge));
        assert!(s.offers(CompanionActionClass::Comment));
        // Following / acknowledging never mutates beyond scope.
        assert!(!s.creates_hidden_mutate_channel);
    }
    assert!(
        register
            .conformance
            .follow_acknowledge_available_within_scope
    );
}

#[test]
fn read_only_steps_follow_in_scope_without_mutation() {
    let register = canonical();
    let inspect = register
        .surface("step:inspect-pipeline-state")
        .expect("inspect surface");
    assert_eq!(
        inspect.scope_disposition,
        CompanionScopeDisposition::FollowInScope
    );
    assert!(!inspect.offers(CompanionActionClass::ExecuteInScope));
    assert!(!inspect.offers(CompanionActionClass::GrantScopedApproval));
    assert!(!inspect.offers(CompanionActionClass::HandoffToDesktop));
    assert!(!inspect.privileged_mutate_blocked_on_companion);
    assert!(inspect.reused_approval_authority_ref.is_none());
    assert!(inspect.reused_action_envelope_ref.is_none());
}

#[test]
fn companion_may_act_within_scope_on_self_approve_mutation() {
    let register = canonical();
    let mitigate = register
        .surface("step:mitigate-restart-worker")
        .expect("mitigate surface");
    assert_eq!(
        mitigate.scope_disposition,
        CompanionScopeDisposition::ActInScope
    );
    assert!(mitigate.companion_may_execute);
    assert!(mitigate.companion_may_grant_approval);
    assert!(mitigate.offers(CompanionActionClass::ExecuteInScope));
    assert!(mitigate.offers(CompanionActionClass::GrantScopedApproval));
    assert!(!mitigate.privileged_mutate_blocked_on_companion);
    assert!(mitigate.desktop_handoff_message_id.is_none());
}

#[test]
fn companion_approval_reuses_the_same_desktop_objects() {
    let register = canonical();
    let library = seeded_m5_runbook_step_library();
    // Every companion-granted approval reuses the byte-identical desktop refs, so a
    // companion approval creates the same durable audit/approval object as desktop.
    for surface in &register.surfaces {
        if !surface.companion_may_grant_approval {
            assert!(
                surface.reused_approval_authority_ref.is_none(),
                "{} should not reuse an approval ref",
                surface.step_id
            );
            continue;
        }
        let step = library.step(&surface.step_id).expect("desktop step exists");
        assert_eq!(
            surface.reused_approval_authority_ref.as_deref(),
            Some(step.command_binding.approval_authority_ref.as_str()),
            "{} approval ref must match desktop",
            surface.step_id
        );
        assert_eq!(
            surface.reused_action_envelope_ref.as_deref(),
            Some(step.command_binding.action_envelope_ref.as_str()),
            "{} action envelope must match desktop",
            surface.step_id
        );
        assert!(!step.command_binding.approval_authority_ref.is_empty());
    }
    assert!(
        register
            .conformance
            .companion_approval_reuses_desktop_objects
    );
}

#[test]
fn blocked_privileged_actions_degrade_to_a_clear_desktop_handoff() {
    let register = canonical();
    // Privileged, human-approval, and out-of-plane steps are blocked on the companion
    // and degrade to an explicit desktop handoff — never a silent failure.
    for step_id in [
        "step:rollback-bad-deploy",
        "step:failover-region-privileged",
        "step:console-handoff-vendor-scaling",
        "step:approval-change-gate",
    ] {
        let s = register.surface(step_id).expect("surface exists");
        assert_eq!(
            s.scope_disposition,
            CompanionScopeDisposition::DesktopHandoffRequired,
            "{step_id} should require a desktop handoff"
        );
        assert!(s.privileged_mutate_blocked_on_companion, "{step_id}");
        assert!(s.blocks(CompanionActionClass::ExecuteInScope), "{step_id}");
        assert!(
            s.offers(CompanionActionClass::HandoffToDesktop),
            "{step_id}"
        );
        assert!(s.desktop_handoff_message_id.is_some(), "{step_id}");
        // A companion may still surface a request (a request, not a grant).
        assert!(s.offers(CompanionActionClass::RequestApproval), "{step_id}");
        assert!(
            !s.offers(CompanionActionClass::GrantScopedApproval),
            "{step_id} must not grant"
        );
        assert!(!s.companion_may_execute, "{step_id}");
        assert!(!s.creates_hidden_mutate_channel, "{step_id}");
    }
    assert!(
        register
            .conformance
            .blocked_actions_degrade_to_desktop_handoff
    );
}

#[test]
fn a_companion_can_never_mutate_beyond_scope() {
    let register = canonical();
    for s in &register.surfaces {
        if s.offers(CompanionActionClass::ExecuteInScope) {
            assert!(s.companion_may_execute, "{} over-executes", s.step_id);
        }
        if s.offers(CompanionActionClass::GrantScopedApproval) {
            assert!(s.companion_may_grant_approval, "{} over-grants", s.step_id);
        }
    }
    assert!(register.conformance.companion_never_mutates_beyond_scope);
}

#[test]
fn every_scope_disposition_is_represented() {
    let register = canonical();
    let dispositions: std::collections::BTreeSet<&str> = register
        .surfaces
        .iter()
        .map(|s| s.scope_disposition.as_str())
        .collect();
    for d in CompanionScopeDisposition::ALL {
        assert!(
            dispositions.contains(d.as_str()),
            "disposition {} not represented",
            d.as_str()
        );
    }
}

#[test]
fn surfaces_are_identical_across_surfaces() {
    let register = canonical();
    let app = register.surfaces_for(CompanionRunbookSurfaceKind::CompanionApp);
    let desktop = register.surfaces_for(CompanionRunbookSurfaceKind::DesktopHandoffTarget);
    let support = register.surfaces_for(CompanionRunbookSurfaceKind::SupportExport);
    assert_eq!(app, desktop);
    assert_eq!(app, support);
    assert_eq!(app, register.surfaces);
}

#[test]
fn surface_drift_is_caught() {
    let mut register = canonical();
    register.surfaces[0]
        .available_actions
        .push(CompanionActionClass::ExecuteInScope);
    assert!(register
        .validate()
        .contains(&M5RunbookCompanionViolation::SurfaceDrift));
}

#[test]
fn an_over_scope_execute_is_rejected() {
    let mut register = canonical();
    // Force a read-only surface to offer an in-product execute it does not permit.
    let idx = register
        .surfaces
        .iter()
        .position(|s| {
            matches!(
                s.scope_disposition,
                CompanionScopeDisposition::FollowInScope
            )
        })
        .expect("a follow-in-scope surface");
    register.surfaces[idx]
        .available_actions
        .push(CompanionActionClass::ExecuteInScope);
    let violations = register.surfaces[idx].validate();
    assert!(violations.contains(&M5RunbookCompanionViolation::ExecuteOfferedOutsideScope));
}

#[test]
fn a_blocked_action_without_handoff_is_rejected() {
    let mut register = canonical();
    let idx = register
        .surfaces
        .iter()
        .position(|s| s.privileged_mutate_blocked_on_companion)
        .expect("a blocked surface");
    register.surfaces[idx].desktop_handoff_message_id = None;
    register.surfaces[idx]
        .available_actions
        .retain(|a| *a != CompanionActionClass::HandoffToDesktop);
    let violations = register.surfaces[idx].validate();
    assert!(violations.contains(&M5RunbookCompanionViolation::BlockedActionMissingHandoff));
}

#[test]
fn duplicate_step_ids_are_rejected() {
    let mut register = canonical();
    let dup_step = register.steps[0].clone();
    register.steps.push(dup_step);
    register.surfaces = register
        .steps
        .iter()
        .map(CompanionRunbookSurface::derive)
        .collect();
    assert!(register
        .validate()
        .contains(&M5RunbookCompanionViolation::DuplicateStepId));
}

#[test]
fn round_trips_through_json() {
    let register = canonical();
    let json = register.export_safe_json();
    let parsed: M5RunbookCompanionRegister = serde_json::from_str(&json).expect("round-trips");
    assert_eq!(parsed, register);
    assert!(parsed.validate().is_empty());
}

#[test]
fn markdown_summary_names_surfaces_and_dispositions() {
    let summary = canonical().render_markdown_summary();
    assert!(summary.contains("Companion-scoped step surfaces"));
    assert!(summary.contains("Desktop-handoff-required"));
    assert!(summary.contains("mitigate-restart-worker"));
    assert!(summary.contains("follow"));
}

#[test]
fn export_carries_no_forbidden_boundary_material() {
    let json = canonical().export_safe_json();
    for needle in ["credential", "secret", "password", "bearer_token"] {
        assert!(!json.contains(needle), "export leaked {needle}");
    }
}
