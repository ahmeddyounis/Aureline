//! Shared consumers for the reusable M5 learning components, so the learning-mode toggle, tip
//! card, guided-exercise step, glossary chip / card, safe-explanation banner, and progress
//! marker keep citation, source-class, progress / privacy, and explain-versus-do language
//! aligned across every claimed M5 surface where a user learns: first-run onboarding, migration
//! onboarding, contextual help, the docs / browser surface, a feature-family tour, the companion
//! handoff, and the support / export packet.
//!
//! Aureline's frozen learning-component matrix
//! (`crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix`)
//! names the six governed component families, and three sibling implement lanes narrow those
//! families into working primitives, each with its own canonical schema, contract doc, and
//! support-export artifact:
//!
//! * the learning-mode toggle and tip card
//!   (`implement_learning_mode_toggles_and_tip_cards_...`),
//! * the guided-exercise step and progress marker
//!   (`implement_guided_exercise_steps_and_progress_markers_...`), and
//! * the glossary chip / card and safe-explanation banner
//!   (`implement_glossary_chips_or_cards_and_safe_explanation_banners_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the six families are
//! reusable components — not one onboarding page plus a few isolated help objects — by binding
//! every claimed M5 learning consumer (onboarding, migration, contextual help, the docs / browser
//! surface, a feature-family tour, the companion handoff, and the support / export packet) to the
//! same canonical component schemas and the same descriptor vocabulary. Each consumer points at
//! the primitive's canonical schema and support-export artifact rather than re-wording citation,
//! source-class, progress / privacy, or explain-versus-do facts in local prose, and each keeps
//! that vocabulary truthful even when a glossary / tip pack is served from cache, cited content is
//! stale, a cited source is unavailable or not installed, or progress stays local-only because no
//! supported sync / export path was chosen.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_learning_component_binding`] — that takes one consumer's adoption of
//!    one component family, the descriptor set it surfaces, the parity-health mode it renders
//!    under, and any export caveats, and produces one [`M5LearningComponentResolvedBinding`]
//!    carrying the derived claim-parity state and — whenever parity is weakened — a self-contained
//!    [`M5LearningComponentAutoNarrowBanner`] that names the exact reason (a cached pack, stale
//!    source content, an unavailable / not-installed cited source, or local-only progress), the
//!    descriptors that stay preserved, and the recovery action, rather than a generic "degraded"
//!    note. The resolver never lets a narrowed context drop a required descriptor and never lets an
//!    uncited or unavailable source masquerade as a live, cited one.
//! 2. A parity matrix — [`M5LearningComponentConsumerPacket`] — that binds one row per claimed M5
//!    learning consumer to the six canonical component families, the one shared descriptor
//!    vocabulary, the same parity-health modes, export caveats, parity states, narrowing reasons,
//!    recovery actions, export fields, and non-visual accessibility routes, so citation /
//!    source-class / progress-privacy / explain-versus-do facts stop diverging between the product
//!    UI, the docs, and the support artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes, qualification
//! classes, downgrade triggers, and the six component families themselves are reused verbatim from
//! the frozen learning-component matrix. This module mints new vocabulary only for what the
//! adoption lane itself needs: its learning consumers, the shared descriptor vocabulary, the
//! parity-health modes, the export caveats, the claim-parity states, the narrowing reasons and
//! recovery actions, the consumer anatomy parts, and the export fields.
//!
//! Raw secrets, endpoints, tokens, and raw provider bodies stay outside the support boundary;
//! every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is `schemas/ui/m5-learning-component-consumer.schema.json` and the contract
//! doc is `docs/help/m5_learning_component_consumers.md`. The protected fixture directory is
//! `fixtures/ui/m5-learning-component-consumers/`.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_learning_component_consumer_companion_handoff_preview_narrowed,
    seeded_m5_learning_component_consumer_docs_browser_beta_narrowed,
    seeded_m5_learning_component_consumer_packet, M5_LEARNING_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes, qualification
