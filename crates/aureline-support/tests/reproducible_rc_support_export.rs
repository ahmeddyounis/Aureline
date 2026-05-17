//! Protected tests for reproducible release-candidate support exports.

use std::collections::BTreeSet;

use aureline_support::reproducible_rc::{
    current_reproducible_rc_support_export, REPRODUCIBLE_RC_SUPPORT_EXPORT_RECORD_KIND,
    REPRODUCIBLE_RC_SUPPORT_EXPORT_SCHEMA_VERSION,
};

#[test]
fn generated_support_export_validates_and_blocks_mismatches() {
    let export = current_reproducible_rc_support_export().expect("support export parses");

    assert_eq!(export.validate(), Vec::new());
    assert_eq!(
        export.schema_version,
        REPRODUCIBLE_RC_SUPPORT_EXPORT_SCHEMA_VERSION
    );
    assert_eq!(
        export.record_kind,
        REPRODUCIBLE_RC_SUPPORT_EXPORT_RECORD_KIND
    );
    assert!(export.raw_private_material_excluded);
    assert_eq!(export.redaction_class, "metadata_only_no_package_bytes");
    assert_eq!(export.summary.required_artifact_count, 15);
    assert_eq!(export.summary.matched_artifact_count, 14);
    assert_eq!(export.summary.mismatched_artifact_count, 0);
    assert_eq!(export.summary.non_comparable_artifact_count, 1);
    assert_eq!(export.summary.publication_check_count, 5);
    assert_eq!(export.summary.blocking_failure_count, 0);
    assert!(!export.summary.byte_identity_claimed);
}

#[test]
fn artifact_checks_quote_one_exact_build_identity() {
    let export = current_reproducible_rc_support_export().expect("support export parses");

    for row in &export.artifact_graph_checks {
        assert_eq!(
            row.exact_build_identity_ref,
            export.exact_build_identity_ref
        );
        assert!(row.support_ref.starts_with(
            "artifacts/release/m3/reproducible_rc_packet/support_export_projection.json#"
        ));
        if row.comparison_state == "matched" {
            assert!(row.digest_match);
            assert_eq!(row.promoted_digest, row.rebuilt_digest);
            assert!(row
                .promoted_digest
                .as_deref()
                .unwrap()
                .starts_with("sha256:"));
        }
    }

    let states = export
        .artifact_graph_checks
        .iter()
        .map(|row| row.comparison_state.as_str())
        .collect::<BTreeSet<_>>();
    assert_eq!(
        states,
        BTreeSet::from(["matched", "self_referential_projection"])
    );
}

#[test]
fn validation_rejects_private_material_mismatch_and_nonblocking_checks() {
    let mut export = current_reproducible_rc_support_export().expect("support export parses");
    export.raw_private_material_excluded = false;
    export.artifact_graph_checks[0].rebuilt_digest =
        Some("sha256:0000000000000000000000000000000000000000000000000000000000000000".to_owned());
    export.publication_checks[0].actual_state = "failed".to_owned();
    export.publication_checks[0].blocks_publication = false;

    let violations = export.validate();
    let check_ids = violations
        .iter()
        .map(|violation| violation.check_id.as_str())
        .collect::<BTreeSet<_>>();

    assert!(check_ids.contains("support_export.raw_private_material_excluded"));
    assert!(check_ids.contains("artifact_graph_check.digest_mismatch"));
    assert!(check_ids.contains("publication_check.state_mismatch"));
    assert!(check_ids.contains("publication_check.not_blocking"));
}
