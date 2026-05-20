//! Drill runner for the repository-acquisition and bootstrap truth corpus.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_workspace::{
    AcquisitionSurface, BootstrapQueueItemRecord, CheckoutPlanRecord, InterruptedRecovery,
    RepositoryAcquisitionBetaInputs, RepositoryAcquisitionBetaProjection, SourceLocatorRecord,
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
    /// A projected count did not match the manifest expectation.
    CountMismatch {
        field: &'static str,
        expected: u64,
        actual: u64,
    },
    /// The evidence packet did not reconstruct the locator / plan refs or
    /// was not export-safe.
    EvidencePacketBroken { detail: String },
    /// An interrupted-recovery card was not export-safe.
    InterruptedNotExportSafe,
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
        DrillFailureReason::ManifestLoad(format!("failed to read {}: {err}", manifest_path.display()))
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

/// In-fixture scenario: one locator, one plan, and the bootstrap queue.
#[derive(Debug, Clone, Deserialize)]
struct DrillFixture {
    surface: String,
    locator: SourceLocatorRecord,
    plan: CheckoutPlanRecord,
    #[serde(default)]
    bootstrap_items: Vec<BootstrapQueueItemRecord>,
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

    let projection = match RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
        locator: &fixture.locator,
        plan: &fixture.plan,
        bootstrap_items: &fixture.bootstrap_items,
        surface,
    }) {
        Ok(projection) => projection,
        Err(err) => return fail(DrillFailureReason::PositiveProjectionFailed(err.to_string())),
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
        Err(err) => return report(DrillOutcome::Fail(DrillFailureReason::FixtureIo(err.to_string()))),
    };

    let fixture: DrillFixture = match serde_json::from_str(&payload) {
        Ok(fixture) => fixture,
        Err(err) => {
            let actual = err.to_string();
            if actual.contains(&spec.expected_failure_substring) {
                return report(DrillOutcome::Pass);
            }
            return report(DrillOutcome::Fail(DrillFailureReason::NegativeDrillWrongReason {
                expected_substring: spec.expected_failure_substring.clone(),
                actual_message: actual,
            }));
        }
    };

    let surface = surface_from_token(&fixture.surface).unwrap_or(AcquisitionSurface::Support);

    let result = RepositoryAcquisitionBetaProjection::project(RepositoryAcquisitionBetaInputs {
        locator: &fixture.locator,
        plan: &fixture.plan,
        bootstrap_items: &fixture.bootstrap_items,
        surface,
    });

    match result {
        Ok(_) => report(DrillOutcome::Fail(DrillFailureReason::NegativeDrillAccepted)),
        Err(err) => {
            let actual = err.to_string();
            if actual.contains(&spec.expected_failure_substring) {
                report(DrillOutcome::Pass)
            } else {
                report(DrillOutcome::Fail(DrillFailureReason::NegativeDrillWrongReason {
                    expected_substring: spec.expected_failure_substring.clone(),
                    actual_message: actual,
                }))
            }
        }
    }
}

fn assert_positive_expectations(
    spec: &PositiveDrillSpec,
    fixture: &DrillFixture,
    projection: &RepositoryAcquisitionBetaProjection,
) -> Result<(), DrillFailureReason> {
    scalar("surface", &spec.expected_surface, projection.surface.as_str())?;
    scalar(
        "acquisition_verb",
        &spec.expected_acquisition_verb,
        projection.acquisition_verb.as_str(),
    )?;
    scalar(
        "locator_class",
        &spec.expected_locator_class,
        projection.locator_class.as_str(),
    )?;

    let actual_transport = projection.transport_class.map(|t| t.as_str().to_string());
    if actual_transport != spec.expected_transport_class {
        return Err(DrillFailureReason::ScalarMismatch {
            field: "transport_class",
            expected: spec
                .expected_transport_class
                .clone()
                .unwrap_or_else(|| "<none>".to_string()),
            actual: actual_transport.unwrap_or_else(|| "<none>".to_string()),
        });
    }

    scalar(
        "checkout_mode",
        &spec.expected_checkout_mode,
        projection.checkout_shape.mode.as_str(),
    )?;
    boolean(
        "partial_or_sparse",
        spec.expected_partial_or_sparse,
        projection.checkout_shape.partial_or_sparse,
    )?;
    scalar(
        "submodule_policy",
        &spec.expected_submodule_policy,
        projection.checkout_shape.submodule_policy.as_str(),
    )?;
    scalar(
        "lfs_policy",
        &spec.expected_lfs_policy,
        projection.checkout_shape.lfs_policy.as_str(),
    )?;
    scalar(
        "expected_cost_band",
        &spec.expected_cost_band,
        projection.expected_cost_band.as_str(),
    )?;

    scalar(
        "credential_posture",
        &spec.expected_credential_posture,
        projection.credential_posture.posture_class.as_str(),
    )?;
    boolean(
        "credential_reauth_required",
        spec.expected_credential_reauth_required,
        projection.credential_posture.reauth_required,
    )?;

    boolean(
        "interrupted",
        spec.expected_interrupted,
        projection.interrupted_recovery.is_some(),
    )?;
    let observed_branches: Vec<String> = projection
        .interrupted_recovery
        .as_ref()
        .map(|r| r.branches.iter().map(|b| b.as_str().to_string()).collect())
        .unwrap_or_default();
    list(
        "interrupted_branches",
        &spec.expected_interrupted_branches,
        &observed_branches,
    )?;

    if let Some(recovery) = projection.interrupted_recovery.as_ref() {
        assert_interrupted(spec, recovery)?;
    }

    count(
        "manual_followup_count",
        spec.expected_manual_followup_count,
        projection.manual_followups.len() as u64,
    )?;

    let observed_labels: Vec<String> = projection
        .honesty_labels
        .iter()
        .map(|l| l.as_str().to_string())
        .collect();
    list("honesty_labels", &spec.expected_honesty_labels, &observed_labels)?;

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
        projection.surface_must_disclose_acquisition(),
    )?;
    boolean(
        "every_item_attributed",
        spec.expected_every_item_attributed,
        projection.evidence_packet.every_item_attributed,
    )?;

    assert_evidence_packet(fixture, projection)?;

    Ok(())
}

