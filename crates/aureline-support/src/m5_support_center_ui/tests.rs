use super::*;

fn packet() -> M5SupportCenterLayout {
    current_m5_support_center_layout().expect("packet parses")
}

#[test]
fn embedded_packet_parses_and_validates() {
    let packet = packet();
    assert_eq!(packet.schema_version, M5_SUPPORT_CENTER_UI_SCHEMA_VERSION);
    assert_eq!(packet.record_kind, M5_SUPPORT_CENTER_UI_RECORD_KIND);
    assert_eq!(packet.validate(), Vec::new());
}

#[test]
fn summary_counts_match_entries() {
    let packet = packet();
    assert_eq!(packet.summary, packet.computed_summary());
}

#[test]
fn every_module_has_exactly_one_entry() {
    let packet = packet();
    assert_eq!(packet.nav_entries.len(), SupportModule::ALL.len());
    for module in SupportModule::ALL {
        assert!(
            packet.entry_for(module).is_some(),
            "missing entry for module {}",
            module.as_str()
        );
    }
}

#[test]
fn module_registry_reuses_matrix_vocabulary() {
    // The layout names modules with the exact matrix registry, in the same order, so the desktop
    // shell, CLI/headless, docs/help, and support export all read one registry.
    let packet = packet();
    assert_eq!(packet.modules, SupportModule::ALL.to_vec());
    for entry in &packet.nav_entries {
        assert!(
            entry.matrix_row_ref.contains(entry.module.as_str()),
            "{} does not defer to its matrix row",
            entry.entry_id
        );
    }
}

#[test]
fn all_three_regions_present_and_accessible() {
    let packet = packet();
    for region in LayoutRegion::ALL {
        let descriptor = packet.region_for(region).expect("region present");
        assert!(
            descriptor.is_accessible(),
            "region {} is not accessible",
            region.as_str()
        );
    }
}

#[test]
fn shared_inspector_declares_all_facets_and_persists() {
    let packet = packet();
    assert!(packet.shared_inspector.persists_across_modules);
    assert!(packet.shared_inspector.declares_all_facets());
    for facet in InspectorFacet::ALL {
        assert!(
            packet.shared_inspector.facet(facet).is_some(),
            "shared inspector missing facet {}",
            facet.as_str()
        );
    }
}

#[test]
fn every_entry_is_gate_consistent() {
    let packet = packet();
    let inspector = &packet.shared_inspector;
    assert!(packet.all_entries_gate_consistent());
    for entry in &packet.nav_entries {
        assert_eq!(
            entry.presentation,
            entry.effective_presentation(inspector),
            "{}",
            entry.entry_id
        );
        assert_eq!(
            entry.downgrade_reasons,
            entry.computed_downgrade_reasons(inspector),
            "{}",
            entry.entry_id
        );
        assert_eq!(
            entry.recovery_path,
            entry.computed_recovery_path(inspector),
            "{}",
            entry.entry_id
        );
    }
}

#[test]
fn every_entry_requires_a_facet_and_reuses_a_source() {
    let packet = packet();
    for entry in &packet.nav_entries {
        assert!(
            !entry.required_facets.is_empty(),
            "{} requires no facet",
            entry.entry_id
        );
        assert!(
            !entry.integration_sources.is_empty(),
            "{} reuses no source",
            entry.entry_id
        );
    }
}

#[test]
fn every_entry_carries_required_refs() {
    let packet = packet();
    for entry in &packet.nav_entries {
        assert!(entry.has_required_evidence(), "{}", entry.entry_id);
    }
}

#[test]
fn presented_entries_are_whole() {
    let packet = packet();
    let inspector = &packet.shared_inspector;
    let presented = packet.presented_entries().count();
    assert!(
        presented >= 2,
        "fixture needs at least two cleanly presented entries to prove the gate is not a blanket withhold"
    );
    for entry in packet.presented_entries() {
        assert!(entry.accessibility_complete());
        assert_eq!(
            entry.facet_ceiling(inspector),
            PresentationDecision::Presented
        );
        assert!(entry.downgrade_reasons.is_empty());
        assert!(entry.caveats.is_empty());
        assert!(entry.unmet_or_unwired_fields.is_empty());
        assert!(!entry.recovery_path.is_offered());
        assert!(!entry.offered_actions.is_empty());
    }
}

