//! Python interpreter and environment-manager detector for launch-wedge workspaces.
//!
//! The detector is read-only. It inspects repository-owned manifests, version
//! files, and local environment markers, combines them with caller-provided
//! ambient facts, and emits provenance cards that explain which interpreter
//! and manager Aureline would use before task, test, debug, or navigation
//! flows depend on that environment.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag emitted by [`PythonEnvironmentDetection`].
pub const PYTHON_ENVIRONMENT_DETECTION_RECORD_KIND: &str = "python_environment_detection_record";

/// Schema version for [`PythonEnvironmentDetection`] payloads.
pub const PYTHON_ENVIRONMENT_DETECTION_SCHEMA_VERSION: u32 = 1;

/// Detector implementation version recorded on every report.
pub const PYTHON_ENVIRONMENT_DETECTOR_VERSION: &str = "python.detector.alpha.v1";

/// Subject a Python detector source contributes to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonEnvironmentSubject {
    /// Python interpreter selection.
    Interpreter,
    /// Environment manager or activation lane selection.
    EnvironmentManager,
}

impl PythonEnvironmentSubject {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Interpreter => "interpreter",
            Self::EnvironmentManager => "environment_manager",
        }
    }
}

/// Source kind observed by the Python detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonEnvironmentSourceKind {
    /// Caller supplied an action-local override.
    ExplicitOverride,
    /// Workspace `.venv/pyvenv.cfg`.
    VenvPyvenvCfg,
    /// Workspace `.venv` directory without a readable `pyvenv.cfg`.
    VenvDirectory,
    /// `.python-version`.
    PythonVersionFile,
    /// `.tool-versions`.
    ToolVersions,
    /// `mise.toml`.
    MiseToml,
    /// `pyproject.toml#project.requires-python`.
    PyprojectRequiresPython,
    /// `pyproject.toml#tool.poetry.dependencies.python`.
    PyprojectPoetryDependency,
    /// `uv.lock`.
    UvLockfile,
    /// `pyproject.toml#tool.uv`.
    PyprojectUv,
    /// `poetry.lock`.
    PoetryLockfile,
    /// `pyproject.toml#tool.poetry`.
    PyprojectPoetry,
    /// `environment.yml` or `environment.yaml`.
    CondaEnvironmentFile,
    /// User or profile default supplied by the caller.
    UserProfileDefault,
    /// Host PATH or captured ambient interpreter fact supplied by the caller.
    AmbientPath,
    /// Detector fallback used when no authoritative source settled a value.
    DetectorFallback,
    /// Source file was present but unreadable or unparsable.
    UnreadableSource,
}

impl PythonEnvironmentSourceKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitOverride => "explicit_override",
            Self::VenvPyvenvCfg => "venv_pyvenv_cfg",
            Self::VenvDirectory => "venv_directory",
            Self::PythonVersionFile => "python_version_file",
            Self::ToolVersions => "tool_versions",
            Self::MiseToml => "mise_toml",
            Self::PyprojectRequiresPython => "pyproject_requires_python",
            Self::PyprojectPoetryDependency => "pyproject_poetry_dependency",
            Self::UvLockfile => "uv_lockfile",
            Self::PyprojectUv => "pyproject_uv",
            Self::PoetryLockfile => "poetry_lockfile",
            Self::PyprojectPoetry => "pyproject_poetry",
            Self::CondaEnvironmentFile => "conda_environment_file",
            Self::UserProfileDefault => "user_profile_default",
            Self::AmbientPath => "ambient_path",
            Self::DetectorFallback => "detector_fallback",
            Self::UnreadableSource => "unreadable_source",
        }
    }
}

/// Python environment-manager kind understood by the launch-wedge detector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonEnvironmentManagerKind {
    /// `uv`.
    Uv,
    /// A local virtual environment, usually `.venv`.
    Venv,
    /// Poetry-managed environment.
    Poetry,
    /// Conda environment, detected but outside this alpha launch-wedge lane.
    Conda,
    /// A manager token the alpha detector cannot classify.
    Unknown,
}

impl PythonEnvironmentManagerKind {
    /// Stable string token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Uv => "uv",
            Self::Venv => "venv",
            Self::Poetry => "poetry",
            Self::Conda => "conda",
            Self::Unknown => "unknown",
        }
    }

    /// True when this manager is in the Python launch wedge.
    pub const fn is_launch_wedge_supported(self) -> bool {
        matches!(self, Self::Uv | Self::Venv | Self::Poetry)
    }
}

/// Version or location requirement for a Python environment manager.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonEnvironmentManagerRequirement {
    /// Environment-manager family.
    pub kind: PythonEnvironmentManagerKind,
    /// Version token when the source supplied one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Workspace-relative or host-provided environment reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_ref: Option<String>,
}

impl PythonEnvironmentManagerRequirement {
    /// Constructs a new environment-manager requirement.
    pub fn new(
        kind: PythonEnvironmentManagerKind,
        version: Option<String>,
        environment_ref: Option<String>,
    ) -> Self {
        Self {
            kind,
            version,
            environment_ref,
        }
    }

    /// Stable value token used in provenance cards.
    pub fn value_token(&self) -> String {
        let base = match &self.version {
            Some(version) if !version.is_empty() => {
                format!("{}@{}", self.kind.as_str(), version)
            }
            _ => self.kind.as_str().to_owned(),
        };
        match &self.environment_ref {
            Some(environment_ref) if !environment_ref.is_empty() => {
                format!("{base} ({environment_ref})")
            }
            _ => base,
        }
    }
}

/// Resolution state for a Python interpreter or manager selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PythonEnvironmentResolutionState {
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

