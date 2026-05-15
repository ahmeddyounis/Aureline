//! First-run and imported-profile task-success packets for beta switching rows.
//!
//! The packet pins one bounded measurement projection per claimed beta
//! switching flow so M3 can prove task success and not only ship the UI.
//! Two flows are in scope:
//!
//! - the no-account first-run path that opens a local folder through the
//!   Start Center; and
//! - the imported-profile path that reviews a VS Code settings packet,
//!   mints a rollback checkpoint, and commits per-item outcomes.
//!
//! For every flow the packet enumerates four task-success states:
//!
//! - [`TaskSuccessState::Completion`] — the path reached first-useful-work
//!   or applied the migration per-item without degradation;
//! - [`TaskSuccessState::Fallback`] — the user declined the offered
//!   optional path (sign-in, managed sync, profile widening) and the
//!   flow stayed local-only without silent narrowing;
//! - [`TaskSuccessState::Abandonment`] — the user dropped out before or
//!   after admission so dashboards can keep abandonment off the
//!   completion line; and
//! - [`TaskSuccessState::RepairRequired`] — a typed blocker (forced
//!   sign-in, missing rollback checkpoint, …) prevented completion and
//!   requires an explicit repair action.
//!
//! Each row carries the typed completion/failure metadata, an explicit
//! repair-action token when a repair is required, and refs into the
//! onboarding telemetry capture that backs it. The capture is composed
//! through [`aureline_telemetry::onboarding`] so the packet inherits the
//! metadata-safe-default privacy envelope: raw project content, file
//! paths, prompt or terminal text, and credentials are prohibited and
//! never present.
//!
//! The packet is consumed by the live shell, the headless inspector
//! (`aureline_shell_onboarding_metrics`), the support-export wrapper,
//! and the markdown packet checked in under
//! `artifacts/ux/m3/first_run_task_success_packet.md`. The seeded
//! projection is deterministic so the checked-in fixtures under
//! `fixtures/ux/first_run_task_success_packet/` are bit-for-bit equal
//! to the output of [`seeded_first_run_task_success_packet`].

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use aureline_telemetry::onboarding::{
    CompletionCheckpointClass, CompletionClass, DryRunState, EntryFlowDescriptor, EntryFlowKind,
    EntryRouteId, EntryVerbKind, FailureCategory, FirstUsefulWorkTargetSurface,
    FirstUsefulWorkTiming, MeasurementSurface, MigrationFunnelRecord, MigrationFunnelStep,
    MigrationOutcomeCounts, MigrationSourceKind, OnboardingEventInput, OnboardingEventName,
    OnboardingEventPhase, OnboardingTaskSuccessCaptureRecord, OnboardingTaskSuccessRecorder,
    OnboardingTelemetryContext, OnboardingTelemetryValidationError, OutcomeClass,
    ProhibitedContentClass, RollbackState, SemanticWarmupState, TargetKind,
    TelemetryExportPosture, TelemetryPrivacyClass,
};
use aureline_telemetry::trace_event::BuildIdentityRecord;

/// Schema version exported with every packet record.
pub const FIRST_RUN_TASK_SUCCESS_PACKET_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every packet row.
pub const FIRST_RUN_TASK_SUCCESS_PACKET_SHARED_CONTRACT_REF: &str =
    "shell:first_run_task_success_packet_beta:v1";

/// Stable record kind for [`FirstRunTaskSuccessPacket`] payloads.
pub const FIRST_RUN_TASK_SUCCESS_PACKET_RECORD_KIND: &str =
    "shell_first_run_task_success_packet_beta_record";

/// Stable record kind for [`FirstRunTaskSuccessRow`] payloads.
pub const FIRST_RUN_TASK_SUCCESS_ROW_RECORD_KIND: &str =
    "shell_first_run_task_success_packet_beta_row_record";

/// Stable record kind for [`FirstRunTaskSuccessSupportExport`] payloads.
pub const FIRST_RUN_TASK_SUCCESS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_first_run_task_success_packet_beta_support_export_record";

/// Stable packet id used to pivot across surfaces.
pub const FIRST_RUN_TASK_SUCCESS_PACKET_ID: &str =
    "shell:first_run_task_success_packet_beta:v1:default";

/// Deterministic generated-at value carried by the seeded packet.
const GENERATED_AT: &str = "2026-05-15T00:00:00Z";

/// Claimed beta switching flow this packet measures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SwitchingFlowKind {
    /// No-account first-run path that opens a local folder through the
    /// Start Center.
    FirstRun,
    /// Imported-profile path that reviews a VS Code settings packet,
    /// mints a rollback checkpoint, and commits per-item outcomes.
    ImportedProfile,
}

impl SwitchingFlowKind {
    /// Returns the stable schema token for this flow.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRun => "first_run",
            Self::ImportedProfile => "imported_profile",
        }
    }

    /// Returns the reviewer-facing label for this flow.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::FirstRun => "First run (no account, local folder)",
            Self::ImportedProfile => "Imported profile (VS Code settings)",
        }
    }

    /// Returns the two switching flows in canonical order.
    pub const fn required_flows() -> [Self; 2] {
        [Self::FirstRun, Self::ImportedProfile]
    }
}

/// Bounded task-success state captured by the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskSuccessState {
    /// Flow reached first-useful-work or applied per-item outcomes.
    Completion,
    /// User declined an optional path; flow continued without degradation.
    Fallback,
    /// User dropped out before or after admission.
    Abandonment,
    /// A typed blocker requires explicit repair.
    RepairRequired,
}

impl TaskSuccessState {
    /// Returns the stable schema token for this state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completion => "completion",
            Self::Fallback => "fallback",
            Self::Abandonment => "abandonment",
            Self::RepairRequired => "repair_required",
        }
    }

    /// Returns the reviewer-facing label for this state.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Completion => "Completion",
            Self::Fallback => "Fallback",
            Self::Abandonment => "Abandonment",
            Self::RepairRequired => "Repair required",
        }
    }

    /// Returns the four states in canonical order.
    pub const fn required_states() -> [Self; 4] {
        [
            Self::Completion,
            Self::Fallback,
            Self::Abandonment,
            Self::RepairRequired,
        ]
    }
}

/// Stable repair-action token surfaced when a row reports
/// [`TaskSuccessState::RepairRequired`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairActionToken {
    /// Retry the same flow from the entry surface.
    RetryEntry,
    /// Re-issue admission with explicit policy review.
    ReissueAdmissionReview,
    /// Mint a fresh rollback checkpoint before apply.
    MintRollbackCheckpoint,
    /// Re-run the migration dry-run before applying.
    RerunMigrationDryRun,
    /// Restore from an existing rollback checkpoint.
    RestoreFromCheckpoint,
    /// Open the migration mapping report to review per-item rows.
    OpenMappingReport,
    /// Export the support bundle for partner-facing escalation.
    ExportForSupport,
}

impl RepairActionToken {
    /// Returns the stable schema token for this action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RetryEntry => "retry_entry",
            Self::ReissueAdmissionReview => "reissue_admission_review",
            Self::MintRollbackCheckpoint => "mint_rollback_checkpoint",
            Self::RerunMigrationDryRun => "rerun_migration_dry_run",
            Self::RestoreFromCheckpoint => "restore_from_checkpoint",
            Self::OpenMappingReport => "open_mapping_report",
            Self::ExportForSupport => "export_for_support",
        }
    }
}

