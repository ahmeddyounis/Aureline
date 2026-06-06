//! Stable AI memory-state, cache, delete, and export truth records.
//!
//! This module is the stable-control projection for AI thread state, prompt
//! result caches, reusable repo facts, retained evidence copies, and explicit
//! saved memory. It does not implement a memory store. Instead it freezes the
//! typed support-safe packet that shell, Help/About, support export, and later
//! memory-runtime code read so no surface has to infer AI retention behavior
//! from generic history settings.
//!
//! The packet composes with the broader memory-object contract
//! ([`docs/ai/memory_and_reconciliation_contract.md`](../../../docs/ai/memory_and_reconciliation_contract.md))
//! and the existing memory-object schema
//! ([`schemas/ai/memory_object.schema.json`](../../../schemas/ai/memory_object.schema.json)).
//! It narrows those lower-level rows into the six product-facing classes named
//! by the stable lane and adds UI projection, durable-cache key, invalidation,
//! delete/export drill, and support-safe manifest checks.
//!
//! The record is export-safe. It carries class names, refs, policy labels,
//! reason codes, hashes, counts, and disclosure tokens only. Raw prompts,
//! responses, terminal transcripts, credentials, disallowed path contents, raw
//! vectors, raw provider payloads, and raw support bodies stay outside this
//! boundary.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AiMemoryStatePacket`].
pub const AI_MEMORY_STATE_RECORD_KIND: &str = "ai_memory_state";

/// Schema version for stable AI memory-state packets.
pub const AI_MEMORY_STATE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the AI memory-state boundary schema.
pub const AI_MEMORY_STATE_SCHEMA_REF: &str = "schemas/ai/ai-memory-state.schema.json";

/// Repo-relative path of the stable AI memory delete/export contract doc.
pub const AI_MEMORY_STATE_AI_DOC_REF: &str = "docs/ai/ai-memory-delete-export.md";

/// Repo-relative path of the stable AI memory retention matrix.
pub const AI_MEMORY_STATE_MATRIX_REF: &str = "artifacts/ai/m4/ai-memory-retention-matrix.md";

/// Repo-relative path of the checked stable AI memory-state export.
pub const AI_MEMORY_STATE_ARTIFACT_REF: &str =
    "artifacts/ai/m4/ai_memory_state/support_export.json";

/// Repo-relative path of the lower-level AI memory-object schema.
pub const AI_MEMORY_OBJECT_SCHEMA_REF: &str = "schemas/ai/memory_object.schema.json";

/// Repo-relative path of the lower-level AI memory/reconciliation contract.
pub const AI_MEMORY_RECONCILIATION_CONTRACT_REF: &str =
    "docs/ai/memory_and_reconciliation_contract.md";

/// AI state classes exposed on stable product surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiStateClass {
    /// In-flight prompt assembly, selected context, tool output, and candidate edits.
    TurnState,
    /// User-visible conversation thread, attachments, and explicit thread notes.
    ConversationThread,
    /// Prompt/result cache used for bounded reuse of generated or retrieved results.
    PromptResultCache,
    /// Workspace-scoped derived repo facts, summaries, and docs-derived snippets.
    ReusableRepoFactsSummaries,
    /// Evidence-governed retained copy for review, support, audit, or incident cases.
    RetainedEvidenceCopy,
    /// User-, repo-, or org-scoped memory explicitly saved by an accountable actor.
    ExplicitSavedMemory,
}

impl AiStateClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TurnState => "turn_state",
            Self::ConversationThread => "conversation_thread",
            Self::PromptResultCache => "prompt_result_cache",
            Self::ReusableRepoFactsSummaries => "reusable_repo_facts_summaries",
            Self::RetainedEvidenceCopy => "retained_evidence_copy",
            Self::ExplicitSavedMemory => "explicit_saved_memory",
        }
    }

    /// Classes required before any claimed stable AI memory surface can ship.
    pub const fn required_coverage() -> [Self; 6] {
        [
            Self::TurnState,
            Self::ConversationThread,
            Self::PromptResultCache,
            Self::ReusableRepoFactsSummaries,
            Self::RetainedEvidenceCopy,
            Self::ExplicitSavedMemory,
        ]
    }

    const fn is_cache_like(self) -> bool {
        matches!(
            self,
            Self::PromptResultCache | Self::ReusableRepoFactsSummaries
        )
    }

    const fn is_reusable(self) -> bool {
        matches!(
            self,
            Self::PromptResultCache | Self::ReusableRepoFactsSummaries | Self::ExplicitSavedMemory
        )
    }
}

