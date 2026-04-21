//! Structured timing-trace records for the shell spike.
//!
//! The existing [`crate::trace`] module emits the minimal hook / tick /
//! note record the fixture-repeatability test freezes. This module is
//! the richer record family the benchmark lab and the later journey
//! harness consume: the same marks, but wrapped with the exact-build
//! identity, the fixture scene id, a provisional trace id, one
//! `protected_path` tag per mark, and a counter block for damage /
//! paint / invalidation-class / visible-vs-hidden-pane / frame-miss /
//! off-screen-suppression bookkeeping.
//!
//! The record is boundary-schema'd at
//! `schemas/traces/spike_timing.schema.json`; the mapping between hook
//! names and protected-path concepts is frozen in
//! `docs/benchmarks/spike_metric_names.md`. Both documents MUST be
//! updated in the same change as this module.

use std::fmt::Write as _;

use crate::capabilities::{quote, Backend};
use crate::fixture_scene::FixtureRunResult;
use crate::hooks::Hook;
use crate::render_path::{DamageRecord, Layer};
use crate::zones::ZoneId;
use crate::SpikeBuildIdentity;

/// Schema id emitted into every record.
pub const SCHEMA_ID: &str = "aureline.spike_timing.v1";

/// Integer version for the schema id above.
pub const SCHEMA_VERSION: u32 = 1;

/// Protected-path journey classification for one hook.
///
/// These buckets are the vocabulary `docs/benchmarks/spike_metric_names.md`
/// resolves every emitted hook into. The mapping is normative: two
/// hooks that map to the same protected path are measured against the
/// same journey budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProtectedPath {
    Startup,
    FirstUsefulChrome,
    FirstPaint,
    InputToPaint,
    RenderSubmission,
    FrameBudget,
    PlaceholderOpen,
    PlaceholderEdit,
    PlaceholderSave,
    FallbackResolution,
    Observability,
}

impl ProtectedPath {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Startup => "startup",
            Self::FirstUsefulChrome => "first_useful_chrome",
            Self::FirstPaint => "first_paint",
            Self::InputToPaint => "input_to_paint",
            Self::RenderSubmission => "render_submission",
            Self::FrameBudget => "frame_budget",
            Self::PlaceholderOpen => "placeholder_open",
            Self::PlaceholderEdit => "placeholder_edit",
            Self::PlaceholderSave => "placeholder_save",
            Self::FallbackResolution => "fallback_resolution",
            Self::Observability => "observability",
        }
    }

    /// The full enumeration in documentation order. The capability manifest
    /// and the mapping doc emit the list in this order; no trace consumer
    /// MAY reorder it.
    pub const ALL: &'static [ProtectedPath] = &[
        Self::Startup,
        Self::FirstUsefulChrome,
        Self::FirstPaint,
        Self::InputToPaint,
        Self::RenderSubmission,
        Self::FrameBudget,
        Self::PlaceholderOpen,
        Self::PlaceholderEdit,
        Self::PlaceholderSave,
        Self::FallbackResolution,
        Self::Observability,
    ];
}

/// Map a hook to the protected-path bucket the mapping doc assigns it.
pub const fn protected_path_for(hook: Hook) -> ProtectedPath {
    match hook {
        Hook::WarmStartToFirstPaint => ProtectedPath::Startup,
        Hook::FirstPaint => ProtectedPath::FirstPaint,
        Hook::CaretMove
        | Hook::SelectionChange
        | Hook::ImeCompositionUpdate
        | Hook::ReflowLineRange
        | Hook::ScrollFrame
        | Hook::MultiMonitorScaleChange => ProtectedPath::InputToPaint,
        Hook::FrameSubmit => ProtectedPath::RenderSubmission,
        Hook::FallbackGlyphResolution
        | Hook::AtlasShardRebind
        | Hook::AtlasEviction => ProtectedPath::FallbackResolution,
        Hook::DegradedRendererBanner | Hook::AccessibilityTreeUpdate => {
            ProtectedPath::Observability
        }
    }
}

/// One emitted mark, enriched with path, span, and protected-path
/// classification. `span_id` is provisional: the spike assigns a
/// deterministic `{hook}.{tick}` id so later taxonomy work can replace
/// the allocator without renaming fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpikeMark {
    pub hook: Hook,
    pub tick: u64,
    pub protected_path: ProtectedPath,
    pub protected_hot_path: bool,
    pub scenario_id: String,
    pub path_id: &'static str,
    pub span_id: String,
    pub note: Option<String>,
}

