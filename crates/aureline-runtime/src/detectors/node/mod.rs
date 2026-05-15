//! Node.js and package-manager detector for TS/JS launch-wedge workspaces.
//!
//! The detector is intentionally read-only. It inspects repository-owned
//! manifests and version files, combines them with caller-provided ambient
//! facts such as `node` or `pnpm` versions already known to the host, and
//! returns provenance cards that explain which source won before a task,
//! test, or debug launch is dispatched.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Stable record-kind tag emitted by [`NodeToolchainDetection`].
pub const NODE_TOOLCHAIN_DETECTION_RECORD_KIND: &str = "node_toolchain_detection_record";

/// Schema version for [`NodeToolchainDetection`] payloads.
pub const NODE_TOOLCHAIN_DETECTION_SCHEMA_VERSION: u32 = 1;

/// Detector implementation version recorded on every report.
pub const NODE_TOOLCHAIN_DETECTOR_VERSION: &str = "node.detector.alpha.v1";

/// Subject a Node detector source contributes to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeToolchainSubject {
    /// Node.js runtime selection.
    NodeRuntime,
    /// JavaScript package-manager runner selection.
    PackageManager,
}

impl NodeToolchainSubject {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NodeRuntime => "node_runtime",
            Self::PackageManager => "package_manager",
        }
    }
}

/// Source kind observed by the Node detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeToolchainSourceKind {
    /// Caller supplied an action-local override.
    ExplicitOverride,
    /// `package.json#packageManager`.
    PackageJsonPackageManager,
    /// `package.json#engines.node`.
    PackageJsonEngines,
    /// `package.json#volta`.
    PackageJsonVolta,
    /// `.nvmrc`.
    Nvmrc,
    /// `.node-version`.
    NodeVersionFile,
    /// `.tool-versions`.
    ToolVersions,
    /// `mise.toml`.
    MiseToml,
    /// `pnpm-lock.yaml`.
    PnpmLockfile,
    /// `yarn.lock`.
    YarnLockfile,
    /// `package-lock.json` or `npm-shrinkwrap.json`.
    NpmLockfile,
    /// User or profile default supplied by the caller.
    UserProfileDefault,
    /// Host PATH or captured ambient toolchain fact supplied by the caller.
    AmbientPath,
    /// Detector fallback used when no authoritative source settled a value.
    DetectorFallback,
    /// Source file was present but unreadable or unparsable.
    UnreadableSource,
}

impl NodeToolchainSourceKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitOverride => "explicit_override",
            Self::PackageJsonPackageManager => "package_json_package_manager",
            Self::PackageJsonEngines => "package_json_engines",
            Self::PackageJsonVolta => "package_json_volta",
            Self::Nvmrc => "nvmrc",
            Self::NodeVersionFile => "node_version_file",
            Self::ToolVersions => "tool_versions",
            Self::MiseToml => "mise_toml",
            Self::PnpmLockfile => "pnpm_lockfile",
            Self::YarnLockfile => "yarn_lockfile",
            Self::NpmLockfile => "npm_lockfile",
            Self::UserProfileDefault => "user_profile_default",
            Self::AmbientPath => "ambient_path",
            Self::DetectorFallback => "detector_fallback",
            Self::UnreadableSource => "unreadable_source",
        }
    }
}

/// Package-manager kind understood by the launch-wedge detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodePackageManagerKind {
    /// `npm`.
    Npm,
    /// `pnpm`.
    Pnpm,
    /// `yarn`.
    Yarn,
    /// `bun`, detected but not part of the launch-wedge runner contract.
    Bun,
    /// A package-manager token the alpha detector cannot classify.
    Unknown,
}

impl NodePackageManagerKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Npm => "npm",
            Self::Pnpm => "pnpm",
            Self::Yarn => "yarn",
            Self::Bun => "bun",
            Self::Unknown => "unknown",
        }
    }

    /// True when this package manager is in the launch wedge.
    pub const fn is_launch_wedge_supported(self) -> bool {
        matches!(self, Self::Npm | Self::Pnpm | Self::Yarn)
    }
}

/// Version requirement for a JavaScript package-manager runner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodePackageManagerRequirement {
    /// Package-manager family.
    pub kind: NodePackageManagerKind,
    /// Version or range token when the source supplied one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl NodePackageManagerRequirement {
    /// Constructs a new package-manager requirement.
    pub fn new(kind: NodePackageManagerKind, version: Option<String>) -> Self {
        Self { kind, version }
    }

    /// Stable value token used in provenance cards.
    pub fn value_token(&self) -> String {
        match &self.version {
            Some(version) if !version.is_empty() => {
                format!("{}@{}", self.kind.as_str(), version)
            }
            _ => self.kind.as_str().to_owned(),
        }
    }
}

/// Resolution state for a Node runtime or package-manager selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeToolchainResolutionState {
    /// A supported source resolved a value.
    Resolved,
    /// No authoritative source resolved a value and the detector supplied a
    /// visible fallback path.
    Fallback,
    /// No usable source was available.
    Missing,
    /// Multiple same-precedence sources disagreed and no winner is safe.
    Ambiguous,
    /// A source resolved a value outside the launch-wedge contract.
    Unsupported,
}

impl NodeToolchainResolutionState {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Resolved => "resolved",
            Self::Fallback => "fallback",
            Self::Missing => "missing",
            Self::Ambiguous => "ambiguous",
            Self::Unsupported => "unsupported",
        }
    }

    /// True when the state requires an honesty marker before launch.
    pub const fn requires_honesty_marker(self) -> bool {
        !matches!(self, Self::Resolved)
    }
}

