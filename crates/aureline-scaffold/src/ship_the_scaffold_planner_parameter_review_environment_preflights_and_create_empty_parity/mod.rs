//! Scaffold planner, parameter review, environment preflights, and create-empty parity rows.
//!
//! This module locks the canonical, export-safe packet for the scaffold planner.
//! Each [`ScaffoldPlanRow`] binds one prepared scaffold plan — a templated
//! starter or a create-empty workspace — to its parameter-review state, its
//! environment-preflight state, its previewed write impact and rollback
//! boundary, its create-empty parity posture, and its readiness, so the gallery,
//! preflight sheet, parameter-review sheet, run, recovery, diagnostics, and
//! support surfaces project the same plan, preflight, and parity truth instead of
//! writing before review or presenting a create-empty workspace as if it skipped
//! the safety pipeline.
//!
//! The packet is metadata only. Raw parameter values, secrets, absolute paths,
//! repository URLs, manifest bodies, hook bodies, command output, and
//! user-authored template content never cross this boundary; rows carry opaque
//! refs, closed-vocabulary class tokens, and short reviewable summaries. It
//! references the upstream scaffold-run, template-manifest, and hook-policy
//! contracts by id rather than embedding them.
//!
//! [`ScaffoldPlannerPacket::apply_downgrade_automation`] narrows plans whose
//! required parameters are unresolved, whose environment preflight failed, whose
//! write-impact preview or rollback boundary is unavailable, whose create-empty
//! parity broke, or whose proof or upstream dependency narrowed — withholding
//! apply and labeling the blocking state rather than hiding the plan, so CI or
//! release tooling narrows a stale or underqualified plan before it is offered.
//!
//! The boundary schema is
//! [`schemas/templates/ship-the-scaffold-planner-parameter-review-environment-preflights-and-create-empty-parity.schema.json`](../../../../schemas/templates/ship-the-scaffold-planner-parameter-review-environment-preflights-and-create-empty-parity.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity.md`](../../../../docs/frameworks/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/`](../../../../fixtures/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ScaffoldPlannerPacket`].
pub const SCAFFOLD_PLANNER_RECORD_KIND: &str =
    "scaffold_planner_parameter_review_and_create_empty_parity_rows";

/// Schema version for scaffold-planner packets.
pub const SCAFFOLD_PLANNER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SCAFFOLD_PLANNER_SCHEMA_REF: &str =
    "schemas/templates/ship-the-scaffold-planner-parameter-review-environment-preflights-and-create-empty-parity.schema.json";

/// Repo-relative path of the scaffold-planner contract doc.
pub const SCAFFOLD_PLANNER_DOC_REF: &str =
    "docs/frameworks/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity.md";

/// Repo-relative path of the upstream scaffold-run (preflight and applied run) contract.
pub const SCAFFOLD_RUN_CONTRACT_REF: &str = "schemas/templates/scaffold_run_alpha.schema.json";

/// Repo-relative path of the upstream template-manifest (required parameters) contract.
pub const TEMPLATE_MANIFEST_CONTRACT_REF: &str =
    "schemas/templates/template_manifest_alpha.schema.json";

/// Repo-relative path of the upstream scaffold hook-policy contract.
pub const SCAFFOLD_HOOK_POLICY_CONTRACT_REF: &str =
    "schemas/templates/scaffold_hook_policy.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const SCAFFOLD_PLANNER_FIXTURE_DIR: &str =
    "fixtures/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity";

/// Repo-relative path of the checked support-export artifact.
pub const SCAFFOLD_PLANNER_ARTIFACT_REF: &str =
    "artifacts/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SCAFFOLD_PLANNER_SUMMARY_REF: &str =
    "artifacts/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity.md";

/// Kind of scaffold plan a row prepares.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldPlanKind {
    /// A plan generated from a signed template manifest.
    TemplateScaffold,
    /// A plan that creates an empty, runnable workspace without a starter.
    CreateEmptyWorkspace,
    /// A create-empty plan that bridges a templated archetype to the empty flow for parity.
    CreateEmptyParityBridge,
}

impl ScaffoldPlanKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TemplateScaffold => "template_scaffold",
            Self::CreateEmptyWorkspace => "create_empty_workspace",
            Self::CreateEmptyParityBridge => "create_empty_parity_bridge",
        }
    }

    /// Whether this plan is one of the create-empty kinds that must reach parity.
    pub const fn is_create_empty(self) -> bool {
        matches!(
            self,
            Self::CreateEmptyWorkspace | Self::CreateEmptyParityBridge
        )
    }

    /// Whether this plan is generated from a template manifest.
    pub const fn is_template(self) -> bool {
        matches!(self, Self::TemplateScaffold | Self::CreateEmptyParityBridge)
    }
}

/// Parameter-review state for a plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterReviewClass {
    /// Every parameter resolved and reviewed.
    AllResolvedReviewed,
    /// Defaults accepted after review.
    DefaultsAcceptedReviewed,
    /// Awaiting required input before apply.
    AwaitingRequiredInput,
    /// One or more parameters are invalid; apply is blocked.
    InvalidParametersBlocked,
    /// No parameters are required for this plan.
    NoParametersRequired,
    /// Review required before apply.
    ReviewRequired,
}

impl ParameterReviewClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllResolvedReviewed => "all_resolved_reviewed",
            Self::DefaultsAcceptedReviewed => "defaults_accepted_reviewed",
            Self::AwaitingRequiredInput => "awaiting_required_input",
            Self::InvalidParametersBlocked => "invalid_parameters_blocked",
            Self::NoParametersRequired => "no_parameters_required",
            Self::ReviewRequired => "review_required",
        }
    }

    /// Whether this state blocks apply.
    pub const fn blocks_apply(self) -> bool {
        matches!(
            self,
            Self::AwaitingRequiredInput | Self::InvalidParametersBlocked | Self::ReviewRequired
        )
    }
}

/// Environment-preflight state for a plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvironmentPreflightClass {
    /// Every preflight check passed.
    AllPreflightsPassed,
    /// Preflights passed with non-blocking warnings.
    PassedWithWarnings,
    /// Optional tooling is missing but the plan can still apply.
    MissingOptionalTooling,
    /// A blocking prerequisite is missing; apply is blocked.
    BlockingPrerequisiteMissing,
    /// A preflight check failed; apply is blocked.
    PreflightFailedBlocked,
    /// Preflight was skipped; review required.
    PreflightSkippedReviewRequired,
}

