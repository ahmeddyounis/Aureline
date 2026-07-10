use super::*;

fn assertion_only_card() -> M5SuggestionCardResolutionInput {
    M5SuggestionCardResolutionInput {
        trigger_source: M5GenerationTriggerSource::UncoveredLine,
        target_refs: vec!["src/checkout/cart.rs::apply_discount".to_owned()],
        trigger_context_ref: "uncovered:checkout::apply-discount-line-84".to_owned(),
        assumption_classes: vec![M5GeneratedAssumptionClass::AssertionInferred],
        review_classes: vec![M5GeneratedReviewClass::AssertionChange],
        apply_scope: M5GeneratedApplyScope::AssertionOnly,
        generated_file_count: 1,
        provenance_class: M5TestIntelligenceProvenanceClass::VerifiedCurrentRun,
        offers_sandbox_run: true,
        offers_diff_preview: true,
        offers_rollback: true,
        suggestion_identity_ref: "suggestion-card:review-panel::uncovered-line".to_owned(),
    }
}

// ---- test-generation-suggestion-card resolver ----------------------------

#[test]
fn assertion_only_card_is_apply_capable() {
    let resolved =
        resolve_test_generation_suggestion_card(&assertion_only_card()).expect("resolves");
    assert_eq!(
        resolved.suggestion_posture,
        M5SuggestionPosture::AssertionOnlySuggestion
    );
    assert!(resolved.is_apply_capable);
    assert!(resolved.is_uncovered_path_or_bug);
    assert!(resolved.discloses_assumption_summary);
    assert!(resolved.preserves_preview_and_rollback);
    assert!(!resolved.bundles_non_assertion_churn);
    assert!(resolved
        .available_actions
        .contains(&M5SuggestionCardAction::ApplyReviewedClasses));
    assert!(resolved
        .available_actions
        .contains(&M5SuggestionCardAction::OpenDiffPreview));
    assert!(resolved
        .available_actions
        .contains(&M5SuggestionCardAction::RollbackApplied));
    assert_eq!(
        resolved.suggestion_identity_ref,
        "suggestion-card:review-panel::uncovered-line"
    );
}

#[test]
fn every_apply_scope_has_a_distinct_posture() {
    // The acceptance-criterion axis: a full-bundle proposal never borrows an assertion-only posture.
    let cases = [
        (
            M5GeneratedApplyScope::AssertionOnly,
            M5SuggestionPosture::AssertionOnlySuggestion,
        ),
        (
            M5GeneratedApplyScope::FixtureAndAssertion,
            M5SuggestionPosture::FixtureAndAssertionSuggestion,
        ),
        (
            M5GeneratedApplyScope::SnapshotIncluded,
            M5SuggestionPosture::SnapshotIncludedSuggestion,
        ),
        (
            M5GeneratedApplyScope::FullBundleApply,
            M5SuggestionPosture::FullBundleSuggestion,
        ),
        (
            M5GeneratedApplyScope::ReviewRequired,
            M5SuggestionPosture::ReviewRequiredSuggestion,
        ),
        (
            M5GeneratedApplyScope::ApplyBlocked,
            M5SuggestionPosture::ApplyBlockedSuggestion,
        ),
    ];
    let mut postures = std::collections::BTreeSet::new();
    for (apply_scope, expected) in cases {
        let resolved = resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            apply_scope,
            // Every scope resolves with all three review classes disclosed: apply-capable scopes
            // that name every class, and non-apply-capable scopes that are held to review-first.
            review_classes: M5GeneratedReviewClass::ALL.to_vec(),
            assumption_classes: vec![
                M5GeneratedAssumptionClass::AssertionInferred,
                M5GeneratedAssumptionClass::FixtureAssumed,
                M5GeneratedAssumptionClass::SnapshotGenerated,
            ],
            ..assertion_only_card()
        })
        .map_err(|e| (apply_scope, e));
        match apply_scope {
            // AssertionOnly and FixtureAndAssertion understate the snapshot class, so they fail —
            // exactly the separation guarantee.
            M5GeneratedApplyScope::AssertionOnly | M5GeneratedApplyScope::FixtureAndAssertion => {
                assert_eq!(
                    resolved.unwrap_err().1,
                    M5SuggestionCardResolutionError::ApplyScopeUnderstatesReviewClasses
                );
            }
            _ => {
                let resolved = resolved.expect("resolves");
                assert_eq!(resolved.suggestion_posture, expected);
                assert_eq!(resolved.suggestion_posture.apply_scope(), apply_scope);
                postures.insert(resolved.suggestion_posture);
            }
        }
    }
    // SnapshotIncluded, FullBundle, ReviewRequired, ApplyBlocked resolved distinctly here; the two
    // narrower apply-capable postures are exercised by the seed.
    assert_eq!(postures.len(), 4);
}

