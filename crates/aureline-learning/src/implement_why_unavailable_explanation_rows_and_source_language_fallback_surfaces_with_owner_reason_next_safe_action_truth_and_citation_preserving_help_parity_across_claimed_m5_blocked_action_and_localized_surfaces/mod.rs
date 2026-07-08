//! Two reusable M5 blocked-action / localized-help primitives — the why-unavailable explanation
//! row and the source-language fallback surface — so a user who hits a blocked action or a
//! localized help surface that is behind the canonical source is never left with vague
//! `not available` copy or an unsourced paraphrase.
//!
//! A why-unavailable explanation row names, from the row alone, the blocked action, the exact
//! reason it is unavailable, the owning boundary that gates it, the next safe action the user can
//! take, and the deeper docs / help path — so context, trust, policy, and runtime failures never
//! collapse into one generic disabled state. A source-language fallback surface preserves the
//! source-language text, the stable canonical ID, and a citation-preserving link back to the
//! canonical docs / help whenever localized guidance is incomplete or behind the source — so
//! localized support / training flows never drift into unsourced paraphrase.
//!
//! Aureline's frozen contextual-teaching / migration-bridge component matrix
//! ([`crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix`])
//! names both component families and freezes their controlled vocabulary — the blocked-action
//! owners (`policy_owner`, `workspace_admin`, `provider_service`, `upstream_dependency`,
//! `current_user_scope`, `unknown_owner`), the unavailable reason classes (`policy_blocked`,
//! `missing_permission`, `unmet_precondition`, `feature_flag_off`, `offline_unavailable`,
//! `unsupported_target`), the next-safe-action classes (`request_access`, `satisfy_precondition`,
//! `switch_context`, `open_settings`, `read_docs`, `no_safe_action`), the source-language classes
//! (`authored_locale`, `translated_locale`, `machine_translated`, `fallback_to_source`,
//! `mixed_locale`, `untranslated_source`), and the fallback-state classes (`localized_current`,
//! `source_language_shown`, `partial_translation`, `stale_translation`,
//! `citation_preserved_fallback`, `no_localization`) — plus the surface families, deployment
//! lines, consumer surfaces, accessibility routes, qualification classes, and downgrade triggers.
//! This module *implements* that contract as two reusable resolvers.
//!
//! The module has two resolvers:
//!
//! 1. [`resolve_why_unavailable_explanation_row`] — takes one blocked action's reason class,
//!    owning boundary, next-safe-action class, an opaque next-safe-action target, an opaque
//!    deeper-docs reference, a screen-reader announcement, and an opaque row identity, and
//!    produces one [`M5ResolvedWhyUnavailableRow`] carrying the derived why-unavailable posture
//!    (blocked-by-policy, missing-permission, precondition-unmet, feature-disabled,
//!    offline-unavailable, or unsupported-target), the failure domain (policy / trust / context /
//!    runtime), and the bounded take-next-safe-action / contact-blocking-owner / retry-when-
//!    resolved / open-deeper-docs / export-unavailable-evidence actions. It always names the
//!    blocked action, the exact reason, the owning boundary, the next safe action (or states there
//!    is none honestly), and links the deeper docs — never a generic disabled state.
//! 2. [`resolve_source_language_fallback`] — takes one localized surface's source-language class,
//!    fallback-state class, display locale, an opaque stable-ID reference, an opaque
//!    canonical-citation reference, an optional opaque source-language-text reference, a
//!    screen-reader announcement, and an opaque row identity, and produces one
//!    [`M5ResolvedSourceLanguageFallback`] carrying the derived localization posture and the
//!    bounded view-source-language-text / report-translation-gap / request-localization /
//!    open-canonical-citation / export-locale-evidence actions. It always preserves the
//!    source-language text, the stable ID, and the canonical citation — never an unsourced
//!    paraphrase.
//!
//! A single parity matrix — [`M5BlockedLocalizedRowPacket`] — binds one row per claimed M5
//! blocked-action / localized consumer (the command-help row, the menu-and-action row, the
//! inline-status row, the settings-and-docs row, and the support explanation export) to both
//! shared anatomies, the same frozen vocabularies, the derived postures, the bounded actions, the
//! export fields, and the non-visual accessibility routes, so the owner / reason / next-safe-
//! action / citation vocabulary stays identical across desktop, headless/export, and support
//! consumers.
//!
//! Raw error dumps, stack traces, credentials, and private endpoints stay outside the export
//! boundary; every blocked-action reference, docs reference, stable ID, citation, and
//! source-language-text reference is carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_blocked_localized_menu_and_action_row_beta_narrowed,
    seeded_m5_blocked_localized_row_packet,
    seeded_m5_blocked_localized_support_explanation_export_preview_narrowed,
    M5_BLOCKED_LOCALIZED_ROW_PACKET_ID,
};

