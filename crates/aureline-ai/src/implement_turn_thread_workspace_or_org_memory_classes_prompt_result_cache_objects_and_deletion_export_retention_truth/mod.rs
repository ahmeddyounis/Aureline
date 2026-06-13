//! Materialized turn/thread/workspace/org AI memory classes, prompt-result-cache
//! objects, and per-object deletion/export/retention truth.
//!
//! Where the frozen recall matrix
//! ([`crate::freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix`])
//! qualifies whole surfaces, this module materializes the individual memory
//! objects those surfaces hold. Each [`MemoryClassObjectRecord`] binds one
//! concrete memory object to its scope (turn, thread, workspace, or org), its
//! artifact class (ephemeral turn state, evictable derived cache, prompt-result
//! cache, reusable semantic memory, or durable saved memory), its retention,
//! delete, and export posture, its locality, its invalidation classes, the
//! consumer flows that read it (composer, review, docs, agent), and a precise
//! availability label.
//!
//! The packet preserves the four-way distinction the spec requires: ephemeral
//! state, evictable derived cache, reusable semantic memory, and durable saved
//! memory never collapse into one another. A durable object must declare a real
//! retention, delete, and export posture; an ephemeral object must auto-expire;
//! and a prompt-result cache is keyed by content hash with a bounded lifetime so
//! it can never masquerade as a durable shadow telemetry store. A missing or
//! policy-blocked object degrades to a precise label
//! ([`MemoryAvailabilityClass`] plus [`MemoryClassObjectRecord::degraded_label`])
//! rather than one generic "memory unavailable" state, and
//! [`MemoryClassMaterializationPacket::validate`] rejects any record whose
//! declared posture outruns its class.
//!
//! Raw prompt bodies, cached result bodies, raw embeddings, raw provider
//! payloads, credentials, exact token counts, and exact cost amounts never cross
//! this boundary.
//!
//! The boundary schema is
//! [`schemas/ai/implement-turn-thread-workspace-or-org-memory-classes-prompt-result-cache-objects-and-deletion-export-retention-truth.schema.json`](../../../../schemas/ai/implement-turn-thread-workspace-or-org-memory-classes-prompt-result-cache-objects-and-deletion-export-retention-truth.schema.json).
//! The contract doc is
//! [`docs/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md`](../../../../docs/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/`](../../../../fixtures/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`MemoryClassMaterializationPacket`].
pub const MEMORY_CLASS_MATERIALIZATION_RECORD_KIND: &str =
    "implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth";

/// Schema version for memory-class materialization records.
pub const MEMORY_CLASS_MATERIALIZATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF: &str =
    "schemas/ai/implement-turn-thread-workspace-or-org-memory-classes-prompt-result-cache-objects-and-deletion-export-retention-truth.schema.json";

/// Repo-relative path of the contract doc.
pub const MEMORY_CLASS_MATERIALIZATION_DOC_REF: &str =
    "docs/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md";

/// Repo-relative path of the frozen recall matrix contract this packet realizes.
pub const MEMORY_CLASS_MATERIALIZATION_RECALL_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md";

/// Repo-relative path of the frozen memory-class matrix contract.
pub const MEMORY_CLASS_MATERIALIZATION_MEMORY_CLASS_CONTRACT_REF: &str =
    "docs/ai/memory_class_matrix.md";

/// Repo-relative path of the frozen memory delete/export contract.
pub const MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF: &str =
    "docs/ai/ai-memory-delete-export.md";

/// Repo-relative path of the frozen spend/route receipt contract.
pub const MEMORY_CLASS_MATERIALIZATION_SPEND_RECEIPT_CONTRACT_REF: &str =
    "docs/ai/spend_and_route_receipt_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const MEMORY_CLASS_MATERIALIZATION_FIXTURE_DIR: &str =
    "fixtures/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth";

/// Repo-relative path of the checked support-export artifact.
pub const MEMORY_CLASS_MATERIALIZATION_ARTIFACT_REF: &str =
    "artifacts/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MEMORY_CLASS_MATERIALIZATION_SUMMARY_REF: &str =
    "artifacts/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth.md";

