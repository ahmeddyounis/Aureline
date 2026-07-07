//! Two reusable M5 provider primitives — the offline-capture row and the
//! privacy/redaction row — so a user can tell, from the row alone, *where* a prepared handoff
//! packet will land, *how much* remains queued locally, and *what* support and export will
//! disclose, even when live provider access narrows or disappears.
//!
//! Aureline's frozen provider-account / mapping / offline-capture component matrix
//! ([`crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix`])
//! names the offline-capture row and the privacy/redaction row as two governed component
//! families and freezes their controlled vocabulary — the offline-capture states, the
//! queued-draft states, the redaction classes, and the export-boundary classes, plus the
//! surface families, the deployment lines, the consumer surfaces, the accessibility routes,
//! the qualification classes, and the downgrade triggers. This module *implements* that
//! contract as two reusable resolvers so a prepared handoff never vanishes when connectivity
//! drops, and a metadata-safe export/support default stays explicit before anything leaves the
//! local device.
//!
//! The module has two resolvers, one per family:
//!
//! 1. [`resolve_offline_capture_row`] — takes one captured change's offline-capture state, its
//!    capture kind (a bug report, a task update, or a blocked-work note), its packet
//!    destination class, its queued-draft state, its declared redaction default, and its
//!    queued-draft count, and produces one [`M5ResolvedOfflineCaptureRow`] carrying the derived
//!    row posture (one per capture state), the derived publish-later behavior, whether the
//!    packet destination is explicit or still unrouted, whether local drafts remain queued, and
//!    the bounded reveal / export / clear / retry / defer actions. It never assumes a default
//!    publish destination silently, never erases prepared handoff state, and — above all —
//!    never hides what remains queued locally.
//! 2. [`resolve_privacy_redaction_row`] — takes one provider-linked object's redaction class,
//!    its export boundary, its policy source, and its telemetry/event limit, and produces one
//!    [`M5ResolvedPrivacyRedactionRow`] carrying the derived row posture (one per redaction
//!    class), the derived support-bundle treatment, the exact field classes that are copied /
//!    exported versus withheld, and the bounded reveal / view-policy / adjust / export /
//!    escalation actions. It never hides its export or redaction boundary, never leaks
//!    credentials or endpoints, and always keeps the metadata-safe default explicit before an
//!    export or escalation leaves the device.
//!
//! A single parity matrix — [`M5ProviderOfflinePrivacyRowPacket`] — binds one row per claimed
//! M5 provider surface consumer (the offline-capture panel, the privacy/redaction panel, the
//! provider status bar, the headless/CLI capture surface, and the support privacy export) to
//! the shared offline-capture-row and privacy-row anatomy, the same capture states, publish
//! behaviors, redaction classes, export boundaries, bounded actions, export fields, and
//! non-visual accessibility routes, so the destination, queue, and boundary vocabulary stays
//! identical across desktop, headless/export, and support consumers.
//!
//! The offline-capture state ([`M5OfflineCaptureState`]), queued-draft state
//! ([`M5QueuedDraftState`]), redaction class ([`M5ProviderRedactionClass`]), export-boundary
//! class ([`M5ExportBoundaryClass`]), surface family ([`M5ProviderSurfaceFamily`]), deployment
//! line ([`M5ProviderDeploymentLine`]), consumer surface ([`M5ProviderConsumerSurface`]),
//! accessibility route ([`M5ProviderAccessibilityRoute`]), qualification class
//! ([`M5ProviderQualificationClass`]), and downgrade trigger ([`M5ProviderDowngradeTrigger`])
//! are reused verbatim from the frozen matrix. This module mints new vocabulary only for what
//! that matrix left implicit about the two rows themselves: the two derived row postures, the
//! publish-later behavior, the packet-destination class, the redaction policy source, the
//! telemetry limit, the support-bundle treatment, the privacy field classes, their bounded
//! actions, their anatomy parts, and their export fields. No M5 provider surface invents a
//! second offline-capture or privacy grammar.
//!
//! Raw comment bodies, pasted paths, credentials, and private endpoints stay outside the
//! export boundary; every packet destination, policy label, and capture/redaction identity is
//! carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_provider_offline_privacy_row_offline_capture_beta_narrowed,
    seeded_m5_provider_offline_privacy_row_packet,
    seeded_m5_provider_offline_privacy_row_privacy_redaction_preview_narrowed,
    M5_PROVIDER_OFFLINE_PRIVACY_ROW_PACKET_ID,
};

