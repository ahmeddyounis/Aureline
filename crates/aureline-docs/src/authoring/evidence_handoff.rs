//! Docs-evidence handoff packets: bind a prose change or suggestion to the
//! files, symbols, API contracts, failing examples, test runs, release objects,
//! or human-authored notes that motivated it, with freshness, scope, redaction,
//! and mirror/offline truth preserved end to end.
//!
//! This module owns the runtime truth packet behind the docs-evidence handoff —
//! the surface that makes a docs change explainable by the code, schema, run, or
//! release evidence that raised it instead of free-form narrative alone. Each
//! [`EvidenceHandoffEntry`] names one concrete prose change (the doc file and,
//! when applicable, the section anchor, plus the originating suggestion when the
//! change came from the suggestion panel) and binds it to one or more
//! [`EvidenceBinding`]s. Each binding names a typed evidence object — a source
//! file, a symbol, an API contract/schema, a failing example, a test run, a
//! release object, or a human-authored note — its [`EvidenceScope`]
//! (local-only, review-scoped, export-safe shared, or blocked-unscoped), its
//! [`EvidenceRedactionState`], an [`EvidenceProvenance`] disclosure, a
//! freshness/version/locality posture, a [`MirrorOfflinePosture`] so the binding
//! stays usable in air-gapped or mirror-first profiles, and an open-evidence ref
//! so review, support, and AI flows can reopen the evidence Aureline used.
//!
//! Six invariants make a handoff entry honest:
//!
//! - **Concrete, typed traceability.** Every entry binds its change to at least
//!   one *concrete* typed evidence object (a file, symbol, contract, failing
//!   example, test run, or release object); an entry with no bindings, or whose
//!   only bindings are human notes, relies on narrative alone and is rejected
//!   ([`HandoffFindingKind::ChangeNotConcretelyTraced`]). The canonical packet
//!   demonstrates the full concrete evidence taxonomy.
//! - **Scope and redaction honesty.** A binding that crosses a share/export
//!   boundary must carry an export-safe redaction state, a local-only-redaction
//!   binding must stay local, a local-only-unverified source may not be marked
//!   export-safe, and an entry's scope may never be wider than its bindings — so
//!   local-only evidence is never silently widened to shared/export.
//! - **Mirror/offline continuity.** Every binding carries a mirror/offline
//!   posture; a mirror-served or offline binding may never claim authoritative
//!   live freshness, so docs causality stays truthful in mirror-first and
//!   air-gapped profiles instead of assuming always-online sharing.
//! - **Provenance and freshness truth.** Only first-party evidence is
//!   authoritative; any imported, mirrored, stale, derived, or local-only source
//!   presented as authoritative-live truth collapses
//!   ([`HandoffFindingKind::EvidenceTruthCollapsed`]), a non-current version
//!   presented as authoritative-live collapses
//!   ([`HandoffFindingKind::VersionTruthCollapsed`]), and a non-first-party
//!   binding must stay cited.
//! - **Reopenable from review and support.** Every entry carries a reopen handle
//!   that stays reopenable from both the review and support flows, so support and
//!   review can reopen the same docs-evidence packet Aureline used in the
//!   authoring workspace.
//! - **Export and projection parity.** The export and every required consumer
//!   surface (review, AI explanations, release/public-truth, support export)
//!   preserve the change subjects, evidence bindings, scope, redaction,
//!   provenance, freshness, mirror/offline posture, and reopen truth, so docs
//!   causality is never locked inside the authoring pane.
//!
//! [`DocsEvidenceHandoffPacket::materialize`] computes the handoff findings and
//! the promotion state (`stable`, `narrowed_below_stable`, or `blocks_stable`)
//! from the input — folding the packet-level degradation severities into the
//! promotion decision — so a clean packet stays Stable, a degraded-but-honest
//! packet (mirror offline, source index unavailable, refresh pending, narrowed
//! for export) narrows below Stable, and a packet with an untraced change, a
//! widened scope, a collapsed truth, or a non-reopenable entry blocks before it
//! reaches a consumer surface. The packet is an inspectable, serde-serializable
//! truth packet: it carries no raw document bodies, no raw source files, no raw
//! diffs, no raw URLs, no rendered HTML, no raw provider payloads, and no
//! credentials — only metadata, change subjects, evidence-object refs,
//! scope/redaction/provenance/freshness tokens, mirror/offline posture,
//! open-evidence and reopen refs, finding summaries, and contract refs.
//!
//! The boundary schema is
//! [`schemas/docs/docs-evidence-handoff.schema.json`](../../../../schemas/docs/docs-evidence-handoff.schema.json).
//! The contract doc is
//! [`docs/m5/docs-evidence-handoff.md`](../../../../docs/m5/docs-evidence-handoff.md).
//! The protected fixture directory is
//! [`fixtures/docs/m5/docs-evidence-handoff/`](../../../../fixtures/docs/m5/docs-evidence-handoff/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`DocsEvidenceHandoffPacket`].
pub const DOCS_EVIDENCE_HANDOFF_RECORD_KIND: &str = "docs_evidence_handoff";

/// Record-kind tag carried by the support-export wrapper.
pub const DOCS_EVIDENCE_HANDOFF_SUPPORT_EXPORT_RECORD_KIND: &str =
    "docs_evidence_handoff_support_export";

/// Schema version for docs-evidence-handoff records.
pub const DOCS_EVIDENCE_HANDOFF_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const DOCS_EVIDENCE_HANDOFF_SCHEMA_REF: &str = "schemas/docs/docs-evidence-handoff.schema.json";

/// Repo-relative path of the docs-evidence-handoff contract doc.
pub const DOCS_EVIDENCE_HANDOFF_DOC_REF: &str = "docs/m5/docs-evidence-handoff.md";

/// Repo-relative path of the protected fixture directory.
pub const DOCS_EVIDENCE_HANDOFF_FIXTURE_DIR: &str = "fixtures/docs/m5/docs-evidence-handoff";

/// Repo-relative path of the checked support-export artifact.
pub const DOCS_EVIDENCE_HANDOFF_ARTIFACT_REF: &str =
    "artifacts/docs/m5/docs-evidence-handoff-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const DOCS_EVIDENCE_HANDOFF_SUMMARY_REF: &str =
    "artifacts/docs/m5/docs-evidence-handoff-proof.md";

/// Kind of prose change a handoff entry explains.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DocsChangeKind {
    /// A README edit.
    ReadmeEdit,
    /// A changelog entry.
    ChangelogEntry,
    /// A release-note edit.
    ReleaseNoteEdit,
    /// A help-doc edit.
    HelpEdit,
    /// A tutorial edit.
    TutorialEdit,
    /// An API-reference edit.
    ApiReferenceEdit,
    /// A module / source-doc edit.
    ModuleDocEdit,
    /// A not-yet-applied docs suggestion proposal.
    SuggestionProposal,
}

impl DocsChangeKind {
    /// Stable token recorded in the entry.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadmeEdit => "readme_edit",
            Self::ChangelogEntry => "changelog_entry",
            Self::ReleaseNoteEdit => "release_note_edit",
            Self::HelpEdit => "help_edit",
            Self::TutorialEdit => "tutorial_edit",
            Self::ApiReferenceEdit => "api_reference_edit",
            Self::ModuleDocEdit => "module_doc_edit",
            Self::SuggestionProposal => "suggestion_proposal",
        }
    }
}

/// Kind of typed evidence object a binding points at.
///
/// These are the concrete object types a prose change can be traced back to —
/// source files, symbols, API contracts/schemas, failing examples, test runs,
/// release objects, and human-authored notes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    /// A repo-relative source file.
    SourceFile,
    /// A named symbol (function, type, constant) in source.
    Symbol,
    /// An API contract / schema object.
    ApiContract,
    /// A documented example that failed validation.
    FailingExample,
    /// A test / example run object.
    TestRun,
    /// A release object (release tag, channel, artifact bundle).
    ReleaseObject,
    /// A human-authored maintenance note.
    HumanNote,
}

impl EvidenceKind {
    /// The concrete evidence kinds the canonical packet must demonstrate, so the
    /// handoff proves a docs change can be traced to files, symbols, contracts,
    /// failing examples, runs, and releases rather than narrative alone.
    pub const REQUIRED_COVERAGE: [Self; 6] = [
        Self::SourceFile,
        Self::Symbol,
        Self::ApiContract,
        Self::FailingExample,
        Self::TestRun,
        Self::ReleaseObject,
    ];

    /// Stable token recorded in the binding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFile => "source_file",
            Self::Symbol => "symbol",
            Self::ApiContract => "api_contract",
            Self::FailingExample => "failing_example",
            Self::TestRun => "test_run",
            Self::ReleaseObject => "release_object",
            Self::HumanNote => "human_note",
        }
    }

    /// Whether this kind is a concrete typed evidence object (anything but a
    /// free-form human note).
    pub const fn is_concrete(self) -> bool {
        !matches!(self, Self::HumanNote)
    }
}

/// Sharing/export scope for an evidence binding or handoff entry.
///
/// Mirrors the canonical evidence-handoff scope vocabulary owned by the
/// docs-authoring matrix: work stays local unless it crosses an explicit, scoped
/// review or export boundary, and an unscoped external share attempt is blocked.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceScope {
    /// Stays local to the workspace; never crosses a share/export boundary.
    LocalOnly,
    /// Staged for a scoped review handoff that stays inside review.
    ReviewHandoffScoped,
    /// Export-safe; may cross the support/export/public-truth boundary.
    ExportSafeShared,
    /// An external share/export was attempted without scope and is blocked.
    BlockedUnscoped,
}

