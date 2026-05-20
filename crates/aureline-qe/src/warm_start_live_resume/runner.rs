//! Loader + runner for the warm-start, prebuild, and live-resume drill corpus.
//!
//! [`load_corpus`] reads `manifest.json`; [`run_corpus`] replays every drill
//! against the warm-start choice boundary owned by `aureline-shell`
//! ([`validate_warm_start_choice_card`]). Each positive drill loads a
//! [`WarmStartChoiceCard`], asserts the card is contract-valid, re-pins the
//! cross-cutting warm-start guarantees (a local-safe default that never widens
//! trust or runs networked work, an always-present same-weight Open-minimal lane,
//! both same-weight escape hatches on local-first cards, every side-effecting
//! lane gated behind review, and a stale/invalidated snapshot that never backs a
//! takeable live resume), and matches every pinned expectation — the source /
//! support / runtime classes, the offered lanes and their availability, the
//! snapshot freshness / age / invalidation facts, the environment-starter setup
//! location, and the honesty marker. Each negative drill applies a typed tamper
//! to a contract-valid base card and requires the warm-start contract to reject
//! it with a finding containing the recorded substring, so a warm-start path that
//! presents a stale snapshot as a live resume, masquerades a networked lane as a
//! local open, drops a same-weight escape hatch, lets the default widen trust, or
//! hides a managed attach stays rejected before a beta warm-start row hardens.
//!
//! [`validate_warm_start_choice_card`]: aureline_shell::start_center::warm_start_choice::validate_warm_start_choice_card
//! [`WarmStartChoiceCard`]: aureline_shell::start_center::warm_start_choice::WarmStartChoiceCard

use std::path::{Path, PathBuf};

use aureline_shell::start_center::warm_start_choice::{
    validate_warm_start_choice_card, WarmStartChoiceCard, WarmStartChoiceLane,
    WarmStartLaneAvailability, WarmStartPathClass, WarmStartSideEffectClass,
};

use super::manifest::{
    CorpusManifest, NegativeDrillSpec, PositiveDrillSpec, PositiveExpect, Tamper,
    MANIFEST_FILE_NAME,
};

/// Raw-content tokens that must never appear in a corpus fixture or in a
/// validated card. Their presence would mean a fixture is leaking an actual
/// secret, private key, credential, or absolute local path across the
/// support-safe warm-start boundary.
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
    /// The fixture did not parse into a warm-start choice card.
    Parse(String),
    /// The fixture (or validated card) contained a forbidden raw-content token.
    RawToken(String),
    /// A positive drill loaded a card that was not contract-valid.
    NotContractValid(Vec<String>),
    /// A positive drill missed a pinned expectation.
    Expectation(String),
    /// A negative drill's base card was not contract-valid before tamper.
    NegativeBaseInvalid(Vec<String>),
    /// A negative drill's tamper could not be applied to this base card.
    TamperNotApplicable(String),
    /// A negative drill's tamper left the card contract-valid (not rejected).
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
    /// Marketed beta warm-start row this drill stands in for.
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
        panic!("warm-start and live-resume corpus manifest must load: {err}")
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

fn load_card(fixture_path: &Path) -> Result<WarmStartChoiceCard, DrillFailureReason> {
    let payload = std::fs::read_to_string(fixture_path)
        .map_err(|err| DrillFailureReason::FixtureRead(err.to_string()))?;
    if let Some(token) = forbidden_token(&payload) {
        return Err(DrillFailureReason::RawToken(token));
    }
    serde_json::from_str(&payload).map_err(|err| DrillFailureReason::Parse(err.to_string()))
}

