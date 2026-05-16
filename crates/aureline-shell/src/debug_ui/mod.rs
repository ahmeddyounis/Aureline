//! Beta debugger daily-use surfaces (breakpoints, call stack, variables,
//! watch, evaluate, debug console).
//!
//! This module is a shell projection. The truth source for session
//! identity, adapter capabilities, and lifecycle state is the canonical
//! [`aureline_runtime::DebugSessionSnapshot`]. The projection takes a
//! snapshot plus the shell-owned content rows (breakpoints, frames,
//! variables, watches, evaluate requests, console lines) and emits one
//! reviewable surface row per daily debugger surface that:
//!
//! - stays bound to the active session and target (every content row
//!   carries the same `session_id` and `canonical_target_id` as the
//!   snapshot);
//! - narrows only the affected surface when an adapter capability is
//!   dropped or missing — peers stay available;
//! - keeps focus return, keyboard route, and source-jump stability
//!   posture explicit so reviewers can audit pause / step / reconnect
//!   transitions without inference.
//!
//! Validation is a closed-vocabulary defect lane; the seeded fixture
//! seeds zero defects, and the failure drills exercise the three classes
//! the spec calls out: unbound rows, hidden capability drop, and unsafe
//! focus / source posture during non-steady states.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use aureline_runtime::{
    DebugAdapterCapabilityClass, DebugSessionSnapshot, DebugSessionStateClass,
};

#[cfg(test)]
mod tests;

/// Beta schema version stamped onto every record.
pub const DEBUG_UI_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every surface row.
pub const DEBUG_UI_BETA_SHARED_CONTRACT_REF: &str = "shell:debug_ui_beta:v1";

/// Stable record kind for [`DebugUiProjection`] payloads.
pub const DEBUG_UI_BETA_PROJECTION_RECORD_KIND: &str = "shell_debug_ui_beta_projection_record";

/// Stable record kind for [`DebugUiSurfaceRow`] payloads.
pub const DEBUG_UI_BETA_SURFACE_ROW_RECORD_KIND: &str = "shell_debug_ui_beta_surface_row_record";

/// Stable record kind for [`DebugUiDefect`] payloads.
pub const DEBUG_UI_BETA_DEFECT_RECORD_KIND: &str = "shell_debug_ui_beta_defect_record";

/// Stable record kind for [`DebugUiSupportExport`] payloads.
pub const DEBUG_UI_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_debug_ui_beta_support_export_record";

/// Closed vocabulary of the six daily-use debugger surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugUiSurfaceClass {
    /// Breakpoints surface (file, function, conditional, log).
    Breakpoints,
    /// Call-stack surface for the currently focused thread.
    CallStack,
    /// Variables surface (locals, arguments, registers, globals).
    Variables,
    /// Watch surface (user-entered expressions).
    Watch,
    /// Evaluate / REPL surface for one-off expression requests.
    Evaluate,
    /// Debug console (stdout, stderr, log-point output, adapter notes).
    DebugConsole,
}

impl DebugUiSurfaceClass {
    /// Stable schema token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Breakpoints => "breakpoints",
            Self::CallStack => "call_stack",
            Self::Variables => "variables",
            Self::Watch => "watch",
            Self::Evaluate => "evaluate",
            Self::DebugConsole => "debug_console",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Breakpoints => "Breakpoints",
            Self::CallStack => "Call stack",
            Self::Variables => "Variables",
            Self::Watch => "Watch",
            Self::Evaluate => "Evaluate",
            Self::DebugConsole => "Debug console",
        }
    }

    /// The six required surfaces in canonical render order.
    pub const fn required_surfaces() -> [Self; 6] {
        [
            Self::Breakpoints,
            Self::CallStack,
            Self::Variables,
            Self::Watch,
            Self::Evaluate,
            Self::DebugConsole,
        ]
    }
}

/// Closed availability vocabulary for a surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugUiAvailabilityClass {
    /// Surface is fully available for daily use.
    Available,
    /// Surface is available but the adapter dropped a non-required
    /// capability that narrows the surface (e.g. log_points dropped, so
    /// the Breakpoints surface cannot offer log-point creation but
    /// other breakpoint kinds still work).
    NarrowedByDroppedCapability,
    /// Surface is unavailable because a required capability for the
    /// surface is missing from the negotiated set.
    UnavailableMissingRequiredCapability,
    /// Surface is unavailable because the session is reconnecting; the
    /// last paused-snapshot content rows are dropped to avoid stale
    /// frames or variables.
    UnavailableDuringReconnect,
    /// Surface is unavailable because the session is quarantined.
    UnavailableDuringQuarantine,
    /// Surface is unavailable because no session is bound.
    UnavailableNoActiveSession,
}

impl DebugUiAvailabilityClass {
    /// Stable schema token recorded on the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::NarrowedByDroppedCapability => "narrowed_by_dropped_capability",
            Self::UnavailableMissingRequiredCapability => {
                "unavailable_missing_required_capability"
            }
            Self::UnavailableDuringReconnect => "unavailable_during_reconnect",
            Self::UnavailableDuringQuarantine => "unavailable_during_quarantine",
            Self::UnavailableNoActiveSession => "unavailable_no_active_session",
        }
    }

    /// Returns true when the surface is unavailable in any way.
    pub const fn is_unavailable(self) -> bool {
        matches!(
            self,
            Self::UnavailableMissingRequiredCapability
                | Self::UnavailableDuringReconnect
                | Self::UnavailableDuringQuarantine
                | Self::UnavailableNoActiveSession
        )
    }
}

/// Where focus returns when the surface loses the active row (pause,
/// step, reconnect, quarantine, terminate).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugUiFocusReturnClass {
    /// Focus returns to the row the user invoked from (editor gutter,
    /// command palette, watch entry).
    InvokingSurface,
    /// Focus parks on the session card so the user can read the state
    /// disclosure before re-engaging the surface.
    SessionCard,
    /// Focus parks on the quarantine card so the user repairs the
    /// session before continuing.
    QuarantineCard,
}

impl DebugUiFocusReturnClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvokingSurface => "invoking_surface",
            Self::SessionCard => "session_card",
            Self::QuarantineCard => "quarantine_card",
        }
    }
}

/// Source-jump stability posture for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceJumpStabilityClass {
    /// Source jumps land on a file/line we can resolve through the
    /// adapter's source mapping.
    Stable,
    /// Source mapping is degraded; jumps land on the disclosed last
    /// known frame and the surface labels the degraded path.
    DegradedNoMapping,
    /// Session is not in a steady state; the surface refuses source
    /// jumps and labels the refusal so the user does not chase a stale
    /// pointer.
    UnstableDuringReconnect,
    /// Session is quarantined; jumps are refused.
    UnstableDuringQuarantine,
    /// No session is bound; jumps are not offered.
    NotOfferedNoSession,
}

