//! Smoke harness for the large-file prototype.
//!
//! Drives the prototype through a frozen scenario table — one
//! scenario per switch trigger and one per representative
//! limited-mode capability outcome — and emits a structural
//! metrics record. Metrics are counts only (no wall-clock
//! timings) so the committed seed under
//! `artifacts/bench/large_file_proto_metrics.json` is byte-stable
//! across hosts. The benchmark lab layers wall-clock timing on
//! top of these counts when it scores against the protected
//! hot-path budgets named in the ADR.
//!
//! Fixtures are embedded in the crate via [`include_bytes!`] so
//! the harness is host-independent: it writes each fixture to a
//! scratch directory on the caller's request, runs the scenario
//! against the freshly-written file, and reports the resulting
//! counters, reader metrics, classification decision, and a
//! per-step capability log.

use std::fmt::Write as _;
use std::path::Path;

use crate::buffer::{
    EditOutcome, EditRequest, LargeFileBuffer, LargeFileConfig, SaveOutcome, SaveRequest,
};
use crate::capabilities::CapabilityState;
use crate::classification::{ClassificationPolicy, FileMode, LargeFileTrigger};
use crate::hooks::HookCounters;
use crate::paged::ReaderMetrics;

/// Frozen corpus identifier.
pub const CORPUS_ID: &str = "aureline.largefile_proto_scenarios.v1";

/// Schema version for the emitted metrics JSON.
pub const METRICS_SCHEMA_VERSION: u32 = 1;

/// One embedded fixture. The harness writes the `bytes` to a
/// scratch directory at the configured `name` before running its
/// scenario.
#[derive(Debug, Clone, Copy)]
pub struct EmbeddedFixture {
    pub name: &'static str,
    pub bytes: &'static [u8],
}

/// Frozen catalogue of fixtures the prototype owns. Aligned with
/// `fixtures/text/large/` on disk; the bytes are embedded so the
/// harness does not depend on the workspace path layout at run
/// time.
pub const FIXTURES: &[EmbeddedFixture] = &[
    EmbeddedFixture {
        name: "clean_small_text.txt",
        bytes: include_bytes!("../../../fixtures/text/large/clean_small_text.txt"),
    },
    EmbeddedFixture {
        name: "null_byte_blob.bin",
        bytes: include_bytes!("../../../fixtures/text/large/null_byte_blob.bin"),
    },
    EmbeddedFixture {
        name: "minified_long_line.js",
        bytes: include_bytes!("../../../fixtures/text/large/minified_long_line.js"),
    },
    EmbeddedFixture {
        name: "pack_suffix_clean.min.js",
        bytes: include_bytes!("../../../fixtures/text/large/pack_suffix_clean.min.js"),
    },
    EmbeddedFixture {
        name: "operator_override_target.txt",
        bytes: include_bytes!("../../../fixtures/text/large/operator_override_target.txt"),
    },
    EmbeddedFixture {
        name: "above_threshold_text.txt",
        bytes: include_bytes!("../../../fixtures/text/large/above_threshold_text.txt"),
    },
    EmbeddedFixture {
        name: "decode_recovery_target.txt",
        bytes: include_bytes!("../../../fixtures/text/large/decode_recovery_target.txt"),
    },
];

/// Look up an embedded fixture by name.
pub fn fixture(name: &str) -> Option<&'static EmbeddedFixture> {
    FIXTURES.iter().find(|f| f.name == name)
}

/// Materialise every embedded fixture under `dir`.
pub fn write_fixtures(dir: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dir)?;
    for fixture in FIXTURES {
        let path = dir.join(fixture.name);
        std::fs::write(&path, fixture.bytes)?;
    }
    Ok(())
}

/// One reviewable step inside a scenario script. Recorded so the
/// emitted JSON shows which capability the step consulted, what
/// the request was, and what the buffer answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptedStep {
    pub kind: &'static str,
    pub detail: String,
    pub outcome: String,
    pub capability_id: Option<&'static str>,
}

/// A scenario's full output: counts, reader metrics, the
/// classification decision (mode + trigger + reason), and the
/// per-step capability log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScenarioReport {
    pub label: &'static str,
    pub fixture_name: &'static str,
    pub fixture_size: u64,
    pub mode: FileMode,
    pub trigger: Option<LargeFileTrigger>,
    pub reason: String,
    pub page_size: u64,
    pub max_resident_pages: u64,
    pub page_count: u64,
    pub steps: Vec<ScriptedStep>,
    pub counters: HookCounters,
    pub reader_metrics: ReaderMetrics,
}

