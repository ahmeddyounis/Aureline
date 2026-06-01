use super::*;
use aureline_auth::approval_tickets::{
    audit_approval_ticket_beta_page, seeded_approval_ticket_beta_page,
    ApprovalTicketBetaProfileClass, SandboxProfileClass, UsePosture,
};

fn page() -> StabilizeApprovalTicketPage {
    seeded_stabilize_approval_ticket_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0, "expected zero defects; got {:?}", page.defects);
    assert!(validate_stabilize_approval_ticket_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        StabilizeApprovalTicketQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_seven_stability_conditions() {
    let page = page();
    assert!(page.covers_all_required_profiles());
    assert!(page.covers_all_sandbox_profile_classes());
    assert!(page.all_target_identities_are_present());
    assert!(page.remembered_approvals_have_fresh_ticket_evidence());
    assert!(page.credential_projection_sandbox_is_present());
}

#[test]
fn seeded_page_embeds_clean_approval_ticket_beta_page() {
    let page = page();
    let upstream_defects = audit_approval_ticket_beta_page(
        &page.approval_ticket_beta_page.sandbox_profile_rows,
        &page.approval_ticket_beta_page.capability_envelope_rows,
        &page.approval_ticket_beta_page.ticket_rows,
        &page.approval_ticket_beta_page.spend_attempt_events,
    );
    assert_eq!(upstream_defects.len(), 0);
    assert!(
        aureline_auth::approval_tickets::validate_approval_ticket_beta_page(
            &page.approval_ticket_beta_page
        )
        .is_ok()
    );
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert_eq!(page.rows.len(), ApprovalTicketBetaProfileClass::ALL.len());
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            StabilizeApprovalTicketQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.profile_token,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            StabilizeApprovalTicketNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason",
            row.profile_token
        );
    }
}

#[test]
fn seeded_page_rows_cover_all_four_required_profiles() {
    let page = page();
    let profile_tokens: Vec<&str> =
        page.rows.iter().map(|r| r.profile_token.as_str()).collect();
    for profile in ApprovalTicketBetaProfileClass::ALL {
        assert!(
            profile_tokens.contains(&profile.as_str()),
            "missing profile '{}' in rows",
            profile.as_str()
        );
    }
}

#[test]
fn seeded_page_summary_covers_all_four_sandbox_profile_classes() {
    let page = page();
    for sandbox_class in SandboxProfileClass::ALL {
        assert!(
            page.summary
                .sandbox_profiles_covered
                .contains(&sandbox_class.as_str().to_owned()),
            "sandbox profile class '{}' missing from summary",
            sandbox_class.as_str()
        );
    }
}

#[test]
fn seeded_page_summary_counts_match_rows() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
    assert_eq!(page.summary.beta_page_defect_count, 0);
}

#[test]
fn seeded_page_ticket_counts_match_rows() {
    let page = page();
    let total_per_row: usize = page.rows.iter().map(|r| r.ticket_row_count).sum();
    assert_eq!(
        total_per_row,
        page.approval_ticket_beta_page.ticket_rows.len()
    );
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = StabilizeApprovalTicketSupportExport::from_page(
        "policy:stabilize-export:approval-ticket:stable:0001",
        "2026-06-01T00:00:00Z",
        page,
    );
    assert!(export.raw_authority_material_excluded);
    assert!(export.authority_lineage_preserved);
    assert!(export.no_self_authorization_invariant);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn injected_raw_authority_withdraws_packet() {
    let mut beta_page = seeded_approval_ticket_beta_page();
    // Inject a raw-authority-material guardrail violation on the first sandbox
    // profile row.
    beta_page.sandbox_profile_rows[0].guardrails.raw_authority_material_present = true;
    beta_page.defects = audit_approval_ticket_beta_page(
        &beta_page.sandbox_profile_rows,
        &beta_page.capability_envelope_rows,
        &beta_page.ticket_rows,
        &beta_page.spend_attempt_events,
    );
    let page = StabilizeApprovalTicketPage::new(
        "policy:stabilize_approval_ticket_audit_target_identity:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert!(!page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        StabilizeApprovalTicketQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == StabilizeApprovalTicketNarrowReasonClass::RawAuthorityMaterialPresent
    }));
}

#[test]
fn injected_self_authorization_withdraws_packet() {
    let mut beta_page = seeded_approval_ticket_beta_page();
    // Inject self-authorization on the first sandbox profile row.
    beta_page.sandbox_profile_rows[0].guardrails.self_authorization_attempted = true;
    beta_page.defects = audit_approval_ticket_beta_page(
        &beta_page.sandbox_profile_rows,
        &beta_page.capability_envelope_rows,
        &beta_page.ticket_rows,
        &beta_page.spend_attempt_events,
    );
    let page = StabilizeApprovalTicketPage::new(
        "policy:stabilize_approval_ticket_audit_target_identity:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        StabilizeApprovalTicketQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == StabilizeApprovalTicketNarrowReasonClass::SelfAuthorizationAttempted
    }));
}

