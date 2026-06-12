//! M5 command-governance packet for preview, disabled-reason, and approval parity.
//!
//! The canonical command registry already carries typed descriptor, invocation,
//! result, automation, and disabled-reason truth. This module projects that
//! truth into one M5 governance packet that answers the release question for
//! the depth-surface commands: does desktop, CLI, AI, recipe, extension, and
//! browser/companion routing preserve the same preview, approval, denial, and
//! no-bypass posture?
//!
//! The packet is intentionally export-safe. It preserves actor, target,
//! trust-epoch, and rollout-state references without carrying raw payloads or
//! secrets, so support, release, and audit consumers can reconstruct why a
//! command ran, previewed, handed off, or denied without scraping UI-specific
//! copy.

use std::error::Error;
use std::fmt;
use std::sync::OnceLock;

use serde::{Deserialize, Serialize};

use crate::automation::{labels_include, why_not_automatable_reason, ControlledAutomationLabel};
use crate::enablement::DisabledReasonRecord;
use crate::registry::{seeded_registry, CommandRegistryEntryRecord};

#[cfg(test)]
mod tests;

/// Stable record-kind tag carried by [`M5CommandGovernancePacket`].
pub const M5_COMMAND_GOVERNANCE_RECORD_KIND: &str = "m5_command_governance_packet";

/// Schema version for M5 command-governance packets.
pub const M5_COMMAND_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the M5 command-governance schema.
pub const M5_COMMAND_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/commands/m5_command_governance.schema.json";

/// Repo-relative path of the M5 command-governance companion doc.
pub const M5_COMMAND_GOVERNANCE_DOC_REF: &str = "docs/commands/m5_command_governance.md";

/// Repo-relative path of the checked fixture directory.
pub const M5_COMMAND_GOVERNANCE_FIXTURE_DIR: &str = "fixtures/commands/m5_command_governance";

/// Repo-relative path of the checked support export.
pub const M5_COMMAND_GOVERNANCE_SUPPORT_EXPORT_REF: &str =
    "artifacts/commands/m5_command_governance/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_COMMAND_GOVERNANCE_SUMMARY_REF: &str =
    "artifacts/commands/m5_command_governance/summary.md";

/// Stable packet id used by the seeded export.
pub const M5_COMMAND_GOVERNANCE_PACKET_ID: &str = "m5-command-governance:stable:0001";

/// Stable support-export id used by [`M5CommandGovernanceSupportExport`].
pub const M5_COMMAND_GOVERNANCE_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-command-governance:0001";

const GENERATED_AT: &str = "2026-06-12T00:00:00Z";
const SOURCE_REGISTRY_REF: &str = "artifacts/commands/m5_command_registry_seed.yaml";

const M5_COMMAND_IDS: &[&str] = &[
    "cmd:notebook.run_all_cells",
    "cmd:data_api.send_request",
    "cmd:profiler.start_capture",
    "cmd:trace_replay.replay_session",
    "cmd:docs_browser.open_external",
    "cmd:template_scaffold.scaffold_project",
    "cmd:review_pipeline.run_pipeline",
    "cmd:preview.open_live_preview",
    "cmd:companion.handoff_session",
    "cmd:incident.open_incident",
    "cmd:sync.push_workspace_state",
    "cmd:offboarding.export_and_wipe",
    "cmd:secret_broker.open_credential_review",
    "cmd:secret_broker.open_credential_rotation",
    "cmd:infrastructure.reconcile_workspace",
];

static ACTIVITY_REPORT_PROJECTION: OnceLock<ActivityReportProjection> = OnceLock::new();

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ActivityReportProjection {
    shared_contract_ref: String,
    report_id: String,
    published_report_ref: String,
    published_doc_ref: String,
    support_export_refs: Vec<String>,
    rows: Vec<ActivityReportRowProjection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ActivityReportRowProjection {
    descriptor: ActivityDescriptorProjection,
    bindings: Vec<ActivityBindingProjection>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ActivityDescriptorProjection {
    family_id: String,
    job_family: String,
    reopen_anchor_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct ActivityBindingProjection {
    guarantee: String,
    qualification_status: String,
    projected_export_identity: Option<String>,
}

/// Invocation route whose parity posture the packet proves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5GovernanceSurfaceClass {
    /// Canonical desktop product route.
    Desktop,
    /// CLI or headless route.
    Cli,
    /// AI route that resolves to stable command identity.
    Ai,
    /// Declarative recipe route.
    Recipe,
    /// Extension-owned entry route that must still preserve host authority.
    Extension,
    /// Browser or companion handoff route.
    BrowserCompanion,
}

impl M5GovernanceSurfaceClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Cli => "cli",
            Self::Ai => "ai",
            Self::Recipe => "recipe",
            Self::Extension => "extension",
            Self::BrowserCompanion => "browser_companion",
        }
    }

    /// Required surface coverage for the M5 packet.
    pub const fn required_coverage() -> [Self; 6] {
        [
            Self::Desktop,
            Self::Cli,
            Self::Ai,
            Self::Recipe,
            Self::Extension,
            Self::BrowserCompanion,
        ]
    }
}

/// Route posture a surface may expose without widening desktop authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RoutePostureClass {
    /// The route may run directly after normal preflight.
    DirectAllowed,
    /// The route preserves the preview sheet before apply.
    PreviewRequired,
    /// The route preserves preview and explicit approval.
    ApprovalRequired,
    /// The route denies with a structured diagnostics packet.
    DiagnosticsRequired,
    /// The route must hand off to desktop while preserving context.
    DesktopHandoffRequired,
}

impl M5RoutePostureClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectAllowed => "direct_allowed",
            Self::PreviewRequired => "preview_required",
            Self::ApprovalRequired => "approval_required",
            Self::DiagnosticsRequired => "diagnostics_required",
            Self::DesktopHandoffRequired => "desktop_handoff_required",
        }
    }
}

/// Disabled-reason family carried by export-safe denial packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DisabledReasonFamilyClass {
    /// The current focus or client surface cannot host the command.
    FocusMismatch,
    /// Workspace or content trust blocks the route.
    TrustBlocked,
    /// Policy or entitlement blocks the route.
    PolicyBlocked,
    /// Rollout or lifecycle state blocks the route.
    RolloutBlocked,
    /// Runtime or execution context is unavailable.
    MissingRuntime,
    /// Provider, credential, or capability state is unavailable.
    MissingCapabilityState,
    /// The command must route through preview before apply.
    PreviewRequired,
    /// The command must route through approval before apply.
    ApprovalRequired,
}

impl M5DisabledReasonFamilyClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FocusMismatch => "focus_mismatch",
            Self::TrustBlocked => "trust_blocked",
            Self::PolicyBlocked => "policy_blocked",
            Self::RolloutBlocked => "rollout_blocked",
            Self::MissingRuntime => "missing_runtime",
            Self::MissingCapabilityState => "missing_capability_state",
            Self::PreviewRequired => "preview_required",
            Self::ApprovalRequired => "approval_required",
        }
    }
}

/// Approval model a route must preserve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ApprovalModelClass {
    /// No approval is required.
    NoApprovalRequired,
    /// The route requires explicit user confirmation.
    HumanConfirmRequired,
}

impl M5ApprovalModelClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoApprovalRequired => "no_approval_required",
            Self::HumanConfirmRequired => "human_confirm_required",
        }
    }
}

