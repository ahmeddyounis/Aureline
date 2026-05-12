//! Hot-path metrics and trace-hook recorder.
//!
//! This module provides a small in-memory collector that callers can wire into
//! startup, file-open, typing, and scrolling hot paths. The collector records:
//!
//! - a counter block (cheap, always-on increments), and
//! - an optional trace-event stream (span/mark records) that is only built when
//!   capture is enabled via [`HotPathMetricsConfig::output_path`].
//!
//! The emitted record is intended to be machine-consumable by dashboards and
//! regression checks without requiring a bespoke parser per surface.

use std::borrow::Cow;
use std::fs;
use std::io;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::trace_event::{BuildIdentityRecord, CorpusManifestRefRecord, TraceEventRecord};

/// Schema token for hot-path metrics captures.
pub const SCHEMA: &str = "aureline.hot_path_metrics.v1";

/// Schema version for [`SCHEMA`].
pub const SCHEMA_VERSION: u32 = 1;

/// Record-kind discriminator for hot-path metrics captures.
pub const RECORD_KIND: &str = "hot_path_metrics_record";

/// Configuration for a [`HotPathMetrics`] collector.
#[derive(Debug, Clone, Default)]
pub struct HotPathMetricsConfig {
    /// Optional output path for exporting a JSON capture.
    pub output_path: Option<String>,
}

/// Context shared across every emitted event.
#[derive(Debug, Clone)]
pub struct HotPathMetricsContext {
    /// Trace id for this capture instance.
    pub trace_id: String,
    /// Backend class (for example `native_window`).
    pub backend: Cow<'static, str>,
    /// Host OS class (`macos`, `windows`, `linux`, `unknown`).
    pub host_os: Cow<'static, str>,
    /// Minimum build identity record for the emitter.
    pub build: BuildIdentityRecord,
    /// Optional exact-build identity ref.
    pub exact_build_identity_ref: Option<String>,
    /// Optional hardware-definition ref.
    pub hardware_definition_ref: Option<String>,
    /// Optional environment ref.
    pub environment_ref: Option<String>,
    /// Optional fixture ref.
    pub fixture_ref: Option<String>,
    /// Optional corpus manifest revision pin.
    pub corpus_manifest: Option<CorpusManifestRefRecord>,
    /// Sampling profile class token (`developer_local`, `ci_smoke`, ...).
    pub sampling_profile: Cow<'static, str>,
    /// Sampling profile ref (into `artifacts/benchmarks/trace_sampling_policy.yaml`).
    pub sampling_profile_ref: String,
    /// Retention class (`hot_path_volatile`, ...).
    pub retention_class: Cow<'static, str>,
    /// Export posture (`excluded_by_default`, ...).
    pub export_posture: Cow<'static, str>,
    /// Redaction class (`metadata_safe_default`, ...).
    pub redaction_class: Cow<'static, str>,
}

impl HotPathMetricsContext {
    /// Default context intended for developer-local captures.
    pub fn developer_local(
        trace_id: String,
        host_os: Cow<'static, str>,
        build: BuildIdentityRecord,
    ) -> Self {
        Self {
            trace_id,
            backend: Cow::Borrowed("native_window"),
            host_os,
            build,
            exact_build_identity_ref: None,
            hardware_definition_ref: None,
            environment_ref: None,
            fixture_ref: None,
            corpus_manifest: None,
            sampling_profile: Cow::Borrowed("developer_local"),
            sampling_profile_ref: "profile.trace_sampling.developer_local".to_string(),
            retention_class: Cow::Borrowed("hot_path_volatile"),
            export_posture: Cow::Borrowed("excluded_by_default"),
            redaction_class: Cow::Borrowed("metadata_safe_default"),
        }
    }
}

