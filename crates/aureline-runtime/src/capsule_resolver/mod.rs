//! Metadata-only environment-capsule resolver for alpha workspace entry.
//!
//! This module sits upstream of [`crate::execution_context::ExecutionContextResolver`].
//! It inspects read-only workspace metadata, chooses an alpha capsule hint, and
//! returns the [`EnvironmentCapsuleRef`] plus a prebuild fingerprint stub needed
//! by launch, template, and support surfaces. It intentionally does not parse
//! devcontainer, Nix, or Compose bodies and never executes repository-owned
//! hooks.

use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::detectors::node::{
    NodeToolchainDetection, NodeToolchainDetector, NodeToolchainResolutionState,
};
use crate::detectors::python::{
    PythonEnvironmentDetection, PythonEnvironmentDetector, PythonEnvironmentResolutionState,
};
use crate::execution_context::{CapsuleDriftState, EnvironmentCapsuleRef, PrebuildReuseState};

/// Stable record-kind tag emitted by [`EnvironmentCapsuleResolution`].
pub const ENVIRONMENT_CAPSULE_RESOLUTION_RECORD_KIND: &str =
    "environment_capsule_resolution_alpha_record";

/// Schema version for [`EnvironmentCapsuleResolution`] payloads.
pub const ENVIRONMENT_CAPSULE_RESOLUTION_SCHEMA_VERSION: u32 = 1;

/// Resolver implementation version recorded on every resolution.
pub const ENVIRONMENT_CAPSULE_RESOLVER_VERSION: &str = "environment_capsule_resolver.alpha.v1";

/// Stable record-kind tag emitted by [`PrebuildFingerprintStub`].
pub const PREBUILD_FINGERPRINT_STUB_RECORD_KIND: &str = "prebuild_fingerprint_stub_alpha_record";

/// Template archetype hint supplied by callers that already know project intent.
///
/// The values mirror the template/scaffold archetype vocabulary so the resolver
/// can use caller intent without minting a parallel classification system.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectArchetypeHint {
    /// A browser or full-stack web application.
    WebApplication,
    /// A frontend package or component library.
    WebFrontendLibrary,
    /// A backend API, worker, or service.
    BackendService,
    /// A command-line tool.
    CliTool,
    /// A reusable library or SDK.
    LibraryOrSdk,
    /// A data science, analytics, or machine-learning workspace.
    DataOrMlWorkbench,
    /// A mobile application.
    MobileApplication,
    /// An embedded or firmware project.
    EmbeddedOrFirmware,
    /// A repository root that coordinates multiple workspace members.
    MonorepoRootWithWorkspaces,
    /// A member workspace inside a larger monorepo.
    MonorepoMemberWorkspace,
    /// A documentation site.
    DocumentationSite,
    /// Infrastructure, deployment, or pipeline code.
    InfrastructureOrPipeline,
    /// An extension or plugin project.
    ExtensionOrPlugin,
    /// No reliable archetype hint is available.
    #[default]
    ArchetypeClassUnknownRequiresReview,
}

impl ProjectArchetypeHint {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WebApplication => "web_application",
            Self::WebFrontendLibrary => "web_frontend_library",
            Self::BackendService => "backend_service",
            Self::CliTool => "cli_tool",
            Self::LibraryOrSdk => "library_or_sdk",
            Self::DataOrMlWorkbench => "data_or_ml_workbench",
            Self::MobileApplication => "mobile_application",
            Self::EmbeddedOrFirmware => "embedded_or_firmware",
            Self::MonorepoRootWithWorkspaces => "monorepo_root_with_workspaces",
            Self::MonorepoMemberWorkspace => "monorepo_member_workspace",
            Self::DocumentationSite => "documentation_site",
            Self::InfrastructureOrPipeline => "infrastructure_or_pipeline",
            Self::ExtensionOrPlugin => "extension_or_plugin",
            Self::ArchetypeClassUnknownRequiresReview => "archetype_class_unknown_requires_review",
        }
    }

    const fn prefers_node(self) -> bool {
        matches!(
            self,
            Self::WebApplication
                | Self::WebFrontendLibrary
                | Self::DocumentationSite
                | Self::ExtensionOrPlugin
        )
    }

    const fn prefers_python(self) -> bool {
        matches!(self, Self::DataOrMlWorkbench)
    }
}