/// Canonical execution profile for the command-result lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResultExecutionProfileClass {
    /// Read-only or local-open route with no durable work object.
    EphemeralReadOnly,
    /// Long-running or reopenable work that must stay durable.
    DurableProgress,
    /// State-changing route that must preserve durable mutation truth.
    DurableMutation,
}

impl M5ResultExecutionProfileClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralReadOnly => "ephemeral_read_only",
            Self::DurableProgress => "durable_progress",
            Self::DurableMutation => "durable_mutation",
        }
    }
}

/// Canonical outcome vocabulary for M5 command-result packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResultOutcomeClass {
    /// The command completed successfully.
    Success,
    /// The command completed with only part of the intended scope applied.
    PartialSuccess,
    /// The command was cancelled by a user, system, or policy actor.
    Cancelled,
    /// The result was superseded by a newer authoritative run.
    Superseded,
    /// The command was denied before execution.
    Denied,
    /// The command completed in a visibly degraded posture.
    Degraded,
    /// The command failed.
    Failed,
}

impl M5ResultOutcomeClass {
    /// Stable token used in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::PartialSuccess => "partial_success",
            Self::Cancelled => "cancelled",
            Self::Superseded => "superseded",
            Self::Denied => "denied",
            Self::Degraded => "degraded",
            Self::Failed => "failed",
        }
    }

    /// Required outcome coverage for M5 result packets.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::Success,
            Self::PartialSuccess,
            Self::Cancelled,
            Self::Superseded,
            Self::Denied,
            Self::Degraded,
            Self::Failed,
        ]
    }
}

/// One export-safe outcome projection for an M5 result packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandOutcomeProjectionRow {
    /// Canonical outcome class.
    pub outcome_class: M5ResultOutcomeClass,
    /// Stable outcome code projected into the result packet body.
    pub result_code: String,
    /// Durable activity-row state joined to this outcome when present.
    pub activity_state_class: Option<String>,
    /// Copy-safe summary ref for UI, CLI, support, and incident flows.
    pub export_safe_summary_ref: String,
    /// Raw packet export ref for headless and support reconstruction.
    pub raw_packet_export_ref: String,
    /// Support-export case ref for the projected outcome.
    pub support_export_case_ref: String,
    /// Release-evidence ref for the projected outcome.
    pub release_evidence_ref: String,
    /// Whether this outcome's summary is safe to copy or export by default.
    pub copy_safe: bool,
}

/// Result artifact and join posture for one M5 command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResultArtifactProjectionRecord {
    /// Descriptor-owned result-contract class.
    pub result_contract_class: String,
    /// Descriptor-owned created-artifact kind ref when one exists.
    pub artifact_kind_ref: Option<String>,
    /// Descriptor-owned evidence ref classes.
    pub evidence_ref_classes: Vec<String>,
    /// Whether the packet must include created-object refs.
    pub created_object_ref_required: bool,
    /// Exact-target reopen ref when the result must reopen durable work.
    pub exact_target_reopen_ref: Option<String>,
    /// Notification join ref when the command projects a routed notification.
    pub notification_join_ref: Option<String>,
    /// Activity-center join ref when the command projects durable work.
    pub activity_join_ref: Option<String>,
    /// Whether a rollback handle is required for this command result.
    pub rollback_handle_required: bool,
    /// Rollback posture token preserved by the route.
    pub rollback_handle_posture: String,
    /// Whether checkpoint refs are required for this command result.
    pub checkpoint_ref_required: bool,
    /// Checkpoint classes surfaced by the result packet.
    pub checkpoint_ref_classes: Vec<String>,
    /// Support-export identity ref for durable work.
    pub support_export_identity_ref: Option<String>,
    /// Release-evidence identity ref for this command result.
    pub release_evidence_identity_ref: String,
}

/// Durable-result, support-export, and release-evidence parity for one M5 command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResultPacketGovernanceRecord {
    /// Canonical execution profile for the command result.
    pub execution_profile_class: M5ResultExecutionProfileClass,
    /// Whether the command mutates state.
    pub mutating: bool,
    /// Whether the command is long-running or reopens durable work.
    pub long_running: bool,
    /// Whether durable result truth is required on this command.
    pub durable_truth_required: bool,
    /// Invocation-session schema preserved by the route.
    pub invocation_schema_ref: String,
    /// Result-packet schema preserved by the route.
    pub result_schema_ref: String,
    /// Export posture projected by the result packet.
    pub export_posture_class: String,
    /// Redaction class applied to support and incident exports.
    pub redaction_class: String,
    /// Whether copy-safe summaries are preserved.
    pub preserves_copy_safe_summary: bool,
    /// Whether raw packet exports are preserved for reconstruction.
    pub preserves_raw_packet_export: bool,
    /// Whether the result joins the activity center when required.
    pub joins_activity_center: bool,
    /// Whether the result joins routed notification surfaces.
    pub joins_notification_surface: bool,
    /// Whether the result joins metadata-safe support exports.
    pub joins_support_export: bool,
    /// Whether the result joins release evidence.
    pub joins_release_evidence: bool,
    /// Shared activity-object contract ref when durable work is joined.
    pub activity_shared_contract_ref: Option<String>,
    /// Whether portable-profile export remains allowed.
    pub portable_profile_allowed: bool,
    /// Whether support-bundle export remains allowed.
    pub support_bundle_allowed: bool,
    /// Canonical outcome projections required on every route.
    pub outcome_rows: Vec<M5CommandOutcomeProjectionRow>,
    /// Artifact, reopen, rollback, and checkpoint posture.
    pub artifacts: M5ResultArtifactProjectionRecord,
}

/// Copy-safe command introspection affordances.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CopySafeIntrospectionRecord {
    /// Whether `Copy command ID` is available.
    pub copy_command_id: bool,
    /// Whether `Copy CLI form` is available.
    pub copy_cli_form: bool,
    /// Whether `Add to recipe` is available.
    pub add_to_recipe: bool,
    /// Whether `Why not automatable?` is available.
    pub inspect_why_not_automatable: bool,
}

/// Preview-sheet posture projected from the canonical descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InvocationPreviewParityRecord {
    /// Stable preview class projected from the descriptor.
    pub preview_class: String,
    /// Whether the surface can expose a dry-run path.
    pub dry_run_supported: bool,
    /// Copy-safe introspection actions available from the route.
    pub copy_safe_introspection: M5CopySafeIntrospectionRecord,
    /// Exact structured reason when the command is not fully automatable.
    pub why_not_automatable_reason: Option<String>,
    /// Whether the preview metadata is support-export safe.
    pub export_safe: bool,
}

/// Structured disabled-reason packet used for parity and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DisabledReasonPacketRecord {
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical command id.
    pub command_id: String,
    /// Route that surfaced the denial.
    pub surface_class: M5GovernanceSurfaceClass,
    /// Disabled-reason family preserved by the route.
    pub reason_family: M5DisabledReasonFamilyClass,
    /// Stable disabled reason code or synthesized parity token.
    pub disabled_reason_code: String,
    /// Export-safe explanation ref.
    pub explanation_ref: String,
    /// Optional repair-hook id.
    pub repair_hook_id: Option<String>,
    /// Export-safe actor ref.
    pub actor_ref: String,
    /// Export-safe target ref.
    pub target_ref: String,
    /// Export-safe trust-epoch ref.
    pub trust_epoch_ref: String,
    /// Export-safe rollout-state ref.
    pub rollout_state_ref: String,
    /// Export redaction posture.
    pub redaction_class: String,
    /// Whether this packet is safe for support export.
    pub support_export_safe: bool,
}

