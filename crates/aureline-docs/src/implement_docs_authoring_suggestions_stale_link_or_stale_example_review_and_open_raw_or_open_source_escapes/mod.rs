//! Docs authoring suggestions, stale-link / stale-example review, and
//! open-raw / open-source escapes.
//!
//! This module implements the M5 docs authoring-and-review boundary: the
//! records that let a docs review item — an *authoring suggestion*, a
//! *stale-link review*, or a *stale-example review* — carry an explicit apply
//! posture (whether and how a suggested edit may be applied), an explicit
//! staleness verdict (whether a link or example is current, redirected, drifted,
//! or broken), one trust-class disclosure, the shared source/version/freshness/
//! locality/confidence chip set, and the open-raw / open-source escapes — without
//! ever letting an unverified suggestion present a one-click apply, a stale link
//! or example look current, or an item strand the reader without a path back to
//! the underlying node and upstream source.
//!
//! Each [`DocsReviewItem`] carries one [`DocsReviewItemKind`] (`authoring_suggestion`
//! / `stale_link_review` / `stale_example_review` / `freshness_review`), an
//! [`AuthoringSuggestion`] block (the suggested edit, its [`SuggestionApplyPosture`],
//! and the [`SuggestionTrigger`] that raised it), a [`StaleReviewVerdict`] block
//! (the [`ReviewFindingClass`] and its severity — the stale-link / stale-example
//! review truth), one [`DocsReviewTrustClass`] disclosure, the five-chip set, the
//! live-vs-captured state, citation state, and the open-raw / open-source escapes.
//!
//! Three invariants make a docs review item honest:
//!
//! - **Authoring-suggestion apply-posture truth.** A suggestion may present a
//!   one-click apply only when its origin is verified (an unverified live-mirror
//!   or derived suggestion may not), and never when the item's own review verdict
//!   is a blocking stale finding — a broken or uncompilable example never offers
//!   apply-available.
//! - **Stale-link / stale-example review truth.** A verdict marked stale may not
//!   claim live-authoritative freshness, and a non-current version may not be
//!   presented as a confident live match — a stale link or example always reads
//!   as stale.
//! - **Open-raw / open-source escape preservation.** Every item keeps an open-raw
//!   escape (open the underlying doc node) and an open-source escape (open the
//!   upstream source), so the reader can always leave the qualified surface for
//!   the raw truth.
//!
//! The [`DocsAuthoringReviewExport`] is the projection support, AI evidence, and
//! diagnostics surfaces ingest: one [`DocsAuthoringReviewExportRow`] per item
//! preserving item kind, trust class, source class, confidence, apply posture,
//! finding class, review severity, citation state, and the open-raw / open-source
//! escapes.
//!
//! [`DocsAuthoringReviewPacket::materialize`] computes the validation findings and
//! the promotion state (`stable`, `narrowed_below_stable`, or `blocks_stable`)
//! from the input — folding the review-verdict and degradation severities into the
//! promotion decision — so a verified set whose examples re-validate stays Stable,
//! a set with a narrowing drift narrows below Stable, and a set with a broken link,
//! an uncompilable example, an unverified one-click apply, or a truth collapse
//! blocks before it reaches a consumer surface. The packet is an inspectable,
//! serde-serializable truth packet: it carries no raw document bodies, no raw
//! source files, no raw URLs, no diff bodies, no raw provider payloads, and no
//! credentials — only metadata, apply-posture truth, staleness truth, chip truth,
//! cited refs, provenance, finding summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/implement-docs-authoring-suggestions-stale-link-or-stale-example-review-and-open-raw-or-open-source-escapes.schema.json`](../../../../schemas/docs/implement-docs-authoring-suggestions-stale-link-or-stale-example-review-and-open-raw-or-open-source-escapes.schema.json).
//! The contract doc is
//! [`docs/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes.md`](../../../../docs/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/`](../../../../fixtures/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DocsAuthoringReviewPacket`].
pub const DOCS_AUTHORING_REVIEW_RECORD_KIND: &str = "docs_authoring_review_controls";

/// Record-kind tag carried by the support-export wrapper.
pub const DOCS_AUTHORING_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_authoring_review_controls_support_export";

/// Schema version for docs-authoring-review records.
pub const DOCS_AUTHORING_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_AUTHORING_REVIEW_SCHEMA_REF: &str =
    "schemas/docs/implement-docs-authoring-suggestions-stale-link-or-stale-example-review-and-open-raw-or-open-source-escapes.schema.json";

/// Repo-relative path of the docs-authoring-review contract doc.
pub const DOCS_AUTHORING_REVIEW_DOC_REF: &str =
    "docs/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_AUTHORING_REVIEW_FIXTURE_DIR: &str =
    "fixtures/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_AUTHORING_REVIEW_ARTIFACT_REF: &str =
    "artifacts/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_AUTHORING_REVIEW_SUMMARY_REF: &str =
    "artifacts/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes.md";

/// Kind of docs review item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewItemKind {
    /// A suggested authoring edit to a docs node.
    AuthoringSuggestion,
    /// A review of a link that may be stale (broken or redirected).
    StaleLinkReview,
    /// A review of an example that may be stale (drifted from the code).
    StaleExampleReview,
    /// A review of a docs node's freshness against the active build.
    FreshnessReview,
}

impl DocsReviewItemKind {
    /// The item kinds a packet must cover.
    pub const REQUIRED: [Self; 3] = [
        Self::AuthoringSuggestion,
        Self::StaleLinkReview,
        Self::StaleExampleReview,
    ];

    /// Stable token recorded in the item.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoringSuggestion => "authoring_suggestion",
            Self::StaleLinkReview => "stale_link_review",
            Self::StaleExampleReview => "stale_example_review",
            Self::FreshnessReview => "freshness_review",
        }
    }
}

/// Whether and how an authoring suggestion may be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionApplyPosture {
    /// The suggested edit is shown but a preview is required before applying.
    PreviewRequired,
    /// A one-click apply action is available and explicit.
    ApplyAvailable,
    /// Applying is blocked by policy.
    ApplyBlockedByPolicy,
    /// The item is advisory only; no apply action is offered.
    SuggestionOnly,
    /// Applying is unavailable and disclosed as such.
    ApplyUnavailableDisclosed,
}

impl SuggestionApplyPosture {
    /// Stable token recorded in the suggestion.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRequired => "preview_required",
            Self::ApplyAvailable => "apply_available",
            Self::ApplyBlockedByPolicy => "apply_blocked_by_policy",
            Self::SuggestionOnly => "suggestion_only",
            Self::ApplyUnavailableDisclosed => "apply_unavailable_disclosed",
        }
    }

    /// Whether this posture presents a one-click apply action.
    pub const fn offers_one_click_apply(self) -> bool {
        matches!(self, Self::ApplyAvailable)
    }
}

