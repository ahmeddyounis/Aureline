use super::*;

fn page() -> FinalizeManagedWorkspaceLifecycleTruthPage {
    seeded_finalize_managed_workspace_lifecycle_truth_page()
}

#[test]
fn seeded_page_seeds_zero_defects_and_qualifies_stable() {
    let page = page();
    assert_eq!(page.defects.len(), 0);
    assert!(validate_finalize_managed_workspace_lifecycle_truth_page(&page).is_ok());
    assert!(page.qualifies_stable());
    assert!(page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeManagedWorkspaceLifecycleQualificationClass::Stable.as_str()
    );
}

#[test]
fn seeded_page_passes_all_lifecycle_conditions() {
    let page = page();
    assert!(page.all_rows_have_descriptors());
    assert!(page.lifecycle_states_are_explicit());
    assert!(page.fallback_drills_passed());
    assert!(page.share_tokens_are_explicit());
    assert!(page.persistence_truth_is_explicit());
}

#[test]
fn seeded_page_rows_are_all_stable() {
    let page = page();
    assert!(!page.rows.is_empty());
    for row in &page.rows {
        assert_eq!(
            row.qualification_token,
            FinalizeManagedWorkspaceLifecycleQualificationClass::Stable.as_str(),
            "row '{}' must qualify stable; got '{}'",
            row.row_id,
            row.qualification_token
        );
        assert_eq!(
            row.narrow_reason_token,
            FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed.as_str(),
            "row '{}' must have not_narrowed reason",
            row.row_id
        );
    }
}

#[test]
fn seeded_page_summary_counts_match_rows() {
    let page = page();
    assert_eq!(page.summary.row_count, page.rows.len());
    assert_eq!(page.summary.stable_row_count, page.rows.len());
    assert_eq!(page.summary.beta_row_count, 0);
    assert_eq!(page.summary.preview_row_count, 0);
    assert_eq!(page.summary.withdrawn_row_count, 0);
    assert_eq!(page.summary.defect_count, 0);
}

#[test]
fn support_export_wraps_clean_page() {
    let page = page();
    let export = FinalizeManagedWorkspaceLifecycleTruthSupportExport::from_page(
        "policy:finalize-managed-workspace-lifecycle-truth-export:0001",
        "2026-06-03T00:00:00Z",
        page,
    );
    assert!(export.raw_private_material_excluded);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.defect_counts_by_narrow_reason.is_empty());
    assert_eq!(export.page.summary.withdrawn_row_count, 0);
}

#[test]
fn lifecycle_state_collapsed_narrows_to_beta() {
    let mut input = seeded_managed_workspace_lifecycle_input();
    input.vocabulary_consistent_across_surfaces = false;
    let page = FinalizeManagedWorkspaceLifecycleTruthPage::new(
        "policy:finalize-managed-workspace-lifecycle-truth:test",
        "test page",
        "2026-06-03T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeManagedWorkspaceLifecycleQualificationClass::Beta.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == FinalizeManagedWorkspaceLifecycleNarrowReasonClass::LifecycleStateCollapsed
    }));
}

#[test]
fn missing_suspend_resume_checkpoint_narrows_to_beta() {
    let mut input = seeded_managed_workspace_lifecycle_input();
    if let Some(row) = input.workspace_rows.first_mut() {
        row.suspend_resume_checkpoint = None;
    }
    let page = FinalizeManagedWorkspaceLifecycleTruthPage::new(
        "policy:finalize-managed-workspace-lifecycle-truth:test",
        "test page",
        "2026-06-03T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == FinalizeManagedWorkspaceLifecycleNarrowReasonClass::SuspendResumeCheckpointIncomplete
    }));
}