/// One classified task-success row inside the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstRunTaskSuccessRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable row id quoted across surfaces.
    pub row_id: String,
    /// Switching flow that owns the row.
    pub flow_kind: SwitchingFlowKind,
    /// Task-success state captured by the row.
    pub state: TaskSuccessState,
    /// Measurement surface the row binds to.
    pub measurement_surface: MeasurementSurface,
    /// Entry route id the row was admitted through.
    pub entry_route_id: EntryRouteId,
    /// Entry verb selected by the user.
    pub entry_verb: EntryVerbKind,
    /// Privacy-safe target class for the row.
    pub target_kind: TargetKind,
    /// Completion checkpoint when the row reached completion.
    pub completion_checkpoint_class: Option<CompletionCheckpointClass>,
    /// Completion class for [`TaskSuccessState::Completion`] and
    /// [`TaskSuccessState::Fallback`] rows.
    pub completion_class: Option<CompletionClass>,
    /// Typed failure category for [`TaskSuccessState::Abandonment`] and
    /// [`TaskSuccessState::RepairRequired`] rows.
    pub failure_category: Option<FailureCategory>,
    /// Outcome class echoed by the telemetry capture for this row.
    pub outcome_class: OutcomeClass,
    /// Repair-action token surfaced when the row requires repair.
    pub repair_action_token: Option<RepairActionToken>,
    /// True when no raw sensitive user content is captured for this row.
    pub no_raw_sensitive_user_content: bool,
    /// Event names emitted into the telemetry capture for this row.
    pub telemetry_event_names: Vec<OnboardingEventName>,
    /// Docs/help refs that publish the row.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs that retain the row in support evidence.
    pub support_export_refs: Vec<String>,
    /// Partner scorecard refs that consume this row.
    pub partner_scorecard_refs: Vec<String>,
    /// Reviewer-facing narrative summary.
    pub narrative: String,
}

/// Per-flow counts grouped by [`TaskSuccessState`].
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PerFlowStateCounts {
    /// Completion-row count for the flow.
    pub completion: usize,
    /// Fallback-row count for the flow.
    pub fallback: usize,
    /// Abandonment-row count for the flow.
    pub abandonment: usize,
    /// Repair-required-row count for the flow.
    pub repair_required: usize,
}

impl PerFlowStateCounts {
    fn record(&mut self, state: TaskSuccessState) {
        match state {
            TaskSuccessState::Completion => self.completion += 1,
            TaskSuccessState::Fallback => self.fallback += 1,
            TaskSuccessState::Abandonment => self.abandonment += 1,
            TaskSuccessState::RepairRequired => self.repair_required += 1,
        }
    }

    /// Returns the total number of rows recorded for this flow.
    pub const fn total(&self) -> usize {
        self.completion + self.fallback + self.abandonment + self.repair_required
    }

    /// Returns `true` when every required state is covered at least once.
    pub const fn covers_every_state(&self) -> bool {
        self.completion > 0 && self.fallback > 0 && self.abandonment > 0 && self.repair_required > 0
    }
}

/// State coverage summary across flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketStateSummary {
    /// Per-flow counts for [`SwitchingFlowKind::FirstRun`].
    pub first_run: PerFlowStateCounts,
    /// Per-flow counts for [`SwitchingFlowKind::ImportedProfile`].
    pub imported_profile: PerFlowStateCounts,
}

impl PacketStateSummary {
    fn from_rows(rows: &[FirstRunTaskSuccessRow]) -> Self {
        let mut summary = Self {
            first_run: PerFlowStateCounts::default(),
            imported_profile: PerFlowStateCounts::default(),
        };
        for row in rows {
            match row.flow_kind {
                SwitchingFlowKind::FirstRun => summary.first_run.record(row.state),
                SwitchingFlowKind::ImportedProfile => summary.imported_profile.record(row.state),
            }
        }
        summary
    }

    /// Returns `true` when every (flow, state) cell is covered at least once.
    pub const fn covers_every_required_cell(&self) -> bool {
        self.first_run.covers_every_state() && self.imported_profile.covers_every_state()
    }
}

/// First-run task-success packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstRunTaskSuccessPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// Classified rows in deterministic order.
    pub rows: Vec<FirstRunTaskSuccessRow>,
    /// State coverage summary across the rows.
    pub state_summary: PacketStateSummary,
    /// Onboarding telemetry capture that backs the rows.
    pub telemetry_capture: OnboardingTaskSuccessCaptureRecord,
    /// True when the telemetry capture's privacy envelope prohibits raw
    /// sensitive user content for every row in the packet.
    pub no_raw_sensitive_user_content: bool,
    /// Partner scorecard refs that consume the packet.
    pub partner_scorecard_refs: Vec<String>,
    /// Beta-readiness review refs that consume the packet.
    pub readiness_review_refs: Vec<String>,
    /// Markdown packet artifact that publishes the rows.
    pub published_packet_ref: String,
    /// Docs/help refs the packet reopens from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the packet reopens from.
    pub support_export_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl FirstRunTaskSuccessPacket {
    /// Returns `true` when every required (flow, state) cell is present.
    pub const fn covers_every_required_cell(&self) -> bool {
        self.state_summary.covers_every_required_cell()
    }

    /// Returns the row count for the packet.
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "packet: id={}, rows={}, captures={}",
            self.packet_id,
            self.rows.len(),
            self.telemetry_capture.events.len()
        ));
        lines.push(format!(
            "first_run: completion={}, fallback={}, abandonment={}, repair_required={}",
            self.state_summary.first_run.completion,
            self.state_summary.first_run.fallback,
            self.state_summary.first_run.abandonment,
            self.state_summary.first_run.repair_required,
        ));
        lines.push(format!(
            "imported_profile: completion={}, fallback={}, abandonment={}, repair_required={}",
            self.state_summary.imported_profile.completion,
            self.state_summary.imported_profile.fallback,
            self.state_summary.imported_profile.abandonment,
            self.state_summary.imported_profile.repair_required,
        ));
        for row in &self.rows {
            lines.push(format!(
                "{}: {} [{}] outcome={}",
                row.flow_kind.as_str(),
                row.state.as_str(),
                row.row_id,
                outcome_class_token(row.outcome_class),
            ));
        }
        lines
    }

    /// Renders the markdown artifact for the packet.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# First-run task-success packet (beta)\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::onboarding_metrics`](../../../crates/aureline-shell/src/onboarding_metrics/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- markdown > \\\n  artifacts/ux/m3/first_run_task_success_packet.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Telemetry capture: `{}`\n",
            self.telemetry_capture.capture_id
        ));
        out.push_str(&format!("- Rows: {}\n", self.rows.len()));
        out.push_str(&format!(
            "- No raw sensitive user content: {}\n",
            self.no_raw_sensitive_user_content
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## State coverage\n\n");
        out.push_str("| Flow | Completion | Fallback | Abandonment | Repair required | Total |\n");
        out.push_str("|---|---:|---:|---:|---:|---:|\n");
        out.push_str(&format!(
            "| First run | {} | {} | {} | {} | {} |\n",
            self.state_summary.first_run.completion,
            self.state_summary.first_run.fallback,
            self.state_summary.first_run.abandonment,
            self.state_summary.first_run.repair_required,
            self.state_summary.first_run.total(),
        ));
        out.push_str(&format!(
            "| Imported profile | {} | {} | {} | {} | {} |\n\n",
            self.state_summary.imported_profile.completion,
            self.state_summary.imported_profile.fallback,
            self.state_summary.imported_profile.abandonment,
            self.state_summary.imported_profile.repair_required,
            self.state_summary.imported_profile.total(),
        ));

        for flow in SwitchingFlowKind::required_flows() {
            out.push_str(&format!(
                "## {} (`{}`)\n\n",
                flow.display_label(),
                flow.as_str()
            ));
            out.push_str(
                "| State | Row | Outcome | Repair | Completion class | Failure category |\n",
            );
            out.push_str("|---|---|---|---|---|---|\n");
            for row in self.rows.iter().filter(|row| row.flow_kind == flow) {
                let completion = row
                    .completion_class
                    .map(completion_class_token)
                    .unwrap_or("—");
                let failure = row
                    .failure_category
                    .map(failure_category_token)
                    .unwrap_or("—");
                let repair = row
                    .repair_action_token
                    .map(|token| token.as_str())
                    .unwrap_or("—");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    row.state.display_label(),
                    row.row_id,
                    outcome_class_token(row.outcome_class),
                    repair,
                    completion,
                    failure,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Privacy envelope\n\n");
        out.push_str(&format!(
            "- Privacy class: `{}`\n",
            privacy_class_token(self.telemetry_capture.privacy.privacy_class)
        ));
        out.push_str(&format!(
            "- Export posture: `{}`\n",
            export_posture_token(self.telemetry_capture.privacy.export_posture)
        ));
        out.push_str(
            "- Prohibited content classes: raw_project_content, raw_repo_name, file_path,\n  raw_url, prompt_text, terminal_text, clipboard_content, credential_or_secret\n\n",
        );

        out.push_str("## Partner scorecards\n\n");
        for partner in &self.partner_scorecard_refs {
            out.push_str(&format!("- `{partner}`\n"));
        }
        if self.partner_scorecard_refs.is_empty() {
            out.push_str("- (none)\n");
        }
        out.push('\n');

        out.push_str("## Beta readiness reviews\n\n");
        for review in &self.readiness_review_refs {
            out.push_str(&format!("- `{review}`\n"));
        }
        if self.readiness_review_refs.is_empty() {
            out.push_str("- (none)\n");
        }
        out.push('\n');

        out
    }
}

/// Support-export wrapper that quotes the packet plus every stable id
/// reviewers need to pivot across surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FirstRunTaskSuccessSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the wrapper.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: FirstRunTaskSuccessPacket,
    /// Stable packet id, row ids, telemetry capture id, telemetry event
    /// ids, partner scorecard refs, and readiness review refs in
    /// deterministic order.
    pub case_ids: Vec<String>,
}

impl FirstRunTaskSuccessSupportExport {
    /// Builds the support-export wrapper for a packet.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: FirstRunTaskSuccessPacket,
    ) -> Self {
        let mut case_ids = Vec::new();
        case_ids.push(packet.packet_id.clone());
        case_ids.push(packet.telemetry_capture.capture_id.clone());
        for row in &packet.rows {
            case_ids.push(row.row_id.clone());
        }
        for event in &packet.telemetry_capture.events {
            case_ids.push(event.event_id.clone());
        }
        for scorecard in &packet.partner_scorecard_refs {
            case_ids.push(scorecard.clone());
        }
        for review in &packet.readiness_review_refs {
            case_ids.push(review.clone());
        }
        Self {
            record_kind: FIRST_RUN_TASK_SUCCESS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: FIRST_RUN_TASK_SUCCESS_PACKET_SCHEMA_VERSION,
            shared_contract_ref: FIRST_RUN_TASK_SUCCESS_PACKET_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            case_ids,
        }
    }
}

