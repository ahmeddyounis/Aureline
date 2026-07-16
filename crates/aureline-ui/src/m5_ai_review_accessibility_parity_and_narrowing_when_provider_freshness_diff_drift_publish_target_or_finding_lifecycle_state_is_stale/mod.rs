//! Keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity, and honest automatic claim
//! narrowing for the M5 AI-review-assist finding row / review scope selector / publish-to-review sheet /
//! resolution memory row objects.
//!
//! This module is the M05-1272 accessibility-and-auto-narrowing capstone over the frozen M5
//! AI-review-assist matrix ([`crate::m5_ai_review_assist_matrix`]). Where the freeze matrix defines the
//! reusable AI review finding row, review scope selector, publish-to-review sheet, and resolution memory
//! row objects, and the 1266-1270 implementation lanes resolve their per-surface truth, this lane certifies
//! — per object class — that AI review claims stay **keyboard-complete, assistive-tech-reachable, high-zoom
//! / high-contrast-safe, CLI/export-safe, and self-narrowing** rather than presenting a stale-provider
//! finding, a diff-drifted scope, an unavailable publish target, or an outdated / suppressed lifecycle state
//! as still a trusted, publish-safe AI review surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / CLI reach.** Every object exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, high-contrast-safe, and
//!   CLI/headless-reachable path into the same object identity, finding class / severity, analyzed diff
//!   scope, publish mode / provider destination, local-versus-provider state, and finding lifecycle state
//!   the rich object shows — never a color-only finding badge, a hover-only scope chip, or a pointer-only
//!   publish action that strands assistive-tech or headless-CLI users. Structure-heavy objects (the publish
//!   sheet's outbound action set, the resolution memory row's lifecycle history) additionally bind their
//!   structured layout to a flat list / textual path.
//! - **Export parity.** The support / CLI / release export reconstructs each object's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same analyzed scope, destination
//!   class, and finding lifecycle labels visible in-product so support, help, and release proof can
//!   reconstruct exactly what the user was shown without leaking a raw diff hunk, message payload, secret,
//!   endpoint, or provider token.
//! - **Honest auto-narrowing.** When provider freshness is stale, diff drift invalidates prior findings, a
//!   publish target is unavailable, or a finding's lifecycle state falls outside live publish-safe
//!   conditions, the object's claim auto-narrows from `trusted_review_surface` / `reviewable_review_surface`
//!   to a provider-freshness-unverified / diff-scope-unverified / publish-target-unverified /
//!   finding-lifecycle-unverified projection, discloses the narrowing with a precise trigger and binding
//!   dimension, and preserves the canonical object identity / last-known state. The underlying finding,
//!   scope, publish, and lifecycle truth is never dropped opaquely. An object with every dimension intact
//!   must NOT carry a spurious narrowing, and a stale-provider / diff-drifted / publish-target-unavailable /
//!   lifecycle-degraded state can never keep a trusted, publish-safe claim — AI review never silently
//!   auto-approves, auto-requests changes, or auto-merges, and a lost local draft never masquerades as a
//!   provider-committed publish.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the review detail, AI review panel,
//!   finding row, review scope selector, publish-to-review sheet, pending-review tray, provider publish
//!   review, resolution memory ledger, and support / export packet so product, help, and release
//!   publication stay aligned on downgrade behavior rather than drifting in copy — a trusted-looking object
//!   can never outrun the provider-freshness / diff-scope / publish-target / lifecycle evidence it is being
//!   viewed away from.
//!
//! Each [`AiReviewAccessibilityRow`] keys on one
//! [`crate::m5_ai_review_assist_matrix::M5AiReviewAssistObject`] and reuses that frozen object vocabulary
//! plus the frozen [`M5AiReviewAssistRequiredLabel`], [`M5AiReviewAssistDowngradeTrigger`], and shared
//! [`M5AiReviewAssistConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling AI-review-assist packets.
//!
//! The packet is metadata-only: raw diff hunks, message payloads, credentials, secrets, and endpoint refs
//! never cross this boundary; the packet carries only typed class tokens, opaque object refs, booleans, and
//! controlled labels so support, release, and diagnostics exports can reconstruct exactly what an accessible
//! fallback would have shown without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen AI-review-assist vocabulary — the capstone certifies the freeze matrix's objects, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_ai_review_assist_matrix::{
    M5AiReviewAssistConsumerSurface, M5AiReviewAssistDowngradeTrigger, M5AiReviewAssistObject,
    M5AiReviewAssistRequiredLabel, M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1272 AI-review-assist accessibility parity packet.
pub const AI_REVIEW_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`AiReviewAccessibilityPacket`].
pub const AI_REVIEW_A11Y_RECORD_KIND: &str = "m5_ai_review_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`AiReviewAccessibilityRow`].
pub const AI_REVIEW_A11Y_ROW_RECORD_KIND: &str = "m5_ai_review_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const AI_REVIEW_A11Y_SCHEMA_REF: &str =
    "schemas/review/m5-ai-review-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const AI_REVIEW_A11Y_DOC_REF: &str = "docs/review/m5_ai_review_accessibility_parity.md";

