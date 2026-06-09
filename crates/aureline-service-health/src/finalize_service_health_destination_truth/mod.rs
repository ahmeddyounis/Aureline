//! Stable Help/About, service-health, destination, and support-export truth contracts.
//!
//! This module owns the stable descriptor contract that public-proof surfaces use
//! when they need to answer four questions without sign-in or hidden reachability
//! assumptions:
//!
//! - what build and release channel is running;
//! - which service family is ready, degraded, stale, local-only, blocked, or
//!   unavailable;
//! - which trust class a Help/About handoff destination targets before the user
//!   leaves the product;
//! - what remains safe locally when live service checks, browsers, or managed
//!   services are unavailable.
//!
//! The descriptor is deliberately metadata-only. It contains stable tokens,
//! opaque refs, freshness state, local-continuity copy, and support-save-later
//! semantics. It does not contain raw URLs, credentials, logs, diagnostics
//! payloads, or user-authored report bodies.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::service_health_feed::{
    ServiceHealthContractState as SharedContractState, ServiceHealthFeed, ServiceHealthFeedItem,
    ServiceHealthFreshness, ServiceHealthOutageScope, ServiceHealthSourceClass,
    ServiceHealthSurface, ServiceHealthSurfaceBinding, SERVICE_HEALTH_FEED_ITEM_RECORD_KIND,
    SERVICE_HEALTH_FEED_RECORD_KIND, SERVICE_HEALTH_FEED_SCHEMA_REF,
    SERVICE_HEALTH_FEED_SCHEMA_VERSION, SERVICE_HEALTH_FEED_SHARED_CONTRACT_REF,
};

/// Schema version for finalized service-health and destination truth descriptors.
pub const SERVICE_HEALTH_DESTINATION_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`ServiceHealthDestinationTruthDescriptor`].
pub const SERVICE_HEALTH_DESTINATION_RECORD_KIND: &str =
    "service_health_destination_truth_descriptor";

/// Stable record-kind tag for [`ServiceHealthDestinationSupportExport`].
pub const SERVICE_HEALTH_DESTINATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "service_health_destination_support_export";

/// Stable contract ref consumed by public-proof surfaces.
pub const SERVICE_HEALTH_DESTINATION_SHARED_CONTRACT_REF: &str =
    "service_health_destination_truth:v1";

/// Stable ref for the checked-in descriptor fixture.
pub const SERVICE_HEALTH_DESTINATION_CANONICAL_DESCRIPTOR_REF: &str =
    "fixtures/help/m4/finalize-service-health-destination-truth/canonical_descriptor.json";

/// Stable ref for the checked-in JSON schema.
pub const SERVICE_HEALTH_DESTINATION_SCHEMA_REF: &str =
    "schemas/help/service-health-destination.schema.json";

/// Stable service-contract state vocabulary shared across Help/About, CLI, docs, cached notices, and support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceContractState {
    /// Live checks are current and the service contract is honored.
    Ready,
    /// The service is usable with reduced capacity or partial impact.
    Degraded,
    /// Managed reachability is absent, but the product has an explicit local-only path.
    LocalOnly,
    /// The descriptor or last-known-good result is visibly stale.
    Stale,
    /// The observed payload or route does not match the agreed contract.
    ContractMismatch,
    /// Policy prevents the service or destination from being used.
    PolicyBlocked,
    /// The service is unavailable for the named family or workflow scope.
    Unavailable,
}

impl ServiceContractState {
    /// Every stable service-contract state in canonical order.
    pub const ALL: [Self; 7] = [
        Self::Ready,
        Self::Degraded,
        Self::LocalOnly,
        Self::Stale,
        Self::ContractMismatch,
        Self::PolicyBlocked,
        Self::Unavailable,
    ];

    /// Returns the stable snake-case token for the state.
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

/// Destination trust classes disclosed before browser handoff or packet export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationTrustClass {
    /// Public project-owned destination visible without product sign-in.
    Public,
    /// Official destination requiring an authenticated project or support identity.
    OfficialAuthenticated,
    /// Community-run or discussion destination that is not guaranteed official support.
    Community,
    /// Provider, extension, or vendor-managed destination outside Aureline governance.
    VendorManaged,
    /// Local preview, save-later packet, or copy action that does not leave the machine.
    LocalOnly,
}

impl DestinationTrustClass {
    /// Every stable destination trust class in canonical order.
    pub const ALL: [Self; 5] = [
        Self::Public,
        Self::OfficialAuthenticated,
        Self::Community,
        Self::VendorManaged,
        Self::LocalOnly,
    ];

    /// Returns the stable snake-case token for the class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Public => "public",
            Self::OfficialAuthenticated => "official_authenticated",
            Self::Community => "community",
            Self::VendorManaged => "vendor_managed",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Public-proof surface that consumes the shared descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PublicProofSurface {
    /// Desktop service-health panel or banner.
    DesktopUi,
    /// CLI or headless machine-readable output.
    CliHeadless,
    /// About/provenance card.
    About,
    /// Help center surface.
    Help,
    /// Service-health detail panel.
    ServiceHealth,
    /// Diagnostics inspector.
    Diagnostics,
    /// Support export preview or saved packet.
    SupportExport,
    /// Release notes.
    ReleaseNotes,
    /// Migration notices.
    MigrationNotice,
    /// Issue/report template.
    IssueReportTemplate,
    /// Community handoff chooser.
    CommunityHandoff,
}

impl PublicProofSurface {
    /// Every required public-proof consumer surface in canonical order.
    pub const REQUIRED: [Self; 11] = [
        Self::DesktopUi,
        Self::CliHeadless,
        Self::About,
        Self::Help,
        Self::ServiceHealth,
        Self::Diagnostics,
        Self::SupportExport,
        Self::ReleaseNotes,
        Self::MigrationNotice,
        Self::IssueReportTemplate,
        Self::CommunityHandoff,
    ];

    /// Returns the stable snake-case token for the surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopUi => "desktop_ui",
            Self::CliHeadless => "cli_headless",
            Self::About => "about",
            Self::Help => "help",
            Self::ServiceHealth => "service_health",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::ReleaseNotes => "release_notes",
            Self::MigrationNotice => "migration_notice",
            Self::IssueReportTemplate => "issue_report_template",
            Self::CommunityHandoff => "community_handoff",
        }
    }
}

/// Freshness posture for descriptors, cards, notices, and handoff routes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptorFreshnessState {
    /// Checked against the live source within the declared freshness window.
    Live,
    /// Served from a cache while still carrying source and age labels.
    Cached,
    /// Served from an operator mirror with source and verification labels.
    Mirrored,
    /// Served from an installed offline pack.
    OfflinePack,
    /// Served from a stale cache and visibly downgraded.
    StaleCache,
    /// Visibility is limited by policy rather than network reachability.
    PolicyLimited,
}

impl DescriptorFreshnessState {
    /// Returns true when the state is not live and must not imply live reachability.
    pub const fn requires_cached_or_stale_label(self) -> bool {
        matches!(
            self,
            Self::Cached | Self::Mirrored | Self::OfflinePack | Self::StaleCache
        )
    }
}

/// Drill scenarios used to prove offline and degraded continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContinuityDrillScenario {
    /// No network is available.
    Offline,
    /// Public endpoints are replaced by a mirror.
    Mirrored,
    /// The browser or external-open path is blocked.
    BrowserBlocked,
    /// A managed service is degraded.
    DegradedService,
    /// One service family is impaired while other families remain usable.
    PartialServiceOutage,
}

