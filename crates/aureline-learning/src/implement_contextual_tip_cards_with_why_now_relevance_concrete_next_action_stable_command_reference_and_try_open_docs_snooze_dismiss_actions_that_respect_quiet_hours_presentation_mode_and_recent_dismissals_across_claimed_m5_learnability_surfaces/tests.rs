use super::*;

fn actionable_bound_tip() -> M5ContextualTipCardResolutionInput {
    M5ContextualTipCardResolutionInput {
        trigger_class: M5TipTriggerClass::FirstEncounter,
        command_backing: M5CommandBackingState::BoundCommand,
        dismissal_state: M5TipDismissalState::Dismissible,
        quiet_hours_active: false,
        presentation_mode_active: false,
        recently_dismissed: false,
        underlying_action_requires_approval: false,
        why_now_relevance: "You just opened your first project — open the command palette"
            .to_owned(),
        next_action_command_ref: "command:command-palette.open".to_owned(),
        tip_identity_ref: "tip:onboarding:command-palette".to_owned(),
    }
}

// ---- contextual-tip-card resolver ---------------------------------------

#[test]
fn dismissible_bound_tip_is_delivered_actionable_with_try() {
    let resolved = resolve_contextual_tip_card(&actionable_bound_tip()).expect("resolves");
    assert_eq!(
        resolved.delivery_posture,
        M5ContextualTipDeliveryPosture::DeliveredActionable
    );
    assert!(resolved.is_command_backed);
    assert!(resolved.is_delivered);
    assert!(!resolved.requires_approval_before_try);
    assert!(resolved.teaches_in_place);
    assert!(!resolved.hijacks_workflow);
    assert!(resolved.respects_quiet_hours);
    assert!(resolved.respects_presentation_mode);
    assert!(resolved.respects_recent_dismissals);
    assert!(resolved.is_reversible);
    assert!(resolved.honors_underlying_trust_limits);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5ContextualTipAction::TryNextAction,
            M5ContextualTipAction::OpenDocs,
            M5ContextualTipAction::SnoozeTip,
            M5ContextualTipAction::DismissTip,
        ]
    );
    assert_eq!(resolved.tip_identity_ref, "tip:onboarding:command-palette");
}

#[test]
fn approval_required_tip_offers_request_approval_not_try() {
    let resolved = resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
        underlying_action_requires_approval: true,
        ..actionable_bound_tip()
    })
    .expect("resolves");
    assert_eq!(
        resolved.delivery_posture,
        M5ContextualTipDeliveryPosture::DeliveredActionable
    );
    assert!(resolved
        .available_actions
        .contains(&M5ContextualTipAction::RequestApproval));
    assert!(!resolved
        .available_actions
        .contains(&M5ContextualTipAction::TryNextAction));
    assert!(resolved.honors_underlying_trust_limits);
    assert!(resolved.requires_approval_before_try);
}

#[test]
fn no_command_backing_is_delivered_informational_without_try() {
    let resolved = resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
        command_backing: M5CommandBackingState::NoCommandBacking,
        ..actionable_bound_tip()
    })
    .expect("resolves");
    assert_eq!(
        resolved.delivery_posture,
        M5ContextualTipDeliveryPosture::DeliveredInformational
    );
    assert!(!resolved.is_command_backed);
    assert!(resolved.is_delivered);
    assert_eq!(
        resolved.available_actions,
        vec![
            M5ContextualTipAction::OpenDocs,
            M5ContextualTipAction::SnoozeTip,
            M5ContextualTipAction::DismissTip,
        ]
    );
}

#[test]
fn quiet_hours_withhold_before_everything_else() {
    // Quiet hours dominate presentation mode and dismissal state alike.
    let resolved = resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
        quiet_hours_active: true,
        presentation_mode_active: true,
        dismissal_state: M5TipDismissalState::Snoozed,
        ..actionable_bound_tip()
    })
    .expect("resolves");
    assert_eq!(
        resolved.delivery_posture,
        M5ContextualTipDeliveryPosture::WithheldForQuietHours
    );
    assert!(!resolved.is_delivered);
    assert!(resolved.available_actions.is_empty());
    assert!(resolved.respects_quiet_hours);
}