/// Aggregate across all scenarios.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct AggregateReport {
    pub total_scenarios: u64,
    pub total_large_file_mode_enter: u64,
    pub total_paged_reads: u64,
    pub total_pages_read_from_disk: u64,
    pub total_pages_evicted: u64,
    pub total_bytes_read_from_disk: u64,
    pub total_edits_denied: u64,
    pub total_edits_downgraded: u64,
    pub total_saves_denied: u64,
}

/// Full harness output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessReport {
    pub schema_version: u32,
    pub corpus_id: &'static str,
    pub scenarios: Vec<ScenarioReport>,
    pub aggregate: AggregateReport,
}

/// Run every named scenario in order against fixtures rooted at
/// `scratch_dir`. Callers MUST call [`write_fixtures`] first so
/// the on-disk files exist; the bench binary does this for the
/// caller.
pub fn run_harness(scratch_dir: &Path) -> std::io::Result<HarnessReport> {
    let mut scenarios = Vec::with_capacity(SCENARIOS.len());
    let mut agg = AggregateReport::default();
    for spec in SCENARIOS {
        let report = run_scenario(spec, scratch_dir)?;
        agg.total_large_file_mode_enter += report.counters.large_file_mode_enter;
        agg.total_paged_reads += report.counters.paged_read;
        agg.total_pages_read_from_disk += report.reader_metrics.pages_read_from_disk;
        agg.total_pages_evicted += report.reader_metrics.pages_evicted;
        agg.total_bytes_read_from_disk += report.reader_metrics.bytes_read_from_disk;
        for step in &report.steps {
            if step.outcome.starts_with("denied") {
                if step.kind == "attempt_save" {
                    agg.total_saves_denied += 1;
                } else {
                    agg.total_edits_denied += 1;
                }
            } else if step.outcome.starts_with("downgraded") {
                agg.total_edits_downgraded += 1;
            }
        }
        scenarios.push(report);
    }
    agg.total_scenarios = SCENARIOS.len() as u64;
    Ok(HarnessReport {
        schema_version: METRICS_SCHEMA_VERSION,
        corpus_id: CORPUS_ID,
        scenarios,
        aggregate: agg,
    })
}

fn run_scenario(spec: &ScenarioSpec, scratch_dir: &Path) -> std::io::Result<ScenarioReport> {
    let fix = fixture(spec.fixture_name).expect("fixture name in scenario must be in catalogue");
    let path = scratch_dir.join(fix.name);
    if !path.exists() {
        std::fs::write(&path, fix.bytes)?;
    }
    let config = (spec.config)();
    let mut buf = LargeFileBuffer::open(&path, &config)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, format!("{e}")))?;
    let mut steps = Vec::new();
    (spec.script)(&mut buf, &mut steps)?;
    let snap = buf.structural_snapshot();
    Ok(ScenarioReport {
        label: spec.label,
        fixture_name: fix.name,
        fixture_size: fix.bytes.len() as u64,
        mode: snap.mode,
        trigger: buf.decision().trigger,
        reason: buf.decision().reason.clone(),
        page_size: snap.page_size,
        max_resident_pages: snap.max_resident_pages,
        page_count: snap.page_count,
        steps,
        counters: snap.counters,
        reader_metrics: snap.reader_metrics,
    })
}

#[derive(Debug)]
pub struct ScenarioSpec {
    pub label: &'static str,
    pub fixture_name: &'static str,
    pub config: fn() -> LargeFileConfig,
    pub script: fn(&mut LargeFileBuffer, &mut Vec<ScriptedStep>) -> std::io::Result<()>,
}

