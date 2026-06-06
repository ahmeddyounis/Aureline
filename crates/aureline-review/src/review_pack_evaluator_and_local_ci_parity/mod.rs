//! Stable review-pack evaluator and replay packet contract.
//!
//! This module owns the stable result model that binds repo-defined review
//! packs to local review, CI, AI review, hosted provider overlays, browser
//! companion follow-up, and headless replay. The evaluator input is
//! declarative and arbitrary-code-free: it records identities, scope
//! selectors, required-check vocabulary, ownership signals, per-surface
//! observations, AI finding rows, publish previews, and support export
//! posture. Validation rejects or downgrades green claims when pack digest,
//! base/head identity, stale scope, missing capability, AI freshness, or
//! outbound publish truth is incomplete.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version for every stable review-pack evaluator packet.
pub const REVIEW_PACK_EVALUATOR_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ReviewPackStableEvaluationPacket`].
pub const REVIEW_PACK_EVALUATOR_PACKET_RECORD_KIND: &str = "review_pack_stable_evaluation_packet";

/// Stable record-kind tag for [`ReviewPackStableEvaluationRecord`].
pub const REVIEW_PACK_EVALUATION_RECORD_KIND: &str = "review_pack_stable_evaluation_record";

/// Stable record-kind tag for [`ReviewPackReplayExportPacket`].
pub const REVIEW_PACK_REPLAY_EXPORT_PACKET_RECORD_KIND: &str = "review_pack_replay_export_packet";

/// Stable record-kind tag for [`ReviewPackEvaluationInspectionRecord`].
pub const REVIEW_PACK_EVALUATION_INSPECTION_RECORD_KIND: &str =
    "review_pack_evaluation_inspection_record";

/// Closed set of surfaces that must state the active review-pack truth.
pub const REVIEW_PACK_EVALUATOR_SURFACE_CLASSES: &[&str] = &[
    "local_review_workspace",
    "local_ci_run",
    "hosted_provider_overlay",
    "ai_review",
    "browser_companion_follow_up",
    "cli_headless_support",
];

/// Closed set of surface state classes.
pub const REVIEW_PACK_EVALUATOR_SURFACE_STATE_CLASSES: &[&str] = &[
    "surface_current_full",
    "surface_partial_scope",
    "surface_stale_pack",
    "surface_digest_mismatch",
    "surface_capability_unsupported",
    "surface_provider_unavailable",
    "surface_not_evaluated_here",
];

/// Closed set of scope selector classes.
pub const REVIEW_PACK_EVALUATOR_SCOPE_SELECTOR_CLASSES: &[&str] = &[
    "path_selector",
    "language_selector",
    "service_selector",
    "risk_selector",
    "workset_selector",
];

/// Closed set of scope truth classes.
pub const REVIEW_PACK_EVALUATOR_SCOPE_TRUTH_CLASSES: &[&str] =
    &["full_scope", "partial_scope", "slice_omitted", "stale_pack"];

/// Closed set of required check classes.
pub const REVIEW_PACK_EVALUATOR_REQUIRED_CHECK_CLASSES: &[&str] = &[
    "format_check",
    "lint_check",
    "type_check",
    "unit_test",
    "security_scan",
    "docs_review",
    "migration_review",
    "ai_review_check",
    "browser_follow_up_check",
];

/// Closed set of required check enforcement classes.
pub const REVIEW_PACK_EVALUATOR_CHECK_ENFORCEMENT_CLASSES: &[&str] = &[
    "enforced_blocking",
    "enforced_blocking_unless_waived",
    "advisory_non_blocking",
    "provider_authoritative",
];

/// Closed set of per-surface check result classes.
pub const REVIEW_PACK_EVALUATOR_CHECK_RESULT_CLASSES: &[&str] = &[
    "passed",
    "failed_blocking",
    "failed_advisory",
    "not_evaluated_here",
    "ci_only",
    "provider_unavailable",
    "policy_downgraded",
    "experimental_analyzer",
    "stale_result",
];

/// Closed set of ownership signal classes.
pub const REVIEW_PACK_EVALUATOR_OWNERSHIP_SIGNAL_CLASSES: &[&str] = &[
    "advisory_owner",
    "enforced_owner",
    "provider_authoritative_owner",
];

/// Closed set of ownership signal source classes.
pub const REVIEW_PACK_EVALUATOR_OWNERSHIP_SOURCE_CLASSES: &[&str] = &[
    "graph_ownership",
    "codeowners",
    "provider_branch_rule",
    "release_policy_bundle",
    "manual_reviewer_suggestion",
];

/// Closed set of AI review finding resolution states.
pub const REVIEW_PACK_EVALUATOR_AI_FINDING_STATES: &[&str] = &[
    "open",
    "dismissed",
    "published",
    "outdated",
    "suppressed",
    "rerun_recommended",
];

/// Closed set of AI finding diff freshness classes.
pub const REVIEW_PACK_EVALUATOR_AI_DIFF_FRESHNESS_CLASSES: &[&str] = &[
    "diff_unchanged",
    "diff_changed_materially",
    "diff_freshness_unknown",
];

/// Closed set of publish preview write-access classes.
pub const REVIEW_PACK_EVALUATOR_PROVIDER_WRITE_ACCESS_CLASSES: &[&str] = &[
    "provider_write_available",
    "missing_provider_write_access",
    "provider_write_blocked_by_policy",
];

/// Closed set of publish preview outbound intent classes.
pub const REVIEW_PACK_EVALUATOR_OUTBOUND_INTENT_CLASSES: &[&str] =
    &["publish_to_review", "copy_only", "export_only", "blocked"];

/// Closed set of capability support classes.
pub const REVIEW_PACK_EVALUATOR_CAPABILITY_CLASSES: &[&str] = &[
    "evaluator_supported",
    "unsupported_evaluator_capability",
    "capability_policy_downgraded",
];

/// Closed set of overall stable review-pack verdict classes.
pub const REVIEW_PACK_EVALUATOR_VERDICT_CLASSES: &[&str] = &[
    "full_parity",
    "degraded_requires_review",
    "stale_pack_invalidated",
    "unsupported_capability_downgraded",
];

/// Closed set of divergence labels shared by review surfaces and replay packets.
pub const REVIEW_PACK_EVALUATOR_DIVERGENCE_LABELS: &[&str] = &[
    "no_divergence",
    "partial_scope",
    "slice_omitted",
    "stale_pack",
    "not_evaluated_here",
    "ci_only",
    "provider_unavailable",
    "advisory_owner",
    "enforced_owner",
    "provider_authoritative",
    "policy_downgraded",
    "experimental_analyzer",
    "review_pack_digest_mismatch",
    "unsupported_evaluator_capability",
    "local_ci_disagree",
    "ai_review_outdated",
    "browser_companion_partial",
    "missing_provider_write_access",
];

/// Input used to materialize a stable review-pack evaluation packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackStableEvaluationInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Stable evaluation identity.
    pub evaluation_id: String,
    /// Stable replay packet identity derived from the same evaluation truth.
    pub replay_packet_id: String,
    /// Timestamp used by deterministic fixtures and support export.
    pub generated_at: String,
    /// Stable review pack identity.
    pub review_pack_id: String,
    /// Human-readable review pack version.
    pub review_pack_version: String,
    /// Digest of the normalized review-pack object.
    pub review_pack_digest_ref: String,
    /// Base commit or equivalent base identity.
    pub base_identity_ref: String,
    /// Head commit or equivalent head identity.
    pub head_identity_ref: String,
    /// Repo/worktree scope anchor for the evaluation.
    pub repo_anchor_ref: String,
    /// Capability class for the evaluator that produced the result.
    pub evaluator_capability_class: String,
    /// Overall verdict for the claimed stable review lane.
    pub overall_verdict_class: String,
    /// Divergence labels visible on review surfaces and exported replay packets.
    pub divergence_labels: Vec<String>,
    /// Surface observations for local, CI, provider, AI, browser, and headless lanes.
    pub surface_observations: Vec<ReviewPackSurfaceObservation>,
    /// Scope selectors applied by the pack.
    pub scope_selectors: Vec<ReviewScopeSelectorRecord>,
    /// Required checks and their local/CI/provider/AI/browser results.
    pub required_checks: Vec<ReviewPackRequiredCheckResult>,
    /// Advisory and enforced ownership signals.
    pub ownership_signals: Vec<ReviewPackOwnershipSignalRecord>,
    /// AI review findings retained in the same replayable truth model.
    pub ai_findings: Vec<ReviewPackAiFindingRecord>,
    /// Publish-to-review previews and fallback/copy/export intent.
    pub publish_previews: Vec<ReviewPackPublishPreviewRecord>,
    /// Support export posture.
    pub support_export: ReviewPackEvaluationSupportExport,
    /// Consumer surfaces wired to this packet family.
    pub consumer_surfaces: Vec<String>,
}

/// One surface observation pinning result origin to pack/base/head identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackSurfaceObservation {
    /// Surface class from [`REVIEW_PACK_EVALUATOR_SURFACE_CLASSES`].
    pub surface_class: String,
    /// Surface state class from [`REVIEW_PACK_EVALUATOR_SURFACE_STATE_CLASSES`].
    pub surface_state_class: String,
    /// Review pack digest observed by this surface.
    pub review_pack_digest_ref: String,
    /// Base identity observed by this surface.
    pub base_identity_ref: String,
    /// Head identity observed by this surface.
    pub head_identity_ref: String,
    /// Divergence label for this surface.
    pub divergence_label: String,
    /// Short reviewable sentence explaining the observation.
    pub summary: String,
}

/// One review scope selector applied by the stable review pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewScopeSelectorRecord {
    /// Stable selector identity.
    pub selector_id: String,
    /// Selector class from [`REVIEW_PACK_EVALUATOR_SCOPE_SELECTOR_CLASSES`].
    pub selector_class: String,
    /// Opaque selector reference safe for support export.
    pub selector_ref: String,
    /// Scope truth class from [`REVIEW_PACK_EVALUATOR_SCOPE_TRUTH_CLASSES`].
    pub scope_truth_class: String,
    /// Divergence label required when the selector is partial or stale.
    pub divergence_label: String,
    /// Short reviewable sentence explaining the selector.
    pub summary: String,
}

/// One required-check result normalized across local, CI, AI, provider, and browser lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackRequiredCheckResult {
    /// Stable check identity.
    pub check_id: String,
    /// Stable required-check vocabulary name.
    pub required_check_name: String,
    /// Required check class from [`REVIEW_PACK_EVALUATOR_REQUIRED_CHECK_CLASSES`].
    pub required_check_class: String,
    /// Enforcement class from [`REVIEW_PACK_EVALUATOR_CHECK_ENFORCEMENT_CLASSES`].
    pub enforcement_class: String,
    /// Local review workspace result class.
    pub local_result_class: String,
    /// CI result class.
    pub ci_result_class: String,
    /// AI review result class.
    pub ai_review_result_class: String,
    /// Hosted provider overlay result class.
    pub provider_result_class: String,
    /// Browser companion result class.
    pub browser_result_class: String,
    /// Divergence label for the normalized check result.
    pub divergence_label: String,
    /// Short reviewable sentence explaining the check result.
    pub summary: String,
}

/// One ownership signal preserving advisory and enforced classes separately.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackOwnershipSignalRecord {
    /// Stable ownership signal identity.
    pub signal_id: String,
    /// Referenced scope selector identity.
    pub scope_selector_ref: String,
    /// Opaque owner identity safe for support export.
    pub owner_ref: String,
    /// Ownership signal class from [`REVIEW_PACK_EVALUATOR_OWNERSHIP_SIGNAL_CLASSES`].
    pub ownership_signal_class: String,
    /// Ownership source class from [`REVIEW_PACK_EVALUATOR_OWNERSHIP_SOURCE_CLASSES`].
    pub source_class: String,
    /// Divergence label for the ownership signal.
    pub divergence_label: String,
    /// Short reviewable sentence explaining the ownership signal.
    pub summary: String,
}

/// One AI review finding row retained with review-pack replay truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackAiFindingRecord {
    /// Stable finding identity.
    pub finding_id: String,
    /// Referenced scope selector identity.
    pub scope_selector_ref: String,
    /// Referenced required check identity.
    pub check_ref: String,
    /// Concise finding title.
    pub title: String,
    /// Severity class chosen by the review pack policy.
    pub severity_class: String,
    /// Confidence class chosen by the review pack policy.
    pub confidence_class: String,
    /// Evidence refs safe for export.
    pub evidence_refs: Vec<String>,
    /// Resolution state from [`REVIEW_PACK_EVALUATOR_AI_FINDING_STATES`].
    pub resolution_state: String,
    /// Diff freshness class from [`REVIEW_PACK_EVALUATOR_AI_DIFF_FRESHNESS_CLASSES`].
    pub diff_freshness_class: String,
    /// Optional publish preview linked to this finding.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_preview_ref: Option<String>,
    /// Divergence label for stale or policy-downgraded findings.
    pub divergence_label: String,
    /// Short reviewable sentence explaining the finding state.
    pub summary: String,
}

/// One publish-to-review preview preserving outbound intent without hidden writes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackPublishPreviewRecord {
    /// Stable publish preview identity.
    pub preview_id: String,
    /// Opaque destination reference for the hosted review or local fallback.
    pub destination_ref: String,
    /// Provider write-access class from [`REVIEW_PACK_EVALUATOR_PROVIDER_WRITE_ACCESS_CLASSES`].
    pub provider_write_access_class: String,
    /// Outbound intent class from [`REVIEW_PACK_EVALUATOR_OUTBOUND_INTENT_CLASSES`].
    pub outbound_intent_class: String,
    /// Whether local copy/export fallback is available and visible.
    pub local_copy_export_fallback_available: bool,
    /// Redaction-safe outbound payload preview.
    pub outbound_payload_preview: String,
    /// Divergence label for missing provider write access or policy blocks.
    pub divergence_label: String,
    /// Short reviewable sentence explaining the publish preview.
    pub summary: String,
}

/// Support export posture for stable review-pack evaluations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackEvaluationSupportExport {
    /// Export packet refs safe for support and audit workflows.
    pub export_packet_refs: Vec<String>,
    /// Whether raw paths may cross the support boundary.
    pub raw_path_export_allowed: bool,
    /// Whether raw glob bodies may cross the support boundary.
    pub raw_glob_body_export_allowed: bool,
    /// Whether raw command lines may cross the support boundary.
    pub raw_command_export_allowed: bool,
    /// Whether raw check output may cross the support boundary.
    pub raw_check_output_export_allowed: bool,
    /// Redaction class for the support export.
    pub redaction_class: String,
}

/// Stable review-pack evaluation packet consumed by review surfaces and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackStableEvaluationPacket {
    /// Stable packet record kind.
    pub record_kind: String,
    /// Stable packet schema version.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Normalized stable review-pack evaluation.
    pub evaluation: ReviewPackStableEvaluationRecord,
    /// Replay packet exportable to headless and support workflows.
    pub replay_packet: ReviewPackReplayExportPacket,
    /// Boolean inspection projection for dashboards and tests.
    pub inspection: ReviewPackEvaluationInspectionRecord,
}

/// Normalized stable review-pack evaluation result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackStableEvaluationRecord {
    /// Stable evaluation record kind.
    pub record_kind: String,
    /// Stable evaluation schema version.
    pub schema_version: u32,
    /// Stable evaluation identity.
    pub evaluation_id: String,
    /// Timestamp used by deterministic fixtures and support export.
    pub generated_at: String,
    /// Stable review pack identity.
    pub review_pack_id: String,
    /// Human-readable review pack version.
    pub review_pack_version: String,
    /// Digest of the normalized review-pack object.
    pub review_pack_digest_ref: String,
    /// Base commit or equivalent base identity.
    pub base_identity_ref: String,
    /// Head commit or equivalent head identity.
    pub head_identity_ref: String,
    /// Repo/worktree scope anchor for the evaluation.
    pub repo_anchor_ref: String,
    /// Capability class for the evaluator that produced the result.
    pub evaluator_capability_class: String,
    /// Overall verdict for the claimed stable review lane.
    pub overall_verdict_class: String,
    /// Divergence labels visible on review surfaces and exported replay packets.
    pub divergence_labels: Vec<String>,
    /// Surface observations for local, CI, provider, AI, browser, and headless lanes.
    pub surface_observations: Vec<ReviewPackSurfaceObservation>,
    /// Scope selectors applied by the pack.
    pub scope_selectors: Vec<ReviewScopeSelectorRecord>,
    /// Required checks and their local/CI/provider/AI/browser results.
    pub required_checks: Vec<ReviewPackRequiredCheckResult>,
    /// Advisory and enforced ownership signals.
    pub ownership_signals: Vec<ReviewPackOwnershipSignalRecord>,
    /// AI review findings retained in the same replayable truth model.
    pub ai_findings: Vec<ReviewPackAiFindingRecord>,
    /// Publish-to-review previews and fallback/copy/export intent.
    pub publish_previews: Vec<ReviewPackPublishPreviewRecord>,
    /// Support export posture.
    pub support_export: ReviewPackEvaluationSupportExport,
    /// Consumer surfaces wired to this packet family.
    pub consumer_surfaces: Vec<String>,
}

/// Replay packet that reopens the same stable review-pack result outside the GUI.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackReplayExportPacket {
    /// Stable replay packet record kind.
    pub record_kind: String,
    /// Stable replay schema version.
    pub schema_version: u32,
    /// Stable replay packet identity.
    pub replay_packet_id: String,
    /// Evaluation identity this replay packet reopens.
    pub evaluation_ref: String,
    /// Review pack version preserved by replay.
    pub review_pack_version: String,
    /// Review pack digest preserved by replay.
    pub review_pack_digest_ref: String,
    /// Base identity preserved by replay.
    pub base_identity_ref: String,
    /// Head identity preserved by replay.
    pub head_identity_ref: String,
    /// Required-check vocabulary preserved by replay.
    pub required_check_names: Vec<String>,
    /// Ownership signal classes preserved by replay.
    pub ownership_signal_classes: Vec<String>,
    /// Divergence labels preserved by replay.
    pub divergence_labels: Vec<String>,
    /// AI finding refs preserved by replay.
    pub ai_finding_refs: Vec<String>,
    /// Whether headless replay is supported for this packet.
    pub headless_replay_supported: bool,
    /// Support export refs used to reopen the packet.
    pub export_packet_refs: Vec<String>,
    /// Redaction class for replay.
    pub redaction_class: String,
}

/// Boolean projection used by claimed stable rows and fixture tests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPackEvaluationInspectionRecord {
    /// Stable inspection record kind.
    pub record_kind: String,
    /// Stable inspection schema version.
    pub schema_version: u32,
    /// Whether every required surface carries matching pack/base/head identity.
    pub all_surfaces_bound_to_same_identity: bool,
    /// Whether any surface is partial, stale, mismatched, unsupported, or unavailable.
    pub any_surface_downgraded: bool,
    /// Whether every required check name is preserved by the replay packet.
    pub required_check_vocabulary_replayable: bool,
    /// Whether advisory ownership signals are present.
    pub advisory_ownership_present: bool,
    /// Whether enforced ownership signals are present.
    pub enforced_ownership_present: bool,
    /// Whether stale AI findings were downgraded to outdated or rerun-recommended.
    pub stale_ai_findings_downgraded: bool,
    /// Whether missing provider write access keeps local copy/export fallback visible.
    pub missing_provider_write_access_falls_back: bool,
    /// Whether the replay packet can reopen the stable result headlessly.
    pub replay_packet_exportable: bool,
    /// Whether the packet is allowed to claim full parity.
    pub stable_full_parity_claim_allowed: bool,
}

impl ReviewPackStableEvaluationPacket {
    /// Builds a stable evaluation packet from declarative input.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewPackEvaluatorValidationError`] when required review-pack
    /// truth is missing, mismatched, or overclaimed.
    pub fn from_input(
        input: ReviewPackStableEvaluationInput,
    ) -> Result<Self, ReviewPackEvaluatorValidationError> {
        if input.packet_id.trim().is_empty() {
            return Err(ReviewPackEvaluatorValidationError::new(
                "packet_id must be a non-empty string",
            ));
        }
        let packet_id = input.packet_id;
        let evaluation = ReviewPackStableEvaluationRecord {
            record_kind: REVIEW_PACK_EVALUATION_RECORD_KIND.to_string(),
            schema_version: REVIEW_PACK_EVALUATOR_SCHEMA_VERSION,
            evaluation_id: input.evaluation_id,
            generated_at: input.generated_at,
            review_pack_id: input.review_pack_id,
            review_pack_version: input.review_pack_version,
            review_pack_digest_ref: input.review_pack_digest_ref,
            base_identity_ref: input.base_identity_ref,
            head_identity_ref: input.head_identity_ref,
            repo_anchor_ref: input.repo_anchor_ref,
            evaluator_capability_class: input.evaluator_capability_class,
            overall_verdict_class: input.overall_verdict_class,
            divergence_labels: input.divergence_labels,
            surface_observations: input.surface_observations,
            scope_selectors: input.scope_selectors,
            required_checks: input.required_checks,
            ownership_signals: input.ownership_signals,
            ai_findings: input.ai_findings,
            publish_previews: input.publish_previews,
            support_export: input.support_export,
            consumer_surfaces: input.consumer_surfaces,
        };
        validate_evaluation(&evaluation)?;
        let replay_packet = replay_packet_from_evaluation(input.replay_packet_id, &evaluation);
        validate_replay_packet(&evaluation, &replay_packet)?;
        let inspection = inspection_from_evaluation(&evaluation, &replay_packet);
        Ok(Self {
            record_kind: REVIEW_PACK_EVALUATOR_PACKET_RECORD_KIND.to_string(),
            schema_version: REVIEW_PACK_EVALUATOR_SCHEMA_VERSION,
            packet_id,
            evaluation,
            replay_packet,
            inspection,
        })
    }
}

