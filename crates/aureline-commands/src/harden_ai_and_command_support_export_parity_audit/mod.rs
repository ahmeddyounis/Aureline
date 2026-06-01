//! Hardened AI and command support-export parity, audit lineage, and shiproom inclusion.
//!
//! Where [`crate::stabilize_command_contract`] froze the descriptor fields,
//! invocation/result contract, and cross-surface authority parity for a stable
//! command, and [`crate::harden_high_risk_command`] hardened the preview,
//! approval, and rollback half for high-risk commands, this module hardens the
//! *support-export parity* half of one write-capable AI/command lane into one
//! export-safe packet. For one claimed-stable packet it binds the three truths a
//! reviewer, operator, support engineer, or shiproom participant must be able to
//! reconstruct without re-running the write path:
//!
//! - the **support-export parity contract** — every AI invocation and every
//!   command invocation on a write-capable path exports the same preview record,
//!   approval lineage, provider/route identity, spend/egress disclosure,
//!   tainted-context fence handling, rollback/revert handle, and audit lineage,
//!   and AI plus command surfaces project one descriptor, preview, approval,
//!   result, and rollback model with no hidden provider route;
//! - the **audit-lineage contract** — a stable actor identity, invocation
//!   surface, policy epoch, provider/route identity, decision ref, and outcome are
//!   all recorded in a non-repudiable lineage so support and audit can explain who
//!   invoked what, from where, against which policy epoch, on which provider
//!   route, and how it resolved; and
//! - the **shiproom-inclusion contract** — the parity packet is indexed in the
//!   stable proof index, referenced by the release checklist, included in the
//!   support export bundle, and validated against the checked-in artifact refs so
//!   the release room, field support, and exported evidence packets all speak one
//!   truth.
//!
//! It does not re-derive the descriptor, registry, invocation, result, preview,
//! approval, rollback, or evidence-export models. The frozen command-descriptor
//! contract
//! ([`docs/commands/command_descriptor_contract.md`](../../../docs/commands/command_descriptor_contract.md))
//! and the invocation-result and parity contract
//! ([`docs/commands/invocation_result_and_parity_contract.md`](../../../docs/commands/invocation_result_and_parity_contract.md))
//! remain canonical; this packet references them by stable ref and reuses the
//! canonical contract refs, surface-qualification posture, and evidence-export
//! shape from [`crate::stabilize_command_contract`] so AI, UI, CLI, support, and
//! shiproom views stay one attributable truth rather than parallel dictionaries.
//!
//! The record is export-safe. It carries refs, stable class tokens, booleans,
//! counts, and review labels only. Raw prompts, raw command arguments, raw diff
//! bodies, endpoint URLs, credentials, provider payloads, and signing-key
//! material stay outside the support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_command_contract::{
    CommandContractEvidenceExport, StableContractRefs, SurfaceQualificationClass,
};

/// Stable record-kind tag carried by
/// [`HardenAiAndCommandSupportExportParityAuditPacket`].
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_RECORD_KIND: &str =
    "harden_ai_command_support_export_parity_audit";

/// Schema version for AI/command support-export parity audit records.
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the AI/command support-export parity audit schema.
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SCHEMA_REF: &str =
    "schemas/ai/harden_ai_and_command_support_export_parity_audit.schema.json";

/// Repo-relative path of the AI/command support-export parity audit doc.
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_DOC_REF: &str =
    "docs/ai/m4/harden_ai_and_command_support_export_parity_audit.md";

/// Repo-relative path of the frozen command-descriptor contract.
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_DESCRIPTOR_CONTRACT_REF: &str =
    "docs/commands/command_descriptor_contract.md";

/// Repo-relative path of the frozen invocation-result and parity contract.
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_PARITY_CONTRACT_REF: &str =
    "docs/commands/invocation_result_and_parity_contract.md";

/// Repo-relative path of the protected parity-audit fixture directory.
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_FIXTURE_DIR: &str =
    "fixtures/ai/m4/harden_ai_and_command_support_export_parity_audit";

