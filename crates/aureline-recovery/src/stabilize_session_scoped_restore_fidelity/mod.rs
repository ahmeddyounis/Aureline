//! Session-scoped restore-fidelity truth shared by runtime-backed panes.
//!
//! This module defines the stable recovery vocabulary for panes whose live
//! runtime may not survive restart, crash recovery, display-topology changes,
//! or dependency loss. It intentionally separates restored context and
//! evidence from live authority: terminals, tasks, debug sessions, notebook
//! kernels, preview servers, and remote tunnels may reopen as transcripts,
//! placeholders, reconnect cards, rerun-required cards, or static evidence,
//! but they may not silently rerun work or reacquire privileged authority.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for [`SessionScopedRestoreFidelityPacket`].
pub const SESSION_SCOPED_RESTORE_FIDELITY_PACKET_RECORD_KIND: &str =
    "session_scoped_restore_fidelity_packet";

/// Stable record-kind tag for [`SessionScopedRestoreFidelitySupportExport`].
pub const SESSION_SCOPED_RESTORE_FIDELITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "session_scoped_restore_fidelity_support_export";

/// Integer schema version for the session-scoped restore-fidelity packet.
pub const SESSION_SCOPED_RESTORE_FIDELITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the shared boundary schema.
pub const SESSION_SCOPED_RESTORE_FIDELITY_SCHEMA_REF: &str =
    "schemas/recovery/session-restore-fidelity.schema.json";

/// Repo-relative path of the reviewer contract.
pub const SESSION_SCOPED_RESTORE_FIDELITY_DOC_REF: &str =
    "docs/reliability/stabilize-session-scoped-restore-fidelity.md";

/// Repo-relative path of the human-readable evidence artifact.
pub const SESSION_SCOPED_RESTORE_FIDELITY_ARTIFACT_REF: &str =
    "artifacts/reliability/stabilize-session-scoped-restore-fidelity.md";

/// Repo-relative path of the fixture corpus.
pub const SESSION_SCOPED_RESTORE_FIDELITY_FIXTURE_DIR: &str =
    "fixtures/recovery/stabilize-session-scoped-restore-fidelity";

/// Controlled restore-fidelity classes reused by recovery, diagnostics, crash
/// screens, support bundles, docs, and export packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreFidelityClass {
    /// Nothing downgraded; no placeholder, translation, or review.
    ExactRestore,
    /// Meaning was preserved through a declared compatibility path.
    CompatibleRestore,
    /// Layout and context returned without live authority.
    LayoutOnly,
    /// Dirty drafts returned for compare and explicit save.
    RecoveredDrafts,
    /// Only evidence, transcripts, snapshots, and provenance survived.
    EvidenceOnly,
}

impl RestoreFidelityClass {
    /// Every controlled restore-fidelity class, in declaration order.
    pub const REQUIRED: [Self; 5] = [
        Self::ExactRestore,
        Self::CompatibleRestore,
        Self::LayoutOnly,
        Self::RecoveredDrafts,
        Self::EvidenceOnly,
    ];

    /// Stable token used in schemas and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::RecoveredDrafts => "recovered_drafts",
            Self::EvidenceOnly => "evidence_only",
        }
    }

    /// User-visible controlled label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExactRestore => "Exact restore",
            Self::CompatibleRestore => "Compatible restore",
            Self::LayoutOnly => "Layout only",
            Self::RecoveredDrafts => "Recovered drafts",
            Self::EvidenceOnly => "Evidence only",
        }
    }
}

/// Runtime-backed pane or consumer surface covered by the shared contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionRestoreSurfaceClass {
    /// Terminal session panes.
    TerminalSession,
    /// Task or build/run task rows.
    TaskRun,
    /// Debug adapter sessions.
    DebugSession,
    /// Notebook kernel sessions.
    NotebookKernel,
    /// Preview servers and browser-preview runtimes.
    PreviewServer,
    /// Remote tunnels, helpers, and managed workspace channels.
    RemoteTunnel,
    /// Diagnostics surfaces that summarize restore outcomes.
    Diagnostics,
    /// Support bundle and export projections.
    SupportExport,
}

impl SessionRestoreSurfaceClass {
    /// Runtime-backed surfaces that must never auto-rerun during restore.
    pub const RUNTIME_BACKED: [Self; 6] = [
        Self::TerminalSession,
        Self::TaskRun,
        Self::DebugSession,
        Self::NotebookKernel,
        Self::PreviewServer,
        Self::RemoteTunnel,
    ];

    /// All consumers that must consume this packet directly.
    pub const REQUIRED_CONSUMERS: [Self; 8] = [
        Self::TerminalSession,
        Self::TaskRun,
        Self::DebugSession,
        Self::NotebookKernel,
        Self::PreviewServer,
        Self::RemoteTunnel,
        Self::Diagnostics,
        Self::SupportExport,
    ];