impl PythonEnvironmentResolutionState {
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
pub enum PythonEnvironmentProvenanceDisposition {
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
    /// This source names an environment outside the launch-wedge contract.
    Unsupported,
}

impl PythonEnvironmentProvenanceDisposition {
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
pub struct PythonEnvironmentFallbackPath {
    /// Subject the fallback applies to.
    pub subject: PythonEnvironmentSubject,
    /// Source that produced the fallback.
    pub source_kind: PythonEnvironmentSourceKind,
    /// Stable token for the fallback value.
    pub value_token: String,
    /// Human-readable summary suitable for inspectors and support exports.
    pub summary: String,
}

/// Interpreter selection produced by the Python detector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonInterpreterResolution {
    /// Final resolution state.
    pub resolution_state: PythonEnvironmentResolutionState,
    /// Winning source, when one source safely won.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_source: Option<PythonEnvironmentSourceKind>,
    /// Resolved Python version or range token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_requirement: Option<String>,
    /// Interpreter path or workspace-relative interpreter reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interpreter_ref: Option<String>,
    /// Visible fallback path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_path: Option<PythonEnvironmentFallbackPath>,
}

/// Environment-manager selection produced by the Python detector.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonEnvironmentManagerResolution {
    /// Final resolution state.
    pub resolution_state: PythonEnvironmentResolutionState,
    /// Winning source, when one source safely won.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub winning_source: Option<PythonEnvironmentSourceKind>,
    /// Resolved manager kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<PythonEnvironmentManagerKind>,
    /// Version token when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Workspace-relative environment reference when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_ref: Option<String>,
    /// Visible fallback path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fallback_path: Option<PythonEnvironmentFallbackPath>,
}

/// One detector provenance card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonEnvironmentProvenanceCard {
    /// Stable card id inside the detector report.
    pub card_id: String,
    /// Interpreter or manager subject.
    pub subject: PythonEnvironmentSubject,
    /// Source kind that produced the card.
    pub source_kind: PythonEnvironmentSourceKind,
    /// Workspace-relative source reference, when the source is a file.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Card disposition after precedence was applied.
    pub disposition: PythonEnvironmentProvenanceDisposition,
    /// Stable value token, when this card carries a value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value_token: Option<String>,
    /// Human-readable explanation for inspector rows.
    pub summary: String,
}

/// Unresolved detector ambiguity that must remain visible before launch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonEnvironmentAmbiguity {
    /// Interpreter or manager subject.
    pub subject: PythonEnvironmentSubject,
    /// Conflicting value tokens.
    pub candidate_values: Vec<String>,
    /// Source refs or source-kind tokens that produced the conflict.
    pub source_refs: Vec<String>,
    /// Reviewable hint for a repair or manual selection flow.
    pub resolution_hint: String,
}

/// Complete read-only Python environment detector report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PythonEnvironmentDetection {
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
    /// Python interpreter resolution.
    pub interpreter: PythonInterpreterResolution,
    /// Environment-manager resolution.
    pub environment_manager: PythonEnvironmentManagerResolution,
    /// Ordered provenance cards.
    pub provenance_cards: Vec<PythonEnvironmentProvenanceCard>,
    /// Unresolved ambiguities that must be surfaced before launch.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unresolved_ambiguities: Vec<PythonEnvironmentAmbiguity>,
}

impl PythonEnvironmentDetection {
    /// True when any subject fell back to a lower-confidence path.
    pub fn has_fallback(&self) -> bool {
        self.interpreter.fallback_path.is_some()
            || self.environment_manager.fallback_path.is_some()
            || self
                .provenance_cards
                .iter()
                .any(|card| card.disposition == PythonEnvironmentProvenanceDisposition::Fallback)
    }

    /// True when a same-precedence ambiguity was left unresolved.
    pub fn has_unresolved_ambiguity(&self) -> bool {
        !self.unresolved_ambiguities.is_empty()
            || self.interpreter.resolution_state == PythonEnvironmentResolutionState::Ambiguous
            || self.environment_manager.resolution_state
                == PythonEnvironmentResolutionState::Ambiguous
    }

    /// True when an expected source existed but could not be inspected.
    pub fn has_detector_failure(&self) -> bool {
        self.provenance_cards
            .iter()
            .any(|card| card.source_kind == PythonEnvironmentSourceKind::UnreadableSource)
    }
}

/// Caller-provided facts and overrides for [`PythonEnvironmentDetector`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct PythonEnvironmentDetectorConfig {
    /// Action-local interpreter path override.
    pub explicit_interpreter_ref: Option<String>,
    /// Action-local Python version or range override.
    pub explicit_python_version: Option<String>,
    /// Action-local environment-manager override.
    pub explicit_environment_manager: Option<PythonEnvironmentManagerRequirement>,
    /// User/profile default interpreter path.
    pub profile_interpreter_ref: Option<String>,
    /// User/profile default Python version or range.
    pub profile_python_version: Option<String>,
    /// User/profile default manager.
    pub profile_environment_manager: Option<PythonEnvironmentManagerRequirement>,
    /// Captured ambient `python --version` fact, if a host probe already has it.
    pub ambient_python_version: Option<String>,
    /// Captured ambient interpreter path, if a host probe already has it.
    pub ambient_interpreter_ref: Option<String>,
    /// Captured ambient `uv --version` fact, if a host probe already has it.
    pub ambient_uv_version: Option<String>,
    /// Captured ambient `poetry --version` fact, if a host probe already has it.
    pub ambient_poetry_version: Option<String>,
}

/// Read-only Python environment detector.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PythonEnvironmentDetector {
    config: PythonEnvironmentDetectorConfig,
}

impl PythonEnvironmentDetector {
    /// Creates a detector with caller-provided facts and overrides.
    pub fn new(config: PythonEnvironmentDetectorConfig) -> Self {
        Self { config }
    }

    /// Creates a detector with no caller-provided facts.
    pub fn default_read_only() -> Self {
        Self::new(PythonEnvironmentDetectorConfig::default())
    }

