//! Extension and package-lane consumer of the shared dense-collection contract.
//!
//! Marketplace/package rows project install-review packets into the same
//! collection counters, selection state, and batch-review sheet used by search
//! and review surfaces. Hosted lanes remain read-only consumers of the native
//! review packet and cannot invent narrower approval semantics.

use std::collections::BTreeSet;

use aureline_search::{
    BatchActionClass, BatchExecutionOriginClass, BatchMemberDisposition, BatchReviewMember,
    BatchReviewSheet, CollectionCountStatus, CollectionFilterAst, CollectionFilterClause,
    CollectionFilterLiteral, CollectionFilterOperator, CollectionFilterSourceClass,
    CollectionScopeCounters, CollectionSelectionState, CollectionSurfaceFamily,
    CollectionViewAlphaRecord, SelectionScopeClass, StableCollectionItemRef,
};
use serde::{Deserialize, Serialize};

use crate::install_review::{InstallReviewActionClass, InstallReviewAlphaPacketRecord};

/// Stable record-kind tag for [`ExtensionInstallCollectionAlphaPacket`].
pub const EXTENSION_INSTALL_COLLECTION_ALPHA_PACKET_RECORD_KIND: &str =
    "extension_install_collection_alpha_packet";

/// Schema version for extension install collection projections.
pub const EXTENSION_INSTALL_COLLECTION_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Inputs for projecting an install-review packet into collection state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionInstallCollectionAlphaInput {
    /// Stable collection view identity.
    pub collection_view_id: String,
    /// Stable batch-review identity.
    pub batch_review_id: String,
    /// Selected package or extension subject refs.
    #[serde(default)]
    pub selected_subject_refs: Vec<String>,
    /// Subject refs hidden outside the current filter or compact client lane.
    #[serde(default)]
    pub hidden_subject_refs: Vec<String>,
    /// Subject refs blocked before the action can continue.
    #[serde(default)]
    pub blocked_subject_refs: Vec<String>,
    /// Subject refs stale relative to the current package/review basis.
    #[serde(default)]
    pub stale_subject_refs: Vec<String>,
    /// Timestamp or deterministic fixture clock.
    pub generated_at: String,
}

/// Package or extension collection packet consumed by marketplace and native review lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExtensionInstallCollectionAlphaPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this projection.
    pub schema_version: u32,
    /// Timestamp or deterministic fixture clock.
    pub generated_at: String,
    /// Install-review packet backing the collection row.
    pub review_id_ref: String,
    /// Shared collection view record.
    pub collection_view: CollectionViewAlphaRecord,
    /// Shared batch-review sheet record.
    pub batch_review_sheet: BatchReviewSheet,
}

impl ExtensionInstallCollectionAlphaPacket {
    /// Builds package/inventory collection state from an install-review packet.
    pub fn from_install_review_packet(
        input: ExtensionInstallCollectionAlphaInput,
        packet: &InstallReviewAlphaPacketRecord,
    ) -> Self {
        let selected_refs = if input.selected_subject_refs.is_empty() {
            vec![packet.subject_ref.clone()]
        } else {
            input.selected_subject_refs.clone()
        };
        let selected = selected_refs.iter().cloned().collect::<BTreeSet<_>>();
        let mut blocked = input
            .blocked_subject_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if !packet.mutation_allowed {
            blocked.insert(packet.subject_ref.clone());
        }
        let hidden = input
            .hidden_subject_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let stale = input
            .stale_subject_refs
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        let filter_ast = install_filter_ast(&input, packet);
        let counters = CollectionScopeCounters::from_known_values(
            1,
            1,
            1,
            selected.len() as u64,
            blocked.len() as u64,
            hidden.len() as u64,
            0,
            hidden.len() as u64,
            CollectionCountStatus::Exact,
        );
        let selection_state = CollectionSelectionState::explicit_identity_set(
            format!("selection:{}", input.collection_view_id),
            input.collection_view_id.clone(),
            selected_refs,
            Some(packet.subject_ref.clone()),
            hidden.len() as u64,
            blocked.len() as u64,
            stale.len() as u64,
        );
        let collection_view = CollectionViewAlphaRecord::from_explicit_parts(
            input.collection_view_id.clone(),
            CollectionSurfaceFamily::PackageOrInventoryGrid,
            format!("Package review · {}", packet.subject_ref),
            filter_ast,
            None,
            counters.clone(),
            selection_state,
            vec![packet.subject_ref.clone()],
        );
        let disposition = if blocked.contains(&packet.subject_ref) {
            BatchMemberDisposition::Blocked
        } else if hidden.contains(&packet.subject_ref) {
            BatchMemberDisposition::Hidden
        } else if stale.contains(&packet.subject_ref) {
            BatchMemberDisposition::Stale
        } else if selected.contains(&packet.subject_ref) {
            BatchMemberDisposition::Included
        } else {
            BatchMemberDisposition::Excluded
        };
        let member = BatchReviewMember {
            item: StableCollectionItemRef::new(
                packet.subject_ref.clone(),
                CollectionSurfaceFamily::PackageOrInventoryGrid,
                packet.review_id.clone(),
                packet.decision_summary.clone(),
            )
            .with_blocked(blocked.contains(&packet.subject_ref))
            .with_hidden(hidden.contains(&packet.subject_ref))
            .with_stale(stale.contains(&packet.subject_ref)),
            disposition,
            reason_label: install_member_reason(disposition).to_string(),
        };
        let batch_review_sheet = BatchReviewSheet::from_members(
            input.batch_review_id,
            input.collection_view_id,
            install_action_id(packet.action_class),
            install_action_label(packet.action_class),
            BatchActionClass::ProviderOwnedMutation,
            SelectionScopeClass::ExplicitCustomSet,
            BatchExecutionOriginClass::MixedClientThenProvider,
            counters,
            vec![member],
            "Open the native review sheet before changing install or enablement state.",
        );

        Self {
            record_kind: EXTENSION_INSTALL_COLLECTION_ALPHA_PACKET_RECORD_KIND.to_string(),
            schema_version: EXTENSION_INSTALL_COLLECTION_ALPHA_SCHEMA_VERSION,
            generated_at: input.generated_at,
            review_id_ref: packet.review_id.clone(),
            collection_view,
            batch_review_sheet,
        }
    }

