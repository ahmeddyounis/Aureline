//! Implements the reusable live-resource navigation primitive: a resource-link
//! row, a rendered / live compare card, a resource-explorer row, and a drift /
//! unavailable banner that all resolve from one live-resource context and share
//! one resource identity, so source-to-live navigation and live-resource
//! browsing stay honest *before* users act under drift, partiality, and
//! permission limits.
//!
//! Where
//! [`crate::freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix`]
//! *freezes* the reusable manifest / build-confidence component families as a
//! governed contract, this module *narrows* two of those families —
//! [`crate::M5ManifestBuildComponentFamily::ResourceLinkRow`] and
//! [`crate::M5ManifestBuildComponentFamily::ResourceExplorerRow`] — plus the
//! rendered / live compare card and the drift / unavailable banner they imply
//! into one working primitive with a real **resolver**. A single live-resource
//! context projects onto four surfaces that share one resource identity and one
//! disclosed truth class, so authored / rendered / planned / live / cached /
//! provider-overlay relationships never blur across the link row, the compare
//! card, the explorer row, and the drift banner.
//!
//! The three acceptance criteria the resolver proves:
//!
//! - **AC1 — source config and live / cached resources never blur.** The link
//!   row keeps its two truth-class sides distinct, and the compare card, explorer
//!   row, and drift banner all disclose one shared truth class, so a user can
//!   move between source and live truth without the two collapsing into one.
//! - **AC2 — drift and unavailability are visible before users act.** The drift
//!   banner is present whenever a resource has drifted, gone unavailable, gone
//!   stale, or lost permission, and it names exactly what diverged and what
//!   remains safe to inspect *before* any logs / events / open-detail action is
//!   taken.
//! - **AC3 — partial or permission-limited data is never shown as fully
//!   current.** A resource reads as current only when it is live-fresh, fully
//!   reachable, undrifted, and high / medium confidence; any cached, imported,
//!   drifted, permission-limited, or low-confidence resource is disclosed as such
//!   rather than presented as live truth.
//!
//! Raw resource bodies, credentials, connector tokens, and endpoint data never
//! cross this boundary; the resolver carries only opaque refs, typed class
//! tokens, booleans, and redacted labels, so support and diagnostics exports
//! reconstruct exactly what a surface would have shown without leaking source or
//! live payloads.
//!
//! The boundary schema is
//! [`schemas/ui/m5-live-resource-navigation-primitive.schema.json`](../../../../schemas/ui/m5-live-resource-navigation-primitive.schema.json).
//! The contract doc is
//! [`docs/infra/m5_live_resource_navigation_primitive.md`](../../../../docs/infra/m5_live_resource_navigation_primitive.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    truth_mode_token, DegradedState, M5DiscoveryConfidence, M5ManifestBuildDowngradeTrigger,
    M5ResourceFreshness, M5ResourceLinkClass, TruthMode,
};

/// Stable record-kind tag carried by [`M5LiveResourcePrimitivePacket`].
pub const M5_LIVE_RESOURCE_RECORD_KIND: &str = "m5_live_resource_navigation_primitive";

/// Schema version for the live-resource navigation primitive packet.
pub const M5_LIVE_RESOURCE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_LIVE_RESOURCE_SCHEMA_REF: &str =
    "schemas/ui/m5-live-resource-navigation-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LIVE_RESOURCE_DOC_REF: &str = "docs/infra/m5_live_resource_navigation_primitive.md";

/// Repo-relative path of the frozen component-matrix contract this primitive
/// narrows.
pub const M5_LIVE_RESOURCE_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-manifest-build-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_LIVE_RESOURCE_FIXTURE_DIR: &str = "fixtures/ui/m5-live-resource-navigation-primitive";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const M5_LIVE_RESOURCE_ARTIFACT_REF: &str =
    "artifacts/release/m5-live-resource-navigation-primitive-proof/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_LIVE_RESOURCE_CSV_REF: &str =
    "artifacts/release/m5-live-resource-navigation-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_LIVE_RESOURCE_REPORT_REF: &str =
    "artifacts/release/m5-live-resource-navigation-primitive-proof/report.md";

// --- minted controlled vocabulary ---

/// Closed live-resource navigation surface family. Each family is one parity
/// surface that ingests the shared primitive; the matrix must define every one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LiveResourceSurfaceFamily {
    /// The source-to-live navigator that joins authored / rendered / live truth.
    SourceToLiveNavigator,
    /// The rendered / live compare card.
    RenderedLiveCompare,
    /// The cluster / resource explorer that browses live truth.
    ClusterResourceExplorer,
    /// The drift / unavailable banner gating action on a live surface.
    DriftUnavailableBanner,
    /// The provider-console handoff surface.
    ProviderConsoleHandoff,
    /// The support / export replay surface that reconstructs navigation truth.
    SupportExportReplay,
}

impl M5LiveResourceSurfaceFamily {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SourceToLiveNavigator,
        Self::RenderedLiveCompare,
        Self::ClusterResourceExplorer,
        Self::DriftUnavailableBanner,
        Self::ProviderConsoleHandoff,
        Self::SupportExportReplay,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceToLiveNavigator => "source_to_live_navigator",
            Self::RenderedLiveCompare => "rendered_live_compare",
            Self::ClusterResourceExplorer => "cluster_resource_explorer",
            Self::DriftUnavailableBanner => "drift_unavailable_banner",
            Self::ProviderConsoleHandoff => "provider_console_handoff",
            Self::SupportExportReplay => "support_export_replay",
        }
    }

    /// Human-readable label for the Markdown report.
    pub const fn label(self) -> &'static str {
        match self {
            Self::SourceToLiveNavigator => "Source-to-live navigator",
            Self::RenderedLiveCompare => "Rendered / live compare card",
            Self::ClusterResourceExplorer => "Cluster / resource explorer",
            Self::DriftUnavailableBanner => "Drift / unavailable banner",
            Self::ProviderConsoleHandoff => "Provider-console handoff",
            Self::SupportExportReplay => "Support / export replay",
        }
    }
}

