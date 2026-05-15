//! Telemetry endpoint policy rows for trace-event inspection.
//!
//! The rows in this module describe where trace-event families may be
//! projected, which event classes each endpoint accepts, and which redaction
//! class is applied before any serialized projection leaves the recorder.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::trace_event::TraceEventRecord;

/// Stable record-kind tag for trace endpoint event projections.
pub const ENDPOINT_EVENT_PROJECTION_RECORD_KIND: &str = "trace_endpoint_event_projection_record";

/// Closed trace-event class vocabulary from `schemas/traces/trace_event.schema.json`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceEventClass {
    /// Startup and shell launch events.
    Startup,
    /// First-paint render events.
    FirstPaint,
    /// Input-to-paint editor latency events.
    InputToPaint,
    /// Quick-open, file-open, and placeholder-open events.
    QuickOpen,
    /// Save-pipeline events.
    Save,
    /// Restore and recovery-replay events.
    Restore,
    /// Terminal or task rerun events.
    RerunTask,
    /// Remote-session or managed-call reconnect events.
    Reconnect,
    /// Structured tool-use events.
    ToolUse,
    /// Full AI broker roundtrip events.
    AiTurn,
    /// Fallback-resolution events.
    FallbackResolution,
    /// Observability-only events.
    Observability,
}

impl TraceEventClass {
    /// All event classes accepted by the trace-event schema.
    pub const ALL: [Self; 12] = [
        Self::Startup,
        Self::FirstPaint,
        Self::InputToPaint,
        Self::QuickOpen,
        Self::Save,
        Self::Restore,
        Self::RerunTask,
        Self::Reconnect,
        Self::ToolUse,
        Self::AiTurn,
        Self::FallbackResolution,
        Self::Observability,
    ];

    /// Returns the stable serialized token for this event class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Startup => "startup",
            Self::FirstPaint => "first_paint",
            Self::InputToPaint => "input_to_paint",
            Self::QuickOpen => "quick_open",
            Self::Save => "save",
            Self::Restore => "restore",
            Self::RerunTask => "rerun_task",
            Self::Reconnect => "reconnect",
            Self::ToolUse => "tool_use",
            Self::AiTurn => "ai_turn",
            Self::FallbackResolution => "fallback_resolution",
            Self::Observability => "observability",
        }
    }

    /// Parses a serialized trace-event class token.
    pub fn from_token(token: &str) -> Option<Self> {
        match token {
            "startup" => Some(Self::Startup),
            "first_paint" => Some(Self::FirstPaint),
            "input_to_paint" => Some(Self::InputToPaint),
            "quick_open" => Some(Self::QuickOpen),
            "save" => Some(Self::Save),
            "restore" => Some(Self::Restore),
            "rerun_task" => Some(Self::RerunTask),
            "reconnect" => Some(Self::Reconnect),
            "tool_use" => Some(Self::ToolUse),
            "ai_turn" => Some(Self::AiTurn),
            "fallback_resolution" => Some(Self::FallbackResolution),
            "observability" => Some(Self::Observability),
            _ => None,
        }
    }
}

/// Closed redaction vocabulary for trace-event endpoint projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceRedactionClass {
    /// Metadata-safe default redaction.
    MetadataSafeDefault,
    /// Operator-only support redaction.
    OperatorOnlyRestricted,
    /// Internal support restricted redaction.
    InternalSupportRestricted,
    /// Release-signing evidence-only redaction.
    SigningEvidenceOnly,
}

impl TraceRedactionClass {
    /// Returns the stable serialized token for this redaction class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafeDefault => "metadata_safe_default",
            Self::OperatorOnlyRestricted => "operator_only_restricted",
            Self::InternalSupportRestricted => "internal_support_restricted",
            Self::SigningEvidenceOnly => "signing_evidence_only",
        }
    }
}

/// Current opt-in or policy state for a telemetry endpoint.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum EndpointOptInState {
    /// The endpoint is disabled until explicit opt-in exists.
    #[default]
    OptedOut,
    /// The endpoint is enabled by explicit user opt-in.
    OptedIn,
    /// The endpoint is local-only and does not leave the device.
    LocalOnly,
    /// A managed policy enables the endpoint.
    ManagedPolicyEnabled,
    /// A managed policy disables the endpoint.
    DisabledByPolicy,
}

impl EndpointOptInState {
    /// Returns true when this state permits event projection to the endpoint.
    pub const fn permits_event_projection(self) -> bool {
        matches!(
            self,
            Self::OptedIn | Self::LocalOnly | Self::ManagedPolicyEnabled
        )
    }
}

/// Stable identity for a telemetry endpoint without exposing raw network details.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct EndpointIdentity {
    /// Opaque endpoint id shown by inspectors.
    pub endpoint_id: String,
    /// Review-safe endpoint label.
    pub endpoint_label: String,
    /// Route class such as local-only or optional telemetry upload.
    pub route_class: String,
}