/// Counter block for hot-path milestones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HotPathCounters {
    pub process_start_marks: u64,
    pub editor_surface_ready_marks: u64,
    pub first_interactive_shell_marks: u64,
    pub first_shell_frame_submitted_marks: u64,
    pub file_open_to_paint_spans: u64,
    pub file_switch_to_paint_spans: u64,
    pub keystroke_to_paint_spans: u64,
    pub scroll_to_paint_spans: u64,
    pub spans_errored: u64,
}

/// One emitted metrics capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HotPathMetricsRecord {
    pub schema: Cow<'static, str>,
    pub schema_version: u32,
    pub record_kind: Cow<'static, str>,
    pub trace_id: String,
    pub backend: Cow<'static, str>,
    pub host_os: Cow<'static, str>,
    pub build: BuildIdentityRecord,
    pub exact_build_identity_ref: Option<String>,
    pub hardware_definition_ref: Option<String>,
    pub environment_ref: Option<String>,
    pub fixture_ref: Option<String>,
    pub corpus_manifest: Option<CorpusManifestRefRecord>,
    pub sampling_profile: Cow<'static, str>,
    pub sampling_profile_ref: String,
    pub retention_class: Cow<'static, str>,
    pub export_posture: Cow<'static, str>,
    pub redaction_class: Cow<'static, str>,
    pub counters: HotPathCounters,
    pub events: Vec<TraceEventRecord>,
}

impl HotPathMetricsRecord {
    /// Validates that the record contains the minimum required hot-path events.
    ///
    /// This is intended for deterministic fixture validation and for failure
    /// drills that ensure malformed traces are rejected by guardrails.
    pub fn validate_minimum_required(&self) -> Result<(), HotPathMetricsValidationError> {
        let mut missing: Vec<&'static str> = Vec::new();

        if self
            .events
            .iter()
            .all(|ev| ev.journey_segment_id.as_ref() != "seg.startup.ui_dispatch.process_start")
        {
            missing.push("seg.startup.ui_dispatch.process_start");
        }
        if self
            .events
            .iter()
            .all(|ev| ev.journey_segment_id.as_ref() != "seg.startup.ui_dispatch.boot")
        {
            missing.push("seg.startup.ui_dispatch.boot");
        }
        if self.events.iter().all(|ev| {
            ev.journey_segment_id.as_ref() != "seg.startup.ui_dispatch.first_useful_chrome_ready"
        }) {
            missing.push("seg.startup.ui_dispatch.first_useful_chrome_ready");
        }
        if self
            .events
            .iter()
            .all(|ev| ev.journey_segment_id.as_ref() != "seg.first_paint.renderer_work.submit")
        {
            missing.push("seg.first_paint.renderer_work.submit");
        }
        if self.events.iter().all(|ev| {
            ev.journey_segment_id.as_ref() != "seg.quick_open.ui_dispatch.file_open_to_paint"
        }) {
            missing.push("seg.quick_open.ui_dispatch.file_open_to_paint");
        }
        if self.events.iter().all(|ev| {
            ev.journey_segment_id.as_ref() != "seg.input_to_paint.ui_dispatch.keystroke_to_paint"
        }) {
            missing.push("seg.input_to_paint.ui_dispatch.keystroke_to_paint");
        }
        if self.events.iter().all(|ev| {
            ev.journey_segment_id.as_ref() != "seg.input_to_paint.ui_dispatch.scroll_to_paint"
        }) {
            missing.push("seg.input_to_paint.ui_dispatch.scroll_to_paint");
        }

        if missing.is_empty() {
            Ok(())
        } else {
            Err(HotPathMetricsValidationError { missing })
        }
    }
}

/// Validation error describing which minimum-required milestones were missing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HotPathMetricsValidationError {
    pub missing: Vec<&'static str>,
}

impl std::fmt::Display for HotPathMetricsValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "missing required journey_segment_id entries: {:?}",
            self.missing
        )
    }
}

