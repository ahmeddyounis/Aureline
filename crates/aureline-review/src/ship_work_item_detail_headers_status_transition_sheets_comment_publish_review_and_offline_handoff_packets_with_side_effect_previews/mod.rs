//! Stable M5 work-item mutation review packet with detail headers, transition
//! sheets, comment publish review, and offline handoff continuity.
//!
//! This module composes the existing provider work-item detail, transition
//! review, comment publish-review, and offline handoff vocabularies into one
//! export-safe packet that desktop, CLI/headless, companion, and support
//! surfaces can all reuse without drifting copy or truth classes.
//!
//! The packet keeps four surfaces aligned:
//!
//! - [`WorkItemDetailHeaderRow`] exposes provider boundary, canonical identity,
//!   state, owner, sync state, and open-external truth.
//! - [`StatusTransitionReviewRow`] previews current and requested state, linked
//!   branch or review notes, side effects, authority, publish mode, and local
//!   fallback before any mutation leaves the machine.
//! - [`CommentPublishReviewRow`] keeps local draft and external post separate
//!   while disclosing visibility target, evidence refs, notification fanout,
//!   publish mode, and offline fallback.
//! - [`OfflineHandoffPacketRow`] preserves captured note context, code links,
//!   evidence refs, redaction state, expiry, retry/export affordances, and
//!   publish-later target without implying that the provider has accepted the
//!   update.
//!
//! The packet is intentionally projection-first: it consumes the upstream
//! provider records and re-emits only the stable cross-surface fields needed by
//! review, companion, and support/export consumers. Raw provider payloads, raw
//! URLs, raw comment bodies, credentials, and provider responses stay outside
//! the boundary.
//!
//! The boundary schema is
//! [`schemas/review/ship-work-item-detail-headers-status-transition-sheets-comment-publish-review-and-offline-handoff-packets-with-side-effect-previews.schema.json`](../../../../schemas/review/ship-work-item-detail-headers-status-transition-sheets-comment-publish-review-and-offline-handoff-packets-with-side-effect-previews.schema.json).
//! The contract doc is
//! [`docs/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews.md`](../../../../docs/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews.md).
//! The protected fixture directory is
//! [`fixtures/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/`](../../../../fixtures/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use aureline_provider::{
    seeded_work_item_sync_beta_page, seeded_work_item_transition_beta_page, HandoffAdmissionReasonClass,
    HandoffDrainStateClass, HandoffExportRouteClass, HandoffProviderAcceptanceClass,
    HandoffRetryRouteClass, OpenExternalActionClass, PermissionScopeClass, ProviderFamily,
    PublishReviewActionClass, PublishReviewActorScopeClass, PublishReviewDispositionClass,
    RedactionClass, WorkItemDetailRecord, WorkItemFreshnessClass, WorkItemMutationMode,
    WorkItemPublishPostureClass, WorkItemRowPostureClass, WriteAuthorityClass,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`WorkItemMutationReviewPacket`].
pub const WORK_ITEM_MUTATION_REVIEW_RECORD_KIND: &str =
    "ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews";

/// Schema version for work-item mutation review packet records.
pub const WORK_ITEM_MUTATION_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const WORK_ITEM_MUTATION_REVIEW_SCHEMA_REF: &str =
    "schemas/review/ship-work-item-detail-headers-status-transition-sheets-comment-publish-review-and-offline-handoff-packets-with-side-effect-previews.schema.json";

/// Repo-relative path of the contract doc.
pub const WORK_ITEM_MUTATION_REVIEW_DOC_REF: &str =
    "docs/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews.md";

/// Repo-relative path of the stable work-item governance matrix this packet reuses.
pub const WORK_ITEM_MUTATION_REVIEW_GOVERNANCE_CONTRACT_REF: &str =
    "schemas/review/freeze-the-m5-work-item-provider-link-acting-identity-and-publish-later-continuity-matrix.schema.json";

/// Repo-relative path of the provider work-item detail schema.
pub const WORK_ITEM_MUTATION_REVIEW_DETAIL_CONTRACT_REF: &str =
    "schemas/work_items/work_item_detail.schema.json";

/// Repo-relative path of the provider transition review schema.
pub const WORK_ITEM_MUTATION_REVIEW_TRANSITION_CONTRACT_REF: &str =
    "schemas/work_items/transition_review.schema.json";

/// Repo-relative path of the provider publish-review schema.
pub const WORK_ITEM_MUTATION_REVIEW_COMMENT_CONTRACT_REF: &str =
    "schemas/providers/publish_review.schema.json";

/// Repo-relative path of the provider offline-handoff schema.
pub const WORK_ITEM_MUTATION_REVIEW_HANDOFF_CONTRACT_REF: &str =
    "schemas/providers/offline_handoff_packet.schema.json";

/// Repo-relative path of the browser/provider handoff continuity contract.
pub const WORK_ITEM_MUTATION_REVIEW_BROWSER_HANDOFF_CONTRACT_REF: &str =
    "schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const WORK_ITEM_MUTATION_REVIEW_FIXTURE_DIR: &str =
    "fixtures/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews";

/// Repo-relative path of the checked support-export artifact.
pub const WORK_ITEM_MUTATION_REVIEW_ARTIFACT_REF: &str =
    "artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const WORK_ITEM_MUTATION_REVIEW_SUMMARY_REF: &str =
    "artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews.md";

/// Stable consumer surface that must project this packet's vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemMutationReviewConsumerSurface {
    /// Desktop work-item detail and review sheets.
    DesktopWorkItemDetail,
    /// Review workspace and adjacent review strips.
    ReviewWorkspace,
    /// CLI or headless JSON output.
    CliHeadless,
    /// Browser or mobile companion surfaces.
    Companion,
    /// Support/export packet surfaces.
    SupportExport,
    /// Browser handoff and return-anchor surfaces.
    BrowserHandoff,
}

impl WorkItemMutationReviewConsumerSurface {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopWorkItemDetail => "desktop_work_item_detail",
            Self::ReviewWorkspace => "review_workspace",
            Self::CliHeadless => "cli_headless",
            Self::Companion => "companion",
            Self::SupportExport => "support_export",
            Self::BrowserHandoff => "browser_handoff",
        }
    }
}

