//! Certification of provider-backed work-item, acting-identity, publish-later,
//! and reconciliation depth across all claimed M5 team-workflow rows.
//!
//! This module is the canonical certification layer over the claimed M5
//! provider-backed team-workflow rows frozen in the
//! [`freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix`](crate::freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix)
//! packet. The matrix freezes six row claims at lane granularity; this packet
//! certifies those same rows against current proof, binds each row to its
//! evidence packet refs, downgrade triggers, rollback posture, provider-family
//! compatibility story, acting-identity story, offline or publish-later story,
//! and reconciliation proof, and publishes one export-safe result that release,
//! help/about, service-health, and support surfaces can all ingest.
//!
//! [`certify_from_current_exports`] is the first consumer. It validates the
//! checked team-workflow governance export, work-item mutation review export,
//! browser-handoff continuity export, deferred-publish recovery export, and the
//! checked provider-scope and provider-event-ingestion fixtures. Missing or
//! invalid evidence blocks the affected row; stale proof or a narrowed upstream
//! downgrades it to `narrowed_certified` instead of leaving the broader M5
//! claim greener than the current evidence.
//!
//! The boundary schema is
//! [`schemas/review/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.schema.json`](../../../../schemas/review/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.schema.json).
//! The contract doc is
//! [`docs/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.md`](../../../../docs/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/`](../../../../fixtures/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use aureline_provider::{
    validate_provider_event_ingestion_packet, validate_provider_scope_review_page,
    ProviderEventIngestionPacket, ProviderScopeReviewPage, PROVIDER_EVENT_INGESTION_DOC_REF,
    PROVIDER_EVENT_INGESTION_PACKET_RECORD_KIND, PROVIDER_EVENT_INGESTION_SCHEMA_REF,
    PROVIDER_EVENT_INGESTION_SUPPORT_EXPORT_ARTIFACT_REF, PROVIDER_SCOPE_REVIEW_PAGE_RECORD_KIND,
    PROVIDER_SCOPE_REVIEW_SCHEMA_REF, PROVIDER_SCOPE_REVIEW_SUPPORT_EXPORT_ARTIFACT_REF,
};
use serde::{Deserialize, Serialize};

const PROVIDER_SCOPE_REVIEW_DOC_REF: &str = "docs/providers/m5/provider_scope_review.md";

/// Stable record-kind tag carried by [`M5TeamWorkflowCertificationPacket`].
pub const M5_TEAM_WORKFLOW_CERTIFICATION_RECORD_KIND: &str =
    "certify_work_item_provider_linked_mutation_acting_identity_and_publish_later_or_reconciliation_depth_on_all_claimed_m5_team_workflow_rows";

/// Schema version for M5 team-workflow certification records.
pub const M5_TEAM_WORKFLOW_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_TEAM_WORKFLOW_CERTIFICATION_SCHEMA_REF: &str =
    "schemas/review/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.schema.json";

/// Repo-relative path of the M5 team-workflow certification contract doc.
pub const M5_TEAM_WORKFLOW_CERTIFICATION_DOC_REF: &str =
    "docs/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.md";

/// Repo-relative path of the frozen governance-matrix authority this certification builds on.
pub const M5_TEAM_WORKFLOW_CERTIFICATION_MATRIX_SCHEMA_REF: &str =
    crate::freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix::M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_REF;

/// Repo-relative path of the frozen governance contract doc this certification builds on.
pub const M5_TEAM_WORKFLOW_CERTIFICATION_MATRIX_DOC_REF: &str =
    crate::freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix::M5_PROVIDER_WORKITEM_GOVERNANCE_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_TEAM_WORKFLOW_CERTIFICATION_FIXTURE_DIR: &str =
    "fixtures/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TEAM_WORKFLOW_CERTIFICATION_ARTIFACT_REF: &str =
    "artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_TEAM_WORKFLOW_CERTIFICATION_SUMMARY_REF: &str =
    "artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows.md";

/// One claimed M5 team-workflow row certified by this packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeamWorkflowClaimedRow {
    /// Canonical provider-backed object vocabulary and truth separation.
    WorkItemObjectVocabulary,
    /// Provider-linked mutation review with explicit publish modes.
    ProviderLinkedMutation,
    /// Acting identity and effective-scope visibility.
    ActingIdentityAndEffectiveScope,
    /// Typed browser handoff continuity with return anchors.
    BrowserHandoffContinuity,
    /// Deferred publish continuity across restart, reconnect, and support handoff.
    DeferredPublishContinuity,
    /// External-event ingestion and reconciliation truth.
    ProviderEventReconciliation,
}

impl M5TeamWorkflowClaimedRow {
    /// Every claimed row, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkItemObjectVocabulary,
        Self::ProviderLinkedMutation,
        Self::ActingIdentityAndEffectiveScope,
        Self::BrowserHandoffContinuity,
        Self::DeferredPublishContinuity,
        Self::ProviderEventReconciliation,
    ];

    /// Stable token recorded in the certification.
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

/// Qualification class claimed by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeamWorkflowQualificationClass {
    /// Row claims the Stable maturity.
    Stable,
    /// Row claims the Beta maturity.
    Beta,
    /// Row claims the Preview maturity.
    Preview,
    /// Row is limited to a narrower marketing or provider-specific posture.
    Limited,
    /// Row is experimental and not broadly claimed.
    Experimental,
    /// Row is not marketed on this build.
    NotMarketed,
}

impl M5TeamWorkflowQualificationClass {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Limited => "limited",
            Self::Experimental => "experimental",
            Self::NotMarketed => "not_marketed",
        }
    }
}

/// Certification verdict earned by a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeamWorkflowCertificationVerdict {
    /// Row is certified at its claimed qualification with current, valid evidence.
    Certified,
    /// Row is certified, but explicitly narrower than the broadest team-workflow claim.
    NarrowedCertified,
    /// Row is blocked from broader claims until its evidence or dependency recovers.
    Blocked,
    /// Row could not be certified at all.
    NotCertified,
}

impl M5TeamWorkflowCertificationVerdict {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::NarrowedCertified => "narrowed_certified",
            Self::Blocked => "blocked",
            Self::NotCertified => "not_certified",
        }
    }

    /// Whether the verdict still permits a public claim (possibly narrowed).
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Certified | Self::NarrowedCertified)
    }
}

