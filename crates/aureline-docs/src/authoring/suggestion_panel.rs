//! Docs suggestion panels: target identity, trigger disclosure, diff-first
//! proposals, confidence/freshness labels, and apply/open-evidence/dismiss
//! parity.
//!
//! This module owns the runtime truth packet behind the docs suggestion panel —
//! the diff-first surface that proposes prose edits to README, changelog, help,
//! and tutorial docs and ties each proposal back to the code, schema, or release
//! change that raised it. Each [`PanelSuggestion`] names a concrete
//! [`PanelSuggestionTarget`] (the target file and, when applicable, the section
//! anchor), a concrete [`PanelTriggerSource`] (a code diff, a symbol rename, an
//! API contract change, a failing example, a broken link, or a release-metadata
//! change) with a non-empty trigger detail and triggering-evidence ref, a
//! confidence/freshness/version/locality chip set, one evidence-provenance
//! disclosure ([`PanelEvidenceProvenance`]), a diff-based [`PanelProposal`]
//! (never a prose-only "recommended edit" card), the full [`PanelActionSet`]
//! (Apply, Open evidence, Open source, Dismiss, Save for later), and a durable
//! [`PanelDisposition`] (so applying or dismissing stays attributable,
//! previewable, and reopenable).
//!
//! Five invariants make a docs suggestion honest:
//!
//! - **Concrete target.** A suggestion must point at a concrete file (and, when
//!   it edits a section, a concrete section anchor) — never a generic
//!   recommendation blob with no target.
//! - **Concrete trigger source.** A suggestion must name one trigger source with
//!   a non-empty detail and a triggering-evidence ref — never an unattributed
//!   hint.
//! - **Diff-first proposals.** Every proposal must be diff-based with a summary
//!   and a previewable ref; a prose-only card (or a zero-hunk diff) is rejected
//!   so docs maintenance does not bypass the shared review/diff model just
//!   because the target is prose.
//! - **Action parity.** Every suggestion keeps Open evidence, Open source,
//!   Dismiss, and Save for later available, and an Apply posture; an unverified
//!   (imported / mirrored / local-only / stale / derived) evidence source may
//!   surface a preview but never a one-click apply.
//! - **Durable, honest disposition.** An applied, dismissed, or saved suggestion
//!   carries an attribution ref and a durable history ref and stays previewable
//!   and reopenable; and an unverified evidence source may never be presented as
//!   high-confidence live-authoritative repo truth.
//!
//! The [`DocsSuggestionPanelExport`] is the projection support, AI evidence, and
//! diagnostics surfaces ingest: one [`PanelSuggestionExportRow`] per suggestion
//! preserving target kind, trigger source, confidence, freshness, apply posture,
//! provenance, disposition state, action parity, and citation state.
//!
//! [`DocsSuggestionPanelPacket::materialize`] computes the validation findings
//! and the promotion state (`stable`, `narrowed_below_stable`, or
//! `blocks_stable`) from the input — folding the packet-level degradation
//! severities into the promotion decision — so a clean panel stays Stable, a
//! degraded-but-honest panel narrows below Stable, and a panel with a missing
//! target, an unattributed trigger, a prose-only proposal, incomplete action
//! parity, an unverified one-click apply, a collapsed provenance/version truth,
//! or a non-reopenable disposition blocks before it reaches a consumer surface.
//! The packet is an inspectable, serde-serializable truth packet: it carries no
//! raw document bodies, no raw source files, no raw URLs, no diff bodies, no raw
//! provider payloads, and no credentials — only metadata, target refs, trigger
//! disclosure, chip truth, provenance disclosure, diff summaries, action parity,
//! disposition history, finding summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/docs-suggestion-packet.schema.json`](../../../../schemas/docs/docs-suggestion-packet.schema.json).
//! The contract doc is
//! [`docs/m5/docs-suggestion-panel.md`](../../../../docs/m5/docs-suggestion-panel.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/docs-suggestion-triggers/`](../../../../fixtures/docs/m5/docs-suggestion-triggers/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DocsSuggestionPanelPacket`].
pub const DOCS_SUGGESTION_PANEL_RECORD_KIND: &str = "docs_suggestion_panel";

/// Record-kind tag carried by the support-export wrapper.
pub const DOCS_SUGGESTION_PANEL_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_suggestion_panel_support_export";

/// Schema version for docs-suggestion-panel records.
pub const DOCS_SUGGESTION_PANEL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_SUGGESTION_PANEL_SCHEMA_REF: &str =
    "schemas/docs/docs-suggestion-packet.schema.json";

/// Repo-relative path of the docs-suggestion-panel contract doc.
pub const DOCS_SUGGESTION_PANEL_DOC_REF: &str = "docs/m5/docs-suggestion-panel.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_SUGGESTION_PANEL_FIXTURE_DIR: &str = "fixtures/docs/m5/docs-suggestion-triggers";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_SUGGESTION_PANEL_ARTIFACT_REF: &str =
    "artifacts/docs/m5/docs-suggestion-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_SUGGESTION_PANEL_SUMMARY_REF: &str = "artifacts/docs/m5/docs-suggestion-proof.md";

/// Kind of docs target a suggestion edits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelTargetKind {
    /// A README document.
    Readme,
    /// A changelog / release-notes document.
    Changelog,
    /// An in-product help document.
    Help,
    /// A tutorial / walkthrough document.
    Tutorial,
    /// A long-form guide document.
    Guide,
    /// API-reference prose.
    ApiReference,
}

impl PanelTargetKind {
    /// The target kinds a panel must cover across the M5 docs surfaces.
    pub const REQUIRED: [Self; 4] = [Self::Readme, Self::Changelog, Self::Help, Self::Tutorial];

    /// Stable token recorded in the target.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Readme => "readme",
            Self::Changelog => "changelog",
            Self::Help => "help",
            Self::Tutorial => "tutorial",
            Self::Guide => "guide",
            Self::ApiReference => "api_reference",
        }
    }
}

/// The change that raised a suggestion, projected as the trigger-source chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelTriggerSource {
    /// A code diff touched the documented behaviour.
    CodeDiff,
    /// A symbol was renamed.
    SymbolRename,
    /// An API contract changed.
    ApiContractChange,
    /// A documented example failed to compile or run.
    FailingExample,
    /// A documentation link broke.
    BrokenLink,
    /// Release metadata changed (version, channel, support window).
    ReleaseMetadataChange,
    /// A manual authoring request raised the suggestion.
    ManualAuthoring,
}

impl PanelTriggerSource {
    /// Stable token recorded in the trigger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeDiff => "code_diff",
            Self::SymbolRename => "symbol_rename",
            Self::ApiContractChange => "api_contract_change",
            Self::FailingExample => "failing_example",
            Self::BrokenLink => "broken_link",
            Self::ReleaseMetadataChange => "release_metadata_change",
            Self::ManualAuthoring => "manual_authoring",
        }
    }
}

/// Whether and how a suggestion's proposed edit may be applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelApplyPosture {
    /// The proposed diff is shown but a preview is required before applying.
    PreviewRequired,
    /// A one-click apply action is available and explicit.
    ApplyAvailable,
    /// Applying is blocked by policy.
    ApplyBlockedByPolicy,
    /// Applying is unavailable and disclosed as such.
    ApplyUnavailableDisclosed,
}

impl PanelApplyPosture {
    /// Stable token recorded in the action set.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewRequired => "preview_required",
            Self::ApplyAvailable => "apply_available",
            Self::ApplyBlockedByPolicy => "apply_blocked_by_policy",
            Self::ApplyUnavailableDisclosed => "apply_unavailable_disclosed",
        }
    }

    /// Whether this posture presents a one-click apply action.
    pub const fn offers_one_click_apply(self) -> bool {
        matches!(self, Self::ApplyAvailable)
    }
}

/// The kind of edit a proposal carries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelProposalKind {
    /// A concrete diff of one or more hunks against the target.
    DiffHunks,
    /// A new section added as a diff.
    NewSectionDiff,
    /// A link repointed as a diff.
    LinkRepointDiff,
    /// An example replaced as a diff.
    ExampleReplaceDiff,
    /// A prose-only "recommended edit" card with no diff — rejected.
    ProseOnlyCard,
}