/// Scope boundary a memory class is allowed to occupy by default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryScopeClass {
    /// Process/session/workspace-local state.
    SessionWorkspace,
    /// User-visible state bound to one user and one workspace or repo.
    UserWorkspaceRepo,
    /// Cache state bound to workspace/repo, feature, provider, and model identities.
    WorkspaceFeatureProviderModel,
    /// Derived repo facts bound to one workspace, or tenant+repo only with policy.
    WorkspaceOrTenantRepoPolicy,
    /// Action-scoped evidence state.
    ActionScoped,
    /// Explicit saved state scoped to user, repo, or org labels.
    UserRepoOrgExplicit,
}

impl MemoryScopeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionWorkspace => "session_workspace",
            Self::UserWorkspaceRepo => "user_workspace_repo",
            Self::WorkspaceFeatureProviderModel => "workspace_feature_provider_model",
            Self::WorkspaceOrTenantRepoPolicy => "workspace_or_tenant_repo_policy",
            Self::ActionScoped => "action_scoped",
            Self::UserRepoOrgExplicit => "user_repo_org_explicit",
        }
    }
}

/// Durability posture for a memory class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DurabilityClass {
    /// Memory-only until completion or a short bounded TTL.
    Ephemeral,
    /// Local user-visible durability when saved or required by policy.
    LocalDurable,
    /// Durable only as a TTL-bounded derived cache.
    TtlBoundedCache,
    /// Regeneratable derived state, not authoritative user content.
    DerivedRegeneratable,
    /// Retained only because evidence policy requires a bounded copy.
    EvidencePolicyRetained,
    /// Retained until the accountable user or admin removes it.
    ExplicitlySaved,
}

impl DurabilityClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ephemeral => "ephemeral",
            Self::LocalDurable => "local_durable",
            Self::TtlBoundedCache => "ttl_bounded_cache",
            Self::DerivedRegeneratable => "derived_regeneratable",
            Self::EvidencePolicyRetained => "evidence_policy_retained",
            Self::ExplicitlySaved => "explicitly_saved",
        }
    }
}

/// Retention label shown by thread headers, inspectors, docs, and exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionModeClass {
    /// Dropped on action completion or after a short bounded TTL.
    EphemeralUntilCompletion,
    /// Kept with the local thread until delete or policy expiry.
    LocalThreadUntilDelete,
    /// Kept only while the cache TTL and cache-key identities remain valid.
    TtlBoundedUntilInvalidated,
    /// Regeneratable derived row kept until graph/docs/policy/workspace invalidation.
    DerivedUntilInvalidated,
    /// Retained under evidence expiry, case close, or policy hold.
    EvidencePolicyExpiryOrCaseClose,
    /// Retained until the user or admin removes the saved memory.
    UntilUserOrAdminRemoves,
}

impl RetentionModeClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralUntilCompletion => "ephemeral_until_completion",
            Self::LocalThreadUntilDelete => "local_thread_until_delete",
            Self::TtlBoundedUntilInvalidated => "ttl_bounded_until_invalidated",
            Self::DerivedUntilInvalidated => "derived_until_invalidated",
            Self::EvidencePolicyExpiryOrCaseClose => "evidence_policy_expiry_or_case_close",
            Self::UntilUserOrAdminRemoves => "until_user_or_admin_removes",
        }
    }
}

/// Sensitivity tier controlling whether reusable memory is allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SensitivityTierClass {
    /// Metadata only.
    T0MetadataOnly,
    /// Low-risk derived state such as docs-derived summaries.
    T1LowRiskDerived,
    /// Code-bearing bounded material that stays local unless policy says otherwise.
    T2CodeBearingBounded,
    /// Sensitive or secret-adjacent material that never becomes reusable memory.
    T3SensitiveSecretAdjacent,
}

impl SensitivityTierClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::T0MetadataOnly => "t0_metadata_only",
            Self::T1LowRiskDerived => "t1_low_risk_derived",
            Self::T2CodeBearingBounded => "t2_code_bearing_bounded",
            Self::T3SensitiveSecretAdjacent => "t3_sensitive_secret_adjacent",
        }
    }
}

/// Accountable owner and policy semantics for a memory class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerPolicyClass {
    /// State belongs to the active user/session and has no durable owner.
    ActiveUserSession,
    /// User-visible local thread state governed by workspace policy.
    UserWorkspacePolicy,
    /// Cache-manager owned derived state governed by policy epoch and TTL.
    WorkspaceCachePolicy,
    /// Regeneratable fact store governed by workspace or explicit tenant+repo policy.
    WorkspaceFactPolicy,
    /// Evidence-governed copy with retention, hold, and export rules.
    EvidenceRetentionPolicy,
    /// User, repo owner, or org admin explicitly owns the saved memory.
    ExplicitOwnerPolicy,
}

