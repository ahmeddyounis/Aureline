use super::*;

#[test]
fn seeded_catalog_validates() {
    let packet = seeded_m5_surface_descriptor_catalog();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_SURFACE_DESCRIPTOR_CATALOG_PACKET_ID);
}

#[test]
fn seeded_catalog_covers_every_surface_family() {
    let packet = seeded_m5_surface_descriptor_catalog();
    let present: std::collections::BTreeSet<_> = packet
        .descriptors
        .iter()
        .map(|d| d.surface_family)
        .collect();
    for family in M5SurfaceFamily::ALL {
        assert!(
            present.contains(&family),
            "missing surface family {}",
            family.as_str()
        );
    }
}

#[test]
fn missing_surface_family_fails_validation() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet
        .descriptors
        .retain(|d| d.surface_family != M5SurfaceFamily::OverlaySheet);
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::RequiredSurfaceFamilyMissing));
}

#[test]
fn shared_vocabulary_drift_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.shared_vocabulary_set.bridge_states.pop();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::VocabularySetDrift));
}

#[test]
fn descriptor_vocabulary_drift_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.descriptor_vocabulary_set.surface_families.pop();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::VocabularySetDrift));
}

#[test]
fn duplicate_surface_id_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    let clone = packet.descriptors[0].clone();
    packet.descriptors.push(clone);
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::DuplicateSurfaceId));
}

#[test]
fn empty_regions_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.descriptors[0].regions.clear();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::RegionsIncomplete));
}

#[test]
fn non_contiguous_focus_order_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    // Reindex the first stop to a gap so the order is no longer contiguous.
    packet.descriptors[0].focus_order.stops[0].order_index = 7;
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::FocusOrderMalformed));
}

#[test]
fn focus_stop_to_unknown_region_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.descriptors[0].focus_order.stops[0].region_id = "region:does-not-exist".to_owned();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::FocusOrderMalformed));
}

#[test]
fn interactive_surface_without_focus_stops_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.descriptors[0].focus_order.stops.clear();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::FocusOrderMalformed));
}

#[test]
fn reduced_motion_change_without_posture_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    // Claim behavior changes under reduced motion but declare a no-change posture.
    let descriptor = packet
        .descriptors
        .iter_mut()
        .find(|d| d.surface_family == M5SurfaceFamily::ShellRegion)
        .expect("shell region descriptor present");
    descriptor.motion_zoom.behavior_changes_under_reduced_motion = true;
    descriptor.motion_zoom.reduced_motion = M5ReducedMotionPosture::MotionIndependentAlready;
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::MotionZoomDeclarationMissing));
}

#[test]
fn high_zoom_no_change_with_adapting_posture_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    let descriptor = packet
        .descriptors
        .iter_mut()
        .find(|d| d.surface_family == M5SurfaceFamily::ShellRegion)
        .expect("shell region descriptor present");
    descriptor.motion_zoom.behavior_changes_under_high_zoom = false;
    descriptor.motion_zoom.high_zoom = M5HighZoomPosture::ReflowsToSingleColumn;
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::MotionZoomDeclarationMissing));
}

#[test]
fn healthy_bridge_with_degradation_reason_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.descriptors[0].bridge_mapping.degradation_reason =
        M5BridgeDegradationReason::PartialTreeMapping;
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::BridgeDegradationNotDisclosed));
}

#[test]
fn degraded_bridge_without_disclosure_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    // Drop the bridge to partial but keep full fidelity and a stable claim — the
    // descriptor must be forced to disclose and narrow.
    let descriptor = &mut packet.descriptors[0];
    descriptor.bridge_mapping.bridge_state = A11yBridgeState::Partial;
    let violations = packet.validate();
    assert!(violations.contains(&M5SurfaceDescriptorViolation::BridgeDegradationNotDisclosed));
}

#[test]
fn stable_descriptor_missing_proof_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.descriptors[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::StableDescriptorMissingProof));
}

#[test]
fn missing_downgrade_triggers_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.descriptors[0].downgrade_triggers.clear();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::DowngradeTriggersMissing));
}

#[test]
fn missing_consumer_surfaces_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.descriptors[1].consumer_surfaces.clear();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::ConsumerSurfacesMissing));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::MissingSourceContracts));
}

#[test]
fn every_descriptor_names_an_owner() {
    let packet = seeded_m5_surface_descriptor_catalog();
    for descriptor in &packet.descriptors {
        assert!(
            !descriptor.owner_role.trim().is_empty(),
            "surface {} has no owner",
            descriptor.surface_id
        );
    }
}

#[test]
fn conformance_review_incomplete_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet
        .conformance_review
        .focus_never_teleports_or_vanishes_on_async_update = false;
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::ConformanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet
        .consumer_projection
        .at_conformance_packets_reuse_descriptors = false;
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_surface_descriptor_catalog();
    packet
        .release_posture
        .stable_promotion_blocks_without_mapped_proof = false;
    assert!(packet
        .validate()
        .contains(&M5SurfaceDescriptorViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_surface() {
    let summary = seeded_m5_surface_descriptor_catalog().render_markdown_summary();
    for descriptor in seeded_m5_surface_descriptor_catalog().descriptors {
        assert!(
            summary.contains(&descriptor.surface_id),
            "summary missing surface {}",
            descriptor.surface_id
        );
    }
}

#[test]
fn checked_support_export_validates() {
    let packet = current_stable_m5_surface_descriptor_export()
        .expect("checked M5 surface descriptor export validates");
    assert_eq!(packet.packet_id, M5_SURFACE_DESCRIPTOR_CATALOG_PACKET_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_stable_m5_surface_descriptor_export()
        .expect("checked M5 surface descriptor export validates");
    assert_eq!(
        from_disk,
        seeded_m5_surface_descriptor_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_surfaces_visible() {
    for packet in [
        seeded_m5_surface_descriptor_catalog_bridge_degraded(),
        seeded_m5_surface_descriptor_catalog_proof_stale_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        // Downgrade narrows the claim without removing the descriptor.
        assert_eq!(packet.descriptors.len(), M5SurfaceFamily::ALL.len());
    }

    let degraded = seeded_m5_surface_descriptor_catalog_bridge_degraded();
    let terminal = degraded
        .descriptors
        .iter()
        .find(|d| d.surface_family == M5SurfaceFamily::TerminalCanvas)
        .expect("terminal descriptor present");
    assert_eq!(
        terminal.qualification,
        M5DynamicSurfaceA11yQualificationClass::Preview
    );
    assert_eq!(
        terminal.bridge_mapping.bridge_state,
        A11yBridgeState::Partial
    );
    assert!(terminal.bridge_mapping.degradation_reason.is_degraded());

    let narrowed = seeded_m5_surface_descriptor_catalog_proof_stale_narrowed();
    let editor = narrowed
        .descriptors
        .iter()
        .find(|d| d.surface_family == M5SurfaceFamily::EditorCanvas)
        .expect("editor descriptor present");
    assert_eq!(
        editor.qualification,
        M5DynamicSurfaceA11yQualificationClass::Beta
    );
}

#[test]
fn checked_narrowed_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-surface-descriptors/bridge_degraded.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/a11y/m5-surface-descriptors/proof_stale_narrowed.json"
        )),
    ] {
        let packet: M5SurfaceDescriptorCatalogPacket =
            serde_json::from_str(raw).expect("fixture parses as descriptor catalog packet");
        assert!(
            packet.validate().is_empty(),
            "fixture failed validation: {:?}",
            packet.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_surface_descriptor_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
