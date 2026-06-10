//! AI memory classes, explicit saved memory, and workspace/tenant deletion and
//! export posture.
//!
//! This module ships the canonical M5 packet that locks how the AI's memory
//! classes are deleted and exported at workspace and tenant scope into one
//! export-safe artifact. It builds on the stable
//! [`crate::memory::AiMemoryStatePacket`] lane — which froze the six product
//! AI memory classes ([`crate::memory::AiStateClass`]) and their per-class
//! delete/export posture — and adds the operator-facing dimension the stable
//! lane left to later work: actually executing a workspace-scoped or
//! tenant-scoped delete or export across every memory class, with disclosed
//! retention holds, accountable explicit saved memory, per-class fan-out
//! completeness, verified receipts, and honest partial/blocked states. The
//! packet binds three blocks:
//!
//! - A [`MemoryClassCoverageBlock`] asserts that every product AI memory class
//!   is addressed by the deletion and export fan-out. Each row reuses the frozen
//!   [`crate::memory::AiStateClass`] vocabulary and records whether delete and
//!   export fan-out cover the class, the retention hold (if any) that keeps an
//!   evidence-governed copy beyond a delete request, and whether that hold is
//!   disclosed rather than silently skipped.
//! - An [`ExplicitSavedMemoryBlock`] presents the explicit saved-memory entries
//!   in scope. Each carries the scope it was saved at (user, repo, or org), the
//!   accountable actor who saved it, the consent posture, and whether it is
//!   revocable. Saved memory is never anonymous, never unconsented, and always
//!   revocable.
//! - A [`ScopedDeletionExportBlock`] presents the workspace- or tenant-scoped
//!   delete and export operations. Each carries the operation kind, the scope it
//!   ran at, its per-class fan-out completeness, the receipt handle it produced,
//!   and its receipt verification state. A completed operation addresses every
//!   class and carries a verified receipt; a partial operation states its
//!   incompleteness honestly rather than claiming completion.
//!
//! The packet references upstream lanes by id rather than embedding their
//! content: it cites the stable [`crate::memory`] memory-state contract, the
//! lower-level memory-object and reconciliation contracts, the
//! [`crate::freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents`]
//! workflow matrix, and the frozen context-assembly contract.
//!
//! The record is export-safe. It carries refs, state tokens, coarse classes,
//! counts, and review labels only. Raw saved-memory bodies, raw prompts,
//! responses, terminal transcripts, raw vectors, raw provider payloads, raw
//! support bodies, credentials, endpoint URLs, raw file paths, and
//! billing-account ids stay outside this boundary.
//!
//! The boundary schema is
//! [`schemas/ai/implement-ai-memory-classes-explicit-saved-memory-and-workspace-or-tenant-deletion-and-export-posture.schema.json`](../../../../schemas/ai/implement-ai-memory-classes-explicit-saved-memory-and-workspace-or-tenant-deletion-and-export-posture.schema.json).
//! The contract doc is
//! [`docs/ai/m5/implement_ai_memory_classes_explicit_saved_memory_and_workspace_or_tenant_deletion_and_export_posture.md`](../../../../docs/ai/m5/implement_ai_memory_classes_explicit_saved_memory_and_workspace_or_tenant_deletion_and_export_posture.md).

#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::memory::AiStateClass;

/// Stable record-kind tag carried by [`AiMemoryDeletionExportPosturePacket`].
pub const MEMORY_DELETION_EXPORT_RECORD_KIND: &str = "ai_memory_deletion_export_posture";

/// Schema version for AI memory deletion/export posture records.
pub const MEMORY_DELETION_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MEMORY_DELETION_EXPORT_SCHEMA_REF: &str =
    "schemas/ai/implement-ai-memory-classes-explicit-saved-memory-and-workspace-or-tenant-deletion-and-export-posture.schema.json";

/// Repo-relative path of the M5 contract doc.
pub const MEMORY_DELETION_EXPORT_DOC_REF: &str =
    "docs/ai/m5/implement_ai_memory_classes_explicit_saved_memory_and_workspace_or_tenant_deletion_and_export_posture.md";

/// Repo-relative path of the stable AI memory delete/export contract.
pub const MEMORY_DELETION_EXPORT_DELETE_CONTRACT_REF: &str = "docs/ai/ai-memory-delete-export.md";