/// Downgrade trigger that narrows this packet's claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemMutationReviewDowngradeTrigger {
    /// Provider authority or freshness proof went stale.
    ProviderAuthorityStale,
    /// Side-effect previews are incomplete or missing.
    SideEffectPreviewMissing,
    /// Comment review no longer separates local draft and external post.
    CommentDraftBoundaryMissing,
    /// Publish mode is ambiguous or hidden.
    PublishModeAmbiguous,
    /// Offline continuity or restartability proof went stale.
    OfflineContinuityStale,
    /// Redaction or expiry disclosure went stale.
    RedactionOrExpiryMissing,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl WorkItemMutationReviewDowngradeTrigger {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAuthorityStale => "provider_authority_stale",
            Self::SideEffectPreviewMissing => "side_effect_preview_missing",
            Self::CommentDraftBoundaryMissing => "comment_draft_boundary_missing",
            Self::PublishModeAmbiguous => "publish_mode_ambiguous",
            Self::OfflineContinuityStale => "offline_continuity_stale",
            Self::RedactionOrExpiryMissing => "redaction_or_expiry_missing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Stable detail-header projection reused across review and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemDetailHeaderRow {
    /// Stable header id.
    pub header_id: String,
    /// Underlying provider detail record id.
    pub work_item_detail_record_id_ref: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Reviewable provider label.
    pub provider_label: String,
    /// Project or space ref shown in the header.
    pub project_or_space_ref: String,
    /// Canonical provider-owned identity shown to the user.
    pub canonical_id: String,
    /// Reviewable title shown in the header.
    pub title_label: String,
    /// Provider boundary disclosure label.
    pub provider_boundary_label: String,
    /// Current state summary.
    pub current_state_summary: String,
    /// Owner or assignee summary.
    pub owner_or_assignee_summary: String,
    /// Sync-state summary.
    pub sync_state_summary: String,
    /// Freshness class.
    pub freshness_class: WorkItemFreshnessClass,
    /// Write-authority class.
    pub write_authority_class: WriteAuthorityClass,
    /// Publish posture class.
    pub publish_posture_class: WorkItemPublishPostureClass,
    /// Open-external action class.
    pub open_external_action_class: OpenExternalActionClass,
    /// Open-external action label.
    pub open_external_action_label: String,
    /// Linked transition review refs.
    pub linked_transition_review_ids: Vec<String>,
    /// Linked offline handoff refs.
    pub linked_offline_handoff_ids: Vec<String>,
    /// Export-safe summary.
    pub summary_label: String,
}

/// Stable status-transition review projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusTransitionReviewRow {
    /// Stable row id.
    pub row_id: String,
    /// Underlying transition-review record id.
    pub transition_review_record_id_ref: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Bound status-transition packet ref.
    pub status_transition_packet_record_id_ref: String,
    /// Target item identity shown in review.
    pub target_item_ref: String,
    /// Current state summary.
    pub current_state_summary: String,
    /// Requested state or change-set summary.
    pub requested_state_summary: String,
    /// Linked branch or review note.
    pub linked_branch_or_review_note: String,
    /// Actor authority summary.
    pub actor_authority_summary: String,
    /// Permission scope class.
    pub permission_scope_class: PermissionScopeClass,
    /// Publish mode class.
    pub publish_mode_class: WorkItemMutationMode,
    /// Review disposition summary.
    pub disposition_summary_label: String,
    /// Confirm action availability.
    pub confirm_action_available: bool,
    /// Export action availability.
    pub export_action_available: bool,
    /// Cancel action availability.
    pub cancel_action_available: bool,
    /// Side-effect preview summaries.
    pub side_effect_summaries: Vec<String>,
    /// Local draft is preserved on failure.
    pub local_draft_preserved_on_failure: bool,
    /// Offline or local fallback label.
    pub fallback_label: String,
    /// Export-safe summary.
    pub summary_label: String,
}

/// Stable comment publish-review projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommentPublishReviewRow {
    /// Stable row id.
    pub row_id: String,
    /// Underlying publish-review record id.
    pub publish_review_record_id_ref: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Optional bound comment-sync ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comment_sync_state_record_id_ref: Option<String>,
    /// Optional bound transition packet ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_transition_packet_record_id_ref: Option<String>,
    /// Publish-review action class.
    pub action_class: PublishReviewActionClass,
    /// Publish mode class.
    pub publish_mode_class: WorkItemMutationMode,
    /// Publish-review disposition class.
    pub disposition_class: PublishReviewDispositionClass,
    /// Actor-scope class.
    pub actor_scope_class: PublishReviewActorScopeClass,
    /// Local draft summary.
    pub local_draft_summary: String,
    /// External-post summary.
    pub external_post_summary: String,
    /// Visibility target label.
    pub visibility_target_label: String,
    /// Evidence refs shown during review.
    pub evidence_refs: Vec<String>,
    /// Notification behavior label.
    pub notify_behavior_label: String,
    /// Offline or local fallback label.
    pub fallback_label: String,
    /// Side-effect preview summaries.
    pub side_effect_summaries: Vec<String>,
    /// Export-safe summary.
    pub summary_label: String,
}

/// Stable offline handoff packet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineHandoffPacketRow {
    /// Stable row id.
    pub row_id: String,
    /// Underlying offline-handoff packet ref.
    pub offline_handoff_packet_record_id_ref: String,
    /// Bound work-item detail ref.
    pub work_item_detail_record_id_ref: String,
    /// Bound status-transition packet ref.
    pub status_transition_packet_record_id_ref: String,
    /// Admission reason class.
    pub admission_reason_class: HandoffAdmissionReasonClass,
    /// Provider acceptance class.
    pub provider_acceptance_class: HandoffProviderAcceptanceClass,
    /// Drain-state class.
    pub drain_state_class: HandoffDrainStateClass,
    /// Export route classes.
    pub export_route_classes: Vec<HandoffExportRouteClass>,
    /// Retry route classes.
    pub retry_route_classes: Vec<HandoffRetryRouteClass>,
    /// Captured note summary.
    pub captured_note_summary: String,
    /// Code-link refs preserved by the packet.
    pub code_link_refs: Vec<String>,
    /// Evidence refs preserved by the packet.
    pub evidence_refs: Vec<String>,
    /// Redaction class.
    pub redaction_class: RedactionClass,
    /// Redaction manifest ref.
    pub redaction_manifest_ref: String,
    /// Packet expiry.
    pub expires_at: String,
    /// Publish-later target ref.
    pub publish_target_ref: String,
    /// Retry action ref.
    pub retry_action_ref: String,
    /// Export action ref.
    pub export_action_ref: String,
    /// Packet survives restart.
    pub packet_survives_restart: bool,
    /// Export-safe summary.
    pub summary_label: String,
}

/// Trust-review invariants for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemMutationReviewTrustReview {
    /// Detail headers disclose provider boundary.
    pub detail_headers_disclose_provider_boundary: bool,
    /// Detail headers disclose canonical identity.
    pub detail_headers_disclose_canonical_identity: bool,
    /// Detail headers disclose current state and owner.
    pub detail_headers_disclose_state_and_owner: bool,
    /// Detail headers disclose sync state and open-external action.
    pub detail_headers_disclose_sync_state_and_open_external: bool,
    /// Transition sheets disclose side effects.
    pub transition_sheets_disclose_side_effects: bool,
    /// Transition sheets disclose authority and publish mode.
    pub transition_sheets_disclose_authority_and_publish_mode: bool,
    /// Comment review keeps local draft distinct from external post.
    pub comment_reviews_split_local_draft_from_external_post: bool,
    /// Comment review discloses visibility, evidence, and notifications.
    pub comment_reviews_disclose_visibility_and_notifications: bool,
    /// Offline packets preserve redaction and expiry state.
    pub offline_packets_preserve_redaction_and_expiry: bool,
    /// Offline packets do not claim provider acceptance.
    pub offline_packets_do_not_claim_provider_acceptance: bool,
    /// Outage or policy block preserves local draft or handoff.
    pub outage_or_policy_block_preserves_local_intent: bool,
    /// Desktop, CLI, companion, and support reuse one vocabulary.
    pub stable_vocabulary_across_surfaces: bool,
    /// Downgrade narrows rather than hides the lane.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Passive inspection never publishes externally.
    pub no_passive_inspection_external_publish: bool,
}