impl EvidenceScope {
    /// Stable token recorded in the scope.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ReviewHandoffScoped => "review_handoff_scoped",
            Self::ExportSafeShared => "export_safe_shared",
            Self::BlockedUnscoped => "blocked_unscoped",
        }
    }

    /// Whether the scope keeps the evidence strictly local.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::LocalOnly)
    }

    /// Whether the scope crosses a review or export share boundary.
    pub const fn crosses_share_boundary(self) -> bool {
        matches!(self, Self::ReviewHandoffScoped | Self::ExportSafeShared)
    }

    /// Whether the scope is export-safe (may cross the support/export boundary).
    pub const fn is_export_safe(self) -> bool {
        matches!(self, Self::ExportSafeShared)
    }

    /// Whether an unscoped share/export was attempted and blocked.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BlockedUnscoped)
    }
}

/// Redaction posture for an evidence binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceRedactionState {
    /// Only metadata is present; safe to export as-is.
    MetadataSafe,
    /// Sensitive material was redacted; the remainder is safe to export.
    RedactedForExport,
    /// Contains local-only material that must not cross an export boundary.
    LocalOnlyRedactionRequired,
    /// Redaction does not apply to this binding.
    NotApplicable,
}

impl EvidenceRedactionState {
    /// Stable token recorded in the binding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MetadataSafe => "metadata_safe",
            Self::RedactedForExport => "redacted_for_export",
            Self::LocalOnlyRedactionRequired => "local_only_redaction_required",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this redaction state is safe to cross the support/export boundary.
    pub const fn is_export_ok(self) -> bool {
        matches!(self, Self::MetadataSafe | Self::RedactedForExport)
    }

    /// Whether this redaction state requires the binding to stay local.
    pub const fn requires_local_only(self) -> bool {
        matches!(self, Self::LocalOnlyRedactionRequired)
    }
}

/// Evidence provenance for a binding, kept visible so a cached or imported source
/// is never mistaken for authoritative live truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceProvenance {
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

impl EvidenceProvenance {
    /// Stable token recorded in the binding.
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

    /// Whether this provenance may back authoritative live evidence. Only
    /// first-party verified evidence may.
    pub const fn is_authoritative(self) -> bool {
        matches!(self, Self::FirstPartyVerified)
    }

    /// Whether a binding of this provenance must stay cited.
    pub const fn needs_citation(self) -> bool {
        !matches!(self, Self::FirstPartyVerified)
    }
}

/// Freshness state for a binding, projected as the freshness chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceFreshness {
    /// Live and authoritative at handoff time.
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

impl EvidenceFreshness {
    /// Stable token recorded in the binding.
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

    /// Whether this state claims live authoritative freshness.
    pub const fn is_authoritative_live(self) -> bool {
        matches!(self, Self::AuthoritativeLive)
    }
}

/// Version-match state for a binding, projected as the version chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceVersionMatch {
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

impl EvidenceVersionMatch {
    /// Stable token recorded in the binding.
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

/// Locality / install posture for a binding, projected as the locality chip.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLocality {
    /// Resolved from local content or the in-repo index.
    Local,
    /// Resolved through an imported pack.
    ImportedPack,
    /// Resolved through a mirrored pack.
    MirroredPack,
    /// Resolved through a managed (org-hosted) service.
    Managed,
}

impl EvidenceLocality {
    /// Stable token recorded in the binding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::ImportedPack => "imported_pack",
            Self::MirroredPack => "mirrored_pack",
            Self::Managed => "managed",
        }
    }
}

/// Mirror/offline continuity posture for a binding, so docs causality stays
/// usable in air-gapped or mirror-first profiles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MirrorOfflinePosture {
    /// The evidence source is reachable online and live.
    OnlineLive,
    /// The evidence is served from a pinned mirror.
    MirrorServed,
    /// Offline; the evidence is served from a usable local cache.
    OfflineCachedUsable,
    /// Offline and the evidence is unavailable (continuity degraded).
    OfflineUnavailable,
}

impl MirrorOfflinePosture {
    /// Stable token recorded in the binding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnlineLive => "online_live",
            Self::MirrorServed => "mirror_served",
            Self::OfflineCachedUsable => "offline_cached_usable",
            Self::OfflineUnavailable => "offline_unavailable",
        }
    }

    /// Whether the evidence is served from a mirror or while offline (so it may
    /// not claim authoritative live freshness).
    pub const fn is_mirror_or_offline(self) -> bool {
        !matches!(self, Self::OnlineLive)
    }

    /// Whether mirror/offline continuity is degraded (the evidence is offline and
    /// unavailable).
    pub const fn is_continuity_degraded(self) -> bool {
        matches!(self, Self::OfflineUnavailable)
    }
}

/// Severity of a degradation or handoff finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffFindingSeverity {
    /// Blocks a Stable claim; the packet must block.
    Blocking,
    /// Narrows below Stable but the packet stays valid and attributable.
    Narrowing,
    /// Advisory only.
    Advisory,
}

impl HandoffFindingSeverity {
    /// Stable token recorded in the finding.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Blocking => "blocking",
            Self::Narrowing => "narrowing",
            Self::Advisory => "advisory",
        }
    }
}

/// Consumer surface that must project the docs-evidence-handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffConsumerSurface {
    /// The docs evidence-handoff surface itself.
    DocsEvidenceHandoff,
    /// The docs authoring surface.
    DocsAuthoringSurface,
    /// The shared docs/code review panel.
    DocsReviewPanel,
    /// The docs browser shell.
    DocsBrowserShell,
    /// The AI explanation surface.
    AiExplanation,
    /// The release center / public-truth lane.
    ReleasePublicTruth,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl HandoffConsumerSurface {
    /// Stable token recorded in the projection.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsEvidenceHandoff => "docs_evidence_handoff",
            Self::DocsAuthoringSurface => "docs_authoring_surface",
            Self::DocsReviewPanel => "docs_review_panel",
            Self::DocsBrowserShell => "docs_browser_shell",
            Self::AiExplanation => "ai_explanation",
            Self::ReleasePublicTruth => "release_public_truth",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Class of a packet-level docs-evidence-handoff degradation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffDegradationClass {
    /// A mirror is offline; bindings are served from the last snapshot.
    MirrorOfflineSnapshot,
    /// The source index is unavailable, so some bindings could not re-resolve.
    SourceIndexUnavailable,
    /// One or more bindings are pending a freshness refresh.
    EvidenceRefreshPending,
    /// The handoff was narrowed to the export-safe bindings before sharing.
    ScopeNarrowedForExport,
    /// The handoff claim was narrowed before publication.
    HandoffNarrowed,
    /// An owning evidence source is quarantined.
    QuarantinedSource,
}

impl HandoffDegradationClass {
    /// Stable token recorded in the degradation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MirrorOfflineSnapshot => "mirror_offline_snapshot",
            Self::SourceIndexUnavailable => "source_index_unavailable",
            Self::EvidenceRefreshPending => "evidence_refresh_pending",
            Self::ScopeNarrowedForExport => "scope_narrowed_for_export",
            Self::HandoffNarrowed => "handoff_narrowed",
            Self::QuarantinedSource => "quarantined_source",
        }
    }
}

/// Scope a docs-evidence-handoff export covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffExportScope {
    /// Every entry in the packet.
    AllEntries,
    /// Export-safe entries only.
    ExportSafeOnly,
    /// Attention entries (degraded / blocked-scope) only.
    AttentionOnly,
}

impl HandoffExportScope {
    /// Stable token recorded in the export.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllEntries => "all_entries",
            Self::ExportSafeOnly => "export_safe_only",
            Self::AttentionOnly => "attention_only",
        }
    }
}

/// Promotion state computed for the docs-evidence-handoff packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffPromotionState {
    /// Packet qualifies for the Stable claim.
    Stable,
    /// Packet narrowed below Stable but stays valid and attributable.
    NarrowedBelowStable,
    /// Packet has a blocking finding and must not present as Stable.
    BlocksStable,
}

impl HandoffPromotionState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::NarrowedBelowStable => "narrowed_below_stable",
            Self::BlocksStable => "blocks_stable",
        }
    }
}

