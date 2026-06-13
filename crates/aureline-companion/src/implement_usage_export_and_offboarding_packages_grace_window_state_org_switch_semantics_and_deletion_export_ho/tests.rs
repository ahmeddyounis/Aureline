use super::*;

const PACKET_ID: &str = "usage-export-offboarding-surface:stable:0001";
const PACKET_LABEL: &str =
    "Usage-Export and Offboarding Packages, Grace-Window State, Org-Switch Semantics, and Deletion/Export Honesty";
const MINTED_AT: &str = "2026-06-09T00:00:00Z";

fn proof_freshness() -> OffboardingProofFreshness {
    OffboardingProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: MINTED_AT.to_owned(),
        auto_narrow_on_stale: true,
    }
}

fn packet() -> UsageExportOffboardingSurfacePacket {
    canonical_usage_export_offboarding_surface(
        PACKET_ID.to_owned(),
        PACKET_LABEL.to_owned(),
        MINTED_AT.to_owned(),
        proof_freshness(),
    )
}

#[test]
fn canonical_surface_validates() {
    let packet = packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn canonical_surface_covers_every_section_with_offboarding_lane() {
    let packet = packet();
    assert_eq!(
        packet.section_qualifications.len(),
        OffboardingSection::ALL.len()
    );
    for section in OffboardingSection::ALL {
        let row = packet
            .section_qualifications
            .iter()
            .find(|row| row.section == section)
            .expect("section present");
        assert_eq!(row.matrix_lane_ref, section.matrix_lane().as_str());
        assert_eq!(row.read_write_scope, section.bounded_scope());
        // Every section inherits the offboarding_continuity lane.
        assert_eq!(
            row.matrix_lane_ref,
            M5CompanionMatrixLane::OffboardingContinuity.as_str()
        );
    }
}

#[test]
fn canonical_surface_handoffs_are_exact() {
    let packet = packet();
    assert!(packet.all_handoffs_exact());
    assert!(!packet.usage_export_packages.is_empty());
    assert!(!packet.offboarding_packages.is_empty());
    assert!(!packet.grace_window_rows.is_empty());
    assert!(!packet.org_switch_rows.is_empty());
}

#[test]
fn every_surfaced_item_keeps_a_canonical_lifecycle_binding() {
    let packet = packet();
    assert_eq!(packet.lifecycle_bindings.len(), 13);
    for binding in &packet.lifecycle_bindings {
        assert!(!binding.request_case_ref.trim().is_empty());
        assert!(!binding.export_job_ref.trim().is_empty());
        assert!(!binding.delete_case_ref.trim().is_empty());
        assert!(!binding.export_outcome_token.trim().is_empty());
        assert!(!binding.delete_outcome_token.trim().is_empty());
        assert!(!binding.lifecycle_scope_note.trim().is_empty());
    }
    assert!(packet.lifecycle_bindings.iter().any(|binding| {
        binding.section == OffboardingSection::GraceWindowState
            && binding.item_id == "grace:0003"
            && binding.request_case_ref == "request-case:offboarding:0001"
            && binding.export_job_ref == "export-job:offboarding:0001"
            && binding.delete_case_ref == "delete-case:offboarding:0001"
            && binding.delete_outcome_token == "policy_retained"
    }));
}

#[test]
fn every_section_is_read_only() {
    let packet = packet();
    assert!(packet
        .usage_export_packages
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .offboarding_packages
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .grace_window_rows
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
    assert!(packet
        .org_switch_rows
        .iter()
        .all(|item| item.read_write_scope == CompanionReadWriteScope::ReadOnly));
}

#[test]
fn local_first_paths_are_always_offered() {
    let packet = packet();
    assert!(packet.usage_export_local_path_available());
    assert!(packet.offboarding_package_local_path_available());
    assert!(packet
        .usage_export_packages
        .iter()
        .any(|item| item.availability == PackageAvailability::LocalReady));
    assert!(packet
        .offboarding_packages
        .iter()
        .any(|item| item.availability.is_local_path()));
}

#[test]
fn completeness_claims_are_provable_or_labeled() {
    let packet = packet();
    assert!(packet.export_completeness_honestly_qualified());
    assert!(packet
        .usage_export_packages
        .iter()
        .any(
            |item| item.completeness == ExportCompleteness::CompleteVerified && item.claim_verified
        ));
    let unverified = packet
        .usage_export_packages
        .iter()
        .find(|item| item.completeness == ExportCompleteness::CompleteUnverified)
        .expect("unverified completeness present");
    assert!(unverified.proof_label_shown);
    assert!(!unverified.claim_verified);
}

#[test]
fn deletion_is_honestly_labeled() {
    let packet = packet();
    assert!(packet.deletion_honestly_labeled());
    // The corpus carries reversible and committed-irreversible deletions.
    assert!(packet
        .grace_window_rows
        .iter()
        .any(|item| item.grace_posture == GraceWindowPosture::OpenReversible));
    let committed = packet
        .grace_window_rows
        .iter()
        .find(|item| item.grace_posture == GraceWindowPosture::CommittedIrreversible)
        .expect("committed deletion present");
    assert!(committed.irreversible_labeled);
    assert!(!committed.reversible);
}

#[test]
fn offboarding_never_strands_local_work() {
    let packet = packet();
    assert!(packet.local_work_never_stranded());
    assert!(packet
        .offboarding_packages
        .iter()
        .all(|item| item.local_work_preserved));
    // The user-owned local workspace stays with the user on a switch; the prior-org
    // audit trail is left with the prior org but is not user-owned.
    let user_owned = packet
        .org_switch_rows
        .iter()
        .find(|item| item.user_owned)
        .expect("user-owned row present");
    assert!(user_owned.user_owned_local_retained);
    assert!(!user_owned.disposition.left_with_prior_org());
    assert!(packet.org_switch_rows.iter().any(|item| item.disposition
        == OrgSwitchDisposition::LeftWithPriorOrg
        && !item.user_owned));
}

#[test]
fn org_switch_admin_flags_match_disposition() {
    let packet = packet();
    assert!(packet
        .org_switch_rows
        .iter()
        .all(|item| item.requires_admin == item.disposition.requires_admin()));
    assert!(packet
        .org_switch_rows
        .iter()
        .any(|item| item.disposition == OrgSwitchDisposition::RequiresAdminApproval));
}

#[test]
fn stale_items_are_honestly_labeled() {
    let packet = packet();
    assert!(packet.stale_state_honestly_labeled());
}

#[test]
fn missing_section_fails_validation() {
    let mut packet = packet();
    packet.section_qualifications.pop();
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::RequiredSectionMissing));
}