/// Build identity and channel facts exposed without sign-in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildIdentityDescriptor {
    /// Stable build identity ref consumed by surface bindings.
    pub build_identity_ref: String,
    /// Product name.
    pub product_name: String,
    /// Version string.
    pub version: String,
    /// Exact build id.
    pub build_id: String,
    /// Release channel.
    pub channel: String,
    /// Install mode.
    pub install_mode: String,
    /// Provenance or release evidence ref.
    pub provenance_ref: String,
}

/// Freshness descriptor shown on surfaces that may be live, cached, mirrored, or offline.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FreshnessDescriptor {
    /// Stable freshness ref.
    pub freshness_ref: String,
    /// Freshness state.
    pub state: DescriptorFreshnessState,
    /// Source or mirror ref.
    pub source_ref: String,
    /// ISO-8601 last-checked timestamp.
    pub last_checked_at: String,
    /// ISO-8601 timestamp after which this data becomes stale.
    pub stale_after: String,
    /// Visible label used when cached, mirrored, offline, or stale.
    pub visible_freshness_label: String,
    /// Whether the surface may claim live reachability from this descriptor.
    pub live_reachability_claim_allowed: bool,
}

/// Manifest row describing the required fields for one destination trust class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestinationTrustClassManifest {
    /// Destination class token.
    pub destination_class: DestinationTrustClass,
    /// Visibility boundary shown before exit.
    pub visibility_boundary: String,
    /// Authentication expectation shown before exit.
    pub auth_expectation: String,
    /// Data-exit boundary shown before exit.
    pub data_exit_boundary: String,
    /// Whether this class supports issue templates.
    pub issue_template_support: bool,
    /// Browser-blocked fallback shown before exit.
    pub browser_blocked_fallback: String,
    /// Offline fallback shown before exit.
    pub offline_fallback: String,
    /// Whether the class label is required before opening a browser or exporting a packet.
    pub pre_exit_label_required: bool,
}

/// Concrete destination descriptor consumed by Help/About, release, migration, issue, and community surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DestinationDescriptor {
    /// Stable destination id.
    pub destination_id: String,
    /// Destination title.
    pub title: String,
    /// Destination purpose.
    pub purpose: String,
    /// Destination trust class.
    pub destination_class: DestinationTrustClass,
    /// Visibility boundary for this destination.
    pub visibility_boundary: String,
    /// Authentication expectation for this destination.
    pub auth_expectation: String,
    /// Data-exit boundary for this destination.
    pub data_exit_boundary: String,
    /// Issue-template support statement.
    pub issue_template_support: String,
    /// Browser-blocked fallback ref.
    pub browser_blocked_fallback_ref: String,
    /// Offline fallback ref.
    pub offline_fallback_ref: String,
    /// Surfaces that may expose this destination.
    pub surfaces: Vec<PublicProofSurface>,
}

/// One service-health card emitted by the shared health aggregator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthTruthCard {
    /// Stable service card id.
    pub card_id: String,
    /// Service family token.
    pub service_family: String,
    /// Boundary class for the service family.
    pub boundary_class: String,
    /// Current service-contract state.
    pub service_contract_state: ServiceContractState,
    /// Workflows affected by this service state.
    pub affected_workflows: Vec<String>,
    /// ISO-8601 last-checked timestamp.
    pub last_checked_at: String,
    /// Freshness ref used by this card.
    pub freshness_ref: String,
    /// User-visible freshness label.
    pub visible_freshness_label: String,
    /// Scoped outage statement.
    pub outage_scope: String,
    /// Local-only continuity statement.
    pub local_only_continuity_note: String,
    /// Diagnostics action ref.
    pub diagnostics_action_ref: String,
    /// Surfaces that consume this exact card.
    pub surfaced_on: Vec<PublicProofSurface>,
}

/// Binding that proves one public-proof surface consumes the shared descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceDescriptorBinding {
    /// Surface consuming the descriptor.
    pub surface: PublicProofSurface,
    /// Shared descriptor ref consumed by the surface.
    pub descriptor_ref: String,
    /// Build identity ref consumed by the surface.
    pub build_identity_ref: String,
    /// Freshness refs consumed by the surface.
    pub freshness_refs: Vec<String>,
    /// Service-health card refs consumed by the surface.
    pub service_health_card_refs: Vec<String>,
    /// Destination refs consumed by the surface.
    pub destination_refs: Vec<String>,
    /// Whether the surface reads the shared descriptor source directly.
    pub consumes_shared_descriptor: bool,
    /// Whether reading build, outage, or continuity facts requires sign-in.
    pub requires_sign_in_to_read: bool,
    /// Whether cached/offline/stale labels are visible when applicable.
    pub cached_or_offline_label_visible: bool,
    /// Whether local-only continuity is visible.
    pub local_only_continuity_visible: bool,
    /// Whether the surface is allowed to overclaim live reachability.
    pub may_overclaim_live_reachability: bool,
    /// Surface-local continuity note.
    pub local_continuity_note: String,
}

/// Failure or recovery drill proving descriptor behavior under degraded conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContinuityDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Drill scenario.
    pub scenario: ContinuityDrillScenario,
    /// Drill input summary.
    pub input_state: String,
    /// Surface refs covered by the drill.
    pub expected_surface_refs: Vec<PublicProofSurface>,
    /// Whether cached or stale labels remain visible.
    pub stale_or_cached_label_visible: bool,
    /// Whether destination classes remain accurate before exit.
    pub destination_classes_preserved_before_exit: bool,
    /// Whether local-only continuity remains visible.
    pub local_only_continuity_visible: bool,
    /// Whether save-later support semantics are verified.
    pub support_save_later_verified: bool,
    /// Whether the drill proves there is no implicit upload.
    pub no_implicit_upload: bool,
    /// Result summary.
    pub result_summary: String,
}

/// Local-first support export and save-later packet contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportSaveLaterContract {
    /// Stable support contract ref.
    pub support_contract_ref: String,
    /// Local report path ref.
    pub local_report_path_ref: String,
    /// Redaction profile ref.
    pub redaction_profile_ref: String,
    /// Destination class of the saved packet before explicit submit.
    pub destination_class: DestinationTrustClass,
    /// Save-later packet ref.
    pub save_later_packet_ref: String,
    /// Explicit submit action refs.
    pub explicit_submit_action_refs: Vec<String>,
    /// Whether support export is local-first.
    pub local_first: bool,
    /// Whether implicit upload is allowed.
    pub implicit_upload_allowed: bool,
    /// Whether the user can inspect contents before submit.
    pub inspect_before_submit_required: bool,
}

/// Canonical descriptor consumed by Help/About, service-health, release, migration, community, CLI, diagnostics, and support export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthDestinationTruthDescriptor {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable descriptor id.
    pub descriptor_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Build identity.
    pub build_identity: BuildIdentityDescriptor,
    /// Descriptor freshness rows.
    pub freshness: Vec<FreshnessDescriptor>,
    /// Destination trust-class manifest.
    pub destination_trust_classes: Vec<DestinationTrustClassManifest>,
    /// Service-health cards emitted by the shared aggregator.
    pub service_health_cards: Vec<ServiceHealthTruthCard>,
    /// Destination descriptors.
    pub destinations: Vec<DestinationDescriptor>,
    /// Surface bindings proving shared-source consumption.
    pub surface_bindings: Vec<SurfaceDescriptorBinding>,
    /// Offline, mirrored, browser-blocked, degraded, and partial-outage drills.
    pub continuity_drills: Vec<ContinuityDrill>,
    /// Local-first support save-later contract.
    pub support_save_later: SupportSaveLaterContract,
}

impl ServiceHealthDestinationTruthDescriptor {
    /// Returns the canonical contract-state vocabulary as stable tokens.
    pub fn service_contract_state_vocabulary(&self) -> Vec<String> {
        ServiceContractState::ALL
            .iter()
            .map(|state| state.as_str().to_owned())
            .collect()
    }