impl OwnerPolicyClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveUserSession => "active_user_session",
            Self::UserWorkspacePolicy => "user_workspace_policy",
            Self::WorkspaceCachePolicy => "workspace_cache_policy",
            Self::WorkspaceFactPolicy => "workspace_fact_policy",
            Self::EvidenceRetentionPolicy => "evidence_retention_policy",
            Self::ExplicitOwnerPolicy => "explicit_owner_policy",
        }
    }
}

/// Delete posture for a memory class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeletePostureClass {
    /// No durable copy exists beyond an optional evidence disclosure.
    NoDurableState,
    /// Deletes the user-visible thread and emits a destruction receipt.
    DeleteThreadWithReceipt,
    /// Invalidates cache entries by key class.
    InvalidateCacheByKeyClass,
    /// Deletes or preserves evidence according to retention and hold rules.
    EvidenceRetentionRules,
    /// Deletes explicitly saved memory and emits an owner-scoped receipt.
    DeleteSavedMemoryWithReceipt,
}

impl DeletePostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDurableState => "no_durable_state",
            Self::DeleteThreadWithReceipt => "delete_thread_with_receipt",
            Self::InvalidateCacheByKeyClass => "invalidate_cache_by_key_class",
            Self::EvidenceRetentionRules => "evidence_retention_rules",
            Self::DeleteSavedMemoryWithReceipt => "delete_saved_memory_with_receipt",
        }
    }
}

/// Export posture for a memory class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportPostureClass {
    /// Not exported because the class is active-turn only.
    NotExportableEphemeral,
    /// Exported as the user-visible conversation form.
    UserVisibleConversation,
    /// Exported as inventory and hashes, not raw prompt/response bodies.
    InventoryAndHashesOnly,
    /// Exported as provenance-labeled derived metadata.
    ProvenanceLabeledSummary,
    /// Exported through the retained evidence packet.
    EvidencePacket,
    /// Exported as the explicit saved-memory object.
    ExplicitSavedMemoryObject,
}

impl ExportPostureClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotExportableEphemeral => "not_exportable_ephemeral",
            Self::UserVisibleConversation => "user_visible_conversation",
            Self::InventoryAndHashesOnly => "inventory_and_hashes_only",
            Self::ProvenanceLabeledSummary => "provenance_labeled_summary",
            Self::EvidencePacket => "evidence_packet",
            Self::ExplicitSavedMemoryObject => "explicit_saved_memory_object",
        }
    }
}

/// Cache-key component required for durable AI caches.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CacheKeyComponentClass {
    /// Workspace identity ref.
    WorkspaceIdentity,
    /// Repository identity ref.
    RepoIdentity,
    /// Feature class ref.
    FeatureClass,
    /// Provider id and model version.
    ProviderModelVersion,
    /// Prompt-pack version or digest.
    PromptPackVersion,
    /// Tool-schema version or digest.
    ToolSchemaVersion,
    /// Policy epoch ref.
    PolicyEpoch,
    /// Graph/docs epoch ref.
    GraphDocsEpoch,
    /// Retention posture ref.
    RetentionPosture,
}

impl CacheKeyComponentClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceIdentity => "workspace_identity",
            Self::RepoIdentity => "repo_identity",
            Self::FeatureClass => "feature_class",
            Self::ProviderModelVersion => "provider_model_version",
            Self::PromptPackVersion => "prompt_pack_version",
            Self::ToolSchemaVersion => "tool_schema_version",
            Self::PolicyEpoch => "policy_epoch",
            Self::GraphDocsEpoch => "graph_docs_epoch",
            Self::RetentionPosture => "retention_posture",
        }
    }
}

/// Invalidation reason codes surfaced by cache and memory inspectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InvalidationReasonCode {
    /// Workspace identity changed.
    WorkspaceIdentityChanged,
    /// Repository identity changed.
    RepoIdentityChanged,
    /// Organization, tenant, account, or profile changed.
    OrgTenantProfileChanged,
    /// Workspace trust posture changed.
    WorkspaceTrustChanged,
    /// Provider or model version changed.
    ProviderModelVersionChanged,
    /// Prompt-pack version changed.
    PromptPackChanged,
    /// Tool-schema version changed.
    ToolSchemaChanged,
    /// Policy epoch rolled.
    PolicyEpochRolled,
    /// Graph or docs epoch changed.
    GraphDocsEpochChanged,
    /// Retention posture changed.
    RetentionPostureChanged,
    /// User or admin delete request was received.
    DeleteRequestReceived,
}

