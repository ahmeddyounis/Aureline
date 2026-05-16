//! Fixture-driven coverage for the approval-ticket beta projection.
//!
//! These tests parse the protected fixtures under
//! `/fixtures/security/m3/approval_ticket`, validate the seeded page, and
//! prove that every drill fixture surfaces the expected typed defect kind.
//! They also confirm that the support-export wrapper preserves authority
//! lineage and the no-self-authorization invariant.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    audit_approval_ticket_beta_page, validate_approval_ticket_beta_page,
    ApprovalTicketBetaDefectKind, ApprovalTicketBetaPage, ApprovalTicketBetaProfileClass,
    ApprovalTicketBetaSupportExport, EvaluationOutcome, NativeReapprovalRoute,
    RequestOriginClass, SandboxProfileClass,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/security/m3/approval_ticket")
}

fn load_page(file_name: &str) -> ApprovalTicketBetaPage {
    let path = fixture_dir().join(file_name);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a page: {err}"))
}

#[test]
fn seeded_page_fixture_validates_with_zero_defects() {
    let page = load_page("page.json");
    validate_approval_ticket_beta_page(&page).expect("seeded page validates");
    assert!(page.defects.is_empty());
    for profile in ApprovalTicketBetaProfileClass::ALL {
        assert!(page
            .summary
            .profiles_present
            .iter()
            .any(|token| token == profile.as_str()));
    }
    for sandbox_class in SandboxProfileClass::ALL {
        assert!(page
            .summary
            .sandbox_profile_classes_present
            .iter()
            .any(|token| token == sandbox_class.as_str()));
    }
}

#[test]
fn seeded_page_covers_admitted_and_typed_denial_outcomes() {
    let page = load_page("page.json");
    for required in [
        "admitted",
        "denied_missing_authority",
        "denied_expired",
        "denied_target_drift",
        "denied_sandbox_profile_drift",
        "denied_policy_epoch_drift",
        "denied_capability_envelope_drift",
        "denied_self_authorization_attempted",
    ] {
        assert!(
            page.summary.spend_attempts_by_outcome.contains_key(required),
            "fixture must cover spend outcome {required}",
        );
    }
}

#[test]
fn seeded_tickets_cite_runtime_approval_ticket_refs() {
    let page = load_page("page.json");
    for ticket in &page.ticket_rows {
        assert!(
            !ticket.runtime_approval_ticket_ref.is_empty(),
            "ticket {} must cite a runtime approval-ticket ref",
            ticket.approval_ticket_id,
        );
        if !ticket.request_origin_class.is_intrinsic_issuer() {
            assert!(
                ticket.requesting_surface_ref.is_some(),
                "ticket {} ({:?}) must carry a requesting_surface_ref",
                ticket.approval_ticket_id,
                ticket.request_origin_class,
            );
        }
        let lifetime = ticket.lifetime_seconds;
        assert!(
            lifetime <= 3600,
            "ticket {} lifetime {lifetime}s exceeds reviewable cap",
            ticket.approval_ticket_id,
        );
        assert!(!ticket.guardrails.raw_authority_material_present);
        assert!(!ticket.guardrails.self_authorization_attempted);
        assert!(!ticket.guardrails.silent_widening_attempted);
        assert!(!ticket.guardrails.plaintext_secret_present);
        assert!(!ticket.guardrails.public_endpoint_fallback_offered);
        assert!(ticket.guardrails.local_editing_preserved);
    }
}

#[test]
fn seeded_spend_events_route_denials_to_typed_reapproval() {
    let page = load_page("page.json");
    for event in &page.spend_attempt_events {
        if event.evaluation_outcome.is_admitted() {
            assert!(matches!(
                event.native_reapproval_route,
                NativeReapprovalRoute::NotRequired
            ));
            continue;
        }
        assert!(
            !matches!(
                event.native_reapproval_route,
                NativeReapprovalRoute::NotRequired
            ),
            "denied spend {} must declare a typed reapproval route",
            event.spend_attempt_id,
        );
        assert!(
            !event.audit_event_refs.is_empty(),
            "denied spend {} must cite at least one audit_event_ref",
            event.spend_attempt_id,
        );
    }
}

#[test]
fn drill_raw_authority_material_surfaces_typed_defect() {
    let page = load_page("drill_raw_authority_material.json");
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == ApprovalTicketBetaDefectKind::RawAuthorityMaterialPresent));
}

