//! Frozen M5 notification-row, mobile-review-card, CI-status-card, session-follow-tile,
//! incident-snapshot-card, and desktop-handoff-sheet component matrix.
//!
//! This module locks Aureline's reusable companion-client components into one export-safe
//! packet. Every companion-surface subcomponent M5 claims that still drifts too easily by
//! browser, mobile, desktop-panel, diagnostics, support, or Help/About surface — the
//! notification row, the mobile review card, the CI-status card, the session-follow tile,
//! the incident-snapshot card, and the desktop-handoff sheet — is named once here and
//! constrained by the same object identity, workspace/repo client scope, freshness, companion
//! disposition (review-only, comment-capable, desktop-required, cached, stale, policy-blocked,
//! handoff-ready), severity, and exact desktop-handoff target regardless of the surface family
//! that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families, the object kinds each component binds, the controlled client scopes,
//! the freshness classes, the single controlled disposition vocabulary consumers bind to, the
//! severity classes, the CI statuses, the review kinds, the session-follow states, the
//! notification categories, the exact handoff targets, the deployment lines every component
//! must survive, the non-visual accessibility routes, and the mandatory labels every
//! component must be able to show. It does not re-architect the companion triage, session
//! follow, incident awareness, or desktop-handoff surfaces that already own those records — it
//! is the shared companion-component contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 companion surface may
//! publish an object, scope, freshness, capability-boundary, severity, or handoff-target
//! claim. Notification, review, CI, session-follow, incident, and handoff consumers all read
//! this packet so one notification row names which object a tap opens and its severity, one
//! mobile review card names whether it is review-only or comment-capable, one CI-status card
//! names its status and freshness, one session-follow tile names its scope and whether it is
//! live or stale, one incident-snapshot card names its severity and freshness, and one
//! desktop-handoff sheet names the exact target it will open on desktop. No M5 companion lane
//! invents a second companion grammar or an alternate label for a stale card, a policy-blocked
//! action, a desktop-required action, or an exact handoff target.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5CompanionComponentVocabularySet`] rather than minted per surface. Raw file bodies,
//! diff hunks, secret values, and private endpoints stay outside the export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_companion_component_matrix,
    seeded_m5_companion_component_matrix_desktop_handoff_sheet_preview_narrowed,
    seeded_m5_companion_component_matrix_session_follow_tile_beta_narrowed,
    M5_COMPANION_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CompanionComponentMatrixPacket`].
pub const M5_COMPANION_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_m5_notification_row_mobile_review_card_ci_status_card_session_follow_tile_incident_snapshot_card_and_desktop_handoff_sheet_component_matrix";

/// Schema version for M5 companion component-matrix records.
pub const M5_COMPANION_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined companion component-matrix boundary schema.
pub const M5_COMPANION_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-companion-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_COMPANION_COMPONENT_DOC_REF: &str = "docs/companion/m5_companion_component_matrix.md";

/// Repo-relative path of the notification-row canonical component schema.
pub const M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-companion-notification-row.schema.json";

/// Repo-relative path of the mobile-review-card canonical component schema.
pub const M5_MOBILE_REVIEW_CARD_SCHEMA_REF: &str = "schemas/ui/m5-mobile-review-card.schema.json";

/// Repo-relative path of the CI-status-card canonical component schema.
pub const M5_CI_STATUS_CARD_SCHEMA_REF: &str = "schemas/ui/m5-ci-status-card.schema.json";

/// Repo-relative path of the session-follow-tile canonical component schema.
pub const M5_SESSION_FOLLOW_TILE_SCHEMA_REF: &str = "schemas/ui/m5-session-follow-tile.schema.json";

/// Repo-relative path of the incident-snapshot-card canonical component schema.
pub const M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF: &str =
    "schemas/ui/m5-incident-snapshot-card.schema.json";

/// Repo-relative path of the desktop-handoff-sheet canonical component schema.
pub const M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF: &str =
    "schemas/ui/m5-desktop-handoff-sheet.schema.json";

/// Repo-relative path of the companion triage foundation contract the notification row,
/// mobile review card, and CI-status card bind against.
pub const M5_COMPANION_COMPONENT_FOUNDATION_TRIAGE_REF: &str =
    "schemas/companion/companion-notification-triage-review-queues-and-ci-status-cards-with-desktop-handoff.schema.json";

/// Repo-relative path of the session-follow / incident-awareness foundation contract the
/// session-follow tile and incident-snapshot card bind against.
pub const M5_COMPANION_COMPONENT_FOUNDATION_SESSION_FOLLOW_REF: &str =
    "schemas/companion/ship-session-follow-and-incident-awareness-surfaces-with-bounded-read-write-scope-and-stale-state-honesty.schema.json";

/// Repo-relative path of the frozen companion matrix foundation contract the desktop-handoff
/// sheet binds against.
pub const M5_COMPANION_COMPONENT_FOUNDATION_MATRIX_REF: &str =
    "schemas/companion/freeze-the-m5-companion-incident-sync-and-offboarding-matrix-with-staged-rollout-lanes.schema.json";

/// Repo-relative path of the companion surface contract doc.
pub const M5_COMPANION_COMPONENT_FOUNDATION_SURFACE_CONTRACT_REF: &str =
    "docs/companion/companion_surface_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_COMPANION_COMPONENT_FIXTURE_DIR: &str = "fixtures/ui/m5-companion-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_COMPANION_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-companion-component-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_COMPANION_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-companion-component-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_COMPANION_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-companion-component-matrix.md";

/// One of the six governed companion component families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionComponentFamily {
    /// A notification row carrying its object identity, category, and severity.
    NotificationRow,
    /// A mobile review card carrying its review kind and companion capability boundary.
    MobileReviewCard,
    /// A CI-status card carrying its pipeline status and freshness.
    CiStatusCard,
    /// A session-follow tile carrying its follow state, scope, and freshness.
    SessionFollowTile,
    /// An incident-snapshot card carrying its severity and freshness.
    IncidentSnapshotCard,
    /// A desktop-handoff sheet carrying the exact target it will open on desktop.
    DesktopHandoffSheet,
}

impl M5CompanionComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotificationRow,
        Self::MobileReviewCard,
        Self::CiStatusCard,
        Self::SessionFollowTile,
        Self::IncidentSnapshotCard,
        Self::DesktopHandoffSheet,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationRow => "notification_row",
            Self::MobileReviewCard => "mobile_review_card",
            Self::CiStatusCard => "ci_status_card",
            Self::SessionFollowTile => "session_follow_tile",
            Self::IncidentSnapshotCard => "incident_snapshot_card",
            Self::DesktopHandoffSheet => "desktop_handoff_sheet",
        }
    }

    /// The canonical per-component schema ref a downstream row points at instead of restating
    /// this component's companion truth by hand.
    pub const fn canonical_component_schema_ref(self) -> &'static str {
        match self {
            Self::NotificationRow => M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
            Self::MobileReviewCard => M5_MOBILE_REVIEW_CARD_SCHEMA_REF,
            Self::CiStatusCard => M5_CI_STATUS_CARD_SCHEMA_REF,
            Self::SessionFollowTile => M5_SESSION_FOLLOW_TILE_SCHEMA_REF,
            Self::IncidentSnapshotCard => M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF,
            Self::DesktopHandoffSheet => M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
        }
    }

    /// `true` when this family must declare a controlled severity.
    pub const fn declares_severity(self) -> bool {
        matches!(self, Self::NotificationRow | Self::IncidentSnapshotCard)
    }

    /// `true` when this family must declare a controlled review kind.
    pub const fn declares_review_kind(self) -> bool {
        matches!(self, Self::MobileReviewCard)
    }

    /// `true` when this family must declare a controlled CI status.
    pub const fn declares_ci_status(self) -> bool {
        matches!(self, Self::CiStatusCard)
    }

    /// `true` when this family must declare a controlled session-follow state.
    pub const fn declares_session_follow_state(self) -> bool {
        matches!(self, Self::SessionFollowTile)
    }

    /// `true` when this family must declare a controlled handoff target.
    pub const fn declares_handoff_target(self) -> bool {
        matches!(self, Self::DesktopHandoffSheet)
    }

    /// `true` when this family must declare a controlled notification category.
    pub const fn declares_notification_category(self) -> bool {
        matches!(self, Self::NotificationRow)
    }
}

/// Controlled object kind — which object a companion component binds, so a user never has to
/// infer what a tap opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionObjectKind {
    /// A notification event.
    NotificationEvent,
    /// A review item (change set, diff, thread, or approval).
    ReviewItem,
    /// A CI pipeline run.
    CiRun,
    /// A followed desktop session.
    FollowedSession,
    /// An incident record.
    IncidentRecord,
    /// A desktop-handoff intent.
    HandoffIntent,
}

impl M5CompanionObjectKind {
    /// Every object kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NotificationEvent,
        Self::ReviewItem,
        Self::CiRun,
        Self::FollowedSession,
        Self::IncidentRecord,
        Self::HandoffIntent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationEvent => "notification_event",
            Self::ReviewItem => "review_item",
            Self::CiRun => "ci_run",
            Self::FollowedSession => "followed_session",
            Self::IncidentRecord => "incident_record",
            Self::HandoffIntent => "handoff_intent",
        }
    }
}

/// Controlled client scope — the workspace/repo/account boundary a companion component is
/// scoped to, so scope is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionClientScope {
    /// Scoped to one workspace.
    WorkspaceScoped,
    /// Scoped to one repository.
    RepoScoped,
    /// Scoped to one organization.
    OrgScoped,
    /// Scoped to one device.
    DeviceScoped,
    /// Scoped to the whole account.
    AccountGlobal,
    /// Not scoped to a concrete object yet.
    Unscoped,
}

impl M5CompanionClientScope {
    /// Every client scope, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WorkspaceScoped,
        Self::RepoScoped,
        Self::OrgScoped,
        Self::DeviceScoped,
        Self::AccountGlobal,
        Self::Unscoped,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceScoped => "workspace_scoped",
            Self::RepoScoped => "repo_scoped",
            Self::OrgScoped => "org_scoped",
            Self::DeviceScoped => "device_scoped",
            Self::AccountGlobal => "account_global",
            Self::Unscoped => "unscoped",
        }
    }
}

/// Controlled freshness class — whether a component is live, cached, or stale, so a stale card
/// is never shown as live.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionFreshness {
    /// Streaming live from the local core via the relay.
    Live,
    /// Last-known cached value.
    Cached,
    /// Stale beyond its freshness window.
    Stale,
    /// Held offline, pending reconnection.
    OfflineHeld,
    /// A snapshot that has expired.
    ExpiredSnapshot,
    /// Freshness cannot currently be determined.
    UnknownFreshness,
}

impl M5CompanionFreshness {
    /// Every freshness class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Live,
        Self::Cached,
        Self::Stale,
        Self::OfflineHeld,
        Self::ExpiredSnapshot,
        Self::UnknownFreshness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::OfflineHeld => "offline_held",
            Self::ExpiredSnapshot => "expired_snapshot",
            Self::UnknownFreshness => "unknown_freshness",
        }
    }
}

/// The single controlled disposition vocabulary every companion consumer binds to. These are
/// the exact acceptance-criteria tokens: `review-only`, `comment-capable`, `desktop required`,
/// `cached`, `stale`, `policy blocked`, and `handoff ready`. No companion surface invents a
/// parallel word for any of these dispositions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionComponentDisposition {
    /// The component is review-only from the companion — no comment or authoring path.
    ReviewOnly,
    /// The component can post a bounded comment from the companion.
    CommentCapable,
    /// The action is desktop-required and cannot complete on the companion.
    DesktopRequired,
    /// The component is showing a cached value, not a live one.
    Cached,
    /// The component is stale beyond its freshness window.
    Stale,
    /// The action is blocked by policy on the companion.
    PolicyBlocked,
    /// The component carries an exact, resolvable desktop handoff and is ready to hand off.
    HandoffReady,
}

impl M5CompanionComponentDisposition {
    /// Every disposition token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ReviewOnly,
        Self::CommentCapable,
        Self::DesktopRequired,
        Self::Cached,
        Self::Stale,
        Self::PolicyBlocked,
        Self::HandoffReady,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewOnly => "review_only",
            Self::CommentCapable => "comment_capable",
            Self::DesktopRequired => "desktop_required",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::HandoffReady => "handoff_ready",
        }
    }
}

/// Controlled severity — how urgent an object surfaced on a notification row or incident-
/// snapshot card is, so severity is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionSeverity {
    /// Critical — surfaced at the top of triage.
    Critical,
    /// High severity.
    High,
    /// Moderate severity.
    Moderate,
    /// Low severity.
    Low,
    /// Informational only.
    Informational,
    /// Severity is not yet classified.
    Unspecified,
}

impl M5CompanionSeverity {
    /// Every severity, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Critical,
        Self::High,
        Self::Moderate,
        Self::Low,
        Self::Informational,
        Self::Unspecified,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Moderate => "moderate",
            Self::Low => "low",
            Self::Informational => "informational",
            Self::Unspecified => "unspecified",
        }
    }
}

/// Controlled review kind — the kind of work queued on a mobile review card, so a card never
/// leaves the review kind implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionReviewKind {
    /// A pending agent change set.
    AgentChange,
    /// A diff awaiting review.
    DiffReview,
    /// A comment thread awaiting response.
    CommentThread,
    /// An explicit approval request.
    ApprovalRequest,
    /// A policy gate awaiting acknowledgement.
    PolicyGate,
    /// A merge-readiness summary.
    MergeReadiness,
}

impl M5CompanionReviewKind {
    /// Every review kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AgentChange,
        Self::DiffReview,
        Self::CommentThread,
        Self::ApprovalRequest,
        Self::PolicyGate,
        Self::MergeReadiness,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AgentChange => "agent_change",
            Self::DiffReview => "diff_review",
            Self::CommentThread => "comment_thread",
            Self::ApprovalRequest => "approval_request",
            Self::PolicyGate => "policy_gate",
            Self::MergeReadiness => "merge_readiness",
        }
    }
}

/// Controlled CI status — the pipeline status shown on a CI-status card, so status is never
/// left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionCiStatus {
    /// Pipeline passed.
    Passed,
    /// Pipeline failed.
    Failed,
    /// Pipeline is running.
    Running,
    /// Pipeline is queued.
    Queued,
    /// Pipeline was canceled.
    Canceled,
    /// Status is stale and could not be refreshed.
    Stale,
}

impl M5CompanionCiStatus {
    /// Every CI status, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Passed,
        Self::Failed,
        Self::Running,
        Self::Queued,
        Self::Canceled,
        Self::Stale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Running => "running",
            Self::Queued => "queued",
            Self::Canceled => "canceled",
            Self::Stale => "stale",
        }
    }
}

/// Controlled session-follow state — whether a session-follow tile is live, paused, diverged,
/// or ended, so a followed session's state is never shown greener than reality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionSessionFollowState {
    /// Live following the host session.
    LiveFollowing,
    /// Following is paused.
    PausedFollow,
    /// The companion has diverged from the host.
    DivergedFromHost,
    /// The host session is inactive.
    HostInactive,
    /// A read-only mirror of the last-known state.
    ReadOnlyMirror,
    /// Following has ended.
    FollowEnded,
}

impl M5CompanionSessionFollowState {
    /// Every session-follow state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LiveFollowing,
        Self::PausedFollow,
        Self::DivergedFromHost,
        Self::HostInactive,
        Self::ReadOnlyMirror,
        Self::FollowEnded,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveFollowing => "live_following",
            Self::PausedFollow => "paused_follow",
            Self::DivergedFromHost => "diverged_from_host",
            Self::HostInactive => "host_inactive",
            Self::ReadOnlyMirror => "read_only_mirror",
            Self::FollowEnded => "follow_ended",
        }
    }
}

/// Controlled handoff target — the exact desktop target a desktop-handoff sheet will open, so
/// a user always knows exactly what opens on desktop before a tap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionHandoffTarget {
    /// An exact file location (path plus position).
    FileLocation,
    /// The review panel for a specific item.
    ReviewPanel,
    /// A CI pipeline run view.
    CiPipelineRun,
    /// An incident workspace.
    IncidentWorkspace,
    /// A running agent session.
    AgentSession,
    /// No desktop handoff is available for this item.
    NoHandoff,
}

impl M5CompanionHandoffTarget {
    /// Every handoff target, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FileLocation,
        Self::ReviewPanel,
        Self::CiPipelineRun,
        Self::IncidentWorkspace,
        Self::AgentSession,
        Self::NoHandoff,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileLocation => "file_location",
            Self::ReviewPanel => "review_panel",
            Self::CiPipelineRun => "ci_pipeline_run",
            Self::IncidentWorkspace => "incident_workspace",
            Self::AgentSession => "agent_session",
            Self::NoHandoff => "no_handoff",
        }
    }
}

/// Controlled notification category — the kind of event a notification row surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionNotificationCategory {
    /// A build or compile event.
    Build,
    /// A review or approval event.
    Review,
    /// An agent run event.
    Agent,
    /// An incident or crash event.
    Incident,
    /// A managed-sync event.
    Sync,
    /// A mention or direct message.
    Mention,
}

impl M5CompanionNotificationCategory {
    /// Every notification category, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Build,
        Self::Review,
        Self::Agent,
        Self::Incident,
        Self::Sync,
        Self::Mention,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Review => "review",
            Self::Agent => "agent",
            Self::Incident => "incident",
            Self::Sync => "sync",
            Self::Mention => "mention",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a companion component. No component may
/// invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionSurfaceFamily {
    /// The browser companion surface.
    BrowserCompanion,
    /// The mobile companion surface.
    MobileCompanion,
    /// The desktop companion panel.
    DesktopPanel,
    /// The diagnostics surface.
    Diagnostics,
    /// The support export.
    SupportExport,
    /// The Help/About surface.
    HelpAbout,
}

impl M5CompanionSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BrowserCompanion,
        Self::MobileCompanion,
        Self::DesktopPanel,
        Self::Diagnostics,
        Self::SupportExport,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BrowserCompanion => "browser_companion",
            Self::MobileCompanion => "mobile_companion",
            Self::DesktopPanel => "desktop_panel",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's object,
/// scope, freshness, capability, severity, or handoff truth never silently narrows or widens
/// between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionDeploymentLine {
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

impl M5CompanionDeploymentLine {
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

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionConsumerSurface {
    /// The notification-triage UI.
    NotificationTriageUi,
    /// The review-queue UI.
    ReviewQueueUi,
    /// The CI-status UI.
    CiStatusUi,
    /// The session-follow UI.
    SessionFollowUi,
    /// The incident-awareness UI.
    IncidentAwarenessUi,
    /// The desktop-handoff UI.
    DesktopHandoffUi,
    /// The support export.
    SupportExport,
    /// The status-bar UI.
    StatusBarUi,
    /// The general product UI.
    ProductUi,
}

impl M5CompanionConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::NotificationTriageUi,
        Self::ReviewQueueUi,
        Self::CiStatusUi,
        Self::SessionFollowUi,
        Self::IncidentAwarenessUi,
        Self::DesktopHandoffUi,
        Self::SupportExport,
        Self::StatusBarUi,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotificationTriageUi => "notification_triage_ui",
            Self::ReviewQueueUi => "review_queue_ui",
            Self::CiStatusUi => "ci_status_ui",
            Self::SessionFollowUi => "session_follow_ui",
            Self::IncidentAwarenessUi => "incident_awareness_ui",
            Self::DesktopHandoffUi => "desktop_handoff_ui",
            Self::SupportExport => "support_export",
            Self::StatusBarUi => "status_bar_ui",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no companion truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionAccessibilityRoute {
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

impl M5CompanionAccessibilityRoute {
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

/// Reason a companion component has degraded below its qualified state. Required on every row
/// so a session-only, offline, or policy-blocked fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionDegradedReason {
    /// The companion relay is unavailable.
    RelayUnavailable,
    /// Proof has gone stale.
    ProofStale,
    /// No active desktop host session.
    HostSessionInactive,
    /// Workspace or device trust narrowed.
    TrustNarrowed,
    /// An upstream frozen matrix lane narrowed.
    UpstreamMatrixNarrowed,
    /// One or more handoff targets could not resolve exactly.
    HandoffTargetUnresolved,
}

impl M5CompanionDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RelayUnavailable,
        Self::ProofStale,
        Self::HostSessionInactive,
        Self::TrustNarrowed,
        Self::UpstreamMatrixNarrowed,
        Self::HandoffTargetUnresolved,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RelayUnavailable => "relay_unavailable",
            Self::ProofStale => "proof_stale",
            Self::HostSessionInactive => "host_session_inactive",
            Self::TrustNarrowed => "trust_narrowed",
            Self::UpstreamMatrixNarrowed => "upstream_matrix_narrowed",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
        }
    }
}

/// Mandatory label a claimed companion component must be able to show. The first three are
/// hard requirements on every component; the remaining three close the acceptance-criteria
/// ambiguity about scope/freshness, capability boundary, and severity / handoff target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionRequiredLabel {
    /// The component's stable identity / what object it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The workspace/repo client scope and freshness behind the component.
    ScopeAndFreshness,
    /// The companion-versus-desktop capability boundary behind the component.
    CapabilityBoundary,
    /// The severity and exact desktop handoff target behind the component.
    SeverityAndHandoffTarget,
}

impl M5CompanionRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ScopeAndFreshness,
        Self::CapabilityBoundary,
        Self::SeverityAndHandoffTarget,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ScopeAndFreshness => "scope_and_freshness",
            Self::CapabilityBoundary => "capability_boundary",
            Self::SeverityAndHandoffTarget => "severity_and_handoff_target",
        }
    }
}

/// Qualification class for an M5 companion component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionQualificationClass {
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

impl M5CompanionQualificationClass {
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

/// Downgrade trigger that narrows a companion component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompanionDowngradeTrigger {
    /// A component left its object identity unstated.
    ObjectIdentityUnstated,
    /// A component left its client scope unstated.
    ClientScopeUnstated,
    /// A component hid its freshness.
    FreshnessHidden,
    /// A component left its companion-versus-desktop capability boundary unstated.
    CapabilityBoundaryUnstated,
    /// A component left its severity unstated.
    SeverityUnstated,
    /// A handoff target could not resolve exactly.
    HandoffTargetUnresolved,
    /// A component left its disposition unstated.
    DispositionUnstated,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// Generic companion wording concealed object, scope, or capability truth.
    GenericCompanionWordingUsed,
    /// A stale card was shown as live.
    StaleShownAsLive,
    /// A desktop-required action was offered inline as if companion-safe.
    DesktopRequiredActionOfferedInline,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5CompanionDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ObjectIdentityUnstated,
        Self::ClientScopeUnstated,
        Self::FreshnessHidden,
        Self::CapabilityBoundaryUnstated,
        Self::SeverityUnstated,
        Self::HandoffTargetUnresolved,
        Self::DispositionUnstated,
        Self::AlternateStateLabelInvented,
        Self::GenericCompanionWordingUsed,
        Self::StaleShownAsLive,
        Self::DesktopRequiredActionOfferedInline,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectIdentityUnstated => "object_identity_unstated",
            Self::ClientScopeUnstated => "client_scope_unstated",
            Self::FreshnessHidden => "freshness_hidden",
            Self::CapabilityBoundaryUnstated => "capability_boundary_unstated",
            Self::SeverityUnstated => "severity_unstated",
            Self::HandoffTargetUnresolved => "handoff_target_unresolved",
            Self::DispositionUnstated => "disposition_unstated",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::GenericCompanionWordingUsed => "generic_companion_wording_used",
            Self::StaleShownAsLive => "stale_shown_as_live",
            Self::DesktopRequiredActionOfferedInline => "desktop_required_action_offered_inline",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed companion component family bound to the surface-
/// specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionComponentRow {
    /// Governed component family.
    pub component_family: M5CompanionComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5CompanionQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this component.
    pub surface_families: Vec<M5CompanionSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5CompanionDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5CompanionRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5CompanionRequiredLabel>,
    /// Object kinds this component binds (required on every component).
    pub object_kinds: Vec<M5CompanionObjectKind>,
    /// Client scopes this component names (required on every component).
    pub client_scopes: Vec<M5CompanionClientScope>,
    /// Freshness classes this component names (required on every component).
    pub freshness_classes: Vec<M5CompanionFreshness>,
    /// Dispositions this component can carry (required on every component).
    pub dispositions: Vec<M5CompanionComponentDisposition>,
    /// Severities this component names (severity-declaring families only).
    pub severities: Vec<M5CompanionSeverity>,
    /// Review kinds this component names (mobile-review-card only).
    pub review_kinds: Vec<M5CompanionReviewKind>,
    /// CI statuses this component names (ci-status-card only).
    pub ci_statuses: Vec<M5CompanionCiStatus>,
    /// Session-follow states this component names (session-follow-tile only).
    pub session_follow_states: Vec<M5CompanionSessionFollowState>,
    /// Handoff targets this component names (desktop-handoff-sheet only).
    pub handoff_targets: Vec<M5CompanionHandoffTarget>,
    /// Notification categories this component names (notification-row only).
    pub notification_categories: Vec<M5CompanionNotificationCategory>,
    /// Degraded reasons this component can name (required on every component).
    pub degraded_reasons: Vec<M5CompanionDegradedReason>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5CompanionAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5CompanionConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5CompanionDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component (must include its own canonical
    /// component schema so downstream rows have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its client scope or freshness. MUST be
    /// `false`.
    pub masks_scope_or_freshness: bool,
    /// Hard invariant: this component never hides its companion-versus-desktop capability
    /// boundary. MUST be `false`.
    pub hides_capability_boundary: bool,
    /// Hard invariant: this component never invents an alternate label for a governed state.
    /// MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: this component never implies a desktop-required action is companion-
    /// safe. MUST be `false`.
    pub implies_desktop_action_is_companion_safe: bool,
}

impl M5CompanionComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5CompanionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5CompanionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_scope_or_freshness
            && !self.hides_capability_boundary
            && !self.invents_alternate_state_label
            && !self.implies_desktop_action_is_companion_safe
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Object-kind tokens.
    pub object_kinds: Vec<String>,
    /// Client-scope tokens.
    pub client_scopes: Vec<String>,
    /// Freshness-class tokens.
    pub freshness_classes: Vec<String>,
    /// Disposition tokens.
    pub dispositions: Vec<String>,
    /// Severity tokens.
    pub severities: Vec<String>,
    /// Review-kind tokens.
    pub review_kinds: Vec<String>,
    /// CI-status tokens.
    pub ci_statuses: Vec<String>,
    /// Session-follow-state tokens.
    pub session_follow_states: Vec<String>,
    /// Handoff-target tokens.
    pub handoff_targets: Vec<String>,
    /// Notification-category tokens.
    pub notification_categories: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5CompanionComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5CompanionComponentFamily::ALL, |v| v.as_str()),
            object_kinds: tokens(&M5CompanionObjectKind::ALL, |v| v.as_str()),
            client_scopes: tokens(&M5CompanionClientScope::ALL, |v| v.as_str()),
            freshness_classes: tokens(&M5CompanionFreshness::ALL, |v| v.as_str()),
            dispositions: tokens(&M5CompanionComponentDisposition::ALL, |v| v.as_str()),
            severities: tokens(&M5CompanionSeverity::ALL, |v| v.as_str()),
            review_kinds: tokens(&M5CompanionReviewKind::ALL, |v| v.as_str()),
            ci_statuses: tokens(&M5CompanionCiStatus::ALL, |v| v.as_str()),
            session_follow_states: tokens(&M5CompanionSessionFollowState::ALL, |v| v.as_str()),
            handoff_targets: tokens(&M5CompanionHandoffTarget::ALL, |v| v.as_str()),
            notification_categories: tokens(&M5CompanionNotificationCategory::ALL, |v| v.as_str()),
            surface_families: tokens(&M5CompanionSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5CompanionDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5CompanionConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5CompanionAccessibilityRoute::ALL, |v| v.as_str()),
            degraded_reasons: tokens(&M5CompanionDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5CompanionRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5CompanionComponentGovernanceReview {
    /// The notification row shows its object identity and severity.
    pub notification_row_shows_object_and_severity: bool,
    /// The mobile review card shows its companion-versus-desktop capability boundary.
    pub mobile_review_card_shows_capability_boundary: bool,
    /// The CI-status card shows its status and freshness.
    pub ci_status_card_shows_status_and_freshness: bool,
    /// The session-follow tile shows its scope and freshness.
    pub session_follow_tile_shows_scope_and_freshness: bool,
    /// The incident-snapshot card shows its severity and freshness.
    pub incident_snapshot_card_shows_severity_and_freshness: bool,
    /// The desktop-handoff sheet shows its exact target.
    pub desktop_handoff_sheet_shows_exact_target: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The object identity is always explicit.
    pub object_identity_always_explicit: bool,
    /// The client-scope vocabulary is named once.
    pub client_scope_named_once: bool,
    /// The freshness is always explicit.
    pub freshness_always_explicit: bool,
    /// The companion-versus-desktop capability boundary is always explicit.
    pub capability_boundary_always_explicit: bool,
    /// The severity is always explicit where it applies.
    pub severity_always_explicit: bool,
    /// The exact handoff target is always explicit before a tap.
    pub exact_handoff_target_always_explicit: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel companion vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionComponentConsumerProjection {
    /// Notification surfaces consume the shared severity vocabulary.
    pub notification_surfaces_consume_severity_vocabulary: bool,
    /// Review surfaces consume the shared capability/disposition vocabulary.
    pub review_surfaces_consume_capability_vocabulary: bool,
    /// CI surfaces consume the shared status vocabulary.
    pub ci_surfaces_consume_status_vocabulary: bool,
    /// Follow surfaces consume the shared freshness vocabulary.
    pub follow_surfaces_consume_freshness_vocabulary: bool,
    /// Handoff surfaces consume the shared target vocabulary.
    pub handoff_surfaces_consume_target_vocabulary: bool,
    /// Support / export reads a single canonical companion source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the companion component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting companion audit for the lane.
    pub companion_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CompanionComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CompanionComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5CompanionComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CompanionComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CompanionComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompanionComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompanionComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CompanionComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 companion component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CompanionComponentMatrixPacket {
    /// Record kind; must equal [`M5_COMPANION_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_COMPANION_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5CompanionComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CompanionComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CompanionComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CompanionComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CompanionComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CompanionComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CompanionComponentMatrixPacket {
    /// Builds an M5 companion component matrix packet from stable-lane input.
    pub fn new(input: M5CompanionComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_COMPANION_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_COMPANION_COMPONENT_MATRIX_SCHEMA_VERSION,
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

    /// Validates the M5 companion component matrix invariants.
    pub fn validate(&self) -> Vec<M5CompanionComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_COMPANION_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5CompanionComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_COMPANION_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5CompanionComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CompanionComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 companion component matrix packet serializes"),
        ) {
            violations.push(M5CompanionComponentMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 companion component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.component_family.canonical_component_schema_ref(),
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
            "# M5 Notification-Row, Mobile-Review-Card, CI-Status-Card, Session-Follow-Tile, Incident-Snapshot-Card, and Desktop-Handoff-Sheet Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Dispositions: {}\n",
            self.vocabulary_set.dispositions.join(", ")
        ));
        out.push_str(&format!(
            "- Freshness classes: {}\n",
            self.vocabulary_set.freshness_classes.join(", ")
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
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.component_family.canonical_component_schema_ref()
            ));
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

/// Errors emitted when reading the checked-in M5 companion matrix export.
#[derive(Debug)]
pub enum M5CompanionComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CompanionComponentMatrixViolation>),
}