// classes, downgrade triggers, and the six component families are frozen once, in the
// learning-component matrix. This adoption lane reuses them verbatim so it never invents a
// parallel learning vocabulary.
pub use crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::{
    M5LearningAccessibilityRoute, M5LearningComponentFamily, M5LearningConsumerSurface,
    M5LearningDeploymentLine, M5LearningDowngradeTrigger, M5LearningQualificationClass,
    M5LearningSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at, rather than
// re-wording their facts in local prose.
use crate::freeze_the_m5_learning_mode_toggle_tip_card_guided_exercise_step_glossary_chip_or_card_safe_explanation_banner_and_progress_marker_component_matrix::{
    M5_LEARNING_COMPONENT_DOC_REF, M5_LEARNING_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_glossary_chips_or_cards_and_safe_explanation_banners_with_cited_file_symbol_doc_truth_freshness_source_class_labels_and_explain_versus_do_separation_across_claimed_m5_learning_surfaces::{
    GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_ARTIFACT_REF,
    GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_DOC_REF,
    GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
};
use crate::implement_guided_exercise_steps_and_progress_markers_with_target_object_success_criteria_hint_reveal_reset_skip_sandbox_or_preview_preference_and_privacy_bounded_resume_export_truth_across_claimed_m5_learnability_lanes::{
    GUIDED_EXERCISE_STEP_PROGRESS_MARKER_ARTIFACT_REF, GUIDED_EXERCISE_STEP_PROGRESS_MARKER_DOC_REF,
    GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF,
};
use crate::implement_learning_mode_toggles_and_tip_cards_with_user_workspace_scope_pause_snooze_reset_why_now_context_and_stable_command_file_docs_deep_link_truth_across_claimed_m5_onboarding_and_help_surfaces::{
    LEARNING_MODE_TOGGLE_TIP_CARD_ARTIFACT_REF, LEARNING_MODE_TOGGLE_TIP_CARD_DOC_REF,
    LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5LearningComponentConsumerPacket`].
pub const M5_LEARNING_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_onboarding_migration_contextual_help_docs_browser_feature_family_tour_companion_handoff_and_support_export_consumers_so_learning_components_keep_citation_privacy_and_progress_language_aligned_across_claimed_m5_profiles";

/// Schema version for M5 learning component-consumer records.
pub const M5_LEARNING_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the learning component-consumer boundary schema.
pub const M5_LEARNING_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-learning-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_LEARNING_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/help/m5_learning_component_consumers.md";

/// Repo-relative path of the frozen learning-component matrix this lane adopts from.
pub const M5_LEARNING_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_LEARNING_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_LEARNING_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str = M5_LEARNING_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_LEARNING_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-learning-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LEARNING_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-learning-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_LEARNING_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-learning-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_LEARNING_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-learning-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A consumer that
/// adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(family: M5LearningComponentFamily) -> &'static str {
    use M5LearningComponentFamily as Family;
    match family {
        Family::LearningModeToggle | Family::TipCard => LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF,
        Family::GuidedExerciseStep | Family::ProgressMarker => {
            GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF
        }
        Family::GlossaryChipOrCard | Family::SafeExplanationBanner => {
            GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(family: M5LearningComponentFamily) -> &'static str {
    use M5LearningComponentFamily as Family;
    match family {
        Family::LearningModeToggle | Family::TipCard => LEARNING_MODE_TOGGLE_TIP_CARD_DOC_REF,
        Family::GuidedExerciseStep | Family::ProgressMarker => {
            GUIDED_EXERCISE_STEP_PROGRESS_MARKER_DOC_REF
        }
        Family::GlossaryChipOrCard | Family::SafeExplanationBanner => {
            GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a family.
pub const fn family_canonical_artifact_ref(family: M5LearningComponentFamily) -> &'static str {
    use M5LearningComponentFamily as Family;
    match family {
        Family::LearningModeToggle | Family::TipCard => LEARNING_MODE_TOGGLE_TIP_CARD_ARTIFACT_REF,
        Family::GuidedExerciseStep | Family::ProgressMarker => {
            GUIDED_EXERCISE_STEP_PROGRESS_MARKER_ARTIFACT_REF
        }
        Family::GlossaryChipOrCard | Family::SafeExplanationBanner => {
            GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_ARTIFACT_REF
        }
    }
}

/// One claimed M5 learning consumer that adopts the shared components. These are the consumers the
/// spec names — onboarding, migration, contextual help, the docs / browser surface, a
/// feature-family tour, the companion handoff, and the support / export packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningComponentConsumer {
    /// The first-run onboarding surface.
    Onboarding,
    /// The migration-onboarding surface.
    Migration,
    /// The contextual-help surface.
    ContextualHelp,
    /// The docs / browser surface.
    DocsBrowser,
    /// The feature-family tour surface.
    FeatureFamilyTour,
    /// The companion-handoff surface.
    CompanionHandoff,
    /// The support / export packet.
    SupportExport,
}

impl M5LearningComponentConsumer {
    /// Every claimed learning consumer, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Onboarding,
        Self::Migration,
        Self::ContextualHelp,
        Self::DocsBrowser,
        Self::FeatureFamilyTour,
        Self::CompanionHandoff,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Onboarding => "onboarding",
            Self::Migration => "migration",
            Self::ContextualHelp => "contextual_help",
            Self::DocsBrowser => "docs_browser",
            Self::FeatureFamilyTour => "feature_family_tour",
            Self::CompanionHandoff => "companion_handoff",
            Self::SupportExport => "support_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Onboarding => "First-Run Onboarding",
            Self::Migration => "Migration Onboarding",
            Self::ContextualHelp => "Contextual Help",
            Self::DocsBrowser => "Docs / Browser",
            Self::FeatureFamilyTour => "Feature-Family Tour",
            Self::CompanionHandoff => "Companion Handoff",
            Self::SupportExport => "Support / Export Packet",
        }
    }

    /// True when this consumer is the support / export packet — the surface singled out for a
    /// canonical-schema reference so its prose can never drift from the product truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::SupportExport)
    }
}

/// The one shared descriptor vocabulary every learning component keeps aligned across surfaces, so
/// no consumer invents a new grammar or stale wording. The descriptors in
/// [`M5LearningComponentDescriptor::REQUIRED`] must be present on every binding — the
/// acceptance-criterion that citation, source-class, progress / privacy, and explain-versus-do
/// language stay one truth across guided and unguided teaching surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningComponentDescriptor {
    /// The cited file / symbol / doc source descriptor.
    CitationSource,
    /// The source-class and freshness descriptor (live / cached / stale / not-installed).
    SourceClassFreshness,
    /// The progress ownership / privacy descriptor (user-owned, default-local).
    ProgressOwnershipPrivacy,
    /// The explain-versus-do boundary descriptor.
    ExplainVersusDo,
}

impl M5LearningComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CitationSource,
        Self::SourceClassFreshness,
        Self::ProgressOwnershipPrivacy,
        Self::ExplainVersusDo,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CitationSource => "citation_source",
            Self::SourceClassFreshness => "source_class_freshness",
            Self::ProgressOwnershipPrivacy => "progress_ownership_privacy",
            Self::ExplainVersusDo => "explain_versus_do",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps the
/// descriptor vocabulary — it only discloses that parity is narrowed relative to the authoritative
/// rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningConsumerParityHealth {
    /// Full parity: the authoritative rendering.
    FullParity,
    /// A glossary / tip pack is served from a cached copy, so it is not the live pack.
    CachedPackNarrowed,
    /// The cited source content is stale, so its freshness is disclosed rather than assumed live.
    StaleSourceNarrowed,
    /// A cited source is unavailable or not installed, so the component cannot claim a live cited
    /// source.
    CitationUnavailableNarrowed,
    /// Progress is local-only because no supported sync / export path was chosen, so it is not
    /// carried beyond this device.
    ProgressLocalOnlyNarrowed,
}

impl M5LearningConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::CachedPackNarrowed,
        Self::StaleSourceNarrowed,
        Self::CitationUnavailableNarrowed,
        Self::ProgressLocalOnlyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::CachedPackNarrowed => "cached_pack_narrowed",
            Self::StaleSourceNarrowed => "stale_source_narrowed",
            Self::CitationUnavailableNarrowed => "citation_unavailable_narrowed",
            Self::ProgressLocalOnlyNarrowed => "progress_local_only_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5LearningConsumerNarrowingReason> {
        Some(match self {
            Self::CachedPackNarrowed => M5LearningConsumerNarrowingReason::CachedPackServed,
            Self::StaleSourceNarrowed => M5LearningConsumerNarrowingReason::SourceContentStale,
            Self::CitationUnavailableNarrowed => {
                M5LearningConsumerNarrowingReason::CitedSourceUnavailableOrNotInstalled
            }
            Self::ProgressLocalOnlyNarrowed => M5LearningConsumerNarrowingReason::ProgressLocalOnly,
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow banner
/// never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningConsumerNarrowingReason {
    /// A glossary / tip pack is served from a cached copy, so it is not the live pack.
    CachedPackServed,
    /// The cited source content is stale, so its freshness is disclosed rather than assumed live.
    SourceContentStale,
    /// A cited source is unavailable or not installed, so the component cannot claim a live cited
    /// source.
    CitedSourceUnavailableOrNotInstalled,
    /// Progress is local-only because no supported sync / export path was chosen.
    ProgressLocalOnly,
}

impl M5LearningConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CachedPackServed,
        Self::SourceContentStale,
        Self::CitedSourceUnavailableOrNotInstalled,
        Self::ProgressLocalOnly,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CachedPackServed => "cached_pack_served",
            Self::SourceContentStale => "source_content_stale",
            Self::CitedSourceUnavailableOrNotInstalled => {
                "cited_source_unavailable_or_not_installed"
            }
            Self::ProgressLocalOnly => "progress_local_only",
        }
    }

    /// True when the reason reflects an uncited or unavailable source that must never masquerade as
    /// a live, cited one — the acceptance-criterion boundary for a cited source that is unavailable
    /// or not installed.
    pub const fn is_uncited_or_unavailable(self) -> bool {
        matches!(self, Self::CitedSourceUnavailableOrNotInstalled)
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::CachedPackServed => {
                "a glossary / tip pack is served from a cached copy, so this is cached content and not the live pack"
            }
            Self::SourceContentStale => {
                "the cited source content is stale, so its freshness is disclosed rather than assumed live"
            }
            Self::CitedSourceUnavailableOrNotInstalled => {
                "a cited source is unavailable or not installed, so this cannot claim a live cited source and stays a disclosed gap"
            }
            Self::ProgressLocalOnly => {
                "progress is local-only because no supported sync / export path was chosen, so it stays on this device and is not carried elsewhere"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5LearningConsumerRecoveryAction {
        match self {
            Self::CachedPackServed => M5LearningConsumerRecoveryAction::RefreshPackWhenOnline,
            Self::SourceContentStale => {
                M5LearningConsumerRecoveryAction::ReviewSourceFreshnessBeforeTrusting
            }
            Self::CitedSourceUnavailableOrNotInstalled => {
                M5LearningConsumerRecoveryAction::OpenCitedSourceOrRequestAccess
            }
            Self::ProgressLocalOnly => {
                M5LearningConsumerRecoveryAction::ExportProgressOrEnableSupportedSync
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable from
/// the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningConsumerRecoveryAction {
    /// Refresh the pack when back online rather than trusting the cached copy.
    RefreshPackWhenOnline,
    /// Review the source freshness before treating stale content as live.
    ReviewSourceFreshnessBeforeTrusting,
    /// Open the cited source, or request access, before trusting an unavailable citation.
    OpenCitedSourceOrRequestAccess,
    /// Export progress, or enable a supported sync path, before trusting it beyond this device.
    ExportProgressOrEnableSupportedSync,
}

impl M5LearningConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RefreshPackWhenOnline,
        Self::ReviewSourceFreshnessBeforeTrusting,
        Self::OpenCitedSourceOrRequestAccess,
        Self::ExportProgressOrEnableSupportedSync,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RefreshPackWhenOnline => "refresh_pack_when_online",
            Self::ReviewSourceFreshnessBeforeTrusting => "review_source_freshness_before_trusting",
            Self::OpenCitedSourceOrRequestAccess => "open_cited_source_or_request_access",
            Self::ExportProgressOrEnableSupportedSync => "export_progress_or_enable_supported_sync",
        }
    }
}

/// An export caveat a consumer preserves when a component renders below full parity (a cached pack,
/// stale source content, an unavailable / not-installed cited source, or local-only progress).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningConsumerExportCaveat {
    /// The content was served from a cached pack, not the live pack.
    ContentServedFromCachedPack,
    /// The cited source content is stale.
    SourceContentStale,
    /// The cited source is unavailable or not installed.
    CitedSourceUnavailableOrNotInstalled,
    /// Progress is local-only and not synced beyond this device.
    ProgressLocalOnlyNotSynced,
}

impl M5LearningConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ContentServedFromCachedPack,
        Self::SourceContentStale,
        Self::CitedSourceUnavailableOrNotInstalled,
        Self::ProgressLocalOnlyNotSynced,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContentServedFromCachedPack => "content_served_from_cached_pack",
            Self::SourceContentStale => "source_content_stale",
            Self::CitedSourceUnavailableOrNotInstalled => {
                "cited_source_unavailable_or_not_installed"
            }
            Self::ProgressLocalOnlyNotSynced => "progress_local_only_not_synced",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is kept
/// aligned as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningClaimParityState {
    /// The descriptor vocabulary is kept aligned at full parity.
    ClaimsAligned,
    /// The descriptor vocabulary is kept aligned, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5LearningClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsAligned, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsAligned => "claims_aligned",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5LearningConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningConsumerAnatomyPart {
    /// The adopted component identity.
    ComponentIdentity,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The shared descriptor set.
    DescriptorSet,
    /// The parity-health cue.
    ParityHealthCue,
    /// The export-caveat list.
    ExportCaveats,
    /// The derived claim-parity verdict.
    ClaimParityVerdict,
    /// The auto-narrow banner (shown when narrowed).
    AutoNarrowBanner,
}

impl M5LearningConsumerAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealthCue,
        Self::ExportCaveats,
        Self::ClaimParityVerdict,
        Self::AutoNarrowBanner,
    ];

    /// The anatomy parts every consumer projection must render.
    pub const MANDATORY: [Self; 4] = [
        Self::ComponentIdentity,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityVerdict,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentIdentity => "component_identity",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealthCue => "parity_health_cue",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityVerdict => "claim_parity_verdict",
            Self::AutoNarrowBanner => "auto_narrow_banner",
        }
    }
}

/// A field the support / export packet carries so consumer parity is reconstructable from the
/// shared model. The fields in [`M5LearningConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LearningConsumerExportField {
    /// The consumer identity.
    Consumer,
    /// The adopted component family.
    ComponentFamily,
    /// The canonical schema reference.
    CanonicalSchemaRef,
    /// The descriptor set.
    DescriptorSet,
    /// The parity-health mode.
    ParityHealth,
    /// The export caveats.
    ExportCaveats,
    /// The claim-parity state.
    ClaimParityState,
    /// The narrowing reason (when narrowed).
    NarrowingReason,
}

impl M5LearningConsumerExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ParityHealth,
        Self::ExportCaveats,
        Self::ClaimParityState,
        Self::NarrowingReason,
    ];

    /// The export fields every consumer export must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::Consumer,
        Self::ComponentFamily,
        Self::CanonicalSchemaRef,
        Self::DescriptorSet,
        Self::ClaimParityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Consumer => "consumer",
            Self::ComponentFamily => "component_family",
            Self::CanonicalSchemaRef => "canonical_schema_ref",
            Self::DescriptorSet => "descriptor_set",
            Self::ParityHealth => "parity_health",
            Self::ExportCaveats => "export_caveats",
            Self::ClaimParityState => "claim_parity_state",
            Self::NarrowingReason => "narrowing_reason",
        }
    }
}

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay preserved, the
/// export caveats, and the recovery action, so a narrowed rendering is understood from the banner
/// alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5LearningConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5LearningConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5LearningComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5LearningComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5LearningComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5LearningConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved descriptors, and
    /// the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the learning component-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5LearningComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5LearningComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so citation,
    /// source-class, progress / privacy, and explain-versus-do stay explicit.
    pub descriptor_families: Vec<M5LearningComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5LearningConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5LearningConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5LearningComponentConsumer,
    /// The component family.
    pub component_family: M5LearningComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5LearningComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5LearningConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5LearningConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5LearningClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects an uncited or unavailable source (a cited source that is
    /// unavailable or not installed). Such a binding must always be narrowed and never asserts a
    /// live, cited source.
    pub reflects_uncited_or_unavailable_source: bool,
    /// Hard invariant: whether this binding claims a live, cited source at full parity. Only a
    /// full-parity binding may assert live cited parity; every narrowed binding — and in particular
    /// any uncited or unavailable one — resolves this to `false`.
    pub asserts_live_cited_parity: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5LearningComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_learning_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5LearningComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5LearningComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5LearningComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "learning component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5LearningComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that citation,
/// source-class, progress / privacy, and explain-versus-do stay explicit on every surface. The
/// claim-parity state is kept aligned at full parity and auto-narrowed under any weakened
/// parity-health mode, and a weakened mode always produces a self-contained banner naming the exact
/// reason and recovery action while keeping the descriptor vocabulary intact. An uncited or
/// unavailable source (a cited source that is unavailable or not installed) always narrows and
/// never asserts a live, cited source.
pub fn resolve_learning_component_binding(
    input: &M5LearningComponentBindingInput,
) -> Result<M5LearningComponentResolvedBinding, M5LearningComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5LearningComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5LearningComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5LearningComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5LearningComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5LearningComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text extension
        // from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5LearningComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_uncited_or_unavailable_source =
        narrowing_reason.is_some_and(M5LearningConsumerNarrowingReason::is_uncited_or_unavailable);
    // Only a full-parity binding may assert a live, cited source. Every narrowed binding — and
    // every uncited / unavailable one in particular — is not live-cited.
    let asserts_live_cited_parity = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5LearningClaimParityState::ClaimsAutoNarrowed
    } else {
        M5LearningClaimParityState::ClaimsAligned
    };

    let auto_narrow_banner = narrowing_reason.map(|reason| {
        let recovery_action = reason.recovery_action();
        let headline = format!(
            "Claim auto-narrowed: {} — {} renders {} with {} descriptor(s) preserved; recovery: {}",
            reason.phrase(),
            input.consumer.as_str(),
            input.component_family.as_str(),
            input.descriptor_families.len(),
            recovery_action.as_str()
        );
        M5LearningComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5LearningComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_uncited_or_unavailable_source,
        asserts_live_cited_parity,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs
/// consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentBindingCase {
    /// The resolver input.
    pub input: M5LearningComponentBindingInput,
    /// The resolved truth. Must equal `resolve_learning_component_binding(&input)`.
    pub resolved: M5LearningComponentResolvedBinding,
}

impl M5LearningComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5LearningComponentBindingInput) -> Self {
        let resolved =
            resolve_learning_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_learning_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the consumer
/// points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5LearningComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical schema
    /// ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the family's
    /// canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local re-description of
    /// its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5LearningComponentBindingCase>,
}

