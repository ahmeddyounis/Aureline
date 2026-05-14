//! Protected tests for alpha publication dry-run reconstruction.

use std::path::{Path, PathBuf};

use aureline_support::publication_dry_run::{
    current_alpha_publication_manifest, ALPHA_PUBLICATION_MANIFEST_RECORD_KIND,
    CURRENT_ALPHA_PUBLICATION_MANIFEST_PATH,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

fn assert_repo_ref_exists(root: &Path, reference: &str) {
    if reference.starts_with("artifact_node:")
        || reference.starts_with("artifact_bundle:")
        || reference.starts_with("artifact_family:")
        || reference.starts_with("artifact_verification_row:")
        || reference.starts_with("blocker.")
        || reference.starts_with("build-id:")
        || reference.starts_with("channel.")
        || reference.starts_with("digest.")
        || reference.starts_with("import_instruction:")
        || reference.starts_with("mirror_integrity_packet:")
        || reference.starts_with("offline_verification_packet:")
        || reference.starts_with("policy_pack:")
        || reference.starts_with("publication_posture:")
        || reference.starts_with("receipt:")
        || reference.starts_with("release_candidate:")
    {
        return;
    }
    let path = reference.split('#').next().expect("split ref");
    assert!(root.join(path).exists(), "{reference} must resolve on disk");
}

#[test]
fn checked_in_publication_manifest_validates_and_resolves_sources() {
    let root = repo_root();
    let manifest = current_alpha_publication_manifest().expect("manifest parses");

    assert_eq!(manifest.record_kind, ALPHA_PUBLICATION_MANIFEST_RECORD_KIND);
    assert_eq!(manifest.validate(), Vec::new());
    assert_eq!(
        CURRENT_ALPHA_PUBLICATION_MANIFEST_PATH,
        "artifacts/release/alpha_publication_manifest.yaml"
    );
    assert!(!manifest.broader_publication_allowed);
    assert!(manifest
        .exact_build_identity_ref
        .starts_with("build-id:aureline:preview:0.8.0-alpha.1"));

    for reference in manifest.source_contract_refs.values() {
        assert_repo_ref_exists(&root, reference);
    }
    for row in &manifest.artifact_family_rows {
        for reference in row
            .source_refs
            .iter()
            .chain(row.digest_material_refs.iter())
        {
            assert_repo_ref_exists(&root, reference);
        }
    }
    for receipt in &manifest.verification_receipts {
        for reference in &receipt.evidence_refs {
            assert_repo_ref_exists(&root, reference);
        }
    }
    for blocker in &manifest.blockers {
        for reference in &blocker.evidence_refs {
            assert_repo_ref_exists(&root, reference);
        }
    }
    for reference in &manifest.acceptance.protected_proof_refs {
        if !reference.ends_with("alpha_publication_dry_run_validation_capture.json") {
            assert_repo_ref_exists(&root, reference);
        }
    }
}

#[test]
fn support_projection_names_offline_coverage_and_live_truth_degradation() {
    let manifest = current_alpha_publication_manifest().expect("manifest parses");
    let projection = manifest.support_projection();

    assert!(manifest.exercises_required_postures());
    assert!(manifest.all_required_families_are_vendor_unreachable_verifiable());
    assert!(projection.raw_private_material_excluded);
    assert_eq!(projection.receipt_count, 21);
    assert!(projection.blocking_blocker_count >= 6);

    for family in [
        "binaries",
        "docs_help_packs",
        "policy_bundles",
        "symbols",
        "support_schemas",
        "notices",
        "sbom_provenance",
    ] {
        assert!(
            projection
                .vendor_unreachable_family_keys
                .contains(&family.to_owned()),
            "projection missing vendor-unreachable family {family}"
        );
    }
    for posture in ["mirror_only", "deny_all", "offline_verification"] {
        assert!(
            projection.posture_classes.contains(&posture.to_owned()),
            "projection missing posture {posture}"
        );
    }
    for truth_class in ["live_service_health", "advisory", "revocation"] {
        assert!(
            projection
                .live_truth_degradation_classes
                .contains(&truth_class.to_owned()),
            "projection missing live truth degradation {truth_class}"
        );
    }
}

#[test]
fn blockers_prevent_broader_publication_until_evidence_closes() {
    let manifest = current_alpha_publication_manifest().expect("manifest parses");
    let blocking_ids = manifest
        .blockers
        .iter()
        .filter(|blocker| blocker.blocks_broader_publication)
        .map(|blocker| blocker.blocker_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    for blocker in [
        "blocker.publish.package_bytes_missing",
        "blocker.fitness.evidence_stale",
        "blocker.policy.not_in_alpha_artifact_graph",
        "blocker.notice.reserved_imports_pending",
        "blocker.sbom.placeholder_only",
        "blocker.attestation.signing_absent",
    ] {
        assert!(
            blocking_ids.contains(blocker),
            "expected blocking publication gap {blocker}"
        );
    }
}