/// Handoff finding kind emitted by [`DocsEvidenceHandoffPacket::materialize`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HandoffFindingKind {
    /// A required identity field is missing.
    MissingIdentity,
    /// The entry set is empty.
    EntriesEmpty,
    /// An entry id is duplicated.
    DuplicateEntryId,
    /// A binding id is duplicated.
    DuplicateBindingId,
    /// A required concrete evidence kind is missing from the packet.
    RequiredEvidenceKindMissing,
    /// An entry does not name a concrete change doc/section/label.
    ChangeSubjectMissing,
    /// An entry is missing its detail.
    ChangeDetailMissing,
    /// An entry carries no evidence bindings.
    BindingsEmpty,
    /// An entry's only evidence is human notes (narrative alone).
    ChangeNotConcretelyTraced,
    /// A binding does not name a concrete evidence target.
    BindingTargetMissing,
    /// A binding is missing its open-evidence ref.
    BindingOpenEvidenceMissing,
    /// A binding is missing its provenance disclosure note.
    ProvenanceDisclosureMissing,
    /// A binding's scope and redaction state are inconsistent.
    ScopeRedactionInconsistent,
    /// A local-only-unverified binding is marked export-safe.
    LocalOnlyMarkedExportSafe,
    /// An entry's scope is wider than its bindings allow.
    EntryScopeWiderThanBindings,
    /// A mirror-served or offline binding claims authoritative live freshness.
    OfflineClaimsLiveFreshness,
    /// A non-first-party binding is presented as authoritative live truth.
    EvidenceTruthCollapsed,
    /// A non-current version binding is presented as authoritative live truth.
    VersionTruthCollapsed,
    /// A non-first-party binding is not cited.
    BindingNotCited,
    /// An entry is not reopenable from both review and support.
    EntryNotReopenable,
    /// The export drops a required preservation flag.
    ExportDropsPreservation,
    /// An export row references an entry id absent from the entries.
    ExportEntryOrphan,
    /// An entry has no matching export row.
    ExportCoverageMissing,
    /// An export row's change kind disagrees with the entry.
    ExportChangeKindMismatch,
    /// An export row's doc ref disagrees with the entry.
    ExportDocRefMismatch,
    /// An export row's scope disagrees with the entry.
    ExportScopeMismatch,
    /// An export row's export-safe flag disagrees with the entry.
    ExportExportSafeMismatch,
    /// An export row's reopenable flag disagrees with the entry.
    ExportReopenableMismatch,
    /// An export row's evidence kinds disagree with the entry.
    ExportEvidenceKindsMismatch,
    /// An export row's binding count disagrees with the entry.
    ExportBindingCountMismatch,
    /// An export row's cited flag disagrees with the entry.
    ExportCitedMismatch,
    /// A degradation is incomplete (missing summary).
    DegradationIncomplete,
    /// A degradation references an entry id absent from the entries.
    DegradationOrphan,
    /// A consumer projection drops a required preservation flag.
    ConsumerProjectionDrift,
    /// A consumer projection references the wrong packet id.
    ConsumerProjectionPacketIdMismatch,
    /// A required consumer surface is missing from the projections.
    RequiredSurfaceCoverageMissing,
    /// Raw bodies, raw diffs, raw URLs, or secrets crossed the boundary.
    RawBoundaryMaterialPresent,
}

impl HandoffFindingKind {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingIdentity => "missing_identity",
            Self::EntriesEmpty => "entries_empty",
            Self::DuplicateEntryId => "duplicate_entry_id",
            Self::DuplicateBindingId => "duplicate_binding_id",
            Self::RequiredEvidenceKindMissing => "required_evidence_kind_missing",
            Self::ChangeSubjectMissing => "change_subject_missing",
            Self::ChangeDetailMissing => "change_detail_missing",
            Self::BindingsEmpty => "bindings_empty",
            Self::ChangeNotConcretelyTraced => "change_not_concretely_traced",
            Self::BindingTargetMissing => "binding_target_missing",
            Self::BindingOpenEvidenceMissing => "binding_open_evidence_missing",
            Self::ProvenanceDisclosureMissing => "provenance_disclosure_missing",
            Self::ScopeRedactionInconsistent => "scope_redaction_inconsistent",
            Self::LocalOnlyMarkedExportSafe => "local_only_marked_export_safe",
            Self::EntryScopeWiderThanBindings => "entry_scope_wider_than_bindings",
            Self::OfflineClaimsLiveFreshness => "offline_claims_live_freshness",
            Self::EvidenceTruthCollapsed => "evidence_truth_collapsed",
            Self::VersionTruthCollapsed => "version_truth_collapsed",
            Self::BindingNotCited => "binding_not_cited",
            Self::EntryNotReopenable => "entry_not_reopenable",
            Self::ExportDropsPreservation => "export_drops_preservation",
            Self::ExportEntryOrphan => "export_entry_orphan",
            Self::ExportCoverageMissing => "export_coverage_missing",
            Self::ExportChangeKindMismatch => "export_change_kind_mismatch",
            Self::ExportDocRefMismatch => "export_doc_ref_mismatch",
            Self::ExportScopeMismatch => "export_scope_mismatch",
            Self::ExportExportSafeMismatch => "export_export_safe_mismatch",
            Self::ExportReopenableMismatch => "export_reopenable_mismatch",
            Self::ExportEvidenceKindsMismatch => "export_evidence_kinds_mismatch",
            Self::ExportBindingCountMismatch => "export_binding_count_mismatch",
            Self::ExportCitedMismatch => "export_cited_mismatch",
            Self::DegradationIncomplete => "degradation_incomplete",
            Self::DegradationOrphan => "degradation_orphan",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerProjectionPacketIdMismatch => "consumer_projection_packet_id_mismatch",
            Self::RequiredSurfaceCoverageMissing => "required_surface_coverage_missing",
            Self::RawBoundaryMaterialPresent => "raw_boundary_material_present",
        }
    }

    /// Default severity for this finding kind. Every handoff finding blocks the
    /// Stable claim; narrowing comes only from data-carried degradation
    /// severities so a degraded-but-honest packet narrows rather than blocks.
    pub const fn default_severity(self) -> HandoffFindingSeverity {
        HandoffFindingSeverity::Blocking
    }
}

/// The concrete prose change a handoff entry explains — the doc file and
/// (optionally) the section anchor, plus the originating suggestion when the
/// change came from the suggestion panel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsChangeSubject {
    /// The kind of prose change.
    pub change_kind: DocsChangeKind,
    /// Repo-relative doc file the change lives in (no raw body).
    pub doc_ref: String,
    /// Section/heading anchor within the doc when the change is scoped to one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_anchor: Option<String>,
    /// Human-readable display path for the change.
    pub display_path: String,
    /// Human-readable change label (no raw bodies).
    pub label: String,
    /// Ref to the originating docs-suggestion proposal, when the change came from
    /// the suggestion panel.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originating_suggestion_ref: Option<String>,
}

impl DocsChangeSubject {
    /// Whether the change names a concrete doc, display path, and label (and a
    /// non-empty anchor / suggestion ref when one is recorded).
    pub fn names_concrete_change(&self) -> bool {
        if self.doc_ref.trim().is_empty()
            || self.display_path.trim().is_empty()
            || self.label.trim().is_empty()
        {
            return false;
        }
        if let Some(anchor) = &self.section_anchor {
            if anchor.trim().is_empty() {
                return false;
            }
        }
        if let Some(suggestion) = &self.originating_suggestion_ref {
            if suggestion.trim().is_empty() {
                return false;
            }
        }
        true
    }
}

/// One typed evidence binding — a docs change tied back to one concrete evidence
/// object with scope, redaction, provenance, freshness, and mirror/offline truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceBinding {
    /// Stable binding id within this packet.
    pub binding_id: String,
    /// The kind of evidence object this binding points at.
    pub evidence_kind: EvidenceKind,
    /// Ref to the evidence object (file/symbol/contract/run/release ref) — no raw body.
    pub target_ref: String,
    /// Human-readable display path for the evidence object.
    pub display_path: String,
    /// Human-readable evidence label (no raw bodies).
    pub label: String,
    /// Sharing/export scope for the binding.
    pub scope: EvidenceScope,
    /// Redaction posture for the binding.
    pub redaction_state: EvidenceRedactionState,
    /// Evidence-provenance disclosure for the binding.
    pub provenance: EvidenceProvenance,
    /// Freshness chip.
    pub freshness: EvidenceFreshness,
    /// Version-match chip.
    pub version_match: EvidenceVersionMatch,
    /// Locality chip.
    pub locality: EvidenceLocality,
    /// Mirror/offline continuity posture.
    pub mirror_offline: MirrorOfflinePosture,
    /// Human-readable provenance disclosure note.
    pub provenance_disclosure_note: String,
    /// Ref to open the evidence object (so review/support/AI can reopen it).
    pub open_evidence_ref: String,
    /// Human-readable binding detail (no raw bodies).
    pub detail: String,
    /// Whether the binding is cited back to its source.
    pub cited: bool,
    /// Citation ref when cited.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub citation_ref: Option<String>,
}

impl EvidenceBinding {
    /// Whether the binding names a concrete evidence target, display path, label,
    /// and open-evidence ref.
    pub fn names_concrete_target(&self) -> bool {
        !self.target_ref.trim().is_empty()
            && !self.display_path.trim().is_empty()
            && !self.label.trim().is_empty()
    }
}

/// The reopen handle for an entry — so review and support can reopen the same
/// docs-evidence packet Aureline used in the authoring workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceReopenHandle {
    /// Ref to reopen the entry/packet in the authoring workspace.
    pub reopen_ref: String,
    /// Whether the entry is reopenable at all.
    pub reopenable: bool,
    /// Whether the entry is reopenable from the review flow.
    pub available_in_review: bool,
    /// Whether the entry is reopenable from the support flow.
    pub available_in_support: bool,
}

impl EvidenceReopenHandle {
    /// Whether the entry can be reopened from both the review and support flows.
    pub fn is_reopenable(&self) -> bool {
        self.reopenable
            && self.available_in_review
            && self.available_in_support
            && !self.reopen_ref.trim().is_empty()
    }
}

/// One docs-evidence handoff entry — one prose change and its typed evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffEntry {
    /// Stable entry id within this packet.
    pub entry_id: String,
    /// The concrete prose change this entry explains.
    pub change: DocsChangeSubject,
    /// The typed evidence bindings.
    pub bindings: Vec<EvidenceBinding>,
    /// The overall sharing/export scope for the entry.
    pub entry_scope: EvidenceScope,
    /// The reopen handle.
    pub reopen: EvidenceReopenHandle,
    /// Human-readable detail / summary (no raw bodies).
    pub detail: String,
}

