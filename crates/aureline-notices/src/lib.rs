//! Typed notice, SBOM, and critical-upstream projections for repository compliance.
//!
//! The crate reads the workspace manifests, `Cargo.lock`, and the checked-in
//! governance registers, then projects them into redaction-safe records that
//! support release evidence and support-bundle previews. It references SPDX
//! license identifiers only; it never embeds third-party license text.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Workspace license expected by the repository-level compliance contract.
pub const WORKSPACE_LICENSE_EXPRESSION: &str = "Apache-2.0";

/// SPDX token used when a lockfile package has no reviewed license evidence.
pub const SPDX_NOASSERTION: &str = "NOASSERTION";

const NOTICE_BUNDLE_SCHEMA_VERSION: u32 = 1;
const SPDX_RECORD_SET_SCHEMA_VERSION: u32 = 1;
const CYCLONEDX_PROJECTION_SCHEMA_VERSION: u32 = 1;
const NOTICE_DIGEST_SCHEMA_VERSION: u32 = 1;
const CRITICAL_UPSTREAM_SCHEMA_VERSION: u32 = 1;

/// Errors raised while reading or projecting notice inputs.
#[derive(Debug)]
pub enum NoticeError {
    /// A repository file could not be read.
    Io {
        /// Path that failed to read.
        path: PathBuf,
        /// Source I/O error.
        source: std::io::Error,
    },
    /// A governance YAML file could not be parsed.
    Yaml {
        /// Path that failed to parse.
        path: PathBuf,
        /// Source YAML error.
        source: serde_yaml::Error,
    },
    /// A required workspace or package manifest field is missing.
    Manifest {
        /// Path or logical source containing the invalid manifest.
        path: PathBuf,
        /// Reviewer-facing validation message.
        message: String,
    },
    /// `Cargo.lock` did not contain a complete package entry.
    CargoLock {
        /// Reviewer-facing validation message.
        message: String,
    },
}

impl fmt::Display for NoticeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => write!(f, "read {}: {source}", path.display()),
            Self::Yaml { path, source } => write!(f, "parse {}: {source}", path.display()),
            Self::Manifest { path, message } => {
                write!(f, "invalid manifest {}: {message}", path.display())
            }
            Self::CargoLock { message } => write!(f, "invalid Cargo.lock: {message}"),
        }
    }
}

impl Error for NoticeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Yaml { source, .. } => Some(source),
            Self::Manifest { .. } | Self::CargoLock { .. } => None,
        }
    }
}

/// Complete notice projection generated from repository-local inputs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeBundle {
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Source files consumed to generate the bundle.
    pub source_refs: Vec<String>,
    /// Workspace package and license summary.
    pub workspace: WorkspaceSummary,
    /// Parsed `Cargo.lock` package set and fingerprint.
    pub cargo_lock: CargoLockSnapshot,
    /// SPDX-shaped package record set for the lockfile graph.
    pub spdx_sbom: SpdxSbomRecordSet,
    /// CycloneDX-shaped component projection for the same lockfile graph.
    pub cyclonedx: CycloneDxProjection,
    /// Notice digest grouped by license family.
    pub notice_digest: NoticeDigest,
    /// Red-risk critical-upstream projection.
    pub critical_upstream_health: CriticalUpstreamHealthRegister,
}

impl NoticeBundle {
    /// Builds the support/export summary string used by metadata-only rows.
    pub fn summary_sentence(&self) -> String {
        format!(
            "Notice digest covers {} Cargo.lock package(s), {} license family group(s), and {} red-risk upstream row(s).",
            self.cargo_lock.package_count,
            self.notice_digest.license_groups.len(),
            self.critical_upstream_health.red_risk_rows.len()
        )
    }
}

/// Reads repository-local notice inputs and returns all generated projections.
///
/// # Errors
///
/// Returns an error when a source file cannot be read, a governance YAML file
/// does not parse, a manifest omits required fields, or `Cargo.lock` contains
/// an incomplete package entry.
pub fn generate_notice_bundle(repo_root: impl AsRef<Path>) -> Result<NoticeBundle, NoticeError> {
    let repo_root = repo_root.as_ref();
    let workspace = read_workspace_summary(repo_root)?;
    let lock_path = repo_root.join("Cargo.lock");
    let lock_text = read_to_string(&lock_path)?;
    let cargo_lock = CargoLockSnapshot::parse(&lock_text)?;

    let governance = GovernanceInputs::read(repo_root)?;
    let package_context = PackageContext::new(&workspace, &governance);
    let spdx_sbom = SpdxSbomRecordSet::from_lockfile(&cargo_lock, &package_context);
    let cyclonedx = CycloneDxProjection::from_lockfile(&cargo_lock, &package_context);
    let notice_digest = NoticeDigest::from_inputs(&cargo_lock, &workspace, &governance);
    let critical_upstream_health = CriticalUpstreamHealthRegister::from_inputs(&governance);

    Ok(NoticeBundle {
        schema_version: NOTICE_BUNDLE_SCHEMA_VERSION,
        record_kind: "aureline_notice_bundle".to_owned(),
        source_refs: vec![
            "Cargo.lock".to_owned(),
            "Cargo.toml".to_owned(),
            "crates/*/Cargo.toml".to_owned(),
            "artifacts/governance/dependency_register.yaml".to_owned(),
            "artifacts/governance/third_party_import_manifest.yaml".to_owned(),
            "artifacts/governance/release_notice_seed.yaml".to_owned(),
            "artifacts/governance/critical_upstream_health_register.yaml".to_owned(),
        ],
        workspace,
        cargo_lock,
        spdx_sbom,
        cyclonedx,
        notice_digest,
        critical_upstream_health,
    })
}