    /// Detects interpreter and manager selections for one workspace root.
    ///
    /// The method reads known manifest and version files but never executes
    /// repository-owned hooks, Python, `uv`, or Poetry.
    pub fn detect_workspace(
        &self,
        workspace_root: &Path,
        detected_at: &str,
    ) -> PythonEnvironmentDetection {
        let workspace_root_ref = workspace_root.display().to_string();
        let pyproject = read_pyproject(workspace_root);
        let pyvenv_cfg = read_pyvenv_cfg(workspace_root);

        let mut unreadable_cards = Vec::new();
        if let Some(summary) = pyproject.error {
            unreadable_cards.push(PythonEnvironmentProvenanceCard {
                card_id: "python_detector.card.unreadable.pyproject".to_owned(),
                subject: PythonEnvironmentSubject::Interpreter,
                source_kind: PythonEnvironmentSourceKind::UnreadableSource,
                source_ref: Some("pyproject.toml".to_owned()),
                disposition: PythonEnvironmentProvenanceDisposition::Unsupported,
                value_token: None,
                summary,
            });
        }
        if let Some(summary) = pyvenv_cfg.error {
            unreadable_cards.push(PythonEnvironmentProvenanceCard {
                card_id: "python_detector.card.unreadable.pyvenv_cfg".to_owned(),
                subject: PythonEnvironmentSubject::Interpreter,
                source_kind: PythonEnvironmentSourceKind::UnreadableSource,
                source_ref: Some(".venv/pyvenv.cfg".to_owned()),
                disposition: PythonEnvironmentProvenanceDisposition::Unsupported,
                value_token: None,
                summary,
            });
        }

        let interpreter_candidates = collect_interpreter_candidates(
            workspace_root,
            &self.config,
            &pyproject.value,
            &pyvenv_cfg.value,
        );
        let manager_candidates = collect_manager_candidates(
            workspace_root,
            &self.config,
            &pyproject.value,
            &pyvenv_cfg.value,
        );

        let interpreter = resolve_interpreter(interpreter_candidates, &self.config);
        let manager = resolve_environment_manager(
            manager_candidates,
            &self.config,
            interpreter.resolution.resolution_state,
        );

        let mut provenance_cards = Vec::new();
        provenance_cards.extend(interpreter.cards);
        provenance_cards.extend(manager.cards);
        provenance_cards.append(&mut unreadable_cards);
        provenance_cards.sort_by(|left, right| left.card_id.cmp(&right.card_id));

        let mut unresolved_ambiguities = interpreter.ambiguities;
        unresolved_ambiguities.extend(manager.ambiguities);

        PythonEnvironmentDetection {
            record_kind: PYTHON_ENVIRONMENT_DETECTION_RECORD_KIND.to_owned(),
            schema_version: PYTHON_ENVIRONMENT_DETECTION_SCHEMA_VERSION,
            detector_version: PYTHON_ENVIRONMENT_DETECTOR_VERSION.to_owned(),
            workspace_root_ref,
            detected_at: detected_at.to_owned(),
            interpreter: interpreter.resolution,
            environment_manager: manager.resolution,
            provenance_cards,
            unresolved_ambiguities,
        }
    }
}

#[derive(Debug, Clone)]
struct TomlRead {
    value: Option<SimpleToml>,
    error: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct SimpleToml {
    values: BTreeMap<String, String>,
    sections: BTreeSet<String>,
}

impl SimpleToml {
    fn value(&self, key: &str) -> Option<&str> {
        self.values.get(key).map(String::as_str)
    }

    fn has_section(&self, section: &str) -> bool {
        self.sections.contains(section)
            || self
                .values
                .keys()
                .any(|key| key.starts_with(&format!("{section}.")))
    }
}

#[derive(Debug, Clone)]
struct PyvenvCfgRead {
    value: Option<PyvenvCfg>,
    error: Option<String>,
}

#[derive(Debug, Clone, Default)]
struct PyvenvCfg {
    version: Option<String>,
    executable: Option<String>,
}

#[derive(Debug, Clone)]
struct PythonInterpreterCandidate {
    source_kind: PythonEnvironmentSourceKind,
    source_ref: Option<String>,
    version_requirement: Option<String>,
    interpreter_ref: Option<String>,
    precedence_group: u8,
    source_order: u8,
}

impl PythonInterpreterCandidate {
    fn value_token(&self) -> String {
        python_interpreter_token(
            self.version_requirement.as_deref(),
            self.interpreter_ref.as_deref(),
        )
    }

    fn comparison_token(&self) -> String {
        match &self.version_requirement {
            Some(version) if !version.is_empty() => format!("python@{version}"),
            _ => self.value_token(),
        }
    }
}

#[derive(Debug, Clone)]
struct PythonManagerCandidate {
    source_kind: PythonEnvironmentSourceKind,
    source_ref: Option<String>,
    requirement: PythonEnvironmentManagerRequirement,
    precedence_group: u8,
    source_order: u8,
}

#[derive(Debug, Clone)]
struct InterpreterResolveOutput {
    resolution: PythonInterpreterResolution,
    cards: Vec<PythonEnvironmentProvenanceCard>,
    ambiguities: Vec<PythonEnvironmentAmbiguity>,
}

#[derive(Debug, Clone)]
struct ManagerResolveOutput {
    resolution: PythonEnvironmentManagerResolution,
    cards: Vec<PythonEnvironmentProvenanceCard>,
    ambiguities: Vec<PythonEnvironmentAmbiguity>,
}

fn read_pyproject(workspace_root: &Path) -> TomlRead {
    let path = workspace_root.join("pyproject.toml");
    let payload = match fs::read_to_string(&path) {
        Ok(payload) => payload,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return TomlRead {
                value: None,
                error: None,
            };
        }
        Err(err) => {
            return TomlRead {
                value: None,
                error: Some(format!("pyproject.toml could not be read: {err}")),
            };
        }
    };
    match parse_simple_toml(
        &payload,
        &["project.requires-python", "tool.poetry.dependencies.python"],
    ) {
        Ok(value) => TomlRead {
            value: Some(value),
            error: None,
        },
        Err(err) => TomlRead {
            value: None,
            error: Some(format!("pyproject.toml could not be parsed: {err}")),
        },
    }
}

fn read_pyvenv_cfg(workspace_root: &Path) -> PyvenvCfgRead {
    let path = workspace_root.join(".venv").join("pyvenv.cfg");
    let payload = match fs::read_to_string(&path) {
        Ok(payload) => payload,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            return PyvenvCfgRead {
                value: None,
                error: None,
            };
        }
        Err(err) => {
            return PyvenvCfgRead {
                value: None,
                error: Some(format!(".venv/pyvenv.cfg could not be read: {err}")),
            };
        }
    };

    let mut cfg = PyvenvCfg::default();
    for raw_line in payload.lines() {
        let line = raw_line.split('#').next().unwrap_or_default().trim();
        if line.is_empty() {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        match key {
            "version" => cfg.version = normalize_python_version(value),
            "executable" => cfg.executable = non_empty(value),
            _ => {}
        }
    }

    PyvenvCfgRead {
        value: Some(cfg),
        error: None,
    }
}