impl fmt::Display for M5CompanionComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 companion component matrix export parse failed: {error}"
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
                    "m5 companion component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CompanionComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5CompanionComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CompanionComponentMatrixViolation {
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
    /// A component row does not point at its own canonical component schema.
    ComponentSchemaRefMissing,
    /// A component declares no object kinds.
    ObjectKindMissing,
    /// A component declares no client scopes.
    ClientScopeMissing,
    /// A component declares no freshness classes.
    FreshnessClassMissing,
    /// A component declares no dispositions.
    DispositionMissing,
    /// A severity-declaring component declares no severities.
    SeverityMissing,
    /// A mobile-review-card component declares no review kinds.
    ReviewKindMissing,
    /// A ci-status-card component declares no CI statuses.
    CiStatusMissing,
    /// A session-follow-tile component declares no session-follow states.
    SessionFollowStateMissing,
    /// A desktop-handoff-sheet component declares no handoff targets.
    HandoffTargetMissing,
    /// A notification-row component declares no notification categories.
    NotificationCategoryMissing,
    /// A component declares no degraded reasons.
    DegradedReasonMissing,
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
    /// A component violates a hard invariant (masked scope/freshness, hidden capability
    /// boundary, invented alternate state label, or implied companion-safe desktop action).
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

