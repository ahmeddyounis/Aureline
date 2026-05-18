//! Terminal protocol, clipboard, transcript-export, and restore-truth contract.
//!
//! This module owns the canonical [`TerminalSessionSummary`] record family
//! that every claimed M3 beta terminal row — local, remote, shared, support-
//! export, or AI-promoted slice — projects so the chrome, support packets,
//! and AI tool surfaces can read one truth instead of inventing per-surface
//! wording.
//!
//! The contract composes the existing primitives the crate already owns
//! ([`SessionHeader`], [`RestoredTerminalRecord`], [`TerminalHeaderRecord`],
//! [`TerminalScrollbackSnapshot`], and the shared-terminal control vocabulary
//! from the runtime crate) and adds the missing boundary truth for clipboard
//! posture, bracketed-paste state, linkification, shared roles, restore class,
//! and export/AI slice provenance.
//!
//! ## Why one summary record
//!
//! Without this contract, "is the terminal live, reconnecting, restored from
//! transcript, read-only degraded, or shared with narrowed authority?" gets
//! answered differently in the pane chrome, the activity center, the support
//! export, and AI tool calls — and clipboard, paste, and transcript actions
//! drift toward silent commits because no record names the boundary they cross.
//!
//! The summary forbids:
//!
//! - **Silent clipboard writes.** Every [`TerminalClipboardPostureClass`]
//!   admits or refuses a write with a typed denial reason; an OSC 52 or remote
//!   bridge write that does not cite a posture is invalid.
//! - **Silent paste submit.** Bracketed-paste state is part of the record;
//!   the contract refuses an `enabled` advertisement that pretends to allow
//!   auto-submit without a review surface.
//! - **Hidden ownership.** Shared-terminal rows always cite a
//!   [`TerminalSharedRoleClass`] and any narrowing reason; an
//!   `active_writer_grantee` row that does not name a `control_grant_ref` is
//!   refused.
//! - **Recovery that pretends to be live.** A restored-transcript or ended
//!   session always carries `auto_rerun_forbidden: true` and routes the user
//!   to the fresh-session command id.
//! - **Raw bodies in export.** Support packets and AI-promoted slices carry
//!   the canonical [`ScrollbackRedactionClass`] tokens; raw command lines,
//!   raw env, raw secrets, and raw clipboard payloads never cross the
//!   contract boundary.
//!
//! ## Vocabulary anchor
//!
//! Tokens are mirrored verbatim by:
//!
//! - `/schemas/runtime/terminal_session_summary.schema.json`
//! - `/schemas/runtime/terminal_export_packet.schema.json`
//! - `/fixtures/runtime/m3/terminal_protocol_and_restore/*.json`
//! - `/artifacts/runtime/m3/terminal_boundary_and_restore_truth.md`

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::headers::{TerminalHeaderRecord, TerminalHeaderRestoreState};
use crate::pty_host::{HostClass, PtySessionId, SessionLifecycleState, TerminalTrustState};
use crate::restore::{RestoredTerminalKind, TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID};
use crate::scrollback::{ScrollbackRedactionClass, TerminalScrollbackSnapshot};

/// Stable record-kind tag for [`TerminalSessionSummary`] payloads.
pub const TERMINAL_SESSION_SUMMARY_RECORD_KIND: &str = "terminal_session_summary_record";
/// Stable record-kind tag for [`TerminalExportPacket`] payloads.
pub const TERMINAL_EXPORT_PACKET_RECORD_KIND: &str = "terminal_export_packet_record";
/// Stable record-kind tag for [`TerminalSessionSummaryValidationReport`] payloads.
pub const TERMINAL_SESSION_SUMMARY_VALIDATION_REPORT_KIND: &str =
    "terminal_session_summary_validation_report";
/// Stable record-kind tag for [`TerminalAiPromotedSlice`] payloads.
pub const TERMINAL_AI_PROMOTED_SLICE_RECORD_KIND: &str = "terminal_ai_promoted_slice_record";
/// Schema version shared by every record in the protocol-contract family.
pub const TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Closed vocabulary for the session class.
///
/// Each summary row resolves to exactly one class. The class governs whether
/// clipboard writes, bracketed paste, shared-control fields, and AI-promoted
/// slice provenance are admissible without further review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalSessionClass {
    /// PTY backed by the user's local desktop.
    LocalTerminal,
    /// PTY backed by a managed remote agent or container.
    RemoteTerminal,
    /// Local pane bound to a shared-terminal control object.
    SharedTerminal,
    /// Inspect-only projection minted for support export or transcript review.
    ExportSupportView,
    /// Slice promoted into an AI tool call. Always evidence-only and bounded.
    AiPromotedSlice,
}

impl TerminalSessionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalTerminal => "local_terminal",
            Self::RemoteTerminal => "remote_terminal",
            Self::SharedTerminal => "shared_terminal",
            Self::ExportSupportView => "export_support_view",
            Self::AiPromotedSlice => "ai_promoted_slice",
        }
    }

    /// True when the row backs a live or recoverable interactive shell.
    pub const fn is_interactive_class(self) -> bool {
        matches!(self, Self::LocalTerminal | Self::RemoteTerminal | Self::SharedTerminal)
    }

    /// True when the row is evidence-only.
    pub const fn is_evidence_only(self) -> bool {
        matches!(self, Self::ExportSupportView | Self::AiPromotedSlice)
    }

    /// True when the row carries shared-control fields.
    pub const fn requires_shared_role(self) -> bool {
        matches!(self, Self::SharedTerminal)
    }
}

/// Live-authority class. Mirrors the restore states from the header strip and
/// adds the shared-narrowed and read-only-degraded posture the summary needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalLiveAuthorityClass {
    /// The row can accept input under its trust posture.
    Live,
    /// Transport is preparing or the row is warming.
    Warming,
    /// Transport dropped; explicit reconnect is required.
    Reconnecting,
    /// Prior session is restored as transcript evidence only.
    RecoveredTranscriptOnly,
    /// Prior session ended; a fresh session is required for live execution.
    EndedRequiresFreshSession,
    /// Row is interactive in principle but currently narrowed to read-only.
    ReadOnlyDegraded,
    /// Shared row has narrower authority than full driver.
    SharedNarrowedAuthority,
    /// Authority is blocked by policy, trust, or quarantine.
    AuthorityBlocked,
    /// Row is evidence-only and never had live authority.
    InspectOnly,
}

impl TerminalLiveAuthorityClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Warming => "warming",
            Self::Reconnecting => "reconnecting",
            Self::RecoveredTranscriptOnly => "recovered_transcript_only",
            Self::EndedRequiresFreshSession => "ended_requires_fresh_session",
            Self::ReadOnlyDegraded => "read_only_degraded",
            Self::SharedNarrowedAuthority => "shared_narrowed_authority",
            Self::AuthorityBlocked => "authority_blocked",
            Self::InspectOnly => "inspect_only",
        }
    }

    /// True when execution requires a renewed intent (reconnect, fresh
    /// session, or new approval) before further commands run.
    pub const fn requires_renewed_intent(self) -> bool {
        matches!(
            self,
            Self::Reconnecting
                | Self::EndedRequiresFreshSession
                | Self::RecoveredTranscriptOnly
                | Self::AuthorityBlocked
        )
    }

    /// True when the row admits input under any posture.
    pub const fn admits_input(self) -> bool {
        matches!(self, Self::Live | Self::SharedNarrowedAuthority)
    }
}

