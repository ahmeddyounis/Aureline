use super::*;

const TAXONOMY: &str = M5_STATE_EXPLANATION_COMPONENT_MATRIX_REF;

fn current_vs_selected_drawer() -> M5StateExplanationInput {
    M5StateExplanationInput {
        surface: M5ExplanationConsumerSurface::OnboardingHelp,
        distinction: M5ConfusableStateDistinction::CurrentVsSelected,
        delivery: M5ExplanationDelivery::ExpandedDrawer,
        recovery_class: M5RecoveryDisclosureClass::NamesConsequence,
        state_cause: M5StateCauseClass::UnknownCause,
        recovery_available: true,
        high_contrast_active: false,
        explanation_identity_ref: "explain:onboarding.current-vs-selected".to_owned(),
        taxonomy_ref: TAXONOMY.to_owned(),
        distinction_copy_ref: "copy:onboarding.current-vs-selected-drawer".to_owned(),
        blocked_limited_copy_ref: String::new(),
    }
}

fn pending_vs_loading_chip() -> M5StateExplanationInput {
    M5StateExplanationInput {
        surface: M5ExplanationConsumerSurface::OnboardingHelp,
        distinction: M5ConfusableStateDistinction::PendingVsLoading,
        delivery: M5ExplanationDelivery::InlineChip,
        recovery_class: M5RecoveryDisclosureClass::NamesRetryPath,
        state_cause: M5StateCauseClass::ConnectivityCause,
        recovery_available: true,
        high_contrast_active: false,
        explanation_identity_ref: "explain:onboarding.pending-vs-loading".to_owned(),
        taxonomy_ref: TAXONOMY.to_owned(),
        distinction_copy_ref: "copy:onboarding.pending-vs-loading-chip".to_owned(),
        blocked_limited_copy_ref: String::new(),
    }
}

fn locked_vs_disabled_blocked_copy() -> M5StateExplanationInput {
    M5StateExplanationInput {
        surface: M5ExplanationConsumerSurface::BlockedActionRow,
        distinction: M5ConfusableStateDistinction::LockedVsDisabled,
        delivery: M5ExplanationDelivery::BlockedLimitedCopy,
        recovery_class: M5RecoveryDisclosureClass::NamesRecoveryAction,
        state_cause: M5StateCauseClass::PolicyCause,
        recovery_available: true,
        high_contrast_active: false,
        explanation_identity_ref: "explain:blocked-action.locked-vs-disabled".to_owned(),
        taxonomy_ref: TAXONOMY.to_owned(),
        distinction_copy_ref: "copy:blocked-action.locked-vs-disabled-body".to_owned(),
        blocked_limited_copy_ref: "copy:blocked-action.locked-by-policy-owner-and-recovery"
            .to_owned(),
    }
}

// ---- state-distinction explanation resolver -----------------------------

#[test]
fn distinction_derives_precedence_rule_and_states() {
    for (distinction, rule, primary, contrasted) in [
        (
            M5ConfusableStateDistinction::CurrentVsSelected,
            M5StatePrecedenceRule::CurrentDistinctFromSelected,
            M5SharedComponentStateClass::Current,
            M5SharedComponentStateClass::Selected,
        ),
        (
            M5ConfusableStateDistinction::ReadOnlyVsDisabled,
            M5StatePrecedenceRule::ReadOnlyOverDisabled,
            M5SharedComponentStateClass::ReadOnly,
            M5SharedComponentStateClass::Disabled,
        ),
        (
            M5ConfusableStateDistinction::LockedVsDisabled,
            M5StatePrecedenceRule::LockedOverDisabled,
            M5SharedComponentStateClass::Locked,
            M5SharedComponentStateClass::Disabled,
        ),
        (
            M5ConfusableStateDistinction::PendingVsLoading,
            M5StatePrecedenceRule::PendingDistinctFromLoading,
            M5SharedComponentStateClass::Pending,
            M5SharedComponentStateClass::Loading,
        ),
    ] {
        assert_eq!(distinction.precedence_rule(), rule);
        assert_eq!(distinction.primary_state(), primary);
        assert_eq!(distinction.contrasted_state(), contrasted);
        // The two states never collapse into one another.
        assert_ne!(distinction.primary_state(), distinction.contrasted_state());
    }
}