impl std::error::Error for HotPathMetricsValidationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MarkKind {
    EditorSurfaceReady,
    FirstInteractiveShell,
    FirstShellFrameSubmitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpanKind {
    FileOpen,
    FileSwitch,
    Keystroke,
    Scroll,
}

#[derive(Debug, Clone)]
struct PendingSpan {
    kind: SpanKind,
    span_id: String,
    started_tick: u64,
    note: Option<String>,
}

/// In-memory hot-path collector.
#[derive(Debug)]
pub struct HotPathMetrics {
    config: HotPathMetricsConfig,
    context: HotPathMetricsContext,
    next_seq: u32,
    counters: HotPathCounters,
    events: Vec<TraceEventRecord>,
    pending_spans: Vec<PendingSpan>,
    first_frame_marked: bool,
    root_span_id: String,
}

impl HotPathMetrics {
    /// Creates a new collector instance.
    pub fn new(config: HotPathMetricsConfig, context: HotPathMetricsContext) -> Self {
        let root_span_id = "span.shell.hot_path.root".to_string();
        let mut metrics = Self {
            config,
            context,
            next_seq: 0,
            counters: HotPathCounters::default(),
            events: Vec::new(),
            pending_spans: Vec::new(),
            first_frame_marked: false,
            root_span_id,
        };
        metrics.mark_process_start(0);
        metrics
    }

    /// Returns true when detailed trace capture is enabled.
    pub fn capture_enabled(&self) -> bool {
        self.config.output_path.is_some()
    }

    /// Returns the current counter block.
    pub fn counters(&self) -> HotPathCounters {
        self.counters
    }

    /// Marks the editor surface readiness milestone.
    pub fn mark_editor_surface_ready(&mut self, tick: u64) {
        self.mark(MarkKind::EditorSurfaceReady, tick);
    }

    /// Marks the first interactive shell milestone.
    pub fn mark_first_interactive_shell(&mut self, tick: u64) {
        self.mark(MarkKind::FirstInteractiveShell, tick);
    }

    /// Marks first shell frame submission once.
    pub fn mark_first_shell_frame_submitted(&mut self, tick: u64) {
        if self.first_frame_marked {
            return;
        }
        self.first_frame_marked = true;
        self.mark(MarkKind::FirstShellFrameSubmitted, tick);
    }

    /// Records a file-open request that will be closed on the next frame submit.
    pub fn note_file_open_to_paint_requested(&mut self, tick: u64) {
        self.note_span_start(SpanKind::FileOpen, tick, None);
    }

    /// Records a file switch request that will be closed on the next frame submit.
    pub fn note_file_switch_to_paint_requested(&mut self, tick: u64) {
        self.note_span_start(SpanKind::FileSwitch, tick, None);
    }

    /// Records a keystroke-admitted boundary that will be closed on the next frame submit.
    pub fn note_keystroke_to_paint_admitted(&mut self, tick: u64) {
        self.note_span_start(SpanKind::Keystroke, tick, None);
    }

    /// Records a scroll-admitted boundary that will be closed on the next frame submit.
    pub fn note_scroll_to_paint_admitted(&mut self, tick: u64) {
        self.note_span_start(SpanKind::Scroll, tick, None);
    }

    /// Records that a pending span failed and should be closed immediately.
    pub fn close_latest_span_as_error(&mut self, tick: u64, note: impl Into<String>) {
        let Some(pending) = self.pending_spans.pop() else {
            return;
        };
        self.counters.spans_errored = self.counters.spans_errored.saturating_add(1);
        if self.capture_enabled() {
            self.emit_span_end(
                &pending,
                tick,
                Cow::Borrowed("errored_caught"),
                Some(note.into()),
            );
        }
    }

    /// Closes any pending spans at the frame-submit boundary.
    pub fn note_frame_submitted(&mut self, tick: u64) {
        if self.pending_spans.is_empty() {
            return;
        }
        if !self.capture_enabled() {
            self.pending_spans.clear();
            return;
        }
        let spans = std::mem::take(&mut self.pending_spans);
        for pending in spans {
            let note = pending.note.clone();
            self.emit_span_end(&pending, tick, Cow::Borrowed("completed"), note);
        }
    }