    /// Stable token used in schemas and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalSession => "terminal_session",
            Self::TaskRun => "task_run",
            Self::DebugSession => "debug_session",
            Self::NotebookKernel => "notebook_kernel",
            Self::PreviewServer => "preview_server",
            Self::RemoteTunnel => "remote_tunnel",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
        }
    }

    /// True when the surface is runtime-backed rather than a projection.
    pub const fn is_runtime_backed(self) -> bool {
        matches!(
            self,
            Self::TerminalSession
                | Self::TaskRun
                | Self::DebugSession
                | Self::NotebookKernel
                | Self::PreviewServer
                | Self::RemoteTunnel
        )
    }
}

/// Truthful restore state shown in a restored runtime-backed pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionRestoreStateClass {
    /// Transcript or output evidence restored; command was not rerun.
    TranscriptRestored,
    /// Prior runtime ended and no live continuation is available.
    SessionEnded,
    /// A known target can be reconnected after explicit user intent.
    ReconnectAvailable,
    /// Work can be rerun only after explicit user intent.
    RerunRequired,
    /// Context is unavailable, but the pane slot remains truthful.
    ContextUnavailable,
    /// Preview or output reopened as static evidence only.
    StaticEvidenceOnly,
    /// The same live runtime survived and was verified.
    LiveRuntimeSurvived,
}

impl SessionRestoreStateClass {
    /// Stable token used in schemas and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TranscriptRestored => "transcript_restored",
            Self::SessionEnded => "session_ended",
            Self::ReconnectAvailable => "reconnect_available",
            Self::RerunRequired => "rerun_required",
            Self::ContextUnavailable => "context_unavailable",
            Self::StaticEvidenceOnly => "static_evidence_only",
            Self::LiveRuntimeSurvived => "live_runtime_survived",
        }
    }

    /// True when a restored surface may still have live authority.
    pub const fn permits_live_authority(self) -> bool {
        matches!(self, Self::LiveRuntimeSurvived)
    }
}

/// Dependency state behind a restored pane placeholder.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreDependencyStateClass {
    /// Dependency is available and compatible.
    Available,
    /// Dependency is missing from the current profile or machine.
    Missing,
    /// Dependency exists but is incompatible with the restored record.
    Incompatible,
    /// Dependency was quarantined by recovery or policy.
    Quarantined,
    /// Permission, credential, or grant was revoked.
    Revoked,
    /// Dependency is offline or unreachable.
    Offline,
}

/// Explicit action required before a restored pane can regain liveness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreNextActionClass {
    /// No action is required because the row is a verified live survivor.
    NoneRequired,
    /// Open the retained transcript or static evidence.
    InspectEvidence,
    /// Reconnect to the named target after review.
    ReconnectExplicitly,
    /// Rerun the named task, command, cell, or preview server after review.
    RerunExplicitly,
    /// Reauthenticate before reconnecting or rerunning.
    ReauthenticateThenRetry,
    /// Repair or reinstall a missing dependency.
    RepairDependency,
    /// Export the evidence and recovery summary.
    ExportEvidence,
    /// Dismiss the stale pane after review.
    DismissPlaceholder,
}

/// Support posture for a row or consumer projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreSupportClass {
    /// Row satisfies the stable no-hidden-rerun contract.
    Stable,
    /// Row is intentionally downgraded below stable.
    DowngradedBelowStable,
    /// Row is preview-only and cannot claim stable restore fidelity.
    PreviewOnly,
    /// Row is unsupported.
    Unsupported,
}

impl RestoreSupportClass {
    /// True when the row may appear on a stable claim surface.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// One row in the placeholder-state matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlaceholderStateMatrixRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface covered by this row.
    pub surface_class: SessionRestoreSurfaceClass,
    /// Controlled restore-fidelity class.
    pub restore_fidelity_class: RestoreFidelityClass,
    /// Truthful state rendered in the restored pane or projection.
    pub restore_state: SessionRestoreStateClass,
    /// Current dependency state.
    pub dependency_state: RestoreDependencyStateClass,
    /// Whether the surrounding layout slot is preserved.
    pub layout_preserved: bool,
    /// Whether stale evidence or last-known metadata remains visible.
    pub stale_evidence_visible: bool,
    /// Whether automatic rerun is forbidden.
    pub auto_rerun_forbidden: bool,
    /// Whether silent reattach is forbidden.
    pub silent_reattach_forbidden: bool,
    /// Whether hidden authority reacquisition is forbidden.
    pub hidden_authority_reacquisition_forbidden: bool,
    /// Whether user intent is required before liveness can return.
    pub explicit_user_intent_required: bool,
    /// Redaction-safe target/runtime ref shown before action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_runtime_ref: Option<String>,
    /// Next actions offered by the card.
    pub next_actions: Vec<RestoreNextActionClass>,
    /// Evidence refs retained for diagnostics and support export.
    pub evidence_refs: Vec<String>,
    /// Reviewer-safe summary shown in restore surfaces.
    pub visible_summary: String,
    /// Row support posture.
    pub support_class: RestoreSupportClass,
}

