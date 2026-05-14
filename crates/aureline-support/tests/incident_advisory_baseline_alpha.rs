//! Protected tests for the incident/advisory baseline support projection.

use std::path::{Path, PathBuf};

use aureline_support::advisory_baseline::{
    current_alpha_affected_build_scope, AdvisoryBaselineSupportProjection,
    INCIDENT_ADVISORY_SCOPE_RECORD_KIND, INCIDENT_ADVISORY_SUPPORT_PACKET_RECORD_KIND,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn assert_repo_ref_exists(root: &Path, reference: &str) {
    if reference.starts_with("build-id:")
        || reference.starts_with("support.")
        || reference.starts_with("support_")
        || reference.starts_with("release.")
        || reference.starts_with("advisory_")
        || reference.starts_with("rollback.")
        || reference.starts_with("checkpoint.")
        || reference.starts_with("card.")
    {
        return;
    }
    let path = reference.split('#').next().expect("split ref");
    assert!(root.join(path).exists(), "{reference} must resolve on disk");
}

fn support_projection() -> AdvisoryBaselineSupportProjection {
    current_alpha_affected_build_scope()
        .expect("scope parses")
        .support_projection(
            "support.packet.alpha.incident_advisory_baseline",
            "2026-05-14T10:05:00Z",
        )
}

#[test]
fn checked_in_scope_validates_and_cites_existing_contracts() {
    let root = repo_root();
    let scope = current_alpha_affected_build_scope().expect("scope parses");

    assert_eq!(scope.record_kind, INCIDENT_ADVISORY_SCOPE_RECORD_KIND);
    assert_eq!(scope.validate(), Vec::new());
    assert_eq!(scope.affected_builds.len(), 2);
    assert_eq!(scope.advisory.advisory_id, "AURELINE-ADV-2026-0201");

    for reference in scope.source_contract_refs.values() {
        assert_repo_ref_exists(&root, reference);
    }
    for reference in &scope.acceptance.proof_refs {
        assert_repo_ref_exists(&root, reference);
    }
    for reference in &scope.acceptance.protected_fixture_refs {
        assert_repo_ref_exists(&root, reference);
    }

    for row in &scope.affected_builds {
        assert!(scope
            .advisory
            .affected_install_linkage
            .exact_build_identity_refs
            .contains(&row.exact_build_identity_ref));
        assert_repo_ref_exists(&root, &row.exact_build_identity_source_ref);
        assert_repo_ref_exists(&root, &row.rollback_route.rollback_sequence_ref);
        for reference in &row.known_limits.known_limit_refs {
            assert_repo_ref_exists(&root, reference);
        }
    }
}

#[test]
fn support_projection_preserves_affected_build_mitigation_rollback_and_known_limits() {
    let projection = support_projection();

    assert_eq!(
        projection.record_kind,
        INCIDENT_ADVISORY_SUPPORT_PACKET_RECORD_KIND
    );
    assert_eq!(projection.rows.len(), 2);
    assert!(projection.is_export_safe());
    assert!(projection.raw_private_material_excluded);
    assert_eq!(projection.redaction_class, "metadata_safe_default");
    assert!(projection
        .exact_build_identity_refs
        .iter()
        .any(|reference| reference.contains("0.8.0-alpha.1")));
    assert!(projection
        .exact_build_identity_refs
        .iter()
        .any(|reference| reference.contains("dev:0.0.0")));
    assert!(projection
        .rollback_route_refs
        .contains(&"rollback.target.preview.previous_verified_build".to_owned()));
    assert!(projection
        .rollback_route_refs
        .contains(&"rollback.target.hold_alpha_publication_until_packet_current".to_owned()));
    assert!(projection
        .known_limit_refs
        .contains(&"artifacts/release/protected_fitness_packet_alpha.yaml".to_owned()));
    assert!(projection
        .known_limit_refs
        .contains(&"artifacts/support/diagnosis_latency_scorecard_alpha.yaml".to_owned()));
    assert!(projection
        .support_packet_refs
        .contains(&"support.packet.alpha.incident_advisory_baseline".to_owned()));

    for row in &projection.rows {
        assert!(row
            .exact_build_identity_ref
            .starts_with("build-id:aureline:"));
        assert!(!row.known_limit_refs.is_empty());
        assert!(!row.support_packet_refs.is_empty());
        assert!(row.export_safe_summary.contains("rollback"));
    }
}

#[test]
fn baseline_docs_keep_reusable_template_and_acceptance_markers() {
    let root = repo_root();
    let template = std::fs::read_to_string(root.join("docs/security/advisory_template_seed.md"))
        .expect("read template");
    let baseline =
        std::fs::read_to_string(root.join("artifacts/release/incident_advisory_baseline_alpha.md"))
            .expect("read baseline");

    for marker in [
        "Affected Build Scope",
        "Current Mitigation",
        "Rollback Route",
        "Known Limits",
        "Support And Export Refs",
    ] {
        assert!(template.contains(marker), "template missing {marker}");
    }

    for marker in [
        "AURELINE-ADV-2026-0201",
        "build-id:aureline:preview:0.8.0-alpha.1",
        "build-id:aureline:dev:0.0.0",
        "evidence_stale",
        "metadata-only",
    ] {
        assert!(baseline.contains(marker), "baseline missing {marker}");
    }
}