/// Approval or denial packet used for route parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ApprovalParityPacketRecord {
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical command id.
    pub command_id: String,
    /// Route that surfaced the approval or denial.
    pub surface_class: M5GovernanceSurfaceClass,
    /// Stable preview class preserved by the route.
    pub preview_class: String,
    /// Stable approval model preserved by the route.
    pub approval_model_class: M5ApprovalModelClass,
    /// Reviewer-facing state token.
    pub decision_class: String,
    /// Export-safe actor ref.
    pub actor_ref: String,
    /// Export-safe target ref.
    pub target_ref: String,
    /// Export-safe trust-epoch ref.
    pub trust_epoch_ref: String,
    /// Export-safe rollout-state ref.
    pub rollout_state_ref: String,
    /// Whether the route preserves the no-bypass contract.
    pub no_bypass_rule_preserved: bool,
    /// Whether this packet is safe for support export.
    pub support_export_safe: bool,
}

/// One per-surface route row in the M5 governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SurfaceGovernanceRow {
    /// Surface family under audit.
    pub surface_class: M5GovernanceSurfaceClass,
    /// Client scope token the route runs under.
    pub client_scope: String,
    /// Route posture preserved by the surface.
    pub route_posture_class: M5RoutePostureClass,
    /// Preview-sheet and copy-safe introspection posture.
    pub preview_parity: M5InvocationPreviewParityRecord,
    /// Structured denial packets preserved by the route.
    pub disabled_reason_packets: Vec<M5DisabledReasonPacketRecord>,
    /// Structured approval or denial packet preserved by the route.
    pub approval_parity_packet: M5ApprovalParityPacketRecord,
    /// Whether the surface preserves the canonical result-packet model.
    pub result_packet_parity_preserved: bool,
    /// Whether the surface preserves durable activity joins when required.
    pub activity_join_parity_preserved: bool,
    /// Whether the surface preserves support-export and release-evidence joins.
    pub export_join_parity_preserved: bool,
    /// Whether the route preserves desktop preview semantics.
    pub preview_parity_preserved: bool,
    /// Whether the route preserves desktop approval semantics.
    pub approval_parity_preserved: bool,
    /// Whether the route preserves the same disabled-reason engine.
    pub disabled_reason_parity_preserved: bool,
}

/// One command row in the M5 governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandGovernanceRow {
    /// Canonical command id.
    pub command_id: String,
    /// Descriptor revision pinned for parity joins.
    pub command_revision_ref: String,
    /// Dotted canonical verb.
    pub canonical_verb: String,
    /// Descriptor lifecycle state.
    pub lifecycle_state: String,
    /// Descriptor capability scope class.
    pub capability_scope_class: String,
    /// Descriptor preview class.
    pub preview_class: String,
    /// Descriptor approval posture class.
    pub approval_posture_class: String,
    /// Descriptor AI-tool surfacing class.
    pub ai_tool_surfacing_class: String,
    /// Controlled or descriptor-native automation labels.
    pub automation_labels: Vec<String>,
    /// Invocation schema ref the route preserves.
    pub invocation_schema_ref: String,
    /// Result schema ref the route preserves.
    pub result_schema_ref: String,
    /// Durable result, rollback, reopen, and export governance for this command.
    pub result_packet_governance: M5ResultPacketGovernanceRecord,
    /// Whether the descriptor declares preview-gate metadata.
    pub preview_gate_declared: bool,
    /// Whether the command is high-risk or otherwise gated.
    pub high_risk: bool,
    /// Ordered per-surface parity rows.
    pub surface_rows: Vec<M5SurfaceGovernanceRow>,
    /// Machine-readable findings. Empty means conforming.
    pub finding_codes: Vec<String>,
}

/// Packet summary for support and release consumers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandGovernanceSummary {
    /// Number of commands under audit.
    pub command_count: usize,
    /// Number of per-surface rows.
    pub surface_row_count: usize,
    /// Number of high-risk commands.
    pub high_risk_command_count: usize,
    /// Number of commands with descriptor-declared preview gates.
    pub preview_gate_count: usize,
    /// Number of commands missing required preview-gate metadata.
    pub missing_preview_gate_count: usize,
    /// Number of commands that require durable result truth.
    pub durable_result_command_count: usize,
    /// Number of commands that join the activity center.
    pub activity_join_command_count: usize,
    /// Number of commands that require rollback handles.
    pub rollback_required_command_count: usize,
    /// Number of commands that require checkpoint refs.
    pub checkpoint_required_command_count: usize,
    /// Number of commands that join release evidence.
    pub release_evidence_join_count: usize,
    /// Number of total findings.
    pub finding_count: usize,
}

/// Canonical M5 command-governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandGovernancePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Schema ref for this packet.
    pub schema_ref: String,
    /// Companion doc ref.
    pub doc_ref: String,
    /// Source registry ref.
    pub source_registry_ref: String,
    /// Ordered governance rows.
    pub rows: Vec<M5CommandGovernanceRow>,
    /// Roll-up counts.
    pub summary: M5CommandGovernanceSummary,
}

impl M5CommandGovernancePacket {
    /// Renders a compact Markdown summary for checked artifacts.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Command Governance\n\n");
        out.push_str("| Metric | Value |\n|---|---:|\n");
        out.push_str(&format!("| Commands | {} |\n", self.summary.command_count));
        out.push_str(&format!(
            "| Surface rows | {} |\n",
            self.summary.surface_row_count
        ));
        out.push_str(&format!(
            "| High-risk commands | {} |\n",
            self.summary.high_risk_command_count
        ));
        out.push_str(&format!(
            "| Preview gates declared | {} |\n",
            self.summary.preview_gate_count
        ));
        out.push_str(&format!(
            "| Missing preview gates | {} |\n",
            self.summary.missing_preview_gate_count
        ));
        out.push_str(&format!(
            "| Durable result commands | {} |\n",
            self.summary.durable_result_command_count
        ));
        out.push_str(&format!(
            "| Activity-center joins | {} |\n",
            self.summary.activity_join_command_count
        ));
        out.push_str(&format!(
            "| Rollback-required commands | {} |\n",
            self.summary.rollback_required_command_count
        ));
        out.push_str(&format!(
            "| Checkpoint-required commands | {} |\n",
            self.summary.checkpoint_required_command_count
        ));
        out.push_str(&format!(
            "| Release-evidence joins | {} |\n",
            self.summary.release_evidence_join_count
        ));
        out.push_str(&format!(
            "| Findings | {} |\n\n",
            self.summary.finding_count
        ));

        out.push_str(
            "| Command | Result profile | Activity join | Rollback / checkpoints | Outcomes | Findings |\n",
        );
        out.push_str("|---|---|---|---|---|---|\n");
        for row in &self.rows {
            let findings = if row.finding_codes.is_empty() {
                "none".to_string()
            } else {
                row.finding_codes.join(", ")
            };
            let result = &row.result_packet_governance;
            let rollback_checkpoint = format!(
                "{}/{}",
                if result.artifacts.rollback_handle_required {
                    "rollback"
                } else {
                    "no_rollback"
                },
                if result.artifacts.checkpoint_ref_required {
                    "checkpoints"
                } else {
                    "no_checkpoints"
                }
            );
            let outcomes = result
                .outcome_rows
                .iter()
                .map(|row| row.outcome_class.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                row.command_id,
                result.execution_profile_class.as_str(),
                if result.joins_activity_center {
                    "joined"
                } else {
                    "not_required"
                },
                rollback_checkpoint,
                outcomes,
                findings
            ));
        }
        out.push('\n');
        out
    }
}

