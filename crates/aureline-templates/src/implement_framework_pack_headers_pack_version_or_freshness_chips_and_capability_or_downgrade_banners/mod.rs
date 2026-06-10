//! Framework-pack headers, pack version or freshness chips, and capability or downgrade banners.
//!
//! This module locks the canonical, export-safe packet for the framework-pack
//! presentation lane. Each [`FrameworkPackRow`] binds one framework pack to its
//! header provenance, its pinned pack version and freshness chip, its capability
//! banner, its support class, and its downgrade banner — so the gallery, pack
//! header, run, diff-review, diagnostics, and support surfaces project the same
//! truth about where a pack came from, how fresh it is, what it can actually
//! generate, and on what terms it may be offered, instead of presenting heuristic
//! or bridge behavior as exact first-party truth.
//!
//! The packet is metadata only. Raw pack bodies, raw manifests, repository URLs,
//! hostnames, secrets, and user-authored content never cross this boundary; rows
//! carry opaque refs, closed-vocabulary class tokens, short reviewable summaries,
//! and export-safe chip labels. It references the upstream template-manifest and
//! template-registry contracts by ref rather than embedding them.
//!
//! [`FrameworkPackPacket::apply_downgrade_automation`] narrows rows whose
//! provenance went unverified, whose pack version was yanked, whose freshness
//! went stale, whose capability could not be verified, or whose proof or upstream
//! dependency narrowed — withholding the offer and surfacing a downgrade banner
//! rather than hiding the row, so CI or release tooling narrows a stale or
//! underqualified pack before it is offered.
//!
//! The boundary schema is
//! [`schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json`](../../../../schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners.md`](../../../../docs/frameworks/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/`](../../../../fixtures/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`FrameworkPackPacket`].
pub const FRAMEWORK_PACK_RECORD_KIND: &str = "framework_pack_header_and_capability_banner_rows";

/// Schema version for framework-pack packets.
pub const FRAMEWORK_PACK_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const FRAMEWORK_PACK_SCHEMA_REF: &str =
    "schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json";

/// Repo-relative path of the contract doc.
pub const FRAMEWORK_PACK_DOC_REF: &str =
    "docs/frameworks/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners.md";

/// Repo-relative path of the upstream template-manifest contract this packet references.
pub const TEMPLATE_MANIFEST_CONTRACT_REF: &str =
    "schemas/templates/template_manifest_alpha.schema.json";

/// Repo-relative path of the upstream template-registry-entry contract this packet references.
pub const TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF: &str =
    "schemas/templates/template_registry_entry.schema.json";

/// Repo-relative path of the template-registry and scaffold contract doc.
pub const TEMPLATE_REGISTRY_CONTRACT_DOC_REF: &str =
    "docs/templates/template_registry_and_scaffold_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const FRAMEWORK_PACK_FIXTURE_DIR: &str =
    "fixtures/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners";

/// Repo-relative path of the checked support-export artifact.
pub const FRAMEWORK_PACK_ARTIFACT_REF: &str =
    "artifacts/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/support_export.json";

/// Header provenance and trust source for a framework pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackProvenanceClass {
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

