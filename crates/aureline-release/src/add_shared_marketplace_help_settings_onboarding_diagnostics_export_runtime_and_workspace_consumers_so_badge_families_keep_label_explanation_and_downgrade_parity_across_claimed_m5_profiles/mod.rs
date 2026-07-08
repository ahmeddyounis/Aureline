//! Shared consumers for the reusable M5 badge families, so the support-class,
//! evidence-freshness, lifecycle, channel, deployment-scope, and
//! compatibility-state badges keep the same label, explanation drawer, and
//! downgrade reason across every claimed M5 surface that shows a support,
//! freshness, lifecycle, channel, deployment, or compatibility claim.
//!
//! Aureline's frozen badge-family matrix
//! ([`crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`])
//! names the six governed badge families, and four sibling `implement_*` / `ship_*`
//! lanes narrow those families into working badge resolvers, each with its own
//! canonical schema, contract doc, and support-export artifact:
//!
//! * the support-class / evidence-freshness badges
//!   ([`crate::implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces`]),
//! * the lifecycle / channel badges
//!   ([`crate::implement_lifecycle_and_channel_badges_across_claimed_m5_command_feature_bundle_extension_and_install_surfaces`]),
//! * the deployment-scope badge
//!   ([`crate::implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces`]),
//!   and
//! * the compatibility-state badge
//!   ([`crate::ship_compatibility_state_badges_and_mismatch_review_affordances_across_claimed_m5_workspace_toolchain_extension_bundle_and_artifact_flows`]).
//!
//! This module is the *adoption* lane over those badge families. It proves the
//! badge families are reusable cross-product cues — not release-center-only or
//! ecosystem-only concepts — by binding every claimed M5 badge consumer (the
//! marketplace/install surface, Help/About, settings/policy explainers,
//! onboarding/start-center, diagnostics, the support export, runtime/deployment
//! cards, and workspace/archetype qualification) to the same six canonical badge
//! schemas and the same label / explanation / downgrade parity vocabulary. Each
//! consumer points at the badge family's canonical schema and support-export
//! artifact rather than re-wording the label, explanation, or downgrade reason in
//! local prose, and each keeps that parity truthful even when the badge
//! auto-narrows because its evidence went stale, its deployment/compatibility
//! scope reduced, or it renders in a non-interactive export snapshot outside the
//! live UI.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_badge_consumer_binding`] — that takes one consumer's
//!    adoption of one badge family, the parity facets it surfaces, its render
//!    mode, and any downgrade caveats, and produces one
//!    [`M5ResolvedBadgeConsumerBinding`] carrying the derived label/explanation/
//!    downgrade parity state and — whenever the badge auto-narrows — a
//!    self-contained [`M5BadgeNarrowBanner`] that names the exact downgrade reason
//!    (evidence stale, scope reduced, or export snapshot), the parity facets that
//!    stay preserved, and the next action, rather than a generic "reduced" note.
//!    The resolver never lets a narrowed badge drop a required parity facet and
//!    never invents a second badge grammar.
//! 2. A parity matrix — [`M5BadgeFamilyConsumerPacket`] — that binds one row per
//!    claimed M5 badge consumer to the six canonical badge families, the one shared
//!    parity vocabulary, the same render modes, downgrade caveats, parity states,
//!    narrow reasons, next actions, export fields, and non-visual accessibility
//!    routes, so badge meaning stops diverging between the marketplace, Help/About,
//!    settings, onboarding, diagnostics, the support export, runtime cards, and the
//!    workspace qualification surface.
//!
//! The badge families, surface families, deployment lines, badge consumer surfaces,
//! accessibility routes, qualification classes, and downgrade triggers are reused
//! verbatim from the frozen badge-family matrix. This module mints new vocabulary
//! only for what the adoption lane itself needs: its badge consumers, the shared
//! parity facets, the render modes, the narrow reasons and next actions, the
//! parity states, the consumer anatomy parts, and the export fields.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and
//! user text bodies stay outside the support boundary; every label is carried only
//! as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-badge-family-consumer.schema.json`](../../../../schemas/ui/m5-badge-family-consumer.schema.json)
//! and the contract doc is
//! [`docs/release/m5_badge_family_consumer_contract.md`](../../../../docs/release/m5_badge_family_consumer_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-badge-family-consumers/`](../../../../fixtures/ui/m5-badge-family-consumers/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_badge_family_consumer_diagnostics_freshness_beta_narrowed,
    seeded_m5_badge_family_consumer_packet,
    seeded_m5_badge_family_consumer_support_export_scope_preview_narrowed,
    M5_BADGE_FAMILY_CONSUMER_PACKET_ID,
};

// The badge families, surface families, deployment lines, badge consumer surfaces,
// accessibility routes, qualification classes, and downgrade triggers are frozen
// once, in the badge-family matrix. This adoption lane reuses them verbatim so it
// never invents a parallel badge vocabulary — the same family means the same thing
// everywhere.
pub use crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix::{
    M5BadgeAccessibilityRoute, M5BadgeConsumerSurface, M5BadgeDowngradeTrigger, M5BadgeFamily,
    M5BadgeQualificationClass, M5BadgeSurfaceFamily, M5DeploymentLine, M5_BADGE_FAMILY_DOC_REF,
    M5_BADGE_FAMILY_SCHEMA_REF,
};

