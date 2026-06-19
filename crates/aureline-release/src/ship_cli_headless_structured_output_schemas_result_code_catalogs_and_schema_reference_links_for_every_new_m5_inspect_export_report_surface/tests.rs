//! Inline unit tests for the typed M5 CLI/headless structured-output catalog.

use super::*;

fn catalog() -> M5CliOutputCatalog {
    current_m5_cli_output_catalog().expect("checked-in catalog parses")
}

#[test]
fn checked_in_catalog_parses_and_validates() {
    let c = catalog();
    assert_eq!(c.schema_version, M5_CLI_OUTPUT_CATALOG_SCHEMA_VERSION);
    assert_eq!(c.record_kind, M5_CLI_OUTPUT_CATALOG_RECORD_KIND);
    assert_eq!(c.catalog_id, M5_CLI_OUTPUT_CATALOG_ID);
    let violations = c.validate();
    assert!(
        violations.is_empty(),
        "catalog must validate cleanly: {violations:#?}"
    );
}

#[test]
fn summary_recomputes_from_surfaces() {
    let c = catalog();
    assert_eq!(c.summary, c.computed_summary());
    assert_eq!(c.summary.total_surfaces, c.surfaces.len());
    assert!(c.summary.total_surfaces > 0);
    assert_eq!(
        c.summary.inspect_surfaces
            + c.summary.export_surfaces
            + c.summary.report_surfaces
            + c.summary.health_surfaces,
        c.surfaces.len(),
        "every surface has exactly one kind"
    );
}

#[test]
fn every_surface_binds_schema_result_codes_and_parity() {
    let c = catalog();
    for s in &c.surfaces {
        assert!(
            s.structured_output_schema_ref
                .starts_with("schemas/public/m5-json/"),
            "{} must resolve to a JSON Schema package",
            s.surface_id
        );
        assert!(s.structured_output_schema_id.ends_with(".schema.json"));
        assert!(
            !s.result_code_catalog.is_empty(),
            "{} needs result codes",
            s.surface_id
        );
        assert!(s.result_codes().contains(&ResultCode::Success));
        assert!(s.result_codes().iter().any(|code| !code.is_success()));
        assert!(s
            .partial_result_states
            .contains(&PartialResultState::StaleRetestNeeded));
        assert!(!s.cli_parity_fixture_ref.is_empty());
        assert!(!s.ui_parity_fixture_ref.is_empty());
        assert!(matches!(
            s.lifecycle_label,
            LifecycleLabel::Stable | LifecycleLabel::Beta | LifecycleLabel::Lts
        ));
    }
}

#[test]
fn success_codes_pin_zero_and_partial_carrier_is_flagged() {
    let c = catalog();
    for s in &c.surfaces {
        for row in &s.result_code_catalog {
            if row.result_code.is_success() {
                assert_eq!(
                    row.numeric_code, 0,
                    "{}: success code must be 0",
                    s.surface_id
                );
            }
            if row.result_code == ResultCode::PartialSuccessWithWarnings {
                assert!(row.partial_result, "{}: partial carrier flag", s.surface_id);
            }
        }
        // Coupling: a partial/degraded state implies the carrier code and vice versa.
        assert_eq!(
            s.declares_partial_or_degraded(),
            s.has_partial_result_carrier(),
            "{}: partial state and carrier must agree",
            s.surface_id
        );
    }
}

#[test]
fn resolves_schema_ref_and_lifecycle_label() {
    let c = catalog();
    let (schema_ref, label) = c
        .resolve_surface_schema("command_inspect")
        .expect("command_inspect resolves");
    assert!(schema_ref.ends_with("command_descriptors.schema.json"));
    assert_eq!(label, LifecycleLabel::Stable);
    assert!(c.resolve_surface_schema("not_a_surface").is_none());
}

#[test]
fn surface_kinds_partition_the_set() {
    let c = catalog();
    let total: usize = SurfaceKind::ALL
        .iter()
        .map(|k| c.surfaces_for_kind(*k).len())
        .sum();
    assert_eq!(total, c.surfaces.len());
    assert!(!c.surfaces_for_kind(SurfaceKind::Health).is_empty());
    assert!(!c.surfaces_for_kind(SurfaceKind::Export).is_empty());
}

#[test]
fn result_codes_reuse_the_full_exit_code_vocabulary() {
    let c = catalog();
    // The catalog publishes the same closed vocabulary the CLI output registry
    // froze, so CLI and desktop key off one set of enums.
    assert_eq!(c.result_codes, ResultCode::ALL);
}

#[test]
fn offline_bundle_is_runtime_free() {
    let c = catalog();
    assert!(c.offline_bundle.mirrorable);
    assert!(!c.offline_bundle.requires_runtime_service);
    assert!(!c.offline_bundle.bundle_members.is_empty());
}

#[test]
fn duplicate_surface_id_is_rejected() {
    let mut c = catalog();
    let dup = c.surfaces[0].clone();
    c.surfaces.push(dup);
    c.summary = c.computed_summary();
    let violations = c.validate();
    assert!(
        violations
            .iter()
            .any(|v| v.check_id == "surfaces.duplicate_surface_id"),
        "duplicate surface id must be rejected: {violations:#?}"
    );
}

#[test]
fn summary_drift_is_rejected() {
    let mut c = catalog();
    c.summary.total_surfaces += 1;
    assert!(c
        .validate()
        .iter()
        .any(|v| v.check_id == "summary.count_mismatch"));
}