/// One recovery drill proving the no-hidden-rerun contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NoHiddenRerunDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Surface under test.
    pub surface_class: SessionRestoreSurfaceClass,
    /// Interruption or topology-change condition.
    pub condition: String,
    /// Expected truthful restore state.
    pub expected_restore_state: SessionRestoreStateClass,
    /// Whether hidden rerun was attempted during the drill.
    pub hidden_rerun_attempted: bool,
    /// Whether silent reattach was attempted during the drill.
    pub silent_reattach_attempted: bool,
    /// Whether live authority was reacquired without user intent.
    pub authority_reacquired_without_user_intent: bool,
    /// Whether the action card names the target/runtime before resume.
    pub target_runtime_named_before_action: bool,
    /// Evidence refs produced by the drill.
    pub evidence_refs: Vec<String>,
}

/// Projection row for a consumer that must read this packet verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreConsumerProjection {
    /// Consumer surface.
    pub consumer_surface: SessionRestoreSurfaceClass,
    /// Consumer module, doc, or fixture ref.
    pub consumer_ref: String,
    /// Whether the consumer reads the shared schema.
    pub consumes_schema_ref: bool,
    /// Whether the consumer reads the placeholder-state matrix.
    pub consumes_placeholder_state_matrix: bool,
    /// Whether the consumer reads the no-hidden-rerun drill corpus.
    pub consumes_no_hidden_rerun_corpus: bool,
    /// Support posture for the consumer projection.
    pub support_class: RestoreSupportClass,
}

/// Summary fields reused by diagnostics, crash screens, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreDiagnosticsSummary {
    /// Human-readable restored summary.
    pub restored_summary: String,
    /// Human-readable not-restored summary.
    pub not_restored_summary: String,
    /// Human-readable topology adjustment summary.
    pub topology_adjustment_summary: String,
    /// Human-readable next-action summary.
    pub action_required_summary: String,
}

/// Support/export projection of the same packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionScopedRestoreFidelitySupportExport {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source packet id.
    pub packet_id: String,
    /// Shared schema ref.
    pub schema_ref: String,
    /// Placeholder matrix rows safe for support export.
    pub placeholder_state_matrix: Vec<PlaceholderStateMatrixRow>,
    /// No-hidden-rerun drill ids included by reference.
    pub no_hidden_rerun_drill_refs: Vec<String>,
    /// Diagnostics summary.
    pub diagnostics_summary: RestoreDiagnosticsSummary,
    /// True when raw command lines, raw output bodies, raw paths, raw URLs, and
    /// secrets are absent from the export.
    pub raw_private_material_excluded: bool,
    /// True when live handles, approval tickets, and authority tokens are absent.
    pub live_authority_excluded: bool,
}

/// Canonical session-scoped restore-fidelity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SessionScopedRestoreFidelityPacket {
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// UTC timestamp for this checked-in packet.
    pub generated_at: String,
    /// Shared schema ref.
    pub schema_ref: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Reviewer artifact ref.
    pub artifact_ref: String,
    /// Fixture corpus directory.
    pub fixture_dir_ref: String,
    /// Controlled restore-fidelity classes and labels.
    pub controlled_restore_fidelity_classes: Vec<RestoreFidelityLabel>,
    /// Placeholder-state matrix consumed by runtime-backed panes.
    pub placeholder_state_matrix: Vec<PlaceholderStateMatrixRow>,
    /// Recovery drills proving no hidden rerun.
    pub no_hidden_rerun_drills: Vec<NoHiddenRerunDrill>,
    /// Consumer projections that must read this packet directly.
    pub consumer_projections: Vec<RestoreConsumerProjection>,
    /// Diagnostics summary shared by chrome and export.
    pub diagnostics_summary: RestoreDiagnosticsSummary,
    /// Support/export projection.
    pub support_export: SessionScopedRestoreFidelitySupportExport,
}

impl SessionScopedRestoreFidelityPacket {
    /// Validates the packet against the no-hidden-rerun and coverage contract.
    pub fn validate(&self) -> Result<(), SessionRestoreFidelityValidationError> {
        let findings = validate_session_scoped_restore_fidelity_packet(self);
        if findings.is_empty() {
            Ok(())
        } else {
            Err(SessionRestoreFidelityValidationError { findings })
        }
    }

    /// Returns true when this packet can back stable restore-fidelity claims.
    pub fn is_stable_claim_ready(&self) -> bool {
        self.validate().is_ok()
    }

    /// Renders a concise support-safe summary.
    pub fn render_plaintext_summary(&self) -> String {
        let rows = self
            .placeholder_state_matrix
            .iter()
            .map(|row| {
                format!(
                    "{}={} ({})",
                    row.surface_class.as_str(),
                    row.restore_state.as_str(),
                    row.restore_fidelity_class.label()
                )
            })
            .collect::<Vec<_>>()
            .join("; ");
        format!(
            "Session-scoped restore fidelity: {}; next action: {}",
            rows, self.diagnostics_summary.action_required_summary
        )
    }
}

/// Controlled restore-fidelity token plus display label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreFidelityLabel {
    /// Stable token.
    pub class: RestoreFidelityClass,
    /// Controlled display label.
    pub label: String,
}

/// Validation finding for a malformed packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRestoreFidelityFinding {
    /// Machine-readable finding code.
    pub code: &'static str,
    /// Row, drill, or consumer target.
    pub target: String,
    /// Reviewer-facing message.
    pub message: String,
}