/// Closed vocabulary for clipboard posture.
///
/// Every summary row pins one posture. Writes that do not match the posture
/// are refused at the contract boundary; a `BlockedByPolicy` or `BlockedByTrust`
/// row cites a `denial_reason_class` so the chrome can disclose it instead of
/// silently dropping the write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalClipboardPostureClass {
    /// Writes are admitted from the local terminal under a preview surface.
    LocalAllowedWithPreview,
    /// Writes route through a remote clipboard bridge under a preview surface.
    RemoteBridgeAllowedWithPreview,
    /// Writes are admitted from a shared terminal narrowed to the grantee
    /// scope under a preview surface.
    SharedScopedAllowedWithPreview,
    /// Admin policy blocks every clipboard write on this row.
    BlockedByPolicy,
    /// Workspace trust narrows below the write requirement.
    BlockedByTrust,
    /// The payload class intersects a secret-adjacent class.
    BlockedBySecretClass,
    /// The session class does not admit clipboard writes at all.
    NotApplicable,
}

impl TerminalClipboardPostureClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAllowedWithPreview => "local_allowed_with_preview",
            Self::RemoteBridgeAllowedWithPreview => "remote_bridge_allowed_with_preview",
            Self::SharedScopedAllowedWithPreview => "shared_scoped_allowed_with_preview",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::BlockedByTrust => "blocked_by_trust",
            Self::BlockedBySecretClass => "blocked_by_secret_class",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the posture admits a clipboard write under preview.
    pub const fn admits_write(self) -> bool {
        matches!(
            self,
            Self::LocalAllowedWithPreview
                | Self::RemoteBridgeAllowedWithPreview
                | Self::SharedScopedAllowedWithPreview
        )
    }

    /// True when the posture denies a clipboard write with a typed reason.
    pub const fn denies_write(self) -> bool {
        matches!(
            self,
            Self::BlockedByPolicy | Self::BlockedByTrust | Self::BlockedBySecretClass
        )
    }
}

/// Closed vocabulary for bracketed-paste state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalBracketedPasteClass {
    /// Terminal has not advertised bracketed-paste capability.
    NotAdvertised,
    /// Bracketed paste advertised but disabled by the shell.
    AdvertisedDisabled,
    /// Bracketed paste advertised and enabled; review surface still required
    /// for high-risk paste.
    AdvertisedEnabled,
    /// The contract is forcing no-auto-submit regardless of shell state.
    ForcedNoAutoSubmit,
}

impl TerminalBracketedPasteClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotAdvertised => "not_advertised",
            Self::AdvertisedDisabled => "advertised_disabled",
            Self::AdvertisedEnabled => "advertised_enabled",
            Self::ForcedNoAutoSubmit => "forced_no_auto_submit",
        }
    }

    /// True when the paste path may bracket the inserted body.
    pub const fn brackets_paste(self) -> bool {
        matches!(self, Self::AdvertisedEnabled)
    }
}

/// Closed vocabulary for linkification posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalLinkificationClass {
    /// Linkification disabled on this row.
    Disabled,
    /// Linkified spans surface metadata only; raw URL not rendered active.
    MetadataOnly,
    /// Linkified spans are active under the row's boundary cue.
    EnabledWithBoundaryLabel,
}

impl TerminalLinkificationClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Disabled => "disabled",
            Self::MetadataOnly => "metadata_only",
            Self::EnabledWithBoundaryLabel => "enabled_with_boundary_label",
        }
    }
}

/// Closed vocabulary for the shared-terminal participant role projected onto
/// the summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalSharedRoleClass {
    /// Local pane is not a shared-terminal binding.
    NotShared,
    /// Owner / host of the shared terminal session.
    OwnerHost,
    /// View-only observer; no input authority.
    ViewOnlyObserver,
    /// Follower mirroring presenter cursor; no input authority.
    Follower,
    /// Active writer grantee with a typed control grant.
    ActiveWriterGrantee,
    /// Approver who can admit grants but does not drive.
    ApproverNonDriving,
    /// Admin acting under admin-signed admission; does not drive.
    AdminNonDriving,
}

impl TerminalSharedRoleClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotShared => "not_shared",
            Self::OwnerHost => "owner_host",
            Self::ViewOnlyObserver => "view_only_observer",
            Self::Follower => "follower",
            Self::ActiveWriterGrantee => "active_writer_grantee",
            Self::ApproverNonDriving => "approver_non_driving",
            Self::AdminNonDriving => "admin_non_driving",
        }
    }

    /// True when the role is admitted to drive input within its grant.
    pub const fn is_driver(self) -> bool {
        matches!(self, Self::OwnerHost | Self::ActiveWriterGrantee)
    }

    /// True when the role must cite a `control_grant_ref` to be valid.
    pub const fn requires_control_grant(self) -> bool {
        matches!(self, Self::ActiveWriterGrantee)
    }
}

/// Closed vocabulary for shared-terminal recording / retention posture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalRecordingClass {
    /// Not recorded outside the bounded scrollback ring.
    NotRecorded,
    /// Recording retained in workspace-scoped storage only.
    WorkspaceScopedRetention,
    /// Recording retained under support-bundle scope.
    SupportBundleRetention,
    /// Recording retained under an admin-signed scope.
    AdminScopedRetention,
}

impl TerminalRecordingClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRecorded => "not_recorded",
            Self::WorkspaceScopedRetention => "workspace_scoped_retention",
            Self::SupportBundleRetention => "support_bundle_retention",
            Self::AdminScopedRetention => "admin_scoped_retention",
        }
    }

    /// True when the row must visibly disclose a retention banner.
    pub const fn requires_recording_banner(self) -> bool {
        !matches!(self, Self::NotRecorded)
    }
}

/// Typed denial-reason class. Cited on rows that block a clipboard write,
/// paste commit, restore decision, or shared-control widen request.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalDenialReasonClass {
    /// No denial; the row admitted the action.
    NoDenial,
    /// Admin policy blocked the action.
    AdminPolicyBlocked,
    /// Workspace trust narrowed below the requirement.
    WorkspaceTrustNarrowed,
    /// Action quarantined for protocol-violation budget.
    ProtocolBudgetExceeded,
    /// Secret-class payload intersected the action body.
    SecretClassDetected,
    /// Restore root is missing on disk.
    RestoreRootMissing,
    /// Shared-terminal grant required and not present.
    SharedControlGrantRequired,
    /// Action target identity changed between intent and commit.
    TargetIdentityDrift,
    /// Action route identity changed between intent and commit.
    RouteIdentityDrift,
    /// Reconnect required before further action.
    ReconnectRequired,
    /// Fresh session required before further action.
    FreshSessionRequired,
}

impl TerminalDenialReasonClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDenial => "no_denial",
            Self::AdminPolicyBlocked => "admin_policy_blocked",
            Self::WorkspaceTrustNarrowed => "workspace_trust_narrowed",
            Self::ProtocolBudgetExceeded => "protocol_budget_exceeded",
            Self::SecretClassDetected => "secret_class_detected",
            Self::RestoreRootMissing => "restore_root_missing",
            Self::SharedControlGrantRequired => "shared_control_grant_required",
            Self::TargetIdentityDrift => "target_identity_drift",
            Self::RouteIdentityDrift => "route_identity_drift",
            Self::ReconnectRequired => "reconnect_required",
            Self::FreshSessionRequired => "fresh_session_required",
        }
    }

    /// True when the reason names a denial outcome.
    pub const fn is_denial(self) -> bool {
        !matches!(self, Self::NoDenial)
    }
}

/// Closed vocabulary for the export class on transcript and AI-promoted slice
/// records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalExportClass {
    /// Metadata-only export. Carries provenance, redaction, timestamps; no
    /// raw bodies.
    MetadataOnly,
    /// Support-bundle-scoped export. Carries scrollback bodies under the
    /// support-bundle redaction class.
    SupportBundleScoped,
    /// AI-promoted slice export. Bodies are carried under an explicit
    /// promotion provenance with timestamps and slice bounds.
    AiPromotedSlice,
}