/// Repo-relative path of the checked parity-audit export.
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_ARTIFACT_REF: &str =
    "artifacts/ai/m4/harden_ai_and_command_support_export_parity_audit/support_export.json";

/// Repo-relative path of the checked parity-audit Markdown summary.
pub const HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SUMMARY_REF: &str =
    "artifacts/ai/m4/harden_ai_and_command_support_export_parity_audit/summary.md";

/// Invocation surface whose write-capable export must preserve support parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportSurfaceClass {
    /// The UI command palette/menu/button entry point.
    UiPaletteOrMenu,
    /// Keyboard binding entry point.
    Keybinding,
    /// CLI/headless entry point.
    CliHeadless,
    /// AI tool/assistant entry point.
    AiTool,
    /// Automation recipe entry point.
    AutomationRecipe,
    /// Deep-link or browser companion entry point.
    DeepLinkOrBrowserCompanion,
    /// Voice entry point.
    Voice,
}

impl SupportExportSurfaceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiPaletteOrMenu => "ui_palette_or_menu",
            Self::Keybinding => "keybinding",
            Self::CliHeadless => "cli_headless",
            Self::AiTool => "ai_tool",
            Self::AutomationRecipe => "automation_recipe",
            Self::DeepLinkOrBrowserCompanion => "deep_link_or_browser_companion",
            Self::Voice => "voice",
        }
    }

    /// Invocation surfaces the packet must cover to claim support-export parity.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::UiPaletteOrMenu,
            Self::Keybinding,
            Self::CliHeadless,
            Self::AiTool,
            Self::AutomationRecipe,
            Self::DeepLinkOrBrowserCompanion,
            Self::Voice,
        ]
    }
}

/// Required field class every support export must preserve across surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportParityRequirementClass {
    /// The export must carry a preview record ref.
    PreviewRecordRef,
    /// The export must carry an approval lineage ref.
    ApprovalLineageRef,
    /// The export must carry provider and route identity.
    ProviderAndRouteIdentity,
    /// The export must carry spend/egress disclosure.
    SpendAndEgressDisclosure,
    /// The export must carry tainted-context fence handling.
    TaintedContextFenceHandling,
    /// The export must carry a rollback/revert handle ref.
    RollbackOrRevertHandleRef,
    /// The export must carry an audit lineage trace.
    AuditLineageTrace,
}

impl ExportParityRequirementClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRecordRef => "preview_record_ref",
            Self::ApprovalLineageRef => "approval_lineage_ref",
            Self::ProviderAndRouteIdentity => "provider_and_route_identity",
            Self::SpendAndEgressDisclosure => "spend_and_egress_disclosure",
            Self::TaintedContextFenceHandling => "tainted_context_fence_handling",
            Self::RollbackOrRevertHandleRef => "rollback_or_revert_handle_ref",
            Self::AuditLineageTrace => "audit_lineage_trace",
        }
    }

    /// Requirement classes a stable support export must enumerate.
    pub const fn required_coverage() -> [Self; 7] {
        [
            Self::PreviewRecordRef,
            Self::ApprovalLineageRef,
            Self::ProviderAndRouteIdentity,
            Self::SpendAndEgressDisclosure,
            Self::TaintedContextFenceHandling,
            Self::RollbackOrRevertHandleRef,
            Self::AuditLineageTrace,
        ]
    }
}

/// Required lineage fact every stable audit record must bind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuditLineageRequirementClass {
    /// A stable actor identity is bound to the invocation.
    ActorIdentityBound,
    /// The invocation surface is recorded.
    InvocationSurfaceRecorded,
    /// The policy epoch is pinned at invocation time.
    PolicyEpochPinned,
    /// The provider/route identity is recorded.
    ProviderRouteRecorded,
    /// The decision ref is traceable.
    DecisionRefTraceable,
    /// The outcome is recorded and non-repudiable.
    OutcomeRecorded,
}

