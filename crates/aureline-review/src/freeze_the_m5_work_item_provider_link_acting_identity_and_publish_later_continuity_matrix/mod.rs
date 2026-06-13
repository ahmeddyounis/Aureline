//! Frozen M5 work-item, provider-link, acting-identity, and publish-later continuity matrix.
//!
//! This module locks the canonical M5 governance vocabulary for provider-linked
//! work-item surfaces into one export-safe packet. The packet freezes:
//!
//! - [`ProviderWorkItemObjectVocabularyRow`] — the object vocabulary for
//!   provider-backed work items, review-linked change intent, browser handoff
//!   packets, deferred-publish packets, imported snapshots, and external-event
//!   envelopes.
//! - [`ActingIdentityVocabularyRow`] — the acting-identity and effective-scope
//!   vocabulary that distinguishes human accounts, installation grants,
//!   delegated credentials, browser-only fallbacks, denied scope, and
//!   publish-later local-draft paths.
//! - [`ProviderTruthStateVocabularyRow`] — the state vocabulary that keeps
//!   local drafts, queued publishes, provider-committed state, stale
//!   snapshots, partial scope, mirror-derived state, and callback-denied
//!   objects visibly distinct.
//! - [`M5ProviderWorkItemGovernanceLaneRow`] — the lane-level qualification,
//!   evidence, downgrade triggers, rollback posture, source contracts, and
//!   consumer-surface parity for work-item object governance, provider-linked
//!   mutations, acting identity, browser handoff, deferred publish, and
//!   provider-event reconciliation.
//!
//! The matrix is the canonical source of truth for whether provider-linked
//! team-workflow rows may claim Stable or narrower maturity, and for the exact
//! downgrade rules that apply when provider authority, callback reconciliation,
//! or publish-later continuity proof goes stale. Provider-owned objects never
//! masquerade as canonical local truth; local drafts, queued publishes,
//! provider-committed state, and callback-denied events remain visibly
//! separate; browser handoff stays typed and return-anchor safe; and imported
//! provider events only mutate through typed, deduplicated reconciliation
//! paths.
//!
//! [`canonical_m5_provider_workitem_governance`] builds the frozen matrix and
//! [`current_stable_m5_provider_workitem_governance_export`] reads and
//! validates the checked-in support export, so review, work-item, incident,
//! companion, docs/help, and release surfaces can reuse one governed
//! vocabulary instead of inventing per-surface wording.
//!
//! The boundary schema is
//! [`schemas/review/freeze-the-m5-work-item-provider-link-acting-identity-and-publish-later-continuity-matrix.schema.json`](../../../../schemas/review/freeze-the-m5-work-item-provider-link-acting-identity-and-publish-later-continuity-matrix.schema.json).
//! The contract doc is
//! [`docs/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix.md`](../../../../docs/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix/`](../../../../fixtures/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ProviderWorkItemGovernancePacket`].
pub const M5_PROVIDER_WORKITEM_GOVERNANCE_RECORD_KIND: &str =
    "freeze_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix";

/// Schema version for M5 provider-work-item governance records.
pub const M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_REF: &str =
    "schemas/review/freeze-the-m5-work-item-provider-link-acting-identity-and-publish-later-continuity-matrix.schema.json";

/// Repo-relative path of the M5 provider-work-item governance contract doc.
pub const M5_PROVIDER_WORKITEM_GOVERNANCE_DOC_REF: &str =
    "docs/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix.md";

/// Repo-relative path of the provider-linked review object model contract.
pub const M5_PROVIDER_WORKITEM_PROVIDER_LINK_CONTRACT_REF: &str =
    "schemas/review/provider_linked_review_stabilization.schema.json";

/// Repo-relative path of the stable work-item transition contract.
pub const M5_PROVIDER_WORKITEM_TRANSITION_CONTRACT_REF: &str =
    "schemas/review/stable_work_item_status_transition_review.schema.json";

/// Repo-relative path of the work-item linkage finalization contract.
pub const M5_PROVIDER_WORKITEM_LINKAGE_CONTRACT_REF: &str =
    "schemas/review/finalize_issue_and_work_item_linkage_with_branch.schema.json";

/// Repo-relative path of the browser/provider handoff continuity contract.
pub const M5_PROVIDER_WORKITEM_HANDOFF_CONTRACT_REF: &str =
    "schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json";

/// Repo-relative path of the review/export bundle and publish-later contract.
pub const M5_PROVIDER_WORKITEM_PUBLISH_LATER_CONTRACT_REF: &str =
    "schemas/review/add-review-export-bundles-publish-later-packets-and-offline-follow-up-flows-for-code-review-and-ci-surfaces.schema.json";

/// Repo-relative path of the canonical provider event-ingestion contract.
pub const M5_PROVIDER_WORKITEM_EVENT_INGESTION_CONTRACT_REF: &str =
    "schemas/providers/provider_event_ingestion.schema.json";

/// Repo-relative path of the canonical provider event-ingestion contract doc.
pub const M5_PROVIDER_WORKITEM_EVENT_INGESTION_DOC_REF: &str =
    "docs/providers/m5/event_ingestion.md";

/// Repo-relative path of the incident-workspace contract.
pub const M5_PROVIDER_WORKITEM_INCIDENT_CONTRACT_REF: &str =
    "docs/ops/incident_workspace_contract.md";

/// Repo-relative path of the browser/mobile companion qualification contract.
pub const M5_PROVIDER_WORKITEM_COMPANION_CONTRACT_REF: &str =
    "docs/help/browser-mobile-companion-surface-qualification.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROVIDER_WORKITEM_GOVERNANCE_FIXTURE_DIR: &str =
    "fixtures/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROVIDER_WORKITEM_GOVERNANCE_ARTIFACT_REF: &str =
    "artifacts/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_PROVIDER_WORKITEM_GOVERNANCE_SUMMARY_REF: &str =
    "artifacts/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix.md";

/// One of the six M5 provider-work-item governance lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderWorkItemGovernanceLane {
    /// Canonical vocabulary for provider-linked work-item and related object classes.
    WorkItemObjectVocabulary,
    /// Provider-linked mutation authority, preview, and downgrade behavior.
    ProviderLinkedMutation,
    /// Acting identity and effective-scope vocabulary.
    ActingIdentityAndEffectiveScope,
    /// Typed browser handoff with return-anchor continuity.
    BrowserHandoffContinuity,
    /// Local draft, queued publish, and deferred replay continuity.
    DeferredPublishContinuity,
    /// Callback, webhook, and import-session reconciliation.
    ProviderEventReconciliation,
}

impl M5ProviderWorkItemGovernanceLane {
    /// Every lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkItemObjectVocabulary,
        Self::ProviderLinkedMutation,
        Self::ActingIdentityAndEffectiveScope,
        Self::BrowserHandoffContinuity,
        Self::DeferredPublishContinuity,
        Self::ProviderEventReconciliation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkItemObjectVocabulary => "work_item_object_vocabulary",
            Self::ProviderLinkedMutation => "provider_linked_mutation",
            Self::ActingIdentityAndEffectiveScope => "acting_identity_and_effective_scope",
            Self::BrowserHandoffContinuity => "browser_handoff_continuity",
            Self::DeferredPublishContinuity => "deferred_publish_continuity",
            Self::ProviderEventReconciliation => "provider_event_reconciliation",
        }
    }
}

/// Qualification class for a provider-work-item governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderWorkItemGovernanceQualificationClass {
    /// Lane qualifies for the Stable claim.
    Stable,
    /// Lane is narrowed to Beta.
    Beta,
    /// Lane is narrowed to Preview.
    Preview,
    /// Lane is experimental and not claimed.
    Experimental,
    /// Lane is unavailable on this build.
    Unavailable,
    /// Lane is held pending recovery.
    Held,
}