/// Counter block that accompanies the marks. Every field carries an
/// explicit zero in the emitted JSON even when the spike does not
/// exercise the dimension yet; the benchmark lab consumes the shape
/// and fills in real data once the hook fires.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SpikeCounters {
    pub total_marks: u64,
    pub hot_path_marks: u64,
    pub damage_records: u64,
    pub paint_count_by_zone_editor_viewport: u64,
    pub paint_count_by_zone_sidebar: u64,
    pub paint_count_by_zone_title_bar: u64,
    pub paint_count_by_zone_status_bar: u64,
    pub paint_count_by_layer_text_and_decoration: u64,
    pub paint_count_by_layer_overlay: u64,
    pub invalidation_class_startup: u64,
    pub invalidation_class_first_paint: u64,
    pub invalidation_class_input_to_paint: u64,
    pub invalidation_class_render_submission: u64,
    pub invalidation_class_fallback_resolution: u64,
    pub invalidation_class_observability: u64,
    pub visible_pane_work: u64,
    pub hidden_pane_work: u64,
    pub frame_misses: u64,
    pub offscreen_suppression_eligible: u64,
    pub fallback_glyph_resolutions: u64,
}

/// Full structured timing-trace record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpikeTimingTrace {
    pub schema: &'static str,
    pub schema_version: u32,
    pub scene_id: &'static str,
    pub fixture_ref: &'static str,
    pub journey_id: String,
    pub trace_id: String,
    pub build: SpikeBuildIdentity,
    pub backend: Backend,
    pub host_os: &'static str,
    pub counters: SpikeCounters,
    pub marks: Vec<SpikeMark>,
}

impl SpikeTimingTrace {
    /// Build the aggregate "full_scene" trace for a fixture run.
    pub fn from_full_run(run: &FixtureRunResult, backend: Backend) -> Self {
        let damage: Vec<DamageRecord> = run
            .frames
            .iter()
            .filter_map(|f| f.damage.clone())
            .collect();
        let marks = build_marks(run, "full_scene");
        let counters = compute_counters(&marks, &damage);
        Self {
            schema: SCHEMA_ID,
            schema_version: SCHEMA_VERSION,
            scene_id: run.scene_id,
            fixture_ref: run.scene_id,
            journey_id: format!("{}.full_scene", run.scene_id),
            trace_id: format!("{}.full_scene", run.scene_id),
            build: SpikeBuildIdentity::current(),
            backend,
            host_os: host_os(),
            counters,
            marks,
        }
    }

    /// Build a per-label trace containing only the mark produced by one
    /// fixture-script step. The counter block is the slice's counters,
    /// not the aggregate's, so per-label samples are self-contained.
    pub fn from_label(
        run: &FixtureRunResult,
        label: &'static str,
        backend: Backend,
    ) -> Option<Self> {
        let frame = run.frames.iter().find(|f| f.label == label)?;
        let mark = frame.mark.clone()?;
        let enriched = enrich_mark(&mark, label);
        let damage: Vec<DamageRecord> = frame.damage.clone().into_iter().collect();
        let counters = compute_counters(std::slice::from_ref(&enriched), &damage);
        Some(Self {
            schema: SCHEMA_ID,
            schema_version: SCHEMA_VERSION,
            scene_id: run.scene_id,
            fixture_ref: run.scene_id,
            journey_id: format!("{}.{}", run.scene_id, label),
            trace_id: format!("{}.{}", run.scene_id, label),
            build: SpikeBuildIdentity::current(),
            backend,
            host_os: host_os(),
            counters,
            marks: vec![enriched],
        })
    }

    /// Build one per-label trace per fixture step that produced a hook,
    /// plus one aggregate "full_scene" trace. Tuple is (filename, trace).
    pub fn per_label_plus_full(
        run: &FixtureRunResult,
        backend: Backend,
    ) -> Vec<(String, SpikeTimingTrace)> {
        let mut out: Vec<(String, SpikeTimingTrace)> = Vec::new();
        for frame in &run.frames {
            if let Some(trace) = Self::from_label(run, frame.label, backend) {
                out.push((format!("{}.json", frame.label), trace));
            }
        }
        out.push((
            "full_scene.json".to_owned(),
            Self::from_full_run(run, backend),
        ));
        out
    }

