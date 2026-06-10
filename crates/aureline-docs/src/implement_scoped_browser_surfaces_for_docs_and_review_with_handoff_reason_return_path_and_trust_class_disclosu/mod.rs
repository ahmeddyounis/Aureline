//! Scoped browser surfaces for docs and review with handoff reason, return
//! path, and trust-class disclosure.
//!
//! This module implements the M5 scoped-browser-surface boundary: the surface
//! that hands the reader off to a *narrow* browser view — reading vendor docs,
//! looking at a hosted review thread, or making a light edit — without ever
//! looking like a general web-mode or full browser runtime. Each
//! [`ScopedBrowserSurface`] carries one [`ScopedBrowserScope`] (the qualified
//! `docs_reading` / `review` / `light_edit` scope; anything broader is recorded
//! and blocked), an explicit [`HandoffReason`] (*why* the product handed off),
//! a non-empty [`ReturnPath`] (*how the reader gets back* — the return-path
//! safety guarantee), one [`ScopedBrowserTrustClass`] disclosure (*how
//! trustworthy the destination is*), the same source/version/freshness/
//! locality/confidence chip set the other docs lanes use, the
//! [`HandoffCapability`] posture, the live-vs-captured state, citation state,
//! and the open-raw / open-source escapes.
//!
//! A surface whose trust class cannot back an authoritative claim
//! (`live_provider_handoff`, `derived_inference_only`) may not be presented at
//! high confidence, a handoff blocked by policy may not be presented as
//! available, and every surface must keep a return path — so a scoped browser
//! surface never reads as more authoritative, more available, or more
//! escapable than it is.
//!
//! The [`ScopedBrowserExport`] is the cited projection support, AI evidence,
//! and diagnostics surfaces ingest: one [`ScopedBrowserExportRow`] per surface
//! preserving scope, trust class, source class, confidence, handoff capability,
//! return-path presence, citation state, and the open-raw / open-source
//! escapes.
//!
//! [`ScopedBrowserSurfacesPacket::materialize`] computes the validation
//! findings and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input, so an out-of-scope, return-path-unsafe,
//! trust-collapsed, uncited, or unattributed surface set automatically narrows
//! or blocks before it reaches a consumer surface. The packet is an
//! inspectable, serde-serializable truth packet: it carries no raw page bodies,
//! no raw URLs, no raw review payloads, no raw source files, no raw provider
//! payloads, and no credentials — only metadata, scope truth, handoff reasons,
//! return paths, trust disclosures, chip truth, cited refs, provenance, finding
//! summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/implement-scoped-browser-surfaces-for-docs-and-review-with-handoff-reason-return-path-and-trust-class-disclosu.schema.json`](../../../../schemas/docs/implement-scoped-browser-surfaces-for-docs-and-review-with-handoff-reason-return-path-and-trust-class-disclosu.schema.json).
//! The contract doc is
//! [`docs/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu.md`](../../../../docs/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/`](../../../../fixtures/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ScopedBrowserSurfacesPacket`].
pub const SCOPED_BROWSER_RECORD_KIND: &str = "scoped_browser_surfaces_for_docs_and_review";

/// Record-kind tag carried by the support-export wrapper.
pub const SCOPED_BROWSER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "scoped_browser_surfaces_for_docs_and_review_support_export";

/// Schema version for scoped-browser-surface records.
pub const SCOPED_BROWSER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const SCOPED_BROWSER_SCHEMA_REF: &str =
    "schemas/docs/implement-scoped-browser-surfaces-for-docs-and-review-with-handoff-reason-return-path-and-trust-class-disclosu.schema.json";

/// Repo-relative path of the scoped-browser-surface contract doc.
pub const SCOPED_BROWSER_DOC_REF: &str =
    "docs/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu.md";

/// Repo-relative path of the protected fixture directory.
pub const SCOPED_BROWSER_FIXTURE_DIR: &str =
    "fixtures/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu";

/// Repo-relative path of the checked support-export artifact.
pub const SCOPED_BROWSER_ARTIFACT_REF: &str =
    "artifacts/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const SCOPED_BROWSER_SUMMARY_REF: &str =
    "artifacts/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu.md";

/// The scope a scoped browser surface is qualified for.
///
/// Only `docs_reading`, `review`, and `light_edit` are inside the qualified M5
/// scope. `general_web` and `full_browser_runtime` are recorded so the
/// validator can detect and block a surface that overruns the qualified scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserScope {
    /// A narrow docs-reading surface (vendor / mirrored / live docs).
    DocsReading,
    /// A narrow review surface (a hosted review thread / diff view).
    Review,
    /// A narrow light-edit surface (a small scoped edit, not a full IDE).
    LightEdit,
    /// A general web-mode surface — outside the qualified M5 scope.
    GeneralWeb,
    /// A full browser-runtime surface — outside the qualified M5 scope.
    FullBrowserRuntime,
}

impl ScopedBrowserScope {
    /// The scopes a packet must cover (`docs_reading` and `review`).
    pub const REQUIRED: [Self; 2] = [Self::DocsReading, Self::Review];

    /// Stable token recorded in the surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsReading => "docs_reading",
            Self::Review => "review",
            Self::LightEdit => "light_edit",
            Self::GeneralWeb => "general_web",
            Self::FullBrowserRuntime => "full_browser_runtime",
        }
    }

    /// Whether this scope is inside the qualified docs/review/light-edit scope.
    pub const fn is_within_qualified_scope(self) -> bool {
        matches!(self, Self::DocsReading | Self::Review | Self::LightEdit)
    }
}

/// Why the product handed off to a scoped browser surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffReasonKind {
    /// The exact anchor is not available locally; the upstream page has it.
    ExactAnchorUnavailableLocally,
    /// The live upstream version is newer than the local mirror.
    LiveVersionNewerThanMirror,
    /// The content is not mirrored; only the upstream source has it.
    SourceNotMirrored,
    /// A review thread requires the hosted review view.
    ReviewThreadRequiresHostedView,
    /// A light edit requires a scoped editor surface.
    LightEditRequiresScopedEditor,
    /// The reader explicitly asked to open in a browser surface.
    UserRequestedOpenInBrowser,
}