/// The trigger that raised an authoring suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestionTrigger {
    /// A manual authoring edit by a human.
    ManualAuthoring,
    /// A stale example was detected.
    StaleExampleDetected,
    /// A broken link was detected.
    BrokenLinkDetected,
    /// A version drift was detected.
    VersionDriftDetected,
    /// A style / lint hint.
    StyleLintHint,
    /// An AI authoring assist.
    AiAuthoringAssist,
}

impl SuggestionTrigger {
    /// Stable token recorded in the suggestion.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManualAuthoring => "manual_authoring",
            Self::StaleExampleDetected => "stale_example_detected",
            Self::BrokenLinkDetected => "broken_link_detected",
            Self::VersionDriftDetected => "version_drift_detected",
            Self::StyleLintHint => "style_lint_hint",
            Self::AiAuthoringAssist => "ai_authoring_assist",
        }
    }
}

/// The verdict of a stale-link / stale-example / freshness review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewFindingClass {
    /// The link / example / node re-validated as current.
    FreshOk,
    /// A link is broken (the target no longer resolves).
    StaleLinkBroken,
    /// A link redirects to a new target.
    StaleLinkRedirected,
    /// An example has drifted from the code it documents.
    StaleExampleDrifted,
    /// An example no longer compiles / runs.
    StaleExampleUncompilable,
    /// An example targets a version that no longer matches the build.
    StaleExampleVersionMismatch,
    /// The item still needs review.
    NeedsReview,
    /// The owning source is quarantined.
    QuarantinedSource,
}

impl ReviewFindingClass {
    /// Stable token recorded in the verdict.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshOk => "fresh_ok",
            Self::StaleLinkBroken => "stale_link_broken",
            Self::StaleLinkRedirected => "stale_link_redirected",
            Self::StaleExampleDrifted => "stale_example_drifted",
            Self::StaleExampleUncompilable => "stale_example_uncompilable",
            Self::StaleExampleVersionMismatch => "stale_example_version_mismatch",
            Self::NeedsReview => "needs_review",
            Self::QuarantinedSource => "quarantined_source",
        }
    }

    /// Whether this verdict reports a stale (non-current) link or example. A
    /// stale verdict may never claim live-authoritative freshness.
    pub const fn is_stale(self) -> bool {
        matches!(
            self,
            Self::StaleLinkBroken
                | Self::StaleLinkRedirected
                | Self::StaleExampleDrifted
                | Self::StaleExampleUncompilable
                | Self::StaleExampleVersionMismatch
                | Self::QuarantinedSource
        )
    }
}

/// Trust class of a docs review item's origin, projected as a disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewTrustClass {
    /// First-party docs authored in-repo.
    FirstPartyAuthored,
    /// A signed first-party docs pack.
    SignedDocsPack,
    /// A docs pack imported from a signed extension set.
    ImportedDocsPack,
    /// A live-mirror suggestion — not verified at materialization time.
    LiveMirrorSuggestion,
    /// A derived / inferred suggestion only.
    DerivedHeuristicOnly,
}

impl DocsReviewTrustClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyAuthored => "first_party_authored",
            Self::SignedDocsPack => "signed_docs_pack",
            Self::ImportedDocsPack => "imported_docs_pack",
            Self::LiveMirrorSuggestion => "live_mirror_suggestion",
            Self::DerivedHeuristicOnly => "derived_heuristic_only",
        }
    }

    /// Whether this trust class may back a high-confidence / authoritative claim
    /// or a one-click apply. A live-mirror or derived suggestion may not.
    pub const fn may_be_authoritative(self) -> bool {
        matches!(
            self,
            Self::FirstPartyAuthored | Self::SignedDocsPack | Self::ImportedDocsPack
        )
    }

    /// Whether an item of this trust class must stay cited.
    pub const fn needs_citation(self) -> bool {
        !matches!(self, Self::FirstPartyAuthored)
    }
}

/// Whether the item is live, a captured snapshot, or a narrowed rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturedVsLive {
    /// A live item.
    Live,
    /// A captured snapshot of an earlier review.
    CapturedSnapshot,
    /// A rerun narrowed to a smaller scope.
    NarrowedScopeRerun,
}

impl CapturedVsLive {
    /// Stable token recorded in the item.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::NarrowedScopeRerun => "narrowed_scope_rerun",
        }
    }
}

/// Source class for a review item's underlying material, projected as the chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewSourceClass {
    /// A first-party doc authored in-repo.
    FirstPartyDoc,
    /// A signed first-party docs pack.
    SignedDocsPack,
    /// An imported / extension docs pack.
    ImportedDocsPack,
    /// A mirrored vendor doc.
    MirroredVendorDoc,
    /// A live mirror.
    LiveMirror,
    /// A derived / inferred suggestion.
    DerivedHeuristic,
}

impl DocsReviewSourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyDoc => "first_party_doc",
            Self::SignedDocsPack => "signed_docs_pack",
            Self::ImportedDocsPack => "imported_docs_pack",
            Self::MirroredVendorDoc => "mirrored_vendor_doc",
            Self::LiveMirror => "live_mirror",
            Self::DerivedHeuristic => "derived_heuristic",
        }
    }
}

/// Version-match state for an item, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewVersionMatch {
    /// Item matches the active build/workspace revision exactly.
    ExactBuildMatch,
    /// Item is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Item drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release item has not completed verification.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl DocsReviewVersionMatch {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactBuildMatch => "exact_build_match",
            Self::CompatibleMinorDrift => "compatible_minor_drift",
            Self::IncompatibleDriftDetected => "incompatible_drift_detected",
            Self::PreReleaseUnverified => "pre_release_unverified",
            Self::UnknownTargetBuild => "unknown_target_build",
        }
    }

    /// Whether this state may be presented as a confident current-version match.
    pub const fn is_confident_current(self) -> bool {
        matches!(self, Self::ExactBuildMatch)
    }
}

/// Freshness state for an item, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewFreshness {
    /// Item was live and authoritative at materialization time.
    AuthoritativeLive,
    /// Cached item within its freshness window.
    WarmCached,
    /// Cached item usable only with degraded disclosure.
    DegradedCached,
    /// Item is stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the source has not yet re-synced.
    RefreshPending,
}

impl DocsReviewFreshness {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoritativeLive => "authoritative_live",
            Self::WarmCached => "warm_cached",
            Self::DegradedCached => "degraded_cached",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::RefreshPending => "refresh_pending",
        }
    }

    /// Whether this state may claim live authoritative freshness.
    pub const fn is_authoritative_live(self) -> bool {
        matches!(self, Self::AuthoritativeLive)
    }
}

/// Locality / install posture for an item, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through an imported pack.
    ImportedPack,
    /// Resolved through a mirrored pack.
    MirroredPack,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl DocsReviewLocality {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::ImportedPack => "imported_pack",
            Self::MirroredPack => "mirrored_pack",
            Self::Managed => "managed",
        }
    }
}

/// Confidence label for an item, projected as the confidence chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified claim.
    Heuristic,
}

