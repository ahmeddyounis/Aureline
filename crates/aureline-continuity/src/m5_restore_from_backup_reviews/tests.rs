use super::*;

fn page() -> RestoreReviewPage {
    seeded_restore_review_page()
}

fn review_mut<'a>(
    input: &'a mut RestoreReviewInput,
    review_id: &str,
) -> &'a mut RestoreReviewEntry {
    input
        .reviews
        .iter_mut()
        .find(|review| review.review_id == review_id)
        .unwrap_or_else(|| panic!("missing seeded review: {review_id}"))
}

#[test]
fn seeded_page_qualifies_stable_with_zero_defects() {
    let page = page();
    assert!(page.qualifies_stable());
    assert!(
        page.defects.is_empty(),
        "seeded defects: {:?}",
        page.defects
    );
    assert!(validate_restore_review_page(&page).is_ok());
}

#[test]
fn seeded_page_covers_managed_and_support_artifact_families() {
    let page = page();
    assert_eq!(page.summary.review_count, 5);
    // managed_record, policy_bundle, sync_metadata, support_record, local_workspace_state
    assert_eq!(page.summary.family_count, 5);
    assert_eq!(page.summary.managed_review_count, 4);
    assert!(page.summary.managed_family_compare_covered);
    assert!(page.summary.support_family_compare_covered);
    assert!(page
        .descriptor("continuity-restore:managed-records")
        .is_some());
    assert!(page
        .descriptor("continuity-restore:support-records")
        .is_some());
}

#[test]
fn seeded_review_labels_exact_and_narrower_restore_identity() {
    let page = page();
    let exact = page
        .descriptor("continuity-restore:managed-records")
        .expect("managed records descriptor");
    assert_eq!(exact.restore_fidelity_token, "exact_restore");
    assert!(exact.restore_summary_line.contains("exact"));
    assert!(exact.replicated_data_complete);

    let narrower = page
        .descriptor("continuity-restore:policy-bundle")
        .expect("policy bundle descriptor");
    assert_eq!(
        narrower.restore_fidelity_token,
        "narrower_than_normal_restore"
    );
    assert_eq!(narrower.affected_slice_token, "policy_bundle_revision_gap");
    assert!(narrower
        .restore_summary_line
        .contains("narrower than normal"));
    assert!(narrower
        .restore_summary_line
        .contains("policy bundle revisions"));
    assert!(!narrower.replicated_data_complete);
}

#[test]
fn seeded_review_fences_privileged_and_external_lanes() {
    let page = page();
    let descriptor = page
        .descriptor("continuity-restore:managed-records")
        .expect("descriptor");
    assert!(descriptor.privileged_lanes_fenced);
    assert!(descriptor.replay_fence_line.contains("do not auto-replay"));
    // Two fenced privileged/external lanes (admin policy apply, webhook redelivery).
    assert!(descriptor.replay_fence_line.contains("2 held for review"));
    let outcome = page
        .outcome("continuity-restore:managed-records")
        .expect("outcome");
    assert!(outcome.privileged_lanes_fenced);
    assert!(!outcome.narrowed);
}

#[test]
fn seeded_review_offers_compare_and_export_parity() {
    let page = page();
    let descriptor = page
        .descriptor("continuity-restore:support-records")
        .expect("descriptor");
    assert!(descriptor.compare_export_available);
    assert!(descriptor.compare_export_line.contains("compare available"));
    assert!(descriptor.compare_export_line.contains("export available"));
}

#[test]
fn seeded_summary_flags_guardrails_and_coverage() {
    let page = page();
    assert!(page.summary.no_unsafe_auto_replay);
    assert!(page.summary.no_full_normal_status_overclaim);
    assert!(page.summary.no_restore_lane_conflation);
    assert!(page.summary.all_narrower_restores_name_affected_slice);
    assert!(page.summary.all_expected_claims_covered);
    assert!(page.summary.restore_truth_export_safe);
    assert!(page.summary.raw_payloads_excluded);
    assert_eq!(page.summary.covered_claim_count, 5);
    assert_eq!(page.summary.uncovered_claim_count, 0);
    assert_eq!(page.summary.exact_restore_count, 3);
    assert_eq!(page.summary.narrower_than_normal_count, 2);
}

#[test]
fn registry_reports_every_restored_row_covered() {
    let page = page();
    let registry = RestoreReviewRegistry::from_page(&page);
    assert_eq!(registry, page.registry);
    assert!(registry.all_claims_covered());
    assert!(registry.is_claim_row_covered("continuity:row:managed-records-restore"));
    let row = registry
        .coverage_for_claim_row("continuity:row:support-records-restore")
        .expect("coverage row");
    assert_eq!(row.coverage_class, ReviewCoverageClass::CurrentReview);
    assert!(row.covered);
}

