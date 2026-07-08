//! One reusable M5 lifecycle / channel badge primitive: the lifecycle stage a
//! capability is at (Labs / Preview / Beta / Stable / LTS surface / Deprecated /
//! Removal-scheduled) and the release channel it rides (Nightly / Preview / Beta /
//! Stable / LTS), projected the same way across every claimed M5 command, feature
//! surface, workflow bundle, extension/install row, and release/install surface — as
//! two distinct, composable cues rather than one overloaded badge.
//!
//! Aureline's frozen badge-family matrix
//! ([`crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix`])
//! names the lifecycle badge and the channel badge as two governed badge families and
//! freezes the shared badge infrastructure — the surface families, the deployment
//! lines, the accessibility routes, the qualification classes, the explanation-drawer
//! fields, the consumer surfaces, and the downgrade triggers. This module *implements*
//! those two families as one render-facing badge pair so a user can tell — from the two
//! badges and their explanation drawers alone — exactly how mature a capability is
//! (experimental, stable, deprecated, or scheduled for removal) *and* which channel it
//! is merely running on, without one badge implying the other.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_lifecycle_channel_badge`] — that takes one capability's
//!    subject label, its declared lifecycle stage, its declared channel, an optional
//!    replacement/migration path, and its last-evaluated timestamp, and produces one
//!    [`M5ResolvedLifecycleChannelBadge`] carrying both badges as separate typed fields,
//!    the derived effective maturity posture (experimental / preview / beta / stable /
//!    long-term-supported / deprecated / removal-scheduled), and — whenever the
//!    lifecycle is deprecated or removal-scheduled — a self-contained
//!    [`M5MigrationNote`] that names the exact sunset reason, the next action, the
//!    replacement/migration path, and the *preserved* channel context. The resolver
//!    never collapses the two axes into one pill, never derives the channel from the
//!    lifecycle (a Stable capability may still be running on a Preview channel), never
//!    derives the lifecycle from the channel, and never lets a deprecated or
//!    removal-scheduled badge become an inert warning without a replacement path.
//! 2. A parity matrix — [`M5MaturityBadgePrimitivePacket`] — that binds one row per
//!    claimed M5 badge consumer (the command row, the feature surface, the workflow
//!    bundle, the extension/install row, the release/install surface, and the ecosystem
//!    lifecycle review) to the shared badge anatomy, the same lifecycle values, channel
//!    values, effective-maturity postures, sunset reasons, next actions,
//!    explanation-drawer fields, export fields, and non-visual accessibility routes, so
//!    the lifecycle / channel vocabulary stays identical across commands, feature
//!    surfaces, workflow bundles, extension/install rows, release/install surfaces, and
//!    ecosystem lifecycle review.
//!
//! The badge surface family ([`M5BadgeSurfaceFamily`]), deployment line
//! ([`M5DeploymentLine`]), accessibility route ([`M5BadgeAccessibilityRoute`]),
//! qualification class ([`M5BadgeQualificationClass`]), explanation-drawer field
//! ([`M5BadgeExplanationField`]), consumer surface ([`M5BadgeConsumerSurface`]), and
//! downgrade trigger ([`M5BadgeDowngradeTrigger`]) are reused verbatim from the frozen
//! badge-family matrix. This module mints new vocabulary only for what that matrix left
//! implicit about the two rendered badges themselves: their render-facing value sets,
//! their badge consumers, their badge-pair anatomy parts, their effective-maturity
//! postures, their sunset reasons, their next actions, and their export fields. No M5
//! badge surface invents a second lifecycle or channel grammar.
//!
//! Raw URLs, raw signing keys, raw tokens, credentials, private endpoints, and user
//! text bodies stay outside the support boundary; every subject label, replacement
//! path, and timestamp is carried only as an opaque, export-safe representation.
//!
//! The boundary schema is
//! [`schemas/ui/m5-lifecycle-and-channel-badge.schema.json`](../../../../schemas/ui/m5-lifecycle-and-channel-badge.schema.json)
//! and the contract doc is
//! [`docs/release/m5_lifecycle_and_channel_badge_contract.md`](../../../../docs/release/m5_lifecycle_and_channel_badge_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-lifecycle-and-channel-badges/`](../../../../fixtures/ui/m5-lifecycle-and-channel-badges/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_maturity_badge_primitive_ecosystem_review_preview_narrowed,
    seeded_m5_maturity_badge_primitive_extension_install_row_beta_narrowed,
    seeded_m5_maturity_badge_primitive_packet, M5_MATURITY_BADGE_PRIMITIVE_PACKET_ID,
};