/// Workspace package and license summary derived from `Cargo.toml` files.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceSummary {
    /// Workspace-level license expression.
    pub workspace_license_expression: String,
    /// Workspace member crates.
    pub members: Vec<WorkspaceCrate>,
}

impl WorkspaceSummary {
    /// Returns true when `package_name` is a workspace member.
    pub fn contains_package(&self, package_name: &str) -> bool {
        self.members
            .iter()
            .any(|member| member.package_name == package_name)
    }
}

/// One Rust crate declared as a workspace member.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceCrate {
    /// Package name from the crate manifest.
    pub package_name: String,
    /// Path to the crate manifest relative to the repository root.
    pub manifest_path: String,
    /// Effective license expression for the package.
    pub license_expression: String,
}

/// Parsed package set from `Cargo.lock`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoLockSnapshot {
    /// Number of package entries in the lockfile.
    pub package_count: usize,
    /// Stable fingerprint over package names, versions, sources, checksums,
    /// and dependency entries in lockfile order.
    pub lockfile_fingerprint: String,
    /// Parsed package entries in lockfile order.
    pub packages: Vec<CargoLockPackage>,
}

impl CargoLockSnapshot {
    /// Parses `Cargo.lock` package entries.
    ///
    /// # Errors
    ///
    /// Returns an error when a package entry omits `name` or `version`.
    pub fn parse(lockfile: &str) -> Result<Self, NoticeError> {
        let mut packages = Vec::new();
        let mut current: Option<PartialLockPackage> = None;
        let mut in_dependencies = false;

        for raw_line in lockfile.lines() {
            let line = raw_line.trim();
            if line == "[[package]]" {
                if let Some(package) = current.take() {
                    packages.push(package.finish()?);
                }
                current = Some(PartialLockPackage::default());
                in_dependencies = false;
                continue;
            }

            let Some(package) = current.as_mut() else {
                continue;
            };

            if in_dependencies {
                if line == "]" {
                    in_dependencies = false;
                } else if let Some(value) = quoted_value(line) {
                    package.dependencies.push(value);
                }
                continue;
            }

            if line.starts_with("dependencies = [") {
                in_dependencies = true;
                continue;
            }

            if let Some(value) = key_value(line, "name") {
                package.name = Some(value);
            } else if let Some(value) = key_value(line, "version") {
                package.version = Some(value);
            } else if let Some(value) = key_value(line, "source") {
                package.source = Some(value);
            } else if let Some(value) = key_value(line, "checksum") {
                package.checksum = Some(value);
            }
        }

        if let Some(package) = current.take() {
            packages.push(package.finish()?);
        }

        let lockfile_fingerprint = lockfile_fingerprint(&packages);
        Ok(Self {
            package_count: packages.len(),
            lockfile_fingerprint,
            packages,
        })
    }

    /// Returns the package reference strings in lockfile order.
    pub fn package_refs(&self) -> Vec<String> {
        self.packages
            .iter()
            .map(CargoLockPackage::cargo_lock_package_ref)
            .collect()
    }
}

/// One package entry parsed from `Cargo.lock`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CargoLockPackage {
    /// Cargo package name.
    pub name: String,
    /// Resolved package version.
    pub version: String,
    /// Package source, absent for workspace members.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Registry checksum when Cargo recorded one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    /// Raw dependency entries from `Cargo.lock`.
    pub dependencies: Vec<String>,
}

impl CargoLockPackage {
    /// Returns true when Cargo represented the package as a local workspace
    /// package.
    pub fn is_workspace_package(&self) -> bool {
        self.source.is_none()
    }

    /// Returns a stable package ref used across SPDX, CycloneDX, and notice rows.
    pub fn cargo_lock_package_ref(&self) -> String {
        match (&self.source, &self.checksum) {
            (Some(source), Some(checksum)) => {
                format!(
                    "cargo-lock:{}@{}:{}:{}",
                    self.name, self.version, source, checksum
                )
            }
            (Some(source), None) => {
                format!("cargo-lock:{}@{}:{}", self.name, self.version, source)
            }
            (None, _) => format!("cargo-lock:{}@{}:workspace", self.name, self.version),
        }
    }
}

#[derive(Default)]
struct PartialLockPackage {
    name: Option<String>,
    version: Option<String>,
    source: Option<String>,
    checksum: Option<String>,
    dependencies: Vec<String>,
}

impl PartialLockPackage {
    fn finish(self) -> Result<CargoLockPackage, NoticeError> {
        let name = self.name.ok_or_else(|| NoticeError::CargoLock {
            message: "package entry is missing name".to_owned(),
        })?;
        let version = self.version.ok_or_else(|| NoticeError::CargoLock {
            message: format!("package {name} is missing version"),
        })?;
        Ok(CargoLockPackage {
            name,
            version,
            source: self.source,
            checksum: self.checksum,
            dependencies: self.dependencies,
        })
    }
}