impl EndpointIdentity {
    /// Creates a review-safe endpoint identity.
    pub fn new(
        endpoint_id: impl Into<String>,
        endpoint_label: impl Into<String>,
        route_class: impl Into<String>,
    ) -> Self {
        Self {
            endpoint_id: endpoint_id.into(),
            endpoint_label: endpoint_label.into(),
            route_class: route_class.into(),
        }
    }
}

/// Endpoint policy row for one trace-event destination.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointPolicyRow {
    /// Stable row id.
    pub endpoint_policy_row_id: String,
    /// Review-safe endpoint identity.
    pub endpoint_identity: EndpointIdentity,
    /// Redaction class applied to events before projection serialization.
    pub redaction_class: TraceRedactionClass,
    /// Event classes this endpoint may receive.
    pub scope_allowed_event_kinds: Vec<TraceEventClass>,
    /// Current opt-in or policy state for the endpoint.
    pub current_opt_in_state: EndpointOptInState,
}

impl EndpointPolicyRow {
    /// Creates a policy row for an endpoint.
    pub fn new(
        endpoint_policy_row_id: impl Into<String>,
        endpoint_identity: EndpointIdentity,
        redaction_class: TraceRedactionClass,
        scope_allowed_event_kinds: Vec<TraceEventClass>,
        current_opt_in_state: EndpointOptInState,
    ) -> Self {
        Self {
            endpoint_policy_row_id: endpoint_policy_row_id.into(),
            endpoint_identity,
            redaction_class,
            scope_allowed_event_kinds,
            current_opt_in_state,
        }
    }

    /// Returns a local diagnostics row covering every trace-event class.
    pub fn local_diagnostics() -> Self {
        Self::new(
            "endpoint_policy.trace_event.local_diagnostics",
            EndpointIdentity::new(
                "endpoint.telemetry.trace_event.local_diagnostics",
                "Trace-event local diagnostics",
                "local_only",
            ),
            TraceRedactionClass::MetadataSafeDefault,
            TraceEventClass::ALL.to_vec(),
            EndpointOptInState::LocalOnly,
        )
    }

    /// Returns an optional upload row covering every trace-event class.
    pub fn optional_upload(current_opt_in_state: EndpointOptInState) -> Self {
        Self::new(
            "endpoint_policy.trace_event.optional_upload",
            EndpointIdentity::new(
                "endpoint.telemetry.trace_event.optional_upload",
                "Trace-event optional telemetry upload",
                "optional_telemetry_upload",
            ),
            TraceRedactionClass::MetadataSafeDefault,
            TraceEventClass::ALL.to_vec(),
            current_opt_in_state,
        )
    }

    /// Returns true when this row allows the supplied event class.
    pub fn allows_event_class(&self, event_class: TraceEventClass) -> bool {
        self.scope_allowed_event_kinds.contains(&event_class)
    }

    /// Projects trace events through this endpoint policy.
    pub fn project_trace_events(&self, events: &[TraceEventRecord]) -> EndpointEventProjection {
        let projected_events = if self.current_opt_in_state.permits_event_projection() {
            events
                .iter()
                .filter(|event| {
                    TraceEventClass::from_token(event.event_class.as_ref())
                        .map(|event_class| self.allows_event_class(event_class))
                        .unwrap_or(false)
                })
                .map(|event| self.redacted_event(event))
                .collect()
        } else {
            Vec::new()
        };

        EndpointEventProjection {
            record_kind: ENDPOINT_EVENT_PROJECTION_RECORD_KIND.to_owned(),
            endpoint_policy_row: self.clone(),
            event_count: projected_events.len(),
            events_redacted_before_serialization: true,
            raw_event_content_excluded: true,
            events: projected_events,
        }
    }

    fn redacted_event(&self, event: &TraceEventRecord) -> TraceEventRecord {
        let mut event = event.clone();
        event.redaction_class = Cow::Borrowed(self.redaction_class.as_str());
        event.note = None;
        event
    }
}

/// Redacted event projection for one endpoint-policy row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointEventProjection {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Endpoint policy applied to the event list.
    pub endpoint_policy_row: EndpointPolicyRow,
    /// Number of events after opt-in, scope, and redaction policy.
    pub event_count: usize,
    /// True when copied events were redacted before this projection was built.
    pub events_redacted_before_serialization: bool,
    /// True when free-form event content is excluded from the projection.
    pub raw_event_content_excluded: bool,
    /// Redacted trace-event records admitted to this endpoint.
    pub events: Vec<TraceEventRecord>,
}

/// Returns the default trace-event endpoint policy rows for the current upload state.
pub fn default_trace_endpoint_policy_rows(
    upload_opt_in_state: EndpointOptInState,
) -> Vec<EndpointPolicyRow> {
    vec![
        EndpointPolicyRow::local_diagnostics(),
        EndpointPolicyRow::optional_upload(upload_opt_in_state),
    ]
}