// The blocked-action owners, unavailable reason classes, next-safe-action classes, source-language
// classes, fallback-state classes, surface families, deployment lines, consumer surfaces,
// accessibility routes, qualification classes, and downgrade triggers are frozen once, in the
// contextual-teaching / migration-bridge component matrix. This primitive reuses them verbatim so
// it never invents a parallel blocked-action or localized-help vocabulary.
pub use crate::freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix::{
    M5BlockedActionOwner, M5FallbackStateClass, M5NextSafeActionClass, M5SourceLanguageClass,
    M5TeachingAccessibilityRoute, M5TeachingConsumerSurface, M5TeachingDeploymentLine,
    M5TeachingDowngradeTrigger, M5TeachingQualificationClass, M5TeachingSurfaceFamily,
    M5UnavailableReasonClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5BlockedLocalizedRowPacket`].
pub const M5_BLOCKED_LOCALIZED_ROW_RECORD_KIND: &str =
    "implement_why_unavailable_explanation_rows_and_source_language_fallback_surfaces_with_owner_reason_next_safe_action_truth_and_citation_preserving_help_parity_across_claimed_m5_blocked_action_and_localized_surfaces";

/// Schema version for M5 why-unavailable / source-language records.
pub const M5_BLOCKED_LOCALIZED_ROW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the why-unavailable / source-language boundary schema.
pub const M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF: &str =
    "schemas/ui/m5-why-unavailable-source-language.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BLOCKED_LOCALIZED_ROW_DOC_REF: &str =
    "docs/help/m5_why_unavailable_source_language_primitive.md";

/// Repo-relative path of the frozen contextual-teaching / migration-bridge component matrix this
/// primitive narrows from.
pub const M5_BLOCKED_LOCALIZED_ROW_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json";

/// Repo-relative path of the feature-availability-row contract the why-unavailable row binds
/// against.
pub const M5_BLOCKED_LOCALIZED_ROW_FEATURE_AVAILABILITY_REF: &str =
    "schemas/ux/feature_availability_row.schema.json";

/// Repo-relative path of the locale-fallback-state contract the source-language fallback binds
/// against.
pub const M5_BLOCKED_LOCALIZED_ROW_LOCALE_FALLBACK_REF: &str =
    "schemas/ux/locale_fallback_state.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BLOCKED_LOCALIZED_ROW_FIXTURE_DIR: &str =
    "fixtures/ui/m5-why-unavailable-source-language-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BLOCKED_LOCALIZED_ROW_ARTIFACT_REF: &str =
    "artifacts/release/m5-why-unavailable-source-language-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_BLOCKED_LOCALIZED_ROW_CSV_REF: &str =
    "artifacts/release/m5-why-unavailable-source-language-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_BLOCKED_LOCALIZED_ROW_REPORT_REF: &str =
    "artifacts/design/m5-why-unavailable-source-language-primitive.md";

/// One claimed M5 blocked-action / localized consumer that renders both the why-unavailable
/// explanation row and the source-language fallback surface. These are the consumers the
/// acceptance criteria name — the command-help row, the menu-and-action row, the inline-status
/// row, the settings-and-docs row, and the support explanation export — so the same owner /
/// reason / next-safe-action / citation grammar works across command help, menus, inline states,
/// settings, and every related blocked-action or localized teaching moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BlockedLocalizedConsumerSurface {
    /// The command-help row surface.
    CommandHelpRow,
    /// The menu-and-action row surface.
    MenuAndActionRow,
    /// The inline-status row surface.
    InlineStatusRow,
    /// The settings-and-docs row surface.
    SettingsAndDocsRow,
    /// The support explanation-export surface.
    SupportExplanationExport,
}

impl M5BlockedLocalizedConsumerSurface {
    /// Every claimed blocked-action / localized consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CommandHelpRow,
        Self::MenuAndActionRow,
        Self::InlineStatusRow,
        Self::SettingsAndDocsRow,
        Self::SupportExplanationExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandHelpRow => "command_help_row",
            Self::MenuAndActionRow => "menu_and_action_row",
            Self::InlineStatusRow => "inline_status_row",
            Self::SettingsAndDocsRow => "settings_and_docs_row",
            Self::SupportExplanationExport => "support_explanation_export",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CommandHelpRow => "Command-Help Row",
            Self::MenuAndActionRow => "Menu-and-Action Row",
            Self::InlineStatusRow => "Inline-Status Row",
            Self::SettingsAndDocsRow => "Settings-and-Docs Row",
            Self::SupportExplanationExport => "Support Explanation Export",
        }
    }
}

// =========================================================================
// Family 1 — why-unavailable explanation row
// =========================================================================

/// The failure domain a blocked action belongs to. Derived from the unavailable reason so that
/// context, trust, policy, and runtime failures are always distinguished and never collapsed into
/// one generic disabled state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UnavailableFailureDomain {
    /// A policy decision blocks the action.
    Policy,
    /// A trust / permission boundary blocks the action.
    Trust,
    /// A contextual precondition or configuration blocks the action.
    Context,
    /// A runtime condition (offline, unsupported) blocks the action.
    Runtime,
}

impl M5UnavailableFailureDomain {
    /// Every failure domain, in declaration order.
    pub const ALL: [Self; 4] = [Self::Policy, Self::Trust, Self::Context, Self::Runtime];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Policy => "policy",
            Self::Trust => "trust",
            Self::Context => "context",
            Self::Runtime => "runtime",
        }
    }
}

/// The derived why-unavailable posture — the resolver's honest verdict about why an action is
/// blocked. Derived one-to-one from the frozen unavailable reason class so a policy, trust,
/// context, or runtime failure is always named for exactly what it is and never left as a generic
/// disabled state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WhyUnavailablePosture {
    /// Blocked by a policy decision.
    BlockedByPolicy,
    /// Blocked by a missing permission.
    MissingPermission,
    /// Blocked by an unmet precondition.
    PreconditionUnmet,
    /// Blocked because a feature is disabled.
    FeatureDisabled,
    /// Blocked because the action is unavailable while offline.
    OfflineUnavailable,
    /// Blocked because the target is unsupported.
    UnsupportedTarget,
}

impl M5WhyUnavailablePosture {
    /// Every why-unavailable posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BlockedByPolicy,
        Self::MissingPermission,
        Self::PreconditionUnmet,
        Self::FeatureDisabled,
        Self::OfflineUnavailable,
        Self::UnsupportedTarget,
    ];

    /// The posture that honestly reflects an unavailable reason class — one-to-one, never
    /// collapsing distinct reasons into one generic disabled state.
    pub const fn from_reason(reason: M5UnavailableReasonClass) -> Self {
        match reason {
            M5UnavailableReasonClass::PolicyBlocked => Self::BlockedByPolicy,
            M5UnavailableReasonClass::MissingPermission => Self::MissingPermission,
            M5UnavailableReasonClass::UnmetPrecondition => Self::PreconditionUnmet,
            M5UnavailableReasonClass::FeatureFlagOff => Self::FeatureDisabled,
            M5UnavailableReasonClass::OfflineUnavailable => Self::OfflineUnavailable,
            M5UnavailableReasonClass::UnsupportedTarget => Self::UnsupportedTarget,
        }
    }

    /// The failure domain this posture belongs to — proving context, trust, policy, and runtime
    /// failures are never collapsed together.
    pub const fn failure_domain(self) -> M5UnavailableFailureDomain {
        match self {
            Self::BlockedByPolicy => M5UnavailableFailureDomain::Policy,
            Self::MissingPermission => M5UnavailableFailureDomain::Trust,
            Self::PreconditionUnmet | Self::FeatureDisabled => M5UnavailableFailureDomain::Context,
            Self::OfflineUnavailable | Self::UnsupportedTarget => {
                M5UnavailableFailureDomain::Runtime
            }
        }
    }

    /// True when the block is transient and may clear on its own or with a retry — an unmet
    /// precondition or an offline condition.
    pub const fn is_transient(self) -> bool {
        matches!(self, Self::PreconditionUnmet | Self::OfflineUnavailable)
    }

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::MissingPermission => "missing_permission",
            Self::PreconditionUnmet => "precondition_unmet",
            Self::FeatureDisabled => "feature_disabled",
            Self::OfflineUnavailable => "offline_unavailable",
            Self::UnsupportedTarget => "unsupported_target",
        }
    }
}

/// Whether a blocked-action owner can be contacted or escalated to. The current user's own scope
/// and an unknown owner cannot.
const fn owner_is_contactable(owner: M5BlockedActionOwner) -> bool {
    matches!(
        owner,
        M5BlockedActionOwner::PolicyOwner
            | M5BlockedActionOwner::WorkspaceAdmin
            | M5BlockedActionOwner::ProviderService
            | M5BlockedActionOwner::UpstreamDependency
    )
}

/// One bounded action a why-unavailable explanation row offers, so a user hitting a blocked action
/// can always take the next safe action, reach the owning boundary, retry when the block clears,
/// open the deeper docs, or export the evidence — never trapped in a dead-end generic disabled
/// state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WhyUnavailableAction {
    /// Take the next safe action the row names.
    TakeNextSafeAction,
    /// Contact or escalate to the blocking owner.
    ContactBlockingOwner,
    /// Retry the action once the transient block clears.
    RetryWhenResolved,
    /// Open the deeper docs / help path.
    OpenDeeperDocs,
    /// Export the why-unavailable evidence.
    ExportUnavailableEvidence,
}

impl M5WhyUnavailableAction {
    /// Every why-unavailable action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TakeNextSafeAction,
        Self::ContactBlockingOwner,
        Self::RetryWhenResolved,
        Self::OpenDeeperDocs,
        Self::ExportUnavailableEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TakeNextSafeAction => "take_next_safe_action",
            Self::ContactBlockingOwner => "contact_blocking_owner",
            Self::RetryWhenResolved => "retry_when_resolved",
            Self::OpenDeeperDocs => "open_deeper_docs",
            Self::ExportUnavailableEvidence => "export_unavailable_evidence",
        }
    }
}

/// Controlled why-unavailable-row anatomy part the shared row surfaces. The parts in
/// [`M5WhyUnavailableAnatomyPart::MANDATORY`] are required on every row so the blocked action, the
/// reason, the owning boundary, the next safe action, and the deeper-docs path are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WhyUnavailableAnatomyPart {
    /// The blocked-action cue.
    BlockedActionCue,
    /// The unavailable-reason cue.
    UnavailableReasonCue,
    /// The blocking-owner cue.
    BlockingOwnerCue,
    /// The next-safe-action cue.
    NextSafeActionCue,
    /// The deeper-docs-path cue.
    DeeperDocsPathCue,
    /// The failure-domain cue.
    FailureDomainCue,
    /// The screen-reader-announcement cue.
    ScreenReaderAnnouncementCue,
    /// The evidence-export cue.
    EvidenceExportCue,
}

impl M5WhyUnavailableAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::BlockedActionCue,
        Self::UnavailableReasonCue,
        Self::BlockingOwnerCue,
        Self::NextSafeActionCue,
        Self::DeeperDocsPathCue,
        Self::FailureDomainCue,
        Self::ScreenReaderAnnouncementCue,
        Self::EvidenceExportCue,
    ];

    /// The anatomy parts every row must render.
    pub const MANDATORY: [Self; 5] = [
        Self::BlockedActionCue,
        Self::UnavailableReasonCue,
        Self::BlockingOwnerCue,
        Self::NextSafeActionCue,
        Self::DeeperDocsPathCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockedActionCue => "blocked_action_cue",
            Self::UnavailableReasonCue => "unavailable_reason_cue",
            Self::BlockingOwnerCue => "blocking_owner_cue",
            Self::NextSafeActionCue => "next_safe_action_cue",
            Self::DeeperDocsPathCue => "deeper_docs_path_cue",
            Self::FailureDomainCue => "failure_domain_cue",
            Self::ScreenReaderAnnouncementCue => "screen_reader_announcement_cue",
            Self::EvidenceExportCue => "evidence_export_cue",
        }
    }
}

/// A field the why-unavailable-row export carries so blocked-action truth is reconstructable. The
/// fields in [`M5WhyUnavailableExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WhyUnavailableExportField {
    /// The unavailable reason class.
    UnavailableReason,
    /// The blocking owner.
    BlockingOwner,
    /// The blocked-action reference.
    BlockedActionRef,
    /// The next-safe-action class.
    NextSafeAction,
    /// The deeper-docs reference.
    DeeperDocsRef,
    /// The failure domain.
    FailureDomain,
    /// The screen-reader announcement.
    ScreenReaderAnnouncement,
    /// The next-safe-action reference.
    NextSafeActionRef,
}

impl M5WhyUnavailableExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::UnavailableReason,
        Self::BlockingOwner,
        Self::BlockedActionRef,
        Self::NextSafeAction,
        Self::DeeperDocsRef,
        Self::FailureDomain,
        Self::ScreenReaderAnnouncement,
        Self::NextSafeActionRef,
    ];

    /// The export fields every row must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::UnavailableReason,
        Self::BlockingOwner,
        Self::BlockedActionRef,
        Self::NextSafeAction,
        Self::DeeperDocsRef,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnavailableReason => "unavailable_reason",
            Self::BlockingOwner => "blocking_owner",
            Self::BlockedActionRef => "blocked_action_ref",
            Self::NextSafeAction => "next_safe_action",
            Self::DeeperDocsRef => "deeper_docs_ref",
            Self::FailureDomain => "failure_domain",
            Self::ScreenReaderAnnouncement => "screen_reader_announcement",
            Self::NextSafeActionRef => "next_safe_action_ref",
        }
    }
}

/// The full input to the why-unavailable-row resolver for one blocked action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WhyUnavailableRowResolutionInput {
    /// The opaque blocked action identity (must be non-empty).
    pub blocked_action_ref: String,
    /// Why the action is unavailable.
    pub unavailable_reason: M5UnavailableReasonClass,
    /// Who owns / gates the blocked action.
    pub blocking_owner: M5BlockedActionOwner,
    /// The next safe action the user can take.
    pub next_safe_action: M5NextSafeActionClass,
    /// The opaque target for the next safe action. `None` only for a `no_safe_action` row;
    /// `Some(non-empty)` for every actionable row.
    pub next_safe_action_ref: Option<String>,
    /// The opaque deeper-docs / help path (must be non-empty) so the row always links deeper truth.
    pub deeper_docs_ref: String,
    /// The screen-reader announcement text (must be non-empty) so the row is never hover- or
    /// sight-only.
    pub screen_reader_announcement: String,
    /// The opaque stable row identity (must be non-empty).
    pub row_identity_ref: String,
}