impl PanelProposalKind {
    /// Stable token recorded in the proposal.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffHunks => "diff_hunks",
            Self::NewSectionDiff => "new_section_diff",
            Self::LinkRepointDiff => "link_repoint_diff",
            Self::ExampleReplaceDiff => "example_replace_diff",
            Self::ProseOnlyCard => "prose_only_card",
        }
    }

    /// Whether this proposal is a reviewable diff rather than a prose-only card.
    pub const fn is_diff_based(self) -> bool {
        !matches!(self, Self::ProseOnlyCard)
    }
}

/// Evidence provenance for a suggestion, kept visible so a useful hint is never
/// mistaken for authoritative repo truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelEvidenceProvenance {
    /// First-party evidence verified against the in-repo build.
    FirstPartyVerified,
    /// Local-only evidence that has not been verified.
    LocalOnlyUnverified,
    /// Evidence imported from a signed pack or extension.
    Imported,
    /// Evidence served from a mirror.
    Mirrored,
    /// Evidence known to be stale.
    Stale,
    /// Derived / inferred evidence only.
    DerivedHeuristic,
}

impl PanelEvidenceProvenance {
    /// Stable token recorded in the suggestion.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstPartyVerified => "first_party_verified",
            Self::LocalOnlyUnverified => "local_only_unverified",
            Self::Imported => "imported",
            Self::Mirrored => "mirrored",
            Self::Stale => "stale",
            Self::DerivedHeuristic => "derived_heuristic",
        }
    }

    /// Whether this provenance may back a one-click apply or an authoritative
    /// high-confidence live claim. Only first-party verified evidence may.
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::FirstPartyVerified)
    }

    /// Whether a suggestion of this provenance must stay cited.
    pub const fn needs_citation(self) -> bool {
        !matches!(self, Self::FirstPartyVerified)
    }
}

/// Version-match state for a suggestion, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelVersionMatch {
    /// Matches the active build/workspace revision exactly.
    ExactBuildMatch,
    /// Within an accepted compatible drift window.
    CompatibleMinorDrift,
    /// Drifted incompatibly from the active target.
    IncompatibleDriftDetected,
    /// Pre-release; verification has not completed.
    PreReleaseUnverified,
    /// The target build/workspace revision could not be verified.
    UnknownTargetBuild,
}

impl PanelVersionMatch {
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

/// Freshness state for a suggestion, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelFreshness {
    /// Live and authoritative at materialization time.
    AuthoritativeLive,
    /// Cached within its freshness window.
    WarmCached,
    /// Cached and usable only with degraded disclosure.
    DegradedCached,
    /// Stale and must not claim current authority.
    Stale,
    /// Freshness could not be verified.
    Unverified,
    /// A refresh is pending; the source has not yet re-synced.
    RefreshPending,
}

impl PanelFreshness {
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

/// Locality / install posture for a suggestion, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through an imported pack.
    ImportedPack,
    /// Resolved through a mirrored pack.
    MirroredPack,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl PanelLocality {
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

/// Confidence label for a suggestion, projected as the confidence chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelConfidence {
    /// High confidence.
    High,
    /// Medium confidence.
    Medium,
    /// Low confidence.
    Low,
    /// Heuristic only; not a verified claim.
    Heuristic,
}

impl PanelConfidence {
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

/// Disposition state of a suggestion in durable history.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelDispositionState {
    /// The suggestion is open and awaiting a decision.
    Pending,
    /// The suggestion's diff was applied.
    Applied,
    /// The suggestion was dismissed.
    Dismissed,
    /// The suggestion was saved for later.
    SavedForLater,
    /// The suggestion was superseded by a newer one.
    Superseded,
}

impl PanelDispositionState {
    /// Stable token recorded in the disposition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Applied => "applied",
            Self::Dismissed => "dismissed",
            Self::SavedForLater => "saved_for_later",
            Self::Superseded => "superseded",
        }
    }

    /// Whether this state is the result of a recorded action (apply / dismiss /
    /// save / supersede) rather than the open default. A resolved disposition
    /// must be attributable, previewable, and reopenable.
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::Pending)
    }
}

/// Severity of a degradation or validation finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelFindingSeverity {
    /// Blocks a Stable claim; the panel must block.
    Blocking,
    /// Narrows below Stable but the panel stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl PanelFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the docs-suggestion-panel packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelConsumerSurface {
    /// The docs suggestion panel itself.
    DocsSuggestionPanel,
    /// The docs authoring surface.
    DocsAuthoringSurface,
    /// The shared docs review panel.
    DocsReviewPanel,
    /// The docs browser shell.
    DocsBrowserShell,
    /// The release center (changelog suggestions).
    ReleaseCenter,
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

impl PanelConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsSuggestionPanel => "docs_suggestion_panel",
            Self::DocsAuthoringSurface => "docs_authoring_surface",
            Self::DocsReviewPanel => "docs_review_panel",
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::ReleaseCenter => "release_center",
            Self::AiContextInspector => "ai_context_inspector",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level docs-suggestion degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelDegradationClass {
    /// A mirror is offline; suggestions are served from the last snapshot.
    MirrorOfflineSnapshot,
    /// The example harness is unavailable, so example triggers could not re-run.
    ExampleHarnessUnavailable,
    /// The link checker is offline, so link triggers could not be re-verified.
    LinkCheckerOffline,
    /// The diff/preview engine is degraded.
    DiffEngineDegraded,
    /// The suggestion engine is degraded.
    SuggestionEngineDegraded,
    /// The panel was rerun at a narrowed scope.
    ScopeNarrowedRerun,
    /// The panel claim was narrowed before publication.
    PanelNarrowed,
    /// The owning source is quarantined.
    QuarantinedSource,
}

impl PanelDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::ExampleHarnessUnavailable => "example_harness_unavailable",
            Self::LinkCheckerOffline => "link_checker_offline",
            Self::DiffEngineDegraded => "diff_engine_degraded",
            Self::SuggestionEngineDegraded => "suggestion_engine_degraded",
            Self::ScopeNarrowedRerun => "scope_narrowed_rerun",
            Self::PanelNarrowed => "panel_narrowed",
            Self::QuarantinedSource => "quarantined_source",
        }
    }
}

/// Scope a docs-suggestion export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelExportScope {
    /// Every suggestion in the packet.
    AllSuggestions,
    /// Pending suggestions only.
    PendingOnly,
    /// Resolved (applied / dismissed / saved) suggestions only.
    ResolvedOnly,
    /// Applied suggestions only.
    AppliedOnly,
}

impl PanelExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllSuggestions => "all_suggestions",
            Self::PendingOnly => "pending_only",
            Self::ResolvedOnly => "resolved_only",
            Self::AppliedOnly => "applied_only",
        }
    }
}