/// Disposition of one provenance card in the detector report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeToolchainProvenanceDisposition {
    /// This source won the precedence contest.
    Winning,
    /// This source is the visible fallback path.
    Fallback,
    /// This source agreed with the winning value.
    Corroborating,
    /// This source lost because a higher-precedence source disagreed.
    Conflicting,
    /// This source participates in an unresolved same-precedence ambiguity.
    Ambiguous,
    /// This source was observed but did not affect the selection.
    Ignored,
    /// This source names a tool outside the launch-wedge runner contract.
    Unsupported,
}

impl NodeToolchainProvenanceDisposition {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Winning => "winning",
            Self::Fallback => "fallback",
            Self::Corroborating => "corroborating",
            Self::Conflicting => "conflicting",
            Self::Ambiguous => "ambiguous",
            Self::Ignored => "ignored",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Visible fallback path when the detector cannot rely only on the winner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeToolchainFallbackPath {
    /// Subject the fallback applies to.
    pub subject: NodeToolchainSubject,
    /// Source that produced the fallback.
    pub source_kind: NodeToolchainSourceKind,
    /// Stable token for the fallback value.
    pub value_token: String,
    /// Human-readable summary suitable for inspectors and support exports.
    pub summary: String,
}

/// Runtime selection produced by the Node detector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeRuntimeResolution {
    /// Final resolution state.
    pub resolution_state: NodeToolchainResolutionState,
    /// Winning source, when one source safely won.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_source: Option<NodeToolchainSourceKind>,
    /// Resolved Node version or range token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_requirement: Option<String>,
    /// Visible fallback path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_path: Option<NodeToolchainFallbackPath>,
}

/// Package-manager selection produced by the Node detector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodePackageManagerResolution {
    /// Final resolution state.
    pub resolution_state: NodeToolchainResolutionState,
    /// Winning source, when one source safely won.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_source: Option<NodeToolchainSourceKind>,
    /// Resolved package-manager kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<NodePackageManagerKind>,
    /// Version or range token when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Visible fallback path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_path: Option<NodeToolchainFallbackPath>,
}

/// One detector provenance card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeToolchainProvenanceCard {
    /// Stable card id inside the detector report.
    pub card_id: String,
    /// Runtime or package-manager subject.
    pub subject: NodeToolchainSubject,
    /// Source kind that produced the card.
    pub source_kind: NodeToolchainSourceKind,
    /// Workspace-relative source reference, when the source is a file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Card disposition after precedence was applied.
    pub disposition: NodeToolchainProvenanceDisposition,
    /// Stable value token, when this card carries a value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_token: Option<String>,
    /// Human-readable explanation for inspector rows.
    pub summary: String,
}

/// Unresolved detector ambiguity that must remain visible before launch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeToolchainAmbiguity {
    /// Runtime or package-manager subject.
    pub subject: NodeToolchainSubject,
    /// Conflicting value tokens.
    pub candidate_values: Vec<String>,
    /// Source refs or source-kind tokens that produced the conflict.
    pub source_refs: Vec<String>,
    /// Reviewable hint for a repair or manual selection flow.
    pub resolution_hint: String,
}

/// Complete read-only Node detector report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NodeToolchainDetection {
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
    /// Node runtime resolution.
    pub node_runtime: NodeRuntimeResolution,
    /// Package-manager resolution.
    pub package_manager: NodePackageManagerResolution,
    /// Ordered provenance cards.
    pub provenance_cards: Vec<NodeToolchainProvenanceCard>,
    /// Unresolved ambiguities that must be surfaced before launch.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unresolved_ambiguities: Vec<NodeToolchainAmbiguity>,
}

impl NodeToolchainDetection {
    /// True when any subject fell back to a lower-confidence path.
    pub fn has_fallback(&self) -> bool {
        self.node_runtime.fallback_path.is_some()
            || self.package_manager.fallback_path.is_some()
            || self
                .provenance_cards
                .iter()
                .any(|card| card.disposition == NodeToolchainProvenanceDisposition::Fallback)
    }

    /// True when a same-precedence ambiguity was left unresolved.
    pub fn has_unresolved_ambiguity(&self) -> bool {
        !self.unresolved_ambiguities.is_empty()
            || self.node_runtime.resolution_state == NodeToolchainResolutionState::Ambiguous
            || self.package_manager.resolution_state == NodeToolchainResolutionState::Ambiguous
    }
}

/// Caller-provided facts and overrides for [`NodeToolchainDetector`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NodeToolchainDetectorConfig {
    /// Action-local Node version override.
    pub explicit_node_version: Option<String>,
    /// Action-local package-manager override.
    pub explicit_package_manager: Option<NodePackageManagerRequirement>,
    /// User/profile default Node version.
    pub profile_node_version: Option<String>,
    /// User/profile default package manager.
    pub profile_package_manager: Option<NodePackageManagerRequirement>,
    /// Captured ambient `node --version` fact, if a host probe already has it.
    pub ambient_node_version: Option<String>,
    /// Captured ambient `npm --version` fact, if a host probe already has it.
    pub ambient_npm_version: Option<String>,
    /// Captured ambient `pnpm --version` fact, if a host probe already has it.
    pub ambient_pnpm_version: Option<String>,
    /// Captured ambient `yarn --version` fact, if a host probe already has it.
    pub ambient_yarn_version: Option<String>,
}

/// Read-only Node.js detector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NodeToolchainDetector {
    config: NodeToolchainDetectorConfig,
}

impl NodeToolchainDetector {
    /// Creates a detector with caller-provided facts and overrides.
    pub fn new(config: NodeToolchainDetectorConfig) -> Self {
        Self { config }
    }

    /// Creates a detector with no caller-provided facts.
    pub fn default_read_only() -> Self {
        Self::new(NodeToolchainDetectorConfig::default())
    }