// The four narrowed badge-family primitives this adoption lane points every
// consumer at, rather than re-wording their facts in local prose.
use crate::implement_deployment_scope_badges_with_local_only_managed_self_hosted_mirrored_offline_capable_and_browser_companion_truth_across_claimed_m5_runtime_install_help_and_export_surfaces::{
    M5_DEPLOYMENT_SCOPE_BADGE_ARTIFACT_REF, M5_DEPLOYMENT_SCOPE_BADGE_DOC_REF,
    M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF,
};
use crate::implement_lifecycle_and_channel_badges_across_claimed_m5_command_feature_bundle_extension_and_install_surfaces::{
    M5_MATURITY_BADGE_ARTIFACT_REF, M5_MATURITY_BADGE_DOC_REF, M5_MATURITY_BADGE_SCHEMA_REF,
};
use crate::implement_support_class_and_evidence_freshness_badges_across_claimed_m5_onboarding_help_marketplace_and_diagnostics_surfaces::{
    M5_BADGE_CLAIM_ARTIFACT_REF, M5_BADGE_CLAIM_DOC_REF, M5_BADGE_CLAIM_SCHEMA_REF,
};
use crate::ship_compatibility_state_badges_and_mismatch_review_affordances_across_claimed_m5_workspace_toolchain_extension_bundle_and_artifact_flows::{
    M5_COMPATIBILITY_STATE_BADGE_ARTIFACT_REF, M5_COMPATIBILITY_STATE_BADGE_DOC_REF,
    M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5BadgeFamilyConsumerPacket`].
pub const M5_BADGE_FAMILY_CONSUMER_RECORD_KIND: &str =
    "add_shared_marketplace_help_settings_onboarding_diagnostics_export_runtime_and_workspace_consumers_so_badge_families_keep_label_explanation_and_downgrade_parity_across_claimed_m5_profiles";

/// Schema version for M5 badge-family-consumer records.
pub const M5_BADGE_FAMILY_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the badge-family-consumer boundary schema.
pub const M5_BADGE_FAMILY_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-badge-family-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BADGE_FAMILY_CONSUMER_DOC_REF: &str =
    "docs/release/m5_badge_family_consumer_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_BADGE_FAMILY_CONSUMER_FIXTURE_DIR: &str = "fixtures/ui/m5-badge-family-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BADGE_FAMILY_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-badge-family-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_BADGE_FAMILY_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-badge-family-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BADGE_FAMILY_CONSUMER_REPORT_REF: &str =
    "artifacts/components/m5-badge-family-consumer.md";

/// The canonical boundary schema ref of the narrowed badge primitive that owns
/// `family`. A consumer that adopts this family must point at this schema, not at a
/// local re-description.
pub const fn badge_family_canonical_schema_ref(family: M5BadgeFamily) -> &'static str {
    match family {
        M5BadgeFamily::SupportClass | M5BadgeFamily::EvidenceFreshness => M5_BADGE_CLAIM_SCHEMA_REF,
        M5BadgeFamily::Lifecycle | M5BadgeFamily::Channel => M5_MATURITY_BADGE_SCHEMA_REF,
        M5BadgeFamily::DeploymentScope => M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF,
        M5BadgeFamily::CompatibilityState => M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF,
    }
}

/// The canonical contract-doc ref of the narrowed badge primitive that owns
/// `family`.
pub const fn badge_family_canonical_doc_ref(family: M5BadgeFamily) -> &'static str {
    match family {
        M5BadgeFamily::SupportClass | M5BadgeFamily::EvidenceFreshness => M5_BADGE_CLAIM_DOC_REF,
        M5BadgeFamily::Lifecycle | M5BadgeFamily::Channel => M5_MATURITY_BADGE_DOC_REF,
        M5BadgeFamily::DeploymentScope => M5_DEPLOYMENT_SCOPE_BADGE_DOC_REF,
        M5BadgeFamily::CompatibilityState => M5_COMPATIBILITY_STATE_BADGE_DOC_REF,
    }
}

/// The canonical support-export artifact ref of the narrowed badge primitive that
/// owns `family`.
pub const fn badge_family_canonical_artifact_ref(family: M5BadgeFamily) -> &'static str {
    match family {
        M5BadgeFamily::SupportClass | M5BadgeFamily::EvidenceFreshness => {
            M5_BADGE_CLAIM_ARTIFACT_REF
        }
        M5BadgeFamily::Lifecycle | M5BadgeFamily::Channel => M5_MATURITY_BADGE_ARTIFACT_REF,
        M5BadgeFamily::DeploymentScope => M5_DEPLOYMENT_SCOPE_BADGE_ARTIFACT_REF,
        M5BadgeFamily::CompatibilityState => M5_COMPATIBILITY_STATE_BADGE_ARTIFACT_REF,
    }
}

/// One claimed M5 badge consumer that adopts the shared badge families. These are
/// the consumers the acceptance criteria name — the marketplace/install surface,
/// Help/About, settings/policy explainers, onboarding/start-center, diagnostics,
/// the support export, runtime/deployment cards, and workspace/archetype
/// qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeConsumer {
    /// The marketplace / install surface.
    Marketplace,
    /// The Help / About surface.
    HelpAbout,
    /// The settings / policy-explainer surface.
    Settings,
    /// The onboarding / start-center surface.
    Onboarding,
    /// The diagnostics surface.
    Diagnostics,
    /// The support export.
    SupportExport,
    /// The runtime / deployment-card surface.
    Runtime,
    /// The workspace / archetype-qualification surface.
    Workspace,
}

impl M5BadgeConsumer {
    /// Every claimed badge consumer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Marketplace,
        Self::HelpAbout,
        Self::Settings,
        Self::Onboarding,
        Self::Diagnostics,
        Self::SupportExport,
        Self::Runtime,
        Self::Workspace,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Marketplace => "marketplace",
            Self::HelpAbout => "help_about",
            Self::Settings => "settings",
            Self::Onboarding => "onboarding",
            Self::Diagnostics => "diagnostics",
            Self::SupportExport => "support_export",
            Self::Runtime => "runtime",
            Self::Workspace => "workspace",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Marketplace => "Marketplace / Install",
            Self::HelpAbout => "Help / About",
            Self::Settings => "Settings / Policy",
            Self::Onboarding => "Onboarding / Start Center",
            Self::Diagnostics => "Diagnostics",
            Self::SupportExport => "Support Export",
            Self::Runtime => "Runtime / Deployment",
            Self::Workspace => "Workspace / Archetype",
        }
    }

    /// True when this consumer is a docs/help or exported-evidence surface — the
    /// surfaces the acceptance criteria single out for a canonical-schema reference
    /// so their prose or export can never drift from the live badge truth.
    pub const fn is_docs_help_or_export(self) -> bool {
        matches!(self, Self::HelpAbout | Self::SupportExport)
    }
}

/// The one shared parity vocabulary every badge family keeps aligned across
/// surfaces, so no consumer invents a new badge or re-words a facet. The facets in
/// [`M5BadgeParityFacet::REQUIRED`] must be present on every binding — the track
/// invariant that a badge's label, explanation, and downgrade reason stay explicit
/// and separately filterable everywhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeParityFacet {
    /// The badge's stable label / value state.
    Label,
    /// The badge's explanation-drawer content.
    Explanation,
    /// The badge's downgrade reason / rule.
    DowngradeReason,
    /// The badge's separately-filterable key.
    FilterKey,
}

impl M5BadgeParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Label,
        Self::Explanation,
        Self::DowngradeReason,
        Self::FilterKey,
    ];

    /// Every parity facet is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Label => "label",
            Self::Explanation => "explanation",
            Self::DowngradeReason => "downgrade_reason",
            Self::FilterKey => "filter_key",
        }
    }
}

/// The render mode a consumer shows a badge under. A narrowed mode still keeps the
/// parity vocabulary — it only discloses that the badge auto-narrowed because its
/// truth weakened relative to the authoritative live claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeRenderMode {
    /// Full claim scope: the authoritative live badge rendering.
    FullClaimScope,
    /// The badge auto-narrowed because its evidence went stale.
    FreshnessNarrowed,
    /// The badge auto-narrowed because its deployment / compatibility scope reduced.
    ScopeNarrowed,
    /// The badge renders in a non-interactive export / mirror snapshot.
    ExportProjection,
}

impl M5BadgeRenderMode {
    /// Every render mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullClaimScope,
        Self::FreshnessNarrowed,
        Self::ScopeNarrowed,
        Self::ExportProjection,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullClaimScope => "full_claim_scope",
            Self::FreshnessNarrowed => "freshness_narrowed",
            Self::ScopeNarrowed => "scope_narrowed",
            Self::ExportProjection => "export_projection",
        }
    }

    /// True when the mode renders below the authoritative full claim scope and so
    /// must disclose a self-contained narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullClaimScope)
    }

    /// The narrow reason a narrowed mode discloses, if any.
    pub const fn narrow_reason(self) -> Option<M5BadgeNarrowReason> {
        Some(match self {
            Self::FreshnessNarrowed => M5BadgeNarrowReason::EvidenceStale,
            Self::ScopeNarrowed => M5BadgeNarrowReason::ScopeReduced,
            Self::ExportProjection => M5BadgeNarrowReason::ExportSnapshot,
            Self::FullClaimScope => return None,
        })
    }
}

/// The exact reason a badge auto-narrowed, so a narrow banner never reads like a
/// generic "reduced" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeNarrowReason {
    /// The badge's evidence went stale, so it auto-narrowed.
    EvidenceStale,
    /// The badge's deployment / compatibility scope reduced.
    ScopeReduced,
    /// The badge renders from an export / mirror snapshot rather than the live UI.
    ExportSnapshot,
}

impl M5BadgeNarrowReason {
    /// Every narrow reason, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::EvidenceStale,
        Self::ScopeReduced,
        Self::ExportSnapshot,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EvidenceStale => "evidence_stale",
            Self::ScopeReduced => "scope_reduced",
            Self::ExportSnapshot => "export_snapshot",
        }
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::EvidenceStale => "the badge auto-narrowed because its evidence went stale",
            Self::ScopeReduced => {
                "the badge auto-narrowed because its deployment or compatibility scope reduced"
            }
            Self::ExportSnapshot => "the badge renders from an export or mirror snapshot",
        }
    }

    /// The next action a reader should take to reach the authoritative live badge.
    pub const fn next_action(self) -> M5BadgeNarrowNextAction {
        match self {
            Self::EvidenceStale => M5BadgeNarrowNextAction::RefreshStaleEvidence,
            Self::ScopeReduced => M5BadgeNarrowNextAction::ReviewNarrowedScope,
            Self::ExportSnapshot => M5BadgeNarrowNextAction::OpenLiveBadgeSurface,
        }
    }
}

/// The next action named on a narrow banner, so a narrowed badge is actionable from
/// the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeNarrowNextAction {
    /// Refresh the stale evidence behind the badge.
    RefreshStaleEvidence,
    /// Review the narrowed deployment / compatibility scope.
    ReviewNarrowedScope,
    /// Open the live badge surface for the interactive explanation drawer.
    OpenLiveBadgeSurface,
}

impl M5BadgeNarrowNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 3] = [
        Self::RefreshStaleEvidence,
        Self::ReviewNarrowedScope,
        Self::OpenLiveBadgeSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshStaleEvidence => "refresh_stale_evidence",
            Self::ReviewNarrowedScope => "review_narrowed_scope",
            Self::OpenLiveBadgeSurface => "open_live_badge_surface",
        }
    }
}

/// The derived label/explanation/downgrade parity state of a binding — whether the
/// shared parity vocabulary is preserved as-is or preserved with a disclosed
/// narrowing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeParityState {
    /// The parity vocabulary is preserved at full claim scope.
    FacetsPreserved,
    /// The parity vocabulary is preserved, with a disclosed auto-narrowing.
    FacetsDisclosedNarrowed,
}

impl M5BadgeParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::FacetsPreserved, Self::FacetsDisclosedNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5BadgeConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeConsumerAnatomyPart {
    /// The adopted badge identity.
    BadgeIdentity,
    /// The canonical badge schema reference.
    CanonicalSchemaRef,
    /// The shared parity-facet set.
    ParityFacetSet,
    /// The explanation drawer.
    ExplanationDrawer,
    /// The downgrade-caveat list.
    DowngradeCaveats,
    /// The derived parity verdict.
    ParityVerdict,
    /// The narrow banner (shown when narrowed).
    NarrowBanner,
}

impl M5BadgeConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::BadgeIdentity,
        Self::CanonicalSchemaRef,
        Self::ParityFacetSet,
        Self::ExplanationDrawer,
        Self::DowngradeCaveats,
        Self::ParityVerdict,
        Self::NarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::BadgeIdentity,
        Self::CanonicalSchemaRef,
        Self::ParityFacetSet,
        Self::ParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BadgeIdentity => "badge_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::ParityFacetSet => "parity_facet_set",
            Self::ExplanationDrawer => "explanation_drawer",
            Self::DowngradeCaveats => "downgrade_caveats",
            Self::ParityVerdict => "parity_verdict",
            Self::NarrowBanner => "narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is
/// reconstructable from the shared model. The fields in
/// [`M5BadgeConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted badge family.
    BadgeFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The parity-facet set.
    ParityFacetSet,
    /// The render mode.
    RenderMode,
    /// The downgrade caveats.
    DowngradeCaveats,
    /// The parity state.
    ParityState,
    /// The narrow reason (when narrowed).
    NarrowReason,
}

