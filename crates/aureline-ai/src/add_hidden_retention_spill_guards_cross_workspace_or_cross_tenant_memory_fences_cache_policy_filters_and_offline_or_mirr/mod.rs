//! Hidden-retention spill guards, cross-workspace-or-cross-tenant memory fences,
//! cache-policy filters, and offline-or-mirror-safe retrieval fallback truth
//! across claimed M5 profiles.
//!
//! Where the frozen recall matrix
//! ([`crate::freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix`])
//! qualifies whole surfaces, the materialized memory-class lane
//! ([`crate::implement_turn_thread_workspace_or_org_memory_classes_prompt_result_cache_objects_and_deletion_export_retention_truth`])
//! materializes the per-scope memory objects, and the semantic-recall records lane
//! ([`crate::ship_reusable_semantic_memory_and_embedding_index_records_with_epoch_invalidation_locality_and_no_mixed_generation_truth`])
//! materializes the derived retrieval artifacts, this module locks the *policy
//! posture* every memory or cache object carries on a claimed M5 profile. Each
//! [`MemoryFenceFallbackRow`] binds one governed artifact on one profile to four
//! interlocking truths the spec requires.
//!
//! First, a **hidden-retention spill guard** ([`RetentionSpillGuard`]) keeps a
//! prompt-result cache or evictable derived cache content-keyed and
//! lifetime-bounded with export to telemetry refused, so a cache can never become
//! a hidden retention or shadow-telemetry store; a cache that would retain past
//! its bound is [`SpillState::WouldSpillBlocked`] and one that would feed
//! telemetry is [`SpillState::ShadowStoreBlocked`]. Second, a
//! **cross-workspace-or-cross-tenant memory fence** ([`MemoryFence`]) enforces and
//! discloses that no recall crosses a workspace or tenant boundary by default.
//! Third, a **cache-policy filter** ([`CachePolicyFilter`]) discloses what a
//! policy, region, retention-floor, tenant-isolation, or BYOK boundary narrows and
//! why, rather than silently dropping rows. Fourth, an
//! **offline-or-mirror-safe retrieval fallback** ([`RetrievalFallback`]) carries an
//! ordered fallback chain that ends in a non-AI terminal and keeps a precise label
//! on any non-primary lane, so a spend or route failure never collapses into a
//! generic provider error when a more precise mirror, offline, cached, or
//! policy-blocked fallback exists.
//!
//! [`MemoryFenceFallbackPacket::validate`] rejects any row whose declared posture
//! outruns its spill, fence, filter, or fallback evidence.
//!
//! Raw prompt bodies, cached result bodies, raw embeddings, raw provider payloads,
//! credentials, raw endpoint URLs, exact token counts, and exact cost amounts
//! never cross this boundary.
//!
//! The boundary schema is
//! [`schemas/ai/add-hidden-retention-spill-guards-cross-workspace-or-cross-tenant-memory-fences-cache-policy-filters-and-offline-or-mirr.schema.json`](../../../../schemas/ai/add-hidden-retention-spill-guards-cross-workspace-or-cross-tenant-memory-fences-cache-policy-filters-and-offline-or-mirr.schema.json).
//! The contract doc is
//! [`docs/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr.md`](../../../../docs/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr.md).
//! The protected fixture directory is
//! [`fixtures/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr/`](../../../../fixtures/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`MemoryFenceFallbackPacket`].
pub const MEMORY_FENCE_FALLBACK_RECORD_KIND: &str =
    "add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr";

/// Schema version for the memory-fence-and-fallback packet.
pub const MEMORY_FENCE_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MEMORY_FENCE_FALLBACK_SCHEMA_REF: &str =
    "schemas/ai/add-hidden-retention-spill-guards-cross-workspace-or-cross-tenant-memory-fences-cache-policy-filters-and-offline-or-mirr.schema.json";

/// Repo-relative path of the contract doc.
pub const MEMORY_FENCE_FALLBACK_DOC_REF: &str =
    "docs/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr.md";

/// Repo-relative path of the frozen recall matrix contract this packet hardens.
pub const MEMORY_FENCE_FALLBACK_RECALL_MATRIX_CONTRACT_REF: &str =
    "docs/ai/m5/freeze_the_m5_ai_memory_prompt_result_cache_hybrid_retrieval_and_retrieval_locality_matrix.md";

/// Repo-relative path of the frozen memory-class matrix contract.
pub const MEMORY_FENCE_FALLBACK_MEMORY_CLASS_CONTRACT_REF: &str = "docs/ai/memory_class_matrix.md";

/// Repo-relative path of the frozen memory delete/export contract.
pub const MEMORY_FENCE_FALLBACK_DELETE_EXPORT_CONTRACT_REF: &str =
    "docs/ai/ai-memory-delete-export.md";

/// Repo-relative path of the frozen context-assembly contract (retrieval input).
pub const MEMORY_FENCE_FALLBACK_CONTEXT_ASSEMBLY_CONTRACT_REF: &str =
    "docs/ai/context_assembly_contract.md";

