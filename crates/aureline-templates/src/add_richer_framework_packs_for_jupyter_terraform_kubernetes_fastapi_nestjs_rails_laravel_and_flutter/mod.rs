//! Richer framework-pack lane catalog for the notebook-adjacency,
//! infrastructure, web-API, and mobile lanes.
//!
//! This module locks the canonical, export-safe packet for the deeper
//! framework-pack catalog. Each [`FrameworkLaneRow`] binds one framework pack —
//! across the Jupyter-adjacency, Terraform, Kubernetes, FastAPI, Nest, Rails,
//! Laravel, and Flutter lanes — to its lane domain, its header provenance, its
//! pinned pack and generator versions, its freshness chip, its capability and
//! support class, its authored/generated/runtime-only origin truth, its
//! archetype health state, and its downgrade banner. The gallery, pack header,
//! run, diff-review, diagnostics, and support surfaces project the same truth
//! about where a pack came from, what generated it, how fresh it is, what it can
//! actually produce, whether its scaffolding is authored or generated or only
//! observed at runtime, and on what terms it may be offered — instead of letting
//! a richer long-tail of lanes present heuristic or bridge behavior as exact
//! first-party truth.
//!
//! The packet is metadata only. Raw pack bodies, raw manifests, repository URLs,
//! hostnames, secrets, and user-authored content never cross this boundary; rows
//! carry opaque refs, closed-vocabulary class tokens, short reviewable summaries,
//! and export-safe chip labels. It references the upstream template-manifest,
//! template-registry, and framework-pack-header contracts by ref rather than
//! embedding them, and reuses the prior support-class vocabulary instead of
//! inventing parallel terms.
//!
//! [`FrameworkLanePacket::apply_downgrade_automation`] narrows rows whose
//! provenance went unverified, whose pack or generator version was yanked, whose
//! freshness went stale, whose capability could not be verified, whose archetype
//! health degraded, whose origin truth could not be verified, or whose proof or
//! upstream dependency narrowed — withholding the offer and surfacing a downgrade
//! banner rather than hiding the row, so CI or release tooling narrows a stale or
//! underqualified lane pack before it is offered.
//!
//! The boundary schema is
//! [`schemas/templates/add-richer-framework-packs-for-jupyter-terraform-kubernetes-fastapi-nestjs-rails-laravel-and-flutter.schema.json`](../../../../schemas/templates/add-richer-framework-packs-for-jupyter-terraform-kubernetes-fastapi-nestjs-rails-laravel-and-flutter.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter.md`](../../../../docs/frameworks/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/`](../../../../fixtures/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`FrameworkLanePacket`].
pub const RICHER_FRAMEWORK_PACK_RECORD_KIND: &str = "richer_framework_pack_lane_rows";

/// Schema version for richer framework-pack lane packets.
pub const RICHER_FRAMEWORK_PACK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const RICHER_FRAMEWORK_PACK_SCHEMA_REF: &str =
    "schemas/templates/add-richer-framework-packs-for-jupyter-terraform-kubernetes-fastapi-nestjs-rails-laravel-and-flutter.schema.json";

/// Repo-relative path of the contract doc.
pub const RICHER_FRAMEWORK_PACK_DOC_REF: &str =
    "docs/frameworks/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter.md";

/// Repo-relative path of the upstream template-manifest contract this packet references.
pub const TEMPLATE_MANIFEST_CONTRACT_REF: &str =
    "schemas/templates/template_manifest_alpha.schema.json";

/// Repo-relative path of the upstream template-registry-entry contract this packet references.
pub const TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF: &str =
    "schemas/templates/template_registry_entry.schema.json";

/// Repo-relative path of the upstream framework-pack-header contract this packet extends.
pub const FRAMEWORK_PACK_HEADER_CONTRACT_REF: &str =
    "schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json";

/// Repo-relative path of the template-registry and scaffold contract doc.
pub const TEMPLATE_REGISTRY_CONTRACT_DOC_REF: &str =
    "docs/templates/template_registry_and_scaffold_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const RICHER_FRAMEWORK_PACK_FIXTURE_DIR: &str =
    "fixtures/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter";

/// Repo-relative path of the checked support-export artifact.
pub const RICHER_FRAMEWORK_PACK_ARTIFACT_REF: &str =
    "artifacts/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/support_export.json";

/// Lane domain a framework pack serves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneDomainClass {
    /// Notebook-adjacency lane (e.g. Jupyter adjacency).
    NotebookAdjacency,
    /// Infrastructure-provisioning lane (e.g. Terraform, Kubernetes).
    InfrastructureProvisioning,
    /// Web-API service lane (e.g. FastAPI, Nest, Rails, Laravel).
    WebApiService,
    /// Mobile-application lane (e.g. Flutter).
    MobileApp,
    /// Lane domain could not be determined.
    DomainUnknown,
}

impl LaneDomainClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookAdjacency => "notebook_adjacency",
            Self::InfrastructureProvisioning => "infrastructure_provisioning",
            Self::WebApiService => "web_api_service",
            Self::MobileApp => "mobile_app",
            Self::DomainUnknown => "domain_unknown",
        }
    }
}

/// Header provenance and trust source for a lane pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanePackProvenanceClass {
    /// First-party authored and signed pack.
    FirstParty,
    /// Partner-authored pack certified by the registry.
    PartnerCertified,
    /// Community-authored pack.
    Community,
    /// Mirror of an upstream pack served from a registry mirror.
    Mirror,
    /// Pack that bridges another framework's generator rather than generating natively.
    BridgedFromOtherFramework,
    /// Provenance could not be verified; review required.
    ProvenanceUnknown,
}

impl LanePackProvenanceClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstParty => "first_party",
            Self::PartnerCertified => "partner_certified",
            Self::Community => "community",
            Self::Mirror => "mirror",
            Self::BridgedFromOtherFramework => "bridged_from_other_framework",
            Self::ProvenanceUnknown => "provenance_unknown",
        }
    }

    /// Whether provenance is unresolved and must block any first-party claim.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::ProvenanceUnknown)
    }
}

/// Freshness chip state for a lane pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanePackFreshnessClass {
    /// Verified fresh against the last refresh.
    Fresh,
    /// A newer pack version is available but the pinned version is still serviceable.
    UpdateAvailable,
    /// Aging; a refresh is recommended.
    Aging,
    /// Stale; the pack is past its freshness window.
    Stale,
    /// Freshness could not be determined.
    FreshnessUnknown,
}

impl LanePackFreshnessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::UpdateAvailable => "update_available",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }

    /// Whether this freshness state blocks offering the pack as current.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Stale | Self::FreshnessUnknown)
    }
}

/// Capability banner state for a lane pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanePackCapabilityClass {
    /// Full first-party project-model capability.
    FullCapability,
    /// Core capability with one or more partial generators, disclosed by a banner.
    PartialCapability,
    /// Capability is bridged from another ecosystem rather than generated natively.
    BridgedCapability,
    /// Capability is inferred from naming or layout conventions only.
    HeuristicCapability,
    /// Capability degraded below its declared class.
    CapabilityDegraded,
    /// Capability could not be verified.
    CapabilityUnknown,
}

