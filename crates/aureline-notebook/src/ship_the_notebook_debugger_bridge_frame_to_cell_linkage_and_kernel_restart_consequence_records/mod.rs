//! Notebook debugger bridge, frame-to-cell linkage, and kernel restart
//! consequence records.
//!
//! This module materializes the composed debugger-bridge surface that makes
//! frame-to-cell relationships explicit and kernel-restart consequences
//! observable. It reuses the closed vocabularies and backing records already
//! frozen in the [`crate::runtime_truth`] module — specifically
//! [`DebuggerBridgeState`] and [`ReconnectReviewSheet`] — and adds the
//! [`DebuggerFrameCellLink`] and [`KernelRestartDebuggerConsequence`] records
//! so the chrome never silently drops frame context or hides what a restart
//! does to the debugger.
//!
//! The module exposes:
//!
//! - the [`DebuggerFrameCellLink`] record that carries an explicit mapping
//!   between a debugger frame and a notebook cell, including the link class,
//!   link posture, and available actions so the user always knows whether
//!   stepping in the current frame means stepping in the current cell;
//! - the [`KernelRestartDebuggerConsequence`] record that carries the typed
//!   consequence of a kernel restart on the debugger bridge — breakpoint
//!   retention, variable loss, reattach posture, and an explicit reconnect
//!   review ref — so the user never assumes the debugger survived a restart
//!   when it did not;
//! - the [`NotebookDebuggerBridgePacket`] checked-in artifact that downstream
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
pub const NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`DebuggerFrameCellLink`] payloads.
pub const DEBUGGER_FRAME_CELL_LINK_RECORD_KIND: &str = "notebook_debugger_frame_cell_link";

/// Stable record-kind tag for serialized [`KernelRestartDebuggerConsequence`]
/// payloads.
pub const KERNEL_RESTART_DEBUGGER_CONSEQUENCE_RECORD_KIND: &str =
    "notebook_kernel_restart_debugger_consequence";

/// Stable record-kind tag for the checked-in [`NotebookDebuggerBridgePacket`].
pub const NOTEBOOK_DEBUGGER_BRIDGE_PACKET_RECORD_KIND: &str = "notebook_debugger_bridge_packet";

/// Repo-relative path to the checked-in debugger-bridge packet JSON.
pub const NOTEBOOK_DEBUGGER_BRIDGE_PACKET_PATH: &str =
    "artifacts/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.json";

/// Embedded checked-in debugger-bridge packet JSON.
pub const NOTEBOOK_DEBUGGER_BRIDGE_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/ship_the_notebook_debugger_bridge_frame_to_cell_linkage_and_kernel_restart_consequence_records.json"
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
    /// Relationship class between a debugger frame and a notebook cell.
    /// Pins the answer to "does this frame belong to this cell?" to a
    /// closed vocabulary so the chrome never silently applies a heuristic
    /// without naming it.
    DebuggerFrameCellLinkClass {
        ExactCellMatch => "exact_cell_match",
        NearestCellHeuristic => "nearest_cell_heuristic",
        NoCellMapping => "no_cell_mapping",
        InCellLibraryCode => "in_cell_library_code",
        InCellExternalDependency => "in_cell_external_dependency",
        FrameStaleAfterRestart => "frame_stale_after_restart",
    }
);

closed_vocab!(
    /// Posture of a frame-to-cell link. Distinguishes actionable stepping
    /// from view-only, stale, and unsupported cases so the user never
    /// mistakes a display-only frame for an actionable one.
    DebuggerFrameCellLinkPostureClass {
        ActionableStepIntoCell => "actionable_step_into_cell",
        ActionableStepOverCell => "actionable_step_over_cell",
        ViewOnlyNoStep => "view_only_no_step",
        StaleReinitializeRequired => "stale_reinitialize_required",
        UnsupportedNoSourceMap => "unsupported_no_source_map",
    }
);