impl SourceJumpStabilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::DegradedNoMapping => "degraded_no_mapping",
            Self::UnstableDuringReconnect => "unstable_during_reconnect",
            Self::UnstableDuringQuarantine => "unstable_during_quarantine",
            Self::NotOfferedNoSession => "not_offered_no_session",
        }
    }
}

/// Breakpoint kind, closed vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BreakpointKindClass {
    Line,
    Function,
    Conditional,
    HitCount,
    LogPoint,
    Data,
    Exception,
}

impl BreakpointKindClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Function => "function",
            Self::Conditional => "conditional",
            Self::HitCount => "hit_count",
            Self::LogPoint => "log_point",
            Self::Data => "data",
            Self::Exception => "exception",
        }
    }

    /// Required adapter capability for this kind. Line breakpoints are
    /// universal in DAP and have no extra capability requirement.
    pub fn required_capability(self) -> Option<DebugAdapterCapabilityClass> {
        match self {
            Self::Line => None,
            Self::Function => Some(DebugAdapterCapabilityClass::FunctionBreakpoints),
            Self::Conditional => Some(DebugAdapterCapabilityClass::ConditionalBreakpoints),
            Self::HitCount => Some(DebugAdapterCapabilityClass::HitCountBreakpoints),
            Self::LogPoint => Some(DebugAdapterCapabilityClass::LogPoints),
            Self::Data => Some(DebugAdapterCapabilityClass::DataBreakpoints),
            Self::Exception => Some(DebugAdapterCapabilityClass::ExceptionBreakpointFilters),
        }
    }
}

/// Closed vocabulary for the defect lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebugUiDefectKind {
    /// A required surface is missing from the projection.
    MissingRequiredSurface,
    /// Two surfaces collided on the same surface class.
    DuplicateSurfaceClass,
    /// A content row carries a session id or target id that does not
    /// match the active binding (cross-session bleed).
    ContentRowNotBoundToActiveSession,
    /// A content row was kept on a row marked unavailable, hiding the
    /// degraded state behind stale data.
    ContentKeptOnUnavailableSurface,
    /// The surface claims full availability while a capability it
    /// depends on was dropped during negotiation.
    HiddenCapabilityDowngrade,
    /// The surface claims availability while a required capability is
    /// missing.
    HiddenMissingRequiredCapability,
    /// Source-jump stability is `stable` while the session is not in a
    /// steady state (reconnect, quarantine, no session).
    UnsafeSourceJumpDuringNonSteadyState,
    /// Focus returns to the invoking surface during reconnect or
    /// quarantine, which hides the disclosed state behind a normal
    /// focus path.
    UnsafeFocusReturnDuringNonSteadyState,
    /// Honesty marker was claimed false while the snapshot requires
    /// shell disclosure.
    MissingHonestyMarker,
    /// Honesty marker is true while every surface is `available` — the
    /// projection has no real disclosure to back the marker.
    UnsupportedHonestyMarker,
    /// A required surface is missing a keyboard route ref.
    MissingKeyboardRoute,
}

impl DebugUiDefectKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingRequiredSurface => "missing_required_surface",
            Self::DuplicateSurfaceClass => "duplicate_surface_class",
            Self::ContentRowNotBoundToActiveSession => "content_row_not_bound_to_active_session",
            Self::ContentKeptOnUnavailableSurface => "content_kept_on_unavailable_surface",
            Self::HiddenCapabilityDowngrade => "hidden_capability_downgrade",
            Self::HiddenMissingRequiredCapability => "hidden_missing_required_capability",
            Self::UnsafeSourceJumpDuringNonSteadyState => {
                "unsafe_source_jump_during_non_steady_state"
            }
            Self::UnsafeFocusReturnDuringNonSteadyState => {
                "unsafe_focus_return_during_non_steady_state"
            }
            Self::MissingHonestyMarker => "missing_honesty_marker",
            Self::UnsupportedHonestyMarker => "unsupported_honesty_marker",
            Self::MissingKeyboardRoute => "missing_keyboard_route",
        }
    }
}

/// Active session binding carried on every row in the projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugUiActiveBinding {
    /// Stable session id (empty when no session is bound).
    pub session_id: String,
    /// Workspace id.
    pub workspace_id: String,
    /// Canonical target id (empty when no session is bound).
    pub canonical_target_id: String,
    /// Target-class token.
    pub target_class_token: String,
    /// Target label suitable for export.
    pub target_label: String,
    /// Adapter id (empty when no session is bound).
    pub adapter_id: String,
    /// Adapter label.
    pub adapter_label: String,
    /// Lifecycle state token of the bound session.
    pub state_class_token: String,
    /// Mode token (launch / attach / reconnect; empty when no session).
    pub mode_token: String,
}

impl DebugUiActiveBinding {
    /// Build a binding from a runtime snapshot.
    pub fn from_snapshot(snapshot: &DebugSessionSnapshot) -> Self {
        Self {
            session_id: snapshot.identity.session_id.clone(),
            workspace_id: snapshot.identity.workspace_id.clone(),
            canonical_target_id: snapshot.identity.target.canonical_target_id.clone(),
            target_class_token: snapshot.identity.target.target_class_token.clone(),
            target_label: snapshot.identity.target.target_label.clone(),
            adapter_id: snapshot.identity.adapter.adapter_id.clone(),
            adapter_label: snapshot.identity.adapter.adapter_label.clone(),
            state_class_token: snapshot.state_class_token.clone(),
            mode_token: snapshot.identity.mode_token.clone(),
        }
    }

    /// Build an empty binding for the "no active session" projection.
    pub fn empty(workspace_id: impl Into<String>) -> Self {
        Self {
            session_id: String::new(),
            workspace_id: workspace_id.into(),
            canonical_target_id: String::new(),
            target_class_token: String::new(),
            target_label: String::new(),
            adapter_id: String::new(),
            adapter_label: String::new(),
            state_class_token: String::new(),
            mode_token: String::new(),
        }
    }

    /// Returns true when no session is bound.
    pub fn is_empty(&self) -> bool {
        self.session_id.is_empty()
    }
}

