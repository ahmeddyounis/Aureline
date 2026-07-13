use super::*;

fn clean_banner_input() -> M5BannerResolutionInput {
    M5BannerResolutionInput {
        banner_id: "banner:test".to_owned(),
        banner_label: "Review index is showing cached results while the fresh scan finishes"
            .to_owned(),
        notice_scope: M5NoticeScope::PageScoped,
        disposition: M5DecisionFeedbackDisposition::Warning,
        surface_context: M5DecisionStateSurfaceContext::ReviewWorkspace,
        degraded_variant: M5DegradedStateVariant::PartialCapability,
        action_posture: M5BannerActionPosture::PrimaryNextAction,
        cause_named: true,
        what_still_works_stated: true,
        primary_next_action_present: true,
        support_or_help_backlink_present: true,
        avoids_generic_failure_language: true,
        reconstructable_from_export: true,
        proof_fresh: true,
    }
}

fn clean_empty_input() -> M5EmptyStateResolutionInput {
    M5EmptyStateResolutionInput {
        empty_state_id: "empty:test".to_owned(),
        empty_state_label: "No review items yet — approved changes will appear here".to_owned(),
        empty_purpose: M5EmptyStatePurpose::ExplainsPurpose,
        disposition: M5DecisionFeedbackDisposition::Info,
        surface_context: M5DecisionStateSurfaceContext::ReviewWorkspace,
        degraded_variant: M5DegradedStateVariant::PartialCapability,
        empty_reason: M5EmptyStateReason::NeverPopulated,
        purpose_stated: true,
        emptiness_explained: true,
        best_next_action_present: true,
        avoids_decorative_filler: true,
        avoids_generic_failure_language: true,
        reconstructable_from_export: true,
        proof_fresh: true,
    }
}

#[test]
fn seeded_controls_validates() {
    let packet = seeded_m5_banner_empty_state_controls();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_BANNER_EMPTY_STATE_CONTROLS_PACKET_ID);
}

#[test]
fn banner_clean_states_scope_cause_and_next_action() {
    let resolved = resolve_banner(clean_banner_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.states_scope_cause_and_next_action);
    assert!(!resolved.scope_is_unscoped_or_color_only);
    assert_eq!(resolved.notice_scope, "page_scoped");
    assert_eq!(resolved.surface_context, "review_workspace");
    assert_eq!(resolved.degraded_variant, "partial_capability");
    assert_eq!(
        resolved.next_action,
        M5BannerEmptyStateNextAction::ReviewScopeAndCause
    );
}

#[test]
fn banner_label_unstated_degrades() {
    let mut input = clean_banner_input();
    input.banner_label = "   ".to_owned();
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::BannerLabelUnstated)
    );
}

#[test]
fn banner_unscoped_or_color_only_degrades() {
    let mut input = clean_banner_input();
    input.notice_scope = M5NoticeScope::UnscopedColorOnlyDisallowed;
    let resolved = resolve_banner(input).unwrap();
    assert!(resolved.scope_is_unscoped_or_color_only);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5BannerDegradeReason::ScopeUnscopedOrColorOnly)
    );
}

#[test]
fn banner_variant_and_cause_and_what_still_works_degrade() {
    let mut input = clean_banner_input();
    input.degraded_variant = M5DegradedStateVariant::VariantUnknown;
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::DegradedVariantUnresolved)
    );

    let mut input = clean_banner_input();
    input.cause_named = false;
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::CauseUnstated)
    );

    let mut input = clean_banner_input();
    input.what_still_works_stated = false;
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::WhatStillWorksUnstated)
    );
}

#[test]
fn banner_next_action_backlink_generic_and_reconstructable_degrade() {
    let mut input = clean_banner_input();
    input.primary_next_action_present = false;
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::PrimaryNextActionMissing)
    );

    let mut input = clean_banner_input();
    input.support_or_help_backlink_present = false;
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::SupportBacklinkMissing)
    );

    let mut input = clean_banner_input();
    input.avoids_generic_failure_language = false;
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::GenericFailureLanguageUsed)
    );

    let mut input = clean_banner_input();
    input.reconstructable_from_export = false;
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::NotReconstructableFromExport)
    );
}

#[test]
fn banner_surface_unresolved_degrades() {
    let mut input = clean_banner_input();
    input.surface_context = M5DecisionStateSurfaceContext::ContextUnknown;
    assert_eq!(
        resolve_banner(input).unwrap().degrade_reason,
        Some(M5BannerDegradeReason::SurfaceContextUnresolved)
    );
}

#[test]
fn banner_empty_id_and_forbidden_material_error() {
    let mut input = clean_banner_input();
    input.banner_id = "".to_owned();
    assert_eq!(
        resolve_banner(input).unwrap_err(),
        M5BannerEmptyStateResolutionError::EmptyBannerId
    );

    let mut input = clean_banner_input();
    input.banner_label = "https://relay.internal/leak".to_owned();
    assert_eq!(
        resolve_banner(input).unwrap_err(),
        M5BannerEmptyStateResolutionError::ForbiddenMaterial
    );
}