/// Repo-relative path of the frozen retrieval identity/ranking contract.
pub const MEMORY_FENCE_FALLBACK_RETRIEVAL_CONTRACT_REF: &str =
    "docs/search/result_identity_and_ranking.md";

/// Repo-relative path of the routing/cost alpha contract (fallback chain truth).
pub const MEMORY_FENCE_FALLBACK_ROUTING_CONTRACT_REF: &str = "docs/ai/routing_cost_alpha.md";

/// Repo-relative path of the protected fixture directory.
pub const MEMORY_FENCE_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr";

/// Repo-relative path of the checked support-export artifact.
pub const MEMORY_FENCE_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MEMORY_FENCE_FALLBACK_SUMMARY_REF: &str =
    "artifacts/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr.md";

/// Claimed M5 deployment profile a row is governed under.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5Profile {
    /// On-device only; no request leaves the machine.
    LocalOnly,
    /// Vendor reached through the user's own credential.
    ByokDirect,
    /// First-party managed endpoint.
    ManagedHosted,
    /// On-device execution from a mirrored or offline pack channel.
    OfflineMirror,
    /// Managed control plane with a local or BYOK execution leg.
    HybridManaged,
}

impl M5Profile {
    /// Every claimed profile, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOnly,
        Self::ByokDirect,
        Self::ManagedHosted,
        Self::OfflineMirror,
        Self::HybridManaged,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ByokDirect => "byok_direct",
            Self::ManagedHosted => "managed_hosted",
            Self::OfflineMirror => "offline_mirror",
            Self::HybridManaged => "hybrid_managed",
        }
    }

    /// Whether this profile must keep retrieval generatable offline or mirrored.
    pub const fn requires_offline_safe(self) -> bool {
        matches!(self, Self::LocalOnly | Self::OfflineMirror)
    }
}

/// Memory or cache artifact class a row governs.
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

    /// Whether this class is a cache that must be content-keyed and bounded so it
    /// can never become a hidden retention or shadow-telemetry store.
    pub const fn is_bounded_cache(self) -> bool {
        matches!(self, Self::EvictableDerivedCache | Self::PromptResultCache)
    }

    /// Whether this class is durable and so must declare a delete/export posture.
    pub const fn is_durable(self) -> bool {
        matches!(
            self,
            Self::ReusableSemanticMemory | Self::DurableSavedMemory
        )
    }
}

/// Retention class declared for a row's artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetentionClass {
    /// Session-scoped; dropped when the session ends.
    SessionScoped,
    /// Keyed by content hash with a bounded lifetime.
    ContentKeyedBounded,
    /// Bounded by an explicit time-to-live.
    TtlBounded,
    /// Durable and retained under recorded consent.
    DurableConsented,
    /// Nothing is retained.
    NoneRetained,
}

impl RetentionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SessionScoped => "session_scoped",
            Self::ContentKeyedBounded => "content_keyed_bounded",
            Self::TtlBounded => "ttl_bounded",
            Self::DurableConsented => "durable_consented",
            Self::NoneRetained => "none_retained",
        }
    }

    /// Whether this retention class names a bounded cache lifetime.
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::ContentKeyedBounded | Self::TtlBounded)
    }
}

/// Hidden-retention spill state of a cache or memory artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SpillState {
    /// Retention stays within the declared, bounded, content-keyed envelope.
    Guarded,
    /// Would retain past its bound; blocked from becoming hidden retention.
    WouldSpillBlocked,
    /// Would export to telemetry; blocked from becoming a shadow-telemetry store.
    ShadowStoreBlocked,
}

impl SpillState {
    /// Every spill state, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::Guarded,
        Self::WouldSpillBlocked,
        Self::ShadowStoreBlocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Guarded => "guarded",
            Self::WouldSpillBlocked => "would_spill_blocked",
            Self::ShadowStoreBlocked => "shadow_store_blocked",
        }
    }

    /// Whether this state is a blocked spill that requires a precise label.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::WouldSpillBlocked | Self::ShadowStoreBlocked)
    }
}

/// Hidden-retention spill guard for a cache or memory artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetentionSpillGuard {
    /// Declared retention class.
    pub retention_class: RetentionClass,
    /// True when the artifact is keyed by a content hash.
    pub content_key_bound: bool,
    /// Bounded-lifetime label, required for a bounded cache.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_lifetime_label: Option<String>,
    /// True when export to a telemetry sink is permitted. Caches must keep this
    /// `false` so they cannot become shadow-telemetry stores.
    pub telemetry_export_allowed: bool,
    /// Spill state.
    pub spill_state: SpillState,
}

impl RetentionSpillGuard {
    /// Whether a bounded cache is content-keyed, lifetime-bounded, and never
    /// telemetry-exported — the spill guard that keeps it from becoming a hidden
    /// retention or shadow-telemetry store.
    pub fn cache_is_bounded(&self, class: MemoryArtifactClass) -> bool {
        if !class.is_bounded_cache() {
            return true;
        }
        self.retention_class.is_bounded()
            && self.content_key_bound
            && self
                .max_lifetime_label
                .as_ref()
                .is_some_and(|label| !label.trim().is_empty())
            && !self.telemetry_export_allowed
    }
}