/// Validation error containing all findings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionRestoreFidelityValidationError {
    /// Findings collected during validation.
    pub findings: Vec<SessionRestoreFidelityFinding>,
}

impl fmt::Display for SessionRestoreFidelityValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "session-scoped restore-fidelity packet has {} finding(s)",
            self.findings.len()
        )
    }
}

impl Error for SessionRestoreFidelityValidationError {}

/// Builds the checked-in session-scoped restore-fidelity packet.
pub fn current_session_scoped_restore_fidelity_packet() -> SessionScopedRestoreFidelityPacket {
    let controlled_restore_fidelity_classes = RestoreFidelityClass::REQUIRED
        .iter()
        .map(|class| RestoreFidelityLabel {
            class: *class,
            label: class.label().to_owned(),
        })
        .collect::<Vec<_>>();

    let placeholder_state_matrix = seeded_placeholder_state_matrix();
    let no_hidden_rerun_drills = seeded_no_hidden_rerun_drills();
    let diagnostics_summary = RestoreDiagnosticsSummary {
        restored_summary:
            "layout, transcripts, checkpoints, static preview evidence, and stale metadata restored"
                .to_owned(),
        not_restored_summary:
            "live terminal, task, debug, notebook, preview, and remote authority not restored"
                .to_owned(),
        topology_adjustment_summary:
            "pane slots preserved; missing, revoked, incompatible, and offline dependencies render placeholder cards"
                .to_owned(),
        action_required_summary:
            "rerun, reconnect, reauthenticate, or repair only after explicit user intent"
                .to_owned(),
    };

    let support_export = SessionScopedRestoreFidelitySupportExport {
        record_kind: SESSION_SCOPED_RESTORE_FIDELITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
        schema_version: SESSION_SCOPED_RESTORE_FIDELITY_SCHEMA_VERSION,
        packet_id: "session-restore-fidelity-stable-seed".to_owned(),
        schema_ref: SESSION_SCOPED_RESTORE_FIDELITY_SCHEMA_REF.to_owned(),
        placeholder_state_matrix: placeholder_state_matrix.clone(),
        no_hidden_rerun_drill_refs: no_hidden_rerun_drills
            .iter()
            .map(|drill| drill.drill_id.clone())
            .collect(),
        diagnostics_summary: diagnostics_summary.clone(),
        raw_private_material_excluded: true,
        live_authority_excluded: true,
    };

    SessionScopedRestoreFidelityPacket {
        record_kind: SESSION_SCOPED_RESTORE_FIDELITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: SESSION_SCOPED_RESTORE_FIDELITY_SCHEMA_VERSION,
        packet_id: "session-restore-fidelity-stable-seed".to_owned(),
        generated_at: "2026-06-06T00:00:00Z".to_owned(),
        schema_ref: SESSION_SCOPED_RESTORE_FIDELITY_SCHEMA_REF.to_owned(),
        doc_ref: SESSION_SCOPED_RESTORE_FIDELITY_DOC_REF.to_owned(),
        artifact_ref: SESSION_SCOPED_RESTORE_FIDELITY_ARTIFACT_REF.to_owned(),
        fixture_dir_ref: SESSION_SCOPED_RESTORE_FIDELITY_FIXTURE_DIR.to_owned(),
        controlled_restore_fidelity_classes,
        placeholder_state_matrix,
        no_hidden_rerun_drills,
        consumer_projections: seeded_consumer_projections(),
        diagnostics_summary,
        support_export,
    }
}

/// Validates a session-scoped restore-fidelity packet.
pub fn validate_session_scoped_restore_fidelity_packet(
    packet: &SessionScopedRestoreFidelityPacket,
) -> Vec<SessionRestoreFidelityFinding> {
    let mut findings = Vec::new();

    if packet.record_kind != SESSION_SCOPED_RESTORE_FIDELITY_PACKET_RECORD_KIND {
        push_finding(
            &mut findings,
            "packet.record_kind",
            &packet.record_kind,
            "record_kind must be session_scoped_restore_fidelity_packet",
        );
    }
    if packet.schema_version != SESSION_SCOPED_RESTORE_FIDELITY_SCHEMA_VERSION {
        push_finding(
            &mut findings,
            "packet.schema_version",
            &packet.packet_id,
            "schema_version must be 1",
        );
    }
    if packet.schema_ref != SESSION_SCOPED_RESTORE_FIDELITY_SCHEMA_REF {
        push_finding(
            &mut findings,
            "packet.schema_ref",
            &packet.schema_ref,
            "packet must reference the shared session-restore-fidelity schema",
        );
    }

    validate_fidelity_labels(&mut findings, packet);
    validate_placeholder_matrix(&mut findings, &packet.placeholder_state_matrix);
    validate_drill_corpus(&mut findings, &packet.no_hidden_rerun_drills);
    validate_consumers(&mut findings, &packet.consumer_projections);
    validate_support_export(&mut findings, packet);

    findings
}