impl AuditLineageRequirementClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorIdentityBound => "actor_identity_bound",
            Self::InvocationSurfaceRecorded => "invocation_surface_recorded",
            Self::PolicyEpochPinned => "policy_epoch_pinned",
            Self::ProviderRouteRecorded => "provider_route_recorded",
            Self::DecisionRefTraceable => "decision_ref_traceable",
            Self::OutcomeRecorded => "outcome_recorded",
        }
    }

    /// Audit-lineage requirement classes a stable packet must enumerate.
    pub const fn required_coverage() -> [Self; 6] {
        [
            Self::ActorIdentityBound,
            Self::InvocationSurfaceRecorded,
            Self::PolicyEpochPinned,
            Self::ProviderRouteRecorded,
            Self::DecisionRefTraceable,
            Self::OutcomeRecorded,
        ]
    }
}

/// Required shiproom inclusion a stable proof packet must preserve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShiproomInclusionClass {
    /// The packet is indexed in the stable proof index.
    IndexedInStableProofIndex,
    /// The packet is referenced by the release checklist.
    ReferencedByReleaseChecklist,
    /// The packet is included in the support export bundle.
    IncludedInSupportExportBundle,
    /// The packet's artifact refs are validated against checked-in files.
    ArtifactRefsValidated,
}

impl ShiproomInclusionClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndexedInStableProofIndex => "indexed_in_stable_proof_index",
            Self::ReferencedByReleaseChecklist => "referenced_by_release_checklist",
            Self::IncludedInSupportExportBundle => "included_in_support_export_bundle",
            Self::ArtifactRefsValidated => "artifact_refs_validated",
        }
    }

    /// Shiproom inclusions a stable packet must enumerate.
    pub const fn required_coverage() -> [Self; 4] {
        [
            Self::IndexedInStableProofIndex,
            Self::ReferencedByReleaseChecklist,
            Self::IncludedInSupportExportBundle,
            Self::ArtifactRefsValidated,
        ]
    }
}

/// One cross-surface support-export parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportParityRow {
    /// Invocation surface this row covers.
    pub surface_class: SupportExportSurfaceClass,
    /// Command descriptor ref projected on this surface.
    pub descriptor_ref: String,
    /// True when the write-capable path is reachable on this surface.
    pub reachable: bool,
    /// True when the export carries a preview record ref.
    pub carries_preview_record: bool,
    /// True when the export carries an approval-lineage ref.
    pub carries_approval_lineage: bool,
    /// True when the export carries provider/route identity.
    pub carries_provider_route_identity: bool,
    /// True when the export carries spend/egress disclosure.
    pub carries_spend_egress_disclosure: bool,
    /// True when the export carries tainted-context fence handling.
    pub carries_tainted_context_fence: bool,
    /// True when the export carries a rollback or revert handle ref.
    pub carries_rollback_handle: bool,
    /// True when the export carries an audit-lineage trace.
    pub carries_audit_lineage: bool,
    /// True when this surface does not widen authority.
    pub no_authority_widening: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl SupportExportParityRow {
    fn preserves_full_parity(&self) -> bool {
        self.carries_preview_record
            && self.carries_approval_lineage
            && self.carries_provider_route_identity
            && self.carries_spend_egress_disclosure
            && self.carries_tainted_context_fence
            && self.carries_rollback_handle
            && self.carries_audit_lineage
            && self.no_authority_widening
    }
}

/// The shared support-export parity contract for AI and command surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportParityContract {
    /// True when support-export parity is mandatory on the claimed row.
    pub required: bool,
    /// Required parity facts the export enumerates.
    pub requirements: Vec<ExportParityRequirementClass>,
    /// True when AI and command share one descriptor contract.
    pub ai_and_command_share_one_descriptor: bool,
    /// True when AI and command share one preview model.
    pub ai_and_command_share_one_preview_model: bool,
    /// True when AI and command share one approval model.
    pub ai_and_command_share_one_approval_model: bool,
    /// True when AI and command share one result model.
    pub ai_and_command_share_one_result_model: bool,
    /// True when AI and command share one rollback model.
    pub ai_and_command_share_one_rollback_model: bool,
    /// True when provider/route identity is explicit on every surface.
    pub provider_route_explicit_on_every_surface: bool,
    /// True when spend/egress disclosure is explicit on every surface.
    pub spend_egress_explicit_on_every_surface: bool,
    /// True when tainted-context handling is explicit on every surface.
    pub tainted_context_explicit_on_every_surface: bool,
    /// True when no provider route is hidden behind a surface-specific path.
    pub no_hidden_provider_route: bool,
}