    /// Returns the canonical destination trust-class vocabulary as stable tokens.
    pub fn destination_trust_class_vocabulary(&self) -> Vec<String> {
        DestinationTrustClass::ALL
            .iter()
            .map(|class| class.as_str().to_owned())
            .collect()
    }

    /// Validates the descriptor against public-proof and local-only continuity invariants.
    pub fn validate(&self) -> ServiceHealthDestinationValidationReport {
        let mut validator = Validator::new(self);
        validator.run();
        validator.finish()
    }

    /// Builds a redaction-safe support export projection.
    pub fn support_export_projection(&self) -> ServiceHealthDestinationSupportExport {
        ServiceHealthDestinationSupportExport {
            record_kind: SERVICE_HEALTH_DESTINATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SERVICE_HEALTH_DESTINATION_SCHEMA_VERSION,
            descriptor_id: self.descriptor_id.clone(),
            build_identity_ref: self.build_identity.build_identity_ref.clone(),
            contract_state_vocabulary: self.service_contract_state_vocabulary(),
            destination_trust_class_vocabulary: self.destination_trust_class_vocabulary(),
            service_health_cards: self.service_health_cards.clone(),
            destination_summaries: self.destinations.clone(),
            support_save_later: self.support_save_later.clone(),
            surface_count: self.surface_bindings.len(),
            drill_count: self.continuity_drills.len(),
        }
    }

    /// Projects the destination descriptor into the shared service-health feed
    /// contract consumed across desktop, CLI/headless, Help/About, diagnostics,
    /// and support/export surfaces.
    pub fn shared_service_health_feed(&self) -> ServiceHealthFeed {
        let items = self
            .service_health_cards
            .iter()
            .map(|card| card.to_shared_feed_item(self))
            .collect::<Vec<_>>();

        ServiceHealthFeed {
            record_kind: SERVICE_HEALTH_FEED_RECORD_KIND.to_owned(),
            schema_version: SERVICE_HEALTH_FEED_SCHEMA_VERSION,
            feed_id: format!("{}:shared_feed", self.descriptor_id),
            shared_contract_ref: SERVICE_HEALTH_FEED_SHARED_CONTRACT_REF.to_owned(),
            schema_ref: SERVICE_HEALTH_FEED_SCHEMA_REF.to_owned(),
            items,
            surface_bindings: self
                .surface_bindings
                .iter()
                .map(|binding| ServiceHealthSurfaceBinding {
                    surface: binding.surface.into(),
                    feed_ref: self.descriptor_id.clone(),
                    item_refs: binding.service_health_card_refs.clone(),
                    consumes_shared_feed: binding.consumes_shared_descriptor,
                    last_checked_visible: true,
                    freshness_visible: binding.cached_or_offline_label_visible,
                    local_only_continuity_visible: binding.local_only_continuity_visible,
                    may_overclaim_live_reachability: binding.may_overclaim_live_reachability,
                    copyable_exportable: true,
                })
                .collect(),
        }
    }
}

impl ServiceHealthTruthCard {
    fn to_shared_feed_item(
        &self,
        descriptor: &ServiceHealthDestinationTruthDescriptor,
    ) -> ServiceHealthFeedItem {
        let freshness = descriptor
            .freshness
            .iter()
            .find(|row| row.freshness_ref == self.freshness_ref)
            .map(FreshnessDescriptor::to_shared_freshness)
            .unwrap_or_else(|| ServiceHealthFreshness {
                freshness_ref: self.freshness_ref.clone(),
                source_class: ServiceHealthSourceClass::CachedData,
                last_checked_at: self.last_checked_at.clone(),
                stale_after: self.last_checked_at.clone(),
                visible_freshness_label: self.visible_freshness_label.clone(),
                live_reachability_claim_allowed: false,
            });

        ServiceHealthFeedItem {
            schema_version: SERVICE_HEALTH_FEED_SCHEMA_VERSION,
            record_kind: SERVICE_HEALTH_FEED_ITEM_RECORD_KIND.to_owned(),
            item_id: self.card_id.clone(),
            service_family: self.service_family.clone(),
            boundary_class: self.boundary_class.clone(),
            contract_state: self.service_contract_state.into(),
            outage_scope: self.shared_outage_scope(),
            affected_workflows: self.affected_workflows.clone(),
            unaffected_workflows: unaffected_workflows_from_scope(&self.outage_scope),
            summary: self.outage_scope.clone(),
            freshness,
            diagnostics_actions: vec![self.diagnostics_action_ref.clone()],
            local_only_continuity_note: self.local_only_continuity_note.clone(),
            surfaced_on: self.surfaced_on.iter().copied().map(Into::into).collect(),
        }
    }

    fn shared_outage_scope(&self) -> ServiceHealthOutageScope {
        let scope = self.outage_scope.to_ascii_lowercase();
        if scope.contains("scheduled")
            || scope.contains("read-only")
            || scope.contains("read only")
            || scope.contains("drain")
            || scope.contains("migration")
            || scope.contains("failover")
            || scope.contains("reconcil")
            || scope.contains("resolved")
        {
            ServiceHealthOutageScope::MaintenanceWindow
        } else if self.service_contract_state == ServiceContractState::Ready {
            ServiceHealthOutageScope::None
        } else if scope.contains("unaffected")
            || scope.contains("remain available")
            || scope.contains("remain usable")
        {
            ServiceHealthOutageScope::PartialService
        } else {
            ServiceHealthOutageScope::SingleService
        }
    }
}

impl FreshnessDescriptor {
    fn to_shared_freshness(&self) -> ServiceHealthFreshness {
        ServiceHealthFreshness {
            freshness_ref: self.freshness_ref.clone(),
            source_class: match self.state {
                DescriptorFreshnessState::Live => ServiceHealthSourceClass::LivePolling,
                DescriptorFreshnessState::Cached | DescriptorFreshnessState::StaleCache => {
                    ServiceHealthSourceClass::CachedData
                }
                DescriptorFreshnessState::Mirrored => ServiceHealthSourceClass::MirroredNotice,
                DescriptorFreshnessState::OfflinePack => ServiceHealthSourceClass::OfflineBundle,
                DescriptorFreshnessState::PolicyLimited => ServiceHealthSourceClass::CachedData,
            },
            last_checked_at: self.last_checked_at.clone(),
            stale_after: self.stale_after.clone(),
            visible_freshness_label: self.visible_freshness_label.clone(),
            live_reachability_claim_allowed: self.live_reachability_claim_allowed,
        }
    }
}

impl From<ServiceContractState> for SharedContractState {
    fn from(value: ServiceContractState) -> Self {
        match value {
            ServiceContractState::Ready => Self::Ready,
            ServiceContractState::Degraded => Self::Degraded,
            ServiceContractState::LocalOnly => Self::LocalOnly,
            ServiceContractState::Stale => Self::Stale,
            ServiceContractState::ContractMismatch => Self::ContractMismatch,
            ServiceContractState::PolicyBlocked => Self::PolicyBlocked,
            ServiceContractState::Unavailable => Self::Unavailable,
        }
    }
}

impl From<PublicProofSurface> for ServiceHealthSurface {
    fn from(value: PublicProofSurface) -> Self {
        match value {
            PublicProofSurface::DesktopUi => Self::DesktopUi,
            PublicProofSurface::CliHeadless => Self::CliHeadless,
            PublicProofSurface::About => Self::About,
            PublicProofSurface::Help => Self::Help,
            PublicProofSurface::ServiceHealth => Self::ServiceHealth,
            PublicProofSurface::Diagnostics => Self::Diagnostics,
            PublicProofSurface::SupportExport => Self::SupportExport,
            PublicProofSurface::ReleaseNotes => Self::ReleaseNotes,
            PublicProofSurface::MigrationNotice => Self::MigrationNotice,
            PublicProofSurface::IssueReportTemplate => Self::IssueReportTemplate,
            PublicProofSurface::CommunityHandoff => Self::CommunityHandoff,
        }
    }
}

