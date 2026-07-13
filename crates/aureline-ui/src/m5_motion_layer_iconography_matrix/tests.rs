use super::*;

#[test]
fn seeded_matrix_validates() {
    let packet = seeded_m5_motion_layer_iconography_matrix();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(
        packet.packet_id,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_PACKET_ID
    );
}

#[test]
fn seeded_matrix_names_every_interaction_family() {
    let packet = seeded_m5_motion_layer_iconography_matrix();
    let present: std::collections::BTreeSet<_> = packet
        .interaction_rows
        .iter()
        .map(|r| r.interaction_family)
        .collect();
    for family in M5VisualInteractionFamily::ALL {
        assert!(
            present.contains(&family),
            "missing interaction family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.interaction_rows.len(),
        M5VisualInteractionFamily::ALL.len()
    );
}

#[test]
fn frozen_interaction_role_vocabulary_is_exact() {
    // The one acceptance-criteria vocabulary: motion / overlay / layer / portal / icon / illustration /
    // attention stays in one controlled token set that no desktop, dialog, onboarding, notification, or
    // embedded surface reinvents.
    let tokens: Vec<&str> = M5VisualInteractionRole::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    assert_eq!(
        tokens,
        vec![
            "motion",
            "overlay",
            "layer",
            "portal",
            "icon",
            "illustration",
            "attention",
        ]
    );
    assert!(M5VisualInteractionRole::Motion.demands_accessible_fallback());
    assert!(M5VisualInteractionRole::Overlay.demands_accessible_fallback());
    assert!(M5VisualInteractionRole::Icon.demands_accessible_fallback());
    assert!(M5VisualInteractionRole::Illustration.demands_accessible_fallback());
    assert!(M5VisualInteractionRole::Attention.demands_accessible_fallback());
    assert!(!M5VisualInteractionRole::Layer.demands_accessible_fallback());
    assert!(!M5VisualInteractionRole::Portal.demands_accessible_fallback());
}

#[test]
fn every_family_declares_mandatory_labels_schema_and_deployment_lines() {
    let packet = seeded_m5_motion_layer_iconography_matrix();
    for row in &packet.interaction_rows {
        for label in M5VisualInteractionRequiredLabel::MANDATORY {
            assert!(
                row.required_labels.contains(&label),
                "family {} missing mandatory label {}",
                row.interaction_family.as_str(),
                label.as_str()
            );
        }
        assert!(
            row.source_contract_refs.contains(
                &row.interaction_family
                    .canonical_domain_schema_ref()
                    .to_owned()
            ),
            "family {} does not point at its canonical schema",
            row.interaction_family.as_str()
        );
        assert!(!row.surface_families.is_empty());
        assert!(!row.deployment_lines.is_empty());
        assert!(!row.semantic_roles.is_empty());
        assert!(!row.degraded_reasons.is_empty());
        assert!(!row.accessibility_routes.is_empty());
        assert!(row
            .accessibility_routes
            .contains(&M5VisualInteractionAccessibilityRoute::ReducedMotionSafe));
    }
}

#[test]
fn family_specific_vocabularies_are_declared_only_where_applicable() {
    let packet = seeded_m5_motion_layer_iconography_matrix();
    for row in &packet.interaction_rows {
        let family = row.interaction_family;
        assert_eq!(
            !row.motion_roles.is_empty(),
            family.declares_motion_roles(),
            "motion_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.reduced_motion_roles.is_empty(),
            family.declares_reduced_motion_roles(),
            "reduced_motion_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.opacity_scrim_roles.is_empty(),
            family.declares_opacity_scrim_roles(),
            "opacity_scrim_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.layer_order_roles.is_empty(),
            family.declares_layer_order_roles(),
            "layer_order_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.portal_ownership_roles.is_empty(),
            family.declares_portal_ownership_roles(),
            "portal_ownership_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.iconography_roles.is_empty(),
            family.declares_iconography_roles(),
            "iconography_roles presence wrong for {}",
            family.as_str()
        );
        assert_eq!(
            !row.illustration_roles.is_empty(),
            family.declares_illustration_roles(),
            "illustration_roles presence wrong for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_vocabulary_token_is_declared_by_some_family() {
    let packet = seeded_m5_motion_layer_iconography_matrix();
    for role in M5VisualInteractionRole::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.semantic_roles.contains(&role)),
            "no family declares interaction role {}",
            role.as_str()
        );
    }
    for role in M5MotionTokenRole::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.motion_roles.contains(&role)),
            "no family declares motion role {}",
            role.as_str()
        );
    }
    for role in M5ReducedMotionRole::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.reduced_motion_roles.contains(&role)),
            "no family declares reduced-motion role {}",
            role.as_str()
        );
    }
    for role in M5OpacityScrimRole::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.opacity_scrim_roles.contains(&role)),
            "no family declares opacity / scrim role {}",
            role.as_str()
        );
    }
    for role in M5LayerOrderRole::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.layer_order_roles.contains(&role)),
            "no family declares layer-order role {}",
            role.as_str()
        );
    }
    for role in M5PortalOwnershipRole::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.portal_ownership_roles.contains(&role)),
            "no family declares portal-ownership role {}",
            role.as_str()
        );
    }
    for role in M5IconographyRole::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.iconography_roles.contains(&role)),
            "no family declares iconography role {}",
            role.as_str()
        );
    }
    for role in M5IllustrationRole::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.illustration_roles.contains(&role)),
            "no family declares illustration role {}",
            role.as_str()
        );
    }
    for reason in M5VisualInteractionDegradedReason::ALL {
        assert!(
            packet
                .interaction_rows
                .iter()
                .any(|row| row.degraded_reasons.contains(&reason)),
            "no family declares degraded reason {}",
            reason.as_str()
        );
    }
}