/// Per-surface availability and posture row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugUiSurfaceRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Surface class.
    pub surface_class: DebugUiSurfaceClass,
    /// Stable schema token for the surface class.
    pub surface_class_token: String,
    /// Reviewer-facing label.
    pub display_label: String,
    /// Availability class.
    pub availability_class: DebugUiAvailabilityClass,
    /// Stable schema token for availability.
    pub availability_class_token: String,
    /// Focus return class.
    pub focus_return_class: DebugUiFocusReturnClass,
    /// Stable schema token for focus return.
    pub focus_return_class_token: String,
    /// Source-jump stability class.
    pub source_jump_stability_class: SourceJumpStabilityClass,
    /// Stable schema token for source-jump stability.
    pub source_jump_stability_class_token: String,
    /// Keyboard route ref the surface uses for engagement.
    pub keyboard_route_ref: String,
    /// Capability tokens this surface depends on (closed vocabulary).
    pub dependent_capability_tokens: Vec<String>,
    /// Capability tokens dropped during negotiation that affect this
    /// surface.
    pub dropped_capability_tokens: Vec<String>,
    /// Capability tokens this surface required that are missing from
    /// the negotiated set.
    pub missing_required_capability_tokens: Vec<String>,
    /// Number of content rows the projection emitted on this surface.
    pub content_row_count: usize,
    /// Reviewer-facing single-line summary.
    pub summary: String,
}

/// Breakpoint content row, always bound to the active session and target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointRow {
    /// Stable breakpoint id (scoped to the session).
    pub breakpoint_id: String,
    /// Active session this breakpoint is bound to.
    pub session_id: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// Breakpoint kind.
    pub kind: BreakpointKindClass,
    /// Stable schema token for the kind.
    pub kind_token: String,
    /// Source ref (may be empty for function / exception breakpoints).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Source line (1-based, may be omitted).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// Reviewer-facing label.
    pub label: String,
    /// True when the adapter verified the breakpoint location.
    pub verified: bool,
    /// True when this breakpoint is currently enabled.
    pub enabled: bool,
}

/// Call-stack frame row for the focused thread.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CallStackFrameRow {
    /// Stable frame id.
    pub frame_id: String,
    /// Active session this frame is bound to.
    pub session_id: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// Thread id for the frame.
    pub thread_id: String,
    /// Zero-based frame index (0 == top).
    pub frame_index: u32,
    /// Function / method name.
    pub function_label: String,
    /// Source ref, when resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<String>,
    /// Source line, when resolved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
    /// True when this frame is the currently focused frame in the surface.
    pub is_focused_frame: bool,
}

/// Variable scope row (locals, arguments, registers, globals, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableScopeRow {
    /// Stable scope id.
    pub scope_id: String,
    /// Active session this scope is bound to.
    pub session_id: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// Frame id this scope hangs off.
    pub frame_id: String,
    /// Scope name (e.g. "Locals", "Arguments").
    pub scope_name: String,
    /// Number of variables in this scope.
    pub variable_count: usize,
    /// True when the scope is currently expanded in the surface.
    pub expanded: bool,
}

/// Watch-expression row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WatchExpressionRow {
    /// Stable watch id.
    pub watch_id: String,
    /// Active session this watch is bound to.
    pub session_id: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// User-entered expression.
    pub expression: String,
    /// Last evaluated value summary (empty while not yet evaluated).
    pub last_value_summary: String,
    /// True when the last evaluation failed.
    pub last_evaluation_failed: bool,
}

/// One evaluate request observed on the evaluate / REPL surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvaluateRequestRow {
    /// Stable request id.
    pub request_id: String,
    /// Active session this request is bound to.
    pub session_id: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// Expression evaluated.
    pub expression: String,
    /// Result summary (empty while pending).
    pub result_summary: String,
    /// True when the request errored.
    pub errored: bool,
    /// Timestamp the request was issued.
    pub issued_at: String,
}

/// One line emitted on the debug console.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsoleLineRow {
    /// Stable line id.
    pub line_id: String,
    /// Active session this line is bound to.
    pub session_id: String,
    /// Canonical target id.
    pub canonical_target_id: String,
    /// Stream class (stdout / stderr / log_point / adapter_note).
    pub stream_class_token: String,
    /// Line body.
    pub body: String,
    /// Timestamp the line was observed.
    pub observed_at: String,
}

/// One reported defect.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugUiDefect {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Defect kind.
    pub defect_kind: DebugUiDefectKind,
    /// Stable schema token for the defect kind.
    pub defect_kind_token: String,
    /// Surface class implicated, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_class_token: Option<String>,
    /// Field implicated.
    pub field: String,
    /// Free-form note.
    pub note: String,
}

/// Shell-owned content payload the projection consumes.
///
/// The shell supplies this; the projection enforces every row is bound
/// to the active session and target.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugUiContent {
    /// Breakpoint rows the user has defined.
    #[serde(default)]
    pub breakpoints: Vec<BreakpointRow>,
    /// Call-stack frame rows for the focused thread.
    #[serde(default)]
    pub call_stack_frames: Vec<CallStackFrameRow>,
    /// Variable scope rows for the focused frame.
    #[serde(default)]
    pub variable_scopes: Vec<VariableScopeRow>,
    /// Watch expressions the user has entered.
    #[serde(default)]
    pub watch_expressions: Vec<WatchExpressionRow>,
    /// Evaluate / REPL requests issued on the active session.
    #[serde(default)]
    pub evaluate_requests: Vec<EvaluateRequestRow>,
    /// Console lines emitted on the active session.
    #[serde(default)]
    pub console_lines: Vec<ConsoleLineRow>,
}

/// One full debug-ui projection ready for the live shell, headless
/// inspector, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugUiProjection {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable projection id.
    pub projection_id: String,
    /// Wall-clock capture timestamp.
    pub captured_at: String,
    /// Active session binding.
    pub active_binding: DebugUiActiveBinding,
    /// Per-surface availability rows in canonical order.
    pub surfaces: Vec<DebugUiSurfaceRow>,
    /// Breakpoint content rows.
    pub breakpoints: Vec<BreakpointRow>,
    /// Call-stack frame rows.
    pub call_stack_frames: Vec<CallStackFrameRow>,
    /// Variable scope rows.
    pub variable_scopes: Vec<VariableScopeRow>,
    /// Watch expression rows.
    pub watch_expressions: Vec<WatchExpressionRow>,
    /// Evaluate request rows.
    pub evaluate_requests: Vec<EvaluateRequestRow>,
    /// Console line rows.
    pub console_lines: Vec<ConsoleLineRow>,
    /// True when the shell MUST disclose the session state on its
    /// status surface.
    pub honesty_marker_present: bool,
    /// Export-safe summary line.
    pub export_safe_summary: String,
}

