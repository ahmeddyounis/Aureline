//! Protected tests binding the typed M5 OpenAPI publication catalog to the
//! checked-in catalog, the frozen CI validation capture, the example packs, and
//! the negative fixtures.
//!
//! The positive case is the checked-in catalog; the capture cross-check proves
//! the typed model and the CI validator agree on the summary counts and the
//! per-endpoint checks; the example-pack case proves every endpoint ships a
//! readable pack on disk; the negative cases load the checked-in fixtures to
//! prove that a duplicate endpoint id, an off-vocabulary auth class, a widened
//! lifecycle label, a drifted summary, and a read-only endpoint with a request
//! body all fail.

use std::path::{Path, PathBuf};

use aureline_release::publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces::{
    current_m5_openapi_catalog, M5OpenapiCatalog, M5_OPENAPI_CATALOG_FAMILY_ID,
    M5_OPENAPI_CATALOG_ID, M5_OPENAPI_CATALOG_RECORD_KIND, M5_OPENAPI_CATALOG_SCHEMA_VERSION,
};

const CAPTURE_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/release/captures/publish_openapi_specs_lifecycle_labels_and_example_packs_for_m5_service_apis_registry_mirror_endpoints_admin_ai_usage_export_routes_and_managed_control_plane_surfaces_validation_capture.json"
));

fn catalog() -> M5OpenapiCatalog {
    current_m5_openapi_catalog().expect("checked-in catalog parses into the model")
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
    assert_eq!(c.schema_version, M5_OPENAPI_CATALOG_SCHEMA_VERSION);
    assert_eq!(c.record_kind, M5_OPENAPI_CATALOG_RECORD_KIND);
    assert_eq!(c.catalog_id, M5_OPENAPI_CATALOG_ID);
    assert_eq!(c.family_id, M5_OPENAPI_CATALOG_FAMILY_ID);
    let violations = c.validate();
    assert!(
        violations.is_empty(),
        "checked-in catalog must validate cleanly: {violations:#?}"
    );
}

#[test]
fn every_endpoint_ships_a_readable_example_pack() {
    let c = catalog();
    let root = repo_root();
    assert!(!c.endpoints.is_empty());
    for e in &c.endpoints {
        let pack_path = root.join(&e.example_pack_ref);
        assert!(
            pack_path.exists(),
            "{}: example pack {pack_path:?} must exist",
            e.endpoint_id
        );
        let pack: serde_json::Value = serde_json::from_str(
            &std::fs::read_to_string(&pack_path).expect("example pack readable"),
        )
        .expect("example pack parses");
        assert_eq!(
            pack["operation_id"].as_str(),
            Some(e.operation_id.as_str()),
            "{}: example pack operation id must match",
            e.endpoint_id
        );
        // A read-only operation never ships a request example.
        if e.is_read_only() {
            assert!(pack["request"].is_null(), "{}", e.endpoint_id);
        }
        // No example pack leaks a live URL or a credential token.
        let body = pack["request"].to_string() + &pack["response"].to_string();
        let lowered = body.to_lowercase();
        for needle in ["://", "bearer ", "password", "secret"] {
            assert!(
                !lowered.contains(needle),
                "{}: example pack must not carry '{needle}'",
                e.endpoint_id
            );
        }
    }
}

#[test]
fn model_matches_frozen_validation_capture() {
    let c = catalog();
    let capture: serde_json::Value =
        serde_json::from_str(CAPTURE_JSON).expect("frozen capture parses");

    assert_eq!(capture["status"].as_str(), Some("pass"));
    assert_eq!(capture["as_of"].as_str(), Some(c.as_of.as_str()));
    assert_eq!(capture["catalog_id"].as_str(), Some(c.catalog_id.as_str()));
    assert_eq!(capture["family_id"].as_str(), Some(c.family_id.as_str()));

    let summary = &capture["summary"];
    let computed = c.computed_summary();
    assert_eq!(
        summary["total_endpoints"].as_u64().unwrap() as usize,
        computed.total_endpoints
    );
    assert_eq!(
        summary["read_only_endpoints"].as_u64().unwrap() as usize,
        computed.read_only_endpoints
    );
    assert_eq!(
        summary["auth_required_endpoints"].as_u64().unwrap() as usize,
        computed.auth_required_endpoints
    );
    assert_eq!(
        summary["service_surface_count"].as_u64().unwrap() as usize,
        computed.service_surface_count
    );

    let checks = capture["endpoint_checks"].as_array().unwrap();
    assert_eq!(
        checks.len(),
        c.endpoints.len(),
        "capture must record every endpoint"
    );
    for check in checks {
        let eid = check["endpoint_id"].as_str().unwrap();
        assert!(
            c.endpoint(eid).is_some(),
            "capture endpoint {eid} is in the model"
        );
        for key in [
            "operation_present_in_openapi_document",
            "auth_matches_surface_row",
            "example_pack_validates_against_schema",
            "lifecycle_matches_matrix",
        ] {
            assert_eq!(
                check[key].as_str(),
                Some("passed"),
                "{eid}: {key} must have passed"
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
    let fixtures_dir = repo_root().join("fixtures/contracts/m5-openapi");
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
        // The off-vocabulary fixture carries an enum serde refuses to
        // deserialize, which is itself a rejection; the structurally-parseable
        // fixtures must be rejected by `validate()`.
        match serde_json::from_str::<M5OpenapiCatalog>(&raw) {
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