fn collect_interpreter_candidates(
    workspace_root: &Path,
    config: &PythonEnvironmentDetectorConfig,
    pyproject: &Option<SimpleToml>,
    pyvenv_cfg: &Option<PyvenvCfg>,
) -> Vec<PythonInterpreterCandidate> {
    let mut candidates = Vec::new();
    if config.explicit_interpreter_ref.is_some() || config.explicit_python_version.is_some() {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::ExplicitOverride,
            None,
            config
                .explicit_python_version
                .as_deref()
                .and_then(normalize_python_version),
            config.explicit_interpreter_ref.clone(),
            0,
            0,
        ));
    }
    if let Some(cfg) = pyvenv_cfg {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::VenvPyvenvCfg,
            Some(".venv/pyvenv.cfg"),
            cfg.version.as_deref().and_then(normalize_python_version),
            Some(venv_interpreter_ref(workspace_root, cfg)),
            1,
            0,
        ));
    }
    if let Some(value) =
        read_first_line(workspace_root, ".python-version").and_then(normalize_python_version)
    {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::PythonVersionFile,
            Some(".python-version"),
            Some(value),
            None,
            2,
            0,
        ));
    }
    if let Some(value) =
        read_tool_versions(workspace_root, "python").and_then(normalize_python_version)
    {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::ToolVersions,
            Some(".tool-versions#python"),
            Some(value),
            None,
            2,
            1,
        ));
    }
    if let Some(value) =
        read_mise_tool(workspace_root, &["python"]).and_then(normalize_python_version)
    {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::MiseToml,
            Some("mise.toml#tools.python"),
            Some(value),
            None,
            2,
            2,
        ));
    }
    if let Some(value) =
        pyproject_value(pyproject, "project.requires-python").and_then(normalize_python_version)
    {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::PyprojectRequiresPython,
            Some("pyproject.toml#project.requires-python"),
            Some(value),
            None,
            3,
            0,
        ));
    }
    if let Some(value) = pyproject_value(pyproject, "tool.poetry.dependencies.python")
        .and_then(normalize_python_version)
    {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::PyprojectPoetryDependency,
            Some("pyproject.toml#tool.poetry.dependencies.python"),
            Some(value),
            None,
            3,
            1,
        ));
    }
    if config.profile_interpreter_ref.is_some() || config.profile_python_version.is_some() {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::UserProfileDefault,
            None,
            config
                .profile_python_version
                .as_deref()
                .and_then(normalize_python_version),
            config.profile_interpreter_ref.clone(),
            4,
            0,
        ));
    }
    if config.ambient_interpreter_ref.is_some() || config.ambient_python_version.is_some() {
        candidates.push(interpreter_candidate(
            PythonEnvironmentSourceKind::AmbientPath,
            None,
            config
                .ambient_python_version
                .as_deref()
                .and_then(normalize_python_version),
            config.ambient_interpreter_ref.clone(),
            5,
            0,
        ));
    }
    candidates
}

fn collect_manager_candidates(
    workspace_root: &Path,
    config: &PythonEnvironmentDetectorConfig,
    pyproject: &Option<SimpleToml>,
    pyvenv_cfg: &Option<PyvenvCfg>,
) -> Vec<PythonManagerCandidate> {
    let mut candidates = Vec::new();
    if let Some(requirement) = &config.explicit_environment_manager {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::ExplicitOverride,
            None,
            requirement.clone(),
            0,
            0,
        ));
    }
    if workspace_root.join("uv.lock").is_file() {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::UvLockfile,
            Some("uv.lock"),
            PythonEnvironmentManagerRequirement::new(
                PythonEnvironmentManagerKind::Uv,
                None,
                Some(".venv".to_owned()),
            ),
            1,
            0,
        ));
    }
    if pyproject
        .as_ref()
        .map(|toml| toml.has_section("tool.uv"))
        .unwrap_or(false)
    {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::PyprojectUv,
            Some("pyproject.toml#tool.uv"),
            PythonEnvironmentManagerRequirement::new(
                PythonEnvironmentManagerKind::Uv,
                None,
                Some(".venv".to_owned()),
            ),
            1,
            1,
        ));
    }
    if workspace_root.join("poetry.lock").is_file() {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::PoetryLockfile,
            Some("poetry.lock"),
            PythonEnvironmentManagerRequirement::new(
                PythonEnvironmentManagerKind::Poetry,
                None,
                None,
            ),
            1,
            2,
        ));
    }
    if pyproject
        .as_ref()
        .map(|toml| toml.has_section("tool.poetry"))
        .unwrap_or(false)
    {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::PyprojectPoetry,
            Some("pyproject.toml#tool.poetry"),
            PythonEnvironmentManagerRequirement::new(
                PythonEnvironmentManagerKind::Poetry,
                None,
                None,
            ),
            1,
            3,
        ));
    }
    if workspace_root.join("environment.yml").is_file() {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::CondaEnvironmentFile,
            Some("environment.yml"),
            PythonEnvironmentManagerRequirement::new(
                PythonEnvironmentManagerKind::Conda,
                None,
                None,
            ),
            1,
            4,
        ));
    }
    if workspace_root.join("environment.yaml").is_file() {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::CondaEnvironmentFile,
            Some("environment.yaml"),
            PythonEnvironmentManagerRequirement::new(
                PythonEnvironmentManagerKind::Conda,
                None,
                None,
            ),
            1,
            5,
        ));
    }
    if pyvenv_cfg.is_some() {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::VenvPyvenvCfg,
            Some(".venv/pyvenv.cfg"),
            PythonEnvironmentManagerRequirement::new(
                PythonEnvironmentManagerKind::Venv,
                None,
                Some(".venv".to_owned()),
            ),
            2,
            0,
        ));
    } else if workspace_root.join(".venv").is_dir() {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::VenvDirectory,
            Some(".venv"),
            PythonEnvironmentManagerRequirement::new(
                PythonEnvironmentManagerKind::Venv,
                None,
                Some(".venv".to_owned()),
            ),
            2,
            1,
        ));
    }
    if let Some(requirement) = &config.profile_environment_manager {
        candidates.push(manager_candidate(
            PythonEnvironmentSourceKind::UserProfileDefault,
            None,
            requirement.clone(),
            4,
            0,
        ));
    }
    candidates
}