impl InvalidationReasonCode {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceIdentityChanged => "workspace_identity_changed",
            Self::RepoIdentityChanged => "repo_identity_changed",
            Self::OrgTenantProfileChanged => "org_tenant_profile_changed",
            Self::WorkspaceTrustChanged => "workspace_trust_changed",
            Self::ProviderModelVersionChanged => "provider_model_version_changed",
            Self::PromptPackChanged => "prompt_pack_changed",
            Self::ToolSchemaChanged => "tool_schema_changed",
            Self::PolicyEpochRolled => "policy_epoch_rolled",
            Self::GraphDocsEpochChanged => "graph_docs_epoch_changed",
            Self::RetentionPostureChanged => "retention_posture_changed",
            Self::DeleteRequestReceived => "delete_request_received",
        }
    }
}

/// Product surface that must consume the same memory vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemorySurfaceClass {
    /// Thread header scope chips.
    ThreadHeader,
    /// Memory inventory inspector.
    MemoryInspector,
    /// Delete/export review sheet.
    DeleteExportReview,
    /// Help/About truth surface.
    HelpAbout,
    /// Support export manifest.
    SupportExport,
}

impl MemorySurfaceClass {
    /// Stable token used in exports and fixtures.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThreadHeader => "thread_header",
            Self::MemoryInspector => "memory_inspector",
            Self::DeleteExportReview => "delete_export_review",
            Self::HelpAbout => "help_about",
            Self::SupportExport => "support_export",
        }
    }

    /// Surfaces required for stable memory-vocabulary parity.
    pub const fn required_coverage() -> [Self; 5] {
        [
            Self::ThreadHeader,
            Self::MemoryInspector,
            Self::DeleteExportReview,
            Self::HelpAbout,
            Self::SupportExport,
        ]
    }
}

/// One stable AI memory class row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryStateClassRow {
    /// Stable memory class.
    pub state_class: AiStateClass,
    /// Review-safe class label.
    pub label: String,
    /// Default scope class.
    pub scope_class: MemoryScopeClass,
    /// Default durability class.
    pub durability_class: DurabilityClass,
    /// Default retention mode.
    pub retention_mode: RetentionModeClass,
    /// Sensitivity tier.
    pub sensitivity_tier: SensitivityTierClass,
    /// Accountable owner and policy semantics.
    pub owner_policy: OwnerPolicyClass,
    /// Delete posture.
    pub delete_posture: DeletePostureClass,
    /// Export posture.
    pub export_posture: ExportPostureClass,
    /// Cache-key components required before reuse.
    pub invalidation_key_components: Vec<CacheKeyComponentClass>,
    /// Machine-readable invalidation reason codes.
    pub invalidation_reason_codes: Vec<InvalidationReasonCode>,
    /// True when broader reuse requires an explicit policy and scope label.
    pub broader_reuse_requires_policy_label: bool,
    /// True when raw prompt/response bodies are excluded from default export.
    pub raw_prompt_response_bodies_excluded_by_default: bool,
    /// True when T3 secret-adjacent material is barred from reusable memory.
    pub t3_reusable_memory_forbidden: bool,
    /// Review-safe disclosure shown on inspectors and export sheets.
    pub disclosure_label: String,
}

/// One product surface projection row for memory truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemorySurfaceProjectionRow {
    /// Product surface class.
    pub surface_class: MemorySurfaceClass,
    /// Projection ref.
    pub projection_ref: String,
    /// True when the surface uses [`AiStateClass`] names.
    pub uses_memory_class_vocabulary: bool,
    /// True when scope chips are visible without hidden settings.
    pub shows_scope_chip: bool,
    /// True when provider/model is visible where a thread or run can dispatch.
    pub shows_provider_model: bool,
    /// True when retention mode is visible.
    pub shows_retention_mode: bool,
    /// True when saved-memory owner and policy are visible.
    pub shows_saved_memory_owner_policy: bool,
    /// True when retained evidence copies are disclosed.
    pub discloses_retained_evidence_copy: bool,
    /// True when delete/export actions or refs are directly reachable.
    pub delete_export_reachable: bool,
}

impl MemorySurfaceProjectionRow {
    fn complete(&self) -> bool {
        self.uses_memory_class_vocabulary
            && self.shows_scope_chip
            && self.shows_provider_model
            && self.shows_retention_mode
            && self.shows_saved_memory_owner_policy
            && self.discloses_retained_evidence_copy
            && self.delete_export_reachable
            && !self.projection_ref.trim().is_empty()
    }
}

