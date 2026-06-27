use super::*;

#[test]
fn seeded_catalog_validates() {
    let packet = seeded_m5_nonvisual_summary_catalog();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_NONVISUAL_SUMMARY_CATALOG_PACKET_ID);
}

#[test]
fn seeded_catalog_covers_every_surface_kind() {
    let packet = seeded_m5_nonvisual_summary_catalog();
    let present: std::collections::BTreeSet<_> =
        packet.summaries.iter().map(|s| s.surface_kind).collect();
    for kind in M5SummarySurfaceKind::ALL {
        assert!(
            present.contains(&kind),
            "missing surface kind {}",
            kind.as_str()
        );
    }
}

#[test]
fn every_summary_is_actionable_and_object_linked() {
    let packet = seeded_m5_nonvisual_summary_catalog();
    for summary in &packet.summaries {
        // The guardrail: detailed drill-down navigation, not a vague one-liner.
        assert!(
            summary.drilldowns.len() >= M5_SUMMARY_MIN_DRILLDOWNS,
            "summary {} has too few drill-downs",
            summary.summary_id
        );
        assert!(
            !summary.object_identity_ref.trim().is_empty(),
            "summary {} is not object-linked",
            summary.summary_id
        );
        assert!(
            !summary.structure.dimensions.is_empty(),
            "summary {} has an unquantified structure",
            summary.summary_id
        );
        for drilldown in &summary.drilldowns {
            assert!(
                drilldown.keyboard_reachable,
                "drill-down {} is not keyboard reachable",
                drilldown.drilldown_id
            );
            assert!(
                !drilldown.target_identity_ref.trim().is_empty(),
                "drill-down {} is not object-linked",
                drilldown.drilldown_id
            );
            assert!(
                drilldown
                    .route_message_id
                    .starts_with(M5_SUMMARY_MESSAGE_ID_PREFIX),
                "drill-down {} route id missing prefix",
                drilldown.drilldown_id
            );
        }
    }
}

#[test]
fn visual_surfaces_provide_text_alternatives_and_text_native_do_not() {
    let packet = seeded_m5_nonvisual_summary_catalog();
    for summary in &packet.summaries {
        let alt = &summary.text_alternative;
        if summary.surface_kind.requires_text_alternative() {
            assert!(
                alt.provided
                    && alt.kind.is_applicable()
                    && !alt.alt_text_message_id.is_empty()
                    && !alt.export_metadata_fields.is_empty(),
                "visual surface {} lacks a text alternative",
                summary.summary_id
            );
        } else {
            assert!(
                !alt.provided
                    && !alt.kind.is_applicable()
                    && alt.alt_text_message_id.is_empty()
                    && alt.export_metadata_fields.is_empty(),
                "text-native surface {} should declare no alternative",
                summary.summary_id
            );
        }
    }
}

#[test]
fn every_provisional_presentation_state_is_exercised() {
    let packet = seeded_m5_nonvisual_summary_catalog();
    let present: std::collections::BTreeSet<_> = packet
        .summaries
        .iter()
        .map(|s| s.presentation_state)
        .collect();
    for state in M5SummaryPresentationState::ALL {
        if state.is_provisional() {
            assert!(
                present.contains(&state),
                "provisional state {} is never exercised",
                state.as_str()
            );
        }
    }
}

#[test]
fn missing_surface_kind_fails_validation() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet
        .summaries
        .retain(|s| s.surface_kind != M5SummarySurfaceKind::ArtifactViewer);
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::RequiredSurfaceKindMissing));
}

#[test]
fn shared_vocabulary_drift_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.shared_vocabulary_set.semantic_role_classes.pop();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::VocabularySetDrift));
}

#[test]
fn summary_vocabulary_drift_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summary_vocabulary_set.surface_kinds.pop();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::VocabularySetDrift));
}

#[test]
fn duplicate_summary_id_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    let mut clone = packet.summaries[0].clone();
    // Keep the surface kind distinct so the duplicate-id check is what fires.
    clone.surface_kind = M5SummarySurfaceKind::ArtifactViewer;
    packet.summaries.push(clone);
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::DuplicateSummaryId));
}

#[test]
fn duplicate_surface_kind_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    let mut clone = packet.summaries[0].clone();
    clone.summary_id = "summary:custom-editor-dup".to_owned();
    for drilldown in &mut clone.drilldowns {
        drilldown.drilldown_id = format!("{}-dup", drilldown.drilldown_id);
    }
    packet.summaries.push(clone);
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::DuplicateSurfaceKind));
}

#[test]
fn duplicate_drilldown_id_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    let dup = packet.summaries[0].drilldowns[0].clone();
    packet.summaries[1].drilldowns.push(dup);
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::DuplicateDrillDownId));
}

#[test]
fn too_few_drilldowns_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].drilldowns.truncate(1);
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::SummaryNotActionable));
}

#[test]
fn missing_object_identity_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].object_identity_ref = "  ".to_owned();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::MissingObjectIdentity));
}

#[test]
fn unquantified_structure_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].structure.dimensions.clear();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::StructureIncomplete));
}

#[test]
fn structure_prefix_missing_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].structure.structure_message_id = "editor.structure".to_owned();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::MessageIdPrefixMissing));
}

#[test]
fn drilldown_prefix_missing_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].drilldowns[0].route_message_id = "editor.enumerate".to_owned();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::MessageIdPrefixMissing));
}

