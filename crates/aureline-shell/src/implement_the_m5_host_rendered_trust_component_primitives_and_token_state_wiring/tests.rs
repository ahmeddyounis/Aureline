use super::*;

fn input(
    family: M5HostRenderedPrimitiveFamily,
    render_mode: M5PrimitiveRenderMode,
    wrapper_ref: Option<&str>,
) -> M5PrimitiveBindingInput {
    M5PrimitiveBindingInput {
        primitive_family: family,
        consumer_id: "consumer:test:1".to_owned(),
        host_surface: M5PrimitiveHostSurface::DesktopApp,
        render_mode,
        audited_wrapper_ref: wrapper_ref.map(str::to_owned),
        wired_token_slots: family.fixed_token_slots(),
        restyled_aspects: vec![M5RestylableAspect::AccentTint],
        overridden_contract_parts: Vec::new(),
    }
}

// ---- resolver -----------------------------------------------------------

#[test]
fn resolver_blesses_a_canonical_conformant_binding() {
    let resolved = resolve_binding(&input(
        M5HostRenderedPrimitiveFamily::SettingsRow,
        M5PrimitiveRenderMode::HostRenderedCanonical,
        None,
    ))
    .expect("resolves");
    assert_eq!(resolved.conformance, M5BindingConformance::Conformant);
    assert!(resolved.renders_through_canonical);
    assert!(resolved.fixed_token_slots_wired);
    assert!(resolved.restyle_within_bounds);
    assert_eq!(
        resolved.required_token_slots,
        M5HostRenderedPrimitiveFamily::SettingsRow.fixed_token_slots()
    );
}

#[test]
fn resolver_accepts_an_audited_wrapper_with_a_ref() {
    let resolved = resolve_binding(&input(
        M5HostRenderedPrimitiveFamily::CapabilitySheet,
        M5PrimitiveRenderMode::AuditedWrapper,
        Some("audit:wrapper:1"),
    ))
    .expect("resolves");
    assert_eq!(resolved.conformance, M5BindingConformance::Conformant);
    assert!(resolved.renders_through_canonical);
}

#[test]
fn resolver_flags_a_bespoke_local_variant_as_drift() {
    let resolved = resolve_binding(&input(
        M5HostRenderedPrimitiveFamily::SettingsRow,
        M5PrimitiveRenderMode::BespokeLocalVariant,
        None,
    ))
    .expect("resolves");
    assert_eq!(resolved.conformance, M5BindingConformance::BespokeDrift);
    assert!(!resolved.renders_through_canonical);
}

#[test]
fn resolver_flags_an_unwired_fixed_token_slot() {
    let mut malformed = input(
        M5HostRenderedPrimitiveFamily::EventHistoryRow,
        M5PrimitiveRenderMode::HostRenderedCanonical,
        None,
    );
    malformed
        .wired_token_slots
        .retain(|slot| *slot != M5DesignTokenSlot::ProvenanceBadge);
    let resolved = resolve_binding(&malformed).expect("resolves");
    assert_eq!(
        resolved.conformance,
        M5BindingConformance::TokenWiringIncomplete
    );
    assert!(!resolved.fixed_token_slots_wired);
}

#[test]
fn resolver_flags_an_overridden_contract_part() {
    let mut malformed = input(
        M5HostRenderedPrimitiveFamily::TimelineGroup,
        M5PrimitiveRenderMode::HostRenderedCanonical,
        None,
    );
    malformed.overridden_contract_parts = vec![M5PrimitiveContractPart::SeveritySemantics];
    let resolved = resolve_binding(&malformed).expect("resolves");
    assert_eq!(
        resolved.conformance,
        M5BindingConformance::ContractPartOverridden
    );
    assert!(!resolved.restyle_within_bounds);
}