fn unaffected_workflows_from_scope(scope: &str) -> Vec<String> {
    let lower = scope.to_ascii_lowercase();
    let mut values = Vec::new();
    if lower.contains("local editing") || lower.contains("local edits") {
        values.push("editing".to_owned());
    }
    if lower.contains("installed docs") || lower.contains("local docs search") {
        values.push("installed_help".to_owned());
        values.push("local_docs_search".to_owned());
    }
    if lower.contains("search") {
        values.push("search".to_owned());
    }
    if lower.contains("git") {
        values.push("git".to_owned());
    }
    if lower.contains("diagnostics") {
        values.push("diagnostics".to_owned());
    }
    if lower.contains("installed extensions") {
        values.push("installed_extensions".to_owned());
    }
    values.sort();
    values.dedup();
    values
}

/// Validation report emitted for service-health destination truth descriptors.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthDestinationValidationReport {
    /// Descriptor id under validation.
    pub descriptor_id: String,
    /// Whether validation passed.
    pub passed: bool,
    /// Coverage observed during validation.
    pub coverage: ServiceHealthDestinationCoverage,
    /// Findings emitted by failed checks.
    pub findings: Vec<ServiceHealthDestinationFinding>,
}

/// Coverage observed during descriptor validation.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ServiceHealthDestinationCoverage {
    /// Contract states covered by service-health cards.
    pub service_contract_states: BTreeSet<ServiceContractState>,
    /// Destination classes covered by manifests and destinations.
    pub destination_classes: BTreeSet<DestinationTrustClass>,
    /// Public-proof surfaces covered by bindings.
    pub surfaces: BTreeSet<PublicProofSurface>,
    /// Continuity drill scenarios covered.
    pub drill_scenarios: BTreeSet<ContinuityDrillScenario>,
}

/// Validation finding severity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthDestinationFindingSeverity {
    /// Error that blocks validation.
    Error,
    /// Warning that leaves the descriptor reviewable but downgraded.
    Warning,
}

/// One validation finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthDestinationFinding {
    /// Finding severity.
    pub severity: ServiceHealthDestinationFindingSeverity,
    /// Stable check id.
    pub check_id: String,
    /// Redaction-safe finding message.
    pub message: String,
}

/// Redaction-safe support export projection for the descriptor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServiceHealthDestinationSupportExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Descriptor id.
    pub descriptor_id: String,
    /// Build identity ref.
    pub build_identity_ref: String,
    /// Contract-state vocabulary tokens.
    pub contract_state_vocabulary: Vec<String>,
    /// Destination trust-class vocabulary tokens.
    pub destination_trust_class_vocabulary: Vec<String>,
    /// Service-health cards included in the export projection.
    pub service_health_cards: Vec<ServiceHealthTruthCard>,
    /// Destination descriptors included in the export projection.
    pub destination_summaries: Vec<DestinationDescriptor>,
    /// Local-first save-later support contract.
    pub support_save_later: SupportSaveLaterContract,
    /// Number of surface bindings.
    pub surface_count: usize,
    /// Number of continuity drills.
    pub drill_count: usize,
}

/// Returns the canonical deterministic descriptor used by checked-in fixtures.
pub fn canonical_service_health_destination_truth_descriptor(
) -> ServiceHealthDestinationTruthDescriptor {
    ServiceHealthDestinationTruthDescriptor {
        record_kind: SERVICE_HEALTH_DESTINATION_RECORD_KIND.to_owned(),
        schema_version: SERVICE_HEALTH_DESTINATION_SCHEMA_VERSION,
        descriptor_id: "service_health_destination_truth:stable.public_proof".to_owned(),
        shared_contract_ref: SERVICE_HEALTH_DESTINATION_SHARED_CONTRACT_REF.to_owned(),
        schema_ref: SERVICE_HEALTH_DESTINATION_SCHEMA_REF.to_owned(),
        build_identity: BuildIdentityDescriptor {
            build_identity_ref: "build_identity:stable.public_proof".to_owned(),
            product_name: "Aureline".to_owned(),
            version: "0.0.0-stable-public-proof".to_owned(),
            build_id: "build:stable-public-proof-2026-06-06".to_owned(),
            channel: "stable".to_owned(),
            install_mode: "local_or_mirrored_install".to_owned(),
            provenance_ref: "artifacts/build/build_identity.json".to_owned(),
        },
        freshness: canonical_freshness(),
        destination_trust_classes: canonical_destination_trust_classes(),
        service_health_cards: canonical_service_health_cards(),
        destinations: canonical_destinations(),
        surface_bindings: canonical_surface_bindings(),
        continuity_drills: canonical_continuity_drills(),
        support_save_later: SupportSaveLaterContract {
            support_contract_ref: "support_export:local_save_later:v1".to_owned(),
            local_report_path_ref: "local-support-path:aureline-support-packets".to_owned(),
            redaction_profile_ref: "redaction-profile:metadata-safe-default".to_owned(),
            destination_class: DestinationTrustClass::LocalOnly,
            save_later_packet_ref: "save-later-packet:service-health-destination-truth".to_owned(),
            explicit_submit_action_refs: vec![
                "support.submit.public_issue_after_preview".to_owned(),
                "support.submit.official_authenticated_after_preview".to_owned(),
                "support.submit.vendor_managed_after_preview".to_owned(),
            ],
            local_first: true,
            implicit_upload_allowed: false,
            inspect_before_submit_required: true,
        },
    }
}

fn canonical_freshness() -> Vec<FreshnessDescriptor> {
    vec![
        FreshnessDescriptor {
            freshness_ref: "freshness:live-status-feed".to_owned(),
            state: DescriptorFreshnessState::Live,
            source_ref: "status-feed:official-public".to_owned(),
            last_checked_at: "2026-06-06T19:30:00Z".to_owned(),
            stale_after: "2026-06-06T19:45:00Z".to_owned(),
            visible_freshness_label: "Live status checked 2026-06-06T19:30:00Z".to_owned(),
            live_reachability_claim_allowed: true,
        },
        FreshnessDescriptor {
            freshness_ref: "freshness:offline-docs-pack".to_owned(),
            state: DescriptorFreshnessState::OfflinePack,
            source_ref: "docs-pack:stable-offline".to_owned(),
            last_checked_at: "2026-06-05T18:00:00Z".to_owned(),
            stale_after: "2026-06-12T18:00:00Z".to_owned(),
            visible_freshness_label: "Offline pack; live service reachability not checked"
                .to_owned(),
            live_reachability_claim_allowed: false,
        },
        FreshnessDescriptor {
            freshness_ref: "freshness:mirrored-release-notes".to_owned(),
            state: DescriptorFreshnessState::Mirrored,
            source_ref: "mirror:operator-release-notes".to_owned(),
            last_checked_at: "2026-06-06T18:00:00Z".to_owned(),
            stale_after: "2026-06-13T18:00:00Z".to_owned(),
            visible_freshness_label: "Mirrored release notes; source and age shown".to_owned(),
            live_reachability_claim_allowed: false,
        },
        FreshnessDescriptor {
            freshness_ref: "freshness:stale-community-cache".to_owned(),
            state: DescriptorFreshnessState::StaleCache,
            source_ref: "cache:community-handoff-last-known-good".to_owned(),
            last_checked_at: "2026-05-30T12:00:00Z".to_owned(),
            stale_after: "2026-06-02T12:00:00Z".to_owned(),
            visible_freshness_label: "Stale cache; do not infer current community governance"
                .to_owned(),
            live_reachability_claim_allowed: false,
        },
    ]
}