/// Support-export wrapper for the M5 governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CommandGovernanceSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet schema ref.
    pub schema_ref: String,
    /// Case ids useful for support joins.
    pub case_ids: Vec<String>,
    /// Quoted governance packet.
    pub packet: M5CommandGovernancePacket,
}

impl M5CommandGovernanceSupportExport {
    /// Builds a deterministic support-export wrapper from a packet.
    pub fn from_packet(support_export_id: String, packet: M5CommandGovernancePacket) -> Self {
        let mut case_ids = vec![packet.packet_id.clone()];
        for row in &packet.rows {
            case_ids.push(row.command_id.clone());
            case_ids.push(row.command_revision_ref.clone());
            for outcome in &row.result_packet_governance.outcome_rows {
                case_ids.push(outcome.export_safe_summary_ref.clone());
                case_ids.push(outcome.raw_packet_export_ref.clone());
                case_ids.push(outcome.support_export_case_ref.clone());
                case_ids.push(outcome.release_evidence_ref.clone());
            }
            if let Some(reference) = row
                .result_packet_governance
                .artifacts
                .exact_target_reopen_ref
                .clone()
            {
                case_ids.push(reference);
            }
            if let Some(reference) = row
                .result_packet_governance
                .artifacts
                .activity_join_ref
                .clone()
            {
                case_ids.push(reference);
            }
            if let Some(reference) = row
                .result_packet_governance
                .artifacts
                .support_export_identity_ref
                .clone()
            {
                case_ids.push(reference);
            }
            case_ids.push(
                row.result_packet_governance
                    .artifacts
                    .release_evidence_identity_ref
                    .clone(),
            );
        }
        case_ids.sort();
        case_ids.dedup();
        Self {
            record_kind: "m5_command_governance_support_export".to_string(),
            schema_version: 1,
            support_export_id,
            schema_ref: M5_COMMAND_GOVERNANCE_SCHEMA_REF.to_string(),
            case_ids,
            packet,
        }
    }
}

/// Validation error raised by [`validate_m5_command_governance_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CommandGovernanceValidationError {
    /// The packet has no rows.
    NoRows,
    /// A command row is missing one of the required surfaces.
    MissingRequiredSurface {
        /// Command id that regressed.
        command_id: String,
        /// Missing surface token.
        surface_class: String,
    },
    /// A high-risk command is missing preview-gate metadata.
    MissingPreviewGate {
        /// Command id that regressed.
        command_id: String,
    },
    /// A route widened preview or approval posture.
    ParityBroken {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// A packet lost actor, target, trust, or rollout lineage.
    MissingApprovalLineage {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// A route lost support-export safety.
    ExportSafetyBroken {
        /// Command id that regressed.
        command_id: String,
        /// Surface that regressed.
        surface_class: String,
    },
    /// A command row is missing canonical result-outcome coverage.
    MissingResultOutcomeCoverage {
        /// Command id that regressed.
        command_id: String,
        /// Missing outcome token.
        outcome_class: String,
    },
    /// A durable result lost its activity-center join or exact reopen.
    MissingDurableResultJoin {
        /// Command id that regressed.
        command_id: String,
    },
    /// A command requiring rollback or checkpoints failed to preserve them.
    MissingRollbackOrCheckpoint {
        /// Command id that regressed.
        command_id: String,
    },
    /// A command lost copy-safe summary, raw export, support-export, or release-evidence parity.
    MissingExportParity {
        /// Command id that regressed.
        command_id: String,
    },
}

impl fmt::Display for M5CommandGovernanceValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NoRows => write!(f, "m5 command governance packet has no rows"),
            Self::MissingRequiredSurface {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} is missing required surface {surface_class}"
            ),
            Self::MissingPreviewGate { command_id } => {
                write!(f, "high-risk command {command_id} is missing preview-gate metadata")
            }
            Self::ParityBroken {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} broke preview/approval/disabled-reason parity on {surface_class}"
            ),
            Self::MissingApprovalLineage {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} lost actor/target/trust/rollout lineage on {surface_class}"
            ),
            Self::ExportSafetyBroken {
                command_id,
                surface_class,
            } => write!(
                f,
                "command {command_id} is not export safe on {surface_class}"
            ),
            Self::MissingResultOutcomeCoverage {
                command_id,
                outcome_class,
            } => write!(
                f,
                "command {command_id} is missing canonical result outcome {outcome_class}"
            ),
            Self::MissingDurableResultJoin { command_id } => write!(
                f,
                "command {command_id} lost its durable activity-center join or exact reopen ref"
            ),
            Self::MissingRollbackOrCheckpoint { command_id } => write!(
                f,
                "command {command_id} requires rollback/checkpoint truth but did not preserve it"
            ),
            Self::MissingExportParity { command_id } => write!(
                f,
                "command {command_id} lost copy-safe summary, raw export, support-export, or release-evidence parity"
            ),
        }
    }
}

impl Error for M5CommandGovernanceValidationError {}

fn activity_report_projection() -> &'static ActivityReportProjection {
    ACTIVITY_REPORT_PROJECTION.get_or_init(|| {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/ux/m5/activity-center/report.json"
        )))
        .expect("checked M5 activity-object report must parse")
    })
}

fn activity_family_id_for_command(command_id: &str) -> Option<&'static str> {
    match command_id {
        "cmd:notebook.run_all_cells" => Some("activity:notebook_run"),
        "cmd:data_api.send_request" => Some("activity:query_run"),
        "cmd:profiler.start_capture" => Some("activity:profiler_capture"),
        "cmd:trace_replay.replay_session" => Some("activity:replay_session"),
        "cmd:review_pipeline.run_pipeline" => Some("activity:pipeline_action"),
        "cmd:preview.open_live_preview" => Some("activity:preview_route"),
        "cmd:incident.open_incident" => Some("activity:incident_packet"),
        "cmd:sync.push_workspace_state" => Some("activity:sync_state_change"),
        "cmd:offboarding.export_and_wipe" => Some("activity:offboarding_job"),
        _ => None,
    }
}

fn activity_row_for_command(command_id: &str) -> Option<&'static ActivityReportRowProjection> {
    let family_id = activity_family_id_for_command(command_id)?;
    activity_report_projection()
        .rows
        .iter()
        .find(|row| row.descriptor.family_id == family_id)
}

fn default_invocation_schema_ref(entry: &CommandRegistryEntryRecord) -> String {
    entry
        .descriptor
        .invocation_schema_ref
        .clone()
        .unwrap_or_else(|| "schemas/commands/command_invocation_session.schema.json".to_string())
}

fn default_result_schema_ref(entry: &CommandRegistryEntryRecord) -> String {
    entry
        .descriptor
        .result_schema_ref
        .clone()
        .unwrap_or_else(|| "schemas/commands/command_result_packet.schema.json".to_string())
}

fn preview_required(entry: &CommandRegistryEntryRecord) -> bool {
    entry.descriptor.preview_class != "no_preview_required"
}

fn approval_required(entry: &CommandRegistryEntryRecord) -> bool {
    entry.descriptor.approval_posture_class != "no_approval_required"
}