    /// Detects Node and package-manager selections for one workspace root.
    ///
    /// The method reads known manifest and version files but never executes
    /// repository-owned hooks or package-manager binaries.
    pub fn detect_workspace(
        &self,
        workspace_root: &Path,
        detected_at: &str,
    ) -> NodeToolchainDetection {
        let workspace_root_ref = workspace_root.display().to_string();
        let package_json = read_package_json(workspace_root);
        let mut unreadable_cards = package_json
            .error
            .into_iter()
            .map(|summary| NodeToolchainProvenanceCard {
                card_id: "node_detector.card.unreadable.package_json".to_owned(),
                subject: NodeToolchainSubject::PackageManager,
                source_kind: NodeToolchainSourceKind::UnreadableSource,
                source_ref: Some("package.json".to_owned()),
                disposition: NodeToolchainProvenanceDisposition::Unsupported,
                value_token: None,
                summary,
            })
            .collect::<Vec<_>>();

        let runtime_candidates =
            collect_runtime_candidates(workspace_root, &self.config, &package_json.value);
        let package_candidates =
            collect_package_manager_candidates(workspace_root, &self.config, &package_json.value);

        let runtime = resolve_runtime(runtime_candidates, &self.config);
        let package_manager = resolve_package_manager(
            package_candidates,
            &self.config,
            runtime.resolution.resolution_state,
        );

        let mut provenance_cards = Vec::new();
        provenance_cards.extend(runtime.cards);
        provenance_cards.extend(package_manager.cards);
        provenance_cards.append(&mut unreadable_cards);
        provenance_cards.sort_by(|left, right| left.card_id.cmp(&right.card_id));

        let mut unresolved_ambiguities = runtime.ambiguities;
        unresolved_ambiguities.extend(package_manager.ambiguities);

        NodeToolchainDetection {
            record_kind: NODE_TOOLCHAIN_DETECTION_RECORD_KIND.to_owned(),
            schema_version: NODE_TOOLCHAIN_DETECTION_SCHEMA_VERSION,
            detector_version: NODE_TOOLCHAIN_DETECTOR_VERSION.to_owned(),
            workspace_root_ref,
            detected_at: detected_at.to_owned(),
            node_runtime: runtime.resolution,
            package_manager: package_manager.resolution,
            provenance_cards,
            unresolved_ambiguities,
        }
    }
}

#[derive(Debug, Clone)]
struct PackageJsonRead {
    value: Option<Value>,
    error: Option<String>,
}

#[derive(Debug, Clone)]
struct RuntimeCandidate {
    source_kind: NodeToolchainSourceKind,
    source_ref: Option<String>,
    value: String,
    precedence_group: u8,
    source_order: u8,
}

#[derive(Debug, Clone)]
struct PackageManagerCandidate {
    source_kind: NodeToolchainSourceKind,
    source_ref: Option<String>,
    requirement: NodePackageManagerRequirement,
    precedence_group: u8,
    source_order: u8,
}

#[derive(Debug, Clone)]
struct RuntimeResolveOutput {
    resolution: NodeRuntimeResolution,
    cards: Vec<NodeToolchainProvenanceCard>,
    ambiguities: Vec<NodeToolchainAmbiguity>,
}

#[derive(Debug, Clone)]
struct PackageManagerResolveOutput {
    resolution: NodePackageManagerResolution,
    cards: Vec<NodeToolchainProvenanceCard>,
    ambiguities: Vec<NodeToolchainAmbiguity>,
}

fn read_package_json(workspace_root: &Path) -> PackageJsonRead {
    let path = workspace_root.join("package.json");
    let payload = match fs::read_to_string(&path) {
        Ok(payload) => payload,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return PackageJsonRead {
                value: None,
                error: None,
            };
        }
        Err(err) => {
            return PackageJsonRead {
                value: None,
                error: Some(format!("package.json could not be read: {err}")),
            };
        }
    };
    match serde_json::from_str(&payload) {
        Ok(value) => PackageJsonRead {
            value: Some(value),
            error: None,
        },
        Err(err) => PackageJsonRead {
            value: None,
            error: Some(format!("package.json could not be parsed: {err}")),
        },
    }
}

fn collect_runtime_candidates(
    workspace_root: &Path,
    config: &NodeToolchainDetectorConfig,
    package_json: &Option<Value>,
) -> Vec<RuntimeCandidate> {
    let mut candidates = Vec::new();
    if let Some(value) = config
        .explicit_node_version
        .as_deref()
        .and_then(normalize_version)
    {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::ExplicitOverride,
            None,
            value,
            0,
            0,
        ));
    }
    if let Some(value) = json_string(package_json, &["volta", "node"]).and_then(normalize_version) {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::PackageJsonVolta,
            Some("package.json#volta.node"),
            value,
            1,
            0,
        ));
    }
    if let Some(value) = read_first_line(workspace_root, ".nvmrc").and_then(normalize_version) {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::Nvmrc,
            Some(".nvmrc"),
            value,
            1,
            1,
        ));
    }
    if let Some(value) =
        read_first_line(workspace_root, ".node-version").and_then(normalize_version)
    {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::NodeVersionFile,
            Some(".node-version"),
            value,
            1,
            2,
        ));
    }
    if let Some(value) = read_tool_versions(workspace_root, "nodejs").and_then(normalize_version) {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::ToolVersions,
            Some(".tool-versions#nodejs"),
            value,
            1,
            3,
        ));
    }
    if let Some(value) =
        read_mise_tool(workspace_root, &["node", "nodejs"]).and_then(normalize_version)
    {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::MiseToml,
            Some("mise.toml#tools.node"),
            value,
            1,
            4,
        ));
    }
    if let Some(value) = json_string(package_json, &["engines", "node"]).and_then(normalize_version)
    {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::PackageJsonEngines,
            Some("package.json#engines.node"),
            value,
            2,
            0,
        ));
    }
    if let Some(value) = config
        .profile_node_version
        .as_deref()
        .and_then(normalize_version)
    {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::UserProfileDefault,
            None,
            value,
            4,
            0,
        ));
    }
    if let Some(value) = config
        .ambient_node_version
        .as_deref()
        .and_then(normalize_version)
    {
        candidates.push(runtime_candidate(
            NodeToolchainSourceKind::AmbientPath,
            None,
            value,
            5,
            0,
        ));
    }
    candidates
}