/// Recall scope an artifact is bound to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecallScope {
    /// Single-turn scope.
    Turn,
    /// Thread scope.
    Thread,
    /// Workspace scope.
    Workspace,
    /// Tenant scope.
    Tenant,
    /// Org scope.
    Org,
}

impl RecallScope {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Turn => "turn",
            Self::Thread => "thread",
            Self::Workspace => "workspace",
            Self::Tenant => "tenant",
            Self::Org => "org",
        }
    }
}

/// State of a cross-boundary memory fence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FenceState {
    /// Fenced by default; recall does not cross the boundary.
    Fenced,
    /// An attempted crossing was detected and blocked.
    BreachBlocked,
    /// Crossing is allowed only under a recorded, explicit consent.
    ExplicitlyConsented,
}

impl FenceState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fenced => "fenced",
            Self::BreachBlocked => "breach_blocked",
            Self::ExplicitlyConsented => "explicitly_consented",
        }
    }

    /// Whether this state keeps the boundary closed by default (no silent
    /// crossing). Explicit consent is an opt-in, not a default.
    pub const fn is_closed_by_default(self) -> bool {
        matches!(self, Self::Fenced | Self::BreachBlocked)
    }
}

/// Cross-workspace-or-cross-tenant memory fence for an artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryFence {
    /// The recall scope this artifact is bound to.
    pub recall_scope: RecallScope,
    /// Cross-workspace fence state; must stay closed by default.
    pub cross_workspace_state: FenceState,
    /// Cross-tenant fence state; must stay closed by default.
    pub cross_tenant_state: FenceState,
    /// True when the fence and its state are visible in-product.
    pub fence_visible: bool,
    /// Recorded consent ref when a fence state is [`FenceState::ExplicitlyConsented`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub consent_ref: Option<String>,
}

impl MemoryFence {
    /// Whether neither boundary crosses by default and any explicit consent is
    /// backed by a recorded consent ref.
    pub fn fences_hold(&self) -> bool {
        let consent_ok = if matches!(self.cross_workspace_state, FenceState::ExplicitlyConsented)
            || matches!(self.cross_tenant_state, FenceState::ExplicitlyConsented)
        {
            self.consent_ref
                .as_ref()
                .is_some_and(|reference| !reference.trim().is_empty())
        } else {
            true
        };
        self.fence_visible && consent_ok
    }

    /// Whether either boundary is in a breach-blocked state needing a label.
    pub fn has_breach(&self) -> bool {
        matches!(self.cross_workspace_state, FenceState::BreachBlocked)
            || matches!(self.cross_tenant_state, FenceState::BreachBlocked)
    }
}

/// State of a cache-policy filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FilterState {
    /// No policy narrowing applies.
    Unfiltered,
    /// Recall is narrowed by a disclosed policy reason.
    Narrowed,
    /// Recall is fully blocked by a disclosed policy reason.
    FullyBlocked,
}

impl FilterState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unfiltered => "unfiltered",
            Self::Narrowed => "narrowed",
            Self::FullyBlocked => "fully_blocked",
        }
    }

    /// Whether this state narrows or blocks and so must disclose a reason.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::Narrowed | Self::FullyBlocked)
    }
}

/// Reason a cache-policy filter narrows or blocks recall.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowReason {
    /// A retention/policy class forbids this material in recall.
    PolicyClass,
    /// A region gate blocks recall in this region.
    RegionGate,
    /// A retention floor holds this material out of recall.
    RetentionFloor,
    /// Tenant isolation forbids crossing the tenant boundary.
    TenantIsolation,
    /// A BYOK boundary keeps this material on the user's own credential path.
    ByokBoundary,
}

impl NarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyClass => "policy_class",
            Self::RegionGate => "region_gate",
            Self::RetentionFloor => "retention_floor",
            Self::TenantIsolation => "tenant_isolation",
            Self::ByokBoundary => "byok_boundary",
        }
    }
}

/// Cache-policy filter disclosing what is narrowed and why.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CachePolicyFilter {
    /// Filter state.
    pub filter_state: FilterState,
    /// Reason the filter narrows or blocks, required when narrowed or blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_reason: Option<NarrowReason>,
    /// Human-readable disclosure of what was narrowed, required when narrowed or
    /// blocked.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowed_disclosure: Option<String>,
}

impl CachePolicyFilter {
    /// Whether a narrowing or blocking filter discloses its reason and a precise,
    /// non-generic narrowing label.
    pub fn disclosure_consistent(&self) -> bool {
        if !self.filter_state.requires_disclosure() {
            return true;
        }
        self.narrowed_reason.is_some()
            && self
                .narrowed_disclosure
                .as_ref()
                .is_some_and(|label| !label_is_generic(label))
    }
}

/// One ordered hop in a retrieval fallback chain.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackHopKind {
    /// First-party managed endpoint.
    ManagedPrimary,
    /// Vendor reached through the user's own credential.
    ByokDirect,
    /// Workspace-mirrored recall.
    WorkspaceMirror,
    /// On-device offline pack recall.
    OfflineLocalPack,
    /// A prior cached result reused under its content key.
    CachedPriorResult,
    /// A non-AI terminal that keeps the surface reachable without a model.
    NonAiTerminal,
}

