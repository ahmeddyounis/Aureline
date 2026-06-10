//! Browser-lite light remote edit surfaces with narrow scope, stale-state
//! honesty, and no hidden authority expansion.
//!
//! This module implements the M5 light-remote-edit boundary: the browser-lite
//! surface that lets the reader make a *small, scoped* edit — fixing a doc
//! comment, adjusting a config value, or replying to a review — through a
//! narrow remote or local edit surface without ever looking like a full editor,
//! a multi-file refactor, or a general automation runtime. Each
//! [`LightRemoteEditSurface`] carries one [`LightRemoteEditScope`] (the
//! qualified `doc_comment_edit` / `single_file_text_edit` / `config_value_edit`
//! / `review_reply` scope; anything broader is recorded and blocked), an
//! explicit [`EditIntent`] (*why* the edit was offered), a non-empty
//! [`ReturnPath`] (*how the reader gets back* — the return-path safety
//! guarantee), one [`EditTrustClass`] disclosure, a [`StaleStateDisclosure`]
//! (*what base state the edit was prepared against, and whether a stale base is
//! disclosed* — the stale-state honesty guarantee), the granted vs. effective
//! [`AuthorityScope`] (*the no-hidden-authority-expansion guarantee*), the same
//! source/version/freshness/locality/confidence chip set the other docs lanes
//! use, the [`ApplyPosture`], the live-vs-captured state, citation state, and
//! the open-raw / open-source escapes.
//!
//! Three invariants make a light remote edit honest:
//!
//! - **Narrow scope.** Only the four qualified edit scopes are in bounds; a
//!   surface declaring `multi_file_refactor`, `repo_wide_automation`, or
//!   `arbitrary_command_execution` is recorded and blocks promotion.
//! - **Stale-state honesty.** An edit prepared against a stale or unknown base
//!   state must disclose it, and may not be presented at high confidence.
//! - **No hidden authority expansion.** The edit's effective authority may
//!   never exceed the authority the user/policy granted, and may never exceed
//!   the authority the surface's scope permits.
//!
//! The [`LightRemoteEditExport`] is the cited projection support, AI evidence,
//! and diagnostics surfaces ingest: one [`LightRemoteEditExportRow`] per surface
//! preserving scope, trust class, source class, confidence, apply posture,
//! granted/effective authority, base-state disclosure, return-path presence,
//! citation state, and the open-raw / open-source escapes.
//!
//! [`LightRemoteEditSurfacesPacket::materialize`] computes the validation
//! findings and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input, so an out-of-scope, return-path-unsafe,
//! trust-collapsed, stale-state-dishonest, authority-expanding, uncited, or
//! unattributed surface set automatically narrows or blocks before it reaches a
//! consumer surface. The packet is an inspectable, serde-serializable truth
//! packet: it carries no raw page bodies, no raw URLs, no raw edit diffs, no raw
//! source files, no raw provider payloads, and no credentials — only metadata,
//! scope truth, edit intents, return paths, trust disclosures, stale-state
//! disclosures, authority truth, chip truth, cited refs, provenance, finding
//! summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/add-browser-lite-light-remote-edit-surfaces-with-narrow-scope-stale-state-honesty-and-no-hidden-authority-expa.schema.json`](../../../../schemas/docs/add-browser-lite-light-remote-edit-surfaces-with-narrow-scope-stale-state-honesty-and-no-hidden-authority-expa.schema.json).
//! The contract doc is
//! [`docs/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa.md`](../../../../docs/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/`](../../../../fixtures/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`LightRemoteEditSurfacesPacket`].
pub const LIGHT_REMOTE_EDIT_RECORD_KIND: &str = "light_remote_edit_surfaces";

/// Record-kind tag carried by the support-export wrapper.
pub const LIGHT_REMOTE_EDIT_SUPPORT_EXPORT_RECORD_KIND: &str =
    "light_remote_edit_surfaces_support_export";

/// Schema version for light-remote-edit-surface records.
pub const LIGHT_REMOTE_EDIT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const LIGHT_REMOTE_EDIT_SCHEMA_REF: &str =
    "schemas/docs/add-browser-lite-light-remote-edit-surfaces-with-narrow-scope-stale-state-honesty-and-no-hidden-authority-expa.schema.json";

/// Repo-relative path of the light-remote-edit contract doc.
pub const LIGHT_REMOTE_EDIT_DOC_REF: &str =
    "docs/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa.md";

/// Repo-relative path of the protected fixture directory.
pub const LIGHT_REMOTE_EDIT_FIXTURE_DIR: &str =
    "fixtures/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa";

/// Repo-relative path of the checked support-export artifact.
pub const LIGHT_REMOTE_EDIT_ARTIFACT_REF: &str =
    "artifacts/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const LIGHT_REMOTE_EDIT_SUMMARY_REF: &str =
    "artifacts/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa.md";

/// The scope a light remote edit surface is qualified for.
///
/// Only `doc_comment_edit`, `single_file_text_edit`, `config_value_edit`, and
/// `review_reply` are inside the qualified M5 scope. `multi_file_refactor`,
/// `repo_wide_automation`, and `arbitrary_command_execution` are recorded so the
/// validator can detect and block a surface that overruns the qualified scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightRemoteEditScope {
    /// A narrow edit to a documentation comment.
    DocCommentEdit,
    /// A narrow single-file text edit.
    SingleFileTextEdit,
    /// A narrow edit to a single configuration value.
    ConfigValueEdit,
    /// A narrow reply to a review comment.
    ReviewReply,
    /// A multi-file refactor — outside the qualified M5 scope.
    MultiFileRefactor,
    /// Repo-wide automation — outside the qualified M5 scope.
    RepoWideAutomation,
    /// Arbitrary command execution — outside the qualified M5 scope.
    ArbitraryCommandExecution,
}

impl LightRemoteEditScope {
    /// The scopes a packet must cover (`doc_comment_edit` and
    /// `single_file_text_edit`).
    pub const REQUIRED: [Self; 2] = [Self::DocCommentEdit, Self::SingleFileTextEdit];

    /// Stable token recorded in the surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocCommentEdit => "doc_comment_edit",
            Self::SingleFileTextEdit => "single_file_text_edit",
            Self::ConfigValueEdit => "config_value_edit",
            Self::ReviewReply => "review_reply",
            Self::MultiFileRefactor => "multi_file_refactor",
            Self::RepoWideAutomation => "repo_wide_automation",
            Self::ArbitraryCommandExecution => "arbitrary_command_execution",
        }
    }

    /// Whether this scope is inside the qualified light-edit scope.
    pub const fn is_within_qualified_scope(self) -> bool {
        matches!(
            self,
            Self::DocCommentEdit
                | Self::SingleFileTextEdit
                | Self::ConfigValueEdit
                | Self::ReviewReply
        )
    }

    /// The most authority a surface of this scope may exercise. An effective
    /// authority above this ceiling is a hidden authority expansion.
    pub const fn max_authority(self) -> AuthorityScope {
        match self {
            Self::DocCommentEdit | Self::SingleFileTextEdit => AuthorityScope::SingleFileWrite,
            Self::ConfigValueEdit | Self::ReviewReply => AuthorityScope::SingleFieldWrite,
            Self::MultiFileRefactor => AuthorityScope::MultiFileWrite,
            Self::RepoWideAutomation | Self::ArbitraryCommandExecution => AuthorityScope::RepoWide,
        }
    }
}

