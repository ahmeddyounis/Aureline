//! Finalized tainted-context fences, content-boundary handling, and
//! imported-data downgrade rules.
//!
//! This module stabilizes the tainted-context lane into one export-safe packet
//! that binds, for a single material AI or high-risk command run:
//!
//! - the finalized **tainted fences** that strip instruction and write authority
//!   from suspicious or externally sourced context before any run can widen
//!   scope;
//! - the **content-boundary handling** that keeps untrusted data content in a
//!   data-only lane and never lets it cross into the trusted instruction surface;
//! - the **imported-data downgrade rules** that map a real imported artifact to
//!   one of the exact / translated / partial / shimmed / unsupported outcome
//!   labels, narrow execution authority when a mapping is lossy, and always
//!   preserve a rollback checkpoint plus diagnostics when mapping fails; and
//! - the **command-surface parity** that forces the AI, palette, menu,
//!   keybinding, CLI/headless, deep-link, and automation routes to share one
//!   command descriptor, preview, approval, result, and rollback model so no
//!   surface can quietly widen authority behind the user's back.
//!
//! It does not re-derive context-assembly, evidence, or scoped-apply truth. The
//! beta [`crate::tainted_context::TaintedContextBetaPacket`] proves the live
//! narrowing run, the [`crate::evidence::TaintedContextFence`] rows prove a given
//! mutation carried its fence, and the
//! [`crate::harden_ai_scoped_apply::AiScopedApplyHardeningPacket`] proves the
//! apply lifecycle and cross-wedge command parity. This packet re-exports those
//! source/taint/origin and command-surface vocabularies verbatim and adds the
//! finalized invariants the stable line needs: that tainted content stays
//! fenced, that the instruction/data boundary holds, that an imported artifact's
//! mapping outcome and authority downgrade agree, and that the same evidence id
//! reconstructs the run for admin and support.
//!
//! The frozen contracts this lane projects against are the prompt-injection and
//! taint contract
//! ([`docs/ai/prompt_injection_and_taint_contract.md`](../../../docs/ai/prompt_injection_and_taint_contract.md))
//! and the context-assembly contract
//! ([`docs/ai/context_assembly_contract.md`](../../../docs/ai/context_assembly_contract.md)).
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw imported bodies, raw prompts, endpoint
//! URLs, credentials, and billing-account ids stay outside the support boundary.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::harden_ai_scoped_apply::{CommandSurfaceClass, SurfaceQualificationClass};
use crate::tainted_context::{
    TaintedContextInputSourceClass, TaintedContextOriginLocusClass, TaintedContextReasonClass,
    TaintedContextRunModeClass, TaintedContextTaintClass,
};

/// Stable record-kind tag carried by [`FinalizedTaintedContextPacket`].
pub const FINALIZE_TAINTED_CONTEXT_RECORD_KIND: &str = "finalize_tainted_context";

/// Schema version for finalized tainted-context records.
pub const FINALIZE_TAINTED_CONTEXT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the finalized tainted-context boundary schema.
pub const FINALIZE_TAINTED_CONTEXT_SCHEMA_REF: &str =
    "schemas/ai/finalize_tainted_context.schema.json";

/// Repo-relative path of the finalized tainted-context contract doc.
pub const FINALIZE_TAINTED_CONTEXT_AI_DOC_REF: &str =
    "docs/ai/m4/finalize_tainted_context_fences.md";

/// Repo-relative path of the frozen prompt-injection and taint contract.
pub const FINALIZE_TAINTED_CONTEXT_TAINT_CONTRACT_REF: &str =
    "docs/ai/prompt_injection_and_taint_contract.md";

/// Repo-relative path of the frozen context-assembly contract.
pub const FINALIZE_TAINTED_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the protected finalized tainted-context fixture dir.
pub const FINALIZE_TAINTED_CONTEXT_FIXTURE_DIR: &str =
    "fixtures/ai/m4/finalize_tainted_context_fences";

/// Repo-relative path of the checked finalized tainted-context export.
pub const FINALIZE_TAINTED_CONTEXT_ARTIFACT_REF: &str =
    "artifacts/ai/m4/finalize_tainted_context_fences/support_export.json";

