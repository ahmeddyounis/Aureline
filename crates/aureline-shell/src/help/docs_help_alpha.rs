//! Searchable docs/help alpha catalog with About and service-health truth rows.
//!
//! The catalog is a shell-side projection over the command registry,
//! docs-pack identity, destination descriptors, and the existing Help / About
//! surface. It keeps help search, About cards, service-health notes, stale
//! examples, docs suggestions, and browser handoffs on one exportable record
//! family.

use aureline_commands::invocation::now_rfc3339;
use aureline_commands::{CommandRegistry, CommandRegistryEntryRecord};
use serde::{Deserialize, Serialize};

use crate::embedded::boundary_card::{FreshnessClass, SourceClass, VersionMatchState};
use crate::help_about::HelpAboutSurface;

/// Stable record-kind tag for [`HelpAlphaCatalog`] payloads.
pub const HELP_ALPHA_CATALOG_RECORD_KIND: &str = "docs_help_alpha_catalog_record";

/// Stable schema version for docs/help alpha catalog payloads.
pub const HELP_ALPHA_CATALOG_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`HelpAlphaSupportExport`] payloads.
pub const HELP_ALPHA_SUPPORT_EXPORT_RECORD_KIND: &str = "docs_help_alpha_support_export_record";

/// Stable schema version for docs/help alpha support-export payloads.
pub const HELP_ALPHA_SUPPORT_EXPORT_SCHEMA_VERSION: u32 = 1;

/// Classifies the kind of material a help row is presenting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaContentClass {
    /// Canonical help copied from a governed docs pack or command descriptor.
    CanonicalHelp,
    /// Generated docs suggestion that must remain evidence-backed.
    GeneratedSuggestion,
    /// Stale example or stale screenshot warning.
    StaleExampleWarning,
    /// Handoff row that crosses a publish, browser, or external-console boundary.
    PublishBoundaryHandoff,
    /// Migration guidance tied to a canonical command or docs pack.
    MigrationHint,
    /// Service-health note from the shared health or runbook truth lane.
    ServiceHealthNote,
    /// About or service-health descriptor row for the running build.
    AboutServiceHealthTruth,
}

impl HelpAlphaContentClass {
    /// Returns the stable token for this content class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalHelp => "canonical_help",
            Self::GeneratedSuggestion => "generated_suggestion",
            Self::StaleExampleWarning => "stale_example_warning",
            Self::PublishBoundaryHandoff => "publish_boundary_handoff",
            Self::MigrationHint => "migration_hint",
            Self::ServiceHealthNote => "service_health_note",
            Self::AboutServiceHealthTruth => "about_service_health_truth",
        }
    }

    /// Returns the compact label shown on result cards.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CanonicalHelp => "Canonical help",
            Self::GeneratedSuggestion => "Generated suggestion",
            Self::StaleExampleWarning => "Stale example",
            Self::PublishBoundaryHandoff => "Publish handoff",
            Self::MigrationHint => "Migration hint",
            Self::ServiceHealthNote => "Service note",
            Self::AboutServiceHealthTruth => "About truth",
        }
    }
}

/// Support commitment shown separately from lifecycle and freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaSupportClass {
    /// Evidence-backed certified claim.
    Certified,
    /// Supported alpha or product claim.
    Supported,
    /// Limited support for a narrowed surface or stale source.
    Limited,
    /// Community-supported route or content.
    Community,
    /// Experimental or preview-only support.
    Experimental,
}

impl HelpAlphaSupportClass {
    /// Returns the stable token for this support class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::Supported => "supported",
            Self::Limited => "limited",
            Self::Community => "community",
            Self::Experimental => "experimental",
        }
    }
}

/// Client scopes that may render or claim a help row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaClientScope {
    /// Native desktop product.
    DesktopProduct,
    /// CLI or headless command path.
    Cli,
    /// Browser or lightweight companion surface.
    CompanionSurface,
    /// Remote agent surface.
    RemoteAgent,
    /// SDK or API consumer.
    SdkOrApi,
    /// Managed admin surface.
    ManagedAdminSurface,
}

impl HelpAlphaClientScope {
    /// Returns the stable token for this client scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopProduct => "desktop_product",
            Self::Cli => "cli",
            Self::CompanionSurface => "companion_surface",
            Self::RemoteAgent => "remote_agent",
            Self::SdkOrApi => "sdk_or_api",
            Self::ManagedAdminSurface => "managed_admin_surface",
        }
    }
}

/// Destination trust class for a help or service-health route.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaDestinationTrustClass {
    /// First-party public destination.
    OfficialPublic,
    /// First-party authenticated destination.
    OfficialAuthenticated,
    /// Community-owned destination.
    Community,
    /// Local-only product boundary.
    LocalOnly,
}

impl HelpAlphaDestinationTrustClass {
    /// Returns the stable token for this trust class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialPublic => "official_public",
            Self::OfficialAuthenticated => "official_authenticated",
            Self::Community => "community",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Contract state shared by About, help, and service-health rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaContractState {
    /// Current contract is ready for the row's declared scope.
    Ready,
    /// Row is usable but degraded.
    Degraded,
    /// Row has narrowed to local-only continuity.
    LocalOnly,
    /// Row is useful as a stale snapshot only.
    Stale,
    /// The source contract no longer matches the consumer.
    ContractMismatch,
    /// Policy blocks the route or row.
    PolicyBlocked,
    /// No usable descriptor is available.
    Unavailable,
}

impl HelpAlphaContractState {
    /// Returns the stable token for this contract state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::LocalOnly => "local_only",
            Self::Stale => "stale",
            Self::ContractMismatch => "contract_mismatch",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Publish-boundary state for docs/help and service-health material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaPublishBoundaryState {
    /// Content is local-only and not publication-bearing.
    LocalOnlyAuthoring,
    /// Content can be reviewed locally but is not automatically published.
    LocalReviewReady,
    /// A publish-boundary handoff is required before publication.
    PublishBoundaryHandoff,
    /// Handoff is blocked until validation or owner review clears.
    BlockedPendingValidation,
    /// External console or browser fallback owns the next step.
    ExternalConsoleFallback,
    /// The row is not a docs-publish-bearing object.
    NotPublishBearing,
}

impl HelpAlphaPublishBoundaryState {
    /// Returns the stable token for this publish-boundary state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnlyAuthoring => "local_only_authoring",
            Self::LocalReviewReady => "local_review_ready",
            Self::PublishBoundaryHandoff => "publish_boundary_handoff",
            Self::BlockedPendingValidation => "blocked_pending_validation",
            Self::ExternalConsoleFallback => "external_console_fallback",
            Self::NotPublishBearing => "not_publish_bearing",
        }
    }
}

/// Citation posture for a help row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaCitationAvailability {
    /// Citation anchors are available and exposed through a detail target.
    Available,
    /// Citation anchors are required but missing.
    Missing,
    /// The row does not require citations.
    NotRequired,
}

impl HelpAlphaCitationAvailability {
    /// Returns the stable token for this citation posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Missing => "missing",
            Self::NotRequired => "not_required",
        }
    }
}

/// Locale availability state for source descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaLocaleAvailability {
    /// The requested locale is authoritative.
    RequestedLocaleAuthoritative,
    /// The requested locale is partially available.
    RequestedLocalePartial,
    /// The source falls back to the primary language.
    RequestedLocaleMissingFallbackToPrimary,
    /// Locale does not apply to the row.
    LocaleNotApplicableToRow,
}

impl HelpAlphaLocaleAvailability {
    /// Returns the stable token for this locale state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedLocaleAuthoritative => "requested_locale_authoritative",
            Self::RequestedLocalePartial => "requested_locale_partial",
            Self::RequestedLocaleMissingFallbackToPrimary => {
                "requested_locale_missing_fallback_to_primary"
            }
            Self::LocaleNotApplicableToRow => "locale_not_applicable_to_row",
        }
    }
}

/// Source-language fallback posture for partially localized rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaSourceLanguageFallback {
    /// Requested locale is complete enough that no fallback is needed.
    NotNeeded,
    /// The canonical source language is available as an escape hatch.
    SourceLanguageFallbackAvailable,
    /// The row is partial but source-language fallback is unavailable.
    SourceLanguageFallbackUnavailable,
    /// Language fallback does not apply to the row.
    NotApplicable,
}

impl HelpAlphaSourceLanguageFallback {
    /// Returns the stable token for this fallback posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNeeded => "not_needed",
            Self::SourceLanguageFallbackAvailable => "source_language_fallback_available",
            Self::SourceLanguageFallbackUnavailable => "source_language_fallback_unavailable",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Offline posture for a docs/help descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaOfflinePosture {
    /// Live source is currently authoritative.
    LiveOnline,
    /// Cached content remains inside the accepted freshness window.
    WarmCached,
    /// Signed mirror is available offline.
    MirroredOffline,
    /// Local-only descriptor remains useful without a service.
    AirGappedLocalOnly,
    /// No useful offline copy is available.
    UnavailableOffline,
}

impl HelpAlphaOfflinePosture {
    /// Returns the stable token for this offline posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveOnline => "live_online",
            Self::WarmCached => "warm_cached",
            Self::MirroredOffline => "mirrored_offline",
            Self::AirGappedLocalOnly => "air_gapped_local_only",
            Self::UnavailableOffline => "unavailable_offline",
        }
    }
}

/// Allowed browser-handoff reasons for this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaBrowserHandoffReason {
    /// Authoritative docs or runbook content lives outside the product.
    ExternalDocsOrRunbook,
    /// Provider consent must happen outside the product.
    ProviderConsentFlow,
    /// Provider admin delegation must happen outside the product.
    ProviderAdminDelegation,
    /// License or portal acceptance must happen outside the product.
    LicenseOrPortalAcceptance,
    /// Admin-only surface must handle the route.
    AdminOnlySurface,
    /// Step-up authentication is required outside the product boundary.
    StepUpRequired,
    /// The product can inspect state but cannot perform the mutation.
    MutationNotSupportedInProduct,
}