impl EvidenceHandoffEntry {
    /// The set of evidence kinds bound by this entry.
    pub fn evidence_kinds(&self) -> BTreeSet<EvidenceKind> {
        self.bindings.iter().map(|b| b.evidence_kind).collect()
    }

    /// Whether the entry binds its change to at least one concrete typed evidence
    /// object (not narrative/human-note alone).
    pub fn is_concretely_traced(&self) -> bool {
        self.bindings.iter().any(|b| b.evidence_kind.is_concrete())
    }

    /// Whether every binding is cited.
    pub fn all_bindings_cited(&self) -> bool {
        self.bindings.iter().all(|b| b.cited)
    }

    /// Whether the entry and all its bindings are export-safe.
    pub fn is_export_safe(&self) -> bool {
        self.entry_scope.is_export_safe() && self.bindings.iter().all(|b| b.scope.is_export_safe())
    }
}

/// One export row, mirroring a handoff entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceHandoffExportRow {
    /// The entry this export row mirrors.
    pub entry_id_ref: String,
    /// Change kind (must match the entry).
    pub change_kind: DocsChangeKind,
    /// Doc ref (must match the entry's change).
    pub doc_ref: String,
    /// Evidence kinds present in the entry (sorted; must match the entry).
    pub evidence_kinds: Vec<EvidenceKind>,
    /// Number of bindings (must match the entry).
    pub binding_count: u32,
    /// Entry scope (must match the entry).
    pub entry_scope: EvidenceScope,
    /// Whether the entry is export-safe.
    pub export_safe: bool,
    /// Whether the entry is reopenable.
    pub reopenable: bool,
    /// Whether every binding is cited.
    pub cited: bool,
}

/// The docs-evidence-handoff export projection for the entry set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsEvidenceHandoffExport {
    /// Scope this export covers.
    pub scope: HandoffExportScope,
    /// Whether the export preserves each entry's change subject.
    pub preserves_change_subjects: bool,
    /// Whether the export preserves each entry's evidence bindings.
    pub preserves_evidence_bindings: bool,
    /// Whether the export preserves each binding's scope.
    pub preserves_scope: bool,
    /// Whether the export preserves each binding's redaction state.
    pub preserves_redaction: bool,
    /// Whether the export preserves each binding's provenance.
    pub preserves_provenance: bool,
    /// Whether the export preserves each binding's freshness.
    pub preserves_freshness: bool,
    /// Whether the export preserves each binding's mirror/offline posture.
    pub preserves_mirror_offline: bool,
    /// Whether the export preserves each entry's reopen handle.
    pub preserves_reopen: bool,
    /// Per-entry export rows.
    pub rows: Vec<EvidenceHandoffExportRow>,
}

impl DocsEvidenceHandoffExport {
    /// Whether the export preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_change_subjects
            && self.preserves_evidence_bindings
            && self.preserves_scope
            && self.preserves_redaction
            && self.preserves_provenance
            && self.preserves_freshness
            && self.preserves_mirror_offline
            && self.preserves_reopen
    }
}

/// A packet-level docs-evidence-handoff degradation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffDegradation {
    /// Degradation class.
    pub degradation_class: HandoffDegradationClass,
    /// Severity.
    pub severity: HandoffFindingSeverity,
    /// Human-readable summary (no raw bodies).
    pub summary: String,
    /// The entry this degradation annotates, if scoped to one entry.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub entry_id_ref: Option<String>,
    /// Optional supporting evidence ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_ref: Option<String>,
}

/// How a consumer surface projects the docs-evidence-handoff entry set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffConsumerProjection {
    /// Surface that consumes the set.
    pub surface: HandoffConsumerSurface,
    /// Packet id this projection mirrors.
    pub packet_id_ref: String,
    /// Whether the surface preserves the change subjects.
    pub preserves_change_subjects: bool,
    /// Whether the surface preserves the evidence bindings.
    pub preserves_evidence_bindings: bool,
    /// Whether the surface preserves the scope.
    pub preserves_scope: bool,
    /// Whether the surface preserves the redaction state.
    pub preserves_redaction: bool,
    /// Whether the surface preserves the provenance disclosures.
    pub preserves_provenance: bool,
    /// Whether the surface preserves the freshness chips.
    pub preserves_freshness: bool,
    /// Whether the surface preserves the mirror/offline posture.
    pub preserves_mirror_offline: bool,
    /// Whether the surface preserves the reopen handles.
    pub preserves_reopen: bool,
}

impl HandoffConsumerProjection {
    /// Whether the projection preserves every required field.
    pub const fn preserves_all(&self) -> bool {
        self.preserves_change_subjects
            && self.preserves_evidence_bindings
            && self.preserves_scope
            && self.preserves_redaction
            && self.preserves_provenance
            && self.preserves_freshness
            && self.preserves_mirror_offline
            && self.preserves_reopen
    }
}

/// A single handoff finding on the docs-evidence entry set.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffFinding {
    /// Finding kind.
    pub finding_kind: HandoffFindingKind,
    /// Finding severity.
    pub severity: HandoffFindingSeverity,
    /// Human-readable summary.
    pub summary: String,
}

/// Constructor input for [`DocsEvidenceHandoffPacket::materialize`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsEvidenceHandoffPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable handoff label (no raw URLs / no raw bodies).
    pub handoff_label: String,
    /// Opaque digest/ref for the handoff run.
    pub handoff_digest_ref: String,
    /// The handoff entries.
    pub entries: Vec<EvidenceHandoffEntry>,
    /// The export projection.
    pub export: DocsEvidenceHandoffExport,
    /// Packet-level degradations.
    pub handoff_degradations: Vec<HandoffDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<HandoffConsumerProjection>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: String,
}

/// Export-safe docs-evidence-handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsEvidenceHandoffPacket {
    /// Record kind; must equal [`DOCS_EVIDENCE_HANDOFF_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DOCS_EVIDENCE_HANDOFF_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable handoff label.
    pub handoff_label: String,
    /// Opaque digest/ref for the handoff run.
    pub handoff_digest_ref: String,
    /// The handoff entries.
    pub entries: Vec<EvidenceHandoffEntry>,
    /// The export projection.
    pub export: DocsEvidenceHandoffExport,
    /// Packet-level degradations.
    pub handoff_degradations: Vec<HandoffDegradation>,
    /// Consumer projections.
    pub consumer_projections: Vec<HandoffConsumerProjection>,
    /// Computed promotion state.
    pub promotion_state: HandoffPromotionState,
    /// Computed handoff findings.
    pub handoff_findings: Vec<HandoffFinding>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Required consumer surfaces that every docs-evidence packet must project: the
/// review lane, the AI explanation surface, the release/public-truth lane, and
/// the support export — so docs causality is never locked inside the authoring
/// pane.
const REQUIRED_SURFACES: [HandoffConsumerSurface; 4] = [
    HandoffConsumerSurface::DocsReviewPanel,
    HandoffConsumerSurface::AiExplanation,
    HandoffConsumerSurface::ReleasePublicTruth,
    HandoffConsumerSurface::SupportExport,
];

