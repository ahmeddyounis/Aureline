//! Review-lane consumer of the shared dense-collection contract.
//!
//! Review workspaces expose diff anchors as stable collection items, then
//! reuse the collection selection and batch-review records from
//! `aureline-search` so review queues do not define private select-all or
//! blocked-member semantics.

use std::collections::BTreeSet;

use aureline_search::{
    BatchActionClass, BatchExecutionOriginClass, BatchMemberDisposition, BatchReviewMember,
    BatchReviewSheet, CollectionCountStatus, CollectionFilterAst, CollectionFilterClause,
    CollectionFilterLiteral, CollectionFilterOperator, CollectionFilterSourceClass,
    CollectionScopeCounters, CollectionSelectionState, CollectionSurfaceFamily,
    CollectionViewAlphaRecord, SelectionScopeClass, StableCollectionItemRef,
};
use serde::{Deserialize, Serialize};

use crate::workspace::ReviewWorkspaceSeedPacket;

/// Stable record-kind tag for [`ReviewCollectionAlphaPacket`].
pub const REVIEW_COLLECTION_ALPHA_PACKET_RECORD_KIND: &str = "review_collection_alpha_packet";

/// Schema version for review collection projections.
pub const REVIEW_COLLECTION_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Inputs for projecting a review workspace into collection and batch-review state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewCollectionAlphaInput {
    /// Stable collection view identity.
    pub collection_view_id: String,
    /// Stable batch-review identity.
    pub batch_review_id: String,
    /// Stable action id for the reviewed action.
    pub action_id: String,
    /// Human-readable action label.
    pub action_label: String,
    /// Batch action class.
    pub action_class: BatchActionClass,
    /// Selection scope class for the reviewed action.
    pub selection_scope_class: SelectionScopeClass,
    /// Execution origin class for the reviewed action.
    pub execution_origin_class: BatchExecutionOriginClass,
    /// Stable selected anchor ids.
    pub selected_anchor_id_refs: Vec<String>,
    /// Selected anchor ids blocked by policy, authority, or stale basis.
    #[serde(default)]
    pub blocked_anchor_id_refs: Vec<String>,
    /// Selected anchor ids hidden outside the current filter or viewport.
    #[serde(default)]
    pub hidden_anchor_id_refs: Vec<String>,
    /// Selected anchor ids stale relative to the current review basis.
    #[serde(default)]
    pub stale_anchor_id_refs: Vec<String>,
    /// Timestamp or deterministic fixture clock.
    pub generated_at: String,
}

/// Review collection packet consumed by review, support, and batch sheets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewCollectionAlphaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this projection.
    pub schema_version: u32,
    /// Timestamp or deterministic fixture clock.
    pub generated_at: String,
    /// Review workspace backing the collection.
    pub review_workspace_id_ref: String,
    /// Shared collection view record.
    pub collection_view: CollectionViewAlphaRecord,
    /// Shared batch-review sheet record.
    pub batch_review_sheet: BatchReviewSheet,
}