impl HandoffReasonKind {
    /// Stable token recorded in the handoff reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactAnchorUnavailableLocally => "exact_anchor_unavailable_locally",
            Self::LiveVersionNewerThanMirror => "live_version_newer_than_mirror",
            Self::SourceNotMirrored => "source_not_mirrored",
            Self::ReviewThreadRequiresHostedView => "review_thread_requires_hosted_view",
            Self::LightEditRequiresScopedEditor => "light_edit_requires_scoped_editor",
            Self::UserRequestedOpenInBrowser => "user_requested_open_in_browser",
        }
    }
}

/// Where the reader returns to when leaving a scoped browser surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnPathKind {
    /// Back to the inline peek the handoff came from.
    BackToInlinePeek,
    /// Back to the docs browser shell.
    BackToDocsBrowser,
    /// Back to the review panel.
    BackToReviewPanel,
    /// Back to the workspace / editor.
    BackToWorkspace,
}

impl ReturnPathKind {
    /// Stable token recorded in the return path.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BackToInlinePeek => "back_to_inline_peek",
            Self::BackToDocsBrowser => "back_to_docs_browser",
            Self::BackToReviewPanel => "back_to_review_panel",
            Self::BackToWorkspace => "back_to_workspace",
        }
    }
}

/// Trust class of the scoped browser destination, projected as a disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserTrustClass {
    /// First-party authoritative content (e.g. the workspace's own docs).
    FirstPartyAuthoritative,
    /// A pinned, signed mirror of upstream docs.
    SignedMirrorVerified,
    /// A signed extension / imported docs pack.
    ExtensionPackSigned,
    /// A live provider handoff — not verified at materialization time.
    LiveProviderHandoff,
    /// Derived / inferred content only.
    DerivedInferenceOnly,
}

impl ScopedBrowserTrustClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyAuthoritative => "first_party_authoritative",
            Self::SignedMirrorVerified => "signed_mirror_verified",
            Self::ExtensionPackSigned => "extension_pack_signed",
            Self::LiveProviderHandoff => "live_provider_handoff",
            Self::DerivedInferenceOnly => "derived_inference_only",
        }
    }

    /// Whether this trust class may back a high-confidence / authoritative
    /// claim. A live provider handoff or derived inference may not.
    pub const fn may_be_authoritative(self) -> bool {
        matches!(
            self,
            Self::FirstPartyAuthoritative | Self::SignedMirrorVerified | Self::ExtensionPackSigned
        )
    }

    /// Whether a surface of this trust class must stay cited.
    pub const fn needs_citation(self) -> bool {
        !matches!(self, Self::FirstPartyAuthoritative)
    }
}

/// The handoff capability posture for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffCapability {
    /// Handoff is not required; the content is fully local.
    NotRequiredLocal,
    /// Handoff is available and explicit.
    AvailableExplicit,
    /// Handoff is blocked by policy.
    BlockedByPolicy,
    /// Handoff is unavailable and disclosed as such.
    UnavailableDisclosed,
}

impl HandoffCapability {
    /// Stable token recorded in the surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequiredLocal => "not_required_local",
            Self::AvailableExplicit => "available_explicit",
            Self::BlockedByPolicy => "blocked_by_policy",
            Self::UnavailableDisclosed => "unavailable_disclosed",
        }
    }

    /// Whether the surface may present the handoff as an available action.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::AvailableExplicit)
    }
}

/// Whether the surface is live, a captured snapshot, or a narrowed rerun.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapturedVsLive {
    /// A live surface.
    Live,
    /// A captured snapshot of an earlier view.
    CapturedSnapshot,
    /// A rerun narrowed to a smaller scope.
    NarrowedScopeRerun,
}

impl CapturedVsLive {
    /// Stable token recorded in the surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::CapturedSnapshot => "captured_snapshot",
            Self::NarrowedScopeRerun => "narrowed_scope_rerun",
        }
    }
}

/// Source class for a surface's underlying material, projected as the source chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserSourceClass {
    /// Workspace-local project docs.
    ProjectDocs,
    /// Pinned, signed mirror of official upstream docs.
    MirroredOfficialDocs,
    /// An imported / extension docs pack.
    ExtensionDocsPack,
    /// Live external docs (handed off, not mirrored).
    LiveExternalDocs,
    /// A hosted code-review thread / diff host.
    ReviewHost,
    /// Generated API / reference docs.
    GeneratedReference,
    /// Derived / inferred explanation.
    DerivedExplanation,
}

impl ScopedBrowserSourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::ExtensionDocsPack => "extension_docs_pack",
            Self::LiveExternalDocs => "live_external_docs",
            Self::ReviewHost => "review_host",
            Self::GeneratedReference => "generated_reference",
            Self::DerivedExplanation => "derived_explanation",
        }
    }
}

/// Version-match state for a surface, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserVersionMatch {
    /// Surface matches the active build/workspace revision exactly.
    ExactBuildMatch,
    /// Surface is within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Surface drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release surface has not completed verification.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl ScopedBrowserVersionMatch {
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

/// Freshness state for a surface, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserFreshness {
    /// Surface was live and authoritative at materialization time.
    AuthoritativeLive,
    /// Cached surface within its freshness window.
    WarmCached,
    /// Cached surface usable only with degraded disclosure.
    DegradedCached,
    /// Surface is stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the source has not yet re-synced.
    RefreshPending,
}

impl ScopedBrowserFreshness {
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

/// Locality / install posture for a surface, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through a pinned mirror pack.
    MirroredPack,
    /// Resolved through a remote helper.
    RemoteHelper,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl ScopedBrowserLocality {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::MirroredPack => "mirrored_pack",
            Self::RemoteHelper => "remote_helper",
            Self::Managed => "managed",
        }
    }
}

/// Confidence label for a surface, projected as the confidence chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified claim.
    Heuristic,
}

