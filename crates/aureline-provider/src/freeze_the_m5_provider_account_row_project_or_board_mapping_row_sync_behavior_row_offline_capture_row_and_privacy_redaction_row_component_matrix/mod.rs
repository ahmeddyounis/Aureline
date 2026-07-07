//! Frozen M5 provider-account-row, project-or-board-mapping-row, sync-behavior-row,
//! offline-capture-row, and privacy-redaction-row component matrix.
//!
//! This module locks Aureline's reusable provider-boundary settings and status
//! components into one export-safe packet. Every provider-account- and offline-capture-
//! facing subcomponent M5 claims that still drifts too easily by issue, review, incident,
//! support, settings, or CLI surface — the provider-account row, the project/board
//! mapping row, the sync-behavior row, the offline-capture row, and the privacy/redaction
//! row — is named once here and constrained by the same provider identity class, account
//! connection state, tenant scope, mapping origin, default-destination target, sync mode,
//! effective write scope, queued-draft state, offline-capture state, and reviewed
//! redaction and export boundary regardless of the surface family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves:
//! the component families, the provider identity classes and account connection states
//! (`not_configured`, `signed_in`, `limited_scope`, `stale_session`,
//! `offline_cached_read`, `policy_blocked`) and tenant scopes the account row binds, the
//! mapping origins and target kinds the mapping row binds, the sync modes and write
//! scopes the sync row binds, the offline-capture states and shared queued-draft states
//! the offline and sync rows bind, the redaction classes and metadata-safe export
//! boundaries the privacy row binds, the deployment lines every component must survive,
//! the non-visual accessibility routes, and the mandatory labels every component must be
//! able to show. It does not re-architect the connected-provider registry, target
//! mapping, sync-health, publish-later queue, or export-redaction contracts that already
//! own those records — it is the shared provider-account / mapping / offline-capture
//! contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 issue, review,
//! incident, support, provider-settings, or CLI provider surface may publish a
//! connection, scope, mapping, sync, offline-capture, or redaction claim. Account,
//! mapping, sync, offline, privacy, and export consumers all read this packet so one
//! provider-account row names its connection state and tenant scope, one mapping row
//! names where a publish will land and how that mapping was derived, one sync-behavior
//! row names its sync mode and effective write scope, one offline-capture row names what
//! remains queued locally, and one privacy-redaction row names what support and export
//! will reveal. No M5 lane invents a second provider grammar or an alternate label for a
//! stale session, a mapping origin, a sync mode, an offline-capture state, or a
//! metadata-safe export boundary.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5ProviderAccountOfflineComponentVocabularySet`] rather than minted per surface.
//! Raw comment bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_provider_account_offline_capture_component_matrix,
    seeded_m5_provider_account_offline_capture_component_matrix_offline_capture_row_beta_narrowed,
    seeded_m5_provider_account_offline_capture_component_matrix_privacy_redaction_row_preview_narrowed,
    M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ProviderAccountOfflineComponentMatrixPacket`].
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix";

/// Schema version for M5 provider-account / offline-capture component-matrix records.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the provider-account / offline-capture component boundary schema.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_DOC_REF: &str =
    "docs/providers/m5_provider_account_offline_capture_component_matrix.md";

/// Repo-relative path of the connected-account contract the account row binds against.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_CONNECTED_ACCOUNT_REF: &str =
    "schemas/providers/connected_account_record.schema.json";

/// Repo-relative path of the target-mapping contract the mapping row binds against.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_TARGET_MAPPING_REF: &str =
    "schemas/providers/provider_target_mapping.schema.json";

/// Repo-relative path of the sync-health contract the sync-behavior row binds against.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SYNC_HEALTH_REF: &str =
    "schemas/providers/provider_sync_health_view.schema.json";

/// Repo-relative path of the offline-handoff contract the offline-capture row binds
/// against.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_OFFLINE_HANDOFF_REF: &str =
    "schemas/providers/offline_handoff_packet.schema.json";

/// Repo-relative path of the export-redaction contract the privacy-redaction row binds
/// against.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_EXPORT_REDACTION_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-provider-account-offline-capture-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-provider-account-offline-capture-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-provider-account-offline-capture-component-matrix.md";

/// One of the five governed provider-account / offline-capture component families this
/// matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccountOfflineComponentFamily {
    /// A provider-account row carrying its connection state, identity class, and tenant
    /// scope.
    ProviderAccountRow,
    /// A project/board mapping row carrying its default-destination target and mapping
    /// origin.
    ProjectOrBoardMappingRow,
    /// A sync-behavior row carrying its sync mode, effective write scope, and queued-
    /// draft state.
    SyncBehaviorRow,
    /// An offline-capture row carrying its offline-capture state and queued-draft state.
    OfflineCaptureRow,
    /// A privacy/redaction row carrying its redaction class and metadata-safe export
    /// boundary.
    PrivacyRedactionRow,
}

impl M5ProviderAccountOfflineComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ProviderAccountRow,
        Self::ProjectOrBoardMappingRow,
        Self::SyncBehaviorRow,
        Self::OfflineCaptureRow,
        Self::PrivacyRedactionRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderAccountRow => "provider_account_row",
            Self::ProjectOrBoardMappingRow => "project_or_board_mapping_row",
            Self::SyncBehaviorRow => "sync_behavior_row",
            Self::OfflineCaptureRow => "offline_capture_row",
            Self::PrivacyRedactionRow => "privacy_redaction_row",
        }
    }

    /// `true` when this family is a provider-account row and must therefore declare its
    /// provider identity classes, account connection states, and tenant scopes.
    pub const fn is_provider_account_row(self) -> bool {
        matches!(self, Self::ProviderAccountRow)
    }

    /// `true` when this family is a project/board mapping row and must therefore declare
    /// its mapping origins and target kinds.
    pub const fn is_project_or_board_mapping_row(self) -> bool {
        matches!(self, Self::ProjectOrBoardMappingRow)
    }

    /// `true` when this family is a sync-behavior row and must therefore declare its sync
    /// modes and write scopes.
    pub const fn is_sync_behavior_row(self) -> bool {
        matches!(self, Self::SyncBehaviorRow)
    }

    /// `true` when this family is an offline-capture row and must therefore declare its
    /// offline-capture states.
    pub const fn is_offline_capture_row(self) -> bool {
        matches!(self, Self::OfflineCaptureRow)
    }

    /// `true` when this family is a privacy/redaction row and must therefore declare its
    /// redaction classes and export boundaries.
    pub const fn is_privacy_redaction_row(self) -> bool {
        matches!(self, Self::PrivacyRedactionRow)
    }
}

/// Controlled provider identity class — how a provider-account row identifies the acting
/// account, so a row never leaves identity implicit or invents a parallel identity
/// taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderIdentityClass {
    /// A personal connected account.
    PersonalAccount,
    /// A member acting within an organization.
    OrganizationMember,
    /// A service / machine account.
    ServiceAccount,
    /// A delegated credential acting on behalf of another principal.
    DelegatedCredential,
    /// An installation grant scoped to selected resources.
    InstallationGrant,
    /// An unlinked identity not yet connected.
    UnlinkedIdentity,
}

impl M5ProviderIdentityClass {
    /// Every provider identity class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PersonalAccount,
        Self::OrganizationMember,
        Self::ServiceAccount,
        Self::DelegatedCredential,
        Self::InstallationGrant,
        Self::UnlinkedIdentity,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PersonalAccount => "personal_account",
            Self::OrganizationMember => "organization_member",
            Self::ServiceAccount => "service_account",
            Self::DelegatedCredential => "delegated_credential",
            Self::InstallationGrant => "installation_grant",
            Self::UnlinkedIdentity => "unlinked_identity",
        }
    }
}

/// Controlled account connection state — the one governed vocabulary every provider
/// surface binds so a user never has to infer whether Aureline can read or write right
/// now. These are the exact acceptance-criteria labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccountConnectionState {
    /// No provider account is configured.
    NotConfigured,
    /// A provider account is signed in with full scope.
    SignedIn,
    /// A provider account is signed in but scope is limited.
    LimitedScope,
    /// The session is stale and needs reauthentication.
    StaleSession,
    /// Only an offline cached read is available.
    OfflineCachedRead,
    /// The account is blocked by policy.
    PolicyBlocked,
}

impl M5AccountConnectionState {
    /// Every account connection state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotConfigured,
        Self::SignedIn,
        Self::LimitedScope,
        Self::StaleSession,
        Self::OfflineCachedRead,
        Self::PolicyBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotConfigured => "not_configured",
            Self::SignedIn => "signed_in",
            Self::LimitedScope => "limited_scope",
            Self::StaleSession => "stale_session",
            Self::OfflineCachedRead => "offline_cached_read",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// Controlled tenant scope — the boundary an account acts within, so a provider-account
/// row never leaves the tenant scope implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TenantScopeClass {
    /// A single tenant.
    SingleTenant,
    /// Multiple tenants.
    MultiTenant,
    /// Scoped to an organization.
    OrgScoped,
    /// Scoped to a project.
    ProjectScoped,
    /// A personal scope.
    PersonalScope,
    /// An unknown tenant scope.
    UnknownTenant,
}

impl M5TenantScopeClass {
    /// Every tenant scope class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SingleTenant,
        Self::MultiTenant,
        Self::OrgScoped,
        Self::ProjectScoped,
        Self::PersonalScope,
        Self::UnknownTenant,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleTenant => "single_tenant",
            Self::MultiTenant => "multi_tenant",
            Self::OrgScoped => "org_scoped",
            Self::ProjectScoped => "project_scoped",
            Self::PersonalScope => "personal_scope",
            Self::UnknownTenant => "unknown_tenant",
        }
    }
}

/// Controlled mapping origin — how a project/board mapping row's default destination was
/// derived, so a publish target is never assumed silently or given an alternate label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MappingOriginClass {
    /// The user chose the mapping explicitly.
    ExplicitUserChoice,
    /// The mapping was inherited from a default.
    InheritedDefault,
    /// The mapping was auto-matched by heuristics.
    AutoMatched,
    /// The mapping was imported from external config.
    ImportedConfig,
    /// The mapping is pinned by policy.
    PolicyPinned,
    /// The row has no mapping origin yet.
    UnmappedOrigin,
}

impl M5MappingOriginClass {
    /// Every mapping origin class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExplicitUserChoice,
        Self::InheritedDefault,
        Self::AutoMatched,
        Self::ImportedConfig,
        Self::PolicyPinned,
        Self::UnmappedOrigin,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExplicitUserChoice => "explicit_user_choice",
            Self::InheritedDefault => "inherited_default",
            Self::AutoMatched => "auto_matched",
            Self::ImportedConfig => "imported_config",
            Self::PolicyPinned => "policy_pinned",
            Self::UnmappedOrigin => "unmapped_origin",
        }
    }
}

/// Controlled mapping target kind — what a project/board mapping row points a publish at,
/// so the destination kind is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MappingTargetKind {
    /// An issue-tracker project.
    IssueTrackerProject,
    /// A kanban board.
    KanbanBoard,
    /// A repository.
    Repository,
    /// A milestone.
    Milestone,
    /// A label set.
    LabelSet,
    /// An unmapped target.
    UnmappedTarget,
}

impl M5MappingTargetKind {
    /// Every mapping target kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IssueTrackerProject,
        Self::KanbanBoard,
        Self::Repository,
        Self::Milestone,
        Self::LabelSet,
        Self::UnmappedTarget,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IssueTrackerProject => "issue_tracker_project",
            Self::KanbanBoard => "kanban_board",
            Self::Repository => "repository",
            Self::Milestone => "milestone",
            Self::LabelSet => "label_set",
            Self::UnmappedTarget => "unmapped_target",
        }
    }
}

/// Controlled sync mode — how a sync-behavior row keeps local and provider truth in step,
/// so no surface invents an alternate label for a degraded or one-way sync.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderSyncMode {
    /// Live two-way sync.
    LiveBidirectional,
    /// A read-only mirror.
    ReadOnlyMirror,
    /// Manual push only.
    ManualPush,
    /// Scheduled periodic sync.
    ScheduledSync,
    /// Sync is paused.
    PausedSync,
    /// Offline-only, no sync.
    OfflineOnly,
}

impl M5ProviderSyncMode {
    /// Every sync mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveBidirectional,
        Self::ReadOnlyMirror,
        Self::ManualPush,
        Self::ScheduledSync,
        Self::PausedSync,
        Self::OfflineOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveBidirectional => "live_bidirectional",
            Self::ReadOnlyMirror => "read_only_mirror",
            Self::ManualPush => "manual_push",
            Self::ScheduledSync => "scheduled_sync",
            Self::PausedSync => "paused_sync",
            Self::OfflineOnly => "offline_only",
        }
    }
}

/// Controlled effective write scope — what a sync-behavior row can actually write to the
/// provider right now, so a user never has to infer whether Aureline can write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderWriteScope {
    /// Full write access.
    FullWrite,
    /// Comment-only write access.
    CommentOnly,
    /// Status-only write access.
    StatusOnly,
    /// Read-only, no write.
    ReadOnly,
    /// No write access at all.
    NoWrite,
    /// The write scope is unknown.
    ScopeUnknown,
}

impl M5ProviderWriteScope {
    /// Every write scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullWrite,
        Self::CommentOnly,
        Self::StatusOnly,
        Self::ReadOnly,
        Self::NoWrite,
        Self::ScopeUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullWrite => "full_write",
            Self::CommentOnly => "comment_only",
            Self::StatusOnly => "status_only",
            Self::ReadOnly => "read_only",
            Self::NoWrite => "no_write",
            Self::ScopeUnknown => "scope_unknown",
        }
    }
}

/// Controlled offline-capture state — how an offline-capture row holds a locally captured
/// change, so no surface invents an alternate label for what remains queued locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflineCaptureState {
    /// Captured and held locally.
    CapturedLocal,
    /// Queued for publish when reachable.
    QueuedForPublish,
    /// Publish deferred by the user.
    PublishDeferred,
    /// Held because of a conflict.
    ConflictHeld,
    /// Pending discard.
    DiscardPending,
    /// Synced and cleared from the queue.
    SyncedCleared,
}

impl M5OfflineCaptureState {
    /// Every offline-capture state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CapturedLocal,
        Self::QueuedForPublish,
        Self::PublishDeferred,
        Self::ConflictHeld,
        Self::DiscardPending,
        Self::SyncedCleared,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedLocal => "captured_local",
            Self::QueuedForPublish => "queued_for_publish",
            Self::PublishDeferred => "publish_deferred",
            Self::ConflictHeld => "conflict_held",
            Self::DiscardPending => "discard_pending",
            Self::SyncedCleared => "synced_cleared",
        }
    }
}

/// Controlled queued-draft state — the state of a locally queued draft, declared by both
/// the sync-behavior row and the offline-capture row so a pending publish is never
/// silently dropped or shown as reconciled.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QueuedDraftState {
    /// No local draft is queued.
    NoLocalDraft,
    /// A draft is pending.
    DraftPending,
    /// A draft is queued for publish.
    QueuedPublish,
    /// Publish is blocked pending resolution.
    PublishBlocked,
    /// A prior publish attempt failed.
    PublishFailed,
    /// The draft was published and reconciled.
    PublishedReconciled,
}

impl M5QueuedDraftState {
    /// Every queued-draft state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoLocalDraft,
        Self::DraftPending,
        Self::QueuedPublish,
        Self::PublishBlocked,
        Self::PublishFailed,
        Self::PublishedReconciled,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLocalDraft => "no_local_draft",
            Self::DraftPending => "draft_pending",
            Self::QueuedPublish => "queued_publish",
            Self::PublishBlocked => "publish_blocked",
            Self::PublishFailed => "publish_failed",
            Self::PublishedReconciled => "published_reconciled",
        }
    }
}

/// Controlled redaction class — how much of a provider-linked object a privacy/redaction
/// row will reveal, so a user always sees what support and export will disclose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderRedactionClass {
    /// Full body is visible.
    FullBodyVisible,
    /// Metadata only.
    MetadataOnly,
    /// A redacted share.
    RedactedShare,
    /// Restricted by policy.
    PolicyRestricted,
    /// Raw bodies withheld.
    RawWithheld,
    /// No export at all.
    NoExport,
}

impl M5ProviderRedactionClass {
    /// Every redaction class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullBodyVisible,
        Self::MetadataOnly,
        Self::RedactedShare,
        Self::PolicyRestricted,
        Self::RawWithheld,
        Self::NoExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullBodyVisible => "full_body_visible",
            Self::MetadataOnly => "metadata_only",
            Self::RedactedShare => "redacted_share",
            Self::PolicyRestricted => "policy_restricted",
            Self::RawWithheld => "raw_withheld",
            Self::NoExport => "no_export",
        }
    }
}

/// Controlled export boundary — the metadata-safe boundary a privacy/redaction row keeps,
/// so no surface invents an alternate label for a metadata-safe export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExportBoundaryClass {
    /// Metadata-safe export.
    MetadataSafe,
    /// Bodies excluded from export.
    BodyExcluded,
    /// Credentials scrubbed from export.
    CredentialsScrubbed,
    /// Endpoints masked in export.
    EndpointsMasked,
    /// Local-only, never exported.
    LocalOnly,
    /// Full disclosure is blocked.
    FullDisclosureBlocked,
}

impl M5ExportBoundaryClass {
    /// Every export boundary class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MetadataSafe,
        Self::BodyExcluded,
        Self::CredentialsScrubbed,
        Self::EndpointsMasked,
        Self::LocalOnly,
        Self::FullDisclosureBlocked,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafe => "metadata_safe",
            Self::BodyExcluded => "body_excluded",
            Self::CredentialsScrubbed => "credentials_scrubbed",
            Self::EndpointsMasked => "endpoints_masked",
            Self::LocalOnly => "local_only",
            Self::FullDisclosureBlocked => "full_disclosure_blocked",
        }
    }
}

/// Claimed M5 provider surface family that renders / consumes a provider-account /
/// offline-capture component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderSurfaceFamily {
    /// The issue-workflow surface.
    IssueWorkflow,
    /// The review-workflow surface.
    ReviewWorkflow,
    /// The incident-workflow surface.
    IncidentWorkflow,
    /// The support-workflow surface.
    SupportWorkflow,
    /// The provider-settings surface.
    ProviderSettings,
    /// The CLI provider surface.
    CliProvider,
}

impl M5ProviderSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::IssueWorkflow,
        Self::ReviewWorkflow,
        Self::IncidentWorkflow,
        Self::SupportWorkflow,
        Self::ProviderSettings,
        Self::CliProvider,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IssueWorkflow => "issue_workflow",
            Self::ReviewWorkflow => "review_workflow",
            Self::IncidentWorkflow => "incident_workflow",
            Self::SupportWorkflow => "support_workflow",
            Self::ProviderSettings => "provider_settings",
            Self::CliProvider => "cli_provider",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's
/// connection, scope, mapping, sync, or redaction truth never silently narrows or widens
/// between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderDeploymentLine {
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

impl M5ProviderDeploymentLine {
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

/// Provider subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderConsumerSurface {
    /// The account-settings UI.
    AccountSettingsUi,
    /// The mapping-picker UI.
    MappingPickerUi,
    /// The sync-status UI.
    SyncStatusUi,
    /// The offline-queue UI.
    OfflineQueueUi,
    /// The privacy-review UI.
    PrivacyReviewUi,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The status-bar UI.
    StatusBarUi,
    /// The general product UI.
    ProductUi,
}

impl M5ProviderConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::AccountSettingsUi,
        Self::MappingPickerUi,
        Self::SyncStatusUi,
        Self::OfflineQueueUi,
        Self::PrivacyReviewUi,
        Self::SupportExport,
        Self::CliInspect,
        Self::StatusBarUi,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccountSettingsUi => "account_settings_ui",
            Self::MappingPickerUi => "mapping_picker_ui",
            Self::SyncStatusUi => "sync_status_ui",
            Self::OfflineQueueUi => "offline_queue_ui",
            Self::PrivacyReviewUi => "privacy_review_ui",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::StatusBarUi => "status_bar_ui",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no provider truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderAccessibilityRoute {
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

impl M5ProviderAccessibilityRoute {
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

/// Mandatory label a claimed provider-account / offline-capture component must be able to
/// show. The first three are hard requirements on every component; the remaining three
/// close the acceptance-criteria ambiguity about connection/scope, mapping/sync mode, and
/// redaction / export boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderRequiredLabel {
    /// The component's stable identity / what provider object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The connection state and tenant / write scope behind the component.
    ConnectionAndScope,
    /// The mapping origin and sync mode behind the component.
    MappingAndSyncMode,
    /// The redaction class and export boundary behind the component.
    RedactionAndExportBoundary,
}

impl M5ProviderRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ConnectionAndScope,
        Self::MappingAndSyncMode,
        Self::RedactionAndExportBoundary,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ConnectionAndScope => "connection_and_scope",
            Self::MappingAndSyncMode => "mapping_and_sync_mode",
            Self::RedactionAndExportBoundary => "redaction_and_export_boundary",
        }
    }
}

/// Qualification class for an M5 provider-account / offline-capture component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderQualificationClass {
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

impl M5ProviderQualificationClass {
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

/// Downgrade trigger that narrows a provider-account / offline-capture component below its
/// claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProviderDowngradeTrigger {
    /// An account row left its connection state unstated.
    ConnectionStateUnstated,
    /// An account row left its tenant scope unstated.
    TenantScopeUnstated,
    /// A mapping row left its mapping origin unstated.
    MappingOriginUnstated,
    /// A sync row left its sync mode unstated.
    SyncModeUnstated,
    /// A sync row left its effective write scope unstated.
    WriteScopeUnstated,
    /// A queued-draft state was hidden.
    QueuedDraftStateHidden,
    /// An offline-capture row left its capture state unstated.
    OfflineCaptureStateUnstated,
    /// A privacy row left its redaction class unstated.
    RedactionClassUnstated,
    /// A privacy row hid its export boundary.
    ExportBoundaryHidden,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// A default publish destination was assumed without disclosure.
    DefaultDestinationAssumed,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5ProviderDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConnectionStateUnstated,
        Self::TenantScopeUnstated,
        Self::MappingOriginUnstated,
        Self::SyncModeUnstated,
        Self::WriteScopeUnstated,
        Self::QueuedDraftStateHidden,
        Self::OfflineCaptureStateUnstated,
        Self::RedactionClassUnstated,
        Self::ExportBoundaryHidden,
        Self::AlternateStateLabelInvented,
        Self::DefaultDestinationAssumed,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConnectionStateUnstated => "connection_state_unstated",
            Self::TenantScopeUnstated => "tenant_scope_unstated",
            Self::MappingOriginUnstated => "mapping_origin_unstated",
            Self::SyncModeUnstated => "sync_mode_unstated",
            Self::WriteScopeUnstated => "write_scope_unstated",
            Self::QueuedDraftStateHidden => "queued_draft_state_hidden",
            Self::OfflineCaptureStateUnstated => "offline_capture_state_unstated",
            Self::RedactionClassUnstated => "redaction_class_unstated",
            Self::ExportBoundaryHidden => "export_boundary_hidden",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::DefaultDestinationAssumed => "default_destination_assumed",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed provider-account / offline-capture component family
/// bound to the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountOfflineComponentRow {
    /// Governed component family.
    pub component_family: M5ProviderAccountOfflineComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5ProviderQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 provider surface families that render / consume this component.
    pub surface_families: Vec<M5ProviderSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5ProviderDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5ProviderRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5ProviderRequiredLabel>,
    /// Provider identity classes this component names (provider-account-row only).
    pub provider_identity_classes: Vec<M5ProviderIdentityClass>,
    /// Account connection states this component names (provider-account-row only).
    pub account_connection_states: Vec<M5AccountConnectionState>,
    /// Tenant scopes this component names (provider-account-row only).
    pub tenant_scopes: Vec<M5TenantScopeClass>,
    /// Mapping origins this component names (project-or-board-mapping-row only).
    pub mapping_origins: Vec<M5MappingOriginClass>,
    /// Mapping target kinds this component names (project-or-board-mapping-row only).
    pub mapping_target_kinds: Vec<M5MappingTargetKind>,
    /// Sync modes this component names (sync-behavior-row only).
    pub sync_modes: Vec<M5ProviderSyncMode>,
    /// Effective write scopes this component names (sync-behavior-row only).
    pub write_scopes: Vec<M5ProviderWriteScope>,
    /// Offline-capture states this component names (offline-capture-row only).
    pub offline_capture_states: Vec<M5OfflineCaptureState>,
    /// Queued-draft states this component names (sync-behavior-row and
    /// offline-capture-row).
    pub queued_draft_states: Vec<M5QueuedDraftState>,
    /// Redaction classes this component names (privacy-redaction-row only).
    pub redaction_classes: Vec<M5ProviderRedactionClass>,
    /// Export boundaries this component names (privacy-redaction-row only).
    pub export_boundaries: Vec<M5ExportBoundaryClass>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5ProviderAccessibilityRoute>,
    /// Provider subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5ProviderConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5ProviderDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its connection state or tenant / write
    /// scope. MUST be `false`.
    pub masks_connection_or_scope: bool,
    /// Hard invariant: this component never hides its redaction class or export boundary.
    /// MUST be `false`.
    pub hides_export_or_redaction_boundary: bool,
    /// Hard invariant: this component never invents an alternate label for a governed
    /// state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: this component never assumes a default publish destination
    /// silently. MUST be `false`.
    pub assumes_default_destination_silently: bool,
}

impl M5ProviderAccountOfflineComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ProviderRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ProviderRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_connection_or_scope
            && !self.hides_export_or_redaction_boundary
            && !self.invents_alternate_state_label
            && !self.assumes_default_destination_silently
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountOfflineComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Provider-identity-class tokens.
    pub provider_identity_classes: Vec<String>,
    /// Account-connection-state tokens.
    pub account_connection_states: Vec<String>,
    /// Tenant-scope tokens.
    pub tenant_scopes: Vec<String>,
    /// Mapping-origin tokens.
    pub mapping_origins: Vec<String>,
    /// Mapping-target-kind tokens.
    pub mapping_target_kinds: Vec<String>,
    /// Sync-mode tokens.
    pub sync_modes: Vec<String>,
    /// Write-scope tokens.
    pub write_scopes: Vec<String>,
    /// Offline-capture-state tokens.
    pub offline_capture_states: Vec<String>,
    /// Queued-draft-state tokens.
    pub queued_draft_states: Vec<String>,
    /// Redaction-class tokens.
    pub redaction_classes: Vec<String>,
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

impl M5ProviderAccountOfflineComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5ProviderAccountOfflineComponentFamily::ALL, |v| {
                v.as_str()
            }),
            provider_identity_classes: tokens(&M5ProviderIdentityClass::ALL, |v| v.as_str()),
            account_connection_states: tokens(&M5AccountConnectionState::ALL, |v| v.as_str()),
            tenant_scopes: tokens(&M5TenantScopeClass::ALL, |v| v.as_str()),
            mapping_origins: tokens(&M5MappingOriginClass::ALL, |v| v.as_str()),
            mapping_target_kinds: tokens(&M5MappingTargetKind::ALL, |v| v.as_str()),
            sync_modes: tokens(&M5ProviderSyncMode::ALL, |v| v.as_str()),
            write_scopes: tokens(&M5ProviderWriteScope::ALL, |v| v.as_str()),
            offline_capture_states: tokens(&M5OfflineCaptureState::ALL, |v| v.as_str()),
            queued_draft_states: tokens(&M5QueuedDraftState::ALL, |v| v.as_str()),
            redaction_classes: tokens(&M5ProviderRedactionClass::ALL, |v| v.as_str()),
            export_boundaries: tokens(&M5ExportBoundaryClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ProviderSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ProviderDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ProviderConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ProviderAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ProviderRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5ProviderAccountOfflineComponentGovernanceReview {
    /// The provider-account row shows its connection state and tenant / write scope.
    pub account_row_shows_connection_and_scope: bool,
    /// The project/board mapping row shows its origin and default-destination target.
    pub mapping_row_shows_origin_and_target: bool,
    /// The sync-behavior row shows its sync mode and effective write scope.
    pub sync_row_shows_mode_and_write_scope: bool,
    /// The offline-capture row shows its capture state and queued-draft state.
    pub offline_row_shows_capture_and_queued_state: bool,
    /// The privacy/redaction row shows its redaction class and export boundary.
    pub privacy_row_shows_redaction_and_export_boundary: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The `not_configured` / `signed_in` / `limited_scope` / `stale_session` /
    /// `offline_cached_read` / `policy_blocked` states are named once.
    pub connection_state_vocabulary_named_once: bool,
    /// Mapping origin, sync mode, offline-capture state, and export boundary are each
    /// named once.
    pub mapping_sync_offline_export_named_once: bool,
    /// The default publish destination is always explicit.
    pub default_destination_always_explicit: bool,
    /// The effective write scope is always explicit.
    pub write_scope_always_explicit: bool,
    /// The queued-draft state is always explicit.
    pub queued_draft_state_always_explicit: bool,
    /// The export boundary is always explicit.
    pub export_boundary_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel provider vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountOfflineComponentConsumerProjection {
    /// Account surfaces consume the shared connection-state vocabulary.
    pub account_surfaces_consume_connection_vocabulary: bool,
    /// Mapping surfaces consume the mapping-origin vocabulary.
    pub mapping_surfaces_consume_origin_vocabulary: bool,
    /// Sync surfaces consume the sync-mode and write-scope vocabulary.
    pub sync_surfaces_consume_mode_vocabulary: bool,
    /// Offline surfaces consume the offline-capture and queued-draft vocabulary.
    pub offline_surfaces_consume_capture_vocabulary: bool,
    /// Privacy surfaces consume the redaction and export-boundary vocabulary.
    pub privacy_surfaces_consume_redaction_and_export_vocabulary: bool,
    /// Support / export reads a single canonical provider source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountOfflineComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the provider-account / offline-capture
/// component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountOfflineComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting provider-account audit for the lane.
    pub provider_account_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ProviderAccountOfflineComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProviderAccountOfflineComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ProviderAccountOfflineComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProviderAccountOfflineComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProviderAccountOfflineComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderAccountOfflineComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderAccountOfflineComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProviderAccountOfflineComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 provider-account / offline-capture component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderAccountOfflineComponentMatrixPacket {
    /// Record kind; must equal
    /// [`M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ProviderAccountOfflineComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ProviderAccountOfflineComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ProviderAccountOfflineComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ProviderAccountOfflineComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ProviderAccountOfflineComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ProviderAccountOfflineComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ProviderAccountOfflineComponentMatrixPacket {
    /// Builds an M5 provider-account / offline-capture component matrix packet from
    /// stable-lane input.
    pub fn new(input: M5ProviderAccountOfflineComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_RECORD_KIND
                .to_owned(),
            schema_version: M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_SCHEMA_VERSION,
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

    /// Validates the M5 provider-account / offline-capture component matrix invariants.
    pub fn validate(&self) -> Vec<M5ProviderAccountOfflineComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version
            != M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_MATRIX_SCHEMA_VERSION
        {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 provider-account offline-capture component matrix packet serializes"),
        ) {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 provider-account offline-capture component matrix packet serializes")
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
        out.push_str(
            "# M5 Provider-Account-Row, Project-or-Board-Mapping-Row, Sync-Behavior-Row, Offline-Capture-Row, and Privacy-Redaction-Row Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Account connection states: {}\n",
            self.vocabulary_set.account_connection_states.join(", ")
        ));
        out.push_str(&format!(
            "- Sync modes: {}\n",
            self.vocabulary_set.sync_modes.join(", ")
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

/// Errors emitted when reading the checked-in M5 provider-account matrix export.
#[derive(Debug)]
pub enum M5ProviderAccountOfflineComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ProviderAccountOfflineComponentMatrixViolation>),
}

impl fmt::Display for M5ProviderAccountOfflineComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 provider-account offline-capture component matrix export parse failed: {error}"
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
                    "m5 provider-account offline-capture component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ProviderAccountOfflineComponentMatrixArtifactError {}

/// Validation failures emitted by
/// [`M5ProviderAccountOfflineComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ProviderAccountOfflineComponentMatrixViolation {
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
    /// A provider-account-row component declares no provider identity classes.
    ProviderIdentityClassMissing,
    /// A provider-account-row component declares no account connection states.
    AccountConnectionStateMissing,
    /// A provider-account-row component declares no tenant scopes.
    TenantScopeMissing,
    /// A project-or-board-mapping-row component declares no mapping origins.
    MappingOriginMissing,
    /// A project-or-board-mapping-row component declares no mapping target kinds.
    MappingTargetKindMissing,
    /// A sync-behavior-row component declares no sync modes.
    SyncModeMissing,
    /// A sync-behavior-row component declares no write scopes.
    WriteScopeMissing,
    /// An offline-capture-row component declares no offline-capture states.
    OfflineCaptureStateMissing,
    /// A sync-behavior-row or offline-capture-row component declares no queued-draft
    /// states.
    QueuedDraftStateMissing,
    /// A privacy-redaction-row component declares no redaction classes.
    RedactionClassMissing,
    /// A privacy-redaction-row component declares no export boundaries.
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
    /// A component violates a hard invariant (masked connection/scope, hidden export or
    /// redaction boundary, invented alternate state label, or silently assumed default
    /// destination).
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

impl M5ProviderAccountOfflineComponentMatrixViolation {
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
            Self::ProviderIdentityClassMissing => "provider_identity_class_missing",
            Self::AccountConnectionStateMissing => "account_connection_state_missing",
            Self::TenantScopeMissing => "tenant_scope_missing",
            Self::MappingOriginMissing => "mapping_origin_missing",
            Self::MappingTargetKindMissing => "mapping_target_kind_missing",
            Self::SyncModeMissing => "sync_mode_missing",
            Self::WriteScopeMissing => "write_scope_missing",
            Self::OfflineCaptureStateMissing => "offline_capture_state_missing",
            Self::QueuedDraftStateMissing => "queued_draft_state_missing",
            Self::RedactionClassMissing => "redaction_class_missing",
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

/// Reads and validates the checked-in stable M5 provider-account matrix export.
pub fn current_stable_m5_provider_account_offline_capture_component_matrix_export() -> Result<
    M5ProviderAccountOfflineComponentMatrixPacket,
    M5ProviderAccountOfflineComponentMatrixArtifactError,
> {
    let packet: M5ProviderAccountOfflineComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-provider-account-offline-capture-proof/support_export.json"
        )))
        .map_err(M5ProviderAccountOfflineComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProviderAccountOfflineComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ProviderAccountOfflineComponentMatrixPacket,
    violations: &mut Vec<M5ProviderAccountOfflineComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SCHEMA_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_DOC_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_CONNECTED_ACCOUNT_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_TARGET_MAPPING_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_SYNC_HEALTH_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_OFFLINE_HANDOFF_REF,
        M5_PROVIDER_ACCOUNT_OFFLINE_CAPTURE_COMPONENT_EXPORT_REDACTION_REF,
    ] {
        if !refs.contains(required) {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ProviderAccountOfflineComponentMatrixPacket,
    violations: &mut Vec<M5ProviderAccountOfflineComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ProviderAccountOfflineComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5ProviderAccountOfflineComponentMatrixPacket,
    violations: &mut Vec<M5ProviderAccountOfflineComponentMatrixViolation>,
) {
    let present: BTreeSet<M5ProviderAccountOfflineComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5ProviderAccountOfflineComponentFamily::ALL {
        if !present.contains(&required) {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::RequiredComponentMissing);
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
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_provider_account_row() && row.provider_identity_classes.is_empty() {
            violations.push(
                M5ProviderAccountOfflineComponentMatrixViolation::ProviderIdentityClassMissing,
            );
        }
        if family.is_provider_account_row() && row.account_connection_states.is_empty() {
            violations.push(
                M5ProviderAccountOfflineComponentMatrixViolation::AccountConnectionStateMissing,
            );
        }
        if family.is_provider_account_row() && row.tenant_scopes.is_empty() {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::TenantScopeMissing);
        }
        if family.is_project_or_board_mapping_row() && row.mapping_origins.is_empty() {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::MappingOriginMissing);
        }
        if family.is_project_or_board_mapping_row() && row.mapping_target_kinds.is_empty() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::MappingTargetKindMissing);
        }
        if family.is_sync_behavior_row() && row.sync_modes.is_empty() {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::SyncModeMissing);
        }
        if family.is_sync_behavior_row() && row.write_scopes.is_empty() {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::WriteScopeMissing);
        }
        if family.is_offline_capture_row() && row.offline_capture_states.is_empty() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::OfflineCaptureStateMissing);
        }
        // Queued-draft state is shared by the sync-behavior row and the offline-capture
        // row.
        if (family.is_sync_behavior_row() || family.is_offline_capture_row())
            && row.queued_draft_states.is_empty()
        {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::QueuedDraftStateMissing);
        }
        if family.is_privacy_redaction_row() && row.redaction_classes.is_empty() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::RedactionClassMissing);
        }
        if family.is_privacy_redaction_row() && row.export_boundaries.is_empty() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::ExportBoundaryMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ProviderAccountOfflineComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(
                M5ProviderAccountOfflineComponentMatrixViolation::StableComponentMissingProof,
            );
        }
        if !row.honours_invariants() {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ProviderAccountOfflineComponentMatrixPacket,
    violations: &mut Vec<M5ProviderAccountOfflineComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.account_row_shows_connection_and_scope,
        review.mapping_row_shows_origin_and_target,
        review.sync_row_shows_mode_and_write_scope,
        review.offline_row_shows_capture_and_queued_state,
        review.privacy_row_shows_redaction_and_export_boundary,
        review.no_surface_invents_alternate_state_label,
        review.connection_state_vocabulary_named_once,
        review.mapping_sync_offline_export_named_once,
        review.default_destination_always_explicit,
        review.write_scope_always_explicit,
        review.queued_draft_state_always_explicit,
        review.export_boundary_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5ProviderAccountOfflineComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ProviderAccountOfflineComponentMatrixPacket,
    violations: &mut Vec<M5ProviderAccountOfflineComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.account_surfaces_consume_connection_vocabulary,
        projection.mapping_surfaces_consume_origin_vocabulary,
        projection.sync_surfaces_consume_mode_vocabulary,
        projection.offline_surfaces_consume_capture_vocabulary,
        projection.privacy_surfaces_consume_redaction_and_export_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(
                M5ProviderAccountOfflineComponentMatrixViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ProviderAccountOfflineComponentMatrixPacket,
    violations: &mut Vec<M5ProviderAccountOfflineComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ProviderAccountOfflineComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ProviderAccountOfflineComponentMatrixPacket,
    violations: &mut Vec<M5ProviderAccountOfflineComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.provider_account_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ProviderAccountOfflineComponentMatrixViolation::ReleasePostureIncomplete);
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
