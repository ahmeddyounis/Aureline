//! Beta finalize layer for the canonical task-event model.
//!
//! This module declares the closed set of beta task-event lanes (run, test,
//! debug, review, AI, and support-export) and the wedges, event kinds, and
//! consumer surfaces each lane is allowed to use. The canonical type
//! definitions live in [`crate::tasks`]; this module pins them at the beta
//! finalize boundary so shell rows, activity-center rows, support exports,
//! review consumers, and AI consumers can read the same event grammar
//! without forking their own parsers.
//!
//! The machine-readable boundary lives at
//! [`/schemas/runtime/task_event.schema.json`](../../../schemas/runtime/task_event.schema.json)
//! and the reviewer-facing companion doc at
//! [`/docs/runtime/m3/task_event_model_beta.md`](../../../docs/runtime/m3/task_event_model_beta.md).

use serde::{Deserialize, Serialize};

use crate::tasks::{
    TaskConsumerSurfaceClass, TaskEvent, TaskEventKind, TaskEventStream, TaskWedgeClass,
    TASK_EVENT_SCHEMA_VERSION,
};

/// Stable record-kind tag for the beta lane coverage manifest.
pub const TASK_EVENT_BETA_COVERAGE_MANIFEST_RECORD_KIND: &str =
    "task_event_beta_coverage_manifest_record";

/// Beta task-event lane declared by the canonical task-event model.
///
/// Lanes group wedges and consumer surfaces into the closed set the beta
/// program ships with. Adding a lane is a vocabulary change that must update
/// the canonical schema and the reviewer doc together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskEventBetaLane {
    /// Run lane: build, terminal-backed task, package, notebook, and generic
    /// task runs.
    Run,
    /// Test lane: unit, integration, watch, and imported-CI test attempts.
    Test,
    /// Debug lane: debug launch, attach, and reconnect attempts.
    Debug,
    /// Review lane: review-wedge validation, comparison, and audit work.
    Review,
    /// AI lane: AI-tool validation and tool-execution work.
    Ai,
    /// Support-export lane: retained event stream replayed in support and
    /// activity-center exports.
    SupportExport,
}

impl TaskEventBetaLane {
    /// All beta task-event lanes declared by the canonical model.
    pub const ALL: [Self; 6] = [
        Self::Run,
        Self::Test,
        Self::Debug,
        Self::Review,
        Self::Ai,
        Self::SupportExport,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Test => "test",
            Self::Debug => "debug",
            Self::Review => "review",
            Self::Ai => "ai",
            Self::SupportExport => "support_export",
        }
    }

    /// Wedge classes that participate in this lane.
    pub fn wedges(self) -> Vec<TaskWedgeClass> {
        match self {
            Self::Run => vec![
                TaskWedgeClass::Build,
                TaskWedgeClass::Terminal,
                TaskWedgeClass::Package,
                TaskWedgeClass::Notebook,
                TaskWedgeClass::Generic,
            ],
            Self::Test => vec![TaskWedgeClass::Test],
            Self::Debug => vec![TaskWedgeClass::Debug],
            Self::Review => vec![TaskWedgeClass::Review],
            Self::Ai => vec![TaskWedgeClass::AiTool],
            Self::SupportExport => vec![
                TaskWedgeClass::Build,
                TaskWedgeClass::Test,
                TaskWedgeClass::Debug,
                TaskWedgeClass::Review,
                TaskWedgeClass::AiTool,
                TaskWedgeClass::Terminal,
                TaskWedgeClass::Package,
                TaskWedgeClass::Notebook,
                TaskWedgeClass::Generic,
            ],
        }
    }

    /// Event kinds the lane is allowed to emit.
    ///
    /// Every lane shares the canonical lifecycle, progress, output,
    /// diagnostic, artifact, input-request, blocked, and degraded event
    /// kinds so consumers can be written once.
    pub fn claimed_event_kinds(self) -> Vec<TaskEventKind> {
        canonical_event_kinds()
    }

    /// Consumer surfaces this lane projects into.
    pub fn consumer_surfaces(self) -> Vec<TaskConsumerSurfaceClass> {
        match self {
            Self::SupportExport => vec![TaskConsumerSurfaceClass::SupportBundleExport],
            _ => vec![
                TaskConsumerSurfaceClass::Shell,
                TaskConsumerSurfaceClass::ActivityCenter,
                TaskConsumerSurfaceClass::SupportBundleExport,
            ],
        }
    }
}

fn canonical_event_kinds() -> Vec<TaskEventKind> {
    vec![
        TaskEventKind::TaskQueued,
        TaskEventKind::TaskStarted,
        TaskEventKind::TaskBlocked,
        TaskEventKind::InputRequested,
        TaskEventKind::ProgressUpdated,
        TaskEventKind::OutputAppended,
        TaskEventKind::DiagnosticEmitted,
        TaskEventKind::ArtifactPublished,
        TaskEventKind::DegradedStateReported,
        TaskEventKind::TaskCompleted,
        TaskEventKind::TaskFailed,
        TaskEventKind::TaskCancelled,
    ]
}