impl ScopedBrowserConfidence {
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

/// Severity of a degradation or validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserFindingSeverity {
    /// Blocks a Stable claim; the set must block.
    Blocking,
    /// Narrows below Stable but the set stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl ScopedBrowserFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the scoped-browser packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserConsumerSurface {
    /// The docs browser shell.
    DocsBrowserShell,
    /// The review surface.
    ReviewSurface,
    /// The browser handoff packet view.
    BrowserHandoffPacket,
    /// An inline peek overlay.
    PeekOverlay,
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

impl ScopedBrowserConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::ReviewSurface => "review_surface",
            Self::BrowserHandoffPacket => "browser_handoff_packet",
            Self::PeekOverlay => "peek_overlay",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level scoped-browser degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserDegradationClass {
    /// A mirror is offline; the surface is served from the last snapshot.
    MirrorOfflineSnapshot,
    /// A live provider was unreachable; a captured snapshot is shown instead.
    LiveProviderUnreachableCapturedSnapshot,
    /// Handoff was blocked by policy.
    HandoffBlockedByPolicy,
    /// The return path is degraded (still present, but reduced).
    ReturnPathDegraded,
    /// The surface was rerun at a narrowed scope.
    ScopeNarrowedRerun,
    /// A referenced anchor is broken.
    BrokenAnchor,
    /// The owning source is quarantined.
    QuarantinedSource,
}

impl ScopedBrowserDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::LiveProviderUnreachableCapturedSnapshot => {
                "live_provider_unreachable_captured_snapshot"
            }
            Self::HandoffBlockedByPolicy => "handoff_blocked_by_policy",
            Self::ReturnPathDegraded => "return_path_degraded",
            Self::ScopeNarrowedRerun => "scope_narrowed_rerun",
            Self::BrokenAnchor => "broken_anchor",
            Self::QuarantinedSource => "quarantined_source",
        }
    }
}

/// Scope a scoped-browser export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserExportScope {
    /// Every surface in the packet.
    AllSurfaces,
    /// Docs-reading surfaces only.
    DocsReadingOnly,
    /// Review surfaces only.
    ReviewOnly,
    /// Light-edit surfaces only.
    LightEditOnly,
}

impl ScopedBrowserExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllSurfaces => "all_surfaces",
            Self::DocsReadingOnly => "docs_reading_only",
            Self::ReviewOnly => "review_only",
            Self::LightEditOnly => "light_edit_only",
        }
    }
}

/// Promotion state computed for the scoped-browser packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserPromotionState {
    /// Set qualifies for the Stable claim.
    Stable,
    /// Set narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Set has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl ScopedBrowserPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`ScopedBrowserSurfacesPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopedBrowserFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The surface set is empty.
    SurfacesEmpty,
    /// A surface id is duplicated.
    DuplicateSurfaceId,
    /// A required scope (docs / review) is missing.
    RequiredScopeMissing,
    /// A surface declares a scope outside the qualified docs/review scope.
    SurfaceScopeOutOfBounds,
    /// A surface is missing its title or headline.
    SurfaceTitleOrHeadlineMissing,
    /// A surface is missing its handoff reason.
    HandoffReasonMissing,
    /// A surface is missing its return path (return-path safety violation).
    ReturnPathMissing,
    /// A surface is missing its trust-class disclosure note.
    TrustClassDisclosureMissing,
    /// An untrusted destination is presented as a high-confidence claim.
    TrustClassDisclosureCollapsed,
    /// A handoff blocked by policy is presented as available.
    BlockedHandoffPresentedAvailable,
    /// A surface is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// An imported / live / derived surface is not cited.
    SurfaceNotCited,
    /// A non-current version-match is presented as a confident live match.
    VersionTruthCollapsed,
    /// An export row references a surface id absent from the surfaces.
    ExportRowOrphan,
    /// A surface has no matching export row.
    ExportCoverageMissing,
    /// The export drops a required preservation flag.
    ExportDropsPreservation,
    /// An export row's scope disagrees with the surface.
    ExportScopeMismatch,
    /// An export row's trust class disagrees with the surface.
    ExportTrustClassMismatch,
    /// An export row's source class disagrees with the surface's chip.
    ExportSourceClassMismatch,
    /// An export row's confidence disagrees with the surface's chip.
    ExportConfidenceMismatch,
    /// An export row drops the return-path-present flag a surface keeps.
    ExportReturnPathMismatch,
    /// A degradation is incomplete (missing summary).
    DegradationIncomplete,
    /// A degradation references a surface id absent from the surfaces.
    DegradationOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw bodies, raw URLs, raw review payloads, or secrets crossed the boundary.
    RawBoundaryMaterialPresent,
}

impl ScopedBrowserFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::SurfacesEmpty => "surfaces_empty",
            Self::DuplicateSurfaceId => "duplicate_surface_id",
            Self::RequiredScopeMissing => "required_scope_missing",
            Self::SurfaceScopeOutOfBounds => "surface_scope_out_of_bounds",
            Self::SurfaceTitleOrHeadlineMissing => "surface_title_or_headline_missing",
            Self::HandoffReasonMissing => "handoff_reason_missing",
            Self::ReturnPathMissing => "return_path_missing",
            Self::TrustClassDisclosureMissing => "trust_class_disclosure_missing",
            Self::TrustClassDisclosureCollapsed => "trust_class_disclosure_collapsed",
            Self::BlockedHandoffPresentedAvailable => "blocked_handoff_presented_available",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::SurfaceNotCited => "surface_not_cited",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::ExportRowOrphan => "export_row_orphan",
            Self::ExportCoverageMissing => "export_coverage_missing",
            Self::ExportDropsPreservation => "export_drops_preservation",
            Self::ExportScopeMismatch => "export_scope_mismatch",
            Self::ExportTrustClassMismatch => "export_trust_class_mismatch",
            Self::ExportSourceClassMismatch => "export_source_class_mismatch",
            Self::ExportConfidenceMismatch => "export_confidence_mismatch",
            Self::ExportReturnPathMismatch => "export_return_path_mismatch",
            Self::DegradationIncomplete => "degradation_incomplete",
            Self::DegradationOrphan => "degradation_orphan",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::RequiredSurfaceCoverageMissing => "required_surface_coverage_missing",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
        }
    }

    /// Default severity for this finding kind. Every validation finding blocks
    /// the Stable claim; narrowing comes only from data-carried degradation
    /// severities so a degraded-but-honest set narrows rather than blocks.
    pub const fn default_severity(self) -> ScopedBrowserFindingSeverity {
        ScopedBrowserFindingSeverity::Blocking
    }
}

/// The chip set rendered for one scoped browser surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserChipSet {
    /// Source-class chip.
    pub source_class: ScopedBrowserSourceClass,
    /// Version-match chip.
    pub version_match: ScopedBrowserVersionMatch,
    /// Freshness chip.
    pub freshness: ScopedBrowserFreshness,
    /// Locality chip.
    pub locality: ScopedBrowserLocality,
    /// Confidence chip (the confidence label).
    pub confidence: ScopedBrowserConfidence,
}