/// Validation error produced by [`validate_first_run_task_success_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum FirstRunTaskSuccessValidationError {
    /// A required (flow, state) cell has no rows in the packet.
    MissingRequiredCell {
        /// Switching flow with the missing cell.
        flow: String,
        /// Task-success state with the missing cell.
        state: String,
    },
    /// A row declares it captures raw sensitive user content.
    RawSensitiveContentDeclared {
        /// Row that violated the privacy invariant.
        row_id: String,
    },
    /// The telemetry capture's privacy envelope does not prohibit raw
    /// project content.
    TelemetryEnvelopeAllowsRawContent,
    /// The telemetry capture's privacy class is not local-only-no-emission
    /// or opt-in.
    TelemetryEnvelopeDefaultEmits,
    /// A telemetry event referenced by a row is missing from the capture.
    TelemetryEventMissing {
        /// Row whose event reference could not be resolved.
        row_id: String,
        /// Event name that was missing.
        event_name: String,
    },
    /// A row reports [`TaskSuccessState::RepairRequired`] without a
    /// repair-action token.
    RepairActionMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// A row reports [`TaskSuccessState::RepairRequired`] or
    /// [`TaskSuccessState::Abandonment`] without a typed failure
    /// category.
    FailureCategoryMissing {
        /// Row that violated the invariant.
        row_id: String,
        /// State that requires a failure category.
        state: String,
    },
    /// A row reports [`TaskSuccessState::Completion`] without a
    /// completion checkpoint class.
    CompletionCheckpointMissing {
        /// Row that violated the invariant.
        row_id: String,
    },
    /// The state summary does not match the rows.
    StateSummaryStale,
    /// The packet does not declare a partner scorecard ref.
    PartnerScorecardMissing,
    /// The packet does not declare a beta readiness review ref.
    ReadinessReviewMissing,
}