impl M5BadgeConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::BadgeFamily,
        Self::CanonicalSchemaRef,
        Self::ParityFacetSet,
        Self::RenderMode,
        Self::DowngradeCaveats,
        Self::ParityState,
        Self::NarrowReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::BadgeFamily,
        Self::CanonicalSchemaRef,
        Self::ParityFacetSet,
        Self::ParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::BadgeFamily => "badge_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::ParityFacetSet => "parity_facet_set",
            Self::RenderMode => "render_mode",
            Self::DowngradeCaveats => "downgrade_caveats",
            Self::ParityState => "parity_state",
            Self::NarrowReason => "narrow_reason",
        }
    }
}

/// A self-contained narrow banner: the exact downgrade reason, the parity facets
/// that stay preserved, the downgrade caveats, and the next action, so a narrowed
/// badge is understood from the banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeNarrowBanner {
    /// The exact narrow reason.
    pub reason: M5BadgeNarrowReason,
    /// The next action a reader should take.
    pub next_action: M5BadgeNarrowNextAction,
    /// The consumer the banner applies to.
    pub consumer: M5BadgeConsumer,
    /// The badge family the banner applies to.
    pub badge_family: M5BadgeFamily,
    /// The parity facets that stay preserved under the narrowing.
    pub preserved_facets: Vec<M5BadgeParityFacet>,
    /// The downgrade caveats disclosed alongside the narrowing.
    pub downgrade_caveats: Vec<M5BadgeDowngradeTrigger>,
    /// A deterministic, self-contained headline naming the reason, the preserved
    /// facets, and the next action — never a generic "reduced" note.
    pub headline: String,
}