/// Why the product handed off to this scoped browser surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffReason {
    /// Kind of handoff reason.
    pub reason_kind: HandoffReasonKind,
    /// Human-readable explanation (no raw URLs / no raw bodies).
    pub note: String,
}

/// How the reader returns from this scoped browser surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnPath {
    /// Kind of return path.
    pub return_kind: ReturnPathKind,
    /// Stable destination ref the reader returns to (no raw URLs).
    pub return_ref: String,
    /// Human-readable return label.
    pub label: String,
}

/// One scoped browser surface — one bounded handoff to docs / review / edit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserSurface {
    /// Stable surface id within this packet.
    pub surface_id: String,
    /// The qualified scope this surface is for.
    pub scope: ScopedBrowserScope,
    /// Subject ref the surface points at (no raw URL / no raw body).
    pub subject_ref: String,
    /// Human-readable title.
    pub title: String,
    /// Human-readable headline / summary (no raw bodies).
    pub headline: String,
    /// Source/version/freshness/locality/confidence chips.
    pub chips: ScopedBrowserChipSet,
    /// The trust-class disclosure for the destination.
    pub trust_class: ScopedBrowserTrustClass,
    /// Human-readable trust-class disclosure note.
    pub trust_disclosure_note: String,
    /// Why the product handed off here.
    pub handoff_reason: HandoffReason,
    /// How the reader returns (return-path safety).
    pub return_path: ReturnPath,
    /// The handoff capability posture.
    pub handoff_capability: HandoffCapability,
    /// Whether the surface is live, captured, or a narrowed rerun.
    pub captured_vs_live: CapturedVsLive,
    /// Whether the surface is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
    /// Open-raw escape ref (open the underlying node/subject).
    pub open_raw_escape_ref: String,
    /// Open-source escape ref (open the upstream/source).
    pub open_source_escape_ref: String,
}

/// One export row, mirroring a surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserExportRow {
    /// The surface this export row mirrors.
    pub surface_id_ref: String,
    /// Scope (must match the surface).
    pub scope: ScopedBrowserScope,
    /// Trust class (must match the surface).
    pub trust_class: ScopedBrowserTrustClass,
    /// Source class (must match the surface's chip).
    pub source_class: ScopedBrowserSourceClass,
    /// Confidence (must match the surface's chip).
    pub confidence: ScopedBrowserConfidence,
    /// Handoff capability (must match the surface).
    pub handoff_capability: HandoffCapability,
    /// Whether the surface keeps a return path.
    pub has_return_path: bool,
    /// Whether the surface is cited.
    pub cited: bool,
    /// Open-raw escape ref.
    pub open_raw_escape_ref: String,
    /// Open-source escape ref.
    pub open_source_escape_ref: String,
}

/// The scoped-browser export projection for the surface set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserExport {
    /// Scope this export covers.
    pub scope: ScopedBrowserExportScope,
    /// Whether the export preserves each surface's scope.
    pub preserves_scope: bool,
    /// Whether the export preserves each surface's handoff reason.
    pub preserves_handoff_reason: bool,
    /// Whether the export preserves each surface's return path.
    pub preserves_return_path: bool,
    /// Whether the export preserves each surface's trust class.
    pub preserves_trust_class: bool,
    /// Whether the export preserves each surface's source class.
    pub preserves_source_class: bool,
    /// Whether the export preserves each surface's confidence label.
    pub preserves_confidence: bool,
    /// Whether the export preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
    /// Per-surface export rows.
    pub rows: Vec<ScopedBrowserExportRow>,
}

impl ScopedBrowserExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_scope
            && self.preserves_handoff_reason
            && self.preserves_return_path
            && self.preserves_trust_class
            && self.preserves_source_class
            && self.preserves_confidence
            && self.preserves_open_raw_open_source_escape
    }
}

/// A packet-level scoped-browser degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserDegradation {
    /// Degradation class.
    pub degradation_class: ScopedBrowserDegradationClass,
    /// Severity.
    pub severity: ScopedBrowserFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The surface this degradation annotates, if scoped to one surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the scoped-browser set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserConsumerProjection {
    /// Surface that consumes the set.
    pub surface: ScopedBrowserConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves all scopes.
    pub preserves_scopes: bool,
    /// Whether the surface preserves the trust classes.
    pub preserves_trust_classes: bool,
    /// Whether the surface preserves the handoff reasons.
    pub preserves_handoff_reasons: bool,
    /// Whether the surface preserves the return paths.
    pub preserves_return_paths: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
}

impl ScopedBrowserConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_scopes
            && self.preserves_trust_classes
            && self.preserves_handoff_reasons
            && self.preserves_return_paths
            && self.preserves_open_raw_open_source_escape
    }
}

/// A single validation finding on the scoped-browser set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserValidationFinding {
    /// Finding kind.
    pub finding_kind: ScopedBrowserFindingKind,
    /// Finding severity.
    pub severity: ScopedBrowserFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`ScopedBrowserSurfacesPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserSurfacesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label (no raw URLs / no raw query text).
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The scoped browser surfaces.
    pub surfaces: Vec<ScopedBrowserSurface>,
    /// The export projection.
    pub export: ScopedBrowserExport,
    /// Packet-level degradations.
    pub browser_degradations: Vec<ScopedBrowserDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<ScopedBrowserConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe scoped-browser packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserSurfacesPacket {
    /// Record kind; must equal [`SCOPED_BROWSER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SCOPED_BROWSER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label.
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The scoped browser surfaces.
    pub surfaces: Vec<ScopedBrowserSurface>,
    /// The export projection.
    pub export: ScopedBrowserExport,
    /// Packet-level degradations.
    pub browser_degradations: Vec<ScopedBrowserDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<ScopedBrowserConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: ScopedBrowserPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<ScopedBrowserValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every scoped-browser packet must project.
const REQUIRED_SURFACES: [ScopedBrowserConsumerSurface; 4] = [
    ScopedBrowserConsumerSurface::DocsBrowserShell,
    ScopedBrowserConsumerSurface::ReviewSurface,
    ScopedBrowserConsumerSurface::BrowserHandoffPacket,
    ScopedBrowserConsumerSurface::SupportExport,
];

