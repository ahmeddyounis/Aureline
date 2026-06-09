//! Notebook cell chrome, run-scope controls, and durable execution-state rows.
//!
//! This module materializes the composed cell-level UI surface that the notebook
//! chrome consumes to render per-cell chrome, run-scope selectors, and durable
//! execution-state rows. It reuses the closed vocabularies and backing records
//! already frozen in the [`crate::runtime_truth`] module and adds the
//! [`NotebookCellChrome`], [`RunScopeControl`], and [`DurableExecutionStateRow`]
//! records so the cell-level surface is honest about status, available actions,
//! run scope, and execution history without requiring a live kernel.
//!
//! The module exposes:
//!
//! - the [`NotebookCellChrome`] record that carries execution badge, status
//!   class, run-scope control, output trust, available actions, and chrome
//!   visibility state;
//! - the [`RunScopeControl`] record that carries current scope, available
//!   scopes, changeability, and lock reason so the user never mistakes a locked
//!   scope for a free choice;
//! - the [`DurableExecutionStateRow`] record that projects the latest
//!   cell-execution detail into a kernel-surviving row the chrome can render
//!   when no live session is present;
//! - the [`NotebookCellChromePacket`] checked-in artifact that downstream docs,
//!   help, support, and CI surfaces ingest instead of cloning status text.
//!
//! Raw notebook JSON bodies, raw cell source bytes, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, and raw URLs MUST NOT
//! appear on any record carried here. Only opaque handles and closed-
//! vocabulary tokens cross the boundary.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every record carried by this module. Bumped only
/// on breaking payload changes; additive-optional fields do not bump this
/// value.
pub const NOTEBOOK_CELL_CHROME_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`NotebookCellChrome`] payloads.
pub const NOTEBOOK_CELL_CHROME_RECORD_KIND: &str = "notebook_cell_chrome";

/// Stable record-kind tag for serialized [`RunScopeControl`] payloads.
pub const RUN_SCOPE_CONTROL_RECORD_KIND: &str = "notebook_run_scope_control";

/// Stable record-kind tag for serialized [`DurableExecutionStateRow`] payloads.
pub const DURABLE_EXECUTION_STATE_ROW_RECORD_KIND: &str = "notebook_durable_execution_state_row";

/// Stable record-kind tag for the checked-in [`NotebookCellChromePacket`].
pub const NOTEBOOK_CELL_CHROME_PACKET_RECORD_KIND: &str = "notebook_cell_chrome_packet";

/// Repo-relative path to the checked-in cell-chrome packet JSON.
pub const NOTEBOOK_CELL_CHROME_PACKET_PATH: &str =
    "artifacts/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.json";

/// Embedded checked-in cell-chrome packet JSON.
pub const NOTEBOOK_CELL_CHROME_PACKET_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/notebook/m5/ship_notebook_cell_chrome_run_scope_controls_and_durable_execution_state_rows.json"
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
    /// Cell-chrome status class. Distinguishes idle, queued, executing,
    /// succeeded, errored, interrupted, cancelled, stale-output, and no-kernel
    /// states so the chrome never implies a dead kernel is active through
    /// silence.
    CellChromeStatusClass {
        Idle => "idle",
        Queued => "queued",
        Executing => "executing",
        Succeeded => "succeeded",
        Errored => "errored",
        Interrupted => "interrupted",
        Cancelled => "cancelled",
        StaleOutput => "stale_output",
        NoKernel => "no_kernel",
    }
);

impl CellChromeStatusClass {
    /// True for statuses that denote an active or pending execution.
    pub const fn is_active_or_pending(self) -> bool {
        matches!(self, Self::Queued | Self::Executing)
    }

    /// True for terminal statuses that do not admit a re-run without an
    /// explicit user action.
    pub const fn is_terminal(self) -> bool {
        matches!(
            self,
            Self::Succeeded | Self::Errored | Self::Interrupted | Self::Cancelled | Self::StaleOutput
        )
    }

    /// True when no kernel is available for this cell.
    pub const fn is_no_kernel(self) -> bool {
        matches!(self, Self::NoKernel)
    }
}