/// Repo-relative path of the lower-level memory/reconciliation contract.
pub const MEMORY_DELETION_EXPORT_RECONCILIATION_CONTRACT_REF: &str =
    "docs/ai/memory_and_reconciliation_contract.md";

/// Repo-relative path of the lower-level memory-object schema.
pub const MEMORY_DELETION_EXPORT_MEMORY_OBJECT_SCHEMA_REF: &str =
    "schemas/ai/memory_object.schema.json";

/// Repo-relative path of the machine-readable memory-class matrix.
pub const MEMORY_DELETION_EXPORT_MEMORY_CLASSES_REF: &str = "artifacts/ai/memory_classes.yaml";

/// Repo-relative path of the frozen context-assembly contract.
pub const MEMORY_DELETION_EXPORT_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the frozen M5 AI workflow matrix contract.
pub const MEMORY_DELETION_EXPORT_M5_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_workflow_matrix_for_inline_assist_patch_review_and_branch_or_worktree_agents.md";

/// Repo-relative path of the protected fixture directory.
pub const MEMORY_DELETION_EXPORT_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_ai_memory_classes_explicit_saved_memory_and_workspace_or_tenant_deletion_and_export_posture";

/// Repo-relative path of the checked support-export artifact.
pub const MEMORY_DELETION_EXPORT_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_ai_memory_classes_explicit_saved_memory_and_workspace_or_tenant_deletion_and_export_posture/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MEMORY_DELETION_EXPORT_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_ai_memory_classes_explicit_saved_memory_and_workspace_or_tenant_deletion_and_export_posture.md";

/// Retention hold that keeps a memory class beyond a delete request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionHoldClass {
    /// No hold; the class is deleted on request.
    NoHold,
    /// Held under an evidence-retention policy until expiry or case close.
    EvidenceHold,
    /// Held under a legal or compliance hold.
    LegalHold,
}

impl RetentionHoldClass {
    /// Returns whether this hold keeps a copy beyond a delete request.
    #[must_use]
    pub fn holds_beyond_delete(self) -> bool {
        !matches!(self, Self::NoHold)
    }
}

/// Scope at which an explicit saved-memory entry was saved.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedMemoryScopeClass {
    /// Saved against one user.
    UserScoped,
    /// Saved against one repository.
    RepoScoped,
    /// Saved against an organization.
    OrgScoped,
}

/// Accountable actor that saved an explicit memory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedMemoryActorClass {
    /// An end user saved the memory.
    EndUser,
    /// A repository owner saved the memory.
    RepoOwner,
    /// An organization admin saved the memory.
    OrgAdmin,
    /// No accountable actor (not allowed for saved memory).
    Unattributed,
}

impl SavedMemoryActorClass {
    /// Returns whether an accountable actor saved the memory.
    #[must_use]
    pub fn is_accountable(self) -> bool {
        !matches!(self, Self::Unattributed)
    }
}

/// Consent posture recorded for an explicit saved-memory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SavedMemoryConsentClass {
    /// Explicitly consented by the accountable actor.
    ExplicitlyConsented,
    /// Saved under a disclosed policy default.
    PolicyDefault,
    /// Not consented (not allowed for saved memory).
    NotConsented,
}

impl SavedMemoryConsentClass {
    /// Returns whether the entry is consented or disclosed-policy-default.
    #[must_use]
    pub fn is_permitted(self) -> bool {
        !matches!(self, Self::NotConsented)
    }
}

/// Kind of scoped memory operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryOperationKindClass {
    /// A deletion of memory at the operation's scope.
    Deletion,
    /// An export of memory at the operation's scope.
    Export,
}

/// Scope at which a deletion or export operation runs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeletionExportScopeClass {
    /// Scoped to one workspace.
    WorkspaceScoped,
    /// Scoped to one repository.
    RepositoryScoped,
    /// Scoped to one tenant.
    TenantScoped,
    /// Scoped to one account.
    AccountScoped,
    /// Scoped to one organization.
    OrganizationScoped,
}

/// Per-class fan-out completeness for a deletion or export operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FanOutCompletenessClass {
    /// Every memory class was addressed.
    AllClassesCovered,
    /// Complete except for disclosed retention-held classes.
    PartialPendingRetentionHold,
    /// Incomplete and unverified.
    IncompleteUnverified,
}

impl FanOutCompletenessClass {
    /// Returns whether this completeness claims a fully covered fan-out.
    #[must_use]
    pub fn claims_complete(self) -> bool {
        matches!(self, Self::AllClassesCovered)
    }
}

