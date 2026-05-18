//! Retained notebook preview runtime-truth records.
//!
//! See the crate-level docs for what this module owns and why. The records,
//! closed vocabularies, and validators here mirror the boundary schemas at
//! `/schemas/notebook/kernel_session_summary.schema.json` and
//! `/schemas/notebook/output_trust_record.schema.json`. They reuse the four
//! orthogonal trust axes — document, kernel, output, widget — already frozen
//! by `/schemas/notebook/notebook_metadata_aureline.schema.json` and the
//! kernels-and-trust matrix at
//! `/artifacts/notebook/kernels_and_trust_matrix.yaml`. Nothing on a record
//! here may carry raw notebook JSON, raw cell source, raw output bytes, raw
//! kernel-protocol frames, raw widget state bytes, or raw URLs.
//!
//! The module exposes:
//!
//! - the [`NotebookHeaderBlock`] / [`KernelSessionSummary`] pair that backs
//!   the notebook header and kernel bar, including selected kernel or
//!   `No kernel`, execution origin, paired-export state, dirty state,
//!   document-trust state, last-successful-run summary, and the
//!   select/restart/interrupt action posture;
//! - the [`CellExecutionDetailRow`] record carried by the cell-execution
//!   detail row (cell id/index, run scope, timestamps, duration, outcome,
//!   output count, trace/log handoff);
//! - the [`VariableExplorerEntry`] record that keeps variable-explorer entries
//!   labelled live-vs-snapshot-vs-stale with controlled truncation/export
//!   actions instead of implying durable project facts;
//! - the [`OutputTrustRecord`] record that pins rich-output trust into the
//!   four explicit classes [`OutputTrustClass::Sanitized`],
//!   [`OutputTrustClass::Sandboxed`], [`OutputTrustClass::TrustedActive`], and
//!   [`OutputTrustClass::Stale`], with raw/open/export fallbacks and an
//!   explicit no-hidden-escalation posture;
//! - the [`DebuggerBridgeState`] record that says whether a debugger bridge is
//!   supported, why it is or is not, what adapter/kernel class is in force,
//!   how the current cell/frame relates, and what reconnect review is
//!   required when kernel/runtime drift occurs;
//! - the [`ReconnectReviewSheet`] derived when a kernel restart, reconnect,
//!   or runtime drift occurs, so the user always knows what runtime state
//!   will be lost, what queued work is affected, and whether Aureline is
//!   reopening a transcript, a live kernel, or a degraded preview.
//!
//! Every record carries a stable `record_kind` tag and a
//! `notebook_runtime_truth_schema_version`. Every closed vocabulary exposes
//! the `as_str` token that audits, exports, fixtures, and UI surfaces share.

use serde::{Deserialize, Serialize};

/// Schema version stamped on every runtime-truth record carried by this
/// module. Bumped only on breaking payload changes; additive-optional fields
/// do not bump this value.
pub const NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`KernelSessionSummary`] payloads.
pub const KERNEL_SESSION_SUMMARY_RECORD_KIND: &str = "notebook_kernel_session_summary_record";

/// Stable record-kind tag for serialized [`CellExecutionDetailRow`] payloads.
pub const CELL_EXECUTION_DETAIL_ROW_RECORD_KIND: &str = "notebook_cell_execution_detail_row_record";

/// Stable record-kind tag for serialized [`VariableExplorerEntry`] payloads.
pub const VARIABLE_EXPLORER_ENTRY_RECORD_KIND: &str = "notebook_variable_explorer_entry_record";

/// Stable record-kind tag for serialized [`OutputTrustRecord`] payloads.
pub const OUTPUT_TRUST_RECORD_KIND: &str = "notebook_output_trust_record";

/// Stable record-kind tag for serialized [`DebuggerBridgeState`] payloads.
pub const DEBUGGER_BRIDGE_STATE_RECORD_KIND: &str = "notebook_debugger_bridge_state_record";

/// Stable record-kind tag for serialized [`ReconnectReviewSheet`] payloads.
pub const RECONNECT_REVIEW_SHEET_RECORD_KIND: &str = "notebook_reconnect_review_sheet_record";

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
    /// Notebook document trust class projected into the header. Mirrors the
    /// document-trust axis frozen in
    /// `/schemas/notebook/notebook_metadata_aureline.schema.json` (axis 1 of 4)
    /// so the header chip and audit logs quote the same token.
    NotebookDocumentTrustClass {
        InheritedFromWorkspace => "document_trust_inherited_from_workspace",
        ElevatedOnExplicitGrant => "document_trust_elevated_on_explicit_grant",
        RestrictedByPolicy => "document_trust_restricted_by_policy",
        Revoked => "document_trust_revoked",
    }
);

closed_vocab!(
    /// Header-visible dirty state. Distinguishes saved-clean documents from
    /// documents with unsaved edits and from documents whose canonical
    /// `.ipynb` cannot be reconciled with the in-memory model.
    NotebookDirtyStateClass {
        Clean => "clean_saved",
        DirtyEdited => "dirty_user_edited",
        DirtyCellOrdering => "dirty_cell_ordering_changed",
        DirtyExternalChange => "dirty_external_change_detected",
        UnreconciledCanonicalMismatch => "unreconciled_canonical_mismatch",
    }
);

closed_vocab!(
    /// Where the kernel actually runs. Mirrors
    /// `notebook_kernel_transport_class` from the metadata schema so the
    /// header chip never implies a local execution origin for a remote
    /// kernel — and adds an explicit `no_kernel` value for the case where
    /// the document is editable but no kernel has been selected or could be
    /// resolved.
    KernelOriginClass {
        NoKernel => "no_kernel",
        LocalManagedToolchainKernel => "local_managed_toolchain_kernel",
        LocalProvisionedKernel => "local_provisioned_kernel",
        RemoteAgentPrimaryKernel => "remote_agent_primary_kernel",
        ManagedWorkspaceAgentKernel => "managed_workspace_agent_kernel",
        ProviderSideRemoteKernel => "provider_side_remote_kernel",
        CompatibilityBridgeRemoteKernel => "compatibility_bridge_remote_kernel",
    }
);

impl KernelOriginClass {
    /// True for any execution origin that runs the kernel off the local host.
    /// The header MUST render the local-vs-remote boundary cue whenever this
    /// returns true.
    pub const fn is_remote_boundary(self) -> bool {
        matches!(
            self,
            Self::RemoteAgentPrimaryKernel
                | Self::ManagedWorkspaceAgentKernel
                | Self::ProviderSideRemoteKernel
                | Self::CompatibilityBridgeRemoteKernel
        )
    }

    /// True when there is no resolvable kernel at all. The header reads
    /// `No kernel` and execution-dependent affordances render as
    /// `chip_execution_unavailable_without_kernel`.
    pub const fn is_no_kernel(self) -> bool {
        matches!(self, Self::NoKernel)
    }
}

