//! Certified-archetype health-check bundles, stack diagnostics, and fix-forward
//! guidance.
//!
//! This module locks the canonical, export-safe packet for the certified-archetype
//! health-check bundle lane. Each [`ArchetypeHealthBundleRow`] binds one
//! health-check bundle run — a health check of a project created from a certified,
//! provisional, community, or uncertified archetype — to its archetype
//! certification class, its pinned health-check bundle version, its overall health
//! state, the worst stack-diagnostic severity it observed, whether fix-forward
//! guidance is available and how, how scan-fresh it is, the support class on which
//! it may be presented, and its downgrade banner. The archetype gallery,
//! health-check panel, stack-diagnostics surface, fix-forward guidance, run,
//! diagnostics, and support surfaces project the same truth about whether a
//! bundle's health verdict may be trusted — and on what terms — instead of
//! presenting an uncertified, heuristic, or bridged check as exact first-party
//! truth.
//!
//! The packet is metadata only. Raw source bodies, raw diagnostic logs, generated
//! file contents, repository URLs, hostnames, secrets, and user-authored content
//! never cross this boundary; rows carry opaque refs, closed-vocabulary class
//! tokens, short reviewable summaries, structural locators, and export-safe chip
//! labels. It references the upstream template-manifest, framework-pack, and
//! generator-run contracts by ref rather than embedding them, and reuses the prior
//! support-class and downgrade vocabulary instead of inventing parallel terms.
//!
//! [`ArchetypeHealthBundlePacket::apply_downgrade_automation`] narrows bundles whose
//! certification could not be verified, whose health could not be determined, whose
//! stack diagnostics could not be produced, whose fix-forward guidance went
//! unavailable, whose scan record went stale, or whose proof or upstream dependency
//! narrowed — withholding a confident verdict and surfacing a downgrade banner
//! rather than hiding the bundle, so CI or release tooling narrows a stale or
//! underqualified archetype health bundle before it is published.
//!
//! The boundary schema is
//! [`schemas/templates/ship-certified-archetype-health-check-bundles-stack-diagnostics-and-fix-forward-guidance.schema.json`](../../../../schemas/templates/ship-certified-archetype-health-check-bundles-stack-diagnostics-and-fix-forward-guidance.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance.md`](../../../../docs/frameworks/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/`](../../../../fixtures/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ArchetypeHealthBundlePacket`].
pub const ARCHETYPE_HEALTH_RECORD_KIND: &str = "certified_archetype_health_check_bundle_rows";

/// Schema version for archetype health-bundle packets.
pub const ARCHETYPE_HEALTH_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ARCHETYPE_HEALTH_SCHEMA_REF: &str =
    "schemas/templates/ship-certified-archetype-health-check-bundles-stack-diagnostics-and-fix-forward-guidance.schema.json";

/// Repo-relative path of the contract doc.
pub const ARCHETYPE_HEALTH_DOC_REF: &str =
    "docs/frameworks/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance.md";

/// Repo-relative path of the upstream template-manifest contract this packet references.
pub const TEMPLATE_MANIFEST_CONTRACT_REF: &str =
    "schemas/templates/template_manifest_alpha.schema.json";

/// Repo-relative path of the upstream framework-pack contract this packet references.
pub const FRAMEWORK_PACK_CONTRACT_REF: &str =
    "schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json";

/// Repo-relative path of the upstream generator-run contract this packet references.
pub const GENERATOR_RUN_CONTRACT_REF: &str =
    "schemas/templates/implement-framework-generators-or-codemods-with-preview-diff-rollback-and-execution-context-reuse.schema.json";

/// Repo-relative path of the template-registry and scaffold contract doc.
pub const TEMPLATE_REGISTRY_CONTRACT_DOC_REF: &str =
    "docs/templates/template_registry_and_scaffold_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const ARCHETYPE_HEALTH_FIXTURE_DIR: &str =
    "fixtures/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance";

/// Repo-relative path of the checked support-export artifact.
pub const ARCHETYPE_HEALTH_ARTIFACT_REF: &str =
    "artifacts/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/support_export.json";

/// Which kind of project archetype a health-check bundle covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeKind {
    /// A backend service archetype.
    ServiceArchetype,
    /// A web-application archetype.
    WebAppArchetype,
    /// A full-stack (frontend + backend) archetype.
    FullStackArchetype,
    /// A command-line application archetype.
    CliArchetype,
    /// A library or package archetype.
    LibraryArchetype,
}

impl ArchetypeKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServiceArchetype => "service_archetype",
            Self::WebAppArchetype => "web_app_archetype",
            Self::FullStackArchetype => "full_stack_archetype",
            Self::CliArchetype => "cli_archetype",
            Self::LibraryArchetype => "library_archetype",
        }
    }
}

/// Certification class of the archetype a bundle covers — keeps uncertified honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeCertificationClass {
    /// A fully certified first-party archetype.
    CertifiedArchetype,
    /// A provisional archetype on a certification path but not yet certified.
    ProvisionalArchetype,
    /// A community archetype, not first-party certified.
    CommunityArchetype,
    /// An explicitly uncertified archetype.
    UncertifiedArchetype,
    /// Certification class could not be determined.
    CertificationUnknown,
}

impl ArchetypeCertificationClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CertifiedArchetype => "certified_archetype",
            Self::ProvisionalArchetype => "provisional_archetype",
            Self::CommunityArchetype => "community_archetype",
            Self::UncertifiedArchetype => "uncertified_archetype",
            Self::CertificationUnknown => "certification_unknown",
        }
    }

    /// Whether this class must be disclosed because it is not full certification.
    ///
    /// A provisional, community, uncertified, or unknown archetype must never be
    /// presented as a certified first-party verdict without a present banner and
    /// the matching disclosure trigger.
    pub const fn requires_disclosure(self) -> bool {
        !matches!(self, Self::CertifiedArchetype)
    }
}

/// Overall health state a bundle reports for a project.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HealthCheckClass {
    /// Every check passed cleanly.
    Healthy,
    /// Checks passed, but advisories were raised.
    HealthyWithAdvisories,
    /// One or more checks degraded; the project still builds.
    Degraded,
    /// One or more checks failed; the project is broken.
    Failing,
    /// The health verdict could not be determined; the bundle must be blocked.
    HealthUnknown,
}

impl HealthCheckClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::HealthyWithAdvisories => "healthy_with_advisories",
            Self::Degraded => "degraded",
            Self::Failing => "failing",
            Self::HealthUnknown => "health_unknown",
        }
    }

    /// Whether the health verdict could not be determined.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::HealthUnknown)
    }
}

/// Worst stack-diagnostic severity a bundle observed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StackDiagnosticClass {
    /// No stack diagnostics were raised.
    NoDiagnostics,
    /// Only advisory diagnostics were raised.
    Advisory,
    /// At least one warning diagnostic was raised.
    Warning,
    /// At least one error diagnostic was raised.
    Error,
    /// At least one blocking diagnostic was raised.
    Blocking,
    /// Stack diagnostics could not be produced; the bundle must be blocked.
    DiagnosticsUnavailable,
}