#[test]
fn inline_chip_names_primary_state_and_marks_the_distinction() {
    let resolved =
        resolve_state_distinction_explanation(&pending_vs_loading_chip()).expect("resolves");
    assert_eq!(resolved.delivery, M5ExplanationDelivery::InlineChip);
    assert_eq!(
        resolved.required_non_color_cues,
        vec![
            M5ExplanationCue::PrimaryStateLabel,
            M5ExplanationCue::DistinctionMarker
        ]
    );
    assert!(!resolved.carries_blocked_action_help);
    assert!(resolved.touches_blocked_or_limited_state);
    assert!(resolved.explains_distinction_in_place);
    assert!(resolved.states_stay_distinct);
    assert!(resolved.no_one_off_language);
    assert!(resolved.aligned_with_shared_taxonomy);
    assert!(resolved.blocked_action_help_aligned_with_component_truth);
    assert!(resolved.no_color_only_signaling);
    assert!(resolved.keyboard_and_screen_reader_explainable);
    assert!(resolved.driven_by_shared_state_contract);
}

#[test]
fn expanded_drawer_names_both_states_and_links_taxonomy() {
    let resolved =
        resolve_state_distinction_explanation(&current_vs_selected_drawer()).expect("resolves");
    assert_eq!(resolved.delivery, M5ExplanationDelivery::ExpandedDrawer);
    for cue in [
        M5ExplanationCue::PrimaryStateLabel,
        M5ExplanationCue::ContrastedStateLabel,
        M5ExplanationCue::DistinctionMarker,
        M5ExplanationCue::TaxonomyReferenceCue,
    ] {
        assert!(
            resolved.required_non_color_cues.contains(&cue),
            "drawer missing cue {}",
            cue.as_str()
        );
    }
    for trigger in [
        M5StateDisclosureTrigger::StateCauseRequired,
        M5StateDisclosureTrigger::RecoveryActionRequired,
        M5StateDisclosureTrigger::SilentStyleOnlyForbidden,
    ] {
        assert!(resolved.required_disclosures.contains(&trigger));
    }
    assert!(!resolved.touches_blocked_or_limited_state);
}

#[test]
fn blocked_limited_copy_names_owner_block_reason_and_recovery() {
    let resolved = resolve_state_distinction_explanation(&locked_vs_disabled_blocked_copy())
        .expect("resolves");
    assert_eq!(resolved.delivery, M5ExplanationDelivery::BlockedLimitedCopy);
    assert!(resolved.carries_blocked_action_help);
    for trigger in [
        M5StateDisclosureTrigger::StateCauseRequired,
        M5StateDisclosureTrigger::OwnerRequired,
        M5StateDisclosureTrigger::BlockReasonRequired,
        M5StateDisclosureTrigger::RecoveryActionRequired,
        M5StateDisclosureTrigger::SilentStyleOnlyForbidden,
    ] {
        assert!(
            resolved.required_disclosures.contains(&trigger),
            "blocked/limited copy missing disclosure {}",
            trigger.as_str()
        );
    }
    assert!(resolved
        .required_non_color_cues
        .contains(&M5ExplanationCue::BlockedLimitedGlyph));
    assert!(resolved
        .required_non_color_cues
        .contains(&M5ExplanationCue::RecoveryAffordance));
}

#[test]
fn empty_identity_taxonomy_or_copy_is_rejected() {
    assert_eq!(
        resolve_state_distinction_explanation(&M5StateExplanationInput {
            explanation_identity_ref: "  ".to_owned(),
            ..current_vs_selected_drawer()
        }),
        Err(M5StateExplanationResolutionError::EmptyExplanationIdentity)
    );
    assert_eq!(
        resolve_state_distinction_explanation(&M5StateExplanationInput {
            taxonomy_ref: String::new(),
            ..current_vs_selected_drawer()
        }),
        Err(M5StateExplanationResolutionError::EmptyTaxonomyRef)
    );
    assert_eq!(
        resolve_state_distinction_explanation(&M5StateExplanationInput {
            distinction_copy_ref: "   ".to_owned(),
            ..current_vs_selected_drawer()
        }),
        Err(M5StateExplanationResolutionError::EmptyDistinctionCopyRef)
    );
}

#[test]
fn blocked_limited_copy_on_current_vs_selected_is_rejected() {
    // current-vs-selected has no blocked or limited side, so it may not use a blocked/limited copy
    // delivery.
    let err = resolve_state_distinction_explanation(&M5StateExplanationInput {
        distinction: M5ConfusableStateDistinction::CurrentVsSelected,
        delivery: M5ExplanationDelivery::BlockedLimitedCopy,
        blocked_limited_copy_ref: "copy:x.y".to_owned(),
        ..locked_vs_disabled_blocked_copy()
    });
    assert_eq!(
        err,
        Err(M5StateExplanationResolutionError::BlockedLimitedCopyOnUnblockableDistinction)
    );
}