impl M5ProviderWorkItemGovernanceQualificationClass {
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

    /// Whether the lane may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Narrows the qualification one step toward [`Self::Held`].
    pub const fn narrowed_one_step(self) -> Self {
        match self {
            Self::Stable => Self::Beta,
            Self::Beta => Self::Preview,
            Self::Preview => Self::Experimental,
            Self::Experimental | Self::Unavailable | Self::Held => Self::Held,
        }
    }
}

/// Evidence requirement level for a governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderWorkItemGovernanceEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Evidence does not apply to the current qualification.
    NotApplicable,
}

impl M5ProviderWorkItemGovernanceEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow a governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderWorkItemGovernanceDowngradeTrigger {
    /// Authority or scope proof went stale.
    ProviderAuthorityStale,
    /// Scope resolution drifted or narrowed.
    EffectiveScopeDrift,
    /// Browser-only policy or host mismatch blocks in-product mutation.
    BrowserOnlyFallback,
    /// Typed handoff can no longer prove its return anchor.
    ReturnAnchorUnproven,
    /// Local draft or queued publish continuity proof went stale.
    PublishLaterContinuityStale,
    /// Callback or webhook reconciliation proof went stale.
    CallbackReconciliationStale,
    /// Deduplication or replay ledger proof is missing.
    ReplayLedgerUnavailable,
    /// A callback or webhook was denied by policy.
    CallbackDenied,
    /// Imported snapshot freshness drifted below the allowed floor.
    ImportedSnapshotStale,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl M5ProviderWorkItemGovernanceDowngradeTrigger {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthorityStale => "provider_authority_stale",
            Self::EffectiveScopeDrift => "effective_scope_drift",
            Self::BrowserOnlyFallback => "browser_only_fallback",
            Self::ReturnAnchorUnproven => "return_anchor_unproven",
            Self::PublishLaterContinuityStale => "publish_later_continuity_stale",
            Self::CallbackReconciliationStale => "callback_reconciliation_stale",
            Self::ReplayLedgerUnavailable => "replay_ledger_unavailable",
            Self::CallbackDenied => "callback_denied",
            Self::ImportedSnapshotStale => "imported_snapshot_stale",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a governance lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderWorkItemGovernanceRollbackPosture {
    /// Local draft and offline continuity preserve user intent without provider mutation.
    LocalDraftPreserved,
    /// Publish remains attributable and may be retried or exported later.
    AttributablePublishLater,
    /// Browser handoff preserves reason, destination, and return anchor.
    ReturnAnchorPreserved,
    /// Callback denial remains auditable without mutating local truth.
    AuditOnlyNoMutation,
    /// Provider-committed truth stays distinct from cached or mirror state.
    ProviderCommittedDistinct,
    /// Not applicable for the lane's current qualification.
    NotApplicable,
}

impl M5ProviderWorkItemGovernanceRollbackPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraftPreserved => "local_draft_preserved",
            Self::AttributablePublishLater => "attributable_publish_later",
            Self::ReturnAnchorPreserved => "return_anchor_preserved",
            Self::AuditOnlyNoMutation => "audit_only_no_mutation",
            Self::ProviderCommittedDistinct => "provider_committed_distinct",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Consumer surface that must project this matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderWorkItemGovernanceConsumerSurface {
    /// Review workspace and related review strips.
    ReviewWorkspace,
    /// Work-item detail, transition, and linkage surfaces.
    WorkItemDetail,
    /// Incident workspace and related evidence surfaces.
    IncidentWorkspace,
    /// Browser or mobile companion triage surfaces.
    CompanionTriage,
    /// Typed browser handoff cards and follow-up surfaces.
    BrowserHandoff,
    /// CLI or headless JSON output.
    CliHeadless,
    /// Support export or redaction-safe field packet.
    SupportExport,
    /// Help, docs, or operator-facing explainability surface.
    DocsHelp,
    /// Release qualification, shiproom, or evidence index surface.
    ReleasePacket,
}

impl M5ProviderWorkItemGovernanceConsumerSurface {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewWorkspace => "review_workspace",
            Self::WorkItemDetail => "work_item_detail",
            Self::IncidentWorkspace => "incident_workspace",
            Self::CompanionTriage => "companion_triage",
            Self::BrowserHandoff => "browser_handoff",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::DocsHelp => "docs_help",
            Self::ReleasePacket => "release_packet",
        }
    }
}

/// Canonical provider-linked object class governed by the matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderWorkItemObjectClass {
    /// Provider-backed work-item, issue, ticket, or incident object.
    ProviderWorkItem,
    /// Review-linked change-intent object that joins local work to provider state.
    ReviewLinkedChangeIntent,
    /// Typed browser-handoff packet that leaves the product boundary.
    BrowserHandoffPacket,
    /// Deferred publish, retry, or export-safe local packet.
    DeferredPublishPacket,
    /// Imported provider snapshot, mirror record, or cached overlay.
    ImportedSnapshot,
    /// Callback, webhook, polling, or browser-return event envelope.
    ProviderEventEnvelope,
}

impl ProviderWorkItemObjectClass {
    /// Every object class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderWorkItem,
        Self::ReviewLinkedChangeIntent,
        Self::BrowserHandoffPacket,
        Self::DeferredPublishPacket,
        Self::ImportedSnapshot,
        Self::ProviderEventEnvelope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderWorkItem => "provider_work_item",
            Self::ReviewLinkedChangeIntent => "review_linked_change_intent",
            Self::BrowserHandoffPacket => "browser_handoff_packet",
            Self::DeferredPublishPacket => "deferred_publish_packet",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::ProviderEventEnvelope => "provider_event_envelope",
        }
    }
}

/// Acting-identity class for a provider-linked action or fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActingIdentityClass {
    /// The action executes as the signed-in human account.
    HumanAccount,
    /// The action executes through an installation or app grant.
    InstallationGrant,
    /// The action executes through a delegated credential.
    DelegatedCredential,
    /// Policy or host rules allow only a browser handoff.
    BrowserOnlyFallback,
    /// Effective scope denies the requested provider mutation.
    DeniedScope,
    /// The action is preserved only as a local draft for later publish.
    PublishLaterLocalDraft,
}

impl ActingIdentityClass {
    /// Every acting-identity class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HumanAccount,
        Self::InstallationGrant,
        Self::DelegatedCredential,
        Self::BrowserOnlyFallback,
        Self::DeniedScope,
        Self::PublishLaterLocalDraft,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanAccount => "human_account",
            Self::InstallationGrant => "installation_grant",
            Self::DelegatedCredential => "delegated_credential",
            Self::BrowserOnlyFallback => "browser_only_fallback",
            Self::DeniedScope => "denied_scope",
            Self::PublishLaterLocalDraft => "publish_later_local_draft",
        }
    }
}

/// Effective-scope result bound to an acting identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EffectiveScopeClass {
    /// The provider mutation may proceed in-product.
    ProviderMutation,
    /// Only limited comment or linkage mutation is allowed.
    LimitedCommentLink,
    /// The mutation must fall back to a typed browser handoff.
    BrowserOnlyFallback,
    /// The requested mutation is denied.
    DeniedScope,
    /// The action is preserved locally for later publish.
    PublishLaterLocalDraft,
}

impl EffectiveScopeClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderMutation => "provider_mutation",
            Self::LimitedCommentLink => "limited_comment_link",
            Self::BrowserOnlyFallback => "browser_only_fallback",
            Self::DeniedScope => "denied_scope",
            Self::PublishLaterLocalDraft => "publish_later_local_draft",
        }
    }
}