fn validate_fidelity_labels(
    findings: &mut Vec<SessionRestoreFidelityFinding>,
    packet: &SessionScopedRestoreFidelityPacket,
) {
    let label_set = packet
        .controlled_restore_fidelity_classes
        .iter()
        .map(|label| label.class)
        .collect::<BTreeSet<_>>();
    for required in RestoreFidelityClass::REQUIRED {
        if !label_set.contains(&required) {
            push_finding(
                findings,
                "fidelity_class.missing",
                required.as_str(),
                "controlled restore-fidelity class is missing",
            );
        }
    }
    for label in &packet.controlled_restore_fidelity_classes {
        if label.label != label.class.label() {
            push_finding(
                findings,
                "fidelity_class.label",
                label.class.as_str(),
                "controlled restore-fidelity label must render verbatim",
            );
        }
    }
}

fn validate_placeholder_matrix(
    findings: &mut Vec<SessionRestoreFidelityFinding>,
    rows: &[PlaceholderStateMatrixRow],
) {
    let mut covered_surfaces = BTreeSet::new();
    let mut row_ids = BTreeSet::new();

    for row in rows {
        if !row_ids.insert(row.row_id.clone()) {
            push_finding(
                findings,
                "placeholder.duplicate_row_id",
                &row.row_id,
                "placeholder-state row ids must be unique",
            );
        }

        if row.surface_class.is_runtime_backed() {
            covered_surfaces.insert(row.surface_class);
        }

        if row.surface_class.is_runtime_backed() && !row.layout_preserved {
            push_finding(
                findings,
                "placeholder.layout_not_preserved",
                &row.row_id,
                "runtime-backed rows must preserve the surrounding layout slot",
            );
        }

        if row.surface_class.is_runtime_backed() && !row.stale_evidence_visible {
            push_finding(
                findings,
                "placeholder.evidence_not_visible",
                &row.row_id,
                "runtime-backed rows must keep stale evidence or last-known metadata visible",
            );
        }

        if !row.auto_rerun_forbidden {
            push_finding(
                findings,
                "placeholder.auto_rerun_allowed",
                &row.row_id,
                "auto-rerun must be forbidden for session-scoped restore rows",
            );
        }
        if !row.silent_reattach_forbidden {
            push_finding(
                findings,
                "placeholder.silent_reattach_allowed",
                &row.row_id,
                "silent reattach must be forbidden for session-scoped restore rows",
            );
        }
        if !row.hidden_authority_reacquisition_forbidden {
            push_finding(
                findings,
                "placeholder.hidden_authority_allowed",
                &row.row_id,
                "hidden authority reacquisition must be forbidden",
            );
        }

        if !row.restore_state.permits_live_authority() && !row.explicit_user_intent_required {
            push_finding(
                findings,
                "placeholder.missing_user_intent",
                &row.row_id,
                "non-live restore states require explicit user intent before rerun or reconnect",
            );
        }

        if row.explicit_user_intent_required
            && row
                .target_runtime_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
        {
            push_finding(
                findings,
                "placeholder.missing_target_runtime",
                &row.row_id,
                "rows requiring rerun or reconnect must name the target/runtime",
            );
        }

        if row.next_actions.is_empty() {
            push_finding(
                findings,
                "placeholder.missing_next_action",
                &row.row_id,
                "each placeholder row must expose at least one typed next action",
            );
        }

        if row.support_class.is_stable()
            && (!row.auto_rerun_forbidden
                || !row.silent_reattach_forbidden
                || !row.hidden_authority_reacquisition_forbidden)
        {
            push_finding(
                findings,
                "placeholder.stable_guardrail_missing",
                &row.row_id,
                "stable rows must satisfy no-hidden-rerun and no-hidden-authority guardrails",
            );
        }
    }

    for required in SessionRestoreSurfaceClass::RUNTIME_BACKED {
        if !covered_surfaces.contains(&required) {
            push_finding(
                findings,
                "placeholder.required_surface_missing",
                required.as_str(),
                "runtime-backed surface has no placeholder-state matrix row",
            );
        }
    }
}

fn validate_drill_corpus(
    findings: &mut Vec<SessionRestoreFidelityFinding>,
    drills: &[NoHiddenRerunDrill],
) {
    let mut covered_surfaces = BTreeSet::new();
    let mut drill_ids = BTreeSet::new();

    for drill in drills {
        if !drill_ids.insert(drill.drill_id.clone()) {
            push_finding(
                findings,
                "drill.duplicate_id",
                &drill.drill_id,
                "drill ids must be unique",
            );
        }

        if drill.surface_class.is_runtime_backed() {
            covered_surfaces.insert(drill.surface_class);
        }

        if drill.hidden_rerun_attempted {
            push_finding(
                findings,
                "drill.hidden_rerun",
                &drill.drill_id,
                "drill attempted hidden rerun during restore",
            );
        }
        if drill.silent_reattach_attempted {
            push_finding(
                findings,
                "drill.silent_reattach",
                &drill.drill_id,
                "drill attempted silent reattach during restore",
            );
        }
        if drill.authority_reacquired_without_user_intent {
            push_finding(
                findings,
                "drill.hidden_authority",
                &drill.drill_id,
                "drill reacquired live authority without explicit user intent",
            );
        }
        if !drill.target_runtime_named_before_action {
            push_finding(
                findings,
                "drill.target_runtime_not_named",
                &drill.drill_id,
                "drill must show the target/runtime before rerun or reconnect",
            );
        }
        if drill.evidence_refs.is_empty() {
            push_finding(
                findings,
                "drill.evidence_missing",
                &drill.drill_id,
                "drill must retain evidence refs",
            );
        }
    }

    for required in SessionRestoreSurfaceClass::RUNTIME_BACKED {
        if !covered_surfaces.contains(&required) {
            push_finding(
                findings,
                "drill.required_surface_missing",
                required.as_str(),
                "runtime-backed surface has no no-hidden-rerun drill",
            );
        }
    }
}

