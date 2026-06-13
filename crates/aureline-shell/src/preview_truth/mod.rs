//! Preview-truth records for retained notebook and structured-config wedges.
//!
//! The records in this module keep preview-only notebook and structured-config
//! surfaces honest before they mutate runtime-heavy artifacts. They separate
//! document, runtime, and output trust; distinguish authored, effective, and
//! live structured-state projections; and bind rerun, repair, trust-raise, and
//! output-clear flows to checkpoints plus mutation-journal lineage.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::document_identity::{DocumentFamilyClass, DocumentIdentityDisclosure};

/// Stable record-kind tag for [`PreviewTruthRecord`].
pub const PREVIEW_TRUTH_RECORD_KIND: &str = "preview_truth_record";

/// Schema version for preview-truth records.
pub const PREVIEW_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`PreviewTruthSupportExportRecord`].
pub const PREVIEW_TRUTH_SUPPORT_EXPORT_RECORD_KIND: &str = "preview_truth_support_export_record";

/// Retained preview surface covered by a preview-truth record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewSurfaceClass {
    /// Notebook preview row with document/runtime/output trust.
    NotebookPreview,
    /// Structured configuration preview row with source/effective/live views.
    StructuredConfigPreview,
}

impl PreviewSurfaceClass {
    /// Returns the stable token used in fixtures and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookPreview => "notebook_preview",
            Self::StructuredConfigPreview => "structured_config_preview",
        }
    }
}

/// Qualification posture for a retained preview wedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewQualificationClass {
    /// The surface is explicitly retained as a preview with limited depth.
    PreviewLimited,
    /// The surface is limited even if parts of the row are production-backed.
    Limited,
    /// The surface has evidence for a stable claim.
    StableQualified,
}

impl PreviewQualificationClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewLimited => "preview_limited",
            Self::Limited => "limited",
            Self::StableQualified => "stable_qualified",
        }
    }

    const fn is_stable(self) -> bool {
        matches!(self, Self::StableQualified)
    }
}

/// One explicit scope limit for a preview wedge claim manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewClaimLimitRow {
    /// Stable claim-limit token.
    pub token: String,
    /// Reviewer-facing limit label.
    pub label: String,
}

/// Claim manifest carried by a retained preview wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewClaimManifest {
    /// Human-readable surface label.
    pub surface_label: String,
    /// Qualification posture for this retained wedge.
    pub qualification_class: PreviewQualificationClass,
    /// Stable token for [`Self::qualification_class`].
    pub qualification_class_token: String,
    /// Visible label shown anywhere the surface can be mistaken for stable.
    pub visible_qualification_label: String,
    /// Evidence ref required for stable qualification.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qualification_evidence_ref: Option<String>,
    /// Explicit claim limits shown beside the preview label.
    pub claim_limits: Vec<PreviewClaimLimitRow>,
}

impl PreviewClaimManifest {
    fn refresh_tokens(&mut self) {
        self.qualification_class_token = self.qualification_class.as_str().to_owned();
    }
}

/// Trust layer that can independently block or permit a preview action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustLayerClass {
    /// Stored document or artifact trust.
    Document,
    /// Kernel, connector, runtime, or live-target trust.
    Runtime,
    /// Output, rendered projection, or observed-state trust.
    Output,
}

impl TrustLayerClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::Runtime => "runtime",
            Self::Output => "output",
        }
    }
}

/// Gate state for one trust layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustGateState {
    /// The trust layer permits the named action.
    PermitsAction,
    /// The trust layer permits inspection but no mutation.
    PermitsInspectionOnly,
    /// The trust layer blocks the named action.
    BlocksAction,
}

impl TrustGateState {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PermitsAction => "permits_action",
            Self::PermitsInspectionOnly => "permits_inspection_only",
            Self::BlocksAction => "blocks_action",
        }
    }
}

/// One document/runtime/output trust chip row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustLayerRow {
    /// Trust layer class.
    pub layer_class: TrustLayerClass,
    /// Stable token for [`Self::layer_class`].
    pub layer_class_token: String,
    /// Surface-specific trust state token.
    pub trust_state_token: String,
    /// Gate state for the action refs.
    pub gate_state: TrustGateState,
    /// Stable token for [`Self::gate_state`].
    pub gate_state_token: String,
    /// Action refs this layer blocks or permits.
    pub action_refs: Vec<String>,
    /// Reviewer-facing reason for the gate.
    pub gate_explanation_label: String,
    /// Evidence ref for the trust state, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

impl TrustLayerRow {
    fn refresh_tokens(&mut self) {
        self.layer_class_token = self.layer_class.as_str().to_owned();
        self.gate_state_token = self.gate_state.as_str().to_owned();
    }
}

/// View class for structured-config truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuredViewClass {
    /// Authored source is the canonical editable artifact.
    AuthoredSource,
    /// Effective projection after defaults, overlays, policy, or refs resolve.
    EffectiveProjection,
    /// Live or observed state reported by a runtime target.
    LiveObservedState,
}

impl StructuredViewClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredSource => "authored_source",
            Self::EffectiveProjection => "effective_projection",
            Self::LiveObservedState => "live_observed_state",
        }
    }
}

/// Write posture for a structured-config view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StructuredWriteEligibility {
    /// The row is the editable canonical source.
    EditableCanonicalSource,
    /// The row is a read-only projection.
    ReadOnlyProjection,
    /// The row is inspect-only because authority or freshness is limited.
    InspectOnly,
    /// The row is unavailable.
    NotAvailable,
}

impl StructuredWriteEligibility {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditableCanonicalSource => "editable_canonical_source",
            Self::ReadOnlyProjection => "read_only_projection",
            Self::InspectOnly => "inspect_only",
            Self::NotAvailable => "not_available",
        }
    }

    const fn can_write(self) -> bool {
        matches!(self, Self::EditableCanonicalSource)
    }
}

/// One authored/effective/live structured-state projection row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredStateViewRow {
    /// Stable view id.
    pub view_id: String,
    /// Structured view class.
    pub view_class: StructuredViewClass,
    /// Stable token for [`Self::view_class`].
    pub view_class_token: String,
    /// Opaque source or projection ref.
    pub source_ref: String,
    /// Resolution time label such as `authored`, `at run`, or `observed live`.
    pub resolution_time_label: String,
    /// Target boundary ref used for effective or live state.
    pub target_boundary_ref: String,
    /// Write eligibility for this view.
    pub write_eligibility: StructuredWriteEligibility,
    /// Stable token for [`Self::write_eligibility`].
    pub write_eligibility_token: String,
    /// Summary of redaction, unresolved, deferred, or literal value posture.
    pub value_posture_label: String,
    /// Freshness label for the projection.
    pub freshness_label: String,
}

impl StructuredStateViewRow {
    fn refresh_tokens(&mut self) {
        self.view_class_token = self.view_class.as_str().to_owned();
        self.write_eligibility_token = self.write_eligibility.as_str().to_owned();
    }
}

/// Structured round-trip risk class re-exported by preview records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundTripRiskClass {
    /// Every declared field and attachment survives.
    LosslessRoundtrip,
    /// Metadata, comments, formatting, or display-only annotations are lost.
    LossyMetadataOnly,
    /// Output representations are normalized or downgraded.
    LossyOutputRepresentation,
    /// Structural fields, ordering, attachments, or unknown fields may change.
    LossyStructural,
    /// Apply would trigger an irreversible side effect.
    LossyIrreversible,
    /// The surface cannot determine risk.
    RoundTripUnavailable,
    /// Policy blocks the round trip.
    RoundTripPolicyBlocked,
}