/// One row of beta lane coverage.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskEventBetaLaneCoverageRow {
    /// Beta lane.
    pub lane: TaskEventBetaLane,
    /// Stable lane token.
    pub lane_token: String,
    /// Wedge classes that participate in this lane.
    pub wedges: Vec<TaskWedgeClass>,
    /// Stable wedge tokens.
    pub wedge_tokens: Vec<String>,
    /// Event kinds the lane is allowed to emit.
    pub claimed_event_kinds: Vec<TaskEventKind>,
    /// Stable event-kind tokens.
    pub claimed_event_kind_tokens: Vec<String>,
    /// Consumer surfaces this lane projects into.
    pub consumer_surfaces: Vec<TaskConsumerSurfaceClass>,
}

impl TaskEventBetaLaneCoverageRow {
    /// Builds the canonical coverage row for one lane.
    pub fn canonical(lane: TaskEventBetaLane) -> Self {
        let wedges = lane.wedges();
        let wedge_tokens = wedges.iter().map(|w| w.as_str().to_owned()).collect();
        let claimed_event_kinds = lane.claimed_event_kinds();
        let claimed_event_kind_tokens = claimed_event_kinds
            .iter()
            .map(|kind| kind.as_str().to_owned())
            .collect();
        Self {
            lane,
            lane_token: lane.as_str().to_owned(),
            wedges,
            wedge_tokens,
            claimed_event_kinds,
            claimed_event_kind_tokens,
            consumer_surfaces: lane.consumer_surfaces(),
        }
    }
}

/// Coverage manifest pinning the canonical beta task-event lanes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskEventBetaCoverageManifest {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version shared with the canonical task-event model.
    pub task_event_schema_version: u32,
    /// Manifest id.
    pub manifest_id: String,
    /// Manifest timestamp.
    pub generated_at: String,
    /// Canonical lane coverage rows.
    pub lanes: Vec<TaskEventBetaLaneCoverageRow>,
}

impl TaskEventBetaCoverageManifest {
    /// Builds the canonical beta coverage manifest.
    pub fn canonical(manifest_id: impl Into<String>, generated_at: impl Into<String>) -> Self {
        Self {
            record_kind: TASK_EVENT_BETA_COVERAGE_MANIFEST_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            manifest_id: manifest_id.into(),
            generated_at: generated_at.into(),
            lanes: TaskEventBetaLane::ALL
                .into_iter()
                .map(TaskEventBetaLaneCoverageRow::canonical)
                .collect(),
        }
    }

    /// Returns the canonical row for one lane, if present.
    pub fn row_for_lane(&self, lane: TaskEventBetaLane) -> Option<&TaskEventBetaLaneCoverageRow> {
        self.lanes.iter().find(|row| row.lane == lane)
    }

    /// Verifies that the supplied event stream uses only the canonical event
    /// grammar declared by this manifest. Returns the list of wedges that
    /// appeared in the stream but are not covered by any lane.
    pub fn unclaimed_wedges(&self, stream: &TaskEventStream) -> Vec<TaskWedgeClass> {
        let mut unclaimed: Vec<TaskWedgeClass> = Vec::new();
        for event in &stream.events {
            let wedge = event.identity.wedge;
            if !self.lanes.iter().any(|row| row.wedges.contains(&wedge))
                && !unclaimed.contains(&wedge)
            {
                unclaimed.push(wedge);
            }
        }
        unclaimed
    }
}

/// Resolves the beta lane that owns a typed task event.
///
/// Test, debug, review, and AI wedges map to their dedicated lanes; the
/// remaining wedges map to the run lane. Support-export coverage is a
/// consumer-surface lane and is not assigned per event here.
pub fn lane_for_event(event: &TaskEvent) -> TaskEventBetaLane {
    lane_for_wedge(event.identity.wedge)
}