fn validate_consumers(
    findings: &mut Vec<SessionRestoreFidelityFinding>,
    projections: &[RestoreConsumerProjection],
) {
    let mut covered = BTreeSet::new();

    for projection in projections {
        covered.insert(projection.consumer_surface);
        if !projection.consumes_schema_ref {
            push_finding(
                findings,
                "consumer.schema_not_consumed",
                projection.consumer_surface.as_str(),
                "consumer must read the shared restore-fidelity schema",
            );
        }
        if !projection.consumes_placeholder_state_matrix {
            push_finding(
                findings,
                "consumer.placeholder_matrix_not_consumed",
                projection.consumer_surface.as_str(),
                "consumer must read the placeholder-state matrix",
            );
        }
        if !projection.consumes_no_hidden_rerun_corpus {
            push_finding(
                findings,
                "consumer.drill_corpus_not_consumed",
                projection.consumer_surface.as_str(),
                "consumer must read the no-hidden-rerun corpus",
            );
        }
        if !projection.support_class.is_stable() {
            push_finding(
                findings,
                "consumer.not_stable",
                projection.consumer_surface.as_str(),
                "consumer projection is below stable",
            );
        }
    }

    for required in SessionRestoreSurfaceClass::REQUIRED_CONSUMERS {
        if !covered.contains(&required) {
            push_finding(
                findings,
                "consumer.required_missing",
                required.as_str(),
                "required consumer projection is missing",
            );
        }
    }
}

fn validate_support_export(
    findings: &mut Vec<SessionRestoreFidelityFinding>,
    packet: &SessionScopedRestoreFidelityPacket,
) {
    let export = &packet.support_export;
    if export.record_kind != SESSION_SCOPED_RESTORE_FIDELITY_SUPPORT_EXPORT_RECORD_KIND {
        push_finding(
            findings,
            "support_export.record_kind",
            &export.record_kind,
            "support export record_kind is incorrect",
        );
    }
    if !export.raw_private_material_excluded {
        push_finding(
            findings,
            "support_export.raw_private_material",
            &packet.packet_id,
            "support export must exclude raw private material",
        );
    }
    if !export.live_authority_excluded {
        push_finding(
            findings,
            "support_export.live_authority",
            &packet.packet_id,
            "support export must exclude live authority handles",
        );
    }
    if export.placeholder_state_matrix.len() != packet.placeholder_state_matrix.len() {
        push_finding(
            findings,
            "support_export.placeholder_matrix_mismatch",
            &packet.packet_id,
            "support export must carry the same placeholder matrix rows",
        );
    }
    if export.no_hidden_rerun_drill_refs.len() != packet.no_hidden_rerun_drills.len() {
        push_finding(
            findings,
            "support_export.drill_refs_mismatch",
            &packet.packet_id,
            "support export must include one drill ref per no-hidden-rerun drill",
        );
    }
}

