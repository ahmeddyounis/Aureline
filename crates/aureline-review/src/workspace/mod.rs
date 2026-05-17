//! Review-workspace seed packets and beta packets for local diff discussion.
//!
//! This module consumes the local Git review seed and diff-view packets, then
//! materializes the first review-workspace packet with deterministic row anchor
//! IDs and work-item relation rows. It keeps provider overlay fields out of the
//! anchor hash so the same anchors can later be published to hosted review
//! providers without changing their meaning.

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use aureline_graph::GraphFactCuePacket;
use aureline_search::{
    SearchOperatorConsumerSurface, SearchOperatorTruthFinding, SearchOperatorTruthPacket,
    SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION,
};

use crate::diff::{DiffLineKind, DiffLineView, DiffViewSurfacePacket};

/// Stable record-kind tag for [`ReviewWorkspaceSeedPacket`].
pub const REVIEW_WORKSPACE_SEED_PACKET_RECORD_KIND: &str = "review_workspace_seed_packet";

/// Stable record-kind tag for [`ReviewWorkspaceRecord`].
pub const REVIEW_WORKSPACE_RECORD_KIND: &str = "review_workspace_record";

/// Stable record-kind tag for [`ReviewAnchorIdAlphaRecord`].
pub const REVIEW_ANCHOR_ID_ALPHA_RECORD_KIND: &str = "review_anchor_id_alpha_record";

/// Stable record-kind tag for [`ReviewWorkItemLinkageRecord`].
pub const REVIEW_WORK_ITEM_LINKAGE_RECORD_KIND: &str = "review_work_item_linkage_record";

/// Stable record-kind tag for [`ReviewWorkspaceInspectionRecord`].
pub const REVIEW_WORKSPACE_INSPECTION_RECORD_KIND: &str = "review_workspace_inspection_record";

/// Stable record-kind tag for [`ReviewWorkspaceBetaPacket`].
pub const REVIEW_WORKSPACE_BETA_PACKET_RECORD_KIND: &str = "review_workspace_beta_packet";

/// Stable record-kind tag for [`ReviewWorkspaceDurableCommentAnchorRecord`].
pub const REVIEW_WORKSPACE_DURABLE_COMMENT_ANCHOR_RECORD_KIND: &str =
    "review_workspace_durable_comment_anchor_record";

/// Stable record-kind tag for [`ReviewWorkspaceObjectLineageRecord`].
pub const REVIEW_WORKSPACE_OBJECT_LINEAGE_RECORD_KIND: &str =
    "review_workspace_object_lineage_record";

/// Stable record-kind tag for [`ReviewWorkspaceCheckFreshnessRecord`].
pub const REVIEW_WORKSPACE_CHECK_FRESHNESS_RECORD_KIND: &str =
    "review_workspace_check_freshness_record";

/// Stable record-kind tag for [`ReviewWorkspaceBrowserHandoffRecord`].
pub const REVIEW_WORKSPACE_BROWSER_HANDOFF_RECORD_KIND: &str =
    "review_workspace_browser_handoff_record";

/// Stable record-kind tag for [`ReviewWorkspaceSupportExportPacket`].
pub const REVIEW_WORKSPACE_SUPPORT_EXPORT_PACKET_RECORD_KIND: &str =
    "review_workspace_support_export_packet";

/// Stable record-kind tag for [`ReviewWorkspaceBetaInspectionRecord`].
pub const REVIEW_WORKSPACE_BETA_INSPECTION_RECORD_KIND: &str =
    "review_workspace_beta_inspection_record";

/// Stable record-kind tag for [`ReviewWorkspaceSearchOperatorTruthExport`].
pub const REVIEW_WORKSPACE_SEARCH_OPERATOR_TRUTH_EXPORT_RECORD_KIND: &str =
    "review_workspace_search_operator_truth_export_record";

/// Schema version for beta review-workspace packets.
pub const REVIEW_WORKSPACE_BETA_SCHEMA_VERSION: u32 = 1;

/// Closed set of beta anchor drift states.
pub const REVIEW_WORKSPACE_BETA_ANCHOR_DRIFT_STATES: &[&str] = &[
    "anchor_bound_exact",
    "anchor_remapped_with_recorded_mapping",
    "anchor_drifted_user_must_resolve",
    "anchor_target_deleted_re_anchor_or_resolve",
    "anchor_scope_unavailable",
    "anchor_archived_tombstone",
];

/// Closed set of beta anchor required-action tokens.
pub const REVIEW_WORKSPACE_BETA_ANCHOR_REQUIRED_ACTIONS: &[&str] = &[
    "no_user_action_required_anchor_bound_or_remapped",
    "user_must_pick_successor_or_dismiss_drifted",
    "user_must_re_anchor_or_resolve_deleted_target",
    "user_must_widen_scope_or_load_pack_or_reach_remote",
    "user_must_restore_from_archive_or_acknowledge_tombstone",
];

/// Closed set of per-anchor local/provider freshness classes.
pub const REVIEW_WORKSPACE_BETA_ANCHOR_FRESHNESS_CLASSES: &[&str] = &[
    "local_only_no_provider_overlay",
    "local_and_provider_match_fresh",
    "local_and_provider_match_stale_within_grace",
    "local_and_provider_disagree_user_review_required",
    "provider_overlay_unavailable_local_continues",
];

/// Closed set of review check status classes.
pub const REVIEW_WORKSPACE_BETA_CHECK_STATUS_CLASSES: &[&str] = &[
    "check_passed",
    "check_failed",
    "check_not_evaluated_on_this_surface",
    "check_blocked_by_stale_context",
];

/// Closed set of review check freshness classes.
pub const REVIEW_WORKSPACE_BETA_CHECK_FRESHNESS_CLASSES: &[&str] = &[
    "check_current",
    "check_stale_within_grace",
    "check_stale_blocks_operator_truth",
    "check_unavailable_blocks_operator_truth",
];

/// Closed set of review check authority classes.
pub const REVIEW_WORKSPACE_BETA_CHECK_AUTHORITY_CLASSES: &[&str] = &[
    "local_review_pack",
    "ci_provider_overlay",
    "ai_evidence_packet",
    "imported_review_bundle",
];

/// Closed set of browser-handoff destination classes used by review workspaces.
pub const REVIEW_WORKSPACE_BETA_HANDOFF_DESTINATION_CLASSES: &[&str] = &[
    "code_host_web",
    "issue_tracker_web",
    "ci_provider_web",
    "docs_or_portal_web",
    "managed_admin_web",
    "external_generic_web",
];

/// Closed set of browser-handoff reason codes used by review workspaces.
pub const REVIEW_WORKSPACE_BETA_HANDOFF_REASON_CODES: &[&str] = &[
    "mutation_not_supported_in_product",
    "publish_requires_browser_auth",
    "external_docs_or_runbook",
    "provider_consent_flow",
    "step_up_required",
];

/// Closed set of browser-handoff replay posture values.
pub const REVIEW_WORKSPACE_BETA_HANDOFF_REPLAY_POSTURES: &[&str] =
    &["single_use", "bounded_reuse", "read_only_resumable"];

/// Closed set of consumer surfaces for the beta packet and support export.
pub const REVIEW_WORKSPACE_BETA_CONSUMER_SURFACES: &[&str] = &[
    "review_workspace_inspector",
    "review_preview",
    "cli_headless_entry",
    "support_export",
    "docs_review",
    "browser_companion",
];

const REVIEW_WORKSPACE_SEED_PACKET_SCHEMA_VERSION: u32 = 1;
const REVIEW_WORKSPACE_SCHEMA_VERSION: u32 = 1;
const REVIEW_ANCHOR_ID_ALPHA_SCHEMA_VERSION: u32 = 1;
const REVIEW_WORK_ITEM_LINKAGE_SCHEMA_VERSION: u32 = 1;
const REVIEW_WORKSPACE_INSPECTION_SCHEMA_VERSION: u32 = 1;

/// Input fields used to seed a review workspace from a local Git review seed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceSeedInput {
    /// Stable review workspace identity.
    pub review_workspace_id: String,
    /// Local branch, worktree, or source-locator ref backing the workspace.
    pub branch_or_worktree_ref: String,
    /// Optional base revision ref used by hosted providers and drift checks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_revision_ref: Option<String>,
    /// Optional head revision ref used by hosted providers and drift checks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub head_revision_ref: Option<String>,
    /// Actor that opened or refreshed the seed.
    pub actor_ref: String,
    /// Policy epoch copied into workspace, anchor, and linkage records.
    pub policy_epoch: String,
    /// Trust state copied into the policy context.
    pub trust_state: String,
    /// Optional execution-context ref for support and replay joins.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<String>,
    /// Client scopes admitted for the seed.
    pub client_scopes: Vec<String>,
    /// Timestamp used for deterministic fixture output.
    pub created_at: String,
    /// Optional provider overlay attached to the same local workspace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_overlay: Option<ReviewProviderOverlayInput>,
    /// Work-item relations projected into the review workspace.
    #[serde(default)]
    pub work_item_links: Vec<ReviewWorkItemLinkInput>,
    /// Graph readiness cues inherited by the review surface.
    #[serde(default)]
    pub graph_cue_packets: Vec<GraphFactCuePacket>,
}

impl ReviewWorkspaceSeedInput {
    /// Builds a review-workspace input from the Git review seed projection.
    pub fn from_git_review_seed(
        seed: &aureline_git::GitReviewSeedRecord,
        actor_ref: impl Into<String>,
        policy_epoch: impl Into<String>,
        trust_state: impl Into<String>,
        created_at: impl Into<String>,
    ) -> Self {
        let review_workspace_id = seed.review_workspace_ref.clone();
        Self {
            branch_or_worktree_ref: review_workspace_id.replace("review.git.", "worktree.git."),
            review_workspace_id,
            base_revision_ref: seed.base_revision_ref.clone(),
            head_revision_ref: seed.head_revision_ref.clone(),
            actor_ref: actor_ref.into(),
            policy_epoch: policy_epoch.into(),
            trust_state: trust_state.into(),
            execution_context_id: None,
            client_scopes: vec!["desktop_product".to_string(), "cli".to_string()],
            created_at: created_at.into(),
            provider_overlay: None,
            work_item_links: Vec::new(),
            graph_cue_packets: Vec::new(),
        }
    }

    /// Returns a copy with a provider overlay attached.
    pub fn with_provider_overlay(mut self, provider_overlay: ReviewProviderOverlayInput) -> Self {
        self.provider_overlay = Some(provider_overlay);
        self
    }

    /// Returns a copy with an inherited graph cue packet attached.
    pub fn with_graph_cue_packet(mut self, graph_cue_packet: GraphFactCuePacket) -> Self {
        self.graph_cue_packets.push(graph_cue_packet);
        self
    }
}

/// Provider overlay input that must not affect anchor identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewProviderOverlayInput {
    /// Provider class token from the review-workspace contract.
    pub provider_class: String,
    /// Opaque connected-provider ref.
    pub connected_provider_record_id_ref: String,
    /// Opaque provider object identity ref.
    pub provider_object_identity_ref: String,
    /// Provider overlay freshness token.
    pub provider_overlay_freshness_class: String,
    /// Timestamp of the provider fetch that produced this overlay.
    pub last_fetched_at: String,
    /// Optional freshness grace window in seconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grace_window_seconds: Option<u32>,
}

/// Work-item relation input projected into the review workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkItemLinkInput {
    /// Opaque work-item detail record ref.
    pub work_item_detail_record_id_ref: String,
    /// Opaque provider object identity ref for the issue or work item.
    pub target_object_identity_ref: String,
    /// Work-item authority class copied from the work-item contract.
    pub work_item_authority_class: String,
    /// Write-authority class copied from the work-item contract.
    pub write_authority_class: String,
    /// Issue-to-branch relation class copied from the work-item contract.
    pub issue_to_branch_link_class: String,
    /// Actor that created or refreshed the relation.
    pub actor_ref: String,
    /// Command id that created or refreshed the relation.
    pub command_id_ref: String,
    /// Timestamp the relation was linked.
    pub linked_at: String,
    /// Reviewable relation summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Review-workspace record mirroring the VCS review-workspace contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the review-workspace contract.
    pub review_workspace_schema_version: u32,
    /// Stable review workspace identity.
    pub review_workspace_id: String,
    /// Source class token for the workspace.
    pub review_workspace_source_class: String,
    /// Provider authority token for the workspace.
    pub provider_authority_class: String,
    /// Lifecycle state token for the workspace.
    pub review_workspace_lifecycle_state: String,
    /// Local branch or worktree locator.
    pub local_locator: Option<ReviewLocalLocator>,
    /// Provider overlay block, when available.
    pub provider_overlay: Option<ReviewProviderOverlay>,
    /// Imported bundle envelope; always `None` for this alpha seed.
    pub imported_bundle_envelope: Option<String>,
    /// Browser handoff envelope; always `None` for this alpha seed.
    pub browser_handoff_envelope: Option<String>,
    /// Policy context copied into the seed.
    pub policy_context: ReviewPolicyContext,
    /// Client scopes admitted for this workspace.
    pub client_scopes: Vec<String>,
    /// Redaction class for exported workspace metadata.
    pub redaction_class: String,
    /// Workspace-level freshness class.
    pub freshness_class: String,
    /// Reviewable summary label.
    pub summary_label: String,
    /// Creation timestamp.
    pub created_at: String,
    /// Last update timestamp.
    pub updated_at: String,
    /// Archive timestamp, when retained as a tombstone.
    pub archived_at: Option<String>,
    /// Hosted-review inbox ref reserved for later provider integration.
    pub hosted_review_inbox_record_id_ref: Option<String>,
    /// Merge policy ref reserved for later provider integration.
    pub merge_policy_record_id_ref: Option<String>,
}