/// Durable cache-key contract for prompt/result caches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DurableCacheKeyContract {
    /// Stable cache contract id.
    pub cache_contract_id: String,
    /// State class covered by this cache contract.
    pub state_class: AiStateClass,
    /// Key components required on the durable cache key.
    pub required_components: Vec<CacheKeyComponentClass>,
    /// Invalidation reason codes emitted for matching drift.
    pub invalidation_reason_codes: Vec<InvalidationReasonCode>,
    /// True when cache entries are scoped to a workspace/repo and cannot cross profiles.
    pub workspace_repo_profile_scoped: bool,
    /// True when delete-thread or delete-workspace AI-state requests invalidate matches.
    pub delete_request_invalidates_matching_entries: bool,
    /// True when raw cache bodies stay outside support exports.
    pub support_export_hashes_only: bool,
}

/// Reusable-memory admission fence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReusableMemoryFence {
    /// Stable fence id.
    pub fence_id: String,
    /// State class protected by the fence.
    pub state_class: AiStateClass,
    /// Whether T3 sensitive or secret-adjacent material is denied.
    pub denies_t3_secret_adjacent: bool,
    /// Whether raw terminal transcripts are denied.
    pub denies_raw_terminal_transcripts: bool,
    /// Whether credentials and credential handles are denied.
    pub denies_credentials: bool,
    /// Whether disallowed path contents are denied.
    pub denies_disallowed_path_contents: bool,
    /// Whether only redaction-reviewed evidence packets may retain bounded copies.
    pub retained_copy_requires_reviewed_redaction_packet: bool,
    /// Review-safe policy ref.
    pub policy_ref: String,
}

impl ReusableMemoryFence {
    fn complete(&self) -> bool {
        self.state_class.is_reusable()
            && self.denies_t3_secret_adjacent
            && self.denies_raw_terminal_transcripts
            && self.denies_credentials
            && self.denies_disallowed_path_contents
            && self.retained_copy_requires_reviewed_redaction_packet
            && !self.fence_id.trim().is_empty()
            && !self.policy_ref.trim().is_empty()
    }
}

/// Delete/export drill row for one user or admin action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeleteExportDrillRow {
    /// Stable drill id.
    pub drill_id: String,
    /// Memory classes selected for this drill.
    pub selected_classes: Vec<AiStateClass>,
    /// Memory classes excluded because policy owns retention.
    pub excluded_by_policy: Vec<AiStateClass>,
    /// True when matching durable cache keys are invalidated.
    pub invalidates_matching_durable_caches: bool,
    /// True when retained copies are labeled instead of hidden.
    pub retained_copies_labeled: bool,
    /// True when export is available before destructive delete.
    pub export_before_delete_available: bool,
    /// True when a destruction receipt or omission reason is emitted.
    pub receipt_or_omission_reason_emitted: bool,
}

impl DeleteExportDrillRow {
    fn complete(&self) -> bool {
        !self.drill_id.trim().is_empty()
            && !self.selected_classes.is_empty()
            && self.invalidates_matching_durable_caches
            && self.retained_copies_labeled
            && self.export_before_delete_available
            && self.receipt_or_omission_reason_emitted
    }
}

/// Support-safe export manifest for AI memory state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportSafeMemoryManifest {
    /// Manifest ref.
    pub manifest_ref: String,
    /// Conversation-history inventory ref.
    pub conversation_history_inventory_ref: String,
    /// Reusable-facts inventory ref.
    pub reusable_facts_inventory_ref: String,
    /// Retained-evidence inventory ref.
    pub retained_evidence_inventory_ref: String,
    /// Cache inventory hash ref.
    pub cache_inventory_hash_ref: String,
    /// True when raw prompt bodies are excluded by default.
    pub raw_prompt_bodies_excluded: bool,
    /// True when raw response bodies are excluded by default.
    pub raw_response_bodies_excluded: bool,
    /// True when raw terminal transcripts are excluded by default.
    pub raw_terminal_transcripts_excluded: bool,
    /// True when credentials are excluded by default.
    pub credentials_excluded: bool,
}

impl SupportSafeMemoryManifest {
    fn complete(&self) -> bool {
        !self.manifest_ref.trim().is_empty()
            && !self.conversation_history_inventory_ref.trim().is_empty()
            && !self.reusable_facts_inventory_ref.trim().is_empty()
            && !self.retained_evidence_inventory_ref.trim().is_empty()
            && !self.cache_inventory_hash_ref.trim().is_empty()
            && self.raw_prompt_bodies_excluded
            && self.raw_response_bodies_excluded
            && self.raw_terminal_transcripts_excluded
            && self.credentials_excluded
    }
}