/// The frozen scenario table. One row per switch trigger and one
/// per representative limited-mode capability outcome so the
/// emitted seed exercises the full vocabulary.
pub const SCENARIOS: &[ScenarioSpec] = &[
    ScenarioSpec {
        label: "normal_mode_clean_small_text",
        fixture_name: "clean_small_text.txt",
        config: configs::default_with_small_pages,
        script: scripts::normal_mode_round_trip,
    },
    ScenarioSpec {
        label: "trigger_size_threshold",
        fixture_name: "above_threshold_text.txt",
        config: configs::tight_size_threshold,
        script: scripts::limited_mode_round_trip,
    },
    ScenarioSpec {
        label: "trigger_classification_null_byte",
        fixture_name: "null_byte_blob.bin",
        config: configs::default_with_small_pages,
        script: scripts::limited_mode_round_trip,
    },
    ScenarioSpec {
        label: "trigger_classification_minified_long_line",
        fixture_name: "minified_long_line.js",
        config: configs::minified_threshold_low,
        script: scripts::limited_mode_round_trip,
    },
    ScenarioSpec {
        label: "trigger_classification_pack_suffix",
        fixture_name: "pack_suffix_clean.min.js",
        config: configs::default_with_small_pages,
        script: scripts::limited_mode_round_trip,
    },
    ScenarioSpec {
        label: "trigger_decode_posture",
        fixture_name: "decode_recovery_target.txt",
        config: configs::decode_recovery_chose_large_file,
        script: scripts::limited_mode_round_trip,
    },
    ScenarioSpec {
        label: "trigger_operator_override",
        fixture_name: "operator_override_target.txt",
        config: configs::operator_override,
        script: scripts::limited_mode_round_trip,
    },
];

mod configs {
    use super::*;

    pub(super) fn default_with_small_pages() -> LargeFileConfig {
        let mut cfg = LargeFileConfig::default();
        cfg.page_size = 256;
        cfg.max_resident_pages = 2;
        cfg
    }

    pub(super) fn tight_size_threshold() -> LargeFileConfig {
        let mut cfg = LargeFileConfig::default();
        cfg.policy.large_file_size_threshold = 256;
        cfg.page_size = 256;
        cfg.max_resident_pages = 2;
        cfg
    }

    pub(super) fn decode_recovery_chose_large_file() -> LargeFileConfig {
        let mut cfg = LargeFileConfig::default();
        cfg.policy.decode_recovery_chose_large_file = true;
        cfg.page_size = 256;
        cfg.max_resident_pages = 2;
        cfg
    }

    pub(super) fn operator_override() -> LargeFileConfig {
        let mut cfg = LargeFileConfig::default();
        cfg.policy.operator_override = true;
        cfg.page_size = 256;
        cfg.max_resident_pages = 2;
        cfg
    }

    pub(super) fn minified_threshold_low() -> LargeFileConfig {
        // The committed minified fixture uses a ~2 KiB single
        // line so the repo stays small. The default policy keeps
        // the conservative 8 KiB minified threshold; this config
        // pulls the threshold down for the dedicated scenario so
        // the heuristic still fires on the small fixture.
        let mut cfg = LargeFileConfig::default();
        cfg.policy.minified_line_length = 512;
        cfg.page_size = 256;
        cfg.max_resident_pages = 2;
        cfg
    }
}

mod scripts {
    use super::*;

    fn record_edit(buf: &LargeFileBuffer, steps: &mut Vec<ScriptedStep>, request: EditRequest) {
        let outcome = buf.attempt_edit(request);
        let outcome_str = match outcome {
            EditOutcome::Accepted { .. } => "accepted".to_owned(),
            EditOutcome::Denied { reason, .. } => format!("denied: {reason}"),
            EditOutcome::Downgraded { reason, .. } => format!("downgraded: {reason}"),
        };
        steps.push(ScriptedStep {
            kind: "attempt_edit",
            detail: format!("{request:?}"),
            outcome: outcome_str,
            capability_id: Some(outcome.capability_id()),
        });
    }

    fn record_save(buf: &LargeFileBuffer, steps: &mut Vec<ScriptedStep>, request: SaveRequest) {
        let outcome = buf.attempt_save(request);
        let outcome_str = match outcome {
            SaveOutcome::Accepted { reason } => format!("accepted: {reason}"),
            SaveOutcome::Denied { reason } => format!("denied: {reason}"),
        };
        steps.push(ScriptedStep {
            kind: "attempt_save",
            detail: format!("{request:?}"),
            outcome: outcome_str,
            capability_id: None,
        });
    }