/// Validation failure for stable review-pack evaluator packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewPackEvaluatorValidationError {
    message: String,
}

impl ReviewPackEvaluatorValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }

    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for ReviewPackEvaluatorValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "review-pack evaluator validation error: {}",
            self.message
        )
    }
}

impl std::error::Error for ReviewPackEvaluatorValidationError {}

/// Error returned when a stable review-pack JSON payload cannot be projected.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReviewPackEvaluatorError {
    /// JSON deserialization failed before validation.
    Json(String),
    /// Parsed JSON failed the stable evaluator contract.
    Validation(ReviewPackEvaluatorValidationError),
}

impl ReviewPackEvaluatorError {
    /// Returns a displayable error message.
    pub fn message(&self) -> &str {
        match self {
            Self::Json(message) => message,
            Self::Validation(error) => error.message(),
        }
    }
}

impl fmt::Display for ReviewPackEvaluatorError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(message) => write!(formatter, "review-pack evaluator JSON error: {message}"),
            Self::Validation(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for ReviewPackEvaluatorError {}

/// Parses and validates a stable review-pack evaluator JSON input.
///
/// # Errors
///
/// Returns [`ReviewPackEvaluatorError::Json`] when the payload is not valid JSON
/// input and [`ReviewPackEvaluatorError::Validation`] when stable review-pack
/// invariants are violated.
pub fn project_review_pack_stable_evaluation(
    payload: &str,
) -> Result<ReviewPackStableEvaluationPacket, ReviewPackEvaluatorError> {
    let input: ReviewPackStableEvaluationInput = serde_json::from_str(payload)
        .map_err(|err| ReviewPackEvaluatorError::Json(err.to_string()))?;
    ReviewPackStableEvaluationPacket::from_input(input)
        .map_err(ReviewPackEvaluatorError::Validation)
}

fn replay_packet_from_evaluation(
    replay_packet_id: String,
    evaluation: &ReviewPackStableEvaluationRecord,
) -> ReviewPackReplayExportPacket {
    let mut ownership_signal_classes: Vec<String> = evaluation
        .ownership_signals
        .iter()
        .map(|signal| signal.ownership_signal_class.clone())
        .collect();
    ownership_signal_classes.sort();
    ownership_signal_classes.dedup();

    ReviewPackReplayExportPacket {
        record_kind: REVIEW_PACK_REPLAY_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: REVIEW_PACK_EVALUATOR_SCHEMA_VERSION,
        replay_packet_id,
        evaluation_ref: evaluation.evaluation_id.clone(),
        review_pack_version: evaluation.review_pack_version.clone(),
        review_pack_digest_ref: evaluation.review_pack_digest_ref.clone(),
        base_identity_ref: evaluation.base_identity_ref.clone(),
        head_identity_ref: evaluation.head_identity_ref.clone(),
        required_check_names: evaluation
            .required_checks
            .iter()
            .map(|check| check.required_check_name.clone())
            .collect(),
        ownership_signal_classes,
        divergence_labels: evaluation.divergence_labels.clone(),
        ai_finding_refs: evaluation
            .ai_findings
            .iter()
            .map(|finding| finding.finding_id.clone())
            .collect(),
        headless_replay_supported: true,
        export_packet_refs: evaluation.support_export.export_packet_refs.clone(),
        redaction_class: evaluation.support_export.redaction_class.clone(),
    }
}

fn inspection_from_evaluation(
    evaluation: &ReviewPackStableEvaluationRecord,
    replay_packet: &ReviewPackReplayExportPacket,
) -> ReviewPackEvaluationInspectionRecord {
    let all_surfaces_bound_to_same_identity =
        evaluation.surface_observations.iter().all(|surface| {
            surface.review_pack_digest_ref == evaluation.review_pack_digest_ref
                && surface.base_identity_ref == evaluation.base_identity_ref
                && surface.head_identity_ref == evaluation.head_identity_ref
        });
    let any_surface_downgraded = evaluation
        .surface_observations
        .iter()
        .any(|surface| surface.surface_state_class != "surface_current_full");
    let advisory_ownership_present = evaluation
        .ownership_signals
        .iter()
        .any(|signal| signal.ownership_signal_class == "advisory_owner");
    let enforced_ownership_present = evaluation.ownership_signals.iter().any(|signal| {
        signal.ownership_signal_class == "enforced_owner"
            || signal.ownership_signal_class == "provider_authoritative_owner"
    });
    let stale_ai_findings_downgraded = evaluation.ai_findings.iter().all(|finding| {
        finding.diff_freshness_class != "diff_changed_materially"
            || finding.resolution_state == "outdated"
            || finding.resolution_state == "rerun_recommended"
    });
    let missing_provider_write_access_falls_back =
        evaluation.publish_previews.iter().all(|preview| {
            preview.provider_write_access_class != "missing_provider_write_access"
                || preview.local_copy_export_fallback_available
        });
    let required_check_vocabulary_replayable = replay_packet.required_check_names
        == evaluation
            .required_checks
            .iter()
            .map(|check| check.required_check_name.clone())
            .collect::<Vec<_>>();
    let replay_packet_exportable = replay_packet.headless_replay_supported
        && replay_packet.review_pack_digest_ref == evaluation.review_pack_digest_ref
        && replay_packet.base_identity_ref == evaluation.base_identity_ref
        && replay_packet.head_identity_ref == evaluation.head_identity_ref
        && !replay_packet.export_packet_refs.is_empty();
    let stable_full_parity_claim_allowed = evaluation.overall_verdict_class == "full_parity"
        && all_surfaces_bound_to_same_identity
        && !any_surface_downgraded
        && stale_ai_findings_downgraded
        && missing_provider_write_access_falls_back
        && replay_packet_exportable;

    ReviewPackEvaluationInspectionRecord {
        record_kind: REVIEW_PACK_EVALUATION_INSPECTION_RECORD_KIND.to_string(),
        schema_version: REVIEW_PACK_EVALUATOR_SCHEMA_VERSION,
        all_surfaces_bound_to_same_identity,
        any_surface_downgraded,
        required_check_vocabulary_replayable,
        advisory_ownership_present,
        enforced_ownership_present,
        stale_ai_findings_downgraded,
        missing_provider_write_access_falls_back,
        replay_packet_exportable,
        stable_full_parity_claim_allowed,
    }
}

fn validate_evaluation(
    evaluation: &ReviewPackStableEvaluationRecord,
) -> Result<(), ReviewPackEvaluatorValidationError> {
    require_equal(
        "record_kind",
        REVIEW_PACK_EVALUATION_RECORD_KIND,
        &evaluation.record_kind,
    )?;
    require_schema_version(evaluation.schema_version)?;
    require_non_empty("evaluation_id", &evaluation.evaluation_id)?;
    require_non_empty("generated_at", &evaluation.generated_at)?;
    require_non_empty("review_pack_id", &evaluation.review_pack_id)?;
    require_non_empty("review_pack_version", &evaluation.review_pack_version)?;
    require_non_empty("review_pack_digest_ref", &evaluation.review_pack_digest_ref)?;
    require_non_empty("base_identity_ref", &evaluation.base_identity_ref)?;
    require_non_empty("head_identity_ref", &evaluation.head_identity_ref)?;
    require_non_empty("repo_anchor_ref", &evaluation.repo_anchor_ref)?;
    require_one_of(
        "evaluator_capability_class",
        REVIEW_PACK_EVALUATOR_CAPABILITY_CLASSES,
        &evaluation.evaluator_capability_class,
    )?;
    require_one_of(
        "overall_verdict_class",
        REVIEW_PACK_EVALUATOR_VERDICT_CLASSES,
        &evaluation.overall_verdict_class,
    )?;
    validate_divergence_labels(&evaluation.divergence_labels)?;
    validate_surface_observations(evaluation)?;
    validate_scope_selectors(&evaluation.scope_selectors, &evaluation.divergence_labels)?;
    validate_required_checks(&evaluation.required_checks, &evaluation.divergence_labels)?;
    validate_ownership_signals(
        &evaluation.ownership_signals,
        &evaluation.scope_selectors,
        &evaluation.divergence_labels,
    )?;
    validate_ai_findings(
        &evaluation.ai_findings,
        &evaluation.scope_selectors,
        &evaluation.required_checks,
        &evaluation.publish_previews,
    )?;
    validate_publish_previews(&evaluation.publish_previews)?;
    validate_support_export(&evaluation.support_export)?;
    validate_consumer_surfaces(&evaluation.consumer_surfaces)?;
    cross_check_overall_verdict(evaluation)?;
    Ok(())
}

fn validate_surface_observations(
    evaluation: &ReviewPackStableEvaluationRecord,
) -> Result<(), ReviewPackEvaluatorValidationError> {
    let mut seen = BTreeSet::new();
    for surface in &evaluation.surface_observations {
        require_one_of(
            "surface_observations[].surface_class",
            REVIEW_PACK_EVALUATOR_SURFACE_CLASSES,
            &surface.surface_class,
        )?;
        require_one_of(
            "surface_observations[].surface_state_class",
            REVIEW_PACK_EVALUATOR_SURFACE_STATE_CLASSES,
            &surface.surface_state_class,
        )?;
        require_non_empty(
            "surface_observations[].review_pack_digest_ref",
            &surface.review_pack_digest_ref,
        )?;
        require_non_empty(
            "surface_observations[].base_identity_ref",
            &surface.base_identity_ref,
        )?;
        require_non_empty(
            "surface_observations[].head_identity_ref",
            &surface.head_identity_ref,
        )?;
        require_one_of(
            "surface_observations[].divergence_label",
            REVIEW_PACK_EVALUATOR_DIVERGENCE_LABELS,
            &surface.divergence_label,
        )?;
        require_non_empty("surface_observations[].summary", &surface.summary)?;
        if !seen.insert(surface.surface_class.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "surface_observations contains a duplicate surface_class: {}",
                surface.surface_class
            )));
        }
        if surface.surface_state_class != "surface_current_full"
            && surface.divergence_label == "no_divergence"
        {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "surface {} is downgraded but uses no_divergence",
                surface.surface_class
            )));
        }
        let identity_matches = surface.review_pack_digest_ref == evaluation.review_pack_digest_ref
            && surface.base_identity_ref == evaluation.base_identity_ref
            && surface.head_identity_ref == evaluation.head_identity_ref;
        if !identity_matches && surface.divergence_label != "review_pack_digest_mismatch" {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "surface {} identity drift must use review_pack_digest_mismatch",
                surface.surface_class
            )));
        }
    }
    for required in REVIEW_PACK_EVALUATOR_SURFACE_CLASSES {
        if !seen.contains(required) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "surface_observations must include {required}"
            )));
        }
    }
    Ok(())
}

