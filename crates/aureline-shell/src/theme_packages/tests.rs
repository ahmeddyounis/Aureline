//! Unit tests for the M5 theme-package manifest audit.

use super::*;

fn manifests() -> Vec<ThemePackageManifest> {
    seeded_theme_package_manifest_audit().manifests
}

fn descriptor(surface_id: &str) -> ThemePackageSurfaceDescriptor {
    ThemePackageSurfaceDescriptor {
        surface_id: surface_id.to_owned(),
        surface_family: ThemePackageSurfaceFamily::Notebook,
        descriptor_revision_ref: format!("rev:{surface_id}:1"),
        primary_label_ref: format!("label:{surface_id}"),
        appearance_anchor_ref: format!("anchor:{surface_id}"),
        accessibility_note: "Keeps semantic meaning across themes.".to_owned(),
        semantic_salience: SemanticSalience::LifecycleBearing,
        marketed_on_desktop_rows: true,
        registered_on_appearance_session: true,
    }
}

#[test]
fn seeded_audit_is_clean_and_validates() {
    let report = seeded_theme_package_manifest_audit();
    assert!(report.report_clean);
    assert_eq!(report.findings_summary.total_blocking_findings, 0);
    validate_theme_package_manifests(&report).expect("seeded audit must validate");
}

#[test]
fn seeded_audit_envelope_is_stable() {
    let report = seeded_theme_package_manifest_audit();
    assert_eq!(report.record_kind, THEME_PACKAGE_REPORT_RECORD_KIND);
    assert_eq!(report.schema_version, THEME_PACKAGE_SCHEMA_VERSION);
    assert_eq!(
        report.shared_contract_ref,
        THEME_PACKAGE_SHARED_CONTRACT_REF
    );
    assert_eq!(report.report_id, THEME_PACKAGE_REPORT_ID);
    assert_eq!(report.source_schema_ref, THEME_PACKAGE_SOURCE_SCHEMA_REF);
    assert_eq!(
        report.canonical_manifest_schema_ref,
        THEME_PACKAGE_CANONICAL_MANIFEST_SCHEMA_REF
    );
    assert_eq!(
        report.published_report_ref,
        THEME_PACKAGE_PUBLISHED_REPORT_REF
    );
    assert_eq!(report.published_doc_ref, THEME_PACKAGE_PUBLISHED_DOC_REF);
}

#[test]
fn seeded_audit_is_deterministic() {
    let first = seeded_theme_package_manifest_audit();
    let second = seeded_theme_package_manifest_audit();
    assert_eq!(first, second);
}

#[test]
fn seeded_audit_covers_every_surface_family() {
    let report = seeded_theme_package_manifest_audit();
    let families: std::collections::BTreeSet<_> = report
        .surfaces
        .iter()
        .map(|s| s.descriptor.surface_family)
        .collect();
    for family in [
        ThemePackageSurfaceFamily::Notebook,
        ThemePackageSurfaceFamily::ResultGrid,
        ThemePackageSurfaceFamily::ProfilerTimeline,
        ThemePackageSurfaceFamily::PreviewBrowserPane,
        ThemePackageSurfaceFamily::DocsHelpPane,
        ThemePackageSurfaceFamily::CompanionSurface,
        ThemePackageSurfaceFamily::ExtensionBackedSurface,
    ] {
        assert!(
            families.contains(&family),
            "missing surface family {}",
            family.as_str()
        );
    }
}

#[test]
fn every_surface_resolves_its_active_package() {
    let report = seeded_theme_package_manifest_audit();
    assert!(report.every_surface_package_resolved());
}

#[test]
fn surfaces_and_manifests_are_sorted() {
    let report = seeded_theme_package_manifest_audit();
    let surface_ids: Vec<_> = report
        .surfaces
        .iter()
        .map(|s| s.descriptor.surface_id.clone())
        .collect();
    let mut sorted = surface_ids.clone();
    sorted.sort();
    assert_eq!(surface_ids, sorted);

    let package_ids: Vec<_> = report
        .manifests
        .iter()
        .map(|m| m.package_id.clone())
        .collect();
    let mut sorted_packages = package_ids.clone();
    sorted_packages.sort();
    assert_eq!(package_ids, sorted_packages);
}

#[test]
fn unknown_active_package_is_blocking() {
    let registry = manifests();
    let binding = build_theme_package_surface_binding(
        descriptor("surface:test.unknown"),
        "theme-pkg:does-not-exist",
        vec![ThemeModeClass::DarkReference],
        vec![DensityClass::Standard],
        vec![MotionPostureClass::MotionStandard],
        InheritancePosture::FullyInherited,
        vec![
            InheritanceAxis::Theme,
            InheritanceAxis::Contrast,
            InheritanceAxis::Density,
            InheritanceAxis::Focus,
            InheritanceAxis::ReducedMotion,
        ],
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:test.unknown",
        true,
        &registry,
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "active_package_unknown"));
}

