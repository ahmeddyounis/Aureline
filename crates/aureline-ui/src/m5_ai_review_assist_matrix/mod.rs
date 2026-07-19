//! Frozen M5 AI-review-finding-row, review-scope-selector, publish-to-review-sheet, and resolution-memory-row matrix.
//!
//! This module locks Aureline's AI-review-assist model — the reusable AI review finding row, the review scope
//! selector, the publish-to-review sheet, and the resolution memory row that a review-capable consumer must
//! treat as first-class, durable, publish-safe review objects rather than ad hoc AI or review chrome — into one
//! export-safe packet. Every covered object class is named once here and constrained by the same shared
//! AI-review-assist role taxonomy (finding_classification, analyzed_scope_disclosure,
//! publish_destination_disclosure, local_versus_provider_state, lifecycle_state_tracking,
//! publish_export_fallback, resolution_memory_disclosure), the same required visible state (finding label,
//! finding class and severity, analyzed scope, publish destination, local-versus-provider state, lifecycle
//! state, and publish / export fallback), the same no-AI-review-result-publishes-or-merges-implicitly rule, the
//! same no-hiding-whether-output-stays-local-or-becomes-a-provider-comment-suggested-patch-or-check-annotation
//! rule, the same no-stale-finding-looks-current-after-diff-or-instruction-drift rule, the same
//! no-local-draft-or-evidence-lost-when-provider-write-scope-is-missing-or-publish-fails rule, and the same
//! no-finding-presented-without-its-analyzed-scope-publish-destination-or-lifecycle-state rule regardless of the
//! surface that renders it.
//!
//! The matrix makes a provider-committed publish mechanically distinct from a local draft (see
//! [`M5AiReviewAssistPublishState`]) so review detail, the AI review panel, finding rows, the review scope
//! selector, the publish-to-review sheet, pending-review trays, provider publish review, the resolution memory
//! ledger, and support / export packets can key off the publish state and lifecycle state rather than guessing
//! from stale chrome. It does not widen M5 into autonomous review approval or merge behavior — it reuses the
//! already-landed AI evidence packets, review-workspace anchors, and provider-linked draft / publish-now /
//! open-in-provider semantics — it is the shared reusable AI-review-assist contract those consumers read, and
//! it binds back to the already-landed stable-proof-index and migration-task-row packets so AI-review-assist
//! truth is not split across scattered internal notes. The controlled vocabularies are frozen in one
//! self-describing [`M5AiReviewAssistVocabularySet`] rather than minted per surface. Raw secret values and
//! private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ai_review_assist_matrix,
    seeded_m5_ai_review_assist_matrix_publish_sheet_beta_narrowed,
    seeded_m5_ai_review_assist_matrix_resolution_memory_preview_narrowed,
    M5_AI_REVIEW_ASSIST_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5AiReviewAssistMatrixPacket`].
pub const M5_AI_REVIEW_ASSIST_MATRIX_RECORD_KIND: &str =
    "freeze_m5_ai_review_finding_scope_selector_publish_sheet_and_resolution_memory_matrix";

/// Schema version for M5 AI-review-assist matrix records.
pub const M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined AI-review-assist matrix schema.
pub const M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF: &str =
    "schemas/review/m5-ai-review-assist-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF: &str = "docs/review/m5-ai-review-assist-ops.md";

/// Repo-relative path of the canonical AI-review-finding domain schema (the finding class, severity /
/// confidence, analyzed scope, and lifecycle state of one inspectable AI review finding).
pub const M5_AI_REVIEW_FINDING_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-ai-review-finding.schema.json";

/// Repo-relative path of the canonical review-scope-selector domain schema (the analyzed diff scope, the
/// repo-instruction / check source, scope drift, and the rerun recommendation that keep findings bound to the
/// diff they were produced from).
pub const M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-ai-review-scope-selector.schema.json";

/// Repo-relative path of the canonical publish-to-review-sheet domain schema (the publish mode, provider
/// destination, local-versus-provider state, and publish / export fallback so a review is never published or
/// merged implicitly).
pub const M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-ai-review-publish-sheet.schema.json";

/// Repo-relative path of the canonical resolution-memory domain schema (the resolution state, finding
/// freshness / outdated state, reopen / rerun path, and preserved-on-failure guarantee so a finding's history
/// stays provable).
pub const M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF: &str =
    "schemas/review/m5-ai-review-resolution-memory.schema.json";

/// Repo-relative path of the already-landed stable-proof-index schema the matrix binds back to.
pub const M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF: &str =
    "schemas/release/stable_proof_index.schema.json";

/// Repo-relative path of the already-landed migration-task-row schema the matrix binds back to.
pub const M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF: &str =
    "schemas/release/m5-migration-task-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_AI_REVIEW_ASSIST_FIXTURE_DIR: &str = "fixtures/review/m5-ai-review-assist";

/// Repo-relative path of the checked support-export artifact.
pub const M5_AI_REVIEW_ASSIST_ARTIFACT_REF: &str =
    "artifacts/review/m5-ai-review-publish-packets/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_AI_REVIEW_ASSIST_CSV_REF: &str =
    "artifacts/review/m5-ai-review-publish-packets/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_AI_REVIEW_ASSIST_REPORT_REF: &str =
    "artifacts/review/m5-ai-review-assist-components.md";

/// Repo-relative path of the checked AI-review-assist-health dashboard.
pub const M5_AI_REVIEW_ASSIST_DASHBOARD_REF: &str = "dashboards/m5-ai-review-assist-health.json";

/// One of the four governed AI-review-assist object classes this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistObject {
    /// A reusable AI review finding row: one inspectable AI review finding with its class, severity / confidence, analyzed scope, and lifecycle state.
    AiReviewFindingRow,
    /// A review scope selector: the analyzed diff scope and the repo-instruction / check source that bound an AI review run.
    ReviewScopeSelector,
    /// A publish-to-review sheet: the outbound publish mode and provider destination for one or more AI review findings.
    PublishToReviewSheet,
    /// A resolution memory row: the durable resolution / history record that keeps a finding's dismissed, published, outdated, or suppressed state provable over time.
    ResolutionMemoryRow,
}

impl M5AiReviewAssistObject {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AiReviewFindingRow,
        Self::ReviewScopeSelector,
        Self::PublishToReviewSheet,
        Self::ResolutionMemoryRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiReviewFindingRow => "ai_review_finding_row",
            Self::ReviewScopeSelector => "review_scope_selector",
            Self::PublishToReviewSheet => "publish_to_review_sheet",
            Self::ResolutionMemoryRow => "resolution_memory_row",
        }
    }
    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// class's AI-review finding, scope-selector, publish-sheet, or resolution-memory meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::AiReviewFindingRow => M5_AI_REVIEW_FINDING_DOMAIN_SCHEMA_REF,
            Self::ReviewScopeSelector => M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF,
            Self::PublishToReviewSheet => M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF,
            Self::ResolutionMemoryRow => M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
        }
    }

    /// `true` when this class must name a controlled finding row role.
    pub const fn declares_finding_row_roles(self) -> bool {
        matches!(self, Self::AiReviewFindingRow)
    }

    /// `true` when this class must name a controlled scope selector role.
    pub const fn declares_scope_selector_roles(self) -> bool {
        matches!(self, Self::ReviewScopeSelector)
    }

    /// `true` when this class must name a controlled publish sheet role.
    pub const fn declares_publish_sheet_roles(self) -> bool {
        matches!(self, Self::PublishToReviewSheet)
    }

    /// `true` when this class must name a controlled resolution memory role.
    pub const fn declares_resolution_memory_roles(self) -> bool {
        matches!(self, Self::ResolutionMemoryRow)
    }
}

/// The single controlled AI-review-assist role vocabulary every review, AI, provider, pending-review, help / docs, or support / export consumer binds to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistRole {
    /// The visible finding class, severity, and confidence badge for an AI review finding.
    FindingClassification,
    /// The analyzed diff scope and the repo-instruction or check source that bound the AI review run.
    AnalyzedScopeDisclosure,
    /// The publish mode and provider destination disclosed before any outbound mutation.
    PublishDestinationDisclosure,
    /// The local-draft-versus-provider-committed state of the finding or publish.
    LocalVersusProviderState,
    /// The finding lifecycle state (open, dismissed, published, outdated, suppressed, rerun recommended).
    LifecycleStateTracking,
    /// The publish or export fallback offered when provider write scope is missing or a publish fails.
    PublishExportFallback,
    /// The durable resolution / history memory a finding is joined to.
    ResolutionMemoryDisclosure,
}

impl M5AiReviewAssistRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FindingClassification,
        Self::AnalyzedScopeDisclosure,
        Self::PublishDestinationDisclosure,
        Self::LocalVersusProviderState,
        Self::LifecycleStateTracking,
        Self::PublishExportFallback,
        Self::ResolutionMemoryDisclosure,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FindingClassification => "finding_classification",
            Self::AnalyzedScopeDisclosure => "analyzed_scope_disclosure",
            Self::PublishDestinationDisclosure => "publish_destination_disclosure",
            Self::LocalVersusProviderState => "local_versus_provider_state",
            Self::LifecycleStateTracking => "lifecycle_state_tracking",
            Self::PublishExportFallback => "publish_export_fallback",
            Self::ResolutionMemoryDisclosure => "resolution_memory_disclosure",
        }
    }
    /// Whether this role is a hard posture requirement that must be present before a class may be
    /// surfaced as an AI review finding (`finding_classification`, `analyzed_scope_disclosure`,
    /// `publish_destination_disclosure`, `local_versus_provider_state`). The contextual roles
    /// (`lifecycle_state_tracking`, `publish_export_fallback`, `resolution_memory_disclosure`) apply
    /// where the object class calls for them.
    pub const fn must_be_present_before_surfacing_as_ai_review_finding(self) -> bool {
        matches!(
            self,
            Self::FindingClassification
                | Self::AnalyzedScopeDisclosure
                | Self::PublishDestinationDisclosure
                | Self::LocalVersusProviderState
        )
    }
}

/// Publish state that makes a provider-committed publish (a provider comment, suggested patch, check annotation, or open-in-provider handoff) mechanically distinct from a local draft or a local export fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistPublishState {
    /// A local draft AI review finding or publish that has not been written to any provider.
    LocalDraft,
    /// Provider-committed: published now as a provider review comment on the connected provider.
    PublishNowProviderComment,
    /// Provider-committed: published now as a suggested patch on the connected provider.
    PublishNowSuggestedPatch,
    /// Provider-committed: published now as a provider-specific check annotation.
    PublishNowCheckAnnotation,
    /// Provider-committed: handed off to be opened in the provider's own review surface.
    OpenInProvider,
    /// A local export / copy fallback used when provider write scope is missing or a publish fails; nothing is written to the provider.
    ExportFallbackOffline,
}

impl M5AiReviewAssistPublishState {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalDraft,
        Self::PublishNowProviderComment,
        Self::PublishNowSuggestedPatch,
        Self::PublishNowCheckAnnotation,
        Self::OpenInProvider,
        Self::ExportFallbackOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraft => "local_draft",
            Self::PublishNowProviderComment => "publish_now_provider_comment",
            Self::PublishNowSuggestedPatch => "publish_now_suggested_patch",
            Self::PublishNowCheckAnnotation => "publish_now_check_annotation",
            Self::OpenInProvider => "open_in_provider",
            Self::ExportFallbackOffline => "export_fallback_offline",
        }
    }
    /// `true` for every provider-committed publish state, so downstream review detail, the AI panel,
    /// pending-review trays, provider publish review, and support / export packets can key off a
    /// provider-committed publish rather than confusing it with a local draft or a local export fallback.
    pub const fn is_provider_committed(self) -> bool {
        matches!(
            self,
            Self::PublishNowProviderComment
                | Self::PublishNowSuggestedPatch
                | Self::PublishNowCheckAnnotation
                | Self::OpenInProvider
        )
    }
}

/// Named finding lifecycle state (open, dismissed, published, outdated, suppressed, rerun recommended) so no claimed surface lacks a named state for stale / outdated / suppressed findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistFindingLifecycle {
    /// The finding is open and awaiting a resolution.
    Open,
    /// The finding was reviewed and dismissed.
    Dismissed,
    /// The finding was published to a provider review destination.
    Published,
    /// The finding is outdated because the diff or instruction source drifted from the analyzed scope.
    Outdated,
    /// The finding is suppressed and intentionally kept out of the active review.
    Suppressed,
    /// A rerun is recommended because the analyzed scope no longer matches the current diff.
    RerunRecommended,
}

impl M5AiReviewAssistFindingLifecycle {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Open,
        Self::Dismissed,
        Self::Published,
        Self::Outdated,
        Self::Suppressed,
        Self::RerunRecommended,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Dismissed => "dismissed",
            Self::Published => "published",
            Self::Outdated => "outdated",
            Self::Suppressed => "suppressed",
            Self::RerunRecommended => "rerun_recommended",
        }
    }
    /// `true` for the stale / suppressed lifecycle states (`outdated`, `suppressed`) so a consumer can
    /// mechanically refuse to show a stale finding as current.
    pub const fn is_stale_or_suppressed(self) -> bool {
        matches!(self, Self::Outdated | Self::Suppressed)
    }
}

/// Controlled AI-review-finding-row role for one inspectable AI review finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistFindingRowRole {
    /// Finding class, severity, and confidence shown so a finding is never an unlabelled note.
    FindingClassAndSeverityShown,
    /// Analyzed diff scope shown so a finding names the diff it was produced from.
    AnalyzedScopeShown,
    /// Finding lifecycle state (open, outdated, suppressed) shown so a stale finding never looks current.
    FindingLifecycleStateShown,
    /// Finding linked to its durable resolution memory row.
    ResolutionMemoryLinked,
    /// A role bound to the single AI-review-assist registry.
    BoundToAiReviewAssistRegistry,
    /// Auto-approving, auto-requesting changes, or auto-merging from a finding, which is disallowed.
    AutoApproveRequestChangesOrMergeFromFindingDisallowed,
}

impl M5AiReviewAssistFindingRowRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FindingClassAndSeverityShown,
        Self::AnalyzedScopeShown,
        Self::FindingLifecycleStateShown,
        Self::ResolutionMemoryLinked,
        Self::BoundToAiReviewAssistRegistry,
        Self::AutoApproveRequestChangesOrMergeFromFindingDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FindingClassAndSeverityShown => "finding_class_and_severity_shown",
            Self::AnalyzedScopeShown => "analyzed_scope_shown",
            Self::FindingLifecycleStateShown => "finding_lifecycle_state_shown",
            Self::ResolutionMemoryLinked => "resolution_memory_linked",
            Self::BoundToAiReviewAssistRegistry => "bound_to_ai_review_assist_registry",
            Self::AutoApproveRequestChangesOrMergeFromFindingDisallowed => {
                "auto_approve_request_changes_or_merge_from_finding_disallowed"
            }
        }
    }
}

/// Controlled review-scope-selector role for the analyzed diff scope of an AI review run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistScopeSelectorRole {
    /// Analyzed diff scope shown so the review names exactly which diff it covers.
    AnalyzedDiffScopeShown,
    /// Repo instruction and enabled check source named as the scope's authority.
    RepoInstructionAndCheckSourceNamed,
    /// Scope drift flagged so findings do not silently outlive the diff they were bound to.
    ScopeDriftFlagged,
    /// Rerun-within-scope safe next step offered instead of a silent scope widening.
    RerunWithinScopeOffered,
    /// A role bound to the single AI-review-assist registry.
    BoundToAiReviewAssistRegistry,
    /// Silently widening the analyzed scope beyond the selected diff, which is disallowed.
    SilentScopeWideningDisallowed,
}

impl M5AiReviewAssistScopeSelectorRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AnalyzedDiffScopeShown,
        Self::RepoInstructionAndCheckSourceNamed,
        Self::ScopeDriftFlagged,
        Self::RerunWithinScopeOffered,
        Self::BoundToAiReviewAssistRegistry,
        Self::SilentScopeWideningDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnalyzedDiffScopeShown => "analyzed_diff_scope_shown",
            Self::RepoInstructionAndCheckSourceNamed => "repo_instruction_and_check_source_named",
            Self::ScopeDriftFlagged => "scope_drift_flagged",
            Self::RerunWithinScopeOffered => "rerun_within_scope_offered",
            Self::BoundToAiReviewAssistRegistry => "bound_to_ai_review_assist_registry",
            Self::SilentScopeWideningDisallowed => "silent_scope_widening_disallowed",
        }
    }
}

/// Controlled publish-to-review-sheet role for the outbound publish of AI review findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistPublishSheetRole {
    /// Publish mode (local draft, publish now, open in provider) shown before any outbound mutation.
    PublishModeShown,
    /// Provider destination (comment, suggested patch, check annotation) named for a publish.
    ProviderDestinationNamed,
    /// Local-draft-versus-provider-committed state shown so publishing is never implicit.
    LocalDraftVersusProviderCommittedShown,
    /// Publish-or-export fallback offered when provider write scope is missing or a publish fails.
    PublishOrExportFallbackOffered,
    /// A role bound to the single AI-review-assist registry.
    BoundToAiReviewAssistRegistry,
    /// Implicitly publishing or merging AI review output without an explicit action, which is disallowed.
    ImplicitPublishOrMergeDisallowed,
}

impl M5AiReviewAssistPublishSheetRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PublishModeShown,
        Self::ProviderDestinationNamed,
        Self::LocalDraftVersusProviderCommittedShown,
        Self::PublishOrExportFallbackOffered,
        Self::BoundToAiReviewAssistRegistry,
        Self::ImplicitPublishOrMergeDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublishModeShown => "publish_mode_shown",
            Self::ProviderDestinationNamed => "provider_destination_named",
            Self::LocalDraftVersusProviderCommittedShown => {
                "local_draft_versus_provider_committed_shown"
            }
            Self::PublishOrExportFallbackOffered => "publish_or_export_fallback_offered",
            Self::BoundToAiReviewAssistRegistry => "bound_to_ai_review_assist_registry",
            Self::ImplicitPublishOrMergeDisallowed => "implicit_publish_or_merge_disallowed",
        }
    }
}

/// Controlled resolution-memory-row role for the durable resolution / history of a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistResolutionMemoryRole {
    /// Resolution state (dismissed, published, suppressed) shown so a resolved finding stays attributable.
    ResolutionStateShown,
    /// Finding freshness and outdated state shown so a stale resolution never masquerades as current.
    FindingFreshnessAndOutdatedShown,
    /// Reopen-or-rerun path named so a closed finding can be reopened without inventing new chrome.
    ReopenOrRerunPathNamed,
    /// Local draft and evidence preserved when a publish fails, never dropped.
    LocalDraftPreservedOnPublishFailure,
    /// A role bound to the single AI-review-assist registry.
    BoundToAiReviewAssistRegistry,
    /// Silently resurfacing a suppressed or outdated finding as current, which is disallowed.
    SilentStaleResurfaceDisallowed,
}

impl M5AiReviewAssistResolutionMemoryRole {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ResolutionStateShown,
        Self::FindingFreshnessAndOutdatedShown,
        Self::ReopenOrRerunPathNamed,
        Self::LocalDraftPreservedOnPublishFailure,
        Self::BoundToAiReviewAssistRegistry,
        Self::SilentStaleResurfaceDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolutionStateShown => "resolution_state_shown",
            Self::FindingFreshnessAndOutdatedShown => "finding_freshness_and_outdated_shown",
            Self::ReopenOrRerunPathNamed => "reopen_or_rerun_path_named",
            Self::LocalDraftPreservedOnPublishFailure => "local_draft_preserved_on_publish_failure",
            Self::BoundToAiReviewAssistRegistry => "bound_to_ai_review_assist_registry",
            Self::SilentStaleResurfaceDisallowed => "silent_stale_resurface_disallowed",
        }
    }
}

/// Claimed M5 surface family that renders / consumes an AI-review-assist object class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistSurfaceFamily {
    /// The review surface (review detail, diff review headers, pending-review trays).
    Review,
    /// The AI surface (AI review panel and finding rows).
    Ai,
    /// The provider-backed publish / open-in-provider surface.
    Provider,
    /// The pending-review tray surface.
    PendingReview,
    /// The support / export surface.
    SupportExport,
    /// The help / docs surface.
    HelpDocs,
}

impl M5AiReviewAssistSurfaceFamily {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Review,
        Self::Ai,
        Self::Provider,
        Self::PendingReview,
        Self::SupportExport,
        Self::HelpDocs,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Review => "review",
            Self::Ai => "ai",
            Self::Provider => "provider",
            Self::PendingReview => "pending_review",
            Self::SupportExport => "support_export",
            Self::HelpDocs => "help_docs",
        }
    }
}

/// Classification stage a class passes through from finding production to a scope-resolved, publish-destination-selected, published-or-exported, and resolution-recorded AI review object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistClassificationStage {
    /// The finding-produced stage: an AI review finding is produced.
    FindingProduced,
    /// The scope-resolved stage: the analyzed diff scope and instruction source are resolved.
    ScopeResolved,
    /// The publish-destination-selected stage: a publish mode and provider destination are chosen.
    PublishDestinationSelected,
    /// The publish-or-export-resolved stage: the finding is published or falls back to a local export.
    PublishOrExportResolved,
    /// The resolution-recorded stage: the durable resolution memory is recorded.
    ResolutionRecorded,
}

impl M5AiReviewAssistClassificationStage {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FindingProduced,
        Self::ScopeResolved,
        Self::PublishDestinationSelected,
        Self::PublishOrExportResolved,
        Self::ResolutionRecorded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FindingProduced => "finding_produced",
            Self::ScopeResolved => "scope_resolved",
            Self::PublishDestinationSelected => "publish_destination_selected",
            Self::PublishOrExportResolved => "publish_or_export_resolved",
            Self::ResolutionRecorded => "resolution_recorded",
        }
    }
}

/// Shared consumer surface that must agree on a class's AI-review-assist truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistConsumerSurface {
    /// The review detail surface.
    ReviewDetail,
    /// The AI review panel.
    AiReviewPanel,
    /// The AI review finding row.
    FindingRow,
    /// The review scope selector.
    ReviewScopeSelector,
    /// The publish-to-review sheet.
    PublishToReviewSheet,
    /// The pending-review tray.
    PendingReviewTray,
    /// The provider publish-review surface.
    ProviderPublishReview,
    /// The resolution memory ledger.
    ResolutionMemoryLedger,
    /// The support / export packet.
    SupportExportPacket,
}

impl M5AiReviewAssistConsumerSurface {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ReviewDetail,
        Self::AiReviewPanel,
        Self::FindingRow,
        Self::ReviewScopeSelector,
        Self::PublishToReviewSheet,
        Self::PendingReviewTray,
        Self::ProviderPublishReview,
        Self::ResolutionMemoryLedger,
        Self::SupportExportPacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewDetail => "review_detail",
            Self::AiReviewPanel => "ai_review_panel",
            Self::FindingRow => "finding_row",
            Self::ReviewScopeSelector => "review_scope_selector",
            Self::PublishToReviewSheet => "publish_to_review_sheet",
            Self::PendingReviewTray => "pending_review_tray",
            Self::ProviderPublishReview => "provider_publish_review",
            Self::ResolutionMemoryLedger => "resolution_memory_ledger",
            Self::SupportExportPacket => "support_export_packet",
        }
    }
}

/// Non-visual / accessibility route every class must offer so no AI-review-assist meaning disappears under zoom, high contrast, keyboard-only use, or export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Preserves truth under high-contrast and forced-colors modes.
    HighContrastSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5AiReviewAssistAccessibilityRoute {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::HighContrastSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a class has degraded below its qualified AI-review-assist-handling state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistDegradedReason {
    /// The finding class / severity badge has gone stale.
    FindingClassStale,
    /// The analyzed diff scope or instruction source is unresolved.
    AnalyzedScopeUnresolved,
    /// The publish mode or provider destination is unresolved.
    PublishDestinationUnresolved,
    /// The local-draft-versus-provider-committed state is unknown.
    LocalVersusProviderStateUnknown,
    /// The finding lifecycle state is unknown.
    LifecycleStateUnknown,
    /// The publish / export fallback is unknown.
    PublishExportFallbackUnknown,
}

impl M5AiReviewAssistDegradedReason {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FindingClassStale,
        Self::AnalyzedScopeUnresolved,
        Self::PublishDestinationUnresolved,
        Self::LocalVersusProviderStateUnknown,
        Self::LifecycleStateUnknown,
        Self::PublishExportFallbackUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FindingClassStale => "finding_class_stale",
            Self::AnalyzedScopeUnresolved => "analyzed_scope_unresolved",
            Self::PublishDestinationUnresolved => "publish_destination_unresolved",
            Self::LocalVersusProviderStateUnknown => "local_versus_provider_state_unknown",
            Self::LifecycleStateUnknown => "lifecycle_state_unknown",
            Self::PublishExportFallbackUnknown => "publish_export_fallback_unknown",
        }
    }
}

/// Mandatory label a claimed AI-review-assist class must be able to show. The first three are hard requirements; the remaining three make the finding class badge, the publish destination, and the lifecycle state mechanically distinct for every covered class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistRequiredLabel {
    /// The class's stable identity.
    Identity,
    /// The class's AI-review-assist role.
    FindingRole,
    /// The canonical per-domain descriptor the class points at.
    CanonicalReference,
    /// The finding class / severity badge the class must show.
    FindingClassBadge,
    /// The publish mode / provider destination the class must state.
    PublishDestination,
    /// The lifecycle state the class must state.
    LifecycleState,
}

impl M5AiReviewAssistRequiredLabel {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::FindingRole,
        Self::CanonicalReference,
        Self::FindingClassBadge,
        Self::PublishDestination,
        Self::LifecycleState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::FindingRole => "finding_role",
            Self::CanonicalReference => "canonical_reference",
            Self::FindingClassBadge => "finding_class_badge",
            Self::PublishDestination => "publish_destination",
            Self::LifecycleState => "lifecycle_state",
        }
    }
    /// The three labels every claimed class must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::FindingRole, Self::CanonicalReference];
}

/// Qualification class for an M5 AI-review-assist row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistQualificationClass {
    /// Class AI-review-assist handling qualifies for the Stable claim.
    Stable,
    /// Class AI-review-assist handling is narrowed to Beta.
    Beta,
    /// Class AI-review-assist handling is narrowed to Preview.
    Preview,
    /// Class AI-review-assist handling is experimental and not claimed.
    Experimental,
    /// Class AI-review-assist handling is unavailable on this build.
    Unavailable,
    /// Class AI-review-assist handling is held pending review.
    Held,
}

impl M5AiReviewAssistQualificationClass {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Unavailable,
        Self::Held,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }
    /// Whether the class may carry a public Stable AI-review-assist-handling claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows an AI-review-assist class below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewAssistDowngradeTrigger {
    /// AI review auto-approved, auto-requested changes, or auto-merged.
    AiReviewAutoActioned,
    /// A finding was shown without its analyzed diff scope.
    FindingShownWithoutScope,
    /// A publish hid whether output stays local or becomes a provider comment, suggested patch, or check annotation.
    PublishDestinationHidden,
    /// A local draft or evidence was lost when provider write scope was missing or a publish failed.
    LocalDraftLostOnPublishFailure,
    /// A stale / outdated finding was shown as current after diff or instruction drift.
    StaleFindingShownAsCurrent,
    /// A class left its finding class / severity badge missing.
    FindingClassBadgeMissing,
    /// A class left its analyzed diff scope unstated.
    AnalyzedScopeUnstated,
    /// A class left its publish mode / provider destination unstated.
    PublishModeUnstated,
    /// A class left its finding lifecycle state missing.
    LifecycleStateMissing,
    /// A class left its publish / export fallback missing.
    PublishExportFallbackMissing,
    /// A class left its durable resolution memory unstated.
    ResolutionMemoryUnstated,
    /// The AI-review-assist matrix packet has gone stale.
    AiReviewAssistMatrixStale,
}

impl M5AiReviewAssistDowngradeTrigger {
    /// Every variant, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::AiReviewAutoActioned,
        Self::FindingShownWithoutScope,
        Self::PublishDestinationHidden,
        Self::LocalDraftLostOnPublishFailure,
        Self::StaleFindingShownAsCurrent,
        Self::FindingClassBadgeMissing,
        Self::AnalyzedScopeUnstated,
        Self::PublishModeUnstated,
        Self::LifecycleStateMissing,
        Self::PublishExportFallbackMissing,
        Self::ResolutionMemoryUnstated,
        Self::AiReviewAssistMatrixStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiReviewAutoActioned => "ai_review_auto_actioned",
            Self::FindingShownWithoutScope => "finding_shown_without_scope",
            Self::PublishDestinationHidden => "publish_destination_hidden",
            Self::LocalDraftLostOnPublishFailure => "local_draft_lost_on_publish_failure",
            Self::StaleFindingShownAsCurrent => "stale_finding_shown_as_current",
            Self::FindingClassBadgeMissing => "finding_class_badge_missing",
            Self::AnalyzedScopeUnstated => "analyzed_scope_unstated",
            Self::PublishModeUnstated => "publish_mode_unstated",
            Self::LifecycleStateMissing => "lifecycle_state_missing",
            Self::PublishExportFallbackMissing => "publish_export_fallback_missing",
            Self::ResolutionMemoryUnstated => "resolution_memory_unstated",
            Self::AiReviewAssistMatrixStale => "ai_review_assist_matrix_stale",
        }
    }
}

/// Required visible state a class must carry so an AI review finding never reads without its scope, publish
/// destination, or lifecycle state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewAssistVisibleState {
    /// Finding / object label shown on the surface (finding row, scope selector, publish sheet, resolution memory row).
    pub finding_label: String,
    /// Finding class, severity, and confidence.
    pub finding_class_and_severity: String,
    /// Analyzed diff scope and the repo-instruction or check source that bound it.
    pub analyzed_scope: String,
    /// Publish mode and provider destination (local draft, provider comment, suggested patch, check annotation, open in provider).
    pub publish_destination: String,
    /// Local-draft-versus-provider-committed state disclosed before any outbound mutation.
    pub local_versus_provider_state: String,
    /// Finding lifecycle state (open, dismissed, published, outdated, suppressed, rerun recommended).
    pub lifecycle_state: String,
    /// Publish / export fallback used when provider write scope is missing or a publish fails.
    pub publish_export_fallback: String,
}

impl M5AiReviewAssistVisibleState {
    /// `true` when every required visible-state field is present.
    fn is_complete(&self) -> bool {
        !self.finding_label.trim().is_empty()
            && !self.finding_class_and_severity.trim().is_empty()
            && !self.analyzed_scope.trim().is_empty()
            && !self.publish_destination.trim().is_empty()
            && !self.local_versus_provider_state.trim().is_empty()
            && !self.lifecycle_state.trim().is_empty()
            && !self.publish_export_fallback.trim().is_empty()
    }
}

/// One row in the matrix: one governed AI-review-assist object class bound to the surface-specific
/// AI-review truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewAssistRow {
    /// Governed AI-review-assist object class.
    pub object_class: M5AiReviewAssistObject,
    /// Qualification class earned by this class's AI-review-assist handling.
    pub qualification: M5AiReviewAssistQualificationClass,
    /// Publish state this row governs (distinguishes a provider-committed publish from a local draft or export fallback).
    pub publish_state: M5AiReviewAssistPublishState,
    /// Owner role accountable for keeping this class's AI-review-assist state governed.
    pub owner_role: String,
    /// Backup owner role accountable when the primary owner is unavailable.
    pub backup_owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Required visible state that keeps this class's finding visibly scoped, publish-safe, and attributable.
    pub required_visible_state: M5AiReviewAssistVisibleState,
    /// Claimed M5 surface families that render / consume this class.
    pub surface_families: Vec<M5AiReviewAssistSurfaceFamily>,
    /// Classification stages this class passes through from finding production to a recorded resolution.
    pub classification_stages: Vec<M5AiReviewAssistClassificationStage>,
    /// Mandatory labels this class must be able to show (must include the three
    /// [`M5AiReviewAssistRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5AiReviewAssistRequiredLabel>,
    /// AI-review-assist roles this class can carry (the frozen AC vocabulary; required on every class).
    pub semantic_roles: Vec<M5AiReviewAssistRole>,
    /// AiReviewFindingRow roles this class names (AiReviewFindingRow only).
    pub finding_row_roles: Vec<M5AiReviewAssistFindingRowRole>,
    /// ReviewScopeSelector roles this class names (ReviewScopeSelector only).
    pub scope_selector_roles: Vec<M5AiReviewAssistScopeSelectorRole>,
    /// PublishToReviewSheet roles this class names (PublishToReviewSheet only).
    pub publish_sheet_roles: Vec<M5AiReviewAssistPublishSheetRole>,
    /// ResolutionMemoryRow roles this class names (ResolutionMemoryRow only).
    pub resolution_memory_roles: Vec<M5AiReviewAssistResolutionMemoryRole>,
    /// Degraded reasons this class can name (required on every class).
    pub degraded_reasons: Vec<M5AiReviewAssistDegradedReason>,
    /// Non-visual accessibility routes this class offers.
    pub accessibility_routes: Vec<M5AiReviewAssistAccessibilityRoute>,
    /// First consumer surfaces that consume this class's AI-review-assist projection.
    pub consumer_surfaces: Vec<M5AiReviewAssistConsumerSurface>,
    /// Downgrade triggers that apply to this class.
    pub downgrade_triggers: Vec<M5AiReviewAssistDowngradeTrigger>,
    /// Required closure-artifact refs that keep this class's AI-review-assist state provable.
    pub required_closure_artifact_refs: Vec<String>,
    /// Source contract refs consumed by this class (must include its own canonical domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this class never lets AI review results publish or merge implicitly. MUST be `false`.
    pub lets_ai_review_results_publish_or_merge_implicitly: bool,
    /// Hard invariant: this class never hides whether output stays local or becomes a provider comment, a suggested patch, or a provider check annotation. MUST be `false`.
    pub hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation:
        bool,
    /// Hard invariant: this class never keeps stale findings looking current after diff or instruction drift. MUST be `false`.
    pub keeps_stale_findings_looking_current_after_diff_or_instruction_drift: bool,
    /// Hard invariant: this class never loses local drafts or evidence when provider write scope is missing or a publish fails. MUST be `false`.
    pub loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails: bool,
    /// Hard invariant: this class never presents an AI review finding without its analyzed scope, publish destination, or lifecycle state. MUST be `false`.
    pub presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state:
        bool,
}