/// Receipt verification state for a deletion or export operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptVerificationClass {
    /// A verified receipt was produced.
    VerifiedReceipt,
    /// A receipt exists but verification is pending.
    PendingVerification,
    /// No receipt was produced (not allowed for a claimed-complete operation).
    UnverifiedNoReceipt,
}

impl ReceiptVerificationClass {
    /// Returns whether a verified receipt backs the operation.
    #[must_use]
    pub fn is_verified(self) -> bool {
        matches!(self, Self::VerifiedReceipt)
    }
}

/// Consumer surface that must project this memory deletion/export lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryPostureConsumerSurface {
    /// The memory inventory inspector.
    MemoryInspector,
    /// The delete/export review sheet.
    DeleteExportReview,
    /// The workspace/tenant admin console.
    AdminConsole,
    /// The CLI/headless surface.
    CliHeadless,
    /// The support-export surface.
    SupportExport,
    /// The diagnostics surface.
    Diagnostics,
}

impl MemoryPostureConsumerSurface {
    /// All consumer surfaces that must project this lane.
    pub const ALL: [MemoryPostureConsumerSurface; 6] = [
        MemoryPostureConsumerSurface::MemoryInspector,
        MemoryPostureConsumerSurface::DeleteExportReview,
        MemoryPostureConsumerSurface::AdminConsole,
        MemoryPostureConsumerSurface::CliHeadless,
        MemoryPostureConsumerSurface::SupportExport,
        MemoryPostureConsumerSurface::Diagnostics,
    ];
}

/// Qualification class for a consumer-surface projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryPostureQualificationClass {
    /// Stable and fully reachable.
    Stable,
    /// Beta-qualified.
    Beta,
    /// Preview-qualified.
    Preview,
    /// Experimental.
    Experimental,
    /// Not available on this surface.
    Unavailable,
}

impl MemoryPostureQualificationClass {
    /// Returns whether this qualification class is stable.
    #[must_use]
    pub fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryPostureDowngradeTrigger {
    /// The proof packet is stale.
    ProofStale,
    /// Policy blocked the lane.
    PolicyBlocked,
    /// The provider is unavailable.
    ProviderUnavailable,
    /// The workspace or tenant trust posture narrowed.
    TrustNarrowing,
    /// A scope expansion was claimed without qualification.
    ScopeExpansionUnqualified,
    /// An upstream dependency narrowed.
    UpstreamDependencyNarrowed,
    /// A delete fan-out was incomplete for a non-held class.
    DeleteFanOutIncomplete,
    /// An export fan-out was incomplete.
    ExportFanOutIncomplete,
    /// Saved memory was found without an accountable owner.
    SavedMemoryWithoutAccountableOwner,
    /// A deletion receipt was unverified for a claimed-complete operation.
    DeletionReceiptUnverified,
    /// A retention hold was applied without disclosure.
    RetentionHoldUndisclosed,
}

/// One memory-class coverage row in the deletion/export fan-out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryClassCoverageRow {
    /// Memory class this row covers (reuses the frozen state-class vocabulary).
    pub state_class: AiStateClass,
    /// Whether the delete fan-out covers this class.
    pub delete_fan_out_covered: bool,
    /// Whether the export fan-out covers this class.
    pub export_fan_out_covered: bool,
    /// Retention hold that keeps this class beyond a delete request, if any.
    pub retention_hold: RetentionHoldClass,
    /// Whether any retention hold on this class is disclosed.
    pub hold_disclosed: bool,
    /// Whether this row is disclosed rather than hidden.
    pub disclosed: bool,
}

/// Block asserting every memory class is addressed by the fan-out.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryClassCoverageBlock {
    /// Stable id of the coverage set.
    pub coverage_set_id: String,
    /// Whether every product memory class is present.
    pub all_classes_covered: bool,
    /// The per-class coverage rows.
    pub class_rows: Vec<MemoryClassCoverageRow>,
}

/// One explicit saved-memory entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplicitSavedMemoryRow {
    /// Stable id of the saved-memory entry.
    pub entry_id: String,
    /// Scope at which the memory was saved.
    pub scope: SavedMemoryScopeClass,
    /// Accountable actor that saved the memory.
    pub actor_class: SavedMemoryActorClass,
    /// Consent posture recorded for the entry.
    pub consent: SavedMemoryConsentClass,
    /// Whether the entry is revocable.
    pub revocable: bool,
    /// Whether the entry is disclosed rather than hidden.
    pub disclosed: bool,
}

