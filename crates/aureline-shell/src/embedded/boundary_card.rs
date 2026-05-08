//! Render-side embedded boundary card contract types.
//!
//! The boundary card is host-rendered chrome painted next to embedded surfaces
//! (docs/help panes, marketplace/account pages, service dashboards, auth
//! confirmation sheets, extension-hosted web-like surfaces). It exposes owner,
//! origin, data boundary, permission state, action partition, and browser
//! fallback posture using a closed vocabulary shared across the product.

use serde::{Deserialize, Serialize};

/// One embedded-surface boundary card record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmbeddedBoundaryCardRecord {
    pub record_kind: String,
    pub embedded_boundary_card_schema_version: u32,
    pub card_id: String,
    pub surface_id_ref: String,
    pub surface_family: SurfaceFamily,
    pub owner_identity: OwnerIdentityRecord,
    pub publisher_or_service_identity: PublisherOrServiceIdentityRecord,
    pub origin_identity: OriginIdentityRecord,
    pub data_boundary_class: DataBoundaryClass,
    pub data_boundary_label: String,
    pub boundary_state: BoundaryState,
    pub boundary_state_label: String,
    pub plain_language_summary: String,
    pub permission_state: PermissionStateRecord,
    pub action_partition: Vec<ActionPartitionRecord>,
    pub browser_fallback: BrowserFallbackRecord,
    pub capability_limitations: Vec<CapabilityLimitation>,
    pub reserved_native_surfaces_host_owned: Vec<NativeReservedSurface>,
    pub layout_constraints: Vec<LayoutConstraintId>,
    pub chrome_inheritance_axes: Vec<ChromeInheritanceAxis>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_truth: Option<SourceTruthRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provider_identity: Option<ProviderIdentityRecord>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth_handoff: Option<AuthHandoffCardRecord>,
    pub policy_context: PolicyContext,
    pub redaction_class: RedactionClass,
    pub minted_at: String,
}

impl EmbeddedBoundaryCardRecord {
    /// Returns the first action partition row with action id `open_in_system_browser`.
    pub fn open_in_browser_action(&self) -> Option<&ActionPartitionRecord> {
        self.action_partition
            .iter()
            .find(|row| row.action_id == BoundaryActionId::OpenInSystemBrowser)
    }
}

/// Closed embedded-surface family vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SurfaceFamily {
    EmbeddedDocsHelp,
    EmbeddedMarketplaceOrAccount,
    EmbeddedServiceDashboard,
    EmbeddedAuthConfirmation,
    ExtensionHostedSurface,
}

/// Human owner of the embedded surface as rendered by the host shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OwnerClass {
    HostProduct,
    FirstPartyProject,
    ExtensionBundle,
    ConnectedProvider,
    EnterpriseAdmin,
    CustomerServiceOwner,
    UnknownOwner,
}

/// Publisher / provider / service identity distinct from the host product.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublisherOrServiceClass {
    FirstPartyProject,
    MarketplaceService,
    ConnectedProviderService,
    CustomerService,
    ExtensionPublisher,
    IdentityProvider,
    UnknownPublisherOrService,
}

/// Closed origin-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginClass {
    LocalPackOrArtifact,
    FirstPartyHostedWeb,
    ConnectedProviderHostedWeb,
    CustomerOrEnterpriseHostedWeb,
    ExtensionPublisherHostedWeb,
    CrossOriginSubframe,
    SystemBrowserReturn,
    UnknownOriginClass,
}

/// Origin verification state token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OriginVerificationState {
    Verified,
    Unverified,
    CertificateFailed,
    PolicyBlocked,
    CrossOriginLimited,
    OfflineCached,
}

/// Closed data-boundary vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DataBoundaryClass {
    LocalProductBoundary,
    FirstPartyHostedServiceBoundary,
    ConnectedProviderBoundary,
    CustomerControlPlaneBoundary,
    ExtensionPublisherBoundary,
    CrossOriginLimitedBoundary,
}

/// Closed embedded-surface boundary-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryState {
    LiveVerified,
    StaleSnapshot,
    PolicyBlocked,
    CertificateFailed,
    CrossOriginLimited,
    OfflineSnapshot,
    ExternalOpenOnly,
}

