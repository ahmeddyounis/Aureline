//! Browser-handoff, import-session, and provider-reconnect continuity on
//! claimed provider rows.
//!
//! The provider-object alpha ([`crate::object_model`]) named the typed local
//! object rows that workspace, runtime, review, and git lanes mint for every
//! code-host, issue, and CI/check object. The route-resolution beta
//! ([`crate::route_resolution`]) named the typed browser-handoff and
//! authority-truth panels per provider-linked beta row. This module owns the
//! bounded alpha contract that keeps browser handoff *and* re-entry honest:
//! every typed [`BrowserHandoffPacket`] names origin, destination class,
//! object id, and intended follow-up action; every [`ProviderReconnectFlow`]
//! resolves to either the authoritative local object row or a truthful
//! placeholder; every [`ImportSessionRecord`] preserves the same continuity
//! vocabulary the provider-object support export already uses.
//!
//! The alpha promise is narrow and strict:
//!
//! - Browser-handoff packets do not collapse into a generic "open in
//!   browser" affordance. Each packet names the [`HandoffOriginClass`] (which
//!   workspace lane minted it), the [`HandoffDestinationClass`] (which
//!   provider surface it routes to), the opaque object row id on the local
//!   object model, and the typed [`HandoffFollowUpActionClass`] the user is
//!   intended to take when they return to product scope.
//! - When the user reopens a provider flow Aureline has minted before, the
//!   [`ProviderReconnectFlow`] resolves to either the authoritative local
//!   object row (when the local model is still authoritative) or to a
//!   typed [`HandoffPlaceholderClass`] that names *why* the authoritative
//!   row could not be restored. A reconnect never silently drops to a
//!   generic "session lost" state.
//! - When the upstream provider is offline, stale, expired, revoked, or
//!   disagrees with the local model, every packet, import session, and
//!   reconnect flow records a [`HandoffContinuityObservation`] using the
//!   same closed [`ContinuityObservationClass`] /
//!   [`RetainedCapabilityClass`] / [`DegradedActionClass`] vocabulary as the
//!   provider-object support export, so support packets read one truth.
//!
//! The cross-tool boundary lives at
//! [`/schemas/providers/provider_browser_handoff_alpha.schema.json`](../../../../schemas/providers/provider_browser_handoff_alpha.schema.json).
//! The reviewer-facing landing page lives at
//! [`/docs/runtime/m3/provider_handoff_alpha.md`](../../../../docs/runtime/m3/provider_handoff_alpha.md).
//! The reviewer fixture lives at
//! [`/fixtures/providers/m3/browser_handoff/page.json`](../../../../fixtures/providers/m3/browser_handoff/page.json).

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::object_model::{
    ContinuityObservationClass, DegradedActionClass, RetainedCapabilityClass,
};
use crate::registry::{FreshnessLabel, FreshnessTruth, ProviderFamily, RedactionClass};

/// Alpha schema version exported with every browser-handoff record.
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every browser-handoff record.
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF: &str =
    "providers:provider_browser_handoff_alpha:v1";

/// Stable record-kind tag for [`ProviderBrowserHandoffAlphaPage`] payloads.
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_PAGE_RECORD_KIND: &str =
    "provider_browser_handoff_alpha_page_record";

/// Stable record-kind tag for [`BrowserHandoffPacket`] payloads.
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_PACKET_RECORD_KIND: &str =
    "provider_browser_handoff_alpha_packet_record";

/// Stable record-kind tag for [`ImportSessionRecord`] payloads.
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_IMPORT_SESSION_RECORD_KIND: &str =
    "provider_browser_handoff_alpha_import_session_record";

/// Stable record-kind tag for [`ProviderReconnectFlow`] payloads.
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_RECONNECT_FLOW_RECORD_KIND: &str =
    "provider_browser_handoff_alpha_reconnect_flow_record";

/// Stable record-kind tag for [`HandoffContinuityObservation`] payloads.
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND: &str =
    "provider_browser_handoff_alpha_continuity_observation_record";

/// Stable record-kind tag for
/// [`ProviderBrowserHandoffAlphaValidationReport`] payloads.
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_VALIDATION_REPORT_RECORD_KIND: &str =
    "provider_browser_handoff_alpha_validation_report";

/// Stable record-kind tag for [`ProviderBrowserHandoffAlphaSupportExport`].
pub const PROVIDER_BROWSER_HANDOFF_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "provider_browser_handoff_alpha_support_export";

/// Origin class for one handoff packet. Names *which workspace lane*
/// minted the packet so the same vocabulary appears in support exports,
/// audit rollups, and re-entry routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffOriginClass {
    /// Minted from the workspace start center / main window.
    WorkspaceStartCenter,
    /// Minted from the review pack inspector lane.
    WorkspaceReviewLane,
    /// Minted from the runtime/task-graph lane.
    WorkspaceRuntimeLane,
    /// Minted from the git/change-orchestration lane.
    WorkspaceGitLane,
    /// Minted from the provider/registry inspector lane.
    WorkspaceProviderLane,
    /// Minted from the support/diagnostics export lane.
    WorkspaceSupportLane,
    /// Minted from the headless CLI surface.
    HeadlessCliSurface,
}

impl HandoffOriginClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceStartCenter => "workspace_start_center",
            Self::WorkspaceReviewLane => "workspace_review_lane",
            Self::WorkspaceRuntimeLane => "workspace_runtime_lane",
            Self::WorkspaceGitLane => "workspace_git_lane",
            Self::WorkspaceProviderLane => "workspace_provider_lane",
            Self::WorkspaceSupportLane => "workspace_support_lane",
            Self::HeadlessCliSurface => "headless_cli_surface",
        }
    }
}

/// Destination class for one handoff packet. Mirrors the closed
/// destination-class vocabulary frozen on the integration browser-handoff
/// packet schema, narrowed to the shapes the alpha lane admits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDestinationClass {
    /// Code-host web surface (pull-request, branch view, comment).
    CodeHostWeb,
    /// Issue-tracker / work-item web surface.
    IssueTrackerWeb,
    /// CI provider web surface (pipeline, check run, log, artifact).
    CiProviderWeb,
    /// Identity provider web surface (re-auth, step-up).
    IdentityProviderWeb,
    /// Provider admin / managed admin web surface.
    ManagedAdminWeb,
    /// External documentation, runbook, or portal acceptance surface.
    DocsOrPortalWeb,
}

impl HandoffDestinationClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeHostWeb => "code_host_web",
            Self::IssueTrackerWeb => "issue_tracker_web",
            Self::CiProviderWeb => "ci_provider_web",
            Self::IdentityProviderWeb => "identity_provider_web",
            Self::ManagedAdminWeb => "managed_admin_web",
            Self::DocsOrPortalWeb => "docs_or_portal_web",
        }
    }
}

/// Intended follow-up action the user is asked to take when they return to
/// product scope. A packet without a typed follow-up is refused at mint
/// time; an "Open in browser" link with no return contract is forbidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffFollowUpActionClass {
    /// Return to the local-draft authoring view for the same object row.
    ReturnToLocalDraftAuthoring,
    /// Return to the publish-later queue item for the same object row.
    ReturnToPublishLaterQueueItem,
    /// Return to the inspect-only view of the same object row.
    ReturnToInspectOnlyView,
    /// Return to the review-pack anchor that minted the packet.
    ReturnToReviewAnchor,
    /// Return to the run/task-graph anchor that minted the packet.
    ReturnToRunOrTaskAnchor,
    /// Return to the change-lineage / publish-readiness view.
    ReturnToChangeLineageView,
    /// Return to the authority-repair (reauth, rescope) prompt.
    ReturnToAuthorityRepair,
    /// Return to a truthful placeholder when the authoritative local object
    /// is no longer available (e.g. revoked, beyond retention).
    ReturnToTruthfulPlaceholder,
}

impl HandoffFollowUpActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReturnToLocalDraftAuthoring => "return_to_local_draft_authoring",
            Self::ReturnToPublishLaterQueueItem => "return_to_publish_later_queue_item",
            Self::ReturnToInspectOnlyView => "return_to_inspect_only_view",
            Self::ReturnToReviewAnchor => "return_to_review_anchor",
            Self::ReturnToRunOrTaskAnchor => "return_to_run_or_task_anchor",
            Self::ReturnToChangeLineageView => "return_to_change_lineage_view",
            Self::ReturnToAuthorityRepair => "return_to_authority_repair",
            Self::ReturnToTruthfulPlaceholder => "return_to_truthful_placeholder",
        }
    }
}

/// Packet-state vocabulary. A packet never collapses into a generic
/// "closed" state — silent transition between states is forbidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffPacketStateClass {
    /// Minted and waiting for the user to confirm departure.
    MintedAwaitingConfirmation,
    /// User confirmed departure; packet pending launch.
    UserConfirmedPendingLaunch,
    /// Browser was launched and Aureline is awaiting return.
    LaunchedAwaitingReturn,
    /// User returned; follow-up action resolved to the authoritative
    /// local object.
    ReturnedAuthoritativeLocalObject,
    /// User returned; follow-up action resolved to a truthful placeholder
    /// because the authoritative local object is no longer available.
    ReturnedTruthfulPlaceholder,
    /// User returned; provider observation supersedes the local model
    /// (`published_observed_authoritative`).
    ReturnedProviderObservedAuthoritative,
    /// User cancelled the handoff before launch or before return.
    ReturnedUserCancelled,
    /// Callback failed validation (origin mismatch, replay outside
    /// window, etc.).
    ReturnedCallbackInvalid,
    /// Provider revoked authority before return.
    ReturnedAuthorityRevoked,
    /// Packet expired before the user returned.
    ExpiredUnused,
}