impl TerminalExportClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataOnly => "metadata_only",
            Self::SupportBundleScoped => "support_bundle_scoped",
            Self::AiPromotedSlice => "ai_promoted_slice",
        }
    }

    /// True when this class may carry transcript bodies (under a redaction
    /// class).
    pub const fn admits_body(self) -> bool {
        matches!(self, Self::SupportBundleScoped | Self::AiPromotedSlice)
    }

    /// Resolve the corresponding scrollback redaction class for the export.
    pub const fn scrollback_redaction(self) -> ScrollbackRedactionClass {
        match self {
            Self::MetadataOnly => ScrollbackRedactionClass::MetadataAndHashesOnly,
            Self::SupportBundleScoped => ScrollbackRedactionClass::SupportBundleScoped,
            Self::AiPromotedSlice => ScrollbackRedactionClass::BroadenedCapture,
        }
    }
}

/// Reconnect / restore drift posture. Discloses whether the new target or
/// route identity matches the prior one or has skewed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReconnectDriftClass {
    /// Reconnect or restore is not applicable to this row.
    NotApplicable,
    /// Identity confirmed unchanged across the reconnect / restore boundary.
    IdentityUnchanged,
    /// The host or workspace identity changed; renewed intent required.
    HostOrWorkspaceDrift,
    /// Toolchain / runtime class drifted; review required.
    ToolchainDrift,
    /// Trust state narrowed; review required.
    TrustNarrowed,
    /// Policy epoch regressed; review required.
    PolicyEpochRegressed,
    /// Drift cannot be evaluated; reviewer must confirm.
    UnknownRequiresReview,
}

impl TerminalReconnectDriftClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotApplicable => "not_applicable",
            Self::IdentityUnchanged => "identity_unchanged",
            Self::HostOrWorkspaceDrift => "host_or_workspace_drift",
            Self::ToolchainDrift => "toolchain_drift",
            Self::TrustNarrowed => "trust_narrowed",
            Self::PolicyEpochRegressed => "policy_epoch_regressed",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// True when the drift class implies renewed intent is required before
    /// further execution.
    pub const fn requires_renewed_intent(self) -> bool {
        matches!(
            self,
            Self::HostOrWorkspaceDrift
                | Self::ToolchainDrift
                | Self::TrustNarrowed
                | Self::PolicyEpochRegressed
                | Self::UnknownRequiresReview
        )
    }
}

/// Boundary projection of the host class onto the summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalBoundary {
    /// Stable token derived from [`HostClass`].
    pub host_class_token: String,
    /// Short reviewable target badge.
    pub target_badge: String,
    /// Stable boundary cue token. Visible when [`Self::cue_visible`] is true.
    pub boundary_cue_token: String,
    /// True when the chrome must render the boundary cue prominently.
    pub cue_visible: bool,
}

impl TerminalBoundary {
    /// Build a boundary projection for the given host class.
    pub fn from_host_class(host_class: HostClass) -> Self {
        Self {
            host_class_token: host_class.as_str().to_owned(),
            target_badge: host_class.target_badge().to_owned(),
            boundary_cue_token: host_class.boundary_cue_token().to_owned(),
            cue_visible: host_class.needs_boundary_cue(),
        }
    }
}

/// Recovery posture summary. Combines restore-state, reconnect-drift, and the
/// command-id the chrome must route through when the user wants live state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalRecoveryPosture {
    /// Restore-state token. Mirrors [`TerminalHeaderRestoreState::as_str`].
    pub restore_state_token: String,
    /// Reconnect / restore drift class.
    pub reconnect_drift_class: TerminalReconnectDriftClass,
    /// Stable token for [`Self::reconnect_drift_class`].
    pub reconnect_drift_token: String,
    /// True when restore must never silently rerun a prior command.
    pub auto_rerun_forbidden: bool,
    /// True when live execution requires the user to open a fresh session.
    pub fresh_session_required: bool,
    /// Command id the chrome routes to when the user wants a fresh session.
    /// Populated whenever [`Self::fresh_session_required`] is `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_fresh_session_command_id: Option<String>,
    /// True when execution requires renewed intent (reconnect, fresh session,
    /// new approval) before further commands run.
    pub renewed_intent_required: bool,
}

impl TerminalRecoveryPosture {
    fn from_live(state: TerminalHeaderRestoreState) -> Self {
        let auto_rerun_forbidden = true;
        let fresh_session_required = matches!(
            state,
            TerminalHeaderRestoreState::CommandRerunRequired
                | TerminalHeaderRestoreState::RestoreBlocked
        );
        let renewed_intent_required = matches!(
            state,
            TerminalHeaderRestoreState::CommandRerunRequired
                | TerminalHeaderRestoreState::ReconnectRequired
                | TerminalHeaderRestoreState::RestoreBlocked
                | TerminalHeaderRestoreState::TranscriptRestored
        );
        Self {
            restore_state_token: state.as_str().to_owned(),
            reconnect_drift_class: TerminalReconnectDriftClass::NotApplicable,
            reconnect_drift_token: TerminalReconnectDriftClass::NotApplicable
                .as_str()
                .to_owned(),
            auto_rerun_forbidden,
            fresh_session_required,
            open_fresh_session_command_id: fresh_session_required
                .then(|| TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID.to_owned()),
            renewed_intent_required,
        }
    }
}

/// Promoted-range provenance carried by an AI-promoted slice or by an export
/// packet that bundles AI-readable bodies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalPromotedRangeProvenance {
    /// Slice id stable across reads.
    pub slice_id: String,
    /// Inclusive lower line index of the promoted range.
    pub first_line_index: u64,
    /// Inclusive upper line index of the promoted range.
    pub last_line_index: u64,
    /// Token naming the actor that promoted the slice (e.g. `ai_tool_call`).
    pub promoter_class_token: String,
    /// Stable opaque ref to the approval ticket or intent that admitted the
    /// promotion.
    pub admission_ref: String,
    /// Timestamp at which the promotion was minted.
    pub promoted_at: String,
}