impl DocsEvidenceHandoffPacket {
    /// Materializes a docs-evidence-handoff packet, computing handoff findings
    /// and the promotion state from the input.
    pub fn materialize(input: DocsEvidenceHandoffPacketInput) -> Self {
        let mut findings = Vec::new();

        check_identity(&input, &mut findings);
        check_entries(&input, &mut findings);
        check_export(&input, &mut findings);
        check_degradations(&input, &mut findings);
        check_consumer_projections(&input, &mut findings);
        check_boundary(&input, &mut findings);

        let promotion_state = promotion_state(&findings, &input.handoff_degradations);

        Self {
            record_kind: DOCS_EVIDENCE_HANDOFF_RECORD_KIND.to_owned(),
            schema_version: DOCS_EVIDENCE_HANDOFF_SCHEMA_VERSION,
            packet_id: input.packet_id,
            handoff_label: input.handoff_label,
            handoff_digest_ref: input.handoff_digest_ref,
            entries: input.entries,
            export: input.export,
            handoff_degradations: input.handoff_degradations,
            consumer_projections: input.consumer_projections,
            promotion_state,
            handoff_findings: findings,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Whether the packet qualifies for the Stable claim with no findings.
    pub fn is_clean_stable(&self) -> bool {
        self.promotion_state == HandoffPromotionState::Stable && self.handoff_findings.is_empty()
    }

    /// Wraps the packet in a support-export envelope.
    pub fn support_export(
        &self,
        export_id: &str,
        exported_at: &str,
    ) -> DocsEvidenceHandoffSupportExport {
        DocsEvidenceHandoffSupportExport {
            record_kind: DOCS_EVIDENCE_HANDOFF_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: DOCS_EVIDENCE_HANDOFF_SCHEMA_VERSION,
            export_id: export_id.to_owned(),
            exported_at: exported_at.to_owned(),
            schema_ref: DOCS_EVIDENCE_HANDOFF_SCHEMA_REF.to_owned(),
            doc_ref: DOCS_EVIDENCE_HANDOFF_DOC_REF.to_owned(),
            packet: self.clone(),
        }
    }

    /// Deterministic export-safe pretty JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("docs-evidence-handoff packet serializes")
    }

    /// Deterministic Markdown summary for docs, support, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# Docs Evidence Handoff (prose changes traced to code/schema/run/release)\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Handoff: {}\n", self.handoff_label));
        out.push_str(&format!(
            "- Promotion: `{}` ({} findings)\n",
            self.promotion_state.as_str(),
            self.handoff_findings.len()
        ));
        out.push_str(&format!(
            "- Entries: {} | Degradations: {}\n",
            self.entries.len(),
            self.handoff_degradations.len()
        ));
        out.push_str("\n## Entries\n\n");
        for entry in &self.entries {
            let anchor = entry
                .change
                .section_anchor
                .as_deref()
                .map(|anchor| format!("#{anchor}"))
                .unwrap_or_default();
            out.push_str(&format!(
                "- [{}] `{}` ({}) — change `{}{}`\n",
                entry.change.change_kind.as_str(),
                entry.entry_id,
                entry.change.label,
                entry.change.display_path,
                anchor,
            ));
            out.push_str(&format!(
                "  - Scope: `{}` | export-safe {} | reopenable {}\n",
                entry.entry_scope.as_str(),
                entry.is_export_safe(),
                entry.reopen.is_reopenable(),
            ));
            for binding in &entry.bindings {
                out.push_str(&format!(
                    "  - evidence [{}] `{}` — {} | scope `{}` | redaction `{}` | provenance `{}` | freshness `{}` | version `{}` | mirror `{}` | cited {}\n",
                    binding.evidence_kind.as_str(),
                    binding.binding_id,
                    binding.display_path,
                    binding.scope.as_str(),
                    binding.redaction_state.as_str(),
                    binding.provenance.as_str(),
                    binding.freshness.as_str(),
                    binding.version_match.as_str(),
                    binding.mirror_offline.as_str(),
                    binding.cited,
                ));
            }
        }
        if !self.handoff_degradations.is_empty() {
            out.push_str("\n## Degradations\n\n");
            for degradation in &self.handoff_degradations {
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

/// Support-export envelope for the docs-evidence-handoff packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocsEvidenceHandoffSupportExport {
    /// Record kind; must equal [`DOCS_EVIDENCE_HANDOFF_SUPPORT_EXPORT_RECORD_KIND`].
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
    /// The wrapped docs-evidence-handoff packet.
    pub packet: DocsEvidenceHandoffPacket,
}

/// Errors emitted when reading the checked-in docs-evidence-handoff support export.
#[derive(Debug)]
pub enum DocsEvidenceHandoffArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Re-materialization disagreed with the checked-in promotion state.
    PromotionDrift {
        /// Promotion state recorded in the export.
        recorded: HandoffPromotionState,
        /// Promotion state computed by re-materialization.
        computed: HandoffPromotionState,
    },
    /// The checked-in packet should be clean Stable but is not.
    NotCleanStable(Vec<HandoffFinding>),
}

impl fmt::Display for DocsEvidenceHandoffArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "docs-evidence-handoff export parse failed: {error}"
                )
            }
            Self::PromotionDrift { recorded, computed } => write!(
                formatter,
                "docs-evidence-handoff promotion drift: recorded {} but computed {}",
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
                    "docs-evidence-handoff export is not clean stable: {tokens}"
                )
            }
        }
    }
}

impl Error for DocsEvidenceHandoffArtifactError {}

/// Reads and re-validates the checked-in stable docs-evidence-handoff support export.
pub fn current_stable_docs_evidence_handoff_export(
) -> Result<DocsEvidenceHandoffSupportExport, DocsEvidenceHandoffArtifactError> {
    let export: DocsEvidenceHandoffSupportExport = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/docs/m5/docs-evidence-handoff-proof/support_export.json"
    )))
    .map_err(DocsEvidenceHandoffArtifactError::SupportExport)?;

    let recomputed = DocsEvidenceHandoffPacket::materialize(packet_to_input(&export.packet));
    if recomputed.promotion_state != export.packet.promotion_state {
        return Err(DocsEvidenceHandoffArtifactError::PromotionDrift {
            recorded: export.packet.promotion_state,
            computed: recomputed.promotion_state,
        });
    }
    if !export.packet.is_clean_stable() {
        return Err(DocsEvidenceHandoffArtifactError::NotCleanStable(
            export.packet.handoff_findings.clone(),
        ));
    }
    Ok(export)
}

/// Rebuilds the materialization input from a packet (used for re-validation).
pub fn packet_to_input(packet: &DocsEvidenceHandoffPacket) -> DocsEvidenceHandoffPacketInput {
    DocsEvidenceHandoffPacketInput {
        packet_id: packet.packet_id.clone(),
        handoff_label: packet.handoff_label.clone(),
        handoff_digest_ref: packet.handoff_digest_ref.clone(),
        entries: packet.entries.clone(),
        export: packet.export.clone(),
        handoff_degradations: packet.handoff_degradations.clone(),
        consumer_projections: packet.consumer_projections.clone(),
        redaction_class_token: packet.redaction_class_token.clone(),
        minted_at: packet.minted_at.clone(),
    }
}

fn push_finding(
    findings: &mut Vec<HandoffFinding>,
    kind: HandoffFindingKind,
    summary: impl Into<String>,
) {
    findings.push(HandoffFinding {
        finding_kind: kind,
        severity: kind.default_severity(),
        summary: summary.into(),
    });
}

fn check_identity(input: &DocsEvidenceHandoffPacketInput, findings: &mut Vec<HandoffFinding>) {
    if input.packet_id.trim().is_empty()
        || input.handoff_label.trim().is_empty()
        || input.handoff_digest_ref.trim().is_empty()
        || input.redaction_class_token.trim().is_empty()
        || input.minted_at.trim().is_empty()
    {
        push_finding(
            findings,
            HandoffFindingKind::MissingIdentity,
            "packet identity fields must all be present",
        );
    }
}

fn check_entries(input: &DocsEvidenceHandoffPacketInput, findings: &mut Vec<HandoffFinding>) {
    if input.entries.is_empty() {
        push_finding(
            findings,
            HandoffFindingKind::EntriesEmpty,
            "the docs-evidence handoff must carry at least one entry",
        );
        return;
    }

    let present_kinds: BTreeSet<EvidenceKind> = input
        .entries
        .iter()
        .flat_map(|entry| entry.bindings.iter().map(|b| b.evidence_kind))
        .collect();
    for required in EvidenceKind::REQUIRED_COVERAGE {
        if !present_kinds.contains(&required) {
            push_finding(
                findings,
                HandoffFindingKind::RequiredEvidenceKindMissing,
                format!("required evidence kind `{}` is missing", required.as_str()),
            );
        }
    }

    let mut seen_entry_ids: BTreeSet<&str> = BTreeSet::new();
    let mut seen_binding_ids: BTreeSet<&str> = BTreeSet::new();
    for entry in &input.entries {
        if !seen_entry_ids.insert(entry.entry_id.as_str()) {
            push_finding(
                findings,
                HandoffFindingKind::DuplicateEntryId,
                format!("duplicate entry id `{}`", entry.entry_id),
            );
        }
        for binding in &entry.bindings {
            if !seen_binding_ids.insert(binding.binding_id.as_str()) {
                push_finding(
                    findings,
                    HandoffFindingKind::DuplicateBindingId,
                    format!("duplicate binding id `{}`", binding.binding_id),
                );
            }
        }
        check_one_entry(entry, findings);
    }
}

fn check_one_entry(entry: &EvidenceHandoffEntry, findings: &mut Vec<HandoffFinding>) {
    let id = &entry.entry_id;

    // Concrete change subject.
    if !entry.change.names_concrete_change() {
        push_finding(
            findings,
            HandoffFindingKind::ChangeSubjectMissing,
            format!("entry `{id}` must name a concrete change doc/section/label"),
        );
    }
    if entry.detail.trim().is_empty() {
        push_finding(
            findings,
            HandoffFindingKind::ChangeDetailMissing,
            format!("entry `{id}` is missing its detail"),
        );
    }

    // Concrete, typed traceability.
    if entry.bindings.is_empty() {
        push_finding(
            findings,
            HandoffFindingKind::BindingsEmpty,
            format!("entry `{id}` carries no evidence bindings"),
        );
    } else if !entry.is_concretely_traced() {
        push_finding(
            findings,
            HandoffFindingKind::ChangeNotConcretelyTraced,
            format!(
                "entry `{id}` is traced by human notes alone; it must bind to at least one concrete typed evidence object"
            ),
        );
    }

    // Reopenable from review and support.
    if !entry.reopen.is_reopenable() {
        push_finding(
            findings,
            HandoffFindingKind::EntryNotReopenable,
            format!("entry `{id}` must stay reopenable from both the review and support flows"),
        );
    }

    // Entry scope may never be wider than its bindings.
    let widens = if entry.entry_scope.is_export_safe() {
        entry.bindings.iter().any(|b| !b.scope.is_export_safe())
    } else if entry.entry_scope.crosses_share_boundary() {
        entry.bindings.iter().any(|b| b.scope.is_local_only())
    } else {
        false
    };
    if widens {
        push_finding(
            findings,
            HandoffFindingKind::EntryScopeWiderThanBindings,
            format!(
                "entry `{id}` scope `{}` is wider than its bindings allow",
                entry.entry_scope.as_str()
            ),
        );
    }

    for binding in &entry.bindings {
        check_one_binding(id, binding, findings);
    }
}