#[test]
fn presentation_mode_withholds_when_quiet_hours_off() {
    let resolved = resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
        presentation_mode_active: true,
        ..actionable_bound_tip()
    })
    .expect("resolves");
    assert_eq!(
        resolved.delivery_posture,
        M5ContextualTipDeliveryPosture::WithheldForPresentationMode
    );
    assert!(!resolved.is_delivered);
    assert!(resolved.available_actions.is_empty());
}

#[test]
fn recent_dismissal_and_resolved_states_are_withheld_already_resolved() {
    // A like tip recently dismissed is withheld even when dismissal_state is dismissible.
    let recent = resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
        recently_dismissed: true,
        ..actionable_bound_tip()
    })
    .expect("resolves");
    assert_eq!(
        recent.delivery_posture,
        M5ContextualTipDeliveryPosture::WithheldAlreadyResolved
    );

    for state in [
        M5TipDismissalState::Dismissed,
        M5TipDismissalState::AutoExpired,
        M5TipDismissalState::SuppressedByPreference,
    ] {
        let resolved = resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
            dismissal_state: state,
            ..actionable_bound_tip()
        })
        .expect("resolves");
        assert_eq!(
            resolved.delivery_posture,
            M5ContextualTipDeliveryPosture::WithheldAlreadyResolved,
            "dismissal state {} was not withheld-already-resolved",
            state.as_str()
        );
        assert!(resolved.available_actions.is_empty());
    }
}

#[test]
fn snoozed_tip_offers_only_dismiss() {
    let resolved = resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
        dismissal_state: M5TipDismissalState::Snoozed,
        ..actionable_bound_tip()
    })
    .expect("resolves");
    assert_eq!(
        resolved.delivery_posture,
        M5ContextualTipDeliveryPosture::SnoozedForLater
    );
    assert_eq!(
        resolved.available_actions,
        vec![M5ContextualTipAction::DismissTip]
    );
    assert!(resolved.is_reversible);
}

#[test]
fn persistent_until_acted_is_delivered() {
    let resolved = resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
        dismissal_state: M5TipDismissalState::PersistentUntilActed,
        ..actionable_bound_tip()
    })
    .expect("resolves");
    assert_eq!(
        resolved.delivery_posture,
        M5ContextualTipDeliveryPosture::DeliveredActionable
    );
}

#[test]
fn resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
            why_now_relevance: " ".to_owned(),
            ..actionable_bound_tip()
        }),
        Err(M5ContextualTipCardResolutionError::EmptyWhyNowRelevance)
    );
    assert_eq!(
        resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
            next_action_command_ref: "".to_owned(),
            ..actionable_bound_tip()
        }),
        Err(M5ContextualTipCardResolutionError::EmptyNextActionCommandRef)
    );
    assert_eq!(
        resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
            tip_identity_ref: "".to_owned(),
            ..actionable_bound_tip()
        }),
        Err(M5ContextualTipCardResolutionError::EmptyTipIdentity)
    );
    assert_eq!(
        resolve_contextual_tip_card(&M5ContextualTipCardResolutionInput {
            next_action_command_ref: "command:https://evil.example/x".to_owned(),
            ..actionable_bound_tip()
        }),
        Err(M5ContextualTipCardResolutionError::ForbiddenTipMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_contextual_tip_card_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_CONTEXTUAL_TIP_CARD_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_contextual_tip_card_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5ContextualTipConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(packet.rows.len(), M5ContextualTipConsumerSurface::ALL.len());
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_contextual_tip_card_packet();
    for row in &packet.rows {
        for part in M5ContextualTipAnatomyPart::MANDATORY {
            assert!(row.anatomy_parts.contains(&part));
        }
        for field in M5ContextualTipExportField::MANDATORY {
            assert!(row.export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable));
        assert!(!row.tip_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_contextual_tip_card_packet();
    let cases: Vec<&M5ContextualTipCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.tip_examples.iter())
        .collect();

    for posture in M5ContextualTipDeliveryPosture::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.delivery_posture == posture),
            "no example exercises delivery posture {}",
            posture.as_str()
        );
    }
    for action in M5ContextualTipAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises action {}",
            action.as_str()
        );
    }
    for trigger in M5TipTriggerClass::ALL {
        assert!(
            cases.iter().any(|c| c.resolved.trigger_class == trigger),
            "no example exercises trigger class {}",
            trigger.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity_and_reversibility() {
    let packet = seeded_m5_contextual_tip_card_packet();
    for row in &packet.rows {
        for case in &row.tip_examples {
            assert!(
                case.is_self_consistent(),
                "tip case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "tip case for {} lost identity",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_reversibility(),
                "tip case for {} lost reversibility",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5ContextualTipConsumerSurface::CommandPaletteHint);
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.vocabulary_set.delivery_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.rows[0]
        .anatomy_parts
        .retain(|p| *p != M5ContextualTipAnatomyPart::StableCommandReferenceCue);
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.rows[0]
        .export_fields
        .retain(|f| *f != M5ContextualTipExportField::DeliveryPosture);
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.rows[0].tip_examples[0].resolved.is_delivered = false;
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::ExampleResolutionDrift));
}

#[test]
fn tip_example_missing_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.rows[1].tip_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::TipExampleMissing));
}

