//! Workspace archetype detection from source-controlled marker files.
//!
//! The detector is intentionally read-only: it scans a bounded workspace root,
//! reads common project manifests, and returns an inspectable recommendation
//! that later admission surfaces may show to the user.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::admission::checkpoint::{
    ArchetypeTruth, DetectionConfidenceClass, DetectionEvidenceFreshness, DetectionOutcome,
    DetectionSignal, DetectionSignalSourceClass, DetectorState, SignalFreshnessClass,
    SignalMaterialEffect, SupportClaimClass,
};

const ARCHETYPE_SEED_ROWS: (&str, &str) = (
    "artifacts/compat/m3/archetype_detection_matrix.yaml",
    include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/compat/m3/archetype_detection_matrix.yaml"
    )),
);

const STRONG_PROPOSAL_SCORE: u8 = 70;
const CONFLICTING_PROPOSAL_SCORE: u8 = 65;
const CONFLICT_MARGIN: u8 = 25;
const MAX_EXTENSION_SCAN_ENTRIES: usize = 512;

/// Launch archetype family inferred before binding to a seed row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LaunchArchetypeFamily {
    /// TypeScript or JavaScript web app or service.
    TypeScriptJavaScriptWeb,
    /// Python service or data app.
    PythonServiceOrDataApp,
    /// Rust workspace.
    RustWorkspace,
    /// Go service or monorepo slice.
    GoServiceOrMonorepoSlice,
    /// Java or Kotlin service.
    JavaOrKotlinService,
    /// C or C++ native project.
    COrCppNativeProject,
}

impl LaunchArchetypeFamily {
    /// Returns the stable detector-local token for this family.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TypeScriptJavaScriptWeb => "typescript_javascript_web",
            Self::PythonServiceOrDataApp => "python_service_or_data_app",
            Self::RustWorkspace => "rust_workspace",
            Self::GoServiceOrMonorepoSlice => "go_service_or_monorepo_slice",
            Self::JavaOrKotlinService => "java_or_kotlin_service",
            Self::COrCppNativeProject => "c_or_cpp_native_project",
        }
    }
}

/// Seed-row catalog loaded from the checked-in archetype certification file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchetypeSeedCatalog {
    /// Rows available to archetype detection.
    pub rows: Vec<ArchetypeSeedRow>,
}

impl ArchetypeSeedCatalog {
    /// Returns the seed row matching a detector family, when the seed file declares one.
    pub fn row_for_family(&self, family: LaunchArchetypeFamily) -> Option<&ArchetypeSeedRow> {
        self.rows.iter().find(|row| row.matches_family(family))
    }
}

/// One archetype seed row consumed by detection.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ArchetypeSeedRow {
    /// Stable seed-row id from the certification artifact.
    pub row_id: String,
    /// Certified-archetype row reference from the certification artifact.
    pub archetype_row_ref: String,
    /// Human-readable archetype label.
    pub public_label: String,
    /// Representative stack text used only for family matching.
    pub representative_stack: String,
    /// Launch bundle joined to this row.
    pub bundle_ref: String,
    /// Bundle manifest joined to this row.
    pub bundle_manifest_ref: String,
    /// Current support class declared by the seed row.
    pub current_support_class: String,
    /// Certification state declared by the seed row.
    pub certification_state: String,
    /// Evidence packet opened from archetype badges.
    pub evidence_packet_ref: String,
    /// Freshness fields used to expose evidence age on certified/probable states.
    #[serde(default)]
    pub freshness: Option<ArchetypeSeedFreshness>,
}

impl ArchetypeSeedRow {
    fn matches_family(&self, family: LaunchArchetypeFamily) -> bool {
        let text = format!(
            "{}\n{}",
            self.public_label.to_ascii_lowercase(),
            self.representative_stack.to_ascii_lowercase()
        );
        match family {
            LaunchArchetypeFamily::TypeScriptJavaScriptWeb => {
                text.contains("typescript") || text.contains("javascript")
            }
            LaunchArchetypeFamily::PythonServiceOrDataApp => text.contains("python"),
            LaunchArchetypeFamily::RustWorkspace => text.contains("rust"),
            LaunchArchetypeFamily::GoServiceOrMonorepoSlice => {
                text.contains("go service") || text.contains("monorepo slice")
            }
            LaunchArchetypeFamily::JavaOrKotlinService => {
                text.contains("java") || text.contains("kotlin")
            }
            LaunchArchetypeFamily::COrCppNativeProject => {
                text.contains("c / c++") || text.contains("c++") || text.contains("native")
            }
        }
    }
}