/// Scope class an AI memory object is bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryScopeClass {
    /// Single AI turn; cleared when the turn ends.
    Turn,
    /// One conversation thread; cleared when the thread closes.
    Thread,
    /// Workspace-scoped; no cross-workspace recall by default.
    Workspace,
    /// Org/tenant-scoped; no cross-tenant recall by default.
    Org,
}

impl MemoryScopeClass {
    /// Every scope class, in widening order.
    pub const ALL: [Self; 4] = [Self::Turn, Self::Thread, Self::Workspace, Self::Org];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Turn => "turn",
            Self::Thread => "thread",
            Self::Workspace => "workspace",
            Self::Org => "org",
        }
    }
}

/// Artifact class that fixes how a memory object persists and is reused.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryArtifactClass {
    /// Ephemeral, session-scoped working state; never durably retained.
    EphemeralTurnState,
    /// Evictable derived cache; recomputable, not authoritative.
    EvictableDerivedCache,
    /// Prompt-result cache keyed by content hash with a bounded lifetime.
    PromptResultCache,
    /// Durable reusable semantic memory.
    ReusableSemanticMemory,
    /// Durable explicit saved memory.
    DurableSavedMemory,
}

impl MemoryArtifactClass {
    /// Every artifact class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::EphemeralTurnState,
        Self::EvictableDerivedCache,
        Self::PromptResultCache,
        Self::ReusableSemanticMemory,
        Self::DurableSavedMemory,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EphemeralTurnState => "ephemeral_turn_state",
            Self::EvictableDerivedCache => "evictable_derived_cache",
            Self::PromptResultCache => "prompt_result_cache",
            Self::ReusableSemanticMemory => "reusable_semantic_memory",
            Self::DurableSavedMemory => "durable_saved_memory",
        }
    }

    /// Whether this class persists beyond a single session and so must declare a
    /// real retention, delete, and export posture.
    pub const fn is_durable(self) -> bool {
        matches!(
            self,
            Self::EvictableDerivedCache
                | Self::PromptResultCache
                | Self::ReusableSemanticMemory
                | Self::DurableSavedMemory
        )
    }

    /// Whether this class is a derived cache that must stay evictable rather than
    /// be treated as an authoritative durable store.
    pub const fn is_evictable_cache(self) -> bool {
        matches!(self, Self::EvictableDerivedCache | Self::PromptResultCache)
    }
}

/// Retention class declared by a memory object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryRetentionClass {
    /// Retained only for the current session; dropped on session end.
    SessionOnly,
    /// Retained for a bounded time-to-live, then auto-evicted.
    TtlBounded,
    /// Retained until manually evicted or purged.
    UntilManualEvict,
    /// Retained until the user revokes consent.
    UntilUserRevoked,
    /// Retained under a disclosed evidence/governance hold.
    EvidenceRetentionHold,
    /// Retained durably until explicitly deleted.
    DurableUntilDeleted,
}

impl MemoryRetentionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionOnly => "session_only",
            Self::TtlBounded => "ttl_bounded",
            Self::UntilManualEvict => "until_manual_evict",
            Self::UntilUserRevoked => "until_user_revoked",
            Self::EvidenceRetentionHold => "evidence_retention_hold",
            Self::DurableUntilDeleted => "durable_until_deleted",
        }
    }

    /// Whether this retention class outlives a single session.
    pub const fn is_durable(self) -> bool {
        !matches!(self, Self::SessionOnly)
    }
}

/// Delete or export posture for a memory object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryDeleteExportPosture {
    /// User-initiated delete or export of their own copy.
    UserScoped,
    /// Workspace-scoped delete or export.
    WorkspaceScoped,
    /// Tenant-scoped delete or export.
    TenantScoped,
    /// Org-scoped delete or export.
    OrgScoped,
    /// Object auto-expires; explicit delete/export is not required.
    EphemeralAutoExpire,
    /// Not applicable; object holds nothing durable to delete or export.
    NotApplicable,
}