/// SPDX-shaped SBOM package record set generated from `Cargo.lock`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpdxSbomRecordSet {
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// SPDX version targeted by the record-set fields.
    pub spdx_version: String,
    /// SPDX data license identifier for generated document metadata.
    pub data_license: String,
    /// Human-readable document name.
    pub document_name: String,
    /// Stable namespace derived from the Cargo lockfile fingerprint.
    pub document_namespace: String,
    /// Workspace license expression used for first-party packages.
    pub workspace_license_expression: String,
    /// Number of packages read from `Cargo.lock`.
    pub cargo_lock_package_count: usize,
    /// Lockfile fingerprint the package rows were generated from.
    pub cargo_lock_fingerprint: String,
    /// SPDX package rows in lockfile order.
    pub packages: Vec<SpdxPackageRecord>,
}

impl SpdxSbomRecordSet {
    fn from_lockfile(lock: &CargoLockSnapshot, context: &PackageContext<'_>) -> Self {
        let packages = lock
            .packages
            .iter()
            .enumerate()
            .map(|(index, package)| {
                let attribution = context.attribution_for(package);
                SpdxPackageRecord {
                    spdx_id: format!("SPDXRef-Package-{}", sanitized_spdx_id(package, index)),
                    package_name: package.name.clone(),
                    version_info: package.version.clone(),
                    cargo_lock_package_ref: package.cargo_lock_package_ref(),
                    download_location: package
                        .source
                        .clone()
                        .unwrap_or_else(|| SPDX_NOASSERTION.to_owned()),
                    files_analyzed: false,
                    license_declared: attribution.spdx_license_expression.clone(),
                    license_concluded: attribution.spdx_license_expression,
                    license_source_refs: attribution.source_refs,
                    supplier: if package.is_workspace_package() {
                        "Organization: Aureline".to_owned()
                    } else {
                        SPDX_NOASSERTION.to_owned()
                    },
                    copyright_text: SPDX_NOASSERTION.to_owned(),
                }
            })
            .collect();

        Self {
            schema_version: SPDX_RECORD_SET_SCHEMA_VERSION,
            record_kind: "aureline_spdx_sbom_record_set".to_owned(),
            spdx_version: "SPDX-2.3".to_owned(),
            data_license: "CC0-1.0".to_owned(),
            document_name: "Aureline Cargo.lock package set".to_owned(),
            document_namespace: format!(
                "https://aureline.dev/spdx/cargo-lock/{}",
                lock.lockfile_fingerprint.replace(':', "-")
            ),
            workspace_license_expression: context.workspace.workspace_license_expression.clone(),
            cargo_lock_package_count: lock.package_count,
            cargo_lock_fingerprint: lock.lockfile_fingerprint.clone(),
            packages,
        }
    }
}

/// One SPDX package row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SpdxPackageRecord {
    /// SPDX package identifier.
    pub spdx_id: String,
    /// Package name.
    pub package_name: String,
    /// Package version.
    pub version_info: String,
    /// Cargo lockfile package ref.
    pub cargo_lock_package_ref: String,
    /// Download location or `NOASSERTION`.
    pub download_location: String,
    /// Whether file-level analysis is represented in this record.
    pub files_analyzed: bool,
    /// Declared license expression, or `NOASSERTION`.
    pub license_declared: String,
    /// Concluded license expression, or `NOASSERTION`.
    pub license_concluded: String,
    /// Source refs that justify the license expression.
    pub license_source_refs: Vec<String>,
    /// SPDX supplier field.
    pub supplier: String,
    /// SPDX copyright text field.
    pub copyright_text: String,
}

/// CycloneDX-shaped component projection generated from `Cargo.lock`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CycloneDxProjection {
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// CycloneDX format label.
    pub bom_format: String,
    /// CycloneDX specification version targeted by this projection.
    pub spec_version: String,
    /// Stable serial ref derived from the Cargo lockfile fingerprint.
    pub serial_ref: String,
    /// Number of packages read from `Cargo.lock`.
    pub cargo_lock_package_count: usize,
    /// Lockfile fingerprint the components were generated from.
    pub cargo_lock_fingerprint: String,
    /// Component rows in lockfile order.
    pub components: Vec<CycloneDxComponent>,
}

impl CycloneDxProjection {
    fn from_lockfile(lock: &CargoLockSnapshot, context: &PackageContext<'_>) -> Self {
        let components = lock
            .packages
            .iter()
            .enumerate()
            .map(|(index, package)| {
                let attribution = context.attribution_for(package);
                CycloneDxComponent {
                    bom_ref: format!("pkg:cargo/{}@{}#{}", package.name, package.version, index),
                    component_type: if package.is_workspace_package() {
                        "application".to_owned()
                    } else {
                        "library".to_owned()
                    },
                    name: package.name.clone(),
                    version: package.version.clone(),
                    package_url: format!("pkg:cargo/{}@{}", package.name, package.version),
                    scope: "required".to_owned(),
                    license_expression: attribution.spdx_license_expression,
                    license_source_refs: attribution.source_refs,
                    hashes: package
                        .checksum
                        .iter()
                        .map(|checksum| CycloneDxHash {
                            algorithm: "SHA-256".to_owned(),
                            value: checksum.clone(),
                        })
                        .collect(),
                    external_references: package
                        .source
                        .iter()
                        .map(|source| CycloneDxExternalReference {
                            reference_type: "distribution".to_owned(),
                            url: source.clone(),
                        })
                        .collect(),
                    dependencies: package.dependencies.clone(),
                }
            })
            .collect();

        Self {
            schema_version: CYCLONEDX_PROJECTION_SCHEMA_VERSION,
            record_kind: "aureline_cyclonedx_projection".to_owned(),
            bom_format: "CycloneDX".to_owned(),
            spec_version: "1.5".to_owned(),
            serial_ref: format!(
                "urn:aureline:cyclonedx:cargo-lock:{}",
                lock.lockfile_fingerprint.replace(':', "-")
            ),
            cargo_lock_package_count: lock.package_count,
            cargo_lock_fingerprint: lock.lockfile_fingerprint.clone(),
            components,
        }
    }
}

