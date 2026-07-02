use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_shell_primitives_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID);
}

#[test]
fn seeded_matrix_names_every_primitive_family() {
    let packet = seeded_m5_shell_primitives_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .primitive_rows
        .iter()
        .map(|r| r.primitive_family)
        .collect();
    for family in M5ShellPrimitiveFamily::ALL {
        assert!(
            present.contains(&family),
            "missing primitive family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.primitive_rows.len(),
        M5ShellPrimitiveFamily::ALL.len()
    );
}

#[test]
fn every_primitive_declares_mandatory_labels_and_a_zone() {
    let packet = seeded_m5_shell_primitives_matrix();
    for row in &packet.primitive_rows {
        for label in M5PrimitiveRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "primitive {} missing mandatory label {}",
                row.primitive_family.as_str(),
                label.as_str()
            );
        }
        assert!(!row.responsive_classes.is_empty());
        assert!(!row.window_classes.is_empty());
        assert!(!row.surface_families.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5AccessibilityRoute::KeyboardFocusable));
    }
}

#[test]
fn family_specific_vocabularies_are_declared() {
    let packet = seeded_m5_shell_primitives_matrix();
    for row in &packet.primitive_rows {
        let family = row.primitive_family;
        if family.is_ambient() {
            assert!(
                !row.status_item_classes.is_empty(),
                "ambient {} declares no status-item classes",
                family.as_str()
            );
            assert!(
                !row.overflow_behaviors.is_empty(),
                "ambient {} declares no overflow behaviors",
                family.as_str()
            );
        }
        if family.is_transient_inspect() {
            assert!(
                !row.representation_classes.is_empty(),
                "transient {} declares no representation classes",
                family.as_str()
            );
        }
        if family.promotes() {
            assert!(
                !row.promotion_states.is_empty(),
                "promoting {} declares no promotion states",
                family.as_str()
            );
        }
        if family.is_pane_control() {
            assert!(
                !row.pane_resize_states.is_empty(),
                "pane control {} declares no pane-resize states",
                family.as_str()
            );
        }
        if family.is_progress() {
            assert!(
                !row.progress_states.is_empty(),
                "progress {} declares no progress states",
                family.as_str()
            );
        }
        if family.carries_freshness() {
            assert!(
                !row.source_freshness_labels.is_empty(),
                "freshness-carrying {} declares no source-freshness labels",
                family.as_str()
            );
        } else {
            assert!(
                row.source_freshness_labels.is_empty(),
                "pure-layout {} should carry no freshness",
                family.as_str()
            );
        }
    }
}

#[test]
fn every_status_item_class_is_declared_by_some_primitive() {
    let packet = seeded_m5_shell_primitives_matrix();
    for class in M5StatusItemClass::ALL {
        assert!(
            packet
                .primitive_rows
                .iter()
                .any(|row| row.status_item_classes.contains(&class)),
            "no primitive declares status-item class {}",
            class.as_str()
        );
    }
}

#[test]
fn every_pane_resize_state_and_progress_state_is_declared() {
    let packet = seeded_m5_shell_primitives_matrix();
    for state in M5PaneResizeState::ALL {
        assert!(
            packet
                .primitive_rows
                .iter()
                .any(|row| row.pane_resize_states.contains(&state)),
            "no primitive declares pane-resize state {}",
            state.as_str()
        );
    }
    for state in M5ProgressState::ALL {
        assert!(
            packet
                .primitive_rows
                .iter()
                .any(|row| row.progress_states.contains(&state)),
            "no primitive declares progress state {}",
            state.as_str()
        );
    }
}

#[test]
fn missing_primitive_family_fails_validation() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet
        .primitive_rows
        .retain(|row| row.primitive_family != M5ShellPrimitiveFamily::SplitterHandle);
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::RequiredPrimitiveMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.vocabulary_set.status_item_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.primitive_rows[0]
        .required_labels
        .retain(|label| *label != M5PrimitiveRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn status_item_class_missing_fails_for_ambient_surface() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::StatusBarItem)
        .expect("status-bar item present");
    row.status_item_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::StatusItemClassMissing));
}

#[test]
fn representation_class_missing_fails_for_hovercard() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::Hovercard)
        .expect("hovercard present");
    row.representation_classes.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::RepresentationClassMissing));
}