/// Local locator for a review workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewLocalLocator {
    /// Opaque workspace ref.
    pub workspace_id_ref: String,
    /// Opaque branch or worktree ref.
    pub branch_or_worktree_ref: String,
    /// Optional base revision ref.
    pub base_revision_ref: Option<String>,
    /// Optional head revision ref.
    pub head_revision_ref: Option<String>,
}

/// Provider overlay block for a review workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewProviderOverlay {
    /// Provider class token.
    pub provider_class: String,
    /// Opaque connected-provider ref.
    pub connected_provider_record_id_ref: String,
    /// Opaque provider object identity ref.
    pub provider_object_identity_ref: String,
    /// Provider overlay freshness token.
    pub provider_overlay_freshness_class: String,
    /// Timestamp of the provider fetch.
    pub last_fetched_at: String,
    /// Optional freshness grace window in seconds.
    pub grace_window_seconds: Option<u32>,
}

/// Policy context shared by workspace, anchor, and linkage records.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewPolicyContext {
    /// Policy epoch used to mint the record.
    pub policy_epoch: String,
    /// Workspace trust state token.
    pub trust_state: String,
    /// Optional execution-context ref.
    pub execution_context_id: Option<String>,
    /// Workspace trust state class token.
    pub workspace_trust_state_class: String,
}

/// One diff opened inside the review workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceDiffEntry {
    /// Diff surface ref opened inside the review workspace.
    pub diff_surface_ref: String,
    /// Change-list row or launcher ref that opened the diff.
    pub launch_source_ref: String,
    /// Path-truth ref from the diff packet.
    pub path_truth_ref: String,
    /// Compare-target ref from the diff packet.
    pub compare_target_ref: String,
    /// Visible path label.
    pub path_label: String,
    /// Visible compare target label.
    pub compare_target_label: String,
    /// Anchor ids materialized for protected diff rows.
    pub anchor_id_refs: Vec<String>,
}

/// Stable alpha anchor identity for one protected diff row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAnchorIdAlphaRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha anchor contract.
    pub schema_version: u32,
    /// Stable anchor id.
    pub anchor_id: String,
    /// Review workspace this anchor belongs to.
    pub review_workspace_id_ref: String,
    /// Diff surface that rendered this target.
    pub diff_surface_ref: String,
    /// Diff row ref that owns this anchor target.
    pub target_ref: String,
    /// Path-truth ref from the diff packet.
    pub path_truth_ref: String,
    /// Compare-target ref from the diff packet.
    pub compare_target_ref: String,
    /// Anchor target kind copied from the review-anchor vocabulary.
    pub anchor_target_kind: String,
    /// Review artifact class copied from the review-anchor vocabulary.
    pub review_artifact_class: String,
    /// Review surface class copied from the review-anchor vocabulary.
    pub review_surface_class: String,
    /// Diff line kind token.
    pub line_kind: String,
    /// Old-side line number, when present.
    pub old_line_number: Option<u32>,
    /// New-side line number, when present.
    pub new_line_number: Option<u32>,
    /// Deterministic hash of the context used for drift checks.
    pub fallback_context_hash: String,
    /// Fields that define the stable anchor hash.
    pub stable_identity_fields: Vec<String>,
    /// True when provider object ids are excluded from the anchor hash.
    pub provider_excluded_from_anchor_hash: bool,
    /// Initial drift state copied from the review-anchor vocabulary.
    pub anchor_drift_state: String,
    /// Initial required action copied from the review-anchor vocabulary.
    pub anchor_drift_required_user_action: String,
    /// Human-readable summary for inspection surfaces.
    pub summary_label: String,
}

/// Work-item relation row projected from the review workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkItemLinkageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the linkage projection.
    pub schema_version: u32,
    /// Stable linkage identity.
    pub linkage_ref: String,
    /// Opaque work-item detail record ref.
    pub work_item_detail_record_id_ref: String,
    /// Opaque provider object identity ref.
    pub target_object_identity_ref: String,
    /// Review workspace linked to the work item.
    pub linked_review_workspace_record_id_ref: String,
    /// Anchors attached to this work-item relation.
    pub linked_review_anchor_id_refs: Vec<String>,
    /// Work-item authority class copied from the work-item contract.
    pub work_item_authority_class: String,
    /// Write-authority class copied from the work-item contract.
    pub write_authority_class: String,
    /// Issue-to-branch relation class copied from the work-item contract.
    pub issue_to_branch_link_class: String,
    /// Linked-review class copied from the work-item contract.
    pub linked_review_class: String,
    /// Actor that created or refreshed the relation.
    pub actor_ref: String,
    /// Command id that created or refreshed the relation.
    pub command_id_ref: String,
    /// Timestamp the relation was linked.
    pub linked_at: String,
    /// Source schemas this projection consumes.
    pub source_schema_refs: Vec<String>,
    /// Reviewable relation summary.
    pub summary_label: String,
}

/// Inspection surface emitted for support/export and fixture checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the inspection record.
    pub schema_version: u32,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// Diff count opened inside the workspace.
    pub diff_entry_count: usize,
    /// Anchor count materialized from protected rows.
    pub anchor_count: usize,
    /// Work-item relation count.
    pub work_item_linkage_count: usize,
    /// True when every anchor excludes provider ids from its stable hash.
    pub provider_ready_anchor_semantics: bool,
    /// True when at least one linkage row is actor and command attributed.
    pub attributable_work_item_linkage_present: bool,
    /// Graph cue packet ids inherited by the review surface.
    #[serde(default)]
    pub graph_cue_packet_refs: Vec<String>,
    /// Readiness tokens copied from inherited graph cue packets.
    #[serde(default)]
    pub graph_cue_readiness_tokens: Vec<String>,
    /// Cue packet epochs copied from inherited graph cue packets.
    #[serde(default)]
    pub graph_cue_epoch_refs: Vec<String>,
    /// Summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Review-workspace seed packet consumed by review, support, and export lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceSeedPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this alpha packet.
    pub schema_version: u32,
    /// Generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Workspace record consumed by review surfaces.
    pub review_workspace: ReviewWorkspaceRecord,
    /// Diffs opened inside the review workspace.
    pub diff_entries: Vec<ReviewWorkspaceDiffEntry>,
    /// Stable anchor ids for protected diff rows.
    pub anchors: Vec<ReviewAnchorIdAlphaRecord>,
    /// Work-item relation rows attached to the workspace.
    pub work_item_linkages: Vec<ReviewWorkItemLinkageRecord>,
    /// Graph readiness cues inherited by the review surface.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub graph_cue_packets: Vec<GraphFactCuePacket>,
    /// Inspection record used by support/export and tests.
    pub inspection: ReviewWorkspaceInspectionRecord,
}

impl ReviewWorkspaceSeedPacket {
    /// Builds a review-workspace seed from a diff packet.
    pub fn from_diff_packet(
        input: ReviewWorkspaceSeedInput,
        diff_packet: &DiffViewSurfacePacket,
    ) -> Self {
        let review_workspace = workspace_record(&input, diff_packet);
        let anchors = anchors_for_diff(&input, diff_packet);
        let graph_cue_packets = input.graph_cue_packets.clone();
        let diff_entries = vec![ReviewWorkspaceDiffEntry {
            diff_surface_ref: diff_packet.diff_surface_ref.clone(),
            launch_source_ref: diff_packet.launch_source_ref.clone(),
            path_truth_ref: diff_packet.path_truth.path_truth_ref.clone(),
            compare_target_ref: diff_packet.compare_target.compare_target_ref.clone(),
            path_label: diff_packet.path_truth.path_label.clone(),
            compare_target_label: diff_packet.compare_target.exact_target_label.clone(),
            anchor_id_refs: anchors
                .iter()
                .map(|anchor| anchor.anchor_id.clone())
                .collect(),
        }];
        let work_item_linkages = work_item_linkages_for(&input, &anchors);
        let inspection = inspection_for(
            &review_workspace,
            &diff_entries,
            &anchors,
            &work_item_linkages,
            &graph_cue_packets,
        );

        Self {
            record_kind: REVIEW_WORKSPACE_SEED_PACKET_RECORD_KIND.to_string(),
            schema_version: REVIEW_WORKSPACE_SEED_PACKET_SCHEMA_VERSION,
            generated_at: input.created_at,
            review_workspace,
            diff_entries,
            anchors,
            work_item_linkages,
            graph_cue_packets,
            inspection,
        }
    }

    /// Returns true when every diff row anchor uses provider-neutral identity.
    pub fn provider_ready_anchor_semantics(&self) -> bool {
        self.anchors
            .iter()
            .all(|anchor| anchor.provider_excluded_from_anchor_hash)
    }

    /// Returns true when every opened diff entry has at least one anchor.
    pub fn every_diff_entry_has_stable_anchors(&self) -> bool {
        !self.diff_entries.is_empty()
            && self
                .diff_entries
                .iter()
                .all(|entry| !entry.anchor_id_refs.is_empty())
    }

    /// Returns true when at least one work-item relation is reviewable and attributed.
    pub fn has_attributable_work_item_linkage(&self) -> bool {
        self.work_item_linkages.iter().any(|linkage| {
            !linkage.work_item_detail_record_id_ref.trim().is_empty()
                && !linkage
                    .linked_review_workspace_record_id_ref
                    .trim()
                    .is_empty()
                && !linkage.actor_ref.trim().is_empty()
                && !linkage.command_id_ref.trim().is_empty()
        })
    }

    /// Returns true when inherited graph cue freshness is preserved on inspection rows.
    pub fn preserves_graph_cue_epoch_parity(&self) -> bool {
        let packet_epochs = self
            .graph_cue_packets
            .iter()
            .map(|packet| packet.emitted_at.as_str())
            .collect::<Vec<_>>();
        let inspection_epochs = self
            .inspection
            .graph_cue_epoch_refs
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        packet_epochs == inspection_epochs
    }
}

/// Input used to build a beta review-workspace packet from a seed packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceBetaInput {
    /// Stable packet identity.
    pub packet_id: String,
    /// Timestamp used for deterministic fixture output.
    pub generated_at: String,
    /// Durable comment anchors attached to alpha anchor IDs.
    pub comment_anchors: Vec<ReviewWorkspaceDurableCommentAnchorInput>,
    /// Current check freshness rows attached to the workspace.
    pub check_freshness: Vec<ReviewWorkspaceCheckFreshnessInput>,
    /// Optional browser handoff skeleton for companion or provider review.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff: Option<ReviewWorkspaceBrowserHandoffInput>,
    /// Support/export envelope that can reopen the review context.
    pub support_export: ReviewWorkspaceSupportExportInput,
}