impl MemoryDeleteExportPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserScoped => "user_scoped",
            Self::WorkspaceScoped => "workspace_scoped",
            Self::TenantScoped => "tenant_scoped",
            Self::OrgScoped => "org_scoped",
            Self::EphemeralAutoExpire => "ephemeral_auto_expire",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this posture names a real, actionable delete/export operation.
    pub const fn is_actionable(self) -> bool {
        matches!(
            self,
            Self::UserScoped | Self::WorkspaceScoped | Self::TenantScoped | Self::OrgScoped
        )
    }
}

/// Locality posture for the data a memory object holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryLocalityClass {
    /// Data stays on the local device only.
    LocalDeviceOnly,
    /// Data is scoped to the workspace; no cross-workspace recall.
    WorkspaceLocal,
    /// Data is tenant-scoped and region-pinned; no cross-tenant recall.
    TenantRegionPinned,
    /// Data is managed-hosted and region-pinned within the tenant.
    ManagedHostedRegionPinned,
}

impl MemoryLocalityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDeviceOnly => "local_device_only",
            Self::WorkspaceLocal => "workspace_local",
            Self::TenantRegionPinned => "tenant_region_pinned",
            Self::ManagedHostedRegionPinned => "managed_hosted_region_pinned",
        }
    }
}

/// Cache or memory invalidation class keeping recall from masquerading as truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryInvalidationClass {
    /// Time-to-live expiry.
    TtlExpiry,
    /// Content-hash keying invalidates on input change.
    ContentHashKey,
    /// Policy-epoch bump invalidates prior entries.
    PolicyEpochBump,
    /// Trust narrowing invalidates wider-trust entries.
    TrustNarrowing,
    /// Embedding-generation bump invalidates mixed-generation entries.
    EmbeddingGenerationBump,
    /// Scope revocation drops entries when scope or consent is revoked.
    ScopeRevocation,
    /// Manual purge by the user or operator.
    ManualPurge,
}

impl MemoryInvalidationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TtlExpiry => "ttl_expiry",
            Self::ContentHashKey => "content_hash_key",
            Self::PolicyEpochBump => "policy_epoch_bump",
            Self::TrustNarrowing => "trust_narrowing",
            Self::EmbeddingGenerationBump => "embedding_generation_bump",
            Self::ScopeRevocation => "scope_revocation",
            Self::ManualPurge => "manual_purge",
        }
    }
}

/// Consumer flow that reads a memory object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryConsumerFlow {
    /// Composer inline assist and prompt-composer recall.
    ComposerAssist,
    /// Patch / code review assist.
    PatchReview,
    /// Docs and in-app browser recall.
    DocsBrowserRecall,
    /// Branch or worktree background agent.
    BranchAgent,
    /// Support / export packet projection.
    SupportExport,
}

impl MemoryConsumerFlow {
    /// Every consumer flow, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ComposerAssist,
        Self::PatchReview,
        Self::DocsBrowserRecall,
        Self::BranchAgent,
        Self::SupportExport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComposerAssist => "composer_assist",
            Self::PatchReview => "patch_review",
            Self::DocsBrowserRecall => "docs_browser_recall",
            Self::BranchAgent => "branch_agent",
            Self::SupportExport => "support_export",
        }
    }
}

/// Availability of a memory object, with precise labels rather than one generic
/// "memory unavailable" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryAvailabilityClass {
    /// Object is materialized and available.
    Available,
    /// Blocked by a retention-class policy gate.
    PolicyBlockedRetentionGate,
    /// Blocked by a region/locality policy gate.
    PolicyBlockedRegionGate,
    /// Unavailable because the storage backend is unsupported on this build.
    UnavailableUnsupportedBackend,
    /// Narrowed because the backing proof has gone stale.
    NarrowedStaleProof,
    /// Revoked by the user; consent withdrawn.
    RevokedByUser,
    /// Auto-evicted because the retention window elapsed.
    ExpiredAutoEvicted,
}