/// The authority an edit holds or exercises, ordered from least to most.
///
/// The variants are declared in ascending order, so the derived [`Ord`] lets
/// the validator compare an effective authority against a granted authority or
/// a scope ceiling: a larger value is a wider authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityScope {
    /// Read-only; no write authority.
    ReadOnly,
    /// Authority to write a single field / value.
    SingleFieldWrite,
    /// Authority to write within a single file.
    SingleFileWrite,
    /// Authority to write across multiple files.
    MultiFileWrite,
    /// Repo-wide authority.
    RepoWide,
}

impl AuthorityScope {
    /// Stable token recorded in the surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::SingleFieldWrite => "single_field_write",
            Self::SingleFileWrite => "single_file_write",
            Self::MultiFileWrite => "multi_file_write",
            Self::RepoWide => "repo_wide",
        }
    }
}

/// Why the product offered this light remote edit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditIntentKind {
    /// Fix a typo in a doc comment.
    FixDocTypo,
    /// Adjust a single configuration value.
    AdjustConfigValue,
    /// Apply a suggestion raised in review.
    ApplyReviewSuggestion,
    /// Reply to a review comment.
    ReplyToReviewComment,
    /// A small inline correction.
    SmallInlineCorrection,
}

impl EditIntentKind {
    /// Stable token recorded in the edit intent.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FixDocTypo => "fix_doc_typo",
            Self::AdjustConfigValue => "adjust_config_value",
            Self::ApplyReviewSuggestion => "apply_review_suggestion",
            Self::ReplyToReviewComment => "reply_to_review_comment",
            Self::SmallInlineCorrection => "small_inline_correction",
        }
    }
}

/// Where the reader returns to when leaving a light remote edit surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnPathKind {
    /// Back to the inline peek the edit came from.
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

/// Trust class of the light remote edit destination, projected as a disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditTrustClass {
    /// First-party workspace content (the workspace's own files).
    FirstPartyWorkspace,
    /// A pinned, signed mirror-backed suggestion.
    SignedMirrorBackedSuggestion,
    /// A signed extension / imported pack suggestion.
    ExtensionPackSuggestion,
    /// A live provider edit surface — not verified at materialization time.
    LiveProviderEditSurface,
    /// A derived / inferred suggestion only.
    DerivedSuggestionOnly,
}

impl EditTrustClass {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyWorkspace => "first_party_workspace",
            Self::SignedMirrorBackedSuggestion => "signed_mirror_backed_suggestion",
            Self::ExtensionPackSuggestion => "extension_pack_suggestion",
            Self::LiveProviderEditSurface => "live_provider_edit_surface",
            Self::DerivedSuggestionOnly => "derived_suggestion_only",
        }
    }

    /// Whether this trust class may back a high-confidence / authoritative
    /// claim. A live provider edit surface or derived suggestion may not.
    pub const fn may_be_authoritative(self) -> bool {
        matches!(
            self,
            Self::FirstPartyWorkspace
                | Self::SignedMirrorBackedSuggestion
                | Self::ExtensionPackSuggestion
        )
    }

    /// Whether a surface of this trust class must stay cited.
    pub const fn needs_citation(self) -> bool {
        !matches!(self, Self::FirstPartyWorkspace)
    }
}

/// Whether and how the edit may be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ApplyPosture {
    /// The edit applies directly to local content.
    LocalDirectApply,
    /// A remote apply is available and explicit.
    RemoteApplyAvailable,
    /// Apply is blocked by policy.
    ApplyBlockedByPolicy,
    /// Apply is unavailable and disclosed as such.
    ApplyUnavailableDisclosed,
}

impl ApplyPosture {
    /// Stable token recorded in the surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalDirectApply => "local_direct_apply",
            Self::RemoteApplyAvailable => "remote_apply_available",
            Self::ApplyBlockedByPolicy => "apply_blocked_by_policy",
            Self::ApplyUnavailableDisclosed => "apply_unavailable_disclosed",
        }
    }

    /// Whether the surface may present the apply as an available action.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::LocalDirectApply | Self::RemoteApplyAvailable)
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

/// The base state an edit was prepared against, projected for stale-state honesty.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BaseStateKind {
    /// The edit was prepared against the current live head.
    LiveHead,
    /// The edit was prepared against a recent, in-window snapshot.
    WarmSnapshot,
    /// The edit was prepared against a known-stale snapshot.
    StaleSnapshot,
    /// The base state could not be verified.
    UnknownBase,
}

impl BaseStateKind {
    /// Stable token recorded in the disclosure.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveHead => "live_head",
            Self::WarmSnapshot => "warm_snapshot",
            Self::StaleSnapshot => "stale_snapshot",
            Self::UnknownBase => "unknown_base",
        }
    }

    /// Whether this base state requires an explicit stale disclosure. A stale or
    /// unknown base must be disclosed (the stale-state honesty guarantee).
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::StaleSnapshot | Self::UnknownBase)
    }

    /// Whether this base state may be presented at high confidence.
    pub const fn may_be_confident(self) -> bool {
        matches!(self, Self::LiveHead | Self::WarmSnapshot)
    }
}

/// Source class for a surface's underlying material, projected as the source chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditSourceClass {
    /// Workspace-local project docs.
    ProjectDocs,
    /// Pinned, signed mirror of official upstream docs.
    MirroredOfficialDocs,
    /// An imported / extension docs pack.
    ExtensionDocsPack,
    /// A live external provider surface (handed off, not mirrored).
    LiveProviderSurface,
    /// A hosted code-review thread / diff host.
    ReviewHost,
    /// Generated API / reference docs.
    GeneratedReference,
    /// Derived / inferred suggestion.
    DerivedSuggestion,
}

impl EditSourceClass {
    /// Stable token recorded in the chip.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectDocs => "project_docs",
            Self::MirroredOfficialDocs => "mirrored_official_docs",
            Self::ExtensionDocsPack => "extension_docs_pack",
            Self::LiveProviderSurface => "live_provider_surface",
            Self::ReviewHost => "review_host",
            Self::GeneratedReference => "generated_reference",
            Self::DerivedSuggestion => "derived_suggestion",
        }
    }
}

/// Version-match state for a surface, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditVersionMatch {
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

