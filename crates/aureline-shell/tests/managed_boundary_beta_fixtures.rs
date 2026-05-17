//! Fixture replay for the managed-boundary, org-switch, seat-quota,
//! grace-window, and offboarding beta projection.
//!
//! The fixtures under `fixtures/security/m3/managed_boundary/` are generated
//! by the `aureline_shell_managed_boundary_beta` headless inspector, so the
//! checked-in JSON stays a literal projection of the published boundary
//! manifest.

use aureline_shell::managed_boundary::{
    audit_managed_boundary_beta_rows, seeded_managed_boundary_beta_page,
    validate_managed_boundary_beta_page, BoundaryClass, ManagedBoundaryBetaDefect,
    ManagedBoundaryBetaGraceWindowRow, ManagedBoundaryBetaOffboardingRow,
    ManagedBoundaryBetaOrgSwitchRow, ManagedBoundaryBetaPage, ManagedBoundaryBetaRenderSummary,
    ManagedBoundaryBetaRow, ManagedBoundaryBetaSeatQuotaRow, ManagedBoundaryBetaSummary,
    ManagedBoundaryBetaSupportExport, MANAGED_BOUNDARY_BETA_SCHEMA_VERSION,
    MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/security/m3/managed_boundary"
);

fn load<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn page_fixture_matches_seeded_builder() {
    let from_file: ManagedBoundaryBetaPage = load("page.json");
    let from_code = seeded_managed_boundary_beta_page();
    assert_eq!(
        from_file, from_code,
        "fixture must be a literal projection of the published manifest; regenerate via the headless bin"
    );
}

#[test]
fn page_fixture_validates_with_zero_defects() {
    let page: ManagedBoundaryBetaPage = load("page.json");
    assert_eq!(
        page.shared_contract_ref,
        MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF
    );
    assert_eq!(page.schema_version, MANAGED_BOUNDARY_BETA_SCHEMA_VERSION);
    assert!(page.defects.is_empty());
    validate_managed_boundary_beta_page(&page).expect("published manifest must validate");
}

#[test]
fn rows_fixture_matches_page_rows_and_covers_required_classes() {
    let page: ManagedBoundaryBetaPage = load("page.json");
    let rows: Vec<ManagedBoundaryBetaRow> = load("rows.json");
    assert_eq!(page.rows, rows);

    let classes: Vec<BoundaryClass> = rows.iter().map(|row| row.boundary_class).collect();
    assert!(classes.contains(&BoundaryClass::LocalOnly));
    assert!(classes.contains(&BoundaryClass::Mirrored));
    assert!(classes.contains(&BoundaryClass::SelfHosted));
    assert!(classes.contains(&BoundaryClass::Managed));
    assert!(classes.contains(&BoundaryClass::PaidSeatBound));
}

#[test]
fn org_switch_rows_fixture_matches_projection() {
    let page: ManagedBoundaryBetaPage = load("page.json");
    let from_file: Vec<ManagedBoundaryBetaOrgSwitchRow> = load("org_switch_rows.json");
    let from_page = page.org_switch_projection();
    assert_eq!(from_file, from_page);
    for row in &from_file {
        assert_eq!(row.behavior_class.as_str(), row.behavior_class_token);
    }
}

#[test]
fn seat_quota_rows_fixture_matches_projection() {
    let page: ManagedBoundaryBetaPage = load("page.json");
    let from_file: Vec<ManagedBoundaryBetaSeatQuotaRow> = load("seat_quota_rows.json");
    let from_page = page.seat_quota_projection();
    assert_eq!(from_file, from_page);
    for row in &from_file {
        assert_eq!(row.quota_state.as_str(), row.quota_state_token);
    }
}

#[test]
fn grace_window_rows_fixture_matches_projection() {
    let page: ManagedBoundaryBetaPage = load("page.json");
    let from_file: Vec<ManagedBoundaryBetaGraceWindowRow> = load("grace_window_rows.json");
    let from_page = page.grace_window_projection();
    assert_eq!(from_file, from_page);
    for row in &from_file {
        assert_eq!(row.window_class.as_str(), row.window_class_token);
    }
}

#[test]
fn offboarding_rows_fixture_matches_projection() {
    let page: ManagedBoundaryBetaPage = load("page.json");
    let from_file: Vec<ManagedBoundaryBetaOffboardingRow> = load("offboarding_rows.json");
    let from_page = page.offboarding_projection();
    assert_eq!(from_file, from_page);
    for row in &from_file {
        assert_eq!(
            row.export_packet_class.as_str(),
            row.export_packet_class_token
        );
    }
}

#[test]
fn defects_fixture_replays_through_validator() {
    let defects: Vec<ManagedBoundaryBetaDefect> = load("defects.json");
    let page: ManagedBoundaryBetaPage = load("page.json");
    let recomputed =
        audit_managed_boundary_beta_rows(&page.rows, &page.required_boundary_class_coverage);
    assert_eq!(defects, recomputed);
    assert!(defects.is_empty());
}

#[test]
fn summary_fixture_matches_page_summary() {
    let from_file: ManagedBoundaryBetaSummary = load("summary.json");
    let page: ManagedBoundaryBetaPage = load("page.json");
    assert_eq!(from_file, page.summary);
}

#[test]
fn render_summary_fixture_matches_page_summary() {
    let from_file: ManagedBoundaryBetaRenderSummary = load("render_summary.json");
    let page: ManagedBoundaryBetaPage = load("page.json");
    let from_page = ManagedBoundaryBetaRenderSummary::from_page(&page);
    assert_eq!(from_file, from_page);
}

#[test]
fn support_export_round_trips_with_invariants_preserved() {
    let export: ManagedBoundaryBetaSupportExport = load("support_export.json");
    assert!(export.raw_private_material_excluded);
    assert!(export.no_public_endpoint_fallback_invariant);
    assert!(export.local_core_continuity_invariant);
    assert!(export.absence_narrowing_invariant);
    assert_eq!(
        export.shared_contract_ref,
        MANAGED_BOUNDARY_BETA_SHARED_CONTRACT_REF
    );
    assert_eq!(export.org_switch_rows.len(), export.page.rows.len());
    assert_eq!(export.seat_quota_rows.len(), export.page.rows.len());
    assert_eq!(export.grace_window_rows.len(), export.page.rows.len());
    assert_eq!(export.offboarding_rows.len(), export.page.rows.len());
    assert!(export.defect_kinds_present.is_empty());
}
