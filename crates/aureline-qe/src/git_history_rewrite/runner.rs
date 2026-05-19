//! Drill runner for the history-rewrite conformance corpus.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use aureline_git::{
    HistoryRewriteError, HistoryRewriteProjection, HistoryRewriteRecord,
    HISTORY_REWRITE_OPERATION_KINDS,
};

use super::manifest::{
    CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, CORPUS_DIR_REL, MANIFEST_FILE_NAME,
};

/// Per-drill outcome classification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrillOutcome {
    /// Drill passed every expectation.
    Pass,
    /// Drill failed; the [`DrillFailureReason`] explains why.
    Fail(DrillFailureReason),
}

/// Structured failure reason for a drill.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DrillFailureReason {
    /// Fixture file could not be read.
    FixtureIo(String),
    /// Manifest file could not be loaded.
    ManifestLoad(String),
    /// Positive drill failed JSON parsing or contract validation.
    PositiveValidationFailed(String),
    /// Positive drill produced an unexpected record kind.
    RecordKindMismatch { expected: String, actual: String },
    /// Positive drill produced an unexpected operation kind.
    OperationKindMismatch { expected: String, actual: String },
    /// Positive drill produced an unexpected lifecycle state.
    LifecycleMismatch { expected: String, actual: String },
    /// Positive drill carried an operation kind outside the closed
    /// vocabulary published by `aureline-git`.
    OperationKindOutsideVocabulary { actual: String },
    /// Positive drill destructive-gate flag did not match.
    DestructiveGateMismatch { expected: bool, actual: bool },
    /// Positive drill recovery-posture class did not match.
    RecoveryPostureMismatch { expected: String, actual: String },
    /// Positive drill next-safe-path classes did not match.
    NextSafePathSetMismatch {
        missing: Vec<String>,
        unexpected: Vec<String>,
    },
    /// Positive drill blocks summary did not include an expected prefix.
    BlocksSummaryMissingPrefix { missing: Vec<String> },
    /// Positive drill audit-event ids missed an expected event.
    AuditEventsMissing { missing: Vec<String> },
    /// Positive drill leaked a raw export flag.
    RawExportLeak { which: String },
    /// Positive drill consumer surfaces did not include both
    /// `support_export` and `audit_lane`.
    SupportAuditWiringMissing { missing: Vec<String> },
    /// Negative drill unexpectedly parsed and validated.
    NegativeDrillAccepted,
    /// Negative drill failed with the wrong reason.
    NegativeDrillWrongReason {
        expected_substring: String,
        actual_message: String,
    },
}

/// Single drill result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrillReport {
    /// Stable drill id.
    pub drill_id: String,
    /// Resolved fixture path.
    pub fixture_path: PathBuf,
    /// True when the drill is from the positive-drill set.
    pub positive: bool,
    /// Outcome classification.
    pub outcome: DrillOutcome,
}

impl DrillReport {
    /// Returns true when the drill passed.
    pub fn passed(&self) -> bool {
        matches!(self.outcome, DrillOutcome::Pass)
    }
}

/// Corpus-level report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusReport {
    /// Resolved corpus directory.
    pub corpus_dir: PathBuf,
    /// Per-drill reports in manifest order (positive drills first,
    /// then negatives).
    pub drills: Vec<DrillReport>,
}

impl CorpusReport {
    /// True when every drill passed.
    pub fn all_passed(&self) -> bool {
        self.drills.iter().all(DrillReport::passed)
    }

    /// Returns just the failed drills.
    pub fn failures(&self) -> Vec<&DrillReport> {
        self.drills.iter().filter(|drill| !drill.passed()).collect()
    }
}

/// Loads the corpus manifest from the given corpus directory.
pub fn load_corpus(corpus_dir: &Path) -> Result<CorpusManifest, DrillFailureReason> {
    let manifest_path = corpus_dir.join(MANIFEST_FILE_NAME);
    let payload = fs::read_to_string(&manifest_path).map_err(|err| {
        DrillFailureReason::ManifestLoad(format!(
            "failed to read {}: {err}",
            manifest_path.display()
        ))
    })?;
    serde_json::from_str(&payload).map_err(|err| {
        DrillFailureReason::ManifestLoad(format!(
            "failed to parse {}: {err}",
            manifest_path.display()
        ))
    })
}

/// Resolves the corpus directory from a repository root path.
pub fn corpus_dir_from_repo_root(repo_root: &Path) -> PathBuf {
    repo_root.join(CORPUS_DIR_REL)
}

/// Runs the corpus pinned at `repo_root/<CORPUS_DIR_REL>`.
pub fn run_corpus_from_repo_root(repo_root: &Path) -> CorpusReport {
    let corpus_dir = corpus_dir_from_repo_root(repo_root);
    run_corpus(&corpus_dir)
}

