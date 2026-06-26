use super::*;

#[test]
fn seeded_catalog_validates() {
    let catalog = seeded_boundary_wording_catalog();
    assert!(catalog.validate().is_empty(), "{:?}", catalog.validate());
    assert_eq!(catalog.catalog_id, BOUNDARY_WORDING_CATALOG_ID);
}

#[test]
fn inventories_match_canonical() {
    let catalog = seeded_boundary_wording_catalog();
    assert_eq!(catalog.term_inventory.len(), BoundaryTerm::ALL.len());
    assert_eq!(
        catalog.actual_posture_inventory.len(),
        ActualBoundaryPosture::ALL.len()
    );
    assert_eq!(catalog.surface_inventory.len(), BoundarySurface::ALL.len());
    assert_eq!(
        catalog.claim_kind_inventory.len(),
        BoundaryClaimKind::ALL.len()
    );
    assert_eq!(
        catalog.implication_inventory.len(),
        BoundaryImplication::ALL.len()
    );
    assert_eq!(
        catalog.implication_posture_inventory.len(),
        ImplicationPosture::ALL.len()
    );
    assert_eq!(
        catalog.alternative_path_inventory.len(),
        AlternativePath::ALL.len()
    );
}

#[test]
fn inventory_drift_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.term_inventory.pop();
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::InventoryDrift));
}

#[test]
fn coverage_spans_terms_surfaces_kinds_postures_and_alternatives() {
    let catalog = seeded_boundary_wording_catalog();
    let terms: std::collections::BTreeSet<_> = catalog.entries.iter().map(|e| e.term).collect();
    let surfaces: std::collections::BTreeSet<_> =
        catalog.entries.iter().map(|e| e.surface).collect();
    let kinds: std::collections::BTreeSet<_> =
        catalog.entries.iter().map(|e| e.claim_kind).collect();
    let postures: std::collections::BTreeSet<_> = catalog
        .entries
        .iter()
        .map(|e| e.actual_boundary_posture)
        .collect();
    let impl_postures: std::collections::BTreeSet<_> = catalog
        .entries
        .iter()
        .flat_map(|e| e.implications.iter().map(|s| s.posture))
        .collect();
    let alts: std::collections::BTreeSet<_> = catalog
        .entries
        .iter()
        .flat_map(|e| e.alternative_paths.iter().map(|a| a.path))
        .collect();
    assert_eq!(terms.len(), BoundaryTerm::ALL.len());
    assert_eq!(surfaces.len(), BoundarySurface::ALL.len());
    assert_eq!(kinds.len(), BoundaryClaimKind::ALL.len());
    assert_eq!(postures.len(), ActualBoundaryPosture::ALL.len());
    assert_eq!(impl_postures.len(), ImplicationPosture::ALL.len());
    assert_eq!(alts.len(), AlternativePath::ALL.len());
}

#[test]
fn coverage_gap_fails_when_a_term_is_missing() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.entries.retain(|e| e.term != BoundaryTerm::Trial);
    assert!(catalog.validate().contains(&BoundaryViolation::CoverageGap));
}

#[test]
fn entry_ids_concepts_and_refs_are_locale_neutral() {
    let catalog = seeded_boundary_wording_catalog();
    let ok = |t: &str| {
        !t.is_empty()
            && t.chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_' || c == '.')
    };
    for entry in &catalog.entries {
        assert!(ok(&entry.entry_id), "entry id: {}", entry.entry_id);
        assert!(ok(&entry.concept_id), "concept id: {}", entry.concept_id);
        assert!(ok(&entry.source_ref), "source ref: {}", entry.source_ref);
        if let Some(support) = &entry.support_metadata_ref {
            assert!(ok(support), "support ref: {support}");
        }
        for disclosure in &entry.alternative_paths {
            if let Some(reference) = &disclosure.reference_ref {
                assert!(ok(reference), "alternative ref: {reference}");
            }
        }
    }
}

#[test]
fn non_locale_neutral_concept_id_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.entries[0].concept_id = "Concept.Cloud Sync".to_owned();
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::EntryTokenNotLocaleNeutral));
}

#[test]
fn duplicate_entry_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    let clone = catalog.entries[0].clone();
    catalog.entries.push(clone);
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::DuplicateEntry));
}