    /// Deterministic pretty JSON. Field order is fixed so committed
    /// artifacts under `artifacts/traces/examples/` stay diffable.
    pub fn to_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n");
        writeln_kv(&mut out, 1, "schema", &quote(self.schema), false);
        writeln_kv(
            &mut out,
            1,
            "schema_version",
            &self.schema_version.to_string(),
            false,
        );
        writeln_kv(&mut out, 1, "scene_id", &quote(self.scene_id), false);
        writeln_kv(&mut out, 1, "fixture_ref", &quote(self.fixture_ref), false);
        writeln_kv(&mut out, 1, "journey_id", &quote(&self.journey_id), false);
        writeln_kv(&mut out, 1, "trace_id", &quote(&self.trace_id), false);

        // build block
        writeln_key(&mut out, 1, "build");
        out.push_str(" {\n");
        writeln_kv(&mut out, 2, "crate_name", &quote(self.build.crate_name), false);
        writeln_kv(
            &mut out,
            2,
            "crate_version",
            &quote(self.build.crate_version),
            false,
        );
        writeln_kv(
            &mut out,
            2,
            "rustc_target_triple",
            &quote(self.build.rustc_target_triple),
            true,
        );
        indent(&mut out, 1);
        out.push_str("},\n");

        writeln_kv(&mut out, 1, "backend", &quote(self.backend.name()), false);
        writeln_kv(&mut out, 1, "host_os", &quote(self.host_os), false);

        // counters block
        writeln_key(&mut out, 1, "counters");
        out.push_str(" {\n");
        let c = &self.counters;
        writeln_num(&mut out, 2, "total_marks", c.total_marks, false);
        writeln_num(&mut out, 2, "hot_path_marks", c.hot_path_marks, false);
        writeln_num(&mut out, 2, "damage_records", c.damage_records, false);

        writeln_key(&mut out, 2, "paint_count_by_zone");
        out.push_str(" {\n");
        writeln_num(
            &mut out,
            3,
            "editor_viewport",
            c.paint_count_by_zone_editor_viewport,
            false,
        );
        writeln_num(
            &mut out,
            3,
            "sidebar",
            c.paint_count_by_zone_sidebar,
            false,
        );
        writeln_num(
            &mut out,
            3,
            "title_bar",
            c.paint_count_by_zone_title_bar,
            false,
        );
        writeln_num(
            &mut out,
            3,
            "status_bar",
            c.paint_count_by_zone_status_bar,
            true,
        );
        indent(&mut out, 2);
        out.push_str("},\n");

        writeln_key(&mut out, 2, "paint_count_by_layer");
        out.push_str(" {\n");
        writeln_num(
            &mut out,
            3,
            "text_and_decoration",
            c.paint_count_by_layer_text_and_decoration,
            false,
        );
        writeln_num(
            &mut out,
            3,
            "overlay",
            c.paint_count_by_layer_overlay,
            true,
        );
        indent(&mut out, 2);
        out.push_str("},\n");

        writeln_key(&mut out, 2, "invalidation_class_counts");
        out.push_str(" {\n");
        writeln_num(&mut out, 3, "startup", c.invalidation_class_startup, false);
        writeln_num(
            &mut out,
            3,
            "first_paint",
            c.invalidation_class_first_paint,
            false,
        );
        writeln_num(
            &mut out,
            3,
            "input_to_paint",
            c.invalidation_class_input_to_paint,
            false,
        );
        writeln_num(
            &mut out,
            3,
            "render_submission",
            c.invalidation_class_render_submission,
            false,
        );
        writeln_num(
            &mut out,
            3,
            "fallback_resolution",
            c.invalidation_class_fallback_resolution,
            false,
        );
        writeln_num(
            &mut out,
            3,
            "observability",
            c.invalidation_class_observability,
            true,
        );
        indent(&mut out, 2);
        out.push_str("},\n");

        writeln_num(&mut out, 2, "visible_pane_work", c.visible_pane_work, false);
        writeln_num(&mut out, 2, "hidden_pane_work", c.hidden_pane_work, false);
        writeln_num(&mut out, 2, "frame_misses", c.frame_misses, false);
        writeln_num(
            &mut out,
            2,
            "offscreen_suppression_eligible",
            c.offscreen_suppression_eligible,
            false,
        );
        writeln_num(
            &mut out,
            2,
            "fallback_glyph_resolutions",
            c.fallback_glyph_resolutions,
            true,
        );
        indent(&mut out, 1);
        out.push_str("},\n");

