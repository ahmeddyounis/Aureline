//! Loader + runner for the workflow-bundle lifecycle drill corpus.
//!
//! [`load_corpus`] reads `manifest.json`; [`run_corpus`] replays every drill
//! against the `aureline-workspace::bundles` beta boundary and returns a
//! [`CorpusReport`]. Positive `workflow_bundle_review` drills parse and validate
//! a [`WorkflowBundleReviewRecord`], project it, and must match every pinned
//! expectation — the bundle/source/status classes, the effective badge after
//! evidence/dependency/mirror checks, the support claim, the mirror posture, the
//! granular drift / removal / override counts, the review and resolve actions,
//! the user-asset-preservation and rollback-restoration guarantees, and the
//! capability-dependency and lifecycle-sensitive markers that must propagate
//! across the certification, install/update, and export surfaces. Negative
//! drills must FAIL validation with an error whose message contains the recorded
//! substring.

use std::path::{Path, PathBuf};

use aureline_workspace::WorkflowBundleReviewRecord;

use super::manifest::{
    drill_kind, CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, MANIFEST_FILE_NAME,
};

/// Raw-content tokens that must never appear in a corpus fixture. Their presence
/// would mean a fixture is leaking an actual secret, private key, credential, or
/// absolute local path across the support-safe review boundary. (The boolean
/// `raw_secret_export_allowed` flag is a legitimate field and is governed by the
/// runtime validator, not this scan.)
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
    /// Source class (positive drills only; empty for negatives).
    pub source_class: String,
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
    let manifest = load_corpus(&corpus_dir)
        .unwrap_or_else(|err| panic!("workflow-bundle lifecycle corpus manifest must load: {err}"));
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
        source_class: spec.source_class.clone(),
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
        source_class: String::new(),
        fixture_path,
        outcome,
    }
}

fn evaluate_positive(fixture_path: &Path, spec: &PositiveDrillSpec) -> DrillOutcome {
    let payload = match std::fs::read_to_string(fixture_path) {
        Ok(text) => text,
        Err(err) => return DrillOutcome::Fail(DrillFailureReason::FixtureRead(err.to_string())),
    };
    if let Some(token) = forbidden_token(&payload) {
        return DrillOutcome::Fail(DrillFailureReason::RawExportToken(token));
    }
    if spec.kind != drill_kind::WORKFLOW_BUNDLE_REVIEW {
        return DrillOutcome::Fail(DrillFailureReason::UnknownKind(spec.kind.clone()));
    }

    let record: WorkflowBundleReviewRecord = match serde_json::from_str(&payload) {
        Ok(record) => record,
        Err(err) => return DrillOutcome::Fail(DrillFailureReason::Parse(err.to_string())),
    };
    if let Err(err) = record.validate() {
        return DrillOutcome::Fail(DrillFailureReason::Validation(err.to_string()));
    }
    if let Err(reason) = check_expectations(&record, spec) {
        return DrillOutcome::Fail(DrillFailureReason::Expectation(reason));
    }
    DrillOutcome::Pass
}