/// Truth-state class that keeps local and provider state visibly separate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderTruthStateClass {
    /// Local-only draft not yet committed to the provider.
    LocalDraft,
    /// Queued publish or replayable local intent awaiting review or reconnect.
    QueuedPublish,
    /// Provider-committed state confirmed by a current authority path.
    ProviderCommitted,
    /// Imported or cached state whose freshness has gone stale.
    StaleSnapshot,
    /// Object state visible only through a partial or narrowed effective scope.
    PartialScope,
    /// Imported state derived from a mirror rather than the live provider path.
    MirrorDerived,
    /// Callback or webhook mutation denied before local state changed.
    CallbackDenied,
}

impl ProviderTruthStateClass {
    /// Every truth-state class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LocalDraft,
        Self::QueuedPublish,
        Self::ProviderCommitted,
        Self::StaleSnapshot,
        Self::PartialScope,
        Self::MirrorDerived,
        Self::CallbackDenied,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDraft => "local_draft",
            Self::QueuedPublish => "queued_publish",
            Self::ProviderCommitted => "provider_committed",
            Self::StaleSnapshot => "stale_snapshot",
            Self::PartialScope => "partial_scope",
            Self::MirrorDerived => "mirror_derived",
            Self::CallbackDenied => "callback_denied",
        }
    }
}

/// One row in the frozen M5 provider-work-item governance matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderWorkItemGovernanceLaneRow {
    /// Governance lane governed by this row.
    pub lane: M5ProviderWorkItemGovernanceLane,
    /// Qualification class earned by this lane.
    pub qualification: M5ProviderWorkItemGovernanceQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Evidence requirement level.
    pub evidence_requirement: M5ProviderWorkItemGovernanceEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<M5ProviderWorkItemGovernanceDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5ProviderWorkItemGovernanceRollbackPosture,
    /// Source contract refs consumed by this lane.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this lane.
    pub consumer_surfaces: Vec<M5ProviderWorkItemGovernanceConsumerSurface>,
}

/// Frozen vocabulary row for a provider-linked object class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderWorkItemObjectVocabularyRow {
    /// Provider-linked object class.
    pub object_class: ProviderWorkItemObjectClass,
    /// Stable summary of the object's contract.
    pub summary: String,
    /// Whether provider authority remains visible when this object appears.
    pub provider_authority_visible: bool,
    /// Whether acting identity remains visible when this object appears.
    pub acting_identity_visible: bool,
    /// Whether effective scope remains visible when this object appears.
    pub effective_scope_visible: bool,
    /// Whether the object preserves a local anchor or durable local identity.
    pub local_anchor_required: bool,
    /// Truth-state classes this object may project.
    pub state_classes: Vec<ProviderTruthStateClass>,
}

/// Frozen vocabulary row for an acting-identity / effective-scope pairing.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActingIdentityVocabularyRow {
    /// Acting-identity class.
    pub identity_class: ActingIdentityClass,
    /// Effective scope that this row discloses.
    pub effective_scope_class: EffectiveScopeClass,
    /// Stable summary of the identity/scope contract.
    pub summary: String,
    /// Whether this row may publish directly to the provider.
    pub publish_now_allowed: bool,
    /// Whether this row requires a typed browser handoff for mutation.
    pub browser_handoff_required: bool,
    /// Whether this row blocks mutation while preserving inspectability.
    pub denied_scope_visible: bool,
    /// Whether this row preserves local draft continuity for later publish.
    pub local_draft_continuity_preserved: bool,
}

/// Frozen vocabulary row for a provider-linked truth state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderTruthStateVocabularyRow {
    /// Truth-state class.
    pub truth_state_class: ProviderTruthStateClass,
    /// Stable summary of the state contract.
    pub summary: String,
    /// Whether this state is provider authoritative.
    pub provider_authoritative: bool,
    /// Whether this state preserves local continuity.
    pub local_continuity_preserved: bool,
    /// Whether replay or review is required before mutation resumes.
    pub review_before_replay_required: bool,
    /// Whether callback or import lineage remains visible.
    pub lineage_visible: bool,
}

/// Trust and provenance review block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderWorkItemGovernanceTrustReview {
    /// Provider-owned objects never masquerade as canonical local truth.
    pub provider_objects_never_masquerade_as_local_truth: bool,
    /// Publish mode is always explicit as local draft, publish later, publish now, or handoff.
    pub publish_mode_explicit: bool,
    /// Acting identity is always visible on claimed mutation lanes.
    pub acting_identity_visible: bool,
    /// Effective scope result is always visible on claimed mutation lanes.
    pub effective_scope_visible: bool,
    /// Local draft, queued publish, and provider-committed states remain separate.
    pub local_and_provider_state_distinct: bool,
    /// Imported snapshots never claim provider-committed freshness.
    pub imported_snapshot_never_claims_provider_commit: bool,
    /// Browser handoff stays typed and preserves a return anchor.
    pub browser_handoff_return_anchor_safe: bool,
    /// Deferred publish survives restart.
    pub deferred_publish_survives_restart: bool,
    /// Deferred publish survives reconnect.
    pub deferred_publish_survives_reconnect: bool,
    /// Callback or webhook events are deduplicated before state mutation.
    pub callback_events_deduplicated: bool,
    /// Callback denials remain auditable without mutating user-visible truth.
    pub callback_denials_auditable: bool,
    /// Downgrade narrows the claim rather than hiding the row.
    pub downgrade_narrows_instead_of_hides: bool,
}

/// Consumer projection block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderWorkItemGovernanceConsumerProjection {
    /// Review workspace reuses the canonical vocabulary.
    pub review_workspace_reuses_canonical_vocabulary: bool,
    /// Work-item detail shows acting identity, effective scope, and truth state.
    pub work_item_detail_shows_identity_scope_and_state: bool,
    /// Incident workspace preserves provider lineage and narrow continuity wording.
    pub incident_workspace_preserves_provider_lineage: bool,
    /// Companion triage shows scoped draft, stale, or handoff truth.
    pub companion_triage_shows_scoped_draft_or_handoff_truth: bool,
    /// Browser handoff shows reason, destination class, and return anchor.
    pub browser_handoff_shows_reason_and_return_anchor: bool,
    /// CLI or headless surfaces show qualification truth.
    pub cli_headless_shows_qualification: bool,
    /// Support export shows provider authority, acting identity, and truth state.
    pub support_export_shows_provider_authority_and_state: bool,
    /// Docs/help surfaces reuse the publish-mode and acting-identity terminology.
    pub docs_help_shows_publish_mode_terminology: bool,
    /// Release packets show qualification and narrowing truth.
    pub release_packet_shows_qualification_and_narrowing: bool,
}

/// Proof freshness block for the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderWorkItemGovernanceProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// Whether stale authority proof auto-narrows affected lanes.
    pub auto_narrow_on_authority_stale: bool,
    /// Whether stale reconciliation proof auto-narrows affected lanes.
    pub auto_narrow_on_reconciliation_stale: bool,
    /// Whether stale publish-later continuity proof auto-narrows affected lanes.
    pub auto_narrow_on_publish_later_stale: bool,
}