/// Repo-relative path of the checked finalized tainted-context Markdown summary.
pub const FINALIZE_TAINTED_CONTEXT_SUMMARY_REF: &str =
    "artifacts/ai/m4/finalize_tainted_context_fences/summary.md";

/// Content-boundary lane a piece of assembled context sits in.
///
/// The instruction/data split is the spine of the finalized fence: trusted
/// instruction surfaces can author intent, but data content — however it
/// arrived — can never cross into the instruction lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentBoundaryClass {
    /// First-party instruction surface (system, policy, or user-authored prompt).
    TrustedInstructionSurface,
    /// Data-only content that can never carry instruction authority.
    UntrustedDataContent,
    /// Quarantined content that may only expose a summary ref.
    QuarantinedDataContent,
    /// Unknown boundary that must fail closed to data-only.
    UnknownBoundaryFailClosed,
}

impl ContentBoundaryClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedInstructionSurface => "trusted_instruction_surface",
            Self::UntrustedDataContent => "untrusted_data_content",
            Self::QuarantinedDataContent => "quarantined_data_content",
            Self::UnknownBoundaryFailClosed => "unknown_boundary_fail_closed",
        }
    }

    /// True when this lane is allowed to carry instruction authority.
    pub const fn may_carry_instruction_authority(self) -> bool {
        matches!(self, Self::TrustedInstructionSurface)
    }

    /// True when this lane must be fenced as data and joined to a fence row.
    pub const fn requires_fence(self) -> bool {
        matches!(
            self,
            Self::UntrustedDataContent
                | Self::QuarantinedDataContent
                | Self::UnknownBoundaryFailClosed
        )
    }

    /// Content boundaries the packet must cover to claim the stable line.
    pub const fn required_coverage() -> [Self; 2] {
        [Self::TrustedInstructionSurface, Self::UntrustedDataContent]
    }
}

/// How a content boundary is enforced for one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryEnforcementClass {
    /// Content stays in a delimited data channel separate from instructions.
    DelimitedDataChannel,
    /// Content is wrapped in a structured data envelope with no execution path.
    StructuredDataEnvelope,
    /// Only a summary ref is exposed; the body never reaches the model.
    SummaryRefOnly,
    /// Content is dropped from context entirely.
    DroppedFromContext,
}

impl BoundaryEnforcementClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DelimitedDataChannel => "delimited_data_channel",
            Self::StructuredDataEnvelope => "structured_data_envelope",
            Self::SummaryRefOnly => "summary_ref_only",
            Self::DroppedFromContext => "dropped_from_context",
        }
    }

    /// True when this enforcement keeps the raw body out of the model entirely.
    pub const fn fails_closed(self) -> bool {
        matches!(self, Self::SummaryRefOnly | Self::DroppedFromContext)
    }
}

/// Outcome label generated when a real imported artifact is mapped into the
/// workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportMappingOutcomeClass {
    /// The imported artifact mapped exactly with no loss.
    Exact,
    /// The artifact was translated to an equivalent native form.
    Translated,
    /// Only part of the artifact mapped; the remainder was dropped or deferred.
    Partial,
    /// The artifact was shimmed behind a compatibility layer.
    Shimmed,
    /// The artifact could not be mapped and is unsupported.
    Unsupported,
}

impl ImportMappingOutcomeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Translated => "translated",
            Self::Partial => "partial",
            Self::Shimmed => "shimmed",
            Self::Unsupported => "unsupported",
        }
    }

    /// Outcome labels the packet must cover, generated from real artifacts.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::Exact,
            Self::Translated,
            Self::Partial,
            Self::Shimmed,
            Self::Unsupported,
        ]
    }

    /// True when the mapping was lossy and authority must narrow.
    pub const fn is_lossy(self) -> bool {
        matches!(self, Self::Partial | Self::Shimmed | Self::Unsupported)
    }

    /// True when mapping diagnostics are mandatory for this outcome.
    pub const fn requires_diagnostics(self) -> bool {
        matches!(self, Self::Partial | Self::Shimmed | Self::Unsupported)
    }
}