impl PackProvenanceClass {
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

/// Freshness chip state for a framework pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackFreshnessClass {
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

impl PackFreshnessClass {
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

/// Capability banner state for a framework pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackCapabilityClass {
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

impl PackCapabilityClass {
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

/// Support class communicated for a pack — keeps bridge/heuristic behavior honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackSupportClass {
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

impl PackSupportClass {
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
    /// Bridge and heuristic packs must never be presented as exact first-party
    /// truth without a known issue, a support-class banner, and the matching
    /// disclosure trigger.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::BridgeBehavior | Self::HeuristicMapping)
    }
}

/// Downgrade banner shown for a pack — the explicit narrowing cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PackDowngradeBannerClass {
    /// No downgrade banner is shown.
    NoBanner,
    /// Freshness banner: the pack is aging, stale, or yanked.
    FreshnessBanner,
    /// Capability banner: the pack is partial, bridged, or degraded.
    CapabilityBanner,
    /// Support-class banner: bridge or heuristic behavior is disclosed.
    SupportClassBanner,
    /// Policy-block banner: the pack is blocked by policy or trust.
    PolicyBlockBanner,
    /// Provenance-unknown banner: the pack's provenance could not be verified.
    ProvenanceUnknownBanner,
}

impl PackDowngradeBannerClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBanner => "no_banner",
            Self::FreshnessBanner => "freshness_banner",
            Self::CapabilityBanner => "capability_banner",
            Self::SupportClassBanner => "support_class_banner",
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

/// Downgrade trigger that can narrow a framework-pack row below its claimed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkPackDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The pack provenance could not be verified.
    ProvenanceUnknown,
    /// The pinned pack version was yanked.
    PackVersionYanked,
    /// The pack freshness went stale.
    FreshnessStale,
    /// The pack capability degraded below its declared class.
    CapabilityDegraded,
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

impl FrameworkPackDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProvenanceUnknown,
        Self::PackVersionYanked,
        Self::FreshnessStale,
        Self::CapabilityDegraded,
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
            Self::FreshnessStale => "freshness_stale",
            Self::CapabilityDegraded => "capability_degraded",
            Self::BridgeBehaviorDisclosed => "bridge_behavior_disclosed",
            Self::HeuristicMappingDisclosed => "heuristic_mapping_disclosed",
            Self::KnownIssueBlocking => "known_issue_blocking",
            Self::ValidationFailed => "validation_failed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a framework-pack row's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FrameworkPackConsumerSurface {
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

impl FrameworkPackConsumerSurface {
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

/// One framework-pack row: one pack and its header, chip, and banner truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackRow {
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
    /// Pinned pack version semver shown in the header.
    pub pack_version_semver: String,
    /// Short reviewable header summary.
    pub header_summary: String,
    /// Header provenance and trust source.
    pub provenance_class: PackProvenanceClass,
    /// Freshness chip state.
    pub freshness_class: PackFreshnessClass,
    /// Export-safe freshness/version chip label.
    pub freshness_chip_label: String,
    /// RFC 3339 timestamp the freshness was last verified.
    pub last_verified: String,
    /// Capability banner state.
    pub capability_class: PackCapabilityClass,
    /// Short reviewable capability summary.
    pub capability_summary: String,
    /// Support class communicated for this pack.
    pub support_class: PackSupportClass,
    /// Downgrade banner shown for this pack.
    pub downgrade_banner_class: PackDowngradeBannerClass,
    /// Opaque known-issue refs disclosed before the pack is offered.
    pub known_issue_refs: Vec<String>,
    /// Whether this pack is admitted to be offered.
    pub admitted_for_offer: bool,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<FrameworkPackDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<FrameworkPackConsumerSurface>,
}

impl FrameworkPackRow {
    /// Whether this row is structurally blocked from being offered.
    pub const fn is_blocked(&self) -> bool {
        self.freshness_class.is_blocking()
            || self.capability_class.is_blocking()
            || self.provenance_class.is_unknown()
            || self.downgrade_banner_class.is_hard_block()
    }
}

/// Review block asserting the lane's honesty invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackReview {
    /// The pack header shows provenance and the pinned pack version.
    pub pack_header_shows_provenance_and_version: bool,
    /// A freshness chip is shown for every pack.
    pub freshness_chip_shown_for_every_pack: bool,
    /// A capability banner is shown whenever capability is not full.
    pub capability_banner_shown_when_not_full: bool,
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
pub struct FrameworkPackConsumerProjection {
    /// Gallery shows the pack header.
    pub gallery_shows_pack_header: bool,
    /// Gallery shows the freshness chip.
    pub gallery_shows_freshness_chip: bool,
    /// Run surface shows the capability banner.
    pub run_surface_shows_capability_banner: bool,
    /// Diff-review surface shows the downgrade banner.
    pub diff_review_shows_downgrade_banner: bool,
    /// CLI / headless shows pack rows.
    pub cli_headless_shows_pack_rows: bool,
    /// Support export shows pack rows.
    pub support_export_shows_pack_rows: bool,
    /// Diagnostics shows freshness and capability state.
    pub diagnostics_shows_freshness_and_capability_state: bool,
    /// Blocked rows are visibly labeled rather than hidden.
    pub blocked_rows_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected rows.
    pub auto_narrow_on_stale: bool,
}

/// Per-row observation fed to [`FrameworkPackPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkPackRowObservation {
    /// Row id the observation applies to.
    pub row_id: String,
    /// True when the pack provenance currently resolves.
    pub provenance_resolved: bool,
    /// True when the pinned pack version is current (not yanked).
    pub pack_version_current: bool,
    /// True when the pack freshness is currently fresh.
    pub freshness_fresh: bool,
    /// True when the pack capability currently verifies.
    pub capability_verified: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`FrameworkPackPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkPackPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Pack rows.
    pub rows: Vec<FrameworkPackRow>,
    /// Review block.
    pub review: FrameworkPackReview,
    /// Consumer projection block.
    pub consumer_projection: FrameworkPackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: FrameworkPackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe framework-pack header, freshness chip, and banner packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FrameworkPackPacket {
    /// Record kind; must equal [`FRAMEWORK_PACK_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`FRAMEWORK_PACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Pack rows.
    pub rows: Vec<FrameworkPackRow>,
    /// Review block.
    pub review: FrameworkPackReview,
    /// Consumer projection block.
    pub consumer_projection: FrameworkPackConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: FrameworkPackProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl FrameworkPackPacket {
    /// Builds a framework-pack packet from stable-row input.
    pub fn new(input: FrameworkPackPacketInput) -> Self {
        Self {
            record_kind: FRAMEWORK_PACK_RECORD_KIND.to_owned(),
            schema_version: FRAMEWORK_PACK_SCHEMA_VERSION,
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

    /// Narrows rows whose provenance went unverified, whose pack version was
    /// yanked, whose freshness went stale, whose capability could not be
    /// verified, or whose proof or upstream narrowed.
    ///
    /// Unknown provenance is the hardest block: the header, freshness, and
    /// capability are all marked unknown, a provenance-unknown banner is raised,
    /// and the row loses its offer. A yanked or stale pack narrows freshness to
    /// stale and raises a freshness banner. An unverified capability narrows the
    /// capability to degraded and raises a capability banner. Stale proof or a
    /// narrowed upstream withholds the offer until evidence refreshes. A raised
    /// banner is never lowered. Rows without a matching observation are left
    /// unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[FrameworkPackRowObservation]) {
        for row in &mut self.rows {
            let Some(observation) = observations.iter().find(|obs| obs.row_id == row.row_id) else {
                continue;
            };

            if !observation.provenance_resolved {
                row.provenance_class = PackProvenanceClass::ProvenanceUnknown;
                row.freshness_class = PackFreshnessClass::FreshnessUnknown;
                row.capability_class = PackCapabilityClass::CapabilityUnknown;
                row.downgrade_banner_class = PackDowngradeBannerClass::ProvenanceUnknownBanner;
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkPackDowngradeTrigger::ProvenanceUnknown,
                );
                continue;
            }

            if !observation.pack_version_current {
                row.freshness_class = PackFreshnessClass::Stale;
                raise_banner(row, PackDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkPackDowngradeTrigger::PackVersionYanked,
                );
            }

            if !observation.freshness_fresh {
                if !row.freshness_class.is_blocking() {
                    row.freshness_class = PackFreshnessClass::Stale;
                }
                raise_banner(row, PackDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkPackDowngradeTrigger::FreshnessStale,
                );
            }

            if !observation.capability_verified {
                row.capability_class = PackCapabilityClass::CapabilityDegraded;
                raise_banner(row, PackDowngradeBannerClass::CapabilityBanner);
                row.admitted_for_offer = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    FrameworkPackDowngradeTrigger::CapabilityDegraded,
                );
            }

            if (!observation.proof_fresh || observation.upstream_narrowed) && row.admitted_for_offer
            {
                row.admitted_for_offer = false;
                let trigger = if observation.proof_fresh {
                    FrameworkPackDowngradeTrigger::UpstreamDependencyNarrowed
                } else {
                    FrameworkPackDowngradeTrigger::ProofStale
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
            }
        }
    }

    /// Validates the framework-pack invariants.
    pub fn validate(&self) -> Vec<FrameworkPackViolation> {
        let mut violations = Vec::new();

        if self.record_kind != FRAMEWORK_PACK_RECORD_KIND {
            violations.push(FrameworkPackViolation::WrongRecordKind);
        }
        if self.schema_version != FRAMEWORK_PACK_SCHEMA_VERSION {
            violations.push(FrameworkPackViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(FrameworkPackViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("framework-pack packet serializes"),
        ) {
            violations.push(FrameworkPackViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("framework-pack packet serializes")
    }

    /// Rows currently admitted to be offered.
    pub fn admitted_rows(&self) -> impl Iterator<Item = &FrameworkPackRow> {
        self.rows.iter().filter(|row| row.admitted_for_offer)
    }

    /// Deterministic Markdown summary for gallery, support, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let admitted = self.admitted_rows().count();
        let mut out = String::new();
        out.push_str(
            "# Framework-Pack Headers, Pack Version/Freshness Chips, and Capability/Downgrade Banners\n\n",
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
                "- **{}** `{}` ({}): {} / {}\n",
                row.pack_label,
                row.pack_version_semver,
                row.framework_label,
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
                "  - Capability: {} (banner: {})\n",
                row.capability_class.as_str(),
                row.downgrade_banner_class.as_str()
            ));
            out.push_str(&format!("  - Offered: {}\n", row.admitted_for_offer));
        }
        out
    }
}

/// Errors emitted when reading the checked-in framework-pack export.
#[derive(Debug)]
pub enum FrameworkPackArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<FrameworkPackViolation>),
}

impl fmt::Display for FrameworkPackArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "framework-pack export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "framework-pack export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for FrameworkPackArtifactError {}

/// Validation failures emitted by [`FrameworkPackPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameworkPackViolation {
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

impl FrameworkPackViolation {
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

/// Reads and validates the checked-in framework-pack export.
///
/// This is the first real consumer of the framework-pack lane: a gallery, pack
/// header, run, diagnostics, or support-export surface calls it to ingest the
/// canonical packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`FrameworkPackArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_framework_pack_export() -> Result<FrameworkPackPacket, FrameworkPackArtifactError> {
    let packet: FrameworkPackPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/implement_framework_pack_headers_pack_version_or_freshness_chips_and_capability_or_downgrade_banners/support_export.json"
    )))
    .map_err(FrameworkPackArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(FrameworkPackArtifactError::Validation(violations))
    }
}

