//! Shared consumers for the reusable M5 shared-component-state taxonomy, so the shared state
//! taxonomy, the interactive-state contract, the selection-or-lock-state contract, and the
//! degraded-state-application contract keep state-semantics, state-cause, consequence/recovery,
//! and accessibility-label truth aligned across every claimed M5 profile where a user reads
//! shell chrome, runs a command / reads help, scans a dense search collection, reviews a
//! work-item, answers a settings / capability prompt, connects a provider / offline-capture
//! row, watches a test run, or opens a support / recovery lane.
//!
//! Aureline's frozen shared-component-state-taxonomy component matrix
//! (`crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix`)
//! names the four governed component families, and sibling implement lanes narrow the last three
//! families into working primitives, each with its own canonical schema, contract doc, and
//! support-export artifact:
//!
//! * the shared component-state taxonomy itself (the frozen matrix),
//! * the interactive-state contract (`implement_default_hover_focus_visible_pressed_...`),
//! * the selection-or-lock-state contract (`implement_selected_current_read_only_disabled_and_locked_...`), and
//! * the loading / pending / warning-error / degraded state-application contract
//!   (`implement_loading_pending_warning_error_and_degraded_...`).
//!
//! This module is the *adoption* lane over those contracts. It proves the four families are
//! reusable state contracts — not a design-system island — by binding every claimed M5 consumer
//! (shell chrome, command / help, search / dense collections, review / work-item flows,
//! settings / capability prompts, provider / offline-capture rows, test / watch surfaces, and
//! support / recovery lanes) to the same canonical contract schemas and the same descriptor
//! vocabulary. Each consumer points at the contract's canonical schema and support-export
//! artifact rather than re-wording state-semantics, state-cause, consequence/recovery, or
//! accessibility-label facts in local prose, and each keeps that vocabulary truthful even when a
//! state's cause is not yet resolved, no recovery path is available, a lock / block owner is
//! re-resolved, or a non-visual accessibility route is reduced.
//!
//! The module has two halves:
//!
//! 1. A resolver — [`resolve_state_component_binding`] — that takes one consumer's adoption of
//!    one component family, the descriptor set it surfaces, the parity-health mode it renders
//!    under, and any export caveats, and produces one [`M5StateComponentResolvedBinding`]
//!    carrying the derived claim-parity state and — whenever parity is weakened — a
//!    self-contained [`M5StateComponentAutoNarrowBanner`] that names the exact reason (state
//!    cause unresolved, recovery unavailable, lock owner unresolved, or an accessibility route
//!    reduced), the descriptors that stay preserved, and the recovery action, rather than a
//!    generic "degraded" note. The resolver never lets a narrowed context drop a required
//!    descriptor and never lets an incomplete or degraded state masquerade as an exact,
//!    healthy state.
//! 2. A parity matrix — [`M5StateComponentConsumerPacket`] — that binds one row per claimed M5
//!    consumer to the four canonical component families, the one shared descriptor vocabulary,
//!    the same parity-health modes, export caveats, parity states, narrowing reasons, recovery
//!    actions, export fields, and non-visual accessibility routes, so state-semantics /
//!    state-cause / consequence-recovery / accessibility-label facts stop diverging between the
//!    product UI, the docs, and the support artifact.
//!
//! The surface families, deployment lines, consumer surfaces, accessibility routes,
//! qualification classes, downgrade triggers, and the four component families themselves are
//! reused verbatim from the frozen shared-component-state-taxonomy component matrix. This module
//! mints new vocabulary only for what the adoption lane itself needs: its consumers, the shared
//! descriptor vocabulary, the parity-health modes, the export caveats, the claim-parity states,
//! the narrowing reasons and recovery actions, the consumer anatomy parts, and the export
//! fields.
//!
//! Raw local paths, credentials, tokens, and private endpoints stay outside the support
//! boundary; every label is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! `schemas/ui/m5-shared-state-taxonomy-component-consumer.schema.json` and the contract doc is
//! `docs/design-system/m5_shared_state_taxonomy_component_consumers.md`. The protected fixture
//! directory is `fixtures/ui/m5-shared-state-taxonomy-component-consumers/`.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_state_component_consumer_packet,
    seeded_m5_state_component_consumer_provider_offline_capture_beta_narrowed,
    seeded_m5_state_component_consumer_test_watch_preview_narrowed,
    M5_STATE_COMPONENT_CONSUMER_PACKET_ID,
};

// The surface families, deployment lines, consumer surfaces, accessibility routes,
// qualification classes, downgrade triggers, and the four component families are frozen once,
// in the shared-component-state-taxonomy component matrix. This adoption lane reuses them
// verbatim so it never invents a parallel state vocabulary.
pub use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::{
    M5ComponentStateAccessibilityRoute, M5ComponentStateConsumerSurface,
    M5ComponentStateDeploymentLine, M5ComponentStateDowngradeTrigger,
    M5ComponentStateQualificationClass, M5ComponentStateSurfaceFamily,
    M5SharedComponentStateFamily,
};