/// Authority downgrade applied to an imported artifact based on its outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportAuthorityDowngradeClass {
    /// No downgrade: the artifact mapped exactly.
    NoDowngradeExactMapping,
    /// The mapped artifact must be reviewed before it can apply.
    ReviewBeforeApply,
    /// The mapped artifact may preview but cannot apply automatically.
    NarrowedToPreviewOnly,
    /// The mapped artifact is narrowed to local-only effect.
    NarrowedToLocalOnly,
    /// The artifact is unsupported and blocked from applying.
    BlockedUnsupported,
}

impl ImportAuthorityDowngradeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDowngradeExactMapping => "no_downgrade_exact_mapping",
            Self::ReviewBeforeApply => "review_before_apply",
            Self::NarrowedToPreviewOnly => "narrowed_to_preview_only",
            Self::NarrowedToLocalOnly => "narrowed_to_local_only",
            Self::BlockedUnsupported => "blocked_unsupported",
        }
    }

    /// True when this downgrade actually narrowed authority.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::NoDowngradeExactMapping)
    }

    /// True when this downgrade is consistent with the given mapping outcome.
    fn consistent_with_outcome(self, outcome: ImportMappingOutcomeClass) -> bool {
        match outcome {
            // An exact mapping never narrows.
            ImportMappingOutcomeClass::Exact => {
                matches!(self, Self::NoDowngradeExactMapping)
            }
            // A translated artifact keeps authority but must be reviewed first.
            ImportMappingOutcomeClass::Translated => matches!(
                self,
                Self::ReviewBeforeApply | Self::NarrowedToPreviewOnly | Self::NarrowedToLocalOnly
            ),
            // A lossy mapping narrows but is not necessarily blocked.
            ImportMappingOutcomeClass::Partial | ImportMappingOutcomeClass::Shimmed => matches!(
                self,
                Self::ReviewBeforeApply | Self::NarrowedToPreviewOnly | Self::NarrowedToLocalOnly
            ),
            // An unsupported artifact must be blocked.
            ImportMappingOutcomeClass::Unsupported => matches!(self, Self::BlockedUnsupported),
        }
    }
}

/// One finalized tainted fence covering a suspicious or external context source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedFenceRow {
    /// Stable fence ref.
    pub fence_ref: String,
    /// Stable source ref the fence covers.
    pub source_ref: String,
    /// Input-source class resolved at assembly time.
    pub input_source_class: TaintedContextInputSourceClass,
    /// Taint posture assigned to the source.
    pub taint_class: TaintedContextTaintClass,
    /// Origin locus disclosed for the source.
    pub origin_locus_class: TaintedContextOriginLocusClass,
    /// Reasons the source was fenced.
    pub reason_classes: Vec<TaintedContextReasonClass>,
    /// Fence strategy token preserved from the context/evidence contract.
    pub fence_strategy_token: String,
    /// Usage-constraint tokens the fence imposes.
    pub usage_constraint_tokens: Vec<String>,
    /// True when the fence strips instruction authority from the source.
    pub strips_instruction_authority: bool,
    /// True when provider, tool, or workspace writes are impossible without
    /// re-approval through this fence.
    pub blocks_hidden_provider_write: bool,
    /// True when the fence is reconstructible from audit/support state.
    pub auditable: bool,
    /// True when raw bodies are forbidden on this boundary.
    pub raw_body_forbidden: bool,
    /// Review-safe explanation shown to users and support.
    pub user_visible_label: String,
}

/// One content-boundary handling row for a piece of assembled context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentBoundaryRow {
    /// Stable boundary ref.
    pub boundary_ref: String,
    /// Source ref this boundary covers (joins a fence row for data lanes).
    pub source_ref: String,
    /// Boundary lane this content sits in.
    pub boundary_class: ContentBoundaryClass,
    /// How the boundary is enforced.
    pub enforcement_class: BoundaryEnforcementClass,
    /// True when this content carries instruction authority.
    ///
    /// Must be `false` for every lane except a trusted instruction surface.
    pub carries_instruction_authority: bool,
    /// True when executable/tool authority was stripped from this content.
    pub executable_authority_stripped: bool,
    /// True when raw bodies are forbidden on this boundary.
    pub raw_body_forbidden: bool,
    /// Review-safe explanation shown to users and support.
    pub user_visible_label: String,
}