fn resolve_interpreter(
    mut candidates: Vec<PythonInterpreterCandidate>,
    config: &PythonEnvironmentDetectorConfig,
) -> InterpreterResolveOutput {
    candidates.sort_by_key(|candidate| (candidate.precedence_group, candidate.source_order));
    if candidates.is_empty() {
        let fallback = interpreter_fallback(config);
        return InterpreterResolveOutput {
            resolution: PythonInterpreterResolution {
                resolution_state: PythonEnvironmentResolutionState::Missing,
                winning_source: None,
                resolved_requirement: None,
                interpreter_ref: None,
                fallback_path: Some(fallback.clone()),
            },
            cards: vec![fallback_card(
                PythonEnvironmentSubject::Interpreter,
                &fallback,
                "Python interpreter did not resolve from workspace pins; launch must select or install Python.",
            )],
            ambiguities: Vec::new(),
        };
    }

    let winning_group = candidates[0].precedence_group;
    let same_group = candidates
        .iter()
        .filter(|candidate| candidate.precedence_group == winning_group)
        .collect::<Vec<_>>();
    let distinct_values = distinct_interpreter_values(&same_group);
    if distinct_values.len() > 1 {
        let fallback = interpreter_fallback(config);
        let ambiguity = ambiguity_for_interpreter(&same_group, distinct_values);
        let mut cards = cards_for_interpreter_candidates(
            &candidates,
            None,
            PythonEnvironmentProvenanceDisposition::Ambiguous,
            Some(winning_group),
        );
        cards.push(fallback_card(
            PythonEnvironmentSubject::Interpreter,
            &fallback,
            "Python interpreter ambiguity requires review before launch; fallback is inspect-only.",
        ));
        return InterpreterResolveOutput {
            resolution: PythonInterpreterResolution {
                resolution_state: PythonEnvironmentResolutionState::Ambiguous,
                winning_source: None,
                resolved_requirement: None,
                interpreter_ref: None,
                fallback_path: Some(fallback),
            },
            cards,
            ambiguities: vec![ambiguity],
        };
    }

    let winner = candidates[0].clone();
    let fallback = interpreter_fallback(config);
    InterpreterResolveOutput {
        resolution: PythonInterpreterResolution {
            resolution_state: PythonEnvironmentResolutionState::Resolved,
            winning_source: Some(winner.source_kind),
            resolved_requirement: winner.version_requirement.clone(),
            interpreter_ref: winner.interpreter_ref.clone(),
            fallback_path: Some(fallback.clone()),
        },
        cards: {
            let mut cards = cards_for_interpreter_candidates(
                &candidates,
                Some(&winner),
                PythonEnvironmentProvenanceDisposition::Winning,
                None,
            );
            cards.push(fallback_card(
                PythonEnvironmentSubject::Interpreter,
                &fallback,
                "Fallback Python path is visible if the winning source cannot be activated.",
            ));
            cards
        },
        ambiguities: Vec::new(),
    }
}

fn resolve_environment_manager(
    mut candidates: Vec<PythonManagerCandidate>,
    config: &PythonEnvironmentDetectorConfig,
    interpreter_state: PythonEnvironmentResolutionState,
) -> ManagerResolveOutput {
    candidates.sort_by_key(|candidate| (candidate.precedence_group, candidate.source_order));
    if candidates.is_empty() {
        let fallback = manager_fallback(None, config, interpreter_state);
        let resolution_state = if interpreter_state == PythonEnvironmentResolutionState::Missing {
            PythonEnvironmentResolutionState::Missing
        } else {
            PythonEnvironmentResolutionState::Fallback
        };
        return ManagerResolveOutput {
            resolution: PythonEnvironmentManagerResolution {
                resolution_state,
                winning_source: None,
                kind: if resolution_state == PythonEnvironmentResolutionState::Fallback {
                    Some(PythonEnvironmentManagerKind::Venv)
                } else {
                    None
                },
                version: None,
                environment_ref: if resolution_state == PythonEnvironmentResolutionState::Fallback {
                    Some(".venv".to_owned())
                } else {
                    None
                },
                fallback_path: Some(fallback.clone()),
            },
            cards: vec![fallback_card(
                PythonEnvironmentSubject::EnvironmentManager,
                &fallback,
                "No Python environment manager pin resolved; venv fallback remains visible before launch.",
            )],
            ambiguities: Vec::new(),
        };
    }

    let winning_group = candidates[0].precedence_group;
    let same_group = candidates
        .iter()
        .filter(|candidate| candidate.precedence_group == winning_group)
        .collect::<Vec<_>>();
    let distinct_values = distinct_manager_values(&same_group);
    if distinct_values.len() > 1 {
        let fallback = manager_fallback(None, config, interpreter_state);
        let ambiguity = ambiguity_for_manager(&same_group, distinct_values);
        let mut cards = cards_for_manager_candidates(
            &candidates,
            None,
            PythonEnvironmentProvenanceDisposition::Ambiguous,
            Some(winning_group),
        );
        cards.push(fallback_card(
            PythonEnvironmentSubject::EnvironmentManager,
            &fallback,
            "Python manager ambiguity requires review before launch; fallback is inspect-only.",
        ));
        return ManagerResolveOutput {
            resolution: PythonEnvironmentManagerResolution {
                resolution_state: PythonEnvironmentResolutionState::Ambiguous,
                winning_source: None,
                kind: None,
                version: None,
                environment_ref: None,
                fallback_path: Some(fallback),
            },
            cards,
            ambiguities: vec![ambiguity],
        };
    }

    let winner = candidates[0].clone();
    let fallback = manager_fallback(Some(winner.requirement.kind), config, interpreter_state);
    let resolution_state = if winner.requirement.kind.is_launch_wedge_supported() {
        PythonEnvironmentResolutionState::Resolved
    } else {
        PythonEnvironmentResolutionState::Unsupported
    };
    ManagerResolveOutput {
        resolution: PythonEnvironmentManagerResolution {
            resolution_state,
            winning_source: Some(winner.source_kind),
            kind: Some(winner.requirement.kind),
            version: winner.requirement.version.clone(),
            environment_ref: winner.requirement.environment_ref.clone(),
            fallback_path: Some(fallback.clone()),
        },
        cards: {
            let winner_disposition =
                if resolution_state == PythonEnvironmentResolutionState::Unsupported {
                    PythonEnvironmentProvenanceDisposition::Unsupported
                } else {
                    PythonEnvironmentProvenanceDisposition::Winning
                };
            let mut cards =
                cards_for_manager_candidates(&candidates, Some(&winner), winner_disposition, None);
            cards.push(fallback_card(
                PythonEnvironmentSubject::EnvironmentManager,
                &fallback,
                "Fallback Python manager path is visible if the winning source cannot be activated.",
            ));
            cards
        },
        ambiguities: Vec::new(),
    }
}

fn interpreter_candidate(
    source_kind: PythonEnvironmentSourceKind,
    source_ref: Option<&str>,
    version_requirement: Option<String>,
    interpreter_ref: Option<String>,
    precedence_group: u8,
    source_order: u8,
) -> PythonInterpreterCandidate {
    PythonInterpreterCandidate {
        source_kind,
        source_ref: source_ref.map(str::to_owned),
        version_requirement,
        interpreter_ref,
        precedence_group,
        source_order,
    }
}

