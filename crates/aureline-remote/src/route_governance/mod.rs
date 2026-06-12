//! Governed route-object and exposure-review record model.
//!
//! See the crate-level docs for what this module owns and why. The shapes,
//! vocabularies, and validators here mirror the boundary schemas at
//! `/schemas/remote/route_object.schema.json` and
//! `/schemas/remote/exposure_review.schema.json` so the chrome, audits,
//! support exports, and issue reports never invent a divergent shape.
//!
//! The module exposes:
//!
//! - the closed vocabularies (route kind, lifecycle state, host class,
//!   protocol, TLS posture, auth source, audience, controlled exposure
//!   label, viewer state, data sensitivity, copy/share disclosure,
//!   revocation, downgrade, reopen, last-access, review outcome,
//!   reachability, cookie/session, lingering local preview, cross-origin);
//! - the [`RouteObject`] record with a [`RouteObject::validate`] check that
//!   surfaces every truth rule the schema freezes;
//! - the [`ExposureReview`] record with an
//!   [`ExposureReview::validate`] check covering the proposed transition
//!   admittance, public-widening posture, and viewer-state honesty rules;
//! - the [`RevocationSummary`] helper that derives a stable revocation
//!   summary from a [`RouteObject`] so UI, audits, and support exports can
//!   show the same answer to "what was exposed, to whom, for how long, and
//!   under what auth/expiry posture?".

use aureline_auth::{
    secret_boundary_use_audit_result_for_health, seeded_secret_boundary_active_repair_state,
    seeded_secret_boundary_profile_parity_rows, seeded_secret_boundary_repairable_states,
    SecretBoundaryActingIdentityClass, SecretBoundaryConsumerIdentityClass,
    SecretBoundaryConsumerIdentityReceipt, SecretBoundaryCredentialMode,
    SecretBoundaryCredentialStateRow, SecretBoundaryDeclinePath,
    SecretBoundaryDelegatedCredentialRow, SecretBoundaryDelegatedUseClass,
    SecretBoundaryExportSafetyBanner, SecretBoundaryHealthStateClass,
    SecretBoundaryProjectionControl, SecretBoundaryProjectionControlClass,
    SecretBoundaryProjectionMode, SecretBoundaryProjectionModeAudit,
    SecretBoundaryRepairOwnerClass, SecretBoundarySecretAccessPrompt, SecretBoundarySecretClass,
    SecretBoundaryStorageClass, SecretBoundarySurfaceState, SecretBoundaryVaultPickerOption,
    SecretBoundaryVaultPickerState, SecretBoundaryWorkflowDependency,
    M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF,
};
use serde::{Deserialize, Serialize};

/// Stable record-kind tag for serialized [`RouteObject`] payloads.
pub const ROUTE_OBJECT_RECORD_KIND: &str = "route_object_record";

/// Schema version for the [`RouteObject`] payload shape.
pub const ROUTE_OBJECT_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for serialized [`ExposureReview`] payloads.
pub const EXPOSURE_REVIEW_RECORD_KIND: &str = "exposure_review_record";

/// Schema version for the [`ExposureReview`] payload shape.
pub const EXPOSURE_REVIEW_SCHEMA_VERSION: u32 = 1;

const PREVIEW_ROUTE_MATRIX_ROW_ID: &str = "m5.secret.preview_route.remote_preview";

