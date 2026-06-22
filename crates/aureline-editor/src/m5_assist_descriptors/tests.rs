//! Unit tests for the canonical assist-descriptor model.

use super::*;

#[test]
fn model_builds_and_every_invariant_holds() {
    let model = assist_descriptor_model();
    assert_eq!(model.record_kind, M5_ASSIST_DESCRIPTORS_RECORD_KIND);
    assert_eq!(model.schema_ref, M5_ASSIST_DESCRIPTORS_SCHEMA_REF);
    assert_eq!(model.model_id, M5_ASSIST_DESCRIPTORS_MODEL_ID);
    assert!(
        model.all_invariants_hold(),
        "every frozen invariant must hold: {:?}",
        model
            .invariants
            .iter()
            .filter(|invariant| !invariant.holds)
            .map(|invariant| &invariant.invariant_id)
            .collect::<Vec<_>>()
    );
    assert!(model.is_support_export_safe());
    assert!(model.raw_payload_excluded);
}

#[test]
fn model_serialization_round_trips() {
    let model = assist_descriptor_model();
    let json = serde_json::to_string(&model).expect("model serializes");
    let restored: AssistDescriptorModel = serde_json::from_str(&json).expect("model round-trips");
    assert_eq!(model, restored);
}

#[test]
fn catalog_holds_one_descriptor_per_class() {
    let model = assist_descriptor_model();
    let expected =
        DecorationClass::ALL.len() + CodeLensClass::ALL.len() + InlayHintClass::ALL.len();
    assert_eq!(model.descriptor_catalog.len(), expected);
    for class in DecorationClass::ALL {
        assert!(
            model
                .descriptor(&format!("decoration:{}", class.as_str()))
                .is_some(),
            "missing decoration {}",
            class.as_str()
        );
    }
    for class in CodeLensClass::ALL {
        assert!(
            model
                .descriptor(&format!("hint:code-lens:{}", class.as_str()))
                .is_some(),
            "missing code lens {}",
            class.as_str()
        );
    }
    for class in InlayHintClass::ALL {
        assert!(
            model
                .descriptor(&format!("hint:inlay:{}", class.as_str()))
                .is_some(),
            "missing inlay hint {}",
            class.as_str()
        );
    }
}

#[test]
fn decorations_are_editing_truth_and_convenience_is_not() {
    let model = assist_descriptor_model();
    for descriptor in &model.descriptor_catalog {
        match descriptor.family {
            AssistDescriptorFamily::Decoration => {
                assert_eq!(
                    descriptor.truth_tier,
                    TruthTier::EditingTruth,
                    "{} must be editing truth",
                    descriptor.descriptor_id
                );
                assert!(
                    !descriptor.layout_shifting,
                    "{} must not shift layout",
                    descriptor.descriptor_id
                );
            }
            AssistDescriptorFamily::CodeLens | AssistDescriptorFamily::InlayHint => {
                assert_eq!(
                    descriptor.truth_tier,
                    TruthTier::ConvenienceMetadata,
                    "{} must be convenience metadata",
                    descriptor.descriptor_id
                );
                assert!(
                    descriptor.layout_shifting,
                    "{} must shift layout",
                    descriptor.descriptor_id
                );
            }
        }
    }
}

#[test]
fn every_actionable_or_severity_decoration_is_accessible() {
    let model = assist_descriptor_model();
    for descriptor in &model.descriptor_catalog {
        if descriptor.family != AssistDescriptorFamily::Decoration {
            continue;
        }
        if !descriptor.actionability.requires_keyboard_path() {
            continue;
        }
        assert!(
            descriptor.accessibility.keyboard_path.is_some(),
            "{} needs a keyboard path",
            descriptor.descriptor_id
        );
        assert!(!descriptor.accessibility.screen_reader_label.is_empty());
        assert!(!descriptor.accessibility.non_color_differentiator.is_empty());
    }
}

#[test]
fn ai_descriptors_are_labeled_and_distinct() {
    let model = assist_descriptor_model();
    let ai_lens = model
        .descriptor("hint:code-lens:ai_explain_action")
        .expect("ai explain lens");
    let ai_hint = model
        .descriptor("hint:inlay:ai_inferred")
        .expect("ai inferred hint");
    for descriptor in [ai_lens, ai_hint] {
        assert!(descriptor.source.requires_ai_label);
        assert_eq!(
            descriptor.source.source_label_class,
            AssistSourceLabelClass::AiInlineAssist
        );
        assert!(descriptor.source.requires_visual_distinction);
    }
}