#[test]
fn missing_interaction_family_fails_validation() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet
        .interaction_rows
        .retain(|row| row.interaction_family != M5VisualInteractionFamily::Illustration);
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::RequiredFamilyMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.vocabulary_set.semantic_roles.pop();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::VocabularySetDrift));
}

#[test]
fn mandatory_label_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[0]
        .required_labels
        .retain(|label| *label != M5VisualInteractionRequiredLabel::Identity);
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::MandatoryLabelMissing));
}

#[test]
fn domain_schema_ref_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let own = M5VisualInteractionFamily::MotionToken.canonical_domain_schema_ref();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::MotionToken)
        .expect("motion-token row present");
    row.source_contract_refs.retain(|r| r != own);
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::DomainSchemaRefMissing));
}

#[test]
fn semantic_role_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[0].semantic_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::SemanticRoleMissing));
}

#[test]
fn motion_role_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::MotionToken)
        .expect("motion-token present");
    row.motion_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::MotionRoleMissing));
}

#[test]
fn reduced_motion_role_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::ReducedMotion)
        .expect("reduced-motion present");
    row.reduced_motion_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::ReducedMotionRoleMissing));
}

#[test]
fn opacity_scrim_role_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::OpacityScrim)
        .expect("opacity-scrim present");
    row.opacity_scrim_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::OpacityScrimRoleMissing));
}

#[test]
fn layer_order_role_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::LayerOrder)
        .expect("layer-order present");
    row.layer_order_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::LayerOrderRoleMissing));
}

#[test]
fn portal_ownership_role_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::PortalOwnership)
        .expect("portal-ownership present");
    row.portal_ownership_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::PortalOwnershipRoleMissing));
}

#[test]
fn iconography_role_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::Iconography)
        .expect("iconography present");
    row.iconography_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::IconographyRoleMissing));
}

#[test]
fn illustration_role_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::Illustration)
        .expect("illustration present");
    row.illustration_roles.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::IllustrationRoleMissing));
}

#[test]
fn degraded_reason_missing_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[3].degraded_reasons.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::DegradedReasonMissing));
}

#[test]
fn interaction_invariant_violation_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[0].delays_protected_input_with_motion = true;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::InteractionInvariantViolated));

    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[2].scrim_erases_orientation_or_contrast = true;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::InteractionInvariantViolated));

    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[3].overlay_bypasses_shared_z_order = true;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::InteractionInvariantViolated));

    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[5].uses_unlabeled_icon_for_uncommon_or_destructive_action = true;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::InteractionInvariantViolated));

    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[6].lets_illustration_impersonate_operational_or_security_truth = true;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::InteractionInvariantViolated));
}

