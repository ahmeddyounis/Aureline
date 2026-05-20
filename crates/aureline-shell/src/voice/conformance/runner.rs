//! Drill runner for the voice/dictation conformance corpus.
//!
//! The runner reads the manifest pinned at
//! `fixtures/ux/m3/voice_conformance_corpus/`, parses each fixture into the
//! canonical [`VoicePreviewRow`], and replays it through the canonical
//! validation [`build_voice_preview_row`] owned by [`crate::voice`]. It never
//! re-implements the ruleset.
//!
//! Positive drills must validate cleanly, carry no raw-transcript/audio leak,
//! and match the manifest's pinned tokens. Negative drills must be rejected by
//! their recorded detection mechanism — the validator (a blocking finding whose
//! class contains the recorded violation) or the redaction scan.

use std::fs;
use std::path::{Path, PathBuf};

use crate::voice::{build_voice_preview_row, VoicePreviewRow};

use super::corpus::{
    VoiceCorpusManifest, VoiceNegativeDetection, VoiceNegativeDrill, VoicePositiveDrill,
    CORPUS_DIR_REL, MANIFEST_FILE_NAME,
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
    /// Manifest file could not be loaded.
    ManifestLoad(String),
    /// Fixture file could not be read.
    FixtureIo(String),
    /// Fixture failed JSON parsing into [`VoicePreviewRow`].
    ParseFailed(String),
    /// A positive drill produced one or more blocking findings.
    PositiveHasFindings(Vec<String>),
    /// A positive drill payload leaked a raw URL / raw transcript / audio body.
    RawLeak { detail: String },
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
    /// A drill pinned a high-impact command id, but the row bound none.
    HighImpactCommandIdMissing { expected: String },
    /// A negative validator drill unexpectedly validated cleanly.
    NegativeValidatorAccepted,
    /// A negative validator drill failed for a class other than the recorded one.
    NegativeValidatorWrongClass {
        expected_class: String,
        actual_classes: Vec<String>,
    },
    /// A validator negative drill is missing its `expected_violation_class`.
    NegativeValidatorMissingExpectedClass,
    /// A negative redaction-scan drill was not flagged by the leak scan.
    NegativeRedactionNotCaught,
}

/// Single drill result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrillReport {
    /// Stable drill id.
    pub drill_id: String,
    /// Resolved fixture path.
    pub fixture_path: PathBuf,
    /// True for positive-set drills.
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
    /// Per-drill reports in manifest order (positives first, then negatives).
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

    /// Count of positive drills.
    pub fn positive_count(&self) -> usize {
        self.drills.iter().filter(|drill| drill.positive).count()
    }

    /// Count of negative drills.
    pub fn negative_count(&self) -> usize {
        self.drills.iter().filter(|drill| !drill.positive).count()
    }
}

/// Loads the corpus manifest from the given corpus directory.
pub fn load_corpus(corpus_dir: &Path) -> Result<VoiceCorpusManifest, DrillFailureReason> {
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
    run_corpus(&corpus_dir_from_repo_root(repo_root))
}