/// Validates a packet against the M3 acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_first_run_task_success_packet(
    packet: &FirstRunTaskSuccessPacket,
) -> Result<(), Vec<FirstRunTaskSuccessValidationError>> {
    let mut errors = Vec::new();

    let summary = PacketStateSummary::from_rows(&packet.rows);
    if summary != packet.state_summary {
        errors.push(FirstRunTaskSuccessValidationError::StateSummaryStale);
    }

    for flow in SwitchingFlowKind::required_flows() {
        for state in TaskSuccessState::required_states() {
            let observed = packet
                .rows
                .iter()
                .any(|row| row.flow_kind == flow && row.state == state);
            if !observed {
                errors.push(FirstRunTaskSuccessValidationError::MissingRequiredCell {
                    flow: flow.as_str().to_owned(),
                    state: state.as_str().to_owned(),
                });
            }
        }
    }

    if packet.telemetry_capture.privacy.contains_raw_project_content {
        errors.push(FirstRunTaskSuccessValidationError::TelemetryEnvelopeAllowsRawContent);
    }
    if !packet
        .telemetry_capture
        .privacy
        .prohibited_content_classes
        .contains(&ProhibitedContentClass::RawProjectContent)
    {
        errors.push(FirstRunTaskSuccessValidationError::TelemetryEnvelopeAllowsRawContent);
    }
    if !matches!(
        packet.telemetry_capture.privacy.privacy_class,
        TelemetryPrivacyClass::PrivacyLocalOnlyNoEmission
            | TelemetryPrivacyClass::PrivacyOptInAggregateOnly
    ) {
        errors.push(FirstRunTaskSuccessValidationError::TelemetryEnvelopeDefaultEmits);
    }

    let capture_event_names = packet
        .telemetry_capture
        .events
        .iter()
        .map(|event| event.event_name)
        .collect::<BTreeSet<_>>();

    for row in &packet.rows {
        if !row.no_raw_sensitive_user_content {
            errors.push(
                FirstRunTaskSuccessValidationError::RawSensitiveContentDeclared {
                    row_id: row.row_id.clone(),
                },
            );
        }
        match row.state {
            TaskSuccessState::Completion => {
                if row.completion_checkpoint_class.is_none() {
                    errors.push(
                        FirstRunTaskSuccessValidationError::CompletionCheckpointMissing {
                            row_id: row.row_id.clone(),
                        },
                    );
                }
            }
            TaskSuccessState::Fallback => {
                // Fallback rows must report the typed "decline continued
                // without degradation" completion class so dashboards do
                // not silently fold decline into completion.
            }
            TaskSuccessState::Abandonment | TaskSuccessState::RepairRequired => {
                if row.failure_category.is_none() {
                    errors.push(FirstRunTaskSuccessValidationError::FailureCategoryMissing {
                        row_id: row.row_id.clone(),
                        state: row.state.as_str().to_owned(),
                    });
                }
                if row.state == TaskSuccessState::RepairRequired
                    && row.repair_action_token.is_none()
                {
                    errors.push(FirstRunTaskSuccessValidationError::RepairActionMissing {
                        row_id: row.row_id.clone(),
                    });
                }
            }
        }

        for event_name in &row.telemetry_event_names {
            if !capture_event_names.contains(event_name) {
                errors.push(FirstRunTaskSuccessValidationError::TelemetryEventMissing {
                    row_id: row.row_id.clone(),
                    event_name: serde_event_name_token(*event_name).to_owned(),
                });
            }
        }
    }

    if packet.partner_scorecard_refs.is_empty() {
        errors.push(FirstRunTaskSuccessValidationError::PartnerScorecardMissing);
    }
    if packet.readiness_review_refs.is_empty() {
        errors.push(FirstRunTaskSuccessValidationError::ReadinessReviewMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Builds the seeded first-run task-success packet.
///
/// # Panics
/// Panics if the deterministic input violates a privacy invariant in
/// [`aureline_telemetry::onboarding`].
pub fn seeded_first_run_task_success_packet() -> FirstRunTaskSuccessPacket {
    let context = packet_telemetry_context();
    let mut recorder = OnboardingTaskSuccessRecorder::new(context);
    let row_inputs = seeded_row_inputs();
    let mut rows = Vec::with_capacity(row_inputs.len());

    for input in row_inputs {
        for event in &input.events {
            recorder
                .record_event(event.clone())
                .expect("seeded telemetry event must validate");
        }
        rows.push(build_row(&input));
    }

    let capture = recorder.capture(
        "capture:first-run-task-success-packet-beta",
        GENERATED_AT.to_owned(),
    );

    let state_summary = PacketStateSummary::from_rows(&rows);

    FirstRunTaskSuccessPacket {
        record_kind: FIRST_RUN_TASK_SUCCESS_PACKET_RECORD_KIND.to_owned(),
        schema_version: FIRST_RUN_TASK_SUCCESS_PACKET_SCHEMA_VERSION,
        shared_contract_ref: FIRST_RUN_TASK_SUCCESS_PACKET_SHARED_CONTRACT_REF.to_owned(),
        packet_id: FIRST_RUN_TASK_SUCCESS_PACKET_ID.to_owned(),
        headline: "First-run and imported-profile task-success packet for beta switching rows."
            .to_owned(),
        rows,
        state_summary,
        telemetry_capture: capture,
        no_raw_sensitive_user_content: true,
        partner_scorecard_refs: vec![
            "partner-scorecard:beta-readiness:first_run".to_owned(),
            "partner-scorecard:beta-readiness:imported_profile".to_owned(),
        ],
        readiness_review_refs: vec![
            "readiness-review:beta:m3:switching_rows".to_owned(),
            "readiness-review:beta:m3:onboarding_measurement".to_owned(),
        ],
        published_packet_ref: "artifacts/ux/m3/first_run_task_success_packet.md".to_owned(),
        docs_help_refs: vec![
            "docs/ux/m3/first_run_task_success_packet.md".to_owned(),
            "docs/product/onboarding_measurement_plan.md".to_owned(),
        ],
        support_export_refs: vec![
            "support:export.include_first_run_task_success_packet".to_owned(),
        ],
        generated_at: GENERATED_AT.to_owned(),
    }
}

struct RowSeed {
    row_id: &'static str,
    flow: SwitchingFlowKind,
    state: TaskSuccessState,
    measurement_surface: MeasurementSurface,
    entry_route_id: EntryRouteId,
    entry_verb: EntryVerbKind,
    target_kind: TargetKind,
    completion_checkpoint_class: Option<CompletionCheckpointClass>,
    completion_class: Option<CompletionClass>,
    failure_category: Option<FailureCategory>,
    outcome_class: OutcomeClass,
    repair_action_token: Option<RepairActionToken>,
    docs_help_refs: &'static [&'static str],
    support_export_refs: &'static [&'static str],
    partner_scorecard_refs: &'static [&'static str],
    narrative: &'static str,
    events: Vec<OnboardingEventInput>,
}