/// Support-export wrapper carrying the projection plus the surface
/// row vocabulary summary the support packet renders.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebugUiSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Projection body.
    pub projection: DebugUiProjection,
    /// Availability counts keyed by token (e.g. `available`: 4).
    pub availability_counts: BTreeMap<String, usize>,
    /// True when no raw inferior memory or stack values are present.
    pub raw_private_material_excluded: bool,
}

impl DebugUiSupportExport {
    /// Build the support export from a projection.
    pub fn from_projection(
        export_id: impl Into<String>,
        captured_at: impl Into<String>,
        projection: DebugUiProjection,
    ) -> Self {
        let captured_at = captured_at.into();
        let mut availability_counts: BTreeMap<String, usize> = BTreeMap::new();
        for row in &projection.surfaces {
            *availability_counts
                .entry(row.availability_class_token.clone())
                .or_default() += 1;
        }
        Self {
            record_kind: DEBUG_UI_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DEBUG_UI_BETA_SCHEMA_VERSION,
            shared_contract_ref: DEBUG_UI_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            captured_at,
            projection,
            availability_counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Inputs to a single projection.
pub struct DebugUiProjectionInput<'a> {
    /// Stable projection id baked into the record.
    pub projection_id: &'a str,
    /// Workspace id this projection is scoped to.
    pub workspace_id: &'a str,
    /// Optional snapshot. `None` means no session is currently bound.
    pub snapshot: Option<&'a DebugSessionSnapshot>,
    /// Content rows the shell supplies.
    pub content: DebugUiContent,
    /// Capture timestamp.
    pub captured_at: &'a str,
}

/// Project the daily-use debug-ui surfaces from a runtime snapshot plus
/// shell content.
pub fn project_debug_ui(input: DebugUiProjectionInput<'_>) -> DebugUiProjection {
    let DebugUiProjectionInput {
        projection_id,
        workspace_id,
        snapshot,
        content,
        captured_at,
    } = input;

    let (binding, posture, snapshot_requires_disclosure, dropped_caps, agreed_caps) =
        match snapshot {
            Some(s) => (
                DebugUiActiveBinding::from_snapshot(s),
                session_posture(s),
                s.requires_shell_disclosure(),
                s.dropped_capabilities.clone(),
                s.agreed_capabilities.clone(),
            ),
            None => (
                DebugUiActiveBinding::empty(workspace_id),
                SessionPosture::NoSession,
                false,
                Vec::new(),
                Vec::new(),
            ),
        };

    let surfaces: Vec<DebugUiSurfaceRow> = DebugUiSurfaceClass::required_surfaces()
        .into_iter()
        .map(|class| {
            build_surface_row(class, posture, &agreed_caps, &dropped_caps, &content)
        })
        .collect();

    // Honesty marker fires when the runtime snapshot already requires
    // disclosure (reconnect, quarantine, post-fault terminated), or
    // when any rendered surface is narrowed or unavailable for a
    // reason that has a real story to tell. The "no active session"
    // posture is the baseline empty state and does not light the
    // marker on its own.
    let surfaces_demand_disclosure = surfaces.iter().any(|row| match row.availability_class {
        DebugUiAvailabilityClass::NarrowedByDroppedCapability
        | DebugUiAvailabilityClass::UnavailableMissingRequiredCapability
        | DebugUiAvailabilityClass::UnavailableDuringReconnect
        | DebugUiAvailabilityClass::UnavailableDuringQuarantine => true,
        _ => false,
    });
    let honesty_marker_present = snapshot_requires_disclosure || surfaces_demand_disclosure;

    // Content rows survive on Available / NarrowedByDroppedCapability
    // posture; anything else drops content so a paused snapshot does not
    // bleed across reconnect / quarantine / no-session boundaries.
    let keep_content = matches!(
        posture,
        SessionPosture::Paused | SessionPosture::Running
    );
    let (breakpoints, call_stack_frames, variable_scopes, watch_expressions, evaluate_requests, console_lines) =
        if keep_content {
            (
                content.breakpoints,
                content.call_stack_frames,
                content.variable_scopes,
                content.watch_expressions,
                content.evaluate_requests,
                content.console_lines,
            )
        } else {
            (
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
                Vec::new(),
            )
        };

    let export_safe_summary = format_summary(&binding, &surfaces, posture);

    DebugUiProjection {
        record_kind: DEBUG_UI_BETA_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: DEBUG_UI_BETA_SCHEMA_VERSION,
        shared_contract_ref: DEBUG_UI_BETA_SHARED_CONTRACT_REF.to_owned(),
        projection_id: projection_id.to_owned(),
        captured_at: captured_at.to_owned(),
        active_binding: binding,
        surfaces,
        breakpoints,
        call_stack_frames,
        variable_scopes,
        watch_expressions,
        evaluate_requests,
        console_lines,
        honesty_marker_present,
        export_safe_summary,
    }
}

#[derive(Debug, Clone, Copy)]
enum SessionPosture {
    NoSession,
    Negotiating,
    Running,
    Paused,
    Reconnecting,
    Quarantined,
    Terminated,
}

fn session_posture(snapshot: &DebugSessionSnapshot) -> SessionPosture {
    match snapshot.state_class {
        DebugSessionStateClass::LaunchRequested
        | DebugSessionStateClass::NegotiatingCapabilities
        | DebugSessionStateClass::HandshakeComplete => SessionPosture::Negotiating,
        DebugSessionStateClass::AttachedRunning | DebugSessionStateClass::LaunchedRunning => {
            SessionPosture::Running
        }
        DebugSessionStateClass::Paused => SessionPosture::Paused,
        DebugSessionStateClass::Reconnecting => SessionPosture::Reconnecting,
        DebugSessionStateClass::Degraded => SessionPosture::Running,
        DebugSessionStateClass::Quarantined => SessionPosture::Quarantined,
        DebugSessionStateClass::Terminated => SessionPosture::Terminated,
    }
}

fn build_surface_row(
    class: DebugUiSurfaceClass,
    posture: SessionPosture,
    agreed: &[DebugAdapterCapabilityClass],
    dropped: &[DebugAdapterCapabilityClass],
    content: &DebugUiContent,
) -> DebugUiSurfaceRow {
    let dependent_caps = dependent_capabilities(class);
    let required_caps = required_capabilities(class);

    let missing_required: Vec<DebugAdapterCapabilityClass> = required_caps
        .iter()
        .copied()
        .filter(|c| !agreed.contains(c))
        .collect();
    let dropped_for_surface: Vec<DebugAdapterCapabilityClass> = dependent_caps
        .iter()
        .copied()
        .filter(|c| dropped.contains(c))
        .collect();

    let availability = match posture {
        SessionPosture::NoSession => DebugUiAvailabilityClass::UnavailableNoActiveSession,
        SessionPosture::Reconnecting => DebugUiAvailabilityClass::UnavailableDuringReconnect,
        SessionPosture::Quarantined => DebugUiAvailabilityClass::UnavailableDuringQuarantine,
        SessionPosture::Terminated => DebugUiAvailabilityClass::UnavailableNoActiveSession,
        SessionPosture::Negotiating => DebugUiAvailabilityClass::UnavailableDuringReconnect,
        SessionPosture::Running | SessionPosture::Paused => {
            if !missing_required.is_empty() {
                DebugUiAvailabilityClass::UnavailableMissingRequiredCapability
            } else if !dropped_for_surface.is_empty() {
                DebugUiAvailabilityClass::NarrowedByDroppedCapability
            } else {
                DebugUiAvailabilityClass::Available
            }
        }
    };

    let (focus_return, source_jump) = focus_and_source_for(class, posture, availability);

    let content_row_count = match class {
        DebugUiSurfaceClass::Breakpoints => content.breakpoints.len(),
        DebugUiSurfaceClass::CallStack => content.call_stack_frames.len(),
        DebugUiSurfaceClass::Variables => content.variable_scopes.len(),
        DebugUiSurfaceClass::Watch => content.watch_expressions.len(),
        DebugUiSurfaceClass::Evaluate => content.evaluate_requests.len(),
        DebugUiSurfaceClass::DebugConsole => content.console_lines.len(),
    };
    let content_row_count = if matches!(
        posture,
        SessionPosture::Paused | SessionPosture::Running
    ) {
        content_row_count
    } else {
        0
    };

    let summary = format!(
        "{} :: {} ({} rows)",
        class.display_label(),
        availability.as_str(),
        content_row_count
    );

    DebugUiSurfaceRow {
        record_kind: DEBUG_UI_BETA_SURFACE_ROW_RECORD_KIND.to_owned(),
        schema_version: DEBUG_UI_BETA_SCHEMA_VERSION,
        surface_class: class,
        surface_class_token: class.as_str().to_owned(),
        display_label: class.display_label().to_owned(),
        availability_class: availability,
        availability_class_token: availability.as_str().to_owned(),
        focus_return_class: focus_return,
        focus_return_class_token: focus_return.as_str().to_owned(),
        source_jump_stability_class: source_jump,
        source_jump_stability_class_token: source_jump.as_str().to_owned(),
        keyboard_route_ref: keyboard_route_ref_for(class).to_owned(),
        dependent_capability_tokens: dependent_caps
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        dropped_capability_tokens: dropped_for_surface
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        missing_required_capability_tokens: missing_required
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        content_row_count,
        summary,
    }
}

fn dependent_capabilities(class: DebugUiSurfaceClass) -> &'static [DebugAdapterCapabilityClass] {
    match class {
        DebugUiSurfaceClass::Breakpoints => &[
            DebugAdapterCapabilityClass::FunctionBreakpoints,
            DebugAdapterCapabilityClass::ConditionalBreakpoints,
            DebugAdapterCapabilityClass::HitCountBreakpoints,
            DebugAdapterCapabilityClass::LogPoints,
            DebugAdapterCapabilityClass::DataBreakpoints,
            DebugAdapterCapabilityClass::ExceptionBreakpointFilters,
            DebugAdapterCapabilityClass::BreakpointLocations,
        ],
        DebugUiSurfaceClass::CallStack => &[DebugAdapterCapabilityClass::GranularityStepping],
        DebugUiSurfaceClass::Variables => &[DebugAdapterCapabilityClass::MemoryAccess],
        DebugUiSurfaceClass::Watch => &[DebugAdapterCapabilityClass::ConditionalBreakpoints],
        DebugUiSurfaceClass::Evaluate => &[],
        DebugUiSurfaceClass::DebugConsole => &[
            DebugAdapterCapabilityClass::LogPoints,
            DebugAdapterCapabilityClass::ModulesEvents,
            DebugAdapterCapabilityClass::LoadedSourcesEvents,
        ],
    }
}

fn required_capabilities(_class: DebugUiSurfaceClass) -> &'static [DebugAdapterCapabilityClass] {
    // None of the daily-use surfaces require a specific capability in
    // the beta wedge: line breakpoints, call stack, variables, watch,
    // evaluate, and the console all ride on the base DAP contract. The
    // breakpoint-kind-class capability requirements are enforced at the
    // row level by [`BreakpointKindClass::required_capability`].
    &[]
}