/// Constructor input for [`AiMemoryStatePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiMemoryStatePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace identity ref.
    pub workspace_identity_ref: String,
    /// Repository identity ref.
    pub repo_identity_ref: String,
    /// Profile identity ref.
    pub profile_identity_ref: String,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Provider/model ref shown on thread-capable surfaces.
    pub provider_model_ref: String,
    /// Memory class rows.
    pub memory_classes: Vec<MemoryStateClassRow>,
    /// Surface projection rows.
    pub surface_projections: Vec<MemorySurfaceProjectionRow>,
    /// Durable cache-key contracts.
    pub durable_cache_key_contracts: Vec<DurableCacheKeyContract>,
    /// Reusable-memory fences.
    pub reusable_memory_fences: Vec<ReusableMemoryFence>,
    /// Delete/export drills.
    pub delete_export_drills: Vec<DeleteExportDrillRow>,
    /// Support-safe manifest.
    pub support_manifest: SupportSafeMemoryManifest,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe stable AI memory-state record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiMemoryStatePacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Display label.
    pub display_label: String,
    /// Workspace identity ref.
    pub workspace_identity_ref: String,
    /// Repository identity ref.
    pub repo_identity_ref: String,
    /// Profile identity ref.
    pub profile_identity_ref: String,
    /// Policy epoch ref.
    pub policy_epoch_ref: String,
    /// Provider/model ref shown on thread-capable surfaces.
    pub provider_model_ref: String,
    /// Memory class rows.
    pub memory_classes: Vec<MemoryStateClassRow>,
    /// Surface projection rows.
    pub surface_projections: Vec<MemorySurfaceProjectionRow>,
    /// Durable cache-key contracts.
    pub durable_cache_key_contracts: Vec<DurableCacheKeyContract>,
    /// Reusable-memory fences.
    pub reusable_memory_fences: Vec<ReusableMemoryFence>,
    /// Delete/export drills.
    pub delete_export_drills: Vec<DeleteExportDrillRow>,
    /// Support-safe manifest.
    pub support_manifest: SupportSafeMemoryManifest,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Overall packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AiMemoryStatePacket {
    /// Builds a stable AI memory-state packet.
    pub fn new(input: AiMemoryStatePacketInput) -> Self {
        Self {
            record_kind: AI_MEMORY_STATE_RECORD_KIND.to_owned(),
            schema_version: AI_MEMORY_STATE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            display_label: input.display_label,
            workspace_identity_ref: input.workspace_identity_ref,
            repo_identity_ref: input.repo_identity_ref,
            profile_identity_ref: input.profile_identity_ref,
            policy_epoch_ref: input.policy_epoch_ref,
            provider_model_ref: input.provider_model_ref,
            memory_classes: input.memory_classes,
            surface_projections: input.surface_projections,
            durable_cache_key_contracts: input.durable_cache_key_contracts,
            reusable_memory_fences: input.reusable_memory_fences,
            delete_export_drills: input.delete_export_drills,
            support_manifest: input.support_manifest,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the stable AI memory-state invariants.
    pub fn validate(&self) -> Vec<AiMemoryStateViolation> {
        let mut violations = Vec::new();
        if self.record_kind != AI_MEMORY_STATE_RECORD_KIND {
            violations.push(AiMemoryStateViolation::WrongRecordKind);
        }
        if self.schema_version != AI_MEMORY_STATE_SCHEMA_VERSION {
            violations.push(AiMemoryStateViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.display_label.trim().is_empty()
            || self.workspace_identity_ref.trim().is_empty()
            || self.repo_identity_ref.trim().is_empty()
            || self.profile_identity_ref.trim().is_empty()
            || self.policy_epoch_ref.trim().is_empty()
            || self.provider_model_ref.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AiMemoryStateViolation::MissingIdentity);
        }
        validate_memory_class_coverage(self, &mut violations);
        validate_surface_coverage(self, &mut violations);
        validate_cache_contracts(self, &mut violations);
        validate_reusable_memory_fences(self, &mut violations);
        validate_delete_export_drills(self, &mut violations);
        if !self.support_manifest.complete() {
            violations.push(AiMemoryStateViolation::SupportManifestUnsafe);
        }
        if !required_refs_present(
            &self.source_contract_refs,
            [
                AI_MEMORY_STATE_AI_DOC_REF,
                AI_MEMORY_STATE_MATRIX_REF,
                AI_MEMORY_STATE_SCHEMA_REF,
                AI_MEMORY_OBJECT_SCHEMA_REF,
                AI_MEMORY_RECONCILIATION_CONTRACT_REF,
            ],
        ) {
            violations.push(AiMemoryStateViolation::MissingSourceContracts);
        }
        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).unwrap_or(serde_json::Value::Null),
        ) {
            violations.push(AiMemoryStateViolation::RawBoundaryMaterialInExport);
        }
        violations.sort_by_key(|violation| violation.as_str());
        violations.dedup();
        violations
    }
}