fn seeded_row_inputs() -> Vec<RowSeed> {
    let mut rows = Vec::new();

    // ---- First-run flow ---------------------------------------------

    rows.push(RowSeed {
        row_id: "row:first_run.completion.start_center_local_folder",
        flow: SwitchingFlowKind::FirstRun,
        state: TaskSuccessState::Completion,
        measurement_surface: MeasurementSurface::SurfaceFirstRun,
        entry_route_id: EntryRouteId::StartCenter,
        entry_verb: EntryVerbKind::OpenFolder,
        target_kind: TargetKind::LocalFolder,
        completion_checkpoint_class: Some(CompletionCheckpointClass::FirstUsefulEdit),
        completion_class: Some(CompletionClass::CompletedFirstUsefulEdit),
        failure_category: None,
        outcome_class: OutcomeClass::Completed,
        repair_action_token: None,
        docs_help_refs: &[
            "docs/ux/m3/first_run_task_success_packet.md",
            "docs/help/start_center_open_folder.md",
        ],
        support_export_refs: &[
            "support:export.include_first_run_task_success_packet",
        ],
        partner_scorecard_refs: &["partner-scorecard:beta-readiness:first_run"],
        narrative: "Start Center opens a local folder, reaches a first useful edit before semantic warm-up, and stays useful with no account.",
        events: vec![
            event(
                first_run_entry(),
                OnboardingEventName::FirstRunAdmitted,
                OnboardingEventPhase::Admission,
                OutcomeClass::Completed,
                None,
                None,
                None,
                40,
            ),
            event(
                first_run_entry(),
                OnboardingEventName::FirstUsefulEditDurable,
                OnboardingEventPhase::UsefulWork,
                OutcomeClass::Completed,
                Some(CompletionCheckpointClass::FirstUsefulEdit),
                Some(CompletionClass::CompletedFirstUsefulEdit),
                Some(FirstUsefulWorkTiming::new(
                    20,
                    62,
                    FirstUsefulWorkTargetSurface::TreePlusReadmeOrChangedFiles,
                    SemanticWarmupState::BeforeSemanticWarmup,
                )),
                62,
            ),
        ],
    });

    rows.push(RowSeed {
        row_id: "row:first_run.fallback.managed_sign_in_declined",
        flow: SwitchingFlowKind::FirstRun,
        state: TaskSuccessState::Fallback,
        measurement_surface: MeasurementSurface::SurfaceOptInBoundary,
        entry_route_id: EntryRouteId::StartCenter,
        entry_verb: EntryVerbKind::OpenFolder,
        target_kind: TargetKind::LocalFolder,
        completion_checkpoint_class: Some(
            CompletionCheckpointClass::DeclineContinuedWithoutDegradation,
        ),
        completion_class: Some(CompletionClass::CompletedDeclineWithoutDegradation),
        failure_category: None,
        outcome_class: OutcomeClass::Completed,
        repair_action_token: None,
        docs_help_refs: &[
            "docs/ux/m3/first_run_task_success_packet.md",
            "docs/help/managed_sign_in_decline.md",
        ],
        support_export_refs: &[
            "support:export.include_first_run_task_success_packet",
        ],
        partner_scorecard_refs: &["partner-scorecard:beta-readiness:first_run"],
        narrative: "User declined managed sign-in at the opt-in boundary and continued local work without silent narrowing.",
        events: vec![
            event(
                first_run_entry(),
                OnboardingEventName::FirstRunAdmitted,
                OnboardingEventPhase::Admission,
                OutcomeClass::Completed,
                Some(CompletionCheckpointClass::DeclineContinuedWithoutDegradation),
                Some(CompletionClass::CompletedDeclineWithoutDegradation),
                None,
                80,
            ),
        ],
    });

    rows.push(RowSeed {
        row_id: "row:first_run.abandonment.dropped_before_admission",
        flow: SwitchingFlowKind::FirstRun,
        state: TaskSuccessState::Abandonment,
        measurement_surface: MeasurementSurface::SurfaceFirstRun,
        entry_route_id: EntryRouteId::StartCenter,
        entry_verb: EntryVerbKind::OpenFolder,
        target_kind: TargetKind::Unknown,
        completion_checkpoint_class: None,
        completion_class: Some(CompletionClass::AbortedBeforeAdmission),
        failure_category: Some(FailureCategory::AdmissionDeniedTrust),
        outcome_class: OutcomeClass::Abandoned,
        repair_action_token: None,
        docs_help_refs: &["docs/ux/m3/first_run_task_success_packet.md"],
        support_export_refs: &[
            "support:export.include_first_run_task_success_packet",
        ],
        partner_scorecard_refs: &["partner-scorecard:beta-readiness:first_run"],
        narrative: "User closed the Start Center before admission; dashboards keep abandonment off the completion line.",
        events: vec![
            event(
                first_run_entry(),
                OnboardingEventName::FirstRunReached,
                OnboardingEventPhase::Intent,
                OutcomeClass::Abandoned,
                None,
                Some(CompletionClass::AbortedBeforeAdmission),
                None,
                100,
            ),
        ],
    });

    rows.push(RowSeed {
        row_id: "row:first_run.repair_required.forced_sign_in_before_local_work",
        flow: SwitchingFlowKind::FirstRun,
        state: TaskSuccessState::RepairRequired,
        measurement_surface: MeasurementSurface::SurfaceFirstRun,
        entry_route_id: EntryRouteId::StartCenter,
        entry_verb: EntryVerbKind::OpenFolder,
        target_kind: TargetKind::LocalFolder,
        completion_checkpoint_class: None,
        completion_class: Some(CompletionClass::FailedWithTypedBlocker),
        failure_category: Some(FailureCategory::ForcedSignInBeforeUsefulLocalWork),
        outcome_class: OutcomeClass::Blocked,
        repair_action_token: Some(RepairActionToken::ReissueAdmissionReview),
        docs_help_refs: &[
            "docs/ux/m3/first_run_task_success_packet.md",
            "docs/help/no_account_first_run.md",
        ],
        support_export_refs: &[
            "support:export.include_first_run_task_success_packet",
        ],
        partner_scorecard_refs: &["partner-scorecard:beta-readiness:first_run"],
        narrative: "First-run blocked by a sign-in prompt before useful local work; repair re-issues admission review to restore the no-account path.",
        events: vec![
            event(
                first_run_entry(),
                OnboardingEventName::AdmissionDecided,
                OnboardingEventPhase::Admission,
                OutcomeClass::Blocked,
                None,
                Some(CompletionClass::FailedWithTypedBlocker),
                None,
                120,
            ),
        ],
    });

    // ---- Imported-profile flow --------------------------------------

    rows.push(RowSeed {
        row_id: "row:imported_profile.completion.vs_code_settings_per_item",
        flow: SwitchingFlowKind::ImportedProfile,
        state: TaskSuccessState::Completion,
        measurement_surface: MeasurementSurface::SurfaceMigrationReview,
        entry_route_id: EntryRouteId::CloneOrImport,
        entry_verb: EntryVerbKind::ImportFromExternal,
        target_kind: TargetKind::ImportPacket,
        completion_checkpoint_class: Some(
            CompletionCheckpointClass::MigrationCommittedWithPerItemOutcomes,
        ),
        completion_class: Some(CompletionClass::CompletedMigrationCommittedPerItem),
        failure_category: None,
        outcome_class: OutcomeClass::Completed,
        repair_action_token: None,
        docs_help_refs: &[
            "docs/ux/m3/first_run_task_success_packet.md",
            "docs/migration/imported_profile_review.md",
        ],
        support_export_refs: &[
            "support:export.include_first_run_task_success_packet",
        ],
        partner_scorecard_refs: &["partner-scorecard:beta-readiness:imported_profile"],
        narrative: "VS Code settings import committed per-item outcomes with a rollback checkpoint minted before apply.",
        events: vec![
            event_with_migration(
                imported_profile_entry(),
                OnboardingEventName::MigrationDryRunProduced,
                OnboardingEventPhase::MigrationReview,
                OutcomeClass::Completed,
                None,
                None,
                None,
                migration_funnel(
                    MigrationFunnelStep::DryRunProduced,
                    DryRunState::Produced,
                    RollbackState::Available,
                ),
                150,
            ),
            event_with_migration(
                imported_profile_entry(),
                OnboardingEventName::MigrationRollbackCheckpointWritten,
                OnboardingEventPhase::MigrationReview,
                OutcomeClass::Completed,
                None,
                None,
                None,
                migration_funnel(
                    MigrationFunnelStep::CheckpointWritten,
                    DryRunState::Produced,
                    RollbackState::Available,
                ),
                160,
            ),
            event_with_migration(
                imported_profile_entry(),
                OnboardingEventName::MigrationApplied,
                OnboardingEventPhase::MigrationReview,
                OutcomeClass::Completed,
                Some(CompletionCheckpointClass::MigrationCommittedWithPerItemOutcomes),
                Some(CompletionClass::CompletedMigrationCommittedPerItem),
                Some(FirstUsefulWorkTiming::new(
                    140,
                    180,
                    FirstUsefulWorkTargetSurface::MigrationCenterBeforeCommit,
                    SemanticWarmupState::SemanticWarmupNotApplicable,
                )),
                migration_funnel(
                    MigrationFunnelStep::Applied,
                    DryRunState::Produced,
                    RollbackState::Available,
                ),
                180,
            ),
        ],
    });

    rows.push(RowSeed {
        row_id: "row:imported_profile.fallback.managed_sync_declined",
        flow: SwitchingFlowKind::ImportedProfile,
        state: TaskSuccessState::Fallback,
        measurement_surface: MeasurementSurface::SurfaceOptInBoundary,
        entry_route_id: EntryRouteId::CloneOrImport,
        entry_verb: EntryVerbKind::ImportFromExternal,
        target_kind: TargetKind::ImportPacket,
        completion_checkpoint_class: Some(
            CompletionCheckpointClass::DeclineContinuedWithoutDegradation,
        ),
        completion_class: Some(CompletionClass::CompletedDeclineWithoutDegradation),
        failure_category: None,
        outcome_class: OutcomeClass::Completed,
        repair_action_token: None,
        docs_help_refs: &[
            "docs/ux/m3/first_run_task_success_packet.md",
            "docs/help/managed_sync_decline.md",
        ],
        support_export_refs: &[
            "support:export.include_first_run_task_success_packet",
        ],
        partner_scorecard_refs: &["partner-scorecard:beta-readiness:imported_profile"],
        narrative: "User declined managed sync after import; per-item outcomes stay local-only without silent widening.",
        events: vec![
            event_with_migration(
                imported_profile_entry(),
                OnboardingEventName::MigrationOutcomeRecorded,
                OnboardingEventPhase::MigrationReview,
                OutcomeClass::Completed,
                Some(CompletionCheckpointClass::DeclineContinuedWithoutDegradation),
                Some(CompletionClass::CompletedDeclineWithoutDegradation),
                None,
                migration_funnel(
                    MigrationFunnelStep::PerItemOutcomesRecorded,
                    DryRunState::Produced,
                    RollbackState::Available,
                ),
                200,
            ),
        ],
    });

    rows.push(RowSeed {
        row_id: "row:imported_profile.abandonment.dry_run_dismissed",
        flow: SwitchingFlowKind::ImportedProfile,
        state: TaskSuccessState::Abandonment,
        measurement_surface: MeasurementSurface::SurfaceMigrationReview,
        entry_route_id: EntryRouteId::CloneOrImport,
        entry_verb: EntryVerbKind::ImportFromExternal,
        target_kind: TargetKind::ImportPacket,
        completion_checkpoint_class: None,
        completion_class: Some(CompletionClass::AbandonedAfterAdmission),
        failure_category: Some(FailureCategory::OutcomeAggregatedNotPerItem),
        outcome_class: OutcomeClass::Abandoned,
        repair_action_token: None,
        docs_help_refs: &[
            "docs/ux/m3/first_run_task_success_packet.md",
            "docs/migration/imported_profile_review.md",
        ],
        support_export_refs: &[
            "support:export.include_first_run_task_success_packet",
        ],
        partner_scorecard_refs: &["partner-scorecard:beta-readiness:imported_profile"],
        narrative: "User reviewed the dry-run and closed the wizard before commit; abandonment is recorded with a typed reason.",
        events: vec![
            event_with_migration(
                imported_profile_entry(),
                OnboardingEventName::MigrationDryRunProduced,
                OnboardingEventPhase::MigrationReview,
                OutcomeClass::Abandoned,
                None,
                Some(CompletionClass::AbandonedAfterAdmission),
                None,
                migration_funnel(
                    MigrationFunnelStep::DryRunProduced,
                    DryRunState::Produced,
                    RollbackState::Available,
                ),
                220,
            ),
        ],
    });

    rows.push(RowSeed {
        row_id: "row:imported_profile.repair_required.checkpoint_missing",
        flow: SwitchingFlowKind::ImportedProfile,
        state: TaskSuccessState::RepairRequired,
        measurement_surface: MeasurementSurface::SurfaceMigrationReview,
        entry_route_id: EntryRouteId::CloneOrImport,
        entry_verb: EntryVerbKind::ImportFromExternal,
        target_kind: TargetKind::ImportPacket,
        completion_checkpoint_class: None,
        completion_class: Some(CompletionClass::FailedWithTypedBlocker),
        failure_category: Some(FailureCategory::RollbackCheckpointMissing),
        outcome_class: OutcomeClass::Blocked,
        repair_action_token: Some(RepairActionToken::MintRollbackCheckpoint),
        docs_help_refs: &[
            "docs/ux/m3/first_run_task_success_packet.md",
            "docs/migration/rollback_checkpoint.md",
        ],
        support_export_refs: &[
            "support:export.include_first_run_task_success_packet",
        ],
        partner_scorecard_refs: &["partner-scorecard:beta-readiness:imported_profile"],
        narrative: "Apply was denied because no rollback checkpoint was minted; repair mints a fresh checkpoint before retrying apply.",
        events: vec![
            event_with_migration(
                imported_profile_entry(),
                OnboardingEventName::MigrationOutcomeRecorded,
                OnboardingEventPhase::MigrationReview,
                OutcomeClass::Blocked,
                None,
                Some(CompletionClass::FailedWithTypedBlocker),
                None,
                migration_funnel(
                    MigrationFunnelStep::DryRunProduced,
                    DryRunState::Produced,
                    RollbackState::Missing,
                ),
                240,
            ),
        ],
    });

    rows
}