impl RoundTripRiskClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LosslessRoundtrip => "lossless_roundtrip",
            Self::LossyMetadataOnly => "lossy_metadata_only",
            Self::LossyOutputRepresentation => "lossy_output_representation",
            Self::LossyStructural => "lossy_structural",
            Self::LossyIrreversible => "lossy_irreversible",
            Self::RoundTripUnavailable => "round_trip_unavailable",
            Self::RoundTripPolicyBlocked => "round_trip_policy_blocked",
        }
    }

    const fn is_lossless(self) -> bool {
        matches!(self, Self::LosslessRoundtrip)
    }

    const fn needs_raw_preservation(self) -> bool {
        matches!(self, Self::LossyStructural | Self::LossyIrreversible)
    }
}

/// Preview representation class for a round-trip risk row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewRepresentationClass {
    /// Full-fidelity preview.
    FullFidelityPreview,
    /// Normalized preview.
    NormalisedPreview,
    /// Summary-only preview.
    SummaryOnlyPreview,
    /// Tombstone refusal preview.
    TombstonePreview,
}

impl PreviewRepresentationClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullFidelityPreview => "full_fidelity_preview",
            Self::NormalisedPreview => "normalised_preview",
            Self::SummaryOnlyPreview => "summary_only_preview",
            Self::TombstonePreview => "tombstone_preview",
        }
    }
}

/// Apply gate class for a round-trip risk row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyGateClass {
    /// Apply can proceed after basis checks.
    AllowApply,
    /// Apply can proceed with a visible warning.
    WarnAllowApply,
    /// Compare-first review is required.
    RequireCompareFirstReview,
    /// Typed confirmation is required.
    RequireTypedConfirmation,
    /// Rewrite is refused.
    RefuseRewrite,
}

impl ApplyGateClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllowApply => "allow_apply",
            Self::WarnAllowApply => "warn_allow_apply",
            Self::RequireCompareFirstReview => "require_compare_first_review",
            Self::RequireTypedConfirmation => "require_typed_confirmation",
            Self::RefuseRewrite => "refuse_rewrite",
        }
    }
}

/// Warning reason shown for a non-lossless round-trip risk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundTripWarningClass {
    /// Unknown fields or extension namespaces may be lost.
    UnknownFieldLoss,
    /// Ordering may drift.
    OrderingDrift,
    /// Comments or formatting may be dropped.
    CommentLoss,
    /// Output representation may be normalized or downgraded.
    OutputRepresentationLoss,
    /// The rewrite is lossy.
    LossyRoundTrip,
    /// The round trip is unavailable.
    RoundTripUnavailable,
    /// Policy blocks the write.
    PolicyBlocked,
    /// The preview is bounded because the result is oversized.
    OversizedSafePreview,
}

impl RoundTripWarningClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnknownFieldLoss => "unknown_field_loss",
            Self::OrderingDrift => "ordering_drift",
            Self::CommentLoss => "comment_loss",
            Self::OutputRepresentationLoss => "output_representation_loss",
            Self::LossyRoundTrip => "lossy_round_trip",
            Self::RoundTripUnavailable => "round_trip_unavailable",
            Self::PolicyBlocked => "policy_blocked",
            Self::OversizedSafePreview => "oversized_safe_preview",
        }
    }
}

/// One round-trip risk warning row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundTripRiskRow {
    /// Stable risk id.
    pub risk_id: String,
    /// Subject ref being previewed.
    pub subject_ref: String,
    /// Round-trip risk class.
    pub risk_class: RoundTripRiskClass,
    /// Stable token for [`Self::risk_class`].
    pub risk_class_token: String,
    /// Preview representation class.
    pub preview_representation_class: PreviewRepresentationClass,
    /// Stable token for [`Self::preview_representation_class`].
    pub preview_representation_class_token: String,
    /// Apply gate class.
    pub apply_gate_class: ApplyGateClass,
    /// Stable token for [`Self::apply_gate_class`].
    pub apply_gate_class_token: String,
    /// Visible warning classes.
    pub warning_classes: Vec<RoundTripWarningClass>,
    /// Stable tokens for [`Self::warning_classes`].
    pub warning_class_tokens: Vec<String>,
    /// Affected field paths for lossy or structural changes.
    pub affected_field_paths: Vec<String>,
    /// Raw preservation ref when required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub raw_preservation_ref: Option<String>,
    /// Compare view ref when compare-first is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_view_ref: Option<String>,
    /// Recovery posture visible before apply.
    pub recovery_hint_label: String,
}

impl RoundTripRiskRow {
    fn refresh_tokens(&mut self) {
        self.risk_class_token = self.risk_class.as_str().to_owned();
        self.preview_representation_class_token =
            self.preview_representation_class.as_str().to_owned();
        self.apply_gate_class_token = self.apply_gate_class.as_str().to_owned();
        self.warning_class_tokens = self
            .warning_classes
            .iter()
            .map(|warning| warning.as_str().to_owned())
            .collect();
    }
}

/// Flow class for a reversible or attributable preview action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepairFlowClass {
    /// Rerun or reexecute a runtime-backed cell/action.
    Rerun,
    /// Repair a structured artifact or notebook payload.
    Repair,
    /// Raise document, runtime, or output trust.
    TrustRaise,
    /// Clear notebook or preview output.
    OutputClear,
    /// Apply a structured config edit.
    StructuredApply,
    /// Render deeper after an explicit review.
    DeeperRender,
}

impl RepairFlowClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rerun => "rerun",
            Self::Repair => "repair",
            Self::TrustRaise => "trust_raise",
            Self::OutputClear => "output_clear",
            Self::StructuredApply => "structured_apply",
            Self::DeeperRender => "deeper_render",
        }
    }

    const fn requires_lineage(self) -> bool {
        matches!(
            self,
            Self::Rerun
                | Self::Repair
                | Self::TrustRaise
                | Self::OutputClear
                | Self::StructuredApply
        )
    }
}

/// Mutation posture for a repair-lineage row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MutationActionPosture {
    /// The row previews only and does not apply.
    PreviewOnly,
    /// The row may mutate after review.
    MutatesAfterReview,
    /// The row observes only and writes nothing.
    ObserveOnly,
    /// The row refuses the mutation.
    Refused,
}

impl MutationActionPosture {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewOnly => "preview_only",
            Self::MutatesAfterReview => "mutates_after_review",
            Self::ObserveOnly => "observe_only",
            Self::Refused => "refused",
        }
    }
}

/// Reversal posture for a repair-lineage row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewReversalClass {
    /// Reversal can restore exact prior state.
    Exact,
    /// Reversal uses a compensating action.
    Compensating,
    /// Reversal regenerates derived state.
    Regenerate,
    /// Reversal requires a manual action.
    Manual,
    /// No durable state changed.
    AuditOnly,
}

impl PreviewReversalClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compensating => "compensating",
            Self::Regenerate => "regenerate",
            Self::Manual => "manual",
            Self::AuditOnly => "audit_only",
        }
    }
}

/// Checkpoint and mutation-journal lineage for one repairable flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepairLineageRow {
    /// Stable lineage id.
    pub lineage_id: String,
    /// Flow class.
    pub flow_class: RepairFlowClass,
    /// Stable token for [`Self::flow_class`].
    pub flow_class_token: String,
    /// Mutation posture.
    pub action_posture: MutationActionPosture,
    /// Stable token for [`Self::action_posture`].
    pub action_posture_token: String,
    /// Checkpoint ref bound to this flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checkpoint_ref: Option<String>,
    /// Mutation-journal ref bound to this flow.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Preview sheet, repair packet, or approval packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_or_repair_packet_ref: Option<String>,
    /// Actor or authority ref for attribution.
    pub actor_ref: String,
    /// Affected cells, outputs, artifacts, or state refs.
    pub affected_refs: Vec<String>,
    /// Reversal posture.
    pub reversal_class: PreviewReversalClass,
    /// Stable token for [`Self::reversal_class`].
    pub reversal_class_token: String,
    /// Reviewer-facing consequence label.
    pub consequence_label: String,
    /// Safe next actions.
    pub safe_next_actions: Vec<String>,
}