/// One CycloneDX component row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CycloneDxComponent {
    /// Component reference.
    pub bom_ref: String,
    /// CycloneDX component type.
    pub component_type: String,
    /// Component name.
    pub name: String,
    /// Component version.
    pub version: String,
    /// Package URL for the Cargo component.
    pub package_url: String,
    /// Component scope.
    pub scope: String,
    /// License expression or `NOASSERTION`.
    pub license_expression: String,
    /// Source refs that justify the license expression.
    pub license_source_refs: Vec<String>,
    /// Hashes supplied by `Cargo.lock`.
    pub hashes: Vec<CycloneDxHash>,
    /// External references supplied by `Cargo.lock`.
    pub external_references: Vec<CycloneDxExternalReference>,
    /// Raw dependency entries from `Cargo.lock`.
    pub dependencies: Vec<String>,
}

/// Hash value attached to a CycloneDX component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CycloneDxHash {
    /// Hash algorithm name.
    pub algorithm: String,
    /// Hash value.
    pub value: String,
}

/// External reference attached to a CycloneDX component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CycloneDxExternalReference {
    /// Reference type.
    pub reference_type: String,
    /// Reference URL or opaque source string.
    pub url: String,
}

/// Notice digest grouped by license family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeDigest {
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Number of packages read from `Cargo.lock`.
    pub cargo_lock_package_count: usize,
    /// Lockfile fingerprint the digest was generated from.
    pub cargo_lock_fingerprint: String,
    /// Workspace license expression.
    pub workspace_license_expression: String,
    /// License-family groups covering every lockfile package.
    pub license_groups: Vec<NoticeLicenseGroup>,
    /// Import-manifest rows that affect notices or SPDX/REUSE state.
    pub import_rows: Vec<NoticeImportRow>,
    /// Deterministic digest over the notice groups and import rows.
    pub digest_fingerprint: String,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl NoticeDigest {
    fn from_inputs(
        lock: &CargoLockSnapshot,
        workspace: &WorkspaceSummary,
        governance: &GovernanceInputs,
    ) -> Self {
        let context = PackageContext::new(workspace, governance);
        let mut groups: BTreeMap<String, NoticeLicenseGroupBuilder> = BTreeMap::new();

        for package in &lock.packages {
            let attribution = context.attribution_for(package);
            let key = attribution.license_family_id.clone();
            groups
                .entry(key.clone())
                .or_insert_with(|| NoticeLicenseGroupBuilder::new(key, &attribution))
                .push(package, &attribution);
        }

        let license_groups: Vec<NoticeLicenseGroup> = groups
            .into_values()
            .map(NoticeLicenseGroupBuilder::finish)
            .collect();
        let import_rows = governance
            .third_party_import_manifest
            .rows
            .iter()
            .map(NoticeImportRow::from)
            .collect::<Vec<_>>();
        let digest_fingerprint = notice_digest_fingerprint(&license_groups, &import_rows);
        let summary = format!(
            "{} Cargo.lock package(s) grouped into {} license family bucket(s); {} import-manifest row(s) carry notice or SPDX/REUSE state.",
            lock.package_count,
            license_groups.len(),
            import_rows.len()
        );

        Self {
            schema_version: NOTICE_DIGEST_SCHEMA_VERSION,
            record_kind: "aureline_notice_digest".to_owned(),
            cargo_lock_package_count: lock.package_count,
            cargo_lock_fingerprint: lock.lockfile_fingerprint.clone(),
            workspace_license_expression: workspace.workspace_license_expression.clone(),
            license_groups,
            import_rows,
            digest_fingerprint,
            summary,
        }
    }

    /// Returns true when all lockfile packages are represented by the digest.
    pub fn covers_lockfile(&self) -> bool {
        let grouped_count: usize = self
            .license_groups
            .iter()
            .map(|group| group.package_count)
            .sum();
        grouped_count == self.cargo_lock_package_count
    }
}

/// One notice digest group for a license family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeLicenseGroup {
    /// Stable group id.
    pub license_family_id: String,
    /// License expression used by SPDX and CycloneDX rows.
    pub license_expression: String,
    /// Review state for this license group.
    pub notice_state: String,
    /// Package count in the group.
    pub package_count: usize,
    /// Cargo lockfile package refs in the group.
    pub package_refs: Vec<String>,
    /// Source refs backing the group.
    pub source_refs: Vec<String>,
    /// Reviewer-facing note.
    pub notes: String,
}

struct NoticeLicenseGroupBuilder {
    license_family_id: String,
    license_expression: String,
    notice_state: String,
    package_refs: Vec<String>,
    source_refs: BTreeSet<String>,
    notes: String,
}