fn build_row(seed: &RowSeed) -> FirstRunTaskSuccessRow {
    FirstRunTaskSuccessRow {
        record_kind: FIRST_RUN_TASK_SUCCESS_ROW_RECORD_KIND.to_owned(),
        schema_version: FIRST_RUN_TASK_SUCCESS_PACKET_SCHEMA_VERSION,
        shared_contract_ref: FIRST_RUN_TASK_SUCCESS_PACKET_SHARED_CONTRACT_REF.to_owned(),
        row_id: seed.row_id.to_owned(),
        flow_kind: seed.flow,
        state: seed.state,
        measurement_surface: seed.measurement_surface,
        entry_route_id: seed.entry_route_id,
        entry_verb: seed.entry_verb,
        target_kind: seed.target_kind,
        completion_checkpoint_class: seed.completion_checkpoint_class,
        completion_class: seed.completion_class,
        failure_category: seed.failure_category,
        outcome_class: seed.outcome_class,
        repair_action_token: seed.repair_action_token,
        no_raw_sensitive_user_content: true,
        telemetry_event_names: seed
            .events
            .iter()
            .map(|event| event.event_name)
            .collect(),
        docs_help_refs: seed.docs_help_refs.iter().map(|s| (*s).to_owned()).collect(),
        support_export_refs: seed
            .support_export_refs
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        partner_scorecard_refs: seed
            .partner_scorecard_refs
            .iter()
            .map(|s| (*s).to_owned())
            .collect(),
        narrative: seed.narrative.to_owned(),
    }
}

fn packet_telemetry_context() -> OnboardingTelemetryContext {
    let build = BuildIdentityRecord {
        crate_name: "aureline-shell".to_owned(),
        crate_version: "0.0.0".to_owned(),
        rustc_target_triple: "fixture-target".to_owned(),
    };
    let mut context = OnboardingTelemetryContext::developer_local(
        "trace:first-run-task-success-packet-beta",
        "session:first-run-task-success-packet-beta",
        build,
    );
    context.privacy.export_posture = TelemetryExportPosture::SupportExportOnRequest;
    context.evidence_refs = vec![
        "schemas/ux/first_run_metrics.schema.json".to_owned(),
        "artifacts/ux/m3/first_run_task_success_packet.md".to_owned(),
    ];
    context
}

fn first_run_entry() -> EntryFlowDescriptor {
    EntryFlowDescriptor {
        flow_kind: EntryFlowKind::FirstRun,
        entry_verb: EntryVerbKind::OpenFolder,
        entry_route_id: EntryRouteId::StartCenter,
        measurement_surface: MeasurementSurface::SurfaceFirstRun,
        target_kind: TargetKind::LocalFolder,
        target_ref: Some("target:first-run-local-folder".to_owned()),
        deployment_profile_id: "deployment_profile:individual-local-beta".to_owned(),
    }
}

fn imported_profile_entry() -> EntryFlowDescriptor {
    EntryFlowDescriptor {
        flow_kind: EntryFlowKind::Import,
        entry_verb: EntryVerbKind::ImportFromExternal,
        entry_route_id: EntryRouteId::CloneOrImport,
        measurement_surface: MeasurementSurface::SurfaceMigrationReview,
        target_kind: TargetKind::ImportPacket,
        target_ref: Some("target:imported-profile-vs-code-settings".to_owned()),
        deployment_profile_id: "deployment_profile:individual-local-beta".to_owned(),
    }
}

