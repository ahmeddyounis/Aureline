//! Integration tests for the notice, SBOM, and critical-upstream projections.

use std::path::{Path, PathBuf};

use aureline_notices::{
    generate_notice_bundle, RedRiskClass, SPDX_NOASSERTION, WORKSPACE_LICENSE_EXPRESSION,
};

const EXPECTED_LOCKFILE_PACKAGE_COUNT: usize = 490;
const EXPECTED_LOCKFILE_FINGERPRINT: &str = "lock-fnv64:9ac5be305e2d2e97";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("derive repo root")
        .to_path_buf()
}

#[test]
fn notice_digest_covers_lockfile_exactly() {
    let bundle = generate_notice_bundle(repo_root()).expect("generate notice bundle");

    assert_eq!(
        bundle.cargo_lock.package_count, EXPECTED_LOCKFILE_PACKAGE_COUNT,
        "Cargo.lock package count drifted; update the notice digest test snapshot intentionally"
    );
    assert_eq!(
        bundle.cargo_lock.lockfile_fingerprint, EXPECTED_LOCKFILE_FINGERPRINT,
        "Cargo.lock package graph drifted; update the notice digest test snapshot intentionally"
    );
    assert!(bundle.notice_digest.covers_lockfile());
    assert_eq!(
        bundle.notice_digest.cargo_lock_fingerprint,
        bundle.cargo_lock.lockfile_fingerprint
    );

    let grouped_count: usize = bundle
        .notice_digest
        .license_groups
        .iter()
        .map(|group| group.package_count)
        .sum();
    assert_eq!(grouped_count, bundle.cargo_lock.package_count);
}

#[test]
fn spdx_record_set_matches_workspace_license_without_license_text() {
    let bundle = generate_notice_bundle(repo_root()).expect("generate notice bundle");

    assert_eq!(
        bundle.spdx_sbom.record_kind,
        "aureline_spdx_sbom_record_set"
    );
    assert_eq!(
        bundle.spdx_sbom.workspace_license_expression,
        WORKSPACE_LICENSE_EXPRESSION
    );
    assert_eq!(
        bundle.spdx_sbom.packages.len(),
        bundle.cargo_lock.package_count
    );

    let workspace_packages = bundle
        .spdx_sbom
        .packages
        .iter()
        .filter(|package| package.supplier == "Organization: Aureline")
        .collect::<Vec<_>>();
    assert_eq!(
        workspace_packages.len(),
        bundle.workspace.members.len(),
        "every workspace member should be represented once in the SPDX record set"
    );
    assert!(workspace_packages.iter().all(|package| {
        package.license_declared == WORKSPACE_LICENSE_EXPRESSION
            && package.license_concluded == WORKSPACE_LICENSE_EXPRESSION
    }));
    assert!(bundle
        .spdx_sbom
        .packages
        .iter()
        .filter(|package| package.supplier != "Organization: Aureline")
        .all(|package| package.license_declared == SPDX_NOASSERTION));
}

#[test]
fn cyclonedx_projection_tracks_the_same_package_set() {
    let bundle = generate_notice_bundle(repo_root()).expect("generate notice bundle");

    assert_eq!(
        bundle.cyclonedx.record_kind,
        "aureline_cyclonedx_projection"
    );
    assert_eq!(bundle.cyclonedx.bom_format, "CycloneDX");
    assert_eq!(bundle.cyclonedx.spec_version, "1.5");
    assert_eq!(
        bundle.cyclonedx.components.len(),
        bundle.cargo_lock.package_count
    );

    let component_refs = bundle
        .cyclonedx
        .components
        .iter()
        .map(|component| component.bom_ref.as_str())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(component_refs.len(), bundle.cyclonedx.components.len());
    assert!(bundle.cyclonedx.components.iter().any(|component| {
        component.name == "serde"
            && component.license_expression == SPDX_NOASSERTION
            && !component.hashes.is_empty()
    }));
}

#[test]
fn critical_upstream_register_flags_required_red_risk_classes() {
    let bundle = generate_notice_bundle(repo_root()).expect("generate notice bundle");
    let health = &bundle.critical_upstream_health;

    assert_eq!(
        health.record_kind,
        "aureline_critical_upstream_health_register"
    );
    assert!(!health.red_risk_rows.is_empty());
    assert!(health.contains_risk_class(RedRiskClass::SingleMaintainer));
    assert!(health.contains_risk_class(RedRiskClass::Unmaintained));
    assert!(health.contains_risk_class(RedRiskClass::LicenseIncompatible));
    assert!(health.red_risk_rows.iter().all(|row| {
        row.risk_state == "red"
            && !row.owner_dri.is_empty()
            && !row.fork_replace_escalate_trigger.is_empty()
    }));
}