/// Constructor input for [`M5ProviderWorkItemGovernancePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProviderWorkItemGovernancePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Frozen object vocabulary rows.
    pub object_vocabulary_rows: Vec<ProviderWorkItemObjectVocabularyRow>,
    /// Frozen acting-identity rows.
    pub acting_identity_rows: Vec<ActingIdentityVocabularyRow>,
    /// Frozen truth-state rows.
    pub truth_state_rows: Vec<ProviderTruthStateVocabularyRow>,
    /// Governance lane rows.
    pub lane_rows: Vec<M5ProviderWorkItemGovernanceLaneRow>,
    /// Trust review block.
    pub trust_review: M5ProviderWorkItemGovernanceTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderWorkItemGovernanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderWorkItemGovernanceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Degradation inputs that automatically narrow claimed governance lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct M5ProviderWorkItemGovernanceDegradation {
    /// Provider-authority proof went stale.
    pub provider_authority_stale: bool,
    /// Effective-scope resolution proof went stale or drifted.
    pub effective_scope_drifted: bool,
    /// Typed browser handoff can no longer prove its return anchor.
    pub browser_return_anchor_unproven: bool,
    /// Deferred publish continuity proof went stale.
    pub publish_later_continuity_stale: bool,
    /// Callback or webhook reconciliation proof went stale.
    pub callback_reconciliation_stale: bool,
    /// Replay ledger proof is unavailable.
    pub replay_ledger_unavailable: bool,
    /// An upstream dependency lane narrowed.
    pub upstream_dependency_narrowed: bool,
}