        // marks array
        writeln_key(&mut out, 1, "marks");
        out.push_str(" [\n");
        for (i, mark) in self.marks.iter().enumerate() {
            let last = i + 1 == self.marks.len();
            indent(&mut out, 2);
            out.push_str("{\n");
            writeln_kv(&mut out, 3, "hook", &quote(mark.hook.name()), false);
            writeln_kv(
                &mut out,
                3,
                "protected_path",
                &quote(mark.protected_path.name()),
                false,
            );
            writeln_kv(
                &mut out,
                3,
                "protected_hot_path",
                if mark.protected_hot_path { "true" } else { "false" },
                false,
            );
            writeln_kv(&mut out, 3, "path_id", &quote(mark.path_id), false);
            writeln_kv(&mut out, 3, "scenario_id", &quote(&mark.scenario_id), false);
            writeln_kv(&mut out, 3, "span_id", &quote(&mark.span_id), false);
            writeln_num(&mut out, 3, "tick", mark.tick, false);
            let note_value = match &mark.note {
                Some(note) => quote(note),
                None => "null".to_owned(),
            };
            indent(&mut out, 3);
            let _ = write!(out, "\"note\": {note_value}\n");
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
}

fn build_marks(run: &FixtureRunResult, journey_label: &'static str) -> Vec<SpikeMark> {
    // Synthesize a scenario label for each mark: per-frame marks inherit
    // the step label; the warm-start pair and the trailing frame_submit
    // are synthesized from the scene's note field.
    let mut marks = Vec::new();
    for raw in &run.marks {
        let scenario = raw
            .note
            .clone()
            .unwrap_or_else(|| journey_label.to_owned());
        let scenario_id = match scenario.as_str() {
            "scene.begin" => "scene.begin".to_owned(),
            "scene.first_paint" => "scene.first_paint".to_owned(),
            "scene.end" => "scene.end".to_owned(),
            other => other.to_owned(),
        };
        let enriched = SpikeMark {
            hook: raw.hook,
            tick: raw.tick.0,
            protected_path: protected_path_for(raw.hook),
            protected_hot_path: raw.hook.is_hot_path(),
            scenario_id: scenario_id.clone(),
            path_id: raw.hook.name(),
            span_id: format!("{}.{}", raw.hook.name(), raw.tick.0),
            note: raw.note.clone(),
        };
        marks.push(enriched);
    }
    marks
}

fn enrich_mark(raw: &crate::frame_timing::TimingMark, label: &'static str) -> SpikeMark {
    SpikeMark {
        hook: raw.hook,
        tick: raw.tick.0,
        protected_path: protected_path_for(raw.hook),
        protected_hot_path: raw.hook.is_hot_path(),
        scenario_id: label.to_owned(),
        path_id: raw.hook.name(),
        span_id: format!("{}.{}", raw.hook.name(), raw.tick.0),
        note: raw.note.clone(),
    }
}

fn compute_counters(marks: &[SpikeMark], damage: &[DamageRecord]) -> SpikeCounters {
    let mut c = SpikeCounters::default();
    c.total_marks = marks.len() as u64;
    for m in marks {
        if m.protected_hot_path {
            c.hot_path_marks += 1;
        }
        match m.protected_path {
            ProtectedPath::Startup => c.invalidation_class_startup += 1,
            ProtectedPath::FirstPaint => c.invalidation_class_first_paint += 1,
            ProtectedPath::InputToPaint => c.invalidation_class_input_to_paint += 1,
            ProtectedPath::RenderSubmission => c.invalidation_class_render_submission += 1,
            ProtectedPath::FallbackResolution => c.invalidation_class_fallback_resolution += 1,
            ProtectedPath::Observability => c.invalidation_class_observability += 1,
            // Not yet fired by the spike; the counter block reserves
            // zero so the schema doesn't drift when these wire in.
            ProtectedPath::FirstUsefulChrome
            | ProtectedPath::FrameBudget
            | ProtectedPath::PlaceholderOpen
            | ProtectedPath::PlaceholderEdit
            | ProtectedPath::PlaceholderSave => {}
        }
        if matches!(m.hook, Hook::FallbackGlyphResolution) {
            c.fallback_glyph_resolutions += 1;
        }
    }
    c.damage_records = damage.len() as u64;
    for record in damage {
        match record.zone {
            ZoneId::EditorViewport => c.paint_count_by_zone_editor_viewport += 1,
            ZoneId::Sidebar => c.paint_count_by_zone_sidebar += 1,
            ZoneId::TitleBar => c.paint_count_by_zone_title_bar += 1,
            ZoneId::StatusBar => c.paint_count_by_zone_status_bar += 1,
        }
        match record.layer {
            Layer::TextAndDecoration => c.paint_count_by_layer_text_and_decoration += 1,
            Layer::Overlay => c.paint_count_by_layer_overlay += 1,
        }
    }
    // In the fixture scene every zone is visible, so all damage is
    // visible-pane work. Hidden-pane and off-screen-suppression
    // accounting is provisional: the spike has no hidden panes and no
    // viewport clipping; later work wires these counters without
    // renaming them.
    c.visible_pane_work = c.damage_records;
    c.hidden_pane_work = 0;
    c.frame_misses = 0;
    c.offscreen_suppression_eligible = 0;
    c
}

fn host_os() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(target_os = "windows")]
    {
        "windows"
    }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        "unknown"
    }
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn writeln_key(out: &mut String, depth: usize, key: &str) {
    indent(out, depth);
    out.push('"');
    out.push_str(key);
    out.push_str("\":");
}