/// Closed resource-kind vocabulary. Names what kind of resource an explorer row
/// is so a workload, a network object, and a config object never read as one
/// another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResourceKind {
    /// A workload (deployment, statefulset, job, pod).
    Workload,
    /// A network object (service, ingress, route).
    Network,
    /// A config object (configmap, secret ref, settings).
    Config,
    /// A storage object (volume, claim, bucket).
    Storage,
    /// An identity / access object (role, binding, account).
    Identity,
    /// A custom / provider-defined resource.
    CustomResource,
}

impl M5ResourceKind {
    /// Every resource kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Workload,
        Self::Network,
        Self::Config,
        Self::Storage,
        Self::Identity,
        Self::CustomResource,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Workload => "workload",
            Self::Network => "network",
            Self::Config => "config",
            Self::Storage => "storage",
            Self::Identity => "identity",
            Self::CustomResource => "custom_resource",
        }
    }
}

/// Closed resource-health vocabulary. Names the health an explorer row discloses
/// so an unknown or unavailable resource never reads as healthy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResourceHealth {
    /// Healthy and reconciled.
    Healthy,
    /// Degraded relative to desired state.
    Degraded,
    /// Progressing toward desired state.
    Progressing,
    /// Unavailable / not observed.
    Unavailable,
    /// Health not yet established.
    Unknown,
}

impl M5ResourceHealth {
    /// Every health state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Healthy,
        Self::Degraded,
        Self::Progressing,
        Self::Unavailable,
        Self::Unknown,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Degraded => "degraded",
            Self::Progressing => "progressing",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }
}

/// Closed permission / connection posture vocabulary. Names how reachable a
/// resource is so permission-limited or disconnected data never reads as fully
/// current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PermissionPosture {
    /// Full read / act access.
    FullAccess,
    /// Read-only access.
    ReadOnly,
    /// Permission-limited: some data is withheld by policy.
    PermissionLimited,
    /// The live connector was lost mid-session.
    ConnectionLost,
    /// Offline: no live connector at all.
    Offline,
}

impl M5PermissionPosture {
    /// Every permission posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullAccess,
        Self::ReadOnly,
        Self::PermissionLimited,
        Self::ConnectionLost,
        Self::Offline,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullAccess => "full_access",
            Self::ReadOnly => "read_only",
            Self::PermissionLimited => "permission_limited",
            Self::ConnectionLost => "connection_lost",
            Self::Offline => "offline",
        }
    }

    /// True when the posture grants complete, unrestricted read access.
    pub const fn is_full_or_read(self) -> bool {
        matches!(self, Self::FullAccess | Self::ReadOnly)
    }

    /// True when the posture is limited: some data is withheld or unreachable.
    pub const fn is_limited(self) -> bool {
        matches!(
            self,
            Self::PermissionLimited | Self::ConnectionLost | Self::Offline
        )
    }

    /// True when the live target is unreachable (connector lost or offline).
    pub const fn is_disconnected(self) -> bool {
        matches!(self, Self::ConnectionLost | Self::Offline)
    }
}

/// Closed rendered / live compare-verdict vocabulary. Names how a rendered
/// resource compares to its live counterpart so drift and absence are explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompareVerdict {
    /// Rendered and live agree.
    InSync,
    /// Rendered and live diverge.
    Drifted,
    /// Rendered exists with no live counterpart (not yet applied).
    RenderedOnlyNoLive,
    /// Live exists with no source counterpart (unmanaged / orphan).
    LiveOnlyUnmanaged,
    /// The provider console holds authoritative state; local cannot compare.
    OverlayAuthoritative,
    /// The comparison could not be performed (connector / permission).
    ComparisonUnavailable,
}

impl M5CompareVerdict {
    /// Every compare verdict, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InSync,
        Self::Drifted,
        Self::RenderedOnlyNoLive,
        Self::LiveOnlyUnmanaged,
        Self::OverlayAuthoritative,
        Self::ComparisonUnavailable,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSync => "in_sync",
            Self::Drifted => "drifted",
            Self::RenderedOnlyNoLive => "rendered_only_no_live",
            Self::LiveOnlyUnmanaged => "live_only_unmanaged",
            Self::OverlayAuthoritative => "overlay_authoritative",
            Self::ComparisonUnavailable => "comparison_unavailable",
        }
    }

    /// True when the two sides are known to diverge.
    pub const fn is_drift(self) -> bool {
        matches!(self, Self::Drifted)
    }

    /// True when the comparison itself could not be performed.
    pub const fn is_comparison_unavailable(self) -> bool {
        matches!(self, Self::ComparisonUnavailable)
    }
}

/// Closed resource-action vocabulary. Names the safe, read-only actions a
/// navigation surface offers so inspection stays available even under drift.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ResourceActionKind {
    /// Open the resource detail view.
    OpenDetail,
    /// View the resource logs.
    ViewLogs,
    /// View the resource events.
    ViewEvents,
    /// Inspect the rendered / live diff.
    InspectDiff,
    /// Open the resource in the provider console.
    OpenInProviderConsole,
}

impl M5ResourceActionKind {
    /// Every action kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OpenDetail,
        Self::ViewLogs,
        Self::ViewEvents,
        Self::InspectDiff,
        Self::OpenInProviderConsole,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenDetail => "open_detail",
            Self::ViewLogs => "view_logs",
            Self::ViewEvents => "view_events",
            Self::InspectDiff => "inspect_diff",
            Self::OpenInProviderConsole => "open_in_provider_console",
        }
    }
}

/// Closed export-field vocabulary. Names the fields the support / export packet
/// must carry per surface; the mandatory subset must appear on every row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LiveResourceExportField {
    /// The stable resource identity shared across surfaces.
    ResourceId,
    /// The typed resource identity (kind, stable id, namespace, project).
    ResourceIdentity,
    /// The target-identity ref the surface acts against.
    TargetIdentity,
    /// The authored / rendered / planned / live / provider-overlay truth class.
    TruthClass,
    /// The freshness disclosed on the explorer row and drift banner.
    Freshness,
    /// The discovery confidence.
    Confidence,
    /// The permission / connection posture.
    PermissionPosture,
}

