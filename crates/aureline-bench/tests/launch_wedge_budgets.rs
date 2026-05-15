//! Launch-wedge warm-path budget gate.
//!
//! Reads the warm-path budget register from
//! `artifacts/benchmarks/launch_wedge_warm_path_budgets.yaml` and
//! exercises the gate against synthetic capture rows. The numeric
//! values are NEVER hard-coded here; the register is the source of
//! truth.
//!
//! The artifact projects §7.1 of the Milestones document onto the two
//! alpha launch wedges (TypeScript / JavaScript, Python) at the
//! typical-project reference class. Each metric row carries a budget
//! in milliseconds, and the gate fails a synthetic capture whose
//! `measured_ms` exceeds the matching `budget_ms`, unless the row
//! cites an active known-limit id.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::Deserialize;

const BUDGET_REGISTER_RELATIVE_PATH: &str =
    "artifacts/benchmarks/launch_wedge_warm_path_budgets.yaml";

const KNOWN_LIMITS_RELATIVE_PATH: &str = "artifacts/milestones/m2/known_limits_alpha.yaml";

const REQUIRED_WEDGES: &[&str] = &["typescript_javascript", "python"];

const REQUIRED_METRICS: &[&str] = &[
    "cold_startup_ms",
    "warm_startup_ms",
    "time_to_interactive_ms",
    "warm_first_paint_ms",
    "file_open_1mb_ms",
    "warm_file_switch_ms",
    "keystroke_to_screen_p95_ms",
    "scroll_latency_p95_ms",
    "hot_set_indexing_ms",
    "search_first_results_ms",
    "git_status_refresh_ms",
    "median_extension_activation_ms",
];

// ---------------------------------------------------------------------------
// Register types.
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct BudgetRegister {
    schema_version: u32,
    register_id: String,
    wedges: Vec<WedgeRow>,
    metrics: Vec<MetricRow>,
}

#[derive(Debug, Deserialize)]
struct WedgeRow {
    wedge_id: String,
}

#[derive(Debug, Deserialize)]
struct MetricRow {
    metric_id: String,
    budget_ms: u64,
}

#[derive(Debug, Deserialize)]
struct KnownLimitsPacket {
    known_limits: Vec<KnownLimitRow>,
}

#[derive(Debug, Deserialize)]
struct KnownLimitRow {
    known_limit_id: String,
    note_state: String,
}

// ---------------------------------------------------------------------------
// Repo paths and loaders.
// ---------------------------------------------------------------------------

fn repo_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(format!("../../{relative}"))
}

fn load_register() -> BudgetRegister {
    let path = repo_path(BUDGET_REGISTER_RELATIVE_PATH);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("budget register at {} must read: {err}", path.display()));
    serde_yaml::from_str(&raw)
        .unwrap_or_else(|err| panic!("budget register at {} must parse: {err}", path.display()))
}

fn load_known_limits() -> KnownLimitsPacket {
    let path = repo_path(KNOWN_LIMITS_RELATIVE_PATH);
    let raw = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("known limits at {} must read: {err}", path.display()));
    serde_yaml::from_str(&raw)
        .unwrap_or_else(|err| panic!("known limits at {} must parse: {err}", path.display()))
}

// ---------------------------------------------------------------------------
// Gate.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
struct CaptureRow {
    wedge_id: String,
    metric_id: String,
    measured_ms: u64,
    known_limit_refs: Vec<String>,
}

impl CaptureRow {
    fn new(wedge_id: &str, metric_id: &str, measured_ms: u64) -> Self {
        Self {
            wedge_id: wedge_id.to_owned(),
            metric_id: metric_id.to_owned(),
            measured_ms,
            known_limit_refs: Vec::new(),
        }
    }

    fn with_known_limit(mut self, known_limit_id: &str) -> Self {
        self.known_limit_refs.push(known_limit_id.to_owned());
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum GateVerdict {
    WithinBudget,
    BreachWithActiveKnownLimit { known_limit_id: String },
    Regression { breach_ms: u64 },
    UnknownWedge,
    UnknownMetric,
    UnresolvedKnownLimit { known_limit_id: String },
}

struct Gate<'a> {
    wedges: BTreeSet<&'a str>,
    budgets: BTreeMap<&'a str, u64>,
    active_known_limits: BTreeSet<String>,
}

impl<'a> Gate<'a> {
    fn new(register: &'a BudgetRegister, known_limits: &'a KnownLimitsPacket) -> Self {
        let wedges = register.wedges.iter().map(|w| w.wedge_id.as_str()).collect();
        let budgets = register
            .metrics
            .iter()
            .map(|m| (m.metric_id.as_str(), m.budget_ms))
            .collect();
        let active_known_limits = known_limits
            .known_limits
            .iter()
            .filter(|row| row.note_state == "active")
            .map(|row| row.known_limit_id.clone())
            .collect();
        Self {
            wedges,
            budgets,
            active_known_limits,
        }
    }

