use super::*;

#[test]
fn seeded_manifest_validates() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    validate_m5_tour_and_glossary_packages(&manifest)
        .expect("seeded tour/glossary manifest must pass validation");
}

#[test]
fn covers_every_family_with_a_glossary_pack_and_tour_package() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    assert_eq!(
        manifest.glossary_packs.len(),
        M5LearningSurfaceFamily::ALL.len()
    );
    assert_eq!(
        manifest.tour_packages.len(),
        M5LearningSurfaceFamily::ALL.len()
    );
    for family in M5LearningSurfaceFamily::ALL {
        assert!(
            manifest.glossary_pack(&glossary_pack_id(family)).is_some(),
            "missing glossary pack for {}",
            family.as_str()
        );
        assert!(
            manifest.tour_package(&tour_package_id(family)).is_some(),
            "missing tour package for {}",
            family.as_str()
        );
    }
}

#[test]
fn every_step_references_a_stable_object() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    for pkg in &manifest.tour_packages {
        for step in &pkg.steps {
            assert!(
                !step.relies_on_coordinates_only(),
                "{} relies on coordinates alone",
                step.step_id
            );
            assert!(!step.stable_targets.is_empty());
        }
    }
}

#[test]
fn scope_widening_is_named_when_it_occurs() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    let scaffold = manifest
        .tour_package(&tour_package_id(M5LearningSurfaceFamily::TemplateScaffold))
        .expect("scaffold tour");
    let widening_step = scaffold
        .steps
        .iter()
        .find(|s| s.scope_widening.widens)
        .expect("scaffold tour widens scope in at least one step");
    assert!(widening_step.scope_widening.qualifies_stable());
    assert!(widening_step.scope_widening.from_scope_ref.is_some());
    assert!(widening_step.scope_widening.to_scope_ref.is_some());
    assert!(widening_step.scope_widening.named_reason.is_some());
    // A named widening does not by itself narrow the package.
    assert_eq!(scaffold.verdict, QualificationVerdict::QualifiedStable);
}

#[test]
fn cached_and_local_only_packages_narrow_but_stay_disclosed() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    let companion = manifest
        .tour_package(&tour_package_id(M5LearningSurfaceFamily::Companion))
        .expect("companion tour");
    assert_eq!(companion.freshness_state, FreshnessState::CachedDisclosed);
    assert_eq!(companion.verdict, QualificationVerdict::NarrowedBeta);
    assert!(companion.mirror_parity.explicit_freshness_disclosed);

    let preview = manifest
        .glossary_pack(&glossary_pack_id(M5LearningSurfaceFamily::Preview))
        .expect("preview glossary");
    assert_eq!(preview.freshness_state, FreshnessState::LocalOnlyDisclosed);
    assert_eq!(preview.verdict, QualificationVerdict::NarrowedBeta);
    assert!(preview.mirror_parity.explicit_freshness_disclosed);
    assert!(!preview.mirror_parity.silent_dead_link_on_stale);
}

#[test]
fn live_families_qualify_stable() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    let notebook = manifest
        .tour_package(&tour_package_id(M5LearningSurfaceFamily::Notebook))
        .expect("notebook tour");
    assert_eq!(notebook.freshness_state, FreshnessState::LiveAuthoritative);
    assert_eq!(notebook.verdict, QualificationVerdict::QualifiedStable);
}

#[test]
fn mirror_synced_docs_browser_stays_stable() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    let docs = manifest
        .glossary_pack(&glossary_pack_id(M5LearningSurfaceFamily::DocsBrowser))
        .expect("docs_browser glossary");
    assert_eq!(docs.freshness_state, FreshnessState::MirrorSyncedDisclosed);
    assert_eq!(docs.verdict, QualificationVerdict::QualifiedStable);
}

#[test]
fn overall_verdict_reflects_narrowed_packages() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    assert_eq!(manifest.overall_verdict, QualificationVerdict::NarrowedBeta);
    assert!(!manifest.overall_narrowing_reasons.is_empty());
}

#[test]
fn export_and_reopen_preserves_targets_and_citations() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let reopened = reopen_manifest_from_json(&json).expect("reopen");
    assert_eq!(manifest, reopened);

    for (orig, back) in manifest
        .tour_packages
        .iter()
        .zip(reopened.tour_packages.iter())
    {
        assert_eq!(orig.target_ref_fingerprint(), back.target_ref_fingerprint());
        assert_eq!(
            orig.citation_ref_fingerprint(),
            back.citation_ref_fingerprint()
        );
        assert_eq!(orig.version, back.version);
    }
    for (orig, back) in manifest
        .glossary_packs
        .iter()
        .zip(reopened.glossary_packs.iter())
    {
        assert_eq!(orig.target_ref_fingerprint(), back.target_ref_fingerprint());
        assert_eq!(
            orig.citation_ref_fingerprint(),
            back.citation_ref_fingerprint()
        );
        assert_eq!(orig.version, back.version);
    }
}