#[test]
fn unqualified_fallback_narrows_to_beta() {
    let mut input = seeded_managed_workspace_lifecycle_input();
    if let Some(row) = input.workspace_rows.first_mut() {
        row.fallback_qualified = false;
    }
    let page = FinalizeManagedWorkspaceLifecycleTruthPage::new(
        "policy:finalize-managed-workspace-lifecycle-truth:test",
        "test page",
        "2026-06-03T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason
            == FinalizeManagedWorkspaceLifecycleNarrowReasonClass::FallbackClaimedButUnqualified
    }));
}

#[test]
fn missing_outage_drill_narrows_to_beta() {
    let mut input = seeded_managed_workspace_lifecycle_input();
    if let Some(row) = input.workspace_rows.first_mut() {
        row.outage_drill_passed = false;
    }
    let page = FinalizeManagedWorkspaceLifecycleTruthPage::new(
        "policy:finalize-managed-workspace-lifecycle-truth:test",
        "test page",
        "2026-06-03T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == FinalizeManagedWorkspaceLifecycleNarrowReasonClass::OutageDrillNotPassed
    }));
}

#[test]
fn implicit_share_token_narrows_to_beta() {
    let mut input = seeded_managed_workspace_lifecycle_input();
    if let Some(row) = input.workspace_rows.first_mut() {
        if let Some(token) = row.share_handoff_tokens.first_mut() {
            token.join_mode_token.clear();
        }
    }
    let page = FinalizeManagedWorkspaceLifecycleTruthPage::new(
        "policy:finalize-managed-workspace-lifecycle-truth:test",
        "test page",
        "2026-06-03T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == FinalizeManagedWorkspaceLifecycleNarrowReasonClass::ShareTokenImplicit
    }));
}

#[test]
fn missing_descriptor_withdraws_packet() {
    let mut input = seeded_managed_workspace_lifecycle_input();
    input.all_rows_have_descriptors = false;
    if let Some(row) = input.workspace_rows.first_mut() {
        row.descriptor.workspace_id.clear();
    }
    let page = FinalizeManagedWorkspaceLifecycleTruthPage::new(
        "policy:finalize-managed-workspace-lifecycle-truth:test",
        "test page",
        "2026-06-03T00:00:00Z",
        input,
    );
    assert!(!page.qualifies_stable());
    assert!(!page.no_withdrawn_rows());
    assert_eq!(
        page.summary.overall_qualification_token,
        FinalizeManagedWorkspaceLifecycleQualificationClass::Withdrawn.as_str()
    );
    assert!(page.defects.iter().any(|d| {
        d.narrow_reason == FinalizeManagedWorkspaceLifecycleNarrowReasonClass::DescriptorMissing
    }));
}

#[test]
fn re_audit_matches_embedded_defects() {
    let page = page();
    let reaudited = audit_finalize_managed_workspace_lifecycle_truth_page(&page);
    assert_eq!(
        reaudited.len(),
        page.defects.len(),
        "re-audit must match embedded defects"
    );
}

#[test]
fn qualification_class_checks() {
    assert!(FinalizeManagedWorkspaceLifecycleQualificationClass::Stable.is_stable());
    assert!(!FinalizeManagedWorkspaceLifecycleQualificationClass::Beta.is_stable());
    assert!(!FinalizeManagedWorkspaceLifecycleQualificationClass::Withdrawn.is_stable());
    assert!(FinalizeManagedWorkspaceLifecycleQualificationClass::Stable.is_claimable());
    assert!(FinalizeManagedWorkspaceLifecycleQualificationClass::Beta.is_claimable());
    assert!(!FinalizeManagedWorkspaceLifecycleQualificationClass::Withdrawn.is_claimable());
    assert!(!FinalizeManagedWorkspaceLifecycleQualificationClass::Preview.is_claimable());
}

#[test]
fn narrow_reason_withdrawal_sentinel_check() {
    assert!(
        FinalizeManagedWorkspaceLifecycleNarrowReasonClass::DescriptorMissing
            .is_withdrawal_reason()
    );
    assert!(
        !FinalizeManagedWorkspaceLifecycleNarrowReasonClass::LifecycleStateCollapsed
            .is_withdrawal_reason()
    );
    assert!(
        !FinalizeManagedWorkspaceLifecycleNarrowReasonClass::NotNarrowed.is_withdrawal_reason()
    );
}