macro_rules! closed_vocab {
    (
        $(#[$type_doc:meta])*
        $name:ident {
            $(
                $(#[$variant_doc:meta])*
                $variant:ident => $token:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$type_doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name {
            $(
                $(#[$variant_doc])*
                $variant
            ),+
        }

        impl $name {
            /// Stable closed-vocabulary token recorded in records, schemas,
            /// and exports.
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $token),+
                }
            }
        }
    };
}

closed_vocab!(
    /// Closed route-kind vocabulary covering the governed exposure lanes.
    RouteKind {
        LocalPortForward => "local_port_forward",
        RemotePortForward => "remote_port_forward",
        ReverseTunnel => "reverse_tunnel",
        ManagedWorkspaceTunnel => "managed_workspace_tunnel",
        BrowserPreviewRoute => "browser_preview_route",
        DevcontainerExposedService => "devcontainer_exposed_service",
        RemoteAgentExposedService => "remote_agent_exposed_service",
        NotebookEndpoint => "notebook_endpoint",
        DebuggerSocket => "debugger_socket",
    }
);

closed_vocab!(
    /// Closed lifecycle-state vocabulary; reconnect, paused, degraded,
    /// stale-target, policy-denied, expired-approval, capability-narrowed,
    /// provider-unavailable, revoked, expired, closed, and blocked states are
    /// deliberately distinct so the surface can name the reason.
    LifecycleState {
        Proposed => "proposed",
        PendingReview => "pending_review",
        Active => "active",
        SuspendedReconnect => "suspended_reconnect",
        Paused => "paused",
        Degraded => "degraded",
        StaleTarget => "stale_target",
        PolicyDenied => "policy_denied",
        ApprovalExpired => "approval_expired",
        CapabilityNarrowed => "capability_narrowed",
        ProviderUnavailable => "provider_unavailable",
        Revoked => "revoked",
        Expired => "expired",
        Closed => "closed",
        Blocked => "blocked",
    }
);

closed_vocab!(
    /// Closed host-class vocabulary. Names where the source service runs.
    HostClass {
        LocalHost => "local_host",
        UserModeSandbox => "user_mode_sandbox",
        ContainerLocal => "container_local",
        Devcontainer => "devcontainer",
        RemoteSsh => "remote_ssh",
        RemoteAgent => "remote_agent",
        ManagedWorkspace => "managed_workspace",
        NotebookKernelLocal => "notebook_kernel_local",
        NotebookKernelRemote => "notebook_kernel_remote",
        TunnelExposed => "tunnel_exposed",
        BridgedHelper => "bridged_helper",
        UnknownHost => "unknown_host",
    }
);

closed_vocab!(
    /// Closed protocol class for the source service.
    ProtocolClass {
        Http => "http",
        Https => "https",
        Tcp => "tcp",
        TlsTcp => "tls_tcp",
        Websocket => "websocket",
        Grpc => "grpc",
        DebugProtocol => "debug_protocol",
        UnknownProtocolRequiresReview => "unknown_protocol_requires_review",
    }
);

closed_vocab!(
    /// Closed TLS posture vocabulary, including helper- and edge-termination
    /// and end-to-end passthrough.
    TlsPostureClass {
        NoTlsLoopbackOnly => "no_tls_loopback_only",
        TlsTerminatedAtHelper => "tls_terminated_at_helper",
        TlsTerminatedAtTunnelEdge => "tls_terminated_at_tunnel_edge",
        TlsPassthroughToTarget => "tls_passthrough_to_target",
        TlsUnknownRequiresReview => "tls_unknown_requires_review",
    }
);

closed_vocab!(
    /// Closed auth-source vocabulary. Local-only routes use
    /// `no_auth_loopback_only`; shareable routes require a real auth source.
    AuthSourceClass {
        NoAuthLoopbackOnly => "no_auth_loopback_only",
        WorkspaceSessionAuth => "workspace_session_auth",
        OrganizationSso => "organization_sso",
        SignedPreviewLink => "signed_preview_link",
        MachineToMachineAllowlist => "machine_to_machine_allowlist",
        ExternalAuthPassthrough => "external_auth_passthrough",
        ApprovalTicketRequired => "approval_ticket_required",
        AuthUnknownRequiresReview => "auth_unknown_requires_review",
    }
);

closed_vocab!(
    /// Closed audience vocabulary.
    AudienceClass {
        SelfOnly => "self_only",
        SameDeviceLan => "same_device_lan",
        WorkspaceMembers => "workspace_members",
        OrganizationMembers => "organization_members",
        TenantMembers => "tenant_members",
        SignedLinkHolders => "signed_link_holders",
        PublicEphemeralHolders => "public_ephemeral_holders",
        MachineCallbackAllowlist => "machine_callback_allowlist",
        AudienceUnknownRequiresReview => "audience_unknown_requires_review",
    }
);

closed_vocab!(
    /// Closed exposure label (action_exposure_class) for audits and exports.
    ExposureLabelClass {
        NoSideEffectLocalRead => "no_side_effect_local_read",
        LocalOnlyMutation => "local_only_mutation",
        WorkspaceVisibleMutation => "workspace_visible_mutation",
        ProviderVisibleMutation => "provider_visible_mutation",
        PubliclyVisiblePublish => "publicly_visible_publish",
        CrossTenantVisible => "cross_tenant_visible",
        BrowserSessionVisible => "browser_session_visible",
        TunnelExposedPublic => "tunnel_exposed_public",
        ExposureUnknownRequiresReview => "exposure_unknown_requires_review",
    }
);

closed_vocab!(
    /// User-visible controlled exposure label. The exact token is shared by
    /// the chrome row, audit events, support exports, and issue reports.
    ControlledExposureLabel {
        LocalOnly => "local_only",
        SameDeviceLan => "same_device_lan",
        AuthenticatedOrgRoute => "authenticated_org_route",
        SignedPreviewLink => "signed_preview_link",
        PublicRoute => "public_route",
    }
);

impl ControlledExposureLabel {
    /// Short human-readable label rendered in the chrome row, audits, and
    /// support exports.
    pub const fn label(self) -> &'static str {
        match self {
            Self::LocalOnly => "Local only",
            Self::SameDeviceLan => "Same device / LAN",
            Self::AuthenticatedOrgRoute => "Authenticated org route",
            Self::SignedPreviewLink => "Signed preview link",
            Self::PublicRoute => "Public route",
        }
    }

    /// Whether this label admits sharing a link beyond the local device.
    pub const fn is_shareable(self) -> bool {
        matches!(
            self,
            Self::AuthenticatedOrgRoute | Self::SignedPreviewLink | Self::PublicRoute
        )
    }

    /// Whether this label leaves the route reachable beyond the user's org.
    pub const fn is_publicly_reachable(self) -> bool {
        matches!(self, Self::SignedPreviewLink | Self::PublicRoute)
    }
}

closed_vocab!(
    /// Closed viewer-state vocabulary.
    ViewerStateClass {
        LiveService => "live_service",
        CapturedSnapshot => "captured_snapshot",
        MockOrSampleData => "mock_or_sample_data",
        StaleMirroredState => "stale_mirrored_state",
        NoLiveViewer => "no_live_viewer",
    }
);

closed_vocab!(
    /// Closed data-sensitivity vocabulary.
    DataSensitivityClass {
        PublicDemo => "public_demo",
        WorkspacePrivate => "workspace_private",
        SecretsOrUserDataPossible => "secrets_or_user_data_possible",
        AdminOrInfrastructure => "admin_or_infrastructure",
        UnknownRequiresReview => "unknown_requires_review",
    }
);

closed_vocab!(
    /// Closed copy/share disclosure vocabulary.
    CopyDisclosureClass {
        LocalOnlyCopy => "local_only_copy",
        AuthenticatedRouteCopy => "authenticated_route_copy",
        PublicLinkStepUpRequired => "public_link_step_up_required",
        CopyBlocked => "copy_blocked",
        UnknownRequiresReview => "unknown_requires_review",
    }
);

closed_vocab!(
    /// Closed teardown-state vocabulary. Non-active routes cannot remain
    /// "active".
    TeardownState {
        Active => "active",
        PendingRevoke => "pending_revoke",
        SuspendedNoTraffic => "suspended_no_traffic",
        Revoked => "revoked",
        Expired => "expired",
        Closed => "closed",
    }
);

closed_vocab!(
    /// Closed revoke posture; who, if anyone, can revoke this route.
    RevokePostureClass {
        NoLiveRoute => "no_live_route",
        UserSelfRevoke => "user_self_revoke",
        WorkspaceAdminRevoke => "workspace_admin_revoke",
        ManagedAdminRevoke => "managed_admin_revoke",
        PolicyRevocationOnly => "policy_revocation_only",
        AutomaticExpiryOnly => "automatic_expiry_only",
        ProviderRevokeRequired => "provider_revoke_required",
        UnknownRevokeRequiresReview => "unknown_revoke_requires_review",
    }
);

closed_vocab!(
    /// Closed downgrade-state vocabulary describing how a route degrades when
    /// network, policy, approval, or capability shifts.
    DowngradeState {
        None => "none",
        NetworkLossSuspended => "network_loss_suspended",
        RetargetPendingReview => "retarget_pending_review",
        PolicyDeniedBlocked => "policy_denied_blocked",
        ApprovalExpiredReissueRequired => "approval_expired_reissue_required",
        CapabilityNarrowedNoShare => "capability_narrowed_no_share",
        ProviderUnavailableSuspended => "provider_unavailable_suspended",
        PausedNoTraffic => "paused_no_traffic",
        DegradedReadOnly => "degraded_read_only",
        StaleTargetInspectOnly => "stale_target_inspect_only",
        ClosedNoRecovery => "closed_no_recovery",
    }
);

closed_vocab!(
    /// Closed reopen vocabulary. Names whether a revoked or expired route can
    /// be reopened, and under what conditions.
    ReopenClass {
        ReopenSameIdentity => "reopen_same_identity",
        ReopenRequiresReapproval => "reopen_requires_reapproval",
        ReopenRequiresNewTarget => "reopen_requires_new_target",
        ReopenBlockedNoRecovery => "reopen_blocked_no_recovery",
    }
);

closed_vocab!(
    /// Closed stale-shared-link vocabulary.
    StaleSharedLinkState {
        NoSharedLink => "no_shared_link",
        SharedLinkNowStale => "shared_link_now_stale",
        SharedLinkRevoked => "shared_link_revoked",
        SharedLinkExpired => "shared_link_expired",
        SharedLinkUnknownRequiresReview => "shared_link_unknown_requires_review",
    }
);

closed_vocab!(
    /// Closed last-access disclosure vocabulary.
    LastAccessClass {
        NotObserved => "not_observed",
        ObservedMetadataOnly => "observed_metadata_only",
        RedactedByPolicy => "redacted_by_policy",
    }
);

closed_vocab!(
    /// Closed review-outcome vocabulary.
    ReviewOutcomeClass {
        ApprovedAsProposed => "approved_as_proposed",
        ApprovedWithNarrowing => "approved_with_narrowing",
        Denied => "denied",
        BlockedPendingPolicyReview => "blocked_pending_policy_review",
        DeferredPendingInformation => "deferred_pending_information",
    }
);

closed_vocab!(
    /// Closed cookie/session vocabulary.
    CookieSessionClass {
        NoCookiesNoSession => "no_cookies_no_session",
        SessionCookieScopedToRoute => "session_cookie_scoped_to_route",
        SessionCookieInheritsOrg => "session_cookie_inherits_org",
        SharedBrowserSessionAttached => "shared_browser_session_attached",
        CookieBehaviorUnknownRequiresReview => "cookie_behavior_unknown_requires_review",
    }
);

closed_vocab!(
    /// Closed local reachability vocabulary.
    ReachabilityLocalClass {
        LoopbackOnly => "loopback_only",
        LoopbackPlusLan => "loopback_plus_lan",
        LoopbackUnreachable => "loopback_unreachable",
    }
);

closed_vocab!(
    /// Closed public reachability vocabulary.
    ReachabilityPublicClass {
        NotPublic => "not_public",
        PublicEphemeralWithTtl => "public_ephemeral_with_ttl",
        PublicViaSignedLink => "public_via_signed_link",
        PublicBlockedByPolicy => "public_blocked_by_policy",
    }
);

closed_vocab!(
    /// Closed lingering-local-preview vocabulary.
    LingeringLocalPreviewClass {
        NoLocalPreviewRemains => "no_local_preview_remains",
        LocalPreviewRemainsAvailable => "local_preview_remains_available",
        LocalPreviewInspectOnly => "local_preview_inspect_only",
        LocalPreviewRevokedByPolicy => "local_preview_revoked_by_policy",
    }
);

closed_vocab!(
    /// Closed cross-origin/browser-handoff vocabulary.
    CrossOriginClass {
        NoBrowserHandoff => "no_browser_handoff",
        EmbeddedWebviewOnly => "embedded_webview_only",
        SystemBrowserRequiredStepUp => "system_browser_required_step_up",
        CrossOriginBlockedByPolicy => "cross_origin_blocked_by_policy",
    }
);

/// Source service / process block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceBlock {
    /// Opaque source service handle.
    pub service_ref: String,
    /// Opaque source-process handle when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_ref: Option<String>,
    /// Export-safe service label rendered in the row.
    pub service_label: String,
    /// Source protocol class.
    pub protocol_class: ProtocolClass,
}