// The canonical matrix schema / doc / artifact refs this adoption lane points the shared
// taxonomy family at, rather than re-wording its facts in local prose.
use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::{
    M5_SHARED_COMPONENT_STATE_ARTIFACT_REF, M5_SHARED_COMPONENT_STATE_DOC_REF,
    M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
};
// The canonical narrowed-primitive schema / doc / artifact refs each family maps to.
use crate::implement_default_hover_focus_visible_pressed_state_contracts_with_no_color_only_and_no_layout_shift_rules_across_claimed_m5_controls_and_pane_affordances::{
    M5_INTERACTIVE_STATE_CONTRACT_ARTIFACT_REF, M5_INTERACTIVE_STATE_CONTRACT_DOC_REF,
    M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF,
};
use crate::implement_loading_pending_warning_error_and_degraded_state_blocks_with_submission_lineage_health_and_recovery_truth_across_claimed_m5_workflows::{
    M5_DEGRADED_STATE_CONTRACT_ARTIFACT_REF, M5_DEGRADED_STATE_CONTRACT_DOC_REF,
    M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF,
};
use crate::implement_selected_current_read_only_disabled_and_locked_state_parity_with_owner_reason_recovery_truth_across_claimed_m5_tabs_trees_lists_tables_badges_and_inspectors::{
    M5_SELECTION_OR_LOCK_STATE_CONTRACT_ARTIFACT_REF, M5_SELECTION_OR_LOCK_STATE_CONTRACT_DOC_REF,
    M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5StateComponentConsumerPacket`].
pub const M5_STATE_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "add_shared_shell_command_search_review_settings_provider_test_and_support_consumers_so_state_taxonomy_components_keep_label_recovery_and_accessibility_parity_across_claimed_m5_profiles";

/// Schema version for M5 shared-state-taxonomy component-consumer records.
pub const M5_STATE_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the state-component-consumer boundary schema.
pub const M5_STATE_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-shared-state-taxonomy-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_STATE_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/design-system/m5_shared_state_taxonomy_component_consumers.md";

/// Repo-relative path of the frozen shared-component-state-taxonomy component matrix this lane
/// adopts from.
pub const M5_STATE_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF: &str =
    M5_SHARED_COMPONENT_STATE_SCHEMA_REF;

/// Repo-relative path of the frozen matrix contract doc this lane binds against.
pub const M5_STATE_COMPONENT_CONSUMER_OBJECT_MODEL_REF: &str = M5_SHARED_COMPONENT_STATE_DOC_REF;

/// Repo-relative path of the canonical state-class contract every consumer maps its state names
/// back to, so no surface invents a private state label.
pub const M5_STATE_COMPONENT_CONSUMER_STATE_CLASS_REF: &str =
    "schemas/state/state_class.schema.json";

/// Repo-relative path of the state-class recovery contract every consumer reads its state
/// cause / consequence / recovery truth from, so support and docs never clone divergent copy.
pub const M5_STATE_COMPONENT_CONSUMER_STATE_RECOVERY_REF: &str =
    "schemas/state/state_class_recovery.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_STATE_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-shared-state-taxonomy-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_STATE_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-shared-state-taxonomy-component-consumer-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_STATE_COMPONENT_CONSUMER_CSV_REF: &str =
    "artifacts/release/m5-shared-state-taxonomy-component-consumer-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_STATE_COMPONENT_CONSUMER_REPORT_REF: &str =
    "artifacts/release/m5-shared-state-taxonomy-component-consumer-proof/report.md";

/// The canonical boundary schema ref of the contract that owns a family. A consumer that adopts
/// a family must point at this schema, not a local re-description.
pub const fn family_canonical_schema_ref(family: M5SharedComponentStateFamily) -> &'static str {
    use M5SharedComponentStateFamily as Family;
    match family {
        Family::SharedComponentStateTaxonomy => M5_SHARED_COMPONENT_STATE_SCHEMA_REF,
        Family::InteractiveState => M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF,
        Family::SelectionOrLockState => M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF,
        Family::DegradedStateApplication => M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF,
    }
}

/// The canonical contract-doc ref of the contract that owns a family.
pub const fn family_canonical_doc_ref(family: M5SharedComponentStateFamily) -> &'static str {
    use M5SharedComponentStateFamily as Family;
    match family {
        Family::SharedComponentStateTaxonomy => M5_SHARED_COMPONENT_STATE_DOC_REF,
        Family::InteractiveState => M5_INTERACTIVE_STATE_CONTRACT_DOC_REF,
        Family::SelectionOrLockState => M5_SELECTION_OR_LOCK_STATE_CONTRACT_DOC_REF,
        Family::DegradedStateApplication => M5_DEGRADED_STATE_CONTRACT_DOC_REF,
    }
}

/// The canonical support-export artifact ref of the contract that owns a family.
pub const fn family_canonical_artifact_ref(family: M5SharedComponentStateFamily) -> &'static str {
    use M5SharedComponentStateFamily as Family;
    match family {
        Family::SharedComponentStateTaxonomy => M5_SHARED_COMPONENT_STATE_ARTIFACT_REF,
        Family::InteractiveState => M5_INTERACTIVE_STATE_CONTRACT_ARTIFACT_REF,
        Family::SelectionOrLockState => M5_SELECTION_OR_LOCK_STATE_CONTRACT_ARTIFACT_REF,
        Family::DegradedStateApplication => M5_DEGRADED_STATE_CONTRACT_ARTIFACT_REF,
    }
}

/// One claimed M5 consumer that adopts the shared state contracts. These are the consumers the
/// spec names — shell chrome, command / help, search / dense collections, review / work-item
/// flows, settings / capability prompts, provider / offline-capture rows, test / watch surfaces,
/// and support / recovery lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateComponentConsumer {
    /// Shell chrome (status bar, panes, progress affordances).
    ShellChrome,
    /// Command / help surfaces (command palette, help pane).
    CommandHelp,
    /// Search and other dense collections (lists, trees, grids).
    SearchDenseCollection,
    /// Review / work-item flows.
    ReviewWorkItem,
    /// Settings / capability prompts.
    SettingsCapability,
    /// Provider / offline-capture rows.
    ProviderOfflineCapture,
    /// Test / watch surfaces.
    TestWatch,
    /// Support / recovery lanes (support export, diagnostics, Help/About).
    SupportRecovery,
}

impl M5StateComponentConsumer {
    /// Every claimed consumer, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ShellChrome,
        Self::CommandHelp,
        Self::SearchDenseCollection,
        Self::ReviewWorkItem,
        Self::SettingsCapability,
        Self::ProviderOfflineCapture,
        Self::TestWatch,
        Self::SupportRecovery,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellChrome => "shell_chrome",
            Self::CommandHelp => "command_help",
            Self::SearchDenseCollection => "search_dense_collection",
            Self::ReviewWorkItem => "review_work_item",
            Self::SettingsCapability => "settings_capability",
            Self::ProviderOfflineCapture => "provider_offline_capture",
            Self::TestWatch => "test_watch",
            Self::SupportRecovery => "support_recovery",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ShellChrome => "Shell Chrome",
            Self::CommandHelp => "Command / Help",
            Self::SearchDenseCollection => "Search / Dense Collection",
            Self::ReviewWorkItem => "Review / Work-Item",
            Self::SettingsCapability => "Settings / Capability",
            Self::ProviderOfflineCapture => "Provider / Offline-Capture",
            Self::TestWatch => "Test / Watch",
            Self::SupportRecovery => "Support / Recovery",
        }
    }

    /// True when this consumer is the support / recovery lane — the surface singled out for a
    /// canonical-schema reference so its exported prose can never drift from the product truth.
    pub const fn is_support_or_export(self) -> bool {
        matches!(self, Self::SupportRecovery)
    }
}

/// The one shared descriptor vocabulary every state-taxonomy component keeps aligned across
/// surfaces, so no consumer invents a new grammar or stale wording. The descriptors in
/// [`M5StateComponentDescriptor::REQUIRED`] must be present on every binding — the
/// acceptance-criterion that state semantics, state cause, consequence/recovery, and the
/// accessibility label stay one truth across in-product and exported state surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateComponentDescriptor {
    /// The canonical state class and its precedence / distinctness rule (locked-over-disabled,
    /// read-only-over-disabled, current-vs-selected, pending-vs-loading).
    StateSemantics,
    /// Why the state applies — its state cause.
    StateCause,
    /// The consequence a state carries and the recovery action out of it.
    ConsequenceAndRecovery,
    /// The non-visual, keyboard-visible, screen-reader-explainable state label.
    AccessibilityLabel,
}

impl M5StateComponentDescriptor {
    /// Every descriptor, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StateSemantics,
        Self::StateCause,
        Self::ConsequenceAndRecovery,
        Self::AccessibilityLabel,
    ];

    /// Every descriptor is required on every binding.
    pub const REQUIRED: [Self; 4] = Self::ALL;

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateSemantics => "state_semantics",
            Self::StateCause => "state_cause",
            Self::ConsequenceAndRecovery => "consequence_and_recovery",
            Self::AccessibilityLabel => "accessibility_label",
        }
    }
}

/// The parity-health mode a consumer renders a component under. A weakened mode still keeps the
/// descriptor vocabulary — it only discloses that parity is narrowed relative to the
/// authoritative rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateConsumerParityHealth {
    /// Full parity: the authoritative rendering.
    FullParity,
    /// The state cause is not yet resolved, so the state is disclosed as unexplained rather than
    /// asserted as a settled, exact state.
    StateCauseUnresolvedNarrowed,
    /// No recovery path is available yet, so the state renders as degraded and names its reduced
    /// capability rather than a healthy exact state.
    RecoveryUnavailableNarrowed,
    /// The lock / block owner is re-resolved, so the locked posture is named rather than masked
    /// as a plain disabled control or inherited from a healthier profile.
    LockOwnerUnresolvedNarrowed,
    /// A non-visual accessibility route is reduced here, so it falls back to the full accessible
    /// state description rather than a color-only or hover-only cue.
    AccessibilityRouteReducedNarrowed,
}

impl M5StateConsumerParityHealth {
    /// Every parity-health mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullParity,
        Self::StateCauseUnresolvedNarrowed,
        Self::RecoveryUnavailableNarrowed,
        Self::LockOwnerUnresolvedNarrowed,
        Self::AccessibilityRouteReducedNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::StateCauseUnresolvedNarrowed => "state_cause_unresolved_narrowed",
            Self::RecoveryUnavailableNarrowed => "recovery_unavailable_narrowed",
            Self::LockOwnerUnresolvedNarrowed => "lock_owner_unresolved_narrowed",
            Self::AccessibilityRouteReducedNarrowed => "accessibility_route_reduced_narrowed",
        }
    }

    /// True when the mode renders below the authoritative full parity and so must disclose a
    /// self-contained auto-narrow banner.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }

    /// The narrowing reason a weakened mode discloses, if any.
    pub const fn narrowing_reason(self) -> Option<M5StateConsumerNarrowingReason> {
        Some(match self {
            Self::StateCauseUnresolvedNarrowed => {
                M5StateConsumerNarrowingReason::StateCauseUnresolved
            }
            Self::RecoveryUnavailableNarrowed => {
                M5StateConsumerNarrowingReason::RecoveryUnavailable
            }
            Self::LockOwnerUnresolvedNarrowed => {
                M5StateConsumerNarrowingReason::LockOwnerUnresolved
            }
            Self::AccessibilityRouteReducedNarrowed => {
                M5StateConsumerNarrowingReason::AccessibilityRouteReduced
            }
            Self::FullParity => return None,
        })
    }
}

/// The exact reason a binding auto-narrows its parity claim language, so an auto-narrow banner
/// never reads like a generic "degraded" note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateConsumerNarrowingReason {
    /// The state cause is not yet resolved, so the state is disclosed as unexplained rather than
    /// asserted as a settled, exact state.
    StateCauseUnresolved,
    /// No recovery path is available yet, so the state is degraded and names its reduced
    /// capability instead of a healthy exact state.
    RecoveryUnavailable,
    /// The lock / block owner is re-resolved, so the locked posture is named rather than masked
    /// or inherited.
    LockOwnerUnresolved,
    /// A non-visual accessibility route is reduced, so the state falls back to its full
    /// accessible description.
    AccessibilityRouteReduced,
}

impl M5StateConsumerNarrowingReason {
    /// Every narrowing reason, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StateCauseUnresolved,
        Self::RecoveryUnavailable,
        Self::LockOwnerUnresolved,
        Self::AccessibilityRouteReduced,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateCauseUnresolved => "state_cause_unresolved",
            Self::RecoveryUnavailable => "recovery_unavailable",
            Self::LockOwnerUnresolved => "lock_owner_unresolved",
            Self::AccessibilityRouteReduced => "accessibility_route_reduced",
        }
    }

    /// True when the reason reflects an incomplete or degraded state that must never masquerade
    /// as an exact, healthy state — the acceptance-criterion boundary for a state whose cause is
    /// unresolved or whose recovery path is unavailable.
    pub const fn is_incomplete_or_degraded(self) -> bool {
        matches!(self, Self::StateCauseUnresolved | Self::RecoveryUnavailable)
    }

    /// Review-safe reason phrase for the banner headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::StateCauseUnresolved => {
                "the state cause is not yet resolved, so this state is disclosed as unexplained rather than asserted as a settled, exact state"
            }
            Self::RecoveryUnavailable => {
                "no recovery path is available yet, so this renders as a degraded state that names its consequence and reduced capability instead of a healthy exact state"
            }
            Self::LockOwnerUnresolved => {
                "the lock / block owner is re-resolved here, so the locked posture is named rather than masked as a plain disabled control or inherited from a healthier profile"
            }
            Self::AccessibilityRouteReduced => {
                "a non-visual accessibility route is reduced here, so this falls back to the full accessible state description rather than a color-only or hover-only cue"
            }
        }
    }

    /// The recovery action a reader should take before trusting full parity.
    pub const fn recovery_action(self) -> M5StateConsumerRecoveryAction {
        match self {
            Self::StateCauseUnresolved => {
                M5StateConsumerRecoveryAction::ResolveStateCauseBeforeTrusting
            }
            Self::RecoveryUnavailable => M5StateConsumerRecoveryAction::FollowDisclosedRecoveryPath,
            Self::LockOwnerUnresolved => M5StateConsumerRecoveryAction::ContactCurrentLockOwner,
            Self::AccessibilityRouteReduced => {
                M5StateConsumerRecoveryAction::OpenFullAccessibleStateDescription
            }
        }
    }
}

/// The recovery action named on an auto-narrow banner, so a narrowed rendering is actionable
/// from the banner itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateConsumerRecoveryAction {
    /// Resolve the state cause before treating the state as a settled, exact state.
    ResolveStateCauseBeforeTrusting,
    /// Follow the disclosed recovery path out of the degraded state.
    FollowDisclosedRecoveryPath,
    /// Contact the current lock / block owner rather than the stale one.
    ContactCurrentLockOwner,
    /// Open the full accessible state description rather than relying on a reduced cue.
    OpenFullAccessibleStateDescription,
}

impl M5StateConsumerRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ResolveStateCauseBeforeTrusting,
        Self::FollowDisclosedRecoveryPath,
        Self::ContactCurrentLockOwner,
        Self::OpenFullAccessibleStateDescription,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolveStateCauseBeforeTrusting => "resolve_state_cause_before_trusting",
            Self::FollowDisclosedRecoveryPath => "follow_disclosed_recovery_path",
            Self::ContactCurrentLockOwner => "contact_current_lock_owner",
            Self::OpenFullAccessibleStateDescription => "open_full_accessible_state_description",
        }
    }
}

/// An export caveat a consumer preserves when a component renders below full parity (state cause
/// unresolved, recovery unavailable, a re-resolved lock owner, or a reduced accessibility route).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateConsumerExportCaveat {
    /// The state cause is not yet resolved, so the state is not asserted as exact.
    StateCauseUnresolvedNotExplained,
    /// No recovery path is available, so the state is degraded with reduced capability.
    RecoveryUnavailableDegraded,
    /// The lock / block owner was reassigned, so owner / recovery truth is re-resolved.
    LockOwnerReassigned,
    /// A non-visual accessibility route is reduced, so the state falls back to its full
    /// accessible description.
    AccessibilityRouteReducedFallback,
}

impl M5StateConsumerExportCaveat {
    /// Every export caveat, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StateCauseUnresolvedNotExplained,
        Self::RecoveryUnavailableDegraded,
        Self::LockOwnerReassigned,
        Self::AccessibilityRouteReducedFallback,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateCauseUnresolvedNotExplained => "state_cause_unresolved_not_explained",
            Self::RecoveryUnavailableDegraded => "recovery_unavailable_degraded",
            Self::LockOwnerReassigned => "lock_owner_reassigned",
            Self::AccessibilityRouteReducedFallback => "accessibility_route_reduced_fallback",
        }
    }
}

/// The derived claim-parity state of a binding — whether the shared descriptor vocabulary is
/// preserved as-is or auto-narrowed with a disclosed reason.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateClaimParityState {
    /// The descriptor vocabulary is preserved at full parity.
    ClaimsPreserved,
    /// The descriptor vocabulary is preserved, with a disclosed auto-narrowing.
    ClaimsAutoNarrowed,
}

impl M5StateClaimParityState {
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
/// [`M5StateConsumerAnatomyPart::MANDATORY`] are required on every consumer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateConsumerAnatomyPart {
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

impl M5StateConsumerAnatomyPart {
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
/// shared model. The fields in [`M5StateConsumerExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateConsumerExportField {
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

impl M5StateConsumerExportField {
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
pub struct M5StateComponentAutoNarrowBanner {
    /// The exact narrowing reason.
    pub reason: M5StateConsumerNarrowingReason,
    /// The recovery action a reader should take.
    pub recovery_action: M5StateConsumerRecoveryAction,
    /// The consumer the banner applies to.
    pub consumer: M5StateComponentConsumer,
    /// The component family the banner applies to.
    pub component_family: M5SharedComponentStateFamily,
    /// The descriptors that stay preserved under the narrowing.
    pub preserved_descriptors: Vec<M5StateComponentDescriptor>,
    /// The export caveats disclosed alongside the narrowing.
    pub export_caveats: Vec<M5StateConsumerExportCaveat>,
    /// A deterministic, self-contained headline naming the reason, the preserved descriptors, and
    /// the recovery action — never a generic "degraded" note.
    pub headline: String,
}

/// The full input to the state component-binding resolver for one consumer/family adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentBindingInput {
    /// The consumer that adopts the component.
    pub consumer: M5StateComponentConsumer,
    /// The canonical component family being adopted.
    pub component_family: M5SharedComponentStateFamily,
    /// The descriptor set the binding surfaces. Must cover every required descriptor so state
    /// semantics, state cause, consequence/recovery, and the accessibility label stay explicit.
    pub descriptor_families: Vec<M5StateComponentDescriptor>,
    /// The parity-health mode the binding renders under.
    pub parity_health: M5StateConsumerParityHealth,
    /// The export caveats disclosed.
    pub export_caveats: Vec<M5StateConsumerExportCaveat>,
    /// An opaque, export-safe note recorded with the binding.
    pub note_repr: Option<String>,
}

/// The resolved claim-parity / auto-narrow truth for one adoption.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentResolvedBinding {
    /// The consumer.
    pub consumer: M5StateComponentConsumer,
    /// The component family.
    pub component_family: M5SharedComponentStateFamily,
    /// The canonical schema ref for the family (never a local re-description).
    pub canonical_schema_ref: String,
    /// The descriptor set the binding surfaces.
    pub descriptor_families: Vec<M5StateComponentDescriptor>,
    /// The parity-health mode.
    pub parity_health: M5StateConsumerParityHealth,
    /// The export caveats.
    pub export_caveats: Vec<M5StateConsumerExportCaveat>,
    /// The derived claim-parity state.
    pub claim_parity_state: M5StateClaimParityState,
    /// True when the binding renders under a weakened parity-health mode.
    pub is_narrowed: bool,
    /// True when the binding reflects an incomplete or degraded state (state cause unresolved, or
    /// no recovery path available). Such a binding must always be narrowed and never asserts an
    /// exact, healthy state.
    pub reflects_incomplete_or_degraded_state: bool,
    /// Hard invariant: whether this binding claims exact state parity. Only a full-parity binding
    /// may assert exact parity; every narrowed binding — and in particular any incomplete or
    /// degraded one — resolves this to `false`.
    pub asserts_exact_state_parity: bool,
    /// The auto-narrow banner, present when narrowed.
    pub auto_narrow_banner: Option<M5StateComponentAutoNarrowBanner>,
}

/// Errors returned by [`resolve_state_component_binding`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5StateComponentBindingError {
    /// The descriptor set was empty.
    EmptyDescriptorSet,
    /// A required descriptor was missing from the binding.
    MissingRequiredDescriptor,
    /// A binding note carried forbidden material.
    ForbiddenBindingMaterial,
}