fn seeded_placeholder_state_matrix() -> Vec<PlaceholderStateMatrixRow> {
    vec![
        PlaceholderStateMatrixRow {
            row_id: "terminal-transcript-restored".to_owned(),
            surface_class: SessionRestoreSurfaceClass::TerminalSession,
            restore_fidelity_class: RestoreFidelityClass::EvidenceOnly,
            restore_state: SessionRestoreStateClass::TranscriptRestored,
            dependency_state: RestoreDependencyStateClass::Available,
            layout_preserved: true,
            stale_evidence_visible: true,
            auto_rerun_forbidden: true,
            silent_reattach_forbidden: true,
            hidden_authority_reacquisition_forbidden: true,
            explicit_user_intent_required: true,
            target_runtime_ref: Some("terminal.target.last-known".to_owned()),
            next_actions: vec![
                RestoreNextActionClass::InspectEvidence,
                RestoreNextActionClass::RerunExplicitly,
            ],
            evidence_refs: vec!["fixtures/recovery/stabilize-session-scoped-restore-fidelity/terminal_transcript_restored.json".to_owned()],
            visible_summary: "terminal transcript restored; command not rerun".to_owned(),
            support_class: RestoreSupportClass::Stable,
        },
        PlaceholderStateMatrixRow {
            row_id: "task-rerun-required".to_owned(),
            surface_class: SessionRestoreSurfaceClass::TaskRun,
            restore_fidelity_class: RestoreFidelityClass::LayoutOnly,
            restore_state: SessionRestoreStateClass::RerunRequired,
            dependency_state: RestoreDependencyStateClass::Available,
            layout_preserved: true,
            stale_evidence_visible: true,
            auto_rerun_forbidden: true,
            silent_reattach_forbidden: true,
            hidden_authority_reacquisition_forbidden: true,
            explicit_user_intent_required: true,
            target_runtime_ref: Some("task.target.build-default".to_owned()),
            next_actions: vec![
                RestoreNextActionClass::InspectEvidence,
                RestoreNextActionClass::RerunExplicitly,
            ],
            evidence_refs: vec!["fixtures/recovery/stabilize-session-scoped-restore-fidelity/task_rerun_required.json".to_owned()],
            visible_summary: "task context restored; rerun required".to_owned(),
            support_class: RestoreSupportClass::Stable,
        },
        PlaceholderStateMatrixRow {
            row_id: "debug-session-ended".to_owned(),
            surface_class: SessionRestoreSurfaceClass::DebugSession,
            restore_fidelity_class: RestoreFidelityClass::LayoutOnly,
            restore_state: SessionRestoreStateClass::SessionEnded,
            dependency_state: RestoreDependencyStateClass::Available,
            layout_preserved: true,
            stale_evidence_visible: true,
            auto_rerun_forbidden: true,
            silent_reattach_forbidden: true,
            hidden_authority_reacquisition_forbidden: true,
            explicit_user_intent_required: true,
            target_runtime_ref: Some("debug.target.last-adapter".to_owned()),
            next_actions: vec![
                RestoreNextActionClass::InspectEvidence,
                RestoreNextActionClass::RerunExplicitly,
            ],
            evidence_refs: vec!["fixtures/recovery/stabilize-session-scoped-restore-fidelity/debug_session_ended.json".to_owned()],
            visible_summary: "debug layout restored; adapter not reattached".to_owned(),
            support_class: RestoreSupportClass::Stable,
        },
        PlaceholderStateMatrixRow {
            row_id: "notebook-kernel-rerun-required".to_owned(),
            surface_class: SessionRestoreSurfaceClass::NotebookKernel,
            restore_fidelity_class: RestoreFidelityClass::RecoveredDrafts,
            restore_state: SessionRestoreStateClass::RerunRequired,
            dependency_state: RestoreDependencyStateClass::Offline,
            layout_preserved: true,
            stale_evidence_visible: true,
            auto_rerun_forbidden: true,
            silent_reattach_forbidden: true,
            hidden_authority_reacquisition_forbidden: true,
            explicit_user_intent_required: true,
            target_runtime_ref: Some("notebook.kernel.last-known".to_owned()),
            next_actions: vec![
                RestoreNextActionClass::InspectEvidence,
                RestoreNextActionClass::ReconnectExplicitly,
                RestoreNextActionClass::RerunExplicitly,
            ],
            evidence_refs: vec!["fixtures/recovery/stabilize-session-scoped-restore-fidelity/notebook_kernel_rerun_required.json".to_owned()],
            visible_summary: "notebook outputs and drafts restored; kernel not resumed".to_owned(),
            support_class: RestoreSupportClass::Stable,
        },
        PlaceholderStateMatrixRow {
            row_id: "preview-static-evidence-only".to_owned(),
            surface_class: SessionRestoreSurfaceClass::PreviewServer,
            restore_fidelity_class: RestoreFidelityClass::EvidenceOnly,
            restore_state: SessionRestoreStateClass::StaticEvidenceOnly,
            dependency_state: RestoreDependencyStateClass::Missing,
            layout_preserved: true,
            stale_evidence_visible: true,
            auto_rerun_forbidden: true,
            silent_reattach_forbidden: true,
            hidden_authority_reacquisition_forbidden: true,
            explicit_user_intent_required: true,
            target_runtime_ref: Some("preview.runtime.last-known".to_owned()),
            next_actions: vec![
                RestoreNextActionClass::InspectEvidence,
                RestoreNextActionClass::RepairDependency,
                RestoreNextActionClass::RerunExplicitly,
            ],
            evidence_refs: vec!["fixtures/recovery/stabilize-session-scoped-restore-fidelity/preview_static_evidence_only.json".to_owned()],
            visible_summary: "preview reopened as static evidence only".to_owned(),
            support_class: RestoreSupportClass::Stable,
        },
        PlaceholderStateMatrixRow {
            row_id: "remote-reconnect-available".to_owned(),
            surface_class: SessionRestoreSurfaceClass::RemoteTunnel,
            restore_fidelity_class: RestoreFidelityClass::CompatibleRestore,
            restore_state: SessionRestoreStateClass::ReconnectAvailable,
            dependency_state: RestoreDependencyStateClass::Revoked,
            layout_preserved: true,
            stale_evidence_visible: true,
            auto_rerun_forbidden: true,
            silent_reattach_forbidden: true,
            hidden_authority_reacquisition_forbidden: true,
            explicit_user_intent_required: true,
            target_runtime_ref: Some("remote.target.managed-workspace".to_owned()),
            next_actions: vec![
                RestoreNextActionClass::InspectEvidence,
                RestoreNextActionClass::ReauthenticateThenRetry,
                RestoreNextActionClass::ReconnectExplicitly,
            ],
            evidence_refs: vec!["fixtures/recovery/stabilize-session-scoped-restore-fidelity/remote_reconnect_available.json".to_owned()],
            visible_summary: "remote context restored; reconnect available after review".to_owned(),
            support_class: RestoreSupportClass::Stable,
        },
    ]
}