closed_vocab!(
    /// Kernel-selection state shown on the kernel bar. Distinguishes the
    /// resolved-and-selected case from no-selection, user-pending, and
    /// policy-narrowed cases.
    KernelSelectionState {
        NoKernelSelected => "no_kernel_selected",
        SelectedKernelResolved => "selected_kernel_resolved",
        SelectionPendingUser => "selection_pending_user",
        SelectionPendingResolver => "selection_pending_resolver",
        SelectionNarrowedByPolicy => "selection_narrowed_by_policy",
        SelectionUnavailableNoCompatibleKernel => "selection_unavailable_no_compatible_kernel",
    }
);

closed_vocab!(
    /// Paired-text export posture for the header chip. Mirrors the
    /// metadata schema's `notebook_paired_text_export_posture_class` so the
    /// header chip never implies a paired form can be promoted to canonical.
    NotebookPairedExportPosture {
        NotApplicable => "paired_text_export_not_applicable",
        DerivedNotebookToScript => "paired_text_export_derived_notebook_to_script",
        DerivedNotebookToMarkdown => "paired_text_export_derived_notebook_to_markdown",
    }
);

closed_vocab!(
    /// Available kernel-bar actions. The header surfaces these as discrete
    /// affordances so a user is never asked to infer the difference between
    /// "Select kernel", "Restart", and "Interrupt" from a single overloaded
    /// chip.
    KernelBarActionClass {
        SelectKernel => "select_kernel",
        ChangeKernel => "change_kernel",
        Restart => "restart_kernel",
        RestartAndRunAll => "restart_kernel_and_run_all",
        Interrupt => "interrupt_kernel",
        Reconnect => "reconnect_kernel",
        Shutdown => "shutdown_kernel",
    }
);

closed_vocab!(
    /// Cell-execution run scope. Records whether the row describes a run in
    /// the current session, an attempt captured from a prior session, a
    /// replay drawn from captured output, or a manual user-driven action.
    CellExecutionRunScope {
        CurrentSession => "current_session",
        PriorSession => "prior_session",
        ReplayFromCapturedOutput => "replay_from_captured_output",
        ManualUserAction => "manual_user_action",
        QueuedNotYetStarted => "queued_not_yet_started",
    }
);

closed_vocab!(
    /// Outcome of a single cell execution. Distinguishes the success,
    /// error, interrupted, cancelled, queued, and no-kernel-skip paths
    /// rather than collapsing them into a single bool.
    CellExecutionOutcomeClass {
        Succeeded => "succeeded",
        Errored => "errored",
        Interrupted => "interrupted",
        Cancelled => "cancelled",
        Queued => "queued",
        SkippedNoKernel => "skipped_no_kernel",
        SkippedByPolicy => "skipped_by_policy",
    }
);

closed_vocab!(
    /// Variable-explorer freshness class. Names whether the value the user
    /// sees is the live kernel value, a snapshot captured from a prior
    /// session, a stale value retained after restart, or an imported value
    /// drawn from offline evidence.
    VariableExplorerFreshnessClass {
        LiveFromCurrentSession => "live_from_current_session",
        SnapshotFromPriorSession => "snapshot_from_prior_session",
        StaleAfterRestart => "stale_after_restart",
        ImportedSnapshot => "imported_snapshot",
        NoLiveVariablesNoKernel => "no_live_variables_no_kernel",
    }
);

closed_vocab!(
    /// Controlled truncation class on a variable-explorer entry. Names the
    /// reason a value is partial so the user is never asked to assume a
    /// truncated value is the whole value.
    VariableExplorerTruncationClass {
        NoTruncation => "no_truncation",
        TruncatedForDisplay => "truncated_for_display",
        TruncatedForRedaction => "truncated_for_redaction",
        TruncatedForSize => "truncated_for_size",
        UnsupportedTypeNoPreview => "unsupported_type_no_preview",
    }
);

closed_vocab!(
    /// Rich-output trust class projected onto rendered outputs. This is the
    /// rendering trust posture the row shows; it is orthogonal to (and never
    /// silently escalated from) the lineage-bearing
    /// `notebook_output_trust_state` axis in the metadata schema.
    OutputTrustClass {
        Sanitized => "sanitized",
        Sandboxed => "sandboxed",
        TrustedActive => "trusted_active",
        Stale => "stale",
    }
);

impl OutputTrustClass {
    /// Whether this class admits live scripts, fonts, network requests, or
    /// other active behavior in the rendered output.
    pub const fn admits_active_behavior(self) -> bool {
        matches!(self, Self::TrustedActive)
    }

    /// Whether outputs in this class require an explicit user review before
    /// any change in rendering posture.
    pub const fn requires_explicit_review_to_escalate(self) -> bool {
        matches!(self, Self::Sanitized | Self::Sandboxed | Self::Stale)
    }
}

closed_vocab!(
    /// Fallback actions exposed by the output viewer. Every output row
    /// surfaces a stable list of these; raw and export actions never
    /// substitute for the typed compatible viewer without an explicit
    /// chip change.
    OutputTrustFallbackActionClass {
        OpenCompatibleViewer => "open_compatible_viewer",
        OpenRawFallback => "open_raw_fallback",
        ExportWithRedaction => "export_with_redaction",
        CopyAsText => "copy_as_text",
        ReviewBeforeTrust => "review_before_trust",
    }
);

closed_vocab!(
    /// Hidden-escalation posture for an output. Pins the answer to
    /// "could rendering escalate trust without the user noticing?" to a
    /// closed vocabulary; the chrome row never silently flips from
    /// `Sanitized` to `TrustedActive`.
    OutputTrustHiddenEscalationPosture {
        NoHiddenEscalationAllowed => "no_hidden_escalation_allowed",
        ExplicitReviewRequired => "explicit_review_required",
        BlockedByPolicy => "blocked_by_policy",
    }
);

closed_vocab!(
    /// Stale-reason vocabulary on an [`OutputTrustClass::Stale`] output.
    /// Every stale output cites exactly one of these values so the row
    /// never implies the output is current.
    OutputTrustStaleReasonClass {
        KernelRestartedSinceProduce => "kernel_restarted_since_produce",
        KernelLostTransport => "kernel_lost_transport",
        DocumentTrustDowngradedSinceProduce => "document_trust_downgraded_since_produce",
        OutputCapturedFromPriorSession => "output_captured_from_prior_session",
        SourceCellEditedSinceProduce => "source_cell_edited_since_produce",
        OrphanedNoKernelBinding => "orphaned_no_kernel_binding",
    }
);

closed_vocab!(
    /// Whether the runtime exposes a debugger bridge for the active kernel
    /// at all, and if so to what extent.
    DebuggerBridgeSupportClass {
        Supported => "supported",
        SupportedPartial => "supported_partial",
        Unsupported => "unsupported",
        UnsupportedByPolicy => "unsupported_by_policy",
        UnsupportedNoKernel => "unsupported_no_kernel",
        UnsupportedRemoteParityUnverified => "unsupported_remote_parity_unverified",
    }
);