/// Closed permission-state vocabulary describing host authority over the embedded surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionClass {
    HostOwnedFullAuthority,
    HostOwnedInspectOnly,
    HostOwnedBrowserOnly,
    HostOwnedCopyExportOnly,
    HostOwnedWithNativeStepUpRequired,
    EmbeddedLowerTrustSessionRefresh,
    EmbeddedLowerTrustPasswordException,
    NoPermissionWithinProduct,
}

/// Closed host-owned boundary action vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BoundaryActionId {
    ReloadEmbeddedSurface,
    OpenInSystemBrowser,
    SwitchToDeviceCode,
    CopyDeviceCode,
    RetryAuthHandoff,
    InspectCertificateDetails,
    InspectPolicyReason,
    ContinueLocalWithoutSurface,
    OpenSupportEvidence,
    ViewAuthExceptionRecord,
}

/// Closed action-partition vocabulary describing where an action is owned/rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionPartitionRole {
    ProductOwnedNative,
    ProductOwnedHandoff,
    EmbeddedInspectOnly,
    EmbeddedRequestOnly,
}

/// Closed browser-fallback posture vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BrowserFallbackPostureClass {
    SystemBrowserFirst,
    DeviceCodeFallbackOffered,
    ExternalOpenBlockedByPolicy,
    ExternalOpenUnavailableOffline,
    BrowserFallbackNotApplicable,
}

/// Closed fallback-target vocabulary naming where the user is sent when the embedded body is not trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackTargetClass {
    SystemBrowserHandoffPacket,
    DeviceCodeCompanionCard,
    PlatformAuthenticatorNative,
    HostNativeReviewOrApproval,
    LocalInspectOrExport,
    NoFallbackAvailable,
}

/// Closed capability-limitation vocabulary rendered on the boundary card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityLimitation {
    CannotIssueNativeApproval,
    CannotVerifyUpdatesOrSignatures,
    CannotRaiseWorkspaceTrust,
    CannotPerformRollbackOrRestore,
    CannotApplyAiChanges,
    CookiesOrStorageOutsideProductBoundary,
    CrossOriginDomOrStorageHidden,
    LiveNetworkMutationDisabledWhenOffline,
    ProviderScopeMayBeNarrowerThanPageClaims,
    EmbeddedAuthLowerTrust,
}

/// Native-reserved surfaces that embedded content may never host or imitate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NativeReservedSurface {
    ProductSecurityMessaging,
    UpdateVerification,
    WorkspaceTrustElevation,
    RollbackOrRestoreConfirmation,
    AiApplyReview,
    HighRiskApprovalSheet,
}

/// Docs/help freshness class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    AuthoritativeLive,
    WarmCached,
    DegradedCached,
    Stale,
    Unverified,
}

/// Docs/help version-match-state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionMatchState {
    ExactBuildMatch,
    CompatibleMinorDrift,
    IncompatibleDriftDetected,
    PreReleaseUnverified,
    UnknownTargetBuild,
}

/// Docs/help source-of-truth class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    ProjectDocs,
    GeneratedReference,
    MirroredOfficialDocs,
    CuratedKnowledgePack,
    DerivedExplanation,
    VendorProviderDocs,
    SupportRunbook,
    ExternalStatusFeed,
}

/// Connected-provider class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderClass {
    ReviewOrCodeHost,
    IssueOrPlanningTracker,
    CiOrCheckProvider,
    DocsOrPortalProvider,
    IdentityOrEnterpriseProvider,
    CallbackOrEventProvider,
    AiProvider,
    PackageRegistryProvider,
    ReleasePublisherProvider,
    ManagedAdminProvider,
}

/// Connected-provider actor class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderActorClass {
    HumanAccount,
    InstallationOrAppGrant,
    DelegatedUserToken,
    ProjectScopedGrant,
    PolicyInjectedServiceIdentity,
    UnknownActorClass,
}

/// Connected-provider health-state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderHealthState {
    Healthy,
    Degraded,
    Unavailable,
    Revoked,
    Suspended,
    Expired,
}