impl HelpAlphaBrowserHandoffReason {
    /// Returns the stable token for this handoff reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExternalDocsOrRunbook => "external_docs_or_runbook",
            Self::ProviderConsentFlow => "provider_consent_flow",
            Self::ProviderAdminDelegation => "provider_admin_delegation",
            Self::LicenseOrPortalAcceptance => "license_or_portal_acceptance",
            Self::AdminOnlySurface => "admin_only_surface",
            Self::StepUpRequired => "step_up_required",
            Self::MutationNotSupportedInProduct => "mutation_not_supported_in_product",
        }
    }
}

/// Target kind used to reopen or inspect a help row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HelpAlphaTargetKind {
    /// Reopen the owning command descriptor.
    Command,
    /// Reopen a docs node or docs-pack anchor.
    DocsNode,
    /// Reopen a product surface.
    ProductSurface,
    /// Follow a governed browser handoff packet.
    BrowserHandoff,
    /// Open the citation drawer.
    CitationDrawer,
    /// Open an evidence card.
    EvidenceCard,
}

impl HelpAlphaTargetKind {
    /// Returns the stable token for this target kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Command => "command",
            Self::DocsNode => "docs_node",
            Self::ProductSurface => "product_surface",
            Self::BrowserHandoff => "browser_handoff",
            Self::CitationDrawer => "citation_drawer",
            Self::EvidenceCard => "evidence_card",
        }
    }
}

/// Locale disclosure embedded in a source descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaLocaleState {
    /// Canonical source locale.
    pub primary_locale: String,
    /// Locale requested by the rendering surface.
    pub requested_locale: String,
    /// Locale actually rendered.
    pub effective_locale: String,
    /// Availability state for the requested locale.
    pub availability: HelpAlphaLocaleAvailability,
    /// Stable token for [`Self::availability`].
    pub availability_token: String,
    /// Source-language fallback posture.
    pub source_language_fallback: HelpAlphaSourceLanguageFallback,
    /// Stable token for [`Self::source_language_fallback`].
    pub source_language_fallback_token: String,
}

/// Offline and local-only disclosure embedded in a source descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaOfflineState {
    /// Offline posture for this row.
    pub posture: HelpAlphaOfflinePosture,
    /// Stable token for [`Self::posture`].
    pub posture_token: String,
    /// Optional last refresh timestamp or age label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refreshed_at: Option<String>,
    /// Optional offline expiry timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_expiration_at: Option<String>,
    /// Optional local-only limit shown while offline.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local_only_continuity_limit: Option<String>,
}

/// Reopen or detail target for a help row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaTarget {
    /// Target kind.
    pub target_kind: HelpAlphaTargetKind,
    /// Stable token for [`Self::target_kind`].
    pub target_kind_token: String,
    /// Stable target ref.
    pub target_ref: String,
    /// Exact reopen ref preserved in support exports.
    pub exact_reopen_ref: String,
    /// Owning command id when the target is command-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_id: Option<String>,
    /// Destination descriptor ref when a route or handoff is involved.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_descriptor_ref: Option<String>,
    /// Browser-handoff reason when the target crosses a browser boundary.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason: Option<HelpAlphaBrowserHandoffReason>,
    /// Stable token for [`Self::browser_handoff_reason`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason_token: Option<String>,
}

/// Citation state attached to a result or About/service-health row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaCitationState {
    /// Citation availability posture.
    pub availability: HelpAlphaCitationAvailability,
    /// Stable token for [`Self::availability`].
    pub availability_token: String,
    /// Anchor refs that support the row.
    pub citation_anchor_refs: Vec<String>,
    /// Details target that opens a citation drawer or evidence card.
    pub open_details_target: HelpAlphaTarget,
}

/// Shared source descriptor for help search and About/service-health rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaSourceDescriptor {
    /// Source class from the docs/help truth vocabulary.
    pub source_class: SourceClass,
    /// Stable token for [`Self::source_class`].
    pub source_class_token: String,
    /// Source object ref.
    pub source_ref: String,
    /// Source revision or manifest ref.
    pub source_revision_ref: String,
    /// Human-readable source version.
    pub display_source_version: String,
    /// Running build identity when the row is build-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub running_build_identity_ref: Option<String>,
    /// Build/version match state when the row is build-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_match_state: Option<VersionMatchState>,
    /// Stable token for [`Self::version_match_state`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version_match_token: Option<String>,
    /// Freshness class from the docs/help truth vocabulary.
    pub freshness_class: FreshnessClass,
    /// Stable token for [`Self::freshness_class`].
    pub freshness_class_token: String,
    /// Support class for the row.
    pub support_class: HelpAlphaSupportClass,
    /// Stable token for [`Self::support_class`].
    pub support_class_token: String,
    /// Client scopes allowed to claim this row.
    pub client_scopes: Vec<HelpAlphaClientScope>,
    /// Stable tokens for [`Self::client_scopes`].
    pub client_scope_tokens: Vec<String>,
    /// Destination trust class for linked routes.
    pub destination_trust_class: HelpAlphaDestinationTrustClass,
    /// Stable token for [`Self::destination_trust_class`].
    pub destination_trust_class_token: String,
    /// Contract state for About/help/service-health consumers.
    pub contract_state: HelpAlphaContractState,
    /// Stable token for [`Self::contract_state`].
    pub contract_state_token: String,
    /// Publish-boundary state for docs/help consumers.
    pub publish_boundary_state: HelpAlphaPublishBoundaryState,
    /// Stable token for [`Self::publish_boundary_state`].
    pub publish_boundary_state_token: String,
    /// Exact build or channel label shown on cards.
    pub exact_build_or_channel: String,
    /// Deployment mode shown on cards.
    pub deployment_mode: String,
    /// Provenance record ref shown on cards.
    pub provenance_ref: String,
    /// Docs-pack manifest ref, when a pack backs the row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_pack_manifest_ref: Option<String>,
    /// Help status badge ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub help_status_badge_ref: Option<String>,
    /// Destination descriptor ref, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_descriptor_ref: Option<String>,
    /// Browser-handoff reason, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason: Option<HelpAlphaBrowserHandoffReason>,
    /// Stable token for [`Self::browser_handoff_reason`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason_token: Option<String>,
    /// Locale disclosure for the row.
    pub locale: HelpAlphaLocaleState,
    /// Offline disclosure for the row.
    pub offline: HelpAlphaOfflineState,
}

/// One docs/help search result card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaSearchResult {
    /// Stable result id.
    pub result_id: String,
    /// Result title.
    pub title: String,
    /// Result summary.
    pub summary: String,
    /// Content class.
    pub content_class: HelpAlphaContentClass,
    /// Stable token for [`Self::content_class`].
    pub content_class_token: String,
    /// Stable docs node id when the result opens documentation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_node_id: Option<String>,
    /// Stable help anchor id.
    pub help_anchor_id: String,
    /// Owning command id when the result is command-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owning_command_id: Option<String>,
    /// Owning command revision when command-backed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_revision_ref: Option<String>,
    /// Product surface id when the result opens a surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_surface_id: Option<String>,
    /// Source descriptor rendered on the card.
    pub source: HelpAlphaSourceDescriptor,
    /// Exact reopen target for the primary action.
    pub exact_reopen: HelpAlphaTarget,
    /// Citation or evidence details.
    pub citation: HelpAlphaCitationState,
}

/// One About or service-health descriptor row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaAboutServiceRow {
    /// Stable row id.
    pub row_id: String,
    /// Row label.
    pub label: String,
    /// Row summary.
    pub summary: String,
    /// Content class.
    pub content_class: HelpAlphaContentClass,
    /// Stable token for [`Self::content_class`].
    pub content_class_token: String,
    /// Surface family that consumes the row.
    pub surface_class: String,
    /// True when sign-in is required before the row is useful.
    pub sign_in_required: bool,
    /// Workflows affected by the row.
    pub affected_workflows: Vec<String>,
    /// Local-only continuity notes.
    pub local_only_continuity: Vec<String>,
    /// Source descriptor rendered on the row.
    pub source: HelpAlphaSourceDescriptor,
    /// Exact reopen or handoff target.
    pub exact_reopen: HelpAlphaTarget,
    /// Citation or evidence details.
    pub citation: HelpAlphaCitationState,
}

/// Query request for the alpha help catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaSearchQuery {
    /// User-entered query text.
    pub query_text: String,
    /// Optional content classes to include.
    pub content_classes: Vec<HelpAlphaContentClass>,
    /// Optional client-scope filter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_scope: Option<HelpAlphaClientScope>,
}

impl HelpAlphaSearchQuery {
    /// Builds a query that searches all content classes.
    pub fn text(query_text: impl Into<String>) -> Self {
        Self {
            query_text: query_text.into(),
            content_classes: Vec::new(),
            client_scope: None,
        }
    }

    /// Limits the query to one content class.
    pub fn with_content_class(mut self, class: HelpAlphaContentClass) -> Self {
        self.content_classes.push(class);
        self
    }

    /// Limits the query to one client scope.
    pub fn with_client_scope(mut self, scope: HelpAlphaClientScope) -> Self {
        self.client_scope = Some(scope);
        self
    }
}

/// Search response that preserves the query, searched lanes, and results.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaSearchSnapshot {
    /// Query used to produce the snapshot.
    pub query: HelpAlphaSearchQuery,
    /// Source refs searched by the catalog.
    pub searched_source_refs: Vec<String>,
    /// Number of matching results.
    pub result_count: usize,
    /// Matching help results.
    pub results: Vec<HelpAlphaSearchResult>,
}

/// Support/export projection for one help search result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaSupportSearchResult {
    /// Stable result id.
    pub result_id: String,
    /// Content class token.
    pub content_class_token: String,
    /// Source class token.
    pub source_class_token: String,
    /// Source revision ref.
    pub source_revision_ref: String,
    /// Display source version.
    pub display_source_version: String,
    /// Freshness class token.
    pub freshness_class_token: String,
    /// Support class token.
    pub support_class_token: String,
    /// Client-scope tokens.
    pub client_scope_tokens: Vec<String>,
    /// Publish-boundary state token.
    pub publish_boundary_state_token: String,
    /// Stable help anchor id.
    pub help_anchor_id: String,
    /// Stable docs node id, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub docs_node_id: Option<String>,
    /// Owning command id, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owning_command_id: Option<String>,
    /// Citation availability token.
    pub citation_availability_token: String,
    /// Citation anchor refs.
    pub citation_anchor_refs: Vec<String>,
    /// Locale availability token.
    pub locale_availability_token: String,
    /// Source-language fallback token.
    pub source_language_fallback_token: String,
    /// Offline posture token.
    pub offline_posture_token: String,
    /// Exact reopen ref.
    pub exact_reopen_ref: String,
    /// Browser-handoff reason token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason_token: Option<String>,
}

