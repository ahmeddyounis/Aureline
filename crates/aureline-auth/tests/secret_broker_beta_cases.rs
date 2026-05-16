//! Fixture-driven coverage for the secret-broker beta projection.
//!
//! These tests parse the protected fixtures under
//! `/fixtures/security/m3/secret_broker`, validate the seeded page, and prove
//! that every drill fixture surfaces the expected typed defect kind. They
//! also confirm that the support-export wrapper preserves consumer lineage
//! while excluding raw secret values and runtime handle ids from the export.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_auth::{
    audit_secret_broker_beta_page, validate_secret_broker_beta_page, ConsumerAuditOutcomeClass,
    SecretBrokerBetaDefectKind, SecretBrokerBetaPage, SecretBrokerBetaProfileClass,
    SecretBrokerBetaSupportExport, SecretReferenceMode,
};

fn fixture_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/security/m3/secret_broker")
}

fn load_page(file_name: &str) -> SecretBrokerBetaPage {
    let path = fixture_dir().join(file_name);
    let body = fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("fixture {} must read: {err}", path.display()));
    serde_json::from_str(&body)
        .unwrap_or_else(|err| panic!("fixture {file_name} must parse as a page: {err}"))
}

#[test]
fn seeded_page_fixture_validates_with_zero_defects() {
    let page = load_page("page.json");
    validate_secret_broker_beta_page(&page).expect("seeded page validates");
    assert!(page.defects.is_empty());
    for profile in SecretBrokerBetaProfileClass::ALL {
        assert!(page
            .summary
            .profiles_present
            .iter()
            .any(|token| token == profile.as_str()));
    }
    let modes: Vec<&str> = page
        .summary
        .reference_modes_present
        .iter()
        .map(String::as_str)
        .collect();
    assert!(modes.contains(&"handle"));
    assert!(modes.contains(&"delegated"));
    assert!(modes.contains(&"session_only"));
}

#[test]
fn consumer_audit_preserves_lineage_for_every_row() {
    let page = load_page("page.json");
    for row in &page.handle_rows {
        let events: Vec<_> = page
            .consumer_audit
            .iter()
            .filter(|event| event.secret_broker_row_ref == row.secret_broker_row_id)
            .collect();
        assert!(
            !events.is_empty(),
            "row {} missing consumer audit events",
            row.secret_broker_row_id
        );
        for event in events {
            assert_eq!(event.consumer.consumer_id, row.consumer.consumer_id);
            assert_eq!(event.target_ref, row.target_ref);
            assert_eq!(event.workspace_scope_ref, row.workspace_scope_ref);
            assert_eq!(event.secret_class, row.secret_class);
            assert_eq!(event.profile, row.profile);
            assert!(!event.raw_secret_material_present);
            assert!(!event.raw_handle_id_exposed);
            assert!(event.no_public_endpoint_fallback);
            if event.outcome.is_denial() {
                assert!(event.denial_note.is_some());
            }
        }
    }
}

#[test]
fn drill_raw_secret_material_surfaces_typed_defect() {
    let page = load_page("drill_raw_secret_material.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind == SecretBrokerBetaDefectKind::RawSecretMaterialPresent));
}

#[test]
fn drill_managed_authority_missing_signature_surfaces_typed_defect() {
    let page = load_page("drill_managed_authority_missing_signature.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind
            == SecretBrokerBetaDefectKind::ManagedAuthorityMissingSignature));
}

#[test]
fn drill_consumer_audit_missing_surfaces_typed_defect() {
    let page = load_page("drill_consumer_audit_missing.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind == SecretBrokerBetaDefectKind::ConsumerAuditMissing));
}

#[test]
fn drill_denied_audit_missing_reason_surfaces_typed_defect() {
    let page = load_page("drill_denied_audit_missing_reason.json");
    assert!(page
        .defects
        .iter()
        .any(|defect| defect.defect_kind == SecretBrokerBetaDefectKind::DeniedAuditMissingReason));
}

#[test]
fn defects_match_validator_output_for_each_drill() {
    for drill in [
        "drill_raw_secret_material.json",
        "drill_managed_authority_missing_signature.json",
        "drill_consumer_audit_missing.json",
        "drill_denied_audit_missing_reason.json",
    ] {
        let page = load_page(drill);
        let recomputed = audit_secret_broker_beta_page(&page.handle_rows, &page.consumer_audit);
        assert_eq!(recomputed, page.defects, "{drill} validator drift");
    }
}

#[test]
fn support_export_preserves_consumer_lineage_and_excludes_raw_material() {
    let page = load_page("page.json");
    let export = SecretBrokerBetaSupportExport::from_page(
        "secret-broker-beta:support-export:test",
        "2026-05-16T05:00:00Z",
        page,
    );
    assert!(export.raw_secret_values_excluded);
    assert!(export.raw_handle_ids_excluded);
    assert!(export.consumer_lineage_preserved);
    assert!(export.defect_kinds_present.is_empty());

    let encoded = serde_json::to_string(&export).expect("serialize support export");
    // Raw runtime handle ids must not appear in the exported text.
    assert!(!encoded.contains("credential-handle:registry:npm:payments:0001"));
    assert!(!encoded.contains("session-secret:provider:byok-ai:local-only:0001"));
    assert!(!encoded.contains("delegated-credential:tunnel:fleet:0001"));
}

#[test]
fn audit_outcomes_cover_handle_grants_and_typed_denial_vocabulary() {
    let page = load_page("page.json");
    let outcomes: Vec<ConsumerAuditOutcomeClass> = page
        .consumer_audit
        .iter()
        .map(|event| event.outcome)
        .collect();
    assert!(outcomes.contains(&ConsumerAuditOutcomeClass::GrantedHandle));
    assert!(outcomes.contains(&ConsumerAuditOutcomeClass::GrantedDelegated));
    assert!(outcomes.contains(&ConsumerAuditOutcomeClass::GrantedSessionOnly));
    assert!(outcomes.contains(&ConsumerAuditOutcomeClass::DeniedByPlaintextRequested));
    assert!(outcomes.contains(&ConsumerAuditOutcomeClass::DeniedByPublicEndpointFallback));
    assert!(outcomes.contains(&ConsumerAuditOutcomeClass::DeniedByStaleHandleReuse));
    assert!(outcomes.contains(&ConsumerAuditOutcomeClass::DeniedBySilentInMemoryPromotion));
    assert!(outcomes.contains(&ConsumerAuditOutcomeClass::DeniedByPolicy));

    for event in &page.consumer_audit {
        if event.outcome.is_grant() {
            let mode = event.granted_reference_mode.expect("grant has mode");
            assert_eq!(mode, event.outcome.implied_reference_mode().unwrap());
        } else {
            assert_eq!(event.granted_reference_mode, None::<SecretReferenceMode>);
        }
    }
}
