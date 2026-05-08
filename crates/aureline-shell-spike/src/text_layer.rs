//! Text-layer integration seam.
//!
//! Wires the prototype text stack from `aureline-text` into the spike
//! as a side-channel. The default fixture scene does not route through
//! this module (that would alter the committed trace samples under
//! `artifacts/render/`); instead, the spike binary's
//! `--text-stack-smoke` mode runs the committed shaping corpus through
//! it and prints per-case counts. The intent is to expose the same
//! [`aureline_text::prototype::TextLayer`] the benchmark lab drives, so
//! the spike and the bench harness cannot drift.

use std::fmt::Write as _;

use aureline_text::prototype::{FallbackStage, FeatureSet, ShapedRun, TextLayer};

use crate::capabilities::quote;
use crate::hooks::Hook;

/// One case resolved through the text layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeCaseResult {
    pub label: String,
    pub cluster_count: usize,
    pub missing_glyph_count: u32,
    pub fired_fallback_hook: bool,
    pub fallback_stage_counts: [u64; 5],
}

impl SmokeCaseResult {
    fn from_run(label: &str, run: &ShapedRun) -> Self {
        Self {
            label: label.to_owned(),
            cluster_count: run.clusters.len(),
            missing_glyph_count: run.missing_glyph_count,
            fired_fallback_hook: run.fired_fallback_hook(),
            fallback_stage_counts: run.fallback_stage_counts(),
        }
    }
}

/// Aggregate summary of a smoke run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SmokeRunSummary {
    pub cases: Vec<SmokeCaseResult>,
    pub shape_cache_hits: u64,
    pub shape_cache_misses: u64,
    pub raster_cache_hits: u64,
    pub raster_cache_misses: u64,
    pub total_clusters: u64,
    pub missing_glyph_count: u64,
    pub fired_fallback_hook_cases: u64,
}

/// Run the given `(label, text)` cases through one shared text layer,
/// twice each so the caches are exercised. The output is deterministic
/// and does NOT touch the fixture scene's trace samples.
pub fn run_smoke_cases(cases: &[(String, String)]) -> SmokeRunSummary {
    let mut layer = TextLayer::new_default();
    let mut case_results: Vec<SmokeCaseResult> = Vec::with_capacity(cases.len());
    let mut fired_cases: u64 = 0;
    for (label, text) in cases {
        let mut last_run: Option<ShapedRun> = None;
        for _ in 0..2 {
            last_run = Some(layer.render(text, FeatureSet::plain()));
        }
        let run = last_run.expect("each case must render at least once");
        let result = SmokeCaseResult::from_run(label, &run);
        if result.fired_fallback_hook {
            fired_cases += 1;
        }
        case_results.push(result);
    }
    let metrics = layer.metrics();
    SmokeRunSummary {
        cases: case_results,
        shape_cache_hits: metrics.shape_cache_hits,
        shape_cache_misses: metrics.shape_cache_misses,
        raster_cache_hits: metrics.raster_cache_hits,
        raster_cache_misses: metrics.raster_cache_misses,
        total_clusters: metrics.cluster_count,
        missing_glyph_count: metrics.missing_glyph_count,
        fired_fallback_hook_cases: fired_cases,
    }
}

/// Map a shaped-run's observed fallback behaviour to the spike's
/// canonical hook vocabulary. Used by callers that want to report a
/// hook name without instantiating a recorder.
pub fn hook_for_run(run: &ShapedRun) -> Option<Hook> {
    if run.fired_fallback_hook() {
        Some(Hook::FallbackGlyphResolution)
    } else {
        None
    }
}