/// Canonical terminal session summary row.
///
/// One row per terminal pane / restored object / export view / AI-promoted
/// slice. Surfaces consume the typed fields verbatim instead of inventing
/// per-surface vocabulary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalSessionSummary {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the row.
    pub schema_version: u32,
    /// Stable opaque summary id.
    pub summary_id: String,
    /// PTY session id this summary projects.
    pub session_id: PtySessionId,
    /// Opaque workspace id this summary lives under.
    pub workspace_id: String,
    /// Reviewable display title.
    pub display_title: String,
    /// Opaque ref to the execution context that owns the runtime/toolchain
    /// resolution for this row.
    pub execution_context_ref: String,
    /// Trust state at observation time.
    pub trust_state: TerminalTrustState,
    /// Stable token for [`Self::trust_state`].
    pub trust_state_token: String,
    /// Session class.
    pub session_class: TerminalSessionClass,
    /// Stable token for [`Self::session_class`].
    pub session_class_token: String,
    /// Boundary projection of the host class.
    pub boundary: TerminalBoundary,
    /// Lifecycle-state token from the PTY host. Empty when the row is an
    /// export view or AI-promoted slice.
    pub lifecycle_state_token: String,
    /// Live authority class.
    pub live_authority: TerminalLiveAuthorityClass,
    /// Stable token for [`Self::live_authority`].
    pub live_authority_token: String,
    /// Recovery posture. Always present so the chrome can disclose
    /// fresh-session and reconnect requirements without re-deriving them.
    pub recovery: TerminalRecoveryPosture,
    /// Clipboard posture.
    pub clipboard_posture: TerminalClipboardPostureClass,
    /// Stable token for [`Self::clipboard_posture`].
    pub clipboard_posture_token: String,
    /// Bracketed-paste state.
    pub bracketed_paste: TerminalBracketedPasteClass,
    /// Stable token for [`Self::bracketed_paste`].
    pub bracketed_paste_token: String,
    /// Linkification posture.
    pub linkification: TerminalLinkificationClass,
    /// Stable token for [`Self::linkification`].
    pub linkification_token: String,
    /// Shared-terminal role.
    pub shared_role: TerminalSharedRoleClass,
    /// Stable token for [`Self::shared_role`].
    pub shared_role_token: String,
    /// Opaque control-grant ref. Required when [`Self::shared_role`] is
    /// `active_writer_grantee`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub control_grant_ref: Option<String>,
    /// Recording / retention posture.
    pub recording_class: TerminalRecordingClass,
    /// Stable token for [`Self::recording_class`].
    pub recording_class_token: String,
    /// Last-known cwd hint. Class-only; never carries raw paths beyond a
    /// reviewable label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd_hint: Option<String>,
    /// Last-known prompt hint (e.g. `OSC133` boundary class). Never carries
    /// the command body.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prompt_hint: Option<String>,
    /// Denial reason class. `no_denial` whenever the row admits its actions.
    pub denial_reason: TerminalDenialReasonClass,
    /// Stable token for [`Self::denial_reason`].
    pub denial_reason_token: String,
    /// Reviewable denial label, when [`Self::denial_reason`] is set. Class-
    /// label only; never the raw payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denial_label: Option<String>,
    /// Guardrail: row carries no raw PTY bytes.
    pub raw_pty_bytes_present: bool,
    /// Guardrail: row carries no raw command body.
    pub raw_command_body_present: bool,
    /// Guardrail: row carries no raw clipboard payload.
    pub raw_clipboard_payload_present: bool,
    /// Guardrail: row preserves local terminal continuity when shared control
    /// ends or degrades.
    pub local_terminal_continuity_preserved: bool,
    /// Timestamp at which the summary was observed.
    pub observed_at: String,
}

impl TerminalSessionSummary {
    /// Build a redaction-safe summary row for a local PTY pane projected
    /// through the canonical header record. The result wires every field
    /// from the header strip and the host-class boundary, defaulting
    /// clipboard, paste, linkification, and shared fields to the safe
    /// `local_allowed_with_preview` / `not_advertised` / `disabled` /
    /// `not_shared` posture. Callers refine those fields with the
    /// `with_*` builders before publishing.
    pub fn from_header(
        summary_id: impl Into<String>,
        header: &TerminalHeaderRecord,
        trust_state: TerminalTrustState,
    ) -> Self {
        let session_class = if header.boundary_cue_visible {
            TerminalSessionClass::RemoteTerminal
        } else {
            TerminalSessionClass::LocalTerminal
        };
        let recovery = TerminalRecoveryPosture::from_live(header.restore_state);
        let live_authority = live_authority_for_restore_state(header.restore_state);
        let default_clipboard = if matches!(session_class, TerminalSessionClass::RemoteTerminal) {
            TerminalClipboardPostureClass::RemoteBridgeAllowedWithPreview
        } else {
            TerminalClipboardPostureClass::LocalAllowedWithPreview
        };
        Self {
            record_kind: TERMINAL_SESSION_SUMMARY_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION,
            summary_id: summary_id.into(),
            session_id: header.session_id.clone(),
            workspace_id: header.workspace_id.clone(),
            display_title: header.display_title.clone(),
            execution_context_ref: header.execution_context_ref.clone(),
            trust_state,
            trust_state_token: trust_state_token(trust_state).to_owned(),
            session_class,
            session_class_token: session_class.as_str().to_owned(),
            boundary: TerminalBoundary::from_host_class(header.host_class),
            lifecycle_state_token: header.lifecycle_state_token.clone(),
            live_authority,
            live_authority_token: live_authority.as_str().to_owned(),
            recovery,
            clipboard_posture: default_clipboard,
            clipboard_posture_token: default_clipboard.as_str().to_owned(),
            bracketed_paste: TerminalBracketedPasteClass::NotAdvertised,
            bracketed_paste_token: TerminalBracketedPasteClass::NotAdvertised.as_str().to_owned(),
            linkification: TerminalLinkificationClass::Disabled,
            linkification_token: TerminalLinkificationClass::Disabled.as_str().to_owned(),
            shared_role: TerminalSharedRoleClass::NotShared,
            shared_role_token: TerminalSharedRoleClass::NotShared.as_str().to_owned(),
            control_grant_ref: None,
            recording_class: TerminalRecordingClass::NotRecorded,
            recording_class_token: TerminalRecordingClass::NotRecorded.as_str().to_owned(),
            cwd_hint: if header.cwd_chip.state_token == "missing"
                || header.cwd_chip.display_value.is_empty()
            {
                None
            } else {
                Some(header.cwd_chip.display_value.clone())
            },
            prompt_hint: None,
            denial_reason: TerminalDenialReasonClass::NoDenial,
            denial_reason_token: TerminalDenialReasonClass::NoDenial.as_str().to_owned(),
            denial_label: None,
            raw_pty_bytes_present: false,
            raw_command_body_present: false,
            raw_clipboard_payload_present: false,
            local_terminal_continuity_preserved: true,
            observed_at: header.captured_at.clone(),
        }
    }

    /// Override the clipboard posture with an explicit class and optional
    /// denial reason.
    pub fn with_clipboard_posture(
        mut self,
        posture: TerminalClipboardPostureClass,
        denial: TerminalDenialReasonClass,
        denial_label: Option<String>,
    ) -> Self {
        self.clipboard_posture = posture;
        self.clipboard_posture_token = posture.as_str().to_owned();
        self.denial_reason = denial;
        self.denial_reason_token = denial.as_str().to_owned();
        self.denial_label = denial_label;
        self
    }

    /// Override the bracketed-paste state.
    pub fn with_bracketed_paste(mut self, state: TerminalBracketedPasteClass) -> Self {
        self.bracketed_paste = state;
        self.bracketed_paste_token = state.as_str().to_owned();
        self
    }

    /// Override the linkification posture.
    pub fn with_linkification(mut self, state: TerminalLinkificationClass) -> Self {
        self.linkification = state;
        self.linkification_token = state.as_str().to_owned();
        self
    }

    /// Promote the row into a shared-terminal binding with the given role and
    /// optional control-grant ref.
    pub fn with_shared_role(
        mut self,
        role: TerminalSharedRoleClass,
        control_grant_ref: Option<String>,
        recording: TerminalRecordingClass,
    ) -> Self {
        if matches!(role, TerminalSharedRoleClass::NotShared) {
            self.shared_role = role;
            self.shared_role_token = role.as_str().to_owned();
            self.control_grant_ref = None;
            self.recording_class = recording;
            self.recording_class_token = recording.as_str().to_owned();
            return self;
        }
        self.session_class = TerminalSessionClass::SharedTerminal;
        self.session_class_token = TerminalSessionClass::SharedTerminal.as_str().to_owned();
        self.shared_role = role;
        self.shared_role_token = role.as_str().to_owned();
        self.control_grant_ref = control_grant_ref;
        self.recording_class = recording;
        self.recording_class_token = recording.as_str().to_owned();
        if matches!(role, TerminalSharedRoleClass::ActiveWriterGrantee) {
            self.live_authority = TerminalLiveAuthorityClass::SharedNarrowedAuthority;
            self.live_authority_token = TerminalLiveAuthorityClass::SharedNarrowedAuthority
                .as_str()
                .to_owned();
        } else if matches!(role, TerminalSharedRoleClass::OwnerHost) {
            // owner retains live authority class assigned by from_header()
        } else {
            self.live_authority = TerminalLiveAuthorityClass::ReadOnlyDegraded;
            self.live_authority_token = TerminalLiveAuthorityClass::ReadOnlyDegraded
                .as_str()
                .to_owned();
        }
        self
    }

