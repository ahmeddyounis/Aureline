//! Text-stack smoke harness.
//!
//! Drives the prototype shaper from `aureline-text` against a corpus of
//! shaping smoke cases and emits a structured metrics record. The
//! record is deterministic (counts, not wall-clock times) so the
//! committed seed artifact under `artifacts/bench/` can be diffed
//! without re-running the harness.
//!
//! The harness is intentionally tiny: parse a labelled TSV corpus,
//! run the text layer once per case per iteration, aggregate counts.
//! The benchmark lab layers wall-clock timing on top when it needs it.

use std::fmt::Write as _;

use aureline_text::prototype::{FeatureSet, ShapedRun, TextLayer};

/// The canonical corpus identifier written into emitted artifacts.
pub const CORPUS_ID: &str = "aureline.shaping_smoke_cases.v1";

/// Schema version for the emitted metrics JSON.
pub const METRICS_SCHEMA_VERSION: u32 = 1;

/// One labelled case parsed out of the corpus file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorpusCase {
    pub label: String,
    pub text: String,
}

/// Errors the corpus parser can raise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CorpusParseError {
    /// A non-blank, non-comment line is missing a tab separator.
    MissingTab { line_number: usize },
    /// Two cases share the same label.
    DuplicateLabel { label: String, line_number: usize },
    /// Label contains characters outside `[a-z0-9_]`.
    InvalidLabel { label: String, line_number: usize },
}

impl std::fmt::Display for CorpusParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingTab { line_number } => {
                write!(f, "line {line_number}: expected <label><TAB><text>")
            }
            Self::DuplicateLabel { label, line_number } => {
                write!(f, "line {line_number}: duplicate label {label:?}")
            }
            Self::InvalidLabel { label, line_number } => {
                write!(
                    f,
                    "line {line_number}: label {label:?} must match [a-z0-9_]+"
                )
            }
        }
    }
}

impl std::error::Error for CorpusParseError {}

/// Parse a corpus file. Lines starting with `#` are comments; blank
/// lines are ignored; other lines are `label<TAB>text`.
pub fn parse_corpus(contents: &str) -> Result<Vec<CorpusCase>, CorpusParseError> {
    let mut cases = Vec::new();
    for (idx, raw) in contents.lines().enumerate() {
        let line_number = idx + 1;
        let line = raw.trim_end_matches('\r');
        if line.is_empty() {
            continue;
        }
        if line.starts_with('#') {
            continue;
        }
        let (label, text) = match line.split_once('\t') {
            Some(parts) => parts,
            None => return Err(CorpusParseError::MissingTab { line_number }),
        };
        if !is_valid_label(label) {
            return Err(CorpusParseError::InvalidLabel {
                label: label.to_owned(),
                line_number,
            });
        }
        if cases.iter().any(|c: &CorpusCase| c.label == label) {
            return Err(CorpusParseError::DuplicateLabel {
                label: label.to_owned(),
                line_number,
            });
        }
        cases.push(CorpusCase {
            label: label.to_owned(),
            text: text.to_owned(),
        });
    }
    Ok(cases)
}

fn is_valid_label(label: &str) -> bool {
    !label.is_empty()
        && label
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Per-case metrics. The counts are structural (no wall-clock) so the
/// seed artifact is byte-stable across hosts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseMetrics {
    pub label: String,
    pub byte_count: u64,
    pub codepoint_count: u64,
    pub cluster_count: u64,
    pub missing_glyph_count: u64,
    pub fired_fallback_hook: bool,
    pub fallback_stage_counts: [u64; 5],
    pub scripts: Vec<(&'static str, u64)>,
}

fn case_metrics(label: &str, text: &str, run: &ShapedRun) -> CaseMetrics {
    let mut scripts_hist = std::collections::BTreeMap::<&'static str, u64>::new();
    for cluster in &run.clusters {
        *scripts_hist.entry(cluster.script.name()).or_default() += 1;
    }
    CaseMetrics {
        label: label.to_owned(),
        byte_count: text.len() as u64,
        codepoint_count: text.chars().count() as u64,
        cluster_count: run.clusters.len() as u64,
        missing_glyph_count: u64::from(run.missing_glyph_count),
        fired_fallback_hook: run.fired_fallback_hook(),
        fallback_stage_counts: run.fallback_stage_counts(),
        scripts: scripts_hist.into_iter().collect(),
    }
}

/// Aggregate across a full harness run.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct AggregateMetrics {
    pub total_cases: u64,
    pub total_iterations: u64,
    pub total_clusters: u64,
    pub shape_calls: u64,
    pub shape_cache_hits: u64,
    pub shape_cache_misses: u64,
    pub raster_cache_hits: u64,
    pub raster_cache_misses: u64,
    pub missing_glyph_count: u64,
    pub fired_fallback_hook_cases: u64,
    pub fallback_stage_counts: [u64; 5],
}

