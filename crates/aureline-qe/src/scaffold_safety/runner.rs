//! Drill runner for the scaffold and generated-project safety corpus.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_workspace::{
    ScaffoldPlanRecord, ScaffoldRunRecord, ScaffoldSafetyBetaInputs, ScaffoldSafetyBetaProjection,
    ScaffoldSurface, TemplateGeneratorDescriptor,
};
use serde::Deserialize;

use super::manifest::{
    CorpusManifest, GuardrailExpectations, NegativeDrillSpec, PositiveDrillSpec, CORPUS_DIR_REL,
    MANIFEST_FILE_NAME,
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
    /// A projected scalar token did not match the manifest expectation.
    ScalarMismatch {
        field: &'static str,
        expected: String,
        actual: String,
    },
    /// A projected boolean did not match the manifest expectation.
    BoolMismatch {
        field: &'static str,
        expected: bool,
        actual: bool,
    },
    /// A projected ordered list did not match the manifest expectation.
    ListMismatch {
        field: &'static str,
        expected: Vec<String>,
        actual: Vec<String>,
    },
    /// The generated-project lineage chain could not be reconstructed from a
    /// run-bearing drill.
    LineageBroken { detail: String },
    /// Positive drill leaked a raw-export flag through the fixture.
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
    /// Per-drill reports in manifest order (positive drills first, then
    /// negatives).
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

/// In-fixture scenario: one signed descriptor, one scaffold plan, and an
/// optional scaffold run. The top-level `__fixture__` prelude (scenario
/// metadata) is ignored here; the manifest, not the fixture, owns the pinned
/// expectations.
#[derive(Debug, Clone, Deserialize)]
struct DrillFixture {
    surface: String,
    descriptor: TemplateGeneratorDescriptor,
    plan: ScaffoldPlanRecord,
    #[serde(default)]
    run: Option<ScaffoldRunRecord>,
}

fn run_positive_drill(corpus_dir: &Path, spec: &PositiveDrillSpec) -> DrillReport {
    let fixture_path = corpus_dir.join(&spec.fixture);
    let fail = |reason: DrillFailureReason| DrillReport {
        drill_id: spec.drill_id.clone(),
        fixture_path: fixture_path.clone(),
        positive: true,
        outcome: DrillOutcome::Fail(reason),
    };

    let payload = match fs::read_to_string(&fixture_path) {
        Ok(payload) => payload,
        Err(err) => return fail(DrillFailureReason::FixtureIo(err.to_string())),
    };

    if let Some(field) = scan_for_raw_export_leak(&payload) {
        return fail(DrillFailureReason::RawExportLeak { field });
    }

    let fixture: DrillFixture = match serde_json::from_str(&payload) {
        Ok(fixture) => fixture,
        Err(err) => return fail(DrillFailureReason::PositiveParseFailed(err.to_string())),
    };

    if fixture.surface != spec.expected_surface {
        return fail(DrillFailureReason::ScalarMismatch {
            field: "surface",
            expected: spec.expected_surface.clone(),
            actual: fixture.surface,
        });
    }

    let surface = match surface_from_token(&fixture.surface) {
        Some(surface) => surface,
        None => {
            return fail(DrillFailureReason::ScalarMismatch {
                field: "surface",
                expected: spec.expected_surface.clone(),
                actual: fixture.surface,
            })
        }
    };

    let projection = match ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
        descriptor: &fixture.descriptor,
        plan: &fixture.plan,
        run: fixture.run.as_ref(),
        surface,
    }) {
        Ok(projection) => projection,
        Err(err) => {
            return fail(DrillFailureReason::PositiveProjectionFailed(
                err.to_string(),
            ))
        }
    };

    if let Err(reason) = assert_positive_expectations(spec, &fixture, &projection) {
        return fail(reason);
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
    let report = |outcome: DrillOutcome| DrillReport {
        drill_id: spec.drill_id.clone(),
        fixture_path: fixture_path.clone(),
        positive: false,
        outcome,
    };

    let payload = match fs::read_to_string(&fixture_path) {
        Ok(payload) => payload,
        Err(err) => {
            return report(DrillOutcome::Fail(DrillFailureReason::FixtureIo(
                err.to_string(),
            )))
        }
    };

    let fixture: DrillFixture = match serde_json::from_str(&payload) {
        Ok(fixture) => fixture,
        Err(err) => {
            let actual = err.to_string();
            if actual.contains(&spec.expected_failure_substring) {
                return report(DrillOutcome::Pass);
            }
            return report(DrillOutcome::Fail(
                DrillFailureReason::NegativeDrillWrongReason {
                    expected_substring: spec.expected_failure_substring.clone(),
                    actual_message: actual,
                },
            ));
        }
    };

    let surface = surface_from_token(&fixture.surface).unwrap_or(ScaffoldSurface::Support);

    let result = ScaffoldSafetyBetaProjection::project(ScaffoldSafetyBetaInputs {
        descriptor: &fixture.descriptor,
        plan: &fixture.plan,
        run: fixture.run.as_ref(),
        surface,
    });

    match result {
        Ok(_) => report(DrillOutcome::Fail(
            DrillFailureReason::NegativeDrillAccepted,
        )),
        Err(err) => {
            let actual = err.to_string();
            if actual.contains(&spec.expected_failure_substring) {
                report(DrillOutcome::Pass)
            } else {
                report(DrillOutcome::Fail(
                    DrillFailureReason::NegativeDrillWrongReason {
                        expected_substring: spec.expected_failure_substring.clone(),
                        actual_message: actual,
                    },
                ))
            }
        }
    }
}