    /// Override the prompt hint.
    pub fn with_prompt_hint(mut self, prompt_hint: impl Into<String>) -> Self {
        let value = prompt_hint.into();
        self.prompt_hint = if value.is_empty() { None } else { Some(value) };
        self
    }

    /// Override the cwd hint.
    pub fn with_cwd_hint(mut self, cwd_hint: Option<String>) -> Self {
        self.cwd_hint = cwd_hint;
        self
    }

    /// Refine the reconnect / restore drift class.
    pub fn with_reconnect_drift(mut self, drift: TerminalReconnectDriftClass) -> Self {
        self.recovery.reconnect_drift_class = drift;
        self.recovery.reconnect_drift_token = drift.as_str().to_owned();
        if drift.requires_renewed_intent() {
            self.recovery.renewed_intent_required = true;
        }
        self
    }

    /// Mark the row as an evidence-only export or AI-promoted slice view.
    pub fn as_evidence_view(mut self, class: TerminalSessionClass) -> Self {
        debug_assert!(class.is_evidence_only());
        self.session_class = class;
        self.session_class_token = class.as_str().to_owned();
        self.live_authority = TerminalLiveAuthorityClass::InspectOnly;
        self.live_authority_token = TerminalLiveAuthorityClass::InspectOnly.as_str().to_owned();
        self.bracketed_paste = TerminalBracketedPasteClass::ForcedNoAutoSubmit;
        self.bracketed_paste_token =
            TerminalBracketedPasteClass::ForcedNoAutoSubmit.as_str().to_owned();
        self.clipboard_posture = TerminalClipboardPostureClass::NotApplicable;
        self.clipboard_posture_token =
            TerminalClipboardPostureClass::NotApplicable.as_str().to_owned();
        self
    }

    /// Project a restored terminal record into a summary row. The restored
    /// projection is always inspect-only and never claims live authority.
    pub fn from_restored_header(
        summary_id: impl Into<String>,
        header: &TerminalHeaderRecord,
        restored_kind: RestoredTerminalKind,
        trust_state: TerminalTrustState,
    ) -> Self {
        let mut summary = Self::from_header(summary_id, header, trust_state);
        summary.session_class = TerminalSessionClass::ExportSupportView;
        summary.session_class_token = TerminalSessionClass::ExportSupportView.as_str().to_owned();
        let live = match restored_kind {
            RestoredTerminalKind::Transcript => TerminalLiveAuthorityClass::RecoveredTranscriptOnly,
            RestoredTerminalKind::EndedSession => {
                TerminalLiveAuthorityClass::EndedRequiresFreshSession
            }
            RestoredTerminalKind::Declined => TerminalLiveAuthorityClass::AuthorityBlocked,
        };
        summary.live_authority = live;
        summary.live_authority_token = live.as_str().to_owned();
        summary.clipboard_posture = TerminalClipboardPostureClass::NotApplicable;
        summary.clipboard_posture_token =
            TerminalClipboardPostureClass::NotApplicable.as_str().to_owned();
        summary.bracketed_paste = TerminalBracketedPasteClass::ForcedNoAutoSubmit;
        summary.bracketed_paste_token =
            TerminalBracketedPasteClass::ForcedNoAutoSubmit.as_str().to_owned();
        summary
    }

    /// Validate the summary record against the contract. Returns a structured
    /// report. The row is valid when `report.passed` is true.
    pub fn validate(&self) -> TerminalSessionSummaryValidationReport {
        let mut errors: Vec<String> = Vec::new();
        let mut checks: BTreeSet<&'static str> = BTreeSet::new();

        if self.record_kind != TERMINAL_SESSION_SUMMARY_RECORD_KIND {
            errors.push(format!(
                "record_kind must be {TERMINAL_SESSION_SUMMARY_RECORD_KIND}"
            ));
        }
        checks.insert("record_kind");
        if self.schema_version != TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION {
            errors.push(format!(
                "schema_version must be {TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION}"
            ));
        }
        checks.insert("schema_version");
        if self.summary_id.is_empty() {
            errors.push("summary_id must be non-empty".to_owned());
        }
        checks.insert("summary_id");
        if self.workspace_id.is_empty() {
            errors.push("workspace_id must be non-empty".to_owned());
        }
        checks.insert("workspace_id");
        if self.execution_context_ref.is_empty() {
            errors.push("execution_context_ref must be non-empty".to_owned());
        }
        checks.insert("execution_context_ref");

        if self.session_class_token != self.session_class.as_str() {
            errors.push("session_class_token must mirror session_class".to_owned());
        }
        checks.insert("session_class_token");

        if self.live_authority_token != self.live_authority.as_str() {
            errors.push("live_authority_token must mirror live_authority".to_owned());
        }
        checks.insert("live_authority_token");

        if self.clipboard_posture_token != self.clipboard_posture.as_str() {
            errors.push("clipboard_posture_token must mirror clipboard_posture".to_owned());
        }
        checks.insert("clipboard_posture_token");

        if self.bracketed_paste_token != self.bracketed_paste.as_str() {
            errors.push("bracketed_paste_token must mirror bracketed_paste".to_owned());
        }
        checks.insert("bracketed_paste_token");

        if self.linkification_token != self.linkification.as_str() {
            errors.push("linkification_token must mirror linkification".to_owned());
        }
        checks.insert("linkification_token");

        if self.shared_role_token != self.shared_role.as_str() {
            errors.push("shared_role_token must mirror shared_role".to_owned());
        }
        checks.insert("shared_role_token");

        if self.denial_reason_token != self.denial_reason.as_str() {
            errors.push("denial_reason_token must mirror denial_reason".to_owned());
        }
        checks.insert("denial_reason_token");

        if self.recording_class_token != self.recording_class.as_str() {
            errors.push("recording_class_token must mirror recording_class".to_owned());
        }
        checks.insert("recording_class_token");

        if self.session_class.requires_shared_role()
            && matches!(self.shared_role, TerminalSharedRoleClass::NotShared)
        {
            errors.push("shared_terminal session_class requires a typed shared_role".to_owned());
        }
        checks.insert("shared_role_required");

        if !self.session_class.requires_shared_role()
            && !matches!(self.shared_role, TerminalSharedRoleClass::NotShared)
            && self.session_class != TerminalSessionClass::ExportSupportView
            && self.session_class != TerminalSessionClass::AiPromotedSlice
        {
            errors.push("non-shared session_class must keep shared_role=not_shared".to_owned());
        }
        checks.insert("shared_role_consistency");

        if self.shared_role.requires_control_grant() && self.control_grant_ref.is_none() {
            errors.push(
                "active_writer_grantee shared_role must cite a control_grant_ref".to_owned(),
            );
        }
        checks.insert("control_grant_required");

        if self.clipboard_posture.denies_write() && !self.denial_reason.is_denial() {
            errors.push("denied clipboard posture must cite a denial_reason".to_owned());
        }
        checks.insert("clipboard_denial_reason");

        if self.denial_reason.is_denial() && self.denial_label.is_none() {
            errors.push("denial_reason must cite a class-only denial_label".to_owned());
        }
        checks.insert("denial_label_required");

        if !self.recovery.auto_rerun_forbidden {
            errors.push("recovery.auto_rerun_forbidden must remain true".to_owned());
        }
        checks.insert("auto_rerun_forbidden");

        if self.recovery.fresh_session_required
            && self.recovery.open_fresh_session_command_id.as_deref()
                != Some(TERMINAL_OPEN_FRESH_SESSION_COMMAND_ID)
        {
            errors.push(
                "fresh_session_required rows must cite the open_fresh_session command id"
                    .to_owned(),
            );
        }
        checks.insert("fresh_session_command_id");

        if self.live_authority.requires_renewed_intent()
            && !self.recovery.renewed_intent_required
        {
            errors.push(
                "live_authority requires renewed intent but recovery.renewed_intent_required is false"
                    .to_owned(),
            );
        }
        checks.insert("renewed_intent_required");

        if self.raw_pty_bytes_present {
            errors.push("raw_pty_bytes_present must be false".to_owned());
        }
        if self.raw_command_body_present {
            errors.push("raw_command_body_present must be false".to_owned());
        }
        if self.raw_clipboard_payload_present {
            errors.push("raw_clipboard_payload_present must be false".to_owned());
        }
        checks.insert("no_raw_bodies");

        if self.session_class.is_evidence_only()
            && self.clipboard_posture != TerminalClipboardPostureClass::NotApplicable
        {
            errors.push(
                "evidence-only session_class must keep clipboard_posture=not_applicable".to_owned(),
            );
        }
        checks.insert("evidence_only_clipboard");

        if self.session_class.is_evidence_only()
            && self.bracketed_paste != TerminalBracketedPasteClass::ForcedNoAutoSubmit
        {
            errors.push(
                "evidence-only session_class must keep bracketed_paste=forced_no_auto_submit"
                    .to_owned(),
            );
        }
        checks.insert("evidence_only_bracketed_paste");

        if !self.local_terminal_continuity_preserved {
            errors.push("local_terminal_continuity_preserved must remain true".to_owned());
        }
        checks.insert("local_terminal_continuity_preserved");

        let passed = errors.is_empty();
        TerminalSessionSummaryValidationReport {
            record_kind: TERMINAL_SESSION_SUMMARY_VALIDATION_REPORT_KIND.to_owned(),
            schema_version: TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION,
            summary_id: self.summary_id.clone(),
            passed,
            checks: checks.into_iter().map(str::to_owned).collect(),
            errors,
        }
    }
}

