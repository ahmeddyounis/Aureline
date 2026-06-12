//! Shared work-item rows, provider chips, freshness disclosure, and relation strips.
//!
//! This module projects the canonical provider-owned [`super::WorkItemDetailRecord`]
//! into one row vocabulary that list, board, queue, search, detail-card, companion,
//! and incident consumers can reuse without flattening provider truth. The row keeps:
//!
//! - provider identity and scope visible through [`WorkItemProviderChip`];
//! - canonical object identity, title, exact provider lifecycle token, and owner or
//!   assignee disclosure on [`WorkItemObjectRowRecord`];
//! - local-draft, queued-publish, offline-captured, cached, and provider-committed
//!   truth distinct through [`WorkItemSyncScopeClass`];
//! - branch/worktree, review, run, incident, and validation evidence relations
//!   visible through [`WorkItemRelationStrip`] and [`WorkItemRelationStripItem`];
//! - export-safe support summaries that preserve provider kind, object kind, link
//!   state, sync scope, and relation identity without raw provider URLs or account
//!   material.
//!
//! The seeded packet built by [`seeded_work_item_object_rows_packet`] is the shared
//! truth source for first-consumer fixtures and tests.

#[cfg(test)]
mod tests;

use serde::{Deserialize, Serialize};

use super::{
    EngineeringArtifactRelations, LinkedReviewClass, ValidationEvidenceClass, WorkItemDetailRecord,
    WorkItemFreshnessClass, WorkItemObjectClass, WorkItemRowPostureClass,
};
use crate::registry::ProviderFamily;

/// Stable record-kind tag carried by [`WorkItemObjectRowsPacket`].
pub const WORK_ITEM_OBJECT_ROWS_PACKET_RECORD_KIND: &str = "work_item_object_rows_packet";

/// Stable record-kind tag carried by [`WorkItemObjectRowRecord`].
pub const WORK_ITEM_OBJECT_ROW_RECORD_KIND: &str = "work_item_object_row_record";

/// Schema version for work-item object rows.
pub const WORK_ITEM_OBJECT_ROWS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const WORK_ITEM_OBJECT_ROWS_SCHEMA_REF: &str = "schemas/work_items/object_rows.schema.json";

/// Repo-relative path of the contract doc.
pub const WORK_ITEM_OBJECT_ROWS_DOC_REF: &str = "docs/work_items/object_rows.md";

/// Repo-relative path of the protected fixture directory.
pub const WORK_ITEM_OBJECT_ROWS_FIXTURE_DIR: &str = "fixtures/work_items/object_rows";

/// Repo-relative path of the checked support-export artifact.
pub const WORK_ITEM_OBJECT_ROWS_ARTIFACT_REF: &str =
    "artifacts/provider/m5/work_item_object_rows/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const WORK_ITEM_OBJECT_ROWS_SUMMARY_REF: &str =
    "artifacts/provider/m5/work_item_object_rows.md";

/// Export-safe sync scope shown on a work-item row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemSyncScopeClass {
    /// Provider state is the committed source of truth.
    ProviderCommitted,
    /// Local draft exists but has not been provider committed.
    LocalDraftOnly,
    /// Local draft has been admitted to publish-later.
    QueuedPublish,
    /// Row is a captured offline packet or imported handoff snapshot.
    OfflineCaptured,
    /// Row is inspect-only cached provider state.
    CachedInspectOnly,
    /// Row is linked through local engineering context only.
    LocalRelationOnly,
}

impl WorkItemSyncScopeClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderCommitted => "provider_committed",
            Self::LocalDraftOnly => "local_draft_only",
            Self::QueuedPublish => "queued_publish",
            Self::OfflineCaptured => "offline_captured",
            Self::CachedInspectOnly => "cached_inspect_only",
            Self::LocalRelationOnly => "local_relation_only",
        }
    }

    /// Derives sync scope from a work-item detail row.
    pub fn from_detail(detail: &WorkItemDetailRecord) -> Self {
        match detail.row_posture_class {
            WorkItemRowPostureClass::ProviderAuthoritative => Self::ProviderCommitted,
            WorkItemRowPostureClass::CachedStale | WorkItemRowPostureClass::ReadOnly => {
                Self::CachedInspectOnly
            }
            WorkItemRowPostureClass::PolicyBlocked => {
                if detail.publish_later_queue_item_ref.is_some() {
                    Self::QueuedPublish
                } else if detail.local_draft_ref.is_some() {
                    Self::LocalDraftOnly
                } else {
                    Self::ProviderCommitted
                }
            }
            WorkItemRowPostureClass::LocalDraft => Self::LocalDraftOnly,
            WorkItemRowPostureClass::Queued => Self::QueuedPublish,
            WorkItemRowPostureClass::OfflineCaptured => Self::OfflineCaptured,
        }
    }
}