/// Downgrade trigger that can narrow a certified row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeamWorkflowCertificationDowngradeTrigger {
    /// Proof packet has gone stale relative to its freshness SLO.
    ProofStale,
    /// Evidence packet failed validation or is missing.
    EvidencePacketInvalid,
    /// Provider authority or identity proof has gone stale.
    ProviderAuthorityStale,
    /// Effective scope proof drifted or expired.
    EffectiveScopeDrift,
    /// Browser-only fallback narrowed a previously broader claim.
    BrowserOnlyFallback,
    /// Publish-later continuity proof has gone stale.
    PublishLaterContinuityStale,
    /// Callback or reconciliation proof has gone stale.
    CallbackReconciliationStale,
    /// Return anchor or handoff continuity is no longer proven.
    ReturnAnchorUnproven,
    /// Replay ledger or typed reconciliation path is unavailable.
    ReplayLedgerUnavailable,
    /// An upstream dependency row narrowed.
    UpstreamDependencyNarrowed,
}

impl M5TeamWorkflowCertificationDowngradeTrigger {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::EvidencePacketInvalid => "evidence_packet_invalid",
            Self::ProviderAuthorityStale => "provider_authority_stale",
            Self::EffectiveScopeDrift => "effective_scope_drift",
            Self::BrowserOnlyFallback => "browser_only_fallback",
            Self::PublishLaterContinuityStale => "publish_later_continuity_stale",
            Self::CallbackReconciliationStale => "callback_reconciliation_stale",
            Self::ReturnAnchorUnproven => "return_anchor_unproven",
            Self::ReplayLedgerUnavailable => "replay_ledger_unavailable",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Rollback posture for a certified row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeamWorkflowCertificationRollbackPosture {
    /// Provider and local truth remain explicitly separated with local continuity preserved.
    ProviderTruthSeparationPreserved,
    /// Local draft survives failure and can replay later under review.
    LocalDraftPreserved,
    /// Effective scope must be recomputed before retry.
    RecomputeScopeBeforeRetry,
    /// Browser handoff preserves return anchor and typed packet lineage.
    ReturnAnchorPreserved,
    /// Deferred publish queue preserves packet lineage for retry, export, or discard.
    DeferredPublishPacketPreserved,
    /// Reconciliation keeps audit-only evidence even when provider mutation is denied.
    AuditOnlyEvidencePreserved,
}

impl M5TeamWorkflowCertificationRollbackPosture {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderTruthSeparationPreserved => "provider_truth_separation_preserved",
            Self::LocalDraftPreserved => "local_draft_preserved",
            Self::RecomputeScopeBeforeRetry => "recompute_scope_before_retry",
            Self::ReturnAnchorPreserved => "return_anchor_preserved",
            Self::DeferredPublishPacketPreserved => "deferred_publish_packet_preserved",
            Self::AuditOnlyEvidencePreserved => "audit_only_evidence_preserved",
        }
    }
}

/// Provider family carried by compatibility stories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeamWorkflowProviderFamily {
    /// Issue, task, or incident tracker family.
    IssueTracker,
    /// Code-host or pull-request family.
    CodeHost,
    /// CI or check provider family.
    CiChecks,
}

impl M5TeamWorkflowProviderFamily {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IssueTracker => "issue_tracker",
            Self::CodeHost => "code_host",
            Self::CiChecks => "ci_checks",
        }
    }
}

/// Marketing or compatibility posture for one provider family on one row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeamWorkflowProviderFamilyPosture {
    /// Provider family is qualified at the row's current claim level.
    Qualified,
    /// Provider family is supported only in a narrower or provider-specific way.
    Limited,
    /// Provider family must not inherit the row's broader marketing claim.
    NotMarketed,
}

impl M5TeamWorkflowProviderFamilyPosture {
    /// Stable token recorded in the certification.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::Limited => "limited",
            Self::NotMarketed => "not_marketed",
        }
    }
}

/// Per-provider-family compatibility entry for a certified row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowProviderFamilyCompatibility {
    /// Provider family the compatibility story applies to.
    pub provider_family: M5TeamWorkflowProviderFamily,
    /// Marketing or compatibility posture for the family on this row.
    pub posture: M5TeamWorkflowProviderFamilyPosture,
    /// Human-readable explanation of the family-specific posture.
    pub rationale: String,
}

/// Acting-identity posture carried by one certified row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowActingIdentityStory {
    /// Primary proof packet or support export that backs this acting-identity story.
    pub proof_ref: String,
    /// Human-account identity is visibly distinct.
    pub human_account_visible: bool,
    /// Installation grant identity is visibly distinct.
    pub installation_grant_visible: bool,
    /// Delegated credential identity is visibly distinct.
    pub delegated_credential_visible: bool,
    /// Browser-only fallback remains visibly distinct.
    pub browser_only_fallback_visible: bool,
    /// Denied scope remains visibly distinct.
    pub denied_scope_visible: bool,
    /// Human-readable explanation of the acting-identity posture.
    pub rationale: String,
}

/// Offline, browser-handoff, or publish-later continuity story carried by one row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowOfflinePublishLaterStory {
    /// Primary proof packet or support export that backs this continuity story.
    pub proof_ref: String,
    /// Local draft survives provider outage or policy block.
    pub local_draft_preserved: bool,
    /// Queued publish survives restart or reconnect.
    pub queued_publish_preserved: bool,
    /// Export-safe packet lineage remains available for support handoff.
    pub export_safe_packet_preserved: bool,
    /// Browser handoff preserves a typed return anchor when applicable.
    pub return_anchor_preserved: bool,
    /// Replay requires a fresh scope or target review when drift occurs.
    pub replay_requires_review_after_drift: bool,
    /// Human-readable explanation of the continuity posture.
    pub rationale: String,
}

/// Reconciliation story carried by one certified row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowReconciliationStory {
    /// Primary proof packet or support export that backs this reconciliation story.
    pub proof_ref: String,
    /// Typed event envelopes remain visible.
    pub typed_event_envelopes_visible: bool,
    /// Replay-ledger lineage remains visible.
    pub replay_ledger_visible: bool,
    /// Callback deny events remain auditable.
    pub callback_denials_auditable: bool,
    /// Dedupe windows and replay decisions remain visible.
    pub dedupe_visible: bool,
    /// Provider mutation continues only through typed reconciliation paths.
    pub typed_reconciliation_required_for_mutation: bool,
    /// Human-readable explanation of the reconciliation posture.
    pub rationale: String,
}

