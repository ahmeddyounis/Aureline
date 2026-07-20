// SPDX-FileCopyrightText: 2026 Aureline contributors
// SPDX-License-Identifier: Apache-2.0

//! Emit the fail-closed structural SBOM and provenance summary used by CI.

use std::collections::BTreeMap;
use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use aureline_notices::{generate_notice_bundle, NoticeBundle, SPDX_NOASSERTION};
use serde::{Deserialize, Serialize};

const SBOM_FILE_NAME: &str = "sbom_workspace.json";
const PROVENANCE_FILE_NAME: &str = "provenance_summary.json";

const REQUIRED_REPOSITORY_INPUTS: &[(&str, &str)] = &[
    ("workspace_manifest", "Cargo.toml"),
    ("cargo_lock", "Cargo.lock"),
    ("toolchain_pin", "rust-toolchain.toml"),
    ("cargo_configuration", ".cargo/config.toml"),
    (
        "dependency_register",
        "artifacts/governance/dependency_register.yaml",
    ),
    (
        "third_party_import_register",
        "artifacts/governance/third_party_import_register.yaml",
    ),
    (
        "release_notice_seed",
        "artifacts/governance/release_notice_seed.yaml",
    ),
    (
        "compliance_checklist",
        "artifacts/governance/compliance_checklist.yaml",
    ),
    (
        "third_party_import_manifest",
        "artifacts/governance/third_party_import_manifest.yaml",
    ),
    (
        "critical_upstream_health_register",
        "artifacts/governance/critical_upstream_health_register.yaml",
    ),
];

#[derive(Debug)]
struct Args {
    repo_root: PathBuf,
    out_dir: PathBuf,
    build_identity: PathBuf,
    sha256_program: PathBuf,
    sha256_args: Vec<String>,
}

impl Args {
    fn parse() -> Result<Self, String> {
        let mut repo_root = None;
        let mut out_dir = None;
        let mut build_identity = None;
        let mut sha256_program = None;
        let mut sha256_args = Vec::new();
        let mut args = env::args().skip(1);

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--repo-root" => repo_root = Some(next_value(&mut args, "--repo-root")?),
                "--out-dir" => out_dir = Some(next_value(&mut args, "--out-dir")?),
                "--build-identity" => {
                    build_identity = Some(next_value(&mut args, "--build-identity")?)
                }
                "--sha256-program" => {
                    sha256_program = Some(next_value(&mut args, "--sha256-program")?)
                }
                "--sha256-arg" => sha256_args.push(next_value(&mut args, "--sha256-arg")?),
                "--help" | "-h" => return Err(usage()),
                _ if arg.starts_with("--sha256-arg=") => {
                    sha256_args.push(arg["--sha256-arg=".len()..].to_owned());
                }
                _ => return Err(format!("unknown argument: {arg}\n{}", usage())),
            }
        }

        let parsed = Self {
            repo_root: PathBuf::from(repo_root.ok_or_else(|| missing_arg("--repo-root"))?),
            out_dir: PathBuf::from(out_dir.ok_or_else(|| missing_arg("--out-dir"))?),
            build_identity: PathBuf::from(
                build_identity.ok_or_else(|| missing_arg("--build-identity"))?,
            ),
            sha256_program: PathBuf::from(
                sha256_program.ok_or_else(|| missing_arg("--sha256-program"))?,
            ),
            sha256_args,
        };
        if !parsed.sha256_program.is_absolute() {
            return Err("--sha256-program must resolve to an absolute executable path".to_owned());
        }
        Ok(parsed)
    }
}

fn next_value(args: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    args.next()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("{flag} requires a value"))
}

fn missing_arg(flag: &str) -> String {
    format!("missing required argument {flag}\n{}", usage())
}

