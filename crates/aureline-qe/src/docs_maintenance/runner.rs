//! Drill runner for the docs preview / maintenance integrity corpus.
//!
//! The runner reuses the canonical docs-maintenance records and their
//! validation owned by `aureline-docs::maintenance` — it never re-implements
//! the ruleset. Positive drills must validate cleanly and match the manifest's
//! pinned truth; negative drills must fail validation with a finding whose
//! `check_id` carries the recorded violation. Review-packet drills are checked
//! against the seeded contract so a flattened or wrong-channel export is caught
//! as drift.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_docs::{
    seeded_docs_preview_and_maintenance_contract, DocsExampleFindingRow, DocsMaintenanceContract,
    DocsMaintenanceFinding, DocsMaintenanceReviewPacket, DocsMaintenanceRow, DocsPreviewHeader,
    DocsSuggestionCard,
};

use super::manifest::{
    CorpusManifest, DrillRecordType, NegativeDrillSpec, PositiveDrillSpec, CORPUS_DIR_REL,
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
    /// Fixture failed JSON parsing into the declared record type.
    ParseFailed(String),
    /// A positive drill produced validation findings (it must validate clean).
    PositiveHasFindings(Vec<String>),
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
    /// A positive drill payload leaked a raw URL or raw-body export flag.
    RawExportLeak { detail: String },
    /// A negative drill unexpectedly validated cleanly.
    NegativeDrillAccepted,
    /// A negative drill failed for a check_id other than the recorded one.
    NegativeDrillWrongCheck {
        expected_check_id: String,
        actual_check_ids: Vec<String>,
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

/// One parsed canonical record from a fixture.
enum ParsedRecord {
    PreviewHeader(Box<DocsPreviewHeader>),
    SuggestionCard(Box<DocsSuggestionCard>),
    FindingRow(Box<DocsExampleFindingRow>),
    MaintenanceRow(Box<DocsMaintenanceRow>),
    Contract(Box<DocsMaintenanceContract>),
    ReviewPacket(Box<DocsMaintenanceReviewPacket>),
}

fn parse_record(
    record_type: DrillRecordType,
    payload: &str,
) -> Result<ParsedRecord, serde_json::Error> {
    Ok(match record_type {
        DrillRecordType::PreviewHeader => {
            ParsedRecord::PreviewHeader(Box::new(serde_json::from_str(payload)?))
        }
        DrillRecordType::SuggestionCard => {
            ParsedRecord::SuggestionCard(Box::new(serde_json::from_str(payload)?))
        }
        DrillRecordType::FindingRow => {
            ParsedRecord::FindingRow(Box::new(serde_json::from_str(payload)?))
        }
        DrillRecordType::MaintenanceRow => {
            ParsedRecord::MaintenanceRow(Box::new(serde_json::from_str(payload)?))
        }
        DrillRecordType::Contract => {
            ParsedRecord::Contract(Box::new(serde_json::from_str(payload)?))
        }
        DrillRecordType::ReviewPacket => {
            ParsedRecord::ReviewPacket(Box::new(serde_json::from_str(payload)?))
        }
    })
}

/// Runs the canonical ruleset for a parsed record, returning any findings.
/// Review packets are validated against the seeded contract so an export that
/// flattens or rewrites a row is caught as drift.
fn validate_record(parsed: &ParsedRecord) -> Vec<DocsMaintenanceFinding> {
    match parsed {
        ParsedRecord::PreviewHeader(header) => header.validate_record(),
        ParsedRecord::SuggestionCard(card) => card.validate_record(),
        ParsedRecord::FindingRow(row) => row.validate_record(),
        ParsedRecord::MaintenanceRow(row) => row.validate_record(),
        ParsedRecord::Contract(contract) => contract.validate(),
        ParsedRecord::ReviewPacket(packet) => {
            let contract = seeded_docs_preview_and_maintenance_contract();
            packet
                .validate_against_contract(&contract)
                .err()
                .unwrap_or_default()
        }
    }
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

    if let Some(detail) = scan_for_raw_export_leak(&payload) {
        return fail(DrillFailureReason::RawExportLeak { detail });
    }

    let parsed = match parse_record(spec.record_type, &payload) {
        Ok(parsed) => parsed,
        Err(err) => return fail(DrillFailureReason::ParseFailed(err.to_string())),
    };

    let findings = validate_record(&parsed);
    if !findings.is_empty() {
        return fail(DrillFailureReason::PositiveHasFindings(
            findings
                .iter()
                .map(|finding| format!("{}: {}", finding.check_id, finding.message))
                .collect(),
        ));
    }

    if let Err(reason) = assert_positive_expectations(spec, &parsed) {
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

    let parsed = match parse_record(spec.record_type, &payload) {
        Ok(parsed) => parsed,
        Err(err) => {
            return report(DrillOutcome::Fail(DrillFailureReason::ParseFailed(
                err.to_string(),
            )))
        }
    };

    let findings = validate_record(&parsed);
    if findings.is_empty() {
        return report(DrillOutcome::Fail(
            DrillFailureReason::NegativeDrillAccepted,
        ));
    }

    let matched = findings
        .iter()
        .any(|finding| finding.check_id.contains(&spec.expected_violation_check_id));
    if matched {
        report(DrillOutcome::Pass)
    } else {
        report(DrillOutcome::Fail(
            DrillFailureReason::NegativeDrillWrongCheck {
                expected_check_id: spec.expected_violation_check_id.clone(),
                actual_check_ids: findings
                    .iter()
                    .map(|finding| finding.check_id.clone())
                    .collect(),
            },
        ))
    }
}

fn assert_positive_expectations(
    spec: &PositiveDrillSpec,
    parsed: &ParsedRecord,
) -> Result<(), DrillFailureReason> {
    match parsed {
        ParsedRecord::PreviewHeader(header) => {
            scalar_opt(
                "preview_mode",
                &spec.expected_preview_mode,
                header.preview_mode.as_str(),
            )?;
            scalar_opt(
                "sanitization_state",
                &spec.expected_sanitization_state,
                header.sanitization_state.as_str(),
            )?;
            bool_opt(
                "commonmark_baseline",
                spec.expected_commonmark_baseline,
                header.commonmark_baseline,
            )?;
        }
        ParsedRecord::SuggestionCard(card) => {
            scalar_opt("trigger", &spec.expected_trigger, card.trigger.as_str())?;
            scalar_opt(
                "apply_posture",
                &spec.expected_apply_posture,
                card.apply_posture.as_str(),
            )?;
            scalar_opt(
                "artifact_kind",
                &spec.expected_artifact_kind,
                card.artifact_kind.as_str(),
            )?;
            scalar_opt(
                "publish_boundary_state",
                &spec.expected_publish_boundary_state,
                card.publish_boundary_state.as_str(),
            )?;
        }
        ParsedRecord::FindingRow(row) => {
            scalar_opt(
                "finding_class",
                &spec.expected_finding_class,
                row.finding_class.as_str(),
            )?;
            scalar_opt(
                "detection_state",
                &spec.expected_detection_state,
                row.detection_state.as_str(),
            )?;
            scalar_opt(
                "validation_mode",
                &spec.expected_validation_mode,
                row.validation_mode.as_str(),
            )?;
            scalar_opt(
                "suppression_state",
                &spec.expected_suppression_state,
                row.suppression_state.as_str(),
            )?;
            scalar_opt(
                "artifact_kind",
                &spec.expected_artifact_kind,
                row.artifact_kind.as_str(),
            )?;
            if let Some(true) = spec.expected_suppression_attribution {
                let attributed = row.suppression_detail.as_ref().is_some_and(|detail| {
                    !detail.actor_ref.trim().is_empty()
                        && !detail.reason.trim().is_empty()
                        && !detail.expiry_at.trim().is_empty()
                        && !detail.evidence_refs.is_empty()
                });
                bool_opt("suppression_attribution", Some(true), attributed)?;
            }
        }
        ParsedRecord::MaintenanceRow(row) => {
            scalar_opt(
                "artifact_kind",
                &spec.expected_artifact_kind,
                row.artifact_kind.as_str(),
            )?;
            scalar_opt(
                "audience_scope",
                &spec.expected_audience_scope,
                row.audience_scope.as_str(),
            )?;
            scalar_opt(
                "publish_boundary_state",
                &spec.expected_publish_boundary_state,
                row.publish_boundary_state.as_str(),
            )?;
            scope_opt(
                "branch_scope",
                &spec.expected_branch_scope,
                &row.publish_scope.branch_scope,
            )?;
            scope_opt(
                "release_scope",
                &spec.expected_release_scope,
                &row.publish_scope.release_scope,
            )?;
            scope_opt(
                "channel_scope",
                &spec.expected_channel_scope,
                &row.publish_scope.channel_scope,
            )?;
        }
        // Contract and review-packet drills assert clean validation only; their
        // truth is the whole-corpus parity proven by the conformance test.
        ParsedRecord::Contract(_) | ParsedRecord::ReviewPacket(_) => {}
    }
    Ok(())
}

fn scalar_opt(
    field: &'static str,
    expected: &Option<String>,
    actual: &str,
) -> Result<(), DrillFailureReason> {
    if let Some(expected) = expected {
        if expected != actual {
            return Err(DrillFailureReason::ScalarMismatch {
                field,
                expected: expected.clone(),
                actual: actual.to_string(),
            });
        }
    }
    Ok(())
}

fn scope_opt(
    field: &'static str,
    expected: &Option<String>,
    actual: &Option<String>,
) -> Result<(), DrillFailureReason> {
    if let Some(expected) = expected {
        let actual = actual.as_deref().unwrap_or("<none>");
        if expected != actual {
            return Err(DrillFailureReason::ScalarMismatch {
                field,
                expected: expected.clone(),
                actual: actual.to_string(),
            });
        }
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

/// Scans a positive fixture payload for raw-export leaks. The docs-maintenance
/// vocabulary carries only opaque refs and typed labels: no raw URLs, and no
/// exported raw document bodies. If a fixture ever names one, the corpus fails
/// before the drill runs so the redaction contract lives on the corpus itself.
fn scan_for_raw_export_leak(payload: &str) -> Option<String> {
    if payload.contains("://") {
        return Some("payload carries a raw URL (\"://\")".to_string());
    }
    const FORBIDDEN: &[&str] = &[
        "\"raw_document_bodies_exported\": true",
        "\"raw_document_bodies_exported\":true",
    ];
    for needle in FORBIDDEN {
        if payload.contains(needle) {
            return Some(format!("payload sets {needle}"));
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