fn validate_scope_selectors(
    selectors: &[ReviewScopeSelectorRecord],
    packet_divergence_labels: &[String],
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if selectors.is_empty() {
        return Err(ReviewPackEvaluatorValidationError::new(
            "scope_selectors must list at least one selector",
        ));
    }
    let mut seen = BTreeSet::new();
    for selector in selectors {
        require_non_empty("scope_selectors[].selector_id", &selector.selector_id)?;
        if !seen.insert(selector.selector_id.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "scope_selectors contains a duplicate selector_id: {}",
                selector.selector_id
            )));
        }
        require_one_of(
            "scope_selectors[].selector_class",
            REVIEW_PACK_EVALUATOR_SCOPE_SELECTOR_CLASSES,
            &selector.selector_class,
        )?;
        require_non_empty("scope_selectors[].selector_ref", &selector.selector_ref)?;
        require_one_of(
            "scope_selectors[].scope_truth_class",
            REVIEW_PACK_EVALUATOR_SCOPE_TRUTH_CLASSES,
            &selector.scope_truth_class,
        )?;
        require_one_of(
            "scope_selectors[].divergence_label",
            REVIEW_PACK_EVALUATOR_DIVERGENCE_LABELS,
            &selector.divergence_label,
        )?;
        require_non_empty("scope_selectors[].summary", &selector.summary)?;
        if selector.scope_truth_class != "full_scope" {
            require_packet_label(packet_divergence_labels, &selector.divergence_label)?;
        }
    }
    Ok(())
}

