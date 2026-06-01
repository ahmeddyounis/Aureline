//! Hardened high-risk command preview, approval lineage, and rollback handles.
//!
//! Where [`crate::stabilize_command_contract`] froze the descriptor fields,
//! invocation/result contract, and cross-surface authority parity for a stable
//! command, and [`crate::finalize_command_parity`] finalized the discoverability
//! half, this module hardens the *write-capable safety* half of the same lane
//! into one export-safe packet. For one canonical high-risk command family on a
//! claimed stable row it binds the four truths a high-risk command must keep
//! identical no matter where it is *applied from*:
//!
//! - the **high-risk preview contract** — the typed, non-bypassable preview a
//!   write-capable command must present before it applies: the effect summary,
//!   scope/targets, dominant side-effect class, diff/impact refs, provider/route
//!   disclosure, spend/egress disclosure, and tainted-context handling, plus the
//!   apply-guard ref and the no-blind-apply invariant that an apply can never
//!   proceed without an acknowledged preview;
//! - the **approval lineage** — the chain of typed approval records that
//!   authorizes a high-risk apply (requested, reviewed, granted, recorded in
//!   audit), each carrying an authority class, a decision ref, and a basis
//!   snapshot ref, bound to one policy epoch, with the no-self-approval and
//!   no-authority-widening guards and an enforced expiry, so support and audit can
//!   reconstruct *who* authorized *what* against *which* basis;
//! - the **rollback-handle issuance** — the reversible rollback handle every
//!   apply-capable high-risk command issues, bound to checkpoint refs and the
//!   in-product evidence id, with a typed revert posture and the invariant that no
//!   durable apply happens without an issued, replayable handle; and
//! - the **cross-surface authority parity** — the same high-risk command requires
//!   the same preview, approval, rollback, route disclosure, and policy checks from
//!   menu/button, keybinding, palette, CLI/headless, AI tool, voice, recipe, deep
//!   link, and browser companion, and no surface widens authority or claims the
//!   Stable lane while it is narrowed below it.
//!
//! It does not re-derive the descriptor, registry, invocation, result, authority,
//! or preview-gate models. The
//! [`crate::registry::CommandPreviewGateMetadata`],
//! [`crate::authority::CommandAuthorityScenarioRecord`], and
//! [`crate::invocation::CommandResultPacketRecord`] own those contracts. This
//! packet references them by stable ref and reuses the canonical contract refs,
//! command-surface vocabulary, surface-qualification posture, and evidence-export
//! shape from [`crate::stabilize_command_contract`] so the high-risk safety lane
//! stays one command truth rather than a parallel dictionary.
//!
//! The frozen contracts this lane projects against are the command-descriptor
//! contract ([`docs/commands/command_descriptor_contract.md`](../../../docs/commands/command_descriptor_contract.md))
//! and the invocation-result and parity contract
//! ([`docs/commands/invocation_result_and_parity_contract.md`](../../../docs/commands/invocation_result_and_parity_contract.md)).
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw command arguments, raw prompts, raw diff
//! bodies, endpoint URLs, credentials, and signing-key material stay outside the
//! support boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::stabilize_command_contract::{
    CommandContractEvidenceExport, CommandSurfaceClass, StableContractRefs,
    SurfaceQualificationClass,
};

/// Stable record-kind tag carried by [`HighRiskCommandHardeningPacket`].
pub const HARDEN_HIGH_RISK_COMMAND_RECORD_KIND: &str = "high_risk_command_hardening_packet";

/// Schema version for hardened high-risk command records.
pub const HARDEN_HIGH_RISK_COMMAND_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the hardened high-risk command boundary schema.
pub const HARDEN_HIGH_RISK_COMMAND_SCHEMA_REF: &str =
    "schemas/commands/harden_high_risk_command.schema.json";