/// Structured validation report emitted by [`TerminalSessionSummary::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalSessionSummaryValidationReport {
    pub record_kind: String,
    pub schema_version: u32,
    pub summary_id: String,
    pub passed: bool,
    pub checks: Vec<String>,
    pub errors: Vec<String>,
}

/// One promoted slice fed into an AI tool call. The slice is always evidence-
/// only and carries promoted-range provenance.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalAiPromotedSlice {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable opaque slice id.
    pub slice_id: String,
    /// Summary id this slice projects from.
    pub summary_id: String,
    /// PTY session id this slice was promoted from.
    pub session_id: PtySessionId,
    /// Promoted-range provenance.
    pub provenance: TerminalPromotedRangeProvenance,
    /// Bounded scrollback snapshot the slice carries. Bodies are admitted
    /// only under the `broadened_capture` redaction class.
    pub transcript: TerminalScrollbackSnapshot,
    /// Export class — always `ai_promoted_slice`.
    pub export_class: TerminalExportClass,
    /// Stable token for [`Self::export_class`].
    pub export_class_token: String,
    /// Guardrail: slice was admitted under an explicit promotion intent.
    pub admitted_with_intent: bool,
    /// Guardrail: slice carries no raw env / secret / clipboard bytes.
    pub raw_environment_or_secret_present: bool,
}

impl TerminalAiPromotedSlice {
    /// True when the slice's transcript carries no plaintext bodies for
    /// records at the metadata-only class.
    pub fn admits_body(&self) -> bool {
        self.export_class.admits_body()
    }
}

/// Support-export packet bundling a summary, a transcript snapshot, an
/// optional AI-promoted slice, and structured provenance fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TerminalExportPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable opaque export id.
    pub export_id: String,
    /// Summary projection that owns the boundary truth.
    pub summary: TerminalSessionSummary,
    /// Export class.
    pub export_class: TerminalExportClass,
    /// Stable token for [`Self::export_class`].
    pub export_class_token: String,
    /// Stable token for the scrollback redaction class the bundle uses.
    pub scrollback_redaction_class_token: String,
    /// Bounded scrollback transcript. May be absent for metadata-only exports
    /// or declined-restore views.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transcript: Option<TerminalScrollbackSnapshot>,
    /// Optional AI-promoted slice the bundle carries.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promoted_slice: Option<TerminalAiPromotedSlice>,
    /// Promoted-range provenance for the bundle. Required when
    /// [`Self::promoted_slice`] is present; otherwise optional.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub promoted_range_provenance: Option<TerminalPromotedRangeProvenance>,
    /// Reviewable summary string safe for support export.
    pub support_export_summary: String,
    /// Guardrail: packet exports raw bodies by default.
    pub raw_bodies_exported_by_default: bool,
    /// Guardrail: packet carries no raw env / secret / clipboard bytes.
    pub raw_environment_or_secret_present: bool,
    /// Timestamp at which the export packet was minted.
    pub minted_at: String,
}

impl TerminalExportPacket {
    /// Build a metadata-only export packet for the given summary. Transcript
    /// and promoted-slice fields are omitted.
    pub fn metadata_only(
        export_id: impl Into<String>,
        summary: TerminalSessionSummary,
        support_export_summary: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        let class = TerminalExportClass::MetadataOnly;
        Self {
            record_kind: TERMINAL_EXPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION,
            export_id: export_id.into(),
            summary,
            export_class: class,
            export_class_token: class.as_str().to_owned(),
            scrollback_redaction_class_token: class.scrollback_redaction().as_str().to_owned(),
            transcript: None,
            promoted_slice: None,
            promoted_range_provenance: None,
            support_export_summary: support_export_summary.into(),
            raw_bodies_exported_by_default: false,
            raw_environment_or_secret_present: false,
            minted_at: minted_at.into(),
        }
    }

    /// Build a support-bundle-scoped export packet that carries a bounded
    /// transcript snapshot under the support-bundle redaction class.
    pub fn support_bundle_scoped(
        export_id: impl Into<String>,
        summary: TerminalSessionSummary,
        transcript: TerminalScrollbackSnapshot,
        support_export_summary: impl Into<String>,
        minted_at: impl Into<String>,
    ) -> Self {
        let class = TerminalExportClass::SupportBundleScoped;
        Self {
            record_kind: TERMINAL_EXPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION,
            export_id: export_id.into(),
            summary,
            export_class: class,
            export_class_token: class.as_str().to_owned(),
            scrollback_redaction_class_token: class.scrollback_redaction().as_str().to_owned(),
            transcript: Some(transcript),
            promoted_slice: None,
            promoted_range_provenance: None,
            support_export_summary: support_export_summary.into(),
            raw_bodies_exported_by_default: false,
            raw_environment_or_secret_present: false,
            minted_at: minted_at.into(),
        }
    }