impl ReviewCollectionAlphaPacket {
    /// Builds review collection state from a review-workspace seed packet.
    pub fn from_workspace_seed(
        input: ReviewCollectionAlphaInput,
        seed: &ReviewWorkspaceSeedPacket,
    ) -> Self {
        let blocked = input
            .blocked_anchor_id_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let hidden = input
            .hidden_anchor_id_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let stale = input
            .stale_anchor_id_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let selected = input
            .selected_anchor_id_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();

        let filter_ast = review_filter_ast(&input, seed);
        let counters = CollectionScopeCounters::from_known_values(
            seed.diff_entries.len() as u64,
            seed.anchors.len() as u64,
            seed.anchors.len() as u64,
            input.selected_anchor_id_refs.len() as u64,
            blocked.len() as u64,
            hidden.len() as u64,
            0,
            hidden.len() as u64,
            CollectionCountStatus::Exact,
        );
        let selection_state = CollectionSelectionState::explicit_identity_set(
            format!("selection:{}", input.collection_view_id),
            input.collection_view_id.clone(),
            input.selected_anchor_id_refs.clone(),
            input.selected_anchor_id_refs.first().cloned(),
            hidden.len() as u64,
            blocked.len() as u64,
            stale.len() as u64,
        );
        let item_id_refs = seed
            .anchors
            .iter()
            .map(|anchor| anchor.anchor_id.clone())
            .collect::<Vec<_>>();
        let collection_view = CollectionViewAlphaRecord::from_explicit_parts(
            input.collection_view_id.clone(),
            CollectionSurfaceFamily::ReviewCollection,
            seed.review_workspace.summary_label.clone(),
            filter_ast,
            None,
            counters.clone(),
            selection_state,
            item_id_refs,
        );
        let members = seed
            .anchors
            .iter()
            .map(|anchor| {
                let item = StableCollectionItemRef::new(
                    anchor.anchor_id.clone(),
                    CollectionSurfaceFamily::ReviewCollection,
                    anchor.target_ref.clone(),
                    anchor.summary_label.clone(),
                )
                .with_blocked(blocked.contains(&anchor.anchor_id))
                .with_hidden(hidden.contains(&anchor.anchor_id))
                .with_stale(stale.contains(&anchor.anchor_id));
                let disposition = if blocked.contains(&anchor.anchor_id) {
                    BatchMemberDisposition::Blocked
                } else if hidden.contains(&anchor.anchor_id) {
                    BatchMemberDisposition::Hidden
                } else if stale.contains(&anchor.anchor_id) {
                    BatchMemberDisposition::Stale
                } else if selected.contains(&anchor.anchor_id) {
                    BatchMemberDisposition::Included
                } else {
                    BatchMemberDisposition::Excluded
                };
                BatchReviewMember {
                    item,
                    disposition,
                    reason_label: review_member_reason(disposition).to_string(),
                }
            })
            .collect::<Vec<_>>();
        let batch_review_sheet = BatchReviewSheet::from_members(
            input.batch_review_id,
            input.collection_view_id,
            input.action_id,
            input.action_label,
            input.action_class,
            input.selection_scope_class,
            input.execution_origin_class,
            counters,
            members,
            "Review workspace changes remain local until the reviewed action continues.",
        );

        Self {
            record_kind: REVIEW_COLLECTION_ALPHA_PACKET_RECORD_KIND.to_string(),
            schema_version: REVIEW_COLLECTION_ALPHA_SCHEMA_VERSION,
            generated_at: input.generated_at,
            review_workspace_id_ref: seed.review_workspace.review_workspace_id.clone(),
            collection_view,
            batch_review_sheet,
        }
    }

    /// True when the packet preserves selection by stable review anchor id.
    pub fn preserves_anchor_identity_selection(&self) -> bool {
        self.collection_view
            .selection_state
            .selected_item_id_refs
            .iter()
            .all(|selected| self.collection_view.item_id_refs.contains(selected))
    }
}

fn review_filter_ast(
    input: &ReviewCollectionAlphaInput,
    seed: &ReviewWorkspaceSeedPacket,
) -> CollectionFilterAst {
    let mut clauses = vec![CollectionFilterClause::new(
        "review.workspace",
        "review_workspace",
        "Review workspace",
        CollectionFilterOperator::Equals,
        Some(CollectionFilterLiteral::redacted(
            seed.review_workspace.review_workspace_id.clone(),
        )),
        CollectionFilterSourceClass::Workset,
    )];
    if seed.review_workspace.policy_context.trust_state != "trusted" {
        clauses.push(CollectionFilterClause::new(
            "review.policy.trust",
            "trust_state",
            "Policy",
            CollectionFilterOperator::Equals,
            Some(CollectionFilterLiteral::redacted(
                seed.review_workspace.policy_context.trust_state.clone(),
            )),
            CollectionFilterSourceClass::Policy,
        ));
    }
    if let Some(provider_overlay) = &seed.review_workspace.provider_overlay {
        clauses.push(CollectionFilterClause::new(
            "review.provider.overlay",
            "provider_overlay_freshness",
            "Provider overlay",
            CollectionFilterOperator::Equals,
            Some(CollectionFilterLiteral::redacted(
                provider_overlay.provider_overlay_freshness_class.clone(),
            )),
            CollectionFilterSourceClass::ProviderLimit,
        ));
    }
    if !input.hidden_anchor_id_refs.is_empty() {
        clauses.push(CollectionFilterClause::new(
            "review.selection.hidden",
            "hidden_selected",
            "Hidden selected",
            CollectionFilterOperator::GreaterOrEqual,
            Some(CollectionFilterLiteral::redacted(
                input.hidden_anchor_id_refs.len().to_string(),
            )),
            CollectionFilterSourceClass::ClientLimit,
        ));
    }
    CollectionFilterAst::from_clauses(
        format!("filter_ast:{}", input.collection_view_id),
        seed.review_workspace.summary_label.clone(),
        clauses,
        "aureline-review",
        input.generated_at.clone(),
    )
}

fn review_member_reason(disposition: BatchMemberDisposition) -> &'static str {
    match disposition {
        BatchMemberDisposition::Included => "Included in the reviewed action.",
        BatchMemberDisposition::Excluded => "Excluded from the current selection.",
        BatchMemberDisposition::Blocked => "Blocked before the action can continue.",
        BatchMemberDisposition::Hidden => "Hidden outside the current filter or viewport.",
        BatchMemberDisposition::Stale => "Stale relative to the current review basis.",
    }
}