impl HandoffPacketStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MintedAwaitingConfirmation => "minted_awaiting_confirmation",
            Self::UserConfirmedPendingLaunch => "user_confirmed_pending_launch",
            Self::LaunchedAwaitingReturn => "launched_awaiting_return",
            Self::ReturnedAuthoritativeLocalObject => "returned_authoritative_local_object",
            Self::ReturnedTruthfulPlaceholder => "returned_truthful_placeholder",
            Self::ReturnedProviderObservedAuthoritative => {
                "returned_provider_observed_authoritative"
            }
            Self::ReturnedUserCancelled => "returned_user_cancelled",
            Self::ReturnedCallbackInvalid => "returned_callback_invalid",
            Self::ReturnedAuthorityRevoked => "returned_authority_revoked",
            Self::ExpiredUnused => "expired_unused",
        }
    }

    /// True when this state represents a returned packet (i.e. the user
    /// has come back to product scope, successfully or otherwise).
    pub const fn is_returned(self) -> bool {
        matches!(
            self,
            Self::ReturnedAuthoritativeLocalObject
                | Self::ReturnedTruthfulPlaceholder
                | Self::ReturnedProviderObservedAuthoritative
                | Self::ReturnedUserCancelled
                | Self::ReturnedCallbackInvalid
                | Self::ReturnedAuthorityRevoked
        )
    }

    /// True when the state requires a return-summary to be populated.
    pub const fn requires_return_summary(self) -> bool {
        matches!(
            self,
            Self::ReturnedAuthoritativeLocalObject
                | Self::ReturnedTruthfulPlaceholder
                | Self::ReturnedProviderObservedAuthoritative
                | Self::ReturnedUserCancelled
                | Self::ReturnedCallbackInvalid
                | Self::ReturnedAuthorityRevoked
        )
    }
}

/// Lifecycle state for one import session. Import sessions are how
/// reopened provider flows preserve continuity: the same opaque session id
/// is replayed across reconnect attempts, and its state reads from the
/// same vocabulary as the provider-object support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportSessionStateClass {
    /// Session is pending its first observation; no provider truth seen.
    PendingFirstObservation,
    /// Session has observed fresh provider truth.
    ObservedFresh,
    /// Session is stale within the freshness window.
    StaleWithinWindow,
    /// Session has expired beyond the freshness window.
    ExpiredBeyondWindow,
    /// Session was revoked or disconnected by the provider.
    RevokedOrDisconnected,
    /// Session was replaced by a newer session and is retained for audit
    /// only.
    ReplacedByNewerSession,
    /// Session was abandoned by the user without a return.
    AbandonedByUser,
}

impl ImportSessionStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingFirstObservation => "pending_first_observation",
            Self::ObservedFresh => "observed_fresh",
            Self::StaleWithinWindow => "stale_within_window",
            Self::ExpiredBeyondWindow => "expired_beyond_window",
            Self::RevokedOrDisconnected => "revoked_or_disconnected",
            Self::ReplacedByNewerSession => "replaced_by_newer_session",
            Self::AbandonedByUser => "abandoned_by_user",
        }
    }

    /// True when the state requires a continuity observation to be
    /// attached.
    pub const fn requires_continuity_observation(self) -> bool {
        matches!(
            self,
            Self::StaleWithinWindow
                | Self::ExpiredBeyondWindow
                | Self::RevokedOrDisconnected
                | Self::AbandonedByUser
        )
    }
}

/// Outcome class for one provider reconnect flow. A reconnect always
/// names a typed outcome — the workflow never silently drops to a generic
/// "session lost" state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReconnectOutcomeClass {
    /// Reconnect resolved to the authoritative local object row. The
    /// `restored_object_row_ref` is required.
    RestoredAuthoritativeLocalObject,
    /// Reconnect resolved to a truthful placeholder. The
    /// `placeholder_kind` is required.
    RestoredTruthfulPlaceholder,
    /// Reconnect deferred the action into the publish-later queue.
    DeferredToPublishLaterQueue,
    /// Reconnect re-offered the browser handoff for a follow-up packet.
    BrowserHandoffOfferedAgain,
    /// Reconnect was denied because the actor identity is no longer
    /// known.
    DeniedUnknownActor,
    /// Reconnect was denied because the policy or trust posture changed.
    DeniedPolicyOrTrustMismatch,
    /// Reconnect is pending re-authentication.
    PendingReauth,
    /// Reconnect is pending rescope.
    PendingRescope,
    /// Reconnect is pending reapproval.
    PendingReapproval,
}

impl ReconnectOutcomeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoredAuthoritativeLocalObject => "restored_authoritative_local_object",
            Self::RestoredTruthfulPlaceholder => "restored_truthful_placeholder",
            Self::DeferredToPublishLaterQueue => "deferred_to_publish_later_queue",
            Self::BrowserHandoffOfferedAgain => "browser_handoff_offered_again",
            Self::DeniedUnknownActor => "denied_unknown_actor",
            Self::DeniedPolicyOrTrustMismatch => "denied_policy_or_trust_mismatch",
            Self::PendingReauth => "pending_reauth",
            Self::PendingRescope => "pending_rescope",
            Self::PendingReapproval => "pending_reapproval",
        }
    }

    /// True when the outcome must cite an `restored_object_row_ref`.
    pub const fn requires_restored_object(self) -> bool {
        matches!(self, Self::RestoredAuthoritativeLocalObject)
    }

    /// True when the outcome must cite a `placeholder_kind`.
    pub const fn requires_placeholder_kind(self) -> bool {
        matches!(self, Self::RestoredTruthfulPlaceholder)
    }

    /// True when the outcome must cite a `publish_later_queue_item_ref`.
    pub const fn requires_publish_later_ref(self) -> bool {
        matches!(self, Self::DeferredToPublishLaterQueue)
    }

    /// True when the outcome must cite a `browser_handoff_packet_ref`.
    pub const fn requires_followup_packet_ref(self) -> bool {
        matches!(self, Self::BrowserHandoffOfferedAgain)
    }

    /// True when the outcome closes mutation authority without restoring
    /// a local row.
    pub const fn closes_mutation_authority(self) -> bool {
        matches!(
            self,
            Self::DeniedUnknownActor
                | Self::DeniedPolicyOrTrustMismatch
                | Self::PendingReauth
                | Self::PendingRescope
                | Self::PendingReapproval
        )
    }
}

/// Placeholder class for reconnect flows that cannot restore an
/// authoritative local object. A reconnect never silently drops to
/// `unknown` — the placeholder names exactly *why* the authoritative row
/// could not be restored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffPlaceholderClass {
    /// The local object was evicted beyond retention.
    LocalObjectEvictedBeyondRetention,
    /// The local object was superseded by a newer import session.
    SupersededByNewerImportSession,
    /// The local object was revoked by the provider and the local body
    /// was wiped.
    RevokedAndLocalBodyWiped,
    /// The local object never existed (the packet was minted for an
    /// inspect-only target with no draft).
    InspectOnlyTargetWithoutDraft,
    /// The local object exists but the workspace trust posture changed
    /// and the row is locked.
    LockedByTrustPostureChange,
}

impl HandoffPlaceholderClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalObjectEvictedBeyondRetention => "local_object_evicted_beyond_retention",
            Self::SupersededByNewerImportSession => "superseded_by_newer_import_session",
            Self::RevokedAndLocalBodyWiped => "revoked_and_local_body_wiped",
            Self::InspectOnlyTargetWithoutDraft => "inspect_only_target_without_draft",
            Self::LockedByTrustPostureChange => "locked_by_trust_posture_change",
        }
    }
}

/// Redaction-safe origin metadata. Names the workspace lane that minted
/// the packet, plus opaque host/workspace/session refs so support and
/// audit can correlate without raw URLs or raw token material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffOriginDisclosure {
    /// Origin class.
    pub origin_class: HandoffOriginClass,
    /// Opaque host identity ref (e.g. `host.identity.workspace.primary`).
    pub host_identity_ref: String,
    /// Opaque workspace id ref.
    pub workspace_id_ref: String,
    /// Opaque execution-context id ref.
    pub execution_context_id_ref: String,
}

/// Redaction-safe destination metadata. Names the destination class and
/// the canonical host / tenant scope. Raw URLs and raw query strings are
/// forbidden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffDestinationDisclosure {
    /// Destination class.
    pub destination_class: HandoffDestinationClass,
    /// Opaque canonical host ref.
    pub canonical_host_ref: String,
    /// Opaque tenant / org scope ref.
    pub tenant_or_org_scope_ref: String,
    /// Optional opaque environment ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub environment_ref: Option<String>,
    /// Provider family the destination belongs to.
    pub provider_family: ProviderFamily,
}

/// References to upstream schema files consumed by this page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBrowserHandoffAlphaContractRefs {
    /// Existing provider-object alpha schema reference.
    pub provider_object_model_alpha_schema_ref: String,
    /// Existing browser-handoff packet schema reference (integration
    /// boundary).
    pub browser_handoff_packet_schema_ref: String,
    /// Existing publish-later queue alpha schema reference.
    pub publish_later_queue_alpha_schema_ref: String,
    /// Existing approval-ticket alpha schema reference.
    pub approval_ticket_alpha_schema_ref: String,
    /// Existing connected-provider registry schema reference.
    pub connected_provider_registry_schema_ref: String,
}