/// Host/workspace identity block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HostIdentityBlock {
    /// Closed host class.
    pub host_class: HostClass,
    /// Opaque host identity ref.
    pub host_identity_ref: String,
    /// Opaque workspace identity ref.
    pub workspace_identity_ref: String,
    /// Opaque environment identity ref.
    pub environment_identity_ref: String,
    /// Opaque target identity witness ref.
    pub target_identity_witness_ref: String,
}

/// Opaque endpoint handles. The schema preserves port and path as opaque
/// handles only; raw values never appear here.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EndpointHandlesBlock {
    /// Opaque target-port handle.
    pub target_port_handle_ref: String,
    /// Opaque local-bind handle, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_bind_handle_ref: Option<String>,
    /// Opaque public-route handle, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub public_route_handle_ref: Option<String>,
    /// Opaque target-path handle, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_path_handle_ref: Option<String>,
}

/// Audience block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AudienceBlock {
    /// Closed audience class.
    pub audience_class: AudienceClass,
    /// Opaque audience reference.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience_ref: Option<String>,
    /// Export-safe audience summary.
    pub audience_summary: String,
}

/// Auth/TLS block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthBlock {
    /// Closed auth-source class.
    pub auth_source_class: AuthSourceClass,
    /// Closed TLS posture class.
    pub tls_posture_class: TlsPostureClass,
    /// Opaque authority-ticket ref, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub authority_ticket_ref: Option<String>,
    /// Opaque session-handle ref, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_handle_ref: Option<String>,
}