// The surface families, deployment lines, accessibility routes, qualification classes,
// explanation-drawer fields, consumer surfaces, and downgrade triggers are frozen once,
// in the badge-family matrix. This primitive reuses them verbatim so it never invents a
// parallel badge grammar for the shared badge infrastructure.
pub use crate::freeze_the_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix::{
    M5BadgeAccessibilityRoute, M5BadgeConsumerSurface, M5BadgeDowngradeTrigger,
    M5BadgeExplanationField, M5BadgeQualificationClass, M5BadgeSurfaceFamily, M5DeploymentLine,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5MaturityBadgePrimitivePacket`].
pub const M5_MATURITY_BADGE_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_lifecycle_and_channel_badges_across_claimed_m5_command_feature_bundle_extension_and_install_surfaces";

/// Schema version for M5 lifecycle / channel badge records.
pub const M5_MATURITY_BADGE_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the lifecycle / channel badge boundary schema.
pub const M5_MATURITY_BADGE_SCHEMA_REF: &str =
    "schemas/ui/m5-lifecycle-and-channel-badge.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MATURITY_BADGE_DOC_REF: &str =
    "docs/release/m5_lifecycle_and_channel_badge_contract.md";

/// Repo-relative path of the frozen badge-family matrix this primitive narrows from.
pub const M5_MATURITY_BADGE_FAMILY_MATRIX_REF: &str =
    "schemas/ui/m5-badge-family-matrix.schema.json";

/// Repo-relative path of the lifecycle badge contract this primitive projects
/// lifecycle posture from.
pub const M5_MATURITY_BADGE_LIFECYCLE_REF: &str = "schemas/ux/lifecycle_badge.schema.json";

/// Repo-relative path of the channel-association review row this primitive projects
/// channel truth from.
pub const M5_MATURITY_BADGE_CHANNEL_REF: &str =
    "schemas/ui/m5-channel-association-review-row.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_MATURITY_BADGE_FIXTURE_DIR: &str = "fixtures/ui/m5-lifecycle-and-channel-badges";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MATURITY_BADGE_ARTIFACT_REF: &str =
    "artifacts/release/m5-lifecycle-and-channel-badge-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_MATURITY_BADGE_CSV_REF: &str =
    "artifacts/release/m5-lifecycle-and-channel-badge-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_MATURITY_BADGE_REPORT_REF: &str =
    "artifacts/components/m5-lifecycle-and-channel-badges.md";

/// One claimed M5 badge consumer that renders the shared lifecycle and channel badge
/// pair. These are the surfaces the acceptance criteria name — commands, feature
/// surfaces, workflow bundles, extension/install rows, release/install surfaces, and the
/// ecosystem lifecycle review lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MaturityBadgeConsumerSurface {
    /// A command row (command palette / CLI listing).
    CommandRow,
    /// A feature surface (a capability card in the product).
    FeatureSurface,
    /// A workflow bundle launch card.
    WorkflowBundle,
    /// An extension / install row.
    ExtensionInstallRow,
    /// A release / install summary surface.
    ReleaseInstallSurface,
    /// The ecosystem lifecycle / install review lane.
    EcosystemLifecycleReview,
}

impl M5MaturityBadgeConsumerSurface {
    /// Every claimed badge consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CommandRow,
        Self::FeatureSurface,
        Self::WorkflowBundle,
        Self::ExtensionInstallRow,
        Self::ReleaseInstallSurface,
        Self::EcosystemLifecycleReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandRow => "command_row",
            Self::FeatureSurface => "feature_surface",
            Self::WorkflowBundle => "workflow_bundle",
            Self::ExtensionInstallRow => "extension_install_row",
            Self::ReleaseInstallSurface => "release_install_surface",
            Self::EcosystemLifecycleReview => "ecosystem_lifecycle_review",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::CommandRow => "Command Row",
            Self::FeatureSurface => "Feature Surface",
            Self::WorkflowBundle => "Workflow Bundle",
            Self::ExtensionInstallRow => "Extension / Install Row",
            Self::ReleaseInstallSurface => "Release / Install Surface",
            Self::EcosystemLifecycleReview => "Ecosystem Lifecycle Review",
        }
    }
}

/// Controlled lifecycle badge value — how mature a capability is. This is the
/// render-facing lifecycle vocabulary the acceptance criteria name: Labs, Preview,
/// Beta, Stable, LTS surface, Deprecated, Removal scheduled. A lifecycle badge never
/// leaves its stage implicit and never implies anything about the release channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleBadgeValue {
    /// Labs: earliest experimental stage; may change or be withdrawn.
    Labs,
    /// Preview: pre-release, close to Beta but still forming.
    Preview,
    /// Beta: feature-complete pre-release under active hardening.
    Beta,
    /// Stable: generally available and supported.
    Stable,
    /// LTS surface: stable on a long-term-support line.
    LtsSurface,
    /// Deprecated: superseded; still present but pointing to a replacement.
    Deprecated,
    /// Removal scheduled: a removal date is set; migration must complete first.
    RemovalScheduled,
}

impl M5LifecycleBadgeValue {
    /// Every lifecycle value, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Labs,
        Self::Preview,
        Self::Beta,
        Self::Stable,
        Self::LtsSurface,
        Self::Deprecated,
        Self::RemovalScheduled,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Labs => "labs",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::LtsSurface => "lts_surface",
            Self::Deprecated => "deprecated",
            Self::RemovalScheduled => "removal_scheduled",
        }
    }

    /// Review-safe label for the badge and note.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Labs => "Labs",
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::Stable => "Stable",
            Self::LtsSurface => "LTS surface",
            Self::Deprecated => "Deprecated",
            Self::RemovalScheduled => "Removal scheduled",
        }
    }
}

/// Controlled channel badge value — which release channel a capability is running on.
/// This is the render-facing channel vocabulary the acceptance criteria name: Nightly,
/// Preview, Beta, Stable, LTS. A channel badge never leaves the channel implicit and
/// never implies a lifecycle stage — a Stable capability may still be running on a
/// Preview channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelBadgeValue {
    /// The nightly channel.
    Nightly,
    /// The preview channel.
    Preview,
    /// The beta channel.
    Beta,
    /// The stable channel.
    Stable,
    /// The long-term-support channel.
    Lts,
}

impl M5ChannelBadgeValue {
    /// Every channel value, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Nightly,
        Self::Preview,
        Self::Beta,
        Self::Stable,
        Self::Lts,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Nightly => "nightly",
            Self::Preview => "preview",
            Self::Beta => "beta",
            Self::Stable => "stable",
            Self::Lts => "lts",
        }
    }

    /// Review-safe label for the badge.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Nightly => "Nightly",
            Self::Preview => "Preview",
            Self::Beta => "Beta",
            Self::Stable => "Stable",
            Self::Lts => "LTS",
        }
    }

    /// True when this is a pre-release channel (Nightly, Preview, or Beta) — the
    /// channels a *stable* capability can still be running on, proving the channel is
    /// not the lifecycle.
    pub const fn is_prerelease_channel(self) -> bool {
        matches!(self, Self::Nightly | Self::Preview | Self::Beta)
    }
}

/// One anatomy part the shared lifecycle / channel badge pair surfaces. The parts in
/// [`M5MaturityBadgeAnatomyPart::MANDATORY`] are required on every consumer so the two
/// cues stay distinct and each opens its own explanation drawer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MaturityBadgeAnatomyPart {
    /// The lifecycle badge itself.
    LifecycleBadge,
    /// The channel badge itself.
    ChannelBadge,
    /// The lifecycle explanation drawer.
    LifecycleExplanationDrawer,
    /// The channel explanation drawer.
    ChannelExplanationDrawer,
    /// The separately-filterable filter keys for both axes.
    FilterKeys,
    /// The derived effective-maturity note.
    EffectiveMaturityNote,
    /// The migration banner (shown when the lifecycle is deprecated or removal-scheduled).
    MigrationBanner,
}

impl M5MaturityBadgeAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LifecycleBadge,
        Self::ChannelBadge,
        Self::LifecycleExplanationDrawer,
        Self::ChannelExplanationDrawer,
        Self::FilterKeys,
        Self::EffectiveMaturityNote,
        Self::MigrationBanner,
    ];

    /// The anatomy parts every badge consumer must render: both badges as distinct
    /// cues, and both explanation drawers.
    pub const MANDATORY: [Self; 4] = [
        Self::LifecycleBadge,
        Self::ChannelBadge,
        Self::LifecycleExplanationDrawer,
        Self::ChannelExplanationDrawer,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LifecycleBadge => "lifecycle_badge",
            Self::ChannelBadge => "channel_badge",
            Self::LifecycleExplanationDrawer => "lifecycle_explanation_drawer",
            Self::ChannelExplanationDrawer => "channel_explanation_drawer",
            Self::FilterKeys => "filter_keys",
            Self::EffectiveMaturityNote => "effective_maturity_note",
            Self::MigrationBanner => "migration_banner",
        }
    }
}

/// The derived effective maturity — the resolver's verdict about how mature a
/// capability is, computed from the lifecycle axis alone so it never implies or is
/// implied by the channel.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EffectiveMaturityPosture {
    /// Experimental: earliest labs stage.
    MaturityExperimental,
    /// Preview: pre-release, forming.
    MaturityPreview,
    /// Beta: feature-complete pre-release.
    MaturityBeta,
    /// Stable: generally available.
    MaturityStable,
    /// Long-term supported: stable on an LTS line.
    MaturityLongTermSupported,
    /// Deprecated: superseded, pointing to a replacement.
    MaturityDeprecated,
    /// Removal scheduled: a removal date is set.
    MaturityRemovalScheduled,
}

impl M5EffectiveMaturityPosture {
    /// Every effective-maturity posture, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MaturityExperimental,
        Self::MaturityPreview,
        Self::MaturityBeta,
        Self::MaturityStable,
        Self::MaturityLongTermSupported,
        Self::MaturityDeprecated,
        Self::MaturityRemovalScheduled,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MaturityExperimental => "maturity_experimental",
            Self::MaturityPreview => "maturity_preview",
            Self::MaturityBeta => "maturity_beta",
            Self::MaturityStable => "maturity_stable",
            Self::MaturityLongTermSupported => "maturity_long_term_supported",
            Self::MaturityDeprecated => "maturity_deprecated",
            Self::MaturityRemovalScheduled => "maturity_removal_scheduled",
        }
    }

    /// True when the capability is on a stable line (Stable or long-term supported).
    pub const fn is_stable_line(self) -> bool {
        matches!(self, Self::MaturityStable | Self::MaturityLongTermSupported)
    }

    /// True when the capability is at a pre-release stage (experimental / preview / beta).
    pub const fn is_prerelease(self) -> bool {
        matches!(
            self,
            Self::MaturityExperimental | Self::MaturityPreview | Self::MaturityBeta
        )
    }

    /// True when the capability is sunsetting (deprecated or removal-scheduled) and must
    /// therefore point to a replacement/migration path.
    pub const fn is_sunsetting(self) -> bool {
        matches!(
            self,
            Self::MaturityDeprecated | Self::MaturityRemovalScheduled
        )
    }

    /// The sunset reason that requires a migration path, if any. Returns `None` for a
    /// non-sunsetting posture.
    pub const fn sunset_reason(self) -> Option<M5LifecycleSunsetReason> {
        Some(match self {
            Self::MaturityDeprecated => M5LifecycleSunsetReason::Deprecated,
            Self::MaturityRemovalScheduled => M5LifecycleSunsetReason::RemovalScheduled,
            _ => return None,
        })
    }
}

/// The exact reason a lifecycle stage is sunsetting, so a migration note never reads
/// like a generic `no longer available` warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleSunsetReason {
    /// The capability is deprecated and superseded by a replacement.
    Deprecated,
    /// The capability has a scheduled removal date; migration must complete first.
    RemovalScheduled,
}

impl M5LifecycleSunsetReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 2] = [Self::Deprecated, Self::RemovalScheduled];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deprecated => "deprecated",
            Self::RemovalScheduled => "removal_scheduled",
        }
    }

    /// Review-safe reason phrase for the migration-note headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::Deprecated => "this capability is deprecated and superseded",
            Self::RemovalScheduled => "this capability has a scheduled removal date",
        }
    }

    /// True when a removal date is scheduled (as opposed to a plain deprecation).
    pub const fn is_removal_scheduled(self) -> bool {
        matches!(self, Self::RemovalScheduled)
    }

    /// The next action a reviewer should take to complete the migration.
    pub const fn next_action(self) -> M5MaturityBadgeNextAction {
        match self {
            Self::Deprecated => M5MaturityBadgeNextAction::FollowMigrationPath,
            Self::RemovalScheduled => M5MaturityBadgeNextAction::CompleteMigrationBeforeRemoval,
        }
    }
}

/// The next action named on a migration note, so a deprecated or removal-scheduled
/// badge is actionable from the note itself rather than being an inert warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MaturityBadgeNextAction {
    /// Follow the replacement/migration path.
    FollowMigrationPath,
    /// Complete the migration before the scheduled removal date.
    CompleteMigrationBeforeRemoval,
}

impl M5MaturityBadgeNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 2] = [
        Self::FollowMigrationPath,
        Self::CompleteMigrationBeforeRemoval,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FollowMigrationPath => "follow_migration_path",
            Self::CompleteMigrationBeforeRemoval => "complete_migration_before_removal",
        }
    }
}

/// A field the support / export packet carries so lifecycle and channel truth is
/// reconstructable from the shared model. The fields in
/// [`M5MaturityBadgeExportField::MANDATORY`] are required, and the lifecycle, the
/// channel, and the replacement path are always carried as *separate* fields so
/// exported evidence never loses badge meaning or drops the migration path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MaturityBadgeExportField {
    /// The lifecycle value.
    Lifecycle,
    /// The channel value.
    Channel,
    /// The derived effective-maturity posture.
    EffectiveMaturity,
    /// The lifecycle explanation.
    LifecycleExplanation,
    /// The channel explanation.
    ChannelExplanation,
    /// The replacement / migration path (when deprecated or removal-scheduled).
    ReplacementPath,
    /// The opaque last-evaluated timestamp.
    LastEvaluated,
    /// The sunset reason (when deprecated or removal-scheduled).
    SunsetReason,
    /// The separately-filterable filter keys.
    FilterKeys,
}

impl M5MaturityBadgeExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::Lifecycle,
        Self::Channel,
        Self::EffectiveMaturity,
        Self::LifecycleExplanation,
        Self::ChannelExplanation,
        Self::ReplacementPath,
        Self::LastEvaluated,
        Self::SunsetReason,
        Self::FilterKeys,
    ];

    /// The export fields every badge export must carry: both badge axes as separate
    /// fields, the effective maturity, and the replacement path so a deprecated badge
    /// keeps its migration path in exported evidence.
    pub const MANDATORY: [Self; 4] = [
        Self::Lifecycle,
        Self::Channel,
        Self::EffectiveMaturity,
        Self::ReplacementPath,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lifecycle => "lifecycle",
            Self::Channel => "channel",
            Self::EffectiveMaturity => "effective_maturity",
            Self::LifecycleExplanation => "lifecycle_explanation",
            Self::ChannelExplanation => "channel_explanation",
            Self::ReplacementPath => "replacement_path",
            Self::LastEvaluated => "last_evaluated",
            Self::SunsetReason => "sunset_reason",
            Self::FilterKeys => "filter_keys",
        }
    }
}

/// A self-contained migration note: the exact sunset reason, the next action, the
/// replacement/migration path, and — the acceptance-criterion invariant — the
/// *preserved* channel context, so a deprecated or removal-scheduled badge points to a
/// real replacement path instead of becoming an inert warning, and the channel the
/// capability was running on is never dropped.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MigrationNote {
    /// The exact reason the lifecycle is sunsetting.
    pub reason: M5LifecycleSunsetReason,
    /// The next action a reviewer should take.
    pub next_action: M5MaturityBadgeNextAction,
    /// The opaque, export-safe replacement / migration path this badge points to.
    pub replacement_path: String,
    /// The channel the capability was running on, preserved as context even though the
    /// lifecycle is sunsetting. Always equals the resolved channel.
    pub preserved_channel: M5ChannelBadgeValue,
    /// True when a removal date is scheduled.
    pub is_removal_scheduled: bool,
    /// A deterministic, self-contained headline naming the reason, the replacement
    /// path, the preserved channel, and the next action — never a generic `no longer
    /// available` warning and never implying the channel from the lifecycle.
    pub headline: String,
}