    /// Exports the collected record.
    pub fn record(&self) -> HotPathMetricsRecord {
        HotPathMetricsRecord {
            schema: Cow::Borrowed(SCHEMA),
            schema_version: SCHEMA_VERSION,
            record_kind: Cow::Borrowed(RECORD_KIND),
            trace_id: self.context.trace_id.clone(),
            backend: self.context.backend.clone(),
            host_os: self.context.host_os.clone(),
            build: self.context.build.clone(),
            exact_build_identity_ref: self.context.exact_build_identity_ref.clone(),
            hardware_definition_ref: self.context.hardware_definition_ref.clone(),
            environment_ref: self.context.environment_ref.clone(),
            fixture_ref: self.context.fixture_ref.clone(),
            corpus_manifest: self.context.corpus_manifest.clone(),
            sampling_profile: self.context.sampling_profile.clone(),
            sampling_profile_ref: self.context.sampling_profile_ref.clone(),
            retention_class: self.context.retention_class.clone(),
            export_posture: self.context.export_posture.clone(),
            redaction_class: self.context.redaction_class.clone(),
            counters: self.counters,
            events: self.events.clone(),
        }
    }

    /// Writes the collected record to disk if an output path is configured.
    pub fn write_if_configured(&self) -> io::Result<()> {
        let Some(path) = self.config.output_path.as_deref() else {
            return Ok(());
        };
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.record())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
        fs::write(path, format!("{json}\n"))
    }

    fn mark_process_start(&mut self, tick: u64) {
        self.counters.process_start_marks = self.counters.process_start_marks.saturating_add(1);
        if !self.capture_enabled() {
            return;
        }
        let seq = self.bump_seq();
        self.events.push(TraceEventRecord {
            schema: Cow::Borrowed(TraceEventRecord::SCHEMA),
            schema_version: TraceEventRecord::SCHEMA_VERSION,
            record_kind: Cow::Borrowed(TraceEventRecord::RECORD_KIND),
            event_id: format!("evt.shell.hot_path.{seq}"),
            trace_id: self.context.trace_id.clone(),
            span_id: self.root_span_id.clone(),
            parent_span_id: None,
            span_kind: Cow::Borrowed("trace_root"),
            event_class: Cow::Borrowed("startup"),
            protected_journey: Cow::Borrowed("startup"),
            dispatch_layer: Cow::Borrowed("ui_dispatch"),
            journey_segment_id: Cow::Borrowed("seg.startup.ui_dispatch.process_start"),
            budget_ref: Cow::Borrowed("path.shell.launch"),
            attempt_class: Cow::Borrowed("first_attempt"),
            outcome_class: Cow::Borrowed("completed"),
            degraded_posture: Cow::Borrowed("healthy"),
            fallback_posture: Cow::Borrowed("none"),
            backend: self.context.backend.clone(),
            host_os: self.context.host_os.clone(),
            build: self.context.build.clone(),
            exact_build_identity_ref: self.context.exact_build_identity_ref.clone(),
            hardware_definition_ref: self.context.hardware_definition_ref.clone(),
            environment_ref: self.context.environment_ref.clone(),
            fixture_ref: self.context.fixture_ref.clone(),
            corpus_manifest: self.context.corpus_manifest.clone(),
            sampling_profile: self.context.sampling_profile.clone(),
            sampling_profile_ref: self.context.sampling_profile_ref.clone(),
            retention_class: self.context.retention_class.clone(),
            export_posture: self.context.export_posture.clone(),
            redaction_class: self.context.redaction_class.clone(),
            started_tick: tick,
            finished_tick: None,
            duration_ticks: None,
            linked_spike_trace_refs: Vec::new(),
            linked_journey_trace_refs: Vec::new(),
            evidence_refs: Vec::new(),
            requirement_refs: Vec::new(),
            note: Some("shell.process_start".to_string()),
        });
    }