impl StackDiagnosticClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDiagnostics => "no_diagnostics",
            Self::Advisory => "advisory",
            Self::Warning => "warning",
            Self::Error => "error",
            Self::Blocking => "blocking",
            Self::DiagnosticsUnavailable => "diagnostics_unavailable",
        }
    }

    /// Whether stack diagnostics could not be produced.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::DiagnosticsUnavailable)
    }

    /// Whether this severity is grounded in concrete findings that need refs.
    pub const fn requires_findings(self) -> bool {
        matches!(self, Self::Warning | Self::Error | Self::Blocking)
    }
}

/// Whether and how fix-forward guidance is available for a bundle's findings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixForwardClass {
    /// No fix is needed; the project is healthy.
    NoFixNeeded,
    /// A fix can be applied automatically and previewed before apply.
    FixAutomatic,
    /// Manual fix-forward steps are provided.
    FixGuided,
    /// Only advisory guidance is provided; no concrete steps.
    FixAdvisoryOnly,
    /// No fix-forward guidance could be produced; a labeled gap.
    FixUnavailable,
}

impl FixForwardClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoFixNeeded => "no_fix_needed",
            Self::FixAutomatic => "fix_automatic",
            Self::FixGuided => "fix_guided",
            Self::FixAdvisoryOnly => "fix_advisory_only",
            Self::FixUnavailable => "fix_unavailable",
        }
    }

    /// Whether this state carries concrete fix-forward guidance that needs refs.
    pub const fn carries_guidance(self) -> bool {
        matches!(
            self,
            Self::FixAutomatic | Self::FixGuided | Self::FixAdvisoryOnly
        )
    }

    /// Whether no fix-forward guidance could be produced.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::FixUnavailable)
    }
}

/// Bundle-scan freshness state for a health-check bundle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeHealthFreshnessClass {
    /// Verified fresh against the last scan record.
    Fresh,
    /// A newer scan is available but the current record is still serviceable.
    RescanAvailable,
    /// Aging; a re-scan is recommended.
    Aging,
    /// Stale; the scan record is past its freshness window.
    Stale,
    /// Freshness could not be determined.
    FreshnessUnknown,
}

impl ArchetypeHealthFreshnessClass {
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

    /// Whether this freshness state blocks presenting the bundle as current.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Stale | Self::FreshnessUnknown)
    }
}

/// Support class on which a health-check bundle may be presented.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeHealthSupportClass {
    /// Exactly modeled first-party health-check bundle.
    ExactlyModeled,
    /// Experimental; may change without notice.
    Experimental,
    /// Bridge behavior: bridged from another health-check tool rather than modeled natively.
    BridgeBehavior,
    /// Heuristic mapping; inferred rather than exactly modeled.
    HeuristicMapping,
    /// Explicitly unsupported.
    Unsupported,
    /// Support class unknown.
    SupportUnknown,
}

impl ArchetypeHealthSupportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactlyModeled => "exactly_modeled",
            Self::Experimental => "experimental",
            Self::BridgeBehavior => "bridge_behavior",
            Self::HeuristicMapping => "heuristic_mapping",
            Self::Unsupported => "unsupported",
            Self::SupportUnknown => "support_unknown",
        }
    }

    /// Whether this class is bridge or heuristic behavior that must be disclosed.
    ///
    /// Bridge and heuristic bundles must never be presented as exact first-party
    /// truth without a known issue, a support-class banner, and the matching
    /// disclosure trigger.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::BridgeBehavior | Self::HeuristicMapping)
    }
}

/// Downgrade banner shown for a health-check bundle — the explicit narrowing cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeHealthDowngradeBannerClass {
    /// No downgrade banner is shown.
    NoBanner,
    /// Freshness banner: the scan record is aging, stale, or unverifiable.
    FreshnessBanner,
    /// Health banner: the health verdict could not be determined.
    HealthUnknownBanner,
    /// Diagnostics banner: stack diagnostics could not be produced.
    DiagnosticsUnavailableBanner,
    /// Fix banner: no fix-forward guidance could be produced.
    FixUnavailableBanner,
    /// Certification banner: a non-certified archetype is disclosed.
    CertificationBanner,
    /// Support-class banner: bridge or heuristic behavior is disclosed.
    SupportClassBanner,
    /// Policy-block banner: the bundle is blocked by policy or trust.
    PolicyBlockBanner,
}

impl ArchetypeHealthDowngradeBannerClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBanner => "no_banner",
            Self::FreshnessBanner => "freshness_banner",
            Self::HealthUnknownBanner => "health_unknown_banner",
            Self::DiagnosticsUnavailableBanner => "diagnostics_unavailable_banner",
            Self::FixUnavailableBanner => "fix_unavailable_banner",
            Self::CertificationBanner => "certification_banner",
            Self::SupportClassBanner => "support_class_banner",
            Self::PolicyBlockBanner => "policy_block_banner",
        }
    }

    /// Whether a banner is shown at all.
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoBanner)
    }

    /// Whether this banner hard-blocks a confident verdict (not merely a soft cue).
    pub const fn is_hard_block(self) -> bool {
        matches!(
            self,
            Self::HealthUnknownBanner
                | Self::DiagnosticsUnavailableBanner
                | Self::PolicyBlockBanner
        )
    }
}

/// Downgrade trigger that can narrow a health-check bundle below its claimed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeHealthDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The archetype certification could not be verified.
    CertificationUnverified,
    /// The health verdict could not be determined.
    HealthUndeterminable,
    /// Stack diagnostics could not be produced.
    DiagnosticsUnavailable,
    /// Fix-forward guidance could not be produced.
    FixGuidanceUnavailable,
    /// The pinned health-check bundle version could not be verified.
    BundleVersionUnverified,
    /// The scan record that produced the row went stale.
    BundleRecordStale,
    /// A non-certified archetype is disclosed and held from certified-truth claims.
    UncertifiedArchetypeDisclosed,
    /// Heuristic mapping is disclosed and held from exact-truth claims.
    HeuristicMappingDisclosed,
    /// Bridge behavior is disclosed and held from exact-truth claims.
    BridgeBehaviorDisclosed,
    /// A blocking known issue applies.
    KnownIssueBlocking,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl ArchetypeHealthDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::CertificationUnverified,
        Self::HealthUndeterminable,
        Self::DiagnosticsUnavailable,
        Self::FixGuidanceUnavailable,
        Self::BundleVersionUnverified,
        Self::BundleRecordStale,
        Self::UncertifiedArchetypeDisclosed,
        Self::HeuristicMappingDisclosed,
        Self::BridgeBehaviorDisclosed,
        Self::KnownIssueBlocking,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::CertificationUnverified => "certification_unverified",
            Self::HealthUndeterminable => "health_undeterminable",
            Self::DiagnosticsUnavailable => "diagnostics_unavailable",
            Self::FixGuidanceUnavailable => "fix_guidance_unavailable",
            Self::BundleVersionUnverified => "bundle_version_unverified",
            Self::BundleRecordStale => "bundle_record_stale",
            Self::UncertifiedArchetypeDisclosed => "uncertified_archetype_disclosed",
            Self::HeuristicMappingDisclosed => "heuristic_mapping_disclosed",
            Self::BridgeBehaviorDisclosed => "bridge_behavior_disclosed",
            Self::KnownIssueBlocking => "known_issue_blocking",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a health-check bundle's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArchetypeHealthConsumerSurface {
    /// Certified-archetype gallery.
    ArchetypeGallery,
    /// Health-check panel.
    HealthCheckPanel,
    /// Stack-diagnostics surface.
    StackDiagnostics,
    /// Fix-forward guidance surface.
    FixForwardGuidance,
    /// Scaffold or generator run surface.
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

