//! Fixture replay for the offline policy-bundle and entitlement verifier
//! beta projection.
//!
//! The fixtures live under
//! `fixtures/security/m3/offline_entitlement_verifier/` and are generated
//! by the `aureline_shell_offline_entitlement_beta` headless inspector,
//! so checked-in JSON stays a literal projection of the seeded page.

use aureline_shell::offline_entitlement_beta::{
    audit_offline_entitlement_verifier_beta_rows, seeded_offline_entitlement_verifier_beta_page,
    validate_offline_entitlement_verifier_beta_page, OfflineEntitlementVerifierBetaDefectKind,
    OfflineEntitlementVerifierBetaPage, OfflineEntitlementVerifierBetaRenderSummary,
    OfflineEntitlementVerifierBetaRow, OfflineEntitlementVerifierBetaSupportExport,
    VerifierOutcomeClass, OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION,
    OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF,
};

const FIXTURE_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../fixtures/security/m3/offline_entitlement_verifier"
);

fn load<T: serde::de::DeserializeOwned>(filename: &str) -> T {
    let path = format!("{}/{}", FIXTURE_DIR, filename);
    let body =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"));
    serde_json::from_str(&body).unwrap_or_else(|err| panic!("failed to parse {path}: {err}"))
}

#[test]
fn page_fixture_matches_seeded_builder() {
    let from_file: OfflineEntitlementVerifierBetaPage = load("page.json");
    let from_code = seeded_offline_entitlement_verifier_beta_page();
    assert_eq!(
        from_file, from_code,
        "fixture must be a literal projection of the seeded page; regenerate via the headless bin"
    );
}

#[test]
fn page_fixture_validates_with_zero_defects() {
    let page: OfflineEntitlementVerifierBetaPage = load("page.json");
    assert_eq!(
        page.shared_contract_ref,
        OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF
    );
    assert_eq!(
        page.schema_version,
        OFFLINE_ENTITLEMENT_VERIFIER_BETA_SCHEMA_VERSION
    );
    assert!(page.defects.is_empty());
    validate_offline_entitlement_verifier_beta_page(&page).expect("seeded page must validate");
}

#[test]
fn rows_fixture_matches_page_rows_and_covers_required_outcomes() {
    let page: OfflineEntitlementVerifierBetaPage = load("page.json");
    let rows: Vec<OfflineEntitlementVerifierBetaRow> = load("rows.json");
    assert_eq!(page.rows, rows);

    let outcomes: Vec<_> = rows.iter().map(|row| row.outcome_class).collect();
    assert!(outcomes.contains(&VerifierOutcomeClass::VerifiedLive));
    assert!(outcomes.contains(&VerifierOutcomeClass::VerifiedMirror));
    assert!(outcomes.contains(&VerifierOutcomeClass::VerifiedAirGapped));
    assert!(outcomes.contains(&VerifierOutcomeClass::VerifiedManualImport));
    assert!(outcomes.contains(&VerifierOutcomeClass::Expired));
    assert!(outcomes.contains(&VerifierOutcomeClass::SignatureMissing));
    assert!(outcomes.contains(&VerifierOutcomeClass::UntrustedSigner));
    assert!(outcomes.contains(&VerifierOutcomeClass::UnsignedLocalAdvisory));
}

#[test]
fn support_export_round_trips_through_validator() {
    let export: OfflineEntitlementVerifierBetaSupportExport = load("support_export.json");
    assert!(export.raw_private_material_excluded);
    assert!(export.defect_kinds_present.is_empty());
    assert_eq!(
        export.shared_contract_ref,
        OFFLINE_ENTITLEMENT_VERIFIER_BETA_SHARED_CONTRACT_REF
    );
    assert_eq!(export.support_rows.len(), export.page.rows.len());
}

#[test]
fn render_summary_matches_page_summary() {
    let from_file: OfflineEntitlementVerifierBetaRenderSummary = load("render_summary.json");
    let page: OfflineEntitlementVerifierBetaPage = load("page.json");
    let from_page = OfflineEntitlementVerifierBetaRenderSummary::from_page(&page);
    assert_eq!(from_file, from_page);
}

#[test]
fn drill_expired_bundle_accepted_replays_typed_defect() {
    let page: OfflineEntitlementVerifierBetaPage = load("drill_expired_bundle_accepted.json");
    let recomputed = audit_offline_entitlement_verifier_beta_rows(&page.rows);
    assert_eq!(recomputed, page.defects);
    assert!(recomputed.iter().any(|defect| defect.defect_kind
        == OfflineEntitlementVerifierBetaDefectKind::ExpiredBundleAcceptedWithoutDowngrade));
}

#[test]
fn drill_untrusted_signer_accepted_replays_typed_defect() {
    let page: OfflineEntitlementVerifierBetaPage = load("drill_untrusted_signer_accepted.json");
    let recomputed = audit_offline_entitlement_verifier_beta_rows(&page.rows);
    assert_eq!(recomputed, page.defects);
    assert!(recomputed.iter().any(|defect| defect.defect_kind
        == OfflineEntitlementVerifierBetaDefectKind::UntrustedSignerAccepted));
}

#[test]
fn drill_local_editing_blocked_replays_typed_defect() {
    let page: OfflineEntitlementVerifierBetaPage = load("drill_local_editing_blocked.json");
    let recomputed = audit_offline_entitlement_verifier_beta_rows(&page.rows);
    assert_eq!(recomputed, page.defects);
    assert!(recomputed.iter().any(|defect| defect.defect_kind
        == OfflineEntitlementVerifierBetaDefectKind::LocalEditingBlockedOnFailedVerification));
}