fn assert_interrupted(
    spec: &PositiveDrillSpec,
    recovery: &InterruptedRecovery,
) -> Result<(), DrillFailureReason> {
    // Interrupted recovery state is always export-safe by contract; the
    // corpus pins it so a regression that leaks raw state is caught here.
    if !recovery.export_safe {
        return Err(DrillFailureReason::InterruptedNotExportSafe);
    }
    if let Some(expected) = &spec.expected_discard_posture {
        scalar(
            "discard_posture",
            expected,
            recovery.discard_posture.as_str(),
        )?;
    }
    if let Some(expected) = spec.expected_open_read_only_available {
        boolean(
            "open_read_only_available",
            expected,
            recovery.open_read_only_available,
        )?;
    }
    Ok(())
}

fn assert_guardrails(
    expected: GuardrailExpectations,
    projection: &RepositoryAcquisitionBetaProjection,
) -> Result<(), DrillFailureReason> {
    let g = projection.guardrails;
    boolean(
        "guardrails.clone_not_confused_with_open",
        expected.clone_not_confused_with_open,
        g.clone_not_confused_with_open,
    )?;
    boolean(
        "guardrails.no_implicit_repo_code_execution",
        expected.no_implicit_repo_code_execution,
        g.no_implicit_repo_code_execution,
    )?;
    boolean(
        "guardrails.bootstrap_items_attributed",
        expected.bootstrap_items_attributed,
        g.bootstrap_items_attributed,
    )?;
    boolean(
        "guardrails.browse_safe_inspection_available",
        expected.browse_safe_inspection_available,
        g.browse_safe_inspection_available,
    )?;
    boolean(
        "guardrails.mirror_not_masquerading_as_live",
        expected.mirror_not_masquerading_as_live,
        g.mirror_not_masquerading_as_live,
    )?;
    boolean(
        "guardrails.no_hidden_trust_elevation",
        expected.no_hidden_trust_elevation,
        g.no_hidden_trust_elevation,
    )?;
    Ok(())
}

fn assert_evidence_packet(
    fixture: &DrillFixture,
    projection: &RepositoryAcquisitionBetaProjection,
) -> Result<(), DrillFailureReason> {
    let packet = &projection.evidence_packet;
    if packet.source_locator_ref != fixture.locator.source_locator_id {
        return Err(DrillFailureReason::EvidencePacketBroken {
            detail: format!(
                "evidence locator ref {} != fixture locator {}",
                packet.source_locator_ref, fixture.locator.source_locator_id
            ),
        });
    }
    if packet.checkout_plan_ref != fixture.plan.checkout_plan_id {
        return Err(DrillFailureReason::EvidencePacketBroken {
            detail: format!(
                "evidence plan ref {} != fixture plan {}",
                packet.checkout_plan_ref, fixture.plan.checkout_plan_id
            ),
        });
    }
    if packet.bootstrap_item_refs.len() != fixture.bootstrap_items.len() {
        return Err(DrillFailureReason::EvidencePacketBroken {
            detail: format!(
                "evidence joined {} item refs but fixture has {} bootstrap items",
                packet.bootstrap_item_refs.len(),
                fixture.bootstrap_items.len()
            ),
        });
    }
    if !packet.export_safe {
        return Err(DrillFailureReason::EvidencePacketBroken {
            detail: "evidence packet is not export-safe".to_string(),
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

fn count(field: &'static str, expected: u64, actual: u64) -> Result<(), DrillFailureReason> {
    if expected == actual {
        Ok(())
    } else {
        Err(DrillFailureReason::CountMismatch {
            field,
            expected,
            actual,
        })
    }
}

fn list(field: &'static str, expected: &[String], actual: &[String]) -> Result<(), DrillFailureReason> {
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

fn surface_from_token(token: &str) -> Option<AcquisitionSurface> {
    Some(match token {
        "start_center" => AcquisitionSurface::StartCenter,
        "command_palette" => AcquisitionSurface::CommandPalette,
        "deep_link" => AcquisitionSurface::DeepLink,
        "cli_headless" => AcquisitionSurface::CliHeadless,
        "policy_guided_deployment" => AcquisitionSurface::PolicyGuidedDeployment,
        "support" => AcquisitionSurface::Support,
        _ => return None,
    })
}

/// Scans a fixture payload for raw-export flags. The closed acquisition
/// vocabulary carries only opaque refs and typed labels; if a future
/// fixture ever names one of these flags, the corpus fails before the
/// drill runs so the redaction contract lives on the corpus itself, not
/// only on individual surface read paths.
fn scan_for_raw_export_leak(payload: &str) -> Option<String> {
    const FORBIDDEN: &[&str] = &[
        "raw_path_export_allowed",
        "raw_remote_url_export_allowed",
        "raw_credential_export_allowed",
        "raw_secret_export_allowed",
        "raw_token_export_allowed",
        "raw_archive_bytes_export_allowed",
        "raw_policy_bundle_bytes_export_allowed",
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