fn manager_candidate(
    source_kind: PythonEnvironmentSourceKind,
    source_ref: Option<&str>,
    requirement: PythonEnvironmentManagerRequirement,
    precedence_group: u8,
    source_order: u8,
) -> PythonManagerCandidate {
    PythonManagerCandidate {
        source_kind,
        source_ref: source_ref.map(str::to_owned),
        requirement,
        precedence_group,
        source_order,
    }
}

fn interpreter_fallback(config: &PythonEnvironmentDetectorConfig) -> PythonEnvironmentFallbackPath {
    if config.ambient_interpreter_ref.is_some() || config.ambient_python_version.is_some() {
        let value_token = python_interpreter_token(
            config
                .ambient_python_version
                .as_deref()
                .and_then(normalize_python_version)
                .as_deref(),
            config.ambient_interpreter_ref.as_deref(),
        );
        PythonEnvironmentFallbackPath {
            subject: PythonEnvironmentSubject::Interpreter,
            source_kind: PythonEnvironmentSourceKind::AmbientPath,
            value_token,
            summary: "Captured ambient Python interpreter from host baseline.".to_owned(),
        }
    } else {
        PythonEnvironmentFallbackPath {
            subject: PythonEnvironmentSubject::Interpreter,
            source_kind: PythonEnvironmentSourceKind::DetectorFallback,
            value_token: "python@unresolved".to_owned(),
            summary:
                "No Python fallback is known; Project Doctor should repair or select an interpreter."
                    .to_owned(),
        }
    }
}

fn manager_fallback(
    preferred: Option<PythonEnvironmentManagerKind>,
    config: &PythonEnvironmentDetectorConfig,
    interpreter_state: PythonEnvironmentResolutionState,
) -> PythonEnvironmentFallbackPath {
    if interpreter_state == PythonEnvironmentResolutionState::Missing {
        return PythonEnvironmentFallbackPath {
            subject: PythonEnvironmentSubject::EnvironmentManager,
            source_kind: PythonEnvironmentSourceKind::DetectorFallback,
            value_token: "environment_manager@unresolved".to_owned(),
            summary: "Python manager fallback is blocked until the interpreter resolves."
                .to_owned(),
        };
    }
    match preferred {
        Some(PythonEnvironmentManagerKind::Uv) => {
            if let Some(version) = config
                .ambient_uv_version
                .as_deref()
                .and_then(normalize_python_version)
            {
                return PythonEnvironmentFallbackPath {
                    subject: PythonEnvironmentSubject::EnvironmentManager,
                    source_kind: PythonEnvironmentSourceKind::AmbientPath,
                    value_token: format!("uv@{version}"),
                    summary: "Captured ambient uv version from host baseline.".to_owned(),
                };
            }
        }
        Some(PythonEnvironmentManagerKind::Poetry) => {
            if let Some(version) = config
                .ambient_poetry_version
                .as_deref()
                .and_then(normalize_python_version)
            {
                return PythonEnvironmentFallbackPath {
                    subject: PythonEnvironmentSubject::EnvironmentManager,
                    source_kind: PythonEnvironmentSourceKind::AmbientPath,
                    value_token: format!("poetry@{version}"),
                    summary: "Captured ambient Poetry version from host baseline.".to_owned(),
                };
            }
        }
        Some(PythonEnvironmentManagerKind::Venv) | None => {}
        Some(PythonEnvironmentManagerKind::Conda | PythonEnvironmentManagerKind::Unknown) => {}
    }
    PythonEnvironmentFallbackPath {
        subject: PythonEnvironmentSubject::EnvironmentManager,
        source_kind: PythonEnvironmentSourceKind::DetectorFallback,
        value_token: "venv (.venv)".to_owned(),
        summary: "venv is the visible detector fallback for unpinned Python launch-wedge repos."
            .to_owned(),
    }
}

fn cards_for_interpreter_candidates(
    candidates: &[PythonInterpreterCandidate],
    winner: Option<&PythonInterpreterCandidate>,
    winner_disposition: PythonEnvironmentProvenanceDisposition,
    ambiguous_group: Option<u8>,
) -> Vec<PythonEnvironmentProvenanceCard> {
    candidates
        .iter()
        .enumerate()
        .map(|(idx, candidate)| {
            let disposition = if ambiguous_group == Some(candidate.precedence_group) {
                PythonEnvironmentProvenanceDisposition::Ambiguous
            } else if winner
                .map(|winner| same_interpreter_candidate(candidate, winner))
                .unwrap_or(false)
            {
                winner_disposition
            } else if let Some(winner) = winner {
                if candidate.comparison_token() == winner.comparison_token() {
                    PythonEnvironmentProvenanceDisposition::Corroborating
                } else if candidate.precedence_group > winner.precedence_group {
                    PythonEnvironmentProvenanceDisposition::Conflicting
                } else {
                    PythonEnvironmentProvenanceDisposition::Ignored
                }
            } else {
                PythonEnvironmentProvenanceDisposition::Ignored
            };
            PythonEnvironmentProvenanceCard {
                card_id: card_id(
                    PythonEnvironmentSubject::Interpreter,
                    candidate.source_kind,
                    idx,
                ),
                subject: PythonEnvironmentSubject::Interpreter,
                source_kind: candidate.source_kind,
                source_ref: candidate.source_ref.clone(),
                disposition,
                value_token: Some(candidate.value_token()),
                summary: format!(
                    "{} provided Python interpreter requirement `{}`.",
                    source_label(candidate.source_kind),
                    candidate.value_token()
                ),
            }
        })
        .collect()
}