/// Repo-relative path of the frozen AI-review-assist matrix this lane certifies.
pub const AI_REVIEW_A11Y_MATRIX_REF: &str = M5_AI_REVIEW_ASSIST_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const AI_REVIEW_A11Y_FIXTURE_DIR: &str = "fixtures/review/m5-ai-review-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const AI_REVIEW_A11Y_ARTIFACT_REF: &str =
    "artifacts/review/m5-ai-review-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const AI_REVIEW_A11Y_CSV_REF: &str =
    "artifacts/review/m5-ai-review-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const AI_REVIEW_A11Y_REPORT_REF: &str = "artifacts/review/m5-ai-review-accessibility-parity.md";

/// The reusable objects that render a dense, structured surface (the publish sheet's outbound action set,
/// the resolution memory row's lifecycle history) and therefore MUST bind their structured layout to an
/// equivalent flat list / textual path so the structure is navigable non-visually.
const fn object_is_structure_heavy(object: M5AiReviewAssistObject) -> bool {
    matches!(
        object,
        M5AiReviewAssistObject::PublishToReviewSheet | M5AiReviewAssistObject::ResolutionMemoryRow
    )
}

/// The AI-review-truth dimension whose weakening an object primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn object_primary_dimension(object: M5AiReviewAssistObject) -> M5AiReviewClaimDimension {
    match object {
        M5AiReviewAssistObject::AiReviewFindingRow => {
            M5AiReviewClaimDimension::ProviderFreshnessClarity
        }
        M5AiReviewAssistObject::ReviewScopeSelector => {
            M5AiReviewClaimDimension::DiffScopeDriftClarity
        }
        M5AiReviewAssistObject::PublishToReviewSheet => {
            M5AiReviewClaimDimension::PublishTargetAvailabilityClarity
        }
        M5AiReviewAssistObject::ResolutionMemoryRow => {
            M5AiReviewClaimDimension::FindingLifecycleClarity
        }
    }
}

/// A rendered fallback modality for an AI-review-assist object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewFallbackModality {
    /// A rich, structured (outbound action set / lifecycle history) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5AiReviewFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same object may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs export,
/// or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5AiReviewRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline and
    /// therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / high-contrast / CLI reach for an object's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / color-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl AiReviewNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the object meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewExportSummaryState {
    /// The object meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl AiReviewExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl AiReviewNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The AI-review claim ceiling an object asserts: how strong a trusted / publish-safe posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when a provider-freshness / diff-scope /
/// publish-target / finding-lifecycle dimension weakens so a stale-provider finding, a diff-drifted scope,
/// an unavailable publish target, or an outdated / suppressed lifecycle state can never keep an old
/// `TrustedReviewSurface` or `ReviewableReviewSurface` label — AI review never auto-approves, auto-requests
/// changes, or auto-merges from a narrowed object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewA11yClaim {
    /// Trusted review surface: a fully fresh, diff-scoped, publish-target-available, live-lifecycle object —
    /// the strongest claim, an AI review surface Aureline can present as exactly trusted and publish-safe to
    /// inspect, rerun, dismiss, publish, export, or reopen right now.
    TrustedReviewSurface,
    /// Reviewable review surface: a self-sufficient, reviewable read-only object (a resolution memory row a
    /// user can inspect) that is not itself an authoritative, publish-driving surface.
    ReviewableReviewSurface,
    /// Provider-freshness-unverified projection: the finding's provider freshness is stale; the object stays
    /// a provider-freshness-unverified projection with its last-known finding preserved, never a stale
    /// finding shown as a current, provider-committed truth.
    ProviderFreshnessUnverifiedProjection,
    /// Diff-scope-unverified projection: diff drift invalidates prior findings; the object stays a
    /// diff-scope-unverified projection that keeps the last-known analyzed scope explicit and flags rerun,
    /// never a diff-drifted finding shown as current.
    DiffScopeUnverifiedProjection,
    /// Publish-target-unverified projection: the publish target is unavailable; the object stays a
    /// publish-target-unverified projection that keeps the local draft and export fallback explicit, never a
    /// lost local draft shown as a provider-committed publish.
    PublishTargetUnverifiedProjection,
    /// Finding-lifecycle-unverified projection: the finding's lifecycle state falls outside live
    /// publish-safe conditions (outdated / suppressed); the object stays a finding-lifecycle-unverified
    /// projection that discloses the outdated / suppressed lifecycle state, never an outdated or suppressed
    /// finding shown as live and publish-safe.
    FindingLifecycleUnverifiedProjection,
}

impl M5AiReviewA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::TrustedReviewSurface,
        Self::ReviewableReviewSurface,
        Self::ProviderFreshnessUnverifiedProjection,
        Self::DiffScopeUnverifiedProjection,
        Self::PublishTargetUnverifiedProjection,
        Self::FindingLifecycleUnverifiedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedReviewSurface => 5,
            Self::ReviewableReviewSurface => 4,
            Self::ProviderFreshnessUnverifiedProjection => 3,
            Self::DiffScopeUnverifiedProjection => 2,
            Self::PublishTargetUnverifiedProjection => 1,
            Self::FindingLifecycleUnverifiedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, publish-safe review surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedReviewSurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedReviewSurface | Self::ReviewableReviewSurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedReviewSurface => "trusted_review_surface",
            Self::ReviewableReviewSurface => "reviewable_review_surface",
            Self::ProviderFreshnessUnverifiedProjection => {
                "provider_freshness_unverified_projection"
            }
            Self::DiffScopeUnverifiedProjection => "diff_scope_unverified_projection",
            Self::PublishTargetUnverifiedProjection => "publish_target_unverified_projection",
            Self::FindingLifecycleUnverifiedProjection => "finding_lifecycle_unverified_projection",
        }
    }
}