impl M5LiveResourceExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ResourceId,
        Self::ResourceIdentity,
        Self::TargetIdentity,
        Self::TruthClass,
        Self::Freshness,
        Self::Confidence,
        Self::PermissionPosture,
    ];

    /// The mandatory subset every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ResourceId,
        Self::ResourceIdentity,
        Self::TargetIdentity,
        Self::TruthClass,
        Self::Freshness,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResourceId => "resource_id",
            Self::ResourceIdentity => "resource_identity",
            Self::TargetIdentity => "target_identity",
            Self::TruthClass => "truth_class",
            Self::Freshness => "freshness",
            Self::Confidence => "confidence",
            Self::PermissionPosture => "permission_posture",
        }
    }
}

// --- shared value structs ---

/// The typed, stable identity of a resource. Every slot is opaque; raw endpoint
/// data never crosses this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResourceIdentity {
    /// The kind of resource.
    pub resource_kind: M5ResourceKind,
    /// The stable resource id (opaque; never a raw endpoint).
    pub stable_id: String,
    /// The namespace reference, when scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    /// The project reference, when scoped.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
}

impl M5ResourceIdentity {
    /// True when the identity carries a stable, non-empty id.
    pub fn is_stable(&self) -> bool {
        !self.stable_id.trim().is_empty()
    }
}

// --- resolver input ---

/// The full input to the live-resource navigation resolver for one resource
/// context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveResourceInput {
    /// The stable resource identity that must survive across the link row,
    /// compare card, explorer row, and drift banner.
    pub resource_id: String,
    /// Opaque ref to the resource object; never raw resource bytes.
    pub resource_ref: String,
    /// Human-readable resource label.
    pub resource_label: String,
    /// The typed resource identity (kind, stable id, namespace, project).
    pub identity: M5ResourceIdentity,
    /// Which two truth classes the source-to-live link joins.
    pub link_class: M5ResourceLinkClass,
    /// The truth class on the "from" (source) side of the link.
    pub from_truth: TruthMode,
    /// The truth class on the "to" (target) side of the link.
    pub to_truth: TruthMode,
    /// The truth class the resource itself is shown in on the explorer / compare /
    /// banner surfaces.
    pub truth_mode: TruthMode,
    /// How fresh the resource data is.
    pub freshness: M5ResourceFreshness,
    /// The confidence of the discovered resource / link.
    pub confidence: M5DiscoveryConfidence,
    /// The permission / connection posture.
    pub permission: M5PermissionPosture,
    /// The health the explorer row discloses.
    pub health: M5ResourceHealth,
    /// How the rendered resource compares to its live counterpart.
    pub compare_verdict: M5CompareVerdict,
    /// Opaque ref to the target identity the surface acts against; never raw
    /// endpoint data.
    pub target_identity_ref: String,
    /// A precise permission / connection note, required when access is limited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission_connection_note: Option<String>,
    /// A precise note on exactly what diverged, required when the verdict is
    /// [`M5CompareVerdict::Drifted`].
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub divergence_note: Option<String>,
    /// The safe, read-only actions offered (logs / events / open detail / inspect
    /// diff / open in provider console).
    pub available_actions: Vec<M5ResourceActionKind>,
    /// An externally-observed narrowing (drift, connector loss, policy block) that
    /// degrades the surface before action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded: Option<DegradedState>,
}

// --- resolved projections ---

/// The resolved source-to-live resource-link row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedResourceLinkRow {
    /// The resource identity — identical to the compare card, explorer row, and
    /// banner.
    pub resource_id: String,
    /// Which two truth classes this link joins.
    pub link_class: M5ResourceLinkClass,
    /// The truth class on the "from" (source) side.
    pub from_truth: TruthMode,
    /// The truth class on the "to" (target) side.
    pub to_truth: TruthMode,
    /// The confidence of the discovered link.
    pub confidence: M5DiscoveryConfidence,
    /// The permission / connection posture.
    pub permission: M5PermissionPosture,
    /// The link never overwrites a higher-confidence resource silently; always
    /// holds.
    pub never_overwrites_higher_confidence: bool,
    /// The source side is always inspectable.
    pub from_side_navigable: bool,
    /// The live / target side is navigable only when reachable.
    pub to_side_navigable: bool,
}

/// The resolved rendered / live compare card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCompareCard {
    /// The resource identity — identical to every other surface.
    pub resource_id: String,
    /// The truth class the resource is shown in.
    pub truth_mode: TruthMode,
    /// How the rendered resource compares to its live counterpart.
    pub compare_verdict: M5CompareVerdict,
    /// The truth class on the rendered / source side.
    pub rendered_side_truth: TruthMode,
    /// The truth class on the live / target side.
    pub live_side_truth: TruthMode,
    /// Exactly what diverged, when the verdict is drift.
    pub what_diverged: Option<String>,
    /// The comparison reflects current live truth (only when in-sync and
    /// live-fresh).
    pub comparison_current: bool,
    /// Inspecting the card is always safe (read-only); always holds.
    pub safe_to_inspect: bool,
}

/// The resolved resource-explorer row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedResourceExplorerRow {
    /// The resource identity — identical to every other surface.
    pub resource_id: String,
    /// The typed resource identity (kind, stable id, namespace, project).
    pub identity: M5ResourceIdentity,
    /// The truth class the resource is shown in.
    pub truth_mode: TruthMode,
    /// How fresh the resource data is.
    pub freshness: M5ResourceFreshness,
    /// The discovery confidence.
    pub confidence: M5DiscoveryConfidence,
    /// The health the row discloses.
    pub health: M5ResourceHealth,
    /// The permission / connection posture.
    pub permission: M5PermissionPosture,
    /// The precise permission / connection note, when access is limited.
    pub permission_connection_note: Option<String>,
    /// The safe, read-only actions offered.
    pub actions: Vec<M5ResourceActionKind>,
    /// Target context is always visible on the explorer row; always holds.
    pub target_context_visible: bool,
    /// The row presents its data as fully current only when it is live-fresh,
    /// reachable, undrifted, and confident.
    pub presents_as_current: bool,
}