impl FallbackHopKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedPrimary => "managed_primary",
            Self::ByokDirect => "byok_direct",
            Self::WorkspaceMirror => "workspace_mirror",
            Self::OfflineLocalPack => "offline_local_pack",
            Self::CachedPriorResult => "cached_prior_result",
            Self::NonAiTerminal => "non_ai_terminal",
        }
    }

    /// Whether this hop is the non-AI terminal that must end every chain.
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::NonAiTerminal)
    }

    /// Whether this hop can be served without network access.
    pub const fn is_offline_capable(self) -> bool {
        matches!(
            self,
            Self::WorkspaceMirror
                | Self::OfflineLocalPack
                | Self::CachedPriorResult
                | Self::NonAiTerminal
        )
    }
}

/// Resolved state of a retrieval fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackState {
    /// The primary route served the request.
    PrimaryAvailable,
    /// A workspace mirror served the request after the primary was unreachable.
    MirrorServed,
    /// An on-device offline pack served the request.
    OfflineLocalServed,
    /// Recall was policy-blocked and degraded to a precise label.
    PolicyBlockedDegraded,
    /// The chain reached its non-AI terminal.
    TerminalNonAi,
}

impl FallbackState {
    /// Every fallback state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PrimaryAvailable,
        Self::MirrorServed,
        Self::OfflineLocalServed,
        Self::PolicyBlockedDegraded,
        Self::TerminalNonAi,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryAvailable => "primary_available",
            Self::MirrorServed => "mirror_served",
            Self::OfflineLocalServed => "offline_local_served",
            Self::PolicyBlockedDegraded => "policy_blocked_degraded",
            Self::TerminalNonAi => "terminal_non_ai",
        }
    }

    /// Whether this state is the primary lane and so needs no fallback label.
    pub const fn is_primary(self) -> bool {
        matches!(self, Self::PrimaryAvailable)
    }

    /// Whether this state was served from an offline-capable lane.
    pub const fn is_offline_lane(self) -> bool {
        matches!(self, Self::MirrorServed | Self::OfflineLocalServed)
    }
}

/// Offline-or-mirror-safe retrieval fallback for an artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RetrievalFallback {
    /// Resolved fallback state.
    pub fallback_state: FallbackState,
    /// Ordered fallback chain; must end in a non-AI terminal.
    pub fallback_chain: Vec<FallbackHopKind>,
    /// True when the chain can serve a result with no network access.
    pub offline_safe: bool,
    /// Precise fallback label, required on any non-primary lane so a spend or
    /// route failure never collapses into a generic provider error.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub precise_label: Option<String>,
}

impl RetrievalFallback {
    /// Whether the chain is non-empty and ends in the non-AI terminal.
    pub fn chain_ends_in_terminal(&self) -> bool {
        self.fallback_chain
            .last()
            .is_some_and(|hop| hop.is_terminal())
            && self
                .fallback_chain
                .iter()
                .filter(|hop| hop.is_terminal())
                .count()
                == 1
    }

    /// Whether a non-primary lane carries a precise, non-generic label.
    pub fn label_consistent(&self) -> bool {
        if self.fallback_state.is_primary() {
            return true;
        }
        self.precise_label
            .as_ref()
            .is_some_and(|label| !label_is_generic(label))
    }

    /// Whether an offline-served lane is actually backed by an offline-capable
    /// chain marked offline-safe.
    pub fn offline_lane_is_offline_safe(&self) -> bool {
        if !self.fallback_state.is_offline_lane() {
            return true;
        }
        self.offline_safe
            && self
                .fallback_chain
                .iter()
                .any(|hop| hop.is_offline_capable() && !hop.is_terminal())
    }
}

/// Delete or export posture for a durable artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeleteExportPosture {
    /// User-initiated delete or export of their own copy.
    UserScoped,
    /// Workspace-scoped delete or export.
    WorkspaceScoped,
    /// Tenant-scoped delete or export.
    TenantScoped,
    /// Org-scoped delete or export.
    OrgScoped,
    /// Not applicable; nothing durable is retained to delete or export.
    NotApplicable,
}

impl DeleteExportPosture {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserScoped => "user_scoped",
            Self::WorkspaceScoped => "workspace_scoped",
            Self::TenantScoped => "tenant_scoped",
            Self::OrgScoped => "org_scoped",
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

/// Recall surface that reads a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FenceFallbackConsumerSurface {
    /// Composer inline assist and prompt-composer recall.
    ComposerAssist,
    /// Docs and in-app browser recall with provenance.
    DocsBrowserRecall,
    /// Codebase-understanding recall over the workspace graph.
    CodeUnderstanding,
    /// Semantic and hybrid search surface.
    SemanticSearch,
    /// Support / export packet projection.
    SupportExport,
    /// Managed or offline usage and locality reporting.
    ManagedOfflineReport,
}

