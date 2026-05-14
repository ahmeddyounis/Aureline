//! Support-bundle manifest record types.
//!
//! Mirrors the boundary schema at
//! `/schemas/support/support_bundle_manifest.schema.json` and the preview-
//! item schema at
//! `/schemas/support/support_bundle_preview_item.schema.json`. The seed
//! exposes only the fields needed to mint a manifest from the live shell
//! and round-trip it through serde for the on-disk preview snapshot. The
//! schemas remain the authoritative shape — this module never invents
//! its own field names.

use serde::{Deserialize, Serialize};

use super::vocabulary::{
    ActionabilityImpactClass, ActorClass, DiagnosticDataClass, ExcludedReasonClass,
    HighRiskContentClass, PolicyNoteSeverity, RedactionState, ReleaseChannelClass,
    ReviewDecidedByClass, ReviewDecisionClass, SecretScanState, TrustState,
};

/// Stable record-kind tag carried on every preview-item record.
pub const SUPPORT_BUNDLE_PREVIEW_ITEM_RECORD_KIND: &str = "support_bundle_preview_item_record";

/// Stable record-kind tag carried on every manifest record.
pub const SUPPORT_BUNDLE_MANIFEST_RECORD_KIND: &str = "support_bundle_manifest_record";

/// Frozen collection-schema version for every preview / post-export
/// manifest minted by this seed.
pub const COLLECTION_SCHEMA_VERSION: u32 = 1;

/// Frozen preview-item schema version.
pub const SUPPORT_BUNDLE_PREVIEW_ITEM_SCHEMA_VERSION: u32 = 1;

/// Build identity block embedded in every manifest. Mirrors the schema's
/// `build_identity` object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIdentity {
    pub build_id: String,
    pub producer_build_id: String,
    pub product_version: String,
    pub release_channel_class: ReleaseChannelClass,
    pub exact_build_refs: Vec<String>,
}

/// Mirrors the schema's `policy_context` object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    pub policy_epoch: u64,
    pub trust_state: TrustState,
    #[serde(default)]
    pub policy_bundle_ref: Option<String>,
}

/// Mirrors the schema's `policy_note` object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyNote {
    pub note_id: String,
    pub severity: PolicyNoteSeverity,
    pub source_ref: String,
    pub note: String,
}

/// Mirrors the schema's `collection_context` object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CollectionContext {
    pub generated_at: String,
    pub actor_class: ActorClass,
    pub active_redaction_profile_ref: String,
    pub collection_intent: String,
    pub policy_context: PolicyContext,
    pub policy_notes: Vec<PolicyNote>,
}

/// Mirrors the schema's `support_bundle_preview_item.file_section_identity`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileSectionIdentity {
    pub section_id: String,
    pub bundle_section_class: String,
    pub artifact_kind_class: String,
    pub preview_label: String,
    pub manifest_path_ref: String,
    #[serde(default)]
    pub bundle_member_path_ref: Option<String>,
    pub source_refs: Vec<String>,
}

/// Mirrors the schema's `support_bundle_preview_item.size_estimate`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SizeEstimate {
    #[serde(default)]
    pub estimated_bytes: Option<u64>,
    pub confidence_class: String,
    pub display_label: String,
    pub size_source_class: String,
}

/// Mirrors the schema's `support_bundle_preview_item.redaction`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Redaction {
    pub data_class: DiagnosticDataClass,
    pub high_risk_content_class: HighRiskContentClass,
    pub redaction_class: String,
    pub redaction_state: RedactionState,
    #[serde(default)]
    pub visible_high_risk_label: Option<String>,
    pub redaction_rule_refs: Vec<String>,
    pub redaction_summary_ref: String,
}

/// Mirrors the schema's `support_bundle_preview_item.actionability_impact`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionabilityImpact {
    pub impact_class: ActionabilityImpactClass,
    pub affects_first_actionable_diagnosis: bool,
    pub warning_required: bool,
    #[serde(default)]
    pub warning_text: Option<String>,
    pub impact_summary: String,
}

/// Mirrors the schema's `support_bundle_preview_item.parity_binding`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParityBinding {
    pub support_pack_item_id: String,
    pub inclusion_rule_ref: String,
    pub export_manifest_field_ref: String,
    pub preview_decision_ref: String,
    #[serde(default)]
    pub item_digest_ref: Option<String>,
    pub exact_build_identity_refs: Vec<String>,
    pub post_export_reconstruction_fields: Vec<String>,
}

/// Mirrors the schema's `support_bundle_preview_item.policy_lock`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyLock {
    pub locked_by_policy: bool,
    pub reason_class: String,
    #[serde(default)]
    pub policy_ref: Option<String>,
    pub reason_summary: String,
}