impl M5StateComponentBindingError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDescriptorSet => "empty_descriptor_set",
            Self::MissingRequiredDescriptor => "missing_required_descriptor",
            Self::ForbiddenBindingMaterial => "forbidden_binding_material",
        }
    }
}

impl fmt::Display for M5StateComponentBindingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "state component binding error: {}",
            self.as_str()
        )
    }
}

impl Error for M5StateComponentBindingError {}

/// Resolves one consumer/family adoption from its declared state.
///
/// Every required descriptor must be present — the acceptance-criterion that state semantics,
/// state cause, consequence/recovery, and the accessibility label stay explicit on every surface.
/// The claim-parity state is preserved at full parity and auto-narrowed under any weakened
/// parity-health mode, and a weakened mode always produces a self-contained banner naming the
/// exact reason and recovery action while keeping the descriptor vocabulary intact. An incomplete
/// or degraded state (state cause unresolved, or no recovery path available) always narrows and
/// never asserts an exact, healthy state.
pub fn resolve_state_component_binding(
    input: &M5StateComponentBindingInput,
) -> Result<M5StateComponentResolvedBinding, M5StateComponentBindingError> {
    if input.descriptor_families.is_empty() {
        return Err(M5StateComponentBindingError::EmptyDescriptorSet);
    }
    let present: BTreeSet<M5StateComponentDescriptor> =
        input.descriptor_families.iter().copied().collect();
    for required in M5StateComponentDescriptor::REQUIRED {
        if !present.contains(&required) {
            return Err(M5StateComponentBindingError::MissingRequiredDescriptor);
        }
    }
    if let Some(note) = &input.note_repr {
        if value_repr_is_forbidden(note) {
            return Err(M5StateComponentBindingError::ForbiddenBindingMaterial);
        }
    }
    for caveat in &input.export_caveats {
        // Caveat tokens are controlled vocabulary; this only guards a future free-text extension
        // from leaking forbidden material.
        if value_repr_is_forbidden(caveat.as_str()) {
            return Err(M5StateComponentBindingError::ForbiddenBindingMaterial);
        }
    }

    let is_narrowed = input.parity_health.is_narrowed();
    let narrowing_reason = input.parity_health.narrowing_reason();
    let reflects_incomplete_or_degraded_state =
        narrowing_reason.is_some_and(M5StateConsumerNarrowingReason::is_incomplete_or_degraded);
    // Only a full-parity binding may assert exact state parity. Every narrowed binding — and every
    // incomplete / degraded one in particular — is not exact.
    let asserts_exact_state_parity = !is_narrowed;
    let claim_parity_state = if is_narrowed {
        M5StateClaimParityState::ClaimsAutoNarrowed
    } else {
        M5StateClaimParityState::ClaimsPreserved
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
        M5StateComponentAutoNarrowBanner {
            reason,
            recovery_action,
            consumer: input.consumer,
            component_family: input.component_family,
            preserved_descriptors: input.descriptor_families.clone(),
            export_caveats: input.export_caveats.clone(),
            headline,
        }
    });

    Ok(M5StateComponentResolvedBinding {
        consumer: input.consumer,
        component_family: input.component_family,
        canonical_schema_ref: family_canonical_schema_ref(input.component_family).to_owned(),
        descriptor_families: input.descriptor_families.clone(),
        parity_health: input.parity_health,
        export_caveats: input.export_caveats.clone(),
        claim_parity_state,
        is_narrowed,
        reflects_incomplete_or_degraded_state,
        asserts_exact_state_parity,
        auto_narrow_banner,
    })
}