impl LanePackCapabilityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullCapability => "full_capability",
            Self::PartialCapability => "partial_capability",
            Self::BridgedCapability => "bridged_capability",
            Self::HeuristicCapability => "heuristic_capability",
            Self::CapabilityDegraded => "capability_degraded",
            Self::CapabilityUnknown => "capability_unknown",
        }
    }

    /// Whether this capability state must show a capability or downgrade banner.
    pub const fn requires_banner(self) -> bool {
        !matches!(self, Self::FullCapability)
    }

    /// Whether this capability state blocks offering the pack.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::CapabilityDegraded | Self::CapabilityUnknown)
    }
}

/// Support class communicated for a lane pack — keeps bridge/heuristic behavior honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanePackSupportClass {
    /// Officially supported by the vendor.
    OfficiallySupported,
    /// Community-supported, best effort.
    CommunitySupported,
    /// Experimental; may change without notice.
    Experimental,
    /// Bridge behavior: some structure is bridged rather than first-party generated.
    BridgeBehavior,
    /// Heuristic mapping; not exact first-party generation.
    HeuristicMapping,
    /// Explicitly unsupported.
    Unsupported,
    /// Support class unknown.
    SupportUnknown,
}

impl LanePackSupportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficiallySupported => "officially_supported",
            Self::CommunitySupported => "community_supported",
            Self::Experimental => "experimental",
            Self::BridgeBehavior => "bridge_behavior",
            Self::HeuristicMapping => "heuristic_mapping",
            Self::Unsupported => "unsupported",
            Self::SupportUnknown => "support_unknown",
        }
    }

    /// Whether this class is bridge or heuristic behavior that must be disclosed.
    ///
    /// Bridge and heuristic lane packs must never be presented as exact
    /// first-party truth without a known issue, a support-class banner, and the
    /// matching disclosure trigger.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::BridgeBehavior | Self::HeuristicMapping)
    }
}

/// Authored/generated/runtime-only origin truth for a lane pack's scaffolding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanePackOriginTruthClass {
    /// Authored first-party source shipped as-is in the pack.
    AuthoredSource,
    /// Generated into a managed zone the pack owns and can regenerate.
    GeneratedManaged,
    /// Only observed at runtime; the pack does not author or generate it.
    RuntimeObserved,
    /// Bridged from an adjacent tool rather than authored or generated here.
    BridgedAdjacent,
    /// Origin truth could not be verified.
    OriginUnknown,
}

impl LanePackOriginTruthClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredSource => "authored_source",
            Self::GeneratedManaged => "generated_managed",
            Self::RuntimeObserved => "runtime_observed",
            Self::BridgedAdjacent => "bridged_adjacent",
            Self::OriginUnknown => "origin_unknown",
        }
    }

    /// Whether the origin truth is unresolved and blocks offering the pack.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::OriginUnknown)
    }
}

/// Archetype health state bound to a lane pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LaneArchetypeHealthClass {
    /// Certified-healthy against the archetype health-check bundle.
    CertifiedHealthy,
    /// Healthy but not certified by a health-check bundle.
    HealthyUncertified,
    /// Health degraded below its declared class.
    Degraded,
    /// Health could not be determined.
    HealthUnknown,
    /// Health check failed; the pack is blocked.
    Blocked,
}

impl LaneArchetypeHealthClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedHealthy => "certified_healthy",
            Self::HealthyUncertified => "healthy_uncertified",
            Self::Degraded => "degraded",
            Self::HealthUnknown => "health_unknown",
            Self::Blocked => "blocked",
        }
    }

    /// Whether this health state must show a health or downgrade banner.
    pub const fn requires_banner(self) -> bool {
        matches!(self, Self::Degraded | Self::HealthUnknown | Self::Blocked)
    }

    /// Whether this health state blocks offering the pack.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::HealthUnknown | Self::Blocked)
    }
}

/// Downgrade banner shown for a lane pack — the explicit narrowing cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LanePackDowngradeBannerClass {
    /// No downgrade banner is shown.
    NoBanner,
    /// Freshness banner: the pack is aging, stale, or yanked.
    FreshnessBanner,
    /// Capability banner: the pack is partial, bridged, or degraded.
    CapabilityBanner,
    /// Support-class banner: bridge or heuristic behavior is disclosed.
    SupportClassBanner,
    /// Origin-truth banner: the authored/generated/runtime-only truth is narrowed.
    OriginTruthBanner,
    /// Health banner: the archetype health degraded or is unknown.
    HealthBanner,
    /// Policy-block banner: the pack is blocked by policy or trust.
    PolicyBlockBanner,
    /// Provenance-unknown banner: the pack's provenance could not be verified.
    ProvenanceUnknownBanner,
}

impl LanePackDowngradeBannerClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBanner => "no_banner",
            Self::FreshnessBanner => "freshness_banner",
            Self::CapabilityBanner => "capability_banner",
            Self::SupportClassBanner => "support_class_banner",
            Self::OriginTruthBanner => "origin_truth_banner",
            Self::HealthBanner => "health_banner",
            Self::PolicyBlockBanner => "policy_block_banner",
            Self::ProvenanceUnknownBanner => "provenance_unknown_banner",
        }
    }

    /// Whether a banner is shown at all.
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoBanner)
    }

    /// Whether this banner hard-blocks the offer (not merely a soft cue).
    pub const fn is_hard_block(self) -> bool {
        matches!(
            self,
            Self::PolicyBlockBanner | Self::ProvenanceUnknownBanner
        )
    }
}