    fn mark(&mut self, kind: MarkKind, tick: u64) {
        match kind {
            MarkKind::EditorSurfaceReady => {
                self.counters.editor_surface_ready_marks =
                    self.counters.editor_surface_ready_marks.saturating_add(1)
            }
            MarkKind::FirstInteractiveShell => {
                self.counters.first_interactive_shell_marks = self
                    .counters
                    .first_interactive_shell_marks
                    .saturating_add(1)
            }
            MarkKind::FirstShellFrameSubmitted => {
                self.counters.first_shell_frame_submitted_marks = self
                    .counters
                    .first_shell_frame_submitted_marks
                    .saturating_add(1)
            }
        }

        if !self.capture_enabled() {
            return;
        }

        let (event_class, protected_journey, dispatch_layer, journey_segment_id, budget_ref, note) =
            match kind {
                MarkKind::EditorSurfaceReady => (
                    "startup",
                    "startup",
                    "ui_dispatch",
                    "seg.startup.ui_dispatch.boot",
                    "path.shell.launch",
                    "shell.editor_surface_ready",
                ),
                MarkKind::FirstInteractiveShell => (
                    "startup",
                    "first_useful_chrome",
                    "ui_dispatch",
                    "seg.startup.ui_dispatch.first_useful_chrome_ready",
                    "path.shell.first_useful_chrome",
                    "shell.first_interactive_shell",
                ),
                MarkKind::FirstShellFrameSubmitted => (
                    "first_paint",
                    "render_submission",
                    "renderer_work",
                    "seg.first_paint.renderer_work.submit",
                    "path.shell.launch",
                    "shell.first_frame_submit",
                ),
            };

        let seq = self.bump_seq();
        self.events.push(TraceEventRecord {
            schema: Cow::Borrowed(TraceEventRecord::SCHEMA),
            schema_version: TraceEventRecord::SCHEMA_VERSION,
            record_kind: Cow::Borrowed(TraceEventRecord::RECORD_KIND),
            event_id: format!("evt.shell.hot_path.{seq}"),
            trace_id: self.context.trace_id.clone(),
            span_id: format!("span.shell.hot_path.{seq}"),
            parent_span_id: Some(self.root_span_id.clone()),
            span_kind: Cow::Borrowed("point_event"),
            event_class: Cow::Borrowed(event_class),
            protected_journey: Cow::Borrowed(protected_journey),
            dispatch_layer: Cow::Borrowed(dispatch_layer),
            journey_segment_id: Cow::Borrowed(journey_segment_id),
            budget_ref: Cow::Borrowed(budget_ref),
            attempt_class: Cow::Borrowed("first_attempt"),
            outcome_class: Cow::Borrowed("completed"),
            degraded_posture: Cow::Borrowed("healthy"),
            fallback_posture: Cow::Borrowed("none"),
            backend: self.context.backend.clone(),
            host_os: self.context.host_os.clone(),
            build: self.context.build.clone(),
            exact_build_identity_ref: self.context.exact_build_identity_ref.clone(),
            hardware_definition_ref: self.context.hardware_definition_ref.clone(),
            environment_ref: self.context.environment_ref.clone(),
            fixture_ref: self.context.fixture_ref.clone(),
            corpus_manifest: self.context.corpus_manifest.clone(),
            sampling_profile: self.context.sampling_profile.clone(),
            sampling_profile_ref: self.context.sampling_profile_ref.clone(),
            retention_class: self.context.retention_class.clone(),
            export_posture: self.context.export_posture.clone(),
            redaction_class: self.context.redaction_class.clone(),
            started_tick: tick,
            finished_tick: None,
            duration_ticks: None,
            linked_spike_trace_refs: Vec::new(),
            linked_journey_trace_refs: Vec::new(),
            evidence_refs: Vec::new(),
            requirement_refs: Vec::new(),
            note: Some(note.to_string()),
        });
    }