/// Export-safe rollup of relation-link state for a work-item row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemLinkStateClass {
    /// No engineering relation is linked.
    Unlinked,
    /// Exactly one engineering relation is linked.
    SingleRelationLinked,
    /// Multiple engineering relations are linked.
    MultiRelationLinked,
}

impl WorkItemLinkStateClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unlinked => "unlinked",
            Self::SingleRelationLinked => "single_relation_linked",
            Self::MultiRelationLinked => "multi_relation_linked",
        }
    }

    /// Derives link state from engineering relations.
    pub fn from_relations(relations: &EngineeringArtifactRelations) -> Self {
        match relation_identity_refs(relations).len() {
            0 => Self::Unlinked,
            1 => Self::SingleRelationLinked,
            _ => Self::MultiRelationLinked,
        }
    }
}

/// Provider/source chip shown beside a work-item row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemProviderChip {
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Reviewable provider label.
    pub provider_label: String,
    /// Project, board, or space ref.
    pub project_or_space_ref: String,
    /// Provider host ref.
    pub provider_host_ref: String,
    /// Tenant or org scope ref.
    pub tenant_or_org_scope_ref: String,
    /// Sync scope disclosed on the chip group.
    pub sync_scope_class: WorkItemSyncScopeClass,
    /// True when the provider path is inspect-only.
    pub inspect_only: bool,
}

/// Relation kind shown in the compact relation strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemRelationKindClass {
    /// Local branch or worktree relation.
    BranchOrWorktree,
    /// Linked review workspace or review pack relation.
    Review,
    /// Linked run or pipeline relation.
    Run,
    /// Linked incident workspace relation.
    Incident,
    /// Linked validation evidence relation.
    ValidationEvidence,
}

impl WorkItemRelationKindClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BranchOrWorktree => "branch_or_worktree",
            Self::Review => "review",
            Self::Run => "run",
            Self::Incident => "incident",
            Self::ValidationEvidence => "validation_evidence",
        }
    }
}

/// Source-of-truth posture disclosed for one relation strip item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemRelationSourceClass {
    /// Derived from the local workspace.
    LocalWorkspace,
    /// Derived from a provider overlay.
    ProviderOverlay,
    /// Derived from imported or offline handoff state.
    ImportedSnapshot,
    /// Derived from linked review state.
    DerivedReviewState,
    /// Derived from incident or runbook context.
    IncidentWorkspace,
}

impl WorkItemRelationSourceClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::ProviderOverlay => "provider_overlay",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::DerivedReviewState => "derived_review_state",
            Self::IncidentWorkspace => "incident_workspace",
        }
    }
}

/// Freshness class disclosed for one relation strip item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkItemRelationFreshnessClass {
    /// Live or current relation.
    Live,
    /// Cached but still inside the current grace window.
    Cached,
    /// Stale relation.
    Stale,
    /// Local draft relation not yet provider committed.
    LocalDraft,
    /// Imported snapshot relation with no live refresh path.
    ImportedSnapshot,
}

impl WorkItemRelationFreshnessClass {
    /// Stable token recorded in fixtures and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::LocalDraft => "local_draft",
            Self::ImportedSnapshot => "imported_snapshot",
        }
    }
}

/// One compact strip item shown under or beside a work-item row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemRelationStripItem {
    /// Stable strip-item id.
    pub relation_item_id: String,
    /// Relation kind.
    pub relation_kind: WorkItemRelationKindClass,
    /// Export-safe relation identity ref.
    pub relation_identity_ref: String,
    /// Source-of-truth posture for the relation.
    pub source_class: WorkItemRelationSourceClass,
    /// Freshness posture for the relation.
    pub freshness_class: WorkItemRelationFreshnessClass,
    /// Short redaction-safe summary.
    pub summary_label: String,
}

/// Compact relation strip shared by row and detail-card contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemRelationStrip {
    /// Strip items shown on the surface.
    pub items: Vec<WorkItemRelationStripItem>,
    /// Summary label suitable for support/export.
    pub summary_label: String,
}