fn mutates_state(entry: &CommandRegistryEntryRecord) -> bool {
    matches!(
        entry.descriptor.capability_scope_class.as_str(),
        "reversible_local_mutation"
            | "recoverable_durable_mutation"
            | "destructive_bulk_mutation"
            | "credential_or_secret_bearing"
            | "managed_workspace_control"
    ) || !matches!(
        entry.dominant_side_effect_class.as_str(),
        "no_material_side_effect" | "opens_link"
    )
}

fn long_running(entry: &CommandRegistryEntryRecord) -> bool {
    activity_row_for_command(entry.command_id()).is_some()
}

fn durable_truth_required(entry: &CommandRegistryEntryRecord) -> bool {
    mutates_state(entry) || long_running(entry)
}

fn is_high_risk(entry: &CommandRegistryEntryRecord) -> bool {
    preview_required(entry)
        || approval_required(entry)
        || matches!(
            entry.descriptor.capability_scope_class.as_str(),
            "recoverable_durable_mutation"
                | "destructive_bulk_mutation"
                | "irreversible_publish"
                | "credential_or_secret_bearing"
                | "managed_workspace_control"
        )
}

fn approval_model(entry: &CommandRegistryEntryRecord) -> M5ApprovalModelClass {
    if approval_required(entry) {
        M5ApprovalModelClass::HumanConfirmRequired
    } else {
        M5ApprovalModelClass::NoApprovalRequired
    }
}

fn route_posture_for_surface(
    entry: &CommandRegistryEntryRecord,
    surface: M5GovernanceSurfaceClass,
) -> M5RoutePostureClass {
    match surface {
        M5GovernanceSurfaceClass::Desktop => descriptor_posture(entry),
        M5GovernanceSurfaceClass::Cli => {
            if labels_include(
                &entry.automation_labels,
                ControlledAutomationLabel::HeadlessSafe,
            ) {
                descriptor_posture(entry)
            } else {
                M5RoutePostureClass::DiagnosticsRequired
            }
        }
        M5GovernanceSurfaceClass::Ai => match entry.descriptor.ai_tool_surfacing_class.as_str() {
            "ai_callable_read_only" | "ai_callable_preview_then_confirm" => {
                descriptor_posture(entry)
            }
            _ => M5RoutePostureClass::DiagnosticsRequired,
        },
        M5GovernanceSurfaceClass::Recipe => {
            if labels_include(
                &entry.automation_labels,
                ControlledAutomationLabel::RecipeSafe,
            ) && !approval_required(entry)
            {
                descriptor_posture(entry)
            } else {
                M5RoutePostureClass::DiagnosticsRequired
            }
        }
        M5GovernanceSurfaceClass::Extension => descriptor_posture(entry),
        M5GovernanceSurfaceClass::BrowserCompanion => {
            if entry
                .descriptor
                .client_scopes
                .iter()
                .any(|scope| scope == "companion_surface")
            {
                descriptor_posture(entry)
            } else {
                M5RoutePostureClass::DesktopHandoffRequired
            }
        }
    }
}

fn descriptor_posture(entry: &CommandRegistryEntryRecord) -> M5RoutePostureClass {
    if approval_required(entry) {
        M5RoutePostureClass::ApprovalRequired
    } else if preview_required(entry) {
        M5RoutePostureClass::PreviewRequired
    } else {
        M5RoutePostureClass::DirectAllowed
    }
}

fn actor_ref(surface: M5GovernanceSurfaceClass) -> &'static str {
    match surface {
        M5GovernanceSurfaceClass::Desktop => "actor:desktop:local_user",
        M5GovernanceSurfaceClass::Cli => "actor:cli:local_user",
        M5GovernanceSurfaceClass::Ai => "actor:ai:assistant_on_behalf_of_user",
        M5GovernanceSurfaceClass::Recipe => "actor:recipe:workspace_runner",
        M5GovernanceSurfaceClass::Extension => "actor:extension:host_bridge",
        M5GovernanceSurfaceClass::BrowserCompanion => "actor:browser_companion:user",
    }
}

fn target_ref(entry: &CommandRegistryEntryRecord) -> String {
    format!(
        "target:{}",
        entry.descriptor.canonical_verb.replace('.', ":")
    )
}

fn trust_epoch_ref(entry: &CommandRegistryEntryRecord) -> String {
    format!(
        "trust-epoch:{}",
        entry.descriptor.policy_context.policy_epoch
    )
}

fn rollout_state_ref(entry: &CommandRegistryEntryRecord) -> String {
    format!(
        "rollout:{}:{}",
        entry.descriptor.release_channel, entry.descriptor.lifecycle_state
    )
}

fn copy_safe_introspection(entry: &CommandRegistryEntryRecord) -> M5CopySafeIntrospectionRecord {
    let why_not = why_not_automatable_reason(
        &entry.automation_labels,
        &entry.descriptor.approval_posture_class,
    );
    M5CopySafeIntrospectionRecord {
        copy_command_id: true,
        copy_cli_form: labels_include(
            &entry.automation_labels,
            ControlledAutomationLabel::HeadlessSafe,
        ),
        add_to_recipe: labels_include(
            &entry.automation_labels,
            ControlledAutomationLabel::RecipeSafe,
        ),
        inspect_why_not_automatable: why_not.is_some(),
    }
}

fn rollback_handle_required(entry: &CommandRegistryEntryRecord) -> bool {
    entry.preview_gate_metadata.as_ref().is_some_and(|metadata| {
        metadata.revert_posture_class == "rollback_or_checkpoint_available"
    })
}

fn checkpoint_ref_required(entry: &CommandRegistryEntryRecord) -> bool {
    rollback_handle_required(entry)
        || activity_row_for_command(entry.command_id()).is_some()
        || preview_required(entry)
}

fn checkpoint_ref_classes(entry: &CommandRegistryEntryRecord) -> Vec<String> {
    let mut refs = Vec::new();
    if activity_row_for_command(entry.command_id()).is_some() {
        refs.push("activity_resume_checkpoint_ref".to_string());
    }
    if rollback_handle_required(entry) {
        refs.push("rollback_replay_checkpoint_ref".to_string());
    }
    if refs.is_empty() && checkpoint_ref_required(entry) {
        refs.push("result_resume_checkpoint_ref".to_string());
    }
    refs
}

fn execution_profile(entry: &CommandRegistryEntryRecord) -> M5ResultExecutionProfileClass {
    if mutates_state(entry) {
        M5ResultExecutionProfileClass::DurableMutation
    } else if long_running(entry) {
        M5ResultExecutionProfileClass::DurableProgress
    } else {
        M5ResultExecutionProfileClass::EphemeralReadOnly
    }
}

fn export_posture_class(entry: &CommandRegistryEntryRecord) -> &'static str {
    if durable_truth_required(entry) {
        "exportable_with_redaction"
    } else {
        "metadata_exportable"
    }
}

fn portable_profile_allowed(entry: &CommandRegistryEntryRecord) -> bool {
    entry.descriptor.redaction_class != "support_sensitive_metadata"
}

fn support_bundle_allowed(entry: &CommandRegistryEntryRecord) -> bool {
    durable_truth_required(entry) || approval_required(entry) || preview_required(entry)
}

fn activity_support_export_identity_ref(command_id: &str) -> Option<String> {
    let report = activity_report_projection();
    activity_row_for_command(command_id).map(|row| {
        let support_ref = report
            .support_export_refs
            .first()
            .cloned()
            .unwrap_or_else(|| "support:m5-activity-objects".to_string());
        format!("{support_ref}:{}", row.descriptor.family_id)
    })
}