    fn note_span_start(&mut self, kind: SpanKind, tick: u64, note: Option<String>) {
        match kind {
            SpanKind::FileOpen => {
                self.counters.file_open_to_paint_spans =
                    self.counters.file_open_to_paint_spans.saturating_add(1);
            }
            SpanKind::FileSwitch => {
                self.counters.file_switch_to_paint_spans =
                    self.counters.file_switch_to_paint_spans.saturating_add(1);
            }
            SpanKind::Keystroke => {
                self.counters.keystroke_to_paint_spans =
                    self.counters.keystroke_to_paint_spans.saturating_add(1);
            }
            SpanKind::Scroll => {
                self.counters.scroll_to_paint_spans =
                    self.counters.scroll_to_paint_spans.saturating_add(1);
            }
        }

        if !self.capture_enabled() {
            return;
        }

        let (
            event_class,
            protected_journey,
            dispatch_layer,
            journey_segment_id,
            budget_ref,
            metric,
        ) = match kind {
            SpanKind::FileOpen => (
                "quick_open",
                "placeholder_open",
                "ui_dispatch",
                "seg.quick_open.ui_dispatch.file_open_to_paint",
                "path.editor.placeholder_open",
                "file_open_to_paint",
            ),
            SpanKind::FileSwitch => (
                "quick_open",
                "placeholder_open",
                "ui_dispatch",
                "seg.quick_open.ui_dispatch.file_switch_to_paint",
                "path.editor.placeholder_open",
                "file_switch_to_paint",
            ),
            SpanKind::Keystroke => (
                "input_to_paint",
                "input_to_paint",
                "ui_dispatch",
                "seg.input_to_paint.ui_dispatch.keystroke_to_paint",
                "path.editor.first_useful_edit",
                "keystroke_to_paint",
            ),
            SpanKind::Scroll => (
                "input_to_paint",
                "input_to_paint",
                "ui_dispatch",
                "seg.input_to_paint.ui_dispatch.scroll_to_paint",
                "path.editor.first_useful_edit",
                "scroll_to_paint",
            ),
        };

        let seq = self.bump_seq();
        let span_id = format!("span.shell.hot_path.{metric}.{seq}");
        self.events.push(TraceEventRecord {
            schema: Cow::Borrowed(TraceEventRecord::SCHEMA),
            schema_version: TraceEventRecord::SCHEMA_VERSION,
            record_kind: Cow::Borrowed(TraceEventRecord::RECORD_KIND),
            event_id: format!("evt.shell.hot_path.{seq}"),
            trace_id: self.context.trace_id.clone(),
            span_id: span_id.clone(),
            parent_span_id: Some(self.root_span_id.clone()),
            span_kind: Cow::Borrowed("span_start"),
            event_class: Cow::Borrowed(event_class),
            protected_journey: Cow::Borrowed(protected_journey),
            dispatch_layer: Cow::Borrowed(dispatch_layer),
            journey_segment_id: Cow::Borrowed(journey_segment_id),
            budget_ref: Cow::Borrowed(budget_ref),
            attempt_class: Cow::Borrowed("first_attempt"),
            outcome_class: Cow::Borrowed("pending_open_span"),
            degraded_posture: Cow::Borrowed("healthy"),
            fallback_posture: Cow::Borrowed("none"),
            backend: self.context.backend.clone(),
            host_os: self.context.host_os.clone(),
            build: self.context.build.clone(),
            exact_build_identity_ref: self.context.exact_build_identity_ref.clone(),
            hardware_definition_ref: self.context.hardware_definition_ref.clone(),
            environment_ref: self.context.environment_ref.clone(),
            fixture_ref: self.context.fixture_ref.clone(),
            corpus_manifest: self.context.corpus_manifest.clone(),
            sampling_profile: self.context.sampling_profile.clone(),
            sampling_profile_ref: self.context.sampling_profile_ref.clone(),
            retention_class: self.context.retention_class.clone(),
            export_posture: self.context.export_posture.clone(),
            redaction_class: self.context.redaction_class.clone(),
            started_tick: tick,
            finished_tick: None,
            duration_ticks: None,
            linked_spike_trace_refs: Vec::new(),
            linked_journey_trace_refs: Vec::new(),
            evidence_refs: Vec::new(),
            requirement_refs: Vec::new(),
            note: note.clone(),
        });

        self.pending_spans.push(PendingSpan {
            kind,
            span_id,
            started_tick: tick,
            note,
        });
    }