/// Per-row proof-freshness observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowCertificationRowFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the row's last proof refresh.
    pub last_proof_refresh: String,
    /// True when the row's proof is currently within its freshness SLO.
    pub proof_fresh: bool,
}

/// One certified claimed M5 team-workflow row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowCertifiedRow {
    /// Claimed M5 row.
    pub row: M5TeamWorkflowClaimedRow,
    /// Qualification class claimed by the row.
    pub claimed_qualification: M5TeamWorkflowQualificationClass,
    /// Certification verdict.
    pub verdict: M5TeamWorkflowCertificationVerdict,
    /// Upstream packet record kind backing the row.
    pub upstream_record_kind: String,
    /// Support-export or proof artifact ref backing the row.
    pub evidence_artifact_ref: String,
    /// Schema ref backing the row.
    pub evidence_schema_ref: String,
    /// Contract doc ref backing the row.
    pub evidence_doc_ref: String,
    /// Current scorecard summary for the row.
    pub feature_scorecard_summary: String,
    /// Provider-family compatibility story for the row.
    pub provider_family_compatibility: Vec<M5TeamWorkflowProviderFamilyCompatibility>,
    /// Acting-identity story for the row.
    pub acting_identity_story: M5TeamWorkflowActingIdentityStory,
    /// Offline or publish-later continuity story for the row.
    pub offline_publish_later_story: M5TeamWorkflowOfflinePublishLaterStory,
    /// Reconciliation story for the row.
    pub reconciliation_story: M5TeamWorkflowReconciliationStory,
    /// Downgrade triggers that can narrow the row.
    pub downgrade_triggers: Vec<M5TeamWorkflowCertificationDowngradeTrigger>,
    /// Rollback posture.
    pub rollback_posture: M5TeamWorkflowCertificationRollbackPosture,
    /// Per-row proof freshness.
    pub proof_freshness: M5TeamWorkflowCertificationRowFreshness,
}

/// Aggregate compatibility report across all certified rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowCertificationCompatibilityReport {
    /// Total certified rows in the packet.
    pub total_rows: u32,
    /// Count of fully certified rows.
    pub certified_count: u32,
    /// Count of narrowed-but-certified rows.
    pub narrowed_count: u32,
    /// Count of blocked rows.
    pub blocked_count: u32,
    /// Count of rows that could not be certified.
    pub not_certified_count: u32,
    /// True when every row is publishable (certified or narrowed).
    pub all_rows_publishable: bool,
    /// Human-readable promotion note.
    pub promotion_note: String,
}