impl DocsReviewConfidence {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Medium => "medium",
            Self::Low => "low",
            Self::Heuristic => "heuristic",
        }
    }
}

/// Severity of a degradation, review verdict, or validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewFindingSeverity {
    /// Blocks a Stable claim; the set must block.
    Blocking,
    /// Narrows below Stable but the set stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl DocsReviewFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the docs-authoring-review packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewConsumerSurface {
    /// The docs authoring surface.
    DocsAuthoringSurface,
    /// The docs browser shell.
    DocsBrowserShell,
    /// The docs review panel.
    DocsReviewPanel,
    /// The stale-example review queue.
    StaleExampleReviewQueue,
    /// The AI context inspector.
    AiContextInspector,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl DocsReviewConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsAuthoringSurface => "docs_authoring_surface",
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::DocsReviewPanel => "docs_review_panel",
            Self::StaleExampleReviewQueue => "stale_example_review_queue",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level docs-review degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewDegradationClass {
    /// A mirror is offline; the item is served from the last snapshot.
    MirrorOfflineSnapshot,
    /// The example harness is unavailable, so examples could not be re-validated.
    ExampleHarnessUnavailable,
    /// The link checker is offline, so links could not be re-verified.
    LinkCheckerOffline,
    /// The suggestion engine is degraded.
    SuggestionEngineDegraded,
    /// The review was rerun at a narrowed scope.
    ScopeNarrowedRerun,
    /// The review claim was narrowed before publication.
    ReviewNarrowed,
    /// The owning source is quarantined.
    QuarantinedSource,
    /// A referenced anchor is broken.
    BrokenAnchor,
}

impl DocsReviewDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::ExampleHarnessUnavailable => "example_harness_unavailable",
            Self::LinkCheckerOffline => "link_checker_offline",
            Self::SuggestionEngineDegraded => "suggestion_engine_degraded",
            Self::ScopeNarrowedRerun => "scope_narrowed_rerun",
            Self::ReviewNarrowed => "review_narrowed",
            Self::QuarantinedSource => "quarantined_source",
            Self::BrokenAnchor => "broken_anchor",
        }
    }
}

/// Scope a docs-review export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewExportScope {
    /// Every item in the packet.
    AllItems,
    /// Authoring suggestions only.
    SuggestionsOnly,
    /// Stale reviews only.
    ReviewsOnly,
    /// Stale (non-current) items only.
    StaleOnly,
}

impl DocsReviewExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllItems => "all_items",
            Self::SuggestionsOnly => "suggestions_only",
            Self::ReviewsOnly => "reviews_only",
            Self::StaleOnly => "stale_only",
        }
    }
}

/// Promotion state computed for the docs-authoring-review packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewPromotionState {
    /// Set qualifies for the Stable claim.
    Stable,
    /// Set narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Set has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl DocsReviewPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`DocsAuthoringReviewPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsReviewFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The item set is empty.
    ItemsEmpty,
    /// An item id is duplicated.
    DuplicateItemId,
    /// A required item kind (suggestion / stale-link / stale-example) is missing.
    RequiredItemKindMissing,
    /// An item is missing its title or detail.
    ItemTitleOrDetailMissing,
    /// An item is missing its trust-class disclosure note.
    TrustClassDisclosureMissing,
    /// An untrusted item is presented as a high-confidence claim.
    TrustClassDisclosureCollapsed,
    /// An item is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// An imported / mirror / derived item is not cited.
    ItemNotCited,
    /// A non-current version-match is presented as a confident live match.
    VersionTruthCollapsed,
    /// An authoring suggestion is missing its apply note.
    ApplyPostureNoteMissing,
    /// An unverified (live-mirror / derived) suggestion offers a one-click apply.
    UnverifiedSuggestionApplyOffered,
    /// A one-click apply is offered while the item's own review verdict blocks.
    ApplyOfferedOnBlockingFinding,
    /// A review verdict is missing its note.
    ReviewVerdictNoteMissing,
    /// A stale verdict claims live-authoritative freshness.
    StaleVerdictFreshnessMismatch,
    /// The export drops a required preservation flag.
    ExportDropsPreservation,
    /// An export row references an item id absent from the items.
    ExportRowOrphan,
    /// An item has no matching export row.
    ExportCoverageMissing,
    /// An export row's item kind disagrees with the item.
    ExportItemKindMismatch,
    /// An export row's trust class disagrees with the item.
    ExportTrustClassMismatch,
    /// An export row's source class disagrees with the item's chip.
    ExportSourceClassMismatch,
    /// An export row's confidence disagrees with the item's chip.
    ExportConfidenceMismatch,
    /// An export row's apply posture disagrees with the item.
    ExportApplyPostureMismatch,
    /// An export row's finding class disagrees with the item.
    ExportFindingClassMismatch,
    /// An export row's cited flag disagrees with the item.
    ExportCitedMismatch,
    /// A degradation is incomplete (missing summary).
    DegradationIncomplete,
    /// A degradation references an item id absent from the items.
    DegradationOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw bodies, raw URLs, diff bodies, or secrets crossed the boundary.
    RawBoundaryMaterialPresent,
}

impl DocsReviewFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::ItemsEmpty => "items_empty",
            Self::DuplicateItemId => "duplicate_item_id",
            Self::RequiredItemKindMissing => "required_item_kind_missing",
            Self::ItemTitleOrDetailMissing => "item_title_or_detail_missing",
            Self::TrustClassDisclosureMissing => "trust_class_disclosure_missing",
            Self::TrustClassDisclosureCollapsed => "trust_class_disclosure_collapsed",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::ItemNotCited => "item_not_cited",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::ApplyPostureNoteMissing => "apply_posture_note_missing",
            Self::UnverifiedSuggestionApplyOffered => "unverified_suggestion_apply_offered",
            Self::ApplyOfferedOnBlockingFinding => "apply_offered_on_blocking_finding",
            Self::ReviewVerdictNoteMissing => "review_verdict_note_missing",
            Self::StaleVerdictFreshnessMismatch => "stale_verdict_freshness_mismatch",
            Self::ExportDropsPreservation => "export_drops_preservation",
            Self::ExportRowOrphan => "export_row_orphan",
            Self::ExportCoverageMissing => "export_coverage_missing",
            Self::ExportItemKindMismatch => "export_item_kind_mismatch",
            Self::ExportTrustClassMismatch => "export_trust_class_mismatch",
            Self::ExportSourceClassMismatch => "export_source_class_mismatch",
            Self::ExportConfidenceMismatch => "export_confidence_mismatch",
            Self::ExportApplyPostureMismatch => "export_apply_posture_mismatch",
            Self::ExportFindingClassMismatch => "export_finding_class_mismatch",
            Self::ExportCitedMismatch => "export_cited_mismatch",
            Self::DegradationIncomplete => "degradation_incomplete",
            Self::DegradationOrphan => "degradation_orphan",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::RequiredSurfaceCoverageMissing => "required_surface_coverage_missing",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
        }
    }

    /// Default severity for this finding kind. Every validation finding blocks
    /// the Stable claim; narrowing comes only from data-carried review-verdict
    /// and degradation severities so a degraded-but-honest set narrows rather
    /// than blocks.
    pub const fn default_severity(self) -> DocsReviewFindingSeverity {
        DocsReviewFindingSeverity::Blocking
    }
}