    pub(super) fn normal_mode_round_trip(
        buf: &mut LargeFileBuffer,
        steps: &mut Vec<ScriptedStep>,
    ) -> std::io::Result<()> {
        // Walk a small range, then verify every limited-mode-only
        // request is still accepted in normal mode.
        let len = buf.len();
        let _ = buf.read_range(0..len.min(512))?;
        steps.push(ScriptedStep {
            kind: "read_range",
            detail: format!("0..{}", len.min(512)),
            outcome: "ok".to_owned(),
            capability_id: None,
        });
        record_edit(buf, steps, EditRequest::ViewportCursorInsert);
        record_edit(buf, steps, EditRequest::WholeFileMultiCursor);
        record_edit(buf, steps, EditRequest::FullFileFormatOnSave);
        record_edit(buf, steps, EditRequest::RichRefactorMultiFile);
        record_save(buf, steps, SaveRequest::WithWholeFileParticipants);
        record_save(buf, steps, SaveRequest::EditedRangeOnly);
        Ok(())
    }

    pub(super) fn limited_mode_round_trip(
        buf: &mut LargeFileBuffer,
        steps: &mut Vec<ScriptedStep>,
    ) -> std::io::Result<()> {
        // Walk the whole file through the paged reader; then drive
        // the capability table through one accepted, one
        // downgraded, and one denied edit, plus the save split.
        let len = buf.len();
        let _ = buf.read_range(0..len)?;
        steps.push(ScriptedStep {
            kind: "read_range",
            detail: format!("0..{len}"),
            outcome: "ok".to_owned(),
            capability_id: None,
        });
        // A streaming search exercises find_first; if the fixture
        // does not contain "needle" it returns None. The result
        // does not gate the rest of the script.
        let pos = buf.find_first(b"needle")?;
        steps.push(ScriptedStep {
            kind: "find_first",
            detail: "needle".to_owned(),
            outcome: match pos {
                Some(p) => format!("ok: {p}"),
                None => "ok: not_found".to_owned(),
            },
            capability_id: None,
        });
        record_edit(buf, steps, EditRequest::ViewportCursorInsert);
        record_edit(buf, steps, EditRequest::ViewportMultiCursor);
        record_edit(buf, steps, EditRequest::SearchViewport);
        record_edit(buf, steps, EditRequest::SearchWholeFile);
        record_edit(buf, steps, EditRequest::WholeFileMultiCursor);
        record_edit(buf, steps, EditRequest::FullFileFormatOnSave);
        record_edit(buf, steps, EditRequest::RangeFormatOnSave);
        record_edit(buf, steps, EditRequest::FullFileAiApply);
        record_edit(buf, steps, EditRequest::RangeAiApply);
        record_edit(buf, steps, EditRequest::RichRefactorSingleFile);
        record_edit(buf, steps, EditRequest::RichRefactorMultiFile);
        record_edit(buf, steps, EditRequest::Indexing);
        record_edit(buf, steps, EditRequest::DiagnosticsViewport);
        record_edit(buf, steps, EditRequest::DiagnosticsWholeFile);
        record_save(buf, steps, SaveRequest::WithWholeFileParticipants);
        record_save(buf, steps, SaveRequest::WithRangeOnlyParticipants);
        record_save(buf, steps, SaveRequest::EditedRangeOnly);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// JSON renderer. Hand-rolled so we do not pull in a serde dep just
// for the prototype's emitted seed; output stays byte-stable as
// long as the renderer stays byte-stable.
// ---------------------------------------------------------------------------

pub fn report_to_json(report: &HarnessReport) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    kv_u64(
        &mut out,
        1,
        "schema_version",
        u64::from(report.schema_version),
        false,
    );
    kv_str(&mut out, 1, "corpus_id", report.corpus_id, false);

    key(&mut out, 1, "aggregate");
    out.push_str(" {\n");
    let agg = &report.aggregate;
    kv_u64(&mut out, 2, "total_scenarios", agg.total_scenarios, false);
    kv_u64(
        &mut out,
        2,
        "total_large_file_mode_enter",
        agg.total_large_file_mode_enter,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_paged_reads",
        agg.total_paged_reads,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_pages_read_from_disk",
        agg.total_pages_read_from_disk,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_pages_evicted",
        agg.total_pages_evicted,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_bytes_read_from_disk",
        agg.total_bytes_read_from_disk,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_edits_denied",
        agg.total_edits_denied,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_edits_downgraded",
        agg.total_edits_downgraded,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "total_saves_denied",
        agg.total_saves_denied,
        true,
    );
    indent(&mut out, 1);
    out.push_str("},\n");

    key(&mut out, 1, "scenarios");
    out.push_str(" [\n");
    for (i, scenario) in report.scenarios.iter().enumerate() {
        let last = i + 1 == report.scenarios.len();
        write_scenario(&mut out, 2, scenario, last);
    }
    indent(&mut out, 1);
    out.push_str("]\n");
    out.push_str("}\n");
    out
}

fn write_scenario(out: &mut String, depth: usize, s: &ScenarioReport, last: bool) {
    indent(out, depth);
    out.push_str("{\n");
    kv_str(out, depth + 1, "label", s.label, false);
    kv_str(out, depth + 1, "fixture_name", s.fixture_name, false);
    kv_u64(out, depth + 1, "fixture_size", s.fixture_size, false);
    kv_str(out, depth + 1, "mode", s.mode.as_str(), false);
    kv_str(
        out,
        depth + 1,
        "trigger",
        s.trigger.map(|t| t.as_str()).unwrap_or("none"),
        false,
    );
    kv_str(out, depth + 1, "reason", &s.reason, false);
    kv_u64(out, depth + 1, "page_size", s.page_size, false);
    kv_u64(
        out,
        depth + 1,
        "max_resident_pages",
        s.max_resident_pages,
        false,
    );
    kv_u64(out, depth + 1, "page_count", s.page_count, false);

    write_steps(out, depth + 1, &s.steps);
    write_counters(out, depth + 1, &s.counters);
    write_reader_metrics(out, depth + 1, &s.reader_metrics, true);

    indent(out, depth);
    if last {
        out.push_str("}\n");
    } else {
        out.push_str("},\n");
    }
}

fn write_steps(out: &mut String, depth: usize, steps: &[ScriptedStep]) {
    key(out, depth, "steps");
    out.push_str(" [\n");
    for (i, step) in steps.iter().enumerate() {
        let last = i + 1 == steps.len();
        indent(out, depth + 1);
        out.push_str("{\n");
        kv_str(out, depth + 2, "kind", step.kind, false);
        kv_str(out, depth + 2, "detail", &step.detail, false);
        kv_str(out, depth + 2, "outcome", &step.outcome, false);
        kv_str(
            out,
            depth + 2,
            "capability_id",
            step.capability_id.unwrap_or(""),
            true,
        );
        indent(out, depth + 1);
        if last {
            out.push_str("}\n");
        } else {
            out.push_str("},\n");
        }
    }
    indent(out, depth);
    out.push_str("],\n");
}

fn write_counters(out: &mut String, depth: usize, counters: &HookCounters) {
    key(out, depth, "hook_counters");
    out.push_str(" {\n");
    let entries = counters.entries();
    for (i, (name, count)) in entries.iter().enumerate() {
        let last = i + 1 == entries.len();
        kv_u64(out, depth + 1, name, *count, last);
    }
    indent(out, depth);
    out.push_str("},\n");
}

fn write_reader_metrics(out: &mut String, depth: usize, metrics: &ReaderMetrics, last: bool) {
    key(out, depth, "reader_metrics");
    out.push_str(" {\n");
    let entries = metrics.entries();
    for (i, (name, value)) in entries.iter().enumerate() {
        let entry_last = i + 1 == entries.len();
        kv_u64(out, depth + 1, name, *value, entry_last);
    }
    indent(out, depth);
    if last {
        out.push_str("}\n");
    } else {
        out.push_str("},\n");
    }
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn key(out: &mut String, depth: usize, key: &str) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\":");
}