impl WorkItemMutationReviewTrustReview {
    /// Whether every trust invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.detail_headers_disclose_provider_boundary
            && self.detail_headers_disclose_canonical_identity
            && self.detail_headers_disclose_state_and_owner
            && self.detail_headers_disclose_sync_state_and_open_external
            && self.transition_sheets_disclose_side_effects
            && self.transition_sheets_disclose_authority_and_publish_mode
            && self.comment_reviews_split_local_draft_from_external_post
            && self.comment_reviews_disclose_visibility_and_notifications
            && self.offline_packets_preserve_redaction_and_expiry
            && self.offline_packets_do_not_claim_provider_acceptance
            && self.outage_or_policy_block_preserves_local_intent
            && self.stable_vocabulary_across_surfaces
            && self.downgrade_narrows_instead_of_hides
            && self.no_passive_inspection_external_publish
    }
}

/// Consumer-projection invariants for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemMutationReviewConsumerProjection {
    /// Desktop detail cards use the packet's stable vocabulary.
    pub desktop_detail_uses_stable_vocabulary: bool,
    /// Review workspace uses the packet's stable vocabulary.
    pub review_workspace_uses_stable_vocabulary: bool,
    /// CLI/headless export uses the packet's stable vocabulary.
    pub cli_headless_uses_stable_vocabulary: bool,
    /// Companion uses the packet's stable vocabulary.
    pub companion_uses_stable_vocabulary: bool,
    /// Support export uses the packet's stable vocabulary.
    pub support_export_uses_stable_vocabulary: bool,
    /// Browser handoff preserves the packet's publish-mode truth.
    pub browser_handoff_uses_stable_vocabulary: bool,
    /// Transition sheets show side-effect previews.
    pub transition_sheet_shows_side_effect_preview: bool,
    /// Comment review shows publish mode and fallback.
    pub comment_review_shows_publish_mode_and_fallback: bool,
    /// Offline packet shows expiry and redaction.
    pub offline_packet_shows_expiry_and_redaction: bool,
    /// Blocked or offline states preserve retry or export paths.
    pub blocked_or_offline_state_preserves_retry_or_export: bool,
    /// Unqualified rows remain labeled.
    pub label_for_unqualified: bool,
}

impl WorkItemMutationReviewConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.desktop_detail_uses_stable_vocabulary
            && self.review_workspace_uses_stable_vocabulary
            && self.cli_headless_uses_stable_vocabulary
            && self.companion_uses_stable_vocabulary
            && self.support_export_uses_stable_vocabulary
            && self.browser_handoff_uses_stable_vocabulary
            && self.transition_sheet_shows_side_effect_preview
            && self.comment_review_shows_publish_mode_and_fallback
            && self.offline_packet_shows_expiry_and_redaction
            && self.blocked_or_offline_state_preserves_retry_or_export
            && self.label_for_unqualified
    }
}