/// The chip set rendered for one docs review item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsReviewChipSet {
    /// Source-class chip.
    pub source_class: DocsReviewSourceClass,
    /// Version-match chip.
    pub version_match: DocsReviewVersionMatch,
    /// Freshness chip.
    pub freshness: DocsReviewFreshness,
    /// Locality chip.
    pub locality: DocsReviewLocality,
    /// Confidence chip (the confidence label).
    pub confidence: DocsReviewConfidence,
}

/// The authoring-suggestion block for a docs review item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthoringSuggestion {
    /// Whether and how the suggested edit may be applied.
    pub apply_posture: SuggestionApplyPosture,
    /// The trigger that raised the suggestion.
    pub trigger: SuggestionTrigger,
    /// Human-readable apply note (no raw diff bodies).
    pub note: String,
}

/// The stale-link / stale-example review verdict for a docs review item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleReviewVerdict {
    /// The review finding class.
    pub finding_class: ReviewFindingClass,
    /// The severity of the verdict (drives the promotion narrowing/blocking).
    pub severity: DocsReviewFindingSeverity,
    /// Human-readable verdict note (no raw bodies).
    pub note: String,
}

/// One docs review item — one bounded authoring-and-review record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsReviewItem {
    /// Stable item id within this packet.
    pub item_id: String,
    /// The kind of item.
    pub item_kind: DocsReviewItemKind,
    /// Subject ref the item points at (no raw body).
    pub subject_ref: String,
    /// Human-readable title.
    pub title: String,
    /// Human-readable detail / summary (no raw bodies).
    pub detail: String,
    /// Source/version/freshness/locality/confidence chips.
    pub chips: DocsReviewChipSet,
    /// The trust-class disclosure for the origin.
    pub trust_class: DocsReviewTrustClass,
    /// Human-readable trust-class disclosure note.
    pub trust_disclosure_note: String,
    /// The authoring-suggestion block (apply posture + trigger).
    pub suggestion: AuthoringSuggestion,
    /// The stale-link / stale-example review verdict.
    pub review: StaleReviewVerdict,
    /// Whether the item is live, captured, or a narrowed rerun.
    pub captured_vs_live: CapturedVsLive,
    /// Whether the item is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
    /// Open-raw escape ref (open the underlying doc node).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
}

/// One export row, mirroring an item.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsAuthoringReviewExportRow {
    /// The item this export row mirrors.
    pub item_id_ref: String,
    /// Item kind (must match the item).
    pub item_kind: DocsReviewItemKind,
    /// Trust class (must match the item).
    pub trust_class: DocsReviewTrustClass,
    /// Source class (must match the item's chip).
    pub source_class: DocsReviewSourceClass,
    /// Confidence (must match the item's chip).
    pub confidence: DocsReviewConfidence,
    /// Apply posture (must match the item's suggestion).
    pub apply_posture: SuggestionApplyPosture,
    /// Review finding class (must match the item's review).
    pub finding_class: ReviewFindingClass,
    /// Review severity (must match the item's review).
    pub review_severity: DocsReviewFindingSeverity,
    /// Whether the item is cited.
    pub cited: bool,
    /// Open-raw escape ref.
    pub open_raw_escape_ref: String,
    /// Open-source escape ref.
    pub open_source_escape_ref: String,
}

/// The docs-authoring-review export projection for the item set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsAuthoringReviewExport {
    /// Scope this export covers.
    pub scope: DocsReviewExportScope,
    /// Whether the export preserves each item's kind.
    pub preserves_item_kind: bool,
    /// Whether the export preserves each item's trust class.
    pub preserves_trust_class: bool,
    /// Whether the export preserves each item's source class.
    pub preserves_source_class: bool,
    /// Whether the export preserves each item's confidence label.
    pub preserves_confidence: bool,
    /// Whether the export preserves each item's apply posture.
    pub preserves_apply_posture: bool,
    /// Whether the export preserves each item's review finding class.
    pub preserves_finding_class: bool,
    /// Whether the export preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
    /// Per-item export rows.
    pub rows: Vec<DocsAuthoringReviewExportRow>,
}

impl DocsAuthoringReviewExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_item_kind
            && self.preserves_trust_class
            && self.preserves_source_class
            && self.preserves_confidence
            && self.preserves_apply_posture
            && self.preserves_finding_class
            && self.preserves_open_raw_open_source_escape
    }
}

/// A packet-level docs-review degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsReviewDegradation {
    /// Degradation class.
    pub degradation_class: DocsReviewDegradationClass,
    /// Severity.
    pub severity: DocsReviewFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The item this degradation annotates, if scoped to one item.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the docs-review set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsReviewConsumerProjection {
    /// Surface that consumes the set.
    pub surface: DocsReviewConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves all item kinds.
    pub preserves_item_kinds: bool,
    /// Whether the surface preserves the apply postures.
    pub preserves_apply_posture: bool,
    /// Whether the surface preserves the review finding classes.
    pub preserves_finding_class: bool,
    /// Whether the surface preserves the trust classes.
    pub preserves_trust_classes: bool,
    /// Whether the surface preserves the confidence labels.
    pub preserves_confidence: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
}

impl DocsReviewConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_item_kinds
            && self.preserves_apply_posture
            && self.preserves_finding_class
            && self.preserves_trust_classes
            && self.preserves_confidence
            && self.preserves_open_raw_open_source_escape
    }
}

/// A single validation finding on the docs-review set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsReviewValidationFinding {
    /// Finding kind.
    pub finding_kind: DocsReviewFindingKind,
    /// Finding severity.
    pub severity: DocsReviewFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`DocsAuthoringReviewPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsAuthoringReviewPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label (no raw URLs / no raw bodies).
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The docs review items.
    pub items: Vec<DocsReviewItem>,
    /// The export projection.
    pub export: DocsAuthoringReviewExport,
    /// Packet-level degradations.
    pub review_degradations: Vec<DocsReviewDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<DocsReviewConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe docs-authoring-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsAuthoringReviewPacket {
    /// Record kind; must equal [`DOCS_AUTHORING_REVIEW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_AUTHORING_REVIEW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label.
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The docs review items.
    pub items: Vec<DocsReviewItem>,
    /// The export projection.
    pub export: DocsAuthoringReviewExport,
    /// Packet-level degradations.
    pub review_degradations: Vec<DocsReviewDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<DocsReviewConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: DocsReviewPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<DocsReviewValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every docs-review packet must project.