#[test]
fn unsupported_mode_is_blocking() {
    let registry = manifests();
    // partner-dusk does not support light_parity.
    let binding = build_theme_package_surface_binding(
        descriptor("surface:test.mode"),
        "theme-pkg:partner-dusk",
        vec![ThemeModeClass::LightParity],
        vec![DensityClass::Standard],
        vec![MotionPostureClass::MotionStandard],
        InheritancePosture::PartialInheritanceDisclosed,
        vec![
            InheritanceAxis::Theme,
            InheritanceAxis::Contrast,
            InheritanceAxis::Density,
        ],
        vec![InheritanceAxis::Focus],
        true,
        PackageEvidenceState::Current,
        "evidence:test.mode",
        true,
        &registry,
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "unsupported_mode_claimed"));
}

#[test]
fn hidden_inheritance_gap_is_blocking() {
    let registry = manifests();
    // Default pack expects Focus inheritance; this surface neither inherits
    // nor discloses it.
    let binding = build_theme_package_surface_binding(
        descriptor("surface:test.gap"),
        "theme-pkg:aureline-default",
        vec![ThemeModeClass::DarkReference],
        vec![DensityClass::Standard],
        vec![MotionPostureClass::MotionStandard],
        InheritancePosture::FullyInherited,
        vec![
            InheritanceAxis::Theme,
            InheritanceAxis::Contrast,
            InheritanceAxis::Density,
            InheritanceAxis::ReducedMotion,
        ],
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:test.gap",
        true,
        &registry,
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "inheritance_gap_hidden"));
}

#[test]
fn undisclosed_provenance_is_blocking() {
    let registry = manifests();
    let binding = build_theme_package_surface_binding(
        descriptor("surface:test.prov"),
        "theme-pkg:aureline-default",
        vec![ThemeModeClass::DarkReference],
        vec![DensityClass::Standard],
        vec![MotionPostureClass::MotionStandard],
        InheritancePosture::FullyInherited,
        vec![
            InheritanceAxis::Theme,
            InheritanceAxis::Contrast,
            InheritanceAxis::Density,
            InheritanceAxis::Focus,
            InheritanceAxis::ReducedMotion,
        ],
        vec![],
        false,
        PackageEvidenceState::Current,
        "evidence:test.prov",
        true,
        &registry,
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "provenance_not_disclosed"));
}

#[test]
fn stale_evidence_on_marketed_surface_is_blocking() {
    let registry = manifests();
    let all_axes = vec![
        InheritanceAxis::Theme,
        InheritanceAxis::Contrast,
        InheritanceAxis::Density,
        InheritanceAxis::Focus,
        InheritanceAxis::ReducedMotion,
    ];
    let binding = build_theme_package_surface_binding(
        descriptor("surface:test.stale"),
        "theme-pkg:aureline-default",
        vec![ThemeModeClass::DarkReference],
        vec![DensityClass::Standard],
        vec![MotionPostureClass::MotionStandard],
        InheritancePosture::FullyInherited,
        all_axes.clone(),
        vec![],
        true,
        PackageEvidenceState::StaleEvidence,
        "evidence:test.stale",
        true,
        &registry,
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "stale_evidence_on_marketed_surface"));

    // Disclosed stale evidence on a non-marketed surface is allowed.
    let non_marketed = build_theme_package_surface_binding(
        descriptor("surface:test.stale_ok"),
        "theme-pkg:aureline-default",
        vec![ThemeModeClass::DarkReference],
        vec![DensityClass::Standard],
        vec![MotionPostureClass::MotionStandard],
        InheritancePosture::FullyInherited,
        all_axes,
        vec![],
        true,
        PackageEvidenceState::StaleEvidence,
        "evidence:test.stale_ok",
        false,
        &registry,
    );
    assert!(non_marketed.blocking_findings.is_empty());
}

#[test]
fn disabled_package_without_disclosure_is_blocking() {
    let registry = manifests();
    let all_axes = vec![
        InheritanceAxis::Theme,
        InheritanceAxis::Contrast,
        InheritanceAxis::Density,
        InheritanceAxis::Focus,
        InheritanceAxis::ReducedMotion,
    ];
    let binding = build_theme_package_surface_binding(
        descriptor("surface:test.disabled"),
        "theme-pkg:aureline-default",
        vec![ThemeModeClass::DarkReference],
        vec![DensityClass::Standard],
        vec![MotionPostureClass::MotionStandard],
        InheritancePosture::FullyInherited,
        all_axes,
        vec![],
        false,
        PackageEvidenceState::DisabledPackage,
        "evidence:test.disabled",
        true,
        &registry,
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "disabled_package_rendering_undisclosed"));
}

