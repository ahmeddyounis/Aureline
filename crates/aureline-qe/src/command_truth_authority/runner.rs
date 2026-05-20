//! Loader + runner for the command-truth and palette-authority drill corpus.
//!
//! [`load_corpus`] reads `manifest.json`; [`run_corpus`] replays every drill
//! against the `aureline-commands` command-authority boundary
//! ([`CommandAuthorityScenarioRecord`]) and returns a [`CorpusReport`]. Positive
//! drills parse and validate a scenario, project it, and must match every pinned
//! expectation — the canonical command id, lifecycle state, preview/approval
//! posture, agreed enablement decision, the covered invocation surfaces, the
//! honest automation labels, lineage completeness, and rollback requirement.
//! Negative drills must FAIL validation with an error whose message contains the
//! recorded substring, so a surface that widens authority, suppresses
//! preview/approval, lies about automation labels, or drops a lineage join stays
//! rejected.

use std::path::{Path, PathBuf};

use aureline_commands::CommandAuthorityScenarioRecord;

use super::manifest::{
    drill_kind, CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, MANIFEST_FILE_NAME,
};

/// Raw-content tokens that must never appear in a corpus fixture. Their presence
/// would mean a fixture is leaking an actual secret, private key, credential, or
/// absolute local path across the support-safe boundary.
const FORBIDDEN_RAW_TOKENS: &[&str] = &[
    "-----BEGIN",
    "/Users/",
    "/home/",
    "C:\\Users",
    "AKIA",
    "ghp_",
    "password=",
    "BEARER ",
];

/// Outcome of a single drill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrillOutcome {
    /// The drill met every expectation.
    Pass,
    /// The drill failed for the recorded reason.
    Fail(DrillFailureReason),
}

/// Why a drill failed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrillFailureReason {
    /// The fixture could not be read.
    FixtureRead(String),
    /// The fixture did not parse into the expected record.
    Parse(String),
    /// The fixture contained a forbidden raw-content token.
    RawExportToken(String),
    /// A positive drill record failed validation unexpectedly.
    Validation(String),
    /// A positive drill missed a pinned expectation.
    Expectation(String),
    /// A negative drill was accepted by validation instead of being rejected.
    NegativeAccepted,
    /// A negative drill failed, but not with the recorded substring.
    NegativeWrongMessage {
        /// Substring the corpus expected.
        expected: String,
        /// Message the validator actually produced.
        actual: String,
    },
    /// The drill named an unknown kind.
    UnknownKind(String),
}

/// One drill's report row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrillReport {
    /// Stable drill id.
    pub drill_id: String,
    /// Whether this is a positive drill.
    pub positive: bool,
    /// Drill kind token.
    pub kind: String,
    /// Canonical command id (positive drills only; empty for negatives).
    pub command_id: String,
    /// Absolute fixture path replayed.
    pub fixture_path: PathBuf,
    /// Pass / fail outcome.
    pub outcome: DrillOutcome,
}

impl DrillReport {
    /// Returns true when the drill passed.
    pub fn passed(&self) -> bool {
        matches!(self.outcome, DrillOutcome::Pass)
    }
}

/// Aggregate report for the corpus run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusReport {
    /// Corpus id from the manifest.
    pub corpus_id: String,
    /// Per-drill reports in manifest order (positive, then negative).
    pub drills: Vec<DrillReport>,
}

impl CorpusReport {
    /// Returns true when every drill passed.
    pub fn all_passed(&self) -> bool {
        self.drills.iter().all(DrillReport::passed)
    }

    /// Returns the failing drills.
    pub fn failures(&self) -> Vec<&DrillReport> {
        self.drills.iter().filter(|d| !d.passed()).collect()
    }
}

/// Returns the corpus directory under a repository root.
pub fn corpus_dir_from_repo_root(repo_root: &Path) -> PathBuf {
    repo_root.join(super::manifest::CORPUS_DIR_REL)
}

/// Loads and parses the corpus manifest from a corpus directory.
pub fn load_corpus(corpus_dir: &Path) -> Result<CorpusManifest, String> {
    let manifest_path = corpus_dir.join(MANIFEST_FILE_NAME);
    let payload = std::fs::read_to_string(&manifest_path)
        .map_err(|err| format!("failed to read {}: {err}", manifest_path.display()))?;
    serde_json::from_str(&payload)
        .map_err(|err| format!("failed to parse {}: {err}", manifest_path.display()))
}

/// Loads the corpus from a repository root and runs every drill.
pub fn run_corpus_from_repo_root(repo_root: &Path) -> CorpusReport {
    let corpus_dir = corpus_dir_from_repo_root(repo_root);
    let manifest = load_corpus(&corpus_dir).unwrap_or_else(|err| {
        panic!("command-truth and palette-authority corpus manifest must load: {err}")
    });
    run_corpus(&corpus_dir, &manifest)
}

/// Runs every drill named by a manifest against the corpus directory.
pub fn run_corpus(corpus_dir: &Path, manifest: &CorpusManifest) -> CorpusReport {
    let mut drills = Vec::new();
    for spec in &manifest.positive_drills {
        drills.push(run_positive(corpus_dir, spec));
    }
    for spec in &manifest.negative_drills {
        drills.push(run_negative(corpus_dir, spec));
    }
    CorpusReport {
        corpus_id: manifest.corpus_id.clone(),
        drills,
    }
}

