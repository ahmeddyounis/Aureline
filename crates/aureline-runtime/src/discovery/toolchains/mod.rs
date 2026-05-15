//! Opinion-free workspace toolchain discovery for execution-context records.
//!
//! This module performs read-only detection of common JavaScript, Python, and
//! quality-tool markers. It records presence and version evidence only; launch
//! readiness, installation, activation, and configuration decisions remain with
//! the task, test, debug, terminal, and AI surfaces that consume the canonical
//! execution context.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::detectors::node::{
    NodePackageManagerKind, NodeToolchainDetection, NodeToolchainDetector,
    NodeToolchainDetectorConfig, NodeToolchainResolutionState, NodeToolchainSourceKind,
};
use crate::detectors::python::{
    PythonEnvironmentDetection, PythonEnvironmentDetector, PythonEnvironmentDetectorConfig,
    PythonEnvironmentResolutionState, PythonEnvironmentSourceKind,
};

/// Stable record-kind tag emitted by [`WorkspaceToolchainDiscovery`].
pub const WORKSPACE_TOOLCHAIN_DISCOVERY_RECORD_KIND: &str = "workspace_toolchain_discovery_record";

/// Schema version for [`WorkspaceToolchainDiscovery`] payloads.
pub const WORKSPACE_TOOLCHAIN_DISCOVERY_SCHEMA_VERSION: u32 = 1;

/// Detector implementation version recorded on every discovery report.
pub const WORKSPACE_TOOLCHAIN_DETECTOR_VERSION: &str = "workspace.toolchains.discovery.alpha.v1";

/// Tool or environment family recorded by the workspace detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkspaceToolchainKind {
    /// Node.js runtime.
    Node,
    /// npm package manager.
    Npm,
    /// Yarn package manager.
    Yarn,
    /// pnpm package manager.
    Pnpm,
    /// Python interpreter.
    Python,
    /// pyenv-style `.python-version` selector.
    Pyenv,
    /// Local virtual environment.
    Venv,
    /// Poetry environment manager.
    Poetry,
    /// uv environment manager.
    Uv,
    /// TypeScript compiler (`tsc`).
    TypescriptCompiler,
    /// pytest test runner.
    Pytest,
    /// Ruff linter / formatter.
    Ruff,
    /// ESLint linter.
    Eslint,
}

impl WorkspaceToolchainKind {
    /// Stable string token used in support exports and inspector rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Node => "node",
            Self::Npm => "npm",
            Self::Yarn => "yarn",
            Self::Pnpm => "pnpm",
            Self::Python => "python",
            Self::Pyenv => "pyenv",
            Self::Venv => "venv",
            Self::Poetry => "poetry",
            Self::Uv => "uv",
            Self::TypescriptCompiler => "tsc",
            Self::Pytest => "pytest",
            Self::Ruff => "ruff",
            Self::Eslint => "eslint",
        }
    }

    /// Human-readable label for UI and plaintext support output.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Node => "Node.js",
            Self::Npm => "npm",
            Self::Yarn => "Yarn",
            Self::Pnpm => "pnpm",
            Self::Python => "Python",
            Self::Pyenv => "pyenv",
            Self::Venv => "Virtual environment",
            Self::Poetry => "Poetry",
            Self::Uv => "uv",
            Self::TypescriptCompiler => "TypeScript compiler",
            Self::Pytest => "pytest",
            Self::Ruff => "Ruff",
            Self::Eslint => "ESLint",
        }
    }
}

/// Presence state for one detected toolchain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolchainPresenceState {
    /// At least one evidence source indicates the tool is present.
    Present,
    /// No evidence source indicated the tool is present.
    Absent,
}

impl ToolchainPresenceState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Absent => "absent",
        }
    }
}

/// Source kind for one detection evidence row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolchainDetectionSourceKind {
    /// Existing Node detector report.
    NodeDetector,
    /// Existing Python detector report.
    PythonDetector,
    /// `package.json`.
    PackageJson,
    /// `package-lock.json` or `npm-shrinkwrap.json`.
    NpmLockfile,
    /// `yarn.lock`.
    YarnLockfile,
    /// `pnpm-lock.yaml`.
    PnpmLockfile,
    /// `.python-version`.
    PythonVersionFile,
    /// `.venv/pyvenv.cfg`.
    VenvPyvenvCfg,
    /// `.venv` directory.
    VenvDirectory,
    /// `pyproject.toml`.
    PyprojectToml,
    /// `poetry.lock`.
    PoetryLockfile,
    /// `uv.lock`.
    UvLockfile,
    /// pytest configuration file.
    PytestConfig,
    /// Ruff configuration file.
    RuffConfig,
    /// Python requirements file.
    RequirementsFile,
    /// Caller-provided ambient PATH fact.
    AmbientPath,
}

impl ToolchainDetectionSourceKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NodeDetector => "node_detector",
            Self::PythonDetector => "python_detector",
            Self::PackageJson => "package_json",
            Self::NpmLockfile => "npm_lockfile",
            Self::YarnLockfile => "yarn_lockfile",
            Self::PnpmLockfile => "pnpm_lockfile",
            Self::PythonVersionFile => "python_version_file",
            Self::VenvPyvenvCfg => "venv_pyvenv_cfg",
            Self::VenvDirectory => "venv_directory",
            Self::PyprojectToml => "pyproject_toml",
            Self::PoetryLockfile => "poetry_lockfile",
            Self::UvLockfile => "uv_lockfile",
            Self::PytestConfig => "pytest_config",
            Self::RuffConfig => "ruff_config",
            Self::RequirementsFile => "requirements_file",
            Self::AmbientPath => "ambient_path",
        }
    }
}

/// One evidence row explaining why a toolchain entry is present.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolchainDetectionEvidence {
    /// Evidence source kind.
    pub source_kind: ToolchainDetectionSourceKind,
    /// Stable source-kind token.
    pub source_kind_token: String,
    /// Workspace-relative source reference.
    pub source_ref: String,
    /// Version token when the source supplied one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Export-safe evidence summary.
    pub summary: String,
}

impl ToolchainDetectionEvidence {
    fn new(
        source_kind: ToolchainDetectionSourceKind,
        source_ref: impl Into<String>,
        version: Option<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            source_kind,
            source_kind_token: source_kind.as_str().to_owned(),
            source_ref: source_ref.into(),
            version,
            summary: summary.into(),
        }
    }
}

/// Presence and version evidence for one toolchain family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolchainDetectionEntry {
    /// Toolchain family.
    pub kind: WorkspaceToolchainKind,
    /// Stable toolchain family token.
    pub kind_token: String,
    /// Human-readable display name.
    pub display_name: String,
    /// Presence state.
    pub presence_state: ToolchainPresenceState,
    /// Stable presence-state token.
    pub presence_state_token: String,
    /// Selected display version, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Program name or workspace-relative executable hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executable_hint: Option<String>,
    /// Workspace-relative evidence refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    /// Evidence rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<ToolchainDetectionEvidence>,
}

impl ToolchainDetectionEntry {
    /// Builds a present entry from evidence.
    pub fn present(
        kind: WorkspaceToolchainKind,
        version: Option<String>,
        executable_hint: Option<String>,
        evidence: Vec<ToolchainDetectionEvidence>,
    ) -> Self {
        Self::new(
            kind,
            ToolchainPresenceState::Present,
            version,
            executable_hint,
            evidence,
        )
    }

    /// Builds an absent entry for a tool the detector knows how to inspect.
    pub fn absent(kind: WorkspaceToolchainKind) -> Self {
        Self::new(kind, ToolchainPresenceState::Absent, None, None, Vec::new())
    }

    /// Stable token summarizing the entry for cross-surface comparisons.
    pub fn value_token(&self) -> String {
        match (&self.presence_state, &self.version) {
            (ToolchainPresenceState::Present, Some(version)) if !version.is_empty() => {
                format!("{}@{}", self.kind.as_str(), version)
            }
            (ToolchainPresenceState::Present, _) => self.kind.as_str().to_owned(),
            (ToolchainPresenceState::Absent, _) => format!("{}@absent", self.kind.as_str()),
        }
    }

    fn new(
        kind: WorkspaceToolchainKind,
        presence_state: ToolchainPresenceState,
        version: Option<String>,
        executable_hint: Option<String>,
        evidence: Vec<ToolchainDetectionEvidence>,
    ) -> Self {
        let evidence_refs = evidence
            .iter()
            .map(|row| row.source_ref.clone())
            .collect::<Vec<_>>();
        Self {
            kind,
            kind_token: kind.as_str().to_owned(),
            display_name: kind.label().to_owned(),
            presence_state,
            presence_state_token: presence_state.as_str().to_owned(),
            version,
            executable_hint,
            evidence_refs,
            evidence,
        }
    }
}

/// Complete read-only toolchain discovery report for one workspace root.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceToolchainDiscovery {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Detector implementation version.
    pub detector_version: String,
    /// Workspace root inspected by this report.
    pub workspace_root_ref: String,
    /// Timestamp supplied by the caller.
    pub detected_at: String,
    /// Node detector report reused by launch-specific Node consumers.
    pub node_toolchain_detection: NodeToolchainDetection,
    /// Python detector report reused by launch-specific Python consumers.
    pub python_environment_detection: PythonEnvironmentDetection,
    /// Fixed-order toolchain entries for all supported detector families.
    pub detected_toolchains: Vec<ToolchainDetectionEntry>,
}

impl WorkspaceToolchainDiscovery {
    /// Returns a toolchain entry by family.
    pub fn entry(&self, kind: WorkspaceToolchainKind) -> Option<&ToolchainDetectionEntry> {
        self.detected_toolchains
            .iter()
            .find(|entry| entry.kind == kind)
    }