impl NoticeLicenseGroupBuilder {
    fn new(license_family_id: String, attribution: &LicenseAttribution) -> Self {
        Self {
            license_family_id,
            license_expression: attribution.spdx_license_expression.clone(),
            notice_state: attribution.notice_state.clone(),
            package_refs: Vec::new(),
            source_refs: attribution.source_refs.iter().cloned().collect(),
            notes: attribution.notes.clone(),
        }
    }

    fn push(&mut self, package: &CargoLockPackage, attribution: &LicenseAttribution) {
        self.package_refs.push(package.cargo_lock_package_ref());
        for source_ref in &attribution.source_refs {
            self.source_refs.insert(source_ref.clone());
        }
        if self.notice_state == "notice_complete" && attribution.notice_state != "notice_complete" {
            self.notice_state = attribution.notice_state.clone();
        }
    }

    fn finish(self) -> NoticeLicenseGroup {
        NoticeLicenseGroup {
            package_count: self.package_refs.len(),
            license_family_id: self.license_family_id,
            license_expression: self.license_expression,
            notice_state: self.notice_state,
            package_refs: self.package_refs,
            source_refs: self.source_refs.into_iter().collect(),
            notes: self.notes,
        }
    }
}

/// Notice-relevant row projected from the import manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoticeImportRow {
    /// Import manifest row id.
    pub row_id: String,
    /// Source class from the import manifest.
    pub source_class: String,
    /// Source id from dependency or import registers.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    /// License expression or license class recorded by the manifest.
    pub license_expression: String,
    /// SPDX/REUSE state recorded by the manifest.
    pub reuse_spdx_state: String,
    /// Whether a notice delta is required.
    pub notice_delta_required: bool,
    /// Notice source refs recorded by the manifest.
    pub notice_source_refs: Vec<String>,
}

impl From<&ThirdPartyImportManifestRow> for NoticeImportRow {
    fn from(row: &ThirdPartyImportManifestRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            source_class: row.source_class.clone(),
            source_id: row.source_id.clone(),
            license_expression: row.license_expression.clone(),
            reuse_spdx_state: row.reuse_spdx_state.clone(),
            notice_delta_required: row.notice_delta_required,
            notice_source_refs: row.notice_source_refs.clone(),
        }
    }
}

/// Critical-upstream health projection focused on red-risk rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalUpstreamHealthRegister {
    /// Projection schema version.
    pub schema_version: u32,
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Source register id read from the governance artifact.
    pub source_register_id: String,
    /// Source register status.
    pub status: String,
    /// Source register timestamp.
    pub as_of: String,
    /// Red-risk rows with normalized risk classes.
    pub red_risk_rows: Vec<CriticalUpstreamHealthRow>,
    /// Reviewer-facing summary.
    pub summary: String,
}

impl CriticalUpstreamHealthRegister {
    fn from_inputs(governance: &GovernanceInputs) -> Self {
        let dependency_by_id = governance
            .dependency_register
            .rows
            .iter()
            .map(|row| (row.id.as_str(), row))
            .collect::<BTreeMap<_, _>>();

        let red_risk_rows = governance
            .critical_upstream_health
            .rows
            .iter()
            .filter(|row| row.risk_state == "red")
            .map(|row| {
                let dependency = dependency_by_id.get(row.dependency_id.as_str()).copied();
                CriticalUpstreamHealthRow::from_source(row, dependency)
            })
            .collect::<Vec<_>>();

        let summary = format!(
            "{} red-risk upstream row(s) require owner, activity, or license review before stronger release claims.",
            red_risk_rows.len()
        );

        Self {
            schema_version: CRITICAL_UPSTREAM_SCHEMA_VERSION,
            record_kind: "aureline_critical_upstream_health_register".to_owned(),
            source_register_id: governance.critical_upstream_health.register_id.clone(),
            status: governance.critical_upstream_health.status.clone(),
            as_of: governance.critical_upstream_health.as_of.clone(),
            red_risk_rows,
            summary,
        }
    }

    /// Returns true when at least one row carries `risk_class`.
    pub fn contains_risk_class(&self, risk_class: RedRiskClass) -> bool {
        self.red_risk_rows
            .iter()
            .any(|row| row.red_risk_classes.contains(&risk_class))
    }
}

/// One red-risk critical-upstream row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CriticalUpstreamHealthRow {
    /// Dependency id from the governance register.
    pub dependency_id: String,
    /// Dependency display name.
    pub dependency_name: String,
    /// Risk state from the source register.
    pub risk_state: String,
    /// Health status from the source register.
    pub health_status: String,
    /// Owner named by the source register.
    pub owner_dri: String,
    /// Backup-owner state named by the source register.
    pub backup_owner_state: String,
    /// Normalized red-risk classes.
    pub red_risk_classes: Vec<RedRiskClass>,
    /// Next review due date.
    pub next_review_due: String,
    /// Fork, replace, or escalate trigger.
    pub fork_replace_escalate_trigger: String,
    /// Source refs backing the row.
    pub source_refs: Vec<String>,
}