/// Support/export projection for one About or service-health row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaSupportAboutServiceRow {
    /// Stable row id.
    pub row_id: String,
    /// Surface class.
    pub surface_class: String,
    /// Contract state token.
    pub contract_state_token: String,
    /// Source class token.
    pub source_class_token: String,
    /// Source revision ref.
    pub source_revision_ref: String,
    /// Display source version.
    pub display_source_version: String,
    /// Freshness class token.
    pub freshness_class_token: String,
    /// Destination trust class token.
    pub destination_trust_class_token: String,
    /// Publish-boundary state token.
    pub publish_boundary_state_token: String,
    /// Citation availability token.
    pub citation_availability_token: String,
    /// Citation anchor refs.
    pub citation_anchor_refs: Vec<String>,
    /// Locale availability token.
    pub locale_availability_token: String,
    /// Source-language fallback token.
    pub source_language_fallback_token: String,
    /// Offline posture token.
    pub offline_posture_token: String,
    /// Exact reopen ref.
    pub exact_reopen_ref: String,
    /// Browser-handoff reason token, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_reason_token: Option<String>,
}

/// Export-safe payload for support bundles and community handoff packets.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export id.
    pub export_id: String,
    /// Source catalog id.
    pub catalog_id: String,
    /// Running build identity ref.
    pub running_build_identity_ref: String,
    /// Timestamp when the export was projected.
    pub generated_at: String,
    /// Exported search results.
    pub search_results: Vec<HelpAlphaSupportSearchResult>,
    /// Exported About and service-health rows.
    pub about_service_rows: Vec<HelpAlphaSupportAboutServiceRow>,
}

/// Searchable docs/help alpha catalog.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaCatalog {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable catalog id.
    pub catalog_id: String,
    /// Running build identity ref.
    pub running_build_identity_ref: String,
    /// Release channel shown in the catalog.
    pub release_channel: String,
    /// Timestamp when the catalog was projected.
    pub generated_at: String,
    /// Searchable help results.
    pub search_results: Vec<HelpAlphaSearchResult>,
    /// About and service-health truth rows.
    pub about_service_rows: Vec<HelpAlphaAboutServiceRow>,
}

impl HelpAlphaCatalog {
    /// Builds the seeded catalog from the command registry and a build identity.
    pub fn from_command_registry(
        registry: &CommandRegistry,
        build_identity_ref: impl Into<String>,
    ) -> Self {
        let build_identity_ref = build_identity_ref.into();
        Self {
            record_kind: HELP_ALPHA_CATALOG_RECORD_KIND.to_owned(),
            schema_version: HELP_ALPHA_CATALOG_SCHEMA_VERSION,
            catalog_id: "docs-help-alpha:catalog:seed".to_owned(),
            running_build_identity_ref: build_identity_ref.clone(),
            release_channel: "preview".to_owned(),
            generated_at: now_rfc3339(),
            search_results: seeded_search_results(registry, &build_identity_ref),
            about_service_rows: seeded_about_service_rows(&build_identity_ref),
        }
    }

    /// Builds the catalog from the command registry and the Help / About surface.
    pub fn from_command_registry_and_about_surface(
        registry: &CommandRegistry,
        surface: &HelpAboutSurface,
    ) -> Self {
        let build_identity_ref = surface.build_identity.exact_build_identity_ref.clone();
        Self {
            record_kind: HELP_ALPHA_CATALOG_RECORD_KIND.to_owned(),
            schema_version: HELP_ALPHA_CATALOG_SCHEMA_VERSION,
            catalog_id: "docs-help-alpha:catalog:about-surface".to_owned(),
            running_build_identity_ref: build_identity_ref.clone(),
            release_channel: surface.build_identity.release_channel_class_token.clone(),
            generated_at: now_rfc3339(),
            search_results: seeded_search_results(registry, &build_identity_ref),
            about_service_rows: project_about_service_rows(surface),
        }
    }

    /// Searches the catalog without narrowing filters.
    pub fn search(&self, query_text: &str) -> HelpAlphaSearchSnapshot {
        self.search_with_query(HelpAlphaSearchQuery::text(query_text))
    }

    /// Searches the catalog with query and filter controls.
    pub fn search_with_query(&self, query: HelpAlphaSearchQuery) -> HelpAlphaSearchSnapshot {
        let normalized = normalize_query(&query.query_text);
        let results: Vec<_> = self
            .search_results
            .iter()
            .filter(|result| result_matches(result, &normalized))
            .filter(|result| {
                query.content_classes.is_empty()
                    || query.content_classes.contains(&result.content_class)
            })
            .filter(|result| {
                query
                    .client_scope
                    .map(|scope| result.source.client_scopes.contains(&scope))
                    .unwrap_or(true)
            })
            .cloned()
            .collect();
        let searched_source_refs = self
            .search_results
            .iter()
            .map(|result| result.source.source_ref.clone())
            .collect();

        HelpAlphaSearchSnapshot {
            query,
            searched_source_refs,
            result_count: results.len(),
            results,
        }
    }

    /// Validates command-backed results against the supplied registry.
    pub fn validate_command_links(
        &self,
        registry: &CommandRegistry,
    ) -> Result<(), Vec<HelpAlphaValidationFinding>> {
        let mut findings = Vec::new();
        for result in &self.search_results {
            if let Some(command_id) = &result.owning_command_id {
                match registry.get(command_id) {
                    Some(entry) => {
                        if result.help_anchor_id != entry.descriptor.docs_help_anchor_ref.anchor_id
                        {
                            findings.push(HelpAlphaValidationFinding {
                                result_or_row_id: result.result_id.clone(),
                                message: "help anchor does not match command descriptor".to_owned(),
                            });
                        }
                    }
                    None => findings.push(HelpAlphaValidationFinding {
                        result_or_row_id: result.result_id.clone(),
                        message: format!("owning command is not in registry: {command_id}"),
                    }),
                }
            }
            if result.owning_command_id.is_none() && result.source.docs_pack_manifest_ref.is_none()
            {
                findings.push(HelpAlphaValidationFinding {
                    result_or_row_id: result.result_id.clone(),
                    message: "result has neither owning command nor docs-pack manifest ref"
                        .to_owned(),
                });
            }
        }
        if findings.is_empty() {
            Ok(())
        } else {
            Err(findings)
        }
    }

    /// Projects an export-safe support payload from the catalog.
    pub fn support_export(&self) -> HelpAlphaSupportExport {
        HelpAlphaSupportExport {
            record_kind: HELP_ALPHA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: HELP_ALPHA_SUPPORT_EXPORT_SCHEMA_VERSION,
            export_id: format!("support-export:{}", self.catalog_id),
            catalog_id: self.catalog_id.clone(),
            running_build_identity_ref: self.running_build_identity_ref.clone(),
            generated_at: now_rfc3339(),
            search_results: self
                .search_results
                .iter()
                .map(HelpAlphaSupportSearchResult::from_result)
                .collect(),
            about_service_rows: self
                .about_service_rows
                .iter()
                .map(HelpAlphaSupportAboutServiceRow::from_row)
                .collect(),
        }
    }
}

/// Validation finding produced by [`HelpAlphaCatalog::validate_command_links`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HelpAlphaValidationFinding {
    /// Result or row id that failed validation.
    pub result_or_row_id: String,
    /// Human-readable validation message.
    pub message: String,
}

impl HelpAlphaSupportSearchResult {
    fn from_result(result: &HelpAlphaSearchResult) -> Self {
        Self {
            result_id: result.result_id.clone(),
            content_class_token: result.content_class_token.clone(),
            source_class_token: result.source.source_class_token.clone(),
            source_revision_ref: result.source.source_revision_ref.clone(),
            display_source_version: result.source.display_source_version.clone(),
            freshness_class_token: result.source.freshness_class_token.clone(),
            support_class_token: result.source.support_class_token.clone(),
            client_scope_tokens: result.source.client_scope_tokens.clone(),
            publish_boundary_state_token: result.source.publish_boundary_state_token.clone(),
            help_anchor_id: result.help_anchor_id.clone(),
            docs_node_id: result.docs_node_id.clone(),
            owning_command_id: result.owning_command_id.clone(),
            citation_availability_token: result.citation.availability_token.clone(),
            citation_anchor_refs: result.citation.citation_anchor_refs.clone(),
            locale_availability_token: result.source.locale.availability_token.clone(),
            source_language_fallback_token: result
                .source
                .locale
                .source_language_fallback_token
                .clone(),
            offline_posture_token: result.source.offline.posture_token.clone(),
            exact_reopen_ref: result.exact_reopen.exact_reopen_ref.clone(),
            browser_handoff_reason_token: result.source.browser_handoff_reason_token.clone(),
        }
    }
}

impl HelpAlphaSupportAboutServiceRow {
    fn from_row(row: &HelpAlphaAboutServiceRow) -> Self {
        Self {
            row_id: row.row_id.clone(),
            surface_class: row.surface_class.clone(),
            contract_state_token: row.source.contract_state_token.clone(),
            source_class_token: row.source.source_class_token.clone(),
            source_revision_ref: row.source.source_revision_ref.clone(),
            display_source_version: row.source.display_source_version.clone(),
            freshness_class_token: row.source.freshness_class_token.clone(),
            destination_trust_class_token: row.source.destination_trust_class_token.clone(),
            publish_boundary_state_token: row.source.publish_boundary_state_token.clone(),
            citation_availability_token: row.citation.availability_token.clone(),
            citation_anchor_refs: row.citation.citation_anchor_refs.clone(),
            locale_availability_token: row.source.locale.availability_token.clone(),
            source_language_fallback_token: row
                .source
                .locale
                .source_language_fallback_token
                .clone(),
            offline_posture_token: row.source.offline.posture_token.clone(),
            exact_reopen_ref: row.exact_reopen.exact_reopen_ref.clone(),
            browser_handoff_reason_token: row.source.browser_handoff_reason_token.clone(),
        }
    }
}