fn canonical_destination_trust_classes() -> Vec<DestinationTrustClassManifest> {
    vec![
        DestinationTrustClassManifest {
            destination_class: DestinationTrustClass::Public,
            visibility_boundary: "Public project space; visible outside the product".to_owned(),
            auth_expectation:
                "Readable without product sign-in; host sign-in may be required to post".to_owned(),
            data_exit_boundary: "Only reviewed, redacted report metadata may leave".to_owned(),
            issue_template_support: true,
            browser_blocked_fallback: "Save a local issue packet and open later".to_owned(),
            offline_fallback: "Save local packet; no live reachability claimed".to_owned(),
            pre_exit_label_required: true,
        },
        DestinationTrustClassManifest {
            destination_class: DestinationTrustClass::OfficialAuthenticated,
            visibility_boundary: "Official authenticated support or security lane".to_owned(),
            auth_expectation: "Project, support, or security identity required before submit"
                .to_owned(),
            data_exit_boundary: "Reviewed support or security packet exits after explicit submit"
                .to_owned(),
            issue_template_support: true,
            browser_blocked_fallback: "Save encrypted or redacted local packet for later submit"
                .to_owned(),
            offline_fallback: "Save authenticated-submit packet locally; retry later".to_owned(),
            pre_exit_label_required: true,
        },
        DestinationTrustClassManifest {
            destination_class: DestinationTrustClass::Community,
            visibility_boundary: "Community-run space; not guaranteed official support".to_owned(),
            auth_expectation: "Community account may be required by the host".to_owned(),
            data_exit_boundary: "Community-safe summary only; diagnostics stay local".to_owned(),
            issue_template_support: false,
            browser_blocked_fallback: "Save local community summary and open later".to_owned(),
            offline_fallback: "Keep the summary local; no current governance status claimed"
                .to_owned(),
            pre_exit_label_required: true,
        },
        DestinationTrustClassManifest {
            destination_class: DestinationTrustClass::VendorManaged,
            visibility_boundary: "Vendor-managed destination outside Aureline governance"
                .to_owned(),
            auth_expectation: "Vendor identity or provider session may be required".to_owned(),
            data_exit_boundary: "Vendor receives only the packet explicitly submitted by the user"
                .to_owned(),
            issue_template_support: true,
            browser_blocked_fallback: "Save vendor-support packet locally and submit later"
                .to_owned(),
            offline_fallback: "Save vendor-support packet locally; no vendor reachability claimed"
                .to_owned(),
            pre_exit_label_required: true,
        },
        DestinationTrustClassManifest {
            destination_class: DestinationTrustClass::LocalOnly,
            visibility_boundary: "Local machine only".to_owned(),
            auth_expectation: "No sign-in required".to_owned(),
            data_exit_boundary:
                "No data leaves unless the user later chooses an explicit submit action".to_owned(),
            issue_template_support: false,
            browser_blocked_fallback: "Already local; keep packet and retry external route later"
                .to_owned(),
            offline_fallback: "Remain local; all saved facts are visibly stale or cached"
                .to_owned(),
            pre_exit_label_required: true,
        },
    ]
}

fn canonical_service_health_cards() -> Vec<ServiceHealthTruthCard> {
    let all_surfaces = PublicProofSurface::REQUIRED.to_vec();
    vec![
        ServiceHealthTruthCard {
            card_id: "service-card:local-core".to_owned(),
            service_family: "local_core".to_owned(),
            boundary_class: "local_only".to_owned(),
            service_contract_state: ServiceContractState::Ready,
            affected_workflows: vec![],
            last_checked_at: "2026-06-06T19:29:00Z".to_owned(),
            freshness_ref: "freshness:live-status-feed".to_owned(),
            visible_freshness_label: "Live local-core check".to_owned(),
            outage_scope: "Local editing, save, search, Git, and diagnostics are available"
                .to_owned(),
            local_only_continuity_note: "Core local work does not require managed service reachability"
                .to_owned(),
            diagnostics_action_ref: "diagnostics.open.local_core".to_owned(),
            surfaced_on: all_surfaces.clone(),
        },
        ServiceHealthTruthCard {
            card_id: "service-card:docs-live".to_owned(),
            service_family: "docs_knowledge".to_owned(),
            boundary_class: "local_with_remote_optional".to_owned(),
            service_contract_state: ServiceContractState::Stale,
            affected_workflows: vec!["docs_browse_remote".to_owned()],
            last_checked_at: "2026-06-05T18:00:00Z".to_owned(),
            freshness_ref: "freshness:offline-docs-pack".to_owned(),
            visible_freshness_label: "Offline docs pack; live docs service not checked".to_owned(),
            outage_scope: "Remote docs enrichment may be stale; installed docs remain available"
                .to_owned(),
            local_only_continuity_note: "Local docs search and installed help pages continue from the offline pack"
                .to_owned(),
            diagnostics_action_ref: "diagnostics.open.docs_pack".to_owned(),
            surfaced_on: all_surfaces.clone(),
        },
        ServiceHealthTruthCard {
            card_id: "service-card:sync".to_owned(),
            service_family: "workspace_sync".to_owned(),
            boundary_class: "local_with_remote_required".to_owned(),
            service_contract_state: ServiceContractState::LocalOnly,
            affected_workflows: vec!["workspace_sync".to_owned(), "external_publish".to_owned()],
            last_checked_at: "2026-06-06T19:20:00Z".to_owned(),
            freshness_ref: "freshness:live-status-feed".to_owned(),
            visible_freshness_label: "Live check reports local-only continuity".to_owned(),
            outage_scope: "Sync and external publish are paused; local edits and review stay available"
                .to_owned(),
            local_only_continuity_note: "Save locally now and use publish-later after reconnect"
                .to_owned(),
            diagnostics_action_ref: "diagnostics.open.workspace_sync".to_owned(),
            surfaced_on: all_surfaces.clone(),
        },
        ServiceHealthTruthCard {
            card_id: "service-card:ai-provider".to_owned(),
            service_family: "ai_assist".to_owned(),
            boundary_class: "vendor_managed".to_owned(),
            service_contract_state: ServiceContractState::Degraded,
            affected_workflows: vec!["ai_completion".to_owned(), "ai_chat".to_owned()],
            last_checked_at: "2026-06-06T19:25:00Z".to_owned(),
            freshness_ref: "freshness:live-status-feed".to_owned(),
            visible_freshness_label: "Live provider check".to_owned(),
            outage_scope: "AI assistance is slow; editor, local search, Git, and diagnostics are unaffected"
                .to_owned(),
            local_only_continuity_note: "Use local editing and diagnostics while managed AI retries"
                .to_owned(),
            diagnostics_action_ref: "diagnostics.open.ai_provider".to_owned(),
            surfaced_on: all_surfaces.clone(),
        },
        ServiceHealthTruthCard {
            card_id: "service-card:release-feed".to_owned(),
            service_family: "release_feed".to_owned(),
            boundary_class: "public".to_owned(),
            service_contract_state: ServiceContractState::ContractMismatch,
            affected_workflows: vec!["release_notes_live_refresh".to_owned()],
            last_checked_at: "2026-06-06T18:00:00Z".to_owned(),
            freshness_ref: "freshness:mirrored-release-notes".to_owned(),
            visible_freshness_label: "Mirrored notes shown; live feed contract mismatch".to_owned(),
            outage_scope: "Live release-note refresh is held; mirrored notes stay inspectable"
                .to_owned(),
            local_only_continuity_note: "Use checked-in release notes and support export refs until live contract clears"
                .to_owned(),
            diagnostics_action_ref: "diagnostics.open.release_feed".to_owned(),
            surfaced_on: all_surfaces.clone(),
        },
        ServiceHealthTruthCard {
            card_id: "service-card:telemetry-upload".to_owned(),
            service_family: "telemetry_upload".to_owned(),
            boundary_class: "hosted_optional".to_owned(),
            service_contract_state: ServiceContractState::PolicyBlocked,
            affected_workflows: vec!["telemetry_upload".to_owned()],
            last_checked_at: "2026-06-06T19:15:00Z".to_owned(),
            freshness_ref: "freshness:live-status-feed".to_owned(),
            visible_freshness_label: "Live policy state".to_owned(),
            outage_scope: "Telemetry upload is blocked by policy; local crash capture continues"
                .to_owned(),
            local_only_continuity_note: "Support packets save locally and require explicit submit"
                .to_owned(),
            diagnostics_action_ref: "diagnostics.open.telemetry_policy".to_owned(),
            surfaced_on: all_surfaces.clone(),
        },
        ServiceHealthTruthCard {
            card_id: "service-card:marketplace".to_owned(),
            service_family: "marketplace".to_owned(),
            boundary_class: "hosted_optional".to_owned(),
            service_contract_state: ServiceContractState::Unavailable,
            affected_workflows: vec!["marketplace_browse".to_owned(), "extension_install".to_owned()],
            last_checked_at: "2026-06-06T19:18:00Z".to_owned(),
            freshness_ref: "freshness:live-status-feed".to_owned(),
            visible_freshness_label: "Live check reports hosted outage".to_owned(),
            outage_scope: "Marketplace browse and extension install are unavailable; installed extensions remain usable"
                .to_owned(),
            local_only_continuity_note: "Existing local workspace and installed extension workflows continue"
                .to_owned(),
            diagnostics_action_ref: "diagnostics.open.marketplace".to_owned(),
            surfaced_on: all_surfaces,
        },
    ]
}