fn evaluate_negative(fixture_path: &Path, spec: &NegativeDrillSpec) -> DrillOutcome {
    let payload = match std::fs::read_to_string(fixture_path) {
        Ok(text) => text,
        Err(err) => return DrillOutcome::Fail(DrillFailureReason::FixtureRead(err.to_string())),
    };
    if let Some(token) = forbidden_token(&payload) {
        return DrillOutcome::Fail(DrillFailureReason::RawExportToken(token));
    }
    if spec.kind != drill_kind::WORKFLOW_BUNDLE_REVIEW {
        return DrillOutcome::Fail(DrillFailureReason::UnknownKind(spec.kind.clone()));
    }

    let record: WorkflowBundleReviewRecord = match serde_json::from_str(&payload) {
        Ok(record) => record,
        Err(err) => return DrillOutcome::Fail(DrillFailureReason::Parse(err.to_string())),
    };
    match record.validate() {
        Ok(()) => DrillOutcome::Fail(DrillFailureReason::NegativeAccepted),
        Err(err) => {
            let actual = err.to_string();
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
    record: &WorkflowBundleReviewRecord,
    spec: &PositiveDrillSpec,
) -> Result<(), String> {
    let projection = record.project();

    expect_eq(
        "bundle_class",
        &projection.bundle_class,
        &spec.expected_bundle_class,
    )?;
    expect_eq(
        "source_class",
        &projection.bundle_source_class,
        &spec.expected_source_class,
    )?;
    expect_eq(
        "status_class",
        &record.bundle_identity.bundle_status_class,
        &spec.expected_status_class,
    )?;
    expect_eq(
        "support_class",
        &record.bundle_identity.support_class,
        &spec.expected_support_class,
    )?;
    expect_eq(
        "effective_badge_class",
        &projection.effective_badge_class,
        &spec.expected_effective_badge_class,
    )?;
    expect_eq(
        "support_claim_class",
        &record.certification.support_claim_class,
        &spec.expected_support_claim_class,
    )?;
    expect_eq(
        "evidence_freshness_class",
        &record.certification.evidence_freshness_class,
        &spec.expected_evidence_freshness_class,
    )?;
    expect_eq(
        "certification_state_class",
        &record.certification.certification_state_class,
        &spec.expected_certification_state_class,
    )?;
    if record.certification.retest_required != spec.expected_retest_required {
        return Err(format!(
            "retest_required mismatch: observed {}, expected {}",
            record.certification.retest_required, spec.expected_retest_required
        ));
    }
    expect_eq(
        "mirror_posture_class",
        &record.mirror_offline.posture_class,
        &spec.expected_mirror_posture_class,
    )?;

    if spec.expected_required_diff_axes_complete
        && !projection.missing_required_diff_axes.is_empty()
    {
        return Err(format!(
            "install/update review is missing required diff axes: {:?}",
            projection.missing_required_diff_axes
        ));
    }
    if projection.guardrails_pass != spec.expected_guardrails_pass {
        return Err(format!(
            "guardrails_pass mismatch: observed {}, expected {}",
            projection.guardrails_pass, spec.expected_guardrails_pass
        ));
    }
    if projection.raw_export_allowed != spec.expected_raw_export_allowed {
        return Err(format!(
            "raw_export_allowed mismatch: observed {}, expected {}",
            projection.raw_export_allowed, spec.expected_raw_export_allowed
        ));
    }

    expect_count(
        "drift_entry_count",
        projection.drift_entry_count,
        spec.expected_drift_entry_count,
    )?;
    expect_count(
        "removable_asset_count",
        projection.removable_asset_count,
        spec.expected_removable_asset_count,
    )?;
    expect_count(
        "retained_override_count",
        projection.retained_override_count,
        spec.expected_retained_override_count,
    )?;

    for action in &spec.expected_review_actions_present {
        if !projection.review_actions.iter().any(|a| a == action) {
            return Err(format!(
                "expected review action `{action}` not present; observed {:?}",
                projection.review_actions
            ));
        }
    }
    for action in &spec.expected_resolve_actions_present {
        if !projection.resolve_actions.iter().any(|a| a == action) {
            return Err(format!(
                "expected resolve action `{action}` not present; observed {:?}",
                projection.resolve_actions
            ));
        }
    }

    if spec.expected_preserves_user_owned_assets {
        check_preserves_user_owned_assets(record)?;
    }
    if spec.expected_rollback_restores_bundle_owned {
        check_rollback_restores_bundle_owned(record)?;
    }

    for marker in &spec.expected_capability_dependency_markers {
        check_capability_marker_propagation(record, marker)?;
    }
    for dependency in &spec.expected_lifecycle_sensitive_dependencies {
        check_lifecycle_dependency_propagation(record, dependency)?;
    }

    Ok(())
}

/// Removal must keep every user-owned asset non-deletable, must actually carry a
/// user-owned asset (so the drill exercises preservation), and must retain at
/// least one local override.
fn check_preserves_user_owned_assets(record: &WorkflowBundleReviewRecord) -> Result<(), String> {
    let assets = &record.remove_rollback_review.removable_assets;
    let user_owned: Vec<_> = assets
        .iter()
        .filter(|asset| asset.ownership_class == "user_owned")
        .collect();
    if user_owned.is_empty() {
        return Err(
            "drill claims to preserve user assets but lists no user_owned removable asset"
                .to_string(),
        );
    }
    for asset in user_owned {
        if asset.safe_to_remove_class != "not_safe_to_remove_user_owned" {
            return Err(format!(
                "user_owned asset {} is not protected from removal (safe_to_remove_class={})",
                asset.asset_ref, asset.safe_to_remove_class
            ));
        }
        if !asset.review_required {
            return Err(format!(
                "user_owned asset {} must require review before removal",
                asset.asset_ref
            ));
        }
    }
    if record
        .remove_rollback_review
        .retained_local_overrides
        .is_empty()
    {
        return Err("removal must retain at least one local override".to_string());
    }
    Ok(())
}

/// The install/update rollback checkpoint must be reversible (carry a checkpoint
/// ref) and name the bundle-owned axes it restores.
fn check_rollback_restores_bundle_owned(record: &WorkflowBundleReviewRecord) -> Result<(), String> {
    let checkpoint = &record.install_update_review.rollback_checkpoint;
    if checkpoint.checkpoint_ref.is_none() {
        return Err("rollback checkpoint must carry a checkpoint_ref to be restorable".to_string());
    }
    if checkpoint.restorable_axes.is_empty() {
        return Err("rollback checkpoint must name the bundle-owned axes it restores".to_string());
    }
    if !checkpoint.attributable_to_review {
        return Err("rollback checkpoint must be attributable to this review".to_string());
    }
    Ok(())
}

/// A capability-dependency marker must propagate across the certification
/// evidence, the install/update review sheet, and the support/diagnostics export
/// so no consumer surface can silently drop it.
fn check_capability_marker_propagation(
    record: &WorkflowBundleReviewRecord,
    marker: &str,
) -> Result<(), String> {
    if !record
        .certification
        .compatibility_evidence_refs
        .iter()
        .any(|r| r == marker)
    {
        return Err(format!(
            "capability marker `{marker}` is missing from certification evidence refs"
        ));
    }
    if !record
        .install_update_review
        .diff_entries
        .iter()
        .any(|entry| entry.subject_ref == marker)
    {
        return Err(format!(
            "capability marker `{marker}` is missing from the install/update review sheet"
        ));
    }
    let in_export = record
        .support_export
        .export_packet_refs
        .iter()
        .chain(record.support_export.diagnostics_refs.iter())
        .chain(record.support_export.cli_headless_refs.iter())
        .any(|r| r == marker);
    if !in_export {
        return Err(format!(
            "capability marker `{marker}` is missing from the support/diagnostics export"
        ));
    }
    Ok(())
}

/// A lifecycle-sensitive dependency must surface both on a drift row (so the
/// user sees the lifecycle hazard) and on the install/update review sheet.
fn check_lifecycle_dependency_propagation(
    record: &WorkflowBundleReviewRecord,
    dependency: &str,
) -> Result<(), String> {
    if !record
        .drift_override_review
        .drift_entries
        .iter()
        .any(|entry| entry.subject_ref == dependency)
    {
        return Err(format!(
            "lifecycle-sensitive dependency `{dependency}` is missing from the drift review"
        ));
    }
    if !record
        .install_update_review
        .diff_entries
        .iter()
        .any(|entry| entry.subject_ref == dependency)
    {
        return Err(format!(
            "lifecycle-sensitive dependency `{dependency}` is missing from the install/update sheet"
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

fn expect_count(field: &str, observed: usize, expected: usize) -> Result<(), String> {
    if observed == expected {
        Ok(())
    } else {
        Err(format!(
            "{field} mismatch: observed {observed}, expected {expected}"
        ))
    }
}