/// Shared object row consumed by list, board, queue, search, detail-card, incident, and companion surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemObjectRowRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// Ref to the source detail record.
    pub work_item_detail_record_id_ref: String,
    /// Provider/source chip group.
    pub provider_chip: WorkItemProviderChip,
    /// Provider-side work-item object class.
    pub object_class: WorkItemObjectClass,
    /// Export-safe canonical id.
    pub canonical_id: String,
    /// Export-safe title label.
    pub title_label: String,
    /// Exact provider or local state token shown on the row.
    pub primary_state_label: String,
    /// Optional owner or assignee label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner_or_assignee_label: Option<String>,
    /// Freshness class shown on the row.
    pub freshness_class: WorkItemFreshnessClass,
    /// Export-safe freshness label.
    pub freshness_label: String,
    /// Row posture copied from the source detail record.
    pub row_posture_class: WorkItemRowPostureClass,
    /// Sync scope disclosed beside the row.
    pub sync_scope_class: WorkItemSyncScopeClass,
    /// True when the row carries a visible local-draft marker.
    pub local_draft_marker_visible: bool,
    /// Compact relation strip.
    pub relation_strip: WorkItemRelationStrip,
    /// Export-safe relation-link state rollup.
    pub link_state_class: WorkItemLinkStateClass,
    /// Export-safe summary.
    pub support_export_summary: String,
}

/// Packet grouping the shared object rows for first-consumer surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkItemObjectRowsPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// RFC 3339 timestamp the packet was generated at.
    pub generated_at: String,
    /// Source detail page id.
    pub source_page_id_ref: String,
    /// Shared object rows.
    pub rows: Vec<WorkItemObjectRowRecord>,
    /// Export-safe summary.
    pub summary_label: String,
}

/// Validation issue emitted by [`WorkItemObjectRowsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WorkItemObjectRowsViolation {
    /// The packet contains no rows.
    MissingRows,
    /// A row is missing a canonical id.
    MissingCanonicalId,
    /// A row is missing a primary state label.
    MissingPrimaryStateLabel,
    /// A local-only row is missing its draft marker.
    MissingLocalDraftMarker,
    /// A relation-bearing row dropped its relation strip.
    MissingRelationStrip,
}

impl WorkItemObjectRowsPacket {
    /// Validates the seeded/shared packet invariants.
    pub fn validate(&self) -> Vec<WorkItemObjectRowsViolation> {
        let mut violations = Vec::new();
        if self.rows.is_empty() {
            violations.push(WorkItemObjectRowsViolation::MissingRows);
        }
        for row in &self.rows {
            if row.canonical_id.trim().is_empty() {
                violations.push(WorkItemObjectRowsViolation::MissingCanonicalId);
            }
            if row.primary_state_label.trim().is_empty() {
                violations.push(WorkItemObjectRowsViolation::MissingPrimaryStateLabel);
            }
            if matches!(
                row.sync_scope_class,
                WorkItemSyncScopeClass::LocalDraftOnly
                    | WorkItemSyncScopeClass::QueuedPublish
                    | WorkItemSyncScopeClass::OfflineCaptured
            ) && !row.local_draft_marker_visible
            {
                violations.push(WorkItemObjectRowsViolation::MissingLocalDraftMarker);
            }
            if row.link_state_class != WorkItemLinkStateClass::Unlinked
                && row.relation_strip.items.is_empty()
            {
                violations.push(WorkItemObjectRowsViolation::MissingRelationStrip);
            }
        }
        violations
    }
}