/// Errors emitted when reading the checked-in stable AI memory-state export.
#[derive(Debug)]
pub enum AiMemoryStateArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiMemoryStateViolation>),
}

impl fmt::Display for AiMemoryStateArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "ai memory-state export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "ai memory-state export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AiMemoryStateArtifactError {}

/// Validation failures emitted by [`AiMemoryStatePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AiMemoryStateViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required memory class is absent.
    MemoryClassCoverageMissing,
    /// A class row has incomplete class semantics.
    MemoryClassSemanticsIncomplete,
    /// A reusable class admits cross-workspace or cross-tenant reuse by default.
    HiddenCrossWorkspaceReuse,
    /// A durable cache key is missing required components or reason codes.
    DurableCacheKeyIncomplete,
    /// A product surface does not carry the shared vocabulary and disclosures.
    SurfaceProjectionIncomplete,
    /// T3 or secret-adjacent material can become reusable memory.
    T3ReusableMemoryAllowed,
    /// Delete/export drills do not invalidate caches or label retained copies.
    DeleteExportDrillIncomplete,
    /// Support-safe manifest can export raw bodies or collapses inventories.
    SupportManifestUnsafe,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AiMemoryStateViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MemoryClassCoverageMissing => "memory_class_coverage_missing",
            Self::MemoryClassSemanticsIncomplete => "memory_class_semantics_incomplete",
            Self::HiddenCrossWorkspaceReuse => "hidden_cross_workspace_reuse",
            Self::DurableCacheKeyIncomplete => "durable_cache_key_incomplete",
            Self::SurfaceProjectionIncomplete => "surface_projection_incomplete",
            Self::T3ReusableMemoryAllowed => "t3_reusable_memory_allowed",
            Self::DeleteExportDrillIncomplete => "delete_export_drill_incomplete",
            Self::SupportManifestUnsafe => "support_manifest_unsafe",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the checked-in stable AI memory-state export.
///
/// # Errors
///
/// Returns an artifact error if the checked-in export does not parse or validate.
pub fn current_stable_ai_memory_state_export(
) -> Result<AiMemoryStatePacket, AiMemoryStateArtifactError> {
    let packet: AiMemoryStatePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m4/ai_memory_state/support_export.json"
    )))
    .map_err(AiMemoryStateArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiMemoryStateArtifactError::Validation(violations))
    }
}

fn validate_memory_class_coverage(
    packet: &AiMemoryStatePacket,
    violations: &mut Vec<AiMemoryStateViolation>,
) {
    for required in AiStateClass::required_coverage() {
        let Some(row) = packet
            .memory_classes
            .iter()
            .find(|row| row.state_class == required)
        else {
            violations.push(AiMemoryStateViolation::MemoryClassCoverageMissing);
            continue;
        };

        if row.label.trim().is_empty()
            || row.disclosure_label.trim().is_empty()
            || !row.raw_prompt_response_bodies_excluded_by_default
        {
            violations.push(AiMemoryStateViolation::MemoryClassSemanticsIncomplete);
        }
        if row.state_class.is_reusable() && !row.broader_reuse_requires_policy_label {
            violations.push(AiMemoryStateViolation::HiddenCrossWorkspaceReuse);
        }
        if row.state_class.is_reusable()
            && row.sensitivity_tier == SensitivityTierClass::T3SensitiveSecretAdjacent
        {
            violations.push(AiMemoryStateViolation::T3ReusableMemoryAllowed);
        }
        if !row.t3_reusable_memory_forbidden {
            violations.push(AiMemoryStateViolation::T3ReusableMemoryAllowed);
        }
        if row.state_class.is_cache_like() && row.invalidation_key_components.is_empty() {
            violations.push(AiMemoryStateViolation::DurableCacheKeyIncomplete);
        }
    }
}

fn validate_surface_coverage(
    packet: &AiMemoryStatePacket,
    violations: &mut Vec<AiMemoryStateViolation>,
) {
    for required in MemorySurfaceClass::required_coverage() {
        let Some(row) = packet
            .surface_projections
            .iter()
            .find(|row| row.surface_class == required)
        else {
            violations.push(AiMemoryStateViolation::SurfaceProjectionIncomplete);
            continue;
        };
        if !row.complete() {
            violations.push(AiMemoryStateViolation::SurfaceProjectionIncomplete);
        }
    }
}