const REQUIRED_SURFACES: [DocsReviewConsumerSurface; 4] = [
    DocsReviewConsumerSurface::DocsAuthoringSurface,
    DocsReviewConsumerSurface::DocsReviewPanel,
    DocsReviewConsumerSurface::StaleExampleReviewQueue,
    DocsReviewConsumerSurface::SupportExport,
];

impl DocsAuthoringReviewPacket {
    /// Materializes a docs-authoring-review packet, computing validation findings
    /// and the promotion state from the input.
    pub fn materialize(input: DocsAuthoringReviewPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_items(&input, &mut findings);
        check_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.items, &input.review_degradations);

        Self {
            record_kind: DOCS_AUTHORING_REVIEW_RECORD_KIND.to_owned(),
            schema_version: DOCS_AUTHORING_REVIEW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            session_label: input.session_label,
            session_digest_ref: input.session_digest_ref,
            items: input.items,
            export: input.export,
            review_degradations: input.review_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the set qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == DocsReviewPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> DocsAuthoringReviewSupportExport {
        DocsAuthoringReviewSupportExport {
            record_kind: DOCS_AUTHORING_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_AUTHORING_REVIEW_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: DOCS_AUTHORING_REVIEW_SCHEMA_REF.to_owned(),
            doc_ref: DOCS_AUTHORING_REVIEW_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs-authoring-review packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Docs Authoring Suggestions and Stale-Link / Stale-Example Review (open-raw / open-source escapes)\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Session: {}\n", self.session_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Items: {} | Degradations: {}\n",
            self.items.len(),
            self.review_degradations.len()
        ));
        out.push_str("\n## Items\n\n");
        for item in &self.items {
            out.push_str(&format!(
                "- [{}] `{}` ({}) — trust `{}` — {} / {} / {} / {} / {}\n",
                item.item_kind.as_str(),
                item.item_id,
                item.title,
                item.trust_class.as_str(),
                item.chips.source_class.as_str(),
                item.chips.version_match.as_str(),
                item.chips.freshness.as_str(),
                item.chips.locality.as_str(),
                item.chips.confidence.as_str(),
            ));
            out.push_str(&format!(
                "  - Suggestion: apply `{}` (trigger `{}`)\n",
                item.suggestion.apply_posture.as_str(),
                item.suggestion.trigger.as_str(),
            ));
            out.push_str(&format!(
                "  - Review: [{}/{}]\n",
                item.review.finding_class.as_str(),
                item.review.severity.as_str(),
            ));
            out.push_str(&format!(
                "  - Captured/live: {} | Cited: {}\n",
                item.captured_vs_live.as_str(),
                item.cited,
            ));
            out.push_str(&format!(
                "  - Escapes: open-raw `{}` / open-source `{}`\n",
                item.open_raw_escape_ref, item.open_source_escape_ref,
            ));
        }
        if !self.review_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.review_degradations {
                out.push_str(&format!(
                    "- [{}/{}]: {}\n",
                    degradation.degradation_class.as_str(),
                    degradation.severity.as_str(),
                    degradation.summary,
                ));
            }
        }
        out
    }
}

/// Support-export envelope for the docs-authoring-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsAuthoringReviewSupportExport {
    /// Record kind; must equal [`DOCS_AUTHORING_REVIEW_SUPPORT_EXPORT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Contract doc ref.
    pub doc_ref: String,
    /// The wrapped docs-authoring-review packet.
    pub packet: DocsAuthoringReviewPacket,
}

/// Errors emitted when reading the checked-in docs-authoring-review support export.
#[derive(Debug)]
pub enum DocsAuthoringReviewArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: DocsReviewPromotionState,
        /// Promotion state computed by re-materialization.
        computed: DocsReviewPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<DocsReviewValidationFinding>),
}