#[test]
fn drill_self_authorization_attempted_surfaces_typed_defect() {
    let page = load_page("drill_self_authorization_attempted.json");
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == ApprovalTicketBetaDefectKind::SelfAuthorizationAttempted));
}

#[test]
fn drill_admitted_under_drift_surfaces_typed_defect() {
    let page = load_page("drill_admitted_under_drift.json");
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == ApprovalTicketBetaDefectKind::SpendAdmittedUnderDrift));
}

#[test]
fn drill_denial_missing_audit_ref_surfaces_typed_defect() {
    let page = load_page("drill_denial_missing_audit_ref.json");
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == ApprovalTicketBetaDefectKind::SpendDenialMissingAuditRef));
}

#[test]
fn drill_ticket_lifetime_exceeds_sandbox_budget_surfaces_typed_defect() {
    let page = load_page("drill_ticket_lifetime_exceeds_sandbox_budget.json");
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == ApprovalTicketBetaDefectKind::TicketLifetimeExceedsSandboxBudget));
}

#[test]
fn drill_envelope_capability_outside_sandbox_surfaces_typed_defect() {
    let page = load_page("drill_envelope_capability_outside_sandbox.json");
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == ApprovalTicketBetaDefectKind::EnvelopeCapabilityOutsideSandbox));
}

#[test]
fn drill_missing_requesting_surface_ref_surfaces_typed_defect() {
    let page = load_page("drill_missing_requesting_surface_ref.json");
    assert!(page.defects.iter().any(|defect| defect.defect_kind
        == ApprovalTicketBetaDefectKind::MissingRequestingSurfaceRef));
}

#[test]
fn support_export_round_trip_preserves_authority_lineage_and_invariant() {
    let page = load_page("page.json");
    let export = ApprovalTicketBetaSupportExport::from_page(
        "approval-ticket-beta:support-export:fixture-001",
        "2026-05-16T05:00:00Z",
        page,
    );
    assert!(export.raw_authority_material_excluded);
    assert!(export.authority_lineage_preserved);
    assert!(export.no_self_authorization_invariant);
    assert!(export.defect_kinds_present.is_empty());
    // The export must keep ticket and audit lineage referenceable: every
    // spend event must still resolve to its ticket on the embedded page.
    let ticket_ids: std::collections::BTreeSet<&str> = export
        .page
        .ticket_rows
        .iter()
        .map(|row| row.approval_ticket_id.as_str())
        .collect();
    for event in &export.page.spend_attempt_events {
        if matches!(
            event.evaluation_outcome,
            EvaluationOutcome::DeniedMissingAuthority
        ) {
            continue;
        }
        // The seeded self-authorization event presents a ticket minted under a
        // different request origin; ensure the export still tracks the
        // presented ticket.
        if matches!(
            event.evaluation_outcome,
            EvaluationOutcome::DeniedSelfAuthorizationAttempted
        ) {
            assert!(event.presented_approval_ticket_ref.is_some());
            continue;
        }
        let presented = event
            .presented_approval_ticket_ref
            .as_deref()
            .expect("typed spend event must cite a presented ticket ref");
        assert!(
            ticket_ids.contains(presented),
            "presented ticket {presented} must remain in the exported page",
        );
    }
}

#[test]
fn fixture_audit_matches_validator_recompute() {
    let page = load_page("page.json");
    let recomputed = audit_approval_ticket_beta_page(
        &page.sandbox_profile_rows,
        &page.capability_envelope_rows,
        &page.ticket_rows,
        &page.spend_attempt_events,
    );
    assert!(recomputed.is_empty(), "fixture must hold zero defects");
}

#[test]
fn intrinsic_issuer_surfaces_are_consistent_with_request_origin_table() {
    for origin in [
        RequestOriginClass::UserShellPrompt,
        RequestOriginClass::PolicyDecision,
        RequestOriginClass::SupervisorControlPath,
    ] {
        assert!(origin.is_intrinsic_issuer());
    }
    for origin in [
        RequestOriginClass::AiToolPlan,
        RequestOriginClass::ExtensionRequest,
        RequestOriginClass::CliScriptRequest,
        RequestOriginClass::BrowserCompanionRequest,
        RequestOriginClass::RemoteHelperRequest,
        RequestOriginClass::AutomationSchedulerRequest,
    ] {
        assert!(!origin.is_intrinsic_issuer());
    }
}
