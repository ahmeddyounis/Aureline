//! Route explorers, component trees, and app-topology views with authored,
//! generated, and runtime-only truth.
//!
//! This module locks the canonical, export-safe packet for the app-structure
//! presentation lane. Each [`AppTopologyRow`] binds one node of a route explorer,
//! a component tree, or an app-topology view to its authored/generated/runtime-only
//! origin, the generator version that produced any generated node, how fresh the
//! view scan is, how the node truth was derived, the support class on which the
//! node may be presented, and its downgrade banner — so the route explorer,
//! component tree, app-topology view, diff-review, run, diagnostics, and support
//! surfaces project the same truth about whether a node was hand-authored,
//! generated into a managed zone, or only ever exists at runtime, instead of
//! presenting heuristic, bridged, or runtime-observed structure as exact authored
//! or generated source truth.
//!
//! The packet is metadata only. Raw source bodies, raw manifests, runtime
//! payloads, repository URLs, hostnames, secrets, and user-authored content never
//! cross this boundary; rows carry opaque refs, closed-vocabulary class tokens,
//! short reviewable summaries, structural locators, and export-safe chip labels.
//! It references the upstream template-manifest, framework-pack, and
//! generated-project-lineage contracts by ref rather than embedding them.
//!
//! [`AppTopologyPacket::apply_downgrade_automation`] narrows nodes whose origin
//! went unresolved, whose generator version was yanked, whose view scan went
//! stale, whose derivation could not be verified, or whose proof or upstream
//! dependency narrowed — withholding confident display and surfacing a downgrade
//! banner rather than hiding the node, so CI or release tooling narrows a stale or
//! underqualified view before it is presented.
//!
//! The boundary schema is
//! [`schemas/templates/ship-route-explorers-component-trees-and-app-topology-views-with-authored-generated-runtime-only-truth.schema.json`](../../../../schemas/templates/ship-route-explorers-component-trees-and-app-topology-views-with-authored-generated-runtime-only-truth.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth.md`](../../../../docs/frameworks/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/`](../../../../fixtures/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`AppTopologyPacket`].
pub const APP_TOPOLOGY_RECORD_KIND: &str = "route_component_and_app_topology_view_rows";

/// Schema version for app-topology view packets.
pub const APP_TOPOLOGY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const APP_TOPOLOGY_SCHEMA_REF: &str =
    "schemas/templates/ship-route-explorers-component-trees-and-app-topology-views-with-authored-generated-runtime-only-truth.schema.json";

/// Repo-relative path of the contract doc.
pub const APP_TOPOLOGY_DOC_REF: &str =
    "docs/frameworks/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth.md";

/// Repo-relative path of the upstream template-manifest contract this packet references.
pub const TEMPLATE_MANIFEST_CONTRACT_REF: &str =
    "schemas/templates/template_manifest_alpha.schema.json";

/// Repo-relative path of the upstream framework-pack contract this packet references.
pub const FRAMEWORK_PACK_CONTRACT_REF: &str =
    "schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json";

/// Repo-relative path of the upstream generated-project-lineage contract this packet references.
pub const GENERATED_LINEAGE_CONTRACT_REF: &str =
    "schemas/templates/generated_project_lineage_alpha.schema.json";

/// Repo-relative path of the template-registry and scaffold contract doc.
pub const TEMPLATE_REGISTRY_CONTRACT_DOC_REF: &str =
    "docs/templates/template_registry_and_scaffold_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const APP_TOPOLOGY_FIXTURE_DIR: &str =
    "fixtures/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth";

/// Repo-relative path of the checked support-export artifact.
pub const APP_TOPOLOGY_ARTIFACT_REF: &str =
    "artifacts/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/support_export.json";

/// Which structural view a node belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyViewKind {
    /// Route explorer / route tree view.
    RouteExplorer,
    /// Component hierarchy tree view.
    ComponentTree,
    /// App / module topology graph view.
    AppTopology,
}

impl TopologyViewKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteExplorer => "route_explorer",
            Self::ComponentTree => "component_tree",
            Self::AppTopology => "app_topology",
        }
    }
}

/// Authored/generated/runtime-only origin of a node — the central truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeOriginClass {
    /// Hand-authored by the user, outside any managed zone.
    Authored,
    /// Generated by the scaffold or generator into a managed zone.
    Generated,
    /// User-authored edits inside a generated/managed zone.
    AuthoredInGeneratedZone,
    /// Exists only at runtime (dynamically registered) and is absent from source.
    RuntimeOnly,
    /// Origin could not be resolved; review required.
    OriginUnknown,
}

impl NodeOriginClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Authored => "authored",
            Self::Generated => "generated",
            Self::AuthoredInGeneratedZone => "authored_in_generated_zone",
            Self::RuntimeOnly => "runtime_only",
            Self::OriginUnknown => "origin_unknown",
        }
    }

    /// Whether the origin is unresolved and must block any authored/generated claim.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::OriginUnknown)
    }

    /// Whether this node exists only at runtime and must be disclosed as such.
    pub const fn is_runtime_only(self) -> bool {
        matches!(self, Self::RuntimeOnly)
    }

    /// Whether this origin was produced by the generator and shows a generator version.
    pub const fn is_generated(self) -> bool {
        matches!(self, Self::Generated | Self::AuthoredInGeneratedZone)
    }
}

