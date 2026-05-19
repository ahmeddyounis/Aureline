//! Drill runner for the repo-topology conformance corpus.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_workspace::{
    BodyExportPosture, FetchDepthDescriptor, FullCoverageBlocker, LfsHydrationDescriptor,
    MutationTarget, RepoRootDescriptor, RepoRootKind, RepoTopologyBetaInputs,
    RepoTopologyBetaProjection, RepoTopologySurface, SubmoduleLink,
};
use serde::Deserialize;

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
    /// Positive drill failed JSON parsing.
    PositiveParseFailed(String),
    /// Positive drill failed projection.
    PositiveProjectionFailed(String),
    /// Positive drill produced an unexpected surface token.
    SurfaceMismatch { expected: String, actual: String },
    /// Positive drill produced an unexpected `repo_root_kind`.
    RepoRootKindMismatch { expected: String, actual: String },
    /// Positive drill `may_claim_full_coverage` did not match.
    MayClaimFullCoverageMismatch { expected: bool, actual: bool },
    /// Positive drill `full_coverage_blockers` did not match.
    FullCoverageBlockersMismatch {
        expected: Vec<String>,
        actual: Vec<String>,
    },
    /// Positive drill `required_affordances` did not match.
    RequiredAffordancesMismatch {
        expected: Vec<String>,
        actual: Vec<String>,
    },
    /// Positive drill `mutation_target` did not match.
    MutationTargetMismatch { expected: String, actual: String },
    /// Positive drill `body_export_posture` did not match.
    BodyExportPostureMismatch { expected: String, actual: String },
    /// Positive drill `honesty_labels` did not match.
    HonestyLabelsMismatch {
        expected: Vec<String>,
        actual: Vec<String>,
    },
    /// Positive drill leaked a raw export flag through the fixture.
    RawExportLeak { field: String },
    /// Negative drill unexpectedly projected successfully.
    NegativeDrillAccepted,
    /// Negative drill failed for the wrong reason.
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

#[derive(Debug, Clone, Deserialize)]
struct DrillFixture {
    surface: String,
    repo_root: RepoRootDescriptor,
    #[serde(default)]
    fetch_depth: Option<FetchDepthDescriptor>,
    #[serde(default)]
    submodule_links: Vec<SubmoduleLink>,
    #[serde(default)]
    lfs_hydration: Option<LfsHydrationDescriptor>,
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

    if let Some(field) = scan_for_raw_export_leak(&payload) {
        return DrillReport {
            drill_id: spec.drill_id.clone(),
            fixture_path,
            positive: true,
            outcome: DrillOutcome::Fail(DrillFailureReason::RawExportLeak { field }),
        };
    }

    let fixture: DrillFixture = match serde_json::from_str(&payload) {
        Ok(fixture) => fixture,
        Err(err) => {
            return DrillReport {
                drill_id: spec.drill_id.clone(),
                fixture_path,
                positive: true,
                outcome: DrillOutcome::Fail(DrillFailureReason::PositiveParseFailed(
                    err.to_string(),
                )),
            };
        }
    };

    if fixture.surface != spec.expected_surface {
        return DrillReport {
            drill_id: spec.drill_id.clone(),
            fixture_path,
            positive: true,
            outcome: DrillOutcome::Fail(DrillFailureReason::SurfaceMismatch {
                expected: spec.expected_surface.clone(),
                actual: fixture.surface,
            }),
        };
    }

    let surface = match surface_from_token(&fixture.surface) {
        Some(surface) => surface,
        None => {
            return DrillReport {
                drill_id: spec.drill_id.clone(),
                fixture_path,
                positive: true,
                outcome: DrillOutcome::Fail(DrillFailureReason::SurfaceMismatch {
                    expected: spec.expected_surface.clone(),
                    actual: fixture.surface,
                }),
            };
        }
    };