/// The resolved why-unavailable-row truth for one blocked action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWhyUnavailableRow {
    /// The unavailable reason class.
    pub unavailable_reason: M5UnavailableReasonClass,
    /// The blocking owner.
    pub blocking_owner: M5BlockedActionOwner,
    /// The next-safe-action class.
    pub next_safe_action: M5NextSafeActionClass,
    /// The opaque blocked-action reference, preserved exactly from the input.
    pub blocked_action_ref: String,
    /// The opaque next-safe-action reference, preserved exactly from the input.
    pub next_safe_action_ref: Option<String>,
    /// The opaque deeper-docs reference, preserved exactly from the input.
    pub deeper_docs_ref: String,
    /// The screen-reader announcement, preserved exactly from the input.
    pub screen_reader_announcement: String,
    /// The opaque stable row identity, preserved exactly from the input.
    pub row_identity_ref: String,
    /// The derived why-unavailable posture.
    pub help_posture: M5WhyUnavailablePosture,
    /// The derived failure domain.
    pub failure_domain: M5UnavailableFailureDomain,
    /// The bounded actions this row offers.
    pub available_actions: Vec<M5WhyUnavailableAction>,
    /// True when the row names a concrete next safe action (not `no_safe_action`).
    pub has_next_safe_action: bool,
    /// True when the blocking owner can be contacted / escalated to.
    pub owner_is_contactable: bool,
    /// True when the block is transient and may clear on its own or with a retry.
    pub reason_is_transient: bool,
    /// True when the row offers the open-deeper-docs action. ALWAYS `true`.
    pub docs_available: bool,
    /// True when the row offers the export-unavailable-evidence action. ALWAYS `true`.
    pub evidence_export_available: bool,
    /// The row always names the blocked action. ALWAYS `true`.
    pub names_blocked_action: bool,
    /// The row always names the exact reason. ALWAYS `true`.
    pub names_exact_reason: bool,
    /// The row always names the owning boundary. ALWAYS `true`.
    pub names_owning_boundary: bool,
    /// The row always names the next safe action or honestly states there is none. ALWAYS `true`.
    pub names_next_safe_action_or_states_none: bool,
    /// The row always links the deeper docs / help path. ALWAYS `true`.
    pub links_deeper_docs: bool,
    /// The row never collapses into a generic disabled state — the reason, owner, and domain are
    /// always specific. ALWAYS `true`.
    pub never_generic_disabled: bool,
    /// The row never requires pointer hover. ALWAYS `true`.
    pub never_requires_pointer_hover: bool,
    /// The row always carries a screen-reader announcement. ALWAYS `true`.
    pub provides_screen_reader_announcement: bool,
    /// The row always preserves the docs path honestly. ALWAYS `true`.
    pub preserves_docs_path_honestly: bool,
}

/// Errors returned by [`resolve_why_unavailable_explanation_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5WhyUnavailableRowResolutionError {
    /// The blocked-action reference was empty.
    EmptyBlockedActionRef,
    /// The deeper-docs reference was empty.
    EmptyDeeperDocsRef,
    /// The screen-reader announcement was empty.
    EmptyScreenReaderAnnouncement,
    /// The row identity ref was empty.
    EmptyRowIdentity,
    /// An actionable row (next safe action is not `no_safe_action`) named no concrete next-safe-
    /// action target — it would say "do X" with nothing to do.
    MissingNextActionRefForActionableRow,
    /// A `no_safe_action` row wrongly declared a next-safe-action target.
    NextActionRefOnNoSafeAction,
    /// A row descriptor carried forbidden material.
    ForbiddenUnavailableMaterial,
}

impl M5WhyUnavailableRowResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyBlockedActionRef => "empty_blocked_action_ref",
            Self::EmptyDeeperDocsRef => "empty_deeper_docs_ref",
            Self::EmptyScreenReaderAnnouncement => "empty_screen_reader_announcement",
            Self::EmptyRowIdentity => "empty_row_identity",
            Self::MissingNextActionRefForActionableRow => {
                "missing_next_action_ref_for_actionable_row"
            }
            Self::NextActionRefOnNoSafeAction => "next_action_ref_on_no_safe_action",
            Self::ForbiddenUnavailableMaterial => "forbidden_unavailable_material",
        }
    }
}

impl fmt::Display for M5WhyUnavailableRowResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "why unavailable row resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WhyUnavailableRowResolutionError {}

/// Resolves one why-unavailable explanation row from its declared reason, owning boundary, next
/// safe action, next-action target, and deeper-docs path.
///
/// The posture is derived one-to-one from the frozen unavailable reason class, and the failure
/// domain (policy / trust / context / runtime) from the posture, so a blocked action never
/// collapses context, trust, policy, and runtime failures into one generic disabled state. The
/// action set offers take-next-safe-action whenever the row names a concrete next safe action,
/// contact-blocking-owner whenever the owner can be reached, retry-when-resolved whenever the
/// block is transient, and always offers open-deeper-docs and export-unavailable-evidence. An
/// actionable row with no concrete next-action target is rejected outright, so the row never tells
/// a user to "do X" with nothing to do.
pub fn resolve_why_unavailable_explanation_row(
    input: &M5WhyUnavailableRowResolutionInput,
) -> Result<M5ResolvedWhyUnavailableRow, M5WhyUnavailableRowResolutionError> {
    if input.blocked_action_ref.trim().is_empty() {
        return Err(M5WhyUnavailableRowResolutionError::EmptyBlockedActionRef);
    }
    if input.deeper_docs_ref.trim().is_empty() {
        return Err(M5WhyUnavailableRowResolutionError::EmptyDeeperDocsRef);
    }
    if input.screen_reader_announcement.trim().is_empty() {
        return Err(M5WhyUnavailableRowResolutionError::EmptyScreenReaderAnnouncement);
    }
    if input.row_identity_ref.trim().is_empty() {
        return Err(M5WhyUnavailableRowResolutionError::EmptyRowIdentity);
    }
    if why_unavailable_input_has_forbidden_material(input) {
        return Err(M5WhyUnavailableRowResolutionError::ForbiddenUnavailableMaterial);
    }

    let has_next_safe_action =
        !matches!(input.next_safe_action, M5NextSafeActionClass::NoSafeAction);
    let next_action_ref_present = input
        .next_safe_action_ref
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty());
    if has_next_safe_action {
        if !next_action_ref_present {
            return Err(M5WhyUnavailableRowResolutionError::MissingNextActionRefForActionableRow);
        }
    } else if next_action_ref_present {
        return Err(M5WhyUnavailableRowResolutionError::NextActionRefOnNoSafeAction);
    }

    let help_posture = M5WhyUnavailablePosture::from_reason(input.unavailable_reason);
    let failure_domain = help_posture.failure_domain();
    let owner_is_contactable = owner_is_contactable(input.blocking_owner);
    let reason_is_transient = help_posture.is_transient();

    let available_actions = derive_why_unavailable_actions(
        has_next_safe_action,
        owner_is_contactable,
        reason_is_transient,
    );

    Ok(M5ResolvedWhyUnavailableRow {
        unavailable_reason: input.unavailable_reason,
        blocking_owner: input.blocking_owner,
        next_safe_action: input.next_safe_action,
        blocked_action_ref: input.blocked_action_ref.clone(),
        next_safe_action_ref: input.next_safe_action_ref.clone(),
        deeper_docs_ref: input.deeper_docs_ref.clone(),
        screen_reader_announcement: input.screen_reader_announcement.clone(),
        row_identity_ref: input.row_identity_ref.clone(),
        help_posture,
        failure_domain,
        available_actions,
        has_next_safe_action,
        owner_is_contactable,
        reason_is_transient,
        docs_available: true,
        evidence_export_available: true,
        // The acceptance criteria: a why-unavailable row always names the blocked action, the
        // exact reason, the owning boundary, the next safe action (or that there is none), and
        // links the deeper docs — never a generic disabled state, never hover-only, always with a
        // screen-reader announcement, and preserving the docs path honestly.
        names_blocked_action: true,
        names_exact_reason: true,
        names_owning_boundary: true,
        names_next_safe_action_or_states_none: true,
        links_deeper_docs: true,
        never_generic_disabled: true,
        never_requires_pointer_hover: true,
        provides_screen_reader_announcement: true,
        preserves_docs_path_honestly: true,
    })
}

/// Derives the bounded action set from whether the row names a concrete next safe action, whether
/// the owner can be contacted, and whether the block is transient.
///
/// Every row offers open-deeper-docs and export-unavailable-evidence so a user can always reach
/// deeper truth and export the evidence. A row with a concrete next safe action additionally
/// offers take-next-safe-action; a contactable owner offers contact-blocking-owner; and a
/// transient block offers retry-when-resolved.
fn derive_why_unavailable_actions(
    has_next_safe_action: bool,
    owner_is_contactable: bool,
    reason_is_transient: bool,
) -> Vec<M5WhyUnavailableAction> {
    use M5WhyUnavailableAction as Action;

    let mut actions = Vec::new();
    if has_next_safe_action {
        actions.push(Action::TakeNextSafeAction);
    }
    if owner_is_contactable {
        actions.push(Action::ContactBlockingOwner);
    }
    if reason_is_transient {
        actions.push(Action::RetryWhenResolved);
    }
    actions.push(Action::OpenDeeperDocs);
    actions.push(Action::ExportUnavailableEvidence);
    actions
}

/// True when any opaque descriptor on the why-unavailable input carries obviously forbidden
/// material.
fn why_unavailable_input_has_forbidden_material(
    input: &M5WhyUnavailableRowResolutionInput,
) -> bool {
    if value_repr_is_forbidden(&input.blocked_action_ref)
        || value_repr_is_forbidden(&input.deeper_docs_ref)
        || value_repr_is_forbidden(&input.screen_reader_announcement)
        || value_repr_is_forbidden(&input.row_identity_ref)
    {
        return true;
    }
    if let Some(reference) = &input.next_safe_action_ref {
        if value_repr_is_forbidden(reference) {
            return true;
        }
    }
    false
}

// =========================================================================
// Family 2 — source-language fallback surface
// =========================================================================

/// The derived localization posture — the resolver's honest verdict about a localized surface's
/// state. Derived one-to-one from the frozen fallback-state class so a fallback, partial, stale,
/// or missing localization is always named for exactly what it is and never masqueraded as
/// authoritative localized guidance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceLanguagePosture {
    /// Fully localized and current.
    FullyLocalized,
    /// Showing the source language as a fallback.
    ShowingSourceLanguage,
    /// Partially localized.
    PartiallyLocalized,
    /// Localized but stale relative to the source.
    StaleLocalization,
    /// A fallback with its canonical citation preserved.
    CitationPreservedFallback,
    /// No localization available.
    NoLocalization,
}