    /// Stable tokens for all present toolchain entries.
    pub fn present_toolchain_tokens(&self) -> Vec<String> {
        self.detected_toolchains
            .iter()
            .filter(|entry| entry.presence_state == ToolchainPresenceState::Present)
            .map(ToolchainDetectionEntry::value_token)
            .collect()
    }

    /// Stable tokens for all absent toolchain entries.
    pub fn absent_toolchain_tokens(&self) -> Vec<String> {
        self.detected_toolchains
            .iter()
            .filter(|entry| entry.presence_state == ToolchainPresenceState::Absent)
            .map(ToolchainDetectionEntry::value_token)
            .collect()
    }
}

/// Caller-provided facts for [`WorkspaceToolchainDetector`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct WorkspaceToolchainDetectorConfig {
    /// Node detector configuration.
    pub node_detector: NodeToolchainDetectorConfig,
    /// Python detector configuration.
    pub python_detector: PythonEnvironmentDetectorConfig,
    /// Captured ambient `tsc --version` fact, if already known.
    pub ambient_tsc_version: Option<String>,
    /// Captured ambient `pytest --version` fact, if already known.
    pub ambient_pytest_version: Option<String>,
    /// Captured ambient `ruff --version` fact, if already known.
    pub ambient_ruff_version: Option<String>,
    /// Captured ambient `eslint --version` fact, if already known.
    pub ambient_eslint_version: Option<String>,
}

/// Read-only detector for workspace toolchain presence and versions.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceToolchainDetector {
    config: WorkspaceToolchainDetectorConfig,
}

impl WorkspaceToolchainDetector {
    /// Creates a detector from caller-provided facts.
    pub fn new(config: WorkspaceToolchainDetectorConfig) -> Self {
        Self { config }
    }

    /// Creates a detector with no caller-provided facts.
    pub fn default_read_only() -> Self {
        Self::new(WorkspaceToolchainDetectorConfig::default())
    }

    /// Detects workspace toolchain presence without executing tools or hooks.
    pub fn detect_workspace(
        &self,
        workspace_root: &Path,
        detected_at: &str,
    ) -> WorkspaceToolchainDiscovery {
        let node_toolchain_detection =
            NodeToolchainDetector::new(self.config.node_detector.clone())
                .detect_workspace(workspace_root, detected_at);
        let python_environment_detection =
            PythonEnvironmentDetector::new(self.config.python_detector.clone())
                .detect_workspace(workspace_root, detected_at);
        let package_json = read_package_json(workspace_root);
        let pyproject = read_to_string_if_present(workspace_root, "pyproject.toml");

        let detected_toolchains = vec![
            node_entry(&node_toolchain_detection),
            package_manager_entry(
                workspace_root,
                &package_json,
                &node_toolchain_detection,
                NodePackageManagerKind::Npm,
                WorkspaceToolchainKind::Npm,
                self.config.node_detector.ambient_npm_version.clone(),
            ),
            package_manager_entry(
                workspace_root,
                &package_json,
                &node_toolchain_detection,
                NodePackageManagerKind::Yarn,
                WorkspaceToolchainKind::Yarn,
                self.config.node_detector.ambient_yarn_version.clone(),
            ),
            package_manager_entry(
                workspace_root,
                &package_json,
                &node_toolchain_detection,
                NodePackageManagerKind::Pnpm,
                WorkspaceToolchainKind::Pnpm,
                self.config.node_detector.ambient_pnpm_version.clone(),
            ),
            python_entry(&python_environment_detection),
            pyenv_entry(workspace_root),
            venv_entry(workspace_root),
            poetry_entry(
                workspace_root,
                pyproject.as_deref(),
                self.config.python_detector.ambient_poetry_version.clone(),
            ),
            uv_entry(
                workspace_root,
                pyproject.as_deref(),
                self.config.python_detector.ambient_uv_version.clone(),
            ),
            npm_tool_entry(
                WorkspaceToolchainKind::TypescriptCompiler,
                "typescript",
                "tsc",
                &package_json,
                self.config.ambient_tsc_version.clone(),
            ),
            python_tool_entry(
                WorkspaceToolchainKind::Pytest,
                "pytest",
                "pytest",
                workspace_root,
                pyproject.as_deref(),
                self.config.ambient_pytest_version.clone(),
            ),
            python_tool_entry(
                WorkspaceToolchainKind::Ruff,
                "ruff",
                "ruff",
                workspace_root,
                pyproject.as_deref(),
                self.config.ambient_ruff_version.clone(),
            ),
            npm_tool_entry(
                WorkspaceToolchainKind::Eslint,
                "eslint",
                "eslint",
                &package_json,
                self.config.ambient_eslint_version.clone(),
            ),
        ];

        WorkspaceToolchainDiscovery {
            record_kind: WORKSPACE_TOOLCHAIN_DISCOVERY_RECORD_KIND.to_owned(),
            schema_version: WORKSPACE_TOOLCHAIN_DISCOVERY_SCHEMA_VERSION,
            detector_version: WORKSPACE_TOOLCHAIN_DETECTOR_VERSION.to_owned(),
            workspace_root_ref: workspace_root.display().to_string(),
            detected_at: detected_at.to_owned(),
            node_toolchain_detection,
            python_environment_detection,
            detected_toolchains,
        }
    }
}

