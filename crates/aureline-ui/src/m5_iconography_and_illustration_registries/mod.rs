//! Implemented M5 iconography (semantic, labeled icon) and illustration-boundary registries.
//!
//! The frozen [motion / layer / iconography matrix][matrix] names Aureline's seven visual-interaction
//! families and locks their controlled vocabulary. This module is the icon / illustration implement lane over
//! that matrix: it turns the two families that carry the *symbol language* grammar — the **iconography**
//! registry (semantic shell / action / status / navigation / file-type / trust-overlay icon categories that
//! carry a text-label or tooltip equivalent and reuse one metaphor across commands and surfaces) and the
//! **illustration-boundary** registry (onboarding and empty-state illustration kept secondary, calm, and
//! non-anthropomorphic that never stands in for operational or security truth) — into registry resolvers that
//! produce export-safe, honest projections, so a user can trust that every icon stays semantic and labeled,
//! that file-type meaning never collapses into shell or trust/status meaning in a dense surface, and that an
//! illustration never masquerades as operational state, safety approval, or a security message.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement the canonical shell / action / status icon rules with tooltip or label parity, accessible
//!   text equivalents, and stable reuse of metaphors across commands and surfaces.**
//!   [`resolve_icon_entry`] refuses to read as a clean, semantic icon entry unless it names a canonical token,
//!   a classified [meaning class][M5IconMeaningClass], an iconography role, and a surface context, carries an
//!   accessible text equivalent, and reuses a stable metaphor rather than a private icon grammar; otherwise it
//!   degrades.
//! * **Preserve explicit boundaries between shell / action icons, file-type icons, and trust / status
//!   overlays so meaning does not collapse in dense surfaces.** Every icon entry carries the `boundary_distinct`
//!   guard, and degrades to [`M5IconEntryDegradeReason::FileTypeShellStatusBoundaryCollapsed`] when file-type,
//!   shell/action, and trust/status meaning would otherwise collapse together.
//! * **Wire first shell / explorer / tab / result-row / onboarding consumers plus fixtures so illustration use
//!   stays secondary.** [`resolve_illustration_entry`] refuses to read as a clean, secondary illustration entry
//!   unless it names a canonical token, an illustration role, and a [placement][M5IllustrationPlacement], stays
//!   secondary to content, and never impersonates operational or security truth or replaces operational
//!   messaging; an illustration used as trust or state degrades honestly. Each registry row carries the render
//!   [surface context][M5IconIllustrationSurfaceContext] so an icon-semantics or illustration-boundary
//!   regression degrades honestly, and the acceptance-criteria gate proves the first claimed consumers use one
//!   canonical symbol language before release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5VisualInteractionRole`] role
//! vocabulary, the [`M5IconographyRole`] iconography vocabulary, and the [`M5IllustrationRole`] illustration
//! vocabulary — so shell, editor, onboarding, marketplace, settings, and support surfaces can never fork their
//! own icon or illustration meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_motion_layer_iconography_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_iconography_and_illustration_registries,
    seeded_m5_iconography_and_illustration_registries_onboarding_ui_preview_narrowed,
    seeded_m5_iconography_and_illustration_registries_shell_ui_beta_narrowed,
    M5_ICON_ILLUSTRATION_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_motion_layer_iconography_matrix::{
    M5IconographyRole, M5IllustrationRole, M5VisualInteractionAccessibilityRoute,
    M5VisualInteractionConsumerSurface, M5VisualInteractionDeploymentLine,
    M5VisualInteractionDowngradeTrigger, M5VisualInteractionFamily,
    M5VisualInteractionQualificationClass, M5VisualInteractionRequiredLabel,
    M5VisualInteractionRole, M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF,
    M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF, M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5IconIllustrationRegistriesPacket`].
pub const M5_ICON_ILLUSTRATION_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_iconography_and_illustration_registries";

/// Schema version for M5 iconography and illustration registry records.
pub const M5_ICON_ILLUSTRATION_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_ICON_ILLUSTRATION_REGISTRIES_SCHEMA_REF: &str =
    "schemas/design-system/m5-iconography-and-illustration-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_ICON_ILLUSTRATION_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_iconography_and_illustration_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ICON_ILLUSTRATION_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-iconography-and-illustration-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_ICON_ILLUSTRATION_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-iconography-and-illustration-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_ICON_ILLUSTRATION_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-iconography-and-illustration-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_ICON_ILLUSTRATION_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-iconography-and-illustration-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5IconIllustrationRegistriesConsumerSurface = M5VisualInteractionConsumerSurface;

/// Controlled icon meaning class an icon entry maps, so shell chrome, common actions, status severity,
/// navigation, file-type documents, and trust / status overlays stay distinct symbol categories rather than
/// collapsing into one another in a dense explorer, tab strip, or result row. Minted by this lane because the
/// frozen matrix carries the high-level iconography role but not the concrete named meaning classes the
/// file-type-versus-shell/status acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconMeaningClass {
    /// The shell / chrome icon class.
    ShellIcon,
    /// The common-action icon class.
    ActionIcon,
    /// The status / severity icon class.
    StatusIcon,
    /// The navigation icon class.
    NavigationIcon,
    /// The file-type / document icon class.
    FileTypeIcon,
    /// The trust / status overlay class (a badge layered over another icon).
    TrustStatusOverlay,
    /// The icon meaning class is unclassified, which is disallowed.
    MeaningUnclassified,
}

impl M5IconMeaningClass {
    /// Every icon meaning class, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ShellIcon,
        Self::ActionIcon,
        Self::StatusIcon,
        Self::NavigationIcon,
        Self::FileTypeIcon,
        Self::TrustStatusOverlay,
        Self::MeaningUnclassified,
    ];

    /// The six canonical meaning classes the icon registry names.
    pub const CANONICAL_CLASSES: [Self; 6] = [
        Self::ShellIcon,
        Self::ActionIcon,
        Self::StatusIcon,
        Self::NavigationIcon,
        Self::FileTypeIcon,
        Self::TrustStatusOverlay,
    ];

    /// The meaning classes whose boundary must stay distinct in dense surfaces so file-type meaning never
    /// collapses into shell / action or trust / status meaning.
    pub const BOUNDARY_SENSITIVE: [Self; 4] = [
        Self::ShellIcon,
        Self::StatusIcon,
        Self::FileTypeIcon,
        Self::TrustStatusOverlay,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellIcon => "shell_icon",
            Self::ActionIcon => "action_icon",
            Self::StatusIcon => "status_icon",
            Self::NavigationIcon => "navigation_icon",
            Self::FileTypeIcon => "file_type_icon",
            Self::TrustStatusOverlay => "trust_status_overlay",
            Self::MeaningUnclassified => "meaning_unclassified",
        }
    }

    /// Whether the meaning class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::MeaningUnclassified)
    }

    /// Whether this is a boundary-sensitive class that must stay distinct in dense surfaces.
    pub const fn is_boundary_sensitive(self) -> bool {
        matches!(
            self,
            Self::ShellIcon | Self::StatusIcon | Self::FileTypeIcon | Self::TrustStatusOverlay
        )
    }
}

/// Controlled illustration placement mode an illustration entry pairs with its role so it stays secondary to
/// content: an empty-state accent, an onboarding accent, a decorative accent, a calm non-anthropomorphic
/// figure, or an accent subordinate to the operational messaging. Minted by this lane, tracking the
/// secondary / calm / non-anthropomorphic rule the illustration acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IllustrationPlacement {
    /// A secondary empty-state accent.
    EmptyStateSecondary,
    /// A secondary onboarding accent.
    OnboardingSecondary,
    /// A decorative accent.
    DecorativeAccent,
    /// A calm, non-anthropomorphic figure.
    CalmNonAnthropomorphic,
    /// An accent kept subordinate to the operational messaging.
    SubordinateToMessaging,
    /// No placement is paired with the illustration, which is disallowed.
    NoneDisallowed,
}

impl M5IllustrationPlacement {
    /// Every placement, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EmptyStateSecondary,
        Self::OnboardingSecondary,
        Self::DecorativeAccent,
        Self::CalmNonAnthropomorphic,
        Self::SubordinateToMessaging,
        Self::NoneDisallowed,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStateSecondary => "empty_state_secondary",
            Self::OnboardingSecondary => "onboarding_secondary",
            Self::DecorativeAccent => "decorative_accent",
            Self::CalmNonAnthropomorphic => "calm_non_anthropomorphic",
            Self::SubordinateToMessaging => "subordinate_to_messaging",
            Self::NoneDisallowed => "none_disallowed",
        }
    }

    /// Whether a placement is present (never the disallowed none sentinel).
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoneDisallowed)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so an icon's meaning or an
/// illustration's boundary stays stable whether it appears in the shell, a file explorer, a tab strip, a
/// result row, or an onboarding surface. Minted by this lane, tracking the first-consumer surfaces the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconIllustrationSurfaceContext {
    /// The shell surface.
    Shell,
    /// The file explorer surface.
    Explorer,
    /// The tab-strip surface.
    Tab,
    /// The search / result-row surface.
    ResultRow,
    /// The onboarding / empty-state surface.
    Onboarding,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5IconIllustrationSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Explorer,
        Self::Tab,
        Self::ResultRow,
        Self::Onboarding,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::Shell,
        Self::Explorer,
        Self::Tab,
        Self::ResultRow,
        Self::Onboarding,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Explorer => "explorer",
            Self::Tab => "tab",
            Self::ResultRow => "result_row",
            Self::Onboarding => "onboarding",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part an icon or illustration entry must be able to show, so no meaning class,
/// placement, label, or token fact is left implicit behind a bare glyph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconIllustrationRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical token reference the entry points at.
    TokenReference,
    /// The icon meaning class the entry maps (icon entry).
    MeaningClass,
    /// The accessible text equivalent / tooltip label (icon entry).
    AccessibleLabel,
    /// The iconography role named by the entry (icon entry).
    IconographyRole,
    /// The illustration role named by the entry (illustration entry).
    IllustrationRole,
    /// The illustration placement paired with the role (illustration entry).
    IllustrationPlacement,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the token (both entries).
    PlainLanguageMeaning,
}

impl M5IconIllustrationRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::MeaningClass,
        Self::AccessibleLabel,
        Self::IconographyRole,
        Self::IllustrationRole,
        Self::IllustrationPlacement,
        Self::SurfaceContext,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::TokenReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::TokenReference => "token_reference",
            Self::MeaningClass => "meaning_class",
            Self::AccessibleLabel => "accessible_label",
            Self::IconographyRole => "iconography_role",
            Self::IllustrationRole => "illustration_role",
            Self::IllustrationPlacement => "illustration_placement",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect icon
/// semantics, add an accessible label, restore a secondary illustration, or trace a degraded token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconIllustrationRegistryNextAction {
    /// Expand the icon / illustration's plain-language meaning.
    ExpandIconMeaning,
    /// Inspect the icon semantics the entry maps.
    InspectIconSemantics,
    /// Add an accessible text label / tooltip to the icon.
    AddAccessibleLabel,
    /// Restore the illustration to a secondary, non-impersonating placement.
    RestoreSecondaryIllustration,
    /// Trace the entry back to its canonical token.
    TraceCanonicalToken,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5IconIllustrationRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ExpandIconMeaning,
        Self::InspectIconSemantics,
        Self::AddAccessibleLabel,
        Self::RestoreSecondaryIllustration,
        Self::TraceCanonicalToken,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandIconMeaning => "expand_icon_meaning",
            Self::InspectIconSemantics => "inspect_icon_semantics",
            Self::AddAccessibleLabel => "add_accessible_label",
            Self::RestoreSecondaryIllustration => "restore_secondary_illustration",
            Self::TraceCanonicalToken => "trace_canonical_token",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconIllustrationRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The interaction families covered.
    InteractionFamilies,
    /// The icon meaning classes carried.
    MeaningClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// Whether icon entries carry an accessible-label parity.
    AccessibleLabelParity,
    /// The illustration placements paired.
    IllustrationPlacements,
    /// The render / surface context.
    SurfaceContext,
    /// The illustration roles named.
    IllustrationRoles,
    /// The accountable owner role.
    OwnerRole,
}

impl M5IconIllustrationRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::InteractionFamilies,
        Self::MeaningClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::AccessibleLabelParity,
        Self::IllustrationPlacements,
        Self::SurfaceContext,
        Self::IllustrationRoles,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::InteractionFamilies,
        Self::MeaningClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::InteractionFamilies => "interaction_families",
            Self::MeaningClasses => "meaning_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::AccessibleLabelParity => "accessible_label_parity",
            Self::IllustrationPlacements => "illustration_placements",
            Self::SurfaceContext => "surface_context",
            Self::IllustrationRoles => "illustration_roles",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an icon entry degraded below a clean, semantic, labeled state. The degrade-first ladder returns one
/// of these instead of ever letting an unlabeled, private-grammar, boundary-collapsed, or unstable-metaphor
/// entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconEntryDegradeReason {
    /// The canonical token name is unstated; a user cannot trace what the icon means.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The icon meaning class is unclassified (not shell / action / status / navigation / file-type / trust).
    IconMeaningUnclassified,
    /// An uncommon or destructive action uses an unlabeled icon with no accessible text equivalent.
    UnlabeledIconForUncommonOrDestructive,
    /// A private icon grammar is used instead of tracing to a canonical token.
    PrivateIconGrammarInsteadOfToken,
    /// The icon metaphor is not reused stably across commands and surfaces.
    MetaphorReuseUnstable,
    /// File-type, shell / action, and trust / status meaning collapse together in a dense surface.
    FileTypeShellStatusBoundaryCollapsed,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5IconEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::IconMeaningUnclassified,
        Self::UnlabeledIconForUncommonOrDestructive,
        Self::PrivateIconGrammarInsteadOfToken,
        Self::MetaphorReuseUnstable,
        Self::FileTypeShellStatusBoundaryCollapsed,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::IconMeaningUnclassified => "icon_meaning_unclassified",
            Self::UnlabeledIconForUncommonOrDestructive => {
                "unlabeled_icon_for_uncommon_or_destructive"
            }
            Self::PrivateIconGrammarInsteadOfToken => "private_icon_grammar_instead_of_token",
            Self::MetaphorReuseUnstable => "metaphor_reuse_unstable",
            Self::FileTypeShellStatusBoundaryCollapsed => {
                "file_type_shell_status_boundary_collapsed"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5IconIllustrationRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::PrivateIconGrammarInsteadOfToken => {
                M5IconIllustrationRegistryNextAction::TraceCanonicalToken
            }
            Self::IconMeaningUnclassified
            | Self::MetaphorReuseUnstable
            | Self::FileTypeShellStatusBoundaryCollapsed => {
                M5IconIllustrationRegistryNextAction::InspectIconSemantics
            }
            Self::UnlabeledIconForUncommonOrDestructive => {
                M5IconIllustrationRegistryNextAction::AddAccessibleLabel
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5IconIllustrationRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            Self::UnlabeledIconForUncommonOrDestructive => {
                M5VisualInteractionDowngradeTrigger::UnlabeledIconForUncommonOrDestructiveAction
            }
            Self::IconMeaningUnclassified
            | Self::MetaphorReuseUnstable
            | Self::FileTypeShellStatusBoundaryCollapsed => {
                M5VisualInteractionDowngradeTrigger::IconSemanticsAmbiguous
            }
            Self::TokenNameUnstated | Self::PrivateIconGrammarInsteadOfToken => {
                M5VisualInteractionDowngradeTrigger::TokenReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5VisualInteractionDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualInteractionDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an illustration entry degraded below a clean, secondary, non-impersonating state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IllustrationEntryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// An illustration impersonates operational, safety, or security truth (a disallowed role or a claim).
    IllustrationImpersonatesOperationalOrSecurityTruth,
    /// An illustration replaces the operational messaging or a trust explanation instead of staying secondary.
    ReplacesOperationalMessaging,
    /// No placement is paired with the illustration.
    PlacementModeMissing,
    /// A private illustration grammar is used instead of tracing to a canonical token.
    PrivateIllustrationGrammarInsteadOfToken,
    /// The illustration is not kept secondary to content (becomes primary or anthropomorphic).
    NotSecondaryToContent,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5IllustrationEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::IllustrationImpersonatesOperationalOrSecurityTruth,
        Self::ReplacesOperationalMessaging,
        Self::PlacementModeMissing,
        Self::PrivateIllustrationGrammarInsteadOfToken,
        Self::NotSecondaryToContent,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::IllustrationImpersonatesOperationalOrSecurityTruth => {
                "illustration_impersonates_operational_or_security_truth"
            }
            Self::ReplacesOperationalMessaging => "replaces_operational_messaging",
            Self::PlacementModeMissing => "placement_mode_missing",
            Self::PrivateIllustrationGrammarInsteadOfToken => {
                "private_illustration_grammar_instead_of_token"
            }
            Self::NotSecondaryToContent => "not_secondary_to_content",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5IconIllustrationRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::PrivateIllustrationGrammarInsteadOfToken => {
                M5IconIllustrationRegistryNextAction::TraceCanonicalToken
            }
            Self::IllustrationImpersonatesOperationalOrSecurityTruth
            | Self::ReplacesOperationalMessaging
            | Self::PlacementModeMissing
            | Self::NotSecondaryToContent => {
                M5IconIllustrationRegistryNextAction::RestoreSecondaryIllustration
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5IconIllustrationRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            Self::IllustrationImpersonatesOperationalOrSecurityTruth
            | Self::ReplacesOperationalMessaging
            | Self::PlacementModeMissing
            | Self::NotSecondaryToContent => {
                M5VisualInteractionDowngradeTrigger::IllustrationImpersonatedOperationalState
            }
            Self::TokenNameUnstated | Self::PrivateIllustrationGrammarInsteadOfToken => {
                M5VisualInteractionDowngradeTrigger::TokenReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5VisualInteractionDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualInteractionDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_icon_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5IconEntryResolutionInput {
    /// Stable identity of the icon entry.
    pub entry_id: String,
    /// The canonical token name (e.g. `icon.action.save`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualInteractionRole,
    /// The iconography role (from the frozen matrix vocabulary).
    pub iconography_role: M5IconographyRole,
    /// The icon meaning class this entry maps.
    pub meaning_class: M5IconMeaningClass,
    /// The render / surface context.
    pub surface_context: M5IconIllustrationSurfaceContext,
    /// True when the icon carries an accessible text equivalent / tooltip label (never unlabeled for an
    /// uncommon or destructive action).
    pub has_accessible_text_equivalent: bool,
    /// True when the icon reuses a stable metaphor across commands and surfaces.
    pub reuses_stable_metaphor: bool,
    /// True when the icon keeps its meaning distinct from file-type / shell / trust meaning in dense surfaces.
    pub boundary_distinct: bool,
    /// True when the entry traces to a canonical token (never a private icon grammar).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe icon projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedIconEntry {
    /// Stable identity of the icon entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands an accessible fallback.
    pub semantic_role_demands_accessible_fallback: bool,
    /// The iconography-role token named by the entry.
    pub iconography_role: String,
    /// Whether the iconography role names the disallowed unlabeled-uncommon-or-destructive token.
    pub iconography_role_is_unlabeled_disallowed: bool,
    /// The icon-meaning-class token named by the entry.
    pub meaning_class: String,
    /// Whether the meaning class is classified into the canonical set.
    pub meaning_class_is_classified: bool,
    /// Whether this is a boundary-sensitive class that must stay distinct.
    pub meaning_class_is_boundary_sensitive: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the icon carries an accessible text equivalent / tooltip label.
    pub has_accessible_text_equivalent: bool,
    /// Whether the icon reuses a stable metaphor across commands and surfaces.
    pub reuses_stable_metaphor: bool,
    /// Whether the icon keeps its meaning distinct in dense surfaces.
    pub boundary_distinct: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean, semantic, labeled state.
    pub degrade_reason: Option<M5IconEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5IconIllustrationRegistryNextAction,
    /// Whether the icon semantics hold (clean entry naming every fact).
    pub icon_semantics_hold: bool,
}

impl M5ResolvedIconEntry {
    /// Whether this icon entry reads as a clean, semantic, labeled state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_illustration_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5IllustrationEntryResolutionInput {
    /// Stable identity of the illustration entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The illustration role (from the frozen matrix vocabulary).
    pub illustration_role: M5IllustrationRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualInteractionRole,
    /// The placement paired with the illustration role.
    pub placement: M5IllustrationPlacement,
    /// The render / surface context.
    pub surface_context: M5IconIllustrationSurfaceContext,
    /// True when the illustration stays secondary to content (calm, non-anthropomorphic).
    pub stays_secondary_to_content: bool,
    /// True when the illustration never impersonates operational, safety, or security truth.
    pub never_impersonates_operational_or_security_truth: bool,
    /// True when the illustration replaces the operational messaging / a trust explanation (disallowed — must
    /// be `false` for a clean pass).
    pub replaces_operational_messaging: bool,
    /// True when the entry traces to a canonical token (never a private illustration grammar).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe illustration projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedIllustrationEntry {
    /// Stable identity of the illustration entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The illustration-role token named by the entry.
    pub illustration_role: String,
    /// Whether the illustration role names the disallowed operational-truth token.
    pub illustration_role_is_operational_truth_disallowed: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands an accessible fallback.
    pub semantic_role_demands_accessible_fallback: bool,
    /// The placement token named by the entry.
    pub placement: String,
    /// Whether a placement is present.
    pub placement_present: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the illustration stays secondary to content.
    pub stays_secondary_to_content: bool,
    /// Whether the illustration never impersonates operational or security truth.
    pub never_impersonates_operational_or_security_truth: bool,
    /// Whether the illustration replaces the operational messaging.
    pub replaces_operational_messaging: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean, secondary, non-impersonating state.
    pub degrade_reason: Option<M5IllustrationEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5IconIllustrationRegistryNextAction,
    /// Whether the illustration boundary is preserved (clean entry naming every fact).
    pub illustration_boundary_preserved: bool,
}

impl M5ResolvedIllustrationEntry {
    /// Whether this illustration entry reads as a clean, secondary, non-impersonating state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5IconIllustrationResolutionError {
    /// The icon-entry id was empty.
    EmptyIconEntryId,
    /// The illustration-entry id was empty.
    EmptyIllustrationEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5IconIllustrationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyIconEntryId => "empty_icon_entry_id",
            Self::EmptyIllustrationEntryId => "empty_illustration_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5IconIllustrationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 iconography and illustration registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5IconIllustrationResolutionError {}

/// Resolves an icon entry so it stays semantic and labeled: the entry names its canonical token, semantic
/// role, iconography role, meaning class, and surface context, carries an accessible text equivalent, reuses a
/// stable metaphor, keeps file-type / shell / trust meaning distinct, and traces to a canonical token rather
/// than a private icon grammar.
pub fn resolve_icon_entry(
    input: M5IconEntryResolutionInput,
) -> Result<M5ResolvedIconEntry, M5IconIllustrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5IconIllustrationResolutionError::EmptyIconEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5IconIllustrationResolutionError::ForbiddenMaterial);
    }

    let iconography_role_is_unlabeled_disallowed = matches!(
        input.iconography_role,
        M5IconographyRole::UnlabeledUncommonOrDestructiveDisallowed
    );

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5IconEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5IconEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.meaning_class.is_classified() {
        Some(M5IconEntryDegradeReason::IconMeaningUnclassified)
    } else if iconography_role_is_unlabeled_disallowed || !input.has_accessible_text_equivalent {
        Some(M5IconEntryDegradeReason::UnlabeledIconForUncommonOrDestructive)
    } else if !input.references_canonical_token {
        Some(M5IconEntryDegradeReason::PrivateIconGrammarInsteadOfToken)
    } else if !input.reuses_stable_metaphor {
        Some(M5IconEntryDegradeReason::MetaphorReuseUnstable)
    } else if !input.boundary_distinct {
        Some(M5IconEntryDegradeReason::FileTypeShellStatusBoundaryCollapsed)
    } else if !input.proof_fresh {
        Some(M5IconEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5IconIllustrationRegistryNextAction::InspectIconSemantics,
    };

    Ok(M5ResolvedIconEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_accessible_fallback: input
            .semantic_role
            .demands_accessible_fallback(),
        iconography_role: input.iconography_role.as_str().to_owned(),
        iconography_role_is_unlabeled_disallowed,
        meaning_class: input.meaning_class.as_str().to_owned(),
        meaning_class_is_classified: input.meaning_class.is_classified(),
        meaning_class_is_boundary_sensitive: input.meaning_class.is_boundary_sensitive(),
        surface_context: input.surface_context.as_str().to_owned(),
        has_accessible_text_equivalent: input.has_accessible_text_equivalent,
        reuses_stable_metaphor: input.reuses_stable_metaphor,
        boundary_distinct: input.boundary_distinct,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        icon_semantics_hold: degrade_reason.is_none(),
    })
}

/// Resolves an illustration entry so it stays secondary to content: the entry names its canonical token,
/// illustration role, semantic role, placement, and surface context, stays secondary, never impersonates
/// operational or security truth, never replaces the operational messaging, and traces to a canonical token
/// rather than standing in for trust or state.
pub fn resolve_illustration_entry(
    input: M5IllustrationEntryResolutionInput,
) -> Result<M5ResolvedIllustrationEntry, M5IconIllustrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5IconIllustrationResolutionError::EmptyIllustrationEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5IconIllustrationResolutionError::ForbiddenMaterial);
    }

    let illustration_role_is_operational_truth_disallowed = matches!(
        input.illustration_role,
        M5IllustrationRole::IllustrationAsOperationalTruthDisallowed
    );

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5IllustrationEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5IllustrationEntryDegradeReason::SurfaceContextUnresolved)
    } else if illustration_role_is_operational_truth_disallowed
        || !input.never_impersonates_operational_or_security_truth
    {
        Some(M5IllustrationEntryDegradeReason::IllustrationImpersonatesOperationalOrSecurityTruth)
    } else if input.replaces_operational_messaging {
        Some(M5IllustrationEntryDegradeReason::ReplacesOperationalMessaging)
    } else if !input.placement.is_present() {
        Some(M5IllustrationEntryDegradeReason::PlacementModeMissing)
    } else if !input.references_canonical_token {
        Some(M5IllustrationEntryDegradeReason::PrivateIllustrationGrammarInsteadOfToken)
    } else if !input.stays_secondary_to_content {
        Some(M5IllustrationEntryDegradeReason::NotSecondaryToContent)
    } else if !input.proof_fresh {
        Some(M5IllustrationEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5IconIllustrationRegistryNextAction::ExpandIconMeaning,
    };

    Ok(M5ResolvedIllustrationEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        illustration_role: input.illustration_role.as_str().to_owned(),
        illustration_role_is_operational_truth_disallowed,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_accessible_fallback: input
            .semantic_role
            .demands_accessible_fallback(),
        placement: input.placement.as_str().to_owned(),
        placement_present: input.placement.is_present(),
        surface_context: input.surface_context.as_str().to_owned(),
        stays_secondary_to_content: input.stays_secondary_to_content,
        never_impersonates_operational_or_security_truth: input
            .never_impersonates_operational_or_security_truth,
        replaces_operational_messaging: input.replaces_operational_messaging,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        illustration_boundary_preserved: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved icon and illustration entries it must project
/// honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IconIllustrationRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5IconIllustrationRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5VisualInteractionQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5VisualInteractionDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5VisualInteractionRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5VisualInteractionAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5IconIllustrationRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5IconIllustrationRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    /// Resolved icon examples.
    pub icon_entries: Vec<M5ResolvedIconEntry>,
    /// Resolved illustration examples.
    pub illustration_entries: Vec<M5ResolvedIllustrationEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical icon / illustration domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: an uncommon or destructive action never uses an unlabeled icon. MUST be `false`.
    pub icon_uses_unlabeled_symbol_for_uncommon_or_destructive_action: bool,
    /// Hard invariant: file-type and shell / status meaning never collapse together. MUST be `false`.
    pub file_type_and_shell_status_meaning_collapsed: bool,
    /// Hard invariant: an illustration never impersonates operational or security truth. MUST be `false`.
    pub illustration_impersonates_operational_or_security_truth: bool,
    /// Hard invariant: a private icon / illustration grammar is never used instead of a token. MUST be `false`.
    pub private_icon_or_illustration_grammar_instead_of_token: bool,
}

impl M5IconIllustrationRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5IconIllustrationRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5IconIllustrationRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5IconIllustrationRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5IconIllustrationRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.icon_uses_unlabeled_symbol_for_uncommon_or_destructive_action
            && !self.file_type_and_shell_status_meaning_collapsed
            && !self.illustration_impersonates_operational_or_security_truth
            && !self.private_icon_or_illustration_grammar_instead_of_token
    }

    /// True when a clean icon entry preserves semantic-labeled safety: it traces to a canonical token, never
    /// names the disallowed unlabeled role, carries an accessible text equivalent, keeps a classified meaning
    /// class, reuses a stable metaphor, and keeps its boundary distinct.
    fn icon_is_honest(ex: &M5ResolvedIconEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.iconography_role_is_unlabeled_disallowed
                && ex.has_accessible_text_equivalent
                && ex.meaning_class_is_classified
                && ex.reuses_stable_metaphor
                && ex.boundary_distinct)
    }

    /// True when a clean illustration entry preserves its secondary boundary: it traces to a canonical token,
    /// never names the disallowed operational-truth role, stays secondary, never impersonates truth, never
    /// replaces the operational messaging, and pairs a placement.
    fn illustration_is_honest(ex: &M5ResolvedIllustrationEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.illustration_role_is_operational_truth_disallowed
                && ex.stays_secondary_to_content
                && ex.never_impersonates_operational_or_security_truth
                && !ex.replaces_operational_messaging
                && ex.placement_present)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.icon_entries.iter().all(Self::icon_is_honest)
            && self
                .illustration_entries
                .iter()
                .all(Self::illustration_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IconIllustrationRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Iconography-role tokens (bound from the frozen matrix).
    pub iconography_roles: Vec<String>,
    /// Illustration-role tokens (bound from the frozen matrix).
    pub illustration_roles: Vec<String>,
    /// Icon-meaning-class tokens (minted by this lane).
    pub meaning_classes: Vec<String>,
    /// Illustration-placement tokens (minted by this lane).
    pub placements: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Icon-entry degrade-reason tokens.
    pub icon_degrade_reasons: Vec<String>,
    /// Illustration-entry degrade-reason tokens.
    pub illustration_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5IconIllustrationRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5VisualInteractionRole::ALL, |v| v.as_str()),
            iconography_roles: tokens(&M5IconographyRole::ALL, |v| v.as_str()),
            illustration_roles: tokens(&M5IllustrationRole::ALL, |v| v.as_str()),
            meaning_classes: tokens(&M5IconMeaningClass::ALL, |v| v.as_str()),
            placements: tokens(&M5IllustrationPlacement::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5IconIllustrationSurfaceContext::ALL, |v| v.as_str()),
            icon_degrade_reasons: tokens(&M5IconEntryDegradeReason::ALL, |v| v.as_str()),
            illustration_degrade_reasons: tokens(&M5IllustrationEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5IconIllustrationRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5IconIllustrationRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5IconIllustrationRegistryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5VisualInteractionConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5IconIllustrationRegistriesGovernanceReview {
    /// The icon registry names a canonical token, iconography role, and meaning class for every entry.
    pub icon_registry_names_token_role_and_meaning_class: bool,
    /// The icon registry distinguishes shell / action / status / navigation / file-type / trust classes.
    pub icon_registry_covers_canonical_meaning_classes: bool,
    /// No uncommon or destructive action uses an unlabeled icon.
    pub no_unlabeled_icon_for_uncommon_or_destructive_action: bool,
    /// File-type and shell / status meaning stays distinct in dense surfaces.
    pub file_type_and_shell_status_meaning_stays_distinct: bool,
    /// Illustrations stay secondary and never impersonate operational, safety, or security truth.
    pub illustrations_stay_secondary_and_never_impersonate_truth: bool,
    /// Illustrations name a placement rather than standing in for operational messaging.
    pub illustrations_name_placement_not_operational_stand_in: bool,
    /// Icons and illustrations trace to canonical tokens rather than a private symbol grammar.
    pub icons_and_illustrations_trace_to_canonical_tokens: bool,
    /// Icon or illustration drift is caught by fixtures / diagnostics / release proof before stable promotion.
    pub icon_or_illustration_drift_caught_before_release: bool,
    /// The first shell / explorer / tab / result-row / onboarding consumers use the canonical icon grammar.
    pub first_consumers_use_canonical_icon_grammar: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IconIllustrationRegistriesConsumerProjection {
    /// The shell surface consumes the shared icon / illustration registries.
    pub shell_consumes_shared_registries: bool,
    /// The explorer surface consumes the shared icon / illustration registries.
    pub explorer_consumes_shared_registries: bool,
    /// The tab and result-row surfaces consume the shared icon / illustration registries.
    pub tab_and_result_row_consume_shared_registries: bool,
    /// The onboarding surface consumes the shared icon / illustration registries.
    pub onboarding_consumes_shared_registries: bool,
    /// Icon / illustration meaning traces back to the canonical iconography-and-illustration domain contract.
    pub icon_meaning_traces_to_domain_contract: bool,
    /// Support / export reads a single canonical icon / illustration registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IconIllustrationRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IconIllustrationRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-interaction audit for the lane.
    pub interaction_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5IconIllustrationRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5IconIllustrationRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5IconIllustrationRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5IconIllustrationRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5IconIllustrationRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5IconIllustrationRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5IconIllustrationRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5IconIllustrationRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 iconography and illustration registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5IconIllustrationRegistriesPacket {
    /// Record kind; must equal [`M5_ICON_ILLUSTRATION_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ICON_ILLUSTRATION_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5IconIllustrationRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5IconIllustrationRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5IconIllustrationRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5IconIllustrationRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5IconIllustrationRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5IconIllustrationRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5IconIllustrationRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5IconIllustrationRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_ICON_ILLUSTRATION_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_ICON_ILLUSTRATION_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
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

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5IconIllustrationRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ICON_ILLUSTRATION_REGISTRIES_RECORD_KIND {
            violations.push(M5IconIllustrationRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ICON_ILLUSTRATION_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5IconIllustrationRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5IconIllustrationRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5IconIllustrationRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 iconography and illustration registries packet serializes"),
        ) {
            violations.push(M5IconIllustrationRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 iconography and illustration registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,icon_entries,illustration_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .icon_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.illustration_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.icon_entries.len(),
                row.illustration_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Iconography and Illustration Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Meaning classes: {}\n",
            self.vocabulary_set.meaning_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Illustration placements: {}\n",
            self.vocabulary_set.placements.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Icon entries: {} / illustration entries: {}\n",
                row.icon_entries.len(),
                row.illustration_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5IconIllustrationRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5IconIllustrationRegistriesViolation>),
}

impl fmt::Display for M5IconIllustrationRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 iconography and illustration registries export parse failed: {error}"
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
                    "m5 iconography and illustration registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5IconIllustrationRegistriesArtifactError {}

/// Validation failures emitted by [`M5IconIllustrationRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5IconIllustrationRegistriesViolation {
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
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at the canonical icon / illustration domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (unlabeled, private-grammar, boundary-collapsed, or
    /// unstable-metaphor icon entry, or an impersonating / replacing / not-secondary illustration entry).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// First-consumer canonical adoption is not proven: clean entries do not cover the canonical semantic-role
    /// families or the first shell / explorer / tab / result-row / onboarding surfaces, no unlabeled example
    /// degrades, or a clean icon lacks an accessible label or a private grammar reads as clean.
    FirstConsumersStableIconSemanticsNotProven,
    /// File-type-versus-shell/status distinctness is not proven: clean icon entries do not cover the
    /// boundary-sensitive shell / status / file-type / trust classes while staying distinct, no
    /// boundary-collapse example degrades, or a clean icon collapses the boundary.
    FileTypeVersusShellStatusDistinctNotProven,
    /// Illustration secondary-boundary truth is not proven: clean illustration entries do not cover the first
    /// surfaces staying secondary and non-impersonating, no impersonating / replacing / not-secondary example
    /// degrades, clean illustrations do not trace to a canonical token, or a clean illustration impersonates.
    IllustrationNeverReplacesOperationalTruthNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5IconIllustrationRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::FirstConsumersStableIconSemanticsNotProven => {
                "first_consumers_stable_icon_semantics_not_proven"
            }
            Self::FileTypeVersusShellStatusDistinctNotProven => {
                "file_type_versus_shell_status_distinct_not_proven"
            }
            Self::IllustrationNeverReplacesOperationalTruthNotProven => {
                "illustration_never_replaces_operational_truth_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_iconography_and_illustration_registries_export(
) -> Result<M5IconIllustrationRegistriesPacket, M5IconIllustrationRegistriesArtifactError> {
    let packet: M5IconIllustrationRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-iconography-and-illustration-registries-proof/support_export.json"
    )))
    .map_err(M5IconIllustrationRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5IconIllustrationRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5IconIllustrationRegistriesPacket,
    violations: &mut Vec<M5IconIllustrationRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ICON_ILLUSTRATION_REGISTRIES_SCHEMA_REF,
        M5_ICON_ILLUSTRATION_REGISTRIES_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5IconIllustrationRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5IconIllustrationRegistriesPacket,
    violations: &mut Vec<M5IconIllustrationRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5IconIllustrationRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5IconIllustrationRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5IconIllustrationRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5IconIllustrationRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF) {
            violations.push(M5IconIllustrationRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.icon_entries.is_empty() || row.illustration_entries.is_empty() {
            violations.push(M5IconIllustrationRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5IconIllustrationRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5IconIllustrationRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5IconIllustrationRegistriesPacket,
    violations: &mut Vec<M5IconIllustrationRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.icon_registry_names_token_role_and_meaning_class,
        review.icon_registry_covers_canonical_meaning_classes,
        review.no_unlabeled_icon_for_uncommon_or_destructive_action,
        review.file_type_and_shell_status_meaning_stays_distinct,
        review.illustrations_stay_secondary_and_never_impersonate_truth,
        review.illustrations_name_placement_not_operational_stand_in,
        review.icons_and_illustrations_trace_to_canonical_tokens,
        review.icon_or_illustration_drift_caught_before_release,
        review.first_consumers_use_canonical_icon_grammar,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5IconIllustrationRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5IconIllustrationRegistriesPacket,
    violations: &mut Vec<M5IconIllustrationRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.explorer_consumes_shared_registries,
        projection.tab_and_result_row_consume_shared_registries,
        projection.onboarding_consumes_shared_registries,
        projection.icon_meaning_traces_to_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5IconIllustrationRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5IconIllustrationRegistriesPacket,
    violations: &mut Vec<M5IconIllustrationRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5IconIllustrationRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5IconIllustrationRegistriesPacket,
    violations: &mut Vec<M5IconIllustrationRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.interaction_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5IconIllustrationRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5IconIllustrationRegistriesPacket,
    violations: &mut Vec<M5IconIllustrationRegistriesViolation>,
) {
    let icons = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.icon_entries.iter())
    };
    let illustrations = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.illustration_entries.iter())
    };

    // AC1: the first claimed consumers show stable icon semantics with accessible labels and no private icon
    // grammar. Clean entries cover the icon / illustration semantic-role families and the first shell /
    // explorer / tab / result-row / onboarding surfaces, an unlabeled example degrades, no clean icon lacks an
    // accessible label, and no clean entry uses a private grammar.
    let clean_semantic_roles: BTreeSet<String> = icons()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .chain(
            illustrations()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.semantic_role.clone()),
        )
        .collect();
    let clean_surfaces: BTreeSet<String> = icons()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .chain(
            illustrations()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.surface_context.clone()),
        )
        .collect();
    let semantic_families_covered = ["icon", "illustration"]
        .iter()
        .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5IconIllustrationSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let unlabeled_degrades = icons().any(|ex| {
        ex.degrade_reason == Some(M5IconEntryDegradeReason::UnlabeledIconForUncommonOrDestructive)
    });
    let no_clean_unlabeled_or_private = !icons().any(|ex| {
        ex.is_clean() && (!ex.has_accessible_text_equivalent || !ex.references_canonical_token)
    }) && !illustrations()
        .any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(semantic_families_covered
        && first_surfaces_covered
        && unlabeled_degrades
        && no_clean_unlabeled_or_private)
    {
        violations.push(
            M5IconIllustrationRegistriesViolation::FirstConsumersStableIconSemanticsNotProven,
        );
    }

    // AC2: file-type versus shell / status meaning remains distinct in explorers, tabs, and result rows. Clean
    // icon entries cover every boundary-sensitive meaning class while staying boundary-distinct, a
    // boundary-collapse example degrades, and no clean icon collapses the boundary.
    let clean_boundary_classes: BTreeSet<String> = icons()
        .filter(|ex| {
            ex.is_clean() && ex.meaning_class_is_boundary_sensitive && ex.boundary_distinct
        })
        .map(|ex| ex.meaning_class.clone())
        .collect();
    let boundary_classes_covered = M5IconMeaningClass::BOUNDARY_SENSITIVE
        .iter()
        .all(|c| clean_boundary_classes.contains(c.as_str()));
    let boundary_collapse_degrades = icons().any(|ex| {
        ex.degrade_reason == Some(M5IconEntryDegradeReason::FileTypeShellStatusBoundaryCollapsed)
    });
    let no_clean_collapse = !icons().any(|ex| ex.is_clean() && !ex.boundary_distinct);
    if !(boundary_classes_covered && boundary_collapse_degrades && no_clean_collapse) {
        violations.push(
            M5IconIllustrationRegistriesViolation::FileTypeVersusShellStatusDistinctNotProven,
        );
    }

    // AC3: onboarding / empty-state illustration use does not replace operational messaging or trust
    // explanations. Clean illustration entries cover the first surfaces staying secondary and non-impersonating,
    // an impersonating example degrades, a replacing example degrades, a not-secondary drift example degrades,
    // clean illustrations trace to a canonical token, and no clean illustration impersonates.
    let clean_secondary_surfaces: BTreeSet<String> = illustrations()
        .filter(|ex| {
            ex.is_clean()
                && ex.stays_secondary_to_content
                && ex.never_impersonates_operational_or_security_truth
                && !ex.replaces_operational_messaging
        })
        .map(|ex| ex.surface_context.clone())
        .collect();
    let secondary_surfaces_covered = M5IconIllustrationSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_secondary_surfaces.contains(s.as_str()));
    let impersonates_degrades = illustrations().any(|ex| {
        ex.degrade_reason
            == Some(M5IllustrationEntryDegradeReason::IllustrationImpersonatesOperationalOrSecurityTruth)
    });
    let replaces_degrades = illustrations().any(|ex| {
        ex.degrade_reason == Some(M5IllustrationEntryDegradeReason::ReplacesOperationalMessaging)
    });
    let not_secondary_degrades = illustrations().any(|ex| {
        ex.degrade_reason == Some(M5IllustrationEntryDegradeReason::NotSecondaryToContent)
    });
    let traceable_illustration =
        illustrations().any(|ex| ex.is_clean() && ex.references_canonical_token);
    let no_clean_impersonates = !illustrations()
        .any(|ex| ex.is_clean() && !ex.never_impersonates_operational_or_security_truth);
    if !(secondary_surfaces_covered
        && impersonates_degrades
        && replaces_degrades
        && not_secondary_degrades
        && traceable_illustration
        && no_clean_impersonates)
    {
        violations.push(
            M5IconIllustrationRegistriesViolation::IllustrationNeverReplacesOperationalTruthNotProven,
        );
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The two interaction families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5VisualInteractionFamily; 2] = [
    M5VisualInteractionFamily::Iconography,
    M5VisualInteractionFamily::Illustration,
];