/// The full input to the lifecycle / channel badge resolver for one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleChannelBadgeInput {
    /// The opaque, export-safe subject label.
    pub subject_label: String,
    /// The declared lifecycle stage.
    pub lifecycle: M5LifecycleBadgeValue,
    /// The declared release channel.
    pub channel: M5ChannelBadgeValue,
    /// The opaque, export-safe replacement / migration path. Required (non-empty)
    /// whenever the lifecycle is deprecated or removal-scheduled.
    pub replacement_path_repr: Option<String>,
    /// The opaque, export-safe last-evaluated representation.
    pub last_evaluated_repr: String,
}

/// The resolved lifecycle / channel truth for one capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLifecycleChannelBadge {
    /// The opaque subject label.
    pub subject_label: String,
    /// The lifecycle — carried as its own field, never merged with the channel.
    pub lifecycle: M5LifecycleBadgeValue,
    /// The channel — carried as its own field, never merged with the lifecycle.
    pub channel: M5ChannelBadgeValue,
    /// The derived effective maturity, computed from the lifecycle alone.
    pub effective_maturity: M5EffectiveMaturityPosture,
    /// True when the capability is on a stable line.
    pub is_stable_line: bool,
    /// True when the capability is at a pre-release stage.
    pub is_prerelease: bool,
    /// True when the capability is sunsetting (deprecated or removal-scheduled).
    pub is_sunsetting: bool,
    /// The opaque last-evaluated representation.
    pub last_evaluated_repr: String,
    /// The migration note, present whenever the lifecycle is sunsetting.
    pub migration_note: Option<M5MigrationNote>,
}