fn node_entry(detection: &NodeToolchainDetection) -> ToolchainDetectionEntry {
    let version = detection
        .node_runtime
        .resolved_requirement
        .clone()
        .or_else(|| {
            detection
                .node_runtime
                .fallback_path
                .as_ref()
                .and_then(|fallback| version_after_at(&fallback.value_token))
        });
    let mut evidence = Vec::new();
    if let Some(source) = detection.node_runtime.winning_source {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::NodeDetector,
            source.as_str(),
            version.clone(),
            format!("Node runtime resolved from {}.", node_source_label(source)),
        ));
    } else if detection.node_runtime.resolution_state == NodeToolchainResolutionState::Missing {
        return ToolchainDetectionEntry::absent(WorkspaceToolchainKind::Node);
    } else if let Some(fallback) = &detection.node_runtime.fallback_path {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::NodeDetector,
            fallback.source_kind.as_str(),
            version.clone(),
            fallback.summary.clone(),
        ));
    }
    if evidence.is_empty() {
        ToolchainDetectionEntry::absent(WorkspaceToolchainKind::Node)
    } else {
        ToolchainDetectionEntry::present(
            WorkspaceToolchainKind::Node,
            version,
            Some("node".to_owned()),
            evidence,
        )
    }
}

fn package_manager_entry(
    workspace_root: &Path,
    package_json: &Option<Value>,
    detection: &NodeToolchainDetection,
    manager: NodePackageManagerKind,
    kind: WorkspaceToolchainKind,
    ambient_version: Option<String>,
) -> ToolchainDetectionEntry {
    let mut evidence = Vec::new();
    let package_manager = detection.package_manager.kind == Some(manager);
    let mut version = if package_manager {
        detection.package_manager.version.clone()
    } else {
        None
    };

    if package_manager {
        let source_ref = detection
            .package_manager
            .winning_source
            .map(|source| source.as_str().to_owned())
            .unwrap_or_else(|| "node_detector".to_owned());
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::NodeDetector,
            source_ref,
            version.clone(),
            format!("{} resolved by the Node detector.", manager.as_str()),
        ));
    }

    match manager {
        NodePackageManagerKind::Npm => {
            if workspace_root.join("package-lock.json").is_file() {
                evidence.push(lockfile_evidence(
                    ToolchainDetectionSourceKind::NpmLockfile,
                    "package-lock.json",
                    version.clone().or_else(|| ambient_version.clone()),
                ));
            }
            if workspace_root.join("npm-shrinkwrap.json").is_file() {
                evidence.push(lockfile_evidence(
                    ToolchainDetectionSourceKind::NpmLockfile,
                    "npm-shrinkwrap.json",
                    version.clone().or_else(|| ambient_version.clone()),
                ));
            }
        }
        NodePackageManagerKind::Yarn => {
            if workspace_root.join("yarn.lock").is_file() {
                evidence.push(lockfile_evidence(
                    ToolchainDetectionSourceKind::YarnLockfile,
                    "yarn.lock",
                    version.clone().or_else(|| ambient_version.clone()),
                ));
            }
        }
        NodePackageManagerKind::Pnpm => {
            if workspace_root.join("pnpm-lock.yaml").is_file() {
                evidence.push(lockfile_evidence(
                    ToolchainDetectionSourceKind::PnpmLockfile,
                    "pnpm-lock.yaml",
                    version.clone().or_else(|| ambient_version.clone()),
                ));
            }
        }
        NodePackageManagerKind::Bun | NodePackageManagerKind::Unknown => {}
    }

    if version.is_none() {
        version = ambient_version.clone();
    }
    if evidence.is_empty() {
        if let Some(package_manager_value) =
            json_string(package_json, &["packageManager"]).and_then(parse_package_manager_name)
        {
            if package_manager_value == manager {
                evidence.push(ToolchainDetectionEvidence::new(
                    ToolchainDetectionSourceKind::PackageJson,
                    "package.json#packageManager",
                    version.clone(),
                    format!(
                        "package.json names {} as package manager.",
                        manager.as_str()
                    ),
                ));
            }
        }
    }
    if evidence.is_empty() {
        ToolchainDetectionEntry::absent(kind)
    } else {
        ToolchainDetectionEntry::present(kind, version, Some(manager.as_str().to_owned()), evidence)
    }
}