closed_vocab!(
    /// Typed reason describing why a debugger bridge is unsupported or only
    /// partially supported. Every non-`Supported` support class MUST cite
    /// one of these values; the chrome row never claims debugger parity
    /// silently when the runtime cannot actually back it.
    DebuggerBridgeUnsupportedReasonClass {
        NotApplicableSupported => "not_applicable_supported",
        AdapterUnavailableForKernelClass => "adapter_unavailable_for_kernel_class",
        AdapterCapabilityNarrowedByPolicy => "adapter_capability_narrowed_by_policy",
        KernelDoesNotImplementDebugProtocol => "kernel_does_not_implement_debug_protocol",
        KernelClassCellSteppingUnsupported => "kernel_class_cell_stepping_unsupported",
        RemoteAdapterRoundTripUnverified => "remote_adapter_round_trip_unverified",
        BridgeCancelledByRestartOrReconnect => "bridge_cancelled_by_restart_or_reconnect",
        NoKernelSession => "no_kernel_session",
    }
);

closed_vocab!(
    /// Adapter classes that can back the debugger bridge.
    DebuggerBridgeAdapterClass {
        NoAdapter => "no_adapter",
        KernelEmbeddedDebugProtocol => "kernel_embedded_debug_protocol",
        ExternalDebugAdapter => "external_debug_adapter",
        RemoteAgentForwardedAdapter => "remote_agent_forwarded_adapter",
        CompatibilityBridgeForwardedAdapter => "compatibility_bridge_forwarded_adapter",
    }
);

closed_vocab!(
    /// Kernel classes referenced by the debugger bridge so the row can
    /// honestly say what runtime the bridge is talking to. Mirrors the
    /// metadata schema's `notebook_kernel_transport_class` and the
    /// kernels-and-trust matrix.
    DebuggerBridgeKernelClass {
        NoKernel => "no_kernel",
        LocalManagedToolchainKernel => "local_managed_toolchain_kernel",
        LocalProvisionedKernel => "local_provisioned_kernel",
        RemoteAgentPrimaryKernel => "remote_agent_primary_kernel",
        ManagedWorkspaceAgentKernel => "managed_workspace_agent_kernel",
        ProviderSideRemoteKernel => "provider_side_remote_kernel",
        CompatibilityBridgeRemoteKernel => "compatibility_bridge_remote_kernel",
    }
);

closed_vocab!(
    /// Relationship between the cell/frame the user is looking at and the
    /// debugger's current point of execution. Pins the answer to "does
    /// stepping in this cell mean what I think it does?" to a closed
    /// vocabulary.
    DebuggerBridgeFrameRelationClass {
        NoActiveFrame => "no_active_frame",
        CurrentCellMatchesCurrentFrame => "current_cell_matches_current_frame",
        DifferentCellPaused => "different_cell_paused",
        FrameInImportedLibrary => "frame_in_imported_library",
        FrameInPriorCellAttempt => "frame_in_prior_cell_attempt",
        FrameStaleAfterRestart => "frame_stale_after_restart",
    }
);

closed_vocab!(
    /// Breakpoint posture the runtime can actually honour. Distinguishes
    /// stable breakpoint support, source-map-only support, and the cases
    /// where breakpoints are silently dropped.
    DebuggerBridgeBreakpointPostureClass {
        BreakpointsHonoured => "breakpoints_honoured",
        BreakpointsHonouredSourceMapOnly => "breakpoints_honoured_source_map_only",
        BreakpointsDeferredUntilCellRun => "breakpoints_deferred_until_cell_run",
        BreakpointsNotSupportedByKernel => "breakpoints_not_supported_by_kernel",
        BreakpointsCancelledByRestart => "breakpoints_cancelled_by_restart",
        BreakpointsBlockedByPolicy => "breakpoints_blocked_by_policy",
    }
);

closed_vocab!(
    /// Why the reconnect/restart review sheet was opened. The chrome row
    /// always cites one of these reasons rather than implying the row is
    /// being shown for unrelated reasons.
    ReconnectReviewKind {
        UserInitiatedRestart => "user_initiated_restart",
        UserInitiatedReconnect => "user_initiated_reconnect",
        UserInitiatedShutdown => "user_initiated_shutdown",
        TransportLostReconnectAttempted => "transport_lost_reconnect_attempted",
        IdentityRotationRequiresRenegotiation => "identity_rotation_requires_renegotiation",
        TrustDowngradeCancelsInFlight => "trust_downgrade_cancels_in_flight",
        ManagedWorkspaceLifecycleChange => "managed_workspace_lifecycle_change",
        PolicyDeniesContinuedExecution => "policy_denies_continued_execution",
        WindowExceededFreshSessionRequired => "window_exceeded_fresh_session_required",
    }
);

closed_vocab!(
    /// What Aureline is actually reopening on the other side of the
    /// reconnect/restart. Pins the answer to "am I looking at a live
    /// kernel, a captured transcript, or a degraded preview?" — the chrome
    /// row never lets that drift to silence.
    ReconnectReviewConsequenceClass {
        ReopeningTranscriptNoLiveKernel => "reopening_transcript_no_live_kernel",
        ReopeningLiveKernelFreshSession => "reopening_live_kernel_fresh_session",
        ReopeningLiveKernelSameIdentity => "reopening_live_kernel_same_identity",
        ReopeningLiveKernelIdentityChanged => "reopening_live_kernel_identity_changed",
        ReopeningDegradedPreviewNoExecution => "reopening_degraded_preview_no_execution",
        QuarantinedAwaitingOperatorReview => "quarantined_awaiting_operator_review",
    }
);

/// Generic finding shape used by every record validator. Mirrors the
/// finding shapes other Aureline crates expose so a single review/audit/
/// support pipeline can consume them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeTruthFinding {
    /// Stable check id (e.g. `kernel_session_summary.no_kernel_actions`).
    pub check_id: String,
    /// Subject row id (record id, route id, kernel session id, ...).
    pub subject_ref: String,
    /// Export-safe finding message; carries no raw payloads.
    pub message: String,
}

impl RuntimeTruthFinding {
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

/// Typed validation finding for a [`KernelSessionSummary`].
pub type KernelSessionSummaryFinding = RuntimeTruthFinding;

/// Typed validation finding for a [`CellExecutionDetailRow`].
pub type CellExecutionFinding = RuntimeTruthFinding;

/// Typed validation finding for a [`VariableExplorerEntry`].
pub type VariableExplorerEntryFinding = RuntimeTruthFinding;

/// Typed validation finding for an [`OutputTrustRecord`].
pub type OutputTrustRecordFinding = RuntimeTruthFinding;

/// Typed validation finding for a [`DebuggerBridgeState`].
pub type DebuggerBridgeFinding = RuntimeTruthFinding;

/// Typed validation finding for a [`ReconnectReviewSheet`].
pub type ReconnectReviewSheetFinding = RuntimeTruthFinding;

/// Notebook-identity / document-trust / dirty-state block carried at the top
/// of the kernel-bar header.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookHeaderBlock {
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque VFS path-identity token for the document.
    pub document_path_token_ref: String,
    /// Export-safe notebook title for chrome / audits / exports.
    pub document_title_label: String,
    /// Document trust class projected for the header chip.
    pub document_trust_class: NotebookDocumentTrustClass,
    /// Header-visible dirty-state class.
    pub dirty_state_class: NotebookDirtyStateClass,
    /// Paired-text export posture.
    pub paired_export_posture: NotebookPairedExportPosture,
    /// Opaque paired-export ref; non-null only when the posture is a
    /// derived class.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paired_export_ref: Option<String>,
}