/// Errors returned by [`resolve_lifecycle_channel_badge`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5LifecycleChannelBadgeError {
    /// The subject label was empty.
    EmptySubjectLabel,
    /// The last-evaluated representation was empty.
    EmptyLastEvaluated,
    /// The lifecycle is deprecated or removal-scheduled but no replacement/migration
    /// path was supplied — a deprecated badge must never be an inert warning.
    MissingReplacementPath,
    /// A subject label, replacement path, or timestamp carried forbidden material.
    ForbiddenBadgeMaterial,
}

impl M5LifecycleChannelBadgeError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptySubjectLabel => "empty_subject_label",
            Self::EmptyLastEvaluated => "empty_last_evaluated",
            Self::MissingReplacementPath => "missing_replacement_path",
            Self::ForbiddenBadgeMaterial => "forbidden_badge_material",
        }
    }
}

impl fmt::Display for M5LifecycleChannelBadgeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "lifecycle/channel badge resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5LifecycleChannelBadgeError {}

/// Resolves one lifecycle / channel badge from its declared lifecycle stage and channel.
///
/// The lifecycle and the channel stay two separate, composable cues. The derived
/// effective maturity is computed from the lifecycle axis alone — a Stable capability
/// running on a Preview channel is still Stable, because maturity is never derived from
/// the channel and the channel is never derived from the lifecycle. When the lifecycle
/// is deprecated or removal-scheduled, the resolver requires a replacement/migration
/// path and produces a self-contained migration note that *preserves* the channel
/// context rather than dropping it — a deprecated badge always points somewhere.
pub fn resolve_lifecycle_channel_badge(
    input: &M5LifecycleChannelBadgeInput,
) -> Result<M5ResolvedLifecycleChannelBadge, M5LifecycleChannelBadgeError> {
    if input.subject_label.trim().is_empty() {
        return Err(M5LifecycleChannelBadgeError::EmptySubjectLabel);
    }
    if input.last_evaluated_repr.trim().is_empty() {
        return Err(M5LifecycleChannelBadgeError::EmptyLastEvaluated);
    }
    let replacement_path = input
        .replacement_path_repr
        .as_deref()
        .map(str::trim)
        .unwrap_or("");
    if value_repr_is_forbidden(&input.subject_label)
        || value_repr_is_forbidden(&input.last_evaluated_repr)
        || value_repr_is_forbidden(replacement_path)
    {
        return Err(M5LifecycleChannelBadgeError::ForbiddenBadgeMaterial);
    }

    let effective_maturity = derive_effective_maturity(input.lifecycle);
    let is_stable_line = effective_maturity.is_stable_line();
    let is_prerelease = effective_maturity.is_prerelease();
    let is_sunsetting = effective_maturity.is_sunsetting();

    let migration_note = match effective_maturity.sunset_reason() {
        Some(reason) => {
            if replacement_path.is_empty() {
                return Err(M5LifecycleChannelBadgeError::MissingReplacementPath);
            }
            let next_action = reason.next_action();
            let headline = format!(
                "Lifecycle {}: {} — migrate via '{}'; running on '{}' channel (preserved); next: {}",
                if reason.is_removal_scheduled() {
                    "removal scheduled"
                } else {
                    "deprecated"
                },
                reason.phrase(),
                replacement_path,
                input.channel.label(),
                next_action.as_str()
            );
            Some(M5MigrationNote {
                reason,
                next_action,
                replacement_path: replacement_path.to_owned(),
                preserved_channel: input.channel,
                is_removal_scheduled: reason.is_removal_scheduled(),
                headline,
            })
        }
        None => None,
    };

    Ok(M5ResolvedLifecycleChannelBadge {
        subject_label: input.subject_label.clone(),
        lifecycle: input.lifecycle,
        channel: input.channel,
        effective_maturity,
        is_stable_line,
        is_prerelease,
        is_sunsetting,
        last_evaluated_repr: input.last_evaluated_repr.clone(),
        migration_note,
    })
}