/// Export-safe frozen M5 provider-work-item governance packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderWorkItemGovernancePacket {
    /// Record kind; must equal [`M5_PROVIDER_WORKITEM_GOVERNANCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Frozen object vocabulary rows.
    pub object_vocabulary_rows: Vec<ProviderWorkItemObjectVocabularyRow>,
    /// Frozen acting-identity rows.
    pub acting_identity_rows: Vec<ActingIdentityVocabularyRow>,
    /// Frozen truth-state rows.
    pub truth_state_rows: Vec<ProviderTruthStateVocabularyRow>,
    /// Governance lane rows.
    pub lane_rows: Vec<M5ProviderWorkItemGovernanceLaneRow>,
    /// Trust review block.
    pub trust_review: M5ProviderWorkItemGovernanceTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderWorkItemGovernanceConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderWorkItemGovernanceProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ProviderWorkItemGovernancePacket {
    /// Builds an M5 provider-work-item governance packet from stable input.
    pub fn new(input: M5ProviderWorkItemGovernancePacketInput) -> Self {
        Self {
            record_kind: M5_PROVIDER_WORKITEM_GOVERNANCE_RECORD_KIND.to_owned(),
            schema_version: M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            object_vocabulary_rows: input.object_vocabulary_rows,
            acting_identity_rows: input.acting_identity_rows,
            truth_state_rows: input.truth_state_rows,
            lane_rows: input.lane_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 provider-work-item governance invariants.
    pub fn validate(&self) -> Vec<M5ProviderWorkItemGovernanceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROVIDER_WORKITEM_GOVERNANCE_RECORD_KIND {
            violations.push(M5ProviderWorkItemGovernanceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_VERSION {
            violations.push(M5ProviderWorkItemGovernanceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ProviderWorkItemGovernanceViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_object_rows(self, &mut violations);
        validate_identity_rows(self, &mut violations);
        validate_truth_state_rows(self, &mut violations);
        validate_lane_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("m5 provider-work-item governance packet serializes"),
        ) {
            violations.push(M5ProviderWorkItemGovernanceViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Applies downgrade automation and returns a narrowed packet copy.
    pub fn apply_downgrade_automation(
        &self,
        degradation: M5ProviderWorkItemGovernanceDegradation,
    ) -> Self {
        let mut packet = self.clone();
        for row in &mut packet.lane_rows {
            let affected = degradation.upstream_dependency_narrowed
                || match row.lane {
                    M5ProviderWorkItemGovernanceLane::WorkItemObjectVocabulary => {
                        degradation.provider_authority_stale
                            || degradation.effective_scope_drifted
                            || degradation.publish_later_continuity_stale
                            || degradation.callback_reconciliation_stale
                    }
                    M5ProviderWorkItemGovernanceLane::ProviderLinkedMutation => {
                        degradation.provider_authority_stale || degradation.effective_scope_drifted
                    }
                    M5ProviderWorkItemGovernanceLane::ActingIdentityAndEffectiveScope => {
                        degradation.provider_authority_stale || degradation.effective_scope_drifted
                    }
                    M5ProviderWorkItemGovernanceLane::BrowserHandoffContinuity => {
                        degradation.browser_return_anchor_unproven
                    }
                    M5ProviderWorkItemGovernanceLane::DeferredPublishContinuity => {
                        degradation.publish_later_continuity_stale
                    }
                    M5ProviderWorkItemGovernanceLane::ProviderEventReconciliation => {
                        degradation.callback_reconciliation_stale
                            || degradation.replay_ledger_unavailable
                    }
                };
            if affected {
                row.qualification = row.qualification.narrowed_one_step();
            }
        }
        packet
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 provider-work-item governance packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_lanes = self
            .lane_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Provider-Work-Item Governance Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Object classes: {} · identities: {} · truth states: {}\n",
            self.object_vocabulary_rows.len(),
            self.acting_identity_rows.len(),
            self.truth_state_rows.len()
        ));
        out.push_str(&format!(
            "- Lanes: {} ({} stable)\n",
            self.lane_rows.len(),
            stable_lanes
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Lanes\n\n");
        for row in &self.lane_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.lane.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Evidence: {} ({} refs)\n",
                row.evidence_requirement.as_str(),
                row.required_evidence_packet_refs.len()
            ));
            out.push_str(&format!(
                "  - Rollback: {}\n",
                row.rollback_posture.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 provider-work-item governance export.
#[derive(Debug)]
pub enum M5ProviderWorkItemGovernanceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ProviderWorkItemGovernanceViolation>),
}

impl fmt::Display for M5ProviderWorkItemGovernanceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 provider-work-item governance export parse failed: {error}"
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
                    "m5 provider-work-item governance export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ProviderWorkItemGovernanceArtifactError {}

/// Validation failures emitted by [`M5ProviderWorkItemGovernancePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ProviderWorkItemGovernanceViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required lane is missing from the matrix.
    RequiredLaneMissing,
    /// A required provider-linked object class is missing.
    RequiredObjectClassMissing,
    /// A required acting-identity class is missing.
    RequiredActingIdentityMissing,
    /// A required truth-state class is missing.
    RequiredTruthStateMissing,
    /// A lane row is incomplete.
    LaneRowIncomplete,
    /// A stable lane is missing required evidence.
    StableLaneMissingEvidence,
    /// A lane has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A lane has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// An object-vocabulary row is incomplete.
    ObjectVocabularyIncomplete,
    /// An acting-identity row is incomplete.
    ActingIdentityVocabularyIncomplete,
    /// A truth-state row is incomplete.
    TruthStateVocabularyIncomplete,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ProviderWorkItemGovernanceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredLaneMissing => "required_lane_missing",
            Self::RequiredObjectClassMissing => "required_object_class_missing",
            Self::RequiredActingIdentityMissing => "required_acting_identity_missing",
            Self::RequiredTruthStateMissing => "required_truth_state_missing",
            Self::LaneRowIncomplete => "lane_row_incomplete",
            Self::StableLaneMissingEvidence => "stable_lane_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ObjectVocabularyIncomplete => "object_vocabulary_incomplete",
            Self::ActingIdentityVocabularyIncomplete => "acting_identity_vocabulary_incomplete",
            Self::TruthStateVocabularyIncomplete => "truth_state_vocabulary_incomplete",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Builds the canonical frozen M5 provider-work-item governance packet.
pub fn canonical_m5_provider_workitem_governance() -> M5ProviderWorkItemGovernancePacket {
    M5ProviderWorkItemGovernancePacket::new(M5ProviderWorkItemGovernancePacketInput {
        packet_id: "m5-provider-workitem-governance:stable:0001".to_owned(),
        matrix_label: "M5 Provider-Work-Item Governance Matrix".to_owned(),
        object_vocabulary_rows: canonical_object_vocabulary_rows(),
        acting_identity_rows: canonical_acting_identity_rows(),
        truth_state_rows: canonical_truth_state_rows(),
        lane_rows: canonical_lane_rows(),
        trust_review: canonical_trust_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness: canonical_proof_freshness(),
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T00:00:00Z".to_owned(),
    })
}

/// Reads and validates the checked-in stable M5 provider-work-item governance export.
pub fn current_stable_m5_provider_workitem_governance_export(
) -> Result<M5ProviderWorkItemGovernancePacket, M5ProviderWorkItemGovernanceArtifactError> {
    let packet: M5ProviderWorkItemGovernancePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix/support_export.json"
    )))
    .map_err(M5ProviderWorkItemGovernanceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProviderWorkItemGovernanceArtifactError::Validation(
            violations,
        ))
    }
}

/// Returns the canonical source contract refs for the packet.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_REF.to_owned(),
        M5_PROVIDER_WORKITEM_GOVERNANCE_DOC_REF.to_owned(),
        M5_PROVIDER_WORKITEM_PROVIDER_LINK_CONTRACT_REF.to_owned(),
        M5_PROVIDER_WORKITEM_TRANSITION_CONTRACT_REF.to_owned(),
        M5_PROVIDER_WORKITEM_LINKAGE_CONTRACT_REF.to_owned(),
        M5_PROVIDER_WORKITEM_HANDOFF_CONTRACT_REF.to_owned(),
        M5_PROVIDER_WORKITEM_PUBLISH_LATER_CONTRACT_REF.to_owned(),
        M5_PROVIDER_WORKITEM_EVENT_INGESTION_CONTRACT_REF.to_owned(),
        M5_PROVIDER_WORKITEM_EVENT_INGESTION_DOC_REF.to_owned(),
        M5_PROVIDER_WORKITEM_INCIDENT_CONTRACT_REF.to_owned(),
        M5_PROVIDER_WORKITEM_COMPANION_CONTRACT_REF.to_owned(),
    ]
}

/// Returns the canonical consumer projection for the packet.
pub fn canonical_consumer_projection() -> M5ProviderWorkItemGovernanceConsumerProjection {
    M5ProviderWorkItemGovernanceConsumerProjection {
        review_workspace_reuses_canonical_vocabulary: true,
        work_item_detail_shows_identity_scope_and_state: true,
        incident_workspace_preserves_provider_lineage: true,
        companion_triage_shows_scoped_draft_or_handoff_truth: true,
        browser_handoff_shows_reason_and_return_anchor: true,
        cli_headless_shows_qualification: true,
        support_export_shows_provider_authority_and_state: true,
        docs_help_shows_publish_mode_terminology: true,
        release_packet_shows_qualification_and_narrowing: true,
    }
}

fn canonical_object_vocabulary_rows() -> Vec<ProviderWorkItemObjectVocabularyRow> {
    vec![
        ProviderWorkItemObjectVocabularyRow {
            object_class: ProviderWorkItemObjectClass::ProviderWorkItem,
            summary: "Provider-backed work item, issue, ticket, or incident object that stays visibly distinct from local draft and cached overlays".to_owned(),
            provider_authority_visible: true,
            acting_identity_visible: true,
            effective_scope_visible: true,
            local_anchor_required: true,
            state_classes: vec![
                ProviderTruthStateClass::LocalDraft,
                ProviderTruthStateClass::QueuedPublish,
                ProviderTruthStateClass::ProviderCommitted,
                ProviderTruthStateClass::StaleSnapshot,
                ProviderTruthStateClass::PartialScope,
            ],
        },
        ProviderWorkItemObjectVocabularyRow {
            object_class: ProviderWorkItemObjectClass::ReviewLinkedChangeIntent,
            summary: "Review-linked change intent joins local branch or review context to provider state without claiming provider commit until publish completes".to_owned(),
            provider_authority_visible: true,
            acting_identity_visible: true,
            effective_scope_visible: true,
            local_anchor_required: true,
            state_classes: vec![
                ProviderTruthStateClass::LocalDraft,
                ProviderTruthStateClass::QueuedPublish,
                ProviderTruthStateClass::ProviderCommitted,
                ProviderTruthStateClass::PartialScope,
            ],
        },
        ProviderWorkItemObjectVocabularyRow {
            object_class: ProviderWorkItemObjectClass::BrowserHandoffPacket,
            summary: "Typed browser-handoff packet records reason, destination class, acting identity, and return anchor instead of flattening the escape to a raw URL".to_owned(),
            provider_authority_visible: true,
            acting_identity_visible: true,
            effective_scope_visible: true,
            local_anchor_required: true,
            state_classes: vec![
                ProviderTruthStateClass::QueuedPublish,
                ProviderTruthStateClass::PartialScope,
            ],
        },
        ProviderWorkItemObjectVocabularyRow {
            object_class: ProviderWorkItemObjectClass::DeferredPublishPacket,
            summary: "Deferred publish packet preserves requested mutation, redaction-safe evidence, and replay lineage across restart, reconnect, export, and support handoff".to_owned(),
            provider_authority_visible: true,
            acting_identity_visible: true,
            effective_scope_visible: true,
            local_anchor_required: true,
            state_classes: vec![
                ProviderTruthStateClass::LocalDraft,
                ProviderTruthStateClass::QueuedPublish,
            ],
        },
        ProviderWorkItemObjectVocabularyRow {
            object_class: ProviderWorkItemObjectClass::ImportedSnapshot,
            summary: "Imported snapshot or mirror-derived overlay remains inspectable as imported or stale and never upgrades itself to provider-committed truth".to_owned(),
            provider_authority_visible: true,
            acting_identity_visible: false,
            effective_scope_visible: false,
            local_anchor_required: true,
            state_classes: vec![
                ProviderTruthStateClass::StaleSnapshot,
                ProviderTruthStateClass::MirrorDerived,
                ProviderTruthStateClass::PartialScope,
            ],
        },
        ProviderWorkItemObjectVocabularyRow {
            object_class: ProviderWorkItemObjectClass::ProviderEventEnvelope,
            summary: "Callback, webhook, polling, and browser-return events enter through one typed external-event envelope with dedupe, deny, and replay lineage".to_owned(),
            provider_authority_visible: true,
            acting_identity_visible: true,
            effective_scope_visible: true,
            local_anchor_required: true,
            state_classes: vec![
                ProviderTruthStateClass::ProviderCommitted,
                ProviderTruthStateClass::MirrorDerived,
                ProviderTruthStateClass::CallbackDenied,
            ],
        },
    ]
}

fn canonical_acting_identity_rows() -> Vec<ActingIdentityVocabularyRow> {
    vec![
        ActingIdentityVocabularyRow {
            identity_class: ActingIdentityClass::HumanAccount,
            effective_scope_class: EffectiveScopeClass::ProviderMutation,
            summary: "The signed-in human account may mutate the provider directly when current scope proof remains valid".to_owned(),
            publish_now_allowed: true,
            browser_handoff_required: false,
            denied_scope_visible: true,
            local_draft_continuity_preserved: true,
        },
        ActingIdentityVocabularyRow {
            identity_class: ActingIdentityClass::InstallationGrant,
            effective_scope_class: EffectiveScopeClass::LimitedCommentLink,
            summary: "Installation or app grant may update bounded comment or link fields without implying full human-equivalent provider authority".to_owned(),
            publish_now_allowed: true,
            browser_handoff_required: false,
            denied_scope_visible: true,
            local_draft_continuity_preserved: true,
        },
        ActingIdentityVocabularyRow {
            identity_class: ActingIdentityClass::DelegatedCredential,
            effective_scope_class: EffectiveScopeClass::ProviderMutation,
            summary: "Delegated credential may publish while its source, expiry, and revocation path remain explicit and current".to_owned(),
            publish_now_allowed: true,
            browser_handoff_required: false,
            denied_scope_visible: true,
            local_draft_continuity_preserved: true,
        },
        ActingIdentityVocabularyRow {
            identity_class: ActingIdentityClass::BrowserOnlyFallback,
            effective_scope_class: EffectiveScopeClass::BrowserOnlyFallback,
            summary: "Policy, host, or provider capability may narrow the action to a typed browser handoff while preserving local context and return anchors".to_owned(),
            publish_now_allowed: false,
            browser_handoff_required: true,
            denied_scope_visible: true,
            local_draft_continuity_preserved: true,
        },
        ActingIdentityVocabularyRow {
            identity_class: ActingIdentityClass::DeniedScope,
            effective_scope_class: EffectiveScopeClass::DeniedScope,
            summary: "Denied scope keeps the target object, requested action, and policy reason visible while blocking in-product mutation".to_owned(),
            publish_now_allowed: false,
            browser_handoff_required: false,
            denied_scope_visible: true,
            local_draft_continuity_preserved: true,
        },
        ActingIdentityVocabularyRow {
            identity_class: ActingIdentityClass::PublishLaterLocalDraft,
            effective_scope_class: EffectiveScopeClass::PublishLaterLocalDraft,
            summary: "Publish-later local draft preserves the requested mutation, evidence, and actor lineage without claiming live provider authority".to_owned(),
            publish_now_allowed: false,
            browser_handoff_required: false,
            denied_scope_visible: true,
            local_draft_continuity_preserved: true,
        },
    ]
}

fn canonical_truth_state_rows() -> Vec<ProviderTruthStateVocabularyRow> {
    vec![
        ProviderTruthStateVocabularyRow {
            truth_state_class: ProviderTruthStateClass::LocalDraft,
            summary: "Local draft exists only inside Aureline until publish now, publish later, or handoff explicitly occurs".to_owned(),
            provider_authoritative: false,
            local_continuity_preserved: true,
            review_before_replay_required: false,
            lineage_visible: true,
        },
        ProviderTruthStateVocabularyRow {
            truth_state_class: ProviderTruthStateClass::QueuedPublish,
            summary: "Queued publish is a deferred intent that keeps original preview, actor, target, and validation state for later review or replay".to_owned(),
            provider_authoritative: false,
            local_continuity_preserved: true,
            review_before_replay_required: true,
            lineage_visible: true,
        },
        ProviderTruthStateVocabularyRow {
            truth_state_class: ProviderTruthStateClass::ProviderCommitted,
            summary: "Provider-committed state is confirmed through a current authority path and remains visibly separate from local drafts and imported snapshots".to_owned(),
            provider_authoritative: true,
            local_continuity_preserved: true,
            review_before_replay_required: false,
            lineage_visible: true,
        },
        ProviderTruthStateVocabularyRow {
            truth_state_class: ProviderTruthStateClass::StaleSnapshot,
            summary: "Stale snapshot remains inspectable as stale and may not claim the freshness or authority of a current provider read".to_owned(),
            provider_authoritative: false,
            local_continuity_preserved: true,
            review_before_replay_required: true,
            lineage_visible: true,
        },
        ProviderTruthStateVocabularyRow {
            truth_state_class: ProviderTruthStateClass::PartialScope,
            summary: "Partial-scope state names the missing authority or host mismatch rather than implying full provider coverage".to_owned(),
            provider_authoritative: false,
            local_continuity_preserved: true,
            review_before_replay_required: true,
            lineage_visible: true,
        },
        ProviderTruthStateVocabularyRow {
            truth_state_class: ProviderTruthStateClass::MirrorDerived,
            summary: "Mirror-derived state preserves import lineage and never silently upgrades itself to live-provider truth".to_owned(),
            provider_authoritative: false,
            local_continuity_preserved: true,
            review_before_replay_required: true,
            lineage_visible: true,
        },
        ProviderTruthStateVocabularyRow {
            truth_state_class: ProviderTruthStateClass::CallbackDenied,
            summary: "Callback-denied state records why an inbound event was rejected and preserves auditability without mutating local truth".to_owned(),
            provider_authoritative: false,
            local_continuity_preserved: true,
            review_before_replay_required: false,
            lineage_visible: true,
        },
    ]
}

fn canonical_lane_rows() -> Vec<M5ProviderWorkItemGovernanceLaneRow> {
    vec![
        M5ProviderWorkItemGovernanceLaneRow {
            lane: M5ProviderWorkItemGovernanceLane::WorkItemObjectVocabulary,
            qualification: M5ProviderWorkItemGovernanceQualificationClass::Stable,
            scope_summary: "Canonical object vocabulary keeps provider-backed work items, change intent, handoff packets, deferred publish packets, imported snapshots, and event envelopes governed as typed engineering objects instead of shallow links".to_owned(),
            evidence_requirement: M5ProviderWorkItemGovernanceEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:provider-object-vocabulary-freeze:m5".to_owned(),
                "evidence:work-item-state-separation:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ProviderWorkItemGovernanceDowngradeTrigger::ProviderAuthorityStale,
                M5ProviderWorkItemGovernanceDowngradeTrigger::ImportedSnapshotStale,
                M5ProviderWorkItemGovernanceDowngradeTrigger::PublishLaterContinuityStale,
                M5ProviderWorkItemGovernanceDowngradeTrigger::CallbackReconciliationStale,
                M5ProviderWorkItemGovernanceDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ProviderWorkItemGovernanceRollbackPosture::ProviderCommittedDistinct,
            source_contract_refs: vec![
                M5_PROVIDER_WORKITEM_PROVIDER_LINK_CONTRACT_REF.to_owned(),
                M5_PROVIDER_WORKITEM_TRANSITION_CONTRACT_REF.to_owned(),
                M5_PROVIDER_WORKITEM_LINKAGE_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ProviderWorkItemGovernanceConsumerSurface::ReviewWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::WorkItemDetail,
                M5ProviderWorkItemGovernanceConsumerSurface::SupportExport,
                M5ProviderWorkItemGovernanceConsumerSurface::DocsHelp,
                M5ProviderWorkItemGovernanceConsumerSurface::ReleasePacket,
            ],
        },
        M5ProviderWorkItemGovernanceLaneRow {
            lane: M5ProviderWorkItemGovernanceLane::ProviderLinkedMutation,
            qualification: M5ProviderWorkItemGovernanceQualificationClass::Stable,
            scope_summary: "Provider-linked mutation keeps preview hash, target object, acting identity, effective scope, and downgrade behavior explicit before any review, status, issue, or work-item write".to_owned(),
            evidence_requirement: M5ProviderWorkItemGovernanceEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:provider-mutation-preview-and-attribution:m5".to_owned(),
                "evidence:scope-narrowing-visible:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ProviderWorkItemGovernanceDowngradeTrigger::ProviderAuthorityStale,
                M5ProviderWorkItemGovernanceDowngradeTrigger::EffectiveScopeDrift,
                M5ProviderWorkItemGovernanceDowngradeTrigger::BrowserOnlyFallback,
                M5ProviderWorkItemGovernanceDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ProviderWorkItemGovernanceRollbackPosture::LocalDraftPreserved,
            source_contract_refs: vec![
                M5_PROVIDER_WORKITEM_PROVIDER_LINK_CONTRACT_REF.to_owned(),
                M5_PROVIDER_WORKITEM_TRANSITION_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ProviderWorkItemGovernanceConsumerSurface::ReviewWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::WorkItemDetail,
                M5ProviderWorkItemGovernanceConsumerSurface::CliHeadless,
                M5ProviderWorkItemGovernanceConsumerSurface::SupportExport,
            ],
        },
        M5ProviderWorkItemGovernanceLaneRow {
            lane: M5ProviderWorkItemGovernanceLane::ActingIdentityAndEffectiveScope,
            qualification: M5ProviderWorkItemGovernanceQualificationClass::Stable,
            scope_summary: "Acting identity and effective scope remain first-class on every claimed provider lane so human account, installation grant, delegated credential, browser-only fallback, denied scope, and publish-later local draft never collapse into one optimistic connected state".to_owned(),
            evidence_requirement: M5ProviderWorkItemGovernanceEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:acting-identity-visible:m5".to_owned(),
                "evidence:effective-scope-resolution-visible:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ProviderWorkItemGovernanceDowngradeTrigger::ProviderAuthorityStale,
                M5ProviderWorkItemGovernanceDowngradeTrigger::EffectiveScopeDrift,
                M5ProviderWorkItemGovernanceDowngradeTrigger::BrowserOnlyFallback,
                M5ProviderWorkItemGovernanceDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ProviderWorkItemGovernanceRollbackPosture::LocalDraftPreserved,
            source_contract_refs: vec![
                M5_PROVIDER_WORKITEM_PROVIDER_LINK_CONTRACT_REF.to_owned(),
                M5_PROVIDER_WORKITEM_TRANSITION_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ProviderWorkItemGovernanceConsumerSurface::ReviewWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::WorkItemDetail,
                M5ProviderWorkItemGovernanceConsumerSurface::IncidentWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::SupportExport,
                M5ProviderWorkItemGovernanceConsumerSurface::DocsHelp,
            ],
        },
        M5ProviderWorkItemGovernanceLaneRow {
            lane: M5ProviderWorkItemGovernanceLane::BrowserHandoffContinuity,
            qualification: M5ProviderWorkItemGovernanceQualificationClass::Stable,
            scope_summary: "Typed browser handoff preserves reason code, destination class, acting identity, privacy consequence, and return anchor whenever the product must leave in-product provider scope".to_owned(),
            evidence_requirement: M5ProviderWorkItemGovernanceEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:typed-browser-handoff-return-anchor:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ProviderWorkItemGovernanceDowngradeTrigger::BrowserOnlyFallback,
                M5ProviderWorkItemGovernanceDowngradeTrigger::ReturnAnchorUnproven,
                M5ProviderWorkItemGovernanceDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ProviderWorkItemGovernanceRollbackPosture::ReturnAnchorPreserved,
            source_contract_refs: vec![M5_PROVIDER_WORKITEM_HANDOFF_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![
                M5ProviderWorkItemGovernanceConsumerSurface::BrowserHandoff,
                M5ProviderWorkItemGovernanceConsumerSurface::ReviewWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::CompanionTriage,
                M5ProviderWorkItemGovernanceConsumerSurface::SupportExport,
            ],
        },
        M5ProviderWorkItemGovernanceLaneRow {
            lane: M5ProviderWorkItemGovernanceLane::DeferredPublishContinuity,
            qualification: M5ProviderWorkItemGovernanceQualificationClass::Stable,
            scope_summary: "Deferred publish continuity keeps local draft, queued publish, retry, cancel, and export-safe packet semantics explicit across restart, reconnect, maintenance windows, and support handoff".to_owned(),
            evidence_requirement: M5ProviderWorkItemGovernanceEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:deferred-publish-survives-restart:m5".to_owned(),
                "evidence:deferred-publish-survives-reconnect:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ProviderWorkItemGovernanceDowngradeTrigger::PublishLaterContinuityStale,
                M5ProviderWorkItemGovernanceDowngradeTrigger::EffectiveScopeDrift,
                M5ProviderWorkItemGovernanceDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ProviderWorkItemGovernanceRollbackPosture::AttributablePublishLater,
            source_contract_refs: vec![
                M5_PROVIDER_WORKITEM_LINKAGE_CONTRACT_REF.to_owned(),
                M5_PROVIDER_WORKITEM_PUBLISH_LATER_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ProviderWorkItemGovernanceConsumerSurface::WorkItemDetail,
                M5ProviderWorkItemGovernanceConsumerSurface::ReviewWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::IncidentWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::SupportExport,
                M5ProviderWorkItemGovernanceConsumerSurface::ReleasePacket,
            ],
        },
        M5ProviderWorkItemGovernanceLaneRow {
            lane: M5ProviderWorkItemGovernanceLane::ProviderEventReconciliation,
            qualification: M5ProviderWorkItemGovernanceQualificationClass::Stable,
            scope_summary: "Provider event reconciliation uses one canonical event-ingestion packet with typed import sessions, event envelopes, dedupe windows, replay ledgers, deny or audit events, and linked-object freshness vocabulary so external callbacks never mutate visible state invisibly".to_owned(),
            evidence_requirement: M5ProviderWorkItemGovernanceEvidenceRequirement::Required,
            required_evidence_packet_refs: vec![
                "evidence:provider-event-envelope-and-dedupe:m5".to_owned(),
                "evidence:callback-deny-audit-visible:m5".to_owned(),
                "evidence:linked-object-freshness-vocabulary-stable:m5".to_owned(),
            ],
            downgrade_triggers: vec![
                M5ProviderWorkItemGovernanceDowngradeTrigger::CallbackReconciliationStale,
                M5ProviderWorkItemGovernanceDowngradeTrigger::ReplayLedgerUnavailable,
                M5ProviderWorkItemGovernanceDowngradeTrigger::CallbackDenied,
                M5ProviderWorkItemGovernanceDowngradeTrigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: M5ProviderWorkItemGovernanceRollbackPosture::AuditOnlyNoMutation,
            source_contract_refs: vec![
                M5_PROVIDER_WORKITEM_EVENT_INGESTION_CONTRACT_REF.to_owned(),
                M5_PROVIDER_WORKITEM_EVENT_INGESTION_DOC_REF.to_owned(),
                M5_PROVIDER_WORKITEM_INCIDENT_CONTRACT_REF.to_owned(),
            ],
            consumer_surfaces: vec![
                M5ProviderWorkItemGovernanceConsumerSurface::WorkItemDetail,
                M5ProviderWorkItemGovernanceConsumerSurface::ReviewWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::IncidentWorkspace,
                M5ProviderWorkItemGovernanceConsumerSurface::DocsHelp,
                M5ProviderWorkItemGovernanceConsumerSurface::SupportExport,
                M5ProviderWorkItemGovernanceConsumerSurface::ReleasePacket,
            ],
        },
    ]
}

fn canonical_trust_review() -> M5ProviderWorkItemGovernanceTrustReview {
    M5ProviderWorkItemGovernanceTrustReview {
        provider_objects_never_masquerade_as_local_truth: true,
        publish_mode_explicit: true,
        acting_identity_visible: true,
        effective_scope_visible: true,
        local_and_provider_state_distinct: true,
        imported_snapshot_never_claims_provider_commit: true,
        browser_handoff_return_anchor_safe: true,
        deferred_publish_survives_restart: true,
        deferred_publish_survives_reconnect: true,
        callback_events_deduplicated: true,
        callback_denials_auditable: true,
        downgrade_narrows_instead_of_hides: true,
    }
}

fn canonical_proof_freshness() -> M5ProviderWorkItemGovernanceProofFreshness {
    M5ProviderWorkItemGovernanceProofFreshness {
        proof_freshness_slo_hours: 168,
        last_proof_refresh: "2026-06-12T00:00:00Z".to_owned(),
        auto_narrow_on_authority_stale: true,
        auto_narrow_on_reconciliation_stale: true,
        auto_narrow_on_publish_later_stale: true,
    }
}

fn validate_source_contracts(
    packet: &M5ProviderWorkItemGovernancePacket,
    violations: &mut Vec<M5ProviderWorkItemGovernanceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_REF,
        M5_PROVIDER_WORKITEM_GOVERNANCE_DOC_REF,
        M5_PROVIDER_WORKITEM_PROVIDER_LINK_CONTRACT_REF,
        M5_PROVIDER_WORKITEM_TRANSITION_CONTRACT_REF,
        M5_PROVIDER_WORKITEM_LINKAGE_CONTRACT_REF,
        M5_PROVIDER_WORKITEM_HANDOFF_CONTRACT_REF,
        M5_PROVIDER_WORKITEM_PUBLISH_LATER_CONTRACT_REF,
        M5_PROVIDER_WORKITEM_INCIDENT_CONTRACT_REF,
        M5_PROVIDER_WORKITEM_COMPANION_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ProviderWorkItemGovernanceViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_object_rows(
    packet: &M5ProviderWorkItemGovernancePacket,
    violations: &mut Vec<M5ProviderWorkItemGovernanceViolation>,
) {
    let present: BTreeSet<ProviderWorkItemObjectClass> = packet
        .object_vocabulary_rows
        .iter()
        .map(|row| row.object_class)
        .collect();
    for required in ProviderWorkItemObjectClass::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderWorkItemGovernanceViolation::RequiredObjectClassMissing);
            return;
        }
    }

    for row in &packet.object_vocabulary_rows {
        if row.summary.trim().is_empty()
            || row.state_classes.is_empty()
            || !row.local_anchor_required
        {
            violations.push(M5ProviderWorkItemGovernanceViolation::ObjectVocabularyIncomplete);
        }
        if row.object_class != ProviderWorkItemObjectClass::ImportedSnapshot
            && (!row.provider_authority_visible
                || !row.acting_identity_visible
                || !row.effective_scope_visible)
        {
            violations.push(M5ProviderWorkItemGovernanceViolation::ObjectVocabularyIncomplete);
        }
    }
}

fn validate_identity_rows(
    packet: &M5ProviderWorkItemGovernancePacket,
    violations: &mut Vec<M5ProviderWorkItemGovernanceViolation>,
) {
    let present: BTreeSet<ActingIdentityClass> = packet
        .acting_identity_rows
        .iter()
        .map(|row| row.identity_class)
        .collect();
    for required in ActingIdentityClass::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderWorkItemGovernanceViolation::RequiredActingIdentityMissing);
            return;
        }
    }

    for row in &packet.acting_identity_rows {
        if row.summary.trim().is_empty()
            || (!row.publish_now_allowed
                && !row.browser_handoff_required
                && !row.local_draft_continuity_preserved)
        {
            violations
                .push(M5ProviderWorkItemGovernanceViolation::ActingIdentityVocabularyIncomplete);
        }
        if row.identity_class == ActingIdentityClass::BrowserOnlyFallback
            && !row.browser_handoff_required
        {
            violations
                .push(M5ProviderWorkItemGovernanceViolation::ActingIdentityVocabularyIncomplete);
        }
        if row.identity_class == ActingIdentityClass::DeniedScope && !row.denied_scope_visible {
            violations
                .push(M5ProviderWorkItemGovernanceViolation::ActingIdentityVocabularyIncomplete);
        }
        if row.identity_class == ActingIdentityClass::PublishLaterLocalDraft
            && !row.local_draft_continuity_preserved
        {
            violations
                .push(M5ProviderWorkItemGovernanceViolation::ActingIdentityVocabularyIncomplete);
        }
    }
}

