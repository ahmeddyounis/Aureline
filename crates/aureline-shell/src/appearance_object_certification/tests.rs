//! Unit tests for the M5 appearance-object certification capstone.

use super::*;

fn seed() -> AppearanceObjectCertificationReport {
    seeded_appearance_object_certification_report()
}

#[test]
fn seeded_report_is_clean_and_validates() {
    let report = seed();
    assert!(report.report_clean, "seeded report must be clean");
    assert!(report.blocking_findings.is_empty());
    validate_appearance_object_certification_report(&report).expect("seeded report must validate");
}

#[test]
fn envelope_uses_frozen_contract_constants() {
    let report = seed();
    assert_eq!(report.record_kind, M5_APPEARANCE_CERT_REPORT_RECORD_KIND);
    assert_eq!(report.schema_version, M5_APPEARANCE_CERT_SCHEMA_VERSION);
    assert_eq!(
        report.shared_contract_ref,
        M5_APPEARANCE_CERT_SHARED_CONTRACT_REF
    );
    assert_eq!(report.report_id, M5_APPEARANCE_CERT_REPORT_ID);
    assert_eq!(
        report.source_schema_ref,
        M5_APPEARANCE_CERT_SOURCE_SCHEMA_REF
    );
    assert_eq!(
        report.published_report_ref,
        M5_APPEARANCE_CERT_PUBLISHED_REPORT_REF
    );
    assert_eq!(
        report.published_doc_ref,
        M5_APPEARANCE_CERT_PUBLISHED_DOC_REF
    );
    assert_eq!(report.build_identity_ref, SEED_BUILD_IDENTITY_REF);
}

#[test]
fn object_model_index_registers_every_family_from_its_own_constants() {
    let report = seed();
    assert_eq!(
        report.object_model_index.len(),
        AppearanceObjectFamily::ALL.len()
    );
    for family in AppearanceObjectFamily::ALL {
        let entry = report
            .index_entry(family)
            .unwrap_or_else(|| panic!("index must register {}", family.as_str()));
        // The index source report id must equal the family's own constant.
        assert_eq!(entry.source_report_id, family.source_report_id());
        assert!(!entry.canonical_schema_ref.is_empty());
        assert!(!entry.published_doc_ref.is_empty());
    }
}

#[test]
fn every_claimed_surface_is_certified_across_all_five_families() {
    let report = seed();
    for surface_family in REQUIRED_SURFACE_FAMILIES {
        let surface = report
            .surfaces
            .iter()
            .find(|surface| surface.surface_family == surface_family)
            .unwrap_or_else(|| panic!("surface {} must be certified", surface_family.as_str()));
        for family in AppearanceObjectFamily::ALL {
            assert!(
                surface.family(family).is_some(),
                "surface {} omits family {}",
                surface_family.as_str(),
                family.as_str()
            );
        }
    }
}

#[test]
fn every_family_certification_is_backed_by_the_canonical_index_report() {
    let report = seed();
    for surface in &report.surfaces {
        for certification in &surface.family_certifications {
            let entry = report
                .index_entry(certification.object_family)
                .expect("index entry exists");
            assert_eq!(
                certification.source_report_id,
                entry.source_report_id,
                "surface {} family {} cites an unbacked report",
                surface.certification_id,
                certification.object_family.as_str()
            );
        }
    }
}

#[test]
fn claim_scope_is_derived_not_asserted() {
    let report = seed();
    for surface in &report.surfaces {
        assert_eq!(
            surface.certified_claim_scope,
            surface.recompute_claim_scope(),
            "surface {} declares a stale claim scope",
            surface.certification_id
        );
    }
}

#[test]
fn seed_exercises_full_and_narrowed_scopes() {
    let report = seed();
    assert!(
        report.certified_full_surface_count >= 1,
        "seed must show certified-full surfaces"
    );
    assert!(
        report.narrowed_surface_count >= 1,
        "seed must show auto-narrowed surfaces"
    );
    assert_eq!(
        report.blocked_surface_count, 0,
        "seed must have no blocked surfaces"
    );
    assert!(report.all_surfaces_publishable);
    assert_eq!(
        report.surface_count,
        report.certified_full_surface_count
            + report.narrowed_surface_count
            + report.blocked_surface_count
    );
}