/// Projects About and service-health rows from the existing Help / About surface.
pub fn project_about_service_rows(surface: &HelpAboutSurface) -> Vec<HelpAlphaAboutServiceRow> {
    let build_ref = surface.build_identity.exact_build_identity_ref.clone();
    let build_version = surface.build_identity.workspace_version.clone();
    let docs_source_class = surface
        .docs_help_truth
        .source_class
        .unwrap_or(SourceClass::GeneratedReference);
    let docs_freshness = surface
        .docs_help_truth
        .freshness_class
        .unwrap_or(FreshnessClass::Unverified);
    let docs_version_match = surface.docs_help_truth.version_match_state;
    let docs_contract_state = contract_state_for_docs_truth(
        surface.docs_help_truth.source_missing,
        docs_freshness,
        docs_version_match,
    );

    vec![
        HelpAlphaAboutServiceRow {
            row_id: "about:build-provenance".to_owned(),
            label: "Build provenance".to_owned(),
            summary: "Exact build identity, channel, deployment mode, and local provenance."
                .to_owned(),
            content_class: HelpAlphaContentClass::AboutServiceHealthTruth,
            content_class_token: HelpAlphaContentClass::AboutServiceHealthTruth
                .as_str()
                .to_owned(),
            surface_class: "help_about".to_owned(),
            sign_in_required: false,
            affected_workflows: vec!["copy_build_info".to_owned(), "support_export".to_owned()],
            local_only_continuity: vec!["Core build facts remain inspectable offline.".to_owned()],
            source: source_descriptor(HelpAlphaSourceDescriptorInput {
                source_class: SourceClass::GeneratedReference,
                source_ref: "about-packet:desktop".to_owned(),
                source_revision_ref: format!("about-packet:{build_ref}"),
                display_source_version: build_version.clone(),
                running_build_identity_ref: Some(build_ref.clone()),
                version_match_state: Some(VersionMatchState::ExactBuildMatch),
                freshness_class: FreshnessClass::AuthoritativeLive,
                support_class: HelpAlphaSupportClass::Supported,
                client_scopes: vec![
                    HelpAlphaClientScope::DesktopProduct,
                    HelpAlphaClientScope::Cli,
                ],
                destination_trust_class: HelpAlphaDestinationTrustClass::LocalOnly,
                contract_state: HelpAlphaContractState::Ready,
                publish_boundary_state: HelpAlphaPublishBoundaryState::NotPublishBearing,
                exact_build_or_channel: surface.build_identity.release_channel_class_token.clone(),
                deployment_mode: "individual_local".to_owned(),
                provenance_ref: format!("provenance:about:{build_ref}"),
                docs_pack_manifest_ref: None,
                help_status_badge_ref: None,
                destination_descriptor_ref: Some("dest:about:local-provenance".to_owned()),
                browser_handoff_reason: None,
                locale: locale_not_applicable(),
                offline: offline_local_only("Build facts stay local and copy-safe."),
            }),
            exact_reopen: target(
                HelpAlphaTargetKind::ProductSurface,
                "surface:help-about:build-provenance",
                "reopen:help-about:build-provenance",
                None,
                Some("dest:about:local-provenance"),
                None,
            ),
            citation: citation_not_required("evidence:about:build-provenance"),
        },
        HelpAlphaAboutServiceRow {
            row_id: "service:docs-help-descriptor".to_owned(),
            label: "Docs and help descriptors".to_owned(),
            summary: "Docs/help source, version, freshness, locale, and offline posture."
                .to_owned(),
            content_class: HelpAlphaContentClass::ServiceHealthNote,
            content_class_token: HelpAlphaContentClass::ServiceHealthNote.as_str().to_owned(),
            surface_class: "service_health".to_owned(),
            sign_in_required: false,
            affected_workflows: vec![
                "help_search".to_owned(),
                "docs_browser".to_owned(),
                "support_export".to_owned(),
            ],
            local_only_continuity: vec![
                "Cached or mirrored descriptors remain inspectable with freshness labels."
                    .to_owned(),
            ],
            source: source_descriptor(HelpAlphaSourceDescriptorInput {
                source_class: docs_source_class,
                source_ref: "service-health:docs-help-descriptor".to_owned(),
                source_revision_ref: surface
                    .docs_help_truth
                    .running_build_identity_ref
                    .clone()
                    .unwrap_or_else(|| "docs-help:source:missing".to_owned()),
                display_source_version: build_version,
                running_build_identity_ref: Some(build_ref),
                version_match_state: docs_version_match,
                freshness_class: docs_freshness,
                support_class: HelpAlphaSupportClass::Supported,
                client_scopes: vec![
                    HelpAlphaClientScope::DesktopProduct,
                    HelpAlphaClientScope::Cli,
                ],
                destination_trust_class: HelpAlphaDestinationTrustClass::LocalOnly,
                contract_state: docs_contract_state,
                publish_boundary_state: HelpAlphaPublishBoundaryState::LocalReviewReady,
                exact_build_or_channel: surface.build_identity.release_channel_class_token.clone(),
                deployment_mode: "individual_local".to_owned(),
                provenance_ref: "provenance:docs-help:descriptor".to_owned(),
                docs_pack_manifest_ref: Some(
                    "docs-pack-manifest:project:aureline:alpha".to_owned(),
                ),
                help_status_badge_ref: surface
                    .docs_help_truth
                    .source_missing
                    .then_some("help-status-badge:docs-help:missing-source".to_owned()),
                destination_descriptor_ref: Some("dest:docs:project-docs:local-pack".to_owned()),
                browser_handoff_reason: None,
                locale: locale_authoritative("en"),
                offline: offline_local_only(
                    "Help descriptors can be copied locally when service refresh is unavailable.",
                ),
            }),
            exact_reopen: target(
                HelpAlphaTargetKind::EvidenceCard,
                "evidence:docs-help:descriptor",
                "reopen:evidence:docs-help:descriptor",
                None,
                Some("dest:docs:project-docs:local-pack"),
                None,
            ),
            citation: citation_available(
                vec!["docs:anchor:docs-help-alpha:descriptor".to_owned()],
                "citation:docs-help:descriptor",
            ),
        },
    ]
}

fn seeded_search_results(
    registry: &CommandRegistry,
    build_identity_ref: &str,
) -> Vec<HelpAlphaSearchResult> {
    let mut rows = Vec::new();
    if let Some(entry) = registry.get("cmd:workspace.open_folder") {
        rows.push(command_backed_result(
            entry,
            build_identity_ref,
            HelpAlphaContentClass::CanonicalHelp,
            "help-result:workspace.open-folder",
            "Open a local folder without sign-in",
            "Open local work first; setup, trust, and starter flows stay separate.",
            "docs-node:project-entry.open-folder",
            HelpAlphaPublishBoundaryState::NotPublishBearing,
            FreshnessClass::AuthoritativeLive,
            HelpAlphaSupportClass::Supported,
        ));
    }
    if let Some(entry) = registry.get("cmd:workspace.clone_repository") {
        rows.push(command_backed_result(
            entry,
            build_identity_ref,
            HelpAlphaContentClass::MigrationHint,
            "help-result:workspace.clone-repository",
            "Clone without granting workspace trust",
            "Clone materializes source locally; trust review and setup remain explicit next steps.",
            "docs-node:project-entry.clone-review",
            HelpAlphaPublishBoundaryState::NotPublishBearing,
            FreshnessClass::WarmCached,
            HelpAlphaSupportClass::Supported,
        ));
    }
    if let Some(entry) = registry.get("cmd:docs.open_in_browser") {
        rows.push(command_backed_result(
            entry,
            build_identity_ref,
            HelpAlphaContentClass::PublishBoundaryHandoff,
            "help-result:docs.open-in-browser",
            "Open external docs through governed handoff",
            "Use the browser only when the destination descriptor says in-product truth is insufficient.",
            "docs-node:help.external-handoff",
            HelpAlphaPublishBoundaryState::ExternalConsoleFallback,
            FreshnessClass::WarmCached,
            HelpAlphaSupportClass::Limited,
        ));
        rows.push(generated_suggestion_result(entry, build_identity_ref));
        rows.push(stale_example_result(entry, build_identity_ref));
    }
    rows
}