fn python_entry(detection: &PythonEnvironmentDetection) -> ToolchainDetectionEntry {
    if detection.interpreter.resolution_state == PythonEnvironmentResolutionState::Missing {
        return ToolchainDetectionEntry::absent(WorkspaceToolchainKind::Python);
    }
    let version = detection
        .interpreter
        .resolved_requirement
        .clone()
        .or_else(|| {
            detection
                .interpreter
                .fallback_path
                .as_ref()
                .and_then(|fallback| version_after_at(&fallback.value_token))
        });
    let mut evidence = Vec::new();
    if let Some(source) = detection.interpreter.winning_source {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::PythonDetector,
            source.as_str(),
            version.clone(),
            format!(
                "Python interpreter resolved from {}.",
                python_source_label(source)
            ),
        ));
    } else if let Some(fallback) = &detection.interpreter.fallback_path {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::PythonDetector,
            fallback.source_kind.as_str(),
            version.clone(),
            fallback.summary.clone(),
        ));
    }
    ToolchainDetectionEntry::present(
        WorkspaceToolchainKind::Python,
        version,
        detection
            .interpreter
            .interpreter_ref
            .clone()
            .or_else(|| Some("python".to_owned())),
        evidence,
    )
}

fn pyenv_entry(workspace_root: &Path) -> ToolchainDetectionEntry {
    match read_first_line(workspace_root, ".python-version") {
        Some(version) => ToolchainDetectionEntry::present(
            WorkspaceToolchainKind::Pyenv,
            Some(normalize_version(&version)),
            Some("pyenv".to_owned()),
            vec![ToolchainDetectionEvidence::new(
                ToolchainDetectionSourceKind::PythonVersionFile,
                ".python-version",
                Some(normalize_version(&version)),
                ".python-version is present.",
            )],
        ),
        None => ToolchainDetectionEntry::absent(WorkspaceToolchainKind::Pyenv),
    }
}

fn venv_entry(workspace_root: &Path) -> ToolchainDetectionEntry {
    if let Some(payload) = read_to_string_if_present(workspace_root, ".venv/pyvenv.cfg") {
        let version = pyvenv_version(&payload);
        return ToolchainDetectionEntry::present(
            WorkspaceToolchainKind::Venv,
            version.clone(),
            Some(".venv/bin/python".to_owned()),
            vec![ToolchainDetectionEvidence::new(
                ToolchainDetectionSourceKind::VenvPyvenvCfg,
                ".venv/pyvenv.cfg",
                version,
                ".venv/pyvenv.cfg is present.",
            )],
        );
    }
    if workspace_root.join(".venv").is_dir() {
        return ToolchainDetectionEntry::present(
            WorkspaceToolchainKind::Venv,
            None,
            Some(".venv/bin/python".to_owned()),
            vec![ToolchainDetectionEvidence::new(
                ToolchainDetectionSourceKind::VenvDirectory,
                ".venv",
                None,
                ".venv directory is present.",
            )],
        );
    }
    ToolchainDetectionEntry::absent(WorkspaceToolchainKind::Venv)
}

fn poetry_entry(
    workspace_root: &Path,
    pyproject: Option<&str>,
    ambient_version: Option<String>,
) -> ToolchainDetectionEntry {
    let mut evidence = Vec::new();
    if workspace_root.join("poetry.lock").is_file() {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::PoetryLockfile,
            "poetry.lock",
            ambient_version.clone(),
            "poetry.lock is present.",
        ));
    }
    if pyproject_has_section(pyproject, "tool.poetry") {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::PyprojectToml,
            "pyproject.toml#tool.poetry",
            ambient_version.clone(),
            "pyproject.toml has a Poetry section.",
        ));
    }
    if evidence.is_empty() {
        ToolchainDetectionEntry::absent(WorkspaceToolchainKind::Poetry)
    } else {
        ToolchainDetectionEntry::present(
            WorkspaceToolchainKind::Poetry,
            ambient_version,
            Some("poetry".to_owned()),
            evidence,
        )
    }
}

fn uv_entry(
    workspace_root: &Path,
    pyproject: Option<&str>,
    ambient_version: Option<String>,
) -> ToolchainDetectionEntry {
    let mut evidence = Vec::new();
    if workspace_root.join("uv.lock").is_file() {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::UvLockfile,
            "uv.lock",
            ambient_version.clone(),
            "uv.lock is present.",
        ));
    }
    if pyproject_has_section(pyproject, "tool.uv") {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::PyprojectToml,
            "pyproject.toml#tool.uv",
            ambient_version.clone(),
            "pyproject.toml has a uv section.",
        ));
    }
    if evidence.is_empty() {
        ToolchainDetectionEntry::absent(WorkspaceToolchainKind::Uv)
    } else {
        ToolchainDetectionEntry::present(
            WorkspaceToolchainKind::Uv,
            ambient_version,
            Some("uv".to_owned()),
            evidence,
        )
    }
}