/// Freshness metadata for an archetype seed or scorecard row.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ArchetypeSeedFreshness {
    /// Source freshness state.
    pub state: String,
    /// Date the evidence row was reviewed.
    pub reviewed_on: String,
    /// Review window or stale-after duration.
    pub stale_after: String,
}

/// Outcome of one read-only archetype detection pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchetypeDetectionOutcome {
    /// One launch archetype is strong enough to suggest.
    Proposed,
    /// No recognized launch archetype is strong enough to suggest.
    NoRecognizedArchetype,
    /// Multiple strong marker sets compete, so no single archetype is suggested.
    ConflictingMarkers,
}

impl ArchetypeDetectionOutcome {
    /// Returns the stable detector-local token for this outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Proposed => "proposed",
            Self::NoRecognizedArchetype => "no_recognized_archetype",
            Self::ConflictingMarkers => "conflicting_markers",
        }
    }
}

/// One marker signal found during archetype detection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchetypeDetectionSignal {
    /// Opaque signal reference.
    pub signal_ref: String,
    /// Family this signal contributed to.
    pub family: LaunchArchetypeFamily,
    /// Marker file or bounded file cue that produced the signal.
    pub marker: String,
    /// Source class used by admission checkpoint truth.
    pub source_class: DetectionSignalSourceClass,
    /// Score contribution before family-level capping.
    pub score_delta: u8,
    /// Redacted reviewer-facing summary.
    pub summary: String,
}

/// Proposed archetype with seed-row identity and a confidence score.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchetypeProposal {
    /// Family selected by the detector.
    pub family: LaunchArchetypeFamily,
    /// Confidence score in the inclusive range `0..=100`.
    pub confidence_score: u8,
    /// Stable seed-row id loaded from the seed rows artifact.
    pub archetype_seed_row_id: String,
    /// Archetype row reference loaded from the seed rows artifact.
    pub archetype_row_ref: String,
    /// Human-readable archetype label loaded from the seed rows artifact.
    pub public_label: String,
    /// Launch bundle reference loaded from the seed rows artifact.
    pub bundle_ref: String,
    /// Bundle manifest reference loaded from the seed rows artifact.
    pub bundle_manifest_ref: String,
    /// Current support class loaded from the seed rows artifact.
    pub current_support_class: String,
    /// Current certification state loaded from the seed rows artifact.
    pub certification_state: String,
    /// Evidence packet reference loaded from the seed rows artifact.
    pub evidence_packet_ref: String,
    /// Evidence freshness class derived from the seed or scorecard row.
    pub evidence_freshness_class: SignalFreshnessClass,
    /// Date the evidence was reviewed, when declared by the source row.
    pub evidence_reviewed_on: Option<String>,
    /// Review window after which the evidence must be retested, when declared.
    pub evidence_stale_after: Option<String>,
}

impl ArchetypeProposal {
    fn from_seed_row(
        family: LaunchArchetypeFamily,
        confidence_score: u8,
        row: &ArchetypeSeedRow,
    ) -> Self {
        Self {
            family,
            confidence_score,
            archetype_seed_row_id: row.row_id.clone(),
            archetype_row_ref: row.archetype_row_ref.clone(),
            public_label: row.public_label.clone(),
            bundle_ref: row.bundle_ref.clone(),
            bundle_manifest_ref: row.bundle_manifest_ref.clone(),
            current_support_class: row.current_support_class.clone(),
            certification_state: row.certification_state.clone(),
            evidence_packet_ref: row.evidence_packet_ref.clone(),
            evidence_freshness_class: evidence_freshness_class_for(row),
            evidence_reviewed_on: row
                .freshness
                .as_ref()
                .map(|freshness| freshness.reviewed_on.clone()),
            evidence_stale_after: row
                .freshness
                .as_ref()
                .map(|freshness| freshness.stale_after.clone()),
        }
    }
}

/// Full archetype detection report for a workspace root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchetypeDetectionReport {
    /// Workspace root that was scanned.
    pub workspace_root: PathBuf,
    /// Detector outcome.
    pub outcome: ArchetypeDetectionOutcome,
    /// Proposed archetype, when exactly one strong seed-backed family wins.
    pub proposal: Option<ArchetypeProposal>,
    /// Source-labeled marker signals that explain the outcome.
    pub signals: Vec<ArchetypeDetectionSignal>,
    /// Competing archetype row refs or family tokens when markers conflict.
    pub competing_archetype_refs: Vec<String>,
    /// Redacted unknowns or gaps that prevented a stronger claim.
    pub unknowns: Vec<String>,
}