/// One worked binding case carried in the packet so the support / export packet reconstructs
/// consumer parity from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentBindingCase {
    /// The resolver input.
    pub input: M5StateComponentBindingInput,
    /// The resolved truth. Must equal `resolve_state_component_binding(&input)`.
    pub resolved: M5StateComponentResolvedBinding,
}

impl M5StateComponentBindingCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5StateComponentBindingInput) -> Self {
        let resolved = resolve_state_component_binding(&input).expect("seed binding case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_state_component_binding(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One consumer's adoption of one canonical component family: the canonical refs the consumer
/// points at, and the worked bindings proving parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentBinding {
    /// The canonical component family being adopted.
    pub component_family: M5SharedComponentStateFamily,
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
    pub example_bindings: Vec<M5StateComponentBindingCase>,
}

impl M5StateComponentBinding {
    /// True when the binding points at the family's canonical refs and references the canonical
    /// family rather than local prose.
    fn points_to_canonical_family(&self) -> bool {
        self.canonical_schema_ref == family_canonical_schema_ref(self.component_family)
            && self.canonical_artifact_ref == family_canonical_artifact_ref(self.component_family)
            && self.references_canonical_not_local_prose
    }
}

/// One row in the consumer matrix: one consumer bound to the canonical component families, the
/// shared descriptor vocabulary, the parity-health modes, export caveats, parity states,
/// narrowing reasons, recovery actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentConsumerRow {
    /// Consumer.
    pub consumer: M5StateComponentConsumer,
    /// Qualification class earned by this consumer.
    pub qualification: M5ComponentStateQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this projection.
    pub surface_families: Vec<M5ComponentStateSurfaceFamily>,
    /// Deployment lines this projection keeps the same truth across.
    pub deployment_lines: Vec<M5ComponentStateDeploymentLine>,
    /// Anatomy parts this projection renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5StateConsumerAnatomyPart>,
    /// Descriptor families this consumer keeps aligned (must include the required set).
    pub descriptor_families: Vec<M5StateComponentDescriptor>,
    /// Parity-health modes this consumer distinguishes.
    pub parity_health_modes: Vec<M5StateConsumerParityHealth>,
    /// Export caveats this consumer preserves.
    pub export_caveats: Vec<M5StateConsumerExportCaveat>,
    /// Claim-parity states this consumer distinguishes.
    pub claim_parity_states: Vec<M5StateClaimParityState>,
    /// Narrowing reasons this consumer names.
    pub narrowing_reasons: Vec<M5StateConsumerNarrowingReason>,
    /// Recovery actions this consumer names.
    pub recovery_actions: Vec<M5StateConsumerRecoveryAction>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5StateConsumerExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5ComponentStateAccessibilityRoute>,
    /// Subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ComponentStateConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5ComponentStateDowngradeTrigger>,
    /// The canonical component families this consumer adopts, with worked bindings.
    pub component_bindings: Vec<M5StateComponentBinding>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this consumer never re-words the state semantics per surface. MUST be
    /// `false`.
    pub rewords_state_semantics_per_surface: bool,
    /// Hard invariant: this consumer never invents a private / alternate state name. MUST be
    /// `false`.
    pub invents_private_state_names: bool,
    /// Hard invariant: this consumer never drops state-cause or consequence/recovery truth when
    /// narrowed. MUST be `false`.
    pub drops_cause_or_recovery_when_narrowed: bool,
    /// Hard invariant: this consumer never shows an incomplete or degraded state as an exact,
    /// healthy state. MUST be `false`.
    pub shows_partial_state_as_exact: bool,
    /// Hard invariant: this consumer never collapses two distinct states into one another and
    /// never encodes a state by color alone. MUST be `false`.
    pub collapses_distinct_states_or_uses_color_only: bool,
}