/// One row of a support-bundle preview. Mirrors
/// `support_bundle_preview_item_record` in the boundary schema. Keeps the
/// record small: the seed uses fixed `materialization` and `deselectability`
/// blocks projected from the redaction defaults rather than asking the
/// caller to pick them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportBundlePreviewItem {
    pub support_bundle_preview_item_schema_version: u32,
    pub record_kind: String,
    pub preview_item_id: String,
    pub title: String,
    pub file_section_identity: FileSectionIdentity,
    pub size_estimate: SizeEstimate,
    pub redaction: Redaction,
    pub materialization: serde_json::Value,
    pub deselectability: serde_json::Value,
    pub actionability_impact: ActionabilityImpact,
    pub parity_binding: ParityBinding,
    pub policy_lock: PolicyLock,
    pub notes: String,
}

/// Mirrors the schema's `review_decision`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewDecision {
    pub preview_item_id: String,
    pub support_pack_item_id: String,
    pub decision_class: ReviewDecisionClass,
    pub selected_redaction_state: RedactionState,
    pub decided_by_class: ReviewDecidedByClass,
    pub decision_reason: String,
    #[serde(default)]
    pub actionability_warning_ack_ref: Option<String>,
}

/// Mirrors the schema's `excluded_class`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExcludedClass {
    pub data_class: DiagnosticDataClass,
    pub high_risk_content_class: HighRiskContentClass,
    #[serde(default)]
    pub support_pack_item_id: Option<String>,
    pub artifact_kind_class: String,
    pub exclusion_reason_class: ExcludedReasonClass,
    pub explicit_reason: String,
    #[serde(default)]
    pub policy_ref: Option<String>,
    pub omission_marker_ref: String,
}

/// Mirrors the schema's `redaction_report.high_risk_items[]` entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighRiskItemEntry {
    pub preview_item_id: String,
    pub data_class: DiagnosticDataClass,
    pub high_risk_content_class: HighRiskContentClass,
    pub handling_summary: String,
}

/// Mirrors the schema's `redaction_report.secret_scan_summary`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SecretScanSummary {
    pub scan_state: SecretScanState,
    pub detected_marker_count: u32,
    /// Always `false`. The seed never exports raw secret values; the
    /// schema also pins this to `false` via `const`.
    pub raw_secret_values_exported: bool,
    pub notes: String,
}

/// Mirrors the schema's `redaction_report`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionReport {
    pub report_id: String,
    pub redaction_profile_ref: String,
    pub redaction_pass_ref: String,
    pub redaction_states_present: Vec<RedactionState>,
    pub high_risk_items: Vec<HighRiskItemEntry>,
    pub prohibited_items_confirmed_absent: Vec<String>,
    pub applied_rule_refs: Vec<String>,
    pub secret_scan_summary: SecretScanSummary,
    pub reviewer_visible_summary: String,
}

/// Roll-up of included and excluded preview classes before export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewClassificationSummary {
    /// Count of preview rows that are included in the default export.
    pub included_count: u32,
    /// Count of preview rows omitted, retained locally, policy locked, or prohibited.
    pub excluded_count: u32,
    /// Count of rows retained on the local machine only.
    pub retained_local_only_count: u32,
    /// Count of rows prohibited from export.
    pub prohibited_count: u32,
    /// Diagnostic data classes present in the preview.
    pub data_classes_present: Vec<DiagnosticDataClass>,
    /// Redaction states present in the preview.
    pub redaction_states_present: Vec<RedactionState>,
    /// Support-pack item ids included by default.
    pub included_support_pack_item_ids: Vec<String>,
    /// Support-pack item ids excluded before export.
    pub excluded_support_pack_item_ids: Vec<String>,
    /// Reviewer-visible summary for the preview classification.
    pub summary: String,
}

/// Redaction control exposed for one preview item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RedactionControl {
    /// Stable control id for audit and preview/export parity.
    pub control_id: String,
    /// Preview item controlled by this row.
    pub preview_item_id: String,
    /// Support-pack item id controlled by this row.
    pub support_pack_item_id: String,
    /// Default redaction state from the active profile.
    pub default_redaction_state: RedactionState,
    /// Redaction state selected for this manifest.
    pub selected_redaction_state: RedactionState,
    /// States the reviewer may choose without broadening capture.
    pub allowed_narrower_states: Vec<RedactionState>,
    /// Whether a broader capture would require a reviewed follow-up path.
    pub broadening_requires_review: bool,
    /// Whether raw content may be exported through this control.
    pub raw_content_export_allowed: bool,
    /// Whether active policy locks the row.
    pub policy_locked: bool,
    /// Reviewer-visible reason for the available control set.
    pub control_note: String,
}

/// Mirrors the schema's `actionability_warning`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionabilityWarning {
    pub warning_id: String,
    pub preview_item_id: String,
    pub impact_class: ActionabilityImpactClass,
    pub warning_text: String,
    pub required_before_export: bool,
    #[serde(default)]
    pub acknowledged_by_ref: Option<String>,
}