impl ArchetypeHealthConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ArchetypeGallery,
        Self::HealthCheckPanel,
        Self::StackDiagnostics,
        Self::FixForwardGuidance,
        Self::RunSurface,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArchetypeGallery => "archetype_gallery",
            Self::HealthCheckPanel => "health_check_panel",
            Self::StackDiagnostics => "stack_diagnostics",
            Self::FixForwardGuidance => "fix_forward_guidance",
            Self::RunSurface => "run_surface",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One health-bundle row: a certified-archetype health-check bundle and its
/// certification, health, stack-diagnostic, fix-forward, and banner truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchetypeHealthBundleRow {
    /// Opaque stable row id.
    pub row_id: String,
    /// Which kind of archetype this bundle covers.
    pub archetype_kind: ArchetypeKind,
    /// Opaque stable archetype id.
    pub archetype_id: String,
    /// Display label for the archetype.
    pub archetype_label: String,
    /// Structural locator for the archetype definition.
    pub archetype_locator: String,
    /// Pinned archetype version — provenance always disclosed.
    pub archetype_version: String,
    /// Certification class of the archetype.
    pub certification_class: ArchetypeCertificationClass,
    /// Opaque stable health-check bundle id.
    pub bundle_id: String,
    /// Pinned health-check bundle version — provenance always disclosed.
    pub bundle_version: String,
    /// Opaque stable app / project id the bundle ran against.
    pub app_id: String,
    /// Opaque framework-pack ref this archetype belongs to; a sentinel otherwise.
    pub framework_pack_ref: String,
    /// Overall health state the bundle reports.
    pub health_check_class: HealthCheckClass,
    /// Short reviewable health summary.
    pub health_summary: String,
    /// Worst stack-diagnostic severity the bundle observed.
    pub stack_diagnostic_class: StackDiagnosticClass,
    /// Short reviewable stack-diagnostic summary.
    pub diagnostic_summary: String,
    /// Export-safe diagnostic-stat chip label.
    pub diagnostic_stat_label: String,
    /// Opaque diagnostic refs that ground the findings.
    pub diagnostic_refs: Vec<String>,
    /// Whether and how fix-forward guidance is available.
    pub fix_forward_class: FixForwardClass,
    /// Short reviewable fix-forward summary.
    pub fix_forward_summary: String,
    /// Opaque fix-forward refs that ground the remediation guidance.
    pub fix_forward_refs: Vec<String>,
    /// Bundle-scan freshness state.
    pub freshness_class: ArchetypeHealthFreshnessClass,
    /// Export-safe freshness/scan chip label.
    pub freshness_chip_label: String,
    /// RFC 3339 timestamp the bundle last checked.
    pub last_checked: String,
    /// Support class on which the bundle may be presented.
    pub support_class: ArchetypeHealthSupportClass,
    /// Downgrade banner shown for this bundle.
    pub downgrade_banner_class: ArchetypeHealthDowngradeBannerClass,
    /// Opaque known-issue refs disclosed before the bundle is presented.
    pub known_issue_refs: Vec<String>,
    /// Whether this bundle is admitted to be presented as a confident certified verdict.
    pub admitted_for_display: bool,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<ArchetypeHealthDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<ArchetypeHealthConsumerSurface>,
}

impl ArchetypeHealthBundleRow {
    /// Whether this row is structurally blocked from a confident verdict.
    pub const fn is_blocked(&self) -> bool {
        self.freshness_class.is_blocking()
            || self.health_check_class.is_unknown()
            || self.stack_diagnostic_class.is_unavailable()
            || self.downgrade_banner_class.is_hard_block()
    }
}

/// Review block asserting the lane's honesty invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchetypeHealthReview {
    /// A certification class is disclosed for every bundle.
    pub certification_disclosed_for_every_bundle: bool,
    /// A health state is shown for every bundle.
    pub health_state_shown_for_every_bundle: bool,
    /// A stack-diagnostic state is shown for every bundle.
    pub stack_diagnostics_shown_for_every_bundle: bool,
    /// Fix-forward guidance is shown whenever it is available.
    pub fix_forward_guidance_shown_when_available: bool,
    /// An unknown health verdict blocks any confident verdict.
    pub health_unknown_blocks_confident_verdict: bool,
    /// Unavailable diagnostics block any confident verdict.
    pub diagnostics_unavailable_blocks_confident_verdict: bool,
    /// A non-certified archetype is never presented as certified.
    pub uncertified_archetype_never_presented_as_certified: bool,
    /// A heuristic or bridged bundle is never presented as exact truth.
    pub heuristic_or_bridge_never_presented_as_exact_truth: bool,
    /// An unavailable fix-forward guidance is labeled rather than silently hidden.
    pub fix_guidance_unavailable_labeled_not_hidden: bool,
    /// A stale scan record is never presented as current.
    pub stale_bundle_not_presented_as_current: bool,
    /// The support class is visible before a bundle is presented.
    pub support_class_visible_before_display: bool,
    /// Known issues are disclosed before a bundle is presented.
    pub known_issues_disclosed_before_display: bool,
    /// No raw source bodies, diagnostic logs, or URLs cross the export boundary.
    pub no_raw_source_bodies_or_urls_in_export: bool,
    /// Downgrade narrows the bundle's claim rather than hiding the bundle.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
    /// The certification class is visible before a bundle is presented.
    pub certification_visible_before_display: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchetypeHealthConsumerProjection {
    /// Archetype gallery shows the certification class.
    pub gallery_shows_certification: bool,
    /// Health-check panel shows the health state.
    pub health_check_panel_shows_health_state: bool,
    /// Stack-diagnostics surface shows the diagnostics.
    pub stack_diagnostics_shows_diagnostics: bool,
    /// Fix-forward guidance surface shows the remediation guidance.
    pub fix_forward_guidance_shows_remediation: bool,
    /// Run surface shows the health state.
    pub run_surface_shows_health_state: bool,
    /// CLI / headless shows bundle rows.
    pub cli_headless_shows_bundle_rows: bool,
    /// Support export shows bundle rows.
    pub support_export_shows_bundle_rows: bool,
    /// Diagnostics shows health, diagnostic, and fix state.
    pub diagnostics_shows_health_diagnostics_fix_state: bool,
    /// Blocked bundles are visibly labeled rather than hidden.
    pub blocked_bundles_labeled_not_hidden: bool,
    /// Uncertified bundles are visibly labeled rather than hidden.
    pub uncertified_bundles_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchetypeHealthProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected rows.
    pub auto_narrow_on_stale: bool,
}