    /// True when non-native package lanes expose a review sheet instead of direct approval.
    pub fn routes_mutation_through_review_sheet(&self) -> bool {
        self.batch_review_sheet.review_required
            && self
                .batch_review_sheet
                .included_item_id_refs
                .iter()
                .all(|included| self.collection_view.item_id_refs.contains(included))
    }
}

fn install_filter_ast(
    input: &ExtensionInstallCollectionAlphaInput,
    packet: &InstallReviewAlphaPacketRecord,
) -> CollectionFilterAst {
    let mut clauses = vec![
        CollectionFilterClause::new(
            "package.subject",
            "subject_ref",
            "Package",
            CollectionFilterOperator::Equals,
            Some(CollectionFilterLiteral::redacted(
                packet.subject_ref.clone(),
            )),
            CollectionFilterSourceClass::SavedView,
        ),
        CollectionFilterClause::new(
            "package.scope",
            "profile_scope",
            "Scope",
            CollectionFilterOperator::Equals,
            Some(CollectionFilterLiteral::redacted(
                packet.boundary_truth.profile_scope_ref.clone(),
            )),
            CollectionFilterSourceClass::ClientLimit,
        ),
        CollectionFilterClause::new(
            "package.review.authority",
            "native_review_authority",
            "Review authority",
            CollectionFilterOperator::Equals,
            Some(CollectionFilterLiteral::redacted(format!(
                "{:?}",
                packet.boundary_truth.canonical_review_authority_class
            ))),
            CollectionFilterSourceClass::Policy,
        ),
    ];
    if !packet.mutation_allowed {
        clauses.push(CollectionFilterClause::new(
            "package.mutation.blocked",
            "mutation_allowed",
            "Blocked",
            CollectionFilterOperator::Equals,
            Some(CollectionFilterLiteral::redacted("false")),
            CollectionFilterSourceClass::Policy,
        ));
    }
    if !input.hidden_subject_refs.is_empty() {
        clauses.push(CollectionFilterClause::new(
            "package.selection.hidden",
            "hidden_selected",
            "Hidden selected",
            CollectionFilterOperator::GreaterOrEqual,
            Some(CollectionFilterLiteral::redacted(
                input.hidden_subject_refs.len().to_string(),
            )),
            CollectionFilterSourceClass::ClientLimit,
        ));
    }
    CollectionFilterAst::from_clauses(
        format!("filter_ast:{}", input.collection_view_id),
        format!("Package review · {}", packet.subject_ref),
        clauses,
        "aureline-extensions",
        input.generated_at.clone(),
    )
}

fn install_action_id(action: InstallReviewActionClass) -> &'static str {
    match action {
        InstallReviewActionClass::Install => "package.install.review",
        InstallReviewActionClass::Enable => "package.enable.review",
        InstallReviewActionClass::Update => "package.update.review",
    }
}

fn install_action_label(action: InstallReviewActionClass) -> &'static str {
    match action {
        InstallReviewActionClass::Install => "Install package",
        InstallReviewActionClass::Enable => "Enable package",
        InstallReviewActionClass::Update => "Update package",
    }
}

fn install_member_reason(disposition: BatchMemberDisposition) -> &'static str {
    match disposition {
        BatchMemberDisposition::Included => "Included in the native review action.",
        BatchMemberDisposition::Excluded => "Excluded from the current package selection.",
        BatchMemberDisposition::Blocked => "Blocked by review, policy, or missing evidence.",
        BatchMemberDisposition::Hidden => "Hidden outside the current package lane.",
        BatchMemberDisposition::Stale => "Stale relative to the current package review basis.",
    }
}