fn npm_tool_entry(
    kind: WorkspaceToolchainKind,
    package_name: &str,
    executable: &str,
    package_json: &Option<Value>,
    ambient_version: Option<String>,
) -> ToolchainDetectionEntry {
    let manifest_version = package_dependency_version(package_json, package_name);
    let version = manifest_version.clone().or(ambient_version.clone());
    let mut evidence = Vec::new();
    if let Some(version) = manifest_version {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::PackageJson,
            format!("package.json#{package_name}"),
            Some(version),
            format!("package.json declares {package_name}."),
        ));
    }
    if evidence.is_empty() && ambient_version.is_some() {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::AmbientPath,
            executable,
            ambient_version.clone(),
            format!("Captured ambient {executable} version from host baseline."),
        ));
    }
    if evidence.is_empty() {
        ToolchainDetectionEntry::absent(kind)
    } else {
        ToolchainDetectionEntry::present(kind, version, Some(executable.to_owned()), evidence)
    }
}

fn python_tool_entry(
    kind: WorkspaceToolchainKind,
    package_name: &str,
    executable: &str,
    workspace_root: &Path,
    pyproject: Option<&str>,
    ambient_version: Option<String>,
) -> ToolchainDetectionEntry {
    let mut evidence = Vec::new();
    let mut version = ambient_version.clone();
    match kind {
        WorkspaceToolchainKind::Pytest => {
            if pyproject_has_section(pyproject, "tool.pytest")
                || pyproject_has_section(pyproject, "tool.pytest.ini_options")
            {
                evidence.push(ToolchainDetectionEvidence::new(
                    ToolchainDetectionSourceKind::PyprojectToml,
                    "pyproject.toml#tool.pytest",
                    ambient_version.clone(),
                    "pyproject.toml has pytest configuration.",
                ));
            }
            if workspace_root.join("pytest.ini").is_file() {
                evidence.push(ToolchainDetectionEvidence::new(
                    ToolchainDetectionSourceKind::PytestConfig,
                    "pytest.ini",
                    ambient_version.clone(),
                    "pytest.ini is present.",
                ));
            }
        }
        WorkspaceToolchainKind::Ruff => {
            if pyproject_has_section(pyproject, "tool.ruff") {
                evidence.push(ToolchainDetectionEvidence::new(
                    ToolchainDetectionSourceKind::PyprojectToml,
                    "pyproject.toml#tool.ruff",
                    ambient_version.clone(),
                    "pyproject.toml has Ruff configuration.",
                ));
            }
            for rel in ["ruff.toml", ".ruff.toml"] {
                if workspace_root.join(rel).is_file() {
                    evidence.push(ToolchainDetectionEvidence::new(
                        ToolchainDetectionSourceKind::RuffConfig,
                        rel,
                        ambient_version.clone(),
                        format!("{rel} is present."),
                    ));
                }
            }
        }
        _ => {}
    }

    if let Some(requirement_version) = requirements_declared_version(workspace_root, package_name) {
        if let Some(version_token) = &requirement_version {
            version = Some(version_token.clone());
        }
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::RequirementsFile,
            "requirements*.txt",
            requirement_version,
            format!("requirements file declares {package_name}."),
        ));
    }
    if evidence.is_empty() && ambient_version.is_some() {
        evidence.push(ToolchainDetectionEvidence::new(
            ToolchainDetectionSourceKind::AmbientPath,
            executable,
            ambient_version.clone(),
            format!("Captured ambient {executable} version from host baseline."),
        ));
    }
    if evidence.is_empty() {
        ToolchainDetectionEntry::absent(kind)
    } else {
        ToolchainDetectionEntry::present(kind, version, Some(executable.to_owned()), evidence)
    }
}

fn lockfile_evidence(
    source_kind: ToolchainDetectionSourceKind,
    source_ref: &str,
    version: Option<String>,
) -> ToolchainDetectionEvidence {
    ToolchainDetectionEvidence::new(
        source_kind,
        source_ref,
        version,
        format!("{source_ref} is present."),
    )
}

fn read_package_json(workspace_root: &Path) -> Option<Value> {
    let payload = fs::read_to_string(workspace_root.join("package.json")).ok()?;
    serde_json::from_str(&payload).ok()
}

fn read_to_string_if_present(workspace_root: &Path, rel: &str) -> Option<String> {
    fs::read_to_string(workspace_root.join(rel)).ok()
}

fn read_first_line(workspace_root: &Path, rel: &str) -> Option<String> {
    read_to_string_if_present(workspace_root, rel)?
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
}

fn json_string<'a>(value: &'a Option<Value>, path: &[&str]) -> Option<&'a str> {
    let mut current = value.as_ref()?;
    for segment in path {
        current = current.get(*segment)?;
    }
    current.as_str()
}