impl EnvironmentPreflightClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllPreflightsPassed => "all_preflights_passed",
            Self::PassedWithWarnings => "passed_with_warnings",
            Self::MissingOptionalTooling => "missing_optional_tooling",
            Self::BlockingPrerequisiteMissing => "blocking_prerequisite_missing",
            Self::PreflightFailedBlocked => "preflight_failed_blocked",
            Self::PreflightSkippedReviewRequired => "preflight_skipped_review_required",
        }
    }

    /// Whether this state blocks apply.
    pub const fn blocks_apply(self) -> bool {
        matches!(
            self,
            Self::BlockingPrerequisiteMissing
                | Self::PreflightFailedBlocked
                | Self::PreflightSkippedReviewRequired
        )
    }
}

/// Create-empty parity posture for a plan.
///
/// Parity is the property that a create-empty workspace reaches the same safety
/// pipeline — parameter review, environment preflight, write-impact preview, and
/// rollback boundary — as a templated scaffold, instead of being treated as an
/// unreviewed shortcut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CreateEmptyParityClass {
    /// Full parity with the templated scaffold flow.
    FullParityWithTemplateFlow,
    /// Parity with documented, inspectable differences.
    ParityWithDocumentedDifferences,
    /// Reduced parity; review required before apply.
    ReducedParityReviewRequired,
    /// Parity broke; apply is blocked.
    ParityBrokenBlocked,
}

impl CreateEmptyParityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParityWithTemplateFlow => "full_parity_with_template_flow",
            Self::ParityWithDocumentedDifferences => "parity_with_documented_differences",
            Self::ReducedParityReviewRequired => "reduced_parity_review_required",
            Self::ParityBrokenBlocked => "parity_broken_blocked",
        }
    }

    /// Whether this posture blocks apply.
    pub const fn blocks_apply(self) -> bool {
        matches!(
            self,
            Self::ReducedParityReviewRequired | Self::ParityBrokenBlocked
        )
    }

    /// Whether this posture claims full parity guarantees.
    pub const fn is_full_parity(self) -> bool {
        matches!(self, Self::FullParityWithTemplateFlow)
    }
}

/// Rollback posture recorded for a plan's previewed write impact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanRollbackPostureClass {
    /// The full rollback boundary is recorded and reversible.
    FullRollbackBoundaryRecorded,
    /// Rollback is partial and requires documented manual steps.
    PartialRollbackWithManualSteps,
    /// No writes have happened yet; the plan is fully reversible by discarding it.
    NoWritesYetFullyReversible,
    /// Rollback is unavailable; review required before apply.
    RollbackUnavailableReviewRequired,
}

impl PlanRollbackPostureClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRollbackBoundaryRecorded => "full_rollback_boundary_recorded",
            Self::PartialRollbackWithManualSteps => "partial_rollback_with_manual_steps",
            Self::NoWritesYetFullyReversible => "no_writes_yet_fully_reversible",
            Self::RollbackUnavailableReviewRequired => "rollback_unavailable_review_required",
        }
    }

    /// Whether this posture blocks apply.
    pub const fn blocks_apply(self) -> bool {
        matches!(self, Self::RollbackUnavailableReviewRequired)
    }
}

/// Readiness of a plan to apply its writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanReadinessState {
    /// Reviewed, preflighted, previewed, and ready to apply.
    ReadyForApply,
    /// Preview only; awaiting explicit confirmation before any write.
    PreviewOnlyAwaitingConfirmation,
    /// Blocked awaiting required parameter input.
    BlockedAwaitingInput,
    /// Blocked on a failed environment preflight.
    BlockedFailedPreflight,
    /// Blocked because create-empty parity broke.
    BlockedParityBroken,
    /// Review required before the plan can advance.
    ReviewRequired,
}

impl PlanReadinessState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadyForApply => "ready_for_apply",
            Self::PreviewOnlyAwaitingConfirmation => "preview_only_awaiting_confirmation",
            Self::BlockedAwaitingInput => "blocked_awaiting_input",
            Self::BlockedFailedPreflight => "blocked_failed_preflight",
            Self::BlockedParityBroken => "blocked_parity_broken",
            Self::ReviewRequired => "review_required",
        }
    }

    /// Whether a plan in this state may be admitted for apply.
    pub const fn admits_apply(self) -> bool {
        matches!(self, Self::ReadyForApply)
    }
}

/// Downgrade trigger that can narrow a plan below its claimed readiness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldPlannerDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// A required parameter is unresolved.
    RequiredParameterUnresolved,
    /// Parameter validation failed.
    ParameterValidationFailed,
    /// A blocking environment prerequisite is missing.
    EnvironmentPrerequisiteMissing,
    /// An environment preflight check failed.
    PreflightFailed,
    /// The write-impact preview is unavailable.
    WriteImpactPreviewUnavailable,
    /// The rollback boundary is unavailable.
    RollbackBoundaryUnavailable,
    /// Create-empty parity broke.
    CreateEmptyParityBroken,
    /// The signed template revision is unavailable.
    TemplateRevisionUnavailable,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl ScaffoldPlannerDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::RequiredParameterUnresolved,
        Self::ParameterValidationFailed,
        Self::EnvironmentPrerequisiteMissing,
        Self::PreflightFailed,
        Self::WriteImpactPreviewUnavailable,
        Self::RollbackBoundaryUnavailable,
        Self::CreateEmptyParityBroken,
        Self::TemplateRevisionUnavailable,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::RequiredParameterUnresolved => "required_parameter_unresolved",
            Self::ParameterValidationFailed => "parameter_validation_failed",
            Self::EnvironmentPrerequisiteMissing => "environment_prerequisite_missing",
            Self::PreflightFailed => "preflight_failed",
            Self::WriteImpactPreviewUnavailable => "write_impact_preview_unavailable",
            Self::RollbackBoundaryUnavailable => "rollback_boundary_unavailable",
            Self::CreateEmptyParityBroken => "create_empty_parity_broken",
            Self::TemplateRevisionUnavailable => "template_revision_unavailable",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a plan's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScaffoldPlannerConsumerSurface {
    /// Template/starter gallery.
    Gallery,
    /// Scaffold preflight sheet.
    Preflight,
    /// Parameter-review sheet.
    ParameterReview,
    /// Scaffold run surface.
    RunSurface,
    /// Recovery / rollback surface.
    Recovery,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
}

