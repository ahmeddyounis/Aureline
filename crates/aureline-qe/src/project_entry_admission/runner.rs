//! Loader + runner for the project-entry and workspace-admission drill corpus.
//!
//! [`load_corpus`] reads `manifest.json`; [`run_corpus`] replays every drill
//! against the project-entry review boundary owned by `aureline-workspace`
//! ([`build_project_entry_review`]). Each positive drill loads a
//! [`ProjectEntryReviewRequest`], builds the review record, asserts the record is
//! contract-valid, and matches every pinned expectation — the verb-specific
//! review sheet, the source-labelled access class, the first-useful entry source
//! and landing surface, the resulting mode and primary next action, the
//! destination-collision posture, the Blocking now / Recommended soon / Optional
//! later readiness counts, the deferred-work classes, and (for imports) the
//! inspect/write posture. The runner also pins the cross-cutting entry
//! guarantees on every drill: no silent trust grant, no setup execution, no task
//! or hook execution, no route auto-trust or auto-install, a preserved entry
//! intent, and a deep-link parity row that always requires deep-link intent
//! review. Each negative drill applies a typed tamper to the built record and
//! requires the entry contract to raise a finding containing the recorded
//! substring, so an unsafe regression stays rejected before a beta entry row
//! hardens.

use std::path::{Path, PathBuf};

use aureline_workspace::{
    build_project_entry_review, AdmissionSourceSurface, EntryReviewRequirementClass,
    EntryReviewSheetKind, ProjectEntryReviewRecord, ProjectEntryReviewRequest, ResultingMode,
};
use serde::Deserialize;

use super::manifest::{
    CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, PositiveExpect, Tamper,
    MANIFEST_FILE_NAME,
};

/// Raw-content tokens that must never appear in a corpus fixture or in a built
/// record. Their presence would mean a fixture is leaking an actual secret,
/// private key, credential, or absolute local path across the support-safe entry
/// review boundary. (Credential-bearing source URLs the user types are a
/// legitimate input; the corpus instead pins that the built record redacts them
/// through the per-drill `redacted_marker_absent` check.)
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
    /// The fixture did not parse into a request envelope.
    Parse(String),
    /// The fixture (or built record) contained a forbidden raw-content token.
    RawToken(String),
    /// A positive drill built a record that was not contract-valid.
    NotContractValid(Vec<String>),
    /// A positive drill missed a pinned expectation.
    Expectation(String),
    /// A negative drill's base record was not contract-valid before tamper.
    NegativeBaseInvalid(Vec<String>),
    /// A negative drill's tamper could not be applied to this base record.
    TamperNotApplicable(String),
    /// A negative drill's tamper left the record contract-valid (not rejected).
    NegativeAccepted,
    /// A negative drill raised findings, but none matched the recorded substring.
    NegativeWrongMessage {
        /// Substring the corpus expected.
        expected: String,
        /// Findings the contract actually produced.
        actual: Vec<String>,
    },
}

/// One drill's report row.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DrillReport {
    /// Stable drill id.
    pub drill_id: String,
    /// Whether this is a positive drill.
    pub positive: bool,
    /// Marketed beta switching row this drill stands in for.
    pub row_label: String,
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

/// In-fixture envelope: a `$schema` prelude plus one entry review request.
#[derive(Debug, Clone, Deserialize)]
struct DrillFixture {
    request: ProjectEntryReviewRequest,
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
        panic!("project-entry and admission corpus manifest must load: {err}")
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
        row_label: spec.row_label.clone(),
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
        row_label: spec.row_label.clone(),
        fixture_path,
        outcome,
    }
}

fn load_request(fixture_path: &Path) -> Result<ProjectEntryReviewRequest, DrillFailureReason> {
    let payload = std::fs::read_to_string(fixture_path)
        .map_err(|err| DrillFailureReason::FixtureRead(err.to_string()))?;
    if let Some(token) = forbidden_token(&payload) {
        return Err(DrillFailureReason::RawToken(token));
    }
    let fixture: DrillFixture =
        serde_json::from_str(&payload).map_err(|err| DrillFailureReason::Parse(err.to_string()))?;
    Ok(fixture.request)
}

fn evaluate_positive(fixture_path: &Path, spec: &PositiveDrillSpec) -> DrillOutcome {
    let request = match load_request(fixture_path) {
        Ok(request) => request,
        Err(reason) => return DrillOutcome::Fail(reason),
    };
    let record = build_project_entry_review(request);

    let findings = record.contract_findings();
    if !findings.is_empty() {
        return DrillOutcome::Fail(DrillFailureReason::NotContractValid(findings));
    }
    if let Err(reason) = check_universal_invariants(&record) {
        return DrillOutcome::Fail(DrillFailureReason::Expectation(reason));
    }
    if let Err(reason) = check_expectations(&record, &spec.expect) {
        return DrillOutcome::Fail(DrillFailureReason::Expectation(reason));
    }
    DrillOutcome::Pass
}