impl ExportParityContract {
    fn covers_all_cues(&self) -> bool {
        self.ai_and_command_share_one_descriptor
            && self.ai_and_command_share_one_preview_model
            && self.ai_and_command_share_one_approval_model
            && self.ai_and_command_share_one_result_model
            && self.ai_and_command_share_one_rollback_model
            && self.provider_route_explicit_on_every_surface
            && self.spend_egress_explicit_on_every_surface
            && self.tainted_context_explicit_on_every_surface
            && self.no_hidden_provider_route
    }
}

/// The audit-lineage contract a stable packet must bind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuditLineageContract {
    /// True when audit lineage is mandatory on the claimed row.
    pub required: bool,
    /// Required lineage facts the packet enumerates.
    pub requirements: Vec<AuditLineageRequirementClass>,
    /// True when a stable actor identity is bound.
    pub actor_identity_bound: bool,
    /// True when the invocation surface is recorded.
    pub invocation_surface_recorded: bool,
    /// True when the policy epoch is pinned.
    pub policy_epoch_pinned: bool,
    /// True when the provider/route identity is recorded.
    pub provider_route_recorded: bool,
    /// True when the decision ref is traceable.
    pub decision_ref_traceable: bool,
    /// True when the outcome is recorded.
    pub outcome_recorded: bool,
    /// True when the lineage is non-repudiable.
    pub non_repudiable: bool,
    /// Policy epoch ref the lineage was bound to.
    pub policy_epoch_ref: String,
}

impl AuditLineageContract {
    fn covers_all_cues(&self) -> bool {
        self.actor_identity_bound
            && self.invocation_surface_recorded
            && self.policy_epoch_pinned
            && self.provider_route_recorded
            && self.decision_ref_traceable
            && self.outcome_recorded
            && self.non_repudiable
            && !self.policy_epoch_ref.trim().is_empty()
    }
}

/// The shiproom inclusion contract for the parity-audit packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShiproomInclusionContract {
    /// True when shiproom inclusion is mandatory on the claimed row.
    pub required: bool,
    /// Required shiproom inclusions the packet enumerates.
    pub inclusions: Vec<ShiproomInclusionClass>,
    /// True when the packet is indexed in the stable proof index.
    pub indexed_in_stable_proof_index: bool,
    /// True when the packet is referenced by the release checklist.
    pub referenced_by_release_checklist: bool,
    /// True when the packet is included in the support export bundle.
    pub included_in_support_export_bundle: bool,
    /// True when artifact refs were validated against checked-in files.
    pub artifact_refs_validated: bool,
    /// Stable proof index ref that carries this packet.
    pub stable_proof_index_ref: String,
}

impl ShiproomInclusionContract {
    fn guards_hold(&self) -> bool {
        self.indexed_in_stable_proof_index
            && self.referenced_by_release_checklist
            && self.included_in_support_export_bundle
            && self.artifact_refs_validated
            && !self.stable_proof_index_ref.trim().is_empty()
    }
}

