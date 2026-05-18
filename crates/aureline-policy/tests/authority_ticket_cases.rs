use std::path::{Path, PathBuf};

use aureline_policy::{
    audit_authority_ticket_page, validate_authority_ticket_page, AuthorityEvaluationOutcome,
    AuthorityTicketClass, AuthorityTicketDefectKind, AuthorityTicketPage,
    AuthorityTicketSupportExport, CredentialProjectionMode,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../fixtures/security/m3/credential_projection_and_privileged_attach")
}

fn load_page(file_name: &str) -> AuthorityTicketPage {
    let path = fixture_dir().join(file_name);
    let body = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a page: {err}"))
}

#[test]
fn seeded_page_fixture_validates() {
    let page = load_page("page.json");
    validate_authority_ticket_page(&page).expect("fixture validates");
    assert!(page.defects.is_empty());
    for ticket_class in AuthorityTicketClass::ALL {
        assert!(page
            .summary
            .ticket_classes_present
            .iter()
            .any(|token| token == ticket_class.as_str()));
    }
}

#[test]
fn seeded_page_covers_projection_modes_and_privileged_denials() {
    let page = load_page("page.json");
    for mode in [
        CredentialProjectionMode::DelegatedHandle,
        CredentialProjectionMode::SessionOnlySecret,
        CredentialProjectionMode::SignOnly,
    ] {
        assert!(page
            .summary
            .credential_projection_modes_present
            .iter()
            .any(|token| token == mode.as_str()));
    }
    for outcome in [
        AuthorityEvaluationOutcome::Admitted,
        AuthorityEvaluationOutcome::DeniedMissingTicket,
        AuthorityEvaluationOutcome::DeniedPolicyEpochDrift,
        AuthorityEvaluationOutcome::DeniedAuthoritySourceMismatch,
    ] {
        assert!(page
            .summary
            .spend_attempts_by_outcome
            .contains_key(outcome.as_str()));
    }
}

#[test]
fn support_export_preserves_lineage_without_raw_credentials() {
    let page = load_page("page.json");
    let export = AuthorityTicketSupportExport::from_page(
        "authority-ticket:support-export:test",
        "2026-05-18T10:30:00Z",
        page,
    );
    assert!(export.privileged_action_lineage_preserved);
    assert!(export.raw_credentials_excluded);
    assert!(export.root_authority_proof_refs_preserved);
    assert!(!export.page.tickets.is_empty());
    assert!(!export.page.credential_projections.is_empty());
    assert!(!export.page.root_authority_changes.is_empty());
}

#[test]
fn fixture_audit_matches_validator_recompute() {
    let page = load_page("page.json");
    let recomputed = audit_authority_ticket_page(
        &page.tickets,
        &page.credential_projections,
        &page.root_authority_changes,
        &page.spend_attempts,
    );
    assert!(recomputed.is_empty(), "fixture must hold zero defects");
}

#[test]
fn drill_raw_secret_projection_surfaces_typed_defect() {
    let page = load_page("drill_raw_secret_projection.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind == AuthorityTicketDefectKind::RawSecretMaterialPresent));
}

#[test]
fn drill_unsigned_root_change_surfaces_typed_defect() {
    let page = load_page("drill_unsigned_root_change.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind == AuthorityTicketDefectKind::RootAuthorityProofMissing));
}

#[test]
fn drill_admitted_without_ticket_surfaces_typed_defect() {
    let page = load_page("drill_admitted_without_ticket.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind == AuthorityTicketDefectKind::SpendAdmittedWithoutTicket));
}