/// Input used to attach a durable comment anchor to an existing row anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceDurableCommentAnchorInput {
    /// Alpha anchor id that this durable comment anchor preserves.
    pub source_anchor_id_ref: String,
    /// Stable comment thread identity.
    pub comment_thread_id: String,
    /// Opaque ref for the redaction-aware display label.
    pub comment_payload_label_opaque_ref: String,
    /// Actor that posted or imported the comment.
    pub posted_actor_ref: String,
    /// Timestamp when the comment was posted or imported.
    pub posted_at: String,
    /// Drift state from the review-anchor vocabulary.
    pub anchor_drift_state: String,
    /// Required user action paired with the drift state.
    pub anchor_drift_required_user_action: String,
    /// Local/provider freshness state for this anchor.
    pub local_vs_provider_freshness_class: String,
    /// Recorded remap chain when the anchor was safely remapped.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub remap_chain_target_id_refs: Vec<String>,
    /// Archive timestamp when the anchor is retained as a tombstone.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archived_at: Option<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input row describing one check's current freshness in the workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceCheckFreshnessInput {
    /// Stable check identity.
    pub check_id: String,
    /// Check-kind token from the review/check family.
    pub check_kind: String,
    /// Current check status class.
    pub check_status_class: String,
    /// Freshness class for the check result.
    pub check_freshness_class: String,
    /// Authority that produced or imported the check.
    pub check_authority_class: String,
    /// Evidence row or packet backing the check claim.
    pub evidence_ref: String,
    /// Timestamp when the evidence was captured.
    pub captured_at: String,
    /// Expiry timestamp for freshness-sensitive rows.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// True when the row remains valid without browser state.
    pub browser_state_independent: bool,
    /// True when stale or unavailable evidence blocks operator-truth claims.
    pub blocks_operator_truth_claim_when_stale: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input row for a typed and reversible browser handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceBrowserHandoffInput {
    /// Stable handoff identity.
    pub handoff_id: String,
    /// Opaque ref to the integration handoff packet.
    pub browser_handoff_packet_ref: String,
    /// Typed destination class, never a raw URL class.
    pub destination_class: String,
    /// Opaque destination token resolved by the launcher.
    pub destination_ref: String,
    /// Provider-side object identity ref.
    pub object_identity_ref: String,
    /// Reason code explaining why browser handoff is used.
    pub reason_code: String,
    /// Return anchor kind used to reopen the source review context.
    pub return_anchor_kind: String,
    /// Return anchor ref used for reversible handoff.
    pub return_anchor_ref: String,
    /// Replay posture from the browser-handoff contract.
    pub replay_posture: String,
    /// Timestamp when the handoff was issued.
    pub issued_at: String,
    /// Timestamp when the handoff expires.
    pub expires_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Input row for the review-workspace support/export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceSupportExportInput {
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Stable context ref used to reopen the review workspace.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Durable comment anchor record for one review comment or thread.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceDurableCommentAnchorRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the beta record.
    pub schema_version: u32,
    /// Stable durable comment anchor id.
    pub durable_comment_anchor_id: String,
    /// Alpha anchor id that this record preserves.
    pub source_anchor_id_ref: String,
    /// Review workspace this anchor belongs to.
    pub review_workspace_id_ref: String,
    /// Diff target ref inherited from the alpha anchor.
    pub target_ref: String,
    /// Path-truth ref inherited from the alpha anchor.
    pub path_truth_ref: String,
    /// Compare-target ref inherited from the alpha anchor.
    pub compare_target_ref: String,
    /// Fallback context hash inherited from the alpha anchor.
    pub fallback_context_hash: String,
    /// Stable identity fields inherited from the alpha anchor.
    pub stable_identity_fields: Vec<String>,
    /// True when provider object IDs are excluded from the stable anchor hash.
    pub provider_excluded_from_anchor_hash: bool,
    /// Stable comment thread identity.
    pub comment_thread_id: String,
    /// Opaque ref for the redaction-aware display label.
    pub comment_payload_label_opaque_ref: String,
    /// Actor that posted or imported the comment.
    pub posted_actor_ref: String,
    /// Timestamp when the comment was posted or imported.
    pub posted_at: String,
    /// Drift state from the review-anchor vocabulary.
    pub anchor_drift_state: String,
    /// Required user action paired with the drift state.
    pub anchor_drift_required_user_action: String,
    /// Local/provider freshness state for this anchor.
    pub local_vs_provider_freshness_class: String,
    /// Recorded remap chain when the anchor was safely remapped.
    pub remap_chain_target_id_refs: Vec<String>,
    /// Archive timestamp when the anchor is retained as a tombstone.
    pub archived_at: Option<String>,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Object-lineage row tying workspace, diff, anchor, handoff, and export refs together.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceObjectLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the beta record.
    pub schema_version: u32,
    /// Stable lineage row id.
    pub lineage_id: String,
    /// Review workspace this lineage belongs to.
    pub review_workspace_id_ref: String,
    /// Source object ref for the lineage edge.
    pub source_object_ref: String,
    /// Source object kind for the lineage edge.
    pub source_object_kind: String,
    /// Derived object ref for the lineage edge.
    pub derived_object_ref: String,
    /// Derived object kind for the lineage edge.
    pub derived_object_kind: String,
    /// Relation class explaining the lineage edge.
    pub lineage_relation_class: String,
    /// Timestamp when the lineage edge was captured.
    pub captured_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Check-freshness row proving current review checks without browser state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceCheckFreshnessRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the beta record.
    pub schema_version: u32,
    /// Stable check identity.
    pub check_id: String,
    /// Review workspace this check belongs to.
    pub review_workspace_id_ref: String,
    /// Check-kind token from the review/check family.
    pub check_kind: String,
    /// Current check status class.
    pub check_status_class: String,
    /// Freshness class for the check result.
    pub check_freshness_class: String,
    /// Authority that produced or imported the check.
    pub check_authority_class: String,
    /// Evidence row or packet backing the check claim.
    pub evidence_ref: String,
    /// Timestamp when the evidence was captured.
    pub captured_at: String,
    /// Expiry timestamp for freshness-sensitive rows.
    pub expires_at: Option<String>,
    /// True when the row remains valid without browser state.
    pub browser_state_independent: bool,
    /// True when stale or unavailable evidence blocks operator-truth claims.
    pub blocks_operator_truth_claim_when_stale: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Typed browser handoff row that keeps a return anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceBrowserHandoffRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the beta record.
    pub schema_version: u32,
    /// Stable handoff identity.
    pub handoff_id: String,
    /// Review workspace this handoff belongs to.
    pub review_workspace_id_ref: String,
    /// Opaque ref to the integration handoff packet.
    pub browser_handoff_packet_ref: String,
    /// Typed destination class, never a raw URL class.
    pub destination_class: String,
    /// Opaque destination token resolved by the launcher.
    pub destination_ref: String,
    /// Provider-side object identity ref.
    pub object_identity_ref: String,
    /// Reason code explaining why browser handoff is used.
    pub reason_code: String,
    /// Return anchor kind used to reopen the source review context.
    pub return_anchor_kind: String,
    /// Return anchor ref used for reversible handoff.
    pub return_anchor_ref: String,
    /// Replay posture from the browser-handoff contract.
    pub replay_posture: String,
    /// True when the handoff has enough data to return to this workspace.
    pub reversible_handoff: bool,
    /// False so raw URL escape hatches cannot cross the review boundary.
    pub raw_url_export_allowed: bool,
    /// Current handoff state.
    pub handoff_state: String,
    /// Timestamp when the handoff was issued.
    pub issued_at: String,
    /// Timestamp when the handoff expires.
    pub expires_at: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Support/export packet that can reopen a review workspace truthfully.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceSupportExportPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the beta record.
    pub schema_version: u32,
    /// Stable support/export packet identity.
    pub support_export_id: String,
    /// Review workspace this packet exports.
    pub review_workspace_id_ref: String,
    /// Stable context ref used to reopen the review workspace.
    pub reopen_context_ref: String,
    /// Command id used by CLI/headless or support tooling to reopen context.
    pub reopen_command_id_ref: String,
    /// Durable comment anchor refs included in the packet.
    pub durable_comment_anchor_refs: Vec<String>,
    /// Check freshness refs included in the packet.
    pub check_freshness_refs: Vec<String>,
    /// Object lineage refs included in the packet.
    pub object_lineage_refs: Vec<String>,
    /// Browser handoff ref included in the packet.
    pub browser_handoff_ref: Option<String>,
    /// Consumer surfaces that can read this support export.
    pub consumer_surfaces: Vec<String>,
    /// Source schemas the export cites.
    pub source_schema_refs: Vec<String>,
    /// False so raw comment bodies cannot cross the support boundary.
    pub raw_comment_body_export_allowed: bool,
    /// False so raw URLs cannot cross the support boundary.
    pub raw_url_export_allowed: bool,
    /// False so raw source bodies cannot cross the support boundary.
    pub raw_source_body_export_allowed: bool,
    /// Redaction class applied to exported metadata.
    pub redaction_class: String,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Review-side wrapper proving the workspace consumes the same search operator-truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceSearchOperatorTruthExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version for this wrapper.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Review workspace that displayed the operator-truth state.
    pub review_workspace_id_ref: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exact search operator-truth packet shown in the product surface.
    pub operator_truth_packet: SearchOperatorTruthPacket,
}

impl ReviewWorkspaceSearchOperatorTruthExport {
    /// Builds a review export wrapper around the exact search operator-truth packet.
    pub fn from_packet(
        export_id: impl Into<String>,
        review_workspace_id_ref: impl Into<String>,
        exported_at: impl Into<String>,
        operator_truth_packet: SearchOperatorTruthPacket,
    ) -> Self {
        Self {
            record_kind: REVIEW_WORKSPACE_SEARCH_OPERATOR_TRUTH_EXPORT_RECORD_KIND.to_owned(),
            schema_version: REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
            export_id: export_id.into(),
            review_workspace_id_ref: review_workspace_id_ref.into(),
            exported_at: exported_at.into(),
            operator_truth_packet,
        }
    }

    /// Validates that the review export preserves a valid review-workspace packet projection.
    pub fn validate(&self) -> Vec<ReviewWorkspaceSearchOperatorTruthExportViolation> {
        let mut violations = Vec::new();
        if self.record_kind != REVIEW_WORKSPACE_SEARCH_OPERATOR_TRUTH_EXPORT_RECORD_KIND {
            violations.push(ReviewWorkspaceSearchOperatorTruthExportViolation::WrongRecordKind);
        }
        if self.schema_version != REVIEW_WORKSPACE_BETA_SCHEMA_VERSION {
            violations.push(ReviewWorkspaceSearchOperatorTruthExportViolation::WrongSchemaVersion);
        }
        if self.export_id.trim().is_empty()
            || self.review_workspace_id_ref.trim().is_empty()
            || self.exported_at.trim().is_empty()
        {
            violations.push(ReviewWorkspaceSearchOperatorTruthExportViolation::MissingIdentity);
        }
        if !self
            .operator_truth_packet
            .has_projection_for(SearchOperatorConsumerSurface::ReviewWorkspace)
        {
            violations
                .push(ReviewWorkspaceSearchOperatorTruthExportViolation::MissingReviewProjection);
        }
        if self.operator_truth_packet.schema_version != SEARCH_OPERATOR_TRUTH_SCHEMA_VERSION
            || !self.operator_truth_packet.validate().is_empty()
        {
            violations.push(
                ReviewWorkspaceSearchOperatorTruthExportViolation::OperatorTruthPacketInvalid,
            );
        }
        violations
    }

    /// Returns the operator-truth validation findings from the embedded packet.
    pub fn operator_truth_findings(&self) -> Vec<SearchOperatorTruthFinding> {
        self.operator_truth_packet.validate()
    }
}

/// Closed validation vocabulary for [`ReviewWorkspaceSearchOperatorTruthExport`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewWorkspaceSearchOperatorTruthExportViolation {
    /// Wrapper has the wrong record kind.
    WrongRecordKind,
    /// Wrapper has the wrong schema version.
    WrongSchemaVersion,
    /// Wrapper identity fields are incomplete.
    MissingIdentity,
    /// Embedded packet lacks a review-workspace projection preserving the same packet.
    MissingReviewProjection,
    /// Embedded operator-truth packet is invalid.
    OperatorTruthPacketInvalid,
}

impl ReviewWorkspaceSearchOperatorTruthExportViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingReviewProjection => "missing_review_projection",
            Self::OperatorTruthPacketInvalid => "operator_truth_packet_invalid",
        }
    }
}

/// Inspection row used by fixtures, support/export, and shell rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceBetaInspectionRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the beta record.
    pub schema_version: u32,
    /// Review workspace inspected by this row.
    pub review_workspace_id_ref: String,
    /// Number of durable comment anchors.
    pub durable_comment_anchor_count: usize,
    /// Number of object-lineage rows.
    pub object_lineage_count: usize,
    /// Number of check-freshness rows.
    pub check_freshness_count: usize,
    /// True when durable comment anchors preserve source anchor identity.
    pub anchor_identity_preserved: bool,
    /// True when lineage rows connect the workspace to exportable objects.
    pub object_lineage_preserved: bool,
    /// True when check freshness does not depend on browser state.
    pub check_freshness_browser_independent: bool,
    /// True when a typed reversible browser handoff is present.
    pub typed_reversible_browser_handoff_present: bool,
    /// True when support/export can reopen this review context.
    pub support_export_reopenable: bool,
    /// True when raw URL, comment-body, and source-body escapes are absent.
    pub raw_escape_hatches_absent: bool,
    /// True when no check freshness row blocks operator-truth claims.
    pub operator_truth_current: bool,
    /// True when at least one stale check blocks operator-truth claims.
    pub stale_check_blocks_operator_truth: bool,
    /// Reviewable summary safe for support/export surfaces.
    pub summary_label: String,
}

