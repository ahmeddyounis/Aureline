//! Protected fixture checks for governed extension appearance descriptors.
//!
//! The integration test replays the JSON fixtures under
//! `fixtures/ux/m5/extension-theme-inheritance/` through the Rust types and
//! asserts the contract invariants. The audit fixture is asserted bit-for-bit
//! equal to the audit minted by `seeded_extension_appearance_audit`, and the
//! markdown artifact under
//! `artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md`
//! is asserted bit-for-bit equal to the rendering, so the headless inspector
//! stays the only mint-from-truth path.

use std::path::{Path, PathBuf};

use aureline_extensions::appearance_descriptors::{
    project_extension_appearance_support_export, seeded_extension_appearance_audit,
    validate_extension_appearance_audit, validate_extension_appearance_support_export,
    ExtensionAppearanceAudit, ExtensionAppearanceSupportExport, InheritanceBadgeClass,
    ParityClaimStateClass, EXTENSION_APPEARANCE_AUDIT_RECORD_KIND,
    EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_REPORT_REF,
    EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF, RENDERED_SURFACE_TOKENS,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/ux/m5/extension-theme-inheritance")
}

fn artifacts_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../artifacts/ux/m5/extension-appearance-audit")
}

fn load_json<T: serde::de::DeserializeOwned>(file: &str) -> T {
    let path = fixtures_root().join(file);
    let payload = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    serde_json::from_str(&payload)
        .unwrap_or_else(|err| panic!("failed to parse {}: {err}", path.display()))
}

#[test]
fn fixture_audit_is_bit_for_bit_equal_to_seed() {
    let on_disk: ExtensionAppearanceAudit = load_json("audit.json");
    let seeded = seeded_extension_appearance_audit();
    assert_eq!(on_disk, seeded, "fixture audit diverged from seeded audit");
    assert_eq!(seeded.record_kind, EXTENSION_APPEARANCE_AUDIT_RECORD_KIND);
    assert_eq!(
        seeded.shared_contract_ref,
        EXTENSION_APPEARANCE_DESCRIPTOR_SHARED_CONTRACT_REF
    );
    assert_eq!(
        seeded.report_ref,
        EXTENSION_APPEARANCE_DESCRIPTOR_PUBLISHED_REPORT_REF
    );
}

#[test]
fn fixture_audit_validates_and_is_clean() {
    let audit: ExtensionAppearanceAudit = load_json("audit.json");
    validate_extension_appearance_audit(&audit).expect("fixture audit must validate");
    assert!(audit.is_clean());
}

#[test]
fn fixture_descriptors_disclose_posture_badge_and_rendered_surfaces() {
    let audit: ExtensionAppearanceAudit = load_json("audit.json");
    for descriptor in &audit.descriptors {
        assert_eq!(
            descriptor.axes.len(),
            5,
            "descriptor {} must carry the five governed axes",
            descriptor.descriptor_id
        );
        assert!(
            !descriptor.host_id.trim().is_empty(),
            "descriptor {} must name its host",
            descriptor.descriptor_id
        );
        assert!(
            !descriptor.package_id.trim().is_empty(),
            "descriptor {} must name its package",
            descriptor.descriptor_id
        );
        assert!(
            descriptor.host_rendered_appearance_badge,
            "descriptor {} must keep its host badge visible",
            descriptor.descriptor_id
        );
        for token in RENDERED_SURFACE_TOKENS {
            assert!(
                descriptor.rendered_on_surfaces.iter().any(|s| s == token),
                "descriptor {} must render its badge on {token}",
                descriptor.descriptor_id
            );
        }
    }
}

