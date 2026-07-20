// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Disposable regressions for fail-closed SBOM/provenance source validation.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicU64, Ordering};

use aureline_notices::generate_notice_bundle;

static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct TempRepo {
    root: PathBuf,
}

impl TempRepo {
    fn new() -> Self {
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "aureline-sbom-provenance-{}-{sequence}",
            std::process::id()
        ));
        fs::create_dir(&root).expect("create disposable provenance fixture");
        Self { root }
    }

    fn path(&self) -> &Path {
        &self.root
    }

    fn write(&self, relative: &str, contents: &str) {
        let path = self.root.join(relative);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create fixture parent");
        }
        fs::write(path, contents).expect("write fixture file");
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("remove disposable provenance fixture");
    }
}

fn seed_fixture() -> TempRepo {
    let fixture = TempRepo::new();
    fixture.write(
        "Cargo.toml",
        r#"[workspace]
members = ["crates/app"]

[workspace.package]
license = "Apache-2.0"
"#,
    );
    fixture.write(
        "crates/app/Cargo.toml",
        r#"[package]
name = "fixture-app"
version = "0.1.0"
license.workspace = true
"#,
    );
    fixture.write(
        "Cargo.lock",
        r#"version = 3

[[package]]
name = "fixture-app"
version = "0.1.0"

[[package]]
name = "fixture-dependency"
version = "1.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
"#,
    );
    fixture.write("rust-toolchain.toml", "[toolchain]\nchannel = \"1.84.0\"\n");
    fixture.write(".cargo/config.toml", "[net]\noffline = true\n");
    fixture.write(
        "build_identity.json",
        r#"{
  "schema_version": 1,
  "commit": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
  "commit_short": "aaaaaaaaaaaa",
  "dirty": false,
  "toolchain_channel": "1.84.0",
  "rustc_version": "rustc 1.84.0 (fixture)",
  "cargo_version": "cargo 1.84.0 (fixture)",
  "host_triple": "x86_64-fixture-linux-gnu",
  "target_triple": "x86_64-fixture-linux-gnu",
  "profile": "dev",
  "workspace_version": "0.1.0",
  "source_date_epoch": 0,
  "build_timestamp_utc": "1970-01-01T00:00:00Z"
}
"#,
    );
    fixture.write(
        "artifacts/governance/dependency_register.yaml",
        r#"schema_version: 1
rows:
  - id: dep.fixture.dependency
    name: fixture-dependency
    dependency_kind: cargo_crate
    admission_state: admitted_repo_tooling
    license_class: permissive_oss
    provenance_status: pinned_in_repo_and_observed
    sbom_inclusion_class: runtime_dependency_when_manifested
    protected_path: false
"#,
    );
    fixture.write(
        "artifacts/governance/third_party_import_register.yaml",
        r#"schema_version: 1
rows:
  - id: import.fixture.asset
    name: Fixture asset
    import_kind: bundled_asset_subset
    admission_state: reserved_not_yet_imported
    license_class: permissive_oss
    provenance_status: reserved_by_contract_not_imported
    sbom_inclusion_class: bundled_asset_when_imported
"#,
    );
    fixture.write(
        "artifacts/governance/release_notice_seed.yaml",
        r#"schema_version: 1
rows:
  - source_register: dependency_register
    source_id: dep.fixture.dependency
    publication_targets: [spdx_sbom, provenance_statement]
  - source_register: third_party_import_register
    source_id: import.fixture.asset
    publication_targets: [third_party_notice, spdx_sbom]
"#,
    );
    fixture.write(
        "artifacts/governance/third_party_import_manifest.yaml",
        r#"schema_version: 1
rows:
  - row_id: fixture.first_party
    source_class: first_party_release_artifact
    source_register: repository
    source_id: null
    license_expression: Apache-2.0
    reuse_spdx_state: covered_by_repository_default
  - row_id: fixture.import
    source_class: third_party_reserved_import
    source_register: third_party_import_register
    source_id: import.fixture.asset
    license_expression: NOASSERTION
    reuse_spdx_state: third_party_notice_pending_first_import
"#,
    );
    fixture.write(
        "artifacts/governance/critical_upstream_health_register.yaml",
        r#"schema_version: 1
register_id: fixture.health
as_of: "2026-07-20"
status: review_required
rows: []
"#,
    );
    fixture.write(
        "artifacts/governance/compliance_checklist.yaml",
        r#"schema_version: 2
canonical_registers:
  dependency_register: artifacts/governance/dependency_register.yaml
  third_party_import_register: artifacts/governance/third_party_import_register.yaml
  release_notice_seed: artifacts/governance/release_notice_seed.yaml
"#,
    );
    fixture
}

#[test]
fn valid_fixture_projects_every_locked_external_dependency_with_checksum() {
    let fixture = seed_fixture();
    let bundle = generate_notice_bundle(fixture.path()).expect("generate fixture bundle");

    let external = bundle
        .cargo_lock
        .packages
        .iter()
        .filter(|package| package.source.is_some())
        .collect::<Vec<_>>();
    assert_eq!(external.len(), 1);
    assert_eq!(external[0].name, "fixture-dependency");
    assert_eq!(external[0].checksum.as_deref().map(str::len), Some(64));
}

#[test]
fn missing_import_register_fails_closed() {
    let fixture = seed_fixture();
    fs::remove_file(
        fixture
            .path()
            .join("artifacts/governance/third_party_import_register.yaml"),
    )
    .expect("remove import register");

    let error = generate_notice_bundle(fixture.path()).expect_err("missing register must fail");
    assert!(error
        .to_string()
        .contains("third_party_import_register.yaml"));
}