#[test]
fn narrowed_surfaces_disclose_a_reason() {
    let report = seed();
    for surface in &report.surfaces {
        if !matches!(
            surface.certified_claim_scope,
            CertifiedClaimScope::CertifiedFull
        ) {
            let reason = surface.narrowing_reason.as_deref().unwrap_or("").trim();
            assert!(
                !reason.is_empty(),
                "narrowed surface {} hides its reason",
                surface.certification_id
            );
        }
    }
}

#[test]
fn not_applicable_family_keeps_a_surface_certified_full() {
    let report = seed();
    let notebook = report
        .surfaces
        .iter()
        .find(|surface| surface.surface_family == M5AppearanceSurfaceFamily::NotebookCellChrome)
        .expect("notebook surface exists");
    let ext = notebook
        .family(AppearanceObjectFamily::ExtensionAppearanceDescriptor)
        .expect("extension family present");
    assert_eq!(
        ext.certification_status,
        M5QualificationStatus::NotApplicable
    );
    assert!(ext.narrowing_reason.is_some());
    // A not-applicable family must not narrow the surface.
    assert!(!ext.reduces_claim_scope());
    assert_eq!(
        notebook.certified_claim_scope,
        CertifiedClaimScope::CertifiedFull
    );
}

#[test]
fn disclosed_downgrade_auto_narrows_the_surface() {
    let report = seed();
    let preview = report
        .surfaces
        .iter()
        .find(|surface| surface.surface_family == M5AppearanceSurfaceFamily::PreviewRouteBadge)
        .expect("preview surface exists");
    let session = preview
        .family(AppearanceObjectFamily::AppearanceSession)
        .expect("appearance session family present");
    assert!(session.is_certified());
    assert_eq!(
        session.compatibility_state,
        AppearanceCompatibilityState::RestartOrReloadRequired
    );
    assert!(session.downgrade_disclosed);
    assert_eq!(
        preview.certified_claim_scope,
        CertifiedClaimScope::CertifiedNarrowed
    );
}

#[test]
fn support_export_quotes_index_surface_and_family_refs() {
    let report = seed();
    let export = AppearanceObjectCertificationSupportExport::from_report(
        M5_APPEARANCE_CERT_SUPPORT_EXPORT_ID,
        report.clone(),
    );
    let case_ids: std::collections::BTreeSet<&str> =
        export.case_ids.iter().map(String::as_str).collect();
    assert!(case_ids.contains(report.report_id.as_str()));
    assert!(case_ids.contains(report.build_identity_ref.as_str()));
    for entry in &report.object_model_index {
        assert!(case_ids.contains(entry.source_report_id.as_str()));
        assert!(case_ids.contains(entry.canonical_schema_ref.as_str()));
    }
    for surface in &report.surfaces {
        assert!(case_ids.contains(surface.certification_id.as_str()));
        for certification in &surface.family_certifications {
            for evidence_ref in &certification.evidence_refs {
                assert!(case_ids.contains(evidence_ref.as_str()));
            }
        }
    }
}

#[test]
fn hidden_downgrade_is_caught() {
    let mut report = seed();
    // Hide a disclosed downgrade by clearing the disclosure flag.
    let preview = report
        .surfaces
        .iter_mut()
        .find(|surface| surface.surface_family == M5AppearanceSurfaceFamily::PreviewRouteBadge)
        .unwrap();
    let session = preview
        .family_certifications
        .iter_mut()
        .find(|certification| {
            certification.object_family == AppearanceObjectFamily::AppearanceSession
        })
        .unwrap();
    session.downgrade_disclosed = false;
    let rebuilt = build_appearance_object_certification_report(
        report.build_identity_ref.clone(),
        report.release_channel_class.clone(),
        report.object_model_index.clone(),
        report.surfaces.clone(),
    );
    assert!(!rebuilt.report_clean);
    assert!(rebuilt.blocking_findings.iter().any(|finding| matches!(
        finding,
        CertificationBlockingFinding::HiddenDowngrade { .. }
    )));
    // The surface auto-narrows to blocked.
    let preview = rebuilt
        .surfaces
        .iter()
        .find(|surface| surface.surface_family == M5AppearanceSurfaceFamily::PreviewRouteBadge)
        .unwrap();
    assert_eq!(preview.certified_claim_scope, CertifiedClaimScope::Blocked);
}