#[test]
fn resolver_bespoke_drift_wins_over_other_faults() {
    // A bespoke variant that also skips a token slot is still classified as the
    // worse bespoke drift.
    let mut malformed = input(
        M5HostRenderedPrimitiveFamily::SettingsRow,
        M5PrimitiveRenderMode::BespokeLocalVariant,
        None,
    );
    malformed.wired_token_slots.clear();
    let resolved = resolve_binding(&malformed).expect("resolves");
    assert_eq!(resolved.conformance, M5BindingConformance::BespokeDrift);
}

#[test]
fn resolver_rejects_malformed_input() {
    // Empty consumer id.
    let mut malformed = input(
        M5HostRenderedPrimitiveFamily::SettingsRow,
        M5PrimitiveRenderMode::HostRenderedCanonical,
        None,
    );
    malformed.consumer_id = "  ".to_owned();
    assert_eq!(
        resolve_binding(&malformed),
        Err(M5BindingResolutionError::EmptyConsumerId)
    );

    // Audited wrapper without a ref.
    assert_eq!(
        resolve_binding(&input(
            M5HostRenderedPrimitiveFamily::SettingsRow,
            M5PrimitiveRenderMode::AuditedWrapper,
            None,
        )),
        Err(M5BindingResolutionError::WrapperRefMissing)
    );

    // Audited wrapper with a blank ref.
    assert_eq!(
        resolve_binding(&input(
            M5HostRenderedPrimitiveFamily::SettingsRow,
            M5PrimitiveRenderMode::AuditedWrapper,
            Some("   "),
        )),
        Err(M5BindingResolutionError::WrapperRefMissing)
    );

    // Wrapper ref on a non-wrapper mode.
    assert_eq!(
        resolve_binding(&input(
            M5HostRenderedPrimitiveFamily::SettingsRow,
            M5PrimitiveRenderMode::HostRenderedCanonical,
            Some("audit:wrapper:1"),
        )),
        Err(M5BindingResolutionError::UnexpectedWrapperRef)
    );

    // Forbidden material in the consumer id.
    let mut malformed = input(
        M5HostRenderedPrimitiveFamily::SettingsRow,
        M5PrimitiveRenderMode::HostRenderedCanonical,
        None,
    );
    malformed.consumer_id = "reach https://example.test".to_owned();
    assert_eq!(
        resolve_binding(&malformed),
        Err(M5BindingResolutionError::ForbiddenMaterial)
    );
}

// ---- packet -------------------------------------------------------------

#[test]
fn seeded_packet_validates() {
    let packet = seeded_m5_host_rendered_primitive_packet();
    assert!(packet.validate().is_empty(), "{:?}", packet.validate());
    assert_eq!(packet.packet_id, M5_HOST_RENDERED_PRIMITIVE_PACKET_ID);
}

#[test]
fn seeded_packet_names_every_primitive_family() {
    let packet = seeded_m5_host_rendered_primitive_packet();
    let present: std::collections::BTreeSet<_> = packet
        .primitive_rows
        .iter()
        .map(|r| r.primitive_family)
        .collect();
    for family in M5HostRenderedPrimitiveFamily::ALL {
        assert!(
            present.contains(&family),
            "missing primitive family {}",
            family.as_str()
        );
    }
    assert_eq!(
        packet.primitive_rows.len(),
        M5HostRenderedPrimitiveFamily::ALL.len()
    );
}

#[test]
fn bound_families_cover_every_frozen_component_family() {
    let packet = seeded_m5_host_rendered_primitive_packet();
    let covered: std::collections::BTreeSet<_> = packet
        .primitive_rows
        .iter()
        .flat_map(|row| row.bound_component_families.iter().copied())
        .collect();
    for family in M5TrustComponentFamily::ALL {
        assert!(
            covered.contains(&family),
            "frozen component family {} not host-rendered",
            family.as_str()
        );
    }
}