/// The provider-freshness / diff-scope / publish-target / finding-lifecycle dimension whose state governs
/// how far an object may claim to be a fully trusted, publish-safe review surface. The dimensions map to the
/// four frozen AI-review-assist objects so every object carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewClaimDimension {
    /// Provider-freshness clarity: does the finding row keep its provider freshness current so a stale
    /// finding never reads as a current, provider-committed truth (ai-review-finding-row)?
    ProviderFreshnessClarity,
    /// Diff-scope-drift clarity: does the review scope selector keep its analyzed diff scope current and
    /// flag drift rather than letting prior findings outlive the diff (review-scope-selector)?
    DiffScopeDriftClarity,
    /// Publish-target-availability clarity: does the publish-to-review sheet keep its provider publish target
    /// available, or fall back to a disclosed local draft / export rather than a lost publish
    /// (publish-to-review-sheet)?
    PublishTargetAvailabilityClarity,
    /// Finding-lifecycle clarity: does the resolution memory row keep its finding lifecycle state live and
    /// publish-safe rather than showing an outdated / suppressed finding as current (resolution-memory-row)?
    FindingLifecycleClarity,
}

impl M5AiReviewClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ProviderFreshnessClarity,
        Self::DiffScopeDriftClarity,
        Self::PublishTargetAvailabilityClarity,
        Self::FindingLifecycleClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderFreshnessClarity => "provider_freshness_clarity",
            Self::DiffScopeDriftClarity => "diff_scope_drift_clarity",
            Self::PublishTargetAvailabilityClarity => "publish_target_availability_clarity",
            Self::FindingLifecycleClarity => "finding_lifecycle_clarity",
        }
    }
}

/// The observed condition of one AI-review-truth dimension. Anything weaker than [`Self::FullyQualified`]
/// imposes a narrowing ceiling on the object's claim. The stale / drifted / unavailable / degraded states
/// the lane must auto-narrow on — a stale provider freshness, a diff drift that invalidates prior findings,
/// an unavailable publish target, and a lifecycle state outside live publish-safe conditions — are the
/// states that [`Self::cannot_be_shown_trusted`] flags: each is a genuine truth degradation that can never
/// be shown as a trusted, publish-safe review surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AiReviewConditionState {
    /// Fully fresh, diff-scoped, publish-target-available, live-lifecycle — imposes no ceiling.
    FullyQualified,
    /// The finding's provider freshness is stale — claim drops to a provider-freshness-unverified
    /// projection.
    ProviderFreshnessStale,
    /// Diff drift invalidates prior findings — claim drops to a diff-scope-unverified projection.
    DiffDriftInvalidatesFindings,
    /// The publish target is unavailable — claim drops to a publish-target-unverified projection.
    PublishTargetUnavailable,
    /// The finding's lifecycle state falls outside live publish-safe conditions (outdated / suppressed) —
    /// claim drops to a finding-lifecycle-unverified projection.
    LifecycleOutsidePublishSafe,
}

impl M5AiReviewConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::FullyQualified,
        Self::ProviderFreshnessStale,
        Self::DiffDriftInvalidatesFindings,
        Self::PublishTargetUnavailable,
        Self::LifecycleOutsidePublishSafe,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects a weakened state that cannot be shown as a fully trusted,
    /// publish-safe review surface and must never be shown as such. Every weak AI-review condition is a
    /// genuine truth degradation, so all four flag here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::ProviderFreshnessStale
                | Self::DiffDriftInvalidatesFindings
                | Self::PublishTargetUnavailable
                | Self::LifecycleOutsidePublishSafe
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5AiReviewA11yClaim {
        match self {
            Self::FullyQualified => M5AiReviewA11yClaim::TrustedReviewSurface,
            Self::ProviderFreshnessStale => {
                M5AiReviewA11yClaim::ProviderFreshnessUnverifiedProjection
            }
            Self::DiffDriftInvalidatesFindings => {
                M5AiReviewA11yClaim::DiffScopeUnverifiedProjection
            }
            Self::PublishTargetUnavailable => {
                M5AiReviewA11yClaim::PublishTargetUnverifiedProjection
            }
            Self::LifecycleOutsidePublishSafe => {
                M5AiReviewA11yClaim::FindingLifecycleUnverifiedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state
    /// maps to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5AiReviewAssistDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5AiReviewAssistDowngradeTrigger::AiReviewAssistMatrixStale,
            Self::ProviderFreshnessStale => {
                M5AiReviewAssistDowngradeTrigger::StaleFindingShownAsCurrent
            }
            Self::DiffDriftInvalidatesFindings => {
                M5AiReviewAssistDowngradeTrigger::AnalyzedScopeUnstated
            }
            Self::PublishTargetUnavailable => M5AiReviewAssistDowngradeTrigger::PublishModeUnstated,
            Self::LifecycleOutsidePublishSafe => {
                M5AiReviewAssistDowngradeTrigger::LifecycleStateMissing
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::ProviderFreshnessStale => "provider_freshness_stale",
            Self::DiffDriftInvalidatesFindings => "diff_drift_invalidates_findings",
            Self::PublishTargetUnavailable => "publish_target_unavailable",
            Self::LifecycleOutsidePublishSafe => "lifecycle_outside_publish_safe",
        }
    }
}

/// One AI-review-truth dimension's observed condition on an object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5AiReviewClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5AiReviewConditionState,
}