impl M5LearningComponentBinding {
    /// True when the binding points at the family's canonical refs and references the canonical
    /// family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one learning consumer bound to the canonical component families,
/// the shared descriptor vocabulary, the parity-health modes, export caveats, parity states,
/// narrowing reasons, recovery actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentConsumerRow {
    /// Learning consumer.
    pub consumer: M5LearningComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5LearningQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 learning surface families that render / consume this projection.
    pub surface_families: Vec<M5LearningSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5LearningDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5LearningConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5LearningComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5LearningConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5LearningConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5LearningClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5LearningConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5LearningConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5LearningConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5LearningAccessibilityRoute>,
    /// Learning subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5LearningConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5LearningDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5LearningComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new learning grammar. MUST be `false`.
    pub invents_new_learning_grammar: bool,
    /// Hard invariant: this consumer never drops citation, source-class, progress / privacy, or
    /// explain-versus-do truth when narrowed. MUST be `false`.
    pub drops_citation_progress_or_explain_do_when_narrowed: bool,
    /// Hard invariant: this consumer never shows an uncited or unavailable source as a live, cited
    /// one. MUST be `false`.
    pub shows_uncited_or_unavailable_source_as_live_cited: bool,
    /// Hard invariant: this consumer never widens trust or mutating authority through a learning
    /// component. MUST be `false`.
    pub widens_trust_or_mutating_authority: bool,
}