impl EditVersionMatch {
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
pub enum EditFreshness {
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

impl EditFreshness {
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
pub enum EditLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through a pinned mirror pack.
    MirroredPack,
    /// Resolved through a remote helper.
    RemoteHelper,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl EditLocality {
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
pub enum EditConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified claim.
    Heuristic,
}

impl EditConfidence {
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
pub enum LightRemoteEditFindingSeverity {
    /// Blocks a Stable claim; the set must block.
    Blocking,
    /// Narrows below Stable but the set stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl LightRemoteEditFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the light-remote-edit packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightRemoteEditConsumerSurface {
    /// The docs browser shell.
    DocsBrowserShell,
    /// The review surface.
    ReviewSurface,
    /// The light-edit surface.
    LightEditSurface,
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

impl LightRemoteEditConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::ReviewSurface => "review_surface",
            Self::LightEditSurface => "light_edit_surface",
            Self::PeekOverlay => "peek_overlay",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level light-remote-edit degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightRemoteEditDegradationClass {
    /// A mirror is offline; the surface is served from the last snapshot.
    MirrorOfflineSnapshot,
    /// A live provider was unreachable; a captured snapshot is shown instead.
    LiveProviderUnreachableCapturedSnapshot,
    /// Apply was blocked by policy.
    ApplyBlockedByPolicy,
    /// The return path is degraded (still present, but reduced).
    ReturnPathDegraded,
    /// The surface was rerun at a narrowed scope.
    ScopeNarrowedRerun,
    /// The edit was prepared against a stale base and the claim was narrowed.
    StaleBaseStateNarrowed,
    /// The edit's authority was narrowed (e.g. to read-only) before publication.
    AuthorityNarrowed,
    /// A referenced anchor is broken.
    BrokenAnchor,
    /// The owning source is quarantined.
    QuarantinedSource,
}

impl LightRemoteEditDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::LiveProviderUnreachableCapturedSnapshot => {
                "live_provider_unreachable_captured_snapshot"
            }
            Self::ApplyBlockedByPolicy => "apply_blocked_by_policy",
            Self::ReturnPathDegraded => "return_path_degraded",
            Self::ScopeNarrowedRerun => "scope_narrowed_rerun",
            Self::StaleBaseStateNarrowed => "stale_base_state_narrowed",
            Self::AuthorityNarrowed => "authority_narrowed",
            Self::BrokenAnchor => "broken_anchor",
            Self::QuarantinedSource => "quarantined_source",
        }
    }
}

/// Scope a light-remote-edit export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightRemoteEditExportScope {
    /// Every surface in the packet.
    AllSurfaces,
    /// Text-edit surfaces only.
    TextEditOnly,
    /// Config-edit surfaces only.
    ConfigEditOnly,
    /// Review-reply surfaces only.
    ReviewReplyOnly,
}

impl LightRemoteEditExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllSurfaces => "all_surfaces",
            Self::TextEditOnly => "text_edit_only",
            Self::ConfigEditOnly => "config_edit_only",
            Self::ReviewReplyOnly => "review_reply_only",
        }
    }
}

/// Promotion state computed for the light-remote-edit packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightRemoteEditPromotionState {
    /// Set qualifies for the Stable claim.
    Stable,
    /// Set narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Set has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl LightRemoteEditPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`LightRemoteEditSurfacesPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LightRemoteEditFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The surface set is empty.
    SurfacesEmpty,
    /// A surface id is duplicated.
    DuplicateSurfaceId,
    /// A required scope (doc-comment / single-file-text) is missing.
    RequiredScopeMissing,
    /// A surface declares a scope outside the qualified light-edit scope.
    SurfaceScopeOutOfBounds,
    /// A surface is missing its title or headline.
    SurfaceTitleOrHeadlineMissing,
    /// A surface is missing its edit intent.
    EditIntentMissing,
    /// A surface is missing its return path (return-path safety violation).
    ReturnPathMissing,
    /// A surface is missing its trust-class disclosure note.
    TrustClassDisclosureMissing,
    /// An untrusted destination is presented as a high-confidence claim.
    TrustClassDisclosureCollapsed,
    /// An apply blocked by policy is presented as available.
    BlockedApplyPresentedAvailable,
    /// A surface is missing an open-raw / open-source escape ref.
    OpenRawOpenSourceEscapeMissing,
    /// An imported / live / derived surface is not cited.
    SurfaceNotCited,
    /// A non-current version-match is presented as a confident live match.
    VersionTruthCollapsed,
    /// A stale or unknown base state is not disclosed (stale-state honesty).
    StaleStateNotDisclosed,
    /// A stale or unknown base state is presented at high confidence.
    StaleStatePresentedConfident,
    /// The effective authority exceeds the granted authority (hidden expansion).
    AuthorityExpansionDetected,
    /// The effective authority exceeds the authority the scope permits.
    ScopeAuthorityMismatch,
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
    /// An export row's granted/effective authority disagrees with the surface.
    ExportAuthorityMismatch,
    /// An export row's base-state disclosure disagrees with the surface.
    ExportStaleStateMismatch,
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
    /// Raw bodies, raw URLs, raw edit diffs, or secrets crossed the boundary.
    RawBoundaryMaterialPresent,
}

impl LightRemoteEditFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::SurfacesEmpty => "surfaces_empty",
            Self::DuplicateSurfaceId => "duplicate_surface_id",
            Self::RequiredScopeMissing => "required_scope_missing",
            Self::SurfaceScopeOutOfBounds => "surface_scope_out_of_bounds",
            Self::SurfaceTitleOrHeadlineMissing => "surface_title_or_headline_missing",
            Self::EditIntentMissing => "edit_intent_missing",
            Self::ReturnPathMissing => "return_path_missing",
            Self::TrustClassDisclosureMissing => "trust_class_disclosure_missing",
            Self::TrustClassDisclosureCollapsed => "trust_class_disclosure_collapsed",
            Self::BlockedApplyPresentedAvailable => "blocked_apply_presented_available",
            Self::OpenRawOpenSourceEscapeMissing => "open_raw_open_source_escape_missing",
            Self::SurfaceNotCited => "surface_not_cited",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::StaleStateNotDisclosed => "stale_state_not_disclosed",
            Self::StaleStatePresentedConfident => "stale_state_presented_confident",
            Self::AuthorityExpansionDetected => "authority_expansion_detected",
            Self::ScopeAuthorityMismatch => "scope_authority_mismatch",
            Self::ExportRowOrphan => "export_row_orphan",
            Self::ExportCoverageMissing => "export_coverage_missing",
            Self::ExportDropsPreservation => "export_drops_preservation",
            Self::ExportScopeMismatch => "export_scope_mismatch",
            Self::ExportTrustClassMismatch => "export_trust_class_mismatch",
            Self::ExportSourceClassMismatch => "export_source_class_mismatch",
            Self::ExportConfidenceMismatch => "export_confidence_mismatch",
            Self::ExportReturnPathMismatch => "export_return_path_mismatch",
            Self::ExportAuthorityMismatch => "export_authority_mismatch",
            Self::ExportStaleStateMismatch => "export_stale_state_mismatch",
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
    pub const fn default_severity(self) -> LightRemoteEditFindingSeverity {
        LightRemoteEditFindingSeverity::Blocking
    }
}