/// Downgrade trigger that can narrow a lane-pack row below its claimed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkLaneDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The pack provenance could not be verified.
    ProvenanceUnknown,
    /// The pinned pack version was yanked.
    PackVersionYanked,
    /// The pinned generator version was yanked.
    GeneratorVersionYanked,
    /// The pack freshness went stale.
    FreshnessStale,
    /// The pack capability degraded below its declared class.
    CapabilityDegraded,
    /// The archetype health degraded below its declared class.
    ArchetypeHealthDegraded,
    /// The authored/generated/runtime-only origin truth could not be verified.
    OriginTruthUnverified,
    /// Bridge behavior is disclosed and held from first-party claims.
    BridgeBehaviorDisclosed,
    /// Heuristic mapping is disclosed and held from first-party claims.
    HeuristicMappingDisclosed,
    /// A blocking known issue applies.
    KnownIssueBlocking,
    /// A validation bundle failed.
    ValidationFailed,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl FrameworkLaneDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProvenanceUnknown,
        Self::PackVersionYanked,
        Self::GeneratorVersionYanked,
        Self::FreshnessStale,
        Self::CapabilityDegraded,
        Self::ArchetypeHealthDegraded,
        Self::OriginTruthUnverified,
        Self::BridgeBehaviorDisclosed,
        Self::HeuristicMappingDisclosed,
        Self::KnownIssueBlocking,
        Self::ValidationFailed,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProvenanceUnknown => "provenance_unknown",
            Self::PackVersionYanked => "pack_version_yanked",
            Self::GeneratorVersionYanked => "generator_version_yanked",
            Self::FreshnessStale => "freshness_stale",
            Self::CapabilityDegraded => "capability_degraded",
            Self::ArchetypeHealthDegraded => "archetype_health_degraded",
            Self::OriginTruthUnverified => "origin_truth_unverified",
            Self::BridgeBehaviorDisclosed => "bridge_behavior_disclosed",
            Self::HeuristicMappingDisclosed => "heuristic_mapping_disclosed",
            Self::KnownIssueBlocking => "known_issue_blocking",
            Self::ValidationFailed => "validation_failed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a lane-pack row's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkLaneConsumerSurface {
    /// Template / starter gallery.
    Gallery,
    /// Framework-pack header surface.
    PackHeader,
    /// Scaffold run surface.
    RunSurface,
    /// Generation diff-review surface.
    DiffReview,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl FrameworkLaneConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Gallery,
        Self::PackHeader,
        Self::RunSurface,
        Self::DiffReview,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Gallery => "gallery",
            Self::PackHeader => "pack_header",
            Self::RunSurface => "run_surface",
            Self::DiffReview => "diff_review",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One lane-pack row: one framework pack and its lane, version, origin, health,
/// and banner truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkLaneRow {
    /// Opaque stable row id.
    pub row_id: String,
    /// Opaque stable pack id.
    pub pack_id: String,
    /// Header display label for the pack.
    pub pack_label: String,
    /// Opaque stable framework id.
    pub framework_id: String,
    /// Header display label for the framework.
    pub framework_label: String,
    /// Display label for the lane this pack serves.
    pub lane_label: String,
    /// Lane domain this pack serves.
    pub lane_domain_class: LaneDomainClass,
    /// Pinned pack version semver shown in the header.
    pub pack_version_semver: String,
    /// Pinned generator version semver bound to this pack.
    pub generator_version_semver: String,
    /// Short reviewable header summary.
    pub header_summary: String,
    /// Header provenance and trust source.
    pub provenance_class: LanePackProvenanceClass,
    /// Freshness chip state.
    pub freshness_class: LanePackFreshnessClass,
    /// Export-safe freshness/version chip label.
    pub freshness_chip_label: String,
    /// RFC 3339 timestamp the freshness was last verified.
    pub last_verified: String,
    /// Capability banner state.
    pub capability_class: LanePackCapabilityClass,
    /// Short reviewable capability summary.
    pub capability_summary: String,
    /// Support class communicated for this pack.
    pub support_class: LanePackSupportClass,
    /// Authored/generated/runtime-only origin truth for this pack's scaffolding.
    pub origin_truth_class: LanePackOriginTruthClass,
    /// Archetype health state bound to this pack.
    pub archetype_health_class: LaneArchetypeHealthClass,
    /// Short reviewable health summary.
    pub health_summary: String,
    /// Downgrade banner shown for this pack.
    pub downgrade_banner_class: LanePackDowngradeBannerClass,
    /// Opaque known-issue refs disclosed before the pack is offered.
    pub known_issue_refs: Vec<String>,
    /// Whether this pack is admitted to be offered.
    pub admitted_for_offer: bool,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<FrameworkLaneDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<FrameworkLaneConsumerSurface>,
}

impl FrameworkLaneRow {
    /// Whether this row is structurally blocked from being offered.
    pub const fn is_blocked(&self) -> bool {
        self.freshness_class.is_blocking()
            || self.capability_class.is_blocking()
            || self.provenance_class.is_unknown()
            || self.origin_truth_class.is_blocking()
            || self.archetype_health_class.is_blocking()
            || self.downgrade_banner_class.is_hard_block()
    }
}

/// Review block asserting the lane's honesty invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkLaneReview {
    /// The pack header shows provenance and the pinned pack and generator versions.
    pub pack_header_shows_provenance_and_versions: bool,
    /// A freshness chip is shown for every pack.
    pub freshness_chip_shown_for_every_pack: bool,
    /// A capability banner is shown whenever capability is not full.
    pub capability_banner_shown_when_not_full: bool,
    /// The authored/generated/runtime-only origin truth is shown for every pack.
    pub origin_truth_shown_for_every_pack: bool,
    /// The archetype health state is shown for every pack.
    pub archetype_health_shown_for_every_pack: bool,
    /// A downgrade banner is shown whenever a pack is narrowed.
    pub downgrade_banner_shown_when_narrowed: bool,
    /// Bridge or heuristic packs are never presented as exact first-party truth.
    pub bridge_or_heuristic_never_presented_as_first_party: bool,
    /// A stale or yanked pack is never offered as current.
    pub stale_or_yanked_pack_not_offered_as_current: bool,
    /// A provenance-unknown pack is labeled rather than hidden.
    pub provenance_unknown_pack_labeled_not_hidden: bool,
    /// The support class is visible before a pack is offered.
    pub support_class_visible_before_offer: bool,
    /// Known issues are disclosed before a pack is offered.
    pub known_issues_disclosed_before_offer: bool,
    /// No raw pack bodies or URLs cross the export boundary.
    pub no_raw_pack_bodies_or_urls_in_export: bool,
    /// Downgrade narrows the row's claim rather than hiding the row.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkLaneConsumerProjection {
    /// Gallery shows the pack header and lane.
    pub gallery_shows_pack_header_and_lane: bool,
    /// Gallery shows the freshness chip.
    pub gallery_shows_freshness_chip: bool,
    /// Run surface shows the capability banner.
    pub run_surface_shows_capability_banner: bool,
    /// Run surface shows the origin truth.
    pub run_surface_shows_origin_truth: bool,
    /// Diff-review surface shows the downgrade banner.
    pub diff_review_shows_downgrade_banner: bool,
    /// Diagnostics shows the archetype health state.
    pub diagnostics_shows_archetype_health: bool,
    /// CLI / headless shows pack rows.
    pub cli_headless_shows_pack_rows: bool,
    /// Support export shows pack rows.
    pub support_export_shows_pack_rows: bool,
    /// Blocked rows are visibly labeled rather than hidden.
    pub blocked_rows_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkLaneProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected rows.
    pub auto_narrow_on_stale: bool,
}