#[test]
fn section_lane_mismatch_fails() {
    let mut packet = packet();
    packet.section_qualifications[0].matrix_lane_ref = "residency_encryption".to_owned();
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::SectionLaneMismatch));
}

#[test]
fn read_only_item_with_write_scope_fails() {
    let mut packet = packet();
    packet.usage_export_packages[0].read_write_scope =
        CompanionReadWriteScope::BoundedWriteRelayedToHost;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::ReadOnlyScopeViolated));
}

#[test]
fn missing_local_usage_export_path_fails() {
    let mut packet = packet();
    packet
        .usage_export_packages
        .retain(|item| !item.availability.is_local_path());
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::LocalUsageExportPathMissing));
}

#[test]
fn missing_local_offboarding_path_fails() {
    let mut packet = packet();
    packet
        .offboarding_packages
        .retain(|item| !item.availability.is_local_path());
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::LocalOffboardingPathMissing));
}

#[test]
fn verified_completeness_claim_without_verification_fails() {
    let mut packet = packet();
    let verified = packet
        .usage_export_packages
        .iter_mut()
        .find(|item| item.completeness.is_complete_claim())
        .expect("verified claim present");
    verified.claim_verified = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::CompletenessClaimedButUnverified));
}

#[test]
fn unverified_completeness_without_label_fails() {
    let mut packet = packet();
    let unverified = packet
        .usage_export_packages
        .iter_mut()
        .find(|item| item.completeness == ExportCompleteness::CompleteUnverified)
        .expect("unverified completeness present");
    unverified.proof_label_shown = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::CompletenessClaimNotLabeled));
}