    /// Attach an AI-promoted slice to the bundle. The summary class is
    /// rewritten to `ai_promoted_slice`.
    pub fn with_promoted_slice(mut self, slice: TerminalAiPromotedSlice) -> Self {
        self.export_class = TerminalExportClass::AiPromotedSlice;
        self.export_class_token = TerminalExportClass::AiPromotedSlice.as_str().to_owned();
        self.scrollback_redaction_class_token = TerminalExportClass::AiPromotedSlice
            .scrollback_redaction()
            .as_str()
            .to_owned();
        self.promoted_range_provenance = Some(slice.provenance.clone());
        self.transcript = Some(slice.transcript.clone());
        self.summary = self.summary.as_evidence_view(TerminalSessionClass::AiPromotedSlice);
        self.promoted_slice = Some(slice);
        self
    }

    /// Validate the packet. Returns a list of validation errors. Empty
    /// indicates the packet is valid.
    pub fn validate(&self) -> Vec<String> {
        let mut errors: Vec<String> = Vec::new();
        if self.record_kind != TERMINAL_EXPORT_PACKET_RECORD_KIND {
            errors.push(format!(
                "record_kind must be {TERMINAL_EXPORT_PACKET_RECORD_KIND}"
            ));
        }
        if self.schema_version != TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION {
            errors.push("schema_version must match the contract version".to_owned());
        }
        if self.export_id.is_empty() {
            errors.push("export_id must be non-empty".to_owned());
        }
        if self.export_class_token != self.export_class.as_str() {
            errors.push("export_class_token must mirror export_class".to_owned());
        }
        if self.scrollback_redaction_class_token != self.export_class.scrollback_redaction().as_str()
        {
            errors.push(
                "scrollback_redaction_class_token must mirror export_class.scrollback_redaction()"
                    .to_owned(),
            );
        }
        if self.raw_bodies_exported_by_default {
            errors.push("raw_bodies_exported_by_default must be false".to_owned());
        }
        if self.raw_environment_or_secret_present {
            errors.push("raw_environment_or_secret_present must be false".to_owned());
        }
        if matches!(self.export_class, TerminalExportClass::MetadataOnly)
            && self.transcript.is_some()
        {
            errors.push(
                "metadata_only export packets must omit the transcript snapshot".to_owned(),
            );
        }
        if matches!(self.export_class, TerminalExportClass::AiPromotedSlice)
            && self.promoted_slice.is_none()
        {
            errors.push(
                "ai_promoted_slice export packets must attach a promoted_slice record".to_owned(),
            );
        }
        if matches!(self.export_class, TerminalExportClass::AiPromotedSlice)
            && self.promoted_range_provenance.is_none()
        {
            errors.push(
                "ai_promoted_slice export packets must cite promoted_range_provenance".to_owned(),
            );
        }
        let inner = self.summary.validate();
        if !inner.passed {
            errors.extend(
                inner
                    .errors
                    .into_iter()
                    .map(|err| format!("summary: {err}")),
            );
        }
        errors
    }
}

const fn trust_state_token(trust: TerminalTrustState) -> &'static str {
    match trust {
        TerminalTrustState::Trusted => "trusted",
        TerminalTrustState::Restricted => "restricted",
        TerminalTrustState::PendingEvaluation => "pending_evaluation",
    }
}

const fn live_authority_for_restore_state(
    state: TerminalHeaderRestoreState,
) -> TerminalLiveAuthorityClass {
    match state {
        TerminalHeaderRestoreState::Live => TerminalLiveAuthorityClass::Live,
        TerminalHeaderRestoreState::Warming => TerminalLiveAuthorityClass::Warming,
        TerminalHeaderRestoreState::ReconnectRequired => TerminalLiveAuthorityClass::Reconnecting,
        TerminalHeaderRestoreState::CommandRerunRequired => {
            TerminalLiveAuthorityClass::EndedRequiresFreshSession
        }
        TerminalHeaderRestoreState::TranscriptRestored => {
            TerminalLiveAuthorityClass::RecoveredTranscriptOnly
        }
        TerminalHeaderRestoreState::InspectOnly => TerminalLiveAuthorityClass::InspectOnly,
        TerminalHeaderRestoreState::RestoreBlocked => TerminalLiveAuthorityClass::AuthorityBlocked,
    }
}