fn validate_required_checks(
    checks: &[ReviewPackRequiredCheckResult],
    packet_divergence_labels: &[String],
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if checks.is_empty() {
        return Err(ReviewPackEvaluatorValidationError::new(
            "required_checks must list at least one check",
        ));
    }
    let mut seen_ids = BTreeSet::new();
    let mut seen_names = BTreeSet::new();
    for check in checks {
        require_non_empty("required_checks[].check_id", &check.check_id)?;
        if !seen_ids.insert(check.check_id.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "required_checks contains a duplicate check_id: {}",
                check.check_id
            )));
        }
        require_non_empty(
            "required_checks[].required_check_name",
            &check.required_check_name,
        )?;
        if !seen_names.insert(check.required_check_name.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "required_checks contains a duplicate required_check_name: {}",
                check.required_check_name
            )));
        }
        require_one_of(
            "required_checks[].required_check_class",
            REVIEW_PACK_EVALUATOR_REQUIRED_CHECK_CLASSES,
            &check.required_check_class,
        )?;
        require_one_of(
            "required_checks[].enforcement_class",
            REVIEW_PACK_EVALUATOR_CHECK_ENFORCEMENT_CLASSES,
            &check.enforcement_class,
        )?;
        for (label, value) in [
            ("local_result_class", &check.local_result_class),
            ("ci_result_class", &check.ci_result_class),
            ("ai_review_result_class", &check.ai_review_result_class),
            ("provider_result_class", &check.provider_result_class),
            ("browser_result_class", &check.browser_result_class),
        ] {
            require_one_of(label, REVIEW_PACK_EVALUATOR_CHECK_RESULT_CLASSES, value)?;
        }
        require_one_of(
            "required_checks[].divergence_label",
            REVIEW_PACK_EVALUATOR_DIVERGENCE_LABELS,
            &check.divergence_label,
        )?;
        require_non_empty("required_checks[].summary", &check.summary)?;
        if [
            &check.local_result_class,
            &check.ci_result_class,
            &check.ai_review_result_class,
            &check.provider_result_class,
            &check.browser_result_class,
        ]
        .iter()
        .any(|result| *result != "passed")
            && check.divergence_label == "no_divergence"
        {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "check {} has a non-passing result but uses no_divergence",
                check.check_id
            )));
        }
        if check.divergence_label != "no_divergence" {
            require_packet_label(packet_divergence_labels, &check.divergence_label)?;
        }
    }
    Ok(())
}