fn seeded_about_service_rows(build_identity_ref: &str) -> Vec<HelpAlphaAboutServiceRow> {
    vec![
        HelpAlphaAboutServiceRow {
            row_id: "about:alpha-build-channel".to_owned(),
            label: "Alpha build channel".to_owned(),
            summary: "Exact build, preview channel, deployment mode, and provenance.".to_owned(),
            content_class: HelpAlphaContentClass::AboutServiceHealthTruth,
            content_class_token: HelpAlphaContentClass::AboutServiceHealthTruth
                .as_str()
                .to_owned(),
            surface_class: "help_about".to_owned(),
            sign_in_required: false,
            affected_workflows: vec!["about".to_owned(), "support_export".to_owned()],
            local_only_continuity: vec!["Build/channel facts remain local and copyable.".to_owned()],
            source: source_descriptor(HelpAlphaSourceDescriptorInput {
                source_class: SourceClass::GeneratedReference,
                source_ref: "about-packet:desktop".to_owned(),
                source_revision_ref: "about-packet:alpha:preview".to_owned(),
                display_source_version: "alpha-preview".to_owned(),
                running_build_identity_ref: Some(build_identity_ref.to_owned()),
                version_match_state: Some(VersionMatchState::ExactBuildMatch),
                freshness_class: FreshnessClass::AuthoritativeLive,
                support_class: HelpAlphaSupportClass::Supported,
                client_scopes: vec![
                    HelpAlphaClientScope::DesktopProduct,
                    HelpAlphaClientScope::Cli,
                ],
                destination_trust_class: HelpAlphaDestinationTrustClass::LocalOnly,
                contract_state: HelpAlphaContractState::Ready,
                publish_boundary_state: HelpAlphaPublishBoundaryState::NotPublishBearing,
                exact_build_or_channel: "preview".to_owned(),
                deployment_mode: "individual_local".to_owned(),
                provenance_ref: "provenance:about:alpha-preview".to_owned(),
                docs_pack_manifest_ref: None,
                help_status_badge_ref: Some("help-status-badge:about:alpha-build".to_owned()),
                destination_descriptor_ref: Some("dest:about:local-provenance".to_owned()),
                browser_handoff_reason: None,
                locale: locale_not_applicable(),
                offline: offline_local_only("Core About facts do not require sign-in."),
            }),
            exact_reopen: target(
                HelpAlphaTargetKind::ProductSurface,
                "surface:help-about:summary",
                "reopen:help-about:summary",
                None,
                Some("dest:about:local-provenance"),
                None,
            ),
            citation: citation_not_required("evidence:about:alpha-build-channel"),
        },
        HelpAlphaAboutServiceRow {
            row_id: "service:local-continuity-docs-help".to_owned(),
            label: "Local docs/help continuity".to_owned(),
            summary: "Cached descriptors remain useful offline with local-only limits.".to_owned(),
            content_class: HelpAlphaContentClass::ServiceHealthNote,
            content_class_token: HelpAlphaContentClass::ServiceHealthNote.as_str().to_owned(),
            surface_class: "service_health".to_owned(),
            sign_in_required: false,
            affected_workflows: vec![
                "help_search".to_owned(),
                "docs_browser".to_owned(),
                "support_export".to_owned(),
            ],
            local_only_continuity: vec![
                "Search cached help rows.".to_owned(),
                "Copy descriptors into a support export.".to_owned(),
            ],
            source: source_descriptor(HelpAlphaSourceDescriptorInput {
                source_class: SourceClass::SupportRunbook,
                source_ref: "service-health:aggregator".to_owned(),
                source_revision_ref: "svc-health-snapshot:docs-help-alpha:local-only".to_owned(),
                display_source_version: "aggregator-alpha".to_owned(),
                running_build_identity_ref: Some(build_identity_ref.to_owned()),
                version_match_state: Some(VersionMatchState::ExactBuildMatch),
                freshness_class: FreshnessClass::Stale,
                support_class: HelpAlphaSupportClass::Limited,
                client_scopes: vec![
                    HelpAlphaClientScope::DesktopProduct,
                    HelpAlphaClientScope::Cli,
                ],
                destination_trust_class: HelpAlphaDestinationTrustClass::LocalOnly,
                contract_state: HelpAlphaContractState::LocalOnly,
                publish_boundary_state: HelpAlphaPublishBoundaryState::LocalOnlyAuthoring,
                exact_build_or_channel: "preview".to_owned(),
                deployment_mode: "individual_local_offline".to_owned(),
                provenance_ref: "provenance:service-health:docs-help-local-only".to_owned(),
                docs_pack_manifest_ref: Some(
                    "docs-pack-manifest:project:aureline:alpha".to_owned(),
                ),
                help_status_badge_ref: Some(
                    "help-status-badge:service:docs-help-local-only".to_owned(),
                ),
                destination_descriptor_ref: Some(
                    "dest:service-health:local-only-continuation".to_owned(),
                ),
                browser_handoff_reason: None,
                locale: locale_with_source_fallback("en", "es"),
                offline: HelpAlphaOfflineState {
                    posture: HelpAlphaOfflinePosture::AirGappedLocalOnly,
                    posture_token: HelpAlphaOfflinePosture::AirGappedLocalOnly
                        .as_str()
                        .to_owned(),
                    last_refreshed_at: Some("2026-05-13T03:42:00Z".to_owned()),
                    offline_expiration_at: Some("2026-05-27T03:42:00Z".to_owned()),
                    local_only_continuity_limit: Some(
                        "Live service status cannot be asserted while offline.".to_owned(),
                    ),
                },
            }),
            exact_reopen: target(
                HelpAlphaTargetKind::EvidenceCard,
                "evidence:service-health:docs-help-local-only",
                "reopen:evidence:service-health:docs-help-local-only",
                None,
                Some("dest:service-health:local-only-continuation"),
                None,
            ),
            citation: citation_available(
                vec!["docs:anchor:runbook:docs-help-local-only".to_owned()],
                "citation:service-health:docs-help-local-only",
            ),
        },
        HelpAlphaAboutServiceRow {
            row_id: "service:external-status-feed-handoff".to_owned(),
            label: "External status feed".to_owned(),
            summary: "Last cached status is stale; live details require a browser handoff."
                .to_owned(),
            content_class: HelpAlphaContentClass::PublishBoundaryHandoff,
            content_class_token: HelpAlphaContentClass::PublishBoundaryHandoff
                .as_str()
                .to_owned(),
            surface_class: "service_health".to_owned(),
            sign_in_required: false,
            affected_workflows: vec!["managed_workspace_status".to_owned()],
            local_only_continuity: vec![
                "Local editing is unaffected by the external feed.".to_owned()
            ],
            source: source_descriptor(HelpAlphaSourceDescriptorInput {
                source_class: SourceClass::ExternalStatusFeed,
                source_ref: "external-status-feed:aureline-cloud".to_owned(),
                source_revision_ref: "feed-snapshot:2026-05-13T02:00:00Z".to_owned(),
                display_source_version: "feed-2026-05-13T02:00Z".to_owned(),
                running_build_identity_ref: None,
                version_match_state: None,
                freshness_class: FreshnessClass::Stale,
                support_class: HelpAlphaSupportClass::Limited,
                client_scopes: vec![
                    HelpAlphaClientScope::DesktopProduct,
                    HelpAlphaClientScope::CompanionSurface,
                ],
                destination_trust_class: HelpAlphaDestinationTrustClass::OfficialPublic,
                contract_state: HelpAlphaContractState::Degraded,
                publish_boundary_state: HelpAlphaPublishBoundaryState::ExternalConsoleFallback,
                exact_build_or_channel: "not_build_bound".to_owned(),
                deployment_mode: "optional_managed_service".to_owned(),
                provenance_ref: "provenance:external-status-feed:aureline-cloud".to_owned(),
                docs_pack_manifest_ref: None,
                help_status_badge_ref: Some("help-status-badge:service:external-feed".to_owned()),
                destination_descriptor_ref: Some(
                    "dest:service-health:external-status-feed".to_owned(),
                ),
                browser_handoff_reason: Some(HelpAlphaBrowserHandoffReason::ExternalDocsOrRunbook),
                locale: locale_authoritative("en"),
                offline: HelpAlphaOfflineState {
                    posture: HelpAlphaOfflinePosture::UnavailableOffline,
                    posture_token: HelpAlphaOfflinePosture::UnavailableOffline
                        .as_str()
                        .to_owned(),
                    last_refreshed_at: Some("2026-05-13T02:00:00Z".to_owned()),
                    offline_expiration_at: None,
                    local_only_continuity_limit: Some(
                        "Only cached incident metadata is available offline.".to_owned(),
                    ),
                },
            }),
            exact_reopen: target(
                HelpAlphaTargetKind::BrowserHandoff,
                "browser-handoff:service-health:external-status-feed",
                "reopen:browser-handoff:service-health:external-status-feed",
                Some("cmd:docs.open_in_browser"),
                Some("dest:service-health:external-status-feed"),
                Some(HelpAlphaBrowserHandoffReason::ExternalDocsOrRunbook),
            ),
            citation: citation_available(
                vec!["docs:anchor:external-feed:aureline-cloud:last-snapshot".to_owned()],
                "citation:service-health:external-status-feed",
            ),
        },
    ]
}

fn command_backed_result(
    entry: &CommandRegistryEntryRecord,
    build_identity_ref: &str,
    content_class: HelpAlphaContentClass,
    result_id: &str,
    title: &str,
    summary: &str,
    docs_node_id: &str,
    publish_boundary_state: HelpAlphaPublishBoundaryState,
    freshness_class: FreshnessClass,
    support_class: HelpAlphaSupportClass,
) -> HelpAlphaSearchResult {
    let anchor = &entry.descriptor.docs_help_anchor_ref;
    let browser_handoff_reason = (entry.descriptor.command_id == "cmd:docs.open_in_browser")
        .then_some(HelpAlphaBrowserHandoffReason::ExternalDocsOrRunbook);
    let destination_descriptor_ref = if entry.descriptor.command_id == "cmd:docs.open_in_browser" {
        Some("dest:docs:mirrored-reference:offline".to_owned())
    } else {
        Some("dest:docs:project-docs:local-pack".to_owned())
    };
    let target_kind = if entry.descriptor.command_id == "cmd:docs.open_in_browser" {
        HelpAlphaTargetKind::BrowserHandoff
    } else {
        HelpAlphaTargetKind::Command
    };
    let source = source_descriptor(HelpAlphaSourceDescriptorInput {
        source_class: SourceClass::ProjectDocs,
        source_ref: anchor.pack_id.clone(),
        source_revision_ref: format!("docs-pack-rev:{}", anchor.pack_id),
        display_source_version: "alpha-help-2026.05".to_owned(),
        running_build_identity_ref: Some(build_identity_ref.to_owned()),
        version_match_state: Some(VersionMatchState::ExactBuildMatch),
        freshness_class,
        support_class,
        client_scopes: entry
            .descriptor
            .client_scopes
            .iter()
            .filter_map(|scope| parse_client_scope(scope))
            .collect(),
        destination_trust_class: HelpAlphaDestinationTrustClass::OfficialPublic,
        contract_state: HelpAlphaContractState::Ready,
        publish_boundary_state,
        exact_build_or_channel: entry.descriptor.release_channel.clone(),
        deployment_mode: "individual_local".to_owned(),
        provenance_ref: format!("provenance:command:{}", entry.descriptor.command_id),
        docs_pack_manifest_ref: Some(format!("docs-pack-manifest:{}", anchor.pack_id)),
        help_status_badge_ref: Some(format!("help-status-badge:{}", entry.descriptor.command_id)),
        destination_descriptor_ref: destination_descriptor_ref.clone(),
        browser_handoff_reason,
        locale: locale_authoritative("en"),
        offline: HelpAlphaOfflineState {
            posture: HelpAlphaOfflinePosture::WarmCached,
            posture_token: HelpAlphaOfflinePosture::WarmCached.as_str().to_owned(),
            last_refreshed_at: Some("2026-05-13T03:42:00Z".to_owned()),
            offline_expiration_at: Some("2026-05-27T03:42:00Z".to_owned()),
            local_only_continuity_limit: Some(
                "Search result remains openable from the local docs pack.".to_owned(),
            ),
        },
    });
    let exact_reopen = target(
        target_kind,
        if target_kind == HelpAlphaTargetKind::Command {
            &entry.descriptor.command_id
        } else {
            "browser-handoff:docs:open-in-browser"
        },
        &format!("reopen:{}", entry.descriptor.command_id),
        Some(&entry.descriptor.command_id),
        destination_descriptor_ref.as_deref(),
        browser_handoff_reason,
    );
    HelpAlphaSearchResult {
        result_id: result_id.to_owned(),
        title: title.to_owned(),
        summary: summary.to_owned(),
        content_class,
        content_class_token: content_class.as_str().to_owned(),
        docs_node_id: Some(docs_node_id.to_owned()),
        help_anchor_id: anchor.anchor_id.clone(),
        owning_command_id: Some(entry.descriptor.command_id.clone()),
        command_revision_ref: Some(entry.descriptor.command_revision_ref.clone()),
        product_surface_id: Some("surface:docs-help-alpha:search".to_owned()),
        source,
        exact_reopen,
        citation: citation_available(
            vec![anchor.anchor_id.clone()],
            &format!("citation:{}", entry.descriptor.command_id),
        ),
    }
}