/// Block of explicit saved-memory entries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExplicitSavedMemoryBlock {
    /// Stable id of the saved-memory set.
    pub saved_set_id: String,
    /// Whether every entry carries an accountable owner.
    pub owner_accountable: bool,
    /// Whether every entry is revocable.
    pub all_revocable: bool,
    /// The saved-memory entries.
    pub saved_rows: Vec<ExplicitSavedMemoryRow>,
}

/// One workspace- or tenant-scoped deletion or export operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedDeletionExportRow {
    /// Stable id of the operation.
    pub operation_id: String,
    /// Kind of operation.
    pub operation_kind: MemoryOperationKindClass,
    /// Scope at which the operation ran.
    pub scope: DeletionExportScopeClass,
    /// Per-class fan-out completeness.
    pub fan_out_completeness: FanOutCompletenessClass,
    /// Receipt verification state.
    pub receipt_verification: ReceiptVerificationClass,
    /// Receipt handle ref the operation produced.
    pub receipt_ref: String,
    /// Whether the operation addressed every memory class.
    pub all_classes_addressed: bool,
    /// Whether the operation is disclosed rather than hidden.
    pub disclosed: bool,
}

/// Block of scoped deletion and export operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedDeletionExportBlock {
    /// Stable id of the operation set.
    pub operation_set_id: String,
    /// Whether per-class fan-out is enforced on every operation.
    pub per_class_fan_out_enforced: bool,
    /// Whether a receipt is required for every operation.
    pub receipts_required: bool,
    /// The scoped operations.
    pub operation_rows: Vec<ScopedDeletionExportRow>,
}

/// Cross-surface consumer-parity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryPostureSurfaceParityRow {
    /// Consumer surface this row describes.
    pub surface: MemoryPostureConsumerSurface,
    /// Whether the surface projects memory-class coverage.
    pub shows_memory_classes: bool,
    /// Whether the surface projects explicit saved memory.
    pub shows_saved_memory: bool,
    /// Whether the surface projects scoped delete/export operations.
    pub shows_scoped_operations: bool,
    /// Whether the surface is reachable.
    pub reachable: bool,
    /// Qualification class of the projection.
    pub qualification: MemoryPostureQualificationClass,
    /// Whether the surface claims stable.
    pub claimed_stable: bool,
}