// The offline-capture state, queued-draft state, redaction class, export-boundary class,
// surface family, deployment line, consumer surface, accessibility route, qualification class,
// and downgrade triggers are frozen once, in the provider-account / offline-capture component
// matrix. This primitive reuses them verbatim so it never invents a parallel offline-capture
// or privacy vocabulary.
pub use crate::freeze_the_m5_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_component_matrix::{
    M5ExportBoundaryClass, M5OfflineCaptureState, M5ProviderAccessibilityRoute,
    M5ProviderConsumerSurface, M5ProviderDeploymentLine, M5ProviderDowngradeTrigger,
    M5ProviderQualificationClass, M5ProviderRedactionClass, M5ProviderSurfaceFamily,
    M5QueuedDraftState,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ProviderOfflinePrivacyRowPacket`].
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_RECORD_KIND: &str =
    "implement_m5_offline_capture_rows_and_privacy_redaction_rows_with_packet_destination_queued_draft_count_export_clear_actions_and_metadata_safe_boundary_truth_across_claimed_m5_provider_workflows";

/// Schema version for M5 provider offline-capture / privacy-redaction-row records.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the offline-capture / privacy-redaction-row boundary schema.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-provider-offline-capture-privacy-redaction-row.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_DOC_REF: &str =
    "docs/providers/m5_provider_offline_capture_privacy_redaction_row_primitive.md";

/// Repo-relative path of the frozen provider-account / offline-capture component matrix this
/// primitive narrows from.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-provider-account-offline-capture-component-matrix.schema.json";

/// Repo-relative path of the offline-handoff-packet contract this primitive binds its
/// packet-destination / queued-draft truth against.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_OFFLINE_HANDOFF_REF: &str =
    "schemas/providers/offline_handoff_packet.schema.json";

/// Repo-relative path of the export-redaction-profile contract this primitive binds its
/// redaction / export-boundary truth against.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_EXPORT_REDACTION_REF: &str =
    "schemas/support/export_redaction_profile.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-provider-offline-capture-privacy-redaction-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-provider-offline-capture-privacy-redaction-row-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_CSV_REF: &str =
    "artifacts/release/m5-provider-offline-capture-privacy-redaction-row-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PROVIDER_OFFLINE_PRIVACY_ROW_REPORT_REF: &str =
    "artifacts/design/m5-provider-offline-capture-privacy-redaction-row-primitive.md";

/// One claimed M5 provider-surface consumer that renders the shared offline-capture and
/// privacy/redaction rows. These are the consumers the acceptance criteria name — the
/// offline-capture panel, the privacy/redaction panel, the provider status bar, the
/// headless/CLI capture surface, and the support privacy export — so the same capture and
/// redaction grammar works across every claimed provider surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflinePrivacyConsumerSurface {
    /// The offline-capture panel surface.
    OfflineCapturePanel,
    /// The privacy/redaction panel surface.
    PrivacyRedactionPanel,
    /// The provider status-bar surface.
    ProviderStatusBar,
    /// The headless / CLI capture surface.
    HeadlessCliCapture,
    /// The support privacy-export surface.
    SupportPrivacyExport,
}

impl M5OfflinePrivacyConsumerSurface {
    /// Every claimed provider-surface consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OfflineCapturePanel,
        Self::PrivacyRedactionPanel,
        Self::ProviderStatusBar,
        Self::HeadlessCliCapture,
        Self::SupportPrivacyExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfflineCapturePanel => "offline_capture_panel",
            Self::PrivacyRedactionPanel => "privacy_redaction_panel",
            Self::ProviderStatusBar => "provider_status_bar",
            Self::HeadlessCliCapture => "headless_cli_capture",
            Self::SupportPrivacyExport => "support_privacy_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OfflineCapturePanel => "Offline-Capture Panel",
            Self::PrivacyRedactionPanel => "Privacy / Redaction Panel",
            Self::ProviderStatusBar => "Provider Status Bar",
            Self::HeadlessCliCapture => "Headless / CLI Capture",
            Self::SupportPrivacyExport => "Support Privacy Export",
        }
    }
}

// ---- offline-capture row vocabulary --------------------------------------

/// The kind of prepared work a captured packet holds, so an offline-capture row never blurs a
/// bug report, a task update, and a blocked-work note into one anonymous queued item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflineCaptureKind {
    /// A captured bug report.
    BugReport,
    /// A captured task update.
    TaskUpdate,
    /// A captured blocked-work note.
    BlockedWorkNote,
}

impl M5OfflineCaptureKind {
    /// Every capture kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::BugReport, Self::TaskUpdate, Self::BlockedWorkNote];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BugReport => "bug_report",
            Self::TaskUpdate => "task_update",
            Self::BlockedWorkNote => "blocked_work_note",
        }
    }
}

/// Where a captured packet will land, so an offline-capture row always names its destination
/// and never resolves to a silent default. An unrouted packet is flagged unrouted rather than
/// assumed to publish somewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflinePacketDestinationClass {
    /// The packet is routed to a mapped provider target and will publish there when reachable.
    RoutedToProvider,
    /// The packet stays a local export bundle only and never auto-publishes.
    LocalBundleOnly,
    /// The packet has no destination yet and must be routed before it can publish.
    UnroutedPending,
}

impl M5OfflinePacketDestinationClass {
    /// Every packet-destination class, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RoutedToProvider,
        Self::LocalBundleOnly,
        Self::UnroutedPending,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoutedToProvider => "routed_to_provider",
            Self::LocalBundleOnly => "local_bundle_only",
            Self::UnroutedPending => "unrouted_pending",
        }
    }

    /// True when the row points at an explicit destination rather than flagging itself
    /// unrouted.
    pub const fn shows_explicit_destination(self) -> bool {
        !matches!(self, Self::UnroutedPending)
    }
}

/// The derived posture of an offline-capture row — the resolver's verdict about the captured
/// packet. Derived one-to-one from the frozen offline-capture state so the six governed states
/// are never collapsed into one generic "queued" chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflineCaptureRowPosture {
    /// Captured and held locally.
    CapturedLocallyRow,
    /// Queued for publish when reachable.
    QueuedForPublishRow,
    /// Publish deferred by the user.
    PublishDeferredRow,
    /// Held because of a conflict.
    ConflictHeldRow,
    /// Pending discard.
    DiscardPendingRow,
    /// Synced and cleared from the queue.
    SyncedClearedRow,
}

impl M5OfflineCaptureRowPosture {
    /// Every offline-capture-row posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CapturedLocallyRow,
        Self::QueuedForPublishRow,
        Self::PublishDeferredRow,
        Self::ConflictHeldRow,
        Self::DiscardPendingRow,
        Self::SyncedClearedRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CapturedLocallyRow => "captured_locally_row",
            Self::QueuedForPublishRow => "queued_for_publish_row",
            Self::PublishDeferredRow => "publish_deferred_row",
            Self::ConflictHeldRow => "conflict_held_row",
            Self::DiscardPendingRow => "discard_pending_row",
            Self::SyncedClearedRow => "synced_cleared_row",
        }
    }

    /// True when the captured packet still holds prepared handoff state (anything but a cleared,
    /// already-synced row).
    pub const fn holds_prepared_handoff(self) -> bool {
        !matches!(self, Self::SyncedClearedRow)
    }
}

/// What a captured packet will do when connectivity returns, so an offline-capture row states
/// its publish-later behavior instead of leaving the user to guess.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PublishLaterBehavior {
    /// Held locally until a publish is initiated.
    HeldLocallyUntilPublish,
    /// Will publish automatically when the provider is reachable.
    PublishesWhenReachable,
    /// Held by the user's explicit choice.
    HeldByUserChoice,
    /// Held pending resolution of a conflict.
    HeldPendingConflict,
    /// Will discard on confirmation.
    WillDiscardOnConfirm,
    /// Already published and reconciled.
    AlreadyPublished,
}

impl M5PublishLaterBehavior {
    /// Every publish-later behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HeldLocallyUntilPublish,
        Self::PublishesWhenReachable,
        Self::HeldByUserChoice,
        Self::HeldPendingConflict,
        Self::WillDiscardOnConfirm,
        Self::AlreadyPublished,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HeldLocallyUntilPublish => "held_locally_until_publish",
            Self::PublishesWhenReachable => "publishes_when_reachable",
            Self::HeldByUserChoice => "held_by_user_choice",
            Self::HeldPendingConflict => "held_pending_conflict",
            Self::WillDiscardOnConfirm => "will_discard_on_confirm",
            Self::AlreadyPublished => "already_published",
        }
    }
}

/// One bounded action an offline-capture row offers, so a row never hides its reveal / export /
/// clear / retry / defer affordances and a user can preserve, publish, or clear queued work
/// without leaving the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflineCaptureRowAction {
    /// Reveal the capture state, destination, queued count, and redaction default.
    RevealCapture,
    /// Defer the publish of a queued packet.
    DeferPublish,
    /// Retry a blocked or failed publish.
    RetryPublish,
    /// Clear the captured packet from the local queue.
    ClearCapture,
    /// Export the captured packet as a local bundle.
    ExportPacket,
}

impl M5OfflineCaptureRowAction {
    /// Every offline-capture-row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealCapture,
        Self::DeferPublish,
        Self::RetryPublish,
        Self::ClearCapture,
        Self::ExportPacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealCapture => "reveal_capture",
            Self::DeferPublish => "defer_publish",
            Self::RetryPublish => "retry_publish",
            Self::ClearCapture => "clear_capture",
            Self::ExportPacket => "export_packet",
        }
    }
}

/// Controlled offline-capture-row anatomy part the shared row surfaces. The parts in
/// [`M5OfflineCaptureRowAnatomyPart::MANDATORY`] are required on every row so the packet
/// destination, queued-draft count, capture state, redaction default, and capture action cue
/// are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflineCaptureRowAnatomyPart {
    /// The packet-destination cue.
    PacketDestinationCue,
    /// The queued-draft-count cue.
    QueuedDraftCountCue,
    /// The capture-state cue.
    CaptureStateCue,
    /// The redaction-default cue.
    RedactionDefaultCue,
    /// The publish-later-behavior cue.
    PublishLaterCue,
    /// The capture-kind cue.
    CaptureKindCue,
    /// The export / clear action cue.
    CaptureActionCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5OfflineCaptureRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PacketDestinationCue,
        Self::QueuedDraftCountCue,
        Self::CaptureStateCue,
        Self::RedactionDefaultCue,
        Self::PublishLaterCue,
        Self::CaptureKindCue,
        Self::CaptureActionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every offline-capture row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::PacketDestinationCue,
        Self::QueuedDraftCountCue,
        Self::CaptureStateCue,
        Self::RedactionDefaultCue,
        Self::CaptureActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PacketDestinationCue => "packet_destination_cue",
            Self::QueuedDraftCountCue => "queued_draft_count_cue",
            Self::CaptureStateCue => "capture_state_cue",
            Self::RedactionDefaultCue => "redaction_default_cue",
            Self::PublishLaterCue => "publish_later_cue",
            Self::CaptureKindCue => "capture_kind_cue",
            Self::CaptureActionCue => "capture_action_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the offline-capture-row export carries so offline-capture-row truth is
/// reconstructable. The fields in [`M5OfflineCaptureRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OfflineCaptureRowExportField {
    /// The offline-capture state.
    CaptureState,
    /// The capture kind.
    CaptureKind,
    /// The packet destination.
    PacketDestination,
    /// The queued-draft count.
    QueuedDraftCount,
    /// The redaction default.
    RedactionDefault,
    /// The publish-later behavior.
    PublishLaterBehavior,
    /// The derived offline-capture-row posture.
    RowPosture,
    /// The bounded available actions.
    AvailableActions,
}

impl M5OfflineCaptureRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CaptureState,
        Self::CaptureKind,
        Self::PacketDestination,
        Self::QueuedDraftCount,
        Self::RedactionDefault,
        Self::PublishLaterBehavior,
        Self::RowPosture,
        Self::AvailableActions,
    ];

    /// The export fields every offline-capture row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::CaptureState,
        Self::PacketDestination,
        Self::QueuedDraftCount,
        Self::RedactionDefault,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CaptureState => "capture_state",
            Self::CaptureKind => "capture_kind",
            Self::PacketDestination => "packet_destination",
            Self::QueuedDraftCount => "queued_draft_count",
            Self::RedactionDefault => "redaction_default",
            Self::PublishLaterBehavior => "publish_later_behavior",
            Self::RowPosture => "row_posture",
            Self::AvailableActions => "available_actions",
        }
    }
}

// ---- privacy/redaction row vocabulary ------------------------------------

/// The derived posture of a privacy/redaction row — the resolver's verdict about how much of a
/// provider-linked object the row will disclose. Derived one-to-one from the frozen redaction
/// class so a full-body-visible row never reads the same as a metadata-only, a redacted, a
/// policy-restricted, a raw-withheld, or a no-export row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrivacyRedactionRowPosture {
    /// Full body is visible.
    FullBodyVisibleRow,
    /// Metadata only.
    MetadataOnlyRow,
    /// A redacted share.
    RedactedShareRow,
    /// Restricted by policy.
    PolicyRestrictedRow,
    /// Raw bodies withheld.
    RawWithheldRow,
    /// No export at all.
    NoExportRow,
}

impl M5PrivacyRedactionRowPosture {
    /// Every privacy/redaction-row posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullBodyVisibleRow,
        Self::MetadataOnlyRow,
        Self::RedactedShareRow,
        Self::PolicyRestrictedRow,
        Self::RawWithheldRow,
        Self::NoExportRow,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullBodyVisibleRow => "full_body_visible_row",
            Self::MetadataOnlyRow => "metadata_only_row",
            Self::RedactedShareRow => "redacted_share_row",
            Self::PolicyRestrictedRow => "policy_restricted_row",
            Self::RawWithheldRow => "raw_withheld_row",
            Self::NoExportRow => "no_export_row",
        }
    }

    /// True when a bundle may still be exported from this row (anything but a no-export row).
    pub const fn allows_export(self) -> bool {
        !matches!(self, Self::NoExportRow)
    }
}

/// Where a privacy/redaction row's boundary comes from, so a user always sees the policy source
/// and whether they may adjust it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RedactionPolicySource {
    /// The user's own default.
    UserDefault,
    /// A workspace-level policy.
    WorkspacePolicy,
    /// An organisation-level policy.
    OrgPolicy,
    /// A regulatory policy.
    RegulatoryPolicy,
    /// A provider-imposed policy.
    ProviderPolicy,
}

impl M5RedactionPolicySource {
    /// Every policy source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UserDefault,
        Self::WorkspacePolicy,
        Self::OrgPolicy,
        Self::RegulatoryPolicy,
        Self::ProviderPolicy,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserDefault => "user_default",
            Self::WorkspacePolicy => "workspace_policy",
            Self::OrgPolicy => "org_policy",
            Self::RegulatoryPolicy => "regulatory_policy",
            Self::ProviderPolicy => "provider_policy",
        }
    }

    /// True when the user may adjust the redaction locally (their own or a workspace default);
    /// org, regulatory, and provider policies are locked.
    pub const fn is_user_adjustable(self) -> bool {
        matches!(self, Self::UserDefault | Self::WorkspacePolicy)
    }
}