/// Beta review-workspace packet consumed by review, CLI/headless, and support/export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewWorkspaceBetaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for the beta packet.
    pub schema_version: u32,
    /// Stable packet identity.
    pub packet_id: String,
    /// Generation timestamp supplied by the caller.
    pub generated_at: String,
    /// Workspace record consumed by review surfaces.
    pub review_workspace: ReviewWorkspaceRecord,
    /// Diffs opened inside the review workspace.
    pub diff_entries: Vec<ReviewWorkspaceDiffEntry>,
    /// Durable comment anchors attached to stable row anchors.
    pub durable_comment_anchors: Vec<ReviewWorkspaceDurableCommentAnchorRecord>,
    /// Object lineage rows that preserve reopen and export identity.
    pub object_lineage: Vec<ReviewWorkspaceObjectLineageRecord>,
    /// Check-freshness rows independent of browser state.
    pub check_freshness: Vec<ReviewWorkspaceCheckFreshnessRecord>,
    /// Optional typed browser handoff skeleton.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff: Option<ReviewWorkspaceBrowserHandoffRecord>,
    /// Support/export packet that can reopen the review context.
    pub support_export: ReviewWorkspaceSupportExportPacket,
    /// Inspection row used by support/export and tests.
    pub inspection: ReviewWorkspaceBetaInspectionRecord,
}

impl ReviewWorkspaceBetaPacket {
    /// Builds a beta review-workspace packet from an alpha seed packet.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewWorkspaceBetaValidationError`] when the input violates
    /// the beta review-workspace invariants.
    pub fn from_seed_packet(
        input: ReviewWorkspaceBetaInput,
        seed_packet: &ReviewWorkspaceSeedPacket,
    ) -> Result<Self, ReviewWorkspaceBetaValidationError> {
        validate_beta_input(&input, seed_packet)?;

        let durable_comment_anchors = input
            .comment_anchors
            .iter()
            .map(|anchor_input| durable_comment_anchor_for(seed_packet, anchor_input))
            .collect::<Result<Vec<_>, _>>()?;
        let check_freshness = input
            .check_freshness
            .iter()
            .map(|check_input| check_freshness_for(seed_packet, check_input))
            .collect::<Vec<_>>();
        let browser_handoff = input
            .browser_handoff
            .as_ref()
            .map(|handoff| browser_handoff_for(seed_packet, handoff));
        let object_lineage = object_lineage_for(
            seed_packet,
            &durable_comment_anchors,
            browser_handoff.as_ref(),
            &input.support_export,
            &input.generated_at,
        );
        let support_export = support_export_for(
            seed_packet,
            &input.support_export,
            &durable_comment_anchors,
            &check_freshness,
            &object_lineage,
            browser_handoff.as_ref(),
        );
        let inspection = beta_inspection_for(
            &seed_packet.review_workspace,
            &durable_comment_anchors,
            &object_lineage,
            &check_freshness,
            browser_handoff.as_ref(),
            &support_export,
        );

        let packet = Self {
            record_kind: REVIEW_WORKSPACE_BETA_PACKET_RECORD_KIND.to_string(),
            schema_version: REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
            packet_id: input.packet_id,
            generated_at: input.generated_at,
            review_workspace: seed_packet.review_workspace.clone(),
            diff_entries: seed_packet.diff_entries.clone(),
            durable_comment_anchors,
            object_lineage,
            check_freshness,
            browser_handoff,
            support_export,
            inspection,
        };
        packet.validate()?;
        Ok(packet)
    }

    /// Validates this packet against the beta review-workspace invariants.
    ///
    /// # Errors
    ///
    /// Returns [`ReviewWorkspaceBetaValidationError`] when a required
    /// invariant is violated.
    pub fn validate(&self) -> Result<(), ReviewWorkspaceBetaValidationError> {
        ensure_eq(
            self.record_kind.as_str(),
            REVIEW_WORKSPACE_BETA_PACKET_RECORD_KIND,
            "record_kind",
        )?;
        ensure_eq(
            self.schema_version,
            REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
            "schema_version",
        )?;
        ensure_nonempty(&self.packet_id, "packet_id")?;
        ensure_nonempty(&self.generated_at, "generated_at")?;
        if self.durable_comment_anchors.is_empty() {
            return Err(beta_validation_error(
                "durable_comment_anchors must include at least one anchored comment",
            ));
        }
        if self.check_freshness.is_empty() {
            return Err(beta_validation_error(
                "check_freshness must include at least one current check row",
            ));
        }

        for anchor in &self.durable_comment_anchors {
            validate_durable_anchor(anchor, &self.review_workspace.review_workspace_id)?;
        }
        for check in &self.check_freshness {
            validate_check_freshness(check, &self.review_workspace.review_workspace_id)?;
        }
        for lineage in &self.object_lineage {
            validate_object_lineage(lineage, &self.review_workspace.review_workspace_id)?;
        }
        if let Some(handoff) = &self.browser_handoff {
            validate_browser_handoff(handoff, &self.review_workspace.review_workspace_id)?;
        }
        validate_support_export(
            &self.support_export,
            &self.review_workspace.review_workspace_id,
            &self.durable_comment_anchors,
            &self.check_freshness,
            &self.object_lineage,
            self.browser_handoff.as_ref(),
        )?;
        validate_beta_inspection(
            &self.inspection,
            &self.review_workspace.review_workspace_id,
            self,
        )?;
        Ok(())
    }

    /// Returns true when every durable comment anchor preserves source anchor identity.
    pub fn preserves_anchor_identity(&self) -> bool {
        !self.durable_comment_anchors.is_empty()
            && self.durable_comment_anchors.iter().all(|anchor| {
                !anchor.source_anchor_id_ref.trim().is_empty()
                    && anchor.provider_excluded_from_anchor_hash
                    && !anchor.fallback_context_hash.trim().is_empty()
                    && !anchor.stable_identity_fields.is_empty()
            })
    }

    /// Returns true when every check row is independent of transient browser state.
    pub fn check_freshness_is_browser_independent(&self) -> bool {
        !self.check_freshness.is_empty()
            && self
                .check_freshness
                .iter()
                .all(|check| check.browser_state_independent)
    }

    /// Returns true when the packet carries a typed reversible browser handoff.
    pub fn has_typed_reversible_browser_handoff(&self) -> bool {
        self.browser_handoff
            .as_ref()
            .map(is_typed_reversible_handoff)
            .unwrap_or(false)
    }

    /// Returns true when support/export can reopen the review context.
    pub fn support_export_can_reopen_context(&self) -> bool {
        support_export_reopenable(
            &self.support_export,
            &self.durable_comment_anchors,
            &self.check_freshness,
            &self.object_lineage,
        )
    }

    /// Returns true when no check blocks operator-truth claims.
    pub fn operator_truth_current(&self) -> bool {
        self.check_freshness.iter().all(|check| {
            check.check_freshness_class == "check_current"
                || check.check_freshness_class == "check_stale_within_grace"
        })
    }

    /// Returns true when raw URL/comment/source escapes are absent.
    pub fn raw_escape_hatches_absent(&self) -> bool {
        self.support_export.raw_comment_body_export_allowed == false
            && self.support_export.raw_url_export_allowed == false
            && self.support_export.raw_source_body_export_allowed == false
            && self
                .browser_handoff
                .as_ref()
                .map(|handoff| {
                    handoff.raw_url_export_allowed == false
                        && !looks_like_raw_url(&handoff.destination_ref)
                })
                .unwrap_or(true)
    }
}

/// Compact projection consumed by CLI/headless and inspector surfaces.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewWorkspaceBetaProjection {
    /// Stable packet identity.
    pub packet_id: String,
    /// Review workspace identity.
    pub review_workspace_id: String,
    /// Review workspace source class.
    pub review_workspace_source_class: String,
    /// Provider authority class.
    pub provider_authority_class: String,
    /// Workspace freshness class.
    pub freshness_class: String,
    /// Durable comment anchor count.
    pub durable_comment_anchor_count: usize,
    /// Check freshness row count.
    pub check_freshness_count: usize,
    /// Object lineage row count.
    pub object_lineage_count: usize,
    /// True when a typed reversible browser handoff is present.
    pub typed_reversible_browser_handoff_present: bool,
    /// True when support/export can reopen the review context.
    pub support_export_reopenable: bool,
    /// True when stale checks block operator-truth claims.
    pub stale_check_blocks_operator_truth: bool,
    /// Consumer surfaces wired through the support export.
    pub consumer_surfaces: Vec<String>,
    /// Redaction class used by the support export.
    pub redaction_class: String,
}

/// Parses and validates a materialized beta review-workspace packet.
///
/// # Errors
///
/// Returns [`ReviewWorkspaceBetaError`] when the payload fails to parse or
/// violates the beta review-workspace invariants.
pub fn project_review_workspace_beta_packet(
    payload: &str,
) -> Result<ReviewWorkspaceBetaProjection, ReviewWorkspaceBetaError> {
    let packet: ReviewWorkspaceBetaPacket = serde_json::from_str(payload)?;
    packet.validate()?;
    Ok(ReviewWorkspaceBetaProjection::from(packet))
}

impl From<ReviewWorkspaceBetaPacket> for ReviewWorkspaceBetaProjection {
    fn from(packet: ReviewWorkspaceBetaPacket) -> Self {
        Self {
            packet_id: packet.packet_id,
            review_workspace_id: packet.review_workspace.review_workspace_id,
            review_workspace_source_class: packet.review_workspace.review_workspace_source_class,
            provider_authority_class: packet.review_workspace.provider_authority_class,
            freshness_class: packet.review_workspace.freshness_class,
            durable_comment_anchor_count: packet.inspection.durable_comment_anchor_count,
            check_freshness_count: packet.inspection.check_freshness_count,
            object_lineage_count: packet.inspection.object_lineage_count,
            typed_reversible_browser_handoff_present: packet
                .inspection
                .typed_reversible_browser_handoff_present,
            support_export_reopenable: packet.inspection.support_export_reopenable,
            stale_check_blocks_operator_truth: packet.inspection.stale_check_blocks_operator_truth,
            consumer_surfaces: packet.support_export.consumer_surfaces,
            redaction_class: packet.support_export.redaction_class,
        }
    }
}

/// Error returned when a beta review-workspace payload cannot be projected.
#[derive(Debug)]
pub enum ReviewWorkspaceBetaError {
    /// The payload failed JSON parsing.
    Parse(serde_json::Error),
    /// The payload parsed but violated the beta invariants.
    Validation(ReviewWorkspaceBetaValidationError),
}

impl fmt::Display for ReviewWorkspaceBetaError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse(err) => write!(formatter, "review workspace beta parse error: {err}"),
            Self::Validation(err) => {
                write!(formatter, "review workspace beta validation error: {err}")
            }
        }
    }
}

impl std::error::Error for ReviewWorkspaceBetaError {}

impl From<serde_json::Error> for ReviewWorkspaceBetaError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<ReviewWorkspaceBetaValidationError> for ReviewWorkspaceBetaError {
    fn from(err: ReviewWorkspaceBetaValidationError) -> Self {
        Self::Validation(err)
    }
}

/// Validation failure for beta review-workspace packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReviewWorkspaceBetaValidationError {
    message: String,
}

