//! Framework support / status strip record model.

use serde::{Deserialize, Serialize};

use super::{Finding, FreshnessClass, SurfaceClass};

/// Stable record-kind tag for serialized [`FrameworkSupportStrip`] payloads.
pub const FRAMEWORK_SUPPORT_STRIP_RECORD_KIND: &str = "framework_support_strip_record";

/// Schema version for the [`FrameworkSupportStrip`] payload shape.
pub const FRAMEWORK_SUPPORT_STRIP_SCHEMA_VERSION: u32 = 1;

/// Re-exported freshness tokens so consumers don't need to import the
/// enum just to compare strings against a fixture.
pub const FRAMEWORK_FRESHNESS_AUTHORITATIVE_LIVE: &str = "authoritative_live";
pub const FRAMEWORK_FRESHNESS_WARM_CACHED: &str = "warm_cached";
pub const FRAMEWORK_FRESHNESS_DEGRADED_CACHED: &str = "degraded_cached";
pub const FRAMEWORK_FRESHNESS_STALE: &str = "stale";
pub const FRAMEWORK_FRESHNESS_UNVERIFIED: &str = "unverified";

/// Re-export of the framework adapter family used by the support strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkFamilyClass {
    ReactOrJsxFamily,
    VueOrTemplateFamily,
    SvelteOrCompileFamily,
    AngularOrDecoratorFamily,
    SolidOrSignalFamily,
    WebComponentsFamily,
    FlutterWidgetFamily,
    SwiftuiWidgetFamily,
    ComposeWidgetFamily,
    HtmlDomPassthrough,
    StaticMarkdownRenderer,
    DesignTokenOnlyAdapter,
    UnsupportedOrUnclaimedFamily,
}

impl FrameworkFamilyClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReactOrJsxFamily => "react_or_jsx_family",
            Self::VueOrTemplateFamily => "vue_or_template_family",
            Self::SvelteOrCompileFamily => "svelte_or_compile_family",
            Self::AngularOrDecoratorFamily => "angular_or_decorator_family",
            Self::SolidOrSignalFamily => "solid_or_signal_family",
            Self::WebComponentsFamily => "web_components_family",
            Self::FlutterWidgetFamily => "flutter_widget_family",
            Self::SwiftuiWidgetFamily => "swiftui_widget_family",
            Self::ComposeWidgetFamily => "compose_widget_family",
            Self::HtmlDomPassthrough => "html_dom_passthrough",
            Self::StaticMarkdownRenderer => "static_markdown_renderer",
            Self::DesignTokenOnlyAdapter => "design_token_only_adapter",
            Self::UnsupportedOrUnclaimedFamily => "unsupported_or_unclaimed_family",
        }
    }
}

/// Closed support-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportClass {
    CoreNative,
    FrameworkPack,
    BridgeCompatibilityLayer,
    HeuristicConventionMode,
    UnsupportedOrUnclaimed,
}

impl SupportClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoreNative => "core_native",
            Self::FrameworkPack => "framework_pack",
            Self::BridgeCompatibilityLayer => "bridge_compatibility_layer",
            Self::HeuristicConventionMode => "heuristic_convention_mode",
            Self::UnsupportedOrUnclaimed => "unsupported_or_unclaimed",
        }
    }

    /// Whether this support class admits "exact" certainty labels at the
    /// row / convention-diagnostic / generator-preview level.
    pub const fn admits_exact_certainty(self) -> bool {
        matches!(self, Self::CoreNative | Self::FrameworkPack)
    }
}

/// Closed pack / bridge source vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackSourceClass {
    FirstPartyNative,
    GovernedFrameworkPack,
    CommunityFrameworkPack,
    BridgeCompatibilityLayer,
    HeuristicConventionOnly,
    ImportedSnapshotOnly,
    NoPackOrBridge,
}