impl fmt::Display for DocsAuthoringReviewArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "docs-authoring-review export parse failed: {error}"
                )
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "docs-authoring-review promotion drift: recorded {} but computed {}",
                recorded.as_str(),
                computed.as_str()
            ),
            Self::NotCleanStable(findings) => {
                let tokens = findings
                    .iter()
                    .map(|finding| finding.finding_kind.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "docs-authoring-review export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsAuthoringReviewArtifactError {}

/// Reads and re-validates the checked-in stable docs-authoring-review support export.
pub fn current_stable_docs_authoring_review_export(
) -> Result<DocsAuthoringReviewSupportExport, DocsAuthoringReviewArtifactError> {
    let export: DocsAuthoringReviewSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/implement_docs_authoring_suggestions_stale_link_or_stale_example_review_and_open_raw_or_open_source_escapes/support_export.json"
    )))
    .map_err(DocsAuthoringReviewArtifactError::SupportExport)?;

    let recomputed = DocsAuthoringReviewPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(DocsAuthoringReviewArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(DocsAuthoringReviewArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &DocsAuthoringReviewPacket) -> DocsAuthoringReviewPacketInput {
    DocsAuthoringReviewPacketInput {
        packet_id: packet.packet_id.clone(),
        session_label: packet.session_label.clone(),
        session_digest_ref: packet.session_digest_ref.clone(),
        items: packet.items.clone(),
        export: packet.export.clone(),
        review_degradations: packet.review_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<DocsReviewValidationFinding>,
    kind: DocsReviewFindingKind,
    summary: impl Into<String>,
) {
    findings.push(DocsReviewValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &DocsAuthoringReviewPacketInput,
    findings: &mut Vec<DocsReviewValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.session_label.trim().is_empty()
        || input.session_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            DocsReviewFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_items(
    input: &DocsAuthoringReviewPacketInput,
    findings: &mut Vec<DocsReviewValidationFinding>,
) {
    if input.items.is_empty() {
        push_finding(
            findings,
            DocsReviewFindingKind::ItemsEmpty,
            "the docs-review set must carry at least one item",
        );
        return;
    }

    let present_kinds: BTreeSet<DocsReviewItemKind> =
        input.items.iter().map(|item| item.item_kind).collect();
    for required in DocsReviewItemKind::REQUIRED {
        if !present_kinds.contains(&required) {
            push_finding(
                findings,
                DocsReviewFindingKind::RequiredItemKindMissing,
                format!("required item kind `{}` is missing", required.as_str()),
            );
        }
    }

    let mut seen_item_ids: BTreeSet<&str> = BTreeSet::new();
    for item in &input.items {
        if !seen_item_ids.insert(item.item_id.as_str()) {
            push_finding(
                findings,
                DocsReviewFindingKind::DuplicateItemId,
                format!("duplicate item id `{}`", item.item_id),
            );
        }
        check_one_item(item, findings);
    }
}

fn check_one_item(item: &DocsReviewItem, findings: &mut Vec<DocsReviewValidationFinding>) {
    if item.title.trim().is_empty() || item.detail.trim().is_empty() {
        push_finding(
            findings,
            DocsReviewFindingKind::ItemTitleOrDetailMissing,
            format!("item `{}` is missing a title or detail", item.item_id),
        );
    }
    if item.trust_disclosure_note.trim().is_empty() {
        push_finding(
            findings,
            DocsReviewFindingKind::TrustClassDisclosureMissing,
            format!(
                "item `{}` is missing its trust-class disclosure",
                item.item_id
            ),
        );
    }
    if item.open_raw_escape_ref.trim().is_empty() || item.open_source_escape_ref.trim().is_empty() {
        push_finding(
            findings,
            DocsReviewFindingKind::OpenRawOpenSourceEscapeMissing,
            format!(
                "item `{}` must keep open-raw and open-source escapes",
                item.item_id
            ),
        );
    }

    // An untrusted origin must stay cited.
    if item.trust_class.needs_citation() && !item.cited {
        push_finding(
            findings,
            DocsReviewFindingKind::ItemNotCited,
            format!(
                "item `{}` is `{}` but is not cited",
                item.item_id,
                item.trust_class.as_str()
            ),
        );
    }
    // An untrusted origin may never be presented at high confidence.
    if !item.trust_class.may_be_authoritative()
        && item.chips.confidence == DocsReviewConfidence::High
    {
        push_finding(
            findings,
            DocsReviewFindingKind::TrustClassDisclosureCollapsed,
            format!(
                "item `{}` is `{}` but presented as high confidence",
                item.item_id,
                item.trust_class.as_str()
            ),
        );
    }
    // A non-current version may not be presented as a confident live match.
    if !item.chips.version_match.is_confident_current()
        && item.chips.confidence == DocsReviewConfidence::High
        && item.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            DocsReviewFindingKind::VersionTruthCollapsed,
            format!(
                "item `{}` presents version `{}` as a confident live match",
                item.item_id,
                item.chips.version_match.as_str()
            ),
        );
    }

    // Authoring-suggestion apply-posture truth.
    if item.suggestion.note.trim().is_empty() {
        push_finding(
            findings,
            DocsReviewFindingKind::ApplyPostureNoteMissing,
            format!("item `{}` is missing its apply note", item.item_id),
        );
    }
    // An unverified origin may never offer a one-click apply.
    if item.suggestion.apply_posture.offers_one_click_apply()
        && !item.trust_class.may_be_authoritative()
    {
        push_finding(
            findings,
            DocsReviewFindingKind::UnverifiedSuggestionApplyOffered,
            format!(
                "item `{}` is `{}` but offers a one-click apply",
                item.item_id,
                item.trust_class.as_str()
            ),
        );
    }
    // A one-click apply may never be offered when the item's review verdict blocks.
    if item.suggestion.apply_posture.offers_one_click_apply()
        && item.review.severity == DocsReviewFindingSeverity::Blocking
    {
        push_finding(
            findings,
            DocsReviewFindingKind::ApplyOfferedOnBlockingFinding,
            format!(
                "item `{}` offers a one-click apply while its review verdict `{}` blocks",
                item.item_id,
                item.review.finding_class.as_str()
            ),
        );
    }

    // Stale-link / stale-example review truth.
    if item.review.note.trim().is_empty() {
        push_finding(
            findings,
            DocsReviewFindingKind::ReviewVerdictNoteMissing,
            format!("item `{}` is missing its review note", item.item_id),
        );
    }
    if item.review.finding_class.is_stale() && item.chips.freshness.is_authoritative_live() {
        push_finding(
            findings,
            DocsReviewFindingKind::StaleVerdictFreshnessMismatch,
            format!(
                "item `{}` reports stale `{}` but claims live-authoritative freshness",
                item.item_id,
                item.review.finding_class.as_str()
            ),
        );
    }
}

fn check_export(
    input: &DocsAuthoringReviewPacketInput,
    findings: &mut Vec<DocsReviewValidationFinding>,
) {
    let export = &input.export;
    if !export.preserves_all() {
        push_finding(
            findings,
            DocsReviewFindingKind::ExportDropsPreservation,
            "the export must preserve item kind, trust class, source class, confidence, apply posture, finding class, and escapes",
        );
    }

    let mut export_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &export.rows {
        export_ids.insert(row.item_id_ref.as_str());
        let item = input
            .items
            .iter()
            .find(|item| item.item_id == row.item_id_ref);
        match item {
            None => push_finding(
                findings,
                DocsReviewFindingKind::ExportRowOrphan,
                format!("export row references unknown item `{}`", row.item_id_ref),
            ),
            Some(item) => check_export_row(item, row, findings),
        }
    }

    for item in &input.items {
        if !export_ids.contains(item.item_id.as_str()) {
            push_finding(
                findings,
                DocsReviewFindingKind::ExportCoverageMissing,
                format!("item `{}` has no export row", item.item_id),
            );
        }
    }
}

fn check_export_row(
    item: &DocsReviewItem,
    row: &DocsAuthoringReviewExportRow,
    findings: &mut Vec<DocsReviewValidationFinding>,
) {
    if item.item_kind != row.item_kind {
        push_finding(
            findings,
            DocsReviewFindingKind::ExportItemKindMismatch,
            format!(
                "export for `{}` records kind `{}` but the item is `{}`",
                row.item_id_ref,
                row.item_kind.as_str(),
                item.item_kind.as_str()
            ),
        );
    }
    if item.trust_class != row.trust_class {
        push_finding(
            findings,
            DocsReviewFindingKind::ExportTrustClassMismatch,
            format!(
                "export for `{}` records trust `{}` but the item is `{}`",
                row.item_id_ref,
                row.trust_class.as_str(),
                item.trust_class.as_str()
            ),
        );
    }
    if item.chips.source_class != row.source_class {
        push_finding(
            findings,
            DocsReviewFindingKind::ExportSourceClassMismatch,
            format!(
                "export for `{}` records source `{}` but the item chip is `{}`",
                row.item_id_ref,
                row.source_class.as_str(),
                item.chips.source_class.as_str()
            ),
        );
    }
    if item.chips.confidence != row.confidence {
        push_finding(
            findings,
            DocsReviewFindingKind::ExportConfidenceMismatch,
            format!(
                "export for `{}` records confidence `{}` but the item chip is `{}`",
                row.item_id_ref,
                row.confidence.as_str(),
                item.chips.confidence.as_str()
            ),
        );
    }
    if item.suggestion.apply_posture != row.apply_posture {
        push_finding(
            findings,
            DocsReviewFindingKind::ExportApplyPostureMismatch,
            format!(
                "export for `{}` records apply posture `{}` but the item is `{}`",
                row.item_id_ref,
                row.apply_posture.as_str(),
                item.suggestion.apply_posture.as_str()
            ),
        );
    }
    if item.review.finding_class != row.finding_class || item.review.severity != row.review_severity
    {
        push_finding(
            findings,
            DocsReviewFindingKind::ExportFindingClassMismatch,
            format!(
                "export for `{}` records review `{}`/`{}` but the item is `{}`/`{}`",
                row.item_id_ref,
                row.finding_class.as_str(),
                row.review_severity.as_str(),
                item.review.finding_class.as_str(),
                item.review.severity.as_str()
            ),
        );
    }
    if item.cited != row.cited {
        push_finding(
            findings,
            DocsReviewFindingKind::ExportCitedMismatch,
            format!(
                "export for `{}` records cited `{}` but the item is `{}`",
                row.item_id_ref, row.cited, item.cited
            ),
        );
    }
}

fn check_degradations(
    input: &DocsAuthoringReviewPacketInput,
    findings: &mut Vec<DocsReviewValidationFinding>,
) {
    let item_ids: BTreeSet<&str> = input
        .items
        .iter()
        .map(|item| item.item_id.as_str())
        .collect();

    for degradation in &input.review_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                DocsReviewFindingKind::DegradationIncomplete,
                format!(
                    "degradation `{}` is missing a summary",
                    degradation.degradation_class.as_str()
                ),
            );
        }
        if let Some(item_id) = &degradation.item_id_ref {
            if !item_id.trim().is_empty() && !item_ids.contains(item_id.as_str()) {
                push_finding(
                    findings,
                    DocsReviewFindingKind::DegradationOrphan,
                    format!("degradation references unknown item `{}`", item_id),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &DocsAuthoringReviewPacketInput,
    findings: &mut Vec<DocsReviewValidationFinding>,
) {
    let present: BTreeSet<DocsReviewConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                DocsReviewFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                DocsReviewFindingKind::ConsumerProjectionPacketIdMismatch,
                format!(
                    "surface `{}` references packet `{}`",
                    projection.surface.as_str(),
                    projection.packet_id_ref
                ),
            );
        }
        if !projection.preserves_all() {
            push_finding(
                findings,
                DocsReviewFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &DocsAuthoringReviewPacketInput,
    findings: &mut Vec<DocsReviewValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("docs-authoring-review input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            DocsReviewFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw URLs, diff bodies, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across the validation
/// findings, the per-item review verdicts, and the attached degradations.
///
/// A blocking validation finding (truth collapse, unverified apply, missing
/// escape, or boundary violation) blocks the Stable claim; an otherwise-clean
/// set whose review verdicts or degradations carry a narrowing severity narrows
/// below Stable rather than hiding the items.
fn promotion_state(
    findings: &[DocsReviewValidationFinding],
    items: &[DocsReviewItem],
    degradations: &[DocsReviewDegradation],
) -> DocsReviewPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == DocsReviewFindingSeverity::Blocking)
        || items
            .iter()
            .any(|item| item.review.severity == DocsReviewFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == DocsReviewFindingSeverity::Blocking);
    if any_blocking {
        return DocsReviewPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == DocsReviewFindingSeverity::Narrowing)
        || items
            .iter()
            .any(|item| item.review.severity == DocsReviewFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == DocsReviewFindingSeverity::Narrowing);
    if any_narrowing {
        DocsReviewPromotionState::NarrowedBelowStable
    } else {
        DocsReviewPromotionState::Stable
    }
}

/// Heuristic that rejects obviously forbidden material in the export.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("raw_body:")
                || lower.contains("raw_url:")
                || lower.contains("diff_body:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable docs-authoring-review input used by the producer, tests, and fixtures.
pub fn seeded_stable_docs_authoring_review_input() -> DocsAuthoringReviewPacketInput {
    let packet_id = "packet:m5:docs_authoring_review:retry_backoff_guide".to_owned();
    DocsAuthoringReviewPacketInput {
        packet_id: packet_id.clone(),
        session_label: "docs authoring + review: the networking retry/backoff guide".to_owned(),
        session_digest_ref: "sessiondigest:sha256:net-retry-backoff-guide".to_owned(),
        items: vec![
            authoring_suggestion_item(),
            stale_link_review_item(),
            stale_example_review_item(),
        ],
        export: seeded_export(),
        review_degradations: vec![DocsReviewDegradation {
            degradation_class: DocsReviewDegradationClass::LinkCheckerOffline,
            severity: DocsReviewFindingSeverity::Advisory,
            summary: "the live link checker was offline for one external host; the redirected link verdict is served from the last snapshot".to_owned(),
            item_id_ref: Some("item:stale_link_review:retry_backoff_runbook_link".to_owned()),
            evidence_ref: Some("evidence:docs-authoring-review:link-checker-state".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    }
}

fn authoring_suggestion_item() -> DocsReviewItem {
    DocsReviewItem {
        item_id: "item:authoring_suggestion:retry_backoff_guide_intro".to_owned(),
        item_kind: DocsReviewItemKind::AuthoringSuggestion,
        subject_ref: "docnode:project:guides/retry_with_backoff#intro".to_owned(),
        title: "Authoring suggestion: tighten the retry/backoff intro".to_owned(),
        detail: "a first-party authoring suggestion to tighten the retry/backoff guide intro and cross-link the symbol reference".to_owned(),
        chips: DocsReviewChipSet {
            source_class: DocsReviewSourceClass::FirstPartyDoc,
            version_match: DocsReviewVersionMatch::ExactBuildMatch,
            freshness: DocsReviewFreshness::AuthoritativeLive,
            locality: DocsReviewLocality::Local,
            confidence: DocsReviewConfidence::High,
        },
        trust_class: DocsReviewTrustClass::FirstPartyAuthored,
        trust_disclosure_note: "first-party doc authored in-repo; the suggestion is reviewable and reversible".to_owned(),
        suggestion: AuthoringSuggestion {
            apply_posture: SuggestionApplyPosture::ApplyAvailable,
            trigger: SuggestionTrigger::ManualAuthoring,
            note: "apply rewrites the intro paragraph and adds the symbol cross-link; the edit is previewable and reversible".to_owned(),
        },
        review: StaleReviewVerdict {
            finding_class: ReviewFindingClass::FreshOk,
            severity: DocsReviewFindingSeverity::Advisory,
            note: "the surrounding section re-validated as current against the active build".to_owned(),
        },
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:docnode:project:guides/retry_with_backoff#intro".to_owned()),
        open_raw_escape_ref: "open-raw:docnode:project:guides/retry_with_backoff#intro".to_owned(),
        open_source_escape_ref: "open-source:repo:docs/guides/retry_with_backoff.md".to_owned(),
    }
}

fn stale_link_review_item() -> DocsReviewItem {
    DocsReviewItem {
        item_id: "item:stale_link_review:retry_backoff_runbook_link".to_owned(),
        item_kind: DocsReviewItemKind::StaleLinkReview,
        subject_ref: "docnode:project:guides/retry_with_backoff#runbook".to_owned(),
        title: "Stale-link review: the runbook link redirects".to_owned(),
        detail: "a stale-link review of the operations runbook link, which now redirects to a renamed page".to_owned(),
        chips: DocsReviewChipSet {
            source_class: DocsReviewSourceClass::SignedDocsPack,
            version_match: DocsReviewVersionMatch::CompatibleMinorDrift,
            freshness: DocsReviewFreshness::WarmCached,
            locality: DocsReviewLocality::ImportedPack,
            confidence: DocsReviewConfidence::Medium,
        },
        trust_class: DocsReviewTrustClass::SignedDocsPack,
        trust_disclosure_note: "from the signed docs pack; the redirect target is disclosed and held to medium pending a re-check".to_owned(),
        suggestion: AuthoringSuggestion {
            apply_posture: SuggestionApplyPosture::PreviewRequired,
            trigger: SuggestionTrigger::BrokenLinkDetected,
            note: "apply repoints the runbook link to the redirect target; a preview is required before applying".to_owned(),
        },
        review: StaleReviewVerdict {
            finding_class: ReviewFindingClass::StaleLinkRedirected,
            severity: DocsReviewFindingSeverity::Advisory,
            note: "the runbook link returns a stable redirect; the target resolves and the redirect is disclosed as a warm snapshot".to_owned(),
        },
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:docnode:project:guides/retry_with_backoff#runbook".to_owned()),
        open_raw_escape_ref: "open-raw:docnode:project:guides/retry_with_backoff#runbook".to_owned(),
        open_source_escape_ref: "open-source:pack:ops/runbooks/retry_backoff_runbook.md".to_owned(),
    }
}

fn stale_example_review_item() -> DocsReviewItem {
    DocsReviewItem {
        item_id: "item:stale_example_review:retry_backoff_example".to_owned(),
        item_kind: DocsReviewItemKind::StaleExampleReview,
        subject_ref: "docnode:project:guides/retry_with_backoff#example".to_owned(),
        title: "Stale-example review: the backoff example re-validated".to_owned(),
        detail: "a stale-example review of the retry/backoff code example, which re-validated against the active build".to_owned(),
        chips: DocsReviewChipSet {
            source_class: DocsReviewSourceClass::FirstPartyDoc,
            version_match: DocsReviewVersionMatch::ExactBuildMatch,
            freshness: DocsReviewFreshness::AuthoritativeLive,
            locality: DocsReviewLocality::Local,
            confidence: DocsReviewConfidence::High,
        },
        trust_class: DocsReviewTrustClass::FirstPartyAuthored,
        trust_disclosure_note: "first-party example checked against the in-repo build; it compiles and matches the documented symbol".to_owned(),
        suggestion: AuthoringSuggestion {
            apply_posture: SuggestionApplyPosture::SuggestionOnly,
            trigger: SuggestionTrigger::StaleExampleDetected,
            note: "no edit is needed; the example re-validated, so the suggestion is advisory only".to_owned(),
        },
        review: StaleReviewVerdict {
            finding_class: ReviewFindingClass::FreshOk,
            severity: DocsReviewFindingSeverity::Advisory,
            note: "the example compiles and matches the documented retry_with_backoff signature on the active build".to_owned(),
        },
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:docnode:project:guides/retry_with_backoff#example".to_owned()),
        open_raw_escape_ref: "open-raw:docnode:project:guides/retry_with_backoff#example".to_owned(),
        open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
    }
}

fn seeded_export() -> DocsAuthoringReviewExport {
    DocsAuthoringReviewExport {
        scope: DocsReviewExportScope::AllItems,
        preserves_item_kind: true,
        preserves_trust_class: true,
        preserves_source_class: true,
        preserves_confidence: true,
        preserves_apply_posture: true,
        preserves_finding_class: true,
        preserves_open_raw_open_source_escape: true,
        rows: vec![
            DocsAuthoringReviewExportRow {
                item_id_ref: "item:authoring_suggestion:retry_backoff_guide_intro".to_owned(),
                item_kind: DocsReviewItemKind::AuthoringSuggestion,
                trust_class: DocsReviewTrustClass::FirstPartyAuthored,
                source_class: DocsReviewSourceClass::FirstPartyDoc,
                confidence: DocsReviewConfidence::High,
                apply_posture: SuggestionApplyPosture::ApplyAvailable,
                finding_class: ReviewFindingClass::FreshOk,
                review_severity: DocsReviewFindingSeverity::Advisory,
                cited: true,
                open_raw_escape_ref: "open-raw:docnode:project:guides/retry_with_backoff#intro"
                    .to_owned(),
                open_source_escape_ref: "open-source:repo:docs/guides/retry_with_backoff.md"
                    .to_owned(),
            },
            DocsAuthoringReviewExportRow {
                item_id_ref: "item:stale_link_review:retry_backoff_runbook_link".to_owned(),
                item_kind: DocsReviewItemKind::StaleLinkReview,
                trust_class: DocsReviewTrustClass::SignedDocsPack,
                source_class: DocsReviewSourceClass::SignedDocsPack,
                confidence: DocsReviewConfidence::Medium,
                apply_posture: SuggestionApplyPosture::PreviewRequired,
                finding_class: ReviewFindingClass::StaleLinkRedirected,
                review_severity: DocsReviewFindingSeverity::Advisory,
                cited: true,
                open_raw_escape_ref: "open-raw:docnode:project:guides/retry_with_backoff#runbook"
                    .to_owned(),
                open_source_escape_ref: "open-source:pack:ops/runbooks/retry_backoff_runbook.md"
                    .to_owned(),
            },
            DocsAuthoringReviewExportRow {
                item_id_ref: "item:stale_example_review:retry_backoff_example".to_owned(),
                item_kind: DocsReviewItemKind::StaleExampleReview,
                trust_class: DocsReviewTrustClass::FirstPartyAuthored,
                source_class: DocsReviewSourceClass::FirstPartyDoc,
                confidence: DocsReviewConfidence::High,
                apply_posture: SuggestionApplyPosture::SuggestionOnly,
                finding_class: ReviewFindingClass::FreshOk,
                review_severity: DocsReviewFindingSeverity::Advisory,
                cited: true,
                open_raw_escape_ref: "open-raw:docnode:project:guides/retry_with_backoff#example"
                    .to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs"
                    .to_owned(),
            },
        ],
    }
}

fn required_projections(packet_id: &str) -> Vec<DocsReviewConsumerProjection> {
    [
        DocsReviewConsumerSurface::DocsAuthoringSurface,
        DocsReviewConsumerSurface::DocsBrowserShell,
        DocsReviewConsumerSurface::DocsReviewPanel,
        DocsReviewConsumerSurface::StaleExampleReviewQueue,
        DocsReviewConsumerSurface::AiContextInspector,
        DocsReviewConsumerSurface::CliHeadless,
        DocsReviewConsumerSurface::SupportExport,
        DocsReviewConsumerSurface::Diagnostics,
        DocsReviewConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| DocsReviewConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_item_kinds: true,
        preserves_apply_posture: true,
        preserves_finding_class: true,
        preserves_trust_classes: true,
        preserves_confidence: true,
        preserves_open_raw_open_source_escape: true,
    })
    .collect()
}