impl M5AiReviewAssistRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5AiReviewAssistRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5AiReviewAssistRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.lets_ai_review_results_publish_or_merge_implicitly
            && !self.hides_whether_output_stays_local_or_becomes_a_provider_comment_suggested_patch_or_check_annotation
            && !self.keeps_stale_findings_looking_current_after_diff_or_instruction_drift
            && !self.loses_local_drafts_or_evidence_when_provider_write_scope_is_missing_or_publish_fails
            && !self.presents_an_ai_review_finding_without_its_analyzed_scope_publish_destination_or_lifecycle_state
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewAssistVocabularySet {
    /// Object classes tokens.
    pub object_classes: Vec<String>,
    /// Publish states tokens.
    pub publish_states: Vec<String>,
    /// Finding lifecycles tokens.
    pub finding_lifecycles: Vec<String>,
    /// Semantic roles tokens.
    pub semantic_roles: Vec<String>,
    /// Finding row roles tokens.
    pub finding_row_roles: Vec<String>,
    /// Scope selector roles tokens.
    pub scope_selector_roles: Vec<String>,
    /// Publish sheet roles tokens.
    pub publish_sheet_roles: Vec<String>,
    /// Resolution memory roles tokens.
    pub resolution_memory_roles: Vec<String>,
    /// Surface families tokens.
    pub surface_families: Vec<String>,
    /// Classification stages tokens.
    pub classification_stages: Vec<String>,
    /// Consumer surfaces tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility routes tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded reasons tokens.
    pub degraded_reasons: Vec<String>,
    /// Required labels tokens.
    pub required_labels: Vec<String>,
    /// Downgrade triggers tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5AiReviewAssistVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            object_classes: tokens(&M5AiReviewAssistObject::ALL, |v| v.as_str()),
            publish_states: tokens(&M5AiReviewAssistPublishState::ALL, |v| v.as_str()),
            finding_lifecycles: tokens(&M5AiReviewAssistFindingLifecycle::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5AiReviewAssistRole::ALL, |v| v.as_str()),
            finding_row_roles: tokens(&M5AiReviewAssistFindingRowRole::ALL, |v| v.as_str()),
            scope_selector_roles: tokens(&M5AiReviewAssistScopeSelectorRole::ALL, |v| v.as_str()),
            publish_sheet_roles: tokens(&M5AiReviewAssistPublishSheetRole::ALL, |v| v.as_str()),
            resolution_memory_roles: tokens(&M5AiReviewAssistResolutionMemoryRole::ALL, |v| {
                v.as_str()
            }),
            surface_families: tokens(&M5AiReviewAssistSurfaceFamily::ALL, |v| v.as_str()),
            classification_stages: tokens(&M5AiReviewAssistClassificationStage::ALL, |v| {
                v.as_str()
            }),
            consumer_surfaces: tokens(&M5AiReviewAssistConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5AiReviewAssistAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5AiReviewAssistDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5AiReviewAssistRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5AiReviewAssistDowngradeTrigger::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewAssistGovernanceReview {
    /// No ai review finding publishes or merges implicitly.
    pub no_ai_review_finding_publishes_or_merges_implicitly: bool,
    /// Every covered object class names owner backup owner and first consumer.
    pub every_covered_object_class_names_owner_backup_owner_and_first_consumer: bool,
    /// Local draft state is mechanically distinct from provider committed state.
    pub local_draft_state_is_mechanically_distinct_from_provider_committed_state: bool,
    /// Every finding names its class severity and confidence.
    pub every_finding_names_its_class_severity_and_confidence: bool,
    /// Every finding names its analyzed diff scope and instruction source.
    pub every_finding_names_its_analyzed_diff_scope_and_instruction_source: bool,
    /// Every publish names its mode and provider destination before mutation.
    pub every_publish_names_its_mode_and_provider_destination_before_mutation: bool,
    /// Publish or export fallback is named for every finding.
    pub publish_or_export_fallback_is_named_for_every_finding: bool,
    /// No stale or outdated finding is shown as current.
    pub no_stale_or_outdated_finding_is_shown_as_current: bool,
    /// No local draft or evidence is lost when publish fails.
    pub no_local_draft_or_evidence_is_lost_when_publish_fails: bool,
    /// Every object declares classification stages.
    pub every_object_declares_classification_stages: bool,
    /// Every object declares accessibility route.
    pub every_object_declares_accessibility_route: bool,
    /// Support export reads single ai review assist source.
    pub support_export_reads_single_ai_review_assist_source: bool,
    /// Review ai provider pending and support bind to single source.
    pub review_ai_provider_pending_and_support_bind_to_single_source: bool,
    /// Later rows cannot invent parallel ai review assist vocabulary.
    pub later_rows_cannot_invent_parallel_ai_review_assist_vocabulary: bool,
    /// Ai review assist truth survives zoom and high contrast.
    pub ai_review_assist_truth_survives_zoom_and_high_contrast: bool,
    /// Claims narrow automatically when matrix row missing or stale.
    pub claims_narrow_automatically_when_matrix_row_missing_or_stale: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewAssistConsumerProjection {
    /// Review detail and ai panel consume shared ai review finding truth.
    pub review_detail_and_ai_panel_consume_shared_ai_review_finding_truth: bool,
    /// Pending review and provider publish consume shared publish destination truth.
    pub pending_review_and_provider_publish_consume_shared_publish_destination_truth: bool,
    /// Help and support export consume shared finding lifecycle truth.
    pub help_and_support_export_consume_shared_finding_lifecycle_truth: bool,
    /// Docs help and screenshots read single ai review assist source.
    pub docs_help_and_screenshots_read_single_ai_review_assist_source: bool,
    /// Findings bind to shared resolution memory relation.
    pub findings_bind_to_shared_resolution_memory_relation: bool,
    /// Support export reads single ai review assist source.
    pub support_export_reads_single_ai_review_assist_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewAssistProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof / audit refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the class.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the AI-review-assist lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewAssistReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting AI-review-assist audit for the lane.
    pub ai_review_assist_audit_ref: String,
    /// True when support/export parity is required for every class.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every class.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5AiReviewAssistMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5AiReviewAssistMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// AI-review-assist rows.
    pub ai_review_assist_rows: Vec<M5AiReviewAssistRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiReviewAssistVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiReviewAssistGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiReviewAssistConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiReviewAssistProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiReviewAssistReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 AI-review-assist matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AiReviewAssistMatrixPacket {
    /// Record kind; must equal [`M5_AI_REVIEW_ASSIST_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// AI-review-assist rows.
    pub ai_review_assist_rows: Vec<M5AiReviewAssistRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5AiReviewAssistVocabularySet,
    /// Governance-review block.
    pub governance_review: M5AiReviewAssistGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5AiReviewAssistConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5AiReviewAssistProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5AiReviewAssistReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5AiReviewAssistMatrixPacket {
    /// Builds an M5 AI-review-assist matrix packet from input.
    pub fn new(input: M5AiReviewAssistMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_AI_REVIEW_ASSIST_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            ai_review_assist_rows: input.ai_review_assist_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 AI-review-assist matrix invariants.
    pub fn validate(&self) -> Vec<M5AiReviewAssistMatrixViolation> {
        let mut violations = Vec::new();
        if self.record_kind != M5_AI_REVIEW_ASSIST_MATRIX_RECORD_KIND {
            violations.push(M5AiReviewAssistMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_VERSION {
            violations.push(M5AiReviewAssistMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5AiReviewAssistMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_ai_review_assist_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 ai-review-assist matrix serializes"),
        ) {
            violations.push(M5AiReviewAssistMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 ai-review-assist matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed AI-review-assist class.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "object_class,qualification,publish_state,owner,backup_owner,canonical_schema,surface_families,classification_stages,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.ai_review_assist_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.publish_state.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.backup_owner_role),
                row.object_class.canonical_domain_schema_ref(),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.classification_stages, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic AI-review-assist-health dashboard JSON that review and support surfaces render from one
    /// canonical matrix instead of hand-authoring readiness chrome.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn render_dashboard_json(&self) -> String {
        let objects: Vec<serde_json::Value> = self
            .ai_review_assist_rows
            .iter()
            .map(|row| {
                serde_json::json!({
                    "object_class": row.object_class.as_str(),
                    "qualification": row.qualification.as_str(),
                    "publish_state": row.publish_state.as_str(),
                    "canonical_schema": row.object_class.canonical_domain_schema_ref(),
                    "classification_stages": row
                        .classification_stages
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                    "consumer_surfaces": row
                        .consumer_surfaces
                        .iter()
                        .map(|v| v.as_str())
                        .collect::<Vec<_>>(),
                })
            })
            .collect();
        let dashboard = serde_json::json!({
            "record_kind": "m5_ai_review_assist_health",
            "packet_id": self.packet_id,
            "matrix_label": self.matrix_label,
            "matrix_schema_ref": M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
            "support_export_ref": M5_AI_REVIEW_ASSIST_ARTIFACT_REF,
            "classification_stages": self.vocabulary_set.classification_stages,
            "downgrade_triggers": self.vocabulary_set.downgrade_triggers,
            "objects": objects,
        });
        serde_json::to_string_pretty(&dashboard)
            .expect("m5 ai-review-assist-health dashboard serializes")
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_objects = self
            .ai_review_assist_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 AI Review Assist: Finding Row, Review Scope Selector, Publish-to-Review Sheet, and Resolution Memory Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} ({} stable)\n",
            self.ai_review_assist_rows.len(),
            stable_objects
        ));
        out.push_str(&format!(
            "- AI-review-assist roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Classification stages: {}\n",
            self.vocabulary_set.classification_stages.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last audit: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Object classes\n\n");
        for row in &self.ai_review_assist_rows {
            out.push_str(&format!(
                "- **{}**: `{}` (publish_state: `{}`)\n",
                row.object_class.as_str(),
                row.qualification.as_str(),
                row.publish_state.as_str()
            ));
            out.push_str(&format!(
                "  - Owner: {} (backup: {})\n",
                row.owner_role, row.backup_owner_role
            ));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.object_class.canonical_domain_schema_ref()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Publish destination: {}\n",
                row.required_visible_state.publish_destination
            ));
            out.push_str(&format!(
                "  - Lifecycle state: {}\n",
                row.required_visible_state.lifecycle_state
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 AI-review-assist matrix export.
#[derive(Debug)]
pub enum M5AiReviewAssistMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5AiReviewAssistMatrixViolation>),
}

impl fmt::Display for M5AiReviewAssistMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 ai-review-assist matrix export parse failed: {error}"
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
                    "m5 ai-review-assist matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5AiReviewAssistMatrixArtifactError {}

/// Validation failures emitted by [`M5AiReviewAssistMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5AiReviewAssistMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required governed object class is missing from the matrix.
    RequiredObjectMissing,
    /// An AI-review-assist row is incomplete.
    AiReviewAssistRowIncomplete,
    /// An AI-review-assist row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// An AI-review-assist row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A class declares no AI-review-assist roles.
    SemanticRoleMissing,
    /// The AiReviewFindingRow class declares no AiReviewFindingRow roles.
    FindingRowRoleMissing,
    /// The ReviewScopeSelector class declares no ReviewScopeSelector roles.
    ScopeSelectorRoleMissing,
    /// The PublishToReviewSheet class declares no PublishToReviewSheet roles.
    PublishSheetRoleMissing,
    /// The ResolutionMemoryRow class declares no ResolutionMemoryRow roles.
    ResolutionMemoryRoleMissing,
    /// A class omits required visible-state fields.
    VisibleStateIncomplete,
    /// A class declares no degraded reasons.
    DegradedReasonMissing,
    /// A class declares no surface families.
    SurfaceFamilyMissing,
    /// A class declares no classification stages.
    ClassificationStageMissing,
    /// A class declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A class declares no first consumer surfaces.
    ConsumerSurfacesMissing,
    /// A class declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A class claiming Stable is missing required closure-artifact refs.
    StableObjectMissingClosureArtifact,
    /// A class violates a hard AI-review-assist invariant.
    AiReviewAssistInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5AiReviewAssistMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredObjectMissing => "required_object_missing",
            Self::AiReviewAssistRowIncomplete => "ai_review_assist_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::FindingRowRoleMissing => "finding_row_role_missing",
            Self::ScopeSelectorRoleMissing => "scope_selector_role_missing",
            Self::PublishSheetRoleMissing => "publish_sheet_role_missing",
            Self::ResolutionMemoryRoleMissing => "resolution_memory_role_missing",
            Self::VisibleStateIncomplete => "visible_state_incomplete",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::ClassificationStageMissing => "classification_stage_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableObjectMissingClosureArtifact => "stable_object_missing_closure_artifact",
            Self::AiReviewAssistInvariantViolated => "ai_review_assist_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 AI-review-assist matrix export.
pub fn current_stable_m5_ai_review_assist_matrix_export(
) -> Result<M5AiReviewAssistMatrixPacket, M5AiReviewAssistMatrixArtifactError> {
    let packet: M5AiReviewAssistMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-ai-review-publish-packets/support_export.json"
    )))
    .map_err(M5AiReviewAssistMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5AiReviewAssistMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5AiReviewAssistMatrixPacket,
    violations: &mut Vec<M5AiReviewAssistMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
        M5_AI_REVIEW_ASSIST_MATRIX_DOC_REF,
        M5_AI_REVIEW_FINDING_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_SCOPE_SELECTOR_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_PUBLISH_SHEET_DOMAIN_SCHEMA_REF,
        M5_AI_REVIEW_RESOLUTION_MEMORY_DOMAIN_SCHEMA_REF,
        M5_STABLE_PROOF_INDEX_LANDED_SCHEMA_REF,
        M5_MIGRATION_TASK_ROW_LANDED_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5AiReviewAssistMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5AiReviewAssistMatrixPacket,
    violations: &mut Vec<M5AiReviewAssistMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5AiReviewAssistMatrixViolation::VocabularySetDrift);
    }
}

fn validate_ai_review_assist_rows(
    packet: &M5AiReviewAssistMatrixPacket,
    violations: &mut Vec<M5AiReviewAssistMatrixViolation>,
) {
    let present: BTreeSet<M5AiReviewAssistObject> = packet
        .ai_review_assist_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in M5AiReviewAssistObject::ALL {
        if !present.contains(&required) {
            violations.push(M5AiReviewAssistMatrixViolation::RequiredObjectMissing);
            return;
        }
    }

    for row in &packet.ai_review_assist_rows {
        let class = row.object_class;
        if row.owner_role.trim().is_empty()
            || row.backup_owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5AiReviewAssistMatrixViolation::AiReviewAssistRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5AiReviewAssistMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == class.canonical_domain_schema_ref())
        {
            violations.push(M5AiReviewAssistMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::SemanticRoleMissing);
        }
        if class.declares_finding_row_roles() && row.finding_row_roles.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::FindingRowRoleMissing);
        }
        if class.declares_scope_selector_roles() && row.scope_selector_roles.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::ScopeSelectorRoleMissing);
        }
        if class.declares_publish_sheet_roles() && row.publish_sheet_roles.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::PublishSheetRoleMissing);
        }
        if class.declares_resolution_memory_roles() && row.resolution_memory_roles.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::ResolutionMemoryRoleMissing);
        }
        if !row.required_visible_state.is_complete() {
            violations.push(M5AiReviewAssistMatrixViolation::VisibleStateIncomplete);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::SurfaceFamilyMissing);
        }
        if row.classification_stages.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::ClassificationStageMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_closure_artifact_refs.is_empty() {
            violations.push(M5AiReviewAssistMatrixViolation::StableObjectMissingClosureArtifact);
        }
        if !row.honours_invariants() {
            violations.push(M5AiReviewAssistMatrixViolation::AiReviewAssistInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5AiReviewAssistMatrixPacket,
    violations: &mut Vec<M5AiReviewAssistMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.no_ai_review_finding_publishes_or_merges_implicitly,
        review.every_covered_object_class_names_owner_backup_owner_and_first_consumer,
        review.local_draft_state_is_mechanically_distinct_from_provider_committed_state,
        review.every_finding_names_its_class_severity_and_confidence,
        review.every_finding_names_its_analyzed_diff_scope_and_instruction_source,
        review.every_publish_names_its_mode_and_provider_destination_before_mutation,
        review.publish_or_export_fallback_is_named_for_every_finding,
        review.no_stale_or_outdated_finding_is_shown_as_current,
        review.no_local_draft_or_evidence_is_lost_when_publish_fails,
        review.every_object_declares_classification_stages,
        review.every_object_declares_accessibility_route,
        review.support_export_reads_single_ai_review_assist_source,
        review.review_ai_provider_pending_and_support_bind_to_single_source,
        review.later_rows_cannot_invent_parallel_ai_review_assist_vocabulary,
        review.ai_review_assist_truth_survives_zoom_and_high_contrast,
        review.claims_narrow_automatically_when_matrix_row_missing_or_stale,
    ] {
        if !ok {
            violations.push(M5AiReviewAssistMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5AiReviewAssistMatrixPacket,
    violations: &mut Vec<M5AiReviewAssistMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_detail_and_ai_panel_consume_shared_ai_review_finding_truth,
        projection.pending_review_and_provider_publish_consume_shared_publish_destination_truth,
        projection.help_and_support_export_consume_shared_finding_lifecycle_truth,
        projection.docs_help_and_screenshots_read_single_ai_review_assist_source,
        projection.findings_bind_to_shared_resolution_memory_relation,
        projection.support_export_reads_single_ai_review_assist_source,
    ] {
        if !ok {
            violations.push(M5AiReviewAssistMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5AiReviewAssistMatrixPacket,
    violations: &mut Vec<M5AiReviewAssistMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5AiReviewAssistMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5AiReviewAssistMatrixPacket,
    violations: &mut Vec<M5AiReviewAssistMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.ai_review_assist_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5AiReviewAssistMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled vocabulary
/// deliberately uses finding / review / publish / provider words; what is rejected is a raw secret *value*
/// shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