#[test]
fn provisioning_state_class_helpers() {
    assert!(ManagedProvisioningStateClass::Ready.admits_remote_mutation());
    assert!(!ManagedProvisioningStateClass::Suspended.admits_remote_mutation());
    assert!(!ManagedProvisioningStateClass::Queued.admits_remote_mutation());
    assert!(ManagedProvisioningStateClass::Queued.is_non_interactive());
    assert!(!ManagedProvisioningStateClass::Ready.is_non_interactive());
}

#[test]
fn join_mode_class_helpers() {
    assert!(ManagedJoinModeClass::SameLive.implies_live_continuity());
    assert!(!ManagedJoinModeClass::ResumeSnapshot.implies_live_continuity());
    assert!(!ManagedJoinModeClass::FreshReprovision.implies_live_continuity());
}

#[test]
fn fallback_path_class_helpers() {
    assert!(!ManagedFallbackPathClass::NotDeclared.has_fallback());
    assert!(ManagedFallbackPathClass::LocalWorkspace.has_fallback());
    assert!(ManagedFallbackPathClass::DirectSsh.has_fallback());
    assert!(ManagedFallbackPathClass::LocalAndDirectRemote.has_fallback());
}

#[test]
fn managed_workspace_descriptor_roundtrip() {
    let descriptor = ManagedWorkspaceDescriptor::new(
        "ws-test",
        "Test Workspace",
        "org-test",
        "ap-southeast-1",
        ManagedPersistenceClass::PolicyRetentionWithExpiry,
        "template:v3.0.0",
        "billing:enterprise",
        ManagedSecretModelClass::EphemeralPerSession,
        "suspend_after_15min_ttl_30d",
        "control_plane_reachable",
    );
    assert_eq!(descriptor.workspace_id, "ws-test");
    assert_eq!(
        descriptor.persistence_class_token,
        "policy_retention_with_expiry"
    );
    assert_eq!(descriptor.secret_model_token, "ephemeral_per_session");
}

#[test]
fn managed_rebuild_plan_fields() {
    let plan = ManagedRebuildPlan::new(
        ManagedDestructiveOperationClass::Delete,
        vec!["none".to_owned()],
        vec!["all".to_owned()],
        vec!["export_everything".to_owned()],
        vec!["policy:delete_requires_admin".to_owned()],
        "Delete stops billing immediately.",
        "Delete reprovisions nothing; all data is destroyed unless exported first.",
    );
    assert_eq!(plan.operation_token, "delete");
    assert_eq!(plan.preserved_state_classes, vec!["none"]);
    assert_eq!(plan.reprovisioned_state_classes, vec!["all"]);
}

#[test]
fn managed_share_handoff_token_fields() {
    let token = ManagedShareHandoffToken::new(
        "ws-gamma",
        "external-collaborator@example.com",
        "2026-07-01T00:00:00Z",
        "read-only-scope",
        ManagedJoinModeClass::FreshReprovision,
        "workspace-editor-read-only",
    );
    assert_eq!(token.join_mode_token, "fresh_reprovision");
    assert_eq!(token.target_workspace_id, "ws-gamma");
}

#[test]
fn managed_suspend_resume_checkpoint_fields() {
    let cp = ManagedSuspendResumeCheckpoint {
        retained_state_classes: vec!["files".to_owned()],
        drift_state_classes: vec!["ports".to_owned()],
        pinned_routes: vec!["http:8080".to_owned()],
        same_live_environment: true,
        resumed_snapshot: false,
        local_journals_outside_boundary: vec!["buffer_a".to_owned()],
        summary: "test checkpoint".to_owned(),
    };
    assert!(cp.same_live_environment);
    assert!(!cp.resumed_snapshot);
}