impl RepairLineageRow {
    fn refresh_tokens(&mut self) {
        self.flow_class_token = self.flow_class.as_str().to_owned();
        self.action_posture_token = self.action_posture.as_str().to_owned();
        self.reversal_class_token = self.reversal_class.as_str().to_owned();
    }
}

/// Output risk posture for safe preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputRiskClass {
    /// Ordinary output with no extra preview restriction.
    Ordinary,
    /// Active or risky content.
    RiskyActiveContent,
    /// Oversized result or artifact.
    OversizedResult,
    /// Renderer support is unknown.
    UnknownRenderer,
    /// Widget runtime is required.
    WidgetRuntime,
}

impl OutputRiskClass {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::RiskyActiveContent => "risky_active_content",
            Self::OversizedResult => "oversized_result",
            Self::UnknownRenderer => "unknown_renderer",
            Self::WidgetRuntime => "widget_runtime",
        }
    }

    const fn requires_safe_disposition(self) -> bool {
        !matches!(self, Self::Ordinary)
    }
}

/// Safe preview disposition for a risky or oversized output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeOutputDisposition {
    /// Full render is allowed.
    FullRenderAllowed,
    /// Safe preview is the default.
    SafePreviewDefault,
    /// Deeper render requires an explicit action.
    DeepRenderExplicit,
    /// Rerun is required for a deeper result.
    RerunExplicit,
    /// Policy blocks the output.
    BlockedByPolicy,
}

impl SafeOutputDisposition {
    /// Returns the stable token used in records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRenderAllowed => "full_render_allowed",
            Self::SafePreviewDefault => "safe_preview_default",
            Self::DeepRenderExplicit => "deep_render_explicit",
            Self::RerunExplicit => "rerun_explicit",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }
}

/// One safe-output row for risky or oversized results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SafeOutputRow {
    /// Output or projection ref.
    pub output_ref: String,
    /// Output risk class.
    pub risk_class: OutputRiskClass,
    /// Stable token for [`Self::risk_class`].
    pub risk_class_token: String,
    /// Safe preview disposition.
    pub disposition: SafeOutputDisposition,
    /// Stable token for [`Self::disposition`].
    pub disposition_token: String,
    /// Output trust state token.
    pub trust_state_token: String,
    /// Bounded safe-preview ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preview_ref: Option<String>,
    /// Explicit deeper-render action ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deeper_render_action_ref: Option<String>,
    /// Explicit rerun action ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rerun_action_ref: Option<String>,
    /// Policy ref when blocked by policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_ref: Option<String>,
    /// Reviewer-facing summary.
    pub summary_label: String,
}

impl SafeOutputRow {
    fn refresh_tokens(&mut self) {
        self.risk_class_token = self.risk_class.as_str().to_owned();
        self.disposition_token = self.disposition.as_str().to_owned();
    }
}

/// Preview-truth record for one retained preview wedge.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewTruthRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable preview id.
    pub preview_id: String,
    /// Surface class.
    pub surface_class: PreviewSurfaceClass,
    /// Stable token for [`Self::surface_class`].
    pub surface_class_token: String,
    /// Workspace or workset ref.
    pub workspace_ref: String,
    /// Root/path/save-target disclosure for the retained preview object.
    pub identity_disclosure: DocumentIdentityDisclosure,
    /// Claim manifest for preview or limited posture.
    pub claim_manifest: PreviewClaimManifest,
    /// Document/runtime/output trust layer rows.
    pub trust_layers: Vec<TrustLayerRow>,
    /// Authored/effective/live structured-state rows.
    pub structured_state_views: Vec<StructuredStateViewRow>,
    /// Round-trip risk rows.
    pub round_trip_risks: Vec<RoundTripRiskRow>,
    /// Repair lineage rows.
    pub repair_lineage: Vec<RepairLineageRow>,
    /// Safe output rows for risky or oversized results.
    pub safe_outputs: Vec<SafeOutputRow>,
    /// Support/export packet refs that include this preview truth.
    pub support_export_scope_refs: Vec<String>,
}

impl PreviewTruthRecord {
    /// Normalizes derived token fields after fixture deserialization.
    pub fn normalized(mut self) -> Self {
        self.record_kind = PREVIEW_TRUTH_RECORD_KIND.to_owned();
        self.schema_version = PREVIEW_TRUTH_SCHEMA_VERSION;
        self.surface_class_token = self.surface_class.as_str().to_owned();
        self.identity_disclosure = self.identity_disclosure.clone().normalized();
        self.claim_manifest.refresh_tokens();
        for layer in &mut self.trust_layers {
            layer.refresh_tokens();
        }
        for view in &mut self.structured_state_views {
            view.refresh_tokens();
        }
        for risk in &mut self.round_trip_risks {
            risk.refresh_tokens();
        }
        for lineage in &mut self.repair_lineage {
            lineage.refresh_tokens();
        }
        for output in &mut self.safe_outputs {
            output.refresh_tokens();
        }
        self
    }

    /// Validates the preview-truth record against retained-wedge invariants.
    pub fn validate(&self) -> Vec<PreviewTruthViolation> {
        let mut out = Vec::new();

        if self.record_kind != PREVIEW_TRUTH_RECORD_KIND {
            out.push(PreviewTruthViolation::UnexpectedRecordKind {
                record_kind: self.record_kind.clone(),
            });
        }
        if self.schema_version != PREVIEW_TRUTH_SCHEMA_VERSION {
            out.push(PreviewTruthViolation::UnexpectedSchemaVersion {
                schema_version: self.schema_version,
            });
        }

        self.validate_claim_manifest(&mut out);
        self.validate_identity_disclosure(&mut out);
        self.validate_trust_layers(&mut out);
        self.validate_structured_views(&mut out);
        self.validate_round_trip_risks(&mut out);
        self.validate_repair_lineage(&mut out);
        self.validate_safe_outputs(&mut out);

        if self.support_export_scope_refs.is_empty() {
            out.push(PreviewTruthViolation::SupportScopeMissing);
        }

        out
    }

    /// Returns true when validation finds no violations.
    pub fn is_valid(&self) -> bool {
        self.validate().is_empty()
    }