impl M5SourceLanguagePosture {
    /// Every localization posture, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FullyLocalized,
        Self::ShowingSourceLanguage,
        Self::PartiallyLocalized,
        Self::StaleLocalization,
        Self::CitationPreservedFallback,
        Self::NoLocalization,
    ];

    /// The posture that honestly reflects a fallback-state class — one-to-one, never upgrading a
    /// partial, stale, or missing localization into fully localized guidance.
    pub const fn from_fallback_state(state: M5FallbackStateClass) -> Self {
        match state {
            M5FallbackStateClass::LocalizedCurrent => Self::FullyLocalized,
            M5FallbackStateClass::SourceLanguageShown => Self::ShowingSourceLanguage,
            M5FallbackStateClass::PartialTranslation => Self::PartiallyLocalized,
            M5FallbackStateClass::StaleTranslation => Self::StaleLocalization,
            M5FallbackStateClass::CitationPreservedFallback => Self::CitationPreservedFallback,
            M5FallbackStateClass::NoLocalization => Self::NoLocalization,
        }
    }

    /// True when the surface is fully localized and current.
    pub const fn is_fully_localized(self) -> bool {
        matches!(self, Self::FullyLocalized)
    }

    /// True when the surface must preserve the source-language text — anything that is not fully
    /// localized is incomplete or behind the source and must keep the source text reachable.
    pub const fn requires_source_text(self) -> bool {
        !self.is_fully_localized()
    }

    /// True when the surface prominently shows the source-language text as its content.
    pub const fn shows_source_language(self) -> bool {
        matches!(
            self,
            Self::ShowingSourceLanguage | Self::CitationPreservedFallback | Self::NoLocalization
        )
    }

    /// True when the surface has a translation gap worth reporting — partial, stale, or missing.
    pub const fn is_incomplete_or_stale(self) -> bool {
        matches!(
            self,
            Self::PartiallyLocalized | Self::StaleLocalization | Self::NoLocalization
        )
    }

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyLocalized => "fully_localized",
            Self::ShowingSourceLanguage => "showing_source_language",
            Self::PartiallyLocalized => "partially_localized",
            Self::StaleLocalization => "stale_localization",
            Self::CitationPreservedFallback => "citation_preserved_fallback",
            Self::NoLocalization => "no_localization",
        }
    }
}

/// One bounded action a source-language fallback surface offers, so a user reading localized help
/// that is behind the source can always view the source-language text, report a translation gap,
/// request localization, open the canonical citation, or export the locale evidence — never left
/// with an unsourced paraphrase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceLanguageAction {
    /// View the preserved source-language text.
    ViewSourceLanguageText,
    /// Report a translation gap.
    ReportTranslationGap,
    /// Request localization for this surface.
    RequestLocalization,
    /// Open the canonical citation back to the source docs / help.
    OpenCanonicalCitation,
    /// Export the locale evidence.
    ExportLocaleEvidence,
}

impl M5SourceLanguageAction {
    /// Every source-language action, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ViewSourceLanguageText,
        Self::ReportTranslationGap,
        Self::RequestLocalization,
        Self::OpenCanonicalCitation,
        Self::ExportLocaleEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ViewSourceLanguageText => "view_source_language_text",
            Self::ReportTranslationGap => "report_translation_gap",
            Self::RequestLocalization => "request_localization",
            Self::OpenCanonicalCitation => "open_canonical_citation",
            Self::ExportLocaleEvidence => "export_locale_evidence",
        }
    }
}

/// Controlled source-language-fallback anatomy part the shared surface surfaces. The parts in
/// [`M5SourceLanguageAnatomyPart::MANDATORY`] are required on every surface so the source-language
/// text, the stable ID, the canonical citation link, and the localization / fallback state are
/// never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceLanguageAnatomyPart {
    /// The source-language-text cue.
    SourceLanguageTextCue,
    /// The stable-ID cue.
    StableIdCue,
    /// The canonical-citation-link cue.
    CanonicalCitationLinkCue,
    /// The localization-state cue.
    LocalizationStateCue,
    /// The fallback-state cue.
    FallbackStateCue,
    /// The source-language-class cue.
    SourceLanguageClassCue,
    /// The screen-reader-announcement cue.
    ScreenReaderAnnouncementCue,
    /// The locale-export cue.
    LocaleExportCue,
}

impl M5SourceLanguageAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SourceLanguageTextCue,
        Self::StableIdCue,
        Self::CanonicalCitationLinkCue,
        Self::LocalizationStateCue,
        Self::FallbackStateCue,
        Self::SourceLanguageClassCue,
        Self::ScreenReaderAnnouncementCue,
        Self::LocaleExportCue,
    ];

    /// The anatomy parts every surface must render.
    pub const MANDATORY: [Self; 5] = [
        Self::SourceLanguageTextCue,
        Self::StableIdCue,
        Self::CanonicalCitationLinkCue,
        Self::LocalizationStateCue,
        Self::FallbackStateCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLanguageTextCue => "source_language_text_cue",
            Self::StableIdCue => "stable_id_cue",
            Self::CanonicalCitationLinkCue => "canonical_citation_link_cue",
            Self::LocalizationStateCue => "localization_state_cue",
            Self::FallbackStateCue => "fallback_state_cue",
            Self::SourceLanguageClassCue => "source_language_class_cue",
            Self::ScreenReaderAnnouncementCue => "screen_reader_announcement_cue",
            Self::LocaleExportCue => "locale_export_cue",
        }
    }
}

/// A field the source-language-fallback export carries so localized-help truth is reconstructable.
/// The fields in [`M5SourceLanguageExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceLanguageExportField {
    /// The source-language class.
    SourceLanguageClass,
    /// The fallback-state class.
    FallbackState,
    /// The stable-ID reference.
    StableIdRef,
    /// The canonical-citation reference.
    CanonicalCitationRef,
    /// The display locale.
    DisplayLocale,
    /// The source-language-text reference.
    SourceLanguageTextRef,
    /// The screen-reader announcement.
    ScreenReaderAnnouncement,
    /// The locale-export reference.
    LocaleExportRef,
}

impl M5SourceLanguageExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::SourceLanguageClass,
        Self::FallbackState,
        Self::StableIdRef,
        Self::CanonicalCitationRef,
        Self::DisplayLocale,
        Self::SourceLanguageTextRef,
        Self::ScreenReaderAnnouncement,
        Self::LocaleExportRef,
    ];

    /// The export fields every surface must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::SourceLanguageClass,
        Self::FallbackState,
        Self::StableIdRef,
        Self::CanonicalCitationRef,
        Self::DisplayLocale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceLanguageClass => "source_language_class",
            Self::FallbackState => "fallback_state",
            Self::StableIdRef => "stable_id_ref",
            Self::CanonicalCitationRef => "canonical_citation_ref",
            Self::DisplayLocale => "display_locale",
            Self::SourceLanguageTextRef => "source_language_text_ref",
            Self::ScreenReaderAnnouncement => "screen_reader_announcement",
            Self::LocaleExportRef => "locale_export_ref",
        }
    }
}

/// The full input to the source-language-fallback resolver for one localized surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceLanguageFallbackResolutionInput {
    /// The localization origin of the rendered text.
    pub source_language_class: M5SourceLanguageClass,
    /// How the surface preserves canonical IDs / citations while falling back.
    pub fallback_state: M5FallbackStateClass,
    /// The opaque display locale (must be non-empty).
    pub display_locale: String,
    /// The opaque stable canonical ID that must be preserved (must be non-empty).
    pub stable_id_ref: String,
    /// The opaque citation link back to the canonical docs / help (must be non-empty).
    pub canonical_citation_ref: String,
    /// The opaque preserved source-language text. `None` is allowed only for a fully localized
    /// surface; every fallback (incomplete or behind the source) must carry `Some(non-empty)`.
    pub source_language_text_ref: Option<String>,
    /// The screen-reader announcement text (must be non-empty).
    pub screen_reader_announcement: String,
    /// The opaque stable row identity (must be non-empty).
    pub row_identity_ref: String,
}

/// The resolved source-language-fallback truth for one localized surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSourceLanguageFallback {
    /// The source-language class.
    pub source_language_class: M5SourceLanguageClass,
    /// The fallback-state class.
    pub fallback_state: M5FallbackStateClass,
    /// The opaque display locale, preserved exactly from the input.
    pub display_locale: String,
    /// The opaque stable-ID reference, preserved exactly from the input.
    pub stable_id_ref: String,
    /// The opaque canonical-citation reference, preserved exactly from the input.
    pub canonical_citation_ref: String,
    /// The opaque source-language-text reference, preserved exactly from the input.
    pub source_language_text_ref: Option<String>,
    /// The screen-reader announcement, preserved exactly from the input.
    pub screen_reader_announcement: String,
    /// The opaque stable row identity, preserved exactly from the input.
    pub row_identity_ref: String,
    /// The derived localization posture.
    pub help_posture: M5SourceLanguagePosture,
    /// The bounded actions this surface offers.
    pub available_actions: Vec<M5SourceLanguageAction>,
    /// True when the surface prominently shows the source-language text.
    pub shows_source_language: bool,
    /// True when the surface has a translation gap worth reporting.
    pub is_incomplete_or_stale: bool,
    /// True when the surface is fully localized and current.
    pub is_fully_localized: bool,
    /// True when the surface must preserve the source-language text.
    pub requires_source_text: bool,
    /// True when the surface offers the open-canonical-citation action. ALWAYS `true`.
    pub citation_available: bool,
    /// True when the surface offers the export-locale-evidence action. ALWAYS `true`.
    pub locale_export_available: bool,
    /// The surface always preserves the source-language text. ALWAYS `true`.
    pub preserves_source_language_text: bool,
    /// The surface always preserves the stable canonical ID. ALWAYS `true`.
    pub preserves_stable_id: bool,
    /// The surface always preserves the canonical citation. ALWAYS `true`.
    pub preserves_canonical_citation: bool,
    /// The surface always names its localization state. ALWAYS `true`.
    pub names_localization_state: bool,
    /// The surface never drifts into an unsourced paraphrase — the canonical citation is always
    /// reachable. ALWAYS `true`.
    pub never_unsourced_paraphrase: bool,
    /// The surface never requires pointer hover. ALWAYS `true`.
    pub never_requires_pointer_hover: bool,
    /// The surface always carries a screen-reader announcement. ALWAYS `true`.
    pub provides_screen_reader_announcement: bool,
    /// The surface stays aligned with the canonical command / doc IDs. ALWAYS `true`.
    pub aligned_with_canonical_ids: bool,
}