impl ScaffoldPlannerConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Gallery,
        Self::Preflight,
        Self::ParameterReview,
        Self::RunSurface,
        Self::Recovery,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gallery => "gallery",
            Self::Preflight => "preflight",
            Self::ParameterReview => "parameter_review",
            Self::RunSurface => "run_surface",
            Self::Recovery => "recovery",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
        }
    }
}

/// Parameter-review block for a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldParameterReview {
    /// Parameter-review state.
    pub review_class: ParameterReviewClass,
    /// Total parameters offered by the plan.
    pub total_parameters: u32,
    /// Parameters resolved to a reviewed value.
    pub resolved_parameters: u32,
    /// Parameters that are required.
    pub required_parameters: u32,
    /// Required parameters still unresolved.
    pub unresolved_required_parameters: u32,
    /// Parameters that failed validation.
    pub invalid_parameters: u32,
    /// Opaque parameter-review refs.
    pub parameter_review_refs: Vec<String>,
    /// True when reviewing parameters performs no mutation before apply.
    pub no_mutation_during_review: bool,
}

/// Environment-preflight block for a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldEnvironmentPreflight {
    /// Environment-preflight state.
    pub preflight_class: EnvironmentPreflightClass,
    /// Total preflight checks run.
    pub total_checks: u32,
    /// Checks that passed.
    pub passed_checks: u32,
    /// Checks that warned but did not block.
    pub warning_checks: u32,
    /// Checks that blocked apply.
    pub blocking_failures: u32,
    /// Opaque preflight-check refs.
    pub preflight_check_refs: Vec<String>,
}

/// Create-empty parity block for a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldCreateEmptyParity {
    /// Parity posture.
    pub parity_class: CreateEmptyParityClass,
    /// True when this plan shares the templated preflight pipeline.
    pub shares_preflight_pipeline: bool,
    /// True when this plan shares the templated parameter-review step.
    pub shares_parameter_review: bool,
    /// True when this plan shares the templated rollback boundary.
    pub shares_rollback_boundary: bool,
    /// Opaque parity-note refs documenting any differences.
    pub parity_note_refs: Vec<String>,
}

/// Previewed write-impact block for a plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldWriteImpactPreview {
    /// Files the plan would create.
    pub files_created: u32,
    /// Files the plan would modify.
    pub files_modified: u32,
    /// Directories the plan would create.
    pub directories_created: u32,
    /// True when no write happens before explicit confirmation.
    pub no_writes_before_confirmation: bool,
    /// Opaque write-impact preview ref.
    pub write_impact_preview_ref: String,
    /// Rollback posture for the previewed impact.
    pub rollback_posture: PlanRollbackPostureClass,
}

/// One scaffold-planner row: a single prepared plan and its truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPlanRow {
    /// Opaque stable plan id.
    pub plan_id: String,
    /// Kind of scaffold plan.
    pub plan_kind: ScaffoldPlanKind,
    /// Opaque template id for template-backed plans.
    pub template_id: Option<String>,
    /// Plan revision semver.
    pub plan_revision_semver: String,
    /// Opaque ref to the upstream template manifest for template-backed plans.
    pub manifest_ref: Option<String>,
    /// Opaque ref to the upstream scaffold-run record this plan projects.
    pub scaffold_run_ref: String,
    /// Short reviewable scope summary.
    pub scope_summary: String,
    /// Parameter-review block.
    pub parameter_review: ScaffoldParameterReview,
    /// Environment-preflight block.
    pub environment_preflight: ScaffoldEnvironmentPreflight,
    /// Create-empty parity block.
    pub create_empty_parity: ScaffoldCreateEmptyParity,
    /// Previewed write-impact block.
    pub write_impact_preview: ScaffoldWriteImpactPreview,
    /// Readiness of this plan to apply.
    pub readiness_state: PlanReadinessState,
    /// Whether this plan is admitted for apply.
    pub admitted_for_apply: bool,
    /// Opaque known-blocker refs disclosed before apply.
    pub known_blocker_refs: Vec<String>,
    /// Downgrade triggers that apply to this plan.
    pub downgrade_triggers: Vec<ScaffoldPlannerDowngradeTrigger>,
    /// Consumer surfaces that must project this plan.
    pub consumer_surfaces: Vec<ScaffoldPlannerConsumerSurface>,
}

impl ScaffoldPlanRow {
    /// Whether this plan is one of the create-empty kinds that must reach parity.
    pub const fn is_create_empty(&self) -> bool {
        self.plan_kind.is_create_empty()
    }
}

/// Safety review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPlannerSafetyReview {
    /// No write happens before the plan is reviewed.
    pub no_writes_before_review: bool,
    /// Parameters are reviewed before apply.
    pub parameters_reviewed_before_apply: bool,
    /// The environment is preflighted before apply.
    pub environment_preflighted_before_apply: bool,
    /// Write impact is previewed before apply.
    pub write_impact_previewed_before_apply: bool,
    /// The rollback boundary is recorded before apply.
    pub rollback_boundary_recorded: bool,
    /// Create-empty plans reach preflight parity with the templated flow.
    pub create_empty_reaches_preflight_parity: bool,
    /// Create-empty plans reach rollback parity with the templated flow.
    pub create_empty_reaches_rollback_parity: bool,
    /// A blocking preflight blocks apply.
    pub blocking_preflight_blocks_apply: bool,
    /// Unresolved required parameters block apply.
    pub unresolved_parameters_block_apply: bool,
    /// Downgrade narrows the plan's claim rather than hiding the plan.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified plans automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
    /// No credential bodies or raw payloads cross the export boundary.
    pub no_credential_or_raw_payload_in_export: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPlannerConsumerProjection {
    /// Gallery shows plan kind and readiness.
    pub gallery_shows_plan_kind_and_readiness: bool,
    /// Preflight shows environment checks.
    pub preflight_shows_environment_checks: bool,
    /// Parameter-review sheet shows resolution state.
    pub parameter_review_shows_resolution_state: bool,
    /// Run surface shows write impact and rollback.
    pub run_surface_shows_write_impact_and_rollback: bool,
    /// Recovery surface shows the rollback boundary.
    pub recovery_shows_rollback_boundary: bool,
    /// CLI / headless shows plan rows.
    pub cli_headless_shows_plan_rows: bool,
    /// Support export shows plan rows.
    pub support_export_shows_plan_rows: bool,
    /// Diagnostics shows readiness state.
    pub diagnostics_shows_readiness_state: bool,
    /// Blocked plans are visibly labeled rather than hidden.
    pub blocked_plans_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPlannerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected plans.
    pub auto_narrow_on_stale: bool,
}