#[test]
fn trigger_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    for row in &mut packet.rows {
        row.tip_examples = vec![M5ContextualTipCardResolutionCase::resolved(
            actionable_bound_tip(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::TriggerClassCoverageUnproven));
}

#[test]
fn delivery_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    // Every example delivered-actionable → no informational, snoozed, or withheld posture.
    for row in &mut packet.rows {
        row.tip_examples = vec![M5ContextualTipCardResolutionCase::resolved(
            actionable_bound_tip(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::DeliveryPostureCoverageUnproven));
}

#[test]
fn action_coverage_unproven_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    // Every example delivered-actionable-with-try → no request-approval and no snoozed dismiss.
    for row in &mut packet.rows {
        row.tip_examples = vec![M5ContextualTipCardResolutionCase::resolved(
            actionable_bound_tip(),
        )];
    }
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::ActionCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.rows[0].hijacks_workflow_as_blocking_tour = true;
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.governance_review.tips_respect_quiet_hours = false;
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.consumer_projection.action_set_reads_single_source = false;
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_contextual_tip_card_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ContextualTipCardViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_contextual_tip_card_packet().render_markdown_summary();
    for surface in M5ContextualTipConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_contextual_tip_card_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ContextualTipConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5ContextualTipConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_contextual_tip_card_export()
        .expect("checked M5 contextual tip card primitive export validates");
    assert_eq!(from_disk.packet_id, M5_CONTEXTUAL_TIP_CARD_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_contextual_tip_card_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_contextual_tip_card_command_palette_hint_beta_narrowed(),
        seeded_m5_contextual_tip_card_support_tip_export_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(packet.rows.len(), M5ContextualTipConsumerSurface::ALL.len());
    }

    let palette = seeded_m5_contextual_tip_card_command_palette_hint_beta_narrowed();
    let row = palette
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5ContextualTipConsumerSurface::CommandPaletteHint)
        .expect("command-palette-hint row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Beta);

    let support = seeded_m5_contextual_tip_card_support_tip_export_preview_narrowed();
    let row = support
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5ContextualTipConsumerSurface::SupportTipExport)
        .expect("support-tip-export row present");
    assert_eq!(row.qualification, M5TeachingQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let palette: M5ContextualTipCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-contextual-tip-card-primitive/command_palette_hint_beta_narrowed.json"
    )))
    .expect("command-palette-hint fixture parses");
    assert!(palette.validate().is_empty());
    assert_eq!(
        palette,
        seeded_m5_contextual_tip_card_command_palette_hint_beta_narrowed()
    );

    let support: M5ContextualTipCardPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-contextual-tip-card-primitive/support_tip_export_preview_narrowed.json"
    )))
    .expect("support-tip-export fixture parses");
    assert!(support.validate().is_empty());
    assert_eq!(
        support,
        seeded_m5_contextual_tip_card_support_tip_export_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_contextual_tip_card_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