impl CriticalUpstreamHealthRow {
    fn from_source(
        row: &CriticalHealthSourceRow,
        dependency: Option<&DependencyRegisterRow>,
    ) -> Self {
        let mut source_refs = BTreeSet::new();
        source_refs.insert(row.source_dependency_ref.clone());
        for source_ref in &row.critical_dependency_entry_refs {
            source_refs.insert(source_ref.clone());
        }
        if let Some(scorecard_ref) = &row.upstream_health_scorecard_ref {
            source_refs.insert(scorecard_ref.clone());
        }
        for evidence_ref in &row.evidence_refs {
            source_refs.insert(evidence_ref.clone());
        }

        Self {
            dependency_id: row.dependency_id.clone(),
            dependency_name: row.dependency_name.clone(),
            risk_state: row.risk_state.clone(),
            health_status: row.health_status.clone(),
            owner_dri: row.owner_dri.clone(),
            backup_owner_state: row.backup_owner_state.clone(),
            red_risk_classes: derive_red_risk_classes(row, dependency),
            next_review_due: row.next_review_due.clone(),
            fork_replace_escalate_trigger: row.fork_replace_escalate_trigger.clone(),
            source_refs: source_refs.into_iter().collect(),
        }
    }
}

/// Normalized red-risk class for critical upstreams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedRiskClass {
    /// Backup or maintainer depth is missing or single-person.
    SingleMaintainer,
    /// Upstream activity, review, or scorecard evidence is missing or stale.
    Unmaintained,
    /// License terms are incompatible, ambiguous, or unverified for the path.
    LicenseIncompatible,
}

impl RedRiskClass {
    /// Returns the stable snake-case token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleMaintainer => "single_maintainer",
            Self::Unmaintained => "unmaintained",
            Self::LicenseIncompatible => "license_incompatible",
        }
    }
}

struct PackageContext<'a> {
    workspace: &'a WorkspaceSummary,
    dependency_by_crate: BTreeMap<String, &'a DependencyRegisterRow>,
    release_notice_by_source_id: BTreeMap<String, &'a ReleaseNoticeSeedRow>,
}

impl<'a> PackageContext<'a> {
    fn new(workspace: &'a WorkspaceSummary, governance: &'a GovernanceInputs) -> Self {
        let mut dependency_by_crate = BTreeMap::new();
        for row in &governance.dependency_register.rows {
            if row.dependency_kind == "cargo_crate" {
                let crate_key = dependency_crate_key(row);
                dependency_by_crate.insert(crate_key, row);
            }
        }
        let release_notice_by_source_id = governance
            .release_notice_seed
            .rows
            .iter()
            .map(|row| (row.source_id.clone(), row))
            .collect();
        Self {
            workspace,
            dependency_by_crate,
            release_notice_by_source_id,
        }
    }

    fn attribution_for(&self, package: &CargoLockPackage) -> LicenseAttribution {
        if self.workspace.contains_package(&package.name) {
            return LicenseAttribution {
                license_family_id: WORKSPACE_LICENSE_EXPRESSION.to_owned(),
                spdx_license_expression: WORKSPACE_LICENSE_EXPRESSION.to_owned(),
                notice_state: "notice_complete".to_owned(),
                source_refs: vec!["Cargo.toml#workspace.package.license".to_owned()],
                notes: "First-party workspace package covered by the workspace license expression."
                    .to_owned(),
            };
        }

        if let Some(row) = self.dependency_by_crate.get(package.name.as_str()) {
            let mut source_refs = vec![format!(
                "artifacts/governance/dependency_register.yaml#{}",
                row.id
            )];
            if let Some(notice_row) = self.release_notice_by_source_id.get(row.id.as_str()) {
                source_refs.push(format!(
                    "artifacts/governance/release_notice_seed.yaml#{}",
                    notice_row.source_id
                ));
            }
            return LicenseAttribution {
                license_family_id: format!("governance.{}", row.license_class),
                spdx_license_expression: SPDX_NOASSERTION.to_owned(),
                notice_state: "review_required".to_owned(),
                source_refs,
                notes: "Governance row names the license class, but the Cargo lockfile does not carry reviewed SPDX license metadata."
                    .to_owned(),
            };
        }

        LicenseAttribution {
            license_family_id: "unknown.lockfile_no_license_metadata".to_owned(),
            spdx_license_expression: SPDX_NOASSERTION.to_owned(),
            notice_state: "review_required".to_owned(),
            source_refs: vec!["Cargo.lock".to_owned()],
            notes: "Cargo.lock records the package identity but not license metadata; review before release notice publication."
                .to_owned(),
        }
    }
}

struct LicenseAttribution {
    license_family_id: String,
    spdx_license_expression: String,
    notice_state: String,
    source_refs: Vec<String>,
    notes: String,
}

struct GovernanceInputs {
    dependency_register: DependencyRegister,
    release_notice_seed: ReleaseNoticeSeed,
    third_party_import_manifest: ThirdPartyImportManifest,
    critical_upstream_health: CriticalUpstreamHealthSource,
}

impl GovernanceInputs {
    fn read(repo_root: &Path) -> Result<Self, NoticeError> {
        Ok(Self {
            dependency_register: read_yaml(
                &repo_root.join("artifacts/governance/dependency_register.yaml"),
            )?,
            release_notice_seed: read_yaml(
                &repo_root.join("artifacts/governance/release_notice_seed.yaml"),
            )?,
            third_party_import_manifest: read_yaml(
                &repo_root.join("artifacts/governance/third_party_import_manifest.yaml"),
            )?,
            critical_upstream_health: read_yaml(
                &repo_root.join("artifacts/governance/critical_upstream_health_register.yaml"),
            )?,
        })
    }
}

#[derive(Debug, Deserialize)]
struct DependencyRegister {
    #[serde(default)]
    rows: Vec<DependencyRegisterRow>,
}