/// Per-plan observation fed to [`ScaffoldPlannerPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldPlanRowObservation {
    /// Plan id the observation applies to.
    pub plan_id: String,
    /// True when every required parameter currently resolves.
    pub parameters_resolved: bool,
    /// True when the environment currently satisfies the plan's prerequisites.
    pub environment_ready: bool,
    /// True when the write-impact preview is currently available.
    pub write_impact_preview_available: bool,
    /// True when the rollback boundary is currently available.
    pub rollback_boundary_available: bool,
    /// True when create-empty parity is currently intact (ignored for templated-only plans).
    pub create_empty_parity_intact: bool,
    /// True when the plan's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the plan narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`ScaffoldPlannerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScaffoldPlannerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable planner label.
    pub planner_label: String,
    /// Plan rows.
    pub rows: Vec<ScaffoldPlanRow>,
    /// Safety review block.
    pub safety_review: ScaffoldPlannerSafetyReview,
    /// Consumer projection block.
    pub consumer_projection: ScaffoldPlannerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScaffoldPlannerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe scaffold-planner packet with parameter, preflight, and parity rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScaffoldPlannerPacket {
    /// Record kind; must equal [`SCAFFOLD_PLANNER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCAFFOLD_PLANNER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable planner label.
    pub planner_label: String,
    /// Plan rows.
    pub rows: Vec<ScaffoldPlanRow>,
    /// Safety review block.
    pub safety_review: ScaffoldPlannerSafetyReview,
    /// Consumer projection block.
    pub consumer_projection: ScaffoldPlannerConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ScaffoldPlannerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ScaffoldPlannerPacket {
    /// Builds a scaffold-planner packet from stable-row input.
    pub fn new(input: ScaffoldPlannerPacketInput) -> Self {
        Self {
            record_kind: SCAFFOLD_PLANNER_RECORD_KIND.to_owned(),
            schema_version: SCAFFOLD_PLANNER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            planner_label: input.planner_label,
            rows: input.rows,
            safety_review: input.safety_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows plans whose parameters are unresolved, whose preflight failed,
    /// whose preview or rollback is unavailable, whose parity broke, or whose
    /// proof or upstream narrowed.
    ///
    /// Each blocking condition withdraws apply and records a labeled blocking
    /// state and trigger rather than hiding the plan. Unresolved parameters are
    /// the first block, then a failed preflight, then a missing write-impact
    /// preview or rollback boundary, then broken create-empty parity, then a
    /// stale proof or narrowed upstream. Plans without a matching observation are
    /// left unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[ScaffoldPlanRowObservation]) {
        for row in &mut self.rows {
            let Some(observation) = observations.iter().find(|obs| obs.plan_id == row.plan_id)
            else {
                continue;
            };

            if !observation.parameters_resolved {
                row.parameter_review.review_class = ParameterReviewClass::AwaitingRequiredInput;
                row.readiness_state = PlanReadinessState::BlockedAwaitingInput;
                row.admitted_for_apply = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ScaffoldPlannerDowngradeTrigger::RequiredParameterUnresolved,
                );
            }

            if !observation.environment_ready {
                row.environment_preflight.preflight_class =
                    EnvironmentPreflightClass::PreflightFailedBlocked;
                if !matches!(
                    row.readiness_state,
                    PlanReadinessState::BlockedAwaitingInput
                ) {
                    row.readiness_state = PlanReadinessState::BlockedFailedPreflight;
                }
                row.admitted_for_apply = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ScaffoldPlannerDowngradeTrigger::PreflightFailed,
                );
            }

            if !observation.write_impact_preview_available {
                row.readiness_state = PlanReadinessState::ReviewRequired;
                row.admitted_for_apply = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ScaffoldPlannerDowngradeTrigger::WriteImpactPreviewUnavailable,
                );
            }

            if !observation.rollback_boundary_available {
                row.write_impact_preview.rollback_posture =
                    PlanRollbackPostureClass::RollbackUnavailableReviewRequired;
                row.readiness_state = PlanReadinessState::ReviewRequired;
                row.admitted_for_apply = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ScaffoldPlannerDowngradeTrigger::RollbackBoundaryUnavailable,
                );
            }

            if row.is_create_empty() && !observation.create_empty_parity_intact {
                row.create_empty_parity.parity_class = CreateEmptyParityClass::ParityBrokenBlocked;
                row.readiness_state = PlanReadinessState::BlockedParityBroken;
                row.admitted_for_apply = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ScaffoldPlannerDowngradeTrigger::CreateEmptyParityBroken,
                );
            }

            if (!observation.proof_fresh || observation.upstream_narrowed) && row.admitted_for_apply
            {
                row.admitted_for_apply = false;
                let trigger = if observation.proof_fresh {
                    ScaffoldPlannerDowngradeTrigger::UpstreamDependencyNarrowed
                } else {
                    ScaffoldPlannerDowngradeTrigger::ProofStale
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
            }
        }
    }

    /// Validates the scaffold-planner invariants.
    pub fn validate(&self) -> Vec<ScaffoldPlannerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != SCAFFOLD_PLANNER_RECORD_KIND {
            violations.push(ScaffoldPlannerViolation::WrongRecordKind);
        }
        if self.schema_version != SCAFFOLD_PLANNER_SCHEMA_VERSION {
            violations.push(ScaffoldPlannerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.planner_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ScaffoldPlannerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_safety_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("scaffold-planner packet serializes"),
        ) {
            violations.push(ScaffoldPlannerViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("scaffold-planner packet serializes")
    }

    /// Plans currently admitted for apply.
    pub fn admitted_rows(&self) -> impl Iterator<Item = &ScaffoldPlanRow> {
        self.rows.iter().filter(|row| row.admitted_for_apply)
    }

    /// Deterministic Markdown summary for gallery, support, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let admitted = self.admitted_rows().count();
        let mut out = String::new();
        out.push_str("# Scaffold Planner, Parameter Review, Environment Preflights, and Create-Empty Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.planner_label));
        out.push_str(&format!(
            "- Plans: {} ({} admitted for apply)\n",
            self.rows.len(),
            admitted
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Plans\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** `{}`: {} ({})\n",
                row.plan_id,
                row.plan_revision_semver,
                row.plan_kind.as_str(),
                row.readiness_state.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Parameters: {} (resolved {}/{}, unresolved required {})\n",
                row.parameter_review.review_class.as_str(),
                row.parameter_review.resolved_parameters,
                row.parameter_review.total_parameters,
                row.parameter_review.unresolved_required_parameters
            ));
            out.push_str(&format!(
                "  - Preflight: {} (passed {}/{}, blocking {})\n",
                row.environment_preflight.preflight_class.as_str(),
                row.environment_preflight.passed_checks,
                row.environment_preflight.total_checks,
                row.environment_preflight.blocking_failures
            ));
            out.push_str(&format!(
                "  - Create-empty parity: {}\n",
                row.create_empty_parity.parity_class.as_str()
            ));
            out.push_str(&format!(
                "  - Write impact: +{} files, ~{} modified, +{} dirs (rollback: {}, admitted: {})\n",
                row.write_impact_preview.files_created,
                row.write_impact_preview.files_modified,
                row.write_impact_preview.directories_created,
                row.write_impact_preview.rollback_posture.as_str(),
                row.admitted_for_apply
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in scaffold-planner export.
#[derive(Debug)]
pub enum ScaffoldPlannerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ScaffoldPlannerViolation>),
}

impl fmt::Display for ScaffoldPlannerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "scaffold-planner export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "scaffold-planner export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ScaffoldPlannerArtifactError {}

/// Validation failures emitted by [`ScaffoldPlannerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScaffoldPlannerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The planner carries no plans.
    RowsEmpty,
    /// A plan is incomplete.
    RowIncomplete,
    /// A template-backed plan is missing its template id or manifest ref.
    TemplateProvenanceIncomplete,
    /// A parameter-review block has incoherent counts.
    ParameterCountsIncoherent,
    /// An environment-preflight block has incoherent counts.
    PreflightCountsIncoherent,
    /// A blocking parameter-review state is still admitted for apply.
    BlockingParameterAdmitted,
    /// A blocking preflight state is still admitted for apply.
    BlockingPreflightAdmitted,
    /// A non-ready plan is still admitted for apply.
    NonReadyAdmitted,
    /// A create-empty plan does not reach the required parity guarantees.
    CreateEmptyParityIncomplete,
    /// A plan would write before confirmation.
    WritesBeforeConfirmation,
    /// A plan is missing its write-impact preview ref.
    WriteImpactPreviewRefMissing,
    /// A plan has no preflight-check refs.
    PreflightCheckRefsMissing,
    /// A plan has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A plan has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Safety review does not satisfy required invariants.
    SafetyReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ScaffoldPlannerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsEmpty => "rows_empty",
            Self::RowIncomplete => "row_incomplete",
            Self::TemplateProvenanceIncomplete => "template_provenance_incomplete",
            Self::ParameterCountsIncoherent => "parameter_counts_incoherent",
            Self::PreflightCountsIncoherent => "preflight_counts_incoherent",
            Self::BlockingParameterAdmitted => "blocking_parameter_admitted",
            Self::BlockingPreflightAdmitted => "blocking_preflight_admitted",
            Self::NonReadyAdmitted => "non_ready_admitted",
            Self::CreateEmptyParityIncomplete => "create_empty_parity_incomplete",
            Self::WritesBeforeConfirmation => "writes_before_confirmation",
            Self::WriteImpactPreviewRefMissing => "write_impact_preview_ref_missing",
            Self::PreflightCheckRefsMissing => "preflight_check_refs_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::SafetyReviewIncomplete => "safety_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in scaffold-planner export.
///
/// This is the first real consumer of the scaffold planner: a gallery,
/// preflight, parameter-review, diagnostics, or support-export surface calls it
/// to ingest the canonical packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`ScaffoldPlannerArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_scaffold_planner_export(
) -> Result<ScaffoldPlannerPacket, ScaffoldPlannerArtifactError> {
    let packet: ScaffoldPlannerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/ship_the_scaffold_planner_parameter_review_environment_preflights_and_create_empty_parity/support_export.json"
    )))
    .map_err(ScaffoldPlannerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ScaffoldPlannerArtifactError::Validation(violations))
    }
}