#[test]
fn code_file_comfortable_renders_truth_and_suppresses_only_low_confidence() {
    let model = assist_descriptor_model();
    let scenario = model
        .scenario(SCENARIO_CODE_FILE_COMFORTABLE)
        .expect("comfortable scenario");
    // All decorations render.
    for resolved in scenario
        .resolved
        .iter()
        .filter(|r| r.truth_tier == TruthTier::EditingTruth)
    {
        assert_eq!(
            resolved.visibility,
            VisibilityVerdict::Rendered,
            "{} must render on a code file",
            resolved.descriptor_id
        );
    }
    // The AI-inferred hint is the only low-confidence suppression.
    let ai_hint = scenario
        .resolved("hint:inlay:ai_inferred")
        .expect("ai hint");
    assert_eq!(ai_hint.visibility, VisibilityVerdict::Suppressed);
    assert_eq!(ai_hint.suppression_reason, SuppressionReason::LowConfidence);
    // A high-confidence lens renders.
    let refs = scenario
        .resolved("hint:code-lens:reference_count")
        .expect("reference lens");
    assert_eq!(refs.visibility, VisibilityVerdict::Rendered);
}

#[test]
fn dense_compacts_optional_metadata_only() {
    let model = assist_descriptor_model();
    let scenario = model.scenario(SCENARIO_CODE_FILE_DENSE).expect("dense");
    let chained = scenario
        .resolved("hint:inlay:chained_call_type")
        .expect("chained call hint");
    assert_eq!(chained.visibility, VisibilityVerdict::Suppressed);
    assert_eq!(
        chained.suppression_reason,
        SuppressionReason::DensityCompaction
    );
    // Parameter-name hints are not density-optional and stay rendered.
    let param = scenario
        .resolved("hint:inlay:parameter_name")
        .expect("parameter hint");
    assert_eq!(param.visibility, VisibilityVerdict::Rendered);
    // Authorship lens is density-optional.
    let authorship = scenario
        .resolved("hint:code-lens:vcs_authorship")
        .expect("authorship lens");
    assert_eq!(
        authorship.suppression_reason,
        SuppressionReason::DensityCompaction
    );
}

#[test]
fn high_zoom_drops_inline_type_hints() {
    let model = assist_descriptor_model();
    let scenario = model.scenario(SCENARIO_CODE_FILE_HIGH_ZOOM).expect("zoom");
    let inferred = scenario
        .resolved("hint:inlay:inferred_type")
        .expect("inferred type hint");
    assert_eq!(inferred.visibility, VisibilityVerdict::Suppressed);
    assert_eq!(
        inferred.suppression_reason,
        SuppressionReason::HighZoomHorizontalBudget
    );
    // Parameter-name hints are not zoom-optional.
    let param = scenario
        .resolved("hint:inlay:parameter_name")
        .expect("parameter hint");
    assert_eq!(param.visibility, VisibilityVerdict::Rendered);
}

#[test]
fn typing_defers_all_convenience_and_keeps_decorations() {
    let model = assist_descriptor_model();
    let scenario = model.scenario(SCENARIO_CODE_FILE_TYPING).expect("typing");
    for resolved in &scenario.resolved {
        match resolved.truth_tier {
            TruthTier::EditingTruth => {
                assert_eq!(
                    resolved.visibility,
                    VisibilityVerdict::Rendered,
                    "{} must keep drawing while typing",
                    resolved.descriptor_id
                );
            }
            TruthTier::ConvenienceMetadata => {
                assert!(
                    matches!(
                        resolved.visibility,
                        VisibilityVerdict::Deferred | VisibilityVerdict::Suppressed
                    ),
                    "{} must not render while typing",
                    resolved.descriptor_id
                );
            }
        }
    }
    // The high-confidence reference lens is held, not dropped.
    let refs = scenario
        .resolved("hint:code-lens:reference_count")
        .expect("reference lens");
    assert_eq!(refs.visibility, VisibilityVerdict::Deferred);
    assert_eq!(refs.suppression_reason, SuppressionReason::TypingBudget);
}

#[test]
fn large_file_suppresses_convenience_reduces_decorations() {
    let model = assist_descriptor_model();
    let scenario = model.scenario(SCENARIO_LARGE_FILE).expect("large file");
    for resolved in &scenario.resolved {
        match resolved.truth_tier {
            TruthTier::ConvenienceMetadata => {
                assert_eq!(resolved.visibility, VisibilityVerdict::Suppressed);
                assert_eq!(
                    resolved.suppression_reason,
                    SuppressionReason::LargeFileRestricted
                );
            }
            TruthTier::EditingTruth => {
                assert_eq!(resolved.visibility, VisibilityVerdict::Downgraded);
                assert_eq!(
                    resolved.suppression_reason,
                    SuppressionReason::ReducedDecoration
                );
            }
        }
    }
}

#[test]
fn partial_index_pends_semantic_descriptors() {
    let model = assist_descriptor_model();
    let scenario = model
        .scenario(SCENARIO_PARTIAL_INDEX)
        .expect("partial index");
    let refs = scenario
        .resolved("hint:code-lens:reference_count")
        .expect("reference lens");
    assert_eq!(refs.visibility, VisibilityVerdict::Downgraded);
    assert_eq!(
        refs.suppression_reason,
        SuppressionReason::PartialIndexPending
    );
    // Decorations stay full on a partial index.
    let diag = scenario
        .resolved("decoration:diagnostic_underline")
        .expect("diagnostic");
    assert_eq!(diag.visibility, VisibilityVerdict::Rendered);
}