impl ProviderBrowserHandoffAlphaContractRefs {
    fn all_refs(&self) -> [&str; 5] {
        [
            &self.provider_object_model_alpha_schema_ref,
            &self.browser_handoff_packet_schema_ref,
            &self.publish_later_queue_alpha_schema_ref,
            &self.approval_ticket_alpha_schema_ref,
            &self.connected_provider_registry_schema_ref,
        ]
    }
}

/// One typed browser-handoff packet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque packet id.
    pub packet_id: String,
    /// Reviewable label safe for support export.
    pub display_label: String,
    /// Origin metadata for the packet.
    pub origin: HandoffOriginDisclosure,
    /// Destination metadata for the packet.
    pub destination: HandoffDestinationDisclosure,
    /// Opaque ref to the provider-object row this packet targets.
    pub provider_object_row_ref: String,
    /// Opaque target-side object id (e.g. `pr.4013`,
    /// `issue.aur.104`). Distinct from the local row id; this is what
    /// the destination provider knows the object as.
    pub provider_side_object_id: String,
    /// Typed intended follow-up action the user is asked to take when
    /// they return to product scope.
    pub intended_follow_up_action: HandoffFollowUpActionClass,
    /// Reviewable sentence quoting the follow-up action in human-legible
    /// form.
    pub follow_up_summary: String,
    /// Current packet state.
    pub packet_state: HandoffPacketStateClass,
    /// Opaque ref to the integration-level browser-handoff packet record
    /// once minted (empty string before launch).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub integration_packet_ref: Option<String>,
    /// Opaque ref to a publish-later queue item when the follow-up
    /// defers to publish-later.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Opaque ref to an approval ticket admitting the mutation when the
    /// destination is in-product (publish-now) after return.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Opaque ref to an import session this packet binds to (set when
    /// the packet resumes an existing reconnect/import lineage).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub import_session_ref: Option<String>,
    /// Optional return summary, required when the packet is in a
    /// returned state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_summary: Option<String>,
    /// Optional placeholder kind, required when the packet returned to a
    /// truthful placeholder.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_kind: Option<HandoffPlaceholderClass>,
    /// Audit event refs minted by this packet on the provider-handoff
    /// stream.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Redaction posture for the packet.
    pub redaction_class: RedactionClass,
    /// Guardrail: packet does not carry a raw URL.
    pub raw_url_present: bool,
    /// Guardrail: packet does not carry raw token material.
    pub raw_token_material_present: bool,
    /// Guardrail: packet does not carry raw provider payload.
    pub raw_provider_payload_present: bool,
    /// Guardrail: packet did not silently widen mutation authority.
    pub silent_authority_widening_taken: bool,
    /// Export-safe summary of the packet.
    pub support_export_summary: String,
    /// Timestamp at which the packet was minted.
    pub minted_at: String,
    /// Timestamp at which the packet expires (replay window).
    pub replay_expires_at: String,
}

/// One import session record. Import sessions are how reopened provider
/// flows preserve continuity across reconnect attempts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportSessionRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque import-session id.
    pub import_session_id: String,
    /// Reviewable label safe for support export.
    pub display_label: String,
    /// Opaque ref to the provider-object row this session feeds.
    pub provider_object_row_ref: String,
    /// Provider family the session belongs to.
    pub provider_family: ProviderFamily,
    /// Lifecycle state of the session.
    pub session_state: ImportSessionStateClass,
    /// Freshness truth observed for the session.
    pub freshness: FreshnessTruth,
    /// Optional opaque ref to the prior session this one supersedes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub superseded_session_ref: Option<String>,
    /// Audit event refs minted by this session.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
    /// Guardrail: session does not carry raw provider payload.
    pub raw_provider_payload_present: bool,
    /// Guardrail: session preserves local editing on the bound row.
    pub local_editing_preserved: bool,
    /// Export-safe summary.
    pub support_export_summary: String,
    /// Timestamp at which the session was first opened.
    pub opened_at: String,
    /// Timestamp at which the session was last observed.
    pub last_observed_at: String,
}

/// One provider reconnect flow. Names the typed outcome plus the typed
/// restored object row or placeholder. A reconnect never silently
/// drops.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderReconnectFlow {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque reconnect-flow id.
    pub reconnect_flow_id: String,
    /// Reviewable label safe for support export.
    pub display_label: String,
    /// Opaque ref to the packet that initiated this reconnect, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originating_packet_ref: Option<String>,
    /// Opaque ref to the import session that backs this reconnect.
    pub import_session_ref: String,
    /// Opaque ref to the provider-object row this reconnect resolves
    /// against.
    pub bound_object_row_ref: String,
    /// Typed outcome of the reconnect.
    pub outcome: ReconnectOutcomeClass,
    /// Opaque ref to the restored authoritative local object row.
    /// Required when `outcome` is `restored_authoritative_local_object`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub restored_object_row_ref: Option<String>,
    /// Typed placeholder kind. Required when `outcome` is
    /// `restored_truthful_placeholder`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub placeholder_kind: Option<HandoffPlaceholderClass>,
    /// Opaque ref to a publish-later queue item the reconnect deferred
    /// to. Required when `outcome` is `deferred_to_publish_later_queue`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Opaque ref to a follow-up browser-handoff packet. Required when
    /// `outcome` is `browser_handoff_offered_again`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_up_packet_ref: Option<String>,
    /// Reviewable sentence explaining the outcome.
    pub outcome_summary: String,
    /// Audit event refs minted by this reconnect.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Redaction posture.
    pub redaction_class: RedactionClass,
    /// Guardrail: flow preserves local editing on the bound row.
    pub local_editing_preserved: bool,
    /// Guardrail: flow did not silently widen mutation authority.
    pub silent_authority_widening_taken: bool,
    /// Timestamp at which the reconnect was observed.
    pub observed_at: String,
}

/// One continuity observation attached to a packet, import session, or
/// reconnect flow. Reuses the closed vocabulary already used by the
/// provider-object support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffContinuityObservation {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for this record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque observation id.
    pub observation_id: String,
    /// Opaque ref to the bound packet, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_packet_ref: Option<String>,
    /// Opaque ref to the bound import session, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_import_session_ref: Option<String>,
    /// Opaque ref to the bound reconnect flow, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_reconnect_flow_ref: Option<String>,
    /// Typed observation class (shared vocabulary with the provider-object
    /// support export).
    pub observation_class: ContinuityObservationClass,
    /// Retained capability the local model preserves under this
    /// observation.
    pub retained_capability_class: RetainedCapabilityClass,
    /// Typed degraded action the row still offers.
    pub degraded_action: DegradedActionClass,
    /// Export-safe rationale.
    pub rationale_summary: String,
    /// Timestamp at which the observation was made.
    pub observed_at: String,
    /// Guardrail: observation did not silently widen mutation authority.
    pub silent_mutation_authority_widened: bool,
    /// Guardrail: local editing preserved when the row admits it.
    pub local_editing_preserved: bool,
}

/// Fixture metadata used by protected cases.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBrowserHandoffAlphaFixtureMetadata {
    /// Short fixture name.
    pub name: String,
    /// Reviewer-safe scenario summary.
    pub scenario: String,
}

/// One alpha page: packets + import sessions + reconnect flows +
/// continuity observations under one packet id.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBrowserHandoffAlphaPage {
    /// Optional fixture metadata for validation lanes.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<ProviderBrowserHandoffAlphaFixtureMetadata>,
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version for the page.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque page id.
    pub page_id: String,
    /// Upstream contracts this page consumes by reference.
    pub contract_refs: ProviderBrowserHandoffAlphaContractRefs,
    /// Handoff packets in this page.
    pub packets: Vec<BrowserHandoffPacket>,
    /// Import sessions in this page.
    pub import_sessions: Vec<ImportSessionRecord>,
    /// Reconnect flows in this page.
    pub reconnect_flows: Vec<ProviderReconnectFlow>,
    /// Continuity observations attached to packets, sessions, or flows.
    #[serde(default)]
    pub continuity_observations: Vec<HandoffContinuityObservation>,
    /// Export-safe page summary.
    pub support_export_summary: String,
}