impl ScopedBrowserSurfacesPacket {
    /// Materializes a scoped-browser packet, computing validation findings and
    /// the promotion state from the input.
    pub fn materialize(input: ScopedBrowserSurfacesPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_surfaces(&input, &mut findings);
        check_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.browser_degradations);

        Self {
            record_kind: SCOPED_BROWSER_RECORD_KIND.to_owned(),
            schema_version: SCOPED_BROWSER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            session_label: input.session_label,
            session_digest_ref: input.session_digest_ref,
            surfaces: input.surfaces,
            export: input.export,
            browser_degradations: input.browser_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the set qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == ScopedBrowserPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(&self, export_id: &str, exported_at: &str) -> ScopedBrowserSupportExport {
        ScopedBrowserSupportExport {
            record_kind: SCOPED_BROWSER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: SCOPED_BROWSER_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: SCOPED_BROWSER_SCHEMA_REF.to_owned(),
            doc_ref: SCOPED_BROWSER_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("scoped-browser packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Scoped Browser Surfaces (docs and review)\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Session: {}\n", self.session_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Surfaces: {} | Degradations: {}\n",
            self.surfaces.len(),
            self.browser_degradations.len()
        ));
        out.push_str("\n## Surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- [{}] `{}` ({}) — trust `{}` — {} / {} / {} / {} / {}\n",
                surface.scope.as_str(),
                surface.surface_id,
                surface.title,
                surface.trust_class.as_str(),
                surface.chips.source_class.as_str(),
                surface.chips.version_match.as_str(),
                surface.chips.freshness.as_str(),
                surface.chips.locality.as_str(),
                surface.chips.confidence.as_str(),
            ));
            out.push_str(&format!(
                "  - Handoff reason: [{}] {}\n",
                surface.handoff_reason.reason_kind.as_str(),
                surface.handoff_reason.note,
            ));
            out.push_str(&format!(
                "  - Return path: [{}] {}\n",
                surface.return_path.return_kind.as_str(),
                surface.return_path.label,
            ));
            out.push_str(&format!(
                "  - Capability: {} | Captured/live: {} | Cited: {}\n",
                surface.handoff_capability.as_str(),
                surface.captured_vs_live.as_str(),
                surface.cited,
            ));
        }
        if !self.browser_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.browser_degradations {
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

/// Support-export envelope for the scoped-browser packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopedBrowserSupportExport {
    /// Record kind; must equal [`SCOPED_BROWSER_SUPPORT_EXPORT_RECORD_KIND`].
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
    /// The wrapped scoped-browser packet.
    pub packet: ScopedBrowserSurfacesPacket,
}

/// Errors emitted when reading the checked-in scoped-browser support export.
#[derive(Debug)]
pub enum ScopedBrowserArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: ScopedBrowserPromotionState,
        /// Promotion state computed by re-materialization.
        computed: ScopedBrowserPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<ScopedBrowserValidationFinding>),
}

impl fmt::Display for ScopedBrowserArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "scoped-browser export parse failed: {error}")
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "scoped-browser promotion drift: recorded {} but computed {}",
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
                    "scoped-browser export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for ScopedBrowserArtifactError {}