    fn evaluate(&self, capture: &CaptureRow) -> GateVerdict {
        if !self.wedges.contains(capture.wedge_id.as_str()) {
            return GateVerdict::UnknownWedge;
        }
        let Some(&budget_ms) = self.budgets.get(capture.metric_id.as_str()) else {
            return GateVerdict::UnknownMetric;
        };
        if capture.measured_ms <= budget_ms {
            return GateVerdict::WithinBudget;
        }
        let breach_ms = capture.measured_ms - budget_ms;
        for known_limit_id in &capture.known_limit_refs {
            if !self.active_known_limits.contains(known_limit_id) {
                return GateVerdict::UnresolvedKnownLimit {
                    known_limit_id: known_limit_id.clone(),
                };
            }
        }
        if let Some(known_limit_id) = capture.known_limit_refs.first() {
            return GateVerdict::BreachWithActiveKnownLimit {
                known_limit_id: known_limit_id.clone(),
            };
        }
        GateVerdict::Regression { breach_ms }
    }
}

// ---------------------------------------------------------------------------
// Tests.
// ---------------------------------------------------------------------------

#[test]
fn register_loads_from_artifact() {
    let register = load_register();
    assert_eq!(register.schema_version, 1);
    assert_eq!(register.register_id, "aureline.launch_wedge_warm_path_budgets");
    assert!(!register.wedges.is_empty(), "register must list wedges");
    assert!(!register.metrics.is_empty(), "register must list metrics");
}

#[test]
fn register_covers_alpha_launch_wedges() {
    let register = load_register();
    let listed: BTreeSet<&str> = register.wedges.iter().map(|w| w.wedge_id.as_str()).collect();
    for required in REQUIRED_WEDGES {
        assert!(
            listed.contains(required),
            "register must cover wedge {required}; listed: {listed:?}"
        );
    }
}

#[test]
fn register_covers_warm_path_metrics_named_in_milestones_7_1() {
    let register = load_register();
    let listed: BTreeSet<&str> =
        register.metrics.iter().map(|m| m.metric_id.as_str()).collect();
    for required in REQUIRED_METRICS {
        assert!(
            listed.contains(required),
            "register must carry metric {required}; listed: {listed:?}"
        );
    }
}

#[test]
fn metric_ids_are_unique_and_well_formed() {
    let register = load_register();
    let mut seen = BTreeSet::new();
    for metric in &register.metrics {
        assert!(
            seen.insert(metric.metric_id.clone()),
            "duplicate metric_id {}",
            metric.metric_id
        );
        assert!(
            metric
                .metric_id
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
            "metric_id {} must be [a-z0-9_]+",
            metric.metric_id
        );
        assert!(metric.budget_ms > 0, "metric {} must have a positive budget", metric.metric_id);
    }
}

#[test]
fn fixture_within_budget_passes_gate() {
    let register = load_register();
    let known_limits = load_known_limits();
    let gate = Gate::new(&register, &known_limits);

    // Synthetic capture: every required metric is reported at half its
    // budget on each wedge. The gate must clear every row.
    for wedge in REQUIRED_WEDGES {
        for metric in &register.metrics {
            let measured_ms = metric.budget_ms / 2;
            let capture = CaptureRow::new(wedge, &metric.metric_id, measured_ms);
            assert_eq!(
                gate.evaluate(&capture),
                GateVerdict::WithinBudget,
                "wedge {wedge} metric {} at {} ms must clear budget {} ms",
                metric.metric_id,
                measured_ms,
                metric.budget_ms,
            );
        }
    }
}

#[test]
fn fixture_at_budget_boundary_passes_gate() {
    let register = load_register();
    let known_limits = load_known_limits();
    let gate = Gate::new(&register, &known_limits);

    // Equal to the budget is still within budget; the bound is
    // inclusive on the budget side.
    let metric = register
        .metrics
        .iter()
        .find(|m| m.metric_id == "warm_first_paint_ms")
        .expect("warm_first_paint_ms must be registered");
    let capture = CaptureRow::new("typescript_javascript", &metric.metric_id, metric.budget_ms);
    assert_eq!(gate.evaluate(&capture), GateVerdict::WithinBudget);
}

#[test]
fn fixture_exceeding_warm_path_budget_fails_gate() {
    let register = load_register();
    let known_limits = load_known_limits();
    let gate = Gate::new(&register, &known_limits);

    // Synthetic capture: one millisecond over the registered budget
    // for warm_first_paint_ms on the TS/JS wedge. The gate must
    // report a regression and name the breach amount.
    let metric = register
        .metrics
        .iter()
        .find(|m| m.metric_id == "warm_first_paint_ms")
        .expect("warm_first_paint_ms must be registered");
    let measured_ms = metric.budget_ms + 1;
    let capture = CaptureRow::new("typescript_javascript", &metric.metric_id, measured_ms);
    assert_eq!(
        gate.evaluate(&capture),
        GateVerdict::Regression { breach_ms: 1 },
        "capture {} ms over budget {} ms must register as regression",
        measured_ms,
        metric.budget_ms,
    );
}

#[test]
fn every_wedge_x_metric_breach_is_reported_as_regression() {
    let register = load_register();
    let known_limits = load_known_limits();
    let gate = Gate::new(&register, &known_limits);

    for wedge in REQUIRED_WEDGES {
        for metric in &register.metrics {
            let measured_ms = metric.budget_ms + 10;
            let capture = CaptureRow::new(wedge, &metric.metric_id, measured_ms);
            match gate.evaluate(&capture) {
                GateVerdict::Regression { breach_ms } => {
                    assert_eq!(breach_ms, 10, "breach amount must equal measured - budget");
                }
                other => panic!(
                    "wedge {wedge} metric {} at {} ms must fail budget {} ms; got {other:?}",
                    metric.metric_id, measured_ms, metric.budget_ms,
                ),
            }
        }
    }
}

#[test]
fn breach_disclosed_under_active_known_limit_does_not_regress() {
    let register = load_register();
    let known_limits = load_known_limits();

    // Pick any active known-limit id from the alpha known-limits
    // packet. We use this id to model the constraint that a
    // currently-disclosed alpha limit MAY shelter a breach without
    // relaxing the published budget.
    let active = known_limits
        .known_limits
        .iter()
        .find(|row| row.note_state == "active")
        .expect("alpha known-limits packet must list at least one active limit")
        .known_limit_id
        .clone();

    let gate = Gate::new(&register, &known_limits);
    let metric = register
        .metrics
        .iter()
        .find(|m| m.metric_id == "search_first_results_ms")
        .expect("search_first_results_ms must be registered");
    let capture = CaptureRow::new("python", &metric.metric_id, metric.budget_ms + 50)
        .with_known_limit(&active);

    assert_eq!(
        gate.evaluate(&capture),
        GateVerdict::BreachWithActiveKnownLimit { known_limit_id: active },
    );
}

#[test]
fn breach_citing_unknown_known_limit_id_remains_a_regression_signal() {
    let register = load_register();
    let known_limits = load_known_limits();
    let gate = Gate::new(&register, &known_limits);

    let metric = register
        .metrics
        .iter()
        .find(|m| m.metric_id == "warm_startup_ms")
        .expect("warm_startup_ms must be registered");
    let capture = CaptureRow::new("typescript_javascript", &metric.metric_id, metric.budget_ms + 1)
        .with_known_limit("known_limit:does_not_exist");

    assert_eq!(
        gate.evaluate(&capture),
        GateVerdict::UnresolvedKnownLimit {
            known_limit_id: "known_limit:does_not_exist".to_owned(),
        },
    );
}

#[test]
fn unknown_wedge_and_unknown_metric_are_schema_breaches() {
    let register = load_register();
    let known_limits = load_known_limits();
    let gate = Gate::new(&register, &known_limits);

    let unknown_wedge = CaptureRow::new("unlisted_wedge", "warm_first_paint_ms", 10);
    assert_eq!(gate.evaluate(&unknown_wedge), GateVerdict::UnknownWedge);

    let unknown_metric = CaptureRow::new("typescript_javascript", "unlisted_metric_ms", 10);
    assert_eq!(gate.evaluate(&unknown_metric), GateVerdict::UnknownMetric);
}