fn canonical_destinations() -> Vec<DestinationDescriptor> {
    vec![
        destination(
            "destination:public-issue",
            "Public issue tracker",
            "Open-source bugs, regressions, and docs-truth defects",
            DestinationTrustClass::Public,
            &[
                PublicProofSurface::Help,
                PublicProofSurface::About,
                PublicProofSurface::IssueReportTemplate,
                PublicProofSurface::CommunityHandoff,
                PublicProofSurface::ReleaseNotes,
                PublicProofSurface::MigrationNotice,
            ],
        ),
        destination(
            "destination:official-security",
            "Official security disclosure",
            "Private security and vulnerability intake",
            DestinationTrustClass::OfficialAuthenticated,
            &[
                PublicProofSurface::Help,
                PublicProofSurface::About,
                PublicProofSurface::IssueReportTemplate,
                PublicProofSurface::CommunityHandoff,
                PublicProofSurface::SupportExport,
            ],
        ),
        destination(
            "destination:community-discussion",
            "Community discussion",
            "Community help and design discussion without guaranteed official support",
            DestinationTrustClass::Community,
            &[
                PublicProofSurface::Help,
                PublicProofSurface::About,
                PublicProofSurface::CommunityHandoff,
            ],
        ),
        destination(
            "destination:vendor-support",
            "Vendor-managed support",
            "Provider-owned support lane for selected integrations",
            DestinationTrustClass::VendorManaged,
            &[
                PublicProofSurface::Help,
                PublicProofSurface::SupportExport,
                PublicProofSurface::CommunityHandoff,
            ],
        ),
        destination(
            "destination:save-later-local",
            "Save locally for later",
            "Local-only support packet, issue draft, or copied build facts",
            DestinationTrustClass::LocalOnly,
            &[
                PublicProofSurface::DesktopUi,
                PublicProofSurface::CliHeadless,
                PublicProofSurface::About,
                PublicProofSurface::Help,
                PublicProofSurface::ServiceHealth,
                PublicProofSurface::Diagnostics,
                PublicProofSurface::SupportExport,
                PublicProofSurface::ReleaseNotes,
                PublicProofSurface::MigrationNotice,
                PublicProofSurface::IssueReportTemplate,
                PublicProofSurface::CommunityHandoff,
            ],
        ),
    ]
}

fn destination(
    id: &str,
    title: &str,
    purpose: &str,
    class: DestinationTrustClass,
    surfaces: &[PublicProofSurface],
) -> DestinationDescriptor {
    let (visibility, auth, data, issue) = match class {
        DestinationTrustClass::Public => (
            "Public project space; visible outside the product",
            "Readable without product sign-in; host sign-in may be required to post",
            "Reviewed redacted metadata exits only after preview",
            "Uses public issue templates",
        ),
        DestinationTrustClass::OfficialAuthenticated => (
            "Official authenticated support or security lane",
            "Project, support, or security identity required",
            "Reviewed support or security packet exits after explicit submit",
            "Uses official authenticated templates",
        ),
        DestinationTrustClass::Community => (
            "Community-run space; not guaranteed official support",
            "Community account may be required",
            "Community-safe summary only; diagnostics stay local",
            "No official support template guarantee",
        ),
        DestinationTrustClass::VendorManaged => (
            "Vendor-managed destination outside Aureline governance",
            "Vendor identity or provider session may be required",
            "Vendor receives only the explicitly submitted packet",
            "Uses vendor template when available",
        ),
        DestinationTrustClass::LocalOnly => (
            "Local machine only",
            "No sign-in required",
            "No data leaves unless the user later chooses explicit submit",
            "Local save-later packet",
        ),
    };
    DestinationDescriptor {
        destination_id: id.to_owned(),
        title: title.to_owned(),
        purpose: purpose.to_owned(),
        destination_class: class,
        visibility_boundary: visibility.to_owned(),
        auth_expectation: auth.to_owned(),
        data_exit_boundary: data.to_owned(),
        issue_template_support: issue.to_owned(),
        browser_blocked_fallback_ref: "fallback:save-local-open-later".to_owned(),
        offline_fallback_ref: "fallback:save-local-offline".to_owned(),
        surfaces: surfaces.to_vec(),
    }
}

fn canonical_surface_bindings() -> Vec<SurfaceDescriptorBinding> {
    PublicProofSurface::REQUIRED
        .iter()
        .copied()
        .map(|surface| SurfaceDescriptorBinding {
            surface,
            descriptor_ref: SERVICE_HEALTH_DESTINATION_CANONICAL_DESCRIPTOR_REF.to_owned(),
            build_identity_ref: "build_identity:stable.public_proof".to_owned(),
            freshness_refs: vec![
                "freshness:live-status-feed".to_owned(),
                "freshness:offline-docs-pack".to_owned(),
                "freshness:mirrored-release-notes".to_owned(),
                "freshness:stale-community-cache".to_owned(),
            ],
            service_health_card_refs: canonical_card_refs_for(surface),
            destination_refs: canonical_destination_refs_for(surface),
            consumes_shared_descriptor: true,
            requires_sign_in_to_read: false,
            cached_or_offline_label_visible: true,
            local_only_continuity_visible: true,
            may_overclaim_live_reachability: false,
            local_continuity_note: format!(
                "{} consumes the shared descriptor and keeps local-only continuity visible",
                surface.as_str()
            ),
        })
        .collect()
}

fn canonical_card_refs_for(_surface: PublicProofSurface) -> Vec<String> {
    [
        "service-card:local-core",
        "service-card:docs-live",
        "service-card:sync",
        "service-card:ai-provider",
        "service-card:release-feed",
        "service-card:telemetry-upload",
        "service-card:marketplace",
    ]
    .iter()
    .map(|id| (*id).to_owned())
    .collect()
}