    fn emit_span_end(
        &mut self,
        pending: &PendingSpan,
        finished_tick: u64,
        outcome_class: Cow<'static, str>,
        note: Option<String>,
    ) {
        let (event_class, protected_journey, dispatch_layer, journey_segment_id, budget_ref) =
            match pending.kind {
                SpanKind::FileOpen => (
                    "quick_open",
                    "placeholder_open",
                    "ui_dispatch",
                    "seg.quick_open.ui_dispatch.file_open_to_paint",
                    "path.editor.placeholder_open",
                ),
                SpanKind::FileSwitch => (
                    "quick_open",
                    "placeholder_open",
                    "ui_dispatch",
                    "seg.quick_open.ui_dispatch.file_switch_to_paint",
                    "path.editor.placeholder_open",
                ),
                SpanKind::Keystroke => (
                    "input_to_paint",
                    "input_to_paint",
                    "ui_dispatch",
                    "seg.input_to_paint.ui_dispatch.keystroke_to_paint",
                    "path.editor.first_useful_edit",
                ),
                SpanKind::Scroll => (
                    "input_to_paint",
                    "input_to_paint",
                    "ui_dispatch",
                    "seg.input_to_paint.ui_dispatch.scroll_to_paint",
                    "path.editor.first_useful_edit",
                ),
            };

        let seq = self.bump_seq();
        let duration = finished_tick.saturating_sub(pending.started_tick);
        self.events.push(TraceEventRecord {
            schema: Cow::Borrowed(TraceEventRecord::SCHEMA),
            schema_version: TraceEventRecord::SCHEMA_VERSION,
            record_kind: Cow::Borrowed(TraceEventRecord::RECORD_KIND),
            event_id: format!("evt.shell.hot_path.{seq}"),
            trace_id: self.context.trace_id.clone(),
            span_id: pending.span_id.clone(),
            parent_span_id: Some(self.root_span_id.clone()),
            span_kind: Cow::Borrowed("span_end"),
            event_class: Cow::Borrowed(event_class),
            protected_journey: Cow::Borrowed(protected_journey),
            dispatch_layer: Cow::Borrowed(dispatch_layer),
            journey_segment_id: Cow::Borrowed(journey_segment_id),
            budget_ref: Cow::Borrowed(budget_ref),
            attempt_class: Cow::Borrowed("first_attempt"),
            outcome_class,
            degraded_posture: Cow::Borrowed("healthy"),
            fallback_posture: Cow::Borrowed("none"),
            backend: self.context.backend.clone(),
            host_os: self.context.host_os.clone(),
            build: self.context.build.clone(),
            exact_build_identity_ref: self.context.exact_build_identity_ref.clone(),
            hardware_definition_ref: self.context.hardware_definition_ref.clone(),
            environment_ref: self.context.environment_ref.clone(),
            fixture_ref: self.context.fixture_ref.clone(),
            corpus_manifest: self.context.corpus_manifest.clone(),
            sampling_profile: self.context.sampling_profile.clone(),
            sampling_profile_ref: self.context.sampling_profile_ref.clone(),
            retention_class: self.context.retention_class.clone(),
            export_posture: self.context.export_posture.clone(),
            redaction_class: self.context.redaction_class.clone(),
            started_tick: pending.started_tick,
            finished_tick: Some(finished_tick),
            duration_ticks: Some(duration),
            linked_spike_trace_refs: Vec::new(),
            linked_journey_trace_refs: Vec::new(),
            evidence_refs: Vec::new(),
            requirement_refs: Vec::new(),
            note,
        });
    }

    fn bump_seq(&mut self) -> u32 {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.saturating_add(1);
        seq
    }
}