/// Full harness output.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HarnessReport {
    pub schema_version: u32,
    pub corpus_id: &'static str,
    pub iterations: u32,
    pub shaper_policy: &'static str,
    pub cases: Vec<CaseMetrics>,
    pub aggregate: AggregateMetrics,
}

/// Run the harness. `iterations` must be >= 1; a second iteration is
/// what exercises the shape and raster caches.
pub fn run_harness(cases: &[CorpusCase], iterations: u32) -> HarnessReport {
    assert!(iterations >= 1, "iterations must be at least 1");
    let mut layer = TextLayer::new_default();
    let mut case_reports: Vec<CaseMetrics> = Vec::with_capacity(cases.len());
    let mut fired_cases: u64 = 0;
    for case in cases {
        let mut last_run: Option<ShapedRun> = None;
        for _ in 0..iterations {
            let run = layer.render(&case.text, FeatureSet::plain());
            last_run = Some(run);
        }
        let run = last_run.expect("iterations >= 1");
        let case_metric = case_metrics(&case.label, &case.text, &run);
        if case_metric.fired_fallback_hook {
            fired_cases += 1;
        }
        case_reports.push(case_metric);
    }
    let totals = layer.metrics();
    let aggregate = AggregateMetrics {
        total_cases: cases.len() as u64,
        total_iterations: u64::from(iterations),
        total_clusters: totals.cluster_count,
        shape_calls: totals.shape_calls,
        shape_cache_hits: totals.shape_cache_hits,
        shape_cache_misses: totals.shape_cache_misses,
        raster_cache_hits: totals.raster_cache_hits,
        raster_cache_misses: totals.raster_cache_misses,
        missing_glyph_count: totals.missing_glyph_count,
        fired_fallback_hook_cases: fired_cases,
        fallback_stage_counts: totals.fallback_stage_counts,
    };
    HarnessReport {
        schema_version: METRICS_SCHEMA_VERSION,
        corpus_id: CORPUS_ID,
        iterations,
        shaper_policy: layer.policy().name(),
        cases: case_reports,
        aggregate,
    }
}

/// Render as deterministic pretty JSON.
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
    kv_u64(
        &mut out,
        1,
        "iterations",
        u64::from(report.iterations),
        false,
    );
    kv_str(&mut out, 1, "shaper_policy", report.shaper_policy, false);

    key(&mut out, 1, "aggregate");
    out.push_str(" {\n");
    let agg = &report.aggregate;
    kv_u64(&mut out, 2, "total_cases", agg.total_cases, false);
    kv_u64(&mut out, 2, "total_iterations", agg.total_iterations, false);
    kv_u64(&mut out, 2, "total_clusters", agg.total_clusters, false);
    kv_u64(&mut out, 2, "shape_calls", agg.shape_calls, false);
    kv_u64(&mut out, 2, "shape_cache_hits", agg.shape_cache_hits, false);
    kv_u64(
        &mut out,
        2,
        "shape_cache_misses",
        agg.shape_cache_misses,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "raster_cache_hits",
        agg.raster_cache_hits,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "raster_cache_misses",
        agg.raster_cache_misses,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "missing_glyph_count",
        agg.missing_glyph_count,
        false,
    );
    kv_u64(
        &mut out,
        2,
        "fired_fallback_hook_cases",
        agg.fired_fallback_hook_cases,
        false,
    );
    write_fallback_histogram(&mut out, 2, &agg.fallback_stage_counts, true);
    indent(&mut out, 1);
    out.push_str("},\n");

    key(&mut out, 1, "cases");
    out.push_str(" [\n");
    for (i, case) in report.cases.iter().enumerate() {
        let last = i + 1 == report.cases.len();
        indent(&mut out, 2);
        out.push_str("{\n");
        kv_str(&mut out, 3, "label", &case.label, false);
        kv_u64(&mut out, 3, "byte_count", case.byte_count, false);
        kv_u64(&mut out, 3, "codepoint_count", case.codepoint_count, false);
        kv_u64(&mut out, 3, "cluster_count", case.cluster_count, false);
        kv_u64(
            &mut out,
            3,
            "missing_glyph_count",
            case.missing_glyph_count,
            false,
        );
        kv_bool(
            &mut out,
            3,
            "fired_fallback_hook",
            case.fired_fallback_hook,
            false,
        );
        write_fallback_histogram(&mut out, 3, &case.fallback_stage_counts, false);
        write_scripts(&mut out, 3, &case.scripts, true);
        indent(&mut out, 2);
        if last {
            out.push_str("}\n");
        } else {
            out.push_str("},\n");
        }
    }
    indent(&mut out, 1);
    out.push_str("]\n");
    out.push_str("}\n");
    out
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