impl M5TeamWorkflowCertificationCompatibilityReport {
    /// Recomputes the compatibility report from a row set.
    pub fn from_rows(rows: &[M5TeamWorkflowCertifiedRow]) -> Self {
        let mut certified = 0u32;
        let mut narrowed = 0u32;
        let mut blocked = 0u32;
        let mut not_certified = 0u32;
        for row in rows {
            match row.verdict {
                M5TeamWorkflowCertificationVerdict::Certified => certified += 1,
                M5TeamWorkflowCertificationVerdict::NarrowedCertified => narrowed += 1,
                M5TeamWorkflowCertificationVerdict::Blocked => blocked += 1,
                M5TeamWorkflowCertificationVerdict::NotCertified => not_certified += 1,
            }
        }
        let all_publishable = blocked == 0 && not_certified == 0;
        let promotion_note = if all_publishable {
            "all claimed M5 provider-backed team-workflow rows are publishable; reconciliation remains explicitly beta-scoped".to_owned()
        } else {
            format!(
                "{} row(s) blocked and {} row(s) uncertified; broad team-workflow claims must narrow",
                blocked, not_certified
            )
        };
        Self {
            total_rows: rows.len() as u32,
            certified_count: certified,
            narrowed_count: narrowed,
            blocked_count: blocked,
            not_certified_count: not_certified,
            all_rows_publishable: all_publishable,
            promotion_note,
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowCertificationTrustReview {
    /// Provider objects never masquerade as canonical local truth.
    pub provider_objects_never_masquerade_as_local_truth: bool,
    /// Local draft, queued publish, provider commit, and stale import remain distinct.
    pub truth_states_remain_visibly_distinct: bool,
    /// Acting identity remains inspectable.
    pub acting_identity_visible: bool,
    /// Effective scope remains inspectable.
    pub effective_scope_visible: bool,
    /// Browser handoff preserves return-anchor continuity.
    pub browser_handoff_return_anchor_safe: bool,
    /// Deferred publish survives restart.
    pub deferred_publish_survives_restart: bool,
    /// Deferred publish survives reconnect.
    pub deferred_publish_survives_reconnect: bool,
    /// Callback and webhook events stay deduplicated.
    pub callback_events_deduplicated: bool,
    /// Callback-denied events remain auditable.
    pub callback_denials_auditable: bool,
    /// Downgrade narrows the claim rather than hiding the row.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block broad marketing.
    pub stale_or_underqualified_blocks_broad_claims: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowCertificationConsumerProjection {
    /// Review workspace shows certification truth.
    pub review_workspace_shows_certification: bool,
    /// Work-item detail shows certification truth.
    pub work_item_detail_shows_certification: bool,
    /// Incident workspace shows certification truth.
    pub incident_workspace_shows_certification: bool,
    /// CLI or headless shows certification truth.
    pub cli_headless_shows_certification: bool,
    /// Support export shows certification truth.
    pub support_export_shows_certification: bool,
    /// Help / About shows certification truth.
    pub help_about_shows_certification: bool,
    /// Service health shows certification truth.
    pub service_health_shows_certification: bool,
    /// Public truth pack shows certification truth.
    pub public_truth_pack_shows_certification: bool,
    /// Release manifests show certification truth.
    pub release_manifests_show_certification: bool,
    /// Provider-family-specific badging remains visible when capability gaps matter.
    pub provider_family_specific_badging_visible: bool,
}

/// Packet-level proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowCertificationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last certification refresh.
    pub last_proof_refresh: String,
    /// True when stale provider authority narrows the certification.
    pub auto_narrow_on_provider_authority_stale: bool,
    /// True when stale publish-later continuity narrows the certification.
    pub auto_narrow_on_publish_later_stale: bool,
    /// True when stale reconciliation proof narrows the certification.
    pub auto_narrow_on_reconciliation_stale: bool,
}

/// Per-row observation fed to [`M5TeamWorkflowCertificationPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TeamWorkflowCertificationRowObservation {
    /// Row the observation applies to.
    pub row: M5TeamWorkflowClaimedRow,
    /// True when the row's checked evidence currently validates.
    pub evidence_valid: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`M5TeamWorkflowCertificationPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TeamWorkflowCertificationPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified rows.
    pub certified_rows: Vec<M5TeamWorkflowCertifiedRow>,
    /// Compatibility report.
    pub compatibility_report: M5TeamWorkflowCertificationCompatibilityReport,
    /// Trust review block.
    pub trust_review: M5TeamWorkflowCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5TeamWorkflowCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TeamWorkflowCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 team-workflow certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeamWorkflowCertificationPacket {
    /// Record kind; must equal [`M5_TEAM_WORKFLOW_CERTIFICATION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TEAM_WORKFLOW_CERTIFICATION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable certification label.
    pub certification_label: String,
    /// Certified rows.
    pub certified_rows: Vec<M5TeamWorkflowCertifiedRow>,
    /// Compatibility report.
    pub compatibility_report: M5TeamWorkflowCertificationCompatibilityReport,
    /// Trust review block.
    pub trust_review: M5TeamWorkflowCertificationTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5TeamWorkflowCertificationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TeamWorkflowCertificationProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TeamWorkflowCertificationPacket {
    /// Builds an M5 team-workflow certification packet from stable-lane input.
    pub fn new(input: M5TeamWorkflowCertificationPacketInput) -> Self {
        Self {
            record_kind: M5_TEAM_WORKFLOW_CERTIFICATION_RECORD_KIND.to_owned(),
            schema_version: M5_TEAM_WORKFLOW_CERTIFICATION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            certification_label: input.certification_label,
            certified_rows: input.certified_rows,
            compatibility_report: input.compatibility_report,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows rows whose evidence is invalid, whose proof is stale, or whose
    /// upstream dependency narrowed, then recomputes the compatibility report.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[M5TeamWorkflowCertificationRowObservation],
    ) {
        for row in &mut self.certified_rows {
            let Some(observation) = observations.iter().find(|obs| obs.row == row.row) else {
                continue;
            };
            row.proof_freshness.proof_fresh = observation.proof_fresh;
            if !observation.evidence_valid {
                row.verdict = M5TeamWorkflowCertificationVerdict::Blocked;
            } else if (!observation.proof_fresh || observation.upstream_narrowed)
                && row.verdict == M5TeamWorkflowCertificationVerdict::Certified
            {
                row.verdict = M5TeamWorkflowCertificationVerdict::NarrowedCertified;
            }
        }
        self.compatibility_report =
            M5TeamWorkflowCertificationCompatibilityReport::from_rows(&self.certified_rows);
    }

    /// Validates the M5 team-workflow certification invariants.
    pub fn validate(&self) -> Vec<M5TeamWorkflowCertificationViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TEAM_WORKFLOW_CERTIFICATION_RECORD_KIND {
            violations.push(M5TeamWorkflowCertificationViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TEAM_WORKFLOW_CERTIFICATION_SCHEMA_VERSION {
            violations.push(M5TeamWorkflowCertificationViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.certification_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TeamWorkflowCertificationViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_compatibility_report(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 team-workflow certification packet serializes"),
        ) {
            violations.push(M5TeamWorkflowCertificationViolation::RawBoundaryMaterialInExport);
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
            .expect("m5 team-workflow certification packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Provider-Backed Team-Workflow Certification\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.certification_label));
        out.push_str(&format!(
            "- Rows: {} ({} certified, {} narrowed, {} blocked, {} uncertified)\n",
            self.compatibility_report.total_rows,
            self.compatibility_report.certified_count,
            self.compatibility_report.narrowed_count,
            self.compatibility_report.blocked_count,
            self.compatibility_report.not_certified_count,
        ));
        out.push_str(&format!(
            "- All rows publishable: {}\n",
            self.compatibility_report.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Promotion: {}\n",
            self.compatibility_report.promotion_note
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.certified_rows {
            let provider_families = row
                .provider_family_compatibility
                .iter()
                .map(|entry| {
                    format!(
                        "{}={}",
                        entry.provider_family.as_str(),
                        entry.posture.as_str()
                    )
                })
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "- **{}**: `{}` (claimed `{}`)\n",
                row.row.as_str(),
                row.verdict.as_str(),
                row.claimed_qualification.as_str(),
            ));
            out.push_str(&format!("  - Evidence: `{}`\n", row.evidence_artifact_ref));
            out.push_str(&format!("  - Providers: {}\n", provider_families));
            out.push_str(&format!(
                "  - Proof fresh: {} (last refresh: {})\n",
                row.proof_freshness.proof_fresh, row.proof_freshness.last_proof_refresh
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 team-workflow certification export.
#[derive(Debug)]
pub enum M5TeamWorkflowCertificationArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TeamWorkflowCertificationViolation>),
}

impl fmt::Display for M5TeamWorkflowCertificationArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 team-workflow certification export parse failed: {error}"
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
                    "m5 team-workflow certification export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TeamWorkflowCertificationArtifactError {}

/// Validation failures emitted by [`M5TeamWorkflowCertificationPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TeamWorkflowCertificationViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required claimed row is missing from the certification.
    RequiredRowMissing,
    /// A certified row is incomplete.
    RowIncomplete,
    /// A current feature-scorecard summary is missing.
    FeatureScorecardMissing,
    /// Provider-family compatibility story is missing or empty.
    ProviderFamilyCompatibilityMissing,
    /// Acting-identity story is incomplete.
    ActingIdentityStoryIncomplete,
    /// Offline or publish-later story is incomplete.
    OfflinePublishLaterStoryIncomplete,
    /// Reconciliation story is incomplete.
    ReconciliationStoryIncomplete,
    /// A publishable row is missing evidence refs.
    PublishableRowMissingEvidence,
    /// A row has no downgrade triggers.
    DowngradeTriggersMissing,
    /// The compatibility report does not agree with the row verdicts.
    CompatibilityReportMismatch,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5TeamWorkflowCertificationViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredRowMissing => "required_row_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::FeatureScorecardMissing => "feature_scorecard_missing",
            Self::ProviderFamilyCompatibilityMissing => "provider_family_compatibility_missing",
            Self::ActingIdentityStoryIncomplete => "acting_identity_story_incomplete",
            Self::OfflinePublishLaterStoryIncomplete => "offline_publish_later_story_incomplete",
            Self::ReconciliationStoryIncomplete => "reconciliation_story_incomplete",
            Self::PublishableRowMissingEvidence => "publishable_row_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::CompatibilityReportMismatch => "compatibility_report_mismatch",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in M5 team-workflow certification export.
pub fn current_m5_team_workflow_certification_export(
) -> Result<M5TeamWorkflowCertificationPacket, M5TeamWorkflowCertificationArtifactError> {
    let packet: M5TeamWorkflowCertificationPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/certify-work-item-provider-linked-mutation-acting-identity-and-publish-later-or-reconciliation-depth-on-all-claimed-m5-team-workflow-rows/support_export.json"
    )))
    .map_err(M5TeamWorkflowCertificationArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TeamWorkflowCertificationArtifactError::Validation(
            violations,
        ))
    }
}

/// First-consumer certification: validates every claimed row's current export
/// or checked proof packet and certifies the row only when its evidence is
/// current and valid.
pub fn certify_from_current_exports(
    packet_id: String,
    certification_label: String,
    minted_at: String,
    proof_freshness: M5TeamWorkflowCertificationProofFreshness,
) -> M5TeamWorkflowCertificationPacket {
    let rows = M5TeamWorkflowClaimedRow::ALL
        .into_iter()
        .map(|row| {
            let descriptor = row_descriptor(row);
            let evidence_valid = descriptor.evidence_valid();
            let verdict = if !evidence_valid {
                M5TeamWorkflowCertificationVerdict::Blocked
            } else {
                descriptor.default_verdict
            };
            M5TeamWorkflowCertifiedRow {
                row,
                claimed_qualification: descriptor.claimed_qualification,
                verdict,
                upstream_record_kind: descriptor.upstream_record_kind.to_owned(),
                evidence_artifact_ref: descriptor.evidence_artifact_ref.to_owned(),
                evidence_schema_ref: descriptor.evidence_schema_ref.to_owned(),
                evidence_doc_ref: descriptor.evidence_doc_ref.to_owned(),
                feature_scorecard_summary: descriptor.feature_scorecard_summary.to_owned(),
                provider_family_compatibility: descriptor.provider_family_compatibility.clone(),
                acting_identity_story: descriptor.acting_identity_story.clone(),
                offline_publish_later_story: descriptor.offline_publish_later_story.clone(),
                reconciliation_story: descriptor.reconciliation_story.clone(),
                downgrade_triggers: descriptor.downgrade_triggers.clone(),
                rollback_posture: descriptor.rollback_posture,
                proof_freshness: M5TeamWorkflowCertificationRowFreshness {
                    proof_freshness_slo_hours: proof_freshness.proof_freshness_slo_hours,
                    last_proof_refresh: proof_freshness.last_proof_refresh.clone(),
                    proof_fresh: true,
                },
            }
        })
        .collect::<Vec<_>>();

    let compatibility_report = M5TeamWorkflowCertificationCompatibilityReport::from_rows(&rows);

    M5TeamWorkflowCertificationPacket::new(M5TeamWorkflowCertificationPacketInput {
        packet_id,
        certification_label,
        certified_rows: rows,
        compatibility_report,
        trust_review: canonical_trust_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical trust review block with every invariant satisfied.
pub fn canonical_trust_review() -> M5TeamWorkflowCertificationTrustReview {
    M5TeamWorkflowCertificationTrustReview {
        provider_objects_never_masquerade_as_local_truth: true,
        truth_states_remain_visibly_distinct: true,
        acting_identity_visible: true,
        effective_scope_visible: true,
        browser_handoff_return_anchor_safe: true,
        deferred_publish_survives_restart: true,
        deferred_publish_survives_reconnect: true,
        callback_events_deduplicated: true,
        callback_denials_auditable: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_broad_claims: true,
    }
}

/// Canonical consumer projection block with every surface projecting certification truth.
pub fn canonical_consumer_projection() -> M5TeamWorkflowCertificationConsumerProjection {
    M5TeamWorkflowCertificationConsumerProjection {
        review_workspace_shows_certification: true,
        work_item_detail_shows_certification: true,
        incident_workspace_shows_certification: true,
        cli_headless_shows_certification: true,
        support_export_shows_certification: true,
        help_about_shows_certification: true,
        service_health_shows_certification: true,
        public_truth_pack_shows_certification: true,
        release_manifests_show_certification: true,
        provider_family_specific_badging_visible: true,
    }
}

/// Canonical source contract refs that every certification export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        M5_TEAM_WORKFLOW_CERTIFICATION_SCHEMA_REF.to_owned(),
        M5_TEAM_WORKFLOW_CERTIFICATION_DOC_REF.to_owned(),
        M5_TEAM_WORKFLOW_CERTIFICATION_MATRIX_SCHEMA_REF.to_owned(),
        M5_TEAM_WORKFLOW_CERTIFICATION_MATRIX_DOC_REF.to_owned(),
        PROVIDER_SCOPE_REVIEW_SCHEMA_REF.to_owned(),
        PROVIDER_EVENT_INGESTION_SCHEMA_REF.to_owned(),
    ]
}

/// Static descriptor binding a claimed row to its evidence refs and stories.
struct RowDescriptor {
    claimed_qualification: M5TeamWorkflowQualificationClass,
    default_verdict: M5TeamWorkflowCertificationVerdict,
    upstream_record_kind: &'static str,
    evidence_artifact_ref: &'static str,
    evidence_schema_ref: &'static str,
    evidence_doc_ref: &'static str,
    feature_scorecard_summary: &'static str,
    provider_family_compatibility: Vec<M5TeamWorkflowProviderFamilyCompatibility>,
    acting_identity_story: M5TeamWorkflowActingIdentityStory,
    offline_publish_later_story: M5TeamWorkflowOfflinePublishLaterStory,
    reconciliation_story: M5TeamWorkflowReconciliationStory,
    downgrade_triggers: Vec<M5TeamWorkflowCertificationDowngradeTrigger>,
    rollback_posture: M5TeamWorkflowCertificationRollbackPosture,
    evidence_probe: fn() -> bool,
}

impl RowDescriptor {
    fn evidence_valid(&self) -> bool {
        (self.evidence_probe)()
    }
}

fn row_descriptor(row: M5TeamWorkflowClaimedRow) -> RowDescriptor {
    use M5TeamWorkflowCertificationDowngradeTrigger as Trigger;
    use M5TeamWorkflowCertificationRollbackPosture as Rollback;
    use M5TeamWorkflowCertificationVerdict as Verdict;
    use M5TeamWorkflowProviderFamily as Family;
    use M5TeamWorkflowProviderFamilyPosture as Posture;
    use M5TeamWorkflowQualificationClass as Qual;

    let scope_story = M5TeamWorkflowActingIdentityStory {
        proof_ref: PROVIDER_SCOPE_REVIEW_SUPPORT_EXPORT_ARTIFACT_REF.to_owned(),
        human_account_visible: true,
        installation_grant_visible: true,
        delegated_credential_visible: true,
        browser_only_fallback_visible: true,
        denied_scope_visible: true,
        rationale: "Provider scope review keeps human account, installation grant, delegated credential, browser-only fallback, and denied scope explicitly distinct before any provider-backed mutation proceeds.".to_owned(),
    };
    let review_queue_story = M5TeamWorkflowOfflinePublishLaterStory {
        proof_ref: crate::ship_deferred_publish_queue_recovery_packets::DEFERRED_PUBLISH_QUEUE_RECOVERY_ARTIFACT_REF.to_owned(),
        local_draft_preserved: true,
        queued_publish_preserved: true,
        export_safe_packet_preserved: true,
        return_anchor_preserved: false,
        replay_requires_review_after_drift: true,
        rationale: "Deferred publish recovery preserves local draft, queued publish, export-safe packet lineage, and replay review when target or scope drift occurs.".to_owned(),
    };
    let handoff_story = M5TeamWorkflowOfflinePublishLaterStory {
        proof_ref: crate::ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links::HANDOFF_CONTINUITY_ARTIFACT_REF.to_owned(),
        local_draft_preserved: true,
        queued_publish_preserved: false,
        export_safe_packet_preserved: true,
        return_anchor_preserved: true,
        replay_requires_review_after_drift: false,
        rationale: "Typed browser handoff packets preserve the reason code, privacy consequence, and return anchor without dropping local context.".to_owned(),
    };
    let reconciliation_story = M5TeamWorkflowReconciliationStory {
        proof_ref: PROVIDER_EVENT_INGESTION_SUPPORT_EXPORT_ARTIFACT_REF.to_owned(),
        typed_event_envelopes_visible: true,
        replay_ledger_visible: true,
        callback_denials_auditable: true,
        dedupe_visible: true,
        typed_reconciliation_required_for_mutation: true,
        rationale: "Provider event ingestion and reconciliation keep typed envelopes, replay-ledger lineage, dedupe decisions, and callback-deny audit events visible before imported truth can affect user-visible state.".to_owned(),
    };

    match row {
        M5TeamWorkflowClaimedRow::WorkItemObjectVocabulary => RowDescriptor {
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix::M5_PROVIDER_WORKITEM_GOVERNANCE_RECORD_KIND,
            evidence_artifact_ref:
                crate::freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix::M5_PROVIDER_WORKITEM_GOVERNANCE_ARTIFACT_REF,
            evidence_schema_ref:
                crate::freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix::M5_PROVIDER_WORKITEM_GOVERNANCE_SCHEMA_REF,
            evidence_doc_ref:
                crate::freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix::M5_PROVIDER_WORKITEM_GOVERNANCE_DOC_REF,
            feature_scorecard_summary: "Stable when provider-backed work items, local drafts, queued publishes, imported snapshots, and provider-committed state remain visibly distinct.",
            provider_family_compatibility: vec![
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::IssueTracker,
                    posture: Posture::Qualified,
                    rationale: "Issue-tracker work items, tickets, and incidents are explicitly modeled as provider-backed objects with local-draft, queued-publish, and provider-committed separation.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CodeHost,
                    posture: Posture::Limited,
                    rationale: "Code-host evidence currently covers review-linked change intent and browser handoff, but not full provider-backed work-item detail rows.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CiChecks,
                    posture: Posture::NotMarketed,
                    rationale: "CI providers contribute acting-identity and reconciliation proof, but they do not inherit the work-item object-vocabulary claim.".to_owned(),
                },
            ],
            acting_identity_story: scope_story.clone(),
            offline_publish_later_story: review_queue_story.clone(),
            reconciliation_story: reconciliation_story.clone(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::ProviderAuthorityStale,
                Trigger::PublishLaterContinuityStale,
                Trigger::CallbackReconciliationStale,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::ProviderTruthSeparationPreserved,
            evidence_probe: || crate::freeze_the_m5_work_item_provider_link_acting_identity_and_publish_later_continuity_matrix::current_stable_m5_provider_workitem_governance_export().is_ok(),
        },
        M5TeamWorkflowClaimedRow::ProviderLinkedMutation => RowDescriptor {
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews::WORK_ITEM_MUTATION_REVIEW_RECORD_KIND,
            evidence_artifact_ref:
                crate::ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews::WORK_ITEM_MUTATION_REVIEW_ARTIFACT_REF,
            evidence_schema_ref:
                crate::ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews::WORK_ITEM_MUTATION_REVIEW_SCHEMA_REF,
            evidence_doc_ref:
                crate::ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews::WORK_ITEM_MUTATION_REVIEW_DOC_REF,
            feature_scorecard_summary: "Stable when publish-now, local-draft, and open-in-provider mutation review all keep target, actor, side effects, and fallback posture explicit.",
            provider_family_compatibility: vec![
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::IssueTracker,
                    posture: Posture::Qualified,
                    rationale: "Issue-tracker mutation review covers provider-authoritative rows, local drafts, queued publishes, offline capture, and browser-only handoff.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CodeHost,
                    posture: Posture::Limited,
                    rationale: "Code-host evidence currently proves browser-only and comment-scope acting-identity resolution, not a full provider-backed work-item mutation review sheet.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CiChecks,
                    posture: Posture::NotMarketed,
                    rationale: "CI providers do not inherit the work-item mutation-review badge because no current transition-review packet exists for that family.".to_owned(),
                },
            ],
            acting_identity_story: scope_story.clone(),
            offline_publish_later_story: review_queue_story.clone(),
            reconciliation_story: reconciliation_story.clone(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::ProviderAuthorityStale,
                Trigger::EffectiveScopeDrift,
                Trigger::BrowserOnlyFallback,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::LocalDraftPreserved,
            evidence_probe: || crate::ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews::current_work_item_mutation_review_export().is_ok(),
        },
        M5TeamWorkflowClaimedRow::ActingIdentityAndEffectiveScope => RowDescriptor {
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind: PROVIDER_SCOPE_REVIEW_PAGE_RECORD_KIND,
            evidence_artifact_ref: PROVIDER_SCOPE_REVIEW_SUPPORT_EXPORT_ARTIFACT_REF,
            evidence_schema_ref: PROVIDER_SCOPE_REVIEW_SCHEMA_REF,
            evidence_doc_ref: PROVIDER_SCOPE_REVIEW_DOC_REF,
            feature_scorecard_summary: "Stable when every provider-backed action keeps acting identity, authority health, effective scope, policy locks, and least-privilege fallbacks explicit.",
            provider_family_compatibility: vec![
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::IssueTracker,
                    posture: Posture::Qualified,
                    rationale: "Issue-tracker deferred-publish, denied-scope, and delegated-credential rows remain first-class in scope review.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CodeHost,
                    posture: Posture::Qualified,
                    rationale: "Code-host scope review covers human-account publish-now and browser-only fallback without collapsing them into one optimistic connected state.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CiChecks,
                    posture: Posture::Qualified,
                    rationale: "CI scope review proves installation-grant authority is explicit and bounded rather than inheriting human-account scope.".to_owned(),
                },
            ],
            acting_identity_story: scope_story.clone(),
            offline_publish_later_story: review_queue_story.clone(),
            reconciliation_story: reconciliation_story.clone(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::ProviderAuthorityStale,
                Trigger::EffectiveScopeDrift,
                Trigger::BrowserOnlyFallback,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::RecomputeScopeBeforeRetry,
            evidence_probe: current_provider_scope_review_fixture_is_valid,
        },
        M5TeamWorkflowClaimedRow::BrowserHandoffContinuity => RowDescriptor {
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links::HANDOFF_CONTINUITY_RECORD_KIND,
            evidence_artifact_ref:
                crate::ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links::HANDOFF_CONTINUITY_ARTIFACT_REF,
            evidence_schema_ref:
                crate::ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links::HANDOFF_CONTINUITY_SCHEMA_REF,
            evidence_doc_ref:
                crate::ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links::HANDOFF_CONTINUITY_DOC_REF,
            feature_scorecard_summary: "Stable when typed handoff packets keep reason code, destination class, privacy consequence, and return anchors visible across provider escapes.",
            provider_family_compatibility: vec![
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::IssueTracker,
                    posture: Posture::Qualified,
                    rationale: "Issue-tracker transition review and offline handoff packets preserve browser-open pathways without discarding local review context.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CodeHost,
                    posture: Posture::Qualified,
                    rationale: "Code-host scope review proves browser-only merge fallbacks remain typed and return-anchor safe.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CiChecks,
                    posture: Posture::NotMarketed,
                    rationale: "CI providers do not currently claim a browser-handoff continuity lane for provider-backed team workflows.".to_owned(),
                },
            ],
            acting_identity_story: scope_story.clone(),
            offline_publish_later_story: handoff_story.clone(),
            reconciliation_story: reconciliation_story.clone(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::BrowserOnlyFallback,
                Trigger::ReturnAnchorUnproven,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::ReturnAnchorPreserved,
            evidence_probe: || crate::ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links::current_handoff_continuity_export().is_ok(),
        },
        M5TeamWorkflowClaimedRow::DeferredPublishContinuity => RowDescriptor {
            claimed_qualification: Qual::Stable,
            default_verdict: Verdict::Certified,
            upstream_record_kind:
                crate::ship_deferred_publish_queue_recovery_packets::DEFERRED_PUBLISH_QUEUE_RECOVERY_RECORD_KIND,
            evidence_artifact_ref:
                crate::ship_deferred_publish_queue_recovery_packets::DEFERRED_PUBLISH_QUEUE_RECOVERY_ARTIFACT_REF,
            evidence_schema_ref:
                crate::ship_deferred_publish_queue_recovery_packets::DEFERRED_PUBLISH_QUEUE_RECOVERY_SCHEMA_REF,
            evidence_doc_ref:
                crate::ship_deferred_publish_queue_recovery_packets::DEFERRED_PUBLISH_QUEUE_RECOVERY_DOC_REF,
            feature_scorecard_summary: "Stable when deferred publish packets survive restart, reconnect, provider outage, host mismatch, and redaction policy review without losing lineage.",
            provider_family_compatibility: vec![
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::IssueTracker,
                    posture: Posture::Qualified,
                    rationale: "Issue-tracker deferred publish currently carries the full retry, discard, export-packet, and reopen-external proof set.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CodeHost,
                    posture: Posture::Limited,
                    rationale: "Code-host continuity is limited to browser-only fallback and export-safe handoff evidence; it does not inherit the stable deferred-publish queue badge.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CiChecks,
                    posture: Posture::NotMarketed,
                    rationale: "CI providers do not currently carry a publish-later queue certification for team-workflow claims.".to_owned(),
                },
            ],
            acting_identity_story: scope_story.clone(),
            offline_publish_later_story: review_queue_story.clone(),
            reconciliation_story: reconciliation_story.clone(),
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::PublishLaterContinuityStale,
                Trigger::EffectiveScopeDrift,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::DeferredPublishPacketPreserved,
            evidence_probe: || crate::ship_deferred_publish_queue_recovery_packets::current_deferred_publish_queue_recovery_export().is_ok(),
        },
        M5TeamWorkflowClaimedRow::ProviderEventReconciliation => RowDescriptor {
            claimed_qualification: Qual::Beta,
            default_verdict: Verdict::NarrowedCertified,
            upstream_record_kind: PROVIDER_EVENT_INGESTION_PACKET_RECORD_KIND,
            evidence_artifact_ref: PROVIDER_EVENT_INGESTION_SUPPORT_EXPORT_ARTIFACT_REF,
            evidence_schema_ref: PROVIDER_EVENT_INGESTION_SCHEMA_REF,
            evidence_doc_ref: PROVIDER_EVENT_INGESTION_DOC_REF,
            feature_scorecard_summary: "Beta until typed import sessions, replay ledgers, dedupe, and callback-deny audit events stay current across every provider-backed workflow row.",
            provider_family_compatibility: vec![
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::IssueTracker,
                    posture: Posture::Qualified,
                    rationale: "Issue-tracker imports, polling refresh, and deferred-publish reconciliation are currently proved only at the beta-scoped reconciliation row.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CodeHost,
                    posture: Posture::Qualified,
                    rationale: "Code-host webhook and replay coverage is current, but it remains explicitly beta-scoped inside the canonical reconciliation row.".to_owned(),
                },
                M5TeamWorkflowProviderFamilyCompatibility {
                    provider_family: Family::CiChecks,
                    posture: Posture::Limited,
                    rationale: "CI check-state events participate in typed event ingestion, but broader provider-backed workflow marketing must stay narrower than issue-tracker or code-host proof.".to_owned(),
                },
            ],
            acting_identity_story: scope_story,
            offline_publish_later_story: review_queue_story,
            reconciliation_story,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::EvidencePacketInvalid,
                Trigger::CallbackReconciliationStale,
                Trigger::ReplayLedgerUnavailable,
                Trigger::UpstreamDependencyNarrowed,
            ],
            rollback_posture: Rollback::AuditOnlyEvidencePreserved,
            evidence_probe: current_provider_event_ingestion_fixture_is_valid,
        },
    }
}

fn current_provider_scope_review_fixture_is_valid() -> bool {
    let page: ProviderScopeReviewPage = match serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/providers/m5/provider_scope_review/page.json"
    ))) {
        Ok(page) => page,
        Err(_) => return false,
    };
    validate_provider_scope_review_page(&page).is_ok()
}

fn current_provider_event_ingestion_fixture_is_valid() -> bool {
    let packet: ProviderEventIngestionPacket = match serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../fixtures/providers/m5/event_ingestion/packet.json"
    ))) {
        Ok(packet) => packet,
        Err(_) => return false,
    };
    validate_provider_event_ingestion_packet(&packet).passed
}