    let projection = match RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
        repo_root: &fixture.repo_root,
        fetch_depth: fixture.fetch_depth.as_ref(),
        submodule_links: &fixture.submodule_links,
        lfs_hydration: fixture.lfs_hydration.as_ref(),
        surface,
    }) {
        Ok(projection) => projection,
        Err(err) => {
            return DrillReport {
                drill_id: spec.drill_id.clone(),
                fixture_path,
                positive: true,
                outcome: DrillOutcome::Fail(DrillFailureReason::PositiveProjectionFailed(
                    err.to_string(),
                )),
            };
        }
    };

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

    let fixture: DrillFixture = match serde_json::from_str(&payload) {
        Ok(fixture) => fixture,
        Err(err) => {
            let actual = err.to_string();
            if actual.contains(&spec.expected_failure_substring) {
                return DrillReport {
                    drill_id: spec.drill_id.clone(),
                    fixture_path,
                    positive: false,
                    outcome: DrillOutcome::Pass,
                };
            }
            return DrillReport {
                drill_id: spec.drill_id.clone(),
                fixture_path,
                positive: false,
                outcome: DrillOutcome::Fail(DrillFailureReason::NegativeDrillWrongReason {
                    expected_substring: spec.expected_failure_substring.clone(),
                    actual_message: actual,
                }),
            };
        }
    };

    let surface = surface_from_token(&fixture.surface).unwrap_or(RepoTopologySurface::Workspace);

    let result = RepoTopologyBetaProjection::project(RepoTopologyBetaInputs {
        repo_root: &fixture.repo_root,
        fetch_depth: fixture.fetch_depth.as_ref(),
        submodule_links: &fixture.submodule_links,
        lfs_hydration: fixture.lfs_hydration.as_ref(),
        surface,
    });

    match result {
        Ok(_) => DrillReport {
            drill_id: spec.drill_id.clone(),
            fixture_path,
            positive: false,
            outcome: DrillOutcome::Fail(DrillFailureReason::NegativeDrillAccepted),
        },
        Err(err) => {
            let actual = err.to_string();
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
    projection: &RepoTopologyBetaProjection,
) -> Result<(), DrillFailureReason> {
    let actual_surface = surface_token(projection.surface);
    if actual_surface != spec.expected_surface {
        return Err(DrillFailureReason::SurfaceMismatch {
            expected: spec.expected_surface.clone(),
            actual: actual_surface.to_string(),
        });
    }

    let actual_root_kind = repo_root_kind_token(projection.repo_root_kind);
    if actual_root_kind != spec.expected_repo_root_kind {
        return Err(DrillFailureReason::RepoRootKindMismatch {
            expected: spec.expected_repo_root_kind.clone(),
            actual: actual_root_kind.to_string(),
        });
    }

    if projection.may_claim_full_coverage != spec.expected_may_claim_full_coverage {
        return Err(DrillFailureReason::MayClaimFullCoverageMismatch {
            expected: spec.expected_may_claim_full_coverage,
            actual: projection.may_claim_full_coverage,
        });
    }

    let actual_blockers: Vec<String> = projection
        .full_coverage_blockers
        .iter()
        .map(|b| blocker_token(*b).to_string())
        .collect();
    if actual_blockers != spec.expected_full_coverage_blockers {
        return Err(DrillFailureReason::FullCoverageBlockersMismatch {
            expected: spec.expected_full_coverage_blockers.clone(),
            actual: actual_blockers,
        });
    }

    let actual_affordances: Vec<String> = projection
        .required_affordances
        .iter()
        .map(|a| a.as_str().to_string())
        .collect();
    if actual_affordances != spec.expected_required_affordances {
        return Err(DrillFailureReason::RequiredAffordancesMismatch {
            expected: spec.expected_required_affordances.clone(),
            actual: actual_affordances,
        });
    }

    let actual_target = mutation_target_token(projection.mutation_target).to_string();
    if actual_target != spec.expected_mutation_target {
        return Err(DrillFailureReason::MutationTargetMismatch {
            expected: spec.expected_mutation_target.clone(),
            actual: actual_target,
        });
    }

    let actual_posture = body_export_token(projection.body_export_posture).to_string();
    if actual_posture != spec.expected_body_export_posture {
        return Err(DrillFailureReason::BodyExportPostureMismatch {
            expected: spec.expected_body_export_posture.clone(),
            actual: actual_posture,
        });
    }

    if projection.honesty_labels != spec.expected_honesty_labels {
        return Err(DrillFailureReason::HonestyLabelsMismatch {
            expected: spec.expected_honesty_labels.clone(),
            actual: projection.honesty_labels.clone(),
        });
    }

    Ok(())
}

fn surface_from_token(token: &str) -> Option<RepoTopologySurface> {
    Some(match token {
        "workspace" => RepoTopologySurface::Workspace,
        "search" => RepoTopologySurface::Search,
        "graph" => RepoTopologySurface::Graph,
        "blame" => RepoTopologySurface::Blame,
        "review" => RepoTopologySurface::Review,
        "ai" => RepoTopologySurface::Ai,
        "execution" => RepoTopologySurface::Execution,
        "publish" => RepoTopologySurface::Publish,
        "support" => RepoTopologySurface::Support,
        "migration" => RepoTopologySurface::Migration,
        _ => return None,
    })
}

fn surface_token(surface: RepoTopologySurface) -> &'static str {
    surface.as_str()
}

fn repo_root_kind_token(kind: RepoRootKind) -> &'static str {
    kind.as_str()
}

fn blocker_token(blocker: FullCoverageBlocker) -> &'static str {
    blocker.as_str()
}

fn mutation_target_token(target: MutationTarget) -> &'static str {
    match target {
        MutationTarget::ParentRoot => "parent_root",
        MutationTarget::ChildRoot => "child_root",
        MutationTarget::SwitchRootRequired => "switch_root_required",
        MutationTarget::ReadOnlyUntilHydrated => "read_only_until_hydrated",
        MutationTarget::ReadOnlyUntilInitialized => "read_only_until_initialized",
        MutationTarget::PolicyBlocked => "policy_blocked",
    }
}

fn body_export_token(posture: BodyExportPosture) -> &'static str {
    match posture {
        BodyExportPosture::HydratedBytesAllowed => "hydrated_bytes_allowed",
        BodyExportPosture::PointerMetadataOnly => "pointer_metadata_only",
        BodyExportPosture::BlockedByPolicy => "blocked_by_policy",
        BodyExportPosture::Unavailable => "unavailable",
    }
}

/// Scans a fixture payload for raw-body export flags. The closed
/// repo-topology vocabulary never carries raw bytes; if a future
/// fixture ever names one of these flags, the corpus fails before the
/// drill runs.
fn scan_for_raw_export_leak(payload: &str) -> Option<String> {
    const FORBIDDEN: &[&str] = &[
        "raw_path_export_allowed",
        "raw_remote_url_export_allowed",
        "raw_object_bytes_export_allowed",
        "raw_blob_body_export_allowed",
        "raw_pointer_body_export_allowed",
    ];
    for needle in FORBIDDEN {
        if payload.contains(needle) {
            return Some((*needle).to_string());
        }
    }
    None
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