/// Errors returned by [`resolve_source_language_fallback`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SourceLanguageFallbackResolutionError {
    /// The display locale was empty.
    EmptyDisplayLocale,
    /// The stable-ID reference was empty.
    EmptyStableIdRef,
    /// The canonical-citation reference was empty.
    EmptyCanonicalCitationRef,
    /// The screen-reader announcement was empty.
    EmptyScreenReaderAnnouncement,
    /// The row identity ref was empty.
    EmptyRowIdentity,
    /// A fallback surface (incomplete or behind the source) carried no preserved source-language
    /// text — it would drift into an unsourced paraphrase.
    MissingSourceTextForFallback,
    /// A row descriptor carried forbidden material.
    ForbiddenLocaleMaterial,
}

impl M5SourceLanguageFallbackResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyDisplayLocale => "empty_display_locale",
            Self::EmptyStableIdRef => "empty_stable_id_ref",
            Self::EmptyCanonicalCitationRef => "empty_canonical_citation_ref",
            Self::EmptyScreenReaderAnnouncement => "empty_screen_reader_announcement",
            Self::EmptyRowIdentity => "empty_row_identity",
            Self::MissingSourceTextForFallback => "missing_source_text_for_fallback",
            Self::ForbiddenLocaleMaterial => "forbidden_locale_material",
        }
    }
}

impl fmt::Display for M5SourceLanguageFallbackResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "source language fallback resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SourceLanguageFallbackResolutionError {}

/// Resolves one source-language fallback surface from its declared source-language class,
/// fallback-state class, display locale, stable ID, canonical citation, and preserved
/// source-language text.
///
/// The posture is derived one-to-one from the frozen fallback-state class so a partial, stale, or
/// missing localization is never upgraded into fully localized guidance. The action set offers
/// view-source-language-text whenever the source is shown, report-translation-gap whenever there
/// is a gap, request-localization when there is no localization, and always offers
/// open-canonical-citation and export-locale-evidence so a user can always reach the canonical
/// source. A fallback surface that carries no preserved source-language text is rejected outright,
/// so localized guidance never drifts into an unsourced paraphrase.
pub fn resolve_source_language_fallback(
    input: &M5SourceLanguageFallbackResolutionInput,
) -> Result<M5ResolvedSourceLanguageFallback, M5SourceLanguageFallbackResolutionError> {
    if input.display_locale.trim().is_empty() {
        return Err(M5SourceLanguageFallbackResolutionError::EmptyDisplayLocale);
    }
    if input.stable_id_ref.trim().is_empty() {
        return Err(M5SourceLanguageFallbackResolutionError::EmptyStableIdRef);
    }
    if input.canonical_citation_ref.trim().is_empty() {
        return Err(M5SourceLanguageFallbackResolutionError::EmptyCanonicalCitationRef);
    }
    if input.screen_reader_announcement.trim().is_empty() {
        return Err(M5SourceLanguageFallbackResolutionError::EmptyScreenReaderAnnouncement);
    }
    if input.row_identity_ref.trim().is_empty() {
        return Err(M5SourceLanguageFallbackResolutionError::EmptyRowIdentity);
    }
    if source_language_input_has_forbidden_material(input) {
        return Err(M5SourceLanguageFallbackResolutionError::ForbiddenLocaleMaterial);
    }

    let help_posture = M5SourceLanguagePosture::from_fallback_state(input.fallback_state);
    let requires_source_text = help_posture.requires_source_text();
    let has_source_text = input
        .source_language_text_ref
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty());
    if requires_source_text && !has_source_text {
        return Err(M5SourceLanguageFallbackResolutionError::MissingSourceTextForFallback);
    }

    let available_actions = derive_source_language_actions(help_posture);

    Ok(M5ResolvedSourceLanguageFallback {
        source_language_class: input.source_language_class,
        fallback_state: input.fallback_state,
        display_locale: input.display_locale.clone(),
        stable_id_ref: input.stable_id_ref.clone(),
        canonical_citation_ref: input.canonical_citation_ref.clone(),
        source_language_text_ref: input.source_language_text_ref.clone(),
        screen_reader_announcement: input.screen_reader_announcement.clone(),
        row_identity_ref: input.row_identity_ref.clone(),
        help_posture,
        available_actions,
        shows_source_language: help_posture.shows_source_language(),
        is_incomplete_or_stale: help_posture.is_incomplete_or_stale(),
        is_fully_localized: help_posture.is_fully_localized(),
        requires_source_text,
        citation_available: true,
        locale_export_available: true,
        // The acceptance criteria: a source-language fallback surface always preserves the
        // source-language text, the stable ID, and the canonical citation; always names its
        // localization state; never drifts into an unsourced paraphrase; is never hover-only;
        // always carries a screen-reader announcement; and stays aligned with canonical IDs.
        preserves_source_language_text: true,
        preserves_stable_id: true,
        preserves_canonical_citation: true,
        names_localization_state: true,
        never_unsourced_paraphrase: true,
        never_requires_pointer_hover: true,
        provides_screen_reader_announcement: true,
        aligned_with_canonical_ids: true,
    })
}

/// Derives the bounded action set from the localization posture.
///
/// Every surface offers open-canonical-citation and export-locale-evidence so a user can always
/// reach the canonical source. A surface showing the source language additionally offers
/// view-source-language-text; a surface with a translation gap offers report-translation-gap; and
/// a surface with no localization offers request-localization.
fn derive_source_language_actions(posture: M5SourceLanguagePosture) -> Vec<M5SourceLanguageAction> {
    use M5SourceLanguageAction as Action;

    let mut actions = Vec::new();
    if posture.shows_source_language() {
        actions.push(Action::ViewSourceLanguageText);
    }
    if posture.is_incomplete_or_stale() {
        actions.push(Action::ReportTranslationGap);
    }
    if matches!(posture, M5SourceLanguagePosture::NoLocalization) {
        actions.push(Action::RequestLocalization);
    }
    actions.push(Action::OpenCanonicalCitation);
    actions.push(Action::ExportLocaleEvidence);
    actions
}

/// True when any opaque descriptor on the source-language input carries obviously forbidden
/// material.
fn source_language_input_has_forbidden_material(
    input: &M5SourceLanguageFallbackResolutionInput,
) -> bool {
    if value_repr_is_forbidden(&input.display_locale)
        || value_repr_is_forbidden(&input.stable_id_ref)
        || value_repr_is_forbidden(&input.canonical_citation_ref)
        || value_repr_is_forbidden(&input.screen_reader_announcement)
        || value_repr_is_forbidden(&input.row_identity_ref)
    {
        return true;
    }
    if let Some(text) = &input.source_language_text_ref {
        if value_repr_is_forbidden(text) {
            return true;
        }
    }
    false
}

// =========================================================================
// Worked cases
// =========================================================================

/// One worked why-unavailable-row resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WhyUnavailableRowResolutionCase {
    /// The resolver input.
    pub input: M5WhyUnavailableRowResolutionInput,
    /// The resolved truth. Must equal `resolve_why_unavailable_explanation_row(&input)`.
    pub resolved: M5ResolvedWhyUnavailableRow,
}

impl M5WhyUnavailableRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5WhyUnavailableRowResolutionInput) -> Self {
        let resolved = resolve_why_unavailable_explanation_row(&input)
            .expect("seed why unavailable row case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_why_unavailable_explanation_row(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input identity, blocked-action reference,
    /// next-safe-action reference, deeper-docs reference, and screen-reader announcement exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.row_identity_ref == self.input.row_identity_ref
            && self.resolved.blocked_action_ref == self.input.blocked_action_ref
            && self.resolved.next_safe_action_ref == self.input.next_safe_action_ref
            && self.resolved.deeper_docs_ref == self.input.deeper_docs_ref
            && self.resolved.screen_reader_announcement == self.input.screen_reader_announcement
    }

    /// True when the resolved case names the blocked action, the exact reason, the owning
    /// boundary, the next safe action (or states none), links the deeper docs, never collapses to
    /// a generic disabled state, never requires pointer hover, and carries a screen-reader
    /// announcement.
    pub fn preserves_explanation_parity(&self) -> bool {
        self.resolved.names_blocked_action
            && self.resolved.names_exact_reason
            && self.resolved.names_owning_boundary
            && self.resolved.names_next_safe_action_or_states_none
            && self.resolved.links_deeper_docs
            && self.resolved.never_generic_disabled
            && self.resolved.never_requires_pointer_hover
            && self.resolved.provides_screen_reader_announcement
            && self.resolved.preserves_docs_path_honestly
            // The concrete guarantee: deeper docs and evidence export are always reachable.
            && self.resolved.docs_available
            && self.resolved.evidence_export_available
            && self
                .resolved
                .available_actions
                .contains(&M5WhyUnavailableAction::OpenDeeperDocs)
            && self
                .resolved
                .available_actions
                .contains(&M5WhyUnavailableAction::ExportUnavailableEvidence)
    }
}

/// One worked source-language-fallback resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SourceLanguageFallbackResolutionCase {
    /// The resolver input.
    pub input: M5SourceLanguageFallbackResolutionInput,
    /// The resolved truth. Must equal `resolve_source_language_fallback(&input)`.
    pub resolved: M5ResolvedSourceLanguageFallback,
}

impl M5SourceLanguageFallbackResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SourceLanguageFallbackResolutionInput) -> Self {
        let resolved = resolve_source_language_fallback(&input)
            .expect("seed source language fallback case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_source_language_fallback(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input identity, stable ID, canonical citation,
    /// source-language text, and screen-reader announcement exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.row_identity_ref == self.input.row_identity_ref
            && self.resolved.stable_id_ref == self.input.stable_id_ref
            && self.resolved.canonical_citation_ref == self.input.canonical_citation_ref
            && self.resolved.source_language_text_ref == self.input.source_language_text_ref
            && self.resolved.display_locale == self.input.display_locale
            && self.resolved.screen_reader_announcement == self.input.screen_reader_announcement
    }