/// View scan freshness state for a node's view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ViewFreshnessClass {
    /// Verified fresh against the last scan.
    Fresh,
    /// A newer scan is available but the current view is still serviceable.
    RescanAvailable,
    /// Aging; a rescan is recommended.
    Aging,
    /// Stale; the view is past its freshness window.
    Stale,
    /// Freshness could not be determined.
    FreshnessUnknown,
}

impl ViewFreshnessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::RescanAvailable => "rescan_available",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }

    /// Whether this freshness state blocks presenting the view as current.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Stale | Self::FreshnessUnknown)
    }
}

/// How the node's structural truth was derived — keeps heuristic/bridge honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeDerivationClass {
    /// Derived exactly from the generator's manifest.
    GeneratorManifest,
    /// Derived exactly from static analysis of authored source.
    StaticAnalysis,
    /// Observed exactly from a runtime trace of the running app.
    RuntimeTrace,
    /// Inferred from naming or layout conventions only.
    HeuristicInference,
    /// Bridged from another tool's introspection rather than modeled natively.
    BridgedFromExternalTool,
    /// Derivation degraded below its declared class.
    DerivationDegraded,
    /// Derivation could not be verified.
    DerivationUnknown,
}

impl NodeDerivationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GeneratorManifest => "generator_manifest",
            Self::StaticAnalysis => "static_analysis",
            Self::RuntimeTrace => "runtime_trace",
            Self::HeuristicInference => "heuristic_inference",
            Self::BridgedFromExternalTool => "bridged_from_external_tool",
            Self::DerivationDegraded => "derivation_degraded",
            Self::DerivationUnknown => "derivation_unknown",
        }
    }

    /// Whether this derivation is exact (manifest, static analysis, or runtime trace).
    pub const fn is_exact(self) -> bool {
        matches!(
            self,
            Self::GeneratorManifest | Self::StaticAnalysis | Self::RuntimeTrace
        )
    }

    /// Whether this derivation state must show a derivation or downgrade banner.
    pub const fn requires_banner(self) -> bool {
        !self.is_exact()
    }

    /// Whether this derivation state blocks confident display.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::DerivationDegraded | Self::DerivationUnknown)
    }
}

/// Support class on which a node may be presented — keeps bridge/heuristic honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NodeSupportClass {
    /// Exactly modeled first-party structure.
    ExactlyModeled,
    /// Observed at runtime rather than modeled from source.
    RuntimeObserved,
    /// Experimental; may change without notice.
    Experimental,
    /// Bridge behavior: structure is bridged from another tool rather than modeled.
    BridgeBehavior,
    /// Heuristic mapping; inferred rather than exactly modeled.
    HeuristicMapping,
    /// Explicitly unsupported.
    Unsupported,
    /// Support class unknown.
    SupportUnknown,
}

impl NodeSupportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactlyModeled => "exactly_modeled",
            Self::RuntimeObserved => "runtime_observed",
            Self::Experimental => "experimental",
            Self::BridgeBehavior => "bridge_behavior",
            Self::HeuristicMapping => "heuristic_mapping",
            Self::Unsupported => "unsupported",
            Self::SupportUnknown => "support_unknown",
        }
    }

    /// Whether this class is bridge or heuristic behavior that must be disclosed.
    ///
    /// Bridge and heuristic nodes must never be presented as exact authored or
    /// generated truth without a known issue, a support-class banner, and the
    /// matching disclosure trigger.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::BridgeBehavior | Self::HeuristicMapping)
    }
}

/// Downgrade banner shown for a node — the explicit narrowing cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyDowngradeBannerClass {
    /// No downgrade banner is shown.
    NoBanner,
    /// Freshness banner: the view is aging, stale, or its generator version yanked.
    FreshnessBanner,
    /// Derivation banner: the node is heuristic, bridged, or degraded.
    DerivationBanner,
    /// Support-class banner: bridge or heuristic behavior is disclosed.
    SupportClassBanner,
    /// Policy-block banner: the node is blocked by policy or trust.
    PolicyBlockBanner,
    /// Origin-unknown banner: the node's origin could not be resolved.
    OriginUnknownBanner,
}

impl TopologyDowngradeBannerClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBanner => "no_banner",
            Self::FreshnessBanner => "freshness_banner",
            Self::DerivationBanner => "derivation_banner",
            Self::SupportClassBanner => "support_class_banner",
            Self::PolicyBlockBanner => "policy_block_banner",
            Self::OriginUnknownBanner => "origin_unknown_banner",
        }
    }

    /// Whether a banner is shown at all.
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoBanner)
    }

    /// Whether this banner hard-blocks confident display (not merely a soft cue).
    pub const fn is_hard_block(self) -> bool {
        matches!(self, Self::PolicyBlockBanner | Self::OriginUnknownBanner)
    }
}