fn keyboard_route_ref_for(class: DebugUiSurfaceClass) -> &'static str {
    match class {
        DebugUiSurfaceClass::Breakpoints => "keyboard:debug.surface.breakpoints",
        DebugUiSurfaceClass::CallStack => "keyboard:debug.surface.call_stack",
        DebugUiSurfaceClass::Variables => "keyboard:debug.surface.variables",
        DebugUiSurfaceClass::Watch => "keyboard:debug.surface.watch",
        DebugUiSurfaceClass::Evaluate => "keyboard:debug.surface.evaluate",
        DebugUiSurfaceClass::DebugConsole => "keyboard:debug.surface.debug_console",
    }
}

fn focus_and_source_for(
    _class: DebugUiSurfaceClass,
    posture: SessionPosture,
    availability: DebugUiAvailabilityClass,
) -> (DebugUiFocusReturnClass, SourceJumpStabilityClass) {
    match posture {
        SessionPosture::Quarantined => (
            DebugUiFocusReturnClass::QuarantineCard,
            SourceJumpStabilityClass::UnstableDuringQuarantine,
        ),
        SessionPosture::Reconnecting | SessionPosture::Negotiating => (
            DebugUiFocusReturnClass::SessionCard,
            SourceJumpStabilityClass::UnstableDuringReconnect,
        ),
        SessionPosture::NoSession | SessionPosture::Terminated => (
            DebugUiFocusReturnClass::SessionCard,
            SourceJumpStabilityClass::NotOfferedNoSession,
        ),
        SessionPosture::Paused | SessionPosture::Running => {
            let source = if matches!(
                availability,
                DebugUiAvailabilityClass::UnavailableMissingRequiredCapability
            ) {
                SourceJumpStabilityClass::DegradedNoMapping
            } else {
                SourceJumpStabilityClass::Stable
            };
            (DebugUiFocusReturnClass::InvokingSurface, source)
        }
    }
}

