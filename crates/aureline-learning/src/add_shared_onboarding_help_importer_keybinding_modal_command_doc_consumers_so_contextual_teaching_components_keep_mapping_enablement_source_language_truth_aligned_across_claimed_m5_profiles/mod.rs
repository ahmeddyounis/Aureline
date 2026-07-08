//! Shared consumers for the reusable M5 contextual-teaching / migration-bridge components,
//! so the contextual-tip card, migration-bridge card, sequence-help strip, why-unavailable
//! explanation row, and source-language fallback surface keep command-binding,
//! migration-mapping, blocked-action-explanation, and source-language-citation language
//! aligned across every claimed M5 teaching surface where a user runs first-run onboarding,
//! walks a migration importer, reads keybinding / leader help, reads command docs, opens a
//! Help pane, or exports a localized support packet.
//!
//! Aureline's frozen contextual-teaching / migration-bridge component matrix
//! (`crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`)
//! names the five governed component families, and four sibling implement lanes narrow those
//! families into working primitives, each with its own canonical schema, contract doc, and
//! support-export artifact:
//!
//! * the contextual-tip card (`implement_contextual_tip_cards_...`),
//! * the migration-bridge card (`ship_migration_bridge_cards_...`),
//! * the sequence-help strip (`implement_sequence_help_strips_...`), and
//! * the why-unavailable explanation row and source-language fallback surface
//!   (`implement_why_unavailable_explanation_rows_and_source_language_fallback_surfaces_...`).
//!
//! This module is the *adoption* lane over those primitives. It proves the five families are
//! reusable components — not one onboarding page plus a few isolated help objects — by binding
//! every claimed M5 teaching consumer (first-run onboarding, the migration importer,
//! keybinding / leader help, command docs, the Help pane, and the localized support packet)
//! to the same canonical component schemas and the same descriptor vocabulary. Each consumer
//! points at the primitive's canonical schema and support-export artifact rather than
//! re-wording command-binding, migration-mapping, blocked-action, or source-language facts in
//! local prose, and each keeps that vocabulary truthful even when imported behavior is only
//! partially mapped, a command-language sequence is unsupported, a blocked-action owner
//! changes, or localized fallback content is stale or policy-limited.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_teaching_component_binding`] — that takes one consumer's adoption
//!    of one component family, the descriptor set it surfaces, the parity-health mode it
//!    renders under, and any export caveats, and produces one
//!    [`M5TeachingComponentResolvedBinding`] carrying the derived claim-parity state and —
//!    whenever parity is weakened — a self-contained [`M5TeachingComponentAutoNarrowBanner`]
//!    that names the exact reason (imported behavior only partially mapped, an unsupported
//!    sequence, a changed blocked-action owner, or stale / policy-limited localized fallback),
//!    the descriptors that stay preserved, and the recovery action, rather than a generic
//!    "degraded" note. The resolver never lets a narrowed context drop a required descriptor
//!    and never lets partial or unsupported state masquerade as exact teaching parity.
//! 2. A parity matrix — [`M5TeachingComponentConsumerPacket`] — that binds one row per claimed
//!    M5 teaching consumer to the five canonical component families, the one shared descriptor
//!    vocabulary, the same parity-health modes, export caveats, parity states, narrowing
//!    reasons, recovery actions, export fields, and non-visual accessibility routes, so
//!    command-binding / migration-mapping / blocked-action-explanation / source-language-citation
//!    facts stop diverging between the product UI, the docs, and the support artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, downgrade triggers, and the five component families themselves are
//! reused verbatim from the frozen contextual-teaching / migration-bridge component matrix.
//! This module mints new vocabulary only for what the adoption lane itself needs: its teaching
//! consumers, the shared descriptor vocabulary, the parity-health modes, the export caveats,
//! the claim-parity states, the narrowing reasons and recovery actions, the consumer anatomy
//! parts, and the export fields.
//!
//! Raw secrets, endpoints, tokens, and raw provider bodies stay outside the support boundary;
//! every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! `schemas/ui/m5-contextual-teaching-component-consumer.schema.json` and the contract doc is
//! `docs/help/m5_contextual_teaching_component_consumers.md`. The protected fixture directory
//! is `fixtures/ui/m5-contextual-teaching-component-consumers/`.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_teaching_component_consumer_help_pane_preview_narrowed,
    seeded_m5_teaching_component_consumer_migration_importer_beta_narrowed,
    seeded_m5_teaching_component_consumer_packet, M5_TEACHING_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, downgrade triggers, and the five component families are frozen once,