fn canonical_destination_refs_for(surface: PublicProofSurface) -> Vec<String> {
    let mut refs = vec!["destination:save-later-local".to_owned()];
    match surface {
        PublicProofSurface::About
        | PublicProofSurface::Help
        | PublicProofSurface::ReleaseNotes
        | PublicProofSurface::MigrationNotice
        | PublicProofSurface::IssueReportTemplate => {
            refs.push("destination:public-issue".to_owned());
            refs.push("destination:official-security".to_owned());
        }
        PublicProofSurface::CommunityHandoff => {
            refs.push("destination:public-issue".to_owned());
            refs.push("destination:official-security".to_owned());
            refs.push("destination:community-discussion".to_owned());
            refs.push("destination:vendor-support".to_owned());
        }
        PublicProofSurface::SupportExport => {
            refs.push("destination:official-security".to_owned());
            refs.push("destination:vendor-support".to_owned());
        }
        PublicProofSurface::DesktopUi
        | PublicProofSurface::CliHeadless
        | PublicProofSurface::ServiceHealth
        | PublicProofSurface::Diagnostics => {}
    }
    refs
}

fn canonical_continuity_drills() -> Vec<ContinuityDrill> {
    vec![
        drill(
            "drill:offline-local-only-continuity",
            ContinuityDrillScenario::Offline,
            "Network disabled; descriptor served from offline pack and local support packet saved",
            &[
                PublicProofSurface::About,
                PublicProofSurface::Help,
                PublicProofSurface::ServiceHealth,
                PublicProofSurface::CliHeadless,
                PublicProofSurface::SupportExport,
            ],
        ),
        drill(
            "drill:mirrored-release-notes",
            ContinuityDrillScenario::Mirrored,
            "Release notes and migration notices read from operator mirror",
            &[
                PublicProofSurface::ReleaseNotes,
                PublicProofSurface::MigrationNotice,
                PublicProofSurface::Help,
            ],
        ),
        drill(
            "drill:browser-blocked-community-handoff",
            ContinuityDrillScenario::BrowserBlocked,
            "External browser launch blocked by policy; handoff packet remains local-only",
            &[
                PublicProofSurface::IssueReportTemplate,
                PublicProofSurface::CommunityHandoff,
                PublicProofSurface::SupportExport,
            ],
        ),
        drill(
            "drill:degraded-managed-ai",
            ContinuityDrillScenario::DegradedService,
            "Vendor-managed AI assist degraded; local editing and diagnostics remain safe",
            &[
                PublicProofSurface::DesktopUi,
                PublicProofSurface::ServiceHealth,
                PublicProofSurface::Diagnostics,
                PublicProofSurface::CliHeadless,
            ],
        ),
        drill(
            "drill:partial-marketplace-outage",
            ContinuityDrillScenario::PartialServiceOutage,
            "Marketplace unavailable; installed extensions and local core remain usable",
            &[
                PublicProofSurface::About,
                PublicProofSurface::Help,
                PublicProofSurface::ServiceHealth,
                PublicProofSurface::SupportExport,
            ],
        ),
    ]
}

fn drill(
    id: &str,
    scenario: ContinuityDrillScenario,
    input_state: &str,
    surfaces: &[PublicProofSurface],
) -> ContinuityDrill {
    ContinuityDrill {
        drill_id: id.to_owned(),
        scenario,
        input_state: input_state.to_owned(),
        expected_surface_refs: surfaces.to_vec(),
        stale_or_cached_label_visible: true,
        destination_classes_preserved_before_exit: true,
        local_only_continuity_visible: true,
        support_save_later_verified: true,
        no_implicit_upload: true,
        result_summary:
            "Surface labels stayed shared, stale/local-only continuity remained visible, and no upload occurred"
                .to_owned(),
    }
}

struct Validator<'a> {
    descriptor: &'a ServiceHealthDestinationTruthDescriptor,
    findings: Vec<ServiceHealthDestinationFinding>,
    coverage: ServiceHealthDestinationCoverage,
}

impl<'a> Validator<'a> {
    fn new(descriptor: &'a ServiceHealthDestinationTruthDescriptor) -> Self {
        Self {
            descriptor,
            findings: Vec::new(),
            coverage: ServiceHealthDestinationCoverage::default(),
        }
    }

    fn run(&mut self) {
        self.validate_envelope();
        self.validate_destination_classes();
        self.validate_destinations();
        self.validate_service_health_cards();
        self.validate_surface_bindings();
        self.validate_continuity_drills();
        self.validate_support_save_later();
    }

    fn finish(self) -> ServiceHealthDestinationValidationReport {
        let passed = self
            .findings
            .iter()
            .all(|finding| finding.severity != ServiceHealthDestinationFindingSeverity::Error);
        ServiceHealthDestinationValidationReport {
            descriptor_id: self.descriptor.descriptor_id.clone(),
            passed,
            coverage: self.coverage,
            findings: self.findings,
        }
    }

    fn error(&mut self, check_id: &str, message: impl Into<String>) {
        self.findings.push(ServiceHealthDestinationFinding {
            severity: ServiceHealthDestinationFindingSeverity::Error,
            check_id: check_id.to_owned(),
            message: message.into(),
        });
    }

    fn validate_envelope(&mut self) {
        if self.descriptor.record_kind != SERVICE_HEALTH_DESTINATION_RECORD_KIND {
            self.error(
                "descriptor.record_kind",
                "descriptor record_kind is not canonical",
            );
        }
        if self.descriptor.schema_version != SERVICE_HEALTH_DESTINATION_SCHEMA_VERSION {
            self.error(
                "descriptor.schema_version",
                "descriptor schema_version is unsupported",
            );
        }
        if self.descriptor.shared_contract_ref != SERVICE_HEALTH_DESTINATION_SHARED_CONTRACT_REF {
            self.error(
                "descriptor.shared_contract_ref",
                "descriptor shared contract ref drifted",
            );
        }
        if self
            .descriptor
            .build_identity
            .build_identity_ref
            .trim()
            .is_empty()
            || self.descriptor.build_identity.channel.trim().is_empty()
            || self
                .descriptor
                .build_identity
                .provenance_ref
                .trim()
                .is_empty()
        {
            self.error(
                "descriptor.build_identity",
                "build identity must include ref, channel, and provenance",
            );
        }
    }

    fn validate_destination_classes(&mut self) {
        for row in &self.descriptor.destination_trust_classes {
            self.coverage
                .destination_classes
                .insert(row.destination_class);
            if row.visibility_boundary.trim().is_empty()
                || row.auth_expectation.trim().is_empty()
                || row.data_exit_boundary.trim().is_empty()
                || row.browser_blocked_fallback.trim().is_empty()
                || row.offline_fallback.trim().is_empty()
            {
                self.error(
                    "destination_class.required_fields",
                    format!(
                        "destination class {} is missing boundary fields",
                        row.destination_class.as_str()
                    ),
                );
            }
            if !row.pre_exit_label_required {
                self.error(
                    "destination_class.pre_exit_label",
                    format!(
                        "destination class {} does not require pre-exit labeling",
                        row.destination_class.as_str()
                    ),
                );
            }
        }
        for required in DestinationTrustClass::ALL {
            if !self.coverage.destination_classes.contains(&required) {
                self.error(
                    "destination_class.coverage",
                    format!("missing destination class {}", required.as_str()),
                );
            }
        }
    }