/// Per-row observation fed to [`ArchetypeHealthBundlePacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchetypeHealthRowObservation {
    /// Row id the observation applies to.
    pub row_id: String,
    /// True when the archetype certification is currently verified.
    pub certification_verified: bool,
    /// True when the health verdict could be determined.
    pub health_determinable: bool,
    /// True when stack diagnostics are currently available.
    pub diagnostics_available: bool,
    /// True when fix-forward guidance is currently available.
    pub fix_guidance_available: bool,
    /// True when the scan record is currently fresh.
    pub bundle_fresh: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`ArchetypeHealthBundlePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArchetypeHealthBundlePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Health-bundle rows.
    pub rows: Vec<ArchetypeHealthBundleRow>,
    /// Review block.
    pub review: ArchetypeHealthReview,
    /// Consumer projection block.
    pub consumer_projection: ArchetypeHealthConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ArchetypeHealthProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe health-bundle packet with certification, health, stack-diagnostic,
/// and fix-forward truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArchetypeHealthBundlePacket {
    /// Record kind; must equal [`ARCHETYPE_HEALTH_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ARCHETYPE_HEALTH_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Health-bundle rows.
    pub rows: Vec<ArchetypeHealthBundleRow>,
    /// Review block.
    pub review: ArchetypeHealthReview,
    /// Consumer projection block.
    pub consumer_projection: ArchetypeHealthConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ArchetypeHealthProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ArchetypeHealthBundlePacket {
    /// Builds a health-bundle packet from stable-row input.
    pub fn new(input: ArchetypeHealthBundlePacketInput) -> Self {
        Self {
            record_kind: ARCHETYPE_HEALTH_RECORD_KIND.to_owned(),
            schema_version: ARCHETYPE_HEALTH_SCHEMA_VERSION,
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

    /// Narrows bundles whose certification could not be verified, whose health
    /// could not be determined, whose stack diagnostics could not be produced,
    /// whose fix-forward guidance went unavailable, whose scan record went stale,
    /// or whose proof or upstream narrowed.
    ///
    /// An undeterminable health verdict is the hardest block: the health is marked
    /// unknown, the diagnostics are marked unavailable, the fix-forward guidance is
    /// marked unavailable, the health-unknown banner is raised, and the bundle loses
    /// its confident verdict. Unavailable diagnostics mark the diagnostics
    /// unavailable, raise the diagnostics-unavailable banner, and withdraw display.
    /// An unverified certification narrows the certification to unknown, raises the
    /// certification banner, and withdraws display. A lost fix-forward guidance
    /// narrows the fix state to unavailable and raises a fix banner without
    /// withdrawing display, because a missing fix-forward path is honest, not a
    /// block on the health verdict. A stale scan record narrows freshness to stale
    /// and raises a freshness banner. Stale proof or a narrowed upstream withholds
    /// display until evidence refreshes. A raised banner is never lowered. Rows
    /// without a matching observation are left unchanged.
    pub fn apply_downgrade_automation(&mut self, observations: &[ArchetypeHealthRowObservation]) {
        for row in &mut self.rows {
            let Some(observation) = observations.iter().find(|obs| obs.row_id == row.row_id) else {
                continue;
            };

            if !observation.health_determinable {
                row.health_check_class = HealthCheckClass::HealthUnknown;
                row.stack_diagnostic_class = StackDiagnosticClass::DiagnosticsUnavailable;
                row.fix_forward_class = FixForwardClass::FixUnavailable;
                row.downgrade_banner_class =
                    ArchetypeHealthDowngradeBannerClass::HealthUnknownBanner;
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ArchetypeHealthDowngradeTrigger::HealthUndeterminable,
                );
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ArchetypeHealthDowngradeTrigger::DiagnosticsUnavailable,
                );
                continue;
            }

            if !observation.diagnostics_available && !row.stack_diagnostic_class.is_unavailable() {
                row.stack_diagnostic_class = StackDiagnosticClass::DiagnosticsUnavailable;
                raise_banner(
                    row,
                    ArchetypeHealthDowngradeBannerClass::DiagnosticsUnavailableBanner,
                );
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ArchetypeHealthDowngradeTrigger::DiagnosticsUnavailable,
                );
            }

            if !observation.certification_verified
                && row.certification_class != ArchetypeCertificationClass::CertificationUnknown
            {
                row.certification_class = ArchetypeCertificationClass::CertificationUnknown;
                raise_banner(
                    row,
                    ArchetypeHealthDowngradeBannerClass::CertificationBanner,
                );
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ArchetypeHealthDowngradeTrigger::CertificationUnverified,
                );
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ArchetypeHealthDowngradeTrigger::UncertifiedArchetypeDisclosed,
                );
            }

            if !observation.fix_guidance_available && row.fix_forward_class.carries_guidance() {
                row.fix_forward_class = FixForwardClass::FixUnavailable;
                raise_banner(
                    row,
                    ArchetypeHealthDowngradeBannerClass::FixUnavailableBanner,
                );
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ArchetypeHealthDowngradeTrigger::FixGuidanceUnavailable,
                );
            }

            if !observation.bundle_fresh {
                if !row.freshness_class.is_blocking() {
                    row.freshness_class = ArchetypeHealthFreshnessClass::Stale;
                }
                raise_banner(row, ArchetypeHealthDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    ArchetypeHealthDowngradeTrigger::BundleRecordStale,
                );
            }

            if (!observation.proof_fresh || observation.upstream_narrowed)
                && row.admitted_for_display
            {
                row.admitted_for_display = false;
                let trigger = if observation.proof_fresh {
                    ArchetypeHealthDowngradeTrigger::UpstreamDependencyNarrowed
                } else {
                    ArchetypeHealthDowngradeTrigger::ProofStale
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
            }
        }
    }

    /// Validates the health-bundle invariants.
    pub fn validate(&self) -> Vec<ArchetypeHealthViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ARCHETYPE_HEALTH_RECORD_KIND {
            violations.push(ArchetypeHealthViolation::WrongRecordKind);
        }
        if self.schema_version != ARCHETYPE_HEALTH_SCHEMA_VERSION {
            violations.push(ArchetypeHealthViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ArchetypeHealthViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("health-bundle packet serializes"),
        ) {
            violations.push(ArchetypeHealthViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("health-bundle packet serializes")
    }

    /// Rows currently admitted to be presented as a confident certified verdict.
    pub fn admitted_rows(&self) -> impl Iterator<Item = &ArchetypeHealthBundleRow> {
        self.rows.iter().filter(|row| row.admitted_for_display)
    }

    /// Deterministic Markdown summary for diagnostics, support, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let admitted = self.admitted_rows().count();
        let mut out = String::new();
        out.push_str(
            "# Certified-Archetype Health-Check Bundles, Stack Diagnostics, and Fix-Forward Guidance\n\n",
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
                "- **{}** `{}` ({}) v{} bundle v{}: {} / {}\n",
                row.archetype_label,
                row.archetype_locator,
                row.archetype_kind.as_str(),
                row.archetype_version,
                row.bundle_version,
                row.certification_class.as_str(),
                row.support_class.as_str()
            ));
            out.push_str(&format!(
                "  - Health: {} ({})\n",
                row.health_summary,
                row.health_check_class.as_str()
            ));
            out.push_str(&format!(
                "  - Diagnostics: {} ({}) [{}]\n",
                row.diagnostic_summary,
                row.stack_diagnostic_class.as_str(),
                row.diagnostic_stat_label
            ));
            out.push_str(&format!(
                "  - Fix-forward: {} ({})\n",
                row.fix_forward_summary,
                row.fix_forward_class.as_str()
            ));
            out.push_str(&format!(
                "  - Freshness chip: {} ({})\n",
                row.freshness_chip_label,
                row.freshness_class.as_str()
            ));
            out.push_str(&format!(
                "  - Banner: {}\n",
                row.downgrade_banner_class.as_str()
            ));
            out.push_str(&format!("  - Offered: {}\n", row.admitted_for_display));
        }
        out
    }
}