#[derive(Debug, Deserialize)]
struct DependencyRegisterRow {
    id: String,
    name: String,
    dependency_kind: String,
    license_class: String,
    #[serde(default)]
    protected_path: bool,
}

#[derive(Debug, Deserialize)]
struct ReleaseNoticeSeed {
    #[serde(default)]
    rows: Vec<ReleaseNoticeSeedRow>,
}

#[derive(Debug, Deserialize)]
struct ReleaseNoticeSeedRow {
    source_id: String,
}

#[derive(Debug, Deserialize)]
struct ThirdPartyImportManifest {
    #[serde(default)]
    rows: Vec<ThirdPartyImportManifestRow>,
}

#[derive(Debug, Deserialize)]
struct ThirdPartyImportManifestRow {
    row_id: String,
    source_class: String,
    #[serde(default)]
    source_id: Option<String>,
    license_expression: String,
    reuse_spdx_state: String,
    #[serde(default)]
    notice_delta_required: bool,
    #[serde(default)]
    notice_source_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct CriticalUpstreamHealthSource {
    #[serde(default)]
    register_id: String,
    #[serde(default)]
    as_of: String,
    #[serde(default)]
    status: String,
    #[serde(default)]
    rows: Vec<CriticalHealthSourceRow>,
}

#[derive(Debug, Deserialize)]
struct CriticalHealthSourceRow {
    dependency_id: String,
    dependency_name: String,
    source_dependency_ref: String,
    #[serde(default)]
    critical_dependency_entry_refs: Vec<String>,
    owner_dri: String,
    backup_owner_state: String,
    risk_state: String,
    health_status: String,
    #[serde(default)]
    upstream_health_scorecard_ref: Option<String>,
    fork_replace_escalate_trigger: String,
    next_review_due: String,
    #[serde(default)]
    evidence_refs: Vec<String>,
}

fn read_workspace_summary(repo_root: &Path) -> Result<WorkspaceSummary, NoticeError> {
    let workspace_manifest_path = repo_root.join("Cargo.toml");
    let workspace_manifest = read_to_string(&workspace_manifest_path)?;
    let workspace_license =
        parse_workspace_license(&workspace_manifest).ok_or_else(|| NoticeError::Manifest {
            path: workspace_manifest_path.clone(),
            message: "workspace.package.license is required".to_owned(),
        })?;
    if workspace_license != WORKSPACE_LICENSE_EXPRESSION {
        return Err(NoticeError::Manifest {
            path: workspace_manifest_path,
            message: format!(
                "workspace license must be {WORKSPACE_LICENSE_EXPRESSION}, got {workspace_license}"
            ),
        });
    }

    let mut members = Vec::new();
    for member_path in parse_workspace_members(&workspace_manifest) {
        let manifest_path = repo_root.join(&member_path).join("Cargo.toml");
        let manifest = read_to_string(&manifest_path)?;
        let package_name = parse_package_name(&manifest).ok_or_else(|| NoticeError::Manifest {
            path: manifest_path.clone(),
            message: "package.name is required".to_owned(),
        })?;
        let license_expression =
            parse_package_license(&manifest, &workspace_license).ok_or_else(|| {
                NoticeError::Manifest {
                    path: manifest_path.clone(),
                    message: "package license must be explicit or workspace-inherited".to_owned(),
                }
            })?;
        members.push(WorkspaceCrate {
            package_name,
            manifest_path: format!("{member_path}/Cargo.toml"),
            license_expression,
        });
    }
    members.sort_by(|left, right| left.package_name.cmp(&right.package_name));

    Ok(WorkspaceSummary {
        workspace_license_expression: workspace_license,
        members,
    })
}

fn read_to_string(path: &Path) -> Result<String, NoticeError> {
    fs::read_to_string(path).map_err(|source| NoticeError::Io {
        path: path.to_path_buf(),
        source,
    })
}

fn read_yaml<T>(path: &Path) -> Result<T, NoticeError>
where
    T: for<'de> Deserialize<'de>,
{
    let payload = read_to_string(path)?;
    serde_yaml::from_str(&payload).map_err(|source| NoticeError::Yaml {
        path: path.to_path_buf(),
        source,
    })
}

fn parse_workspace_members(manifest: &str) -> Vec<String> {
    let mut members = Vec::new();
    let mut in_workspace = false;
    let mut in_members = false;

    for raw_line in manifest.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.starts_with('[') {
            in_workspace = line == "[workspace]";
            in_members = false;
            continue;
        }
        if !in_workspace {
            continue;
        }
        if line.starts_with("members") && line.contains('[') {
            in_members = true;
        }
        if in_members {
            members.extend(all_quoted_values(line));
            if line.contains(']') {
                in_members = false;
            }
        }
    }

    members
}

fn parse_workspace_license(manifest: &str) -> Option<String> {
    let mut in_workspace_package = false;
    for raw_line in manifest.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.starts_with('[') {
            in_workspace_package = line == "[workspace.package]";
            continue;
        }
        if in_workspace_package {
            if let Some(value) = key_value(line, "license") {
                return Some(value);
            }
        }
    }
    None
}

fn parse_package_name(manifest: &str) -> Option<String> {
    let mut in_package = false;
    for raw_line in manifest.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if in_package {
            if let Some(value) = key_value(line, "name") {
                return Some(value);
            }
        }
    }
    None
}