fn validate_truth_state_rows(
    packet: &M5ProviderWorkItemGovernancePacket,
    violations: &mut Vec<M5ProviderWorkItemGovernanceViolation>,
) {
    let present: BTreeSet<ProviderTruthStateClass> = packet
        .truth_state_rows
        .iter()
        .map(|row| row.truth_state_class)
        .collect();
    for required in ProviderTruthStateClass::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderWorkItemGovernanceViolation::RequiredTruthStateMissing);
            return;
        }
    }

    for row in &packet.truth_state_rows {
        if row.summary.trim().is_empty() || !row.lineage_visible {
            violations.push(M5ProviderWorkItemGovernanceViolation::TruthStateVocabularyIncomplete);
        }
        if row.truth_state_class == ProviderTruthStateClass::ProviderCommitted
            && !row.provider_authoritative
        {
            violations.push(M5ProviderWorkItemGovernanceViolation::TruthStateVocabularyIncomplete);
        }
        if row.truth_state_class == ProviderTruthStateClass::QueuedPublish
            && !row.review_before_replay_required
        {
            violations.push(M5ProviderWorkItemGovernanceViolation::TruthStateVocabularyIncomplete);
        }
    }
}

fn validate_lane_rows(
    packet: &M5ProviderWorkItemGovernancePacket,
    violations: &mut Vec<M5ProviderWorkItemGovernanceViolation>,
) {
    let present: BTreeSet<M5ProviderWorkItemGovernanceLane> =
        packet.lane_rows.iter().map(|row| row.lane).collect();
    for required in M5ProviderWorkItemGovernanceLane::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderWorkItemGovernanceViolation::RequiredLaneMissing);
            return;
        }
    }

    for row in &packet.lane_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5ProviderWorkItemGovernanceViolation::LaneRowIncomplete);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5ProviderWorkItemGovernanceViolation::StableLaneMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ProviderWorkItemGovernanceViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ProviderWorkItemGovernanceViolation::ConsumerSurfacesMissing);
        }
    }
}