fn usage() -> String {
    "Usage: aureline-sbom-provenance --repo-root PATH --out-dir PATH \
--build-identity PATH --sha256-program ABSOLUTE_PATH [--sha256-arg ARG ...]"
        .to_owned()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BuildIdentity {
    schema_version: u32,
    commit: String,
    commit_short: String,
    dirty: bool,
    toolchain_channel: String,
    rustc_version: String,
    cargo_version: String,
    host_triple: String,
    target_triple: String,
    profile: String,
    workspace_version: String,
    source_date_epoch: u64,
    build_timestamp_utc: String,
}

impl BuildIdentity {
    fn read(path: &Path) -> Result<Self, Box<dyn Error>> {
        let raw = fs::read_to_string(path)
            .map_err(|error| format!("read build identity {}: {error}", path.display()))?;
        let identity: Self = serde_json::from_str(&raw)
            .map_err(|error| format!("parse build identity {}: {error}", path.display()))?;
        identity.validate()?;
        Ok(identity)
    }

    fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.schema_version != 1 {
            return Err(format!(
                "build identity schema_version must be 1, got {}",
                self.schema_version
            )
            .into());
        }
        if self.commit != "unknown" && !is_lower_hex(&self.commit, 40) {
            return Err("build identity commit must be a 40-character lowercase hex digest".into());
        }
        if self.commit == "unknown" {
            if self.commit_short != "unknown" {
                return Err("unknown build commit must use commit_short=unknown".into());
            }
        } else if self.commit_short.len() != 12 || !self.commit.starts_with(&self.commit_short) {
            return Err(
                "build identity commit_short must be the first 12 commit characters".into(),
            );
        }
        for (field, value) in [
            ("toolchain_channel", self.toolchain_channel.as_str()),
            ("rustc_version", self.rustc_version.as_str()),
            ("cargo_version", self.cargo_version.as_str()),
            ("host_triple", self.host_triple.as_str()),
            ("target_triple", self.target_triple.as_str()),
            ("workspace_version", self.workspace_version.as_str()),
        ] {
            if value.trim().is_empty() {
                return Err(format!("build identity {field} must not be empty").into());
            }
        }
        if !matches!(self.profile.as_str(), "dev" | "release") {
            return Err(format!(
                "build identity profile must be dev or release, got {}",
                self.profile
            )
            .into());
        }
        if !looks_like_utc_timestamp(&self.build_timestamp_utc) {
            return Err(
                "build identity build_timestamp_utc must be an ISO-8601 UTC instant".into(),
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize)]
struct InputArtifactDigest {
    role: String,
    path: String,
    sha256: String,
}

#[derive(Debug, Serialize)]
struct WorkspaceSbomProjection<'a> {
    schema_version: u32,
    format: &'static str,
    format_note: &'static str,
    authority_scope: &'static str,
    build_identity_ref: String,
    build_identity_sha256: String,
    build_identity: &'a BuildIdentity,
    workspace_license: &'a str,
    cargo_lock: CargoLockProjection<'a>,
    workspace_member_count: usize,
    workspace_members: &'a [aureline_notices::WorkspaceCrate],
    external_dependency_count: usize,
    external_dependencies: Vec<ExternalDependencyProjection>,
}

#[derive(Debug, Serialize)]
struct CargoLockProjection<'a> {
    path: &'static str,
    sha256: String,
    package_count: usize,
    structural_fingerprint: &'a str,
}

#[derive(Debug, Serialize)]
struct ExternalDependencyProjection {
    name: String,
    version: String,
    source: String,
    checksum_sha256: String,
    cargo_lock_package_ref: String,
    license_declared: String,
    license_source_refs: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ProvenanceSummary<'a> {
    schema_version: u32,
    format: &'static str,
    format_note: &'static str,
    build_identity_ref: String,
    build_identity_sha256: String,
    sbom_ref: &'static str,
    sbom_sha256: String,
    build_identity: &'a BuildIdentity,
    input_artifacts: Vec<InputArtifactDigest>,
    coverage: ProvenanceCoverage,
    integrity: ProvenanceIntegrity,
    attestations: Vec<String>,
    signatures: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ProvenanceCoverage {
    cargo_lock_package_count: usize,
    workspace_member_count: usize,
    external_dependency_count: usize,
}

#[derive(Debug, Serialize)]
struct ProvenanceIntegrity {
    source_artifact_digests: &'static str,
    external_dependency_checksums: &'static str,
    sbom_digest: &'static str,
}

struct Sha256Hasher<'a> {
    program: &'a Path,
    args: &'a [String],
}

impl Sha256Hasher<'_> {
    fn file(&self, path: &Path) -> Result<String, Box<dyn Error>> {
        let output = Command::new(self.program)
            .args(self.args)
            .arg(path)
            .output()
            .map_err(|error| {
                format!(
                    "run SHA-256 program {} for {}: {error}",
                    self.program.display(),
                    path.display()
                )
            })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!(
                "SHA-256 program {} failed for {}: {}",
                self.program.display(),
                path.display(),
                stderr.trim()
            )
            .into());
        }
        let stdout = String::from_utf8(output.stdout)
            .map_err(|error| format!("SHA-256 output was not UTF-8: {error}"))?;
        parse_sha256_output(&stdout).ok_or_else(|| {
            format!(
                "SHA-256 program {} returned an invalid digest for {}",
                self.program.display(),
                path.display()
            )
            .into()
        })
    }
}