fn parse_package_license(manifest: &str, workspace_license: &str) -> Option<String> {
    let mut in_package = false;
    for raw_line in manifest.lines() {
        let line = strip_toml_comment(raw_line).trim();
        if line.starts_with('[') {
            in_package = line == "[package]";
            continue;
        }
        if !in_package {
            continue;
        }
        if let Some(value) = key_value(line, "license") {
            return Some(value);
        }
        if line == "license.workspace = true" {
            return Some(workspace_license.to_owned());
        }
    }
    None
}

fn strip_toml_comment(line: &str) -> &str {
    line.split_once('#')
        .map(|(before_comment, _)| before_comment)
        .unwrap_or(line)
}

fn key_value(line: &str, key: &str) -> Option<String> {
    let (left, right) = line.split_once('=')?;
    if left.trim() != key {
        return None;
    }
    quoted_value(right.trim())
}

fn quoted_value(line: &str) -> Option<String> {
    let start = line.find('"')?;
    let rest = &line[start + 1..];
    let end = rest.find('"')?;
    Some(rest[..end].to_owned())
}

fn all_quoted_values(line: &str) -> Vec<String> {
    let mut values = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find('"') {
        let after_start = &rest[start + 1..];
        let Some(end) = after_start.find('"') else {
            break;
        };
        values.push(after_start[..end].to_owned());
        rest = &after_start[end + 1..];
    }
    values
}

fn dependency_crate_key(row: &DependencyRegisterRow) -> String {
    if !row.name.contains("-class") && !row.name.contains(' ') {
        return row.name.clone();
    }
    row.id
        .rsplit('.')
        .next()
        .unwrap_or(row.id.as_str())
        .to_owned()
}

fn derive_red_risk_classes(
    row: &CriticalHealthSourceRow,
    dependency: Option<&DependencyRegisterRow>,
) -> Vec<RedRiskClass> {
    let mut classes = BTreeSet::new();
    let backup = row.backup_owner_state.as_str();
    if backup.contains("not_yet")
        || backup.contains("missing")
        || backup.contains("single")
        || backup.contains("none")
    {
        classes.insert(RedRiskClass::SingleMaintainer);
    }

    if row.health_status.contains("missing")
        || row.health_status.contains("provisional")
        || row.upstream_health_scorecard_ref.is_none()
    {
        classes.insert(RedRiskClass::Unmaintained);
    }

    let license_needs_review = dependency
        .map(|dep| {
            dep.license_class.contains("pending")
                || dep.license_class.contains("verify")
                || (dep.protected_path && !dep.license_class.contains("permissive_oss"))
        })
        .unwrap_or(false);
    if license_needs_review {
        classes.insert(RedRiskClass::LicenseIncompatible);
    }

    if classes.is_empty() && row.risk_state == "red" {
        classes.insert(RedRiskClass::Unmaintained);
    }

    classes.into_iter().collect()
}

fn sanitized_spdx_id(package: &CargoLockPackage, index: usize) -> String {
    let mut id = String::new();
    for ch in format!("{}-{}-{index}", package.name, package.version).chars() {
        if ch.is_ascii_alphanumeric() || ch == '-' || ch == '.' {
            id.push(ch);
        } else {
            id.push('-');
        }
    }
    id
}

fn lockfile_fingerprint(packages: &[CargoLockPackage]) -> String {
    let mut material = String::new();
    for package in packages {
        material.push_str("[[package]]\n");
        material.push_str(&package.name);
        material.push('\n');
        material.push_str(&package.version);
        material.push('\n');
        material.push_str(package.source.as_deref().unwrap_or(""));
        material.push('\n');
        material.push_str(package.checksum.as_deref().unwrap_or(""));
        material.push('\n');
        for dependency in &package.dependencies {
            material.push_str(dependency);
            material.push('\n');
        }
    }
    format!("lock-fnv64:{:016x}", fnv1a64(material.as_bytes()))
}

fn notice_digest_fingerprint(groups: &[NoticeLicenseGroup], imports: &[NoticeImportRow]) -> String {
    let mut material = String::new();
    for group in groups {
        material.push_str(&group.license_family_id);
        material.push('\n');
        material.push_str(&group.license_expression);
        material.push('\n');
        material.push_str(&group.notice_state);
        material.push('\n');
        for package_ref in &group.package_refs {
            material.push_str(package_ref);
            material.push('\n');
        }
    }
    for row in imports {
        material.push_str(&row.row_id);
        material.push('\n');
        material.push_str(row.source_id.as_deref().unwrap_or(""));
        material.push('\n');
        material.push_str(&row.license_expression);
        material.push('\n');
        material.push_str(&row.reuse_spdx_state);
        material.push('\n');
    }
    format!("notice-fnv64:{:016x}", fnv1a64(material.as_bytes()))
}

fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0001_0000_01b3);
    }
    hash
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cargo_lock_parser_reads_package_dependencies() {
        let lock = CargoLockSnapshot::parse(
            r#"
version = 3

[[package]]
name = "a"
version = "1.0.0"
source = "registry+https://example.invalid"
checksum = "abc"
dependencies = [
 "b",
]

[[package]]
name = "b"
version = "0.1.0"
"#,
        )
        .expect("lockfile parses");

        assert_eq!(lock.package_count, 2);
        assert_eq!(lock.packages[0].dependencies, vec!["b"]);
        assert!(lock.packages[1].is_workspace_package());
    }
}