#[test]
fn empty_state_clean_states_purpose_emptiness_and_next_action() {
    let resolved = resolve_empty_state(clean_empty_input()).unwrap();
    assert!(resolved.is_clean());
    assert!(resolved.states_purpose_emptiness_and_next_action);
    assert!(!resolved.purpose_is_blank_disallowed);
    assert_eq!(resolved.empty_purpose, "explains_purpose");
    assert_eq!(resolved.empty_reason, "never_populated");
    assert_eq!(
        resolved.next_action,
        M5BannerEmptyStateNextAction::ReadEmptyStatePurpose
    );
}

#[test]
fn empty_label_unstated_degrades() {
    let mut input = clean_empty_input();
    input.empty_state_label = "   ".to_owned();
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::EmptyLabelUnstated)
    );
}

#[test]
fn empty_purpose_blank_disallowed_degrades() {
    let mut input = clean_empty_input();
    input.empty_purpose = M5EmptyStatePurpose::BlankNoExplanationDisallowed;
    let resolved = resolve_empty_state(input).unwrap();
    assert!(resolved.purpose_is_blank_disallowed);
    assert_eq!(
        resolved.degrade_reason,
        Some(M5EmptyStateDegradeReason::PurposeAsBlankDisallowed)
    );
}

#[test]
fn empty_variant_purpose_and_emptiness_degrade() {
    let mut input = clean_empty_input();
    input.degraded_variant = M5DegradedStateVariant::VariantUnknown;
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::DegradedVariantUnresolved)
    );

    let mut input = clean_empty_input();
    input.purpose_stated = false;
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::PurposeUnstated)
    );

    let mut input = clean_empty_input();
    input.empty_reason = M5EmptyStateReason::ReasonUnknown;
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::EmptinessReasonUnresolved)
    );

    let mut input = clean_empty_input();
    input.emptiness_explained = false;
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::EmptinessReasonUnresolved)
    );
}

#[test]
fn empty_best_action_filler_generic_and_reconstructable_degrade() {
    let mut input = clean_empty_input();
    input.best_next_action_present = false;
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::BestNextActionMissing)
    );

    let mut input = clean_empty_input();
    input.avoids_decorative_filler = false;
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::DecorativeFillerUsed)
    );

    let mut input = clean_empty_input();
    input.avoids_generic_failure_language = false;
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::GenericFailureLanguageUsed)
    );

    let mut input = clean_empty_input();
    input.reconstructable_from_export = false;
    assert_eq!(
        resolve_empty_state(input).unwrap().degrade_reason,
        Some(M5EmptyStateDegradeReason::NotReconstructableFromExport)
    );
}

#[test]
fn empty_empty_id_and_forbidden_material_error() {
    let mut input = clean_empty_input();
    input.empty_state_id = "   ".to_owned();
    assert_eq!(
        resolve_empty_state(input).unwrap_err(),
        M5BannerEmptyStateResolutionError::EmptyEmptyStateId
    );

    let mut input = clean_empty_input();
    input.empty_state_label = "see internal://notes".to_owned();
    assert_eq!(
        resolve_empty_state(input).unwrap_err(),
        M5BannerEmptyStateResolutionError::ForbiddenMaterial
    );
}

#[test]
fn vocabulary_set_is_canonical() {
    assert!(seeded_m5_banner_empty_state_controls()
        .vocabulary_set
        .matches_canonical());
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.vocabulary_set.degraded_variants.pop();
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::VocabularySetDrift));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::MissingSourceContracts));
}

#[test]
fn component_schema_ref_missing_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.controls_rows[0]
        .source_contract_refs
        .retain(|r| r != M5_BANNER_INLINE_NOTICE_SCHEMA_REF);
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ComponentSchemaRefMissing));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.controls_rows[0]
        .anatomy_parts
        .retain(|p| *p != M5BannerEmptyStateAnatomyPart::Identity);
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_field_missing_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.controls_rows[0]
        .export_fields
        .retain(|f| *f != M5BannerEmptyStateExportField::Dispositions);
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::MandatoryExportFieldMissing));
}

#[test]
fn examples_missing_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.controls_rows[0].empty_state_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ExamplesMissing));
}

#[test]
fn dishonest_clean_example_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    // Force a clean banner to also read as generic failure language — the packet must reject it.
    let row = &mut packet.controls_rows[0];
    row.banner_examples[0].degrade_reason = None;
    row.banner_examples[0].avoids_generic_failure_language = false;
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::DishonestExample));
}

#[test]
fn row_invariant_violation_fails() {
    for mutate in 0u8..4 {
        let mut packet = seeded_m5_banner_empty_state_controls();
        let row = &mut packet.controls_rows[0];
        match mutate {
            0 => row.banner_relies_on_color_alone_for_meaning = true,
            1 => row.banner_uses_generic_failure_language = true,
            2 => row.empty_state_blanks_pane_without_next_action = true,
            _ => row.empty_state_uses_decorative_marketing_filler = true,
        }
        assert!(packet
            .validate()
            .contains(&M5BannerEmptyStateControlsViolation::RowInvariantViolated));
    }
}