/// True when the lifecycle state implies the chrome must render a
/// reconnect-required cue regardless of any UI degradation.
pub const fn lifecycle_state_requires_reconnect(state: SessionLifecycleState) -> bool {
    matches!(state, SessionLifecycleState::LostTransport)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::headers::TerminalHeaderRecord;
    use crate::pty_host::{OpenSessionRequest, PtyHost, PtySessionId};
    use crate::restore::{
        decline_session_restore, restore_session_as_transcript, RestoreDeclinedReason,
    };
    use crate::scrollback::{ScrollbackRedactionClass, TerminalScrollback};

    fn open_local(host: &mut PtyHost) -> PtySessionId {
        host.open_session(OpenSessionRequest {
            workspace_id: "ws-summary",
            host_class: HostClass::HostDesktop,
            display_title: "zsh",
            cwd_hint: Some("~/code/aureline"),
            execution_context_ref: "exec:ws-summary:terminal:0",
            trust_state: TerminalTrustState::Trusted,
            observed_at: "mono:0",
        })
    }

    fn open_remote(host: &mut PtyHost) -> PtySessionId {
        host.open_session(OpenSessionRequest {
            workspace_id: "ws-summary",
            host_class: HostClass::RemoteAgentPrimary,
            display_title: "remote shell",
            cwd_hint: Some("/srv/app"),
            execution_context_ref: "exec:ws-summary:remote:0",
            trust_state: TerminalTrustState::Trusted,
            observed_at: "mono:0",
        })
    }

    #[test]
    fn local_live_summary_admits_local_clipboard_and_passes_validation() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        let session = host.session(&id).expect("session exists");
        let header = TerminalHeaderRecord::project_session(session);

        let summary = TerminalSessionSummary::from_header(
            "summary:local:0",
            &header,
            TerminalTrustState::Trusted,
        );

        assert_eq!(summary.session_class, TerminalSessionClass::LocalTerminal);
        assert_eq!(summary.live_authority, TerminalLiveAuthorityClass::Live);
        assert_eq!(
            summary.clipboard_posture,
            TerminalClipboardPostureClass::LocalAllowedWithPreview
        );
        assert!(summary.recovery.auto_rerun_forbidden);
        assert!(!summary.recovery.fresh_session_required);
        assert!(summary.validate().passed);
    }

    #[test]
    fn remote_lost_transport_summary_requires_reconnect_and_drift_review() {
        let mut host = PtyHost::new();
        let id = open_remote(&mut host);
        host.mark_lost_transport(&id, "mono:1", Some("network_drop"))
            .unwrap();
        let session = host.session(&id).expect("session exists");
        let header = TerminalHeaderRecord::project_session(session);

        let summary = TerminalSessionSummary::from_header(
            "summary:remote:0",
            &header,
            TerminalTrustState::Trusted,
        )
        .with_reconnect_drift(TerminalReconnectDriftClass::HostOrWorkspaceDrift);

        assert_eq!(summary.session_class, TerminalSessionClass::RemoteTerminal);
        assert_eq!(
            summary.live_authority,
            TerminalLiveAuthorityClass::Reconnecting
        );
        assert!(summary.recovery.renewed_intent_required);
        assert_eq!(
            summary.recovery.reconnect_drift_class,
            TerminalReconnectDriftClass::HostOrWorkspaceDrift
        );
        assert_eq!(
            summary.clipboard_posture,
            TerminalClipboardPostureClass::RemoteBridgeAllowedWithPreview
        );
        let report = summary.validate();
        assert!(report.passed, "errors: {:?}", report.errors);
    }

    #[test]
    fn blocked_clipboard_summary_cites_typed_denial_reason() {
        let mut host = PtyHost::new();
        let id = open_remote(&mut host);
        let session = host.session(&id).expect("session exists");
        let header = TerminalHeaderRecord::project_session(session);

        let summary = TerminalSessionSummary::from_header(
            "summary:remote:1",
            &header,
            TerminalTrustState::Trusted,
        )
        .with_clipboard_posture(
            TerminalClipboardPostureClass::BlockedByPolicy,
            TerminalDenialReasonClass::AdminPolicyBlocked,
            Some("clipboard_write_blocked_admin_metadata_only".to_owned()),
        );
        assert!(summary.clipboard_posture.denies_write());
        assert!(summary.denial_reason.is_denial());
        let report = summary.validate();
        assert!(report.passed, "errors: {:?}", report.errors);
    }

    #[test]
    fn shared_active_writer_summary_requires_control_grant_ref() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        let session = host.session(&id).expect("session exists");
        let header = TerminalHeaderRecord::project_session(session);

        let summary = TerminalSessionSummary::from_header(
            "summary:shared:writer",
            &header,
            TerminalTrustState::Trusted,
        )
        .with_shared_role(
            TerminalSharedRoleClass::ActiveWriterGrantee,
            Some("collab.control_grant.alpha.001".to_owned()),
            TerminalRecordingClass::WorkspaceScopedRetention,
        );
        assert_eq!(summary.session_class, TerminalSessionClass::SharedTerminal);
        assert_eq!(
            summary.live_authority,
            TerminalLiveAuthorityClass::SharedNarrowedAuthority
        );
        assert!(summary.validate().passed);

        let missing_grant = TerminalSessionSummary::from_header(
            "summary:shared:writer-missing",
            &header,
            TerminalTrustState::Trusted,
        )
        .with_shared_role(
            TerminalSharedRoleClass::ActiveWriterGrantee,
            None,
            TerminalRecordingClass::WorkspaceScopedRetention,
        );
        let report = missing_grant.validate();
        assert!(!report.passed);
        assert!(report
            .errors
            .iter()
            .any(|err| err.contains("control_grant_ref")));
    }

    #[test]
    fn restored_transcript_summary_is_inspect_only_no_rerun() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.close(&id, "mono:1", Some("user_closed")).unwrap();
        let mut scrollback = TerminalScrollback::new(id.clone());
        scrollback.record_line(
            "$ cargo test",
            ScrollbackRedactionClass::SupportBundleScoped,
            "mono:0",
        );
        let prior = host.session(&id).expect("session exists");
        let restored = restore_session_as_transcript(prior, Some(&scrollback), "mono:restart");
        let header = TerminalHeaderRecord::project_restored(&restored);

        let summary = TerminalSessionSummary::from_restored_header(
            "summary:restored:0",
            &header,
            restored.kind,
            TerminalTrustState::Trusted,
        );
        assert_eq!(
            summary.session_class,
            TerminalSessionClass::ExportSupportView
        );
        assert_eq!(
            summary.live_authority,
            TerminalLiveAuthorityClass::RecoveredTranscriptOnly
        );
        assert!(summary.recovery.auto_rerun_forbidden);
        assert!(summary.recovery.renewed_intent_required);
        assert_eq!(
            summary.clipboard_posture,
            TerminalClipboardPostureClass::NotApplicable
        );
        assert!(summary.validate().passed);
    }

    #[test]
    fn declined_restore_summary_is_authority_blocked() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        host.quarantine(&id, "mono:1", "terminal_protocol_violation_budget_exceeded")
            .unwrap();
        let prior = host.session(&id).expect("session exists");
        let declined =
            decline_session_restore(prior, RestoreDeclinedReason::DeclinedByPolicy, "mono:restart");
        let header = TerminalHeaderRecord::project_restored(&declined);

        let summary = TerminalSessionSummary::from_restored_header(
            "summary:restored:declined",
            &header,
            declined.kind,
            TerminalTrustState::Restricted,
        );
        assert_eq!(
            summary.live_authority,
            TerminalLiveAuthorityClass::AuthorityBlocked
        );
        assert!(summary.recovery.renewed_intent_required);
        let report = summary.validate();
        assert!(report.passed, "errors: {:?}", report.errors);
    }

    #[test]
    fn metadata_only_export_packet_omits_transcript_and_passes_validation() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        let session = host.session(&id).expect("session exists");
        let header = TerminalHeaderRecord::project_session(session);
        let summary = TerminalSessionSummary::from_header(
            "summary:export:0",
            &header,
            TerminalTrustState::Trusted,
        );
        let packet = TerminalExportPacket::metadata_only(
            "export:metadata:0",
            summary,
            "metadata-only export bundle",
            "mono:2",
        );
        assert!(packet.transcript.is_none());
        let errors = packet.validate();
        assert!(errors.is_empty(), "errors: {errors:?}");
    }

    #[test]
    fn ai_promoted_slice_export_packet_carries_provenance() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        let mut scrollback = TerminalScrollback::new(id.clone());
        scrollback.record_line(
            "build succeeded in 3.4s",
            ScrollbackRedactionClass::BroadenedCapture,
            "mono:1",
        );
        let snapshot = scrollback.snapshot("mono:2");
        let session = host.session(&id).expect("session exists");
        let header = TerminalHeaderRecord::project_session(session);
        let summary = TerminalSessionSummary::from_header(
            "summary:ai:0",
            &header,
            TerminalTrustState::Trusted,
        );
        let provenance = TerminalPromotedRangeProvenance {
            slice_id: "slice:ai:0".to_owned(),
            first_line_index: 0,
            last_line_index: 0,
            promoter_class_token: "ai_tool_call".to_owned(),
            admission_ref: "approval.ticket.ai_tool_call.001".to_owned(),
            promoted_at: "mono:2".to_owned(),
        };
        let slice = TerminalAiPromotedSlice {
            record_kind: TERMINAL_AI_PROMOTED_SLICE_RECORD_KIND.to_owned(),
            schema_version: TERMINAL_PROTOCOL_CONTRACT_SCHEMA_VERSION,
            slice_id: "slice:ai:0".to_owned(),
            summary_id: summary.summary_id.clone(),
            session_id: id.clone(),
            provenance,
            transcript: snapshot,
            export_class: TerminalExportClass::AiPromotedSlice,
            export_class_token: TerminalExportClass::AiPromotedSlice.as_str().to_owned(),
            admitted_with_intent: true,
            raw_environment_or_secret_present: false,
        };
        let packet = TerminalExportPacket::metadata_only(
            "export:ai:0",
            summary,
            "ai-promoted slice export",
            "mono:2",
        )
        .with_promoted_slice(slice);

        let errors = packet.validate();
        assert!(errors.is_empty(), "errors: {errors:?}");
        assert!(packet.promoted_range_provenance.is_some());
        assert_eq!(
            packet.summary.session_class,
            TerminalSessionClass::AiPromotedSlice
        );
        assert_eq!(
            packet.summary.live_authority,
            TerminalLiveAuthorityClass::InspectOnly
        );
        assert_eq!(
            packet.summary.clipboard_posture,
            TerminalClipboardPostureClass::NotApplicable
        );
    }

    #[test]
    fn export_packet_round_trips_via_serde() {
        let mut host = PtyHost::new();
        let id = open_local(&mut host);
        let session = host.session(&id).expect("session exists");
        let header = TerminalHeaderRecord::project_session(session);
        let summary = TerminalSessionSummary::from_header(
            "summary:export:rt",
            &header,
            TerminalTrustState::Trusted,
        );
        let packet = TerminalExportPacket::metadata_only(
            "export:metadata:rt",
            summary,
            "round trip metadata export",
            "mono:1",
        );
        let json = serde_json::to_string(&packet).expect("serialize");
        let round: TerminalExportPacket = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(round, packet);
    }
}