closed_vocab!(
    /// Why the kernel restart or reconnect occurred. Every consequence
    /// record cites one of these values so the chrome never shows a generic
    /// "something happened" message.
    KernelRestartKind {
        UserInitiatedRestart => "user_initiated_restart",
        TransportLostReconnectAttempted => "transport_lost_reconnect_attempted",
        TrustDowngradeCancelsInFlight => "trust_downgrade_cancels_in_flight",
        PolicyDeniesContinuedExecution => "policy_denies_continued_execution",
        ManagedWorkspaceLifecycleChange => "managed_workspace_lifecycle_change",
        BridgeCancelledByRestart => "bridge_cancelled_by_restart",
    }
);

closed_vocab!(
    /// Consequence class for the debugger bridge after a kernel restart.
    /// Distinguishes preserved, reset, cancelled, and unavailable states so
    /// the user knows exactly what happened to breakpoints and stepping.
    KernelRestartConsequenceClass {
        BridgePreservedSameSession => "bridge_preserved_same_session",
        BridgeResetFreshSession => "bridge_reset_fresh_session",
        BridgeCancelledPendingReconnect => "bridge_cancelled_pending_reconnect",
        BridgeUnavailableNoKernel => "bridge_unavailable_no_kernel",
        BreakpointsRetainedAcrossRestart => "breakpoints_retained_across_restart",
        BreakpointsLostOnRestart => "breakpoints_lost_on_restart",
        VariableStateLost => "variable_state_lost",
        ExecutionQueueCleared => "execution_queue_cleared",
    }
);