fn collect_package_manager_candidates(
    workspace_root: &Path,
    config: &NodeToolchainDetectorConfig,
    package_json: &Option<Value>,
) -> Vec<PackageManagerCandidate> {
    let mut candidates = Vec::new();
    if let Some(requirement) = &config.explicit_package_manager {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::ExplicitOverride,
            None,
            requirement.clone(),
            0,
            0,
        ));
    }
    if let Some(requirement) =
        json_string(package_json, &["packageManager"]).and_then(parse_package_manager_requirement)
    {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::PackageJsonPackageManager,
            Some("package.json#packageManager"),
            requirement,
            1,
            0,
        ));
    }
    if let Some(version) = json_string(package_json, &["volta", "pnpm"]).and_then(normalize_version)
    {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::PackageJsonVolta,
            Some("package.json#volta.pnpm"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Pnpm, Some(version)),
            1,
            1,
        ));
    }
    if let Some(version) = json_string(package_json, &["volta", "npm"]).and_then(normalize_version)
    {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::PackageJsonVolta,
            Some("package.json#volta.npm"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Npm, Some(version)),
            1,
            2,
        ));
    }
    if let Some(version) = read_tool_versions(workspace_root, "pnpm").and_then(normalize_version) {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::ToolVersions,
            Some(".tool-versions#pnpm"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Pnpm, Some(version)),
            2,
            0,
        ));
    }
    if let Some(version) = read_tool_versions(workspace_root, "npm").and_then(normalize_version) {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::ToolVersions,
            Some(".tool-versions#npm"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Npm, Some(version)),
            2,
            1,
        ));
    }
    if let Some(version) = read_mise_tool(workspace_root, &["pnpm"]).and_then(normalize_version) {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::MiseToml,
            Some("mise.toml#tools.pnpm"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Pnpm, Some(version)),
            2,
            2,
        ));
    }
    if let Some(version) = read_mise_tool(workspace_root, &["npm"]).and_then(normalize_version) {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::MiseToml,
            Some("mise.toml#tools.npm"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Npm, Some(version)),
            2,
            3,
        ));
    }
    if workspace_root.join("pnpm-lock.yaml").is_file() {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::PnpmLockfile,
            Some("pnpm-lock.yaml"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Pnpm, None),
            3,
            0,
        ));
    }
    if workspace_root.join("yarn.lock").is_file() {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::YarnLockfile,
            Some("yarn.lock"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Yarn, None),
            3,
            1,
        ));
    }
    if workspace_root.join("package-lock.json").is_file() {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::NpmLockfile,
            Some("package-lock.json"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Npm, None),
            3,
            2,
        ));
    }
    if workspace_root.join("npm-shrinkwrap.json").is_file() {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::NpmLockfile,
            Some("npm-shrinkwrap.json"),
            NodePackageManagerRequirement::new(NodePackageManagerKind::Npm, None),
            3,
            3,
        ));
    }
    if let Some(requirement) = &config.profile_package_manager {
        candidates.push(package_candidate(
            NodeToolchainSourceKind::UserProfileDefault,
            None,
            requirement.clone(),
            4,
            0,
        ));
    }
    candidates
}

fn resolve_runtime(
    mut candidates: Vec<RuntimeCandidate>,
    config: &NodeToolchainDetectorConfig,
) -> RuntimeResolveOutput {
    candidates.sort_by_key(|candidate| (candidate.precedence_group, candidate.source_order));
    if candidates.is_empty() {
        let fallback = runtime_fallback(config);
        return RuntimeResolveOutput {
            resolution: NodeRuntimeResolution {
                resolution_state: NodeToolchainResolutionState::Missing,
                winning_source: None,
                resolved_requirement: None,
                fallback_path: Some(fallback.clone()),
            },
            cards: vec![fallback_card(
                NodeToolchainSubject::NodeRuntime,
                &fallback,
                "Node runtime did not resolve from workspace pins; launch must select or install Node.",
            )],
            ambiguities: Vec::new(),
        };
    }

    let winning_group = candidates[0].precedence_group;
    let same_group = candidates
        .iter()
        .filter(|candidate| candidate.precedence_group == winning_group)
        .collect::<Vec<_>>();
    let distinct_values = distinct_runtime_values(&same_group);
    if distinct_values.len() > 1 {
        let fallback = runtime_fallback(config);
        let ambiguity = ambiguity_for_runtime(&same_group, distinct_values);
        let mut cards = cards_for_runtime_candidates(
            &candidates,
            None,
            NodeToolchainProvenanceDisposition::Ambiguous,
            Some(winning_group),
        );
        cards.push(fallback_card(
            NodeToolchainSubject::NodeRuntime,
            &fallback,
            "Node runtime ambiguity requires review before launch; fallback is inspect-only.",
        ));
        return RuntimeResolveOutput {
            resolution: NodeRuntimeResolution {
                resolution_state: NodeToolchainResolutionState::Ambiguous,
                winning_source: None,
                resolved_requirement: None,
                fallback_path: Some(fallback),
            },
            cards,
            ambiguities: vec![ambiguity],
        };
    }

    let winner = candidates[0].clone();
    let fallback = runtime_fallback(config);
    RuntimeResolveOutput {
        resolution: NodeRuntimeResolution {
            resolution_state: NodeToolchainResolutionState::Resolved,
            winning_source: Some(winner.source_kind),
            resolved_requirement: Some(winner.value.clone()),
            fallback_path: Some(fallback.clone()),
        },
        cards: {
            let mut cards = cards_for_runtime_candidates(
                &candidates,
                Some(&winner),
                NodeToolchainProvenanceDisposition::Winning,
                None,
            );
            cards.push(fallback_card(
                NodeToolchainSubject::NodeRuntime,
                &fallback,
                "Fallback Node path is visible if the winning source cannot be activated.",
            ));
            cards
        },
        ambiguities: Vec::new(),
    }
}