#[test]
fn every_row_declares_mandatory_contract_parts_host_surfaces_and_examples() {
    let packet = seeded_m5_host_rendered_primitive_packet();
    for row in &packet.primitive_rows {
        for part in M5PrimitiveContractPart::MANDATORY {
            assert!(row.fixed_contract_parts.contains(&part));
        }
        for surface in M5PrimitiveHostSurface::MANDATORY {
            assert!(row.host_surfaces.contains(&surface));
        }
        assert!(row
            .accessibility_routes
            .contains(&M5TrustAccessibilityRoute::KeyboardFocusable));
        assert!(!row.example_bindings.is_empty());
        assert!(!row
            .render_modes
            .contains(&M5PrimitiveRenderMode::BespokeLocalVariant));
    }
}

#[test]
fn every_row_pins_exactly_the_family_token_slots() {
    let packet = seeded_m5_host_rendered_primitive_packet();
    for row in &packet.primitive_rows {
        assert_eq!(
            row.fixed_token_slots,
            row.primitive_family.fixed_token_slots(),
            "row {} pins the wrong token slots",
            row.primitive_family.as_str()
        );
    }
    // The settings row wires the source pill; the chronology families wire the
    // provenance badge; the capability sheet wires neither badge.
    let settings = packet
        .primitive_rows
        .iter()
        .find(|r| r.primitive_family == M5HostRenderedPrimitiveFamily::SettingsRow)
        .unwrap();
    assert!(settings
        .fixed_token_slots
        .contains(&M5DesignTokenSlot::SourcePill));
    let capability = packet
        .primitive_rows
        .iter()
        .find(|r| r.primitive_family == M5HostRenderedPrimitiveFamily::CapabilitySheet)
        .unwrap();
    assert!(!capability
        .fixed_token_slots
        .contains(&M5DesignTokenSlot::SourcePill));
    assert!(!capability
        .fixed_token_slots
        .contains(&M5DesignTokenSlot::ProvenanceBadge));
}

#[test]
fn every_worked_case_is_self_consistent_and_conformant() {
    let packet = seeded_m5_host_rendered_primitive_packet();
    for row in &packet.primitive_rows {
        for case in &row.example_bindings {
            assert!(
                case.is_self_consistent(),
                "worked case for {} drifted from resolver output",
                row.primitive_family.as_str()
            );
            assert!(
                case.resolved.conformance.is_conformant(),
                "seed worked case for {} is not conformant",
                row.primitive_family.as_str()
            );
        }
    }
}

#[test]
fn seed_exercises_every_host_surface_and_both_safe_render_modes() {
    let packet = seeded_m5_host_rendered_primitive_packet();
    let surfaces: std::collections::BTreeSet<_> = packet
        .primitive_rows
        .iter()
        .flat_map(|row| row.example_bindings.iter())
        .map(|case| case.resolved.host_surface)
        .collect();
    for surface in M5PrimitiveHostSurface::ALL {
        assert!(
            surfaces.contains(&surface),
            "surface {} unused",
            surface.as_str()
        );
    }
    let modes: std::collections::BTreeSet<_> = packet
        .primitive_rows
        .iter()
        .flat_map(|row| row.example_bindings.iter())
        .map(|case| case.resolved.render_mode)
        .collect();
    assert!(modes.contains(&M5PrimitiveRenderMode::HostRenderedCanonical));
    assert!(modes.contains(&M5PrimitiveRenderMode::AuditedWrapper));
    assert!(!modes.contains(&M5PrimitiveRenderMode::BespokeLocalVariant));
}

#[test]
fn missing_primitive_family_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet
        .primitive_rows
        .retain(|row| row.primitive_family != M5HostRenderedPrimitiveFamily::CapabilitySheet);
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::RequiredPrimitiveMissing));
}

#[test]
fn vocabulary_set_drift_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.vocabulary_set.token_slots.pop();
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::VocabularySetDrift));
}

#[test]
fn bound_families_mismatch_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0].bound_component_families =
        vec![M5TrustComponentFamily::CapabilitySheet];
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::BoundFamiliesMismatch));
}