/// Downgrade trigger that can narrow an app-topology node below its claimed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The node origin could not be resolved.
    OriginUnknown,
    /// The generator version that produced the node was yanked.
    GeneratorVersionYanked,
    /// The view scan went stale.
    ScanStale,
    /// The node derivation degraded below its declared class.
    DerivationDegraded,
    /// Bridge behavior is disclosed and held from exact-truth claims.
    BridgeBehaviorDisclosed,
    /// Heuristic mapping is disclosed and held from exact-truth claims.
    HeuristicMappingDisclosed,
    /// Runtime-only existence is disclosed and held from authored/generated claims.
    RuntimeOnlyDisclosed,
    /// A blocking known issue applies.
    KnownIssueBlocking,
    /// A validation bundle failed.
    ValidationFailed,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl TopologyDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::OriginUnknown,
        Self::GeneratorVersionYanked,
        Self::ScanStale,
        Self::DerivationDegraded,
        Self::BridgeBehaviorDisclosed,
        Self::HeuristicMappingDisclosed,
        Self::RuntimeOnlyDisclosed,
        Self::KnownIssueBlocking,
        Self::ValidationFailed,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::OriginUnknown => "origin_unknown",
            Self::GeneratorVersionYanked => "generator_version_yanked",
            Self::ScanStale => "scan_stale",
            Self::DerivationDegraded => "derivation_degraded",
            Self::BridgeBehaviorDisclosed => "bridge_behavior_disclosed",
            Self::HeuristicMappingDisclosed => "heuristic_mapping_disclosed",
            Self::RuntimeOnlyDisclosed => "runtime_only_disclosed",
            Self::KnownIssueBlocking => "known_issue_blocking",
            Self::ValidationFailed => "validation_failed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project an app-topology node's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TopologyConsumerSurface {
    /// Route explorer view.
    RouteExplorer,
    /// Component tree view.
    ComponentTree,
    /// App-topology graph view.
    AppTopologyView,
    /// Generation diff-review surface.
    DiffReview,
    /// Scaffold or app run surface.
    RunSurface,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl TopologyConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::RouteExplorer,
        Self::ComponentTree,
        Self::AppTopologyView,
        Self::DiffReview,
        Self::RunSurface,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RouteExplorer => "route_explorer",
            Self::ComponentTree => "component_tree",
            Self::AppTopologyView => "app_topology_view",
            Self::DiffReview => "diff_review",
            Self::RunSurface => "run_surface",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One app-topology row: one node and its origin, derivation, and banner truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppTopologyRow {
    /// Opaque stable row id.
    pub row_id: String,
    /// Which structural view this node belongs to.
    pub view_kind: TopologyViewKind,
    /// Opaque stable node id.
    pub node_id: String,
    /// Display label for the node.
    pub node_label: String,
    /// Structural locator (route pattern, component path, or module ref).
    pub node_path: String,
    /// Opaque stable app / project id.
    pub app_id: String,
    /// Generator version that produced a generated node; a sentinel otherwise.
    pub generator_version: String,
    /// Short reviewable origin summary.
    pub origin_summary: String,
    /// Authored/generated/runtime-only origin of the node.
    pub origin_class: NodeOriginClass,
    /// View scan freshness state.
    pub freshness_class: ViewFreshnessClass,
    /// Export-safe freshness/scan chip label.
    pub freshness_chip_label: String,
    /// RFC 3339 timestamp the view was last scanned.
    pub last_scanned: String,
    /// How the node truth was derived.
    pub derivation_class: NodeDerivationClass,
    /// Short reviewable derivation summary.
    pub derivation_summary: String,
    /// Support class on which the node may be presented.
    pub support_class: NodeSupportClass,
    /// Downgrade banner shown for this node.
    pub downgrade_banner_class: TopologyDowngradeBannerClass,
    /// Opaque known-issue refs disclosed before the node is presented.
    pub known_issue_refs: Vec<String>,
    /// Whether this node is admitted to be presented as confident truth.
    pub admitted_for_display: bool,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<TopologyDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<TopologyConsumerSurface>,
}

impl AppTopologyRow {
    /// Whether this row is structurally blocked from confident display.
    pub const fn is_blocked(&self) -> bool {
        self.freshness_class.is_blocking()
            || self.derivation_class.is_blocking()
            || self.origin_class.is_unknown()
            || self.downgrade_banner_class.is_hard_block()
    }
}

/// Review block asserting the lane's honesty invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppTopologyReview {
    /// Every node shows its authored/generated/runtime-only origin.
    pub view_shows_origin_for_every_node: bool,
    /// A generator version is shown for every generated node.
    pub generator_version_shown_for_generated_nodes: bool,
    /// A freshness chip is shown for every view.
    pub freshness_chip_shown_for_every_view: bool,
    /// A derivation banner is shown whenever derivation is not exact.
    pub derivation_banner_shown_when_not_exact: bool,
    /// A downgrade banner is shown whenever a node is narrowed.
    pub downgrade_banner_shown_when_narrowed: bool,
    /// A runtime-only node is never presented as authored or generated source.
    pub runtime_only_never_presented_as_authored_or_generated: bool,
    /// Bridge or heuristic structure is never presented as exact truth.
    pub bridge_or_heuristic_never_presented_as_exact_truth: bool,
    /// A stale view is never presented as current.
    pub stale_view_not_presented_as_current: bool,
    /// An origin-unknown node is labeled rather than hidden.
    pub origin_unknown_node_labeled_not_hidden: bool,
    /// The support class is visible before a node is presented.
    pub support_class_visible_before_display: bool,
    /// Known issues are disclosed before a node is presented.
    pub known_issues_disclosed_before_display: bool,
    /// No raw source bodies or URLs cross the export boundary.
    pub no_raw_source_bodies_or_urls_in_export: bool,
    /// Downgrade narrows the node's claim rather than hiding the node.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppTopologyConsumerProjection {
    /// Route explorer shows the node origin.
    pub route_explorer_shows_node_origin: bool,
    /// Component tree shows the node origin.
    pub component_tree_shows_node_origin: bool,
    /// App-topology view shows the node origin.
    pub app_topology_shows_node_origin: bool,
    /// Structure views show the freshness chip.
    pub structure_view_shows_freshness_chip: bool,
    /// Run surface shows the derivation banner.
    pub run_surface_shows_derivation_banner: bool,
    /// Diff-review surface shows the downgrade banner.
    pub diff_review_shows_downgrade_banner: bool,
    /// CLI / headless shows topology rows.
    pub cli_headless_shows_topology_rows: bool,
    /// Support export shows topology rows.
    pub support_export_shows_topology_rows: bool,
    /// Diagnostics shows freshness and derivation state.
    pub diagnostics_shows_freshness_and_derivation_state: bool,
    /// Blocked nodes are visibly labeled rather than hidden.
    pub blocked_nodes_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppTopologyProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected rows.
    pub auto_narrow_on_stale: bool,
}