fn resolve_package_manager(
    mut candidates: Vec<PackageManagerCandidate>,
    config: &NodeToolchainDetectorConfig,
    runtime_state: NodeToolchainResolutionState,
) -> PackageManagerResolveOutput {
    candidates.sort_by_key(|candidate| (candidate.precedence_group, candidate.source_order));
    if candidates.is_empty() {
        let fallback = package_manager_fallback(None, config, runtime_state);
        let resolution_state = if runtime_state == NodeToolchainResolutionState::Missing {
            NodeToolchainResolutionState::Missing
        } else {
            NodeToolchainResolutionState::Fallback
        };
        return PackageManagerResolveOutput {
            resolution: NodePackageManagerResolution {
                resolution_state,
                winning_source: None,
                kind: if resolution_state == NodeToolchainResolutionState::Fallback {
                    Some(NodePackageManagerKind::Npm)
                } else {
                    None
                },
                version: config.ambient_npm_version.clone(),
                fallback_path: Some(fallback.clone()),
            },
            cards: vec![fallback_card(
                NodeToolchainSubject::PackageManager,
                &fallback,
                "No package-manager pin resolved; npm fallback remains visible before launch.",
            )],
            ambiguities: Vec::new(),
        };
    }

    let winning_group = candidates[0].precedence_group;
    let same_group = candidates
        .iter()
        .filter(|candidate| candidate.precedence_group == winning_group)
        .collect::<Vec<_>>();
    let distinct_values = distinct_package_values(&same_group);
    if distinct_values.len() > 1 {
        let fallback = package_manager_fallback(None, config, runtime_state);
        let ambiguity = ambiguity_for_package_manager(&same_group, distinct_values);
        let mut cards = cards_for_package_candidates(
            &candidates,
            None,
            NodeToolchainProvenanceDisposition::Ambiguous,
            Some(winning_group),
        );
        cards.push(fallback_card(
            NodeToolchainSubject::PackageManager,
            &fallback,
            "Package-manager ambiguity requires review before launch; fallback is inspect-only.",
        ));
        return PackageManagerResolveOutput {
            resolution: NodePackageManagerResolution {
                resolution_state: NodeToolchainResolutionState::Ambiguous,
                winning_source: None,
                kind: None,
                version: None,
                fallback_path: Some(fallback),
            },
            cards,
            ambiguities: vec![ambiguity],
        };
    }

    let winner = candidates[0].clone();
    let fallback = package_manager_fallback(Some(winner.requirement.kind), config, runtime_state);
    let resolution_state = if winner.requirement.kind.is_launch_wedge_supported() {
        NodeToolchainResolutionState::Resolved
    } else {
        NodeToolchainResolutionState::Unsupported
    };
    PackageManagerResolveOutput {
        resolution: NodePackageManagerResolution {
            resolution_state,
            winning_source: Some(winner.source_kind),
            kind: Some(winner.requirement.kind),
            version: winner.requirement.version.clone(),
            fallback_path: Some(fallback.clone()),
        },
        cards: {
            let winner_disposition =
                if resolution_state == NodeToolchainResolutionState::Unsupported {
                    NodeToolchainProvenanceDisposition::Unsupported
                } else {
                    NodeToolchainProvenanceDisposition::Winning
                };
            let mut cards =
                cards_for_package_candidates(&candidates, Some(&winner), winner_disposition, None);
            cards.push(fallback_card(
                NodeToolchainSubject::PackageManager,
                &fallback,
                "Fallback package-manager path is visible if the winning source cannot be activated.",
            ));
            cards
        },
        ambiguities: Vec::new(),
    }
}

fn runtime_candidate(
    source_kind: NodeToolchainSourceKind,
    source_ref: Option<&str>,
    value: String,
    precedence_group: u8,
    source_order: u8,
) -> RuntimeCandidate {
    RuntimeCandidate {
        source_kind,
        source_ref: source_ref.map(str::to_owned),
        value,
        precedence_group,
        source_order,
    }
}

fn package_candidate(
    source_kind: NodeToolchainSourceKind,
    source_ref: Option<&str>,
    requirement: NodePackageManagerRequirement,
    precedence_group: u8,
    source_order: u8,
) -> PackageManagerCandidate {
    PackageManagerCandidate {
        source_kind,
        source_ref: source_ref.map(str::to_owned),
        requirement,
        precedence_group,
        source_order,
    }
}

fn runtime_fallback(config: &NodeToolchainDetectorConfig) -> NodeToolchainFallbackPath {
    if let Some(version) = config
        .ambient_node_version
        .as_deref()
        .and_then(normalize_version)
    {
        NodeToolchainFallbackPath {
            subject: NodeToolchainSubject::NodeRuntime,
            source_kind: NodeToolchainSourceKind::AmbientPath,
            value_token: format!("node@{version}"),
            summary: "Captured ambient Node version from host baseline.".to_owned(),
        }
    } else {
        NodeToolchainFallbackPath {
            subject: NodeToolchainSubject::NodeRuntime,
            source_kind: NodeToolchainSourceKind::DetectorFallback,
            value_token: "node@unresolved".to_owned(),
            summary: "No Node fallback is known; Project Doctor should repair or select a runtime."
                .to_owned(),
        }
    }
}