fn validate_source_contracts(
    packet: &M5TeamWorkflowCertificationPacket,
    violations: &mut Vec<M5TeamWorkflowCertificationViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TEAM_WORKFLOW_CERTIFICATION_SCHEMA_REF,
        M5_TEAM_WORKFLOW_CERTIFICATION_DOC_REF,
        M5_TEAM_WORKFLOW_CERTIFICATION_MATRIX_SCHEMA_REF,
        M5_TEAM_WORKFLOW_CERTIFICATION_MATRIX_DOC_REF,
        PROVIDER_SCOPE_REVIEW_SCHEMA_REF,
        PROVIDER_EVENT_INGESTION_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TeamWorkflowCertificationViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &M5TeamWorkflowCertificationPacket,
    violations: &mut Vec<M5TeamWorkflowCertificationViolation>,
) {
    let present: BTreeSet<M5TeamWorkflowClaimedRow> =
        packet.certified_rows.iter().map(|row| row.row).collect();
    for required in M5TeamWorkflowClaimedRow::ALL {
        if !present.contains(&required) {
            violations.push(M5TeamWorkflowCertificationViolation::RequiredRowMissing);
            return;
        }
    }

    for row in &packet.certified_rows {
        if row.upstream_record_kind.trim().is_empty()
            || row.evidence_artifact_ref.trim().is_empty()
            || row.evidence_schema_ref.trim().is_empty()
            || row.evidence_doc_ref.trim().is_empty()
            || row.proof_freshness.last_proof_refresh.trim().is_empty()
            || row.proof_freshness.proof_freshness_slo_hours == 0
        {
            violations.push(M5TeamWorkflowCertificationViolation::RowIncomplete);
        }
        if row.feature_scorecard_summary.trim().is_empty() {
            violations.push(M5TeamWorkflowCertificationViolation::FeatureScorecardMissing);
        }
        if row.provider_family_compatibility.is_empty()
            || row
                .provider_family_compatibility
                .iter()
                .any(|entry| entry.rationale.trim().is_empty())
        {
            violations
                .push(M5TeamWorkflowCertificationViolation::ProviderFamilyCompatibilityMissing);
        }
        if row.acting_identity_story.proof_ref.trim().is_empty()
            || row.acting_identity_story.rationale.trim().is_empty()
        {
            violations.push(M5TeamWorkflowCertificationViolation::ActingIdentityStoryIncomplete);
        }
        if row.offline_publish_later_story.proof_ref.trim().is_empty()
            || row.offline_publish_later_story.rationale.trim().is_empty()
        {
            violations
                .push(M5TeamWorkflowCertificationViolation::OfflinePublishLaterStoryIncomplete);
        }
        if row.reconciliation_story.proof_ref.trim().is_empty()
            || row.reconciliation_story.rationale.trim().is_empty()
        {
            violations.push(M5TeamWorkflowCertificationViolation::ReconciliationStoryIncomplete);
        }
        if row.verdict.is_publishable() && row.evidence_artifact_ref.trim().is_empty() {
            violations.push(M5TeamWorkflowCertificationViolation::PublishableRowMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5TeamWorkflowCertificationViolation::DowngradeTriggersMissing);
        }
    }
}