/// The full input to the badge-consumer-binding resolver for one consumer/family
/// adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerBindingInput {
    /// The consumer that adopts the badge.
    pub consumer: M5BadgeConsumer,
    /// The canonical badge family being adopted.
    pub badge_family: M5BadgeFamily,
    /// The parity facets the binding surfaces. Must cover every required facet so
    /// the label, explanation, downgrade reason, and filter key stay explicit.
    pub parity_facets: Vec<M5BadgeParityFacet>,
    /// The render mode the binding shows the badge under.
    pub render_mode: M5BadgeRenderMode,
    /// The downgrade triggers disclosed alongside an auto-narrowing.
    pub downgrade_caveats: Vec<M5BadgeDowngradeTrigger>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved parity / narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedBadgeConsumerBinding {
    /// The consumer.
    pub consumer: M5BadgeConsumer,
    /// The badge family.
    pub badge_family: M5BadgeFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The parity facets the binding surfaces.
    pub parity_facets: Vec<M5BadgeParityFacet>,
    /// The render mode.
    pub render_mode: M5BadgeRenderMode,
    /// The downgrade caveats.
    pub downgrade_caveats: Vec<M5BadgeDowngradeTrigger>,
    /// The derived parity state.
    pub parity_state: M5BadgeParityState,
    /// True when the badge renders auto-narrowed.
    pub is_narrowed: bool,
    /// The narrow banner, present when narrowed.
    pub narrow_banner: Option<M5BadgeNarrowBanner>,
}