    /// True when the resolved case preserves the stable ID and the canonical citation and keeps
    /// the open-canonical-citation action reachable — the acceptance criterion that localized help
    /// stays aligned with canonical IDs and cited source material.
    pub fn preserves_citation(&self) -> bool {
        self.resolved.preserves_stable_id
            && self.resolved.preserves_canonical_citation
            && self.resolved.aligned_with_canonical_ids
            && self.resolved.never_unsourced_paraphrase
            && !self.resolved.stable_id_ref.trim().is_empty()
            && !self.resolved.canonical_citation_ref.trim().is_empty()
            && self.resolved.citation_available
            && self
                .resolved
                .available_actions
                .contains(&M5SourceLanguageAction::OpenCanonicalCitation)
    }

    /// True when the resolved case preserves the source-language text, names its localization
    /// state, never requires pointer hover, and carries a screen-reader announcement.
    pub fn preserves_localized_parity(&self) -> bool {
        self.resolved.preserves_source_language_text
            && self.resolved.names_localization_state
            && self.resolved.never_requires_pointer_hover
            && self.resolved.provides_screen_reader_announcement
            && self.resolved.locale_export_available
            // The concrete guarantee: a fallback surface always carries the preserved source text.
            && (!self.resolved.requires_source_text
                || self
                    .resolved
                    .source_language_text_ref
                    .as_ref()
                    .is_some_and(|value| !value.trim().is_empty()))
    }
}

// =========================================================================
// Combined parity matrix
// =========================================================================

/// One row in the primitive matrix: one blocked-action / localized consumer bound to both shared
/// anatomies, the frozen vocabularies, the derived postures, the bounded actions, the export
/// fields, and the accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BlockedLocalizedConsumerRow {
    /// Blocked-action / localized consumer family.
    pub consumer_surface: M5BlockedLocalizedConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5TeachingQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this row.
    pub surface_families: Vec<M5TeachingSurfaceFamily>,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5TeachingDeploymentLine>,
    /// Why-unavailable-row anatomy parts this row renders (must include the mandatory parts).
    pub why_unavailable_anatomy_parts: Vec<M5WhyUnavailableAnatomyPart>,
    /// Source-language-fallback anatomy parts this row renders (must include the mandatory parts).
    pub source_language_anatomy_parts: Vec<M5SourceLanguageAnatomyPart>,
    /// Blocked-action owners this consumer distinguishes.
    pub blocked_action_owners: Vec<M5BlockedActionOwner>,
    /// Unavailable reason classes this consumer distinguishes.
    pub unavailable_reason_classes: Vec<M5UnavailableReasonClass>,
    /// Next-safe-action classes this consumer distinguishes.
    pub next_safe_action_classes: Vec<M5NextSafeActionClass>,
    /// Failure domains this consumer distinguishes.
    pub failure_domains: Vec<M5UnavailableFailureDomain>,
    /// Why-unavailable postures this consumer distinguishes.
    pub why_unavailable_postures: Vec<M5WhyUnavailablePosture>,
    /// Bounded why-unavailable actions this consumer offers.
    pub why_unavailable_actions: Vec<M5WhyUnavailableAction>,
    /// Source-language classes this consumer distinguishes.
    pub source_language_classes: Vec<M5SourceLanguageClass>,
    /// Fallback-state classes this consumer distinguishes.
    pub fallback_state_classes: Vec<M5FallbackStateClass>,
    /// Source-language postures this consumer distinguishes.
    pub source_language_postures: Vec<M5SourceLanguagePosture>,
    /// Bounded source-language actions this consumer offers.
    pub source_language_actions: Vec<M5SourceLanguageAction>,
    /// Why-unavailable export fields this row carries (must include the mandatory fields).
    pub why_unavailable_export_fields: Vec<M5WhyUnavailableExportField>,
    /// Source-language export fields this row carries (must include the mandatory fields).
    pub source_language_export_fields: Vec<M5SourceLanguageExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5TeachingAccessibilityRoute>,
    /// Teaching subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5TeachingConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5TeachingDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked why-unavailable resolutions proving the resolver on this consumer.
    pub why_unavailable_examples: Vec<M5WhyUnavailableRowResolutionCase>,
    /// Worked source-language resolutions proving the resolver on this consumer.
    pub source_language_examples: Vec<M5SourceLanguageFallbackResolutionCase>,
    /// Hard invariant: this consumer never collapses distinct blocks into one generic disabled
    /// state. MUST be `false`.
    pub collapses_into_generic_disabled_state: bool,
    /// Hard invariant: this consumer never hides its blocking owner or reason. MUST be `false`.
    pub hides_blocking_owner_or_reason: bool,
    /// Hard invariant: this consumer never severs the canonical citation or stable ID. MUST be
    /// `false`.
    pub severs_canonical_citation_or_id: bool,
    /// Hard invariant: this consumer never drifts into an unsourced paraphrase. MUST be `false`.
    pub drifts_into_unsourced_paraphrase: bool,
}