/// Per-row observation fed to [`FrameworkLanePacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkLaneRowObservation {
    /// Row id the observation applies to.
    pub row_id: String,
    /// True when the pack provenance currently resolves.
    pub provenance_resolved: bool,
    /// True when the pinned pack version is current (not yanked).
    pub pack_version_current: bool,
    /// True when the pinned generator version is current (not yanked).
    pub generator_version_current: bool,
    /// True when the pack freshness is currently fresh.
    pub freshness_fresh: bool,
    /// True when the pack capability currently verifies.
    pub capability_verified: bool,
    /// True when the archetype health currently passes.
    pub archetype_health_ok: bool,
    /// True when the authored/generated/runtime-only origin truth currently verifies.
    pub origin_truth_verified: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`FrameworkLanePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkLanePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Pack rows.
    pub rows: Vec<FrameworkLaneRow>,
    /// Review block.
    pub review: FrameworkLaneReview,
    /// Consumer projection block.
    pub consumer_projection: FrameworkLaneConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: FrameworkLaneProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe richer framework-pack lane catalog packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkLanePacket {
    /// Record kind; must equal [`RICHER_FRAMEWORK_PACK_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`RICHER_FRAMEWORK_PACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Pack rows.
    pub rows: Vec<FrameworkLaneRow>,
    /// Review block.
    pub review: FrameworkLaneReview,
    /// Consumer projection block.
    pub consumer_projection: FrameworkLaneConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: FrameworkLaneProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl FrameworkLanePacket {
    /// Builds a richer framework-pack lane packet from stable-row input.
    pub fn new(input: FrameworkLanePacketInput) -> Self {
        Self {
            record_kind: RICHER_FRAMEWORK_PACK_RECORD_KIND.to_owned(),
            schema_version: RICHER_FRAMEWORK_PACK_SCHEMA_VERSION,
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

    /// Narrows rows whose provenance went unverified, whose pack or generator
    /// version was yanked, whose freshness went stale, whose capability could not
    /// be verified, whose archetype health degraded, whose origin truth could not
    /// be verified, or whose proof or upstream narrowed.
    ///
    /// Unknown provenance is the hardest block: the header, freshness, and
    /// capability are all marked unknown, a provenance-unknown banner is raised,
    /// and the row loses its offer. A yanked pack or generator version narrows
    /// freshness to stale and raises a freshness banner. An unverified capability
    /// narrows the capability to degraded and raises a capability banner. A failed
    /// health check narrows the health to degraded and raises a health banner. An
    /// unverified origin truth narrows the origin to unknown and raises an
    /// origin-truth banner. Stale proof or a narrowed upstream withholds the offer
    /// until evidence refreshes. A raised banner is never lowered. Rows without a
    /// matching observation are left unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[FrameworkLaneRowObservation]) {
        for row in &mut self.rows {
            let Some(observation) = observations.iter().find(|obs| obs.row_id == row.row_id) else {
                continue;
            };

            if !observation.provenance_resolved {
                row.provenance_class = LanePackProvenanceClass::ProvenanceUnknown;
                row.freshness_class = LanePackFreshnessClass::FreshnessUnknown;
                row.capability_class = LanePackCapabilityClass::CapabilityUnknown;
                row.origin_truth_class = LanePackOriginTruthClass::OriginUnknown;
                row.archetype_health_class = LaneArchetypeHealthClass::HealthUnknown;
                row.downgrade_banner_class = LanePackDowngradeBannerClass::ProvenanceUnknownBanner;
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkLaneDowngradeTrigger::ProvenanceUnknown,
                );
                continue;
            }

            if !observation.pack_version_current {
                row.freshness_class = LanePackFreshnessClass::Stale;
                raise_banner(row, LanePackDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkLaneDowngradeTrigger::PackVersionYanked,
                );
            }

            if !observation.generator_version_current {
                if !row.freshness_class.is_blocking() {
                    row.freshness_class = LanePackFreshnessClass::Stale;
                }
                raise_banner(row, LanePackDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkLaneDowngradeTrigger::GeneratorVersionYanked,
                );
            }

            if !observation.freshness_fresh {
                if !row.freshness_class.is_blocking() {
                    row.freshness_class = LanePackFreshnessClass::Stale;
                }
                raise_banner(row, LanePackDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkLaneDowngradeTrigger::FreshnessStale,
                );
            }

            if !observation.capability_verified {
                row.capability_class = LanePackCapabilityClass::CapabilityDegraded;
                raise_banner(row, LanePackDowngradeBannerClass::CapabilityBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkLaneDowngradeTrigger::CapabilityDegraded,
                );
            }

            if !observation.archetype_health_ok {
                if !row.archetype_health_class.is_blocking() {
                    row.archetype_health_class = LaneArchetypeHealthClass::Degraded;
                }
                raise_banner(row, LanePackDowngradeBannerClass::HealthBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkLaneDowngradeTrigger::ArchetypeHealthDegraded,
                );
            }

            if !observation.origin_truth_verified {
                row.origin_truth_class = LanePackOriginTruthClass::OriginUnknown;
                raise_banner(row, LanePackDowngradeBannerClass::OriginTruthBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkLaneDowngradeTrigger::OriginTruthUnverified,
                );
            }

            if (!observation.proof_fresh || observation.upstream_narrowed) && row.admitted_for_offer
            {
                row.admitted_for_offer = false;
                let trigger = if observation.proof_fresh {
                    FrameworkLaneDowngradeTrigger::UpstreamDependencyNarrowed
                } else {
                    FrameworkLaneDowngradeTrigger::ProofStale
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
            }
        }
    }

    /// Validates the richer framework-pack lane invariants.
    pub fn validate(&self) -> Vec<FrameworkLaneViolation> {
        let mut violations = Vec::new();

        if self.record_kind != RICHER_FRAMEWORK_PACK_RECORD_KIND {
            violations.push(FrameworkLaneViolation::WrongRecordKind);
        }
        if self.schema_version != RICHER_FRAMEWORK_PACK_SCHEMA_VERSION {
            violations.push(FrameworkLaneViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(FrameworkLaneViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("richer framework-pack packet serializes"),
        ) {
            violations.push(FrameworkLaneViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("richer framework-pack packet serializes")
    }

    /// Rows currently admitted to be offered.
    pub fn admitted_rows(&self) -> impl Iterator<Item = &FrameworkLaneRow> {
        self.rows.iter().filter(|row| row.admitted_for_offer)
    }

    /// Rows that serve the given lane domain.
    pub fn rows_for_domain(
        &self,
        domain: LaneDomainClass,
    ) -> impl Iterator<Item = &FrameworkLaneRow> {
        self.rows
            .iter()
            .filter(move |row| row.lane_domain_class == domain)
    }

    /// Deterministic Markdown summary for gallery, support, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let admitted = self.admitted_rows().count();
        let mut out = String::new();
        out.push_str(
            "# Richer Framework Packs: Jupyter Adjacency, Terraform/Kubernetes, FastAPI, Nest, Rails, Laravel, and Flutter\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Rows: {} ({} admitted for offer)\n",
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
                "- **{}** `{}` (gen `{}`, {} — {}): {} / {}\n",
                row.pack_label,
                row.pack_version_semver,
                row.generator_version_semver,
                row.lane_label,
                row.lane_domain_class.as_str(),
                row.provenance_class.as_str(),
                row.support_class.as_str()
            ));
            out.push_str(&format!("  - Header: {}\n", row.header_summary));
            out.push_str(&format!(
                "  - Freshness chip: {} ({})\n",
                row.freshness_chip_label,
                row.freshness_class.as_str()
            ));
            out.push_str(&format!(
                "  - Capability: {} | Origin: {} | Health: {}\n",
                row.capability_class.as_str(),
                row.origin_truth_class.as_str(),
                row.archetype_health_class.as_str()
            ));
            out.push_str(&format!(
                "  - Banner: {} | Offered: {}\n",
                row.downgrade_banner_class.as_str(),
                row.admitted_for_offer
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in richer framework-pack export.
#[derive(Debug)]
pub enum FrameworkLaneArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<FrameworkLaneViolation>),
}

impl fmt::Display for FrameworkLaneArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "richer framework-pack export parse failed: {error}"
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
                    "richer framework-pack export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for FrameworkLaneArtifactError {}

/// Validation failures emitted by [`FrameworkLanePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameworkLaneViolation {
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
    /// A non-full-capability row is missing its capability or downgrade banner.
    CapabilityBannerMissing,
    /// A degraded, unknown, or blocked-health row is missing a downgrade banner.
    HealthBannerMissing,
    /// An origin-unknown row is missing a downgrade banner.
    OriginTruthBannerMissing,
    /// A bridge/heuristic row is missing a known issue, banner, or disclosure trigger.
    BridgeBehaviorUndisclosed,
    /// A provenance-unknown row is missing its provenance-unknown banner.
    ProvenanceUnknownBannerMissing,
    /// A stale or unknown-freshness row is missing a downgrade banner.
    FreshnessBannerMissing,
    /// A blocked row is still admitted to be offered.
    BlockedOfferAdmitted,
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

impl FrameworkLaneViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsEmpty => "rows_empty",
            Self::RowIncomplete => "row_incomplete",
            Self::CapabilityBannerMissing => "capability_banner_missing",
            Self::HealthBannerMissing => "health_banner_missing",
            Self::OriginTruthBannerMissing => "origin_truth_banner_missing",
            Self::BridgeBehaviorUndisclosed => "bridge_behavior_undisclosed",
            Self::ProvenanceUnknownBannerMissing => "provenance_unknown_banner_missing",
            Self::FreshnessBannerMissing => "freshness_banner_missing",
            Self::BlockedOfferAdmitted => "blocked_offer_admitted",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in richer framework-pack export.
///
/// This is the first real consumer of the richer framework-pack lane: a gallery,
/// pack header, run, diagnostics, or support-export surface calls it to ingest the
/// canonical packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`FrameworkLaneArtifactError`] when the checked-in support export fails
/// to parse or fails validation.
pub fn current_richer_framework_pack_export(
) -> Result<FrameworkLanePacket, FrameworkLaneArtifactError> {
    let packet: FrameworkLanePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/add_richer_framework_packs_for_jupyter_terraform_kubernetes_fastapi_nestjs_rails_laravel_and_flutter/support_export.json"
    )))
    .map_err(FrameworkLaneArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(FrameworkLaneArtifactError::Validation(violations))
    }
}