#[test]
fn every_surface_renders_identical_vocabulary_for_a_review() {
    let page = page();
    let review_id = "continuity-restore:policy-bundle";
    let descriptor = page.descriptor(review_id).expect("descriptor");
    let projections: Vec<&RestoreReviewSurfaceProjection> = page
        .surface_projections
        .iter()
        .filter(|projection| projection.review_id == review_id)
        .collect();
    assert_eq!(projections.len(), ReviewSurfaceClass::ALL.len());
    for projection in projections {
        assert_eq!(
            projection.restore_summary_line,
            descriptor.restore_summary_line
        );
        assert_eq!(projection.replay_fence_line, descriptor.replay_fence_line);
        assert_eq!(
            projection.compare_export_line,
            descriptor.compare_export_line
        );
    }
}

#[test]
fn surface_projection_count_matches_five_reviews_across_five_surfaces() {
    let page = page();
    assert_eq!(
        page.summary.surface_projection_count,
        5 * ReviewSurfaceClass::ALL.len()
    );
}

#[test]
fn full_normal_status_overclaim_fails_closed_and_is_withdrawn() {
    let mut input = seeded_restore_review_input();
    // The policy-bundle review is narrower than normal; assert full normal status.
    review_mut(&mut input, "continuity-restore:policy-bundle")
        .identity_summary
        .asserts_full_normal_status = true;

    let page = RestoreReviewPage::new("t:ov", "ov", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.summary.no_full_normal_status_overclaim);
    let outcome = page
        .outcome("continuity-restore:policy-bundle")
        .expect("outcome");
    assert!(outcome.claim_withheld);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::FullNormalStatusOverclaimed));
    let row = page
        .registry
        .coverage_for_claim_row("continuity:row:policy-bundle-restore")
        .expect("row");
    assert_eq!(row.coverage_class, ReviewCoverageClass::ReviewWithheld);
    assert!(!page.summary.all_expected_claims_covered);
}

#[test]
fn missing_replicated_data_with_green_status_fails_closed() {
    let mut input = seeded_restore_review_input();
    // The sync-metadata review is exact and complete; drop replicated data while
    // still asserting full normal status.
    let review = review_mut(&mut input, "continuity-restore:sync-metadata");
    review.identity_summary.replicated_data_complete = false;
    // It already asserts full normal status in the seed.

    let page = RestoreReviewPage::new("t:rd", "rd", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::FullNormalStatusOverclaimed));
}

#[test]
fn restore_lane_conflation_fails_closed_and_is_withdrawn() {
    let mut input = seeded_restore_review_input();
    review_mut(&mut input, "continuity-restore:managed-records").restore_lane =
        RestoreLaneClass::OrdinaryWorkspaceRestore;
    review_mut(&mut input, "continuity-restore:managed-records").restore_lane_token =
        RestoreLaneClass::OrdinaryWorkspaceRestore
            .as_str()
            .to_owned();

    let page = RestoreReviewPage::new("t:lc", "lc", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.summary.no_restore_lane_conflation);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::RestoreLaneConflated));
}

#[test]
fn privileged_lane_auto_replay_fails_closed_and_is_withdrawn() {
    let mut input = seeded_restore_review_input();
    let fence = &mut review_mut(&mut input, "continuity-restore:policy-bundle").replay_fences[0];
    fence.fence_state = ReplayFenceStateClass::NoFenceLocalSafe;
    fence.fence_state_token = ReplayFenceStateClass::NoFenceLocalSafe.as_str().to_owned();

    let page = RestoreReviewPage::new("t:ar", "ar", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Withdrawn.as_str()
    );
    assert!(!page.summary.no_unsafe_auto_replay);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::PrivilegedLaneAutoReplayed));
    let descriptor = page
        .descriptor("continuity-restore:policy-bundle")
        .expect("descriptor");
    assert!(!descriptor.privileged_lanes_fenced);
    assert!(descriptor.replay_fence_line.contains("WARNING"));
}

#[test]
fn cleared_fence_without_review_ref_narrows_to_beta() {
    let mut input = seeded_restore_review_input();
    review_mut(&mut input, "continuity-restore:policy-bundle").replay_fences[0].reviewed_step_ref =
        String::new();

    let page = RestoreReviewPage::new("t:rr", "rr", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::ReplayFenceReviewMissing));
}

#[test]
fn undisclosed_restore_fidelity_narrows_to_beta() {
    let mut input = seeded_restore_review_input();
    let review = review_mut(&mut input, "continuity-restore:managed-records");
    review.identity_summary.restore_fidelity = RestoreFidelityClass::Undisclosed;
    review.identity_summary.restore_fidelity_token =
        RestoreFidelityClass::Undisclosed.as_str().to_owned();

    let page = RestoreReviewPage::new("t:uf", "uf", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::RestoreFidelityUndisclosed));
}

#[test]
fn narrower_restore_without_affected_slice_narrows_to_beta() {
    let mut input = seeded_restore_review_input();
    let review = review_mut(&mut input, "continuity-restore:policy-bundle");
    review.identity_summary.affected_slice = AffectedSliceClass::NoneNarrowed;
    review.identity_summary.affected_slice_token =
        AffectedSliceClass::NoneNarrowed.as_str().to_owned();
    review.identity_summary.affected_slice_note = String::new();

    let page = RestoreReviewPage::new("t:as", "as", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(!page.summary.all_narrower_restores_name_affected_slice);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::AffectedSliceUnnamed));
}