fn assert_positive_expectations(
    spec: &PositiveDrillSpec,
    fixture: &DrillFixture,
    projection: &ScaffoldSafetyBetaProjection,
) -> Result<(), DrillFailureReason> {
    scalar(
        "surface",
        &spec.expected_surface,
        projection.surface.as_str(),
    )?;
    scalar(
        "provider_class",
        &spec.expected_provider_class,
        projection.provider_class.as_str(),
    )?;
    scalar(
        "signature_state",
        &spec.expected_signature_state,
        projection.signature_state.as_str(),
    )?;
    scalar(
        "generation_kind",
        &spec.expected_generation_kind,
        projection.generation_kind.as_str(),
    )?;
    scalar(
        "generation_verb",
        &spec.expected_generation_verb,
        projection.generation_verb.as_str(),
    )?;
    scalar(
        "egress_posture",
        &spec.expected_egress_posture,
        projection.egress_posture.as_str(),
    )?;
    scalar(
        "trust_expectation",
        &spec.expected_trust_expectation,
        projection.trust_expectation.as_str(),
    )?;
    // Provenance is read straight off the fixture descriptor: the projection
    // preserves provider / signature truth, and the descriptor preserves the
    // source distribution, so a mirrored or offline template can never be
    // flattened into a generic local file in the corpus.
    scalar(
        "source_distribution_class",
        &spec.expected_source_distribution_class,
        fixture
            .descriptor
            .provenance
            .source_distribution_class
            .as_str(),
    )?;

    let observed_side_effects: Vec<String> = sorted(
        projection
            .declared_side_effects
            .classes
            .iter()
            .map(|class| class.as_str().to_string())
            .collect(),
    );
    list(
        "declared_side_effect_classes",
        &sorted(spec.expected_declared_side_effect_classes.clone()),
        &observed_side_effects,
    )?;

    boolean(
        "create_empty_available",
        spec.expected_create_empty_available,
        projection.setup_handoff.create_empty_available,
    )?;
    boolean(
        "set_up_later_available",
        spec.expected_set_up_later_available,
        projection.setup_handoff.set_up_later_available,
    )?;
    scalar(
        "rollback_boundary",
        &spec.expected_rollback_boundary,
        projection.setup_handoff.rollback_boundary.as_str(),
    )?;
    boolean(
        "rollback_automatic",
        spec.expected_rollback_automatic,
        projection.setup_handoff.rollback_automatic,
    )?;

    boolean(
        "has_run",
        spec.expected_has_run,
        projection.run_summary.is_some(),
    )?;
    let observed_outcome = projection
        .run_summary
        .as_ref()
        .map(|run| run.outcome_class.as_str().to_string());
    if observed_outcome != spec.expected_run_outcome {
        return Err(DrillFailureReason::ScalarMismatch {
            field: "run_outcome",
            expected: spec
                .expected_run_outcome
                .clone()
                .unwrap_or_else(|| "<none>".to_string()),
            actual: observed_outcome.unwrap_or_else(|| "<none>".to_string()),
        });
    }

    let observed_labels: Vec<String> = sorted(
        projection
            .honesty_labels
            .iter()
            .map(|label| label.as_str().to_string())
            .collect(),
    );
    list(
        "honesty_labels",
        &sorted(spec.expected_honesty_labels.clone()),
        &observed_labels,
    )?;

    boolean(
        "guardrails_all_hold",
        spec.expected_guardrails_all_hold,
        projection.guardrails.all_hold(),
    )?;
    if let Some(expected) = spec.expected_guardrails {
        assert_guardrails(expected, projection)?;
    }

    boolean(
        "surface_must_disclose",
        spec.expected_surface_must_disclose,
        projection.surface_must_disclose_generation(),
    )?;

    assert_lineage(fixture, projection)?;

    Ok(())
}

fn assert_guardrails(
    expected: GuardrailExpectations,
    projection: &ScaffoldSafetyBetaProjection,
) -> Result<(), DrillFailureReason> {
    let g = projection.guardrails;
    boolean(
        "guardrails.no_writes_before_review",
        expected.no_writes_before_review,
        g.no_writes_before_review,
    )?;
    boolean(
        "guardrails.side_effects_declared_before_execution",
        expected.side_effects_declared_before_execution,
        g.side_effects_declared_before_execution,
    )?;
    boolean(
        "guardrails.side_effects_attributable_after_rollback",
        expected.side_effects_attributable_after_rollback,
        g.side_effects_attributable_after_rollback,
    )?;
    boolean(
        "guardrails.no_undeclared_hooks_or_bootstrap",
        expected.no_undeclared_hooks_or_bootstrap,
        g.no_undeclared_hooks_or_bootstrap,
    )?;
    boolean(
        "guardrails.generated_output_is_plain_workspace_content",
        expected.generated_output_is_plain_workspace_content,
        g.generated_output_is_plain_workspace_content,
    )?;
    boolean(
        "guardrails.rollback_boundary_visible",
        expected.rollback_boundary_visible,
        g.rollback_boundary_visible,
    )?;
    boolean(
        "guardrails.ai_extension_uses_governed_surface",
        expected.ai_extension_uses_governed_surface,
        g.ai_extension_uses_governed_surface,
    )?;
    Ok(())
}