/// Stable last-successful-run summary carried on the kernel bar. Refers to
/// the run-lineage records the runtime crate already owns by opaque ref so
/// the chrome row never imports raw run metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookLastSuccessfulRunSummary {
    /// Opaque run id of the last successful run.
    pub run_id_ref: String,
    /// Opaque attempt id of the last successful attempt.
    pub attempt_id_ref: String,
    /// Wall-clock timestamp of the last successful run.
    pub completed_at: String,
    /// Number of cells the last successful run completed.
    pub cells_completed: u32,
    /// Export-safe summary label rendered in the chip.
    pub summary_label: String,
}

/// Canonical notebook header / kernel-bar record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KernelSessionSummary {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_runtime_truth_schema_version: u32,
    /// Stable opaque kernel-session-summary id (one per displayed row).
    pub summary_id: String,
    /// Opaque kernel-session id; null when no kernel is selected.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Opaque kernelspec id; null when no kernel is selected or no
    /// kernelspec is resolvable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernelspec_ref: Option<String>,
    /// Notebook-identity / document-trust / dirty-state block.
    pub header: NotebookHeaderBlock,
    /// Kernel-selection state shown on the kernel bar.
    pub kernel_selection_state: KernelSelectionState,
    /// Execution-origin class for the selected kernel.
    pub kernel_origin_class: KernelOriginClass,
    /// Whether the row renders the local-vs-remote boundary cue. MUST be
    /// `true` whenever `kernel_origin_class.is_remote_boundary()` returns
    /// `true`.
    pub local_vs_remote_boundary_cue_visible: bool,
    /// Opaque target-identity witness ref; required for any remote kernel
    /// origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_identity_witness_ref: Option<String>,
    /// Opaque remote-agent session id ref; required for any remote kernel
    /// origin.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remote_agent_session_id_ref: Option<String>,
    /// Opaque execution-context root ref for the kernel session, when one
    /// is bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_root_ref: Option<String>,
    /// Last-successful-run summary; absent when no successful run has been
    /// observed for this notebook.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_successful_run: Option<NotebookLastSuccessfulRunSummary>,
    /// Kernel-bar actions exposed to the user. Always at least
    /// `select_kernel` or `change_kernel`.
    pub available_actions: Vec<KernelBarActionClass>,
    /// Whether auto-rerun is forbidden after any restart, reconnect, or
    /// no-kernel transition. MUST be `true`; the field exists to make the
    /// invariant explicit in the record.
    pub auto_rerun_forbidden: bool,
    /// Capture timestamp.
    pub captured_at: String,
    /// Export-safe summary line.
    pub summary: String,
}

impl KernelSessionSummary {
    /// Returns typed truth-rule findings; an empty vector means the row is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<KernelSessionSummaryFinding> {
        let mut findings = Vec::new();
        let subject = self.summary_id.as_str();

        if self.record_kind != KERNEL_SESSION_SUMMARY_RECORD_KIND {
            findings.push(KernelSessionSummaryFinding::new(
                "kernel_session_summary.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    KERNEL_SESSION_SUMMARY_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_runtime_truth_schema_version != NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION {
            findings.push(KernelSessionSummaryFinding::new(
                "kernel_session_summary.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION}, found {}",
                    self.notebook_runtime_truth_schema_version
                ),
            ));
        }

        if !self.auto_rerun_forbidden {
            findings.push(KernelSessionSummaryFinding::new(
                "kernel_session_summary.auto_rerun_forbidden",
                subject,
                "auto_rerun_forbidden must be true for every retained header row",
            ));
        }

        if self.kernel_origin_class.is_remote_boundary() {
            if !self.local_vs_remote_boundary_cue_visible {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.remote_boundary_cue",
                    subject,
                    "remote kernel origins must render the local-vs-remote boundary cue",
                ));
            }
            if self.target_identity_witness_ref.is_none() {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.target_identity_witness_required",
                    subject,
                    "remote kernel origins must carry a target_identity_witness_ref",
                ));
            }
            if self.remote_agent_session_id_ref.is_none() {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.remote_agent_session_id_required",
                    subject,
                    "remote kernel origins must carry a remote_agent_session_id_ref",
                ));
            }
        } else {
            if self.target_identity_witness_ref.is_some() {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.local_no_target_identity_witness",
                    subject,
                    "local kernel origins must not carry a target_identity_witness_ref",
                ));
            }
            if self.remote_agent_session_id_ref.is_some() {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.local_no_remote_agent_session_id",
                    subject,
                    "local kernel origins must not carry a remote_agent_session_id_ref",
                ));
            }
        }

        if self.kernel_origin_class.is_no_kernel() {
            if self.kernel_session_id_ref.is_some() {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.no_kernel_session_id",
                    subject,
                    "no_kernel origins must not carry a kernel_session_id_ref",
                ));
            }
            let has_select = self
                .available_actions
                .iter()
                .any(|action| matches!(action, KernelBarActionClass::SelectKernel));
            if !has_select {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.no_kernel_actions",
                    subject,
                    "no_kernel origins must expose the select_kernel action",
                ));
            }
            let claims_running_actions = self.available_actions.iter().any(|action| {
                matches!(
                    action,
                    KernelBarActionClass::Interrupt | KernelBarActionClass::Restart
                )
            });
            if claims_running_actions {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.no_kernel_actions_running",
                    subject,
                    "no_kernel origins must not expose restart or interrupt actions",
                ));
            }
            if matches!(
                self.kernel_selection_state,
                KernelSelectionState::SelectedKernelResolved
            ) {
                findings.push(KernelSessionSummaryFinding::new(
                    "kernel_session_summary.no_kernel_selection_mismatch",
                    subject,
                    "no_kernel origins must not declare selected_kernel_resolved",
                ));
            }
        }

        if matches!(
            self.kernel_selection_state,
            KernelSelectionState::SelectedKernelResolved
        ) && self.kernel_origin_class.is_no_kernel()
        {
            findings.push(KernelSessionSummaryFinding::new(
                "kernel_session_summary.selection_resolved_requires_kernel_origin",
                subject,
                "selected_kernel_resolved requires a non-no_kernel execution origin",
            ));
        }

        match self.header.paired_export_posture {
            NotebookPairedExportPosture::NotApplicable => {
                if self.header.paired_export_ref.is_some() {
                    findings.push(KernelSessionSummaryFinding::new(
                        "kernel_session_summary.paired_export_ref_not_applicable",
                        subject,
                        "paired_text_export_not_applicable must not carry a paired_export_ref",
                    ));
                }
            }
            NotebookPairedExportPosture::DerivedNotebookToScript
            | NotebookPairedExportPosture::DerivedNotebookToMarkdown => {
                if self.header.paired_export_ref.is_none() {
                    findings.push(KernelSessionSummaryFinding::new(
                        "kernel_session_summary.paired_export_ref_required",
                        subject,
                        "derived paired-export postures must carry a paired_export_ref",
                    ));
                }
            }
        }

        if self.available_actions.is_empty() {
            findings.push(KernelSessionSummaryFinding::new(
                "kernel_session_summary.available_actions_required",
                subject,
                "kernel bar must expose at least one action",
            ));
        }

        findings
    }
}