/// Canonical safety review block with every invariant satisfied.
pub fn canonical_safety_review() -> ScaffoldPlannerSafetyReview {
    ScaffoldPlannerSafetyReview {
        no_writes_before_review: true,
        parameters_reviewed_before_apply: true,
        environment_preflighted_before_apply: true,
        write_impact_previewed_before_apply: true,
        rollback_boundary_recorded: true,
        create_empty_reaches_preflight_parity: true,
        create_empty_reaches_rollback_parity: true,
        blocking_preflight_blocks_apply: true,
        unresolved_parameters_block_apply: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
        no_credential_or_raw_payload_in_export: true,
    }
}

/// Canonical consumer projection block with every surface projecting plan truth.
pub fn canonical_consumer_projection() -> ScaffoldPlannerConsumerProjection {
    ScaffoldPlannerConsumerProjection {
        gallery_shows_plan_kind_and_readiness: true,
        preflight_shows_environment_checks: true,
        parameter_review_shows_resolution_state: true,
        run_surface_shows_write_impact_and_rollback: true,
        recovery_shows_rollback_boundary: true,
        cli_headless_shows_plan_rows: true,
        support_export_shows_plan_rows: true,
        diagnostics_shows_readiness_state: true,
        blocked_plans_labeled_not_hidden: true,
    }
}