#[test]
fn promotion_state_missing_fails_for_pinned_preview() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::PinnedPreviewPromotion)
        .expect("pinned-preview promotion present");
    row.promotion_states.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::PromotionStateMissing));
}

#[test]
fn pane_resize_state_missing_fails_for_splitter() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::SplitterHandle)
        .expect("splitter handle present");
    row.pane_resize_states.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::PaneResizeStateMissing));
}

#[test]
fn progress_state_missing_fails_for_durable_job_row() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::DurableJobRow)
        .expect("durable job row present");
    row.progress_states.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::ProgressStateMissing));
}

#[test]
fn source_freshness_missing_fails_for_progress_indicator() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::ProgressIndicator)
        .expect("progress indicator present");
    row.source_freshness_labels.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::SourceFreshnessMissing));
}

#[test]
fn primitive_invariant_violation_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.primitive_rows[0].reflows_around_vanity_items = true;
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::PrimitiveInvariantViolated));

    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.primitive_rows[0].keeps_critical_truth_hover_only = true;
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::PrimitiveInvariantViolated));

    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.primitive_rows[0].resizable_by_pointer_only = true;
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::PrimitiveInvariantViolated));
}

#[test]
fn stable_primitive_missing_proof_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|row| row.primitive_family == M5ShellPrimitiveFamily::StatusBarItem)
        .expect("status-bar item present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::StablePrimitiveMissingProof));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.primitive_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet
        .governance_review
        .no_status_reflow_around_vanity_items = false;
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet
        .consumer_projection
        .splitter_consumes_resize_state_vocabulary = false;
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_shell_primitives_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5ShellPrimitivesMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_primitive_family() {
    let summary = seeded_m5_shell_primitives_matrix().render_markdown_summary();
    for family in M5ShellPrimitiveFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing primitive {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_primitive() {
    let csv = seeded_m5_shell_primitives_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5ShellPrimitiveFamily::ALL.len());
    assert!(lines[0].starts_with("primitive_family,qualification,owner,"));
    for family in M5ShellPrimitiveFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing primitive {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_shell_primitives_matrix_export()
        .expect("checked M5 shell primitives matrix export validates");
    assert_eq!(packet.packet_id, M5_SHELL_PRIMITIVES_MATRIX_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_shell_primitives_matrix_export()
        .expect("checked M5 shell primitives matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_shell_primitives_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_primitives_visible() {
    for packet in [
        seeded_m5_shell_primitives_matrix_pane_resize_preset_beta_narrowed(),
        seeded_m5_shell_primitives_matrix_pinned_preview_promotion_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.primitive_rows.len(),
            M5ShellPrimitiveFamily::ALL.len()
        );
    }

    let preset = seeded_m5_shell_primitives_matrix_pane_resize_preset_beta_narrowed();
    let row = preset
        .primitive_rows
        .iter()
        .find(|r| r.primitive_family == M5ShellPrimitiveFamily::PaneResizePreset)
        .expect("pane-resize-preset row present");
    assert_eq!(row.qualification, M5PrimitiveQualificationClass::Beta);

    let pinned = seeded_m5_shell_primitives_matrix_pinned_preview_promotion_preview_narrowed();
    let row = pinned
        .primitive_rows
        .iter()
        .find(|r| r.primitive_family == M5ShellPrimitiveFamily::PinnedPreviewPromotion)
        .expect("pinned-preview-promotion row present");
    assert_eq!(row.qualification, M5PrimitiveQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let preset: M5ShellPrimitivesMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-primitives/pane_resize_preset_beta_narrowed.json"
    )))
    .expect("preset fixture parses");
    assert!(preset.validate().is_empty());
    assert_eq!(
        preset,
        seeded_m5_shell_primitives_matrix_pane_resize_preset_beta_narrowed()
    );

    let pinned: M5ShellPrimitivesMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-shell-primitives/pinned_preview_promotion_preview_narrowed.json"
    )))
    .expect("pinned fixture parses");
    assert!(pinned.validate().is_empty());
    assert_eq!(
        pinned,
        seeded_m5_shell_primitives_matrix_pinned_preview_promotion_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_shell_primitives_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