fn parse_package_manager_name(raw: &str) -> Option<NodePackageManagerKind> {
    let name = raw
        .trim()
        .rsplit_once('@')
        .map(|(name, _)| name)
        .unwrap_or(raw)
        .trim();
    match name {
        "npm" => Some(NodePackageManagerKind::Npm),
        "yarn" => Some(NodePackageManagerKind::Yarn),
        "pnpm" => Some(NodePackageManagerKind::Pnpm),
        _ => None,
    }
}

fn package_dependency_version(package_json: &Option<Value>, package_name: &str) -> Option<String> {
    for section in [
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ] {
        if let Some(version) = json_string(package_json, &[section, package_name]) {
            return Some(normalize_version(version));
        }
    }
    None
}

fn pyproject_has_section(pyproject: Option<&str>, section: &str) -> bool {
    let Some(payload) = pyproject else {
        return false;
    };
    let exact = format!("[{section}]");
    let prefix = format!("[{section}.");
    payload.lines().any(|line| {
        let line = strip_toml_comment(line).trim();
        line == exact || line.starts_with(&prefix)
    })
}

fn requirements_declared_version(
    workspace_root: &Path,
    package_name: &str,
) -> Option<Option<String>> {
    for rel in [
        "requirements.txt",
        "requirements-dev.txt",
        "dev-requirements.txt",
        "test-requirements.txt",
    ] {
        let Some(payload) = read_to_string_if_present(workspace_root, rel) else {
            continue;
        };
        for raw_line in payload.lines() {
            let line = raw_line.split('#').next().unwrap_or_default().trim();
            if line.is_empty() {
                continue;
            }
            if let Some(version) = requirement_version_token(line, package_name) {
                return Some(version);
            }
        }
    }
    None
}

fn requirement_version_token(line: &str, package_name: &str) -> Option<Option<String>> {
    let requirement = line.split(';').next().unwrap_or_default().trim();
    if requirement.is_empty() {
        return None;
    }
    let package_name = package_name.to_ascii_lowercase();
    if requirement.to_ascii_lowercase() == package_name {
        return Some(None);
    }
    for operator in ["==", "~=", ">=", "<=", "!=", ">", "<"] {
        let Some((raw_name, raw_version)) = requirement.split_once(operator) else {
            continue;
        };
        let normalized_name = raw_name
            .trim()
            .split('[')
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if normalized_name != package_name {
            continue;
        }
        let raw_version = raw_version.trim();
        if raw_version.is_empty() {
            return Some(None);
        }
        let version = normalize_version(raw_version);
        return Some(Some(if operator == "==" {
            version
        } else {
            format!("{operator}{version}")
        }));
    }
    None
}

fn pyvenv_version(payload: &str) -> Option<String> {
    for raw_line in payload.lines() {
        let line = raw_line.split('#').next().unwrap_or_default().trim();
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        if key.trim() == "version" {
            return Some(normalize_version(value));
        }
    }
    None
}

fn strip_toml_comment(line: &str) -> &str {
    let mut in_single = false;
    let mut in_double = false;
    for (idx, ch) in line.char_indices() {
        match ch {
            '\'' if !in_double => in_single = !in_single,
            '"' if !in_single => in_double = !in_double,
            '#' if !in_single && !in_double => return &line[..idx],
            _ => {}
        }
    }
    line
}

fn version_after_at(value: &str) -> Option<String> {
    value
        .split_once('@')
        .map(|(_, version)| normalize_version(version))
        .filter(|version| version != "unresolved" && !version.is_empty())
}

fn normalize_version(raw: impl AsRef<str>) -> String {
    let mut value = raw.as_ref().trim();
    value = value.strip_prefix("Python ").unwrap_or(value);
    value = value.strip_prefix("python ").unwrap_or(value);
    value = value.strip_prefix('v').unwrap_or(value);
    value.to_owned()
}

const fn node_source_label(source: NodeToolchainSourceKind) -> &'static str {
    match source {
        NodeToolchainSourceKind::ExplicitOverride => "explicit override",
        NodeToolchainSourceKind::PackageJsonPackageManager => "package.json packageManager",
        NodeToolchainSourceKind::PackageJsonEngines => "package.json engines",
        NodeToolchainSourceKind::PackageJsonVolta => "package.json Volta pin",
        NodeToolchainSourceKind::Nvmrc => ".nvmrc",
        NodeToolchainSourceKind::NodeVersionFile => ".node-version",
        NodeToolchainSourceKind::ToolVersions => ".tool-versions",
        NodeToolchainSourceKind::MiseToml => "mise.toml",
        NodeToolchainSourceKind::YarnLockfile => "Yarn lockfile",
        NodeToolchainSourceKind::PnpmLockfile => "pnpm lockfile",
        NodeToolchainSourceKind::NpmLockfile => "npm lockfile",
        NodeToolchainSourceKind::UserProfileDefault => "user/profile default",
        NodeToolchainSourceKind::AmbientPath => "ambient PATH",
        NodeToolchainSourceKind::DetectorFallback => "detector fallback",
        NodeToolchainSourceKind::UnreadableSource => "unreadable source",
    }
}

