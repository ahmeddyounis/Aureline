//! Frozen M5 support-class, evidence-freshness, lifecycle, channel,
//! deployment-scope, compatibility-state, and explanation-drawer badge matrix.
//!
//! This module locks Aureline's reusable badge families into one export-safe
//! packet. Every badge axis M5 shows on a claim-bearing surface — the support
//! class, the evidence freshness, the lifecycle, the release channel, the
//! deployment scope, and the compatibility state — is named once here and
//! constrained by the same value vocabulary, explanation-drawer requirement,
//! axis-separation rule, and downgrade rule regardless of which surface renders
//! it. A badge on the marketplace means exactly what the same badge means in
//! Help, in Settings, in onboarding, in diagnostics, and in exported support
//! evidence.
//!
//! What this matrix freezes is the stable vocabulary for the *badge families*
//! themselves: the six badge families, the controlled value set for each axis,
//! the mandatory explanation-drawer fields every badge must be able to open, the
//! axis-separation rules that keep one family from implying another, the
//! surface families and deployment lines every badge must survive, the non-visual
//! accessibility routes, and the mandatory labels every badge must be able to
//! show. It does not re-architect the support-class ledger, freshness descriptor,
//! lifecycle registry, channel matrix, deployment profile, or compatibility
//! forecast that already own those records — it is the shared badge contract
//! layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 badge may
//! render a support, freshness, lifecycle, channel, deployment, or compatibility
//! claim. Marketplace, Help/Docs, Settings, onboarding, diagnostics, runtime, and
//! exported-evidence surfaces all consume this packet so one support-class badge
//! never implies freshness, one deployment-scope badge never implies a lifecycle
//! stage, and no badge collapses two axes into a single overloaded pill. No M5
//! lane invents a second badge grammar, merges two badge axes, implies one axis
//! from another, or lets exported evidence lose badge meaning.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5BadgeVocabularySet`] rather than minted per surface. Raw URLs, raw signing
//! keys, raw tokens, credentials, private endpoints, and user text bodies stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-badge-family-matrix.schema.json`](../../../../schemas/ui/m5-badge-family-matrix.schema.json)
//! and the contract doc is
//! [`docs/release/m5_badge_family_matrix_contract.md`](../../../../docs/release/m5_badge_family_matrix_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-badge-family-consumers/`](../../../../fixtures/ui/m5-badge-family-consumers/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_badge_family_matrix, seeded_m5_badge_family_matrix_channel_badge_beta_narrowed,
    seeded_m5_badge_family_matrix_compatibility_state_badge_preview_narrowed,
    M5_BADGE_FAMILY_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5BadgeFamilyMatrixPacket`].
pub const M5_BADGE_FAMILY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_support_class_evidence_freshness_lifecycle_channel_deployment_scope_compatibility_state_and_explanation_drawer_badge_matrix";

/// Schema version for M5 badge-family-matrix records.
pub const M5_BADGE_FAMILY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the badge-family-matrix boundary schema.
pub const M5_BADGE_FAMILY_SCHEMA_REF: &str = "schemas/ui/m5-badge-family-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_BADGE_FAMILY_DOC_REF: &str = "docs/release/m5_badge_family_matrix_contract.md";

/// Repo-relative path of the support-class ledger this matrix binds against.
pub const M5_BADGE_FAMILY_SUPPORT_CLASS_REF: &str =
    "schemas/release/support_class_ledger.schema.json";

/// Repo-relative path of the evidence-freshness descriptor this matrix binds
/// against.
pub const M5_BADGE_FAMILY_FRESHNESS_REF: &str =
    "schemas/provenance/m5-freshness-descriptor.schema.json";

/// Repo-relative path of the lifecycle vocabulary this matrix binds against.
pub const M5_BADGE_FAMILY_LIFECYCLE_REF: &str =
    "schemas/lifecycle/m5-lifecycle-vocabulary-parity.schema.json";

/// Repo-relative path of the compatibility forecast this matrix binds against.
pub const M5_BADGE_FAMILY_COMPATIBILITY_REF: &str =
    "schemas/release/m5-compatibility-forecast.schema.json";

/// Repo-relative path of the badge-vocabulary contract this matrix binds against.
pub const M5_BADGE_FAMILY_BADGE_VOCABULARY_REF: &str =
    "schemas/provenance/m5-badge-vocabulary.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_BADGE_FAMILY_FIXTURE_DIR: &str = "fixtures/ui/m5-badge-family-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const M5_BADGE_FAMILY_ARTIFACT_REF: &str =
    "artifacts/release/m5-badge-family-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_BADGE_FAMILY_CSV_REF: &str = "artifacts/release/m5-badge-family-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_BADGE_FAMILY_REPORT_REF: &str = "artifacts/components/m5-badge-family-components.md";

/// One of the six governed badge families this matrix freezes. Each family is a
/// distinct badge axis with its own controlled value vocabulary; no family may
/// imply, merge with, or stand in for another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeFamily {
    /// The support-class badge: how supported a thing is.
    SupportClass,
    /// The evidence-freshness badge: how fresh the proof behind a claim is.
    EvidenceFreshness,
    /// The lifecycle badge: the lifecycle stage of a thing.
    Lifecycle,
    /// The channel badge: which release channel a thing rides.
    Channel,
    /// The deployment-scope badge: where a thing runs / is available.
    DeploymentScope,
    /// The compatibility-state badge: how compatible a thing is with the host.
    CompatibilityState,
}

impl M5BadgeFamily {
    /// Every governed badge family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SupportClass,
        Self::EvidenceFreshness,
        Self::Lifecycle,
        Self::Channel,
        Self::DeploymentScope,
        Self::CompatibilityState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportClass => "support_class",
            Self::EvidenceFreshness => "evidence_freshness",
            Self::Lifecycle => "lifecycle",
            Self::Channel => "channel",
            Self::DeploymentScope => "deployment_scope",
            Self::CompatibilityState => "compatibility_state",
        }
    }

    /// `true` when this family is the support-class badge and must declare its
    /// support-class values.
    pub const fn is_support_class(self) -> bool {
        matches!(self, Self::SupportClass)
    }

    /// `true` when this family is the evidence-freshness badge and must declare its
    /// freshness values.
    pub const fn is_evidence_freshness(self) -> bool {
        matches!(self, Self::EvidenceFreshness)
    }

    /// `true` when this family is the lifecycle badge and must declare its
    /// lifecycle values.
    pub const fn is_lifecycle(self) -> bool {
        matches!(self, Self::Lifecycle)
    }

    /// `true` when this family is the channel badge and must declare its channel
    /// values.
    pub const fn is_channel(self) -> bool {
        matches!(self, Self::Channel)
    }

    /// `true` when this family is the deployment-scope badge and must declare its
    /// deployment-scope values.
    pub const fn is_deployment_scope(self) -> bool {
        matches!(self, Self::DeploymentScope)
    }

    /// `true` when this family is the compatibility-state badge and must declare
    /// its compatibility-state values.
    pub const fn is_compatibility_state(self) -> bool {
        matches!(self, Self::CompatibilityState)
    }
}

/// Controlled support-class badge value — how supported a thing is, so a
/// support-class badge never leaves its support posture implicit and never
/// implies anything about freshness.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SupportClassBadge {
    /// Certified: fully supported and independently certified.
    Certified,
    /// Fully supported by the vendor with an active support window.
    FullySupported,
    /// Community supported only.
    CommunitySupported,
    /// Best-effort support, no guarantee.
    BestEffort,
    /// Deprecated: still available but scheduled to be withdrawn.
    Deprecated,
    /// Unsupported: no support offered.
    Unsupported,
}

impl M5SupportClassBadge {
    /// Every support-class value, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Certified,
        Self::FullySupported,
        Self::CommunitySupported,
        Self::BestEffort,
        Self::Deprecated,
        Self::Unsupported,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Certified => "certified",
            Self::FullySupported => "fully_supported",
            Self::CommunitySupported => "community_supported",
            Self::BestEffort => "best_effort",
            Self::Deprecated => "deprecated",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Controlled evidence-freshness badge value — how fresh the proof behind a claim
/// is, so a freshness badge never presents stale or unverified evidence as fresh.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshnessBadge {
    /// Fresh: evidence is within the freshness SLO.
    Fresh,
    /// Recent: evidence is aging but still inside the window.
    Recent,
    /// Aging: evidence is approaching the freshness SLO.
    Aging,
    /// Stale: evidence is past the freshness SLO.
    Stale,
    /// Expired: evidence is no longer valid.
    Expired,
    /// Unverified: no freshness reading is available yet.
    Unverified,
}

impl M5EvidenceFreshnessBadge {
    /// Every evidence-freshness value, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Fresh,
        Self::Recent,
        Self::Aging,
        Self::Stale,
        Self::Expired,
        Self::Unverified,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Recent => "recent",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Unverified => "unverified",
        }
    }
}

/// Controlled lifecycle badge value — the lifecycle stage of a thing, so a
/// lifecycle badge never leaves the stage implicit and never stands in for a
/// channel or support class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LifecycleBadge {
    /// Stable.
    Stable,
    /// Beta.
    Beta,
    /// Preview.
    Preview,
    /// Experimental.
    Experimental,
    /// Maintenance (still supported, no new work).
    Maintenance,
    /// End-of-life.
    EndOfLife,
}

impl M5LifecycleBadge {
    /// Every lifecycle value, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Stable,
        Self::Beta,
        Self::Preview,
        Self::Experimental,
        Self::Maintenance,
        Self::EndOfLife,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Maintenance => "maintenance",
            Self::EndOfLife => "end_of_life",
        }
    }
}

/// Controlled channel badge value — which release channel a thing rides, so a
/// channel badge never leaves the channel implicit and never implies a support
/// class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ChannelBadge {
    /// The stable channel.
    StableChannel,
    /// The beta channel.
    BetaChannel,
    /// The nightly channel.
    NightlyChannel,
    /// The edge / canary channel.
    EdgeChannel,
    /// The long-term-support channel.
    LtsChannel,
    /// A custom / private channel.
    CustomChannel,
}

impl M5ChannelBadge {
    /// Every channel value, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StableChannel,
        Self::BetaChannel,
        Self::NightlyChannel,
        Self::EdgeChannel,
        Self::LtsChannel,
        Self::CustomChannel,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableChannel => "stable_channel",
            Self::BetaChannel => "beta_channel",
            Self::NightlyChannel => "nightly_channel",
            Self::EdgeChannel => "edge_channel",
            Self::LtsChannel => "lts_channel",
            Self::CustomChannel => "custom_channel",
        }
    }
}

/// Controlled deployment-scope badge value — where a thing runs / is available,
/// so a deployment-scope badge never leaves the scope implicit and never implies
/// an experimental or lower lifecycle stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentScopeBadge {
    /// Desktop-only (native desktop app scope).
    DesktopOnly,
    /// The local open-source scope.
    LocalOssScope,
    /// The self-hosted scope.
    SelfHostedScope,
    /// The managed scope.
    ManagedScope,
    /// The air-gapped scope.
    AirGappedScope,
    /// The mirror / offline scope.
    MirrorOfflineScope,
}

impl M5DeploymentScopeBadge {
    /// Every deployment-scope value, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DesktopOnly,
        Self::LocalOssScope,
        Self::SelfHostedScope,
        Self::ManagedScope,
        Self::AirGappedScope,
        Self::MirrorOfflineScope,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopOnly => "desktop_only",
            Self::LocalOssScope => "local_oss_scope",
            Self::SelfHostedScope => "self_hosted_scope",
            Self::ManagedScope => "managed_scope",
            Self::AirGappedScope => "air_gapped_scope",
            Self::MirrorOfflineScope => "mirror_offline_scope",
        }
    }
}

/// Controlled compatibility-state badge value — how compatible a thing is with
/// the host, so a compatibility badge never hides skew or a required migration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompatibilityStateBadge {
    /// Compatible with the current host.
    Compatible,
    /// Minor version skew, still compatible.
    MinorSkew,
    /// Major version skew, degraded.
    MajorSkew,
    /// Incompatible with the current host.
    Incompatible,
    /// A migration is required for compatibility.
    MigrationRequired,
    /// Compatibility is not yet evaluated.
    CompatibilityUnknown,
}

impl M5CompatibilityStateBadge {
    /// Every compatibility-state value, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Compatible,
        Self::MinorSkew,
        Self::MajorSkew,
        Self::Incompatible,
        Self::MigrationRequired,
        Self::CompatibilityUnknown,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compatible => "compatible",
            Self::MinorSkew => "minor_skew",
            Self::MajorSkew => "major_skew",
            Self::Incompatible => "incompatible",
            Self::MigrationRequired => "migration_required",
            Self::CompatibilityUnknown => "compatibility_unknown",
        }
    }
}

/// A mandatory field of a badge's explanation drawer. Every badge, on every
/// surface, must be able to open a drawer carrying at least the three
/// [`M5BadgeExplanationField::MANDATORY`] fields so a badge is a compact contract
/// with an explanation rather than an unexplained decoration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeExplanationField {
    /// What the badge value means in plain language.
    WhatItMeans,
    /// Why the badge is shown here / now.
    WhyShown,
    /// What would change the badge value.
    WhatChangesIt,
    /// The evidence source behind the badge.
    EvidenceSource,
    /// How to improve / resolve the badge value.
    HowToImprove,
    /// When the badge value was last evaluated.
    LastEvaluated,
}

impl M5BadgeExplanationField {
    /// Every explanation field, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::WhatItMeans,
        Self::WhyShown,
        Self::WhatChangesIt,
        Self::EvidenceSource,
        Self::HowToImprove,
        Self::LastEvaluated,
    ];

    /// The three explanation fields every badge drawer must carry.
    pub const MANDATORY: [Self; 3] = [Self::WhatItMeans, Self::WhyShown, Self::WhatChangesIt];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WhatItMeans => "what_it_means",
            Self::WhyShown => "why_shown",
            Self::WhatChangesIt => "what_changes_it",
            Self::EvidenceSource => "evidence_source",
            Self::HowToImprove => "how_to_improve",
            Self::LastEvaluated => "last_evaluated",
        }
    }
}

/// An axis-separation rule: a forbidden implication one badge family must never
/// make about another. These are the acceptance-criteria comparison rules that
/// keep the badge families separate — "Certified does not mean Fresh",
/// "Desktop-only does not mean Experimental", and so on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeAxisSeparationRule {
    /// A support-class value never implies an evidence-freshness value.
    SupportClassDoesNotImplyFreshness,
    /// A deployment-scope value never implies a lifecycle value.
    DeploymentScopeDoesNotImplyLifecycle,
    /// A lifecycle value never implies a channel value.
    LifecycleDoesNotImplyChannel,
    /// A channel value never implies a support-class value.
    ChannelDoesNotImplySupportClass,
    /// A compatibility-state value never implies a support-class value.
    CompatibilityDoesNotImplySupportClass,
    /// An evidence-freshness value never implies a compatibility-state value.
    FreshnessDoesNotImplyCompatibility,
}

impl M5BadgeAxisSeparationRule {
    /// Every axis-separation rule, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SupportClassDoesNotImplyFreshness,
        Self::DeploymentScopeDoesNotImplyLifecycle,
        Self::LifecycleDoesNotImplyChannel,
        Self::ChannelDoesNotImplySupportClass,
        Self::CompatibilityDoesNotImplySupportClass,
        Self::FreshnessDoesNotImplyCompatibility,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportClassDoesNotImplyFreshness => "support_class_does_not_imply_freshness",
            Self::DeploymentScopeDoesNotImplyLifecycle => {
                "deployment_scope_does_not_imply_lifecycle"
            }
            Self::LifecycleDoesNotImplyChannel => "lifecycle_does_not_imply_channel",
            Self::ChannelDoesNotImplySupportClass => "channel_does_not_imply_support_class",
            Self::CompatibilityDoesNotImplySupportClass => {
                "compatibility_does_not_imply_support_class"
            }
            Self::FreshnessDoesNotImplyCompatibility => "freshness_does_not_imply_compatibility",
        }
    }
}

/// Claimed M5 surface family that renders / consumes a badge. No badge may invent
/// a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeSurfaceFamily {
    /// The marketplace surface.
    Marketplace,
    /// The Help / Docs surface.
    HelpDocs,
    /// The Settings surface.
    Settings,
    /// The onboarding surface.
    Onboarding,
    /// The diagnostics surface.
    Diagnostics,
    /// The runtime surface.
    Runtime,
    /// The exported-evidence surface.
    ExportedEvidence,
}

impl M5BadgeSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Marketplace,
        Self::HelpDocs,
        Self::Settings,
        Self::Onboarding,
        Self::Diagnostics,
        Self::Runtime,
        Self::ExportedEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Marketplace => "marketplace",
            Self::HelpDocs => "help_docs",
            Self::Settings => "settings",
            Self::Onboarding => "onboarding",
            Self::Diagnostics => "diagnostics",
            Self::Runtime => "runtime",
            Self::ExportedEvidence => "exported_evidence",
        }
    }
}

/// Deployment line a badge must survive with the same meaning, so a badge's
/// meaning never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5DeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a badge's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeConsumerSurface {
    /// The marketplace UI.
    MarketplaceUi,
    /// The Help / About surface.
    HelpAbout,
    /// The Settings UI.
    SettingsUi,
    /// The onboarding flow.
    OnboardingFlow,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The docs portal.
    DocsPortal,
    /// The evaluation pack.
    EvaluationPack,
    /// The support export.
    SupportExport,
    /// The CLI inspect / headless surface.
    CliInspect,
    /// The general product UI.
    ProductUi,
}

impl M5BadgeConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::MarketplaceUi,
        Self::HelpAbout,
        Self::SettingsUi,
        Self::OnboardingFlow,
        Self::DiagnosticsSurface,
        Self::DocsPortal,
        Self::EvaluationPack,
        Self::SupportExport,
        Self::CliInspect,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MarketplaceUi => "marketplace_ui",
            Self::HelpAbout => "help_about",
            Self::SettingsUi => "settings_ui",
            Self::OnboardingFlow => "onboarding_flow",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::DocsPortal => "docs_portal",
            Self::EvaluationPack => "evaluation_pack",
            Self::SupportExport => "support_export",
            Self::CliInspect => "cli_inspect",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every badge must offer so no badge truth is
/// hover-only, pointer-only, or color-encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (with its axis name and value).
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Meaning is never encoded by color alone.
    NonColorEncoded,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5BadgeAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::NonColorEncoded,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::NonColorEncoded => "non_color_encoded",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed badge must be able to show. The first three are hard
/// requirements on every badge; the remaining three close the acceptance-criteria
/// ambiguity about the explanation drawer, evidence source, and filter key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeRequiredLabel {
    /// The badge's stable identity / what object it labels.
    Identity,
    /// The badge's current typed value.
    ValueState,
    /// The badge's axis name, so a badge is never mistaken for another axis.
    AxisName,
    /// The explanation-drawer affordance.
    ExplanationDrawer,
    /// The evidence source behind the badge's claim.
    EvidenceSource,
    /// The separately-filterable key for this axis.
    FilterKey,
}

impl M5BadgeRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::ValueState,
        Self::AxisName,
        Self::ExplanationDrawer,
        Self::EvidenceSource,
        Self::FilterKey,
    ];

    /// The three labels every claimed badge must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::ValueState, Self::AxisName];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::ValueState => "value_state",
            Self::AxisName => "axis_name",
            Self::ExplanationDrawer => "explanation_drawer",
            Self::EvidenceSource => "evidence_source",
            Self::FilterKey => "filter_key",
        }
    }
}

/// Qualification class for an M5 badge-family row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeQualificationClass {
    /// Badge family qualifies for the Stable claim.
    Stable,
    /// Badge family is narrowed to Beta.
    Beta,
    /// Badge family is narrowed to Preview.
    Preview,
    /// Badge family is experimental and not claimed.
    Experimental,
    /// Badge family is unavailable on this build.
    Unavailable,
    /// Badge family is held pending upstream resolution.
    Held,
}

impl M5BadgeQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the badge family may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a badge family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BadgeDowngradeTrigger {
    /// A support-class badge left its value unstated.
    SupportClassValueUnstated,
    /// An evidence-freshness badge hid its freshness reading.
    EvidenceFreshnessHidden,
    /// A lifecycle badge left its stage unstated.
    LifecycleValueUnstated,
    /// A channel badge left its channel unstated.
    ChannelValueUnstated,
    /// A deployment-scope badge left its scope unstated.
    DeploymentScopeUnstated,
    /// A compatibility-state badge left its state unstated.
    CompatibilityStateUnstated,
    /// A badge could not open its explanation drawer.
    ExplanationDrawerMissing,
    /// A badge merged its axis into another badge axis.
    AxisMergedIntoAnother,
    /// A badge implied freshness from its support class (or any cross-axis
    /// implication).
    FreshnessImpliedFromSupportClass,
    /// A badge dropped its separately-filterable key.
    FilterKeyDropped,
    /// Exported evidence lost the badge's meaning.
    ExportLostBadgeMeaning,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5BadgeDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::SupportClassValueUnstated,
        Self::EvidenceFreshnessHidden,
        Self::LifecycleValueUnstated,
        Self::ChannelValueUnstated,
        Self::DeploymentScopeUnstated,
        Self::CompatibilityStateUnstated,
        Self::ExplanationDrawerMissing,
        Self::AxisMergedIntoAnother,
        Self::FreshnessImpliedFromSupportClass,
        Self::FilterKeyDropped,
        Self::ExportLostBadgeMeaning,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportClassValueUnstated => "support_class_value_unstated",
            Self::EvidenceFreshnessHidden => "evidence_freshness_hidden",
            Self::LifecycleValueUnstated => "lifecycle_value_unstated",
            Self::ChannelValueUnstated => "channel_value_unstated",
            Self::DeploymentScopeUnstated => "deployment_scope_unstated",
            Self::CompatibilityStateUnstated => "compatibility_state_unstated",
            Self::ExplanationDrawerMissing => "explanation_drawer_missing",
            Self::AxisMergedIntoAnother => "axis_merged_into_another",
            Self::FreshnessImpliedFromSupportClass => "freshness_implied_from_support_class",
            Self::FilterKeyDropped => "filter_key_dropped",
            Self::ExportLostBadgeMeaning => "export_lost_badge_meaning",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed badge family bound to the badge value
/// vocabulary and surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeFamilyRow {
    /// Governed badge family.
    pub badge_family: M5BadgeFamily,
    /// Qualification class earned by this badge family.
    pub qualification: M5BadgeQualificationClass,
    /// Owner role accountable for keeping this badge family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this badge.
    pub surface_families: Vec<M5BadgeSurfaceFamily>,
    /// Deployment lines this badge keeps the same meaning across.
    pub deployment_lines: Vec<M5DeploymentLine>,
    /// Mandatory labels this badge must be able to show (must include the three
    /// [`M5BadgeRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5BadgeRequiredLabel>,
    /// Explanation-drawer fields this badge can open (must include the three
    /// [`M5BadgeExplanationField::MANDATORY`] fields).
    pub explanation_fields: Vec<M5BadgeExplanationField>,
    /// Support-class values this badge names (support-class only).
    pub support_class_values: Vec<M5SupportClassBadge>,
    /// Evidence-freshness values this badge names (evidence-freshness only).
    pub evidence_freshness_values: Vec<M5EvidenceFreshnessBadge>,
    /// Lifecycle values this badge names (lifecycle only).
    pub lifecycle_values: Vec<M5LifecycleBadge>,
    /// Channel values this badge names (channel only).
    pub channel_values: Vec<M5ChannelBadge>,
    /// Deployment-scope values this badge names (deployment-scope only).
    pub deployment_scope_values: Vec<M5DeploymentScopeBadge>,
    /// Compatibility-state values this badge names (compatibility-state only).
    pub compatibility_state_values: Vec<M5CompatibilityStateBadge>,
    /// Non-visual accessibility routes this badge offers.
    pub accessibility_routes: Vec<M5BadgeAccessibilityRoute>,
    /// Subsystems that consume this badge's projection.
    pub consumer_surfaces: Vec<M5BadgeConsumerSurface>,
    /// Downgrade triggers that apply to this badge.
    pub downgrade_triggers: Vec<M5BadgeDowngradeTrigger>,
    /// Proof packet refs that keep this badge current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this badge.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this badge never collapses two axes into one overloaded
    /// pill. MUST be `false`.
    pub collapses_multiple_axes_into_one_pill: bool,
    /// Hard invariant: this badge never implies evidence freshness from its
    /// support class. MUST be `false`.
    pub implies_freshness_from_support_class: bool,
    /// Hard invariant: this badge never implies a lifecycle stage from its
    /// deployment scope. MUST be `false`.
    pub implies_lifecycle_from_deployment_scope: bool,
    /// Hard invariant: this badge never lets exported evidence lose its meaning.
    /// MUST be `false`.
    pub drops_badge_meaning_in_export: bool,
}