/// Canonical review block with every invariant satisfied.
pub fn canonical_review() -> FrameworkLaneReview {
    FrameworkLaneReview {
        pack_header_shows_provenance_and_versions: true,
        freshness_chip_shown_for_every_pack: true,
        capability_banner_shown_when_not_full: true,
        origin_truth_shown_for_every_pack: true,
        archetype_health_shown_for_every_pack: true,
        downgrade_banner_shown_when_narrowed: true,
        bridge_or_heuristic_never_presented_as_first_party: true,
        stale_or_yanked_pack_not_offered_as_current: true,
        provenance_unknown_pack_labeled_not_hidden: true,
        support_class_visible_before_offer: true,
        known_issues_disclosed_before_offer: true,
        no_raw_pack_bodies_or_urls_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting row truth.
pub fn canonical_consumer_projection() -> FrameworkLaneConsumerProjection {
    FrameworkLaneConsumerProjection {
        gallery_shows_pack_header_and_lane: true,
        gallery_shows_freshness_chip: true,
        run_surface_shows_capability_banner: true,
        run_surface_shows_origin_truth: true,
        diff_review_shows_downgrade_banner: true,
        diagnostics_shows_archetype_health: true,
        cli_headless_shows_pack_rows: true,
        support_export_shows_pack_rows: true,
        blocked_rows_labeled_not_hidden: true,
    }
}

/// Canonical source contract refs that every richer framework-pack export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        RICHER_FRAMEWORK_PACK_SCHEMA_REF.to_owned(),
        RICHER_FRAMEWORK_PACK_DOC_REF.to_owned(),
        TEMPLATE_MANIFEST_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF.to_owned(),
        FRAMEWORK_PACK_HEADER_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF.to_owned(),
    ]
}