fn evaluate_positive(fixture_path: &Path, spec: &PositiveDrillSpec) -> DrillOutcome {
    let card = match load_card(fixture_path) {
        Ok(card) => card,
        Err(reason) => return DrillOutcome::Fail(reason),
    };

    if let Err(findings) = validate_warm_start_choice_card(&card) {
        return DrillOutcome::Fail(DrillFailureReason::NotContractValid(findings));
    }
    if let Err(reason) = check_universal_invariants(&card) {
        return DrillOutcome::Fail(DrillFailureReason::Expectation(reason));
    }
    if let Err(reason) = check_expectations(&card, &spec.expect) {
        return DrillOutcome::Fail(DrillFailureReason::Expectation(reason));
    }
    DrillOutcome::Pass
}

fn evaluate_negative(fixture_path: &Path, spec: &NegativeDrillSpec) -> DrillOutcome {
    let mut card = match load_card(fixture_path) {
        Ok(card) => card,
        Err(reason) => return DrillOutcome::Fail(reason),
    };

    if let Err(findings) = validate_warm_start_choice_card(&card) {
        return DrillOutcome::Fail(DrillFailureReason::NegativeBaseInvalid(findings));
    }
    if let Err(reason) = apply_tamper(&mut card, spec.tamper) {
        return DrillOutcome::Fail(DrillFailureReason::TamperNotApplicable(reason));
    }

    match validate_warm_start_choice_card(&card) {
        Ok(()) => DrillOutcome::Fail(DrillFailureReason::NegativeAccepted),
        Err(findings) => {
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
    }
}

/// Cross-cutting warm-start guarantees every card must hold, independent of its
/// pinned expectations: a local-safe default that never widens trust or runs
/// networked work; an always-present same-weight Open-minimal lane so the user
/// keeps a path to open without the starter; both same-weight escape hatches on
/// local-first cards; every side-effecting lane gated behind review; a stale or
/// invalidated snapshot that never backs a takeable live resume; and redaction
/// safety on the serialized card.
fn check_universal_invariants(card: &WarmStartChoiceCard) -> Result<(), String> {
    if card.default_widens_trust {
        return Err("warm-start default must never widen trust".to_string());
    }
    if card.default_runs_networked_work {
        return Err("warm-start default must never run networked work".to_string());
    }

    let safest = card
        .lane(card.safest_next_action)
        .ok_or_else(|| "safest next action must resolve to a lane".to_string())?;
    if !safest.is_local_safe() {
        return Err("safest next action must resolve to a local-safe lane".to_string());
    }

    // The user always keeps a same-weight path to open without the starter.
    let open_minimal = card
        .lane(WarmStartPathClass::OpenMinimal)
        .ok_or_else(|| "every card must keep an open-minimal lane".to_string())?;
    if !open_minimal.is_local_safe() {
        return Err("the open-minimal lane must stay local-safe".to_string());
    }

    if card.local_first {
        for path in [
            WarmStartPathClass::OpenMinimal,
            WarmStartPathClass::SetUpLater,
        ] {
            let lane = card
                .lane(path)
                .ok_or_else(|| format!("local-first card must keep the {} lane", path.as_str()))?;
            if !lane.same_weight_local_path || !lane.is_local_safe() {
                return Err(format!(
                    "local-first card must keep {} as a same-weight local-safe lane",
                    path.as_str()
                ));
            }
        }
    }

    // No lane may run setup, widen trust, fetch over the network, or attach a
    // managed/remote runtime while immediately available; such work is always
    // gated behind review.
    for lane in &card.choice_lanes {
        if is_side_effecting(lane) && lane.availability == WarmStartLaneAvailability::Available {
            return Err(format!(
                "lane {} runs a side effect but is immediately available without review",
                lane.path_token
            ));
        }
    }

    // A stale or invalidated snapshot must never back a takeable live resume.
    if let Some(snapshot) = &card.snapshot {
        if snapshot.freshness.is_stale_or_invalidated() {
            if let Some(resume) = card.lane(WarmStartPathClass::ResumeLiveWorkspace) {
                if resume.availability.is_takeable() {
                    return Err(
                        "a stale or invalidated snapshot must not back a takeable live resume"
                            .to_string(),
                    );
                }
            }
        }
    }

    if let Some(token) = forbidden_token(&serialize_card(card)) {
        return Err(format!("validated card leaked forbidden token `{token}`"));
    }
    Ok(())
}

fn check_expectations(card: &WarmStartChoiceCard, expect: &PositiveExpect) -> Result<(), String> {
    expect_eq(
        "source_class",
        card.source_class.as_str(),
        &expect.source_class,
    )?;
    expect_eq(
        "support_class",
        card.support_class.as_str(),
        &expect.support_class,
    )?;
    expect_eq(
        "runtime_or_host_model",
        card.runtime_or_host_model.as_str(),
        &expect.runtime_or_host_model,
    )?;
    if card.local_first != expect.local_first {
        return Err(format!(
            "local_first mismatch: observed {}, expected {}",
            card.local_first, expect.local_first
        ));
    }
    expect_eq(
        "safest_next_action",
        card.safest_next_action.as_str(),
        &expect.safest_next_action,
    )?;
    expect_eq(
        "setup_location_class",
        card.environment_starter.setup_location_class.as_str(),
        &expect.setup_location_class,
    )?;
    if card.honesty_marker_present != expect.honesty_marker_present {
        return Err(format!(
            "honesty_marker_present mismatch: observed {}, expected {}",
            card.honesty_marker_present, expect.honesty_marker_present
        ));
    }

    // The set of offered lanes must match exactly, so the distinct warm-start
    // choices on the card stay pinned.
    let mut observed_lanes: Vec<String> = card
        .choice_lanes
        .iter()
        .map(|lane| lane.path_class.as_str().to_string())
        .collect();
    observed_lanes.sort();
    let mut expected_lanes = expect.present_lanes.clone();
    expected_lanes.sort();
    if observed_lanes != expected_lanes {
        return Err(format!(
            "present_lanes mismatch: observed {observed_lanes:?}, expected {expected_lanes:?}"
        ));
    }

    let snapshot_present = card.snapshot.is_some();
    if snapshot_present != expect.snapshot_present {
        return Err(format!(
            "snapshot_present mismatch: observed {snapshot_present}, expected {}",
            expect.snapshot_present
        ));
    }
    if let Some(snapshot) = &card.snapshot {
        if let Some(expected_freshness) = &expect.snapshot_freshness {
            expect_eq(
                "snapshot_freshness",
                snapshot.freshness.as_str(),
                expected_freshness,
            )?;
        }
        if let Some(expected_age) = &expect.snapshot_age_class {
            expect_eq(
                "snapshot_age_class",
                snapshot.age_class.as_str(),
                expected_age,
            )?;
        }
        if let Some(expected_reason_present) = expect.snapshot_invalidation_reason_present {
            let reason_present = snapshot.invalidation_reason.is_some();
            if reason_present != expected_reason_present {
                return Err(format!(
                    "snapshot_invalidation_reason_present mismatch: observed {reason_present}, expected {expected_reason_present}"
                ));
            }
        }
    }

    for lane_expect in &expect.lane_availability {
        let lane = card
            .choice_lanes
            .iter()
            .find(|lane| lane.path_class.as_str() == lane_expect.path)
            .ok_or_else(|| {
                format!(
                    "lane_availability expects lane `{}` but it is absent",
                    lane_expect.path
                )
            })?;
        expect_eq(
            &format!("lane[{}].availability", lane_expect.path),
            lane.availability.as_str(),
            &lane_expect.availability,
        )?;
    }

    if let Some(marker) = &expect.redacted_marker_absent {
        if serialize_card(card).contains(marker) {
            return Err(format!(
                "redaction failed: serialized card still contains marker `{marker}`"
            ));
        }
    }

    Ok(())
}

/// True when a lane reaches outside read-only local state — it fetches over the
/// network, widens trust, runs setup tasks, or attaches a managed/remote runtime.
fn is_side_effecting(lane: &WarmStartChoiceLane) -> bool {
    lane.requires_network
        || lane.requires_trust_grant
        || lane.runs_setup_tasks
        || lane.materializes_remote_work
}

/// Applies a typed tamper to a contract-valid card so the negative drill can
/// prove the warm-start contract rejects the regression.
fn apply_tamper(card: &mut WarmStartChoiceCard, tamper: Tamper) -> Result<(), String> {
    match tamper {
        Tamper::StaleSnapshotResumeTakeable => {
            let lane = lane_mut(card, WarmStartPathClass::ResumeLiveWorkspace)
                .ok_or_else(|| "tamper needs a resume-live lane".to_string())?;
            lane.availability = WarmStartLaneAvailability::Available;
            lane.availability_token = WarmStartLaneAvailability::Available.as_str().to_string();
        }
        Tamper::StaleSnapshotMissingReason => {
            let snapshot = card
                .snapshot
                .as_mut()
                .ok_or_else(|| "tamper needs a snapshot".to_string())?;
            snapshot.invalidation_reason = None;
        }
        Tamper::RemoteLaneMasqueradesAsLocal => {
            let lane = card
                .choice_lanes
                .iter_mut()
                .find(|lane| lane.requires_network)
                .ok_or_else(|| "tamper needs a networked lane".to_string())?;
            lane.side_effect_class = WarmStartSideEffectClass::LocalReadOnly;
            lane.side_effect_token = WarmStartSideEffectClass::LocalReadOnly.as_str().to_string();
        }
        Tamper::EscapeHatchHasSideEffect => {
            let lane = lane_mut(card, WarmStartPathClass::OpenMinimal)
                .ok_or_else(|| "tamper needs an open-minimal lane".to_string())?;
            lane.requires_network = true;
        }
        Tamper::SafestActionNotLocalSafe => {
            let path = card
                .choice_lanes
                .iter()
                .find(|lane| !lane.is_local_safe())
                .map(|lane| lane.path_class)
                .ok_or_else(|| "tamper needs a non-local-safe lane".to_string())?;
            card.safest_next_action = path;
            card.safest_next_action_token = path.as_str().to_string();
        }
        Tamper::DefaultWidensTrust => {
            card.default_widens_trust = true;
        }
        Tamper::LocalFirstEscapeHatchNotSameWeight => {
            if !card.local_first {
                return Err("tamper needs a local-first card".to_string());
            }
            let lane = lane_mut(card, WarmStartPathClass::SetUpLater)
                .ok_or_else(|| "tamper needs a set-up-later lane".to_string())?;
            lane.same_weight_local_path = false;
        }
        Tamper::EnvironmentStarterMissingBypass => {
            card.environment_starter.bypass_route_ids.clear();
        }
        Tamper::EnvironmentStarterMissingDefer => {
            card.environment_starter.defer_route_ids.clear();
        }
        Tamper::ManagedAttachUndisclosed => {
            if !card
                .choice_lanes
                .iter()
                .any(|lane| lane.materializes_remote_work)
            {
                return Err("tamper needs a managed/remote-attach lane".to_string());
            }
            card.side_effects.managed_or_remote_attach = false;
        }
        Tamper::SourceClassTokenDrift => {
            card.source_class_token = format!("{}_drifted", card.source_class_token);
        }
        Tamper::HonestyMarkerInconsistent => {
            let stale = card
                .snapshot
                .as_ref()
                .map(|snapshot| snapshot.freshness.is_stale_or_invalidated())
                .unwrap_or(false);
            if !stale {
                return Err("tamper needs a card with a stale snapshot".to_string());
            }
            card.honesty_marker_present = false;
        }
    }
    Ok(())
}

fn lane_mut(
    card: &mut WarmStartChoiceCard,
    path: WarmStartPathClass,
) -> Option<&mut WarmStartChoiceLane> {
    card.choice_lanes
        .iter_mut()
        .find(|lane| lane.path_class == path)
}

fn serialize_card(card: &WarmStartChoiceCard) -> String {
    serde_json::to_string(card).unwrap_or_default()
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