impl M5BadgeFamilyRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5BadgeRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5BadgeRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row declares all mandatory explanation-drawer fields.
    fn declares_mandatory_explanation_fields(&self) -> bool {
        let present: BTreeSet<M5BadgeExplanationField> =
            self.explanation_fields.iter().copied().collect();
        M5BadgeExplanationField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_multiple_axes_into_one_pill
            && !self.implies_freshness_from_support_class
            && !self.implies_lifecycle_from_deployment_scope
            && !self.drops_badge_meaning_in_export
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeVocabularySet {
    /// Badge-family tokens.
    pub badge_families: Vec<String>,
    /// Support-class-value tokens.
    pub support_class_values: Vec<String>,
    /// Evidence-freshness-value tokens.
    pub evidence_freshness_values: Vec<String>,
    /// Lifecycle-value tokens.
    pub lifecycle_values: Vec<String>,
    /// Channel-value tokens.
    pub channel_values: Vec<String>,
    /// Deployment-scope-value tokens.
    pub deployment_scope_values: Vec<String>,
    /// Compatibility-state-value tokens.
    pub compatibility_state_values: Vec<String>,
    /// Explanation-field tokens.
    pub explanation_fields: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5BadgeVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            badge_families: tokens(&M5BadgeFamily::ALL, |v| v.as_str()),
            support_class_values: tokens(&M5SupportClassBadge::ALL, |v| v.as_str()),
            evidence_freshness_values: tokens(&M5EvidenceFreshnessBadge::ALL, |v| v.as_str()),
            lifecycle_values: tokens(&M5LifecycleBadge::ALL, |v| v.as_str()),
            channel_values: tokens(&M5ChannelBadge::ALL, |v| v.as_str()),
            deployment_scope_values: tokens(&M5DeploymentScopeBadge::ALL, |v| v.as_str()),
            compatibility_state_values: tokens(&M5CompatibilityStateBadge::ALL, |v| v.as_str()),
            explanation_fields: tokens(&M5BadgeExplanationField::ALL, |v| v.as_str()),
            surface_families: tokens(&M5BadgeSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5DeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5BadgeConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5BadgeAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5BadgeRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5BadgeGovernanceReview {
    /// Each badge family shows its own value with a stable label.
    pub each_family_shows_its_own_value: bool,
    /// Every badge can open its explanation drawer.
    pub every_badge_opens_explanation_drawer: bool,
    /// Support class never implies evidence freshness.
    pub support_class_never_implies_freshness: bool,
    /// Deployment scope never implies a lifecycle stage.
    pub deployment_scope_never_implies_lifecycle: bool,
    /// No badge collapses two axes into one overloaded pill.
    pub no_badge_collapses_two_axes: bool,
    /// Every badge is separately filterable.
    pub every_badge_is_separately_filterable: bool,
    /// Exported evidence never loses badge meaning.
    pub exported_evidence_keeps_badge_meaning: bool,
    /// No badge invents a second badge grammar.
    pub no_badge_invents_second_grammar: bool,
    /// Every badge keeps the same meaning across every deployment line.
    pub every_badge_declares_deployment_lines: bool,
    /// Every badge declares a non-visual accessibility route.
    pub every_badge_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel badge vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeConsumerProjection {
    /// Marketplace and Help surfaces consume the shared badge vocabulary.
    pub marketplace_and_help_surfaces_consume_matrix: bool,
    /// Settings and onboarding surfaces consume the shared badge vocabulary.
    pub settings_and_onboarding_surfaces_consume_matrix: bool,
    /// Diagnostics and runtime surfaces consume the shared badge vocabulary.
    pub diagnostics_and_runtime_surfaces_consume_matrix: bool,
    /// Filters read one canonical badge source per axis.
    pub filters_read_single_source_per_axis: bool,
    /// Support / export reads a single canonical badge source.
    pub support_export_reads_single_source: bool,
    /// Docs / help read a single canonical badge source.
    pub docs_help_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the badge family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the badge-family lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting badge-family audit for the lane.
    pub badge_family_audit_ref: String,
    /// True when support/export parity is required for every badge.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every badge.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5BadgeFamilyMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5BadgeFamilyMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge-family rows.
    pub badge_rows: Vec<M5BadgeFamilyRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BadgeVocabularySet,
    /// Frozen axis-separation rules.
    pub axis_separation_rules: Vec<String>,
    /// Governance-review block.
    pub governance_review: M5BadgeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BadgeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BadgeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BadgeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 badge-family matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5BadgeFamilyMatrixPacket {
    /// Record kind; must equal [`M5_BADGE_FAMILY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_BADGE_FAMILY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Badge-family rows.
    pub badge_rows: Vec<M5BadgeFamilyRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5BadgeVocabularySet,
    /// Frozen axis-separation rules.
    pub axis_separation_rules: Vec<String>,
    /// Governance-review block.
    pub governance_review: M5BadgeGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5BadgeConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5BadgeProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5BadgeReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5BadgeFamilyMatrixPacket {
    /// Builds an M5 badge-family matrix packet from stable-lane input.
    pub fn new(input: M5BadgeFamilyMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_BADGE_FAMILY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_BADGE_FAMILY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            badge_rows: input.badge_rows,
            vocabulary_set: input.vocabulary_set,
            axis_separation_rules: input.axis_separation_rules,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 badge-family matrix invariants.
    pub fn validate(&self) -> Vec<M5BadgeFamilyMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_BADGE_FAMILY_MATRIX_RECORD_KIND {
            violations.push(M5BadgeFamilyMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_BADGE_FAMILY_MATRIX_SCHEMA_VERSION {
            violations.push(M5BadgeFamilyMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5BadgeFamilyMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_axis_separation_rules(self, &mut violations);
        validate_badge_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 badge family matrix packet serializes"),
        ) {
            violations.push(M5BadgeFamilyMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 badge family matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed badge
    /// family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "badge_family,qualification,owner,surface_families,deployment_lines,required_labels,explanation_fields,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.badge_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.badge_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.explanation_fields, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_families = self
            .badge_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Support-Class, Evidence-Freshness, Lifecycle, Channel, Deployment-Scope, Compatibility-State, and Explanation-Drawer Badge Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Badge families: {} ({} stable)\n",
            self.badge_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Axis-separation rules: {}\n",
            self.axis_separation_rules.join(", ")
        ));
        out.push_str(&format!(
            "- Explanation fields: {}\n",
            self.vocabulary_set.explanation_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Badge families\n\n");
        for row in &self.badge_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.badge_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Explanation fields: {}\n",
                row.explanation_fields
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 badge-family matrix export.
#[derive(Debug)]
pub enum M5BadgeFamilyMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5BadgeFamilyMatrixViolation>),
}

impl fmt::Display for M5BadgeFamilyMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 badge family matrix export parse failed: {error}"
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
                    "m5 badge family matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5BadgeFamilyMatrixArtifactError {}

/// Validation failures emitted by [`M5BadgeFamilyMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5BadgeFamilyMatrixViolation {
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
    /// The frozen axis-separation rules drifted from the canonical list.
    AxisSeparationRulesDrift,
    /// A required governed badge family is missing from the matrix.
    RequiredBadgeFamilyMissing,
    /// A badge row is incomplete.
    BadgeRowIncomplete,
    /// A badge row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A badge row omits one of the mandatory explanation-drawer fields.
    ExplanationDrawerIncomplete,
    /// A support-class badge declares no support-class values.
    SupportClassValueMissing,
    /// An evidence-freshness badge declares no freshness values.
    EvidenceFreshnessValueMissing,
    /// A lifecycle badge declares no lifecycle values.
    LifecycleValueMissing,
    /// A channel badge declares no channel values.
    ChannelValueMissing,
    /// A deployment-scope badge declares no deployment-scope values.
    DeploymentScopeValueMissing,
    /// A compatibility-state badge declares no compatibility-state values.
    CompatibilityStateValueMissing,
    /// A badge declares no surface families.
    SurfaceFamilyMissing,
    /// A badge declares no deployment lines.
    DeploymentLineMissing,
    /// A badge declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A badge declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A badge declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A badge claiming Stable is missing required proof packet refs.
    StableBadgeMissingProof,
    /// A badge violates a hard invariant (collapsed axes, freshness implied from
    /// support class, lifecycle implied from deployment scope, or dropped export
    /// meaning).
    BadgeInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5BadgeFamilyMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::AxisSeparationRulesDrift => "axis_separation_rules_drift",
            Self::RequiredBadgeFamilyMissing => "required_badge_family_missing",
            Self::BadgeRowIncomplete => "badge_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ExplanationDrawerIncomplete => "explanation_drawer_incomplete",
            Self::SupportClassValueMissing => "support_class_value_missing",
            Self::EvidenceFreshnessValueMissing => "evidence_freshness_value_missing",
            Self::LifecycleValueMissing => "lifecycle_value_missing",
            Self::ChannelValueMissing => "channel_value_missing",
            Self::DeploymentScopeValueMissing => "deployment_scope_value_missing",
            Self::CompatibilityStateValueMissing => "compatibility_state_value_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableBadgeMissingProof => "stable_badge_missing_proof",
            Self::BadgeInvariantViolated => "badge_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 badge-family matrix export.
pub fn current_stable_m5_badge_family_matrix_export(
) -> Result<M5BadgeFamilyMatrixPacket, M5BadgeFamilyMatrixArtifactError> {
    let packet: M5BadgeFamilyMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-badge-family-proof/support_export.json"
    )))
    .map_err(M5BadgeFamilyMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5BadgeFamilyMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5BadgeFamilyMatrixPacket,
    violations: &mut Vec<M5BadgeFamilyMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_BADGE_FAMILY_SCHEMA_REF,
        M5_BADGE_FAMILY_DOC_REF,
        M5_BADGE_FAMILY_SUPPORT_CLASS_REF,
        M5_BADGE_FAMILY_FRESHNESS_REF,
        M5_BADGE_FAMILY_LIFECYCLE_REF,
        M5_BADGE_FAMILY_COMPATIBILITY_REF,
        M5_BADGE_FAMILY_BADGE_VOCABULARY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5BadgeFamilyMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5BadgeFamilyMatrixPacket,
    violations: &mut Vec<M5BadgeFamilyMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5BadgeFamilyMatrixViolation::VocabularySetDrift);
    }
}

fn validate_axis_separation_rules(
    packet: &M5BadgeFamilyMatrixPacket,
    violations: &mut Vec<M5BadgeFamilyMatrixViolation>,
) {
    let canonical = tokens(&M5BadgeAxisSeparationRule::ALL, |v| v.as_str());
    if packet.axis_separation_rules != canonical {
        violations.push(M5BadgeFamilyMatrixViolation::AxisSeparationRulesDrift);
    }
}

fn validate_badge_rows(
    packet: &M5BadgeFamilyMatrixPacket,
    violations: &mut Vec<M5BadgeFamilyMatrixViolation>,
) {
    let present: BTreeSet<M5BadgeFamily> = packet
        .badge_rows
        .iter()
        .map(|row| row.badge_family)
        .collect();
    for required in M5BadgeFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5BadgeFamilyMatrixViolation::RequiredBadgeFamilyMissing);
            return;
        }
    }

    for row in &packet.badge_rows {
        let family = row.badge_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
            || row.explanation_fields.is_empty()
        {
            violations.push(M5BadgeFamilyMatrixViolation::BadgeRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5BadgeFamilyMatrixViolation::MandatoryLabelMissing);
        }
        if !row.declares_mandatory_explanation_fields() {
            violations.push(M5BadgeFamilyMatrixViolation::ExplanationDrawerIncomplete);
        }
        if family.is_support_class() && row.support_class_values.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::SupportClassValueMissing);
        }
        if family.is_evidence_freshness() && row.evidence_freshness_values.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::EvidenceFreshnessValueMissing);
        }
        if family.is_lifecycle() && row.lifecycle_values.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::LifecycleValueMissing);
        }
        if family.is_channel() && row.channel_values.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::ChannelValueMissing);
        }
        if family.is_deployment_scope() && row.deployment_scope_values.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::DeploymentScopeValueMissing);
        }
        if family.is_compatibility_state() && row.compatibility_state_values.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::CompatibilityStateValueMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5BadgeFamilyMatrixViolation::StableBadgeMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5BadgeFamilyMatrixViolation::BadgeInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5BadgeFamilyMatrixPacket,
    violations: &mut Vec<M5BadgeFamilyMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.each_family_shows_its_own_value,
        review.every_badge_opens_explanation_drawer,
        review.support_class_never_implies_freshness,
        review.deployment_scope_never_implies_lifecycle,
        review.no_badge_collapses_two_axes,
        review.every_badge_is_separately_filterable,
        review.exported_evidence_keeps_badge_meaning,
        review.no_badge_invents_second_grammar,
        review.every_badge_declares_deployment_lines,
        review.every_badge_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5BadgeFamilyMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5BadgeFamilyMatrixPacket,
    violations: &mut Vec<M5BadgeFamilyMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.marketplace_and_help_surfaces_consume_matrix,
        projection.settings_and_onboarding_surfaces_consume_matrix,
        projection.diagnostics_and_runtime_surfaces_consume_matrix,
        projection.filters_read_single_source_per_axis,
        projection.support_export_reads_single_source,
        projection.docs_help_read_single_source,
    ] {
        if !ok {
            violations.push(M5BadgeFamilyMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5BadgeFamilyMatrixPacket,
    violations: &mut Vec<M5BadgeFamilyMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5BadgeFamilyMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5BadgeFamilyMatrixPacket,
    violations: &mut Vec<M5BadgeFamilyMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.badge_family_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5BadgeFamilyMatrixViolation::ReleasePostureIncomplete);
    }
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