#[test]
fn non_keyboard_reachable_drilldown_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].drilldowns[0].keyboard_reachable = false;
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::DrillDownNotKeyboardReachable));
}

#[test]
fn visual_surface_without_text_alternative_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    let chart = packet
        .summaries
        .iter_mut()
        .find(|s| s.surface_kind == M5SummarySurfaceKind::Chart)
        .expect("chart summary present");
    chart.text_alternative = M5SummaryTextAlternative {
        kind: M5SummaryTextAlternativeKind::NotApplicable,
        provided: false,
        alt_text_message_id: String::new(),
        export_metadata_fields: Vec::new(),
    };
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::TextAlternativeInconsistent));
}

#[test]
fn text_native_surface_with_text_alternative_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    let editor = packet
        .summaries
        .iter_mut()
        .find(|s| s.surface_kind == M5SummarySurfaceKind::CustomEditor)
        .expect("editor summary present");
    editor.text_alternative = M5SummaryTextAlternative {
        kind: M5SummaryTextAlternativeKind::ChartDescription,
        provided: true,
        alt_text_message_id: "summary.editor.alt".to_owned(),
        export_metadata_fields: vec!["x".to_owned()],
    };
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::TextAlternativeInconsistent));
}

#[test]
fn missing_provisional_state_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    // Collapse every Preview state to Authoritative; the provisional Preview state is
    // now unexercised.
    for summary in &mut packet.summaries {
        if summary.presentation_state == M5SummaryPresentationState::Preview {
            summary.presentation_state = M5SummaryPresentationState::Authoritative;
        }
    }
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::PresentationStateNotExercised));
}

#[test]
fn unsupported_fidelity_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].non_visual_fidelity = A11yNonVisualFidelity::UnsupportedBlocked;
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::SummaryNonVisualFidelityInvalid));
}

#[test]
fn stable_summary_missing_proof_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::StableSummaryMissingProof));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::ConsumerSurfacesMissing));
}

#[test]
fn non_reopenable_durable_fallback_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.summaries[0].durable_fallback.reopenable = false;
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::DurableFallbackMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::MissingSourceContracts));
}

#[test]
fn conformance_review_incomplete_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet
        .conformance_review
        .drilldowns_remain_actionable_not_vague_one_liners = false;
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::ConformanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.consumer_projection.support_export_reuses_summaries = false;
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_nonvisual_summary_catalog();
    packet
        .release_posture
        .stable_promotion_blocks_without_mapped_proof = false;
    assert!(packet
        .validate()
        .contains(&M5NonVisualSummaryViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface_and_drilldown() {
    let packet = seeded_m5_nonvisual_summary_catalog();
    let summary = packet.render_markdown_summary();
    for s in &packet.summaries {
        assert!(
            summary.contains(&s.summary_id),
            "summary missing surface {}",
            s.summary_id
        );
        assert!(
            summary.contains(&s.object_identity_ref),
            "summary missing object identity {}",
            s.object_identity_ref
        );
        for drilldown in &s.drilldowns {
            assert!(
                summary.contains(&drilldown.route_message_id),
                "summary missing drill-down {}",
                drilldown.route_message_id
            );
        }
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_nonvisual_summary_export()
        .expect("checked M5 non-visual summary export validates");
    assert_eq!(packet.packet_id, M5_NONVISUAL_SUMMARY_CATALOG_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_nonvisual_summary_export()
        .expect("checked M5 non-visual summary export validates");
    assert_eq!(
        from_disk,
        seeded_m5_nonvisual_summary_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_nonvisual_summary_catalog_proof_stale_narrowed(),
        seeded_m5_nonvisual_summary_catalog_bridge_unavailable_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing a surface.
        assert_eq!(packet.summaries.len(), M5SummarySurfaceKind::ALL.len());
    }

    let proof_stale = seeded_m5_nonvisual_summary_catalog_proof_stale_narrowed();
    let chart = proof_stale
        .summaries
        .iter()
        .find(|s| s.surface_kind == M5SummarySurfaceKind::Chart)
        .expect("chart summary present");
    assert_eq!(
        chart.qualification,
        M5DynamicSurfaceA11yQualificationClass::Beta
    );
    // The narrowed chart still keeps its text alternative and drill-downs.
    assert!(chart.text_alternative.provided);
    assert!(chart.drilldowns.len() >= M5_SUMMARY_MIN_DRILLDOWNS);

    let bridge_down = seeded_m5_nonvisual_summary_catalog_bridge_unavailable_narrowed();
    let artifact = bridge_down
        .summaries
        .iter()
        .find(|s| s.surface_kind == M5SummarySurfaceKind::ArtifactViewer)
        .expect("artifact-viewer summary present");
    assert_eq!(
        artifact.qualification,
        M5DynamicSurfaceA11yQualificationClass::Preview
    );
    assert_eq!(
        artifact.non_visual_fidelity,
        A11yNonVisualFidelity::DegradedAccessible
    );
    // The artifact still exposes its non-visual alternative rather than disappearing.
    assert!(artifact.text_alternative.provided);
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-nonvisual-summaries/proof_stale_narrowed.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-nonvisual-summaries/bridge_unavailable_narrowed.json"
        )),
    ] {
        let packet: M5NonVisualSummaryCatalogPacket =
            serde_json::from_str(raw).expect("fixture parses as non-visual summary packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_nonvisual_summary_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