#[test]
fn stale_evidence_on_a_certified_family_is_caught_and_blocks() {
    let mut surfaces = seeded_surfaces();
    let surface = surfaces
        .iter_mut()
        .find(|surface| surface.surface_family == M5AppearanceSurfaceFamily::ResultGridRow)
        .unwrap();
    surface
        .family_certifications
        .iter_mut()
        .find(|certification| certification.object_family == AppearanceObjectFamily::ThemePackage)
        .unwrap()
        .evidence_freshness = M5EvidenceFreshness::Stale;
    let rebuilt = build_appearance_object_certification_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        canonical_object_model_index(),
        surfaces,
    );
    assert!(rebuilt.blocking_findings.iter().any(|finding| matches!(
        finding,
        CertificationBlockingFinding::StaleEvidenceOnCertifiedFamily { .. }
    )));
    assert_eq!(rebuilt.blocked_surface_count, 1);
    assert!(!rebuilt.all_surfaces_publishable);
}

#[test]
fn missing_evidence_family_is_caught() {
    let mut surfaces = seeded_surfaces();
    let surface = surfaces
        .iter_mut()
        .find(|surface| surface.surface_family == M5AppearanceSurfaceFamily::TracePanel)
        .unwrap();
    surface
        .family_certifications
        .iter_mut()
        .find(|certification| certification.object_family == AppearanceObjectFamily::TokenOverlay)
        .unwrap()
        .certification_status = M5QualificationStatus::MissingEvidence;
    let rebuilt = build_appearance_object_certification_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        canonical_object_model_index(),
        surfaces,
    );
    assert!(rebuilt.blocking_findings.iter().any(|finding| matches!(
        finding,
        CertificationBlockingFinding::FamilyMissingEvidence { .. }
    )));
}

#[test]
fn unbacked_family_source_is_caught() {
    let mut surfaces = seeded_surfaces();
    surfaces[0]
        .family_certifications
        .iter_mut()
        .find(|certification| certification.object_family == AppearanceObjectFamily::ThemePackage)
        .unwrap()
        .source_report_id = "shell:not_a_real_report:v1".to_owned();
    let rebuilt = build_appearance_object_certification_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        canonical_object_model_index(),
        surfaces,
    );
    assert!(rebuilt.blocking_findings.iter().any(|finding| matches!(
        finding,
        CertificationBlockingFinding::UnbackedFamilySource { .. }
    )));
}

#[test]
fn uncertified_required_surface_is_caught() {
    let surfaces: Vec<SurfaceCertification> = seeded_surfaces()
        .into_iter()
        .filter(|surface| surface.surface_family != M5AppearanceSurfaceFamily::OffboardingSurface)
        .collect();
    let rebuilt = build_appearance_object_certification_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        canonical_object_model_index(),
        surfaces,
    );
    assert!(rebuilt.blocking_findings.iter().any(|finding| matches!(
        finding,
        CertificationBlockingFinding::UncertifiedRequiredSurface { .. }
    )));
}

#[test]
fn incomplete_object_model_index_is_caught() {
    let index: Vec<ObjectFamilyIndexEntry> = canonical_object_model_index()
        .into_iter()
        .filter(|entry| {
            entry.object_family != AppearanceObjectFamily::ExtensionAppearanceDescriptor
        })
        .collect();
    let rebuilt = build_appearance_object_certification_report(
        SEED_BUILD_IDENTITY_REF,
        SEED_RELEASE_CHANNEL_CLASS,
        index,
        seeded_surfaces(),
    );
    assert!(rebuilt.blocking_findings.iter().any(|finding| matches!(
        finding,
        CertificationBlockingFinding::IndexFamilyMissing { .. }
    )));
    assert!(validate_appearance_object_certification_report(&rebuilt).is_err());
}

#[test]
fn compact_lines_and_markdown_are_deterministic_and_nonempty() {
    let report = seed();
    assert_eq!(report.compact_lines(), seed().compact_lines());
    assert!(!report.compact_lines().is_empty());
    assert_eq!(report.render_markdown(), seed().render_markdown());
    assert!(report
        .render_markdown()
        .contains("# M5 appearance-object certification"));
}
