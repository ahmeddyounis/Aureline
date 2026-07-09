//! Frozen M5 work-item-row, provider-chip-group, relation-strip, sync-pending-pill,
//! work-item-detail-header, status-transition-sheet, related-evidence-card, and
//! offline-handoff-packet-card component matrix.
//!
//! This module locks Aureline's reusable provider-backed work-item components into one
//! export-safe packet. Every issue-, task-, and incident-facing subcomponent M5 claims
//! that still drifts too easily by inbox, detail, review, incident, support, or CLI
//! surface — the work-item row, the provider-chip group, the relation strip, the
//! sync-pending pill, the work-item detail header, the status-transition sheet, the
//! related-evidence card, and the offline-handoff-packet card — is named once here and
//! constrained by the same canonical work-item identity, provider authority,
//! local-versus-provider state, linked engineering context, side-effect preview, and
//! publish-later continuity regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families, the work-item kinds and provider-authority classes the row and
//! detail header bind, the local-versus-provider states the row and sync-pending pill
//! bind, the relation kinds the relation strip binds, the evidence kinds the
//! related-evidence card binds, the transition effects the status-transition sheet binds,
//! the handoff destinations and metadata-safe export boundaries the offline-handoff card
//! binds, the deployment lines every component must survive, the non-visual accessibility
//! routes, and the mandatory labels every component must be able to show. It does not
//! re-architect the connected-provider registry, work-item detail, link-state,
//! status-transition, evidence-link, or offline-handoff contracts that already own those
//! records — it is the shared work-item component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 issue, task,
//! incident, review, support, or CLI work-item surface may publish an identity, authority,
//! sync-state, linked-context, side-effect, or handoff claim. Inbox, detail, relation,
//! sync, transition, evidence, offline-handoff, support, and CLI consumers all read this
//! packet so one work-item row names its identity, authority, and local state, one
//! provider-chip group names who owns the object, one relation strip names the linked
//! branch/review/test context, one sync-pending pill names what is only local and not yet
//! published, one status-transition sheet previews the side effects before a write, one
//! related-evidence card names its provenance, and one offline-handoff card names where a
//! deferred change will land and what export will reveal. No M5 lane invents a second
//! work-item grammar or an alternate label for provider authority, a pending publish, a
//! linked relation, a transition side effect, or a metadata-safe export boundary.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5WorkItemComponentVocabularySet`] rather than minted per surface. Raw comment
//! bodies, pasted paths, credentials, and private endpoints stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_work_item_component_matrix,
    seeded_m5_work_item_component_matrix_offline_handoff_packet_card_preview_narrowed,
    seeded_m5_work_item_component_matrix_status_transition_sheet_beta_narrowed,
    M5_WORK_ITEM_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5WorkItemComponentMatrixPacket`].
pub const M5_WORK_ITEM_COMPONENT_MATRIX_RECORD_KIND: &str = "freeze_m5_work_item_component_matrix";

/// Schema version for M5 work-item component-matrix records.
pub const M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined work-item component boundary schema.
pub const M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF: &str =
    "schemas/ui/m5-work-item-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_WORK_ITEM_COMPONENT_DOC_REF: &str =
    "docs/team-workflows/m5_work_item_component_matrix.md";

/// Repo-relative path of the canonical work-item-row component contract.
pub const M5_WORK_ITEM_ROW_SCHEMA_REF: &str = "schemas/ui/m5-work-item-row.schema.json";

/// Repo-relative path of the canonical provider-chip-group component contract.
pub const M5_PROVIDER_CHIP_GROUP_SCHEMA_REF: &str = "schemas/ui/m5-provider-chip-group.schema.json";

/// Repo-relative path of the canonical relation-strip component contract.
pub const M5_RELATION_STRIP_SCHEMA_REF: &str = "schemas/ui/m5-relation-strip.schema.json";

/// Repo-relative path of the canonical sync-pending-pill component contract.
pub const M5_SYNC_PENDING_PILL_SCHEMA_REF: &str = "schemas/ui/m5-sync-pending-pill.schema.json";

/// Repo-relative path of the canonical work-item-detail-header component contract.
pub const M5_WORK_ITEM_DETAIL_HEADER_SCHEMA_REF: &str =
    "schemas/ui/m5-work-item-detail-header.schema.json";

/// Repo-relative path of the canonical status-transition-sheet component contract.
pub const M5_STATUS_TRANSITION_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-status-transition-sheet.schema.json";

/// Repo-relative path of the canonical related-evidence-card component contract.
pub const M5_RELATED_EVIDENCE_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-related-evidence-card.schema.json";

/// Repo-relative path of the canonical offline-handoff-packet-card component contract.
pub const M5_OFFLINE_HANDOFF_PACKET_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-offline-handoff-packet-card.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_WORK_ITEM_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-work-item-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WORK_ITEM_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-work-item-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_WORK_ITEM_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-work-item-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_WORK_ITEM_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-work-item-component-matrix.md";

/// One of the eight governed work-item component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemComponentFamily {
    /// A work-item row carrying its canonical identity, provider authority, and local
    /// state.
    WorkItemRow,
    /// A provider-chip group carrying provider authority / who owns the object.
    ProviderChipGroup,
    /// A relation strip carrying linked engineering context.
    RelationStrip,
    /// A sync-pending pill carrying local-versus-provider state.
    SyncPendingPill,
    /// A work-item detail header carrying canonical identity and provider authority.
    WorkItemDetailHeader,
    /// A status-transition sheet carrying a side-effect preview before write.
    StatusTransitionSheet,
    /// A related-evidence card carrying evidence provenance.
    RelatedEvidenceCard,
    /// An offline-handoff-packet card carrying handoff destination and export boundary.
    OfflineHandoffPacketCard,
}

impl M5WorkItemComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::WorkItemRow,
        Self::ProviderChipGroup,
        Self::RelationStrip,
        Self::SyncPendingPill,
        Self::WorkItemDetailHeader,
        Self::StatusTransitionSheet,
        Self::RelatedEvidenceCard,
        Self::OfflineHandoffPacketCard,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkItemRow => "work_item_row",
            Self::ProviderChipGroup => "provider_chip_group",
            Self::RelationStrip => "relation_strip",
            Self::SyncPendingPill => "sync_pending_pill",
            Self::WorkItemDetailHeader => "work_item_detail_header",
            Self::StatusTransitionSheet => "status_transition_sheet",
            Self::RelatedEvidenceCard => "related_evidence_card",
            Self::OfflineHandoffPacketCard => "offline_handoff_packet_card",
        }
    }

    /// `true` when this family is a work-item row (declares kinds, authorities, states).
    pub const fn is_work_item_row(self) -> bool {
        matches!(self, Self::WorkItemRow)
    }

    /// `true` when this family is a provider-chip group (declares provider authorities).
    pub const fn is_provider_chip_group(self) -> bool {
        matches!(self, Self::ProviderChipGroup)
    }

    /// `true` when this family is a relation strip (declares relation kinds).
    pub const fn is_relation_strip(self) -> bool {
        matches!(self, Self::RelationStrip)
    }

    /// `true` when this family is a sync-pending pill (declares local states).
    pub const fn is_sync_pending_pill(self) -> bool {
        matches!(self, Self::SyncPendingPill)
    }

    /// `true` when this family is a work-item detail header (declares kinds, authorities).
    pub const fn is_work_item_detail_header(self) -> bool {
        matches!(self, Self::WorkItemDetailHeader)
    }

    /// `true` when this family is a status-transition sheet (declares transition effects).
    pub const fn is_status_transition_sheet(self) -> bool {
        matches!(self, Self::StatusTransitionSheet)
    }

    /// `true` when this family is a related-evidence card (declares evidence kinds).
    pub const fn is_related_evidence_card(self) -> bool {
        matches!(self, Self::RelatedEvidenceCard)
    }

    /// `true` when this family is an offline-handoff-packet card (declares destinations
    /// and export boundaries).
    pub const fn is_offline_handoff_packet_card(self) -> bool {
        matches!(self, Self::OfflineHandoffPacketCard)
    }

    /// `true` when this family carries canonical work-item kind (row or detail header).
    pub const fn carries_work_item_kind(self) -> bool {
        self.is_work_item_row() || self.is_work_item_detail_header()
    }

    /// `true` when this family carries provider authority (row, chip group, or detail
    /// header).
    pub const fn carries_provider_authority(self) -> bool {
        self.is_work_item_row()
            || self.is_provider_chip_group()
            || self.is_work_item_detail_header()
    }

    /// `true` when this family carries local-versus-provider state (row or sync pill).
    pub const fn carries_local_state(self) -> bool {
        self.is_work_item_row() || self.is_sync_pending_pill()
    }
}

/// Controlled work-item kind — the canonical identity class of a work item, so a row or
/// header never leaves the kind implicit or invents a parallel kind taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemKind {
    /// A tracked issue.
    Issue,
    /// A task.
    Task,
    /// An incident.
    Incident,
    /// A change request.
    ChangeRequest,
    /// An epic / parent grouping.
    Epic,
    /// An unknown / not-yet-classified kind.
    UnknownKind,
}

impl M5WorkItemKind {
    /// Every work-item kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Issue,
        Self::Task,
        Self::Incident,
        Self::ChangeRequest,
        Self::Epic,
        Self::UnknownKind,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Issue => "issue",
            Self::Task => "task",
            Self::Incident => "incident",
            Self::ChangeRequest => "change_request",
            Self::Epic => "epic",
            Self::UnknownKind => "unknown_kind",
        }
    }
}

/// Controlled provider-authority class — who owns a work item and whether Aureline may
/// write to it, so a chip group, row, or header never leaves authority implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemProviderAuthority {
    /// The provider owns the object; writes flow to the provider.
    ProviderOwned,
    /// A local draft not yet owned by any provider.
    LocalDraft,
    /// A read-only mirror of a provider object.
    MirroredReadOnly,
    /// An imported snapshot detached from live provider truth.
    ImportedSnapshot,
    /// An unlinked local item with no provider binding.
    UnlinkedLocal,
    /// Authority is pinned by policy.
    PolicyPinned,
}

impl M5WorkItemProviderAuthority {
    /// Every provider-authority class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProviderOwned,
        Self::LocalDraft,
        Self::MirroredReadOnly,
        Self::ImportedSnapshot,
        Self::UnlinkedLocal,
        Self::PolicyPinned,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOwned => "provider_owned",
            Self::LocalDraft => "local_draft",
            Self::MirroredReadOnly => "mirrored_read_only",
            Self::ImportedSnapshot => "imported_snapshot",
            Self::UnlinkedLocal => "unlinked_local",
            Self::PolicyPinned => "policy_pinned",
        }
    }
}

/// Controlled local-versus-provider state — whether a work item is reconciled with the
/// provider or only local, so a pending publish is never silently dropped or shown as
/// reconciled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemLocalState {
    /// Synced and reconciled with the provider.
    SyncedWithProvider,
    /// A local-only draft not yet published.
    LocalOnlyDraft,
    /// Queued for publish when reachable.
    QueuedForPublish,
    /// Publish deferred by the user.
    PublishDeferred,
    /// A prior publish attempt failed.
    PublishFailed,
    /// Held because of a conflict.
    ConflictHeld,
}

impl M5WorkItemLocalState {
    /// Every local state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SyncedWithProvider,
        Self::LocalOnlyDraft,
        Self::QueuedForPublish,
        Self::PublishDeferred,
        Self::PublishFailed,
        Self::ConflictHeld,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SyncedWithProvider => "synced_with_provider",
            Self::LocalOnlyDraft => "local_only_draft",
            Self::QueuedForPublish => "queued_for_publish",
            Self::PublishDeferred => "publish_deferred",
            Self::PublishFailed => "publish_failed",
            Self::ConflictHeld => "conflict_held",
        }
    }
}

/// Controlled relation kind — the linked engineering context a relation strip names, so a
/// branch / review / test link is never left implicit or given an alternate label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemRelationKind {
    /// A linked branch.
    LinkedBranch,
    /// A linked pull request.
    LinkedPullRequest,
    /// A linked review.
    LinkedReview,
    /// A linked test run.
    LinkedTestRun,
    /// A linked incident.
    LinkedIncident,
    /// An unmapped / dangling relation.
    UnmappedRelation,
}

impl M5WorkItemRelationKind {
    /// Every relation kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LinkedBranch,
        Self::LinkedPullRequest,
        Self::LinkedReview,
        Self::LinkedTestRun,
        Self::LinkedIncident,
        Self::UnmappedRelation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LinkedBranch => "linked_branch",
            Self::LinkedPullRequest => "linked_pull_request",
            Self::LinkedReview => "linked_review",
            Self::LinkedTestRun => "linked_test_run",
            Self::LinkedIncident => "linked_incident",
            Self::UnmappedRelation => "unmapped_relation",
        }
    }
}

/// Controlled evidence kind — the provenance a related-evidence card names, so linked
/// evidence never appears without disclosing what it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemEvidenceKind {
    /// A test result.
    TestResult,
    /// A CI check.
    CiCheck,
    /// A review thread.
    ReviewThread,
    /// A linked change / diff.
    LinkedChange,
    /// An attached artifact.
    AttachedArtifact,
    /// An external reference.
    ExternalReference,
}

impl M5WorkItemEvidenceKind {
    /// Every evidence kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TestResult,
        Self::CiCheck,
        Self::ReviewThread,
        Self::LinkedChange,
        Self::AttachedArtifact,
        Self::ExternalReference,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestResult => "test_result",
            Self::CiCheck => "ci_check",
            Self::ReviewThread => "review_thread",
            Self::LinkedChange => "linked_change",
            Self::AttachedArtifact => "attached_artifact",
            Self::ExternalReference => "external_reference",
        }
    }
}

/// Controlled transition effect — the side effect a status-transition sheet previews
/// before a write, so a user never has to infer whether a transition is only local or
/// publishes to the provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemTransitionEffect {
    /// A local-only transition; nothing publishes.
    LocalOnlyTransition,
    /// A publish-now transition; the provider is written immediately.
    PublishNowTransition,
    /// Opens the item in the provider instead of writing locally.
    OpenInProvider,
    /// A comment side effect will be posted.
    CommentSideEffect,
    /// A status side effect will be written.
    StatusSideEffect,
    /// The transition is blocked pending resolution.
    BlockedTransition,
}

impl M5WorkItemTransitionEffect {
    /// Every transition effect, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalOnlyTransition,
        Self::PublishNowTransition,
        Self::OpenInProvider,
        Self::CommentSideEffect,
        Self::StatusSideEffect,
        Self::BlockedTransition,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyTransition => "local_only_transition",
            Self::PublishNowTransition => "publish_now_transition",
            Self::OpenInProvider => "open_in_provider",
            Self::CommentSideEffect => "comment_side_effect",
            Self::StatusSideEffect => "status_side_effect",
            Self::BlockedTransition => "blocked_transition",
        }
    }
}

/// Controlled handoff destination — where a deferred change captured by an
/// offline-handoff-packet card will land, so a handoff destination is never assumed
/// silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemHandoffDestination {
    /// Held in the local publish-later queue.
    LocalQueue,
    /// Published to the provider on handoff.
    ProviderPublish,
    /// Written to an exported packet.
    ExportedPacket,
    /// Attached to a support bundle.
    SupportBundle,
    /// Handed off to another device.
    AnotherDevice,
    /// Discarded after review.
    DiscardAfterReview,
}

impl M5WorkItemHandoffDestination {
    /// Every handoff destination, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalQueue,
        Self::ProviderPublish,
        Self::ExportedPacket,
        Self::SupportBundle,
        Self::AnotherDevice,
        Self::DiscardAfterReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalQueue => "local_queue",
            Self::ProviderPublish => "provider_publish",
            Self::ExportedPacket => "exported_packet",
            Self::SupportBundle => "support_bundle",
            Self::AnotherDevice => "another_device",
            Self::DiscardAfterReview => "discard_after_review",
        }
    }
}

/// Controlled export boundary — the metadata-safe boundary an offline-handoff-packet card
/// keeps, so no surface invents an alternate label for a metadata-safe export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemExportBoundary {
    /// Metadata-safe export.
    MetadataSafe,
    /// Bodies excluded from export.
    BodyExcluded,
    /// Identifiers masked in export.
    IdentifiersMasked,
    /// Credentials scrubbed from export.
    CredentialsScrubbed,
    /// Local-only, never exported.
    LocalOnly,
    /// Full disclosure is blocked.
    FullDisclosureBlocked,
}

impl M5WorkItemExportBoundary {
    /// Every export boundary class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MetadataSafe,
        Self::BodyExcluded,
        Self::IdentifiersMasked,
        Self::CredentialsScrubbed,
        Self::LocalOnly,
        Self::FullDisclosureBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafe => "metadata_safe",
            Self::BodyExcluded => "body_excluded",
            Self::IdentifiersMasked => "identifiers_masked",
            Self::CredentialsScrubbed => "credentials_scrubbed",
            Self::LocalOnly => "local_only",
            Self::FullDisclosureBlocked => "full_disclosure_blocked",
        }
    }
}

/// Claimed M5 work-item surface family that renders / consumes a work-item component. No
/// component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemSurfaceFamily {
    /// The issue-inbox surface.
    IssueInbox,
    /// The work-item-detail surface.
    WorkItemDetail,
    /// The review-workspace surface.
    ReviewWorkspace,
    /// The incident-workspace surface.
    IncidentWorkspace,
    /// The support-workflow surface.
    SupportWorkflow,
    /// The CLI work-item surface.
    CliWorkItem,
}

impl M5WorkItemSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IssueInbox,
        Self::WorkItemDetail,
        Self::ReviewWorkspace,
        Self::IncidentWorkspace,
        Self::SupportWorkflow,
        Self::CliWorkItem,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IssueInbox => "issue_inbox",
            Self::WorkItemDetail => "work_item_detail",
            Self::ReviewWorkspace => "review_workspace",
            Self::IncidentWorkspace => "incident_workspace",
            Self::SupportWorkflow => "support_workflow",
            Self::CliWorkItem => "cli_work_item",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// identity, authority, sync, context, side-effect, or handoff truth never silently
/// narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5WorkItemDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Work-item subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemConsumerSurface {
    /// The inbox UI.
    InboxUi,
    /// The detail UI.
    DetailUi,
    /// The relation-panel UI.
    RelationPanelUi,
    /// The sync-status UI.
    SyncStatusUi,
    /// The transition-sheet UI.
    TransitionSheetUi,
    /// The evidence-panel UI.
    EvidencePanelUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5WorkItemConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::InboxUi,
        Self::DetailUi,
        Self::RelationPanelUi,
        Self::SyncStatusUi,
        Self::TransitionSheetUi,
        Self::EvidencePanelUi,
        Self::SupportExport,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InboxUi => "inbox_ui",
            Self::DetailUi => "detail_ui",
            Self::RelationPanelUi => "relation_panel_ui",
            Self::SyncStatusUi => "sync_status_ui",
            Self::TransitionSheetUi => "transition_sheet_ui",
            Self::EvidencePanelUi => "evidence_panel_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no work-item truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5WorkItemAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed work-item component must be able to show. The first three are
/// hard requirements on every component; the remaining five close the acceptance-criteria
/// ambiguity about authority, local state, linked context, side effects, and publish-later
/// continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemRequiredLabel {
    /// The component's canonical work-item identity / what object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The provider authority behind the component.
    ProviderAuthority,
    /// The local-versus-provider state behind the component.
    LocalVersusProviderState,
    /// The linked engineering context behind the component.
    LinkedEngineeringContext,
    /// The side-effect preview behind the component.
    SideEffectPreview,
    /// The publish-later continuity behind the component.
    PublishLaterContinuity,
}

impl M5WorkItemRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ProviderAuthority,
        Self::LocalVersusProviderState,
        Self::LinkedEngineeringContext,
        Self::SideEffectPreview,
        Self::PublishLaterContinuity,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ProviderAuthority => "provider_authority",
            Self::LocalVersusProviderState => "local_versus_provider_state",
            Self::LinkedEngineeringContext => "linked_engineering_context",
            Self::SideEffectPreview => "side_effect_preview",
            Self::PublishLaterContinuity => "publish_later_continuity",
        }
    }
}

/// Qualification class for an M5 work-item component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5WorkItemQualificationClass {
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

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a work-item component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkItemDowngradeTrigger {
    /// A component left its canonical work-item identity unstated.
    IdentityUnstated,
    /// A component left its provider authority unstated.
    ProviderAuthorityUnstated,
    /// A component hid its local-versus-provider state.
    LocalVersusProviderStateHidden,
    /// A component left its linked engineering context unstated.
    LinkedContextUnstated,
    /// A component hid its side-effect preview before write.
    SideEffectPreviewHidden,
    /// A component hid its publish-later continuity.
    PublishLaterContinuityHidden,
    /// A sync-pending state was hidden.
    SyncPendingStateHidden,
    /// An evidence card left its provenance unstated.
    EvidenceProvenanceUnstated,
    /// An offline-handoff card left its destination unstated.
    HandoffDestinationUnstated,
    /// An offline-handoff card hid its export boundary.
    ExportBoundaryHidden,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// Generic ticket / task wording concealed provider ownership or queued state.
    GenericTicketWordingUsed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5WorkItemDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::IdentityUnstated,
        Self::ProviderAuthorityUnstated,
        Self::LocalVersusProviderStateHidden,
        Self::LinkedContextUnstated,
        Self::SideEffectPreviewHidden,
        Self::PublishLaterContinuityHidden,
        Self::SyncPendingStateHidden,
        Self::EvidenceProvenanceUnstated,
        Self::HandoffDestinationUnstated,
        Self::ExportBoundaryHidden,
        Self::AlternateStateLabelInvented,
        Self::GenericTicketWordingUsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityUnstated => "identity_unstated",
            Self::ProviderAuthorityUnstated => "provider_authority_unstated",
            Self::LocalVersusProviderStateHidden => "local_versus_provider_state_hidden",
            Self::LinkedContextUnstated => "linked_context_unstated",
            Self::SideEffectPreviewHidden => "side_effect_preview_hidden",
            Self::PublishLaterContinuityHidden => "publish_later_continuity_hidden",
            Self::SyncPendingStateHidden => "sync_pending_state_hidden",
            Self::EvidenceProvenanceUnstated => "evidence_provenance_unstated",
            Self::HandoffDestinationUnstated => "handoff_destination_unstated",
            Self::ExportBoundaryHidden => "export_boundary_hidden",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::GenericTicketWordingUsed => "generic_ticket_wording_used",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed work-item component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentRow {
    /// Governed component family.
    pub component_family: M5WorkItemComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5WorkItemQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 work-item surface families that render / consume this component.
    pub surface_families: Vec<M5WorkItemSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5WorkItemDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5WorkItemRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5WorkItemRequiredLabel>,
    /// Work-item kinds this component names (work-item-row and detail-header).
    pub work_item_kinds: Vec<M5WorkItemKind>,
    /// Provider-authority classes this component names (row, chip group, detail header).
    pub provider_authorities: Vec<M5WorkItemProviderAuthority>,
    /// Local-versus-provider states this component names (row and sync-pending pill).
    pub local_states: Vec<M5WorkItemLocalState>,
    /// Relation kinds this component names (relation-strip only).
    pub relation_kinds: Vec<M5WorkItemRelationKind>,
    /// Evidence kinds this component names (related-evidence-card only).
    pub evidence_kinds: Vec<M5WorkItemEvidenceKind>,
    /// Transition effects this component names (status-transition-sheet only).
    pub transition_effects: Vec<M5WorkItemTransitionEffect>,
    /// Handoff destinations this component names (offline-handoff-packet-card only).
    pub handoff_destinations: Vec<M5WorkItemHandoffDestination>,
    /// Export boundaries this component names (offline-handoff-packet-card only).
    pub export_boundaries: Vec<M5WorkItemExportBoundary>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5WorkItemAccessibilityRoute>,
    /// Work-item subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5WorkItemConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5WorkItemDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its canonical identity or provider
    /// authority. MUST be `false`.
    pub masks_identity_or_authority: bool,
    /// Hard invariant: this component never hides its local-versus-provider or
    /// publish-later state. MUST be `false`.
    pub hides_local_or_publish_later_state: bool,
    /// Hard invariant: this component never invents an alternate label for a governed
    /// state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: this component never lets generic ticket / task wording conceal
    /// provider ownership, queued state, or linked context. MUST be `false`.
    pub uses_generic_ticket_wording: bool,
}

impl M5WorkItemComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5WorkItemRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5WorkItemRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_identity_or_authority
            && !self.hides_local_or_publish_later_state
            && !self.invents_alternate_state_label
            && !self.uses_generic_ticket_wording
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Work-item-kind tokens.
    pub work_item_kinds: Vec<String>,
    /// Provider-authority tokens.
    pub provider_authorities: Vec<String>,
    /// Local-state tokens.
    pub local_states: Vec<String>,
    /// Relation-kind tokens.
    pub relation_kinds: Vec<String>,
    /// Evidence-kind tokens.
    pub evidence_kinds: Vec<String>,
    /// Transition-effect tokens.
    pub transition_effects: Vec<String>,
    /// Handoff-destination tokens.
    pub handoff_destinations: Vec<String>,
    /// Export-boundary tokens.
    pub export_boundaries: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5WorkItemComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5WorkItemComponentFamily::ALL, |v| v.as_str()),
            work_item_kinds: tokens(&M5WorkItemKind::ALL, |v| v.as_str()),
            provider_authorities: tokens(&M5WorkItemProviderAuthority::ALL, |v| v.as_str()),
            local_states: tokens(&M5WorkItemLocalState::ALL, |v| v.as_str()),
            relation_kinds: tokens(&M5WorkItemRelationKind::ALL, |v| v.as_str()),
            evidence_kinds: tokens(&M5WorkItemEvidenceKind::ALL, |v| v.as_str()),
            transition_effects: tokens(&M5WorkItemTransitionEffect::ALL, |v| v.as_str()),
            handoff_destinations: tokens(&M5WorkItemHandoffDestination::ALL, |v| v.as_str()),
            export_boundaries: tokens(&M5WorkItemExportBoundary::ALL, |v| v.as_str()),
            surface_families: tokens(&M5WorkItemSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5WorkItemDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5WorkItemConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5WorkItemAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5WorkItemRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5WorkItemComponentGovernanceReview {
    /// The work-item row shows its identity, authority, and local state.
    pub row_shows_identity_authority_and_state: bool,
    /// The provider-chip group shows provider authority.
    pub chip_group_shows_authority: bool,
    /// The relation strip shows its linked engineering context.
    pub relation_strip_shows_linked_context: bool,
    /// The sync-pending pill shows its local-versus-provider state.
    pub sync_pill_shows_local_versus_provider_state: bool,
    /// The detail header shows its identity and authority.
    pub detail_header_shows_identity_and_authority: bool,
    /// The status-transition sheet shows its side-effect preview before write.
    pub transition_sheet_shows_side_effect_preview: bool,
    /// The related-evidence card shows its provenance.
    pub evidence_card_shows_provenance: bool,
    /// The offline-handoff card shows its destination and export boundary.
    pub handoff_card_shows_destination_and_export_boundary: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// Generic ticket / task wording never conceals provider ownership or queued state.
    pub no_generic_ticket_wording_conceals_authority: bool,
    /// Publish-later continuity is always explicit before write.
    pub publish_later_continuity_always_explicit: bool,
    /// The side-effect preview is always shown before a write.
    pub side_effect_preview_always_before_write: bool,
    /// The export boundary is always explicit.
    pub export_boundary_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel work-item vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentConsumerProjection {
    /// Inbox and detail surfaces consume the shared identity and kind vocabulary.
    pub inbox_and_detail_surfaces_consume_identity_vocabulary: bool,
    /// Provider-chip surfaces consume the provider-authority vocabulary.
    pub chip_surfaces_consume_authority_vocabulary: bool,
    /// Sync surfaces consume the local-versus-provider state vocabulary.
    pub sync_surfaces_consume_local_state_vocabulary: bool,
    /// Relation and evidence surfaces consume the linked-context vocabulary.
    pub relation_and_evidence_surfaces_consume_context_vocabulary: bool,
    /// Transition and handoff surfaces consume the publish-later vocabulary.
    pub transition_and_handoff_surfaces_consume_publish_later_vocabulary: bool,
    /// Support / export reads a single canonical work-item source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the work-item component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting work-item matrix audit for the lane.
    pub work_item_matrix_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WorkItemComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WorkItemComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5WorkItemComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkItemComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkItemComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkItemComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkItemComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkItemComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 work-item component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkItemComponentMatrixPacket {
    /// Record kind; must equal [`M5_WORK_ITEM_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5WorkItemComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkItemComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkItemComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkItemComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkItemComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkItemComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WorkItemComponentMatrixPacket {
    /// Builds an M5 work-item component matrix packet from stable-lane input.
    pub fn new(input: M5WorkItemComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_WORK_ITEM_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 work-item component matrix invariants.
    pub fn validate(&self) -> Vec<M5WorkItemComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WORK_ITEM_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5WorkItemComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5WorkItemComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WorkItemComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 work-item component matrix packet serializes"),
        ) {
            violations.push(M5WorkItemComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 work-item component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Work-Item Component Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Provider authorities: {}\n",
            self.vocabulary_set.provider_authorities.join(", ")
        ));
        out.push_str(&format!(
            "- Local states: {}\n",
            self.vocabulary_set.local_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
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

/// Errors emitted when reading the checked-in M5 work-item matrix export.
#[derive(Debug)]
pub enum M5WorkItemComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WorkItemComponentMatrixViolation>),
}

impl fmt::Display for M5WorkItemComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 work-item component matrix export parse failed: {error}"
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
                    "m5 work-item component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WorkItemComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5WorkItemComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WorkItemComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A work-item-row or detail-header component declares no work-item kinds.
    WorkItemKindMissing,
    /// A row / chip-group / detail-header component declares no provider authorities.
    ProviderAuthorityMissing,
    /// A row or sync-pending-pill component declares no local states.
    LocalStateMissing,
    /// A relation-strip component declares no relation kinds.
    RelationKindMissing,
    /// A related-evidence-card component declares no evidence kinds.
    EvidenceKindMissing,
    /// A status-transition-sheet component declares no transition effects.
    TransitionEffectMissing,
    /// An offline-handoff-packet-card component declares no handoff destinations.
    HandoffDestinationMissing,
    /// An offline-handoff-packet-card component declares no export boundaries.
    ExportBoundaryMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked identity/authority, hidden
    /// local/publish-later state, invented alternate label, or generic ticket wording).
    ComponentInvariantViolated,
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

impl M5WorkItemComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::WorkItemKindMissing => "work_item_kind_missing",
            Self::ProviderAuthorityMissing => "provider_authority_missing",
            Self::LocalStateMissing => "local_state_missing",
            Self::RelationKindMissing => "relation_kind_missing",
            Self::EvidenceKindMissing => "evidence_kind_missing",
            Self::TransitionEffectMissing => "transition_effect_missing",
            Self::HandoffDestinationMissing => "handoff_destination_missing",
            Self::ExportBoundaryMissing => "export_boundary_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 work-item matrix export.
pub fn current_stable_m5_work_item_component_matrix_export(
) -> Result<M5WorkItemComponentMatrixPacket, M5WorkItemComponentMatrixArtifactError> {
    let packet: M5WorkItemComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-work-item-component-proof/support_export.json"
    )))
    .map_err(M5WorkItemComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WorkItemComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5WorkItemComponentMatrixPacket,
    violations: &mut Vec<M5WorkItemComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WORK_ITEM_COMPONENT_MATRIX_SCHEMA_REF,
        M5_WORK_ITEM_COMPONENT_DOC_REF,
        M5_WORK_ITEM_ROW_SCHEMA_REF,
        M5_PROVIDER_CHIP_GROUP_SCHEMA_REF,
        M5_RELATION_STRIP_SCHEMA_REF,
        M5_SYNC_PENDING_PILL_SCHEMA_REF,
        M5_WORK_ITEM_DETAIL_HEADER_SCHEMA_REF,
        M5_STATUS_TRANSITION_SHEET_SCHEMA_REF,
        M5_RELATED_EVIDENCE_CARD_SCHEMA_REF,
        M5_OFFLINE_HANDOFF_PACKET_CARD_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5WorkItemComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5WorkItemComponentMatrixPacket,
    violations: &mut Vec<M5WorkItemComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5WorkItemComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5WorkItemComponentMatrixPacket,
    violations: &mut Vec<M5WorkItemComponentMatrixViolation>,
) {
    let present: BTreeSet<M5WorkItemComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5WorkItemComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5WorkItemComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5WorkItemComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5WorkItemComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.carries_work_item_kind() && row.work_item_kinds.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::WorkItemKindMissing);
        }
        if family.carries_provider_authority() && row.provider_authorities.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::ProviderAuthorityMissing);
        }
        if family.carries_local_state() && row.local_states.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::LocalStateMissing);
        }
        if family.is_relation_strip() && row.relation_kinds.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::RelationKindMissing);
        }
        if family.is_related_evidence_card() && row.evidence_kinds.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::EvidenceKindMissing);
        }
        if family.is_status_transition_sheet() && row.transition_effects.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::TransitionEffectMissing);
        }
        if family.is_offline_handoff_packet_card() && row.handoff_destinations.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::HandoffDestinationMissing);
        }
        if family.is_offline_handoff_packet_card() && row.export_boundaries.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::ExportBoundaryMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5WorkItemComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5WorkItemComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5WorkItemComponentMatrixPacket,
    violations: &mut Vec<M5WorkItemComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.row_shows_identity_authority_and_state,
        review.chip_group_shows_authority,
        review.relation_strip_shows_linked_context,
        review.sync_pill_shows_local_versus_provider_state,
        review.detail_header_shows_identity_and_authority,
        review.transition_sheet_shows_side_effect_preview,
        review.evidence_card_shows_provenance,
        review.handoff_card_shows_destination_and_export_boundary,
        review.no_surface_invents_alternate_state_label,
        review.no_generic_ticket_wording_conceals_authority,
        review.publish_later_continuity_always_explicit,
        review.side_effect_preview_always_before_write,
        review.export_boundary_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5WorkItemComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WorkItemComponentMatrixPacket,
    violations: &mut Vec<M5WorkItemComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.inbox_and_detail_surfaces_consume_identity_vocabulary,
        projection.chip_surfaces_consume_authority_vocabulary,
        projection.sync_surfaces_consume_local_state_vocabulary,
        projection.relation_and_evidence_surfaces_consume_context_vocabulary,
        projection.transition_and_handoff_surfaces_consume_publish_later_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5WorkItemComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WorkItemComponentMatrixPacket,
    violations: &mut Vec<M5WorkItemComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5WorkItemComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WorkItemComponentMatrixPacket,
    violations: &mut Vec<M5WorkItemComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.work_item_matrix_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5WorkItemComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