/// Alpha capsule family selected from workspace metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentCapsuleHint {
    /// Node or TypeScript/JavaScript workspace metadata was detected.
    Node,
    /// Python interpreter or environment-manager metadata was detected.
    Python,
    /// Multiple launch-wedge ecosystems were detected.
    Polyglot,
    /// No alpha resolver signal was present.
    NoSignal,
}

impl EnvironmentCapsuleHint {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Python => "python",
            Self::Polyglot => "polyglot",
            Self::NoSignal => "no_signal",
        }
    }
}

/// Caller-provided configuration for [`EnvironmentCapsuleResolver`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentCapsuleResolverConfig {
    /// Timestamp token passed through to detector reports.
    pub detected_at: String,
    /// Capsule schema version recorded on the returned reference.
    pub resolved_schema_version: String,
    /// Policy epoch included in the prebuild fingerprint stub.
    pub policy_epoch: u64,
    /// Platform/architecture token included in the prebuild fingerprint stub.
    pub platform_arch: String,
    /// Resolver implementation token recorded on the result.
    pub resolver_version: String,
}

impl Default for EnvironmentCapsuleResolverConfig {
    fn default() -> Self {
        Self {
            detected_at: "metadata-only".to_owned(),
            resolved_schema_version: "environment_capsule_alpha.v1".to_owned(),
            policy_epoch: 0,
            platform_arch: "platform_pending".to_owned(),
            resolver_version: ENVIRONMENT_CAPSULE_RESOLVER_VERSION.to_owned(),
        }
    }
}

/// Metadata-only prebuild compatibility fingerprint produced by the resolver.
///
/// The alpha resolver does not materialize a prebuild. This record provides
/// stable refs and hash-shaped tokens so reuse decisions can be reviewed and
/// invalidated without claiming that a warm snapshot exists.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PrebuildFingerprintStub {
    /// Stable record kind.
    pub record_kind: String,
    /// Opaque fingerprint reference.
    pub fingerprint_ref: String,
    /// Environment capsule id this fingerprint cites.
    pub capsule_ref: String,
    /// Environment capsule hash this fingerprint cites.
    pub capsule_hash: String,
    /// Digest-shaped token for the metadata sources that influenced the result.
    pub source_digest_set_hash: String,
    /// Platform/architecture class included in compatibility checks.
    pub platform_arch: String,
    /// Policy epoch included in compatibility checks.
    pub policy_epoch: u64,
    /// Digest-shaped tokens for critical launch-wedge toolchain facts.
    pub critical_toolchain_hashes: Vec<String>,
    /// Metadata-only prebuild reuse state.
    pub reuse_state: PrebuildReuseState,
}

/// Result returned by [`EnvironmentCapsuleResolver::resolve_workspace`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EnvironmentCapsuleResolution {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Resolver implementation token.
    pub resolver_version: String,
    /// Workspace root inspected by this report.
    pub workspace_root_ref: String,
    /// Caller-supplied project archetype hint.
    pub archetype_hint: ProjectArchetypeHint,
    /// Capsule family selected from workspace metadata.
    pub capsule_hint: EnvironmentCapsuleHint,
    /// Timestamp token supplied to detector reports.
    pub detected_at: String,
    /// Environment capsule reference to pass into execution-context resolution.
    pub environment_capsule_ref: EnvironmentCapsuleRef,
    /// Drift state repeated for callers that only need the state label.
    pub capsule_drift_state: CapsuleDriftState,
    /// Metadata-only prebuild fingerprint stub.
    pub prebuild_fingerprint_stub: PrebuildFingerprintStub,
    /// Repository-relative metadata refs that influenced the result.
    pub source_refs: Vec<String>,
    /// Node detector report when Node metadata selected or contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub node_toolchain_detection: Option<NodeToolchainDetection>,
    /// Python detector report when Python metadata selected or contributed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub python_environment_detection: Option<PythonEnvironmentDetection>,
}