fn check_one_binding(
    entry_id: &str,
    binding: &EvidenceBinding,
    findings: &mut Vec<HandoffFinding>,
) {
    let bid = &binding.binding_id;

    if !binding.names_concrete_target() {
        push_finding(
            findings,
            HandoffFindingKind::BindingTargetMissing,
            format!("binding `{bid}` (entry `{entry_id}`) must name a concrete evidence target"),
        );
    }
    if binding.open_evidence_ref.trim().is_empty() {
        push_finding(
            findings,
            HandoffFindingKind::BindingOpenEvidenceMissing,
            format!("binding `{bid}` is missing its open-evidence ref"),
        );
    }
    if binding.provenance_disclosure_note.trim().is_empty() {
        push_finding(
            findings,
            HandoffFindingKind::ProvenanceDisclosureMissing,
            format!("binding `{bid}` is missing its provenance disclosure"),
        );
    }

    // Scope and redaction honesty.
    if binding.redaction_state.requires_local_only() && !binding.scope.is_local_only() {
        push_finding(
            findings,
            HandoffFindingKind::ScopeRedactionInconsistent,
            format!(
                "binding `{bid}` requires local-only redaction but scope is `{}`",
                binding.scope.as_str()
            ),
        );
    }
    if binding.scope.is_export_safe() && !binding.redaction_state.is_export_ok() {
        push_finding(
            findings,
            HandoffFindingKind::ScopeRedactionInconsistent,
            format!(
                "binding `{bid}` is export-safe but redaction `{}` is not export-ready",
                binding.redaction_state.as_str()
            ),
        );
    }
    if binding.provenance == EvidenceProvenance::LocalOnlyUnverified
        && binding.scope.is_export_safe()
    {
        push_finding(
            findings,
            HandoffFindingKind::LocalOnlyMarkedExportSafe,
            format!("binding `{bid}` is local-only-unverified but marked export-safe"),
        );
    }

    // Mirror/offline continuity.
    if binding.mirror_offline.is_mirror_or_offline() && binding.freshness.is_authoritative_live() {
        push_finding(
            findings,
            HandoffFindingKind::OfflineClaimsLiveFreshness,
            format!(
                "binding `{bid}` is `{}` but claims authoritative live freshness",
                binding.mirror_offline.as_str()
            ),
        );
    }

    // Provenance and freshness truth.
    if !binding.provenance.is_authoritative() && binding.freshness.is_authoritative_live() {
        push_finding(
            findings,
            HandoffFindingKind::EvidenceTruthCollapsed,
            format!(
                "binding `{bid}` is `{}` but presented as authoritative live truth",
                binding.provenance.as_str()
            ),
        );
    }
    if !binding.version_match.is_confident_current() && binding.freshness.is_authoritative_live() {
        push_finding(
            findings,
            HandoffFindingKind::VersionTruthCollapsed,
            format!(
                "binding `{bid}` presents version `{}` as authoritative live truth",
                binding.version_match.as_str()
            ),
        );
    }
    if binding.provenance.needs_citation() && !binding.cited {
        push_finding(
            findings,
            HandoffFindingKind::BindingNotCited,
            format!(
                "binding `{bid}` is `{}` but is not cited",
                binding.provenance.as_str()
            ),
        );
    }
}

fn check_export(input: &DocsEvidenceHandoffPacketInput, findings: &mut Vec<HandoffFinding>) {
    let export = &input.export;
    if !export.preserves_all() {
        push_finding(
            findings,
            HandoffFindingKind::ExportDropsPreservation,
            "the export must preserve change subjects, evidence bindings, scope, redaction, provenance, freshness, mirror/offline posture, and reopen",
        );
    }

    let mut export_ids: BTreeSet<&str> = BTreeSet::new();
    for row in &export.rows {
        export_ids.insert(row.entry_id_ref.as_str());
        match input
            .entries
            .iter()
            .find(|e| e.entry_id == row.entry_id_ref)
        {
            None => push_finding(
                findings,
                HandoffFindingKind::ExportEntryOrphan,
                format!("export row references unknown entry `{}`", row.entry_id_ref),
            ),
            Some(source) => check_export_row(source, row, findings),
        }
    }

    for entry in &input.entries {
        if !export_ids.contains(entry.entry_id.as_str()) {
            push_finding(
                findings,
                HandoffFindingKind::ExportCoverageMissing,
                format!("entry `{}` has no export row", entry.entry_id),
            );
        }
    }
}

fn check_export_row(
    source: &EvidenceHandoffEntry,
    row: &EvidenceHandoffExportRow,
    findings: &mut Vec<HandoffFinding>,
) {
    let id = &row.entry_id_ref;
    if source.change.change_kind != row.change_kind {
        push_finding(
            findings,
            HandoffFindingKind::ExportChangeKindMismatch,
            format!(
                "export for `{id}` records change kind `{}` but the entry is `{}`",
                row.change_kind.as_str(),
                source.change.change_kind.as_str()
            ),
        );
    }
    if source.change.doc_ref != row.doc_ref {
        push_finding(
            findings,
            HandoffFindingKind::ExportDocRefMismatch,
            format!("export for `{id}` records a different doc ref than the entry"),
        );
    }
    if source.entry_scope != row.entry_scope {
        push_finding(
            findings,
            HandoffFindingKind::ExportScopeMismatch,
            format!(
                "export for `{id}` records scope `{}` but the entry is `{}`",
                row.entry_scope.as_str(),
                source.entry_scope.as_str()
            ),
        );
    }
    if source.is_export_safe() != row.export_safe {
        push_finding(
            findings,
            HandoffFindingKind::ExportExportSafeMismatch,
            format!(
                "export for `{id}` records export-safe `{}` but the entry is `{}`",
                row.export_safe,
                source.is_export_safe()
            ),
        );
    }
    if source.reopen.is_reopenable() != row.reopenable {
        push_finding(
            findings,
            HandoffFindingKind::ExportReopenableMismatch,
            format!(
                "export for `{id}` records reopenable `{}` but the entry is `{}`",
                row.reopenable,
                source.reopen.is_reopenable()
            ),
        );
    }
    let source_kinds: Vec<EvidenceKind> = source.evidence_kinds().into_iter().collect();
    if source_kinds != row.evidence_kinds {
        push_finding(
            findings,
            HandoffFindingKind::ExportEvidenceKindsMismatch,
            format!("export for `{id}` records different evidence kinds than the entry"),
        );
    }
    if source.bindings.len() as u32 != row.binding_count {
        push_finding(
            findings,
            HandoffFindingKind::ExportBindingCountMismatch,
            format!(
                "export for `{id}` records binding count `{}` but the entry has `{}`",
                row.binding_count,
                source.bindings.len()
            ),
        );
    }
    if source.all_bindings_cited() != row.cited {
        push_finding(
            findings,
            HandoffFindingKind::ExportCitedMismatch,
            format!(
                "export for `{id}` records cited `{}` but the entry is `{}`",
                row.cited,
                source.all_bindings_cited()
            ),
        );
    }
}

fn check_degradations(input: &DocsEvidenceHandoffPacketInput, findings: &mut Vec<HandoffFinding>) {
    let entry_ids: BTreeSet<&str> = input
        .entries
        .iter()
        .map(|entry| entry.entry_id.as_str())
        .collect();

    for degradation in &input.handoff_degradations {
        if degradation.summary.trim().is_empty() {
            push_finding(
                findings,
                HandoffFindingKind::DegradationIncomplete,
                format!(
                    "degradation `{}` is missing a summary",
                    degradation.degradation_class.as_str()
                ),
            );
        }
        if let Some(entry_id) = &degradation.entry_id_ref {
            if !entry_id.trim().is_empty() && !entry_ids.contains(entry_id.as_str()) {
                push_finding(
                    findings,
                    HandoffFindingKind::DegradationOrphan,
                    format!("degradation references unknown entry `{entry_id}`"),
                );
            }
        }
    }
}

fn check_consumer_projections(
    input: &DocsEvidenceHandoffPacketInput,
    findings: &mut Vec<HandoffFinding>,
) {
    let present: BTreeSet<HandoffConsumerSurface> = input
        .consumer_projections
        .iter()
        .map(|projection| projection.surface)
        .collect();
    for required in REQUIRED_SURFACES {
        if !present.contains(&required) {
            push_finding(
                findings,
                HandoffFindingKind::RequiredSurfaceCoverageMissing,
                format!("required surface `{}` is missing", required.as_str()),
            );
        }
    }

    for projection in &input.consumer_projections {
        if projection.packet_id_ref != input.packet_id {
            push_finding(
                findings,
                HandoffFindingKind::ConsumerProjectionPacketIdMismatch,
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
                HandoffFindingKind::ConsumerProjectionDrift,
                format!(
                    "surface `{}` drops a required preservation flag",
                    projection.surface.as_str()
                ),
            );
        }
    }
}

fn check_boundary(input: &DocsEvidenceHandoffPacketInput, findings: &mut Vec<HandoffFinding>) {
    let value = serde_json::to_value(input).expect("docs-evidence-handoff input serializes");
    if json_contains_forbidden_boundary_material(&value) {
        push_finding(
            findings,
            HandoffFindingKind::RawBoundaryMaterialPresent,
            "export must not carry raw bodies, raw diffs, raw URLs, or secrets",
        );
    }
}