fn validate_ownership_signals(
    signals: &[ReviewPackOwnershipSignalRecord],
    selectors: &[ReviewScopeSelectorRecord],
    packet_divergence_labels: &[String],
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if signals.is_empty() {
        return Err(ReviewPackEvaluatorValidationError::new(
            "ownership_signals must list advisory or enforced ownership truth",
        ));
    }
    let selector_ids: BTreeSet<&str> = selectors
        .iter()
        .map(|selector| selector.selector_id.as_str())
        .collect();
    let mut seen = BTreeSet::new();
    for signal in signals {
        require_non_empty("ownership_signals[].signal_id", &signal.signal_id)?;
        if !seen.insert(signal.signal_id.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "ownership_signals contains a duplicate signal_id: {}",
                signal.signal_id
            )));
        }
        if !selector_ids.contains(signal.scope_selector_ref.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "ownership signal {} references unknown scope selector {}",
                signal.signal_id, signal.scope_selector_ref
            )));
        }
        require_non_empty("ownership_signals[].owner_ref", &signal.owner_ref)?;
        require_one_of(
            "ownership_signals[].ownership_signal_class",
            REVIEW_PACK_EVALUATOR_OWNERSHIP_SIGNAL_CLASSES,
            &signal.ownership_signal_class,
        )?;
        require_one_of(
            "ownership_signals[].source_class",
            REVIEW_PACK_EVALUATOR_OWNERSHIP_SOURCE_CLASSES,
            &signal.source_class,
        )?;
        require_one_of(
            "ownership_signals[].divergence_label",
            REVIEW_PACK_EVALUATOR_DIVERGENCE_LABELS,
            &signal.divergence_label,
        )?;
        require_non_empty("ownership_signals[].summary", &signal.summary)?;
        if signal.ownership_signal_class == "advisory_owner"
            && matches!(
                signal.source_class.as_str(),
                "codeowners" | "provider_branch_rule" | "release_policy_bundle"
            )
        {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "ownership signal {} marks authoritative source as advisory_owner",
                signal.signal_id
            )));
        }
        if signal.ownership_signal_class != "advisory_owner"
            && matches!(
                signal.source_class.as_str(),
                "graph_ownership" | "manual_reviewer_suggestion"
            )
        {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "ownership signal {} marks advisory source as enforced",
                signal.signal_id
            )));
        }
        require_packet_label(packet_divergence_labels, &signal.divergence_label)?;
    }
    Ok(())
}