fn cards_for_manager_candidates(
    candidates: &[PythonManagerCandidate],
    winner: Option<&PythonManagerCandidate>,
    winner_disposition: PythonEnvironmentProvenanceDisposition,
    ambiguous_group: Option<u8>,
) -> Vec<PythonEnvironmentProvenanceCard> {
    candidates
        .iter()
        .enumerate()
        .map(|(idx, candidate)| {
            let disposition = if ambiguous_group == Some(candidate.precedence_group) {
                PythonEnvironmentProvenanceDisposition::Ambiguous
            } else if winner
                .map(|winner| same_manager_candidate(candidate, winner))
                .unwrap_or(false)
            {
                winner_disposition
            } else if let Some(winner) = winner {
                if candidate.requirement.value_token() == winner.requirement.value_token() {
                    PythonEnvironmentProvenanceDisposition::Corroborating
                } else if candidate.precedence_group > winner.precedence_group {
                    PythonEnvironmentProvenanceDisposition::Conflicting
                } else {
                    PythonEnvironmentProvenanceDisposition::Ignored
                }
            } else {
                PythonEnvironmentProvenanceDisposition::Ignored
            };
            PythonEnvironmentProvenanceCard {
                card_id: card_id(
                    PythonEnvironmentSubject::EnvironmentManager,
                    candidate.source_kind,
                    idx,
                ),
                subject: PythonEnvironmentSubject::EnvironmentManager,
                source_kind: candidate.source_kind,
                source_ref: candidate.source_ref.clone(),
                disposition,
                value_token: Some(candidate.requirement.value_token()),
                summary: format!(
                    "{} provided Python manager requirement `{}`.",
                    source_label(candidate.source_kind),
                    candidate.requirement.value_token()
                ),
            }
        })
        .collect()
}

fn fallback_card(
    subject: PythonEnvironmentSubject,
    fallback: &PythonEnvironmentFallbackPath,
    summary: &str,
) -> PythonEnvironmentProvenanceCard {
    PythonEnvironmentProvenanceCard {
        card_id: card_id(subject, fallback.source_kind, 999),
        subject,
        source_kind: fallback.source_kind,
        source_ref: None,
        disposition: PythonEnvironmentProvenanceDisposition::Fallback,
        value_token: Some(fallback.value_token.clone()),
        summary: summary.to_owned(),
    }
}

fn ambiguity_for_interpreter(
    candidates: &[&PythonInterpreterCandidate],
    candidate_values: Vec<String>,
) -> PythonEnvironmentAmbiguity {
    PythonEnvironmentAmbiguity {
        subject: PythonEnvironmentSubject::Interpreter,
        candidate_values,
        source_refs: source_refs_for_interpreter(candidates),
        resolution_hint:
            "Pick one Python version source or align the repo version files before launching tasks."
                .to_owned(),
    }
}

fn ambiguity_for_manager(
    candidates: &[&PythonManagerCandidate],
    candidate_values: Vec<String>,
) -> PythonEnvironmentAmbiguity {
    PythonEnvironmentAmbiguity {
        subject: PythonEnvironmentSubject::EnvironmentManager,
        candidate_values,
        source_refs: source_refs_for_manager(candidates),
        resolution_hint:
            "Pick one Python manager or remove conflicting lockfiles before launching tasks."
                .to_owned(),
    }
}

fn distinct_interpreter_values(candidates: &[&PythonInterpreterCandidate]) -> Vec<String> {
    let set = candidates
        .iter()
        .map(|candidate| candidate.comparison_token())
        .collect::<BTreeSet<_>>();
    set.into_iter().collect()
}

fn distinct_manager_values(candidates: &[&PythonManagerCandidate]) -> Vec<String> {
    let set = candidates
        .iter()
        .map(|candidate| candidate.requirement.value_token())
        .collect::<BTreeSet<_>>();
    set.into_iter().collect()
}

fn source_refs_for_interpreter(candidates: &[&PythonInterpreterCandidate]) -> Vec<String> {
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

fn source_refs_for_manager(candidates: &[&PythonManagerCandidate]) -> Vec<String> {
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

fn same_interpreter_candidate(
    left: &PythonInterpreterCandidate,
    right: &PythonInterpreterCandidate,
) -> bool {
    left.source_kind == right.source_kind
        && left.source_ref == right.source_ref
        && left.version_requirement == right.version_requirement
        && left.interpreter_ref == right.interpreter_ref
}

fn same_manager_candidate(left: &PythonManagerCandidate, right: &PythonManagerCandidate) -> bool {
    left.source_kind == right.source_kind
        && left.source_ref == right.source_ref
        && left.requirement == right.requirement
}

fn card_id(
    subject: PythonEnvironmentSubject,
    source: PythonEnvironmentSourceKind,
    index: usize,
) -> String {
    format!(
        "python_detector.card.{}.{}.{}",
        subject.as_str(),
        source.as_str(),
        index
    )
}

fn source_label(source: PythonEnvironmentSourceKind) -> &'static str {
    match source {
        PythonEnvironmentSourceKind::ExplicitOverride => "Explicit override",
        PythonEnvironmentSourceKind::VenvPyvenvCfg => ".venv pyvenv.cfg",
        PythonEnvironmentSourceKind::VenvDirectory => ".venv directory",
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
        PythonEnvironmentSourceKind::UserProfileDefault => "User/profile default",
        PythonEnvironmentSourceKind::AmbientPath => "Ambient PATH",
        PythonEnvironmentSourceKind::DetectorFallback => "Detector fallback",
        PythonEnvironmentSourceKind::UnreadableSource => "Unreadable source",
    }
}

fn python_interpreter_token(version: Option<&str>, interpreter_ref: Option<&str>) -> String {
    match (version, interpreter_ref) {
        (Some(version), Some(interpreter_ref))
            if !version.is_empty() && !interpreter_ref.is_empty() =>
        {
            format!("python@{version} ({interpreter_ref})")
        }
        (Some(version), _) if !version.is_empty() => format!("python@{version}"),
        (_, Some(interpreter_ref)) if !interpreter_ref.is_empty() => interpreter_ref.to_owned(),
        _ => "python@unresolved".to_owned(),
    }
}

fn venv_interpreter_ref(_workspace_root: &Path, cfg: &PyvenvCfg) -> String {
    if let Some(executable) = &cfg.executable {
        return executable.clone();
    }
    ".venv/bin/python".to_owned()
}

fn pyproject_value<'a>(pyproject: &'a Option<SimpleToml>, key: &str) -> Option<&'a str> {
    pyproject.as_ref()?.value(key)
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