/// Builds the canonical richer framework-pack lane packet from stable-row truth.
///
/// The rows cover the eight named lanes across the notebook-adjacency,
/// infrastructure, web-API, and mobile domains, spanning the provenance,
/// capability, support, origin-truth, and health spectrum: a disclosed
/// Jupyter-adjacency bridge offered behind a support-class banner, first-party
/// Terraform/Kubernetes/FastAPI/Rails packs offered cleanly, a community Nest pack
/// offered with a partial-capability banner, a held heuristic Laravel pack, and a
/// mirror Flutter pack whose provenance could not be verified and is blocked
/// rather than hidden.
pub fn canonical_richer_framework_pack(
    packet_id: String,
    packet_label: String,
    minted_at: String,
    proof_freshness: FrameworkLaneProofFreshness,
) -> FrameworkLanePacket {
    FrameworkLanePacket::new(FrameworkLanePacketInput {
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
pub fn canonical_rows() -> Vec<FrameworkLaneRow> {
    use FrameworkLaneConsumerSurface as Surface;
    use FrameworkLaneDowngradeTrigger as Trigger;

    vec![
        FrameworkLaneRow {
            row_id: "framework-lane-row:jupyter_adjacency.partner_bridge:2026.06".to_owned(),
            pack_id: "framework-pack:partner.python.jupyter_adjacency:11".to_owned(),
            pack_label: "Jupyter Adjacency Pack".to_owned(),
            framework_id: "framework:python.jupyter".to_owned(),
            framework_label: "Jupyter".to_owned(),
            lane_label: "Notebook adjacency".to_owned(),
            lane_domain_class: LaneDomainClass::NotebookAdjacency,
            pack_version_semver: "1.3.0".to_owned(),
            generator_version_semver: "1.3.0".to_owned(),
            header_summary: "Partner-certified Jupyter adjacency pack; the header shows partner provenance and the pinned pack and generator versions, the freshness chip reads fresh, and a support-class banner discloses that notebook execution is bridged to an adjacent kernel rather than generated as first-party source".to_owned(),
            provenance_class: LanePackProvenanceClass::PartnerCertified,
            freshness_class: LanePackFreshnessClass::Fresh,
            freshness_chip_label: "v1.3.0 · fresh".to_owned(),
            last_verified: "2026-06-06T00:00:00Z".to_owned(),
            capability_class: LanePackCapabilityClass::PartialCapability,
            capability_summary: "Notebook scaffolding and project wiring are generated; kernel execution is bridged to an adjacent runtime and disclosed by the support-class banner rather than presented as native generation".to_owned(),
            support_class: LanePackSupportClass::BridgeBehavior,
            origin_truth_class: LanePackOriginTruthClass::BridgedAdjacent,
            archetype_health_class: LaneArchetypeHealthClass::HealthyUncertified,
            health_summary: "Healthy but uncertified; the adjacency bridge is exercised by smoke checks rather than a certified health-check bundle".to_owned(),
            downgrade_banner_class: LanePackDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:jupyter_adjacency:kernel_execution_bridged".to_owned(),
            ],
            admitted_for_offer: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::BridgeBehaviorDisclosed,
                Trigger::ArchetypeHealthDegraded,
                Trigger::UpstreamDependencyNarrowed,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::RunSurface,
                Surface::DiffReview,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        FrameworkLaneRow {
            row_id: "framework-lane-row:terraform.first_party:2026.06".to_owned(),
            pack_id: "framework-pack:first_party.infra.terraform:12".to_owned(),
            pack_label: "Terraform Provisioning Pack".to_owned(),
            framework_id: "framework:infra.terraform".to_owned(),
            framework_label: "Terraform".to_owned(),
            lane_label: "Infrastructure provisioning".to_owned(),
            lane_domain_class: LaneDomainClass::InfrastructureProvisioning,
            pack_version_semver: "4.1.0".to_owned(),
            generator_version_semver: "4.1.0".to_owned(),
            header_summary: "First-party Terraform provisioning pack; the header shows certified first-party provenance and the pinned pack and generator versions, the freshness chip reads fresh, and the pack generates managed infrastructure modules natively with no downgrade banner".to_owned(),
            provenance_class: LanePackProvenanceClass::FirstParty,
            freshness_class: LanePackFreshnessClass::Fresh,
            freshness_chip_label: "v4.1.0 · fresh".to_owned(),
            last_verified: "2026-06-06T00:00:00Z".to_owned(),
            capability_class: LanePackCapabilityClass::FullCapability,
            capability_summary: "Full first-party capability; modules, variables, and state wiring are generated into a managed zone the pack owns and can regenerate".to_owned(),
            support_class: LanePackSupportClass::OfficiallySupported,
            origin_truth_class: LanePackOriginTruthClass::GeneratedManaged,
            archetype_health_class: LaneArchetypeHealthClass::CertifiedHealthy,
            health_summary: "Certified healthy against the infrastructure archetype health-check bundle".to_owned(),
            downgrade_banner_class: LanePackDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_offer: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProvenanceUnknown,
                Trigger::FreshnessStale,
                Trigger::CapabilityDegraded,
                Trigger::ArchetypeHealthDegraded,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        FrameworkLaneRow {
            row_id: "framework-lane-row:kubernetes.first_party_update:2026.06".to_owned(),
            pack_id: "framework-pack:first_party.infra.kubernetes:13".to_owned(),
            pack_label: "Kubernetes Manifests Pack".to_owned(),
            framework_id: "framework:infra.kubernetes".to_owned(),
            framework_label: "Kubernetes".to_owned(),
            lane_label: "Infrastructure provisioning".to_owned(),
            lane_domain_class: LaneDomainClass::InfrastructureProvisioning,
            pack_version_semver: "3.0.2".to_owned(),
            generator_version_semver: "3.0.2".to_owned(),
            header_summary: "First-party Kubernetes manifests pack; the header shows first-party provenance and the pinned versions, the freshness chip reads update-available because a newer pack version exists, and the pinned version still offers full first-party generation".to_owned(),
            provenance_class: LanePackProvenanceClass::FirstParty,
            freshness_class: LanePackFreshnessClass::UpdateAvailable,
            freshness_chip_label: "v3.0.2 · update available".to_owned(),
            last_verified: "2026-06-04T00:00:00Z".to_owned(),
            capability_class: LanePackCapabilityClass::FullCapability,
            capability_summary: "Full first-party capability; deployments, services, and config manifests are generated into a managed zone the pack owns and can regenerate".to_owned(),
            support_class: LanePackSupportClass::OfficiallySupported,
            origin_truth_class: LanePackOriginTruthClass::GeneratedManaged,
            archetype_health_class: LaneArchetypeHealthClass::CertifiedHealthy,
            health_summary: "Certified healthy against the infrastructure archetype health-check bundle".to_owned(),
            downgrade_banner_class: LanePackDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_offer: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::PackVersionYanked,
                Trigger::FreshnessStale,
                Trigger::CapabilityDegraded,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        FrameworkLaneRow {
            row_id: "framework-lane-row:fastapi.first_party:2026.06".to_owned(),
            pack_id: "framework-pack:first_party.python.fastapi_service:14".to_owned(),
            pack_label: "FastAPI Service Pack".to_owned(),
            framework_id: "framework:python.fastapi".to_owned(),
            framework_label: "FastAPI".to_owned(),
            lane_label: "Web API service".to_owned(),
            lane_domain_class: LaneDomainClass::WebApiService,
            pack_version_semver: "2.7.1".to_owned(),
            generator_version_semver: "2.7.1".to_owned(),
            header_summary: "First-party FastAPI service pack; the header shows certified first-party provenance and the pinned versions, the freshness chip reads fresh, and routes, models, and dependency wiring are generated natively into a managed zone with no downgrade banner".to_owned(),
            provenance_class: LanePackProvenanceClass::FirstParty,
            freshness_class: LanePackFreshnessClass::Fresh,
            freshness_chip_label: "v2.7.1 · fresh".to_owned(),
            last_verified: "2026-06-06T00:00:00Z".to_owned(),
            capability_class: LanePackCapabilityClass::FullCapability,
            capability_summary: "Full first-party capability; routers, schemas, and services are generated into a managed zone the pack owns and can regenerate".to_owned(),
            support_class: LanePackSupportClass::OfficiallySupported,
            origin_truth_class: LanePackOriginTruthClass::GeneratedManaged,
            archetype_health_class: LaneArchetypeHealthClass::CertifiedHealthy,
            health_summary: "Certified healthy against the web-API archetype health-check bundle".to_owned(),
            downgrade_banner_class: LanePackDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_offer: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProvenanceUnknown,
                Trigger::FreshnessStale,
                Trigger::CapabilityDegraded,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        FrameworkLaneRow {
            row_id: "framework-lane-row:nest.community_update:2026.05".to_owned(),
            pack_id: "framework-pack:community.node.nest_service:15".to_owned(),
            pack_label: "Nest Service Pack".to_owned(),
            framework_id: "framework:node.nest".to_owned(),
            framework_label: "Nest".to_owned(),
            lane_label: "Web API service".to_owned(),
            lane_domain_class: LaneDomainClass::WebApiService,
            pack_version_semver: "2.4.0".to_owned(),
            generator_version_semver: "2.3.0".to_owned(),
            header_summary: "Community Nest service pack; the header shows community provenance and the pinned pack and generator versions, the freshness chip reads update-available, and a capability banner discloses that the background-worker generator is partial so the pack is offered with the partial-capability banner rather than as full first-party truth".to_owned(),
            provenance_class: LanePackProvenanceClass::Community,
            freshness_class: LanePackFreshnessClass::UpdateAvailable,
            freshness_chip_label: "v2.4.0 · update available".to_owned(),
            last_verified: "2026-06-01T00:00:00Z".to_owned(),
            capability_class: LanePackCapabilityClass::PartialCapability,
            capability_summary: "Core generation is supported; the background-worker generator is partial and is disclosed by the capability banner instead of being presented as complete".to_owned(),
            support_class: LanePackSupportClass::CommunitySupported,
            origin_truth_class: LanePackOriginTruthClass::GeneratedManaged,
            archetype_health_class: LaneArchetypeHealthClass::HealthyUncertified,
            health_summary: "Healthy but uncertified; the community pack passes smoke checks rather than a certified health-check bundle".to_owned(),
            downgrade_banner_class: LanePackDowngradeBannerClass::CapabilityBanner,
            known_issue_refs: vec![
                "known-issue:node_nest_service:worker_generator_partial".to_owned(),
            ],
            admitted_for_offer: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::FreshnessStale,
                Trigger::CapabilityDegraded,
                Trigger::ArchetypeHealthDegraded,
                Trigger::UpstreamDependencyNarrowed,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::RunSurface,
                Surface::DiffReview,
                Surface::SupportExport,
            ],
        },
        FrameworkLaneRow {
            row_id: "framework-lane-row:rails.community:2026.06".to_owned(),
            pack_id: "framework-pack:community.ruby.rails_service:16".to_owned(),
            pack_label: "Rails Service Pack".to_owned(),
            framework_id: "framework:ruby.rails".to_owned(),
            framework_label: "Rails".to_owned(),
            lane_label: "Web API service".to_owned(),
            lane_domain_class: LaneDomainClass::WebApiService,
            pack_version_semver: "5.2.0".to_owned(),
            generator_version_semver: "5.2.0".to_owned(),
            header_summary: "Community Rails service pack; the header shows community provenance and the pinned versions, the freshness chip reads fresh, and controllers, models, and migrations are generated natively into a managed zone with no downgrade banner".to_owned(),
            provenance_class: LanePackProvenanceClass::Community,
            freshness_class: LanePackFreshnessClass::Fresh,
            freshness_chip_label: "v5.2.0 · fresh".to_owned(),
            last_verified: "2026-06-05T00:00:00Z".to_owned(),
            capability_class: LanePackCapabilityClass::FullCapability,
            capability_summary: "Full capability for the community lane; controllers, models, and migrations are generated into a managed zone the pack owns and can regenerate".to_owned(),
            support_class: LanePackSupportClass::CommunitySupported,
            origin_truth_class: LanePackOriginTruthClass::GeneratedManaged,
            archetype_health_class: LaneArchetypeHealthClass::CertifiedHealthy,
            health_summary: "Certified healthy against the web-API archetype health-check bundle".to_owned(),
            downgrade_banner_class: LanePackDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_offer: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::FreshnessStale,
                Trigger::CapabilityDegraded,
                Trigger::ArchetypeHealthDegraded,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        FrameworkLaneRow {
            row_id: "framework-lane-row:laravel.heuristic_degraded:2026.04".to_owned(),
            pack_id: "framework-pack:community.php.laravel_bridge:17".to_owned(),
            pack_label: "Laravel Bridge Pack".to_owned(),
            framework_id: "framework:php.laravel".to_owned(),
            framework_label: "Laravel".to_owned(),
            lane_label: "Web API service".to_owned(),
            lane_domain_class: LaneDomainClass::WebApiService,
            pack_version_semver: "0.9.0".to_owned(),
            generator_version_semver: "0.9.0".to_owned(),
            header_summary: "Laravel bridge pack that maps some structure through heuristic conventions rather than exact first-party generation; the header marks community provenance, the capability banner reads heuristic, the support-class banner discloses the heuristic mapping and its known issues, the archetype health reads degraded, and the pack is held from being offered as exact truth".to_owned(),
            provenance_class: LanePackProvenanceClass::Community,
            freshness_class: LanePackFreshnessClass::Aging,
            freshness_chip_label: "v0.9.0 · aging".to_owned(),
            last_verified: "2026-04-18T00:00:00Z".to_owned(),
            capability_class: LanePackCapabilityClass::HeuristicCapability,
            capability_summary: "Routes and entities are inferred from naming and layout conventions only; this is heuristic mapping, not exact first-party generation, and is disclosed by the support-class banner".to_owned(),
            support_class: LanePackSupportClass::HeuristicMapping,
            origin_truth_class: LanePackOriginTruthClass::RuntimeObserved,
            archetype_health_class: LaneArchetypeHealthClass::Degraded,
            health_summary: "Health degraded; the heuristic mapping fails part of the web-API archetype health-check bundle and is held behind a health banner".to_owned(),
            downgrade_banner_class: LanePackDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:php_laravel_bridge:heuristic_route_inference".to_owned(),
                "known-issue:php_laravel_bridge:degraded_health_check".to_owned(),
            ],
            admitted_for_offer: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HeuristicMappingDisclosed,
                Trigger::ArchetypeHealthDegraded,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::DiffReview,
                Surface::Diagnostics,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        FrameworkLaneRow {
            row_id: "framework-lane-row:flutter.mirror_unknown:2026.03".to_owned(),
            pack_id: "framework-pack:mirror.dart.flutter_unverified:18".to_owned(),
            pack_label: "Flutter Pack (Mirror, Unverified)".to_owned(),
            framework_id: "framework:dart.flutter".to_owned(),
            framework_label: "Flutter".to_owned(),
            lane_label: "Mobile application".to_owned(),
            lane_domain_class: LaneDomainClass::MobileApp,
            pack_version_semver: "0.0.0".to_owned(),
            generator_version_semver: "0.0.0".to_owned(),
            header_summary: "Flutter mobile pack served from a mirror whose provenance could not be verified; the header is marked provenance-unknown, the freshness chip reads unverified, the capability and origin truth read unknown, the archetype health reads unknown, and the provenance-unknown downgrade banner blocks the pack from being offered rather than hiding it".to_owned(),
            provenance_class: LanePackProvenanceClass::ProvenanceUnknown,
            freshness_class: LanePackFreshnessClass::FreshnessUnknown,
            freshness_chip_label: "v0.0.0 · unverified".to_owned(),
            last_verified: "2026-03-12T00:00:00Z".to_owned(),
            capability_class: LanePackCapabilityClass::CapabilityUnknown,
            capability_summary: "Capability could not be verified for this mirror pack; it is labeled unknown and blocked rather than presented as supported".to_owned(),
            support_class: LanePackSupportClass::SupportUnknown,
            origin_truth_class: LanePackOriginTruthClass::OriginUnknown,
            archetype_health_class: LaneArchetypeHealthClass::HealthUnknown,
            health_summary: "Health could not be determined for this unverified mirror pack; it is labeled unknown and blocked".to_owned(),
            downgrade_banner_class: LanePackDowngradeBannerClass::ProvenanceUnknownBanner,
            known_issue_refs: vec![
                "known-issue:flutter_mirror:provenance_unverified".to_owned(),
            ],
            admitted_for_offer: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProvenanceUnknown,
                Trigger::FreshnessStale,
                Trigger::OriginTruthUnverified,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
    ]
}

fn validate_source_contracts(
    packet: &FrameworkLanePacket,
    violations: &mut Vec<FrameworkLaneViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        RICHER_FRAMEWORK_PACK_SCHEMA_REF,
        RICHER_FRAMEWORK_PACK_DOC_REF,
        TEMPLATE_MANIFEST_CONTRACT_REF,
        TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF,
        FRAMEWORK_PACK_HEADER_CONTRACT_REF,
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(FrameworkLaneViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(packet: &FrameworkLanePacket, violations: &mut Vec<FrameworkLaneViolation>) {
    if packet.rows.is_empty() {
        violations.push(FrameworkLaneViolation::RowsEmpty);
        return;
    }

    for row in &packet.rows {
        if row.row_id.trim().is_empty()
            || row.pack_id.trim().is_empty()
            || row.pack_label.trim().is_empty()
            || row.framework_id.trim().is_empty()
            || row.framework_label.trim().is_empty()
            || row.lane_label.trim().is_empty()
            || row.pack_version_semver.trim().is_empty()
            || row.generator_version_semver.trim().is_empty()
            || row.header_summary.trim().is_empty()
            || row.freshness_chip_label.trim().is_empty()
            || row.last_verified.trim().is_empty()
            || row.capability_summary.trim().is_empty()
            || row.health_summary.trim().is_empty()
        {
            violations.push(FrameworkLaneViolation::RowIncomplete);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(FrameworkLaneViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(FrameworkLaneViolation::ConsumerSurfacesMissing);
        }

        validate_row_banners(row, violations);
    }
}

fn validate_row_banners(row: &FrameworkLaneRow, violations: &mut Vec<FrameworkLaneViolation>) {
    // A non-full-capability pack must show a banner.
    if row.capability_class.requires_banner() && !row.downgrade_banner_class.is_present() {
        violations.push(FrameworkLaneViolation::CapabilityBannerMissing);
    }

    // A degraded, unknown, or blocked-health pack must show a banner.
    if row.archetype_health_class.requires_banner() && !row.downgrade_banner_class.is_present() {
        violations.push(FrameworkLaneViolation::HealthBannerMissing);
    }

    // An origin-unknown pack must show a banner.
    if row.origin_truth_class.is_blocking() && !row.downgrade_banner_class.is_present() {
        violations.push(FrameworkLaneViolation::OriginTruthBannerMissing);
    }

    // Bridge/heuristic packs must disclose a known issue, a banner, and the matching trigger.
    if row.support_class.requires_disclosure() {
        let matching_trigger = match row.support_class {
            LanePackSupportClass::BridgeBehavior => {
                FrameworkLaneDowngradeTrigger::BridgeBehaviorDisclosed
            }
            _ => FrameworkLaneDowngradeTrigger::HeuristicMappingDisclosed,
        };
        if row.known_issue_refs.is_empty()
            || !row.downgrade_banner_class.is_present()
            || !row.downgrade_triggers.contains(&matching_trigger)
        {
            violations.push(FrameworkLaneViolation::BridgeBehaviorUndisclosed);
        }
    }

    // A provenance-unknown pack must carry the provenance-unknown banner.
    if row.provenance_class.is_unknown()
        && row.downgrade_banner_class != LanePackDowngradeBannerClass::ProvenanceUnknownBanner
    {
        violations.push(FrameworkLaneViolation::ProvenanceUnknownBannerMissing);
    }

    // A stale or unknown-freshness pack must show a downgrade banner.
    if row.freshness_class.is_blocking() && !row.downgrade_banner_class.is_present() {
        violations.push(FrameworkLaneViolation::FreshnessBannerMissing);
    }

    // A blocked pack cannot be admitted to be offered.
    if row.is_blocked() && row.admitted_for_offer {
        violations.push(FrameworkLaneViolation::BlockedOfferAdmitted);
    }
}

fn validate_review(packet: &FrameworkLanePacket, violations: &mut Vec<FrameworkLaneViolation>) {
    let review = &packet.review;
    for ok in [
        review.pack_header_shows_provenance_and_versions,
        review.freshness_chip_shown_for_every_pack,
        review.capability_banner_shown_when_not_full,
        review.origin_truth_shown_for_every_pack,
        review.archetype_health_shown_for_every_pack,
        review.downgrade_banner_shown_when_narrowed,
        review.bridge_or_heuristic_never_presented_as_first_party,
        review.stale_or_yanked_pack_not_offered_as_current,
        review.provenance_unknown_pack_labeled_not_hidden,
        review.support_class_visible_before_offer,
        review.known_issues_disclosed_before_offer,
        review.no_raw_pack_bodies_or_urls_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(FrameworkLaneViolation::ReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &FrameworkLanePacket,
    violations: &mut Vec<FrameworkLaneViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.gallery_shows_pack_header_and_lane,
        projection.gallery_shows_freshness_chip,
        projection.run_surface_shows_capability_banner,
        projection.run_surface_shows_origin_truth,
        projection.diff_review_shows_downgrade_banner,
        projection.diagnostics_shows_archetype_health,
        projection.cli_headless_shows_pack_rows,
        projection.support_export_shows_pack_rows,
        projection.blocked_rows_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(FrameworkLaneViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &FrameworkLanePacket,
    violations: &mut Vec<FrameworkLaneViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(FrameworkLaneViolation::ProofFreshnessIncomplete);
    }
}

/// Raises the row's downgrade banner only when none is currently shown, so an
/// already-raised banner is never lowered to a softer cue.
fn raise_banner(row: &mut FrameworkLaneRow, banner: LanePackDowngradeBannerClass) {
    if !row.downgrade_banner_class.is_present() {
        row.downgrade_banner_class = banner;
    }
}

fn push_unique_trigger(
    triggers: &mut Vec<FrameworkLaneDowngradeTrigger>,
    trigger: FrameworkLaneDowngradeTrigger,
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