#[test]
fn apply_capable_scope_that_understates_churn_fails() {
    // An assertion-only apply that actually touches a snapshot fails — the core acceptance
    // criterion that snapshot churn is never applied through a narrower click.
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            apply_scope: M5GeneratedApplyScope::AssertionOnly,
            review_classes: vec![
                M5GeneratedReviewClass::AssertionChange,
                M5GeneratedReviewClass::SnapshotOrGoldenUpdate,
            ],
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::ApplyScopeUnderstatesReviewClasses)
    );
    // A fixture-and-assertion apply that hides a snapshot also fails.
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            apply_scope: M5GeneratedApplyScope::FixtureAndAssertion,
            review_classes: vec![
                M5GeneratedReviewClass::AssertionChange,
                M5GeneratedReviewClass::SnapshotOrGoldenUpdate,
            ],
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::ApplyScopeUnderstatesReviewClasses)
    );
}

#[test]
fn full_bundle_multi_class_is_held_to_review_first() {
    // A full bundle that mixes all three classes resolves, but is NOT apply-capable — it is held to
    // a review-first path rather than a one-click apply.
    let resolved = resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
        apply_scope: M5GeneratedApplyScope::FullBundleApply,
        review_classes: M5GeneratedReviewClass::ALL.to_vec(),
        assumption_classes: vec![M5GeneratedAssumptionClass::SnapshotGenerated],
        ..assertion_only_card()
    })
    .expect("resolves");
    assert_eq!(
        resolved.suggestion_posture,
        M5SuggestionPosture::FullBundleSuggestion
    );
    assert!(!resolved.is_apply_capable);
    assert!(resolved.needs_attention);
    assert!(resolved.bundles_non_assertion_churn);
    assert!(!resolved
        .available_actions
        .contains(&M5SuggestionCardAction::ApplyReviewedClasses));
}

#[test]
fn apply_capable_generated_card_without_assumptions_fails() {
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            assumption_classes: vec![],
            generated_file_count: 2,
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::GeneratedWithoutAssumptionSummary)
    );
}

#[test]
fn apply_capable_without_preview_or_rollback_fails() {
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            offers_diff_preview: false,
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::ApplyWithoutDiffPreviewOrRollback)
    );
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            offers_rollback: false,
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::ApplyWithoutDiffPreviewOrRollback)
    );
}

#[test]
fn apply_blocked_needs_no_apply_action_and_allows_empty_assumptions() {
    let resolved = resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
        trigger_source: M5GenerationTriggerSource::ManualRequest,
        apply_scope: M5GeneratedApplyScope::ApplyBlocked,
        assumption_classes: vec![],
        generated_file_count: 0,
        offers_sandbox_run: false,
        offers_diff_preview: false,
        offers_rollback: false,
        ..assertion_only_card()
    })
    .expect("resolves");
    assert_eq!(
        resolved.suggestion_posture,
        M5SuggestionPosture::ApplyBlockedSuggestion
    );
    assert!(!resolved.is_apply_capable);
    assert!(!resolved.is_uncovered_path_or_bug);
    assert!(resolved.discloses_assumption_summary);
    assert!(!resolved
        .available_actions
        .contains(&M5SuggestionCardAction::ApplyReviewedClasses));
    assert!(resolved
        .available_actions
        .contains(&M5SuggestionCardAction::ExportSuggestion));
}

#[test]
fn suggestion_resolver_rejects_malformed_input() {
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            suggestion_identity_ref: "  ".to_owned(),
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::EmptySuggestionIdentity)
    );
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            target_refs: vec![],
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::EmptyTargetReference)
    );
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            trigger_context_ref: "".to_owned(),
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::EmptyTriggerContext)
    );
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            review_classes: vec![],
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::MissingReviewClasses)
    );
    assert_eq!(
        resolve_test_generation_suggestion_card(&M5SuggestionCardResolutionInput {
            trigger_context_ref: "bug:https://tracker.example/1188".to_owned(),
            ..assertion_only_card()
        }),
        Err(M5SuggestionCardResolutionError::ForbiddenSuggestionMaterial)
    );
}