fn main() {
    if let Err(error) = run() {
        eprintln!("[sbom-provenance] error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = Args::parse().map_err(|error| format!("argument error: {error}"))?;
    let repo_root = args
        .repo_root
        .canonicalize()
        .map_err(|error| format!("resolve repo root {}: {error}", args.repo_root.display()))?;
    if !args.out_dir.is_dir() {
        return Err(format!(
            "output directory does not exist: {}",
            args.out_dir.display()
        )
        .into());
    }
    let hasher = Sha256Hasher {
        program: &args.sha256_program,
        args: &args.sha256_args,
    };

    let identity = BuildIdentity::read(&args.build_identity)?;
    let bundle = generate_notice_bundle(&repo_root)?;
    let input_artifacts = collect_input_digests(&repo_root, &bundle, &hasher)?;
    let build_identity_sha256 = prefixed_sha256(hasher.file(&args.build_identity)?);
    let cargo_lock_sha256 = input_artifacts
        .iter()
        .find(|input| input.path == "Cargo.lock")
        .map(|input| input.sha256.clone())
        .ok_or("Cargo.lock digest was not captured")?;

    let external_dependencies = external_dependency_projection(&bundle)?;
    let sbom = WorkspaceSbomProjection {
        schema_version: 2,
        format: "aureline-workspace-sbom-projection",
        format_note: "Structural, checksum-complete Cargo.lock projection. This is not a conformant SPDX or CycloneDX document.",
        authority_scope: "Authoritative only for the checked-in Cargo.lock package identities, sources, and registry checksums; license conclusions remain NOASSERTION unless backed by a reviewed register row.",
        build_identity_ref: file_name(&args.build_identity)?,
        build_identity_sha256: build_identity_sha256.clone(),
        build_identity: &identity,
        workspace_license: &bundle.workspace.workspace_license_expression,
        cargo_lock: CargoLockProjection {
            path: "Cargo.lock",
            sha256: cargo_lock_sha256,
            package_count: bundle.cargo_lock.package_count,
            structural_fingerprint: &bundle.cargo_lock.lockfile_fingerprint,
        },
        workspace_member_count: bundle.workspace.members.len(),
        workspace_members: &bundle.workspace.members,
        external_dependency_count: external_dependencies.len(),
        external_dependencies,
    };

    let sbom_path = args.out_dir.join(SBOM_FILE_NAME);
    write_json_new(&sbom_path, &sbom)?;
    let sbom_sha256 = prefixed_sha256(hasher.file(&sbom_path)?);

    let external_dependency_count = sbom.external_dependency_count;
    let provenance = ProvenanceSummary {
        schema_version: 2,
        format: "aureline-provenance-summary",
        format_note: "Unsigned structural provenance summary. Release-grade signed attestation remains out of scope.",
        build_identity_ref: file_name(&args.build_identity)?,
        build_identity_sha256,
        sbom_ref: SBOM_FILE_NAME,
        sbom_sha256,
        build_identity: &identity,
        input_artifacts,
        coverage: ProvenanceCoverage {
            cargo_lock_package_count: bundle.cargo_lock.package_count,
            workspace_member_count: bundle.workspace.members.len(),
            external_dependency_count,
        },
        integrity: ProvenanceIntegrity {
            source_artifact_digests: "complete_sha256",
            external_dependency_checksums: "complete_sha256",
            sbom_digest: "sha256",
        },
        attestations: Vec::new(),
        signatures: Vec::new(),
    };
    write_json_new(&args.out_dir.join(PROVENANCE_FILE_NAME), &provenance)?;
    Ok(())
}

fn collect_input_digests(
    repo_root: &Path,
    bundle: &NoticeBundle,
    hasher: &Sha256Hasher<'_>,
) -> Result<Vec<InputArtifactDigest>, Box<dyn Error>> {
    let mut inputs = Vec::new();
    for (role, relative_path) in REQUIRED_REPOSITORY_INPUTS {
        let path = repo_root.join(relative_path);
        if !path.is_file() {
            return Err(format!("missing required provenance input: {relative_path}").into());
        }
        inputs.push(InputArtifactDigest {
            role: (*role).to_owned(),
            path: (*relative_path).to_owned(),
            sha256: prefixed_sha256(hasher.file(&path)?),
        });
    }
    for member in &bundle.workspace.members {
        let path = repo_root.join(&member.manifest_path);
        inputs.push(InputArtifactDigest {
            role: "workspace_member_manifest".to_owned(),
            path: member.manifest_path.clone(),
            sha256: prefixed_sha256(hasher.file(&path)?),
        });
    }
    Ok(inputs)
}

fn external_dependency_projection(
    bundle: &NoticeBundle,
) -> Result<Vec<ExternalDependencyProjection>, Box<dyn Error>> {
    let spdx_by_ref = bundle
        .spdx_sbom
        .packages
        .iter()
        .map(|package| (package.cargo_lock_package_ref.as_str(), package))
        .collect::<BTreeMap<_, _>>();
    let mut dependencies = Vec::new();
    for package in bundle
        .cargo_lock
        .packages
        .iter()
        .filter(|package| package.source.is_some())
    {
        let package_ref = package.cargo_lock_package_ref();
        let spdx = spdx_by_ref
            .get(package_ref.as_str())
            .ok_or_else(|| format!("SBOM package projection missing {package_ref}"))?;
        dependencies.push(ExternalDependencyProjection {
            name: package.name.clone(),
            version: package.version.clone(),
            source: package
                .source
                .clone()
                .ok_or("external package source missing")?,
            checksum_sha256: prefixed_sha256(
                package
                    .checksum
                    .clone()
                    .ok_or("external package checksum missing")?,
            ),
            cargo_lock_package_ref: package_ref,
            license_declared: if spdx.license_declared.is_empty() {
                SPDX_NOASSERTION.to_owned()
            } else {
                spdx.license_declared.clone()
            },
            license_source_refs: spdx.license_source_refs.clone(),
        });
    }
    Ok(dependencies)
}

fn write_json_new(path: &Path, value: &impl Serialize) -> Result<(), Box<dyn Error>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|error| format!("create output {}: {error}", path.display()))?;
    file.write_all(&bytes)
        .map_err(|error| format!("write output {}: {error}", path.display()))?;
    file.sync_all()
        .map_err(|error| format!("sync output {}: {error}", path.display()))?;
    Ok(())
}