/// Promotion state computed for the docs-suggestion-panel packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelPromotionState {
    /// Panel qualifies for the Stable claim.
    Stable,
    /// Panel narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Panel has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl PanelPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Validation finding kind emitted by [`DocsSuggestionPanelPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PanelFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The suggestion set is empty.
    SuggestionsEmpty,
    /// A suggestion id is duplicated.
    DuplicateSuggestionId,
    /// A required target kind (readme / changelog / help / tutorial) is missing.
    RequiredTargetKindMissing,
    /// A suggestion does not name a concrete target file/section.
    TargetIdentityMissing,
    /// A suggestion does not name a concrete trigger source detail/evidence.
    TriggerSourceDetailMissing,
    /// A suggestion is missing its title or detail.
    TitleOrDetailMissing,
    /// A suggestion is missing its provenance disclosure note.
    ProvenanceDisclosureMissing,
    /// An unverified evidence source is presented as high-confidence live truth.
    ProvenanceTruthCollapsed,
    /// An unverified evidence source is not cited.
    SuggestionNotCited,
    /// A non-current version is presented as a confident live match.
    VersionTruthCollapsed,
    /// A proposal is prose-only (or a zero-hunk diff) rather than diff-based.
    ProposalNotDiffBased,
    /// A proposal is missing its summary or previewable ref.
    ProposalSummaryMissing,
    /// The action parity (open-evidence / open-source / dismiss / save) is incomplete.
    ActionParityIncomplete,
    /// An unverified evidence source offers a one-click apply.
    UnverifiedApplyOffered,
    /// A resolved disposition is missing its attribution or history ref.
    DispositionNotAttributable,
    /// A resolved disposition is not previewable or reopenable.
    DispositionNotReopenable,
    /// The export drops a required preservation flag.
    ExportDropsPreservation,
    /// An export row references a suggestion id absent from the suggestions.
    ExportRowOrphan,
    /// A suggestion has no matching export row.
    ExportCoverageMissing,
    /// An export row's target kind disagrees with the suggestion.
    ExportTargetKindMismatch,
    /// An export row's trigger source disagrees with the suggestion.
    ExportTriggerSourceMismatch,
    /// An export row's confidence disagrees with the suggestion's chip.
    ExportConfidenceMismatch,
    /// An export row's freshness disagrees with the suggestion's chip.
    ExportFreshnessMismatch,
    /// An export row's apply posture disagrees with the suggestion.
    ExportApplyPostureMismatch,
    /// An export row's provenance disagrees with the suggestion.
    ExportProvenanceMismatch,
    /// An export row's disposition state disagrees with the suggestion.
    ExportDispositionMismatch,
    /// An export row's cited flag disagrees with the suggestion.
    ExportCitedMismatch,
    /// A degradation is incomplete (missing summary).
    DegradationIncomplete,
    /// A degradation references a suggestion id absent from the suggestions.
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

impl PanelFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::SuggestionsEmpty => "suggestions_empty",
            Self::DuplicateSuggestionId => "duplicate_suggestion_id",
            Self::RequiredTargetKindMissing => "required_target_kind_missing",
            Self::TargetIdentityMissing => "target_identity_missing",
            Self::TriggerSourceDetailMissing => "trigger_source_detail_missing",
            Self::TitleOrDetailMissing => "title_or_detail_missing",
            Self::ProvenanceDisclosureMissing => "provenance_disclosure_missing",
            Self::ProvenanceTruthCollapsed => "provenance_truth_collapsed",
            Self::SuggestionNotCited => "suggestion_not_cited",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::ProposalNotDiffBased => "proposal_not_diff_based",
            Self::ProposalSummaryMissing => "proposal_summary_missing",
            Self::ActionParityIncomplete => "action_parity_incomplete",
            Self::UnverifiedApplyOffered => "unverified_apply_offered",
            Self::DispositionNotAttributable => "disposition_not_attributable",
            Self::DispositionNotReopenable => "disposition_not_reopenable",
            Self::ExportDropsPreservation => "export_drops_preservation",
            Self::ExportRowOrphan => "export_row_orphan",
            Self::ExportCoverageMissing => "export_coverage_missing",
            Self::ExportTargetKindMismatch => "export_target_kind_mismatch",
            Self::ExportTriggerSourceMismatch => "export_trigger_source_mismatch",
            Self::ExportConfidenceMismatch => "export_confidence_mismatch",
            Self::ExportFreshnessMismatch => "export_freshness_mismatch",
            Self::ExportApplyPostureMismatch => "export_apply_posture_mismatch",
            Self::ExportProvenanceMismatch => "export_provenance_mismatch",
            Self::ExportDispositionMismatch => "export_disposition_mismatch",
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
    /// the Stable claim; narrowing comes only from data-carried degradation
    /// severities so a degraded-but-honest panel narrows rather than blocks.
    pub const fn default_severity(self) -> PanelFindingSeverity {
        PanelFindingSeverity::Blocking
    }
}

/// The concrete target a suggestion edits — names the file and (optionally) the
/// section anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelSuggestionTarget {
    /// The kind of target document.
    pub target_kind: PanelTargetKind,
    /// Repo-relative target file ref (no raw body).
    pub file_ref: String,
    /// Section anchor within the file when the edit is scoped to a section.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_anchor: Option<String>,
    /// Human-readable display path for the target.
    pub display_path: String,
}

impl PanelSuggestionTarget {
    /// Whether the target names a concrete file (and a section anchor when one is
    /// recorded is non-empty).
    pub fn names_concrete_target(&self) -> bool {
        if self.file_ref.trim().is_empty() || self.display_path.trim().is_empty() {
            return false;
        }
        match &self.section_anchor {
            Some(anchor) => !anchor.trim().is_empty(),
            None => true,
        }
    }
}

/// The trigger that raised a suggestion, with its disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelTrigger {
    /// The trigger source class.
    pub source: PanelTriggerSource,
    /// Human-readable trigger detail (no raw bodies).
    pub detail: String,
    /// Ref to the triggering evidence (a diff ref, a contract ref, a link
    /// checker result ref) — no raw body.
    pub evidence_ref: String,
}

impl PanelTrigger {
    /// Whether the trigger names a concrete detail and evidence ref.
    pub fn names_concrete_source(&self) -> bool {
        !self.detail.trim().is_empty() && !self.evidence_ref.trim().is_empty()
    }
}

/// The diff-based proposal a suggestion carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelProposal {
    /// The kind of proposal.
    pub proposal_kind: PanelProposalKind,
    /// Number of diff hunks in the proposal.
    pub hunk_count: u32,
    /// Number of added lines (metadata only).
    pub added_lines: u32,
    /// Number of removed lines (metadata only).
    pub removed_lines: u32,
    /// Human-readable proposal summary (no raw diff body).
    pub summary: String,
    /// Ref to the previewable diff (no raw diff body).
    pub preview_ref: String,
}

impl PanelProposal {
    /// Whether the proposal is a reviewable diff with at least one hunk.
    pub fn is_diff_based(&self) -> bool {
        self.proposal_kind.is_diff_based() && self.hunk_count >= 1
    }
}

/// The action set a suggestion exposes — Apply, Open evidence, Open source,
/// Dismiss, and Save for later.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelActionSet {
    /// Whether and how the proposed edit may be applied.
    pub apply_posture: PanelApplyPosture,
    /// Open-evidence escape ref (open the triggering evidence).
    pub open_evidence_ref: String,
    /// Open-source escape ref (open the underlying source).
    pub open_source_ref: String,
    /// Whether Dismiss is available.
    pub dismiss_available: bool,
    /// Whether Save for later is available.
    pub save_for_later_available: bool,
}

impl PanelActionSet {
    /// Whether every non-apply action is present (open-evidence, open-source,
    /// dismiss, and save-for-later parity).
    pub fn parity_complete(&self) -> bool {
        !self.open_evidence_ref.trim().is_empty()
            && !self.open_source_ref.trim().is_empty()
            && self.dismiss_available
            && self.save_for_later_available
    }
}

/// The durable disposition of a suggestion — its history-backed state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelDisposition {
    /// The disposition state.
    pub state: PanelDispositionState,
    /// Ref to the actor who resolved the suggestion (required when resolved).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributed_to_ref: Option<String>,
    /// Durable history-entry ref (required when resolved).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_ref: Option<String>,
    /// Whether the disposition is previewable.
    pub previewable: bool,
    /// Whether the disposition is reopenable.
    pub reopenable: bool,
    /// Human-readable disposition note (no raw bodies).
    pub note: String,
}

impl PanelDisposition {
    /// Whether a resolved disposition carries its attribution and history refs.
    pub fn is_attributable(&self) -> bool {
        if !self.state.is_resolved() {
            return true;
        }
        let attributed = self
            .attributed_to_ref
            .as_ref()
            .is_some_and(|r| !r.trim().is_empty());
        let history = self
            .history_ref
            .as_ref()
            .is_some_and(|r| !r.trim().is_empty());
        attributed && history
    }

    /// Whether a resolved disposition is previewable and reopenable.
    pub fn is_reopenable(&self) -> bool {
        if !self.state.is_resolved() {
            return true;
        }
        self.previewable && self.reopenable
    }
}