// ---- packet --------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_suggestion_card_components_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SUGGESTION_CARD_COMPONENTS_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_consumer_surface() {
    let packet = seeded_m5_suggestion_card_components_packet();
    let present: std::collections::BTreeSet<_> =
        packet.rows.iter().map(|r| r.consumer_surface).collect();
    for surface in M5SuggestionCardConsumerSurface::ALL {
        assert!(
            present.contains(&surface),
            "missing consumer surface {}",
            surface.as_str()
        );
    }
    assert_eq!(
        packet.rows.len(),
        M5SuggestionCardConsumerSurface::ALL.len()
    );
}

#[test]
fn every_row_declares_mandatory_anatomy_and_export() {
    let packet = seeded_m5_suggestion_card_components_packet();
    for row in &packet.rows {
        for part in M5SuggestionCardAnatomyPart::MANDATORY {
            assert!(row.suggestion_anatomy_parts.contains(&part));
        }
        for field in M5SuggestionCardExportField::MANDATORY {
            assert!(row.suggestion_export_fields.contains(&field));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TestIntelligenceAccessibilityRoute::KeyboardFocusable));
        assert!(!row.suggestion_examples.is_empty());
    }
}

#[test]
fn every_derived_state_is_exercised_by_some_example() {
    let packet = seeded_m5_suggestion_card_components_packet();
    let cases: Vec<&M5SuggestionCardResolutionCase> = packet
        .rows
        .iter()
        .flat_map(|row| row.suggestion_examples.iter())
        .collect();

    for posture in M5SuggestionPosture::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.suggestion_posture == posture),
            "no example exercises suggestion posture {}",
            posture.as_str()
        );
    }
    for action in M5SuggestionCardAction::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.available_actions.contains(&action)),
            "no example exercises suggestion action {}",
            action.as_str()
        );
    }
    for class in M5GeneratedReviewClass::ALL {
        assert!(
            cases
                .iter()
                .any(|c| c.resolved.review_classes.contains(&class)),
            "no example exercises review class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_worked_case_is_self_consistent_and_preserves_identity() {
    let packet = seeded_m5_suggestion_card_components_packet();
    for row in &packet.rows {
        for case in &row.suggestion_examples {
            assert!(
                case.is_self_consistent(),
                "suggestion case for {} drifted",
                row.consumer_surface.as_str()
            );
            assert!(
                case.preserves_identity(),
                "suggestion case for {} lost identity",
                row.consumer_surface.as_str()
            );
        }
    }
}

#[test]
fn missing_consumer_surface_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet
        .rows
        .retain(|row| row.consumer_surface != M5SuggestionCardConsumerSurface::TestTreeSuggestion);
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::RequiredConsumerMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.vocabulary_set.suggestion_postures.pop();
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::VocabularySetDrift));
}

#[test]
fn mandatory_anatomy_missing_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.rows[0]
        .suggestion_anatomy_parts
        .retain(|p| *p != M5SuggestionCardAnatomyPart::ReviewClassCue);
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::MandatoryAnatomyMissing));
}

#[test]
fn mandatory_export_missing_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.rows[0]
        .suggestion_export_fields
        .retain(|f| *f != M5SuggestionCardExportField::ReviewClasses);
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::MandatoryExportMissing));
}

#[test]
fn example_resolution_drift_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.rows[0].suggestion_examples[0]
        .resolved
        .is_apply_capable = false;
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::ExampleResolutionDrift));
}

#[test]
fn example_missing_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.rows[1].suggestion_examples.clear();
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::ExampleMissing));
}

#[test]
fn suggestion_posture_coverage_unproven_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    let assertion_only = M5SuggestionCardResolutionCase::resolved(assertion_only_card());
    for row in &mut packet.rows {
        row.suggestion_examples = vec![assertion_only.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::SuggestionPostureCoverageUnproven));
}

#[test]
fn apply_capability_separation_unproven_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    // Replace every example with an apply-capable one so no multi-class review-first case remains.
    let assertion_only = M5SuggestionCardResolutionCase::resolved(assertion_only_card());
    for row in &mut packet.rows {
        row.suggestion_examples = vec![assertion_only.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::ApplyCapabilitySeparationUnproven));
}

#[test]
fn assumption_disclosure_unproven_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    // Replace every example with a non-apply-capable one so no apply-capable disclosed-assumption
    // case remains.
    let blocked = M5SuggestionCardResolutionCase::resolved(M5SuggestionCardResolutionInput {
        trigger_source: M5GenerationTriggerSource::ManualRequest,
        apply_scope: M5GeneratedApplyScope::ApplyBlocked,
        assumption_classes: vec![],
        generated_file_count: 0,
        offers_diff_preview: false,
        offers_rollback: false,
        ..assertion_only_card()
    });
    for row in &mut packet.rows {
        row.suggestion_examples = vec![blocked.clone()];
    }
    let violations = packet.validate();
    assert!(violations.contains(&M5SuggestionCardComponentViolation::AssumptionDisclosureUnproven));
}

