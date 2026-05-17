//! Beta support packet for composer, context, evidence, and spend truth.
//!
//! The packet does not replace the detailed composer, context-inspector,
//! evidence, retrieval, or spend records. It binds their refs and operator
//! truth rows into one export-safe support projection so claimed AI rows can
//! be blocked when context state, retrieval state, evidence lineage, spend
//! receipts, docs, or UI projections drift.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::context_inspector::{
    AiContextRetrievalExport, ComposerContextAlphaSnapshot, ComposerContextItem,
    ContextItemStateClass,
};
use crate::evidence::{AiMutationEvidencePacket, MutationEvidenceState};
use crate::routing_policy::SpendReceiptRecord;

/// Stable record-kind tag carried by [`ComposerContextEvidenceBetaPacket`].
pub const COMPOSER_CONTEXT_EVIDENCE_BETA_PACKET_RECORD_KIND: &str =
    "composer_context_evidence_beta_packet";

/// Schema version for composer/context/evidence beta support packets.
pub const COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the beta boundary schema.
pub const COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_REF: &str =
    "schemas/ai/composer_context_evidence_beta.schema.json";

/// Repo-relative path of the AI reviewer contract.
pub const COMPOSER_CONTEXT_EVIDENCE_BETA_AI_DOC_REF: &str =
    "docs/ai/m3/composer_context_evidence_beta.md";

/// Repo-relative path of the UX reviewer contract.
pub const COMPOSER_CONTEXT_EVIDENCE_BETA_UX_DOC_REF: &str = "docs/ux/m3/ai_composer_beta.md";

/// Repo-relative path of the protected fixture corpus.
pub const COMPOSER_CONTEXT_EVIDENCE_BETA_FIXTURE_DIR: &str =
    "fixtures/ai/m3/composer_context_evidence";

/// Repo-relative path of the checked-in support export.
pub const COMPOSER_CONTEXT_EVIDENCE_BETA_ARTIFACT_REF: &str =
    "artifacts/ai/m3/composer_context_evidence_beta_support_export.json";

const REQUIRED_CONTEXT_STATE_TOKENS: &[&str] =
    &["included", "pinned", "omitted", "stale", "trimmed"];

/// Product/support surface that must preserve the same operator truth refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComposerContextEvidenceSurfaceClass {
    /// Prompt composer or pre-send sheet.
    Composer,
    /// AI context inspector.
    ContextInspector,
    /// Review workspace or evidence panel.
    ReviewWorkspace,
    /// Documentation/help projection.
    DocsHelp,
    /// Support export or issue-report projection.
    SupportExport,
    /// CLI or headless audit projection.
    Cli,
}

impl ComposerContextEvidenceSurfaceClass {
    /// Stable token used in support exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Composer => "composer",
            Self::ContextInspector => "context_inspector",
            Self::ReviewWorkspace => "review_workspace",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::Cli => "cli",
        }
    }
}

/// One surface's proof that it reads the same context/evidence/spend refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerContextEvidenceSurfaceRow {
    /// Surface class.
    pub surface_class: ComposerContextEvidenceSurfaceClass,
    /// Stable projection ref for this surface.
    pub projection_ref: String,
    /// Context snapshot ref consumed by the surface.
    pub composer_context_snapshot_ref: String,
    /// Evidence packet ref consumed by the surface.
    pub evidence_packet_ref: String,
    /// Route receipt ref consumed by the surface.
    pub route_receipt_ref: String,
    /// Spend receipt ref consumed by the surface.
    pub spend_receipt_ref: String,
    /// Context-state tokens rendered by the surface.
    pub context_state_tokens: Vec<String>,
    /// True when the surface keeps the same refs instead of reminting truth.
    pub preserves_operator_truth: bool,
    /// True when the projection excludes raw prompt, file, provider, or credential bodies.
    pub raw_private_material_excluded: bool,
    /// True when the surface can reach the JSON export.
    pub supports_json_export: bool,
    /// True when the surface can reach the Markdown summary.
    pub supports_markdown_summary: bool,
}