#[test]
fn surface_off_appearance_session_is_blocking() {
    let registry = manifests();
    let mut desc = descriptor("surface:test.offsession");
    desc.registered_on_appearance_session = false;
    let all_axes = vec![
        InheritanceAxis::Theme,
        InheritanceAxis::Contrast,
        InheritanceAxis::Density,
        InheritanceAxis::Focus,
        InheritanceAxis::ReducedMotion,
    ];
    let binding = build_theme_package_surface_binding(
        desc,
        "theme-pkg:aureline-default",
        vec![ThemeModeClass::DarkReference],
        vec![DensityClass::Standard],
        vec![MotionPostureClass::MotionStandard],
        InheritancePosture::FullyInherited,
        all_axes,
        vec![],
        true,
        PackageEvidenceState::Current,
        "evidence:test.offsession",
        true,
        &registry,
    );
    assert!(binding
        .blocking_findings
        .iter()
        .any(|f| f.class_token() == "surface_not_on_appearance_session"));
}

#[test]
fn first_party_manifest_missing_token_set_is_blocking() {
    let mut manifest = manifests()
        .into_iter()
        .find(|m| m.package_id == "theme-pkg:aureline-default")
        .expect("default pack");
    manifest
        .token_sets
        .retain(|set| set.kind != TokenSetKind::Syntax);
    let findings = compute_manifest_findings(&manifest);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "manifest_token_set_incomplete"));
}

#[test]
fn first_party_manifest_missing_required_mode_is_blocking() {
    let mut manifest = manifests()
        .into_iter()
        .find(|m| m.package_id == "theme-pkg:aureline-default")
        .expect("default pack");
    manifest
        .supported_theme_modes
        .retain(|mode| *mode != ThemeModeClass::LightParity);
    let findings = compute_manifest_findings(&manifest);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "manifest_missing_required_mode"));
}

#[test]
fn signature_failed_manifest_is_blocking() {
    let mut manifest = manifests()
        .into_iter()
        .find(|m| m.package_id == "theme-pkg:partner-dusk")
        .expect("partner pack");
    manifest.signature_state = SignatureState::SignatureFailedBlocked;
    let findings = compute_manifest_findings(&manifest);
    assert!(findings
        .iter()
        .any(|f| f.class_token() == "manifest_signature_failed_still_registered"));
}

#[test]
fn support_export_quotes_report_packages_and_surfaces() {
    let report = seeded_theme_package_manifest_audit();
    let export =
        ThemePackageSupportExport::from_report(THEME_PACKAGE_SUPPORT_EXPORT_ID, report.clone());
    assert_eq!(export.record_kind, THEME_PACKAGE_SUPPORT_EXPORT_RECORD_KIND);
    assert!(export.case_ids.contains(&report.report_id));
    for manifest in &report.manifests {
        assert!(export.case_ids.contains(&manifest.package_id));
        assert!(export.case_ids.contains(&manifest.package_revision_ref));
    }
    for surface in &report.surfaces {
        assert!(export.case_ids.contains(&surface.descriptor.surface_id));
        assert!(export
            .case_ids
            .contains(&surface.descriptor.descriptor_revision_ref));
    }
}

#[test]
fn provenance_index_reports_all_packages() {
    let report = seeded_theme_package_manifest_audit();
    assert_eq!(report.provenance_index.len(), report.manifests.len());
    let extension_entry = report
        .provenance_index
        .iter()
        .find(|e| e.package_id == "theme-pkg:partner-dusk")
        .expect("partner pack provenance");
    assert_eq!(
        extension_entry.provenance_class,
        ProvenanceClass::ExtensionContributed
    );
    assert_eq!(
        extension_entry.signature_state,
        SignatureState::SignedVerified
    );
}

#[test]
fn markdown_and_compact_are_deterministic() {
    let report = seeded_theme_package_manifest_audit();
    assert_eq!(report.render_markdown(), report.render_markdown());
    assert_eq!(report.compact_lines(), report.compact_lines());
    assert!(report
        .render_markdown()
        .contains("M5 theme-package manifest audit"));
}

#[test]
fn clean_audit_has_no_narrowable_surfaces() {
    let report = seeded_theme_package_manifest_audit();
    assert!(report.narrowable_marketed_surfaces.is_empty());
}

#[test]
fn json_round_trips() {
    let report = seeded_theme_package_manifest_audit();
    let json = serde_json::to_string(&report).expect("serialize");
    let back: ThemePackageManifestReport = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(report, back);
}