/// Derives the effective maturity from the lifecycle alone, so the channel is never
/// derived from the lifecycle and the lifecycle is never derived from the channel.
fn derive_effective_maturity(lifecycle: M5LifecycleBadgeValue) -> M5EffectiveMaturityPosture {
    match lifecycle {
        M5LifecycleBadgeValue::Labs => M5EffectiveMaturityPosture::MaturityExperimental,
        M5LifecycleBadgeValue::Preview => M5EffectiveMaturityPosture::MaturityPreview,
        M5LifecycleBadgeValue::Beta => M5EffectiveMaturityPosture::MaturityBeta,
        M5LifecycleBadgeValue::Stable => M5EffectiveMaturityPosture::MaturityStable,
        M5LifecycleBadgeValue::LtsSurface => M5EffectiveMaturityPosture::MaturityLongTermSupported,
        M5LifecycleBadgeValue::Deprecated => M5EffectiveMaturityPosture::MaturityDeprecated,
        M5LifecycleBadgeValue::RemovalScheduled => {
            M5EffectiveMaturityPosture::MaturityRemovalScheduled
        }
    }
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs lifecycle and channel truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LifecycleChannelResolutionCase {
    /// The resolver input.
    pub input: M5LifecycleChannelBadgeInput,
    /// The resolved truth. Must equal `resolve_lifecycle_channel_badge(&input)`.
    pub resolved: M5ResolvedLifecycleChannelBadge,
}

impl M5LifecycleChannelResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5LifecycleChannelBadgeInput) -> Self {
        let resolved =
            resolve_lifecycle_channel_badge(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_lifecycle_channel_badge(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one badge consumer bound to the shared badge
/// anatomy, lifecycle values, channel values, effective-maturity postures, sunset
/// reasons, next actions, explanation-drawer fields, export fields, and accessibility
/// routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MaturityBadgeRow {
    /// Badge consumer family.
    pub consumer_surface: M5MaturityBadgeConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5BadgeQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 badge surface families that render / consume this pair.
    pub surface_families: Vec<M5BadgeSurfaceFamily>,
    /// Deployment lines this pair keeps the same truth across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Anatomy parts this consumer renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5MaturityBadgeAnatomyPart>,
    /// Lifecycle values this consumer names.
    pub lifecycle_values: Vec<M5LifecycleBadgeValue>,
    /// Channel values this consumer distinguishes.
    pub channel_values: Vec<M5ChannelBadgeValue>,
    /// Effective-maturity postures this consumer distinguishes.
    pub effective_maturity_postures: Vec<M5EffectiveMaturityPosture>,
    /// Sunset reasons this consumer names.
    pub sunset_reasons: Vec<M5LifecycleSunsetReason>,
    /// Next actions this consumer names.
    pub next_actions: Vec<M5MaturityBadgeNextAction>,
    /// Explanation-drawer fields this consumer opens (must include the mandatory
    /// [`M5BadgeExplanationField::MANDATORY`] fields).
    pub explanation_fields: Vec<M5BadgeExplanationField>,
    /// Export fields this consumer carries (must include the mandatory fields).
    pub export_fields: Vec<M5MaturityBadgeExportField>,
    /// Non-visual accessibility routes this consumer offers.
    pub accessibility_routes: Vec<M5BadgeAccessibilityRoute>,
    /// Badge subsystems that consume this pair's projection.
    pub consumer_surfaces: Vec<M5BadgeConsumerSurface>,
    /// Downgrade triggers that apply to this consumer.
    pub downgrade_triggers: Vec<M5BadgeDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this consumer.
    pub example_resolutions: Vec<M5LifecycleChannelResolutionCase>,
    /// Hard invariant: this consumer never collapses the lifecycle and channel axes
    /// into one overloaded badge. MUST be `false`.
    pub collapses_lifecycle_and_channel_into_one_badge: bool,
    /// Hard invariant: this consumer never implies the channel from the lifecycle. MUST
    /// be `false`.
    pub implies_channel_from_lifecycle: bool,
    /// Hard invariant: this consumer never drops the replacement/migration path when a
    /// badge is deprecated or removal-scheduled. MUST be `false`.
    pub drops_migration_path_on_deprecation: bool,
    /// Hard invariant: this consumer never lets exported evidence lose badge meaning.
    /// MUST be `false`.
    pub drops_badge_meaning_in_export: bool,
}

impl M5MaturityBadgeRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5MaturityBadgeAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5MaturityBadgeAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5MaturityBadgeExportField> =
            self.export_fields.iter().copied().collect();
        M5MaturityBadgeExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory explanation-drawer field.
    fn declares_mandatory_explanation_fields(&self) -> bool {
        let present: BTreeSet<M5BadgeExplanationField> =
            self.explanation_fields.iter().copied().collect();
        M5BadgeExplanationField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_lifecycle_and_channel_into_one_badge
            && !self.implies_channel_from_lifecycle
            && !self.drops_migration_path_on_deprecation
            && !self.drops_badge_meaning_in_export
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MaturityBadgeVocabularySet {
    /// Badge-consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Lifecycle-value tokens.
    pub lifecycle_values: Vec<String>,
    /// Channel-value tokens.
    pub channel_values: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Effective-maturity-posture tokens.
    pub effective_maturity_postures: Vec<String>,
    /// Sunset-reason tokens.
    pub sunset_reasons: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Explanation-field tokens (reused from the frozen matrix).
    pub explanation_fields: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
    /// Badge-consumer-subsystem tokens (reused from the frozen matrix).
    pub badge_consumer_surfaces: Vec<String>,
    /// Downgrade-trigger tokens (reused from the frozen matrix).
    pub downgrade_triggers: Vec<String>,
}

impl M5MaturityBadgeVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5MaturityBadgeConsumerSurface::ALL, |v| v.as_str()),
            lifecycle_values: tokens(&M5LifecycleBadgeValue::ALL, |v| v.as_str()),
            channel_values: tokens(&M5ChannelBadgeValue::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5MaturityBadgeAnatomyPart::ALL, |v| v.as_str()),
            effective_maturity_postures: tokens(&M5EffectiveMaturityPosture::ALL, |v| v.as_str()),
            sunset_reasons: tokens(&M5LifecycleSunsetReason::ALL, |v| v.as_str()),
            next_actions: tokens(&M5MaturityBadgeNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5MaturityBadgeExportField::ALL, |v| v.as_str()),
            explanation_fields: tokens(&M5BadgeExplanationField::ALL, |v| v.as_str()),
            surface_families: tokens(&M5BadgeSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DeploymentLine::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5BadgeAccessibilityRoute::ALL, |v| v.as_str()),
            badge_consumer_surfaces: tokens(&M5BadgeConsumerSurface::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5BadgeDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5MaturityBadgeGovernanceReview {
    /// Lifecycle and channel are shown as two distinct, composable cues.
    pub lifecycle_and_channel_shown_as_distinct_cues: bool,
    /// Neither badge is ever collapsed into the other.
    pub neither_badge_collapsed_into_the_other: bool,
    /// The lifecycle never implies the channel.
    pub lifecycle_never_implies_channel: bool,
    /// The channel never implies the lifecycle.
    pub channel_never_implies_lifecycle: bool,
    /// A deprecated or removal-scheduled badge automatically points to a
    /// replacement/migration path.
    pub deprecated_or_removal_auto_points_to_migration_path: bool,
    /// The migration note preserves the underlying channel context.
    pub migration_note_preserves_channel_context: bool,
    /// Every badge can open its explanation drawer.
    pub every_badge_opens_explanation_drawer: bool,
    /// Every badge is separately filterable.
    pub every_badge_is_separately_filterable: bool,
    /// Exported evidence keeps both badges' meaning.
    pub exported_evidence_keeps_badge_meaning: bool,
    /// No surface invents a second lifecycle or channel grammar.
    pub no_surface_invents_second_badge_grammar: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel badge vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MaturityBadgeConsumerProjection {
    /// Command, feature-surface, and workflow-bundle surfaces consume the shared pair.
    pub command_feature_bundle_surfaces_consume_shared_badges: bool,
    /// Extension/install and release/install surfaces consume the shared pair.
    pub extension_install_release_surfaces_consume_shared_badges: bool,
    /// The lifecycle filter reads a single canonical source.
    pub lifecycle_filter_reads_single_source: bool,
    /// The channel filter reads a single canonical source.
    pub channel_filter_reads_single_source: bool,
    /// Support / export reads a single canonical badge-pair source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MaturityBadgeProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the lifecycle / channel badge primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MaturityBadgeReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting badge audit.
    pub badge_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MaturityBadgePrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MaturityBadgePrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge rows.
    pub badge_rows: Vec<M5MaturityBadgeRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MaturityBadgeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MaturityBadgeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MaturityBadgeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MaturityBadgeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MaturityBadgeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 lifecycle / channel badge primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MaturityBadgePrimitivePacket {
    /// Record kind; must equal [`M5_MATURITY_BADGE_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MATURITY_BADGE_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge rows.
    pub badge_rows: Vec<M5MaturityBadgeRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MaturityBadgeVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MaturityBadgeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MaturityBadgeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MaturityBadgeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MaturityBadgeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MaturityBadgePrimitivePacket {
    /// Builds an M5 lifecycle / channel badge primitive packet from stable-lane input.
    pub fn new(input: M5MaturityBadgePrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_MATURITY_BADGE_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_MATURITY_BADGE_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            badge_rows: input.badge_rows,
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

    /// Validates the M5 lifecycle / channel badge primitive invariants.
    pub fn validate(&self) -> Vec<M5MaturityBadgePrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MATURITY_BADGE_PRIMITIVE_RECORD_KIND {
            violations.push(M5MaturityBadgePrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MATURITY_BADGE_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5MaturityBadgePrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MaturityBadgePrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_badge_rows(self, &mut violations);
        validate_distinction_coverage(self, &mut violations);
        validate_migration_path_preservation_coverage(self, &mut violations);
        validate_stable_and_sunsetting_coverage(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 maturity badge primitive packet serializes"),
        ) {
            violations.push(M5MaturityBadgePrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 maturity badge primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per badge consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,anatomy_parts,lifecycle_values,channel_values,effective_maturity_postures,sunset_reasons,next_actions,export_fields,example_count\n",
        );
        for row in &self.badge_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.lifecycle_values, |v| v.as_str()),
                join_tokens(&row.channel_values, |v| v.as_str()),
                join_tokens(&row.effective_maturity_postures, |v| v.as_str()),
                join_tokens(&row.sunset_reasons, |v| v.as_str()),
                join_tokens(&row.next_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_resolutions.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .badge_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Lifecycle and Channel Badge Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Badge consumers: {} ({} stable)\n",
            self.badge_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Lifecycle values: {}\n",
            self.vocabulary_set.lifecycle_values.join(", ")
        ));
        out.push_str(&format!(
            "- Channel values: {}\n",
            self.vocabulary_set.channel_values.join(", ")
        ));
        out.push_str(&format!(
            "- Effective-maturity postures: {}\n",
            self.vocabulary_set.effective_maturity_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Badge consumers\n\n");
        for row in &self.badge_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked resolutions: {}\n",
                row.example_resolutions.len()
            ));
            for case in &row.example_resolutions {
                let note = match &case.resolved.migration_note {
                    Some(note) => note.reason.as_str(),
                    None => "no_migration",
                };
                out.push_str(&format!(
                    "    - lifecycle `{}` + channel `{}` → `{}` (note `{}`)\n",
                    case.resolved.lifecycle.as_str(),
                    case.resolved.channel.as_str(),
                    case.resolved.effective_maturity.as_str(),
                    note
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 lifecycle / channel badge primitive
/// export.
#[derive(Debug)]
pub enum M5MaturityBadgePrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MaturityBadgePrimitiveViolation>),
}

impl fmt::Display for M5MaturityBadgePrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 maturity badge primitive export parse failed: {error}"
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
                    "m5 maturity badge primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MaturityBadgePrimitiveArtifactError {}

/// Validation failures emitted by [`M5MaturityBadgePrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MaturityBadgePrimitiveViolation {
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
    /// A required badge consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A badge row is incomplete.
    BadgeRowIncomplete,
    /// A badge row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A badge row declares no lifecycle values.
    LifecycleValueMissing,
    /// A badge row declares no channel values.
    ChannelValueMissing,
    /// A badge row declares no effective-maturity postures.
    EffectiveMaturityPostureMissing,
    /// A badge row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A badge row omits one of the mandatory explanation-drawer fields.
    ExplanationDrawerIncomplete,
    /// A badge row declares no accessibility routes (or misses keyboard focus or
    /// non-color encoding).
    AccessibilityRouteMissing,
    /// A badge row declares no badge-consumer subsystems.
    ConsumerSurfacesMissing,
    /// A badge row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A badge row declares no worked resolution cases.
    ExampleResolutionMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A badge claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// No worked resolution proves lifecycle and channel as distinct cues (a stable-line
    /// capability merely running on a pre-release channel, or a pre-release capability on
    /// the stable channel).
    LifecycleChannelDistinctionUnproven,
    /// No worked resolution proves a sunsetting badge preserving its channel context and
    /// pointing to a replacement/migration path.
    MigrationPathPreservationUnproven,
    /// No worked resolution proves both a stable-line and a sunsetting capability.
    StableAndSunsettingCoverageUnproven,
    /// A badge row violates a hard invariant.
    BadgeInvariantViolated,
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

impl M5MaturityBadgePrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::BadgeRowIncomplete => "badge_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::LifecycleValueMissing => "lifecycle_value_missing",
            Self::ChannelValueMissing => "channel_value_missing",
            Self::EffectiveMaturityPostureMissing => "effective_maturity_posture_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ExplanationDrawerIncomplete => "explanation_drawer_incomplete",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::LifecycleChannelDistinctionUnproven => "lifecycle_channel_distinction_unproven",
            Self::MigrationPathPreservationUnproven => "migration_path_preservation_unproven",
            Self::StableAndSunsettingCoverageUnproven => "stable_and_sunsetting_coverage_unproven",
            Self::BadgeInvariantViolated => "badge_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 lifecycle / channel badge primitive
/// export.
pub fn current_stable_m5_maturity_badge_primitive_export(
) -> Result<M5MaturityBadgePrimitivePacket, M5MaturityBadgePrimitiveArtifactError> {
    let packet: M5MaturityBadgePrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-lifecycle-and-channel-badge-proof/support_export.json"
    )))
    .map_err(M5MaturityBadgePrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MaturityBadgePrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MATURITY_BADGE_SCHEMA_REF,
        M5_MATURITY_BADGE_DOC_REF,
        M5_MATURITY_BADGE_FAMILY_MATRIX_REF,
        M5_MATURITY_BADGE_LIFECYCLE_REF,
        M5_MATURITY_BADGE_CHANNEL_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MaturityBadgePrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5MaturityBadgePrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_badge_rows(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    let present: BTreeSet<M5MaturityBadgeConsumerSurface> = packet
        .badge_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5MaturityBadgeConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5MaturityBadgePrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.badge_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.sunset_reasons.is_empty()
            || row.next_actions.is_empty()
        {
            violations.push(M5MaturityBadgePrimitiveViolation::BadgeRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5MaturityBadgePrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.lifecycle_values.is_empty() {
            violations.push(M5MaturityBadgePrimitiveViolation::LifecycleValueMissing);
        }
        if row.channel_values.is_empty() {
            violations.push(M5MaturityBadgePrimitiveViolation::ChannelValueMissing);
        }
        if row.effective_maturity_postures.is_empty() {
            violations.push(M5MaturityBadgePrimitiveViolation::EffectiveMaturityPostureMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5MaturityBadgePrimitiveViolation::MandatoryExportFieldMissing);
        }
        if !row.declares_mandatory_explanation_fields() {
            violations.push(M5MaturityBadgePrimitiveViolation::ExplanationDrawerIncomplete);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::KeyboardFocusable)
            || !row
                .accessibility_routes
                .contains(&M5BadgeAccessibilityRoute::NonColorEncoded)
        {
            violations.push(M5MaturityBadgePrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5MaturityBadgePrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5MaturityBadgePrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_resolutions.is_empty() {
            violations.push(M5MaturityBadgePrimitiveViolation::ExampleResolutionMissing);
        }
        if row
            .example_resolutions
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5MaturityBadgePrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5MaturityBadgePrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5MaturityBadgePrimitiveViolation::BadgeInvariantViolated);
        }
    }
}

/// AC1: at least one worked resolution must prove the lifecycle and the channel stay
/// distinct, composable cues — a stable-line capability merely running on a pre-release
/// channel (Nightly / Preview / Beta), or a pre-release capability on the stable
/// channel — proving that neither axis is derived from the other.
fn validate_distinction_coverage(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    let proven = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            let resolved = &case.resolved;
            (resolved.is_stable_line && resolved.channel.is_prerelease_channel())
                || (resolved.is_prerelease && resolved.channel == M5ChannelBadgeValue::Stable)
        })
    });
    if !proven {
        violations.push(M5MaturityBadgePrimitiveViolation::LifecycleChannelDistinctionUnproven);
    }
}

/// AC2: at least one worked resolution must prove a deprecated or removal-scheduled
/// badge whose migration note points to a non-empty replacement/migration path and
/// preserves the underlying channel context — the badge points somewhere rather than
/// becoming an inert warning.
fn validate_migration_path_preservation_coverage(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    let proven = packet.badge_rows.iter().any(|row| {
        row.example_resolutions.iter().any(|case| {
            case.resolved.is_sunsetting
                && case.resolved.migration_note.as_ref().is_some_and(|note| {
                    !note.replacement_path.trim().is_empty()
                        && note.preserved_channel == case.resolved.channel
                        && !note.headline.trim().is_empty()
                })
        })
    });
    if !proven {
        violations.push(M5MaturityBadgePrimitiveViolation::MigrationPathPreservationUnproven);
    }
}

/// At least one worked resolution must prove a stable-line capability and at least one
/// must prove a sunsetting capability — the acceptance-criterion example that lifecycle
/// spans stable through removal-scheduled independently of the channel.
fn validate_stable_and_sunsetting_coverage(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    let has_stable = packet.badge_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_stable_line)
    });
    let has_sunsetting = packet.badge_rows.iter().any(|row| {
        row.example_resolutions
            .iter()
            .any(|case| case.resolved.is_sunsetting)
    });
    if !(has_stable && has_sunsetting) {
        violations.push(M5MaturityBadgePrimitiveViolation::StableAndSunsettingCoverageUnproven);
    }
}

fn validate_governance_review(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.lifecycle_and_channel_shown_as_distinct_cues,
        review.neither_badge_collapsed_into_the_other,
        review.lifecycle_never_implies_channel,
        review.channel_never_implies_lifecycle,
        review.deprecated_or_removal_auto_points_to_migration_path,
        review.migration_note_preserves_channel_context,
        review.every_badge_opens_explanation_drawer,
        review.every_badge_is_separately_filterable,
        review.exported_evidence_keeps_badge_meaning,
        review.no_surface_invents_second_badge_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5MaturityBadgePrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.command_feature_bundle_surfaces_consume_shared_badges,
        projection.extension_install_release_surfaces_consume_shared_badges,
        projection.lifecycle_filter_reads_single_source,
        projection.channel_filter_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5MaturityBadgePrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MaturityBadgePrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MaturityBadgePrimitivePacket,
    violations: &mut Vec<M5MaturityBadgePrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.badge_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MaturityBadgePrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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