#[test]
fn withheld_entries_offer_nothing() {
    let packet = packet();
    for entry in packet.withheld_entries() {
        assert_eq!(
            entry.presentation,
            PresentationDecision::Withheld,
            "{}",
            entry.entry_id
        );
        assert!(
            entry.offered_actions.is_empty(),
            "{} is withheld but offers actions",
            entry.entry_id
        );
    }
}

#[test]
fn narrowed_and_withheld_entries_offer_recovery_and_caveats() {
    let packet = packet();
    for entry in &packet.nav_entries {
        if entry.presentation.requires_recovery() {
            assert!(entry.recovery_path.is_offered(), "{}", entry.entry_id);
            assert!(!entry.caveats.is_empty(), "{}", entry.entry_id);
            assert!(
                !entry.unmet_or_unwired_fields.is_empty(),
                "{}",
                entry.entry_id
            );
        }
    }
}

#[test]
fn every_required_consumer_surface_binds() {
    let packet = packet();
    for surface in LayoutConsumerSurface::REQUIRED {
        assert!(
            packet.has_binding_for(surface),
            "missing binding for {}",
            surface.as_str()
        );
    }
}

#[test]
fn export_projection_reflects_entries_and_gate() {
    let packet = packet();
    let projection = packet.export_projection();
    assert_eq!(projection.rows.len(), packet.nav_entries.len());
    assert_eq!(projection.packet_id, packet.packet_id);
    assert_eq!(
        projection.all_entries_gate_consistent,
        packet.all_entries_gate_consistent()
    );
    assert_eq!(
        projection.presented_count,
        packet.presented_entries().count()
    );
    assert_eq!(projection.narrowed_count, packet.narrowed_entries().count());
    assert_eq!(projection.withheld_count, packet.withheld_entries().count());
    for (entry, row) in packet.nav_entries.iter().zip(projection.rows.iter()) {
        assert_eq!(row.presentation, entry.presentation.as_str());
        assert_eq!(row.presented, entry.is_presented(&packet.shared_inspector));
    }
}

#[test]
fn support_export_is_export_safe() {
    let packet = packet();
    let export = packet.support_export("support:m5:support-center-ui", "2026-06-16T13:00:00Z");
    assert!(export.is_export_safe());
    assert_eq!(export.layout_packet_id_ref, packet.packet_id);
    assert!(export.raw_private_material_excluded);
}

#[test]
fn presentations_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PresentationDecision> =
        packet.nav_entries.iter().map(|e| e.presentation).collect();
    for decision in PresentationDecision::ALL {
        assert!(
            present.contains(&decision),
            "no entry exercises {}",
            decision.as_str()
        );
    }
}

#[test]
fn downgrade_reasons_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PresentationDowngradeReason> = packet
        .nav_entries
        .iter()
        .flat_map(|e| e.downgrade_reasons.iter().copied())
        .collect();
    for reason in PresentationDowngradeReason::ALL {
        assert!(
            present.contains(&reason),
            "no entry exercises {}",
            reason.as_str()
        );
    }
}

#[test]
fn recovery_paths_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<PresentationRecoveryPath> =
        packet.nav_entries.iter().map(|e| e.recovery_path).collect();
    for path in PresentationRecoveryPath::ALL {
        assert!(
            present.contains(&path),
            "no entry exercises {}",
            path.as_str()
        );
    }
}

#[test]
fn facet_availabilities_are_exhaustive() {
    let packet = packet();
    let present: BTreeSet<FacetAvailability> = packet
        .shared_inspector
        .facets
        .iter()
        .map(|b| b.availability)
        .collect();
    for availability in FacetAvailability::ALL {
        assert!(
            present.contains(&availability),
            "no facet exercises {}",
            availability.as_str()
        );
    }
}