/// Errors emitted when reading the checked-in health-bundle export.
#[derive(Debug)]
pub enum ArchetypeHealthArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ArchetypeHealthViolation>),
}

impl fmt::Display for ArchetypeHealthArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "health-bundle export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "health-bundle export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ArchetypeHealthArtifactError {}

/// Validation failures emitted by [`ArchetypeHealthBundlePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArchetypeHealthViolation {
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
    /// A health-unknown row is missing its health-unknown banner.
    HealthUnknownBannerMissing,
    /// A diagnostics-unavailable row is missing a downgrade banner.
    DiagnosticsUnavailableBannerMissing,
    /// A fix-unavailable row is missing a downgrade banner.
    FixUnavailableBannerMissing,
    /// A grounded stack-diagnostic severity carries no diagnostic refs.
    DiagnosticRefsMissing,
    /// A row carrying fix-forward guidance carries no fix-forward refs.
    FixForwardRefsMissing,
    /// A non-certified row is missing a banner or the disclosure trigger.
    CertificationUndisclosed,
    /// A bridge/heuristic row is missing a known issue, banner, or disclosure trigger.
    SupportClassUndisclosed,
    /// A stale or unknown-freshness row is missing a downgrade banner.
    FreshnessBannerMissing,
    /// A blocked row is still admitted for a confident verdict.
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

impl ArchetypeHealthViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsEmpty => "rows_empty",
            Self::RowIncomplete => "row_incomplete",
            Self::HealthUnknownBannerMissing => "health_unknown_banner_missing",
            Self::DiagnosticsUnavailableBannerMissing => "diagnostics_unavailable_banner_missing",
            Self::FixUnavailableBannerMissing => "fix_unavailable_banner_missing",
            Self::DiagnosticRefsMissing => "diagnostic_refs_missing",
            Self::FixForwardRefsMissing => "fix_forward_refs_missing",
            Self::CertificationUndisclosed => "certification_undisclosed",
            Self::SupportClassUndisclosed => "support_class_undisclosed",
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

/// Reads and validates the checked-in health-bundle export.
///
/// This is the first real consumer of the health-bundle lane: an archetype
/// gallery, health-check panel, stack-diagnostics, fix-forward guidance, run,
/// diagnostics, or support-export surface calls it to ingest the canonical packet
/// rather than cloning status text.
///
/// # Errors
///
/// Returns [`ArchetypeHealthArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_archetype_health_export(
) -> Result<ArchetypeHealthBundlePacket, ArchetypeHealthArtifactError> {
    let packet: ArchetypeHealthBundlePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/ship_certified_archetype_health_check_bundles_stack_diagnostics_and_fix_forward_guidance/support_export.json"
    )))
    .map_err(ArchetypeHealthArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ArchetypeHealthArtifactError::Validation(violations))
    }
}

/// Canonical review block with every invariant satisfied.
pub fn canonical_review() -> ArchetypeHealthReview {
    ArchetypeHealthReview {
        certification_disclosed_for_every_bundle: true,
        health_state_shown_for_every_bundle: true,
        stack_diagnostics_shown_for_every_bundle: true,
        fix_forward_guidance_shown_when_available: true,
        health_unknown_blocks_confident_verdict: true,
        diagnostics_unavailable_blocks_confident_verdict: true,
        uncertified_archetype_never_presented_as_certified: true,
        heuristic_or_bridge_never_presented_as_exact_truth: true,
        fix_guidance_unavailable_labeled_not_hidden: true,
        stale_bundle_not_presented_as_current: true,
        support_class_visible_before_display: true,
        known_issues_disclosed_before_display: true,
        no_raw_source_bodies_or_urls_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
        certification_visible_before_display: true,
    }
}

/// Canonical consumer projection block with every surface projecting health truth.
pub fn canonical_consumer_projection() -> ArchetypeHealthConsumerProjection {
    ArchetypeHealthConsumerProjection {
        gallery_shows_certification: true,
        health_check_panel_shows_health_state: true,
        stack_diagnostics_shows_diagnostics: true,
        fix_forward_guidance_shows_remediation: true,
        run_surface_shows_health_state: true,
        cli_headless_shows_bundle_rows: true,
        support_export_shows_bundle_rows: true,
        diagnostics_shows_health_diagnostics_fix_state: true,
        blocked_bundles_labeled_not_hidden: true,
        uncertified_bundles_labeled_not_hidden: true,
    }
}

/// Canonical source contract refs that every health-bundle export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        ARCHETYPE_HEALTH_SCHEMA_REF.to_owned(),
        ARCHETYPE_HEALTH_DOC_REF.to_owned(),
        TEMPLATE_MANIFEST_CONTRACT_REF.to_owned(),
        FRAMEWORK_PACK_CONTRACT_REF.to_owned(),
        GENERATOR_RUN_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF.to_owned(),
    ]
}