fn validate_compatibility_report(
    packet: &M5TeamWorkflowCertificationPacket,
    violations: &mut Vec<M5TeamWorkflowCertificationViolation>,
) {
    let recomputed =
        M5TeamWorkflowCertificationCompatibilityReport::from_rows(&packet.certified_rows);
    if recomputed != packet.compatibility_report {
        violations.push(M5TeamWorkflowCertificationViolation::CompatibilityReportMismatch);
    }
}

fn validate_trust_review(
    packet: &M5TeamWorkflowCertificationPacket,
    violations: &mut Vec<M5TeamWorkflowCertificationViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.provider_objects_never_masquerade_as_local_truth,
        review.truth_states_remain_visibly_distinct,
        review.acting_identity_visible,
        review.effective_scope_visible,
        review.browser_handoff_return_anchor_safe,
        review.deferred_publish_survives_restart,
        review.deferred_publish_survives_reconnect,
        review.callback_events_deduplicated,
        review.callback_denials_auditable,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_broad_claims,
    ] {
        if !ok {
            violations.push(M5TeamWorkflowCertificationViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TeamWorkflowCertificationPacket,
    violations: &mut Vec<M5TeamWorkflowCertificationViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.review_workspace_shows_certification,
        projection.work_item_detail_shows_certification,
        projection.incident_workspace_shows_certification,
        projection.cli_headless_shows_certification,
        projection.support_export_shows_certification,
        projection.help_about_shows_certification,
        projection.service_health_shows_certification,
        projection.public_truth_pack_shows_certification,
        projection.release_manifests_show_certification,
        projection.provider_family_specific_badging_visible,
    ] {
        if !ok {
            violations.push(M5TeamWorkflowCertificationViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TeamWorkflowCertificationPacket,
    violations: &mut Vec<M5TeamWorkflowCertificationViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TeamWorkflowCertificationViolation::ProofFreshnessIncomplete);
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
