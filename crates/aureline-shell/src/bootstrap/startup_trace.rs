//! Startup milestone capture for the native desktop shell.
//!
//! The trace records emitted here intentionally align with:
//!
//! - `schemas/traces/trace_event.schema.json`
//! - `artifacts/benchmarks/journey_segment_ids.yaml`
//! - `artifacts/perf/protected_path_ledger.yaml`
//!
//! The capture is currently a small, self-contained JSON export used for
//! developer-local smoke evidence and deterministic tests.

use std::fs;
use std::io;
use std::path::Path;
use std::time::{Duration, Instant};

use aureline_build_info as build_info;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
struct BuildIdentityRecord {
    crate_name: String,
    crate_version: String,
    rustc_target_triple: String,
}

#[derive(Debug, Clone, Serialize)]
struct CorpusManifestRefRecord {
    manifest_id: String,
    manifest_revision: u32,
}

#[derive(Debug, Clone, Serialize)]
struct TraceEventRecord {
    schema: &'static str,
    schema_version: u32,
    record_kind: &'static str,
    event_id: String,
    trace_id: String,
    span_id: String,
    parent_span_id: Option<String>,
    span_kind: &'static str,
    event_class: &'static str,
    protected_journey: &'static str,
    dispatch_layer: &'static str,
    journey_segment_id: String,
    budget_ref: String,
    attempt_class: &'static str,
    outcome_class: &'static str,
    degraded_posture: &'static str,
    fallback_posture: &'static str,
    backend: &'static str,
    host_os: &'static str,
    build: BuildIdentityRecord,
    exact_build_identity_ref: Option<String>,
    hardware_definition_ref: Option<String>,
    environment_ref: Option<String>,
    fixture_ref: Option<String>,
    corpus_manifest: Option<CorpusManifestRefRecord>,
    sampling_profile: &'static str,
    sampling_profile_ref: String,
    retention_class: &'static str,
    export_posture: &'static str,
    redaction_class: &'static str,
    started_tick: u64,
    finished_tick: Option<u64>,
    duration_ticks: Option<u64>,
    linked_spike_trace_refs: Vec<String>,
    linked_journey_trace_refs: Vec<String>,
    evidence_refs: Vec<String>,
    requirement_refs: Vec<String>,
    note: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum StartupMilestone {
    FirstInteractiveShell,
    EditorSurfaceReady,
    FirstShellFrameSubmitted,
}

impl StartupMilestone {
    const fn event_class(self) -> &'static str {
        match self {
            Self::FirstInteractiveShell | Self::EditorSurfaceReady => "startup",
            Self::FirstShellFrameSubmitted => "first_paint",
        }
    }