impl ArchetypeDetectionReport {
    /// Converts this detection report into checkpoint archetype truth.
    pub fn to_archetype_truth(&self) -> ArchetypeTruth {
        match (&self.outcome, &self.proposal) {
            (ArchetypeDetectionOutcome::Proposed, Some(proposal)) => {
                let support_claim = support_claim_for(proposal);
                ArchetypeTruth::new(
                    detection_outcome_for(proposal),
                    confidence_for(proposal, support_claim),
                    support_claim,
                    detector_state_for(proposal),
                    self.checkpoint_signals(),
                )
                .with_archetype_ref(proposal.archetype_row_ref.clone())
                .with_compatible_bundle_refs(vec![proposal.bundle_ref.clone()])
                .with_evidence_freshness(vec![evidence_freshness_row(proposal)])
                .with_detected_fact_refs(self.detected_fact_refs())
                .with_recommendation_refs(vec![format!(
                    "recommendation.archetype.{:016x}",
                    stable_hash(&proposal.archetype_seed_row_id)
                )])
            }
            (ArchetypeDetectionOutcome::ConflictingMarkers, _) => ArchetypeTruth::new(
                DetectionOutcome::MixedOrAmbiguousWorkspace,
                DetectionConfidenceClass::MixedConflicting,
                SupportClaimClass::GenericNoClaim,
                DetectorState::Partial,
                self.checkpoint_signals(),
            )
            .with_detected_fact_refs(self.detected_fact_refs())
            .with_recommendation_refs(vec![format!(
                "recommendation.archetype.conflict.{:016x}",
                stable_hash(&self.competing_archetype_refs.join("\n"))
            )]),
            _ => ArchetypeTruth::new(
                DetectionOutcome::UnknownOrGenericWorkspace,
                DetectionConfidenceClass::GenericUnknown,
                SupportClaimClass::GenericNoClaim,
                DetectorState::Unknown,
                self.checkpoint_signals(),
            )
            .with_detected_fact_refs(self.detected_fact_refs()),
        }
    }

    fn checkpoint_signals(&self) -> Vec<DetectionSignal> {
        if self.signals.is_empty() {
            return vec![DetectionSignal::new(
                "signal.archetype.none",
                DetectionSignalSourceClass::FilesystemLayout,
                vec![SignalMaterialEffect::DiagnosticOnly],
                "No recognized archetype markers were found in the workspace root.",
            )
            .with_freshness_class(SignalFreshnessClass::FreshCurrent)];
        }

        self.signals
            .iter()
            .map(|signal| {
                DetectionSignal::new(
                    signal.signal_ref.clone(),
                    signal.source_class,
                    vec![
                        SignalMaterialEffect::Recommendation,
                        SignalMaterialEffect::RouteSelection,
                    ],
                    signal.summary.clone(),
                )
                .with_freshness_class(SignalFreshnessClass::FreshCurrent)
            })
            .collect()
    }

    fn detected_fact_refs(&self) -> Vec<String> {
        if self.signals.is_empty() {
            return vec!["fact.archetype.no_markers".to_string()];
        }
        self.signals
            .iter()
            .map(|signal| format!("fact.archetype.{:016x}", stable_hash(&signal.signal_ref)))
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

/// Error returned when archetype detection cannot complete.
#[derive(Debug)]
pub enum ArchetypeDetectionError {
    /// The seed row artifact could not be parsed.
    SeedRowsParse {
        /// Artifact path used as the source of truth.
        source_ref: &'static str,
        /// Parse failure.
        source: serde_yaml::Error,
    },
    /// The workspace root could not be inspected.
    WorkspaceRootRead {
        /// Workspace root path.
        path: PathBuf,
        /// I/O failure.
        source: std::io::Error,
    },
    /// The supplied path is not a directory.
    WorkspaceRootNotDirectory {
        /// Supplied workspace root path.
        path: PathBuf,
    },
}

impl fmt::Display for ArchetypeDetectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SeedRowsParse { source_ref, source } => {
                write!(formatter, "{source_ref}: parse seed rows failed: {source}")
            }
            Self::WorkspaceRootRead { path, source } => {
                write!(
                    formatter,
                    "{}: read workspace root failed: {source}",
                    path.display()
                )
            }
            Self::WorkspaceRootNotDirectory { path } => {
                write!(formatter, "{} is not a workspace directory", path.display())
            }
        }
    }
}

impl std::error::Error for ArchetypeDetectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::SeedRowsParse { source, .. } => Some(source),
            Self::WorkspaceRootRead { source, .. } => Some(source),
            Self::WorkspaceRootNotDirectory { .. } => None,
        }
    }
}

/// Loads the default archetype seed catalog from the checked-in seed rows artifact.
pub fn default_archetype_seed_catalog() -> Result<ArchetypeSeedCatalog, ArchetypeDetectionError> {
    load_archetype_seed_catalog(ARCHETYPE_SEED_ROWS.1)
}