/// Context row copied into the beta support packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerContextEvidenceContextRow {
    /// Context item id.
    pub context_item_id: String,
    /// Context group token.
    pub group_token: String,
    /// Context state token.
    pub state_token: String,
    /// Source class token.
    pub source_class_token: String,
    /// Stable identity ref.
    pub stable_identity_ref: String,
    /// Freshness token.
    pub freshness_token: String,
    /// Trust token.
    pub trust_token: String,
    /// Locality token.
    pub locality_token: String,
    /// Omission or trimming reason token.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub omission_reason_token: Option<String>,
    /// Source attachment ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_attachment_ref: Option<String>,
    /// Source mention ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_mention_ref: Option<String>,
}

impl ComposerContextEvidenceContextRow {
    fn from_context_item(item: &ComposerContextItem) -> Self {
        Self {
            context_item_id: item.context_item_id.clone(),
            group_token: item.group_class.as_str().to_owned(),
            state_token: item.state_class.as_str().to_owned(),
            source_class_token: item.source_class.as_str().to_owned(),
            stable_identity_ref: item.stable_identity_ref.clone(),
            freshness_token: item.freshness_class.as_str().to_owned(),
            trust_token: item.trust_class.as_str().to_owned(),
            locality_token: item.locality_class.as_str().to_owned(),
            omission_reason_token: item
                .omission_reason_class
                .map(|reason| reason.as_str().to_owned()),
            source_attachment_ref: item.source_attachment_ref.clone(),
            source_mention_ref: item.source_mention_ref.clone(),
        }
    }
}