fn format_summary(
    binding: &DebugUiActiveBinding,
    surfaces: &[DebugUiSurfaceRow],
    posture: SessionPosture,
) -> String {
    let state = if binding.is_empty() {
        "no_active_session".to_string()
    } else {
        binding.state_class_token.clone()
    };
    let target = if binding.is_empty() {
        "(none)".to_string()
    } else {
        binding.canonical_target_id.clone()
    };
    let available = surfaces
        .iter()
        .filter(|r| matches!(r.availability_class, DebugUiAvailabilityClass::Available))
        .count();
    let narrowed = surfaces
        .iter()
        .filter(|r| {
            matches!(
                r.availability_class,
                DebugUiAvailabilityClass::NarrowedByDroppedCapability
            )
        })
        .count();
    let _ = posture; // posture is implicit in `state`.
    format!(
        "debug_ui_beta state={state} target={target} surfaces={total} \
         available={available} narrowed={narrowed}",
        total = surfaces.len(),
    )
}

/// Validate a debug-ui projection. Returns the list of detected
/// defects; an empty list means the projection is clean.
pub fn validate_debug_ui_projection(
    projection: &DebugUiProjection,
) -> Result<(), Vec<DebugUiDefect>> {
    let mut defects = Vec::new();
    validate_surfaces_complete(projection, &mut defects);
    validate_content_binding(projection, &mut defects);
    validate_content_only_when_available(projection, &mut defects);
    validate_capability_disclosure(projection, &mut defects);
    validate_safety_posture(projection, &mut defects);
    validate_honesty_marker(projection, &mut defects);
    validate_keyboard_routes(projection, &mut defects);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

fn defect(kind: DebugUiDefectKind, field: &str, note: String) -> DebugUiDefect {
    DebugUiDefect {
        record_kind: DEBUG_UI_BETA_DEFECT_RECORD_KIND.to_owned(),
        schema_version: DEBUG_UI_BETA_SCHEMA_VERSION,
        defect_kind: kind,
        defect_kind_token: kind.as_str().to_owned(),
        surface_class_token: None,
        field: field.to_owned(),
        note,
    }
}

fn surface_defect(
    kind: DebugUiDefectKind,
    surface_class_token: &str,
    field: &str,
    note: String,
) -> DebugUiDefect {
    DebugUiDefect {
        record_kind: DEBUG_UI_BETA_DEFECT_RECORD_KIND.to_owned(),
        schema_version: DEBUG_UI_BETA_SCHEMA_VERSION,
        defect_kind: kind,
        defect_kind_token: kind.as_str().to_owned(),
        surface_class_token: Some(surface_class_token.to_owned()),
        field: field.to_owned(),
        note,
    }
}

fn validate_surfaces_complete(projection: &DebugUiProjection, defects: &mut Vec<DebugUiDefect>) {
    let mut seen: BTreeSet<DebugUiSurfaceClass> = BTreeSet::new();
    for row in &projection.surfaces {
        if !seen.insert(row.surface_class) {
            defects.push(surface_defect(
                DebugUiDefectKind::DuplicateSurfaceClass,
                row.surface_class_token.as_str(),
                "surface_class",
                format!("surface class {} is repeated", row.surface_class_token),
            ));
        }
    }
    for required in DebugUiSurfaceClass::required_surfaces() {
        if !seen.contains(&required) {
            defects.push(surface_defect(
                DebugUiDefectKind::MissingRequiredSurface,
                required.as_str(),
                "surfaces",
                format!("required surface {} is missing", required.as_str()),
            ));
        }
    }
}

fn check_binding(
    binding: &DebugUiActiveBinding,
    session_id: &str,
    canonical_target_id: &str,
    surface: &str,
    row_id: &str,
    defects: &mut Vec<DebugUiDefect>,
) {
    if session_id != binding.session_id || canonical_target_id != binding.canonical_target_id {
        defects.push(surface_defect(
            DebugUiDefectKind::ContentRowNotBoundToActiveSession,
            surface,
            "active_binding",
            format!(
                "row {row_id} carries session_id={session_id} target={canonical_target_id} \
                 but active binding is session_id={active_session} target={active_target}",
                active_session = binding.session_id,
                active_target = binding.canonical_target_id,
            ),
        ));
    }
}

fn validate_content_binding(projection: &DebugUiProjection, defects: &mut Vec<DebugUiDefect>) {
    let binding = &projection.active_binding;
    if binding.is_empty() {
        // Nothing to check; the "kept content on unavailable surface"
        // pass below will flag any rows that slipped through.
        return;
    }
    for row in &projection.breakpoints {
        check_binding(
            binding,
            &row.session_id,
            &row.canonical_target_id,
            DebugUiSurfaceClass::Breakpoints.as_str(),
            &row.breakpoint_id,
            defects,
        );
    }
    for row in &projection.call_stack_frames {
        check_binding(
            binding,
            &row.session_id,
            &row.canonical_target_id,
            DebugUiSurfaceClass::CallStack.as_str(),
            &row.frame_id,
            defects,
        );
    }
    for row in &projection.variable_scopes {
        check_binding(
            binding,
            &row.session_id,
            &row.canonical_target_id,
            DebugUiSurfaceClass::Variables.as_str(),
            &row.scope_id,
            defects,
        );
    }
    for row in &projection.watch_expressions {
        check_binding(
            binding,
            &row.session_id,
            &row.canonical_target_id,
            DebugUiSurfaceClass::Watch.as_str(),
            &row.watch_id,
            defects,
        );
    }
    for row in &projection.evaluate_requests {
        check_binding(
            binding,
            &row.session_id,
            &row.canonical_target_id,
            DebugUiSurfaceClass::Evaluate.as_str(),
            &row.request_id,
            defects,
        );
    }
    for row in &projection.console_lines {
        check_binding(
            binding,
            &row.session_id,
            &row.canonical_target_id,
            DebugUiSurfaceClass::DebugConsole.as_str(),
            &row.line_id,
            defects,
        );
    }
}

fn validate_content_only_when_available(
    projection: &DebugUiProjection,
    defects: &mut Vec<DebugUiDefect>,
) {
    for row in &projection.surfaces {
        if !row.availability_class.is_unavailable() {
            continue;
        }
        if row.content_row_count == 0 {
            continue;
        }
        defects.push(surface_defect(
            DebugUiDefectKind::ContentKeptOnUnavailableSurface,
            row.surface_class_token.as_str(),
            "content_row_count",
            format!(
                "surface {surface} is {availability} but kept {n} content rows",
                surface = row.surface_class_token,
                availability = row.availability_class_token,
                n = row.content_row_count
            ),
        ));
    }
}

fn validate_capability_disclosure(
    projection: &DebugUiProjection,
    defects: &mut Vec<DebugUiDefect>,
) {
    for row in &projection.surfaces {
        if matches!(row.availability_class, DebugUiAvailabilityClass::Available)
            && !row.dropped_capability_tokens.is_empty()
        {
            defects.push(surface_defect(
                DebugUiDefectKind::HiddenCapabilityDowngrade,
                row.surface_class_token.as_str(),
                "availability_class",
                format!(
                    "surface {} claims available while dropped capabilities are: {}",
                    row.surface_class_token,
                    row.dropped_capability_tokens.join(",")
                ),
            ));
        }
        if matches!(row.availability_class, DebugUiAvailabilityClass::Available)
            && !row.missing_required_capability_tokens.is_empty()
        {
            defects.push(surface_defect(
                DebugUiDefectKind::HiddenMissingRequiredCapability,
                row.surface_class_token.as_str(),
                "availability_class",
                format!(
                    "surface {} claims available while required capabilities are missing: {}",
                    row.surface_class_token,
                    row.missing_required_capability_tokens.join(",")
                ),
            ));
        }
    }
}

fn validate_safety_posture(projection: &DebugUiProjection, defects: &mut Vec<DebugUiDefect>) {
    for row in &projection.surfaces {
        match row.availability_class {
            DebugUiAvailabilityClass::UnavailableDuringReconnect
            | DebugUiAvailabilityClass::UnavailableDuringQuarantine
            | DebugUiAvailabilityClass::UnavailableNoActiveSession => {
                if matches!(
                    row.source_jump_stability_class,
                    SourceJumpStabilityClass::Stable
                ) {
                    defects.push(surface_defect(
                        DebugUiDefectKind::UnsafeSourceJumpDuringNonSteadyState,
                        row.surface_class_token.as_str(),
                        "source_jump_stability_class",
                        format!(
                            "surface {} is {} but advertises stable source jumps",
                            row.surface_class_token, row.availability_class_token
                        ),
                    ));
                }
                if matches!(
                    row.focus_return_class,
                    DebugUiFocusReturnClass::InvokingSurface
                ) {
                    defects.push(surface_defect(
                        DebugUiDefectKind::UnsafeFocusReturnDuringNonSteadyState,
                        row.surface_class_token.as_str(),
                        "focus_return_class",
                        format!(
                            "surface {} is {} but routes focus back to the invoking surface",
                            row.surface_class_token, row.availability_class_token
                        ),
                    ));
                }
            }
            _ => {}
        }
    }
}

fn validate_honesty_marker(projection: &DebugUiProjection, defects: &mut Vec<DebugUiDefect>) {
    let demands_disclosure = projection.surfaces.iter().any(|row| {
        matches!(
            row.availability_class,
            DebugUiAvailabilityClass::NarrowedByDroppedCapability
                | DebugUiAvailabilityClass::UnavailableMissingRequiredCapability
                | DebugUiAvailabilityClass::UnavailableDuringReconnect
                | DebugUiAvailabilityClass::UnavailableDuringQuarantine
        )
    });
    if demands_disclosure && !projection.honesty_marker_present {
        defects.push(defect(
            DebugUiDefectKind::MissingHonestyMarker,
            "honesty_marker_present",
            "projection has narrowed or unavailable surfaces but honesty_marker_present=false"
                .to_owned(),
        ));
    }
    if projection.honesty_marker_present && !demands_disclosure {
        defects.push(defect(
            DebugUiDefectKind::UnsupportedHonestyMarker,
            "honesty_marker_present",
            "projection claims honesty_marker_present=true but no surface signals disclosure"
                .to_owned(),
        ));
    }
}

fn validate_keyboard_routes(projection: &DebugUiProjection, defects: &mut Vec<DebugUiDefect>) {
    for row in &projection.surfaces {
        if row.keyboard_route_ref.is_empty() {
            defects.push(surface_defect(
                DebugUiDefectKind::MissingKeyboardRoute,
                row.surface_class_token.as_str(),
                "keyboard_route_ref",
                format!("surface {} is missing a keyboard route ref", row.surface_class_token),
            ));
        }
    }
}

/// Build the seeded protected-walk projection used by the headless
/// inspector and tests. The walk simulates a paused local launch on a
/// Node DAP adapter with `log_points` dropped during negotiation, so
/// the Breakpoints and DebugConsole surfaces narrow while the rest stay
/// available.
pub fn seeded_protected_walk_projection() -> DebugUiProjection {
    use aureline_runtime::{
        DapHostSupervisor, DebugAdapterCapabilityRequest, DebugAdapterCapabilityResponse,
        DebugAdapterNegotiationInput, DebugSessionLaunchSpec, DebugSessionTargetIdentity,
    };
    let mut supervisor = DapHostSupervisor::new();
    let target = DebugSessionTargetIdentity {
        canonical_target_id: "target:local:debugged:node:01".into(),
        target_class_token: "local_host".into(),
        target_label: "Local Node process".into(),
        working_directory_digest: Some("digest:cwd:webapp".into()),
        inferior_process_id: Some(4242),
    };
    let session_id = supervisor.open_session(
        DebugSessionLaunchSpec::local_launch(
            "ws-debug-ui",
            "workspace:root:webapp",
            "language:typescript",
            "ctx:ws-debug-ui:01",
            "adapter:node:dap",
            "Node DAP adapter",
            "1.2.3",
            "DAP/1.55",
            target,
        ),
        "2026-05-15T00:00:00Z",
    );
    supervisor
        .negotiate(
            &session_id,
            DebugAdapterNegotiationInput::AdapterResponded {
                request: DebugAdapterCapabilityRequest::new(
                    [
                        DebugAdapterCapabilityClass::FunctionBreakpoints,
                        DebugAdapterCapabilityClass::ConditionalBreakpoints,
                        DebugAdapterCapabilityClass::LogPoints,
                    ],
                    [DebugAdapterCapabilityClass::FunctionBreakpoints],
                ),
                response: DebugAdapterCapabilityResponse::new(
                    [
                        DebugAdapterCapabilityClass::FunctionBreakpoints,
                        DebugAdapterCapabilityClass::ConditionalBreakpoints,
                    ],
                    "DAP/1.55",
                ),
            },
            "2026-05-15T00:00:01Z",
        )
        .expect("negotiation succeeds");
    supervisor
        .mark_session_ready(&session_id, "2026-05-15T00:00:02Z")
        .expect("ready");
    supervisor
        .mark_paused(
            &session_id,
            "2026-05-15T00:00:10Z",
            "Paused at breakpoint app.ts:42",
        )
        .expect("paused");
    let snapshot = supervisor.snapshot(&session_id).expect("snapshot present");
    let target_id = snapshot.identity.target.canonical_target_id.clone();
    let content = seeded_paused_content(&session_id, &target_id);
    project_debug_ui(DebugUiProjectionInput {
        projection_id: "shell:debug_ui_beta:projection:protected_walk",
        workspace_id: "ws-debug-ui",
        snapshot: Some(&snapshot),
        content,
        captured_at: "2026-05-15T00:00:11Z",
    })
}

fn seeded_paused_content(session_id: &str, target_id: &str) -> DebugUiContent {
    DebugUiContent {
        breakpoints: vec![
            BreakpointRow {
                breakpoint_id: "bp:1".into(),
                session_id: session_id.to_owned(),
                canonical_target_id: target_id.to_owned(),
                kind: BreakpointKindClass::Line,
                kind_token: BreakpointKindClass::Line.as_str().to_owned(),
                source_ref: Some("workspace:webapp/src/app.ts".into()),
                line: Some(42),
                label: "app.ts:42".into(),
                verified: true,
                enabled: true,
            },
            BreakpointRow {
                breakpoint_id: "bp:2".into(),
                session_id: session_id.to_owned(),
                canonical_target_id: target_id.to_owned(),
                kind: BreakpointKindClass::Function,
                kind_token: BreakpointKindClass::Function.as_str().to_owned(),
                source_ref: None,
                line: None,
                label: "render()".into(),
                verified: true,
                enabled: true,
            },
        ],
        call_stack_frames: vec![
            CallStackFrameRow {
                frame_id: "frame:0".into(),
                session_id: session_id.to_owned(),
                canonical_target_id: target_id.to_owned(),
                thread_id: "thread:main".into(),
                frame_index: 0,
                function_label: "render".into(),
                source_ref: Some("workspace:webapp/src/app.ts".into()),
                line: Some(42),
                is_focused_frame: true,
            },
            CallStackFrameRow {
                frame_id: "frame:1".into(),
                session_id: session_id.to_owned(),
                canonical_target_id: target_id.to_owned(),
                thread_id: "thread:main".into(),
                frame_index: 1,
                function_label: "main".into(),
                source_ref: Some("workspace:webapp/src/main.ts".into()),
                line: Some(7),
                is_focused_frame: false,
            },
        ],
        variable_scopes: vec![VariableScopeRow {
            scope_id: "scope:locals".into(),
            session_id: session_id.to_owned(),
            canonical_target_id: target_id.to_owned(),
            frame_id: "frame:0".into(),
            scope_name: "Locals".into(),
            variable_count: 3,
            expanded: true,
        }],
        watch_expressions: vec![WatchExpressionRow {
            watch_id: "watch:1".into(),
            session_id: session_id.to_owned(),
            canonical_target_id: target_id.to_owned(),
            expression: "props.title".into(),
            last_value_summary: "\"hello\"".into(),
            last_evaluation_failed: false,
        }],
        evaluate_requests: vec![EvaluateRequestRow {
            request_id: "eval:1".into(),
            session_id: session_id.to_owned(),
            canonical_target_id: target_id.to_owned(),
            expression: "Object.keys(props)".into(),
            result_summary: "[\"title\", \"items\"]".into(),
            errored: false,
            issued_at: "2026-05-15T00:00:10Z".into(),
        }],
        console_lines: vec![ConsoleLineRow {
            line_id: "console:1".into(),
            session_id: session_id.to_owned(),
            canonical_target_id: target_id.to_owned(),
            stream_class_token: "stdout".into(),
            body: "render() entered".into(),
            observed_at: "2026-05-15T00:00:09Z".into(),
        }],
    }
}

/// Build the reconnect failure-drill projection: the same session, but
/// the adapter crashed inside budget so the projection is in
/// `reconnecting`. Every surface row narrows to
/// `unavailable_during_reconnect` and content rows are dropped.
pub fn seeded_reconnect_drill_projection() -> DebugUiProjection {
    use aureline_runtime::{
        DapHostSupervisor, DebugAdapterCapabilityRequest, DebugAdapterCapabilityResponse,
        DebugAdapterNegotiationInput, DebugSessionExitReasonClass, DebugSessionLaunchSpec,
        DebugSessionTargetIdentity,
    };
    let mut supervisor = DapHostSupervisor::new();
    let target = DebugSessionTargetIdentity {
        canonical_target_id: "target:local:debugged:node:01".into(),
        target_class_token: "local_host".into(),
        target_label: "Local Node process".into(),
        working_directory_digest: Some("digest:cwd:webapp".into()),
        inferior_process_id: Some(4242),
    };
    let session_id = supervisor.open_session(
        DebugSessionLaunchSpec::local_launch(
            "ws-debug-ui",
            "workspace:root:webapp",
            "language:typescript",
            "ctx:ws-debug-ui:01",
            "adapter:node:dap",
            "Node DAP adapter",
            "1.2.3",
            "DAP/1.55",
            target,
        ),
        "2026-05-15T00:01:00Z",
    );
    supervisor
        .negotiate(
            &session_id,
            DebugAdapterNegotiationInput::AdapterResponded {
                request: DebugAdapterCapabilityRequest::new(
                    [DebugAdapterCapabilityClass::FunctionBreakpoints],
                    [DebugAdapterCapabilityClass::FunctionBreakpoints],
                ),
                response: DebugAdapterCapabilityResponse::new(
                    [DebugAdapterCapabilityClass::FunctionBreakpoints],
                    "DAP/1.55",
                ),
            },
            "2026-05-15T00:01:01Z",
        )
        .expect("negotiation succeeds");
    supervisor
        .mark_session_ready(&session_id, "2026-05-15T00:01:02Z")
        .expect("ready");
    supervisor
        .record_adapter_exit(
            &session_id,
            DebugSessionExitReasonClass::AdapterCrashUnhandled,
            "2026-05-15T00:01:03Z",
        )
        .expect("crash recorded");
    let snapshot = supervisor.snapshot(&session_id).expect("snapshot present");
    project_debug_ui(DebugUiProjectionInput {
        projection_id: "shell:debug_ui_beta:projection:reconnect_drill",
        workspace_id: "ws-debug-ui",
        snapshot: Some(&snapshot),
        content: DebugUiContent::default(),
        captured_at: "2026-05-15T00:01:04Z",
    })
}

/// Build the no-session drill projection: no snapshot bound. All
/// surfaces narrow to `unavailable_no_active_session` and content rows
/// are empty. The honesty marker stays false because the runtime has
/// nothing to disclose.
pub fn seeded_no_session_drill_projection() -> DebugUiProjection {
    project_debug_ui(DebugUiProjectionInput {
        projection_id: "shell:debug_ui_beta:projection:no_session_drill",
        workspace_id: "ws-debug-ui",
        snapshot: None,
        content: DebugUiContent::default(),
        captured_at: "2026-05-15T00:02:00Z",
    })
}