    /// Projects this record into an export-safe support packet.
    pub fn support_export(&self) -> PreviewTruthSupportExportRecord {
        let violations = self.validate();
        PreviewTruthSupportExportRecord {
            record_kind: PREVIEW_TRUTH_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PREVIEW_TRUTH_SCHEMA_VERSION,
            source_preview_id: self.preview_id.clone(),
            surface_class_token: self.surface_class.as_str().to_owned(),
            qualification_token: self.claim_manifest.qualification_class.as_str().to_owned(),
            identity_tokens: self.identity_disclosure.identity_tokens(),
            trust_layer_tokens: self
                .trust_layers
                .iter()
                .map(|layer| {
                    format!(
                        "{}:{}:{}",
                        layer.layer_class.as_str(),
                        layer.trust_state_token,
                        layer.gate_state.as_str()
                    )
                })
                .collect(),
            round_trip_risk_tokens: self
                .round_trip_risks
                .iter()
                .map(|risk| {
                    format!(
                        "{}:{}",
                        risk.risk_class.as_str(),
                        risk.apply_gate_class.as_str()
                    )
                })
                .collect(),
            repair_lineage_refs: self
                .repair_lineage
                .iter()
                .map(|lineage| {
                    format!(
                        "{}:{}:{}",
                        lineage.flow_class.as_str(),
                        lineage
                            .checkpoint_ref
                            .as_deref()
                            .unwrap_or("checkpoint_missing"),
                        lineage
                            .mutation_journal_ref
                            .as_deref()
                            .unwrap_or("journal_missing")
                    )
                })
                .collect(),
            safe_output_tokens: self
                .safe_outputs
                .iter()
                .map(|output| {
                    format!(
                        "{}:{}",
                        output.risk_class.as_str(),
                        output.disposition.as_str()
                    )
                })
                .collect(),
            support_export_scope_refs: self.support_export_scope_refs.clone(),
            violation_tokens: violations
                .iter()
                .map(|violation| violation.token().to_owned())
                .collect(),
            plaintext_summary: self.render_plaintext(),
        }
    }

    /// Renders a deterministic support/export summary without artifact bodies.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {} ({})\n",
            self.surface_class.as_str(),
            self.claim_manifest.surface_label,
            self.claim_manifest.visible_qualification_label
        ));
        out.push_str(&format!(
            "preview={} workspace={} qualification={}\n",
            self.preview_id,
            self.workspace_ref,
            self.claim_manifest.qualification_class.as_str()
        ));
        out.push_str("identity:\n");
        for line in self.identity_disclosure.render_plaintext_lines() {
            out.push_str(&format!("  - {line}\n"));
        }
        out.push_str("trust_layers:\n");
        for layer in &self.trust_layers {
            out.push_str(&format!(
                "  - {} trust={} gate={} actions=[{}] reason={}\n",
                layer.layer_class.as_str(),
                layer.trust_state_token,
                layer.gate_state.as_str(),
                layer.action_refs.join(","),
                layer.gate_explanation_label
            ));
        }
        out.push_str("structured_views:\n");
        for view in &self.structured_state_views {
            out.push_str(&format!(
                "  - {} source={} write={} resolution={} freshness={}\n",
                view.view_class.as_str(),
                view.source_ref,
                view.write_eligibility.as_str(),
                view.resolution_time_label,
                view.freshness_label
            ));
        }
        out.push_str("round_trip_risks:\n");
        for risk in &self.round_trip_risks {
            out.push_str(&format!(
                "  - {} risk={} gate={} warnings=[{}] raw={} compare={}\n",
                risk.risk_id,
                risk.risk_class.as_str(),
                risk.apply_gate_class.as_str(),
                risk.warning_class_tokens.join(","),
                risk.raw_preservation_ref.as_deref().unwrap_or("(none)"),
                risk.compare_view_ref.as_deref().unwrap_or("(none)")
            ));
        }
        out.push_str("repair_lineage:\n");
        for lineage in &self.repair_lineage {
            out.push_str(&format!(
                "  - {} flow={} posture={} checkpoint={} journal={} packet={} reversal={}\n",
                lineage.lineage_id,
                lineage.flow_class.as_str(),
                lineage.action_posture.as_str(),
                lineage.checkpoint_ref.as_deref().unwrap_or("(none)"),
                lineage.mutation_journal_ref.as_deref().unwrap_or("(none)"),
                lineage
                    .preview_or_repair_packet_ref
                    .as_deref()
                    .unwrap_or("(none)"),
                lineage.reversal_class.as_str()
            ));
        }
        out.push_str("safe_outputs:\n");
        for output in &self.safe_outputs {
            out.push_str(&format!(
                "  - {} risk={} disposition={} trust={} preview={} deeper={} rerun={}\n",
                output.output_ref,
                output.risk_class.as_str(),
                output.disposition.as_str(),
                output.trust_state_token,
                output.preview_ref.as_deref().unwrap_or("(none)"),
                output
                    .deeper_render_action_ref
                    .as_deref()
                    .unwrap_or("(none)"),
                output.rerun_action_ref.as_deref().unwrap_or("(none)")
            ));
        }
        out.push_str("claim_limits:\n");
        for limit in &self.claim_manifest.claim_limits {
            out.push_str(&format!("  - {}: {}\n", limit.token, limit.label));
        }
        out
    }

    fn validate_claim_manifest(&self, out: &mut Vec<PreviewTruthViolation>) {
        if self.claim_manifest.surface_label.trim().is_empty()
            || self
                .claim_manifest
                .visible_qualification_label
                .trim()
                .is_empty()
            || self.claim_manifest.claim_limits.is_empty()
        {
            out.push(PreviewTruthViolation::ClaimManifestIncomplete);
        }

        if self.claim_manifest.qualification_class.is_stable() {
            if self
                .claim_manifest
                .qualification_evidence_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                out.push(PreviewTruthViolation::StableQualificationMissingEvidence);
            }
        } else {
            let label = self
                .claim_manifest
                .visible_qualification_label
                .to_ascii_lowercase();
            if !label.contains("preview") && !label.contains("limited") {
                out.push(PreviewTruthViolation::PreviewOrLimitedLabelMissing);
            }
        }
    }

    fn validate_identity_disclosure(&self, out: &mut Vec<PreviewTruthViolation>) {
        for field in self.identity_disclosure.missing_fields() {
            out.push(PreviewTruthViolation::IdentityDisclosureIncomplete {
                field: field.to_owned(),
            });
        }

        let expected_family = match self.surface_class {
            PreviewSurfaceClass::NotebookPreview => DocumentFamilyClass::NotebookPreviewOutput,
            PreviewSurfaceClass::StructuredConfigPreview => {
                DocumentFamilyClass::StructuredConfigPreview
            }
        };
        if self.identity_disclosure.document_family != expected_family {
            out.push(PreviewTruthViolation::IdentityDisclosureFamilyMismatch {
                family: self.identity_disclosure.document_family.as_str().to_owned(),
            });
        }
    }

    fn validate_trust_layers(&self, out: &mut Vec<PreviewTruthViolation>) {
        let present: BTreeSet<_> = self
            .trust_layers
            .iter()
            .map(|layer| layer.layer_class)
            .collect();
        for required in [
            TrustLayerClass::Document,
            TrustLayerClass::Runtime,
            TrustLayerClass::Output,
        ] {
            if !present.contains(&required) {
                out.push(PreviewTruthViolation::TrustLayerMissing {
                    layer: required.as_str().to_owned(),
                });
            }
        }

        for layer in &self.trust_layers {
            if layer.trust_state_token.trim().is_empty()
                || layer.action_refs.is_empty()
                || layer.gate_explanation_label.trim().is_empty()
            {
                out.push(PreviewTruthViolation::TrustLayerIncomplete {
                    layer: layer.layer_class.as_str().to_owned(),
                });
            }
        }
    }

    fn validate_structured_views(&self, out: &mut Vec<PreviewTruthViolation>) {
        if self.surface_class == PreviewSurfaceClass::StructuredConfigPreview {
            let present: BTreeSet<_> = self
                .structured_state_views
                .iter()
                .map(|view| view.view_class)
                .collect();
            for required in [
                StructuredViewClass::AuthoredSource,
                StructuredViewClass::EffectiveProjection,
                StructuredViewClass::LiveObservedState,
            ] {
                if !present.contains(&required) {
                    out.push(PreviewTruthViolation::StructuredViewMissing {
                        view: required.as_str().to_owned(),
                    });
                }
            }
        }

        for view in &self.structured_state_views {
            if view.view_id.trim().is_empty()
                || view.source_ref.trim().is_empty()
                || view.resolution_time_label.trim().is_empty()
                || view.value_posture_label.trim().is_empty()
            {
                out.push(PreviewTruthViolation::StructuredViewIncomplete {
                    view: view.view_class.as_str().to_owned(),
                });
            }
            if view.view_class != StructuredViewClass::AuthoredSource
                && view.write_eligibility.can_write()
            {
                out.push(PreviewTruthViolation::ProjectionClaimsWritable {
                    view: view.view_class.as_str().to_owned(),
                });
            }
        }
    }

    fn validate_round_trip_risks(&self, out: &mut Vec<PreviewTruthViolation>) {
        if self.round_trip_risks.is_empty() {
            out.push(PreviewTruthViolation::RoundTripRiskMissing);
        }

        for risk in &self.round_trip_risks {
            if risk.risk_id.trim().is_empty()
                || risk.subject_ref.trim().is_empty()
                || risk.recovery_hint_label.trim().is_empty()
            {
                out.push(PreviewTruthViolation::RoundTripRiskIncomplete {
                    risk_id: risk.risk_id.clone(),
                });
            }
            if !risk.risk_class.is_lossless() {
                if risk.warning_classes.is_empty() {
                    out.push(PreviewTruthViolation::RiskWarningMissing {
                        risk_id: risk.risk_id.clone(),
                    });
                }
                if risk.apply_gate_class == ApplyGateClass::AllowApply {
                    out.push(PreviewTruthViolation::RiskGateTooPermissive {
                        risk_id: risk.risk_id.clone(),
                    });
                }
            }
            if risk.risk_class.needs_raw_preservation()
                && risk.apply_gate_class != ApplyGateClass::RefuseRewrite
                && risk
                    .raw_preservation_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
            {
                out.push(PreviewTruthViolation::RawPreservationMissing {
                    risk_id: risk.risk_id.clone(),
                });
            }
            if risk.apply_gate_class == ApplyGateClass::RequireCompareFirstReview
                && risk
                    .compare_view_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
            {
                out.push(PreviewTruthViolation::CompareViewMissing {
                    risk_id: risk.risk_id.clone(),
                });
            }
            if risk.risk_class == RoundTripRiskClass::LossyStructural
                && risk.affected_field_paths.is_empty()
            {
                out.push(PreviewTruthViolation::AffectedFieldsMissing {
                    risk_id: risk.risk_id.clone(),
                });
            }
        }
    }

    fn validate_repair_lineage(&self, out: &mut Vec<PreviewTruthViolation>) {
        if self.repair_lineage.is_empty() {
            out.push(PreviewTruthViolation::RepairLineageMissing);
        }

        for lineage in &self.repair_lineage {
            if lineage.lineage_id.trim().is_empty() {
                out.push(PreviewTruthViolation::RepairLineageIncomplete {
                    lineage_id: lineage.lineage_id.clone(),
                    missing_field: "lineage_id".to_owned(),
                });
            }
            if lineage.flow_class.requires_lineage() {
                if lineage
                    .checkpoint_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
                {
                    out.push(PreviewTruthViolation::RepairLineageIncomplete {
                        lineage_id: lineage.lineage_id.clone(),
                        missing_field: "checkpoint_ref".to_owned(),
                    });
                }
                if lineage
                    .mutation_journal_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
                {
                    out.push(PreviewTruthViolation::RepairLineageIncomplete {
                        lineage_id: lineage.lineage_id.clone(),
                        missing_field: "mutation_journal_ref".to_owned(),
                    });
                }
                if lineage
                    .preview_or_repair_packet_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
                {
                    out.push(PreviewTruthViolation::RepairLineageIncomplete {
                        lineage_id: lineage.lineage_id.clone(),
                        missing_field: "preview_or_repair_packet_ref".to_owned(),
                    });
                }
            }
            if lineage.actor_ref.trim().is_empty()
                || lineage.affected_refs.is_empty()
                || lineage.consequence_label.trim().is_empty()
                || lineage.safe_next_actions.is_empty()
            {
                out.push(PreviewTruthViolation::RepairConsequenceMissing {
                    lineage_id: lineage.lineage_id.clone(),
                });
            }
        }
    }

    fn validate_safe_outputs(&self, out: &mut Vec<PreviewTruthViolation>) {
        for output in &self.safe_outputs {
            if output.risk_class.requires_safe_disposition()
                && output.disposition == SafeOutputDisposition::FullRenderAllowed
            {
                out.push(PreviewTruthViolation::RiskyOutputNotSafePreviewed {
                    output_ref: output.output_ref.clone(),
                });
            }
            if output.disposition == SafeOutputDisposition::SafePreviewDefault
                && output
                    .preview_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
            {
                out.push(PreviewTruthViolation::SafeOutputActionMissing {
                    output_ref: output.output_ref.clone(),
                    action: "preview_ref".to_owned(),
                });
            }
            if output.disposition == SafeOutputDisposition::DeepRenderExplicit
                && output
                    .deeper_render_action_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
            {
                out.push(PreviewTruthViolation::SafeOutputActionMissing {
                    output_ref: output.output_ref.clone(),
                    action: "deeper_render_action_ref".to_owned(),
                });
            }
            if output.disposition == SafeOutputDisposition::RerunExplicit
                && output
                    .rerun_action_ref
                    .as_deref()
                    .unwrap_or("")
                    .trim()
                    .is_empty()
            {
                out.push(PreviewTruthViolation::SafeOutputActionMissing {
                    output_ref: output.output_ref.clone(),
                    action: "rerun_action_ref".to_owned(),
                });
            }
        }
    }
}