/// Mirrors the schema's `reopen_after_export_path`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReopenAfterExportPath {
    pub manifest_member_path: String,
    pub local_reopen_ref: String,
    pub product_route_ref: String,
    pub can_reopen_without_network: bool,
    pub preserved_preview_snapshot_ref: String,
    pub notes: String,
}

/// Mirrors the schema's `preview_export_parity`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewExportParity {
    pub preview_snapshot_ref: String,
    pub preview_item_order_digest: String,
    pub export_manifest_digest: String,
    /// Always `true` per schema.
    pub manifest_reconstructs_shared_payload: bool,
    pub reconstruction_fields: Vec<String>,
    pub item_decision_refs: Vec<String>,
    pub unknown_field_policy: String,
    pub parity_assertions: Vec<String>,
}

/// Policy source active for a reconstructed action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionPolicySourceContext {
    /// Stable policy source ref, policy bundle ref, or typed local default token.
    pub policy_source_ref: String,
    /// Policy epoch active when the action was reviewed or invoked.
    pub policy_epoch: String,
    /// Trust state active when the action was reviewed or invoked.
    pub trust_state: String,
    /// Optional policy bundle or execution-context ref backing the decision.
    #[serde(default)]
    pub policy_bundle_ref: Option<String>,
    /// Source class for this policy context.
    pub source_class: String,
}

/// Typed route and command reconstruction context for one reviewed action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionReconstructionContext {
    /// Stable context id within the manifest.
    pub reconstruction_context_id: String,
    /// Preview row that carries the action-route truth packet.
    pub preview_item_id: String,
    /// Support-pack item id for the action-route truth packet.
    pub support_pack_item_id: String,
    /// Command Aureline believed it was running.
    pub command_id: String,
    /// Descriptor or revision ref used for the command.
    pub command_descriptor_ref: String,
    /// Invocation session id joined to command result and incident packets.
    pub invocation_session_id: String,
    /// Target identity ref or typed target token.
    pub target_identity_ref: String,
    /// Optional route-truth packet ref.
    #[serde(default)]
    pub action_route_packet_ref: Option<String>,
    /// Origin class from the action-route taxonomy.
    pub action_origin_class: String,
    /// Target class from the action-route taxonomy.
    pub action_target_class: String,
    /// Route class from the action-route taxonomy.
    pub action_route_class: String,
    /// Exposure class from the action-route taxonomy.
    pub action_exposure_class: String,
    /// Policy source that governed the action.
    pub policy_source: ActionPolicySourceContext,
    /// Redaction-safe route summary.
    pub route_summary: String,
    /// Optional link to the reviewed-command enforcement row.
    #[serde(default)]
    pub reviewed_enforcement_ref: Option<String>,
    /// Exact-build refs copied from the manifest build identity.
    pub exact_build_refs: Vec<String>,
    /// Redaction class applied to this reconstruction row.
    pub redaction_class: String,
    /// Always false for the alpha manifest path.
    pub raw_content_exported: bool,
}

/// Top-level support-bundle manifest record. Mirrors
/// `support_bundle_manifest_record` in the boundary schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportBundleManifest {
    pub collection_schema_version: u32,
    pub record_kind: String,
    pub manifest_id: String,
    pub support_bundle_id: String,
    pub title: String,
    pub build_identity: BuildIdentity,
    pub collection_context: CollectionContext,
    pub preview_items: Vec<SupportBundlePreviewItem>,
    pub review_decisions: Vec<ReviewDecision>,
    pub excluded_classes: Vec<ExcludedClass>,
    pub redaction_report: RedactionReport,
    pub preview_classification_summary: PreviewClassificationSummary,
    pub redaction_controls: Vec<RedactionControl>,
    pub action_reconstruction_contexts: Vec<ActionReconstructionContext>,
    pub actionability_warnings: Vec<ActionabilityWarning>,
    pub reopen_after_export_path: ReopenAfterExportPath,
    pub preview_export_parity: PreviewExportParity,
    pub emitted_at: String,
    pub notes: String,
}

impl SupportBundleManifest {
    /// True when the manifest carries at least one row whose redaction
    /// state is `prohibited` (i.e. the failure drill rewrote a queued
    /// row to keep raw secret material out of the export).
    pub fn has_prohibited_row(&self) -> bool {
        self.preview_items
            .iter()
            .any(|item| matches!(item.redaction.redaction_state, RedactionState::Prohibited))
    }

    /// True when the manifest's exact-build refs resolve to a non-empty
    /// list. Used by the reviewer-facing assertion that every export
    /// carries exact-build identity.
    pub fn has_exact_build_identity(&self) -> bool {
        !self.build_identity.exact_build_refs.is_empty()
    }
}