/// The telemetry / event limit a privacy/redaction row states, so a user knows what — if
/// anything — leaves the device as telemetry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TelemetryEventLimit {
    /// No events are ever exported.
    NoEventExport,
    /// Only anonymous metadata counters are exported.
    MetadataCountersOnly,
    /// A redacted event share is exported.
    RedactedEventShare,
    /// Events are suppressed by policy.
    EventsSuppressed,
}

impl M5TelemetryEventLimit {
    /// Every telemetry/event limit, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoEventExport,
        Self::MetadataCountersOnly,
        Self::RedactedEventShare,
        Self::EventsSuppressed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoEventExport => "no_event_export",
            Self::MetadataCountersOnly => "metadata_counters_only",
            Self::RedactedEventShare => "redacted_event_share",
            Self::EventsSuppressed => "events_suppressed",
        }
    }
}

/// How a provider-linked object is treated inside a support bundle, so a privacy/redaction row
/// states support-bundle treatment explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportBundleTreatment {
    /// The full body is included in the bundle.
    IncludesFullBody,
    /// Only metadata is included in the bundle.
    MetadataOnlyInBundle,
    /// The object is redacted in the bundle.
    RedactedInBundle,
    /// The object is excluded from the bundle.
    ExcludedFromBundle,
    /// The object is blocked from the bundle entirely.
    BlockedFromBundle,
}

impl M5SupportBundleTreatment {
    /// Every support-bundle treatment, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::IncludesFullBody,
        Self::MetadataOnlyInBundle,
        Self::RedactedInBundle,
        Self::ExcludedFromBundle,
        Self::BlockedFromBundle,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncludesFullBody => "includes_full_body",
            Self::MetadataOnlyInBundle => "metadata_only_in_bundle",
            Self::RedactedInBundle => "redacted_in_bundle",
            Self::ExcludedFromBundle => "excluded_from_bundle",
            Self::BlockedFromBundle => "blocked_from_bundle",
        }
    }
}

/// One class of field a provider-linked object carries, so a privacy/redaction row can state
/// exactly which fields are copied / exported and which are withheld. Endpoints and credentials
/// are never exported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrivacyFieldClass {
    /// The object's stable identity.
    ObjectIdentity,
    /// The object's title / summary.
    Title,
    /// The object's body text.
    BodyText,
    /// The object's attachments.
    Attachments,
    /// The author identity.
    AuthorIdentity,
    /// Private endpoints.
    Endpoints,
    /// Credentials.
    Credentials,
}

impl M5PrivacyFieldClass {
    /// Every privacy field class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ObjectIdentity,
        Self::Title,
        Self::BodyText,
        Self::Attachments,
        Self::AuthorIdentity,
        Self::Endpoints,
        Self::Credentials,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectIdentity => "object_identity",
            Self::Title => "title",
            Self::BodyText => "body_text",
            Self::Attachments => "attachments",
            Self::AuthorIdentity => "author_identity",
            Self::Endpoints => "endpoints",
            Self::Credentials => "credentials",
        }
    }

    /// True when this field class may never cross the export boundary, whatever the redaction
    /// class.
    pub const fn is_never_exportable(self) -> bool {
        matches!(self, Self::Endpoints | Self::Credentials)
    }
}

/// One bounded action a privacy/redaction row offers, so a row never hides its reveal /
/// view-policy / adjust / export / escalation affordances and a user can inspect the boundary
/// or request a reviewed escalation without leaving the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrivacyRedactionRowAction {
    /// Reveal the redaction class, export boundary, and copied/exported fields.
    RevealRedaction,
    /// View the policy source behind the redaction.
    ViewPolicySource,
    /// Adjust the redaction locally (when the policy is user-adjustable).
    AdjustRedaction,
    /// Export a redacted bundle.
    ExportRedactedBundle,
    /// Request a reviewed escalation to widen disclosure.
    RequestEscalationReview,
}

impl M5PrivacyRedactionRowAction {
    /// Every privacy/redaction-row action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::RevealRedaction,
        Self::ViewPolicySource,
        Self::AdjustRedaction,
        Self::ExportRedactedBundle,
        Self::RequestEscalationReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RevealRedaction => "reveal_redaction",
            Self::ViewPolicySource => "view_policy_source",
            Self::AdjustRedaction => "adjust_redaction",
            Self::ExportRedactedBundle => "export_redacted_bundle",
            Self::RequestEscalationReview => "request_escalation_review",
        }
    }
}

/// Controlled privacy/redaction-row anatomy part the shared row surfaces. The parts in
/// [`M5PrivacyRedactionRowAnatomyPart::MANDATORY`] are required on every row so the redaction
/// class, export boundary, copied/exported fields, policy source, and escalation action cue are
/// never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrivacyRedactionRowAnatomyPart {
    /// The redaction-class cue.
    RedactionClassCue,
    /// The export-boundary cue.
    ExportBoundaryCue,
    /// The copied / exported fields cue.
    CopiedExportedFieldsCue,
    /// The support-bundle-treatment cue.
    SupportBundleTreatmentCue,
    /// The telemetry / event-limit cue.
    TelemetryLimitCue,
    /// The policy-source cue.
    PolicySourceCue,
    /// The reviewed-escalation action cue.
    EscalationActionCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5PrivacyRedactionRowAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RedactionClassCue,
        Self::ExportBoundaryCue,
        Self::CopiedExportedFieldsCue,
        Self::SupportBundleTreatmentCue,
        Self::TelemetryLimitCue,
        Self::PolicySourceCue,
        Self::EscalationActionCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every privacy/redaction row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::RedactionClassCue,
        Self::ExportBoundaryCue,
        Self::CopiedExportedFieldsCue,
        Self::PolicySourceCue,
        Self::EscalationActionCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactionClassCue => "redaction_class_cue",
            Self::ExportBoundaryCue => "export_boundary_cue",
            Self::CopiedExportedFieldsCue => "copied_exported_fields_cue",
            Self::SupportBundleTreatmentCue => "support_bundle_treatment_cue",
            Self::TelemetryLimitCue => "telemetry_limit_cue",
            Self::PolicySourceCue => "policy_source_cue",
            Self::EscalationActionCue => "escalation_action_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the privacy/redaction-row export carries so privacy-row truth is reconstructable.
/// The fields in [`M5PrivacyRedactionRowExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PrivacyRedactionRowExportField {
    /// The redaction class.
    RedactionClass,
    /// The export boundary.
    ExportBoundary,
    /// The exported field classes.
    ExportedFieldClasses,
    /// The withheld field classes.
    WithheldFieldClasses,
    /// The support-bundle treatment.
    SupportBundleTreatment,
    /// The telemetry / event limit.
    TelemetryLimit,
    /// The policy source.
    PolicySource,
    /// The bounded available actions.
    AvailableActions,
}

impl M5PrivacyRedactionRowExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::RedactionClass,
        Self::ExportBoundary,
        Self::ExportedFieldClasses,
        Self::WithheldFieldClasses,
        Self::SupportBundleTreatment,
        Self::TelemetryLimit,
        Self::PolicySource,
        Self::AvailableActions,
    ];

    /// The export fields every privacy/redaction row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::RedactionClass,
        Self::ExportBoundary,
        Self::ExportedFieldClasses,
        Self::PolicySource,
        Self::AvailableActions,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactionClass => "redaction_class",
            Self::ExportBoundary => "export_boundary",
            Self::ExportedFieldClasses => "exported_field_classes",
            Self::WithheldFieldClasses => "withheld_field_classes",
            Self::SupportBundleTreatment => "support_bundle_treatment",
            Self::TelemetryLimit => "telemetry_limit",
            Self::PolicySource => "policy_source",
            Self::AvailableActions => "available_actions",
        }
    }
}

// ---- offline-capture row resolver ----------------------------------------

/// The full input to the offline-capture-row resolver for one captured packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OfflineCaptureRowResolutionInput {
    /// The offline-capture state.
    pub capture_state: M5OfflineCaptureState,
    /// The capture kind.
    pub capture_kind: M5OfflineCaptureKind,
    /// The packet-destination class.
    pub destination_class: M5OfflinePacketDestinationClass,
    /// The queued-draft state.
    pub queued_draft_state: M5QueuedDraftState,
    /// The declared redaction default for the packet.
    pub redaction_default: M5ProviderRedactionClass,
    /// The number of drafts queued behind this row.
    pub queued_draft_count: u32,
    /// The opaque packet-destination label (must be non-empty).
    pub packet_destination_label: String,
    /// The opaque user-facing capture label (must be non-empty).
    pub capture_label: String,
    /// The opaque stable capture identity (must be non-empty).
    pub capture_ref: String,
}

/// The resolved offline-capture-row truth for one captured packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOfflineCaptureRow {
    /// The offline-capture state.
    pub capture_state: M5OfflineCaptureState,
    /// The capture kind.
    pub capture_kind: M5OfflineCaptureKind,
    /// The packet-destination class.
    pub destination_class: M5OfflinePacketDestinationClass,
    /// The queued-draft state.
    pub queued_draft_state: M5QueuedDraftState,
    /// The declared redaction default.
    pub redaction_default: M5ProviderRedactionClass,
    /// The number of drafts queued behind this row.
    pub queued_draft_count: u32,
    /// The opaque packet-destination label, preserved exactly from the input.
    pub packet_destination_label: String,
    /// The opaque capture label, preserved exactly from the input.
    pub capture_label: String,
    /// The opaque stable capture identity, preserved exactly from the input.
    pub capture_ref: String,
    /// The derived offline-capture-row posture.
    pub row_posture: M5OfflineCaptureRowPosture,
    /// The derived publish-later behavior.
    pub publish_later_behavior: M5PublishLaterBehavior,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5OfflineCaptureRowAction>,
    /// True when the row points at an explicit destination rather than flagging itself
    /// unrouted.
    pub shows_packet_destination: bool,
    /// True when local drafts remain queued behind this row.
    pub has_queued_drafts: bool,
    /// The offline-capture row always retains prepared handoff state. ALWAYS `true`.
    pub retains_prepared_handoff: bool,
    /// The offline-capture row never hides what remains queued locally. ALWAYS `false`.
    pub hides_queued_local_work: bool,
    /// The offline-capture row never assumes a default publish destination silently. ALWAYS
    /// `false`.
    pub assumes_default_destination_silently: bool,
}

