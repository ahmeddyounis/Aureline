//! Inline unit tests for the typed M5 OpenAPI publication catalog.

use super::*;

fn catalog() -> M5OpenapiCatalog {
    current_m5_openapi_catalog().expect("checked-in catalog parses into the model")
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
fn closed_vocabularies_round_trip() {
    let c = catalog();
    assert_eq!(c.http_methods, HttpMethod::ALL.to_vec());
    assert_eq!(c.auth_source_classes, AuthSourceClass::ALL.to_vec());
    assert_eq!(c.entitlement_classes, EntitlementClass::ALL.to_vec());
    assert_eq!(c.mutability_postures, MutabilityPosture::ALL.to_vec());
    assert_eq!(c.preview_support_classes, PreviewSupportClass::ALL.to_vec());
    assert_eq!(c.lifecycle_labels, LifecycleLabel::ALL.to_vec());
    assert_eq!(c.downgrade_behaviors, DowngradeBehavior::ALL.to_vec());
}

#[test]
fn family_publishes_stable_and_every_endpoint_inherits_the_label() {
    let c = catalog();
    assert!(
        c.publishes_stable(),
        "the OpenAPI family must be published at the stable cutline once the full family lands"
    );
    assert!(!c.endpoints.is_empty());
    for e in &c.endpoints {
        assert_eq!(
            e.lifecycle_label, c.family_lifecycle_label,
            "{} must inherit the family label",
            e.endpoint_id
        );
    }
}

#[test]
fn every_endpoint_binds_auth_mutability_and_an_example_pack() {
    let c = catalog();
    for e in &c.endpoints {
        assert!(!e.compatibility_note.is_empty(), "{}", e.endpoint_id);
        assert!(
            e.example_pack_ref
                .starts_with("examples/contracts/m5-openapi/"),
            "{}",
            e.endpoint_id
        );
        assert!(
            e.response_schema_ref
                .starts_with("openapi/service_api_seed.yaml#/components/schemas/"),
            "{}",
            e.endpoint_id
        );
        // Read-only operations never carry a request body or a preview.
        if e.is_read_only() {
            assert!(!e.has_request_body(), "{}", e.endpoint_id);
            assert_eq!(
                e.preview_support_class,
                PreviewSupportClass::ReadOnlyNoMutation,
                "{}",
                e.endpoint_id
            );
        }
    }
}

#[test]
fn computed_summary_matches_recorded_summary() {
    let c = catalog();
    assert_eq!(c.summary, c.computed_summary());
}

#[test]
fn duplicate_endpoint_id_fails() {
    let mut c = catalog();
    let first = c.endpoints[0].clone();
    c.endpoints.push(first);
    c.summary = c.computed_summary();
    assert!(
        c.validate()
            .iter()
            .any(|v| v.check_id == "endpoints.duplicate_endpoint_id"),
        "two endpoints may not share an id"
    );
}

#[test]
fn read_only_with_request_body_fails() {
    let mut c = catalog();
    let ep = c
        .endpoints
        .iter_mut()
        .find(|e| e.is_read_only())
        .expect("catalog has a read-only endpoint");
    ep.request_schema_ref = Some(
        "openapi/service_api_seed.yaml#/components/schemas/ExtensionInstallRequest".to_string(),
    );
    c.summary = c.computed_summary();
    assert!(
        c.validate()
            .iter()
            .any(|v| v.check_id == "endpoints.read_only_with_request_body"),
        "a read-only endpoint may not carry a request body"
    );
}

#[test]
fn lifecycle_wider_than_family_fails() {
    let mut c = catalog();
    c.endpoints[0].lifecycle_label = LifecycleLabel::Lts;
    assert!(
        c.validate()
            .iter()
            .any(|v| v.check_id == "endpoints.lifecycle_wider_than_family"),
        "an endpoint may not publish a lifecycle label other than the family label"
    );
}

#[test]
fn summary_count_mismatch_fails() {
    let mut c = catalog();
    c.summary.total_endpoints += 1;
    assert!(
        c.validate()
            .iter()
            .any(|v| v.check_id == "summary.count_mismatch"),
        "a drifted summary must fail"
    );
}