fn validate_ai_findings(
    findings: &[ReviewPackAiFindingRecord],
    selectors: &[ReviewScopeSelectorRecord],
    checks: &[ReviewPackRequiredCheckResult],
    publish_previews: &[ReviewPackPublishPreviewRecord],
) -> Result<(), ReviewPackEvaluatorValidationError> {
    let selector_ids: BTreeSet<&str> = selectors
        .iter()
        .map(|selector| selector.selector_id.as_str())
        .collect();
    let check_ids: BTreeSet<&str> = checks.iter().map(|check| check.check_id.as_str()).collect();
    let preview_ids: BTreeSet<&str> = publish_previews
        .iter()
        .map(|preview| preview.preview_id.as_str())
        .collect();
    let mut seen = BTreeSet::new();
    for finding in findings {
        require_non_empty("ai_findings[].finding_id", &finding.finding_id)?;
        if !seen.insert(finding.finding_id.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "ai_findings contains a duplicate finding_id: {}",
                finding.finding_id
            )));
        }
        if !selector_ids.contains(finding.scope_selector_ref.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "AI finding {} references unknown scope selector {}",
                finding.finding_id, finding.scope_selector_ref
            )));
        }
        if !check_ids.contains(finding.check_ref.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "AI finding {} references unknown check {}",
                finding.finding_id, finding.check_ref
            )));
        }
        require_non_empty("ai_findings[].title", &finding.title)?;
        require_non_empty("ai_findings[].severity_class", &finding.severity_class)?;
        require_non_empty("ai_findings[].confidence_class", &finding.confidence_class)?;
        require_unique("ai_findings[].evidence_refs", &finding.evidence_refs)?;
        require_one_of(
            "ai_findings[].resolution_state",
            REVIEW_PACK_EVALUATOR_AI_FINDING_STATES,
            &finding.resolution_state,
        )?;
        require_one_of(
            "ai_findings[].diff_freshness_class",
            REVIEW_PACK_EVALUATOR_AI_DIFF_FRESHNESS_CLASSES,
            &finding.diff_freshness_class,
        )?;
        require_one_of(
            "ai_findings[].divergence_label",
            REVIEW_PACK_EVALUATOR_DIVERGENCE_LABELS,
            &finding.divergence_label,
        )?;
        require_non_empty("ai_findings[].summary", &finding.summary)?;
        if finding.diff_freshness_class == "diff_changed_materially"
            && finding.resolution_state != "outdated"
            && finding.resolution_state != "rerun_recommended"
        {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "AI finding {} changed materially but is not outdated or rerun_recommended",
                finding.finding_id
            )));
        }
        if let Some(preview_ref) = &finding.publish_preview_ref {
            if !preview_ids.contains(preview_ref.as_str()) {
                return Err(ReviewPackEvaluatorValidationError::new(format!(
                    "AI finding {} references unknown publish preview {}",
                    finding.finding_id, preview_ref
                )));
            }
        }
    }
    Ok(())
}