/// Errors returned by [`resolve_offline_capture_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5OfflineCaptureRowResolutionError {
    /// The packet-destination label was empty.
    EmptyDestinationLabel,
    /// The capture label was empty.
    EmptyCaptureLabel,
    /// The capture ref was empty.
    EmptyCaptureRef,
    /// A cleared, already-synced capture still reported queued drafts.
    ClearedCaptureHasQueuedDrafts,
    /// A capture descriptor carried forbidden material.
    ForbiddenCaptureMaterial,
}

impl M5OfflineCaptureRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDestinationLabel => "empty_destination_label",
            Self::EmptyCaptureLabel => "empty_capture_label",
            Self::EmptyCaptureRef => "empty_capture_ref",
            Self::ClearedCaptureHasQueuedDrafts => "cleared_capture_has_queued_drafts",
            Self::ForbiddenCaptureMaterial => "forbidden_capture_material",
        }
    }
}

impl fmt::Display for M5OfflineCaptureRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "offline capture row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5OfflineCaptureRowResolutionError {}

/// Resolves one offline-capture row from its declared capture state.
///
/// The derived row posture is taken one-to-one from the frozen offline-capture state so the six
/// governed states never collapse into one generic "queued" chip; the derived publish-later
/// behavior states what the packet does when connectivity returns. The row always reveals its
/// capture and offers export; it offers clear whenever there is still something to clear, retry
/// when a publish is blocked or failed, and defer when a publish is queued. An unrouted packet
/// is flagged unrouted rather than resolving to a silent default destination, and the queued
/// count is always carried.
pub fn resolve_offline_capture_row(
    input: &M5OfflineCaptureRowResolutionInput,
) -> Result<M5ResolvedOfflineCaptureRow, M5OfflineCaptureRowResolutionError> {
    if input.packet_destination_label.trim().is_empty() {
        return Err(M5OfflineCaptureRowResolutionError::EmptyDestinationLabel);
    }
    if input.capture_label.trim().is_empty() {
        return Err(M5OfflineCaptureRowResolutionError::EmptyCaptureLabel);
    }
    if input.capture_ref.trim().is_empty() {
        return Err(M5OfflineCaptureRowResolutionError::EmptyCaptureRef);
    }
    if matches!(input.capture_state, M5OfflineCaptureState::SyncedCleared)
        && input.queued_draft_count > 0
    {
        return Err(M5OfflineCaptureRowResolutionError::ClearedCaptureHasQueuedDrafts);
    }
    if value_repr_is_forbidden(&input.packet_destination_label)
        || value_repr_is_forbidden(&input.capture_label)
        || value_repr_is_forbidden(&input.capture_ref)
    {
        return Err(M5OfflineCaptureRowResolutionError::ForbiddenCaptureMaterial);
    }

    let row_posture = derive_capture_posture(input.capture_state);
    let publish_later_behavior = derive_publish_later_behavior(input.capture_state);
    let available_actions = derive_capture_actions(input.capture_state, input.queued_draft_state);

    Ok(M5ResolvedOfflineCaptureRow {
        capture_state: input.capture_state,
        capture_kind: input.capture_kind,
        destination_class: input.destination_class,
        queued_draft_state: input.queued_draft_state,
        redaction_default: input.redaction_default,
        queued_draft_count: input.queued_draft_count,
        packet_destination_label: input.packet_destination_label.clone(),
        capture_label: input.capture_label.clone(),
        capture_ref: input.capture_ref.clone(),
        row_posture,
        publish_later_behavior,
        available_actions,
        shows_packet_destination: input.destination_class.shows_explicit_destination(),
        has_queued_drafts: input.queued_draft_count > 0,
        // The acceptance criterion: connectivity loss never erases prepared handoff state and
        // never hides what remains queued locally.
        retains_prepared_handoff: true,
        hides_queued_local_work: false,
        assumes_default_destination_silently: false,
    })
}

/// Derives the offline-capture-row posture one-to-one from the frozen offline-capture state.
fn derive_capture_posture(state: M5OfflineCaptureState) -> M5OfflineCaptureRowPosture {
    use M5OfflineCaptureRowPosture as Posture;
    use M5OfflineCaptureState as State;
    match state {
        State::CapturedLocal => Posture::CapturedLocallyRow,
        State::QueuedForPublish => Posture::QueuedForPublishRow,
        State::PublishDeferred => Posture::PublishDeferredRow,
        State::ConflictHeld => Posture::ConflictHeldRow,
        State::DiscardPending => Posture::DiscardPendingRow,
        State::SyncedCleared => Posture::SyncedClearedRow,
    }
}

/// Derives the publish-later behavior from the frozen offline-capture state.
fn derive_publish_later_behavior(state: M5OfflineCaptureState) -> M5PublishLaterBehavior {
    use M5OfflineCaptureState as State;
    use M5PublishLaterBehavior as Behavior;
    match state {
        State::CapturedLocal => Behavior::HeldLocallyUntilPublish,
        State::QueuedForPublish => Behavior::PublishesWhenReachable,
        State::PublishDeferred => Behavior::HeldByUserChoice,
        State::ConflictHeld => Behavior::HeldPendingConflict,
        State::DiscardPending => Behavior::WillDiscardOnConfirm,
        State::SyncedCleared => Behavior::AlreadyPublished,
    }
}

/// Derives the bounded offline-capture action set from the capture and queued-draft states.
///
/// Reveal and export are always offered; defer is offered when the packet is queued for
/// publish; retry is offered when a publish is blocked or failed; clear is offered whenever the
/// capture is not already synced and cleared.
fn derive_capture_actions(
    capture_state: M5OfflineCaptureState,
    queued_draft_state: M5QueuedDraftState,
) -> Vec<M5OfflineCaptureRowAction> {
    use M5OfflineCaptureRowAction as Action;
    let mut actions = vec![Action::RevealCapture];
    if matches!(capture_state, M5OfflineCaptureState::QueuedForPublish) {
        actions.push(Action::DeferPublish);
    }
    if matches!(
        queued_draft_state,
        M5QueuedDraftState::PublishBlocked | M5QueuedDraftState::PublishFailed
    ) {
        actions.push(Action::RetryPublish);
    }
    if !matches!(capture_state, M5OfflineCaptureState::SyncedCleared) {
        actions.push(Action::ClearCapture);
    }
    actions.push(Action::ExportPacket);
    actions
}

// ---- privacy/redaction row resolver --------------------------------------

/// The full input to the privacy/redaction-row resolver for one provider-linked object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrivacyRedactionRowResolutionInput {
    /// The redaction class.
    pub redaction_class: M5ProviderRedactionClass,
    /// The export boundary.
    pub export_boundary: M5ExportBoundaryClass,
    /// The policy source.
    pub policy_source: M5RedactionPolicySource,
    /// The telemetry / event limit.
    pub telemetry_limit: M5TelemetryEventLimit,
    /// The opaque user-facing policy label (must be non-empty).
    pub policy_label: String,
    /// The opaque stable redaction identity (must be non-empty).
    pub redaction_ref: String,
}

/// The resolved privacy/redaction-row truth for one provider-linked object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPrivacyRedactionRow {
    /// The redaction class.
    pub redaction_class: M5ProviderRedactionClass,
    /// The export boundary.
    pub export_boundary: M5ExportBoundaryClass,
    /// The policy source.
    pub policy_source: M5RedactionPolicySource,
    /// The telemetry / event limit.
    pub telemetry_limit: M5TelemetryEventLimit,
    /// The opaque policy label, preserved exactly from the input.
    pub policy_label: String,
    /// The opaque stable redaction identity, preserved exactly from the input.
    pub redaction_ref: String,
    /// The derived privacy/redaction-row posture.
    pub row_posture: M5PrivacyRedactionRowPosture,
    /// The derived support-bundle treatment.
    pub support_bundle_treatment: M5SupportBundleTreatment,
    /// The field classes copied / exported across the boundary.
    pub exported_field_classes: Vec<M5PrivacyFieldClass>,
    /// The field classes withheld from the boundary.
    pub withheld_field_classes: Vec<M5PrivacyFieldClass>,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5PrivacyRedactionRowAction>,
    /// True when a bundle may still be exported from this row.
    pub can_export: bool,
    /// The metadata-safe default stays explicit before an export or escalation leaves the
    /// device. ALWAYS `true`.
    pub metadata_safe_default_explicit: bool,
    /// The row always withholds credentials and endpoints from the boundary. ALWAYS `true`.
    pub withholds_credentials_and_endpoints: bool,
    /// A wider disclosure always requires a reviewed escalation. ALWAYS `true`.
    pub escalation_requires_review: bool,
    /// The row never hides its export or redaction boundary. ALWAYS `false`.
    pub hides_export_or_redaction_boundary: bool,
}

/// Errors returned by [`resolve_privacy_redaction_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5PrivacyRedactionRowResolutionError {
    /// The policy label was empty.
    EmptyPolicyLabel,
    /// The redaction ref was empty.
    EmptyRedactionRef,
    /// A redaction descriptor carried forbidden material.
    ForbiddenRedactionMaterial,
}

impl M5PrivacyRedactionRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyPolicyLabel => "empty_policy_label",
            Self::EmptyRedactionRef => "empty_redaction_ref",
            Self::ForbiddenRedactionMaterial => "forbidden_redaction_material",
        }
    }
}