impl ProviderBrowserHandoffAlphaPage {
    /// Validate the page against alpha invariants.
    pub fn validate(&self) -> ProviderBrowserHandoffAlphaValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Build a redaction-safe support export projection.
    pub fn support_export_projection(&self) -> ProviderBrowserHandoffAlphaSupportExport {
        let packet_summaries = self
            .packets
            .iter()
            .map(|packet| BrowserHandoffPacketSummary {
                packet_id: packet.packet_id.clone(),
                display_label: packet.display_label.clone(),
                origin_class: packet.origin.origin_class,
                destination_class: packet.destination.destination_class,
                provider_object_row_ref: packet.provider_object_row_ref.clone(),
                provider_side_object_id: packet.provider_side_object_id.clone(),
                intended_follow_up_action: packet.intended_follow_up_action,
                packet_state: packet.packet_state,
                follow_up_summary: packet.follow_up_summary.clone(),
                return_summary: packet.return_summary.clone(),
                placeholder_kind: packet.placeholder_kind,
                support_export_summary: packet.support_export_summary.clone(),
            })
            .collect();
        let import_session_summaries = self
            .import_sessions
            .iter()
            .map(|session| ImportSessionSummary {
                import_session_id: session.import_session_id.clone(),
                display_label: session.display_label.clone(),
                provider_object_row_ref: session.provider_object_row_ref.clone(),
                provider_family: session.provider_family,
                session_state: session.session_state,
                freshness_class: session.freshness.freshness_class,
                support_export_summary: session.support_export_summary.clone(),
            })
            .collect();
        let reconnect_summaries = self
            .reconnect_flows
            .iter()
            .map(|flow| ProviderReconnectFlowSummary {
                reconnect_flow_id: flow.reconnect_flow_id.clone(),
                display_label: flow.display_label.clone(),
                bound_object_row_ref: flow.bound_object_row_ref.clone(),
                import_session_ref: flow.import_session_ref.clone(),
                outcome: flow.outcome,
                placeholder_kind: flow.placeholder_kind,
                outcome_summary: flow.outcome_summary.clone(),
            })
            .collect();
        let continuity_summaries = self
            .continuity_observations
            .iter()
            .map(|observation| HandoffContinuityObservationSummary {
                observation_id: observation.observation_id.clone(),
                bound_packet_ref: observation.bound_packet_ref.clone(),
                bound_import_session_ref: observation.bound_import_session_ref.clone(),
                bound_reconnect_flow_ref: observation.bound_reconnect_flow_ref.clone(),
                observation_class: observation.observation_class,
                retained_capability_class: observation.retained_capability_class,
                degraded_action: observation.degraded_action,
                rationale_summary: observation.rationale_summary.clone(),
            })
            .collect();
        ProviderBrowserHandoffAlphaSupportExport {
            record_kind: PROVIDER_BROWSER_HANDOFF_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            packet_summaries,
            import_session_summaries,
            reconnect_summaries,
            continuity_summaries,
            redaction_class: RedactionClass::MetadataSafeDefault,
        }
    }
}

/// Validation report emitted by the alpha validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBrowserHandoffAlphaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version validated.
    pub schema_version: u32,
    /// Page id under validation.
    pub page_id: String,
    /// Whether no error-severity checks failed.
    pub passed: bool,
    /// Coverage observed while validating the page.
    pub coverage: ProviderBrowserHandoffAlphaCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<ProviderBrowserHandoffAlphaFinding>,
}

/// Coverage observed during alpha validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ProviderBrowserHandoffAlphaCoverage {
    /// Origin classes covered by packets.
    pub origin_classes: BTreeSet<HandoffOriginClass>,
    /// Destination classes covered by packets.
    pub destination_classes: BTreeSet<HandoffDestinationClass>,
    /// Follow-up action classes covered by packets.
    pub follow_up_action_classes: BTreeSet<HandoffFollowUpActionClass>,
    /// Packet states covered.
    pub packet_states: BTreeSet<HandoffPacketStateClass>,
    /// Import-session states covered.
    pub import_session_states: BTreeSet<ImportSessionStateClass>,
    /// Reconnect outcome classes covered.
    pub reconnect_outcomes: BTreeSet<ReconnectOutcomeClass>,
    /// Continuity observation classes covered.
    pub continuity_observation_classes: BTreeSet<ContinuityObservationClass>,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBrowserHandoffAlphaFinding {
    /// Severity.
    pub severity: ProviderBrowserHandoffAlphaFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe message.
    pub message: String,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderBrowserHandoffAlphaFindingSeverity {
    /// Error that blocks the page.
    Error,
    /// Warning that keeps the page reviewable but visibly degraded.
    Warning,
}

/// Redaction-safe support export projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderBrowserHandoffAlphaSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Page id.
    pub page_id: String,
    /// Packet summaries safe for support bundles.
    pub packet_summaries: Vec<BrowserHandoffPacketSummary>,
    /// Import-session summaries safe for support bundles.
    pub import_session_summaries: Vec<ImportSessionSummary>,
    /// Reconnect-flow summaries safe for support bundles.
    pub reconnect_summaries: Vec<ProviderReconnectFlowSummary>,
    /// Continuity-observation summaries safe for support bundles.
    pub continuity_summaries: Vec<HandoffContinuityObservationSummary>,
    /// Redaction posture for the projection.
    pub redaction_class: RedactionClass,
}

/// Redaction-safe summary of one browser-handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserHandoffPacketSummary {
    /// Packet id.
    pub packet_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Origin class.
    pub origin_class: HandoffOriginClass,
    /// Destination class.
    pub destination_class: HandoffDestinationClass,
    /// Provider-object row ref.
    pub provider_object_row_ref: String,
    /// Provider-side object id (e.g. `pr.4013`).
    pub provider_side_object_id: String,
    /// Intended follow-up action.
    pub intended_follow_up_action: HandoffFollowUpActionClass,
    /// Current packet state.
    pub packet_state: HandoffPacketStateClass,
    /// Reviewable follow-up summary.
    pub follow_up_summary: String,
    /// Optional return summary.
    pub return_summary: Option<String>,
    /// Optional placeholder kind.
    pub placeholder_kind: Option<HandoffPlaceholderClass>,
    /// Support-safe summary.
    pub support_export_summary: String,
}

/// Redaction-safe summary of one import session.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImportSessionSummary {
    /// Import-session id.
    pub import_session_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Provider-object row ref the session feeds.
    pub provider_object_row_ref: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Session state.
    pub session_state: ImportSessionStateClass,
    /// Freshness class.
    pub freshness_class: FreshnessLabel,
    /// Support-safe summary.
    pub support_export_summary: String,
}

/// Redaction-safe summary of one provider reconnect flow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderReconnectFlowSummary {
    /// Reconnect-flow id.
    pub reconnect_flow_id: String,
    /// Reviewable label.
    pub display_label: String,
    /// Bound object row ref.
    pub bound_object_row_ref: String,
    /// Import-session ref the reconnect resolves under.
    pub import_session_ref: String,
    /// Outcome class.
    pub outcome: ReconnectOutcomeClass,
    /// Optional placeholder kind.
    pub placeholder_kind: Option<HandoffPlaceholderClass>,
    /// Reviewable outcome summary.
    pub outcome_summary: String,
}

/// Redaction-safe summary of one continuity observation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffContinuityObservationSummary {
    /// Observation id.
    pub observation_id: String,
    /// Bound packet ref.
    pub bound_packet_ref: Option<String>,
    /// Bound import-session ref.
    pub bound_import_session_ref: Option<String>,
    /// Bound reconnect-flow ref.
    pub bound_reconnect_flow_ref: Option<String>,
    /// Observation class.
    pub observation_class: ContinuityObservationClass,
    /// Retained capability class.
    pub retained_capability_class: RetainedCapabilityClass,
    /// Degraded action class.
    pub degraded_action: DegradedActionClass,
    /// Rationale summary.
    pub rationale_summary: String,
}

struct Validator<'a> {
    page: &'a ProviderBrowserHandoffAlphaPage,
    packet_ids: BTreeSet<&'a str>,
    import_session_ids: BTreeSet<&'a str>,
    reconnect_flow_ids: BTreeSet<&'a str>,
    observation_ids: BTreeSet<&'a str>,
    coverage: ProviderBrowserHandoffAlphaCoverage,
    findings: Vec<ProviderBrowserHandoffAlphaFinding>,
}

impl<'a> Validator<'a> {
    fn new(page: &'a ProviderBrowserHandoffAlphaPage) -> Self {
        Self {
            page,
            packet_ids: BTreeSet::new(),
            import_session_ids: BTreeSet::new(),
            reconnect_flow_ids: BTreeSet::new(),
            observation_ids: BTreeSet::new(),
            coverage: ProviderBrowserHandoffAlphaCoverage::default(),
            findings: Vec::new(),
        }
    }

    fn run(&mut self) {
        self.validate_page_header();
        self.validate_packets();
        self.validate_import_sessions();
        self.validate_reconnect_flows();
        self.validate_continuity_observations();
        self.validate_required_coverage();
    }