/// Reads and re-validates the checked-in stable scoped-browser support export.
pub fn current_stable_scoped_browser_export(
) -> Result<ScopedBrowserSupportExport, ScopedBrowserArtifactError> {
    let export: ScopedBrowserSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/implement_scoped_browser_surfaces_for_docs_and_review_with_handoff_reason_return_path_and_trust_class_disclosu/support_export.json"
    )))
    .map_err(ScopedBrowserArtifactError::SupportExport)?;

    let recomputed = ScopedBrowserSurfacesPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(ScopedBrowserArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(ScopedBrowserArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &ScopedBrowserSurfacesPacket) -> ScopedBrowserSurfacesPacketInput {
    ScopedBrowserSurfacesPacketInput {
        packet_id: packet.packet_id.clone(),
        session_label: packet.session_label.clone(),
        session_digest_ref: packet.session_digest_ref.clone(),
        surfaces: packet.surfaces.clone(),
        export: packet.export.clone(),
        browser_degradations: packet.browser_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<ScopedBrowserValidationFinding>,
    kind: ScopedBrowserFindingKind,
    summary: impl Into<String>,
) {
    findings.push(ScopedBrowserValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &ScopedBrowserSurfacesPacketInput,
    findings: &mut Vec<ScopedBrowserValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.session_label.trim().is_empty()
        || input.session_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            ScopedBrowserFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_surfaces(
    input: &ScopedBrowserSurfacesPacketInput,
    findings: &mut Vec<ScopedBrowserValidationFinding>,
) {
    if input.surfaces.is_empty() {
        push_finding(
            findings,
            ScopedBrowserFindingKind::SurfacesEmpty,
            "the scoped-browser set must carry at least one surface",
        );
        return;
    }

    let present_scopes: BTreeSet<ScopedBrowserScope> =
        input.surfaces.iter().map(|surface| surface.scope).collect();
    for required in ScopedBrowserScope::REQUIRED {
        if !present_scopes.contains(&required) {
            push_finding(
                findings,
                ScopedBrowserFindingKind::RequiredScopeMissing,
                format!("required scope `{}` is missing", required.as_str()),
            );
        }
    }

    let mut seen_surface_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &input.surfaces {
        if !seen_surface_ids.insert(surface.surface_id.as_str()) {
            push_finding(
                findings,
                ScopedBrowserFindingKind::DuplicateSurfaceId,
                format!("duplicate surface id `{}`", surface.surface_id),
            );
        }
        check_one_surface(surface, findings);
    }
}

fn check_one_surface(
    surface: &ScopedBrowserSurface,
    findings: &mut Vec<ScopedBrowserValidationFinding>,
) {
    if !surface.scope.is_within_qualified_scope() {
        push_finding(
            findings,
            ScopedBrowserFindingKind::SurfaceScopeOutOfBounds,
            format!(
                "surface `{}` declares out-of-scope `{}`; only docs/review/light-edit are qualified",
                surface.surface_id,
                surface.scope.as_str()
            ),
        );
    }
    if surface.title.trim().is_empty() || surface.headline.trim().is_empty() {
        push_finding(
            findings,
            ScopedBrowserFindingKind::SurfaceTitleOrHeadlineMissing,
            format!(
                "surface `{}` is missing a title or headline",
                surface.surface_id
            ),
        );
    }
    if surface.handoff_reason.note.trim().is_empty() {
        push_finding(
            findings,
            ScopedBrowserFindingKind::HandoffReasonMissing,
            format!(
                "surface `{}` is missing a handoff reason",
                surface.surface_id
            ),
        );
    }
    if surface.return_path.return_ref.trim().is_empty()
        || surface.return_path.label.trim().is_empty()
    {
        push_finding(
            findings,
            ScopedBrowserFindingKind::ReturnPathMissing,
            format!(
                "surface `{}` must keep a return path (return-path safety)",
                surface.surface_id
            ),
        );
    }
    if surface.trust_disclosure_note.trim().is_empty() {
        push_finding(
            findings,
            ScopedBrowserFindingKind::TrustClassDisclosureMissing,
            format!(
                "surface `{}` is missing its trust-class disclosure",
                surface.surface_id
            ),
        );
    }
    if surface.open_raw_escape_ref.trim().is_empty()
        || surface.open_source_escape_ref.trim().is_empty()
    {
        push_finding(
            findings,
            ScopedBrowserFindingKind::OpenRawOpenSourceEscapeMissing,
            format!(
                "surface `{}` must keep open-raw and open-source escapes",
                surface.surface_id
            ),
        );
    }

    // An untrusted destination must stay cited.
    if surface.trust_class.needs_citation() && !surface.cited {
        push_finding(
            findings,
            ScopedBrowserFindingKind::SurfaceNotCited,
            format!(
                "surface `{}` is `{}` but is not cited",
                surface.surface_id,
                surface.trust_class.as_str()
            ),
        );
    }
    // An untrusted destination may never be presented at high confidence.
    if !surface.trust_class.may_be_authoritative()
        && surface.chips.confidence == ScopedBrowserConfidence::High
    {
        push_finding(
            findings,
            ScopedBrowserFindingKind::TrustClassDisclosureCollapsed,
            format!(
                "surface `{}` is `{}` but presented as high confidence",
                surface.surface_id,
                surface.trust_class.as_str()
            ),
        );
    }
    // A handoff blocked by policy may not present as available.
    if surface.handoff_capability == HandoffCapability::BlockedByPolicy
        && surface.captured_vs_live == CapturedVsLive::Live
    {
        push_finding(
            findings,
            ScopedBrowserFindingKind::BlockedHandoffPresentedAvailable,
            format!(
                "surface `{}` is blocked by policy but presented as a live handoff",
                surface.surface_id
            ),
        );
    }
    // A non-current version may not be presented as a confident live match.
    if !surface.chips.version_match.is_confident_current()
        && surface.chips.confidence == ScopedBrowserConfidence::High
        && surface.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            ScopedBrowserFindingKind::VersionTruthCollapsed,
            format!(
                "surface `{}` presents version `{}` as a confident live match",
                surface.surface_id,
                surface.chips.version_match.as_str()
            ),
        );
    }
}

fn check_export(
    input: &ScopedBrowserSurfacesPacketInput,
    findings: &mut Vec<ScopedBrowserValidationFinding>,
) {
    let export = &input.export;
    if !export.preserves_all() {
        push_finding(
            findings,
            ScopedBrowserFindingKind::ExportDropsPreservation,
            "the export must preserve scope, handoff reason, return path, trust class, source class, confidence, and escapes",
        );
    }

    let mut export_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &export.rows {
        export_ids.insert(row.surface_id_ref.as_str());
        let surface = input
            .surfaces
            .iter()
            .find(|surface| surface.surface_id == row.surface_id_ref);
        match surface {
            None => push_finding(
                findings,
                ScopedBrowserFindingKind::ExportRowOrphan,
                format!(
                    "export row references unknown surface `{}`",
                    row.surface_id_ref
                ),
            ),
            Some(surface) => check_export_row(surface, row, findings),
        }
    }

    for surface in &input.surfaces {
        if !export_ids.contains(surface.surface_id.as_str()) {
            push_finding(
                findings,
                ScopedBrowserFindingKind::ExportCoverageMissing,
                format!("surface `{}` has no export row", surface.surface_id),
            );
        }
    }
}

fn check_export_row(
    surface: &ScopedBrowserSurface,
    row: &ScopedBrowserExportRow,
    findings: &mut Vec<ScopedBrowserValidationFinding>,
) {
    if surface.scope != row.scope {
        push_finding(
            findings,
            ScopedBrowserFindingKind::ExportScopeMismatch,
            format!(
                "export for `{}` records scope `{}` but the surface is `{}`",
                row.surface_id_ref,
                row.scope.as_str(),
                surface.scope.as_str()
            ),
        );
    }
    if surface.trust_class != row.trust_class {
        push_finding(
            findings,
            ScopedBrowserFindingKind::ExportTrustClassMismatch,
            format!(
                "export for `{}` records trust `{}` but the surface is `{}`",
                row.surface_id_ref,
                row.trust_class.as_str(),
                surface.trust_class.as_str()
            ),
        );
    }
    if surface.chips.source_class != row.source_class {
        push_finding(
            findings,
            ScopedBrowserFindingKind::ExportSourceClassMismatch,
            format!(
                "export for `{}` records source `{}` but the surface chip is `{}`",
                row.surface_id_ref,
                row.source_class.as_str(),
                surface.chips.source_class.as_str()
            ),
        );
    }
    if surface.chips.confidence != row.confidence {
        push_finding(
            findings,
            ScopedBrowserFindingKind::ExportConfidenceMismatch,
            format!(
                "export for `{}` records confidence `{}` but the surface chip is `{}`",
                row.surface_id_ref,
                row.confidence.as_str(),
                surface.chips.confidence.as_str()
            ),
        );
    }
    // The surface always keeps a return path, so the export row must mark it.
    if !row.has_return_path {
        push_finding(
            findings,
            ScopedBrowserFindingKind::ExportReturnPathMismatch,
            format!(
                "export for `{}` drops the return-path-present flag",
                row.surface_id_ref
            ),
        );
    }
}

fn check_degradations(
    input: &ScopedBrowserSurfacesPacketInput,
    findings: &mut Vec<ScopedBrowserValidationFinding>,
) {
    let surface_ids: BTreeSet<&str> = input
        .surfaces
        .iter()
        .map(|surface| surface.surface_id.as_str())
        .collect();

    for degradation in &input.browser_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                ScopedBrowserFindingKind::DegradationIncomplete,
                format!(
                    "degradation `{}` is missing a summary",
                    degradation.degradation_class.as_str()
                ),
            );
        }
        if let Some(surface_id) = &degradation.surface_id_ref {
            if !surface_id.trim().is_empty() && !surface_ids.contains(surface_id.as_str()) {
                push_finding(
                    findings,
                    ScopedBrowserFindingKind::DegradationOrphan,
                    format!("degradation references unknown surface `{}`", surface_id),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &ScopedBrowserSurfacesPacketInput,
    findings: &mut Vec<ScopedBrowserValidationFinding>,
) {
    let present: BTreeSet<ScopedBrowserConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                ScopedBrowserFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                ScopedBrowserFindingKind::ConsumerProjectionPacketIdMismatch,
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
                ScopedBrowserFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &ScopedBrowserSurfacesPacketInput,
    findings: &mut Vec<ScopedBrowserValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("scoped-browser input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            ScopedBrowserFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw URLs, raw review payloads, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across both the
/// validation findings and the attached degradations.
///
/// A blocking finding (scope, trust, return-path, citation, or boundary
/// violation) blocks the Stable claim; an otherwise-clean set that carries a
/// narrowing degradation narrows below Stable rather than hiding the surfaces.
fn promotion_state(
    findings: &[ScopedBrowserValidationFinding],
    degradations: &[ScopedBrowserDegradation],
) -> ScopedBrowserPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == ScopedBrowserFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == ScopedBrowserFindingSeverity::Blocking);
    if any_blocking {
        return ScopedBrowserPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == ScopedBrowserFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == ScopedBrowserFindingSeverity::Narrowing);
    if any_narrowing {
        ScopedBrowserPromotionState::NarrowedBelowStable
    } else {
        ScopedBrowserPromotionState::Stable
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
                || lower.contains("raw_review:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable scoped-browser input used by the producer, tests, and fixtures.
pub fn seeded_stable_scoped_browser_input() -> ScopedBrowserSurfacesPacketInput {
    let packet_id = "packet:m5:scoped_browser:retry_backoff_handoffs".to_owned();
    ScopedBrowserSurfacesPacketInput {
        packet_id: packet_id.clone(),
        session_label: "scoped browser: reviewing the networking retry backoff change".to_owned(),
        session_digest_ref: "sessiondigest:sha256:net-retry-backoff-review".to_owned(),
        surfaces: vec![docs_reading_surface(), review_surface(), light_edit_surface()],
        export: seeded_export(),
        browser_degradations: vec![ScopedBrowserDegradation {
            degradation_class: ScopedBrowserDegradationClass::MirrorOfflineSnapshot,
            severity: ScopedBrowserFindingSeverity::Advisory,
            summary: "the docs mirror was last synced two days ago; the docs surface is served from the cached snapshot".to_owned(),
            surface_id_ref: Some("surface:docs:tokio_retry_guide".to_owned()),
            evidence_ref: Some("evidence:scoped-browser:mirror-sync-state".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-09T00:00:00Z".to_owned(),
    }
}

fn docs_reading_surface() -> ScopedBrowserSurface {
    ScopedBrowserSurface {
        surface_id: "surface:docs:tokio_retry_guide".to_owned(),
        scope: ScopedBrowserScope::DocsReading,
        subject_ref: "docnode:mirror:tokio/retry-backoff#exponential-backoff".to_owned(),
        title: "Exponential backoff guidance (mirrored docs)".to_owned(),
        headline: "a scoped docs-reading handoff to the pinned, signed mirror of the upstream retry guide".to_owned(),
        chips: ScopedBrowserChipSet {
            source_class: ScopedBrowserSourceClass::MirroredOfficialDocs,
            version_match: ScopedBrowserVersionMatch::CompatibleMinorDrift,
            freshness: ScopedBrowserFreshness::WarmCached,
            locality: ScopedBrowserLocality::MirroredPack,
            confidence: ScopedBrowserConfidence::Medium,
        },
        trust_class: ScopedBrowserTrustClass::SignedMirrorVerified,
        trust_disclosure_note: "signed mirror of the upstream docs; verified at pin time, served from a cached snapshot".to_owned(),
        handoff_reason: HandoffReason {
            reason_kind: HandoffReasonKind::ExactAnchorUnavailableLocally,
            note: "the exact backoff anchor is only on the upstream page; the inline peek could not resolve it locally".to_owned(),
        },
        return_path: ReturnPath {
            return_kind: ReturnPathKind::BackToInlinePeek,
            return_ref: "return:peek:symbol:aureline-net::retry::retry_with_backoff".to_owned(),
            label: "Back to the retry_with_backoff peek".to_owned(),
        },
        handoff_capability: HandoffCapability::AvailableExplicit,
        captured_vs_live: CapturedVsLive::CapturedSnapshot,
        cited: true,
        citation_ref: Some("cite:docnode:mirror:tokio/retry-backoff".to_owned()),
        open_raw_escape_ref: "open-raw:docnode:mirror:tokio/retry-backoff#exponential-backoff"
            .to_owned(),
        open_source_escape_ref: "open-source:mirror:tokio/retry-backoff".to_owned(),
    }
}

fn review_surface() -> ScopedBrowserSurface {
    ScopedBrowserSurface {
        surface_id: "surface:review:retry_backoff_thread".to_owned(),
        scope: ScopedBrowserScope::Review,
        subject_ref: "reviewthread:host:retry-backoff/thread-7".to_owned(),
        title: "Review thread: retry/backoff change".to_owned(),
        headline: "a scoped review handoff to the hosted review thread for the backoff change".to_owned(),
        chips: ScopedBrowserChipSet {
            source_class: ScopedBrowserSourceClass::ReviewHost,
            version_match: ScopedBrowserVersionMatch::ExactBuildMatch,
            freshness: ScopedBrowserFreshness::AuthoritativeLive,
            locality: ScopedBrowserLocality::Managed,
            confidence: ScopedBrowserConfidence::Medium,
        },
        trust_class: ScopedBrowserTrustClass::LiveProviderHandoff,
        trust_disclosure_note: "live handoff to the hosted review provider; not verified at materialization time, held to medium".to_owned(),
        handoff_reason: HandoffReason {
            reason_kind: HandoffReasonKind::ReviewThreadRequiresHostedView,
            note: "the review thread and its inline comments live on the hosted review surface".to_owned(),
        },
        return_path: ReturnPath {
            return_kind: ReturnPathKind::BackToReviewPanel,
            return_ref: "return:review-panel:retry-backoff".to_owned(),
            label: "Back to the review panel".to_owned(),
        },
        handoff_capability: HandoffCapability::AvailableExplicit,
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:reviewthread:host:retry-backoff/thread-7".to_owned()),
        open_raw_escape_ref: "open-raw:reviewthread:host:retry-backoff/thread-7".to_owned(),
        open_source_escape_ref: "open-source:review-host:retry-backoff/thread-7".to_owned(),
    }
}

fn light_edit_surface() -> ScopedBrowserSurface {
    ScopedBrowserSurface {
        surface_id: "surface:light_edit:retry_doc_comment".to_owned(),
        scope: ScopedBrowserScope::LightEdit,
        subject_ref: "docnode:project:crates/aureline-net/src/retry.rs#doc".to_owned(),
        title: "Light edit: retry doc comment".to_owned(),
        headline: "a scoped light-edit handoff to fix a typo in the local retry doc comment"
            .to_owned(),
        chips: ScopedBrowserChipSet {
            source_class: ScopedBrowserSourceClass::ProjectDocs,
            version_match: ScopedBrowserVersionMatch::ExactBuildMatch,
            freshness: ScopedBrowserFreshness::AuthoritativeLive,
            locality: ScopedBrowserLocality::Local,
            confidence: ScopedBrowserConfidence::High,
        },
        trust_class: ScopedBrowserTrustClass::FirstPartyAuthoritative,
        trust_disclosure_note:
            "first-party workspace doc; the light edit stays local and authoritative".to_owned(),
        handoff_reason: HandoffReason {
            reason_kind: HandoffReasonKind::LightEditRequiresScopedEditor,
            note: "the typo fix opens a scoped editor surface over the local doc comment"
                .to_owned(),
        },
        return_path: ReturnPath {
            return_kind: ReturnPathKind::BackToWorkspace,
            return_ref: "return:workspace:crates/aureline-net/src/retry.rs".to_owned(),
            label: "Back to the workspace".to_owned(),
        },
        handoff_capability: HandoffCapability::NotRequiredLocal,
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:docnode:project:crates/aureline-net/src/retry.rs".to_owned()),
        open_raw_escape_ref: "open-raw:docnode:project:crates/aureline-net/src/retry.rs#doc"
            .to_owned(),
        open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
    }
}

fn seeded_export() -> ScopedBrowserExport {
    ScopedBrowserExport {
        scope: ScopedBrowserExportScope::AllSurfaces,
        preserves_scope: true,
        preserves_handoff_reason: true,
        preserves_return_path: true,
        preserves_trust_class: true,
        preserves_source_class: true,
        preserves_confidence: true,
        preserves_open_raw_open_source_escape: true,
        rows: vec![
            ScopedBrowserExportRow {
                surface_id_ref: "surface:docs:tokio_retry_guide".to_owned(),
                scope: ScopedBrowserScope::DocsReading,
                trust_class: ScopedBrowserTrustClass::SignedMirrorVerified,
                source_class: ScopedBrowserSourceClass::MirroredOfficialDocs,
                confidence: ScopedBrowserConfidence::Medium,
                handoff_capability: HandoffCapability::AvailableExplicit,
                has_return_path: true,
                cited: true,
                open_raw_escape_ref:
                    "open-raw:docnode:mirror:tokio/retry-backoff#exponential-backoff".to_owned(),
                open_source_escape_ref: "open-source:mirror:tokio/retry-backoff".to_owned(),
            },
            ScopedBrowserExportRow {
                surface_id_ref: "surface:review:retry_backoff_thread".to_owned(),
                scope: ScopedBrowserScope::Review,
                trust_class: ScopedBrowserTrustClass::LiveProviderHandoff,
                source_class: ScopedBrowserSourceClass::ReviewHost,
                confidence: ScopedBrowserConfidence::Medium,
                handoff_capability: HandoffCapability::AvailableExplicit,
                has_return_path: true,
                cited: true,
                open_raw_escape_ref: "open-raw:reviewthread:host:retry-backoff/thread-7".to_owned(),
                open_source_escape_ref: "open-source:review-host:retry-backoff/thread-7".to_owned(),
            },
            ScopedBrowserExportRow {
                surface_id_ref: "surface:light_edit:retry_doc_comment".to_owned(),
                scope: ScopedBrowserScope::LightEdit,
                trust_class: ScopedBrowserTrustClass::FirstPartyAuthoritative,
                source_class: ScopedBrowserSourceClass::ProjectDocs,
                confidence: ScopedBrowserConfidence::High,
                handoff_capability: HandoffCapability::NotRequiredLocal,
                has_return_path: true,
                cited: true,
                open_raw_escape_ref:
                    "open-raw:docnode:project:crates/aureline-net/src/retry.rs#doc".to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs"
                    .to_owned(),
            },
        ],
    }
}

fn required_projections(packet_id: &str) -> Vec<ScopedBrowserConsumerProjection> {
    [
        ScopedBrowserConsumerSurface::DocsBrowserShell,
        ScopedBrowserConsumerSurface::ReviewSurface,
        ScopedBrowserConsumerSurface::BrowserHandoffPacket,
        ScopedBrowserConsumerSurface::PeekOverlay,
        ScopedBrowserConsumerSurface::AiContextInspector,
        ScopedBrowserConsumerSurface::CliHeadless,
        ScopedBrowserConsumerSurface::SupportExport,
        ScopedBrowserConsumerSurface::Diagnostics,
        ScopedBrowserConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| ScopedBrowserConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_scopes: true,
        preserves_trust_classes: true,
        preserves_handoff_reasons: true,
        preserves_return_paths: true,
        preserves_open_raw_open_source_escape: true,
    })
    .collect()
}