fn activity_shared_contract_ref(command_id: &str) -> Option<String> {
    activity_row_for_command(command_id)
        .map(|_| activity_report_projection().shared_contract_ref.clone())
}

fn exact_target_reopen_ref(command_id: &str) -> Option<String> {
    activity_row_for_command(command_id).map(|row| row.descriptor.reopen_anchor_ref.clone())
}

fn activity_join_ref(command_id: &str) -> Option<String> {
    activity_row_for_command(command_id).map(|row| row.descriptor.family_id.clone())
}

fn notification_join_ref(entry: &CommandRegistryEntryRecord) -> Option<String> {
    durable_truth_required(entry)
        .then(|| format!("notification:{}:canonical", entry.descriptor.canonical_verb))
}

fn activity_identity_is_stable(command_id: &str) -> bool {
    let Some(row) = activity_row_for_command(command_id) else {
        return true;
    };
    row.bindings.iter().any(|binding| {
        binding.guarantee == "support_export_identity"
            && binding.qualification_status == "qualified"
            && binding.projected_export_identity.as_deref() == Some("stable_reference")
    })
}

fn activity_state_for_outcome(
    entry: &CommandRegistryEntryRecord,
    outcome: M5ResultOutcomeClass,
) -> Option<String> {
    if !long_running(entry) {
        return None;
    }
    let value = match outcome {
        M5ResultOutcomeClass::Success => "completed",
        M5ResultOutcomeClass::PartialSuccess => "partially_completed",
        M5ResultOutcomeClass::Cancelled => "cancelled",
        M5ResultOutcomeClass::Superseded => "superseded",
        M5ResultOutcomeClass::Denied => "policy_suppressed",
        M5ResultOutcomeClass::Degraded => "running",
        M5ResultOutcomeClass::Failed => "failed",
    };
    Some(value.to_string())
}

fn result_code_for_outcome(outcome: M5ResultOutcomeClass) -> &'static str {
    match outcome {
        M5ResultOutcomeClass::Success => "succeeded",
        M5ResultOutcomeClass::PartialSuccess => "partially_applied",
        M5ResultOutcomeClass::Cancelled => "cancelled",
        M5ResultOutcomeClass::Superseded => "superseded",
        M5ResultOutcomeClass::Denied => "denied_by_enablement",
        M5ResultOutcomeClass::Degraded => "succeeded_with_degraded_truth",
        M5ResultOutcomeClass::Failed => "failed",
    }
}

fn build_outcome_row(
    entry: &CommandRegistryEntryRecord,
    outcome: M5ResultOutcomeClass,
) -> M5CommandOutcomeProjectionRow {
    let base = format!("result-packet:{}:{}", entry.descriptor.canonical_verb, outcome.as_str());
    M5CommandOutcomeProjectionRow {
        outcome_class: outcome,
        result_code: result_code_for_outcome(outcome).to_string(),
        activity_state_class: activity_state_for_outcome(entry, outcome),
        export_safe_summary_ref: format!("{base}:summary"),
        raw_packet_export_ref: format!("{base}:raw"),
        support_export_case_ref: format!("support:{}:{}", entry.descriptor.canonical_verb, outcome.as_str()),
        release_evidence_ref: format!(
            "release-evidence:{}:{}",
            entry.descriptor.canonical_verb,
            outcome.as_str()
        ),
        copy_safe: true,
    }
}

fn build_result_artifacts(entry: &CommandRegistryEntryRecord) -> M5ResultArtifactProjectionRecord {
    M5ResultArtifactProjectionRecord {
        result_contract_class: entry.descriptor.result_contract.result_contract_class.clone(),
        artifact_kind_ref: entry.descriptor.result_contract.artifact_kind_ref.clone(),
        evidence_ref_classes: entry
            .descriptor
            .result_contract
            .evidence_ref_class_required
            .clone(),
        created_object_ref_required: entry.descriptor.result_contract.artifact_kind_ref.is_some(),
        exact_target_reopen_ref: exact_target_reopen_ref(entry.command_id()),
        notification_join_ref: notification_join_ref(entry),
        activity_join_ref: activity_join_ref(entry.command_id()),
        rollback_handle_required: rollback_handle_required(entry),
        rollback_handle_posture: entry
            .preview_gate_metadata
            .as_ref()
            .map(|metadata| metadata.revert_posture_class.clone())
            .unwrap_or_else(|| "no_rollback_claimed".to_string()),
        checkpoint_ref_required: checkpoint_ref_required(entry),
        checkpoint_ref_classes: checkpoint_ref_classes(entry),
        support_export_identity_ref: activity_support_export_identity_ref(entry.command_id()),
        release_evidence_identity_ref: format!(
            "release-evidence:{}:current",
            entry.descriptor.canonical_verb
        ),
    }
}

fn build_result_packet_governance(
    entry: &CommandRegistryEntryRecord,
) -> M5ResultPacketGovernanceRecord {
    let durable = durable_truth_required(entry);
    let activity_join = long_running(entry);
    M5ResultPacketGovernanceRecord {
        execution_profile_class: execution_profile(entry),
        mutating: mutates_state(entry),
        long_running: long_running(entry),
        durable_truth_required: durable,
        invocation_schema_ref: default_invocation_schema_ref(entry),
        result_schema_ref: default_result_schema_ref(entry),
        export_posture_class: export_posture_class(entry).to_string(),
        redaction_class: entry.descriptor.redaction_class.clone(),
        preserves_copy_safe_summary: true,
        preserves_raw_packet_export: durable || approval_required(entry) || preview_required(entry),
        joins_activity_center: activity_join,
        joins_notification_surface: durable || approval_required(entry) || preview_required(entry),
        joins_support_export: true,
        joins_release_evidence: true,
        activity_shared_contract_ref: activity_shared_contract_ref(entry.command_id()),
        portable_profile_allowed: portable_profile_allowed(entry),
        support_bundle_allowed: support_bundle_allowed(entry),
        outcome_rows: M5ResultOutcomeClass::required_coverage()
            .into_iter()
            .map(|outcome| build_outcome_row(entry, outcome))
            .collect(),
        artifacts: build_result_artifacts(entry),
    }
}

fn preview_parity(entry: &CommandRegistryEntryRecord) -> M5InvocationPreviewParityRecord {
    M5InvocationPreviewParityRecord {
        preview_class: entry.descriptor.preview_class.clone(),
        dry_run_supported: preview_required(entry)
            || matches!(
                entry.descriptor.capability_scope_class.as_str(),
                "recoverable_durable_mutation"
                    | "destructive_bulk_mutation"
                    | "irreversible_publish"
                    | "credential_or_secret_bearing"
                    | "managed_workspace_control"
            ),
        copy_safe_introspection: copy_safe_introspection(entry),
        why_not_automatable_reason: why_not_automatable_reason(
            &entry.automation_labels,
            &entry.descriptor.approval_posture_class,
        ),
        export_safe: true,
    }
}

fn disabled_reason_record<'a>(
    entry: &'a CommandRegistryEntryRecord,
    code: &str,
) -> Option<&'a DisabledReasonRecord> {
    entry
        .disabled_reason_records
        .iter()
        .find(|record| record.disabled_reason_code.as_str() == code)
}