#[test]
fn docs_code_block_marks_lenses_and_hints_unavailable() {
    let model = assist_descriptor_model();
    let scenario = model.scenario(SCENARIO_DOCS_CODE_BLOCK).expect("docs code");
    for id in [
        "hint:code-lens:reference_count",
        "hint:inlay:parameter_name",
    ] {
        let resolved = scenario.resolved(id).expect("resolved");
        assert_eq!(resolved.visibility, VisibilityVerdict::Suppressed);
        assert_eq!(
            resolved.suppression_reason,
            SuppressionReason::UnavailableOnSurface
        );
        assert!(!resolved.keyboard_reachable);
    }
}

#[test]
fn sql_editor_labels_lenses_and_hints_fallback() {
    let model = assist_descriptor_model();
    let scenario = model.scenario(SCENARIO_SQL_EDITOR).expect("sql");
    let refs = scenario
        .resolved("hint:code-lens:reference_count")
        .expect("reference lens");
    assert_eq!(refs.visibility, VisibilityVerdict::Downgraded);
    assert_eq!(refs.suppression_reason, SuppressionReason::SourceFallback);
}

#[test]
fn reduced_motion_disables_animations_everywhere_in_scenario() {
    let model = assist_descriptor_model();
    let scenario = model
        .scenario(SCENARIO_CODE_FILE_REDUCED_MOTION)
        .expect("reduced motion");
    for resolved in &scenario.resolved {
        assert!(
            !resolved.animations_enabled,
            "{} must not animate under reduced motion",
            resolved.descriptor_id
        );
    }
    // The same descriptor animates when motion is allowed.
    let comfortable = model
        .scenario(SCENARIO_CODE_FILE_COMFORTABLE)
        .expect("comfortable");
    let debug_line = comfortable
        .resolved("decoration:debug_current_line")
        .expect("debug line");
    assert!(debug_line.animations_enabled);
}

#[test]
fn precedence_conflicts_are_won_by_editing_truth() {
    let model = assist_descriptor_model();
    assert!(!model.precedence_conflicts.is_empty());
    for case in &model.precedence_conflicts {
        assert_eq!(case.winner_descriptor_id, case.editing_truth_descriptor_id);
        assert_eq!(case.yielded_descriptor_id, case.convenience_descriptor_id);
        assert_eq!(case.yielded_visibility, VisibilityVerdict::Deferred);
        assert_eq!(
            case.yielded_reason,
            SuppressionReason::OutrankedByEditingTruth
        );
        assert!(case.shared_anchor.overlaps(&case.shared_anchor));
    }
}

#[test]
fn keyboard_reachability_tracks_offered_state() {
    let model = assist_descriptor_model();
    for scenario in &model.scenarios {
        for resolved in &scenario.resolved {
            assert_eq!(
                resolved.keyboard_reachable,
                resolved.visibility.is_offered(),
                "{}::{} reachability must track offered state",
                scenario.context.scenario_id,
                resolved.descriptor_id
            );
        }
    }
}

#[test]
fn lens_and_hint_ids_reuse_the_frozen_hint_prefix() {
    let model = assist_descriptor_model();
    let prefix = MicroSurfaceKind::HintDescriptor.id_prefix();
    for descriptor in &model.descriptor_catalog {
        if matches!(
            descriptor.family,
            AssistDescriptorFamily::CodeLens | AssistDescriptorFamily::InlayHint
        ) {
            assert!(
                descriptor.descriptor_id.starts_with(prefix),
                "{} must reuse the frozen hint prefix {prefix}",
                descriptor.descriptor_id
            );
        }
    }
}

#[test]
fn lines_projection_renders_every_section() {
    let model = assist_descriptor_model();
    let lines = assist_descriptor_model_lines(&model);
    assert!(lines
        .iter()
        .any(|line| line.contains("Assist-descriptor model")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Descriptor catalog:")));
    assert!(lines.iter().any(|line| line.contains("Scenarios:")));
    assert!(lines
        .iter()
        .any(|line| line.contains("Precedence conflicts:")));
    assert!(lines.iter().any(|line| line.contains("Invariants:")));
    for scenario in &model.scenarios {
        assert!(
            lines
                .iter()
                .any(|line| line.contains(&scenario.context.scenario_id)),
            "lines must mention scenario {}",
            scenario.context.scenario_id
        );
    }
}

#[test]
fn class_catalogs_have_unique_tokens() {
    let model = assist_descriptor_model();
    for catalog in [
        &model.descriptor_families,
        &model.placement_classes,
        &model.actionability_classes,
        &model.confidence_classes,
        &model.freshness_classes,
        &model.motion_classes,
        &model.density_tiers,
        &model.zoom_tiers,
        &model.visibility_verdicts,
        &model.suppression_reasons,
    ] {
        let mut tokens: Vec<&str> = catalog
            .iter()
            .map(|descriptor| descriptor.class_token.as_str())
            .collect();
        let total = tokens.len();
        tokens.sort_unstable();
        tokens.dedup();
        assert_eq!(tokens.len(), total, "catalog tokens must be unique");
    }
}