/// The chip set rendered for one light remote edit surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditChipSet {
    /// Source-class chip.
    pub source_class: EditSourceClass,
    /// Version-match chip.
    pub version_match: EditVersionMatch,
    /// Freshness chip.
    pub freshness: EditFreshness,
    /// Locality chip.
    pub locality: EditLocality,
    /// Confidence chip (the confidence label).
    pub confidence: EditConfidence,
}

/// Why the product offered this light remote edit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditIntent {
    /// Kind of edit intent.
    pub intent_kind: EditIntentKind,
    /// Human-readable explanation (no raw diffs / no raw bodies).
    pub note: String,
}

/// How the reader returns from this light remote edit surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReturnPath {
    /// Kind of return path.
    pub return_kind: ReturnPathKind,
    /// Stable destination ref the reader returns to (no raw URLs).
    pub return_ref: String,
    /// Human-readable return label.
    pub label: String,
}

/// The authority a surface holds vs. exercises (no hidden authority expansion).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthorityGrant {
    /// The authority the user/policy granted.
    pub granted: AuthorityScope,
    /// The authority the edit actually exercises.
    pub effective: AuthorityScope,
}

impl AuthorityGrant {
    /// Whether the effective authority exceeds the granted authority.
    pub fn is_expansion(&self) -> bool {
        self.effective > self.granted
    }
}

/// The base state an edit was prepared against (stale-state honesty).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleStateDisclosure {
    /// The base state kind.
    pub base_state_kind: BaseStateKind,
    /// Whether a stale or uncertain base is disclosed to the reader.
    pub disclosed: bool,
    /// Human-readable disclosure note (no raw bodies).
    pub note: String,
}

/// One light remote edit surface — one bounded, scoped edit handoff.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditSurface {
    /// Stable surface id within this packet.
    pub surface_id: String,
    /// The qualified scope this surface is for.
    pub scope: LightRemoteEditScope,
    /// Subject ref the surface points at (no raw URL / no raw body).
    pub subject_ref: String,
    /// Human-readable title.
    pub title: String,
    /// Human-readable headline / summary (no raw bodies).
    pub headline: String,
    /// Source/version/freshness/locality/confidence chips.
    pub chips: LightRemoteEditChipSet,
    /// The trust-class disclosure for the destination.
    pub trust_class: EditTrustClass,
    /// Human-readable trust-class disclosure note.
    pub trust_disclosure_note: String,
    /// Why the product offered this edit.
    pub edit_intent: EditIntent,
    /// How the reader returns (return-path safety).
    pub return_path: ReturnPath,
    /// The granted vs. effective authority (no hidden authority expansion).
    pub authority: AuthorityGrant,
    /// The base-state disclosure (stale-state honesty).
    pub stale_state: StaleStateDisclosure,
    /// Whether and how the edit may be applied.
    pub apply_posture: ApplyPosture,
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
pub struct LightRemoteEditExportRow {
    /// The surface this export row mirrors.
    pub surface_id_ref: String,
    /// Scope (must match the surface).
    pub scope: LightRemoteEditScope,
    /// Trust class (must match the surface).
    pub trust_class: EditTrustClass,
    /// Source class (must match the surface's chip).
    pub source_class: EditSourceClass,
    /// Confidence (must match the surface's chip).
    pub confidence: EditConfidence,
    /// Apply posture (must match the surface).
    pub apply_posture: ApplyPosture,
    /// Granted authority (must match the surface).
    pub granted_authority: AuthorityScope,
    /// Effective authority (must match the surface).
    pub effective_authority: AuthorityScope,
    /// Base state kind (must match the surface).
    pub base_state_kind: BaseStateKind,
    /// Whether a stale base is disclosed (must match the surface).
    pub stale_disclosed: bool,
    /// Whether the surface keeps a return path.
    pub has_return_path: bool,
    /// Whether the surface is cited.
    pub cited: bool,
    /// Open-raw escape ref.
    pub open_raw_escape_ref: String,
    /// Open-source escape ref.
    pub open_source_escape_ref: String,
}

/// The light-remote-edit export projection for the surface set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditExport {
    /// Scope this export covers.
    pub scope: LightRemoteEditExportScope,
    /// Whether the export preserves each surface's scope.
    pub preserves_scope: bool,
    /// Whether the export preserves each surface's edit intent.
    pub preserves_edit_intent: bool,
    /// Whether the export preserves each surface's return path.
    pub preserves_return_path: bool,
    /// Whether the export preserves each surface's trust class.
    pub preserves_trust_class: bool,
    /// Whether the export preserves each surface's source class.
    pub preserves_source_class: bool,
    /// Whether the export preserves each surface's confidence label.
    pub preserves_confidence: bool,
    /// Whether the export preserves each surface's authority truth.
    pub preserves_authority: bool,
    /// Whether the export preserves each surface's base-state disclosure.
    pub preserves_stale_state: bool,
    /// Whether the export preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
    /// Per-surface export rows.
    pub rows: Vec<LightRemoteEditExportRow>,
}

impl LightRemoteEditExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_scope
            && self.preserves_edit_intent
            && self.preserves_return_path
            && self.preserves_trust_class
            && self.preserves_source_class
            && self.preserves_confidence
            && self.preserves_authority
            && self.preserves_stale_state
            && self.preserves_open_raw_open_source_escape
    }
}

/// A packet-level light-remote-edit degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditDegradation {
    /// Degradation class.
    pub degradation_class: LightRemoteEditDegradationClass,
    /// Severity.
    pub severity: LightRemoteEditFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The surface this degradation annotates, if scoped to one surface.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the light-remote-edit set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditConsumerProjection {
    /// Surface that consumes the set.
    pub surface: LightRemoteEditConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the chip set verbatim.
    pub preserves_chips: bool,
    /// Whether the surface preserves all scopes.
    pub preserves_scopes: bool,
    /// Whether the surface preserves the trust classes.
    pub preserves_trust_classes: bool,
    /// Whether the surface preserves the edit intents.
    pub preserves_edit_intents: bool,
    /// Whether the surface preserves the return paths.
    pub preserves_return_paths: bool,
    /// Whether the surface preserves the authority truth.
    pub preserves_authority: bool,
    /// Whether the surface preserves the base-state disclosures.
    pub preserves_stale_state: bool,
    /// Whether the surface preserves the open-raw / open-source escapes.
    pub preserves_open_raw_open_source_escape: bool,
}

impl LightRemoteEditConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_chips
            && self.preserves_scopes
            && self.preserves_trust_classes
            && self.preserves_edit_intents
            && self.preserves_return_paths
            && self.preserves_authority
            && self.preserves_stale_state
            && self.preserves_open_raw_open_source_escape
    }
}