#[test]
fn fixed_token_slots_mismatch_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0]
        .fixed_token_slots
        .retain(|s| *s != M5DesignTokenSlot::FocusRing);
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::FixedTokenSlotsMismatch));
}

#[test]
fn mandatory_contract_part_missing_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0]
        .fixed_contract_parts
        .retain(|p| *p != M5PrimitiveContractPart::AuditExportAnchor);
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::MandatoryContractPartMissing));
}

#[test]
fn mandatory_host_surface_missing_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0]
        .host_surfaces
        .retain(|s| *s != M5PrimitiveHostSurface::ExtensionHost);
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::MandatoryHostSurfaceMissing));
}

#[test]
fn render_mode_unsafe_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0]
        .render_modes
        .push(M5PrimitiveRenderMode::BespokeLocalVariant);
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::RenderModeUnsafe));
}

#[test]
fn example_binding_drift_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0].example_bindings[0]
        .resolved
        .conformance = M5BindingConformance::TokenWiringIncomplete;
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::ExampleBindingDrift));
}

#[test]
fn example_binding_missing_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[2].example_bindings.clear();
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::ExampleBindingMissing));
}

#[test]
fn example_family_mismatch_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    // Swap two rows' families so every family stays present (no
    // RequiredPrimitiveMissing early return) but each row's examples no longer match
    // its family.
    let fam0 = packet.primitive_rows[0].primitive_family;
    let fam1 = packet.primitive_rows[1].primitive_family;
    packet.primitive_rows[0].primitive_family = fam1;
    packet.primitive_rows[1].primitive_family = fam0;
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::ExampleFamilyMismatch));
}

#[test]
fn canonical_rendering_unproven_fails_when_a_bespoke_binding_exists() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    let family = packet.primitive_rows[0].primitive_family;
    // Replace one worked case with a self-consistent bespoke binding.
    packet.primitive_rows[0].example_bindings[0] =
        M5PrimitiveBindingCase::resolved(M5PrimitiveBindingInput {
            primitive_family: family,
            consumer_id: "consumer:bespoke:1".to_owned(),
            host_surface: M5PrimitiveHostSurface::DesktopApp,
            render_mode: M5PrimitiveRenderMode::BespokeLocalVariant,
            audited_wrapper_ref: None,
            wired_token_slots: family.fixed_token_slots(),
            restyled_aspects: Vec::new(),
            overridden_contract_parts: Vec::new(),
        });
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::CanonicalRenderingUnproven));
}

#[test]
fn token_wiring_parity_unproven_fails_with_a_single_surface() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    // Keep only the desktop binding — one surface can't prove cross-surface parity.
    packet.primitive_rows[0]
        .example_bindings
        .retain(|case| case.resolved.host_surface == M5PrimitiveHostSurface::DesktopApp);
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::TokenWiringParityUnproven));
}

#[test]
fn naming_parity_unproven_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0].naming_parity.screenshot_name = "settings_widget".to_owned();
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::NamingParityUnproven));
}

#[test]
fn matrix_family_coverage_unproven_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    // Drop the narrative summary card from the timeline-group binding.
    let row = packet
        .primitive_rows
        .iter_mut()
        .find(|r| r.primitive_family == M5HostRenderedPrimitiveFamily::TimelineGroup)
        .unwrap();
    row.bound_component_families = vec![M5TrustComponentFamily::TimelineGroup];
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::MatrixFamilyCoverageUnproven));
}

#[test]
fn primitive_invariant_violation_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0].allows_bespoke_local_variant = true;
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::PrimitiveInvariantViolated));
}

#[test]
fn stable_primitive_missing_proof_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.primitive_rows[0].required_proof_packet_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::StablePrimitiveMissingProof));
}

#[test]
fn missing_source_contracts_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.source_contract_refs.clear();
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::MissingSourceContracts));
}

#[test]
fn governance_review_incomplete_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.governance_review.shared_token_state_wiring_pinned = false;
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::GovernanceReviewIncomplete));
}