fn validate_trust_review(
    packet: &M5ProviderWorkItemGovernancePacket,
    violations: &mut Vec<M5ProviderWorkItemGovernanceViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.provider_objects_never_masquerade_as_local_truth,
        review.publish_mode_explicit,
        review.acting_identity_visible,
        review.effective_scope_visible,
        review.local_and_provider_state_distinct,
        review.imported_snapshot_never_claims_provider_commit,
        review.browser_handoff_return_anchor_safe,
        review.deferred_publish_survives_restart,
        review.deferred_publish_survives_reconnect,
        review.callback_events_deduplicated,
        review.callback_denials_auditable,
        review.downgrade_narrows_instead_of_hides,
    ] {
        if !ok {
            violations.push(M5ProviderWorkItemGovernanceViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ProviderWorkItemGovernancePacket,
    violations: &mut Vec<M5ProviderWorkItemGovernanceViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_workspace_reuses_canonical_vocabulary,
        projection.work_item_detail_shows_identity_scope_and_state,
        projection.incident_workspace_preserves_provider_lineage,
        projection.companion_triage_shows_scoped_draft_or_handoff_truth,
        projection.browser_handoff_shows_reason_and_return_anchor,
        projection.cli_headless_shows_qualification,
        projection.support_export_shows_provider_authority_and_state,
        projection.docs_help_shows_publish_mode_terminology,
        projection.release_packet_shows_qualification_and_narrowing,
    ] {
        if !ok {
            violations.push(M5ProviderWorkItemGovernanceViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ProviderWorkItemGovernancePacket,
    violations: &mut Vec<M5ProviderWorkItemGovernanceViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
        || !packet.proof_freshness.auto_narrow_on_authority_stale
        || !packet.proof_freshness.auto_narrow_on_reconciliation_stale
        || !packet.proof_freshness.auto_narrow_on_publish_later_stale
    {
        violations.push(M5ProviderWorkItemGovernanceViolation::ProofFreshnessIncomplete);
    }
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
                || lower.contains("token body")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