impl MemoryAvailabilityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::PolicyBlockedRetentionGate => "policy_blocked_retention_gate",
            Self::PolicyBlockedRegionGate => "policy_blocked_region_gate",
            Self::UnavailableUnsupportedBackend => "unavailable_unsupported_backend",
            Self::NarrowedStaleProof => "narrowed_stale_proof",
            Self::RevokedByUser => "revoked_by_user",
            Self::ExpiredAutoEvicted => "expired_auto_evicted",
        }
    }

    /// Whether the object is fully available.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

/// One materialized AI memory object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryClassObjectRecord {
    /// Stable object id.
    pub object_id: String,
    /// Scope class the object is bound to.
    pub scope: MemoryScopeClass,
    /// Artifact class fixing how the object persists and is reused.
    pub artifact_class: MemoryArtifactClass,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Retention class declared by the object.
    pub retention: MemoryRetentionClass,
    /// Delete posture for the object.
    pub delete_posture: MemoryDeleteExportPosture,
    /// Export posture for the object.
    pub export_posture: MemoryDeleteExportPosture,
    /// Locality posture for the data the object holds.
    pub locality: MemoryLocalityClass,
    /// Invalidation classes that apply to the object.
    pub invalidation_classes: Vec<MemoryInvalidationClass>,
    /// Availability class.
    pub availability: MemoryAvailabilityClass,
    /// Precise degraded label, required when [`Self::availability`] is not
    /// [`MemoryAvailabilityClass::Available`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Consumer flows that read this object.
    pub consumer_flows: Vec<MemoryConsumerFlow>,
    /// Evidence packet refs backing this object.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this object.
    pub source_contract_refs: Vec<String>,
}

impl MemoryClassObjectRecord {
    /// Whether the object declares a real delete and export posture appropriate
    /// for its artifact class.
    ///
    /// A durable class must name an actionable delete and export posture; an
    /// ephemeral class must auto-expire (or declare not-applicable).
    pub fn delete_export_consistent(&self) -> bool {
        if self.artifact_class.is_durable() {
            self.delete_posture.is_actionable() && self.export_posture.is_actionable()
        } else {
            matches!(
                self.delete_posture,
                MemoryDeleteExportPosture::EphemeralAutoExpire
                    | MemoryDeleteExportPosture::NotApplicable
            ) && matches!(
                self.export_posture,
                MemoryDeleteExportPosture::EphemeralAutoExpire
                    | MemoryDeleteExportPosture::NotApplicable
            )
        }
    }

    /// Whether the retention class agrees with the artifact class.
    ///
    /// A durable class must not claim session-only retention, and an ephemeral
    /// class must not claim a durable retention class.
    pub fn retention_consistent(&self) -> bool {
        self.artifact_class.is_durable() == self.retention.is_durable()
    }

    /// Whether the object's locality stays within its scope, never widening.
    pub fn locality_within_scope(&self) -> bool {
        match self.scope {
            MemoryScopeClass::Turn | MemoryScopeClass::Thread | MemoryScopeClass::Workspace => {
                matches!(
                    self.locality,
                    MemoryLocalityClass::LocalDeviceOnly | MemoryLocalityClass::WorkspaceLocal
                )
            }
            MemoryScopeClass::Org => matches!(
                self.locality,
                MemoryLocalityClass::TenantRegionPinned
                    | MemoryLocalityClass::ManagedHostedRegionPinned
            ),
        }
    }

    /// Whether a prompt-result cache object stays bounded and content-keyed
    /// rather than behaving like a durable shadow telemetry store.
    pub fn prompt_result_cache_bounded(&self) -> bool {
        if self.artifact_class != MemoryArtifactClass::PromptResultCache {
            return true;
        }
        let bounded_retention = matches!(
            self.retention,
            MemoryRetentionClass::TtlBounded | MemoryRetentionClass::UntilManualEvict
        );
        let content_keyed = self
            .invalidation_classes
            .contains(&MemoryInvalidationClass::ContentHashKey);
        let ttl_keyed = self
            .invalidation_classes
            .contains(&MemoryInvalidationClass::TtlExpiry);
        bounded_retention && content_keyed && ttl_keyed
    }