/// Canonical review block with every invariant satisfied.
pub fn canonical_review() -> FrameworkPackReview {
    FrameworkPackReview {
        pack_header_shows_provenance_and_version: true,
        freshness_chip_shown_for_every_pack: true,
        capability_banner_shown_when_not_full: true,
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
pub fn canonical_consumer_projection() -> FrameworkPackConsumerProjection {
    FrameworkPackConsumerProjection {
        gallery_shows_pack_header: true,
        gallery_shows_freshness_chip: true,
        run_surface_shows_capability_banner: true,
        diff_review_shows_downgrade_banner: true,
        cli_headless_shows_pack_rows: true,
        support_export_shows_pack_rows: true,
        diagnostics_shows_freshness_and_capability_state: true,
        blocked_rows_labeled_not_hidden: true,
    }
}

/// Canonical source contract refs that every framework-pack export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        FRAMEWORK_PACK_SCHEMA_REF.to_owned(),
        FRAMEWORK_PACK_DOC_REF.to_owned(),
        TEMPLATE_MANIFEST_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF.to_owned(),
    ]
}

/// Builds the canonical framework-pack packet from stable-row truth.
///
/// The rows mirror the checked-in support export and cover the provenance,
/// freshness, and capability spectrum: a clean first-party pack offered with no
/// banner, a community pack offered with an update-available chip and a partial
/// capability banner, a held heuristic bridge pack that discloses its support
/// class, and a mirror pack whose provenance is unknown and is blocked rather
/// than hidden.
pub fn canonical_framework_pack(
    packet_id: String,
    packet_label: String,
    minted_at: String,
    proof_freshness: FrameworkPackProofFreshness,
) -> FrameworkPackPacket {
    FrameworkPackPacket::new(FrameworkPackPacketInput {
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
pub fn canonical_rows() -> Vec<FrameworkPackRow> {
    use FrameworkPackConsumerSurface as Surface;
    use FrameworkPackDowngradeTrigger as Trigger;

    vec![
        FrameworkPackRow {
            row_id: "framework-pack-row:rust_axum.first_party:2026.05".to_owned(),
            pack_id: "framework-pack:first_party.rust.axum_service:01".to_owned(),
            pack_label: "Rust Axum Service Pack".to_owned(),
            framework_id: "framework:rust.axum".to_owned(),
            framework_label: "Axum".to_owned(),
            pack_version_semver: "3.2.1".to_owned(),
            header_summary: "First-party Axum service pack; the header shows certified first-party provenance and the pinned pack version, the freshness chip reads fresh against the last verification, and the pack offers full first-party generation with no downgrade banner".to_owned(),
            provenance_class: PackProvenanceClass::FirstParty,
            freshness_class: PackFreshnessClass::Fresh,
            freshness_chip_label: "v3.2.1 · fresh".to_owned(),
            last_verified: "2026-06-05T00:00:00Z".to_owned(),
            capability_class: PackCapabilityClass::FullCapability,
            capability_summary: "Full first-party project-model capability; routes, services, and entities are generated natively rather than bridged or inferred".to_owned(),
            support_class: PackSupportClass::OfficiallySupported,
            downgrade_banner_class: PackDowngradeBannerClass::NoBanner,
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
        FrameworkPackRow {
            row_id: "framework-pack-row:node_nest.community_update:2026.05".to_owned(),
            pack_id: "framework-pack:community.node.nest_service:02".to_owned(),
            pack_label: "Nest Service Pack".to_owned(),
            framework_id: "framework:node.nest".to_owned(),
            framework_label: "Nest".to_owned(),
            pack_version_semver: "2.4.0".to_owned(),
            header_summary: "Community Nest pack; the header shows community provenance and the current pack version, the freshness chip reads update-available, and a capability banner discloses that one optional generator is partial so the pack is offered with the partial-capability banner rather than as full first-party truth".to_owned(),
            provenance_class: PackProvenanceClass::Community,
            freshness_class: PackFreshnessClass::UpdateAvailable,
            freshness_chip_label: "v2.4.0 · update available".to_owned(),
            last_verified: "2026-06-01T00:00:00Z".to_owned(),
            capability_class: PackCapabilityClass::PartialCapability,
            capability_summary: "Core generation is supported; the background-worker generator is partial and is disclosed by the capability banner instead of being presented as complete".to_owned(),
            support_class: PackSupportClass::CommunitySupported,
            downgrade_banner_class: PackDowngradeBannerClass::CapabilityBanner,
            known_issue_refs: vec![
                "known-issue:node_nest_service:worker_generator_partial".to_owned(),
            ],
            admitted_for_offer: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::FreshnessStale,
                Trigger::CapabilityDegraded,
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
        FrameworkPackRow {
            row_id: "framework-pack-row:py_flask.bridge_heuristic:2026.04".to_owned(),
            pack_id: "framework-pack:community.python.flask_bridge:04".to_owned(),
            pack_label: "Flask Bridge Pack".to_owned(),
            framework_id: "framework:python.flask".to_owned(),
            framework_label: "Flask".to_owned(),
            pack_version_semver: "0.8.0".to_owned(),
            header_summary: "Flask bridge pack that maps some structure through heuristic conventions rather than exact first-party generation; the header marks bridged provenance, the capability banner reads heuristic, the support-class banner discloses the bridge behavior and its known issues, and the pack is held from being offered as exact truth".to_owned(),
            provenance_class: PackProvenanceClass::BridgedFromOtherFramework,
            freshness_class: PackFreshnessClass::Aging,
            freshness_chip_label: "v0.8.0 · aging".to_owned(),
            last_verified: "2026-04-20T00:00:00Z".to_owned(),
            capability_class: PackCapabilityClass::HeuristicCapability,
            capability_summary: "Routes and services are inferred from naming and layout conventions only; this is heuristic mapping, not exact first-party generation, and is disclosed by the support-class banner".to_owned(),
            support_class: PackSupportClass::HeuristicMapping,
            downgrade_banner_class: PackDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:python_flask_bridge:heuristic_route_inference".to_owned(),
                "known-issue:python_flask_bridge:partial_entity_mapping".to_owned(),
            ],
            admitted_for_offer: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HeuristicMappingDisclosed,
                Trigger::BridgeBehaviorDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::Gallery,
                Surface::PackHeader,
                Surface::DiffReview,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        FrameworkPackRow {
            row_id: "framework-pack-row:mirror.unknown_provenance:2026.03".to_owned(),
            pack_id: "framework-pack:mirror.unverified_origin:00".to_owned(),
            pack_label: "Mirror Pack (Unverified)".to_owned(),
            framework_id: "framework:unknown.mirror".to_owned(),
            framework_label: "Unverified Mirror".to_owned(),
            pack_version_semver: "0.0.0".to_owned(),
            header_summary: "Mirror pack whose provenance could not be verified; the header is marked provenance-unknown, the freshness chip reads unverified, the capability banner reads unknown, and the provenance-unknown downgrade banner blocks the pack from being offered rather than hiding it".to_owned(),
            provenance_class: PackProvenanceClass::ProvenanceUnknown,
            freshness_class: PackFreshnessClass::FreshnessUnknown,
            freshness_chip_label: "v0.0.0 · unverified".to_owned(),
            last_verified: "2026-03-10T00:00:00Z".to_owned(),
            capability_class: PackCapabilityClass::CapabilityUnknown,
            capability_summary: "Capability could not be verified for this mirror pack; it is labeled unknown and blocked rather than presented as supported".to_owned(),
            support_class: PackSupportClass::SupportUnknown,
            downgrade_banner_class: PackDowngradeBannerClass::ProvenanceUnknownBanner,
            known_issue_refs: vec![
                "known-issue:mirror_pack:provenance_unverified".to_owned(),
            ],
            admitted_for_offer: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProvenanceUnknown,
                Trigger::FreshnessStale,
                Trigger::CapabilityDegraded,
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
    packet: &FrameworkPackPacket,
    violations: &mut Vec<FrameworkPackViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        FRAMEWORK_PACK_SCHEMA_REF,
        FRAMEWORK_PACK_DOC_REF,
        TEMPLATE_MANIFEST_CONTRACT_REF,
        TEMPLATE_REGISTRY_ENTRY_CONTRACT_REF,
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(FrameworkPackViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(packet: &FrameworkPackPacket, violations: &mut Vec<FrameworkPackViolation>) {
    if packet.rows.is_empty() {
        violations.push(FrameworkPackViolation::RowsEmpty);
        return;
    }

    for row in &packet.rows {
        if row.row_id.trim().is_empty()
            || row.pack_id.trim().is_empty()
            || row.pack_label.trim().is_empty()
            || row.framework_id.trim().is_empty()
            || row.framework_label.trim().is_empty()
            || row.pack_version_semver.trim().is_empty()
            || row.header_summary.trim().is_empty()
            || row.freshness_chip_label.trim().is_empty()
            || row.last_verified.trim().is_empty()
            || row.capability_summary.trim().is_empty()
        {
            violations.push(FrameworkPackViolation::RowIncomplete);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(FrameworkPackViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(FrameworkPackViolation::ConsumerSurfacesMissing);
        }

        validate_row_banners(row, violations);
    }
}

fn validate_row_banners(row: &FrameworkPackRow, violations: &mut Vec<FrameworkPackViolation>) {
    // A non-full-capability pack must show a banner.
    if row.capability_class.requires_banner() && !row.downgrade_banner_class.is_present() {
        violations.push(FrameworkPackViolation::CapabilityBannerMissing);
    }

    // Bridge/heuristic packs must disclose a known issue, a banner, and the matching trigger.
    if row.support_class.requires_disclosure() {
        let matching_trigger = match row.support_class {
            PackSupportClass::BridgeBehavior => {
                FrameworkPackDowngradeTrigger::BridgeBehaviorDisclosed
            }
            _ => FrameworkPackDowngradeTrigger::HeuristicMappingDisclosed,
        };
        if row.known_issue_refs.is_empty()
            || !row.downgrade_banner_class.is_present()
            || !row.downgrade_triggers.contains(&matching_trigger)
        {
            violations.push(FrameworkPackViolation::BridgeBehaviorUndisclosed);
        }
    }

    // A provenance-unknown pack must carry the provenance-unknown banner.
    if row.provenance_class.is_unknown()
        && row.downgrade_banner_class != PackDowngradeBannerClass::ProvenanceUnknownBanner
    {
        violations.push(FrameworkPackViolation::ProvenanceUnknownBannerMissing);
    }

    // A stale or unknown-freshness pack must show a downgrade banner.
    if row.freshness_class.is_blocking() && !row.downgrade_banner_class.is_present() {
        violations.push(FrameworkPackViolation::FreshnessBannerMissing);
    }

    // A blocked pack cannot be admitted to be offered.
    if row.is_blocked() && row.admitted_for_offer {
        violations.push(FrameworkPackViolation::BlockedOfferAdmitted);
    }
}

fn validate_review(packet: &FrameworkPackPacket, violations: &mut Vec<FrameworkPackViolation>) {
    let review = &packet.review;
    for ok in [
        review.pack_header_shows_provenance_and_version,
        review.freshness_chip_shown_for_every_pack,
        review.capability_banner_shown_when_not_full,
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
            violations.push(FrameworkPackViolation::ReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &FrameworkPackPacket,
    violations: &mut Vec<FrameworkPackViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.gallery_shows_pack_header,
        projection.gallery_shows_freshness_chip,
        projection.run_surface_shows_capability_banner,
        projection.diff_review_shows_downgrade_banner,
        projection.cli_headless_shows_pack_rows,
        projection.support_export_shows_pack_rows,
        projection.diagnostics_shows_freshness_and_capability_state,
        projection.blocked_rows_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(FrameworkPackViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &FrameworkPackPacket,
    violations: &mut Vec<FrameworkPackViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(FrameworkPackViolation::ProofFreshnessIncomplete);
    }
}

/// Raises the row's downgrade banner only when none is currently shown, so an
/// already-raised banner is never lowered to a softer cue.
fn raise_banner(row: &mut FrameworkPackRow, banner: PackDowngradeBannerClass) {
    if !row.downgrade_banner_class.is_present() {
        row.downgrade_banner_class = banner;
    }
}

fn push_unique_trigger(
    triggers: &mut Vec<FrameworkPackDowngradeTrigger>,
    trigger: FrameworkPackDowngradeTrigger,
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