/// A single validation finding on the light-remote-edit set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditValidationFinding {
    /// Finding kind.
    pub finding_kind: LightRemoteEditFindingKind,
    /// Finding severity.
    pub severity: LightRemoteEditFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`LightRemoteEditSurfacesPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditSurfacesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label (no raw URLs / no raw query text).
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The light remote edit surfaces.
    pub surfaces: Vec<LightRemoteEditSurface>,
    /// The export projection.
    pub export: LightRemoteEditExport,
    /// Packet-level degradations.
    pub edit_degradations: Vec<LightRemoteEditDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<LightRemoteEditConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe light-remote-edit packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditSurfacesPacket {
    /// Record kind; must equal [`LIGHT_REMOTE_EDIT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`LIGHT_REMOTE_EDIT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable session label.
    pub session_label: String,
    /// Opaque digest/ref for the session.
    pub session_digest_ref: String,
    /// The light remote edit surfaces.
    pub surfaces: Vec<LightRemoteEditSurface>,
    /// The export projection.
    pub export: LightRemoteEditExport,
    /// Packet-level degradations.
    pub edit_degradations: Vec<LightRemoteEditDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<LightRemoteEditConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: LightRemoteEditPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<LightRemoteEditValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every light-remote-edit packet must project.
const REQUIRED_SURFACES: [LightRemoteEditConsumerSurface; 4] = [
    LightRemoteEditConsumerSurface::DocsBrowserShell,
    LightRemoteEditConsumerSurface::ReviewSurface,
    LightRemoteEditConsumerSurface::LightEditSurface,
    LightRemoteEditConsumerSurface::SupportExport,
];

impl LightRemoteEditSurfacesPacket {
    /// Materializes a light-remote-edit packet, computing validation findings
    /// and the promotion state from the input.
    pub fn materialize(input: LightRemoteEditSurfacesPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_surfaces(&input, &mut findings);
        check_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.edit_degradations);

        Self {
            record_kind: LIGHT_REMOTE_EDIT_RECORD_KIND.to_owned(),
            schema_version: LIGHT_REMOTE_EDIT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            session_label: input.session_label,
            session_digest_ref: input.session_digest_ref,
            surfaces: input.surfaces,
            export: input.export,
            edit_degradations: input.edit_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the set qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == LightRemoteEditPromotionState::Stable
            && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> LightRemoteEditSupportExport {
        LightRemoteEditSupportExport {
            record_kind: LIGHT_REMOTE_EDIT_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: LIGHT_REMOTE_EDIT_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: LIGHT_REMOTE_EDIT_SCHEMA_REF.to_owned(),
            doc_ref: LIGHT_REMOTE_EDIT_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("light-remote-edit packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Light Remote Edit Surfaces (narrow scope, stale-state honesty)\n\n");
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
            self.edit_degradations.len()
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
                "  - Edit intent: [{}] {}\n",
                surface.edit_intent.intent_kind.as_str(),
                surface.edit_intent.note,
            ));
            out.push_str(&format!(
                "  - Return path: [{}] {}\n",
                surface.return_path.return_kind.as_str(),
                surface.return_path.label,
            ));
            out.push_str(&format!(
                "  - Authority: granted `{}` / effective `{}`\n",
                surface.authority.granted.as_str(),
                surface.authority.effective.as_str(),
            ));
            out.push_str(&format!(
                "  - Base state: [{}] disclosed {}\n",
                surface.stale_state.base_state_kind.as_str(),
                surface.stale_state.disclosed,
            ));
            out.push_str(&format!(
                "  - Apply: {} | Captured/live: {} | Cited: {}\n",
                surface.apply_posture.as_str(),
                surface.captured_vs_live.as_str(),
                surface.cited,
            ));
        }
        if !self.edit_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.edit_degradations {
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

/// Support-export envelope for the light-remote-edit packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LightRemoteEditSupportExport {
    /// Record kind; must equal [`LIGHT_REMOTE_EDIT_SUPPORT_EXPORT_RECORD_KIND`].
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
    /// The wrapped light-remote-edit packet.
    pub packet: LightRemoteEditSurfacesPacket,
}

/// Errors emitted when reading the checked-in light-remote-edit support export.
#[derive(Debug)]
pub enum LightRemoteEditArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: LightRemoteEditPromotionState,
        /// Promotion state computed by re-materialization.
        computed: LightRemoteEditPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<LightRemoteEditValidationFinding>),
}

impl fmt::Display for LightRemoteEditArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "light-remote-edit export parse failed: {error}")
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "light-remote-edit promotion drift: recorded {} but computed {}",
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
                    "light-remote-edit export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for LightRemoteEditArtifactError {}

/// Reads and re-validates the checked-in stable light-remote-edit support export.
pub fn current_stable_light_remote_edit_export(
) -> Result<LightRemoteEditSupportExport, LightRemoteEditArtifactError> {
    let export: LightRemoteEditSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/add_browser_lite_light_remote_edit_surfaces_with_narrow_scope_stale_state_honesty_and_no_hidden_authority_expa/support_export.json"
    )))
    .map_err(LightRemoteEditArtifactError::SupportExport)?;

    let recomputed = LightRemoteEditSurfacesPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(LightRemoteEditArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(LightRemoteEditArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(
    packet: &LightRemoteEditSurfacesPacket,
) -> LightRemoteEditSurfacesPacketInput {
    LightRemoteEditSurfacesPacketInput {
        packet_id: packet.packet_id.clone(),
        session_label: packet.session_label.clone(),
        session_digest_ref: packet.session_digest_ref.clone(),
        surfaces: packet.surfaces.clone(),
        export: packet.export.clone(),
        edit_degradations: packet.edit_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<LightRemoteEditValidationFinding>,
    kind: LightRemoteEditFindingKind,
    summary: impl Into<String>,
) {
    findings.push(LightRemoteEditValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &LightRemoteEditSurfacesPacketInput,
    findings: &mut Vec<LightRemoteEditValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.session_label.trim().is_empty()
        || input.session_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_surfaces(
    input: &LightRemoteEditSurfacesPacketInput,
    findings: &mut Vec<LightRemoteEditValidationFinding>,
) {
    if input.surfaces.is_empty() {
        push_finding(
            findings,
            LightRemoteEditFindingKind::SurfacesEmpty,
            "the light-remote-edit set must carry at least one surface",
        );
        return;
    }

    let present_scopes: BTreeSet<LightRemoteEditScope> =
        input.surfaces.iter().map(|surface| surface.scope).collect();
    for required in LightRemoteEditScope::REQUIRED {
        if !present_scopes.contains(&required) {
            push_finding(
                findings,
                LightRemoteEditFindingKind::RequiredScopeMissing,
                format!("required scope `{}` is missing", required.as_str()),
            );
        }
    }

    let mut seen_surface_ids: BTreeSet<&str> = BTreeSet::new();
    for surface in &input.surfaces {
        if !seen_surface_ids.insert(surface.surface_id.as_str()) {
            push_finding(
                findings,
                LightRemoteEditFindingKind::DuplicateSurfaceId,
                format!("duplicate surface id `{}`", surface.surface_id),
            );
        }
        check_one_surface(surface, findings);
    }
}

fn check_one_surface(
    surface: &LightRemoteEditSurface,
    findings: &mut Vec<LightRemoteEditValidationFinding>,
) {
    if !surface.scope.is_within_qualified_scope() {
        push_finding(
            findings,
            LightRemoteEditFindingKind::SurfaceScopeOutOfBounds,
            format!(
                "surface `{}` declares out-of-scope `{}`; only doc-comment/single-file-text/config-value/review-reply are qualified",
                surface.surface_id,
                surface.scope.as_str()
            ),
        );
    }
    if surface.title.trim().is_empty() || surface.headline.trim().is_empty() {
        push_finding(
            findings,
            LightRemoteEditFindingKind::SurfaceTitleOrHeadlineMissing,
            format!(
                "surface `{}` is missing a title or headline",
                surface.surface_id
            ),
        );
    }
    if surface.edit_intent.note.trim().is_empty() {
        push_finding(
            findings,
            LightRemoteEditFindingKind::EditIntentMissing,
            format!("surface `{}` is missing an edit intent", surface.surface_id),
        );
    }
    if surface.return_path.return_ref.trim().is_empty()
        || surface.return_path.label.trim().is_empty()
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::ReturnPathMissing,
            format!(
                "surface `{}` must keep a return path (return-path safety)",
                surface.surface_id
            ),
        );
    }
    if surface.trust_disclosure_note.trim().is_empty() {
        push_finding(
            findings,
            LightRemoteEditFindingKind::TrustClassDisclosureMissing,
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
            LightRemoteEditFindingKind::OpenRawOpenSourceEscapeMissing,
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
            LightRemoteEditFindingKind::SurfaceNotCited,
            format!(
                "surface `{}` is `{}` but is not cited",
                surface.surface_id,
                surface.trust_class.as_str()
            ),
        );
    }
    // An untrusted destination may never be presented at high confidence.
    if !surface.trust_class.may_be_authoritative()
        && surface.chips.confidence == EditConfidence::High
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::TrustClassDisclosureCollapsed,
            format!(
                "surface `{}` is `{}` but presented as high confidence",
                surface.surface_id,
                surface.trust_class.as_str()
            ),
        );
    }
    // An apply blocked by policy may not present as available.
    if surface.apply_posture == ApplyPosture::ApplyBlockedByPolicy
        && surface.captured_vs_live == CapturedVsLive::Live
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::BlockedApplyPresentedAvailable,
            format!(
                "surface `{}` is apply-blocked by policy but presented as a live apply",
                surface.surface_id
            ),
        );
    }
    // A non-current version may not be presented as a confident live match.
    if !surface.chips.version_match.is_confident_current()
        && surface.chips.confidence == EditConfidence::High
        && surface.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::VersionTruthCollapsed,
            format!(
                "surface `{}` presents version `{}` as a confident live match",
                surface.surface_id,
                surface.chips.version_match.as_str()
            ),
        );
    }

    // Stale-state honesty: a stale or unknown base must be disclosed.
    if surface.stale_state.base_state_kind.requires_disclosure()
        && (!surface.stale_state.disclosed || surface.stale_state.note.trim().is_empty())
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::StaleStateNotDisclosed,
            format!(
                "surface `{}` was prepared against a `{}` base but does not disclose it",
                surface.surface_id,
                surface.stale_state.base_state_kind.as_str()
            ),
        );
    }
    // Stale-state honesty: a stale or unknown base may not be presented confidently.
    if !surface.stale_state.base_state_kind.may_be_confident()
        && surface.chips.confidence == EditConfidence::High
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::StaleStatePresentedConfident,
            format!(
                "surface `{}` presents a `{}` base at high confidence",
                surface.surface_id,
                surface.stale_state.base_state_kind.as_str()
            ),
        );
    }

    // No hidden authority expansion: effective may not exceed granted.
    if surface.authority.is_expansion() {
        push_finding(
            findings,
            LightRemoteEditFindingKind::AuthorityExpansionDetected,
            format!(
                "surface `{}` exercises `{}` authority but was granted only `{}`",
                surface.surface_id,
                surface.authority.effective.as_str(),
                surface.authority.granted.as_str()
            ),
        );
    }
    // No hidden authority expansion: effective may not exceed the scope ceiling.
    if surface.authority.effective > surface.scope.max_authority() {
        push_finding(
            findings,
            LightRemoteEditFindingKind::ScopeAuthorityMismatch,
            format!(
                "surface `{}` exercises `{}` authority beyond what scope `{}` permits (`{}`)",
                surface.surface_id,
                surface.authority.effective.as_str(),
                surface.scope.as_str(),
                surface.scope.max_authority().as_str()
            ),
        );
    }
}

fn check_export(
    input: &LightRemoteEditSurfacesPacketInput,
    findings: &mut Vec<LightRemoteEditValidationFinding>,
) {
    let export = &input.export;
    if !export.preserves_all() {
        push_finding(
            findings,
            LightRemoteEditFindingKind::ExportDropsPreservation,
            "the export must preserve scope, edit intent, return path, trust class, source class, confidence, authority, stale state, and escapes",
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
                LightRemoteEditFindingKind::ExportRowOrphan,
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
                LightRemoteEditFindingKind::ExportCoverageMissing,
                format!("surface `{}` has no export row", surface.surface_id),
            );
        }
    }
}

fn check_export_row(
    surface: &LightRemoteEditSurface,
    row: &LightRemoteEditExportRow,
    findings: &mut Vec<LightRemoteEditValidationFinding>,
) {
    if surface.scope != row.scope {
        push_finding(
            findings,
            LightRemoteEditFindingKind::ExportScopeMismatch,
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
            LightRemoteEditFindingKind::ExportTrustClassMismatch,
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
            LightRemoteEditFindingKind::ExportSourceClassMismatch,
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
            LightRemoteEditFindingKind::ExportConfidenceMismatch,
            format!(
                "export for `{}` records confidence `{}` but the surface chip is `{}`",
                row.surface_id_ref,
                row.confidence.as_str(),
                surface.chips.confidence.as_str()
            ),
        );
    }
    if surface.authority.granted != row.granted_authority
        || surface.authority.effective != row.effective_authority
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::ExportAuthorityMismatch,
            format!(
                "export for `{}` records authority granted `{}`/effective `{}` but the surface is granted `{}`/effective `{}`",
                row.surface_id_ref,
                row.granted_authority.as_str(),
                row.effective_authority.as_str(),
                surface.authority.granted.as_str(),
                surface.authority.effective.as_str()
            ),
        );
    }
    if surface.stale_state.base_state_kind != row.base_state_kind
        || surface.stale_state.disclosed != row.stale_disclosed
    {
        push_finding(
            findings,
            LightRemoteEditFindingKind::ExportStaleStateMismatch,
            format!(
                "export for `{}` records base `{}`/disclosed `{}` but the surface is base `{}`/disclosed `{}`",
                row.surface_id_ref,
                row.base_state_kind.as_str(),
                row.stale_disclosed,
                surface.stale_state.base_state_kind.as_str(),
                surface.stale_state.disclosed
            ),
        );
    }
    // The surface always keeps a return path, so the export row must mark it.
    if !row.has_return_path {
        push_finding(
            findings,
            LightRemoteEditFindingKind::ExportReturnPathMismatch,
            format!(
                "export for `{}` drops the return-path-present flag",
                row.surface_id_ref
            ),
        );
    }
}

fn check_degradations(
    input: &LightRemoteEditSurfacesPacketInput,
    findings: &mut Vec<LightRemoteEditValidationFinding>,
) {
    let surface_ids: BTreeSet<&str> = input
        .surfaces
        .iter()
        .map(|surface| surface.surface_id.as_str())
        .collect();

    for degradation in &input.edit_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                LightRemoteEditFindingKind::DegradationIncomplete,
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
                    LightRemoteEditFindingKind::DegradationOrphan,
                    format!("degradation references unknown surface `{}`", surface_id),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &LightRemoteEditSurfacesPacketInput,
    findings: &mut Vec<LightRemoteEditValidationFinding>,
) {
    let present: BTreeSet<LightRemoteEditConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                LightRemoteEditFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                LightRemoteEditFindingKind::ConsumerProjectionPacketIdMismatch,
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
                LightRemoteEditFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &LightRemoteEditSurfacesPacketInput,
    findings: &mut Vec<LightRemoteEditValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("light-remote-edit input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            LightRemoteEditFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw URLs, raw edit diffs, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across both the
/// validation findings and the attached degradations.
///
/// A blocking finding (scope, trust, return-path, stale-state, authority,
/// citation, or boundary violation) blocks the Stable claim; an otherwise-clean
/// set that carries a narrowing degradation narrows below Stable rather than
/// hiding the surfaces.
fn promotion_state(
    findings: &[LightRemoteEditValidationFinding],
    degradations: &[LightRemoteEditDegradation],
) -> LightRemoteEditPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == LightRemoteEditFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == LightRemoteEditFindingSeverity::Blocking);
    if any_blocking {
        return LightRemoteEditPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == LightRemoteEditFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == LightRemoteEditFindingSeverity::Narrowing);
    if any_narrowing {
        LightRemoteEditPromotionState::NarrowedBelowStable
    } else {
        LightRemoteEditPromotionState::Stable
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
                || lower.contains("raw_diff:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable light-remote-edit input used by the producer, tests, and fixtures.
pub fn seeded_stable_light_remote_edit_input() -> LightRemoteEditSurfacesPacketInput {
    let packet_id = "packet:m5:light_remote_edit:retry_backoff_edits".to_owned();
    LightRemoteEditSurfacesPacketInput {
        packet_id: packet_id.clone(),
        session_label: "light remote edit: tidying the networking retry backoff change".to_owned(),
        session_digest_ref: "sessiondigest:sha256:net-retry-backoff-edits".to_owned(),
        surfaces: vec![
            doc_comment_edit_surface(),
            single_file_text_edit_surface(),
            review_reply_surface(),
        ],
        export: seeded_export(),
        edit_degradations: vec![LightRemoteEditDegradation {
            degradation_class: LightRemoteEditDegradationClass::MirrorOfflineSnapshot,
            severity: LightRemoteEditFindingSeverity::Advisory,
            summary: "the suggestion mirror was last synced two days ago; the single-file edit base is served from the cached snapshot".to_owned(),
            surface_id_ref: Some("surface:single_file_text_edit:retry_log_message".to_owned()),
            evidence_ref: Some("evidence:light-remote-edit:mirror-sync-state".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    }
}

fn doc_comment_edit_surface() -> LightRemoteEditSurface {
    LightRemoteEditSurface {
        surface_id: "surface:doc_comment_edit:retry_doc_comment".to_owned(),
        scope: LightRemoteEditScope::DocCommentEdit,
        subject_ref: "docnode:project:crates/aureline-net/src/retry.rs#doc".to_owned(),
        title: "Doc-comment edit: retry backoff comment".to_owned(),
        headline: "a scoped doc-comment edit fixing a typo in the local retry backoff doc comment"
            .to_owned(),
        chips: LightRemoteEditChipSet {
            source_class: EditSourceClass::ProjectDocs,
            version_match: EditVersionMatch::ExactBuildMatch,
            freshness: EditFreshness::AuthoritativeLive,
            locality: EditLocality::Local,
            confidence: EditConfidence::High,
        },
        trust_class: EditTrustClass::FirstPartyWorkspace,
        trust_disclosure_note:
            "first-party workspace doc; the light edit stays local and authoritative".to_owned(),
        edit_intent: EditIntent {
            intent_kind: EditIntentKind::FixDocTypo,
            note: "the typo fix opens a scoped editor over the local doc comment only".to_owned(),
        },
        return_path: ReturnPath {
            return_kind: ReturnPathKind::BackToWorkspace,
            return_ref: "return:workspace:crates/aureline-net/src/retry.rs".to_owned(),
            label: "Back to the workspace".to_owned(),
        },
        authority: AuthorityGrant {
            granted: AuthorityScope::SingleFileWrite,
            effective: AuthorityScope::SingleFieldWrite,
        },
        stale_state: StaleStateDisclosure {
            base_state_kind: BaseStateKind::LiveHead,
            disclosed: true,
            note: "prepared against the current workspace head".to_owned(),
        },
        apply_posture: ApplyPosture::LocalDirectApply,
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:docnode:project:crates/aureline-net/src/retry.rs".to_owned()),
        open_raw_escape_ref: "open-raw:docnode:project:crates/aureline-net/src/retry.rs#doc"
            .to_owned(),
        open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
    }
}

fn single_file_text_edit_surface() -> LightRemoteEditSurface {
    LightRemoteEditSurface {
        surface_id: "surface:single_file_text_edit:retry_log_message".to_owned(),
        scope: LightRemoteEditScope::SingleFileTextEdit,
        subject_ref: "docnode:project:crates/aureline-net/src/retry.rs#log".to_owned(),
        title: "Single-file edit: retry log message".to_owned(),
        headline: "a scoped single-file text edit applying a mirror-backed suggestion to one log line".to_owned(),
        chips: LightRemoteEditChipSet {
            source_class: EditSourceClass::MirroredOfficialDocs,
            version_match: EditVersionMatch::CompatibleMinorDrift,
            freshness: EditFreshness::WarmCached,
            locality: EditLocality::RemoteHelper,
            confidence: EditConfidence::Medium,
        },
        trust_class: EditTrustClass::LiveProviderEditSurface,
        trust_disclosure_note: "remote provider edit surface; not verified at materialization time, held to medium and disclosed as a warm snapshot".to_owned(),
        edit_intent: EditIntent {
            intent_kind: EditIntentKind::ApplyReviewSuggestion,
            note: "applies the reviewer's wording suggestion to a single log message".to_owned(),
        },
        return_path: ReturnPath {
            return_kind: ReturnPathKind::BackToInlinePeek,
            return_ref: "return:peek:symbol:aureline-net::retry::retry_with_backoff".to_owned(),
            label: "Back to the retry_with_backoff peek".to_owned(),
        },
        authority: AuthorityGrant {
            granted: AuthorityScope::SingleFileWrite,
            effective: AuthorityScope::SingleFileWrite,
        },
        stale_state: StaleStateDisclosure {
            base_state_kind: BaseStateKind::WarmSnapshot,
            disclosed: true,
            note: "prepared against a warm snapshot synced two days ago; disclosed to the reader".to_owned(),
        },
        apply_posture: ApplyPosture::RemoteApplyAvailable,
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:suggestion:mirror:retry-log-wording".to_owned()),
        open_raw_escape_ref: "open-raw:docnode:project:crates/aureline-net/src/retry.rs#log"
            .to_owned(),
        open_source_escape_ref: "open-source:mirror:retry-log-wording".to_owned(),
    }
}

fn review_reply_surface() -> LightRemoteEditSurface {
    LightRemoteEditSurface {
        surface_id: "surface:review_reply:retry_backoff_thread".to_owned(),
        scope: LightRemoteEditScope::ReviewReply,
        subject_ref: "reviewthread:host:retry-backoff/thread-7#reply".to_owned(),
        title: "Review reply: retry/backoff thread".to_owned(),
        headline: "a scoped review reply to the hosted review thread for the backoff change"
            .to_owned(),
        chips: LightRemoteEditChipSet {
            source_class: EditSourceClass::ReviewHost,
            version_match: EditVersionMatch::ExactBuildMatch,
            freshness: EditFreshness::AuthoritativeLive,
            locality: EditLocality::Managed,
            confidence: EditConfidence::Medium,
        },
        trust_class: EditTrustClass::LiveProviderEditSurface,
        trust_disclosure_note: "live handoff to the hosted review provider; not verified at materialization time, held to medium".to_owned(),
        edit_intent: EditIntent {
            intent_kind: EditIntentKind::ReplyToReviewComment,
            note: "posts a short reply to the reviewer's comment on the backoff change".to_owned(),
        },
        return_path: ReturnPath {
            return_kind: ReturnPathKind::BackToReviewPanel,
            return_ref: "return:review-panel:retry-backoff".to_owned(),
            label: "Back to the review panel".to_owned(),
        },
        authority: AuthorityGrant {
            granted: AuthorityScope::SingleFieldWrite,
            effective: AuthorityScope::SingleFieldWrite,
        },
        stale_state: StaleStateDisclosure {
            base_state_kind: BaseStateKind::LiveHead,
            disclosed: true,
            note: "the reply targets the live review thread head".to_owned(),
        },
        apply_posture: ApplyPosture::RemoteApplyAvailable,
        captured_vs_live: CapturedVsLive::Live,
        cited: true,
        citation_ref: Some("cite:reviewthread:host:retry-backoff/thread-7".to_owned()),
        open_raw_escape_ref: "open-raw:reviewthread:host:retry-backoff/thread-7".to_owned(),
        open_source_escape_ref: "open-source:review-host:retry-backoff/thread-7".to_owned(),
    }
}

fn seeded_export() -> LightRemoteEditExport {
    LightRemoteEditExport {
        scope: LightRemoteEditExportScope::AllSurfaces,
        preserves_scope: true,
        preserves_edit_intent: true,
        preserves_return_path: true,
        preserves_trust_class: true,
        preserves_source_class: true,
        preserves_confidence: true,
        preserves_authority: true,
        preserves_stale_state: true,
        preserves_open_raw_open_source_escape: true,
        rows: vec![
            LightRemoteEditExportRow {
                surface_id_ref: "surface:doc_comment_edit:retry_doc_comment".to_owned(),
                scope: LightRemoteEditScope::DocCommentEdit,
                trust_class: EditTrustClass::FirstPartyWorkspace,
                source_class: EditSourceClass::ProjectDocs,
                confidence: EditConfidence::High,
                apply_posture: ApplyPosture::LocalDirectApply,
                granted_authority: AuthorityScope::SingleFileWrite,
                effective_authority: AuthorityScope::SingleFieldWrite,
                base_state_kind: BaseStateKind::LiveHead,
                stale_disclosed: true,
                has_return_path: true,
                cited: true,
                open_raw_escape_ref:
                    "open-raw:docnode:project:crates/aureline-net/src/retry.rs#doc".to_owned(),
                open_source_escape_ref: "open-source:repo:crates/aureline-net/src/retry.rs"
                    .to_owned(),
            },
            LightRemoteEditExportRow {
                surface_id_ref: "surface:single_file_text_edit:retry_log_message".to_owned(),
                scope: LightRemoteEditScope::SingleFileTextEdit,
                trust_class: EditTrustClass::LiveProviderEditSurface,
                source_class: EditSourceClass::MirroredOfficialDocs,
                confidence: EditConfidence::Medium,
                apply_posture: ApplyPosture::RemoteApplyAvailable,
                granted_authority: AuthorityScope::SingleFileWrite,
                effective_authority: AuthorityScope::SingleFileWrite,
                base_state_kind: BaseStateKind::WarmSnapshot,
                stale_disclosed: true,
                has_return_path: true,
                cited: true,
                open_raw_escape_ref:
                    "open-raw:docnode:project:crates/aureline-net/src/retry.rs#log".to_owned(),
                open_source_escape_ref: "open-source:mirror:retry-log-wording".to_owned(),
            },
            LightRemoteEditExportRow {
                surface_id_ref: "surface:review_reply:retry_backoff_thread".to_owned(),
                scope: LightRemoteEditScope::ReviewReply,
                trust_class: EditTrustClass::LiveProviderEditSurface,
                source_class: EditSourceClass::ReviewHost,
                confidence: EditConfidence::Medium,
                apply_posture: ApplyPosture::RemoteApplyAvailable,
                granted_authority: AuthorityScope::SingleFieldWrite,
                effective_authority: AuthorityScope::SingleFieldWrite,
                base_state_kind: BaseStateKind::LiveHead,
                stale_disclosed: true,
                has_return_path: true,
                cited: true,
                open_raw_escape_ref: "open-raw:reviewthread:host:retry-backoff/thread-7".to_owned(),
                open_source_escape_ref: "open-source:review-host:retry-backoff/thread-7".to_owned(),
            },
        ],
    }
}

fn required_projections(packet_id: &str) -> Vec<LightRemoteEditConsumerProjection> {
    [
        LightRemoteEditConsumerSurface::DocsBrowserShell,
        LightRemoteEditConsumerSurface::ReviewSurface,
        LightRemoteEditConsumerSurface::LightEditSurface,
        LightRemoteEditConsumerSurface::PeekOverlay,
        LightRemoteEditConsumerSurface::AiContextInspector,
        LightRemoteEditConsumerSurface::CliHeadless,
        LightRemoteEditConsumerSurface::SupportExport,
        LightRemoteEditConsumerSurface::Diagnostics,
        LightRemoteEditConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| LightRemoteEditConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_chips: true,
        preserves_scopes: true,
        preserves_trust_classes: true,
        preserves_edit_intents: true,
        preserves_return_paths: true,
        preserves_authority: true,
        preserves_stale_state: true,
        preserves_open_raw_open_source_escape: true,
    })
    .collect()
}