#[test]
fn entry_incomplete_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.entries[0].source_ref = "  ".to_owned();
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::EntryIncomplete));
}

#[test]
fn narrowing_without_support_metadata_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    let narrowing = catalog
        .entries
        .iter_mut()
        .find(|e| e.claim_kind == BoundaryClaimKind::NarrowsBoundary)
        .expect("narrowing entry present");
    narrowing.support_metadata_ref = None;
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::NarrowingWideningMissingSupportMetadata));
}

#[test]
fn widening_without_support_metadata_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    let widening = catalog
        .entries
        .iter_mut()
        .find(|e| e.claim_kind == BoundaryClaimKind::WidensBoundary)
        .expect("widening entry present");
    widening.support_metadata_ref = None;
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::NarrowingWideningMissingSupportMetadata));
}

#[test]
fn every_entry_explains_all_five_implications() {
    let catalog = seeded_boundary_wording_catalog();
    for entry in &catalog.entries {
        for dimension in BoundaryImplication::ALL {
            assert!(
                entry.posture_for(dimension).is_some(),
                "entry {} missing {dimension:?}",
                entry.entry_id
            );
        }
    }
}

#[test]
fn missing_implication_dimension_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.entries[0]
        .implications
        .retain(|s| s.dimension != BoundaryImplication::Data);
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::ImplicationDimensionMissing));
}

#[test]
fn boundary_overstatement_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    // Claim "Local only" for a commercial-paid capability — overstates locality.
    let paid = catalog
        .entries
        .iter_mut()
        .find(|e| e.actual_boundary_posture == ActualBoundaryPosture::CommercialPaid)
        .expect("paid entry present");
    paid.term = BoundaryTerm::LocalOnly;
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::BoundaryOverstatesActualPosture));
}

#[test]
fn no_seeded_entry_overstates_the_boundary() {
    let catalog = seeded_boundary_wording_catalog();
    for entry in &catalog.entries {
        assert!(
            !entry.overstates_boundary(),
            "entry {} overstates: term {} actual {}",
            entry.entry_id,
            entry.term.as_str(),
            entry.actual_boundary_posture.as_str()
        );
    }
}

#[test]
fn vendor_dependence_without_local_alternative_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    // A managed/paid claim with a local-capable core that hides every local/open
    // alternative implies vendor dependence.
    let managed = catalog
        .entries
        .iter_mut()
        .find(|e| e.introduces_managed_or_paid && e.core_workflow_remains_local)
        .expect("managed entry present");
    for disclosure in &mut managed.alternative_paths {
        if disclosure.path.is_local_or_open() {
            disclosure.available = false;
            disclosure.reference_ref = None;
        }
    }
    let violations = catalog.validate();
    assert!(violations.contains(&BoundaryViolation::ImpliesVendorDependenceWhenCoreLocal));
}

#[test]
fn managed_introduction_without_export_route_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    let managed = catalog
        .entries
        .iter_mut()
        .find(|e| e.introduces_managed_or_paid)
        .expect("managed entry present");
    for statement in &mut managed.implications {
        if statement.dimension == BoundaryImplication::Export {
            statement.posture = ImplicationPosture::NotRequired;
        }
    }
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::ManagedOrPaidMissingExportOrRollback));
}

#[test]
fn upgrade_surface_without_alternative_disclosure_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    let upgrade = catalog
        .entries
        .iter_mut()
        .find(|e| {
            e.surface == BoundarySurface::AccountUpgradePrompt && e.introduces_managed_or_paid
        })
        .expect("account/upgrade entry present");
    upgrade
        .alternative_paths
        .retain(|a| !a.path.is_local_or_open());
    let violations = catalog.validate();
    assert!(violations.contains(&BoundaryViolation::UpgradeSurfaceMissingAlternativeDisclosure));
}

#[test]
fn available_alternative_without_reference_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    let entry = &mut catalog.entries[0];
    entry.alternative_paths[0].reference_ref = None;
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::AlternativeDisclosureIncomplete));
}

#[test]
fn parity_holds_for_shared_concepts() {
    let catalog = seeded_boundary_wording_catalog();
    assert!(
        catalog.lint_parity().is_empty(),
        "unexpected parity drift: {:?}",
        catalog.lint_parity()
    );
}