/// Export-safe support projection for a preview-truth record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreviewTruthSupportExportRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Source preview id.
    pub source_preview_id: String,
    /// Surface class token.
    pub surface_class_token: String,
    /// Qualification token.
    pub qualification_token: String,
    /// Identity tokens preserved for support.
    pub identity_tokens: Vec<String>,
    /// Trust-layer tokens.
    pub trust_layer_tokens: Vec<String>,
    /// Round-trip risk tokens.
    pub round_trip_risk_tokens: Vec<String>,
    /// Repair lineage refs.
    pub repair_lineage_refs: Vec<String>,
    /// Safe-output tokens.
    pub safe_output_tokens: Vec<String>,
    /// Support/export scope refs.
    pub support_export_scope_refs: Vec<String>,
    /// Validation violation tokens, empty for a conforming record.
    pub violation_tokens: Vec<String>,
    /// Deterministic plaintext summary.
    pub plaintext_summary: String,
}

/// Validation issue emitted by [`PreviewTruthRecord::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum PreviewTruthViolation {
    /// Record kind did not match the contract.
    UnexpectedRecordKind { record_kind: String },
    /// Schema version did not match the contract.
    UnexpectedSchemaVersion { schema_version: u32 },
    /// Claim manifest is missing required preview or scope data.
    ClaimManifestIncomplete,
    /// Preview or limited label is missing for a non-stable surface.
    PreviewOrLimitedLabelMissing,
    /// Stable qualification lacks evidence.
    StableQualificationMissingEvidence,
    /// Identity disclosure lacks required path/save-target fields.
    IdentityDisclosureIncomplete { field: String },
    /// Identity disclosure names the wrong surface family.
    IdentityDisclosureFamilyMismatch { family: String },
    /// Required trust layer is missing.
    TrustLayerMissing { layer: String },
    /// Trust layer lacks a state, action, or explanation.
    TrustLayerIncomplete { layer: String },
    /// Required structured view is missing.
    StructuredViewMissing { view: String },
    /// Structured view lacks required identity or summary fields.
    StructuredViewIncomplete { view: String },
    /// Effective or live projection claims it can be written as source.
    ProjectionClaimsWritable { view: String },
    /// No round-trip risk row is present.
    RoundTripRiskMissing,
    /// Round-trip risk row lacks required identity or summary fields.
    RoundTripRiskIncomplete { risk_id: String },
    /// Non-lossless risk lacks a visible warning.
    RiskWarningMissing { risk_id: String },
    /// Non-lossless risk incorrectly allows apply without warning.
    RiskGateTooPermissive { risk_id: String },
    /// Raw preservation is required but missing.
    RawPreservationMissing { risk_id: String },
    /// Compare-first review is required but missing.
    CompareViewMissing { risk_id: String },
    /// Structural loss does not enumerate affected fields.
    AffectedFieldsMissing { risk_id: String },
    /// No repair lineage row is present.
    RepairLineageMissing,
    /// Repair lineage row lacks a required ref.
    RepairLineageIncomplete {
        lineage_id: String,
        missing_field: String,
    },
    /// Repair consequence or safe next action is missing.
    RepairConsequenceMissing { lineage_id: String },
    /// Risky output is allowed to render fully by default.
    RiskyOutputNotSafePreviewed { output_ref: String },
    /// Safe-output row lacks the action required by its disposition.
    SafeOutputActionMissing { output_ref: String, action: String },
    /// Support/export scope refs are missing.
    SupportScopeMissing,
}