const fn python_source_label(source: PythonEnvironmentSourceKind) -> &'static str {
    match source {
        PythonEnvironmentSourceKind::ExplicitOverride => "explicit override",
        PythonEnvironmentSourceKind::VenvPyvenvCfg => ".venv pyvenv.cfg",
        PythonEnvironmentSourceKind::VenvDirectory => ".venv directory",
        PythonEnvironmentSourceKind::PyenvVersionFile => ".python-version",
        PythonEnvironmentSourceKind::PythonVersionFile => ".python-version",
        PythonEnvironmentSourceKind::ToolVersions => ".tool-versions",
        PythonEnvironmentSourceKind::MiseToml => "mise.toml",
        PythonEnvironmentSourceKind::PyprojectRequiresPython => "pyproject requires-python",
        PythonEnvironmentSourceKind::PyprojectPoetryDependency => {
            "pyproject Poetry python dependency"
        }
        PythonEnvironmentSourceKind::UvLockfile => "uv lockfile",
        PythonEnvironmentSourceKind::PyprojectUv => "pyproject uv section",
        PythonEnvironmentSourceKind::PoetryLockfile => "Poetry lockfile",
        PythonEnvironmentSourceKind::PyprojectPoetry => "pyproject Poetry section",
        PythonEnvironmentSourceKind::CondaEnvironmentFile => "Conda environment file",
        PythonEnvironmentSourceKind::UserProfileDefault => "user/profile default",
        PythonEnvironmentSourceKind::AmbientPath => "ambient PATH",
        PythonEnvironmentSourceKind::DetectorFallback => "detector fallback",
        PythonEnvironmentSourceKind::UnreadableSource => "unreadable source",
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn fixture_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/toolchain_detection_entry_points/sample_workspace")
    }

    fn detector() -> WorkspaceToolchainDetector {
        WorkspaceToolchainDetector::new(WorkspaceToolchainDetectorConfig {
            node_detector: NodeToolchainDetectorConfig {
                ambient_node_version: Some("22.11.0".to_owned()),
                ambient_npm_version: Some("10.9.0".to_owned()),
                ambient_yarn_version: Some("1.22.22".to_owned()),
                ambient_pnpm_version: Some("9.15.4".to_owned()),
                ..NodeToolchainDetectorConfig::default()
            },
            python_detector: PythonEnvironmentDetectorConfig {
                ambient_python_version: Some("3.12.7".to_owned()),
                ambient_interpreter_ref: Some("/usr/bin/python3".to_owned()),
                ambient_uv_version: Some("0.5.7".to_owned()),
                ambient_poetry_version: Some("1.8.4".to_owned()),
                ..PythonEnvironmentDetectorConfig::default()
            },
            ambient_tsc_version: Some("5.7.2".to_owned()),
            ambient_pytest_version: Some("8.3.4".to_owned()),
            ambient_ruff_version: Some("0.8.4".to_owned()),
            ambient_eslint_version: Some("9.16.0".to_owned()),
        })
    }

    #[test]
    fn sample_workspace_reports_all_entry_point_toolchains() {
        let report = detector().detect_workspace(&fixture_root(), "2026-05-15T12:00:00Z");

        assert_eq!(
            report.record_kind,
            WORKSPACE_TOOLCHAIN_DISCOVERY_RECORD_KIND
        );
        for kind in [
            WorkspaceToolchainKind::Node,
            WorkspaceToolchainKind::Npm,
            WorkspaceToolchainKind::Yarn,
            WorkspaceToolchainKind::Pnpm,
            WorkspaceToolchainKind::Python,
            WorkspaceToolchainKind::Pyenv,
            WorkspaceToolchainKind::Venv,
            WorkspaceToolchainKind::Poetry,
            WorkspaceToolchainKind::Uv,
            WorkspaceToolchainKind::TypescriptCompiler,
            WorkspaceToolchainKind::Pytest,
            WorkspaceToolchainKind::Ruff,
            WorkspaceToolchainKind::Eslint,
        ] {
            let entry = report.entry(kind).expect("entry exists for kind");
            assert_eq!(entry.presence_state, ToolchainPresenceState::Present);
        }

        let tokens = report.present_toolchain_tokens();
        assert!(tokens.contains(&"node@22.11.0".to_owned()));
        assert!(tokens.contains(&"npm@10.9.0".to_owned()));
        assert!(tokens.contains(&"yarn@1.22.22".to_owned()));
        assert!(tokens.contains(&"pnpm@9.15.4".to_owned()));
        assert!(tokens.contains(&"python@3.12.7".to_owned()));
        assert!(tokens.contains(&"tsc@5.7.2".to_owned()));
        assert!(tokens.contains(&"pytest@8.3.4".to_owned()));
        assert!(tokens.contains(&"ruff@0.8.4".to_owned()));
        assert!(tokens.contains(&"eslint@9.16.0".to_owned()));
    }
}