#[test]
fn preview_rollback_parity_unproven_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    // Replace every example with a non-apply-capable one so no apply-capable preview+rollback case
    // remains.
    let review_required =
        M5SuggestionCardResolutionCase::resolved(M5SuggestionCardResolutionInput {
            apply_scope: M5GeneratedApplyScope::ReviewRequired,
            offers_diff_preview: false,
            offers_rollback: false,
            ..assertion_only_card()
        });
    for row in &mut packet.rows {
        row.suggestion_examples = vec![review_required.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::PreviewRollbackParityUnproven));
}

#[test]
fn trigger_source_coverage_unproven_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    // Replace every example with an uncovered-line one so the manual request goes uncovered.
    let uncovered = M5SuggestionCardResolutionCase::resolved(assertion_only_card());
    for row in &mut packet.rows {
        row.suggestion_examples = vec![uncovered.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::TriggerSourceCoverageUnproven));
}

#[test]
fn review_class_coverage_unproven_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    // Replace every example with an assertion-only one so fixture / snapshot classes go uncovered.
    let assertion_only = M5SuggestionCardResolutionCase::resolved(assertion_only_card());
    for row in &mut packet.rows {
        row.suggestion_examples = vec![assertion_only.clone()];
    }
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::ReviewClassCoverageUnproven));
}

#[test]
fn row_invariant_violation_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.rows[0].bundles_assumption_fixture_or_snapshot_into_opaque_apply = true;
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::RowInvariantViolated));
}

#[test]
fn stable_consumer_missing_proof_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::StableConsumerMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.governance_review.apply_scope_never_understates_churn = false;
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet
        .consumer_projection
        .ci_and_support_read_same_suggestion_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_suggestion_card_components_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5SuggestionCardComponentViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_consumer_surface() {
    let summary = seeded_m5_suggestion_card_components_packet().render_markdown_summary();
    for surface in M5SuggestionCardConsumerSurface::ALL {
        assert!(
            summary.contains(surface.label()),
            "summary missing consumer {}",
            surface.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_consumer() {
    let csv = seeded_m5_suggestion_card_components_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5SuggestionCardConsumerSurface::ALL.len());
    assert!(lines[0].starts_with("consumer_surface,qualification,owner,"));
    for surface in M5SuggestionCardConsumerSurface::ALL {
        assert!(
            csv.contains(surface.as_str()),
            "csv missing consumer {}",
            surface.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_suggestion_card_components_export()
        .expect("checked M5 suggestion card components export validates");
    assert_eq!(from_disk.packet_id, M5_SUGGESTION_CARD_COMPONENTS_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_suggestion_card_components_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_consumers_visible() {
    for packet in [
        seeded_m5_suggestion_card_components_suggestion_review_panel_preview_narrowed(),
        seeded_m5_suggestion_card_components_editor_suggestion_inline_beta_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.rows.len(),
            M5SuggestionCardConsumerSurface::ALL.len()
        );
    }

    let panel = seeded_m5_suggestion_card_components_suggestion_review_panel_preview_narrowed();
    let row = panel
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5SuggestionCardConsumerSurface::SuggestionReviewPanel)
        .expect("suggestion-review-panel row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Preview
    );

    let editor = seeded_m5_suggestion_card_components_editor_suggestion_inline_beta_narrowed();
    let row = editor
        .rows
        .iter()
        .find(|r| r.consumer_surface == M5SuggestionCardConsumerSurface::EditorSuggestionInline)
        .expect("editor-suggestion-inline row present");
    assert_eq!(
        row.qualification,
        M5TestIntelligenceQualificationClass::Beta
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let panel: M5SuggestionCardComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-test-generation-suggestion-card-primitive/suggestion_review_panel_preview_narrowed.json"
    )))
    .expect("suggestion-review-panel fixture parses");
    assert!(panel.validate().is_empty());
    assert_eq!(
        panel,
        seeded_m5_suggestion_card_components_suggestion_review_panel_preview_narrowed()
    );

    let editor: M5SuggestionCardComponentsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-test-generation-suggestion-card-primitive/editor_suggestion_inline_beta_narrowed.json"
    )))
    .expect("editor-suggestion-inline fixture parses");
    assert!(editor.validate().is_empty());
    assert_eq!(
        editor,
        seeded_m5_suggestion_card_components_editor_suggestion_inline_beta_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_suggestion_card_components_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