/// Expiry block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExpiryBlock {
    /// Wall-clock expiry timestamp, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// TTL seconds, if known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl_seconds: Option<u64>,
    /// Whether expiry can be renewed without reissuing identity.
    pub renewable: bool,
}

/// Last-access disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LastAccessBlock {
    /// Closed last-access disclosure class.
    pub last_access_class: LastAccessClass,
    /// Last-access wall-clock timestamp, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_access_at: Option<String>,
}

/// Copy/share/open disclosure block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyShareBlock {
    /// Closed copy disclosure class.
    pub copy_disclosure_class: CopyDisclosureClass,
    /// Opaque share-link handle, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub share_link_handle_ref: Option<String>,
    /// Whether the open action is available.
    pub open_action_allowed: bool,
    /// Whether the copy action is available.
    pub copy_action_allowed: bool,
    /// Whether the share action is available.
    pub share_action_allowed: bool,
}

/// Revocation block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationBlock {
    /// Closed teardown state.
    pub teardown_state: TeardownState,
    /// Closed revoke posture.
    pub revoke_posture_class: RevokePostureClass,
    /// Opaque revocation ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revocation_ref: Option<String>,
    /// Opaque link refs whose state changed when this route was revoked.
    #[serde(default)]
    pub affected_link_refs: Vec<String>,
    /// Export-safe session-impact summary.
    pub session_impact_summary: String,
    /// Closed stale-shared-link state.
    pub stale_shared_link_state: StaleSharedLinkState,
    /// Closed reopen class.
    pub reopen_class: ReopenClass,
    /// Export-safe revocation summary.
    pub summary: String,
}

/// Downgrade block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DowngradeBlock {
    /// Closed downgrade state.
    pub downgrade_state: DowngradeState,
    /// Whether local continuation is allowed under this downgrade.
    pub local_continuation_allowed: bool,
    /// Export-safe downgrade summary.
    pub summary: String,
}

/// Canonical governed route-object record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteObject {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Record-payload schema version.
    pub route_object_schema_version: u32,
    /// Stable opaque route identity.
    pub route_id: String,
    /// Capture timestamp.
    pub captured_at: String,
    /// Update timestamp.
    pub updated_at: String,
    /// Route kind.
    pub route_kind: RouteKind,
    /// Route lifecycle state.
    pub lifecycle_state: LifecycleState,
    /// Source service/process block.
    pub source: SourceBlock,
    /// Host/workspace identity block.
    pub host_identity: HostIdentityBlock,
    /// Opaque endpoint handles.
    pub endpoint_handles: EndpointHandlesBlock,
    /// Exposure label (action_exposure_class).
    pub exposure_label: ExposureLabelClass,
    /// User-visible controlled exposure label.
    pub controlled_exposure_label: ControlledExposureLabel,
    /// Audience block.
    pub audience: AudienceBlock,
    /// Auth/TLS block.
    pub auth: AuthBlock,
    /// Viewer-state class.
    pub viewer_state_class: ViewerStateClass,
    /// Data sensitivity class.
    pub data_sensitivity_class: DataSensitivityClass,
    /// Expiry block.
    pub expiry: ExpiryBlock,
    /// Last-access disclosure block.
    pub last_access: LastAccessBlock,
    /// Copy/share/open disclosure block.
    pub copy_share: CopyShareBlock,
    /// Revocation block.
    pub revocation: RevocationBlock,
    /// Downgrade block.
    pub downgrade: DowngradeBlock,
    /// Export-safe row summary.
    pub summary: String,
    /// Opaque review ref, if any.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub review_ref: Option<String>,
}

/// Typed validation finding for a [`RouteObject`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RouteObjectFinding {
    /// Stable check id.
    pub check_id: String,
    /// Subject row id.
    pub subject_ref: String,
    /// Export-safe finding message.
    pub message: String,
}

impl RouteObjectFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