closed_vocab!(
    /// Actions exposed on the cell chrome. Pinned so the chrome never
    /// re-invents run/debug/export actions that would broaden capture or
    /// confuse the user.
    CellChromeActionClass {
        RunCell => "run_cell",
        RunCellAndAdvance => "run_cell_and_advance",
        RunAllAbove => "run_all_above",
        RunAllBelow => "run_all_below",
        ClearOutput => "clear_output",
        ToggleCollapseOutput => "toggle_collapse_output",
        ToggleFoldSource => "toggle_fold_source",
        DebugCell => "debug_cell",
        ExportOutput => "export_output",
    }
);

closed_vocab!(
    /// Reason the run-scope control is locked. Pinned so the user never
    /// mistakes a policy-locked scope for a free choice.
    RunScopeControlLockReasonClass {
        NotLocked => "not_locked",
        LockedByPolicy => "locked_by_policy",
        LockedDuringExecution => "locked_during_execution",
        LockedReplayOnlyEnvironment => "locked_replay_only_environment",
        LockedNoKernel => "locked_no_kernel",
    }
);

/// Generic finding shape used by every record validator in this module.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellChromeFinding {
    /// Stable check id (e.g. `notebook_cell_chrome.record_kind`).
    pub check_id: String,
    /// Subject row id (record id, chrome id, cell id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl CellChromeFinding {
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

/// Typed validation finding for a [`NotebookCellChrome`].
pub type NotebookCellChromeFinding = CellChromeFinding;

/// Typed validation finding for a [`RunScopeControl`].
pub type RunScopeControlFinding = CellChromeFinding;

/// Typed validation finding for a [`DurableExecutionStateRow`].
pub type DurableExecutionStateRowFinding = CellChromeFinding;

/// Typed validation finding for a [`NotebookCellChromePacket`].
pub type NotebookCellChromePacketFinding = CellChromeFinding;

/// Canonical notebook cell-chrome record. The composed UI surface for a
/// single cell: execution badge, status, run-scope control, output trust,
/// available actions, and chrome visibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCellChrome {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_cell_chrome_schema_version: u32,
    /// Stable opaque chrome-state id.
    pub chrome_state_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque cell id (stable across save/diff/merge).
    pub cell_id_ref: String,
    /// 0-based display index of the cell in the document.
    pub cell_display_index: u32,
    /// Execution badge label rendered in the chrome (e.g. `[1]`, `[*]`,
    /// `[ ]`).
    pub execution_badge_label: String,
    /// Cell-chrome status class.
    pub cell_status_class: CellChromeStatusClass,
    /// Run-scope control for this cell.
    pub run_scope_control: RunScopeControl,
    /// Output-trust class projected for the cell's output.
    pub output_trust_class: crate::OutputTrustClass,
    /// Actions exposed on the cell chrome.
    pub available_actions: Vec<CellChromeActionClass>,
    /// Whether the cell output is collapsed.
    #[serde(default)]
    pub collapsed: bool,
    /// Whether the cell is selected.
    #[serde(default)]
    pub selected: bool,
    /// Whether the cell is focused.
    #[serde(default)]
    pub focused: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCellChrome {
    /// Returns typed truth-rule findings; an empty vector means the chrome
    /// state is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<NotebookCellChromeFinding> {
        let mut findings = Vec::new();
        let subject = self.chrome_state_id.as_str();

        if self.record_kind != NOTEBOOK_CELL_CHROME_RECORD_KIND {
            findings.push(NotebookCellChromeFinding::new(
                "notebook_cell_chrome.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_CELL_CHROME_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_cell_chrome_schema_version != NOTEBOOK_CELL_CHROME_SCHEMA_VERSION {
            findings.push(NotebookCellChromeFinding::new(
                "notebook_cell_chrome.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_CELL_CHROME_SCHEMA_VERSION}, found {}",
                    self.notebook_cell_chrome_schema_version
                ),
            ));
        }

        if self.execution_badge_label.trim().is_empty() {
            findings.push(NotebookCellChromeFinding::new(
                "notebook_cell_chrome.execution_badge_label_required",
                subject,
                "execution_badge_label must be non-empty",
            ));
        }

        if self.cell_status_class.is_no_kernel() {
            let has_run = self
                .available_actions
                .iter()
                .any(|action| matches!(action, CellChromeActionClass::RunCell | CellChromeActionClass::RunCellAndAdvance));
            if has_run {
                findings.push(NotebookCellChromeFinding::new(
                    "notebook_cell_chrome.no_kernel_run_actions",
                    subject,
                    "no_kernel status must not expose run_cell or run_cell_and_advance",
                ));
            }
            let has_debug = self
                .available_actions
                .iter()
                .any(|action| matches!(action, CellChromeActionClass::DebugCell));
            if has_debug {
                findings.push(NotebookCellChromeFinding::new(
                    "notebook_cell_chrome.no_kernel_debug_action",
                    subject,
                    "no_kernel status must not expose debug_cell",
                ));
            }
        }

        if self.cell_status_class.is_active_or_pending() {
            let has_run = self
                .available_actions
                .iter()
                .any(|action| matches!(action, CellChromeActionClass::RunCell | CellChromeActionClass::RunCellAndAdvance));
            if has_run {
                findings.push(NotebookCellChromeFinding::new(
                    "notebook_cell_chrome.active_pending_no_rerun",
                    subject,
                    "queued or executing status must not expose run_cell or run_cell_and_advance",
                ));
            }
        }

        findings.extend(self.run_scope_control.validate().into_iter().map(|f| {
            NotebookCellChromeFinding::new(&f.check_id, &f.subject_ref, &f.message)
        }));

        findings
    }
}