/// Repo-relative path of the hardened high-risk command doc.
pub const HARDEN_HIGH_RISK_COMMAND_DOC_REF: &str = "docs/commands/m4/harden_high_risk_command.md";

/// Repo-relative path of the frozen command-descriptor contract.
pub const HARDEN_HIGH_RISK_COMMAND_DESCRIPTOR_CONTRACT_REF: &str =
    "docs/commands/command_descriptor_contract.md";

/// Repo-relative path of the frozen invocation-result and parity contract.
pub const HARDEN_HIGH_RISK_COMMAND_PARITY_CONTRACT_REF: &str =
    "docs/commands/invocation_result_and_parity_contract.md";

/// Repo-relative path of the protected hardened high-risk command fixture dir.
pub const HARDEN_HIGH_RISK_COMMAND_FIXTURE_DIR: &str =
    "fixtures/commands/m4/harden_high_risk_command";

/// Repo-relative path of the checked hardened high-risk command export.
pub const HARDEN_HIGH_RISK_COMMAND_ARTIFACT_REF: &str =
    "artifacts/commands/m4/harden_high_risk_command/support_export.json";

/// Repo-relative path of the checked hardened high-risk command Markdown summary.
pub const HARDEN_HIGH_RISK_COMMAND_SUMMARY_REF: &str =
    "artifacts/commands/m4/harden_high_risk_command/summary.md";

/// High-risk effect class a command family can carry.
///
/// These coarse classes drive preview depth, approval requirements, and rollback
/// posture. A command family declares the subset that applies to it; the packet
/// does not require every class on one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HighRiskClass {
    /// Destructive or bulk filesystem mutation.
    DestructiveFilesystem,
    /// Bulk multi-file or multi-buffer edit.
    BulkMultiFileEdit,
    /// Irreversible or history-rewriting version-control action.
    IrreversibleVcs,
    /// Effect that leaves the device over the network.
    ExternalNetworkEffect,
    /// Access to credentials, secrets, or signing material.
    CredentialOrSecretAccess,
    /// Action that incurs metered spend.
    SpendIncurring,
    /// Policy-sensitive automation reachable without a human in the loop.
    PolicySensitiveAutomation,
}

impl HighRiskClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DestructiveFilesystem => "destructive_filesystem",
            Self::BulkMultiFileEdit => "bulk_multi_file_edit",
            Self::IrreversibleVcs => "irreversible_vcs",
            Self::ExternalNetworkEffect => "external_network_effect",
            Self::CredentialOrSecretAccess => "credential_or_secret_access",
            Self::SpendIncurring => "spend_incurring",
            Self::PolicySensitiveAutomation => "policy_sensitive_automation",
        }
    }
}

/// Typed preview requirement a high-risk command's preview must satisfy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewRequirementClass {
    /// A plain-language effect summary of what will change.
    EffectSummary,
    /// The scope and concrete targets the command will touch.
    ScopeAndTargets,
    /// A diff or impact projection ref.
    DiffOrImpact,
    /// Provider/route and spend/egress disclosure.
    RouteAndSpendDisclosure,
    /// An explicit destructive/irreversible confirmation step.
    DestructiveConfirmation,
}

impl PreviewRequirementClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EffectSummary => "effect_summary",
            Self::ScopeAndTargets => "scope_and_targets",
            Self::DiffOrImpact => "diff_or_impact",
            Self::RouteAndSpendDisclosure => "route_and_spend_disclosure",
            Self::DestructiveConfirmation => "destructive_confirmation",
        }
    }

    /// Preview requirements a stable high-risk preview must enumerate.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::EffectSummary,
            Self::ScopeAndTargets,
            Self::DiffOrImpact,
            Self::RouteAndSpendDisclosure,
            Self::DestructiveConfirmation,
        ]
    }
}