#[test]
fn malformed_register_and_incomplete_notice_join_fail_closed() {
    let fixture = seed_fixture();
    fixture.write(
        "artifacts/governance/third_party_import_register.yaml",
        "schema_version: 1\nrows: [\n",
    );
    let error = generate_notice_bundle(fixture.path()).expect_err("malformed YAML must fail");
    assert!(error.to_string().contains("parse"));

    fixture.write(
        "artifacts/governance/third_party_import_register.yaml",
        r#"schema_version: 1
rows:
  - id: import.fixture.asset
    name: Fixture asset
    import_kind: bundled_asset_subset
    admission_state: reserved_not_yet_imported
    license_class: permissive_oss
    provenance_status: reserved_by_contract_not_imported
    sbom_inclusion_class: bundled_asset_when_imported
"#,
    );
    fixture.write(
        "artifacts/governance/release_notice_seed.yaml",
        r#"schema_version: 1
rows:
  - source_register: dependency_register
    source_id: dep.fixture.dependency
    publication_targets: [spdx_sbom]
"#,
    );
    let error = generate_notice_bundle(fixture.path()).expect_err("missing source join must fail");
    assert!(error.to_string().contains("notice coverage is incomplete"));
}

#[test]
fn missing_or_invalid_external_checksum_fails_closed() {
    let fixture = seed_fixture();
    fixture.write(
        "Cargo.lock",
        r#"version = 3

[[package]]
name = "fixture-app"
version = "0.1.0"

[[package]]
name = "fixture-dependency"
version = "1.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "not-a-sha256"
"#,
    );
    let error = generate_notice_bundle(fixture.path()).expect_err("invalid checksum must fail");
    assert!(error.to_string().contains("invalid SHA-256 checksum"));

    fixture.write(
        "Cargo.lock",
        r#"version = 3

[[package]]
name = "fixture-app"
version = "0.1.0"

[[package]]
name = "fixture-dependency"
version = "1.2.3"
source = "registry+https://github.com/rust-lang/crates.io-index"
"#,
    );
    let error = generate_notice_bundle(fixture.path()).expect_err("missing checksum must fail");
    assert!(error.to_string().contains("missing a content checksum"));
}

#[cfg(unix)]
#[test]
fn executable_emits_nonempty_external_inventory_and_digested_inputs() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = seed_fixture();
    fixture.write(
        "sha256-fixture",
        "#!/bin/sh\nprintf 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa  %s\\n' \"$1\"\n",
    );
    let hasher = fixture.path().join("sha256-fixture");
    let mut permissions = fs::metadata(&hasher)
        .expect("hasher metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hasher, permissions).expect("make hasher executable");
    let out_dir = fixture.path().join("out");
    fs::create_dir(&out_dir).expect("create fixture output directory");

    let status = Command::new(env!("CARGO_BIN_EXE_aureline-sbom-provenance"))
        .arg("--repo-root")
        .arg(fixture.path())
        .arg("--out-dir")
        .arg(&out_dir)
        .arg("--build-identity")
        .arg(fixture.path().join("build_identity.json"))
        .arg("--sha256-program")
        .arg(&hasher)
        .status()
        .expect("run provenance fixture executable");
    assert!(status.success());

    let sbom: serde_json::Value = serde_json::from_slice(
        &fs::read(out_dir.join("sbom_workspace.json")).expect("read fixture SBOM"),
    )
    .expect("parse fixture SBOM");
    assert_eq!(sbom["external_dependency_count"], 1);
    assert_eq!(
        sbom["external_dependencies"][0]["checksum_sha256"],
        format!("sha256:{}", "a".repeat(64))
    );

    let provenance: serde_json::Value = serde_json::from_slice(
        &fs::read(out_dir.join("provenance_summary.json")).expect("read fixture provenance"),
    )
    .expect("parse fixture provenance");
    let inputs = provenance["input_artifacts"]
        .as_array()
        .expect("input_artifacts array");
    assert!(!inputs.is_empty());
    assert!(inputs.iter().all(|input| input["sha256"]
        .as_str()
        .is_some_and(|digest| digest.starts_with("sha256:") && digest.len() == 71)));
}

#[cfg(unix)]
#[test]
fn executable_rejects_invalid_digest_output_without_publishing_artifacts() {
    use std::os::unix::fs::PermissionsExt;

    let fixture = seed_fixture();
    fixture.write("sha256-invalid", "#!/bin/sh\nprintf 'not-a-digest\\n'\n");
    let hasher = fixture.path().join("sha256-invalid");
    let mut permissions = fs::metadata(&hasher)
        .expect("hasher metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&hasher, permissions).expect("make hasher executable");
    let out_dir = fixture.path().join("out");
    fs::create_dir(&out_dir).expect("create fixture output directory");

    let status = Command::new(env!("CARGO_BIN_EXE_aureline-sbom-provenance"))
        .arg("--repo-root")
        .arg(fixture.path())
        .arg("--out-dir")
        .arg(&out_dir)
        .arg("--build-identity")
        .arg(fixture.path().join("build_identity.json"))
        .arg("--sha256-program")
        .arg(&hasher)
        .status()
        .expect("run provenance fixture executable");
    assert!(!status.success());
    assert!(!out_dir.join("sbom_workspace.json").exists());
    assert!(!out_dir.join("provenance_summary.json").exists());
}
