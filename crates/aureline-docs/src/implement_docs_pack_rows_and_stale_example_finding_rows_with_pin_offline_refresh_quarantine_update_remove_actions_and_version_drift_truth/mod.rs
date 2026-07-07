//! Two reusable M5 docs-lifecycle primitives: the docs-pack row and the
//! stale-example finding row, projected the same way across every claimed M5
//! docs-manager, help-pack, onboarding, AI-context, and support surface a user
//! reaches when they manage documentation packs or inspect example drift.
//!
//! Aureline's frozen docs-browser component matrix
//! ([`crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix`])
//! names the docs-pack row and the stale-example finding row as two governed
//! component families and freezes their controlled vocabulary — the pack states, the
//! stale-example statuses, the source providers, the freshness states, the corpus
//! classes, the version scopes, the docs surface families, the deployment lines, the
//! consumer surfaces, the accessibility routes, the qualification classes, and the
//! downgrade triggers. This module *implements* those two contracts as reusable
//! primitives so a user can tell — from the row alone — whether a docs pack is pinned,
//! stale, mirrored, quarantined, update-overdue, or current before trusting it, and can
//! turn "docs may be old" into an actionable stale-example finding anchored to the exact
//! snippet, command, or config shape that drifted, instead of that truth collapsing into
//! one generic warning by docs manager, help pack, onboarding, AI context, or support
//! evidence path.
//!
//! The module has three halves:
//!
//! 1. A pack resolver — [`resolve_docs_pack_row`] — that takes one pack's identity,
//!    selected scope, size/count, signer/source, pin/mirror/offline/quarantine state,
//!    refresh time, and verification state, and produces one [`M5ResolvedDocsPackRow`]
//!    carrying the derived pack trust posture (pinned-current versus mirror-served
//!    versus offline-only versus update-overdue versus stale-needs-refresh versus
//!    quarantined-untrusted versus verification-unverified) — never showing a
//!    quarantined, stale, or mirrored pack as freely trusted or live — plus the exact
//!    pin/offline/refresh/quarantine/update/remove actions the pack allows.
//! 2. A finding resolver — [`resolve_stale_example_row`] — that takes one stale-example
//!    finding's title, the affected snippet/command/config anchor, its stale-example
//!    status, the documented and current versions, and produces one
//!    [`M5ResolvedDocsStaleExampleRow`] carrying the derived example drift posture and
//!    the concrete compare / open-current-source / suppress actions — never showing a
//!    drifted or unverified example as current.
//! 3. A parity matrix — [`M5DocsPackFindingPrimitivePacket`] — that binds one row per
//!    claimed M5 pack/finding consumer (the docs-pack manager, the help pack panel, the
//!    onboarding pack step, the AI pack context, and the support pack evidence) to the
//!    shared pack/finding anatomy, the same trust postures, drift postures, pack states,
//!    stale-example statuses, actions, export fields, and non-visual accessibility
//!    routes, so the pack-lifecycle and example-drift vocabulary stays identical across
//!    docs, help, onboarding, AI, and support.
//!
//! The pack state ([`M5DocsPackState`]), stale-example status
//! ([`M5DocsStaleExampleStatus`]), corpus class ([`M5DocsCorpusClass`]), version scope
//! ([`M5DocsVersionScope`]), source provider ([`M5DocsSourceProvider`]), freshness state
//! ([`M5DocsFreshnessState`]), docs surface family ([`M5DocsSurfaceFamily`]), deployment
//! line ([`M5DocsDeploymentLine`]), consumer surface ([`M5DocsConsumerSurface`]),
//! accessibility route ([`M5DocsAccessibilityRoute`]), qualification class
//! ([`M5DocsQualificationClass`]), and downgrade trigger ([`M5DocsDowngradeTrigger`]) are
//! reused verbatim from the frozen docs-browser component matrix. This module mints new
//! vocabulary only for what that matrix left implicit about the two rows themselves:
//! their consumers, their anatomy parts, their derived trust and drift postures, their
//! verification states, their anchor kinds, their actions, and their export fields. No
//! M5 docs surface invents a second pack-row or stale-example grammar.
//!
//! Raw URLs, raw tokens, credentials, private endpoints, pack payloads, and example
//! bodies stay outside the support boundary; every pack name, signer, refresh time,
//! affected anchor, cited version, and action target is carried only as an opaque,
//! export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_pack_finding_primitive_ai_pack_context_preview_narrowed,
    seeded_m5_pack_finding_primitive_onboarding_pack_beta_narrowed,
    seeded_m5_pack_finding_primitive_packet, M5_DOCS_PACK_FINDING_PRIMITIVE_PACKET_ID,
};