impl fmt::Display for M5PrivacyRedactionRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "privacy redaction row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5PrivacyRedactionRowResolutionError {}

/// Resolves one privacy/redaction row from its declared redaction state.
///
/// The derived row posture is taken one-to-one from the frozen redaction class so a
/// full-body-visible row never reads the same as a metadata-only, redacted, policy-restricted,
/// raw-withheld, or no-export row; the derived support-bundle treatment states how the object
/// appears in a support bundle. The exported / withheld field classes state exactly what is
/// copied out — credentials and endpoints are always withheld, whatever the class. The row
/// always reveals its redaction, views its policy source, and requests a reviewed escalation;
/// it offers a redacted export unless nothing may be exported, and a local adjust unless the
/// policy is locked. The metadata-safe default stays explicit before anything leaves the
/// device.
pub fn resolve_privacy_redaction_row(
    input: &M5PrivacyRedactionRowResolutionInput,
) -> Result<M5ResolvedPrivacyRedactionRow, M5PrivacyRedactionRowResolutionError> {
    if input.policy_label.trim().is_empty() {
        return Err(M5PrivacyRedactionRowResolutionError::EmptyPolicyLabel);
    }
    if input.redaction_ref.trim().is_empty() {
        return Err(M5PrivacyRedactionRowResolutionError::EmptyRedactionRef);
    }
    if value_repr_is_forbidden(&input.policy_label) || value_repr_is_forbidden(&input.redaction_ref)
    {
        return Err(M5PrivacyRedactionRowResolutionError::ForbiddenRedactionMaterial);
    }

    let row_posture = derive_redaction_posture(input.redaction_class);
    let support_bundle_treatment = derive_support_bundle_treatment(input.redaction_class);
    let exported_field_classes = derive_exported_fields(input.redaction_class);
    let withheld_field_classes: Vec<M5PrivacyFieldClass> = M5PrivacyFieldClass::ALL
        .into_iter()
        .filter(|field| !exported_field_classes.contains(field))
        .collect();
    let available_actions = derive_redaction_actions(input.redaction_class, input.policy_source);
    let withholds_credentials_and_endpoints = !exported_field_classes
        .iter()
        .any(|field| field.is_never_exportable());

    Ok(M5ResolvedPrivacyRedactionRow {
        redaction_class: input.redaction_class,
        export_boundary: input.export_boundary,
        policy_source: input.policy_source,
        telemetry_limit: input.telemetry_limit,
        policy_label: input.policy_label.clone(),
        redaction_ref: input.redaction_ref.clone(),
        row_posture,
        support_bundle_treatment,
        exported_field_classes,
        withheld_field_classes,
        available_actions,
        can_export: row_posture.allows_export(),
        // The acceptance criterion: metadata-safe defaults stay explicit and credentials /
        // endpoints never cross the boundary; a wider disclosure is always reviewed.
        metadata_safe_default_explicit: true,
        withholds_credentials_and_endpoints,
        escalation_requires_review: true,
        hides_export_or_redaction_boundary: false,
    })
}

/// Derives the privacy/redaction-row posture one-to-one from the frozen redaction class.
fn derive_redaction_posture(class: M5ProviderRedactionClass) -> M5PrivacyRedactionRowPosture {
    use M5PrivacyRedactionRowPosture as Posture;
    use M5ProviderRedactionClass as Class;
    match class {
        Class::FullBodyVisible => Posture::FullBodyVisibleRow,
        Class::MetadataOnly => Posture::MetadataOnlyRow,
        Class::RedactedShare => Posture::RedactedShareRow,
        Class::PolicyRestricted => Posture::PolicyRestrictedRow,
        Class::RawWithheld => Posture::RawWithheldRow,
        Class::NoExport => Posture::NoExportRow,
    }
}

/// Derives the support-bundle treatment from the frozen redaction class.
fn derive_support_bundle_treatment(class: M5ProviderRedactionClass) -> M5SupportBundleTreatment {
    use M5ProviderRedactionClass as Class;
    use M5SupportBundleTreatment as Treatment;
    match class {
        Class::FullBodyVisible => Treatment::IncludesFullBody,
        Class::MetadataOnly => Treatment::MetadataOnlyInBundle,
        Class::RedactedShare => Treatment::RedactedInBundle,
        Class::PolicyRestricted => Treatment::RedactedInBundle,
        Class::RawWithheld => Treatment::ExcludedFromBundle,
        Class::NoExport => Treatment::BlockedFromBundle,
    }
}

/// Derives the field classes copied / exported across the boundary from the frozen redaction
/// class. Credentials and endpoints are never included, whatever the class.
fn derive_exported_fields(class: M5ProviderRedactionClass) -> Vec<M5PrivacyFieldClass> {
    use M5PrivacyFieldClass as Field;
    use M5ProviderRedactionClass as Class;
    match class {
        Class::FullBodyVisible => vec![
            Field::ObjectIdentity,
            Field::Title,
            Field::BodyText,
            Field::Attachments,
            Field::AuthorIdentity,
        ],
        Class::MetadataOnly => vec![Field::ObjectIdentity, Field::Title, Field::AuthorIdentity],
        Class::RedactedShare => vec![Field::ObjectIdentity, Field::Title],
        Class::PolicyRestricted => vec![Field::ObjectIdentity],
        Class::RawWithheld => vec![Field::ObjectIdentity, Field::Title, Field::AuthorIdentity],
        Class::NoExport => vec![],
    }
}

/// Derives the bounded privacy/redaction action set from the redaction class and policy source.
///
/// Reveal, view-policy, and escalation-review are always offered; a redacted export is offered
/// unless nothing may be exported; a local adjust is offered only when the policy is
/// user-adjustable and the class is not policy-restricted.
fn derive_redaction_actions(
    class: M5ProviderRedactionClass,
    policy_source: M5RedactionPolicySource,
) -> Vec<M5PrivacyRedactionRowAction> {
    use M5PrivacyRedactionRowAction as Action;
    let mut actions = vec![Action::RevealRedaction, Action::ViewPolicySource];
    if policy_source.is_user_adjustable()
        && !matches!(class, M5ProviderRedactionClass::PolicyRestricted)
    {
        actions.push(Action::AdjustRedaction);
    }
    if !matches!(class, M5ProviderRedactionClass::NoExport) {
        actions.push(Action::ExportRedactedBundle);
    }
    actions.push(Action::RequestEscalationReview);
    actions
}

// ---- worked cases -------------------------------------------------------

/// One worked offline-capture-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OfflineCaptureRowResolutionCase {
    /// The resolver input.
    pub input: M5OfflineCaptureRowResolutionInput,
    /// The resolved truth. Must equal `resolve_offline_capture_row(&input)`.
    pub resolved: M5ResolvedOfflineCaptureRow,
}

impl M5OfflineCaptureRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5OfflineCaptureRowResolutionInput) -> Self {
        let resolved =
            resolve_offline_capture_row(&input).expect("seed offline capture row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_offline_capture_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved capture identity preserves the input identity exactly.
    pub fn preserves_capture_identity(&self) -> bool {
        self.resolved.capture_ref == self.input.capture_ref
            && self.resolved.capture_label == self.input.capture_label
            && self.resolved.packet_destination_label == self.input.packet_destination_label
    }

    /// True when the case retains prepared handoff state and never hides queued local work.
    pub fn retains_handoff(&self) -> bool {
        self.resolved.retains_prepared_handoff
            && !self.resolved.hides_queued_local_work
            && !self.resolved.assumes_default_destination_silently
    }
}

/// One worked privacy/redaction-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PrivacyRedactionRowResolutionCase {
    /// The resolver input.
    pub input: M5PrivacyRedactionRowResolutionInput,
    /// The resolved truth. Must equal `resolve_privacy_redaction_row(&input)`.
    pub resolved: M5ResolvedPrivacyRedactionRow,
}