/// Per-row observation fed to [`AppTopologyPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppTopologyRowObservation {
    /// Row id the observation applies to.
    pub row_id: String,
    /// True when the node origin currently resolves.
    pub origin_resolved: bool,
    /// True when the node's generator version is current (not yanked).
    pub generator_version_current: bool,
    /// True when the view scan is currently fresh.
    pub scan_fresh: bool,
    /// True when the node derivation currently verifies.
    pub derivation_verified: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`AppTopologyPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppTopologyPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Topology rows.
    pub rows: Vec<AppTopologyRow>,
    /// Review block.
    pub review: AppTopologyReview,
    /// Consumer projection block.
    pub consumer_projection: AppTopologyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: AppTopologyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe route-explorer, component-tree, and app-topology view packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AppTopologyPacket {
    /// Record kind; must equal [`APP_TOPOLOGY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`APP_TOPOLOGY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Topology rows.
    pub rows: Vec<AppTopologyRow>,
    /// Review block.
    pub review: AppTopologyReview,
    /// Consumer projection block.
    pub consumer_projection: AppTopologyConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: AppTopologyProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AppTopologyPacket {
    /// Builds an app-topology packet from stable-row input.
    pub fn new(input: AppTopologyPacketInput) -> Self {
        Self {
            record_kind: APP_TOPOLOGY_RECORD_KIND.to_owned(),
            schema_version: APP_TOPOLOGY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            rows: input.rows,
            review: input.review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows nodes whose origin went unresolved, whose generator version was
    /// yanked, whose view scan went stale, whose derivation could not be verified,
    /// or whose proof or upstream narrowed.
    ///
    /// Unresolved origin is the hardest block: the origin, freshness, and
    /// derivation are all marked unknown, an origin-unknown banner is raised, and
    /// the node loses confident display. A yanked generator version or a stale
    /// scan narrows freshness to stale and raises a freshness banner. An unverified
    /// derivation narrows the derivation to degraded and raises a derivation
    /// banner. Stale proof or a narrowed upstream withholds display until evidence
    /// refreshes. A raised banner is never lowered. Rows without a matching
    /// observation are left unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[AppTopologyRowObservation]) {
        for row in &mut self.rows {
            let Some(observation) = observations.iter().find(|obs| obs.row_id == row.row_id) else {
                continue;
            };

            if !observation.origin_resolved {
                row.origin_class = NodeOriginClass::OriginUnknown;
                row.freshness_class = ViewFreshnessClass::FreshnessUnknown;
                row.derivation_class = NodeDerivationClass::DerivationUnknown;
                row.downgrade_banner_class = TopologyDowngradeBannerClass::OriginUnknownBanner;
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    TopologyDowngradeTrigger::OriginUnknown,
                );
                continue;
            }

            if !observation.generator_version_current {
                row.freshness_class = ViewFreshnessClass::Stale;
                raise_banner(row, TopologyDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    TopologyDowngradeTrigger::GeneratorVersionYanked,
                );
            }

            if !observation.scan_fresh {
                if !row.freshness_class.is_blocking() {
                    row.freshness_class = ViewFreshnessClass::Stale;
                }
                raise_banner(row, TopologyDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    TopologyDowngradeTrigger::ScanStale,
                );
            }

            if !observation.derivation_verified {
                row.derivation_class = NodeDerivationClass::DerivationDegraded;
                raise_banner(row, TopologyDowngradeBannerClass::DerivationBanner);
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    TopologyDowngradeTrigger::DerivationDegraded,
                );
            }

            if (!observation.proof_fresh || observation.upstream_narrowed)
                && row.admitted_for_display
            {
                row.admitted_for_display = false;
                let trigger = if observation.proof_fresh {
                    TopologyDowngradeTrigger::UpstreamDependencyNarrowed
                } else {
                    TopologyDowngradeTrigger::ProofStale
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
            }
        }
    }

    /// Validates the app-topology invariants.
    pub fn validate(&self) -> Vec<AppTopologyViolation> {
        let mut violations = Vec::new();

        if self.record_kind != APP_TOPOLOGY_RECORD_KIND {
            violations.push(AppTopologyViolation::WrongRecordKind);
        }
        if self.schema_version != APP_TOPOLOGY_SCHEMA_VERSION {
            violations.push(AppTopologyViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AppTopologyViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("app-topology packet serializes"),
        ) {
            violations.push(AppTopologyViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("app-topology packet serializes")
    }

    /// Rows currently admitted to be presented as confident truth.
    pub fn admitted_rows(&self) -> impl Iterator<Item = &AppTopologyRow> {
        self.rows.iter().filter(|row| row.admitted_for_display)
    }

    /// Deterministic Markdown summary for structure-view, support, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let admitted = self.admitted_rows().count();
        let mut out = String::new();
        out.push_str(
            "# Route Explorers, Component Trees, and App-Topology Views with Authored/Generated/Runtime-Only Truth\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Rows: {} ({} admitted for display)\n",
            self.rows.len(),
            admitted
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** `{}` ({}): {} / {}\n",
                row.node_label,
                row.node_path,
                row.view_kind.as_str(),
                row.origin_class.as_str(),
                row.support_class.as_str()
            ));
            out.push_str(&format!("  - Origin: {}\n", row.origin_summary));
            out.push_str(&format!(
                "  - Freshness chip: {} ({})\n",
                row.freshness_chip_label,
                row.freshness_class.as_str()
            ));
            out.push_str(&format!(
                "  - Derivation: {} (banner: {})\n",
                row.derivation_class.as_str(),
                row.downgrade_banner_class.as_str()
            ));
            out.push_str(&format!(
                "  - Generator version: {}\n",
                row.generator_version
            ));
            out.push_str(&format!("  - Displayed: {}\n", row.admitted_for_display));
        }
        out
    }
}

/// Errors emitted when reading the checked-in app-topology export.
#[derive(Debug)]
pub enum AppTopologyArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AppTopologyViolation>),
}

impl fmt::Display for AppTopologyArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "app-topology export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "app-topology export failed validation: {tokens}")
            }
        }
    }
}

impl Error for AppTopologyArtifactError {}

/// Validation failures emitted by [`AppTopologyPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppTopologyViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no rows.
    RowsEmpty,
    /// A row is incomplete.
    RowIncomplete,
    /// A generated node is missing its generator version.
    GeneratorVersionMissing,
    /// A non-exact-derivation row is missing its derivation or downgrade banner.
    DerivationBannerMissing,
    /// A bridge/heuristic row is missing a known issue, banner, or disclosure trigger.
    BridgeBehaviorUndisclosed,
    /// A runtime-only row is missing its runtime-only disclosure trigger.
    RuntimeOnlyUndisclosed,
    /// An origin-unknown row is missing its origin-unknown banner.
    OriginUnknownBannerMissing,
    /// A stale or unknown-freshness row is missing a downgrade banner.
    FreshnessBannerMissing,
    /// A blocked row is still admitted for confident display.
    BlockedDisplayAdmitted,
    /// A row has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Review block does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AppTopologyViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsEmpty => "rows_empty",
            Self::RowIncomplete => "row_incomplete",
            Self::GeneratorVersionMissing => "generator_version_missing",
            Self::DerivationBannerMissing => "derivation_banner_missing",
            Self::BridgeBehaviorUndisclosed => "bridge_behavior_undisclosed",
            Self::RuntimeOnlyUndisclosed => "runtime_only_undisclosed",
            Self::OriginUnknownBannerMissing => "origin_unknown_banner_missing",
            Self::FreshnessBannerMissing => "freshness_banner_missing",
            Self::BlockedDisplayAdmitted => "blocked_display_admitted",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in app-topology export.