// The pack state, stale-example status, corpus class, version scope, source provider,
// freshness state, docs surface family, deployment line, consumer surface,
// accessibility routes, qualification classes, and downgrade triggers are frozen once,
// in the docs-browser component matrix. These primitives reuse them verbatim so they
// never invent a parallel pack-row or stale-example vocabulary.
pub use crate::freeze_the_m5_docs_search_bar_result_row_symbol_reference_card_source_badge_docs_pack_row_and_handoff_banner_component_matrix::{
    M5DocsAccessibilityRoute, M5DocsConsumerSurface, M5DocsCorpusClass, M5DocsDeploymentLine,
    M5DocsDowngradeTrigger, M5DocsFreshnessState, M5DocsPackState, M5DocsQualificationClass,
    M5DocsSourceProvider, M5DocsStaleExampleStatus, M5DocsSurfaceFamily, M5DocsVersionScope,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DocsPackFindingPrimitivePacket`].
pub const M5_DOCS_PACK_FINDING_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_docs_pack_rows_and_stale_example_finding_rows_with_pin_offline_refresh_quarantine_update_remove_actions_and_version_drift_truth";

/// Schema version for M5 docs-pack-row / stale-example-finding-row primitive records.
pub const M5_DOCS_PACK_FINDING_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the pack-row / stale-example-finding-row boundary schema.
pub const M5_DOCS_PACK_FINDING_SCHEMA_REF: &str =
    "schemas/docs/m5-docs-pack-row-and-stale-example-finding-row-primitive.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DOCS_PACK_FINDING_DOC_REF: &str =
    "docs/docs/m5/implement_docs_pack_rows_and_stale_example_finding_rows_with_pin_offline_refresh_quarantine_update_remove_actions_and_version_drift_truth.md";

/// Repo-relative path of the frozen docs-browser component matrix these primitives
/// narrow from.
pub const M5_DOCS_PACK_FINDING_COMPONENT_MATRIX_REF: &str =
    "schemas/docs/freeze-the-m5-docs-search-bar-result-row-symbol-reference-card-source-badge-docs-pack-row-and-handoff-banner-component-matrix.schema.json";

/// Repo-relative path of the stable docs-source/result contract these primitives bind
/// against.
pub const M5_DOCS_PACK_FINDING_SOURCE_RESULT_REF: &str =
    "schemas/docs/stable_docs_source_result_pack_and_citation.schema.json";

/// Repo-relative path of the docs-source precedence / ranking-parity contract these
/// primitives keep source/version truth consistent with.
pub const M5_DOCS_PACK_FINDING_SOURCE_PRECEDENCE_REF: &str =
    "schemas/docs/add-docs-source-precedence-and-ranking-parity-across-search-hover-onboarding-and-ai-context.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DOCS_PACK_FINDING_FIXTURE_DIR: &str =
    "fixtures/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DOCS_PACK_FINDING_ARTIFACT_REF: &str =
    "artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DOCS_PACK_FINDING_CSV_REF: &str =
    "artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DOCS_PACK_FINDING_REPORT_REF: &str =
    "artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive.md";

/// One claimed M5 pack/finding consumer that renders the shared docs-pack row and the
/// stale-example finding row. These are the entrypoints the acceptance criteria name —
/// the docs-pack manager, the help pack panel, the onboarding pack step, the AI pack
/// context, and the support pack evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsPackConsumerSurface {
    /// The docs-pack manager.
    DocsPackManager,
    /// The help / about pack panel.
    HelpPackPanel,
    /// The onboarding pack step.
    OnboardingPackStep,
    /// The AI pack-context panel.
    AiPackContext,
    /// The support / evidence pack view.
    SupportPackEvidence,
}

impl M5DocsPackConsumerSurface {
    /// Every claimed pack/finding consumer, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DocsPackManager,
        Self::HelpPackPanel,
        Self::OnboardingPackStep,
        Self::AiPackContext,
        Self::SupportPackEvidence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsPackManager => "docs_pack_manager",
            Self::HelpPackPanel => "help_pack_panel",
            Self::OnboardingPackStep => "onboarding_pack_step",
            Self::AiPackContext => "ai_pack_context",
            Self::SupportPackEvidence => "support_pack_evidence",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DocsPackManager => "Docs-Pack Manager",
            Self::HelpPackPanel => "Help Pack Panel",
            Self::OnboardingPackStep => "Onboarding Pack Step",
            Self::AiPackContext => "AI Pack Context",
            Self::SupportPackEvidence => "Support Pack Evidence",
        }
    }
}

/// The declared verification state of a docs pack — whether its signature and checksum
/// have been confirmed, so a pack row never presents an unverified or verification-failed
/// pack as trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsPackVerificationState {
    /// The pack's signature is verified.
    SignatureVerified,
    /// Only the pack's checksum is verified (no trusted signature).
    ChecksumOnly,
    /// The pack is not verified.
    Unverified,
    /// The pack failed verification.
    VerificationFailed,
}

impl M5DocsPackVerificationState {
    /// Every verification state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SignatureVerified,
        Self::ChecksumOnly,
        Self::Unverified,
        Self::VerificationFailed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignatureVerified => "signature_verified",
            Self::ChecksumOnly => "checksum_only",
            Self::Unverified => "unverified",
            Self::VerificationFailed => "verification_failed",
        }
    }

    /// True when the pack carries a trusted signature.
    pub const fn is_signature_verified(self) -> bool {
        matches!(self, Self::SignatureVerified)
    }

    /// True when verification failed outright — the pack must never read as trusted.
    pub const fn is_failed(self) -> bool {
        matches!(self, Self::VerificationFailed)
    }
}

/// The derived docs-pack trust posture — the resolver's honest verdict about how far a
/// pack can be trusted: pinned-and-current, tracking-and-current, mirror-served (not
/// live), offline-only, update-overdue, stale-needs-refresh, quarantined-untrusted, or
/// verification-unverified. A quarantined, stale, mirrored, or offline pack is never
/// shown as freely trusted or live, and these states never collapse into one generic
/// warning.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsPackTrustPosture {
    /// Pinned to an exact version and current.
    PinnedCurrent,
    /// Unpinned, tracking upstream, and current.
    TrackingCurrent,
    /// Served from a mirror, shown explicitly and never as live.
    MirrorServedNotLive,
    /// Available offline only.
    OfflineOnly,
    /// An update is available / overdue.
    UpdateOverdue,
    /// Stale / expired and needs a refresh.
    StaleNeedsRefresh,
    /// Quarantined pending review and not trusted.
    QuarantinedUntrusted,
    /// Verification failed, so the pack is not trusted.
    VerificationUnverified,
}

impl M5DocsPackTrustPosture {
    /// Every trust posture, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PinnedCurrent,
        Self::TrackingCurrent,
        Self::MirrorServedNotLive,
        Self::OfflineOnly,
        Self::UpdateOverdue,
        Self::StaleNeedsRefresh,
        Self::QuarantinedUntrusted,
        Self::VerificationUnverified,
    ];

    /// The distinct pack states the acceptance criteria require to stay explicit rather
    /// than collapse into one generic warning — pinned, mirrored, offline, update-overdue,
    /// stale, and quarantined.
    pub const DISTINCT_STATES: [Self; 6] = [
        Self::PinnedCurrent,
        Self::MirrorServedNotLive,
        Self::OfflineOnly,
        Self::UpdateOverdue,
        Self::StaleNeedsRefresh,
        Self::QuarantinedUntrusted,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinnedCurrent => "pinned_current",
            Self::TrackingCurrent => "tracking_current",
            Self::MirrorServedNotLive => "mirror_served_not_live",
            Self::OfflineOnly => "offline_only",
            Self::UpdateOverdue => "update_overdue",
            Self::StaleNeedsRefresh => "stale_needs_refresh",
            Self::QuarantinedUntrusted => "quarantined_untrusted",
            Self::VerificationUnverified => "verification_unverified",
        }
    }

    /// Color-independent glyph label so the trust cue never relies on color alone.
    pub const fn glyph_label(self) -> &'static str {
        match self {
            Self::PinnedCurrent => "[pinned]",
            Self::TrackingCurrent => "[current]",
            Self::MirrorServedNotLive => "[mirror]",
            Self::OfflineOnly => "[offline]",
            Self::UpdateOverdue => "[update]",
            Self::StaleNeedsRefresh => "[stale]",
            Self::QuarantinedUntrusted => "[quarantined]",
            Self::VerificationUnverified => "[unverified]",
        }
    }

    /// Review-safe phrase for the disclosure headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::PinnedCurrent => "the pack is pinned to an exact version and current",
            Self::TrackingCurrent => "the pack is tracking upstream and current",
            Self::MirrorServedNotLive => "the pack is served from a mirror and is not live",
            Self::OfflineOnly => "the pack is available offline only",
            Self::UpdateOverdue => "an update to the pack is available",
            Self::StaleNeedsRefresh => "the pack is stale and needs a refresh",
            Self::QuarantinedUntrusted => "the pack is quarantined pending review and not trusted",
            Self::VerificationUnverified => "the pack failed verification and is not trusted",
        }
    }

    /// True when the pack reads as trusted-and-current (pinned or tracking).
    pub const fn is_trusted_current(self) -> bool {
        matches!(self, Self::PinnedCurrent | Self::TrackingCurrent)
    }

    /// True when the pack is quarantined.
    pub const fn is_quarantined(self) -> bool {
        matches!(self, Self::QuarantinedUntrusted)
    }
}

/// One action a docs-pack row exposes. The acceptance criteria require pin, offline,
/// refresh, quarantine-review, update, and remove actions to stay attached, plus an
/// always-available export so pack actions keep export parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsPackAction {
    /// Pin the pack to an exact version.
    PinPack,
    /// Unpin the pack.
    UnpinPack,
    /// Refresh the pack from its source.
    RefreshPack,
    /// Take the pack offline.
    TakeOffline,
    /// Review a quarantined pack.
    ReviewQuarantine,
    /// Update the pack.
    UpdatePack,
    /// Remove the pack.
    RemovePack,
    /// Export the pack manifest.
    ExportPackManifest,
}

impl M5DocsPackAction {
    /// Every pack action, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PinPack,
        Self::UnpinPack,
        Self::RefreshPack,
        Self::TakeOffline,
        Self::ReviewQuarantine,
        Self::UpdatePack,
        Self::RemovePack,
        Self::ExportPackManifest,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PinPack => "pin_pack",
            Self::UnpinPack => "unpin_pack",
            Self::RefreshPack => "refresh_pack",
            Self::TakeOffline => "take_offline",
            Self::ReviewQuarantine => "review_quarantine",
            Self::UpdatePack => "update_pack",
            Self::RemovePack => "remove_pack",
            Self::ExportPackManifest => "export_pack_manifest",
        }
    }
}

/// One anatomy part the shared docs-pack row surfaces. The parts in
/// [`M5DocsPackRowAnatomyPart::MANDATORY`] are required on every pack row so a user can
/// see pack identity, scope, source, lifecycle state, verification, and actions before
/// trusting a pack.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsPackRowAnatomyPart {
    /// The pack identity label.
    PackIdentityLabel,
    /// The selected-scope badge.
    SelectedScopeBadge,
    /// The size / item-count meter.
    SizeCountMeter,
    /// The signer / source badge.
    SignerSourceBadge,
    /// The pin / mirror / offline / quarantine state badge.
    PackStateBadge,
    /// The refresh-time label.
    RefreshTimeLabel,
    /// The verification-state badge.
    VerificationBadge,
    /// The pin/offline/refresh/quarantine/update/remove action cluster.
    PackActionCluster,
}

impl M5DocsPackRowAnatomyPart {
    /// Every pack-row anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::PackIdentityLabel,
        Self::SelectedScopeBadge,
        Self::SizeCountMeter,
        Self::SignerSourceBadge,
        Self::PackStateBadge,
        Self::RefreshTimeLabel,
        Self::VerificationBadge,
        Self::PackActionCluster,
    ];

    /// The pack-row anatomy parts every consumer must render.
    pub const MANDATORY: [Self; 6] = [
        Self::PackIdentityLabel,
        Self::SelectedScopeBadge,
        Self::SignerSourceBadge,
        Self::PackStateBadge,
        Self::VerificationBadge,
        Self::PackActionCluster,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackIdentityLabel => "pack_identity_label",
            Self::SelectedScopeBadge => "selected_scope_badge",
            Self::SizeCountMeter => "size_count_meter",
            Self::SignerSourceBadge => "signer_source_badge",
            Self::PackStateBadge => "pack_state_badge",
            Self::RefreshTimeLabel => "refresh_time_label",
            Self::VerificationBadge => "verification_badge",
            Self::PackActionCluster => "pack_action_cluster",
        }
    }
}

/// The code entity a stale-example finding connects to — a snippet, a shell command, a
/// config shape, an API signature, or a link target — so a finding is always anchored to
/// something concrete rather than a vague "docs may be old" hint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsExampleAnchorKind {
    /// A code snippet.
    CodeSnippet,
    /// A shell command.
    ShellCommand,
    /// A config shape.
    ConfigShape,
    /// An API signature.
    ApiSignature,
    /// A link target.
    LinkTarget,
}

impl M5DocsExampleAnchorKind {
    /// Every anchor kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::CodeSnippet,
        Self::ShellCommand,
        Self::ConfigShape,
        Self::ApiSignature,
        Self::LinkTarget,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeSnippet => "code_snippet",
            Self::ShellCommand => "shell_command",
            Self::ConfigShape => "config_shape",
            Self::ApiSignature => "api_signature",
            Self::LinkTarget => "link_target",
        }
    }
}

/// The derived example drift posture — the resolver's honest verdict about a documented
/// example: verified-current, current-pending-reverify, signature-drift, deprecated-symbol,
/// broken-reference, version-mismatch, or unverified-needs-check. A drifted or unverified
/// example is never shown as current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsExampleDriftPosture {
    /// The example is verified current.
    ExampleVerifiedCurrent,
    /// The example claims current but its freshness is stale/unknown — reverify.
    ExampleCurrentPendingReverify,
    /// The example's API signature has drifted.
    SignatureDriftActionable,
    /// The example uses a deprecated symbol.
    DeprecatedSymbolActionable,
    /// The example links to a broken target.
    BrokenReferenceActionable,
    /// The example is bound to a mismatched version.
    VersionMismatchActionable,
    /// The example is unverified and needs a check.
    UnverifiedNeedsCheck,
}

impl M5DocsExampleDriftPosture {
    /// Every drift posture, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ExampleVerifiedCurrent,
        Self::ExampleCurrentPendingReverify,
        Self::SignatureDriftActionable,
        Self::DeprecatedSymbolActionable,
        Self::BrokenReferenceActionable,
        Self::VersionMismatchActionable,
        Self::UnverifiedNeedsCheck,
    ];

    /// The actionable-drift postures the acceptance criteria require to become concrete,
    /// anchored rows rather than a vague hint.
    pub const ACTIONABLE_DRIFTS: [Self; 4] = [
        Self::SignatureDriftActionable,
        Self::DeprecatedSymbolActionable,
        Self::BrokenReferenceActionable,
        Self::VersionMismatchActionable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExampleVerifiedCurrent => "example_verified_current",
            Self::ExampleCurrentPendingReverify => "example_current_pending_reverify",
            Self::SignatureDriftActionable => "signature_drift_actionable",
            Self::DeprecatedSymbolActionable => "deprecated_symbol_actionable",
            Self::BrokenReferenceActionable => "broken_reference_actionable",
            Self::VersionMismatchActionable => "version_mismatch_actionable",
            Self::UnverifiedNeedsCheck => "unverified_needs_check",
        }
    }

    /// Review-safe phrase for the disclosure headline.
    pub const fn phrase(self) -> &'static str {
        match self {
            Self::ExampleVerifiedCurrent => "the example is verified current",
            Self::ExampleCurrentPendingReverify => {
                "the example claims current but its freshness is stale or unknown"
            }
            Self::SignatureDriftActionable => "the API signature the example uses has drifted",
            Self::DeprecatedSymbolActionable => "the example uses a deprecated symbol",
            Self::BrokenReferenceActionable => "the example links to a broken target",
            Self::VersionMismatchActionable => "the example is bound to a mismatched version",
            Self::UnverifiedNeedsCheck => "the example is unverified and needs a check",
        }
    }

    /// True when the example reads as current — only the verified-current posture.
    pub const fn shows_as_current(self) -> bool {
        matches!(self, Self::ExampleVerifiedCurrent)
    }

    /// True when the example is an actionable drift (a concrete drift a user can compare
    /// and fix).
    pub const fn is_actionable_drift(self) -> bool {
        matches!(
            self,
            Self::SignatureDriftActionable
                | Self::DeprecatedSymbolActionable
                | Self::BrokenReferenceActionable
                | Self::VersionMismatchActionable
        )
    }
}

/// One action a stale-example finding row exposes: compare the drift, open the current
/// source, suppress the finding, or export it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsExampleAction {
    /// Compare the documented example against the current source.
    CompareDrift,
    /// Open the current source of truth.
    OpenCurrentSource,
    /// Suppress the finding.
    SuppressFinding,
    /// Export the finding.
    ExportFinding,
}

impl M5DocsExampleAction {
    /// Every example action, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CompareDrift,
        Self::OpenCurrentSource,
        Self::SuppressFinding,
        Self::ExportFinding,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompareDrift => "compare_drift",
            Self::OpenCurrentSource => "open_current_source",
            Self::SuppressFinding => "suppress_finding",
            Self::ExportFinding => "export_finding",
        }
    }
}

/// One anatomy part the shared stale-example finding row surfaces. The parts in
/// [`M5DocsStaleExampleRowAnatomyPart::MANDATORY`] are required so a finding always
/// carries a concrete anchor, a drift status, version-drift context, and its actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsStaleExampleRowAnatomyPart {
    /// The finding title label.
    FindingTitleLabel,
    /// The affected snippet/command/config anchor reference.
    AffectedAnchorRef,
    /// The drift-status badge.
    DriftStatusBadge,
    /// The version-drift context (documented vs current).
    VersionDriftContext,
    /// The source-provider badge.
    SourceProviderBadge,
    /// The compare / open-current-source / suppress action cluster.
    ExampleActionCluster,
}

impl M5DocsStaleExampleRowAnatomyPart {
    /// Every stale-example anatomy part, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FindingTitleLabel,
        Self::AffectedAnchorRef,
        Self::DriftStatusBadge,
        Self::VersionDriftContext,
        Self::SourceProviderBadge,
        Self::ExampleActionCluster,
    ];

    /// The stale-example anatomy parts every consumer must render.
    pub const MANDATORY: [Self; 5] = [
        Self::FindingTitleLabel,
        Self::AffectedAnchorRef,
        Self::DriftStatusBadge,
        Self::VersionDriftContext,
        Self::ExampleActionCluster,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FindingTitleLabel => "finding_title_label",
            Self::AffectedAnchorRef => "affected_anchor_ref",
            Self::DriftStatusBadge => "drift_status_badge",
            Self::VersionDriftContext => "version_drift_context",
            Self::SourceProviderBadge => "source_provider_badge",
            Self::ExampleActionCluster => "example_action_cluster",
        }
    }
}

/// A field the support / export packet carries so pack-row and stale-example-row identity
/// is reconstructable from the shared model. The fields in
/// [`M5DocsPackFindingExportField::MANDATORY`] are required so the pin/mirror/offline/
/// quarantine state, verification, trust posture, drift posture, and freshness survive
/// export/support paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DocsPackFindingExportField {
    /// The docs-pack state.
    PackState,
    /// The selected version / package scope.
    SelectedScope,
    /// The signer / source.
    SignerSource,
    /// The verification state.
    VerificationState,
    /// The derived pack trust posture.
    TrustPosture,
    /// The declared freshness state.
    FreshnessState,
    /// The stale-example status.
    StaleExampleStatus,
    /// The derived example drift posture.
    DriftPosture,
    /// The affected snippet/command/config anchor.
    AffectedAnchor,
    /// The version-drift context.
    VersionDrift,
    /// The source provider.
    SourceProvider,
    /// The refresh time.
    RefreshTime,
}

impl M5DocsPackFindingExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::PackState,
        Self::SelectedScope,
        Self::SignerSource,
        Self::VerificationState,
        Self::TrustPosture,
        Self::FreshnessState,
        Self::StaleExampleStatus,
        Self::DriftPosture,
        Self::AffectedAnchor,
        Self::VersionDrift,
        Self::SourceProvider,
        Self::RefreshTime,
    ];

    /// The export fields every consumer must carry so identity survives export/support.
    pub const MANDATORY: [Self; 7] = [
        Self::PackState,
        Self::SignerSource,
        Self::VerificationState,
        Self::TrustPosture,
        Self::StaleExampleStatus,
        Self::DriftPosture,
        Self::FreshnessState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackState => "pack_state",
            Self::SelectedScope => "selected_scope",
            Self::SignerSource => "signer_source",
            Self::VerificationState => "verification_state",
            Self::TrustPosture => "trust_posture",
            Self::FreshnessState => "freshness_state",
            Self::StaleExampleStatus => "stale_example_status",
            Self::DriftPosture => "drift_posture",
            Self::AffectedAnchor => "affected_anchor",
            Self::VersionDrift => "version_drift",
            Self::SourceProvider => "source_provider",
            Self::RefreshTime => "refresh_time",
        }
    }
}

// ---- pack resolver ------------------------------------------------------

/// The full input to the pack-row resolver for one docs pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsPackRowResolutionInput {
    /// The opaque, export-safe pack identity. Must be non-empty.
    pub pack_name_repr: String,
    /// The corpus class the pack covers.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider behind the pack.
    pub source_provider: M5DocsSourceProvider,
    /// The version / package scope the pack is selected at.
    pub version_scope: M5DocsVersionScope,
    /// The lifecycle state of the pack.
    pub pack_state: M5DocsPackState,
    /// The declared freshness state of the pack.
    pub freshness_state: M5DocsFreshnessState,
    /// The verification state of the pack.
    pub verification_state: M5DocsPackVerificationState,
    /// The number of documents in the pack.
    pub item_count: u32,
    /// The pack size in bytes.
    pub size_bytes: u64,
    /// The opaque, export-safe signer representation. May be empty ("unsigned").
    pub signer_repr: String,
    /// The opaque, export-safe refresh-time representation. May be empty.
    pub refresh_time_repr: String,
    /// The opaque, export-safe management action target. Must be non-empty.
    pub manage_action_target_repr: String,
}

/// The resolved pin/mirror/offline/quarantine/verification truth for one docs pack.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDocsPackRow {
    /// The opaque pack identity.
    pub pack_name_repr: String,
    /// The corpus class.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider.
    pub source_provider: M5DocsSourceProvider,
    /// The version / package scope.
    pub version_scope: M5DocsVersionScope,
    /// The lifecycle state.
    pub pack_state: M5DocsPackState,
    /// The declared freshness state.
    pub freshness_state: M5DocsFreshnessState,
    /// The verification state.
    pub verification_state: M5DocsPackVerificationState,
    /// The number of documents.
    pub item_count: u32,
    /// The pack size in bytes.
    pub size_bytes: u64,
    /// The opaque signer.
    pub signer_repr: String,
    /// The opaque refresh time.
    pub refresh_time_repr: String,
    /// The opaque management action target.
    pub manage_action_target_repr: String,
    /// The derived pack trust posture.
    pub trust_posture: M5DocsPackTrustPosture,
    /// True when the pack reads as trusted-and-current.
    pub is_trusted_current: bool,
    /// True when the pack reads as live (never true for mirror, offline, stale,
    /// update-overdue, quarantined, or unverified packs).
    pub shows_as_live: bool,
    /// True when the pack is quarantined.
    pub is_quarantined: bool,
    /// True when the pack is signed / verified.
    pub is_signature_verified: bool,
    /// The pin/offline/refresh/quarantine/update/remove actions the pack allows.
    pub available_actions: Vec<M5DocsPackAction>,
    /// A deterministic, self-contained disclosure headline naming the trust posture,
    /// pack state, source, verification, and scope.
    pub disclosure_headline: String,
}

// ---- stale-example resolver ---------------------------------------------

/// The full input to the stale-example finding resolver for one finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsStaleExampleRowResolutionInput {
    /// The opaque, export-safe finding title. Must be non-empty.
    pub finding_title_repr: String,
    /// The opaque, export-safe affected snippet/command/config anchor. Must be
    /// non-empty so the finding is anchored to something concrete.
    pub affected_anchor_repr: String,
    /// The kind of code entity the finding is anchored to.
    pub anchor_kind: M5DocsExampleAnchorKind,
    /// The corpus class the example belongs to.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider behind the example.
    pub source_provider: M5DocsSourceProvider,
    /// The version / package scope in effect.
    pub version_scope: M5DocsVersionScope,
    /// The stale-example status.
    pub stale_example_status: M5DocsStaleExampleStatus,
    /// The declared freshness state of the example.
    pub freshness_state: M5DocsFreshnessState,
    /// The opaque, export-safe documented version. May be empty.
    pub documented_version_repr: String,
    /// The opaque, export-safe current version. May be empty.
    pub current_version_repr: String,
    /// The opaque, export-safe open-current-source target. Must be non-empty.
    pub open_current_source_target_repr: String,
}

/// The resolved drift / version-drift truth for one stale-example finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedDocsStaleExampleRow {
    /// The opaque finding title.
    pub finding_title_repr: String,
    /// The opaque affected anchor.
    pub affected_anchor_repr: String,
    /// The anchor kind.
    pub anchor_kind: M5DocsExampleAnchorKind,
    /// The corpus class.
    pub corpus_class: M5DocsCorpusClass,
    /// The source provider.
    pub source_provider: M5DocsSourceProvider,
    /// The version / package scope.
    pub version_scope: M5DocsVersionScope,
    /// The stale-example status.
    pub stale_example_status: M5DocsStaleExampleStatus,
    /// The declared freshness state.
    pub freshness_state: M5DocsFreshnessState,
    /// The opaque documented version.
    pub documented_version_repr: String,
    /// The opaque current version.
    pub current_version_repr: String,
    /// The opaque open-current-source target.
    pub open_current_source_target_repr: String,
    /// The derived example drift posture.
    pub drift_posture: M5DocsExampleDriftPosture,
    /// True when the example reads as current (never for a drifted/unverified example).
    pub shows_as_current: bool,
    /// True when the example is an actionable drift.
    pub is_actionable_drift: bool,
    /// True when the documented and current versions differ (version drift).
    pub has_version_drift: bool,
    /// The compare / open-current-source / suppress / export actions the finding allows.
    pub available_actions: Vec<M5DocsExampleAction>,
    /// A deterministic, self-contained disclosure headline naming the drift posture, the
    /// anchor kind, the version-drift context, and the source.
    pub disclosure_headline: String,
}

/// Errors returned by the pack-row and stale-example resolvers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5DocsPackFindingResolutionError {
    /// The pack name was empty.
    EmptyPackName,
    /// The affected example anchor was empty (a finding must be anchored to something
    /// concrete).
    EmptyExampleAnchor,
    /// The finding title was empty.
    EmptyFindingTitle,
    /// An action target was empty (the row must be actionable).
    EmptyActionTarget,
    /// A representation carried forbidden material.
    ForbiddenFindingMaterial,
}

impl M5DocsPackFindingResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyPackName => "empty_pack_name",
            Self::EmptyExampleAnchor => "empty_example_anchor",
            Self::EmptyFindingTitle => "empty_finding_title",
            Self::EmptyActionTarget => "empty_action_target",
            Self::ForbiddenFindingMaterial => "forbidden_finding_material",
        }
    }
}

impl fmt::Display for M5DocsPackFindingResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "pack/finding resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5DocsPackFindingResolutionError {}

/// Resolves one docs-pack row from its declared lifecycle state.
///
/// The derived trust posture is computed in a fixed, honesty-first order so a
/// quarantined, verification-failed, update-available, stale, offline, or mirrored pack
/// keeps a distinct posture and never collapses into one generic warning or reads as
/// live: quarantine and verification failure come first, then update-available, then
/// stale freshness, then offline, then mirror, then the pinned/tracking current states.
/// The pack's pin/offline/refresh/quarantine/update/remove actions are attached from the
/// same posture, with an always-available manifest export so pack actions keep export
/// parity.
pub fn resolve_docs_pack_row(
    input: &M5DocsPackRowResolutionInput,
) -> Result<M5ResolvedDocsPackRow, M5DocsPackFindingResolutionError> {
    if input.pack_name_repr.trim().is_empty() {
        return Err(M5DocsPackFindingResolutionError::EmptyPackName);
    }
    if input.manage_action_target_repr.trim().is_empty() {
        return Err(M5DocsPackFindingResolutionError::EmptyActionTarget);
    }
    if value_repr_is_forbidden(&input.pack_name_repr)
        || value_repr_is_forbidden(&input.signer_repr)
        || value_repr_is_forbidden(&input.refresh_time_repr)
        || value_repr_is_forbidden(&input.manage_action_target_repr)
    {
        return Err(M5DocsPackFindingResolutionError::ForbiddenFindingMaterial);
    }

    let trust_posture = derive_pack_trust_posture(
        input.pack_state,
        input.freshness_state,
        input.verification_state,
    );
    let is_trusted_current = trust_posture.is_trusted_current();
    let shows_as_live = is_trusted_current;
    let is_quarantined = trust_posture.is_quarantined();
    let is_signature_verified = input.verification_state.is_signature_verified();
    let available_actions =
        derive_pack_actions(trust_posture, input.pack_state, input.verification_state);

    let disclosure_headline = format!(
        "This pack is shown as {} because {} — {} state on a {} pack (source: {}, verification: {}, scope: {})",
        trust_posture.glyph_label(),
        trust_posture.phrase(),
        input.pack_state.as_str(),
        input.pack_state.as_str(),
        input.source_provider.as_str(),
        input.verification_state.as_str(),
        input.version_scope.as_str()
    );

    Ok(M5ResolvedDocsPackRow {
        pack_name_repr: input.pack_name_repr.clone(),
        corpus_class: input.corpus_class,
        source_provider: input.source_provider,
        version_scope: input.version_scope,
        pack_state: input.pack_state,
        freshness_state: input.freshness_state,
        verification_state: input.verification_state,
        item_count: input.item_count,
        size_bytes: input.size_bytes,
        signer_repr: input.signer_repr.clone(),
        refresh_time_repr: input.refresh_time_repr.clone(),
        manage_action_target_repr: input.manage_action_target_repr.clone(),
        trust_posture,
        is_trusted_current,
        shows_as_live,
        is_quarantined,
        is_signature_verified,
        available_actions,
        disclosure_headline,
    })
}

/// The fixed, honesty-first pack trust-posture ladder.
fn derive_pack_trust_posture(
    pack_state: M5DocsPackState,
    freshness: M5DocsFreshnessState,
    verification: M5DocsPackVerificationState,
) -> M5DocsPackTrustPosture {
    use M5DocsFreshnessState as Fresh;
    use M5DocsPackState as Pack;

    if matches!(pack_state, Pack::QuarantinedPack) {
        // Quarantine is the strongest untrusted signal and never collapses into another
        // state.
        M5DocsPackTrustPosture::QuarantinedUntrusted
    } else if verification.is_failed() {
        M5DocsPackTrustPosture::VerificationUnverified
    } else if matches!(pack_state, Pack::UpdateAvailable) {
        M5DocsPackTrustPosture::UpdateOverdue
    } else if matches!(freshness, Fresh::StaleExpired) {
        M5DocsPackTrustPosture::StaleNeedsRefresh
    } else if matches!(pack_state, Pack::OfflinePack) {
        M5DocsPackTrustPosture::OfflineOnly
    } else if matches!(pack_state, Pack::MirroredPack) {
        M5DocsPackTrustPosture::MirrorServedNotLive
    } else if matches!(pack_state, Pack::PinnedPack) {
        M5DocsPackTrustPosture::PinnedCurrent
    } else {
        M5DocsPackTrustPosture::TrackingCurrent
    }
}

/// The pin/offline/refresh/quarantine/update/remove action set for a pack, emitted in
/// canonical [`M5DocsPackAction::ALL`] order. The manifest export is always available so
/// pack actions keep export parity.
fn derive_pack_actions(
    posture: M5DocsPackTrustPosture,
    pack_state: M5DocsPackState,
    _verification: M5DocsPackVerificationState,
) -> Vec<M5DocsPackAction> {
    use M5DocsPackAction as Action;
    use M5DocsPackState as Pack;

    let mut actions = Vec::new();
    for action in Action::ALL {
        let include = match action {
            Action::PinPack => !matches!(pack_state, Pack::PinnedPack),
            Action::UnpinPack => matches!(pack_state, Pack::PinnedPack),
            Action::RefreshPack => !posture.is_trusted_current(),
            Action::TakeOffline => !matches!(pack_state, Pack::OfflinePack),
            Action::ReviewQuarantine => posture.is_quarantined(),
            Action::UpdatePack => {
                matches!(posture, M5DocsPackTrustPosture::UpdateOverdue)
                    || matches!(pack_state, Pack::UpdateAvailable)
            }
            Action::RemovePack => true,
            Action::ExportPackManifest => true,
        };
        if include {
            actions.push(action);
        }
    }
    actions
}

/// Resolves one stale-example finding row from its declared drift state.
///
/// The derived drift posture keeps a drifted or unverified example from ever reading as
/// current: only an `example_current` status with live/recent freshness reads as
/// verified-current; `example_current` with stale/unknown freshness reads as
/// pending-reverify; and every other status maps to its own actionable-drift or
/// needs-check posture. The compare / open-current-source / suppress / export actions are
/// attached from the same posture so a drift becomes a concrete, anchored, actionable row.
pub fn resolve_stale_example_row(
    input: &M5DocsStaleExampleRowResolutionInput,
) -> Result<M5ResolvedDocsStaleExampleRow, M5DocsPackFindingResolutionError> {
    if input.finding_title_repr.trim().is_empty() {
        return Err(M5DocsPackFindingResolutionError::EmptyFindingTitle);
    }
    if input.affected_anchor_repr.trim().is_empty() {
        return Err(M5DocsPackFindingResolutionError::EmptyExampleAnchor);
    }
    if input.open_current_source_target_repr.trim().is_empty() {
        return Err(M5DocsPackFindingResolutionError::EmptyActionTarget);
    }
    if value_repr_is_forbidden(&input.finding_title_repr)
        || value_repr_is_forbidden(&input.affected_anchor_repr)
        || value_repr_is_forbidden(&input.documented_version_repr)
        || value_repr_is_forbidden(&input.current_version_repr)
        || value_repr_is_forbidden(&input.open_current_source_target_repr)
    {
        return Err(M5DocsPackFindingResolutionError::ForbiddenFindingMaterial);
    }

    let drift_posture =
        derive_example_drift_posture(input.stale_example_status, input.freshness_state);
    let shows_as_current = drift_posture.shows_as_current();
    let is_actionable_drift = drift_posture.is_actionable_drift();
    let has_version_drift = !input.documented_version_repr.trim().is_empty()
        && !input.current_version_repr.trim().is_empty()
        && input.documented_version_repr.trim() != input.current_version_repr.trim();
    let available_actions = derive_example_actions(drift_posture);

    let disclosure_headline = format!(
        "This finding is shown because {} — {} drift on a {} anchor (documented: {}, current: {}, source: {})",
        drift_posture.phrase(),
        drift_posture.as_str(),
        input.anchor_kind.as_str(),
        version_repr_or_unstated(&input.documented_version_repr),
        version_repr_or_unstated(&input.current_version_repr),
        input.source_provider.as_str()
    );

    Ok(M5ResolvedDocsStaleExampleRow {
        finding_title_repr: input.finding_title_repr.clone(),
        affected_anchor_repr: input.affected_anchor_repr.clone(),
        anchor_kind: input.anchor_kind,
        corpus_class: input.corpus_class,
        source_provider: input.source_provider,
        version_scope: input.version_scope,
        stale_example_status: input.stale_example_status,
        freshness_state: input.freshness_state,
        documented_version_repr: input.documented_version_repr.clone(),
        current_version_repr: input.current_version_repr.clone(),
        open_current_source_target_repr: input.open_current_source_target_repr.clone(),
        drift_posture,
        shows_as_current,
        is_actionable_drift,
        has_version_drift,
        available_actions,
        disclosure_headline,
    })
}

/// The fixed drift-posture ladder: a drifted or unverified example never reads as
/// current, and an example claiming current with stale/unknown freshness is held for
/// reverification rather than shown as verified.
fn derive_example_drift_posture(
    status: M5DocsStaleExampleStatus,
    freshness: M5DocsFreshnessState,
) -> M5DocsExampleDriftPosture {
    use M5DocsFreshnessState as Fresh;
    use M5DocsStaleExampleStatus as Status;

    match status {
        Status::ExampleCurrent => match freshness {
            Fresh::StaleExpired | Fresh::UnknownFreshness => {
                M5DocsExampleDriftPosture::ExampleCurrentPendingReverify
            }
            _ => M5DocsExampleDriftPosture::ExampleVerifiedCurrent,
        },
        Status::ApiSignatureDrifted => M5DocsExampleDriftPosture::SignatureDriftActionable,
        Status::DeprecatedSymbolUsed => M5DocsExampleDriftPosture::DeprecatedSymbolActionable,
        Status::BrokenLinkTarget => M5DocsExampleDriftPosture::BrokenReferenceActionable,
        Status::VersionMismatchExample => M5DocsExampleDriftPosture::VersionMismatchActionable,
        Status::UnverifiedExample => M5DocsExampleDriftPosture::UnverifiedNeedsCheck,
    }
}

/// The compare / open-current-source / suppress / export action set for a finding,
/// emitted in canonical [`M5DocsExampleAction::ALL`] order. Open-current-source and
/// export are always available so findings keep export parity.
fn derive_example_actions(posture: M5DocsExampleDriftPosture) -> Vec<M5DocsExampleAction> {
    use M5DocsExampleAction as Action;

    let mut actions = Vec::new();
    for action in Action::ALL {
        let include = match action {
            Action::CompareDrift => !posture.shows_as_current(),
            Action::OpenCurrentSource => true,
            Action::SuppressFinding => !posture.shows_as_current(),
            Action::ExportFinding => true,
        };
        if include {
            actions.push(action);
        }
    }
    actions
}

fn version_repr_or_unstated(value: &str) -> &str {
    if value.trim().is_empty() {
        "unstated"
    } else {
        value
    }
}

// ---- worked cases -------------------------------------------------------

/// One worked pack resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsPackRowResolutionCase {
    /// The resolver input.
    pub input: M5DocsPackRowResolutionInput,
    /// The resolved truth. Must equal `resolve_docs_pack_row(&input)`.
    pub resolved: M5ResolvedDocsPackRow,
}

impl M5DocsPackRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DocsPackRowResolutionInput) -> Self {
        let resolved = resolve_docs_pack_row(&input).expect("seed pack resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_docs_pack_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One worked stale-example resolution case carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsStaleExampleRowResolutionCase {
    /// The resolver input.
    pub input: M5DocsStaleExampleRowResolutionInput,
    /// The resolved truth. Must equal `resolve_stale_example_row(&input)`.
    pub resolved: M5ResolvedDocsStaleExampleRow,
}

impl M5DocsStaleExampleRowResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5DocsStaleExampleRowResolutionInput) -> Self {
        let resolved =
            resolve_stale_example_row(&input).expect("seed stale-example resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_stale_example_row(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one pack/finding consumer bound to the shared
/// pack-row and stale-example anatomy, the same trust postures, drift postures, pack
/// states, stale-example statuses, actions, export fields, and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsPackFindingRow {
    /// Pack/finding consumer family.
    pub consumer_surface: M5DocsPackConsumerSurface,
    /// Qualification class earned by this consumer.
    pub qualification: M5DocsQualificationClass,
    /// Owner role accountable for keeping this consumer governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 docs surface families that render / consume these rows.
    pub surface_families: Vec<M5DocsSurfaceFamily>,
    /// Deployment lines these rows keep the same truth across.
    pub deployment_lines: Vec<M5DocsDeploymentLine>,
    /// Pack-row anatomy parts this consumer renders (must include the mandatory parts).
    pub pack_anatomy_parts: Vec<M5DocsPackRowAnatomyPart>,
    /// Stale-example anatomy parts this consumer renders (must include the mandatory
    /// parts).
    pub example_anatomy_parts: Vec<M5DocsStaleExampleRowAnatomyPart>,
    /// Pack states this consumer distinguishes.
    pub pack_states: Vec<M5DocsPackState>,
    /// Pack trust postures this consumer distinguishes.
    pub trust_postures: Vec<M5DocsPackTrustPosture>,
    /// Verification states this consumer distinguishes.
    pub verification_states: Vec<M5DocsPackVerificationState>,
    /// Pack actions this consumer offers.
    pub pack_actions: Vec<M5DocsPackAction>,
    /// Stale-example statuses this consumer distinguishes.
    pub stale_example_statuses: Vec<M5DocsStaleExampleStatus>,
    /// Example drift postures this consumer distinguishes.
    pub drift_postures: Vec<M5DocsExampleDriftPosture>,
    /// Example anchor kinds this consumer distinguishes.
    pub anchor_kinds: Vec<M5DocsExampleAnchorKind>,
    /// Example actions this consumer offers.
    pub example_actions: Vec<M5DocsExampleAction>,
    /// Corpus classes these rows name.
    pub corpus_classes: Vec<M5DocsCorpusClass>,
    /// Source providers these rows name.
    pub source_providers: Vec<M5DocsSourceProvider>,
    /// Version scopes these rows name.
    pub version_scopes: Vec<M5DocsVersionScope>,
    /// Freshness states these rows disclose.
    pub freshness_states: Vec<M5DocsFreshnessState>,
    /// Export fields these rows carry (must include the mandatory fields).
    pub export_fields: Vec<M5DocsPackFindingExportField>,
    /// Non-visual accessibility routes these rows offer.
    pub accessibility_routes: Vec<M5DocsAccessibilityRoute>,
    /// Docs subsystems that consume these rows' projection.
    pub consumer_surfaces: Vec<M5DocsConsumerSurface>,
    /// Downgrade triggers that apply to these rows.
    pub downgrade_triggers: Vec<M5DocsDowngradeTrigger>,
    /// Proof packet refs that keep these rows current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by these rows.
    pub source_contract_refs: Vec<String>,
    /// Worked pack resolution cases proving the pack resolver on this consumer.
    pub pack_examples: Vec<M5DocsPackRowResolutionCase>,
    /// Worked stale-example resolution cases proving the finding resolver on this
    /// consumer.
    pub stale_example_findings: Vec<M5DocsStaleExampleRowResolutionCase>,
    /// Hard invariant: these rows never mask the pack state or the source provider. MUST
    /// be `false`.
    pub masks_pack_state_or_source: bool,
    /// Hard invariant: these rows never show a quarantined, stale, or mirrored pack (or a
    /// drifted example) as trusted / live / current. MUST be `false`.
    pub shows_quarantined_or_stale_as_trusted: bool,
    /// Hard invariant: these rows never invent a private pack/finding grammar. MUST be
    /// `false`.
    pub invents_private_pack_grammar: bool,
    /// Hard invariant: these rows never hide the version-drift context or the affected
    /// anchor. MUST be `false`.
    pub hides_version_drift: bool,
}

impl M5DocsPackFindingRow {
    fn declares_mandatory_pack_anatomy(&self) -> bool {
        let present: BTreeSet<M5DocsPackRowAnatomyPart> =
            self.pack_anatomy_parts.iter().copied().collect();
        M5DocsPackRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_example_anatomy(&self) -> bool {
        let present: BTreeSet<M5DocsStaleExampleRowAnatomyPart> =
            self.example_anatomy_parts.iter().copied().collect();
        M5DocsStaleExampleRowAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5DocsPackFindingExportField> =
            self.export_fields.iter().copied().collect();
        M5DocsPackFindingExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.masks_pack_state_or_source
            && !self.shows_quarantined_or_stale_as_trusted
            && !self.invents_private_pack_grammar
            && !self.hides_version_drift
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsPackFindingVocabularySet {
    /// Pack/finding consumer tokens.
    pub consumer_surfaces: Vec<String>,
    /// Pack-row anatomy-part tokens.
    pub pack_anatomy_parts: Vec<String>,
    /// Stale-example anatomy-part tokens.
    pub example_anatomy_parts: Vec<String>,
    /// Trust-posture tokens.
    pub trust_postures: Vec<String>,
    /// Verification-state tokens.
    pub verification_states: Vec<String>,
    /// Pack-action tokens.
    pub pack_actions: Vec<String>,
    /// Drift-posture tokens.
    pub drift_postures: Vec<String>,
    /// Anchor-kind tokens.
    pub anchor_kinds: Vec<String>,
    /// Example-action tokens.
    pub example_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Pack-state tokens (reused from the frozen matrix).
    pub pack_states: Vec<String>,
    /// Stale-example-status tokens (reused from the frozen matrix).
    pub stale_example_statuses: Vec<String>,
    /// Corpus-class tokens (reused from the frozen matrix).
    pub corpus_classes: Vec<String>,
    /// Version-scope tokens (reused from the frozen matrix).
    pub version_scopes: Vec<String>,
    /// Source-provider tokens (reused from the frozen matrix).
    pub source_providers: Vec<String>,
    /// Freshness-state tokens (reused from the frozen matrix).
    pub freshness_states: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5DocsPackFindingVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            consumer_surfaces: tokens(&M5DocsPackConsumerSurface::ALL, |v| v.as_str()),
            pack_anatomy_parts: tokens(&M5DocsPackRowAnatomyPart::ALL, |v| v.as_str()),
            example_anatomy_parts: tokens(&M5DocsStaleExampleRowAnatomyPart::ALL, |v| v.as_str()),
            trust_postures: tokens(&M5DocsPackTrustPosture::ALL, |v| v.as_str()),
            verification_states: tokens(&M5DocsPackVerificationState::ALL, |v| v.as_str()),
            pack_actions: tokens(&M5DocsPackAction::ALL, |v| v.as_str()),
            drift_postures: tokens(&M5DocsExampleDriftPosture::ALL, |v| v.as_str()),
            anchor_kinds: tokens(&M5DocsExampleAnchorKind::ALL, |v| v.as_str()),
            example_actions: tokens(&M5DocsExampleAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5DocsPackFindingExportField::ALL, |v| v.as_str()),
            pack_states: tokens(&M5DocsPackState::ALL, |v| v.as_str()),
            stale_example_statuses: tokens(&M5DocsStaleExampleStatus::ALL, |v| v.as_str()),
            corpus_classes: tokens(&M5DocsCorpusClass::ALL, |v| v.as_str()),
            version_scopes: tokens(&M5DocsVersionScope::ALL, |v| v.as_str()),
            source_providers: tokens(&M5DocsSourceProvider::ALL, |v| v.as_str()),
            freshness_states: tokens(&M5DocsFreshnessState::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5DocsAccessibilityRoute::ALL, |v| v.as_str()),
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
pub struct M5DocsPackFindingGovernanceReview {
    /// One pack-row primitive and one stale-example primitive carry pack/finding truth on
    /// every consumer.
    pub shared_primitives_carry_truth: bool,
    /// Pinned, stale, mirrored, quarantined, update-overdue, and current packs stay
    /// distinct rather than collapsing into one generic warning.
    pub pack_states_stay_distinct: bool,
    /// A quarantined, stale, mirrored, or offline pack is never shown as freely trusted
    /// or live.
    pub quarantined_or_stale_never_shown_trusted: bool,
    /// The verification state stays visible.
    pub verification_state_visible: bool,
    /// Example drift is a concrete, anchored, actionable row rather than a vague hint.
    pub example_drift_actionable_with_anchor: bool,
    /// A drifted or unverified example is never shown as current.
    pub drifted_example_never_shown_current: bool,
    /// The version-drift context stays visible.
    pub version_drift_context_visible: bool,
    /// Pack and example actions keep mirror/offline/export parity across consumers.
    pub actions_keep_mirror_offline_export_parity: bool,
    /// No consumer invents a second pack-row or stale-example grammar.
    pub no_surface_invents_second_grammar: bool,
    /// Every consumer declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel pack/finding vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsPackFindingConsumerProjection {
    /// Docs-manager, help, onboarding, AI-context, and support consumers all consume the
    /// shared primitives.
    pub consumers_consume_shared_primitives: bool,
    /// The pack trust posture reads a single canonical source.
    pub trust_posture_reads_single_source: bool,
    /// The example drift posture reads a single canonical source.
    pub drift_posture_reads_single_source: bool,
    /// The pack/example actions read a single canonical source.
    pub actions_read_single_source: bool,
    /// Support / export reads a single canonical pack/finding source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsPackFindingProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the pack/finding primitives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsPackFindingReleasePosture {
    /// Ref of the supporting proof packet.
    pub proof_packet_ref: String,
    /// Ref of the supporting pack/finding audit.
    pub pack_finding_audit_ref: String,
    /// True when support / export parity is required for every consumer.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every consumer.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DocsPackFindingPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DocsPackFindingPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Pack/finding rows.
    pub pack_finding_rows: Vec<M5DocsPackFindingRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsPackFindingVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsPackFindingGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsPackFindingConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsPackFindingProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsPackFindingReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 docs-pack-row / stale-example-finding-row primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DocsPackFindingPrimitivePacket {
    /// Record kind; must equal [`M5_DOCS_PACK_FINDING_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DOCS_PACK_FINDING_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Pack/finding rows.
    pub pack_finding_rows: Vec<M5DocsPackFindingRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DocsPackFindingVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DocsPackFindingGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DocsPackFindingConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DocsPackFindingProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DocsPackFindingReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DocsPackFindingPrimitivePacket {
    /// Builds an M5 pack/finding-primitive packet from stable-lane input.
    pub fn new(input: M5DocsPackFindingPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_DOCS_PACK_FINDING_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_DOCS_PACK_FINDING_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            pack_finding_rows: input.pack_finding_rows,
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

    /// Validates the M5 pack/finding-primitive invariants.
    pub fn validate(&self) -> Vec<M5DocsPackFindingPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DOCS_PACK_FINDING_PRIMITIVE_RECORD_KIND {
            violations.push(M5DocsPackFindingPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DOCS_PACK_FINDING_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5DocsPackFindingPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DocsPackFindingPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_pack_finding_rows(self, &mut violations);
        validate_pack_state_distinctness(self, &mut violations);
        validate_example_drift_actionable(self, &mut violations);
        validate_action_parity(self, &mut violations);
        validate_trust_honesty(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 pack/finding primitive packet serializes"),
        ) {
            violations.push(M5DocsPackFindingPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 pack/finding primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per pack/finding consumer.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,pack_states,trust_postures,pack_actions,stale_example_statuses,drift_postures,example_actions,export_fields,pack_examples,example_findings\n",
        );
        for row in &self.pack_finding_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.pack_states, |v| v.as_str()),
                join_tokens(&row.trust_postures, |v| v.as_str()),
                join_tokens(&row.pack_actions, |v| v.as_str()),
                join_tokens(&row.stale_example_statuses, |v| v.as_str()),
                join_tokens(&row.drift_postures, |v| v.as_str()),
                join_tokens(&row.example_actions, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.pack_examples.len(),
                row.stale_example_findings.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .pack_finding_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Docs-Pack Row & Stale-Example Finding-Row Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Pack/finding consumers: {} ({} stable)\n",
            self.pack_finding_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Trust postures: {}\n",
            self.vocabulary_set.trust_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Drift postures: {}\n",
            self.vocabulary_set.drift_postures.join(", ")
        ));
        out.push_str(&format!(
            "- Pack actions: {}\n",
            self.vocabulary_set.pack_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Pack/finding consumers\n\n");
        for row in &self.pack_finding_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked pack rows: {}\n",
                row.pack_examples.len()
            ));
            for case in &row.pack_examples {
                out.push_str(&format!(
                    "    - `{}` → trust `{}` (pack `{}`, verification `{}`, live `{}`)\n",
                    case.resolved.pack_name_repr,
                    case.resolved.trust_posture.as_str(),
                    case.resolved.pack_state.as_str(),
                    case.resolved.verification_state.as_str(),
                    case.resolved.shows_as_live,
                ));
            }
            out.push_str(&format!(
                "  - Worked stale-example findings: {}\n",
                row.stale_example_findings.len()
            ));
            for case in &row.stale_example_findings {
                out.push_str(&format!(
                    "    - `{}` on `{}` → drift `{}` (anchor `{}`, current `{}`, version-drift `{}`)\n",
                    case.resolved.finding_title_repr,
                    case.resolved.affected_anchor_repr,
                    case.resolved.drift_posture.as_str(),
                    case.resolved.anchor_kind.as_str(),
                    case.resolved.shows_as_current,
                    case.resolved.has_version_drift,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 pack/finding-primitive export.
#[derive(Debug)]
pub enum M5DocsPackFindingPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DocsPackFindingPrimitiveViolation>),
}

impl fmt::Display for M5DocsPackFindingPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 pack/finding primitive export parse failed: {error}"
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
                    "m5 pack/finding primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DocsPackFindingPrimitiveArtifactError {}

/// Validation failures emitted by [`M5DocsPackFindingPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DocsPackFindingPrimitiveViolation {
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
    /// A required pack/finding consumer family is missing from the matrix.
    RequiredConsumerMissing,
    /// A pack/finding row is incomplete.
    PackFindingRowIncomplete,
    /// A row omits one of the mandatory pack-row anatomy parts.
    MandatoryPackAnatomyMissing,
    /// A row omits one of the mandatory stale-example anatomy parts.
    MandatoryExampleAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked pack or stale-example cases.
    ExampleResolutionMissing,
    /// A worked case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableConsumerMissingProof,
    /// The distinct pinned/stale/mirrored/quarantined/offline/update-overdue pack states
    /// are not all proven by some worked pack row.
    PackStateDistinctnessUnproven,
    /// No worked finding proves an actionable drift anchored to a concrete example.
    ExampleDriftActionableUnproven,
    /// The pack update/remove/export and example compare/open/export actions do not all
    /// appear across the worked cases.
    ActionParityUnproven,
    /// A worked case shows a quarantined/stale/mirrored pack or a drifted example as
    /// trusted/live/current, or no live-and-not-live contrast is proven.
    TrustHonestyUnproven,
    /// A pack/finding row violates a hard invariant.
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

impl M5DocsPackFindingPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredConsumerMissing => "required_consumer_missing",
            Self::PackFindingRowIncomplete => "pack_finding_row_incomplete",
            Self::MandatoryPackAnatomyMissing => "mandatory_pack_anatomy_missing",
            Self::MandatoryExampleAnatomyMissing => "mandatory_example_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleResolutionMissing => "example_resolution_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableConsumerMissingProof => "stable_consumer_missing_proof",
            Self::PackStateDistinctnessUnproven => "pack_state_distinctness_unproven",
            Self::ExampleDriftActionableUnproven => "example_drift_actionable_unproven",
            Self::ActionParityUnproven => "action_parity_unproven",
            Self::TrustHonestyUnproven => "trust_honesty_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 pack/finding-primitive export.
pub fn current_stable_m5_pack_finding_primitive_export(
) -> Result<M5DocsPackFindingPrimitivePacket, M5DocsPackFindingPrimitiveArtifactError> {
    let packet: M5DocsPackFindingPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/m5-docs-pack-row-and-stale-example-finding-row-primitive/support_export.json"
    )))
    .map_err(M5DocsPackFindingPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DocsPackFindingPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DOCS_PACK_FINDING_SCHEMA_REF,
        M5_DOCS_PACK_FINDING_DOC_REF,
        M5_DOCS_PACK_FINDING_COMPONENT_MATRIX_REF,
        M5_DOCS_PACK_FINDING_SOURCE_RESULT_REF,
        M5_DOCS_PACK_FINDING_SOURCE_PRECEDENCE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DocsPackFindingPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DocsPackFindingPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_pack_finding_rows(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    let present: BTreeSet<M5DocsPackConsumerSurface> = packet
        .pack_finding_rows
        .iter()
        .map(|row| row.consumer_surface)
        .collect();
    for required in M5DocsPackConsumerSurface::ALL {
        if !present.contains(&required) {
            violations.push(M5DocsPackFindingPrimitiveViolation::RequiredConsumerMissing);
            return;
        }
    }

    for row in &packet.pack_finding_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.pack_anatomy_parts.is_empty()
            || row.example_anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.pack_states.is_empty()
            || row.trust_postures.is_empty()
            || row.verification_states.is_empty()
            || row.pack_actions.is_empty()
            || row.stale_example_statuses.is_empty()
            || row.drift_postures.is_empty()
            || row.anchor_kinds.is_empty()
            || row.example_actions.is_empty()
            || row.corpus_classes.is_empty()
            || row.source_providers.is_empty()
            || row.version_scopes.is_empty()
            || row.freshness_states.is_empty()
        {
            violations.push(M5DocsPackFindingPrimitiveViolation::PackFindingRowIncomplete);
        }
        if !row.declares_mandatory_pack_anatomy() {
            violations.push(M5DocsPackFindingPrimitiveViolation::MandatoryPackAnatomyMissing);
        }
        if !row.declares_mandatory_example_anatomy() {
            violations.push(M5DocsPackFindingPrimitiveViolation::MandatoryExampleAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5DocsPackFindingPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5DocsAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5DocsPackFindingPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DocsPackFindingPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DocsPackFindingPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.pack_examples.is_empty() || row.stale_example_findings.is_empty() {
            violations.push(M5DocsPackFindingPrimitiveViolation::ExampleResolutionMissing);
        }
        let pack_drift = row
            .pack_examples
            .iter()
            .any(|case| !case.is_self_consistent());
        let finding_drift = row
            .stale_example_findings
            .iter()
            .any(|case| !case.is_self_consistent());
        if pack_drift || finding_drift {
            violations.push(M5DocsPackFindingPrimitiveViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DocsPackFindingPrimitiveViolation::StableConsumerMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DocsPackFindingPrimitiveViolation::RowInvariantViolated);
        }
    }
}

/// Every distinct pack state — pinned, mirrored, offline, update-overdue, stale, and
/// quarantined — must be proven by some worked pack row so they stay explicit rather than
/// collapsing into one generic warning (the acceptance criterion that a user can tell
/// whether a pack is pinned, stale, mirrored, quarantined, or current).
fn validate_pack_state_distinctness(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    for required in M5DocsPackTrustPosture::DISTINCT_STATES {
        let proven = packet.pack_finding_rows.iter().any(|row| {
            row.pack_examples
                .iter()
                .any(|case| case.resolved.trust_posture == required)
        });
        if !proven {
            violations.push(M5DocsPackFindingPrimitiveViolation::PackStateDistinctnessUnproven);
            return;
        }
    }
}

/// At least one worked stale-example finding must be an actionable drift anchored to a
/// concrete example with the compare and open-current-source actions attached (the
/// acceptance criterion that example drift becomes an actionable row with concrete
/// anchors instead of a vague hint).
fn validate_example_drift_actionable(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    let proven = packet.pack_finding_rows.iter().any(|row| {
        row.stale_example_findings.iter().any(|case| {
            case.resolved.is_actionable_drift
                && !case.resolved.affected_anchor_repr.trim().is_empty()
                && case
                    .resolved
                    .available_actions
                    .contains(&M5DocsExampleAction::CompareDrift)
                && case
                    .resolved
                    .available_actions
                    .contains(&M5DocsExampleAction::OpenCurrentSource)
        })
    });
    if !proven {
        violations.push(M5DocsPackFindingPrimitiveViolation::ExampleDriftActionableUnproven);
    }
}

/// The pack update/remove/export-manifest actions and the example compare/open/export
/// actions must all appear across the worked cases so pack/update/remove actions keep
/// mirror/offline/export parity across consumers (the acceptance criterion).
fn validate_action_parity(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    let pack_actions: BTreeSet<M5DocsPackAction> = packet
        .pack_finding_rows
        .iter()
        .flat_map(|row| row.pack_examples.iter())
        .flat_map(|case| case.resolved.available_actions.iter().copied())
        .collect();
    let example_actions: BTreeSet<M5DocsExampleAction> = packet
        .pack_finding_rows
        .iter()
        .flat_map(|row| row.stale_example_findings.iter())
        .flat_map(|case| case.resolved.available_actions.iter().copied())
        .collect();

    let pack_ok = [
        M5DocsPackAction::UpdatePack,
        M5DocsPackAction::RemovePack,
        M5DocsPackAction::ExportPackManifest,
    ]
    .iter()
    .all(|action| pack_actions.contains(action));
    let example_ok = [
        M5DocsExampleAction::CompareDrift,
        M5DocsExampleAction::OpenCurrentSource,
        M5DocsExampleAction::ExportFinding,
    ]
    .iter()
    .all(|action| example_actions.contains(action));

    if !(pack_ok && example_ok) {
        violations.push(M5DocsPackFindingPrimitiveViolation::ActionParityUnproven);
    }
}

/// No worked case may show a quarantined/stale/mirrored/offline pack (or a drifted
/// example) as trusted/live/current, and the matrix must prove both a trusted-current and
/// a not-live pack plus both a verified-current and a drifted example so the honest
/// contrast is always present.
fn validate_trust_honesty(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    let pack_cases = || {
        packet
            .pack_finding_rows
            .iter()
            .flat_map(|row| row.pack_examples.iter())
    };
    let finding_cases = || {
        packet
            .pack_finding_rows
            .iter()
            .flat_map(|row| row.stale_example_findings.iter())
    };

    let no_untrusted_shown_live = pack_cases().all(|case| {
        if case.resolved.shows_as_live {
            case.resolved.trust_posture.is_trusted_current()
        } else {
            true
        }
    });
    let no_drift_shown_current = finding_cases().all(|case| {
        if case.resolved.shows_as_current {
            case.resolved.drift_posture.shows_as_current() && !case.resolved.is_actionable_drift
        } else {
            true
        }
    });
    let has_trusted_pack = pack_cases().any(|case| case.resolved.is_trusted_current);
    let has_not_live_pack = pack_cases().any(|case| !case.resolved.shows_as_live);
    let has_current_example = finding_cases().any(|case| case.resolved.shows_as_current);
    let has_drift_example = finding_cases().any(|case| case.resolved.is_actionable_drift);

    if !(no_untrusted_shown_live
        && no_drift_shown_current
        && has_trusted_pack
        && has_not_live_pack
        && has_current_example
        && has_drift_example)
    {
        violations.push(M5DocsPackFindingPrimitiveViolation::TrustHonestyUnproven);
    }
}

fn validate_governance_review(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.shared_primitives_carry_truth,
        review.pack_states_stay_distinct,
        review.quarantined_or_stale_never_shown_trusted,
        review.verification_state_visible,
        review.example_drift_actionable_with_anchor,
        review.drifted_example_never_shown_current,
        review.version_drift_context_visible,
        review.actions_keep_mirror_offline_export_parity,
        review.no_surface_invents_second_grammar,
        review.every_row_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DocsPackFindingPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.consumers_consume_shared_primitives,
        projection.trust_posture_reads_single_source,
        projection.drift_posture_reads_single_source,
        projection.actions_read_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DocsPackFindingPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DocsPackFindingPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DocsPackFindingPrimitivePacket,
    violations: &mut Vec<M5DocsPackFindingPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.pack_finding_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DocsPackFindingPrimitiveViolation::ReleasePostureIncomplete);
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