#[test]
fn deletion_reversibility_mismatch_fails() {
    let mut packet = packet();
    let committed = packet
        .grace_window_rows
        .iter_mut()
        .find(|item| item.grace_posture == GraceWindowPosture::CommittedIrreversible)
        .expect("committed deletion present");
    committed.reversible = true;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::DeletionReversibilityMismatch));
}

#[test]
fn irreversible_deletion_without_label_fails() {
    let mut packet = packet();
    let committed = packet
        .grace_window_rows
        .iter_mut()
        .find(|item| item.grace_posture == GraceWindowPosture::CommittedIrreversible)
        .expect("committed deletion present");
    committed.irreversible_labeled = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::IrreversibleDeletionNotLabeled));
}

#[test]
fn org_switch_admin_flag_mismatch_fails() {
    let mut packet = packet();
    packet.org_switch_rows[0].requires_admin = !packet.org_switch_rows[0].requires_admin;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::OrgSwitchAdminFlagMismatch));
}

#[test]
fn user_owned_local_left_with_prior_org_fails() {
    let mut packet = packet();
    let user_owned = packet
        .org_switch_rows
        .iter_mut()
        .find(|item| item.user_owned)
        .expect("user-owned row present");
    user_owned.disposition = OrgSwitchDisposition::LeftWithPriorOrg;
    user_owned.requires_admin = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::UserLocalWorkStranded));
}

#[test]
fn stranded_offboarding_local_work_fails() {
    let mut packet = packet();
    packet.offboarding_packages[0].local_work_preserved = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::LocalWorkStranded));
}

#[test]
fn unlabeled_stale_item_fails() {
    let mut packet = packet();
    packet.usage_export_packages[0].freshness = CompanionFreshnessState::Stale;
    packet.usage_export_packages[0].stale_label_shown = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::StaleStateNotLabeled));
}

#[test]
fn missing_handoff_ref_fails() {
    let mut packet = packet();
    packet.usage_export_packages[0].handoff.deep_link_ref = String::new();
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::HandoffRefMissing));
}

#[test]
fn missing_lifecycle_binding_fails() {
    let mut packet = packet();
    let missing_item_id = packet.usage_export_packages[0].item_id.clone();
    packet
        .lifecycle_bindings
        .retain(|binding| binding.item_id != missing_item_id);
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::MissingLifecycleBinding));
}

#[test]
fn terminal_lifecycle_delete_without_receipt_fails() {
    let mut packet = packet();
    packet.lifecycle_bindings[0].delete_outcome_token = "completed".to_owned();
    packet.lifecycle_bindings[0].destruction_receipt_ref = None;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::LifecycleReceiptMissing));
}

#[test]
fn empty_section_content_fails() {
    let mut packet = packet();
    packet.org_switch_rows.clear();
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::SectionContentMissing));
}

#[test]
fn scope_contract_incomplete_fails() {
    let mut packet = packet();
    packet
        .scope_contract
        .action_applied_by_local_core_not_surface = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::ScopeContractIncomplete));
}

#[test]
fn honesty_contract_incomplete_fails() {
    let mut packet = packet();
    packet
        .honesty_contract
        .user_owned_local_never_left_with_prior_org = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::HonestyContractIncomplete));
}

#[test]
fn stale_state_honesty_incomplete_fails() {
    let mut packet = packet();
    packet.stale_state_honesty.never_show_stale_as_live = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::StaleStateHonestyIncomplete));
}