#[test]
fn term_drift_across_a_concept_is_caught() {
    let mut catalog = seeded_boundary_wording_catalog();
    // Make one surface call cloud sync "Hosted" while the others call it "Managed".
    let drift = catalog
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry.cloud_sync.help_about")
        .expect("entry present");
    drift.term = BoundaryTerm::Hosted;
    let findings = catalog.lint_parity();
    assert!(
        findings
            .iter()
            .any(|f| f.kind == ParityFindingKind::TermDrift),
        "expected term drift, got {findings:?}"
    );
    assert!(catalog.validate().contains(&BoundaryViolation::ParityDrift));
}

#[test]
fn implication_posture_drift_across_a_concept_is_caught() {
    let mut catalog = seeded_boundary_wording_catalog();
    let drift = catalog
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry.premium_models.help_about")
        .expect("entry present");
    for statement in &mut drift.implications {
        if statement.dimension == BoundaryImplication::Network {
            statement.posture = ImplicationPosture::NotRequired;
        }
    }
    let findings = catalog.lint_parity();
    assert!(findings
        .iter()
        .any(|f| f.kind == ParityFindingKind::ImplicationPostureDrift));
    assert!(catalog.validate().contains(&BoundaryViolation::ParityDrift));
}

#[test]
fn alternative_availability_drift_across_a_concept_is_caught() {
    let mut catalog = seeded_boundary_wording_catalog();
    let drift = catalog
        .entries
        .iter_mut()
        .find(|e| e.entry_id == "entry.cloud_sync.onboarding")
        .expect("entry present");
    if let Some(byok) = drift
        .alternative_paths
        .iter_mut()
        .find(|a| a.path == AlternativePath::Byok)
    {
        byok.available = false;
    }
    assert!(catalog
        .lint_parity()
        .iter()
        .any(|f| f.kind == ParityFindingKind::AlternativeAvailabilityDrift));
}

#[test]
fn shared_concepts_span_minimum_surfaces() {
    let catalog = seeded_boundary_wording_catalog();
    let surfaces = catalog.concept_surfaces();
    for concept_id in &catalog.shared_concept_ids {
        let spans = surfaces.get(concept_id).map(|s| s.len()).unwrap_or(0);
        assert!(
            spans >= SHARED_CONCEPT_MIN_SURFACES,
            "shared concept {concept_id} only spans {spans} surfaces"
        );
    }
}

#[test]
fn shared_concept_below_minimum_surfaces_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    // Drop cloud-sync down to a single surface but keep it in the shared list.
    catalog
        .entries
        .retain(|e| e.concept_id != "concept.cloud_sync" || e.surface == BoundarySurface::Settings);
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::SharedConceptParityInsufficient));
}

#[test]
fn empty_shared_concepts_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.shared_concept_ids.clear();
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::SharedConceptParityInsufficient));
}

#[test]
fn missing_source_contracts_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.source_contract_refs.clear();
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::MissingSourceContracts));
}

#[test]
fn trust_review_incomplete_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog
        .trust_review
        .never_pressures_away_from_local_or_open_path = false;
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::TrustReviewIncomplete));
}

#[test]
fn parity_projection_incomplete_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog
        .parity_projection
        .marketplace_resolves_through_catalog = false;
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::ParityProjectionIncomplete));
}

#[test]
fn proof_freshness_incomplete_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.proof_freshness.proof_freshness_slo_hours = 0;
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::ProofFreshnessIncomplete));
}

#[test]
fn release_posture_incomplete_fails() {
    let mut catalog = seeded_boundary_wording_catalog();
    catalog.release_posture.mirror_offline_parity_required = false;
    assert!(catalog
        .validate()
        .contains(&BoundaryViolation::ReleasePostureIncomplete));
}

#[test]
fn render_boundary_explanation_uses_controlled_vocabulary() {
    let catalog = seeded_boundary_wording_catalog();
    let line = catalog
        .render_boundary_explanation("entry.cloud_sync.account_upgrade")
        .expect("entry resolves");
    assert!(line.contains("term: managed"));
    assert!(line.contains("actual: managed_optional"));
    assert!(line.contains("surface: account_upgrade_prompt"));
    assert!(line.contains("export: retained"));
    assert!(line.contains("rollback: retained"));
    // Discloses the local/open alternatives and the support metadata it is anchored to.
    assert!(line.contains("alternatives: "));
    assert!(line.contains("local_only"));
    assert!(line.contains("support: support.metadata.cloud_sync_compatibility"));
}