closed_vocab!(
    /// Action class describing how the debugger bridge should reattach after
    /// a kernel restart. Distinguishes automatic, on-demand, unavailable, and
    /// review-required paths so the user never waits for an automatic reattach
    /// that will not happen.
    KernelRestartDebuggerActionClass {
        ReattachAutomatically => "reattach_automatically",
        ReattachOnDemand => "reattach_on_demand",
        ReattachUnavailable => "reattach_unavailable",
        ReattachRequiresReview => "reattach_requires_review",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerBridgeLinkRestartFinding {
    /// Stable check id (e.g. `debugger_frame_cell_link.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, link id, consequence id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl DebuggerBridgeLinkRestartFinding {
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

/// Typed validation finding for a [`DebuggerFrameCellLink`].
pub type DebuggerFrameCellLinkFinding = DebuggerBridgeLinkRestartFinding;

/// Typed validation finding for a [`KernelRestartDebuggerConsequence`].
pub type KernelRestartDebuggerConsequenceFinding = DebuggerBridgeLinkRestartFinding;

/// Typed validation finding for a [`NotebookDebuggerBridgePacket`].
pub type NotebookDebuggerBridgePacketFinding = DebuggerBridgeLinkRestartFinding;

/// Canonical debugger frame-to-cell link record. Carries an explicit mapping
/// between a debugger frame and a notebook cell so the chrome never silently
/// applies an unnamed heuristic.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerFrameCellLink {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_debugger_bridge_schema_version: u32,
    /// Stable opaque link id.
    pub link_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque kernel-session id this link is attributed to.
    pub kernel_session_id_ref: String,
    /// Opaque debugger frame id.
    pub frame_ref: String,
    /// Opaque cell id this frame is linked to; null when the link class is
    /// `no_cell_mapping`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_id_ref: Option<String>,
    /// Relationship class between the frame and the cell.
    pub link_class: DebuggerFrameCellLinkClass,
    /// Posture of this link.
    pub link_posture_class: DebuggerFrameCellLinkPostureClass,
    /// Opaque source-line descriptor ref; null when unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_line_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl DebuggerFrameCellLink {
    /// Returns typed truth-rule findings; an empty vector means the link is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<DebuggerFrameCellLinkFinding> {
        let mut findings = Vec::new();
        let subject = self.link_id.as_str();

        if self.record_kind != DEBUGGER_FRAME_CELL_LINK_RECORD_KIND {
            findings.push(DebuggerFrameCellLinkFinding::new(
                "debugger_frame_cell_link.record_kind",
                subject,
                format!(
                    "record_kind must be '{DEBUGGER_FRAME_CELL_LINK_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_debugger_bridge_schema_version != NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION {
            findings.push(DebuggerFrameCellLinkFinding::new(
                "debugger_frame_cell_link.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION}, found {}",
                    self.notebook_debugger_bridge_schema_version
                ),
            ));
        }

        if self.frame_ref.trim().is_empty() {
            findings.push(DebuggerFrameCellLinkFinding::new(
                "debugger_frame_cell_link.frame_ref_required",
                subject,
                "frame_ref must be non-empty",
            ));
        }

        match self.link_class {
            DebuggerFrameCellLinkClass::NoCellMapping => {
                if self.cell_id_ref.is_some() {
                    findings.push(DebuggerFrameCellLinkFinding::new(
                        "debugger_frame_cell_link.no_cell_mapping_no_cell_ref",
                        subject,
                        "no_cell_mapping must not carry a cell_id_ref",
                    ));
                }
            }
            _ => {
                if self.cell_id_ref.is_none() {
                    findings.push(DebuggerFrameCellLinkFinding::new(
                        "debugger_frame_cell_link.cell_ref_required",
                        subject,
                        "link classes other than no_cell_mapping must carry a cell_id_ref",
                    ));
                }
            }
        }

        if matches!(
            self.link_posture_class,
            DebuggerFrameCellLinkPostureClass::StaleReinitializeRequired
        ) && !matches!(self.link_class, DebuggerFrameCellLinkClass::FrameStaleAfterRestart)
        {
            findings.push(DebuggerFrameCellLinkFinding::new(
                "debugger_frame_cell_link.stale_requires_stale_class",
                subject,
                "stale_reinitialize_required posture requires frame_stale_after_restart link class",
            ));
        }

        findings
    }
}

/// Canonical kernel-restart debugger-consequence record. Carries the typed
/// consequence of a kernel restart on the debugger bridge so the user always
/// knows whether breakpoints survived, whether variable state was lost, and
/// how reattachment works on the other side.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelRestartDebuggerConsequence {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_debugger_bridge_schema_version: u32,
    /// Stable opaque consequence id.
    pub consequence_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque prior kernel-session id, when one existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_kernel_session_id_ref: Option<String>,
    /// Opaque next kernel-session id, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_kernel_session_id_ref: Option<String>,
    /// Why the restart or reconnect occurred.
    pub restart_kind: KernelRestartKind,
    /// Consequence class for the debugger bridge.
    pub consequence_class: KernelRestartConsequenceClass,
    /// Whether in-flight debug sessions are cancelled by this event.
    pub in_flight_debug_cancelled: bool,
    /// Number of active breakpoints affected by this event.
    pub breakpoints_affected: u32,
    /// Action class describing how the debugger bridge should reattach.
    pub reattach_action_class: KernelRestartDebuggerActionClass,
    /// Opaque reconnect-review sheet ref, when one has been generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconnect_review_sheet_ref: Option<String>,
    /// Whether auto-rerun is forbidden on the other side of this event.
    /// MUST be `true`; the field exists to make the invariant explicit.
    pub auto_rerun_forbidden: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl KernelRestartDebuggerConsequence {
    /// Returns typed truth-rule findings; an empty vector means the consequence
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<KernelRestartDebuggerConsequenceFinding> {
        let mut findings = Vec::new();
        let subject = self.consequence_id.as_str();

        if self.record_kind != KERNEL_RESTART_DEBUGGER_CONSEQUENCE_RECORD_KIND {
            findings.push(KernelRestartDebuggerConsequenceFinding::new(
                "kernel_restart_debugger_consequence.record_kind",
                subject,
                format!(
                    "record_kind must be '{KERNEL_RESTART_DEBUGGER_CONSEQUENCE_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_debugger_bridge_schema_version != NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION {
            findings.push(KernelRestartDebuggerConsequenceFinding::new(
                "kernel_restart_debugger_consequence.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION}, found {}",
                    self.notebook_debugger_bridge_schema_version
                ),
            ));
        }

        if !self.auto_rerun_forbidden {
            findings.push(KernelRestartDebuggerConsequenceFinding::new(
                "kernel_restart_debugger_consequence.auto_rerun_forbidden",
                subject,
                "auto_rerun_forbidden must be true on every kernel-restart consequence record",
            ));
        }

        match self.consequence_class {
            KernelRestartConsequenceClass::BridgePreservedSameSession => {
                if self.in_flight_debug_cancelled {
                    findings.push(KernelRestartDebuggerConsequenceFinding::new(
                        "kernel_restart_debugger_consequence.preserved_no_cancel",
                        subject,
                        "bridge_preserved_same_session must not cancel in-flight debug sessions",
                    ));
                }
            }
            KernelRestartConsequenceClass::BridgeResetFreshSession
            | KernelRestartConsequenceClass::BridgeCancelledPendingReconnect
            | KernelRestartConsequenceClass::BridgeUnavailableNoKernel
            | KernelRestartConsequenceClass::BreakpointsLostOnRestart
            | KernelRestartConsequenceClass::VariableStateLost
            | KernelRestartConsequenceClass::ExecutionQueueCleared => {
                if !self.in_flight_debug_cancelled {
                    findings.push(KernelRestartDebuggerConsequenceFinding::new(
                        "kernel_restart_debugger_consequence.cancel_required",
                        subject,
                        "this consequence class must cancel in-flight debug sessions",
                    ));
                }
            }
            KernelRestartConsequenceClass::BreakpointsRetainedAcrossRestart => {
                // Breakpoints may be retained while other state is lost;
                // in_flight_debug_cancelled is independent.
            }
        }

        match self.reattach_action_class {
            KernelRestartDebuggerActionClass::ReattachUnavailable => {
                if self.next_kernel_session_id_ref.is_some() {
                    findings.push(KernelRestartDebuggerConsequenceFinding::new(
                        "kernel_restart_debugger_consequence.unavailable_no_next_session",
                        subject,
                        "reattach_unavailable must not carry a next_kernel_session_id_ref",
                    ));
                }
            }
            _ => {}
        }

        if matches!(
            self.restart_kind,
            KernelRestartKind::TrustDowngradeCancelsInFlight
                | KernelRestartKind::PolicyDeniesContinuedExecution
                | KernelRestartKind::BridgeCancelledByRestart
        ) && !matches!(
            self.reattach_action_class,
            KernelRestartDebuggerActionClass::ReattachRequiresReview
                | KernelRestartDebuggerActionClass::ReattachUnavailable
        ) {
            findings.push(KernelRestartDebuggerConsequenceFinding::new(
                "kernel_restart_debugger_consequence.restart_kind_requires_review_or_unavailable",
                subject,
                "this restart kind requires reattach_requires_review or reattach_unavailable",
            ));
        }

        findings
    }
}

/// Checked-in debugger-bridge packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookDebuggerBridgePacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: frame-to-cell link classes.
    pub debugger_frame_cell_link_classes: Vec<DebuggerFrameCellLinkClass>,
    /// Closed vocabulary: frame-to-cell link posture classes.
    pub debugger_frame_cell_link_posture_classes: Vec<DebuggerFrameCellLinkPostureClass>,
    /// Closed vocabulary: kernel restart kinds.
    pub kernel_restart_kinds: Vec<KernelRestartKind>,
    /// Closed vocabulary: kernel restart consequence classes.
    pub kernel_restart_consequence_classes: Vec<KernelRestartConsequenceClass>,
    /// Closed vocabulary: kernel restart debugger action classes.
    pub kernel_restart_debugger_action_classes: Vec<KernelRestartDebuggerActionClass>,
    /// Worked example frame-to-cell links.
    pub example_debugger_frame_cell_links: Vec<DebuggerFrameCellLink>,
    /// Worked example kernel-restart debugger consequences.
    pub example_kernel_restart_debugger_consequences: Vec<KernelRestartDebuggerConsequence>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookDebuggerBridgePacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookDebuggerBridgePacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION {
            findings.push(NotebookDebuggerBridgePacketFinding::new(
                "notebook_debugger_bridge_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_DEBUGGER_BRIDGE_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_DEBUGGER_BRIDGE_PACKET_RECORD_KIND {
            findings.push(NotebookDebuggerBridgePacketFinding::new(
                "notebook_debugger_bridge_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_DEBUGGER_BRIDGE_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.debugger_frame_cell_link_classes.len() != DebuggerFrameCellLinkClass::ALL.len() {
            findings.push(NotebookDebuggerBridgePacketFinding::new(
                "notebook_debugger_bridge_packet.link_classes_coverage",
                subject,
                "debugger_frame_cell_link_classes must list every variant",
            ));
        }
        if self.debugger_frame_cell_link_posture_classes.len()
            != DebuggerFrameCellLinkPostureClass::ALL.len()
        {
            findings.push(NotebookDebuggerBridgePacketFinding::new(
                "notebook_debugger_bridge_packet.posture_classes_coverage",
                subject,
                "debugger_frame_cell_link_posture_classes must list every variant",
            ));
        }
        if self.kernel_restart_kinds.len() != KernelRestartKind::ALL.len() {
            findings.push(NotebookDebuggerBridgePacketFinding::new(
                "notebook_debugger_bridge_packet.restart_kinds_coverage",
                subject,
                "kernel_restart_kinds must list every variant",
            ));
        }
        if self.kernel_restart_consequence_classes.len() != KernelRestartConsequenceClass::ALL.len()
        {
            findings.push(NotebookDebuggerBridgePacketFinding::new(
                "notebook_debugger_bridge_packet.consequence_classes_coverage",
                subject,
                "kernel_restart_consequence_classes must list every variant",
            ));
        }
        if self.kernel_restart_debugger_action_classes.len()
            != KernelRestartDebuggerActionClass::ALL.len()
        {
            findings.push(NotebookDebuggerBridgePacketFinding::new(
                "notebook_debugger_bridge_packet.action_classes_coverage",
                subject,
                "kernel_restart_debugger_action_classes must list every variant",
            ));
        }

        for link in &self.example_debugger_frame_cell_links {
            findings.extend(link.validate().into_iter().map(|f| {
                NotebookDebuggerBridgePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for consequence in &self.example_kernel_restart_debugger_consequences {
            findings.extend(consequence.validate().into_iter().map(|f| {
                NotebookDebuggerBridgePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl DebuggerFrameCellLinkClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExactCellMatch,
        Self::NearestCellHeuristic,
        Self::NoCellMapping,
        Self::InCellLibraryCode,
        Self::InCellExternalDependency,
        Self::FrameStaleAfterRestart,
    ];
}

impl DebuggerFrameCellLinkPostureClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ActionableStepIntoCell,
        Self::ActionableStepOverCell,
        Self::ViewOnlyNoStep,
        Self::StaleReinitializeRequired,
        Self::UnsupportedNoSourceMap,
    ];
}

impl KernelRestartKind {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::UserInitiatedRestart,
        Self::TransportLostReconnectAttempted,
        Self::TrustDowngradeCancelsInFlight,
        Self::PolicyDeniesContinuedExecution,
        Self::ManagedWorkspaceLifecycleChange,
        Self::BridgeCancelledByRestart,
    ];
}

impl KernelRestartConsequenceClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BridgePreservedSameSession,
        Self::BridgeResetFreshSession,
        Self::BridgeCancelledPendingReconnect,
        Self::BridgeUnavailableNoKernel,
        Self::BreakpointsRetainedAcrossRestart,
        Self::BreakpointsLostOnRestart,
        Self::VariableStateLost,
        Self::ExecutionQueueCleared,
    ];
}

impl KernelRestartDebuggerActionClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReattachAutomatically,
        Self::ReattachOnDemand,
        Self::ReattachUnavailable,
        Self::ReattachRequiresReview,
    ];
}

/// Parses the checked-in debugger-bridge packet JSON.
pub fn current_notebook_debugger_bridge_packet() -> Result<NotebookDebuggerBridgePacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_DEBUGGER_BRIDGE_PACKET_JSON)
}

#[cfg(test)]
mod tests;