/// Loads an archetype seed catalog from YAML text.
pub fn load_archetype_seed_catalog(
    yaml: &str,
) -> Result<ArchetypeSeedCatalog, ArchetypeDetectionError> {
    let doc: SeedRowsDoc =
        serde_yaml::from_str(yaml).map_err(|source| ArchetypeDetectionError::SeedRowsParse {
            source_ref: ARCHETYPE_SEED_ROWS.0,
            source,
        })?;
    Ok(ArchetypeSeedCatalog {
        rows: doc.archetype_seed_rows,
    })
}

/// Detects a workspace archetype using the checked-in seed row catalog.
pub fn detect_workspace_archetype(
    workspace_root: impl AsRef<Path>,
) -> Result<ArchetypeDetectionReport, ArchetypeDetectionError> {
    let catalog = default_archetype_seed_catalog()?;
    detect_workspace_archetype_with_catalog(workspace_root, &catalog)
}

/// Returns only the proposed archetype, if detection finds one.
pub fn propose_workspace_archetype(
    workspace_root: impl AsRef<Path>,
) -> Result<Option<ArchetypeProposal>, ArchetypeDetectionError> {
    Ok(detect_workspace_archetype(workspace_root)?.proposal)
}

/// Detects a workspace archetype using an explicit seed row catalog.
pub fn detect_workspace_archetype_with_catalog(
    workspace_root: impl AsRef<Path>,
    catalog: &ArchetypeSeedCatalog,
) -> Result<ArchetypeDetectionReport, ArchetypeDetectionError> {
    let root = workspace_root.as_ref();
    if !root.is_dir() {
        return Err(ArchetypeDetectionError::WorkspaceRootNotDirectory {
            path: root.to_path_buf(),
        });
    }

    let root_entries =
        std::fs::read_dir(root).map_err(|source| ArchetypeDetectionError::WorkspaceRootRead {
            path: root.to_path_buf(),
            source,
        })?;
    let marker_names = root_entries
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().into_string().ok())
        .collect::<BTreeSet<_>>();
    let extension_cues = collect_extension_cues(root);

    let mut scores = BTreeMap::<LaunchArchetypeFamily, FamilyScore>::new();
    score_tsjs(root, &marker_names, &extension_cues, &mut scores);
    score_python(root, &marker_names, &extension_cues, &mut scores);
    score_rust(root, &marker_names, &extension_cues, &mut scores);
    score_go(root, &marker_names, &extension_cues, &mut scores);
    score_java_kotlin(&marker_names, &extension_cues, &mut scores);
    score_c_cpp(&marker_names, &extension_cues, &mut scores);

    let mut ranked = scores
        .iter()
        .map(|(family, score)| (*family, score.total.min(100) as u8))
        .filter(|(_, score)| *score >= CONFLICTING_PROPOSAL_SCORE)
        .collect::<Vec<_>>();
    ranked.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));

    let signals = scores
        .values()
        .flat_map(|score| score.signals.iter().cloned())
        .collect::<Vec<_>>();

    let Some((top_family, top_score)) = ranked.first().copied() else {
        let unknowns = if signals.is_empty() {
            vec![
                "No package, Python, Rust, or lockfile marker crossed the proposal threshold."
                    .to_string(),
            ]
        } else {
            vec![
                "Marker signals were present but not strong enough for an archetype suggestion."
                    .to_string(),
            ]
        };
        return Ok(ArchetypeDetectionReport {
            workspace_root: root.to_path_buf(),
            outcome: ArchetypeDetectionOutcome::NoRecognizedArchetype,
            proposal: None,
            signals,
            competing_archetype_refs: Vec::new(),
            unknowns,
        });
    };

    if has_conflict(&ranked) {
        return Ok(ArchetypeDetectionReport {
            workspace_root: root.to_path_buf(),
            outcome: ArchetypeDetectionOutcome::ConflictingMarkers,
            proposal: None,
            signals,
            competing_archetype_refs: ranked
                .iter()
                .map(|(family, _)| family_ref_for(catalog, *family))
                .collect(),
            unknowns: vec![
                "Multiple strong marker families were present; Start Center must ask the user to choose.".to_string(),
            ],
        });
    }

    let Some(seed_row) = catalog.row_for_family(top_family) else {
        return Ok(ArchetypeDetectionReport {
            workspace_root: root.to_path_buf(),
            outcome: ArchetypeDetectionOutcome::NoRecognizedArchetype,
            proposal: None,
            signals,
            competing_archetype_refs: vec![top_family.as_str().to_string()],
            unknowns: vec![format!(
                "{} markers were strong, but the seed rows artifact has no matching archetype row.",
                top_family.as_str()
            )],
        });
    };

    Ok(ArchetypeDetectionReport {
        workspace_root: root.to_path_buf(),
        outcome: ArchetypeDetectionOutcome::Proposed,
        proposal: Some(ArchetypeProposal::from_seed_row(
            top_family, top_score, seed_row,
        )),
        signals,
        competing_archetype_refs: Vec::new(),
        unknowns: Vec::new(),
    })
}