/// Proof-freshness block for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemMutationReviewProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`WorkItemMutationReviewPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkItemMutationReviewPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Detail-header rows.
    pub detail_headers: Vec<WorkItemDetailHeaderRow>,
    /// Transition-review rows.
    pub transition_reviews: Vec<StatusTransitionReviewRow>,
    /// Comment publish-review rows.
    pub comment_publish_reviews: Vec<CommentPublishReviewRow>,
    /// Offline handoff rows.
    pub offline_handoff_packets: Vec<OfflineHandoffPacketRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<WorkItemMutationReviewDowngradeTrigger>,
    /// Consumer surfaces that must project this lane.
    pub consumer_surfaces: Vec<WorkItemMutationReviewConsumerSurface>,
    /// Trust review block.
    pub trust_review: WorkItemMutationReviewTrustReview,
    /// Consumer projection block.
    pub consumer_projection: WorkItemMutationReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: WorkItemMutationReviewProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 packet for work-item mutation review continuity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemMutationReviewPacket {
    /// Record kind; must equal [`WORK_ITEM_MUTATION_REVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`WORK_ITEM_MUTATION_REVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Detail-header rows.
    pub detail_headers: Vec<WorkItemDetailHeaderRow>,
    /// Transition-review rows.
    pub transition_reviews: Vec<StatusTransitionReviewRow>,
    /// Comment publish-review rows.
    pub comment_publish_reviews: Vec<CommentPublishReviewRow>,
    /// Offline handoff rows.
    pub offline_handoff_packets: Vec<OfflineHandoffPacketRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<WorkItemMutationReviewDowngradeTrigger>,
    /// Consumer surfaces that must project this lane.
    pub consumer_surfaces: Vec<WorkItemMutationReviewConsumerSurface>,
    /// Trust review block.
    pub trust_review: WorkItemMutationReviewTrustReview,
    /// Consumer projection block.
    pub consumer_projection: WorkItemMutationReviewConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: WorkItemMutationReviewProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl WorkItemMutationReviewPacket {
    /// Builds a work-item mutation review packet from stable-lane input.
    pub fn new(input: WorkItemMutationReviewPacketInput) -> Self {
        Self {
            record_kind: WORK_ITEM_MUTATION_REVIEW_RECORD_KIND.to_owned(),
            schema_version: WORK_ITEM_MUTATION_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            detail_headers: input.detail_headers,
            transition_reviews: input.transition_reviews,
            comment_publish_reviews: input.comment_publish_reviews,
            offline_handoff_packets: input.offline_handoff_packets,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the packet invariants.
    pub fn validate(&self) -> Vec<WorkItemMutationReviewViolation> {
        let mut violations = Vec::new();

        if self.record_kind != WORK_ITEM_MUTATION_REVIEW_RECORD_KIND {
            violations.push(WorkItemMutationReviewViolation::WrongRecordKind);
        }
        if self.schema_version != WORK_ITEM_MUTATION_REVIEW_SCHEMA_VERSION {
            violations.push(WorkItemMutationReviewViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(WorkItemMutationReviewViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(WorkItemMutationReviewViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(WorkItemMutationReviewViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_detail_headers(self, &mut violations);
        validate_transition_reviews(self, &mut violations);
        validate_comment_publish_reviews(self, &mut violations);
        validate_offline_handoff_packets(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(WorkItemMutationReviewViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(WorkItemMutationReviewViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(WorkItemMutationReviewViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("work-item mutation review packet serializes"),
        ) {
            violations.push(WorkItemMutationReviewViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("work-item mutation review packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let deferred_comments = self
            .comment_publish_reviews
            .iter()
            .filter(|row| row.publish_mode_class != WorkItemMutationMode::PublishNow)
            .count();
        let blocked_transitions = self
            .transition_reviews
            .iter()
            .filter(|row| !row.confirm_action_available)
            .count();
        let offline_packets = self.offline_handoff_packets.len();

        let mut out = String::new();
        out.push_str(
            "# Work-Item Detail Headers, Transition Sheets, Comment Publish Review, and Offline Handoff Packets\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!("- Detail headers: {}\n", self.detail_headers.len()));
        out.push_str(&format!(
            "- Transition sheets: {} ({} blocked)\n",
            self.transition_reviews.len(),
            blocked_transitions
        ));
        out.push_str(&format!(
            "- Comment publish reviews: {} ({} deferred or local)\n",
            self.comment_publish_reviews.len(),
            deferred_comments
        ));
        out.push_str(&format!(
            "- Offline handoff packets: {}\n",
            offline_packets
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Detail headers\n\n");
        for row in &self.detail_headers {
            out.push_str(&format!(
                "- **{}** (`{}`): {}, owner `{}`, sync `{}`, open `{}`\n",
                row.canonical_id,
                row.provider_boundary_label,
                row.current_state_summary,
                row.owner_or_assignee_summary,
                row.sync_state_summary,
                row.open_external_action_label
            ));
        }

        out.push_str("\n## Transition sheets\n\n");
        for row in &self.transition_reviews {
            out.push_str(&format!(
                "- **{}**: `{}` -> `{}` via `{}`; fallback `{}`\n",
                row.target_item_ref,
                row.current_state_summary,
                row.requested_state_summary,
                work_item_mutation_mode_label(row.publish_mode_class),
                row.fallback_label
            ));
        }

        out.push_str("\n## Comment publish review\n\n");
        for row in &self.comment_publish_reviews {
            out.push_str(&format!(
                "- **{}**: `{}` / `{}`; visibility `{}`; fallback `{}`\n",
                row.summary_label,
                row.local_draft_summary,
                row.external_post_summary,
                row.visibility_target_label,
                row.fallback_label
            ));
        }

        out.push_str("\n## Offline handoff packets\n\n");
        for row in &self.offline_handoff_packets {
            out.push_str(&format!(
                "- **{}**: acceptance `{}`, drain `{}`, expiry `{}`, target `{}`\n",
                row.offline_handoff_packet_record_id_ref,
                handoff_provider_acceptance_label(row.provider_acceptance_class),
                handoff_drain_state_label(row.drain_state_class),
                row.expires_at,
                row.publish_target_ref
            ));
        }

        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum WorkItemMutationReviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<WorkItemMutationReviewViolation>),
}

impl fmt::Display for WorkItemMutationReviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "work-item mutation review export parse failed: {error}"
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
                    "work-item mutation review export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for WorkItemMutationReviewArtifactError {}

/// Validation failures emitted by [`WorkItemMutationReviewPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkItemMutationReviewViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No detail-header rows are present.
    DetailHeadersMissing,
    /// A detail-header row is incomplete.
    DetailHeaderIncomplete,
    /// No transition-review rows are present.
    TransitionReviewsMissing,
    /// A transition-review row is incomplete.
    TransitionReviewIncomplete,
    /// Transition review references a missing detail header.
    OrphanTransitionReference,
    /// No comment publish-review rows are present.
    CommentPublishReviewsMissing,
    /// A comment publish-review row is incomplete.
    CommentPublishReviewIncomplete,
    /// A publish-review row is not a comment action.
    CommentPublishReviewNotCommentAction,
    /// Comment publish-review references a missing detail header.
    OrphanCommentReference,
    /// No offline-handoff rows are present.
    OfflineHandoffPacketsMissing,
    /// An offline-handoff row is incomplete.
    OfflineHandoffPacketIncomplete,
    /// An offline-handoff row references a missing detail header.
    OrphanOfflineHandoffReference,
    /// An offline-handoff row implies provider acceptance.
    OfflineHandoffClaimsProviderAcceptance,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl WorkItemMutationReviewViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::DetailHeadersMissing => "detail_headers_missing",
            Self::DetailHeaderIncomplete => "detail_header_incomplete",
            Self::TransitionReviewsMissing => "transition_reviews_missing",
            Self::TransitionReviewIncomplete => "transition_review_incomplete",
            Self::OrphanTransitionReference => "orphan_transition_reference",
            Self::CommentPublishReviewsMissing => "comment_publish_reviews_missing",
            Self::CommentPublishReviewIncomplete => "comment_publish_review_incomplete",
            Self::CommentPublishReviewNotCommentAction => "comment_publish_review_not_comment_action",
            Self::OrphanCommentReference => "orphan_comment_reference",
            Self::OfflineHandoffPacketsMissing => "offline_handoff_packets_missing",
            Self::OfflineHandoffPacketIncomplete => "offline_handoff_packet_incomplete",
            Self::OrphanOfflineHandoffReference => "orphan_offline_handoff_reference",
            Self::OfflineHandoffClaimsProviderAcceptance => {
                "offline_handoff_claims_provider_acceptance"
            }
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Returns the canonical source-contract refs for this packet.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        WORK_ITEM_MUTATION_REVIEW_SCHEMA_REF.to_owned(),
        WORK_ITEM_MUTATION_REVIEW_DOC_REF.to_owned(),
        WORK_ITEM_MUTATION_REVIEW_GOVERNANCE_CONTRACT_REF.to_owned(),
        WORK_ITEM_MUTATION_REVIEW_DETAIL_CONTRACT_REF.to_owned(),
        WORK_ITEM_MUTATION_REVIEW_TRANSITION_CONTRACT_REF.to_owned(),
        WORK_ITEM_MUTATION_REVIEW_COMMENT_CONTRACT_REF.to_owned(),
        WORK_ITEM_MUTATION_REVIEW_HANDOFF_CONTRACT_REF.to_owned(),
        WORK_ITEM_MUTATION_REVIEW_BROWSER_HANDOFF_CONTRACT_REF.to_owned(),
    ]
}

/// Builds the canonical M5 work-item mutation review packet.
pub fn canonical_work_item_mutation_review_packet() -> WorkItemMutationReviewPacket {
    let transition_page = seeded_work_item_transition_beta_page();
    let sync_page = seeded_work_item_sync_beta_page();
    let detail_by_id: BTreeMap<&str, &WorkItemDetailRecord> = transition_page
        .detail_records
        .iter()
        .map(|detail| (detail.detail_id.as_str(), detail))
        .collect();

    let detail_headers = transition_page
        .detail_records
        .iter()
        .map(project_detail_header)
        .collect();

    let transition_reviews = transition_page
        .transition_reviews
        .iter()
        .map(|review| {
            let detail = detail_by_id
                .get(review.work_item_detail_record_id_ref.as_str())
                .expect("transition review detail exists");
            project_transition_review_row(detail, review)
        })
        .collect();

    let comment_publish_reviews = sync_page
        .publish_reviews
        .iter()
        .filter(|review| review.publish_review_action_class.is_comment_action())
        .map(|review| {
            let detail = detail_by_id
                .get(review.work_item_detail_record_id_ref.as_str())
                .expect("comment review detail exists");
            let comment_sync = review
                .comment_sync_state_record_id_ref
                .as_deref()
                .and_then(|sync_id| {
                    sync_page
                        .comment_sync_records
                        .iter()
                        .find(|record| record.comment_sync_id == sync_id)
                });
            project_comment_publish_review_row(detail, review, comment_sync)
        })
        .collect();

    let offline_handoff_packets = transition_page
        .offline_handoff_packets
        .iter()
        .filter(|packet| {
            !matches!(
                packet.handoff_provider_acceptance_class,
                HandoffProviderAcceptanceClass::ProviderAcceptConfirmedPublishLaterDrained
                    | HandoffProviderAcceptanceClass::ProviderAcceptRejectedWithTypedReason
            )
        })
        .map(|packet| {
            let detail = detail_by_id
                .get(packet.work_item_detail_record_id_ref.as_str())
                .expect("offline handoff detail exists");
            project_offline_handoff_row(detail, packet)
        })
        .collect();

    WorkItemMutationReviewPacket::new(WorkItemMutationReviewPacketInput {
        packet_id: "work-item-mutation-review:stable:0001".to_owned(),
        surface_label: "M5 work-item mutation review side-effect previews".to_owned(),
        detail_headers,
        transition_reviews,
        comment_publish_reviews,
        offline_handoff_packets,
        downgrade_triggers: vec![
            WorkItemMutationReviewDowngradeTrigger::ProviderAuthorityStale,
            WorkItemMutationReviewDowngradeTrigger::SideEffectPreviewMissing,
            WorkItemMutationReviewDowngradeTrigger::CommentDraftBoundaryMissing,
            WorkItemMutationReviewDowngradeTrigger::PublishModeAmbiguous,
            WorkItemMutationReviewDowngradeTrigger::OfflineContinuityStale,
            WorkItemMutationReviewDowngradeTrigger::RedactionOrExpiryMissing,
            WorkItemMutationReviewDowngradeTrigger::UpstreamDependencyNarrowed,
        ],
        consumer_surfaces: vec![
            WorkItemMutationReviewConsumerSurface::DesktopWorkItemDetail,
            WorkItemMutationReviewConsumerSurface::ReviewWorkspace,
            WorkItemMutationReviewConsumerSurface::CliHeadless,
            WorkItemMutationReviewConsumerSurface::Companion,
            WorkItemMutationReviewConsumerSurface::SupportExport,
            WorkItemMutationReviewConsumerSurface::BrowserHandoff,
        ],
        trust_review: WorkItemMutationReviewTrustReview {
            detail_headers_disclose_provider_boundary: true,
            detail_headers_disclose_canonical_identity: true,
            detail_headers_disclose_state_and_owner: true,
            detail_headers_disclose_sync_state_and_open_external: true,
            transition_sheets_disclose_side_effects: true,
            transition_sheets_disclose_authority_and_publish_mode: true,
            comment_reviews_split_local_draft_from_external_post: true,
            comment_reviews_disclose_visibility_and_notifications: true,
            offline_packets_preserve_redaction_and_expiry: true,
            offline_packets_do_not_claim_provider_acceptance: true,
            outage_or_policy_block_preserves_local_intent: true,
            stable_vocabulary_across_surfaces: true,
            downgrade_narrows_instead_of_hides: true,
            no_passive_inspection_external_publish: true,
        },
        consumer_projection: WorkItemMutationReviewConsumerProjection {
            desktop_detail_uses_stable_vocabulary: true,
            review_workspace_uses_stable_vocabulary: true,
            cli_headless_uses_stable_vocabulary: true,
            companion_uses_stable_vocabulary: true,
            support_export_uses_stable_vocabulary: true,
            browser_handoff_uses_stable_vocabulary: true,
            transition_sheet_shows_side_effect_preview: true,
            comment_review_shows_publish_mode_and_fallback: true,
            offline_packet_shows_expiry_and_redaction: true,
            blocked_or_offline_state_preserves_retry_or_export: true,
            label_for_unqualified: true,
        },
        proof_freshness: WorkItemMutationReviewProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-12T22:35:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T22:35:00Z".to_owned(),
    })
}

/// Reads and validates the checked-in packet export.
pub fn current_work_item_mutation_review_export(
) -> Result<WorkItemMutationReviewPacket, WorkItemMutationReviewArtifactError> {
    let packet: WorkItemMutationReviewPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5/ship_work_item_detail_headers_status_transition_sheets_comment_publish_review_and_offline_handoff_packets_with_side_effect_previews/support_export.json"
    )))
    .map_err(WorkItemMutationReviewArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(WorkItemMutationReviewArtifactError::Validation(violations))
    }
}

fn project_detail_header(detail: &WorkItemDetailRecord) -> WorkItemDetailHeaderRow {
    let current_state_summary = detail
        .current_state_rows
        .iter()
        .map(|row| row.summary.as_str())
        .collect::<Vec<_>>()
        .join(" · ");
    let owner_or_assignee_summary = if detail.owner_or_assignee_rows.is_empty() {
        "unassigned".to_owned()
    } else {
        detail
            .owner_or_assignee_rows
            .iter()
            .map(|row| row.actor_label.as_str())
            .collect::<Vec<_>>()
            .join(" · ")
    };
    let sync_state_summary = format!(
        "{} · {} · {}",
        row_posture_label(detail.row_posture_class),
        freshness_label(detail.freshness_class),
        publish_posture_label(detail.publish_posture())
    );

    WorkItemDetailHeaderRow {
        header_id: format!("header:{}", detail.detail_id),
        work_item_detail_record_id_ref: detail.detail_id.clone(),
        provider_family: detail.provider_family,
        provider_label: detail.provider_label.clone(),
        project_or_space_ref: detail.project_or_space_ref.clone(),
        canonical_id: detail.canonical_id.clone(),
        title_label: detail.title_label.clone(),
        provider_boundary_label: format!(
            "{} boundary · {}",
            provider_family_label(detail.provider_family),
            detail.provider_label
        ),
        current_state_summary,
        owner_or_assignee_summary,
        sync_state_summary,
        freshness_class: detail.freshness_class,
        write_authority_class: detail.write_authority_class,
        publish_posture_class: detail.publish_posture(),
        open_external_action_class: detail.open_external_action.action_class,
        open_external_action_label: detail.open_external_action.action_label.clone(),
        linked_transition_review_ids: detail.linked_status_transition_packet_record_id_refs.clone(),
        linked_offline_handoff_ids: detail.linked_offline_handoff_packet_record_id_refs.clone(),
        summary_label: detail.support_export_summary.clone(),
    }
}

fn project_transition_review_row(
    detail: &WorkItemDetailRecord,
    review: &aureline_provider::TransitionReviewSheetRecord,
) -> StatusTransitionReviewRow {
    StatusTransitionReviewRow {
        row_id: format!("transition-row:{}", review.review_id),
        transition_review_record_id_ref: review.review_id.clone(),
        work_item_detail_record_id_ref: review.work_item_detail_record_id_ref.clone(),
        status_transition_packet_record_id_ref: review
            .linked_status_transition_packet_record_id_ref
            .clone(),
        target_item_ref: review.target_item_ref.clone(),
        current_state_summary: review.current_state_summary.clone(),
        requested_state_summary: review.requested_state_summary.clone(),
        linked_branch_or_review_note: linked_branch_or_review_note(detail),
        actor_authority_summary: review.actor_authority_summary.clone(),
        permission_scope_class: review.permission_scope_class,
        publish_mode_class: review.publish_mode_class,
        disposition_summary_label: transition_permission_label(review.permission_scope_class)
            .to_owned(),
        confirm_action_available: review.action_affordances.confirm_action_available,
        export_action_available: review.action_affordances.export_action_available,
        cancel_action_available: review.action_affordances.cancel_action_available,
        side_effect_summaries: review
            .side_effect_fanout_rows
            .iter()
            .map(|row| row.summary.clone())
            .collect(),
        local_draft_preserved_on_failure: review.local_draft_preserved_on_failure,
        fallback_label: transition_fallback_label(review),
        summary_label: review.summary.clone(),
    }
}

fn project_comment_publish_review_row(
    detail: &WorkItemDetailRecord,
    review: &aureline_provider::PublishReviewRecord,
    comment_sync: Option<&aureline_provider::CommentSyncStateRecord>,
) -> CommentPublishReviewRow {
    let visibility_target_label = comment_visibility_target_label(review.publish_mode_class);
    let notify_behavior_label = if review.side_effect_rows.iter().any(|row| {
        matches!(
            row.side_effect_class,
            aureline_provider::PublishReviewSideEffectClass::NotificationEmissionFanout
        )
    }) {
        "provider notifications may fan out on publish".to_owned()
    } else {
        "no provider notification fanout disclosed".to_owned()
    };

    CommentPublishReviewRow {
        row_id: format!("comment-row:{}", review.publish_review_id),
        publish_review_record_id_ref: review.publish_review_id.clone(),
        work_item_detail_record_id_ref: review.work_item_detail_record_id_ref.clone(),
        comment_sync_state_record_id_ref: review.comment_sync_state_record_id_ref.clone(),
        status_transition_packet_record_id_ref: review.status_transition_packet_record_id_ref.clone(),
        action_class: review.publish_review_action_class,
        publish_mode_class: review.publish_mode_class,
        disposition_class: review.publish_review_disposition_class,
        actor_scope_class: review.publish_review_actor_scope_class,
        local_draft_summary: comment_sync
            .map(|row| row.support_export_summary.clone())
            .unwrap_or_else(|| "local draft preserved for review".to_owned()),
        external_post_summary: publish_review_disposition_label(
            review.publish_review_disposition_class,
            review.publish_mode_class,
        )
        .to_owned(),
        visibility_target_label,
        evidence_refs: detail
            .engineering_artifact_relations
            .linked_validation_evidence_record_id_refs
            .clone(),
        notify_behavior_label,
        fallback_label: comment_fallback_label(review),
        side_effect_summaries: review
            .side_effect_rows
            .iter()
            .map(|row| row.summary.clone())
            .collect(),
        summary_label: review.summary.clone(),
    }
}

fn project_offline_handoff_row(
    detail: &WorkItemDetailRecord,
    packet: &aureline_provider::OfflineHandoffPacketRecord,
) -> OfflineHandoffPacketRow {
    let code_link_refs = packet
        .snapshot_engineering_relations
        .iter()
        .filter(|relation| {
            matches!(
                relation.relation_axis_class,
                aureline_provider::SnapshotRelationAxisClass::IssueToBranchLink
                    | aureline_provider::SnapshotRelationAxisClass::LinkedReview
            )
        })
        .map(|relation| relation.linked_record_id_ref.clone())
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    let mut evidence_refs = detail
        .engineering_artifact_relations
        .linked_validation_evidence_record_id_refs
        .clone();
    evidence_refs.extend(
        packet
            .snapshot_engineering_relations
            .iter()
            .filter(|relation| {
                relation.relation_axis_class
                    == aureline_provider::SnapshotRelationAxisClass::ValidationEvidence
            })
            .map(|relation| relation.linked_record_id_ref.clone())
            .filter(|value| !value.trim().is_empty()),
    );
    evidence_refs.sort();
    evidence_refs.dedup();

    OfflineHandoffPacketRow {
        row_id: format!("handoff-row:{}", packet.packet_id),
        offline_handoff_packet_record_id_ref: packet.packet_id.clone(),
        work_item_detail_record_id_ref: packet.work_item_detail_record_id_ref.clone(),
        status_transition_packet_record_id_ref: packet
            .linked_status_transition_packet_record_id_ref
            .clone(),
        admission_reason_class: packet.handoff_admission_reason_class,
        provider_acceptance_class: packet.handoff_provider_acceptance_class,
        drain_state_class: packet.handoff_drain_state_class,
        export_route_classes: packet.handoff_export_route_classes.clone(),
        retry_route_classes: packet.handoff_retry_route_classes.clone(),
        captured_note_summary: packet.summary.clone(),
        code_link_refs,
        evidence_refs,
        redaction_class: packet.redaction_class,
        redaction_manifest_ref: packet.redaction_manifest_ref.clone(),
        expires_at: packet.expires_at.clone(),
        publish_target_ref: packet.publish_target_ref.clone(),
        retry_action_ref: packet.retry_action_ref.clone(),
        export_action_ref: packet.export_action_ref.clone(),
        packet_survives_restart: packet.packet_survives_restart,
        summary_label: format!(
            "{} · {}",
            detail.canonical_id, packet.summary
        ),
    }
}

fn validate_source_contracts(
    packet: &WorkItemMutationReviewPacket,
    violations: &mut Vec<WorkItemMutationReviewViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        WORK_ITEM_MUTATION_REVIEW_SCHEMA_REF,
        WORK_ITEM_MUTATION_REVIEW_DOC_REF,
        WORK_ITEM_MUTATION_REVIEW_GOVERNANCE_CONTRACT_REF,
        WORK_ITEM_MUTATION_REVIEW_DETAIL_CONTRACT_REF,
        WORK_ITEM_MUTATION_REVIEW_TRANSITION_CONTRACT_REF,
        WORK_ITEM_MUTATION_REVIEW_COMMENT_CONTRACT_REF,
        WORK_ITEM_MUTATION_REVIEW_HANDOFF_CONTRACT_REF,
        WORK_ITEM_MUTATION_REVIEW_BROWSER_HANDOFF_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(WorkItemMutationReviewViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_detail_headers(
    packet: &WorkItemMutationReviewPacket,
    violations: &mut Vec<WorkItemMutationReviewViolation>,
) {
    if packet.detail_headers.is_empty() {
        violations.push(WorkItemMutationReviewViolation::DetailHeadersMissing);
        return;
    }

    for row in &packet.detail_headers {
        if row.header_id.trim().is_empty()
            || row.work_item_detail_record_id_ref.trim().is_empty()
            || row.provider_label.trim().is_empty()
            || row.canonical_id.trim().is_empty()
            || row.provider_boundary_label.trim().is_empty()
            || row.current_state_summary.trim().is_empty()
            || row.owner_or_assignee_summary.trim().is_empty()
            || row.sync_state_summary.trim().is_empty()
            || row.open_external_action_label.trim().is_empty()
            || row.summary_label.trim().is_empty()
        {
            violations.push(WorkItemMutationReviewViolation::DetailHeaderIncomplete);
            return;
        }
    }
}

fn validate_transition_reviews(
    packet: &WorkItemMutationReviewPacket,
    violations: &mut Vec<WorkItemMutationReviewViolation>,
) {
    if packet.transition_reviews.is_empty() {
        violations.push(WorkItemMutationReviewViolation::TransitionReviewsMissing);
        return;
    }

    let detail_refs: BTreeSet<&str> = packet
        .detail_headers
        .iter()
        .map(|row| row.work_item_detail_record_id_ref.as_str())
        .collect();

    for row in &packet.transition_reviews {
        if !detail_refs.contains(row.work_item_detail_record_id_ref.as_str()) {
            violations.push(WorkItemMutationReviewViolation::OrphanTransitionReference);
            return;
        }
        if row.row_id.trim().is_empty()
            || row.transition_review_record_id_ref.trim().is_empty()
            || row.status_transition_packet_record_id_ref.trim().is_empty()
            || row.target_item_ref.trim().is_empty()
            || row.current_state_summary.trim().is_empty()
            || row.requested_state_summary.trim().is_empty()
            || row.linked_branch_or_review_note.trim().is_empty()
            || row.actor_authority_summary.trim().is_empty()
            || row.disposition_summary_label.trim().is_empty()
            || row.side_effect_summaries.is_empty()
            || row.fallback_label.trim().is_empty()
            || row.summary_label.trim().is_empty()
            || !row.cancel_action_available
        {
            violations.push(WorkItemMutationReviewViolation::TransitionReviewIncomplete);
            return;
        }
    }
}

fn validate_comment_publish_reviews(
    packet: &WorkItemMutationReviewPacket,
    violations: &mut Vec<WorkItemMutationReviewViolation>,
) {
    if packet.comment_publish_reviews.is_empty() {
        violations.push(WorkItemMutationReviewViolation::CommentPublishReviewsMissing);
        return;
    }

    let detail_refs: BTreeSet<&str> = packet
        .detail_headers
        .iter()
        .map(|row| row.work_item_detail_record_id_ref.as_str())
        .collect();

    for row in &packet.comment_publish_reviews {
        if !detail_refs.contains(row.work_item_detail_record_id_ref.as_str()) {
            violations.push(WorkItemMutationReviewViolation::OrphanCommentReference);
            return;
        }
        if !row.action_class.is_comment_action() {
            violations.push(WorkItemMutationReviewViolation::CommentPublishReviewNotCommentAction);
            return;
        }
        if row.row_id.trim().is_empty()
            || row.publish_review_record_id_ref.trim().is_empty()
            || row.local_draft_summary.trim().is_empty()
            || row.external_post_summary.trim().is_empty()
            || row.visibility_target_label.trim().is_empty()
            || row.notify_behavior_label.trim().is_empty()
            || row.fallback_label.trim().is_empty()
            || row.side_effect_summaries.is_empty()
            || row.summary_label.trim().is_empty()
        {
            violations.push(WorkItemMutationReviewViolation::CommentPublishReviewIncomplete);
            return;
        }
    }
}

fn validate_offline_handoff_packets(
    packet: &WorkItemMutationReviewPacket,
    violations: &mut Vec<WorkItemMutationReviewViolation>,
) {
    if packet.offline_handoff_packets.is_empty() {
        violations.push(WorkItemMutationReviewViolation::OfflineHandoffPacketsMissing);
        return;
    }

    let detail_refs: BTreeSet<&str> = packet
        .detail_headers
        .iter()
        .map(|row| row.work_item_detail_record_id_ref.as_str())
        .collect();

    for row in &packet.offline_handoff_packets {
        if !detail_refs.contains(row.work_item_detail_record_id_ref.as_str()) {
            violations.push(WorkItemMutationReviewViolation::OrphanOfflineHandoffReference);
            return;
        }
        if matches!(
            row.provider_acceptance_class,
            HandoffProviderAcceptanceClass::ProviderAcceptConfirmedPublishLaterDrained
                | HandoffProviderAcceptanceClass::ProviderAcceptRejectedWithTypedReason
        ) {
            violations.push(WorkItemMutationReviewViolation::OfflineHandoffClaimsProviderAcceptance);
            return;
        }
        if row.row_id.trim().is_empty()
            || row.offline_handoff_packet_record_id_ref.trim().is_empty()
            || row.status_transition_packet_record_id_ref.trim().is_empty()
            || row.captured_note_summary.trim().is_empty()
            || row.redaction_manifest_ref.trim().is_empty()
            || row.expires_at.trim().is_empty()
            || row.publish_target_ref.trim().is_empty()
            || row.retry_action_ref.trim().is_empty()
            || row.export_action_ref.trim().is_empty()
            || row.export_route_classes.is_empty()
            || row.retry_route_classes.is_empty()
            || !row.packet_survives_restart
            || row.summary_label.trim().is_empty()
        {
            violations.push(WorkItemMutationReviewViolation::OfflineHandoffPacketIncomplete);
            return;
        }
    }
}

fn linked_branch_or_review_note(detail: &WorkItemDetailRecord) -> String {
    let branch = detail
        .engineering_artifact_relations
        .linked_branch_local_locator_ref
        .as_deref()
        .unwrap_or("no linked branch");
    let review = detail
        .engineering_artifact_relations
        .linked_review_workspace_record_id_ref
        .as_deref()
        .unwrap_or("no linked review");
    format!("branch {branch} · review {review}")
}

fn transition_fallback_label(review: &aureline_provider::TransitionReviewSheetRecord) -> String {
    if review.linked_offline_handoff_packet_record_id_ref.is_some() {
        "save local packet if publish fails".to_owned()
    } else if review.linked_publish_later_queue_item_record_id_ref.is_some() {
        "queue for publish later and preserve the local draft".to_owned()
    } else if review.local_draft_preserved_on_failure {
        "preserve the local draft on failure".to_owned()
    } else {
        review
            .block_reason_summary
            .clone()
            .unwrap_or_else(|| "no local fallback disclosed".to_owned())
    }
}

fn comment_fallback_label(review: &aureline_provider::PublishReviewRecord) -> String {
    if review.linked_offline_handoff_packet_record_id_ref.is_some() {
        "offline handoff packet keeps the comment draft".to_owned()
    } else if review.linked_publish_later_queue_item_record_id_ref.is_some() {
        "queue the comment for publish later".to_owned()
    } else if review.local_draft_preserved_on_failure {
        "preserve the local draft on failure".to_owned()
    } else if let Some(reason) = &review.block_reason_summary {
        reason.clone()
    } else {
        "no local fallback disclosed".to_owned()
    }
}

fn provider_family_label(family: ProviderFamily) -> &'static str {
    match family {
        ProviderFamily::CodeHost => "code_host",
        ProviderFamily::IssueTracker => "issue_tracker",
        ProviderFamily::CiChecks => "ci_checks",
    }
}

fn work_item_mutation_mode_label(mode: WorkItemMutationMode) -> &'static str {
    match mode {
        WorkItemMutationMode::LocalDraft => "local_draft",
        WorkItemMutationMode::PublishNow => "publish_now",
        WorkItemMutationMode::OpenInProvider => "open_in_provider",
        WorkItemMutationMode::DeferredPublish => "deferred_publish",
        WorkItemMutationMode::InspectOnly => "inspect_only",
    }
}

fn publish_review_disposition_label(
    disposition: PublishReviewDispositionClass,
    mode: WorkItemMutationMode,
) -> &'static str {
    match disposition {
        PublishReviewDispositionClass::AdmissibleNowPublishNow => "publish now to the provider",
        PublishReviewDispositionClass::AdmissibleViaQueueForPublishLater => {
            "queue for publish later"
        }
        PublishReviewDispositionClass::AdmissibleViaBrowserHandoffOnly => {
            "route through browser handoff"
        }
        PublishReviewDispositionClass::AdmissibleLocalDraftOnly => "save as a local draft only",
        PublishReviewDispositionClass::AdmissibleViaRetryAfterConflict => {
            "retry after conflict resolution"
        }
        PublishReviewDispositionClass::BlockedPendingConflictResolution => {
            "blocked pending conflict resolution"
        }
        PublishReviewDispositionClass::BlockedProviderReadOnly => "blocked by provider read-only scope",
        PublishReviewDispositionClass::BlockedByPolicy => "blocked by policy",
        PublishReviewDispositionClass::BlockedFreshnessFloorUnsatisfied => {
            "blocked by freshness floor"
        }
        PublishReviewDispositionClass::BlockedAccountMappingPending => {
            if mode == WorkItemMutationMode::DeferredPublish {
                "queued intent held pending account mapping"
            } else {
                "blocked pending account mapping"
            }
        }
    }
}

fn comment_visibility_target_label(mode: WorkItemMutationMode) -> String {
    match mode {
        WorkItemMutationMode::LocalDraft => "local draft only".to_owned(),
        WorkItemMutationMode::PublishNow => "provider work-item timeline".to_owned(),
        WorkItemMutationMode::OpenInProvider => "provider timeline via browser handoff".to_owned(),
        WorkItemMutationMode::DeferredPublish => "provider timeline via publish-later queue".to_owned(),
        WorkItemMutationMode::InspectOnly => "inspect only".to_owned(),
    }
}

fn transition_permission_label(scope: PermissionScopeClass) -> &'static str {
    match scope {
        PermissionScopeClass::PermissionAdmissibleUserWritesProvider => "user may write provider state",
        PermissionScopeClass::PermissionAdmissibleAssigneeOnly => "assignee-only provider scope",
        PermissionScopeClass::PermissionAdmissibleUnderInstallOrAppGrant => {
            "install or app grant admits the write"
        }
        PermissionScopeClass::PermissionAdmissibleUnderBrowserHandoffOnly => {
            "browser handoff is the admitted write path"
        }
        PermissionScopeClass::PermissionAdmissibleUnderManagedAdminOnly => {
            "managed admin route is the admitted write path"
        }
        PermissionScopeClass::PermissionAdmissibleUnderLocalDraftOnly => {
            "local draft is the admitted path"
        }
        PermissionScopeClass::PermissionBlockedProviderReadOnlyScope => {
            "provider grant is read only"
        }
        PermissionScopeClass::PermissionBlockedActorClassForbidden => "actor class is blocked",
        PermissionScopeClass::PermissionBlockedWorkspaceTrustUnsetOrRestricted => {
            "workspace trust blocks the write"
        }
        PermissionScopeClass::PermissionUnknownPendingActorResolution => {
            "permission is pending actor resolution"
        }
    }
}

fn row_posture_label(posture: WorkItemRowPostureClass) -> &'static str {
    match posture {
        WorkItemRowPostureClass::ProviderAuthoritative => "provider_authoritative",
        WorkItemRowPostureClass::CachedStale => "cached_stale",
        WorkItemRowPostureClass::ReadOnly => "read_only",
        WorkItemRowPostureClass::PolicyBlocked => "policy_blocked",
        WorkItemRowPostureClass::LocalDraft => "local_draft",
        WorkItemRowPostureClass::Queued => "queued",
        WorkItemRowPostureClass::OfflineCaptured => "offline_captured",
    }
}

fn freshness_label(freshness: WorkItemFreshnessClass) -> &'static str {
    match freshness {
        WorkItemFreshnessClass::LiveAuthoritativeFresh => "live_authoritative_fresh",
        WorkItemFreshnessClass::WarmWithinGrace => "warm_within_grace",
        WorkItemFreshnessClass::DegradedBeyondGraceLocalContinues => {
            "degraded_beyond_grace_local_continues"
        }
        WorkItemFreshnessClass::UnverifiableProviderUnreachable => {
            "unverifiable_provider_unreachable"
        }
        WorkItemFreshnessClass::ImportedSnapshotNoRefreshPath => {
            "imported_snapshot_no_refresh_path"
        }
        WorkItemFreshnessClass::LocalDraftNeverPublished => "local_draft_never_published",
    }
}

fn publish_posture_label(posture: WorkItemPublishPostureClass) -> &'static str {
    match posture {
        WorkItemPublishPostureClass::PublishedObservedAuthoritative => {
            "published_observed_authoritative"
        }
        WorkItemPublishPostureClass::PublishNowPendingReview => "publish_now_pending_review",
        WorkItemPublishPostureClass::DraftOnly => "draft_only",
        WorkItemPublishPostureClass::QueuedForPublishLater => "queued_for_publish_later",
        WorkItemPublishPostureClass::OfflineCapturedNotSubmitted => {
            "offline_captured_not_submitted"
        }
        WorkItemPublishPostureClass::ImportedEvidenceOnly => "imported_evidence_only",
        WorkItemPublishPostureClass::InspectOnly => "inspect_only",
    }
}

fn handoff_provider_acceptance_label(acceptance: HandoffProviderAcceptanceClass) -> &'static str {
    match acceptance {
        HandoffProviderAcceptanceClass::NotSubmittedLocalCaptureOnly => {
            "not_submitted_local_capture_only"
        }
        HandoffProviderAcceptanceClass::SubmittedPendingProviderAcceptUnverified => {
            "submitted_pending_provider_accept_unverified"
        }
        HandoffProviderAcceptanceClass::ProviderAcceptConfirmedPublishLaterDrained => {
            "provider_accept_confirmed_publish_later_drained"
        }
        HandoffProviderAcceptanceClass::ProviderAcceptRejectedWithTypedReason => {
            "provider_accept_rejected_with_typed_reason"
        }
        HandoffProviderAcceptanceClass::ImportedHandoffEvidenceOnlyNoProviderPath => {
            "imported_handoff_evidence_only_no_provider_path"
        }
    }
}

fn handoff_drain_state_label(state: HandoffDrainStateClass) -> &'static str {
    match state {
        HandoffDrainStateClass::CapturedPendingDrain => "captured_pending_drain",
        HandoffDrainStateClass::CapturedPendingExportUserInitiated => {
            "captured_pending_export_user_initiated"
        }
        HandoffDrainStateClass::ExportedPendingExternalApply => {
            "exported_pending_external_apply"
        }
        HandoffDrainStateClass::DrainAdmittedToPublishLaterQueue => {
            "drain_admitted_to_publish_later_queue"
        }
        HandoffDrainStateClass::DrainedPublishLaterCompleted => {
            "drained_publish_later_completed"
        }
        HandoffDrainStateClass::DrainedPublishLaterRejectedWithTypedReason => {
            "drained_publish_later_rejected_with_typed_reason"
        }
        HandoffDrainStateClass::RevokedByUserBeforeDrain => "revoked_by_user_before_drain",
        HandoffDrainStateClass::SupersededByHandoffPacket => "superseded_by_handoff_packet",
    }
}

fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret ")
                || lower.contains("bearer ")
                || lower.contains("http://")
                || lower.contains("https://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