/// Read-only resolver that mints alpha environment-capsule references.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentCapsuleResolver {
    config: EnvironmentCapsuleResolverConfig,
}

impl EnvironmentCapsuleResolver {
    /// Creates a resolver with caller-provided metadata defaults.
    pub fn new(config: EnvironmentCapsuleResolverConfig) -> Self {
        Self { config }
    }

    /// Creates a resolver with default alpha metadata settings.
    pub fn default_read_only() -> Self {
        Self::new(EnvironmentCapsuleResolverConfig::default())
    }

    /// Resolves a workspace path and project archetype hint into a capsule ref.
    ///
    /// The resolver reads launch-wedge metadata through existing Node and
    /// Python detectors, then mints an alpha capsule reference. No lifecycle
    /// hooks run and no devcontainer, Nix, or Compose file bodies are parsed.
    pub fn resolve_workspace(
        &self,
        workspace_root: &Path,
        archetype_hint: ProjectArchetypeHint,
    ) -> EnvironmentCapsuleResolution {
        let node_detection = NodeToolchainDetector::default_read_only()
            .detect_workspace(workspace_root, &self.config.detected_at);
        let python_detection = PythonEnvironmentDetector::default_read_only()
            .detect_workspace(workspace_root, &self.config.detected_at);

        let node_present = node_signal_present(workspace_root, &node_detection);
        let python_present = python_signal_present(workspace_root, &python_detection);
        let capsule_hint = choose_capsule_hint(archetype_hint, node_present, python_present);
        let capsule_drift_state = drift_state_for_hint(capsule_hint);
        let mut source_refs = source_refs_for(capsule_hint, &node_detection, &python_detection);
        source_refs.sort();
        source_refs.dedup();

        let capsule_id = capsule_id_for(archetype_hint, capsule_hint).to_owned();
        let source_ref_views = source_refs.iter().map(String::as_str).collect::<Vec<_>>();
        let capsule_hash = digest_token(&[
            "capsule",
            capsule_id.as_str(),
            archetype_hint.as_str(),
            capsule_hint.as_str(),
            &source_ref_views.join("|"),
        ]);
        let environment_capsule_ref = EnvironmentCapsuleRef {
            capsule_id: capsule_id.clone(),
            capsule_hash: capsule_hash.clone(),
            resolved_schema_version: self.config.resolved_schema_version.clone(),
            drift_state: capsule_drift_state,
        };
        let critical_toolchain_hashes =
            critical_toolchain_hashes(capsule_hint, &node_detection, &python_detection);
        let source_digest_set_hash = digest_token(&[
            "source-set",
            capsule_id.as_str(),
            &source_ref_views.join("|"),
        ]);
        let fingerprint_ref = format!(
            "fingerprint:{}:{}",
            capsule_id,
            short_digest_suffix(&source_digest_set_hash)
        );
        let prebuild_fingerprint_stub = PrebuildFingerprintStub {
            record_kind: PREBUILD_FINGERPRINT_STUB_RECORD_KIND.to_owned(),
            fingerprint_ref,
            capsule_ref: capsule_id,
            capsule_hash,
            source_digest_set_hash,
            platform_arch: self.config.platform_arch.clone(),
            policy_epoch: self.config.policy_epoch,
            critical_toolchain_hashes,
            reuse_state: if capsule_hint == EnvironmentCapsuleHint::NoSignal {
                PrebuildReuseState::NotApplicable
            } else {
                PrebuildReuseState::Candidate
            },
        };

        EnvironmentCapsuleResolution {
            record_kind: ENVIRONMENT_CAPSULE_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: ENVIRONMENT_CAPSULE_RESOLUTION_SCHEMA_VERSION,
            resolver_version: self.config.resolver_version.clone(),
            workspace_root_ref: workspace_root.display().to_string(),
            archetype_hint,
            capsule_hint,
            detected_at: self.config.detected_at.clone(),
            environment_capsule_ref,
            capsule_drift_state,
            prebuild_fingerprint_stub,
            source_refs,
            node_toolchain_detection: node_present.then_some(node_detection),
            python_environment_detection: python_present.then_some(python_detection),
        }
    }
}