/// One imported-data downgrade row binding a real artifact's mapping outcome to
/// its authority downgrade, rollback checkpoint, and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportedDataDowngradeRow {
    /// Stable import ref.
    pub import_ref: String,
    /// Review-safe label of the imported artifact kind.
    pub artifact_kind_label: String,
    /// Origin locus the imported artifact arrived from.
    pub source_origin_class: TaintedContextOriginLocusClass,
    /// Outcome label generated from the real imported artifact.
    pub mapping_outcome_class: ImportMappingOutcomeClass,
    /// Authority downgrade applied for this outcome.
    pub authority_downgrade_class: ImportAuthorityDowngradeClass,
    /// Effective run mode after the downgrade.
    pub effective_mode_class: TaintedContextRunModeClass,
    /// True when the row was generated from a real imported artifact rather than
    /// a synthesized placeholder.
    pub generated_from_real_artifact: bool,
    /// Rollback checkpoint ref preserved before the import touched the workspace.
    pub rollback_checkpoint_ref: String,
    /// Mapping diagnostics ref (mandatory when the mapping was lossy).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mapping_diagnostics_ref: Option<String>,
    /// Approval fence ref (required when the import narrowed authority).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_fence_ref: Option<String>,
    /// Review-safe outcome label shown to users and support.
    pub user_visible_outcome_label: String,
}

/// One command-surface parity row proving the surface shares the canonical
/// command descriptor, preview, approval, result, and rollback model and honors
/// the same tainted-content boundary and import downgrade.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParityRow {
    /// Launch wedge this row covers.
    pub surface_class: CommandSurfaceClass,
    /// Command descriptor ref projected on this surface.
    pub descriptor_ref: String,
    /// True when this surface shares the canonical command descriptor.
    pub shares_command_descriptor: bool,
    /// True when this surface shares the same preview model.
    pub shares_preview_model: bool,
    /// True when this surface shares the same approval model.
    pub shares_approval_model: bool,
    /// True when this surface shares the same result model.
    pub shares_result_model: bool,
    /// True when this surface shares the same rollback model.
    pub shares_rollback_model: bool,
    /// True when this surface enforces the same content boundary.
    pub enforces_content_boundary: bool,
    /// True when this surface honors the same imported-data downgrade rules.
    pub honors_import_downgrade: bool,
    /// True when this surface discloses provider/route truth.
    pub route_disclosed: bool,
    /// True when this surface runs the same policy checks.
    pub policy_checked: bool,
    /// True when this surface is reachable for this command.
    pub reachable: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl CommandSurfaceParityRow {
    fn preserves_full_parity(&self) -> bool {
        self.shares_command_descriptor
            && self.shares_preview_model
            && self.shares_approval_model
            && self.shares_result_model
            && self.shares_rollback_model
            && self.enforces_content_boundary
            && self.honors_import_downgrade
            && self.route_disclosed
            && self.policy_checked
            && self.reachable
    }
}

/// Exportable evidence lineage binding the in-product evidence id and rollback
/// lineage admin/support reconstruct the same run from.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TaintedContextEvidenceExport {
    /// Evidence id shown in-product and reused by admin/support exports.
    pub evidence_id: String,
    /// JSON export ref.
    pub json_export_ref: String,
    /// Markdown summary ref.
    pub markdown_summary_ref: String,
    /// Admin inspector ref that resolves the same evidence id.
    pub admin_inspector_ref: String,
    /// Support export ref that resolves the same evidence id.
    pub support_export_ref: String,
    /// Rollback checkpoint lineage refs preserved for this run.
    pub rollback_lineage_refs: Vec<String>,
    /// Export lineage refs (prior exports this one descends from).
    pub export_lineage_refs: Vec<String>,
}