/// Render a summary as deterministic pretty JSON (same style as the
/// capability manifest).
pub fn summary_to_json(summary: &SmokeRunSummary) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    kv_u64(&mut out, 1, "total_clusters", summary.total_clusters, false);
    kv_u64(
        &mut out,
        1,
        "shape_cache_hits",
        summary.shape_cache_hits,
        false,
    );
    kv_u64(
        &mut out,
        1,
        "shape_cache_misses",
        summary.shape_cache_misses,
        false,
    );
    kv_u64(
        &mut out,
        1,
        "raster_cache_hits",
        summary.raster_cache_hits,
        false,
    );
    kv_u64(
        &mut out,
        1,
        "raster_cache_misses",
        summary.raster_cache_misses,
        false,
    );
    kv_u64(
        &mut out,
        1,
        "missing_glyph_count",
        summary.missing_glyph_count,
        false,
    );
    kv_u64(
        &mut out,
        1,
        "fired_fallback_hook_cases",
        summary.fired_fallback_hook_cases,
        false,
    );
    key(&mut out, 1, "cases");
    out.push_str(" [\n");
    for (i, case) in summary.cases.iter().enumerate() {
        let last = i + 1 == summary.cases.len();
        indent(&mut out, 2);
        out.push_str("{\n");
        kv_str(&mut out, 3, "label", &case.label, false);
        kv_u64(
            &mut out,
            3,
            "cluster_count",
            case.cluster_count as u64,
            false,
        );
        kv_u64(
            &mut out,
            3,
            "missing_glyph_count",
            u64::from(case.missing_glyph_count),
            false,
        );
        kv_bool(
            &mut out,
            3,
            "fired_fallback_hook",
            case.fired_fallback_hook,
            false,
        );
        key(&mut out, 3, "fallback_stage_histogram");
        out.push_str(" {\n");
        for (si, stage) in FallbackStage::ALL.iter().enumerate() {
            let stage_last = si + 1 == FallbackStage::ALL.len();
            let idx = (stage.stage_number() - 1) as usize;
            kv_u64(
                &mut out,
                4,
                stage.name(),
                case.fallback_stage_counts[idx],
                stage_last,
            );
        }
        indent(&mut out, 3);
        out.push_str("}\n");
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

fn kv_u64(out: &mut String, depth: usize, k: &str, value: u64, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{k}\": {value}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn kv_bool(out: &mut String, depth: usize, k: &str, value: bool, last: bool) {
    indent(out, depth);
    let v = if value { "true" } else { "false" };
    let _ = write!(out, "\"{k}\": {v}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn kv_str(out: &mut String, depth: usize, k: &str, value: &str, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{k}\": {}", quote(value));
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_cases() -> Vec<(String, String)> {
        vec![
            ("ascii_basic".to_owned(), "hello".to_owned()),
            ("cjk_han".to_owned(), "漢字".to_owned()),
            ("arabic_plain".to_owned(), "مرحبا".to_owned()),
        ]
    }

    #[test]
    fn smoke_run_counts_fallback_cases() {
        let cases = sample_cases();
        let summary = run_smoke_cases(&cases);
        assert_eq!(summary.cases.len(), 3);
        let ascii = summary
            .cases
            .iter()
            .find(|c| c.label == "ascii_basic")
            .unwrap();
        assert!(!ascii.fired_fallback_hook);
        let cjk = summary.cases.iter().find(|c| c.label == "cjk_han").unwrap();
        assert!(cjk.fired_fallback_hook);
        assert_eq!(summary.fired_fallback_hook_cases, 2);
    }

    #[test]
    fn summary_json_is_byte_stable() {
        let cases = sample_cases();
        let a = summary_to_json(&run_smoke_cases(&cases));
        let b = summary_to_json(&run_smoke_cases(&cases));
        assert_eq!(a, b);
    }

    #[test]
    fn hook_for_run_names_the_adr_hook() {
        let mut layer = TextLayer::new_default();
        let cjk = layer.render("漢", FeatureSet::plain());
        assert_eq!(hook_for_run(&cjk), Some(Hook::FallbackGlyphResolution));
        let latin = layer.render("hi", FeatureSet::plain());
        assert_eq!(hook_for_run(&latin), None);
    }
}