#[test]
fn generic_language_not_proven_when_generic_banner_removed() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    for row in &mut packet.controls_rows {
        row.banner_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BannerDegradeReason::GenericFailureLanguageUsed)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::GenericLanguageAndNextActionNotProven));
}

#[test]
fn generic_language_not_proven_when_next_action_example_removed() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    for row in &mut packet.controls_rows {
        row.banner_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BannerDegradeReason::PrimaryNextActionMissing)
        });
        row.empty_state_examples.retain(|ex| {
            ex.degrade_reason != Some(M5EmptyStateDegradeReason::BestNextActionMissing)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::GenericLanguageAndNextActionNotProven));
}

#[test]
fn scope_vocabulary_not_proven_when_unscoped_example_removed() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    for row in &mut packet.controls_rows {
        row.banner_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BannerDegradeReason::ScopeUnscopedOrColorOnly)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ScopeAndDegradedVocabularyNotProven));
}

#[test]
fn scope_vocabulary_not_proven_when_variant_coverage_collapses() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    // Drop every clean example carrying the restricted-access variant so the variant grammar no longer
    // covers it.
    for row in &mut packet.controls_rows {
        row.banner_examples
            .retain(|ex| !(ex.is_clean() && ex.degraded_variant == "restricted_access"));
        row.empty_state_examples
            .retain(|ex| !(ex.is_clean() && ex.degraded_variant == "restricted_access"));
    }
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ScopeAndDegradedVocabularyNotProven));
}

#[test]
fn reconstructable_not_proven_when_banner_screenshot_example_removed() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    for row in &mut packet.controls_rows {
        row.banner_examples.retain(|ex| {
            ex.degrade_reason != Some(M5BannerDegradeReason::NotReconstructableFromExport)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ReconstructableFromExportNotProven));
}

#[test]
fn reconstructable_not_proven_when_empty_screenshot_example_removed() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    for row in &mut packet.controls_rows {
        row.empty_state_examples.retain(|ex| {
            ex.degrade_reason != Some(M5EmptyStateDegradeReason::NotReconstructableFromExport)
        });
    }
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ReconstructableFromExportNotProven));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet
        .governance_review
        .empty_state_never_blank_without_explanation = false;
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet
        .consumer_projection
        .support_export_reads_single_banner_empty_state_source = false;
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::ReleasePostureIncomplete));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_banner_empty_state_controls();
    packet.controls_rows[0].scope_summary =
        "raw endpoint https://relay.internal.example/session leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5BannerEmptyStateControlsViolation::RawMaterialInExport));
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_banner_empty_state_controls().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn csv_has_a_row_per_consumer_surface() {
    let packet = seeded_m5_banner_empty_state_controls();
    let csv = packet.render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + packet.controls_rows.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let packet = seeded_m5_banner_empty_state_controls();
    let summary = packet.render_markdown_summary();
    for row in &packet.controls_rows {
        assert!(summary.contains(row.consumer_surface.as_str()));
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_banner_empty_state_controls_export()
        .expect("checked M5 banner / empty-state controls export validates");
    assert_eq!(
        from_disk.packet_id,
        M5_BANNER_EMPTY_STATE_CONTROLS_PACKET_ID
    );
    assert_eq!(
        from_disk,
        seeded_m5_banner_empty_state_controls(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_rows_visible() {
    let beta = seeded_m5_banner_empty_state_controls_review_ui_beta_narrowed();
    assert!(beta.validate().is_empty(), "{:?}", beta.validate());
    assert_eq!(beta.controls_rows.len(), 6);
    let row = beta
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5DecisionFeedbackConsumerSurface::ReviewUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Beta
    );

    let preview = seeded_m5_banner_empty_state_controls_updates_ui_preview_narrowed();
    assert!(preview.validate().is_empty(), "{:?}", preview.validate());
    assert_eq!(preview.controls_rows.len(), 6);
    let row = preview
        .controls_rows
        .iter()
        .find(|r| r.consumer_surface == M5DecisionFeedbackConsumerSurface::UpdatesUi)
        .unwrap();
    assert_eq!(
        row.qualification,
        M5DecisionFeedbackQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let beta: M5BannerEmptyStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-banner-inline-notice-and-empty-state-controls/review_ui_beta_narrowed.json"
    )))
    .expect("review-ui fixture parses");
    assert!(beta.validate().is_empty());
    assert_eq!(
        beta,
        seeded_m5_banner_empty_state_controls_review_ui_beta_narrowed()
    );

    let preview: M5BannerEmptyStateControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-banner-inline-notice-and-empty-state-controls/updates_ui_preview_narrowed.json"
    )))
    .expect("updates-ui fixture parses");
    assert!(preview.validate().is_empty());
    assert_eq!(
        preview,
        seeded_m5_banner_empty_state_controls_updates_ui_preview_narrowed()
    );
}

#[test]
fn implemented_families_are_banner_and_empty_state() {
    assert_eq!(
        IMPLEMENTED_FAMILIES,
        [
            M5DecisionFeedbackFamily::BannerInlineNotice,
            M5DecisionFeedbackFamily::EmptyState
        ]
    );
}