/// Builds the canonical health-bundle packet from stable-row truth.
///
/// The rows mirror the checked-in support export and cover the certification,
/// health, stack-diagnostic, and fix-forward spectrum: a certified service
/// archetype shown healthy and active with no banner; a certified web-app
/// archetype shown healthy-with-advisories and active with guided fix-forward
/// steps; a certified full-stack archetype whose heuristic diagnostics are held
/// behind a support-class banner while degraded; a provisional CLI archetype shown
/// failing with an automatic fix-forward path but held behind a certification
/// banner; an uncertified library archetype whose health could not be determined
/// and is blocked; and a community service archetype bridged from an external
/// health-check tool and held from exact-truth claims.
pub fn canonical_archetype_health_bundles(
    packet_id: String,
    packet_label: String,
    minted_at: String,
    proof_freshness: ArchetypeHealthProofFreshness,
) -> ArchetypeHealthBundlePacket {
    ArchetypeHealthBundlePacket::new(ArchetypeHealthBundlePacketInput {
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
pub fn canonical_rows() -> Vec<ArchetypeHealthBundleRow> {
    use ArchetypeHealthConsumerSurface as Surface;
    use ArchetypeHealthDowngradeTrigger as Trigger;

    vec![
        ArchetypeHealthBundleRow {
            row_id: "archetype-health-row:rust.axum.service.certified.healthy:2026.06".to_owned(),
            archetype_kind: ArchetypeKind::ServiceArchetype,
            archetype_id: "archetype:rust.axum.service".to_owned(),
            archetype_label: "Rust Axum service".to_owned(),
            archetype_locator: "archetype:framework_pack/rust.axum/service".to_owned(),
            archetype_version: "1.8.0".to_owned(),
            certification_class: ArchetypeCertificationClass::CertifiedArchetype,
            bundle_id: "health-bundle:rust.axum.service".to_owned(),
            bundle_version: "2.3.0".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            health_check_class: HealthCheckClass::Healthy,
            health_summary: "Every certified health check passed: build, toolchain, dependencies, and config are clean".to_owned(),
            stack_diagnostic_class: StackDiagnosticClass::NoDiagnostics,
            diagnostic_summary: "No stack diagnostics were raised across the certified bundle".to_owned(),
            diagnostic_stat_label: "8 checks · 0 findings".to_owned(),
            diagnostic_refs: vec![],
            fix_forward_class: FixForwardClass::NoFixNeeded,
            fix_forward_summary: "No fix is needed; the project is healthy".to_owned(),
            fix_forward_refs: vec![],
            freshness_class: ArchetypeHealthFreshnessClass::Fresh,
            freshness_chip_label: "checked · fresh".to_owned(),
            last_checked: "2026-06-08T00:00:00Z".to_owned(),
            support_class: ArchetypeHealthSupportClass::ExactlyModeled,
            downgrade_banner_class: ArchetypeHealthDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HealthUndeterminable,
                Trigger::DiagnosticsUnavailable,
                Trigger::BundleRecordStale,
            ],
            consumer_surfaces: vec![
                Surface::ArchetypeGallery,
                Surface::HealthCheckPanel,
                Surface::StackDiagnostics,
                Surface::FixForwardGuidance,
                Surface::SupportExport,
            ],
        },
        ArchetypeHealthBundleRow {
            row_id: "archetype-health-row:ts.next.webapp.certified.advisories:2026.06".to_owned(),
            archetype_kind: ArchetypeKind::WebAppArchetype,
            archetype_id: "archetype:ts.next.webapp".to_owned(),
            archetype_label: "TypeScript Next.js web app".to_owned(),
            archetype_locator: "archetype:framework_pack/ts.next/webapp".to_owned(),
            archetype_version: "3.1.0".to_owned(),
            certification_class: ArchetypeCertificationClass::CertifiedArchetype,
            bundle_id: "health-bundle:ts.next.webapp".to_owned(),
            bundle_version: "2.3.0".to_owned(),
            app_id: "app:ts.next.sample_webapp".to_owned(),
            framework_pack_ref: "framework-pack:ts.next@3.1.0".to_owned(),
            health_check_class: HealthCheckClass::HealthyWithAdvisories,
            health_summary: "All checks passed, but two advisories recommend pinning a dev dependency and enabling a lint rule".to_owned(),
            stack_diagnostic_class: StackDiagnosticClass::Advisory,
            diagnostic_summary: "Two advisory diagnostics were raised; none block the build".to_owned(),
            diagnostic_stat_label: "10 checks · 2 advisories".to_owned(),
            diagnostic_refs: vec![
                "diagnostic:ts.next.webapp/dev_dependency_unpinned".to_owned(),
                "diagnostic:ts.next.webapp/lint_rule_disabled".to_owned(),
            ],
            fix_forward_class: FixForwardClass::FixGuided,
            fix_forward_summary: "Guided fix-forward steps pin the dependency and enable the lint rule".to_owned(),
            fix_forward_refs: vec![
                "fix-forward:ts.next.webapp/pin_dependency".to_owned(),
                "fix-forward:ts.next.webapp/enable_lint_rule".to_owned(),
            ],
            freshness_class: ArchetypeHealthFreshnessClass::Fresh,
            freshness_chip_label: "checked · fresh".to_owned(),
            last_checked: "2026-06-08T00:00:00Z".to_owned(),
            support_class: ArchetypeHealthSupportClass::ExactlyModeled,
            downgrade_banner_class: ArchetypeHealthDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HealthUndeterminable,
                Trigger::FixGuidanceUnavailable,
                Trigger::BundleRecordStale,
            ],
            consumer_surfaces: vec![
                Surface::ArchetypeGallery,
                Surface::HealthCheckPanel,
                Surface::StackDiagnostics,
                Surface::FixForwardGuidance,
                Surface::SupportExport,
            ],
        },
        ArchetypeHealthBundleRow {
            row_id: "archetype-health-row:py.fastapi.fullstack.heuristic.degraded:2026.05".to_owned(),
            archetype_kind: ArchetypeKind::FullStackArchetype,
            archetype_id: "archetype:py.fastapi.fullstack".to_owned(),
            archetype_label: "Python FastAPI full-stack (heuristic checks)".to_owned(),
            archetype_locator: "archetype:framework_pack/py.fastapi/fullstack".to_owned(),
            archetype_version: "2.0.0".to_owned(),
            certification_class: ArchetypeCertificationClass::CertifiedArchetype,
            bundle_id: "health-bundle:py.fastapi.fullstack".to_owned(),
            bundle_version: "2.1.0".to_owned(),
            app_id: "app:py.fastapi.sample_fullstack".to_owned(),
            framework_pack_ref: "framework-pack:py.fastapi@2.0.0".to_owned(),
            health_check_class: HealthCheckClass::Degraded,
            health_summary: "A dependency-resolution check degraded; some findings are inferred rather than exactly modeled".to_owned(),
            stack_diagnostic_class: StackDiagnosticClass::Warning,
            diagnostic_summary: "One warning diagnostic about a transitive dependency was inferred heuristically".to_owned(),
            diagnostic_stat_label: "9 checks · 1 warning".to_owned(),
            diagnostic_refs: vec![
                "diagnostic:py.fastapi.fullstack/transitive_dependency_drift".to_owned(),
            ],
            fix_forward_class: FixForwardClass::FixAdvisoryOnly,
            fix_forward_summary: "Only advisory fix-forward guidance is offered because the finding is heuristic".to_owned(),
            fix_forward_refs: vec![
                "fix-forward:py.fastapi.fullstack/review_transitive_dependency".to_owned(),
            ],
            freshness_class: ArchetypeHealthFreshnessClass::Aging,
            freshness_chip_label: "checked · aging".to_owned(),
            last_checked: "2026-05-22T00:00:00Z".to_owned(),
            support_class: ArchetypeHealthSupportClass::HeuristicMapping,
            downgrade_banner_class: ArchetypeHealthDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:archetype_health:heuristic_dependency_diagnostics".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HeuristicMappingDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::ArchetypeGallery,
                Surface::HealthCheckPanel,
                Surface::StackDiagnostics,
                Surface::FixForwardGuidance,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        ArchetypeHealthBundleRow {
            row_id: "archetype-health-row:go.cli.provisional.failing:2026.06".to_owned(),
            archetype_kind: ArchetypeKind::CliArchetype,
            archetype_id: "archetype:go.cobra.cli".to_owned(),
            archetype_label: "Go Cobra CLI (provisional)".to_owned(),
            archetype_locator: "archetype:framework_pack/go.cobra/cli".to_owned(),
            archetype_version: "0.9.0".to_owned(),
            certification_class: ArchetypeCertificationClass::ProvisionalArchetype,
            bundle_id: "health-bundle:go.cobra.cli".to_owned(),
            bundle_version: "1.0.0".to_owned(),
            app_id: "app:go.cobra.sample_cli".to_owned(),
            framework_pack_ref: "framework-pack:go.cobra@0.9.0".to_owned(),
            health_check_class: HealthCheckClass::Failing,
            health_summary: "The build check failed against the pinned toolchain; the project does not compile".to_owned(),
            stack_diagnostic_class: StackDiagnosticClass::Error,
            diagnostic_summary: "One error diagnostic reports a missing module entry in the manifest".to_owned(),
            diagnostic_stat_label: "6 checks · 1 error".to_owned(),
            diagnostic_refs: vec![
                "diagnostic:go.cobra.cli/missing_module_entry".to_owned(),
            ],
            fix_forward_class: FixForwardClass::FixAutomatic,
            fix_forward_summary: "An automatic fix-forward adds the missing module entry; the change is previewed before apply".to_owned(),
            fix_forward_refs: vec![
                "fix-forward:go.cobra.cli/add_module_entry".to_owned(),
            ],
            freshness_class: ArchetypeHealthFreshnessClass::Fresh,
            freshness_chip_label: "checked · fresh".to_owned(),
            last_checked: "2026-06-07T00:00:00Z".to_owned(),
            support_class: ArchetypeHealthSupportClass::SupportUnknown,
            downgrade_banner_class: ArchetypeHealthDowngradeBannerClass::CertificationBanner,
            known_issue_refs: vec![
                "known-issue:archetype_health:provisional_certification".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::UncertifiedArchetypeDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::ArchetypeGallery,
                Surface::HealthCheckPanel,
                Surface::StackDiagnostics,
                Surface::FixForwardGuidance,
                Surface::RunSurface,
                Surface::SupportExport,
            ],
        },
        ArchetypeHealthBundleRow {
            row_id: "archetype-health-row:rust.lib.uncertified.health_unknown.blocked:2026.04".to_owned(),
            archetype_kind: ArchetypeKind::LibraryArchetype,
            archetype_id: "archetype:rust.lib.uncertified".to_owned(),
            archetype_label: "Rust library (uncertified)".to_owned(),
            archetype_locator: "archetype:framework_pack/rust.lib/uncertified".to_owned(),
            archetype_version: "0.2.0".to_owned(),
            certification_class: ArchetypeCertificationClass::UncertifiedArchetype,
            bundle_id: "health-bundle:rust.lib.uncertified".to_owned(),
            bundle_version: "0.1.0".to_owned(),
            app_id: "app:rust.lib.sample_uncertified".to_owned(),
            framework_pack_ref: "framework-pack:rust.lib@0.2.0".to_owned(),
            health_check_class: HealthCheckClass::HealthUnknown,
            health_summary: "The health verdict could not be determined; the bundle did not run to completion".to_owned(),
            stack_diagnostic_class: StackDiagnosticClass::DiagnosticsUnavailable,
            diagnostic_summary: "Stack diagnostics could not be produced for the uncertified archetype".to_owned(),
            diagnostic_stat_label: "diagnostics unavailable".to_owned(),
            diagnostic_refs: vec![],
            fix_forward_class: FixForwardClass::FixUnavailable,
            fix_forward_summary: "No fix-forward guidance could be produced without a health verdict".to_owned(),
            fix_forward_refs: vec![],
            freshness_class: ArchetypeHealthFreshnessClass::Fresh,
            freshness_chip_label: "checked · fresh".to_owned(),
            last_checked: "2026-04-12T00:00:00Z".to_owned(),
            support_class: ArchetypeHealthSupportClass::SupportUnknown,
            downgrade_banner_class: ArchetypeHealthDowngradeBannerClass::HealthUnknownBanner,
            known_issue_refs: vec![
                "known-issue:archetype_health:health_check_incomplete".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HealthUndeterminable,
                Trigger::DiagnosticsUnavailable,
                Trigger::UncertifiedArchetypeDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::HealthCheckPanel,
                Surface::StackDiagnostics,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        ArchetypeHealthBundleRow {
            row_id: "archetype-health-row:node.express.community.bridge.degraded:2026.06".to_owned(),
            archetype_kind: ArchetypeKind::ServiceArchetype,
            archetype_id: "archetype:node.express.service".to_owned(),
            archetype_label: "Node Express service (community, bridged checks)".to_owned(),
            archetype_locator: "archetype:framework_pack/node.express/service".to_owned(),
            archetype_version: "1.2.0".to_owned(),
            certification_class: ArchetypeCertificationClass::CommunityArchetype,
            bundle_id: "health-bundle:node.express.service".to_owned(),
            bundle_version: "0.5.0".to_owned(),
            app_id: "app:node.express.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:node.express@1.2.0".to_owned(),
            health_check_class: HealthCheckClass::Degraded,
            health_summary: "A security-audit check degraded; the verdict is bridged from an external health-check tool".to_owned(),
            stack_diagnostic_class: StackDiagnosticClass::Blocking,
            diagnostic_summary: "One blocking diagnostic reports a known-vulnerable dependency, bridged from an external auditor".to_owned(),
            diagnostic_stat_label: "7 checks · 1 blocking".to_owned(),
            diagnostic_refs: vec![
                "diagnostic:node.express.service/vulnerable_dependency".to_owned(),
            ],
            fix_forward_class: FixForwardClass::FixAdvisoryOnly,
            fix_forward_summary: "Only advisory fix-forward guidance is offered because the audit is bridged, not first-party".to_owned(),
            fix_forward_refs: vec![
                "fix-forward:node.express.service/upgrade_vulnerable_dependency".to_owned(),
            ],
            freshness_class: ArchetypeHealthFreshnessClass::RescanAvailable,
            freshness_chip_label: "checked · rescan available".to_owned(),
            last_checked: "2026-06-05T00:00:00Z".to_owned(),
            support_class: ArchetypeHealthSupportClass::BridgeBehavior,
            downgrade_banner_class: ArchetypeHealthDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:archetype_health:external_audit_bridge".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::BridgeBehaviorDisclosed,
                Trigger::UncertifiedArchetypeDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::ArchetypeGallery,
                Surface::HealthCheckPanel,
                Surface::StackDiagnostics,
                Surface::FixForwardGuidance,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
    ]
}

fn validate_source_contracts(
    packet: &ArchetypeHealthBundlePacket,
    violations: &mut Vec<ArchetypeHealthViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ARCHETYPE_HEALTH_SCHEMA_REF,
        ARCHETYPE_HEALTH_DOC_REF,
        TEMPLATE_MANIFEST_CONTRACT_REF,
        FRAMEWORK_PACK_CONTRACT_REF,
        GENERATOR_RUN_CONTRACT_REF,
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ArchetypeHealthViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &ArchetypeHealthBundlePacket,
    violations: &mut Vec<ArchetypeHealthViolation>,
) {
    if packet.rows.is_empty() {
        violations.push(ArchetypeHealthViolation::RowsEmpty);
        return;
    }

    for row in &packet.rows {
        if row.row_id.trim().is_empty()
            || row.archetype_id.trim().is_empty()
            || row.archetype_label.trim().is_empty()
            || row.archetype_locator.trim().is_empty()
            || row.archetype_version.trim().is_empty()
            || row.bundle_id.trim().is_empty()
            || row.bundle_version.trim().is_empty()
            || row.app_id.trim().is_empty()
            || row.framework_pack_ref.trim().is_empty()
            || row.health_summary.trim().is_empty()
            || row.diagnostic_summary.trim().is_empty()
            || row.diagnostic_stat_label.trim().is_empty()
            || row.fix_forward_summary.trim().is_empty()
            || row.freshness_chip_label.trim().is_empty()
            || row.last_checked.trim().is_empty()
        {
            violations.push(ArchetypeHealthViolation::RowIncomplete);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(ArchetypeHealthViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(ArchetypeHealthViolation::ConsumerSurfacesMissing);
        }

        validate_row_banners(row, violations);
    }
}

fn validate_row_banners(
    row: &ArchetypeHealthBundleRow,
    violations: &mut Vec<ArchetypeHealthViolation>,
) {
    // An unknown health verdict must raise the health-unknown banner, so a bundle is
    // never presented as a confident verdict it could not produce.
    if row.health_check_class.is_unknown()
        && row.downgrade_banner_class != ArchetypeHealthDowngradeBannerClass::HealthUnknownBanner
    {
        violations.push(ArchetypeHealthViolation::HealthUnknownBannerMissing);
    }

    // Unavailable diagnostics must show a downgrade banner. A health-unknown bundle
    // shows the health banner, which also covers its unavailable diagnostics.
    if row.stack_diagnostic_class.is_unavailable() && !row.downgrade_banner_class.is_present() {
        violations.push(ArchetypeHealthViolation::DiagnosticsUnavailableBannerMissing);
    }

    // An unavailable fix-forward path must show a downgrade banner so a missing fix
    // is labeled rather than silently hidden.
    if row.fix_forward_class.is_unavailable() && !row.downgrade_banner_class.is_present() {
        violations.push(ArchetypeHealthViolation::FixUnavailableBannerMissing);
    }

    // A grounded stack-diagnostic severity must carry at least one diagnostic ref.
    if row.stack_diagnostic_class.requires_findings() && row.diagnostic_refs.is_empty() {
        violations.push(ArchetypeHealthViolation::DiagnosticRefsMissing);
    }

    // A bundle carrying fix-forward guidance must carry at least one fix-forward ref.
    if row.fix_forward_class.carries_guidance() && row.fix_forward_refs.is_empty() {
        violations.push(ArchetypeHealthViolation::FixForwardRefsMissing);
    }

    // A non-certified archetype must show a banner and carry the disclosure trigger.
    if row.certification_class.requires_disclosure()
        && (!row.downgrade_banner_class.is_present()
            || !row
                .downgrade_triggers
                .contains(&ArchetypeHealthDowngradeTrigger::UncertifiedArchetypeDisclosed))
    {
        violations.push(ArchetypeHealthViolation::CertificationUndisclosed);
    }

    // Bridge/heuristic bundles must disclose a known issue, a banner, and the trigger.
    if row.support_class.requires_disclosure() {
        let matching_trigger = match row.support_class {
            ArchetypeHealthSupportClass::BridgeBehavior => {
                ArchetypeHealthDowngradeTrigger::BridgeBehaviorDisclosed
            }
            _ => ArchetypeHealthDowngradeTrigger::HeuristicMappingDisclosed,
        };
        if row.known_issue_refs.is_empty()
            || !row.downgrade_banner_class.is_present()
            || !row.downgrade_triggers.contains(&matching_trigger)
        {
            violations.push(ArchetypeHealthViolation::SupportClassUndisclosed);
        }
    }

    // A stale or unknown-freshness bundle must show a downgrade banner.
    if row.freshness_class.is_blocking() && !row.downgrade_banner_class.is_present() {
        violations.push(ArchetypeHealthViolation::FreshnessBannerMissing);
    }

    // A blocked bundle cannot be admitted for a confident verdict.
    if row.is_blocked() && row.admitted_for_display {
        violations.push(ArchetypeHealthViolation::BlockedDisplayAdmitted);
    }
}

fn validate_review(
    packet: &ArchetypeHealthBundlePacket,
    violations: &mut Vec<ArchetypeHealthViolation>,
) {
    let review = &packet.review;
    for ok in [
        review.certification_disclosed_for_every_bundle,
        review.health_state_shown_for_every_bundle,
        review.stack_diagnostics_shown_for_every_bundle,
        review.fix_forward_guidance_shown_when_available,
        review.health_unknown_blocks_confident_verdict,
        review.diagnostics_unavailable_blocks_confident_verdict,
        review.uncertified_archetype_never_presented_as_certified,
        review.heuristic_or_bridge_never_presented_as_exact_truth,
        review.fix_guidance_unavailable_labeled_not_hidden,
        review.stale_bundle_not_presented_as_current,
        review.support_class_visible_before_display,
        review.known_issues_disclosed_before_display,
        review.no_raw_source_bodies_or_urls_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
        review.certification_visible_before_display,
    ] {
        if !ok {
            violations.push(ArchetypeHealthViolation::ReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &ArchetypeHealthBundlePacket,
    violations: &mut Vec<ArchetypeHealthViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.gallery_shows_certification,
        projection.health_check_panel_shows_health_state,
        projection.stack_diagnostics_shows_diagnostics,
        projection.fix_forward_guidance_shows_remediation,
        projection.run_surface_shows_health_state,
        projection.cli_headless_shows_bundle_rows,
        projection.support_export_shows_bundle_rows,
        projection.diagnostics_shows_health_diagnostics_fix_state,
        projection.blocked_bundles_labeled_not_hidden,
        projection.uncertified_bundles_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(ArchetypeHealthViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &ArchetypeHealthBundlePacket,
    violations: &mut Vec<ArchetypeHealthViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ArchetypeHealthViolation::ProofFreshnessIncomplete);
    }
}

/// Raises the row's downgrade banner only when none is currently shown, so an
/// already-raised banner is never lowered to a softer cue.
fn raise_banner(row: &mut ArchetypeHealthBundleRow, banner: ArchetypeHealthDowngradeBannerClass) {
    if !row.downgrade_banner_class.is_present() {
        row.downgrade_banner_class = banner;
    }
}

fn push_unique_trigger(
    triggers: &mut Vec<ArchetypeHealthDowngradeTrigger>,
    trigger: ArchetypeHealthDowngradeTrigger,
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