/// Authority class that can request or grant a high-risk approval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalAuthorityClass {
    /// The invoking user acting alone. Never a valid *approver* for a high-risk apply.
    SelfOnly,
    /// The workspace owner.
    WorkspaceOwner,
    /// A policy administrator.
    PolicyAdmin,
    /// A distinct second human reviewer.
    SecondHumanReviewer,
    /// A managed policy gate acting under an explicit governing feature.
    ManagedPolicyGate,
}

impl ApprovalAuthorityClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelfOnly => "self_only",
            Self::WorkspaceOwner => "workspace_owner",
            Self::PolicyAdmin => "policy_admin",
            Self::SecondHumanReviewer => "second_human_reviewer",
            Self::ManagedPolicyGate => "managed_policy_gate",
        }
    }

    /// True when this authority is the invoking user acting alone.
    pub const fn is_self_only(self) -> bool {
        matches!(self, Self::SelfOnly)
    }
}

/// One step in a high-risk command's approval lineage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalStepClass {
    /// The apply was requested.
    Requested,
    /// The request was reviewed against its basis.
    Reviewed,
    /// The apply was granted.
    Granted,
    /// The grant was recorded in the audit lineage.
    RecordedInAudit,
}

impl ApprovalStepClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::Reviewed => "reviewed",
            Self::Granted => "granted",
            Self::RecordedInAudit => "recorded_in_audit",
        }
    }

    /// Approval-lineage steps a stable high-risk approval must record.
    pub const fn required_coverage() -> [Self; 4] {
        [
            Self::Requested,
            Self::Reviewed,
            Self::Granted,
            Self::RecordedInAudit,
        ]
    }
}

/// Rollback posture issued for a high-risk apply.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RollbackPostureClass {
    /// Fully reversible; the apply can be undone with no residue.
    FullyReversible,
    /// Reversible by replaying a checkpoint.
    CheckpointRevert,
    /// Reversible only by a compensating action.
    CompensatingAction,
    /// Irreversible but gated behind an explicit confirmation.
    IrreversibleWithExplicitConfirmation,
    /// No rollback is available. Never valid for a claimed-stable durable apply.
    NoRollbackAvailable,
}

impl RollbackPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyReversible => "fully_reversible",
            Self::CheckpointRevert => "checkpoint_revert",
            Self::CompensatingAction => "compensating_action",
            Self::IrreversibleWithExplicitConfirmation => "irreversible_with_explicit_confirmation",
            Self::NoRollbackAvailable => "no_rollback_available",
        }
    }

    /// True when this posture can issue a replayable rollback handle.
    pub const fn issues_handle(self) -> bool {
        matches!(
            self,
            Self::FullyReversible | Self::CheckpointRevert | Self::CompensatingAction
        )
    }
}

/// The typed, non-bypassable preview a high-risk command presents before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighRiskPreviewContract {
    /// True when a preview is mandatory on apply-capable paths.
    pub required: bool,
    /// Preview requirements the preview enumerates.
    pub requirements: Vec<PreviewRequirementClass>,
    /// True when the preview shows the plain-language effect summary.
    pub shows_effect_summary: bool,
    /// True when the preview shows the scope and concrete targets.
    pub shows_scope_and_targets: bool,
    /// True when the preview shows the dominant side-effect cue.
    pub shows_dominant_side_effect: bool,
    /// True when the preview discloses the provider/route.
    pub discloses_route_and_provider: bool,
    /// True when the preview discloses spend/egress.
    pub discloses_spend_and_egress: bool,
    /// True when the preview surfaces tainted-context handling.
    pub surfaces_tainted_context: bool,
    /// Guard ref checked before the command mutates or exposes effects.
    pub apply_guard_ref: String,
    /// True when an apply can never proceed without an acknowledged preview.
    pub no_blind_apply: bool,
}

impl HighRiskPreviewContract {
    fn covers_all_cues(&self) -> bool {
        self.shows_effect_summary
            && self.shows_scope_and_targets
            && self.shows_dominant_side_effect
            && self.discloses_route_and_provider
            && self.discloses_spend_and_egress
            && self.surfaces_tainted_context
            && self.no_blind_apply
            && !self.apply_guard_ref.trim().is_empty()
    }
}