fn build_disabled_reason_packet(
    entry: &CommandRegistryEntryRecord,
    surface: M5GovernanceSurfaceClass,
    family: M5DisabledReasonFamilyClass,
    code_hint: &str,
) -> M5DisabledReasonPacketRecord {
    let record = disabled_reason_record(entry, code_hint);
    let disabled_reason_code = record
        .map(|record| record.disabled_reason_code.as_str().to_string())
        .unwrap_or_else(|| code_hint.to_string());
    let explanation_ref = record
        .map(|record| record.explanation_ref.clone())
        .unwrap_or_else(|| format!("reason:{}:{}", entry.descriptor.canonical_verb, code_hint));
    let repair_hook_id = record.map(|record| record.repair_hook_ref.hook_id.clone());

    M5DisabledReasonPacketRecord {
        packet_id: format!(
            "disabled-reason:{}:{}:{}",
            entry.descriptor.canonical_verb,
            surface.as_str(),
            family.as_str()
        ),
        command_id: entry.descriptor.command_id.clone(),
        surface_class: surface,
        reason_family: family,
        disabled_reason_code,
        explanation_ref,
        repair_hook_id,
        actor_ref: actor_ref(surface).to_string(),
        target_ref: target_ref(entry),
        trust_epoch_ref: trust_epoch_ref(entry),
        rollout_state_ref: rollout_state_ref(entry),
        redaction_class: entry.descriptor.redaction_class.clone(),
        support_export_safe: true,
    }
}

fn denial_packets_for_surface(
    entry: &CommandRegistryEntryRecord,
    surface: M5GovernanceSurfaceClass,
    route_posture: M5RoutePostureClass,
) -> Vec<M5DisabledReasonPacketRecord> {
    let mut packets = vec![
        build_disabled_reason_packet(
            entry,
            surface,
            M5DisabledReasonFamilyClass::TrustBlocked,
            "workspace_trust_restricted",
        ),
        build_disabled_reason_packet(
            entry,
            surface,
            M5DisabledReasonFamilyClass::PolicyBlocked,
            "policy_blocked_in_context",
        ),
        build_disabled_reason_packet(
            entry,
            surface,
            M5DisabledReasonFamilyClass::RolloutBlocked,
            "rollout_blocked",
        ),
        build_disabled_reason_packet(
            entry,
            surface,
            M5DisabledReasonFamilyClass::MissingRuntime,
            "execution_context_unavailable",
        ),
    ];

    let capability_code = if disabled_reason_record(entry, "required_provider_unlinked").is_some() {
        "required_provider_unlinked"
    } else if disabled_reason_record(entry, "required_credential_missing").is_some() {
        "required_credential_missing"
    } else if disabled_reason_record(entry, "managed_only_channel_required").is_some() {
        "managed_only_channel_required"
    } else {
        "missing_capability_state"
    };
    packets.push(build_disabled_reason_packet(
        entry,
        surface,
        M5DisabledReasonFamilyClass::MissingCapabilityState,
        capability_code,
    ));

    if matches!(
        route_posture,
        M5RoutePostureClass::DiagnosticsRequired | M5RoutePostureClass::DesktopHandoffRequired
    ) {
        packets.push(build_disabled_reason_packet(
            entry,
            surface,
            M5DisabledReasonFamilyClass::FocusMismatch,
            "client_scope_excludes_surface",
        ));
    }
    if preview_required(entry) {
        packets.push(build_disabled_reason_packet(
            entry,
            surface,
            M5DisabledReasonFamilyClass::PreviewRequired,
            "preview_required_not_shown",
        ));
    }
    if approval_required(entry) {
        packets.push(build_disabled_reason_packet(
            entry,
            surface,
            M5DisabledReasonFamilyClass::ApprovalRequired,
            "approval_denial_no_approval_path",
        ));
    }

    packets
}

fn approval_decision_class(
    entry: &CommandRegistryEntryRecord,
    route_posture: M5RoutePostureClass,
) -> &'static str {
    match route_posture {
        M5RoutePostureClass::DesktopHandoffRequired | M5RoutePostureClass::DiagnosticsRequired => {
            if approval_required(entry) {
                "approval_denied"
            } else {
                "not_required"
            }
        }
        M5RoutePostureClass::ApprovalRequired => "approval_pending",
        _ => "not_required",
    }
}

fn build_approval_packet(
    entry: &CommandRegistryEntryRecord,
    surface: M5GovernanceSurfaceClass,
    route_posture: M5RoutePostureClass,
) -> M5ApprovalParityPacketRecord {
    M5ApprovalParityPacketRecord {
        packet_id: format!(
            "approval:{}:{}",
            entry.descriptor.canonical_verb,
            surface.as_str()
        ),
        command_id: entry.descriptor.command_id.clone(),
        surface_class: surface,
        preview_class: entry.descriptor.preview_class.clone(),
        approval_model_class: approval_model(entry),
        decision_class: approval_decision_class(entry, route_posture).to_string(),
        actor_ref: actor_ref(surface).to_string(),
        target_ref: target_ref(entry),
        trust_epoch_ref: trust_epoch_ref(entry),
        rollout_state_ref: rollout_state_ref(entry),
        no_bypass_rule_preserved: true,
        support_export_safe: true,
    }
}

fn build_surface_row(
    entry: &CommandRegistryEntryRecord,
    surface: M5GovernanceSurfaceClass,
) -> M5SurfaceGovernanceRow {
    let route_posture = route_posture_for_surface(entry, surface);
    let result_governance = build_result_packet_governance(entry);
    M5SurfaceGovernanceRow {
        surface_class: surface,
        client_scope: match surface {
            M5GovernanceSurfaceClass::Desktop => "desktop_product",
            M5GovernanceSurfaceClass::Cli => "cli",
            M5GovernanceSurfaceClass::Ai => "ai_tool_surface",
            M5GovernanceSurfaceClass::Recipe => "recipe_runner",
            M5GovernanceSurfaceClass::Extension => "extension_host",
            M5GovernanceSurfaceClass::BrowserCompanion => "companion_surface",
        }
        .to_string(),
        route_posture_class: route_posture,
        preview_parity: preview_parity(entry),
        disabled_reason_packets: denial_packets_for_surface(entry, surface, route_posture),
        approval_parity_packet: build_approval_packet(entry, surface, route_posture),
        result_packet_parity_preserved: true,
        activity_join_parity_preserved: !result_governance.joins_activity_center
            || result_governance.artifacts.activity_join_ref.is_some(),
        export_join_parity_preserved: result_governance.joins_support_export
            && result_governance.joins_release_evidence,
        preview_parity_preserved: true,
        approval_parity_preserved: true,
        disabled_reason_parity_preserved: true,
    }
}

fn build_row(entry: &CommandRegistryEntryRecord) -> M5CommandGovernanceRow {
    let result_packet_governance = build_result_packet_governance(entry);
    let surface_rows = M5GovernanceSurfaceClass::required_coverage()
        .into_iter()
        .map(|surface| build_surface_row(entry, surface))
        .collect::<Vec<_>>();
    let high_risk = is_high_risk(entry);
    let mut finding_codes = Vec::new();
    if high_risk && entry.preview_gate_metadata.is_none() {
        finding_codes.push("missing_preview_gate_metadata".to_string());
    }

    M5CommandGovernanceRow {
        command_id: entry.descriptor.command_id.clone(),
        command_revision_ref: entry.descriptor.command_revision_ref.clone(),
        canonical_verb: entry.descriptor.canonical_verb.clone(),
        lifecycle_state: entry.descriptor.lifecycle_state.clone(),
        capability_scope_class: entry.descriptor.capability_scope_class.clone(),
        preview_class: entry.descriptor.preview_class.clone(),
        approval_posture_class: entry.descriptor.approval_posture_class.clone(),
        ai_tool_surfacing_class: entry.descriptor.ai_tool_surfacing_class.clone(),
        automation_labels: if entry.descriptor.automation_labels.is_empty() {
            entry.automation_labels.clone()
        } else {
            entry.descriptor.automation_labels.clone()
        },
        invocation_schema_ref: default_invocation_schema_ref(entry),
        result_schema_ref: default_result_schema_ref(entry),
        result_packet_governance,
        preview_gate_declared: entry.preview_gate_metadata.is_some(),
        high_risk,
        surface_rows,
        finding_codes,
    }
}