/// Projects a work-item detail row into the shared row vocabulary.
pub fn project_work_item_object_row(detail: &WorkItemDetailRecord) -> WorkItemObjectRowRecord {
    let sync_scope_class = WorkItemSyncScopeClass::from_detail(detail);
    let relation_strip = relation_strip(detail);
    let link_state_class =
        WorkItemLinkStateClass::from_relations(&detail.engineering_artifact_relations);
    WorkItemObjectRowRecord {
        record_kind: WORK_ITEM_OBJECT_ROW_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_OBJECT_ROWS_SCHEMA_VERSION,
        row_id: format!("row:{}", detail.detail_id),
        work_item_detail_record_id_ref: detail.detail_id.clone(),
        provider_chip: WorkItemProviderChip {
            provider_family: detail.provider_family,
            provider_label: detail.provider_label.clone(),
            project_or_space_ref: detail.project_or_space_ref.clone(),
            provider_host_ref: detail.target_object_identity.provider_host.clone(),
            tenant_or_org_scope_ref: detail.target_object_identity.tenant_or_org_scope.clone(),
            sync_scope_class,
            inspect_only: matches!(
                detail.row_posture_class,
                WorkItemRowPostureClass::ReadOnly | WorkItemRowPostureClass::CachedStale
            ),
        },
        object_class: detail.target_object_identity.object_class,
        canonical_id: detail.canonical_id.clone(),
        title_label: detail.title_label.clone(),
        primary_state_label: detail
            .current_state_rows
            .iter()
            .find(|row| {
                matches!(
                    row.state_family_class,
                    super::StateFamilyClass::LifecycleState
                )
            })
            .map(|row| row.state_value.clone())
            .unwrap_or_else(|| "state_unknown".to_string()),
        owner_or_assignee_label: detail
            .owner_or_assignee_rows
            .iter()
            .find(|row| matches!(row.owner_role_class, super::OwnerRoleClass::Assignee))
            .or_else(|| detail.owner_or_assignee_rows.first())
            .map(|row| row.actor_label.clone()),
        freshness_class: detail.freshness_class,
        freshness_label: freshness_label(detail.freshness_class, &detail.freshness_observed_at),
        row_posture_class: detail.row_posture_class,
        sync_scope_class,
        local_draft_marker_visible: !matches!(
            sync_scope_class,
            WorkItemSyncScopeClass::ProviderCommitted
        ),
        relation_strip,
        link_state_class,
        support_export_summary: format!(
            "{} keeps provider {}, state {}, scope {}, and {} relation(s) visible.",
            detail.canonical_id,
            detail.provider_label,
            detail
                .current_state_rows
                .iter()
                .find(|row| matches!(
                    row.state_family_class,
                    super::StateFamilyClass::LifecycleState
                ))
                .map(|row| row.state_value.as_str())
                .unwrap_or("state_unknown"),
            sync_scope_class.as_str(),
            relation_identity_refs(&detail.engineering_artifact_relations).len()
        ),
    }
}

/// Builds the seeded packet for first-consumer object-row fixtures and tests.
pub fn seeded_work_item_object_rows_packet() -> WorkItemObjectRowsPacket {
    let page = super::seeded_work_item_transition_beta_page();
    WorkItemObjectRowsPacket {
        record_kind: WORK_ITEM_OBJECT_ROWS_PACKET_RECORD_KIND.to_string(),
        schema_version: WORK_ITEM_OBJECT_ROWS_SCHEMA_VERSION,
        packet_id: "providers.work_item_object_rows.packet".to_string(),
        generated_at: "2026-05-18T09:00:00Z".to_string(),
        source_page_id_ref: page.page_id,
        rows: page
            .detail_records
            .iter()
            .map(project_work_item_object_row)
            .collect(),
        summary_label: "Shared work-item rows preserve provider chips, canonical ids, provider-native state, freshness or local-draft truth, and compact relation strips for first-consumer surfaces.".to_string(),
    }
}

/// Returns export-safe relation identity refs in stable order.
pub fn relation_identity_refs(relations: &EngineeringArtifactRelations) -> Vec<String> {
    let mut refs = Vec::new();
    if let Some(branch_ref) = &relations.linked_branch_local_locator_ref {
        refs.push(branch_ref.clone());
    }
    if let Some(review_ref) = &relations.linked_review_workspace_record_id_ref {
        refs.push(review_ref.clone());
    }
    refs.extend(relations.linked_run_record_id_refs.clone());
    refs.extend(relations.linked_incident_workspace_record_id_refs.clone());
    refs.extend(relations.linked_validation_evidence_record_id_refs.clone());
    refs
}