impl M5PrivacyRedactionRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5PrivacyRedactionRowResolutionInput) -> Self {
        let resolved = resolve_privacy_redaction_row(&input)
            .expect("seed privacy redaction row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_privacy_redaction_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved redaction identity preserves the input identity exactly.
    pub fn preserves_redaction_identity(&self) -> bool {
        self.resolved.redaction_ref == self.input.redaction_ref
            && self.resolved.policy_label == self.input.policy_label
    }

    /// True when the case keeps its export/redaction boundary, withholds credentials and
    /// endpoints, and keeps the metadata-safe default explicit.
    pub fn keeps_boundary_and_withholds(&self) -> bool {
        !self.resolved.hides_export_or_redaction_boundary
            && self.resolved.withholds_credentials_and_endpoints
            && self.resolved.metadata_safe_default_explicit
    }
}

/// One row in the primitive matrix: one provider-surface consumer bound to the shared
/// offline-capture-row and privacy-row anatomy, the capture states, kinds, destinations,
/// publish behaviors, redaction classes, export boundaries, policy sources, telemetry limits,
/// bounded actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OfflinePrivacyConsumerRow {
    /// Provider-surface consumer family.
    pub consumer_surface: M5OfflinePrivacyConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5ProviderQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 provider surface families that render / consume this row.
    pub surface_families: Vec<M5ProviderSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5ProviderDeploymentLine>,
    /// Offline-capture-row anatomy parts this row renders (must include the mandatory parts).
    pub offline_anatomy_parts: Vec<M5OfflineCaptureRowAnatomyPart>,
    /// Privacy-row anatomy parts this row renders (must include the mandatory parts).
    pub privacy_anatomy_parts: Vec<M5PrivacyRedactionRowAnatomyPart>,
    /// Offline-capture states this consumer distinguishes.
    pub capture_states: Vec<M5OfflineCaptureState>,
    /// Capture kinds this consumer distinguishes.
    pub capture_kinds: Vec<M5OfflineCaptureKind>,
    /// Packet-destination classes this consumer distinguishes.
    pub destination_classes: Vec<M5OfflinePacketDestinationClass>,
    /// Offline-capture-row postures this consumer distinguishes.
    pub capture_row_postures: Vec<M5OfflineCaptureRowPosture>,
    /// Publish-later behaviors this consumer distinguishes.
    pub publish_later_behaviors: Vec<M5PublishLaterBehavior>,
    /// Bounded offline-capture-row actions this consumer offers.
    pub offline_row_actions: Vec<M5OfflineCaptureRowAction>,
    /// Redaction classes this consumer distinguishes.
    pub redaction_classes: Vec<M5ProviderRedactionClass>,
    /// Export-boundary classes this consumer distinguishes.
    pub export_boundaries: Vec<M5ExportBoundaryClass>,
    /// Redaction policy sources this consumer distinguishes.
    pub policy_sources: Vec<M5RedactionPolicySource>,
    /// Telemetry / event limits this consumer distinguishes.
    pub telemetry_limits: Vec<M5TelemetryEventLimit>,
    /// Support-bundle treatments this consumer distinguishes.
    pub support_bundle_treatments: Vec<M5SupportBundleTreatment>,
    /// Privacy field classes this consumer distinguishes.
    pub privacy_field_classes: Vec<M5PrivacyFieldClass>,
    /// Privacy/redaction-row postures this consumer distinguishes.
    pub privacy_row_postures: Vec<M5PrivacyRedactionRowPosture>,
    /// Bounded privacy/redaction-row actions this consumer offers.
    pub privacy_row_actions: Vec<M5PrivacyRedactionRowAction>,
    /// Queued-draft states this consumer distinguishes.
    pub queued_draft_states: Vec<M5QueuedDraftState>,
    /// Offline-capture-row export fields this row carries (must include the mandatory fields).
    pub offline_export_fields: Vec<M5OfflineCaptureRowExportField>,
    /// Privacy/redaction-row export fields this row carries (must include the mandatory fields).
    pub privacy_export_fields: Vec<M5PrivacyRedactionRowExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ProviderAccessibilityRoute>,
    /// Provider subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ProviderConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ProviderDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked offline-capture-row resolutions proving the offline resolver on this consumer.
    pub offline_examples: Vec<M5OfflineCaptureRowResolutionCase>,
    /// Worked privacy/redaction-row resolutions proving the privacy resolver on this consumer.
    pub privacy_examples: Vec<M5PrivacyRedactionRowResolutionCase>,
    /// Hard invariant: this consumer never assumes a default publish destination silently. MUST
    /// be `false`.
    pub assumes_default_destination_silently: bool,
    /// Hard invariant: this consumer never hides what remains queued locally. MUST be `false`.
    pub hides_queued_local_work: bool,
    /// Hard invariant: this consumer never drops prepared handoff state. MUST be `false`.
    pub drops_prepared_handoff_state: bool,
    /// Hard invariant: this consumer never hides its export or redaction boundary. MUST be
    /// `false`.
    pub hides_export_or_redaction_boundary: bool,
    /// Hard invariant: this consumer never leaks credentials or endpoints. MUST be `false`.
    pub leaks_credentials_or_endpoints: bool,
}

