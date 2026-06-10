//! Notebook debugger-support states, breakpoint affordances, and unsupported-state cues.
//!
//! This module materializes the composed debugger surface that the notebook chrome
//! consumes to render debugger status, breakpoint controls, and explicit cues when
//! debugging is unsupported or degraded. It reuses the closed vocabularies and
//! backing records already frozen in the [`crate::runtime_truth`] module and adds
//! the [`NotebookDebuggerSupportState`], [`BreakpointAffordance`], and
//! [`UnsupportedStateCue`] records so the chrome never implies debugger parity
//! silently and never hides unsupported states behind silence.
//!
//! The module exposes:
//!
//! - the [`NotebookDebuggerSupportState`] record that carries the composed debugger
//!   surface — support class, breakpoint affordances, unsupported cues, and an
//!   opaque reference to the underlying [`crate::DebuggerBridgeState`] — so the
//!   chrome can render truthfully without touching raw debug protocol state;
//! - the [`BreakpointAffordance`] record that carries a single breakpoint action,
//!   its posture, and the cell it applies to so the user always knows which
//!   actions are available and why;
//! - the [`UnsupportedStateCue`] record that carries an explicit cue class,
//!   tooltip, and action hint so the user never mistakes an unsupported debugger
//!   for a hidden limitation;
//! - the [`NotebookDebuggerSupportPacket`] checked-in artifact that downstream
//!   docs, help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookDebuggerSupportState`] payloads.
pub const NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND: &str = "notebook_debugger_support_state";

/// Stable record-kind tag for serialized [`BreakpointAffordance`] payloads.
pub const BREAKPOINT_AFFORDANCE_RECORD_KIND: &str = "notebook_breakpoint_affordance";

/// Stable record-kind tag for serialized [`UnsupportedStateCue`] payloads.
pub const UNSUPPORTED_STATE_CUE_RECORD_KIND: &str = "notebook_unsupported_state_cue";

/// Stable record-kind tag for the checked-in [`NotebookDebuggerSupportPacket`].
pub const NOTEBOOK_DEBUGGER_SUPPORT_PACKET_RECORD_KIND: &str = "notebook_debugger_support_packet";

/// Repo-relative path to the checked-in debugger-support packet JSON.
pub const NOTEBOOK_DEBUGGER_SUPPORT_PACKET_PATH: &str =
    "artifacts/notebook/m5/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues.json";

/// Embedded checked-in debugger-support packet JSON.
pub const NOTEBOOK_DEBUGGER_SUPPORT_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/implement_notebook_debugger_support_states_breakpoint_affordances_and_unsupported_state_cues.json"
));

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                #[serde(rename = $token)]
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// fixtures, and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Composed debugger-support state class projected onto the notebook chrome.
    /// Distinguishes idle, active, paused, stepping, disconnected, unsupported,
    /// partial, and degraded states so the chrome never invents ad hoc labels.
    DebuggerSupportStateClass {
        Idle => "idle",
        Paused => "paused",
        Stepping => "stepping",
        Running => "running",
        Disconnected => "disconnected",
        Unsupported => "unsupported",
        UnsupportedPartial => "unsupported_partial",
        Degraded => "degraded",
    }
);

impl DebuggerSupportStateClass {
    /// True for states that denote a live debugger session is present.
    pub const fn is_live_session(self) -> bool {
        matches!(
            self,
            Self::Idle | Self::Paused | Self::Stepping | Self::Running
        )
    }

    /// True for states that denote the debugger is not fully supported.
    pub const fn is_degraded_or_unsupported(self) -> bool {
        matches!(
            self,
            Self::Unsupported | Self::UnsupportedPartial | Self::Degraded | Self::Disconnected
        )
    }
}

closed_vocab!(
    /// Breakpoint affordance class. Pinned so the chrome never re-invents
    /// breakpoint actions that would confuse the user or silently fail.
    BreakpointAffordanceClass {
        SetBreakpoint => "set_breakpoint",
        RemoveBreakpoint => "remove_breakpoint",
        EnableBreakpoint => "enable_breakpoint",
        DisableBreakpoint => "disable_breakpoint",
        ClearAllBreakpoints => "clear_all_breakpoints",
        SetConditionalBreakpoint => "set_conditional_breakpoint",
    }
);

closed_vocab!(
    /// Posture of a breakpoint affordance. Distinguishes available from
    /// unavailable actions and names the reason so the user never mistakes a
    /// disabled action for a bug.
    BreakpointAffordancePostureClass {
        Available => "available",
        UnavailableNoKernel => "unavailable_no_kernel",
        UnavailableUnsupported => "unavailable_unsupported",
        UnavailablePolicyBlocked => "unavailable_policy_blocked",
        UnavailableRequiresReview => "unavailable_requires_review",
    }
);