/// Canonical cell-execution detail row record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellExecutionDetailRow {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_runtime_truth_schema_version: u32,
    /// Stable opaque cell-execution-detail row id.
    pub row_id: String,
    /// Opaque kernel-session id this row is attributed to; null only for
    /// the queued-no-kernel case.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque cell id (stable across save/diff/merge per the metadata
    /// schema's cell-id stability invariant).
    pub cell_id_ref: String,
    /// 0-based display index of the cell in the document.
    pub cell_display_index: u32,
    /// Opaque cell-execution id minted by the execution queue.
    pub cell_execution_id_ref: String,
    /// Run-scope class for this row.
    pub run_scope: CellExecutionRunScope,
    /// Wall-clock timestamp when execution started; null for queued rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    /// Wall-clock timestamp when execution finished; null for unfinished
    /// rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    /// Duration in milliseconds; null for unfinished rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_millis: Option<u64>,
    /// Outcome class for this row.
    pub outcome_class: CellExecutionOutcomeClass,
    /// Number of output blocks attributed to this execution.
    pub output_count: u32,
    /// Opaque task-event envelope ref describing the trace/log handoff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task_event_envelope_ref: Option<String>,
    /// Opaque log-slice ref handed off to the run-history / log surfaces.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_slice_ref: Option<String>,
    /// Opaque diagnostic ref handed off when the outcome is `Errored`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub diagnostic_ref: Option<String>,
    /// Export-safe row summary.
    pub summary: String,
}

impl CellExecutionDetailRow {
    /// Returns typed truth-rule findings; an empty vector means the row is
    /// internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<CellExecutionFinding> {
        let mut findings = Vec::new();
        let subject = self.row_id.as_str();

        if self.record_kind != CELL_EXECUTION_DETAIL_ROW_RECORD_KIND {
            findings.push(CellExecutionFinding::new(
                "cell_execution_detail_row.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    CELL_EXECUTION_DETAIL_ROW_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_runtime_truth_schema_version != NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION {
            findings.push(CellExecutionFinding::new(
                "cell_execution_detail_row.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION}, found {}",
                    self.notebook_runtime_truth_schema_version
                ),
            ));
        }

        match self.outcome_class {
            CellExecutionOutcomeClass::Queued => {
                if self.run_scope != CellExecutionRunScope::QueuedNotYetStarted {
                    findings.push(CellExecutionFinding::new(
                        "cell_execution_detail_row.queued_run_scope",
                        subject,
                        "queued outcome must use run_scope=queued_not_yet_started",
                    ));
                }
                if self.started_at.is_some()
                    || self.finished_at.is_some()
                    || self.duration_millis.is_some()
                {
                    findings.push(CellExecutionFinding::new(
                        "cell_execution_detail_row.queued_timestamps",
                        subject,
                        "queued rows must not carry started_at, finished_at, or duration_millis",
                    ));
                }
                if self.output_count != 0 {
                    findings.push(CellExecutionFinding::new(
                        "cell_execution_detail_row.queued_output_count",
                        subject,
                        "queued rows must report output_count=0",
                    ));
                }
            }
            CellExecutionOutcomeClass::SkippedNoKernel => {
                if self.kernel_session_id_ref.is_some() {
                    findings.push(CellExecutionFinding::new(
                        "cell_execution_detail_row.skipped_no_kernel_session_id",
                        subject,
                        "skipped_no_kernel rows must not carry a kernel_session_id_ref",
                    ));
                }
            }
            CellExecutionOutcomeClass::Errored => {
                if self.diagnostic_ref.is_none() {
                    findings.push(CellExecutionFinding::new(
                        "cell_execution_detail_row.errored_diagnostic_required",
                        subject,
                        "errored outcomes must carry a diagnostic_ref",
                    ));
                }
            }
            CellExecutionOutcomeClass::Interrupted | CellExecutionOutcomeClass::Cancelled => {
                if self.finished_at.is_none() {
                    findings.push(CellExecutionFinding::new(
                        "cell_execution_detail_row.interrupted_finished_at",
                        subject,
                        "interrupted/cancelled rows must record a finished_at timestamp",
                    ));
                }
            }
            CellExecutionOutcomeClass::Succeeded | CellExecutionOutcomeClass::SkippedByPolicy => {}
        }

        if matches!(
            self.run_scope,
            CellExecutionRunScope::ReplayFromCapturedOutput
        ) && self.outcome_class == CellExecutionOutcomeClass::Succeeded
        {
            findings.push(CellExecutionFinding::new(
                "cell_execution_detail_row.replay_outcome_class",
                subject,
                "replay rows must not silently claim succeeded; use prior_session if attributing to the capture",
            ));
        }

        if matches!(self.run_scope, CellExecutionRunScope::QueuedNotYetStarted)
            && self.outcome_class != CellExecutionOutcomeClass::Queued
        {
            findings.push(CellExecutionFinding::new(
                "cell_execution_detail_row.queued_outcome_class",
                subject,
                "queued_not_yet_started must report outcome_class=queued",
            ));
        }

        findings
    }
}

/// Canonical variable-explorer entry record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VariableExplorerEntry {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_runtime_truth_schema_version: u32,
    /// Stable opaque entry id.
    pub entry_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque kernel-session id this entry is attributed to; null when the
    /// freshness class is `no_live_variables_no_kernel`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Opaque variable handle ref (stable across the kernel session).
    pub variable_handle_ref: String,
    /// Export-safe variable display name (no raw values).
    pub display_name_label: String,
    /// Opaque type-descriptor ref for the variable's runtime type.
    pub type_descriptor_ref: String,
    /// Freshness class.
    pub freshness_class: VariableExplorerFreshnessClass,
    /// Controlled truncation class.
    pub truncation_class: VariableExplorerTruncationClass,
    /// Available controlled actions on this entry.
    pub available_actions: Vec<VariableExplorerEntryActionClass>,
    /// Export-safe entry summary.
    pub summary: String,
}