/// Computes the promotion state from the worst severity across the handoff
/// findings and the attached degradations.
///
/// A blocking handoff finding (untraced change, widened scope, scope/redaction
/// inconsistency, collapsed truth, non-reopenable entry, export/projection drift,
/// or boundary violation) blocks the Stable claim; an otherwise-clean packet
/// whose degradations carry a narrowing severity narrows below Stable rather than
/// hiding the entries.
fn promotion_state(
    findings: &[HandoffFinding],
    degradations: &[HandoffDegradation],
) -> HandoffPromotionState {
    let any_blocking = findings
        .iter()
        .any(|finding| finding.severity == HandoffFindingSeverity::Blocking)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == HandoffFindingSeverity::Blocking);
    if any_blocking {
        return HandoffPromotionState::BlocksStable;
    }

    let any_narrowing = findings
        .iter()
        .any(|finding| finding.severity == HandoffFindingSeverity::Narrowing)
        || degradations
            .iter()
            .any(|degradation| degradation.severity == HandoffFindingSeverity::Narrowing);
    if any_narrowing {
        HandoffPromotionState::NarrowedBelowStable
    } else {
        HandoffPromotionState::Stable
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
                || lower.contains("raw_diff:")
                || lower.contains("raw_url:")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Seeded stable docs-evidence-handoff input used by the producer, tests, and
/// fixtures.
pub fn seeded_stable_docs_evidence_handoff_input() -> DocsEvidenceHandoffPacketInput {
    let packet_id = "packet:m5:docs_evidence_handoff:retry_backoff_release".to_owned();
    DocsEvidenceHandoffPacketInput {
        packet_id: packet_id.clone(),
        handoff_label: "docs evidence handoff: the retry/backoff release docs sweep".to_owned(),
        handoff_digest_ref: "handoffdigest:sha256:retry-backoff-release-evidence".to_owned(),
        entries: seeded_entries(),
        export: seeded_export(),
        handoff_degradations: vec![HandoffDegradation {
            degradation_class: HandoffDegradationClass::MirrorOfflineSnapshot,
            severity: HandoffFindingSeverity::Advisory,
            summary: "one imported ops-pack binding is served from the mirror snapshot; it is held to warm-cached freshness rather than claiming live authority".to_owned(),
            entry_id_ref: Some("entry:help:offline_runbook_note".to_owned()),
            evidence_ref: Some("evidence:docs-evidence-handoff:mirror-state".to_owned()),
        }],
        consumer_projections: required_projections(&packet_id),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-12T00:00:00Z".to_owned(),
    }
}

fn reopen_handle(reopen_ref: &str) -> EvidenceReopenHandle {
    EvidenceReopenHandle {
        reopen_ref: reopen_ref.to_owned(),
        reopenable: true,
        available_in_review: true,
        available_in_support: true,
    }
}

fn seeded_entries() -> Vec<EvidenceHandoffEntry> {
    vec![
        readme_fix_entry(),
        changelog_entry(),
        api_reference_entry(),
        help_offline_note_entry(),
    ]
}

fn readme_fix_entry() -> EvidenceHandoffEntry {
    EvidenceHandoffEntry {
        entry_id: "entry:readme:config_example_fix".to_owned(),
        change: DocsChangeSubject {
            change_kind: DocsChangeKind::ReadmeEdit,
            doc_ref: "docs/guides/retry_with_backoff/README.md".to_owned(),
            section_anchor: Some("configuration".to_owned()),
            display_path: "README → Configuration → max_elapsed example".to_owned(),
            label: "update the retry_with_backoff configuration example".to_owned(),
            originating_suggestion_ref: Some(
                "suggestion:docs-suggestion-panel:readme-config-example".to_owned(),
            ),
        },
        bindings: vec![
            EvidenceBinding {
                binding_id: "binding:readme:source_file".to_owned(),
                evidence_kind: EvidenceKind::SourceFile,
                target_ref: "source:crates/aureline-net/src/retry.rs@workspace-rev".to_owned(),
                display_path: "crates/aureline-net/src/retry.rs".to_owned(),
                label: "the retry source the example documents".to_owned(),
                scope: EvidenceScope::ExportSafeShared,
                redaction_state: EvidenceRedactionState::MetadataSafe,
                provenance: EvidenceProvenance::FirstPartyVerified,
                freshness: EvidenceFreshness::AuthoritativeLive,
                version_match: EvidenceVersionMatch::ExactBuildMatch,
                locality: EvidenceLocality::Local,
                mirror_offline: MirrorOfflinePosture::OnlineLive,
                provenance_disclosure_note: "first-party source verified against the in-repo build at the active revision".to_owned(),
                open_evidence_ref: "open-source:repo:crates/aureline-net/src/retry.rs".to_owned(),
                detail: "the README example was updated to match the current retry source".to_owned(),
                cited: true,
                citation_ref: Some("cite:source-file:retry-rs".to_owned()),
            },
            EvidenceBinding {
                binding_id: "binding:readme:symbol".to_owned(),
                evidence_kind: EvidenceKind::Symbol,
                target_ref: "symbol:crates/aureline-net/src/retry.rs#with_backoff".to_owned(),
                display_path: "retry::with_backoff".to_owned(),
                label: "the with_backoff builder the example calls".to_owned(),
                scope: EvidenceScope::ExportSafeShared,
                redaction_state: EvidenceRedactionState::MetadataSafe,
                provenance: EvidenceProvenance::FirstPartyVerified,
                freshness: EvidenceFreshness::AuthoritativeLive,
                version_match: EvidenceVersionMatch::ExactBuildMatch,
                locality: EvidenceLocality::Local,
                mirror_offline: MirrorOfflinePosture::OnlineLive,
                provenance_disclosure_note: "first-party symbol resolved from the in-repo index at the active revision".to_owned(),
                open_evidence_ref: "open-symbol:crates/aureline-net/src/retry.rs#with_backoff".to_owned(),
                detail: "the documented call site matches the current with_backoff signature".to_owned(),
                cited: true,
                citation_ref: Some("cite:symbol:with-backoff".to_owned()),
            },
            EvidenceBinding {
                binding_id: "binding:readme:failing_example".to_owned(),
                evidence_kind: EvidenceKind::FailingExample,
                target_ref: "validation-row:docs_validation_report:readme:jitter_stale_example".to_owned(),
                display_path: "Docs validation → README → with_jitter example (stale)".to_owned(),
                label: "the stale-example finding that motivated the fix".to_owned(),
                scope: EvidenceScope::ExportSafeShared,
                redaction_state: EvidenceRedactionState::MetadataSafe,
                provenance: EvidenceProvenance::FirstPartyVerified,
                freshness: EvidenceFreshness::WarmCached,
                version_match: EvidenceVersionMatch::ExactBuildMatch,
                locality: EvidenceLocality::Local,
                mirror_offline: MirrorOfflinePosture::OnlineLive,
                provenance_disclosure_note: "first-party validation finding cited from the docs validation report".to_owned(),
                open_evidence_ref: "open-validation-row:docs_validation_report:readme:jitter_stale_example".to_owned(),
                detail: "the docs validation report flagged the example as stale, prompting this change".to_owned(),
                cited: true,
                citation_ref: Some("cite:failing-example:readme-jitter".to_owned()),
            },
        ],
        entry_scope: EvidenceScope::ExportSafeShared,
        reopen: reopen_handle("reopen:docs-evidence-handoff:readme_config_example_fix"),
        detail: "the README configuration example fix is traced to the retry source, the with_backoff symbol, and the stale-example validation finding".to_owned(),
    }
}

fn changelog_entry() -> EvidenceHandoffEntry {
    EvidenceHandoffEntry {
        entry_id: "entry:changelog:retry_backoff_release".to_owned(),
        change: DocsChangeSubject {
            change_kind: DocsChangeKind::ChangelogEntry,
            doc_ref: "CHANGELOG.md".to_owned(),
            section_anchor: Some("retry-backoff".to_owned()),
            display_path: "Changelog → next channel → retry/backoff".to_owned(),
            label: "add the retry/backoff changelog entry".to_owned(),
            originating_suggestion_ref: None,
        },
        bindings: vec![
            EvidenceBinding {
                binding_id: "binding:changelog:release_object".to_owned(),
                evidence_kind: EvidenceKind::ReleaseObject,
                target_ref: "release:next-channel@retry_backoff".to_owned(),
                display_path: "Release center → next channel → retry/backoff".to_owned(),
                label: "the release object the entry describes".to_owned(),
                scope: EvidenceScope::ExportSafeShared,
                redaction_state: EvidenceRedactionState::MetadataSafe,
                provenance: EvidenceProvenance::FirstPartyVerified,
                freshness: EvidenceFreshness::WarmCached,
                version_match: EvidenceVersionMatch::CompatibleMinorDrift,
                locality: EvidenceLocality::Managed,
                mirror_offline: MirrorOfflinePosture::OnlineLive,
                provenance_disclosure_note: "first-party release object from the release center; held to warm-cached freshness for the next channel".to_owned(),
                open_evidence_ref: "open-release:next-channel@retry_backoff".to_owned(),
                detail: "the changelog entry is bound to the release object it documents".to_owned(),
                cited: true,
                citation_ref: Some("cite:release-object:retry-backoff".to_owned()),
            },
            EvidenceBinding {
                binding_id: "binding:changelog:test_run".to_owned(),
                evidence_kind: EvidenceKind::TestRun,
                target_ref: "test-run:ci:retry-backoff-suite@next-channel".to_owned(),
                display_path: "CI → retry/backoff suite → next channel".to_owned(),
                label: "the test run that gated the release".to_owned(),
                scope: EvidenceScope::ExportSafeShared,
                redaction_state: EvidenceRedactionState::MetadataSafe,
                provenance: EvidenceProvenance::FirstPartyVerified,
                freshness: EvidenceFreshness::WarmCached,
                version_match: EvidenceVersionMatch::CompatibleMinorDrift,
                locality: EvidenceLocality::Managed,
                mirror_offline: MirrorOfflinePosture::OnlineLive,
                provenance_disclosure_note: "first-party CI run referenced by digest; no raw logs cross the boundary".to_owned(),
                open_evidence_ref: "open-test-run:ci:retry-backoff-suite@next-channel".to_owned(),
                detail: "the changelog entry cites the test run that gated the release".to_owned(),
                cited: true,
                citation_ref: Some("cite:test-run:retry-backoff-suite".to_owned()),
            },
        ],
        entry_scope: EvidenceScope::ExportSafeShared,
        reopen: reopen_handle("reopen:docs-evidence-handoff:changelog_retry_backoff"),
        detail: "the changelog entry is traced to the release object and the gating test run".to_owned(),
    }
}

fn api_reference_entry() -> EvidenceHandoffEntry {
    EvidenceHandoffEntry {
        entry_id: "entry:api_reference:retry_policy".to_owned(),
        change: DocsChangeSubject {
            change_kind: DocsChangeKind::ApiReferenceEdit,
            doc_ref: "docs/api/retry_policy.md".to_owned(),
            section_anchor: Some("retry-policy".to_owned()),
            display_path: "API reference → RetryPolicy".to_owned(),
            label: "update the RetryPolicy API reference".to_owned(),
            originating_suggestion_ref: None,
        },
        bindings: vec![
            EvidenceBinding {
                binding_id: "binding:api_reference:contract".to_owned(),
                evidence_kind: EvidenceKind::ApiContract,
                target_ref: "contract:schemas/net/retry_policy.schema.json".to_owned(),
                display_path: "schemas/net/retry_policy.schema.json".to_owned(),
                label: "the RetryPolicy schema the reference documents".to_owned(),
                scope: EvidenceScope::ExportSafeShared,
                redaction_state: EvidenceRedactionState::MetadataSafe,
                provenance: EvidenceProvenance::FirstPartyVerified,
                freshness: EvidenceFreshness::AuthoritativeLive,
                version_match: EvidenceVersionMatch::ExactBuildMatch,
                locality: EvidenceLocality::Local,
                mirror_offline: MirrorOfflinePosture::OnlineLive,
                provenance_disclosure_note: "first-party schema contract verified against the in-repo schema at the active revision".to_owned(),
                open_evidence_ref: "open-contract:schemas/net/retry_policy.schema.json".to_owned(),
                detail: "the API reference is bound to the RetryPolicy schema contract".to_owned(),
                cited: true,
                citation_ref: Some("cite:api-contract:retry-policy".to_owned()),
            },
            EvidenceBinding {
                binding_id: "binding:api_reference:symbol".to_owned(),
                evidence_kind: EvidenceKind::Symbol,
                target_ref: "symbol:crates/aureline-net/src/retry.rs#RetryPolicy".to_owned(),
                display_path: "retry::RetryPolicy".to_owned(),
                label: "the RetryPolicy type the reference documents".to_owned(),
                scope: EvidenceScope::ExportSafeShared,
                redaction_state: EvidenceRedactionState::MetadataSafe,
                provenance: EvidenceProvenance::FirstPartyVerified,
                freshness: EvidenceFreshness::AuthoritativeLive,
                version_match: EvidenceVersionMatch::ExactBuildMatch,
                locality: EvidenceLocality::Local,
                mirror_offline: MirrorOfflinePosture::OnlineLive,
                provenance_disclosure_note: "first-party symbol resolved from the in-repo index at the active revision".to_owned(),
                open_evidence_ref: "open-symbol:crates/aureline-net/src/retry.rs#RetryPolicy".to_owned(),
                detail: "the reference matches the current RetryPolicy type definition".to_owned(),
                cited: true,
                citation_ref: Some("cite:symbol:retry-policy".to_owned()),
            },
        ],
        entry_scope: EvidenceScope::ExportSafeShared,
        reopen: reopen_handle("reopen:docs-evidence-handoff:api_reference_retry_policy"),
        detail: "the API reference update is traced to the RetryPolicy schema contract and type".to_owned(),
    }
}

fn help_offline_note_entry() -> EvidenceHandoffEntry {
    EvidenceHandoffEntry {
        entry_id: "entry:help:offline_runbook_note".to_owned(),
        change: DocsChangeSubject {
            change_kind: DocsChangeKind::HelpEdit,
            doc_ref: "docs/help/retry-and-backoff.md".to_owned(),
            section_anchor: Some("operations-runbook".to_owned()),
            display_path: "Help → Retry and backoff → Operations runbook".to_owned(),
            label: "annotate the operations-runbook help note".to_owned(),
            originating_suggestion_ref: None,
        },
        bindings: vec![
            EvidenceBinding {
                binding_id: "binding:help:imported_runbook_source".to_owned(),
                evidence_kind: EvidenceKind::SourceFile,
                target_ref: "pack:ops/runbooks/retry_backoff_runbook.md@imported-rev".to_owned(),
                display_path: "imported ops pack → runbooks → retry_backoff_runbook.md".to_owned(),
                label: "the imported ops-pack runbook the note references".to_owned(),
                scope: EvidenceScope::LocalOnly,
                redaction_state: EvidenceRedactionState::LocalOnlyRedactionRequired,
                provenance: EvidenceProvenance::Imported,
                freshness: EvidenceFreshness::WarmCached,
                version_match: EvidenceVersionMatch::CompatibleMinorDrift,
                locality: EvidenceLocality::ImportedPack,
                mirror_offline: MirrorOfflinePosture::MirrorServed,
                provenance_disclosure_note: "imported from the signed ops pack and served from the mirror; kept local-only because it carries org-internal runbook context that must not auto-export".to_owned(),
                open_evidence_ref: "open-pack:ops/runbooks/retry_backoff_runbook.md".to_owned(),
                detail: "the help note references the imported ops runbook, held local-only on the mirror-first profile".to_owned(),
                cited: true,
                citation_ref: Some("cite:imported:ops-runbook".to_owned()),
            },
            EvidenceBinding {
                binding_id: "binding:help:maintainer_note".to_owned(),
                evidence_kind: EvidenceKind::HumanNote,
                target_ref: "note:maintainer:ops-owner:retry-runbook-context".to_owned(),
                display_path: "Maintainer note → ops owner".to_owned(),
                label: "the maintainer's local context note".to_owned(),
                scope: EvidenceScope::LocalOnly,
                redaction_state: EvidenceRedactionState::LocalOnlyRedactionRequired,
                provenance: EvidenceProvenance::LocalOnlyUnverified,
                freshness: EvidenceFreshness::Unverified,
                version_match: EvidenceVersionMatch::UnknownTargetBuild,
                locality: EvidenceLocality::Local,
                mirror_offline: MirrorOfflinePosture::OfflineCachedUsable,
                provenance_disclosure_note: "local-only maintainer note; unverified and held local until reviewed, never auto-shared".to_owned(),
                open_evidence_ref: "open-note:maintainer:ops-owner:retry-runbook-context".to_owned(),
                detail: "a maintainer recorded local context for the runbook note; it stays local-only".to_owned(),
                cited: true,
                citation_ref: Some("cite:human-note:ops-owner".to_owned()),
            },
        ],
        entry_scope: EvidenceScope::LocalOnly,
        reopen: reopen_handle("reopen:docs-evidence-handoff:help_offline_runbook_note"),
        detail: "the help note stays local-only with imported and human-note evidence held to mirror/offline truth, so the change remains usable air-gapped without widening scope".to_owned(),
    }
}

fn export_row(entry: &EvidenceHandoffEntry) -> EvidenceHandoffExportRow {
    EvidenceHandoffExportRow {
        entry_id_ref: entry.entry_id.clone(),
        change_kind: entry.change.change_kind,
        doc_ref: entry.change.doc_ref.clone(),
        evidence_kinds: entry.evidence_kinds().into_iter().collect(),
        binding_count: entry.bindings.len() as u32,
        entry_scope: entry.entry_scope,
        export_safe: entry.is_export_safe(),
        reopenable: entry.reopen.is_reopenable(),
        cited: entry.all_bindings_cited(),
    }
}

fn seeded_export() -> DocsEvidenceHandoffExport {
    let rows = seeded_entries().iter().map(export_row).collect();
    DocsEvidenceHandoffExport {
        scope: HandoffExportScope::AllEntries,
        preserves_change_subjects: true,
        preserves_evidence_bindings: true,
        preserves_scope: true,
        preserves_redaction: true,
        preserves_provenance: true,
        preserves_freshness: true,
        preserves_mirror_offline: true,
        preserves_reopen: true,
        rows,
    }
}

fn required_projections(packet_id: &str) -> Vec<HandoffConsumerProjection> {
    [
        HandoffConsumerSurface::DocsEvidenceHandoff,
        HandoffConsumerSurface::DocsAuthoringSurface,
        HandoffConsumerSurface::DocsReviewPanel,
        HandoffConsumerSurface::DocsBrowserShell,
        HandoffConsumerSurface::AiExplanation,
        HandoffConsumerSurface::ReleasePublicTruth,
        HandoffConsumerSurface::CliHeadless,
        HandoffConsumerSurface::SupportExport,
        HandoffConsumerSurface::Diagnostics,
        HandoffConsumerSurface::HelpAbout,
    ]
    .into_iter()
    .map(|surface| HandoffConsumerProjection {
        surface,
        packet_id_ref: packet_id.to_owned(),
        preserves_change_subjects: true,
        preserves_evidence_bindings: true,
        preserves_scope: true,
        preserves_redaction: true,
        preserves_provenance: true,
        preserves_freshness: true,
        preserves_mirror_offline: true,
        preserves_reopen: true,
    })
    .collect()
}