fn parse_simple_toml(payload: &str, strict_keys: &[&str]) -> Result<SimpleToml, String> {
    let mut current_section = String::new();
    let mut toml = SimpleToml::default();
    for (idx, raw_line) in payload.lines().enumerate() {
        let line = strip_toml_comment(raw_line).trim().to_owned();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') {
            if !line.ends_with(']') {
                return Err(format!("line {} has an unterminated table header", idx + 1));
            }
            current_section = line
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_owned();
            if current_section.is_empty() {
                return Err(format!("line {} has an empty table header", idx + 1));
            }
            toml.sections.insert(current_section.clone());
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim().trim_matches('"').trim_matches('\'');
        let full_key = if current_section.is_empty() {
            key.to_owned()
        } else {
            format!("{current_section}.{key}")
        };
        let strict = strict_keys.contains(&full_key.as_str());
        match parse_simple_toml_value(value.trim()) {
            Some(value) => {
                toml.values.insert(full_key, value);
            }
            None if strict => {
                return Err(format!(
                    "line {} has an unsupported value for {full_key}",
                    idx + 1
                ));
            }
            None => {}
        }
    }
    Ok(toml)
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

fn parse_simple_toml_value(raw: &str) -> Option<String> {
    let value = raw.trim();
    if value.is_empty() || value.starts_with('[') || value.starts_with('{') {
        return None;
    }
    if value.len() >= 2 {
        let bytes = value.as_bytes();
        let first = bytes[0] as char;
        let last = bytes[value.len() - 1] as char;
        if (first == '"' && last == '"') || (first == '\'' && last == '\'') {
            return Some(value[1..value.len() - 1].trim().to_owned());
        }
    }
    non_empty(value)
}

fn normalize_python_version(raw: impl AsRef<str>) -> Option<String> {
    let mut value = raw.as_ref().trim();
    if value.is_empty() {
        return None;
    }
    value = value.strip_prefix("Python ").unwrap_or(value);
    value = value.strip_prefix("python ").unwrap_or(value);
    value = value.strip_prefix('v').unwrap_or(value);
    non_empty(value)
}

fn non_empty(raw: impl AsRef<str>) -> Option<String> {
    let value = raw.as_ref().trim();
    if value.is_empty() {
        None
    } else {
        Some(value.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_root(name: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/runtime/python_detection_alpha")
            .join(name)
    }

    fn detector() -> PythonEnvironmentDetector {
        PythonEnvironmentDetector::new(PythonEnvironmentDetectorConfig {
            ambient_python_version: Some("3.12.7".to_owned()),
            ambient_interpreter_ref: Some("/usr/bin/python3".to_owned()),
            ambient_uv_version: Some("0.5.7".to_owned()),
            ambient_poetry_version: Some("1.8.4".to_owned()),
            ..PythonEnvironmentDetectorConfig::default()
        })
    }

    #[test]
    fn uv_workspace_selects_venv_interpreter_and_uv_manager() {
        let report = detector().detect_workspace(&fixture_root("uv_workspace"), "mono:0");

        assert_eq!(report.record_kind, PYTHON_ENVIRONMENT_DETECTION_RECORD_KIND);
        assert_eq!(
            report.interpreter.resolution_state,
            PythonEnvironmentResolutionState::Resolved
        );
        assert_eq!(
            report.interpreter.winning_source,
            Some(PythonEnvironmentSourceKind::VenvPyvenvCfg)
        );
        assert_eq!(
            report.interpreter.resolved_requirement.as_deref(),
            Some("3.12.6")
        );
        assert_eq!(
            report.interpreter.interpreter_ref.as_deref(),
            Some(".venv/bin/python")
        );
        assert_eq!(
            report.environment_manager.resolution_state,
            PythonEnvironmentResolutionState::Resolved
        );
        assert_eq!(
            report.environment_manager.kind,
            Some(PythonEnvironmentManagerKind::Uv)
        );
        assert!(report.provenance_cards.iter().any(|card| card.source_kind
            == PythonEnvironmentSourceKind::PythonVersionFile
            && card.disposition == PythonEnvironmentProvenanceDisposition::Corroborating));
        assert!(!report.has_unresolved_ambiguity());
        assert!(report.has_fallback());
    }

    #[test]
    fn poetry_workspace_resolves_poetry_manager() {
        let report = detector().detect_workspace(&fixture_root("poetry_workspace"), "mono:0");

        assert_eq!(
            report.interpreter.winning_source,
            Some(PythonEnvironmentSourceKind::PyprojectPoetryDependency)
        );
        assert_eq!(
            report.environment_manager.winning_source,
            Some(PythonEnvironmentSourceKind::PoetryLockfile)
        );
        assert_eq!(
            report.environment_manager.kind,
            Some(PythonEnvironmentManagerKind::Poetry)
        );
        assert_eq!(
            report
                .environment_manager
                .fallback_path
                .as_ref()
                .map(|fallback| fallback.value_token.as_str()),
            Some("poetry@1.8.4")
        );
    }

    #[test]
    fn venv_only_workspace_names_local_environment() {
        let report = detector().detect_workspace(&fixture_root("venv_only"), "mono:0");

        assert_eq!(
            report.interpreter.winning_source,
            Some(PythonEnvironmentSourceKind::VenvPyvenvCfg)
        );
        assert_eq!(
            report.environment_manager.winning_source,
            Some(PythonEnvironmentSourceKind::VenvPyvenvCfg)
        );
        assert_eq!(
            report.environment_manager.kind,
            Some(PythonEnvironmentManagerKind::Venv)
        );
        assert_eq!(
            report.environment_manager.environment_ref.as_deref(),
            Some(".venv")
        );
    }

    #[test]
    fn conflicting_python_version_files_block_interpreter_winner() {
        let report =
            detector().detect_workspace(&fixture_root("ambiguous_interpreter_pins"), "mono:0");

        assert_eq!(
            report.interpreter.resolution_state,
            PythonEnvironmentResolutionState::Ambiguous
        );
        assert!(report.has_unresolved_ambiguity());
        assert!(report.unresolved_ambiguities.iter().any(|ambiguity| {
            ambiguity.subject == PythonEnvironmentSubject::Interpreter
                && ambiguity
                    .candidate_values
                    .contains(&"python@3.11.9".to_owned())
                && ambiguity
                    .candidate_values
                    .contains(&"python@3.12.1".to_owned())
        }));
        assert!(report
            .provenance_cards
            .iter()
            .any(|card| card.disposition == PythonEnvironmentProvenanceDisposition::Ambiguous));
    }

    #[test]
    fn malformed_pyproject_surfaces_detector_failure() {
        let report = detector().detect_workspace(&fixture_root("malformed_pyproject"), "mono:0");

        assert!(report.has_detector_failure());
        assert!(report.provenance_cards.iter().any(|card| {
            card.source_kind == PythonEnvironmentSourceKind::UnreadableSource
                && card.disposition == PythonEnvironmentProvenanceDisposition::Unsupported
                && card.summary.contains("pyproject.toml could not be parsed")
        }));
    }
}