fn migration_funnel(
    step: MigrationFunnelStep,
    dry_run_state: DryRunState,
    rollback_state: RollbackState,
) -> MigrationFunnelRecord {
    MigrationFunnelRecord {
        migration_session_ref: "migration-session:imported-profile-vs-code-settings".to_owned(),
        source_kind: MigrationSourceKind::VsCode,
        step,
        dry_run_state,
        rollback_state,
        checkpoint_ref: Some("checkpoint:imported-profile-pre-apply".to_owned()),
        outcome_counts: MigrationOutcomeCounts::new(4, 5, 1, 2, 1, 1),
        parity_scorecard_ref: Some("parity-scorecard:imported-profile-vs-code-settings".to_owned()),
        migration_report_ref: Some("migration-report:imported-profile-vs-code-settings".to_owned()),
    }
}

fn event(
    entry: EntryFlowDescriptor,
    event_name: OnboardingEventName,
    event_phase: OnboardingEventPhase,
    outcome_class: OutcomeClass,
    completion_checkpoint_class: Option<CompletionCheckpointClass>,
    completion_class: Option<CompletionClass>,
    first_useful_work: Option<FirstUsefulWorkTiming>,
    occurred_tick: u64,
) -> OnboardingEventInput {
    OnboardingEventInput {
        entry,
        event_name,
        event_phase,
        completion_checkpoint_class,
        completion_class,
        outcome_class,
        first_useful_work,
        migration_funnel: None,
        failure_category: None,
        evidence_refs: vec![
            "fixtures/ux/first_run_task_success_packet/packet.json".to_owned(),
        ],
        occurred_tick,
    }
}

fn event_with_migration(
    entry: EntryFlowDescriptor,
    event_name: OnboardingEventName,
    event_phase: OnboardingEventPhase,
    outcome_class: OutcomeClass,
    completion_checkpoint_class: Option<CompletionCheckpointClass>,
    completion_class: Option<CompletionClass>,
    first_useful_work: Option<FirstUsefulWorkTiming>,
    migration_funnel: MigrationFunnelRecord,
    occurred_tick: u64,
) -> OnboardingEventInput {
    OnboardingEventInput {
        entry,
        event_name,
        event_phase,
        completion_checkpoint_class,
        completion_class,
        outcome_class,
        first_useful_work,
        migration_funnel: Some(migration_funnel),
        failure_category: None,
        evidence_refs: vec![
            "fixtures/ux/first_run_task_success_packet/packet.json".to_owned(),
            "schemas/migration/importer_outcome.schema.json".to_owned(),
        ],
        occurred_tick,
    }
}

fn outcome_class_token(class: OutcomeClass) -> &'static str {
    match class {
        OutcomeClass::Pending => "pending",
        OutcomeClass::Completed => "completed",
        OutcomeClass::Partial => "partial",
        OutcomeClass::Blocked => "blocked",
        OutcomeClass::Abandoned => "abandoned",
        OutcomeClass::Failed => "failed",
        OutcomeClass::Denied => "denied",
        OutcomeClass::Restored => "restored",
        OutcomeClass::RolledBack => "rolled_back",
    }
}

fn completion_class_token(class: CompletionClass) -> &'static str {
    match class {
        CompletionClass::CompletedFirstUsefulEdit => "completed_first_useful_edit",
        CompletionClass::CompletedFirstUsefulNavigationOnly => {
            "completed_first_useful_navigation_only"
        }
        CompletionClass::CompletedWithAdvertisedNarrowing => "completed_with_advertised_narrowing",
        CompletionClass::CompletedMigrationCommittedPerItem => {
            "completed_migration_committed_per_item"
        }
        CompletionClass::CompletedRestoreLevelDelivered => "completed_restore_level_delivered",
        CompletionClass::CompletedDeclineWithoutDegradation => {
            "completed_decline_without_degradation"
        }
        CompletionClass::CompletedReconnectDelivered => "completed_reconnect_delivered",
        CompletionClass::AbortedBeforeAdmission => "aborted_before_admission",
        CompletionClass::AbandonedAfterAdmission => "abandoned_after_admission",
        CompletionClass::FailedWithTypedBlocker => "failed_with_typed_blocker",
    }
}

fn failure_category_token(category: FailureCategory) -> &'static str {
    match category {
        FailureCategory::ForcedSignInBeforeUsefulLocalWork => {
            "forced_sign_in_before_useful_local_work"
        }
        FailureCategory::NetworkRequiredForLocalEntry => "network_required_for_local_entry",
        FailureCategory::AdmissionDeniedPolicy => "admission_denied_policy",
        FailureCategory::AdmissionDeniedTrust => "admission_denied_trust",
        FailureCategory::ResultingModeSilentlyDowngraded => "resulting_mode_silently_downgraded",
        FailureCategory::EditorBlockedOnIndexWarmup => "editor_blocked_on_index_warmup",
        FailureCategory::SaveBlockedOnService => "save_blocked_on_service",
        FailureCategory::DryRunSkipped => "dry_run_skipped",
        FailureCategory::OutcomeAggregatedNotPerItem => "outcome_aggregated_not_per_item",
        FailureCategory::RollbackCheckpointMissing => "rollback_checkpoint_missing",
        FailureCategory::RestoreLevelPromisedHigherThanDelivered => {
            "restore_level_promised_higher_than_delivered"
        }
        FailureCategory::MissingTargetStateSilentlyDropped => {
            "missing_target_state_silently_dropped"
        }
        FailureCategory::SilentMutatingCommandReplay => "silent_mutating_command_replay",
        FailureCategory::ReconnectTargetUnavailable => "reconnect_target_unavailable",
        FailureCategory::ReconnectRequiresReauth => "reconnect_requires_reauth",
        FailureCategory::ReconnectPolicyBlocked => "reconnect_policy_blocked",
    }
}

fn privacy_class_token(class: TelemetryPrivacyClass) -> &'static str {
    match class {
        TelemetryPrivacyClass::PrivacyLocalOnlyNoEmission => "privacy_local_only_no_emission",
        TelemetryPrivacyClass::PrivacyOptInAggregateOnly => "privacy_opt_in_aggregate_only",
        TelemetryPrivacyClass::PrivacyOptInAttributable => "privacy_opt_in_attributable",
    }
}

fn export_posture_token(posture: TelemetryExportPosture) -> &'static str {
    match posture {
        TelemetryExportPosture::ExcludedByDefault => "excluded_by_default",
        TelemetryExportPosture::DesignPartnerOptIn => "design_partner_opt_in",
        TelemetryExportPosture::SupportExportOnRequest => "support_export_on_request",
    }
}

fn serde_event_name_token(name: OnboardingEventName) -> &'static str {
    match name {
        OnboardingEventName::FirstRunReached => "first_run_reached",
        OnboardingEventName::FirstRunEntryRouteSelected => "first_run_entry_route_selected",
        OnboardingEventName::FirstRunAdmitted => "first_run_admitted",
        OnboardingEventName::EntryVerbResolved => "entry_verb_resolved",
        OnboardingEventName::AdmissionDecided => "admission_decided",
        OnboardingEventName::FirstOpenCompleted => "first_open_completed",
        OnboardingEventName::FirstUsefulNavigationReached => "first_useful_navigation_reached",
        OnboardingEventName::FirstUsefulEditDurable => "first_useful_edit_durable",
        OnboardingEventName::MigrationDryRunProduced => "migration_dry_run_produced",
        OnboardingEventName::MigrationOutcomeRecorded => "migration_outcome_recorded",
        OnboardingEventName::MigrationApplied => "migration_applied",
        OnboardingEventName::MigrationRolledBack => "migration_rolled_back",
        OnboardingEventName::MigrationRollbackCheckpointWritten => {
            "migration_rollback_checkpoint_written"
        }
        OnboardingEventName::MigrationRollbackCheckpointRestored => {
            "migration_rollback_checkpoint_restored"
        }
        OnboardingEventName::RestorePromptPresented => "restore_prompt_presented",
        OnboardingEventName::RestoreLevelDelivered => "restore_level_delivered",
        OnboardingEventName::RestoreCompleted => "restore_completed",
        OnboardingEventName::ReconnectPromptPresented => "reconnect_prompt_presented",
        OnboardingEventName::ReconnectCompleted => "reconnect_completed",
        OnboardingEventName::ReconnectFailed => "reconnect_failed",
    }
}