fn generated_suggestion_result(
    entry: &CommandRegistryEntryRecord,
    build_identity_ref: &str,
) -> HelpAlphaSearchResult {
    let anchor = &entry.descriptor.docs_help_anchor_ref;
    HelpAlphaSearchResult {
        result_id: "help-result:docs-suggestion.review-before-publish".to_owned(),
        title: "Review README and changelog suggestions before publish".to_owned(),
        summary:
            "Generated docs suggestions stay local until validation budget and owner review pass."
                .to_owned(),
        content_class: HelpAlphaContentClass::GeneratedSuggestion,
        content_class_token: HelpAlphaContentClass::GeneratedSuggestion
            .as_str()
            .to_owned(),
        docs_node_id: Some("docs-node:docs-maintenance.review-suggestions".to_owned()),
        help_anchor_id: anchor.anchor_id.clone(),
        owning_command_id: Some(entry.descriptor.command_id.clone()),
        command_revision_ref: Some(entry.descriptor.command_revision_ref.clone()),
        product_surface_id: Some("surface:docs-maintenance:suggestion-card".to_owned()),
        source: source_descriptor(HelpAlphaSourceDescriptorInput {
            source_class: SourceClass::GeneratedReference,
            source_ref: "docs-suggestion:readme-changelog-alpha".to_owned(),
            source_revision_ref: "docs-suggestion:readme-changelog-alpha:2026-05-13".to_owned(),
            display_source_version: "suggestion-alpha-2026.05".to_owned(),
            running_build_identity_ref: Some(build_identity_ref.to_owned()),
            version_match_state: Some(VersionMatchState::ExactBuildMatch),
            freshness_class: FreshnessClass::WarmCached,
            support_class: HelpAlphaSupportClass::Limited,
            client_scopes: vec![
                HelpAlphaClientScope::DesktopProduct,
                HelpAlphaClientScope::Cli,
            ],
            destination_trust_class: HelpAlphaDestinationTrustClass::LocalOnly,
            contract_state: HelpAlphaContractState::Ready,
            publish_boundary_state: HelpAlphaPublishBoundaryState::LocalReviewReady,
            exact_build_or_channel: "preview".to_owned(),
            deployment_mode: "individual_local".to_owned(),
            provenance_ref: "provenance:docs-suggestion:readme-changelog-alpha".to_owned(),
            docs_pack_manifest_ref: Some("docs-pack-manifest:project:aureline:alpha".to_owned()),
            help_status_badge_ref: Some("help-status-badge:docs-suggestion:alpha".to_owned()),
            destination_descriptor_ref: Some("dest:docs:project-docs:local-pack".to_owned()),
            browser_handoff_reason: None,
            locale: locale_authoritative("en"),
            offline: offline_local_only(
                "Draft suggestions remain reviewable locally; publication does not occur offline.",
            ),
        }),
        exact_reopen: target(
            HelpAlphaTargetKind::EvidenceCard,
            "evidence:docs-suggestion:readme-changelog-alpha",
            "reopen:evidence:docs-suggestion:readme-changelog-alpha",
            Some(&entry.descriptor.command_id),
            Some("dest:docs:project-docs:local-pack"),
            None,
        ),
        citation: citation_available(
            vec![
                "docs:anchor:docs-suggestion:validation-budget".to_owned(),
                "docs:anchor:docs-suggestion:publish-boundary".to_owned(),
            ],
            "citation:docs-suggestion:readme-changelog-alpha",
        ),
    }
}

fn stale_example_result(
    entry: &CommandRegistryEntryRecord,
    build_identity_ref: &str,
) -> HelpAlphaSearchResult {
    let anchor = &entry.descriptor.docs_help_anchor_ref;
    HelpAlphaSearchResult {
        result_id: "help-result:stale-example.publish-boundary-warning".to_owned(),
        title: "Stale example warning before external publish".to_owned(),
        summary:
            "A stale example remains visible with source, freshness, and publish-boundary block."
                .to_owned(),
        content_class: HelpAlphaContentClass::StaleExampleWarning,
        content_class_token: HelpAlphaContentClass::StaleExampleWarning
            .as_str()
            .to_owned(),
        docs_node_id: Some("docs-node:docs-maintenance.stale-example-warning".to_owned()),
        help_anchor_id: anchor.anchor_id.clone(),
        owning_command_id: Some(entry.descriptor.command_id.clone()),
        command_revision_ref: Some(entry.descriptor.command_revision_ref.clone()),
        product_surface_id: Some("surface:docs-maintenance:stale-example-row".to_owned()),
        source: source_descriptor(HelpAlphaSourceDescriptorInput {
            source_class: SourceClass::CuratedKnowledgePack,
            source_ref: "docs-pack:curated:stale-examples-alpha".to_owned(),
            source_revision_ref: "docs-pack-rev:curated:stale-examples-alpha:2026-05-13".to_owned(),
            display_source_version: "stale-example-ledger-2026.05".to_owned(),
            running_build_identity_ref: Some(build_identity_ref.to_owned()),
            version_match_state: Some(VersionMatchState::CompatibleMinorDrift),
            freshness_class: FreshnessClass::Stale,
            support_class: HelpAlphaSupportClass::Limited,
            client_scopes: vec![
                HelpAlphaClientScope::DesktopProduct,
                HelpAlphaClientScope::Cli,
            ],
            destination_trust_class: HelpAlphaDestinationTrustClass::OfficialPublic,
            contract_state: HelpAlphaContractState::Stale,
            publish_boundary_state: HelpAlphaPublishBoundaryState::BlockedPendingValidation,
            exact_build_or_channel: "preview".to_owned(),
            deployment_mode: "individual_local".to_owned(),
            provenance_ref: "provenance:docs-stale-example:alpha".to_owned(),
            docs_pack_manifest_ref: Some(
                "docs-pack-manifest:curated:stale-examples-alpha".to_owned(),
            ),
            help_status_badge_ref: Some("help-status-badge:stale-example:alpha".to_owned()),
            destination_descriptor_ref: Some("dest:docs:mirrored-reference:offline".to_owned()),
            browser_handoff_reason: Some(HelpAlphaBrowserHandoffReason::ExternalDocsOrRunbook),
            locale: locale_with_source_fallback("en", "fr"),
            offline: HelpAlphaOfflineState {
                posture: HelpAlphaOfflinePosture::MirroredOffline,
                posture_token: HelpAlphaOfflinePosture::MirroredOffline.as_str().to_owned(),
                last_refreshed_at: Some("2026-05-13T03:42:00Z".to_owned()),
                offline_expiration_at: Some("2026-05-20T03:42:00Z".to_owned()),
                local_only_continuity_limit: Some(
                    "The warning is useful offline, but validation cannot be refreshed.".to_owned(),
                ),
            },
        }),
        exact_reopen: target(
            HelpAlphaTargetKind::BrowserHandoff,
            "browser-handoff:docs:stale-example-warning",
            "reopen:browser-handoff:docs:stale-example-warning",
            Some(&entry.descriptor.command_id),
            Some("dest:docs:mirrored-reference:offline"),
            Some(HelpAlphaBrowserHandoffReason::ExternalDocsOrRunbook),
        ),
        citation: citation_available(
            vec![
                "docs:anchor:stale-example:warning".to_owned(),
                "docs:anchor:validation-budget:docs-examples".to_owned(),
            ],
            "citation:stale-example:publish-boundary-warning",
        ),
    }
}

#[derive(Debug)]
struct HelpAlphaSourceDescriptorInput {
    source_class: SourceClass,
    source_ref: String,
    source_revision_ref: String,
    display_source_version: String,
    running_build_identity_ref: Option<String>,
    version_match_state: Option<VersionMatchState>,
    freshness_class: FreshnessClass,
    support_class: HelpAlphaSupportClass,
    client_scopes: Vec<HelpAlphaClientScope>,
    destination_trust_class: HelpAlphaDestinationTrustClass,
    contract_state: HelpAlphaContractState,
    publish_boundary_state: HelpAlphaPublishBoundaryState,
    exact_build_or_channel: String,
    deployment_mode: String,
    provenance_ref: String,
    docs_pack_manifest_ref: Option<String>,
    help_status_badge_ref: Option<String>,
    destination_descriptor_ref: Option<String>,
    browser_handoff_reason: Option<HelpAlphaBrowserHandoffReason>,
    locale: HelpAlphaLocaleState,
    offline: HelpAlphaOfflineState,
}