/// Runs the corpus pinned at the given corpus directory.
pub fn run_corpus(corpus_dir: &Path) -> CorpusReport {
    let manifest = match load_corpus(corpus_dir) {
        Ok(manifest) => manifest,
        Err(reason) => {
            return CorpusReport {
                corpus_dir: corpus_dir.to_path_buf(),
                drills: vec![DrillReport {
                    drill_id: "manifest".to_string(),
                    fixture_path: corpus_dir.join(MANIFEST_FILE_NAME),
                    positive: true,
                    outcome: DrillOutcome::Fail(reason),
                }],
            };
        }
    };

    let mut drills: Vec<DrillReport> = Vec::new();
    for spec in &manifest.positive_drills {
        drills.push(run_positive_drill(corpus_dir, spec));
    }
    for spec in &manifest.negative_drills {
        drills.push(run_negative_drill(corpus_dir, spec));
    }

    CorpusReport {
        corpus_dir: corpus_dir.to_path_buf(),
        drills,
    }
}

fn run_positive_drill(corpus_dir: &Path, spec: &PositiveDrillSpec) -> DrillReport {
    let fixture_path = corpus_dir.join(&spec.fixture);
    let payload = match fs::read_to_string(&fixture_path) {
        Ok(payload) => payload,
        Err(err) => {
            return DrillReport {
                drill_id: spec.drill_id.clone(),
                fixture_path,
                positive: true,
                outcome: DrillOutcome::Fail(DrillFailureReason::FixtureIo(err.to_string())),
            };
        }
    };

    let record: HistoryRewriteRecord = match serde_json::from_str(&payload) {
        Ok(record) => record,
        Err(err) => {
            return DrillReport {
                drill_id: spec.drill_id.clone(),
                fixture_path,
                positive: true,
                outcome: DrillOutcome::Fail(DrillFailureReason::PositiveValidationFailed(
                    err.to_string(),
                )),
            };
        }
    };

    if let Err(err) = record.validate() {
        return DrillReport {
            drill_id: spec.drill_id.clone(),
            fixture_path,
            positive: true,
            outcome: DrillOutcome::Fail(DrillFailureReason::PositiveValidationFailed(
                err.message().to_string(),
            )),
        };
    }

    let projection = record.project();
    if let Err(reason) = assert_positive_expectations(spec, &projection) {
        return DrillReport {
            drill_id: spec.drill_id.clone(),
            fixture_path,
            positive: true,
            outcome: DrillOutcome::Fail(reason),
        };
    }

    DrillReport {
        drill_id: spec.drill_id.clone(),
        fixture_path,
        positive: true,
        outcome: DrillOutcome::Pass,
    }
}

fn run_negative_drill(corpus_dir: &Path, spec: &NegativeDrillSpec) -> DrillReport {
    let fixture_path = corpus_dir.join(&spec.fixture);
    let payload = match fs::read_to_string(&fixture_path) {
        Ok(payload) => payload,
        Err(err) => {
            return DrillReport {
                drill_id: spec.drill_id.clone(),
                fixture_path,
                positive: false,
                outcome: DrillOutcome::Fail(DrillFailureReason::FixtureIo(err.to_string())),
            };
        }
    };

    let result: Result<HistoryRewriteRecord, _> = serde_json::from_str(&payload);
    let validation_result = match result {
        Ok(record) => record.validate().map_err(HistoryRewriteError::Validation),
        Err(err) => Err(HistoryRewriteError::Json(err.to_string())),
    };

    match validation_result {
        Ok(()) => DrillReport {
            drill_id: spec.drill_id.clone(),
            fixture_path,
            positive: false,
            outcome: DrillOutcome::Fail(DrillFailureReason::NegativeDrillAccepted),
        },
        Err(err) => {
            let actual = err.message().to_string();
            if actual.contains(&spec.expected_failure_substring) {
                DrillReport {
                    drill_id: spec.drill_id.clone(),
                    fixture_path,
                    positive: false,
                    outcome: DrillOutcome::Pass,
                }
            } else {
                DrillReport {
                    drill_id: spec.drill_id.clone(),
                    fixture_path,
                    positive: false,
                    outcome: DrillOutcome::Fail(DrillFailureReason::NegativeDrillWrongReason {
                        expected_substring: spec.expected_failure_substring.clone(),
                        actual_message: actual,
                    }),
                }
            }
        }
    }
}