impl RouteObject {
    /// Returns typed truth-rule findings; an empty vector means the row is
    /// internally consistent with the schema's allOf rules.
    pub fn validate(&self) -> Vec<RouteObjectFinding> {
        let mut findings = Vec::new();
        let subject = self.route_id.as_str();

        if self.record_kind != ROUTE_OBJECT_RECORD_KIND {
            findings.push(RouteObjectFinding::new(
                "route_object.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    ROUTE_OBJECT_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.route_object_schema_version != ROUTE_OBJECT_SCHEMA_VERSION {
            findings.push(RouteObjectFinding::new(
                "route_object.schema_version",
                subject,
                format!(
                    "route_object_schema_version must be {ROUTE_OBJECT_SCHEMA_VERSION}, found {}",
                    self.route_object_schema_version
                ),
            ));
        }

        match self.controlled_exposure_label {
            ControlledExposureLabel::LocalOnly => {
                if self.copy_share.copy_disclosure_class != CopyDisclosureClass::LocalOnlyCopy {
                    findings.push(RouteObjectFinding::new(
                        "route_object.local_only_copy_disclosure",
                        subject,
                        "local_only routes must use local_only_copy disclosure",
                    ));
                }
                if self.copy_share.share_link_handle_ref.is_some() {
                    findings.push(RouteObjectFinding::new(
                        "route_object.local_only_share_link",
                        subject,
                        "local_only routes must not carry a share-link handle",
                    ));
                }
                if self.copy_share.share_action_allowed {
                    findings.push(RouteObjectFinding::new(
                        "route_object.local_only_share_allowed",
                        subject,
                        "local_only routes must not enable the share action",
                    ));
                }
                if self.auth.auth_source_class != AuthSourceClass::NoAuthLoopbackOnly {
                    findings.push(RouteObjectFinding::new(
                        "route_object.local_only_auth_source",
                        subject,
                        "local_only routes must use no_auth_loopback_only",
                    ));
                }
            }
            ControlledExposureLabel::PublicRoute => {
                if self.expiry.expires_at.is_none() {
                    findings.push(RouteObjectFinding::new(
                        "route_object.public_route_expiry_required",
                        subject,
                        "public_route widening must declare an expiry timestamp",
                    ));
                }
                if self.copy_share.copy_disclosure_class
                    != CopyDisclosureClass::PublicLinkStepUpRequired
                {
                    findings.push(RouteObjectFinding::new(
                        "route_object.public_route_copy_disclosure",
                        subject,
                        "public_route widening must use public_link_step_up_required",
                    ));
                }
                if !self.copy_share.share_action_allowed {
                    findings.push(RouteObjectFinding::new(
                        "route_object.public_route_share_action",
                        subject,
                        "public_route widening must enable the share action",
                    ));
                }
            }
            _ => {}
        }

        if self.controlled_exposure_label.is_shareable()
            && matches!(
                self.auth.auth_source_class,
                AuthSourceClass::NoAuthLoopbackOnly
            )
        {
            findings.push(RouteObjectFinding::new(
                "route_object.shareable_loopback_auth",
                subject,
                "shareable controlled-exposure routes cannot use no_auth_loopback_only",
            ));
        }

        let non_active = matches!(
            self.lifecycle_state,
            LifecycleState::Revoked
                | LifecycleState::Expired
                | LifecycleState::SuspendedReconnect
                | LifecycleState::PolicyDenied
                | LifecycleState::ApprovalExpired
                | LifecycleState::CapabilityNarrowed
                | LifecycleState::StaleTarget
                | LifecycleState::Blocked
        );
        if non_active && self.revocation.teardown_state == TeardownState::Active {
            findings.push(RouteObjectFinding::new(
                "route_object.non_active_teardown_state",
                subject,
                "non-active lifecycle states must not declare an active teardown state",
            ));
        }

        if self.lifecycle_state == LifecycleState::StaleTarget {
            let viewer_ok = matches!(
                self.viewer_state_class,
                ViewerStateClass::StaleMirroredState
                    | ViewerStateClass::CapturedSnapshot
                    | ViewerStateClass::NoLiveViewer
            );
            if !viewer_ok {
                findings.push(RouteObjectFinding::new(
                    "route_object.stale_viewer_state",
                    subject,
                    "stale_target routes must not claim live_service viewer state",
                ));
            }
            if !matches!(
                self.downgrade.downgrade_state,
                DowngradeState::StaleTargetInspectOnly | DowngradeState::RetargetPendingReview
            ) {
                findings.push(RouteObjectFinding::new(
                    "route_object.stale_downgrade_state",
                    subject,
                    "stale_target routes must downgrade to stale-inspect or retarget-pending",
                ));
            }
        }

        if self.lifecycle_state == LifecycleState::Revoked
            && self.revocation.teardown_state != TeardownState::Revoked
        {
            findings.push(RouteObjectFinding::new(
                "route_object.revoked_teardown_state",
                subject,
                "revoked routes must declare revoked teardown state",
            ));
        }
        if self.lifecycle_state == LifecycleState::Expired
            && self.revocation.teardown_state != TeardownState::Expired
        {
            findings.push(RouteObjectFinding::new(
                "route_object.expired_teardown_state",
                subject,
                "expired routes must declare expired teardown state",
            ));
        }

        findings
    }

    /// Projects the shared M5 secret-boundary state for a preview-route
    /// object plus its optional exposure review.
    pub fn secret_boundary_state(
        &self,
        review: Option<&ExposureReview>,
    ) -> SecretBoundarySurfaceState {
        let credential_mode = route_credential_mode(self.auth.auth_source_class);
        let storage_class = route_storage_class(self.auth.auth_source_class);
        let projection_mode = route_projection_mode(self.auth.auth_source_class);
        let health_state = route_health_state(self.auth.auth_source_class, self.lifecycle_state);
        let actor_identity = route_actor_identity(self.auth.auth_source_class);
        let consumer_identity = SecretBoundaryConsumerIdentityClass::PreviewPublisher;
        let target_label = format!(
            "{} / {}",
            self.source.service_label,
            self.controlled_exposure_label.as_str()
        );
        let decline_path = SecretBoundaryDeclinePath {
            decline_label: "Keep local preview only".to_owned(),
            still_works_summary:
                "Declining keeps local preview, route review, and exact desktop handoff instructions available."
                    .to_owned(),
        };
        let projection_controls =
            route_projection_controls(PREVIEW_ROUTE_MATRIX_ROW_ID, self.auth.auth_source_class);
        let audit_result = secret_boundary_use_audit_result_for_health(health_state);
        let issuer_label = review
            .map(|review| review.summary.clone())
            .unwrap_or_else(|| self.revocation.summary.clone());

        SecretBoundarySurfaceState {
            matrix_row_id: PREVIEW_ROUTE_MATRIX_ROW_ID.to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            secret_access_prompt: SecretBoundarySecretAccessPrompt {
                matrix_row_id: PREVIEW_ROUTE_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                requester_label: "Preview route exposure review".to_owned(),
                secret_class: SecretBoundarySecretClass::CloudDelegatedIdentity,
                target_workflow_label: target_label.clone(),
                storage_class,
                credential_mode,
                projection_mode,
                lifetime_label: "Preview route session".to_owned(),
                expires_at: self.expiry.expires_at.clone(),
                dependent_workflows: vec![
                    route_workflow("workflow:preview.route", "Open preview route"),
                    route_workflow("workflow:preview.share", "Share or revoke preview route"),
                ],
                decline_path: decline_path.clone(),
            },
            credential_state_row: SecretBoundaryCredentialStateRow {
                matrix_row_id: PREVIEW_ROUTE_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                display_label: "Preview route credential state".to_owned(),
                secret_class: SecretBoundarySecretClass::CloudDelegatedIdentity,
                source_class: credential_mode,
                target_boundary_label: target_label.clone(),
                storage_class,
                projection_mode,
                health_state,
                expires_at: self.expiry.expires_at.clone(),
                rotate_action_label: "Refresh preview route".to_owned(),
                revoke_action_label: "Revoke preview route".to_owned(),
                test_action_label: "Validate preview trust".to_owned(),
                dependent_workflows: vec![
                    route_workflow("workflow:preview.route", "Open preview route"),
                    route_workflow("workflow:preview.share", "Share or revoke preview route"),
                ],
                decline_path,
            },
            vault_picker: Some(route_picker_state()),
            delegated_credential_row: Some(SecretBoundaryDelegatedCredentialRow {
                matrix_row_id: PREVIEW_ROUTE_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                delegated_use_class: route_delegated_use(self.auth.auth_source_class),
                target_host_or_workspace_label: self.host_identity.workspace_identity_ref.clone(),
                expires_at: self.expiry.expires_at.clone(),
                policy_owner_label: issuer_label.clone(),
                projection_controls: projection_controls.clone(),
            }),
            consumer_identity_receipt: SecretBoundaryConsumerIdentityReceipt::new(
                format!("{PREVIEW_ROUTE_MATRIX_ROW_ID}:consumer-receipt"),
                PREVIEW_ROUTE_MATRIX_ROW_ID,
                actor_identity,
                consumer_identity,
                issuer_label.clone(),
                self.host_identity.workspace_identity_ref.clone(),
                credential_mode,
                projection_mode,
                storage_class,
                audit_result,
            ),
            projection_mode_audit: SecretBoundaryProjectionModeAudit::new(
                format!("{PREVIEW_ROUTE_MATRIX_ROW_ID}:projection-audit"),
                PREVIEW_ROUTE_MATRIX_ROW_ID,
                actor_identity,
                consumer_identity,
                issuer_label,
                self.host_identity.workspace_identity_ref.clone(),
                projection_mode,
                audit_result,
                SecretBoundaryRepairOwnerClass::RemoteOperator,
                projection_controls
                    .iter()
                    .map(|control| control.control_class)
                    .collect(),
            ),
            repairable_states: seeded_secret_boundary_repairable_states(PREVIEW_ROUTE_MATRIX_ROW_ID),
            active_repair_state: seeded_secret_boundary_active_repair_state(
                PREVIEW_ROUTE_MATRIX_ROW_ID,
                health_state,
            ),
            profile_parity_rows: seeded_secret_boundary_profile_parity_rows(
                PREVIEW_ROUTE_MATRIX_ROW_ID,
            ),
            export_safety_banner: SecretBoundaryExportSafetyBanner::standard(
                PREVIEW_ROUTE_MATRIX_ROW_ID,
                "Raw preview credentials, signed-link material, and callback payloads stay excluded from support bundles, route shares, and exported route history.",
            ),
        }
    }
}

fn route_actor_identity(auth_source: AuthSourceClass) -> SecretBoundaryActingIdentityClass {
    match auth_source {
        AuthSourceClass::OrganizationSso
        | AuthSourceClass::ExternalAuthPassthrough
        | AuthSourceClass::ApprovalTicketRequired => {
            SecretBoundaryActingIdentityClass::DelegatedCredential
        }
        AuthSourceClass::MachineToMachineAllowlist => {
            SecretBoundaryActingIdentityClass::ServiceIssuedAuthority
        }
        _ => SecretBoundaryActingIdentityClass::LocalOnlyHandle,
    }
}

fn route_projection_controls(
    matrix_row_id: &str,
    auth_source: AuthSourceClass,
) -> Vec<SecretBoundaryProjectionControl> {
    let local_safe_note =
        "Local preview, route review, and exact desktop handoff instructions remain available.";
    let mut controls = vec![
        SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::PauseForwarding,
            "Pause forwarded preview credential",
            local_safe_note,
        ),
        SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::StopUsingSecret,
            "Stop preview route exposure",
            local_safe_note,
        ),
    ];
    if matches!(
        auth_source,
        AuthSourceClass::OrganizationSso
            | AuthSourceClass::ExternalAuthPassthrough
            | AuthSourceClass::ApprovalTicketRequired
            | AuthSourceClass::MachineToMachineAllowlist
    ) {
        controls.push(SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            "Drop delegated preview identity",
            local_safe_note,
        ));
    }
    controls
}