impl PreviewTruthViolation {
    /// Returns the stable violation token.
    pub const fn token(&self) -> &'static str {
        match self {
            Self::UnexpectedRecordKind { .. } => "unexpected_record_kind",
            Self::UnexpectedSchemaVersion { .. } => "unexpected_schema_version",
            Self::ClaimManifestIncomplete => "claim_manifest_incomplete",
            Self::PreviewOrLimitedLabelMissing => "preview_or_limited_label_missing",
            Self::StableQualificationMissingEvidence => "stable_qualification_missing_evidence",
            Self::IdentityDisclosureIncomplete { .. } => "identity_disclosure_incomplete",
            Self::IdentityDisclosureFamilyMismatch { .. } => "identity_disclosure_family_mismatch",
            Self::TrustLayerMissing { .. } => "trust_layer_missing",
            Self::TrustLayerIncomplete { .. } => "trust_layer_incomplete",
            Self::StructuredViewMissing { .. } => "structured_view_missing",
            Self::StructuredViewIncomplete { .. } => "structured_view_incomplete",
            Self::ProjectionClaimsWritable { .. } => "projection_claims_writable",
            Self::RoundTripRiskMissing => "round_trip_risk_missing",
            Self::RoundTripRiskIncomplete { .. } => "round_trip_risk_incomplete",
            Self::RiskWarningMissing { .. } => "risk_warning_missing",
            Self::RiskGateTooPermissive { .. } => "risk_gate_too_permissive",
            Self::RawPreservationMissing { .. } => "raw_preservation_missing",
            Self::CompareViewMissing { .. } => "compare_view_missing",
            Self::AffectedFieldsMissing { .. } => "affected_fields_missing",
            Self::RepairLineageMissing => "repair_lineage_missing",
            Self::RepairLineageIncomplete { .. } => "repair_lineage_incomplete",
            Self::RepairConsequenceMissing { .. } => "repair_consequence_missing",
            Self::RiskyOutputNotSafePreviewed { .. } => "risky_output_not_safe_previewed",
            Self::SafeOutputActionMissing { .. } => "safe_output_action_missing",
            Self::SupportScopeMissing => "support_scope_missing",
        }
    }
}

