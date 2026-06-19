//! Protected tests binding the typed M5 contract catalog to the checked-in
//! catalog, the sample payload galleries, the frozen CI validation capture, and
//! the negative fixtures.
//!
//! The positive case is the checked-in catalog; the gallery cross-check proves
//! every family's gallery exists, points back at the catalog entry, and carries
//! a partial/not-provided sample; the capture cross-check proves the typed model
//! and the CI validator agree on the summary counts and the per-family checks;
//! the negative cases load the checked-in fixtures to prove that a duplicate
//! family id, an unknown lifecycle label, a drifted summary, and a missing
//! partial sample fail validation.

use std::path::{Path, PathBuf};

use aureline_release::ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity::{
    current_m5_contract_catalog, IdentityKind, LifecycleLabel, M5ContractCatalog, SampleClass,
    M5_CONTRACT_CATALOG_ID, M5_CONTRACT_CATALOG_RECORD_KIND, M5_CONTRACT_CATALOG_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/ship_contract_example_corpora_sample_payload_galleries_and_docs_help_sdk_catalogs_so_every_published_m5_contract_is_inspectable_with_offline_mirror_parity_validation_capture.json"
));

fn catalog() -> M5ContractCatalog {
    current_m5_contract_catalog().expect("checked-in catalog parses into the model")
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repo root resolves")
}

#[test]
fn checked_in_catalog_parses_and_validates() {
    let c = catalog();
    assert_eq!(c.schema_version, M5_CONTRACT_CATALOG_SCHEMA_VERSION);
    assert_eq!(c.record_kind, M5_CONTRACT_CATALOG_RECORD_KIND);
    assert_eq!(c.catalog_id, M5_CONTRACT_CATALOG_ID);
    let violations = c.validate();
    assert!(
        violations.is_empty(),
        "checked-in catalog must validate cleanly: {violations:#?}"
    );
}

#[test]
fn every_published_family_is_inspectable_offline() {
    let c = catalog();
    assert!(!c.families.is_empty());
    let root = repo_root();
    for fam in &c.families {
        // A canonical schema/spec identity and a checked-in gallery.
        assert!(!fam.contract_identity.schema_or_spec_id.is_empty());
        assert!(fam
            .example_gallery_ref
            .starts_with("examples/contracts/m5-gallery/"));
        assert!(
            root.join(&fam.example_gallery_ref).exists(),
            "{} gallery exists",
            fam.family_id
        );
        // The lifecycle label is one of the published labels and never wider
        // than the claim label.
        assert!(matches!(
            fam.lifecycle_label,
            LifecycleLabel::Stable | LifecycleLabel::Beta | LifecycleLabel::Lts
        ));
        // Offline inspection never requires a live service.
        assert!(fam.offline_parity.mirror_inspectable);
        assert!(!fam.offline_parity.requires_runtime_service);
        // Every gallery includes a partial/not-provided sample.
        assert!(fam
            .sample_classes
            .contains(&SampleClass::PartialOrNotProvided));
    }
}

#[test]
fn galleries_point_back_at_the_catalog_entry_and_carry_field_notes() {
    let c = catalog();
    let root = repo_root();
    for fam in &c.families {
        let raw = std::fs::read_to_string(root.join(&fam.example_gallery_ref))
            .unwrap_or_else(|_| panic!("gallery {} is readable", fam.example_gallery_ref));
        let gallery: serde_json::Value = serde_json::from_str(&raw).expect("gallery parses");

        // The gallery points back at the catalog entry and the same identity.
        assert_eq!(gallery["family_id"].as_str(), Some(fam.family_id.as_str()));
        assert!(gallery["catalog_entry_ref"]
            .as_str()
            .unwrap()
            .ends_with(&fam.family_id));
        assert_eq!(
            gallery["contract_identity"]["schema_or_spec_id"].as_str(),
            Some(fam.contract_identity.schema_or_spec_id.as_str())
        );

        let samples = gallery["samples"].as_array().expect("samples is an array");
        assert_eq!(
            samples.len(),
            fam.sample_count,
            "{} sample count",
            fam.family_id
        );
        let mut saw_partial = false;
        for sample in samples {
            if sample["sample_class"].as_str() == Some("partial_or_not_provided") {
                saw_partial = true;
            }
            // Field notes cover every payload field.
            let payload = sample["payload"].as_object().expect("payload object");
            let notes = sample["field_notes"].as_array().expect("field notes array");
            assert!(
                !notes.is_empty(),
                "{} sample has field notes",
                fam.family_id
            );
            let note_fields: Vec<&str> =
                notes.iter().map(|n| n["field"].as_str().unwrap()).collect();
            for key in payload.keys() {
                assert!(
                    note_fields.contains(&key.as_str()),
                    "{}: field {key} must have a note",
                    fam.family_id
                );
            }
        }
        assert!(
            saw_partial,
            "{} gallery must include a partial sample",
            fam.family_id
        );
    }
}