#[test]
fn blocked_limited_copy_without_copy_is_rejected() {
    let err = resolve_state_distinction_explanation(&M5StateExplanationInput {
        blocked_limited_copy_ref: "   ".to_owned(),
        ..locked_vs_disabled_blocked_copy()
    });
    assert_eq!(
        err,
        Err(M5StateExplanationResolutionError::BlockedLimitedCopyMissing)
    );
}

#[test]
fn non_blocked_delivery_with_blocked_copy_is_rejected() {
    let err = resolve_state_distinction_explanation(&M5StateExplanationInput {
        blocked_limited_copy_ref: "copy:smuggled".to_owned(),
        ..current_vs_selected_drawer()
    });
    assert_eq!(
        err,
        Err(M5StateExplanationResolutionError::BlockedLimitedCopyOnNonBlockedDelivery)
    );
}

#[test]
fn contradictory_recovery_class_is_rejected() {
    // A named recovery class with no recovery available is a contradiction.
    let err = resolve_state_distinction_explanation(&M5StateExplanationInput {
        recovery_available: false,
        recovery_class: M5RecoveryDisclosureClass::NamesRecoveryAction,
        ..current_vs_selected_drawer()
    });
    assert_eq!(
        err,
        Err(M5StateExplanationResolutionError::RecoveryClassMismatch)
    );
    // `no_recovery_available` while claiming recovery is available is the inverse contradiction.
    let err = resolve_state_distinction_explanation(&M5StateExplanationInput {
        recovery_available: true,
        recovery_class: M5RecoveryDisclosureClass::NoRecoveryAvailable,
        ..current_vs_selected_drawer()
    });
    assert_eq!(
        err,
        Err(M5StateExplanationResolutionError::RecoveryClassMismatch)
    );
}

#[test]
fn no_recovery_available_with_matching_class_resolves() {
    let resolved = resolve_state_distinction_explanation(&M5StateExplanationInput {
        distinction: M5ConfusableStateDistinction::ReadOnlyVsDisabled,
        recovery_available: false,
        recovery_class: M5RecoveryDisclosureClass::NoRecoveryAvailable,
        ..locked_vs_disabled_blocked_copy()
    })
    .expect("resolves");
    assert!(!resolved.recovery_available);
    assert_eq!(
        resolved.recovery_class,
        M5RecoveryDisclosureClass::NoRecoveryAvailable
    );
}

#[test]
fn resolver_rejects_forbidden_material() {
    assert_eq!(
        resolve_state_distinction_explanation(&M5StateExplanationInput {
            distinction_copy_ref: "copy:https://evil.example/x".to_owned(),
            ..current_vs_selected_drawer()
        }),
        Err(M5StateExplanationResolutionError::ForbiddenStateMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_state_explanation_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_STATE_EXPLANATION_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_state_explanation_packet();
    let present: std::collections::BTreeSet<_> = packet.rows.iter().map(|r| r.surface).collect();
    for surface in M5ExplanationConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5ExplanationConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_export_and_labels() {
    let packet = seeded_m5_state_explanation_packet();
    for row in &packet.rows {
        for part in M5ExplanationAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ExplanationExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        for label in M5ComponentStateRequiredLabel::MANDATORY {
            assert!(row.required_labels.contains(&label));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable));
        assert!(row
            .accessibility_routes
            .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded));
        assert!(!row.explanation_examples.is_empty());
    }
}

#[test]
fn every_derived_axis_is_exercised_by_some_example() {
    let packet = seeded_m5_state_explanation_packet();
    let cases: Vec<&M5StateExplanationCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.explanation_examples.iter())
        .collect();

    for distinction in M5ConfusableStateDistinction::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.distinction == distinction),
            "no example exercises distinction {}",
            distinction.as_str()
        );
    }
    for delivery in M5ExplanationDelivery::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.delivery == delivery),
            "no example exercises delivery {}",
            delivery.as_str()
        );
    }
    for cue in M5ExplanationCue::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.required_non_color_cues.contains(&cue)),
            "no example exercises non-color cue {}",
            cue.as_str()
        );
    }
    for trigger in M5StateDisclosureTrigger::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.required_disclosures.contains(&trigger)),
            "no example exercises disclosure {}",
            trigger.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity_and_guarantees() {
    let packet = seeded_m5_state_explanation_packet();
    for row in &packet.rows {
        for case in &row.explanation_examples {
            assert!(
                case.is_self_consistent(),
                "explanation case for {} drifted",
                row.surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "explanation case for {} lost identity",
                row.surface.as_str()
            );
            assert!(
                case.preserves_guarantees(),
                "explanation case for {} lost a guarantee",
                row.surface.as_str()
            );
        }
    }
}