/// One typed approval record in a high-risk command's lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalLineageRecord {
    /// Step class for this lineage record.
    pub step_class: ApprovalStepClass,
    /// Authority class that performed this step.
    pub authority_class: ApprovalAuthorityClass,
    /// Decision ref (opaque) for this step.
    pub decision_ref: String,
    /// Basis snapshot ref the step was evaluated against.
    pub basis_snapshot_ref: String,
    /// True when this step is recorded in the audit lineage.
    pub recorded_in_audit: bool,
}

/// The approval-lineage contract a high-risk command enforces before apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovalLineageContract {
    /// True when an approval is mandatory on apply-capable paths.
    pub required: bool,
    /// Authority class that requested the apply.
    pub requester_authority: ApprovalAuthorityClass,
    /// Authority class that granted the apply.
    pub approver_authority: ApprovalAuthorityClass,
    /// Typed approval records forming the lineage.
    pub records: Vec<ApprovalLineageRecord>,
    /// Policy epoch ref the approval was evaluated under.
    pub policy_epoch_ref: String,
    /// Basis snapshot ref the approval was bound to.
    pub basis_snapshot_ref: String,
    /// True when the requester can never also be the approver.
    pub no_self_approval: bool,
    /// True when the grant can never widen authority beyond the request.
    pub no_authority_widening: bool,
    /// True when approval expiry is enforced.
    pub expiry_enforced: bool,
}

impl ApprovalLineageContract {
    fn guards_hold(&self) -> bool {
        self.no_self_approval
            && self.no_authority_widening
            && self.expiry_enforced
            && !self.approver_authority.is_self_only()
            && !self.policy_epoch_ref.trim().is_empty()
            && !self.basis_snapshot_ref.trim().is_empty()
    }
}

/// The rollback-handle issuance contract for a high-risk apply.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RollbackHandleContract {
    /// True when a rollback handle is issued on apply.
    pub issued: bool,
    /// Opaque rollback handle ref (never raw material).
    pub handle_ref: String,
    /// Typed revert posture for the apply.
    pub posture: RollbackPostureClass,
    /// Checkpoint refs the handle replays from.
    pub checkpoint_refs: Vec<String>,
    /// True when the handle is bound to the in-product evidence id.
    pub bound_to_evidence_id: bool,
    /// True when no durable apply can proceed without an issued handle.
    pub no_durable_apply_without_handle: bool,
    /// True when a revert can be replayed from the handle.
    pub revert_replayable: bool,
}

impl RollbackHandleContract {
    fn guards_hold(&self) -> bool {
        self.issued
            && self.bound_to_evidence_id
            && self.no_durable_apply_without_handle
            && self.revert_replayable
            && self.posture.issues_handle()
            && !self.handle_ref.trim().is_empty()
            && !self.checkpoint_refs.is_empty()
            && self
                .checkpoint_refs
                .iter()
                .all(|reference| !reference.trim().is_empty())
    }
}

/// One cross-surface authority-parity row for a high-risk command.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighRiskSurfaceParityRow {
    /// Invocation surface this row covers.
    pub surface_class: CommandSurfaceClass,
    /// Command descriptor ref projected on this surface.
    pub descriptor_ref: String,
    /// True when the command is reachable on this surface.
    pub reachable: bool,
    /// True when this surface enforces the same preview.
    pub enforces_preview: bool,
    /// True when this surface enforces the same approval lineage.
    pub enforces_approval: bool,
    /// True when this surface issues the same rollback handle.
    pub issues_rollback_handle: bool,
    /// True when this surface discloses provider/route truth.
    pub discloses_route: bool,
    /// True when this surface runs the same policy checks.
    pub policy_checked: bool,
    /// True when this surface never widens command authority.
    pub no_authority_widening: bool,
    /// Stable-qualification posture for this surface.
    pub qualification: SurfaceQualificationClass,
    /// True when this surface claims the Stable lane.
    pub claimed_stable: bool,
}