#[test]
fn sections_kinds_sources_are_exhaustive() {
    let packet = packet();
    let sections: BTreeSet<NavSection> = packet.nav_entries.iter().map(|e| e.section).collect();
    for section in NavSection::ALL {
        assert!(
            sections.contains(&section),
            "no entry in {}",
            section.as_str()
        );
    }
    let kinds: BTreeSet<CenterSurfaceKind> = packet
        .nav_entries
        .iter()
        .map(|e| e.center_surface_kind)
        .collect();
    for kind in CenterSurfaceKind::ALL {
        assert!(kinds.contains(&kind), "no entry renders {}", kind.as_str());
    }
    let sources: BTreeSet<IntegrationSource> = packet
        .nav_entries
        .iter()
        .flat_map(|e| e.integration_sources.iter().copied())
        .collect();
    for source in IntegrationSource::ALL {
        assert!(
            sources.contains(&source),
            "no entry reuses {}",
            source.as_str()
        );
    }
}

#[test]
fn degraded_facet_narrows_dependent_entry() {
    let packet = packet();
    let entry = packet
        .entry_for(SupportModule::Doctor)
        .expect("doctor entry");
    assert_eq!(
        packet
            .shared_inspector
            .facet_availability(InspectorFacet::Policy),
        FacetAvailability::Degraded
    );
    assert_eq!(entry.presentation, PresentationDecision::Narrowed);
    assert_eq!(
        entry.recovery_path,
        PresentationRecoveryPath::RestoreInspectorFacet
    );
    assert_eq!(
        entry.downgrade_reasons,
        vec![PresentationDowngradeReason::InspectorFacetDegraded]
    );
}

#[test]
fn unwired_facet_withholds_dependent_entry() {
    let packet = packet();
    let entry = packet.entry_for(SupportModule::Index).expect("index entry");
    assert_eq!(
        packet
            .shared_inspector
            .facet_availability(InspectorFacet::Residency),
        FacetAvailability::Unwired
    );
    assert_eq!(entry.presentation, PresentationDecision::Withheld);
    assert_eq!(
        entry.recovery_path,
        PresentationRecoveryPath::RestoreInspectorFacet
    );
    assert!(entry
        .downgrade_reasons
        .contains(&PresentationDowngradeReason::InspectorFacetUnwired));
    assert!(entry.offered_actions.is_empty());
}

#[test]
fn accessibility_gap_withholds_entry_and_restores_accessibility_first() {
    let packet = packet();
    let entry = packet
        .entry_for(SupportModule::Network)
        .expect("network entry");
    assert!(!entry.accessibility_complete());
    assert_eq!(entry.presentation, PresentationDecision::Withheld);
    // Accessibility is the harder invariant, so it is restored before the degraded facet.
    assert_eq!(
        entry.recovery_path,
        PresentationRecoveryPath::RestoreAccessibility
    );
    assert!(entry
        .downgrade_reasons
        .contains(&PresentationDowngradeReason::AccessibilityUnmet));
}

#[test]
fn validate_flags_overstated_presentation() {
    let mut packet = packet();
    let inspector = packet.shared_inspector.clone();
    if let Some(entry) = packet
        .nav_entries
        .iter_mut()
        .find(|e| e.effective_presentation(&inspector) != PresentationDecision::Presented)
    {
        entry.presentation = PresentationDecision::Presented;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterLayoutViolation::OverstatedPresentation { .. }
        )));
    }
}

#[test]
fn validate_flags_inaccessible_region() {
    let mut packet = packet();
    if let Some(region) = packet.regions.first_mut() {
        region
            .accessibility
            .retain(|g| *g != AccessibilityGuarantee::KeyboardComplete);
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterLayoutViolation::RegionNotAccessible { .. }
        )));
    }
}

#[test]
fn validate_flags_inspector_that_stops_persisting() {
    let mut packet = packet();
    packet.shared_inspector.persists_across_modules = false;
    assert!(packet
        .validate()
        .contains(&M5SupportCenterLayoutViolation::InspectorNotShared));
}

#[test]
fn validate_flags_missing_inspector_facet() {
    let mut packet = packet();
    packet
        .shared_inspector
        .facets
        .retain(|b| b.facet != InspectorFacet::Residency);
    let violations = packet.validate();
    assert!(violations.iter().any(|v| matches!(
        v,
        M5SupportCenterLayoutViolation::MissingInspectorFacet { .. }
    )));
}