/// Builds a seeded notebook preview-truth record for shell inspectors.
pub fn seeded_notebook_preview_truth(workspace_ref: impl Into<String>) -> PreviewTruthRecord {
    PreviewTruthRecord {
        record_kind: PREVIEW_TRUTH_RECORD_KIND.to_owned(),
        schema_version: PREVIEW_TRUTH_SCHEMA_VERSION,
        preview_id: "preview_truth:notebook:retained_mixed_trust".to_owned(),
        surface_class: PreviewSurfaceClass::NotebookPreview,
        surface_class_token: PreviewSurfaceClass::NotebookPreview.as_str().to_owned(),
        workspace_ref: workspace_ref.into(),
        identity_disclosure: DocumentIdentityDisclosure {
            record_kind: String::new(),
            schema_version: 0,
            document_family: DocumentFamilyClass::NotebookPreviewOutput,
            document_family_token: String::new(),
            root_class: crate::document_identity::RootClassDisclosure::Generated,
            root_class_token: String::new(),
            document_labels: vec![
                crate::document_identity::DocumentLabelClass::GeneratedDocument,
                crate::document_identity::DocumentLabelClass::OverlayProjection,
            ],
            document_label_tokens: Vec::new(),
            presentation_path: "preview://notebook/retained_mixed_trust/output/fit_model_chart"
                .to_owned(),
            logical_identity_ref: "logical:notebook_preview:retained_mixed_trust:fit_model_chart"
                .to_owned(),
            canonical_target: "generated:notebook_preview:retained_mixed_trust:fit_model_chart"
                .to_owned(),
            canonical_target_hint: Some(
                "Preview row is derived from the notebook output lineage, not the canonical notebook file."
                    .to_owned(),
            ),
            alias_status: crate::document_identity::AliasStatusClass::Projection,
            alias_status_token: String::new(),
            alias_status_label:
                "Preview path is a generated projection over captured notebook output."
                    .to_owned(),
            save_target_class: crate::document_identity::SaveTargetClass::GeneratedTarget,
            save_target_class_token: String::new(),
            save_target_label:
                "Current preview is generated; export or rerun before treating it as a durable file."
                    .to_owned(),
            write_posture: crate::document_identity::WritePostureClass::ExportBeforeWrite,
            write_posture_token: String::new(),
            write_posture_label: "Export before durable write".to_owned(),
            backing_source_label: "Captured notebook output".to_owned(),
            freshness_label: "Captured prior-session output".to_owned(),
            docs_help_ref: "help:notebook:preview_identity".to_owned(),
        },
        claim_manifest: PreviewClaimManifest {
            surface_label: "Notebook preview truth".to_owned(),
            qualification_class: PreviewQualificationClass::PreviewLimited,
            qualification_class_token: PreviewQualificationClass::PreviewLimited
                .as_str()
                .to_owned(),
            visible_qualification_label: "Preview-limited notebook depth".to_owned(),
            qualification_evidence_ref: None,
            claim_limits: vec![
                PreviewClaimLimitRow {
                    token: "single_notebook_preview_row".to_owned(),
                    label: "One retained notebook preview row; no full notebook editor claim."
                        .to_owned(),
                },
                PreviewClaimLimitRow {
                    token: "explicit_runtime_rebind".to_owned(),
                    label: "Rerun and deeper render require explicit runtime review.".to_owned(),
                },
            ],
        },
        trust_layers: vec![
            TrustLayerRow {
                layer_class: TrustLayerClass::Document,
                layer_class_token: String::new(),
                trust_state_token: "document_trust_elevated_on_explicit_grant".to_owned(),
                gate_state: TrustGateState::PermitsInspectionOnly,
                gate_state_token: String::new(),
                action_refs: vec!["action:notebook:open_diff".to_owned()],
                gate_explanation_label: "Document structure is trusted for review; execution still depends on runtime trust.".to_owned(),
                evidence_ref: Some("evidence:notebook:user_grant".to_owned()),
            },
            TrustLayerRow {
                layer_class: TrustLayerClass::Runtime,
                layer_class_token: String::new(),
                trust_state_token: "kernel_trust_unavailable_no_kernel".to_owned(),
                gate_state: TrustGateState::BlocksAction,
                gate_state_token: String::new(),
                action_refs: vec!["action:notebook:rerun_all".to_owned()],
                gate_explanation_label: "The prior remote kernel is unavailable; rerun is blocked until the runtime is reconnected or replaced.".to_owned(),
                evidence_ref: Some("execution-context:notebook:remote_kernel_prior".to_owned()),
            },
            TrustLayerRow {
                layer_class: TrustLayerClass::Output,
                layer_class_token: String::new(),
                trust_state_token: "output_trust_captured_from_prior_session".to_owned(),
                gate_state: TrustGateState::PermitsInspectionOnly,
                gate_state_token: String::new(),
                action_refs: vec!["action:notebook:open_safe_preview".to_owned()],
                gate_explanation_label: "Outputs remain captured evidence and cannot be described as live until rerun succeeds.".to_owned(),
                evidence_ref: Some("lineage:notebook:output:prior_session".to_owned()),
            },
        ],
        structured_state_views: Vec::new(),
        round_trip_risks: vec![RoundTripRiskRow {
            risk_id: "risk:notebook:output_representation".to_owned(),
            subject_ref: "notebook:document:retained_mixed_trust".to_owned(),
            risk_class: RoundTripRiskClass::LossyOutputRepresentation,
            risk_class_token: String::new(),
            preview_representation_class: PreviewRepresentationClass::SummaryOnlyPreview,
            preview_representation_class_token: String::new(),
            apply_gate_class: ApplyGateClass::WarnAllowApply,
            apply_gate_class_token: String::new(),
            warning_classes: vec![
                RoundTripWarningClass::OutputRepresentationLoss,
                RoundTripWarningClass::OversizedSafePreview,
            ],
            warning_class_tokens: Vec::new(),
            affected_field_paths: vec!["/cells/*/outputs".to_owned()],
            raw_preservation_ref: Some("raw:notebook:before_output_clear".to_owned()),
            compare_view_ref: Some("compare:notebook:output_scope".to_owned()),
            recovery_hint_label: "Captured output bytes are preserved; clear-output can be reversed by checkpoint or rerun after runtime review.".to_owned(),
        }],
        repair_lineage: vec![
            RepairLineageRow {
                lineage_id: "lineage:notebook:rerun_review".to_owned(),
                flow_class: RepairFlowClass::Rerun,
                flow_class_token: String::new(),
                action_posture: MutationActionPosture::PreviewOnly,
                action_posture_token: String::new(),
                checkpoint_ref: Some("checkpoint:notebook:before_rerun_review".to_owned()),
                mutation_journal_ref: Some("mutation-journal:notebook:rerun_review".to_owned()),
                preview_or_repair_packet_ref: Some("roundtrip-preview:notebook:rerun_review".to_owned()),
                actor_ref: "actor:user:local".to_owned(),
                affected_refs: vec!["notebook:cell:fit_model".to_owned()],
                reversal_class: PreviewReversalClass::Regenerate,
                reversal_class_token: String::new(),
                consequence_label: "Rerun would replace captured outputs only after runtime trust is rechecked.".to_owned(),
                safe_next_actions: vec![
                    "reconnect kernel".to_owned(),
                    "keep captured outputs".to_owned(),
                ],
            },
            RepairLineageRow {
                lineage_id: "lineage:notebook:output_clear".to_owned(),
                flow_class: RepairFlowClass::OutputClear,
                flow_class_token: String::new(),
                action_posture: MutationActionPosture::MutatesAfterReview,
                action_posture_token: String::new(),
                checkpoint_ref: Some("checkpoint:notebook:before_output_clear".to_owned()),
                mutation_journal_ref: Some("mutation-journal:notebook:output_clear".to_owned()),
                preview_or_repair_packet_ref: Some("repair-packet:notebook:output_clear".to_owned()),
                actor_ref: "actor:user:local".to_owned(),
                affected_refs: vec!["notebook:output:chart".to_owned()],
                reversal_class: PreviewReversalClass::Regenerate,
                reversal_class_token: String::new(),
                consequence_label: "Clear outputs removes captured evidence from the notebook and requires rerun for fresh output.".to_owned(),
                safe_next_actions: vec![
                    "open compare before apply".to_owned(),
                    "save clean copy".to_owned(),
                ],
            },
        ],
        safe_outputs: vec![SafeOutputRow {
            output_ref: "notebook:output:chart".to_owned(),
            risk_class: OutputRiskClass::OversizedResult,
            risk_class_token: String::new(),
            disposition: SafeOutputDisposition::SafePreviewDefault,
            disposition_token: String::new(),
            trust_state_token: "output_trust_captured_from_prior_session".to_owned(),
            preview_ref: Some("safe-preview:notebook:chart_window".to_owned()),
            deeper_render_action_ref: Some("action:notebook:render_full_chart".to_owned()),
            rerun_action_ref: Some("action:notebook:rerun_cell".to_owned()),
            policy_ref: None,
            summary_label: "Large captured chart opens as a safe preview; full render is explicit.".to_owned(),
        }],
        support_export_scope_refs: vec!["support:notebook:preview_truth".to_owned()],
    }
    .normalized()
}