/// Builds the seeded M5 command-governance packet from the canonical registry.
pub fn seeded_m5_command_governance_packet() -> M5CommandGovernancePacket {
    let rows = M5_COMMAND_IDS
        .iter()
        .map(|command_id| {
            seeded_registry()
                .get(command_id)
                .unwrap_or_else(|| panic!("M5 governance command {command_id} must exist"))
        })
        .map(build_row)
        .collect::<Vec<_>>();

    let summary = M5CommandGovernanceSummary {
        command_count: rows.len(),
        surface_row_count: rows.iter().map(|row| row.surface_rows.len()).sum(),
        high_risk_command_count: rows.iter().filter(|row| row.high_risk).count(),
        preview_gate_count: rows.iter().filter(|row| row.preview_gate_declared).count(),
        missing_preview_gate_count: rows
            .iter()
            .filter(|row| row.high_risk && !row.preview_gate_declared)
            .count(),
        durable_result_command_count: rows
            .iter()
            .filter(|row| row.result_packet_governance.durable_truth_required)
            .count(),
        activity_join_command_count: rows
            .iter()
            .filter(|row| row.result_packet_governance.joins_activity_center)
            .count(),
        rollback_required_command_count: rows
            .iter()
            .filter(|row| row.result_packet_governance.artifacts.rollback_handle_required)
            .count(),
        checkpoint_required_command_count: rows
            .iter()
            .filter(|row| row.result_packet_governance.artifacts.checkpoint_ref_required)
            .count(),
        release_evidence_join_count: rows
            .iter()
            .filter(|row| row.result_packet_governance.joins_release_evidence)
            .count(),
        finding_count: rows.iter().map(|row| row.finding_codes.len()).sum(),
    };

    M5CommandGovernancePacket {
        record_kind: M5_COMMAND_GOVERNANCE_RECORD_KIND.to_string(),
        schema_version: M5_COMMAND_GOVERNANCE_SCHEMA_VERSION,
        packet_id: M5_COMMAND_GOVERNANCE_PACKET_ID.to_string(),
        generated_at: GENERATED_AT.to_string(),
        schema_ref: M5_COMMAND_GOVERNANCE_SCHEMA_REF.to_string(),
        doc_ref: M5_COMMAND_GOVERNANCE_DOC_REF.to_string(),
        source_registry_ref: SOURCE_REGISTRY_REF.to_string(),
        rows,
        summary,
    }
}

/// Returns the current seeded packet after validating it.
pub fn current_m5_command_governance_export(
) -> Result<M5CommandGovernancePacket, Vec<M5CommandGovernanceValidationError>> {
    let packet = seeded_m5_command_governance_packet();
    validate_m5_command_governance_packet(&packet)?;
    Ok(packet)
}

/// Validates the canonical M5 command-governance packet.
pub fn validate_m5_command_governance_packet(
    packet: &M5CommandGovernancePacket,
) -> Result<(), Vec<M5CommandGovernanceValidationError>> {
    let mut errors = Vec::new();
    if packet.rows.is_empty() {
        errors.push(M5CommandGovernanceValidationError::NoRows);
    }

    for row in &packet.rows {
        for required in M5GovernanceSurfaceClass::required_coverage() {
            if !row
                .surface_rows
                .iter()
                .any(|surface| surface.surface_class == required)
            {
                errors.push(M5CommandGovernanceValidationError::MissingRequiredSurface {
                    command_id: row.command_id.clone(),
                    surface_class: required.as_str().to_string(),
                });
            }
        }

        if row.high_risk && !row.preview_gate_declared {
            errors.push(M5CommandGovernanceValidationError::MissingPreviewGate {
                command_id: row.command_id.clone(),
            });
        }

        for required in M5ResultOutcomeClass::required_coverage() {
            if !row
                .result_packet_governance
                .outcome_rows
                .iter()
                .any(|outcome| outcome.outcome_class == required)
            {
                errors.push(
                    M5CommandGovernanceValidationError::MissingResultOutcomeCoverage {
                        command_id: row.command_id.clone(),
                        outcome_class: required.as_str().to_string(),
                    },
                );
            }
        }

        let result = &row.result_packet_governance;
        if result.durable_truth_required
            && (!result.preserves_copy_safe_summary
                || !result.preserves_raw_packet_export
                || !result.joins_support_export
                || !result.joins_release_evidence)
        {
            errors.push(M5CommandGovernanceValidationError::MissingExportParity {
                command_id: row.command_id.clone(),
            });
        }

        if result.joins_activity_center
            && (result.artifacts.activity_join_ref.is_none()
                || result.artifacts.exact_target_reopen_ref.is_none()
                || result.activity_shared_contract_ref.is_none()
                || !activity_identity_is_stable(&row.command_id))
        {
            errors.push(M5CommandGovernanceValidationError::MissingDurableResultJoin {
                command_id: row.command_id.clone(),
            });
        }

        if (result.artifacts.rollback_handle_required
            && result.artifacts.rollback_handle_posture == "no_rollback_claimed")
            || (result.artifacts.checkpoint_ref_required
                && result.artifacts.checkpoint_ref_classes.is_empty())
        {
            errors.push(M5CommandGovernanceValidationError::MissingRollbackOrCheckpoint {
                command_id: row.command_id.clone(),
            });
        }

        for surface in &row.surface_rows {
            if !surface.preview_parity_preserved
                || !surface.approval_parity_preserved
                || !surface.disabled_reason_parity_preserved
                || !surface.approval_parity_packet.no_bypass_rule_preserved
                || !surface.result_packet_parity_preserved
                || !surface.activity_join_parity_preserved
                || !surface.export_join_parity_preserved
            {
                errors.push(M5CommandGovernanceValidationError::ParityBroken {
                    command_id: row.command_id.clone(),
                    surface_class: surface.surface_class.as_str().to_string(),
                });
            }

            if surface.approval_parity_packet.actor_ref.trim().is_empty()
                || surface.approval_parity_packet.target_ref.trim().is_empty()
                || surface
                    .approval_parity_packet
                    .trust_epoch_ref
                    .trim()
                    .is_empty()
                || surface
                    .approval_parity_packet
                    .rollout_state_ref
                    .trim()
                    .is_empty()
            {
                errors.push(M5CommandGovernanceValidationError::MissingApprovalLineage {
                    command_id: row.command_id.clone(),
                    surface_class: surface.surface_class.as_str().to_string(),
                });
            }

            if !surface.preview_parity.export_safe
                || !surface.approval_parity_packet.support_export_safe
                || surface
                    .disabled_reason_packets
                    .iter()
                    .any(|packet| !packet.support_export_safe)
            {
                errors.push(M5CommandGovernanceValidationError::ExportSafetyBroken {
                    command_id: row.command_id.clone(),
                    surface_class: surface.surface_class.as_str().to_string(),
                });
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