impl PackSourceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyNative => "first_party_native",
            Self::GovernedFrameworkPack => "governed_framework_pack",
            Self::CommunityFrameworkPack => "community_framework_pack",
            Self::BridgeCompatibilityLayer => "bridge_compatibility_layer",
            Self::HeuristicConventionOnly => "heuristic_convention_only",
            Self::ImportedSnapshotOnly => "imported_snapshot_only",
            Self::NoPackOrBridge => "no_pack_or_bridge",
        }
    }

    /// Whether this pack source admits framework-pack action labels
    /// (`request_pack_update`, `open_pack_status`).
    pub const fn admits_pack_actions(self) -> bool {
        matches!(
            self,
            Self::FirstPartyNative
                | Self::GovernedFrameworkPack
                | Self::CommunityFrameworkPack
                | Self::BridgeCompatibilityLayer
        )
    }
}

/// Closed health-class vocabulary for the support strip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthClass {
    HealthyLive,
    DegradedPartial,
    StaleCache,
    UnreachableRuntime,
    PackCapabilityMissing,
    UnknownHealth,
}

impl HealthClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HealthyLive => "healthy_live",
            Self::DegradedPartial => "degraded_partial",
            Self::StaleCache => "stale_cache",
            Self::UnreachableRuntime => "unreachable_runtime",
            Self::PackCapabilityMissing => "pack_capability_missing",
            Self::UnknownHealth => "unknown_health",
        }
    }
}

/// Closed locality vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalityClass {
    LocalWorkspace,
    RemoteWorkspace,
    ManagedWorkspace,
    MixedLocalAndRemote,
    NotebookKernelOnly,
    ExtensionHostOnly,
    UnknownScope,
}

impl LocalityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalWorkspace => "local_workspace",
            Self::RemoteWorkspace => "remote_workspace",
            Self::ManagedWorkspace => "managed_workspace",
            Self::MixedLocalAndRemote => "mixed_local_and_remote",
            Self::NotebookKernelOnly => "notebook_kernel_only",
            Self::ExtensionHostOnly => "extension_host_only",
            Self::UnknownScope => "unknown_scope",
        }
    }
}

/// Closed version-compatibility vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VersionCompatibilityClass {
    WithinSupportedRange,
    BelowSupportedRange,
    AboveSupportedRange,
    BetweenVersions,
    UnsupportedVersion,
    UnknownVersion,
    VersionNotApplicable,
}

impl VersionCompatibilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WithinSupportedRange => "within_supported_range",
            Self::BelowSupportedRange => "below_supported_range",
            Self::AboveSupportedRange => "above_supported_range",
            Self::BetweenVersions => "between_versions",
            Self::UnsupportedVersion => "unsupported_version",
            Self::UnknownVersion => "unknown_version",
            Self::VersionNotApplicable => "version_not_applicable",
        }
    }
}

/// Closed action vocabulary the strip may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkSupportActionClass {
    OpenCompatibilityDetails,
    OpenPackStatus,
    OpenPackDocs,
    OpenMigrationPath,
    OpenRawSourceFallback,
    RequestPackInstall,
    RequestPackUpdate,
    RequestPolicyReview,
    OpenRuntimeInspector,
}

impl FrameworkSupportActionClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenCompatibilityDetails => "open_compatibility_details",
            Self::OpenPackStatus => "open_pack_status",
            Self::OpenPackDocs => "open_pack_docs",
            Self::OpenMigrationPath => "open_migration_path",
            Self::OpenRawSourceFallback => "open_raw_source_fallback",
            Self::RequestPackInstall => "request_pack_install",
            Self::RequestPackUpdate => "request_pack_update",
            Self::RequestPolicyReview => "request_policy_review",
            Self::OpenRuntimeInspector => "open_runtime_inspector",
        }
    }
}

/// Framework identity block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkIdentityBlock {
    pub framework_family_class: FrameworkFamilyClass,
    pub framework_name_label: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework_version_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework_version_range_label: Option<String>,
}

