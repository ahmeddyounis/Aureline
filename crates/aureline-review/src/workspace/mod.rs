//! Review-workspace seed packets for local diff discussion.
//!
//! This module consumes the local Git review seed and diff-view packets, then
//! materializes the first review-workspace packet with deterministic row anchor
//! IDs and work-item relation rows. It keeps provider overlay fields out of the
//! anchor hash so the same anchors can later be published to hosted review
//! providers without changing their meaning.

use serde::{Deserialize, Serialize};

use aureline_graph::GraphFactCuePacket;

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