/// Constructor input for [`FinalizedTaintedContextPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizedTaintedContextPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Context snapshot ref.
    pub context_snapshot_ref: String,
    /// Evidence packet ref, when one has been minted.
    pub evidence_packet_ref: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this run was evaluated under.
    pub policy_epoch_ref: String,
    /// Finalized tainted fences.
    pub fence_rows: Vec<TaintedFenceRow>,
    /// Content-boundary handling rows.
    pub content_boundary_rows: Vec<ContentBoundaryRow>,
    /// Imported-data downgrade rows.
    pub import_downgrade_rows: Vec<ImportedDataDowngradeRow>,
    /// Command-surface parity rows.
    pub surface_parity_rows: Vec<CommandSurfaceParityRow>,
    /// Exportable evidence lineage.
    pub evidence_export: TaintedContextEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe finalized tainted-context record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FinalizedTaintedContextPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Claimed workflow or surface id.
    pub workflow_or_surface_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Context snapshot ref.
    pub context_snapshot_ref: String,
    /// Evidence packet ref, when one has been minted.
    pub evidence_packet_ref: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Workspace trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this run was evaluated under.
    pub policy_epoch_ref: String,
    /// Finalized tainted fences.
    pub fence_rows: Vec<TaintedFenceRow>,
    /// Content-boundary handling rows.
    pub content_boundary_rows: Vec<ContentBoundaryRow>,
    /// Imported-data downgrade rows.
    pub import_downgrade_rows: Vec<ImportedDataDowngradeRow>,
    /// Command-surface parity rows.
    pub surface_parity_rows: Vec<CommandSurfaceParityRow>,
    /// Exportable evidence lineage.
    pub evidence_export: TaintedContextEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl FinalizedTaintedContextPacket {
    /// Builds a finalized tainted-context packet from canonical rows.
    pub fn new(input: FinalizedTaintedContextPacketInput) -> Self {
        Self {
            record_kind: FINALIZE_TAINTED_CONTEXT_RECORD_KIND.to_owned(),
            schema_version: FINALIZE_TAINTED_CONTEXT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            workflow_or_surface_id: input.workflow_or_surface_id,
            display_label: input.display_label,
            context_snapshot_ref: input.context_snapshot_ref,
            evidence_packet_ref: input.evidence_packet_ref,
            claimed_stable: input.claimed_stable,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            fence_rows: input.fence_rows,
            content_boundary_rows: input.content_boundary_rows,
            import_downgrade_rows: input.import_downgrade_rows,
            surface_parity_rows: input.surface_parity_rows,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the finalized tainted-context packet's stable-line invariants.
    pub fn validate(&self) -> Vec<FinalizedTaintedContextViolation> {
        let mut violations = Vec::new();
        if self.record_kind != FINALIZE_TAINTED_CONTEXT_RECORD_KIND {
            violations.push(FinalizedTaintedContextViolation::WrongRecordKind);
        }
        if self.schema_version != FINALIZE_TAINTED_CONTEXT_SCHEMA_VERSION {
            violations.push(FinalizedTaintedContextViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.workflow_or_surface_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.context_snapshot_ref.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(FinalizedTaintedContextViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_fences(self, &mut violations);
        validate_content_boundaries(self, &mut violations);
        validate_import_downgrades(self, &mut violations);
        validate_surface_parity(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("finalized tainted context packet serializes"),
        ) {
            violations.push(FinalizedTaintedContextViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("finalized tainted context packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let lossy_imports = self
            .import_downgrade_rows
            .iter()
            .filter(|row| row.mapping_outcome_class.is_lossy())
            .count();
        let narrowed_surfaces = self
            .surface_parity_rows
            .iter()
            .filter(|row| !surface_is_stable(row.qualification))
            .count();
        let mut out = String::new();
        out.push_str("# AI Finalized Tainted Context\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!("- Tainted fences: {}\n", self.fence_rows.len()));
        out.push_str(&format!(
            "- Content boundaries: {}\n",
            self.content_boundary_rows.len()
        ));
        out.push_str(&format!(
            "- Imported-data downgrades: {} ({} lossy)\n",
            self.import_downgrade_rows.len(),
            lossy_imports
        ));
        out.push_str(&format!(
            "- Command surfaces: {} ({} narrowed below Stable)\n",
            self.surface_parity_rows.len(),
            narrowed_surfaces
        ));
        out.push_str(&format!(
            "- Rollback lineage refs: {}\n",
            self.evidence_export.rollback_lineage_refs.len()
        ));
        out
    }
}

/// Errors emitted when reading the checked-in finalized tainted-context export.
#[derive(Debug)]
pub enum FinalizedTaintedContextArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<FinalizedTaintedContextViolation>),
}

impl fmt::Display for FinalizedTaintedContextArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "finalized tainted context export parse failed: {error}"
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
                    "finalized tainted context export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for FinalizedTaintedContextArtifactError {}

/// Validation failures emitted by [`FinalizedTaintedContextPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FinalizedTaintedContextViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No tainted fence rows were provided.
    MissingFenceRows,
    /// A tainted fence does not strip instruction authority or block writes.
    FenceDoesNotStripAuthority,
    /// A tainted fence is missing strategy or usage constraints.
    FenceConstraintsMissing,
    /// Required content-boundary coverage is missing.
    MissingContentBoundaryCoverage,
    /// A data-lane content row carries instruction authority.
    DataContentCarriesInstructionAuthority,
    /// A data-lane content row is not joined to a fence.
    ContentBoundaryUnfenced,
    /// An unknown-boundary row did not fail closed.
    UnknownBoundaryNotFailClosed,
    /// Required import-mapping outcome coverage is missing.
    MissingImportOutcomeCoverage,
    /// An import row was not generated from a real artifact.
    ImportNotFromRealArtifact,
    /// An import's authority downgrade disagrees with its mapping outcome.
    ImportDowngradeInconsistent,
    /// A lossy import is missing mapping diagnostics.
    ImportDiagnosticsMissing,
    /// An import did not preserve a rollback checkpoint.
    ImportRollbackCheckpointMissing,
    /// A narrowed import is missing its approval fence.
    ImportApprovalFenceMissing,
    /// Required command-surface coverage is missing.
    CommandSurfaceCoverageMissing,
    /// A claimed-stable command surface does not preserve full parity.
    CommandParityBroken,
    /// A surface that cannot qualify still claims Stable.
    UnqualifiedSurfaceClaimsStable,
    /// Evidence export refs or the shared evidence id are missing.
    EvidenceExportRefsMissing,
    /// Rollback lineage refs are missing from the evidence export.
    RollbackLineageMissing,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl FinalizedTaintedContextViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MissingFenceRows => "missing_fence_rows",
            Self::FenceDoesNotStripAuthority => "fence_does_not_strip_authority",
            Self::FenceConstraintsMissing => "fence_constraints_missing",
            Self::MissingContentBoundaryCoverage => "missing_content_boundary_coverage",
            Self::DataContentCarriesInstructionAuthority => {
                "data_content_carries_instruction_authority"
            }
            Self::ContentBoundaryUnfenced => "content_boundary_unfenced",
            Self::UnknownBoundaryNotFailClosed => "unknown_boundary_not_fail_closed",
            Self::MissingImportOutcomeCoverage => "missing_import_outcome_coverage",
            Self::ImportNotFromRealArtifact => "import_not_from_real_artifact",
            Self::ImportDowngradeInconsistent => "import_downgrade_inconsistent",
            Self::ImportDiagnosticsMissing => "import_diagnostics_missing",
            Self::ImportRollbackCheckpointMissing => "import_rollback_checkpoint_missing",
            Self::ImportApprovalFenceMissing => "import_approval_fence_missing",
            Self::CommandSurfaceCoverageMissing => "command_surface_coverage_missing",
            Self::CommandParityBroken => "command_parity_broken",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RollbackLineageMissing => "rollback_lineage_missing",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in finalized tainted-context support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_finalize_tainted_context_export(
) -> Result<FinalizedTaintedContextPacket, FinalizedTaintedContextArtifactError> {
    let packet: FinalizedTaintedContextPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/finalize_tainted_context_fences/support_export.json"
    )))
    .map_err(FinalizedTaintedContextArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(FinalizedTaintedContextArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &FinalizedTaintedContextPacket,
    violations: &mut Vec<FinalizedTaintedContextViolation>,
) {
    for required in [
        FINALIZE_TAINTED_CONTEXT_AI_DOC_REF,
        FINALIZE_TAINTED_CONTEXT_SCHEMA_REF,
        FINALIZE_TAINTED_CONTEXT_TAINT_CONTRACT_REF,
        FINALIZE_TAINTED_CONTEXT_ASSEMBLY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(FinalizedTaintedContextViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_fences(
    packet: &FinalizedTaintedContextPacket,
    violations: &mut Vec<FinalizedTaintedContextViolation>,
) {
    if packet.fence_rows.is_empty() {
        violations.push(FinalizedTaintedContextViolation::MissingFenceRows);
        return;
    }

    for fence in &packet.fence_rows {
        if fence.fence_ref.trim().is_empty()
            || fence.source_ref.trim().is_empty()
            || fence.reason_classes.is_empty()
            || fence.user_visible_label.trim().is_empty()
            || !fence.raw_body_forbidden
        {
            violations.push(FinalizedTaintedContextViolation::FenceConstraintsMissing);
            break;
        }
        // A finalized fence over tainted content must strip instruction authority,
        // block hidden provider/tool/workspace writes, and stay auditable.
        if fence.taint_class.requires_fence()
            && (!fence.strips_instruction_authority
                || !fence.blocks_hidden_provider_write
                || !fence.auditable)
        {
            violations.push(FinalizedTaintedContextViolation::FenceDoesNotStripAuthority);
            break;
        }
        if fence.taint_class.requires_fence()
            && (fence.fence_strategy_token.trim().is_empty()
                || fence.usage_constraint_tokens.is_empty())
        {
            violations.push(FinalizedTaintedContextViolation::FenceConstraintsMissing);
            break;
        }
    }
}

fn validate_content_boundaries(
    packet: &FinalizedTaintedContextPacket,
    violations: &mut Vec<FinalizedTaintedContextViolation>,
) {
    for required in ContentBoundaryClass::required_coverage() {
        if !packet
            .content_boundary_rows
            .iter()
            .any(|row| row.boundary_class == required)
        {
            violations.push(FinalizedTaintedContextViolation::MissingContentBoundaryCoverage);
            break;
        }
    }

    let fence_source_refs: BTreeSet<_> = packet
        .fence_rows
        .iter()
        .map(|fence| fence.source_ref.as_str())
        .collect();

    for row in &packet.content_boundary_rows {
        if row.boundary_ref.trim().is_empty()
            || row.source_ref.trim().is_empty()
            || row.user_visible_label.trim().is_empty()
            || !row.raw_body_forbidden
        {
            violations.push(FinalizedTaintedContextViolation::MissingContentBoundaryCoverage);
            break;
        }
        // Only a trusted instruction surface may carry instruction authority;
        // every data lane must have it stripped.
        if !row.boundary_class.may_carry_instruction_authority()
            && (row.carries_instruction_authority || !row.executable_authority_stripped)
        {
            violations
                .push(FinalizedTaintedContextViolation::DataContentCarriesInstructionAuthority);
            break;
        }
        // Data lanes must be joined to a fence so the body cannot widen scope.
        if row.boundary_class.requires_fence()
            && !fence_source_refs.contains(row.source_ref.as_str())
        {
            violations.push(FinalizedTaintedContextViolation::ContentBoundaryUnfenced);
            break;
        }
        // An unknown boundary must fail closed (summary-ref-only or dropped).
        if row.boundary_class == ContentBoundaryClass::UnknownBoundaryFailClosed
            && !row.enforcement_class.fails_closed()
        {
            violations.push(FinalizedTaintedContextViolation::UnknownBoundaryNotFailClosed);
            break;
        }
    }
}

fn validate_import_downgrades(
    packet: &FinalizedTaintedContextPacket,
    violations: &mut Vec<FinalizedTaintedContextViolation>,
) {
    // The exact/translated/partial/shimmed/unsupported labels must all be
    // covered, generated from real imported artifacts.
    for required in ImportMappingOutcomeClass::required_coverage() {
        if !packet
            .import_downgrade_rows
            .iter()
            .any(|row| row.mapping_outcome_class == required)
        {
            violations.push(FinalizedTaintedContextViolation::MissingImportOutcomeCoverage);
            break;
        }
    }

    for row in &packet.import_downgrade_rows {
        if row.import_ref.trim().is_empty()
            || row.artifact_kind_label.trim().is_empty()
            || row.user_visible_outcome_label.trim().is_empty()
        {
            violations.push(FinalizedTaintedContextViolation::MissingImportOutcomeCoverage);
            break;
        }
        if !row.generated_from_real_artifact {
            violations.push(FinalizedTaintedContextViolation::ImportNotFromRealArtifact);
            break;
        }
        // The mapping outcome and the authority downgrade must agree.
        if !row
            .authority_downgrade_class
            .consistent_with_outcome(row.mapping_outcome_class)
        {
            violations.push(FinalizedTaintedContextViolation::ImportDowngradeInconsistent);
            break;
        }
        // A lossy mapping must keep diagnostics so a failed map is inspectable.
        if row.mapping_outcome_class.requires_diagnostics()
            && row
                .mapping_diagnostics_ref
                .as_deref()
                .map_or(true, |reference| reference.trim().is_empty())
        {
            violations.push(FinalizedTaintedContextViolation::ImportDiagnosticsMissing);
            break;
        }
        // A rollback checkpoint is always preserved before an import lands.
        if row.rollback_checkpoint_ref.trim().is_empty() {
            violations.push(FinalizedTaintedContextViolation::ImportRollbackCheckpointMissing);
            break;
        }
        // A narrowed import must name the approval fence that re-gated it.
        if row.authority_downgrade_class.is_narrowed()
            && row
                .approval_fence_ref
                .as_deref()
                .map_or(true, |reference| reference.trim().is_empty())
        {
            violations.push(FinalizedTaintedContextViolation::ImportApprovalFenceMissing);
            break;
        }
    }
}

fn validate_surface_parity(
    packet: &FinalizedTaintedContextPacket,
    violations: &mut Vec<FinalizedTaintedContextViolation>,
) {
    for required in CommandSurfaceClass::required_coverage() {
        if !packet
            .surface_parity_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(FinalizedTaintedContextViolation::CommandSurfaceCoverageMissing);
            break;
        }
    }

    for row in &packet.surface_parity_rows {
        if row.descriptor_ref.trim().is_empty() {
            violations.push(FinalizedTaintedContextViolation::CommandSurfaceCoverageMissing);
            break;
        }
        // A surface that claims Stable must preserve full parity and qualify.
        if row.claimed_stable
            && (!row.preserves_full_parity() || !surface_is_stable(row.qualification))
        {
            violations.push(FinalizedTaintedContextViolation::CommandParityBroken);
            break;
        }
        // A surface that cannot qualify must narrow below Stable, not inherit it.
        if !surface_is_stable(row.qualification) && row.claimed_stable {
            violations.push(FinalizedTaintedContextViolation::UnqualifiedSurfaceClaimsStable);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &FinalizedTaintedContextPacket,
    violations: &mut Vec<FinalizedTaintedContextViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(FinalizedTaintedContextViolation::EvidenceExportRefsMissing);
    }
    // The rollback lineage is the join key a revert reconstructs the run from.
    if export.rollback_lineage_refs.is_empty()
        || export
            .rollback_lineage_refs
            .iter()
            .any(|reference| reference.trim().is_empty())
    {
        violations.push(FinalizedTaintedContextViolation::RollbackLineageMissing);
    }
}

/// True when a command surface's qualification posture is the Stable lane.
const fn surface_is_stable(qualification: SurfaceQualificationClass) -> bool {
    matches!(qualification, SurfaceQualificationClass::Stable)
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(text) => contains_forbidden_boundary_material(text),
        serde_json::Value::Array(values) => {
            values.iter().any(json_contains_forbidden_boundary_material)
        }
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

fn contains_forbidden_boundary_material(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("://")
        || lower.contains("bearer ")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("raw_prompt")
        || lower.contains("raw_body")
        || lower.contains("billing-account")
}

#[cfg(test)]
mod tests;