fn route_workflow(
    workflow_ref: impl Into<String>,
    workflow_label: impl Into<String>,
) -> SecretBoundaryWorkflowDependency {
    SecretBoundaryWorkflowDependency {
        workflow_ref: workflow_ref.into(),
        workflow_label: workflow_label.into(),
    }
}

fn route_picker_state() -> SecretBoundaryVaultPickerState {
    SecretBoundaryVaultPickerState {
        matrix_row_id: PREVIEW_ROUTE_MATRIX_ROW_ID.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        picker_label: "Preview route auth picker".to_owned(),
        options: vec![
            SecretBoundaryVaultPickerOption {
                option_id: "preview-route:delegated".to_owned(),
                option_label: "Delegated preview identity".to_owned(),
                source_class: SecretBoundaryCredentialMode::Delegated,
                storage_class: SecretBoundaryStorageClass::SessionOnly,
                access_scope_label: "Preview route session".to_owned(),
                reveal_policy_label: "No raw token reveal".to_owned(),
                portability_note: "Signed links and session handles stay redacted.".to_owned(),
                open_source_of_truth_action_label: "Open route review".to_owned(),
                selectable: true,
            },
            SecretBoundaryVaultPickerOption {
                option_id: "preview-route:browser".to_owned(),
                option_label: "Browser handoff".to_owned(),
                source_class: SecretBoundaryCredentialMode::BrowserHandoff,
                storage_class: SecretBoundaryStorageClass::SessionOnly,
                access_scope_label: "Step-up route share".to_owned(),
                reveal_policy_label: "No raw callback export".to_owned(),
                portability_note: "Callback and signed-link material stay excluded.".to_owned(),
                open_source_of_truth_action_label: "Open browser handoff detail".to_owned(),
                selectable: true,
            },
        ],
    }
}