/// Resolves the beta lane that owns a wedge class.
pub fn lane_for_wedge(wedge: TaskWedgeClass) -> TaskEventBetaLane {
    match wedge {
        TaskWedgeClass::Test => TaskEventBetaLane::Test,
        TaskWedgeClass::Debug => TaskEventBetaLane::Debug,
        TaskWedgeClass::Review => TaskEventBetaLane::Review,
        TaskWedgeClass::AiTool => TaskEventBetaLane::Ai,
        TaskWedgeClass::Build
        | TaskWedgeClass::Terminal
        | TaskWedgeClass::Package
        | TaskWedgeClass::Notebook
        | TaskWedgeClass::Generic => TaskEventBetaLane::Run,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tasks::{
        RawEnvelopeRetentionState, RawTaskEventEnvelope, TaskEventConfidence, TaskEventIdentity,
        TaskEventKind, TaskEventPayload, TaskEventProvenance, TaskEventRedactionClass,
        TaskEventSourceKind, TaskEventStream, TaskStateClass, TaskWedgeClass,
        RAW_TASK_EVENT_ENVELOPE_RECORD_KIND, TASK_EVENT_RECORD_KIND,
    };

    fn event(seq: u64, wedge: TaskWedgeClass) -> TaskEvent {
        let identity = TaskEventIdentity {
            task_id: format!("task:{}:{seq}", wedge.as_str()),
            run_id: format!("run:{}:{seq}", wedge.as_str()),
            attempt_id: format!("attempt:{}:{seq}", wedge.as_str()),
            workspace_id: "workspace:beta".to_owned(),
            trace_id: "trace:beta".to_owned(),
            execution_context_id: "exec:beta".to_owned(),
            target_id: format!("target:{}:{seq}", wedge.as_str()),
            wedge,
        };
        let event_id = format!("event:{seq}");
        TaskEvent {
            record_kind: TASK_EVENT_RECORD_KIND.to_owned(),
            task_event_schema_version: TASK_EVENT_SCHEMA_VERSION,
            event_id: event_id.clone(),
            stream_id: "stream:beta".to_owned(),
            stream_sequence: seq,
            identity: identity.clone(),
            event_kind: TaskEventKind::TaskQueued,
            state_after: TaskStateClass::Queued,
            occurred_at: format!("2026-05-15T00:00:{seq:02}Z"),
            summary: format!("seed {seq}"),
            payload: TaskEventPayload::Lifecycle {
                lifecycle_reason: Some("seed".to_owned()),
                exit_status: None,
            },
            provenance: TaskEventProvenance {
                source_kind: TaskEventSourceKind::Native,
                source_adapter_id: "adapter:beta".to_owned(),
                adapter_version: "1.0.0".to_owned(),
                workspace_revision: None,
                confidence: TaskEventConfidence::High,
                context_provenance: None,
            },
            raw_envelope: RawTaskEventEnvelope {
                record_kind: RAW_TASK_EVENT_ENVELOPE_RECORD_KIND.to_owned(),
                raw_envelope_ref: format!("raw:{event_id}"),
                task_id: identity.task_id.clone(),
                workspace_id: identity.workspace_id.clone(),
                trace_id: identity.trace_id.clone(),
                source_kind: TaskEventSourceKind::Native,
                adapter_origin_event_id: format!("adapter:{event_id}"),
                redaction_class: TaskEventRedactionClass::MetadataSafeDefault,
                retention_state: RawEnvelopeRetentionState::RetainedInlineRedacted,
                payload_digest: format!("sha256:{seq:064x}"),
                retained_payload: None,
                retained_at: format!("2026-05-15T00:00:{seq:02}Z"),
                reconstruction_fields: vec!["seed".to_owned()],
            },
        }
    }

    #[test]
    fn canonical_manifest_covers_all_lanes_and_wedges() {
        let manifest = TaskEventBetaCoverageManifest::canonical(
            "task-event-beta:canonical",
            "2026-05-15T00:00:00Z",
        );
        assert_eq!(manifest.lanes.len(), TaskEventBetaLane::ALL.len());

        let mut all_wedges: Vec<TaskWedgeClass> = Vec::new();
        for row in &manifest.lanes {
            for wedge in &row.wedges {
                if !all_wedges.contains(wedge) {
                    all_wedges.push(*wedge);
                }
            }
        }
        for wedge in [
            TaskWedgeClass::Build,
            TaskWedgeClass::Test,
            TaskWedgeClass::Debug,
            TaskWedgeClass::Review,
            TaskWedgeClass::AiTool,
            TaskWedgeClass::Terminal,
            TaskWedgeClass::Package,
            TaskWedgeClass::Notebook,
            TaskWedgeClass::Generic,
        ] {
            assert!(all_wedges.contains(&wedge), "{wedge:?} must be covered");
        }
    }

    #[test]
    fn lane_for_wedge_routes_review_and_ai_to_dedicated_lanes() {
        assert_eq!(
            lane_for_wedge(TaskWedgeClass::Review),
            TaskEventBetaLane::Review
        );
        assert_eq!(
            lane_for_wedge(TaskWedgeClass::AiTool),
            TaskEventBetaLane::Ai
        );
        assert_eq!(
            lane_for_wedge(TaskWedgeClass::Build),
            TaskEventBetaLane::Run
        );
        assert_eq!(
            lane_for_wedge(TaskWedgeClass::Terminal),
            TaskEventBetaLane::Run
        );
    }

    #[test]
    fn unclaimed_wedges_returns_empty_for_canonical_stream() {
        let mut stream = TaskEventStream::new("stream:beta", "workspace:beta", "trace:beta");
        for (idx, wedge) in [
            TaskWedgeClass::Build,
            TaskWedgeClass::Test,
            TaskWedgeClass::Debug,
            TaskWedgeClass::Review,
            TaskWedgeClass::AiTool,
        ]
        .into_iter()
        .enumerate()
        {
            let seq = idx as u64 + 1;
            stream.append(event(seq, wedge)).expect("append");
        }
        let manifest = TaskEventBetaCoverageManifest::canonical(
            "task-event-beta:test",
            "2026-05-15T00:00:00Z",
        );
        assert!(manifest.unclaimed_wedges(&stream).is_empty());
    }
}