/// The resolved drift / unavailable banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDriftBanner {
    /// The resource identity — identical to every other surface.
    pub resource_id: String,
    /// The truth class the banner discloses.
    pub truth_mode: TruthMode,
    /// The banner is present whenever there is drift, unavailability, staleness,
    /// or limited access to disclose.
    pub banner_present: bool,
    /// The rendered and live sides are known to diverge.
    pub drift_present: bool,
    /// The live target is unavailable (connector lost, offline, or comparison
    /// unavailable).
    pub unavailable: bool,
    /// Exactly what diverged, when the verdict is drift.
    pub what_diverged: Option<String>,
    /// The data is stale (cached or imported), not live.
    pub what_stale: bool,
    /// Inspecting the resource remains safe (read-only); always holds.
    pub safe_to_inspect: bool,
    /// Why the surface is narrowed, when it is; names a real, reconstructable
    /// trigger.
    pub banner_reason: Option<M5ManifestBuildDowngradeTrigger>,
}

/// The resolved live-resource navigation truth shared across the link row,
/// compare card, explorer row, and drift banner.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLiveResource {
    /// The stable resource identity.
    pub resource_id: String,
    /// The resolved source-to-live resource-link row.
    pub link_row: M5ResolvedResourceLinkRow,
    /// The resolved rendered / live compare card.
    pub compare_card: M5ResolvedCompareCard,
    /// The resolved resource-explorer row.
    pub explorer_row: M5ResolvedResourceExplorerRow,
    /// The resolved drift / unavailable banner.
    pub drift_banner: M5ResolvedDriftBanner,
    /// The two sides of the source-to-live link stay distinct truth classes
    /// (AC1); always holds.
    pub truth_classes_distinct: bool,
    /// Drift and unavailability are disclosed before action (AC2); always holds.
    pub drift_and_unavailability_disclosed: bool,
    /// The narrowing carried through from the input, when present.
    pub degraded: Option<DegradedState>,
}

impl M5ResolvedLiveResource {
    /// True when the resource identity is identical across the link row, compare
    /// card, explorer row, and drift banner.
    pub fn identity_consistent(&self) -> bool {
        self.link_row.resource_id == self.resource_id
            && self.compare_card.resource_id == self.resource_id
            && self.explorer_row.resource_id == self.resource_id
            && self.drift_banner.resource_id == self.resource_id
    }

    /// True when the compare card, explorer row, and drift banner all disclose the
    /// same truth class — authored / rendered / live / cached never blurs across
    /// them.
    pub fn truth_class_disclosed_consistently(&self) -> bool {
        self.compare_card.truth_mode == self.explorer_row.truth_mode
            && self.compare_card.truth_mode == self.drift_banner.truth_mode
    }

    /// True when the source-to-live link keeps its two sides distinct — a user can
    /// move between source and live without the two collapsing (AC1).
    pub fn source_and_live_distinct(&self) -> bool {
        self.truth_classes_distinct && self.link_row.from_truth != self.link_row.to_truth
    }

    /// True when any drift or unavailability is surfaced by a present banner
    /// *before* the user acts (AC2).
    pub fn drift_visible_before_action(&self) -> bool {
        !(self.drift_banner.drift_present || self.drift_banner.unavailable)
            || self.drift_banner.banner_present
    }

    /// True when the explorer row presents its data as current only when it is
    /// genuinely live, reachable, undrifted, and confident (AC3).
    pub fn no_partial_shown_as_current(&self) -> bool {
        !self.explorer_row.presents_as_current
            || (self.explorer_row.freshness.is_live_fresh()
                && self.explorer_row.permission.is_full_or_read()
                && !self.drift_banner.drift_present
                && !self.drift_banner.unavailable
                && confidence_is_certain(self.explorer_row.confidence))
    }
}

/// Errors returned by [`resolve_live_resource_navigation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5LiveResourceResolutionError {
    /// The resource id was empty.
    EmptyResourceId,
    /// The resource ref was empty.
    EmptyResourceRef,
    /// The resource label was empty.
    EmptyResourceLabel,
    /// The resource identity carried no stable id.
    EmptyResourceIdentity,
    /// The target-identity ref was empty.
    EmptyTargetIdentityRef,
    /// A label, ref, or note carried forbidden material.
    ForbiddenMaterial,
    /// The source-to-live link collapsed its two sides into one truth class.
    BlurredLinkTruthClasses,
    /// A live-fresh resource claimed a non-live truth class.
    LiveFreshTruthMismatch,
    /// Access was limited but no precise permission / connection note was given.
    PermissionLimitedWithoutNote,
    /// The verdict was drift but no precise divergence note was given.
    DriftWithoutDivergenceDetail,
    /// No safe, read-only action was offered on the surface.
    NoActionsOffered,
    /// A degraded block carried a generic non-answer label.
    DegradedLabelGeneric,
}

impl M5LiveResourceResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyResourceId => "empty_resource_id",
            Self::EmptyResourceRef => "empty_resource_ref",
            Self::EmptyResourceLabel => "empty_resource_label",
            Self::EmptyResourceIdentity => "empty_resource_identity",
            Self::EmptyTargetIdentityRef => "empty_target_identity_ref",
            Self::ForbiddenMaterial => "forbidden_material",
            Self::BlurredLinkTruthClasses => "blurred_link_truth_classes",
            Self::LiveFreshTruthMismatch => "live_fresh_truth_mismatch",
            Self::PermissionLimitedWithoutNote => "permission_limited_without_note",
            Self::DriftWithoutDivergenceDetail => "drift_without_divergence_detail",
            Self::NoActionsOffered => "no_actions_offered",
            Self::DegradedLabelGeneric => "degraded_label_generic",
        }
    }
}

impl fmt::Display for M5LiveResourceResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "live-resource navigation resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5LiveResourceResolutionError {}