fn relation_strip(detail: &WorkItemDetailRecord) -> WorkItemRelationStrip {
    let relation_freshness = relation_freshness(detail.freshness_class, detail.row_posture_class);
    let mut items = Vec::new();
    if let Some(branch_ref) = &detail
        .engineering_artifact_relations
        .linked_branch_local_locator_ref
    {
        items.push(WorkItemRelationStripItem {
            relation_item_id: format!("{}:branch", detail.detail_id),
            relation_kind: WorkItemRelationKindClass::BranchOrWorktree,
            relation_identity_ref: branch_ref.clone(),
            source_class: WorkItemRelationSourceClass::LocalWorkspace,
            freshness_class: relation_freshness,
            summary_label:
                "Branch or worktree relation remains visible with local source and freshness."
                    .to_string(),
        });
    }
    if let Some(review_ref) = &detail
        .engineering_artifact_relations
        .linked_review_workspace_record_id_ref
    {
        items.push(WorkItemRelationStripItem {
            relation_item_id: format!("{}:review", detail.detail_id),
            relation_kind: WorkItemRelationKindClass::Review,
            relation_identity_ref: review_ref.clone(),
            source_class: if detail.engineering_artifact_relations.linked_review_class
                == LinkedReviewClass::LinkedReviewWorkspaceLocalTruthOnly
            {
                WorkItemRelationSourceClass::DerivedReviewState
            } else {
                WorkItemRelationSourceClass::ProviderOverlay
            },
            freshness_class: relation_freshness,
            summary_label: "Review relation stays source-explicit instead of collapsing into generic linked-work copy.".to_string(),
        });
    }
    for run_ref in &detail
        .engineering_artifact_relations
        .linked_run_record_id_refs
    {
        items.push(WorkItemRelationStripItem {
            relation_item_id: format!("{}:run:{}", detail.detail_id, items.len()),
            relation_kind: WorkItemRelationKindClass::Run,
            relation_identity_ref: run_ref.clone(),
            source_class: WorkItemRelationSourceClass::ProviderOverlay,
            freshness_class: relation_freshness,
            summary_label:
                "Run or pipeline relation keeps provider-backed execution context attached."
                    .to_string(),
        });
    }
    for incident_ref in &detail
        .engineering_artifact_relations
        .linked_incident_workspace_record_id_refs
    {
        items.push(WorkItemRelationStripItem {
            relation_item_id: format!("{}:incident:{}", detail.detail_id, items.len()),
            relation_kind: WorkItemRelationKindClass::Incident,
            relation_identity_ref: incident_ref.clone(),
            source_class: WorkItemRelationSourceClass::IncidentWorkspace,
            freshness_class: relation_freshness,
            summary_label:
                "Incident relation stays visible without depending on a provider-only page."
                    .to_string(),
        });
    }
    if detail
        .engineering_artifact_relations
        .validation_evidence_class
        != ValidationEvidenceClass::NoValidationEvidenceAttached
    {
        for evidence_ref in &detail
            .engineering_artifact_relations
            .linked_validation_evidence_record_id_refs
        {
            items.push(WorkItemRelationStripItem {
                relation_item_id: format!("{}:validation:{}", detail.detail_id, items.len()),
                relation_kind: WorkItemRelationKindClass::ValidationEvidence,
                relation_identity_ref: evidence_ref.clone(),
                source_class: WorkItemRelationSourceClass::DerivedReviewState,
                freshness_class: relation_freshness,
                summary_label: "Validation evidence relation keeps linked checks and review evaluation visible.".to_string(),
            });
        }
    }

    WorkItemRelationStrip {
        summary_label: format!(
            "{} compact relation(s) stay visible on row and detail contexts.",
            items.len()
        ),
        items,
    }
}

fn relation_freshness(
    freshness: WorkItemFreshnessClass,
    row_posture: WorkItemRowPostureClass,
) -> WorkItemRelationFreshnessClass {
    match row_posture {
        WorkItemRowPostureClass::LocalDraft | WorkItemRowPostureClass::Queued => {
            WorkItemRelationFreshnessClass::LocalDraft
        }
        WorkItemRowPostureClass::OfflineCaptured => {
            WorkItemRelationFreshnessClass::ImportedSnapshot
        }
        _ => match freshness {
            WorkItemFreshnessClass::LiveAuthoritativeFresh => WorkItemRelationFreshnessClass::Live,
            WorkItemFreshnessClass::WarmWithinGrace => WorkItemRelationFreshnessClass::Cached,
            WorkItemFreshnessClass::DegradedBeyondGraceLocalContinues
            | WorkItemFreshnessClass::UnverifiableProviderUnreachable => {
                WorkItemRelationFreshnessClass::Stale
            }
            WorkItemFreshnessClass::ImportedSnapshotNoRefreshPath => {
                WorkItemRelationFreshnessClass::ImportedSnapshot
            }
            WorkItemFreshnessClass::LocalDraftNeverPublished => {
                WorkItemRelationFreshnessClass::LocalDraft
            }
        },
    }
}

fn freshness_label(freshness: WorkItemFreshnessClass, observed_at: &str) -> String {
    match freshness {
        WorkItemFreshnessClass::LiveAuthoritativeFresh => {
            format!("synced at {observed_at}")
        }
        WorkItemFreshnessClass::WarmWithinGrace => format!("cached at {observed_at}"),
        WorkItemFreshnessClass::DegradedBeyondGraceLocalContinues => {
            format!("stale provider snapshot from {observed_at}")
        }
        WorkItemFreshnessClass::UnverifiableProviderUnreachable => {
            format!("provider unreachable; last observed {observed_at}")
        }
        WorkItemFreshnessClass::ImportedSnapshotNoRefreshPath => {
            format!("imported snapshot captured at {observed_at}")
        }
        WorkItemFreshnessClass::LocalDraftNeverPublished => {
            "local draft not yet provider committed".to_string()
        }
    }
}