#[test]
fn consumer_projection_incomplete_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet
        .consumer_projection
        .extension_consumers_render_canonical_or_wrapper = false;
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::ConsumerProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut packet = seeded_m5_host_rendered_primitive_packet();
    packet.release_posture.accessibility_parity_required = false;
    assert!(packet
        .validate()
        .contains(&M5HostRenderedPrimitiveViolation::ReleasePostureIncomplete));
}

#[test]
fn markdown_summary_lists_every_primitive_family() {
    let summary = seeded_m5_host_rendered_primitive_packet().render_markdown_summary();
    for family in M5HostRenderedPrimitiveFamily::ALL {
        assert!(
            summary.contains(family.label()),
            "summary missing primitive family {}",
            family.label()
        );
    }
}

#[test]
fn matrix_csv_has_a_row_per_primitive_family() {
    let csv = seeded_m5_host_rendered_primitive_packet().render_matrix_csv();
    let lines: Vec<&str> = csv.lines().collect();
    assert_eq!(lines.len(), 1 + M5HostRenderedPrimitiveFamily::ALL.len());
    assert!(lines[0].starts_with("primitive_family,qualification,owner,"));
    for family in M5HostRenderedPrimitiveFamily::ALL {
        assert!(
            csv.contains(family.as_str()),
            "csv missing primitive family {}",
            family.as_str()
        );
    }
}

#[test]
fn checked_support_export_validates_and_matches_seed() {
    let from_disk = current_stable_m5_host_rendered_primitive_export()
        .expect("checked M5 host-rendered primitive export validates");
    assert_eq!(from_disk.packet_id, M5_HOST_RENDERED_PRIMITIVE_PACKET_ID);
    assert_eq!(
        from_disk,
        seeded_m5_host_rendered_primitive_packet(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn narrowed_variants_validate_and_keep_primitives_visible() {
    for packet in [
        seeded_m5_host_rendered_primitive_capability_sheet_beta_narrowed(),
        seeded_m5_host_rendered_primitive_chronology_export_preview_narrowed(),
    ] {
        assert!(
            packet.validate().is_empty(),
            "narrowed variant failed validation: {:?}",
            packet.validate()
        );
        assert_eq!(
            packet.primitive_rows.len(),
            M5HostRenderedPrimitiveFamily::ALL.len()
        );
    }

    let capability = seeded_m5_host_rendered_primitive_capability_sheet_beta_narrowed();
    let row = capability
        .primitive_rows
        .iter()
        .find(|r| r.primitive_family == M5HostRenderedPrimitiveFamily::CapabilitySheet)
        .expect("capability-sheet row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Beta);

    let chronology = seeded_m5_host_rendered_primitive_chronology_export_preview_narrowed();
    let row = chronology
        .primitive_rows
        .iter()
        .find(|r| r.primitive_family == M5HostRenderedPrimitiveFamily::ChronologyExportPreview)
        .expect("chronology-export-preview row present");
    assert_eq!(row.qualification, M5TrustQualificationClass::Preview);
}

#[test]
fn checked_narrowed_fixtures_validate_and_match_seed_builders() {
    let capability: M5HostRenderedPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-host-rendered-primitives/capability_sheet_beta_narrowed.json"
    )))
    .expect("capability fixture parses");
    assert!(capability.validate().is_empty());
    assert_eq!(
        capability,
        seeded_m5_host_rendered_primitive_capability_sheet_beta_narrowed()
    );

    let chronology: M5HostRenderedPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/ui/m5-host-rendered-primitives/chronology_export_preview_narrowed.json"
    )))
    .expect("chronology fixture parses");
    assert!(chronology.validate().is_empty());
    assert_eq!(
        chronology,
        seeded_m5_host_rendered_primitive_chronology_export_preview_narrowed()
    );
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_m5_host_rendered_primitive_packet().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
    assert!(!lower.contains("://"));
    assert!(!lower.contains("secret"));
}