/// Resolves one live-resource navigation context into its shared source-to-live
/// link row, rendered / live compare card, resource-explorer row, and drift /
/// unavailable banner.
///
/// The four surfaces share one resource identity and one disclosed truth class,
/// so source config and live / cached truth never blur. Drift and
/// unavailability are surfaced by a present banner before the user acts, and a
/// resource reads as current only when it is genuinely live, reachable,
/// undrifted, and confident — partial or permission-limited data is disclosed as
/// such rather than presented as fully current.
pub fn resolve_live_resource_navigation(
    input: &M5LiveResourceInput,
) -> Result<M5ResolvedLiveResource, M5LiveResourceResolutionError> {
    if input.resource_id.trim().is_empty() {
        return Err(M5LiveResourceResolutionError::EmptyResourceId);
    }
    if input.resource_ref.trim().is_empty() {
        return Err(M5LiveResourceResolutionError::EmptyResourceRef);
    }
    if input.resource_label.trim().is_empty() {
        return Err(M5LiveResourceResolutionError::EmptyResourceLabel);
    }
    if !input.identity.is_stable() {
        return Err(M5LiveResourceResolutionError::EmptyResourceIdentity);
    }
    if input.target_identity_ref.trim().is_empty() {
        return Err(M5LiveResourceResolutionError::EmptyTargetIdentityRef);
    }

    for value in [
        input.resource_ref.as_str(),
        input.resource_label.as_str(),
        input.identity.stable_id.as_str(),
        input.target_identity_ref.as_str(),
    ]
    .into_iter()
    .chain(input.identity.namespace.as_deref())
    .chain(input.identity.project.as_deref())
    .chain(input.permission_connection_note.as_deref())
    .chain(input.divergence_note.as_deref())
    {
        if value_is_forbidden(value) {
            return Err(M5LiveResourceResolutionError::ForbiddenMaterial);
        }
    }

    // AC1: a source-to-live link may never collapse its two sides into one truth
    // class.
    if input.from_truth == input.to_truth {
        return Err(M5LiveResourceResolutionError::BlurredLinkTruthClasses);
    }

    // AC3: live-fresh data must be presented as live truth, never as authored,
    // rendered, or planned truth.
    if input.freshness.is_live_fresh() && input.truth_mode != TruthMode::Live {
        return Err(M5LiveResourceResolutionError::LiveFreshTruthMismatch);
    }

    // AC3: limited access must always carry a precise note so partiality is never
    // silent.
    if input.permission.is_limited() && input.permission_connection_note.is_none() {
        return Err(M5LiveResourceResolutionError::PermissionLimitedWithoutNote);
    }

    // AC2: drift must name exactly what diverged.
    if input.compare_verdict.is_drift() && input.divergence_note.is_none() {
        return Err(M5LiveResourceResolutionError::DriftWithoutDivergenceDetail);
    }

    if input.available_actions.is_empty() {
        return Err(M5LiveResourceResolutionError::NoActionsOffered);
    }

    if let Some(degraded) = &input.degraded {
        if !degraded.is_honest() {
            return Err(M5LiveResourceResolutionError::DegradedLabelGeneric);
        }
    }

    let drift_present = input.compare_verdict.is_drift();
    let unavailable =
        input.compare_verdict.is_comparison_unavailable() || input.permission.is_disconnected();
    let what_stale = matches!(
        input.freshness,
        M5ResourceFreshness::CachedStale | M5ResourceFreshness::ImportedSnapshot
    );
    let confident = confidence_is_certain(input.confidence);

    let what_diverged = if drift_present {
        input.divergence_note.clone()
    } else {
        None
    };

    // AC3: a resource reads as current only when everything about it is current.
    let presents_as_current = input.freshness.is_live_fresh()
        && input.permission.is_full_or_read()
        && !drift_present
        && !unavailable
        && confident;

    // AC2: the banner is present whenever there is anything to disclose about
    // drift, unavailability, staleness, limited access, or low confidence.
    let banner_present = drift_present
        || unavailable
        || what_stale
        || input.permission.is_limited()
        || !confident
        || matches!(
            input.compare_verdict,
            M5CompareVerdict::OverlayAuthoritative
                | M5CompareVerdict::RenderedOnlyNoLive
                | M5CompareVerdict::LiveOnlyUnmanaged
        );

    let banner_reason = if let Some(degraded) = &input.degraded {
        Some(degraded.trigger)
    } else if drift_present {
        Some(M5ManifestBuildDowngradeTrigger::DriftFromSource)
    } else if input.permission.is_disconnected()
        || input.compare_verdict.is_comparison_unavailable()
    {
        Some(M5ManifestBuildDowngradeTrigger::ConnectorLoss)
    } else if matches!(input.permission, M5PermissionPosture::PermissionLimited) {
        Some(M5ManifestBuildDowngradeTrigger::PolicyBlock)
    } else if !confident {
        Some(M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery)
    } else {
        None
    };

    let comparison_current = matches!(input.compare_verdict, M5CompareVerdict::InSync)
        && input.freshness.is_live_fresh();

    let to_side_navigable =
        !input.permission.is_disconnected() && !input.compare_verdict.is_comparison_unavailable();

    let link_row = M5ResolvedResourceLinkRow {
        resource_id: input.resource_id.clone(),
        link_class: input.link_class,
        from_truth: input.from_truth,
        to_truth: input.to_truth,
        confidence: input.confidence,
        permission: input.permission,
        never_overwrites_higher_confidence: true,
        from_side_navigable: true,
        to_side_navigable,
    };

    let compare_card = M5ResolvedCompareCard {
        resource_id: input.resource_id.clone(),
        truth_mode: input.truth_mode,
        compare_verdict: input.compare_verdict,
        rendered_side_truth: input.from_truth,
        live_side_truth: input.to_truth,
        what_diverged: what_diverged.clone(),
        comparison_current,
        safe_to_inspect: true,
    };

    let explorer_row = M5ResolvedResourceExplorerRow {
        resource_id: input.resource_id.clone(),
        identity: input.identity.clone(),
        truth_mode: input.truth_mode,
        freshness: input.freshness,
        confidence: input.confidence,
        health: input.health,
        permission: input.permission,
        permission_connection_note: input.permission_connection_note.clone(),
        actions: input.available_actions.clone(),
        target_context_visible: true,
        presents_as_current,
    };

    let drift_banner = M5ResolvedDriftBanner {
        resource_id: input.resource_id.clone(),
        truth_mode: input.truth_mode,
        banner_present,
        drift_present,
        unavailable,
        what_diverged,
        what_stale,
        safe_to_inspect: true,
        banner_reason,
    };

    Ok(M5ResolvedLiveResource {
        resource_id: input.resource_id.clone(),
        link_row,
        compare_card,
        explorer_row,
        drift_banner,
        truth_classes_distinct: true,
        drift_and_unavailability_disclosed: true,
        degraded: input.degraded.clone(),
    })
}