fn package_manager_fallback(
    preferred: Option<NodePackageManagerKind>,
    config: &NodeToolchainDetectorConfig,
    runtime_state: NodeToolchainResolutionState,
) -> NodeToolchainFallbackPath {
    if runtime_state == NodeToolchainResolutionState::Missing {
        return NodeToolchainFallbackPath {
            subject: NodeToolchainSubject::PackageManager,
            source_kind: NodeToolchainSourceKind::DetectorFallback,
            value_token: "package_manager@unresolved".to_owned(),
            summary: "Package-manager fallback is blocked until Node runtime resolves.".to_owned(),
        };
    }
    match preferred {
        Some(NodePackageManagerKind::Pnpm) => {
            if let Some(version) = config
                .ambient_pnpm_version
                .as_deref()
                .and_then(normalize_version)
            {
                return NodeToolchainFallbackPath {
                    subject: NodeToolchainSubject::PackageManager,
                    source_kind: NodeToolchainSourceKind::AmbientPath,
                    value_token: format!("pnpm@{version}"),
                    summary: "Captured ambient pnpm version from host baseline.".to_owned(),
                };
            }
        }
        Some(NodePackageManagerKind::Yarn) => {
            if let Some(version) = config
                .ambient_yarn_version
                .as_deref()
                .and_then(normalize_version)
            {
                return NodeToolchainFallbackPath {
                    subject: NodeToolchainSubject::PackageManager,
                    source_kind: NodeToolchainSourceKind::AmbientPath,
                    value_token: format!("yarn@{version}"),
                    summary: "Captured ambient Yarn version from host baseline.".to_owned(),
                };
            }
        }
        Some(NodePackageManagerKind::Npm) | None => {
            if let Some(version) = config
                .ambient_npm_version
                .as_deref()
                .and_then(normalize_version)
            {
                return NodeToolchainFallbackPath {
                    subject: NodeToolchainSubject::PackageManager,
                    source_kind: NodeToolchainSourceKind::AmbientPath,
                    value_token: format!("npm@{version}"),
                    summary: "Captured ambient npm version from host baseline.".to_owned(),
                };
            }
        }
        Some(NodePackageManagerKind::Bun | NodePackageManagerKind::Unknown) => {}
    }
    NodeToolchainFallbackPath {
        subject: NodeToolchainSubject::PackageManager,
        source_kind: NodeToolchainSourceKind::DetectorFallback,
        value_token: "npm".to_owned(),
        summary: "npm is the visible detector fallback for unpinned TS/JS launch-wedge repos."
            .to_owned(),
    }
}

fn cards_for_runtime_candidates(
    candidates: &[RuntimeCandidate],
    winner: Option<&RuntimeCandidate>,
    winner_disposition: NodeToolchainProvenanceDisposition,
    ambiguous_group: Option<u8>,
) -> Vec<NodeToolchainProvenanceCard> {
    candidates
        .iter()
        .enumerate()
        .map(|(idx, candidate)| {
            let disposition = if ambiguous_group == Some(candidate.precedence_group) {
                NodeToolchainProvenanceDisposition::Ambiguous
            } else if winner
                .map(|winner| same_runtime_candidate(candidate, winner))
                .unwrap_or(false)
            {
                winner_disposition
            } else if let Some(winner) = winner {
                if candidate.value == winner.value {
                    NodeToolchainProvenanceDisposition::Corroborating
                } else if candidate.precedence_group > winner.precedence_group {
                    NodeToolchainProvenanceDisposition::Conflicting
                } else {
                    NodeToolchainProvenanceDisposition::Ignored
                }
            } else {
                NodeToolchainProvenanceDisposition::Ignored
            };
            NodeToolchainProvenanceCard {
                card_id: card_id(
                    NodeToolchainSubject::NodeRuntime,
                    candidate.source_kind,
                    idx,
                ),
                subject: NodeToolchainSubject::NodeRuntime,
                source_kind: candidate.source_kind,
                source_ref: candidate.source_ref.clone(),
                disposition,
                value_token: Some(format!("node@{}", candidate.value)),
                summary: format!(
                    "{} provided Node requirement `{}`.",
                    source_label(candidate.source_kind),
                    candidate.value
                ),
            }
        })
        .collect()
}

fn cards_for_package_candidates(
    candidates: &[PackageManagerCandidate],
    winner: Option<&PackageManagerCandidate>,
    winner_disposition: NodeToolchainProvenanceDisposition,
    ambiguous_group: Option<u8>,
) -> Vec<NodeToolchainProvenanceCard> {
    candidates
        .iter()
        .enumerate()
        .map(|(idx, candidate)| {
            let disposition = if ambiguous_group == Some(candidate.precedence_group) {
                NodeToolchainProvenanceDisposition::Ambiguous
            } else if winner
                .map(|winner| same_package_candidate(candidate, winner))
                .unwrap_or(false)
            {
                winner_disposition
            } else if let Some(winner) = winner {
                if candidate.requirement.value_token() == winner.requirement.value_token() {
                    NodeToolchainProvenanceDisposition::Corroborating
                } else if candidate.precedence_group > winner.precedence_group {
                    NodeToolchainProvenanceDisposition::Conflicting
                } else {
                    NodeToolchainProvenanceDisposition::Ignored
                }
            } else {
                NodeToolchainProvenanceDisposition::Ignored
            };
            NodeToolchainProvenanceCard {
                card_id: card_id(
                    NodeToolchainSubject::PackageManager,
                    candidate.source_kind,
                    idx,
                ),
                subject: NodeToolchainSubject::PackageManager,
                source_kind: candidate.source_kind,
                source_ref: candidate.source_ref.clone(),
                disposition,
                value_token: Some(candidate.requirement.value_token()),
                summary: format!(
                    "{} provided package-manager requirement `{}`.",
                    source_label(candidate.source_kind),
                    candidate.requirement.value_token()
                ),
            }
        })
        .collect()
}