#[test]
fn markdown_summary_lists_concepts_and_parity() {
    let catalog = seeded_boundary_wording_catalog();
    let summary = catalog.render_markdown_summary();
    for entry in &catalog.entries {
        assert!(
            summary.contains(&entry.entry_id),
            "summary missing {}",
            entry.entry_id
        );
    }
    assert!(summary.contains("Copy-parity lint"));
    assert!(summary.contains("No cross-surface boundary drift detected."));
}

#[test]
fn localized_overlay_preserves_machine_identity() {
    let canonical = seeded_boundary_wording_catalog();
    let localized = seeded_boundary_wording_catalog_localized();
    assert!(
        localized.validate().is_empty(),
        "localized overlay failed validation: {:?}",
        localized.validate()
    );
    assert_ne!(canonical.reference_locale, localized.reference_locale);

    let mut any_prose_changed = false;
    for (canon, loc) in canonical.entries.iter().zip(localized.entries.iter()) {
        assert_eq!(canon.entry_id, loc.entry_id);
        assert_eq!(canon.concept_id, loc.concept_id);
        assert_eq!(canon.term, loc.term);
        assert_eq!(canon.surface, loc.surface);
        assert_eq!(canon.actual_boundary_posture, loc.actual_boundary_posture);
        assert_eq!(canon.support_metadata_ref, loc.support_metadata_ref);
        assert_eq!(
            canon
                .implications
                .iter()
                .map(|s| (s.dimension, s.posture))
                .collect::<Vec<_>>(),
            loc.implications
                .iter()
                .map(|s| (s.dimension, s.posture))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            canon
                .alternative_paths
                .iter()
                .map(|a| (a.path, a.available, a.reference_ref.clone()))
                .collect::<Vec<_>>(),
            loc.alternative_paths
                .iter()
                .map(|a| (a.path, a.available, a.reference_ref.clone()))
                .collect::<Vec<_>>()
        );
        if canon.canonical_text != loc.canonical_text {
            any_prose_changed = true;
        }
    }
    assert!(any_prose_changed, "localized overlay changed no prose");
}

#[test]
fn offline_mirror_variant_validates_and_keeps_identity() {
    let canonical = seeded_boundary_wording_catalog();
    let mirror = seeded_boundary_wording_catalog_offline_mirror();
    assert!(mirror.validate().is_empty(), "{:?}", mirror.validate());
    assert_eq!(mirror.entries, canonical.entries);
    assert_ne!(mirror.catalog_id, canonical.catalog_id);
}

#[test]
fn checked_support_export_validates() {
    let catalog = current_boundary_wording_catalog_export()
        .expect("checked boundary wording catalog export validates");
    assert_eq!(catalog.catalog_id, BOUNDARY_WORDING_CATALOG_ID);
}

#[test]
fn checked_support_export_matches_seed() {
    let from_disk = current_boundary_wording_catalog_export()
        .expect("checked boundary wording catalog export validates");
    assert_eq!(
        from_disk,
        seeded_boundary_wording_catalog(),
        "checked support export drifted from the seed builder"
    );
}

#[test]
fn checked_fixtures_validate() {
    for raw in [
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-boundary-wording/localized_overlay.json"
        )),
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/content/m5-boundary-wording/offline_mirror.json"
        )),
    ] {
        let catalog: BoundaryWordingCatalog =
            serde_json::from_str(raw).expect("fixture parses as catalog");
        assert!(
            catalog.validate().is_empty(),
            "fixture failed validation: {:?}",
            catalog.validate()
        );
    }
}

#[test]
fn export_carries_no_forbidden_material() {
    let json = seeded_boundary_wording_catalog().export_safe_json();
    let lower = json.to_lowercase();
    assert!(!lower.contains("api_key"));
    assert!(!lower.contains("password"));
    assert!(!lower.contains("bearer "));
}
