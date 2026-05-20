//! Loader + runner for the portable-state and restore-provenance drill corpus.
//!
//! [`load_corpus`] reads `manifest.json`; [`run_corpus`] replays every drill
//! against the `aureline-workspace::serialization` beta boundary and returns a
//! [`CorpusReport`]. Positive `restore_provenance_card` drills parse and
//! validate a [`WorkspaceRestoreProvenanceCard`] and must match every pinned
//! expectation. Positive `alpha_migration` drills load an older
//! [`PortableStateAlphaPackage`], migrate it forward through
//! [`WorkspacePortableStatePackage::from_alpha_package`], and additionally
//! prove the migration keeps layers separated, machine-local hints excluded,
//! path/host redaction available, live authority un-rehydrated, and the prior
//! artifact available for compare/export. Negative drills must FAIL validation
//! with an error whose message contains the recorded substring.

use std::path::{Path, PathBuf};

use serde::Serialize;

use aureline_workspace::{
    PortableStateAlphaPackage, PortableStateExclusionReason, RestoreSourceEvent,
    WorkspacePortableStatePackage, WorkspaceRestoreFidelity, WorkspaceRestoreProvenanceCard,
    WorkspaceStateLayer,
};

use super::manifest::{
    drill_kind, CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, MANIFEST_FILE_NAME,
};

/// Raw-export tokens that must never appear in a corpus fixture. Their presence
/// would mean a fixture is asserting that raw secrets, delegated approvals,
/// provider-issued capability tickets, live authority, or off-screen geometry
/// may be exported as authoritative truth.
const FORBIDDEN_RAW_TOKENS: &[&str] = &[
    "raw_secret_export_allowed",
    "delegated_approval_export_allowed",
    "approval_ticket_export_allowed",
    "capability_ticket_export_allowed",
    "provider_capability_ticket_export_allowed",
    "raw_path_export_allowed",
    "raw_host_export_allowed",
    "live_authority_export_allowed",
    "off_screen_geometry_authoritative",
    "-----BEGIN",
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
    /// The fixture contained a forbidden raw-export token.
    RawExportToken(String),
    /// A positive drill record failed validation unexpectedly.
    Validation(String),
    /// A positive migration drill failed the alpha->beta projection.
    Migration(String),
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
    /// Restore class label (positive drills only; empty for negatives).
    pub restore_class: String,
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
        .unwrap_or_else(|err| panic!("portable-state / restore corpus manifest must load: {err}"));
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
        restore_class: spec.restore_class.clone(),
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
        restore_class: String::new(),
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

    match spec.kind.as_str() {
        drill_kind::RESTORE_PROVENANCE_CARD => {
            let card: WorkspaceRestoreProvenanceCard = match serde_json::from_str(&payload) {
                Ok(card) => card,
                Err(err) => return DrillOutcome::Fail(DrillFailureReason::Parse(err.to_string())),
            };
            if let Err(err) = card.validate() {
                return DrillOutcome::Fail(DrillFailureReason::Validation(err.to_string()));
            }
            if let Err(reason) = check_card_expectations(&card, spec) {
                return DrillOutcome::Fail(DrillFailureReason::Expectation(reason));
            }
            DrillOutcome::Pass
        }
        drill_kind::ALPHA_MIGRATION => evaluate_migration(&payload, spec),
        other => DrillOutcome::Fail(DrillFailureReason::UnknownKind(other.to_string())),
    }
}