fn evaluate_negative(fixture_path: &Path, spec: &NegativeDrillSpec) -> DrillOutcome {
    let request = match load_request(fixture_path) {
        Ok(request) => request,
        Err(reason) => return DrillOutcome::Fail(reason),
    };
    let mut record = build_project_entry_review(request);

    let base_findings = record.contract_findings();
    if !base_findings.is_empty() {
        return DrillOutcome::Fail(DrillFailureReason::NegativeBaseInvalid(base_findings));
    }
    if let Err(reason) = apply_tamper(&mut record, spec.tamper) {
        return DrillOutcome::Fail(DrillFailureReason::TamperNotApplicable(reason));
    }

    let findings = record.contract_findings();
    if findings.is_empty() {
        return DrillOutcome::Fail(DrillFailureReason::NegativeAccepted);
    }
    if findings
        .iter()
        .any(|finding| finding.contains(&spec.expected_failure_substring))
    {
        DrillOutcome::Pass
    } else {
        DrillOutcome::Fail(DrillFailureReason::NegativeWrongMessage {
            expected: spec.expected_failure_substring.clone(),
            actual: findings,
        })
    }
}

/// Invariants every entry row must hold, independent of its pinned expectations:
/// no silent trust grant, no setup or task / hook execution, no route auto-trust
/// or auto-install, a preserved entry intent, redaction safety, and a deep-link
/// parity row that always requires deep-link intent review.
fn check_universal_invariants(record: &ProjectEntryReviewRecord) -> Result<(), String> {
    let trust = &record.admission_review_packet.trust_and_setup_review;
    if !trust.no_silent_trust_grant {
        return Err("entry must not silently grant trust".to_string());
    }
    if !trust.no_setup_execution {
        return Err("entry must not silently execute setup".to_string());
    }
    if !trust.no_task_or_hook_execution {
        return Err("entry must not silently run tasks or hooks".to_string());
    }
    let route = &record.admission_checkpoint_route;
    if route.auto_install_allowed {
        return Err("entry route must not auto-install setup".to_string());
    }
    if route.auto_trust_allowed {
        return Err("entry route must not auto-trust the workspace".to_string());
    }
    if !route.entry_intent_preserved {
        return Err("entry route must preserve the user's entry intent".to_string());
    }
    if !record
        .post_entry_handoff_card
        .not_yet_done
        .iter()
        .any(|class| class == &aureline_workspace::EntryDeferredWorkClass::TrustGrant)
    {
        return Err("handoff card must declare trust grant as deferred work".to_string());
    }
    let deep_link = record
        .surface_parity
        .iter()
        .find(|parity| parity.source_surface == AdmissionSourceSurface::DeepLink)
        .ok_or_else(|| "surface parity must cover the deep-link surface".to_string())?;
    if deep_link.review_requirement != EntryReviewRequirementClass::DeepLinkIntentReviewRequired {
        return Err(format!(
            "deep-link parity row must require deep-link intent review, observed {}",
            deep_link.review_requirement.as_str()
        ));
    }
    if let Some(token) = forbidden_token(&serialize_record(record)) {
        return Err(format!("built record leaked forbidden token `{token}`"));
    }
    Ok(())
}

fn check_expectations(
    record: &ProjectEntryReviewRecord,
    expect: &PositiveExpect,
) -> Result<(), String> {
    expect_eq(
        "review_sheet_kind",
        record.review_sheet.review_sheet_kind.as_str(),
        &expect.review_sheet_kind,
    )?;
    expect_eq(
        "source_access_class",
        record.vocabulary_review.source_access_class.as_str(),
        &expect.source_access_class,
    )?;
    expect_eq(
        "first_useful_entry_source",
        record
            .admission_checkpoint_route
            .checkpoint
            .entry_source
            .as_str(),
        &expect.first_useful_entry_source,
    )?;
    expect_eq(
        "landing_surface",
        record
            .admission_checkpoint_route
            .first_useful_route
            .landing_surface
            .as_str(),
        &expect.landing_surface,
    )?;
    expect_eq(
        "resulting_mode",
        record.resulting_mode.as_str(),
        &expect.resulting_mode,
    )?;
    expect_eq(
        "primary_next_action",
        record.post_entry_handoff_card.primary_next_action.as_str(),
        &expect.primary_next_action,
    )?;

    let observed_collision = record
        .destination_collision_review
        .as_ref()
        .map(|review| review.collision_class.as_str().to_string());
    if observed_collision.as_deref() != expect.collision_class.as_deref() {
        return Err(format!(
            "collision_class mismatch: observed {observed_collision:?}, expected {:?}",
            expect.collision_class
        ));
    }
    if let Some(expected_choice) = expect.collision_requires_explicit_choice {
        let observed_choice = record
            .destination_collision_review
            .as_ref()
            .map(|review| review.requires_explicit_choice);
        if observed_choice != Some(expected_choice) {
            return Err(format!(
                "collision_requires_explicit_choice mismatch: observed {observed_choice:?}, expected {expected_choice}"
            ));
        }
    }

    expect_count(
        "blocking_now_count",
        record.post_entry_handoff_card.blocked_tasks.len(),
        expect.blocking_now_count,
    )?;
    expect_count(
        "recommended_soon_count",
        record.post_entry_handoff_card.recommended_tasks.len(),
        expect.recommended_soon_count,
    )?;
    expect_count(
        "optional_later_count",
        record.post_entry_handoff_card.optional_tasks.len(),
        expect.optional_later_count,
    )?;

    for class in &expect.deferred_work_present {
        if !record
            .post_entry_handoff_card
            .not_yet_done
            .iter()
            .any(|deferred| deferred.as_str() == class)
        {
            return Err(format!(
                "expected deferred-work class `{class}` not present; observed {:?}",
                record
                    .post_entry_handoff_card
                    .not_yet_done
                    .iter()
                    .map(|d| d.as_str())
                    .collect::<Vec<_>>()
            ));
        }
    }

    if let Some(expected_inspect) = expect.import_inspect_only {
        let import = record
            .review_sheet
            .import_review
            .as_ref()
            .ok_or_else(|| "drill expected an import review sheet".to_string())?;
        if import.inspect_only != expected_inspect {
            return Err(format!(
                "import_inspect_only mismatch: observed {}, expected {expected_inspect}",
                import.inspect_only
            ));
        }
    }
    if let Some(expected_behavior) = &expect.import_write_behavior_class {
        let import = record
            .review_sheet
            .import_review
            .as_ref()
            .ok_or_else(|| "drill expected an import review sheet".to_string())?;
        expect_eq(
            "import_write_behavior_class",
            import.write_behavior_class.as_str(),
            expected_behavior,
        )?;
    }

    if let Some(marker) = &expect.redacted_marker_absent {
        if serialize_record(record).contains(marker) {
            return Err(format!(
                "redaction failed: serialized record still contains marker `{marker}`"
            ));
        }
    }

    Ok(())
}