/// Redaction class for support/export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionClass {
    MetadataSafeDefault,
    OperatorOnlyRestricted,
    InternalSupportRestricted,
    SigningEvidenceOnly,
}

/// Layout constraints the host shell must satisfy when painting the boundary card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LayoutConstraintId {
    CardVisuallyDistinctFromEmbeddedBody,
    CardNotObscuredByEmbeddedBody,
    CardRequiredFieldsNeverHoverOnly,
    CardCompactLayoutPreservesRequiredFields,
    CardActionsRenderInHostChrome,
    EmbeddedBodyCannotOverlapCardActions,
    CardRemainsVisibleWhenEmbeddedBodyIsWithheld,
}

/// Chrome axes the embedded body inherits from the host shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChromeInheritanceAxis {
    ThemePaletteInheritsFromHost,
    DensityClassInheritsFromHost,
    ZoomLevelInheritsFromHost,
    FocusRingInheritsFromHost,
    ReducedMotionPostureInheritsFromHost,
    HighContrastModeInheritsFromHost,
    ForcedColorsModeInheritsFromHost,
}

/// Policy, identity-mode, and trust context the boundary card was minted under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PolicyContext {
    pub identity_mode: IdentityMode,
    pub policy_epoch: String,
    pub trust_state: TrustState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_context_id: Option<String>,
}

/// Identity-mode vocabulary for policy context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityMode {
    AccountFreeLocal,
    SelfHostedOrg,
    ManagedWorkspace,
}

/// Workspace trust-state vocabulary for policy context.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustState {
    Trusted,
    Restricted,
}

/// Human-readable owner identity plus its closed owner-class token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OwnerIdentityRecord {
    pub label: String,
    pub class: OwnerClass,
}

/// Human-readable publisher/service identity plus its closed class token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublisherOrServiceIdentityRecord {
    pub label: String,
    pub class: PublisherOrServiceClass,
}

/// Host-owned origin identity disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OriginIdentityRecord {
    pub origin_class: OriginClass,
    pub origin_label: String,
    pub verification_state: OriginVerificationState,
    pub host_or_domain_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_ref: Option<String>,
}

/// Permission-state row carried on the boundary card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionStateRecord {
    pub permission_class: PermissionClass,
    pub permission_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub host_native_step_up_required: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception_id_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_narrowing_summary: Option<String>,
}

/// One row in the action partition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActionPartitionRecord {
    pub action_id: BoundaryActionId,
    pub partition_role: ActionPartitionRole,
    pub action_label: String,
    pub renders_in_host_chrome: bool,
    pub preserves_object_identity: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
}

/// Browser-fallback posture cue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BrowserFallbackRecord {
    pub posture_class: BrowserFallbackPostureClass,
    pub fallback_target_class: FallbackTargetClass,
    pub summary_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device_code_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_reason_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub return_target_label: Option<String>,
}

/// Quoted docs/help source-version-freshness truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceTruthRecord {
    pub source_class: SourceClass,
    pub version_match_state: VersionMatchState,
    pub freshness_class: FreshnessClass,
    pub running_build_identity_ref: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_status_badge_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub snapshot_age_label: Option<String>,
}

/// Provider/service identity chrome for marketplace/account and service-dashboard surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderIdentityRecord {
    pub provider_class: ProviderClass,
    pub provider_label: String,
    pub provider_scope_label: String,
    pub provider_actor_class: ProviderActorClass,
    pub health_state: ProviderHealthState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub connected_provider_record_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub health_summary_label: Option<String>,
}

/// Render-side projection of the auth-handoff cue family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthHandoffCardRecord {
    pub flow_class: AuthFlowClass,
    pub provider_domain_label: String,
    pub reason_label: String,
    pub return_target_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_continuity_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code_expiry_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exception_id_ref: Option<String>,
}

/// Closed embedded/auth handoff flow-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthFlowClass {
    NotApplicable,
    SystemBrowser,
    DeviceCode,
    PlatformAuthenticatorNative,
    EmbeddedSessionRefresh,
    EmbeddedPasswordException,
}