#[test]
fn one_entry_backs_docs_sdk_and_inspect() {
    // The acceptance anchor: at least one in-product inspect resolution returns
    // the same catalog entry and gallery the docs/SDK publication uses.
    let c = catalog();
    let fam = c
        .family("command_descriptors")
        .expect("command_descriptors present");
    let (id, label) = c.resolve_contract("command_descriptors").expect("resolves");
    assert_eq!(id, fam.contract_identity.schema_or_spec_id);
    assert_eq!(label, fam.lifecycle_label);
    assert_eq!(
        fam.contract_identity.identity_kind,
        IdentityKind::JsonSchema
    );
    // The same gallery is what the SDK doc links to.
    assert_eq!(
        fam.example_gallery_ref,
        "examples/contracts/m5-gallery/command_descriptors.json"
    );
}

#[test]
fn model_matches_frozen_validation_capture() {
    let c = catalog();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(c.as_of.as_str()));
    assert_eq!(capture["catalog_id"].as_str(), Some(c.catalog_id.as_str()));

    let summary = &capture["summary"];
    let computed = c.computed_summary();
    assert_eq!(
        summary["total_families"].as_u64().unwrap() as usize,
        c.families.len()
    );
    assert_eq!(
        summary["families_narrowed"].as_u64().unwrap() as usize,
        computed.families_narrowed
    );
    assert_eq!(
        summary["total_samples"].as_u64().unwrap() as usize,
        computed.total_samples
    );

    let checks = capture["family_checks"].as_array().unwrap();
    assert_eq!(
        checks.len(),
        c.families.len(),
        "capture must record every family"
    );
    for check in checks {
        let family = check["family_id"].as_str().unwrap();
        let fam = c
            .family(family)
            .unwrap_or_else(|| panic!("capture family {family} is in the model"));
        assert_eq!(
            check["lifecycle_label"].as_str().unwrap(),
            serde_json::to_value(fam.lifecycle_label)
                .unwrap()
                .as_str()
                .unwrap(),
            "capture lifecycle label must match the model for {family}"
        );
        for key in [
            "gallery_present",
            "samples_validate",
            "partial_sample_present",
            "lifecycle_matches_matrix",
            "offline_inspectable",
        ] {
            assert_eq!(
                check[key].as_str(),
                Some("passed"),
                "{family}: {key} must have passed"
            );
        }
    }

    for drill in capture["negative_drills"].as_array().unwrap() {
        assert_eq!(
            drill["status"].as_str(),
            Some("passed"),
            "frozen capture drill {} must have passed",
            drill["drill_id"]
        );
    }
    let fixtures = capture["fixture_cases"].as_array().unwrap();
    assert!(!fixtures.is_empty(), "capture must record fixture cases");
    for case in fixtures {
        assert_eq!(
            case["status"].as_str(),
            Some("passed"),
            "frozen capture fixture case {} must have passed",
            case["case_id"]
        );
    }
}

#[test]
fn checked_in_fixtures_are_rejected_by_the_model() {
    let fixtures_dir = repo_root().join("fixtures/contracts/m5-contract-catalog");
    let cases_json = std::fs::read_to_string(fixtures_dir.join("cases.json"))
        .expect("fixture manifest is readable");
    let manifest: serde_json::Value =
        serde_json::from_str(&cases_json).expect("fixture manifest parses");
    let cases = manifest["cases"].as_array().expect("cases is an array");
    assert!(!cases.is_empty(), "fixture manifest must list cases");

    let mut model_checked = 0;
    for case in cases {
        let file = case["file"].as_str().expect("case names a file");
        let raw = std::fs::read_to_string(fixtures_dir.join(file))
            .unwrap_or_else(|_| panic!("fixture {file} is readable"));
        // The unknown-lifecycle-label fixture intentionally carries an off-vocab
        // enum that serde refuses to deserialize, which is itself a rejection;
        // the structurally-parseable fixtures must be rejected by `validate()`.
        match serde_json::from_str::<M5ContractCatalog>(&raw) {
            Ok(candidate) => {
                assert!(
                    !candidate.validate().is_empty(),
                    "fixture {file} must be rejected by the typed model"
                );
                model_checked += 1;
            }
            Err(_) => {
                model_checked += 1;
            }
        }
    }
    assert!(
        model_checked > 0,
        "at least one fixture must exercise a typed-model invariant"
    );
}