#[test]
fn validate_flags_withheld_entry_offering_actions() {
    let mut packet = packet();
    let inspector = packet.shared_inspector.clone();
    if let Some(entry) = packet
        .nav_entries
        .iter_mut()
        .find(|e| e.effective_presentation(&inspector) == PresentationDecision::Withheld)
    {
        entry.offered_actions.push("inspect anyway".to_owned());
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterLayoutViolation::WithheldEntryOffersActions { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_consumer_binding() {
    let mut packet = packet();
    packet
        .consumer_bindings
        .retain(|b| b.consumer_surface != LayoutConsumerSurface::DocsHelp);
    assert!(packet.validate().iter().any(|v| matches!(
        v,
        M5SupportCenterLayoutViolation::MissingConsumerBinding { .. }
    )));
}

#[test]
fn validate_flags_binding_that_stops_narrowing() {
    let mut packet = packet();
    if let Some(binding) = packet.consumer_bindings.first_mut() {
        binding.narrows_on_downgrade = false;
        assert!(packet.validate().iter().any(|v| matches!(
            v,
            M5SupportCenterLayoutViolation::ConsumerBindingDrift { .. }
        )));
    }
}

#[test]
fn validate_flags_missing_module() {
    let mut packet = packet();
    packet
        .nav_entries
        .retain(|e| e.module != SupportModule::Doctor);
    assert!(packet
        .validate()
        .iter()
        .any(|v| matches!(v, M5SupportCenterLayoutViolation::MissingModule { .. })));
}

#[test]
fn validate_flags_summary_mismatch() {
    let mut packet = packet();
    packet.summary.total_modules = packet.summary.total_modules.wrapping_add(1);
    assert!(packet
        .validate()
        .contains(&M5SupportCenterLayoutViolation::SummaryMismatch));
}

#[test]
fn unwiring_a_facet_withholds_every_dependent_entry() {
    // Proves the shared inspector is one source of facet truth: unwiring the build facet withholds
    // every entry that depends on it at once.
    let mut packet = packet();
    if let Some(binding) = packet
        .shared_inspector
        .facets
        .iter_mut()
        .find(|b| b.facet == InspectorFacet::Build)
    {
        binding.availability = FacetAvailability::Unwired;
    }
    let inspector = packet.shared_inspector.clone();
    for entry in &packet.nav_entries {
        if entry.required_facets.contains(&InspectorFacet::Build) {
            assert_eq!(
                entry.effective_presentation(&inspector),
                PresentationDecision::Withheld,
                "{} should be withheld when build is unwired",
                entry.entry_id
            );
        }
    }
}

#[test]
fn tokens_are_stable() {
    assert_eq!(LayoutRegion::RightInspector.as_str(), "right_inspector");
    assert_eq!(NavSection::IntakeExport.as_str(), "intake_export");
    assert_eq!(
        CenterSurfaceKind::IntakeAndExport.as_str(),
        "intake_and_export"
    );
    assert_eq!(
        IntegrationSource::SchemaRegistryState.as_str(),
        "schema_registry_state"
    );
    assert_eq!(InspectorFacet::Residency.as_str(), "residency");
    assert_eq!(FacetAvailability::Unwired.as_str(), "unwired");
    assert_eq!(
        AccessibilityGuarantee::ReducedMotionSafe.as_str(),
        "reduced_motion_safe"
    );
    assert_eq!(PresentationDecision::Withheld.as_str(), "withheld");
    assert_eq!(
        PresentationDowngradeReason::InspectorFacetUnwired.as_str(),
        "inspector_facet_unwired"
    );
    assert_eq!(PresentationRecoveryPath::NoneNeeded.as_str(), "none");
    assert_eq!(
        LayoutConsumerSurface::SupportExport.as_str(),
        "support_export"
    );
}

#[test]
fn facet_ceilings_hold_for_each_state() {
    assert_eq!(
        FacetAvailability::Wired.presentation_ceiling(),
        PresentationDecision::Presented
    );
    assert_eq!(
        FacetAvailability::Degraded.presentation_ceiling(),
        PresentationDecision::Narrowed
    );
    assert_eq!(
        FacetAvailability::Unwired.presentation_ceiling(),
        PresentationDecision::Withheld
    );
}