impl M5StateComponentConsumerRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5StateConsumerAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5StateConsumerAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5StateConsumerExportField> =
            self.export_fields.iter().copied().collect();
        M5StateConsumerExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row keeps every required descriptor.
    fn declares_required_descriptors(&self) -> bool {
        let present: BTreeSet<M5StateComponentDescriptor> =
            self.descriptor_families.iter().copied().collect();
        M5StateComponentDescriptor::REQUIRED
            .iter()
            .all(|descriptor| present.contains(descriptor))
    }

    /// True when every component binding points to its canonical family.
    fn all_bindings_point_to_canonical(&self) -> bool {
        self.component_bindings
            .iter()
            .all(M5StateComponentBinding::points_to_canonical_family)
    }

    /// The set of component families this row adopts.
    fn adopted_families(&self) -> BTreeSet<M5SharedComponentStateFamily> {
        self.component_bindings
            .iter()
            .map(|binding| binding.component_family)
            .collect()
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.rewords_state_semantics_per_surface
            && !self.invents_private_state_names
            && !self.drops_cause_or_recovery_when_narrowed
            && !self.shows_partial_state_as_exact
            && !self.collapses_distinct_states_or_uses_color_only
    }
}

/// Self-describing controlled-vocabulary set carried by this lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentConsumerVocabularySet {
    /// Consumer tokens.
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