#[test]
fn continuity_contract_incomplete_fails() {
    let mut packet = packet();
    packet.continuity_contract.local_work_never_stranded = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::ContinuityContractIncomplete));
}

#[test]
fn locality_disclosure_incomplete_fails() {
    let mut packet = packet();
    packet.locality_disclosure.staged = String::new();
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::LocalityDisclosureIncomplete));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::MissingSourceContracts));
}

#[test]
fn security_review_incomplete_fails() {
    let mut packet = packet();
    packet.security_review.local_work_never_stranded = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::SecurityReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = packet();
    packet
        .consumer_projection
        .help_about_shows_deletion_and_export_honesty = false;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&OffboardingViolation::ProofFreshnessIncomplete));
}

fn healthy_observation() -> OffboardingDegradationObservation {
    OffboardingDegradationObservation {
        export_assembler_available: true,
        deletion_service_available: true,
        proof_fresh: true,
        completeness_verified: true,
        admin_continuity_available: true,
        managed_service_available: true,
        host_session_active: true,
        upstream_matrix_narrowed: false,
    }
}

#[test]
fn degradation_on_managed_service_degraded_narrows_and_stales_items() {
    let mut packet = packet();
    packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
        managed_service_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::ManagedServiceDegraded));
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::FreshnessDowngradedToStale));
    // Every previously live/cached item is now stale, and every item that requires a
    // freshness label carries one.
    assert!(packet
        .usage_export_packages
        .iter()
        .all(|item| item.freshness.requires_label() && item.stale_label_shown));
    // Local work is still preserved everywhere and local paths still offered.
    assert!(packet.local_work_never_stranded());
    assert!(packet.usage_export_local_path_available());
    assert!(packet.offboarding_package_local_path_available());
    // The usage-export section narrows beta → preview.
    let usage = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == OffboardingSection::UsageExportPackage)
        .expect("usage section present");
    assert_eq!(usage.qualification, M5CompanionQualificationClass::Preview);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_export_assembler_loss_narrows_to_local_path() {
    let mut packet = packet();
    packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
        export_assembler_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::ExportAssemblerUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::PackageNarrowedToLocalPath));
    // Local paths stay available; provider-assembled packages narrow to unavailable.
    assert!(packet.usage_export_local_path_available());
    assert!(packet.offboarding_package_local_path_available());
    assert!(packet
        .usage_export_packages
        .iter()
        .filter(|item| !item.availability.is_local_path())
        .all(|item| item.availability == PackageAvailability::Unavailable));
    // Usage-export and offboarding sections narrow; grace-window stays preview.
    let usage = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == OffboardingSection::UsageExportPackage)
        .expect("usage section present");
    assert_eq!(usage.qualification, M5CompanionQualificationClass::Preview);
    let grace = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == OffboardingSection::GraceWindowState)
        .expect("grace section present");
    assert_eq!(grace.qualification, M5CompanionQualificationClass::Preview);
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_completeness_unverified_downgrades_claims() {
    let mut packet = packet();
    packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
        completeness_verified: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::CompletenessUnverified));
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::CompletenessClaimDowngraded));
    assert!(packet
        .usage_export_packages
        .iter()
        .all(|item| !item.completeness.is_complete_claim()));
    assert!(packet
        .offboarding_packages
        .iter()
        .all(|item| !item.completeness.is_complete_claim()));
    assert!(packet.export_completeness_honestly_qualified());
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_deletion_service_loss_holds_grace_windows_open() {
    let mut packet = packet();
    packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
        deletion_service_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::DeletionServiceUnavailable));
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::GraceWindowHeldOpen));
    // No closing window remains; an already-committed deletion stays committed.
    assert!(packet
        .grace_window_rows
        .iter()
        .all(|item| item.grace_posture != GraceWindowPosture::ClosingReversible));
    assert!(packet
        .grace_window_rows
        .iter()
        .any(|item| item.grace_posture == GraceWindowPosture::CommittedIrreversible));
    // Held-open windows are reversible and local work is preserved.
    assert!(packet
        .grace_window_rows
        .iter()
        .all(|item| item.reversible == item.grace_posture.is_reversible()));
    assert!(packet.local_work_never_stranded());
    // The grace-window section narrows preview → experimental.
    let grace = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == OffboardingSection::GraceWindowState)
        .expect("grace section present");
    assert_eq!(
        grace.qualification,
        M5CompanionQualificationClass::Experimental
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_admin_continuity_loss_narrows_offboarding_grace_and_org_switch() {
    let mut packet = packet();
    packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
        admin_continuity_available: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::AdminContinuityUnavailable));
    // Usage-export is untouched by admin continuity loss; the other three narrow.
    let usage = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == OffboardingSection::UsageExportPackage)
        .expect("usage section present");
    assert_eq!(usage.qualification, M5CompanionQualificationClass::Beta);
    let offboarding = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == OffboardingSection::OffboardingPackage)
        .expect("offboarding section present");
    assert_eq!(
        offboarding.qualification,
        M5CompanionQualificationClass::Preview
    );
    let org = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == OffboardingSection::OrgSwitchSemantics)
        .expect("org section present");
    assert_eq!(
        org.qualification,
        M5CompanionQualificationClass::Experimental
    );
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn degradation_on_inactive_host_unresolves_exact_handoffs() {
    let mut packet = packet();
    packet.usage_export_packages[0].handoff.requires_active_host = true;
    packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
        host_session_active: false,
        ..healthy_observation()
    });
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::HostSessionInactive));
    assert!(packet
        .degraded_labels
        .contains(&OffboardingDegradedReason::HandoffTargetUnresolved));
    assert!(packet
        .handoffs()
        .filter(|handoff| handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Unresolved));
    assert!(packet
        .handoffs()
        .filter(|handoff| !handoff.requires_active_host)
        .all(|handoff| handoff.resolution == CompanionHandoffResolution::Exact));
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
}