fn kv_u64(out: &mut String, depth: usize, key: &str, value: u64, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {value}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn kv_str(out: &mut String, depth: usize, key: &str, value: &str, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {}", json_quote(value));
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn json_quote(value: &str) -> String {
    let mut out = String::with_capacity(value.len() + 2);
    out.push('"');
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

// We intentionally avoid silencing this — it surfaces if a
// capability was looked up that we forgot to expose.
impl CapabilityState {
    #[doc(hidden)]
    pub fn _unused_hint(self) -> &'static str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn scratchdir() -> PathBuf {
        let mut p = std::env::temp_dir();
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        p.push(format!(
            "aureline-largefile-proto-harness-{nanos}-{}",
            std::process::id()
        ));
        std::fs::create_dir_all(&p).unwrap();
        p
    }

    fn cleanup(p: PathBuf) {
        let _ = std::fs::remove_dir_all(p);
    }

    #[test]
    fn fixture_catalogue_is_non_empty_and_unique() {
        assert!(!FIXTURES.is_empty());
        let mut names: Vec<&'static str> = FIXTURES.iter().map(|f| f.name).collect();
        names.sort();
        let unique = {
            let mut u = names.clone();
            u.dedup();
            u
        };
        assert_eq!(names, unique);
    }

    #[test]
    fn write_fixtures_round_trips() {
        let dir = scratchdir();
        write_fixtures(&dir).unwrap();
        for fix in FIXTURES {
            let bytes = std::fs::read(dir.join(fix.name)).unwrap();
            assert_eq!(bytes.as_slice(), fix.bytes);
        }
        cleanup(dir);
    }

    #[test]
    fn run_harness_covers_every_scenario_and_is_byte_stable() {
        let dir_a = scratchdir();
        let dir_b = scratchdir();
        write_fixtures(&dir_a).unwrap();
        write_fixtures(&dir_b).unwrap();
        let report_a = run_harness(&dir_a).unwrap();
        let report_b = run_harness(&dir_b).unwrap();
        assert_eq!(report_a.scenarios.len(), SCENARIOS.len());
        let json_a = report_to_json(&report_a);
        let json_b = report_to_json(&report_b);
        assert_eq!(
            json_a, json_b,
            "harness output must be byte-stable across runs"
        );
        cleanup(dir_a);
        cleanup(dir_b);
    }

    #[test]
    fn every_trigger_appears_at_least_once() {
        let dir = scratchdir();
        write_fixtures(&dir).unwrap();
        let report = run_harness(&dir).unwrap();
        let triggers: std::collections::BTreeSet<&'static str> = report
            .scenarios
            .iter()
            .filter_map(|s| s.trigger.map(|t| t.as_str()))
            .collect();
        assert!(triggers.contains("size_threshold"), "{:?}", triggers);
        assert!(triggers.contains("classification"), "{:?}", triggers);
        assert!(triggers.contains("decode_posture"), "{:?}", triggers);
        assert!(triggers.contains("operator_override"), "{:?}", triggers);
        cleanup(dir);
    }

    #[test]
    fn limited_mode_scenarios_never_load_whole_file_into_ram() {
        let dir = scratchdir();
        write_fixtures(&dir).unwrap();
        let report = run_harness(&dir).unwrap();
        for scenario in &report.scenarios {
            if scenario.mode == FileMode::LargeFile {
                let cap_bytes = scenario.page_size * scenario.max_resident_pages;
                assert!(
                    scenario.reader_metrics.bytes_resident_high_water <= cap_bytes,
                    "{} resident high water {} exceeds cap {cap_bytes}",
                    scenario.label,
                    scenario.reader_metrics.bytes_resident_high_water
                );
                assert!(
                    scenario.reader_metrics.bytes_resident_high_water <= scenario.fixture_size
                        || scenario.fixture_size <= cap_bytes,
                    "{} resident high water exceeds fixture size",
                    scenario.label
                );
            }
        }
        cleanup(dir);
    }

    #[test]
    fn limited_mode_denies_whole_file_save_participants() {
        let dir = scratchdir();
        write_fixtures(&dir).unwrap();
        let report = run_harness(&dir).unwrap();
        for scenario in &report.scenarios {
            if scenario.mode == FileMode::LargeFile {
                let denied = scenario.steps.iter().any(|s| {
                    s.kind == "attempt_save"
                        && s.detail == "WithWholeFileParticipants"
                        && s.outcome.starts_with("denied")
                });
                assert!(denied, "{} did not deny whole-file save", scenario.label);
            }
        }
        cleanup(dir);
    }

    #[test]
    fn aggregate_reports_at_least_one_save_denied() {
        let dir = scratchdir();
        write_fixtures(&dir).unwrap();
        let report = run_harness(&dir).unwrap();
        assert!(report.aggregate.total_saves_denied >= 1);
        cleanup(dir);
    }
}