fn fallback_card(
    subject: NodeToolchainSubject,
    fallback: &NodeToolchainFallbackPath,
    summary: &str,
) -> NodeToolchainProvenanceCard {
    NodeToolchainProvenanceCard {
        card_id: card_id(subject, fallback.source_kind, 999),
        subject,
        source_kind: fallback.source_kind,
        source_ref: None,
        disposition: NodeToolchainProvenanceDisposition::Fallback,
        value_token: Some(fallback.value_token.clone()),
        summary: summary.to_owned(),
    }
}

fn ambiguity_for_runtime(
    candidates: &[&RuntimeCandidate],
    candidate_values: Vec<String>,
) -> NodeToolchainAmbiguity {
    NodeToolchainAmbiguity {
        subject: NodeToolchainSubject::NodeRuntime,
        candidate_values,
        source_refs: source_refs_for_runtime(candidates),
        resolution_hint:
            "Pick one Node version source or align the repo version files before launching tasks."
                .to_owned(),
    }
}

fn ambiguity_for_package_manager(
    candidates: &[&PackageManagerCandidate],
    candidate_values: Vec<String>,
) -> NodeToolchainAmbiguity {
    NodeToolchainAmbiguity {
        subject: NodeToolchainSubject::PackageManager,
        candidate_values,
        source_refs: source_refs_for_package(candidates),
        resolution_hint:
            "Pick one package manager or remove conflicting lockfiles before launching tasks."
                .to_owned(),
    }
}

fn distinct_runtime_values(candidates: &[&RuntimeCandidate]) -> Vec<String> {
    let set = candidates
        .iter()
        .map(|candidate| format!("node@{}", candidate.value))
        .collect::<BTreeSet<_>>();
    set.into_iter().collect()
}

fn distinct_package_values(candidates: &[&PackageManagerCandidate]) -> Vec<String> {
    let set = candidates
        .iter()
        .map(|candidate| candidate.requirement.value_token())
        .collect::<BTreeSet<_>>();
    set.into_iter().collect()
}

fn source_refs_for_runtime(candidates: &[&RuntimeCandidate]) -> Vec<String> {
    candidates
        .iter()
        .map(|candidate| {
            candidate
                .source_ref
                .clone()
                .unwrap_or_else(|| candidate.source_kind.as_str().to_owned())
        })
        .collect()
}

fn source_refs_for_package(candidates: &[&PackageManagerCandidate]) -> Vec<String> {
    candidates
        .iter()
        .map(|candidate| {
            candidate
                .source_ref
                .clone()
                .unwrap_or_else(|| candidate.source_kind.as_str().to_owned())
        })
        .collect()
}

fn same_runtime_candidate(left: &RuntimeCandidate, right: &RuntimeCandidate) -> bool {
    left.source_kind == right.source_kind
        && left.source_ref == right.source_ref
        && left.value == right.value
}

fn same_package_candidate(left: &PackageManagerCandidate, right: &PackageManagerCandidate) -> bool {
    left.source_kind == right.source_kind
        && left.source_ref == right.source_ref
        && left.requirement == right.requirement
}

fn card_id(subject: NodeToolchainSubject, source: NodeToolchainSourceKind, index: usize) -> String {
    format!(
        "node_detector.card.{}.{}.{}",
        subject.as_str(),
        source.as_str(),
        index
    )
}

fn source_label(source: NodeToolchainSourceKind) -> &'static str {
    match source {
        NodeToolchainSourceKind::ExplicitOverride => "Explicit override",
        NodeToolchainSourceKind::PackageJsonPackageManager => "package.json packageManager",
        NodeToolchainSourceKind::PackageJsonEngines => "package.json engines",
        NodeToolchainSourceKind::PackageJsonVolta => "package.json Volta pin",
        NodeToolchainSourceKind::Nvmrc => ".nvmrc",
        NodeToolchainSourceKind::NodeVersionFile => ".node-version",
        NodeToolchainSourceKind::ToolVersions => ".tool-versions",
        NodeToolchainSourceKind::MiseToml => "mise.toml",
        NodeToolchainSourceKind::PnpmLockfile => "pnpm lockfile",
        NodeToolchainSourceKind::YarnLockfile => "Yarn lockfile",
        NodeToolchainSourceKind::NpmLockfile => "npm lockfile",
        NodeToolchainSourceKind::UserProfileDefault => "User/profile default",
        NodeToolchainSourceKind::AmbientPath => "Ambient PATH",
        NodeToolchainSourceKind::DetectorFallback => "Detector fallback",
        NodeToolchainSourceKind::UnreadableSource => "Unreadable source",
    }
}

fn json_string<'a>(package_json: &'a Option<Value>, path: &[&str]) -> Option<&'a str> {
    let mut current = package_json.as_ref()?;
    for segment in path {
        current = current.get(*segment)?;
    }
    current.as_str()
}

fn read_first_line(workspace_root: &Path, rel: &str) -> Option<String> {
    let payload = fs::read_to_string(workspace_root.join(rel)).ok()?;
    payload
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_owned)
}

fn read_tool_versions(workspace_root: &Path, tool: &str) -> Option<String> {
    let payload = fs::read_to_string(workspace_root.join(".tool-versions")).ok()?;
    for line in payload.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(name) = parts.next() else {
            continue;
        };
        if name == tool {
            return parts.next().map(str::to_owned);
        }
    }
    None
}

fn read_mise_tool(workspace_root: &Path, tools: &[&str]) -> Option<String> {
    let payload = fs::read_to_string(workspace_root.join("mise.toml")).ok()?;
    let mut in_tools = false;
    for raw_line in payload.lines() {
        let line = raw_line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            in_tools = line == "[tools]";
            continue;
        }
        if !in_tools {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().trim_matches('"').trim_matches('\'');
        if tools.contains(&key) {
            return Some(value.trim().trim_matches('"').trim_matches('\'').to_owned());
        }
    }
    None
}