fn kv_bool(out: &mut String, depth: usize, key: &str, value: bool, last: bool) {
    indent(out, depth);
    let v = if value { "true" } else { "false" };
    let _ = write!(out, "\"{key}\": {v}");
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

fn write_fallback_histogram(out: &mut String, depth: usize, counts: &[u64; 5], last: bool) {
    use aureline_text::prototype::FallbackStage;
    key(out, depth, "fallback_stage_histogram");
    out.push_str(" {\n");
    for (i, stage) in FallbackStage::ALL.iter().enumerate() {
        let stage_last = i + 1 == FallbackStage::ALL.len();
        let idx = (stage.stage_number() - 1) as usize;
        kv_u64(out, depth + 1, stage.name(), counts[idx], stage_last);
    }
    indent(out, depth);
    if last {
        out.push_str("}\n");
    } else {
        out.push_str("},\n");
    }
}

fn write_scripts(out: &mut String, depth: usize, scripts: &[(&str, u64)], last: bool) {
    key(out, depth, "scripts");
    out.push_str(" {\n");
    for (i, (name, count)) in scripts.iter().enumerate() {
        let script_last = i + 1 == scripts.len();
        kv_u64(out, depth + 1, name, *count, script_last);
    }
    indent(out, depth);
    if last {
        out.push_str("}\n");
    } else {
        out.push_str("},\n");
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

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_CORPUS: &str = "\
# sample
ascii_basic\thello
cjk_han\t漢字
arabic_plain\tمرحبا
";

    #[test]
    fn parses_labelled_corpus() {
        let cases = parse_corpus(SAMPLE_CORPUS).unwrap();
        assert_eq!(cases.len(), 3);
        assert_eq!(cases[0].label, "ascii_basic");
        assert_eq!(cases[1].text, "漢字");
        assert_eq!(cases[2].label, "arabic_plain");
    }

    #[test]
    fn rejects_missing_tab() {
        let err = parse_corpus("bad_line_no_tab").unwrap_err();
        assert!(matches!(
            err,
            CorpusParseError::MissingTab { line_number: 1 }
        ));
    }

    #[test]
    fn rejects_duplicate_label() {
        let corpus = "a\tone\na\ttwo\n";
        let err = parse_corpus(corpus).unwrap_err();
        assert!(matches!(err, CorpusParseError::DuplicateLabel { .. }));
    }

    #[test]
    fn rejects_invalid_label_characters() {
        let err = parse_corpus("Bad-Label\thi\n").unwrap_err();
        assert!(matches!(err, CorpusParseError::InvalidLabel { .. }));
    }

    #[test]
    fn harness_report_is_byte_stable() {
        let cases = parse_corpus(SAMPLE_CORPUS).unwrap();
        let a = run_harness(&cases, 2);
        let b = run_harness(&cases, 2);
        assert_eq!(report_to_json(&a), report_to_json(&b));
    }

    #[test]
    fn second_iteration_uses_caches() {
        let cases = parse_corpus(SAMPLE_CORPUS).unwrap();
        let one = run_harness(&cases, 1);
        let two = run_harness(&cases, 2);
        assert!(two.aggregate.shape_cache_hits > one.aggregate.shape_cache_hits);
        assert!(two.aggregate.raster_cache_hits > one.aggregate.raster_cache_hits);
    }

    #[test]
    fn fired_fallback_hook_tracks_nonlatin_cases() {
        let cases = parse_corpus(SAMPLE_CORPUS).unwrap();
        let report = run_harness(&cases, 1);
        let ascii = report
            .cases
            .iter()
            .find(|c| c.label == "ascii_basic")
            .unwrap();
        let cjk = report.cases.iter().find(|c| c.label == "cjk_han").unwrap();
        assert!(!ascii.fired_fallback_hook);
        assert!(cjk.fired_fallback_hook);
    }

    /// The committed seed artifact is produced by this harness against
    /// the committed corpus. If the prototype or the harness changes the
    /// shape of its output, regenerate the seed via
    /// `tools/bench_text_stack.sh` and commit the new bytes in the
    /// same change.
    #[test]
    fn committed_seed_matches_harness_output() {
        const CORPUS: &str = include_str!("../../../fixtures/text/shaping_smoke_cases.txt");
        const SEED: &str = include_str!("../../../artifacts/bench/text_stack_metrics_seed.json");
        let cases = parse_corpus(CORPUS).expect("committed corpus must parse");
        let report = run_harness(&cases, 2);
        let produced = report_to_json(&report);
        assert_eq!(
            produced, SEED,
            "committed seed artifact drifted from the harness output"
        );
    }
}