/// Constructor input for [`ComposerContextEvidenceBetaPacket::from_runtime_parts`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComposerContextEvidenceBetaInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Surface rows that must preserve the same operator truth refs.
    pub surface_rows: Vec<ComposerContextEvidenceSurfaceRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref surfaced to support and review.
    pub json_export_ref: String,
    /// Markdown summary ref surfaced to support and review.
    pub markdown_summary_ref: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe beta packet joining composer, context, evidence, and spend truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComposerContextEvidenceBetaPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Composer context snapshot ref.
    pub composer_context_snapshot_ref: String,
    /// Composer session ref.
    pub composer_session_ref: String,
    /// Turn draft ref.
    pub turn_draft_ref: String,
    /// Request workspace ref.
    pub request_workspace_ref: String,
    /// Review state token from the context snapshot.
    pub context_review_state_token: String,
    /// Required context state tokens for the beta claim.
    pub required_context_state_tokens: Vec<String>,
    /// Observed context state tokens.
    pub observed_context_state_tokens: Vec<String>,
    /// Context rows visible before approval or execution.
    pub context_rows: Vec<ComposerContextEvidenceContextRow>,
    /// Retrieval packet ref used by AI context.
    pub retrieval_packet_ref: String,
    /// Retrieval promotion state token.
    pub retrieval_promotion_state_token: String,
    /// Retrieval validation finding tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub retrieval_validation_finding_tokens: Vec<String>,
    /// Evidence packet ref.
    pub evidence_packet_ref: String,
    /// Evidence packet state token.
    pub evidence_packet_state_token: String,
    /// Routing packet ref.
    pub routing_packet_ref: String,
    /// Route receipt ref.
    pub route_receipt_ref: String,
    /// Spend receipt ref.
    pub spend_receipt_ref: String,
    /// Selected provider entry ref.
    pub selected_provider_entry_ref: String,
    /// Selected model entry ref.
    pub selected_model_entry_ref: String,
    /// Selected provider label.
    pub selected_provider_label: String,
    /// Selected model label.
    pub selected_model_label: String,
    /// Route-origin token.
    pub route_origin_token: String,
    /// Cost-envelope token.
    pub cost_envelope_token: String,
    /// Cost-visibility token.
    pub cost_visibility_token: String,
    /// Tool-call lineage refs attached to the post-run evidence packet.
    pub tool_call_lineage_refs: Vec<String>,
    /// Approval ticket refs attached to the evidence packet.
    pub approval_ticket_refs: Vec<String>,
    /// Apply outcome token.
    pub apply_outcome_token: String,
    /// Spend receipt run-state token.
    pub spend_receipt_run_state_token: String,
    /// Spend receipt cost-envelope token.
    pub spend_receipt_cost_envelope_token: String,
    /// Spend receipt cost-visibility token.
    pub spend_receipt_cost_visibility_token: String,
    /// Spend receipt charge-locus token.
    pub spend_receipt_charge_locus_token: String,
    /// Surface rows that must agree on operator truth.
    pub surface_rows: Vec<ComposerContextEvidenceSurfaceRow>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// JSON export ref surfaced to support and review.
    pub json_export_ref: String,
    /// Markdown summary ref surfaced to support and review.
    pub markdown_summary_ref: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ComposerContextEvidenceBetaPacket {
    /// Builds a beta packet from the canonical runtime records.
    pub fn from_runtime_parts(
        snapshot: &ComposerContextAlphaSnapshot,
        retrieval_export: &AiContextRetrievalExport,
        evidence_packet: &AiMutationEvidencePacket,
        spend_receipt: &SpendReceiptRecord,
        input: ComposerContextEvidenceBetaInput,
    ) -> Self {
        let observed_context_state_tokens =
            ordered_state_tokens(snapshot.context_items.iter().map(|item| item.state_class));
        Self {
            record_kind: COMPOSER_CONTEXT_EVIDENCE_BETA_PACKET_RECORD_KIND.to_owned(),
            schema_version: COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            display_label: input.display_label,
            composer_context_snapshot_ref: snapshot.review_lock.context_snapshot_ref.clone(),
            composer_session_ref: snapshot.composer_session_id.clone(),
            turn_draft_ref: snapshot.composer_draft_id.clone(),
            request_workspace_ref: snapshot.request_workspace_id.clone(),
            context_review_state_token: snapshot.review_state.as_str().to_owned(),
            required_context_state_tokens: REQUIRED_CONTEXT_STATE_TOKENS
                .iter()
                .map(|token| (*token).to_owned())
                .collect(),
            observed_context_state_tokens,
            context_rows: snapshot
                .context_items
                .iter()
                .map(ComposerContextEvidenceContextRow::from_context_item)
                .collect(),
            retrieval_packet_ref: retrieval_export.retrieval_packet.packet_id.clone(),
            retrieval_promotion_state_token: retrieval_export
                .retrieval_packet
                .promotion_state
                .as_str()
                .to_owned(),
            retrieval_validation_finding_tokens: retrieval_export
                .retrieval_findings()
                .into_iter()
                .map(|finding| finding.finding_kind.as_str().to_owned())
                .collect(),
            evidence_packet_ref: evidence_packet.evidence_packet_id.clone(),
            evidence_packet_state_token: evidence_packet.packet_state.as_str().to_owned(),
            routing_packet_ref: evidence_packet
                .route_spend_lineage
                .routing_packet_ref
                .clone(),
            route_receipt_ref: evidence_packet
                .route_spend_lineage
                .route_receipt_ref
                .clone(),
            spend_receipt_ref: evidence_packet
                .route_spend_lineage
                .spend_receipt_ref
                .clone(),
            selected_provider_entry_ref: evidence_packet
                .route_spend_lineage
                .selected_provider_entry_ref
                .clone(),
            selected_model_entry_ref: evidence_packet
                .route_spend_lineage
                .selected_model_entry_ref
                .clone(),
            selected_provider_label: evidence_packet
                .route_spend_lineage
                .selected_provider_label
                .clone(),
            selected_model_label: evidence_packet
                .route_spend_lineage
                .selected_model_label
                .clone(),
            route_origin_token: evidence_packet
                .route_spend_lineage
                .route_origin_token
                .clone(),
            cost_envelope_token: evidence_packet
                .route_spend_lineage
                .cost_envelope_token
                .clone(),
            cost_visibility_token: evidence_packet
                .route_spend_lineage
                .cost_visibility_token
                .clone(),
            tool_call_lineage_refs: evidence_packet.tool_call_lineage_refs.clone(),
            approval_ticket_refs: evidence_packet
                .approval_lineage
                .iter()
                .map(|approval| approval.approval_ticket_ref.clone())
                .collect(),
            apply_outcome_token: evidence_packet
                .review_lineage
                .apply_outcome_class
                .as_str()
                .to_owned(),
            spend_receipt_run_state_token: spend_receipt.run_state_class.as_str().to_owned(),
            spend_receipt_cost_envelope_token: spend_receipt
                .cost_envelope_class
                .as_str()
                .to_owned(),
            spend_receipt_cost_visibility_token: spend_receipt
                .cost_visibility_class
                .as_str()
                .to_owned(),
            spend_receipt_charge_locus_token: spend_receipt
                .was_charged_to_user_class
                .as_str()
                .to_owned(),
            surface_rows: input.surface_rows,
            source_contract_refs: input.source_contract_refs,
            json_export_ref: input.json_export_ref,
            markdown_summary_ref: input.markdown_summary_ref,
            minted_at: input.minted_at,
        }
    }

    /// Validates beta promotion truth without resolving raw bodies.
    pub fn validate(&self) -> Vec<ComposerContextEvidenceBetaViolation> {
        let mut violations = Vec::new();
        if self.record_kind != COMPOSER_CONTEXT_EVIDENCE_BETA_PACKET_RECORD_KIND {
            violations.push(ComposerContextEvidenceBetaViolation::WrongRecordKind);
        }
        if self.schema_version != COMPOSER_CONTEXT_EVIDENCE_BETA_SCHEMA_VERSION {
            violations.push(ComposerContextEvidenceBetaViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.composer_context_snapshot_ref.trim().is_empty()
            || self.composer_session_ref.trim().is_empty()
            || self.turn_draft_ref.trim().is_empty()
            || self.request_workspace_ref.trim().is_empty()
            || self.evidence_packet_ref.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ComposerContextEvidenceBetaViolation::MissingIdentity);
        }
        if self.source_contract_refs.is_empty()
            || !self
                .source_contract_refs
                .iter()
                .any(|reference| reference == COMPOSER_CONTEXT_EVIDENCE_BETA_AI_DOC_REF)
            || !self
                .source_contract_refs
                .iter()
                .any(|reference| reference == COMPOSER_CONTEXT_EVIDENCE_BETA_UX_DOC_REF)
        {
            violations.push(ComposerContextEvidenceBetaViolation::MissingSourceContracts);
        }

        for required in &self.required_context_state_tokens {
            if !self.observed_context_state_tokens.contains(required) {
                violations.push(ComposerContextEvidenceBetaViolation::MissingContextStateCoverage);
                break;
            }
        }
        if self.context_rows.is_empty() {
            violations.push(ComposerContextEvidenceBetaViolation::MissingContextRows);
        }
        for row in &self.context_rows {
            if row.context_item_id.trim().is_empty() || row.stable_identity_ref.trim().is_empty() {
                violations.push(ComposerContextEvidenceBetaViolation::MissingContextRows);
                break;
            }
            if requires_omission_reason(&row.state_token) && row.omission_reason_token.is_none() {
                violations.push(ComposerContextEvidenceBetaViolation::ContextRowMissingReason);
                break;
            }
        }

        if self.retrieval_packet_ref.trim().is_empty()
            || self.retrieval_promotion_state_token != "promotable"
            || !self.retrieval_validation_finding_tokens.is_empty()
        {
            violations.push(ComposerContextEvidenceBetaViolation::RetrievalTruthNotPromotable);
        }
        if self.evidence_packet_state_token == MutationEvidenceState::ReviewPreApply.as_str() {
            violations.push(ComposerContextEvidenceBetaViolation::EvidencePacketNotPostRun);
        }
        if self.routing_packet_ref.trim().is_empty()
            || self.route_receipt_ref.trim().is_empty()
            || self.selected_provider_entry_ref.trim().is_empty()
            || self.selected_model_entry_ref.trim().is_empty()
        {
            violations.push(ComposerContextEvidenceBetaViolation::MissingRouteLineage);
        }
        if self.spend_receipt_ref.trim().is_empty()
            || self.spend_receipt_cost_envelope_token.trim().is_empty()
            || self.spend_receipt_cost_visibility_token.trim().is_empty()
        {
            violations.push(ComposerContextEvidenceBetaViolation::MissingSpendLineage);
        }
        if self.cost_envelope_token != self.spend_receipt_cost_envelope_token
            || self.cost_visibility_token != self.spend_receipt_cost_visibility_token
        {
            violations.push(ComposerContextEvidenceBetaViolation::SpendReceiptMismatch);
        }
        if self.tool_call_lineage_refs.is_empty()
            || self
                .tool_call_lineage_refs
                .iter()
                .any(|reference| reference.trim().is_empty())
        {
            violations.push(ComposerContextEvidenceBetaViolation::MissingToolLineage);
        }
        if self.approval_ticket_refs.is_empty()
            || self
                .approval_ticket_refs
                .iter()
                .any(|reference| reference.trim().is_empty())
        {
            violations.push(ComposerContextEvidenceBetaViolation::MissingApprovalLineage);
        }
        validate_surface_rows(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("composer context evidence packet serializes"),
        ) {
            violations.push(ComposerContextEvidenceBetaViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("composer context evidence beta packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# AI Composer Beta Evidence\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Context snapshot: `{}`\n",
            self.composer_context_snapshot_ref
        ));
        out.push_str(&format!(
            "- Context states: `{}`\n",
            self.observed_context_state_tokens.join(",")
        ));
        out.push_str(&format!(
            "- Evidence packet: `{}` / `{}`\n",
            self.evidence_packet_ref, self.evidence_packet_state_token
        ));
        out.push_str(&format!(
            "- Route/spend: `{}` / `{}`\n",
            self.route_receipt_ref, self.spend_receipt_ref
        ));
        out.push_str(&format!(
            "- Tool lineage refs: {}\n",
            self.tool_call_lineage_refs.len()
        ));
        out.push_str(&format!(
            "- Surface projections: {}\n",
            self.surface_rows.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in beta support export.
#[derive(Debug)]
pub enum ComposerContextEvidenceBetaArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ComposerContextEvidenceBetaViolation>),
}

impl fmt::Display for ComposerContextEvidenceBetaArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "composer context evidence export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "composer context evidence export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ComposerContextEvidenceBetaArtifactError {}

/// Validation failures emitted by [`ComposerContextEvidenceBetaPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ComposerContextEvidenceBetaViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Required context states are not represented.
    MissingContextStateCoverage,
    /// Context rows are missing or malformed.
    MissingContextRows,
    /// Omitted, stale, tainted, summarized, or trimmed row lacks a reason.
    ContextRowMissingReason,
    /// Retrieval packet is missing, stale, or not promotable.
    RetrievalTruthNotPromotable,
    /// Evidence packet is still pre-run.
    EvidencePacketNotPostRun,
    /// Route lineage is missing.
    MissingRouteLineage,
    /// Spend lineage is missing.
    MissingSpendLineage,
    /// Spend receipt disagrees with evidence route lineage.
    SpendReceiptMismatch,
    /// Post-run evidence lacks tool-call lineage.
    MissingToolLineage,
    /// Approval lineage is missing.
    MissingApprovalLineage,
    /// Required consumer surface projection is missing.
    MissingSurfaceProjection,
    /// Surface projection does not preserve the same operator truth refs.
    SurfaceProjectionDrift,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ComposerContextEvidenceBetaViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingContextStateCoverage => "missing_context_state_coverage",
            Self::MissingContextRows => "missing_context_rows",
            Self::ContextRowMissingReason => "context_row_missing_reason",
            Self::RetrievalTruthNotPromotable => "retrieval_truth_not_promotable",
            Self::EvidencePacketNotPostRun => "evidence_packet_not_post_run",
            Self::MissingRouteLineage => "missing_route_lineage",
            Self::MissingSpendLineage => "missing_spend_lineage",
            Self::SpendReceiptMismatch => "spend_receipt_mismatch",
            Self::MissingToolLineage => "missing_tool_lineage",
            Self::MissingApprovalLineage => "missing_approval_lineage",
            Self::MissingSurfaceProjection => "missing_surface_projection",
            Self::SurfaceProjectionDrift => "surface_projection_drift",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in beta support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_beta_composer_context_evidence_support_export(
) -> Result<ComposerContextEvidenceBetaPacket, ComposerContextEvidenceBetaArtifactError> {
    let packet: ComposerContextEvidenceBetaPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m3/composer_context_evidence_beta_support_export.json"
    )))
    .map_err(ComposerContextEvidenceBetaArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ComposerContextEvidenceBetaArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_surface_rows(
    packet: &ComposerContextEvidenceBetaPacket,
    violations: &mut Vec<ComposerContextEvidenceBetaViolation>,
) {
    for required in [
        ComposerContextEvidenceSurfaceClass::Composer,
        ComposerContextEvidenceSurfaceClass::ContextInspector,
        ComposerContextEvidenceSurfaceClass::ReviewWorkspace,
        ComposerContextEvidenceSurfaceClass::DocsHelp,
        ComposerContextEvidenceSurfaceClass::SupportExport,
    ] {
        if !packet
            .surface_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(ComposerContextEvidenceBetaViolation::MissingSurfaceProjection);
            break;
        }
    }

    for row in &packet.surface_rows {
        if row.projection_ref.trim().is_empty()
            || row.composer_context_snapshot_ref != packet.composer_context_snapshot_ref
            || row.evidence_packet_ref != packet.evidence_packet_ref
            || row.route_receipt_ref != packet.route_receipt_ref
            || row.spend_receipt_ref != packet.spend_receipt_ref
            || !row.preserves_operator_truth
            || !row.raw_private_material_excluded
            || !row.supports_json_export
            || !row.supports_markdown_summary
            || packet
                .required_context_state_tokens
                .iter()
                .any(|required| !row.context_state_tokens.contains(required))
        {
            violations.push(ComposerContextEvidenceBetaViolation::SurfaceProjectionDrift);
            break;
        }
    }
}

fn ordered_state_tokens(states: impl Iterator<Item = ContextItemStateClass>) -> Vec<String> {
    let observed = states.collect::<Vec<_>>();
    context_state_token_order()
        .iter()
        .filter(|state| observed.contains(state))
        .map(|state| state.as_str().to_owned())
        .collect()
}

fn context_state_token_order() -> [ContextItemStateClass; 9] {
    [
        ContextItemStateClass::Included,
        ContextItemStateClass::Pinned,
        ContextItemStateClass::Omitted,
        ContextItemStateClass::Blocked,
        ContextItemStateClass::Stale,
        ContextItemStateClass::Tainted,
        ContextItemStateClass::Summarized,
        ContextItemStateClass::Trimmed,
        ContextItemStateClass::NotRequested,
    ]
}

fn requires_omission_reason(state_token: &str) -> bool {
    matches!(
        state_token,
        "omitted" | "blocked" | "stale" | "tainted" | "summarized" | "trimmed"
    )
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(values) => values
            .values()
            .any(json_contains_forbidden_boundary_material),
        _ => false,
    }
}

fn contains_forbidden_boundary_material(value: &str) -> bool {
    let lower = value.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
}

#[cfg(test)]
mod tests;