impl M5OfflinePrivacyConsumerRow {
    /// True when the row declares every mandatory offline anatomy part.
    fn declares_mandatory_offline_anatomy(&self) -> bool {
        let present: BTreeSet<M5OfflineCaptureRowAnatomyPart> =
            self.offline_anatomy_parts.iter().copied().collect();
        M5OfflineCaptureRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory privacy anatomy part.
    fn declares_mandatory_privacy_anatomy(&self) -> bool {
        let present: BTreeSet<M5PrivacyRedactionRowAnatomyPart> =
            self.privacy_anatomy_parts.iter().copied().collect();
        M5PrivacyRedactionRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory offline export field.
    fn declares_mandatory_offline_export(&self) -> bool {
        let present: BTreeSet<M5OfflineCaptureRowExportField> =
            self.offline_export_fields.iter().copied().collect();
        M5OfflineCaptureRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory privacy export field.
    fn declares_mandatory_privacy_export(&self) -> bool {
        let present: BTreeSet<M5PrivacyRedactionRowExportField> =
            self.privacy_export_fields.iter().copied().collect();
        M5PrivacyRedactionRowExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.assumes_default_destination_silently
            && !self.hides_queued_local_work
            && !self.drops_prepared_handoff_state
            && !self.hides_export_or_redaction_boundary
            && !self.leaks_credentials_or_endpoints
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OfflinePrivacyRowVocabularySet {
    /// Provider-surface-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Capture-kind tokens.
    pub capture_kinds: Vec<String>,
    /// Packet-destination-class tokens.
    pub destination_classes: Vec<String>,
    /// Offline-capture-row-posture tokens.
    pub capture_row_postures: Vec<String>,
    /// Publish-later-behavior tokens.
    pub publish_later_behaviors: Vec<String>,
    /// Offline-capture-row-action tokens.
    pub offline_row_actions: Vec<String>,
    /// Offline-anatomy-part tokens.
    pub offline_anatomy_parts: Vec<String>,
    /// Offline-export-field tokens.
    pub offline_export_fields: Vec<String>,
    /// Privacy/redaction-row-posture tokens.
    pub privacy_row_postures: Vec<String>,
    /// Redaction-policy-source tokens.
    pub policy_sources: Vec<String>,
    /// Telemetry / event-limit tokens.
    pub telemetry_limits: Vec<String>,
    /// Support-bundle-treatment tokens.
    pub support_bundle_treatments: Vec<String>,
    /// Privacy-field-class tokens.
    pub privacy_field_classes: Vec<String>,
    /// Privacy/redaction-row-action tokens.
    pub privacy_row_actions: Vec<String>,
    /// Privacy-anatomy-part tokens.
    pub privacy_anatomy_parts: Vec<String>,
    /// Privacy-export-field tokens.
    pub privacy_export_fields: Vec<String>,
    /// Offline-capture-state tokens (reused from the frozen matrix).
    pub capture_states: Vec<String>,
    /// Queued-draft-state tokens (reused from the frozen matrix).
    pub queued_draft_states: Vec<String>,
    /// Redaction-class tokens (reused from the frozen matrix).
    pub redaction_classes: Vec<String>,
    /// Export-boundary-class tokens (reused from the frozen matrix).
    pub export_boundaries: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5OfflinePrivacyRowVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5OfflinePrivacyConsumerSurface::ALL, |v| v.as_str()),
            capture_kinds: tokens(&M5OfflineCaptureKind::ALL, |v| v.as_str()),
            destination_classes: tokens(&M5OfflinePacketDestinationClass::ALL, |v| v.as_str()),
            capture_row_postures: tokens(&M5OfflineCaptureRowPosture::ALL, |v| v.as_str()),
            publish_later_behaviors: tokens(&M5PublishLaterBehavior::ALL, |v| v.as_str()),
            offline_row_actions: tokens(&M5OfflineCaptureRowAction::ALL, |v| v.as_str()),
            offline_anatomy_parts: tokens(&M5OfflineCaptureRowAnatomyPart::ALL, |v| v.as_str()),
            offline_export_fields: tokens(&M5OfflineCaptureRowExportField::ALL, |v| v.as_str()),
            privacy_row_postures: tokens(&M5PrivacyRedactionRowPosture::ALL, |v| v.as_str()),
            policy_sources: tokens(&M5RedactionPolicySource::ALL, |v| v.as_str()),
            telemetry_limits: tokens(&M5TelemetryEventLimit::ALL, |v| v.as_str()),
            support_bundle_treatments: tokens(&M5SupportBundleTreatment::ALL, |v| v.as_str()),
            privacy_field_classes: tokens(&M5PrivacyFieldClass::ALL, |v| v.as_str()),
            privacy_row_actions: tokens(&M5PrivacyRedactionRowAction::ALL, |v| v.as_str()),
            privacy_anatomy_parts: tokens(&M5PrivacyRedactionRowAnatomyPart::ALL, |v| v.as_str()),
            privacy_export_fields: tokens(&M5PrivacyRedactionRowExportField::ALL, |v| v.as_str()),
            capture_states: tokens(&M5OfflineCaptureState::ALL, |v| v.as_str()),
            queued_draft_states: tokens(&M5QueuedDraftState::ALL, |v| v.as_str()),
            redaction_classes: tokens(&M5ProviderRedactionClass::ALL, |v| v.as_str()),
            export_boundaries: tokens(&M5ExportBoundaryClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ProviderSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ProviderDeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ProviderAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5OfflinePrivacyRowGovernanceReview {
    /// The offline-capture row shows its packet destination.
    pub offline_row_shows_packet_destination: bool,
    /// The offline-capture row shows its queued-draft count.
    pub offline_row_shows_queued_draft_count: bool,
    /// The offline-capture row shows its redaction default.
    pub offline_row_shows_redaction_default: bool,
    /// The offline-capture row shows its publish-later behavior.
    pub offline_row_shows_publish_later_behavior: bool,
    /// The offline-capture row offers export and clear actions.
    pub offline_row_offers_export_and_clear: bool,
    /// The offline-capture row never erases prepared handoff state.
    pub offline_never_erases_prepared_handoff: bool,
    /// The privacy row states its copied / exported fields.
    pub privacy_row_states_copied_exported_fields: bool,
    /// The privacy row states its support-bundle treatment.
    pub privacy_row_states_support_bundle_treatment: bool,
    /// The privacy row states its telemetry / event limits.
    pub privacy_row_states_telemetry_event_limits: bool,
    /// The privacy row states its policy source.
    pub privacy_row_states_policy_source: bool,
    /// The privacy row offers a reviewed escalation action.
    pub privacy_row_offers_reviewed_escalation: bool,
    /// The metadata-safe default stays explicit before anything leaves the device.
    pub metadata_safe_default_explicit_before_leaving_device: bool,
    /// Rows keep the same truth across every deployment line.
    pub rows_stable_across_deployment_lines: bool,
    /// Rows keep the same truth across desktop, headless/export, and support consumers.
    pub rows_stable_across_consumer_surfaces: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The support / export packet reconstructs offline and privacy truth.
    pub support_export_reconstructs_offline_and_privacy_truth: bool,
    /// Later M5 rows cannot invent parallel offline or privacy vocabulary.
    pub later_rows_cannot_invent_parallel_offline_or_privacy_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OfflinePrivacyRowConsumerProjection {
    /// Provider surfaces consume the shared offline/privacy vocabulary.
    pub provider_surfaces_consume_offline_privacy_vocabulary: bool,
    /// The offline-posture resolver reads a single canonical source.
    pub offline_posture_reads_single_source: bool,
    /// The privacy-redaction derivation reads a single canonical source.
    pub privacy_redaction_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop rows read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OfflinePrivacyRowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the offline/privacy rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OfflinePrivacyRowReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting provider offline/privacy audit.
    pub provider_offline_privacy_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ProviderOfflinePrivacyRowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ProviderOfflinePrivacyRowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provider-surface rows.
    pub rows: Vec<M5OfflinePrivacyConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OfflinePrivacyRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OfflinePrivacyRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5OfflinePrivacyRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OfflinePrivacyRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OfflinePrivacyRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 provider offline-capture / privacy-redaction-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ProviderOfflinePrivacyRowPacket {
    /// Record kind; must equal [`M5_PROVIDER_OFFLINE_PRIVACY_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Provider-surface rows.
    pub rows: Vec<M5OfflinePrivacyConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OfflinePrivacyRowVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OfflinePrivacyRowGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5OfflinePrivacyRowConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OfflinePrivacyRowProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OfflinePrivacyRowReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ProviderOfflinePrivacyRowPacket {
    /// Builds an M5 offline/privacy-row-primitive packet from stable-lane input.
    pub fn new(input: M5ProviderOfflinePrivacyRowPacketInput) -> Self {
        Self {
            record_kind: M5_PROVIDER_OFFLINE_PRIVACY_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 offline/privacy-row-primitive invariants.
    pub fn validate(&self) -> Vec<M5ProviderOfflinePrivacyRowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PROVIDER_OFFLINE_PRIVACY_ROW_RECORD_KIND {
            violations.push(M5ProviderOfflinePrivacyRowViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_VERSION {
            violations.push(M5ProviderOfflinePrivacyRowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ProviderOfflinePrivacyRowViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_offline_capture_state_coverage(self, &mut violations);
        validate_redaction_class_coverage(self, &mut violations);
        validate_publish_later_separation(self, &mut violations);
        validate_export_clear_action_coverage(self, &mut violations);
        validate_packet_destination_explicitness(self, &mut violations);
        validate_queued_draft_visibility(self, &mut violations);
        validate_metadata_safe_boundary(self, &mut violations);
        validate_escalation_review_coverage(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 offline/privacy row primitive packet serializes"),
        ) {
            violations.push(M5ProviderOfflinePrivacyRowViolation::RawMaterialInExport);
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
            .expect("m5 offline/privacy row primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per provider-surface consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,capture_states,publish_behaviors,capture_actions,redaction_classes,export_boundaries,policy_sources,offline_examples,privacy_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.capture_states, |v| v.as_str()),
                join_tokens(&row.publish_later_behaviors, |v| v.as_str()),
                join_tokens(&row.offline_row_actions, |v| v.as_str()),
                join_tokens(&row.redaction_classes, |v| v.as_str()),
                join_tokens(&row.export_boundaries, |v| v.as_str()),
                join_tokens(&row.policy_sources, |v| v.as_str()),
                row.offline_examples.len(),
                row.privacy_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Provider Offline-Capture / Privacy-Redaction Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Provider-surface consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Capture states: {}\n",
            self.vocabulary_set.capture_states.join(", ")
        ));
        out.push_str(&format!(
            "- Publish-later behaviors: {}\n",
            self.vocabulary_set.publish_later_behaviors.join(", ")
        ));
        out.push_str(&format!(
            "- Redaction classes: {}\n",
            self.vocabulary_set.redaction_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Provider-surface consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str("  - Offline-capture rows:\n");
            for case in &row.offline_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (dest `{}`, queued `{}`, publish `{}`)\n",
                    case.resolved.capture_ref,
                    case.resolved.capture_state.as_str(),
                    case.resolved.capture_kind.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.destination_class.as_str(),
                    case.resolved.queued_draft_count,
                    case.resolved.publish_later_behavior.as_str(),
                ));
            }
            out.push_str("  - Privacy / redaction rows:\n");
            for case in &row.privacy_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (bundle `{}`, export `{}`, escalation-reviewed `{}`)\n",
                    case.resolved.redaction_ref,
                    case.resolved.redaction_class.as_str(),
                    case.resolved.export_boundary.as_str(),
                    case.resolved.row_posture.as_str(),
                    case.resolved.support_bundle_treatment.as_str(),
                    case.resolved.can_export,
                    case.resolved.escalation_requires_review,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 offline/privacy-row-primitive export.
#[derive(Debug)]
pub enum M5ProviderOfflinePrivacyRowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ProviderOfflinePrivacyRowViolation>),
}

impl fmt::Display for M5ProviderOfflinePrivacyRowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 offline/privacy row primitive export parse failed: {error}"
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
                    "m5 offline/privacy row primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ProviderOfflinePrivacyRowArtifactError {}

/// Validation failures emitted by [`M5ProviderOfflinePrivacyRowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ProviderOfflinePrivacyRowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required provider-surface consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A provider-surface row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory offline anatomy parts.
    MandatoryOfflineAnatomyMissing,
    /// A row omits one of the mandatory privacy anatomy parts.
    MandatoryPrivacyAnatomyMissing,
    /// A row omits one of the mandatory offline export fields.
    MandatoryOfflineExportMissing,
    /// A row omits one of the mandatory privacy export fields.
    MandatoryPrivacyExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked offline resolutions.
    OfflineExampleMissing,
    /// A row declares no worked privacy resolutions.
    PrivacyExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every offline-capture state.
    OfflineCaptureStateCoverageUnproven,
    /// The worked resolutions do not exercise every redaction class.
    RedactionClassCoverageUnproven,
    /// The worked resolutions do not prove the publishes-when-reachable, held-by-user,
    /// held-pending-conflict, and already-published separations.
    PublishLaterSeparationUnproven,
    /// The worked resolutions do not prove clear, retry, and a synced-and-cleared row without
    /// clear.
    ExportClearActionCoverageUnproven,
    /// The worked resolutions do not prove both an explicit destination and an unrouted packet.
    PacketDestinationExplicitnessUnproven,
    /// The worked resolutions do not prove both a queued and a cleared local-draft count.
    QueuedDraftVisibilityUnproven,
    /// The worked resolutions do not prove a metadata-safe export, a no-export block, and
    /// credential/endpoint withholding on every row.
    MetadataSafeBoundaryUnproven,
    /// The worked resolutions do not prove a reviewed escalation on every privacy row.
    EscalationReviewCoverageUnproven,
    /// A worked resolution does not preserve its exact offline / privacy identity.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ProviderOfflinePrivacyRowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryOfflineAnatomyMissing => "mandatory_offline_anatomy_missing",
            Self::MandatoryPrivacyAnatomyMissing => "mandatory_privacy_anatomy_missing",
            Self::MandatoryOfflineExportMissing => "mandatory_offline_export_missing",
            Self::MandatoryPrivacyExportMissing => "mandatory_privacy_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::OfflineExampleMissing => "offline_example_missing",
            Self::PrivacyExampleMissing => "privacy_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::OfflineCaptureStateCoverageUnproven => "offline_capture_state_coverage_unproven",
            Self::RedactionClassCoverageUnproven => "redaction_class_coverage_unproven",
            Self::PublishLaterSeparationUnproven => "publish_later_separation_unproven",
            Self::ExportClearActionCoverageUnproven => "export_clear_action_coverage_unproven",
            Self::PacketDestinationExplicitnessUnproven => {
                "packet_destination_explicitness_unproven"
            }
            Self::QueuedDraftVisibilityUnproven => "queued_draft_visibility_unproven",
            Self::MetadataSafeBoundaryUnproven => "metadata_safe_boundary_unproven",
            Self::EscalationReviewCoverageUnproven => "escalation_review_coverage_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 offline/privacy-row-primitive export.
pub fn current_stable_m5_provider_offline_privacy_row_export(
) -> Result<M5ProviderOfflinePrivacyRowPacket, M5ProviderOfflinePrivacyRowArtifactError> {
    let packet: M5ProviderOfflinePrivacyRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-provider-offline-capture-privacy-redaction-row-primitive-proof/support_export.json"
    )))
    .map_err(M5ProviderOfflinePrivacyRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ProviderOfflinePrivacyRowArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_SCHEMA_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_DOC_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_COMPONENT_MATRIX_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_OFFLINE_HANDOFF_REF,
        M5_PROVIDER_OFFLINE_PRIVACY_ROW_EXPORT_REDACTION_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ProviderOfflinePrivacyRowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ProviderOfflinePrivacyRowViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let present: BTreeSet<M5OfflinePrivacyConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5OfflinePrivacyConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5ProviderOfflinePrivacyRowViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.offline_anatomy_parts.is_empty()
            || row.privacy_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.capture_states.is_empty()
            || row.capture_kinds.is_empty()
            || row.destination_classes.is_empty()
            || row.capture_row_postures.is_empty()
            || row.publish_later_behaviors.is_empty()
            || row.offline_row_actions.is_empty()
            || row.redaction_classes.is_empty()
            || row.export_boundaries.is_empty()
            || row.policy_sources.is_empty()
            || row.telemetry_limits.is_empty()
            || row.support_bundle_treatments.is_empty()
            || row.privacy_field_classes.is_empty()
            || row.privacy_row_postures.is_empty()
            || row.privacy_row_actions.is_empty()
            || row.queued_draft_states.is_empty()
            || row.offline_export_fields.is_empty()
            || row.privacy_export_fields.is_empty()
        {
            violations.push(M5ProviderOfflinePrivacyRowViolation::RowIncomplete);
        }
        if !row.declares_mandatory_offline_anatomy() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::MandatoryOfflineAnatomyMissing);
        }
        if !row.declares_mandatory_privacy_anatomy() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::MandatoryPrivacyAnatomyMissing);
        }
        if !row.declares_mandatory_offline_export() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::MandatoryOfflineExportMissing);
        }
        if !row.declares_mandatory_privacy_export() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::MandatoryPrivacyExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ProviderAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5ProviderOfflinePrivacyRowViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::DowngradeTriggersMissing);
        }
        if row.offline_examples.is_empty() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::OfflineExampleMissing);
        }
        if row.privacy_examples.is_empty() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::PrivacyExampleMissing);
        }
        if row
            .offline_examples
            .iter()
            .any(|case| !case.is_self_consistent())
            || row
                .privacy_examples
                .iter()
                .any(|case| !case.is_self_consistent())
        {
            violations.push(M5ProviderOfflinePrivacyRowViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5ProviderOfflinePrivacyRowViolation::RowInvariantViolated);
        }
    }
}