fn source_descriptor(input: HelpAlphaSourceDescriptorInput) -> HelpAlphaSourceDescriptor {
    let version_match_token = input
        .version_match_state
        .map(|state| version_match_token(state).to_owned());
    let browser_handoff_reason_token = input
        .browser_handoff_reason
        .map(|reason| reason.as_str().to_owned());
    let client_scope_tokens = input
        .client_scopes
        .iter()
        .map(|scope| scope.as_str().to_owned())
        .collect();

    HelpAlphaSourceDescriptor {
        source_class: input.source_class,
        source_class_token: source_class_token(input.source_class).to_owned(),
        source_ref: input.source_ref,
        source_revision_ref: input.source_revision_ref,
        display_source_version: input.display_source_version,
        running_build_identity_ref: input.running_build_identity_ref,
        version_match_state: input.version_match_state,
        version_match_token,
        freshness_class: input.freshness_class,
        freshness_class_token: freshness_class_token(input.freshness_class).to_owned(),
        support_class: input.support_class,
        support_class_token: input.support_class.as_str().to_owned(),
        client_scopes: input.client_scopes,
        client_scope_tokens,
        destination_trust_class: input.destination_trust_class,
        destination_trust_class_token: input.destination_trust_class.as_str().to_owned(),
        contract_state: input.contract_state,
        contract_state_token: input.contract_state.as_str().to_owned(),
        publish_boundary_state: input.publish_boundary_state,
        publish_boundary_state_token: input.publish_boundary_state.as_str().to_owned(),
        exact_build_or_channel: input.exact_build_or_channel,
        deployment_mode: input.deployment_mode,
        provenance_ref: input.provenance_ref,
        docs_pack_manifest_ref: input.docs_pack_manifest_ref,
        help_status_badge_ref: input.help_status_badge_ref,
        destination_descriptor_ref: input.destination_descriptor_ref,
        browser_handoff_reason: input.browser_handoff_reason,
        browser_handoff_reason_token,
        locale: input.locale,
        offline: input.offline,
    }
}

fn target(
    target_kind: HelpAlphaTargetKind,
    target_ref: &str,
    exact_reopen_ref: &str,
    command_id: Option<&str>,
    destination_descriptor_ref: Option<&str>,
    browser_handoff_reason: Option<HelpAlphaBrowserHandoffReason>,
) -> HelpAlphaTarget {
    HelpAlphaTarget {
        target_kind,
        target_kind_token: target_kind.as_str().to_owned(),
        target_ref: target_ref.to_owned(),
        exact_reopen_ref: exact_reopen_ref.to_owned(),
        command_id: command_id.map(ToOwned::to_owned),
        destination_descriptor_ref: destination_descriptor_ref.map(ToOwned::to_owned),
        browser_handoff_reason,
        browser_handoff_reason_token: browser_handoff_reason
            .map(|reason| reason.as_str().to_owned()),
    }
}

fn citation_available(
    citation_anchor_refs: Vec<String>,
    details_ref: &str,
) -> HelpAlphaCitationState {
    HelpAlphaCitationState {
        availability: HelpAlphaCitationAvailability::Available,
        availability_token: HelpAlphaCitationAvailability::Available.as_str().to_owned(),
        citation_anchor_refs,
        open_details_target: target(
            HelpAlphaTargetKind::CitationDrawer,
            details_ref,
            &format!("reopen:{details_ref}"),
            None,
            None,
            None,
        ),
    }
}

fn citation_not_required(details_ref: &str) -> HelpAlphaCitationState {
    HelpAlphaCitationState {
        availability: HelpAlphaCitationAvailability::NotRequired,
        availability_token: HelpAlphaCitationAvailability::NotRequired
            .as_str()
            .to_owned(),
        citation_anchor_refs: Vec::new(),
        open_details_target: target(
            HelpAlphaTargetKind::EvidenceCard,
            details_ref,
            &format!("reopen:{details_ref}"),
            None,
            None,
            None,
        ),
    }
}

fn locale_authoritative(locale: &str) -> HelpAlphaLocaleState {
    HelpAlphaLocaleState {
        primary_locale: locale.to_owned(),
        requested_locale: locale.to_owned(),
        effective_locale: locale.to_owned(),
        availability: HelpAlphaLocaleAvailability::RequestedLocaleAuthoritative,
        availability_token: HelpAlphaLocaleAvailability::RequestedLocaleAuthoritative
            .as_str()
            .to_owned(),
        source_language_fallback: HelpAlphaSourceLanguageFallback::NotNeeded,
        source_language_fallback_token: HelpAlphaSourceLanguageFallback::NotNeeded
            .as_str()
            .to_owned(),
    }
}

fn locale_with_source_fallback(primary: &str, requested: &str) -> HelpAlphaLocaleState {
    HelpAlphaLocaleState {
        primary_locale: primary.to_owned(),
        requested_locale: requested.to_owned(),
        effective_locale: primary.to_owned(),
        availability: HelpAlphaLocaleAvailability::RequestedLocaleMissingFallbackToPrimary,
        availability_token: HelpAlphaLocaleAvailability::RequestedLocaleMissingFallbackToPrimary
            .as_str()
            .to_owned(),
        source_language_fallback: HelpAlphaSourceLanguageFallback::SourceLanguageFallbackAvailable,
        source_language_fallback_token:
            HelpAlphaSourceLanguageFallback::SourceLanguageFallbackAvailable
                .as_str()
                .to_owned(),
    }
}

fn locale_not_applicable() -> HelpAlphaLocaleState {
    HelpAlphaLocaleState {
        primary_locale: "und".to_owned(),
        requested_locale: "und".to_owned(),
        effective_locale: "und".to_owned(),
        availability: HelpAlphaLocaleAvailability::LocaleNotApplicableToRow,
        availability_token: HelpAlphaLocaleAvailability::LocaleNotApplicableToRow
            .as_str()
            .to_owned(),
        source_language_fallback: HelpAlphaSourceLanguageFallback::NotApplicable,
        source_language_fallback_token: HelpAlphaSourceLanguageFallback::NotApplicable
            .as_str()
            .to_owned(),
    }
}

fn offline_local_only(limit: &str) -> HelpAlphaOfflineState {
    HelpAlphaOfflineState {
        posture: HelpAlphaOfflinePosture::AirGappedLocalOnly,
        posture_token: HelpAlphaOfflinePosture::AirGappedLocalOnly
            .as_str()
            .to_owned(),
        last_refreshed_at: None,
        offline_expiration_at: None,
        local_only_continuity_limit: Some(limit.to_owned()),
    }
}

fn parse_client_scope(value: &str) -> Option<HelpAlphaClientScope> {
    match value {
        "desktop_product" => Some(HelpAlphaClientScope::DesktopProduct),
        "cli" => Some(HelpAlphaClientScope::Cli),
        "companion_surface" => Some(HelpAlphaClientScope::CompanionSurface),
        "remote_agent" => Some(HelpAlphaClientScope::RemoteAgent),
        "sdk_or_api" => Some(HelpAlphaClientScope::SdkOrApi),
        "managed_admin_surface" => Some(HelpAlphaClientScope::ManagedAdminSurface),
        _ => None,
    }
}

fn normalize_query(query: &str) -> String {
    query.trim().to_ascii_lowercase()
}

fn result_matches(result: &HelpAlphaSearchResult, normalized_query: &str) -> bool {
    if normalized_query.is_empty() {
        return true;
    }
    let mut haystack = String::new();
    haystack.push_str(&result.title);
    haystack.push(' ');
    haystack.push_str(&result.summary);
    haystack.push(' ');
    haystack.push_str(&result.help_anchor_id);
    haystack.push(' ');
    haystack.push_str(&result.source.source_ref);
    haystack.push(' ');
    haystack.push_str(&result.source.source_revision_ref);
    if let Some(command_id) = &result.owning_command_id {
        haystack.push(' ');
        haystack.push_str(command_id);
    }
    if let Some(docs_node_id) = &result.docs_node_id {
        haystack.push(' ');
        haystack.push_str(docs_node_id);
    }
    let haystack = haystack.to_ascii_lowercase();
    haystack.contains(normalized_query)
        || normalized_query
            .split_whitespace()
            .all(|term| haystack.contains(term))
}

fn contract_state_for_docs_truth(
    source_missing: bool,
    freshness: FreshnessClass,
    version: Option<VersionMatchState>,
) -> HelpAlphaContractState {
    if source_missing {
        return HelpAlphaContractState::Unavailable;
    }
    if matches!(
        version,
        Some(VersionMatchState::IncompatibleDriftDetected | VersionMatchState::UnknownTargetBuild)
    ) {
        return HelpAlphaContractState::ContractMismatch;
    }
    match freshness {
        FreshnessClass::AuthoritativeLive | FreshnessClass::WarmCached => {
            HelpAlphaContractState::Ready
        }
        FreshnessClass::DegradedCached => HelpAlphaContractState::Degraded,
        FreshnessClass::Stale | FreshnessClass::Unverified => HelpAlphaContractState::Stale,
    }
}

fn source_class_token(class: SourceClass) -> &'static str {
    match class {
        SourceClass::ProjectDocs => "project_docs",
        SourceClass::GeneratedReference => "generated_reference",
        SourceClass::MirroredOfficialDocs => "mirrored_official_docs",
        SourceClass::CuratedKnowledgePack => "curated_knowledge_pack",
        SourceClass::DerivedExplanation => "derived_explanation",
        SourceClass::VendorProviderDocs => "vendor_provider_docs",
        SourceClass::SupportRunbook => "support_runbook",
        SourceClass::ExternalStatusFeed => "external_status_feed",
    }
}

fn version_match_token(state: VersionMatchState) -> &'static str {
    match state {
        VersionMatchState::ExactBuildMatch => "exact_build_match",
        VersionMatchState::CompatibleMinorDrift => "compatible_minor_drift",
        VersionMatchState::IncompatibleDriftDetected => "incompatible_drift_detected",
        VersionMatchState::PreReleaseUnverified => "pre_release_unverified",
        VersionMatchState::UnknownTargetBuild => "unknown_target_build",
    }
}