#[test]
fn undeclared_restore_identity_narrows_to_beta() {
    let mut input = seeded_restore_review_input();
    let review = review_mut(&mut input, "continuity-restore:managed-records");
    review.identity_summary.restore_identity = RestoreIdentityClass::NotApplicable;
    review.identity_summary.restore_identity_token =
        RestoreIdentityClass::NotApplicable.as_str().to_owned();

    let page = RestoreReviewPage::new("t:ri", "ri", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::RestoreIdentityUndeclared));
}

#[test]
fn missing_compare_parity_holds_at_preview() {
    let mut input = seeded_restore_review_input();
    let review = review_mut(&mut input, "continuity-restore:managed-records");
    review.compare_export.restored_vs_current_available = false;
    review.compare_export.compare_ref = String::new();

    let page = RestoreReviewPage::new("t:cp", "cp", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::CompareParityMissing));
}

#[test]
fn missing_export_parity_narrows_to_beta() {
    let mut input = seeded_restore_review_input();
    let review = review_mut(&mut input, "continuity-restore:sync-metadata");
    review.compare_export.export_available = false;
    review.compare_export.export_ref = String::new();

    let page = RestoreReviewPage::new("t:ep", "ep", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::ExportParityMissing));
}

#[test]
fn missing_support_family_compare_coverage_holds_at_preview() {
    let mut input = seeded_restore_review_input();
    // Drop the only support/export-family review.
    input
        .reviews
        .retain(|review| review.review_id != "continuity-restore:support-records");
    input
        .expected_claim_row_ids
        .retain(|id| id != "continuity:row:support-records-restore");

    let page = RestoreReviewPage::new("t:sc", "sc", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(!page.summary.support_family_compare_covered);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason
            == RestoreReviewNarrowReasonClass::SupportFamilyCompareCoverageMissing));
}

#[test]
fn incomplete_surface_projection_narrows_to_beta() {
    let mut input = seeded_restore_review_input();
    review_mut(&mut input, "continuity-restore:managed-records")
        .projected_surfaces
        .retain(|surface| *surface != ReviewSurfaceClass::ServiceHealth);

    let page = RestoreReviewPage::new("t:surf", "surf", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Beta.as_str()
    );
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::SurfaceReuseIncomplete));
}

#[test]
fn missing_review_for_claimed_row_narrows_at_preview() {
    let mut input = seeded_restore_review_input();
    input
        .reviews
        .retain(|review| review.review_id != "continuity-restore:sync-metadata");

    let page = RestoreReviewPage::new("t:miss", "miss", "2026-06-01T00:00:00Z", input);
    assert_eq!(
        page.summary.overall_qualification_token,
        ContinuityClaimQualificationClass::Preview.as_str()
    );
    assert!(!page.summary.all_expected_claims_covered);
    assert!(page
        .defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::ReviewEvidenceMissing));
    let row = page
        .registry
        .coverage_for_claim_row("continuity:row:sync-metadata-restore")
        .expect("row");
    assert_eq!(row.coverage_class, ReviewCoverageClass::NoReview);
}

#[test]
fn tampered_projection_vocabulary_is_detected_on_reaudit() {
    let mut page = page();
    page.surface_projections[0].restore_summary_line = "drifted vocabulary".to_owned();
    let defects = audit_restore_review_page(&page);
    assert!(defects
        .iter()
        .any(|d| d.narrow_reason == RestoreReviewNarrowReasonClass::ReviewVocabularyDrift));
    assert!(validate_restore_review_page(&page).is_err());
}

#[test]
fn local_ordinary_restore_is_exempt_from_managed_requirements() {
    let page = page();
    // The local ordinary restore declares no restore identity, no compare/export,
    // and no fence, yet it does not narrow because it is exempt.
    let outcome = page
        .outcome("ordinary-restore:local-workspace")
        .expect("outcome");
    assert!(!outcome.narrowed);
    assert_eq!(outcome.restore_identity_token, "not_applicable");
}

#[test]
fn support_export_wraps_seeded_page_without_raw_payloads() {
    let export = RestoreReviewSupportExport::from_page(
        "continuity:restore-from-backup-review:support-export:fixture-001",
        "2026-06-01T00:00:00Z",
        page(),
    );
    assert!(export.raw_payloads_excluded);
    assert!(export.restore_truth_export_safe);
    assert!(export.narrow_reasons_present.is_empty());
    assert!(export.page.qualifies_stable());
}

#[test]
fn reauditing_seeded_page_returns_zero_defects() {
    let defects = audit_restore_review_page(&page());
    assert!(defects.is_empty(), "re-audit defects: {defects:?}");
}

#[test]
fn page_round_trips_through_json() {
    let page = page();
    let json = serde_json::to_string(&page).expect("serialize");
    let restored: RestoreReviewPage = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(page, restored);
}