fn validate_publish_previews(
    previews: &[ReviewPackPublishPreviewRecord],
) -> Result<(), ReviewPackEvaluatorValidationError> {
    let mut seen = BTreeSet::new();
    for preview in previews {
        require_non_empty("publish_previews[].preview_id", &preview.preview_id)?;
        if !seen.insert(preview.preview_id.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "publish_previews contains a duplicate preview_id: {}",
                preview.preview_id
            )));
        }
        require_non_empty(
            "publish_previews[].destination_ref",
            &preview.destination_ref,
        )?;
        require_one_of(
            "publish_previews[].provider_write_access_class",
            REVIEW_PACK_EVALUATOR_PROVIDER_WRITE_ACCESS_CLASSES,
            &preview.provider_write_access_class,
        )?;
        require_one_of(
            "publish_previews[].outbound_intent_class",
            REVIEW_PACK_EVALUATOR_OUTBOUND_INTENT_CLASSES,
            &preview.outbound_intent_class,
        )?;
        require_non_empty(
            "publish_previews[].outbound_payload_preview",
            &preview.outbound_payload_preview,
        )?;
        require_one_of(
            "publish_previews[].divergence_label",
            REVIEW_PACK_EVALUATOR_DIVERGENCE_LABELS,
            &preview.divergence_label,
        )?;
        require_non_empty("publish_previews[].summary", &preview.summary)?;
        if preview.provider_write_access_class == "missing_provider_write_access"
            && !preview.local_copy_export_fallback_available
        {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "publish preview {} is missing provider write access but hides local copy/export fallback",
                preview.preview_id
            )));
        }
        if preview.provider_write_access_class == "missing_provider_write_access"
            && preview.outbound_intent_class == "publish_to_review"
        {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "publish preview {} cannot claim publish_to_review without provider write access",
                preview.preview_id
            )));
        }
    }
    Ok(())
}

fn validate_support_export(
    export: &ReviewPackEvaluationSupportExport,
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if export.raw_path_export_allowed
        || export.raw_glob_body_export_allowed
        || export.raw_command_export_allowed
        || export.raw_check_output_export_allowed
    {
        return Err(ReviewPackEvaluatorValidationError::new(
            "support_export must keep raw_*_export_allowed false",
        ));
    }
    require_unique(
        "support_export.export_packet_refs",
        &export.export_packet_refs,
    )?;
    if export.export_packet_refs.is_empty() {
        return Err(ReviewPackEvaluatorValidationError::new(
            "support_export.export_packet_refs must contain at least one replay/export ref",
        ));
    }
    require_non_empty("support_export.redaction_class", &export.redaction_class)?;
    Ok(())
}

fn validate_consumer_surfaces(
    surfaces: &[String],
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if surfaces.is_empty() {
        return Err(ReviewPackEvaluatorValidationError::new(
            "consumer_surfaces must list at least one surface",
        ));
    }
    require_unique("consumer_surfaces", surfaces)?;
    for required in [
        "review_workspace",
        "hosted_provider_overlay",
        "local_ci",
        "ai_review",
        "browser_companion",
        "cli_headless",
        "support_export",
    ] {
        if !surfaces.iter().any(|surface| surface == required) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "consumer_surfaces must include {required}"
            )));
        }
    }
    Ok(())
}