fn normalize_version(raw: impl AsRef<str>) -> Option<String> {
    let value = raw.as_ref().trim();
    if value.is_empty() {
        return None;
    }
    Some(value.strip_prefix('v').unwrap_or(value).to_owned())
}

fn parse_package_manager_requirement(raw: &str) -> Option<NodePackageManagerRequirement> {
    let value = raw.trim();
    if value.is_empty() {
        return None;
    }
    let (name, version) = match value.rsplit_once('@') {
        Some((name, version)) if !name.is_empty() => (name, normalize_version(version)),
        _ => (value, None),
    };
    let kind = match name {
        "npm" => NodePackageManagerKind::Npm,
        "pnpm" => NodePackageManagerKind::Pnpm,
        "yarn" => NodePackageManagerKind::Yarn,
        "bun" => NodePackageManagerKind::Bun,
        _ => NodePackageManagerKind::Unknown,
    };
    Some(NodePackageManagerRequirement::new(kind, version))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_root(name: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/node_detection_alpha")
            .join(name)
    }

    fn detector() -> NodeToolchainDetector {
        NodeToolchainDetector::new(NodeToolchainDetectorConfig {
            ambient_node_version: Some("22.11.0".to_owned()),
            ambient_npm_version: Some("10.9.0".to_owned()),
            ambient_pnpm_version: Some("9.15.4".to_owned()),
            ambient_yarn_version: Some("1.22.22".to_owned()),
            ..NodeToolchainDetectorConfig::default()
        })
    }

    #[test]
    fn package_json_package_manager_wins_over_conflicting_lockfile() {
        let report =
            detector().detect_workspace(&fixture_root("pnpm_package_manager_wins"), "mono:0");

        assert_eq!(report.record_kind, NODE_TOOLCHAIN_DETECTION_RECORD_KIND);
        assert_eq!(
            report.node_runtime.resolution_state,
            NodeToolchainResolutionState::Resolved
        );
        assert_eq!(
            report.node_runtime.winning_source,
            Some(NodeToolchainSourceKind::PackageJsonVolta)
        );
        assert_eq!(
            report.package_manager.resolution_state,
            NodeToolchainResolutionState::Resolved
        );
        assert_eq!(
            report.package_manager.winning_source,
            Some(NodeToolchainSourceKind::PackageJsonPackageManager)
        );
        assert_eq!(
            report.package_manager.kind,
            Some(NodePackageManagerKind::Pnpm)
        );
        assert!(report.provenance_cards.iter().any(|card| card.source_kind
            == NodeToolchainSourceKind::NpmLockfile
            && card.disposition == NodeToolchainProvenanceDisposition::Conflicting));
        assert!(!report.has_unresolved_ambiguity());
        assert!(report.has_fallback());
    }

    #[test]
    fn same_precedence_lockfiles_remain_ambiguous() {
        let report = detector().detect_workspace(&fixture_root("ambiguous_lockfiles"), "mono:0");

        assert_eq!(
            report.package_manager.resolution_state,
            NodeToolchainResolutionState::Ambiguous
        );
        assert!(report.has_unresolved_ambiguity());
        assert!(report.unresolved_ambiguities.iter().any(|ambiguity| {
            ambiguity.subject == NodeToolchainSubject::PackageManager
                && ambiguity.candidate_values.contains(&"npm".to_owned())
                && ambiguity.candidate_values.contains(&"pnpm".to_owned())
        }));
        assert!(report
            .provenance_cards
            .iter()
            .any(|card| card.disposition == NodeToolchainProvenanceDisposition::Ambiguous));
    }

    #[test]
    fn unpinned_workspace_uses_visible_npm_fallback() {
        let report = detector().detect_workspace(&fixture_root("fallback_npm"), "mono:0");

        assert_eq!(
            report.node_runtime.resolution_state,
            NodeToolchainResolutionState::Resolved
        );
        assert_eq!(
            report.node_runtime.winning_source,
            Some(NodeToolchainSourceKind::AmbientPath)
        );
        assert_eq!(
            report.package_manager.resolution_state,
            NodeToolchainResolutionState::Fallback
        );
        assert_eq!(
            report.package_manager.kind,
            Some(NodePackageManagerKind::Npm)
        );
        assert_eq!(
            report
                .package_manager
                .fallback_path
                .as_ref()
                .map(|fallback| fallback.value_token.as_str()),
            Some("npm@10.9.0")
        );
    }

    #[test]
    fn conflicting_node_version_files_block_runtime_winner() {
        let report = detector().detect_workspace(&fixture_root("ambiguous_node_runtime"), "mono:0");

        assert_eq!(
            report.node_runtime.resolution_state,
            NodeToolchainResolutionState::Ambiguous
        );
        assert!(report
            .unresolved_ambiguities
            .iter()
            .any(|ambiguity| ambiguity.subject == NodeToolchainSubject::NodeRuntime));
    }

    #[test]
    fn package_manager_parser_supports_launch_wedge_managers() {
        let pnpm = parse_package_manager_requirement("pnpm@9.15.4").expect("pnpm parses");
        assert_eq!(pnpm.kind, NodePackageManagerKind::Pnpm);
        assert_eq!(pnpm.version.as_deref(), Some("9.15.4"));

        let yarn = parse_package_manager_requirement("yarn@1.22.22").expect("yarn parses");
        assert_eq!(yarn.kind, NodePackageManagerKind::Yarn);
        assert_eq!(yarn.version.as_deref(), Some("1.22.22"));

        let npm = parse_package_manager_requirement("npm@10.9.0").expect("npm parses");
        assert_eq!(npm.kind, NodePackageManagerKind::Npm);
        assert_eq!(npm.version.as_deref(), Some("10.9.0"));
    }
}