impl Default for EnvironmentCapsuleResolver {
    fn default() -> Self {
        Self::default_read_only()
    }
}

fn choose_capsule_hint(
    archetype_hint: ProjectArchetypeHint,
    node_present: bool,
    python_present: bool,
) -> EnvironmentCapsuleHint {
    match (node_present, python_present) {
        (true, true) if archetype_hint.prefers_node() => EnvironmentCapsuleHint::Node,
        (true, true) if archetype_hint.prefers_python() => EnvironmentCapsuleHint::Python,
        (true, true) => EnvironmentCapsuleHint::Polyglot,
        (true, false) => EnvironmentCapsuleHint::Node,
        (false, true) => EnvironmentCapsuleHint::Python,
        (false, false) => EnvironmentCapsuleHint::NoSignal,
    }
}

const fn drift_state_for_hint(hint: EnvironmentCapsuleHint) -> CapsuleDriftState {
    match hint {
        EnvironmentCapsuleHint::NoSignal => CapsuleDriftState::UnknownLineage,
        EnvironmentCapsuleHint::Node
        | EnvironmentCapsuleHint::Python
        | EnvironmentCapsuleHint::Polyglot => CapsuleDriftState::InSync,
    }
}

const fn capsule_id_for(
    archetype_hint: ProjectArchetypeHint,
    capsule_hint: EnvironmentCapsuleHint,
) -> &'static str {
    match (capsule_hint, archetype_hint) {
        (EnvironmentCapsuleHint::Node, ProjectArchetypeHint::WebApplication)
        | (EnvironmentCapsuleHint::Node, ProjectArchetypeHint::WebFrontendLibrary)
        | (EnvironmentCapsuleHint::Node, ProjectArchetypeHint::DocumentationSite) => {
            "capsule.alpha.ts_web.local_node"
        }
        (EnvironmentCapsuleHint::Node, _) => "capsule.alpha.node.local_metadata",
        (EnvironmentCapsuleHint::Python, _) => "capsule.alpha.python.local_python",
        (EnvironmentCapsuleHint::Polyglot, _) => "capsule.alpha.polyglot.local_metadata",
        (EnvironmentCapsuleHint::NoSignal, _) => "capsule.alpha.unknown.uncertain",
    }
}

fn node_signal_present(workspace_root: &Path, detection: &NodeToolchainDetection) -> bool {
    workspace_root.join("package.json").is_file()
        || workspace_root.join("pnpm-lock.yaml").is_file()
        || workspace_root.join("yarn.lock").is_file()
        || workspace_root.join("package-lock.json").is_file()
        || workspace_root.join("npm-shrinkwrap.json").is_file()
        || detection.node_runtime.resolution_state != NodeToolchainResolutionState::Missing
        || detection.package_manager.resolution_state != NodeToolchainResolutionState::Missing
}

fn python_signal_present(workspace_root: &Path, detection: &PythonEnvironmentDetection) -> bool {
    workspace_root.join("pyproject.toml").is_file()
        || workspace_root.join(".python-version").is_file()
        || workspace_root.join(".venv").is_dir()
        || workspace_root.join("uv.lock").is_file()
        || workspace_root.join("poetry.lock").is_file()
        || detection.interpreter.resolution_state != PythonEnvironmentResolutionState::Missing
        || detection.environment_manager.resolution_state
            != PythonEnvironmentResolutionState::Missing
}