/// Canonical run-scope control record. Tells the user what run scope is in
/// force, what scopes are available, and whether the scope can be changed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunScopeControl {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_cell_chrome_schema_version: u32,
    /// Stable opaque control id.
    pub control_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque cell id when this control is per-cell; null when global.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cell_id_ref: Option<String>,
    /// Current run scope.
    pub current_scope: crate::CellExecutionRunScope,
    /// Available run scopes the user may choose from.
    pub available_scopes: Vec<crate::CellExecutionRunScope>,
    /// Whether the user may change the current scope.
    pub scope_changeable: bool,
    /// Why the scope is locked, if it is locked.
    pub lock_reason_class: RunScopeControlLockReasonClass,
    /// Export-safe summary line.
    pub summary: String,
}

impl RunScopeControl {
    /// Returns typed truth-rule findings; an empty vector means the control
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<RunScopeControlFinding> {
        let mut findings = Vec::new();
        let subject = self.control_id.as_str();

        if self.record_kind != RUN_SCOPE_CONTROL_RECORD_KIND {
            findings.push(RunScopeControlFinding::new(
                "run_scope_control.record_kind",
                subject,
                format!(
                    "record_kind must be '{RUN_SCOPE_CONTROL_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_cell_chrome_schema_version != NOTEBOOK_CELL_CHROME_SCHEMA_VERSION {
            findings.push(RunScopeControlFinding::new(
                "run_scope_control.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_CELL_CHROME_SCHEMA_VERSION}, found {}",
                    self.notebook_cell_chrome_schema_version
                ),
            ));
        }

        if !self.available_scopes.contains(&self.current_scope) {
            findings.push(RunScopeControlFinding::new(
                "run_scope_control.current_scope_available",
                subject,
                "current_scope must be present in available_scopes",
            ));
        }

        if self.scope_changeable {
            if !matches!(self.lock_reason_class, RunScopeControlLockReasonClass::NotLocked) {
                findings.push(RunScopeControlFinding::new(
                    "run_scope_control.changeable_not_locked",
                    subject,
                    "changeable scope must cite lock_reason=not_locked",
                ));
            }
        } else {
            if matches!(self.lock_reason_class, RunScopeControlLockReasonClass::NotLocked) {
                findings.push(RunScopeControlFinding::new(
                    "run_scope_control.locked_reason_required",
                    subject,
                    "non-changeable scope must cite a non-not_locked lock_reason",
                ));
            }
        }

        if matches!(self.current_scope, crate::CellExecutionRunScope::QueuedNotYetStarted)
            && self.scope_changeable
        {
            findings.push(RunScopeControlFinding::new(
                "run_scope_control.queued_not_changeable",
                subject,
                "queued_not_yet_started scope must not be changeable",
            ));
        }

        if self.available_scopes.is_empty() {
            findings.push(RunScopeControlFinding::new(
                "run_scope_control.available_scopes_required",
                subject,
                "available_scopes must contain at least one scope",
            ));
        }

        findings
    }
}

/// Canonical durable execution-state row record. Projects the latest cell
/// execution into a kernel-surviving row the chrome can render when no live
/// session is present. Carries opaque refs only — no raw output bytes, no raw
/// source, no raw timestamps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableExecutionStateRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_cell_chrome_schema_version: u32,
    /// Stable opaque row id.
    pub row_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque cell id.
    pub cell_id_ref: String,
    /// 0-based display index of the cell in the document.
    pub cell_display_index: u32,
    /// Opaque ref to the latest [`CellExecutionDetailRow`] this row projects.
    pub latest_execution_detail_row_ref: String,
    /// Durable outcome class.
    pub durable_outcome_class: crate::CellExecutionOutcomeClass,
    /// Durable run scope.
    pub durable_run_scope: crate::CellExecutionRunScope,
    /// Number of output blocks attributed to the latest execution.
    pub output_count: u32,
    /// Whether the output is stale relative to the current kernel session.
    pub stale_output: bool,
    /// Stale reason when `stale_output` is true; MUST be `None` otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_reason_class: Option<crate::OutputTrustStaleReasonClass>,
    /// Export-safe summary line.
    pub summary: String,
}

impl DurableExecutionStateRow {
    /// Returns typed truth-rule findings; an empty vector means the row is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<DurableExecutionStateRowFinding> {
        let mut findings = Vec::new();
        let subject = self.row_id.as_str();