impl M5LearningComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5LearningConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5LearningConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5LearningConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5LearningConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5LearningComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5LearningComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5LearningComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5LearningComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_learning_grammar
            && !self.drops_citation_progress_or_explain_do_when_narrowed
            && !self.shows_uncited_or_unavailable_source_as_live_cited
            && !self.widens_trust_or_mutating_authority
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentConsumerVocabularySet {
    /// Learning-consumer tokens.
    pub consumers: Vec<String>,
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Descriptor tokens.
    pub descriptors: Vec<String>,
    /// Parity-health-mode tokens.
    pub parity_health_modes: Vec<String>,
    /// Export-caveat tokens.
    pub export_caveats: Vec<String>,
    /// Narrowing-reason tokens.
    pub narrowing_reasons: Vec<String>,
    /// Recovery-action tokens.
    pub recovery_actions: Vec<String>,
    /// Claim-parity-state tokens.
    pub claim_parity_states: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5LearningComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5LearningComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5LearningComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5LearningComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5LearningConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5LearningConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5LearningConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5LearningConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5LearningClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5LearningConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5LearningConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5LearningAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5LearningComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new learning grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Citation, source-class, progress / privacy, and explain-versus-do stay explicit everywhere.
    pub citation_progress_and_explain_do_explicit_on_every_surface: bool,
    /// Cached packs, stale source content, unavailable citations, and local-only progress
    /// auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// An uncited or unavailable source never masquerades as a live, cited one.
    pub uncited_or_unavailable_source_never_shown_as_live_cited: bool,
    /// The support / export packet presents the same learning truth shown in-product.
    pub support_export_presents_same_learning_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentConsumerProjection {
    /// Onboarding, migration, contextual help, docs / browser, the feature-family tour, the
    /// companion handoff, and the support / export packet all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The citation-source descriptor reads a single canonical source.
    pub citation_source_reads_single_source: bool,
    /// The source-class / freshness descriptor reads a single canonical source.
    pub source_class_freshness_reads_single_source: bool,
    /// The progress ownership / privacy descriptor reads a single canonical source.
    pub progress_ownership_privacy_reads_single_source: bool,
    /// The explain-versus-do descriptor reads a single canonical source.
    pub explain_versus_do_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting learning-component consumer audit.
    pub learning_component_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5LearningComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LearningComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5LearningComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LearningComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LearningComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LearningComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LearningComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LearningComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 learning component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LearningComponentConsumerPacket {
    /// Record kind; must equal [`M5_LEARNING_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LEARNING_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5LearningComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LearningComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LearningComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LearningComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LearningComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LearningComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LearningComponentConsumerPacket {
    /// Builds an M5 learning component-consumer packet from stable-lane input.
    pub fn new(input: M5LearningComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_LEARNING_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_LEARNING_COMPONENT_CONSUMER_SCHEMA_VERSION,
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

    /// Validates the M5 learning component-consumer invariants.
    pub fn validate(&self) -> Vec<M5LearningComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LEARNING_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5LearningComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LEARNING_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5LearningComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LearningComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_live_cited_honesty(self, &mut violations);
        validate_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 learning component consumer packet serializes"),
        ) {
            violations.push(M5LearningComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 learning component consumer packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer,qualification,owner,adopted_families,parity_health_modes,claim_parity_states,narrowing_reasons,export_fields,binding_count\n",
        );
        for row in &self.consumer_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.consumer.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.component_bindings, |b| b.component_family.as_str()),
                join_tokens(&row.parity_health_modes, |v| v.as_str()),
                join_tokens(&row.claim_parity_states, |v| v.as_str()),
                join_tokens(&row.narrowing_reasons, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.component_bindings.len(),
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
        out.push_str("# M5 Learning Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Learning consumers: {} ({} stable)\n",
            self.consumer_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Component families: {}\n",
            self.vocabulary_set.component_families.join(", ")
        ));
        out.push_str(&format!(
            "- Descriptors: {}\n",
            self.vocabulary_set.descriptors.join(", ")
        ));
        out.push_str(&format!(
            "- Parity-health modes: {}\n",
            self.vocabulary_set.parity_health_modes.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Learning consumers\n\n");
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
                row.component_bindings.len()
            ));
            for binding in &row.component_bindings {
                out.push_str(&format!(
                    "    - `{}` → `{}` ({} worked binding(s))\n",
                    binding.component_family.as_str(),
                    binding.canonical_schema_ref,
                    binding.example_bindings.len()
                ));
                for case in &binding.example_bindings {
                    let banner = match &case.resolved.auto_narrow_banner {
                        Some(banner) => banner.reason.as_str(),
                        None => "full",
                    };
                    out.push_str(&format!(
                        "      - `{}` → `{}` (banner `{}`)\n",
                        case.resolved.parity_health.as_str(),
                        case.resolved.claim_parity_state.as_str(),
                        banner
                    ));
                }
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 learning component-consumer export.
#[derive(Debug)]
pub enum M5LearningComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LearningComponentConsumerViolation>),
}

impl fmt::Display for M5LearningComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 learning component consumer export parse failed: {error}"
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
                    "m5 learning component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LearningComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5LearningComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LearningComponentConsumerViolation {
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
    /// A required learning consumer is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    ConsumerRowIncomplete,
    /// A consumer row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A consumer row does not keep every required descriptor.
    RequiredDescriptorMissing,
    /// A consumer row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A consumer row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A consumer row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A consumer row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A consumer row declares no component bindings.
    ComponentBindingMissing,
    /// A component binding does not point to its canonical family.
    CanonicalRefMismatch,
    /// A component binding declares no worked binding cases.
    ExampleBindingMissing,
    /// A worked binding case does not match a fresh resolve of its input.
    ExampleBindingDrift,
    /// A consumer claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// A required component family is never adopted, or is adopted by only one consumer (reuse
    /// across surfaces unproven).
    ComponentFamilyReuseUnproven,
    /// No worked binding proves a narrowed rendering with a self-contained banner.
    NarrowingDisclosureUnproven,
    /// No worked binding proves a full-parity rendering with preserved parity and no banner.
    ScopePreservedUnproven,
    /// No worked binding proves that an uncited or unavailable source narrows and never asserts a
    /// live, cited source, or a binding does so incorrectly.
    LiveCitedHonestyUnproven,
    /// The support / export packet consumer does not reference the canonical component schema.
    SupportExportReferenceMissing,
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

impl M5LearningComponentConsumerViolation {
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
            Self::RequiredDescriptorMissing => "required_descriptor_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ComponentBindingMissing => "component_binding_missing",
            Self::CanonicalRefMismatch => "canonical_ref_mismatch",
            Self::ExampleBindingMissing => "example_binding_missing",
            Self::ExampleBindingDrift => "example_binding_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ComponentFamilyReuseUnproven => "component_family_reuse_unproven",
            Self::NarrowingDisclosureUnproven => "narrowing_disclosure_unproven",
            Self::ScopePreservedUnproven => "scope_preserved_unproven",
            Self::LiveCitedHonestyUnproven => "live_cited_honesty_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::ConsumerInvariantViolated => "consumer_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 learning component-consumer export.
pub fn current_stable_m5_learning_component_consumer_export(
) -> Result<M5LearningComponentConsumerPacket, M5LearningComponentConsumerArtifactError> {
    let packet: M5LearningComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-learning-component-consumer-proof/support_export.json"
    )))
    .map_err(M5LearningComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LearningComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LEARNING_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_LEARNING_COMPONENT_CONSUMER_DOC_REF,
        M5_LEARNING_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_LEARNING_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        LEARNING_MODE_TOGGLE_TIP_CARD_SCHEMA_REF,
        GUIDED_EXERCISE_STEP_PROGRESS_MARKER_SCHEMA_REF,
        GLOSSARY_CHIP_CARD_SAFE_EXPLANATION_BANNER_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LearningComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5LearningComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    let present: BTreeSet<M5LearningComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5LearningComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5LearningComponentConsumerViolation::RequiredConsumerMissing);
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
            || row.parity_health_modes.is_empty()
            || row.export_caveats.is_empty()
            || row.claim_parity_states.is_empty()
            || row.narrowing_reasons.is_empty()
            || row.recovery_actions.is_empty()
        {
            violations.push(M5LearningComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5LearningComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5LearningComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5LearningComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5LearningAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5LearningComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5LearningComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5LearningComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5LearningComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5LearningComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5LearningComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5LearningComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5LearningComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5LearningComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable components rather than one onboarding
/// page plus a few isolated help objects.
fn validate_family_reuse(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    for family in M5LearningComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5LearningComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner
/// carries a specific reason, a recovery action, and a non-empty set of preserved descriptors — the
/// acceptance-criterion example that a consumer which cannot preserve parity is visibly narrowed
/// rather than silently dropping citation, progress, or explain-versus-do language.
fn validate_narrowing_disclosure(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        case.resolved.is_narrowed
            && case
                .resolved
                .auto_narrow_banner
                .as_ref()
                .is_some_and(|banner| {
                    !banner.headline.trim().is_empty() && !banner.preserved_descriptors.is_empty()
                })
    });
    if !proven {
        violations.push(M5LearningComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with preserved
/// parity and no banner — the acceptance-criterion example that full-parity consumers keep the
/// descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5LearningClaimParityState::ClaimsAligned
    });
    if !proven {
        violations.push(M5LearningComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every worked binding that reflects an uncited or unavailable source must be narrowed and must
/// not assert a live, cited source, and at least one such binding must be present — the
/// acceptance-criterion that an uncited or unavailable source no longer masquerades as a live,
/// cited one on any claimed consumer.
fn validate_live_cited_honesty(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_uncited_or_unavailable_source {
            // An uncited / unavailable binding that claims live cited parity, or fails to narrow,
            // breaks the acceptance criterion.
            if resolved.asserts_live_cited_parity
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5LearningClaimParityState::ClaimsAutoNarrowed
            {
                violations.push(M5LearningComponentConsumerViolation::LiveCitedHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5LearningComponentConsumerViolation::LiveCitedHonestyUnproven);
    }
}

/// The support / export packet consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that a support / export lane can never drift from
/// the product truth.
fn validate_support_export_reference(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5LearningComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5LearningComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.citation_progress_and_explain_do_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.uncited_or_unavailable_source_never_shown_as_live_cited,
        review.support_export_presents_same_learning_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5LearningComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.citation_source_reads_single_source,
        projection.source_class_freshness_reads_single_source,
        projection.progress_ownership_privacy_reads_single_source,
        projection.explain_versus_do_reads_single_source,
    ] {
        if !ok {
            violations.push(M5LearningComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LearningComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5LearningComponentConsumerPacket,
    violations: &mut Vec<M5LearningComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture
            .learning_component_consumer_audit_ref
            .trim()
            .is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5LearningComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5LearningComponentConsumerPacket,
) -> impl Iterator<Item = &M5LearningComponentBindingCase> {
    packet
        .consumer_rows
        .iter()
        .flat_map(|row| row.component_bindings.iter())
        .flat_map(|binding| binding.example_bindings.iter())
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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