impl FenceFallbackConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ComposerAssist,
        Self::DocsBrowserRecall,
        Self::CodeUnderstanding,
        Self::SemanticSearch,
        Self::SupportExport,
        Self::ManagedOfflineReport,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComposerAssist => "composer_assist",
            Self::DocsBrowserRecall => "docs_browser_recall",
            Self::CodeUnderstanding => "code_understanding",
            Self::SemanticSearch => "semantic_search",
            Self::SupportExport => "support_export",
            Self::ManagedOfflineReport => "managed_offline_report",
        }
    }
}

/// One governed memory or cache artifact on one claimed M5 profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryFenceFallbackRow {
    /// Stable row id.
    pub row_id: String,
    /// Claimed M5 profile this row is governed under.
    pub profile: M5Profile,
    /// Memory or cache artifact class.
    pub artifact_class: MemoryArtifactClass,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Hidden-retention spill guard.
    pub spill_guard: RetentionSpillGuard,
    /// Cross-workspace-or-cross-tenant memory fence.
    pub fence: MemoryFence,
    /// Cache-policy filter.
    pub policy_filter: CachePolicyFilter,
    /// Offline-or-mirror-safe retrieval fallback.
    pub fallback: RetrievalFallback,
    /// Delete posture for this artifact.
    pub delete_posture: DeleteExportPosture,
    /// Export posture for this artifact.
    pub export_posture: DeleteExportPosture,
    /// Recall surfaces that read this row.
    pub consumer_surfaces: Vec<FenceFallbackConsumerSurface>,
    /// Precise degraded label, required when this row is degraded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_label: Option<String>,
    /// Evidence packet refs backing this row.
    pub evidence_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl MemoryFenceFallbackRow {
    /// Whether the spill guard keeps a bounded cache from becoming hidden
    /// retention or a shadow-telemetry store.
    pub fn spill_guard_holds(&self) -> bool {
        self.spill_guard.cache_is_bounded(self.artifact_class)
    }

    /// Whether the cross-boundary fences hold and any consent is recorded.
    pub fn fences_hold(&self) -> bool {
        self.fence.cross_workspace_state.is_closed_by_default()
            && self.fence.cross_tenant_state.is_closed_by_default()
            || self.fence.fences_hold()
    }

    /// Whether neither boundary crosses by default (no silent cross-scope recall).
    pub fn no_default_cross_scope(&self) -> bool {
        // A default crossing is only legitimate behind a recorded consent.
        let workspace_ok = self.fence.cross_workspace_state.is_closed_by_default()
            || self.fence.consent_ref.is_some();
        let tenant_ok = self.fence.cross_tenant_state.is_closed_by_default()
            || self.fence.consent_ref.is_some();
        workspace_ok && tenant_ok && self.fence.fence_visible
    }

    /// Whether a durable artifact declares an actionable delete and export posture.
    pub fn delete_export_actionable(&self) -> bool {
        if !self.artifact_class.is_durable() {
            return true;
        }
        self.delete_posture.is_actionable() && self.export_posture.is_actionable()
    }

    /// Whether an offline-required profile keeps its fallback offline-safe.
    pub fn offline_profile_is_offline_safe(&self) -> bool {
        if !self.profile.requires_offline_safe() {
            return true;
        }
        self.fallback.offline_safe
    }

    /// Whether this row is degraded and so needs a precise label.
    ///
    /// A row is degraded when its spill state is blocked, its fence has a breach,
    /// its policy filter narrows or blocks, or its fallback is not primary.
    pub fn needs_degraded_label(&self) -> bool {
        self.spill_guard.spill_state.is_blocked()
            || self.fence.has_breach()
            || self.policy_filter.filter_state.requires_disclosure()
            || !self.fallback.fallback_state.is_primary()
    }

    /// Whether the degraded-label requirement is satisfied.
    pub fn degraded_label_consistent(&self) -> bool {
        if !self.needs_degraded_label() {
            return true;
        }
        match &self.degraded_label {
            Some(label) => !label_is_generic(label),
            None => false,
        }
    }

    /// Whether every dimension required to materialize this row is present.
    pub fn is_complete(&self) -> bool {
        !self.row_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.spill_guard_holds()
            && self.no_default_cross_scope()
            && self.fence.fences_hold()
            && self.policy_filter.disclosure_consistent()
            && self.fallback.chain_ends_in_terminal()
            && self.fallback.label_consistent()
            && self.fallback.offline_lane_is_offline_safe()
            && self.offline_profile_is_offline_safe()
            && self.delete_export_actionable()
            && self.degraded_label_consistent()
            && !self.consumer_surfaces.is_empty()
            && !self.evidence_refs.is_empty()
            && !self.source_contract_refs.is_empty()
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FenceFallbackGuardrails {
    /// No cross-workspace recall happens by default.
    pub no_cross_workspace_recall_by_default: bool,
    /// No cross-tenant recall happens by default.
    pub no_cross_tenant_recall_by_default: bool,
    /// Prompt-result caches never behave like shadow-telemetry stores.
    pub prompt_result_caches_are_not_shadow_telemetry: bool,
    /// Caches stay content-keyed and lifetime-bounded.
    pub caches_content_keyed_and_lifetime_bounded: bool,
    /// Policy-filtered paths disclose what is narrowed and why.
    pub policy_filtered_paths_disclose_narrowing: bool,
    /// Offline, mirrored, and policy-blocked fallback truth stays visible in export.
    pub fallback_truth_visible_in_export: bool,
    /// Spend or route failures keep a precise fallback rather than a generic error.
    pub spend_route_failures_keep_precise_fallback: bool,
    /// Every durable artifact declares its delete and export posture.
    pub every_durable_artifact_declares_delete_export: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FenceFallbackConsumerProjection {
    /// Composer shows the fence and fallback posture of recall it used.
    pub composer_shows_fence_and_fallback: bool,
    /// Docs/browser shows what a policy filter narrowed.
    pub docs_browser_shows_policy_narrowing: bool,
    /// Search shows the retrieval fallback state.
    pub search_shows_retrieval_fallback_state: bool,
    /// Support export shows retention and fence posture.
    pub support_export_shows_retention_and_fence: bool,
    /// Managed/offline reporting shows fallback truth.
    pub managed_offline_shows_fallback_truth: bool,
    /// Blocked or degraded lanes are visibly labeled below current.
    pub blocked_or_degraded_lanes_labeled_below_current: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FenceFallbackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the row.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`MemoryFenceFallbackPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryFenceFallbackPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub records_label: String,
    /// Materialized rows.
    pub rows: Vec<MemoryFenceFallbackRow>,
    /// Guardrail invariants block.
    pub guardrails: FenceFallbackGuardrails,
    /// Consumer projection block.
    pub consumer_projection: FenceFallbackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: FenceFallbackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe memory-fence-and-fallback packet across claimed M5 profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryFenceFallbackPacket {
    /// Record kind; must equal [`MEMORY_FENCE_FALLBACK_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MEMORY_FENCE_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub records_label: String,
    /// Materialized rows.
    pub rows: Vec<MemoryFenceFallbackRow>,
    /// Guardrail invariants block.
    pub guardrails: FenceFallbackGuardrails,
    /// Consumer projection block.
    pub consumer_projection: FenceFallbackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: FenceFallbackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl MemoryFenceFallbackPacket {
    /// Builds a memory-fence-and-fallback packet.
    pub fn new(input: MemoryFenceFallbackPacketInput) -> Self {
        Self {
            record_kind: MEMORY_FENCE_FALLBACK_RECORD_KIND.to_owned(),
            schema_version: MEMORY_FENCE_FALLBACK_SCHEMA_VERSION,
            packet_id: input.packet_id,
            records_label: input.records_label,
            rows: input.rows,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Profiles represented by some row in this packet.
    pub fn represented_profiles(&self) -> BTreeSet<M5Profile> {
        self.rows.iter().map(|row| row.profile).collect()
    }

    /// Artifact classes represented by some row in this packet.
    pub fn represented_classes(&self) -> BTreeSet<MemoryArtifactClass> {
        self.rows.iter().map(|row| row.artifact_class).collect()
    }

    /// Fallback states represented by some row in this packet.
    pub fn represented_fallback_states(&self) -> BTreeSet<FallbackState> {
        self.rows
            .iter()
            .map(|row| row.fallback.fallback_state)
            .collect()
    }

    /// Validates the memory-fence-and-fallback invariants.
    pub fn validate(&self) -> Vec<MemoryFenceFallbackViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MEMORY_FENCE_FALLBACK_RECORD_KIND {
            violations.push(MemoryFenceFallbackViolation::WrongRecordKind);
        }
        if self.schema_version != MEMORY_FENCE_FALLBACK_SCHEMA_VERSION {
            violations.push(MemoryFenceFallbackViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.records_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(MemoryFenceFallbackViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_guardrails(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("memory fence fallback packet serializes"),
        ) {
            violations.push(MemoryFenceFallbackViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("memory fence fallback packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let primary = self
            .rows
            .iter()
            .filter(|row| row.fallback.fallback_state.is_primary())
            .count();
        let mut out = String::new();
        out.push_str("# Memory Fences, Spill Guards, Cache-Policy Filters, and Fallback Truth\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.records_label));
        out.push_str(&format!(
            "- Rows: {} ({} primary fallback)\n",
            self.rows.len(),
            primary
        ));
        out.push_str(&format!(
            "- Profiles: {} / Classes: {}\n",
            self.represented_profiles().len(),
            self.represented_classes().len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({} / {}): spill `{}`, fallback `{}`\n",
                row.row_id,
                row.profile.as_str(),
                row.artifact_class.as_str(),
                row.spill_guard.spill_state.as_str(),
                row.fallback.fallback_state.as_str()
            ));
            out.push_str(&format!("  - {}\n", row.label_summary));
            out.push_str(&format!(
                "  - Fence: workspace `{}`, tenant `{}`\n",
                row.fence.cross_workspace_state.as_str(),
                row.fence.cross_tenant_state.as_str()
            ));
            out.push_str(&format!(
                "  - Filter: `{}`\n",
                row.policy_filter.filter_state.as_str()
            ));
            out.push_str(&format!(
                "  - Delete/export: `{}` / `{}`\n",
                row.delete_posture.as_str(),
                row.export_posture.as_str()
            ));
            if let Some(label) = &row.degraded_label {
                out.push_str(&format!("  - Degraded: {label}\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in memory-fence-and-fallback export.
#[derive(Debug)]
pub enum MemoryFenceFallbackArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MemoryFenceFallbackViolation>),
}

impl fmt::Display for MemoryFenceFallbackArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "memory fence fallback export parse failed: {error}"
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
                    "memory fence fallback export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MemoryFenceFallbackArtifactError {}

/// Validation failures emitted by [`MemoryFenceFallbackPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MemoryFenceFallbackViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required M5 profile is represented by no row.
    RequiredProfileCoverageMissing,
    /// A required artifact class is represented by no row.
    RequiredClassCoverageMissing,
    /// No row demonstrates a blocked hidden-retention spill.
    SpillBlockedCaseMissing,
    /// No row demonstrates a disclosed policy-narrowed filter.
    NarrowedFilterCaseMissing,
    /// No row demonstrates an offline-or-mirror-served fallback.
    OfflineFallbackCaseMissing,
    /// No row demonstrates a policy-blocked degraded fallback.
    PolicyBlockedFallbackCaseMissing,
    /// A row is incomplete.
    RowIncomplete,
    /// A bounded cache is not content-keyed, lifetime-bounded, telemetry-free.
    CacheUnboundedOrTelemetry,
    /// A cross-workspace or cross-tenant fence crosses by default.
    CrossScopeFenceOpen,
    /// A narrowing or blocking filter does not disclose its reason.
    FilterDisclosureMissing,
    /// A fallback chain does not end in a single non-AI terminal.
    FallbackChainNoTerminal,
    /// A non-primary fallback lacks a precise label.
    FallbackLabelMissing,
    /// An offline-required lane is not actually offline-safe.
    OfflineLaneNotOfflineSafe,
    /// A durable artifact does not declare an actionable delete/export posture.
    DurableArtifactMissingDeleteExport,
    /// A row needing a precise degraded label lacks one.
    DegradedLabelMissing,
    /// A row has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl MemoryFenceFallbackViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredProfileCoverageMissing => "required_profile_coverage_missing",
            Self::RequiredClassCoverageMissing => "required_class_coverage_missing",
            Self::SpillBlockedCaseMissing => "spill_blocked_case_missing",
            Self::NarrowedFilterCaseMissing => "narrowed_filter_case_missing",
            Self::OfflineFallbackCaseMissing => "offline_fallback_case_missing",
            Self::PolicyBlockedFallbackCaseMissing => "policy_blocked_fallback_case_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::CacheUnboundedOrTelemetry => "cache_unbounded_or_telemetry",
            Self::CrossScopeFenceOpen => "cross_scope_fence_open",
            Self::FilterDisclosureMissing => "filter_disclosure_missing",
            Self::FallbackChainNoTerminal => "fallback_chain_no_terminal",
            Self::FallbackLabelMissing => "fallback_label_missing",
            Self::OfflineLaneNotOfflineSafe => "offline_lane_not_offline_safe",
            Self::DurableArtifactMissingDeleteExport => "durable_artifact_missing_delete_export",
            Self::DegradedLabelMissing => "degraded_label_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable memory-fence-and-fallback export.
pub fn current_stable_memory_fence_fallback_export(
) -> Result<MemoryFenceFallbackPacket, MemoryFenceFallbackArtifactError> {
    let packet: MemoryFenceFallbackPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/ai/m5/add_hidden_retention_spill_guards_cross_workspace_or_cross_tenant_memory_fences_cache_policy_filters_and_offline_or_mirr/support_export.json"
    )))
    .map_err(MemoryFenceFallbackArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MemoryFenceFallbackArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &MemoryFenceFallbackPacket,
    violations: &mut Vec<MemoryFenceFallbackViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MEMORY_FENCE_FALLBACK_SCHEMA_REF,
        MEMORY_FENCE_FALLBACK_DOC_REF,
        MEMORY_FENCE_FALLBACK_RECALL_MATRIX_CONTRACT_REF,
        MEMORY_FENCE_FALLBACK_MEMORY_CLASS_CONTRACT_REF,
        MEMORY_FENCE_FALLBACK_DELETE_EXPORT_CONTRACT_REF,
        MEMORY_FENCE_FALLBACK_CONTEXT_ASSEMBLY_CONTRACT_REF,
        MEMORY_FENCE_FALLBACK_RETRIEVAL_CONTRACT_REF,
        MEMORY_FENCE_FALLBACK_ROUTING_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(MemoryFenceFallbackViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_coverage(
    packet: &MemoryFenceFallbackPacket,
    violations: &mut Vec<MemoryFenceFallbackViolation>,
) {
    let profiles = packet.represented_profiles();
    for required in M5Profile::ALL {
        if !profiles.contains(&required) {
            violations.push(MemoryFenceFallbackViolation::RequiredProfileCoverageMissing);
            break;
        }
    }
    let classes = packet.represented_classes();
    for required in [
        MemoryArtifactClass::PromptResultCache,
        MemoryArtifactClass::ReusableSemanticMemory,
        MemoryArtifactClass::DurableSavedMemory,
    ] {
        if !classes.contains(&required) {
            violations.push(MemoryFenceFallbackViolation::RequiredClassCoverageMissing);
            break;
        }
    }
    if !packet
        .rows
        .iter()
        .any(|row| row.spill_guard.spill_state.is_blocked() && row.degraded_label_consistent())
    {
        violations.push(MemoryFenceFallbackViolation::SpillBlockedCaseMissing);
    }
    if !packet.rows.iter().any(|row| {
        row.policy_filter.filter_state.requires_disclosure() && row.degraded_label_consistent()
    }) {
        violations.push(MemoryFenceFallbackViolation::NarrowedFilterCaseMissing);
    }
    if !packet
        .rows
        .iter()
        .any(|row| row.fallback.fallback_state.is_offline_lane() && row.fallback.offline_safe)
    {
        violations.push(MemoryFenceFallbackViolation::OfflineFallbackCaseMissing);
    }
    if !packet.rows.iter().any(|row| {
        matches!(
            row.fallback.fallback_state,
            FallbackState::PolicyBlockedDegraded
        ) && row.degraded_label_consistent()
    }) {
        violations.push(MemoryFenceFallbackViolation::PolicyBlockedFallbackCaseMissing);
    }
}

fn validate_rows(
    packet: &MemoryFenceFallbackPacket,
    violations: &mut Vec<MemoryFenceFallbackViolation>,
) {
    for row in &packet.rows {
        if !row.is_complete() {
            violations.push(MemoryFenceFallbackViolation::RowIncomplete);
        }
        if !row.spill_guard_holds() {
            violations.push(MemoryFenceFallbackViolation::CacheUnboundedOrTelemetry);
        }
        if !row.no_default_cross_scope() {
            violations.push(MemoryFenceFallbackViolation::CrossScopeFenceOpen);
        }
        if !row.policy_filter.disclosure_consistent() {
            violations.push(MemoryFenceFallbackViolation::FilterDisclosureMissing);
        }
        if !row.fallback.chain_ends_in_terminal() {
            violations.push(MemoryFenceFallbackViolation::FallbackChainNoTerminal);
        }
        if !row.fallback.label_consistent() {
            violations.push(MemoryFenceFallbackViolation::FallbackLabelMissing);
        }
        if !row.fallback.offline_lane_is_offline_safe() || !row.offline_profile_is_offline_safe() {
            violations.push(MemoryFenceFallbackViolation::OfflineLaneNotOfflineSafe);
        }
        if !row.delete_export_actionable() {
            violations.push(MemoryFenceFallbackViolation::DurableArtifactMissingDeleteExport);
        }
        if !row.degraded_label_consistent() {
            violations.push(MemoryFenceFallbackViolation::DegradedLabelMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(MemoryFenceFallbackViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_guardrails(
    packet: &MemoryFenceFallbackPacket,
    violations: &mut Vec<MemoryFenceFallbackViolation>,
) {
    let guardrails = &packet.guardrails;
    let ok = guardrails.no_cross_workspace_recall_by_default
        && guardrails.no_cross_tenant_recall_by_default
        && guardrails.prompt_result_caches_are_not_shadow_telemetry
        && guardrails.caches_content_keyed_and_lifetime_bounded
        && guardrails.policy_filtered_paths_disclose_narrowing
        && guardrails.fallback_truth_visible_in_export
        && guardrails.spend_route_failures_keep_precise_fallback
        && guardrails.every_durable_artifact_declares_delete_export;
    if !ok {
        violations.push(MemoryFenceFallbackViolation::GuardrailsIncomplete);
    }
}

fn validate_consumer_projection(
    packet: &MemoryFenceFallbackPacket,
    violations: &mut Vec<MemoryFenceFallbackViolation>,
) {
    let projection = &packet.consumer_projection;
    let ok = projection.composer_shows_fence_and_fallback
        && projection.docs_browser_shows_policy_narrowing
        && projection.search_shows_retrieval_fallback_state
        && projection.support_export_shows_retention_and_fence
        && projection.managed_offline_shows_fallback_truth
        && projection.blocked_or_degraded_lanes_labeled_below_current;
    if !ok {
        violations.push(MemoryFenceFallbackViolation::ConsumerProjectionIncomplete);
    }
}

fn validate_proof_freshness(
    packet: &MemoryFenceFallbackPacket,
    violations: &mut Vec<MemoryFenceFallbackViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(MemoryFenceFallbackViolation::ProofFreshnessIncomplete);
    }
}

/// Whether a degraded or fallback label is a generic non-answer rather than a
/// precise label. A generic provider error must never stand in for a precise
/// fallback truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "retrieval unavailable"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "retrieval error"
            | "provider error"
            | "request failed"
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