#[test]
fn fixture_never_overclaims_host_parity() {
    let audit: ExtensionAppearanceAudit = load_json("audit.json");
    for descriptor in &audit.descriptors {
        if descriptor.parity_claim_state == ParityClaimStateClass::ClaimsHostParity {
            assert_eq!(
                descriptor.badge.badge_class,
                InheritanceBadgeClass::FullInheritance,
                "descriptor {} cannot claim host parity without full inheritance",
                descriptor.descriptor_id
            );
            assert!(
                !descriptor.accessibility_evidence_refs.is_empty(),
                "descriptor {} cannot claim host parity without accessibility evidence",
                descriptor.descriptor_id
            );
            assert!(
                descriptor.known_gaps.is_empty(),
                "descriptor {} cannot claim host parity while disclosing gaps",
                descriptor.descriptor_id
            );
        }
        // A denied parity claim must never reach the published audit.
        assert_ne!(
            descriptor.parity_claim_state,
            ParityClaimStateClass::DeniedClaim,
            "descriptor {} must not ship a denied parity claim",
            descriptor.descriptor_id
        );
    }
}

#[test]
fn fixture_covers_the_badge_spectrum() {
    let audit: ExtensionAppearanceAudit = load_json("audit.json");
    let badges: Vec<InheritanceBadgeClass> = audit
        .descriptors
        .iter()
        .map(|d| d.badge.badge_class)
        .collect();
    for expected in [
        InheritanceBadgeClass::FullInheritance,
        InheritanceBadgeClass::PartialInheritance,
        InheritanceBadgeClass::DoesNotInherit,
    ] {
        assert!(
            badges.contains(&expected),
            "fixture must cover badge {}",
            expected.as_str()
        );
    }
}

#[test]
fn fixture_support_export_quotes_audit_and_descriptors() {
    let audit: ExtensionAppearanceAudit = load_json("audit.json");
    let export: ExtensionAppearanceSupportExport = load_json("support_export.json");
    let expected = project_extension_appearance_support_export(&audit, export.export_id.clone());
    assert_eq!(export, expected);
    validate_extension_appearance_support_export(&audit, &export)
        .expect("support export must validate");
    assert!(export.raw_appearance_material_excluded);
    assert!(export.case_ids.contains(&audit.audit_id));
    for descriptor in &audit.descriptors {
        assert!(
            export.case_ids.contains(&descriptor.descriptor_id),
            "support export must quote descriptor {}",
            descriptor.descriptor_id
        );
    }
}

#[test]
fn published_audit_md_matches_seeded_rendering() {
    let audit = seeded_extension_appearance_audit();
    let rendered = audit.render_markdown();
    let on_disk = std::fs::read_to_string(artifacts_root().join("extension_appearance_audit.md"))
        .expect("published extension_appearance_audit.md must exist");
    assert_eq!(
        on_disk, rendered,
        "published extension_appearance_audit.md diverged from seeded rendering -- regenerate with \
         `cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- markdown`",
    );
}

#[test]
fn fixture_compact_lines_match_seed() {
    let compact_path = fixtures_root().join("compact.txt");
    let on_disk = std::fs::read_to_string(&compact_path).expect("compact fixture must exist");
    let audit = seeded_extension_appearance_audit();
    let mut rendered = audit.compact_lines().join("\n");
    rendered.push('\n');
    assert_eq!(
        on_disk, rendered,
        "fixture compact.txt diverged from seeded compact lines -- regenerate with \
         `cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- compact`",
    );
}

#[test]
fn published_doc_links_artifacts_and_schema() {
    let doc_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/m5/extension-appearance-inheritance.md");
    let body = std::fs::read_to_string(&doc_path)
        .expect("published extension-appearance-inheritance doc must exist");
    for backlink in [
        "artifacts/ux/m5/extension-appearance-audit/extension_appearance_audit.md",
        "fixtures/ux/m5/extension-theme-inheritance/audit.json",
        "schemas/ux/extension-appearance-descriptor.schema.json",
        "tools/ci/m5/extension_appearance_descriptors_check.py",
    ] {
        assert!(body.contains(backlink), "doc must back-link {backlink}");
    }
}