fn seeded_no_hidden_rerun_drills() -> Vec<NoHiddenRerunDrill> {
    seeded_placeholder_state_matrix()
        .into_iter()
        .map(|row| NoHiddenRerunDrill {
            drill_id: format!("drill-{}", row.row_id),
            surface_class: row.surface_class,
            condition: "restart, crash recovery, or display-topology change".to_owned(),
            expected_restore_state: row.restore_state,
            hidden_rerun_attempted: false,
            silent_reattach_attempted: false,
            authority_reacquired_without_user_intent: false,
            target_runtime_named_before_action: row.target_runtime_ref.is_some(),
            evidence_refs: row.evidence_refs,
        })
        .collect()
}

fn seeded_consumer_projections() -> Vec<RestoreConsumerProjection> {
    SessionRestoreSurfaceClass::REQUIRED_CONSUMERS
        .iter()
        .map(|surface| RestoreConsumerProjection {
            consumer_surface: *surface,
            consumer_ref: match surface {
                SessionRestoreSurfaceClass::TerminalSession => {
                    "crates/aureline-terminal/src/restore"
                }
                SessionRestoreSurfaceClass::TaskRun => "crates/aureline-runtime/task-surfaces",
                SessionRestoreSurfaceClass::DebugSession => "crates/aureline-debug",
                SessionRestoreSurfaceClass::NotebookKernel => "crates/aureline-notebook",
                SessionRestoreSurfaceClass::PreviewServer => "crates/aureline-preview",
                SessionRestoreSurfaceClass::RemoteTunnel => "crates/aureline-remote",
                SessionRestoreSurfaceClass::Diagnostics => "crates/aureline-shell/src/diagnostics",
                SessionRestoreSurfaceClass::SupportExport => "crates/aureline-support",
            }
            .to_owned(),
            consumes_schema_ref: true,
            consumes_placeholder_state_matrix: true,
            consumes_no_hidden_rerun_corpus: true,
            support_class: RestoreSupportClass::Stable,
        })
        .collect()
}

fn push_finding(
    findings: &mut Vec<SessionRestoreFidelityFinding>,
    code: &'static str,
    target: impl Into<String>,
    message: impl Into<String>,
) {
    findings.push(SessionRestoreFidelityFinding {
        code,
        target: target.into(),
        message: message.into(),
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_validates_and_covers_all_consumers() {
        let packet = current_session_scoped_restore_fidelity_packet();

        assert!(packet.validate().is_ok());
        assert!(packet.is_stable_claim_ready());

        let consumers = packet
            .consumer_projections
            .iter()
            .map(|projection| projection.consumer_surface)
            .collect::<BTreeSet<_>>();
        for required in SessionRestoreSurfaceClass::REQUIRED_CONSUMERS {
            assert!(consumers.contains(&required), "missing {required:?}");
        }
    }

    #[test]
    fn checked_in_packet_fixture_matches_rust_shape() {
        let packet: SessionScopedRestoreFidelityPacket = serde_json::from_str(include_str!(
            "../../../../fixtures/recovery/stabilize-session-scoped-restore-fidelity/session_restore_fidelity_packet.json"
        ))
        .expect("fixture parses as session-scoped restore-fidelity packet");

        packet.validate().expect("fixture validates");
        assert_eq!(
            packet.placeholder_state_matrix.len(),
            packet.support_export.placeholder_state_matrix.len()
        );
    }

    #[test]
    fn seeded_matrix_forbids_hidden_runtime_continuity() {
        let packet = current_session_scoped_restore_fidelity_packet();

        for row in &packet.placeholder_state_matrix {
            assert!(row.auto_rerun_forbidden);
            assert!(row.silent_reattach_forbidden);
            assert!(row.hidden_authority_reacquisition_forbidden);
            assert!(row.stale_evidence_visible);
            assert!(row.target_runtime_ref.is_some());
            assert_ne!(
                row.restore_state,
                SessionRestoreStateClass::LiveRuntimeSurvived
            );
        }

        for drill in &packet.no_hidden_rerun_drills {
            assert!(!drill.hidden_rerun_attempted);
            assert!(!drill.silent_reattach_attempted);
            assert!(!drill.authority_reacquired_without_user_intent);
            assert!(drill.target_runtime_named_before_action);
        }
    }

    #[test]
    fn validation_rejects_stable_row_that_allows_hidden_rerun() {
        let mut packet = current_session_scoped_restore_fidelity_packet();
        packet.placeholder_state_matrix[0].auto_rerun_forbidden = false;

        let findings = validate_session_scoped_restore_fidelity_packet(&packet);

        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "placeholder.auto_rerun_allowed"),
            "{findings:?}"
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.code == "placeholder.stable_guardrail_missing"),
            "{findings:?}"
        );
    }
}