impl VariableExplorerEntry {
    /// Returns typed truth-rule findings; an empty vector means the entry
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<VariableExplorerEntryFinding> {
        let mut findings = Vec::new();
        let subject = self.entry_id.as_str();

        if self.record_kind != VARIABLE_EXPLORER_ENTRY_RECORD_KIND {
            findings.push(VariableExplorerEntryFinding::new(
                "variable_explorer_entry.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    VARIABLE_EXPLORER_ENTRY_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_runtime_truth_schema_version != NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION {
            findings.push(VariableExplorerEntryFinding::new(
                "variable_explorer_entry.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION}, found {}",
                    self.notebook_runtime_truth_schema_version
                ),
            ));
        }

        match self.freshness_class {
            VariableExplorerFreshnessClass::NoLiveVariablesNoKernel => {
                if self.kernel_session_id_ref.is_some() {
                    findings.push(VariableExplorerEntryFinding::new(
                        "variable_explorer_entry.no_live_no_kernel_session_ref",
                        subject,
                        "no_live_variables_no_kernel must not carry a kernel_session_id_ref",
                    ));
                }
                if self.available_actions.iter().any(|action| {
                    matches!(action, VariableExplorerEntryActionClass::OpenLiveViewer)
                }) {
                    findings.push(VariableExplorerEntryFinding::new(
                        "variable_explorer_entry.no_live_open_live_viewer",
                        subject,
                        "no_live_variables_no_kernel must not expose open_live_viewer",
                    ));
                }
            }
            VariableExplorerFreshnessClass::LiveFromCurrentSession => {
                if self.kernel_session_id_ref.is_none() {
                    findings.push(VariableExplorerEntryFinding::new(
                        "variable_explorer_entry.live_kernel_session_required",
                        subject,
                        "live_from_current_session must carry a kernel_session_id_ref",
                    ));
                }
            }
            VariableExplorerFreshnessClass::SnapshotFromPriorSession
            | VariableExplorerFreshnessClass::StaleAfterRestart
            | VariableExplorerFreshnessClass::ImportedSnapshot => {
                if self.available_actions.iter().any(|action| {
                    matches!(action, VariableExplorerEntryActionClass::OpenLiveViewer)
                }) {
                    findings.push(VariableExplorerEntryFinding::new(
                        "variable_explorer_entry.snapshot_open_live_viewer",
                        subject,
                        "snapshot/stale/imported entries must not expose open_live_viewer",
                    ));
                }
            }
        }

        if matches!(
            self.truncation_class,
            VariableExplorerTruncationClass::UnsupportedTypeNoPreview
        ) && self
            .available_actions
            .iter()
            .any(|action| matches!(action, VariableExplorerEntryActionClass::OpenLiveViewer))
        {
            findings.push(VariableExplorerEntryFinding::new(
                "variable_explorer_entry.unsupported_type_open_live_viewer",
                subject,
                "unsupported_type_no_preview must not expose open_live_viewer",
            ));
        }

        findings
    }
}

closed_vocab!(
    /// Controlled actions exposed on a variable-explorer entry. Pinned so
    /// the chrome row never re-invents export/copy actions that would
    /// broaden capture.
    VariableExplorerEntryActionClass {
        OpenLiveViewer => "open_live_viewer",
        OpenSnapshotViewer => "open_snapshot_viewer",
        ExportWithRedaction => "export_with_redaction",
        ReviewBeforeExport => "review_before_export",
        DismissFromExplorer => "dismiss_from_explorer",
    }
);

/// Canonical rich-output trust record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputTrustRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_runtime_truth_schema_version: u32,
    /// Stable opaque output-trust record id (per displayed output).
    pub record_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque cell id this output is attributed to.
    pub cell_id_ref: String,
    /// Opaque output-block id (one record per rendered output).
    pub output_block_ref: String,
    /// Opaque MIME bundle descriptor ref.
    pub mime_bundle_descriptor_ref: String,
    /// Output trust class for the rendered surface.
    pub trust_class: OutputTrustClass,
    /// Hidden-escalation posture for this output.
    pub hidden_escalation_posture: OutputTrustHiddenEscalationPosture,
    /// Stale-reason class. MUST be `Some` when `trust_class` is `Stale`,
    /// MUST be `None` otherwise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_reason_class: Option<OutputTrustStaleReasonClass>,
    /// Available fallback actions in stable order. MUST contain at least
    /// `OpenCompatibleViewer` for `Sanitized`, `Sandboxed`, and
    /// `TrustedActive`; MAY omit it for `Stale` when no compatible viewer
    /// can be reconstructed.
    pub fallback_actions: Vec<OutputTrustFallbackActionClass>,
    /// Whether a compatible viewer is currently available for the output.
    pub compatible_viewer_available: bool,
    /// Whether a raw fallback viewer is available for the output.
    pub raw_fallback_available: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl OutputTrustRecord {
    /// Returns typed truth-rule findings; an empty vector means the record
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<OutputTrustRecordFinding> {
        let mut findings = Vec::new();
        let subject = self.record_id.as_str();

        if self.record_kind != OUTPUT_TRUST_RECORD_KIND {
            findings.push(OutputTrustRecordFinding::new(
                "output_trust_record.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    OUTPUT_TRUST_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_runtime_truth_schema_version != NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION {
            findings.push(OutputTrustRecordFinding::new(
                "output_trust_record.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION}, found {}",
                    self.notebook_runtime_truth_schema_version
                ),
            ));
        }

        match self.trust_class {
            OutputTrustClass::Stale => {
                if self.stale_reason_class.is_none() {
                    findings.push(OutputTrustRecordFinding::new(
                        "output_trust_record.stale_reason_required",
                        subject,
                        "stale outputs must cite a stale_reason_class",
                    ));
                }
                if self.hidden_escalation_posture
                    == OutputTrustHiddenEscalationPosture::NoHiddenEscalationAllowed
                    && self.compatible_viewer_available
                {
                    // Stale outputs may still expose a compatible viewer;
                    // this branch only checks the inverse rule below.
                }
            }
            _ => {
                if self.stale_reason_class.is_some() {
                    findings.push(OutputTrustRecordFinding::new(
                        "output_trust_record.non_stale_reason_present",
                        subject,
                        "non-stale outputs must not carry a stale_reason_class",
                    ));
                }
            }
        }

        if self.trust_class.requires_explicit_review_to_escalate()
            && self.hidden_escalation_posture
                == OutputTrustHiddenEscalationPosture::NoHiddenEscalationAllowed
            && !self
                .fallback_actions
                .iter()
                .any(|action| matches!(action, OutputTrustFallbackActionClass::ReviewBeforeTrust))
            && self.trust_class != OutputTrustClass::Stale
        {
            findings.push(OutputTrustRecordFinding::new(
                "output_trust_record.review_before_trust_required",
                subject,
                "sanitized/sandboxed outputs must expose review_before_trust under no_hidden_escalation_allowed",
            ));
        }

        if matches!(self.trust_class, OutputTrustClass::TrustedActive)
            && !self.compatible_viewer_available
        {
            findings.push(OutputTrustRecordFinding::new(
                "output_trust_record.trusted_active_requires_viewer",
                subject,
                "trusted_active outputs require a compatible viewer",
            ));
        }

        if !matches!(self.trust_class, OutputTrustClass::Stale)
            && !self.fallback_actions.iter().any(|action| {
                matches!(action, OutputTrustFallbackActionClass::OpenCompatibleViewer)
            })
        {
            findings.push(OutputTrustRecordFinding::new(
                "output_trust_record.open_compatible_viewer_required",
                subject,
                "non-stale outputs must expose open_compatible_viewer",
            ));
        }

        if self.raw_fallback_available
            && !self
                .fallback_actions
                .iter()
                .any(|action| matches!(action, OutputTrustFallbackActionClass::OpenRawFallback))
        {
            findings.push(OutputTrustRecordFinding::new(
                "output_trust_record.raw_fallback_action_required",
                subject,
                "rows that claim raw_fallback_available must expose open_raw_fallback",
            ));
        }

        if self.fallback_actions.is_empty() {
            findings.push(OutputTrustRecordFinding::new(
                "output_trust_record.fallback_actions_required",
                subject,
                "every output must expose at least one fallback action",
            ));
        }

        findings
    }
}

/// Canonical debugger-bridge state record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DebuggerBridgeState {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_runtime_truth_schema_version: u32,
    /// Stable opaque debugger-bridge state id.
    pub state_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque kernel-session id this state is attributed to; null only
    /// when the support class is `unsupported_no_kernel`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kernel_session_id_ref: Option<String>,
    /// Whether the runtime exposes a debugger bridge at all, and if so
    /// to what extent.
    pub support_class: DebuggerBridgeSupportClass,
    /// Typed reason describing the support class. MUST cite a non-`NotApplicableSupported`
    /// value whenever `support_class` is not `Supported`.
    pub unsupported_reason_class: DebuggerBridgeUnsupportedReasonClass,
    /// Adapter class backing the bridge.
    pub adapter_class: DebuggerBridgeAdapterClass,
    /// Kernel class the bridge is talking to.
    pub kernel_class: DebuggerBridgeKernelClass,
    /// Opaque cell id the user is currently looking at, when one is
    /// focused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_cell_id_ref: Option<String>,
    /// Opaque debugger frame id, when paused.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_frame_ref: Option<String>,
    /// Relationship between the focused cell and the current frame.
    pub frame_relation_class: DebuggerBridgeFrameRelationClass,
    /// Breakpoint posture the runtime can actually honour.
    pub breakpoint_posture_class: DebuggerBridgeBreakpointPostureClass,
    /// Whether a reconnect/restart review sheet is required before the
    /// bridge is considered authoritative for stepping.
    pub reconnect_review_required: bool,
    /// Opaque reconnect-review sheet ref, when one has been generated.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconnect_review_sheet_ref: Option<String>,
    /// Export-safe summary line.
    pub summary: String,
}

impl DebuggerBridgeState {
    /// Returns typed truth-rule findings; an empty vector means the state
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<DebuggerBridgeFinding> {
        let mut findings = Vec::new();
        let subject = self.state_id.as_str();

        if self.record_kind != DEBUGGER_BRIDGE_STATE_RECORD_KIND {
            findings.push(DebuggerBridgeFinding::new(
                "debugger_bridge_state.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    DEBUGGER_BRIDGE_STATE_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_runtime_truth_schema_version != NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION {
            findings.push(DebuggerBridgeFinding::new(
                "debugger_bridge_state.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION}, found {}",
                    self.notebook_runtime_truth_schema_version
                ),
            ));
        }

        match self.support_class {
            DebuggerBridgeSupportClass::Supported => {
                if !matches!(
                    self.unsupported_reason_class,
                    DebuggerBridgeUnsupportedReasonClass::NotApplicableSupported
                ) {
                    findings.push(DebuggerBridgeFinding::new(
                        "debugger_bridge_state.supported_reason_class",
                        subject,
                        "supported bridges must cite not_applicable_supported",
                    ));
                }
                if self.kernel_session_id_ref.is_none() {
                    findings.push(DebuggerBridgeFinding::new(
                        "debugger_bridge_state.supported_kernel_required",
                        subject,
                        "supported bridges must carry a kernel_session_id_ref",
                    ));
                }
            }
            DebuggerBridgeSupportClass::UnsupportedNoKernel => {
                if self.kernel_session_id_ref.is_some() {
                    findings.push(DebuggerBridgeFinding::new(
                        "debugger_bridge_state.unsupported_no_kernel_session",
                        subject,
                        "unsupported_no_kernel must not carry a kernel_session_id_ref",
                    ));
                }
                if !matches!(
                    self.unsupported_reason_class,
                    DebuggerBridgeUnsupportedReasonClass::NoKernelSession
                ) {
                    findings.push(DebuggerBridgeFinding::new(
                        "debugger_bridge_state.unsupported_no_kernel_reason_class",
                        subject,
                        "unsupported_no_kernel must cite no_kernel_session",
                    ));
                }
                if !matches!(
                    self.frame_relation_class,
                    DebuggerBridgeFrameRelationClass::NoActiveFrame
                ) {
                    findings.push(DebuggerBridgeFinding::new(
                        "debugger_bridge_state.unsupported_no_kernel_frame_relation",
                        subject,
                        "unsupported_no_kernel must report frame_relation=no_active_frame",
                    ));
                }
            }
            DebuggerBridgeSupportClass::Unsupported
            | DebuggerBridgeSupportClass::SupportedPartial
            | DebuggerBridgeSupportClass::UnsupportedByPolicy
            | DebuggerBridgeSupportClass::UnsupportedRemoteParityUnverified => {
                if matches!(
                    self.unsupported_reason_class,
                    DebuggerBridgeUnsupportedReasonClass::NotApplicableSupported
                ) {
                    findings.push(DebuggerBridgeFinding::new(
                        "debugger_bridge_state.unsupported_reason_required",
                        subject,
                        "non-supported bridges must cite a non-not_applicable unsupported_reason_class",
                    ));
                }
            }
        }

        if self.current_frame_ref.is_some()
            && matches!(
                self.frame_relation_class,
                DebuggerBridgeFrameRelationClass::NoActiveFrame
            )
        {
            findings.push(DebuggerBridgeFinding::new(
                "debugger_bridge_state.frame_relation_consistency",
                subject,
                "frame_relation=no_active_frame must not carry a current_frame_ref",
            ));
        }
        if self.current_frame_ref.is_none()
            && !matches!(
                self.frame_relation_class,
                DebuggerBridgeFrameRelationClass::NoActiveFrame
            )
        {
            findings.push(DebuggerBridgeFinding::new(
                "debugger_bridge_state.frame_relation_requires_frame",
                subject,
                "frame_relation other than no_active_frame requires a current_frame_ref",
            ));
        }

        if self.reconnect_review_required && self.reconnect_review_sheet_ref.is_none() {
            findings.push(DebuggerBridgeFinding::new(
                "debugger_bridge_state.reconnect_review_sheet_required",
                subject,
                "reconnect_review_required=true requires a reconnect_review_sheet_ref",
            ));
        }
        if !self.reconnect_review_required && self.reconnect_review_sheet_ref.is_some() {
            findings.push(DebuggerBridgeFinding::new(
                "debugger_bridge_state.reconnect_review_sheet_not_applicable",
                subject,
                "reconnect_review_required=false must not carry a reconnect_review_sheet_ref",
            ));
        }

        if matches!(self.kernel_class, DebuggerBridgeKernelClass::NoKernel)
            && !matches!(
                self.support_class,
                DebuggerBridgeSupportClass::UnsupportedNoKernel
            )
        {
            findings.push(DebuggerBridgeFinding::new(
                "debugger_bridge_state.no_kernel_support_consistency",
                subject,
                "kernel_class=no_kernel requires support_class=unsupported_no_kernel",
            ));
        }

        findings
    }
}

/// Canonical reconnect / restart review sheet record. Generated whenever a
/// kernel restart, reconnect, shutdown, or runtime drift occurs so the user
/// always knows what runtime state will be lost, what queued work is
/// affected, and whether Aureline is reopening a transcript, a live kernel,
/// or a degraded preview.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReconnectReviewSheet {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub notebook_runtime_truth_schema_version: u32,
    /// Stable opaque sheet id.
    pub sheet_id: String,
    /// Opaque notebook-document id.
    pub document_id_ref: String,
    /// Opaque prior kernel-session id, when one existed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prior_kernel_session_id_ref: Option<String>,
    /// Opaque next kernel-session id, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_kernel_session_id_ref: Option<String>,
    /// Why this sheet is being shown.
    pub kind: ReconnectReviewKind,
    /// What the runtime is actually reopening.
    pub consequence_class: ReconnectReviewConsequenceClass,
    /// Whether all in-flight executions are cancelled by this event. MUST
    /// be `true` for any consequence other than
    /// `ReopeningLiveKernelSameIdentity`.
    pub in_flight_executions_cancelled: bool,
    /// Number of queued cells affected by this event.
    pub queued_cells_affected: u32,
    /// Whether live kernel variable state is lost on this event. MUST be
    /// `true` for `ReopeningTranscriptNoLiveKernel`,
    /// `ReopeningLiveKernelFreshSession`,
    /// `ReopeningLiveKernelIdentityChanged`, and
    /// `ReopeningDegradedPreviewNoExecution`.
    pub live_variable_state_lost: bool,
    /// Whether the consequence requires an explicit user confirmation
    /// before it is committed. MUST be `true` for any restart, identity
    /// change, or trust downgrade.
    pub user_confirmation_required: bool,
    /// Whether auto-rerun is forbidden on the other side of this event.
    /// MUST be `true`; the field exists to make the invariant explicit.
    pub auto_rerun_forbidden: bool,
    /// Export-safe summary line.
    pub summary: String,
}

impl ReconnectReviewSheet {
    /// Returns typed truth-rule findings; an empty vector means the sheet
    /// is internally consistent with the schema's invariants.
    pub fn validate(&self) -> Vec<ReconnectReviewSheetFinding> {
        let mut findings = Vec::new();
        let subject = self.sheet_id.as_str();

        if self.record_kind != RECONNECT_REVIEW_SHEET_RECORD_KIND {
            findings.push(ReconnectReviewSheetFinding::new(
                "reconnect_review_sheet.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    RECONNECT_REVIEW_SHEET_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.notebook_runtime_truth_schema_version != NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION {
            findings.push(ReconnectReviewSheetFinding::new(
                "reconnect_review_sheet.schema_version",
                subject,
                format!(
                    "schema_version must be {NOTEBOOK_RUNTIME_TRUTH_SCHEMA_VERSION}, found {}",
                    self.notebook_runtime_truth_schema_version
                ),
            ));
        }

        if !self.auto_rerun_forbidden {
            findings.push(ReconnectReviewSheetFinding::new(
                "reconnect_review_sheet.auto_rerun_forbidden",
                subject,
                "auto_rerun_forbidden must be true on every reconnect/restart sheet",
            ));
        }

        match self.consequence_class {
            ReconnectReviewConsequenceClass::ReopeningLiveKernelSameIdentity => {
                if !self.in_flight_executions_cancelled
                    && matches!(self.kind, ReconnectReviewKind::UserInitiatedRestart)
                {
                    findings.push(ReconnectReviewSheetFinding::new(
                        "reconnect_review_sheet.same_identity_restart_inflight",
                        subject,
                        "user_initiated_restart with same-identity consequence must cancel in-flight executions",
                    ));
                }
                if self.live_variable_state_lost
                    && !matches!(self.kind, ReconnectReviewKind::UserInitiatedRestart)
                {
                    findings.push(ReconnectReviewSheetFinding::new(
                        "reconnect_review_sheet.same_identity_no_variable_loss",
                        subject,
                        "same-identity reconnect (non-restart) must not claim live_variable_state_lost",
                    ));
                }
            }
            ReconnectReviewConsequenceClass::ReopeningTranscriptNoLiveKernel
            | ReconnectReviewConsequenceClass::ReopeningLiveKernelFreshSession
            | ReconnectReviewConsequenceClass::ReopeningLiveKernelIdentityChanged
            | ReconnectReviewConsequenceClass::ReopeningDegradedPreviewNoExecution
            | ReconnectReviewConsequenceClass::QuarantinedAwaitingOperatorReview => {
                if !self.in_flight_executions_cancelled {
                    findings.push(ReconnectReviewSheetFinding::new(
                        "reconnect_review_sheet.consequence_cancels_inflight",
                        subject,
                        "this consequence must cancel in-flight executions",
                    ));
                }
                if !self.live_variable_state_lost {
                    findings.push(ReconnectReviewSheetFinding::new(
                        "reconnect_review_sheet.consequence_loses_variables",
                        subject,
                        "this consequence must declare live_variable_state_lost=true",
                    ));
                }
            }
        }

        if matches!(
            self.kind,
            ReconnectReviewKind::TrustDowngradeCancelsInFlight
                | ReconnectReviewKind::IdentityRotationRequiresRenegotiation
                | ReconnectReviewKind::UserInitiatedRestart
                | ReconnectReviewKind::UserInitiatedShutdown
                | ReconnectReviewKind::WindowExceededFreshSessionRequired
        ) && !self.user_confirmation_required
        {
            findings.push(ReconnectReviewSheetFinding::new(
                "reconnect_review_sheet.user_confirmation_required",
                subject,
                "this reconnect/restart kind requires user_confirmation_required=true",
            ));
        }

        findings
    }
}

#[cfg(test)]
mod tests;