// in the contextual-teaching / migration-bridge component matrix. This adoption lane reuses
// them verbatim so it never invents a parallel teaching vocabulary.
pub use crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::{
    M5ContextualTeachingComponentFamily, M5TeachingAccessibilityRoute, M5TeachingConsumerSurface,
    M5TeachingDeploymentLine, M5TeachingDowngradeTrigger, M5TeachingQualificationClass,
    M5TeachingSurfaceFamily,
};

// The canonical matrix schema / doc refs this adoption lane points every consumer at, rather
// than re-wording their facts in local prose.
use crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::{
    M5_CONTEXTUAL_TEACHING_COMPONENT_DOC_REF, M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
};
// The canonical primitive schema / doc / artifact refs each family maps to.
use crate::implement_contextual_tip_cards_with_why_now_relevance_concrete_next_action_stable_command_reference_and_try_open_docs_snooze_dismiss_actions_that_respect_quiet_hours_presentation_mode_and_recent_dismissals_across_claimed_m5_learnability_surfaces::{
    M5_CONTEXTUAL_TIP_CARD_ARTIFACT_REF, M5_CONTEXTUAL_TIP_CARD_DOC_REF,
    M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF,
};
use crate::implement_sequence_help_strips_with_current_mode_next_key_guidance_cancel_hints_and_keyboard_only_parity_across_claimed_m5_modal_and_command_language_surfaces::{
    M5_SEQUENCE_HELP_STRIP_ARTIFACT_REF, M5_SEQUENCE_HELP_STRIP_DOC_REF,
    M5_SEQUENCE_HELP_STRIP_SCHEMA_REF,
};
use crate::implement_why_unavailable_explanation_rows_and_source_language_fallback_surfaces_with_owner_reason_next_safe_action_truth_and_citation_preserving_help_parity_across_claimed_m5_blocked_action_and_localized_surfaces::{
    M5_BLOCKED_LOCALIZED_ROW_ARTIFACT_REF, M5_BLOCKED_LOCALIZED_ROW_DOC_REF,
    M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF,
};
use crate::ship_migration_bridge_cards_with_old_path_new_command_mapping_native_bridge_shimmed_partial_states_and_undo_import_parity_across_claimed_m5_importer_and_migration_surfaces::{
    M5_MIGRATION_BRIDGE_CARD_ARTIFACT_REF, M5_MIGRATION_BRIDGE_CARD_DOC_REF,
    M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5TeachingComponentConsumerPacket`].
pub const M5_TEACHING_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_onboarding_help_importer_keybinding_modal_command_doc_consumers_so_contextual_teaching_components_keep_mapping_enablement_source_language_truth_aligned_across_claimed_m5_profiles";

/// Schema version for M5 contextual-teaching component-consumer records.
pub const M5_TEACHING_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the teaching component-consumer boundary schema.
pub const M5_TEACHING_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-contextual-teaching-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_TEACHING_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/help/m5_contextual_teaching_component_consumers.md";

/// Repo-relative path of the frozen contextual-teaching component matrix this lane adopts
/// from.
pub const M5_TEACHING_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_TEACHING_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str =
    M5_CONTEXTUAL_TEACHING_COMPONENT_DOC_REF;

/// Repo-relative path of the protected fixture directory.
pub const M5_TEACHING_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-contextual-teaching-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TEACHING_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_TEACHING_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_TEACHING_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-contextual-teaching-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the narrowed primitive that owns a family. A consumer
/// that adopts a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(
    family: M5ContextualTeachingComponentFamily,
) -> &'static str {
    use M5ContextualTeachingComponentFamily as Family;
    match family {
        Family::ContextualTipCard => M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF,
        Family::MigrationBridgeCard => M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF,
        Family::SequenceHelpStrip => M5_SEQUENCE_HELP_STRIP_SCHEMA_REF,
        Family::WhyUnavailableExplanationRow | Family::SourceLanguageFallback => {
            M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF
        }
    }
}

/// The canonical contract-doc ref of the narrowed primitive that owns a family.
pub const fn family_canonical_doc_ref(family: M5ContextualTeachingComponentFamily) -> &'static str {
    use M5ContextualTeachingComponentFamily as Family;
    match family {
        Family::ContextualTipCard => M5_CONTEXTUAL_TIP_CARD_DOC_REF,
        Family::MigrationBridgeCard => M5_MIGRATION_BRIDGE_CARD_DOC_REF,
        Family::SequenceHelpStrip => M5_SEQUENCE_HELP_STRIP_DOC_REF,
        Family::WhyUnavailableExplanationRow | Family::SourceLanguageFallback => {
            M5_BLOCKED_LOCALIZED_ROW_DOC_REF
        }
    }
}

/// The canonical support-export artifact ref of the narrowed primitive that owns a family.
pub const fn family_canonical_artifact_ref(
    family: M5ContextualTeachingComponentFamily,
) -> &'static str {
    use M5ContextualTeachingComponentFamily as Family;
    match family {
        Family::ContextualTipCard => M5_CONTEXTUAL_TIP_CARD_ARTIFACT_REF,
        Family::MigrationBridgeCard => M5_MIGRATION_BRIDGE_CARD_ARTIFACT_REF,
        Family::SequenceHelpStrip => M5_SEQUENCE_HELP_STRIP_ARTIFACT_REF,
        Family::WhyUnavailableExplanationRow | Family::SourceLanguageFallback => {
            M5_BLOCKED_LOCALIZED_ROW_ARTIFACT_REF
        }
    }
}

/// One claimed M5 teaching consumer that adopts the shared components. These are the consumers
/// the spec names — first-run onboarding, the migration importer, keybinding / leader help,
/// command docs, the Help pane, and the localized support packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingComponentConsumer {
    /// The first-run onboarding surface.
    OnboardingFlow,
    /// The migration importer flow.
    MigrationImporter,
    /// The keybinding / leader-overlay help surface.
    KeybindingLeaderHelp,
    /// The command-docs surface.
    CommandDocs,
    /// The Help pane.
    HelpPane,
    /// The localized support / export packet.
    LocalizedSupportPacket,
}

impl M5TeachingComponentConsumer {
    /// Every claimed teaching consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OnboardingFlow,
        Self::MigrationImporter,
        Self::KeybindingLeaderHelp,
        Self::CommandDocs,
        Self::HelpPane,
        Self::LocalizedSupportPacket,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnboardingFlow => "onboarding_flow",
            Self::MigrationImporter => "migration_importer",
            Self::KeybindingLeaderHelp => "keybinding_leader_help",
            Self::CommandDocs => "command_docs",
            Self::HelpPane => "help_pane",
            Self::LocalizedSupportPacket => "localized_support_packet",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::OnboardingFlow => "First-Run Onboarding",
            Self::MigrationImporter => "Migration Importer",
            Self::KeybindingLeaderHelp => "Keybinding / Leader Help",
            Self::CommandDocs => "Command Docs",
            Self::HelpPane => "Help Pane",
            Self::LocalizedSupportPacket => "Localized Support Packet",
        }
    }

    /// True when this consumer is the localized support packet — the surface singled out for a
    /// canonical-schema reference so its prose can never drift from the product truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::LocalizedSupportPacket)
    }
}

/// The one shared descriptor vocabulary every contextual-teaching component keeps aligned
/// across surfaces, so no consumer invents a new grammar or stale wording. The descriptors in
/// [`M5TeachingComponentDescriptor::REQUIRED`] must be present on every binding — the
/// acceptance-criterion that command binding, migration mapping, blocked-action explanation,
/// and source-language citation stay one truth across in-product and exported teaching
/// surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingComponentDescriptor {
    /// The stable command binding / command reference descriptor.
    CommandBinding,
    /// The migration-mapping class (exact / native / bridge / shimmed / partial / unsupported)
    /// descriptor.
    MigrationMapping,
    /// The blocked-action owner / reason / next-safe-action descriptor.
    BlockedActionExplanation,
    /// The source-language class / canonical-citation descriptor.
    SourceLanguageCitation,
}

impl M5TeachingComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CommandBinding,
        Self::MigrationMapping,
        Self::BlockedActionExplanation,
        Self::SourceLanguageCitation,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandBinding => "command_binding",
            Self::MigrationMapping => "migration_mapping",
            Self::BlockedActionExplanation => "blocked_action_explanation",
            Self::SourceLanguageCitation => "source_language_citation",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps the
/// descriptor vocabulary — it only discloses that parity is narrowed relative to the
/// authoritative rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingConsumerParityHealth {
    /// Full parity: the authoritative rendering.
    FullParity,
    /// Imported behavior is only partially mapped, so the migration bridge is not exact.
    ImportedBehaviorPartialNarrowed,
    /// A command-language sequence is unsupported here, so no backing command completes it.
    SequenceUnsupportedNarrowed,
    /// The blocked-action owner changed, so owner / next-action truth is re-resolved.
    BlockedOwnerChangedNarrowed,
    /// The localized fallback content is stale or policy-limited, so it falls back to source.
    LocalizedFallbackStaleNarrowed,
}

impl M5TeachingConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::ImportedBehaviorPartialNarrowed,
        Self::SequenceUnsupportedNarrowed,
        Self::BlockedOwnerChangedNarrowed,
        Self::LocalizedFallbackStaleNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::ImportedBehaviorPartialNarrowed => "imported_behavior_partial_narrowed",
            Self::SequenceUnsupportedNarrowed => "sequence_unsupported_narrowed",
            Self::BlockedOwnerChangedNarrowed => "blocked_owner_changed_narrowed",
            Self::LocalizedFallbackStaleNarrowed => "localized_fallback_stale_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5TeachingConsumerNarrowingReason> {
        Some(match self {
            Self::ImportedBehaviorPartialNarrowed => {
                M5TeachingConsumerNarrowingReason::ImportedBehaviorPartial
            }
            Self::SequenceUnsupportedNarrowed => {
                M5TeachingConsumerNarrowingReason::SequenceUnsupported
            }
            Self::BlockedOwnerChangedNarrowed => {
                M5TeachingConsumerNarrowingReason::BlockedActionOwnerChanged
            }
            Self::LocalizedFallbackStaleNarrowed => {
                M5TeachingConsumerNarrowingReason::LocalizedFallbackStaleOrPolicyLimited
            }
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow banner
/// never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingConsumerNarrowingReason {
    /// Imported behavior is only partially mapped, so the migration bridge is not an exact
    /// native equivalent.
    ImportedBehaviorPartial,
    /// The command-language sequence is unsupported here, so no backing command completes it.
    SequenceUnsupported,
    /// The blocked-action owner changed, so the named owner and next safe action are
    /// re-resolved rather than inherited.
    BlockedActionOwnerChanged,
    /// The localized fallback content is stale or policy-limited, so this falls back to the
    /// source language with its canonical citation preserved.
    LocalizedFallbackStaleOrPolicyLimited,
}

impl M5TeachingConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ImportedBehaviorPartial,
        Self::SequenceUnsupported,
        Self::BlockedActionOwnerChanged,
        Self::LocalizedFallbackStaleOrPolicyLimited,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportedBehaviorPartial => "imported_behavior_partial",
            Self::SequenceUnsupported => "sequence_unsupported",
            Self::BlockedActionOwnerChanged => "blocked_action_owner_changed",
            Self::LocalizedFallbackStaleOrPolicyLimited => {
                "localized_fallback_stale_or_policy_limited"
            }
        }
    }

    /// True when the reason reflects partial or unsupported behavior that must never
    /// masquerade as exact teaching parity — the acceptance-criterion boundary for imported
    /// behavior that is only partially mapped or a command-language sequence that is
    /// unsupported.
    pub const fn is_partial_or_unsupported(self) -> bool {
        matches!(
            self,
            Self::ImportedBehaviorPartial | Self::SequenceUnsupported
        )
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ImportedBehaviorPartial => {
                "imported behavior is only partially mapped, so this is a partial migration bridge and not an exact native equivalent"
            }
            Self::SequenceUnsupported => {
                "the command-language sequence is unsupported here, so no backing command completes it and the strip stays a disclosed dead-end"
            }
            Self::BlockedActionOwnerChanged => {
                "the blocked-action owner changed, so the named owner and next safe action are re-resolved rather than inherited from a healthier profile"
            }
            Self::LocalizedFallbackStaleOrPolicyLimited => {
                "the localized fallback content is stale or policy-limited, so this falls back to the source language with its canonical citation preserved"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5TeachingConsumerRecoveryAction {
        match self {
            Self::ImportedBehaviorPartial => {
                M5TeachingConsumerRecoveryAction::ReviewMigrationMappingBeforeTrusting
            }
            Self::SequenceUnsupported => {
                M5TeachingConsumerRecoveryAction::OpenFullCheatSheetForSupportedSequence
            }
            Self::BlockedActionOwnerChanged => {
                M5TeachingConsumerRecoveryAction::ContactCurrentBlockingOwner
            }
            Self::LocalizedFallbackStaleOrPolicyLimited => {
                M5TeachingConsumerRecoveryAction::ViewSourceLanguageOrRequestLocalization
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable
/// from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingConsumerRecoveryAction {
    /// Review the migration mapping before treating a partial bridge as an exact equivalent.
    ReviewMigrationMappingBeforeTrusting,
    /// Open the full cheat sheet for a supported sequence rather than the unsupported one.
    OpenFullCheatSheetForSupportedSequence,
    /// Contact the current blocking owner rather than the stale one.
    ContactCurrentBlockingOwner,
    /// View the source language, or request localization, before trusting a stale translation.
    ViewSourceLanguageOrRequestLocalization,
}

impl M5TeachingConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ReviewMigrationMappingBeforeTrusting,
        Self::OpenFullCheatSheetForSupportedSequence,
        Self::ContactCurrentBlockingOwner,
        Self::ViewSourceLanguageOrRequestLocalization,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReviewMigrationMappingBeforeTrusting => {
                "review_migration_mapping_before_trusting"
            }
            Self::OpenFullCheatSheetForSupportedSequence => {
                "open_full_cheat_sheet_for_supported_sequence"
            }
            Self::ContactCurrentBlockingOwner => "contact_current_blocking_owner",
            Self::ViewSourceLanguageOrRequestLocalization => {
                "view_source_language_or_request_localization"
            }
        }
    }
}

/// An export caveat a consumer preserves when a component renders below full parity (imported
/// behavior partial, an unsupported sequence, a changed blocked-action owner, or stale /
/// policy-limited localized fallback).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingConsumerExportCaveat {
    /// Imported behavior is only partially mapped, so the bridge is not exact.
    ImportedBehaviorPartialNotExact,
    /// The sequence is unsupported, so no backing command completes it.
    SequenceUnsupportedNoBackingCommand,
    /// The blocked-action owner was reassigned, so owner / next-action truth is re-resolved.
    BlockedActionOwnerReassigned,
    /// The localized fallback content is stale or policy-limited, so it falls back to source.
    LocalizedFallbackStaleOrPolicyLimited,
}

impl M5TeachingConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ImportedBehaviorPartialNotExact,
        Self::SequenceUnsupportedNoBackingCommand,
        Self::BlockedActionOwnerReassigned,
        Self::LocalizedFallbackStaleOrPolicyLimited,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ImportedBehaviorPartialNotExact => "imported_behavior_partial_not_exact",
            Self::SequenceUnsupportedNoBackingCommand => "sequence_unsupported_no_backing_command",
            Self::BlockedActionOwnerReassigned => "blocked_action_owner_reassigned",
            Self::LocalizedFallbackStaleOrPolicyLimited => {
                "localized_fallback_stale_or_policy_limited"
            }
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is
/// preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingClaimParityState {
    /// The descriptor vocabulary is preserved at full parity.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5TeachingClaimParityState {
    /// Every parity state, in declaration order.
    pub const ALL: [Self; 2] = [Self::ClaimsPreserved, Self::ClaimsAutoNarrowed];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ClaimsPreserved => "claims_preserved",
            Self::ClaimsAutoNarrowed => "claims_auto_narrowed",
        }
    }
}

/// One anatomy part the shared consumer projection surfaces. The parts in
/// [`M5TeachingConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingConsumerAnatomyPart {
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

impl M5TeachingConsumerAnatomyPart {
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
/// shared model. The fields in [`M5TeachingConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingConsumerExportField {
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

impl M5TeachingConsumerExportField {
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

/// A self-contained auto-narrow banner: the exact reason, the descriptors that stay preserved,
/// the export caveats, and the recovery action, so a narrowed rendering is understood from the
/// banner alone.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5TeachingConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5TeachingConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5TeachingComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5ContextualTeachingComponentFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5TeachingComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5TeachingConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved descriptors,
    /// and the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the teaching component-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5TeachingComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5ContextualTeachingComponentFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so command
    /// binding, migration mapping, blocked-action explanation, and source-language citation
    /// stay explicit.
    pub descriptor_families: Vec<M5TeachingComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5TeachingConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5TeachingConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5TeachingComponentConsumer,
    /// The component family.
    pub component_family: M5ContextualTeachingComponentFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5TeachingComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5TeachingConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5TeachingConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5TeachingClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects partial or unsupported behavior (imported behavior only
    /// partially mapped, or an unsupported sequence). Such a binding must always be narrowed
    /// and never asserts exact teaching parity.
    pub reflects_partial_or_unsupported_state: bool,
    /// Hard invariant: whether this binding claims exact teaching parity. Only a full-parity
    /// binding may assert exact parity; every narrowed binding — and in particular any partial
    /// or unsupported one — resolves this to `false`.
    pub asserts_exact_teaching_parity: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5TeachingComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_teaching_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5TeachingComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5TeachingComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5TeachingComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "teaching component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TeachingComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that command binding,
/// migration mapping, blocked-action explanation, and source-language citation stay explicit on
/// every surface. The claim-parity state is preserved at full parity and auto-narrowed under
/// any weakened parity-health mode, and a weakened mode always produces a self-contained banner
/// naming the exact reason and recovery action while keeping the descriptor vocabulary intact.
/// Partial or unsupported state (imported behavior only partially mapped, or an unsupported
/// sequence) always narrows and never asserts exact teaching parity.
pub fn resolve_teaching_component_binding(
    input: &M5TeachingComponentBindingInput,
) -> Result<M5TeachingComponentResolvedBinding, M5TeachingComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5TeachingComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5TeachingComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5TeachingComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5TeachingComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5TeachingComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text
        // extension from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5TeachingComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_partial_or_unsupported_state =
        narrowing_reason.is_some_and(M5TeachingConsumerNarrowingReason::is_partial_or_unsupported);
    // Only a full-parity binding may assert exact teaching parity. Every narrowed binding — and
    // every partial / unsupported one in particular — is not exact.
    let asserts_exact_teaching_parity = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5TeachingClaimParityState::ClaimsAutoNarrowed
    } else {
        M5TeachingClaimParityState::ClaimsPreserved
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
        M5TeachingComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5TeachingComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_partial_or_unsupported_state,
        asserts_exact_teaching_parity,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs
/// consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentBindingCase {
    /// The resolver input.
    pub input: M5TeachingComponentBindingInput,
    /// The resolved truth. Must equal `resolve_teaching_component_binding(&input)`.
    pub resolved: M5TeachingComponentResolvedBinding,
}

impl M5TeachingComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5TeachingComponentBindingInput) -> Self {
        let resolved =
            resolve_teaching_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_teaching_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the consumer
/// points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5ContextualTeachingComponentFamily,
    /// The canonical schema ref the consumer points at. Must equal the family's canonical
    /// schema ref.
    pub canonical_schema_ref: String,
    /// The canonical support-export artifact ref the consumer points at. Must equal the
    /// family's canonical artifact ref.
    pub canonical_artifact_ref: String,
    /// Hard invariant: the consumer references the canonical family, not a local re-description
    /// of its facts. MUST be `true`.
    pub references_canonical_not_local_prose: bool,
    /// Worked binding cases proving the resolver on this consumer/family.
    pub example_bindings: Vec<M5TeachingComponentBindingCase>,
}

impl M5TeachingComponentBinding {
    /// True when the binding points at the family's canonical refs and references the canonical
    /// family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one teaching consumer bound to the canonical component
/// families, the shared descriptor vocabulary, the parity-health modes, export caveats, parity
/// states, narrowing reasons, recovery actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentConsumerRow {
    /// Teaching consumer.
    pub consumer: M5TeachingComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5TeachingQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 teaching surface families that render / consume this projection.
    pub surface_families: Vec<M5TeachingSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5TeachingDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5TeachingConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5TeachingComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5TeachingConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5TeachingConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5TeachingClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5TeachingConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5TeachingConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5TeachingConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TeachingAccessibilityRoute>,
    /// Teaching subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TeachingConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TeachingDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5TeachingComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the claims per surface. MUST be `false`.
    pub rewords_claims_per_surface: bool,
    /// Hard invariant: this consumer never invents a new teaching grammar. MUST be `false`.
    pub invents_new_teaching_grammar: bool,
    /// Hard invariant: this consumer never drops command-binding, migration-mapping,
    /// blocked-action-owner, or source-language-citation truth when narrowed. MUST be `false`.
    pub drops_command_mapping_owner_or_citation_when_narrowed: bool,
    /// Hard invariant: this consumer never shows partial or unsupported state as exact teaching
    /// parity. MUST be `false`.
    pub shows_partial_or_unsupported_state_as_exact: bool,
    /// Hard invariant: this consumer never inherits a stronger label from a healthier profile
    /// instead of narrowing visibly. MUST be `false`.
    pub inherits_stronger_label_from_healthier_profile: bool,
}

impl M5TeachingComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TeachingConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5TeachingConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5TeachingConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5TeachingConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5TeachingComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5TeachingComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5TeachingComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5ContextualTeachingComponentFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_claims_per_surface
            && !self.invents_new_teaching_grammar
            && !self.drops_command_mapping_owner_or_citation_when_narrowed
            && !self.shows_partial_or_unsupported_state_as_exact
            && !self.inherits_stronger_label_from_healthier_profile
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentConsumerVocabularySet {
    /// Teaching-consumer tokens.
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

impl M5TeachingComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5TeachingComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5ContextualTeachingComponentFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5TeachingComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5TeachingConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5TeachingConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5TeachingConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5TeachingConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5TeachingClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5TeachingConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TeachingConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TeachingAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5TeachingComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component primitives.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a new teaching grammar.
    pub no_consumer_invents_new_grammar: bool,
    /// Command binding, migration mapping, blocked-action explanation, and source-language
    /// citation stay explicit everywhere.
    pub command_mapping_owner_citation_explicit_on_every_surface: bool,
    /// Partial imports, unsupported sequences, changed owners, and stale localized fallback
    /// auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// Partial or unsupported state never masquerades as exact teaching parity.
    pub partial_or_unsupported_state_never_shown_as_exact: bool,
    /// The localized support packet presents the same teaching truth shown in-product.
    pub localized_support_presents_same_teaching_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentConsumerProjection {
    /// Onboarding, the importer, keybinding help, command docs, the Help pane, and the
    /// localized support packet all adopt the shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The command-binding descriptor reads a single canonical source.
    pub command_binding_reads_single_source: bool,
    /// The migration-mapping descriptor reads a single canonical source.
    pub migration_mapping_reads_single_source: bool,
    /// The blocked-action-explanation descriptor reads a single canonical source.
    pub blocked_action_explanation_reads_single_source: bool,
    /// The source-language-citation descriptor reads a single canonical source.
    pub source_language_citation_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting teaching-component consumer audit.
    pub teaching_component_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TeachingComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TeachingComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5TeachingComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TeachingComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TeachingComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TeachingComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TeachingComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TeachingComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 contextual-teaching component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TeachingComponentConsumerPacket {
    /// Record kind; must equal [`M5_TEACHING_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TEACHING_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5TeachingComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TeachingComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TeachingComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TeachingComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TeachingComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TeachingComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TeachingComponentConsumerPacket {
    /// Builds an M5 contextual-teaching component-consumer packet from stable-lane input.
    pub fn new(input: M5TeachingComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_TEACHING_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_TEACHING_COMPONENT_CONSUMER_SCHEMA_VERSION,
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

    /// Validates the M5 contextual-teaching component-consumer invariants.
    pub fn validate(&self) -> Vec<M5TeachingComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TEACHING_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5TeachingComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TEACHING_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5TeachingComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TeachingComponentConsumerViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_consumer_rows(self, &mut violations);
        validate_family_reuse(self, &mut violations);
        validate_narrowing_disclosure(self, &mut violations);
        validate_scope_preserved(self, &mut violations);
        validate_exact_parity_honesty(self, &mut violations);
        validate_support_export_reference(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 teaching component consumer packet serializes"),
        ) {
            violations.push(M5TeachingComponentConsumerViolation::RawMaterialInExport);
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
            .expect("m5 teaching component consumer packet serializes")
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
        out.push_str("# M5 Contextual-Teaching Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Teaching consumers: {} ({} stable)\n",
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
        out.push_str("\n## Teaching consumers\n\n");
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

/// Errors emitted when reading the checked-in M5 contextual-teaching component-consumer export.
#[derive(Debug)]
pub enum M5TeachingComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TeachingComponentConsumerViolation>),
}

impl fmt::Display for M5TeachingComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 teaching component consumer export parse failed: {error}"
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
                    "m5 teaching component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TeachingComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5TeachingComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TeachingComponentConsumerViolation {
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
    /// A required teaching consumer is missing from the matrix.
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
    /// No worked binding proves that partial or unsupported state narrows and never asserts
    /// exact teaching parity, or a binding does so incorrectly.
    ExactParityHonestyUnproven,
    /// The localized support packet consumer does not reference the canonical component schema.
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

impl M5TeachingComponentConsumerViolation {
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
            Self::ExactParityHonestyUnproven => "exact_parity_honesty_unproven",
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

/// Reads and validates the checked-in stable M5 contextual-teaching component-consumer export.
pub fn current_stable_m5_teaching_component_consumer_export(
) -> Result<M5TeachingComponentConsumerPacket, M5TeachingComponentConsumerArtifactError> {
    let packet: M5TeachingComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-contextual-teaching-component-consumer-proof/support_export.json"
    )))
    .map_err(M5TeachingComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TeachingComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TEACHING_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_TEACHING_COMPONENT_CONSUMER_DOC_REF,
        M5_TEACHING_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_TEACHING_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        M5_CONTEXTUAL_TIP_CARD_SCHEMA_REF,
        M5_MIGRATION_BRIDGE_CARD_SCHEMA_REF,
        M5_SEQUENCE_HELP_STRIP_SCHEMA_REF,
        M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TeachingComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5TeachingComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    let present: BTreeSet<M5TeachingComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5TeachingComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5TeachingComponentConsumerViolation::RequiredConsumerMissing);
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
            violations.push(M5TeachingComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TeachingComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5TeachingComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5TeachingComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5TeachingComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5TeachingComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5TeachingComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5TeachingComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5TeachingComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5TeachingComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5TeachingComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5TeachingComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5TeachingComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable components rather than one
/// onboarding page plus a few isolated help objects.
fn validate_family_reuse(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    for family in M5ContextualTeachingComponentFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5TeachingComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner
/// carries a specific reason, a recovery action, and a non-empty set of preserved descriptors —
/// the acceptance-criterion example that a consumer which cannot preserve parity is visibly
/// narrowed rather than inheriting stronger labels from healthier profiles.
fn validate_narrowing_disclosure(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
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
        violations.push(M5TeachingComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with
/// preserved parity and no banner — the acceptance-criterion example that full-parity consumers
/// keep the descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5TeachingClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5TeachingComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every worked binding that reflects partial or unsupported state must be narrowed and must
/// not assert exact teaching parity, and at least one such binding must be present — the
/// acceptance-criterion that partial or unsupported state no longer masquerades as exact
/// teaching parity on any claimed consumer.
fn validate_exact_parity_honesty(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_partial_or_unsupported_state {
            // A partial / unsupported binding that claims exact parity, or fails to narrow,
            // breaks the acceptance criterion.
            if resolved.asserts_exact_teaching_parity
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5TeachingClaimParityState::ClaimsAutoNarrowed
            {
                violations.push(M5TeachingComponentConsumerViolation::ExactParityHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5TeachingComponentConsumerViolation::ExactParityHonestyUnproven);
    }
}

/// The localized support packet consumer must reference the canonical component schema for each
/// family it adopts — the acceptance-criterion that a support / export lane can never drift from
/// the product truth.
fn validate_support_export_reference(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5TeachingComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5TeachingComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_new_grammar,
        review.command_mapping_owner_citation_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.partial_or_unsupported_state_never_shown_as_exact,
        review.localized_support_presents_same_teaching_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5TeachingComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.command_binding_reads_single_source,
        projection.migration_mapping_reads_single_source,
        projection.blocked_action_explanation_reads_single_source,
        projection.source_language_citation_reads_single_source,
    ] {
        if !ok {
            violations.push(M5TeachingComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TeachingComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TeachingComponentConsumerPacket,
    violations: &mut Vec<M5TeachingComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture
            .teaching_component_consumer_audit_ref
            .trim()
            .is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TeachingComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5TeachingComponentConsumerPacket,
) -> impl Iterator<Item = &M5TeachingComponentBindingCase> {
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