/// True when a discovery confidence is high or medium — certain enough to treat
/// data as current.
const fn confidence_is_certain(confidence: M5DiscoveryConfidence) -> bool {
    matches!(
        confidence,
        M5DiscoveryConfidence::High | M5DiscoveryConfidence::Medium
    )
}

/// True when a label, ref, or note carries obviously forbidden material.
fn value_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// One worked resolution case carried in the packet so the support / export
/// packet reconstructs navigation truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveResourceCase {
    /// The resolver input.
    pub input: M5LiveResourceInput,
    /// The resolved navigation truth. Must equal
    /// `resolve_live_resource_navigation(&input)`.
    pub resolved: M5ResolvedLiveResource,
}

impl M5LiveResourceCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5LiveResourceInput) -> Self {
        let resolved =
            resolve_live_resource_navigation(&input).expect("seed navigation case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_live_resource_navigation(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one navigation surface family bound to the
/// shared live-resource contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveResourceSurfaceRow {
    /// The navigation surface family.
    pub surface_family: M5LiveResourceSurfaceFamily,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Resource kinds this surface can disclose (must be non-empty).
    pub resource_kinds: Vec<M5ResourceKind>,
    /// Truth classes this surface renders (must be non-empty).
    pub truth_modes: Vec<TruthMode>,
    /// Resource-link classes this surface joins (must be non-empty).
    pub link_classes: Vec<M5ResourceLinkClass>,
    /// Export fields this row carries (must include the mandatory fields).
    pub export_fields: Vec<M5LiveResourceExportField>,
    /// Downgrade triggers that apply to this surface (must be non-empty).
    pub downgrade_triggers: Vec<M5ManifestBuildDowngradeTrigger>,
    /// Consumer surfaces that ingest this row's projection (must be non-empty).
    pub consumer_surfaces: Vec<String>,
    /// Source contract refs consumed by this row (must be non-empty).
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this surface (must be
    /// non-empty).
    pub example_navigation: Vec<M5LiveResourceCase>,
    /// Hard invariant: this row never hides the disclosed truth class. MUST be
    /// `false`.
    pub hides_truth_class: bool,
    /// Hard invariant: this row never blurs source and live truth. MUST be
    /// `false`.
    pub blurs_source_and_live: bool,
    /// Hard invariant: this row never hides drift or unavailability. MUST be
    /// `false`.
    pub hides_drift_or_unavailability: bool,
    /// Hard invariant: this row never presents partial / limited data as current.
    /// MUST be `false`.
    pub presents_partial_as_current: bool,
}

impl M5LiveResourceSurfaceRow {
    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5LiveResourceExportField> =
            self.export_fields.iter().copied().collect();
        M5LiveResourceExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.hides_truth_class
            && !self.blurs_source_and_live
            && !self.hides_drift_or_unavailability
            && !self.presents_partial_as_current
    }
}

/// Self-describing controlled-vocabulary set minted / reused by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveResourceVocabularySet {
    /// Navigation surface-family tokens.
    pub surface_families: Vec<String>,
    /// Resource-kind tokens.
    pub resource_kinds: Vec<String>,
    /// Resource-health tokens.
    pub health_states: Vec<String>,
    /// Permission-posture tokens.
    pub permission_postures: Vec<String>,
    /// Compare-verdict tokens.
    pub compare_verdicts: Vec<String>,
    /// Resource-action tokens.
    pub action_kinds: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Truth-class tokens (reused from the frozen matrix).
    pub truth_modes: Vec<String>,
    /// Resource-link-class tokens (reused from the frozen matrix).
    pub resource_link_classes: Vec<String>,
    /// Resource-freshness tokens (reused from the frozen matrix).
    pub resource_freshness: Vec<String>,
    /// Discovery-confidence tokens (reused from the frozen matrix).
    pub discovery_confidence: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5LiveResourceVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5LiveResourceSurfaceFamily::ALL, |v| v.as_str()),
            resource_kinds: tokens(&M5ResourceKind::ALL, |v| v.as_str()),
            health_states: tokens(&M5ResourceHealth::ALL, |v| v.as_str()),
            permission_postures: tokens(&M5PermissionPosture::ALL, |v| v.as_str()),
            compare_verdicts: tokens(&M5CompareVerdict::ALL, |v| v.as_str()),
            action_kinds: tokens(&M5ResourceActionKind::ALL, |v| v.as_str()),
            export_fields: tokens(&M5LiveResourceExportField::ALL, |v| v.as_str()),
            truth_modes: tokens(&TRUTH_MODE_ALL, truth_mode_token),
            resource_link_classes: tokens(&RESOURCE_LINK_CLASS_ALL, |v| v.as_str()),
            resource_freshness: tokens(&RESOURCE_FRESHNESS_ALL, |v| v.as_str()),
            discovery_confidence: tokens(&DISCOVERY_CONFIDENCE_ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&DOWNGRADE_TRIGGER_ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// The truth classes reused from the frozen matrix, in a stable order.
/// [`TruthMode`] is a pure token set, so the order is pinned here.
const TRUTH_MODE_ALL: [TruthMode; 5] = [
    TruthMode::Desired,
    TruthMode::Rendered,
    TruthMode::Plan,
    TruthMode::Live,
    TruthMode::ProviderOverlay,
];

/// The resource-link classes reused from the frozen matrix, in a stable order.
const RESOURCE_LINK_CLASS_ALL: [M5ResourceLinkClass; 5] = [
    M5ResourceLinkClass::AuthoredToRendered,
    M5ResourceLinkClass::RenderedToLive,
    M5ResourceLinkClass::PlanToLive,
    M5ResourceLinkClass::SchemaBacked,
    M5ResourceLinkClass::CrossTarget,
];

/// The resource-freshness states reused from the frozen matrix, in a stable
/// order.
const RESOURCE_FRESHNESS_ALL: [M5ResourceFreshness; 5] = [
    M5ResourceFreshness::LiveFresh,
    M5ResourceFreshness::CachedStale,
    M5ResourceFreshness::ImportedSnapshot,
    M5ResourceFreshness::PlanOnly,
    M5ResourceFreshness::Unknown,
];

/// The discovery-confidence states reused from the frozen matrix, in a stable
/// order.
const DISCOVERY_CONFIDENCE_ALL: [M5DiscoveryConfidence; 4] = [
    M5DiscoveryConfidence::High,
    M5DiscoveryConfidence::Medium,
    M5DiscoveryConfidence::Low,
    M5DiscoveryConfidence::Unknown,
];

/// The downgrade triggers reused from the frozen matrix, in a stable order.
const DOWNGRADE_TRIGGER_ALL: [M5ManifestBuildDowngradeTrigger; 8] = [
    M5ManifestBuildDowngradeTrigger::SchemaStale,
    M5ManifestBuildDowngradeTrigger::AdapterUnavailable,
    M5ManifestBuildDowngradeTrigger::ConnectorLoss,
    M5ManifestBuildDowngradeTrigger::PolicyBlock,
    M5ManifestBuildDowngradeTrigger::DriftFromSource,
    M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
    M5ManifestBuildDowngradeTrigger::StructuredChannelLost,
    M5ManifestBuildDowngradeTrigger::TargetContextUnresolved,
];

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveResourceGovernanceReview {
    /// One primitive carries link-row / compare-card / explorer-row / drift-banner
    /// truth on every surface.
    pub one_primitive_carries_all_surfaces: bool,
    /// Resource identity is preserved across the link row, compare card, explorer
    /// row, and drift banner.
    pub resource_identity_preserved_across_surfaces: bool,
    /// Source config and live / cached truth are never blurred.
    pub source_and_live_never_blurred: bool,
    /// Drift and unavailability are visible before action.
    pub drift_and_unavailability_visible_before_action: bool,
    /// Partial or permission-limited data is never shown as fully current.
    pub partial_or_limited_never_shown_as_current: bool,
    /// The support / export packet reconstructs navigation truth.
    pub support_export_reconstructs_navigation: bool,
    /// Later M5 rows cannot invent parallel live-resource vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveResourceConsumerProjection {
    /// Navigator / compare / explorer / banner surfaces all consume the shared
    /// primitive.
    pub navigation_surfaces_consume_shared_primitive: bool,
    /// The navigation resolver reads a single canonical model.
    pub resolver_reads_single_model: bool,
    /// The drift banner reads a single canonical status source.
    pub drift_banner_reads_single_status_source: bool,
    /// Support / export reads a single canonical navigation source.
    pub support_export_reads_single_source: bool,
}

/// Release and support parity posture for the live-resource navigation primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveResourceReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting navigation audit.
    pub navigation_audit_ref: String,
    /// True when support / export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5LiveResourcePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LiveResourcePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5LiveResourceSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LiveResourceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LiveResourceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LiveResourceConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5LiveResourceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 live-resource navigation primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LiveResourcePrimitivePacket {
    /// Record kind; must equal [`M5_LIVE_RESOURCE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LIVE_RESOURCE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5LiveResourceSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LiveResourceVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LiveResourceGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LiveResourceConsumerProjection,
    /// Release and support parity posture.
    pub release_posture: M5LiveResourceReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LiveResourcePrimitivePacket {
    /// Builds an M5 live-resource navigation primitive packet from stable-lane
    /// input.
    pub fn new(input: M5LiveResourcePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_LIVE_RESOURCE_RECORD_KIND.to_owned(),
            schema_version: M5_LIVE_RESOURCE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 live-resource navigation primitive invariants.
    pub fn validate(&self) -> Vec<M5LiveResourceViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LIVE_RESOURCE_RECORD_KIND {
            violations.push(M5LiveResourceViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LIVE_RESOURCE_SCHEMA_VERSION {
            violations.push(M5LiveResourceViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LiveResourceViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_acceptance_criteria_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 live-resource navigation primitive serializes"),
        ) {
            violations.push(M5LiveResourceViolation::RawMaterialInExport);
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
            .expect("m5 live-resource navigation primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per surface family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,owner,resource_kinds,truth_modes,link_classes,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.resource_kinds, |v| v.as_str()),
                join_tokens(&row.truth_modes, |v| truth_mode_token(*v)),
                join_tokens(&row.link_classes, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_navigation.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 Live-Resource Navigation Primitive: Link Row, Compare Card, Explorer Row, and Drift Banner\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Navigation surfaces: {} / {}\n",
            self.surface_rows.len(),
            M5LiveResourceSurfaceFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Resource kinds: {}\n",
            self.vocabulary_set.resource_kinds.join(", ")
        ));
        out.push_str(&format!(
            "- Compare verdicts: {}\n",
            self.vocabulary_set.compare_verdicts.join(", ")
        ));
        out.push_str(&format!(
            "- Permission postures: {}\n",
            self.vocabulary_set.permission_postures.join(", ")
        ));
        out.push_str("\n## Navigation surfaces\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!("- **{}**\n", row.surface_family.label()));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked cases: {}\n",
                row.example_navigation.len()
            ));
            for case in &row.example_navigation {
                out.push_str(&format!(
                    "    - `{}` → {} ({}), freshness `{}`, {}\n",
                    case.resolved.resource_id,
                    case.resolved.compare_card.compare_verdict.as_str(),
                    truth_mode_token(case.resolved.explorer_row.truth_mode),
                    case.resolved.explorer_row.freshness.as_str(),
                    if case.resolved.explorer_row.presents_as_current {
                        "current"
                    } else {
                        "narrowed"
                    },
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 live-resource navigation export.
#[derive(Debug)]
pub enum M5LiveResourceArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LiveResourceViolation>),
}

impl fmt::Display for M5LiveResourceArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 live-resource navigation primitive export parse failed: {error}"
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
                    "m5 live-resource navigation primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LiveResourceArtifactError {}

/// Validation failures emitted by [`M5LiveResourcePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LiveResourceViolation {
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
    /// A required navigation surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row declares no resource kinds.
    ResourceKindMissing,
    /// A surface row declares no truth classes.
    TruthModeMissing,
    /// A surface row declares no link classes.
    LinkClassMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no worked navigation cases.
    ExampleNavigationMissing,
    /// A worked navigation case does not match a fresh resolve of its input.
    ExampleNavigationDrift,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// No worked case proves resource identity preserved and source / live truth
    /// kept distinct (AC1).
    IdentityAndTruthUnproven,
    /// No worked case proves drift and unavailability visible before action (AC2).
    DriftVisibilityUnproven,
    /// No worked case proves partial / limited data never shown as current (AC3).
    PartialCurrencyUnproven,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5LiveResourceViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::ResourceKindMissing => "resource_kind_missing",
            Self::TruthModeMissing => "truth_mode_missing",
            Self::LinkClassMissing => "link_class_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ExampleNavigationMissing => "example_navigation_missing",
            Self::ExampleNavigationDrift => "example_navigation_drift",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::IdentityAndTruthUnproven => "identity_and_truth_unproven",
            Self::DriftVisibilityUnproven => "drift_visibility_unproven",
            Self::PartialCurrencyUnproven => "partial_currency_unproven",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 live-resource navigation export.
pub fn current_stable_m5_live_resource_export(
) -> Result<M5LiveResourcePrimitivePacket, M5LiveResourceArtifactError> {
    let packet: M5LiveResourcePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-live-resource-navigation-primitive-proof/support_export.json"
    )))
    .map_err(M5LiveResourceArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LiveResourceArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5LiveResourcePrimitivePacket,
    violations: &mut Vec<M5LiveResourceViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LIVE_RESOURCE_SCHEMA_REF,
        M5_LIVE_RESOURCE_DOC_REF,
        M5_LIVE_RESOURCE_COMPONENT_MATRIX_REF,
        M5_LIVE_RESOURCE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LiveResourceViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5LiveResourcePrimitivePacket,
    violations: &mut Vec<M5LiveResourceViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5LiveResourceViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5LiveResourcePrimitivePacket,
    violations: &mut Vec<M5LiveResourceViolation>,
) {
    let present: BTreeSet<M5LiveResourceSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5LiveResourceSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5LiveResourceViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(M5LiveResourceViolation::SurfaceRowIncomplete);
        }
        if row.resource_kinds.is_empty() {
            violations.push(M5LiveResourceViolation::ResourceKindMissing);
        }
        if row.truth_modes.is_empty() {
            violations.push(M5LiveResourceViolation::TruthModeMissing);
        }
        if row.link_classes.is_empty() {
            violations.push(M5LiveResourceViolation::LinkClassMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5LiveResourceViolation::MandatoryExportFieldMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5LiveResourceViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5LiveResourceViolation::ConsumerSurfacesMissing);
        }
        if row.example_navigation.is_empty() {
            violations.push(M5LiveResourceViolation::ExampleNavigationMissing);
        }
        if row
            .example_navigation
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5LiveResourceViolation::ExampleNavigationDrift);
        }
        if !row.honours_invariants() {
            violations.push(M5LiveResourceViolation::SurfaceInvariantViolated);
        }
    }
}

/// The acceptance criteria must each be demonstrated by at least one worked case
/// across the matrix: resource identity preserved and source / live truth kept
/// distinct (AC1), drift and unavailability visible before action (AC2), and
/// partial / permission-limited data never shown as current (AC3).
fn validate_acceptance_criteria_covered(
    packet: &M5LiveResourcePrimitivePacket,
    violations: &mut Vec<M5LiveResourceViolation>,
) {
    let cases: Vec<&M5ResolvedLiveResource> = packet
        .surface_rows
        .iter()
        .flat_map(|row| row.example_navigation.iter().map(|case| &case.resolved))
        .collect();

    // AC1: some case keeps one identity across surfaces, one disclosed truth
    // class, and two distinct source / live sides.
    let identity_and_truth_proven = cases.iter().any(|resolved| {
        resolved.identity_consistent()
            && resolved.truth_class_disclosed_consistently()
            && resolved.source_and_live_distinct()
    });
    if !identity_and_truth_proven {
        violations.push(M5LiveResourceViolation::IdentityAndTruthUnproven);
    }

    // AC2: some case actually has drift or unavailability disclosed by a present
    // banner, and every case keeps that disclosure before action.
    let drift_proven = cases.iter().any(|resolved| {
        (resolved.drift_banner.drift_present || resolved.drift_banner.unavailable)
            && resolved.drift_banner.banner_present
    }) && cases
        .iter()
        .all(|resolved| resolved.drift_visible_before_action());
    if !drift_proven {
        violations.push(M5LiveResourceViolation::DriftVisibilityUnproven);
    }

    // AC3: some case discloses partial / limited data as not-current, and every
    // case keeps the never-partial-as-current invariant.
    let partial_proven = cases
        .iter()
        .any(|resolved| !resolved.explorer_row.presents_as_current)
        && cases
            .iter()
            .all(|resolved| resolved.no_partial_shown_as_current());
    if !partial_proven {
        violations.push(M5LiveResourceViolation::PartialCurrencyUnproven);
    }
}

fn validate_governance_review(
    packet: &M5LiveResourcePrimitivePacket,
    violations: &mut Vec<M5LiveResourceViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_primitive_carries_all_surfaces,
        review.resource_identity_preserved_across_surfaces,
        review.source_and_live_never_blurred,
        review.drift_and_unavailability_visible_before_action,
        review.partial_or_limited_never_shown_as_current,
        review.support_export_reconstructs_navigation,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5LiveResourceViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LiveResourcePrimitivePacket,
    violations: &mut Vec<M5LiveResourceViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.navigation_surfaces_consume_shared_primitive,
        projection.resolver_reads_single_model,
        projection.drift_banner_reads_single_status_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5LiveResourceViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_release_posture(
    packet: &M5LiveResourcePrimitivePacket,
    violations: &mut Vec<M5LiveResourceViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.navigation_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5LiveResourceViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
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
        serde_json::Value::String(s) => value_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

include!("seed.rs");