/// An honest claim auto-narrow block. When an AI-review-truth dimension weakens, the object's claim lowers
/// to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the canonical
/// object identity / last-known state rather than silently dropping it — the underlying finding, scope,
/// publish, and lifecycle truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewClaimAutoNarrow {
    /// The claim the object is narrowed to.
    pub narrowed_to: M5AiReviewA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5AiReviewClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5AiReviewAssistDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical object identity and last-known state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying finding / scope / publish / lifecycle truth is preserved (never dropped) across the
    /// narrowing; must hold so provider-freshness-unverified, diff-scope-unverified,
    /// publish-target-unverified, and finding-lifecycle-unverified states never fail opaquely, and no local
    /// draft or evidence is lost.
    pub preserves_truth_continuity: bool,
}

impl AiReviewClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and finding / scope /
    /// publish / lifecycle truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for an object's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl AiReviewCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5AiReviewRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: AiReviewNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an AI-review-assist accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AiReviewAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / CLI / export parity with no narrowing
    /// (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl AiReviewAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one AI-review-assist object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewAccessibilityRow {
    /// Record kind; must equal [`AI_REVIEW_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`AI_REVIEW_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen object this row certifies.
    pub object: M5AiReviewAssistObject,
    /// Ref to the frozen per-object domain schema this row certifies.
    pub source_object_schema_ref: String,
    /// Opaque ref to the object this row represents; stays visible on every surface, so this is never empty.
    pub object_context_ref: String,
    /// Rendered modalities offered; a structure-heavy object must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5AiReviewFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical object identity, finding class / severity,
    /// analyzed scope, publish mode / provider destination, local-versus-provider state, and finding
    /// lifecycle state as the rich object; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: AiReviewNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: AiReviewNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: AiReviewNonVisualReachState,
    /// High-contrast / forced-colors behavior of the non-visual path.
    pub high_contrast_reach: AiReviewNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: AiReviewNonVisualReachState,
    /// Whether the export-safe summary preserves object meaning.
    pub export_summary: AiReviewExportSummaryState,
    /// Ref to the export-safe summary object for this object.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: AiReviewCopyExportParity,
    /// The full claim this object asserts when every dimension is intact.
    pub full_ready_claim: M5AiReviewA11yClaim,
    /// The observed condition of each modeled AI-review-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<AiReviewClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the object's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<AiReviewClaimAutoNarrow>,
    /// Whether the underlying finding / scope / publish / lifecycle truth is preserved on this object
    /// regardless of narrowing; must hold so every unverified projection never fails opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this object is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5AiReviewRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<AiReviewRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5AiReviewAssistRequiredLabel>,
    /// Semantic consumer surfaces this object is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5AiReviewAssistConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl AiReviewAccessibilityRow {
    /// Returns true when this object renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        object_is_structure_heavy(self.object)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(&self, dimension: M5AiReviewClaimDimension) -> M5AiReviewConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5AiReviewConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// object's full claim.
    pub fn permitted_claim(&self) -> M5AiReviewA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows below
    /// the object's full claim.
    pub fn binding_condition(&self) -> Option<&AiReviewClaimConditionEntry> {
        let mut binding: Option<(&AiReviewClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5AiReviewClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this object effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5AiReviewA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale-provider finding, a diff-drifted scope, an unavailable publish
    /// target, or an outdated / suppressed lifecycle state can no longer keep an old `TrustedReviewSurface` /
    /// `ReviewableReviewSurface` label. The effective claim never exceeds the permitted ceiling; when a
    /// dimension narrows below the full claim, an honest narrow block is present, narrows to exactly the
    /// permitted ceiling, binds to the ceiling-imposing dimension with its frozen trigger, and preserves
    /// canonical identity and truth. When nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / trusted honesty: a stale-provider / diff-drifted / publish-target-unavailable /
    /// lifecycle-degraded state never keeps a trusted claim — AI review never auto-approves or auto-merges
    /// from a narrowed object. When such a state is modeled, the effective claim must not assert
    /// `TrustedReviewSurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / high-contrast / CLI trap, a structure-heavy object offers a
    /// non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.object_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the object meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying finding / scope / publish /
    /// lifecycle truth. The row must assert `truth_preserved`, and any narrow block must preserve truth
    /// continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the object carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.high_contrast_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced interactivity
    /// and keeps its labels, so product / help / release publication stay aligned on the same narrowed
    /// state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its object's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = object_primary_dimension(self.object);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5AiReviewAssistRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> AiReviewAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return AiReviewAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            AiReviewAccessibilityStatus::NarrowedDisclosed
        } else {
            AiReviewAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == AI_REVIEW_A11Y_ROW_RECORD_KIND
            && self.schema_version == AI_REVIEW_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_object_schema_ref.trim().is_empty()
            && !self.object_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "object={object} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} high_contrast={high_contrast} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            object = self.object.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            high_contrast = self.high_contrast_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1272 AI-review-assist accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewAccessibilitySummary {
    pub row_count: usize,
    pub object_count: usize,
    pub structure_heavy_object_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`AiReviewAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AiReviewAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<AiReviewAccessibilityRow>,
}

/// Checked-in M05-1272 AI-review-assist accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AiReviewAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<AiReviewAccessibilityRow>,
    pub summary: AiReviewAccessibilitySummary,
}

impl AiReviewAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: AiReviewAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: AI_REVIEW_A11Y_SCHEMA_VERSION,
            record_kind: AI_REVIEW_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: AiReviewAccessibilitySummary {
                row_count: 0,
                object_count: 0,
                structure_heavy_object_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Objects represented by some row in this packet.
    pub fn represented_objects(&self) -> BTreeSet<M5AiReviewAssistObject> {
        self.rows.iter().map(|r| r.object).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5AiReviewClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5AiReviewConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5AiReviewA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5AiReviewAssistConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> AiReviewAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5AiReviewAssistConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&AiReviewAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                AiReviewAccessibilityStatus::Parity => green += 1,
                AiReviewAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                AiReviewAccessibilityStatus::Stranded => red += 1,
            }
        }

        AiReviewAccessibilitySummary {
            row_count: self.rows.len(),
            object_count: self.represented_objects().len(),
            structure_heavy_object_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(AiReviewAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(AiReviewAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(AiReviewAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(AiReviewAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(AiReviewAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(AiReviewAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<AiReviewAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != AI_REVIEW_A11Y_SCHEMA_VERSION {
            violations.push(AiReviewAccessibilityViolation::SchemaVersion {
                expected: AI_REVIEW_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != AI_REVIEW_A11Y_RECORD_KIND {
            violations.push(AiReviewAccessibilityViolation::RecordKind {
                expected: AI_REVIEW_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(AiReviewAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_objects = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(AiReviewAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_objects.insert(row.object);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(AiReviewAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its object's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(AiReviewAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: object_primary_dimension(row.object),
                });
            }

            // Each row must preserve every mandatory object label.
            if !row.preserves_mandatory_labels() {
                violations.push(AiReviewAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A structure-heavy object must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5AiReviewFallbackModality::Structured)
            {
                violations.push(
                    AiReviewAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(AiReviewAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: a stale-provider / diff-drifted / publish-target-unavailable /
            // lifecycle-degraded state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(AiReviewAccessibilityViolation::WeakStateShownAsTrusted {
                    id: row.row_id.clone(),
                });
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(AiReviewAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(AiReviewAccessibilityViolation::ExportRequiresRawPayload {
                    id: row.row_id.clone(),
                });
            }

            // AC / no-loss: weakened states preserve finding / scope / publish / lifecycle truth.
            if !row.preserves_truth_continuity() {
                violations.push(AiReviewAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    AiReviewAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(AiReviewAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == AiReviewAccessibilityStatus::Stranded {
                violations.push(AiReviewAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen object is certified at least once.
        for object in M5AiReviewAssistObject::ALL {
            if !seen_objects.contains(&object) {
                violations.push(AiReviewAccessibilityViolation::MissingObjectCoverage { object });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5AiReviewClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(AiReviewAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5AiReviewConditionState::ALL {
            if !states.contains(&state) {
                violations
                    .push(AiReviewAccessibilityViolation::MissingConditionStateCoverage { state });
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → finding-lifecycle-unverified) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5AiReviewA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(AiReviewAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Trusted honesty must be proven with at least one stale-provider / diff-drifted /
        // publish-target-unavailable / lifecycle-degraded row in the packet, so the "cannot-prove never
        // shown as trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(AiReviewAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the review detail, AI panel, finding row, scope
        // selector, publish sheet, pending-review tray, provider publish review, resolution memory ledger,
        // and support / export packet — so every consumer surface is exercised at least once.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5AiReviewAssistConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    AiReviewAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(AiReviewAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("ai-review accessibility parity packet serializes"),
        ) {
            violations.push(AiReviewAccessibilityViolation::RawObjectMaterialInExport);
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
            .expect("ai-review accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,object,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{object},{keyboard},{screen_reader},{high_zoom},{high_contrast},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                object = row.object.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                high_contrast = row.high_contrast_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 AI-Review-Assist Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Objects: {} certified across {} / {} frozen objects\n",
            self.summary.object_count,
            self.represented_objects().len(),
            M5AiReviewAssistObject::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.object.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in AI-review-assist accessibility parity export.
pub fn current_m5_ai_review_accessibility_parity_export(
) -> Result<AiReviewAccessibilityPacket, AiReviewAccessibilityArtifactError> {
    let packet: AiReviewAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/review/m5-ai-review-accessibility-parity/support_export.json"
    )))
    .map_err(AiReviewAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AiReviewAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in AI-review-assist accessibility parity export.
#[derive(Debug)]
pub enum AiReviewAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AiReviewAccessibilityViolation>),
}

impl fmt::Display for AiReviewAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "ai-review accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "ai-review accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for AiReviewAccessibilityArtifactError {}

/// Validation failure for M05-1272 AI-review-assist accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AiReviewAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5AiReviewClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingObjectCoverage {
        object: M5AiReviewAssistObject,
    },
    MissingDimensionCoverage {
        dimension: M5AiReviewClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5AiReviewConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5AiReviewA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5AiReviewAssistConsumerSurface,
    },
    SummaryMismatch,
    RawObjectMaterialInExport,
}

impl AiReviewAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingObjectCoverage { .. } => "missing_object_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawObjectMaterialInExport => "raw_object_material_in_export",
        }
    }
}

impl fmt::Display for AiReviewAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its object's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory object label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows a stale-provider / diff-drifted / publish-target-unavailable / lifecycle-degraded state as a trusted review surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / high-contrast / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve finding / scope / publish / lifecycle truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingObjectCoverage { object } => {
                write!(f, "object {object:?} is not certified in the packet")
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no stale-provider / diff-drifted / publish-target-unavailable / lifecycle-degraded row is present to prove the trusted-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawObjectMaterialInExport => {
                write!(f, "export contains raw object material")
            }
        }
    }
}

impl Error for AiReviewAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const AI_REVIEW_A11Y_PACKET_ID: &str = "m5-ai-review-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in AI-review-assist accessibility parity packet. This is the one source of
/// truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_ai_review_accessibility_parity_packet() -> AiReviewAccessibilityPacket {
    AiReviewAccessibilityPacket::new(AiReviewAccessibilityPacketInput {
        packet_id: AI_REVIEW_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-16T00:00:00Z".to_owned(),
        matrix_ref: AI_REVIEW_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:ai-review-accessibility-parity:{id}")]
}

fn all_required_labels() -> Vec<M5AiReviewAssistRequiredLabel> {
    M5AiReviewAssistRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> AiReviewCopyExportParity {
    AiReviewCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5AiReviewClaimDimension,
    state: M5AiReviewConditionState,
) -> AiReviewClaimConditionEntry {
    AiReviewClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — the support / export packet and the review
/// detail surface — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5AiReviewAssistConsumerSurface],
) -> Vec<M5AiReviewAssistConsumerSurface> {
    let mut out = vec![
        M5AiReviewAssistConsumerSurface::SupportExportPacket,
        M5AiReviewAssistConsumerSurface::ReviewDetail,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions it drops
/// there.
fn surface_disclosures(
    labels: &[&str],
    state: AiReviewNarrowingDisclosureState,
) -> Vec<AiReviewRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        AiReviewRenderingNarrowingDisclosure {
            rendering_surface: M5AiReviewRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        AiReviewRenderingNarrowingDisclosure {
            rendering_surface: M5AiReviewRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_publish_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<AiReviewRenderingNarrowingDisclosure> {
    surface_disclosures(labels, AiReviewNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<AiReviewRenderingNarrowingDisclosure> {
    surface_disclosures(labels, AiReviewNarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5AiReviewRenderingSurface> {
    vec![
        M5AiReviewRenderingSurface::DesktopFull,
        M5AiReviewRenderingSurface::CliHeadless,
        M5AiReviewRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5AiReviewFallbackModality> {
    vec![
        M5AiReviewFallbackModality::List,
        M5AiReviewFallbackModality::Textual,
        M5AiReviewFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5AiReviewFallbackModality> {
    vec![
        M5AiReviewFallbackModality::Structured,
        M5AiReviewFallbackModality::List,
        M5AiReviewFallbackModality::Textual,
        M5AiReviewFallbackModality::Cli,
    ]
}

const REACHABLE: AiReviewNonVisualReachState = AiReviewNonVisualReachState::ReachableAndLabeled;
const REDUCED: AiReviewNonVisualReachState =
    AiReviewNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<AiReviewAccessibilityRow> {
    vec![
        // AI review finding row (fresh, live, scoped) — the finding row keeps its finding class / severity,
        // analyzed scope, and provider freshness current, so it is a trusted, publish-safe review surface
        // reachable on every surface with no narrowing (green). Keyboard-only and screen-reader users can
        // inspect, rerun, dismiss, publish, export, and reopen it without losing scope or lifecycle truth.
        AiReviewAccessibilityRow {
            record_kind: AI_REVIEW_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: AI_REVIEW_A11Y_SCHEMA_VERSION,
            row_id: "a11y:ai-review-finding-row-fresh-and-scoped".to_owned(),
            object: M5AiReviewAssistObject::AiReviewFindingRow,
            source_object_schema_ref: M5AiReviewAssistObject::AiReviewFindingRow
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:ai-review-finding-row:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: AiReviewExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:ai-review-finding-row-fresh-and-scoped:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "finding_class_and_severity",
                "analyzed_scope",
                "finding_lifecycle_state",
            ]),
            full_ready_claim: M5AiReviewA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5AiReviewClaimDimension::ProviderFreshnessClarity,
                M5AiReviewConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "finding_class_and_severity",
                "analyzed_scope",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5AiReviewAssistConsumerSurface::FindingRow,
                M5AiReviewAssistConsumerSurface::AiReviewPanel,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 §15.20 — AI review finding row".to_owned(),
                AI_REVIEW_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("ai-review-finding-row-fresh-and-scoped"),
        },
        // Resolution memory row (live lifecycle) — structure-heavy (a durable lifecycle history); it keeps
        // its finding lifecycle state live and publish-safe, so it is a self-sufficient reviewable review
        // surface a user can inspect, with full parity on every surface (green). Its structured history
        // binds to a flat list / textual path.
        AiReviewAccessibilityRow {
            record_kind: AI_REVIEW_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: AI_REVIEW_A11Y_SCHEMA_VERSION,
            row_id: "a11y:resolution-memory-row-live-lifecycle".to_owned(),
            object: M5AiReviewAssistObject::ResolutionMemoryRow,
            source_object_schema_ref: M5AiReviewAssistObject::ResolutionMemoryRow
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:resolution-memory-row:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: AiReviewExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:resolution-memory-row-live-lifecycle:a11y".to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "finding_lifecycle_state",
                "resolution_actor_and_source",
                "reopen_action",
            ]),
            full_ready_claim: M5AiReviewA11yClaim::ReviewableReviewSurface,
            claim_conditions: vec![condition(
                M5AiReviewClaimDimension::FindingLifecycleClarity,
                M5AiReviewConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "object_identity",
                "finding_lifecycle_state",
                "resolution_actor_and_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5AiReviewAssistConsumerSurface::ResolutionMemoryLedger,
                M5AiReviewAssistConsumerSurface::AiReviewPanel,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.8.14 — Resolution memory row".to_owned(),
                AI_REVIEW_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("resolution-memory-row-live-lifecycle"),
        },
        // AI review finding row (provider freshness stale) — the finding's provider freshness is stale, so
        // it auto-narrows to a provider-freshness-unverified projection that keeps the last-known finding and
        // its analyzed scope visible without presenting a stale finding as a current, provider-committed
        // truth (yellow). Its screen-reader traversal discloses a reduced linear walk.
        AiReviewAccessibilityRow {
            record_kind: AI_REVIEW_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: AI_REVIEW_A11Y_SCHEMA_VERSION,
            row_id: "a11y:ai-review-finding-row-provider-freshness-stale".to_owned(),
            object: M5AiReviewAssistObject::AiReviewFindingRow,
            source_object_schema_ref: M5AiReviewAssistObject::AiReviewFindingRow
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:ai-review-finding-row:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: AiReviewExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:ai-review-finding-row-provider-freshness-stale:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "finding_class_and_severity",
                "local_versus_provider_state",
                "last_known_provider_freshness",
            ]),
            full_ready_claim: M5AiReviewA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5AiReviewClaimDimension::ProviderFreshnessClarity,
                M5AiReviewConditionState::ProviderFreshnessStale,
            )],
            claim_narrow: Some(AiReviewClaimAutoNarrow {
                narrowed_to: M5AiReviewA11yClaim::ProviderFreshnessUnverifiedProjection,
                binding_dimension: M5AiReviewClaimDimension::ProviderFreshnessClarity,
                trigger: M5AiReviewAssistDowngradeTrigger::StaleFindingShownAsCurrent,
                narrowed_label:
                    "This AI review finding's provider freshness is stale — shown as a provider-freshness-unverified projection that keeps the last-known finding class, severity, and analyzed scope visible, never presenting a stale finding as a current, provider-committed truth or auto-approving from it"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "finding_class_and_severity",
                "local_versus_provider_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5AiReviewAssistConsumerSurface::FindingRow,
                M5AiReviewAssistConsumerSurface::PendingReviewTray,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.9.16.2 — Provider-linked freshness".to_owned(),
                AI_REVIEW_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("ai-review-finding-row-provider-freshness-stale"),
        },
        // Review scope selector (diff drift invalidates findings) — diff drift invalidates prior findings,
        // so the scope selector auto-narrows to a diff-scope-unverified projection that keeps the last-known
        // analyzed scope explicit and recommends an in-scope rerun, never letting a diff-drifted finding read
        // as current (yellow). Its dense reflow narrows the high-zoom legibility to a disclosed reduction.
        AiReviewAccessibilityRow {
            record_kind: AI_REVIEW_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: AI_REVIEW_A11Y_SCHEMA_VERSION,
            row_id: "a11y:review-scope-selector-diff-drift-invalidates-findings".to_owned(),
            object: M5AiReviewAssistObject::ReviewScopeSelector,
            source_object_schema_ref: M5AiReviewAssistObject::ReviewScopeSelector
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:review-scope-selector:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: AiReviewExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:review-scope-selector-diff-drift-invalidates-findings:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "analyzed_scope",
                "rerun_within_scope_action",
                "last_known_analyzed_scope",
            ]),
            full_ready_claim: M5AiReviewA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5AiReviewClaimDimension::DiffScopeDriftClarity,
                M5AiReviewConditionState::DiffDriftInvalidatesFindings,
            )],
            claim_narrow: Some(AiReviewClaimAutoNarrow {
                narrowed_to: M5AiReviewA11yClaim::DiffScopeUnverifiedProjection,
                binding_dimension: M5AiReviewClaimDimension::DiffScopeDriftClarity,
                trigger: M5AiReviewAssistDowngradeTrigger::AnalyzedScopeUnstated,
                narrowed_label:
                    "This review scope selector's analyzed diff has drifted — shown as a diff-scope-unverified projection that keeps the last-known analyzed scope explicit and recommends an in-scope rerun, never letting a diff-drifted finding read as current or silently widening scope"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "analyzed_scope",
                "rerun_within_scope_action",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5AiReviewAssistConsumerSurface::ReviewScopeSelector,
                M5AiReviewAssistConsumerSurface::AiReviewPanel,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 §15.20 — Diff-scoped checks".to_owned(),
                AI_REVIEW_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("review-scope-selector-diff-drift-invalidates-findings"),
        },
        // Publish-to-review sheet (publish target unavailable) — structure-heavy (an outbound action set);
        // the provider publish target is unavailable, so it auto-narrows to a publish-target-unverified
        // projection that keeps the local draft and export fallback explicit, never losing the local draft or
        // showing it as a provider-committed publish (yellow).
        AiReviewAccessibilityRow {
            record_kind: AI_REVIEW_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: AI_REVIEW_A11Y_SCHEMA_VERSION,
            row_id: "a11y:publish-to-review-sheet-publish-target-unavailable".to_owned(),
            object: M5AiReviewAssistObject::PublishToReviewSheet,
            source_object_schema_ref: M5AiReviewAssistObject::PublishToReviewSheet
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:publish-to-review-sheet:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: AiReviewExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:publish-to-review-sheet-publish-target-unavailable:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "publish_destination",
                "local_versus_provider_state",
                "export_fallback",
            ]),
            full_ready_claim: M5AiReviewA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5AiReviewClaimDimension::PublishTargetAvailabilityClarity,
                M5AiReviewConditionState::PublishTargetUnavailable,
            )],
            claim_narrow: Some(AiReviewClaimAutoNarrow {
                narrowed_to: M5AiReviewA11yClaim::PublishTargetUnverifiedProjection,
                binding_dimension: M5AiReviewClaimDimension::PublishTargetAvailabilityClarity,
                trigger: M5AiReviewAssistDowngradeTrigger::PublishModeUnstated,
                narrowed_label:
                    "This publish-to-review sheet's provider publish target is unavailable — shown as a publish-target-unverified projection that keeps the local draft and export / copy fallback explicit, never losing the local draft or showing it as a provider-committed publish"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "publish_destination",
                "local_versus_provider_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5AiReviewAssistConsumerSurface::PublishToReviewSheet,
                M5AiReviewAssistConsumerSurface::ProviderPublishReview,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.9.16.2 — Publish-later continuity".to_owned(),
                AI_REVIEW_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("publish-to-review-sheet-publish-target-unavailable"),
        },
        // Resolution memory row (lifecycle outside publish-safe) — structure-heavy (a durable lifecycle
        // history); the finding's lifecycle state falls outside live publish-safe conditions (outdated /
        // suppressed), so it auto-narrows to a finding-lifecycle-unverified projection that discloses the
        // outdated / suppressed lifecycle state, never showing an outdated or suppressed finding as live and
        // publish-safe (yellow).
        AiReviewAccessibilityRow {
            record_kind: AI_REVIEW_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: AI_REVIEW_A11Y_SCHEMA_VERSION,
            row_id: "a11y:resolution-memory-row-lifecycle-outside-publish-safe".to_owned(),
            object: M5AiReviewAssistObject::ResolutionMemoryRow,
            source_object_schema_ref: M5AiReviewAssistObject::ResolutionMemoryRow
                .canonical_domain_schema_ref()
                .to_owned(),
            object_context_ref: "review:resolution-memory-row:0006".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: AiReviewExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:resolution-memory-row-lifecycle-outside-publish-safe:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "object_identity",
                "finding_lifecycle_state",
                "reopen_action",
                "last_known_lifecycle_state",
            ]),
            full_ready_claim: M5AiReviewA11yClaim::TrustedReviewSurface,
            claim_conditions: vec![condition(
                M5AiReviewClaimDimension::FindingLifecycleClarity,
                M5AiReviewConditionState::LifecycleOutsidePublishSafe,
            )],
            claim_narrow: Some(AiReviewClaimAutoNarrow {
                narrowed_to: M5AiReviewA11yClaim::FindingLifecycleUnverifiedProjection,
                binding_dimension: M5AiReviewClaimDimension::FindingLifecycleClarity,
                trigger: M5AiReviewAssistDowngradeTrigger::LifecycleStateMissing,
                narrowed_label:
                    "This resolution memory row's finding lifecycle state is outdated or suppressed — shown as a finding-lifecycle-unverified projection that discloses the outdated / suppressed lifecycle state and keeps the reopen action, never showing an outdated or suppressed finding as live and publish-safe"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "object_identity",
                "finding_lifecycle_state",
                "reopen_action",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5AiReviewAssistConsumerSurface::ResolutionMemoryLedger,
                M5AiReviewAssistConsumerSurface::PendingReviewTray,
            ]),
            source_refs: vec![
                "TDD v3.6 §7.8.14 — Resolution memory lifecycle".to_owned(),
                AI_REVIEW_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-16T00:00:00Z".to_owned(),
            evidence_refs: ev("resolution-memory-row-lifecycle-outside-publish-safe"),
        },
    ]
}