fn source_refs_for(
    capsule_hint: EnvironmentCapsuleHint,
    node_detection: &NodeToolchainDetection,
    python_detection: &PythonEnvironmentDetection,
) -> Vec<String> {
    let mut refs = BTreeSet::new();
    if matches!(
        capsule_hint,
        EnvironmentCapsuleHint::Node | EnvironmentCapsuleHint::Polyglot
    ) {
        refs.extend(
            node_detection
                .provenance_cards
                .iter()
                .filter_map(|card| card.source_ref.clone()),
        );
    }
    if matches!(
        capsule_hint,
        EnvironmentCapsuleHint::Python | EnvironmentCapsuleHint::Polyglot
    ) {
        refs.extend(
            python_detection
                .provenance_cards
                .iter()
                .filter_map(|card| card.source_ref.clone()),
        );
    }
    refs.into_iter().collect()
}

fn critical_toolchain_hashes(
    capsule_hint: EnvironmentCapsuleHint,
    node_detection: &NodeToolchainDetection,
    python_detection: &PythonEnvironmentDetection,
) -> Vec<String> {
    let mut hashes = Vec::new();
    if matches!(
        capsule_hint,
        EnvironmentCapsuleHint::Node | EnvironmentCapsuleHint::Polyglot
    ) {
        let node_requirement = node_detection
            .node_runtime
            .resolved_requirement
            .as_deref()
            .unwrap_or("node.unresolved");
        let package_manager = match node_detection.package_manager.kind {
            Some(kind) => match node_detection.package_manager.version.as_deref() {
                Some(version) => format!("{}@{}", kind.as_str(), version),
                None => kind.as_str().to_owned(),
            },
            None => "package_manager.unresolved".to_owned(),
        };
        hashes.push(digest_token(&["toolchain", "node", node_requirement]));
        hashes.push(digest_token(&[
            "toolchain",
            "node.package_manager",
            package_manager.as_str(),
        ]));
    }
    if matches!(
        capsule_hint,
        EnvironmentCapsuleHint::Python | EnvironmentCapsuleHint::Polyglot
    ) {
        let python_requirement = python_detection
            .interpreter
            .resolved_requirement
            .as_deref()
            .unwrap_or("python.unresolved");
        let manager = match python_detection.environment_manager.kind {
            Some(kind) => match python_detection.environment_manager.version.as_deref() {
                Some(version) => format!("{}@{}", kind.as_str(), version),
                None => kind.as_str().to_owned(),
            },
            None => "environment_manager.unresolved".to_owned(),
        };
        hashes.push(digest_token(&["toolchain", "python", python_requirement]));
        hashes.push(digest_token(&[
            "toolchain",
            "python.environment_manager",
            manager.as_str(),
        ]));
    }
    if hashes.is_empty() {
        hashes.push(digest_token(&["toolchain", "none"]));
    }
    hashes
}

fn digest_token(parts: &[&str]) -> String {
    let mut lanes = [
        0xcbf29ce484222325_u64,
        0x9e3779b97f4a7c15_u64,
        0x517cc1b727220a95_u64,
        0x94d049bb133111eb_u64,
    ];
    for (part_index, part) in parts.iter().enumerate() {
        for byte in part.as_bytes() {
            for (lane_index, lane) in lanes.iter_mut().enumerate() {
                *lane ^= u64::from(*byte)
                    .wrapping_add((part_index as u64) << 8)
                    .wrapping_add(lane_index as u64);
                *lane = lane.wrapping_mul(0x100000001b3);
                *lane ^= *lane >> 32;
            }
        }
        for (lane_index, lane) in lanes.iter_mut().enumerate() {
            *lane ^= 0xff_u64.wrapping_add(lane_index as u64);
            *lane = lane.wrapping_mul(0x100000001b3);
        }
    }
    format!(
        "sha256:{:016x}{:016x}{:016x}{:016x}",
        lanes[0], lanes[1], lanes[2], lanes[3]
    )
}