/// The chip set rendered for one suggestion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelChipSet {
    /// Confidence chip (the confidence label).
    pub confidence: PanelConfidence,
    /// Freshness chip.
    pub freshness: PanelFreshness,
    /// Version-match chip.
    pub version_match: PanelVersionMatch,
    /// Locality chip.
    pub locality: PanelLocality,
}

/// One docs suggestion — one bounded diff-first proposal record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelSuggestion {
    /// Stable suggestion id within this packet.
    pub suggestion_id: String,
    /// The concrete target file/section.
    pub target: PanelSuggestionTarget,
    /// The trigger that raised the suggestion.
    pub trigger: PanelTrigger,
    /// Human-readable title.
    pub title: String,
    /// Human-readable detail / summary (no raw bodies).
    pub detail: String,
    /// Confidence/freshness/version/locality chips.
    pub chips: PanelChipSet,
    /// The evidence-provenance disclosure for the suggestion.
    pub provenance: PanelEvidenceProvenance,
    /// Human-readable provenance disclosure note.
    pub provenance_disclosure_note: String,
    /// The diff-based proposal.
    pub proposal: PanelProposal,
    /// The action set (apply / open-evidence / open-source / dismiss / save).
    pub actions: PanelActionSet,
    /// The durable disposition.
    pub disposition: PanelDisposition,
    /// Whether the suggestion is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
}

/// One export row, mirroring a suggestion.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelSuggestionExportRow {
    /// The suggestion this export row mirrors.
    pub suggestion_id_ref: String,
    /// Target kind (must match the suggestion).
    pub target_kind: PanelTargetKind,
    /// Trigger source (must match the suggestion).
    pub trigger_source: PanelTriggerSource,
    /// Confidence (must match the suggestion's chip).
    pub confidence: PanelConfidence,
    /// Freshness (must match the suggestion's chip).
    pub freshness: PanelFreshness,
    /// Apply posture (must match the suggestion's actions).
    pub apply_posture: PanelApplyPosture,
    /// Provenance (must match the suggestion).
    pub provenance: PanelEvidenceProvenance,
    /// Disposition state (must match the suggestion).
    pub disposition_state: PanelDispositionState,
    /// Whether the suggestion keeps full action parity.
    pub action_parity_complete: bool,
    /// Whether the suggestion is cited.
    pub cited: bool,
}

/// The docs-suggestion-panel export projection for the suggestion set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSuggestionPanelExport {
    /// Scope this export covers.
    pub scope: PanelExportScope,
    /// Whether the export preserves each suggestion's target.
    pub preserves_target: bool,
    /// Whether the export preserves each suggestion's trigger source.
    pub preserves_trigger_source: bool,
    /// Whether the export preserves each suggestion's confidence label.
    pub preserves_confidence: bool,
    /// Whether the export preserves each suggestion's freshness label.
    pub preserves_freshness: bool,
    /// Whether the export preserves each suggestion's apply posture.
    pub preserves_apply_posture: bool,
    /// Whether the export preserves each suggestion's provenance.
    pub preserves_provenance: bool,
    /// Whether the export preserves the full action parity.
    pub preserves_action_parity: bool,
    /// Whether the export preserves each suggestion's disposition state.
    pub preserves_disposition: bool,
    /// Per-suggestion export rows.
    pub rows: Vec<PanelSuggestionExportRow>,
}

impl DocsSuggestionPanelExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_target
            && self.preserves_trigger_source
            && self.preserves_confidence
            && self.preserves_freshness
            && self.preserves_apply_posture
            && self.preserves_provenance
            && self.preserves_action_parity
            && self.preserves_disposition
    }
}

/// A packet-level docs-suggestion degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelDegradation {
    /// Degradation class.
    pub degradation_class: PanelDegradationClass,
    /// Severity.
    pub severity: PanelFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The suggestion this degradation annotates, if scoped to one suggestion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggestion_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the docs-suggestion set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelConsumerProjection {
    /// Surface that consumes the set.
    pub surface: PanelConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the target identities.
    pub preserves_targets: bool,
    /// Whether the surface preserves the trigger sources.
    pub preserves_trigger_sources: bool,
    /// Whether the surface preserves the chip set.
    pub preserves_chips: bool,
    /// Whether the surface preserves the apply postures.
    pub preserves_apply_posture: bool,
    /// Whether the surface preserves the full action parity.
    pub preserves_action_parity: bool,
    /// Whether the surface preserves the provenance disclosures.
    pub preserves_provenance: bool,
    /// Whether the surface preserves the disposition history.
    pub preserves_disposition: bool,
}

impl PanelConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_targets
            && self.preserves_trigger_sources
            && self.preserves_chips
            && self.preserves_apply_posture
            && self.preserves_action_parity
            && self.preserves_provenance
            && self.preserves_disposition
    }
}

/// A single validation finding on the docs-suggestion set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PanelValidationFinding {
    /// Finding kind.
    pub finding_kind: PanelFindingKind,
    /// Finding severity.
    pub severity: PanelFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`DocsSuggestionPanelPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSuggestionPanelPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable panel label (no raw URLs / no raw bodies).
    pub panel_label: String,
    /// Opaque digest/ref for the panel session.
    pub panel_digest_ref: String,
    /// The docs suggestions.
    pub suggestions: Vec<PanelSuggestion>,
    /// The export projection.
    pub export: DocsSuggestionPanelExport,
    /// Packet-level degradations.
    pub panel_degradations: Vec<PanelDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<PanelConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe docs-suggestion-panel packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSuggestionPanelPacket {
    /// Record kind; must equal [`DOCS_SUGGESTION_PANEL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_SUGGESTION_PANEL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable panel label.
    pub panel_label: String,
    /// Opaque digest/ref for the panel session.
    pub panel_digest_ref: String,
    /// The docs suggestions.
    pub suggestions: Vec<PanelSuggestion>,
    /// The export projection.
    pub export: DocsSuggestionPanelExport,
    /// Packet-level degradations.
    pub panel_degradations: Vec<PanelDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<PanelConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: PanelPromotionState,
    /// Computed validation findings.
    pub validation_findings: Vec<PanelValidationFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every docs-suggestion packet must project.
const REQUIRED_SURFACES: [PanelConsumerSurface; 4] = [
    PanelConsumerSurface::DocsSuggestionPanel,
    PanelConsumerSurface::DocsAuthoringSurface,
    PanelConsumerSurface::DocsReviewPanel,
    PanelConsumerSurface::SupportExport,
];

impl DocsSuggestionPanelPacket {
    /// Materializes a docs-suggestion-panel packet, computing validation findings
    /// and the promotion state from the input.
    pub fn materialize(input: DocsSuggestionPanelPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_suggestions(&input, &mut findings);
        check_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.panel_degradations);