#[derive(Debug, Deserialize)]
struct SeedRowsDoc {
    archetype_seed_rows: Vec<ArchetypeSeedRow>,
}

#[derive(Debug, Default)]
struct FamilyScore {
    total: u16,
    signals: Vec<ArchetypeDetectionSignal>,
}

fn score_tsjs(
    root: &Path,
    marker_names: &BTreeSet<String>,
    extension_cues: &BTreeSet<String>,
    scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>,
) {
    let family = LaunchArchetypeFamily::TypeScriptJavaScriptWeb;
    if marker_names.contains("package.json") {
        add_signal(
            scores,
            family,
            "package.json",
            DetectionSignalSourceClass::Manifest,
            40,
            "package.json is present.",
        );
        score_package_json(root, scores);
    }
    for (marker, delta, summary) in [
        ("tsconfig.json", 18, "TypeScript config is present."),
        ("jsconfig.json", 12, "JavaScript project config is present."),
        ("vite.config.ts", 22, "Vite TypeScript config is present."),
        ("vite.config.js", 18, "Vite config is present."),
        ("next.config.js", 22, "Next.js config is present."),
        ("next.config.mjs", 22, "Next.js config is present."),
        (
            "next.config.ts",
            22,
            "Next.js TypeScript config is present.",
        ),
        ("pnpm-lock.yaml", 12, "pnpm lockfile is present."),
        ("package-lock.json", 10, "npm lockfile is present."),
        ("yarn.lock", 10, "Yarn lockfile is present."),
        ("bun.lockb", 10, "Bun lockfile is present."),
    ] {
        if marker_names.contains(marker) {
            let source = if marker.ends_with("lock") || marker.ends_with("lockb") {
                DetectionSignalSourceClass::Lockfile
            } else {
                DetectionSignalSourceClass::Manifest
            };
            add_signal(scores, family, marker, source, delta, summary);
        }
    }
    if ["ts", "tsx", "js", "jsx"]
        .iter()
        .any(|extension| extension_cues.contains(*extension))
    {
        add_signal(
            scores,
            family,
            "bounded_extension_scan",
            DetectionSignalSourceClass::FilesystemLayout,
            10,
            "Bounded scan found TypeScript or JavaScript source files.",
        );
    }
}