impl M5CompanionComponentMatrixViolation {
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
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ObjectKindMissing => "object_kind_missing",
            Self::ClientScopeMissing => "client_scope_missing",
            Self::FreshnessClassMissing => "freshness_class_missing",
            Self::DispositionMissing => "disposition_missing",
            Self::SeverityMissing => "severity_missing",
            Self::ReviewKindMissing => "review_kind_missing",
            Self::CiStatusMissing => "ci_status_missing",
            Self::SessionFollowStateMissing => "session_follow_state_missing",
            Self::HandoffTargetMissing => "handoff_target_missing",
            Self::NotificationCategoryMissing => "notification_category_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
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

/// Reads and validates the checked-in stable M5 companion matrix export.
pub fn current_stable_m5_companion_component_matrix_export(
) -> Result<M5CompanionComponentMatrixPacket, M5CompanionComponentMatrixArtifactError> {
    let packet: M5CompanionComponentMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-companion-component-proof/support_export.json"
    )))
    .map_err(M5CompanionComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CompanionComponentMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CompanionComponentMatrixPacket,
    violations: &mut Vec<M5CompanionComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_COMPANION_COMPONENT_SCHEMA_REF,
        M5_COMPANION_COMPONENT_DOC_REF,
        M5_COMPANION_NOTIFICATION_ROW_SCHEMA_REF,
        M5_MOBILE_REVIEW_CARD_SCHEMA_REF,
        M5_CI_STATUS_CARD_SCHEMA_REF,
        M5_SESSION_FOLLOW_TILE_SCHEMA_REF,
        M5_INCIDENT_SNAPSHOT_CARD_SCHEMA_REF,
        M5_DESKTOP_HANDOFF_SHEET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CompanionComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CompanionComponentMatrixPacket,
    violations: &mut Vec<M5CompanionComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CompanionComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5CompanionComponentMatrixPacket,
    violations: &mut Vec<M5CompanionComponentMatrixViolation>,
) {
    let present: BTreeSet<M5CompanionComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5CompanionComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5CompanionComponentMatrixViolation::RequiredComponentMissing);
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
            violations.push(M5CompanionComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5CompanionComponentMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_component_schema_ref())
        {
            violations.push(M5CompanionComponentMatrixViolation::ComponentSchemaRefMissing);
        }
        if row.object_kinds.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::ObjectKindMissing);
        }
        if row.client_scopes.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::ClientScopeMissing);
        }
        if row.freshness_classes.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::FreshnessClassMissing);
        }
        if row.dispositions.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::DispositionMissing);
        }
        if family.declares_severity() && row.severities.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::SeverityMissing);
        }
        if family.declares_review_kind() && row.review_kinds.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::ReviewKindMissing);
        }
        if family.declares_ci_status() && row.ci_statuses.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::CiStatusMissing);
        }
        if family.declares_session_follow_state() && row.session_follow_states.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::SessionFollowStateMissing);
        }
        if family.declares_handoff_target() && row.handoff_targets.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::HandoffTargetMissing);
        }
        if family.declares_notification_category() && row.notification_categories.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::NotificationCategoryMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5CompanionComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5CompanionComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5CompanionComponentMatrixPacket,
    violations: &mut Vec<M5CompanionComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.notification_row_shows_object_and_severity,
        review.mobile_review_card_shows_capability_boundary,
        review.ci_status_card_shows_status_and_freshness,
        review.session_follow_tile_shows_scope_and_freshness,
        review.incident_snapshot_card_shows_severity_and_freshness,
        review.desktop_handoff_sheet_shows_exact_target,
        review.no_surface_invents_alternate_state_label,
        review.object_identity_always_explicit,
        review.client_scope_named_once,
        review.freshness_always_explicit,
        review.capability_boundary_always_explicit,
        review.severity_always_explicit,
        review.exact_handoff_target_always_explicit,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5CompanionComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CompanionComponentMatrixPacket,
    violations: &mut Vec<M5CompanionComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.notification_surfaces_consume_severity_vocabulary,
        projection.review_surfaces_consume_capability_vocabulary,
        projection.ci_surfaces_consume_status_vocabulary,
        projection.follow_surfaces_consume_freshness_vocabulary,
        projection.handoff_surfaces_consume_target_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5CompanionComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CompanionComponentMatrixPacket,
    violations: &mut Vec<M5CompanionComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CompanionComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CompanionComponentMatrixPacket,
    violations: &mut Vec<M5CompanionComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.companion_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CompanionComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses companion words; what is rejected is a raw secret *value*
/// shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