/// Translates a telemetry-validation error into a packet-validation
/// error so callers can keep a single error type.
pub fn telemetry_error_into_packet_error(
    err: OnboardingTelemetryValidationError,
) -> FirstRunTaskSuccessValidationError {
    match err.check_id {
        "onboarding_task_success.privacy.raw_project_content_present"
        | "onboarding_task_success.privacy.raw_project_content_not_prohibited" => {
            FirstRunTaskSuccessValidationError::TelemetryEnvelopeAllowsRawContent
        }
        _ => FirstRunTaskSuccessValidationError::TelemetryEnvelopeAllowsRawContent,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_packet_passes_validation() {
        let packet = seeded_first_run_task_success_packet();
        validate_first_run_task_success_packet(&packet).expect("seeded packet must validate");
    }

    #[test]
    fn seeded_packet_covers_every_required_cell() {
        let packet = seeded_first_run_task_success_packet();
        assert!(packet.covers_every_required_cell());
        for flow in SwitchingFlowKind::required_flows() {
            for state in TaskSuccessState::required_states() {
                assert!(
                    packet
                        .rows
                        .iter()
                        .any(|row| row.flow_kind == flow && row.state == state),
                    "missing cell {} / {}",
                    flow.as_str(),
                    state.as_str()
                );
            }
        }
    }

    #[test]
    fn seeded_packet_declares_metadata_safe_envelope() {
        let packet = seeded_first_run_task_success_packet();
        assert!(packet.no_raw_sensitive_user_content);
        assert!(!packet
            .telemetry_capture
            .privacy
            .contains_raw_project_content);
        assert!(packet
            .telemetry_capture
            .privacy
            .prohibited_content_classes
            .contains(&ProhibitedContentClass::RawProjectContent));
    }

    #[test]
    fn telemetry_capture_carries_first_run_and_import_events() {
        let packet = seeded_first_run_task_success_packet();
        let flow_kinds: BTreeSet<_> = packet
            .telemetry_capture
            .events
            .iter()
            .map(|event| event.entry.flow_kind)
            .collect();
        assert!(flow_kinds.contains(&EntryFlowKind::FirstRun));
        assert!(flow_kinds.contains(&EntryFlowKind::Import));
    }

    #[test]
    fn validation_flags_missing_cell() {
        let mut packet = seeded_first_run_task_success_packet();
        packet
            .rows
            .retain(|row| row.state != TaskSuccessState::Abandonment);
        packet.state_summary = PacketStateSummary::from_rows(&packet.rows);
        let errors = validate_first_run_task_success_packet(&packet)
            .expect_err("must flag missing abandonment cells");
        assert!(errors.iter().any(|err| matches!(
            err,
            FirstRunTaskSuccessValidationError::MissingRequiredCell { state, .. }
                if state == "abandonment"
        )));
    }

    #[test]
    fn validation_flags_missing_repair_action() {
        let mut packet = seeded_first_run_task_success_packet();
        if let Some(row) = packet
            .rows
            .iter_mut()
            .find(|row| row.state == TaskSuccessState::RepairRequired)
        {
            row.repair_action_token = None;
        }
        let errors = validate_first_run_task_success_packet(&packet)
            .expect_err("must flag missing repair action");
        assert!(errors.iter().any(|err| matches!(
            err,
            FirstRunTaskSuccessValidationError::RepairActionMissing { .. }
        )));
    }

    #[test]
    fn validation_flags_raw_content_declared() {
        let mut packet = seeded_first_run_task_success_packet();
        packet.rows[0].no_raw_sensitive_user_content = false;
        let errors = validate_first_run_task_success_packet(&packet)
            .expect_err("must flag raw content declaration");
        assert!(errors.iter().any(|err| matches!(
            err,
            FirstRunTaskSuccessValidationError::RawSensitiveContentDeclared { .. }
        )));
    }

    #[test]
    fn validation_flags_stale_state_summary() {
        let mut packet = seeded_first_run_task_success_packet();
        packet.state_summary.first_run.completion = 99;
        let errors = validate_first_run_task_success_packet(&packet)
            .expect_err("must flag stale summary");
        assert!(errors
            .iter()
            .any(|err| matches!(err, FirstRunTaskSuccessValidationError::StateSummaryStale)));
    }

    #[test]
    fn support_export_quotes_every_case_id() {
        let packet = seeded_first_run_task_success_packet();
        let export = FirstRunTaskSuccessSupportExport::from_packet(
            "support-export:first-run-task-success:001",
            packet.clone(),
        );
        assert_eq!(
            export.shared_contract_ref,
            FIRST_RUN_TASK_SUCCESS_PACKET_SHARED_CONTRACT_REF
        );
        assert!(export.case_ids.contains(&packet.packet_id));
        assert!(export
            .case_ids
            .contains(&packet.telemetry_capture.capture_id));
        for row in &packet.rows {
            assert!(export.case_ids.contains(&row.row_id));
        }
        for event in &packet.telemetry_capture.events {
            assert!(export.case_ids.contains(&event.event_id));
        }
    }

    #[test]
    fn compact_lines_summarize_state_coverage() {
        let packet = seeded_first_run_task_success_packet();
        let lines = packet.compact_lines();
        assert!(lines.iter().any(|line| line.starts_with("packet:")));
        assert!(lines.iter().any(|line| line.starts_with("first_run:")));
        assert!(lines.iter().any(|line| line.starts_with("imported_profile:")));
    }

    #[test]
    fn markdown_artifact_is_deterministic() {
        let a = seeded_first_run_task_success_packet().render_markdown();
        let b = seeded_first_run_task_success_packet().render_markdown();
        assert_eq!(a, b);
        assert!(a.contains("# First-run task-success packet"));
        assert!(a.contains("Completion"));
        assert!(a.contains("Repair required"));
    }

    #[test]
    fn checked_in_fixture_matches_seeded_packet() {
        let packet = seeded_first_run_task_success_packet();
        let serialized = serde_json::to_string_pretty(&packet)
            .expect("seeded packet must serialize");
        let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/ux/first_run_task_success_packet/packet.json");
        let on_disk = std::fs::read_to_string(&fixture_path)
            .expect("checked-in packet fixture must exist");
        let trimmed = on_disk.trim_end_matches('\n');
        assert_eq!(
            trimmed,
            serialized,
            "seeded packet drifted from fixture; regenerate with \
             `cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- packet > \
             fixtures/ux/first_run_task_success_packet/packet.json`"
        );
    }

    #[test]
    fn checked_in_markdown_matches_render() {
        let rendered = seeded_first_run_task_success_packet().render_markdown();
        let artifact_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../artifacts/ux/m3/first_run_task_success_packet.md");
        let on_disk = std::fs::read_to_string(&artifact_path)
            .expect("checked-in markdown artifact must exist");
        assert_eq!(
            on_disk.trim_end_matches('\n'),
            rendered.trim_end_matches('\n'),
            "markdown artifact drifted from render; regenerate with \
             `cargo run -q -p aureline-shell --bin aureline_shell_onboarding_metrics -- markdown > \
             artifacts/ux/m3/first_run_task_success_packet.md`"
        );
    }
}