closed_vocab!(
    /// Unsupported-state cue class. Every non-fully-supported debugger state
    /// MUST surface at least one of these cues so the user knows exactly why
    /// debugging is limited.
    UnsupportedStateCueClass {
        NoKernel => "no_kernel",
        AdapterUnavailable => "adapter_unavailable",
        KernelDoesNotImplementDebugProtocol => "kernel_does_not_implement_debug_protocol",
        RemoteParityUnverified => "remote_parity_unverified",
        PolicyBlocked => "policy_blocked",
        BridgeCancelledByRestart => "bridge_cancelled_by_restart",
        CellSteppingUnsupported => "cell_stepping_unsupported",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerSupportFinding {
    /// Stable check id (e.g. `notebook_debugger_support_state.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, state id, affordance id, cue id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl DebuggerSupportFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

/// Typed validation finding for a [`NotebookDebuggerSupportState`].
pub type NotebookDebuggerSupportStateFinding = DebuggerSupportFinding;

/// Typed validation finding for a [`BreakpointAffordance`].
pub type BreakpointAffordanceFinding = DebuggerSupportFinding;

/// Typed validation finding for an [`UnsupportedStateCue`].
pub type UnsupportedStateCueFinding = DebuggerSupportFinding;

/// Typed validation finding for a [`NotebookDebuggerSupportPacket`].
pub type NotebookDebuggerSupportPacketFinding = DebuggerSupportFinding;

/// Canonical notebook debugger-support state record. The composed UI surface
/// for the debugger panel: support class, breakpoint affordances,
/// unsupported-state cues, and an opaque reference to the underlying bridge
/// state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDebuggerSupportState {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_debugger_support_schema_version: u32,
    /// Stable opaque debugger-support state id.
    pub state_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque kernel-session id this state is attributed to; null when the
    /// debugger is unsupported due to no kernel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Composed debugger-support state class.
    pub debugger_support_state_class: DebuggerSupportStateClass,
    /// Opaque reference to the underlying [`crate::DebuggerBridgeState`] so
    /// the chrome can link to raw bridge details without importing them.
    pub underlying_bridge_state_ref: String,
    /// Breakpoint affordances rendered for this state.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub breakpoint_affordances: Vec<BreakpointAffordance>,
    /// Unsupported-state cues rendered when the debugger is not fully
    /// supported.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unsupported_state_cues: Vec<UnsupportedStateCue>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDebuggerSupportState {
    /// Returns typed truth-rule findings; an empty vector means the state is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookDebuggerSupportStateFinding> {
        let mut findings = Vec::new();
        let subject = self.state_id.as_str();

        if self.record_kind != NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND {
            findings.push(NotebookDebuggerSupportStateFinding::new(
                "notebook_debugger_support_state.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_DEBUGGER_SUPPORT_STATE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_debugger_support_schema_version != NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION
        {
            findings.push(NotebookDebuggerSupportStateFinding::new(
                "notebook_debugger_support_state.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION}, found {}",
                    self.notebook_debugger_support_schema_version
                ),
            ));
        }

        if self.underlying_bridge_state_ref.trim().is_empty() {
            findings.push(NotebookDebuggerSupportStateFinding::new(
                "notebook_debugger_support_state.underlying_bridge_state_ref_required",
                subject,
                "underlying_bridge_state_ref must be non-empty",
            ));
        }

        if self.debugger_support_state_class.is_live_session() {
            if self.kernel_session_id_ref.is_none() {
                findings.push(NotebookDebuggerSupportStateFinding::new(
                    "notebook_debugger_support_state.live_session_requires_kernel",
                    subject,
                    "live session states must carry a kernel_session_id_ref",
                ));
            }
        }

        if self
            .debugger_support_state_class
            .is_degraded_or_unsupported()
        {
            if self.unsupported_state_cues.is_empty() {
                findings.push(NotebookDebuggerSupportStateFinding::new(
                    "notebook_debugger_support_state.cues_required_for_degraded",
                    subject,
                    "degraded or unsupported states must surface at least one unsupported_state_cue",
                ));
            }
            let has_available_breakpoint = self
                .breakpoint_affordances
                .iter()
                .any(|a| matches!(a.posture_class, BreakpointAffordancePostureClass::Available));
            if has_available_breakpoint {
                findings.push(NotebookDebuggerSupportStateFinding::new(
                    "notebook_debugger_support_state.no_available_breakpoints_when_degraded",
                    subject,
                    "degraded or unsupported states must not expose available breakpoint affordances",
                ));
            }
        } else {
            if !self.unsupported_state_cues.is_empty() {
                findings.push(NotebookDebuggerSupportStateFinding::new(
                    "notebook_debugger_support_state.no_cues_when_supported",
                    subject,
                    "fully supported states must not carry unsupported_state_cues",
                ));
            }
        }

        for affordance in &self.breakpoint_affordances {
            findings.extend(affordance.validate().into_iter().map(|f| {
                NotebookDebuggerSupportStateFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        for cue in &self.unsupported_state_cues {
            findings.extend(cue.validate().into_iter().map(|f| {
                NotebookDebuggerSupportStateFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

/// Canonical breakpoint-affordance record. Carries a single breakpoint action,
/// its posture, and the cell it applies to so the user always knows what is
/// possible and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BreakpointAffordance {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_debugger_support_schema_version: u32,
    /// Stable opaque affordance id.
    pub affordance_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque cell id this affordance applies to; null when global.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_id_ref: Option<String>,
    /// Breakpoint affordance class.
    pub breakpoint_affordance_class: BreakpointAffordanceClass,
    /// Posture of this affordance.
    pub posture_class: BreakpointAffordancePostureClass,
    /// Export-safe summary line.
    pub summary: String,
}

impl BreakpointAffordance {
    /// Returns typed truth-rule findings; an empty vector means the affordance
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<BreakpointAffordanceFinding> {
        let mut findings = Vec::new();
        let subject = self.affordance_id.as_str();

        if self.record_kind != BREAKPOINT_AFFORDANCE_RECORD_KIND {
            findings.push(BreakpointAffordanceFinding::new(
                "breakpoint_affordance.record_kind",
                subject,
                format!(
                    "record_kind must be '{BREAKPOINT_AFFORDANCE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_debugger_support_schema_version != NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION
        {
            findings.push(BreakpointAffordanceFinding::new(
                "breakpoint_affordance.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION}, found {}",
                    self.notebook_debugger_support_schema_version
                ),
            ));
        }

        match self.breakpoint_affordance_class {
            BreakpointAffordanceClass::ClearAllBreakpoints => {
                if self.cell_id_ref.is_some() {
                    findings.push(BreakpointAffordanceFinding::new(
                        "breakpoint_affordance.global_no_cell_ref",
                        subject,
                        "clear_all_breakpoints must not carry a cell_id_ref",
                    ));
                }
            }
            _ => {
                // Per-cell affordances should ideally have a cell_id_ref, but
                // we allow null for notebook-global defaults.
            }
        }

        findings
    }
}

/// Canonical unsupported-state cue record. Carries an explicit cue class,
/// tooltip, and action hint so the user never mistakes an unsupported debugger
/// for a hidden limitation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnsupportedStateCue {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_debugger_support_schema_version: u32,
    /// Stable opaque cue id.
    pub cue_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Unsupported-state cue class.
    pub unsupported_state_cue_class: UnsupportedStateCueClass,
    /// Export-safe tooltip label rendered on hover.
    pub tooltip_label: String,
    /// Export-safe action-hint label rendered next to the cue.
    pub action_hint_label: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl UnsupportedStateCue {
    /// Returns typed truth-rule findings; an empty vector means the cue is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<UnsupportedStateCueFinding> {
        let mut findings = Vec::new();
        let subject = self.cue_id.as_str();

        if self.record_kind != UNSUPPORTED_STATE_CUE_RECORD_KIND {
            findings.push(UnsupportedStateCueFinding::new(
                "unsupported_state_cue.record_kind",
                subject,
                format!(
                    "record_kind must be '{UNSUPPORTED_STATE_CUE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_debugger_support_schema_version != NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION
        {
            findings.push(UnsupportedStateCueFinding::new(
                "unsupported_state_cue.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION}, found {}",
                    self.notebook_debugger_support_schema_version
                ),
            ));
        }

        if self.tooltip_label.trim().is_empty() {
            findings.push(UnsupportedStateCueFinding::new(
                "unsupported_state_cue.tooltip_label_required",
                subject,
                "tooltip_label must be non-empty",
            ));
        }
        if self.action_hint_label.trim().is_empty() {
            findings.push(UnsupportedStateCueFinding::new(
                "unsupported_state_cue.action_hint_label_required",
                subject,
                "action_hint_label must be non-empty",
            ));
        }

        findings
    }
}

/// Checked-in debugger-support packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDebuggerSupportPacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: debugger-support state classes.
    pub debugger_support_state_classes: Vec<DebuggerSupportStateClass>,
    /// Closed vocabulary: breakpoint affordance classes.
    pub breakpoint_affordance_classes: Vec<BreakpointAffordanceClass>,
    /// Closed vocabulary: breakpoint affordance posture classes.
    pub breakpoint_affordance_posture_classes: Vec<BreakpointAffordancePostureClass>,
    /// Closed vocabulary: unsupported-state cue classes.
    pub unsupported_state_cue_classes: Vec<UnsupportedStateCueClass>,
    /// Worked example debugger-support states.
    pub example_notebook_debugger_support_states: Vec<NotebookDebuggerSupportState>,
    /// Worked example breakpoint affordances.
    pub example_breakpoint_affordances: Vec<BreakpointAffordance>,
    /// Worked example unsupported-state cues.
    pub example_unsupported_state_cues: Vec<UnsupportedStateCue>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDebuggerSupportPacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookDebuggerSupportPacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION {
            findings.push(NotebookDebuggerSupportPacketFinding::new(
                "notebook_debugger_support_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DEBUGGER_SUPPORT_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_DEBUGGER_SUPPORT_PACKET_RECORD_KIND {
            findings.push(NotebookDebuggerSupportPacketFinding::new(
                "notebook_debugger_support_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_DEBUGGER_SUPPORT_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.debugger_support_state_classes.len() != DebuggerSupportStateClass::ALL.len() {
            findings.push(NotebookDebuggerSupportPacketFinding::new(
                "notebook_debugger_support_packet.state_classes_coverage",
                subject,
                "debugger_support_state_classes must list every variant",
            ));
        }
        if self.breakpoint_affordance_classes.len() != BreakpointAffordanceClass::ALL.len() {
            findings.push(NotebookDebuggerSupportPacketFinding::new(
                "notebook_debugger_support_packet.affordance_classes_coverage",
                subject,
                "breakpoint_affordance_classes must list every variant",
            ));
        }
        if self.breakpoint_affordance_posture_classes.len()
            != BreakpointAffordancePostureClass::ALL.len()
        {
            findings.push(NotebookDebuggerSupportPacketFinding::new(
                "notebook_debugger_support_packet.posture_classes_coverage",
                subject,
                "breakpoint_affordance_posture_classes must list every variant",
            ));
        }
        if self.unsupported_state_cue_classes.len() != UnsupportedStateCueClass::ALL.len() {
            findings.push(NotebookDebuggerSupportPacketFinding::new(
                "notebook_debugger_support_packet.cue_classes_coverage",
                subject,
                "unsupported_state_cue_classes must list every variant",
            ));
        }

        for state in &self.example_notebook_debugger_support_states {
            findings.extend(state.validate().into_iter().map(|f| {
                NotebookDebuggerSupportPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for affordance in &self.example_breakpoint_affordances {
            findings.extend(affordance.validate().into_iter().map(|f| {
                NotebookDebuggerSupportPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for cue in &self.example_unsupported_state_cues {
            findings.extend(cue.validate().into_iter().map(|f| {
                NotebookDebuggerSupportPacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl DebuggerSupportStateClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Idle,
        Self::Paused,
        Self::Stepping,
        Self::Running,
        Self::Disconnected,
        Self::Unsupported,
        Self::UnsupportedPartial,
        Self::Degraded,
    ];
}

impl BreakpointAffordanceClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SetBreakpoint,
        Self::RemoveBreakpoint,
        Self::EnableBreakpoint,
        Self::DisableBreakpoint,
        Self::ClearAllBreakpoints,
        Self::SetConditionalBreakpoint,
    ];
}

impl BreakpointAffordancePostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Available,
        Self::UnavailableNoKernel,
        Self::UnavailableUnsupported,
        Self::UnavailablePolicyBlocked,
        Self::UnavailableRequiresReview,
    ];
}

impl UnsupportedStateCueClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NoKernel,
        Self::AdapterUnavailable,
        Self::KernelDoesNotImplementDebugProtocol,
        Self::RemoteParityUnverified,
        Self::PolicyBlocked,
        Self::BridgeCancelledByRestart,
        Self::CellSteppingUnsupported,
    ];
}

/// Parses the checked-in debugger-support packet JSON.
pub fn current_notebook_debugger_support_packet(
) -> Result<NotebookDebuggerSupportPacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_DEBUGGER_SUPPORT_PACKET_JSON)
}

#[cfg(test)]
mod tests;