impl M5BlockedLocalizedConsumerRow {
    /// True when the row declares every mandatory why-unavailable anatomy part.
    fn declares_mandatory_why_unavailable_anatomy(&self) -> bool {
        let present: BTreeSet<M5WhyUnavailableAnatomyPart> =
            self.why_unavailable_anatomy_parts.iter().copied().collect();
        M5WhyUnavailableAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory source-language anatomy part.
    fn declares_mandatory_source_language_anatomy(&self) -> bool {
        let present: BTreeSet<M5SourceLanguageAnatomyPart> =
            self.source_language_anatomy_parts.iter().copied().collect();
        M5SourceLanguageAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory why-unavailable export field.
    fn declares_mandatory_why_unavailable_export(&self) -> bool {
        let present: BTreeSet<M5WhyUnavailableExportField> =
            self.why_unavailable_export_fields.iter().copied().collect();
        M5WhyUnavailableExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory source-language export field.
    fn declares_mandatory_source_language_export(&self) -> bool {
        let present: BTreeSet<M5SourceLanguageExportField> =
            self.source_language_export_fields.iter().copied().collect();
        M5SourceLanguageExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_into_generic_disabled_state
            && !self.hides_blocking_owner_or_reason
            && !self.severs_canonical_citation_or_id
            && !self.drifts_into_unsourced_paraphrase
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BlockedLocalizedVocabularySet {
    /// Blocked-action / localized consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Why-unavailable-posture tokens.
    pub why_unavailable_postures: Vec<String>,
    /// Why-unavailable-action tokens.
    pub why_unavailable_actions: Vec<String>,
    /// Failure-domain tokens.
    pub failure_domains: Vec<String>,
    /// Why-unavailable-anatomy-part tokens.
    pub why_unavailable_anatomy_parts: Vec<String>,
    /// Why-unavailable-export-field tokens.
    pub why_unavailable_export_fields: Vec<String>,
    /// Source-language-posture tokens.
    pub source_language_postures: Vec<String>,
    /// Source-language-action tokens.
    pub source_language_actions: Vec<String>,
    /// Source-language-anatomy-part tokens.
    pub source_language_anatomy_parts: Vec<String>,
    /// Source-language-export-field tokens.
    pub source_language_export_fields: Vec<String>,
    /// Blocked-action-owner tokens (reused from the frozen matrix).
    pub blocked_action_owners: Vec<String>,
    /// Unavailable-reason-class tokens (reused from the frozen matrix).
    pub unavailable_reason_classes: Vec<String>,
    /// Next-safe-action-class tokens (reused from the frozen matrix).
    pub next_safe_action_classes: Vec<String>,
    /// Source-language-class tokens (reused from the frozen matrix).
    pub source_language_classes: Vec<String>,
    /// Fallback-state-class tokens (reused from the frozen matrix).
    pub fallback_state_classes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Teaching-consumer-surface tokens (reused from the frozen matrix).
    pub teaching_consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5BlockedLocalizedVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5BlockedLocalizedConsumerSurface::ALL, |v| v.as_str()),
            why_unavailable_postures: tokens(&M5WhyUnavailablePosture::ALL, |v| v.as_str()),
            why_unavailable_actions: tokens(&M5WhyUnavailableAction::ALL, |v| v.as_str()),
            failure_domains: tokens(&M5UnavailableFailureDomain::ALL, |v| v.as_str()),
            why_unavailable_anatomy_parts: tokens(&M5WhyUnavailableAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            why_unavailable_export_fields: tokens(&M5WhyUnavailableExportField::ALL, |v| {
                v.as_str()
            }),
            source_language_postures: tokens(&M5SourceLanguagePosture::ALL, |v| v.as_str()),
            source_language_actions: tokens(&M5SourceLanguageAction::ALL, |v| v.as_str()),
            source_language_anatomy_parts: tokens(&M5SourceLanguageAnatomyPart::ALL, |v| {
                v.as_str()
            }),
            source_language_export_fields: tokens(&M5SourceLanguageExportField::ALL, |v| {
                v.as_str()
            }),
            blocked_action_owners: tokens(&M5BlockedActionOwner::ALL, |v| v.as_str()),
            unavailable_reason_classes: tokens(&M5UnavailableReasonClass::ALL, |v| v.as_str()),
            next_safe_action_classes: tokens(&M5NextSafeActionClass::ALL, |v| v.as_str()),
            source_language_classes: tokens(&M5SourceLanguageClass::ALL, |v| v.as_str()),
            fallback_state_classes: tokens(&M5FallbackStateClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TeachingSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TeachingDeploymentLine::ALL, |v| v.as_str()),
            teaching_consumer_surfaces: tokens(&M5TeachingConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5BlockedLocalizedGovernanceReview {
    /// Why-unavailable rows name the blocked action.
    pub row_names_blocked_action: bool,
    /// Why-unavailable rows name the exact reason.
    pub row_names_exact_reason: bool,
    /// Why-unavailable rows name the owning boundary.
    pub row_names_owning_boundary: bool,
    /// Why-unavailable rows name the next safe action.
    pub row_names_next_safe_action: bool,
    /// Why-unavailable rows link the deeper docs / help path.
    pub row_links_deeper_docs: bool,
    /// Blocked actions never collapse context, trust, policy, and runtime failures into one
    /// generic disabled state.
    pub blocked_actions_never_collapse_into_generic_disabled: bool,
    /// Source-language fallback surfaces preserve the source-language text.
    pub fallback_preserves_source_language_text: bool,
    /// Source-language fallback surfaces preserve the stable canonical ID.
    pub fallback_preserves_stable_id: bool,
    /// Source-language fallback surfaces preserve the canonical citation.
    pub fallback_preserves_canonical_citation: bool,
    /// Localized flows never drift into an unsourced paraphrase.
    pub localized_flows_never_drift_into_unsourced_paraphrase: bool,
    /// Neither surface requires pointer hover.
    pub surfaces_never_require_pointer_hover: bool,
    /// Both surfaces provide a screen-reader announcement.
    pub surfaces_provide_screen_reader_announcement: bool,
    /// Rows keep the same truth across every deployment line.
    pub rows_stable_across_deployment_lines: bool,
    /// Rows keep the same truth across desktop, headless/export, and support consumers.
    pub rows_stable_across_consumer_surfaces: bool,
    /// The support / export packet reconstructs both surfaces' truth.
    pub support_export_reconstructs_truth: bool,
    /// Later M5 rows cannot invent parallel blocked-action or localized vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BlockedLocalizedConsumerProjection {
    /// Blocked-action and localized surfaces consume the shared vocabulary.
    pub surfaces_consume_shared_vocabulary: bool,
    /// The why-unavailable posture resolver reads a single canonical source.
    pub why_unavailable_reads_single_source: bool,
    /// The source-language posture resolver reads a single canonical source.
    pub source_language_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop rows read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BlockedLocalizedProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the blocked-action / localized primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BlockedLocalizedReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting blocked-action / localized audit.
    pub blocked_localized_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BlockedLocalizedRowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BlockedLocalizedRowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Blocked-action / localized consumer rows.
    pub rows: Vec<M5BlockedLocalizedConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BlockedLocalizedVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BlockedLocalizedGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BlockedLocalizedConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BlockedLocalizedProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BlockedLocalizedReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 why-unavailable / source-language primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BlockedLocalizedRowPacket {
    /// Record kind; must equal [`M5_BLOCKED_LOCALIZED_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BLOCKED_LOCALIZED_ROW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Blocked-action / localized consumer rows.
    pub rows: Vec<M5BlockedLocalizedConsumerRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BlockedLocalizedVocabularySet,
    /// Governance-review block.
    pub governance_review: M5BlockedLocalizedGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BlockedLocalizedConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BlockedLocalizedProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BlockedLocalizedReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BlockedLocalizedRowPacket {
    /// Builds an M5 why-unavailable / source-language-primitive packet from stable-lane input.
    pub fn new(input: M5BlockedLocalizedRowPacketInput) -> Self {
        Self {
            record_kind: M5_BLOCKED_LOCALIZED_ROW_RECORD_KIND.to_owned(),
            schema_version: M5_BLOCKED_LOCALIZED_ROW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 why-unavailable / source-language-primitive invariants.
    pub fn validate(&self) -> Vec<M5BlockedLocalizedRowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BLOCKED_LOCALIZED_ROW_RECORD_KIND {
            violations.push(M5BlockedLocalizedRowViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BLOCKED_LOCALIZED_ROW_SCHEMA_VERSION {
            violations.push(M5BlockedLocalizedRowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BlockedLocalizedRowViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_reason_coverage(self, &mut violations);
        validate_failure_domain_coverage(self, &mut violations);
        validate_next_safe_action_coverage(self, &mut violations);
        validate_why_unavailable_action_coverage(self, &mut violations);
        validate_why_unavailable_posture_coverage(self, &mut violations);
        validate_source_language_class_coverage(self, &mut violations);
        validate_fallback_state_coverage(self, &mut violations);
        validate_source_language_action_coverage(self, &mut violations);
        validate_citation_preservation(self, &mut violations);
        validate_keyboard_parity_coverage(self, &mut violations);
        validate_reversibility(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 blocked localized primitive packet serializes"),
        ) {
            violations.push(M5BlockedLocalizedRowViolation::RawMaterialInExport);
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
            .expect("m5 blocked localized primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per blocked-action / localized consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,unavailable_reasons,why_unavailable_postures,failure_domains,source_language_classes,fallback_states,why_unavailable_examples,source_language_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.unavailable_reason_classes, |v| v.as_str()),
                join_tokens(&row.why_unavailable_postures, |v| v.as_str()),
                join_tokens(&row.failure_domains, |v| v.as_str()),
                join_tokens(&row.source_language_classes, |v| v.as_str()),
                join_tokens(&row.fallback_state_classes, |v| v.as_str()),
                row.why_unavailable_examples.len(),
                row.source_language_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Why-Unavailable / Source-Language Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Blocked-action / localized consumers: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Why-unavailable postures: {}\n",
            self.vocabulary_set.why_unavailable_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Source-language postures: {}\n",
            self.vocabulary_set.source_language_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Failure domains: {}\n",
            self.vocabulary_set.failure_domains.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Blocked-action / localized consumers\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked why-unavailable rows: {}\n",
                row.why_unavailable_examples.len()
            ));
            for case in &row.why_unavailable_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` / domain `{}` (next-safe `{}`, owner `{}`)\n",
                    case.resolved.row_identity_ref,
                    case.resolved.unavailable_reason.as_str(),
                    case.resolved.help_posture.as_str(),
                    case.resolved.failure_domain.as_str(),
                    case.resolved.next_safe_action.as_str(),
                    case.resolved.blocking_owner.as_str(),
                ));
            }
            out.push_str(&format!(
                "  - Worked source-language surfaces: {}\n",
                row.source_language_examples.len()
            ));
            for case in &row.source_language_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}` / `{}`) → `{}` (source shown `{}`, requires source text `{}`)\n",
                    case.resolved.row_identity_ref,
                    case.resolved.source_language_class.as_str(),
                    case.resolved.fallback_state.as_str(),
                    case.resolved.help_posture.as_str(),
                    case.resolved.shows_source_language,
                    case.resolved.requires_source_text,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 why-unavailable / source-language export.
#[derive(Debug)]
pub enum M5BlockedLocalizedRowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BlockedLocalizedRowViolation>),
}

impl fmt::Display for M5BlockedLocalizedRowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 blocked localized primitive export parse failed: {error}"
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
                    "m5 blocked localized primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BlockedLocalizedRowArtifactError {}

/// Validation failures emitted by [`M5BlockedLocalizedRowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BlockedLocalizedRowViolation {
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
    /// A required blocked-action / localized consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A consumer row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory why-unavailable anatomy parts.
    WhyUnavailableAnatomyMissing,
    /// A row omits one of the mandatory source-language anatomy parts.
    SourceLanguageAnatomyMissing,
    /// A row omits one of the mandatory why-unavailable export fields.
    WhyUnavailableExportMissing,
    /// A row omits one of the mandatory source-language export fields.
    SourceLanguageExportMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked why-unavailable resolutions.
    WhyUnavailableExampleMissing,
    /// A row declares no worked source-language resolutions.
    SourceLanguageExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The worked resolutions do not exercise every unavailable reason class.
    ReasonCoverageUnproven,
    /// The worked resolutions do not exercise every failure domain — a blocked action would still
    /// be able to collapse context, trust, policy, and runtime failures together.
    FailureDomainCoverageUnproven,
    /// The worked resolutions do not exercise every next-safe-action class.
    NextSafeActionCoverageUnproven,
    /// The worked resolutions do not prove every why-unavailable action.
    WhyUnavailableActionCoverageUnproven,
    /// The worked resolutions do not prove every why-unavailable posture.
    WhyUnavailablePostureCoverageUnproven,
    /// The worked resolutions do not exercise every source-language class.
    SourceLanguageClassCoverageUnproven,
    /// The worked resolutions do not exercise every fallback-state class.
    FallbackStateCoverageUnproven,
    /// The worked resolutions do not prove every source-language action.
    SourceLanguageActionCoverageUnproven,
    /// A source-language resolution does not preserve its stable ID and canonical citation — it
    /// could drift into an unsourced paraphrase.
    CitationPreservationUnproven,
    /// A worked resolution does not keep keyboard-only parity (screen reader + non-hover).
    KeyboardParityUnproven,
    /// A worked resolution does not preserve its owner / reason / next-safe-action or citation
    /// truth.
    ReversibilityUnproven,
    /// A worked resolution does not preserve its exact identity, references, or announcement.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
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

impl M5BlockedLocalizedRowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::WhyUnavailableAnatomyMissing => "why_unavailable_anatomy_missing",
            Self::SourceLanguageAnatomyMissing => "source_language_anatomy_missing",
            Self::WhyUnavailableExportMissing => "why_unavailable_export_missing",
            Self::SourceLanguageExportMissing => "source_language_export_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::WhyUnavailableExampleMissing => "why_unavailable_example_missing",
            Self::SourceLanguageExampleMissing => "source_language_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::ReasonCoverageUnproven => "reason_coverage_unproven",
            Self::FailureDomainCoverageUnproven => "failure_domain_coverage_unproven",
            Self::NextSafeActionCoverageUnproven => "next_safe_action_coverage_unproven",
            Self::WhyUnavailableActionCoverageUnproven => {
                "why_unavailable_action_coverage_unproven"
            }
            Self::WhyUnavailablePostureCoverageUnproven => {
                "why_unavailable_posture_coverage_unproven"
            }
            Self::SourceLanguageClassCoverageUnproven => "source_language_class_coverage_unproven",
            Self::FallbackStateCoverageUnproven => "fallback_state_coverage_unproven",
            Self::SourceLanguageActionCoverageUnproven => {
                "source_language_action_coverage_unproven"
            }
            Self::CitationPreservationUnproven => "citation_preservation_unproven",
            Self::KeyboardParityUnproven => "keyboard_parity_unproven",
            Self::ReversibilityUnproven => "reversibility_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 why-unavailable / source-language export.
pub fn current_stable_m5_blocked_localized_export(
) -> Result<M5BlockedLocalizedRowPacket, M5BlockedLocalizedRowArtifactError> {
    let packet: M5BlockedLocalizedRowPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-why-unavailable-source-language-primitive-proof/support_export.json"
    )))
    .map_err(M5BlockedLocalizedRowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BlockedLocalizedRowArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BLOCKED_LOCALIZED_ROW_SCHEMA_REF,
        M5_BLOCKED_LOCALIZED_ROW_DOC_REF,
        M5_BLOCKED_LOCALIZED_ROW_COMPONENT_MATRIX_REF,
        M5_BLOCKED_LOCALIZED_ROW_FEATURE_AVAILABILITY_REF,
        M5_BLOCKED_LOCALIZED_ROW_LOCALE_FALLBACK_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BlockedLocalizedRowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BlockedLocalizedRowViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let present: BTreeSet<M5BlockedLocalizedConsumerSurface> =
        packet.rows.iter().map(|row| row.consumer_surface).collect();
    for required in M5BlockedLocalizedConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5BlockedLocalizedRowViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.why_unavailable_anatomy_parts.is_empty()
            || row.source_language_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.blocked_action_owners.is_empty()
            || row.unavailable_reason_classes.is_empty()
            || row.next_safe_action_classes.is_empty()
            || row.failure_domains.is_empty()
            || row.why_unavailable_postures.is_empty()
            || row.why_unavailable_actions.is_empty()
            || row.source_language_classes.is_empty()
            || row.fallback_state_classes.is_empty()
            || row.source_language_postures.is_empty()
            || row.source_language_actions.is_empty()
            || row.why_unavailable_export_fields.is_empty()
            || row.source_language_export_fields.is_empty()
        {
            violations.push(M5BlockedLocalizedRowViolation::RowIncomplete);
        }
        if !row.declares_mandatory_why_unavailable_anatomy() {
            violations.push(M5BlockedLocalizedRowViolation::WhyUnavailableAnatomyMissing);
        }
        if !row.declares_mandatory_source_language_anatomy() {
            violations.push(M5BlockedLocalizedRowViolation::SourceLanguageAnatomyMissing);
        }
        if !row.declares_mandatory_why_unavailable_export() {
            violations.push(M5BlockedLocalizedRowViolation::WhyUnavailableExportMissing);
        }
        if !row.declares_mandatory_source_language_export() {
            violations.push(M5BlockedLocalizedRowViolation::SourceLanguageExportMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TeachingAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5BlockedLocalizedRowViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BlockedLocalizedRowViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BlockedLocalizedRowViolation::DowngradeTriggersMissing);
        }
        if row.why_unavailable_examples.is_empty() {
            violations.push(M5BlockedLocalizedRowViolation::WhyUnavailableExampleMissing);
        }
        if row.source_language_examples.is_empty() {
            violations.push(M5BlockedLocalizedRowViolation::SourceLanguageExampleMissing);
        }
        let why_drift = row
            .why_unavailable_examples
            .iter()
            .any(|case| !case.is_self_consistent());
        let source_drift = row
            .source_language_examples
            .iter()
            .any(|case| !case.is_self_consistent());
        if why_drift || source_drift {
            violations.push(M5BlockedLocalizedRowViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5BlockedLocalizedRowViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5BlockedLocalizedRowViolation::RowInvariantViolated);
        }
    }
}

fn why_unavailable_cases(
    packet: &M5BlockedLocalizedRowPacket,
) -> impl Iterator<Item = &M5WhyUnavailableRowResolutionCase> {
    packet
        .rows
        .iter()
        .flat_map(|row| row.why_unavailable_examples.iter())
}

fn source_language_cases(
    packet: &M5BlockedLocalizedRowPacket,
) -> impl Iterator<Item = &M5SourceLanguageFallbackResolutionCase> {
    packet
        .rows
        .iter()
        .flat_map(|row| row.source_language_examples.iter())
}

/// Every unavailable reason class must be exercised — the acceptance criterion that a blocked
/// action names its exact reason rather than a generic disabled state.
fn validate_reason_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let exercised: BTreeSet<M5UnavailableReasonClass> = why_unavailable_cases(packet)
        .map(|case| case.resolved.unavailable_reason)
        .collect();
    if !M5UnavailableReasonClass::ALL
        .iter()
        .all(|reason| exercised.contains(reason))
    {
        violations.push(M5BlockedLocalizedRowViolation::ReasonCoverageUnproven);
    }
}

/// Every failure domain (policy / trust / context / runtime) must be exercised — the acceptance
/// criterion that blocked actions stop collapsing context, trust, policy, and runtime failures
/// into one generic disabled state.
fn validate_failure_domain_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let exercised: BTreeSet<M5UnavailableFailureDomain> = why_unavailable_cases(packet)
        .map(|case| case.resolved.failure_domain)
        .collect();
    if !M5UnavailableFailureDomain::ALL
        .iter()
        .all(|domain| exercised.contains(domain))
    {
        violations.push(M5BlockedLocalizedRowViolation::FailureDomainCoverageUnproven);
    }
}

/// Every next-safe-action class — including the honest `no_safe_action` — must be exercised.
fn validate_next_safe_action_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let exercised: BTreeSet<M5NextSafeActionClass> = why_unavailable_cases(packet)
        .map(|case| case.resolved.next_safe_action)
        .collect();
    if !M5NextSafeActionClass::ALL
        .iter()
        .all(|class| exercised.contains(class))
    {
        violations.push(M5BlockedLocalizedRowViolation::NextSafeActionCoverageUnproven);
    }
}

/// Every bounded why-unavailable action must be proven by some worked resolution.
fn validate_why_unavailable_action_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let covered = M5WhyUnavailableAction::ALL.iter().all(|action| {
        why_unavailable_cases(packet).any(|case| case.resolved.available_actions.contains(action))
    });
    if !covered {
        violations.push(M5BlockedLocalizedRowViolation::WhyUnavailableActionCoverageUnproven);
    }
}

/// Every why-unavailable posture must be proven by some worked resolution.
fn validate_why_unavailable_posture_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let exercised: BTreeSet<M5WhyUnavailablePosture> = why_unavailable_cases(packet)
        .map(|case| case.resolved.help_posture)
        .collect();
    if !M5WhyUnavailablePosture::ALL
        .iter()
        .all(|posture| exercised.contains(posture))
    {
        violations.push(M5BlockedLocalizedRowViolation::WhyUnavailablePostureCoverageUnproven);
    }
}

/// Every source-language class must be exercised.
fn validate_source_language_class_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let exercised: BTreeSet<M5SourceLanguageClass> = source_language_cases(packet)
        .map(|case| case.resolved.source_language_class)
        .collect();
    if !M5SourceLanguageClass::ALL
        .iter()
        .all(|class| exercised.contains(class))
    {
        violations.push(M5BlockedLocalizedRowViolation::SourceLanguageClassCoverageUnproven);
    }
}

/// Every fallback-state class must be exercised.
fn validate_fallback_state_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let exercised: BTreeSet<M5FallbackStateClass> = source_language_cases(packet)
        .map(|case| case.resolved.fallback_state)
        .collect();
    if !M5FallbackStateClass::ALL
        .iter()
        .all(|class| exercised.contains(class))
    {
        violations.push(M5BlockedLocalizedRowViolation::FallbackStateCoverageUnproven);
    }
}

/// Every bounded source-language action must be proven by some worked resolution.
fn validate_source_language_action_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let covered = M5SourceLanguageAction::ALL.iter().all(|action| {
        source_language_cases(packet).any(|case| case.resolved.available_actions.contains(action))
    });
    if !covered {
        violations.push(M5BlockedLocalizedRowViolation::SourceLanguageActionCoverageUnproven);
    }
}

/// Every source-language resolution must preserve its stable ID and canonical citation — the
/// acceptance criterion that localized flows stay aligned with canonical IDs and cited source
/// material instead of drifting into unsourced paraphrase.
fn validate_citation_preservation(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    if !source_language_cases(packet).all(|case| case.preserves_citation()) {
        violations.push(M5BlockedLocalizedRowViolation::CitationPreservationUnproven);
    }
}

/// Every worked resolution must keep keyboard-only parity — never requiring pointer hover and
/// always carrying a screen-reader announcement.
fn validate_keyboard_parity_coverage(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let why_ok = why_unavailable_cases(packet).all(|case| {
        case.resolved.never_requires_pointer_hover
            && case.resolved.provides_screen_reader_announcement
    });
    let source_ok = source_language_cases(packet).all(|case| {
        case.resolved.never_requires_pointer_hover
            && case.resolved.provides_screen_reader_announcement
    });
    if !(why_ok && source_ok) {
        violations.push(M5BlockedLocalizedRowViolation::KeyboardParityUnproven);
    }
}

/// Every worked resolution must preserve its explanation / localized parity — the acceptance
/// criteria that a blocked action is always fully explained and a localized surface always cites
/// its canonical source.
fn validate_reversibility(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let why_ok = why_unavailable_cases(packet).all(|case| case.preserves_explanation_parity());
    let source_ok = source_language_cases(packet).all(|case| case.preserves_localized_parity());
    if !(why_ok && source_ok) {
        violations.push(M5BlockedLocalizedRowViolation::ReversibilityUnproven);
    }
}

/// Every worked resolution must preserve its exact identity and references.
fn validate_identity_preservation(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let why_ok = why_unavailable_cases(packet).all(|case| case.preserves_identity());
    let source_ok = source_language_cases(packet).all(|case| case.preserves_identity());
    if !(why_ok && source_ok) {
        violations.push(M5BlockedLocalizedRowViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.row_names_blocked_action,
        review.row_names_exact_reason,
        review.row_names_owning_boundary,
        review.row_names_next_safe_action,
        review.row_links_deeper_docs,
        review.blocked_actions_never_collapse_into_generic_disabled,
        review.fallback_preserves_source_language_text,
        review.fallback_preserves_stable_id,
        review.fallback_preserves_canonical_citation,
        review.localized_flows_never_drift_into_unsourced_paraphrase,
        review.surfaces_never_require_pointer_hover,
        review.surfaces_provide_screen_reader_announcement,
        review.rows_stable_across_deployment_lines,
        review.rows_stable_across_consumer_surfaces,
        review.support_export_reconstructs_truth,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BlockedLocalizedRowViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.surfaces_consume_shared_vocabulary,
        projection.why_unavailable_reads_single_source,
        projection.source_language_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5BlockedLocalizedRowViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BlockedLocalizedRowViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BlockedLocalizedRowPacket,
    violations: &mut Vec<M5BlockedLocalizedRowViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.blocked_localized_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BlockedLocalizedRowViolation::ReleasePostureIncomplete);
    }
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