fn short_digest_suffix(digest: &str) -> &str {
    digest
        .strip_prefix("sha256:")
        .and_then(|suffix| suffix.get(0..12))
        .unwrap_or("metadata")
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn typescript_workspace_resolves_node_capsule_hint() {
        let root = temp_workspace("node");
        fs::write(
            root.join("package.json"),
            r#"{"scripts":{"test":"vitest"},"engines":{"node":">=20 <23"},"packageManager":"pnpm@9.1.0"}"#,
        )
        .expect("write package.json");

        let resolution = EnvironmentCapsuleResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::WebApplication);

        assert_eq!(resolution.capsule_hint, EnvironmentCapsuleHint::Node);
        assert_eq!(resolution.capsule_drift_state, CapsuleDriftState::InSync);
        assert_eq!(
            resolution.environment_capsule_ref.capsule_id,
            "capsule.alpha.ts_web.local_node"
        );
        assert_eq!(
            resolution.prebuild_fingerprint_stub.reuse_state,
            PrebuildReuseState::Candidate
        );
        assert!(resolution
            .source_refs
            .iter()
            .any(|item| item == "package.json#engines.node"));
        assert!(resolution.node_toolchain_detection.is_some());
        assert!(resolution.python_environment_detection.is_none());
        assert_hash_shape(&resolution.environment_capsule_ref.capsule_hash);
        assert_hash_shape(&resolution.prebuild_fingerprint_stub.source_digest_set_hash);

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn python_workspace_resolves_pyenv_poetry_capsule_hint() {
        let root = temp_workspace("python");
        fs::write(root.join(".python-version"), "3.12.2\n").expect("write .python-version");
        fs::write(
            root.join("pyproject.toml"),
            r#"[tool.poetry]
name = "sample"
version = "0.1.0"

[tool.poetry.dependencies]
python = "^3.12"
"#,
        )
        .expect("write pyproject.toml");

        let resolution = EnvironmentCapsuleResolver::default_read_only()
            .resolve_workspace(&root, ProjectArchetypeHint::DataOrMlWorkbench);

        assert_eq!(resolution.capsule_hint, EnvironmentCapsuleHint::Python);
        assert_eq!(resolution.capsule_drift_state, CapsuleDriftState::InSync);
        assert_eq!(
            resolution.environment_capsule_ref.capsule_id,
            "capsule.alpha.python.local_python"
        );
        assert!(resolution
            .source_refs
            .iter()
            .any(|item| item == ".python-version"));
        assert!(resolution
            .source_refs
            .iter()
            .any(|item| item == "pyproject.toml#tool.poetry"));
        assert!(resolution.python_environment_detection.is_some());
        assert!(resolution.node_toolchain_detection.is_none());
        assert_eq!(
            resolution.prebuild_fingerprint_stub.reuse_state,
            PrebuildReuseState::Candidate
        );

        fs::remove_dir_all(root).ok();
    }

    #[test]
    fn no_signal_workspace_marks_capsule_lineage_uncertain() {
        let root = temp_workspace("empty");

        let resolution = EnvironmentCapsuleResolver::default_read_only().resolve_workspace(
            &root,
            ProjectArchetypeHint::ArchetypeClassUnknownRequiresReview,
        );

        assert_eq!(resolution.capsule_hint, EnvironmentCapsuleHint::NoSignal);
        assert_eq!(
            resolution.capsule_drift_state,
            CapsuleDriftState::UnknownLineage
        );
        assert_eq!(
            resolution.environment_capsule_ref.drift_state,
            CapsuleDriftState::UnknownLineage
        );
        assert!(resolution.source_refs.is_empty());
        assert_eq!(
            resolution.prebuild_fingerprint_stub.reuse_state,
            PrebuildReuseState::NotApplicable
        );
        assert!(resolution.node_toolchain_detection.is_none());
        assert!(resolution.python_environment_detection.is_none());

        fs::remove_dir_all(root).ok();
    }

    fn temp_workspace(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "aureline-capsule-resolver-{label}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&root).expect("create temp workspace");
        root
    }

    fn assert_hash_shape(value: &str) {
        let suffix = value.strip_prefix("sha256:").expect("sha256 prefix");
        assert_eq!(suffix.len(), 64);
        assert!(suffix.chars().all(|ch| ch.is_ascii_hexdigit()));
    }
}