        Self {
            record_kind: DOCS_SUGGESTION_PANEL_RECORD_KIND.to_owned(),
            schema_version: DOCS_SUGGESTION_PANEL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            panel_label: input.panel_label,
            panel_digest_ref: input.panel_digest_ref,
            suggestions: input.suggestions,
            export: input.export,
            panel_degradations: input.panel_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            validation_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the panel qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == PanelPromotionState::Stable && self.validation_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> DocsSuggestionPanelSupportExport {
        DocsSuggestionPanelSupportExport {
            record_kind: DOCS_SUGGESTION_PANEL_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_SUGGESTION_PANEL_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: DOCS_SUGGESTION_PANEL_SCHEMA_REF.to_owned(),
            doc_ref: DOCS_SUGGESTION_PANEL_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs-suggestion-panel packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# Docs Suggestion Panel (diff-first proposals)\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Panel: {}\n", self.panel_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.validation_findings.len()
        ));
        out.push_str(&format!(
            "- Suggestions: {} | Degradations: {}\n",
            self.suggestions.len(),
            self.panel_degradations.len()
        ));
        out.push_str("\n## Suggestions\n\n");
        for suggestion in &self.suggestions {
            let section = suggestion
                .target
                .section_anchor
                .as_deref()
                .map(|anchor| format!("#{anchor}"))
                .unwrap_or_default();
            out.push_str(&format!(
                "- [{}] `{}` ({}) — target `{}{}` — trigger `{}`\n",
                suggestion.target.target_kind.as_str(),
                suggestion.suggestion_id,
                suggestion.title,
                suggestion.target.display_path,
                section,
                suggestion.trigger.source.as_str(),
            ));
            out.push_str(&format!(
                "  - Chips: {} / {} / {} / {}\n",
                suggestion.chips.confidence.as_str(),
                suggestion.chips.freshness.as_str(),
                suggestion.chips.version_match.as_str(),
                suggestion.chips.locality.as_str(),
            ));
            out.push_str(&format!(
                "  - Proposal: `{}` ({} hunks, +{}/-{})\n",
                suggestion.proposal.proposal_kind.as_str(),
                suggestion.proposal.hunk_count,
                suggestion.proposal.added_lines,
                suggestion.proposal.removed_lines,
            ));
            out.push_str(&format!(
                "  - Actions: apply `{}` | open-evidence `{}` | open-source `{}` | dismiss {} | save {}\n",
                suggestion.actions.apply_posture.as_str(),
                suggestion.actions.open_evidence_ref,
                suggestion.actions.open_source_ref,
                suggestion.actions.dismiss_available,
                suggestion.actions.save_for_later_available,
            ));
            out.push_str(&format!(
                "  - Provenance: `{}` | disposition `{}` | cited {}\n",
                suggestion.provenance.as_str(),
                suggestion.disposition.state.as_str(),
                suggestion.cited,
            ));
        }
        if !self.panel_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.panel_degradations {
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

/// Support-export envelope for the docs-suggestion-panel packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsSuggestionPanelSupportExport {
    /// Record kind; must equal [`DOCS_SUGGESTION_PANEL_SUPPORT_EXPORT_RECORD_KIND`].
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
    /// The wrapped docs-suggestion-panel packet.
    pub packet: DocsSuggestionPanelPacket,
}

/// Errors emitted when reading the checked-in docs-suggestion-panel support export.
#[derive(Debug)]
pub enum DocsSuggestionPanelArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: PanelPromotionState,
        /// Promotion state computed by re-materialization.
        computed: PanelPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<PanelValidationFinding>),
}

impl fmt::Display for DocsSuggestionPanelArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "docs-suggestion-panel export parse failed: {error}"
                )
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "docs-suggestion-panel promotion drift: recorded {} but computed {}",
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
                    "docs-suggestion-panel export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsSuggestionPanelArtifactError {}

/// Reads and re-validates the checked-in stable docs-suggestion-panel support export.
pub fn current_stable_docs_suggestion_panel_export(
) -> Result<DocsSuggestionPanelSupportExport, DocsSuggestionPanelArtifactError> {
    let export: DocsSuggestionPanelSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/docs-suggestion-proof/support_export.json"
    )))
    .map_err(DocsSuggestionPanelArtifactError::SupportExport)?;

    let recomputed = DocsSuggestionPanelPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(DocsSuggestionPanelArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(DocsSuggestionPanelArtifactError::NotCleanStable(
            export.packet.validation_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &DocsSuggestionPanelPacket) -> DocsSuggestionPanelPacketInput {
    DocsSuggestionPanelPacketInput {
        packet_id: packet.packet_id.clone(),
        panel_label: packet.panel_label.clone(),
        panel_digest_ref: packet.panel_digest_ref.clone(),
        suggestions: packet.suggestions.clone(),
        export: packet.export.clone(),
        panel_degradations: packet.panel_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<PanelValidationFinding>,
    kind: PanelFindingKind,
    summary: impl Into<String>,
) {
    findings.push(PanelValidationFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(
    input: &DocsSuggestionPanelPacketInput,
    findings: &mut Vec<PanelValidationFinding>,
) {
    if input.packet_id.trim().is_empty()
        || input.panel_label.trim().is_empty()
        || input.panel_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            PanelFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_suggestions(
    input: &DocsSuggestionPanelPacketInput,
    findings: &mut Vec<PanelValidationFinding>,
) {
    if input.suggestions.is_empty() {
        push_finding(
            findings,
            PanelFindingKind::SuggestionsEmpty,
            "the docs-suggestion panel must carry at least one suggestion",
        );
        return;
    }

    let present_kinds: BTreeSet<PanelTargetKind> = input
        .suggestions
        .iter()
        .map(|suggestion| suggestion.target.target_kind)
        .collect();
    for required in PanelTargetKind::REQUIRED {
        if !present_kinds.contains(&required) {
            push_finding(
                findings,
                PanelFindingKind::RequiredTargetKindMissing,
                format!("required target kind `{}` is missing", required.as_str()),
            );
        }
    }

    let mut seen_ids: BTreeSet<&str> = BTreeSet::new();
    for suggestion in &input.suggestions {
        if !seen_ids.insert(suggestion.suggestion_id.as_str()) {
            push_finding(
                findings,
                PanelFindingKind::DuplicateSuggestionId,
                format!("duplicate suggestion id `{}`", suggestion.suggestion_id),
            );
        }
        check_one_suggestion(suggestion, findings);
    }
}

fn check_one_suggestion(suggestion: &PanelSuggestion, findings: &mut Vec<PanelValidationFinding>) {
    let id = &suggestion.suggestion_id;

    // Concrete target.
    if !suggestion.target.names_concrete_target() {
        push_finding(
            findings,
            PanelFindingKind::TargetIdentityMissing,
            format!("suggestion `{id}` must name a concrete target file/section"),
        );
    }
    // Concrete trigger source.
    if !suggestion.trigger.names_concrete_source() {
        push_finding(
            findings,
            PanelFindingKind::TriggerSourceDetailMissing,
            format!("suggestion `{id}` must name a concrete trigger source detail and evidence"),
        );
    }
    if suggestion.title.trim().is_empty() || suggestion.detail.trim().is_empty() {
        push_finding(
            findings,
            PanelFindingKind::TitleOrDetailMissing,
            format!("suggestion `{id}` is missing a title or detail"),
        );
    }
    if suggestion.provenance_disclosure_note.trim().is_empty() {
        push_finding(
            findings,
            PanelFindingKind::ProvenanceDisclosureMissing,
            format!("suggestion `{id}` is missing its provenance disclosure"),
        );
    }

    // Unverified evidence must stay visible: never high-confidence live truth.
    if !suggestion.provenance.is_authoritative()
        && suggestion.chips.confidence == PanelConfidence::High
        && suggestion.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            PanelFindingKind::ProvenanceTruthCollapsed,
            format!(
                "suggestion `{id}` is `{}` but presented as high-confidence live truth",
                suggestion.provenance.as_str()
            ),
        );
    }
    // An unverified evidence source must stay cited.
    if suggestion.provenance.needs_citation() && !suggestion.cited {
        push_finding(
            findings,
            PanelFindingKind::SuggestionNotCited,
            format!(
                "suggestion `{id}` is `{}` but is not cited",
                suggestion.provenance.as_str()
            ),
        );
    }
    // A non-current version may not be presented as a confident live match.
    if !suggestion.chips.version_match.is_confident_current()
        && suggestion.chips.confidence == PanelConfidence::High
        && suggestion.chips.freshness.is_authoritative_live()
    {
        push_finding(
            findings,
            PanelFindingKind::VersionTruthCollapsed,
            format!(
                "suggestion `{id}` presents version `{}` as a confident live match",
                suggestion.chips.version_match.as_str()
            ),
        );
    }

    // Diff-first proposals.
    if !suggestion.proposal.is_diff_based() {
        push_finding(
            findings,
            PanelFindingKind::ProposalNotDiffBased,
            format!(
                "suggestion `{id}` proposal `{}` is not diff-based ({} hunks)",
                suggestion.proposal.proposal_kind.as_str(),
                suggestion.proposal.hunk_count
            ),
        );
    }
    if suggestion.proposal.summary.trim().is_empty()
        || suggestion.proposal.preview_ref.trim().is_empty()
    {
        push_finding(
            findings,
            PanelFindingKind::ProposalSummaryMissing,
            format!("suggestion `{id}` proposal is missing its summary or preview ref"),
        );
    }

    // Action parity.
    if !suggestion.actions.parity_complete() {
        push_finding(
            findings,
            PanelFindingKind::ActionParityIncomplete,
            format!(
                "suggestion `{id}` must keep open-evidence, open-source, dismiss, and save-for-later parity"
            ),
        );
    }
    // An unverified evidence source may never offer a one-click apply.
    if suggestion.actions.apply_posture.offers_one_click_apply()
        && !suggestion.provenance.is_authoritative()
    {
        push_finding(
            findings,
            PanelFindingKind::UnverifiedApplyOffered,
            format!(
                "suggestion `{id}` is `{}` but offers a one-click apply",
                suggestion.provenance.as_str()
            ),
        );
    }

    // Durable, honest disposition.
    if !suggestion.disposition.is_attributable() {
        push_finding(
            findings,
            PanelFindingKind::DispositionNotAttributable,
            format!(
                "suggestion `{id}` disposition `{}` must carry attribution and a durable history ref",
                suggestion.disposition.state.as_str()
            ),
        );
    }
    if !suggestion.disposition.is_reopenable() {
        push_finding(
            findings,
            PanelFindingKind::DispositionNotReopenable,
            format!(
                "suggestion `{id}` disposition `{}` must stay previewable and reopenable",
                suggestion.disposition.state.as_str()
            ),
        );
    }
}

fn check_export(
    input: &DocsSuggestionPanelPacketInput,
    findings: &mut Vec<PanelValidationFinding>,
) {
    let export = &input.export;
    if !export.preserves_all() {
        push_finding(
            findings,
            PanelFindingKind::ExportDropsPreservation,
            "the export must preserve target, trigger source, confidence, freshness, apply posture, provenance, action parity, and disposition",
        );
    }

    let mut export_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &export.rows {
        export_ids.insert(row.suggestion_id_ref.as_str());
        let suggestion = input
            .suggestions
            .iter()
            .find(|suggestion| suggestion.suggestion_id == row.suggestion_id_ref);
        match suggestion {
            None => push_finding(
                findings,
                PanelFindingKind::ExportRowOrphan,
                format!(
                    "export row references unknown suggestion `{}`",
                    row.suggestion_id_ref
                ),
            ),
            Some(suggestion) => check_export_row(suggestion, row, findings),
        }
    }

    for suggestion in &input.suggestions {
        if !export_ids.contains(suggestion.suggestion_id.as_str()) {
            push_finding(
                findings,
                PanelFindingKind::ExportCoverageMissing,
                format!(
                    "suggestion `{}` has no export row",
                    suggestion.suggestion_id
                ),
            );
        }
    }
}

fn check_export_row(
    suggestion: &PanelSuggestion,
    row: &PanelSuggestionExportRow,
    findings: &mut Vec<PanelValidationFinding>,
) {
    let id = &row.suggestion_id_ref;
    if suggestion.target.target_kind != row.target_kind {
        push_finding(
            findings,
            PanelFindingKind::ExportTargetKindMismatch,
            format!(
                "export for `{id}` records target `{}` but the suggestion is `{}`",
                row.target_kind.as_str(),
                suggestion.target.target_kind.as_str()
            ),
        );
    }
    if suggestion.trigger.source != row.trigger_source {
        push_finding(
            findings,
            PanelFindingKind::ExportTriggerSourceMismatch,
            format!(
                "export for `{id}` records trigger `{}` but the suggestion is `{}`",
                row.trigger_source.as_str(),
                suggestion.trigger.source.as_str()
            ),
        );
    }
    if suggestion.chips.confidence != row.confidence {
        push_finding(
            findings,
            PanelFindingKind::ExportConfidenceMismatch,
            format!(
                "export for `{id}` records confidence `{}` but the chip is `{}`",
                row.confidence.as_str(),
                suggestion.chips.confidence.as_str()
            ),
        );
    }
    if suggestion.chips.freshness != row.freshness {
        push_finding(
            findings,
            PanelFindingKind::ExportFreshnessMismatch,
            format!(
                "export for `{id}` records freshness `{}` but the chip is `{}`",
                row.freshness.as_str(),
                suggestion.chips.freshness.as_str()
            ),
        );
    }
    if suggestion.actions.apply_posture != row.apply_posture {
        push_finding(
            findings,
            PanelFindingKind::ExportApplyPostureMismatch,
            format!(
                "export for `{id}` records apply posture `{}` but the suggestion is `{}`",
                row.apply_posture.as_str(),
                suggestion.actions.apply_posture.as_str()
            ),
        );
    }
    if suggestion.provenance != row.provenance {
        push_finding(
            findings,
            PanelFindingKind::ExportProvenanceMismatch,
            format!(
                "export for `{id}` records provenance `{}` but the suggestion is `{}`",
                row.provenance.as_str(),
                suggestion.provenance.as_str()
            ),
        );
    }
    if suggestion.disposition.state != row.disposition_state {
        push_finding(
            findings,
            PanelFindingKind::ExportDispositionMismatch,
            format!(
                "export for `{id}` records disposition `{}` but the suggestion is `{}`",
                row.disposition_state.as_str(),
                suggestion.disposition.state.as_str()
            ),
        );
    }
    if suggestion.actions.parity_complete() != row.action_parity_complete {
        push_finding(
            findings,
            PanelFindingKind::ActionParityIncomplete,
            format!(
                "export for `{id}` records action parity `{}` but the suggestion is `{}`",
                row.action_parity_complete,
                suggestion.actions.parity_complete()
            ),
        );
    }
    if suggestion.cited != row.cited {
        push_finding(
            findings,
            PanelFindingKind::ExportCitedMismatch,
            format!(
                "export for `{id}` records cited `{}` but the suggestion is `{}`",
                row.cited, suggestion.cited
            ),
        );
    }
}

fn check_degradations(
    input: &DocsSuggestionPanelPacketInput,
    findings: &mut Vec<PanelValidationFinding>,
) {
    let suggestion_ids: BTreeSet<&str> = input
        .suggestions
        .iter()
        .map(|suggestion| suggestion.suggestion_id.as_str())
        .collect();

    for degradation in &input.panel_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                PanelFindingKind::DegradationIncomplete,
                format!(
                    "degradation `{}` is missing a summary",
                    degradation.degradation_class.as_str()
                ),
            );
        }
        if let Some(suggestion_id) = &degradation.suggestion_id_ref {
            if !suggestion_id.trim().is_empty() && !suggestion_ids.contains(suggestion_id.as_str())
            {
                push_finding(
                    findings,
                    PanelFindingKind::DegradationOrphan,
                    format!("degradation references unknown suggestion `{suggestion_id}`"),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &DocsSuggestionPanelPacketInput,
    findings: &mut Vec<PanelValidationFinding>,
) {
    let present: BTreeSet<PanelConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                PanelFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                PanelFindingKind::ConsumerProjectionPacketIdMismatch,
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
                PanelFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(
    input: &DocsSuggestionPanelPacketInput,
    findings: &mut Vec<PanelValidationFinding>,
) {
    let value = serde_json::to_value(input).expect("docs-suggestion-panel input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            PanelFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw URLs, diff bodies, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across the validation
/// findings and the attached degradations.
///
/// A blocking validation finding (missing target, unattributed trigger,
/// prose-only proposal, incomplete action parity, unverified apply, collapsed
/// truth, non-reopenable disposition, or boundary violation) blocks the Stable
/// claim; an otherwise-clean panel whose degradations carry a narrowing severity
/// narrows below Stable rather than hiding the suggestions.
fn promotion_state(
    findings: &[PanelValidationFinding],
    degradations: &[PanelDegradation],
) -> PanelPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == PanelFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == PanelFindingSeverity::Blocking);
    if any_blocking {
        return PanelPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == PanelFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == PanelFindingSeverity::Narrowing);
    if any_narrowing {
        PanelPromotionState::NarrowedBelowStable
    } else {
        PanelPromotionState::Stable
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

/// Seeded stable docs-suggestion-panel input used by the producer, tests, and
/// fixtures.
pub fn seeded_stable_docs_suggestion_panel_input() -> DocsSuggestionPanelPacketInput {
    let packet_id = "packet:m5:docs_suggestion_panel:retry_backoff_release".to_owned();
    DocsSuggestionPanelPacketInput {
        packet_id: packet_id.clone(),
        panel_label: "docs suggestion panel: the retry/backoff release docs sweep".to_owned(),
        panel_digest_ref: "paneldigest:sha256:retry-backoff-release-docs".to_owned(),
        suggestions: vec![
            readme_api_contract_suggestion(),
            changelog_release_metadata_suggestion(),
            help_symbol_rename_suggestion(),
            tutorial_failing_example_suggestion(),
            help_broken_link_suggestion(),
        ],
        export: seeded_export(),
        panel_degradations: vec![PanelDegradation {
            degradation_class: PanelDegradationClass::LinkCheckerOffline,
            severity: PanelFindingSeverity::Advisory,
            summary: "the live link checker was offline for one external host; the broken-link suggestion is served from the last snapshot".to_owned(),
            suggestion_id_ref: Some("suggestion:help:retry_backoff_runbook_link".to_owned()),
            evidence_ref: Some("evidence:docs-suggestion-panel:link-checker-state".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T00:00:00Z".to_owned(),
    }
}

fn pending_disposition(note: &str) -> PanelDisposition {
    PanelDisposition {
        state: PanelDispositionState::Pending,
        attributed_to_ref: None,
        history_ref: None,
        previewable: true,
        reopenable: true,
        note: note.to_owned(),
    }
}

fn readme_api_contract_suggestion() -> PanelSuggestion {
    PanelSuggestion {
        suggestion_id: "suggestion:readme:retry_backoff_api_contract".to_owned(),
        target: PanelSuggestionTarget {
            target_kind: PanelTargetKind::Readme,
            file_ref: "docs/guides/retry_with_backoff/README.md".to_owned(),
            section_anchor: Some("configuration".to_owned()),
            display_path: "README → Configuration".to_owned(),
        },
        trigger: PanelTrigger {
            source: PanelTriggerSource::ApiContractChange,
            detail: "the retry_with_backoff builder gained a max_elapsed parameter in the public contract".to_owned(),
            evidence_ref: "evidence:api-contract:retry_with_backoff#max_elapsed".to_owned(),
        },
        title: "Document the new max_elapsed retry parameter".to_owned(),
        detail: "the README configuration section omits the new max_elapsed parameter the API contract now exposes".to_owned(),
        chips: PanelChipSet {
            confidence: PanelConfidence::High,
            freshness: PanelFreshness::AuthoritativeLive,
            version_match: PanelVersionMatch::ExactBuildMatch,
            locality: PanelLocality::Local,
        },
        provenance: PanelEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "first-party evidence verified against the in-repo public contract; the suggested diff is reviewable and reversible".to_owned(),
        proposal: PanelProposal {
            proposal_kind: PanelProposalKind::DiffHunks,
            hunk_count: 1,
            added_lines: 6,
            removed_lines: 1,
            summary: "add a max_elapsed row to the configuration table and a one-line note".to_owned(),
            preview_ref: "preview:diff:readme:retry_backoff_api_contract".to_owned(),
        },
        actions: PanelActionSet {
            apply_posture: PanelApplyPosture::ApplyAvailable,
            open_evidence_ref: "open-evidence:api-contract:retry_with_backoff#max_elapsed".to_owned(),
            open_source_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
            dismiss_available: true,
            save_for_later_available: true,
        },
        disposition: pending_disposition(
            "open; the diff previews cleanly against the current README",
        ),
        cited: true,
        citation_ref: Some("cite:api-contract:retry_with_backoff#max_elapsed".to_owned()),
    }
}

fn changelog_release_metadata_suggestion() -> PanelSuggestion {
    PanelSuggestion {
        suggestion_id: "suggestion:changelog:retry_backoff_release_entry".to_owned(),
        target: PanelSuggestionTarget {
            target_kind: PanelTargetKind::Changelog,
            file_ref: "CHANGELOG.md".to_owned(),
            section_anchor: Some("unreleased".to_owned()),
            display_path: "CHANGELOG → Unreleased".to_owned(),
        },
        trigger: PanelTrigger {
            source: PanelTriggerSource::ReleaseMetadataChange,
            detail: "the release metadata promoted the retry/backoff change into the next channel".to_owned(),
            evidence_ref: "evidence:release-metadata:next-channel#retry_backoff".to_owned(),
        },
        title: "Add a changelog entry for the retry/backoff change".to_owned(),
        detail: "the unreleased changelog section is missing the retry/backoff entry the release metadata now schedules".to_owned(),
        chips: PanelChipSet {
            confidence: PanelConfidence::High,
            freshness: PanelFreshness::AuthoritativeLive,
            version_match: PanelVersionMatch::ExactBuildMatch,
            locality: PanelLocality::Local,
        },
        provenance: PanelEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "first-party evidence verified against the release metadata; the suggested entry is reviewable and reversible".to_owned(),
        proposal: PanelProposal {
            proposal_kind: PanelProposalKind::NewSectionDiff,
            hunk_count: 1,
            added_lines: 3,
            removed_lines: 0,
            summary: "add an Added bullet under Unreleased describing the retry/backoff change".to_owned(),
            preview_ref: "preview:diff:changelog:retry_backoff_release_entry".to_owned(),
        },
        actions: PanelActionSet {
            apply_posture: PanelApplyPosture::ApplyAvailable,
            open_evidence_ref: "open-evidence:release-metadata:next-channel#retry_backoff".to_owned(),
            open_source_ref: "open-source:repo:artifacts/release/next_channel.yaml".to_owned(),
            dismiss_available: true,
            save_for_later_available: true,
        },
        disposition: PanelDisposition {
            state: PanelDispositionState::Applied,
            attributed_to_ref: Some("actor:maintainer:docs-owner".to_owned()),
            history_ref: Some("history:docs-suggestion-panel:changelog_retry_backoff#applied".to_owned()),
            previewable: true,
            reopenable: true,
            note: "applied by the docs owner; the change is recorded in durable history and can be reopened".to_owned(),
        },
        cited: true,
        citation_ref: Some("cite:release-metadata:next-channel#retry_backoff".to_owned()),
    }
}

fn help_symbol_rename_suggestion() -> PanelSuggestion {
    PanelSuggestion {
        suggestion_id: "suggestion:help:retry_backoff_symbol_rename".to_owned(),
        target: PanelSuggestionTarget {
            target_kind: PanelTargetKind::Help,
            file_ref: "docs/help/retry-and-backoff.md".to_owned(),
            section_anchor: Some("builder-api".to_owned()),
            display_path: "Help → Retry and backoff → Builder API".to_owned(),
        },
        trigger: PanelTrigger {
            source: PanelTriggerSource::SymbolRename,
            detail: "RetryPolicy::with_jitter was renamed to RetryPolicy::with_full_jitter".to_owned(),
            evidence_ref: "evidence:symbol-rename:RetryPolicy::with_jitter".to_owned(),
        },
        title: "Update the renamed with_full_jitter symbol in help".to_owned(),
        detail: "the help builder-API section still references the old with_jitter symbol after the rename".to_owned(),
        chips: PanelChipSet {
            confidence: PanelConfidence::High,
            freshness: PanelFreshness::AuthoritativeLive,
            version_match: PanelVersionMatch::ExactBuildMatch,
            locality: PanelLocality::Local,
        },
        provenance: PanelEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "first-party evidence verified against the renamed symbol; the suggested diff is reviewable and reversible".to_owned(),
        proposal: PanelProposal {
            proposal_kind: PanelProposalKind::DiffHunks,
            hunk_count: 2,
            added_lines: 2,
            removed_lines: 2,
            summary: "replace with_jitter with with_full_jitter in the builder example and prose".to_owned(),
            preview_ref: "preview:diff:help:retry_backoff_symbol_rename".to_owned(),
        },
        actions: PanelActionSet {
            apply_posture: PanelApplyPosture::ApplyAvailable,
            open_evidence_ref: "open-evidence:symbol-rename:RetryPolicy::with_jitter".to_owned(),
            open_source_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
            dismiss_available: true,
            save_for_later_available: true,
        },
        disposition: pending_disposition(
            "open; the rename diff previews cleanly across the example and prose",
        ),
        cited: true,
        citation_ref: Some("cite:symbol-rename:RetryPolicy::with_jitter".to_owned()),
    }
}

fn tutorial_failing_example_suggestion() -> PanelSuggestion {
    PanelSuggestion {
        suggestion_id: "suggestion:tutorial:retry_backoff_failing_example".to_owned(),
        target: PanelSuggestionTarget {
            target_kind: PanelTargetKind::Tutorial,
            file_ref: "docs/tutorials/resilient-networking.md".to_owned(),
            section_anchor: Some("step-3-add-backoff".to_owned()),
            display_path: "Tutorial → Resilient networking → Step 3: add backoff".to_owned(),
        },
        trigger: PanelTrigger {
            source: PanelTriggerSource::FailingExample,
            detail: "the step-3 backoff example no longer compiles against the renamed builder".to_owned(),
            evidence_ref: "evidence:failing-example:resilient-networking#step-3".to_owned(),
        },
        title: "Fix the failing step-3 backoff example".to_owned(),
        detail: "the tutorial's step-3 example fails to compile after the builder rename; the fix is shown as a diff and held for preview".to_owned(),
        chips: PanelChipSet {
            confidence: PanelConfidence::Medium,
            freshness: PanelFreshness::WarmCached,
            version_match: PanelVersionMatch::CompatibleMinorDrift,
            locality: PanelLocality::Local,
        },
        provenance: PanelEvidenceProvenance::FirstPartyVerified,
        provenance_disclosure_note: "first-party evidence from the example harness; the proposed fix is held for preview pending a re-run".to_owned(),
        proposal: PanelProposal {
            proposal_kind: PanelProposalKind::ExampleReplaceDiff,
            hunk_count: 1,
            added_lines: 4,
            removed_lines: 4,
            summary: "replace the step-3 example body with the compiling with_full_jitter form".to_owned(),
            preview_ref: "preview:diff:tutorial:retry_backoff_failing_example".to_owned(),
        },
        actions: PanelActionSet {
            apply_posture: PanelApplyPosture::PreviewRequired,
            open_evidence_ref: "open-evidence:failing-example:resilient-networking#step-3".to_owned(),
            open_source_ref: "open-source:repo:crates/aureline-net/examples/backoff.rs".to_owned(),
            dismiss_available: true,
            save_for_later_available: true,
        },
        disposition: PanelDisposition {
            state: PanelDispositionState::SavedForLater,
            attributed_to_ref: Some("actor:maintainer:docs-owner".to_owned()),
            history_ref: Some("history:docs-suggestion-panel:tutorial_failing_example#saved".to_owned()),
            previewable: true,
            reopenable: true,
            note: "saved for later by the docs owner pending the harness re-run; recorded in durable history".to_owned(),
        },
        cited: true,
        citation_ref: Some("cite:failing-example:resilient-networking#step-3".to_owned()),
    }
}

fn help_broken_link_suggestion() -> PanelSuggestion {
    PanelSuggestion {
        suggestion_id: "suggestion:help:retry_backoff_runbook_link".to_owned(),
        target: PanelSuggestionTarget {
            target_kind: PanelTargetKind::Help,
            file_ref: "docs/help/retry-and-backoff.md".to_owned(),
            section_anchor: Some("operations-runbook".to_owned()),
            display_path: "Help → Retry and backoff → Operations runbook".to_owned(),
        },
        trigger: PanelTrigger {
            source: PanelTriggerSource::BrokenLink,
            detail: "the operations runbook link returns a redirect after the page was renamed".to_owned(),
            evidence_ref: "evidence:broken-link:ops/runbooks/retry_backoff_runbook".to_owned(),
        },
        title: "Repoint the redirected operations runbook link".to_owned(),
        detail: "the help operations-runbook link redirects to a renamed page in the imported ops pack; the repoint is shown as a diff and held for preview".to_owned(),
        chips: PanelChipSet {
            confidence: PanelConfidence::Medium,
            freshness: PanelFreshness::WarmCached,
            version_match: PanelVersionMatch::CompatibleMinorDrift,
            locality: PanelLocality::ImportedPack,
        },
        provenance: PanelEvidenceProvenance::Imported,
        provenance_disclosure_note: "imported from the signed ops docs pack; the redirect target is disclosed and held to medium pending a re-check — not authoritative repo truth".to_owned(),
        proposal: PanelProposal {
            proposal_kind: PanelProposalKind::LinkRepointDiff,
            hunk_count: 1,
            added_lines: 1,
            removed_lines: 1,
            summary: "repoint the runbook link to the redirect target".to_owned(),
            preview_ref: "preview:diff:help:retry_backoff_runbook_link".to_owned(),
        },
        actions: PanelActionSet {
            apply_posture: PanelApplyPosture::PreviewRequired,
            open_evidence_ref: "open-evidence:broken-link:ops/runbooks/retry_backoff_runbook".to_owned(),
            open_source_ref: "open-source:pack:ops/runbooks/retry_backoff_runbook.md".to_owned(),
            dismiss_available: true,
            save_for_later_available: true,
        },
        disposition: pending_disposition(
            "open; the repoint is held for preview because the evidence is imported and mirror-served",
        ),
        cited: true,
        citation_ref: Some("cite:broken-link:ops/runbooks/retry_backoff_runbook".to_owned()),
    }
}

fn export_row(suggestion: &PanelSuggestion) -> PanelSuggestionExportRow {
    PanelSuggestionExportRow {
        suggestion_id_ref: suggestion.suggestion_id.clone(),
        target_kind: suggestion.target.target_kind,
        trigger_source: suggestion.trigger.source,
        confidence: suggestion.chips.confidence,
        freshness: suggestion.chips.freshness,
        apply_posture: suggestion.actions.apply_posture,
        provenance: suggestion.provenance,
        disposition_state: suggestion.disposition.state,
        action_parity_complete: suggestion.actions.parity_complete(),
        cited: suggestion.cited,
    }
}

fn seeded_export() -> DocsSuggestionPanelExport {
    let rows = [
        readme_api_contract_suggestion(),
        changelog_release_metadata_suggestion(),
        help_symbol_rename_suggestion(),
        tutorial_failing_example_suggestion(),
        help_broken_link_suggestion(),
    ]
    .iter()
    .map(export_row)
    .collect();
    DocsSuggestionPanelExport {
        scope: PanelExportScope::AllSuggestions,
        preserves_target: true,
        preserves_trigger_source: true,
        preserves_confidence: true,
        preserves_freshness: true,
        preserves_apply_posture: true,
        preserves_provenance: true,
        preserves_action_parity: true,
        preserves_disposition: true,
        rows,
    }
}

fn required_projections(packet_id: &str) -> Vec<PanelConsumerProjection> {
    [
        PanelConsumerSurface::DocsSuggestionPanel,
        PanelConsumerSurface::DocsAuthoringSurface,
        PanelConsumerSurface::DocsReviewPanel,
        PanelConsumerSurface::DocsBrowserShell,
        PanelConsumerSurface::ReleaseCenter,
        PanelConsumerSurface::AiContextInspector,
        PanelConsumerSurface::CliHeadless,
        PanelConsumerSurface::SupportExport,
        PanelConsumerSurface::Diagnostics,
        PanelConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| PanelConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_targets: true,
        preserves_trigger_sources: true,
        preserves_chips: true,
        preserves_apply_posture: true,
        preserves_action_parity: true,
        preserves_provenance: true,
        preserves_disposition: true,
    })
    .collect()
}