/// Every offline-capture state must be exercised by some worked offline resolution — the
/// implementation requirement that offline-capture rows show their state without collapsing the
/// six states into one generic queued chip.
fn validate_offline_capture_state_coverage(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let exercised: BTreeSet<M5OfflineCaptureState> = packet
        .rows
        .iter()
        .flat_map(|row| row.offline_examples.iter())
        .map(|case| case.resolved.capture_state)
        .collect();
    if !M5OfflineCaptureState::ALL
        .iter()
        .all(|state| exercised.contains(state))
    {
        violations.push(M5ProviderOfflinePrivacyRowViolation::OfflineCaptureStateCoverageUnproven);
    }
}

/// Every redaction class must be exercised by some worked privacy resolution.
fn validate_redaction_class_coverage(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let exercised: BTreeSet<M5ProviderRedactionClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.privacy_examples.iter())
        .map(|case| case.resolved.redaction_class)
        .collect();
    if !M5ProviderRedactionClass::ALL
        .iter()
        .all(|class| exercised.contains(class))
    {
        violations.push(M5ProviderOfflinePrivacyRowViolation::RedactionClassCoverageUnproven);
    }
}

/// At least one worked offline resolution must prove each of the publishes-when-reachable,
/// held-by-user, held-pending-conflict, and already-published behaviors — the acceptance
/// criterion that a captured packet's publish-later behavior is never one ambiguous "queued"
/// label.
fn validate_publish_later_separation(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let exercised: BTreeSet<M5PublishLaterBehavior> = packet
        .rows
        .iter()
        .flat_map(|row| row.offline_examples.iter())
        .map(|case| case.resolved.publish_later_behavior)
        .collect();
    let required = [
        M5PublishLaterBehavior::PublishesWhenReachable,
        M5PublishLaterBehavior::HeldByUserChoice,
        M5PublishLaterBehavior::HeldPendingConflict,
        M5PublishLaterBehavior::AlreadyPublished,
    ];
    if !required.iter().all(|behavior| exercised.contains(behavior)) {
        violations.push(M5ProviderOfflinePrivacyRowViolation::PublishLaterSeparationUnproven);
    }
}

/// At least one worked offline resolution must prove clear offered, one retry offered, and one
/// synced-and-cleared row where clear is absent — the implementation requirement that a user can
/// export or clear queued work while an already-published capture no longer offers clear.
fn validate_export_clear_action_coverage(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.offline_examples.iter())
    };
    let has_clear = cases().any(|case| {
        case.resolved
            .available_actions
            .contains(&M5OfflineCaptureRowAction::ClearCapture)
    });
    let has_retry = cases().any(|case| {
        case.resolved
            .available_actions
            .contains(&M5OfflineCaptureRowAction::RetryPublish)
    });
    let has_synced_no_clear = cases().any(|case| {
        matches!(
            case.resolved.capture_state,
            M5OfflineCaptureState::SyncedCleared
        ) && !case
            .resolved
            .available_actions
            .contains(&M5OfflineCaptureRowAction::ClearCapture)
    });
    let all_export = cases().all(|case| {
        case.resolved
            .available_actions
            .contains(&M5OfflineCaptureRowAction::ExportPacket)
    });
    if !(has_clear && has_retry && has_synced_no_clear && all_export) {
        violations.push(M5ProviderOfflinePrivacyRowViolation::ExportClearActionCoverageUnproven);
    }
}

/// At least one worked offline resolution must show an explicit destination and one must flag
/// itself unrouted — the acceptance criterion that a user never has to guess where a queued
/// publish will land and no default is ever assumed silently.
fn validate_packet_destination_explicitness(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.offline_examples.iter())
    };
    let has_explicit = cases().any(|case| case.resolved.shows_packet_destination);
    let has_unrouted = cases().any(|case| !case.resolved.shows_packet_destination);
    let never_defaults = cases().all(|case| !case.resolved.assumes_default_destination_silently);
    if !(has_explicit && has_unrouted && never_defaults) {
        violations
            .push(M5ProviderOfflinePrivacyRowViolation::PacketDestinationExplicitnessUnproven);
    }
}

/// At least one worked offline resolution must show a queued count and one a cleared (zero)
/// count — the acceptance criterion that connectivity loss never hides what remains queued
/// locally.
fn validate_queued_draft_visibility(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.offline_examples.iter())
    };
    let has_queued = cases().any(|case| case.resolved.has_queued_drafts);
    let has_cleared = cases().any(|case| !case.resolved.has_queued_drafts);
    let shows_queue = cases().all(|case| case.retains_handoff());
    if !(has_queued && has_cleared && shows_queue) {
        violations.push(M5ProviderOfflinePrivacyRowViolation::QueuedDraftVisibilityUnproven);
    }
}

/// Every worked privacy resolution must withhold credentials and endpoints and keep its
/// metadata-safe default explicit, and the set must prove a metadata-safe export and a no-export
/// block — the acceptance criterion that metadata-safe export/support defaults stay explicit
/// before anything leaves the local device.
fn validate_metadata_safe_boundary(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let cases = || {
        packet
            .rows
            .iter()
            .flat_map(|row| row.privacy_examples.iter())
    };
    let all_withhold = cases().all(|case| case.keeps_boundary_and_withholds());
    let has_metadata_safe_export = cases().any(|case| {
        case.resolved.can_export
            && matches!(
                case.resolved.export_boundary,
                M5ExportBoundaryClass::MetadataSafe
            )
    });
    let has_no_export_block = cases().any(|case| !case.resolved.can_export);
    if !(all_withhold && has_metadata_safe_export && has_no_export_block) {
        violations.push(M5ProviderOfflinePrivacyRowViolation::MetadataSafeBoundaryUnproven);
    }
}

/// Every worked privacy resolution must require a reviewed escalation and offer the escalation
/// action — the implementation requirement that privacy rows name reviewed escalation actions.
fn validate_escalation_review_coverage(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let all_reviewed = packet
        .rows
        .iter()
        .flat_map(|row| row.privacy_examples.iter())
        .all(|case| {
            case.resolved.escalation_requires_review
                && case
                    .resolved
                    .available_actions
                    .contains(&M5PrivacyRedactionRowAction::RequestEscalationReview)
        });
    if !all_reviewed {
        violations.push(M5ProviderOfflinePrivacyRowViolation::EscalationReviewCoverageUnproven);
    }
}

/// Every worked resolution must preserve its exact offline / privacy identity — the invariant
/// that neither row ever rewrites the user's packet destination, capture, policy, or identity.
fn validate_identity_preservation(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let offline_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.offline_examples.iter())
        .all(|case| case.preserves_capture_identity());
    let privacy_ok = packet
        .rows
        .iter()
        .flat_map(|row| row.privacy_examples.iter())
        .all(|case| case.preserves_redaction_identity());
    if !(offline_ok && privacy_ok) {
        violations.push(M5ProviderOfflinePrivacyRowViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.offline_row_shows_packet_destination,
        review.offline_row_shows_queued_draft_count,
        review.offline_row_shows_redaction_default,
        review.offline_row_shows_publish_later_behavior,
        review.offline_row_offers_export_and_clear,
        review.offline_never_erases_prepared_handoff,
        review.privacy_row_states_copied_exported_fields,
        review.privacy_row_states_support_bundle_treatment,
        review.privacy_row_states_telemetry_event_limits,
        review.privacy_row_states_policy_source,
        review.privacy_row_offers_reviewed_escalation,
        review.metadata_safe_default_explicit_before_leaving_device,
        review.rows_stable_across_deployment_lines,
        review.rows_stable_across_consumer_surfaces,
        review.every_row_declares_accessibility_route,
        review.support_export_reconstructs_offline_and_privacy_truth,
        review.later_rows_cannot_invent_parallel_offline_or_privacy_vocabulary,
    ] {
        if !ok {
            violations.push(M5ProviderOfflinePrivacyRowViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.provider_surfaces_consume_offline_privacy_vocabulary,
        projection.offline_posture_reads_single_source,
        projection.privacy_redaction_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5ProviderOfflinePrivacyRowViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ProviderOfflinePrivacyRowViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ProviderOfflinePrivacyRowPacket,
    violations: &mut Vec<M5ProviderOfflinePrivacyRowViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.provider_offline_privacy_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ProviderOfflinePrivacyRowViolation::ReleasePostureIncomplete);
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