#[test]
fn missing_surface_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet
        .rows
        .retain(|row| row.surface != M5ExplanationConsumerSurface::SettingsRow);
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::RequiredSurfaceMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.vocabulary_set.distinctions.pop();
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ExplanationAnatomyPart::ContrastedStateLabelCue);
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5ExplanationExportField::Distinction);
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::MandatoryExportMissing));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.rows[0]
        .required_labels
        .retain(|l| *l != M5ComponentStateRequiredLabel::KeyboardRoute);
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::MandatoryLabelMissing));
}

#[test]
fn accessibility_route_missing_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.rows[0]
        .accessibility_routes
        .retain(|r| *r != M5ComponentStateAccessibilityRoute::NonColorEncoded);
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::AccessibilityRouteMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.rows[0].explanation_examples[0]
        .resolved
        .carries_blocked_action_help = true;
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::ExampleResolutionDrift));
}

#[test]
fn explanation_example_missing_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.rows[1].explanation_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::ExplanationExampleMissing));
}

#[test]
fn distinction_delivery_cue_and_disclosure_coverage_unproven_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    // Every example a current-vs-selected inline chip → no read-only/locked/pending distinction, no
    // drawer/blocked delivery, no contrasted/blocked/recovery/taxonomy cue, no cause/owner/block/
    // recovery disclosure.
    for row in &mut packet.rows {
        row.explanation_examples =
            vec![M5StateExplanationCase::resolved(M5StateExplanationInput {
                distinction: M5ConfusableStateDistinction::CurrentVsSelected,
                delivery: M5ExplanationDelivery::InlineChip,
                blocked_limited_copy_ref: String::new(),
                ..current_vs_selected_drawer()
            })];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5StateExplanationViolation::DistinctionCoverageUnproven));
    assert!(violations.contains(&M5StateExplanationViolation::DeliveryCoverageUnproven));
    assert!(violations.contains(&M5StateExplanationViolation::CueCoverageUnproven));
    assert!(violations.contains(&M5StateExplanationViolation::DisclosureCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.rows[0].collapses_the_two_states = true;
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::RowInvariantViolated));
}

#[test]
fn stable_surface_missing_proof_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::StableSurfaceMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet
        .governance_review
        .blocked_action_help_aligned_with_component_truth = false;
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.consumer_projection.cue_set_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_state_explanation_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5StateExplanationViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = seeded_m5_state_explanation_packet().render_markdown_summary();
    for surface in M5ExplanationConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing surface {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_surface() {
    let csv = seeded_m5_state_explanation_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ExplanationConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("surface,qualification,owner,"));
    for surface in M5ExplanationConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing surface {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_state_explanation_export()
        .expect("checked M5 state explanation primitive export validates");
    assert_eq!(from_disk.packet_id, M5_STATE_EXPLANATION_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_state_explanation_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_state_explanation_blocked_action_beta_narrowed(),
        seeded_m5_state_explanation_workspace_entry_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5ExplanationConsumerSurface::ALL.len());
    }

    let blocked = seeded_m5_state_explanation_blocked_action_beta_narrowed();
    let row = blocked
        .rows
        .iter()
        .find(|r| r.surface == M5ExplanationConsumerSurface::BlockedActionRow)
        .expect("blocked-action row present");
    assert_eq!(row.qualification, M5ComponentStateQualificationClass::Beta);

    let workspace = seeded_m5_state_explanation_workspace_entry_preview_narrowed();
    let row = workspace
        .rows
        .iter()
        .find(|r| r.surface == M5ExplanationConsumerSurface::WorkspaceEntry)
        .expect("workspace-entry row present");
    assert_eq!(
        row.qualification,
        M5ComponentStateQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let blocked: M5StateExplanationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-state-distinction-explanation-helper-primitive/blocked_action_row_beta_narrowed.json"
    )))
    .expect("blocked-action fixture parses");
    assert!(blocked.validate().is_empty());
    assert_eq!(
        blocked,
        seeded_m5_state_explanation_blocked_action_beta_narrowed()
    );

    let workspace: M5StateExplanationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-state-distinction-explanation-helper-primitive/workspace_entry_preview_narrowed.json"
    )))
    .expect("workspace-entry fixture parses");
    assert!(workspace.validate().is_empty());
    assert_eq!(
        workspace,
        seeded_m5_state_explanation_workspace_entry_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_state_explanation_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