fn freshness_class_token(class: FreshnessClass) -> &'static str {
    match class {
        FreshnessClass::AuthoritativeLive => "authoritative_live",
        FreshnessClass::WarmCached => "warm_cached",
        FreshnessClass::DegradedCached => "degraded_cached",
        FreshnessClass::Stale => "stale",
        FreshnessClass::Unverified => "unverified",
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use aureline_build_info::BuildIdentityRecord;
    use aureline_commands::registry::seeded_registry;
    use aureline_runtime::{
        CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContextRequest,
        ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
        TargetClass, TrustState,
    };

    use super::*;
    use crate::embedded::boundary_card::SourceTruthRecord;
    use crate::help_about::HelpAboutInputs;

    fn fixture_identity() -> BuildIdentityRecord {
        BuildIdentityRecord {
            schema_version: 1,
            commit: "0123456789abcdef0123456789abcdef01234567".to_owned(),
            commit_short: "0123456".to_owned(),
            dirty: false,
            toolchain_channel: "stable".to_owned(),
            rustc_version: "rustc 1.78.0".to_owned(),
            cargo_version: "cargo 1.78.0".to_owned(),
            host_triple: "aarch64-apple-darwin".to_owned(),
            target_triple: "aarch64-apple-darwin".to_owned(),
            profile: "release".to_owned(),
            workspace_version: "0.0.0".to_owned(),
            source_date_epoch: 1_714_492_800,
            build_timestamp_utc: "2024-04-30T12:00:00Z".to_owned(),
        }
    }

    fn fixture_resolver() -> ExecutionContextResolver {
        ExecutionContextResolver::new(ExecutionContextResolverConfig {
            workspace_id: "ws-test".to_owned(),
            profile_id: Some("prof.default".to_owned()),
            identity_mode: IdentityMode::AccountFreeLocal,
            policy_epoch: 1,
            workspace_default_target_class: TargetClass::LocalHost,
            workspace_default_working_directory: Some("/workspace".to_owned()),
            workspace_default_scope_class: ScopeClass::CurrentRoot,
            local_host_canonical_id: "localhost:darwin-arm64".to_owned(),
            environment_capsule_ref: EnvironmentCapsuleRef {
                capsule_id: "caps:ws-test:seed".to_owned(),
                capsule_hash: "sha256:seed".to_owned(),
                resolved_schema_version: "1".to_owned(),
                drift_state: CapsuleDriftState::InSync,
            },
            resolver_version: "seed-0".to_owned(),
        })
    }

    fn fixture_about_surface() -> HelpAboutSurface {
        let mut resolver = fixture_resolver();
        let context = resolver.resolve(ExecutionContextRequest::local_terminal_seed(
            "terminal.open",
            TrustState::Trusted,
            "mono:0",
        ));
        let identity = fixture_identity();
        let docs_truth = SourceTruthRecord {
            source_class: SourceClass::MirroredOfficialDocs,
            version_match_state: VersionMatchState::CompatibleMinorDrift,
            freshness_class: FreshnessClass::WarmCached,
            running_build_identity_ref: "build-id:aureline:dev:0.0.0:aarch64:dev:0123456"
                .to_owned(),
            help_status_badge_ref: Some("help-status-badge:docs-help:mirror".to_owned()),
            snapshot_age_label: Some("1 hour ago".to_owned()),
        };

        HelpAboutSurface::project(HelpAboutInputs {
            build_identity: &identity,
            release_channel_class_token: "preview",
            execution_context: Some(&context),
            docs_source_truth: Some(&docs_truth),
        })
    }

    #[test]
    fn search_results_use_command_graph_anchors() {
        let registry = seeded_registry();
        let catalog = HelpAlphaCatalog::from_command_registry(registry, "build:running:test:alpha");

        catalog
            .validate_command_links(registry)
            .expect("seeded help catalog should only cite registry commands");

        let snapshot = catalog.search("open folder");
        assert_eq!(snapshot.result_count, 1);
        let result = &snapshot.results[0];
        let entry = registry
            .get("cmd:workspace.open_folder")
            .expect("command registry includes open folder");
        assert_eq!(
            result.owning_command_id.as_deref(),
            Some(entry.command_id().as_str())
        );
        assert_eq!(
            result.help_anchor_id,
            entry.descriptor.docs_help_anchor_ref.anchor_id
        );
        assert_eq!(result.source.source_class_token, "project_docs");
        assert_eq!(result.source.freshness_class_token, "authoritative_live");
        assert_eq!(result.source.support_class_token, "supported");
        assert!(result
            .source
            .client_scope_tokens
            .contains(&"desktop_product".to_owned()));
        assert_eq!(
            result.citation.availability,
            HelpAlphaCitationAvailability::Available
        );
        assert_eq!(
            result.exact_reopen.target_kind,
            HelpAlphaTargetKind::Command
        );
    }

    #[test]
    fn filters_publish_boundary_handoffs() {
        let registry = seeded_registry();
        let catalog = HelpAlphaCatalog::from_command_registry(registry, "build:running:test:alpha");
        let snapshot = catalog.search_with_query(
            HelpAlphaSearchQuery::text("handoff")
                .with_content_class(HelpAlphaContentClass::PublishBoundaryHandoff),
        );

        assert_eq!(snapshot.result_count, 1);
        let result = &snapshot.results[0];
        assert_eq!(
            result.content_class,
            HelpAlphaContentClass::PublishBoundaryHandoff
        );
        assert_eq!(
            result.source.publish_boundary_state,
            HelpAlphaPublishBoundaryState::ExternalConsoleFallback
        );
        assert_eq!(
            result.source.browser_handoff_reason_token.as_deref(),
            Some("external_docs_or_runbook")
        );
        assert_eq!(
            result.exact_reopen.destination_descriptor_ref.as_deref(),
            Some("dest:docs:mirrored-reference:offline")
        );
    }

    #[test]
    fn about_surface_projects_exportable_service_truth() {
        let registry = seeded_registry();
        let surface = fixture_about_surface();
        let catalog = HelpAlphaCatalog::from_command_registry_and_about_surface(registry, &surface);

        let docs_row = catalog
            .about_service_rows
            .iter()
            .find(|row| row.row_id == "service:docs-help-descriptor")
            .expect("docs/help descriptor row exists");
        assert_eq!(docs_row.surface_class, "service_health");
        assert!(!docs_row.sign_in_required);
        assert_eq!(docs_row.source.source_class_token, "mirrored_official_docs");
        assert_eq!(docs_row.source.freshness_class_token, "warm_cached");
        assert_eq!(docs_row.source.contract_state_token, "ready");
        assert_eq!(
            docs_row.citation.availability,
            HelpAlphaCitationAvailability::Available
        );

        let export = catalog.support_export();
        let exported_row = export
            .about_service_rows
            .iter()
            .find(|row| row.row_id == "service:docs-help-descriptor")
            .expect("support export preserves docs/help descriptor row");
        assert_eq!(exported_row.source_class_token, "mirrored_official_docs");
        assert_eq!(exported_row.freshness_class_token, "warm_cached");
        assert_eq!(exported_row.contract_state_token, "ready");
        assert_eq!(exported_row.citation_availability_token, "available");
        assert_eq!(
            exported_row.locale_availability_token,
            "requested_locale_authoritative"
        );
        assert_eq!(exported_row.offline_posture_token, "air_gapped_local_only");
    }

    #[test]
    fn support_export_preserves_citation_locale_offline_and_reopen_identity() {
        let registry = seeded_registry();
        let catalog = HelpAlphaCatalog::from_command_registry(registry, "build:running:test:alpha");
        let export = catalog.support_export();

        let stale_result = export
            .search_results
            .iter()
            .find(|row| row.result_id == "help-result:stale-example.publish-boundary-warning")
            .expect("stale example result is exported");
        assert_eq!(stale_result.source_class_token, "curated_knowledge_pack");
        assert_eq!(stale_result.freshness_class_token, "stale");
        assert_eq!(
            stale_result.publish_boundary_state_token,
            "blocked_pending_validation"
        );
        assert_eq!(
            stale_result.browser_handoff_reason_token.as_deref(),
            Some("external_docs_or_runbook")
        );
        assert_eq!(stale_result.citation_availability_token, "available");
        assert!(!stale_result.citation_anchor_refs.is_empty());
        assert_eq!(
            stale_result.locale_availability_token,
            "requested_locale_missing_fallback_to_primary"
        );
        assert_eq!(
            stale_result.source_language_fallback_token,
            "source_language_fallback_available"
        );
        assert_eq!(stale_result.offline_posture_token, "mirrored_offline");
        assert_eq!(
            stale_result.exact_reopen_ref,
            "reopen:browser-handoff:docs:stale-example-warning"
        );

        let service_row = export
            .about_service_rows
            .iter()
            .find(|row| row.row_id == "service:local-continuity-docs-help")
            .expect("local continuity row is exported");
        assert_eq!(service_row.contract_state_token, "local_only");
        assert_eq!(service_row.source_class_token, "support_runbook");
        assert_eq!(service_row.freshness_class_token, "stale");
        assert_eq!(service_row.destination_trust_class_token, "local_only");
    }

    #[test]
    fn fixture_catalog_round_trips() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/docs/help_search_alpha/catalog.yaml");
        let payload = std::fs::read_to_string(path).expect("fixture must read");
        let fixture: HelpAlphaCatalog =
            serde_yaml::from_str(&payload).expect("fixture must parse as catalog");

        assert_eq!(fixture.record_kind, HELP_ALPHA_CATALOG_RECORD_KIND);
        fixture
            .validate_command_links(seeded_registry())
            .expect("fixture command-backed rows must use registry anchors");
        assert!(fixture
            .search_results
            .iter()
            .any(|row| row.result_id == "fixture:help-search:canonical-open-folder"));
        let exported = fixture.support_export();
        assert!(exported
            .search_results
            .iter()
            .any(|row| row.citation_availability_token == "available"));
        assert!(exported
            .about_service_rows
            .iter()
            .any(|row| row.contract_state_token == "local_only"));
    }

    #[test]
    fn support_export_fixture_reconstructs_descriptors() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/docs/docs_help_alpha/support_export_reconstruction.yaml");
        let payload = std::fs::read_to_string(path).expect("fixture must read");
        let fixture: HelpAlphaSupportExport =
            serde_yaml::from_str(&payload).expect("fixture must parse as support export");

        assert_eq!(fixture.record_kind, HELP_ALPHA_SUPPORT_EXPORT_RECORD_KIND);
        let search_row = fixture
            .search_results
            .iter()
            .find(|row| row.result_id == "fixture:help-search:canonical-open-folder")
            .expect("support export preserves canonical help row");
        assert_eq!(
            search_row.owning_command_id.as_deref(),
            Some("cmd:workspace.open_folder")
        );
        assert_eq!(
            search_row.help_anchor_id,
            "docs:anchor:workspace:open_folder_overview"
        );
        assert_eq!(search_row.citation_availability_token, "available");

        let service_row = fixture
            .about_service_rows
            .iter()
            .find(|row| row.row_id == "fixture:about-service:local-continuity")
            .expect("support export preserves service-health descriptor row");
        assert_eq!(service_row.contract_state_token, "local_only");
        assert_eq!(
            service_row.source_language_fallback_token,
            "source_language_fallback_available"
        );
        assert_eq!(
            service_row.exact_reopen_ref,
            "reopen:evidence:service-health:docs-help-local-only"
        );
    }
}