fn file_name(path: &Path) -> Result<String, Box<dyn Error>> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_owned)
        .ok_or_else(|| format!("path has no UTF-8 file name: {}", path.display()).into())
}

fn parse_sha256_output(output: &str) -> Option<String> {
    let candidate = output.split_whitespace().next()?;
    if is_lower_or_upper_hex(candidate, 64) {
        Some(candidate.to_ascii_lowercase())
    } else {
        None
    }
}

fn prefixed_sha256(digest: String) -> String {
    format!("sha256:{digest}")
}

fn is_lower_hex(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_lower_or_upper_hex(value: &str, expected_len: usize) -> bool {
    value.len() == expected_len && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn looks_like_utc_timestamp(value: &str) -> bool {
    value.len() == 20
        && value.as_bytes()[4] == b'-'
        && value.as_bytes()[7] == b'-'
        && value.as_bytes()[10] == b'T'
        && value.as_bytes()[13] == b':'
        && value.as_bytes()[16] == b':'
        && value.as_bytes()[19] == b'Z'
        && value.bytes().enumerate().all(|(index, byte)| {
            matches!(index, 4 | 7 | 10 | 13 | 16 | 19) || byte.is_ascii_digit()
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_parser_rejects_absent_and_malformed_digests() {
        assert_eq!(
            parse_sha256_output(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef  file"
            ),
            Some("0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef".to_owned())
        );
        assert_eq!(parse_sha256_output(""), None);
        assert_eq!(parse_sha256_output("not-a-digest  file"), None);
        assert_eq!(
            parse_sha256_output(&format!("{}  file", "a".repeat(63))),
            None
        );
    }

    #[test]
    fn build_identity_validation_rejects_inconsistent_commit_prefix() {
        let identity = BuildIdentity {
            schema_version: 1,
            commit: "a".repeat(40),
            commit_short: "b".repeat(12),
            dirty: false,
            toolchain_channel: "1.84.0".to_owned(),
            rustc_version: "rustc 1.84.0".to_owned(),
            cargo_version: "cargo 1.84.0".to_owned(),
            host_triple: "x86_64-test".to_owned(),
            target_triple: "x86_64-test".to_owned(),
            profile: "dev".to_owned(),
            workspace_version: "0.0.0".to_owned(),
            source_date_epoch: 0,
            build_timestamp_utc: "1970-01-01T00:00:00Z".to_owned(),
        };
        assert!(identity.validate().is_err());
    }
}