fn validate_cache_contracts(
    packet: &AiMemoryStatePacket,
    violations: &mut Vec<AiMemoryStateViolation>,
) {
    let required_components = [
        CacheKeyComponentClass::WorkspaceIdentity,
        CacheKeyComponentClass::RepoIdentity,
        CacheKeyComponentClass::FeatureClass,
        CacheKeyComponentClass::ProviderModelVersion,
        CacheKeyComponentClass::PromptPackVersion,
        CacheKeyComponentClass::ToolSchemaVersion,
        CacheKeyComponentClass::PolicyEpoch,
        CacheKeyComponentClass::GraphDocsEpoch,
        CacheKeyComponentClass::RetentionPosture,
    ];
    let required_reasons = [
        InvalidationReasonCode::WorkspaceIdentityChanged,
        InvalidationReasonCode::RepoIdentityChanged,
        InvalidationReasonCode::OrgTenantProfileChanged,
        InvalidationReasonCode::WorkspaceTrustChanged,
        InvalidationReasonCode::ProviderModelVersionChanged,
        InvalidationReasonCode::PromptPackChanged,
        InvalidationReasonCode::ToolSchemaChanged,
        InvalidationReasonCode::PolicyEpochRolled,
        InvalidationReasonCode::GraphDocsEpochChanged,
        InvalidationReasonCode::RetentionPostureChanged,
        InvalidationReasonCode::DeleteRequestReceived,
    ];
    let Some(contract) = packet
        .durable_cache_key_contracts
        .iter()
        .find(|contract| contract.state_class == AiStateClass::PromptResultCache)
    else {
        violations.push(AiMemoryStateViolation::DurableCacheKeyIncomplete);
        return;
    };
    if contract.cache_contract_id.trim().is_empty()
        || !required_components
            .iter()
            .all(|component| contract.required_components.contains(component))
        || !required_reasons
            .iter()
            .all(|reason| contract.invalidation_reason_codes.contains(reason))
        || !contract.workspace_repo_profile_scoped
        || !contract.delete_request_invalidates_matching_entries
        || !contract.support_export_hashes_only
    {
        violations.push(AiMemoryStateViolation::DurableCacheKeyIncomplete);
    }
}

fn validate_reusable_memory_fences(
    packet: &AiMemoryStatePacket,
    violations: &mut Vec<AiMemoryStateViolation>,
) {
    for required in [
        AiStateClass::PromptResultCache,
        AiStateClass::ReusableRepoFactsSummaries,
        AiStateClass::ExplicitSavedMemory,
    ] {
        let Some(fence) = packet
            .reusable_memory_fences
            .iter()
            .find(|fence| fence.state_class == required)
        else {
            violations.push(AiMemoryStateViolation::T3ReusableMemoryAllowed);
            continue;
        };
        if !fence.complete() {
            violations.push(AiMemoryStateViolation::T3ReusableMemoryAllowed);
        }
    }
}

fn validate_delete_export_drills(
    packet: &AiMemoryStatePacket,
    violations: &mut Vec<AiMemoryStateViolation>,
) {
    if packet.delete_export_drills.len() < 2
        || !packet.delete_export_drills.iter().all(|d| d.complete())
    {
        violations.push(AiMemoryStateViolation::DeleteExportDrillIncomplete);
        return;
    }
    let deletes_thread = packet.delete_export_drills.iter().any(|drill| {
        drill
            .selected_classes
            .contains(&AiStateClass::ConversationThread)
            && drill
                .selected_classes
                .contains(&AiStateClass::PromptResultCache)
    });
    let deletes_workspace = packet.delete_export_drills.iter().any(|drill| {
        drill
            .selected_classes
            .contains(&AiStateClass::ReusableRepoFactsSummaries)
            && drill
                .selected_classes
                .contains(&AiStateClass::ExplicitSavedMemory)
    });
    if !deletes_thread || !deletes_workspace {
        violations.push(AiMemoryStateViolation::DeleteExportDrillIncomplete);
    }
}

fn required_refs_present<const N: usize>(refs: &[String], required: [&str; N]) -> bool {
    required
        .iter()
        .all(|required_ref| refs.iter().any(|reference| reference == required_ref))
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
        || lower.contains("raw_prompt")
        || lower.contains("raw_response")
        || lower.contains("terminal transcript")
        || lower.contains("bearer ")
        || lower.contains("oauth")
        || lower.contains("api_key")
        || lower.contains("api-key")
        || lower.contains("credential value")
        || lower.contains("/users/")
        || lower.contains("private_key")
}

#[cfg(test)]
mod tests;