fn assert_positive_expectations(
    spec: &PositiveDrillSpec,
    projection: &HistoryRewriteProjection,
) -> Result<(), DrillFailureReason> {
    if projection.record_kind != spec.expected_record_kind {
        return Err(DrillFailureReason::RecordKindMismatch {
            expected: spec.expected_record_kind.clone(),
            actual: projection.record_kind.clone(),
        });
    }
    if projection.operation_kind != spec.expected_operation_kind {
        return Err(DrillFailureReason::OperationKindMismatch {
            expected: spec.expected_operation_kind.clone(),
            actual: projection.operation_kind.clone(),
        });
    }
    if !HISTORY_REWRITE_OPERATION_KINDS
        .iter()
        .any(|kind| *kind == projection.operation_kind)
    {
        return Err(DrillFailureReason::OperationKindOutsideVocabulary {
            actual: projection.operation_kind.clone(),
        });
    }
    if projection.lifecycle_state != spec.expected_lifecycle {
        return Err(DrillFailureReason::LifecycleMismatch {
            expected: spec.expected_lifecycle.clone(),
            actual: projection.lifecycle_state.clone(),
        });
    }
    if projection.destructive_gate_satisfied != spec.expected_destructive_gate_satisfied {
        return Err(DrillFailureReason::DestructiveGateMismatch {
            expected: spec.expected_destructive_gate_satisfied,
            actual: projection.destructive_gate_satisfied,
        });
    }
    if projection.recovery_posture_class != spec.expected_recovery_posture_class {
        return Err(DrillFailureReason::RecoveryPostureMismatch {
            expected: spec.expected_recovery_posture_class.clone(),
            actual: projection.recovery_posture_class.clone(),
        });
    }

    if !spec.expected_next_safe_path_classes.is_empty() {
        let expected: BTreeSet<&str> = spec
            .expected_next_safe_path_classes
            .iter()
            .map(String::as_str)
            .collect();
        let actual: BTreeSet<&str> = projection
            .next_safe_path_classes
            .iter()
            .map(String::as_str)
            .collect();
        let missing: Vec<String> = expected
            .difference(&actual)
            .map(|class| (*class).to_string())
            .collect();
        let unexpected: Vec<String> = actual
            .difference(&expected)
            .map(|class| (*class).to_string())
            .collect();
        if !missing.is_empty() || !unexpected.is_empty() {
            return Err(DrillFailureReason::NextSafePathSetMismatch {
                missing,
                unexpected,
            });
        }
    }

    if !spec.expected_blocks_summary_starts.is_empty() {
        let mut missing: Vec<String> = Vec::new();
        for prefix in &spec.expected_blocks_summary_starts {
            if !projection
                .blocks_summary
                .iter()
                .any(|block| block.starts_with(prefix))
            {
                missing.push(prefix.clone());
            }
        }
        if !missing.is_empty() {
            return Err(DrillFailureReason::BlocksSummaryMissingPrefix { missing });
        }
    }

    if !spec.expected_audit_event_ids.is_empty() {
        let mut missing: Vec<String> = Vec::new();
        let mut actual_remaining: Vec<String> = projection.audit_event_ids.clone();
        for expected_event in &spec.expected_audit_event_ids {
            if let Some(idx) = actual_remaining
                .iter()
                .position(|event| event == expected_event)
            {
                actual_remaining.swap_remove(idx);
            } else {
                missing.push(expected_event.clone());
            }
        }
        if !missing.is_empty() {
            return Err(DrillFailureReason::AuditEventsMissing { missing });
        }
    }

    if projection.raw_path_export_allowed {
        return Err(DrillFailureReason::RawExportLeak {
            which: "raw_path_export_allowed".to_string(),
        });
    }
    if projection.raw_branch_name_export_allowed {
        return Err(DrillFailureReason::RawExportLeak {
            which: "raw_branch_name_export_allowed".to_string(),
        });
    }
    if projection.raw_patch_body_export_allowed {
        return Err(DrillFailureReason::RawExportLeak {
            which: "raw_patch_body_export_allowed".to_string(),
        });
    }
    if projection.raw_reflog_body_export_allowed {
        return Err(DrillFailureReason::RawExportLeak {
            which: "raw_reflog_body_export_allowed".to_string(),
        });
    }
    if projection.raw_stash_body_export_allowed {
        return Err(DrillFailureReason::RawExportLeak {
            which: "raw_stash_body_export_allowed".to_string(),
        });
    }

    let mut missing_surfaces: Vec<String> = Vec::new();
    if !projection
        .consumer_surfaces
        .iter()
        .any(|surface| surface == "support_export")
    {
        missing_surfaces.push("support_export".to_string());
    }
    if !projection
        .consumer_surfaces
        .iter()
        .any(|surface| surface == "audit_lane")
    {
        missing_surfaces.push("audit_lane".to_string());
    }
    if !missing_surfaces.is_empty() {
        return Err(DrillFailureReason::SupportAuditWiringMissing {
            missing: missing_surfaces,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
    }

    #[test]
    fn corpus_loads_and_passes() {
        let report = run_corpus_from_repo_root(&repo_root());
        if !report.all_passed() {
            let failures: Vec<String> = report
                .failures()
                .iter()
                .map(|drill| format!("{}: {:?}", drill.drill_id, drill.outcome))
                .collect();
            panic!("corpus had failures: {failures:#?}");
        }
        assert!(
            !report.drills.is_empty(),
            "corpus must publish at least one drill"
        );
    }
}