#[test]
fn missing_profile_coverage_narrows_to_preview() {
    let mut beta_page = seeded_approval_ticket_beta_page();
    // Remove all enterprise_managed tickets so the profile is uncovered.
    beta_page
        .ticket_rows
        .retain(|t| t.profile_token != "enterprise_managed");
    beta_page.defects = audit_approval_ticket_beta_page(
        &beta_page.sandbox_profile_rows,
        &beta_page.capability_envelope_rows,
        &beta_page.ticket_rows,
        &beta_page.spend_attempt_events,
    );
    let page = StabilizeApprovalTicketPage::new(
        "policy:stabilize_approval_ticket_audit_target_identity:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        StabilizeApprovalTicketQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == StabilizeApprovalTicketNarrowReasonClass::ProfileCoverageMissing
    }));
}

#[test]
fn missing_sandbox_class_coverage_narrows_to_preview() {
    let mut beta_page = seeded_approval_ticket_beta_page();
    // Remove the credential_projection_sandbox row.
    beta_page
        .sandbox_profile_rows
        .retain(|s| s.sandbox_profile_class != SandboxProfileClass::CredentialProjectionSandbox);
    beta_page.defects = audit_approval_ticket_beta_page(
        &beta_page.sandbox_profile_rows,
        &beta_page.capability_envelope_rows,
        &beta_page.ticket_rows,
        &beta_page.spend_attempt_events,
    );
    let page = StabilizeApprovalTicketPage::new(
        "policy:stabilize_approval_ticket_audit_target_identity:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        StabilizeApprovalTicketQualificationClass::Preview.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == StabilizeApprovalTicketNarrowReasonClass::SandboxProfileCoverageMissing
    }));
}

#[test]
fn empty_target_identity_narrows_to_beta() {
    let mut beta_page = seeded_approval_ticket_beta_page();
    // Clear the target_ref on the first ticket row.
    beta_page.ticket_rows[0].target_identity.target_ref.clear();
    beta_page.defects = audit_approval_ticket_beta_page(
        &beta_page.sandbox_profile_rows,
        &beta_page.capability_envelope_rows,
        &beta_page.ticket_rows,
        &beta_page.spend_attempt_events,
    );
    let page = StabilizeApprovalTicketPage::new(
        "policy:stabilize_approval_ticket_audit_target_identity:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        StabilizeApprovalTicketQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == StabilizeApprovalTicketNarrowReasonClass::TargetIdentityMissing
    }));
}

#[test]
fn remembered_approval_missing_evidence_narrows_to_beta() {
    let mut beta_page = seeded_approval_ticket_beta_page();
    // The enterprise_managed ticket uses BoundedReuse; clear its evidence refs.
    let bounded = beta_page
        .ticket_rows
        .iter_mut()
        .find(|t| t.use_posture == UsePosture::BoundedReuse);
    if let Some(ticket) = bounded {
        ticket.evidence_refs.clear();
    }
    beta_page.defects = audit_approval_ticket_beta_page(
        &beta_page.sandbox_profile_rows,
        &beta_page.capability_envelope_rows,
        &beta_page.ticket_rows,
        &beta_page.spend_attempt_events,
    );
    let page = StabilizeApprovalTicketPage::new(
        "policy:stabilize_approval_ticket_audit_target_identity:test",
        "test page",
        "2026-06-01T00:00:00Z",
        beta_page,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        StabilizeApprovalTicketQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == StabilizeApprovalTicketNarrowReasonClass::RememberedApprovalMissingFreshTicketEvidence
    }));
}

#[test]
fn qualification_class_checks() {
    assert!(StabilizeApprovalTicketQualificationClass::Stable.is_stable());
    assert!(!StabilizeApprovalTicketQualificationClass::Beta.is_stable());
    assert!(!StabilizeApprovalTicketQualificationClass::Withdrawn.is_stable());
    assert!(StabilizeApprovalTicketQualificationClass::Stable.is_claimable());
    assert!(StabilizeApprovalTicketQualificationClass::Beta.is_claimable());
    assert!(!StabilizeApprovalTicketQualificationClass::Withdrawn.is_claimable());
    assert!(!StabilizeApprovalTicketQualificationClass::Preview.is_claimable());
}

#[test]
fn narrow_reason_withdrawal_sentinel_checks() {
    assert!(
        StabilizeApprovalTicketNarrowReasonClass::RawAuthorityMaterialPresent
            .is_withdrawal_reason()
    );
    assert!(
        StabilizeApprovalTicketNarrowReasonClass::SelfAuthorizationAttempted
            .is_withdrawal_reason()
    );
    assert!(!StabilizeApprovalTicketNarrowReasonClass::BetaPageHasDefects.is_withdrawal_reason());
    assert!(!StabilizeApprovalTicketNarrowReasonClass::NotNarrowed.is_withdrawal_reason());
}

#[test]
fn narrow_reason_preview_sentinel_checks() {
    assert!(
        StabilizeApprovalTicketNarrowReasonClass::ProfileCoverageMissing.narrows_to_preview()
    );
    assert!(
        StabilizeApprovalTicketNarrowReasonClass::SandboxProfileCoverageMissing
            .narrows_to_preview()
    );
    assert!(
        !StabilizeApprovalTicketNarrowReasonClass::TargetIdentityMissing.narrows_to_preview()
    );
    assert!(
        !StabilizeApprovalTicketNarrowReasonClass::BetaPageHasDefects.narrows_to_preview()
    );
}

#[test]
fn audit_function_returns_same_defects_as_page() {
    let page = page();
    let re_audited = audit_stabilize_approval_ticket_page(&page);
    assert_eq!(re_audited.len(), page.defects.len());
}