impl ReviewWorkspaceBetaValidationError {
    /// Returns the validation failure message.
    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ReviewWorkspaceBetaValidationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for ReviewWorkspaceBetaValidationError {}

fn validate_beta_input(
    input: &ReviewWorkspaceBetaInput,
    seed_packet: &ReviewWorkspaceSeedPacket,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_nonempty(&input.packet_id, "packet_id")?;
    ensure_nonempty(&input.generated_at, "generated_at")?;
    if input.comment_anchors.is_empty() {
        return Err(beta_validation_error(
            "comment_anchors must include at least one durable anchor input",
        ));
    }
    if input.check_freshness.is_empty() {
        return Err(beta_validation_error(
            "check_freshness must include at least one check input",
        ));
    }

    let alpha_anchor_ids = seed_packet
        .anchors
        .iter()
        .map(|anchor| anchor.anchor_id.as_str())
        .collect::<BTreeSet<_>>();
    for anchor in &input.comment_anchors {
        if !alpha_anchor_ids.contains(anchor.source_anchor_id_ref.as_str()) {
            return Err(beta_validation_error(format!(
                "comment anchor source_anchor_id_ref '{}' is not present in the seed packet",
                anchor.source_anchor_id_ref
            )));
        }
        validate_anchor_drift_pair(
            &anchor.anchor_drift_state,
            &anchor.anchor_drift_required_user_action,
            &anchor.remap_chain_target_id_refs,
            anchor.archived_at.as_deref(),
        )?;
    }

    for check in &input.check_freshness {
        validate_check_freshness_input(check)?;
    }
    if let Some(handoff) = &input.browser_handoff {
        validate_browser_handoff_input(handoff)?;
    }
    validate_support_export_input(&input.support_export)?;
    Ok(())
}

fn durable_comment_anchor_for(
    seed_packet: &ReviewWorkspaceSeedPacket,
    input: &ReviewWorkspaceDurableCommentAnchorInput,
) -> Result<ReviewWorkspaceDurableCommentAnchorRecord, ReviewWorkspaceBetaValidationError> {
    let source_anchor = seed_packet
        .anchors
        .iter()
        .find(|anchor| anchor.anchor_id == input.source_anchor_id_ref)
        .ok_or_else(|| {
            beta_validation_error(format!(
                "source anchor '{}' is missing from the seed packet",
                input.source_anchor_id_ref
            ))
        })?;
    let durable_comment_anchor_id = format!(
        "review.comment_anchor.beta.{}.{}",
        sanitize_id(&seed_packet.review_workspace.review_workspace_id),
        stable_hash(&format!(
            "{}|{}",
            input.source_anchor_id_ref, input.comment_thread_id
        ))
    );

    Ok(ReviewWorkspaceDurableCommentAnchorRecord {
        record_kind: REVIEW_WORKSPACE_DURABLE_COMMENT_ANCHOR_RECORD_KIND.to_string(),
        schema_version: REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        durable_comment_anchor_id,
        source_anchor_id_ref: input.source_anchor_id_ref.clone(),
        review_workspace_id_ref: seed_packet.review_workspace.review_workspace_id.clone(),
        target_ref: source_anchor.target_ref.clone(),
        path_truth_ref: source_anchor.path_truth_ref.clone(),
        compare_target_ref: source_anchor.compare_target_ref.clone(),
        fallback_context_hash: source_anchor.fallback_context_hash.clone(),
        stable_identity_fields: source_anchor.stable_identity_fields.clone(),
        provider_excluded_from_anchor_hash: source_anchor.provider_excluded_from_anchor_hash,
        comment_thread_id: input.comment_thread_id.clone(),
        comment_payload_label_opaque_ref: input.comment_payload_label_opaque_ref.clone(),
        posted_actor_ref: input.posted_actor_ref.clone(),
        posted_at: input.posted_at.clone(),
        anchor_drift_state: input.anchor_drift_state.clone(),
        anchor_drift_required_user_action: input.anchor_drift_required_user_action.clone(),
        local_vs_provider_freshness_class: input.local_vs_provider_freshness_class.clone(),
        remap_chain_target_id_refs: input.remap_chain_target_id_refs.clone(),
        archived_at: input.archived_at.clone(),
        summary_label: input.summary_label.clone(),
    })
}

fn check_freshness_for(
    seed_packet: &ReviewWorkspaceSeedPacket,
    input: &ReviewWorkspaceCheckFreshnessInput,
) -> ReviewWorkspaceCheckFreshnessRecord {
    ReviewWorkspaceCheckFreshnessRecord {
        record_kind: REVIEW_WORKSPACE_CHECK_FRESHNESS_RECORD_KIND.to_string(),
        schema_version: REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        check_id: input.check_id.clone(),
        review_workspace_id_ref: seed_packet.review_workspace.review_workspace_id.clone(),
        check_kind: input.check_kind.clone(),
        check_status_class: input.check_status_class.clone(),
        check_freshness_class: input.check_freshness_class.clone(),
        check_authority_class: input.check_authority_class.clone(),
        evidence_ref: input.evidence_ref.clone(),
        captured_at: input.captured_at.clone(),
        expires_at: input.expires_at.clone(),
        browser_state_independent: input.browser_state_independent,
        blocks_operator_truth_claim_when_stale: input.blocks_operator_truth_claim_when_stale,
        summary_label: input.summary_label.clone(),
    }
}

fn browser_handoff_for(
    seed_packet: &ReviewWorkspaceSeedPacket,
    input: &ReviewWorkspaceBrowserHandoffInput,
) -> ReviewWorkspaceBrowserHandoffRecord {
    ReviewWorkspaceBrowserHandoffRecord {
        record_kind: REVIEW_WORKSPACE_BROWSER_HANDOFF_RECORD_KIND.to_string(),
        schema_version: REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        handoff_id: input.handoff_id.clone(),
        review_workspace_id_ref: seed_packet.review_workspace.review_workspace_id.clone(),
        browser_handoff_packet_ref: input.browser_handoff_packet_ref.clone(),
        destination_class: input.destination_class.clone(),
        destination_ref: input.destination_ref.clone(),
        object_identity_ref: input.object_identity_ref.clone(),
        reason_code: input.reason_code.clone(),
        return_anchor_kind: input.return_anchor_kind.clone(),
        return_anchor_ref: input.return_anchor_ref.clone(),
        replay_posture: input.replay_posture.clone(),
        reversible_handoff: !input.return_anchor_kind.trim().is_empty()
            && !input.return_anchor_ref.trim().is_empty(),
        raw_url_export_allowed: false,
        handoff_state: "handoff_minted_not_launched".to_string(),
        issued_at: input.issued_at.clone(),
        expires_at: input.expires_at.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn object_lineage_for(
    seed_packet: &ReviewWorkspaceSeedPacket,
    durable_comment_anchors: &[ReviewWorkspaceDurableCommentAnchorRecord],
    browser_handoff: Option<&ReviewWorkspaceBrowserHandoffRecord>,
    support_export: &ReviewWorkspaceSupportExportInput,
    captured_at: &str,
) -> Vec<ReviewWorkspaceObjectLineageRecord> {
    let workspace_id = &seed_packet.review_workspace.review_workspace_id;
    let mut rows = Vec::new();
    if let Some(local) = &seed_packet.review_workspace.local_locator {
        rows.push(object_lineage_row(
            workspace_id,
            &local.branch_or_worktree_ref,
            "local_branch_or_worktree",
            workspace_id,
            "review_workspace",
            "materialized_from_local_locator",
            captured_at,
            "Review workspace materialized from local branch or worktree.",
        ));
    }
    if let Some(provider_overlay) = &seed_packet.review_workspace.provider_overlay {
        rows.push(object_lineage_row(
            workspace_id,
            &provider_overlay.provider_object_identity_ref,
            "provider_review_object",
            workspace_id,
            "review_workspace",
            "enriched_by_provider_overlay",
            captured_at,
            "Review workspace preserves provider overlay identity separately from local truth.",
        ));
    }
    for diff_entry in &seed_packet.diff_entries {
        rows.push(object_lineage_row(
            workspace_id,
            &diff_entry.diff_surface_ref,
            "diff_surface",
            workspace_id,
            "review_workspace",
            "opened_in_review_workspace",
            captured_at,
            "Diff surface opened inside the review workspace.",
        ));
    }
    for anchor in durable_comment_anchors {
        rows.push(object_lineage_row(
            workspace_id,
            &anchor.source_anchor_id_ref,
            "review_anchor_id",
            &anchor.durable_comment_anchor_id,
            "durable_comment_anchor",
            "preserves_comment_anchor_identity",
            captured_at,
            "Durable comment anchor preserves the provider-neutral source anchor.",
        ));
    }
    if let Some(handoff) = browser_handoff {
        rows.push(object_lineage_row(
            workspace_id,
            workspace_id,
            "review_workspace",
            &handoff.handoff_id,
            "browser_handoff",
            "mints_typed_browser_handoff",
            captured_at,
            "Browser handoff is typed and keeps a return anchor.",
        ));
    }
    rows.push(object_lineage_row(
        workspace_id,
        workspace_id,
        "review_workspace",
        &support_export.support_export_id,
        "support_export_packet",
        "exports_reopenable_support_packet",
        captured_at,
        "Support export preserves enough metadata to reopen the review context.",
    ));
    rows
}

fn object_lineage_row(
    review_workspace_id_ref: &str,
    source_object_ref: &str,
    source_object_kind: &str,
    derived_object_ref: &str,
    derived_object_kind: &str,
    lineage_relation_class: &str,
    captured_at: &str,
    summary_label: &str,
) -> ReviewWorkspaceObjectLineageRecord {
    ReviewWorkspaceObjectLineageRecord {
        record_kind: REVIEW_WORKSPACE_OBJECT_LINEAGE_RECORD_KIND.to_string(),
        schema_version: REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        lineage_id: format!(
            "review.workspace.lineage.{}.{}",
            sanitize_id(review_workspace_id_ref),
            stable_hash(&format!(
                "{source_object_ref}|{derived_object_ref}|{lineage_relation_class}"
            ))
        ),
        review_workspace_id_ref: review_workspace_id_ref.to_string(),
        source_object_ref: source_object_ref.to_string(),
        source_object_kind: source_object_kind.to_string(),
        derived_object_ref: derived_object_ref.to_string(),
        derived_object_kind: derived_object_kind.to_string(),
        lineage_relation_class: lineage_relation_class.to_string(),
        captured_at: captured_at.to_string(),
        summary_label: summary_label.to_string(),
    }
}

fn support_export_for(
    seed_packet: &ReviewWorkspaceSeedPacket,
    input: &ReviewWorkspaceSupportExportInput,
    durable_comment_anchors: &[ReviewWorkspaceDurableCommentAnchorRecord],
    check_freshness: &[ReviewWorkspaceCheckFreshnessRecord],
    object_lineage: &[ReviewWorkspaceObjectLineageRecord],
    browser_handoff: Option<&ReviewWorkspaceBrowserHandoffRecord>,
) -> ReviewWorkspaceSupportExportPacket {
    ReviewWorkspaceSupportExportPacket {
        record_kind: REVIEW_WORKSPACE_SUPPORT_EXPORT_PACKET_RECORD_KIND.to_string(),
        schema_version: REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        support_export_id: input.support_export_id.clone(),
        review_workspace_id_ref: seed_packet.review_workspace.review_workspace_id.clone(),
        reopen_context_ref: input.reopen_context_ref.clone(),
        reopen_command_id_ref: input.reopen_command_id_ref.clone(),
        durable_comment_anchor_refs: durable_comment_anchors
            .iter()
            .map(|anchor| anchor.durable_comment_anchor_id.clone())
            .collect(),
        check_freshness_refs: check_freshness
            .iter()
            .map(|check| check.check_id.clone())
            .collect(),
        object_lineage_refs: object_lineage
            .iter()
            .map(|lineage| lineage.lineage_id.clone())
            .collect(),
        browser_handoff_ref: browser_handoff.map(|handoff| handoff.handoff_id.clone()),
        consumer_surfaces: input.consumer_surfaces.clone(),
        source_schema_refs: vec![
            "schemas/review/review_workspace.schema.json".to_string(),
            "schemas/vcs/review_workspace.schema.json".to_string(),
            "schemas/vcs/review_anchor.schema.json".to_string(),
            "schemas/review/anchor_id_alpha.schema.json".to_string(),
            "schemas/integration/browser_handoff_packet.schema.json".to_string(),
        ],
        raw_comment_body_export_allowed: false,
        raw_url_export_allowed: false,
        raw_source_body_export_allowed: false,
        redaction_class: input.redaction_class.clone(),
        summary_label: input.summary_label.clone(),
    }
}

fn beta_inspection_for(
    workspace: &ReviewWorkspaceRecord,
    durable_comment_anchors: &[ReviewWorkspaceDurableCommentAnchorRecord],
    object_lineage: &[ReviewWorkspaceObjectLineageRecord],
    check_freshness: &[ReviewWorkspaceCheckFreshnessRecord],
    browser_handoff: Option<&ReviewWorkspaceBrowserHandoffRecord>,
    support_export: &ReviewWorkspaceSupportExportPacket,
) -> ReviewWorkspaceBetaInspectionRecord {
    let anchor_identity_preserved = durable_comment_anchors.iter().all(|anchor| {
        !anchor.source_anchor_id_ref.trim().is_empty()
            && anchor.provider_excluded_from_anchor_hash
            && !anchor.fallback_context_hash.trim().is_empty()
            && !anchor.stable_identity_fields.is_empty()
    });
    let object_lineage_preserved = !object_lineage.is_empty()
        && object_lineage.iter().all(|lineage| {
            lineage.review_workspace_id_ref == workspace.review_workspace_id
                && !lineage.source_object_ref.trim().is_empty()
                && !lineage.derived_object_ref.trim().is_empty()
        })
        && object_lineage
            .iter()
            .any(|lineage| lineage.lineage_relation_class == "exports_reopenable_support_packet");
    let check_freshness_browser_independent = check_freshness
        .iter()
        .all(|check| check.browser_state_independent);
    let typed_reversible_browser_handoff_present = browser_handoff
        .map(is_typed_reversible_handoff)
        .unwrap_or(false);
    let support_export_reopenable = support_export_reopenable(
        support_export,
        durable_comment_anchors,
        check_freshness,
        object_lineage,
    );
    let raw_escape_hatches_absent = support_export.raw_comment_body_export_allowed == false
        && support_export.raw_url_export_allowed == false
        && support_export.raw_source_body_export_allowed == false
        && browser_handoff
            .map(|handoff| handoff.raw_url_export_allowed == false)
            .unwrap_or(true);
    let stale_check_blocks_operator_truth = check_freshness
        .iter()
        .any(|check| check.blocks_operator_truth_claim_when_stale);
    let operator_truth_current = check_freshness.iter().all(|check| {
        check.check_freshness_class == "check_current"
            || check.check_freshness_class == "check_stale_within_grace"
    });

    ReviewWorkspaceBetaInspectionRecord {
        record_kind: REVIEW_WORKSPACE_BETA_INSPECTION_RECORD_KIND.to_string(),
        schema_version: REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        review_workspace_id_ref: workspace.review_workspace_id.clone(),
        durable_comment_anchor_count: durable_comment_anchors.len(),
        object_lineage_count: object_lineage.len(),
        check_freshness_count: check_freshness.len(),
        anchor_identity_preserved,
        object_lineage_preserved,
        check_freshness_browser_independent,
        typed_reversible_browser_handoff_present,
        support_export_reopenable,
        raw_escape_hatches_absent,
        operator_truth_current,
        stale_check_blocks_operator_truth,
        summary_label: format!(
            "{} durable anchor(s), {} check freshness row(s), {} lineage row(s)",
            durable_comment_anchors.len(),
            check_freshness.len(),
            object_lineage.len()
        ),
    }
}

fn validate_durable_anchor(
    anchor: &ReviewWorkspaceDurableCommentAnchorRecord,
    workspace_id: &str,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_eq(
        anchor.record_kind.as_str(),
        REVIEW_WORKSPACE_DURABLE_COMMENT_ANCHOR_RECORD_KIND,
        "durable_comment_anchor.record_kind",
    )?;
    ensure_eq(
        anchor.schema_version,
        REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        "durable_comment_anchor.schema_version",
    )?;
    ensure_eq(
        anchor.review_workspace_id_ref.as_str(),
        workspace_id,
        "durable_comment_anchor.review_workspace_id_ref",
    )?;
    ensure_nonempty(
        &anchor.durable_comment_anchor_id,
        "durable_comment_anchor.durable_comment_anchor_id",
    )?;
    ensure_nonempty(
        &anchor.source_anchor_id_ref,
        "durable_comment_anchor.source_anchor_id_ref",
    )?;
    ensure_nonempty(&anchor.target_ref, "durable_comment_anchor.target_ref")?;
    ensure_nonempty(
        &anchor.fallback_context_hash,
        "durable_comment_anchor.fallback_context_hash",
    )?;
    if !anchor.provider_excluded_from_anchor_hash {
        return Err(beta_validation_error(
            "durable_comment_anchor.provider_excluded_from_anchor_hash must be true",
        ));
    }
    if !contains_token(
        REVIEW_WORKSPACE_BETA_ANCHOR_FRESHNESS_CLASSES,
        &anchor.local_vs_provider_freshness_class,
    ) {
        return Err(beta_validation_error(format!(
            "unsupported local_vs_provider_freshness_class '{}'",
            anchor.local_vs_provider_freshness_class
        )));
    }
    validate_anchor_drift_pair(
        &anchor.anchor_drift_state,
        &anchor.anchor_drift_required_user_action,
        &anchor.remap_chain_target_id_refs,
        anchor.archived_at.as_deref(),
    )
}

fn validate_check_freshness_input(
    check: &ReviewWorkspaceCheckFreshnessInput,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_nonempty(&check.check_id, "check_freshness.check_id")?;
    ensure_nonempty(&check.evidence_ref, "check_freshness.evidence_ref")?;
    if !contains_token(
        REVIEW_WORKSPACE_BETA_CHECK_STATUS_CLASSES,
        &check.check_status_class,
    ) {
        return Err(beta_validation_error(format!(
            "unsupported check_status_class '{}'",
            check.check_status_class
        )));
    }
    if !contains_token(
        REVIEW_WORKSPACE_BETA_CHECK_FRESHNESS_CLASSES,
        &check.check_freshness_class,
    ) {
        return Err(beta_validation_error(format!(
            "unsupported check_freshness_class '{}'",
            check.check_freshness_class
        )));
    }
    if !contains_token(
        REVIEW_WORKSPACE_BETA_CHECK_AUTHORITY_CLASSES,
        &check.check_authority_class,
    ) {
        return Err(beta_validation_error(format!(
            "unsupported check_authority_class '{}'",
            check.check_authority_class
        )));
    }
    if !check.browser_state_independent {
        return Err(beta_validation_error(
            "check_freshness.browser_state_independent must be true",
        ));
    }
    if freshness_blocks_operator_truth(&check.check_freshness_class)
        && !check.blocks_operator_truth_claim_when_stale
    {
        return Err(beta_validation_error(
            "stale or unavailable check freshness must block operator-truth claims",
        ));
    }
    Ok(())
}

fn validate_check_freshness(
    check: &ReviewWorkspaceCheckFreshnessRecord,
    workspace_id: &str,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_eq(
        check.record_kind.as_str(),
        REVIEW_WORKSPACE_CHECK_FRESHNESS_RECORD_KIND,
        "check_freshness.record_kind",
    )?;
    ensure_eq(
        check.schema_version,
        REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        "check_freshness.schema_version",
    )?;
    ensure_eq(
        check.review_workspace_id_ref.as_str(),
        workspace_id,
        "check_freshness.review_workspace_id_ref",
    )?;
    validate_check_freshness_input(&ReviewWorkspaceCheckFreshnessInput {
        check_id: check.check_id.clone(),
        check_kind: check.check_kind.clone(),
        check_status_class: check.check_status_class.clone(),
        check_freshness_class: check.check_freshness_class.clone(),
        check_authority_class: check.check_authority_class.clone(),
        evidence_ref: check.evidence_ref.clone(),
        captured_at: check.captured_at.clone(),
        expires_at: check.expires_at.clone(),
        browser_state_independent: check.browser_state_independent,
        blocks_operator_truth_claim_when_stale: check.blocks_operator_truth_claim_when_stale,
        summary_label: check.summary_label.clone(),
    })
}

fn validate_browser_handoff_input(
    handoff: &ReviewWorkspaceBrowserHandoffInput,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_nonempty(&handoff.handoff_id, "browser_handoff.handoff_id")?;
    ensure_nonempty(
        &handoff.browser_handoff_packet_ref,
        "browser_handoff.browser_handoff_packet_ref",
    )?;
    if !contains_token(
        REVIEW_WORKSPACE_BETA_HANDOFF_DESTINATION_CLASSES,
        &handoff.destination_class,
    ) {
        return Err(beta_validation_error(format!(
            "unsupported destination_class '{}'",
            handoff.destination_class
        )));
    }
    if looks_like_raw_url(&handoff.destination_ref) {
        return Err(beta_validation_error(
            "browser_handoff.destination_ref must be opaque and not a raw URL",
        ));
    }
    if !contains_token(
        REVIEW_WORKSPACE_BETA_HANDOFF_REASON_CODES,
        &handoff.reason_code,
    ) {
        return Err(beta_validation_error(format!(
            "unsupported reason_code '{}'",
            handoff.reason_code
        )));
    }
    if !contains_token(
        REVIEW_WORKSPACE_BETA_HANDOFF_REPLAY_POSTURES,
        &handoff.replay_posture,
    ) {
        return Err(beta_validation_error(format!(
            "unsupported replay_posture '{}'",
            handoff.replay_posture
        )));
    }
    ensure_nonempty(
        &handoff.return_anchor_ref,
        "browser_handoff.return_anchor_ref",
    )?;
    ensure_nonempty(
        &handoff.return_anchor_kind,
        "browser_handoff.return_anchor_kind",
    )
}

fn validate_browser_handoff(
    handoff: &ReviewWorkspaceBrowserHandoffRecord,
    workspace_id: &str,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_eq(
        handoff.record_kind.as_str(),
        REVIEW_WORKSPACE_BROWSER_HANDOFF_RECORD_KIND,
        "browser_handoff.record_kind",
    )?;
    ensure_eq(
        handoff.schema_version,
        REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        "browser_handoff.schema_version",
    )?;
    ensure_eq(
        handoff.review_workspace_id_ref.as_str(),
        workspace_id,
        "browser_handoff.review_workspace_id_ref",
    )?;
    validate_browser_handoff_input(&ReviewWorkspaceBrowserHandoffInput {
        handoff_id: handoff.handoff_id.clone(),
        browser_handoff_packet_ref: handoff.browser_handoff_packet_ref.clone(),
        destination_class: handoff.destination_class.clone(),
        destination_ref: handoff.destination_ref.clone(),
        object_identity_ref: handoff.object_identity_ref.clone(),
        reason_code: handoff.reason_code.clone(),
        return_anchor_kind: handoff.return_anchor_kind.clone(),
        return_anchor_ref: handoff.return_anchor_ref.clone(),
        replay_posture: handoff.replay_posture.clone(),
        issued_at: handoff.issued_at.clone(),
        expires_at: handoff.expires_at.clone(),
        summary_label: handoff.summary_label.clone(),
    })?;
    if !handoff.reversible_handoff {
        return Err(beta_validation_error(
            "browser_handoff.reversible_handoff must be true",
        ));
    }
    if handoff.raw_url_export_allowed {
        return Err(beta_validation_error(
            "browser_handoff.raw_url_export_allowed must be false",
        ));
    }
    Ok(())
}

fn validate_object_lineage(
    lineage: &ReviewWorkspaceObjectLineageRecord,
    workspace_id: &str,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_eq(
        lineage.record_kind.as_str(),
        REVIEW_WORKSPACE_OBJECT_LINEAGE_RECORD_KIND,
        "object_lineage.record_kind",
    )?;
    ensure_eq(
        lineage.schema_version,
        REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        "object_lineage.schema_version",
    )?;
    ensure_eq(
        lineage.review_workspace_id_ref.as_str(),
        workspace_id,
        "object_lineage.review_workspace_id_ref",
    )?;
    ensure_nonempty(&lineage.lineage_id, "object_lineage.lineage_id")?;
    ensure_nonempty(
        &lineage.source_object_ref,
        "object_lineage.source_object_ref",
    )?;
    ensure_nonempty(
        &lineage.derived_object_ref,
        "object_lineage.derived_object_ref",
    )?;
    ensure_nonempty(
        &lineage.lineage_relation_class,
        "object_lineage.lineage_relation_class",
    )
}

fn validate_support_export_input(
    export: &ReviewWorkspaceSupportExportInput,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_nonempty(
        &export.support_export_id,
        "support_export.support_export_id",
    )?;
    ensure_nonempty(
        &export.reopen_context_ref,
        "support_export.reopen_context_ref",
    )?;
    ensure_nonempty(
        &export.reopen_command_id_ref,
        "support_export.reopen_command_id_ref",
    )?;
    if !export
        .consumer_surfaces
        .iter()
        .any(|surface| surface == "support_export")
    {
        return Err(beta_validation_error(
            "support_export.consumer_surfaces must include support_export",
        ));
    }
    if !export
        .consumer_surfaces
        .iter()
        .any(|surface| surface == "cli_headless_entry")
    {
        return Err(beta_validation_error(
            "support_export.consumer_surfaces must include cli_headless_entry",
        ));
    }
    for surface in &export.consumer_surfaces {
        if !contains_token(REVIEW_WORKSPACE_BETA_CONSUMER_SURFACES, surface) {
            return Err(beta_validation_error(format!(
                "unsupported consumer_surface '{}'",
                surface
            )));
        }
    }
    Ok(())
}

fn validate_support_export(
    export: &ReviewWorkspaceSupportExportPacket,
    workspace_id: &str,
    durable_comment_anchors: &[ReviewWorkspaceDurableCommentAnchorRecord],
    check_freshness: &[ReviewWorkspaceCheckFreshnessRecord],
    object_lineage: &[ReviewWorkspaceObjectLineageRecord],
    browser_handoff: Option<&ReviewWorkspaceBrowserHandoffRecord>,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_eq(
        export.record_kind.as_str(),
        REVIEW_WORKSPACE_SUPPORT_EXPORT_PACKET_RECORD_KIND,
        "support_export.record_kind",
    )?;
    ensure_eq(
        export.schema_version,
        REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        "support_export.schema_version",
    )?;
    ensure_eq(
        export.review_workspace_id_ref.as_str(),
        workspace_id,
        "support_export.review_workspace_id_ref",
    )?;
    validate_support_export_input(&ReviewWorkspaceSupportExportInput {
        support_export_id: export.support_export_id.clone(),
        reopen_context_ref: export.reopen_context_ref.clone(),
        reopen_command_id_ref: export.reopen_command_id_ref.clone(),
        consumer_surfaces: export.consumer_surfaces.clone(),
        redaction_class: export.redaction_class.clone(),
        summary_label: export.summary_label.clone(),
    })?;
    if export.raw_comment_body_export_allowed
        || export.raw_url_export_allowed
        || export.raw_source_body_export_allowed
    {
        return Err(beta_validation_error(
            "support_export raw export flags must all be false",
        ));
    }
    if !export
        .source_schema_refs
        .iter()
        .any(|schema| schema == "schemas/review/review_workspace.schema.json")
    {
        return Err(beta_validation_error(
            "support_export must cite schemas/review/review_workspace.schema.json",
        ));
    }

    let expected_anchor_refs = durable_comment_anchors
        .iter()
        .map(|anchor| anchor.durable_comment_anchor_id.clone())
        .collect::<BTreeSet<_>>();
    let actual_anchor_refs = export
        .durable_comment_anchor_refs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if expected_anchor_refs != actual_anchor_refs {
        return Err(beta_validation_error(
            "support_export durable anchor refs must match packet anchors",
        ));
    }

    let expected_check_refs = check_freshness
        .iter()
        .map(|check| check.check_id.clone())
        .collect::<BTreeSet<_>>();
    let actual_check_refs = export
        .check_freshness_refs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if expected_check_refs != actual_check_refs {
        return Err(beta_validation_error(
            "support_export check refs must match packet checks",
        ));
    }

    let expected_lineage_refs = object_lineage
        .iter()
        .map(|lineage| lineage.lineage_id.clone())
        .collect::<BTreeSet<_>>();
    let actual_lineage_refs = export
        .object_lineage_refs
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    if expected_lineage_refs != actual_lineage_refs {
        return Err(beta_validation_error(
            "support_export lineage refs must match packet lineage rows",
        ));
    }

    match (browser_handoff, export.browser_handoff_ref.as_ref()) {
        (Some(handoff), Some(export_ref)) if export_ref == &handoff.handoff_id => {}
        (None, None) => {}
        _ => {
            return Err(beta_validation_error(
                "support_export browser handoff ref must match packet handoff",
            ))
        }
    }
    Ok(())
}

fn validate_beta_inspection(
    inspection: &ReviewWorkspaceBetaInspectionRecord,
    workspace_id: &str,
    packet: &ReviewWorkspaceBetaPacket,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    ensure_eq(
        inspection.record_kind.as_str(),
        REVIEW_WORKSPACE_BETA_INSPECTION_RECORD_KIND,
        "inspection.record_kind",
    )?;
    ensure_eq(
        inspection.schema_version,
        REVIEW_WORKSPACE_BETA_SCHEMA_VERSION,
        "inspection.schema_version",
    )?;
    ensure_eq(
        inspection.review_workspace_id_ref.as_str(),
        workspace_id,
        "inspection.review_workspace_id_ref",
    )?;
    ensure_eq(
        inspection.durable_comment_anchor_count,
        packet.durable_comment_anchors.len(),
        "inspection.durable_comment_anchor_count",
    )?;
    ensure_eq(
        inspection.object_lineage_count,
        packet.object_lineage.len(),
        "inspection.object_lineage_count",
    )?;
    ensure_eq(
        inspection.check_freshness_count,
        packet.check_freshness.len(),
        "inspection.check_freshness_count",
    )?;
    ensure_eq(
        inspection.anchor_identity_preserved,
        packet.preserves_anchor_identity(),
        "inspection.anchor_identity_preserved",
    )?;
    ensure_eq(
        inspection.check_freshness_browser_independent,
        packet.check_freshness_is_browser_independent(),
        "inspection.check_freshness_browser_independent",
    )?;
    ensure_eq(
        inspection.typed_reversible_browser_handoff_present,
        packet.has_typed_reversible_browser_handoff(),
        "inspection.typed_reversible_browser_handoff_present",
    )?;
    ensure_eq(
        inspection.support_export_reopenable,
        packet.support_export_can_reopen_context(),
        "inspection.support_export_reopenable",
    )?;
    ensure_eq(
        inspection.raw_escape_hatches_absent,
        packet.raw_escape_hatches_absent(),
        "inspection.raw_escape_hatches_absent",
    )?;
    ensure_eq(
        inspection.operator_truth_current,
        packet.operator_truth_current(),
        "inspection.operator_truth_current",
    )?;
    ensure_eq(
        inspection.stale_check_blocks_operator_truth,
        packet
            .check_freshness
            .iter()
            .any(|check| check.blocks_operator_truth_claim_when_stale),
        "inspection.stale_check_blocks_operator_truth",
    )
}

fn support_export_reopenable(
    support_export: &ReviewWorkspaceSupportExportPacket,
    durable_comment_anchors: &[ReviewWorkspaceDurableCommentAnchorRecord],
    check_freshness: &[ReviewWorkspaceCheckFreshnessRecord],
    object_lineage: &[ReviewWorkspaceObjectLineageRecord],
) -> bool {
    !support_export.reopen_context_ref.trim().is_empty()
        && !support_export.reopen_command_id_ref.trim().is_empty()
        && !durable_comment_anchors.is_empty()
        && !check_freshness.is_empty()
        && !object_lineage.is_empty()
        && support_export.raw_comment_body_export_allowed == false
        && support_export.raw_url_export_allowed == false
        && support_export.raw_source_body_export_allowed == false
}

fn validate_anchor_drift_pair(
    state: &str,
    action: &str,
    remap_chain_target_id_refs: &[String],
    archived_at: Option<&str>,
) -> Result<(), ReviewWorkspaceBetaValidationError> {
    if !contains_token(REVIEW_WORKSPACE_BETA_ANCHOR_DRIFT_STATES, state) {
        return Err(beta_validation_error(format!(
            "unsupported anchor_drift_state '{state}'"
        )));
    }
    if !contains_token(REVIEW_WORKSPACE_BETA_ANCHOR_REQUIRED_ACTIONS, action) {
        return Err(beta_validation_error(format!(
            "unsupported anchor_drift_required_user_action '{action}'"
        )));
    }
    let expected = match state {
        "anchor_bound_exact" | "anchor_remapped_with_recorded_mapping" => {
            "no_user_action_required_anchor_bound_or_remapped"
        }
        "anchor_drifted_user_must_resolve" => "user_must_pick_successor_or_dismiss_drifted",
        "anchor_target_deleted_re_anchor_or_resolve" => {
            "user_must_re_anchor_or_resolve_deleted_target"
        }
        "anchor_scope_unavailable" => "user_must_widen_scope_or_load_pack_or_reach_remote",
        "anchor_archived_tombstone" => "user_must_restore_from_archive_or_acknowledge_tombstone",
        _ => unreachable!("state token checked above"),
    };
    if action != expected {
        return Err(beta_validation_error(format!(
            "anchor drift state '{state}' requires action '{expected}'"
        )));
    }
    if state == "anchor_remapped_with_recorded_mapping" && remap_chain_target_id_refs.is_empty() {
        return Err(beta_validation_error(
            "remapped anchors must cite remap_chain_target_id_refs",
        ));
    }
    if state != "anchor_remapped_with_recorded_mapping" && !remap_chain_target_id_refs.is_empty() {
        return Err(beta_validation_error(
            "non-remapped anchors must not cite remap_chain_target_id_refs",
        ));
    }
    if state == "anchor_archived_tombstone" && archived_at.is_none() {
        return Err(beta_validation_error(
            "archived anchors must cite archived_at",
        ));
    }
    if state != "anchor_archived_tombstone" && archived_at.is_some() {
        return Err(beta_validation_error(
            "non-archived anchors must not cite archived_at",
        ));
    }
    Ok(())
}

fn freshness_blocks_operator_truth(class: &str) -> bool {
    matches!(
        class,
        "check_stale_blocks_operator_truth" | "check_unavailable_blocks_operator_truth"
    )
}

fn is_typed_reversible_handoff(handoff: &ReviewWorkspaceBrowserHandoffRecord) -> bool {
    contains_token(
        REVIEW_WORKSPACE_BETA_HANDOFF_DESTINATION_CLASSES,
        &handoff.destination_class,
    ) && contains_token(
        REVIEW_WORKSPACE_BETA_HANDOFF_REASON_CODES,
        &handoff.reason_code,
    ) && contains_token(
        REVIEW_WORKSPACE_BETA_HANDOFF_REPLAY_POSTURES,
        &handoff.replay_posture,
    ) && handoff.reversible_handoff
        && !handoff.return_anchor_ref.trim().is_empty()
        && !handoff.return_anchor_kind.trim().is_empty()
        && !looks_like_raw_url(&handoff.destination_ref)
        && !handoff.raw_url_export_allowed
}

fn looks_like_raw_url(value: &str) -> bool {
    let lower = value.trim().to_ascii_lowercase();
    lower.starts_with("http://")
        || lower.starts_with("https://")
        || lower.starts_with("ssh://")
        || lower.starts_with("git@")
}

fn contains_token(tokens: &[&str], value: &str) -> bool {
    tokens.iter().any(|token| token == &value)
}

fn ensure_nonempty(value: &str, field: &str) -> Result<(), ReviewWorkspaceBetaValidationError> {
    if value.trim().is_empty() {
        Err(beta_validation_error(format!("{field} must not be empty")))
    } else {
        Ok(())
    }
}

fn ensure_eq<T>(
    actual: T,
    expected: T,
    field: &str,
) -> Result<(), ReviewWorkspaceBetaValidationError>
where
    T: Copy + PartialEq + fmt::Display,
{
    if actual == expected {
        Ok(())
    } else {
        Err(beta_validation_error(format!(
            "{field} expected '{expected}' but got '{actual}'"
        )))
    }
}

fn beta_validation_error(message: impl Into<String>) -> ReviewWorkspaceBetaValidationError {
    ReviewWorkspaceBetaValidationError {
        message: message.into(),
    }
}

fn workspace_record(
    input: &ReviewWorkspaceSeedInput,
    diff_packet: &DiffViewSurfacePacket,
) -> ReviewWorkspaceRecord {
    let provider_overlay = input
        .provider_overlay
        .as_ref()
        .map(|overlay| ReviewProviderOverlay {
            provider_class: overlay.provider_class.clone(),
            connected_provider_record_id_ref: overlay.connected_provider_record_id_ref.clone(),
            provider_object_identity_ref: overlay.provider_object_identity_ref.clone(),
            provider_overlay_freshness_class: overlay.provider_overlay_freshness_class.clone(),
            last_fetched_at: overlay.last_fetched_at.clone(),
            grace_window_seconds: overlay.grace_window_seconds,
        });
    let source_class = if provider_overlay.is_some() {
        "composite_local_with_provider_overlay"
    } else {
        "local_branch_or_worktree"
    };
    let provider_authority = if provider_overlay.is_some() {
        "local_parity_estimate"
    } else {
        "local_truth_only_no_provider_overlay"
    };
    ReviewWorkspaceRecord {
        record_kind: REVIEW_WORKSPACE_RECORD_KIND.to_string(),
        review_workspace_schema_version: REVIEW_WORKSPACE_SCHEMA_VERSION,
        review_workspace_id: input.review_workspace_id.clone(),
        review_workspace_source_class: source_class.to_string(),
        provider_authority_class: provider_authority.to_string(),
        review_workspace_lifecycle_state: "open_under_review".to_string(),
        local_locator: Some(ReviewLocalLocator {
            workspace_id_ref: diff_packet.workspace_ref.clone(),
            branch_or_worktree_ref: input.branch_or_worktree_ref.clone(),
            base_revision_ref: input.base_revision_ref.clone(),
            head_revision_ref: input.head_revision_ref.clone(),
        }),
        provider_overlay,
        imported_bundle_envelope: None,
        browser_handoff_envelope: None,
        policy_context: policy_context(input),
        client_scopes: scopes_or_default(&input.client_scopes),
        redaction_class: "metadata_safe_default".to_string(),
        freshness_class: if input.provider_overlay.is_some() {
            "warm_cached"
        } else {
            "authoritative_live"
        }
        .to_string(),
        summary_label: format!(
            "Local review workspace for {}",
            diff_packet.compare_target.exact_target_label
        ),
        created_at: input.created_at.clone(),
        updated_at: input.created_at.clone(),
        archived_at: None,
        hosted_review_inbox_record_id_ref: None,
        merge_policy_record_id_ref: None,
    }
}

fn anchors_for_diff(
    input: &ReviewWorkspaceSeedInput,
    diff_packet: &DiffViewSurfacePacket,
) -> Vec<ReviewAnchorIdAlphaRecord> {
    diff_packet
        .hunks
        .iter()
        .flat_map(|hunk| {
            hunk.rows
                .iter()
                .map(move |row| anchor_for_row(input, diff_packet, &hunk.hunk_header, row))
        })
        .collect()
}

fn anchor_for_row(
    input: &ReviewWorkspaceSeedInput,
    diff_packet: &DiffViewSurfacePacket,
    hunk_header: &str,
    row: &DiffLineView,
) -> ReviewAnchorIdAlphaRecord {
    let fallback_context_hash = stable_hash(&format!(
        "{}\n{}\n{}\n{}",
        diff_packet.path_truth.path_label,
        hunk_header,
        row.line_kind.as_str(),
        row.raw_text
    ));
    let old_line = line_number_token(row.old_line_number);
    let new_line = line_number_token(row.new_line_number);
    let stable_identity_fields = vec![
        "review_workspace_id_ref".to_string(),
        "path_truth_ref".to_string(),
        "compare_target_ref".to_string(),
        "anchor_target_kind".to_string(),
        "line_kind".to_string(),
        "old_line_number".to_string(),
        "new_line_number".to_string(),
        "fallback_context_hash".to_string(),
    ];
    let anchor_hash = stable_hash(&format!(
        "{}|{}|{}|{}|{}|{}|{}|{}",
        input.review_workspace_id,
        row.path_truth_ref,
        row.compare_target_ref,
        "text_line_range_anchor",
        row.line_kind.as_str(),
        old_line,
        new_line,
        fallback_context_hash
    ));
    let anchor_id = format!(
        "review.anchor.alpha.{}.{}",
        sanitize_id(&input.review_workspace_id),
        anchor_hash
    );

    ReviewAnchorIdAlphaRecord {
        record_kind: REVIEW_ANCHOR_ID_ALPHA_RECORD_KIND.to_string(),
        schema_version: REVIEW_ANCHOR_ID_ALPHA_SCHEMA_VERSION,
        anchor_id,
        review_workspace_id_ref: input.review_workspace_id.clone(),
        diff_surface_ref: diff_packet.diff_surface_ref.clone(),
        target_ref: row.row_ref.clone(),
        path_truth_ref: row.path_truth_ref.clone(),
        compare_target_ref: row.compare_target_ref.clone(),
        anchor_target_kind: "text_line_range_anchor".to_string(),
        review_artifact_class: "line_oriented_source_artifact".to_string(),
        review_surface_class: "line_oriented_compare_viewer".to_string(),
        line_kind: row.line_kind.as_str().to_string(),
        old_line_number: row.old_line_number,
        new_line_number: row.new_line_number,
        fallback_context_hash,
        stable_identity_fields,
        provider_excluded_from_anchor_hash: true,
        anchor_drift_state: "anchor_bound_exact".to_string(),
        anchor_drift_required_user_action: "no_user_action_required_anchor_bound_or_remapped"
            .to_string(),
        summary_label: summary_for_anchor(row),
    }
}

fn work_item_linkages_for(
    input: &ReviewWorkspaceSeedInput,
    anchors: &[ReviewAnchorIdAlphaRecord],
) -> Vec<ReviewWorkItemLinkageRecord> {
    let anchor_refs = anchors
        .iter()
        .map(|anchor| anchor.anchor_id.clone())
        .collect::<Vec<_>>();
    input
        .work_item_links
        .iter()
        .map(|link| {
            let linked_review_class = if input.provider_overlay.is_some() {
                "linked_review_workspace_with_provider_overlay"
            } else {
                "linked_review_workspace_local_truth_only"
            };
            ReviewWorkItemLinkageRecord {
                record_kind: REVIEW_WORK_ITEM_LINKAGE_RECORD_KIND.to_string(),
                schema_version: REVIEW_WORK_ITEM_LINKAGE_SCHEMA_VERSION,
                linkage_ref: format!(
                    "review.work_item_link.{}.{}",
                    sanitize_id(&input.review_workspace_id),
                    sanitize_id(&link.work_item_detail_record_id_ref)
                ),
                work_item_detail_record_id_ref: link.work_item_detail_record_id_ref.clone(),
                target_object_identity_ref: link.target_object_identity_ref.clone(),
                linked_review_workspace_record_id_ref: input.review_workspace_id.clone(),
                linked_review_anchor_id_refs: anchor_refs.clone(),
                work_item_authority_class: link.work_item_authority_class.clone(),
                write_authority_class: link.write_authority_class.clone(),
                issue_to_branch_link_class: link.issue_to_branch_link_class.clone(),
                linked_review_class: linked_review_class.to_string(),
                actor_ref: link.actor_ref.clone(),
                command_id_ref: link.command_id_ref.clone(),
                linked_at: link.linked_at.clone(),
                source_schema_refs: vec![
                    "schemas/work_items/work_item_detail.schema.json".to_string(),
                    "schemas/vcs/review_workspace.schema.json".to_string(),
                    "schemas/review/anchor_id_alpha.schema.json".to_string(),
                ],
                summary_label: link.summary_label.clone(),
            }
        })
        .collect()
}

fn inspection_for(
    workspace: &ReviewWorkspaceRecord,
    diff_entries: &[ReviewWorkspaceDiffEntry],
    anchors: &[ReviewAnchorIdAlphaRecord],
    work_item_linkages: &[ReviewWorkItemLinkageRecord],
    graph_cue_packets: &[GraphFactCuePacket],
) -> ReviewWorkspaceInspectionRecord {
    let provider_ready_anchor_semantics = anchors
        .iter()
        .all(|anchor| anchor.provider_excluded_from_anchor_hash);
    let attributable_work_item_linkage_present = work_item_linkages.iter().any(|linkage| {
        !linkage.actor_ref.trim().is_empty() && !linkage.command_id_ref.trim().is_empty()
    });
    ReviewWorkspaceInspectionRecord {
        record_kind: REVIEW_WORKSPACE_INSPECTION_RECORD_KIND.to_string(),
        schema_version: REVIEW_WORKSPACE_INSPECTION_SCHEMA_VERSION,
        review_workspace_id_ref: workspace.review_workspace_id.clone(),
        diff_entry_count: diff_entries.len(),
        anchor_count: anchors.len(),
        work_item_linkage_count: work_item_linkages.len(),
        provider_ready_anchor_semantics,
        attributable_work_item_linkage_present,
        graph_cue_packet_refs: graph_cue_packets
            .iter()
            .map(|packet| packet.packet_id.clone())
            .collect(),
        graph_cue_readiness_tokens: graph_cue_packets
            .iter()
            .map(|packet| packet.readiness.clone())
            .collect(),
        graph_cue_epoch_refs: graph_cue_packets
            .iter()
            .map(|packet| packet.emitted_at.clone())
            .collect(),
        summary_label: format!(
            "{} diff(s), {} anchor(s), {} work-item link(s)",
            diff_entries.len(),
            anchors.len(),
            work_item_linkages.len()
        ),
    }
}

fn policy_context(input: &ReviewWorkspaceSeedInput) -> ReviewPolicyContext {
    ReviewPolicyContext {
        policy_epoch: input.policy_epoch.clone(),
        trust_state: input.trust_state.clone(),
        execution_context_id: input.execution_context_id.clone(),
        workspace_trust_state_class: input.trust_state.clone(),
    }
}

fn scopes_or_default(scopes: &[String]) -> Vec<String> {
    if scopes.is_empty() {
        vec!["desktop_product".to_string()]
    } else {
        scopes.to_vec()
    }
}

fn summary_for_anchor(row: &DiffLineView) -> String {
    let line = row
        .new_line_number
        .or(row.old_line_number)
        .map(|line| line.to_string())
        .unwrap_or_else(|| "unnumbered".to_string());
    format!(
        "{} {} line {}",
        row.path_label,
        line_kind_label(row.line_kind),
        line
    )
}

fn line_kind_label(kind: DiffLineKind) -> &'static str {
    match kind {
        DiffLineKind::Context => "context",
        DiffLineKind::Addition => "addition",
        DiffLineKind::Deletion => "deletion",
    }
}

fn line_number_token(line: Option<u32>) -> String {
    line.map(|line| line.to_string())
        .unwrap_or_else(|| "none".to_string())
}

fn stable_hash(value: &str) -> String {
    let mut hash = 0xcbf29ce484222325u64;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn sanitize_id(value: &str) -> String {
    let mut out = String::new();
    let mut last_sep = true;
    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_sep = false;
            continue;
        }
        if last_sep {
            continue;
        }
        out.push('-');
        last_sep = true;
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "root".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::diff::{DiffFileInput, DiffHunkInput, DiffLineInput, DiffOpenTarget, DiffViewMode};

    use super::*;

    fn diff_packet() -> DiffViewSurfacePacket {
        let input = DiffFileInput {
            workspace_ref: "workspace.unit.review".to_string(),
            truth_source_ref: "git.status.snapshot.unit.review".to_string(),
            repo_root: PathBuf::from("/workspace/unit"),
            logical_root_ref: "root.local.unit".to_string(),
            worktree_ref: "worktree.local.unit".to_string(),
            group_token: "unstaged".to_string(),
            path: PathBuf::from("src/lib.rs"),
            original_path: None,
            status_code: ".M".to_string(),
            language_id: Some("rust".to_string()),
            view_mode: DiffViewMode::Unified,
            generated_at: "2026-05-13T00:00:00Z".to_string(),
            hunks: vec![DiffHunkInput {
                hunk_header: "@@ -1,2 +1,3 @@".to_string(),
                old_start: 1,
                old_lines: 2,
                new_start: 1,
                new_lines: 3,
                lines: vec![
                    DiffLineInput {
                        line_kind: DiffLineKind::Context,
                        old_line_number: Some(1),
                        new_line_number: Some(1),
                        raw_text: "pub fn demo() {".to_string(),
                    },
                    DiffLineInput {
                        line_kind: DiffLineKind::Addition,
                        old_line_number: None,
                        new_line_number: Some(2),
                        raw_text: "    trace_review();".to_string(),
                    },
                ],
            }],
        };
        let open_target = DiffOpenTarget::from_change_list_row_parts(
            &input.workspace_ref,
            &input.truth_source_ref,
            "git.change.row.unit.review.unstaged.src-lib-rs.modified",
            &input.group_token,
            input.path.clone(),
            input.original_path.clone(),
            &input.status_code,
            "modified",
        );
        DiffViewSurfacePacket::from_file_input(open_target, input)
    }

    fn seed_input() -> ReviewWorkspaceSeedInput {
        ReviewWorkspaceSeedInput {
            review_workspace_id: "review.git.workspace.unit".to_string(),
            branch_or_worktree_ref: "worktree.local.unit".to_string(),
            base_revision_ref: Some("git.rev.base".to_string()),
            head_revision_ref: Some("git.rev.head".to_string()),
            actor_ref: "actor.local.dev".to_string(),
            policy_epoch: "policy.epoch.unit".to_string(),
            trust_state: "trusted".to_string(),
            execution_context_id: Some("exec.ctx.unit".to_string()),
            client_scopes: vec!["desktop_product".to_string()],
            created_at: "2026-05-13T00:00:00Z".to_string(),
            provider_overlay: None,
            work_item_links: vec![ReviewWorkItemLinkInput {
                work_item_detail_record_id_ref: "work_item.detail.unit".to_string(),
                target_object_identity_ref: "provider.object.issue.unit".to_string(),
                work_item_authority_class: "linked_review_only_no_provider_overlay".to_string(),
                write_authority_class: "write_admissible_local_draft_only_no_provider_path"
                    .to_string(),
                issue_to_branch_link_class: "linked_local_branch_or_worktree_no_provider_overlay"
                    .to_string(),
                actor_ref: "actor.local.dev".to_string(),
                command_id_ref: "cmd:review.workspace.link_work_item".to_string(),
                linked_at: "2026-05-13T00:00:00Z".to_string(),
                summary_label: "Work item linked to local review workspace".to_string(),
            }],
            graph_cue_packets: Vec::new(),
        }
    }

    #[test]
    fn anchors_are_stable_when_provider_overlay_is_added() {
        let diff_packet = diff_packet();
        let local = ReviewWorkspaceSeedPacket::from_diff_packet(seed_input(), &diff_packet);
        let overlay = ReviewProviderOverlayInput {
            provider_class: "review_or_code_host".to_string(),
            connected_provider_record_id_ref: "connected_provider.github.unit".to_string(),
            provider_object_identity_ref: "provider.review.unit".to_string(),
            provider_overlay_freshness_class: "provider_overlay_fresh".to_string(),
            last_fetched_at: "2026-05-13T00:00:00Z".to_string(),
            grace_window_seconds: Some(600),
        };
        let hosted = ReviewWorkspaceSeedPacket::from_diff_packet(
            seed_input().with_provider_overlay(overlay),
            &diff_packet,
        );

        let local_ids = local
            .anchors
            .iter()
            .map(|anchor| anchor.anchor_id.as_str())
            .collect::<Vec<_>>();
        let hosted_ids = hosted
            .anchors
            .iter()
            .map(|anchor| anchor.anchor_id.as_str())
            .collect::<Vec<_>>();

        assert_eq!(local_ids, hosted_ids);
        assert!(local.provider_ready_anchor_semantics());
        assert_eq!(
            hosted.review_workspace.review_workspace_source_class,
            "composite_local_with_provider_overlay"
        );
    }

    #[test]
    fn work_item_linkage_is_reviewable_and_attributed() {
        let packet = ReviewWorkspaceSeedPacket::from_diff_packet(seed_input(), &diff_packet());

        assert!(packet.every_diff_entry_has_stable_anchors());
        assert!(packet.has_attributable_work_item_linkage());
        assert_eq!(
            packet.work_item_linkages[0].linked_review_class,
            "linked_review_workspace_local_truth_only"
        );
        assert_eq!(packet.inspection.anchor_count, 2);
        assert!(packet.inspection.attributable_work_item_linkage_present);
    }
}