impl M5StateComponentConsumerVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumers: tokens(&M5StateComponentConsumer::ALL, |v| v.as_str()),
            component_families: tokens(&M5SharedComponentStateFamily::ALL, |v| v.as_str()),
            descriptors: tokens(&M5StateComponentDescriptor::ALL, |v| v.as_str()),
            parity_health_modes: tokens(&M5StateConsumerParityHealth::ALL, |v| v.as_str()),
            export_caveats: tokens(&M5StateConsumerExportCaveat::ALL, |v| v.as_str()),
            narrowing_reasons: tokens(&M5StateConsumerNarrowingReason::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5StateConsumerRecoveryAction::ALL, |v| v.as_str()),
            claim_parity_states: tokens(&M5StateClaimParityState::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5StateConsumerAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5StateConsumerExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ComponentStateAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5StateComponentConsumerGovernanceReview {
    /// Every consumer adopts the same canonical component contracts.
    pub consumers_adopt_shared_primitives: bool,
    /// Every consumer points at the canonical schema, not local prose.
    pub consumers_reference_canonical_schema: bool,
    /// The descriptor vocabulary is shared, never re-worded per surface.
    pub descriptor_vocabulary_shared_not_reworded: bool,
    /// No consumer invents a private / alternate state name.
    pub no_consumer_invents_private_state_names: bool,
    /// State semantics, state cause, consequence/recovery, and the accessibility label stay
    /// explicit everywhere.
    pub state_cause_recovery_accessibility_explicit_on_every_surface: bool,
    /// Unresolved cause, unavailable recovery, a re-resolved lock owner, and a reduced
    /// accessibility route auto-narrow the claim.
    pub degraded_state_auto_narrows_claim: bool,
    /// A narrowed rendering always shows a self-contained auto-narrow banner.
    pub narrowed_rendering_always_shows_self_contained_banner: bool,
    /// The banner names an exact reason and recovery action, never a generic note.
    pub banner_names_exact_reason_and_recovery_action: bool,
    /// An incomplete or degraded state never masquerades as an exact, healthy state.
    pub partial_state_never_shown_as_exact: bool,
    /// The support / recovery lane and docs present the same state cause/recovery truth shown
    /// in-product.
    pub support_docs_present_same_state_truth: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel consumer-adoption vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentConsumerProjection {
    /// Shell, command/help, search, review, settings, provider, test, and support all adopt the
    /// shared components.
    pub all_consumers_adopt_shared_components: bool,
    /// The state-semantics descriptor reads a single canonical source.
    pub state_semantics_reads_single_source: bool,
    /// The state-cause descriptor reads a single canonical source.
    pub state_cause_reads_single_source: bool,
    /// The consequence/recovery descriptor reads a single canonical source.
    pub consequence_and_recovery_reads_single_source: bool,
    /// The accessibility-label descriptor reads a single canonical source.
    pub accessibility_label_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the projection.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the consumer lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentConsumerReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting state-component consumer audit.
    pub state_component_consumer_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5StateComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StateComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5StateComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5StateComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5StateComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5StateComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5StateComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5StateComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 shared-state-taxonomy component-consumer packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StateComponentConsumerPacket {
    /// Record kind; must equal [`M5_STATE_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_STATE_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Consumer rows.
    pub consumer_rows: Vec<M5StateComponentConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5StateComponentConsumerVocabularySet,
    /// Governance-review block.
    pub governance_review: M5StateComponentConsumerGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5StateComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5StateComponentConsumerProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5StateComponentConsumerReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5StateComponentConsumerPacket {
    /// Builds an M5 shared-state-taxonomy component-consumer packet from stable-lane input.
    pub fn new(input: M5StateComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: M5_STATE_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: M5_STATE_COMPONENT_CONSUMER_SCHEMA_VERSION,
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

    /// Validates the M5 shared-state-taxonomy component-consumer invariants.
    pub fn validate(&self) -> Vec<M5StateComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_STATE_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(M5StateComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != M5_STATE_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(M5StateComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5StateComponentConsumerViolation::MissingIdentity);
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
            &serde_json::to_value(self).expect("m5 state component consumer packet serializes"),
        ) {
            violations.push(M5StateComponentConsumerViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 state component consumer packet serializes")
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
        out.push_str("# M5 Shared-State-Taxonomy Component Consumer Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Consumers: {} ({} stable)\n",
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
        out.push_str("\n## Consumers\n\n");
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

/// Errors emitted when reading the checked-in M5 state-component-consumer export.
#[derive(Debug)]
pub enum M5StateComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5StateComponentConsumerViolation>),
}

impl fmt::Display for M5StateComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 state component consumer export parse failed: {error}"
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
                    "m5 state component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5StateComponentConsumerArtifactError {}

/// Validation failures emitted by [`M5StateComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5StateComponentConsumerViolation {
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
    /// A required consumer is missing from the matrix.
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
    /// No worked binding proves that an incomplete or degraded state narrows and never asserts
    /// exact state parity, or a binding does so incorrectly.
    ExactParityHonestyUnproven,
    /// The support / recovery consumer does not reference the canonical component schema.
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

impl M5StateComponentConsumerViolation {
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

/// Reads and validates the checked-in stable M5 state-component-consumer export.
pub fn current_stable_m5_state_component_consumer_export(
) -> Result<M5StateComponentConsumerPacket, M5StateComponentConsumerArtifactError> {
    let packet: M5StateComponentConsumerPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shared-state-taxonomy-component-consumer-proof/support_export.json"
    )))
    .map_err(M5StateComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5StateComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_STATE_COMPONENT_CONSUMER_SCHEMA_REF,
        M5_STATE_COMPONENT_CONSUMER_DOC_REF,
        M5_STATE_COMPONENT_CONSUMER_COMPONENT_MATRIX_REF,
        M5_STATE_COMPONENT_CONSUMER_OBJECT_MODEL_REF,
        M5_STATE_COMPONENT_CONSUMER_STATE_CLASS_REF,
        M5_STATE_COMPONENT_CONSUMER_STATE_RECOVERY_REF,
        M5_INTERACTIVE_STATE_CONTRACT_SCHEMA_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF,
        M5_DEGRADED_STATE_CONTRACT_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5StateComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5StateComponentConsumerViolation::VocabularySetDrift);
    }
}

fn validate_consumer_rows(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    let present: BTreeSet<M5StateComponentConsumer> = packet
        .consumer_rows
        .iter()
        .map(|row| row.consumer)
        .collect();
    for required in M5StateComponentConsumer::ALL {
        if !present.contains(&required) {
            violations.push(M5StateComponentConsumerViolation::RequiredConsumerMissing);
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
            violations.push(M5StateComponentConsumerViolation::ConsumerRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5StateComponentConsumerViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_required_descriptors() {
            violations.push(M5StateComponentConsumerViolation::RequiredDescriptorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5StateComponentConsumerViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5StateComponentConsumerViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5StateComponentConsumerViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5StateComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if row.component_bindings.is_empty() {
            violations.push(M5StateComponentConsumerViolation::ComponentBindingMissing);
        }
        if !row.all_bindings_point_to_canonical() {
            violations.push(M5StateComponentConsumerViolation::CanonicalRefMismatch);
        }
        if row
            .component_bindings
            .iter()
            .any(|binding| binding.example_bindings.is_empty())
        {
            violations.push(M5StateComponentConsumerViolation::ExampleBindingMissing);
        }
        if row.component_bindings.iter().any(|binding| {
            binding
                .example_bindings
                .iter()
                .any(|case| !case.is_self_consistent())
        }) {
            violations.push(M5StateComponentConsumerViolation::ExampleBindingDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5StateComponentConsumerViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5StateComponentConsumerViolation::ConsumerInvariantViolated);
        }
    }
}

/// Every canonical component family must be adopted by at least two distinct consumers — the
/// acceptance-criterion proof that the families are reusable contracts rather than a design-system
/// island.
fn validate_family_reuse(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    for family in M5SharedComponentStateFamily::ALL {
        let consumers_adopting = packet
            .consumer_rows
            .iter()
            .filter(|row| row.adopted_families().contains(&family))
            .count();
        if consumers_adopting < 2 {
            violations.push(M5StateComponentConsumerViolation::ComponentFamilyReuseUnproven);
            return;
        }
    }
}

/// At least one worked binding across the matrix must prove a narrowed rendering whose banner
/// carries a specific reason, a recovery action, and a non-empty set of preserved descriptors —
/// the acceptance-criterion example that a consumer which cannot preserve parity is visibly
/// narrowed rather than inheriting stronger labels from healthier profiles.
fn validate_narrowing_disclosure(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
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
        violations.push(M5StateComponentConsumerViolation::NarrowingDisclosureUnproven);
    }
}

/// At least one worked binding across the matrix must prove a full-parity rendering with preserved
/// parity and no banner — the acceptance-criterion example that full-parity consumers keep the
/// descriptor vocabulary without a spurious narrowing note.
fn validate_scope_preserved(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    let proven = all_cases(packet).any(|case| {
        !case.resolved.is_narrowed
            && case.resolved.auto_narrow_banner.is_none()
            && case.resolved.claim_parity_state == M5StateClaimParityState::ClaimsPreserved
    });
    if !proven {
        violations.push(M5StateComponentConsumerViolation::ScopePreservedUnproven);
    }
}

/// Every worked binding that reflects an incomplete or degraded state must be narrowed and must
/// not assert exact state parity, and at least one such binding must be present — the
/// acceptance-criterion that an incomplete or degraded state no longer masquerades as an exact,
/// healthy state on any claimed consumer.
fn validate_exact_parity_honesty(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    let mut proven = false;
    for case in all_cases(packet) {
        let resolved = &case.resolved;
        if resolved.reflects_incomplete_or_degraded_state {
            // An incomplete / degraded binding that claims exact parity, or fails to narrow, breaks
            // the acceptance criterion.
            if resolved.asserts_exact_state_parity
                || !resolved.is_narrowed
                || resolved.claim_parity_state != M5StateClaimParityState::ClaimsAutoNarrowed
            {
                violations.push(M5StateComponentConsumerViolation::ExactParityHonestyUnproven);
                return;
            }
            proven = true;
        }
    }
    if !proven {
        violations.push(M5StateComponentConsumerViolation::ExactParityHonestyUnproven);
    }
}

/// The support / recovery consumer must reference the canonical component schema for each family
/// it adopts — the acceptance-criterion that a support / export lane can never drift from the
/// product truth.
fn validate_support_export_reference(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    for row in &packet.consumer_rows {
        if !row.consumer.is_support_or_export() {
            continue;
        }
        let references_canonical = !row.component_bindings.is_empty()
            && row
                .component_bindings
                .iter()
                .all(M5StateComponentBinding::points_to_canonical_family);
        if !references_canonical {
            violations.push(M5StateComponentConsumerViolation::SupportExportReferenceMissing);
            return;
        }
    }
}

fn validate_governance_review(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.consumers_adopt_shared_primitives,
        review.consumers_reference_canonical_schema,
        review.descriptor_vocabulary_shared_not_reworded,
        review.no_consumer_invents_private_state_names,
        review.state_cause_recovery_accessibility_explicit_on_every_surface,
        review.degraded_state_auto_narrows_claim,
        review.narrowed_rendering_always_shows_self_contained_banner,
        review.banner_names_exact_reason_and_recovery_action,
        review.partial_state_never_shown_as_exact,
        review.support_docs_present_same_state_truth,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5StateComponentConsumerViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.all_consumers_adopt_shared_components,
        projection.state_semantics_reads_single_source,
        projection.state_cause_reads_single_source,
        projection.consequence_and_recovery_reads_single_source,
        projection.accessibility_label_reads_single_source,
    ] {
        if !ok {
            violations.push(M5StateComponentConsumerViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5StateComponentConsumerViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5StateComponentConsumerPacket,
    violations: &mut Vec<M5StateComponentConsumerViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.state_component_consumer_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5StateComponentConsumerViolation::ReleasePostureIncomplete);
    }
}

/// Iterates every worked binding case across the matrix.
fn all_cases(
    packet: &M5StateComponentConsumerPacket,
) -> impl Iterator<Item = &M5StateComponentBindingCase> {
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