/// Constructor input for [`HardenAiAndCommandSupportExportParityAuditPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HardenAiAndCommandSupportExportParityAuditPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Display label safe for UI, docs, support, and shiproom.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// The support-export parity contract.
    pub export_parity_contract: ExportParityContract,
    /// Cross-surface support-export parity rows.
    pub surface_parity_rows: Vec<SupportExportParityRow>,
    /// The audit-lineage contract.
    pub audit_lineage: AuditLineageContract,
    /// The shiproom inclusion contract.
    pub shiproom_inclusion: ShiproomInclusionContract,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI/command support-export parity audit record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenAiAndCommandSupportExportParityAuditPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Display label safe for UI, docs, support, and shiproom.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// The support-export parity contract.
    pub export_parity_contract: ExportParityContract,
    /// Cross-surface support-export parity rows.
    pub surface_parity_rows: Vec<SupportExportParityRow>,
    /// The audit-lineage contract.
    pub audit_lineage: AuditLineageContract,
    /// The shiproom inclusion contract.
    pub shiproom_inclusion: ShiproomInclusionContract,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl HardenAiAndCommandSupportExportParityAuditPacket {
    /// Builds an AI/command support-export parity audit packet from canonical rows.
    pub fn new(input: HardenAiAndCommandSupportExportParityAuditPacketInput) -> Self {
        Self {
            record_kind: HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_RECORD_KIND.to_owned(),
            schema_version: HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            display_label: input.display_label,
            claimed_stable: input.claimed_stable,
            policy_epoch_ref: input.policy_epoch_ref,
            contract_refs: input.contract_refs,
            export_parity_contract: input.export_parity_contract,
            surface_parity_rows: input.surface_parity_rows,
            audit_lineage: input.audit_lineage,
            shiproom_inclusion: input.shiproom_inclusion,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the parity-audit packet's stable-line invariants.
    pub fn validate(&self) -> Vec<AiAndCommandSupportExportParityAuditViolation> {
        let mut violations = Vec::new();
        if self.record_kind != HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_RECORD_KIND {
            violations.push(AiAndCommandSupportExportParityAuditViolation::WrongRecordKind);
        }
        if self.schema_version != HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SCHEMA_VERSION {
            violations.push(AiAndCommandSupportExportParityAuditViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiAndCommandSupportExportParityAuditViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_contract_refs(self, &mut violations);
        validate_export_parity_contract(self, &mut violations);
        validate_audit_lineage(self, &mut violations);
        validate_shiproom_inclusion(self, &mut violations);
        validate_surface_parity(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("AI and command support-export parity audit packet serializes"),
        ) {
            violations.push(AiAndCommandSupportExportParityAuditViolation::RawMaterialInExport);
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
            .expect("AI and command support-export parity audit packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or shiproom handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed_surfaces = self
            .surface_parity_rows
            .iter()
            .filter(|row| !row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# AI and Command Support-Export Parity Audit\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!(
            "- Export parity requirements: {} (required: {})\n",
            self.export_parity_contract.requirements.len(),
            self.export_parity_contract.required
        ));
        out.push_str(&format!(
            "- Support-export surfaces: {} ({} narrowed below Stable)\n",
            self.surface_parity_rows.len(),
            narrowed_surfaces
        ));
        out.push_str(&format!(
            "- Audit lineage requirements: {} (required: {})\n",
            self.audit_lineage.requirements.len(),
            self.audit_lineage.required
        ));
        out.push_str(&format!(
            "- Shiproom inclusions: {} (required: {})\n",
            self.shiproom_inclusion.inclusions.len(),
            self.shiproom_inclusion.required
        ));
        out.push_str(&format!(
            "- Rollback lineage refs: {}\n",
            self.evidence_export.rollback_lineage_refs.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in parity-audit export.
#[derive(Debug)]
pub enum AiAndCommandSupportExportParityAuditArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiAndCommandSupportExportParityAuditViolation>),
}

impl fmt::Display for AiAndCommandSupportExportParityAuditArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "AI and command support-export parity audit export parse failed: {error}"
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
                    "AI and command support-export parity audit export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiAndCommandSupportExportParityAuditArtifactError {}

/// Validation failures emitted by
/// [`HardenAiAndCommandSupportExportParityAuditPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiAndCommandSupportExportParityAuditViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The bound canonical contract refs drifted from the single registry/schema.
    ContractRefsNotCanonical,
    /// The support-export parity contract is not required on a stable row.
    ExportParityContractNotRequired,
    /// Support-export parity requirement coverage is incomplete.
    ExportParityRequirementCoverageMissing,
    /// A support-export parity cue is missing.
    ExportParityContractCueMissing,
    /// Audit lineage is not required on a stable row.
    AuditLineageNotRequired,
    /// Audit-lineage requirement coverage is incomplete.
    AuditLineageRequirementCoverageMissing,
    /// An audit-lineage cue is missing.
    AuditLineageCueMissing,
    /// Shiproom inclusion is not required on a stable row.
    ShiproomInclusionNotRequired,
    /// Shiproom inclusion coverage or inclusion cues are incomplete.
    ShiproomInclusionCoverageMissing,
    /// Cross-surface coverage is incomplete.
    CommandSurfaceCoverageMissing,
    /// A claimed-stable surface broke support-export parity.
    SupportExportParityBroken,
    /// A surface narrowed below Stable still claims the Stable lane.
    UnqualifiedSurfaceClaimsStable,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl AiAndCommandSupportExportParityAuditViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ContractRefsNotCanonical => "contract_refs_not_canonical",
            Self::ExportParityContractNotRequired => "export_parity_contract_not_required",
            Self::ExportParityRequirementCoverageMissing => {
                "export_parity_requirement_coverage_missing"
            }
            Self::ExportParityContractCueMissing => "export_parity_contract_cue_missing",
            Self::AuditLineageNotRequired => "audit_lineage_not_required",
            Self::AuditLineageRequirementCoverageMissing => {
                "audit_lineage_requirement_coverage_missing"
            }
            Self::AuditLineageCueMissing => "audit_lineage_cue_missing",
            Self::ShiproomInclusionNotRequired => "shiproom_inclusion_not_required",
            Self::ShiproomInclusionCoverageMissing => "shiproom_inclusion_coverage_missing",
            Self::CommandSurfaceCoverageMissing => "command_surface_coverage_missing",
            Self::SupportExportParityBroken => "support_export_parity_broken",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in AI/command support-export parity audit export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_harden_ai_and_command_support_export_parity_audit_export() -> Result<
    HardenAiAndCommandSupportExportParityAuditPacket,
    AiAndCommandSupportExportParityAuditArtifactError,
> {
    let packet: HardenAiAndCommandSupportExportParityAuditPacket = serde_json::from_str(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/ai/m4/harden_ai_and_command_support_export_parity_audit/support_export.json"
        )),
    )
    .map_err(AiAndCommandSupportExportParityAuditArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiAndCommandSupportExportParityAuditArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &HardenAiAndCommandSupportExportParityAuditPacket,
    violations: &mut Vec<AiAndCommandSupportExportParityAuditViolation>,
) {
    for required in [
        HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_DOC_REF,
        HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_SCHEMA_REF,
        HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_DESCRIPTOR_CONTRACT_REF,
        HARDEN_AI_COMMAND_SUPPORT_EXPORT_PARITY_AUDIT_PARITY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(AiAndCommandSupportExportParityAuditViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_contract_refs(
    packet: &HardenAiAndCommandSupportExportParityAuditPacket,
    violations: &mut Vec<AiAndCommandSupportExportParityAuditViolation>,
) {
    if packet.contract_refs != StableContractRefs::canonical() {
        violations.push(AiAndCommandSupportExportParityAuditViolation::ContractRefsNotCanonical);
    }
}

fn validate_export_parity_contract(
    packet: &HardenAiAndCommandSupportExportParityAuditPacket,
    violations: &mut Vec<AiAndCommandSupportExportParityAuditViolation>,
) {
    let contract = &packet.export_parity_contract;
    if packet.claimed_stable && !contract.required {
        violations
            .push(AiAndCommandSupportExportParityAuditViolation::ExportParityContractNotRequired);
    }

    let coverage_missing = ExportParityRequirementClass::required_coverage()
        .into_iter()
        .any(|required| !contract.requirements.iter().any(|item| *item == required));
    if coverage_missing {
        violations.push(
            AiAndCommandSupportExportParityAuditViolation::ExportParityRequirementCoverageMissing,
        );
    }

    if contract.required && !contract.covers_all_cues() {
        violations
            .push(AiAndCommandSupportExportParityAuditViolation::ExportParityContractCueMissing);
    }
}

fn validate_audit_lineage(
    packet: &HardenAiAndCommandSupportExportParityAuditPacket,
    violations: &mut Vec<AiAndCommandSupportExportParityAuditViolation>,
) {
    let audit = &packet.audit_lineage;
    if packet.claimed_stable && !audit.required {
        violations.push(AiAndCommandSupportExportParityAuditViolation::AuditLineageNotRequired);
    }

    let coverage_missing = AuditLineageRequirementClass::required_coverage()
        .into_iter()
        .any(|required| !audit.requirements.iter().any(|item| *item == required));
    if coverage_missing {
        violations.push(
            AiAndCommandSupportExportParityAuditViolation::AuditLineageRequirementCoverageMissing,
        );
    }

    if audit.required
        && (!audit.covers_all_cues() || audit.policy_epoch_ref != packet.policy_epoch_ref)
    {
        violations.push(AiAndCommandSupportExportParityAuditViolation::AuditLineageCueMissing);
    }
}

fn validate_shiproom_inclusion(
    packet: &HardenAiAndCommandSupportExportParityAuditPacket,
    violations: &mut Vec<AiAndCommandSupportExportParityAuditViolation>,
) {
    let shiproom = &packet.shiproom_inclusion;
    if packet.claimed_stable && !shiproom.required {
        violations
            .push(AiAndCommandSupportExportParityAuditViolation::ShiproomInclusionNotRequired);
    }

    let coverage_missing = ShiproomInclusionClass::required_coverage()
        .into_iter()
        .any(|required| !shiproom.inclusions.iter().any(|item| *item == required));
    if coverage_missing || (packet.claimed_stable && !shiproom.guards_hold()) {
        violations
            .push(AiAndCommandSupportExportParityAuditViolation::ShiproomInclusionCoverageMissing);
    }
}

fn validate_surface_parity(
    packet: &HardenAiAndCommandSupportExportParityAuditPacket,
    violations: &mut Vec<AiAndCommandSupportExportParityAuditViolation>,
) {
    let coverage_missing = SupportExportSurfaceClass::required_coverage()
        .into_iter()
        .any(|required| {
            !packet
                .surface_parity_rows
                .iter()
                .any(|row| row.surface_class == required)
        });
    if coverage_missing {
        violations
            .push(AiAndCommandSupportExportParityAuditViolation::CommandSurfaceCoverageMissing);
    }

    for row in &packet.surface_parity_rows {
        if row.descriptor_ref.trim().is_empty() {
            violations
                .push(AiAndCommandSupportExportParityAuditViolation::CommandSurfaceCoverageMissing);
            break;
        }
        if row.claimed_stable && !row.qualification.is_stable() {
            violations.push(
                AiAndCommandSupportExportParityAuditViolation::UnqualifiedSurfaceClaimsStable,
            );
            break;
        }
        if row.qualification.is_stable() && row.reachable && !row.preserves_full_parity() {
            violations
                .push(AiAndCommandSupportExportParityAuditViolation::SupportExportParityBroken);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &HardenAiAndCommandSupportExportParityAuditPacket,
    violations: &mut Vec<AiAndCommandSupportExportParityAuditViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
        || export.rollback_lineage_refs.is_empty()
        || export
            .rollback_lineage_refs
            .iter()
            .any(|reference| reference.trim().is_empty())
    {
        violations.push(AiAndCommandSupportExportParityAuditViolation::EvidenceExportRefsMissing);
    }
}

fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_material(text),
        serde_json::Value::Array(values) => values.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

fn contains_forbidden_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("private_key")
        || lower.contains("signing_key")
        || lower.contains("raw_prompt")
        || lower.contains("raw_diff")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