/// Errors returned by [`resolve_badge_consumer_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5BadgeConsumerBindingError {
    /// The parity-facet set was empty.
    EmptyParityFacetSet,
    /// A required parity facet was missing from the binding.
    MissingRequiredFacet,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5BadgeConsumerBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyParityFacetSet => "empty_parity_facet_set",
            Self::MissingRequiredFacet => "missing_required_facet",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5BadgeConsumerBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "badge consumer binding error: {}", self.as_str())
    }
}

impl Error for M5BadgeConsumerBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required parity facet must be present — the track invariant that a
/// badge's label, explanation, downgrade reason, and filter key stay explicit on
/// every surface. The parity state is preserved at full claim scope and
/// disclosed-narrowed under any auto-narrowing, and a narrowed badge always
/// produces a self-contained banner naming the exact downgrade reason and next
/// action while keeping the parity vocabulary intact.
pub fn resolve_badge_consumer_binding(
    input: &M5BadgeConsumerBindingInput,
) -> Result<M5ResolvedBadgeConsumerBinding, M5BadgeConsumerBindingError> {
    if input.parity_facets.is_empty() {
        return Err(M5BadgeConsumerBindingError::EmptyParityFacetSet);
    }
    let present: BTreeSet<M5BadgeParityFacet> = input.parity_facets.iter().copied().collect();
    for required in M5BadgeParityFacet::REQUIRED {
        if !present.contains(&required) {
            return Err(M5BadgeConsumerBindingError::MissingRequiredFacet);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5BadgeConsumerBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.downgrade_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future
        // free-text extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5BadgeConsumerBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.render_mode.is_narrowed();
    let parity_state = if is_narrowed {
        M5BadgeParityState::FacetsDisclosedNarrowed
    } else {
        M5BadgeParityState::FacetsPreserved
    };

    let narrow_banner = input.render_mode.narrow_reason().map(|reason| {
        let next_action = reason.next_action();
        let headline = format!(
            "Badge auto-narrowed: {} — {} shows {} with {} parity facet(s) preserved; next: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.badge_family.as_str(),
            input.parity_facets.len(),
            next_action.as_str()
        );
        M5BadgeNarrowBanner {
            reason,
            next_action,
            consumer: input.consumer,
            badge_family: input.badge_family,
            preserved_facets: input.parity_facets.clone(),
            downgrade_caveats: input.downgrade_caveats.clone(),
            headline,
        }
    });

    Ok(M5ResolvedBadgeConsumerBinding {
        consumer: input.consumer,
        badge_family: input.badge_family,
        canonical_schema_ref: badge_family_canonical_schema_ref(input.badge_family).to_owned(),
        parity_facets: input.parity_facets.clone(),
        render_mode: input.render_mode,
        downgrade_caveats: input.downgrade_caveats.clone(),
        parity_state,
        is_narrowed,
        narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet
/// reconstructs consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerBindingCase {
    /// The resolver input.
    pub input: M5BadgeConsumerBindingInput,
    /// The resolved truth. Must equal `resolve_badge_consumer_binding(&input)`.
    pub resolved: M5ResolvedBadgeConsumerBinding,
}

impl M5BadgeConsumerBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5BadgeConsumerBindingInput) -> Self {
        let resolved = resolve_badge_consumer_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_badge_consumer_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical badge family: the canonical refs the
/// consumer points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeFamilyBinding {
    /// The canonical badge family being adopted.
    pub badge_family: M5BadgeFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's
    /// canonical schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal
    /// the family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local
    /// re-description of its label / explanation / downgrade reason. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5BadgeConsumerBindingCase>,
}

impl M5BadgeFamilyBinding {
    /// True when the binding points at the family's canonical refs and references
    /// the canonical family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == badge_family_canonical_schema_ref(self.badge_family)
            && self.canonical_artifact_ref == badge_family_canonical_artifact_ref(self.badge_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one badge consumer bound to the canonical badge
/// families, the shared parity vocabulary, the render modes, downgrade caveats,
/// parity states, narrow reasons, next actions, export fields, and accessibility
/// routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerRow {
    /// Badge consumer.
    pub consumer: M5BadgeConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5BadgeQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 badge surface families that render / consume this projection.
    pub surface_families: Vec<M5BadgeSurfaceFamily>,
    /// Deployment lines this projection keeps the same badge truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5BadgeConsumerAnatomyPart>,
    /// Parity facets this consumer keeps aligned (must include the required set).
    pub parity_facets: Vec<M5BadgeParityFacet>,
    /// Render modes this consumer distinguishes.
    pub render_modes: Vec<M5BadgeRenderMode>,
    /// Downgrade caveats this consumer preserves.
    pub downgrade_caveats: Vec<M5BadgeDowngradeTrigger>,
    /// Parity states this consumer distinguishes.
    pub parity_states: Vec<M5BadgeParityState>,
    /// Narrow reasons this consumer names.
    pub narrow_reasons: Vec<M5BadgeNarrowReason>,
    /// Narrow next actions this consumer names.
    pub narrow_next_actions: Vec<M5BadgeNarrowNextAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5BadgeConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5BadgeAccessibilityRoute>,
    /// Badge consumer surfaces that consume this projection.
    pub consumer_surfaces: Vec<M5BadgeConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5BadgeDowngradeTrigger>,
    /// The canonical badge families this consumer adopts, with worked bindings.
    pub family_bindings: Vec<M5BadgeFamilyBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never collapses multiple badge axes into one
    /// overloaded pill. MUST be `false`.
    pub collapses_axes_into_one_pill: bool,
    /// Hard invariant: this consumer never implies freshness from support class (or
    /// any cross-axis implication). MUST be `false`.
    pub implies_freshness_from_support_class: bool,
    /// Hard invariant: this consumer never lets exported evidence lose badge
    /// meaning. MUST be `false`.
    pub drops_badge_meaning_in_export: bool,
    /// Hard invariant: this consumer never re-words the badge label/explanation per
    /// surface. MUST be `false`.
    pub rewords_labels_per_surface: bool,
}

impl M5BadgeConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5BadgeConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5BadgeConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5BadgeConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5BadgeConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required parity facet.
    fn declares_required_facets(&self) -> bool {
        let present: BTreeSet<M5BadgeParityFacet> = self.parity_facets.iter().copied().collect();
        M5BadgeParityFacet::REQUIRED
            .iter()
            .all(|facet| present.contains(facet))
    }

    /// True when every family binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.family_bindings
            .iter()
            .all(M5BadgeFamilyBinding::points_to_canonical_family)
    }

    /// The set of badge families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5BadgeFamily> {
        self.family_bindings
            .iter()
            .map(|binding| binding.badge_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_axes_into_one_pill
            && !self.implies_freshness_from_support_class
            && !self.drops_badge_meaning_in_export
            && !self.rewords_labels_per_surface
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerVocabularySet {
    /// Badge-consumer tokens.
    pub consumers: Vec<String>,
    /// Badge-family tokens (reused from the frozen matrix).
    pub badge_families: Vec<String>,
    /// Parity-facet tokens.
    pub parity_facets: Vec<String>,
    /// Render-mode tokens.
    pub render_modes: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
    /// Narrow-reason tokens.
    pub narrow_reasons: Vec<String>,
    /// Narrow-next-action tokens.
    pub narrow_next_actions: Vec<String>,
    /// Parity-state tokens.
    pub parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5BadgeConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5BadgeConsumer::ALL, |v| v.as_str()),
            badge_families: tokens(&M5BadgeFamily::ALL, |v| v.as_str()),
            parity_facets: tokens(&M5BadgeParityFacet::ALL, |v| v.as_str()),
            render_modes: tokens(&M5BadgeRenderMode::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5BadgeDowngradeTrigger::ALL, |v| v.as_str()),
            narrow_reasons: tokens(&M5BadgeNarrowReason::ALL, |v| v.as_str()),
            narrow_next_actions: tokens(&M5BadgeNarrowNextAction::ALL, |v| v.as_str()),
            parity_states: tokens(&M5BadgeParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5BadgeConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5BadgeConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5BadgeAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerGovernanceReview {
    /// Every consumer adopts the same canonical badge families.
    pub consumers_adopt_shared_badge_families: bool,
    /// Every consumer points at the canonical badge schema, not local prose.
    pub consumers_reference_canonical_badge_schema: bool,
    /// The label vocabulary is shared, never re-worded per surface.
    pub label_vocabulary_shared_not_reworded: bool,
    /// No consumer collapses multiple badge axes into one overloaded pill.
    pub no_consumer_collapses_axes_into_one_pill: bool,
    /// Label, explanation, and downgrade reason stay explicit everywhere.
    pub explanation_and_downgrade_explicit_on_every_surface: bool,
    /// No consumer implies freshness from support class.
    pub freshness_never_implied_from_support_class: bool,
    /// A narrowed badge always shows a self-contained narrow banner.
    pub narrowed_badge_always_shows_self_contained_banner: bool,
    /// The banner names an exact downgrade reason and next action, never a generic
    /// note.
    pub banner_names_exact_downgrade_reason_and_next_action: bool,
    /// The support / export packet preserves the same label, explanation, and
    /// downgrade reason the live UI shows.
    pub support_export_preserves_label_explanation_downgrade: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerProjection {
    /// Marketplace, Help/About, settings, onboarding, diagnostics, support export,
    /// runtime, and workspace consumers all adopt the shared badge families.
    pub all_consumers_adopt_shared_badge_families: bool,
    /// The badge label reads a single canonical source.
    pub label_reads_single_source: bool,
    /// The explanation drawer reads a single canonical source.
    pub explanation_reads_single_source: bool,
    /// The downgrade reason reads a single canonical source.
    pub downgrade_reason_reads_single_source: bool,
    /// The filter key reads a single canonical source.
    pub filter_key_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting consumer audit.
    pub consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BadgeFamilyConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BadgeFamilyConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5BadgeConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BadgeConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BadgeConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BadgeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BadgeConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BadgeConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 badge-family-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeFamilyConsumerPacket {
    /// Record kind; must equal [`M5_BADGE_FAMILY_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BADGE_FAMILY_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5BadgeConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BadgeConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BadgeConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BadgeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BadgeConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BadgeConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BadgeFamilyConsumerPacket {
    /// Builds an M5 badge-family-consumer packet from stable-lane input.
    pub fn new(input: M5BadgeFamilyConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_BADGE_FAMILY_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_BADGE_FAMILY_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            consumer_rows: input.consumer_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 badge-family-consumer invariants.
    pub fn validate(&self) -> Vec<M5BadgeConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BADGE_FAMILY_CONSUMER_RECORD_KIND {
            violations.push(M5BadgeConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BADGE_FAMILY_CONSUMER_SCHEMA_VERSION {
            violations.push(M5BadgeConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BadgeConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_docs_help_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 badge-family consumer packet serializes"),
        ) {
            violations.push(M5BadgeConsumerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 badge-family consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,render_modes,parity_states,narrow_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.family_bindings, |b| b.badge_family.as_str()),
                join_tokens(&row.render_modes, |v| v.as_str()),
                join_tokens(&row.parity_states, |v| v.as_str()),
                join_tokens(&row.narrow_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.family_bindings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .consumer_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Badge-Family Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Badge consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Badge families: {}\n",
            self.vocabulary_set.badge_families.join(", ")
        ));
        out.push_str(&format!(
            "- Parity facets: {}\n",
            self.vocabulary_set.parity_facets.join(", ")
        ));
        out.push_str(&format!(
            "- Render modes: {}\n",
            self.vocabulary_set.render_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Badge consumers\n\n");
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Adopted families: {}\n",
                row.family_bindings.len()
            ));
            for binding in &row.family_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.badge_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.render_mode.as_str(),
                        case.resolved.parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 badge-family-consumer export.
#[derive(Debug)]
pub enum M5BadgeConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BadgeConsumerViolation>),
}

impl fmt::Display for M5BadgeConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 badge-family consumer export parse failed: {error}"
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
                    "m5 badge-family consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BadgeConsumerArtifactError {}

/// Validation failures emitted by [`M5BadgeFamilyConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BadgeConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required badge consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required parity facet.
    RequiredFacetMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no family bindings.
    FamilyBindingMissing,
    /// A family binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A family binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required badge family is never adopted, or is adopted by only one consumer
    /// (reuse across surfaces unproven).
    BadgeFamilyReuseUnproven,
    /// No worked binding proves an auto-narrowed rendering with a self-contained
    /// banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-scope rendering with preserved parity and no
    /// banner.
    ScopePreservedUnproven,
    /// A docs/help/export consumer does not reference the canonical badge schema.
    DocsHelpReferenceMissing,
    /// A consumer row violates a hard invariant.
    ConsumerInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BadgeConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::ConsumerRowIncomplete => "consumer_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::RequiredFacetMissing => "required_facet_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::FamilyBindingMissing => "family_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::BadgeFamilyReuseUnproven => "badge_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::DocsHelpReferenceMissing => "docs_help_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 badge-family-consumer export.
pub fn current_stable_m5_badge_family_consumer_export(
) -> Result<M5BadgeFamilyConsumerPacket, M5BadgeConsumerArtifactError> {
    let packet: M5BadgeFamilyConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-badge-family-consumer-proof/support_export.json"
    )))
    .map_err(M5BadgeConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BadgeConsumerArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BADGE_FAMILY_CONSUMER_SCHEMA_REF,
        M5_BADGE_FAMILY_CONSUMER_DOC_REF,
        M5_BADGE_FAMILY_SCHEMA_REF,
        M5_BADGE_CLAIM_SCHEMA_REF,
        M5_MATURITY_BADGE_SCHEMA_REF,
        M5_DEPLOYMENT_SCOPE_BADGE_SCHEMA_REF,
        M5_COMPATIBILITY_STATE_BADGE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BadgeConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BadgeConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    let present: BTreeSet<M5BadgeConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5BadgeConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5BadgeConsumerViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.consumer_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.render_modes.is_empty()
            || row.downgrade_caveats.is_empty()
            || row.parity_states.is_empty()
            || row.narrow_reasons.is_empty()
            || row.narrow_next_actions.is_empty()
        {
            violations.push(M5BadgeConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5BadgeConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_facets() {
            violations.push(M5BadgeConsumerViolation::RequiredFacetMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5BadgeConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5BadgeConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BadgeConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BadgeConsumerViolation::DowngradeTriggersMissing);
        }
        if row.family_bindings.is_empty() {
            violations.push(M5BadgeConsumerViolation::FamilyBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5BadgeConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .family_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5BadgeConsumerViolation::ExampleBindingMissing);
        }
        if row.family_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5BadgeConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5BadgeConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5BadgeConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical badge family must be adopted by at least two distinct consumers
/// — the acceptance-criterion proof that the families are reusable cross-product
/// cues rather than release-center-only or ecosystem-only concepts.
fn validate_family_reuse(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    for family in M5BadgeFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5BadgeConsumerViolation::BadgeFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove an auto-narrowed
/// rendering whose banner carries a specific downgrade reason, a next action, and a
/// non-empty set of preserved parity facets — the acceptance-criterion example that
/// badges stay truthful and auto-narrow when their truth weakens.
fn validate_narrowing_disclosure(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case.resolved.narrow_banner.as_ref().is_some_and(|banner| {
                !banner.headline.trim().is_empty() && !banner.preserved_facets.is_empty()
            })
    });
    if !proven {
        violations.push(M5BadgeConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-scope rendering
/// with preserved parity and no banner — the acceptance-criterion example that
/// full-scope consumers keep the parity vocabulary without a spurious narrowing
/// note.
fn validate_scope_preserved(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.narrow_banner.is_none()
            && case.resolved.parity_state == M5BadgeParityState::FacetsPreserved
    });
    if !proven {
        violations.push(M5BadgeConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every docs/help/export consumer must reference the canonical badge schema for
/// each family it adopts — the acceptance-criterion that docs/help/support exports
/// preserve the same label, explanation, and downgrade reason the live UI shows.
fn validate_docs_help_reference(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_docs_help_or_export() {
            continue;
        }
        let references_canonical = !row.family_bindings.is_empty()
            && row
                .family_bindings
                .iter()
                .all(M5BadgeFamilyBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5BadgeConsumerViolation::DocsHelpReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_badge_families,
        review.consumers_reference_canonical_badge_schema,
        review.label_vocabulary_shared_not_reworded,
        review.no_consumer_collapses_axes_into_one_pill,
        review.explanation_and_downgrade_explicit_on_every_surface,
        review.freshness_never_implied_from_support_class,
        review.narrowed_badge_always_shows_self_contained_banner,
        review.banner_names_exact_downgrade_reason_and_next_action,
        review.support_export_preserves_label_explanation_downgrade,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BadgeConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_badge_families,
        projection.label_reads_single_source,
        projection.explanation_reads_single_source,
        projection.downgrade_reason_reads_single_source,
        projection.filter_key_reads_single_source,
    ] {
        if !ok {
            violations.push(M5BadgeConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BadgeConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BadgeFamilyConsumerPacket,
    violations: &mut Vec<M5BadgeConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.consumer_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BadgeConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5BadgeFamilyConsumerPacket,
) -> impl Iterator<Item = &M5BadgeConsumerBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.family_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
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

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