fn route_credential_mode(auth_source: AuthSourceClass) -> SecretBoundaryCredentialMode {
    match auth_source {
        AuthSourceClass::NoAuthLoopbackOnly => SecretBoundaryCredentialMode::SessionOnly,
        AuthSourceClass::WorkspaceSessionAuth => SecretBoundaryCredentialMode::HandleOnly,
        AuthSourceClass::OrganizationSso => SecretBoundaryCredentialMode::Delegated,
        AuthSourceClass::SignedPreviewLink => SecretBoundaryCredentialMode::BrowserHandoff,
        AuthSourceClass::MachineToMachineAllowlist => {
            SecretBoundaryCredentialMode::RemoteVaultFetch
        }
        AuthSourceClass::ExternalAuthPassthrough => SecretBoundaryCredentialMode::Delegated,
        AuthSourceClass::ApprovalTicketRequired => SecretBoundaryCredentialMode::Delegated,
        AuthSourceClass::AuthUnknownRequiresReview => SecretBoundaryCredentialMode::NotConfigured,
    }
}

fn route_storage_class(auth_source: AuthSourceClass) -> SecretBoundaryStorageClass {
    match auth_source {
        AuthSourceClass::SignedPreviewLink
        | AuthSourceClass::WorkspaceSessionAuth
        | AuthSourceClass::OrganizationSso
        | AuthSourceClass::ApprovalTicketRequired
        | AuthSourceClass::ExternalAuthPassthrough
        | AuthSourceClass::NoAuthLoopbackOnly => SecretBoundaryStorageClass::SessionOnly,
        AuthSourceClass::MachineToMachineAllowlist => SecretBoundaryStorageClass::RemoteVault,
        AuthSourceClass::AuthUnknownRequiresReview => SecretBoundaryStorageClass::NotConfigured,
    }
}

fn route_projection_mode(auth_source: AuthSourceClass) -> SecretBoundaryProjectionMode {
    match auth_source {
        AuthSourceClass::SignedPreviewLink => SecretBoundaryProjectionMode::BrowserHandoff,
        AuthSourceClass::MachineToMachineAllowlist => {
            SecretBoundaryProjectionMode::RemoteVaultFetch
        }
        AuthSourceClass::OrganizationSso
        | AuthSourceClass::ExternalAuthPassthrough
        | AuthSourceClass::ApprovalTicketRequired => SecretBoundaryProjectionMode::Delegated,
        _ => SecretBoundaryProjectionMode::HandleOnly,
    }
}

fn route_delegated_use(auth_source: AuthSourceClass) -> SecretBoundaryDelegatedUseClass {
    match auth_source {
        AuthSourceClass::MachineToMachineAllowlist => {
            SecretBoundaryDelegatedUseClass::RemoteVaultFetch
        }
        AuthSourceClass::OrganizationSso
        | AuthSourceClass::ExternalAuthPassthrough
        | AuthSourceClass::ApprovalTicketRequired => {
            SecretBoundaryDelegatedUseClass::ServiceIssuedDelegatedIdentity
        }
        _ => SecretBoundaryDelegatedUseClass::LocalSecretHandle,
    }
}

fn route_health_state(
    auth_source: AuthSourceClass,
    lifecycle_state: LifecycleState,
) -> SecretBoundaryHealthStateClass {
    match lifecycle_state {
        LifecycleState::Expired | LifecycleState::ApprovalExpired => {
            SecretBoundaryHealthStateClass::Expired
        }
        LifecycleState::Revoked => SecretBoundaryHealthStateClass::Revoked,
        LifecycleState::PolicyDenied | LifecycleState::CapabilityNarrowed => {
            SecretBoundaryHealthStateClass::PolicyBlocked
        }
        LifecycleState::SuspendedReconnect => SecretBoundaryHealthStateClass::ForwardingPaused,
        LifecycleState::ProviderUnavailable => {
            if matches!(auth_source, AuthSourceClass::MachineToMachineAllowlist) {
                SecretBoundaryHealthStateClass::RemoteVaultUnavailable
            } else {
                SecretBoundaryHealthStateClass::Unavailable
            }
        }
        LifecycleState::Blocked => SecretBoundaryHealthStateClass::Missing,
        LifecycleState::StaleTarget => SecretBoundaryHealthStateClass::Unavailable,
        _ if matches!(auth_source, AuthSourceClass::AuthUnknownRequiresReview) => {
            SecretBoundaryHealthStateClass::Missing
        }
        _ => SecretBoundaryHealthStateClass::Healthy,
    }
}

/// Proposed transition block for an [`ExposureReview`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProposedTransition {
    /// Controlled exposure label currently in effect.
    pub from_controlled_exposure_label: ControlledExposureLabel,
    /// Controlled exposure label the reviewer is being asked to admit.
    pub to_controlled_exposure_label: ControlledExposureLabel,
    /// Whether the transition is admitted by this review.
    pub transition_admitted: bool,
    /// Whether the transition is admitted only with narrowing.
    pub narrowing_applied: bool,
    /// Export-safe narrowing summary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub narrowing_summary: Option<String>,
}

/// Audience block for an [`ExposureReview`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewAudienceBlock {
    pub audience_class: AudienceClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audience_ref: Option<String>,
    pub audience_summary: String,
}

/// Data sensitivity block for an [`ExposureReview`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataSensitivityBlock {
    pub data_sensitivity_class: DataSensitivityClass,
    pub data_sensitivity_summary: String,
}

/// Cookie/session block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CookieSessionBlock {
    pub cookie_session_class: CookieSessionClass,
    pub summary: String,
}

/// Idle timeout block. `idle_timeout_seconds` is required when widening to
/// a public route; the validator enforces that rule.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdleTimeoutBlock {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idle_timeout_seconds: Option<u64>,
    pub summary: String,
}

/// Reachability block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReachabilityBlock {
    pub reachability_local_class: ReachabilityLocalClass,
    pub reachability_public_class: ReachabilityPublicClass,
    pub summary: String,
}

/// Lingering local preview block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LingeringLocalPreviewBlock {
    pub lingering_local_preview_class: LingeringLocalPreviewClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_preview_handle_ref: Option<String>,
    pub summary: String,
}

/// Viewer-state truth block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ViewerStateTruthBlock {
    pub viewer_state_class: ViewerStateClass,
    pub summary: String,
}