///
/// This is the first real consumer of the app-topology lane: a route explorer,
/// component tree, app-topology view, run, diagnostics, or support-export surface
/// calls it to ingest the canonical packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`AppTopologyArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_app_topology_export() -> Result<AppTopologyPacket, AppTopologyArtifactError> {
    let packet: AppTopologyPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/ship_route_explorers_component_trees_and_app_topology_views_with_authored_generated_runtime_only_truth/support_export.json"
    )))
    .map_err(AppTopologyArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AppTopologyArtifactError::Validation(violations))
    }
}

/// Canonical review block with every invariant satisfied.
pub fn canonical_review() -> AppTopologyReview {
    AppTopologyReview {
        view_shows_origin_for_every_node: true,
        generator_version_shown_for_generated_nodes: true,
        freshness_chip_shown_for_every_view: true,
        derivation_banner_shown_when_not_exact: true,
        downgrade_banner_shown_when_narrowed: true,
        runtime_only_never_presented_as_authored_or_generated: true,
        bridge_or_heuristic_never_presented_as_exact_truth: true,
        stale_view_not_presented_as_current: true,
        origin_unknown_node_labeled_not_hidden: true,
        support_class_visible_before_display: true,
        known_issues_disclosed_before_display: true,
        no_raw_source_bodies_or_urls_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting node truth.
pub fn canonical_consumer_projection() -> AppTopologyConsumerProjection {
    AppTopologyConsumerProjection {
        route_explorer_shows_node_origin: true,
        component_tree_shows_node_origin: true,
        app_topology_shows_node_origin: true,
        structure_view_shows_freshness_chip: true,
        run_surface_shows_derivation_banner: true,
        diff_review_shows_downgrade_banner: true,
        cli_headless_shows_topology_rows: true,
        support_export_shows_topology_rows: true,
        diagnostics_shows_freshness_and_derivation_state: true,
        blocked_nodes_labeled_not_hidden: true,
    }
}

/// Canonical source contract refs that every app-topology export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        APP_TOPOLOGY_SCHEMA_REF.to_owned(),
        APP_TOPOLOGY_DOC_REF.to_owned(),
        TEMPLATE_MANIFEST_CONTRACT_REF.to_owned(),
        FRAMEWORK_PACK_CONTRACT_REF.to_owned(),
        GENERATED_LINEAGE_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF.to_owned(),
    ]
}