/// Applies a typed tamper to a built record so the negative drill can prove the
/// entry contract rejects the regression.
fn apply_tamper(record: &mut ProjectEntryReviewRecord, tamper: Tamper) -> Result<(), String> {
    match tamper {
        Tamper::CloneGrantsTrust => {
            let clone = record
                .review_sheet
                .clone_review
                .as_mut()
                .ok_or_else(|| "tamper needs a clone review sheet".to_string())?;
            clone.clone_never_grants_trust = false;
        }
        Tamper::CloneExposesCredentials => {
            let clone = record
                .review_sheet
                .clone_review
                .as_mut()
                .ok_or_else(|| "tamper needs a clone review sheet".to_string())?;
            clone.normalized_remote_url_label = "ci-bot@git.example.com/acme/ops".to_string();
        }
        Tamper::ImportWritesBeforeReview => {
            let import = record
                .review_sheet
                .import_review
                .as_mut()
                .ok_or_else(|| "tamper needs an import review sheet".to_string())?;
            import.no_durable_write_before_review = false;
        }
        Tamper::ImportInspectAdvertisesWrite => {
            let import = record
                .review_sheet
                .import_review
                .as_mut()
                .ok_or_else(|| "tamper needs an import review sheet".to_string())?;
            import.inspect_only = true;
            import.write_behavior_class =
                aureline_workspace::ImportWriteBehaviorClass::WriteToLabelledStaging;
        }
        Tamper::CollisionSkipsExplicitChoice => {
            let collision = record
                .destination_collision_review
                .as_mut()
                .ok_or_else(|| "tamper needs a destination collision review".to_string())?;
            collision.requires_explicit_choice = false;
        }
        Tamper::SurfaceParityDrift => {
            let row = record
                .surface_parity
                .first_mut()
                .ok_or_else(|| "tamper needs a surface parity row".to_string())?;
            row.resulting_mode = drifted_mode(row.resulting_mode);
        }
        Tamper::FailureRepairDropsInputs => {
            record.failure_repair_state.typed_source_input_preserved = false;
        }
        Tamper::RouteAutoTrust => {
            record.admission_checkpoint_route.auto_trust_allowed = true;
        }
        Tamper::RouteAutoInstall => {
            record.admission_checkpoint_route.auto_install_allowed = true;
        }
        Tamper::ReviewSheetMismatch => {
            record.review_sheet.review_sheet_kind = mismatched_sheet(record.entry_verb);
        }
    }
    Ok(())
}

fn drifted_mode(mode: ResultingMode) -> ResultingMode {
    if mode == ResultingMode::InspectOnly {
        ResultingMode::Folder
    } else {
        ResultingMode::InspectOnly
    }
}

fn mismatched_sheet(entry_verb: aureline_workspace::EntryVerb) -> EntryReviewSheetKind {
    if entry_verb == aureline_workspace::EntryVerb::Clone {
        EntryReviewSheetKind::OpenLocalTarget
    } else {
        EntryReviewSheetKind::CloneRepository
    }
}

fn serialize_record(record: &ProjectEntryReviewRecord) -> String {
    serde_json::to_string(record).unwrap_or_default()
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