/// Canonical source contract refs that every planner export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        SCAFFOLD_PLANNER_SCHEMA_REF.to_owned(),
        SCAFFOLD_PLANNER_DOC_REF.to_owned(),
        SCAFFOLD_RUN_CONTRACT_REF.to_owned(),
        TEMPLATE_MANIFEST_CONTRACT_REF.to_owned(),
        SCAFFOLD_HOOK_POLICY_CONTRACT_REF.to_owned(),
    ]
}

/// Builds the canonical scaffold planner from stable-plan truth.
///
/// The plans mirror the checked-in support export and cover the planner
/// spectrum: a templated starter ready for apply, a templated starter awaiting
/// required parameter input, a templated starter blocked on a failed environment
/// preflight, and a create-empty workspace at full parity with the templated
/// flow.
pub fn canonical_scaffold_planner(
    packet_id: String,
    planner_label: String,
    minted_at: String,
    proof_freshness: ScaffoldPlannerProofFreshness,
) -> ScaffoldPlannerPacket {
    ScaffoldPlannerPacket::new(ScaffoldPlannerPacketInput {
        packet_id,
        planner_label,
        rows: canonical_rows(),
        safety_review: canonical_safety_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical plans that match the checked-in support export.
pub fn canonical_rows() -> Vec<ScaffoldPlanRow> {
    use ScaffoldPlannerConsumerSurface as Surface;
    use ScaffoldPlannerDowngradeTrigger as Trigger;

    vec![
        ScaffoldPlanRow {
            plan_id: "scaffold-plan:rust.cli.ready:2026.04".to_owned(),
            plan_kind: ScaffoldPlanKind::TemplateScaffold,
            template_id: Some("template:first_party.rust.cli_tool:01".to_owned()),
            plan_revision_semver: "0.4.2".to_owned(),
            manifest_ref: Some("manifest:first_party.rust.cli_tool:0.4.2".to_owned()),
            scaffold_run_ref: "scaffold-run:rust.cli.ready:2026.04".to_owned(),
            scope_summary: "Officially-supported Rust CLI starter planned with every parameter reviewed, the toolchain preflighted, and the full write impact previewed before any write".to_owned(),
            parameter_review: ScaffoldParameterReview {
                review_class: ParameterReviewClass::AllResolvedReviewed,
                total_parameters: 4,
                resolved_parameters: 4,
                required_parameters: 2,
                unresolved_required_parameters: 0,
                invalid_parameters: 0,
                parameter_review_refs: vec![
                    "param-review:rust_cli:project_name:resolved".to_owned(),
                    "param-review:rust_cli:edition:default_accepted".to_owned(),
                ],
                no_mutation_during_review: true,
            },
            environment_preflight: ScaffoldEnvironmentPreflight {
                preflight_class: EnvironmentPreflightClass::AllPreflightsPassed,
                total_checks: 3,
                passed_checks: 3,
                warning_checks: 0,
                blocking_failures: 0,
                preflight_check_refs: vec![
                    "preflight:rust_cli:toolchain_present:2026.04.20".to_owned(),
                    "preflight:rust_cli:target_dir_writable:2026.04.20".to_owned(),
                ],
            },
            create_empty_parity: ScaffoldCreateEmptyParity {
                parity_class: CreateEmptyParityClass::FullParityWithTemplateFlow,
                shares_preflight_pipeline: true,
                shares_parameter_review: true,
                shares_rollback_boundary: true,
                parity_note_refs: vec![],
            },
            write_impact_preview: ScaffoldWriteImpactPreview {
                files_created: 9,
                files_modified: 0,
                directories_created: 3,
                no_writes_before_confirmation: true,
                write_impact_preview_ref: "write-impact:rust.cli.ready:2026.04".to_owned(),
                rollback_posture: PlanRollbackPostureClass::FullRollbackBoundaryRecorded,
            },
            readiness_state: PlanReadinessState::ReadyForApply,
            admitted_for_apply: true,
            known_blocker_refs: vec![],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::RequiredParameterUnresolved,
                Trigger::PreflightFailed,
                Trigger::TemplateRevisionUnavailable,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::Preflight,
                Surface::ParameterReview,
                Surface::SupportExport,
            ],
        },
        ScaffoldPlanRow {
            plan_id: "scaffold-plan:ts.web.awaiting_input:2026.04".to_owned(),
            plan_kind: ScaffoldPlanKind::TemplateScaffold,
            template_id: Some("template:official.ts.web_application:01".to_owned()),
            plan_revision_semver: "1.8.0".to_owned(),
            manifest_ref: Some("manifest:official.ts.web_application:1.8.0".to_owned()),
            scaffold_run_ref: "scaffold-run:ts.web.awaiting_input:2026.04".to_owned(),
            scope_summary: "Official TypeScript web app starter planned but awaiting a required project name and package scope; apply stays blocked and no write happens until the parameters are reviewed".to_owned(),
            parameter_review: ScaffoldParameterReview {
                review_class: ParameterReviewClass::AwaitingRequiredInput,
                total_parameters: 5,
                resolved_parameters: 3,
                required_parameters: 3,
                unresolved_required_parameters: 2,
                invalid_parameters: 0,
                parameter_review_refs: vec![
                    "param-review:ts_web:project_name:awaiting".to_owned(),
                    "param-review:ts_web:package_scope:awaiting".to_owned(),
                ],
                no_mutation_during_review: true,
            },
            environment_preflight: ScaffoldEnvironmentPreflight {
                preflight_class: EnvironmentPreflightClass::PassedWithWarnings,
                total_checks: 4,
                passed_checks: 3,
                warning_checks: 1,
                blocking_failures: 0,
                preflight_check_refs: vec![
                    "preflight:ts_web:node_present:2026.04.19".to_owned(),
                    "preflight:ts_web:package_manager_detected:warning:2026.04.19".to_owned(),
                ],
            },
            create_empty_parity: ScaffoldCreateEmptyParity {
                parity_class: CreateEmptyParityClass::FullParityWithTemplateFlow,
                shares_preflight_pipeline: true,
                shares_parameter_review: true,
                shares_rollback_boundary: true,
                parity_note_refs: vec![],
            },
            write_impact_preview: ScaffoldWriteImpactPreview {
                files_created: 0,
                files_modified: 0,
                directories_created: 0,
                no_writes_before_confirmation: true,
                write_impact_preview_ref: "write-impact:ts.web.awaiting_input:2026.04".to_owned(),
                rollback_posture: PlanRollbackPostureClass::NoWritesYetFullyReversible,
            },
            readiness_state: PlanReadinessState::BlockedAwaitingInput,
            admitted_for_apply: false,
            known_blocker_refs: vec![
                "known-blocker:ts_web:two_required_params_unresolved".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::RequiredParameterUnresolved,
                Trigger::ParameterValidationFailed,
                Trigger::UpstreamDependencyNarrowed,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::Preflight,
                Surface::ParameterReview,
                Surface::SupportExport,
            ],
        },
        ScaffoldPlanRow {
            plan_id: "scaffold-plan:python.data.preflight_blocked:2026.03".to_owned(),
            plan_kind: ScaffoldPlanKind::TemplateScaffold,
            template_id: Some("template:community.python.data_workbench:07".to_owned()),
            plan_revision_semver: "2.1.0".to_owned(),
            manifest_ref: Some("manifest:community.python.data_workbench:2.1.0".to_owned()),
            scaffold_run_ref: "scaffold-run:python.data.preflight_blocked:2026.03".to_owned(),
            scope_summary: "Community Python data workbench starter whose environment preflight found a missing required interpreter; the blocker is disclosed and apply stays blocked rather than failing mid-write".to_owned(),
            parameter_review: ScaffoldParameterReview {
                review_class: ParameterReviewClass::AllResolvedReviewed,
                total_parameters: 3,
                resolved_parameters: 3,
                required_parameters: 1,
                unresolved_required_parameters: 0,
                invalid_parameters: 0,
                parameter_review_refs: vec![
                    "param-review:python_data:project_name:resolved".to_owned(),
                ],
                no_mutation_during_review: true,
            },
            environment_preflight: ScaffoldEnvironmentPreflight {
                preflight_class: EnvironmentPreflightClass::BlockingPrerequisiteMissing,
                total_checks: 4,
                passed_checks: 2,
                warning_checks: 1,
                blocking_failures: 1,
                preflight_check_refs: vec![
                    "preflight:python_data:interpreter_missing:blocking:2026.03.30".to_owned(),
                    "preflight:python_data:venv_writable:2026.03.30".to_owned(),
                ],
            },
            create_empty_parity: ScaffoldCreateEmptyParity {
                parity_class: CreateEmptyParityClass::FullParityWithTemplateFlow,
                shares_preflight_pipeline: true,
                shares_parameter_review: true,
                shares_rollback_boundary: true,
                parity_note_refs: vec![],
            },
            write_impact_preview: ScaffoldWriteImpactPreview {
                files_created: 0,
                files_modified: 0,
                directories_created: 0,
                no_writes_before_confirmation: true,
                write_impact_preview_ref: "write-impact:python.data.preflight_blocked:2026.03"
                    .to_owned(),
                rollback_posture: PlanRollbackPostureClass::NoWritesYetFullyReversible,
            },
            readiness_state: PlanReadinessState::BlockedFailedPreflight,
            admitted_for_apply: false,
            known_blocker_refs: vec![
                "known-blocker:python_data:interpreter_prerequisite_missing".to_owned(),
            ],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EnvironmentPrerequisiteMissing,
                Trigger::PreflightFailed,
                Trigger::UpstreamDependencyNarrowed,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::Preflight,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        ScaffoldPlanRow {
            plan_id: "scaffold-plan:create_empty.workspace:2026.05".to_owned(),
            plan_kind: ScaffoldPlanKind::CreateEmptyWorkspace,
            template_id: None,
            plan_revision_semver: "1.0.0".to_owned(),
            manifest_ref: None,
            scaffold_run_ref: "scaffold-run:create_empty.workspace:2026.05".to_owned(),
            scope_summary: "Create-empty workspace plan that reaches full parity with the templated flow: it runs the same environment preflight, parameter review for workspace name, write-impact preview, and rollback boundary before any write".to_owned(),
            parameter_review: ScaffoldParameterReview {
                review_class: ParameterReviewClass::AllResolvedReviewed,
                total_parameters: 1,
                resolved_parameters: 1,
                required_parameters: 1,
                unresolved_required_parameters: 0,
                invalid_parameters: 0,
                parameter_review_refs: vec![
                    "param-review:create_empty:workspace_name:resolved".to_owned(),
                ],
                no_mutation_during_review: true,
            },
            environment_preflight: ScaffoldEnvironmentPreflight {
                preflight_class: EnvironmentPreflightClass::AllPreflightsPassed,
                total_checks: 2,
                passed_checks: 2,
                warning_checks: 0,
                blocking_failures: 0,
                preflight_check_refs: vec![
                    "preflight:create_empty:target_dir_writable:2026.05.02".to_owned(),
                    "preflight:create_empty:no_collision:2026.05.02".to_owned(),
                ],
            },
            create_empty_parity: ScaffoldCreateEmptyParity {
                parity_class: CreateEmptyParityClass::FullParityWithTemplateFlow,
                shares_preflight_pipeline: true,
                shares_parameter_review: true,
                shares_rollback_boundary: true,
                parity_note_refs: vec![
                    "parity-note:create_empty:shares_full_safety_pipeline".to_owned(),
                ],
            },
            write_impact_preview: ScaffoldWriteImpactPreview {
                files_created: 3,
                files_modified: 0,
                directories_created: 2,
                no_writes_before_confirmation: true,
                write_impact_preview_ref: "write-impact:create_empty.workspace:2026.05".to_owned(),
                rollback_posture: PlanRollbackPostureClass::FullRollbackBoundaryRecorded,
            },
            readiness_state: PlanReadinessState::ReadyForApply,
            admitted_for_apply: true,
            known_blocker_refs: vec![],
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::PreflightFailed,
                Trigger::CreateEmptyParityBroken,
                Trigger::RollbackBoundaryUnavailable,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::Preflight,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
    ]
}

fn validate_source_contracts(
    packet: &ScaffoldPlannerPacket,
    violations: &mut Vec<ScaffoldPlannerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        SCAFFOLD_PLANNER_SCHEMA_REF,
        SCAFFOLD_PLANNER_DOC_REF,
        SCAFFOLD_RUN_CONTRACT_REF,
        TEMPLATE_MANIFEST_CONTRACT_REF,
        SCAFFOLD_HOOK_POLICY_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ScaffoldPlannerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(packet: &ScaffoldPlannerPacket, violations: &mut Vec<ScaffoldPlannerViolation>) {
    if packet.rows.is_empty() {
        violations.push(ScaffoldPlannerViolation::RowsEmpty);
        return;
    }

    for row in &packet.rows {
        if row.plan_id.trim().is_empty()
            || row.plan_revision_semver.trim().is_empty()
            || row.scaffold_run_ref.trim().is_empty()
            || row.scope_summary.trim().is_empty()
        {
            violations.push(ScaffoldPlannerViolation::RowIncomplete);
        }
        if row
            .write_impact_preview
            .write_impact_preview_ref
            .trim()
            .is_empty()
        {
            violations.push(ScaffoldPlannerViolation::WriteImpactPreviewRefMissing);
        }
        if row.environment_preflight.preflight_check_refs.is_empty() {
            violations.push(ScaffoldPlannerViolation::PreflightCheckRefsMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(ScaffoldPlannerViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(ScaffoldPlannerViolation::ConsumerSurfacesMissing);
        }

        validate_row_provenance(row, violations);
        validate_row_counts(row, violations);
        validate_row_admission(row, violations);
        validate_row_parity(row, violations);

        if !row.write_impact_preview.no_writes_before_confirmation {
            violations.push(ScaffoldPlannerViolation::WritesBeforeConfirmation);
        }
    }
}

fn validate_row_provenance(row: &ScaffoldPlanRow, violations: &mut Vec<ScaffoldPlannerViolation>) {
    // Template-backed plans cite both a template id and a manifest ref.
    if row.plan_kind.is_template()
        && (row
            .template_id
            .as_deref()
            .map(str::trim)
            .map_or(true, str::is_empty)
            || row
                .manifest_ref
                .as_deref()
                .map(str::trim)
                .map_or(true, str::is_empty))
    {
        violations.push(ScaffoldPlannerViolation::TemplateProvenanceIncomplete);
    }
}

fn validate_row_counts(row: &ScaffoldPlanRow, violations: &mut Vec<ScaffoldPlannerViolation>) {
    let params = &row.parameter_review;
    if params.resolved_parameters > params.total_parameters
        || params.required_parameters > params.total_parameters
        || params.unresolved_required_parameters > params.required_parameters
        || params.invalid_parameters > params.total_parameters
    {
        violations.push(ScaffoldPlannerViolation::ParameterCountsIncoherent);
    }

    let preflight = &row.environment_preflight;
    if preflight.passed_checks + preflight.warning_checks + preflight.blocking_failures
        > preflight.total_checks
    {
        violations.push(ScaffoldPlannerViolation::PreflightCountsIncoherent);
    }
}

fn validate_row_admission(row: &ScaffoldPlanRow, violations: &mut Vec<ScaffoldPlannerViolation>) {
    if !row.admitted_for_apply {
        return;
    }
    if row.parameter_review.review_class.blocks_apply() {
        violations.push(ScaffoldPlannerViolation::BlockingParameterAdmitted);
    }
    if row.environment_preflight.preflight_class.blocks_apply() {
        violations.push(ScaffoldPlannerViolation::BlockingPreflightAdmitted);
    }
    if !row.readiness_state.admits_apply()
        || row.create_empty_parity.parity_class.blocks_apply()
        || row.write_impact_preview.rollback_posture.blocks_apply()
    {
        violations.push(ScaffoldPlannerViolation::NonReadyAdmitted);
    }
}

fn validate_row_parity(row: &ScaffoldPlanRow, violations: &mut Vec<ScaffoldPlannerViolation>) {
    // A create-empty plan that claims full parity must actually share the
    // templated preflight pipeline and rollback boundary; parity is never a
    // label asserted without the shared safety guarantees behind it.
    if row.is_create_empty()
        && row.create_empty_parity.parity_class.is_full_parity()
        && (!row.create_empty_parity.shares_preflight_pipeline
            || !row.create_empty_parity.shares_rollback_boundary)
    {
        violations.push(ScaffoldPlannerViolation::CreateEmptyParityIncomplete);
    }
}

fn validate_safety_review(
    packet: &ScaffoldPlannerPacket,
    violations: &mut Vec<ScaffoldPlannerViolation>,
) {
    let review = &packet.safety_review;
    for ok in [
        review.no_writes_before_review,
        review.parameters_reviewed_before_apply,
        review.environment_preflighted_before_apply,
        review.write_impact_previewed_before_apply,
        review.rollback_boundary_recorded,
        review.create_empty_reaches_preflight_parity,
        review.create_empty_reaches_rollback_parity,
        review.blocking_preflight_blocks_apply,
        review.unresolved_parameters_block_apply,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
        review.no_credential_or_raw_payload_in_export,
    ] {
        if !ok {
            violations.push(ScaffoldPlannerViolation::SafetyReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &ScaffoldPlannerPacket,
    violations: &mut Vec<ScaffoldPlannerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.gallery_shows_plan_kind_and_readiness,
        projection.preflight_shows_environment_checks,
        projection.parameter_review_shows_resolution_state,
        projection.run_surface_shows_write_impact_and_rollback,
        projection.recovery_shows_rollback_boundary,
        projection.cli_headless_shows_plan_rows,
        projection.support_export_shows_plan_rows,
        projection.diagnostics_shows_readiness_state,
        projection.blocked_plans_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(ScaffoldPlannerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &ScaffoldPlannerPacket,
    violations: &mut Vec<ScaffoldPlannerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ScaffoldPlannerViolation::ProofFreshnessIncomplete);
    }
}

fn push_unique_trigger(
    triggers: &mut Vec<ScaffoldPlannerDowngradeTrigger>,
    trigger: ScaffoldPlannerDowngradeTrigger,
) {
    if !triggers.contains(&trigger) {
        triggers.push(trigger);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