/// For every run-bearing drill the lineage chain must be reconstructable from
/// the records the projection cites: the projection must bind descriptor and
/// plan refs back to the fixture, expose a run summary, and carry a non-empty
/// generated-project lineage ref. This holds even on failure / partial /
/// caught drills, proving lineage survives rollback, retry, and support
/// capture.
fn assert_lineage(
    fixture: &DrillFixture,
    projection: &ScaffoldSafetyBetaProjection,
) -> Result<(), DrillFailureReason> {
    if projection.descriptor_ref != fixture.descriptor.descriptor_id {
        return Err(DrillFailureReason::LineageBroken {
            detail: format!(
                "projection descriptor ref {} != fixture descriptor {}",
                projection.descriptor_ref, fixture.descriptor.descriptor_id
            ),
        });
    }
    if projection.scaffold_plan_ref != fixture.plan.scaffold_plan_id {
        return Err(DrillFailureReason::LineageBroken {
            detail: format!(
                "projection plan ref {} != fixture plan {}",
                projection.scaffold_plan_ref, fixture.plan.scaffold_plan_id
            ),
        });
    }

    let Some(run) = fixture.run.as_ref() else {
        return Ok(());
    };

    let Some(summary) = projection.run_summary.as_ref() else {
        return Err(DrillFailureReason::LineageBroken {
            detail: "fixture carries a run but the projection has no run summary".to_string(),
        });
    };
    if summary.scaffold_run_ref != run.scaffold_run_id {
        return Err(DrillFailureReason::LineageBroken {
            detail: format!(
                "run summary ref {} != fixture run {}",
                summary.scaffold_run_ref, run.scaffold_run_id
            ),
        });
    }
    if summary.generated_lineage_ref.trim().is_empty() {
        return Err(DrillFailureReason::LineageBroken {
            detail: format!(
                "run {} carries an empty generated lineage ref",
                run.scaffold_run_id
            ),
        });
    }
    if projection.scaffold_run_ref.as_deref() != Some(run.scaffold_run_id.as_str()) {
        return Err(DrillFailureReason::LineageBroken {
            detail: "projection scaffold_run_ref does not echo the fixture run id".to_string(),
        });
    }
    Ok(())
}

fn scalar(field: &'static str, expected: &str, actual: &str) -> Result<(), DrillFailureReason> {
    if expected == actual {
        Ok(())
    } else {
        Err(DrillFailureReason::ScalarMismatch {
            field,
            expected: expected.to_string(),
            actual: actual.to_string(),
        })
    }
}

fn boolean(field: &'static str, expected: bool, actual: bool) -> Result<(), DrillFailureReason> {
    if expected == actual {
        Ok(())
    } else {
        Err(DrillFailureReason::BoolMismatch {
            field,
            expected,
            actual,
        })
    }
}

fn list(
    field: &'static str,
    expected: &[String],
    actual: &[String],
) -> Result<(), DrillFailureReason> {
    if expected == actual {
        Ok(())
    } else {
        Err(DrillFailureReason::ListMismatch {
            field,
            expected: expected.to_vec(),
            actual: actual.to_vec(),
        })
    }
}

fn sorted(mut values: Vec<String>) -> Vec<String> {
    values.sort();
    values
}

fn surface_from_token(token: &str) -> Option<ScaffoldSurface> {
    Some(match token {
        "start_center" => ScaffoldSurface::StartCenter,
        "command_palette" => ScaffoldSurface::CommandPalette,
        "generator_preview" => ScaffoldSurface::GeneratorPreview,
        "ai_assist" => ScaffoldSurface::AiAssist,
        "extension" => ScaffoldSurface::Extension,
        "cli_headless" => ScaffoldSurface::CliHeadless,
        "support" => ScaffoldSurface::Support,
        _ => return None,
    })
}

/// Scans a fixture payload for raw-export flags. The closed scaffold-safety
/// vocabulary carries only opaque refs and typed labels; if a future fixture
/// ever names one of these flags, the corpus fails before the drill runs so
/// the redaction contract lives on the corpus itself, not only on individual
/// surface read paths.
fn scan_for_raw_export_leak(payload: &str) -> Option<String> {
    const FORBIDDEN: &[&str] = &[
        "raw_template_bytes_export_allowed",
        "raw_generated_content_export_allowed",
        "raw_path_export_allowed",
        "raw_credential_export_allowed",
        "raw_secret_export_allowed",
        "raw_token_export_allowed",
        "raw_command_export_allowed",
        "raw_remote_url_export_allowed",
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