    /// Whether a non-available object carries a precise, non-generic label.
    pub fn availability_labeled(&self) -> bool {
        if self.availability.is_available() {
            return true;
        }
        match &self.degraded_label {
            Some(label) => !label_is_generic(label),
            None => false,
        }
    }

    /// Whether every dimension required to materialize this object is present.
    pub fn is_complete(&self) -> bool {
        !self.object_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && !self.invalidation_classes.is_empty()
            && !self.consumer_flows.is_empty()
            && !self.evidence_refs.is_empty()
            && !self.source_contract_refs.is_empty()
            && self.delete_export_consistent()
            && self.retention_consistent()
            && self.locality_within_scope()
            && self.prompt_result_cache_bounded()
            && self.availability_labeled()
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryClassGuardrails {
    /// No cross-workspace memory is created by default.
    pub no_cross_workspace_memory_by_default: bool,
    /// No cross-tenant memory is created by default.
    pub no_cross_tenant_memory_by_default: bool,
    /// Prompt-result caches are not used as shadow telemetry stores.
    pub prompt_result_caches_not_shadow_telemetry: bool,
    /// Every durable class declares its retention, delete, and export posture.
    pub every_durable_class_declares_retention_delete_export: bool,
    /// Ephemeral state stays separated from durable saved memory.
    pub ephemeral_state_separated_from_durable_memory: bool,
    /// Missing or blocked classes degrade to precise labels.
    pub missing_classes_degrade_to_precise_labels: bool,
    /// Mixed retrieval/embedding generations are always labeled.
    pub mixed_retrieval_generations_labeled: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryClassConsumerProjection {
    /// Composer shows which memory classes were used.
    pub composer_shows_memory_classes: bool,
    /// Review shows which memory classes were used.
    pub review_shows_memory_classes: bool,
    /// Docs/browser shows which memory classes were used.
    pub docs_browser_shows_memory_classes: bool,
    /// Agent flow shows which memory classes were used.
    pub agent_flow_shows_memory_classes: bool,
    /// Support export preserves the class distinctions.
    pub support_export_preserves_class_distinctions: bool,
    /// Unavailable or blocked classes are labeled precisely, never generic.
    pub unavailable_classes_labeled_precisely: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryClassProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the object.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`MemoryClassMaterializationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryClassMaterializationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable materialization label.
    pub materialization_label: String,
    /// Materialized memory objects.
    pub memory_objects: Vec<MemoryClassObjectRecord>,
    /// Guardrail invariants block.
    pub guardrails: MemoryClassGuardrails,
    /// Consumer projection block.
    pub consumer_projection: MemoryClassConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MemoryClassProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe materialized memory-class packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryClassMaterializationPacket {
    /// Record kind; must equal [`MEMORY_CLASS_MATERIALIZATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MEMORY_CLASS_MATERIALIZATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable materialization label.
    pub materialization_label: String,
    /// Materialized memory objects.
    pub memory_objects: Vec<MemoryClassObjectRecord>,
    /// Guardrail invariants block.
    pub guardrails: MemoryClassGuardrails,
    /// Consumer projection block.
    pub consumer_projection: MemoryClassConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MemoryClassProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl MemoryClassMaterializationPacket {
    /// Builds a materialized memory-class packet from object input.
    pub fn new(input: MemoryClassMaterializationPacketInput) -> Self {
        Self {
            record_kind: MEMORY_CLASS_MATERIALIZATION_RECORD_KIND.to_owned(),
            schema_version: MEMORY_CLASS_MATERIALIZATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            materialization_label: input.materialization_label,
            memory_objects: input.memory_objects,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Scopes materialized by this packet.
    pub fn materialized_scopes(&self) -> BTreeSet<MemoryScopeClass> {
        self.memory_objects.iter().map(|obj| obj.scope).collect()
    }

    /// Artifact classes materialized by this packet.
    pub fn materialized_classes(&self) -> BTreeSet<MemoryArtifactClass> {
        self.memory_objects
            .iter()
            .map(|obj| obj.artifact_class)
            .collect()
    }

    /// Validates the materialized memory-class invariants.
    pub fn validate(&self) -> Vec<MemoryClassMaterializationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MEMORY_CLASS_MATERIALIZATION_RECORD_KIND {
            violations.push(MemoryClassMaterializationViolation::WrongRecordKind);
        }
        if self.schema_version != MEMORY_CLASS_MATERIALIZATION_SCHEMA_VERSION {
            violations.push(MemoryClassMaterializationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.materialization_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(MemoryClassMaterializationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_objects(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("memory class materialization packet serializes"),
        ) {
            violations.push(MemoryClassMaterializationViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("memory class materialization packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let available = self
            .memory_objects
            .iter()
            .filter(|obj| obj.availability.is_available())
            .count();
        let mut out = String::new();
        out.push_str("# Materialized AI Memory Classes\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.materialization_label));
        out.push_str(&format!(
            "- Objects: {} ({} available)\n",
            self.memory_objects.len(),
            available
        ));
        out.push_str(&format!(
            "- Scopes: {} / Classes: {}\n",
            self.materialized_scopes().len(),
            self.materialized_classes().len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Objects\n\n");
        for obj in &self.memory_objects {
            out.push_str(&format!(
                "- **{}** ({} / {}): `{}`\n",
                obj.object_id,
                obj.scope.as_str(),
                obj.artifact_class.as_str(),
                obj.availability.as_str()
            ));
            out.push_str(&format!("  - {}\n", obj.label_summary));
            out.push_str(&format!("  - Retention: `{}`\n", obj.retention.as_str()));
            out.push_str(&format!(
                "  - Delete/export: `{}` / `{}`\n",
                obj.delete_posture.as_str(),
                obj.export_posture.as_str()
            ));
            out.push_str(&format!("  - Locality: `{}`\n", obj.locality.as_str()));
            if let Some(label) = &obj.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in materialized memory-class export.
#[derive(Debug)]
pub enum MemoryClassMaterializationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MemoryClassMaterializationViolation>),
}

impl fmt::Display for MemoryClassMaterializationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "memory class materialization export parse failed: {error}"
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
                    "memory class materialization export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MemoryClassMaterializationArtifactError {}

/// Validation failures emitted by [`MemoryClassMaterializationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryClassMaterializationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required scope class is missing from the materialization.
    RequiredScopeMissing,
    /// A required artifact class is missing from the materialization.
    RequiredArtifactClassMissing,
    /// A memory object is incomplete.
    ObjectIncomplete,
    /// A durable object does not declare an actionable delete/export posture.
    DurableObjectMissingDeleteExport,
    /// An object's retention class disagrees with its artifact class.
    RetentionClassMismatch,
    /// An object's locality widens beyond its scope.
    LocalityExceedsScope,
    /// A prompt-result cache is not bounded and content-keyed.
    PromptResultCacheUnbounded,
    /// A non-available object lacks a precise degraded label.
    AvailabilityLabelMissing,
    /// An object has no consumer flows.
    ConsumerFlowsMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl MemoryClassMaterializationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredScopeMissing => "required_scope_missing",
            Self::RequiredArtifactClassMissing => "required_artifact_class_missing",
            Self::ObjectIncomplete => "object_incomplete",
            Self::DurableObjectMissingDeleteExport => "durable_object_missing_delete_export",
            Self::RetentionClassMismatch => "retention_class_mismatch",
            Self::LocalityExceedsScope => "locality_exceeds_scope",
            Self::PromptResultCacheUnbounded => "prompt_result_cache_unbounded",
            Self::AvailabilityLabelMissing => "availability_label_missing",
            Self::ConsumerFlowsMissing => "consumer_flows_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable materialized memory-class export.
pub fn current_stable_memory_class_materialization_export(
) -> Result<MemoryClassMaterializationPacket, MemoryClassMaterializationArtifactError> {
    let packet: MemoryClassMaterializationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth/support_export.json"
    )))
    .map_err(MemoryClassMaterializationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MemoryClassMaterializationArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &MemoryClassMaterializationPacket,
    violations: &mut Vec<MemoryClassMaterializationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MEMORY_CLASS_MATERIALIZATION_SCHEMA_REF,
        MEMORY_CLASS_MATERIALIZATION_DOC_REF,
        MEMORY_CLASS_MATERIALIZATION_RECALL_MATRIX_CONTRACT_REF,
        MEMORY_CLASS_MATERIALIZATION_MEMORY_CLASS_CONTRACT_REF,
        MEMORY_CLASS_MATERIALIZATION_DELETE_EXPORT_CONTRACT_REF,
        MEMORY_CLASS_MATERIALIZATION_SPEND_RECEIPT_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(MemoryClassMaterializationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_coverage(
    packet: &MemoryClassMaterializationPacket,
    violations: &mut Vec<MemoryClassMaterializationViolation>,
) {
    let scopes = packet.materialized_scopes();
    for required in MemoryScopeClass::ALL {
        if !scopes.contains(&required) {
            violations.push(MemoryClassMaterializationViolation::RequiredScopeMissing);
            break;
        }
    }
    let classes = packet.materialized_classes();
    for required in MemoryArtifactClass::ALL {
        if !classes.contains(&required) {
            violations.push(MemoryClassMaterializationViolation::RequiredArtifactClassMissing);
            break;
        }
    }
}

fn validate_objects(
    packet: &MemoryClassMaterializationPacket,
    violations: &mut Vec<MemoryClassMaterializationViolation>,
) {
    for obj in &packet.memory_objects {
        if !obj.is_complete() {
            violations.push(MemoryClassMaterializationViolation::ObjectIncomplete);
        }
        if !obj.delete_export_consistent() {
            violations.push(MemoryClassMaterializationViolation::DurableObjectMissingDeleteExport);
        }
        if !obj.retention_consistent() {
            violations.push(MemoryClassMaterializationViolation::RetentionClassMismatch);
        }
        if !obj.locality_within_scope() {
            violations.push(MemoryClassMaterializationViolation::LocalityExceedsScope);
        }
        if !obj.prompt_result_cache_bounded() {
            violations.push(MemoryClassMaterializationViolation::PromptResultCacheUnbounded);
        }
        if !obj.availability_labeled() {
            violations.push(MemoryClassMaterializationViolation::AvailabilityLabelMissing);
        }
        if obj.consumer_flows.is_empty() {
            violations.push(MemoryClassMaterializationViolation::ConsumerFlowsMissing);
        }
    }
}

fn validate_guardrails(
    packet: &MemoryClassMaterializationPacket,
    violations: &mut Vec<MemoryClassMaterializationViolation>,
) {
    let guardrails = &packet.guardrails;
    let ok = guardrails.no_cross_workspace_memory_by_default
        && guardrails.no_cross_tenant_memory_by_default
        && guardrails.prompt_result_caches_not_shadow_telemetry
        && guardrails.every_durable_class_declares_retention_delete_export
        && guardrails.ephemeral_state_separated_from_durable_memory
        && guardrails.missing_classes_degrade_to_precise_labels
        && guardrails.mixed_retrieval_generations_labeled;
    if !ok {
        violations.push(MemoryClassMaterializationViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &MemoryClassMaterializationPacket,
    violations: &mut Vec<MemoryClassMaterializationViolation>,
) {
    let projection = &packet.consumer_projection;
    let ok = projection.composer_shows_memory_classes
        && projection.review_shows_memory_classes
        && projection.docs_browser_shows_memory_classes
        && projection.agent_flow_shows_memory_classes
        && projection.support_export_preserves_class_distinctions
        && projection.unavailable_classes_labeled_precisely;
    if !ok {
        violations.push(MemoryClassMaterializationViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_proof_freshness(
    packet: &MemoryClassMaterializationPacket,
    violations: &mut Vec<MemoryClassMaterializationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(MemoryClassMaterializationViolation::ProofFreshnessIncomplete);
    }
}

/// Whether a degraded label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "memory unavailable"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "memory error"
            | "failed"
    )
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