#[test]
fn publishable_sections_excludes_withheld() {
    let mut packet = packet();
    let total = packet.section_qualifications.len();
    assert_eq!(packet.publishable_sections().count(), total);
    // Drive the preview grace-window section down to withheld via repeated stale-proof
    // narrowing (early_access → internal_only → withheld).
    for _ in 0..2 {
        packet.apply_offboarding_degradation(&OffboardingDegradationObservation {
            proof_fresh: false,
            ..healthy_observation()
        });
    }
    let grace = packet
        .section_qualifications
        .iter()
        .find(|row| row.section == OffboardingSection::GraceWindowState)
        .expect("grace section present");
    assert_eq!(grace.rollout_stage, M5CompanionRolloutStage::Withheld);
    assert!(packet.publishable_sections().count() < total);
}

#[test]
fn export_contains_no_forbidden_material() {
    let packet = packet();
    assert!(!packet
        .validate()
        .contains(&OffboardingViolation::RawBoundaryMaterialInExport));
}

#[test]
fn checked_support_export_validates() {
    let packet =
        current_stable_usage_export_offboarding_surface_export().expect("checked export validates");
    assert_eq!(packet.packet_id, PACKET_ID);
}

#[test]
fn checked_support_export_matches_canonical_builder() {
    let checked =
        current_stable_usage_export_offboarding_surface_export().expect("checked export validates");
    assert_eq!(
        checked,
        packet(),
        "checked export drifted from canonical builder"
    );
}

#[test]
fn markdown_summary_is_deterministic() {
    let packet = packet();
    let first = packet.render_markdown_summary();
    let second = packet.render_markdown_summary();
    assert_eq!(first, second);
    assert!(first.contains("## Usage export packages"));
    assert!(first.contains("## Offboarding packages"));
    assert!(first.contains("## Grace window"));
    assert!(first.contains("## Org-switch semantics"));
}