fn score_package_json(root: &Path, scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>) {
    let family = LaunchArchetypeFamily::TypeScriptJavaScriptWeb;
    let Some(payload) = read_marker(root, "package.json") else {
        return;
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(&payload) else {
        add_signal(
            scores,
            family,
            "package.json",
            DetectionSignalSourceClass::Manifest,
            5,
            "package.json could not be parsed, but still indicates a Node workspace.",
        );
        return;
    };

    let deps = package_dependency_names(&value);
    if deps.iter().any(|dep| dep == "typescript") {
        add_signal(
            scores,
            family,
            "package.json#typescript",
            DetectionSignalSourceClass::Manifest,
            18,
            "package.json declares TypeScript.",
        );
    }
    if deps.iter().any(|dep| {
        matches!(
            dep.as_str(),
            "react"
                | "next"
                | "vite"
                | "@vitejs/plugin-react"
                | "vue"
                | "svelte"
                | "astro"
                | "express"
                | "fastify"
        )
    }) {
        add_signal(
            scores,
            family,
            "package.json#web_stack",
            DetectionSignalSourceClass::Manifest,
            20,
            "package.json declares a web framework or service runtime.",
        );
    }
    if deps
        .iter()
        .any(|dep| matches!(dep.as_str(), "vitest" | "jest" | "playwright" | "cypress"))
    {
        add_signal(
            scores,
            family,
            "package.json#test_runner",
            DetectionSignalSourceClass::Manifest,
            10,
            "package.json declares a JavaScript test runner.",
        );
    }
    if value
        .get("scripts")
        .and_then(serde_json::Value::as_object)
        .is_some_and(|scripts| {
            ["dev", "build", "test", "start"]
                .iter()
                .any(|script| scripts.contains_key(*script))
        })
    {
        add_signal(
            scores,
            family,
            "package.json#scripts",
            DetectionSignalSourceClass::Manifest,
            8,
            "package.json declares runnable project scripts.",
        );
    }
}

fn score_python(
    root: &Path,
    marker_names: &BTreeSet<String>,
    extension_cues: &BTreeSet<String>,
    scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>,
) {
    let family = LaunchArchetypeFamily::PythonServiceOrDataApp;
    if marker_names.contains("pyproject.toml") {
        add_signal(
            scores,
            family,
            "pyproject.toml",
            DetectionSignalSourceClass::Manifest,
            55,
            "pyproject.toml is present.",
        );
        score_pyproject(root, scores);
    }
    for (marker, delta, summary) in [
        ("requirements.txt", 28, "requirements.txt is present."),
        (
            "requirements-dev.txt",
            20,
            "development requirements file is present.",
        ),
        ("poetry.lock", 15, "Poetry lockfile is present."),
        ("uv.lock", 15, "uv lockfile is present."),
        ("Pipfile", 16, "Pipfile is present."),
        ("pytest.ini", 12, "pytest configuration is present."),
        ("tox.ini", 10, "tox configuration is present."),
    ] {
        if marker_names.contains(marker) {
            let source = if marker.ends_with(".lock") || marker == "uv.lock" {
                DetectionSignalSourceClass::Lockfile
            } else {
                DetectionSignalSourceClass::Manifest
            };
            add_signal(scores, family, marker, source, delta, summary);
        }
    }
    if extension_cues.contains("py")
        || marker_names.contains("app.py")
        || marker_names.contains("main.py")
    {
        add_signal(
            scores,
            family,
            "bounded_extension_scan",
            DetectionSignalSourceClass::FilesystemLayout,
            10,
            "Bounded scan found Python source files.",
        );
    }
    if extension_cues.contains("ipynb") {
        add_signal(
            scores,
            family,
            "bounded_notebook_scan",
            DetectionSignalSourceClass::FilesystemLayout,
            5,
            "Bounded scan found notebook adjacency.",
        );
    }
}

fn score_pyproject(root: &Path, scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>) {
    let Some(payload) = read_marker(root, "pyproject.toml") else {
        return;
    };
    let lower = payload.to_ascii_lowercase();
    let family = LaunchArchetypeFamily::PythonServiceOrDataApp;
    if [
        "pytest", "pandas", "numpy", "scipy", "fastapi", "django", "flask", "jupyter",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
    {
        add_signal(
            scores,
            family,
            "pyproject.toml#python_stack",
            DetectionSignalSourceClass::Manifest,
            24,
            "pyproject.toml declares Python service, test, or data dependencies.",
        );
    }
    if lower.contains("[tool.pytest") || lower.contains("pytest.ini_options") {
        add_signal(
            scores,
            family,
            "pyproject.toml#pytest",
            DetectionSignalSourceClass::Manifest,
            10,
            "pyproject.toml declares pytest configuration.",
        );
    }
}

fn score_rust(
    root: &Path,
    marker_names: &BTreeSet<String>,
    extension_cues: &BTreeSet<String>,
    scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>,
) {
    let family = LaunchArchetypeFamily::RustWorkspace;
    if marker_names.contains("Cargo.toml") {
        add_signal(
            scores,
            family,
            "Cargo.toml",
            DetectionSignalSourceClass::Manifest,
            58,
            "Cargo.toml is present.",
        );
        if read_marker(root, "Cargo.toml").is_some_and(|payload| payload.contains("[workspace]")) {
            add_signal(
                scores,
                family,
                "Cargo.toml#workspace",
                DetectionSignalSourceClass::Manifest,
                18,
                "Cargo.toml declares a Rust workspace.",
            );
        }
    }
    if marker_names.contains("Cargo.lock") {
        add_signal(
            scores,
            family,
            "Cargo.lock",
            DetectionSignalSourceClass::Lockfile,
            12,
            "Cargo.lock is present.",
        );
    }
    if extension_cues.contains("rs") {
        add_signal(
            scores,
            family,
            "bounded_extension_scan",
            DetectionSignalSourceClass::FilesystemLayout,
            10,
            "Bounded scan found Rust source files.",
        );
    }
}

fn score_go(
    root: &Path,
    marker_names: &BTreeSet<String>,
    extension_cues: &BTreeSet<String>,
    scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>,
) {
    let family = LaunchArchetypeFamily::GoServiceOrMonorepoSlice;
    if marker_names.contains("go.mod") {
        add_signal(
            scores,
            family,
            "go.mod",
            DetectionSignalSourceClass::Manifest,
            58,
            "go.mod is present.",
        );
        if read_marker(root, "go.mod").is_some_and(|payload| payload.contains("module ")) {
            add_signal(
                scores,
                family,
                "go.mod#module",
                DetectionSignalSourceClass::Manifest,
                18,
                "go.mod declares a Go module.",
            );
        }
    }
    if marker_names.contains("go.sum") {
        add_signal(
            scores,
            family,
            "go.sum",
            DetectionSignalSourceClass::Lockfile,
            12,
            "go.sum is present.",
        );
    }
    if extension_cues.contains("go") {
        add_signal(
            scores,
            family,
            "bounded_extension_scan",
            DetectionSignalSourceClass::FilesystemLayout,
            10,
            "Bounded scan found Go source files.",
        );
    }
}

fn score_java_kotlin(
    marker_names: &BTreeSet<String>,
    extension_cues: &BTreeSet<String>,
    scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>,
) {
    let family = LaunchArchetypeFamily::JavaOrKotlinService;
    for (marker, delta, summary) in [
        ("pom.xml", 55, "Maven project file is present."),
        ("build.gradle", 52, "Gradle build file is present."),
        (
            "build.gradle.kts",
            52,
            "Gradle Kotlin build file is present.",
        ),
        ("settings.gradle", 16, "Gradle settings file is present."),
        (
            "settings.gradle.kts",
            16,
            "Gradle Kotlin settings file is present.",
        ),
        (
            "gradle.properties",
            10,
            "Gradle properties file is present.",
        ),
        ("mvnw", 8, "Maven wrapper is present."),
        ("gradlew", 8, "Gradle wrapper is present."),
    ] {
        if marker_names.contains(marker) {
            add_signal(
                scores,
                family,
                marker,
                DetectionSignalSourceClass::Manifest,
                delta,
                summary,
            );
        }
    }
    if ["java", "kt", "kts"]
        .iter()
        .any(|extension| extension_cues.contains(*extension))
    {
        add_signal(
            scores,
            family,
            "bounded_extension_scan",
            DetectionSignalSourceClass::FilesystemLayout,
            10,
            "Bounded scan found Java or Kotlin source files.",
        );
    }
}

fn score_c_cpp(
    marker_names: &BTreeSet<String>,
    extension_cues: &BTreeSet<String>,
    scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>,
) {
    let family = LaunchArchetypeFamily::COrCppNativeProject;
    for (marker, delta, summary, source_class) in [
        (
            "CMakeLists.txt",
            58,
            "CMake project file is present.",
            DetectionSignalSourceClass::Manifest,
        ),
        (
            "compile_commands.json",
            35,
            "Compilation database is present.",
            DetectionSignalSourceClass::Manifest,
        ),
        (
            "meson.build",
            42,
            "Meson build file is present.",
            DetectionSignalSourceClass::Manifest,
        ),
        (
            "Makefile",
            26,
            "Makefile is present.",
            DetectionSignalSourceClass::Manifest,
        ),
        (
            "vcpkg.json",
            18,
            "vcpkg manifest is present.",
            DetectionSignalSourceClass::Manifest,
        ),
    ] {
        if marker_names.contains(marker) {
            add_signal(scores, family, marker, source_class, delta, summary);
        }
    }
    if ["c", "cc", "cpp", "cxx", "h", "hh", "hpp", "hxx"]
        .iter()
        .any(|extension| extension_cues.contains(*extension))
    {
        add_signal(
            scores,
            family,
            "bounded_extension_scan",
            DetectionSignalSourceClass::FilesystemLayout,
            10,
            "Bounded scan found C or C++ source files.",
        );
    }
}

fn add_signal(
    scores: &mut BTreeMap<LaunchArchetypeFamily, FamilyScore>,
    family: LaunchArchetypeFamily,
    marker: &str,
    source_class: DetectionSignalSourceClass,
    score_delta: u8,
    summary: &str,
) {
    let score = scores.entry(family).or_default();
    score.total = (score.total + u16::from(score_delta)).min(100);
    score.signals.push(ArchetypeDetectionSignal {
        signal_ref: format!(
            "signal.archetype.{}.{:016x}",
            family.as_str(),
            stable_hash(marker)
        ),
        family,
        marker: marker.to_string(),
        source_class,
        score_delta,
        summary: summary.to_string(),
    });
}

fn package_dependency_names(value: &serde_json::Value) -> BTreeSet<String> {
    [
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ]
    .iter()
    .filter_map(|key| value.get(*key).and_then(serde_json::Value::as_object))
    .flat_map(|deps| deps.keys().map(|key| key.to_ascii_lowercase()))
    .collect()
}

fn read_marker(root: &Path, marker: &str) -> Option<String> {
    std::fs::read_to_string(root.join(marker)).ok()
}

fn collect_extension_cues(root: &Path) -> BTreeSet<String> {
    let mut extensions = BTreeSet::new();
    let mut visited = 0;
    collect_extension_cues_inner(root, 0, &mut visited, &mut extensions);
    extensions
}

fn collect_extension_cues_inner(
    dir: &Path,
    depth: usize,
    visited: &mut usize,
    extensions: &mut BTreeSet<String>,
) {
    if depth > 2 || *visited >= MAX_EXTENSION_SCAN_ENTRIES {
        return;
    }
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        if *visited >= MAX_EXTENSION_SCAN_ENTRIES {
            return;
        }
        *visited += 1;
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if path.is_dir() {
            if is_ignored_scan_dir(name) {
                continue;
            }
            collect_extension_cues_inner(&path, depth + 1, visited, extensions);
            continue;
        }
        if let Some(extension) = path.extension().and_then(|extension| extension.to_str()) {
            extensions.insert(extension.to_ascii_lowercase());
        }
    }
}

fn is_ignored_scan_dir(name: &str) -> bool {
    matches!(
        name,
        ".git"
            | "node_modules"
            | "target"
            | ".venv"
            | "venv"
            | "__pycache__"
            | ".mypy_cache"
            | ".pytest_cache"
            | "dist"
            | "build"
    )
}

fn has_conflict(ranked: &[(LaunchArchetypeFamily, u8)]) -> bool {
    let [first, second, ..] = ranked else {
        return false;
    };
    second.1 >= STRONG_PROPOSAL_SCORE
        || (second.1 >= CONFLICTING_PROPOSAL_SCORE
            && first.1.saturating_sub(second.1) < CONFLICT_MARGIN)
}

fn family_ref_for(catalog: &ArchetypeSeedCatalog, family: LaunchArchetypeFamily) -> String {
    catalog
        .row_for_family(family)
        .map(|row| row.archetype_row_ref.clone())
        .unwrap_or_else(|| family.as_str().to_string())
}

fn detection_outcome_for(proposal: &ArchetypeProposal) -> DetectionOutcome {
    if support_claim_for(proposal) == SupportClaimClass::CertifiedCurrent {
        DetectionOutcome::CertifiedArchetypeMatch
    } else {
        DetectionOutcome::ProbableArchetype
    }
}

fn confidence_for(
    proposal: &ArchetypeProposal,
    support_claim: SupportClaimClass,
) -> DetectionConfidenceClass {
    if support_claim == SupportClaimClass::CertifiedCurrent {
        DetectionConfidenceClass::CertifiedExact
    } else if proposal.confidence_score >= 85 {
        DetectionConfidenceClass::HighProbable
    } else {
        DetectionConfidenceClass::MediumProbable
    }
}

fn detector_state_for(proposal: &ArchetypeProposal) -> DetectorState {
    match proposal.certification_state.as_str() {
        "retest_pending" | "evidence_stale" => DetectorState::RetestNeeded,
        _ => DetectorState::ReadyEnough,
    }
}

fn evidence_freshness_row(proposal: &ArchetypeProposal) -> DetectionEvidenceFreshness {
    DetectionEvidenceFreshness::new(
        proposal.evidence_packet_ref.clone(),
        proposal.evidence_freshness_class,
        format!(
            "{} evidence for {} is {}.",
            proposal.evidence_packet_ref,
            proposal.archetype_row_ref,
            proposal.evidence_freshness_class.as_str()
        ),
    )
    .with_review_window(
        proposal.evidence_reviewed_on.clone(),
        proposal.evidence_stale_after.clone(),
    )
}

fn evidence_freshness_class_for(row: &ArchetypeSeedRow) -> SignalFreshnessClass {
    match (
        row.certification_state.as_str(),
        row.freshness
            .as_ref()
            .map(|freshness| freshness.state.as_str()),
    ) {
        ("certified", Some("current" | "current_seed")) => SignalFreshnessClass::FreshCurrent,
        ("retest_pending" | "evidence_stale", _)
        | (_, Some("retest_pending" | "evidence_stale")) => SignalFreshnessClass::StaleRetestNeeded,
        (_, Some("current" | "current_seed")) => SignalFreshnessClass::CachedCurrentEnough,
        _ => SignalFreshnessClass::UnknownFreshness,
    }
}

fn support_claim_for(proposal: &ArchetypeProposal) -> SupportClaimClass {
    match (
        proposal.current_support_class.as_str(),
        proposal.certification_state.as_str(),
    ) {
        ("certified", "certified") => SupportClaimClass::CertifiedCurrent,
        ("certified", "retest_pending" | "evidence_stale") => {
            SupportClaimClass::CertifiedRetestPending
        }
        ("supported", _) => SupportClaimClass::SupportedScoped,
        ("community", _) => SupportClaimClass::CommunityOrExtensionPath,
        _ => SupportClaimClass::ExperimentalPreview,
    }
}

fn stable_hash(value: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