/// Pack / bridge source block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PackOrBridgeSourceBlock {
    pub pack_source_class: PackSourceClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_version_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bridge_label: Option<String>,
}

/// Health / freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthBlock {
    pub health_class: HealthClass,
    pub freshness_class: FreshnessClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_refresh_at: Option<String>,
}

/// Scope block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeBlock {
    pub locality_class: LocalityClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_scope_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_scope_ref: Option<String>,
}

/// Compatibility block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompatibilityBlock {
    pub version_compatibility_class: VersionCompatibilityClass,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub supported_version_range_label: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub downgraded_behavior_summary: Option<String>,
}

/// Canonical framework support / status strip record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkSupportStrip {
    pub record_kind: String,
    pub framework_support_strip_schema_version: u32,
    pub framework_support_strip_id: String,
    pub captured_at: String,
    pub surface_class: SurfaceClass,
    pub framework_identity_block: FrameworkIdentityBlock,
    pub support_class: SupportClass,
    pub pack_or_bridge_source_block: PackOrBridgeSourceBlock,
    pub health_block: HealthBlock,
    pub scope_block: ScopeBlock,
    pub compatibility_block: CompatibilityBlock,
    pub actions: Vec<FrameworkSupportActionClass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework_certainty_row_record_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_sync_chip_record_ref: Option<String>,
    pub summary: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl FrameworkSupportStrip {
    /// Returns typed truth-rule findings; an empty vector means the strip
    /// is internally consistent with the schema's allOf rules.
    pub fn validate(&self) -> Vec<Finding> {
        let mut findings = Vec::new();
        let subject = self.framework_support_strip_id.as_str();

        if self.record_kind != FRAMEWORK_SUPPORT_STRIP_RECORD_KIND {
            findings.push(Finding::new(
                "framework_support_strip.record_kind",
                subject,
                format!(
                    "record_kind must be '{}', found '{}'",
                    FRAMEWORK_SUPPORT_STRIP_RECORD_KIND, self.record_kind
                ),
            ));
        }
        if self.framework_support_strip_schema_version != FRAMEWORK_SUPPORT_STRIP_SCHEMA_VERSION {
            findings.push(Finding::new(
                "framework_support_strip.schema_version",
                subject,
                format!(
                    "framework_support_strip_schema_version must be {}, found {}",
                    FRAMEWORK_SUPPORT_STRIP_SCHEMA_VERSION,
                    self.framework_support_strip_schema_version
                ),
            ));
        }
        if self.actions.is_empty() {
            findings.push(Finding::new(
                "framework_support_strip.actions_not_empty",
                subject,
                "every support strip must declare at least one action",
            ));
        }

        let support = self.support_class;
        let pack_source = self.pack_or_bridge_source_block.pack_source_class;
        let family = self.framework_identity_block.framework_family_class;
        let version_compat = self.compatibility_block.version_compatibility_class;
        let health = self.health_block.health_class;
        let freshness = self.health_block.freshness_class;

        match support {
            SupportClass::CoreNative => {
                if !matches!(
                    pack_source,
                    PackSourceClass::FirstPartyNative | PackSourceClass::GovernedFrameworkPack
                ) {
                    findings.push(Finding::new(
                        "framework_support_strip.core_native_pack_source",
                        subject,
                        "core_native requires a first_party_native or governed_framework_pack source",
                    ));
                }
                if matches!(
                    version_compat,
                    VersionCompatibilityClass::UnsupportedVersion
                        | VersionCompatibilityClass::UnknownVersion
                ) {
                    findings.push(Finding::new(
                        "framework_support_strip.core_native_version_compat",
                        subject,
                        "core_native forbids unsupported_version or unknown_version compatibility",
                    ));
                }
            }
            SupportClass::FrameworkPack => {
                if !matches!(
                    pack_source,
                    PackSourceClass::FirstPartyNative
                        | PackSourceClass::GovernedFrameworkPack
                        | PackSourceClass::CommunityFrameworkPack
                ) {
                    findings.push(Finding::new(
                        "framework_support_strip.framework_pack_source",
                        subject,
                        "framework_pack requires a first_party_native, governed_framework_pack, or community_framework_pack source",
                    ));
                }
            }
            SupportClass::BridgeCompatibilityLayer => {
                if pack_source != PackSourceClass::BridgeCompatibilityLayer {
                    findings.push(Finding::new(
                        "framework_support_strip.bridge_pack_source",
                        subject,
                        "bridge_compatibility_layer requires pack_source_class = bridge_compatibility_layer",
                    ));
                }
            }
            SupportClass::HeuristicConventionMode => {
                if !matches!(
                    pack_source,
                    PackSourceClass::HeuristicConventionOnly
                        | PackSourceClass::ImportedSnapshotOnly
                        | PackSourceClass::NoPackOrBridge
                ) {
                    findings.push(Finding::new(
                        "framework_support_strip.heuristic_pack_source",
                        subject,
                        "heuristic_convention_mode requires a heuristic_convention_only, imported_snapshot_only, or no_pack_or_bridge source",
                    ));
                }
                if version_compat == VersionCompatibilityClass::WithinSupportedRange {
                    findings.push(Finding::new(
                        "framework_support_strip.heuristic_version_within_range",
                        subject,
                        "heuristic_convention_mode forbids within_supported_range compatibility",
                    ));
                }
            }
            SupportClass::UnsupportedOrUnclaimed => {
                if pack_source != PackSourceClass::NoPackOrBridge {
                    findings.push(Finding::new(
                        "framework_support_strip.unsupported_pack_source",
                        subject,
                        "unsupported_or_unclaimed requires pack_source_class = no_pack_or_bridge",
                    ));
                }
            }
        }

        if family == FrameworkFamilyClass::UnsupportedOrUnclaimedFamily
            && support.admits_exact_certainty()
        {
            findings.push(Finding::new(
                "framework_support_strip.unsupported_family_support_class",
                subject,
                "unsupported_or_unclaimed_family forbids core_native or framework_pack support",
            ));
        }

        if health == HealthClass::HealthyLive
            && matches!(freshness, FreshnessClass::Stale | FreshnessClass::Unverified)
        {
            findings.push(Finding::new(
                "framework_support_strip.healthy_live_freshness",
                subject,
                "healthy_live forbids stale or unverified freshness",
            ));
        }
        if health == HealthClass::UnreachableRuntime
            && freshness == FreshnessClass::AuthoritativeLive
        {
            findings.push(Finding::new(
                "framework_support_strip.unreachable_runtime_freshness",
                subject,
                "unreachable_runtime forbids authoritative_live freshness",
            ));
        }

        let needs_compat_action = matches!(
            support,
            SupportClass::CoreNative
                | SupportClass::FrameworkPack
                | SupportClass::BridgeCompatibilityLayer
                | SupportClass::HeuristicConventionMode
        );
        if needs_compat_action
            && !self
                .actions
                .contains(&FrameworkSupportActionClass::OpenCompatibilityDetails)
        {
            findings.push(Finding::new(
                "framework_support_strip.compat_action_required",
                subject,
                "every non-unsupported strip must offer open_compatibility_details",
            ));
        }

        if !pack_source.admits_pack_actions() {
            if self
                .actions
                .contains(&FrameworkSupportActionClass::RequestPackUpdate)
            {
                findings.push(Finding::new(
                    "framework_support_strip.pack_update_without_pack",
                    subject,
                    "request_pack_update is only admissible when a pack or bridge source is bound",
                ));
            }
            if self
                .actions
                .contains(&FrameworkSupportActionClass::OpenPackStatus)
            {
                findings.push(Finding::new(
                    "framework_support_strip.open_pack_status_without_pack",
                    subject,
                    "open_pack_status is only admissible when a pack or bridge source is bound",
                ));
            }
        }

        findings
    }
}