#[test]
fn localization_changes_labels_but_not_target_identity() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    let notebook = manifest
        .tour_package(&tour_package_id(M5LearningSurfaceFamily::Notebook))
        .expect("notebook tour");

    let base_targets = notebook.target_ref_fingerprint();
    let base_citations = notebook.citation_ref_fingerprint();

    for locale in ["fr-FR", "ja-JP"] {
        let labels = notebook
            .localized_labels(locale)
            .unwrap_or_else(|| panic!("missing {locale} overlay"));
        // The overlay localizes a label for each step id.
        for step in &notebook.steps {
            assert!(
                labels.contains_key(&step.step_id),
                "{locale} overlay missing label for {}",
                step.step_id
            );
        }
        // Target identity and citations are unchanged by localization.
        assert_eq!(notebook.target_ref_fingerprint(), base_targets);
        assert_eq!(notebook.citation_ref_fingerprint(), base_citations);
    }
}

#[test]
fn tour_prerequisite_resolves_to_its_glossary_pack() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    for pkg in &manifest.tour_packages {
        assert!(
            manifest.glossary_pack(&pkg.glossary_pack_ref).is_some(),
            "{} glossary ref does not resolve",
            pkg.package_id
        );
        for prereq in &pkg.prerequisite_package_refs {
            assert!(
                manifest.known_package_ids().contains(prereq),
                "{} prerequisite {} does not resolve",
                pkg.package_id,
                prereq
            );
        }
    }
}

#[test]
fn validation_catches_coordinate_only_step() {
    let mut manifest = seeded_m5_tour_and_glossary_packages();
    manifest.tour_packages[0].steps[0].stable_targets.clear();
    manifest.tour_packages[0].sync_verdict();
    let errors = validate_m5_tour_and_glossary_packages(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("coordinates alone")));
}

#[test]
fn validation_catches_unnamed_scope_widening() {
    let mut manifest = seeded_m5_tour_and_glossary_packages();
    manifest.tour_packages[0].steps[0].scope_widening = ScopeWidening {
        widens: true,
        from_scope_ref: None,
        to_scope_ref: None,
        named_reason: None,
    };
    manifest.tour_packages[0].sync_verdict();
    let errors = validate_m5_tour_and_glossary_packages(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("widens scope without naming")));
}

#[test]
fn validation_catches_locale_overlay_dropping_identity() {
    let mut manifest = seeded_m5_tour_and_glossary_packages();
    manifest.glossary_packs[0].locale_overlays[0].preserves_target_identity = false;
    manifest.glossary_packs[0].sync_verdict();
    let errors = validate_m5_tour_and_glossary_packages(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("drops target identity")));
}

#[test]
fn validation_catches_freshness_masquerade() {
    let mut manifest = seeded_m5_tour_and_glossary_packages();
    // A cached pack that does not disclose freshness would masquerade as live.
    let companion = manifest
        .glossary_packs
        .iter_mut()
        .find(|p| p.family == M5LearningSurfaceFamily::Companion)
        .expect("companion glossary");
    companion.mirror_parity.explicit_freshness_disclosed = false;
    companion.sync_verdict();
    let errors = validate_m5_tour_and_glossary_packages(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("masquerade")));
}

#[test]
fn validation_catches_freshness_label_disagreement() {
    let mut manifest = seeded_m5_tour_and_glossary_packages();
    manifest.glossary_packs[0].mirror_parity.freshness_label = "cached_disclosed".to_string();
    manifest.glossary_packs[0].sync_verdict();
    let errors = validate_m5_tour_and_glossary_packages(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("disagrees")));
}

#[test]
fn validation_catches_unresolved_prerequisite() {
    let mut manifest = seeded_m5_tour_and_glossary_packages();
    manifest.tour_packages[0]
        .prerequisite_package_refs
        .push("learning:m5:glossary_pack:does_not_exist:v1".to_string());
    let errors = validate_m5_tour_and_glossary_packages(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("unresolved prerequisite")));
}

#[test]
fn validation_catches_prerequisite_cycle() {
    let mut manifest = seeded_m5_tour_and_glossary_packages();
    let a = manifest.glossary_packs[0].pack_id.clone();
    let b = manifest.glossary_packs[1].pack_id.clone();
    manifest.glossary_packs[0].prerequisite_pack_refs = vec![b.clone()];
    manifest.glossary_packs[1].prerequisite_pack_refs = vec![a.clone()];
    let errors = validate_m5_tour_and_glossary_packages(&manifest).unwrap_err();
    assert!(errors.iter().any(|e| e.message.contains("cycle")));
}

#[test]
fn validation_catches_conflated_step() {
    let mut manifest = seeded_m5_tour_and_glossary_packages();
    manifest.tour_packages[0].steps[0].explain_apply_class = ExplainApplyClass::Conflated;
    manifest.tour_packages[0].sync_verdict();
    let errors = validate_m5_tour_and_glossary_packages(&manifest).unwrap_err();
    assert!(errors
        .iter()
        .any(|e| e.message.contains("conflates explain/apply")));
}

#[test]
fn manifest_serializes_and_roundtrips() {
    let manifest = seeded_m5_tour_and_glossary_packages();
    let json = serde_json::to_string_pretty(&manifest).expect("serialize");
    let back: M5TourAndGlossaryPackageManifest = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(manifest, back);
}