/// Cross-origin/browser-handoff block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CrossOriginBlock {
    pub cross_origin_class: CrossOriginClass,
    pub system_browser_required: bool,
    pub summary: String,
}

/// Canonical exposure-review record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExposureReview {
    pub record_kind: String,
    pub exposure_review_schema_version: u32,
    pub review_id: String,
    pub route_id: String,
    pub captured_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewer_ref: Option<String>,
    pub review_outcome: ReviewOutcomeClass,
    pub proposed_transition: ProposedTransition,
    pub audience: ReviewAudienceBlock,
    pub data_sensitivity: DataSensitivityBlock,
    pub cookie_session_behavior: CookieSessionBlock,
    pub idle_timeout: IdleTimeoutBlock,
    pub reachability: ReachabilityBlock,
    pub lingering_local_preview: LingeringLocalPreviewBlock,
    pub viewer_state_truth: ViewerStateTruthBlock,
    pub cross_origin_disclosure: CrossOriginBlock,
    pub summary: String,
}

/// Typed validation finding for an [`ExposureReview`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExposureReviewFinding {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

impl ExposureReviewFinding {
    fn new(
        check_id: impl Into<String>,
        subject_ref: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            check_id: check_id.into(),
            subject_ref: subject_ref.into(),
            message: message.into(),
        }
    }
}

impl ExposureReview {
    /// Returns typed truth-rule findings; an empty vector means the sheet is
    /// internally consistent with the schema's allOf rules.
    pub fn validate(&self) -> Vec<ExposureReviewFinding> {
        let mut findings = Vec::new();
        let subject = self.review_id.as_str();

        if self.record_kind != EXPOSURE_REVIEW_RECORD_KIND {
            findings.push(ExposureReviewFinding::new(
                "exposure_review.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    EXPOSURE_REVIEW_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.exposure_review_schema_version != EXPOSURE_REVIEW_SCHEMA_VERSION {
            findings.push(ExposureReviewFinding::new(
                "exposure_review.schema_version",
                subject,
                format!(
                    "exposure_review_schema_version must be {EXPOSURE_REVIEW_SCHEMA_VERSION}, found {}",
                    self.exposure_review_schema_version
                ),
            ));
        }

        if matches!(
            self.review_outcome,
            ReviewOutcomeClass::Denied | ReviewOutcomeClass::BlockedPendingPolicyReview
        ) && self.proposed_transition.transition_admitted
        {
            findings.push(ExposureReviewFinding::new(
                "exposure_review.denied_but_admitted",
                subject,
                "denied or blocked reviews must not admit the proposed transition",
            ));
        }

        if self.proposed_transition.to_controlled_exposure_label
            == ControlledExposureLabel::PublicRoute
        {
            if !matches!(
                self.reachability.reachability_public_class,
                ReachabilityPublicClass::PublicEphemeralWithTtl
                    | ReachabilityPublicClass::PublicViaSignedLink
                    | ReachabilityPublicClass::PublicBlockedByPolicy
            ) {
                findings.push(ExposureReviewFinding::new(
                    "exposure_review.public_reachability",
                    subject,
                    "public widening must declare a public reachability class",
                ));
            }
            match self.idle_timeout.idle_timeout_seconds {
                Some(secs) if secs > 0 => {}
                _ => {
                    findings.push(ExposureReviewFinding::new(
                        "exposure_review.public_idle_timeout",
                        subject,
                        "public widening must declare a positive idle_timeout_seconds",
                    ));
                }
            }
        }

        if self.proposed_transition.narrowing_applied
            && self.review_outcome != ReviewOutcomeClass::ApprovedWithNarrowing
        {
            findings.push(ExposureReviewFinding::new(
                "exposure_review.narrowing_outcome_mismatch",
                subject,
                "narrowing_applied requires review_outcome=approved_with_narrowing",
            ));
        }
        if self.proposed_transition.narrowing_applied
            && self.proposed_transition.narrowing_summary.is_none()
        {
            findings.push(ExposureReviewFinding::new(
                "exposure_review.narrowing_summary_required",
                subject,
                "narrowing_applied requires a narrowing_summary",
            ));
        }

        findings
    }
}

/// Stable revocation summary derived from a [`RouteObject`]. UI rows,
/// audits, and support exports share this projection so the answer to
/// "what was exposed, to whom, for how long, and under what auth/expiry?"
/// is the same everywhere.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RevocationSummary {
    pub route_id: String,
    pub controlled_exposure_label: ControlledExposureLabel,
    pub audience_class: AudienceClass,
    pub teardown_state: TeardownState,
    pub revoke_posture_class: RevokePostureClass,
    pub auth_source_class: AuthSourceClass,
    pub expires_at: Option<String>,
    pub last_access_class: LastAccessClass,
    pub last_access_at: Option<String>,
    pub stale_shared_link_state: StaleSharedLinkState,
    pub reopen_class: ReopenClass,
    pub affected_link_refs: Vec<String>,
    pub session_impact_summary: String,
    pub summary: String,
}

impl RevocationSummary {
    /// Derives a stable revocation summary from a [`RouteObject`].
    pub fn from_route(route: &RouteObject) -> Self {
        Self {
            route_id: route.route_id.clone(),
            controlled_exposure_label: route.controlled_exposure_label,
            audience_class: route.audience.audience_class,
            teardown_state: route.revocation.teardown_state,
            revoke_posture_class: route.revocation.revoke_posture_class,
            auth_source_class: route.auth.auth_source_class,
            expires_at: route.expiry.expires_at.clone(),
            last_access_class: route.last_access.last_access_class,
            last_access_at: route.last_access.last_access_at.clone(),
            stale_shared_link_state: route.revocation.stale_shared_link_state,
            reopen_class: route.revocation.reopen_class,
            affected_link_refs: route.revocation.affected_link_refs.clone(),
            session_impact_summary: route.revocation.session_impact_summary.clone(),
            summary: route.revocation.summary.clone(),
        }
    }
}

#[cfg(test)]
mod tests;