/// Builds a seeded structured-config preview-truth record for shell inspectors.
pub fn seeded_structured_config_preview_truth(
    workspace_ref: impl Into<String>,
) -> PreviewTruthRecord {
    PreviewTruthRecord {
        record_kind: PREVIEW_TRUTH_RECORD_KIND.to_owned(),
        schema_version: PREVIEW_TRUTH_SCHEMA_VERSION,
        preview_id: "preview_truth:structured_config:retained_roundtrip".to_owned(),
        surface_class: PreviewSurfaceClass::StructuredConfigPreview,
        surface_class_token: PreviewSurfaceClass::StructuredConfigPreview
            .as_str()
            .to_owned(),
        workspace_ref: workspace_ref.into(),
        identity_disclosure: DocumentIdentityDisclosure {
            record_kind: String::new(),
            schema_version: 0,
            document_family: DocumentFamilyClass::StructuredConfigPreview,
            document_family_token: String::new(),
            root_class: crate::document_identity::RootClassDisclosure::Local,
            root_class_token: String::new(),
            document_labels: vec![
                crate::document_identity::DocumentLabelClass::DurableLocalFile,
                crate::document_identity::DocumentLabelClass::OverlayProjection,
            ],
            document_label_tokens: Vec::new(),
            presentation_path: "workspace://service-config/service.yaml".to_owned(),
            logical_identity_ref: "logical:config_preview:service_yaml".to_owned(),
            canonical_target: "config:source:service_yaml".to_owned(),
            canonical_target_hint: Some(
                "Authored YAML is the canonical target; effective and live rows are overlays."
                    .to_owned(),
            ),
            alias_status: crate::document_identity::AliasStatusClass::Projection,
            alias_status_token: String::new(),
            alias_status_label:
                "Surface combines authored source with effective and live overlay projections."
                    .to_owned(),
            save_target_class: crate::document_identity::SaveTargetClass::LocalFile,
            save_target_class_token: String::new(),
            save_target_label:
                "Durable edits land on the authored YAML source after compare-first review."
                    .to_owned(),
            write_posture: crate::document_identity::WritePostureClass::SaveReviewRequired,
            write_posture_token: String::new(),
            write_posture_label: "Review required before save".to_owned(),
            backing_source_label: "Authored YAML source with effective/live overlays".to_owned(),
            freshness_label: "Authored current; live overlay stale".to_owned(),
            docs_help_ref: "help:config:preview_identity".to_owned(),
        },
        claim_manifest: PreviewClaimManifest {
            surface_label: "Structured config preview truth".to_owned(),
            qualification_class: PreviewQualificationClass::PreviewLimited,
            qualification_class_token: PreviewQualificationClass::PreviewLimited
                .as_str()
                .to_owned(),
            visible_qualification_label: "Preview-limited structured config depth".to_owned(),
            qualification_evidence_ref: None,
            claim_limits: vec![
                PreviewClaimLimitRow {
                    token: "source_effective_live_only".to_owned(),
                    label: "Authored, effective, and live views are labelled; no general config editor claim.".to_owned(),
                },
                PreviewClaimLimitRow {
                    token: "compare_before_lossy_write".to_owned(),
                    label: "Lossy or unknown-field writes require compare-first repair lineage.".to_owned(),
                },
            ],
        },
        trust_layers: vec![
            TrustLayerRow {
                layer_class: TrustLayerClass::Document,
                layer_class_token: String::new(),
                trust_state_token: "source_trusted_workspace_file".to_owned(),
                gate_state: TrustGateState::PermitsAction,
                gate_state_token: String::new(),
                action_refs: vec!["action:config:edit_authored_source".to_owned()],
                gate_explanation_label: "Authored source is editable as the canonical artifact.".to_owned(),
                evidence_ref: Some("vfs:config:authored_source".to_owned()),
            },
            TrustLayerRow {
                layer_class: TrustLayerClass::Runtime,
                layer_class_token: String::new(),
                trust_state_token: "live_connector_policy_limited".to_owned(),
                gate_state: TrustGateState::PermitsInspectionOnly,
                gate_state_token: String::new(),
                action_refs: vec!["action:config:open_live_observed".to_owned()],
                gate_explanation_label: "Live observed state is read-only because the connector cannot prove target freshness.".to_owned(),
                evidence_ref: Some("execution-context:config:live_connector".to_owned()),
            },
            TrustLayerRow {
                layer_class: TrustLayerClass::Output,
                layer_class_token: String::new(),
                trust_state_token: "effective_projection_redacted".to_owned(),
                gate_state: TrustGateState::PermitsInspectionOnly,
                gate_state_token: String::new(),
                action_refs: vec!["action:config:copy_redacted_effective".to_owned()],
                gate_explanation_label: "Effective values are a redacted projection and cannot be written back as source.".to_owned(),
                evidence_ref: Some("projection:config:effective".to_owned()),
            },
        ],
        structured_state_views: vec![
            StructuredStateViewRow {
                view_id: "view:config:authored".to_owned(),
                view_class: StructuredViewClass::AuthoredSource,
                view_class_token: String::new(),
                source_ref: "config:source:service_yaml".to_owned(),
                resolution_time_label: "authored".to_owned(),
                target_boundary_ref: "target:local_workspace".to_owned(),
                write_eligibility: StructuredWriteEligibility::EditableCanonicalSource,
                write_eligibility_token: String::new(),
                value_posture_label: "Canonical YAML source with comments and unknown fields.".to_owned(),
                freshness_label: "current buffer".to_owned(),
            },
            StructuredStateViewRow {
                view_id: "view:config:effective".to_owned(),
                view_class: StructuredViewClass::EffectiveProjection,
                view_class_token: String::new(),
                source_ref: "config:effective:service_yaml".to_owned(),
                resolution_time_label: "at run".to_owned(),
                target_boundary_ref: "target:local_workspace".to_owned(),
                write_eligibility: StructuredWriteEligibility::ReadOnlyProjection,
                write_eligibility_token: String::new(),
                value_posture_label: "Defaults, env refs, and policy-injected values are redacted or deferred.".to_owned(),
                freshness_label: "computed from current buffer".to_owned(),
            },
            StructuredStateViewRow {
                view_id: "view:config:live".to_owned(),
                view_class: StructuredViewClass::LiveObservedState,
                view_class_token: String::new(),
                source_ref: "config:live:service_yaml".to_owned(),
                resolution_time_label: "observed live".to_owned(),
                target_boundary_ref: "target:managed_runtime:staging".to_owned(),
                write_eligibility: StructuredWriteEligibility::InspectOnly,
                write_eligibility_token: String::new(),
                value_posture_label: "Live state is observed through a policy-limited connector.".to_owned(),
                freshness_label: "snapshot stale after target drift".to_owned(),
            },
        ],
        round_trip_risks: vec![RoundTripRiskRow {
            risk_id: "risk:config:unknown_fields_ordering".to_owned(),
            subject_ref: "config:source:service_yaml".to_owned(),
            risk_class: RoundTripRiskClass::LossyStructural,
            risk_class_token: String::new(),
            preview_representation_class: PreviewRepresentationClass::NormalisedPreview,
            preview_representation_class_token: String::new(),
            apply_gate_class: ApplyGateClass::RequireCompareFirstReview,
            apply_gate_class_token: String::new(),
            warning_classes: vec![
                RoundTripWarningClass::UnknownFieldLoss,
                RoundTripWarningClass::OrderingDrift,
                RoundTripWarningClass::CommentLoss,
            ],
            warning_class_tokens: Vec::new(),
            affected_field_paths: vec![
                "/x-vendor".to_owned(),
                "/services/0/env".to_owned(),
                "/#comments".to_owned(),
            ],
            raw_preservation_ref: Some("raw:config:service_yaml:before_structured_apply".to_owned()),
            compare_view_ref: Some("compare:config:service_yaml:structured_apply".to_owned()),
            recovery_hint_label: "Raw YAML is preserved before apply; compare-first shows unknown field and ordering drift.".to_owned(),
        }],
        repair_lineage: vec![RepairLineageRow {
            lineage_id: "lineage:config:structured_apply".to_owned(),
            flow_class: RepairFlowClass::StructuredApply,
            flow_class_token: String::new(),
            action_posture: MutationActionPosture::MutatesAfterReview,
            action_posture_token: String::new(),
            checkpoint_ref: Some("checkpoint:config:before_structured_apply".to_owned()),
            mutation_journal_ref: Some("mutation-journal:config:structured_apply".to_owned()),
            preview_or_repair_packet_ref: Some("roundtrip-preview:config:unknown_fields_ordering".to_owned()),
            actor_ref: "actor:user:local".to_owned(),
            affected_refs: vec!["config:source:service_yaml".to_owned()],
            reversal_class: PreviewReversalClass::Exact,
            reversal_class_token: String::new(),
            consequence_label: "Structured apply can rewrite unknown fields and ordering; checkpoint restore is available.".to_owned(),
            safe_next_actions: vec![
                "open compare before apply".to_owned(),
                "continue source-only editing".to_owned(),
            ],
        }],
        safe_outputs: vec![SafeOutputRow {
            output_ref: "config:effective:service_yaml".to_owned(),
            risk_class: OutputRiskClass::OversizedResult,
            risk_class_token: String::new(),
            disposition: SafeOutputDisposition::SafePreviewDefault,
            disposition_token: String::new(),
            trust_state_token: "effective_projection_redacted".to_owned(),
            preview_ref: Some("safe-preview:config:effective_summary".to_owned()),
            deeper_render_action_ref: Some("action:config:open_full_effective_review".to_owned()),
            rerun_action_ref: None,
            policy_ref: None,
            summary_label: "Large effective projection opens as a redacted safe preview.".to_owned(),
        }],
        support_export_scope_refs: vec!["support:config:preview_truth".to_owned()],
    }
    .normalized()
}

#[cfg(test)]
mod tests;