/// Runs the corpus pinned at the given corpus directory.
pub fn run_corpus(corpus_dir: &Path) -> CorpusReport {
    let manifest = match load_corpus(corpus_dir) {
        Ok(manifest) => manifest,
        Err(reason) => {
            return CorpusReport {
                corpus_dir: corpus_dir.to_path_buf(),
                drills: vec![DrillReport {
                    drill_id: "manifest".to_owned(),
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

fn run_positive_drill(corpus_dir: &Path, spec: &VoicePositiveDrill) -> DrillReport {
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

    if let Some(detail) = scan_for_raw_voice_leak(&payload) {
        return fail(DrillFailureReason::RawLeak { detail });
    }

    let row: VoicePreviewRow = match serde_json::from_str(&payload) {
        Ok(row) => row,
        Err(err) => return fail(DrillFailureReason::ParseFailed(err.to_string())),
    };

    let built = build_voice_preview_row(row);
    if !built.blocking_findings.is_empty() {
        return fail(DrillFailureReason::PositiveHasFindings(
            built
                .blocking_findings
                .iter()
                .map(|finding| finding.class_token().to_owned())
                .collect(),
        ));
    }

    if let Err(reason) = assert_positive_expectations(spec, &built) {
        return fail(reason);
    }

    DrillReport {
        drill_id: spec.drill_id.clone(),
        fixture_path,
        positive: true,
        outcome: DrillOutcome::Pass,
    }
}

fn run_negative_drill(corpus_dir: &Path, spec: &VoiceNegativeDrill) -> DrillReport {
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

    match spec.detection {
        VoiceNegativeDetection::RedactionScan => {
            if scan_for_raw_voice_leak(&payload).is_some() {
                report(DrillOutcome::Pass)
            } else {
                report(DrillOutcome::Fail(
                    DrillFailureReason::NegativeRedactionNotCaught,
                ))
            }
        }
        VoiceNegativeDetection::Validator => {
            let row: VoicePreviewRow = match serde_json::from_str(&payload) {
                Ok(row) => row,
                Err(err) => {
                    return report(DrillOutcome::Fail(DrillFailureReason::ParseFailed(
                        err.to_string(),
                    )))
                }
            };
            let built = build_voice_preview_row(row);
            if built.blocking_findings.is_empty() {
                return report(DrillOutcome::Fail(
                    DrillFailureReason::NegativeValidatorAccepted,
                ));
            }
            let expected = match &spec.expected_violation_class {
                Some(expected) => expected,
                None => {
                    return report(DrillOutcome::Fail(
                        DrillFailureReason::NegativeValidatorMissingExpectedClass,
                    ))
                }
            };
            let matched = built
                .blocking_findings
                .iter()
                .any(|finding| finding.class_token().contains(expected.as_str()));
            if matched {
                report(DrillOutcome::Pass)
            } else {
                report(DrillOutcome::Fail(
                    DrillFailureReason::NegativeValidatorWrongClass {
                        expected_class: expected.clone(),
                        actual_classes: built
                            .blocking_findings
                            .iter()
                            .map(|finding| finding.class_token().to_owned())
                            .collect(),
                    },
                ))
            }
        }
    }
}

fn assert_positive_expectations(
    spec: &VoicePositiveDrill,
    row: &VoicePreviewRow,
) -> Result<(), DrillFailureReason> {
    scalar_opt(
        "claim_posture",
        &spec.expected_claim_posture,
        row.claim_posture.as_str(),
    )?;
    if let Some(expected) = &spec.expected_voice_mode {
        let actual = row
            .mic_pill
            .as_ref()
            .map(|pill| pill.voice_mode_class.as_str())
            .unwrap_or("<none>");
        scalar_eq("voice_mode", expected, actual)?;
    }
    scalar_opt(
        "default_activation",
        &spec.expected_default_activation,
        row.default_activation_class.as_str(),
    )?;
    scalar_opt(
        "processing_locality",
        &spec.expected_processing_locality,
        row.provider_privacy_row.processing_locality_cue.as_str(),
    )?;
    scalar_opt(
        "retention_mode",
        &spec.expected_retention_mode,
        row.provider_privacy_row.retention_mode.as_str(),
    )?;
    scalar_opt(
        "background_listening",
        &spec.expected_background_listening,
        row.background_listening_state.as_str(),
    )?;
    if let Some(expected) = &spec.expected_unavailable_reason {
        let actual = row
            .provider_privacy_row
            .unavailable_reason
            .map(|reason| reason.as_str())
            .unwrap_or("<none>");
        scalar_eq("unavailable_reason", expected, actual)?;
    }
    bool_opt(
        "keyboard_fallback",
        spec.expected_keyboard_fallback,
        row.provider_privacy_row.keyboard_fallback_available,
    )?;
    if let Some(expected) = &spec.expected_high_impact_command_id {
        let bound = row.command_resolutions.iter().any(|resolution| {
            resolution.is_high_impact()
                && resolution.canonical_command_id.as_deref() == Some(expected.as_str())
        });
        if !bound {
            return Err(DrillFailureReason::HighImpactCommandIdMissing {
                expected: expected.clone(),
            });
        }
    }
    Ok(())
}

fn scalar_opt(
    field: &'static str,
    expected: &Option<String>,
    actual: &str,
) -> Result<(), DrillFailureReason> {
    if let Some(expected) = expected {
        scalar_eq(field, expected, actual)?;
    }
    Ok(())
}

fn scalar_eq(field: &'static str, expected: &str, actual: &str) -> Result<(), DrillFailureReason> {
    if expected != actual {
        return Err(DrillFailureReason::ScalarMismatch {
            field,
            expected: expected.to_owned(),
            actual: actual.to_owned(),
        });
    }
    Ok(())
}

fn bool_opt(
    field: &'static str,
    expected: Option<bool>,
    actual: bool,
) -> Result<(), DrillFailureReason> {
    if let Some(expected) = expected {
        if expected != actual {
            return Err(DrillFailureReason::BoolMismatch {
                field,
                expected,
                actual,
            });
        }
    }
    Ok(())
}

/// Scans a fixture payload for a raw-transcript/audio leak. The voice
/// vocabulary carries only opaque ids and typed label refs (`label:`, `a11y:`,
/// `cmd:`, `docs:`, `voice:`, `schemas/...`): no raw URLs, no raw spoken text,
/// and no exported raw audio. If a payload ever names one, the corpus rejects
/// it so the redaction contract lives on the corpus itself.
pub fn scan_for_raw_voice_leak(payload: &str) -> Option<String> {
    if payload.contains("://") {
        return Some("payload carries a raw URL (\"://\")".to_owned());
    }
    const FORBIDDEN: &[&str] = &[
        "-----BEGIN",
        "raw_spoken_text",
        "raw_transcript_bytes",
        "raw_audio_bytes",
    ];
    for needle in FORBIDDEN {
        if payload.contains(needle) {
            return Some(format!("payload carries forbidden marker `{needle}`"));
        }
    }
    None
}