fn writeln_kv(out: &mut String, depth: usize, key: &str, value: &str, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {value}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn writeln_num(out: &mut String, depth: usize, key: &str, value: u64, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {value}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture_scene::run;
    use crate::frame_timing::CountingClock;
    use crate::zones::ShellFrame;

    #[test]
    fn full_scene_trace_contains_every_fired_hook() {
        let result = run(&ShellFrame::fixture(), &CountingClock::new());
        let trace = SpikeTimingTrace::from_full_run(&result, Backend::Headless);
        assert_eq!(trace.marks.len(), result.marks.len());
        for (raw, enriched) in result.marks.iter().zip(trace.marks.iter()) {
            assert_eq!(raw.hook, enriched.hook);
            assert_eq!(raw.tick.0, enriched.tick);
            assert_eq!(enriched.path_id, raw.hook.name());
        }
    }

    #[test]
    fn every_hook_maps_to_exactly_one_protected_path() {
        for hook in Hook::ALL {
            let _ = protected_path_for(*hook);
        }
    }

    #[test]
    fn counters_sum_marks_and_damage() {
        let result = run(&ShellFrame::fixture(), &CountingClock::new());
        let trace = SpikeTimingTrace::from_full_run(&result, Backend::Headless);
        assert_eq!(trace.counters.total_marks as usize, result.marks.len());
        let composited = result.as_composited_frame();
        assert_eq!(
            trace.counters.damage_records as usize,
            composited.damage.len()
        );
    }

    #[test]
    fn per_label_plus_full_has_one_entry_per_fired_step_and_one_aggregate() {
        let result = run(&ShellFrame::fixture(), &CountingClock::new());
        let traces = SpikeTimingTrace::per_label_plus_full(&result, Backend::Headless);
        let names: Vec<_> = traces.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"full_scene.json"));
        assert!(names.contains(&"caret_left.json"));
        // Every fixture-script step fires a hook, so we expect one
        // per-label entry per step plus the aggregate.
        assert_eq!(traces.len(), result.frames.len() + 1);
    }

    #[test]
    fn json_contains_schema_build_and_counter_fields() {
        let result = run(&ShellFrame::fixture(), &CountingClock::new());
        let trace = SpikeTimingTrace::from_full_run(&result, Backend::Headless);
        let json = trace.to_json();
        assert!(json.contains("\"schema\": \"aureline.spike_timing.v1\""));
        assert!(json.contains("\"crate_name\": \"aureline-shell-spike\""));
        assert!(json.contains("\"counters\""));
        assert!(json.contains("\"paint_count_by_zone\""));
        assert!(json.contains("\"invalidation_class_counts\""));
        assert!(json.contains("\"frame_misses\""));
        assert!(json.contains("\"protected_path\""));
    }

    #[test]
    fn warm_start_rides_the_startup_path() {
        assert_eq!(
            protected_path_for(Hook::WarmStartToFirstPaint),
            ProtectedPath::Startup
        );
        assert_eq!(protected_path_for(Hook::FirstPaint), ProtectedPath::FirstPaint);
        assert_eq!(
            protected_path_for(Hook::FrameSubmit),
            ProtectedPath::RenderSubmission
        );
        assert_eq!(
            protected_path_for(Hook::CaretMove),
            ProtectedPath::InputToPaint
        );
    }
}