fn cross_check_overall_verdict(
    evaluation: &ReviewPackStableEvaluationRecord,
) -> Result<(), ReviewPackEvaluatorValidationError> {
    let any_surface_downgraded = evaluation
        .surface_observations
        .iter()
        .any(|surface| surface.surface_state_class != "surface_current_full");
    let identity_mismatch = evaluation.surface_observations.iter().any(|surface| {
        surface.review_pack_digest_ref != evaluation.review_pack_digest_ref
            || surface.base_identity_ref != evaluation.base_identity_ref
            || surface.head_identity_ref != evaluation.head_identity_ref
    });
    let unsupported = evaluation.evaluator_capability_class == "unsupported_evaluator_capability"
        || evaluation
            .divergence_labels
            .iter()
            .any(|label| label == "unsupported_evaluator_capability");
    let stale = evaluation
        .divergence_labels
        .iter()
        .any(|label| label == "stale_pack" || label == "ai_review_outdated");
    if evaluation.overall_verdict_class == "full_parity"
        && (any_surface_downgraded || identity_mismatch || unsupported || stale)
    {
        return Err(ReviewPackEvaluatorValidationError::new(
            "overall_verdict_class=full_parity cannot hide downgraded surfaces, identity mismatch, unsupported capabilities, or stale results",
        ));
    }
    if unsupported && evaluation.overall_verdict_class != "unsupported_capability_downgraded" {
        return Err(ReviewPackEvaluatorValidationError::new(
            "unsupported evaluator capability must use unsupported_capability_downgraded verdict",
        ));
    }
    if stale && evaluation.overall_verdict_class == "full_parity" {
        return Err(ReviewPackEvaluatorValidationError::new(
            "stale review-pack or AI finding truth cannot retain full_parity",
        ));
    }
    if identity_mismatch
        && !evaluation
            .divergence_labels
            .iter()
            .any(|label| label == "review_pack_digest_mismatch")
    {
        return Err(ReviewPackEvaluatorValidationError::new(
            "identity mismatch must be exported with review_pack_digest_mismatch",
        ));
    }
    Ok(())
}

fn validate_replay_packet(
    evaluation: &ReviewPackStableEvaluationRecord,
    replay: &ReviewPackReplayExportPacket,
) -> Result<(), ReviewPackEvaluatorValidationError> {
    require_equal(
        "record_kind",
        REVIEW_PACK_REPLAY_EXPORT_PACKET_RECORD_KIND,
        &replay.record_kind,
    )?;
    require_schema_version(replay.schema_version)?;
    require_non_empty("replay_packet_id", &replay.replay_packet_id)?;
    require_equal(
        "replay_packet.evaluation_ref",
        &evaluation.evaluation_id,
        &replay.evaluation_ref,
    )?;
    require_equal(
        "replay_packet.review_pack_version",
        &evaluation.review_pack_version,
        &replay.review_pack_version,
    )?;
    require_equal(
        "replay_packet.review_pack_digest_ref",
        &evaluation.review_pack_digest_ref,
        &replay.review_pack_digest_ref,
    )?;
    require_equal(
        "replay_packet.base_identity_ref",
        &evaluation.base_identity_ref,
        &replay.base_identity_ref,
    )?;
    require_equal(
        "replay_packet.head_identity_ref",
        &evaluation.head_identity_ref,
        &replay.head_identity_ref,
    )?;
    if !replay.headless_replay_supported {
        return Err(ReviewPackEvaluatorValidationError::new(
            "replay_packet must support headless replay",
        ));
    }
    require_equal_vec(
        "replay_packet.required_check_names",
        &evaluation
            .required_checks
            .iter()
            .map(|check| check.required_check_name.clone())
            .collect::<Vec<_>>(),
        &replay.required_check_names,
    )?;
    require_equal_vec(
        "replay_packet.divergence_labels",
        &evaluation.divergence_labels,
        &replay.divergence_labels,
    )?;
    require_equal_vec(
        "replay_packet.ai_finding_refs",
        &evaluation
            .ai_findings
            .iter()
            .map(|finding| finding.finding_id.clone())
            .collect::<Vec<_>>(),
        &replay.ai_finding_refs,
    )?;
    require_non_empty("replay_packet.redaction_class", &replay.redaction_class)?;
    require_unique(
        "replay_packet.export_packet_refs",
        &replay.export_packet_refs,
    )?;
    if replay.export_packet_refs.is_empty() {
        return Err(ReviewPackEvaluatorValidationError::new(
            "replay_packet.export_packet_refs must not be empty",
        ));
    }
    Ok(())
}

fn validate_divergence_labels(labels: &[String]) -> Result<(), ReviewPackEvaluatorValidationError> {
    if labels.is_empty() {
        return Err(ReviewPackEvaluatorValidationError::new(
            "divergence_labels must list at least no_divergence or a downgrade label",
        ));
    }
    require_unique("divergence_labels", labels)?;
    for label in labels {
        require_one_of(
            "divergence_labels[]",
            REVIEW_PACK_EVALUATOR_DIVERGENCE_LABELS,
            label,
        )?;
    }
    Ok(())
}

fn require_packet_label(
    packet_divergence_labels: &[String],
    label: &str,
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if packet_divergence_labels
        .iter()
        .any(|candidate| candidate == label)
    {
        Ok(())
    } else {
        Err(ReviewPackEvaluatorValidationError::new(format!(
            "packet divergence_labels must include {label}"
        )))
    }
}

fn require_schema_version(version: u32) -> Result<(), ReviewPackEvaluatorValidationError> {
    if version == REVIEW_PACK_EVALUATOR_SCHEMA_VERSION {
        Ok(())
    } else {
        Err(ReviewPackEvaluatorValidationError::new(format!(
            "schema_version is {}, expected {}",
            version, REVIEW_PACK_EVALUATOR_SCHEMA_VERSION
        )))
    }
}

fn require_equal(
    label: &str,
    expected: &str,
    actual: &str,
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(ReviewPackEvaluatorValidationError::new(format!(
            "{label} mismatch: expected {expected}, got {actual}"
        )))
    }
}

fn require_equal_vec(
    label: &str,
    expected: &[String],
    actual: &[String],
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if expected == actual {
        Ok(())
    } else {
        Err(ReviewPackEvaluatorValidationError::new(format!(
            "{label} mismatch: expected {expected:?}, got {actual:?}"
        )))
    }
}

fn require_non_empty(label: &str, value: &str) -> Result<(), ReviewPackEvaluatorValidationError> {
    if value.trim().is_empty() {
        Err(ReviewPackEvaluatorValidationError::new(format!(
            "{label} must be a non-empty string"
        )))
    } else {
        Ok(())
    }
}

fn require_one_of(
    label: &str,
    allowed: &[&str],
    value: &str,
) -> Result<(), ReviewPackEvaluatorValidationError> {
    if allowed.iter().any(|candidate| *candidate == value) {
        Ok(())
    } else {
        Err(ReviewPackEvaluatorValidationError::new(format!(
            "{label} value {value} is not in the closed vocabulary"
        )))
    }
}

fn require_unique(
    label: &str,
    values: &[String],
) -> Result<(), ReviewPackEvaluatorValidationError> {
    let mut seen = BTreeSet::new();
    for value in values {
        if !seen.insert(value.as_str()) {
            return Err(ReviewPackEvaluatorValidationError::new(format!(
                "{label} contains a duplicate entry: {value}"
            )));
        }
    }
    Ok(())
}