/// Stable-lane input used to mint a [`AiMemoryDeletionExportPosturePacket`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMemoryDeletionExportPosturePacketInput {
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical posture-run id shared across surfaces and evidence.
    pub posture_run_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace or tenant trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Memory-class coverage block.
    pub memory_class_coverage: MemoryClassCoverageBlock,
    /// Explicit saved-memory block.
    pub explicit_saved_memory: ExplicitSavedMemoryBlock,
    /// Scoped deletion/export operation block.
    pub scoped_operations: ScopedDeletionExportBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<MemoryPostureSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<MemoryPostureDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe AI memory deletion/export posture record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMemoryDeletionExportPosturePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id for this record.
    pub packet_id: String,
    /// Canonical posture-run id shared across surfaces and evidence.
    pub posture_run_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace or tenant trust-state token at mint time.
    pub trust_state_token: String,
    /// Policy epoch ref this packet was evaluated under.
    pub policy_epoch_ref: String,
    /// Memory-class coverage block.
    pub memory_class_coverage: MemoryClassCoverageBlock,
    /// Explicit saved-memory block.
    pub explicit_saved_memory: ExplicitSavedMemoryBlock,
    /// Scoped deletion/export operation block.
    pub scoped_operations: ScopedDeletionExportBlock,
    /// Cross-surface consumer-parity rows.
    pub consumer_surface_parity: Vec<MemoryPostureSurfaceParityRow>,
    /// Downgrade triggers that apply to this packet.
    pub downgrade_triggers: Vec<MemoryPostureDowngradeTrigger>,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiMemoryDeletionExportPosturePacket {
    /// Builds the packet from the stable-lane input.
    #[must_use]
    pub fn new(input: AiMemoryDeletionExportPosturePacketInput) -> Self {
        Self {
            record_kind: MEMORY_DELETION_EXPORT_RECORD_KIND.to_owned(),
            schema_version: MEMORY_DELETION_EXPORT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            posture_run_id: input.posture_run_id,
            display_label: input.display_label,
            trust_state_token: input.trust_state_token,
            policy_epoch_ref: input.policy_epoch_ref,
            memory_class_coverage: input.memory_class_coverage,
            explicit_saved_memory: input.explicit_saved_memory,
            scoped_operations: input.scoped_operations,
            consumer_surface_parity: input.consumer_surface_parity,
            downgrade_triggers: input.downgrade_triggers,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the packet's stable-line invariants.
    #[must_use]
    pub fn validate(&self) -> Vec<AiMemoryDeletionExportViolation> {
        let mut violations = Vec::new();
        if self.record_kind != MEMORY_DELETION_EXPORT_RECORD_KIND {
            violations.push(AiMemoryDeletionExportViolation::WrongRecordKind);
        }
        if self.schema_version != MEMORY_DELETION_EXPORT_SCHEMA_VERSION {
            violations.push(AiMemoryDeletionExportViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.posture_run_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.trust_state_token.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiMemoryDeletionExportViolation::MissingIdentity);
        }
        validate_source_contracts(self, &mut violations);
        validate_memory_class_coverage(self, &mut violations);
        validate_explicit_saved_memory(self, &mut violations);
        validate_scoped_operations(self, &mut violations);
        validate_consumer_surface_parity(self, &mut violations);
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("memory posture packet serializes"),
        ) {
            violations.push(AiMemoryDeletionExportViolation::RawBoundaryMaterialInExport);
        }
        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    #[must_use]
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("memory posture packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    #[must_use]
    pub fn render_markdown_summary(&self) -> String {
        let stable_surfaces = self
            .consumer_surface_parity
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let held_classes = self
            .memory_class_coverage
            .class_rows
            .iter()
            .filter(|row| row.retention_hold.holds_beyond_delete())
            .count();
        let verified_ops = self
            .scoped_operations
            .operation_rows
            .iter()
            .filter(|row| row.receipt_verification.is_verified())
            .count();
        let mut out = String::new();
        out.push_str(
            "# AI Memory Classes, Explicit Saved Memory, and Workspace/Tenant Deletion and Export Posture\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Posture run: `{}`\n", self.posture_run_id));
        out.push_str(&format!(
            "- Memory classes: `{}` ({} classes, all-covered: {}, {} retention-held)\n",
            self.memory_class_coverage.coverage_set_id,
            self.memory_class_coverage.class_rows.len(),
            self.memory_class_coverage.all_classes_covered,
            held_classes
        ));
        out.push_str(&format!(
            "- Saved memory: `{}` ({} entries, owner-accountable: {}, all-revocable: {})\n",
            self.explicit_saved_memory.saved_set_id,
            self.explicit_saved_memory.saved_rows.len(),
            self.explicit_saved_memory.owner_accountable,
            self.explicit_saved_memory.all_revocable
        ));
        out.push_str(&format!(
            "- Scoped operations: `{}` ({} operations, {} verified, per-class fan-out: {}, receipts required: {})\n",
            self.scoped_operations.operation_set_id,
            self.scoped_operations.operation_rows.len(),
            verified_ops,
            self.scoped_operations.per_class_fan_out_enforced,
            self.scoped_operations.receipts_required
        ));
        out.push_str(&format!(
            "- Surface parity: {} surfaces ({} stable)\n",
            self.consumer_surface_parity.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Downgrade triggers: {}\n",
            self.downgrade_triggers.len()
        ));
        out
    }
}

/// Validation failures emitted by
/// [`AiMemoryDeletionExportPosturePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiMemoryDeletionExportViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The memory-class coverage set has no rows.
    MemoryClassCoverageEmpty,
    /// A required product memory class is missing from the coverage rows.
    MemoryClassCoverageMissing,
    /// A memory-class coverage row is missing required identity.
    MemoryClassRowIncomplete,
    /// A coverage row is disclosed without being marked disclosed.
    HiddenMemoryClassRow,
    /// A class is not covered by the delete fan-out without a disclosed hold.
    DeleteFanOutIncomplete,
    /// A class is not covered by the export fan-out.
    ExportFanOutIncomplete,
    /// A retention hold is applied without disclosure.
    RetentionHoldUndisclosed,
    /// The saved-memory set claims accountable owners but is internally inconsistent.
    SavedMemorySetInconsistent,
    /// A saved-memory entry is missing required identity.
    SavedMemoryRowIncomplete,
    /// A saved-memory entry is disclosed without being marked disclosed.
    HiddenSavedMemoryRow,
    /// A saved-memory entry has no accountable owner.
    SavedMemoryWithoutAccountableOwner,
    /// A saved-memory entry is not consented.
    SavedMemoryNotConsented,
    /// A saved-memory entry is not revocable.
    SavedMemoryNotRevocable,
    /// The scoped-operation set has no operations.
    ScopedOperationSetEmpty,
    /// A scoped operation is missing required identity.
    ScopedOperationIncomplete,
    /// A scoped operation is disclosed without being marked disclosed.
    HiddenScopedOperation,
    /// A scoped operation lacks a receipt while receipts are required.
    ScopedOperationMissingReceipt,
    /// A claimed-complete operation does not address every class.
    ScopedOperationCompletenessOverstated,
    /// A claimed-complete operation does not carry a verified receipt.
    DeletionReceiptUnverified,
    /// A required consumer surface is missing from the parity rows.
    ConsumerSurfaceCoverageMissing,
    /// A surface claims stable while not reachable.
    StableClaimNotQualified,
    /// The export carries raw boundary material.
    RawBoundaryMaterialInExport,
}

/// Error returned when the checked-in export fails to load or validate.
#[derive(Debug)]
pub enum AiMemoryDeletionExportArtifactError {
    /// The checked-in support export failed to parse.
    SupportExport(serde_json::Error),
    /// The checked-in support export failed validation.
    Validation(Vec<AiMemoryDeletionExportViolation>),
}

impl fmt::Display for AiMemoryDeletionExportArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "memory posture support export failed to parse: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| format!("{violation:?}"))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "memory posture export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiMemoryDeletionExportArtifactError {}

/// Returns the checked-in memory deletion/export posture export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or
/// validate.
pub fn current_stable_ai_memory_deletion_export_posture_export(
) -> Result<AiMemoryDeletionExportPosturePacket, AiMemoryDeletionExportArtifactError> {
    let packet: AiMemoryDeletionExportPosturePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_ai_memory_classes_explicit_saved_memory_and_workspace_or_tenant_deletion_and_export_posture/support_export.json"
    )))
    .map_err(AiMemoryDeletionExportArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiMemoryDeletionExportArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &AiMemoryDeletionExportPosturePacket,
    violations: &mut Vec<AiMemoryDeletionExportViolation>,
) {
    for required in [
        MEMORY_DELETION_EXPORT_DOC_REF,
        MEMORY_DELETION_EXPORT_SCHEMA_REF,
        MEMORY_DELETION_EXPORT_DELETE_CONTRACT_REF,
        MEMORY_DELETION_EXPORT_RECONCILIATION_CONTRACT_REF,
        MEMORY_DELETION_EXPORT_MEMORY_OBJECT_SCHEMA_REF,
        MEMORY_DELETION_EXPORT_MEMORY_CLASSES_REF,
        MEMORY_DELETION_EXPORT_CONTEXT_ASSEMBLY_CONTRACT_REF,
        MEMORY_DELETION_EXPORT_M5_MATRIX_CONTRACT_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(AiMemoryDeletionExportViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_memory_class_coverage(
    packet: &AiMemoryDeletionExportPosturePacket,
    violations: &mut Vec<AiMemoryDeletionExportViolation>,
) {
    let coverage = &packet.memory_class_coverage;
    if coverage.coverage_set_id.trim().is_empty() || coverage.class_rows.is_empty() {
        violations.push(AiMemoryDeletionExportViolation::MemoryClassCoverageEmpty);
        return;
    }
    let mut seen = std::collections::HashSet::new();
    for row in &coverage.class_rows {
        seen.insert(row.state_class);
        if !row.disclosed {
            violations.push(AiMemoryDeletionExportViolation::HiddenMemoryClassRow);
        }
        // A class must be covered by the delete fan-out unless a disclosed
        // retention hold keeps an evidence/legal copy beyond the delete request.
        if !row.delete_fan_out_covered && !row.retention_hold.holds_beyond_delete() {
            violations.push(AiMemoryDeletionExportViolation::DeleteFanOutIncomplete);
        }
        if row.retention_hold.holds_beyond_delete() && !row.hold_disclosed {
            violations.push(AiMemoryDeletionExportViolation::RetentionHoldUndisclosed);
        }
        if !row.export_fan_out_covered {
            violations.push(AiMemoryDeletionExportViolation::ExportFanOutIncomplete);
        }
    }
    for required in AiStateClass::required_coverage() {
        if !seen.contains(&required) {
            violations.push(AiMemoryDeletionExportViolation::MemoryClassCoverageMissing);
            break;
        }
    }
    if coverage.all_classes_covered && seen.len() < AiStateClass::required_coverage().len() {
        violations.push(AiMemoryDeletionExportViolation::MemoryClassRowIncomplete);
    }
}

fn validate_explicit_saved_memory(
    packet: &AiMemoryDeletionExportPosturePacket,
    violations: &mut Vec<AiMemoryDeletionExportViolation>,
) {
    let saved = &packet.explicit_saved_memory;
    if saved.saved_set_id.trim().is_empty() {
        violations.push(AiMemoryDeletionExportViolation::SavedMemorySetInconsistent);
        return;
    }
    for entry in &saved.saved_rows {
        if entry.entry_id.trim().is_empty() {
            violations.push(AiMemoryDeletionExportViolation::SavedMemoryRowIncomplete);
        }
        if !entry.disclosed {
            violations.push(AiMemoryDeletionExportViolation::HiddenSavedMemoryRow);
        }
        if !entry.actor_class.is_accountable() {
            violations.push(AiMemoryDeletionExportViolation::SavedMemoryWithoutAccountableOwner);
        }
        if !entry.consent.is_permitted() {
            violations.push(AiMemoryDeletionExportViolation::SavedMemoryNotConsented);
        }
        if !entry.revocable {
            violations.push(AiMemoryDeletionExportViolation::SavedMemoryNotRevocable);
        }
    }
    if saved.owner_accountable
        && saved
            .saved_rows
            .iter()
            .any(|entry| !entry.actor_class.is_accountable())
    {
        violations.push(AiMemoryDeletionExportViolation::SavedMemorySetInconsistent);
    }
    if saved.all_revocable && saved.saved_rows.iter().any(|entry| !entry.revocable) {
        violations.push(AiMemoryDeletionExportViolation::SavedMemorySetInconsistent);
    }
}

fn validate_scoped_operations(
    packet: &AiMemoryDeletionExportPosturePacket,
    violations: &mut Vec<AiMemoryDeletionExportViolation>,
) {
    let operations = &packet.scoped_operations;
    if operations.operation_set_id.trim().is_empty() || operations.operation_rows.is_empty() {
        violations.push(AiMemoryDeletionExportViolation::ScopedOperationSetEmpty);
        return;
    }
    for operation in &operations.operation_rows {
        if operation.operation_id.trim().is_empty() {
            violations.push(AiMemoryDeletionExportViolation::ScopedOperationIncomplete);
        }
        if !operation.disclosed {
            violations.push(AiMemoryDeletionExportViolation::HiddenScopedOperation);
        }
        if operations.receipts_required && operation.receipt_ref.trim().is_empty() {
            violations.push(AiMemoryDeletionExportViolation::ScopedOperationMissingReceipt);
        }
        if operation.fan_out_completeness.claims_complete() {
            if !operation.all_classes_addressed {
                violations
                    .push(AiMemoryDeletionExportViolation::ScopedOperationCompletenessOverstated);
            }
            if !operation.receipt_verification.is_verified() {
                violations.push(AiMemoryDeletionExportViolation::DeletionReceiptUnverified);
            }
        }
    }
}

fn validate_consumer_surface_parity(
    packet: &AiMemoryDeletionExportPosturePacket,
    violations: &mut Vec<AiMemoryDeletionExportViolation>,
) {
    let mut seen = std::collections::HashSet::new();
    for row in &packet.consumer_surface_parity {
        seen.insert(row.surface);
        if row.claimed_stable && !row.reachable {
            violations.push(AiMemoryDeletionExportViolation::StableClaimNotQualified);
        }
    }
    for required in MemoryPostureConsumerSurface::ALL {
        if !seen.contains(&required) {
            violations.push(AiMemoryDeletionExportViolation::ConsumerSurfaceCoverageMissing);
            break;
        }
    }
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
        || lower.contains('@')
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("oauth_token")
        || lower.contains("bearer ")
        || lower.contains("billing-account")
        || lower.contains("raw_prompt")
        || lower.contains("/users/")
}