        if self.record_kind != DURABLE_EXECUTION_STATE_ROW_RECORD_KIND {
            findings.push(DurableExecutionStateRowFinding::new(
                "durable_execution_state_row.record_kind",
                subject,
                format!(
                    "record_kind must be '{DURABLE_EXECUTION_STATE_ROW_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }
        if self.notebook_cell_chrome_schema_version != NOTEBOOK_CELL_CHROME_SCHEMA_VERSION {
            findings.push(DurableExecutionStateRowFinding::new(
                "durable_execution_state_row.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_CELL_CHROME_SCHEMA_VERSION}, found {}",
                    self.notebook_cell_chrome_schema_version
                ),
            ));
        }

        if self.stale_output {
            if self.stale_reason_class.is_none() {
                findings.push(DurableExecutionStateRowFinding::new(
                    "durable_execution_state_row.stale_reason_required",
                    subject,
                    "stale_output=true requires a stale_reason_class",
                ));
            }
        } else if self.stale_reason_class.is_some() {
            findings.push(DurableExecutionStateRowFinding::new(
                "durable_execution_state_row.stale_reason_not_allowed",
                subject,
                "stale_output=false must not carry a stale_reason_class",
            ));
        }

        match self.durable_outcome_class {
            crate::CellExecutionOutcomeClass::SkippedNoKernel
            | crate::CellExecutionOutcomeClass::SkippedByPolicy => {
                if self.output_count != 0 {
                    findings.push(DurableExecutionStateRowFinding::new(
                        "durable_execution_state_row.skipped_output_count",
                        subject,
                        "skipped outcomes must report output_count=0",
                    ));
                }
            }
            crate::CellExecutionOutcomeClass::Queued => {
                if self.output_count != 0 {
                    findings.push(DurableExecutionStateRowFinding::new(
                        "durable_execution_state_row.queued_output_count",
                        subject,
                        "queued outcome must report output_count=0",
                    ));
                }
            }
            _ => {}
        }

        if matches!(
            self.durable_run_scope,
            crate::CellExecutionRunScope::QueuedNotYetStarted
        ) && self.durable_outcome_class != crate::CellExecutionOutcomeClass::Queued
        {
            findings.push(DurableExecutionStateRowFinding::new(
                "durable_execution_state_row.queued_scope_outcome",
                subject,
                "queued_not_yet_started scope must report outcome_class=queued",
            ));
        }

        findings
    }
}

/// Checked-in cell-chrome packet that downstream surfaces ingest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookCellChromePacket {
    /// Schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC date this packet is current as of.
    pub as_of: String,
    /// Closed vocabulary: cell-chrome status classes.
    pub cell_chrome_status_classes: Vec<CellChromeStatusClass>,
    /// Closed vocabulary: cell-chrome action classes.
    pub cell_chrome_action_classes: Vec<CellChromeActionClass>,
    /// Closed vocabulary: run-scope control lock-reason classes.
    pub run_scope_control_lock_reason_classes: Vec<RunScopeControlLockReasonClass>,
    /// Worked example cell-chrome states.
    pub example_cell_chromes: Vec<NotebookCellChrome>,
    /// Worked example run-scope controls.
    pub example_run_scope_controls: Vec<RunScopeControl>,
    /// Worked example durable execution-state rows.
    pub example_durable_execution_state_rows: Vec<DurableExecutionStateRow>,
    /// Export-safe summary line.
    pub summary: String,
}

impl NotebookCellChromePacket {
    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<NotebookCellChromePacketFinding> {
        let mut findings = Vec::new();
        let subject = self.packet_id.as_str();

        if self.schema_version != NOTEBOOK_CELL_CHROME_SCHEMA_VERSION {
            findings.push(NotebookCellChromePacketFinding::new(
                "notebook_cell_chrome_packet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_CELL_CHROME_SCHEMA_VERSION}, found {}",
                    self.schema_version
                ),
            ));
        }
        if self.record_kind != NOTEBOOK_CELL_CHROME_PACKET_RECORD_KIND {
            findings.push(NotebookCellChromePacketFinding::new(
                "notebook_cell_chrome_packet.record_kind",
                subject,
                format!(
                    "record_kind must be '{NOTEBOOK_CELL_CHROME_PACKET_RECORD_KIND}', found '{}'",
                    self.record_kind
                ),
            ));
        }