fn run_positive(corpus_dir: &Path, spec: &PositiveDrillSpec) -> DrillReport {
    let fixture_path = corpus_dir.join(&spec.fixture);
    let outcome = evaluate_positive(&fixture_path, spec);
    DrillReport {
        drill_id: spec.drill_id.clone(),
        positive: true,
        kind: spec.kind.clone(),
        command_id: spec.expected_command_id.clone(),
        fixture_path,
        outcome,
    }
}

fn run_negative(corpus_dir: &Path, spec: &NegativeDrillSpec) -> DrillReport {
    let fixture_path = corpus_dir.join(&spec.fixture);
    let outcome = evaluate_negative(&fixture_path, spec);
    DrillReport {
        drill_id: spec.drill_id.clone(),
        positive: false,
        kind: spec.kind.clone(),
        command_id: String::new(),
        fixture_path,
        outcome,
    }
}

fn load_scenario(
    fixture_path: &Path,
    kind: &str,
) -> Result<CommandAuthorityScenarioRecord, DrillFailureReason> {
    let payload = std::fs::read_to_string(fixture_path)
        .map_err(|err| DrillFailureReason::FixtureRead(err.to_string()))?;
    if let Some(token) = forbidden_token(&payload) {
        return Err(DrillFailureReason::RawExportToken(token));
    }
    if kind != drill_kind::COMMAND_AUTHORITY_SCENARIO {
        return Err(DrillFailureReason::UnknownKind(kind.to_string()));
    }
    serde_json::from_str(&payload).map_err(|err| DrillFailureReason::Parse(err.to_string()))
}

fn evaluate_positive(fixture_path: &Path, spec: &PositiveDrillSpec) -> DrillOutcome {
    let record = match load_scenario(fixture_path, &spec.kind) {
        Ok(record) => record,
        Err(reason) => return DrillOutcome::Fail(reason),
    };
    if let Err(err) = record.validate() {
        return DrillOutcome::Fail(DrillFailureReason::Validation(err));
    }
    if let Err(reason) = check_expectations(&record, spec) {
        return DrillOutcome::Fail(DrillFailureReason::Expectation(reason));
    }
    DrillOutcome::Pass
}

fn evaluate_negative(fixture_path: &Path, spec: &NegativeDrillSpec) -> DrillOutcome {
    let record = match load_scenario(fixture_path, &spec.kind) {
        Ok(record) => record,
        Err(reason) => return DrillOutcome::Fail(reason),
    };
    match record.validate() {
        Ok(()) => DrillOutcome::Fail(DrillFailureReason::NegativeAccepted),
        Err(actual) => {
            if actual.contains(&spec.expected_failure_substring) {
                DrillOutcome::Pass
            } else {
                DrillOutcome::Fail(DrillFailureReason::NegativeWrongMessage {
                    expected: spec.expected_failure_substring.clone(),
                    actual,
                })
            }
        }
    }
}

fn check_expectations(
    record: &CommandAuthorityScenarioRecord,
    spec: &PositiveDrillSpec,
) -> Result<(), String> {
    let projection = record.project();

    expect_eq(
        "command_id",
        &projection.command_id,
        &spec.expected_command_id,
    )?;
    expect_eq(
        "lifecycle_state",
        &projection.lifecycle_state,
        &spec.expected_lifecycle_state,
    )?;
    expect_eq(
        "preview_class",
        &projection.preview_class,
        &spec.expected_preview_class,
    )?;
    expect_eq(
        "approval_posture_class",
        &projection.approval_posture_class,
        &spec.expected_approval_posture_class,
    )?;
    expect_eq(
        "agreed_enablement_decision_class",
        &projection.agreed_enablement_decision_class,
        &spec.expected_enablement_decision_class,
    )?;

    if !projection.parity_clean {
        return Err(format!(
            "scenario {} did not project a clean parity verdict",
            spec.expected_command_id
        ));
    }

    for surface in &spec.expected_surface_classes {
        if !projection
            .surface_classes_covered
            .iter()
            .any(|s| s == surface)
        {
            return Err(format!(
                "expected surface class `{surface}` not covered; observed {:?}",
                projection.surface_classes_covered
            ));
        }
    }

    for label in &spec.expected_automation_labels {
        if !projection.automation_labels.iter().any(|l| l == label) {
            return Err(format!(
                "expected automation label `{label}` not present; observed {:?}",
                projection.automation_labels
            ));
        }
    }

    if projection.lineage_complete != spec.expected_lineage_complete {
        return Err(format!(
            "lineage_complete mismatch: observed {}, expected {}",
            projection.lineage_complete, spec.expected_lineage_complete
        ));
    }
    if projection.rollback_required != spec.expected_rollback_required {
        return Err(format!(
            "rollback_required mismatch: observed {}, expected {}",
            projection.rollback_required, spec.expected_rollback_required
        ));
    }

    Ok(())
}

fn forbidden_token(payload: &str) -> Option<String> {
    FORBIDDEN_RAW_TOKENS
        .iter()
        .find(|token| payload.contains(**token))
        .map(|token| (*token).to_string())
}

fn expect_eq(field: &str, observed: &str, expected: &str) -> Result<(), String> {
    if observed == expected {
        Ok(())
    } else {
        Err(format!(
            "{field} mismatch: observed `{observed}`, expected `{expected}`"
        ))
    }
}