#[test]
fn stable_family_missing_proof_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    let row = packet
        .interaction_rows
        .iter_mut()
        .find(|row| row.interaction_family == M5VisualInteractionFamily::MotionToken)
        .expect("motion-token row present");
    row.required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::StableFamilyMissingProof));
}

#[test]
fn missing_deployment_lines_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[1].deployment_lines.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::DeploymentLineMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.governance_review.motion_never_delays_protected_input = false;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet
        .consumer_projection
        .support_export_reads_single_visual_interaction_source = false;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_interaction_family() {
    let summary = seeded_m5_motion_layer_iconography_matrix().render_markdown_summary();
    for family in M5VisualInteractionFamily::ALL {
        assert!(
            summary.contains(family.as_str()),
            "summary missing family {}",
            family.as_str()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_family() {
    let csv = seeded_m5_motion_layer_iconography_matrix().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5VisualInteractionFamily::ALL.len());
    assert!(lines[0].starts_with("interaction_family,qualification,owner,canonical_schema,"));
    for family in M5VisualInteractionFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing family {}",
            family.as_str()
        );
        assert!(
            csv.contains(family.canonical_domain_schema_ref()),
            "csv missing canonical schema for {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_motion_layer_iconography_matrix_export()
        .expect("checked M5 motion / layer / iconography matrix export validates");
    assert_eq!(
        packet.packet_id,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_PACKET_ID
    );
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_motion_layer_iconography_matrix_export()
        .expect("checked M5 motion / layer / iconography matrix export validates");
    assert_eq!(
        from_disk,
        seeded_m5_motion_layer_iconography_matrix(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_families_visible() {
    for packet in [
        seeded_m5_motion_layer_iconography_matrix_reduced_motion_beta_narrowed(),
        seeded_m5_motion_layer_iconography_matrix_illustration_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.interaction_rows.len(),
            M5VisualInteractionFamily::ALL.len()
        );
    }

    let reduced_motion = seeded_m5_motion_layer_iconography_matrix_reduced_motion_beta_narrowed();
    let row = reduced_motion
        .interaction_rows
        .iter()
        .find(|r| r.interaction_family == M5VisualInteractionFamily::ReducedMotion)
        .expect("reduced-motion row present");
    assert_eq!(
        row.qualification,
        M5VisualInteractionQualificationClass::Beta
    );

    let illustration = seeded_m5_motion_layer_iconography_matrix_illustration_preview_narrowed();
    let row = illustration
        .interaction_rows
        .iter()
        .find(|r| r.interaction_family == M5VisualInteractionFamily::Illustration)
        .expect("illustration row present");
    assert_eq!(
        row.qualification,
        M5VisualInteractionQualificationClass::Preview
    );
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let reduced_motion: M5MotionLayerIconographyMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-motion-layer-iconography/reduced_motion_beta_narrowed.json"
        )))
        .expect("reduced-motion fixture parses");
    assert!(reduced_motion.validate().is_empty());
    assert_eq!(
        reduced_motion,
        seeded_m5_motion_layer_iconography_matrix_reduced_motion_beta_narrowed()
    );

    let illustration: M5MotionLayerIconographyMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ui/m5-motion-layer-iconography/illustration_preview_narrowed.json"
        )))
        .expect("illustration fixture parses");
    assert!(illustration.validate().is_empty());
    assert_eq!(
        illustration,
        seeded_m5_motion_layer_iconography_matrix_illustration_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_raw_material() {
    let json = seeded_m5_motion_layer_iconography_matrix().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("password"));
    assert!(!lower.contains("passphrase"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("-----begin"));
}

#[test]
fn injected_raw_material_is_rejected() {
    let mut packet = seeded_m5_motion_layer_iconography_matrix();
    packet.interaction_rows[0].scope_summary =
        "raw endpoint https://registry.example/artifact leaked".to_owned();
    assert!(packet
        .validate()
        .contains(&M5MotionLayerIconographyMatrixViolation::RawMaterialInExport));
}