impl HighRiskSurfaceParityRow {
    fn preserves_full_parity(&self) -> bool {
        self.enforces_preview
            && self.enforces_approval
            && self.issues_rollback_handle
            && self.discloses_route
            && self.policy_checked
            && self.no_authority_widening
    }
}

/// Constructor input for [`HighRiskCommandHardeningPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HighRiskCommandHardeningPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical command-family id this packet hardens.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// High-risk effect classes this command family carries.
    pub risk_classes: Vec<HighRiskClass>,
    /// The high-risk preview contract.
    pub preview_contract: HighRiskPreviewContract,
    /// The approval-lineage contract.
    pub approval_lineage: ApprovalLineageContract,
    /// The rollback-handle issuance contract.
    pub rollback_handle: RollbackHandleContract,
    /// Cross-surface authority-parity rows.
    pub surface_parity_rows: Vec<HighRiskSurfaceParityRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe hardened high-risk command record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HighRiskCommandHardeningPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Canonical command-family id this packet hardens.
    pub command_family_id: String,
    /// Display label safe for UI, docs, and support.
    pub display_label: String,
    /// Whether this row claims the Stable line.
    pub claimed_stable: bool,
    /// Policy epoch ref this row was evaluated under.
    pub policy_epoch_ref: String,
    /// The single descriptor registry and result schema every surface projects.
    pub contract_refs: StableContractRefs,
    /// High-risk effect classes this command family carries.
    pub risk_classes: Vec<HighRiskClass>,
    /// The high-risk preview contract.
    pub preview_contract: HighRiskPreviewContract,
    /// The approval-lineage contract.
    pub approval_lineage: ApprovalLineageContract,
    /// The rollback-handle issuance contract.
    pub rollback_handle: RollbackHandleContract,
    /// Cross-surface authority-parity rows.
    pub surface_parity_rows: Vec<HighRiskSurfaceParityRow>,
    /// Exportable evidence lineage.
    pub evidence_export: CommandContractEvidenceExport,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl HighRiskCommandHardeningPacket {
    /// Builds a hardened high-risk command packet from canonical rows.
    pub fn new(input: HighRiskCommandHardeningPacketInput) -> Self {
        Self {
            record_kind: HARDEN_HIGH_RISK_COMMAND_RECORD_KIND.to_owned(),
            schema_version: HARDEN_HIGH_RISK_COMMAND_SCHEMA_VERSION,
            packet_id: input.packet_id,
            command_family_id: input.command_family_id,
            display_label: input.display_label,
            claimed_stable: input.claimed_stable,
            policy_epoch_ref: input.policy_epoch_ref,
            contract_refs: input.contract_refs,
            risk_classes: input.risk_classes,
            preview_contract: input.preview_contract,
            approval_lineage: input.approval_lineage,
            rollback_handle: input.rollback_handle,
            surface_parity_rows: input.surface_parity_rows,
            evidence_export: input.evidence_export,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the hardened high-risk command packet's stable-line invariants.
    pub fn validate(&self) -> Vec<HighRiskCommandHardeningViolation> {
        let mut violations = Vec::new();
        if self.record_kind != HARDEN_HIGH_RISK_COMMAND_RECORD_KIND {
            violations.push(HighRiskCommandHardeningViolation::WrongRecordKind);
        }
        if self.schema_version != HARDEN_HIGH_RISK_COMMAND_SCHEMA_VERSION {
            violations.push(HighRiskCommandHardeningViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.command_family_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(HighRiskCommandHardeningViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_contract_refs(self, &mut violations);
        validate_risk_classes(self, &mut violations);
        validate_preview_contract(self, &mut violations);
        validate_approval_lineage(self, &mut violations);
        validate_rollback_handle(self, &mut violations);
        validate_surface_parity(self, &mut violations);
        validate_evidence_export(self, &mut violations);
        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("hardened high-risk command packet serializes"),
        ) {
            violations.push(HighRiskCommandHardeningViolation::RawMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("hardened high-risk command packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed_surfaces = self
            .surface_parity_rows
            .iter()
            .filter(|row| !row.qualification.is_stable())
            .count();
        let audited_steps = self
            .approval_lineage
            .records
            .iter()
            .filter(|record| record.recorded_in_audit)
            .count();
        let mut out = String::new();
        out.push_str("# High-Risk Command Hardening\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Command family: `{}`\n", self.command_family_id));
        out.push_str(&format!(
            "- Evidence id: `{}`\n",
            self.evidence_export.evidence_id
        ));
        out.push_str(&format!("- Claimed stable: {}\n", self.claimed_stable));
        out.push_str(&format!("- Risk classes: {}\n", self.risk_classes.len()));
        out.push_str(&format!(
            "- Preview requirements: {} (required: {})\n",
            self.preview_contract.requirements.len(),
            self.preview_contract.required
        ));
        out.push_str(&format!(
            "- Approval lineage steps: {} ({} recorded in audit)\n",
            self.approval_lineage.records.len(),
            audited_steps
        ));
        out.push_str(&format!(
            "- Rollback posture: `{}` (handle issued: {})\n",
            self.rollback_handle.posture.as_str(),
            self.rollback_handle.issued
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

/// Errors emitted when reading the checked-in hardened high-risk command export.
#[derive(Debug)]
pub enum HighRiskCommandHardeningArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<HighRiskCommandHardeningViolation>),
}

impl fmt::Display for HighRiskCommandHardeningArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "hardened high-risk command export parse failed: {error}"
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
                    "hardened high-risk command export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for HighRiskCommandHardeningArtifactError {}

/// Validation failures emitted by [`HighRiskCommandHardeningPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighRiskCommandHardeningViolation {
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
    /// No high-risk classes are declared for a high-risk row.
    MissingRiskClasses,
    /// The preview is not required on an apply-capable high-risk row.
    PreviewNotRequired,
    /// Preview requirement coverage is incomplete.
    PreviewRequirementCoverageMissing,
    /// A preview cue or no-blind-apply guard is missing.
    PreviewCueMissing,
    /// The approval is not required on an apply-capable high-risk row.
    ApprovalNotRequired,
    /// Approval-lineage step coverage is incomplete.
    ApprovalStepCoverageMissing,
    /// An approval-lineage record is missing structured refs.
    ApprovalRecordRefsMissing,
    /// The grant is not recorded in the audit lineage.
    ApprovalGrantNotAudited,
    /// An approval guard was broken (self-approval, widening, no expiry).
    ApprovalGuardBroken,
    /// The rollback handle was not issued or is missing structured refs.
    RollbackHandleNotIssued,
    /// The rollback posture cannot issue a replayable handle for a durable apply.
    RollbackPostureUnsafe,
    /// Cross-surface coverage is incomplete.
    CommandSurfaceCoverageMissing,
    /// A claimed-stable surface broke preview/approval/rollback parity.
    HighRiskParityBroken,
    /// A surface narrowed below Stable still claims the Stable lane.
    UnqualifiedSurfaceClaimsStable,
    /// Evidence export refs are missing.
    EvidenceExportRefsMissing,
    /// The rollback lineage is missing.
    RollbackLineageMissing,
    /// The packet carries raw material outside the export boundary.
    RawMaterialInExport,
}

impl HighRiskCommandHardeningViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ContractRefsNotCanonical => "contract_refs_not_canonical",
            Self::MissingRiskClasses => "missing_risk_classes",
            Self::PreviewNotRequired => "preview_not_required",
            Self::PreviewRequirementCoverageMissing => "preview_requirement_coverage_missing",
            Self::PreviewCueMissing => "preview_cue_missing",
            Self::ApprovalNotRequired => "approval_not_required",
            Self::ApprovalStepCoverageMissing => "approval_step_coverage_missing",
            Self::ApprovalRecordRefsMissing => "approval_record_refs_missing",
            Self::ApprovalGrantNotAudited => "approval_grant_not_audited",
            Self::ApprovalGuardBroken => "approval_guard_broken",
            Self::RollbackHandleNotIssued => "rollback_handle_not_issued",
            Self::RollbackPostureUnsafe => "rollback_posture_unsafe",
            Self::CommandSurfaceCoverageMissing => "command_surface_coverage_missing",
            Self::HighRiskParityBroken => "high_risk_parity_broken",
            Self::UnqualifiedSurfaceClaimsStable => "unqualified_surface_claims_stable",
            Self::EvidenceExportRefsMissing => "evidence_export_refs_missing",
            Self::RollbackLineageMissing => "rollback_lineage_missing",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Returns the checked-in hardened high-risk command support export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_high_risk_command_hardening_export(
) -> Result<HighRiskCommandHardeningPacket, HighRiskCommandHardeningArtifactError> {
    let packet: HighRiskCommandHardeningPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/commands/m4/harden_high_risk_command/support_export.json"
    )))
    .map_err(HighRiskCommandHardeningArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(HighRiskCommandHardeningArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &HighRiskCommandHardeningPacket,
    violations: &mut Vec<HighRiskCommandHardeningViolation>,
) {
    for required in [
        HARDEN_HIGH_RISK_COMMAND_DOC_REF,
        HARDEN_HIGH_RISK_COMMAND_SCHEMA_REF,
        HARDEN_HIGH_RISK_COMMAND_DESCRIPTOR_CONTRACT_REF,
        HARDEN_HIGH_RISK_COMMAND_PARITY_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(HighRiskCommandHardeningViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_contract_refs(
    packet: &HighRiskCommandHardeningPacket,
    violations: &mut Vec<HighRiskCommandHardeningViolation>,
) {
    if packet.contract_refs != StableContractRefs::canonical() {
        violations.push(HighRiskCommandHardeningViolation::ContractRefsNotCanonical);
    }
}

fn validate_risk_classes(
    packet: &HighRiskCommandHardeningPacket,
    violations: &mut Vec<HighRiskCommandHardeningViolation>,
) {
    if packet.risk_classes.is_empty() {
        violations.push(HighRiskCommandHardeningViolation::MissingRiskClasses);
    }
}

fn validate_preview_contract(
    packet: &HighRiskCommandHardeningPacket,
    violations: &mut Vec<HighRiskCommandHardeningViolation>,
) {
    let preview = &packet.preview_contract;
    // A high-risk, apply-capable, claimed-stable command must require a preview.
    if packet.claimed_stable && !preview.required {
        violations.push(HighRiskCommandHardeningViolation::PreviewNotRequired);
    }
    for required in PreviewRequirementClass::required_coverage() {
        if !preview.requirements.iter().any(|item| *item == required) {
            violations.push(HighRiskCommandHardeningViolation::PreviewRequirementCoverageMissing);
            break;
        }
    }
    // The preview must show every disclosure cue and keep the no-blind-apply guard.
    if preview.required && !preview.covers_all_cues() {
        violations.push(HighRiskCommandHardeningViolation::PreviewCueMissing);
    }
}

fn validate_approval_lineage(
    packet: &HighRiskCommandHardeningPacket,
    violations: &mut Vec<HighRiskCommandHardeningViolation>,
) {
    let approval = &packet.approval_lineage;
    // A high-risk, apply-capable, claimed-stable command must require an approval.
    if packet.claimed_stable && !approval.required {
        violations.push(HighRiskCommandHardeningViolation::ApprovalNotRequired);
    }
    for required in ApprovalStepClass::required_coverage() {
        if !approval
            .records
            .iter()
            .any(|record| record.step_class == required)
        {
            violations.push(HighRiskCommandHardeningViolation::ApprovalStepCoverageMissing);
            break;
        }
    }
    for record in &approval.records {
        if record.decision_ref.trim().is_empty() || record.basis_snapshot_ref.trim().is_empty() {
            violations.push(HighRiskCommandHardeningViolation::ApprovalRecordRefsMissing);
            break;
        }
    }
    // The grant must be recorded in the audit lineage so a revert/audit can replay it.
    if approval.required
        && !approval
            .records
            .iter()
            .any(|record| record.step_class == ApprovalStepClass::Granted && record.recorded_in_audit)
    {
        violations.push(HighRiskCommandHardeningViolation::ApprovalGrantNotAudited);
    }
    // No self-approval, no authority widening, enforced expiry, real basis/epoch.
    if approval.required && !approval.guards_hold() {
        violations.push(HighRiskCommandHardeningViolation::ApprovalGuardBroken);
    }
}

fn validate_rollback_handle(
    packet: &HighRiskCommandHardeningPacket,
    violations: &mut Vec<HighRiskCommandHardeningViolation>,
) {
    let rollback = &packet.rollback_handle;
    // A claimed-stable high-risk apply must issue a bound, replayable rollback handle.
    if packet.claimed_stable && !rollback.guards_hold() {
        violations.push(HighRiskCommandHardeningViolation::RollbackHandleNotIssued);
    }
    // No-rollback-available is never a valid posture for a claimed-stable durable apply.
    if packet.claimed_stable && !rollback.posture.issues_handle() {
        violations.push(HighRiskCommandHardeningViolation::RollbackPostureUnsafe);
    }
}

fn validate_surface_parity(
    packet: &HighRiskCommandHardeningPacket,
    violations: &mut Vec<HighRiskCommandHardeningViolation>,
) {
    for required in CommandSurfaceClass::required_coverage() {
        if !packet
            .surface_parity_rows
            .iter()
            .any(|row| row.surface_class == required)
        {
            violations.push(HighRiskCommandHardeningViolation::CommandSurfaceCoverageMissing);
            break;
        }
    }

    for row in &packet.surface_parity_rows {
        if row.descriptor_ref.trim().is_empty() {
            violations.push(HighRiskCommandHardeningViolation::CommandSurfaceCoverageMissing);
            break;
        }
        // A surface narrowed below Stable may not claim the Stable lane.
        if row.claimed_stable && !row.qualification.is_stable() {
            violations.push(HighRiskCommandHardeningViolation::UnqualifiedSurfaceClaimsStable);
            break;
        }
        // A Stable, reachable surface must enforce preview, approval, rollback,
        // route disclosure, and policy without widening authority.
        if row.qualification.is_stable() && row.reachable && !row.preserves_full_parity() {
            violations.push(HighRiskCommandHardeningViolation::HighRiskParityBroken);
            break;
        }
    }
}

fn validate_evidence_export(
    packet: &HighRiskCommandHardeningPacket,
    violations: &mut Vec<HighRiskCommandHardeningViolation>,
) {
    let export = &packet.evidence_export;
    if export.evidence_id.trim().is_empty()
        || export.json_export_ref.trim().is_empty()
        || export.markdown_summary_ref.trim().is_empty()
        || export.admin_inspector_ref.trim().is_empty()
        || export.support_export_ref.trim().is_empty()
    {
        violations.push(HighRiskCommandHardeningViolation::EvidenceExportRefsMissing);
    }
    if export.rollback_lineage_refs.is_empty()
        || export
            .rollback_lineage_refs
            .iter()
            .any(|reference| reference.trim().is_empty())
    {
        violations.push(HighRiskCommandHardeningViolation::RollbackLineageMissing);
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