/// Builds the canonical app-topology packet from stable-row truth.
///
/// The rows mirror the checked-in support export and cover the origin,
/// derivation, and view spectrum: an authored route shown exactly with no banner,
/// a generated route shown exactly with its generator version, an authored edit
/// inside a generated component zone shown with managed-zone honesty, a generated
/// component whose tree was inferred heuristically and held behind its
/// support-class banner, a runtime-only route shown but labeled runtime-only, and
/// an app-topology node whose origin could not be resolved and is blocked rather
/// than hidden.
pub fn canonical_app_topology(
    packet_id: String,
    packet_label: String,
    minted_at: String,
    proof_freshness: AppTopologyProofFreshness,
) -> AppTopologyPacket {
    AppTopologyPacket::new(AppTopologyPacketInput {
        packet_id,
        packet_label,
        rows: canonical_rows(),
        review: canonical_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical rows that match the checked-in support export.
pub fn canonical_rows() -> Vec<AppTopologyRow> {
    use TopologyConsumerSurface as Surface;
    use TopologyDowngradeTrigger as Trigger;

    vec![
        AppTopologyRow {
            row_id: "app-topology-row:route.authored.dashboard:2026.06".to_owned(),
            view_kind: TopologyViewKind::RouteExplorer,
            node_id: "node:route.authored.dashboard".to_owned(),
            node_label: "Dashboard route".to_owned(),
            node_path: "/dashboard".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            generator_version: "not_generated".to_owned(),
            origin_summary: "Hand-authored route outside any managed zone; the route explorer marks it authored, the freshness chip reads fresh against the last scan, and static analysis derives it exactly so it is shown with no downgrade banner".to_owned(),
            origin_class: NodeOriginClass::Authored,
            freshness_class: ViewFreshnessClass::Fresh,
            freshness_chip_label: "scanned · fresh".to_owned(),
            last_scanned: "2026-06-08T00:00:00Z".to_owned(),
            derivation_class: NodeDerivationClass::StaticAnalysis,
            derivation_summary: "Derived exactly from static analysis of the authored route module; no inference or bridging is involved".to_owned(),
            support_class: NodeSupportClass::ExactlyModeled,
            downgrade_banner_class: TopologyDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::OriginUnknown,
                Trigger::ScanStale,
                Trigger::DerivationDegraded,
            ],
            consumer_surfaces: vec![
                Surface::RouteExplorer,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        AppTopologyRow {
            row_id: "app-topology-row:route.generated.users_api:2026.06".to_owned(),
            view_kind: TopologyViewKind::RouteExplorer,
            node_id: "node:route.generated.users_api".to_owned(),
            node_label: "Users API route".to_owned(),
            node_path: "/api/users".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            generator_version: "1.8.0".to_owned(),
            origin_summary: "Generated into a managed zone by the scaffold; the route explorer marks it generated and shows the pinned generator version, the freshness chip reads fresh, and the generator manifest derives it exactly".to_owned(),
            origin_class: NodeOriginClass::Generated,
            freshness_class: ViewFreshnessClass::Fresh,
            freshness_chip_label: "scanned · fresh".to_owned(),
            last_scanned: "2026-06-08T00:00:00Z".to_owned(),
            derivation_class: NodeDerivationClass::GeneratorManifest,
            derivation_summary: "Derived exactly from the generator manifest entry that produced this managed route; the generator version is shown so generated truth stays inspectable".to_owned(),
            support_class: NodeSupportClass::ExactlyModeled,
            downgrade_banner_class: TopologyDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::OriginUnknown,
                Trigger::GeneratorVersionYanked,
                Trigger::ScanStale,
                Trigger::DerivationDegraded,
            ],
            consumer_surfaces: vec![
                Surface::RouteExplorer,
                Surface::DiffReview,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        AppTopologyRow {
            row_id: "app-topology-row:component.authored_in_zone.user_card:2026.06".to_owned(),
            view_kind: TopologyViewKind::ComponentTree,
            node_id: "node:component.authored_in_zone.user_card".to_owned(),
            node_label: "UserCard component".to_owned(),
            node_path: "components/managed/UserCard".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            generator_version: "1.8.0".to_owned(),
            origin_summary: "User-authored edits inside a generated component zone; the component tree marks it authored-in-generated-zone with managed-zone honesty, shows the generator version that owns the zone, and static analysis derives the edited node exactly so it is shown without claiming the edits are generated".to_owned(),
            origin_class: NodeOriginClass::AuthoredInGeneratedZone,
            freshness_class: ViewFreshnessClass::RescanAvailable,
            freshness_chip_label: "scanned · rescan available".to_owned(),
            last_scanned: "2026-06-06T00:00:00Z".to_owned(),
            derivation_class: NodeDerivationClass::StaticAnalysis,
            derivation_summary: "Derived exactly from static analysis of the authored edits inside the managed component zone; the managed-zone boundary is preserved so authored and generated regions stay distinguishable".to_owned(),
            support_class: NodeSupportClass::ExactlyModeled,
            downgrade_banner_class: TopologyDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::OriginUnknown,
                Trigger::GeneratorVersionYanked,
                Trigger::ScanStale,
                Trigger::DerivationDegraded,
            ],
            consumer_surfaces: vec![
                Surface::ComponentTree,
                Surface::DiffReview,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        AppTopologyRow {
            row_id: "app-topology-row:component.heuristic.legacy_widget:2026.05".to_owned(),
            view_kind: TopologyViewKind::ComponentTree,
            node_id: "node:component.heuristic.legacy_widget".to_owned(),
            node_label: "LegacyWidget component".to_owned(),
            node_path: "components/legacy/LegacyWidget".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            generator_version: "not_generated".to_owned(),
            origin_summary: "Component whose place in the tree is inferred from naming and layout conventions rather than modeled exactly; the component tree marks it heuristic, the support-class banner discloses the heuristic mapping and its known issue, and the node is held from being presented as exact authored or generated truth".to_owned(),
            origin_class: NodeOriginClass::Authored,
            freshness_class: ViewFreshnessClass::Aging,
            freshness_chip_label: "scanned · aging".to_owned(),
            last_scanned: "2026-05-20T00:00:00Z".to_owned(),
            derivation_class: NodeDerivationClass::HeuristicInference,
            derivation_summary: "The component's parent and slot are inferred from naming and layout conventions only; this is heuristic mapping, not exact modeling, and is disclosed by the support-class banner".to_owned(),
            support_class: NodeSupportClass::HeuristicMapping,
            downgrade_banner_class: TopologyDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:component_tree:heuristic_parent_inference".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HeuristicMappingDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::ComponentTree,
                Surface::DiffReview,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        AppTopologyRow {
            row_id: "app-topology-row:topology.runtime_only.webhook:2026.06".to_owned(),
            view_kind: TopologyViewKind::AppTopology,
            node_id: "node:topology.runtime_only.webhook".to_owned(),
            node_label: "Webhook handler (runtime)".to_owned(),
            node_path: "runtime:/webhooks/:provider".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            generator_version: "runtime_registered".to_owned(),
            origin_summary: "Route registered dynamically at runtime and absent from source; the app-topology view marks it runtime-only and observes it from a runtime trace, so it is shown but never presented as authored or generated source truth".to_owned(),
            origin_class: NodeOriginClass::RuntimeOnly,
            freshness_class: ViewFreshnessClass::Fresh,
            freshness_chip_label: "traced · fresh".to_owned(),
            last_scanned: "2026-06-08T00:00:00Z".to_owned(),
            derivation_class: NodeDerivationClass::RuntimeTrace,
            derivation_summary: "Observed exactly from a runtime trace of the running app; the node has no source backing and is disclosed as runtime-only rather than folded into the authored or generated views".to_owned(),
            support_class: NodeSupportClass::RuntimeObserved,
            downgrade_banner_class: TopologyDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::OriginUnknown,
                Trigger::ScanStale,
                Trigger::RuntimeOnlyDisclosed,
            ],
            consumer_surfaces: vec![
                Surface::AppTopologyView,
                Surface::RunSurface,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        AppTopologyRow {
            row_id: "app-topology-row:topology.unknown_origin.orphan:2026.04".to_owned(),
            view_kind: TopologyViewKind::AppTopology,
            node_id: "node:topology.unknown_origin.orphan".to_owned(),
            node_label: "Orphan module (unresolved)".to_owned(),
            node_path: "module:unresolved.orphan".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            generator_version: "unknown".to_owned(),
            origin_summary: "App-topology node whose origin could not be resolved to authored, generated, or runtime-only; the view marks it origin-unknown, the freshness chip reads unverified, the derivation reads unknown, and the origin-unknown banner blocks confident display rather than hiding the node".to_owned(),
            origin_class: NodeOriginClass::OriginUnknown,
            freshness_class: ViewFreshnessClass::FreshnessUnknown,
            freshness_chip_label: "scan · unverified".to_owned(),
            last_scanned: "2026-04-10T00:00:00Z".to_owned(),
            derivation_class: NodeDerivationClass::DerivationUnknown,
            derivation_summary: "Origin and derivation could not be resolved for this node; it is labeled unknown and blocked rather than presented as authored, generated, or runtime-only truth".to_owned(),
            support_class: NodeSupportClass::SupportUnknown,
            downgrade_banner_class: TopologyDowngradeBannerClass::OriginUnknownBanner,
            known_issue_refs: vec![
                "known-issue:app_topology:orphan_origin_unresolved".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::OriginUnknown,
                Trigger::ScanStale,
                Trigger::DerivationDegraded,
            ],
            consumer_surfaces: vec![
                Surface::AppTopologyView,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
    ]
}

fn validate_source_contracts(
    packet: &AppTopologyPacket,
    violations: &mut Vec<AppTopologyViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        APP_TOPOLOGY_SCHEMA_REF,
        APP_TOPOLOGY_DOC_REF,
        TEMPLATE_MANIFEST_CONTRACT_REF,
        FRAMEWORK_PACK_CONTRACT_REF,
        GENERATED_LINEAGE_CONTRACT_REF,
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AppTopologyViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(packet: &AppTopologyPacket, violations: &mut Vec<AppTopologyViolation>) {
    if packet.rows.is_empty() {
        violations.push(AppTopologyViolation::RowsEmpty);
        return;
    }

    for row in &packet.rows {
        if row.row_id.trim().is_empty()
            || row.node_id.trim().is_empty()
            || row.node_label.trim().is_empty()
            || row.node_path.trim().is_empty()
            || row.app_id.trim().is_empty()
            || row.generator_version.trim().is_empty()
            || row.origin_summary.trim().is_empty()
            || row.freshness_chip_label.trim().is_empty()
            || row.last_scanned.trim().is_empty()
            || row.derivation_summary.trim().is_empty()
        {
            violations.push(AppTopologyViolation::RowIncomplete);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(AppTopologyViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(AppTopologyViolation::ConsumerSurfacesMissing);
        }

        validate_row_banners(row, violations);
    }
}

fn validate_row_banners(row: &AppTopologyRow, violations: &mut Vec<AppTopologyViolation>) {
    // A generated node must show the generator version that produced it.
    if row.origin_class.is_generated() && row.generator_version.trim().is_empty() {
        violations.push(AppTopologyViolation::GeneratorVersionMissing);
    }

    // A non-exact-derivation node must show a banner.
    if row.derivation_class.requires_banner() && !row.downgrade_banner_class.is_present() {
        violations.push(AppTopologyViolation::DerivationBannerMissing);
    }

    // Bridge/heuristic nodes must disclose a known issue, a banner, and the matching trigger.
    if row.support_class.requires_disclosure() {
        let matching_trigger = match row.support_class {
            NodeSupportClass::BridgeBehavior => TopologyDowngradeTrigger::BridgeBehaviorDisclosed,
            _ => TopologyDowngradeTrigger::HeuristicMappingDisclosed,
        };
        if row.known_issue_refs.is_empty()
            || !row.downgrade_banner_class.is_present()
            || !row.downgrade_triggers.contains(&matching_trigger)
        {
            violations.push(AppTopologyViolation::BridgeBehaviorUndisclosed);
        }
    }

    // A runtime-only node must carry the runtime-only disclosure trigger so it is
    // never folded into the authored or generated source views.
    if row.origin_class.is_runtime_only()
        && !row
            .downgrade_triggers
            .contains(&TopologyDowngradeTrigger::RuntimeOnlyDisclosed)
    {
        violations.push(AppTopologyViolation::RuntimeOnlyUndisclosed);
    }

    // An origin-unknown node must carry the origin-unknown banner.
    if row.origin_class.is_unknown()
        && row.downgrade_banner_class != TopologyDowngradeBannerClass::OriginUnknownBanner
    {
        violations.push(AppTopologyViolation::OriginUnknownBannerMissing);
    }

    // A stale or unknown-freshness node must show a downgrade banner.
    if row.freshness_class.is_blocking() && !row.downgrade_banner_class.is_present() {
        violations.push(AppTopologyViolation::FreshnessBannerMissing);
    }

    // A blocked node cannot be admitted for confident display.
    if row.is_blocked() && row.admitted_for_display {
        violations.push(AppTopologyViolation::BlockedDisplayAdmitted);
    }
}

fn validate_review(packet: &AppTopologyPacket, violations: &mut Vec<AppTopologyViolation>) {
    let review = &packet.review;
    for ok in [
        review.view_shows_origin_for_every_node,
        review.generator_version_shown_for_generated_nodes,
        review.freshness_chip_shown_for_every_view,
        review.derivation_banner_shown_when_not_exact,
        review.downgrade_banner_shown_when_narrowed,
        review.runtime_only_never_presented_as_authored_or_generated,
        review.bridge_or_heuristic_never_presented_as_exact_truth,
        review.stale_view_not_presented_as_current,
        review.origin_unknown_node_labeled_not_hidden,
        review.support_class_visible_before_display,
        review.known_issues_disclosed_before_display,
        review.no_raw_source_bodies_or_urls_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(AppTopologyViolation::ReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &AppTopologyPacket,
    violations: &mut Vec<AppTopologyViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.route_explorer_shows_node_origin,
        projection.component_tree_shows_node_origin,
        projection.app_topology_shows_node_origin,
        projection.structure_view_shows_freshness_chip,
        projection.run_surface_shows_derivation_banner,
        projection.diff_review_shows_downgrade_banner,
        projection.cli_headless_shows_topology_rows,
        projection.support_export_shows_topology_rows,
        projection.diagnostics_shows_freshness_and_derivation_state,
        projection.blocked_nodes_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(AppTopologyViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &AppTopologyPacket,
    violations: &mut Vec<AppTopologyViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(AppTopologyViolation::ProofFreshnessIncomplete);
    }
}

/// Raises the row's downgrade banner only when none is currently shown, so an
/// already-raised banner is never lowered to a softer cue.
fn raise_banner(row: &mut AppTopologyRow, banner: TopologyDowngradeBannerClass) {
    if !row.downgrade_banner_class.is_present() {
        row.downgrade_banner_class = banner;
    }
}

fn push_unique_trigger(
    triggers: &mut Vec<TopologyDowngradeTrigger>,
    trigger: TopologyDowngradeTrigger,
) {
    if !triggers.contains(&trigger) {
        triggers.push(trigger);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