    fn validate_destinations(&mut self) {
        let class_set: BTreeSet<_> = self
            .descriptor
            .destination_trust_classes
            .iter()
            .map(|row| row.destination_class)
            .collect();
        let mut ids = BTreeSet::new();
        for dest in &self.descriptor.destinations {
            if !ids.insert(dest.destination_id.as_str()) {
                self.error(
                    "destination.unique_id",
                    format!("duplicate destination id {}", dest.destination_id),
                );
            }
            self.coverage
                .destination_classes
                .insert(dest.destination_class);
            if !class_set.contains(&dest.destination_class) {
                self.error(
                    "destination.class_manifest",
                    format!(
                        "destination {} uses class without manifest",
                        dest.destination_id
                    ),
                );
            }
            if dest.title.trim().is_empty()
                || dest.purpose.trim().is_empty()
                || dest.visibility_boundary.trim().is_empty()
                || dest.auth_expectation.trim().is_empty()
                || dest.data_exit_boundary.trim().is_empty()
                || dest.issue_template_support.trim().is_empty()
                || dest.browser_blocked_fallback_ref.trim().is_empty()
                || dest.offline_fallback_ref.trim().is_empty()
            {
                self.error(
                    "destination.required_fields",
                    format!(
                        "destination {} is missing required boundary fields",
                        dest.destination_id
                    ),
                );
            }
        }
    }

    fn validate_service_health_cards(&mut self) {
        let freshness = self.freshness_refs();
        for card in &self.descriptor.service_health_cards {
            self.coverage
                .service_contract_states
                .insert(card.service_contract_state);
            if card.service_family.trim().is_empty()
                || card.boundary_class.trim().is_empty()
                || card.last_checked_at.trim().is_empty()
                || card.visible_freshness_label.trim().is_empty()
                || card.outage_scope.trim().is_empty()
                || card.local_only_continuity_note.trim().is_empty()
                || card.diagnostics_action_ref.trim().is_empty()
            {
                self.error(
                    "service_card.required_fields",
                    format!(
                        "service card {} is missing required visible fields",
                        card.card_id
                    ),
                );
            }
            if !freshness.contains(&card.freshness_ref) {
                self.error(
                    "service_card.freshness_ref",
                    format!(
                        "service card {} points at an unknown freshness ref",
                        card.card_id
                    ),
                );
            }
            if self
                .freshness_for(&card.freshness_ref)
                .map(|f| {
                    f.state.requires_cached_or_stale_label() && f.live_reachability_claim_allowed
                })
                .unwrap_or(false)
            {
                self.error(
                    "freshness.live_reachability",
                    format!(
                        "freshness ref {} may overclaim live reachability",
                        card.freshness_ref
                    ),
                );
            }
        }
        for required in ServiceContractState::ALL {
            if !self.coverage.service_contract_states.contains(&required) {
                self.error(
                    "service_contract_state.coverage",
                    format!("missing service contract state {}", required.as_str()),
                );
            }
        }
    }

    fn validate_surface_bindings(&mut self) {
        let freshness = self.freshness_refs();
        let cards = self.card_refs();
        let destinations = self.destination_refs();
        for binding in &self.descriptor.surface_bindings {
            self.coverage.surfaces.insert(binding.surface);
            if !binding.consumes_shared_descriptor {
                self.error(
                    "surface.shared_descriptor",
                    format!(
                        "{} does not consume the shared descriptor",
                        binding.surface.as_str()
                    ),
                );
            }
            if binding.requires_sign_in_to_read {
                self.error(
                    "surface.sign_in_required",
                    format!(
                        "{} requires sign-in to read public-proof facts",
                        binding.surface.as_str()
                    ),
                );
            }
            if !binding.cached_or_offline_label_visible {
                self.error(
                    "surface.cached_label",
                    format!(
                        "{} hides cached/offline freshness",
                        binding.surface.as_str()
                    ),
                );
            }
            if !binding.local_only_continuity_visible {
                self.error(
                    "surface.local_continuity",
                    format!("{} hides local-only continuity", binding.surface.as_str()),
                );
            }
            if binding.may_overclaim_live_reachability {
                self.error(
                    "surface.live_reachability",
                    format!(
                        "{} may overclaim live reachability",
                        binding.surface.as_str()
                    ),
                );
            }
            if binding.build_identity_ref != self.descriptor.build_identity.build_identity_ref {
                self.error(
                    "surface.build_identity_ref",
                    format!(
                        "{} does not bind the canonical build identity",
                        binding.surface.as_str()
                    ),
                );
            }
            if binding.freshness_refs.is_empty()
                || binding
                    .freshness_refs
                    .iter()
                    .any(|r| !freshness.contains(r))
            {
                self.error(
                    "surface.freshness_refs",
                    format!(
                        "{} has missing or unknown freshness refs",
                        binding.surface.as_str()
                    ),
                );
            }
            if binding.service_health_card_refs.is_empty()
                || binding
                    .service_health_card_refs
                    .iter()
                    .any(|r| !cards.contains(r))
            {
                self.error(
                    "surface.service_health_refs",
                    format!(
                        "{} has missing or unknown service-health refs",
                        binding.surface.as_str()
                    ),
                );
            }
            if binding.destination_refs.is_empty()
                || binding
                    .destination_refs
                    .iter()
                    .any(|r| !destinations.contains(r))
            {
                self.error(
                    "surface.destination_refs",
                    format!(
                        "{} has missing or unknown destination refs",
                        binding.surface.as_str()
                    ),
                );
            }
        }
        for required in PublicProofSurface::REQUIRED {
            if !self.coverage.surfaces.contains(&required) {
                self.error(
                    "surface.coverage",
                    format!("missing surface binding {}", required.as_str()),
                );
            }
        }
    }

    fn validate_continuity_drills(&mut self) {
        for drill in &self.descriptor.continuity_drills {
            self.coverage.drill_scenarios.insert(drill.scenario);
            if drill.expected_surface_refs.is_empty()
                || !drill.stale_or_cached_label_visible
                || !drill.destination_classes_preserved_before_exit
                || !drill.local_only_continuity_visible
                || !drill.support_save_later_verified
                || !drill.no_implicit_upload
            {
                self.error(
                    "drill.required_proof",
                    format!("drill {} does not prove freshness, destination, continuity, and save-later behavior", drill.drill_id),
                );
            }
        }
        for scenario in [
            ContinuityDrillScenario::Offline,
            ContinuityDrillScenario::Mirrored,
            ContinuityDrillScenario::BrowserBlocked,
            ContinuityDrillScenario::DegradedService,
            ContinuityDrillScenario::PartialServiceOutage,
        ] {
            if !self.coverage.drill_scenarios.contains(&scenario) {
                self.error(
                    "drill.coverage",
                    format!("missing continuity drill scenario {:?}", scenario),
                );
            }
        }
    }

    fn validate_support_save_later(&mut self) {
        let support = &self.descriptor.support_save_later;
        if support.destination_class != DestinationTrustClass::LocalOnly {
            self.error(
                "support.destination_class",
                "support save-later packet must be local-only before explicit submit",
            );
        }
        if !support.local_first
            || support.implicit_upload_allowed
            || !support.inspect_before_submit_required
            || support.explicit_submit_action_refs.is_empty()
            || support.save_later_packet_ref.trim().is_empty()
            || support.local_report_path_ref.trim().is_empty()
            || support.redaction_profile_ref.trim().is_empty()
        {
            self.error(
                "support.local_first",
                "support export must be local-first, inspectable, save-later, and explicit-submit only",
            );
        }
    }

    fn freshness_refs(&self) -> BTreeSet<String> {
        self.descriptor
            .freshness
            .iter()
            .map(|f| f.freshness_ref.clone())
            .collect()
    }

    fn card_refs(&self) -> BTreeSet<String> {
        self.descriptor
            .service_health_cards
            .iter()
            .map(|c| c.card_id.clone())
            .collect()
    }

    fn destination_refs(&self) -> BTreeSet<String> {
        self.descriptor
            .destinations
            .iter()
            .map(|d| d.destination_id.clone())
            .collect()
    }

    fn freshness_for(&self, freshness_ref: &str) -> Option<&FreshnessDescriptor> {
        self.descriptor
            .freshness
            .iter()
            .find(|f| f.freshness_ref == freshness_ref)
    }
}