    const fn protected_journey(self) -> &'static str {
        match self {
            Self::FirstInteractiveShell => "first_useful_chrome",
            Self::EditorSurfaceReady => "startup",
            Self::FirstShellFrameSubmitted => "render_submission",
        }
    }

    const fn dispatch_layer(self) -> &'static str {
        match self {
            Self::FirstInteractiveShell | Self::EditorSurfaceReady => "ui_dispatch",
            Self::FirstShellFrameSubmitted => "renderer_work",
        }
    }

    const fn journey_segment_id(self) -> &'static str {
        match self {
            Self::FirstInteractiveShell => "seg.startup.ui_dispatch.first_useful_chrome_ready",
            Self::EditorSurfaceReady => "seg.startup.ui_dispatch.boot",
            Self::FirstShellFrameSubmitted => "seg.first_paint.renderer_work.submit",
        }
    }

    const fn budget_ref(self) -> &'static str {
        match self {
            Self::FirstInteractiveShell => "path.shell.first_useful_chrome",
            Self::EditorSurfaceReady | Self::FirstShellFrameSubmitted => "path.shell.launch",
        }
    }

    const fn note(self) -> &'static str {
        match self {
            Self::FirstInteractiveShell => "shell.command_entry_ready",
            Self::EditorSurfaceReady => "shell.editor_surface_ready",
            Self::FirstShellFrameSubmitted => "shell.first_frame_submit",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct StartupTraceConfig {
    pub output_path: Option<String>,
    pub exit_after_first_frame: bool,
}

impl Default for StartupTraceConfig {
    fn default() -> Self {
        Self {
            output_path: None,
            exit_after_first_frame: false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct StartupTrace {
    config: StartupTraceConfig,
    start: Instant,
    trace_id: String,
    root_span_id: String,
    next_seq: u32,
    first_frame_emitted: bool,
    events: Vec<TraceEventRecord>,
    build: BuildIdentityRecord,
    exact_build_identity_ref: String,
    host_os: &'static str,
    backend: &'static str,
    sampling_profile_ref: String,
}

impl StartupTrace {
    pub(crate) fn new(config: StartupTraceConfig) -> Self {
        let identity = build_info::build_identity();
        let exact_build_identity_ref = build_info::exact_build_identity_ref();
        let trace_id = format!(
            "trace.shell.bootstrap:{}:{}",
            identity.commit_short, identity.build_timestamp_utc
        );

        let build = BuildIdentityRecord {
            crate_name: env!("CARGO_PKG_NAME").to_string(),
            crate_version: env!("CARGO_PKG_VERSION").to_string(),
            rustc_target_triple: identity.target_triple,
        };

        Self {
            config,
            start: Instant::now(),
            trace_id,
            root_span_id: "span.shell.bootstrap.root".to_string(),
            next_seq: 0,
            first_frame_emitted: false,
            events: Vec::new(),
            build,
            exact_build_identity_ref,
            host_os: host_os_class(),
            backend: "native_window",
            sampling_profile_ref: "profile.trace_sampling.developer_local".to_string(),
        }
    }

    pub(crate) fn config(&self) -> &StartupTraceConfig {
        &self.config
    }

    pub(crate) fn tick_now(&self) -> u64 {
        duration_to_ticks(self.start.elapsed())
    }

    pub(crate) fn mark(&mut self, milestone: StartupMilestone) {
        if milestone == StartupMilestone::FirstShellFrameSubmitted {
            if self.first_frame_emitted {
                return;
            }
            self.first_frame_emitted = true;
        }

        let seq = self.next_seq;
        self.next_seq = self.next_seq.saturating_add(1);

        let tick = self.tick_now();
        let event = TraceEventRecord {
            schema: "aureline.trace_event.v1",
            schema_version: 1,
            record_kind: "trace_event_record",
            event_id: format!("evt.shell.bootstrap.{seq}"),
            trace_id: self.trace_id.clone(),
            span_id: format!("span.shell.bootstrap.{seq}"),
            parent_span_id: Some(self.root_span_id.clone()),
            span_kind: "point_event",
            event_class: milestone.event_class(),
            protected_journey: milestone.protected_journey(),
            dispatch_layer: milestone.dispatch_layer(),
            journey_segment_id: milestone.journey_segment_id().to_string(),
            budget_ref: milestone.budget_ref().to_string(),
            attempt_class: "first_attempt",
            outcome_class: "completed",
            degraded_posture: "healthy",
            fallback_posture: "none",
            backend: self.backend,
            host_os: self.host_os,
            build: self.build.clone(),
            exact_build_identity_ref: Some(self.exact_build_identity_ref.clone()),
            hardware_definition_ref: None,
            environment_ref: None,
            fixture_ref: None,
            corpus_manifest: None,
            sampling_profile: "developer_local",
            sampling_profile_ref: self.sampling_profile_ref.clone(),
            retention_class: "hot_path_volatile",
            export_posture: "excluded_by_default",
            redaction_class: "metadata_safe_default",
            started_tick: tick,
            finished_tick: None,
            duration_ticks: None,
            linked_spike_trace_refs: Vec::new(),
            linked_journey_trace_refs: Vec::new(),
            evidence_refs: Vec::new(),
            requirement_refs: Vec::new(),
            note: Some(milestone.note().to_string()),
        };
        self.events.push(event);
    }

    pub(crate) fn first_frame_emitted(&self) -> bool {
        self.first_frame_emitted
    }

    pub(crate) fn write_if_configured(&self) -> io::Result<()> {
        let Some(path) = self.config.output_path.as_deref() else {
            return Ok(());
        };
        let path = Path::new(path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(&self.events)
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err.to_string()))?;
        fs::write(path, format!("{json}\n"))
    }
}

fn duration_to_ticks(duration: Duration) -> u64 {
    duration.as_nanos().min(u128::from(u64::MAX)) as u64
}

const fn host_os_class() -> &'static str {
    if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else {
        "unknown"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_frame_milestone_is_idempotent() {
        let mut trace = StartupTrace::new(StartupTraceConfig::default());
        trace.mark(StartupMilestone::FirstShellFrameSubmitted);
        trace.mark(StartupMilestone::FirstShellFrameSubmitted);
        assert!(trace.first_frame_emitted());
        assert_eq!(trace.events.len(), 1);
    }
}