    fn finish(self) -> ProviderBrowserHandoffAlphaValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != ProviderBrowserHandoffAlphaFindingSeverity::Error);
        ProviderBrowserHandoffAlphaValidationReport {
            record_kind: PROVIDER_BROWSER_HANDOFF_ALPHA_VALIDATION_REPORT_RECORD_KIND.to_string(),
            schema_version: PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
            page_id: self.page.page_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn validate_page_header(&mut self) {
        let page = self.page;
        self.expect(
            page.record_kind == PROVIDER_BROWSER_HANDOFF_ALPHA_PAGE_RECORD_KIND,
            "browser_handoff_alpha.page_record_kind",
            "page.record_kind must be provider_browser_handoff_alpha_page_record",
        );
        self.expect(
            page.schema_version == PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
            "browser_handoff_alpha.page_schema_version",
            "page.schema_version must match the crate constant",
        );
        self.expect(
            page.shared_contract_ref == PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF,
            "browser_handoff_alpha.page_shared_contract_ref",
            "page.shared_contract_ref must match the shared contract id",
        );
        self.expect(
            !page.page_id.trim().is_empty(),
            "browser_handoff_alpha.page_id_missing",
            "page.page_id must be non-empty",
        );
        self.expect(
            !page.support_export_summary.trim().is_empty(),
            "browser_handoff_alpha.page_support_summary_missing",
            "page.support_export_summary must be non-empty",
        );
        for contract_ref in page.contract_refs.all_refs() {
            self.expect(
                !contract_ref.trim().is_empty(),
                "browser_handoff_alpha.contract_ref_missing",
                "every consumed upstream contract ref must be non-empty",
            );
        }
        self.expect(
            !page.packets.is_empty(),
            "browser_handoff_alpha.packets_missing",
            "page must contain at least one browser-handoff packet",
        );
    }

    fn validate_packets(&mut self) {
        for packet in &self.page.packets {
            self.expect(
                packet.record_kind == PROVIDER_BROWSER_HANDOFF_ALPHA_PACKET_RECORD_KIND,
                "browser_handoff_alpha.packet_record_kind",
                "packet.record_kind is wrong",
            );
            self.expect(
                packet.schema_version == PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
                "browser_handoff_alpha.packet_schema_version",
                "packet.schema_version is wrong",
            );
            self.expect(
                packet.shared_contract_ref == PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF,
                "browser_handoff_alpha.packet_shared_contract_ref",
                "packet.shared_contract_ref must match the shared contract id",
            );
            let id_unique = self.packet_ids.insert(&packet.packet_id);
            self.expect(
                id_unique,
                "browser_handoff_alpha.packet_duplicate",
                "packet_id values must be unique within a page",
            );
            self.expect(
                !packet.display_label.trim().is_empty(),
                "browser_handoff_alpha.packet_display_label_missing",
                "packet.display_label must be non-empty",
            );
            self.expect(
                !packet.origin.host_identity_ref.trim().is_empty()
                    && !packet.origin.workspace_id_ref.trim().is_empty()
                    && !packet.origin.execution_context_id_ref.trim().is_empty(),
                "browser_handoff_alpha.packet_origin_incomplete",
                "packet.origin must name host_identity, workspace_id, and execution_context refs",
            );
            self.expect(
                !packet.destination.canonical_host_ref.trim().is_empty()
                    && !packet.destination.tenant_or_org_scope_ref.trim().is_empty(),
                "browser_handoff_alpha.packet_destination_incomplete",
                "packet.destination must name canonical_host and tenant_or_org_scope refs",
            );
            self.expect(
                !packet.provider_object_row_ref.trim().is_empty(),
                "browser_handoff_alpha.packet_object_row_ref_missing",
                "packet.provider_object_row_ref must be non-empty",
            );
            self.expect(
                !packet.provider_side_object_id.trim().is_empty(),
                "browser_handoff_alpha.packet_provider_side_object_id_missing",
                "packet.provider_side_object_id must be non-empty",
            );
            self.expect(
                !packet.follow_up_summary.trim().is_empty(),
                "browser_handoff_alpha.packet_follow_up_summary_missing",
                "packet.follow_up_summary must be non-empty",
            );
            self.expect(
                !packet.support_export_summary.trim().is_empty(),
                "browser_handoff_alpha.packet_support_summary_missing",
                "packet.support_export_summary must be non-empty",
            );
            self.expect(
                !packet.minted_at.trim().is_empty() && !packet.replay_expires_at.trim().is_empty(),
                "browser_handoff_alpha.packet_timestamps_missing",
                "packet must name minted_at and replay_expires_at",
            );
            self.expect(
                !packet.raw_url_present,
                "browser_handoff_alpha.packet_raw_url_present",
                "packet.raw_url_present must be false",
            );
            self.expect(
                !packet.raw_token_material_present,
                "browser_handoff_alpha.packet_raw_token_present",
                "packet.raw_token_material_present must be false",
            );
            self.expect(
                !packet.raw_provider_payload_present,
                "browser_handoff_alpha.packet_raw_provider_payload_present",
                "packet.raw_provider_payload_present must be false",
            );
            self.expect(
                !packet.silent_authority_widening_taken,
                "browser_handoff_alpha.packet_silent_authority_widening",
                "packet.silent_authority_widening_taken must be false",
            );

            self.validate_packet_state_refs(packet);

            self.coverage
                .origin_classes
                .insert(packet.origin.origin_class);
            self.coverage
                .destination_classes
                .insert(packet.destination.destination_class);
            self.coverage
                .follow_up_action_classes
                .insert(packet.intended_follow_up_action);
            self.coverage.packet_states.insert(packet.packet_state);
        }
    }

    fn validate_packet_state_refs(&mut self, packet: &BrowserHandoffPacket) {
        let non_empty =
            |opt: &Option<String>| opt.as_deref().is_some_and(|value| !value.trim().is_empty());

        if packet.packet_state.requires_return_summary() {
            self.expect(
                non_empty(&packet.return_summary),
                "browser_handoff_alpha.packet_return_summary_missing",
                "returned packets must cite a return_summary",
            );
        }

        match packet.packet_state {
            HandoffPacketStateClass::ReturnedAuthoritativeLocalObject => {
                self.expect(
                    packet.placeholder_kind.is_none(),
                    "browser_handoff_alpha.packet_placeholder_conflict",
                    "returned_authoritative_local_object must not cite a placeholder_kind",
                );
            }
            HandoffPacketStateClass::ReturnedTruthfulPlaceholder => {
                self.expect(
                    packet.placeholder_kind.is_some(),
                    "browser_handoff_alpha.packet_placeholder_missing",
                    "returned_truthful_placeholder must cite a placeholder_kind",
                );
            }
            _ => {}
        }

        if matches!(
            packet.intended_follow_up_action,
            HandoffFollowUpActionClass::ReturnToTruthfulPlaceholder
        ) {
            self.expect(
                matches!(
                    packet.packet_state,
                    HandoffPacketStateClass::MintedAwaitingConfirmation
                        | HandoffPacketStateClass::UserConfirmedPendingLaunch
                        | HandoffPacketStateClass::LaunchedAwaitingReturn
                        | HandoffPacketStateClass::ReturnedTruthfulPlaceholder
                        | HandoffPacketStateClass::ExpiredUnused
                        | HandoffPacketStateClass::ReturnedUserCancelled
                        | HandoffPacketStateClass::ReturnedAuthorityRevoked
                        | HandoffPacketStateClass::ReturnedCallbackInvalid
                ),
                "browser_handoff_alpha.packet_placeholder_followup_state",
                "return_to_truthful_placeholder follow-ups cannot claim an authoritative-local \
                 returned state",
            );
        }

        if matches!(
            packet.intended_follow_up_action,
            HandoffFollowUpActionClass::ReturnToPublishLaterQueueItem
        ) {
            self.expect(
                non_empty(&packet.publish_later_queue_item_ref),
                "browser_handoff_alpha.packet_publish_later_ref_missing",
                "return_to_publish_later_queue_item follow-ups must cite a \
                 publish_later_queue_item_ref",
            );
        }

        if matches!(
            packet.intended_follow_up_action,
            HandoffFollowUpActionClass::ReturnToAuthorityRepair
        ) {
            self.expect(
                matches!(
                    packet.destination.destination_class,
                    HandoffDestinationClass::IdentityProviderWeb
                        | HandoffDestinationClass::ManagedAdminWeb
                ),
                "browser_handoff_alpha.packet_authority_repair_destination",
                "return_to_authority_repair follow-ups must route to identity_provider_web \
                 or managed_admin_web",
            );
        }
    }

    fn validate_import_sessions(&mut self) {
        for session in &self.page.import_sessions {
            self.expect(
                session.record_kind == PROVIDER_BROWSER_HANDOFF_ALPHA_IMPORT_SESSION_RECORD_KIND,
                "browser_handoff_alpha.import_session_record_kind",
                "import_session.record_kind is wrong",
            );
            self.expect(
                session.schema_version == PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
                "browser_handoff_alpha.import_session_schema_version",
                "import_session.schema_version is wrong",
            );
            self.expect(
                session.shared_contract_ref == PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF,
                "browser_handoff_alpha.import_session_shared_contract_ref",
                "import_session.shared_contract_ref must match the shared contract id",
            );
            let unique = self.import_session_ids.insert(&session.import_session_id);
            self.expect(
                unique,
                "browser_handoff_alpha.import_session_duplicate",
                "import_session_id values must be unique within a page",
            );
            self.expect(
                !session.display_label.trim().is_empty(),
                "browser_handoff_alpha.import_session_display_label_missing",
                "import_session.display_label must be non-empty",
            );
            self.expect(
                !session.provider_object_row_ref.trim().is_empty(),
                "browser_handoff_alpha.import_session_object_row_ref_missing",
                "import_session.provider_object_row_ref must be non-empty",
            );
            self.expect(
                !session.support_export_summary.trim().is_empty(),
                "browser_handoff_alpha.import_session_support_summary_missing",
                "import_session.support_export_summary must be non-empty",
            );
            self.expect(
                !session.opened_at.trim().is_empty() && !session.last_observed_at.trim().is_empty(),
                "browser_handoff_alpha.import_session_timestamps_missing",
                "import_session must name opened_at and last_observed_at",
            );
            self.expect(
                !session.raw_provider_payload_present,
                "browser_handoff_alpha.import_session_raw_payload_present",
                "import_session.raw_provider_payload_present must be false",
            );
            self.expect(
                session.local_editing_preserved,
                "browser_handoff_alpha.import_session_local_editing_not_preserved",
                "import_session.local_editing_preserved must be true",
            );

            self.validate_session_freshness(session);

            self.coverage
                .import_session_states
                .insert(session.session_state);
        }
    }

    fn validate_session_freshness(&mut self, session: &ImportSessionRecord) {
        self.expect(
            !session.freshness.freshness_floor_ref.trim().is_empty(),
            "browser_handoff_alpha.import_session_freshness_floor_missing",
            "import_session.freshness must cite a freshness_floor_ref",
        );
        let degraded_freshness = matches!(
            session.freshness.freshness_class,
            FreshnessLabel::StaleWithinWindow
                | FreshnessLabel::ExpiredBeyondWindow
                | FreshnessLabel::NeverObserved
                | FreshnessLabel::RevokedOrDisconnected
        );
        if degraded_freshness {
            self.expect(
                session
                    .freshness
                    .degraded_reason
                    .as_deref()
                    .is_some_and(|reason| !reason.trim().is_empty()),
                "browser_handoff_alpha.import_session_freshness_degraded_reason_missing",
                "import_session degraded freshness must name a reason",
            );
        }
        match session.session_state {
            ImportSessionStateClass::ObservedFresh => self.expect(
                matches!(session.freshness.freshness_class, FreshnessLabel::Fresh),
                "browser_handoff_alpha.import_session_state_freshness_drift",
                "observed_fresh sessions must hold fresh freshness",
            ),
            ImportSessionStateClass::StaleWithinWindow => self.expect(
                matches!(
                    session.freshness.freshness_class,
                    FreshnessLabel::StaleWithinWindow
                ),
                "browser_handoff_alpha.import_session_state_freshness_drift",
                "stale_within_window sessions must hold stale_within_window freshness",
            ),
            ImportSessionStateClass::ExpiredBeyondWindow => self.expect(
                matches!(
                    session.freshness.freshness_class,
                    FreshnessLabel::ExpiredBeyondWindow
                ),
                "browser_handoff_alpha.import_session_state_freshness_drift",
                "expired_beyond_window sessions must hold expired_beyond_window freshness",
            ),
            ImportSessionStateClass::RevokedOrDisconnected => self.expect(
                matches!(
                    session.freshness.freshness_class,
                    FreshnessLabel::RevokedOrDisconnected
                ),
                "browser_handoff_alpha.import_session_state_freshness_drift",
                "revoked_or_disconnected sessions must hold revoked_or_disconnected freshness",
            ),
            _ => {}
        }

        if matches!(
            session.session_state,
            ImportSessionStateClass::ReplacedByNewerSession
        ) {
            self.expect(
                session
                    .superseded_session_ref
                    .as_deref()
                    .is_some_and(|reference| !reference.trim().is_empty()),
                "browser_handoff_alpha.import_session_superseded_ref_missing",
                "replaced_by_newer_session must cite a superseded_session_ref",
            );
        }
    }

    fn validate_reconnect_flows(&mut self) {
        for flow in &self.page.reconnect_flows {
            self.expect(
                flow.record_kind == PROVIDER_BROWSER_HANDOFF_ALPHA_RECONNECT_FLOW_RECORD_KIND,
                "browser_handoff_alpha.reconnect_flow_record_kind",
                "reconnect_flow.record_kind is wrong",
            );
            self.expect(
                flow.schema_version == PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
                "browser_handoff_alpha.reconnect_flow_schema_version",
                "reconnect_flow.schema_version is wrong",
            );
            self.expect(
                flow.shared_contract_ref == PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF,
                "browser_handoff_alpha.reconnect_flow_shared_contract_ref",
                "reconnect_flow.shared_contract_ref must match the shared contract id",
            );
            let unique = self.reconnect_flow_ids.insert(&flow.reconnect_flow_id);
            self.expect(
                unique,
                "browser_handoff_alpha.reconnect_flow_duplicate",
                "reconnect_flow_id values must be unique within a page",
            );
            self.expect(
                !flow.display_label.trim().is_empty(),
                "browser_handoff_alpha.reconnect_flow_display_label_missing",
                "reconnect_flow.display_label must be non-empty",
            );
            self.expect(
                !flow.import_session_ref.trim().is_empty(),
                "browser_handoff_alpha.reconnect_flow_import_session_missing",
                "reconnect_flow.import_session_ref must be non-empty",
            );
            self.expect(
                !flow.bound_object_row_ref.trim().is_empty(),
                "browser_handoff_alpha.reconnect_flow_bound_row_missing",
                "reconnect_flow.bound_object_row_ref must be non-empty",
            );
            self.expect(
                !flow.outcome_summary.trim().is_empty(),
                "browser_handoff_alpha.reconnect_flow_summary_missing",
                "reconnect_flow.outcome_summary must be non-empty",
            );
            self.expect(
                !flow.observed_at.trim().is_empty(),
                "browser_handoff_alpha.reconnect_flow_observed_at_missing",
                "reconnect_flow.observed_at must be non-empty",
            );
            self.expect(
                flow.local_editing_preserved,
                "browser_handoff_alpha.reconnect_flow_local_editing_not_preserved",
                "reconnect_flow.local_editing_preserved must be true",
            );
            self.expect(
                !flow.silent_authority_widening_taken,
                "browser_handoff_alpha.reconnect_flow_silent_authority_widening",
                "reconnect_flow.silent_authority_widening_taken must be false",
            );

            self.expect(
                self.import_session_ids
                    .contains(flow.import_session_ref.as_str()),
                "browser_handoff_alpha.reconnect_flow_import_session_unknown",
                "reconnect_flow.import_session_ref must reference an import session in the page",
            );

            self.validate_reconnect_outcome_refs(flow);

            self.coverage.reconnect_outcomes.insert(flow.outcome);
        }
    }

    fn validate_reconnect_outcome_refs(&mut self, flow: &ProviderReconnectFlow) {
        let non_empty =
            |opt: &Option<String>| opt.as_deref().is_some_and(|value| !value.trim().is_empty());

        if flow.outcome.requires_restored_object() {
            self.expect(
                non_empty(&flow.restored_object_row_ref),
                "browser_handoff_alpha.reconnect_flow_restored_object_missing",
                "restored_authoritative_local_object must cite a restored_object_row_ref",
            );
            self.expect(
                flow.placeholder_kind.is_none(),
                "browser_handoff_alpha.reconnect_flow_placeholder_conflict",
                "restored_authoritative_local_object must not cite a placeholder_kind",
            );
        }
        if flow.outcome.requires_placeholder_kind() {
            self.expect(
                flow.placeholder_kind.is_some(),
                "browser_handoff_alpha.reconnect_flow_placeholder_missing",
                "restored_truthful_placeholder must cite a placeholder_kind",
            );
            self.expect(
                flow.restored_object_row_ref.is_none(),
                "browser_handoff_alpha.reconnect_flow_placeholder_object_conflict",
                "restored_truthful_placeholder must not cite a restored_object_row_ref",
            );
        }
        if flow.outcome.requires_publish_later_ref() {
            self.expect(
                non_empty(&flow.publish_later_queue_item_ref),
                "browser_handoff_alpha.reconnect_flow_publish_later_missing",
                "deferred_to_publish_later_queue must cite a publish_later_queue_item_ref",
            );
        }
        if flow.outcome.requires_followup_packet_ref() {
            self.expect(
                non_empty(&flow.follow_up_packet_ref),
                "browser_handoff_alpha.reconnect_flow_followup_packet_missing",
                "browser_handoff_offered_again must cite a follow_up_packet_ref",
            );
        }

        if let Some(packet_ref) = flow.originating_packet_ref.as_deref() {
            self.expect(
                self.packet_ids.contains(packet_ref),
                "browser_handoff_alpha.reconnect_flow_originating_packet_unknown",
                "reconnect_flow.originating_packet_ref must reference a packet in the page",
            );
        }

        if let Some(packet_ref) = flow.follow_up_packet_ref.as_deref() {
            self.expect(
                self.packet_ids.contains(packet_ref),
                "browser_handoff_alpha.reconnect_flow_followup_packet_unknown",
                "reconnect_flow.follow_up_packet_ref must reference a packet in the page",
            );
        }
    }

    fn validate_continuity_observations(&mut self) {
        for observation in &self.page.continuity_observations {
            self.expect(
                observation.record_kind
                    == PROVIDER_BROWSER_HANDOFF_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND,
                "browser_handoff_alpha.observation_record_kind",
                "continuity_observation.record_kind is wrong",
            );
            self.expect(
                observation.schema_version == PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
                "browser_handoff_alpha.observation_schema_version",
                "continuity_observation.schema_version is wrong",
            );
            self.expect(
                observation.shared_contract_ref
                    == PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF,
                "browser_handoff_alpha.observation_shared_contract_ref",
                "continuity_observation.shared_contract_ref must match the shared contract id",
            );
            let unique = self.observation_ids.insert(&observation.observation_id);
            self.expect(
                unique,
                "browser_handoff_alpha.observation_duplicate",
                "observation_id values must be unique within a page",
            );
            self.expect(
                !observation.rationale_summary.trim().is_empty(),
                "browser_handoff_alpha.observation_rationale_missing",
                "continuity_observation.rationale_summary must be non-empty",
            );
            self.expect(
                !observation.silent_mutation_authority_widened,
                "browser_handoff_alpha.observation_silent_widen",
                "continuity_observation.silent_mutation_authority_widened must be false",
            );

            let bound_count = [
                observation.bound_packet_ref.is_some(),
                observation.bound_import_session_ref.is_some(),
                observation.bound_reconnect_flow_ref.is_some(),
            ]
            .into_iter()
            .filter(|present| *present)
            .count();
            self.expect(
                bound_count == 1,
                "browser_handoff_alpha.observation_binding_invalid",
                "continuity_observation must bind to exactly one of packet/import_session/\
                 reconnect_flow",
            );

            if let Some(packet_ref) = observation.bound_packet_ref.as_deref() {
                self.expect(
                    self.packet_ids.contains(packet_ref),
                    "browser_handoff_alpha.observation_packet_ref_unknown",
                    "continuity_observation.bound_packet_ref must reference a packet in the page",
                );
            }
            if let Some(session_ref) = observation.bound_import_session_ref.as_deref() {
                self.expect(
                    self.import_session_ids.contains(session_ref),
                    "browser_handoff_alpha.observation_import_session_ref_unknown",
                    "continuity_observation.bound_import_session_ref must reference an import \
                     session in the page",
                );
            }
            if let Some(flow_ref) = observation.bound_reconnect_flow_ref.as_deref() {
                self.expect(
                    self.reconnect_flow_ids.contains(flow_ref),
                    "browser_handoff_alpha.observation_reconnect_flow_ref_unknown",
                    "continuity_observation.bound_reconnect_flow_ref must reference a reconnect \
                     flow in the page",
                );
            }

            if observation.retained_capability_class
                == RetainedCapabilityClass::NoCapabilityRetained
            {
                self.expect(
                    !observation.local_editing_preserved,
                    "browser_handoff_alpha.observation_no_capability_with_local_editing",
                    "no_capability_retained cannot coexist with local_editing_preserved=true",
                );
                self.expect(
                    observation.degraded_action != DegradedActionClass::ContinueLocalAuthoring,
                    "browser_handoff_alpha.observation_no_capability_continue",
                    "no_capability_retained cannot pair with continue_local_authoring",
                );
            }

            self.coverage
                .continuity_observation_classes
                .insert(observation.observation_class);
        }
    }

    fn validate_required_coverage(&mut self) {
        for origin in [
            HandoffOriginClass::WorkspaceReviewLane,
            HandoffOriginClass::WorkspaceRuntimeLane,
            HandoffOriginClass::WorkspaceProviderLane,
        ] {
            self.expect(
                self.coverage.origin_classes.contains(&origin),
                "browser_handoff_alpha.coverage_origin_class_missing",
                "page must cover review, runtime, and provider workspace lane origins",
            );
        }
        for destination in [
            HandoffDestinationClass::CodeHostWeb,
            HandoffDestinationClass::IssueTrackerWeb,
            HandoffDestinationClass::CiProviderWeb,
        ] {
            self.expect(
                self.coverage.destination_classes.contains(&destination),
                "browser_handoff_alpha.coverage_destination_class_missing",
                "page must cover code_host, issue_tracker, and ci_provider destinations",
            );
        }
        for action in [
            HandoffFollowUpActionClass::ReturnToLocalDraftAuthoring,
            HandoffFollowUpActionClass::ReturnToPublishLaterQueueItem,
            HandoffFollowUpActionClass::ReturnToTruthfulPlaceholder,
        ] {
            self.expect(
                self.coverage.follow_up_action_classes.contains(&action),
                "browser_handoff_alpha.coverage_follow_up_action_missing",
                "page must cover local-draft, publish-later, and truthful-placeholder follow-ups",
            );
        }
        for outcome in [
            ReconnectOutcomeClass::RestoredAuthoritativeLocalObject,
            ReconnectOutcomeClass::RestoredTruthfulPlaceholder,
        ] {
            self.expect(
                self.coverage.reconnect_outcomes.contains(&outcome),
                "browser_handoff_alpha.coverage_reconnect_outcome_missing",
                "page must cover the authoritative-local-object and truthful-placeholder reconnect \
                 outcomes",
            );
        }
    }

    fn expect(&mut self, passed: bool, check_id: &str, message: &str) {
        if !passed {
            self.findings.push(ProviderBrowserHandoffAlphaFinding {
                severity: ProviderBrowserHandoffAlphaFindingSeverity::Error,
                check_id: check_id.to_string(),
                message: message.to_string(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{FreshnessLabel, FreshnessTruth, ProviderFamily};

    fn fresh() -> FreshnessTruth {
        FreshnessTruth {
            freshness_class: FreshnessLabel::Fresh,
            observed_at: Some("2026-05-13T18:00:00Z".to_string()),
            freshness_floor_ref: "freshness.provider.fresh".to_string(),
            stale_after: Some("PT15M".to_string()),
            degraded_reason: None,
            import_session_ref: None,
        }
    }

    fn stale() -> FreshnessTruth {
        FreshnessTruth {
            freshness_class: FreshnessLabel::StaleWithinWindow,
            observed_at: Some("2026-05-13T17:30:00Z".to_string()),
            freshness_floor_ref: "freshness.provider.stale".to_string(),
            stale_after: Some("PT45M".to_string()),
            degraded_reason: Some("Stale within window.".to_string()),
            import_session_ref: Some("import.session.stale".to_string()),
        }
    }

    fn revoked() -> FreshnessTruth {
        FreshnessTruth {
            freshness_class: FreshnessLabel::RevokedOrDisconnected,
            observed_at: Some("2026-05-13T18:00:00Z".to_string()),
            freshness_floor_ref: "freshness.provider.revoked".to_string(),
            stale_after: None,
            degraded_reason: Some("Provider grant revoked.".to_string()),
            import_session_ref: None,
        }
    }

    fn packet(
        id: &str,
        origin: HandoffOriginClass,
        destination: HandoffDestinationClass,
        family: ProviderFamily,
        follow_up: HandoffFollowUpActionClass,
        state: HandoffPacketStateClass,
    ) -> BrowserHandoffPacket {
        BrowserHandoffPacket {
            record_kind: PROVIDER_BROWSER_HANDOFF_ALPHA_PACKET_RECORD_KIND.to_string(),
            schema_version: PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF.to_string(),
            packet_id: id.to_string(),
            display_label: format!("Packet {id}"),
            origin: HandoffOriginDisclosure {
                origin_class: origin,
                host_identity_ref: "host.identity.workspace.primary".to_string(),
                workspace_id_ref: "workspace.id.primary".to_string(),
                execution_context_id_ref: "execution.context.primary".to_string(),
            },
            destination: HandoffDestinationDisclosure {
                destination_class: destination,
                canonical_host_ref: "provider.host.primary".to_string(),
                tenant_or_org_scope_ref: "provider.tenant.aureline".to_string(),
                environment_ref: None,
                provider_family: family,
            },
            provider_object_row_ref: format!("provider_object_alpha.row.{id}"),
            provider_side_object_id: format!("provider.side.{id}"),
            intended_follow_up_action: follow_up,
            follow_up_summary: "Return to draft authoring on resolve.".to_string(),
            packet_state: state,
            integration_packet_ref: None,
            publish_later_queue_item_ref: match follow_up {
                HandoffFollowUpActionClass::ReturnToPublishLaterQueueItem => {
                    Some(format!("queue.{id}"))
                }
                _ => None,
            },
            approval_ticket_ref: None,
            import_session_ref: None,
            return_summary: if state.requires_return_summary() {
                Some("Returned to the local draft.".to_string())
            } else {
                None
            },
            placeholder_kind: if matches!(
                state,
                HandoffPacketStateClass::ReturnedTruthfulPlaceholder
            ) {
                Some(HandoffPlaceholderClass::LocalObjectEvictedBeyondRetention)
            } else {
                None
            },
            audit_event_refs: vec![format!("audit.event.packet.{id}")],
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_url_present: false,
            raw_token_material_present: false,
            raw_provider_payload_present: false,
            silent_authority_widening_taken: false,
            support_export_summary: format!("Support summary for {id}"),
            minted_at: "2026-05-13T18:00:00Z".to_string(),
            replay_expires_at: "2026-05-13T18:15:00Z".to_string(),
        }
    }

    fn import_session(
        id: &str,
        family: ProviderFamily,
        state: ImportSessionStateClass,
        freshness: FreshnessTruth,
    ) -> ImportSessionRecord {
        ImportSessionRecord {
            record_kind: PROVIDER_BROWSER_HANDOFF_ALPHA_IMPORT_SESSION_RECORD_KIND.to_string(),
            schema_version: PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF.to_string(),
            import_session_id: id.to_string(),
            display_label: format!("Import session {id}"),
            provider_object_row_ref: format!("provider_object_alpha.row.{id}.target"),
            provider_family: family,
            session_state: state,
            freshness,
            superseded_session_ref: None,
            audit_event_refs: vec![format!("audit.event.import.{id}")],
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_provider_payload_present: false,
            local_editing_preserved: true,
            support_export_summary: format!("Import session {id} support summary"),
            opened_at: "2026-05-13T17:00:00Z".to_string(),
            last_observed_at: "2026-05-13T18:00:00Z".to_string(),
        }
    }

    fn reconnect(
        id: &str,
        session_ref: &str,
        outcome: ReconnectOutcomeClass,
    ) -> ProviderReconnectFlow {
        ProviderReconnectFlow {
            record_kind: PROVIDER_BROWSER_HANDOFF_ALPHA_RECONNECT_FLOW_RECORD_KIND.to_string(),
            schema_version: PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF.to_string(),
            reconnect_flow_id: id.to_string(),
            display_label: format!("Reconnect {id}"),
            originating_packet_ref: None,
            import_session_ref: session_ref.to_string(),
            bound_object_row_ref: format!("provider_object_alpha.row.{id}.target"),
            outcome,
            restored_object_row_ref: outcome
                .requires_restored_object()
                .then(|| format!("provider_object_alpha.row.{id}.restored")),
            placeholder_kind: outcome
                .requires_placeholder_kind()
                .then_some(HandoffPlaceholderClass::SupersededByNewerImportSession),
            publish_later_queue_item_ref: outcome
                .requires_publish_later_ref()
                .then(|| format!("queue.reconnect.{id}")),
            follow_up_packet_ref: outcome
                .requires_followup_packet_ref()
                .then(|| format!("packet.followup.{id}")),
            outcome_summary: format!("Reconnect {id} resolved."),
            audit_event_refs: vec![format!("audit.event.reconnect.{id}")],
            redaction_class: RedactionClass::MetadataSafeDefault,
            local_editing_preserved: true,
            silent_authority_widening_taken: false,
            observed_at: "2026-05-13T18:01:00Z".to_string(),
        }
    }

    fn baseline_page() -> ProviderBrowserHandoffAlphaPage {
        let packet_review = packet(
            "pr.4013",
            HandoffOriginClass::WorkspaceReviewLane,
            HandoffDestinationClass::CodeHostWeb,
            ProviderFamily::CodeHost,
            HandoffFollowUpActionClass::ReturnToLocalDraftAuthoring,
            HandoffPacketStateClass::LaunchedAwaitingReturn,
        );
        let packet_runtime = packet(
            "issue.aur.104",
            HandoffOriginClass::WorkspaceRuntimeLane,
            HandoffDestinationClass::IssueTrackerWeb,
            ProviderFamily::IssueTracker,
            HandoffFollowUpActionClass::ReturnToPublishLaterQueueItem,
            HandoffPacketStateClass::ReturnedAuthoritativeLocalObject,
        );
        let packet_provider = packet(
            "ci.99012",
            HandoffOriginClass::WorkspaceProviderLane,
            HandoffDestinationClass::CiProviderWeb,
            ProviderFamily::CiChecks,
            HandoffFollowUpActionClass::ReturnToTruthfulPlaceholder,
            HandoffPacketStateClass::ReturnedTruthfulPlaceholder,
        );
        let session_fresh = import_session(
            "issue.session.primary",
            ProviderFamily::IssueTracker,
            ImportSessionStateClass::ObservedFresh,
            fresh(),
        );
        let session_stale = import_session(
            "ci.session.smoke",
            ProviderFamily::CiChecks,
            ImportSessionStateClass::StaleWithinWindow,
            stale(),
        );
        let session_revoked = import_session(
            "ci.session.annotation",
            ProviderFamily::CiChecks,
            ImportSessionStateClass::RevokedOrDisconnected,
            revoked(),
        );
        let reconnect_restored = reconnect(
            "reconnect.issue.104",
            "issue.session.primary",
            ReconnectOutcomeClass::RestoredAuthoritativeLocalObject,
        );
        let reconnect_placeholder = reconnect(
            "reconnect.ci.99012",
            "ci.session.annotation",
            ReconnectOutcomeClass::RestoredTruthfulPlaceholder,
        );

        ProviderBrowserHandoffAlphaPage {
            fixture_metadata: None,
            record_kind: PROVIDER_BROWSER_HANDOFF_ALPHA_PAGE_RECORD_KIND.to_string(),
            schema_version: PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF.to_string(),
            page_id: "browser_handoff_alpha.page.unit_test".to_string(),
            contract_refs: ProviderBrowserHandoffAlphaContractRefs {
                provider_object_model_alpha_schema_ref:
                    "schemas/providers/provider_object.schema.json".to_string(),
                browser_handoff_packet_schema_ref:
                    "schemas/providers/browser_handoff_packet.schema.json".to_string(),
                publish_later_queue_alpha_schema_ref:
                    "schemas/providers/publish_later_queue_alpha.schema.json".to_string(),
                approval_ticket_alpha_schema_ref:
                    "schemas/security/approval_ticket_alpha.schema.json".to_string(),
                connected_provider_registry_schema_ref:
                    "schemas/providers/connected_provider_registry.schema.json".to_string(),
            },
            packets: vec![packet_review, packet_runtime, packet_provider],
            import_sessions: vec![session_fresh, session_stale, session_revoked],
            reconnect_flows: vec![reconnect_restored, reconnect_placeholder],
            continuity_observations: vec![
                HandoffContinuityObservation {
                    record_kind: PROVIDER_BROWSER_HANDOFF_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND
                        .to_string(),
                    schema_version: PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
                    shared_contract_ref: PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF
                        .to_string(),
                    observation_id: "obs.session.stale".to_string(),
                    bound_packet_ref: None,
                    bound_import_session_ref: Some("ci.session.smoke".to_string()),
                    bound_reconnect_flow_ref: None,
                    observation_class: ContinuityObservationClass::ProviderStaleWithinWindow,
                    retained_capability_class: RetainedCapabilityClass::InspectOnlyRetained,
                    degraded_action: DegradedActionClass::HoldForFreshnessRepair,
                    rationale_summary: "Stale within window; inspect-only retained.".to_string(),
                    observed_at: "2026-05-13T17:30:00Z".to_string(),
                    silent_mutation_authority_widened: false,
                    local_editing_preserved: true,
                },
                HandoffContinuityObservation {
                    record_kind: PROVIDER_BROWSER_HANDOFF_ALPHA_CONTINUITY_OBSERVATION_RECORD_KIND
                        .to_string(),
                    schema_version: PROVIDER_BROWSER_HANDOFF_ALPHA_SCHEMA_VERSION,
                    shared_contract_ref: PROVIDER_BROWSER_HANDOFF_ALPHA_SHARED_CONTRACT_REF
                        .to_string(),
                    observation_id: "obs.reconnect.revoked".to_string(),
                    bound_packet_ref: None,
                    bound_import_session_ref: None,
                    bound_reconnect_flow_ref: Some("reconnect.ci.99012".to_string()),
                    observation_class: ContinuityObservationClass::ProviderRevokedOrDisconnected,
                    retained_capability_class: RetainedCapabilityClass::BrowserHandoffOffered,
                    degraded_action: DegradedActionClass::HoldForReauth,
                    rationale_summary:
                        "Provider grant revoked; truthful placeholder served and reauth offered."
                            .to_string(),
                    observed_at: "2026-05-13T18:01:00Z".to_string(),
                    silent_mutation_authority_widened: false,
                    local_editing_preserved: true,
                },
            ],
            support_export_summary:
                "Browser-handoff alpha unit-test page covering review/runtime/provider origins, \
                 code-host/issue/CI destinations, and authoritative/placeholder reconnects."
                    .to_string(),
        }
    }

    #[test]
    fn baseline_page_validates() {
        let page = baseline_page();
        let report = page.validate();
        assert!(report.passed, "baseline must pass: {:#?}", report.findings);
        assert!(report
            .coverage
            .reconnect_outcomes
            .contains(&ReconnectOutcomeClass::RestoredAuthoritativeLocalObject));
        assert!(report
            .coverage
            .reconnect_outcomes
            .contains(&ReconnectOutcomeClass::RestoredTruthfulPlaceholder));
        assert!(report
            .coverage
            .follow_up_action_classes
            .contains(&HandoffFollowUpActionClass::ReturnToTruthfulPlaceholder));
    }

    #[test]
    fn returned_placeholder_state_must_cite_placeholder_kind() {
        let mut page = baseline_page();
        page.packets
            .iter_mut()
            .find(|packet| packet.packet_id == "ci.99012")
            .expect("placeholder packet present")
            .placeholder_kind = None;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "browser_handoff_alpha.packet_placeholder_missing"
        }));
    }

    #[test]
    fn reconnect_restored_authoritative_requires_restored_object() {
        let mut page = baseline_page();
        let flow = page
            .reconnect_flows
            .iter_mut()
            .find(|flow| flow.reconnect_flow_id == "reconnect.issue.104")
            .expect("restored reconnect present");
        flow.restored_object_row_ref = None;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "browser_handoff_alpha.reconnect_flow_restored_object_missing"
        }));
    }

    #[test]
    fn reconnect_unknown_import_session_is_rejected() {
        let mut page = baseline_page();
        let flow = page
            .reconnect_flows
            .iter_mut()
            .find(|flow| flow.reconnect_flow_id == "reconnect.issue.104")
            .expect("flow present");
        flow.import_session_ref = "unknown".to_string();
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "browser_handoff_alpha.reconnect_flow_import_session_unknown"
        }));
    }

    #[test]
    fn packet_raw_url_must_be_false() {
        let mut page = baseline_page();
        page.packets[0].raw_url_present = true;
        let report = page.validate();
        assert!(!report.passed);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.check_id == "browser_handoff_alpha.packet_raw_url_present"));
    }

    #[test]
    fn observation_binding_must_be_exclusive() {
        let mut page = baseline_page();
        let observation = &mut page.continuity_observations[0];
        observation.bound_packet_ref = Some("pr.4013".to_string());
        observation.bound_import_session_ref = Some("ci.session.smoke".to_string());
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "browser_handoff_alpha.observation_binding_invalid"
        }));
    }

    #[test]
    fn coverage_origin_must_include_review_runtime_provider() {
        let mut page = baseline_page();
        page.packets.retain(|packet| {
            packet.origin.origin_class != HandoffOriginClass::WorkspaceProviderLane
        });
        page.reconnect_flows.clear();
        page.continuity_observations
            .retain(|obs| obs.bound_reconnect_flow_ref.is_none());
        let report = page.validate();
        assert!(!report.passed);
        assert!(report.findings.iter().any(|finding| {
            finding.check_id == "browser_handoff_alpha.coverage_origin_class_missing"
        }));
    }

    #[test]
    fn support_export_omits_action_refs() {
        let page = baseline_page();
        let projection = page.support_export_projection();
        let json = serde_json::to_string(&projection).expect("projection serializes");
        assert_eq!(
            projection.record_kind,
            "provider_browser_handoff_alpha_support_export"
        );
        assert!(!json.contains("raw_url"));
        assert!(!json.contains("raw_token"));
        assert!(!json.contains("approval_ticket_ref"));
        assert!(!json.contains("publish_later_queue_item_ref"));
        assert!(!json.contains("integration_packet_ref"));
        assert!(!json.contains("follow_up_packet_ref"));
    }
}