        if self.cell_chrome_status_classes.len() != CellChromeStatusClass::ALL.len() {
            findings.push(NotebookCellChromePacketFinding::new(
                "notebook_cell_chrome_packet.status_classes_coverage",
                subject,
                "cell_chrome_status_classes must list every variant",
            ));
        }
        if self.cell_chrome_action_classes.len() != CellChromeActionClass::ALL.len() {
            findings.push(NotebookCellChromePacketFinding::new(
                "notebook_cell_chrome_packet.action_classes_coverage",
                subject,
                "cell_chrome_action_classes must list every variant",
            ));
        }
        if self.run_scope_control_lock_reason_classes.len() != RunScopeControlLockReasonClass::ALL.len() {
            findings.push(NotebookCellChromePacketFinding::new(
                "notebook_cell_chrome_packet.lock_reason_classes_coverage",
                subject,
                "run_scope_control_lock_reason_classes must list every variant",
            ));
        }

        for chrome in &self.example_cell_chromes {
            findings.extend(chrome.validate().into_iter().map(|f| {
                NotebookCellChromePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for control in &self.example_run_scope_controls {
            findings.extend(control.validate().into_iter().map(|f| {
                NotebookCellChromePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }
        for row in &self.example_durable_execution_state_rows {
            findings.extend(row.validate().into_iter().map(|f| {
                NotebookCellChromePacketFinding::new(&f.check_id, &f.subject_ref, &f.message)
            }));
        }

        findings
    }
}

impl CellChromeStatusClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Idle,
        Self::Queued,
        Self::Executing,
        Self::Succeeded,
        Self::Errored,
        Self::Interrupted,
        Self::Cancelled,
        Self::StaleOutput,
        Self::NoKernel,
    ];
}

impl CellChromeActionClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RunCell,
        Self::RunCellAndAdvance,
        Self::RunAllAbove,
        Self::RunAllBelow,
        Self::ClearOutput,
        Self::ToggleCollapseOutput,
        Self::ToggleFoldSource,
        Self::DebugCell,
        Self::ExportOutput,
    ];
}

impl RunScopeControlLockReasonClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::NotLocked,
        Self::LockedByPolicy,
        Self::LockedDuringExecution,
        Self::LockedReplayOnlyEnvironment,
        Self::LockedNoKernel,
    ];
}

/// Parses the checked-in cell-chrome packet JSON.
pub fn current_notebook_cell_chrome_packet(
) -> Result<NotebookCellChromePacket, serde_json::Error> {
    serde_json::from_str(NOTEBOOK_CELL_CHROME_PACKET_JSON)
}

#[cfg(test)]
mod tests;
