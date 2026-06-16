//! Contract tests for the checked-in M5 publication-certification register and
//! its negative-fixture corpus.

use std::path::PathBuf;

use aureline_release::certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows::{
    current_m5_publication_cert_register, M5PublicationCertRegister, NarrowingReason,
    PublicationDimension, M5_PUBLICATION_CERT_RECORD_KIND, M5_PUBLICATION_CERT_SCHEMA_VERSION,
};
use aureline_release::{M5ArtifactFamilyKind, PromotionDecision};

fn register() -> M5PublicationCertRegister {
    current_m5_publication_cert_register().expect("checked-in register parses into the model")
}

fn fixture_dir() -> PathBuf {
    PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/release/m5/certify_artifact_graph_publication_exact_build_provenance_rollback_revocation_parity_and_mirror_offline_release_truth_on_all_claimed_m5_rows"
    ))
}

#[test]
fn checked_in_register_parses_and_validates() {
    let register = register();
    assert_eq!(register.schema_version, M5_PUBLICATION_CERT_SCHEMA_VERSION);
    assert_eq!(register.record_kind, M5_PUBLICATION_CERT_RECORD_KIND);
    let violations = register.validate();
    assert!(
        violations.is_empty(),
        "checked-in register must validate cleanly: {violations:#?}"
    );
}

#[test]
fn every_artifact_family_is_certified_across_every_dimension() {
    let register = register();
    for kind in M5ArtifactFamilyKind::ALL {
        let rows = register.rows_for_kind(kind);
        assert!(!rows.is_empty(), "missing family kind {}", kind.as_str());
        for row in rows {
            for dimension in PublicationDimension::ALL {
                assert!(
                    row.cell(dimension).is_some(),
                    "family {} missing dimension {}",
                    row.entry_id,
                    dimension.as_str()
                );
            }
        }
    }
}

#[test]
fn promotion_holds_on_stale_partial_or_missing_release_truth() {
    let register = register();
    assert_eq!(
        register.computed_promotion_decision(),
        PromotionDecision::Hold
    );
    assert_eq!(register.promotion.decision, PromotionDecision::Hold);
    assert_eq!(
        register.promotion.blocking_rule_ids,
        register.computed_blocking_rule_ids()
    );
    assert_eq!(
        register.promotion.blocking_claim_ids,
        register.computed_blocking_entry_ids()
    );
    assert!(!register.promotion.blocking_claim_ids.is_empty());
}

#[test]
fn track_invariant_and_guardrail_narrow_their_rows() {
    let register = register();

    // The track invariant: a publish target that inherits ambient credentials
    // narrows the family below the cutline.
    let ambient = register
        .rows
        .iter()
        .find(|row| row.has_active_reason(NarrowingReason::AmbientCredentialInherited))
        .expect("a family that inherits ambient credentials");
    assert!(ambient.publish_target.inherits_ambient_credentials);
    assert!(!ambient.publishes_stable());

    // The guardrail: a family without current mirror/offline drill evidence cannot
    // claim mirror/offline parity and narrows below the cutline.
    let mirror = register
        .rows
        .iter()
        .find(|row| row.has_active_reason(NarrowingReason::MirrorOfflineDrillStale))
        .expect("a family without current mirror parity");
    assert!(!mirror.mirror_offline.fully_proven());
    assert!(!mirror.publishes_stable());
}

#[test]
fn export_projection_round_trips() {
    let register = register();
    let projection = register.support_export_projection();
    assert_eq!(projection.rows.len(), register.rows.len());
    assert_eq!(projection.promotion_decision, register.promotion.decision);
}

#[test]
fn negative_fixtures_are_rejected_with_expected_violations() {
    #[derive(serde::Deserialize)]
    struct Case {
        file: String,
        expected_check_id: String,
    }
    #[derive(serde::Deserialize)]
    struct Cases {
        cases: Vec<Case>,
    }

    let dir = fixture_dir();
    let cases: Cases = serde_json::from_str(
        &std::fs::read_to_string(dir.join("cases.json")).expect("cases.json is readable"),
    )
    .expect("cases.json parses");
    assert!(
        !cases.cases.is_empty(),
        "cases.json declares no negative cases"
    );

    for case in cases.cases {
        let raw = std::fs::read_to_string(dir.join(&case.file))
            .unwrap_or_else(|e| panic!("fixture {} is readable: {e}", case.file));
        let register: M5PublicationCertRegister = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("fixture {} parses into the model: {e}", case.file));
        let violations = register.validate();
        assert!(
            !violations.is_empty(),
            "fixture {} must produce at least one violation",
            case.file
        );
        let found = violations
            .iter()
            .any(|v| format!("{v:?}").starts_with(&case.expected_check_id));
        assert!(
            found,
            "fixture {} must report {} violation; got {violations:#?}",
            case.file, case.expected_check_id
        );
    }
}