fn evaluate_migration(payload: &str, spec: &PositiveDrillSpec) -> DrillOutcome {
    let alpha: PortableStateAlphaPackage = match serde_json::from_str(payload) {
        Ok(alpha) => alpha,
        Err(err) => return DrillOutcome::Fail(DrillFailureReason::Parse(err.to_string())),
    };

    let source_event = match parse_source_event(&spec.expected_source_event) {
        Ok(event) => event,
        Err(reason) => return DrillOutcome::Fail(DrillFailureReason::Expectation(reason)),
    };
    let fidelity = match parse_fidelity(&spec.expected_resulting_fidelity) {
        Ok(fidelity) => fidelity,
        Err(reason) => return DrillOutcome::Fail(DrillFailureReason::Expectation(reason)),
    };

    let card = WorkspaceRestoreProvenanceCard::for_alpha_package(
        &alpha,
        format!("restore-card:migration:{}", alpha.package_id),
        source_event,
        fidelity,
        format!("diagnostics:migration:{}", alpha.package_id),
        format!("support-export:migration:{}", alpha.package_id),
        format!("crash-recovery:migration:{}", alpha.package_id),
    );

    let package = match WorkspacePortableStatePackage::from_alpha_package(&alpha, card) {
        Ok(package) => package,
        Err(err) => return DrillOutcome::Fail(DrillFailureReason::Migration(err.to_string())),
    };
    if let Err(err) = package.validate() {
        return DrillOutcome::Fail(DrillFailureReason::Validation(err.to_string()));
    }

    // The migrated card carries the restore-provenance truth and must satisfy
    // the same card-level expectations as a standalone card drill.
    if let Err(reason) = check_card_expectations(&package.restore_provenance_card, spec) {
        return DrillOutcome::Fail(DrillFailureReason::Expectation(reason));
    }
    if let Err(reason) = check_migration_expectations(&package, spec) {
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

    match spec.kind.as_str() {
        drill_kind::RESTORE_PROVENANCE_CARD => {
            let card: WorkspaceRestoreProvenanceCard = match serde_json::from_str(&payload) {
                Ok(card) => card,
                Err(err) => return DrillOutcome::Fail(DrillFailureReason::Parse(err.to_string())),
            };
            match card.validate() {
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
        other => DrillOutcome::Fail(DrillFailureReason::UnknownKind(other.to_string())),
    }
}

fn check_card_expectations(
    card: &WorkspaceRestoreProvenanceCard,
    spec: &PositiveDrillSpec,
) -> Result<(), String> {
    expect_eq(
        "source_event",
        &enum_token(&card.source_event),
        &spec.expected_source_event,
    )?;
    expect_eq(
        "schema_outcome",
        &enum_token(&card.schema_outcome),
        &spec.expected_schema_outcome,
    )?;
    expect_eq(
        "resulting_fidelity",
        &enum_token(&card.resulting_fidelity),
        &spec.expected_resulting_fidelity,
    )?;
    expect_eq(
        "downgrade_label",
        card.resulting_fidelity.display_label(),
        &spec.expected_downgrade_label,
    )?;

    let mut observed: Vec<String> = card
        .missing_surface_placeholders
        .iter()
        .map(|placeholder| enum_token(&placeholder.dependency))
        .collect();
    observed.sort();
    observed.dedup();
    let mut expected = spec.expected_missing_surface_dependencies.clone();
    expected.sort();
    expected.dedup();
    if observed != expected {
        return Err(format!(
            "missing_surface_dependencies mismatch: observed {observed:?}, expected {expected:?}"
        ));
    }

    if spec.expected_named_exclusions && !names_high_risk_exclusions(card) {
        return Err(format!(
            "card must name secrets, approvals, live authority, and machine-unique trust anchors as exclusions; refs were {:?}",
            card.intentional_exclusion_refs
        ));
    }

    if spec.expected_requires_compare_export
        && (card.compare_ref.is_none() || card.export_ref.is_none())
    {
        return Err(
            "prior artifact must stay available: compare_ref and export_ref are required"
                .to_string(),
        );
    }

    Ok(())
}

fn check_migration_expectations(
    package: &WorkspacePortableStatePackage,
    spec: &PositiveDrillSpec,
) -> Result<(), String> {
    if let Some(expected) = spec.expected_required_layers_present {
        let present = [
            WorkspaceStateLayer::WorkspaceAuthority,
            WorkspaceStateLayer::WindowTopology,
            WorkspaceStateLayer::ProfileDefaults,
            WorkspaceStateLayer::MachineLocalHints,
        ]
        .iter()
        .all(|required| {
            package
                .state_layers
                .iter()
                .any(|row| row.layer == *required)
        });
        if present != expected {
            return Err(format!(
                "required_layers_present mismatch: observed {present}, expected {expected}"
            ));
        }
    }

    if let Some(expected) = spec.expected_machine_local_excluded {
        let machine_local = package
            .state_layers
            .iter()
            .find(|row| row.layer == WorkspaceStateLayer::MachineLocalHints);
        let excluded = match machine_local {
            Some(row) => !row.export_allowed,
            None => false,
        };
        if excluded != expected {
            return Err(format!(
                "machine_local_excluded mismatch: observed {excluded}, expected {expected}"
            ));
        }
    }

    if let Some(expected) = spec.expected_path_redaction_available {
        let observed = package.redaction_manifest.path_redaction_available;
        if observed != expected {
            return Err(format!(
                "path_redaction_available mismatch: observed {observed}, expected {expected}"
            ));
        }
    }
    if let Some(expected) = spec.expected_host_redaction_available {
        let observed = package.redaction_manifest.host_redaction_available;
        if observed != expected {
            return Err(format!(
                "host_redaction_available mismatch: observed {observed}, expected {expected}"
            ));
        }
    }

    // A migrated package must never rehydrate live authority: the live-authority
    // handle must remain a named exclusion.
    let names_live_authority = package
        .exclusions
        .iter()
        .any(|row| row.reason == PortableStateExclusionReason::LiveAuthorityHandle);
    if !names_live_authority {
        return Err(
            "migrated package must keep the live-authority handle as a named exclusion".to_string(),
        );
    }

    // Every exclusion must stay named on both the export and restore summaries.
    if let Some(unnamed) = package
        .exclusions
        .iter()
        .find(|row| !row.named_in_export_summary || !row.named_in_restore_summary)
    {
        return Err(format!(
            "exclusion {} must be named on both export and restore summaries",
            unnamed.exclusion_id
        ));
    }

    // The migrated package must project an inspector and an export sheet without
    // error, proving the remembered-state and review surfaces stay reviewable.
    package
        .inspection()
        .map_err(|err| format!("remembered-state inspection failed: {err}"))?;
    package
        .export_review_sheet(format!("export-review:migration:{}", package.package_id))
        .map_err(|err| format!("export review sheet failed: {err}"))?;
    package
        .import_review_sheet(format!("import-review:migration:{}", package.package_id))
        .map_err(|err| format!("import review sheet failed: {err}"))?;

    Ok(())
}

/// Returns true when the card names secrets, approvals, live authority, and
/// machine-unique trust anchors among its intentional exclusion refs.
fn names_high_risk_exclusions(card: &WorkspaceRestoreProvenanceCard) -> bool {
    if card.intentional_exclusion_refs.len() < 4 {
        return false;
    }
    let joined = card.intentional_exclusion_refs.join(" ").to_lowercase();
    joined.contains("secret")
        && joined.contains("approval")
        && joined.contains("live")
        && (joined.contains("trust") || joined.contains("machine"))
}

fn forbidden_token(payload: &str) -> Option<String> {
    FORBIDDEN_RAW_TOKENS
        .iter()
        .find(|token| payload.contains(**token))
        .map(|token| (*token).to_string())
}

fn enum_token<T: Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(serde_json::Value::String(token)) => token,
        other => format!("{other:?}"),
    }
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

fn parse_source_event(token: &str) -> Result<RestoreSourceEvent, String> {
    serde_json::from_value(serde_json::Value::String(token.to_string()))
        .map_err(|err| format!("unknown source event `{token}`: {err}"))
}

fn parse_fidelity(token: &str) -> Result<WorkspaceRestoreFidelity, String> {
    serde_json::from_value(serde_json::Value::String(token.to_string()))
        .map_err(|err| format!("unknown restore fidelity `{token}`: {err}"))
}
